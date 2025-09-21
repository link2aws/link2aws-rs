//! Test harness for test cases defined in JSON files at `tests/data/`.

use serde_json::Value;

use link2aws::{Arn, ArnParts};

/// Represents the different kinds of test failures.
#[allow(clippy::result_large_err)]
#[derive(Debug, PartialEq)]
enum Link2AwsTestError<'a> {
    UnexpectedParsingError {
        input: String,
        expected_arn: String,
        error: link2aws::Error,
    },
    UnexpectedSuccess {
        input: String,
        actual_arn: String,
    },
    MissingConsoleLink {
        input: String,
        actual_arn: Arn<'a>,
        expected_link: String,
    },
    IncorrectConsoleLink {
        input: String,
        arn: Arn<'a>,
        expected_link: String,
        actual_link: String,
    },
    IncorrectRebuiltArn {
        input: String,
        actual_arn: String,
    },
}

/// Verifies that a bad input either doesn't parse or doesn't yield a link.
/// If the ARN parses, then it must still `.build()` the original input.
#[allow(clippy::result_large_err)]
fn run_negative_test<'a>(input: &'a str) -> Result<(), Link2AwsTestError<'a>> {
    let Ok(actual_arn) = Arn::new(input) else {
        return Ok(()); // Test passed (bad ARN detected).
    };

    if input.trim() != actual_arn.build() {
        // Test failed (built ARN doesn't match original input).
        return Err(Link2AwsTestError::IncorrectRebuiltArn {
            input: input.to_string(),
            actual_arn: actual_arn.build(),
        });
    }

    let Some(actual_console) = actual_arn.link() else {
        return Ok(()); // Test passed (good ARN without console link).
    };

    // Test failed (good ARN with console link).
    Err(Link2AwsTestError::UnexpectedSuccess {
        input: input.to_string(),
        actual_arn: actual_console,
    })
}

/// Verifies that a good input parses, yields the right console link,
/// and that the original ARN can be rebuilt from the parsed parts.
#[allow(clippy::result_large_err)]
fn run_positive_test<'a>(
    input: &'a str,
    expected_console: &str,
) -> Result<(), Link2AwsTestError<'a>> {
    let actual_arn = match Arn::new(input) {
        Ok(arn) => arn,
        Err(err) => {
            // Test failed (failed to parse).
            return Err(Link2AwsTestError::UnexpectedParsingError {
                input: input.to_string(),
                expected_arn: expected_console.to_string(),
                error: err,
            });
        }
    };

    let Some(actual_console) = actual_arn.link() else {
        // Test failed (parsed, but failed to generate a link).
        return Err(Link2AwsTestError::MissingConsoleLink {
            input: input.to_string(),
            actual_arn,
            expected_link: expected_console.to_string(),
        });
    };

    if actual_console != expected_console {
        // Test failed (parsed, but generated an incorrect link).
        return Err(Link2AwsTestError::IncorrectConsoleLink {
            input: input.to_string(),
            arn: actual_arn,
            expected_link: expected_console.to_string(),
            actual_link: actual_console,
        });
    }

    if input.trim() != actual_arn.build() {
        // Test failed (built ARN doesn't match original input).
        return Err(Link2AwsTestError::IncorrectRebuiltArn {
            input: input.to_string(),
            actual_arn: actual_arn.build(),
        });
    }

    // Test passed (ARN parsed, correct link generated, ARN rebuilt correctly).
    Ok(())
}

/// Runs a series of tests specified in a JSON string.
///
/// Example JSON:
///
/// ```text
/// {
///   "arn:aws:good-example:...": "https://console.example.com/",
///   "bad-example": null
/// }
/// ```
fn run_tests(json_str: &str) {
    let cases: Value = serde_json::from_str(json_str).unwrap();
    let cases = cases.as_object().unwrap();

    // Run each test case.
    let results = cases.into_iter().map(|(input, expected)| {
        match expected {
            // A string means the input should lead to that console link.
            Value::String(expected_console) => run_positive_test(input, expected_console),

            // Null means the input should fail to parse, or fail to yield a console link.
            Value::Null => run_negative_test(input),

            // Invalid test case - the value should be either a string or null.
            _ => panic!("Bad test case: {:#?} -> {:#?}", input, expected),
        }
    });

    // Report individual test failures.
    let results = results.inspect(|res| {
        if let Err(e) = res {
            eprintln!("Fail: {:#?}", e);
        }
    });

    // Report summary.
    let num_passed = results.filter(|res| res.is_ok()).count();
    let num_total = cases.len();
    eprintln!("{} of {} passed", num_passed, num_total);

    assert!(num_passed == num_total);
}

/// Runs tests from `aws.json`, which contains valid ARNs.
#[test]
fn aws_cases() {
    run_tests(include_str!("data/aws.json"));
}

/// Runs tests from `aws-negative.json`, which contains ARNs that should not yield a console link.
#[test]
fn aws_negative_cases() {
    run_tests(include_str!("data/aws-negative.json"));
}

/// Runs tests from `aws-negative.json`, which contains string-handling related edge cases.
#[test]
fn string_cases() {
    run_tests(include_str!("data/string.json"));
}

/// Runs a hardcoded testcase via `run_positive_test`.
///
/// This mainly exists as a useful template for debugging.
#[test]
fn single_case() {
    let result = run_positive_test(
        "arn:aws:s3:::abc123",
        "https://s3.console.aws.amazon.com/s3/buckets/abc123",
    );
    eprintln!("Result: {:#?}", result);
    assert!(result.is_ok());
}
