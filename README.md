# link2aws

Converts ARNs to direct links to the AWS console.

Amazon Resource Names (ARNs) are strings like `arn:aws:s3:::abc123`
that uniquely identify resources in Amazon Web Services (AWS).

* Copyright (c) 2020-2025, Felix Kaiser.
* License: ISC (<https://spdx.org/licenses/ISC.html>).

## How to use

### As a command line tool

```text
$ cargo install link2aws
$ link2aws arn:aws:s3:::abc123
https://s3.console.aws.amazon.com/s3/buckets/abc123
```

### As a library

```
use link2aws::arn_to_link;
// https://s3.console.aws.amazon.com/s3/buckets/abc123
println!("{}", arn_to_link("arn:aws:s3:::abc123").unwrap());
```

### Advanced usage

The library consists of an ARN parser, and a console link generator.
These two parts may be used separately.

* See `Arn` for an example of how to just parse an ARN.
* See `ArnParts` for an example of how to generate a link,
  or build an ARN, from your own struct.

## Compatibility note

We will try to keep this library up-to-date as AWS changes.
We will continuously add support for new services and resource types.
The returned link will always be our most up-to-date best guess.
This means that there is no guarantee that the returned link will stay
the same if AWS changes a service, or that we will still be able to
return a link for an ARN if AWS discontinues a service.

## Contributing

Found a bug? Missing a resource type? Open a ticket or send a pull request!
Adding a resource type is usually an easy 1-line change (plus 1-line test).

## Other languages

Also see our [JavaScript library](https://github.com/link2aws/link2aws.github.io)
and the web version at [link2aws.github.io](https://link2aws.github.io/)!