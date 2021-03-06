#[macro_export]
macro_rules! CREATE_TAG {
    () => (r#"
CREATE TABLE IF NOT EXISTS ATAG_TAG (
  ID INTEGER PRIMARY KEY,
  TAG_TYPE VARCHAR(255) NOT NULL,
  CREATED_AT VARCHAR(512),
  REMOTE_ADDR VARCHAR(512),
  UNIQUE_TAG VARCHAR(255) NOT NULL,
  URL_FROM VARCHAR(1024) NOT NULL,
  REFERER VARCHAR(1024) NOT NULL,
  HEADERS CLOB
)
"#)
}

#[macro_export]
macro_rules! INSERT_TAG {
    () => (r#"
INSERT INTO ATAG_TAG (
  TAG_TYPE, CREATED_AT, REMOTE_ADDR,
  UNIQUE_TAG, 
  URL_FROM, REFERER, HEADERS)
VALUES(
  '{tag_type}', '{created_at}', '{remote_addr}',
  '{unique_tag}', 
  '{url_from}', '{referer}', '{headers}')
"#)
}

#[macro_export]
macro_rules! INSERT_FOR_MULTI_TAG {
    () => (r#"
INSERT INTO ATAG_TAG (
  TAG_TYPE, CREATED_AT, REMOTE_ADDR,
  UNIQUE_TAG,
  URL_FROM, REFERER, HEADERS)
VALUES {multiple_values_str}
"#)
}

#[macro_export]
macro_rules! SELECT_WITH_WHERE_TAG {
    () => (r#"
SELECT
  ID, TAG_TYPE, UNIQUE_TAG,
  URL_FROM, REFERER, HEADERS,
  CREATED_AT, REMOTE_ADDR
FROM
  ATAG_TAG
WHERE
    {multi_where_statement}
"#)}

pub const SELECT_ALL_TAG: &'static str = r#"
SELECT
  ID, TAG_TYPE, UNIQUE_TAG, 
  URL_FROM, REFERER, HEADERS,
  CREATED_AT, REMOTE_ADDR
FROM
  ATAG_TAG
"#;

pub const SELECT_GROUP_TAG: &'static str = r#"
SELECT
  count(*),
  TAG_TYPE, UNIQUE_TAG, REFERER, REMOTE_ADDR
FROM
  ATAG_TAG
GROUP BY TAG_TYPE, UNIQUE_TAG, REFERER, REMOTE_ADDR
"#;
