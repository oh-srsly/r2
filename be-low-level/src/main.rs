use be_low_level::{build_router, create_initial_state};
use log::info;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    let state = create_initial_state().await;
    let router = build_router(state);

    info!("listening on 0.0.0.0:8698");
    let acceptor = TcpListener::new("0.0.0.0:8698").bind().await;
    Server::new(acceptor).serve(router).await;
}
