#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::Data;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[put("/")]
fn put_store() -> String {
    format!("Done")
}

#[post("/", data="<input>")]
fn post_store(input:Data) -> Result<String,rocket::response::Debug<std::io::Error>> {
    input.stream_to_file("test")?;
    Ok(format!("Done. Wrote {{}} bytes to file test"))
    // 	Err(x) => Err(format!("Error: {}", x))
    // }
}

fn main() {
    rocket::ignite().mount("/", routes![put_store,post_store]).launch();
}
