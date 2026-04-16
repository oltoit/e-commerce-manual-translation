use diesel::PgConnection;
use serde::Serialize;
use crate::api::resource::relation::Relation;
use crate::config::env_loader::LOADER;
use crate::entity::category::Category;
use crate::service::product_category_service::get_products_for_categories_recursive;

#[derive(Serialize)]
pub struct CategoryResource<'a> {
    name: &'a str,
    links: Vec<Relation>,
}

impl<'a> CategoryResource<'a> {
    pub fn from_entity(connection: &mut PgConnection, entity: &'a Category) -> Self {
        let name = &entity.name;
        let mut links = vec![];

        links.push(get_self_for_resource(entity.id));

        if entity.parentid.is_some() {
            links.push(get_parent_for_resource(entity.parentid.unwrap()))
        }

        // This can be set in every case, since the source-application only checks for not-null on the Set.
        // The Set is always initialized, just not always filled.
        links.push(get_subcategories_for_resource(entity.id));

        if get_products_for_categories_recursive(connection, entity.id).len() > 0 {
            links.push(get_products_for_resource(entity.id))
        }

        Self { name, links }
    }
}

fn get_self_for_resource(id: i64) -> Relation {
    let url = String::from(format!("{}/categories/{}", LOADER.get().unwrap().get_base_url(), id));
    let rel_name = "self".to_string();
    Relation {rel: rel_name, href: url}
}

fn get_parent_for_resource(parent_id: i64) -> Relation {
    let url = String::from(format!("{}/categories/{}", LOADER.get().unwrap().get_base_url(), parent_id));
    let rel_name = "parent".to_string();
    Relation {rel: rel_name, href: url}
}

fn get_subcategories_for_resource(id: i64) -> Relation {
    let url = String::from(format!("{}/categories/{}/subcategories", LOADER.get().unwrap().get_base_url(), id));
    let rel_name = "subcategories".to_string();
    Relation {rel: rel_name, href: url}
}

fn get_products_for_resource(id: i64) -> Relation {
    let url = String::from(format!("{}/categories/{}/products", LOADER.get().unwrap().get_base_url(), id));
    let rel_name = "products".to_string();
    Relation {rel: rel_name, href: url}
}

// For Hateoas messages in the weird almost HAL format of the source-application
#[derive(Serialize)]
pub struct CategoryResourceHal<'a> {
    name: &'a str,
    #[serde(rename = "_links")]
    links: HalLinks
}

impl<'a> CategoryResourceHal<'a> {
    fn new(name: &'a str) -> Self {
        Self { name, links: HalLinks { self_link: None, parent: None, subcategories: None, products: None } }
    }
    pub fn from_entity(connection: &mut PgConnection, entity: &'a Category) -> Self {
        let name = &entity.name;
        let mut category_resource_hal = CategoryResourceHal::new(name);

        category_resource_hal.links.self_link = Some(HalLink { href: get_self_for_resource(entity.id).href });
        category_resource_hal.links.subcategories = Some(HalLink {href: get_subcategories_for_resource(entity.id).href });

        if entity.parentid.is_some() {
            category_resource_hal.links.parent = Some(HalLink { href: get_parent_for_resource(entity.parentid.unwrap()).href });
        }
        if get_products_for_categories_recursive(connection, entity.id).len() > 0 {
            category_resource_hal.links.products = Some(HalLink { href: get_products_for_resource(entity.id).href });
        }

        category_resource_hal
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

#[derive(Serialize)]
pub struct HalLink {
    pub href: String,
}