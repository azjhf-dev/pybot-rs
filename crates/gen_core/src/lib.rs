#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports)]
pub mod runtime;
pub mod __main__;

pub fn register_all() {
    crate::__main__::register();
    crate::runtime::register();
}
