pub mod compose;
pub fn register_all() {
crate::compose::urgent::register();
}
