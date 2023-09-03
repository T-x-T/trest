use json::{self, JsonValue};
use crate::http_request;

pub fn run<'a>(task: &'a str, config: &'a JsonValue, config_file: &JsonValue) -> (&'a str, String) {
  let mut body: Option<&JsonValue> = None;
  if config_file["tasks"][task]["body"].is_object() {
    body = Some(&config_file["tasks"][task]["body"]);
  }
  
  let response = http_request::send(
    config,
    config_file["tasks"][task]["method"].as_str().unwrap(),
    config_file["tasks"][task]["endpoint"].as_str().unwrap(),
    body,
    None,
    None
  );

  if response.is_err() {
    println!("Task \x1b[96m{}\x1b[0m got an error while trying to send a web request:\n\x1b[91m{}\x1b[0m", task, response.as_ref().err().unwrap().to_string());
  }

  return (task, response.unwrap().text().unwrap());
}