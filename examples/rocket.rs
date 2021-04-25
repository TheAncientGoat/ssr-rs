//This example needs rust toolchain nightly version
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use ssr_rs::Ssr;
use rocket::response::content;
use rocket_contrib::serve::StaticFiles;


#[get("/")]
fn index() -> content::Html<String> {

    content::Html(
        Ssr::render_to_string("./client/dist/ssr/index.js", "SSR", "Index", None)
    )

}

fn main() {
    rocket::ignite()
    .mount("/styles", StaticFiles::from("./client/dist/ssr/styles"))
    .mount("/scripts", StaticFiles::from("./client/dist/client/"))
    .mount("/images", StaticFiles::from("./client/dist/ssr/images/"))
    .mount("/", routes![index]).launch();
}