extern crate aws_sdk_rust;


use aws_sdk_rust::aws::common::credentials::DefaultCredentialsProvider;
use aws_sdk_rust::aws::common::region::Region;
use aws_sdk_rust::aws::s3::endpoint::Endpoint;
use aws_sdk_rust::aws::s3::s3client::S3Client;
use aws_sdk_rust::aws::s3::endpoint::Signature;


fn main() {
    let provider = DefaultCredentialsProvider::new(None).unwrap();

    // V4 is the default signature for AWS. However, other systems also use V2.
    let endpoint = Endpoint::new(Region::UsEast1, Signature::V4, None, None, None, None);
    let client = S3Client::new(provider, endpoint);

    match client.list_buckets() {
        Ok(output) => {
            for x in output.buckets {
                println!("{}", x.name);
            }
        }
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }
}