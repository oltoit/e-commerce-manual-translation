use serde::Serialize;

#[derive(Serialize)]
pub struct Relation {
    pub rel: String,
    pub href: String,
}

#[derive(Serialize)]
pub struct HalLink {
    pub href: String,
}