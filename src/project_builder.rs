use std::{fs::{File, self}, io::Read, path::PathBuf};

use json::JsonValue;

use crate::{interpreter::Interpreter, analyzer::analyze};

pub struct Project {
    name: String,
    version: String,
    // libs: Vec<String>,
    files: Vec<PathBuf>
}

impl Project {
    pub fn init(path: String) -> Self {
        let project_file = File::open(format!("{}/{}", path, "project.json"));

        match project_file {
            Ok(mut project_file) => {
                let mut buf = String::new();
                project_file.read_to_string(&mut buf).unwrap();

                if let JsonValue::Object(obj) = json::parse(buf.as_str()).unwrap() {
                    let name = obj["name"].to_string();
                    let version = obj["version"].to_string();
                    let mut files = vec![];

                    let paths = fs::read_dir(format!("{}/{}", path, "src/")).unwrap();
                    for path in paths {
                        files.push(path.unwrap().path());
                    }

                    return Self {
                        name,
                        version,
                        files
                    };
                } else { panic!("The project.json file could not be read") }
            },
            Err(err) => panic!("The project.json was not found: {}", err)
        }
    }

    pub fn build(&self) {
        for file in self.files.iter() {
            if file.file_name().unwrap() == "main.bfp" {
                let file = fs::read_to_string(file).unwrap();
                
                let mut inter = Interpreter::new();
                inter.run(&analyze(file));
            }
        }
    }
}