use std::fs::File;
use std::io;
use std::io::prelude::*;
use yaml_rust::yaml;
use yaml_rust::YamlLoader;


fn main() {
    println!("Hello YAML!");
    let file = "./yaml/test.yaml";
    let y = load_file(file);

    let d1 = &y.unwrap()[0];

    println!("{:#?}", d1["cars"]);
}

fn load_file(file: &str) -> Result<yaml::Yaml, io::Error> {
    let mut file = File::open(file)
        .expect("Unable to open file");
    let mut c = String::new();

    match file.read_to_string(&mut c) {
        Ok(_) => Ok(yaml_rust::Yaml::Array(YamlLoader::load_from_str(&c).unwrap())),
        Err(e) => Err(e),
    }
}