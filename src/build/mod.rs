use std::{env, fs};

mod build;
mod valid;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let mut inf = "".to_owned();
    let mut ouf = "".to_owned();
    for a in args {
        let b: Vec<_> = a.split("=").collect();
        match b[0] {
            "if" => inf = b[1].to_owned(),
            "of" => ouf = b[1].to_owned(),
            &_ => (),
        }
    }
    if !fs::read_dir(&ouf).is_ok() {
        fs::create_dir(&ouf).expect("cannot make dir");
    }
    let files = fs::read_dir(&inf).expect("error reading folder");
    for file in files {
        if let Ok(file) = file {
            if file.file_name().into_string().unwrap().ends_with(".n") {
                println!("{:?}", file.file_name());
                build::build(file.path(), &inf, &ouf);
            }
        }
    }
}
