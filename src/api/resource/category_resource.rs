use diesel::PgConnection;
use serde::Serialize;
use crate::api::controller::pagination::Pagination;
use crate::api::resource::relation::{HalLink, Relation};
use crate::shared::env_loader::get_loader;
use crate::shared::entity::category::Category;
use crate::shared::errors::error_enum::ErrorsEnum;
use crate::service::category_products_service::get_products_for_category;
use crate::shared::auth::auth_user::AuthUser;

#[derive(Serialize)]
pub struct CategoryResource<'a> {
    name: &'a str,
    links: Vec<Relation>,
}

impl<'a> CategoryResource<'a> {
    pub fn from_entity(connection: &mut PgConnection, auth_user: &AuthUser, entity: &'a Category) -> Result<Self, ErrorsEnum> {
        let name = &entity.name;
        let mut links = vec![];

        links.push(get_self_for_resource(entity.id)?);

        if let Some(parent_id) = entity.parentid {
            links.push(get_parent_for_resource(parent_id)?)
        }

        // This can be set in every case, since the source-application only checks for not-null on the Set.
        // The Set is always initialized, just not always filled.
        links.push(get_subcategories_for_resource(entity.id)?);

        let mut pagination = Pagination::default();
        pagination.set_size(1);
        if get_products_for_category(connection, auth_user, &pagination, entity.id)?.0.len() > 0 {
            links.push(get_products_for_resource(entity.id)?)
        }

        Ok(Self { name, links })
    }

    pub fn map_from_entities(connection: &mut PgConnection, auth_user: &AuthUser, result: &'a Vec<Category>) -> Result<Vec<Self>, ErrorsEnum> {
        result.iter().map(|r|
            CategoryResource::from_entity(connection, auth_user, r)
        ).collect::<Result<Vec<CategoryResource>, ErrorsEnum>>()
    }
}

fn get_self_for_resource(id: i64) -> Result<Relation, ErrorsEnum> {
    let url = String::from(format!("{}/categories/{}", get_loader()?.get_base_url(), id));
    let rel_name = "self".to_string();
    Ok(Relation {rel: rel_name, href: url})
}

fn get_parent_for_resource(parent_id: i64) -> Result<Relation, ErrorsEnum> {
    let url = String::from(format!("{}/categories/{}", get_loader()?.get_base_url(), parent_id));
    let rel_name = "parent".to_string();
    Ok(Relation {rel: rel_name, href: url})
}

fn get_subcategories_for_resource(id: i64) -> Result<Relation, ErrorsEnum> {
    let url = String::from(format!("{}/categories/{}/subcategories", get_loader()?.get_base_url(), id));
    let rel_name = "subcategories".to_string();
    Ok(Relation {rel: rel_name, href: url})
}

fn get_products_for_resource(id: i64) -> Result<Relation, ErrorsEnum> {
    let url = String::from(format!("{}/categories/{}/products", get_loader()?.get_base_url(), id));
    let rel_name = "products".to_string();
    Ok(Relation {rel: rel_name, href: url})
}

// For Hateoas messages in the weird almost HAL format of the source-application
#[derive(Serialize)]
pub struct CategoryResourceHal {
    name: String,
    #[serde(rename = "_links")]
    links: HalLinks
}

impl CategoryResourceHal {
    fn new(name: String) -> Self {
        Self { name, links: HalLinks { self_link: None, parent: None, subcategories: None, products: None } }
    }
    pub fn from_entity(connection: &mut PgConnection, auth_user: &AuthUser, entity: &Category) -> Result<Self, ErrorsEnum> {
        let name = &entity.name;
        let mut category_resource_hal = CategoryResourceHal::new(name.to_string());

        category_resource_hal.links.self_link = Some(HalLink { href: get_self_for_resource(entity.id)?.href });
        category_resource_hal.links.subcategories = Some(HalLink {href: get_subcategories_for_resource(entity.id)?.href });

        if let Some(parent_id) = entity.parentid {
            category_resource_hal.links.parent = Some(HalLink { href: get_parent_for_resource(parent_id)?.href });
        }

        let mut pagination = Pagination::default();
        pagination.set_size(1);
        if get_products_for_category(connection, auth_user, &pagination, entity.id)?.0.len() > 0 {
            category_resource_hal.links.products = Some(HalLink { href: get_products_for_resource(entity.id)?.href });
        }

        Ok(category_resource_hal)
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