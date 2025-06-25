use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct LoginData {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize)]
pub(crate) struct UpdatePasswordData {
    pub(crate) new_password: String,
}

#[derive(Deserialize)]
pub(crate) struct Template {
    pub(crate) nation: String,
    pub(crate) tgid: i32,
    pub(crate) key: String,
}
