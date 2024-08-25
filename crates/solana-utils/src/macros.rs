/// Prints a message to the log.
#[macro_export]
macro_rules! log {
    ($($args:tt)*) => {
        match ::core::format_args!($($args)*) {
            args => match args.as_str() {
                Some(msg) => ::solana_program::log::sol_log(msg),
                None => ::solana_program::log::sol_log(&args.to_string()),
            }
        }
    };
}
