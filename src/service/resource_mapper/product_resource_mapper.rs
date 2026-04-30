use actix_web::HttpRequest;
use diesel::PgConnection;
use crate::api::controller::pagination_helper::Pagination;
use crate::api::resource::category_resource::CategoryResourceHal;
use crate::api::resource::product_resource::{ProductHalLinks, ProductResource, ProductResourceList, ProductsHalLinks, ProductsResource};
use crate::service::category_service;
use crate::service::resource_mapper::category_resource_mapper;
use crate::shared::auth::auth_user::AuthUser;
use crate::shared::entity::product::ProductWithUser;
use crate::shared::errors::error_enum::{ErrorsEnum, PRODUCT_NOT_FOUND_MSG};


pub fn map_entity_to_products_resource(
    connection: &mut PgConnection,
    auth_user: &AuthUser,
    products: &Vec<ProductWithUser>,
    page: &Pagination,
    req: &HttpRequest,
    total_elements: i64
) -> Result<ProductsResource, ErrorsEnum> {
    let embedded = match products.iter().map(|p| {
        map_entity_to_product_resource(connection, auth_user, p)
    }).collect::<Result<Vec<ProductResource>, ErrorsEnum>>() {
        Ok(embedded) => embedded,
        Err(e) => return Err(e)
    };

    let embedded = match embedded.len() {
        0 => None,
        _ => Some(ProductResourceList { product_resource_list: embedded })
    };

    let links = ProductsHalLinks::new(req)?;
    let page = crate::api::resource::product_resource::Page::new(page, total_elements);

    Ok (ProductsResource::new(embedded, links, page))
}

pub fn map_entity_to_product_resource(
    connection: &mut PgConnection,
    auth_user: &AuthUser,
    product: &ProductWithUser
) -> Result<ProductResource, ErrorsEnum> {
    let categories = match category_service::get_category_for_product(connection, auth_user, product.product.id) {
        Ok(categories) => categories.into_iter().map(|c|
            category_resource_mapper::map_entity_to_category_resource_hal(connection, auth_user, &c)).collect::<Result<Vec<CategoryResourceHal>, ErrorsEnum>>()?,
        Err(_) => return Err(ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string()))
    };

    let categories = match categories.len() {
        0 => None,
        _ => Some(categories)
    };

    Ok (ProductResource {
        name: product.product.name.to_string(),
        currency: "EUR".to_string(),
        price: product.product.price,
        categories,
        owner: product.user.username.to_string(),
        links: ProductHalLinks::from_product(&product.product)?,
    })
}