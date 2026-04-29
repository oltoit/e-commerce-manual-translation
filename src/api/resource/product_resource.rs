use actix_web::HttpRequest;
use serde::Serialize;
use crate::api::controller::pagination::Pagination;
use crate::api::resource::category_resource::CategoryResourceHal;
use crate::api::resource::relation::{HalLink, Relation};
use crate::shared::env_loader::get_loader;
use crate::shared::entity::product::Product;
use crate::shared::errors::error_enum::ErrorsEnum;

#[derive(Serialize)]
pub struct ProductsResource {
    #[serde(rename = "_embedded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    embedded: Option<ProductResourceList>,
    #[serde(rename = "_links")]
    links: ProductsHalLinks,
    page: Page
}

impl ProductsResource {
    pub fn new(embedded: Option<ProductResourceList>, links: ProductsHalLinks, page: Page) -> Self {
        Self { embedded, links, page }
    }
}

#[derive(Serialize)]
pub struct ProductResourceList {
    #[serde(rename = "productResourceList")]
    pub product_resource_list: Vec<ProductResource>,
}
#[derive(Serialize)]
pub struct ProductResource {
    pub name: String,
    pub currency: String,
    pub price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<CategoryResourceHal>>,
    pub owner: String,
    #[serde(rename = "_links")]
    pub links: ProductHalLinks,
}

impl ProductResource {
    pub fn new(name: String, currency: String, price: f64, categories: Option<Vec<CategoryResourceHal>>, owner: String, links: ProductHalLinks) -> Self {
        Self { name, currency, price, categories, owner, links }
    }
}

fn get_self_link(product: &Product) -> Result<Relation, ErrorsEnum> {
    let rel = "self".to_string();
    let href = String::from(format!("{}/products/{}", get_loader()?.get_base_url(), product.id));
    Ok(Relation { rel, href })
}

#[derive(Serialize)]
pub struct ProductHalLinks {
    #[serde(rename = "self")]
    pub self_link: HalLink,
}
impl ProductHalLinks {
    pub fn from_product(product: &Product) -> Result<Self, ErrorsEnum> {
        Ok(Self { self_link: HalLink { href: get_self_link(product)?.href} })
    }
}

fn get_paginated_self_link(req: &HttpRequest) -> Result<Relation, ErrorsEnum> {
    let rel = "self".to_string();
    let href = String::from(format!(
        "{}/{}",
        get_loader()?.get_base_url(),
        req.match_info().as_str()
    ));
    Ok(Relation { rel, href })
}

#[derive(Serialize)]
pub struct ProductsHalLinks {
    #[serde(rename = "self")]
    pub self_link: HalLink,
}
impl ProductsHalLinks {
    pub fn new(req: &HttpRequest) -> Result<Self, ErrorsEnum> {
        Ok(Self { self_link: HalLink { href: get_paginated_self_link(req)?.href} })
    }
}

#[derive(Serialize)]
pub struct Page {
    pub size: i64,
    #[serde(rename = "totalElements")]
    pub total_elements: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
    pub number: i64,
}
impl Page {
    pub fn new(pagination: &Pagination, total_elements: i64) -> Self {
        let sub_total = total_elements % pagination.get_size();
        let total_pages = total_elements / pagination.get_size() + (1 * (sub_total != 0) as i64);
        Page {
            size: pagination.get_size(),
            total_elements,
            total_pages,
            number: pagination.get_page(),
        }
    }
}