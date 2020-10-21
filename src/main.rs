#![feature(proc_macro_hygiene, decl_macro)]

// webservice stuff
#[macro_use] extern crate rocket;
use rocket::Data;
use std::path::PathBuf;
// DateTime stuff
use chrono::prelude::*;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/<path..>", data="<input>")]
fn post_store(path:PathBuf, input:Data) -> Result<String,rocket::response::Debug<std::io::Error>> {
    let mut name = String::from("");
    for x in path.iter() {
	match x.to_str() {
	    Some(y) => {
		name.push_str(y);
		name.push('_');
	    },
	    None => {
		println!("Failed to use string: '{}'", path.to_string_lossy());
		break;
	    },
	}
    }
    if name.len() > 200 {
	match name.get(..200) {
	    Some(x) => name = x.to_string(),
	    None => {
		println!("Failed to truncate path: '{}'", path.to_string_lossy());
		name = String::from("");
	    }
	}
    }    
    name.push_str(&Local::now().to_rfc3339());
    name.push_str(".log");
    println!("Name: {}", name);
    input.stream_to_file(name)?;
    Ok(format!("Done. Wrote {{}} bytes to file test"))
    // 	Err(x) => Err(format!("Error: {}", x))
    // }
}

fn main() {
    rocket::ignite().mount("/", routes![post_store]).launch();
}
