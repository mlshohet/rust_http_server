use super::method::Method;
// Option enum is automatically imported
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
