pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}