use diesel::PgConnection;
use serde::Serialize;
use crate::api::controller::pagination::Pagination;
use crate::api::resource::category_resource::CategoryResourceHal;
use crate::api::resource::relation::{HalLink, Relation};
use crate::config::env_loader::LOADER;
use crate::entity::product::{Product, ProductWithUser};
use crate::errors::error_enum::{ErrorsEnum, PRODUCT_NOT_FOUND_MSG};
use crate::security::auth_context_holder::AuthUser;
use crate::service::category_service;

#[derive(Serialize)]
pub struct ProductsResource {
    #[serde(rename = "_embedded")]
    embedded: ProductResourceList,
    #[serde(rename = "_links")]
    links: ProductsHalLinks,
    page: Page
}

impl ProductsResource {
    pub fn new(connection: &mut PgConnection, auth_user: &AuthUser, products: &Vec<ProductWithUser>, page: &Pagination, total_elements: i64) -> Result<Self, ErrorsEnum> {
        let embedded = match products.iter().map(|p| {
            ProductResource::from_product(connection, auth_user, p)
        }).collect::<Result<Vec<ProductResource>, ErrorsEnum>>() {
            Ok(embedded) => embedded,
            Err(e) => return Err(e)
        };

        let links = ProductsHalLinks::new(page);
        let page = Page::new(page, total_elements);

        Ok (ProductsResource {embedded: ProductResourceList { product_resource_list: embedded }, links, page})
    }
}

#[derive(Serialize)]
struct ProductResourceList {
    #[serde(rename = "productResourceList")]
    product_resource_list: Vec<ProductResource>,
}
#[derive(Serialize)]
pub struct ProductResource {
    name: String,
    currency: String,
    price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<Vec<CategoryResourceHal>>,
    owner: String,
    #[serde(rename = "_links")]
    links: ProductHalLinks,
}

impl ProductResource {
    pub fn from_product(connection: &mut PgConnection, auth_user: &AuthUser, product: &ProductWithUser) -> Result<Self, ErrorsEnum> {
        let categories = match category_service::get_category_for_product(connection, auth_user, product.product.id) {
            Ok(categories) => Some(categories.iter().map(|c|
                CategoryResourceHal::from_entity(connection, c)).collect::<Vec<CategoryResourceHal>>()
            ),
            Err(_) => return Err(ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string()))
        };
        Ok (Self {
            name: product.product.name.to_string(),
            currency: "EUR".to_string(),
            price: product.product.price,
            categories: match categories {
                Some(categories) => {
                   if categories.len() > 0 { Some(categories) } else { None }
                }, None => None
            },
            owner: product.user.username.to_string(),
            links: ProductHalLinks::from_product(&product.product),
        })
    }
}

fn get_self_link(product: &Product) -> Relation {
    let rel = "self".to_string();
    let href = String::from(format!("{}/products/{}", LOADER.get().unwrap().get_base_url(), product.id));
    Relation { rel, href }
}

#[derive(Serialize)]
struct ProductHalLinks {
    #[serde(rename = "self")]
    self_link: HalLink,
}
impl ProductHalLinks {
    fn from_product(product: &Product) -> Self {
        Self { self_link: HalLink { href: get_self_link(product).href} }
    }
}

fn get_paginated_self_link(pagination: &Pagination) -> Relation {
    let rel = "self".to_string();
    let href = String::from(format!(
        "{}/products?page={}&size={}",
        LOADER.get().unwrap().get_base_url(),
        pagination.get_page(),
        pagination.get_size()
    ));
    Relation { rel, href }
}

#[derive(Serialize)]
struct ProductsHalLinks {
    #[serde(rename = "self")]
    self_link: HalLink,
}
impl ProductsHalLinks {
    fn new(pagination: &Pagination) -> Self {
        Self { self_link: HalLink { href: get_paginated_self_link(pagination).href} }
    }
}

#[derive(Serialize)]
struct Page {
    pub size: i64,
    #[serde(rename = "totalElements")]
    pub total_elements: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
    pub number: i64,
}
impl Page {
    fn new(pagination: &Pagination, total_elements: i64) -> Self {
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