#[macro_use] pub mod constants;

//use crossbeam::sync::MsQueue;
use crossbeam::sync::SegQueue;
use std::collections::BTreeMap;
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
    pub ms_queue: SegQueue<TagRequest>,
    param_name_to_column_name: BTreeMap<String, String>,
}

impl DbController {
    pub fn new(file_path_str: &str) -> DbController {
        create_file_if_not_exists(file_path_str);
        let conn_manager = init_connection(file_path_str);

        let conn = (&conn_manager).connect().unwrap();

        let create_table_str = format!("{}", CREATE_TAG!());
        let create_table = (&conn).execute(&create_table_str, &[]);
        match create_table {
            Ok(_) => debug!("Created table"),
            Err(e) => panic!("APPLICATION_ERROR: Failed badly: {}", e)
        };

        let ms_queue = SegQueue::new();
        return DbController {
            conn_manager: conn_manager,
            file_path_string: file_path_str.to_string(),
            ms_queue: ms_queue,
            param_name_to_column_name: init_columns_wanted(),
        };

        fn init_columns_wanted() -> BTreeMap<String, String> {
            let mut param_to_column_name = BTreeMap::new();

            insert(&mut param_to_column_name, "tag_type", "TAG_TYPE");
            insert(&mut param_to_column_name, "remote_addr", "REMOTE_ADDR");
            insert(&mut param_to_column_name, "unique_tag", "UNIQUE_TAG");
            insert(&mut param_to_column_name, "url_from", "URL_FROM");
            insert(&mut param_to_column_name, "referer", "REFERER");

            return param_to_column_name;

            fn insert<'a>(map: &'a mut BTreeMap<String, String>, key: &'static str, value: &'static str)
                -> &'a BTreeMap<String, String>
            {
                map.insert(key.to_owned(), value.to_owned());
                map
            }
        }

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

    fn select_statement<F, T>(&self, select_statement: &str, f: F) -> Vec<T>
        where F: FnMut(&Row) -> T {
        let conn = self.conn_manager.connect();
        let conn = conn.unwrap();
        let statement = conn.prepare(select_statement);
        let mut statement = match statement {
            Ok(stmt) => stmt,
            Err(e) => {
                panic!("APPLICATION_ERROR: Failed to select grouped: {}", e);
            }
        };
        let all_res = statement.query_map(&[], f).unwrap();
        let iter = all_res.map(|x| x.unwrap());
        iter.collect::<Vec<T>>()
    }

    pub fn select_where_entries(
        &self, param_to_column_value: BTreeMap<String, String>) -> Vec<TagRequest>
    {
        let ref param_name_to_column_name = self.param_name_to_column_name;
        let where_entries = DbController::generate_where_entries(
            &param_to_column_value, &param_name_to_column_name);
        let where_string = where_entries.join(" AND ");
        let query_with_where = format!(
            SELECT_WITH_WHERE_TAG!(),
            multi_where_statement = where_string);
        self.select_statement(&query_with_where, TagRequest::from_row)
    }

    fn generate_where_entries(
        param_to_column_value: &BTreeMap<String, String>,
        param_name_to_column_name: &BTreeMap<String, String>)
        -> Vec<String>
    {
        let mut where_entries = vec![];
        println!("param_to_column_name: {:?}", param_to_column_value);
        println!("db_param_name_column_values_map: {:?}", param_name_to_column_name);
        for (param_name, column_name) in param_name_to_column_name.iter() {
            if let Some(column_value) = param_to_column_value.get(param_name) {
                where_entries.push(format!("{} = '{}'", column_name, column_value));
            };
        }
        where_entries
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
        info!("About to insert to queue");
        self.ms_queue.push(tag_request);
    }

    pub fn insert_many_log_to_db(
        &self,
        tag_request_list: Vec<TagRequest>) {
        let results: Vec<String> = tag_request_list.into_iter()
            .map(|ref curr: TagRequest| curr.log_entry_to_string())
            .collect();
        let to_insert = results.join(", ");
        let insert_str = format!(
            INSERT_FOR_MULTI_TAG!(),
            multiple_values_str = to_insert
        );
        self.generic_db_insert(&insert_str);
    }

    fn generic_db_insert(
        &self, insert_str: &str) {
        debug!("generic_db_insert; insert-string: \n{}", insert_str);
        let conn = self.conn_manager.connect().unwrap();
        let insert_stmt = conn.execute(insert_str, &[]);
        match insert_stmt {
            Ok(_) => {
                debug!("Inserted!");
            },
            Err(e) => {
                panic!("APPLICATION_ERROR: Failed to insert: {}", e);
            }
        };
    }
}
