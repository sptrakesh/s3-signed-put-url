use std::io::BufRead;
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
  /// The AWS credentials profile to use
  #[clap(short = 'p', long = "profile")]
  profile: Option<String>,
  /// The expiration time in seconds for the signed PUT url
  #[arg(short, long, default_value_t = 86400)]
  expiration: i64
}

struct Section
{
  name: String,
  key: String,
  secret: String,
  token: Option<String>
}

fn section(path: &str, profile: &str) -> Result<Section, String>
{
  let file = std::fs::File::open(path);
  if file.is_err() { return Err("Failed to open credentials file".to_string()); }

  let mut section = Section{ name: String::new(), key: String::new(), secret: String::new(), token: None };
  let mut found = false;
  let reader = std::io::BufReader::new(file.unwrap());
  let lines = reader.lines();
  for line in lines
  {
    let line = line.unwrap();
    if line.starts_with("[") && line.ends_with("]")
    {
      let name = &line[1..line.len()-1];
      if name == profile { found = true; }
      else if found { break; }
      section.name = name.to_string();
    }
    else if line.starts_with("aws_access_key_id")
    {
      let parts = line.split("=").collect::<Vec<_>>();
      section.key = parts[1].trim().to_string();
    }
    else if line.starts_with("aws_secret_access_key")
    {
      let parts = line.split("=").collect::<Vec<_>>();
      section.secret = parts[1].trim().to_string();
    }
    else if line.starts_with("aws_session_token")
    {
      let parts = line.split("=").collect::<Vec<_>>();
      section.token = Some(parts[1].trim().to_string());
    }
  }

  if section.name == profile { return Ok(section); }

  Err("Profile not found".to_string())
}

fn from_credentials(profile: &str) -> Result<Credentials, String>
{
  let path = format!("{}/.aws/credentials", std::env::home_dir().unwrap().display());
  if !std::path::Path::new(path.as_str()).exists()
  {
    println!("AWS credentials file not found at {}", path);
    return Err("AWS credentials file not found".to_string());
  }

  let sec = section(&path, profile)?;
  Ok(Credentials::new(sec.key.as_str(), sec.secret.as_str(), if sec.token.is_some() { Some(sec.token.as_ref().unwrap().as_str()) } else { None }))
}

fn from_enviroment() -> Result<Credentials, String>
{
  let key = std::env::var("AWS_ACCESS_KEY_ID");
  if key.is_err()
  {
    println!("AWS_ACCESS_KEY_ID environment variable is not set");
    return Err("AWS_ACCESS_KEY_ID environment variable is not set".to_string());
  }
  let key = key.unwrap();

  let secret = std::env::var("AWS_SECRET_ACCESS_KEY");
  if secret.is_err()
  {
    println!("AWS_SECRET_ACCESS_KEY environment variable is not set");
    return Err("AWS_SECRET_ACCESS_KEY environment variable is not set".to_string());
  }
  let secret = secret.unwrap();

  Ok(Credentials::new(key.as_str(), secret.as_str(), None))
}

fn main() -> Result<(), String>
{
  let args = Cli::parse();

  let credentials = if args.profile.is_some() { from_credentials(args.profile.unwrap().as_str()) } else { from_enviroment() };
  let credentials = credentials?;
  let bucket = Bucket::new(args.region.as_str(), args.bucket.as_str());
  let url = put(&credentials, &bucket, args.key.as_str(), args.expiration).unwrap();
  println!("{}", url);
  Ok(())
}