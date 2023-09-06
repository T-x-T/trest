use crate::{Task, Config, http_request};

pub fn run(config: &Config, task: &Task, task_name: &str) -> String {
  let response = http_request::send(
    config,
    &task.method,
    &task.endpoint,
    task.body.as_ref().map(|x| x.as_str()),
    None,
    None
  );

  if response.is_err() {
    println!("Task \x1b[96m{}\x1b[0m got an error while trying to send a web request:\n\x1b[91m{}\x1b[0m", task_name, response.as_ref().err().unwrap().to_string());
  }

  return response.unwrap().text().unwrap();
}