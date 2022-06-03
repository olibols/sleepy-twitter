//use reqwest::*;
use reqwest::{Method, Body};

pub struct QueryBuilder<'a>{
    base_uri: &'a str,
    method: Method,
    params: Option<String>,
    query: Option<String>,
    body: Option<(Body, &'static str)>
}

impl<'a> QueryBuilder<'a>{
    pub fn new(method: Method, base_uri: &'a str) -> Self {
        QueryBuilder{
            base_uri,
            method,
            params: None,
            query: None,
            body: None,
        }
    }
}