use std::fs;

static MY_STR: &'static str = "Hello, world!";

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    MY_STR
}

#[get("/name/<name>")]
fn printname(name: String) -> String {
    format!("Hello, {}!", name)
}

#[get("/file/<file>")]
fn readfile(file: String) -> String {
    let contents = fs::read_to_string(file).expect("Should have been able to read the file");
    format!("File contents: {}", contents)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, printname, readfile])
}
