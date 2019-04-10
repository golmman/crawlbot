extern crate chrono;

#[macro_export]
macro_rules! log_crawl {
    ($level:expr, $($arg:tt)*) => (
        let timestamp = ::chrono::Utc::now()
            .to_rfc3339_opts(::chrono::SecondsFormat::Millis, true);
        print!("{} {:05} ", timestamp, $level);
        println!($($arg)*);
    )
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => (
        log_crawl!("ERROR", $($arg)*);
        println!("    at file: {}", file!());
        println!("       line: {}", line!());
        println!("       col : {}", column!());
    )
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => (
        log_crawl!("WARN", $($arg)*);
    )
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => (
        if cfg!(debug_assertions) {
            log_crawl!("INFO", $($arg)*); 
        }
    )
}
