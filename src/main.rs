#![feature(async_await)]
use tide;

fn main() {
    let mut app = tide::App::new(());
    app.at("/").get(async || "Hello, world!");
    app.serve("127.0.0.1:7878")
}
