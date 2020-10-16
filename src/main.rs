#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[put("/")]
fn put_store() -> String {
    format!("Done")
}

#[post("/", data="<input>")]
fn post_store(input:String) -> String {
    println!("post: {}", input);
    format!("Done")
}

fn main() {
    rocket::ignite().mount("/", routes![put_store,post_store]).launch();
}
