macro_rules! log_error {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                log::error!("Error: {:?}", e);
                println!("Error: {:?}", e);
                panic!()
            }
        }
    };
}