pub mod rng {

    use rand::{distributions::Alphanumeric, Rng};

    #[allow(unused)]
    pub fn random_alphanumeric_string(len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}

pub mod time {
    use std::time::{SystemTime, UNIX_EPOCH};

    #[allow(unused)]
    pub fn now() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    }
}

#[cfg(test)]
pub mod test_util {
    use serde::{Deserialize, Serialize};
    use std::sync::Once;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TestStruct {
        pub first_name: String,
        pub last_name: String,
    }

    #[allow(unused)]
    static INIT: Once = Once::new();

    #[allow(unused)]
    pub fn init() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("error loading environment variables");
            let _ = env_logger::builder().is_test(true).try_init();
        });
    }
}
