use crate::Error;
use crate::parts::ArnParts;

/// Represents an ARN, separated into its component parts:
/// partition, service, region, account, resource type,
/// resource id, and resource revision.
///
/// # Example
///
/// [`Arn::new`] parses an ARN:
///
/// ```
/// use link2aws::{Arn, ArnParts, ArnOwned};
///
/// let arn_str = "arn:aws:rds:us-east-1:12345:snapshot:rds:db";
/// let arn: Arn = Arn::new(arn_str).unwrap();
///
/// assert_eq!(arn, Arn {
///     partition: "aws",
///     service: "rds",
///     region: "us-east-1",
///     account: "12345",
///     resource_type: "snapshot",
///     resource_id: "rds:db",
///     resource_revision: "",
///     has_path: false,
/// });
/// ```
///
/// [`Arn`] implements [`ArnParts`], which provides methods to get a link, or rebuild the ARN:
///
/// ```
/// # use link2aws::{Arn, ArnParts, ArnOwned};
/// # let arn_str = "arn:aws:rds:us-east-1:12345:snapshot:rds:db";
/// # let arn: Arn = Arn::new(arn_str).unwrap();
/// let expected_link = "https://console.aws.amazon.com/rds/home?region=us-east-1#db-snapshot:id=rds:db";
/// assert_eq!(arn.link().unwrap(), expected_link);
/// assert_eq!(arn.build(), arn_str);
/// ```
///
/// [`Arn`] borrows `&str` slices from the input `&str`.
/// To make an independent version that uses `String`s, call [`to_owned()`](Arn::to_owned).
///
/// [`ArnOwned`] implements [`ArnParts`] too.
///
/// ```
/// # use link2aws::{Arn, ArnParts, ArnOwned};
/// # let arn_str = "arn:aws:rds:us-east-1:12345:snapshot:rds:db";
/// # let arn: Arn = Arn::new(arn_str).unwrap();
/// # let expected_link = "https://console.aws.amazon.com/rds/home?region=us-east-1#db-snapshot:id=rds:db";
/// let arn_owned: ArnOwned = arn.to_owned();
/// assert_eq!(arn_owned.link().unwrap(), expected_link);
/// assert_eq!(arn_owned.build(), arn_str);
/// ```

#[derive(Debug, PartialEq, Clone)]
pub struct Arn<'a> {
    /// `"aws"`, `"aws-cn"`, `"aws-us-gov"`, etc.
    pub partition: &'a str,

    /// `"s3"`, `"ec2"`, `"iam"`, etc.
    pub service: &'a str,

    /// `"us-east-1"`, `"us-west-2"`, etc., or `""`.
    pub region: &'a str,

    /// Usually like `"12345"`, or `""`.
    pub account: &'a str,

    /// `"snapshot"`, `"image"`, `"instance"`, etc.
    pub resource_type: &'a str,

    /// Typically chosen by the user. May contain colons and/or slashes.
    pub resource_id: &'a str,

    /// Resource revision (often empty).
    pub resource_revision: &'a str,

    /// True if there is a `/` before the resource id instead of a `:`.
    pub has_path: bool,
}

/// Like [`Arn`], but with owned `String`s instead of borrowed `&str`s.
#[derive(Debug, PartialEq, Clone)]
pub struct ArnOwned {
    pub partition: String,
    pub service: String,
    pub region: String,
    pub account: String,
    pub resource_type: String,
    pub resource_id: String,
    pub resource_revision: String,
    pub has_path: bool,
}

impl<'a> ArnParts<'a> for Arn<'a> {
    fn partition(&self) -> &str {
        self.partition
    }
    fn service(&self) -> &str {
        self.service
    }
    fn region(&self) -> &str {
        self.region
    }
    fn account(&self) -> &str {
        self.account
    }
    fn resource_revision(&self) -> &str {
        self.resource_revision
    }
    fn resource_type(&self) -> &str {
        self.resource_type
    }
    fn resource_id(&self) -> &str {
        self.resource_id
    }
    fn has_path(&self) -> bool {
        self.has_path
    }
}

impl ArnParts<'static> for ArnOwned {
    fn partition(&self) -> &str {
        self.partition.as_str()
    }
    fn service(&self) -> &str {
        self.service.as_str()
    }
    fn region(&self) -> &str {
        self.region.as_str()
    }
    fn account(&self) -> &str {
        self.account.as_str()
    }
    fn resource_revision(&self) -> &str {
        self.resource_revision.as_str()
    }
    fn resource_type(&self) -> &str {
        self.resource_type.as_str()
    }
    fn resource_id(&self) -> &str {
        self.resource_id.as_str()
    }
    fn has_path(&self) -> bool {
        self.has_path
    }
}

