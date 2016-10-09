use std::path::PathBuf;
use flexi_logger::{self, LogConfig};
pub fn setup_logging() {
    let log_dir = setup_get_logging_dir();
    let log_config = log_config(log_dir);
    let log_init_res = flexi_logger::init(log_config, None);
    match log_init_res {
        Ok(_) => println!("Log initialized"),
        Err(error) => panic!("Issue starting the logger. {}", error)
    }
}

fn log_config(log_dir: String) -> LogConfig {
    let mut log_config = LogConfig::new();
    log_config.rotate_over_size = Some(1024 * 100_000);
    log_config.directory = Some(log_dir);
    log_config.log_to_file = true;
    log_config.timestamp = true;
    log_config.format = flexi_logger::detailed_format;
    log_config
}

use std::fs::DirBuilder;
fn setup_get_logging_dir() -> String {
    let log_dir = "./NEVER_COMMIT/flexi_logs";
    
    let path = PathBuf::from(log_dir);

    let mut dir_builder = DirBuilder::new();
    dir_builder.recursive(true);
    let was_created = dir_builder.create(path.as_path());
    match was_created {
        Ok(_) => (),
        Err(error) => println!("Directory not created, likely already exists. {}", error),
    }
    log_dir.to_owned()
}
