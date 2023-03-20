use std::{env, fs, process, collections::HashMap};

use json;

fn main() {
  println!("Starting trest");
  let config_file = json::parse(&get_config_file()).expect("failed to parse config");
  println!("Config file read");
  println!("There are {} configs to run", config_file["configs"].len());

  for config in config_file["configs"].members() {
    println!("Running config {}: {}", config["name"], config["description"]);

    println!("setting up...");
    println!("{}", 
      String::from_utf8(
        process::Command::new("sh")
          .arg("-c")
          .arg(config["setup"]["cmd"].as_str().unwrap())
          .output()
          .unwrap()
          .stderr
      ).unwrap()
    );
    println!("setup completed");

    for test in config_file["tests"].members() {
      println!("run test {}: {}", test["name"], test["description"]);

      let mut before_task_results: HashMap<&str, String> = HashMap::new();

      for task in test["before"].members() {
        let task_obj = config_file["tasks"][task.as_str().unwrap()].clone();
        println!("run task {} before test {}", task, test["name"]);
        let response: String = match task_obj["method"].as_str().unwrap() {
          "POST" => {
            let mut request_url = String::from(config["api_hostname"].as_str().unwrap());
            request_url.push_str(task_obj["endpoint"].as_str().unwrap());

            let client = reqwest::blocking::Client::new();
            let body = json::object!{
              name: "admin",
              secret: "changeme"
            };
            client.post(request_url).header(reqwest::header::CONTENT_TYPE, "application/json").body(json::stringify(body)).send().unwrap().text().unwrap()
          },
          _ => panic!("task {} tried to use unrecognized method {}", task, task_obj["method"])
        };
        before_task_results.insert(task.as_str().unwrap(), response);
      }

      println!("before tasks completed: {:?}", before_task_results);

      let response: String = match test["method"].as_str().unwrap() {
        "GET" => {
          let mut request_url = String::from(config["api_hostname"].as_str().unwrap());
          request_url.push_str(test["endpoint"].as_str().unwrap());

          let client = reqwest::blocking::Client::new();
          let mut cookies = String::new();
          for cookie in test["cookies"].entries() {
            println!("set cookie {:?}", cookie);
            cookies.push_str(cookie.0);
            cookies.push_str("=");
            if cookie.1.as_str().unwrap().starts_with("$") {
              let values = before_task_results.get(cookie.1.as_str().unwrap().replace("$", "").split(".").collect::<Vec<&str>>()[0]).unwrap();
              let values = json::parse(values).unwrap();
              cookies.push_str(values[cookie.1.as_str().unwrap().replace("$", "").split(".").collect::<Vec<&str>>()[1]].as_str().unwrap());
            } else {
              cookies.push_str(cookie.1.as_str().unwrap());
            }
            cookies.push_str("; ");
          }
          client.get(request_url).header(reqwest::header::COOKIE, cookies).send().unwrap().text().unwrap()
        },
        _ => panic!("test {} tried to use unrecognized method {}", test["name"], test["method"])
      };

      println!("response of test: {}", response);
    }

    if config["cleanup"]["cmd"].is_string() {
      println!("cleaning up...");
      println!("{}", 
      String::from_utf8(
        process::Command::new("sh")
          .arg("-c")
          .arg(config["cleanup"]["cmd"].as_str().unwrap())
          .output()
          .unwrap()
          .stderr
      ).unwrap()
    );
      println!("cleanup completed");

    }
  }
}

fn get_config_file() -> String {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("failed to get config file path");
  }
  return fs::read_to_string(args[1].clone()).expect("failed to read config file");
}