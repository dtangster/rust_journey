pub mod request {
    use super::method::Method;

    pub struct Request {
        path: String,
        query_string: Option<String>,
        method: Method,
    }
}

pub mod method {
    pub enum Method {
        DELETE,
        GET,
        HEAD,
        OPTIONS,
        PATCH,
        POST,
        PUT,
        TRACE,
    }
}
