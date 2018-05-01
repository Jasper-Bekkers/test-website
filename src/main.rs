#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "<h1>Test site</h1>"
}

fn main(){
    rocket::ignite().mount("/", routes![index]).launch();
}