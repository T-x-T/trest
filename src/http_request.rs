use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;

use crate::Config;

pub fn send(config: &Config, method: &str, endpoint: &str, body: Option<&str>, cookies: Option<&LinkedHashMap<String, String>>, before_task_results: Option<HashMap<&str, String>>) -> Result<reqwest::blocking::Response, reqwest::Error> {
  let mut request_url = String::from(&config.api_hostname);
  request_url.push_str(endpoint); 

  let client = reqwest::blocking::Client::new();
  
  let mut cookie_string = String::new();
  if cookies.is_some() {
    cookie_string = parse_cookies(cookies.unwrap(), &before_task_results.unwrap());
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
        .body(body.unwrap_or("").to_string())
        .send()?
    },
    "PUT" => {
      client
        .put(request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::COOKIE, cookie_string)
        .body(body.unwrap_or("").to_string())
        .send()?
    },
    _ => panic!("tried to send http request with unrecognized method {method}"),
  });
}

fn parse_cookies(cookies: &LinkedHashMap<String, String>, before_task_results: &HashMap<&str, String>) -> String {
  return cookies
    .iter()
    .map(|(key, value)| {
      let mut output = String::new();
      output.push_str(key);
      output.push('=');

      if value.starts_with('$') {
        let before_task_results = before_task_results.clone();
        let values = before_task_results.get(value.replace('$', "").split('.').collect::<Vec<&str>>()[0]).unwrap();
        let values = jzon::parse(values).unwrap();
        output.push_str(values[value.replace('$', "").split('.').collect::<Vec<&str>>()[1]].as_str().unwrap_or(""));
      } else {
        output.push_str(value);
      }
      
      return output;
    })
    .collect::<Vec<String>>()
    .join("; ");
}