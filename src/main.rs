mod board;
mod move_generator;
mod game;
mod utils;
mod web;
mod types;

#[tokio::main]
async fn main() {
    web::start_server().await;
}
