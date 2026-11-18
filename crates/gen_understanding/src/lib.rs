#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports)]
pub mod understanding;

pub fn register_all() {
    crate::understanding::register();
    crate::understanding::attempts::register();
    crate::understanding::body::register();
    crate::understanding::breeds::register();
    crate::understanding::car_glass::register();
    crate::understanding::cars::register();
    crate::understanding::confusion::register();
    crate::understanding::kinds::register();
    crate::understanding::lexicon::register();
    crate::understanding::normalize::register();
    crate::understanding::references::register();
    crate::understanding::senses::register();
    crate::understanding::topics::register();
}
