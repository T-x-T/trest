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