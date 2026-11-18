pub mod memory;
pub fn register_all() {
crate::memory::thread::register();
}
