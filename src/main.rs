#![deny(
  clippy::pedantic,
)]

#![allow(
  clippy::needless_return,
  clippy::wildcard_imports,
  clippy::unnecessary_unwrap,
  clippy::module_name_repetitions,
  clippy::needless_return,
)]

use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::{env, fs, process};

mod http_request;
mod test;
mod task;
mod config;

fn main() {
  println!("Starting trest");
  let config_file = get_config_file();
  println!("Config file read");
  println!("There are {} configs to run", config_file.configs.len());

  let mut tests_failed = false;

  for config in config_file.configs.iter() {
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

fn get_config_file() -> ConfigFile {
  let args: Vec<String> = env::args().collect();
  assert!((args.len() >= 2), "failed to get config file path");

  let config_file = fs::read_to_string(args[1].clone()).expect("failed to read config file");
  let json_pared_config_file = jzon::parse(&config_file).expect("failed to parse config");
  let timestamp_before = std::time::Instant::now();
  let parsed_config_file = parse_config_file(&json_pared_config_file);
  println!("Parsed config file in {}ms", timestamp_before.elapsed().as_millis());
  return parsed_config_file;
}

fn parse_config_file(input: &jzon::JsonValue) -> ConfigFile {
  return ConfigFile { 
    configs: input["configs"].members().map(|x| Config {
      name: x["name"].to_string(),
      description: x["description"].to_string(),
      setup: SetupConfig {
        cmd: x["setup"]["cmd"].to_string(),
        finished_condition: SetupFinishedCondition { 
          endpoint_reachable: x["setup"]["finished_condition"]["endpoint_reachable"].to_string(),
        },
      },
      cleanup: CleanupConfig {
        cmd: x["cleanup"]["cmd"].to_string(),
      },
      api_hostname: x["api_hostname"].to_string(),
    }).collect(),
    tasks: input["tasks"].entries().map(|(k, v)| (k.to_string(), Task {
      endpoint: v["endpoint"].to_string(),
      method: v["method"].to_string(),
      body: if v["body"].is_null() { None } else { Some(v["body"].to_string()) },
    })).collect(),
    tests: input["tests"].members().map(|test_chain| TestChain {
      name: test_chain["name"].to_string(),
      defaults: if test_chain["defaults"].is_null() { None } else { Some(PartialTest {
        cookies: if test_chain["defaults"]["cookies"].is_null() { None } else { Some(test_chain["defaults"]["cookies"].entries().map(|(k, v)| (k.to_string(), v.to_string())).collect()) },
        before: if test_chain["defaults"]["before"].is_null() { None } else { Some(test_chain["defaults"]["before"].members().map(std::string::ToString::to_string).collect()) },
      })},
      tests: test_chain["tests"].members().map(|test| Test {
        name: test["name"].to_string(),
        endpoint: test["endpoint"].to_string(),
        method: test["method"].to_string(),
        body: if test["body"].is_null() { None } else { Some(test["body"].to_string() ) },
        cookies: if test["cookies"].is_null() { 
          if test_chain["defaults"]["cookies"].is_null() { None } else { Some(test_chain["defaults"]["cookies"].entries().map(|(k, v)| (k.to_string(), v.to_string())).collect()) }
        } else { Some(test["cookies"].entries().map(|(k, v)| (k.to_string(), v.to_string())).collect()) },
        before: if test["before"].is_null() { 
          if test_chain["defaults"]["before"].is_null() { None } else { Some(test_chain["defaults"]["before"].members().map(std::string::ToString::to_string).collect()) }
        } else { Some(test["before"].members().map(std::string::ToString::to_string).collect()) },
        expected_outcome: TestOutcome {
          status_code_equals: test["expected_outcome"]["status_code_equals"].as_usize(),
          body_equals: if test["expected_outcome"]["body_equals"].is_null() { None } else { Some(test["expected_outcome"]["body_equals"].to_string()) },
        },
      }).collect(),
    }).collect(),
  };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigFile {
  pub configs: LinkedHashSet<Config>,
  pub tasks: LinkedHashMap<String, Task>,
  pub tests: LinkedHashSet<TestChain>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Config {
  pub name: String,
  pub description: String,
  pub setup: SetupConfig,
  pub cleanup: CleanupConfig,
  pub api_hostname: String,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct CleanupConfig {
  pub cmd: String,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct SetupConfig {
  pub cmd: String,
  pub finished_condition: SetupFinishedCondition,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct SetupFinishedCondition {
  pub endpoint_reachable: String,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Task {
  pub endpoint: String,
  pub method: String,
  pub body: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct TestChain {
  pub name: String,
  pub defaults: Option<PartialTest>,
  pub tests: LinkedHashSet<Test>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct PartialTest {
  pub cookies: Option<LinkedHashMap<String, String>>,
  pub before: Option<LinkedHashSet<String>>,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Test {
  pub name: String,
  pub endpoint: String,
  pub method: String,
  pub body: Option<String>,
  pub cookies: Option<LinkedHashMap<String, String>>,
  pub before: Option<LinkedHashSet<String>>,
  pub expected_outcome: TestOutcome,
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, Default)]
pub struct TestOutcome {
  pub status_code_equals: Option<usize>,
  pub body_equals: Option<String>,
}

#[cfg(test)]
mod unit_test {
  use super::*;

  #[test]
  fn parse_config_file() {
    let config_file = ConfigFile {
      configs: [Config {
        name: "backend".to_string(),
        description: "tests for the rest api of the backend".to_string(),
        setup: SetupConfig {
          cmd: "curl -O https://raw.githubusercontent.com/T-x-T/TxTs-Treasury/4b0cb752581eb58f20900bdaccb0caf3f0f6ddf5/docker-compose.yml && docker-compose up -d".to_string(),
          finished_condition: SetupFinishedCondition {
            endpoint_reachable: "/api/v1".to_string(),
          }
        },
        cleanup: CleanupConfig {
          cmd: "docker-compose down && rm docker-compose.yml".to_string(),
        },
        api_hostname: "http://localhost:4000".to_string(),
      }].into_iter().collect(),
      tasks: vec![("login_as_admin".to_string(), Task {
        endpoint: "/api/v1/login".to_string(),
        method: "POST".to_string(),
        body: Some(r#"{"name":"admin","secret":"changeme"}"#.to_string()),
      })].into_iter().collect(),
      tests: [
        TestChain {
          name: "currency".to_string(),
          defaults: None,
          tests: [Test {
            name: "retrieval of all currencies works".to_string(),
					  endpoint: "/api/v1/currencies/all".to_string(),
					  method: "GET".to_string(),
            body: None,
            cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
            before: Some(["login_as_admin".to_string()].into_iter().collect()),
            expected_outcome: TestOutcome {
              status_code_equals: Some(200),
              body_equals: Some(r#"[{"id":0,"name":"Euro","minor_in_mayor":100,"symbol":"€"},{"id":1,"name":"USD","minor_in_mayor":100,"symbol":"$"}]"#.to_string()),
            },
          }].into_iter().collect(),
        },
        TestChain {
          name: "recipients".to_string(),
          defaults: None,
          tests: [
            Test {
              name: "retrieval of all recipients works".to_string(),
              endpoint: "/api/v1/recipients/all".to_string(),
              method: "GET".to_string(),
              body: None,
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: Some(r#"[{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]"#.to_string()),
              },
            },
            Test {
              name: "post of new recipient returns 200".to_string(),
              endpoint: "/api/v1/recipients".to_string(),
              method: "POST".to_string(),
              body: Some(r#"{"name":"test"}"#.to_string()),
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: None,
              },
            },
            Test {
              name: "new recipient created correctly".to_string(),
              endpoint: "/api/v1/recipients/all".to_string(),
              method: "GET".to_string(),
              body: None,
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: Some(r#"[{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]"#.to_string()),
              },
            },
            Test {
              name: "updating name of recipient returns 200".to_string(),
              endpoint: "/api/v1/recipients/1".to_string(),
              method: "PUT".to_string(),
              body: Some(r#"{"name":"edited test"}"#.to_string()),
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: None,
              },
            },
            Test {
              name: "updated recipient name saved correctly".to_string(),
              endpoint: "/api/v1/recipients/all".to_string(),
              method: "GET".to_string(),
              body: None,
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: Some(r#"[{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]"#.to_string()),
              },
            },
          ].into_iter().collect(),
        },
        TestChain {
          name: "recipients_with_defaults".to_string(),
          defaults: Some(PartialTest {
            cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
            before: Some(["login_as_admin".to_string()].into_iter().collect()),
          }),
          tests: [
            Test {
              name: "retrieval of all recipients works".to_string(),
              endpoint: "/api/v1/recipients/all".to_string(),
              method: "GET".to_string(),
              body: None,
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: Some(r#"[{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]"#.to_string()),
              },
            },
            Test {
              name: "post of new recipient returns 200".to_string(),
              endpoint: "/api/v1/recipients".to_string(),
              method: "POST".to_string(),
              body: Some(r#"{"name":"test"}"#.to_string()),
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: None,
              },
            },
            Test {
              name: "new recipient created correctly".to_string(),
              endpoint: "/api/v1/recipients/all".to_string(),
              method: "GET".to_string(),
              body: None,
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: Some(r#"[{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]"#.to_string()),
              },
            },
            Test {
              name: "updating name of recipient returns 200".to_string(),
              endpoint: "/api/v1/recipients/1".to_string(),
              method: "PUT".to_string(),
              body: Some(r#"{"name":"edited test"}"#.to_string()),
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: None,
              },
            },
            Test {
              name: "updated recipient name saved correctly".to_string(),
              endpoint: "/api/v1/recipients/all".to_string(),
              method: "GET".to_string(),
              body: None,
              cookies: Some(vec![("accessToken".to_string(), "$login_as_admin.accessToken".to_string())].into_iter().collect()),
              before: Some(["login_as_admin".to_string()].into_iter().collect()),
              expected_outcome: TestOutcome {
                status_code_equals: Some(200),
                body_equals: Some(r#"[{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]"#.to_string()),
              },
            },
          ].into_iter().collect(),
        },
      ].into_iter().collect(),
    };

    let res = crate::parse_config_file(&jzon::parse(fs::read_to_string("./test/sample_parsed.json").unwrap().as_str()).unwrap());

    assert_eq!(res, config_file);
  }
}