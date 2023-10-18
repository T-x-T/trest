use crate::{Task, Config, http_request};

pub fn run(config: &Config, task: &Task, task_name: &str) -> String {
  let response = http_request::send(
    config,
    &task.method,
    &task.endpoint,
    task.body.as_deref(),
    None,
    None
  );

  let response_status = response.status();
  let response_body = response.into_string().unwrap_or_default();

  if response_status >= 400 {
    println!("Task \x1b[96m{task_name}\x1b[0m got an error while trying to send a web request:\n\x1b[91m{response_body}\x1b[0m");
  }

  return response_body;
}