#![feature(plugin)]
#![plugin(rocket_codegen)]

use std::io::prelude::*;
use std::path::*;
use git2::

extern crate tempdir;
extern crate git2;
extern crate rocket;
use rocket::response::NamedFile;

const nin_project_repo = "https://github.com/ninjadev/zeven.git";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/prods/<path..>")]
fn get_prod(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("prods/").join(path)).ok()
}

fn main() {
    println!("Hello, world!");
    if let Ok(dir) = TempDir::new("build") {
        println!("{:?}", dir);
    }
    //Repository::clone(nin_project_repo, )
    rocket::ignite()
        .mount("/", routes![index, get_prod])
        .launch();
}
