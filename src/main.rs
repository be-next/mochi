mod extractor;
mod file;
mod logger;
mod model;
mod routes;
mod utils;

use crate::extractor::build_all;
use crate::file::ConfigurationFolder;
use crate::logger::{
    setup_logger,
    init_global_subscriber
};
use crate::model::core::SystemCore;
use crate::model::yaml::SystemFolder;
use crate::routes::handle_request;
use crate::utils::shutdown_signal;

use tracing::{info};
use anyhow::{Context, Result};
use axum::body::Body;
use axum::http::Request;
use axum::routing::{get, on, MethodFilter};
use axum::Router;
use axum_prometheus::PrometheusMetricLayer;
// use tower_http::trace::TraceLayer;
// use log::info;
use std::env;
use std::net::SocketAddr;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    //setup_logger().context("Setup logger")?;
    init_global_subscriber()?;

    info!("Starting Mochi...");

    let config_path = env::var("CONFIG_PATH").unwrap_or("./config".to_string());
    info!(
        "Configuration path: {} (absolute path: {})",
        config_path,
        fs::canonicalize(&config_path)?.display()
    );

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let system_folders: Vec<SystemFolder> = ConfigurationFolder::new(config_path).load_systems();

    let rules_maps: Vec<_> = system_folders
        .into_iter()
        .map(|system| {
            let s = SystemCore {
                name: system.name.to_owned(),
                api_sets: build_all(system.shapes.to_owned(), system.apis, system.data).unwrap(),
            };
            s.generate_rules_map()
        })
        .collect();

    let app: Router = rules_maps
        .into_iter()
        .fold(Router::new(), move |r, map| {
            let subrouter = map
                .into_iter()
                .fold(Router::new(), |acc, (endpoint, rules)| {
                    info!("Registration: {:?}", endpoint.clone());
                    acc.route(
                        &endpoint.route,
                        on(MethodFilter::try_from(endpoint.clone().method).unwrap(), {
                            move |request: Request<Body>| handle_request(request, rules.to_owned())
                        }),
                    )
                });
            r.merge(subrouter)
        })
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Mochi started. Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Starting http server")?;

    Ok(())
}