impl<'a> PartialEq<ArnOwned> for Arn<'a> {
    fn eq(&self, other: &ArnOwned) -> bool {
        self.partition() == other.partition()
            && self.service() == other.service()
            && self.region() == other.region()
            && self.account() == other.account()
            && self.resource_type() == other.resource_type()
            && self.resource_id() == other.resource_id()
            && self.resource_revision() == other.resource_revision()
            && self.has_path() == other.has_path()
    }
}

impl<'a> PartialEq<Arn<'a>> for ArnOwned {
    fn eq(&self, other: &Arn<'a>) -> bool {
        other == self
    }
}

impl<'a> Arn<'a> {
    /// Convert all `&str` fields into owned `String`s.
    pub fn to_owned(&self) -> ArnOwned {
        ArnOwned {
            partition: self.partition.to_owned(),
            service: self.service.to_owned(),
            region: self.region.to_owned(),
            account: self.account.to_owned(),
            resource_type: self.resource_type.to_owned(),
            resource_id: self.resource_id.to_owned(),
            resource_revision: self.resource_revision.to_owned(),
            has_path: self.has_path,
        }
    }

    /// Parse the ARN into its components.
    pub fn new(arn_str: &'a str) -> Result<Self, Error> {
        // Remove leading/trailing whitespace for user convenience.
        // It would have been better to do this outside the library,
        // but I'm including it for consistency with link2aws.js.
        let arn_str = arn_str.trim();

        // Length limit.
        // There is no documented limit for ARNs in general.
        // For IAM User, the documented limit is 2048.
        // Please file an issue if you can find a resource type
        // with a higher documented limit.
        if arn_str.len() > 2048 {
            return Err(Error::TooLong);
        }

        // Check for invalid characters.
        // This is meant to catch malicious inputs. This will not
        // catch all invalid ARNs, as some resource types have
        // stricter rules. Please file an issue if you are aware
        // of a valid ARN that is rejected by this check.
        if !arn_str
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || ":/+=,.@_*#-".contains(c))
        {
            return Err(Error::BadCharacters);
        }

        // Parse components of ARN.
        let arn: Arn<'a> = parser::parse(arn_str).map_err(|_| Error::ParseError)?;

        // region must have valid format.
        // This is security relevant as it is used as a subdomain
        // before the console domain.
        if !arn
            .region()
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(Error::BadCharacters);
        }

        Ok(arn)
    }
}

/// Internal parser module using nom.
mod parser {
    use super::Arn;

    use nom::{
        IResult, Parser,
        branch::alt,
        bytes::complete::{tag, take_till},
        character::complete::char,
        combinator::{all_consuming, opt, rest, success},
        sequence::{preceded, terminated},
    };

    fn terminated_by_colon(input: &str) -> IResult<&str, &str> {
        terminated(take_till(|c| c == ':'), char(':')).parse(input)
    }

    fn terminated_by_colon_not_slash(input: &str) -> IResult<&str, &str> {
        terminated(take_till(|c| c == ':' || c == '/'), char(':')).parse(input)
    }

    fn terminated_by_slash_not_colon(input: &str) -> IResult<&str, &str> {
        terminated(take_till(|c| c == ':' || c == '/'), char('/')).parse(input)
    }

    pub fn parse<'a>(
        input: &'a str,
    ) -> Result<super::Arn<'a>, nom::Err<nom::error::Error<&'a str>>> {
        let (_, result) = all_consuming((
            // arn:partition:service:region:account-id:...
            terminated(tag("arn"), char(':')),
            terminated_by_colon, // partition
            terminated_by_colon, // service
            terminated_by_colon, // region
            terminated_by_colon, // account
            alt((
                // ...resource-type/resource-id:resource-revision
                (
                    terminated_by_slash_not_colon, // resource-type
                    terminated_by_colon_not_slash, // resource-id
                    rest,                          // resource-revision
                    success(true),                 // has path?
                ),
                // ...resource-type:resource-id (resource-id can contain colons!)
                (
                    terminated_by_colon_not_slash, // resource-type
                    rest,                          // resource-id
                    success(""),                   // resource-revision
                    success(false),                // has path?
                ),
                // ...resource-type/resource-id (common case)
                // .../resource-type/resource-id (apigateway)
                (
                    preceded(opt(char('/')), terminated_by_slash_not_colon), // resource-type
                    rest,                                                    // resource-id
                    success(""),                                             // resource-revision
                    success(true),                                           // has path?
                ),
                //...resource-id
                (
                    success(""),    // resource-type
                    rest,           // resource-id
                    success(""),    // resource-revision
                    success(false), // has path?
                ),
            )),
        ))
        .parse(input)?;

        let (
            _,
            partition,
            service,
            region,
            account,
            (resource_type, resource_id, resource_revision, has_path),
        ) = result;

        Ok(Arn {
            partition,
            service,
            region,
            account,
            resource_revision,
            resource_type,
            resource_id,
            has_path,
        })
    }
}
