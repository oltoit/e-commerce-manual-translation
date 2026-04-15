// @generated automatically by Diesel CLI.

diesel::table! {
    app_category (id) {
        id -> Int8,
        #[max_length = 100]
        name -> Varchar,
        parentid -> Nullable<Int8>,
    }
}

diesel::table! {
    app_product (id) {
        id -> Int8,
        #[max_length = 300]
        name -> Varchar,
        price -> Float8,
        userid -> Int8,
    }
}

diesel::table! {
    app_product_category (productid, categoryid) {
        productid -> Int8,
        categoryid -> Int8,
    }
}

diesel::table! {
    app_user (id) {
        id -> Int8,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 10]
        role -> Varchar,
    }
}

diesel::joinable!(app_product -> app_user (userid));
diesel::joinable!(app_product_category -> app_category (categoryid));
diesel::joinable!(app_product_category -> app_product (productid));

diesel::allow_tables_to_appear_in_same_query!(
    app_category,
    app_product,
    app_product_category,
    app_user,
);