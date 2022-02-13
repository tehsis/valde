
    use crate::bucket::{Bucket, Response};

    pub struct BucketKeeper {
        current_bucket: Bucket
    }

    impl BucketKeeper {
        pub fn new(max: i32) -> BucketKeeper {
            let bucket = Bucket::new(max);
            BucketKeeper{
                current_bucket: bucket
            }
        }

        pub fn refill(&mut self) {
            self.current_bucket = self.current_bucket.refill();
        }

        pub fn take(&mut self) -> bool {
            match self.current_bucket.take() {
                Response::Yes(new_bucket) => {
                    self.current_bucket = new_bucket;
                    true
                },
                Response::No => {
                    false
                }
            }
        }

        pub fn get_available_tokens(&self) -> i32 {
            return self.current_bucket.current_token_amount;
        }
    }

    #[test]
    fn it_should_take_tokens() {
        let mut keeper = BucketKeeper::new(10);
        keeper.take();
        assert!(keeper.current_bucket.current_token_amount == 9);
    }

    #[test]
    fn it_shoul_refill_tokens() {
        let mut keeper = BucketKeeper::new(10);
        keeper.take();
        keeper.refill();
        assert!(keeper.current_bucket.current_token_amount == 10);
    }
