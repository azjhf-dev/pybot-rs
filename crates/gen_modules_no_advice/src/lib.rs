pub mod modules;
pub fn register_all() {
crate::modules::no_advice::register();
}
