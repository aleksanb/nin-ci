#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate tempdir;
extern crate git2;
extern crate rocket;

use std::io::prelude::*;
use std::path::*;
use rocket::response::NamedFile;
use tempdir::TempDir;

const nin_project_repo: &str = "https://github.com/ninjadev/zeven.git";
const nin_repo: &str = "https://github.com/ninjadev/nin.git";

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
        git2::Repository::clone(nin_project_repo, dir)
        .and_then(|repo| {
            println!("{:?}, {:?}", 10, repo.state());
        })

    }
    //Repository::clone(nin_project_repo, )
    //rocket::ignite()
        //.mount("/", routes![index, get_prod])
        //.launch();
}
