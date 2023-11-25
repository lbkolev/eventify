use clap::Parser;
use futures::TryFutureExt;
use secrecy::ExposeSecret;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};
use types::storage::Postgres;
use url::Url;

use chainthru::settings::Settings;
use chainthru_index as indexer;
use chainthru_primitives as types;
use chainthru_server as server;
use indexer::app::App;

use ethers_providers::{Http, Ipc, Ws};

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::new(env_filter);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(name, sink))
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

#[tokio::main]
async fn main() -> chainthru::Result<()> {
    let settings = Settings::parse();
    let subscriber = get_subscriber(
        "chainthru".into(),
        settings.log_level.as_str().into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    log::info!("{:#?}", settings);

    let server_settings = server::Settings::from(settings.clone());
    let mut handles = vec![];

    if settings.server.server_enabled {
        handles.push(tokio::spawn(
            server::run(server_settings).map_err(chainthru::Error::from),
        ));
    }

    if settings.indexer.indexer_enabled {
        match Url::parse(&settings.node_url)?.scheme() {
            "http" | "https" => {
                handles.push(tokio::spawn(
                    indexer::run::<Http, Postgres>(
                        App::default()
                            .with_src_block(settings.indexer.src_block)
                            .with_dst_block(settings.indexer.dst_block)
                            .with_storage(settings.storage_url.expose_secret())
                            .with_http(&settings.node_url),
                    )
                    .map_err(chainthru::Error::from),
                ));
            }
            "ws" | "wss" => {
                handles.push(tokio::spawn(
                    indexer::run::<Ws, Postgres>(
                        App::default()
                            .with_src_block(settings.indexer.src_block)
                            .with_dst_block(settings.indexer.dst_block)
                            .with_storage(settings.storage_url.expose_secret())
                            .with_websocket(&settings.node_url)
                            .await,
                    )
                    .map_err(chainthru::Error::from),
                ));
            }
            "ipc" => {
                handles.push(tokio::spawn(
                    indexer::run::<Ipc, Postgres>(
                        App::default()
                            .with_src_block(settings.indexer.src_block)
                            .with_dst_block(settings.indexer.dst_block)
                            .with_storage(settings.storage_url.expose_secret())
                            .with_ipc(&settings.node_url)
                            .await,
                    )
                    .map_err(chainthru::Error::from),
                ));
            }
            _ => {
                return Err(chainthru::Error::NodeURLScheme(settings.node_url));
            }
        };
    }

    futures::future::join_all(handles).await;

    Ok(())
}
