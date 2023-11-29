macro_rules! fatal_error {
    ($($arg:expr),+) => {
         fatal_error!($($arg),+; $($arg),+);
    };
    ($($user_arg:expr),+; $($log_arg:tt)+) => {
        tracing::error!($($log_arg)+);
        crate::show_msgbox(&format!($($user_arg)+), "ERROR");
        std::process::exit(1);
    };
}

pub(crate) use fatal_error;
