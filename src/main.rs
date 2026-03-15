use aws_sdk_s3::{Client, config::Region, presigning::PresigningConfig};
use clap::Parser;

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
  /// The AWS credentials profile to use
  #[clap(short = 'p', long = "profile")]
  profile: Option<String>,
  /// The expiration time in seconds for the signed PUT url
  #[arg(short, long, default_value_t = 86400)]
  expiration: u64
}

#[tokio::main(flavor = "current_thread")]
async fn main()
{
  let args = Cli::parse();

  let region = Region::new(args.region.clone());
  let config = aws_config::from_env().region(region).load().await;
  let client = Client::new(&config);

  let expiry: std::time::Duration = std::time::Duration::from_secs(args.expiration);
  let expiry = PresigningConfig::expires_in(expiry).unwrap();
  let req = client.put_object().bucket(args.bucket.as_str()).key(args.key.as_str()).presigned(expiry).await.unwrap();

  println!("{}", req.uri());
}