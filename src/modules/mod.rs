//! The ported modules, registered highest-traffic first.
//!
//! Upstream has 528 files and ~640k lines of rules. This port carries the
//! architecture and a curated subset of the conversation behaviours; see
//! README.md for the honest coverage table.

pub mod arbiter;
pub mod facts;
pub mod fallback;
pub mod farewell;
pub mod greeting;
pub mod howareyou;
pub mod knowledge;
pub mod safety;
pub mod thanks;
pub mod util;

use crate::runtime::Module;

pub fn all() -> Vec<Box<dyn Module>> {
    vec![
        Box::new(safety::Safety),
        Box::new(facts::Facts),
        Box::new(greeting::Greeting),
        Box::new(howareyou::HowAreYou),
        Box::new(thanks::Thanks),
        Box::new(farewell::Farewell),
        Box::new(knowledge::Knowledge),
        Box::new(knowledge::FollowUp),
        Box::new(fallback::Fallback),
    ]
}
