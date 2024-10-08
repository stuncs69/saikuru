mod board;
mod move_generator;
mod game;
mod utils;
mod web;
mod types;

#[tokio::main]
async fn main() {
    println!("starting saikuru v{} server", env!("CARGO_PKG_VERSION"));
    web::start_server().await;
}
