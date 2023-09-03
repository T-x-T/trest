use std::collections::HashMap;
use json::{self, JsonValue};

use crate::http_request;
use crate::task;

#[derive(PartialEq)]
pub enum Outcomes {
  Passed, 
  Failed(String)
}

pub struct Config {
  name: String,
  endpoint: String,
  method: String,
  body: Option<JsonValue>,
  cookies: Option<JsonValue>,
  before: Vec<String>,
  expected_outcome: Vec<ExpectedOutcome>,
}

enum ExpectedOutcome {
  StatusCodeEquals(u16),
  BodyEquals(JsonValue),
}

impl Config {
  pub fn from_config(config: &JsonValue, defaults: Option<&JsonValue>) -> Self {
    let mut expected_outcome: Vec<ExpectedOutcome> = Vec::new();
    if config["expected_outcome"].has_key("status_code_equals") {
      expected_outcome.push(ExpectedOutcome::StatusCodeEquals(config["expected_outcome"]["status_code_equals"].as_u16().unwrap()));
    }
    if config["expected_outcome"].has_key("body_equals") {
      expected_outcome.push(ExpectedOutcome::BodyEquals(config["expected_outcome"]["body_equals"].clone()));
    }

    let before: Vec<String> = if config.has_key("before") {
      config["before"].members().map(std::string::ToString::to_string).collect()
    } else if !defaults.unwrap_or(&json::object!{})["before"].is_empty() {
      defaults.unwrap()["before"].clone().members().map(std::string::ToString::to_string).collect() 
    } else {
      Vec::new()
    };

    return Self {
      name: config["name"].to_string(),
      endpoint: config["endpoint"].to_string(),
      method: config["method"].to_string(),
      body: if config["body"].is_empty() { None } else { Some(config["body"].clone()) },
      cookies: if !config["cookies"].is_empty() { 
        Some(config["cookies"].clone()) 
      } else if !defaults.unwrap_or(&json::object!{})["cookies"].is_empty() {
        Some(defaults.unwrap()["cookies"].clone())
      } else {
        None
      },
      before,
      expected_outcome,
    };
  }
}

pub fn run(test: &Config, config: &JsonValue, config_file: &JsonValue, test_chain_name: &str) -> Outcomes {
  print!("running test \x1b[96m{}\x1b[0m: ", test.name);

  let mut failure_message = format!("Test \x1b[96m{}\x1b[0m: \x1b[96m{}\x1b[0m \x1b[91mfailed\x1b[0m:\n", test_chain_name, test.name);
  let mut passed = true;
  let response = run_test_http_request(test, config, config_file);

  if response.is_err() {
    failure_message.push_str(format!("\x1b[91mgot an error while trying to perform web request: {}\n\x1b[0m", response.as_ref().err().unwrap().to_string()).as_str());
    passed = false;
  } else {
    let response_status_code = response.as_ref().unwrap().status().as_u16();
    let response_body = response.unwrap().text().unwrap();
  
    test.expected_outcome.iter().for_each(|expected_outcome| {
      match expected_outcome {
        ExpectedOutcome::BodyEquals(x) => {
          if json::parse(&response_body).unwrap_or(json::object! {}) != x.clone() {
            failure_message.push_str(format!("\x1b[91mreponse body of\n{}\ndidnt match expected outcome\n{}\n\x1b[0m", response_body, x).as_str());
            passed = false;
          }
        },
        ExpectedOutcome::StatusCodeEquals(x) => {
          if response_status_code != *x {
            failure_message.push_str(format!("\x1b[91mreponse status code of {} didnt match expected outcome {}\n\x1b[0m", response_status_code, x).as_str());
            failure_message.push_str(format!("\x1b[95mresponse body was {}\n\x1b[0m", response_body).as_str());
            passed = false;
          }
        },
      }
    });
  }

  if passed {
    println!("\x1b[92mpassed\x1b[0m");
    return Outcomes::Passed
  }
    
  println!("\x1b[91mfailed\x1b[0m");
  failure_message.push('\n');
  return Outcomes::Failed(failure_message);
}

fn run_test_http_request(test: &Config, config: &JsonValue, config_file: & JsonValue) -> Result<reqwest::blocking::Response, reqwest::Error> {
  let before_task_results: HashMap<&str, String> = run_test_before_tasks(test, config, config_file);

  return http_request::send(
    config,
    test.method.as_str(),
    test.endpoint.as_str(),
    test.body.as_ref(),
    test.cookies.as_ref(),
    Some(before_task_results),
  );
}

fn run_test_before_tasks<'a>(test: &'a Config, config: &'a JsonValue, config_file: &'a JsonValue) -> HashMap<&'a str, String> {
  return test.before
    .iter()
    .map(|x| task::run(x, config, config_file))
    .collect();
}