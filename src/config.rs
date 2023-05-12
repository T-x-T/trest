use std::process;
use json::{self, JsonValue};
use crate::test::{self, TestOutcomes, TestConfig};

pub fn run(config: &JsonValue, config_file: &JsonValue) -> bool {
  println!("Running config \x1b[96m{}\x1b[0m: \x1b[96m{}\x1b[0m", config["name"], config["description"]);
  
  let mut test_outcomes: Vec<TestOutcomes> = Vec::new();

  for test_chain in config_file["tests"].members() {
    println!("\n--------------\nrunning test chain \x1b[96m{}\x1b[0m", test_chain["name"]);
    run_setup(config);
  
    let mut test_chain_outcomes: Vec<TestOutcomes> = test_chain["tests"]
      .members()
      .map(|x| test::run(TestConfig::from_config(x, if test_chain.has_key("defaults") { Some(&test_chain["defaults"]) } else { None }), config, config_file, test_chain["name"].as_str().unwrap()))
      .collect();

    test_outcomes.append(&mut test_chain_outcomes);
  
    if config["cleanup"]["cmd"].is_string() {
      run_cleanup(config);
    }
  }

  let passed_tests = test_outcomes.iter().filter(|x| **x == TestOutcomes::Passed).collect::<Vec<_>>().len();
  let total_tests = test_outcomes.len();

  println!(
    "\nConfig \x1b[96m{}\x1b[0m passed {} of {} tests",
    config["name"],
    passed_tests,
    total_tests
  );

  if passed_tests != total_tests {
    test_outcomes.iter().for_each(|x| {
      match x {
        TestOutcomes::Passed => return,
        TestOutcomes::Failed(x) => println!("{}", x),
      }
    });
    return true;
  } else {
    return false;
  }

}

fn run_setup(config: &JsonValue) {
  print!("setting up... ");

  let output = process::Command::new("sh")
    .arg("-c")
    .arg(config["setup"]["cmd"].as_str().unwrap())
    .output()
    .unwrap();


  if output.stderr.len() > 0 {
    println!("\x1b[91m{}\x1b[0m", String::from_utf8(output.stderr).unwrap_or(String::from("failed to convert stderr of setup")));
  }

  if config["setup"]["finished_condition"]["endpoint_reachable"].is_string() {
    let mut request_url = String::from(config["api_hostname"].as_str().unwrap());
    request_url.push_str(config["setup"]["finished_condition"]["endpoint_reachable"].as_str().unwrap());
    loop {
      match reqwest::blocking::Client::new().get(&request_url).send() {
        Ok(_) => break,
        Err(_) => continue,
      }
    }
  }

  println!("setup completed");
}

fn run_cleanup(config: &JsonValue) {
  print!("cleaning up... ");
  process::Command::new("sh")
    .arg("-c")
    .arg(config["cleanup"]["cmd"].as_str().unwrap())
    .output()
    .unwrap();
  println!("cleanup completed");
}