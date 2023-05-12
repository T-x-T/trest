use std::collections::HashMap;
use json::{self, JsonValue};

pub fn send(config: &JsonValue, method: &str, endpoint: &str, body: Option<&JsonValue>, cookies: Option<&JsonValue>, before_task_results: Option<HashMap<&str, String>>) -> Result<reqwest::blocking::Response, reqwest::Error> {
  let mut request_url = String::from(config["api_hostname"].as_str().unwrap());
  request_url.push_str(endpoint);

  let client = reqwest::blocking::Client::new();
  
  let mut cookie_string = String::new();
  if cookies.is_some() {
    cookie_string = parse_cookies(cookies.unwrap(), before_task_results.unwrap());
  }
  
  return Ok(match method {
    "GET" => {
      client
        .get(request_url)
        .header(reqwest::header::COOKIE, cookie_string)
        .send()?
    },
    "DELETE" => {
      client
        .delete(request_url)
        .header(reqwest::header::COOKIE, cookie_string)
        .send()?
    },
    "POST" => {
      client
        .post(request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::COOKIE, cookie_string)
        .body(body.unwrap_or(&json::object!{}).to_string())
        .send()?
    },
    "PUT" => {
      client
        .put(request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::COOKIE, cookie_string)
        .body(body.unwrap_or(&json::object!{}).to_string())
        .send()?
    },
    _ => panic!("tried to send http request with unrecognized method {}", method)
  });
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
        output.push_str(values[cookie_value.replace("$", "").split(".").collect::<Vec<&str>>()[1]].as_str().unwrap_or(""));
      } else {
        output.push_str(cookie_value);
      }
      
      return output;
    })
    .collect::<Vec<String>>()
    .join("; ");
}