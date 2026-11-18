pub mod modules;
pub fn register_all() {
crate::modules::safety_cards::register();
}
