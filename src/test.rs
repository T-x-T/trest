use std::collections::HashMap;
use json::{self, JsonValue};

use crate::http_request;
use crate::task;

#[derive(PartialEq)]
pub enum TestOutcomes {
  Passed, 
  Failed(String)
}

pub fn run(test: &JsonValue, config: &JsonValue, config_file: &JsonValue, test_chain_name: &str) -> TestOutcomes {
  print!("running test \x1b[96m{}\x1b[0m: ", test["name"]);

  let mut failure_message = format!("Test \x1b[96m{}\x1b[0m: \x1b[96m{}\x1b[0m \x1b[91mfailed\x1b[0m:\n", test_chain_name, test["name"]);
  let mut passed = true;
  let response = run_test_http_request(test, config, config_file);

  if response.is_err() {
    failure_message.push_str(format!("\x1b[91mgot an error while trying to perform web request: {}\n\x1b[0m", response.as_ref().err().unwrap().to_string()).as_str());
    passed = false;
  } else {
    let response_status_code = response.as_ref().unwrap().status().as_u16();
    let response_body = response.unwrap().text().unwrap();
  
    if !test["expected_outcome"]["body_equals"].is_empty() {
      if response_body != json::stringify(test["expected_outcome"]["body_equals"].clone()) {
        failure_message.push_str(format!("\x1b[91mreponse body of\n{}\ndidnt match expected outcome\n{}\n\x1b[0m", response_body, json::stringify(test["expected_outcome"]["body_equals"].clone())).as_str());
        passed = false;
      }
    }
  
    if !test["expected_outcome"]["status_code_equals"].is_empty() {
      if response_status_code != test["expected_outcome"]["status_code_equals"].as_u16().unwrap() {
        failure_message.push_str(format!("\x1b[91mreponse status code of {} didnt match expected outcome {}\n\x1b[0m", response_status_code, test["expected_outcome"]["status_code_equals"].as_u16().unwrap()).as_str());
        failure_message.push_str(format!("\x1b[95mresponse body was {}\n\x1b[0m", response_body).as_str());
        passed = false;
      }
    }
  }

  if passed {
    println!("\x1b[92mpassed\x1b[0m");
    return TestOutcomes::Passed
  } else {
    println!("\x1b[91mfailed\x1b[0m");
    failure_message.push('\n');
    return TestOutcomes::Failed(failure_message);
  }
}

fn run_test_http_request(test: &JsonValue, config: &JsonValue, config_file: & JsonValue) -> Result<reqwest::blocking::Response, reqwest::Error> {
  let before_task_results: HashMap<&str, String> = run_test_before_tasks(test, config, config_file);

  let mut body: Option<&JsonValue> = None;
  if test["body"].is_object() {
    body = Some(&test["body"]);
  }

  return http_request::send(
    config,
    test["method"].as_str().unwrap(),
    test["endpoint"].as_str().unwrap(),
    body,
    Some(&test["cookies"]),
    Some(before_task_results)
  );
}

fn run_test_before_tasks<'a>(test: &'a JsonValue, config: &'a JsonValue, config_file: &'a JsonValue) -> HashMap<&'a str, String> {
  return test["before"]
    .members()
    .map(|x| task::run(x.as_str().unwrap(), config, config_file))
    .collect();
}