use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;

use crate::Config;

pub fn send(config: &Config, method: &str, endpoint: &str, body: Option<&str>, cookies: Option<&LinkedHashMap<String, String>>, before_task_results: Option<&HashMap<String, String>>) -> ureq::Response {
  let mut request_url = String::from(&config.api_hostname);
  request_url.push_str(endpoint); 
  
  let cookie_string = if cookies.is_some() {
    parse_cookies(cookies.unwrap(), before_task_results.unwrap())
  } else {
    String::new()
  };

  let cookie_string = cookie_string.as_str();
  
  let res = match method {
    "GET" => {
      ureq::get(request_url.as_str())
        .set("Cookie", cookie_string)
        .call()
    },
    "DELETE" => {
      ureq::delete(request_url.as_str())
        .set("Cookie", cookie_string)
        .call()
    },
    "POST" => {
      ureq::post(request_url.as_str())
        .set("Content-Type", "application/json")
        .set("Cookie", cookie_string)
        .send_string(body.unwrap_or(""))
    },
    "PUT" => {
      ureq::put(request_url.as_str())
        .set("Content-Type", "application/json")
        .set("Cookie", cookie_string)
        .send_string(body.unwrap_or(""))
    },
    _ => panic!("tried to send http request with unrecognized method {method}"),
  };

  return match res {
    Ok(x) => x,
    Err(e) => e.into_response().unwrap_or(ureq::Response::new(999, "", "").unwrap())
  }
}

fn parse_cookies(cookies: &LinkedHashMap<String, String>, before_task_results: &HashMap<String, String>) -> String {
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