use clap::Parser;

use link2aws::arn_to_link;

/// Converts ARNs to AWS Console links.
///
/// You may pass ARNs as command line arguments:
///
/// $ link2aws arn:aws:s3:::abc123
/// https://s3.console.aws.amazon.com/s3/buckets/abc123
#[derive(Parser, Debug)]
#[command(author, version, verbatim_doc_comment)]
struct Cli {
    /// One or more ARNs.
    #[arg()]
    arns: Vec<String>,

    /// Take ARNs from stdin (one per line), not from args.
    #[arg(long)]
    stdin: bool,

    /// Suppress error messages for failed ARNs.
    #[arg(short, long)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();

    let all_ok: bool = if cli.stdin {
        handle_all(std::io::stdin().lines().map_while(Result::ok), cli.quiet)
    } else {
        handle_all(cli.arns.iter(), cli.quiet)
    };

    std::process::exit(if all_ok { 0 } else { 1 });
}

fn handle_all<I>(lines: I, quiet: bool) -> bool
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut all_ok = true;

    for line in lines {
        match arn_to_link(line.as_ref()) {
            Ok(link) => println!("{}", link),
            Err(err) => {
                all_ok = false;
                if !quiet {
                    eprintln!("link2aws: {:?}: {}", line.as_ref(), err);
                }
            }
        }
    }

    all_ok
}
