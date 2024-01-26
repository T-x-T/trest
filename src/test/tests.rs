use super::*;

mod stringify_test_outcome {
	use super::*;

	#[test]
	fn test_ok() {
		let actual_outcome: TestOutcome = Default::default();
		let res = stringify_test_outcome(&actual_outcome, &actual_outcome, "", "test_chain", "test");

		assert_eq!(res, "");
	}

	#[test]
	fn response_body_different() {
		let actual_outcome = TestOutcome {
			body_equals: Some("hello world".to_string()),
			status_code_equals: None,
		};

		let expected_outcome = TestOutcome {
			body_equals: Some("Hello, world!".to_string()),
			status_code_equals: None,
		};

		let res = stringify_test_outcome(&actual_outcome, &expected_outcome, "Hello, world!", "test_chain", "test");

		assert_eq!(res, "Test \u{1b}[96mtest_chain\u{1b}[0m: \u{1b}[96mtest\u{1b}[0m \u{1b}[91mfailed\u{1b}[0m:\n\u{1b}[91mresponse body of\nhello world\ndidnt match expected outcome\nHello, world!\n\u{1b}[0m");
	}

	#[test]
	fn status_code_different() {
		let actual_outcome = TestOutcome {
			body_equals: None,
			status_code_equals: Some(400),
		};

		let expected_outcome = TestOutcome {
			body_equals: None,
			status_code_equals: Some(200),
		};

		let res = stringify_test_outcome(&actual_outcome, &expected_outcome, "Hello, world!", "test_chain", "test");

		assert_eq!(res, "Test \u{1b}[96mtest_chain\u{1b}[0m: \u{1b}[96mtest\u{1b}[0m \u{1b}[91mfailed\u{1b}[0m:\n\u{1b}[91mresponse status code of 400 didnt match expected outcome 200\n\u{1b}[0m\u{1b}[95mresponse body was Hello, world!\n\u{1b}[0m");
	}

	#[test]
	fn status_code_and_body_different() {
		let actual_outcome = TestOutcome {
			body_equals: Some("hello world".to_string()),
			status_code_equals: Some(400),
		};

		let expected_outcome = TestOutcome {
			body_equals: Some("Hello, world!".to_string()),
			status_code_equals: Some(200),
		};

		let res = stringify_test_outcome(&actual_outcome, &expected_outcome, "Hello, world!", "test_chain", "test");

		assert_eq!(res, "Test \u{1b}[96mtest_chain\u{1b}[0m: \u{1b}[96mtest\u{1b}[0m \u{1b}[91mfailed\u{1b}[0m:\n\u{1b}[91mresponse body of\nhello world\ndidnt match expected outcome\nHello, world!\n\u{1b}[0m\u{1b}[91mresponse status code of 400 didnt match expected outcome 200\n\u{1b}[0m");
	}
}

mod expected_equals_actual_json {
	use super::*;
	use jzon::*;

