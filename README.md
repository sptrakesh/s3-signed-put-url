# Generate S3 Signed PUT URL
A simple utility to generate S3 signed PUT URL.  The AWS CLI does not support generating a signed PUT URL.
AWS documentation indicates using an API to generate the signed URL.  This utility is used to generate
static binaries of various platforms, which allows easy deployment without dependencies.

## Usage
The utility depends on the following standard environment variables:
* `AWS_ACCESS_KEY_ID` - The access key for the AWS account
* `AWS_SECRET_ACCESS_KEY` - The secret key for the AWS account

The rest of the information is passed in via command line arguments.
* `region` - The AWS region in which the bucket is located
* `bucket` - The name of the S3 bucket
* `key` - The name of the S3 object
* `expiration` - The expiration time for the signed URL in seconds.

```shell
AWS_ACCESS_KEY_ID=... AWS_SECRET_ACCESS_KEY=... <path to>/s3-signed-put --region=us-east-2 --bucket=my-bucket --key=some/path/file.txt --expiration=3600
```