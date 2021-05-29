#![deny(warnings)]
use std::fs::read_to_string;
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    let source = read_to_string("./client/dist/ssr/index.js").unwrap();

    let html = warp::path::end()
        .map(move || Response::builder().body(ssr_rs::render_to_string(&source, "SSR", None)));

    let css = warp::path("styles").and(warp::fs::dir("./client/dist/ssr/styles/"));
    let scripts = warp::path("scripts").and(warp::fs::dir("./client/dist/client/"));
    let img = warp::path("images").and(warp::fs::dir("./client/dist/ssr/images/"));

    let routes = warp::get().and(html.or(css).or(scripts).or(img));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
