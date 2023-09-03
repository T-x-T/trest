#![deny(
  clippy::pedantic,
)]

#![allow(
  clippy::needless_return,
  clippy::wildcard_imports,
  clippy::unnecessary_unwrap,
)]

use std::{env, fs, process};

mod http_request;
mod test;
mod task;
mod config;

fn main() {
  println!("Starting trest");
  let config_file = json::parse(&get_config_file()).expect("failed to parse config");
  println!("Config file read");
  println!("There are {} configs to run", config_file["configs"].len());

  let mut tests_failed = false;

  for config in config_file["configs"].members() {
    if config::run(config, &config_file) {
      tests_failed = true;
    }
  }

  if tests_failed {
    process::exit(1);
  } else {
    process::exit(0);
  }
}

fn get_config_file() -> String {
  let args: Vec<String> = env::args().collect();
  assert!((args.len() >= 2), "failed to get config file path");

  return fs::read_to_string(args[1].clone()).expect("failed to read config file");
}