	#[test]
	fn same_null() {
		let expected: JsonValue = JsonValue::Null;
		let actual: JsonValue = JsonValue::Null;

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn same_number() {
		let expected: JsonValue = JsonValue::Number(5.into());
		let actual: JsonValue = JsonValue::Number(5.into());

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn same_number_0() {
		let expected: JsonValue = JsonValue::Number(0.into());
		let actual: JsonValue = JsonValue::Number(0.into());

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn different_number() {
		let expected: JsonValue = JsonValue::Number(5.into());
		let actual: JsonValue = JsonValue::Number(4.into());

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn same_string() {
		let expected: JsonValue = JsonValue::String("Hello World!".to_string());
		let actual: JsonValue = JsonValue::String("Hello World!".to_string());

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn different_string() {
		let expected: JsonValue = JsonValue::String("Hello Universe!".to_string());
		let actual: JsonValue = JsonValue::String("Hello World!".to_string());

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn empty_array() {
		let expected: JsonValue = JsonValue::Array(Vec::new());
		let actual: JsonValue = JsonValue::Array(Vec::new());

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn array_of_strings() {
		let expected: JsonValue = JsonValue::Array(vec![JsonValue::String("Hello World!".to_string()), JsonValue::String("Hello Universe!".to_string())]);
		let actual: JsonValue = JsonValue::Array(vec![JsonValue::String("Hello World!".to_string()), JsonValue::String("Hello Universe!".to_string())]);

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn different_array_of_strings() {
		let expected: JsonValue = JsonValue::Array(vec![JsonValue::String("Moin World!".to_string()), JsonValue::String("Hello Universe!".to_string())]);
		let actual: JsonValue = JsonValue::Array(vec![JsonValue::String("Hello World!".to_string()), JsonValue::String("Hello Universe!".to_string())]);

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn array_missing_element() {
		let expected: JsonValue = JsonValue::Array(vec![JsonValue::String("Hello World!".to_string()), JsonValue::String("Hello Universe!".to_string())]);
		let actual: JsonValue = JsonValue::Array(vec![JsonValue::String("Hello World!".to_string())]);

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn array_additional_element() {
		let expected: JsonValue = JsonValue::Array(vec![JsonValue::String("Hello World!".to_string())]);
		let actual: JsonValue = JsonValue::Array(vec![JsonValue::String("Hello World!".to_string()), JsonValue::String("Hello Universe!".to_string())]);

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn empty_object() {
		let expected: JsonValue = object! {};
		let actual: JsonValue = object! {};

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn object() {
		let expected: JsonValue = object! { number: 42, string: "Hello World!", boolean: true};
		let actual: JsonValue = object! { number: 42, string: "Hello World!", boolean: true};

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn object_different_order() {
		let expected: JsonValue = object! { string: "Hello World!", number: 42, boolean: true};
		let actual: JsonValue = object! { number: 42, string: "Hello World!", boolean: true};

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn object_missing_key() {
		let expected: JsonValue = object! { number: 42, string: "Hello World!", boolean: true};
		let actual: JsonValue = object! { number: 42, string: "Hello World!"};

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn object_additional_key() {
		let expected: JsonValue = object! { number: 42, string: "Hello World!"};
		let actual: JsonValue = object! { number: 42, string: "Hello World!", boolean: true};

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn object_different_key() {
		let expected: JsonValue = object! { number: 42, string: "Hello World!", bool: true};
		let actual: JsonValue = object! { number: 42, string: "Hello World!", boolean: true};

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn object_different_value() {
		let expected: JsonValue = object! { number: 42, string: "Hello World!", boolean: false};
		let actual: JsonValue = object! { number: 42, string: "Hello World!", boolean: true};

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn array_of_objects() {
		let expected: JsonValue = JsonValue::Array(vec![object! { number: 42, string: "Hello World!", boolean: false}, object! { number: 42, string: "Hello World!", boolean: true}]);
		let actual: JsonValue = JsonValue::Array(vec![object! { number: 42, string: "Hello World!", boolean: false}, object! { number: 42, string: "Hello World!", boolean: true}]);

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn complex_object() {
		let expected: JsonValue = object! {
			"configs": [
				{
					"name": "backend",
					"description": "tests for the rest api of the backend",
					"setup": {
						"cmd": "curl -O https://raw.githubusercontent.com/T-x-T/TxTs-Treasury/4b0cb752581eb58f20900bdaccb0caf3f0f6ddf5/docker-compose.yml && docker-compose up -d",
						"finished_condition": {
							"endpoint_reachable": "/api/v1"
						}
					},
					"cleanup": {
						"cmd": "docker-compose down && rm docker-compose.yml"
					},
					"api_hostname": "http://localhost:4000"
				}
			],
			"tasks": {
				"login_as_admin": {
					"endpoint": "/api/v1/login",
					"method": "POST",
					"body": {
						"name": "admin",
						"secret": "changeme"
					}
				}
			},
			"tests": [
				{
					"name": "currency",
					"tests": [
						{
							"name": "retrieval of all currencies works",
							"endpoint": "/api/v1/currencies/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Euro","minor_in_mayor":100,"symbol":"€"},{"id":1,"name":"USD","minor_in_mayor":100,"symbol":"$"}]
							} 
						}
					]
				},
				{
					"name": "recipients",
					"tests": [
						{
							"name": "retrieval of all recipients works",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]
							}
						},
						{
							"name": "post of new recipient returns 200",
							"endpoint": "/api/v1/recipients",
							"method": "POST",
							"body": {
								"name": "test"
							},
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "new recipient created correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]
							}
						},
						{
							"name": "updating name of recipient returns 200",
							"endpoint": "/api/v1/recipients/1",
							"method": "PUT",
							"body": {
								"name":"edited test"
							},
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "updated recipient name saved correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]
							}
						}
					]
				},
				{
					"name": "recipients_with_defaults",
					"defaults": {
						"cookies": {
							"accessToken": "$login_as_admin.accessToken"
						},
						"before": [
							"login_as_admin"
						]
					},
					"tests": [
						{
							"name": "retrieval of all recipients works",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]
							}
						},
						{
							"name": "post of new recipient returns 200",
							"endpoint": "/api/v1/recipients",
							"method": "POST",
							"body": {
								"name": "test"
							},
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "new recipient created correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]
							}
						},
						{
							"name": "updating name of recipient returns 200",
							"endpoint": "/api/v1/recipients/1",
							"method": "PUT",
							"body": {
								"name":"edited test"
							},
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "updated recipient name saved correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]
							}
						}
					]
				}
			]
		};
		let actual: JsonValue = object! {
			"configs": [
				{
					"name": "backend",
					"description": "tests for the rest api of the backend",
					"setup": {
						"cmd": "curl -O https://raw.githubusercontent.com/T-x-T/TxTs-Treasury/4b0cb752581eb58f20900bdaccb0caf3f0f6ddf5/docker-compose.yml && docker-compose up -d",
						"finished_condition": {
							"endpoint_reachable": "/api/v1"
						}
					},
					"cleanup": {
						"cmd": "docker-compose down && rm docker-compose.yml"
					},
					"api_hostname": "http://localhost:4000"
				}
			],
			"tasks": {
				"login_as_admin": {
					"endpoint": "/api/v1/login",
					"method": "POST",
					"body": {
						"name": "admin",
						"secret": "changeme"
					}
				}
			},
			"tests": [
				{
					"name": "currency",
					"tests": [
						{
							"name": "retrieval of all currencies works",
							"endpoint": "/api/v1/currencies/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Euro","minor_in_mayor":100,"symbol":"€"},{"id":1,"name":"USD","minor_in_mayor":100,"symbol":"$"}]
							} 
						}
					]
				},
				{
					"name": "recipients",
					"tests": [
						{
							"name": "retrieval of all recipients works",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]
							}
						},
						{
							"name": "post of new recipient returns 200",
							"endpoint": "/api/v1/recipients",
							"method": "POST",
							"body": {
								"name": "test"
							},
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "new recipient created correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]
							}
						},
						{
							"name": "updating name of recipient returns 200",
							"endpoint": "/api/v1/recipients/1",
							"method": "PUT",
							"body": {
								"name":"edited test"
							},
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "updated recipient name saved correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]
							}
						}
					]
				},
				{
					"name": "recipients_with_defaults",
					"defaults": {
						"cookies": {
							"accessToken": "$login_as_admin.accessToken"
						},
						"before": [
							"login_as_admin"
						]
					},
					"tests": [
						{
							"name": "retrieval of all recipients works",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]
							}
						},
						{
							"name": "post of new recipient returns 200",
							"endpoint": "/api/v1/recipients",
							"method": "POST",
							"body": {
								"name": "test"
							},
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "new recipient created correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]
							}
						},
						{
							"name": "updating name of recipient returns 200",
							"endpoint": "/api/v1/recipients/1",
							"method": "PUT",
							"body": {
								"name":"edited test"
							},
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "updated recipient name saved correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]
							}
						}
					]
				}
			]
		};

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn complex_object_tiny_difference() {
		let expected: JsonValue = object! {
			"configs": [
				{
					"name": "backend",
					"description": "tests for the rest api of the backend",
					"setup": {
						"cmd": "curl -O https://raw.githubusercontent.com/T-x-T/TxTs-Treasury/4b0cb752581eb58f20900bdaccb0caf3f0f6ddf5/docker-compose.yml && docker-compose up -d",
						"finished_condition": {
							"endpoint_reachable": "/api/v1"
						}
					},
					"cleanup": {
						"cmd": "docker-compose down && rm docker-compose.yml"
					},
					"api_hostname": "http://localhost:4000"
				}
			],
			"tasks": {
				"login_as_admin": {
					"endpoint": "/api/v1/login",
					"method": "POST",
					"body": {
						"name": "admin",
						"secret": "changeme"
					}
				}
			},
			"tests": [
				{
					"name": "currency",
					"tests": [
						{
							"name": "retrieval of all currencies works",
							"endpoint": "/api/v1/currencies/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Euro","minor_in_mayor":100,"symbol":"€"},{"id":1,"name":"USD","minor_in_mayor":100,"symbol":"$"}]
							} 
						}
					]
				},
				{
					"name": "recipients",
					"tests": [
						{
							"name": "retrieval of all recipients works",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]
							}
						},
						{
							"name": "post of new recipient returns 200",
							"endpoint": "/api/v1/recipients",
							"method": "POST",
							"body": {
								"name": "test"
							},
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "new recipient created correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]
							}
						},
						{
							"name": "updating name of recipient returns 200",
							"endpoint": "/api/v1/recipients/1",
							"method": "PUT",
							"body": {
								"name":"edited test"
							},
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "updated recipient name saved correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]
							}
						}
					]
				},
				{
					"name": "recipients_with_defaults",
					"defaults": {
						"cookies": {
							"accessToken": "$login_as_admin.accessToken"
						},
						"before": [
							"login_as_admin"
						]
					},
					"tests": [
						{
							"name": "retrieval of all recipients works",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]
							}
						},
						{
							"name": "post of new recipient returns 200",
							"endpoint": "/api/v1/recipients",
							"method": "POST",
							"body": {
								"name": "test"
							},
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "new recipient created correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]
							}
						},
						{
							"name": "updating name of recipient returns 200",
							"endpoint": "/api/v1/recipients/1",
							"method": "PUT",
							"body": {
								"name":"edited test"
							},
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "updated recipient name saved correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]
							}
						}
					]
				}
			]
		};
		let actual: JsonValue = object! {
			"configs": [
				{
					"name": "backendd",
					"description": "tests for the rest api of the backend",
					"setup": {
						"cmd": "curl -O https://raw.githubusercontent.com/T-x-T/TxTs-Treasury/4b0cb752581eb58f20900bdaccb0caf3f0f6ddf5/docker-compose.yml && docker-compose up -d",
						"finished_condition": {
							"endpoint_reachable": "/api/v1"
						}
					},
					"cleanup": {
						"cmd": "docker-compose down && rm docker-compose.yml"
					},
					"api_hostname": "http://localhost:4000"
				}
			],
			"tasks": {
				"login_as_admin": {
					"endpoint": "/api/v1/login",
					"method": "POST",
					"body": {
						"name": "admin",
						"secret": "changeme"
					}
				}
			},
			"tests": [
				{
					"name": "currency",
					"tests": [
						{
							"name": "retrieval of all currencies works",
							"endpoint": "/api/v1/currencies/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Euro","minor_in_mayor":100,"symbol":"€"},{"id":1,"name":"USD","minor_in_mayor":100,"symbol":"$"}]
							} 
						}
					]
				},
				{
					"name": "recipients",
					"tests": [
						{
							"name": "retrieval of all recipients works",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]
							}
						},
						{
							"name": "post of new recipient returns 200",
							"endpoint": "/api/v1/recipients",
							"method": "POST",
							"body": {
								"name": "test"
							},
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "new recipient created correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]
							}
						},
						{
							"name": "updating name of recipient returns 200",
							"endpoint": "/api/v1/recipients/1",
							"method": "PUT",
							"body": {
								"name":"edited test"
							},
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "updated recipient name saved correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"cookies": {
								"accessToken": "$login_as_admin.accessToken"
							},
							"before": [
								"login_as_admin"
							],
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]
							}
						}
					]
				},
				{
					"name": "recipients_with_defaults",
					"defaults": {
						"cookies": {
							"accessToken": "$login_as_admin.accessToken"
						},
						"before": [
							"login_as_admin"
						]
					},
					"tests": [
						{
							"name": "retrieval of all recipients works",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]}]
							}
						},
						{
							"name": "post of new recipient returns 200",
							"endpoint": "/api/v1/recipients",
							"method": "POST",
							"body": {
								"name": "test"
							},
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "new recipient created correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"test","user_id":0,"tag_ids":[]}]
							}
						},
						{
							"name": "updating name of recipient returns 200",
							"endpoint": "/api/v1/recipients/1",
							"method": "PUT",
							"body": {
								"name":"edited test"
							},
							"expected_outcome": {
								"status_code_equals": 200
							}
						},
						{
							"name": "updated recipient name saved correctly",
							"endpoint": "/api/v1/recipients/all",
							"method": "GET",
							"expected_outcome": {
								"status_code_equals": 200,
								"body_equals": [{"id":0,"name":"Default","user_id":null,"tag_ids":[]},{"id":1,"name":"edited test","user_id":0,"tag_ids":[]}]
							}
						}
					]
				}
			]
		};

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn parsed_object() {
		let expected: JsonValue = parse(r#"{"datasets":[{"label":"Earning","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();
		let actual: JsonValue = parse(r#"{"datasets":[{"label":"Earning","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn parsed_object_matches_with_any_placeholder() {
		let expected: JsonValue = parse(r#"{"datasets":[{"label":"%%%ANY%%%","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();
		let actual: JsonValue = parse(r#"{"datasets":[{"label":"Earning","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn parsed_object_matches_with_any_string_placeholder_matches_string() {
		let expected: JsonValue = parse(r#"{"datasets":[{"label":"%%%ANY_STRING%%%","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();
		let actual: JsonValue = parse(r#"{"datasets":[{"label":"Earning","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();

		assert!(expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn parsed_object_matches_with_any_string_placeholder_doesnt_match_number() {
		let expected: JsonValue = parse(r#"{"datasets":[{"label":"%%%ANY_STRING%%%","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();
		let actual: JsonValue = parse(r#"{"datasets":[{"label":100,"data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();

		assert!(!expected_equals_actual_json(expected, actual));
	}

	#[test]
	fn parsed_object_matches_with_any_string_placeholder_doesnt_match_null() {
		let expected: JsonValue = parse(r#"{"datasets":[{"label":"%%%ANY_STRING%%%","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();
		let actual: JsonValue = parse(r#"{"datasets":[{"label":null,"data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.9,"label":"246.90€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Net","data":[{"name":null,"timestamp":"1923-03-01","value":6.0,"label":"3.00€ 3.00$"},{"name":null,"timestamp":"2023-03-01","value":266.0,"label":"246.00€ 20.00$"},{"name":null,"timestamp":"2123-03-01","value":6.0,"label":"3.00€ 3.00$"}]},{"label":"Spending","data":[{"name":null,"timestamp":"2023-03-01","value":-0.9,"label":"-0.90€"}]}]}"#).unwrap();

		assert!(!expected_equals_actual_json(expected, actual));
	}
}