use diesel::PgConnection;
use crate::api::controller::pagination_helper::Pagination;
use crate::api::resource::category_resource::{CategoryResource, CategoryResourceHal};
use crate::api::resource::relation::{HalLink, Relation};
use crate::service::category_products_service::get_products_for_category;
use crate::shared::auth::auth_user::AuthUser;
use crate::shared::entity::category::Category;
use crate::shared::env_loader::get_loader;
use crate::shared::errors::error_enum::ErrorsEnum;

pub fn map_entity_to_category_resource(connection: &mut PgConnection, auth_user: &AuthUser, entity: &Category) -> Result<CategoryResource, ErrorsEnum> {
    let name = entity.name.to_string();
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

    Ok(CategoryResource::new(name, links))
}

pub fn map_entities_to_category_resources(connection: &mut PgConnection, auth_user: &AuthUser, result: &Vec<Category>) -> Result<Vec<CategoryResource>, ErrorsEnum> {
    result.iter().map(|r|
        map_entity_to_category_resource(connection, auth_user, r)
    ).collect::<Result<Vec<CategoryResource>, ErrorsEnum>>()
}

pub fn map_entity_to_category_resource_hal(connection: &mut PgConnection, auth_user: &AuthUser, entity: &Category) -> Result<CategoryResourceHal, ErrorsEnum> {
    let name = entity.name.to_string();
    let mut category_resource_hal = CategoryResourceHal::new(name);

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

// helper functions
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