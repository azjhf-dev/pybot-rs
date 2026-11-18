pub mod modules;
pub fn register_all() {
crate::modules::onsubject::register();
}
