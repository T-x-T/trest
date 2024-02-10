#[cfg(test)]
mod tests;

use std::collections::HashMap;
use jzon::JsonValue;
use uuid::Uuid;

use crate::{task, http_request, Test, Config, ConfigFile, TestOutcome};

#[derive(PartialEq)]
pub enum TestResults {
  Passed, 
  Failed(TestOutcome)
}

#[allow(clippy::if_same_then_else)]
pub fn check_test_result(test: &Test, response_status_code: u16, response_content_type: &str, response_body: &str, test_responses: HashMap<String, jzon::JsonValue>) -> TestResults {
  let mut actual_outcome: TestOutcome = TestOutcome::default();

  if test.expected_outcome.body_equals.is_some() {
    if response_content_type == "application/json" && !expected_equals_actual_json(jzon::parse(test.expected_outcome.body_equals.as_ref().unwrap()).unwrap_or(JsonValue::new_object()), jzon::parse(response_body).unwrap_or(JsonValue::new_object()), test_responses) {
      actual_outcome.body_equals = Some(String::from(response_body));
    } else if response_content_type != "application/json" && test.expected_outcome.body_equals.as_ref().unwrap() != response_body {
      actual_outcome.body_equals = Some(String::from(response_body));
    }
  }

  if test.expected_outcome.status_code_equals.is_some()
  && response_status_code as usize != test.expected_outcome.status_code_equals.unwrap() {
    actual_outcome.status_code_equals = Some(response_status_code as usize);
  }

  if actual_outcome.body_equals.is_some() || actual_outcome.status_code_equals.is_some() {
    println!("\x1b[91mfailed\x1b[0m");
    return TestResults::Failed(actual_outcome);
  }
  
  println!("\x1b[92mpassed\x1b[0m");
  return TestResults::Passed;
}

pub fn stringify_test_outcome(actual_outcome: &TestOutcome, expected_outcome: &TestOutcome, response_body: &str, test_chain_name: &str, test_name: &str) -> String {
  let mut output_parts: Vec<String> = Vec::new();
  
  if actual_outcome.body_equals.is_some() || actual_outcome.status_code_equals.is_some() {
    output_parts.push(format!("Test \x1b[96m{test_chain_name}\x1b[0m: \x1b[96m{test_name}\x1b[0m \x1b[91mfailed\x1b[0m:\n"));
  }

  if actual_outcome.body_equals.is_some() {
    if jzon::parse(actual_outcome.body_equals.as_ref().unwrap()).is_ok() && jzon::parse(expected_outcome.body_equals.as_ref().unwrap()).is_ok() {
      output_parts.push(format!("\x1b[91mresponse body of\n{}\ndidnt match expected outcome\n{}\n\x1b[0m", jzon::parse(actual_outcome.body_equals.as_ref().unwrap()).unwrap(), jzon::parse(expected_outcome.body_equals.as_ref().unwrap()).unwrap()));
    } else {
      output_parts.push(format!("\x1b[91mresponse body of\n{}\ndidnt match expected outcome\n{}\n\x1b[0m", actual_outcome.body_equals.as_ref().unwrap(), expected_outcome.body_equals.clone().unwrap_or_default()));
    }
  }

  if actual_outcome.status_code_equals.is_some() {
    output_parts.push(format!("\x1b[91mresponse status code of {} didnt match expected outcome {}\n\x1b[0m", actual_outcome.status_code_equals.unwrap(), expected_outcome.status_code_equals.unwrap()));
    
    if actual_outcome.body_equals.is_none() {
      output_parts.push(format!("\x1b[95mresponse body was {response_body}\n\x1b[0m"));
    }
  }

  return output_parts.concat()
}

pub fn run_test_http_request(test: &Test, config: &Config, config_file: &ConfigFile) -> ureq::Response {
  let before_task_results: HashMap<&str, String> = run_test_before_tasks(test, config, config_file);

  return http_request::send(
    config,
    test.method.as_str(),
    test.endpoint.as_str(),
    test.body.as_deref(),
    test.cookies.as_ref(),
    Some(&before_task_results),
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

fn expected_equals_actual_json(expected: JsonValue, actual: JsonValue, test_responses: HashMap<String, jzon::JsonValue>) -> bool {
  if expected.is_string() && expected.as_str().unwrap_or_default() == "%%%ANY%%%" {
    return true;
  }

  if expected.is_string() && expected.as_str().unwrap_or_default() == "%%%ANY_STRING%%%" && actual.is_string() {
    return true;
  }

  if expected.is_string() && expected.as_str().unwrap_or_default() == "%%%ANY_UUID%%%" && actual.is_string() && Uuid::parse_str(actual.to_string().as_str()).is_ok() {
    return true;
  }

  if expected.as_str().unwrap_or_default().starts_with("%%%[[[") {
    let start = expected.as_str().unwrap_or_default().replace("%%%", "").replace("[[[", "").replace("]]]", "");
    let key = start.split("...").collect::<Vec<&str>>()[0].to_string();
    let index = start.split("...").collect::<Vec<&str>>()[1].to_string();
    let value = test_responses.get(&key).unwrap_or(&jzon::Null).get(&index).unwrap_or(&jzon::Null);

    if *value == actual {
      return true;
    }
  }

  if !expected.is_array() && !expected.is_object() {
    return expected == actual;
  }

  let default_vec: Vec<JsonValue> = Vec::new();
  let default_json: JsonValue = JsonValue::Null;

  if expected.is_array() {
    if expected.as_array().unwrap_or(&default_vec).len() != actual.as_array().unwrap_or(&default_vec).len() {
      return false;
    }

    for i in 0..expected.as_array().unwrap_or(&default_vec).len() {
      if !expected_equals_actual_json(expected.as_array().unwrap_or(&default_vec).get(i).unwrap_or(&default_json).clone(), actual.as_array().unwrap_or(&default_vec).get(i).unwrap_or(&default_json).clone(), test_responses.clone()) {
        return false;
      }
    }
  }

  let default_object: jzon::object::Object = jzon::object::Object::new();

  if expected.is_object() {
    if expected.as_object().unwrap_or(&default_object).len() != actual.as_object().unwrap_or(&default_object).len() {
      return false;
    }

    for (expected_key, expected_value) in expected.as_object().unwrap_or(&default_object).clone().into_iter() {
      let actual_value = actual.as_object().unwrap_or(&default_object).get(&expected_key).unwrap_or(&default_json).clone();

      if !expected_equals_actual_json(expected_value, actual_value, test_responses.clone()) {
        return false;
      }
    }
  }

  return true;
}