#[macro_export]
macro_rules! INSERT_VALUES {
    () => (r#"
(
  '{tag_type}', '{created_at}', '{remote_addr}',
  '{unique_tag}',
  '{url_from}', '{referer}', '{headers}')
"#)
}