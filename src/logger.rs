use std::time::SystemTime;

/// Logs a critical error message using the `log::error!` macro and then terminates
/// the process with an exit code of `1`.
///
/// Usage is similar to other logging macros from the `log` crate.
/// The purpose is to provide a concise way to log critical errors
/// and immediately exit the program.
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {{
        log::error!($($arg)*);
        std::process::exit(1);
    }}
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
