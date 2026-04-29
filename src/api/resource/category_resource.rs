use serde::Serialize;
use crate::api::resource::relation::{HalLink, Relation};

#[derive(Serialize)]
pub struct CategoryResource {
    pub name: String,
    pub links: Vec<Relation>,
}

impl CategoryResource {
    pub fn new(name: String, links: Vec<Relation>) -> Self {
        Self { name, links }
    }
}

// For Hateoas messages in the weird almost HAL format of the source-application
#[derive(Serialize)]
pub struct CategoryResourceHal {
    pub name: String,
    #[serde(rename = "_links")]
    pub links: HalLinks
}

impl CategoryResourceHal {
    pub fn new(name: String) -> Self {
        Self { name, links: HalLinks { self_link: None, parent: None, subcategories: None, products: None } }
    }
}

#[derive(Serialize)]
pub struct HalLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "self")]
    pub self_link: Option<HalLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<HalLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategories: Option<HalLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<HalLink>,
}