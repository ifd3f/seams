use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::sync::watch;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::info;

use crate::site_data::SiteData;

#[derive(Clone)]
struct SiteDataRx(watch::Receiver<SiteData>);

pub async fn run_server(port: u16, site_data: watch::Receiver<SiteData>) -> anyhow::Result<()> {
    // build our application with some routes
    let app = Router::new()
        .route("/.reload-listener", get(reload_socket_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(SiteDataRx(site_data))
        .into_make_service_with_connect_info::<SocketAddr>();

    // run it with hyper
    let listener = tokio::net::TcpListener::bind(("localhost", port))
        .await
        .unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

#[axum_macros::debug_handler]
async fn reload_socket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(SiteDataRx(sdrx)): State<SiteDataRx>,
) -> impl IntoResponse {
    info!("{addr} connected to websocket");
    let mut sdrx = sdrx.clone();
    ws.on_upgrade(move |mut socket| async move {
        sdrx.changed().await.unwrap();
        socket.send(Message::Text("changed".into())).await.unwrap();
    })
}
