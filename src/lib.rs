//! Converts ARNs to direct links to the AWS console.
//!
//! # Simple example
//!
//! ```
//! use link2aws::arn_to_link;
//!
//! // https://s3.console.aws.amazon.com/s3/buckets/abc123
//! println!("{}", arn_to_link("arn:aws:s3:::abc123").unwrap());
//! ```
//!
//! # Advanced usage
//!
//! The library consists of an ARN parser, and a console link generator.
//! These two parts may be used separately.
//!
//! * See [`Arn`] for an example of how to just parse an ARN.
//! * See [`ArnParts`] for an example of how to generate a link,
//!   or build an ARN, from your own struct.
//!
//! # Command line tool
//!
//! ```text
//! $ cargo install link2aws
//! $ link2aws arn:aws:s3:::abc123
//! https://s3.console.aws.amazon.com/s3/buckets/abc123
//! ```

mod arn;
mod parts;

use std::fmt;

pub use arn::Arn;
pub use arn::ArnOwned;
pub use parts::ArnParts;

/// Error returned by link2aws when parsing failed, or a link could not be generated.
#[non_exhaustive] // We do not consider adding variants a breaking change.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The ARN string was not parsed because it is too long.
    TooLong,
    /// The ARN string was not parsed because it contains unexpected characters.
    BadCharacters,
    /// The ARN string was not parsed because it is malformed.
    ParseError,
    /// We could not generate a link for the ARN. The ARN may still be valid.
    NoLink,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TooLong => write!(f, "ARN is too long"),
            Error::BadCharacters => write!(f, "ARN contains bad characters"),
            Error::ParseError => write!(f, "ARN is malformed"),
            Error::NoLink => write!(f, "No link available"),
        }
    }
}

impl std::error::Error for Error {}

/// Converts an ARN string to an AWS Console link.
///
/// ```
/// use link2aws::arn_to_link;
///
/// // Example of success.
/// let link = arn_to_link("arn:aws:s3:::abc123").unwrap();
/// assert_eq!(link, "https://s3.console.aws.amazon.com/s3/buckets/abc123");
///
/// // Example of failure (valid format, but no link for this service).
/// let err = arn_to_link("arn:aws:does-not-exist:::example").unwrap_err();
/// assert_eq!(err, link2aws::Error::NoLink);
/// ```
pub fn arn_to_link(arn: &str) -> Result<String, Error> {
    Arn::new(arn)?.link().ok_or(Error::NoLink)
}

/// Unit tests for the public API.
///
/// These tests focus on how the public API can be used from a type system
/// perspective, not on what specific values are returned.
#[cfg(test)]
mod tests {
    use super::*;

    // A valid ARN and the corresponding link.
    const TEST_ARN: &str = "arn:aws:s3:::abc123";
    const TEST_LINK: &str = "https://s3.console.aws.amazon.com/s3/buckets/abc123";

    #[test]
    fn test_simple_api_with_owned_string() {
        let input = String::from(TEST_ARN);
        let link = arn_to_link(&input).unwrap();
        assert_eq!(link, TEST_LINK);
    }

    #[test]
    fn test_simple_api_with_borrowed_str() {
        let link: String = arn_to_link(TEST_ARN).unwrap();
        assert_eq!(link, TEST_LINK);
    }

    #[test]
    fn test_arn_with_borrowed_str() {
        // Arn can be constructed from borrowed str.
        let arn = Arn::new(TEST_ARN).unwrap();

        // Fields can be borrowed.
        let _region1 = &arn.region;

        // The same field can be accessed via the getter.
        let _region2: &str = arn.region();

        // Methods can be called on borrowed Arn.
        let link = arn.link().unwrap();
        assert_eq!(link, TEST_LINK);
    }

    #[test]
    fn test_arn_with_owned_string() {
        // Arn can be constructed from owned String.
        let input = String::from(TEST_ARN);
        let arn = Arn::new(&input).unwrap();

        // .to_owned() produces an independent Arn.
        let owned_arn: ArnOwned = arn.to_owned();

        // Fields can be borrowed.
        let _region1 = &owned_arn.region;

        // The same field can be accessed via the getter.
        let _region2: &str = owned_arn.region();

        // Methods can be called on the new Arn.
        let link: String = owned_arn.link().unwrap();
        assert_eq!(link, TEST_LINK);
    }
}
