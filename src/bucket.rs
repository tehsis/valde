    pub struct Bucket {
       pub max_token_amount: i32,
       pub current_token_amount: i32
    }

    pub enum Response {
        Yes(Bucket),
        No
    }

    impl Bucket {
        pub fn new(max: i32) -> Bucket {
            Bucket { max_token_amount: max, current_token_amount: max }
        }


        pub fn refill(&self) -> Bucket {
            Bucket { 
                max_token_amount: self.max_token_amount, 
                current_token_amount: if self.current_token_amount < self.max_token_amount { self.current_token_amount + 1 } else { self.current_token_amount }  
            }
        }

        pub fn take(&self) -> Response {
            if self.current_token_amount > 0 {
                Response::Yes(Bucket {
                    max_token_amount: self.max_token_amount,
                    current_token_amount: self.current_token_amount - 1
                })
            } else {
                Response::No
            }
        }
    }

    #[test]
    fn it_can_be_constructed() {
        let b = Bucket::new(10);
        assert!(b.current_token_amount == b.max_token_amount);
    }

    #[test]
    fn it_should_return_true_only_if_there_are_tokens_available() {
        let mut b = Bucket::new(10);
        let mut number_of_denied_requests = 0;
        for _ in 1..=20  {
           match b.take() {
            Response::No => {
                number_of_denied_requests = number_of_denied_requests + 1;
            },
            Response::Yes(bucket) => {
                b = bucket
            }
           }
        }

        assert!(number_of_denied_requests == 10, "number of denied requests is {} but should be 10", number_of_denied_requests);
    }
