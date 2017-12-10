#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate tempdir;
extern crate git2;
extern crate rocket;

use std::io::prelude::*;
use std::path::*;
use rocket::response::NamedFile;
use tempdir::TempDir;
use std::process::Command;

const nin_project_repo: &str = "git@github.com:sigvef/revision-invite-2018.git";
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
    let dir = TempDir::new("build").unwrap();
    //git2::Repository::clone(nin_project_repo, "demo").unwrap();
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("cd \"{:?}\" && git clone \"{}\" demo && cd demo && nin compile", dir.path(), nin_project_repo))
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    //if let Ok(dir) = TempDir::new("build") {
        //println!("{:?}", dir);
        //let path = dir.path().to_owned();
                //let run_nin_command = format!("cd {:?}", path);
                //println!("{}", run_nin_command);
        //git2::Repository::clone(nin_project_repo, "demo").unwrap();
        //let output = Command::new("bash")
        //    .arg("-c")
        //    .arg("cd demo && nin compile")
        //    .output()
        //    .unwrap();
        //println!("{}", String::from_utf8_lossy(&output.stdout));
        //println!("{}", String::from_utf8_lossy(&output.stderr));
        /*git2::Repository::clone(nin_project_repo, "demo")
            .and_then(|repo| {
                println!("{:?}, {:?}", 10, repo.state());
                let run_nin_command = format!("cd {:?}", "demo");
                println!("{}", run_nin_command);
                let output = Command::new("cd")
                    .arg(run_nin_command)
                    .output()
                    .unwrap();
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));

                Ok(())
            });*/

    //}
    //Repository::clone(nin_project_repo, )
    //rocket::ignite()
    //.mount("/", routes![index, get_prod])
    //.launch();
}
