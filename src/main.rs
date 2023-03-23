use std::{env, fs, process, collections::HashMap};

use json::{self, JsonValue};

fn main() {
  println!("Starting trest");
  let config_file = json::parse(&get_config_file()).expect("failed to parse config");
  println!("Config file read");
  println!("There are {} configs to run", config_file["configs"].len());

  let mut did_tests_fail = false;

  for config in config_file["configs"].members() {
    if !run_config(config, &config_file) {
      did_tests_fail = true;
    }
  }

  if did_tests_fail {
    process::exit(1);
  } else {
    process::exit(0);
  }
}

#[derive(PartialEq)]
enum TestOutcomes {
  Passed, Failed
}

fn get_config_file() -> String {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("failed to get config file path");
  }
  return fs::read_to_string(args[1].clone()).expect("failed to read config file");
}

fn run_config(config: &JsonValue, config_file: &JsonValue) -> bool {
  println!("Running config {}: {}", config["name"], config["description"]);
  
  let mut test_outcomes: Vec<TestOutcomes> = Vec::new();

  for test_chain in config_file["tests"].members() {
    println!("\n--------------\nrunning test chain {}", test_chain["name"]);
    run_setup(config);
  
    let mut test_chain_outcomes: Vec<TestOutcomes> = test_chain["tests"]
      .members()
      .map(|x| run_test(x, config, config_file))
      .collect();

    test_outcomes.append(&mut test_chain_outcomes);
  
    if config["cleanup"]["cmd"].is_string() {
      run_cleanup(config);
    }
  }

  let passed_tests = test_outcomes.iter().filter(|x| **x == TestOutcomes::Passed).collect::<Vec<_>>().len();
  let total_tests = test_outcomes.len();

  println!(
    "\nConfig {} passed {} test of {}",
    config["name"],
    passed_tests,
    total_tests
  );

  return passed_tests == total_tests;
}

fn run_setup(config: &JsonValue) {
  println!("setting up...");
  process::Command::new("sh")
    .arg("-c")
    .arg(config["setup"]["cmd"].as_str().unwrap())
    .output()
    .unwrap();

  if config["setup"]["finished_condition"]["endpoint_reachable"].is_string() {
    let mut request_url = String::from(config["api_hostname"].as_str().unwrap());
    request_url.push_str(config["setup"]["finished_condition"]["endpoint_reachable"].as_str().unwrap());
    loop {
      match reqwest::blocking::Client::new().get(&request_url).send() {
        Ok(_) => break,
        Err(_) => continue,
      }
    }
  }

  println!("setup completed");
}

fn run_cleanup(config: &JsonValue) {
  println!("cleaning up...");
  process::Command::new("sh")
    .arg("-c")
    .arg(config["cleanup"]["cmd"].as_str().unwrap())
    .output()
    .unwrap();
  println!("cleanup completed");
}

fn run_test(test: &JsonValue, config: &JsonValue, config_file: & JsonValue) -> TestOutcomes {
  println!("\nrun test {}", test["name"]);

  let response = run_test_http_request(test, config, config_file);

  //println!("response of test: {}", response);
  //println!("expected outcome: {}", json::stringify(test["expected_outcome"].clone()));

  let mut passed = true;

  let response_status_code = response.status().as_u16();
  let response_body = response.text().unwrap();

  if !test["expected_outcome"]["body_equals"].is_empty() {
    if response_body != json::stringify(test["expected_outcome"]["body_equals"].clone()) {
      println!("reponse body of\n{}\ndidnt match expected outcome\n{}", response_body, json::stringify(test["expected_outcome"]["body_equals"].clone()));
      passed = false;
    }
  }

  if !test["expected_outcome"]["status_code_equals"].is_empty() {
    if response_status_code != test["expected_outcome"]["status_code_equals"].as_u16().unwrap() {
      println!("reponse status code of {} didnt match expected outcome {}", response_status_code, test["expected_outcome"]["status_code_equals"].as_u16().unwrap());
      println!("response body was {}", response_body);
      passed = false;
    }
  }

  if passed {
    println!("passed");
    return TestOutcomes::Passed
  } else {
    println!("failed");
    return TestOutcomes::Failed;
  }
}

fn run_test_before_tasks<'a>(test: &'a JsonValue, config: &'a JsonValue, config_file: &'a JsonValue) -> HashMap<&'a str, String> {
  return test["before"]
    .members()
    .map(|x| run_task(x.as_str().unwrap(), config, config_file))
    .collect();
}

fn run_task<'a>(task: &'a str, config: &'a JsonValue, config_file: &JsonValue) -> (&'a str, String) {
  let mut body: Option<&JsonValue> = None;
  if config_file["tasks"][task]["body"].is_object() {
    body = Some(&config_file["tasks"][task]["body"]);
  }
  
  let response = send_http_request(
    config,
    config_file["tasks"][task]["method"].as_str().unwrap(),
    config_file["tasks"][task]["endpoint"].as_str().unwrap(),
    body,
    None,
    None
  );

  return (task, response.text().unwrap());
}

fn run_test_http_request(test: &JsonValue, config: &JsonValue, config_file: & JsonValue) -> reqwest::blocking::Response {
  let before_task_results: HashMap<&str, String> = run_test_before_tasks(test, config, config_file);

  let mut body: Option<&JsonValue> = None;
  if test["body"].is_object() {
    body = Some(&test["body"]);
  }

  return send_http_request(
    config,
    test["method"].as_str().unwrap(),
    test["endpoint"].as_str().unwrap(),
    body,
    Some(&test["cookies"]),
    Some(before_task_results)
  );
}

fn send_http_request(config: &JsonValue, method: &str, endpoint: &str, body: Option<&JsonValue>, cookies: Option<&JsonValue>, before_task_results: Option<HashMap<&str, String>>) -> reqwest::blocking::Response {
  let mut request_url = String::from(config["api_hostname"].as_str().unwrap());
  request_url.push_str(endpoint);

  let client = reqwest::blocking::Client::new();
  
  let mut cookie_string = String::new();
  if cookies.is_some() {
    cookie_string = parse_cookies(cookies.unwrap(), before_task_results.unwrap());
  }
  
  return match method {
    "GET" => {
      client
        .get(request_url)
        .header(reqwest::header::COOKIE, cookie_string)
        .send()
        .unwrap()
    },
    "POST" => {
      client
        .post(request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::COOKIE, cookie_string)
        .body(body.unwrap_or(&json::object!{}).to_string())
        .send()
        .unwrap()
    },
    "PUT" => {
      client
        .put(request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::COOKIE, cookie_string)
        .body(body.unwrap_or(&json::object!{}).to_string())
        .send()
        .unwrap()
    },
    _ => panic!("tried to send http request with unrecognized method {}", method)
  };
}

fn parse_cookies(cookies: &JsonValue, before_task_results: HashMap<&str, String>) -> String {
  return cookies
    .entries()
    .map(|cookie| {
      let mut output = String::new();
      output.push_str(cookie.0);
      output.push_str("=");

      let cookie_value = cookie.1.as_str().unwrap();

      if cookie_value.starts_with("$") {
        let before_task_results = before_task_results.clone();
        let values = before_task_results.get(cookie_value.replace("$", "").split(".").collect::<Vec<&str>>()[0]).unwrap();
        let values = json::parse(values).unwrap();
        output.push_str(values[cookie_value.replace("$", "").split(".").collect::<Vec<&str>>()[1]].as_str().unwrap());
      } else {
        output.push_str(cookie_value);
      }
      
      return output;
    })
    .collect::<Vec<String>>()
    .join("; ");
}