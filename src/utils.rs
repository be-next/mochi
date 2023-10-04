use tokio::signal;
use log::info;

/// Listens for shutdown signals (like Ctrl+C or a Unix termination signal).
/// Upon receiving a signal, this function merely logs that the signal was received
/// and that the program will terminate. Any additional cleanup or shutdown logic
/// should be added following signal reception.
///
/// This asynchronous function listens for either a Ctrl+C event or a Unix terminate signal
/// (e.g., from the `kill` command) and triggers the process of gracefully shutting down
/// the application.
///
/// On non-Unix platforms, only the Ctrl+C event is handled.
///
/// # Usage
///
/// It's typically used in conjunction with a server's main event loop to ensure
/// that resources are cleaned up properly when the application is interrupted.
///
/// # Example
///
/// ```rust
/// tokio::spawn(async move {
///     shutdown_signal().await;
///     // Perform cleanup and shutdown logic here.
/// });
/// ```
pub async fn shutdown_signal() {
    let sig_ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let sig_terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    // TODO : test on non-unix systems...
    #[cfg(not(unix))]
        let sig_terminate = std::future::pending::<()>();

    tokio::select! {
        _ = sig_ctrl_c => {
            info!("Ctrl-c received. Starting shutdown...")
        },
        _ = sig_terminate => {
            info!("Terminate signal received. Starting shutdown...")
        },
    }
}