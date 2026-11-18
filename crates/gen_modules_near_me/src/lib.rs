pub mod modules;
pub fn register_all() {
crate::modules::near_me::register();
}
