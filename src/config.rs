use std::process;
use crate::{Config, ConfigFile, test};

pub fn run(config: &Config, config_file: &ConfigFile) -> bool {
  println!("Running config \x1b[96m{}\x1b[0m: \x1b[96m{}\x1b[0m", config.name, config.description);
  
  let mut test_outcomes: Vec<test::TestResults> = Vec::new();

  for test_chain in config_file.tests.iter() {
    println!("\n--------------\nrunning test chain \x1b[96m{}\x1b[0m", test_chain.name);
    run_setup(config);
  
    let mut test_chain_outcomes: Vec<test::TestResults> = test_chain.tests
      .iter()
      .map(|test| test::run(&test, config, config_file, &test_chain.name))
      .collect();

    test_outcomes.append(&mut test_chain_outcomes);
  
    if !config.cleanup.cmd.is_empty() {
      run_cleanup(&config.cleanup.cmd);
    }
  }

  let passed_tests = test_outcomes.iter().filter(|x| **x == test::TestResults::Passed).collect::<Vec<_>>().len();
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
      test::TestResults::Passed => continue,
      test::TestResults::Failed(x) => println!("{x}"),
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
      match reqwest::blocking::Client::new().get(&request_url).send() {
        Ok(_) => break,
        Err(_) => continue,
      }
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