#[macro_use]
mod constants;

use std::path::PathBuf;
use rusqlite::Connection;

pub fn insert_log_entry(
    unique_tag: &str, url_from: &str, referer: &str, headers: &str) {
    
}

pub fn create_file_if_not_exist(file_path_str: &str) {
    let file_path = PathBuf::from(file_path_str);
    if(!file_path.exists()) {
        
    }
}
