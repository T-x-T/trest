use std::process;
use std::collections::HashMap;
use crate::{Config, ConfigFile, test};

pub fn run(config: &Config, config_file: &ConfigFile) -> bool {
  println!("Running config \x1b[96m{}\x1b[0m: \x1b[96m{}\x1b[0m", config.name, config.description);
  
  let mut test_outcomes: Vec<Option<String>> = Vec::new();

  for test_chain in config_file.tests.iter() {
    println!("\n--------------\nrunning test chain \x1b[96m{}\x1b[0m", test_chain.name);
    run_setup(config);
  
    let mut test_responses: HashMap<String, jzon::JsonValue> = HashMap::new();

    let mut test_chain_outcomes: Vec<Option<String>> = test_chain.tests
      .iter()
      .map(|test| {
        if test.skip {
          println!("skipping test \x1b[96m{}\x1b[0m", test.name);
          return None;
        }
        print!("running test \x1b[96m{}\x1b[0m: ", test.name);
        let response = test::run_test_http_request(test.clone(), config, config_file, test_responses.clone());
        let response_status_code = response.status();
        let response_content_type = String::from(response.content_type());
        let response_body = response.into_string().unwrap();

        if response_content_type == "application/json" {
          test_responses.insert(test.name.clone(), jzon::parse(response_body.as_str()).unwrap_or(jzon::Null));
        }

        let result = test::check_test_result(test, response_status_code, &response_content_type, &response_body, test_responses.clone());

        return match result {
          test::TestResults::Passed => None,
          test::TestResults::Failed(actual_outcome) => Some(test::stringify_test_outcome(&actual_outcome, &test.expected_outcome, &response_body, &test_chain.name, &test.name)),
        };
      })
      .collect();

    test_outcomes.append(&mut test_chain_outcomes);
  
    if !config.cleanup.cmd.is_empty() {
      run_cleanup(&config.cleanup.cmd);
    }
  }

  let passed_tests = test_outcomes.iter().filter(|x| x.is_none()).collect::<Vec<_>>().len();
  let total_tests = test_outcomes.len();

  println!(
    "\nConfig \x1b[96m{}\x1b[0m passed {} of {} tests",
    config.name,
    passed_tests,
    total_tests
  );

  if passed_tests == total_tests {
    return false;
  }

  for x in test_outcomes {
    match x {
      None => continue,
      Some(x) => println!("{x}"),
    }
  }
  return true;
}

fn run_setup(config: &Config) {
  print!("setting up... ");

  let output = process::Command::new("sh")
    .arg("-c")
    .arg(&config.setup.cmd)
    .output()
    .unwrap();


  if !output.stderr.is_empty() {
    println!("\x1b[91m{}\x1b[0m", String::from_utf8(output.stderr).unwrap_or(String::from("failed to convert stderr of setup")));
  }

  if !config.setup.finished_condition.endpoint_reachable.is_empty() {
    let mut request_url = String::from(&config.api_hostname);
    request_url.push_str(&config.setup.finished_condition.endpoint_reachable);
    
    loop {
      match ureq::get(&request_url).call() {
        Ok(_) => break,
        Err(e) => {
          if e.kind() == ureq::ErrorKind::ConnectionFailed || e.kind() == ureq::ErrorKind::Io {
            continue;
          }
          
          break;
        }
      };
    }
  }

  println!("setup completed");
}

fn run_cleanup(cmd: &str) {
  print!("cleaning up... ");
  process::Command::new("sh")
    .arg("-c")
    .arg(cmd)
    .output()
    .unwrap();
  println!("cleanup completed");
}