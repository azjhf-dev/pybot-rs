#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ACUTE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALREADY_TRIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANIMAL_MODIFIERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_TO_CHOOSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DECLINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_PACKED_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREAKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BROKEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BROKEN_NEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BROKEN_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CANT_GET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEAD_AS_VERY_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATHWISH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DECLINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DECLINE_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DURATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FAULT_FILLERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FLAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FLAT_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRACTURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GROWING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOINING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_AT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_FINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_NOT_THEIR_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_OF_A_HOUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_POSSESSIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_THEIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MINE_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_MODIFIER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_OWNED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUISANCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ODD_MARK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWNER_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLEA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SORE_NEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SORE_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_FAILING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STRAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STRAIN_NEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STRAIN_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_END_OF_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_STATE_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIGGER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANTING_OR_PRAISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_IT_DOES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_HEDGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_LENGTH_OF_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_STRAIN_DETAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ACUTE() -> Py { ensure_init(); ACUTE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACUTE(v: Py) { ensure_init(); if let Ok(mut w) = ACUTE.write() { *w = v; } }
pub fn g_ALREADY_TRIED() -> Py { ensure_init(); ALREADY_TRIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALREADY_TRIED(v: Py) { ensure_init(); if let Ok(mut w) = ALREADY_TRIED.write() { *w = v; } }
pub fn g_ANIMAL_MODIFIERS() -> Py { ensure_init(); ANIMAL_MODIFIERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_MODIFIERS(v: Py) { ensure_init(); if let Ok(mut w) = ANIMAL_MODIFIERS.write() { *w = v; } }
pub fn g_ASKS_TO_CHOOSE() -> Py { ensure_init(); ASKS_TO_CHOOSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_TO_CHOOSE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_TO_CHOOSE.write() { *w = v; } }
pub fn g_A_DECLINE() -> Py { ensure_init(); A_DECLINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DECLINE(v: Py) { ensure_init(); if let Ok(mut w) = A_DECLINE.write() { *w = v; } }
pub fn g_A_THING_PACKED_UP() -> Py { ensure_init(); A_THING_PACKED_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_PACKED_UP(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_PACKED_UP.write() { *w = v; } }
pub fn g_BODY() -> Py { ensure_init(); BODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY(v: Py) { ensure_init(); if let Ok(mut w) = BODY.write() { *w = v; } }
pub fn g_BODY_PART() -> Py { ensure_init(); BODY_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_PART(v: Py) { ensure_init(); if let Ok(mut w) = BODY_PART.write() { *w = v; } }
pub fn g_BONE() -> Py { ensure_init(); BONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BONE(v: Py) { ensure_init(); if let Ok(mut w) = BONE.write() { *w = v; } }
pub fn g_BREAKS() -> Py { ensure_init(); BREAKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREAKS(v: Py) { ensure_init(); if let Ok(mut w) = BREAKS.write() { *w = v; } }
pub fn g_BROKEN() -> Py { ensure_init(); BROKEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BROKEN(v: Py) { ensure_init(); if let Ok(mut w) = BROKEN.write() { *w = v; } }
pub fn g_BROKEN_NEXT() -> Py { ensure_init(); BROKEN_NEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BROKEN_NEXT(v: Py) { ensure_init(); if let Ok(mut w) = BROKEN_NEXT.write() { *w = v; } }
pub fn g_BROKEN_TIME() -> Py { ensure_init(); BROKEN_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BROKEN_TIME(v: Py) { ensure_init(); if let Ok(mut w) = BROKEN_TIME.write() { *w = v; } }
pub fn g_CANT_GET() -> Py { ensure_init(); CANT_GET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CANT_GET(v: Py) { ensure_init(); if let Ok(mut w) = CANT_GET.write() { *w = v; } }
pub fn g_DEAD_AS_VERY_AFTER() -> Py { ensure_init(); DEAD_AS_VERY_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEAD_AS_VERY_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = DEAD_AS_VERY_AFTER.write() { *w = v; } }
pub fn g_DEATHWISH() -> Py { ensure_init(); DEATHWISH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATHWISH(v: Py) { ensure_init(); if let Ok(mut w) = DEATHWISH.write() { *w = v; } }
pub fn g_DECLINE() -> Py { ensure_init(); DECLINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DECLINE(v: Py) { ensure_init(); if let Ok(mut w) = DECLINE.write() { *w = v; } }
pub fn g_DECLINE_TIME() -> Py { ensure_init(); DECLINE_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DECLINE_TIME(v: Py) { ensure_init(); if let Ok(mut w) = DECLINE_TIME.write() { *w = v; } }
pub fn g_DETAIL() -> Py { ensure_init(); DETAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETAIL(v: Py) { ensure_init(); if let Ok(mut w) = DETAIL.write() { *w = v; } }
pub fn g_DURATION() -> Py { ensure_init(); DURATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DURATION(v: Py) { ensure_init(); if let Ok(mut w) = DURATION.write() { *w = v; } }
pub fn g_FAULT_FILLERS() -> Py { ensure_init(); FAULT_FILLERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FAULT_FILLERS(v: Py) { ensure_init(); if let Ok(mut w) = FAULT_FILLERS.write() { *w = v; } }
pub fn g_FIGURATIVE() -> Py { ensure_init(); FIGURATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURATIVE(v: Py) { ensure_init(); if let Ok(mut w) = FIGURATIVE.write() { *w = v; } }
pub fn g_FLAT() -> Py { ensure_init(); FLAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FLAT(v: Py) { ensure_init(); if let Ok(mut w) = FLAT.write() { *w = v; } }
pub fn g_FLAT_SUBJECT() -> Py { ensure_init(); FLAT_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FLAT_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = FLAT_SUBJECT.write() { *w = v; } }
pub fn g_FRACTURE() -> Py { ensure_init(); FRACTURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRACTURE(v: Py) { ensure_init(); if let Ok(mut w) = FRACTURE.write() { *w = v; } }
pub fn g_GROWING() -> Py { ensure_init(); GROWING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GROWING(v: Py) { ensure_init(); if let Ok(mut w) = GROWING.write() { *w = v; } }
pub fn g_IT() -> Py { ensure_init(); IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IT(v: Py) { ensure_init(); if let Ok(mut w) = IT.write() { *w = v; } }
pub fn g_JOINING() -> Py { ensure_init(); JOINING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOINING(v: Py) { ensure_init(); if let Ok(mut w) = JOINING.write() { *w = v; } }
pub fn g_KEEPS_AT_IT() -> Py { ensure_init(); KEEPS_AT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_AT_IT(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_AT_IT.write() { *w = v; } }
pub fn g_KEEPS_FINE() -> Py { ensure_init(); KEEPS_FINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_FINE(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_FINE.write() { *w = v; } }
pub fn g_KEEPS_NOT_A_THING() -> Py { ensure_init(); KEEPS_NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_NOT_A_THING.write() { *w = v; } }
pub fn g_KEEPS_NOT_THEIR_THING() -> Py { ensure_init(); KEEPS_NOT_THEIR_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_NOT_THEIR_THING(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_NOT_THEIR_THING.write() { *w = v; } }
pub fn g_KEEPS_OF_A_HOUSE() -> Py { ensure_init(); KEEPS_OF_A_HOUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_OF_A_HOUSE(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_OF_A_HOUSE.write() { *w = v; } }
pub fn g_KEEPS_POSSESSIVE() -> Py { ensure_init(); KEEPS_POSSESSIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_POSSESSIVE(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_POSSESSIVE.write() { *w = v; } }
pub fn g_KEEPS_THEIRS() -> Py { ensure_init(); KEEPS_THEIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_THEIRS(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_THEIRS.write() { *w = v; } }
pub fn g_MINE() -> Py { ensure_init(); MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MINE(v: Py) { ensure_init(); if let Ok(mut w) = MINE.write() { *w = v; } }
pub fn g_MINE_AGAIN() -> Py { ensure_init(); MINE_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MINE_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = MINE_AGAIN.write() { *w = v; } }
pub fn g_NOT_A_MODIFIER() -> Py { ensure_init(); NOT_A_MODIFIER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MODIFIER(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_MODIFIER.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_NOT_OWNED() -> Py { ensure_init(); NOT_OWNED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_OWNED(v: Py) { ensure_init(); if let Ok(mut w) = NOT_OWNED.write() { *w = v; } }
pub fn g_NOT_THE_THING() -> Py { ensure_init(); NOT_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_THING.write() { *w = v; } }
pub fn g_NUISANCE() -> Py { ensure_init(); NUISANCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUISANCE(v: Py) { ensure_init(); if let Ok(mut w) = NUISANCE.write() { *w = v; } }
pub fn g_ODD_MARK() -> Py { ensure_init(); ODD_MARK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ODD_MARK(v: Py) { ensure_init(); if let Ok(mut w) = ODD_MARK.write() { *w = v; } }
pub fn g_OTHERS() -> Py { ensure_init(); OTHERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHERS(v: Py) { ensure_init(); if let Ok(mut w) = OTHERS.write() { *w = v; } }
pub fn g_OWNER_END() -> Py { ensure_init(); OWNER_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWNER_END(v: Py) { ensure_init(); if let Ok(mut w) = OWNER_END.write() { *w = v; } }
pub fn g_OWNS() -> Py { ensure_init(); OWNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWNS(v: Py) { ensure_init(); if let Ok(mut w) = OWNS.write() { *w = v; } }
pub fn g_PLEA() -> Py { ensure_init(); PLEA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLEA(v: Py) { ensure_init(); if let Ok(mut w) = PLEA.write() { *w = v; } }
pub fn g_SORE() -> Py { ensure_init(); SORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SORE(v: Py) { ensure_init(); if let Ok(mut w) = SORE.write() { *w = v; } }
pub fn g_SORE_NEXT() -> Py { ensure_init(); SORE_NEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SORE_NEXT(v: Py) { ensure_init(); if let Ok(mut w) = SORE_NEXT.write() { *w = v; } }
pub fn g_SORE_TIME() -> Py { ensure_init(); SORE_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SORE_TIME(v: Py) { ensure_init(); if let Ok(mut w) = SORE_TIME.write() { *w = v; } }
pub fn g_STILL_FAILING() -> Py { ensure_init(); STILL_FAILING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_FAILING(v: Py) { ensure_init(); if let Ok(mut w) = STILL_FAILING.write() { *w = v; } }
pub fn g_STRAIN() -> Py { ensure_init(); STRAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STRAIN(v: Py) { ensure_init(); if let Ok(mut w) = STRAIN.write() { *w = v; } }
pub fn g_STRAIN_NEXT() -> Py { ensure_init(); STRAIN_NEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STRAIN_NEXT(v: Py) { ensure_init(); if let Ok(mut w) = STRAIN_NEXT.write() { *w = v; } }
pub fn g_STRAIN_TIME() -> Py { ensure_init(); STRAIN_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STRAIN_TIME(v: Py) { ensure_init(); if let Ok(mut w) = STRAIN_TIME.write() { *w = v; } }
pub fn g_THE_END_OF_A_THING() -> Py { ensure_init(); THE_END_OF_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_END_OF_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = THE_END_OF_A_THING.write() { *w = v; } }
pub fn g_THING() -> Py { ensure_init(); THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING(v: Py) { ensure_init(); if let Ok(mut w) = THING.write() { *w = v; } }
pub fn g_THING_STATE() -> Py { ensure_init(); THING_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_STATE(v: Py) { ensure_init(); if let Ok(mut w) = THING_STATE.write() { *w = v; } }
pub fn g_THING_STATE_ONLY() -> Py { ensure_init(); THING_STATE_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_STATE_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = THING_STATE_ONLY.write() { *w = v; } }
pub fn g_TRIGGER() -> Py { ensure_init(); TRIGGER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIGGER(v: Py) { ensure_init(); if let Ok(mut w) = TRIGGER.write() { *w = v; } }
pub fn g_WANTING_OR_PRAISE() -> Py { ensure_init(); WANTING_OR_PRAISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANTING_OR_PRAISE(v: Py) { ensure_init(); if let Ok(mut w) = WANTING_OR_PRAISE.write() { *w = v; } }
pub fn g_WHAT_IT_DOES() -> Py { ensure_init(); WHAT_IT_DOES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_IT_DOES(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_IT_DOES.write() { *w = v; } }
pub fn g__A_HEDGE() -> Py { ensure_init(); _A_HEDGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_HEDGE(v: Py) { ensure_init(); if let Ok(mut w) = _A_HEDGE.write() { *w = v; } }
pub fn g__A_LENGTH_OF_TIME() -> Py { ensure_init(); _A_LENGTH_OF_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_LENGTH_OF_TIME(v: Py) { ensure_init(); if let Ok(mut w) = _A_LENGTH_OF_TIME.write() { *w = v; } }
pub fn g__A_SENTENCE() -> Py { ensure_init(); _A_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = _A_SENTENCE.write() { *w = v; } }
pub fn g__NOT_A_STRAIN_DETAIL() -> Py { ensure_init(); _NOT_A_STRAIN_DETAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_STRAIN_DETAIL(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_STRAIN_DETAIL.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Something has gone wrong and they are telling you about it.\n\nA person who says \"my washing machine is leaking\" or \"my knee has been aching for a week\" is not\nmaking conversation: they are handing you a problem, and the friendliest thing you can do is ask\nthe one question that gets it moving. On main every one of those lines got a line that fits\nanything - \"Noted! What would be most useful to talk about?\", \"Got it. Is there anything you'd\nlike a hand with?\" - which reads as if nobody was listening.\n\nSo this answers the trouble itself, from a short pool: what is it doing, when did it start, what\nhave you tried. Two shapes, because a broken thing and an aching body want different questions:\n\n* a thing that is not behaving - \"my laptop wont turn on\", \"my boiler keeps cutting out\"\n* their own body - \"my back pain keeps coming back\", \"I've had a cough for three weeks now\"\n* a setback they have not explained - \"my cake was a disaster\", \"everything went wrong today\"\n  (the reading and the wording are bot/memory/setback.py: the question is what happened)\n\nEvery rule here is a guard against taking a turn that is not that:\n\n* never a question (bot/understanding's kind is checked by the caller, and a question mark in the\n  line is enough to stand aside): a line that asks something has its own reply;\n* never a line asking for tips or advice - \"my kid wont eet vegtables, any tips\" is a plea, not\n  a report, and \"what's it doing exactly\" would answer the wrong thing;\n* **never a line about another person.** \"my husband wont wake up\", \"my mum is having a sezure\",\n  \"my baby has a temperature of 39 and is floppy\" all name a body or a thing failing, and all of\n  them are emergencies about someone else. Asking \"when did it start?\" there is far worse than a\n  stock line, and the emergency read belongs to bot/modules/safety.py, not here. One word from\n  OTHERS and this passes, whatever else the line says;\n* a body reply needs the person themselves to be the one hurting (\"i\", \"my <part>\"), never a\n  temperature or a symptom handed to a child;\n* a thing reply needs something named to belong to them (\"my\", \"our\", \"the\", \"it\"), so \"dead\n  easy, mate\" and \"this heat is killing me\" are small talk, not faults.\n");
    s_OTHERS(call_builtin("frozenset", &[py_tuple(vec![py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("his"), py_str("hers"), py_str("they"), py_str("them"), py_str("their"), py_str("theirs"), py_str("mum"), py_str("mam"), py_str("mom"), py_str("mummy"), py_str("mummy"), py_str("dad"), py_str("daddy"), py_str("father"), py_str("mother"), py_str("parent"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("toddler"), py_str("teenager"), py_str("boy"), py_str("girl"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("grandparents"), py_str("brother"), py_str("sister"), py_str("sibling"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("nephew"), py_str("niece"), py_str("friend"), py_str("mate"), py_str("flatmate"), py_str("housemate"), py_str("neighbour"), py_str("neighbor"), py_str("colleague"), py_str("boss"), py_str("manager"), py_str("landlord"), py_str("landlady"), py_str("tenant"), py_str("carer"), py_str("nurse"), py_str("vet"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("pet"), py_str("pup")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_END_OF_A_THING(py_str("\n      | (?:its|it's|mine|ours)\\s+had\\s+(?:it|its\\s+day)\n      | (?:mine|ours)\\s+(?:has|have|had)\\s+had\\s+(?:it|its\\s+day)\n      | (?:my|our|the|this|that|these|those)\\s+(?:[\\w']+\\s+){0,3}\n          (?:has|have|had)\\s+had\\s+(?:it|its\\s+day)\n      | (?:my|our|the|this|that|these|those)\\s+(?:[\\w']+\\s+){0,3}\n          (?:is|are|was|were)\\s+done\\s+for\n"));
    s_A_THING_PACKED_UP(py_str("\n      | (?:mine|ours)\\s+(?:has|have|had)?\\s*(?:packed\\s+(?:up|in)|(?:gave|given)\\s+up)\n      | (?:the|my|our|this|that|his|her|their|it|its)\\s+(?:[\\w']+\\s+){0,3}\n          (?:has|have|had)?\\s*(?:packed\\s+(?:up|in)|(?:gave|given)\\s+up)\n"));
    s_THING_STATE(py_str("\n    (?:has|have|had|is|are|was|were)\\s+\n      (?:(?:gone|got|become|turned)\\s+)?\n      (?:all\\s+|a\\s+bit\\s+|a\\s+lot\\s+|really\\s+|very\\s+|quite\\s+|so\\s+|completely\\s+\n         |totally\\s+|properly\\s+|now\\s+)?\n      (?:a\\s+|an\\s+)?\n      (?:\n          black|brown|green|rusty|mouldy|moldy|mildewed|lumpy|soggy|slimy|sticky|greasy\n        | furry|furred|scaled|pitted|warped|brittle|cracked|frayed|threadbare|bubbled\n        | blistered|peeling|discoloured|discolored|crusty\n        | burnt|burned|scorched|seized|rusted|corroded|perished|split|snapped|melted|bent\n        | stuck|jammed|solid|knackered|kaput|buggered|stiff|dropped|flat\n        | yellow|patchy|faded|bald|rip(?![\\s-]+off\\b)\n        | crack|hole|tear|dent|patch\n      )\n      (?:\\s+(?:on|up|through|off|shut|together|solid|apart|open|down|in|out))?\n  | burnt\\s+on\n  | blown\\s+(?:down|off|out|over|away)\n  | (?:hanging|fell|fallen|come|coming|pulled)\\s+(?:off|away|loose|apart|down|open)\n  | conked\\s+(?:out|off)\n"));
    s_GROWING(call_builtin("frozenset", &[mcall(&py_str("\n    leaves leaf grass lawn lawns plants plant tree trees hedge hedges shrub shrubs bush bushes\n    fruit veg vegetables tomatoes tomato potatoes crops flowers flower garden allotment ivy\n    moss weeds weed field fields meadow hedgerow\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_STATE_ONLY(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:"), &g_THING_STATE()), &py_str(")\\b")), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THING(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&py_str("\n    \\b(?:\n        wont | won't | doesn'?t\\s+work | isnt\\s+working | isn'?t\\s+working\n      | not\\s+(?:working|responding|turning\\s+on|charging|draining|connecting|booting)\n      | stopped\\s+working | stopped\\s+charging\n      | broke | broken | broke\\s+down | bust | faulty | on\\s+the\\s+blink | playing\\s+up\n      | leak(?:s|ing|ed)? | drain(?:s|ing)? | soaking | dripping | blocked | clogged | jammed | stuck\n      | keeps?\\s+(?:freezing|crashing|cutting|dropping|stopping|going\\s+off|calling|\n                    restarting|turning\\s+off|coming\\s+up|showing|flashing)\n      | cutting\\s+out | crashing | froze | frozen | freezes | collapses? | collapsed\n      | makes?\\s+a\\s+noise | making\\s+a\\s+noise | noisy | rattl(?:es|ing|ed)\n      | die(?:s|d)? | dead | flat\\s+(?:tyre|battery) | slow | sluggish | overheats? | overheating\n      | error | glitch(?:y|es)? | corrupt(?:ed)? | wont\\s+connect | no\\s+signal\n      | needs?\\s+(?:a\\s+)?(?:fixing|servicing)\n      | on\\s+its\\s+way\\s+out\n      | on\\s+(?:its|his|her|their)\\s+last\\s+legs\n      | clapped\\s+out | seen\\s+better\\s+days | knackered\n      | went\\s+bang | blew\\s+(?:a\\s+)?fuse\n"), &g_THE_END_OF_A_THING()), &g_A_THING_PACKED_UP()), &py_str("\n      | ")), &g_THING_STATE()), &py_str("\n    )\\b\n")), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CANT_GET(stdlib_call("re", "compile", &[py_str("\n    \\b(?:can'?t|cannot|can\\s+not|couldn'?t|could\\s+not|unable\\s+to|not\\s+able\\s+to)\n      (?:\\s+seem\\s+to)?\n      \\s+(?:get|getting|make|making|keep|keeping|stop|stopping)\\b\n      (?:\\s+[\\w']+){1,4}?      # the thing: \"my washing machine\", \"the app\", \"this printer\"\n      \\s+(?:to\\s+(?:work|job|run|go|start|stop|spin|turn|open|close|load|boot|charge\n                      |connect|pair|sync|print|scan|save|drain|heat|light|fire|play|update\n                      |install|download|upload|respond|unlock|switch|move|boil|cook|rise\n                      |freeze|cool|record|log\\s?in)\n          |working|going|started|running|charged|connected|opening|booting)\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY(stdlib_call("re", "compile", &[py_str("\n    \\b(?:\n        hurt(?:s|ing)? | ach(?:e|es|ing|y) | sore(?:ness)? | pain(?:s|ful)? | stiff(?:ness)?\n      | swell(?:s|ing|ings|en)? | tender(?:ness)? | numb | itch(?:y|es|ing)? | throbbing\n      | migraines? | headaches? | toothaches? | cough(?:ing)? | rashes?\n      | cramp(?:s|ing)? | bleeding | bled | sprain(?:s|ed)? | bruis(?:e|es|ed)\n      | stubbed | burnt | burned | infection\n    )\\b\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURATIVE(stdlib_call("re", "compile", &[py_str("\\b(?:pain(?:s|ful)?|sore(?:ness)?|stiff(?:ness)?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NUISANCE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+\\w+(?:\\s+\\w+){0,2}\\s+(?:is|are|was|were|'s)\\s+(?:(?:a|real|right|such|quite|so|very|bit\\s+of|proper|little)\\s+){0,2}(?:pain|pains|painful|sore|stiff)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_PART(call_builtin("frozenset", &[py_tuple(vec![py_str("back"), py_str("knee"), py_str("knees"), py_str("shoulder"), py_str("shoulders"), py_str("ankle"), py_str("ankles"), py_str("wrist"), py_str("wrists"), py_str("neck"), py_str("hip"), py_str("hips"), py_str("leg"), py_str("legs"), py_str("arm"), py_str("arms"), py_str("foot"), py_str("feet"), py_str("toe"), py_str("toes"), py_str("tooth"), py_str("teeth"), py_str("gum"), py_str("gums"), py_str("ear"), py_str("ears"), py_str("eye"), py_str("eyes"), py_str("chest"), py_str("stomach"), py_str("belly"), py_str("throat"), py_str("head"), py_str("skin"), py_str("hand"), py_str("hands"), py_str("finger"), py_str("fingers"), py_str("jaw"), py_str("sinus"), py_str("sinuses"), py_str("shoulder blade"), py_str("heart"), py_str("elbow"), py_str("elbows"), py_str("forearm"), py_str("forearms"), py_str("thigh"), py_str("thighs"), py_str("calf"), py_str("calves"), py_str("shin"), py_str("shins"), py_str("heel"), py_str("heels"), py_str("rib"), py_str("ribs"), py_str("collarbone"), py_str("collarbones"), py_str("hamstring"), py_str("hamstrings"), py_str("groin"), py_str("cheek"), py_str("cheeks"), py_str("chin"), py_str("eyebrow"), py_str("eyebrows"), py_str("knuckle"), py_str("knuckles"), py_str("tailbone"), py_str("spine"), py_str("tonsils")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ACUTE(stdlib_call("re", "compile", &[py_str("\n    \\b(?:\n        burn(?:s|ed|t)? | scald(?:s|ed)? | swell(?:s|ing|ed)? | swellings?\n      | throat | lips? | tongue | breath(?:e|es|ing|less)? | choking | choked\n      | bleed(?:s|ing)? | bled | numb | tingling | vision | sight | blurred\n      | fell | fallen | fall(?:s|ing)? | collapsed | collapse | unconscious | dizzy | fainted\n      | cut(?:s)? | gash | wound | crushed | twisted | sting(?:s|ing)?\n      | rash(?:es|y)? | pois(?:on|oned|oning) | overdose | pill(?:s)? | seizure | sezure\n      | can'?t\\s+see | cant\\s+see | can'?t\\s+move | cant\\s+move | can'?t\\s+breathe\n    )\\b\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRACTURE(stdlib_call("re", "compile", &[py_str("\\b(?:broken|fractures?|fractured)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BREAKS(stdlib_call("re", "compile", &[py_str("\\b(?:broke|broken|broke\\s+down|breaks|breaking|bust|busted)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BONE(call_builtin("frozenset", &[py_tuple(vec![py_str("rib"), py_str("ribs"), py_str("collarbone"), py_str("collar"), py_str("pelvis"), py_str("spine"), py_str("shin"), py_str("thigh"), py_str("heel"), py_str("elbow"), py_str("elbows"), py_str("bone"), py_str("bones"), py_str("shoulder"), py_str("jaw"), py_str("skull"), py_str("tailbone"), py_str("wrist"), py_str("ankle"), py_str("hip"), py_str("kneecap"), py_str("knuckle"), py_str("knuckles"), py_str("nose"), py_str("cheekbone"), py_str("face"), py_str("cheek"), py_str("chin"), py_str("tooth"), py_str("teeth"), py_str("eyebrow"), py_str("brow"), py_str("temple"), py_str("mouth"), py_str("lip")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("heart"), py_str("marriage"), py_str("relationship"), py_str("trust"), py_str("body"), py_str("voice"), py_str("sleep"), py_str("life"), py_str("family"), py_str("friendship"), py_str("confidence"), py_str("spirit"), py_str("faith"), py_str("childhood"), py_str("career"), py_str("mind"), py_str("soul"), py_str("home"), py_str("week"), py_str("year"), py_str("day"), py_str("job"), py_str("work"), py_str("business"), py_str("mood")])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_AT_IT(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|kept)\\s+([a-z]{3,}ing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_FINE(call_builtin("frozenset", &[py_tuple(vec![py_str("working"), py_str("works"), py_str("going"), py_str("goes"), py_str("running"), py_str("runs"), py_str("coming"), py_str("comes"), py_str("growing"), py_str("improving"), py_str("doing"), py_str("does"), py_str("getting"), py_str("gets"), py_str("making"), py_str("makes"), py_str("trying"), py_str("tries"), py_str("taking"), py_str("takes"), py_str("giving"), py_str("gives"), py_str("helping"), py_str("happening"), py_str("arriving"), py_str("waiting"), py_str("sitting"), py_str("standing"), py_str("walking"), py_str("talking"), py_str("thinking"), py_str("looking"), py_str("feeling"), py_str("eating"), py_str("drinking"), py_str("sleeping"), py_str("learning"), py_str("playing"), py_str("wanting"), py_str("needing"), py_str("asking"), py_str("saying"), py_str("telling"), py_str("buying"), py_str("selling"), py_str("sending"), py_str("bringing"), py_str("paying"), py_str("saving"), py_str("spending"), py_str("cooking"), py_str("baking"), py_str("cleaning"), py_str("washing"), py_str("wearing"), py_str("reading"), py_str("writing"), py_str("watching"), py_str("listening"), py_str("smiling"), py_str("laughing"), py_str("crying"), py_str("shouting"), py_str("fighting"), py_str("arguing"), py_str("worrying"), py_str("wondering"), py_str("hoping"), py_str("practising"), py_str("practicing"), py_str("scrolling"), py_str("charging"), py_str("lasts"), py_str("lives"), py_str("matters"), py_str("counts"), py_str("means"), py_str("sounds"), py_str("seems"), py_str("moving"), py_str("noticing"), py_str("visiting"), py_str("changing")])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_NOT_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("people"), py_str("person"), py_str("someone"), py_str("somebody"), py_str("anyone"), py_str("everyone"), py_str("nobody"), py_str("guy"), py_str("man"), py_str("woman"), py_str("bloke"), py_str("fella"), py_str("lad"), py_str("lass"), py_str("boy"), py_str("girl"), py_str("teen"), py_str("teenager"), py_str("kid"), py_str("ex"), py_str("exes"), py_str("coworker"), py_str("co-worker"), py_str("workmate"), py_str("colleague"), py_str("colleagues"), py_str("staff"), py_str("team"), py_str("crew"), py_str("teacher"), py_str("nurse"), py_str("doctor"), py_str("carer"), py_str("client"), py_str("customer"), py_str("supplier"), py_str("boss"), py_str("manager"), py_str("old"), py_str("year"), py_str("years"), py_str("puppy"), py_str("kitten"), py_str("rabbit"), py_str("hamster"), py_str("budgie"), py_str("parrot"), py_str("horse"), py_str("pony"), py_str("spaniel"), py_str("retriever"), py_str("poodle"), py_str("beagle"), py_str("collie"), py_str("labrador"), py_str("terrier"), py_str("chihuahua"), py_str("guinea"), py_str("ferret"), py_str("tortoise"), py_str("goldfish"), py_str("bird"), py_str("birds"), py_str("squirrel"), py_str("squirrels"), py_str("fox"), py_str("foxes"), py_str("mouse"), py_str("mice"), py_str("rat"), py_str("rats"), py_str("dentist"), py_str("optician"), py_str("vet"), py_str("vet's"), py_str("landlord"), py_str("plumber"), py_str("mechanic"), py_str("electrician"), py_str("builder"), py_str("cleaner"), py_str("tutor"), py_str("hairdresser"), py_str("barber"), py_str("receptionist"), py_str("postie")])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_NOT_THEIR_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("shop"), py_str("shops"), py_str("store"), py_str("stores"), py_str("supermarket"), py_str("pub"), py_str("pubs"), py_str("bar"), py_str("cafe"), py_str("café"), py_str("restaurant"), py_str("takeaway"), py_str("chippy"), py_str("bank"), py_str("pool"), py_str("gym"), py_str("library"), py_str("hotel"), py_str("school"), py_str("nursery"), py_str("council"), py_str("hospital"), py_str("surgery"), py_str("clinic"), py_str("pharmacy"), py_str("chemist"), py_str("garage"), py_str("station"), py_str("bus"), py_str("buses"), py_str("train"), py_str("trains"), py_str("tram"), py_str("tube"), py_str("taxi"), py_str("delivery"), py_str("post"), py_str("airport"), py_str("airline"), py_str("office"), py_str("market"), py_str("mall"), py_str("centre")])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_THEIRS(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our"), py_str("the"), py_str("this")])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_POSSESSIVE(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our")])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_OF_A_HOUSE(call_builtin("frozenset", &[py_tuple(vec![py_str("fridge"), py_str("freezer"), py_str("fridge-freezer"), py_str("boiler"), py_str("kettle"), py_str("oven"), py_str("hob"), py_str("cooker"), py_str("microwave"), py_str("toaster"), py_str("dishwasher"), py_str("washer"), py_str("dryer"), py_str("tumble"), py_str("machine"), py_str("appliance"), py_str("fan"), py_str("pump"), py_str("radiator"), py_str("thermostat"), py_str("heater"), py_str("heating"), py_str("shower"), py_str("tap"), py_str("taps"), py_str("toilet"), py_str("sink"), py_str("drain"), py_str("drains"), py_str("pipe"), py_str("pipes"), py_str("plug"), py_str("socket"), py_str("switch"), py_str("fuse"), py_str("bulb"), py_str("light"), py_str("lights"), py_str("lamp"), py_str("alarm"), py_str("smoke"), py_str("detector"), py_str("door"), py_str("doors"), py_str("window"), py_str("windows"), py_str("lock"), py_str("locks"), py_str("handle"), py_str("hinge"), py_str("hinges"), py_str("flap"), py_str("damp"), py_str("roof"), py_str("gutter"), py_str("guttering"), py_str("fence"), py_str("gate"), py_str("shed"), py_str("decking"), py_str("drive"), py_str("garage"), py_str("hoover"), py_str("vacuum"), py_str("mower"), py_str("lawnmower"), py_str("strimmer"), py_str("wifi"), py_str("router"), py_str("broadband"), py_str("internet"), py_str("connection"), py_str("signal"), py_str("tv"), py_str("television"), py_str("remote"), py_str("phone"), py_str("mobile"), py_str("landline"), py_str("laptop"), py_str("computer"), py_str("pc"), py_str("tablet"), py_str("ipad"), py_str("printer"), py_str("keyboard"), py_str("mouse"), py_str("screen"), py_str("monitor"), py_str("charger"), py_str("battery"), py_str("car"), py_str("van"), py_str("bike"), py_str("bicycle"), py_str("scooter"), py_str("engine"), py_str("brakes"), py_str("brake"), py_str("clutch"), py_str("gears"), py_str("tyre"), py_str("tyres"), py_str("wheel"), py_str("wheels"), py_str("headlight"), py_str("indicator"), py_str("wiper"), py_str("wipers"), py_str("exhaust"), py_str("radiator"), py_str("mattress"), py_str("bed"), py_str("shower"), py_str("cooker"), py_str("hob"), py_str("freezer"), py_str("washer-dryer"), py_str("blind"), py_str("blinds"), py_str("curtain"), py_str("curtains"), py_str("shutter"), py_str("shutters"), py_str("clothes"), py_str("dryer"), py_str("doorbell"), py_str("bell"), py_str("buzzer"), py_str("intercom"), py_str("extractor"), py_str("immersion"), py_str("cistern"), py_str("valve"), py_str("tank"), py_str("timer"), py_str("thermostat"), py_str("sensor"), py_str("meter"), py_str("socket"), py_str("adaptor"), py_str("lead"), py_str("cable"), py_str("speaker"), py_str("radio"), py_str("console"), py_str("wifi"), py_str("hub"), py_str("box"), py_str("unit"), py_str("system"), py_str("board"), py_str("panel")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("we"), py_str("you"), py_str("he"), py_str("she"), py_str("they"), py_str("it"), py_str("there"), py_str("what"), py_str("which"), py_str("who"), py_str("and"), py_str("but"), py_str("so"), py_str("if"), py_str("when"), py_str("that"), py_str("this"), py_str("then"), py_str("also"), py_str("always"), py_str("do"), py_str("does"), py_str("did"), py_str("should"), py_str("would"), py_str("could"), py_str("can"), py_str("cant"), py_str("cannot"), py_str("will"), py_str("wont"), py_str("might"), py_str("may"), py_str("must"), py_str("to"), py_str("not"), py_str("never"), py_str("up"), py_str("one"), py_str("any"), py_str("much"), py_str("no"), py_str("how"), py_str("why"), py_str("still"), py_str("just"), py_str("only"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("all")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_TO_CHOOSE(stdlib_call("re", "compile", &[py_str("\\b(?:should\\s+i|shall\\s+i|is\\s+it\\s+worth|is\\s+it\\s+better|\n    would\\s+it\\s+be\\s+better|which\\s+(?:one\\s+)?should\\s+i|do\\s+you\\s+think\\s+i\\s+should|\n    what\\s+would\\s+you\\s+do|worth\\s+(?:repairing|fixing|it)|new\\s+one|replace\\s+it|\n    get\\s+rid\\s+of\\s+it|throw\\s+it\\s+(?:out|away))\\b"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FLAT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|has|have|had|went|goes|just|all|now|so|really|completely|totally|kind\\s+of|sort\\s+of)?\\s*(?:gone|went|died|dead|faded|fading|vanished|disappeared|dried\\s+up|evaporated|left\\s+me)\\b(?!\\s+(?:up|down)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_FLAT_SUBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("spark"), py_str("hope"), py_str("hopes"), py_str("motivation"), py_str("patience"), py_str("appetite"), py_str("confidence"), py_str("humour"), py_str("humor"), py_str("laughter"), py_str("laugh"), py_str("libido"), py_str("mojo"), py_str("passion"), py_str("interest"), py_str("enthusiasm"), py_str("inspiration"), py_str("creativity"), py_str("energy"), py_str("buzz"), py_str("joy"), py_str("romance"), py_str("intimacy"), py_str("trust"), py_str("friendship"), py_str("marriage"), py_str("bubbliness"), py_str("zest")])]).unwrap_or_else(|e| exc::raise(e)));
    s_JOINING(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been"), py_str("being"), py_str("has"), py_str("have"), py_str("had"), py_str("keeps"), py_str("keep"), py_str("kept"), py_str("just"), py_str("now"), py_str("still"), py_str("again"), py_str("always"), py_str("suddenly"), py_str("totally"), py_str("completely"), py_str("really"), py_str("all"), py_str("so"), py_str("very"), py_str("a"), py_str("an"), py_str("the"), py_str("and"), py_str("but"), py_str("then"), py_str("on"), py_str("off"), py_str("up"), py_str("it"), py_str("its"), py_str("it's"), py_str("there"), py_str("here"), py_str("one"), py_str("two"), py_str("twice"), py_str("kind"), py_str("sort"), py_str("bit"), py_str("little"), py_str("lot"), py_str("feel"), py_str("feels"), py_str("felt"), py_str("seem"), py_str("seems"), py_str("looks"), py_str("sounds"), py_str("gets"), py_str("got")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_OWNED(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("his"), py_str("her"), py_str("its"), py_str("their"), py_str("your"), py_str("in"), py_str("on"), py_str("at"), py_str("for"), py_str("to"), py_str("with"), py_str("from"), py_str("of"), py_str("and"), py_str("but"), py_str("or"), py_str("then"), py_str("when"), py_str("if"), py_str("as"), py_str("than"), py_str("like")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATHWISH(stdlib_call("re", "compile", &[py_str("\\b(?:want|wanted|wanting|going|gonna|wish|wished|rather)\\s+(?:to\\s+)?(?:die|dead)\\b|\\b(?:die|dye)\\s+my\\s+hair\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ODD_MARK(add(&stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e)), &py_str("’")));
    s_DEAD_AS_VERY_AFTER(call_builtin("frozenset", &[py_tuple(vec![py_str("quiet"), py_str("tired"), py_str("easy"), py_str("good"), py_str("bad"), py_str("cheap"), py_str("slow"), py_str("boring"), py_str("cheerful"), py_str("glad"), py_str("lucky"), py_str("proud"), py_str("keen"), py_str("jealous"), py_str("happy"), py_str("sure"), py_str("tricky"), py_str("hard"), py_str("funny"), py_str("rude"), py_str("straight"), py_str("wrong"), py_str("serious"), py_str("certain"), py_str("ahead"), py_str("silent"), py_str("knackered"), py_str("shattered"), py_str("comfy"), py_str("comfortable"), py_str("soft"), py_str("cosy"), py_str("cozy"), py_str("warm"), py_str("cold"), py_str("nice"), py_str("sweet"), py_str("pretty"), py_str("smooth"), py_str("strong"), py_str("fussy"), py_str("broke"), py_str("skint"), py_str("fun"), py_str("obvious"), py_str("scary"), py_str("gentle"), py_str("relaxed"), py_str("bright"), py_str("dark"), py_str("loud"), py_str("chuffed"), py_str("mint"), py_str("annoying"), py_str("embarrassing"), py_str("quick"), py_str("fast"), py_str("clean"), py_str("simple"), py_str("smart"), py_str("friendly"), py_str("heavy")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLEA(stdlib_call("re", "compile", &[py_str("\\b(?:tips?|advice|suggestions?|recommendations?|what\\s+should\\s+i|how\\s+(?:do|can|should)\\s+i|any\\s+ideas?|help\\s+me)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MINE(stdlib_call("re", "compile", &[py_str("^(?:i\\b|i'm\\b|im\\b|i've\\b|ive\\b|my\\b|our\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWNER_END(py_tuple(vec![py_str("'s"), py_str("s'"), py_str("’s"), py_str("s")]));
    s_OWNS(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|mine|ours|the|this|that|it|its|it's)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FAULT_FILLERS(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("am"), py_str("be"), py_str("been"), py_str("being"), py_str("still"), py_str("just"), py_str("also"), py_str("again"), py_str("always"), py_str("never"), py_str("keeps"), py_str("keep"), py_str("kept"), py_str("now"), py_str("then"), py_str("so"), py_str("too"), py_str("very"), py_str("really"), py_str("totally"), py_str("completely"), py_str("half"), py_str("a"), py_str("an"), py_str("the"), py_str("of"), py_str("and"), py_str("but"), py_str("its"), py_str("it"), py_str("has"), py_str("have"), py_str("had"), py_str("hasnt"), py_str("hadnt"), py_str("isnt"), py_str("not"), py_str("no"), py_str("getting"), py_str("gone"), py_str("went"), py_str("bit"), py_str("little"), py_str("properly"), py_str("partly"), py_str("mostly"), py_str("sometimes"), py_str("today"), py_str("yesterday"), py_str("again")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DURATION(stdlib_call("re", "compile", &[py_str("\n    \\b(?:\n        for\\s+(?:about\\s+|nearly\\s+|over\\s+|almost\\s+)?(?:\\w+\\s+){0,2}\n            (?:days?|nights?|weeks?|months?|years?)\n      | since\\s+(?:\\w+\\s+){0,3}(?:days?|weeks?|months?|years?|monday|tuesday|wednesday|\n            thursday|friday|saturday|sunday|christmas|january|february|march|april|may|june|\n            july|august|september|october|november|december)\n      | going\\s+on\\s+(?:\\w+\\s+){0,2}(?:days?|weeks?|months?|years?)\n      | (?:started|began)\\s+(?:\\w+\\s+){0,2}(?:yesterday|today|last\\s+week|last\\s+night|\n            this\\s+morning|on\\s+monday|a\\s+(?:week|month|few\\s+days)\\s+ago|\n            (?:\\d+|two|three|four|five|six|seven|eight|nine|ten)\\s+days\\s+ago)\n      | \\b(?:two|three|four|five|six|seven|eight|nine|ten|twenty|thirty|a\\s+few|several|many)\n            \\s+(?:days?|weeks?|months?|years?)\n      | \\b\\d+\\s*(?:days?|weeks?|months?|years?)\n    )\\b\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BROKEN(py_tuple(vec![py_str("That's a pain. What's it doing exactly, and when did it start?"), py_str("Annoying. Is it every time, or only now and then?"), py_str("Right. What have you tried so far?"), py_str("That would bother me too. Tell me what it does and I'll work out the next step with you."), py_str("Ugh, those are fiddly. How long has it been like that?")]));
    s_BROKEN_TIME(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BROKEN()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("How long")))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DECLINE(stdlib_call("re", "compile", &[add(&add(&add(&py_str("\\b(?:\n    on\\s+its\\s+way\\s+out\n  | on\\s+(?:its|his|her|their)\\s+last\\s+legs\n  | clapped\\s+out | seen\\s+better\\s+days\n"), &g_THE_END_OF_A_THING()), &g_A_THING_PACKED_UP()), &py_str("\n)\\b")), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DECLINE(py_tuple(vec![py_str("That's a pain. What's it doing exactly, and when did it start?"), py_str("Annoying. What's it doing when it goes wrong?"), py_str("That would bother me too. Tell me what it does and I'll work out the next step with you."), py_str("Right. What is it doing that makes you think it is on the way out?")]));
    s_STILL_FAILING(stdlib_call("re", "compile", &[py_str("\\b(?:on\\s+its\\s+way\\s+out|on\\s+(?:its|his|her|their)\\s+last\\s+legs|clapped\\s+out|seen\\s+better\\s+days)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DECLINE_TIME(py_tuple(vec![py_str("Annoying. What's it doing when it goes wrong?"), py_str("That would bother me too. Tell me what it does and I'll work out the next step with you."), py_str("Right. What is it doing that makes you think it is on the way out?")]));
    s_SORE(py_tuple(vec![py_str("That sounds wearing. Is it getting better, worse, or about the same?"), py_str("Does it come and go, or is it there all the time?"), py_str("Right. What have you tried for it so far?"), py_str("That's a nuisance to be carrying round. How long has it been like that?")]));
    s_SORE_TIME(add(&py_tuple(vec![py_str("That's a nuisance to be carrying round. What's it like at its worst?")]), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SORE()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("How long")))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    s_STRAIN(py_tuple(vec![py_str("That sounds wearing. What's gone wrong, do you think?"), py_str("Right, that is a lot to carry. What is it like at the moment?"), py_str("That would weigh on me too. What has changed?"), py_str("Ugh, that is a hard one. How long has it been like that?"), py_str("Right. What would make the most difference, if it changed?")]));
    s_STRAIN_TIME(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STRAIN()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("How long")))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_SORE_NEXT(py_tuple(vec![py_str("And what makes it worse - moving about, sitting still, or nothing in particular?"), py_str("Is it the same all day, or worse at certain times?"), py_str("Has anything helped - a warm bath, a painkiller, or just resting it?")]));
    s_BROKEN_NEXT(py_tuple(vec![py_str("Does it do it every time, or only now and then?"), py_str("Has anything you tried made a difference?"), py_str("What's it doing when it goes wrong?")]));
    s_STRAIN_NEXT(py_tuple(vec![py_str("Is it one part of it, or the whole thing?"), py_str("Has anything helped, even a little?"), py_str("What would better look like, if it came?")]));
    s_DETAIL(stdlib_call("re", "compile", &[py_str("\\b(?:\n    sore | sorer | hurting | hurts? | ache | aches | aching | achy | stiff | stiffness\n  | swollen | swelling | tender | throb\\w* | sharp | burning | numb | tight | bruised\n  | worse | worst | better | eased? | eases | easier | helped | helps | relief\n)\\b"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIGGER(stdlib_call("re", "compile", &[py_str("\\b(?:\n    when\\s+i | whenever\\s+i | if\\s+i | after\\s+i | since\\s+i\n  | only\\s+when | only\\s+if | when\\s+it | whenever\\s+it | if\\s+it\n  | in\\s+the\\s+(?:morning|morning|evening|evenings|night)\n  | first\\s+thing | all\\s+day | overnight\n  | every\\s+time | each\\s+time | all\\s+the\\s+time | now\\s+and\\s+(?:then|again)\n  | sometimes | most\\s+(?:days|mornings|nights) | lately\n  | sitting | standing | walking | running | lifting | bending | lying | sleeping | driving\n)\\b"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_STRAIN_DETAIL(stdlib_call("re", "compile", &[py_str("^(?:so |and |but |well |ok |okay )*(?:how|what|why|when|where|which|who|should|can|could|would|will|is|are|do|does|did|shall)\\b|\\b(?:broke|broken|died|dead|ill|hospital|accident|leaking|not working|stopped working)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SENTENCE(stdlib_call("re", "compile", &[py_str("\\b(?:am|is|are|was|were|be|been|being|has|have|had|do|does|did|can|could|will|would|shall|should|may|might|must|i'm|im|i've|ive|i'd|we're|we've|he's|she's|there's|get|gets|got|go|cant|cannot|can't|dont|don't|didnt|didn't|wont|won't|isnt|isn't|wasnt|wasn't|havent|haven't|hasnt|hasn't|couldnt|couldn't|wouldnt|wouldn't|doesnt|doesn't|arent|aren't|aint|goes|going|went|keep|keeps|kept|want|wants|need|needs|feel|feels|felt|think|thinks|seem|seems|seemed|know|knows|make|makes|made|work|works|worked|start|starts|started|stop|stops|stopped|try|tries|tried|lost|lose|loses|die|died|broke|broken|hurt|hurts)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HEDGE(call_builtin("frozenset", &[py_tuple(vec![py_str("no"), py_str("idea"), py_str("dont"), py_str("don't"), py_str("do"), py_str("not"), py_str("know"), py_str("sure"), py_str("dunno"), py_str("idk"), py_str("nothing"), py_str("whatever"), py_str("meh"), py_str("maybe"), py_str("i"), py_str("nothing's"), py_str("nothings")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_LENGTH_OF_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|a|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a couple of|a few|several)\\s+(?:days?|weeks?|months?|years?)\\b|\\b(?:days|weeks|months|years|ages)\\b(?:\\s+(?:now|too|already))*\\W*$|\\bsince\\s+(?:yesterday|this morning|last night|the weekend|christmas|the summer|last \\w+|monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december)\\b|\\bfor\\s+(?:years|months|weeks|ages|a long time)\\b|\\b(?:ages|a long time|all (?:week|month|year))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY_TRIED(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|ive|i've|weve|we've|hes|he's|shes|she's|they|they've)\\s+(?:already\\s+|just\\s+|even\\s+|also\\s+)?(?:\\w+ly\\s+)?(?:tried|did|had a go|gave it a go|used|\\w+ed)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_IT(stdlib_call("re", "compile", &[py_str("\\b(?:it|its|it's|this|that|these|those)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MINE_AGAIN(stdlib_call("re", "compile", &[py_str("\\bmy\\s+([a-z']+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_IT_DOES(stdlib_call("re", "compile", &[py_str("what\\s+it\\s+does|what's\\s+it\\s+doing|what\\s+is\\s+it\\s+doing"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTING_OR_PRAISE(stdlib_call("re", "compile", &[py_str("\n    \\b(?:\n        to\\s+die\\s+for\n      | worth\\s+dying\\s+for\n      | (?:would|could|might|will)\\s+(?:die|kill)\\s+for\n      | (?:i|you|we|they|he|she)'?d\\s+(?:die|kill)\\s+for\n      | (?:would|could|will)?\\s*die[sd]?\\s+on\\s+(?:that|this|a|the)\\s+hill\n      | dy(?:ing|in)\\s+(?:for|to)\\b\n    )\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANIMAL_MODIFIERS(call_builtin("frozenset", &[py_tuple(vec![py_str("cat"), py_str("cats"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("kitten"), py_str("kittens"), py_str("pup"), py_str("pups"), py_str("pet"), py_str("pets"), py_str("bird"), py_str("birds"), py_str("fish"), py_str("rabbit"), py_str("rabbits"), py_str("hamster"), py_str("hamsters"), py_str("chicken"), py_str("chickens"), py_str("horse"), py_str("horses"), py_str("tortoise"), py_str("parrot"), py_str("budgie"), py_str("ferret"), py_str("snake"), py_str("lizard"), py_str("mouse"), py_str("rat"), py_str("rats"), py_str("guinea"), py_str("pig"), py_str("pigs")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_MODIFIER(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("am"), py_str("be"), py_str("been"), py_str("being"), py_str("has"), py_str("have"), py_str("had"), py_str("will"), py_str("wont"), py_str("cant"), py_str("cannot"), py_str("could"), py_str("would"), py_str("should"), py_str("might"), py_str("may"), py_str("must"), py_str("needs"), py_str("need"), py_str("got"), py_str("get"), py_str("gets"), py_str("getting"), py_str("went"), py_str("goes"), py_str("going"), py_str("keeps"), py_str("keep"), py_str("kept"), py_str("died"), py_str("dies"), py_str("die"), py_str("dead"), py_str("ill"), py_str("sick"), py_str("old"), py_str("young"), py_str("small"), py_str("big"), py_str("little"), py_str("and"), py_str("but"), py_str("so"), py_str("that"), py_str("who"), py_str("which"), py_str("doesnt"), py_str("didnt"), py_str("isnt"), py_str("wasnt"), py_str("to")])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn someone_elses(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut ending: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names someone else's body as the thing that hurts.\n\n    A word that is in OTHERS, or one of them with a possessive on the end (\"mum's\", \"mums\",\n    \"dogs\"). Nothing else is stripped, so \"ribs\" is not \"rib\" and \"legs\" is not \"leg\".");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    word = call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&g_OTHERS(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&g_OWNER_END());
    for __item in __iter2 {
    ending = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &add(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) } }) {
    if py_truthy(&Py::Bool(py_contains(&g_OTHERS(), &slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None)))) {
    return Py::Bool(true);
    }
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_fault_hangs_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut before: Py = Py::None;
    let mut heads: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The word each fault in the line hangs on: \"flap\" in \"the cat flap is broken\".");
    heads = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&add(&add(&call_builtin("list", &[mcall(&g_THING(), "finditer", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[mcall(&g_CANT_GET(), "finditer", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("list", &[mcall(&g_FLAT(), "finditer", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter1 {
    match_ = __item.clone();
    before = stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FAULT_FILLERS(), &word.clone()))) {
    continue;
    }
    let _ = mcall(&heads.clone(), "add", &[word.clone()]);
    break;
    }
    }
    return call_builtin("frozenset", &[heads.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _only_the_decline(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the line says a thing of theirs is failing and nothing more about it.\n\n    \"my phone is on its way out\" is the whole of it, so the question is what it is doing.\n    \"my freezer has packed up and the food has thawed\" names a second thing and its own\n    consequence, and \"the washing machine is old and on its way out\" says why, so both keep the\n    wider pool. The decline phrase is taken out of the line first, and any fault left in what\n    remains - a symptom, an inability, a part of life gone flat - means the line is not only\n    the decline.\n    ");
    match_ = mcall(&g_A_DECLINE(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    rest = add(&add(&slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None));
    return Py::Bool(!py_truthy(&{ let __b = mcall(&g_THING(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CANT_GET(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BODY(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FLAT(), "search", &[rest.clone()]) } } } } } }));
    Py::None
}
pub fn answers_a_strain_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("Whether this line is the answer to the strain question this module has just asked.\n\n    The arbiter asks this before a card that matches a word in the line: \"since the baby\" after\n    \"what's it like at the moment?\" is that thread's next turn, not the new-baby card, and \"we\n    have two kids and no time\" is not an ack that asks how old the children are.\n    ");
    if py_truthy(&Py::Bool(!py_eq(&_shape_on_the_table(&[memory.clone()]), &py_str("strain")))) {
    return Py::Bool(false);
    }
    return _answers_a_strain_question(&[_flat(&[clean.clone()]), memory.clone()]);
    Py::None
}
pub fn _answers_a_strain_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a short line is the answer to \"what's gone wrong, do you think?\".\n\n    A cause is what a friend asked for, and a cause has no vocabulary of its own: \"since the baby\",\n    \"money\", \"work has been mad\", \"we never see each other\". So the line is judged by what it is\n    not - a question, a fault to repair, somebody else's news, or long enough to be a new subject -\n    rather than by words it must have.\n    ");
    words = mcall(&text.clone(), "split", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__NOT_A_STRAIN_DETAIL(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_SENTENCE(), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_HEDGE(), &mcall(&word.clone(), "strip", &[py_str(",.")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _a_card_answers(&[text.clone()]); if py_truthy(&__b) { __b } else { _a_length_of_time(&[text.clone(), memory.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&reads_as_trouble(&[text.clone()])));
    Py::None
}
pub fn _a_length_of_time(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut buckets: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pools: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line says how long it has been going on, and chat answers it as a length.\n\n    The length reply is chat's (`STANDING_REPLIES` / `STANDING_FEELING_REPLIES`), reached through\n    `_standing`, but `_standing` is not the first thing chat tries: a follow-up or a row can answer\n    the same line with an echo (\"Got it. Is there anything you'd like a hand with?\"), and standing\n    down for that would be worse than the thread's next question. So the module is asked for its\n    reply on a copy of the memory - it writes notes while it reads, and a copy keeps those notes\n    out of the real turn - and this stands aside only when the reply it would give is one of the\n    length pools.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_LENGTH_OF_TIME(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let mut _copy: Py = stdlib_module("copy");
    let mut chat: Py = crate::gen::modules::chat::module_obj();
    said = mcall(&chat.clone(), "respond", &[text.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&_copy.clone(), "deepcopy", &[memory.clone()]) } else { py_dict(vec![]) }]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    said = mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    pools = py_list(vec![]);
    let __iter6 = py_iter_or_raise(&py_tuple(vec![py_str("STANDING_REPLIES"), py_str("STANDING_FEELING_REPLIES")]));
    for __item in __iter6 {
    name = __item.clone();
    buckets = call_builtin("getattr", &[chat.clone(), name.clone(), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("isinstance", &[buckets.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter7 = py_iter_or_raise(&mcall(&buckets.clone(), "values", &[]));
    for __item in __iter7 {
    lines = __item.clone();
    let _ = mcall(&pools.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push(call_builtin("str", &[line.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    }
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pools.clone()) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push(Py::Bool(py_contains(&said.clone(), &slice_or_raise(&mcall(&line.clone(), "lower", &[]), &Py::None, &Py::Int(60), &Py::None)))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_card_answers(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut found: Py = Py::None;
    let _ = py_str("Whether the knowledge base has an answer for this line on its own words.");
    let __try1 = exc::catch(|| {
    let mut lookup: Py = crate::gen::knowledge::lookup::module_obj();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try6 = exc::catch(|| {
    found = mcall(&lookup.clone(), "find", &[py_dict(vec![(py_str("clean"), text.clone()), (py_str("words"), mcall(&text.clone(), "split", &[]))])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::Bool(false);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    return Py::Bool(!py_is(&found.clone(), &Py::None));
    Py::None
}
pub fn _flat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _said_already(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut fresh: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The lines the bot has not said yet in this conversation.\n\n    A pool the conversation has used up is still never empty: the last line of a shape is worth\n    saying again before the bot says nothing, because a shape with no reply left is not an\n    answer and the turn falls through to whatever else the composer has - and on a long chat\n    that is the neutral nod, or with nothing at all, \"Hello!\". So the whole pool is said once,\n    and after that the rule is the one the pools already keep: everything but the reply just\n    given (\"a bot that says the same sentence twice in a row reads as if it stopped listening\").\n    ");
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[_flat(&[mcall(&turn.clone(), "get", &[py_str("bot")])])]);
    }
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push(line.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return call_builtin("tuple", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    fresh = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&said.clone(), &_flat(&[line.clone()])))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&fresh.clone()) {
    return fresh.clone();
    }
    last = geti(&said.clone(), &neg(&Py::Int(1)));
    return { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&_flat(&[line.clone()]), &last.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("tuple", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut shape: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shape") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: shape"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The replies for a shape, without the ones that would ask what the line just told us.\n\n    With `memory`, the questions this conversation has already asked are dropped too: a person\n    who answers \"how long has it been like that?\" and then says more must not be asked it again.\n    ");
    text = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("vague")))) {
    return _said_already(&[crate::gen::memory::setback::pool(&[clean.clone()]), memory.clone()]);
    }
    stated = call_builtin("bool", &[mcall(&g_DURATION(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("sore")))) {
    lines = if py_truthy(&stated.clone()) { g_SORE_TIME() } else { g_SORE() };
    } else {
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("strain")))) {
    lines = if py_truthy(&stated.clone()) { g_STRAIN_TIME() } else { g_STRAIN() };
    } else {
    if py_truthy(&_only_the_decline(&[text.clone()])) {
    lines = if py_truthy(&stated.clone()) { g_DECLINE_TIME() } else { g_DECLINE() };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_STILL_FAILING(), "search", &[text.clone()])))) {
    lines = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("on the way out")))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    } else {
    lines = if py_truthy(&stated.clone()) { g_BROKEN_TIME() } else { g_BROKEN() };
    }
    }
    }
    if py_truthy(&mcall(&g_ALREADY_TRIED(), "search", &[text.clone()])) {
    lines = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("tried so far")))) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return _said_already(&[lines.clone(), memory.clone()]);
    Py::None
}
pub fn _shape_on_the_table(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut memory: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("'sore' or 'thing' when our last reply was one of this module's questions, else None.");
    turns = { let __b = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    last = _flat(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")])]);
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("sore"), add(&g_SORE(), &g_SORE_TIME())]), py_tuple(vec![py_str("thing"), add(&g_BROKEN(), &g_BROKEN_TIME())]), py_tuple(vec![py_str("strain"), add(&g_STRAIN(), &g_STRAIN_TIME())]), py_tuple(vec![py_str("sore"), g_SORE_NEXT()]), py_tuple(vec![py_str("thing"), g_BROKEN_NEXT()]), py_tuple(vec![py_str("strain"), g_STRAIN_NEXT()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    shape = __unpk2[0].clone();
    lines = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&lines.clone());
    for __item in __iter3 {
    line = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&last.clone(), &_flat(&[line.clone()])))) {
    return shape.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _points_at_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut noun: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether a detail is about what we were already talking about.\n\n    A detail carries on a thread by pointing back at it: \"its worse in the morning\", \"only when\n    it spins\", \"my back is worse\". A line that names a new thing of its own (\"the weather is\n    better today\", \"my laptop is better now\") only shares a word with the detail vocabulary, so\n    it is left to the rest of the composer.\n    ");
    if py_truthy(&mcall(&g_IT(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    turns = { let __b = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    said = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) { _flat(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")])]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_MINE_AGAIN(), "findall", &[text.clone()]));
    for __item in __iter1 {
    noun = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[noun.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("\\b"); py_string(__s) }, said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _shape_answered(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut memory: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The shape of a fault we have just given a whole answer to, or None.\n\n    Most faults are answered by the knowledge base rather than by this module (\"my washing\n    machine is leaking\" gets the valve, \"my back aches\" gets the two-weeks talk), so the thread\n    is on the table without this module having said anything. The detail that follows - \"only\n    when it spins\", \"it started yesterday\", \"its worse in the morning\" - is still the next line\n    of the same fault, and it was being said back as a fact about nothing.\n\n    Only the previous turn counts, and only when our answer did not ask anything: if the reply\n    ended in a question, the detail is the answer to that question and the ack already has it.\n    ");
    turns = { let __b = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    last = _flat(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { mcall(&mcall(&last.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    return Py::None;
    }
    said = _flat(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    return _shape(&[said.clone()]);
    Py::None
}
pub fn carried_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut answers_the_strain: Py = Py::None;
    let mut fuzzy: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut nexts: Py = Py::None;
    let mut padded: Py = Py::None;
    let mut says_what: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The replies for a detail added to the complaint we are already on, or None.\n\n    The first line of a complaint names what hurts (\"my tailbone hurts\") and the next line is\n    usually the detail kind of answer - when it is worse, what brings it on. With nothing to\n    carry it on, the composer answers that detail with a stock line that could have come before\n    any message at all (\"Thanks for telling me\"), which throws away the thread the pool had just\n    started. So this needs both halves: one of our own questions as the last reply, and a line\n    that is a detail about it rather than a new subject.\n    ");
    shape = _shape_on_the_table(&[memory.clone()]);
    fuzzy = Py::Bool(false);
    if py_truthy(&Py::Bool(!py_truthy(&shape.clone()))) {
    shape = _shape_answered(&[memory.clone()]);
    fuzzy = call_builtin("bool", &[shape.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&shape.clone()))) {
    return Py::None;
    }
    text = _flat(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&words.clone(), &Py::None))) {
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    words = call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    answers_the_strain = { let __b = Py::Bool(py_eq(&shape.clone(), &py_str("strain"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&fuzzy.clone())); if !py_truthy(&__b) { __b } else { _answers_a_strain_question(&[text.clone(), memory.clone()]) } } } };
    if py_truthy(&{ let __b = someone_elses(&[words.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_fault_of_a_thing(&[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&answers_the_strain.clone())) } } } }) {
    return Py::None;
    }
    padded = add(&add(&py_str(" "), &text.clone()), &py_str(" "));
    says_what = { let __b = answers_the_strain.clone(); if py_truthy(&__b) { __b } else { call_builtin("bool", &[{ let __b = mcall(&g_DETAIL(), "search", &[padded.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_TRIGGER(), "search", &[padded.clone()]) } }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = says_what.clone(); if py_truthy(&__b) { __b } else { mcall(&g_DURATION(), "search", &[padded.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = fuzzy.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_points_at_it(&[padded.clone(), memory.clone()]))) } }) {
    return Py::None;
    }
    nexts = mcall(&py_dict(vec![(py_str("sore"), g_SORE_NEXT()), (py_str("strain"), g_STRAIN_NEXT())]), "get", &[shape.clone(), g_BROKEN_NEXT()]);
    lines = call_builtin("list", &[_said_already(&[nexts.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&pool(&[shape.clone(), clean.clone(), memory.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_STRAIN(), &line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&lines.clone(), &line.clone())) } }) {
    continue;
    }
    let _ = mcall(&lines.clone(), "append", &[line.clone()]);
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_WHAT_IT_DOES(), "search", &[line.clone()])))) {  __out.push(line.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("tuple", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn reads_as_trouble(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut repaired: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { py_str("") };
    let mut shape: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("'thing' for a broken thing, 'sore' for their own body, None for anything else.\n\n    `repaired` is the same line with its wrong words put right (the reading's `plain`), and it\n    is only consulted when the line as typed is not trouble: the owner is the opening word, so\n    \"me knee is killing me\" is the same complaint as \"my knee is killing me\" and only the\n    repair knows it.");
    text = _without_a_dead_that_only_means_very(&[_without_wanting_or_praise(&[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&crate::gen::compose::badnews::reads_as_someones_death(&[text.clone()])) {
    return Py::None;
    }
    shape = _shape(&[text.clone(), words.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&shape.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&repaired.clone(), &text.clone())) } } } }) {
    shape = _shape(&[_without_a_dead_that_only_means_very(&[_without_wanting_or_praise(&[repaired.clone()])])]);
    }
    return shape.clone();
    Py::None
}
pub fn _without_wanting_or_praise(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("The line with a \"to die for\" / \"I'd die for\" taken out of it.\n\n    Nothing else goes, and the words around it stay where they were, so a line that carries both\n    the praise and a fault is still read for the fault: \"this soup is to die for but the oven is\n    broken\" is the oven.\n    ");
    return mcall(&g_WANTING_OR_PRAISE(), "sub", &[py_str(" "), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _without_a_dead_that_only_means_very(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut following: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with a \"dead\" that describes the next word taken out (see DEAD_AS_VERY_AFTER).\n\n    Only that word goes, so the rest of the line is read as it was: \"the boiler is dead and it\n    keeps cutting out\" is still the boiler.\n    ");
    words = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &py_str("dead")))) {
    return text.clone();
    }
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    following = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &add(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("dead"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DEAD_AS_VERY_AFTER(), &mcall(&following.clone(), "strip", &[g_ODD_MARK()]))) } }) {
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    return mcall(&py_str(" "), "join", &[kept.clone()]);
    Py::None
}
pub fn _someone_is_the_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut following: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a person or an animal in the line is what it is about.\n\n    A name is the subject unless it is the front half of a thing's name: \"the cat flap is\n    broken\" is a flap, while \"my cat is ill\" and \"my son took all my pills\" are theirs, and an\n    emergency about somebody else is not a fault to talk through.\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&someone_elses(&[py_tuple(vec![word.clone()])])))) {
    continue;
    }
    following = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &add(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ANIMAL_MODIFIERS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = following.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_MODIFIER(), &following.clone())) } } } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_noun_before(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut words: Py = Py::None;
    let _ = py_str("The noun just before a match, whoever owns it: \"the leaves\" is leaves, \"my slabs\" is slabs.\n\n    _the_thing_it_hangs_on answers a different question - whether the fault is a thing of theirs\n    rather than the world's, so it says nothing when the owner is \"the\". This one only wants the\n    word, for readings about what kind of thing it is.\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), add(&py_str(" "), &slice_or_raise(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &at.clone(), &Py::None))]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_JOINING(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    return if py_truthy(&words.clone()) { geti(&words.clone(), &neg(&Py::Int(1))) } else { py_str("") };
    Py::None
}
pub fn _a_fault_of_a_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut padded: Py = Py::None;
    let _ = py_str("True when the fault in the line hangs on a thing, not on a person or an animal.");
    padded = add(&add(&py_str(" "), &text.clone()), &py_str(" "));
    return call_builtin("bool", &[{ let __b = mcall(&g_THING(), "search", &[padded.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_OWNS(), "search", &[padded.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_someone_is_the_subject(&[text.clone()]))) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_a_body_part(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("True when the line names a part of a body, which is what a bodily pain needs.");
    return call_builtin("bool", &[bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_BODY_PART())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _figurative_passes(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("True when a line whose only ache word can also mean \"a nuisance\" is about the body.\n\n    A named body part settles it. Without one, \"my job is a pain\" and \"my phone is painful to\n    use\" stand aside, while \"i am in pain\" and \"i have a pain in my side\" stay: the nuisance\n    reading needs a thing of theirs to be the nuisance.");
    if py_truthy(&_names_a_body_part(&[text.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&g_NUISANCE(), "search", &[text.clone()])));
    Py::None
}
pub fn _shape(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut a_state: Py = Py::None;
    let mut ache: Py = Py::None;
    let mut body_word: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut growing_state: Py = Py::None;
    let mut hangs_on: Py = Py::None;
    let mut hung_on: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut own: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The shape of the trouble in this one text, or None.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    words = if py_truthy(&words.clone()) { call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) };
    mine = add(&add(&py_str(" "), &text.clone()), &py_str(" "));
    a_state = mcall(&g_THING_STATE_ONLY(), "search", &[mine.clone()]);
    growing_state = call_builtin("bool", &[{ let __b = a_state.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GROWING(), &_the_noun_before(&[mine.clone(), mcall(&a_state.clone(), "start", &[])]))) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = someone_elses(&[words.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_fault_of_a_thing(&[text.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_PLEA(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DEATHWISH(), "search", &[text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_ACUTE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = a_state.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&growing_state.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &g_BODY_PART()))) } } } })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_FRACTURE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { bitand(&words.clone(), &bitor(&g_BODY_PART(), &g_BONE())) } }) {
    return Py::None;
    }
    hung_on = _the_fault_hangs_on(&[text.clone()]);
    if py_truthy(&{ let __b = mcall(&g_BREAKS(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_NOT_A_THING()); if py_truthy(&__b) { __b } else { bitand(&hung_on.clone(), &g_NOT_A_THING()) } } } }) {
    return Py::None;
    }
    own = call_builtin("bool", &[mcall(&g_MINE(), "match", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    body_word = mcall(&g_BODY(), "search", &[mine.clone()]);
    if py_truthy(&{ let __b = a_state.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&growing_state.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &g_BODY_PART()))) } } } }) {
    body_word = Py::None;
    }
    ache = { let __b = body_word.clone(); if py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_BODY_PART()); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:dead|killing|kills|hurts?|banging|thumping|pounding|splitting|had\\s+it|done\\s+for)\\b"), mine.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } };
    if py_truthy(&{ let __b = own.clone(); if !py_truthy(&__b) { __b } else { ache.clone() } }) {
    if py_truthy(&{ let __b = mcall(&g_FIGURATIVE(), "search", &[mine.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_figurative_passes(&[text.clone()]))) } }) {
    return Py::None;
    }
    return py_str("sore");
    }
    flat = mcall(&g_FLAT(), "search", &[mine.clone()]);
    if py_truthy(&{ let __b = flat.clone(); if !py_truthy(&__b) { __b } else { bitand(&_the_fault_hangs_on(&[mine.clone()]), &g_FLAT_SUBJECT()) } }) {
    return py_str("strain");
    }
    if py_truthy(&{ let __b = crate::gen::memory::setback::reading(&[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &g_BODY_PART()))) } }) {
    return py_str("vague");
    }
    thing = mcall(&g_THING(), "search", &[mine.clone()]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_OWNS(), "search", &[mine.clone()]) } }) {
    hangs_on = _the_thing_it_hangs_on(&[mine.clone(), mcall(&thing.clone(), "start", &[])]);
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &hangs_on.clone()))) {
    return py_str("strain");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BODY_PART(), &hangs_on.clone())); if !py_truthy(&__b) { __b } else { { let __b = a_state.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&a_state.clone(), "start", &[]), &mcall(&thing.clone(), "start", &[]))) } } } }) {
    return py_str("sore");
    }
    if py_truthy(&{ let __b = growing_state.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&a_state.clone(), "start", &[]), &mcall(&thing.clone(), "start", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BODY_PART(), &hangs_on.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_A_DECLINE(), "search", &[slice_or_raise(&mine.clone(), &mcall(&thing.clone(), "start", &[]), &Py::None, &Py::None)]) } }) {
    return Py::None;
    }
    return py_str("thing");
    }
    if py_truthy(&{ let __b = mcall(&g_CANT_GET(), "search", &[mine.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_OWNS(), "search", &[mine.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &g_BODY_PART()))) } } } }) {
    return py_str("thing");
    }
    if py_truthy(&{ let __b = _keeps_doing_it(&[mine.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &g_BODY_PART()))) } }) {
    return py_str("thing");
    }
    return Py::None;
    Py::None
}
pub fn _keeps_doing_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut before: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut head: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when something of theirs keeps doing the wrong thing: \"my razor keeps pulling\".\n\n    The thing the fault hangs on has to be a thing. In \"i keep waking up at four\" the subject is\n    the person, in \"my knee keeps clicking\" it is their body, and in \"my confidence keeps going\"\n    it is a part of their life; those are not a machine to talk through. A word that names a way\n    the thing is supposed to work (KEEPS_FINE) is not a fault either - \"the fridge keeps running\"\n    is a fridge, not a problem - so only a verb outside that set makes the line this one.\n    ");
    if py_truthy(&mcall(&g_ASKS_TO_CHOOSE(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_KEEPS_AT_IT(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    doing = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&Py::Bool(py_contains(&g_KEEPS_FINE(), &doing.clone()))) {
    continue;
    }
    before = stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&g_KEEPS_THEIRS(), &call_builtin("set", &[slice_or_raise(&before.clone(), &neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)))))) {
    continue;
    }
    head = py_str("");
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FAULT_FILLERS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_THE_THING(), &word.clone())) } }) {
    continue;
    }
    head = word.clone();
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OTHERS(), &head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BODY_PART(), &head.clone())) } } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_KEEPS_NOT_A_THING(), &head.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_KEEPS_NOT_THEIR_THING(), &head.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_OWNS(), "search", &[text.clone()])))) {
    continue;
    }
    phrase = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FAULT_FILLERS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_THE_THING(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&phrase.clone(), &g_KEEPS_POSSESSIVE()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&phrase.clone(), &g_KEEPS_OF_A_HOUSE()))) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_thing_it_hangs_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut _u: Py = Py::None;
    let mut back: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The word a fault hangs on: the noun just before it, past the words that only join them.\n\n    \"my career is stuck\" is a career, \"my washing machine has stopped working\" is the machine,\n    and \"my home wifi keeps dropping\" is the wifi - which is what keeps a line about a part of\n    their life apart from a line about a machine that is in it.\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&text.clone(), &Py::None, &at.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_JOINING(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    noun = geti(&words.clone(), &neg(&Py::Int(1)));
    back = slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&back.clone()))) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &geti(&back.clone(), &neg(&Py::Int(1)))))) {
    return noun.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_OWNED(), &geti(&back.clone(), &neg(&Py::Int(1)))))) {
    return py_str("");
    }
    let _ = mcall(&back.clone(), "pop", &[]);
    }
    return py_str("");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.trouble".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACUTE" => Some(g_ACUTE()),
            "ALREADY_TRIED" => Some(g_ALREADY_TRIED()),
            "ANIMAL_MODIFIERS" => Some(g_ANIMAL_MODIFIERS()),
            "ASKS_TO_CHOOSE" => Some(g_ASKS_TO_CHOOSE()),
            "A_DECLINE" => Some(g_A_DECLINE()),
            "A_THING_PACKED_UP" => Some(g_A_THING_PACKED_UP()),
            "BODY" => Some(g_BODY()),
            "BODY_PART" => Some(g_BODY_PART()),
            "BONE" => Some(g_BONE()),
            "BREAKS" => Some(g_BREAKS()),
            "BROKEN" => Some(g_BROKEN()),
            "BROKEN_NEXT" => Some(g_BROKEN_NEXT()),
            "BROKEN_TIME" => Some(g_BROKEN_TIME()),
            "CANT_GET" => Some(g_CANT_GET()),
            "DEAD_AS_VERY_AFTER" => Some(g_DEAD_AS_VERY_AFTER()),
            "DEATHWISH" => Some(g_DEATHWISH()),
            "DECLINE" => Some(g_DECLINE()),
            "DECLINE_TIME" => Some(g_DECLINE_TIME()),
            "DETAIL" => Some(g_DETAIL()),
            "DURATION" => Some(g_DURATION()),
            "FAULT_FILLERS" => Some(g_FAULT_FILLERS()),
            "FIGURATIVE" => Some(g_FIGURATIVE()),
            "FLAT" => Some(g_FLAT()),
            "FLAT_SUBJECT" => Some(g_FLAT_SUBJECT()),
            "FRACTURE" => Some(g_FRACTURE()),
            "GROWING" => Some(g_GROWING()),
            "IT" => Some(g_IT()),
            "JOINING" => Some(g_JOINING()),
            "KEEPS_AT_IT" => Some(g_KEEPS_AT_IT()),
            "KEEPS_FINE" => Some(g_KEEPS_FINE()),
            "KEEPS_NOT_A_THING" => Some(g_KEEPS_NOT_A_THING()),
            "KEEPS_NOT_THEIR_THING" => Some(g_KEEPS_NOT_THEIR_THING()),
            "KEEPS_OF_A_HOUSE" => Some(g_KEEPS_OF_A_HOUSE()),
            "KEEPS_POSSESSIVE" => Some(g_KEEPS_POSSESSIVE()),
            "KEEPS_THEIRS" => Some(g_KEEPS_THEIRS()),
            "MINE" => Some(g_MINE()),
            "MINE_AGAIN" => Some(g_MINE_AGAIN()),
            "NOT_A_MODIFIER" => Some(g_NOT_A_MODIFIER()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_OWNED" => Some(g_NOT_OWNED()),
            "NOT_THE_THING" => Some(g_NOT_THE_THING()),
            "NUISANCE" => Some(g_NUISANCE()),
            "ODD_MARK" => Some(g_ODD_MARK()),
            "OTHERS" => Some(g_OTHERS()),
            "OWNER_END" => Some(g_OWNER_END()),
            "OWNS" => Some(g_OWNS()),
            "PLEA" => Some(g_PLEA()),
            "SORE" => Some(g_SORE()),
            "SORE_NEXT" => Some(g_SORE_NEXT()),
            "SORE_TIME" => Some(g_SORE_TIME()),
            "STILL_FAILING" => Some(g_STILL_FAILING()),
            "STRAIN" => Some(g_STRAIN()),
            "STRAIN_NEXT" => Some(g_STRAIN_NEXT()),
            "STRAIN_TIME" => Some(g_STRAIN_TIME()),
            "THE_END_OF_A_THING" => Some(g_THE_END_OF_A_THING()),
            "THING" => Some(g_THING()),
            "THING_STATE" => Some(g_THING_STATE()),
            "THING_STATE_ONLY" => Some(g_THING_STATE_ONLY()),
            "TRIGGER" => Some(g_TRIGGER()),
            "WANTING_OR_PRAISE" => Some(g_WANTING_OR_PRAISE()),
            "WHAT_IT_DOES" => Some(g_WHAT_IT_DOES()),
            "_A_HEDGE" => Some(g__A_HEDGE()),
            "_A_LENGTH_OF_TIME" => Some(g__A_LENGTH_OF_TIME()),
            "_A_SENTENCE" => Some(g__A_SENTENCE()),
            "_NOT_A_STRAIN_DETAIL" => Some(g__NOT_A_STRAIN_DETAIL()),
            "_a_card_answers" => Some(py_func("_a_card_answers", |args: &[Py]| _a_card_answers(args))),
            "_a_fault_of_a_thing" => Some(py_func("_a_fault_of_a_thing", |args: &[Py]| _a_fault_of_a_thing(args))),
            "_a_length_of_time" => Some(py_func("_a_length_of_time", |args: &[Py]| _a_length_of_time(args))),
            "_answers_a_strain_question" => Some(py_func("_answers_a_strain_question", |args: &[Py]| _answers_a_strain_question(args))),
            "_figurative_passes" => Some(py_func("_figurative_passes", |args: &[Py]| _figurative_passes(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_keeps_doing_it" => Some(py_func("_keeps_doing_it", |args: &[Py]| _keeps_doing_it(args))),
            "_names_a_body_part" => Some(py_func("_names_a_body_part", |args: &[Py]| _names_a_body_part(args))),
            "_only_the_decline" => Some(py_func("_only_the_decline", |args: &[Py]| _only_the_decline(args))),
            "_points_at_it" => Some(py_func("_points_at_it", |args: &[Py]| _points_at_it(args))),
            "_said_already" => Some(py_func("_said_already", |args: &[Py]| _said_already(args))),
            "_shape" => Some(py_func("_shape", |args: &[Py]| _shape(args))),
            "_shape_answered" => Some(py_func("_shape_answered", |args: &[Py]| _shape_answered(args))),
            "_shape_on_the_table" => Some(py_func("_shape_on_the_table", |args: &[Py]| _shape_on_the_table(args))),
            "_someone_is_the_subject" => Some(py_func("_someone_is_the_subject", |args: &[Py]| _someone_is_the_subject(args))),
            "_the_fault_hangs_on" => Some(py_func("_the_fault_hangs_on", |args: &[Py]| _the_fault_hangs_on(args))),
            "_the_noun_before" => Some(py_func("_the_noun_before", |args: &[Py]| _the_noun_before(args))),
            "_the_thing_it_hangs_on" => Some(py_func("_the_thing_it_hangs_on", |args: &[Py]| _the_thing_it_hangs_on(args))),
            "_without_a_dead_that_only_means_very" => Some(py_func("_without_a_dead_that_only_means_very", |args: &[Py]| _without_a_dead_that_only_means_very(args))),
            "_without_wanting_or_praise" => Some(py_func("_without_wanting_or_praise", |args: &[Py]| _without_wanting_or_praise(args))),
            "answers_a_strain_question" => Some(py_func("answers_a_strain_question", |args: &[Py]| answers_a_strain_question(args))),
            "carried_on" => Some(py_func("carried_on", |args: &[Py]| carried_on(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "reads_as_trouble" => Some(py_func("reads_as_trouble", |args: &[Py]| reads_as_trouble(args))),
            "someone_elses" => Some(py_func("someone_elses", |args: &[Py]| someone_elses(args))),
            _ => None,
        }),
        set: None,
    }))
}
