use toml;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub sensitive_path_key: String
}

impl Config {
    pub fn new() -> Config {
        let toml_value = Config::read_init_config_from_file();
        let root_config = toml_value.as_table().expect("Root should be table");
        let path_configs = root_config.get("paths").expect("Should have 'paths' configured.")
            .as_table().expect("Path config should be table");
        let sensitive_path_key = path_configs
            .get("sensitive_path_key").expect("Should contain the sensitive_path_key")
            .as_str().expect("Sensitive path key should be a string");

        Config {
            sensitive_path_key: sensitive_path_key.to_owned()
        }
    }

    fn read_init_config_from_file() -> toml::Value {
        let mut f = File::open("NEVER_COMMIT/tagaton.toml")
            .ok()
            .expect("File not found");
        let mut toml_string = String::new();
        // try!(f.read_to_string(&mut toml_string));
        f.read_to_string(&mut toml_string).unwrap();

        let mut toml_parser = toml::Parser::new(toml_string.as_ref());

        let toml_table = toml_parser.parse().unwrap();

        debug!("toml: {:?}", toml_table);
        toml::Value::Table(toml_table)
    }
}