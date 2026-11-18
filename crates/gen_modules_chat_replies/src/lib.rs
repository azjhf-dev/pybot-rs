pub mod modules;
pub fn register_all() {
crate::modules::chat_replies::register();
}
