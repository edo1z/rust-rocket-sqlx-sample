#[macro_export]
macro_rules! log_into {
    ($error:expr, $target_type:ty) => {{
        let converted_error: $target_type = $error.into();
        tracing::error!("{} ({}:{})", converted_error, file!(), line!());
        converted_error
    }};
}
