#[macro_export]
macro_rules! println_with_timestamp {
    ($fmt:expr $(, $args:expr)*) => {
        {
            let now = chrono::Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S%.6f").to_string();
            println!("[{}] {}", timestamp, format!($fmt $(, $args)*));
        }
    };
}

