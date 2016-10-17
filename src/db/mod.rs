#[macro_use] mod constants;

use crossbeam::sync::chase_lev::{self, Worker, Stealer};
use crossbeam::sync::{self, MsQueue};
use std::fs::File;
use std::path::PathBuf;
use rusqlite::Row;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::ManageConnection;
use super::server::TagRequest;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct GroupedTag {
    count: i64,
    tag_type: String,
    unique_tag: String,
    referer: String,
    remote_addr: String
}

pub struct DbController {
    pub conn_manager: SqliteConnectionManager,
    pub file_path_string: String,
    //    pub worker: Worker<TagRequest>,
    //    pub stealer: Stealer<TagRequest>,
    pub ms_queue: MsQueue<TagRequest>,
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

        //        let (mut worker, stealer) = chase_lev::deque();
        //        return DbController {
        //            conn_manager: conn_manager,
        //            file_path_string: file_path_str.to_string(),
        //            worker :worker,
        //            stealer: stealer,
        //        };
        let ms_queue = MsQueue::new();
        return DbController {
            conn_manager: conn_manager,
            file_path_string: file_path_str.to_string(),
            ms_queue: ms_queue,
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

    /*
    pub fn start_inserting_thread(&'static mut self) {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(4000));
                for i in 0..10 {
                    let maybe_tag = self.ms_queue.try_pop();
                    match maybe_tag {
                        Some(tag) => self.insert_log_to_db(&tag),
                        None => break,
                    }
                }
            }
        });
        //        thread_running+1;
        //        self.running_thread = Some(thread_running);
    }
    */

    fn select_statement<F, T>(&self, select_statement: &str, f: F) -> Vec<T>
        where F: FnMut(&Row) -> T {
        let conn = self.conn_manager.connect();
        let conn = conn.unwrap();
        let statement = conn.prepare(select_statement);
        let mut statement = match statement {
            Ok(stmt) => stmt,
            Err(e) => {
                panic!("Failed to select grouped: {}", e);
            }
        };
        let all_res = statement.query_map(&[], f).unwrap();
        let iter = all_res.map(|x| x.unwrap());
        iter.collect::<Vec<T>>()
    }

    pub fn select_grouped_entries(&self) -> Vec<GroupedTag> {
        let function = |row: &Row| {
            GroupedTag {
                count: row.get(0),
                tag_type: row.get(1),
                unique_tag: row.get(2),
                referer: row.get(3),
                remote_addr: row.get(4)
            }
        };
        self.select_statement(constants::SELECT_GROUP_TAG, function)
    }

    pub fn select_all_entries(&self) -> Vec<TagRequest> {
        self.select_statement(constants::SELECT_ALL_TAG, TagRequest::from_row)
    }

    pub fn insert_log_entry(
        &self,
        tag_request: TagRequest) {
        self.ms_queue.push(tag_request);
    }

    pub fn insert_log_to_db(
        &self,
        tag_request: &TagRequest) {
        self.insert_log_entry_OLD(
            &tag_request.tag_type.to_string(),
            &tag_request.tag,
            &tag_request.url,
            &tag_request.referer,
            &tag_request.headers,
            &tag_request.created_at,
            &tag_request.remote_addr);
    }

    pub fn insert_log_entry_OLD(
        &self,
        tag_type: &str, unique_tag: &str, url_from: &str,
        referer: &str, headers: &str, created_at: &str,
        remote_addr: &str) {
        let conn = self.conn_manager.connect().unwrap();
        let insert_str = format!(
            INSERT_TAG!(),
            tag_type = tag_type,
            unique_tag = unique_tag.replace("'", "''"),
            url_from = url_from.replace("'", "''"),
            referer = referer,
            headers = headers.replace("'", "''"),
            created_at = created_at,
            remote_addr = remote_addr);
        debug!("insert-string:{}", insert_str);

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
