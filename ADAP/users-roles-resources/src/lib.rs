pub mod resources;
pub mod users;
pub mod sessions;
pub mod blake3;


// App's salt value.
use std::sync::OnceLock;
static APP_SALT : OnceLock<[u8;32]> = OnceLock::new();
pub fn app_salt() -> [u8;32] {
    let salt = APP_SALT.get_or_init(||{
        [0;32]
    });

    salt.clone()
}