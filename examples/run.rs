//This example exist just for develop purposes
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("./client/dist/ssr/index.js").unwrap();

    println!("{}", ssr_rs::render_to_string(&source, "SSR", None))
}
