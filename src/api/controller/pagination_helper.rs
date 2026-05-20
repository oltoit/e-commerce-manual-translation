use std::cmp::{max, min};
use serde::Deserialize;
use serde_qs::actix::QsQuery;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<i64>,
    size: Option<i64>,
    sort: Option<Vec<String>>,
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
    pub fn set_size(&mut self, mut size: i64) {
        if size <= 0 { size = DEFAULT_PAGE_SIZE }
        self.size = Some(min(MAX_PAGE_SIZE, max(size, MIN_PAGE_SIZE)));
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
    pub fn get_unsanitized_sorts(&self) -> Vec<&str> {
        match &self.sort {
            Some(sorts) => sorts.iter().map(|sort| sort.as_str()).collect(),
            None => vec![DEFAULT_SORT],
        }
    }

    pub fn as_str(&self) -> String {
        let mut query = String::new();
        if let Some(page) = self.page {
            query.push_str(&format!("page={}&", page));
        } else {
            query.push_str(&format!("page={}&", DEFAULT_PAGE));
        }

        if let Some(size) = self.size {
            query.push_str(&format!("size={}&", size));
        } else {
            query.push_str(&format!("size={}&", DEFAULT_PAGE_SIZE));
        }

        if let Some(sorts) = &self.sort {
            query.push_str(&format!("sort={}&", sorts.join(",")));
        }

        if query.ends_with('&') { query.pop(); }
        query
    }
}

pub fn get_optional_pagination(pagination: Option<QsQuery<Pagination>>) -> Pagination {
    if let Some(pagination) = pagination {
        pagination.into_inner()
    } else {
        Pagination::default()
    }
}