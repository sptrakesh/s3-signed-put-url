use clap::Parser;
use s3_presign::{Bucket, Credentials, put};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None, ignore_errors(true))]
struct Cli
{
  /// The AWS region where the bucket is located
  #[clap(short = 'r', long = "region")]
  region: String,
  /// The AWS bucket for which the signed PUT url is to be generated
  #[clap(short = 'b', long = "bucket")]
  bucket: String,
  /// The virtual path within the bucket for the destination object
  #[clap(short = 'k', long = "key")]
  key: String,
  /// The expiration time in seconds for the signed PUT url
  #[arg(short, long, default_value_t = 86400)]
  expiration: i64
}

fn main()
{
  let args = Cli::parse();

  let key = std::env::var("AWS_ACCESS_KEY_ID");
  if key.is_err()
  {
    println!("AWS_ACCESS_KEY_ID environment variable is not set");
    std::process::exit(1);
  }
  let key = key.unwrap();

  let secret = std::env::var("AWS_SECRET_ACCESS_KEY");
  if secret.is_err()
  {
    println!("AWS_SECRET_ACCESS_KEY environment variable is not set");
    std::process::exit(1);
  }
  let secret = secret.unwrap();

  let credentials = Credentials::new(key.as_str(), secret.as_str(), None);
  let bucket = Bucket::new(args.region.as_str(), args.bucket.as_str());
  let url = put(&credentials, &bucket, args.key.as_str(), args.expiration).unwrap();
  println!("{}", url);
}