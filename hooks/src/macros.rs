macro_rules! fatal_error {
    ($msg:literal) => {
        fatal_error!($msg, $msg);
    };
    ($msg:literal,$($arg:tt)+) => {
        tracing::error!($($arg)+);
        crate::show_msgbox($msg, "ERROR");
        std::process::exit(1);
    };
}

pub(crate) use fatal_error;
