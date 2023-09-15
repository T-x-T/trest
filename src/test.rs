use std::collections::HashMap;

use crate::{task, http_request, Test, Config, ConfigFile};

#[derive(PartialEq)]
pub enum TestResults {
  Passed, 
  Failed(String)
}

pub fn run(test: &Test, config: &Config, config_file: &ConfigFile, test_chain_name: &str) -> TestResults {
  print!("running test \x1b[96m{}\x1b[0m: ", test.name);

  let mut failure_message = format!("Test \x1b[96m{}\x1b[0m: \x1b[96m{}\x1b[0m \x1b[91mfailed\x1b[0m:\n", test_chain_name, test.name);
  let mut passed = true;
  let response = run_test_http_request(test, config, config_file);

  if response.is_err() {
    failure_message.push_str(format!("\x1b[91mgot an error while trying to perform web request: {}\n\x1b[0m", response.as_ref().err().unwrap().to_string()).as_str());
    passed = false;
  } else {
    let response = response.unwrap();
    let response_status_code = response.status();
    let respoonse_content_type = String::from(response.content_type());
    let response_body = response.into_string().unwrap();

    if test.expected_outcome.body_equals.is_some() {
      if respoonse_content_type == "application/json" && (jzon::parse(&response_body) != jzon::parse(test.expected_outcome.body_equals.as_ref().unwrap())) {
        failure_message.push_str(format!("\x1b[91mreponse body of\n{}\ndidnt match expected outcome\n{}\n\x1b[0m", response_body, test.expected_outcome.body_equals.as_ref().unwrap()).as_str());
        passed = false;
      } else if response_body != response_body {
        failure_message.push_str(format!("\x1b[91mreponse body of\n{}\ndidnt match expected outcome\n{}\n\x1b[0m", response_body, test.expected_outcome.body_equals.as_ref().unwrap()).as_str());
        passed = false;
      }
    }

    if test.expected_outcome.status_code_equals.is_some()
    && response_status_code as usize != test.expected_outcome.status_code_equals.unwrap() {
      failure_message.push_str(format!("\x1b[91mreponse status code of {} didnt match expected outcome {}\n\x1b[0m", response_status_code, test.expected_outcome.status_code_equals.unwrap()).as_str());
      failure_message.push_str(format!("\x1b[95mresponse body was {response_body}\n\x1b[0m").as_str());
      passed = false;
    }
  }

  if passed {
    println!("\x1b[92mpassed\x1b[0m");
    return TestResults::Passed
  }
    
  println!("\x1b[91mfailed\x1b[0m");
  failure_message.push('\n');
  return TestResults::Failed(failure_message);
}

fn run_test_http_request(test: &Test, config: &Config, config_file: &ConfigFile) -> Result<ureq::Response, ureq::Error> {
  let before_task_results: HashMap<&str, String> = run_test_before_tasks(test, config, config_file);

  return http_request::send(
    config,
    test.method.as_str(),
    test.endpoint.as_str(),
    test.body.as_deref(),
    test.cookies.as_ref(),
    Some(before_task_results),
  );
}

fn run_test_before_tasks<'a>(test: &'a Test, config: &'a Config, config_file: &'a ConfigFile) -> HashMap<&'a str, String> {
  if test.before.is_none() {
    return HashMap::new();
  }

  return test.before.as_ref().unwrap()
    .iter()
    .map(|x| {
      let res = task::run(config, config_file.tasks.get(x).expect(format!("test {x} not found").as_str()), x);
      (x.as_str(), res)
    })
    .collect();
}