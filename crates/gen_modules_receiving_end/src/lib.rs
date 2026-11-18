pub mod modules;
pub fn register_all() {
crate::modules::receiving_end::register();
}
