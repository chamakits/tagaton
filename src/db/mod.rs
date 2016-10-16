#[macro_use]
mod constants;

use std::path::PathBuf;
use std::fs::File;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::ManageConnection;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct GroupedTag {
    count: i64,
    tag_type: String,
    unique_tag: String,
    referer: String,
}

pub struct DbController {
    pub conn_manager: SqliteConnectionManager,
    pub file_path_string: String,
}

unsafe impl Sync for DbController {}

impl DbController {
    pub fn new(file_path_str: &str) -> DbController {
        create_file_if_not_exists(file_path_str);
        let conn_manager = init_connection(file_path_str);
        
        let conn = (&conn_manager).connect().unwrap();
        
        let create_table_str = format!("{}", CREATE_TAG!());
        let create_table = (&conn).execute(&create_table_str, &[]);
        match create_table {
            Ok(_) => debug!("Created table"),
            Err(e) => panic!("Failed badly: {}", e)
        };
        
        return DbController {
            conn_manager: conn_manager,
            file_path_string: file_path_str.to_string(),
        };

        fn create_file_if_not_exists(file_path_str: &str) {
            let file_path = PathBuf::from(file_path_str);
            if !file_path.exists() {
                match File::create(&file_path) {
                    Err(e) => panic!("Could not create file for db {}", e),
                    Ok(_) => (),
                };
            }
        }

        fn init_connection(file_path_str: &str) -> SqliteConnectionManager {
            SqliteConnectionManager::new(file_path_str)
        }
    }

    pub fn select_grouped_entries(&self) -> Vec<GroupedTag> {
        let conn = self.conn_manager.connect();
        let conn = conn.unwrap();
        let statement = conn.prepare(constants::SELECT_GROUP_TAG);
        let mut statement = match statement {
            Ok(stmt) => stmt,
            Err(e) => {
                panic!("Failed to select grouped: {}", e);
            }
        };
        let all_res = statement.query_map(&[], |row| {
            GroupedTag {
                count: row.get(0),
                tag_type: row.get(1),
                unique_tag: row.get(2),
                referer: row.get(3),
            }
        }).unwrap();
        let iter = all_res.map(|x| x.unwrap());
        iter.collect::<Vec<GroupedTag>>()
    }
    
    pub fn insert_log_entry(
        &self,
        tag_type: &str, unique_tag: &str, url_from: &str,
        referer: &str, headers: &str, created_at: &str,
        remote_addr: &str) {

        //let conn = (&self.conn);
        // This is currently unfortunate, as it reopens sqlite connection every time
        // However, without this, it isn't writing safely to sqlite.
        // Fortunately, they are both kinda equally slow. So maybe it isn't a problem?
        //let conn = Connection::open(&self.file_path_string).unwrap();
        //let conn = Connection::open(&self.file_path_string).unwrap();
        let conn = self.conn_manager.connect().unwrap();

        let insert_str = format!(
            INSERT_TAG!(),
            tag_type = tag_type,
            unique_tag = unique_tag,
            url_from = url_from,
            referer = referer,
            headers = headers,
            created_at = created_at,
            remote_addr = remote_addr);
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

}
