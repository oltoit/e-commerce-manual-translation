use std::cmp::{max, min};
use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<i64>,
    size: Option<i64>,
    sort: Option<String>,
}

pub const DEFAULT_PAGE: i64 = 0;
pub const DEFAULT_PAGE_SIZE: i64 = 20;
pub const MIN_PAGE_SIZE: i64 = 1;
pub const MAX_PAGE_SIZE: i64 = 2_000;
pub const DEFAULT_SORT: &str = "id,asc";

impl Pagination {
    pub fn default() -> Self {
        Self {
            page: None,
            size: None,
            sort: None,
        }
    }
    pub fn get_page(&self) -> i64 {
        let page = self.page.unwrap_or(DEFAULT_PAGE);
        max(DEFAULT_PAGE, page)
    }
    pub fn get_size(&self) -> i64 {
        let mut size = self.size.unwrap_or(DEFAULT_PAGE_SIZE);
        if size <= 0 { size = DEFAULT_PAGE_SIZE }
        min(MAX_PAGE_SIZE, max(size, MIN_PAGE_SIZE))
    }
    pub fn get_unsanitized_sort(&self) -> &str {
        self.sort.as_deref().unwrap_or(DEFAULT_SORT)
    }
}

pub fn get_optional_pagination(pagination: Option<web::Query<Pagination>>) -> Pagination {
    if pagination.is_some() {
        pagination.unwrap().into_inner()
    } else {
        Pagination::default()
    }
}