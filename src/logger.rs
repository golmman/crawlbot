extern crate chrono;

#[macro_export]
macro_rules! log_crawl {
    ($level:expr, $($arg:tt)*) => (
        let timestamp = ::chrono::Utc::now()
            .to_rfc3339_opts(::chrono::SecondsFormat::Millis, true);

        let file = file!();
        let split_at = 0.max(file.len() as i32 - 20) as usize;
        let filename = file.split_at(split_at);
        print!("[{}] [{:05}] [{:>20}] ", timestamp, $level, filename.1);
        println!($($arg)*);
    )
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => (
        log_crawl!("ERROR", $($arg)*);
        log_crawl!("ERROR", "`---> occurred at {}:{}:{}", file!(), line!(), column!());
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

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => (
        if cfg!(debug_assertions) {
            log_crawl!("DEBUG", $($arg)*);
        }
    )
}
