pub mod modules;
pub fn register_all() {
crate::modules::on_record::register();
}
