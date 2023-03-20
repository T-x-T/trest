use std::{env, fs, process, collections::HashMap};

use json::{self, JsonValue};

fn main() {
  println!("Starting trest");
  let config_file = json::parse(&get_config_file()).expect("failed to parse config");
  println!("Config file read");
  println!("There are {} configs to run", config_file["configs"].len());

  config_file["configs"]
    .members()
    .for_each(|x| run_config(x, &config_file));
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

fn run_config(config: &JsonValue, config_file: &JsonValue) {
  println!("Running config {}: {}", config["name"], config["description"]);
  run_setup(config);

  let test_outcomes: Vec<TestOutcomes> = config_file["tests"]
    .members()
    .map(|x| run_test(x, config, config_file))
    .collect();

  if config["cleanup"]["cmd"].is_string() {
    run_cleanup(config);
  }

  println!(
    "Config {} passed {} test of {}",
    config["name"],
    test_outcomes.iter().filter(|x| **x == TestOutcomes::Passed).collect::<Vec<_>>().len(),
    test_outcomes.len()
  );
}

fn run_setup(config: &JsonValue) {
  println!("setting up...");
  process::Command::new("sh")
    .arg("-c")
    .arg(config["setup"]["cmd"].as_str().unwrap())
    .output()
    .unwrap();
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
  println!("\nrun test {}: {}", test["name"], test["description"]);

  let response: String = run_test_http_request(test, config, config_file);

  println!("response of test: {}", response);
  println!("expected outcome: {}", json::stringify(test["expected_outcome"].clone()));

  if response == json::stringify(test["expected_outcome"].clone()) {
    println!("passed");
    return TestOutcomes::Passed;
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
  let response = send_http_request(
    config,
    config_file["tasks"][task]["method"].as_str().unwrap(),
    config_file["tasks"][task]["endpoint"].as_str().unwrap(),
    None,
    None
  );

  return (task, response);
}

fn run_test_http_request(test: &JsonValue, config: &JsonValue, config_file: & JsonValue) -> String {
  let before_task_results: HashMap<&str, String> = run_test_before_tasks(test, config, config_file);

  return send_http_request(
    config,
    test["method"].as_str().unwrap(),
    test["endpoint"].as_str().unwrap(),
    Some(&test["cookies"]),
    Some(before_task_results)
  );
}

fn send_http_request(config: &JsonValue, method: &str, endpoint: &str, cookies: Option<&JsonValue>, before_task_results: Option<HashMap<&str, String>>) -> String {
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
        .text()
        .unwrap()
    },
    "POST" => {
      client
        .post(request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json::stringify(json::object!{
          name: "admin",
          secret: "changeme"
          }))
        .send()
        .unwrap()
        .text()
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