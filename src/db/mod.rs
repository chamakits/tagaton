#[macro_use]
mod constants;

use std::path::PathBuf;
use std::fs::File;
use rusqlite::Connection;

pub struct DbController {
    // This connection is currently kinda useless
    pub conn: Connection,
    pub file_path_string: String
}

unsafe impl Sync for DbController {}

impl DbController {
    pub fn new(file_path_str: &str) -> DbController {
        let conn = DbController::init_connection(file_path_str);

        let create_table_str = format!("{}", CREATE_TAG!());
        let create_table = (&conn).execute(&create_table_str, &[]);
        match create_table {
            Ok(_) => debug!("Created table"),
            Err(e) => panic!("Failed badly: {}", e)
        };
        
        DbController {
            conn: conn,
            file_path_string: file_path_str.to_string()
        }
    }

    fn init_connection(file_path_str: &str) -> Connection {
        DbController::init_db_if_not_exist_and_connect(file_path_str)
    }

    pub fn insert_log_entry(
        &self, unique_tag: &str, url_from: &str,
        referer: &str, headers: &str) {

        //let conn = (&self.conn);
        // This is currently unfortunate, as it reopens sqlite connection every time
        // However, without this, it isn't writing safely to sqlite.
        // Fortunately, they are both kinda equally slow. So maybe it isn't a problem?
        let conn = Connection::open(&self.file_path_string).unwrap();

        let insert_str = format!(
            INSERT_TAG!(),
            unique_tag = unique_tag,
            url_from = url_from,
            referer = referer,
            headers = headers);
        let insert_stmt = conn.execute(&insert_str, &[]);
        match insert_stmt {
            Ok(_) => {
                debug!("Inserted!");
            },
            Err(e) => {
                //error!("Failed to insert: {}", e);
                panic!("Failed to insert: {}", e);
            }
        };

    }

    fn init_db_if_not_exist_and_connect(file_path_str: &str) -> Connection {
        let file_path = PathBuf::from(file_path_str);
        if !file_path.exists() {
            match File::create(&file_path) {
                Err(e) => panic!("Could not create file for db {}", e),
                Ok(_) => (),
            };
        }
        let conn = Connection::open(file_path).unwrap();
        conn
    }
}
