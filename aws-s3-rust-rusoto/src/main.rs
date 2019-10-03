extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{S3, S3Client};
use std::time::{Duration, Instant};

fn main() {
    let mut chain = ChainProvider::new();
    chain.set_timeout(Duration::from_millis(200));
    let s3client = S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        chain,
        Region::UsEast1,
    );

    let start = Instant::now();
    println!("Starting up at {:?}", start);

    match s3client.list_buckets().sync() {
        Err(e) => println!("Error listing buckets: {}", e),
        Ok(buckets) =>{
            let i = 1;
            for x in buckets.buckets {
                println!("{:?} - {}", x,i);
                i = i+1;
            }
        },
    };
    println!("Took {:?}", Instant::now().duration_since(start));
}
