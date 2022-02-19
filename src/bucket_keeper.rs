
use crate::bucket::*;
use std::collections::HashMap;

pub struct BucketKeeper<T: Refiller + Taker> {
    buckets: HashMap<String, T>
}

pub struct BucketDefinition {
    name: String,
    max: i32
}

impl BucketDefinition {
    pub fn new(name: &str, max: i32) -> BucketDefinition {
        BucketDefinition{
            name: String::from(name),
            max
        }
    }
}

impl BucketKeeper<Bucket> {
    pub fn new(buckets: Vec<BucketDefinition>) -> BucketKeeper<Bucket> {
        let mut buckets_hash_map = HashMap::new();
        buckets
            .into_iter()
            .for_each(|bucket_definition| {
                buckets_hash_map.insert(bucket_definition.name, Bucket::new(bucket_definition.max));                
            }); 
        BucketKeeper{
            buckets: buckets_hash_map
        }
    }
}

impl<T> BucketKeeper<T> where T: Refiller + Taker {
    pub fn refill(&mut self, bucket_name: &str) {
        let bucket = self.buckets.get(bucket_name).clone();
        match bucket {
            Some(bucket) => {
                let new_bucket = bucket.refill();
                self.buckets.insert(String::from(bucket_name), new_bucket);
            }
            None => {}
        }
    }

    pub fn take(&mut self, bucket_name: &str) -> bool {
            match self.buckets.get(bucket_name) {
            Some(bucket) => {
                match bucket.take() {
                    Some(new_bucket) => {
                        self.buckets.insert(String::from(bucket_name), new_bucket);
                        true
                    },
                    None => {
                        false
                    }
                }
            }
            None => {
                false
            }
        }
    }

    pub fn get_available_tokens(&self, bucket_name: &str) -> i32 {
        match self.buckets.get(bucket_name) {
            Some(bucket) => {
                bucket.current()
            },
            None => {
                0
            }
        }
    }
}

#[test]
fn it_should_take_tokens_from_a_bucket() {
    let mut keeper = BucketKeeper::new(vec![BucketDefinition{
        name: String::from("foo"),
        max: 10
    }, BucketDefinition{
        name: String::from("bar"),
        max: 5
    }]);
    keeper.take("foo");
    keeper.take("bar");
    assert!(keeper.get_available_tokens("foo") == 9);
    assert!(keeper.get_available_tokens("bar") == 4);
}

#[test]
fn it_shoul_refill_tokens() {
    let mut keeper = BucketKeeper::new(vec![BucketDefinition{
        name: String::from("foo"),
        max: 10
    }]);   
    keeper.take("foo");
    keeper.refill("foo");
    assert!(keeper.get_available_tokens("foo") == 10);
}
