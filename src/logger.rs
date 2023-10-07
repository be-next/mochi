// use std::time::SystemTime;
use std::error::Error;
// use std::io::ErrorKind;
use std::sync::Arc;
// use anyhow::__private::kind::TraitKind;
// use log::warn;
// use tracing::{info, error};
// use tower_http::trace::{self, TraceLayer};
use tracing_appender::rolling::daily;
use tracing_subscriber::filter::{
    EnvFilter,
    LevelFilter
};

/// Logs a critical error message using the `log::error!` macro and then terminates
/// the process with an exit code of `1`.
///
/// Usage is similar to other logging macros from the `log` crate.
/// The purpose is to provide a concise way to log critical errors
/// and immediately exit the program.
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {{
        tracing::error!($($arg)*);
        std::process::exit(1);
    }}
}

// pub fn setup_logger() -> Result<(), fern::InitError> {
//     fern::Dispatch::new()
//         .format(|out, message, record| {
//             out.finish(format_args!(
//                 "[{} {} {}] {}",
//                 humantime::format_rfc3339_seconds(SystemTime::now()),
//                 record.level(),
//                 record.target(),
//                 message
//             ))
//         })
//         .level(log::LevelFilter::Info)
//         .chain(std::io::stdout())
//         .chain(fern::log_file("output.log")?)
//         .apply()?;
//     Ok(())
// }

// set env var MOCHI_LOG = none,mochi=info
// Work around using Arc instead of Box because of issue 60759
// (cf. https://github.com/rust-lang/rust/issues/60759#:~:text=impl%20Error%20%23-,60759,-Open)
pub fn init_global_subscriber() -> Result<(),  Arc<dyn Error + Send + Sync>> {

    let mochi_log_filter = EnvFilter::try_from_env("MOCHI_LOG")
        .unwrap_or_else(|err| -> EnvFilter {
            println!("Something goes wrong with \"MOCHI_LOG\" env var: {:?}", err);
            println!("Enable \"RUST_LOG\" directives if available, ERROR level otherwise");

            EnvFilter::builder()
                .with_default_directive(LevelFilter::ERROR.into())
                .from_env_lossy()
    });

    let mochi_log_file = daily("./logs", "mochi");

    tracing_subscriber::fmt()
        .with_env_filter(mochi_log_filter)
        .with_writer(mochi_log_file)
        .with_target(false)
        .with_ansi(false)
        .compact()
        .try_init()?;

    Ok(())
}
