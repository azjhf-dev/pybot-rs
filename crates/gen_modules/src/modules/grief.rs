#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use pyrt::builtins::*;
use pyrt::pybuiltins::*;
use pyrt::stdlib::*;
use pyrt::value::*;
use pyrt::exc;
use pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


thread_local! { static G_ADMIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADMIN_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL_PAPERWORK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL_SPAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS_INSTEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARRANGEMENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARRANGEMENTS_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARRANGEMENTS_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BELONGINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BELONGINGS_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANT_TALK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANT_TALK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPANY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COST_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRYING_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEFERS_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT_IT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT_ME_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMERGENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EULOGY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EULOGY_WRITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXPECTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXPECTED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GUILT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GUILT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HARD_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOSS_SPEAKERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSING_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPEN_ASK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAPERWORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAPERWORK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_OLD_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_YOUNG_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRACTICAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROBATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROBATE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REGISTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REGISTER_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMEMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAY_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAY_TO_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_LIFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGNS_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUDDEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUDDEN_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALK_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALK_ABOUT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALK_TO_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELLING_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELL_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKS_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_PRESSURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_PRESSURE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YOUNG_AGE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ADMIN() -> Py { ensure_init(); G_ADMIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADMIN(v: Py) { ensure_init(); G_ADMIN.with(|c| *c.borrow_mut() = v); }
pub fn g_ADMIN_REPLIES() -> Py { ensure_init(); G_ADMIN_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADMIN_REPLIES(v: Py) { ensure_init(); G_ADMIN_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE() -> Py { ensure_init(); G_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE(v: Py) { ensure_init(); G_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_NUMBER() -> Py { ensure_init(); G_AGE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_NUMBER(v: Py) { ensure_init(); G_AGE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_REPLIES() -> Py { ensure_init(); G_AGE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_REPLIES(v: Py) { ensure_init(); G_AGE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL_PAPERWORK_REPLIES() -> Py { ensure_init(); G_ANIMAL_PAPERWORK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_PAPERWORK_REPLIES(v: Py) { ensure_init(); G_ANIMAL_PAPERWORK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL_SPAN() -> Py { ensure_init(); G_ANIMAL_SPAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_SPAN(v: Py) { ensure_init(); G_ANIMAL_SPAN.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS_INSTEAD() -> Py { ensure_init(); G_ANSWERS_INSTEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_INSTEAD(v: Py) { ensure_init(); G_ANSWERS_INSTEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_ARRANGEMENTS() -> Py { ensure_init(); G_ARRANGEMENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARRANGEMENTS(v: Py) { ensure_init(); G_ARRANGEMENTS.with(|c| *c.borrow_mut() = v); }
pub fn g_ARRANGEMENTS_ELSE() -> Py { ensure_init(); G_ARRANGEMENTS_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARRANGEMENTS_ELSE(v: Py) { ensure_init(); G_ARRANGEMENTS_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_ARRANGEMENTS_REPLIES() -> Py { ensure_init(); G_ARRANGEMENTS_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARRANGEMENTS_REPLIES(v: Py) { ensure_init(); G_ARRANGEMENTS_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_OPENERS() -> Py { ensure_init(); G_ASK_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OPENERS(v: Py) { ensure_init(); G_ASK_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_BELONGINGS() -> Py { ensure_init(); G_BELONGINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BELONGINGS(v: Py) { ensure_init(); G_BELONGINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_BELONGINGS_REPLIES() -> Py { ensure_init(); G_BELONGINGS_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BELONGINGS_REPLIES(v: Py) { ensure_init(); G_BELONGINGS_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CANT_TALK() -> Py { ensure_init(); G_CANT_TALK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANT_TALK(v: Py) { ensure_init(); G_CANT_TALK.with(|c| *c.borrow_mut() = v); }
pub fn g_CANT_TALK_REPLIES() -> Py { ensure_init(); G_CANT_TALK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANT_TALK_REPLIES(v: Py) { ensure_init(); G_CANT_TALK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPANY_REPLIES() -> Py { ensure_init(); G_COMPANY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPANY_REPLIES(v: Py) { ensure_init(); G_COMPANY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_COST() -> Py { ensure_init(); G_COST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COST(v: Py) { ensure_init(); G_COST.with(|c| *c.borrow_mut() = v); }
pub fn g_COST_REPLIES() -> Py { ensure_init(); G_COST_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COST_REPLIES(v: Py) { ensure_init(); G_COST_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CRYING() -> Py { ensure_init(); G_CRYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRYING(v: Py) { ensure_init(); G_CRYING.with(|c| *c.borrow_mut() = v); }
pub fn g_CRYING_REPLIES() -> Py { ensure_init(); G_CRYING_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRYING_REPLIES(v: Py) { ensure_init(); G_CRYING_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_DEFERS_TO() -> Py { ensure_init(); G_DEFERS_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEFERS_TO(v: Py) { ensure_init(); G_DEFERS_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT_IT() -> Py { ensure_init(); G_DOUBT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT_IT(v: Py) { ensure_init(); G_DOUBT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT_IT_REPLIES() -> Py { ensure_init(); G_DOUBT_IT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT_IT_REPLIES(v: Py) { ensure_init(); G_DOUBT_IT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT_ME() -> Py { ensure_init(); G_DOUBT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT_ME(v: Py) { ensure_init(); G_DOUBT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT_ME_REPLIES() -> Py { ensure_init(); G_DOUBT_ME_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT_ME_REPLIES(v: Py) { ensure_init(); G_DOUBT_ME_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EMERGENCY() -> Py { ensure_init(); G_EMERGENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY(v: Py) { ensure_init(); G_EMERGENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_EULOGY() -> Py { ensure_init(); G_EULOGY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EULOGY(v: Py) { ensure_init(); G_EULOGY.with(|c| *c.borrow_mut() = v); }
pub fn g_EULOGY_WRITE() -> Py { ensure_init(); G_EULOGY_WRITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EULOGY_WRITE(v: Py) { ensure_init(); G_EULOGY_WRITE.with(|c| *c.borrow_mut() = v); }
pub fn g_EXPECTED() -> Py { ensure_init(); G_EXPECTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXPECTED(v: Py) { ensure_init(); G_EXPECTED.with(|c| *c.borrow_mut() = v); }
pub fn g_EXPECTED_REPLIES() -> Py { ensure_init(); G_EXPECTED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXPECTED_REPLIES(v: Py) { ensure_init(); G_EXPECTED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY() -> Py { ensure_init(); G_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY(v: Py) { ensure_init(); G_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_REPLIES() -> Py { ensure_init(); G_FAMILY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_REPLIES(v: Py) { ensure_init(); G_FAMILY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_GUILT() -> Py { ensure_init(); G_GUILT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GUILT(v: Py) { ensure_init(); G_GUILT.with(|c| *c.borrow_mut() = v); }
pub fn g_GUILT_REPLIES() -> Py { ensure_init(); G_GUILT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GUILT_REPLIES(v: Py) { ensure_init(); G_GUILT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_HARD() -> Py { ensure_init(); G_HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HARD(v: Py) { ensure_init(); G_HARD.with(|c| *c.borrow_mut() = v); }
pub fn g_HARD_REPLIES() -> Py { ensure_init(); G_HARD_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HARD_REPLIES(v: Py) { ensure_init(); G_HARD_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_LOSS_SPEAKERS() -> Py { ensure_init(); G_LOSS_SPEAKERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOSS_SPEAKERS(v: Py) { ensure_init(); G_LOSS_SPEAKERS.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSING() -> Py { ensure_init(); G_MISSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSING(v: Py) { ensure_init(); G_MISSING.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSING_REPLIES() -> Py { ensure_init(); G_MISSING_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSING_REPLIES(v: Py) { ensure_init(); G_MISSING_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_OPEN_ASK() -> Py { ensure_init(); G_OPEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPEN_ASK(v: Py) { ensure_init(); G_OPEN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OPEN_ASK_REPLIES() -> Py { ensure_init(); G_OPEN_ASK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPEN_ASK_REPLIES(v: Py) { ensure_init(); G_OPEN_ASK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PERSON() -> Py { ensure_init(); G_OTHER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PERSON(v: Py) { ensure_init(); G_OTHER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PAPERWORK() -> Py { ensure_init(); G_PAPERWORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAPERWORK(v: Py) { ensure_init(); G_PAPERWORK.with(|c| *c.borrow_mut() = v); }
pub fn g_PAPERWORK_REPLIES() -> Py { ensure_init(); G_PAPERWORK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAPERWORK_REPLIES(v: Py) { ensure_init(); G_PAPERWORK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_ONLY() -> Py { ensure_init(); G_PERSON_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_ONLY(v: Py) { ensure_init(); G_PERSON_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_LOSS() -> Py { ensure_init(); G_PET_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_LOSS(v: Py) { ensure_init(); G_PET_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_OLD_REPLIES() -> Py { ensure_init(); G_PET_OLD_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_OLD_REPLIES(v: Py) { ensure_init(); G_PET_OLD_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_YOUNG_REPLIES() -> Py { ensure_init(); G_PET_YOUNG_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_YOUNG_REPLIES(v: Py) { ensure_init(); G_PET_YOUNG_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE() -> Py { ensure_init(); G_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE(v: Py) { ensure_init(); G_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRACTICAL() -> Py { ensure_init(); G_PRACTICAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRACTICAL(v: Py) { ensure_init(); G_PRACTICAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_PROBATE() -> Py { ensure_init(); G_PROBATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROBATE(v: Py) { ensure_init(); G_PROBATE.with(|c| *c.borrow_mut() = v); }
pub fn g_PROBATE_REPLIES() -> Py { ensure_init(); G_PROBATE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROBATE_REPLIES(v: Py) { ensure_init(); G_PROBATE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_REGISTER() -> Py { ensure_init(); G_REGISTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REGISTER(v: Py) { ensure_init(); G_REGISTER.with(|c| *c.borrow_mut() = v); }
pub fn g_REGISTER_REPLIES() -> Py { ensure_init(); G_REGISTER_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REGISTER_REPLIES(v: Py) { ensure_init(); G_REGISTER_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_REMEMBER() -> Py { ensure_init(); G_REMEMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMEMBER(v: Py) { ensure_init(); G_REMEMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_SAY_TO() -> Py { ensure_init(); G_SAY_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAY_TO(v: Py) { ensure_init(); G_SAY_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_SAY_TO_REPLIES() -> Py { ensure_init(); G_SAY_TO_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAY_TO_REPLIES(v: Py) { ensure_init(); G_SAY_TO_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT() -> Py { ensure_init(); G_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); G_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_LIFE() -> Py { ensure_init(); G_SHORT_LIFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_LIFE(v: Py) { ensure_init(); G_SHORT_LIFE.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGNS() -> Py { ensure_init(); G_SIGNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGNS(v: Py) { ensure_init(); G_SIGNS.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGNS_REPLIES() -> Py { ensure_init(); G_SIGNS_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGNS_REPLIES(v: Py) { ensure_init(); G_SIGNS_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP() -> Py { ensure_init(); G_SLEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP(v: Py) { ensure_init(); G_SLEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP_REPLIES() -> Py { ensure_init(); G_SLEEP_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP_REPLIES(v: Py) { ensure_init(); G_SLEEP_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_SUDDEN() -> Py { ensure_init(); G_SUDDEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUDDEN(v: Py) { ensure_init(); G_SUDDEN.with(|c| *c.borrow_mut() = v); }
pub fn g_SUDDEN_REPLIES() -> Py { ensure_init(); G_SUDDEN_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUDDEN_REPLIES(v: Py) { ensure_init(); G_SUDDEN_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TALK_ABOUT() -> Py { ensure_init(); G_TALK_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALK_ABOUT(v: Py) { ensure_init(); G_TALK_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_TALK_ABOUT_REPLIES() -> Py { ensure_init(); G_TALK_ABOUT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALK_ABOUT_REPLIES(v: Py) { ensure_init(); G_TALK_ABOUT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TALK_TO_SOMEONE() -> Py { ensure_init(); G_TALK_TO_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALK_TO_SOMEONE(v: Py) { ensure_init(); G_TALK_TO_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_TELL() -> Py { ensure_init(); G_TELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELL(v: Py) { ensure_init(); G_TELL.with(|c| *c.borrow_mut() = v); }
pub fn g_TELLING() -> Py { ensure_init(); G_TELLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELLING(v: Py) { ensure_init(); G_TELLING.with(|c| *c.borrow_mut() = v); }
pub fn g_TELLING_REPLIES() -> Py { ensure_init(); G_TELLING_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELLING_REPLIES(v: Py) { ensure_init(); G_TELLING_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TELL_REPLIES() -> Py { ensure_init(); G_TELL_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELL_REPLIES(v: Py) { ensure_init(); G_TELL_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKS_LINE() -> Py { ensure_init(); G_THANKS_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKS_LINE(v: Py) { ensure_init(); G_THANKS_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK() -> Py { ensure_init(); G_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK(v: Py) { ensure_init(); G_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_PRESSURE() -> Py { ensure_init(); G_WORK_PRESSURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_PRESSURE(v: Py) { ensure_init(); G_WORK_PRESSURE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_PRESSURE_REPLIES() -> Py { ensure_init(); G_WORK_PRESSURE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_PRESSURE_REPLIES(v: Py) { ensure_init(); G_WORK_PRESSURE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_REPLIES() -> Py { ensure_init(); G_WORK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_REPLIES(v: Py) { ensure_init(); G_WORK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_WORD() -> Py { ensure_init(); G_WORK_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_WORD(v: Py) { ensure_init(); G_WORK_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_YOUNG_AGE_REPLIES() -> Py { ensure_init(); G_YOUNG_AGE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YOUNG_AGE_REPLIES(v: Py) { ensure_init(); G_YOUNG_AGE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g__LIVING() -> Py { ensure_init(); G__LIVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIVING(v: Py) { ensure_init(); G__LIVING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A loss does not end when the sympathy is said: the next few turns are still about it.\n\nThe composer reads a death and answers it well (bot/compose/loss.py), but it reads one line at\na time. The lines people actually send next — \"it was really sudden\", \"he was 14\", \"i keep\nthinking i should have visited more\", \"i cant sleep properly\" — came back as small talk (\"Got\nit. Is there anything you'd like a hand with?\"), which reads as if the previous turn never\nhappened. This keeps the loss in `memory[\"loss\"]` and answers those short remarks inside it,\nnaming the person each time, for a few turns.\n\nIt answers nothing else. A new line about a death is the composer's (it says the sympathy, and\nthe arbiter gives it the turn); a line with no cue, or one that has turned to another subject,\npasses to the modules that know about that subject.\n");
    s_PRIORITY(Py::Int(2100));
    s_ANSWERS(py_tuple(vec![py_str("feelings"), py_str("grief"), py_str("loss")]));
    s_ANSWERS_INSTEAD(py_tuple(vec![py_str("feelings"), py_str("grief"), py_str("loss")]));
    s_DEFERS_TO(g_defers_to());
    s_REMEMBER(Py::Int(3));
    s_SHORT(Py::Int(16));
    s_POSSESSIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|his|her|their)\\s+$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUDDEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:sudden|suddenly|out of the blue|no warning|shock|shocked)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EXPECTED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it was expected|was expected|we knew (?:it|this) was coming|we saw it coming|long illness|not a surprise|no surprise|been ill for (?:a|years|months))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SLEEP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cant|can'?t|couldnt|couldn'?t|cannot|unable to|barely|hardly|not|never)\\s+(?:sleep|sleeping|slept|stay asleep|get to sleep|get any sleep)\\b|\\b(?:havent|haven'?t|hasnt|hasn'?t|hadnt|hadn'?t|didnt|didn'?t)\\s+(?:slept|been sleeping|had (?:any |much |much of any )?sleep)\\b|\\b(?:keep|kept|still)\\s+waking\\b|\\bwaking (?:up )?(?:at|every|in the)\\b|\\bwoke (?:up )?at\\b|\\bawake (?:at|all|most of the)\\b|\\b(?:insomnia|sleepless|lying awake)\\b|\\b(?:3|4|5)\\s?am\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GUILT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:should have|should've|shouldnt have|blame myself|my fault|regret|if only|wish i (?:had|could|was there)|didn'?t visit|didn'?t go|guilty (?:about|that|for)|so guilty i|more time with)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HARD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|that|yesterday|today|this morning|the day|last night)\\s+(?:was|is|has been|'s been)\\s+(?:so |really |very |pretty |just )?(?:hard|tough|awful|horrible|brutal|rough|difficult|shit|hell|the worst)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CRYING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cry|crying|cried|tears|sobbing)\\b|\\bi broke down\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TALK_ABOUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cant|can't|couldnt|couldn't|could not|dont|don't|do not|not able to|not ready to|struggl\\w* to|hard to|too (?:hard|painful|raw|much) to)\\s+(?:really\\s+|even\\s+|ever\\s+)?talk(?:ing)?\\s+(?:about|of)\\s+(?:it|him|her|them|that|this|any of it|any of that)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LIVING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitor(&gen_compose::compose::loss::g_PEOPLE(), &gen_compose::compose::loss::g_PET_WORDS())); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) });
    s_MISSING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:miss (?:her|him|them)|i miss my (?:"), &g__LIVING()), &py_str(")|think about (?:her|him|them)|(?:the|her|his|their) birthday|anniversary|dreamt? about (?:her|him)|photos? of (?:her|him|them))\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TALK_TO_SOMEONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:talk|talking|speak|speaking)\\s+(?:to|with)\\s+(?:"), &g__LIVING()), &py_str("|my|our|the|his|her|a|an)\\b|\\babout (?:it|that|this)\\s+(?:with|to)\\s+(?:")), &g__LIVING()), &py_str("|my|our|the|his|her|a|an)\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CANT_TALK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+(?:still\\s+|just\\s+|really\\s+)?(?:cant|can't|couldnt|couldn't|cannot|won't|wont|dont|don't|do not|wouldnt|wouldn't|am not|not)\\b[^.!?]{0,24}\\b(?:talk|talking|speak|speaking|say|saying)\\b[^.!?]{0,16}\\b(?:about (?:it|that|this|him|her|them)|yet|now|today)\\b|\\b(?:its|it's|that's|thats)\\s+(?:too\\s+(?:soon|raw|early|fresh)|too hard)\\b|\\bnot ready to talk\\b|\\bcan we not talk about\\b|\\blets not talk about\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SIGNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:still|keep) (?:look|looking|listen|listening|wait|waiting|expect|expecting) for (?:her|him|them)|(?:still|keep) (?:expect|expecting) (?:her|him|them)|(?:still|keep) (?:talk|talking|say|saying|tell|telling)\\b[^.]{0,20}?\\b(?:her|him|them)\\b|think(?:ing)? i (?:see|hear) (?:her|him)|half expect|hear (?:her|him) voice")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|it|they)\\s+was\\s+\\d{1,2}\\b(?!\\s*(?:degrees|miles|kilos|kg|pounds|quid|minutes|hours|days|weeks|months|years|per cent|%))|\\baged\\s+\\d{1,2}\\b|\\bin (?:her|his) (?:70s|80s|90s)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AGE_NUMBER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|it|they)\\s+was\\s+(\\d{1,2})\\b|\\baged\\s+(\\d{1,2})\\b|\\bin (?:her|his) (\\d)0s\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SHORT_LIFE(Py::Int(70));
    s_ANIMAL_SPAN(py_dict(vec![(py_str("dog"), Py::Int(12)), (py_str("puppy"), Py::Int(12)), (py_str("cat"), Py::Int(15)), (py_str("kitten"), Py::Int(15)), (py_str("rabbit"), Py::Int(8)), (py_str("bunny"), Py::Int(8)), (py_str("hamster"), Py::Int(2)), (py_str("guinea"), Py::Int(6)), (py_str("budgie"), Py::Int(8)), (py_str("parrot"), Py::Int(40)), (py_str("horse"), Py::Int(25)), (py_str("pony"), Py::Int(25)), (py_str("ferret"), Py::Int(8)), (py_str("rat"), Py::Int(2)), (py_str("fish"), Py::Int(5)), (py_str("tortoise"), Py::Int(60)), (py_str("snake"), Py::Int(15)), (py_str("lizard"), Py::Int(10)), (py_str("gerbil"), Py::Int(3)), (py_str("chinchilla"), Py::Int(10)), (py_str("cockatiel"), Py::Int(15)), (py_str("guinea pig"), Py::Int(6))]));
    s_BELONGINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:her|his|their|the)\\s+(?:things|stuff|clothes|belongings|flat|house|room|wardrobe|shed)\\b|\\bpack(?:ing)? (?:it|them|her|his) up\\b|\\bclear(?:ing)? (?:out|it|them)\\b|\\bempty(?:ing)? (?:it|them|the)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EULOGY_WRITE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:write|writing|wrote|draft|drafting|help|how|start|starting|opening|open|begin|length|long|words|minutes|structure|template|examples?|what do i say|say)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EULOGY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:eulogy|eulogies)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ARRANGEMENTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:funeral|burial|cremation|eulogy|coffin|undertaker|funeral director|headstone|graveside|cemetery|order of service)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REGISTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:register (?:the|her|his) death|registration|register office|registrar|death certificate|medical certificate)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cost|costs|price|prices|priced|how much (?:does|is|will|do|are)|expensive|afford|cheap\\w*|quote|pay for)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORK_PRESSURE_REPLIES(py_tuple(vec![py_str("You don't owe anyone the details. \"{Who} died on Tuesday, I won't be in this week\" is enough for the first message, and a work friend can pass it round so you are not saying it again and again."), py_str("Going back is often harder than the days before it. Ask for a phased return, a shorter week to start with, and one person to go to when it gets too much: most managers would rather you asked than quietly struggled."), py_str("It is fine to say \"I'd rather not talk about it, but thank you\" and leave it there. People ask because they care and then run out of words, so give them the line and they will use it.")]));
    s_WORK_REPLIES(py_tuple(vec![py_str("Ask your manager in writing and keep it plain: what has happened, and how many days you need. Most employers have a bereavement leave policy, and HR has to give you it in a sentence if you ask."), py_str("You don't owe anyone the details. \"{Who} died on Tuesday, I won't be in this week\" is enough for the first message, and a work friend can send it if a phone call is too much."), py_str("Work can wait a week. Say what you can cover before you go and who is picking it up, then let the rest go - people cover for each other, and this is what that is for.")]));
    s_PROBATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:probate|grant of probate|letters of administration)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PROBATE_REPLIES(py_tuple(vec![py_str("Probate is the court's permission to deal with the estate: you apply to HMCTS with the will and the death certificate. A straightforward application is usually decided within a few weeks, and a solicitor takes it on when the estate is large or the family expect a disagreement."), py_str("How long probate takes depends on the estate. One bank and a house is often a few months from applying to the grant; shares, a business or a dispute take longer. Ask the solicitor for the likely date in writing at the first meeting, so you have something to plan around."), py_str("Not everything needs probate. Joint accounts and anything held in trust usually pass without it, and banks often release small balances on the certificate alone. The rule of thumb is a house, or more than one bank, or a will that is not clear - then apply.")]));
    s_PAPERWORK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:solicitor|paperwork|(?:the|her|his|their) will|(?:her|his|their) (?:bank|pension|insurance|accounts)|bereavement|the estate)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ARRANGEMENTS_ELSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:wear|wearing|outfit|clothes|suit|dress|shoes|tie|music|songs?|hymns?|flowers?|wreath|bouquet|buffet|sandwiches|catering|reception|wake|drink)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FAMILY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:brother|sister|dad|mum|stepdad|stepmum|uncle|aunt|nan|grandad|family|relatives|in-laws|siblings)\\b[^.]{0,40}?\\b(?:argue|arguing|argument|row|rows|rowing|fight|fighting|falling out|fell out|not talking|won'?t talk)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TELL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:who (?:do|should|must|would) (?:i|we) (?:have to |need to |ought to )?(?:tell|notify|inform|ring|contact)|who (?:needs? to know|needs? to be told|has to know|should (?:know|be told))|who to (?:tell|notify|inform)|(?:do|should) (?:i|we) (?:have to |need to )?(?:tell|notify|inform|ring) (?:everyone|everybody|anyone|people|the (?:council|bank|pension|tax office|dwp|hmrc|authorities|employer))|tell us once|who do i inform)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TELLING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow do i (?:tell|say|break)\\b|\\bhow to tell\\b|\\b(?:do|should) i tell\\b|\\bwhat (?:do|should|can) i tell\\b|\\bhow do i let .{0,20} know\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAY_TO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:do|should|can) i say\\b|\\bwhat to say\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bereavement leave|compassionate leave|leave from work|(?:ask|tell|ring|call|email) (?:my|the) (?:boss|manager|work|employer|hr)\\b|\\btell work\\b|\\bwhat do i tell work\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORK_WORD(py_str("\\b(?:work|job|boss|manager|office|shift|shifts|colleagues?|team|employer|hr)\\b"));
    s_WORK_PRESSURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?:(?=.*"), &g_WORK_WORD()), &py_str(").*?\\b(?:back to work|off work|cant face|can't face|cant go back|can't go back|dread|too much|(?:keep|keeps|are) asking|everyone (?:keeps )?asking)\\b|\\b(?:people|everyone|folks|the team) at work\\b)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ADMIN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:paperwork|the papers|(?:a|any|one|the|that) forms?|filling in|fill in|(?:his|her|their|the) (?:bank|banks|accounts?|pension|pensions|insurance|policies|policy|passport|driving licence|licence|mobile|phone contract|utilities|bills?|mortgage|tenancy|landlord|affairs)|council tax|(?:his|her|their) (?:benefits?|tax credits)|universal credit|hmrc|the dwp|the tax office|dvla|the council|(?:cancel|close|stop|sort out|sorted out|sorted|deal with|notify|inform|tell)\\b[^.]{0,36}?\\b(?:subscription\\w*|direct debits?|accounts?|pension|insurance|policy|doctor'?s (?:surgery|practice)|dentist|the vet|employer|the council|the bank|the tax office|his phone|her phone|the phone|the house|the mortgage|the tenancy)\\b)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ADMIN_REPLIES(py_tuple(vec![py_str("None of it has to be done this week, and it goes faster by phone than by letter. The bank, the pension and the council each have a bereavement team: ring one a day, ask them to stop the letters, and write the reference number down on a single sheet as you go."), py_str("One call does most of it: Tell Us Once tells the tax office, the council, the passport office and the DVLA in one go, and the register office sets it up when the death is registered. The bank and any pension are their own two calls."), py_str("Do it in the order it hurts least: the bank first, because the account keeps running, then the pension, then the bills. As next of kin you can do all of it on their behalf, and you don't need probate to tell anybody anything.")]));
    s_PRACTICAL(py_tuple(vec![g_TELL(), g_TELLING(), g_SAY_TO(), g_WORK_PRESSURE(), g_WORK(), g_REGISTER(), g_COST(), g_PROBATE(), g_ADMIN(), g_PAPERWORK(), g_ARRANGEMENTS(), g_BELONGINGS()]));
    s_OTHER_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bmy (?:baby|boy|girl|son|daughter|kid|child|toddler|teenager|husband|wife)\\b|\\b\\d{1,2} year old\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EMERGENCY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:wont wake|cant wake|can'?t wake|unresponsive|not breathing|ambulance|999|911|chest pain|a and e|a&e|temperature|floppy|seizure|overdose|suicidal|bleeding|collapsed)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUDDEN_REPLIES(py_tuple(vec![py_str("Sudden is the hardest kind — nothing gets you ready for it. It comes in waves for weeks, so don't be surprised when a good hour turns into a bad one."), py_str("That kind of shock takes ages to settle, and there's no doing it faster. Go as slowly as you can, and let people do the small things for you.")]));
    s_EXPECTED_REPLIES(py_tuple(vec![py_str("Even when you know it's coming, it still lands hard — knowing and being ready are two different things. There's nothing wrong with being knocked sideways by it."), py_str("Expected doesn't make it easier, it just means you spent a long time watching it happen. That takes it out of you too, so rest when you can.")]));
    s_SLEEP_REPLIES(py_tuple(vec![py_str("Sleep is the first thing to go with this. Don't lie there fighting it: get up, warm drink, something dull, and go back when you're sleepy."), py_str("Hardly sleeping is the usual pattern after a loss, and it does ease. Keep the room cool and dark, get up rather than lie there, and catch up in the day if you can.")]));
    s_GUILT_REPLIES(py_tuple(vec![py_str("That guilt is a very common part of this, and it is almost never deserved. If it helps, say out loud what you'd want {who} to say to you now."), py_str("Everyone finds something to blame themselves for afterwards. {Who} would not want you carrying that.")]));
    s_HARD_REPLIES(py_tuple(vec![py_str("It is, and there's no getting round how hard it is. Do less today than you think you should, and let people bring you things."), py_str("It's allowed to be that hard. Nothing about the first days is normal, so don't measure yourself against anyone else getting through it.")]));
    s_TALK_ABOUT_REPLIES(py_tuple(vec![py_str("You don't have to. Say as much or as little as you want, and I'm here for the parts that aren't words."), py_str("That's alright. Nothing has to be said here - tell me when you want to, and not before.")]));
    s_CRYING_REPLIES(py_tuple(vec![py_str("Crying is the pressure valve, not a setback. Let it come, then get outside for ten minutes if you can."), py_str("That's normal, and better out than held in. It gets further apart, not gone.")]));
    s_MISSING_REPLIES(py_tuple(vec![py_str("That's a big miss, and it doesn't fade quickly. The stories keep {who} close, so go on telling them."), py_str("The dates are the hard ones — the birthday, Christmas, the day itself. Doing one small thing for {who}, a photo out or a meal they liked, can take the edge off.")]));
    s_CANT_TALK_REPLIES(py_tuple(vec![py_str("That's alright, you don't have to — not now, not with me. I'm here whenever it helps, and we can talk about anything else in the meantime."), py_str("Then we won't. Say as much or as little as you want, and we can leave {who} out of it for now."), py_str("Fair enough. Nothing to explain, and no rush: tell me about something ordinary instead if that's easier today.")]));
    s_SIGNS_REPLIES(py_tuple(vec![py_str("That's one of the strangest parts — the house keeps the shape of {who}. It eases on its own, slowly, so don't talk yourself out of it."), py_str("Keep going if it helps. Saying goodnight, or talking out loud, is how a lot of people get through the first year, and it isn't daft.")]));
    s_AGE_REPLIES(py_tuple(vec![py_str("That's a long time together. It's the routines that catch you out more than the big days: the same chair at the same time, nobody in it."), py_str("A good long life still leaves a big hole. Be as sad as you need to be about it.")]));
    s_PET_OLD_REPLIES(py_tuple(vec![py_str("{Age} is a good age for a {animal}, and it still isn't enough. Be as sad as you need to be about {who}."), py_str("That's a long life for a {animal}. It doesn't make the house any quieter, and it isn't meant to.")]));
    s_PET_YOUNG_REPLIES(py_tuple(vec![py_str("{Age} is no age for a {animal}. Nothing about it is fair, and it's alright to say so out loud."), py_str("{Who} should have had years more than that. It was far too soon, and there is no sense to be made of it.")]));
    s_YOUNG_AGE_REPLIES(py_tuple(vec![py_str("That's far too young, and there is no sense to be made of it. You don't have to try tonight."), py_str("{Who} should have had years more than that. Nothing about it is fair, and it's alright to say so out loud.")]));
    s_BELONGINGS_REPLIES(py_tuple(vec![py_str("Do a little at a time, and don't do it alone: one drawer with someone there, then stop. Nothing has to go this week."), py_str("You don't have to sort {who}'s things to be doing it right. Pick one small thing, give the rest a few months, and keep anything you're not sure about.")]));
    s_REGISTER_REPLIES(py_tuple(vec![py_str("The death is registered at the local register office, usually within five days. The hospital or the funeral director tells you which one; take the medical certificate, and get several copies of the death certificate while you're there."), py_str("You do that at the register office for the area where it happened. Book first, take the medical certificate, and buy a few extra certificates — every bank and pension will want one.")]));
    s_COST_REPLIES(py_tuple(vec![py_str("Ask two funeral directors for a written, itemised quote — the coffin and the cars are where it adds up, and a cremation with no service costs a fraction of a full funeral. Nothing has to be decided on the first call."), py_str("Say what you actually want before they price it: a simple coffin, no cars, family carrying it. If money is tight there's a funeral expenses payment from the DWP, and the council will help as a last resort.")]));
    s_ARRANGEMENTS_REPLIES(py_tuple(vec![py_str("Ring the funeral director first — they do the rest of it. Take someone with you to the meeting, and ask about costs up front."), py_str("For the day itself: be early, sit near the back if you might need air, and don't feel you have to talk to everyone.")]));
    s_PAPERWORK_REPLIES(py_tuple(vec![py_str("The bank and the council want the death certificate, and most places have a bereavement line that does the telling once. Ask them to do that rather than ringing round yourself."), py_str("Probate is only needed if there's a house, savings over the limit or shares. A solicitor will do a free first call, and it can wait a few weeks.")]));
    s_FAMILY_REPLIES(py_tuple(vec![py_str("That happens in nearly every family, and it's the grief talking as much as anything — everyone is at a different speed. Split the jobs so you're not doing the same one: you take the house, they take the paperwork."), py_str("It's worth naming it out loud, once, when nobody's tired: \"we're going to snap at each other and that's fine, but let's not do it over the house\". Then give each other room.")]));
    s_TELLING_REPLIES(py_tuple(vec![py_str("Keep it plain, and say the name: \"{who} died on Tuesday\" is kinder than a soft way round it. Then let them talk, and don't be afraid to say you don't know what to say either."), py_str("Say it once, plainly, and then be quiet with them for a bit. People remember being told kindly far longer than the exact words.")]));
    s_SAY_TO_REPLIES(py_tuple(vec![py_str("You don't need the right words. Ask about {who}, and let them talk as long as they want; that does more than anything you could say."), py_str("Say the true things: that you're sorry, that you miss {who} too, and that you're around. Then follow their lead - if they want to laugh about something, laugh."), py_str("Ask what the day has been like, and take the answer however it comes. Not filling the silence is a kindness in itself.")]));
    s_TELL_REPLIES(py_tuple(vec![py_str("Tell Us Once does most of it in one go — the passport, driving licence, HMRC and the council. The bank and any pension want their own call, and their bereavement line is the quickest way in."), py_str("Start with the people who need to act this week: the employer, the bank, the landlord if there is one. Everything else can wait until you've had a few days.")]));
    s_COMPANY_REPLIES(py_tuple(vec![py_str("I'm here. Say as much or as little as you like about {who}."), py_str("No need to be all right about it with me. If you'd rather talk about something else for a while, that's fine too.")]));
    s_DOUBT_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and |so |ok(?:ay)?,? )?(?:is (?:that|this|it) (?:bad|ok(?:ay)?|all right|alright|wrong|weird|odd|a bad sign)|is that how it (?:goes and works|goes|works))(?: then)?[\\s?.!]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DOUBT_ME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and |so |ok(?:ay)?,? )?(?:am i (?:ok(?:ay)?|all right|alright|normal|going mad|going crazy|losing it|losing my mind)|does that make me (?:weird|mad|crazy))(?: then)?[\\s?.!]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DOUBT_IT_REPLIES(py_tuple(vec![py_str("It is not a bad sign, and there is nothing here you have to fix today. The part worth keeping an eye on is eating and sleeping."), py_str("Not that, no. Grief is not an illness and it does not run to a timetable - the waves get further apart, they do not stop."), py_str("That is grief doing its work, not a weakness. It comes in waves, and the waves get further apart as time goes on - they do not stop."), py_str("It eases slowly, and it does not go in a straight line - a better day is not a setback.")]));
    s_DOUBT_ME_REPLIES(py_tuple(vec![py_str("Nothing about that is wrong with you. Grief is not an illness and it does not run to a timetable - the waves get further apart, they do not stop."), py_str("No, you are not losing it. This is what a death does to a person, and it comes in waves rather than a straight line down."), py_str("You are allowed to be like this. If you are not sleeping or eating at all after a couple of weeks, that is the point to say it to your GP."), py_str("It is not something to be good at. If it helps, the thing to hold on to is that a bad hour does not undo a good one.")]));
    s_OPEN_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so |and |ok(?:ay)?,? )?(?:(?:what|where) (?:do|should|can|could|shall) (?:i|we) (?:do|start|begin|go from here|take on)(?: now| next| first| today)?|what now|now what|what happens now|what next|where do i (?:start|begin)(?: from here)?|how do i (?:get through|deal with|cope with) (?:this|today|it|the day)|(?:i |we )?(?:don'?t|dont|do not) know what to (?:do|say)(?: now| next| first| today)?|(?:i |we )?(?:don'?t|dont|do not) know where to (?:start|begin))(?:\\s+then)?\\s*[.?!]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OPEN_ASK_REPLIES(py_tuple(vec![py_str("Nothing big, today. Eat something, drink some water, and let one person who knows be with you. The rest can wait until tomorrow."), py_str("Keep it small. One phone call, one meal, and say yes to anybody who offers to help - nobody expects you to hold it all at once."), py_str("You do not have to work anything out today. Take the offers that come, and leave the decisions until you have slept."), py_str("When you have a quiet hour, write down the things only you know about {who}. The admin can wait a week; the small details are the ones that go."), py_str("Pick the one thing that would make tomorrow easier, do that, and let the rest be someone else's job for now."), py_str("Slow is fine. Do the next necessary thing, and if there is someone who can sit with you, ask them to.")]));
    s_THANKS_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ok(?:ay)?[,.!]?\\s*)?(?:(?:thanks|thank you|thankyou|ta)(?:\\s+(?:for|so much|very much)\\s*(?:listening|talking|that|the chat|your help)?)?|(?:im|i'm|i am)\\s+(?:ok|okay|fine|alright)(?:\\s+now)?|ok(?:ay)?)[.!, ]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LOSS_SPEAKERS(py_tuple(vec![py_str("loss"), py_str("grief")]));
    s_PERSON_ONLY(py_tuple(vec![g_REGISTER(), g_COST(), g_PROBATE(), g_PAPERWORK(), g_ARRANGEMENTS(), g_TELL(), g_ADMIN()]));
    s_ANIMAL_PAPERWORK_REPLIES(py_tuple(vec![py_str("None of that applies to {who}: nothing gets registered and there is no estate to deal with. Ring the vet who saw them - they arrange the cremation or the burial, and they will tell you what it costs."), py_str("A pet leaves no estate and no paperwork. The vet is the one who arranges the cremation or the burial, and they will give you the options and the price in one call."), py_str("There is nothing official to do for {who}. Tell the vet you would like them to arrange it, and ask about the ashes and the cost while you are on the phone.")]));
    s_ASK_OPENERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("how"), py_str("what"), py_str("who"), py_str("when"), py_str("where"), py_str("which"), py_str("should"), py_str("can"), py_str("do"), py_str("does"), py_str("is"), py_str("are"), py_str("could"), py_str("would"), py_str("any"), py_str("am")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PET_LOSS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:die[ds]?|dying|death|dead|passed away|put (?:down|to sleep)|euthan\\w*|cremat\\w*|ashes|urn|casket|coffin|burial|bur(?:y|ied|ies)|body|remains|paw print|lock of fur|last (?:visit|day)|goodbye)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn defers_to(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let _ = py_str("True when the pets entry is the right answer to this line, so it keeps the turn.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::a_pet_died(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::asks_about_the_remains(args))(&__args) } } }))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::names_a_feeling(args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::feeling_about_the_body(args))(&__args) } } };
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The line as words: lower case, apostrophes flattened, punctuation off the ends.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'()")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'()")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    Py::None
}
pub fn _who(args: &[Py]) -> Py {
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
    let mut animal: Py = Py::None;
    let mut singular: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(the one who died, said back as the user would — \"your nan\" — and whether it is a pet),\n    or (None, False) when the line names neither.");
    animal = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::the_animal(args))(&__args) };
    if py_truthy(&animal.clone()) {
    return py_tuple(vec![op_add(&py_str("your "), &animal.clone()), Py::Bool(true)]);
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    singular = if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&gen_compose::compose::loss::g_PET_WORDS(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { word.clone() };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&gen_compose::compose::loss::g_PEOPLE(), &singular.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&gen_compose::compose::loss::g_PET_WORDS(), &singular.clone())) } }) {
    return py_tuple(vec![op_add(&py_str("your "), &singular.clone()), Py::Bool(py_contains(&gen_compose::compose::loss::g_PET_WORDS(), &singular.clone()))]);
    }
    }
    return py_tuple(vec![Py::None, Py::Bool(false)]);
    Py::None
}
pub fn _record(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Write the loss this turn is about, so the turns after it are read in it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) })) } }) {
    return Py::None;
    }
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::reads_as_loss(args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::whose(args))(&__args) }, &py_str("own")))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _who(args))(&__args) }, 2, None);
    who = __unpk1[0].clone();
    pet = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&memory.clone(), &py_str("loss"), py_dict(vec![(py_str("who"), who.clone()), (py_str("pet"), pet.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("ahead"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::is_dying_now(args))(&__args) })]));
    Py::None
}
pub fn _still_here(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the turn before this one was about the loss (either speaker), so a thanks\n    belongs here and not to the small-talk module.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_LOSS_SPEAKERS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn _next_to_the_loss(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut record: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "record") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: record"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut previous: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the loss is the thing in hand, not just somewhere in the thread.\n\n    A short doubt (\"am i ok\", \"is that bad\") is about the loss only when the loss is what was\n    just being talked about: either the last reply was this module's own, or the line before the\n    doubt reads as grief itself. A promotion, a broken boiler or a child's temperature said\n    between a death and \"is that bad\" is that subject's turn - the pools would otherwise read\n    every doubt in a bereaved thread as a doubt about grief, which is how \"my dad died last week\"\n    / \"my boiler is broken\" / \"is that bad\" got the waves of grief.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(record.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("replied")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) }, &op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_LOSS_SPEAKERS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(true);
    }
    previous = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&previous.clone()))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::reads_as_loss(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FAMILY(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GUILT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CRYING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SLEEP(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(previous.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HARD(), "search", args))(&__args) } } } } } } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _remembered(args: &[Py]) -> Py {
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
    let mut record: Py = Py::None;
    let _ = py_str("The loss the last few turns were about, or None when it is older than that.");
    record = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("loss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(record.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) }, &op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_REMEMBER()))) {
    return Py::None;
    }
    return record.clone();
    Py::None
}
pub fn _age(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The number the line gives, or None.");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AGE_NUMBER(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, &py_str("0")) } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _animal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The animal the loss is about, or None when it is a person.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ANIMAL_SPAN()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b"))); __args.push({ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return word.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _any_animal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut animal: Py = Py::None;
    let _ = py_str("The animal, named or generic: \"my pet died\" is a pet as much as \"my dog died\" is.");
    animal = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _animal(args))(&__args) };
    if py_truthy(&animal.clone()) {
    return animal.clone();
    }
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:your )?pets?\\b")); __args.push({ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { py_str("pet") } else { Py::None };
    Py::None
}
pub fn _young(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { py_str("") };
    let mut animal: Py = Py::None;
    let mut number: Py = Py::None;
    let _ = py_str("Whether the age in the line is a life cut short rather than a long one.\n\n    Against the animal's own span for a pet, and against a person's for a person.\n    ");
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _age(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&number.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    animal = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _animal(args))(&__args) };
    if py_truthy(&animal.clone()) {
    return op_lt(&number.clone(), &op_mul(&geti(&g_ANIMAL_SPAN(), &animal.clone()), &Py::Float(0.8)));
    }
    return op_lt(&number.clone(), &g_SHORT_LIFE());
    Py::None
}
pub fn _age_pool(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut animal: Py = Py::None;
    let mut filled: Py = Py::None;
    let mut number: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The pool the age asks for, filled with the number and the animal the loss is about.");
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _age(args))(&__args) };
    animal = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _animal(args))(&__args) };
    pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _young(args))(&__args) }) { g_YOUNG_AGE_REPLIES() } else { g_AGE_REPLIES() };
    if py_truthy(&animal.clone()) {
    pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _young(args))(&__args) }) { g_PET_YOUNG_REPLIES() } else { g_PET_OLD_REPLIES() };
    }
    filled = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pool.clone());
    for __item in __iter1 {
    reply = __item.clone();
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Who}")); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&who.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&who.clone(), &Py::Int(1), &Py::None, &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{who}")); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Age}")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _word(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{animal}")); __args.push({ let __b = animal.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&filled.clone(), "append", args))(&__args) };
    }
    return filled.clone();
    Py::None
}
pub fn _word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut words: Py = Py::None;
    let _ = py_str("The age as a word, because \"14 is a good age\" reads like a form.");
    words = py_dict(vec![(Py::Int(1), py_str("One")), (Py::Int(2), py_str("Two")), (Py::Int(3), py_str("Three")), (Py::Int(4), py_str("Four")), (Py::Int(5), py_str("Five")), (Py::Int(6), py_str("Six")), (Py::Int(7), py_str("Seven")), (Py::Int(8), py_str("Eight")), (Py::Int(9), py_str("Nine")), (Py::Int(10), py_str("Ten")), (Py::Int(11), py_str("Eleven")), (Py::Int(12), py_str("Twelve")), (Py::Int(13), py_str("Thirteen")), (Py::Int(14), py_str("Fourteen")), (Py::Int(15), py_str("Fifteen")), (Py::Int(16), py_str("Sixteen")), (Py::Int(17), py_str("Seventeen")), (Py::Int(18), py_str("Eighteen")), (Py::Int(19), py_str("Nineteen")), (Py::Int(20), py_str("Twenty"))]);
    if py_truthy(&Py::Bool(py_is(&number.clone(), &Py::None))) {
    return py_str("That");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "get", args))(&__args) };
    Py::None
}
pub fn _fill(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut reply: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut reply: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{Who}")); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&who.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&who.clone(), &Py::Int(1), &Py::None, &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{who}")); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }); } py_list(__out) })() };
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut doubt: Py = Py::None;
    let mut next_to_the_loss: Py = Py::None;
    let mut p: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut practical: Py = Py::None;
    let mut record: Py = Py::None;
    let mut who: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Answer a short remark inside a recent loss, naming the person; else None.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_SHORT()) } }) {
    return Py::None;
    }
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OTHER_PERSON(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EMERGENCY(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_knowledge::knowledge::lookup::a_colour_is_worn(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&gen_compose::compose::loss::g_FUNERAL(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::a_part_of_the_day_asked(args))(&__args) } } }) {
    return Py::None;
    }
    record = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _remembered(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&record.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::dying_in_the_chat(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&gen_compose::compose::loss::g_SAY_GOODBYE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TELLING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAY_TO(), "search", args))(&__args) } } } } } } }) {
    return Py::None;
    }
    pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SLEEP(), "search", args))(&__args) }) { Py::None } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _composer_pool(args))(&__args) } };
    if py_truthy(&Py::Bool(!py_is(&pool.clone(), &Py::None))) {
    practical = { let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ADMIN(), "search", args))(&__args) } } }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PRACTICAL()) { let mut p: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = practical.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&pool.clone(), &gen_compose::compose::loss::g_LOSS_REPLIES())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORK_PRESSURE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TELLING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TELL(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ADMIN(), "search", args))(&__args) } } } } } } } } } } } } }))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::reads_as_loss(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PRACTICAL()) { let mut p: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    who = geti(&record.clone(), &py_str("who"));
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CANT_TALK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TALK_TO_SOMEONE(), "search", args))(&__args) })) } }) {
    let _ = py_setitem(&record.clone(), &py_str("replied"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CANT_TALK_REPLIES()); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill(args))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPEN_ASK(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PRACTICAL()) { let mut p: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _answered_by_the_knowledge_base(args))(&__args) })) } } } }) {
    let _ = py_setitem(&record.clone(), &py_str("replied"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_OPEN_ASK_REPLIES()); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill(args))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    }
    next_to_the_loss = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(record.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _next_to_the_loss(args))(&__args) };
    if py_truthy(&next_to_the_loss.clone()) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_DOUBT_IT(), g_DOUBT_IT_REPLIES()]), py_tuple(vec![g_DOUBT_ME(), g_DOUBT_ME_REPLIES()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    doubt = __unpk2[0].clone();
    pool = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&doubt.clone(), "match", args))(&__args) }) {
    let _ = py_setitem(&record.clone(), &py_str("replied"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    }
    }
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_FAMILY(), g_FAMILY_REPLIES()]), py_tuple(vec![g_WORK_PRESSURE(), g_WORK_PRESSURE_REPLIES()]), py_tuple(vec![g_WORK(), g_WORK_REPLIES()]), py_tuple(vec![g_TELL(), g_TELL_REPLIES()]), py_tuple(vec![g_TELLING(), g_TELLING_REPLIES()]), py_tuple(vec![g_SAY_TO(), g_SAY_TO_REPLIES()]), py_tuple(vec![g_REGISTER(), g_REGISTER_REPLIES()]), py_tuple(vec![g_COST(), g_COST_REPLIES()]), py_tuple(vec![g_PROBATE(), g_PROBATE_REPLIES()]), py_tuple(vec![g_PAPERWORK(), g_PAPERWORK_REPLIES()]), py_tuple(vec![g_ADMIN(), g_ADMIN_REPLIES()]), py_tuple(vec![g_ARRANGEMENTS(), g_ARRANGEMENTS_REPLIES()]), py_tuple(vec![g_BELONGINGS(), g_BELONGINGS_REPLIES()]), py_tuple(vec![g_EXPECTED(), g_EXPECTED_REPLIES()]), py_tuple(vec![g_SUDDEN(), g_SUDDEN_REPLIES()]), py_tuple(vec![g_SLEEP(), g_SLEEP_REPLIES()]), py_tuple(vec![g_HARD(), g_HARD_REPLIES()]), py_tuple(vec![g_TALK_ABOUT(), g_TALK_ABOUT_REPLIES()]), py_tuple(vec![g_GUILT(), g_GUILT_REPLIES()]), py_tuple(vec![g_CRYING(), g_CRYING_REPLIES()]), py_tuple(vec![g_AGE(), Py::None]), py_tuple(vec![g_SIGNS(), g_SIGNS_REPLIES()]), py_tuple(vec![g_MISSING(), g_MISSING_REPLIES()])]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk4[0].clone();
    pool = __unpk4[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERSON_ONLY(), &pattern.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _any_animal(args))(&__args) } } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _any_animal(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_answer_says_the_animal(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    let _ = py_setitem(&record.clone(), &py_str("replied"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ANIMAL_PAPERWORK_REPLIES()); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill(args))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pattern.clone(), &g_ARRANGEMENTS())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ARRANGEMENTS_ELSE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::a_part_of_the_day_asked(args))(&__args) } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pattern.clone(), &g_ARRANGEMENTS())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EULOGY(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EULOGY_WRITE(), "search", args))(&__args) } } } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    let _ = py_setitem(&record.clone(), &py_str("replied"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &Py::None))) {
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _age_pool(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill(args))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THANKS_LINE(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _still_here(args))(&__args) } } }) {
    let _ = py_setitem(&record.clone(), &py_str("replied"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_COMPANY_REPLIES()); __args.push(who.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fill(args))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _pick(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _asks(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks something, rather than telling the bot the news.");
    return { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_OPENERS(), &geti(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0)))) } };
    Py::None
}
pub fn _composer_pool(args: &[Py]) -> Py {
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
    let _ = py_str("The pool the composer would answer this line from, or None when it has no line for it.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::loss::reply_for(args))(&__args) }]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn observe(args: &[Py]) -> Py {
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
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _record(args))(&__args) };
    Py::None
}
pub fn _the_answer_says_the_animal(args: &[Py]) -> Py {
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
    let mut animal: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut asked: Py = Py::None;
    let mut got: Py = Py::None;
    let mut low: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("True when the answer on the table for this line is about the animal.\n\n    The paperwork pools are a person's, so for an animal they stand down - but only into an\n    answer that is the pet's own. A line that names the species reaches the pet-loss card (the\n    vet, the cremation, the paw print); a bare \"where do i register the death\" inside a pet's\n    thread reaches the register office for a person, and swapping this module's register line\n    for that one helps nobody.\n    ");
    let __try1 = exc::catch(|| {
    let mut answers: Py = crate::modules::answers::module_obj();
    topic = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    topic = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&topic.clone(), "get", args))(&__args) } } else { py_str("") };
    asked = if py_truthy(&topic.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&topic.clone(), &py_str(" ")), &text.clone()), "strip", args))(&__args) } } else { text.clone() };
    got = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(asked.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("copy", "deepcopy", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answers.clone(), "respond", args))(&__args) };
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
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(got.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    got = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("reply")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&got.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = got.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&low.clone(), &py_str("vet"))); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(animal.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &animal.clone())) } } } }))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PET_LOSS(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _answered_by_the_knowledge_base(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("True when the knowledge base has a real answer for this line.\n\n    The modules are asked in order and grief comes before answers, so an open ask claimed here\n    takes away the answer the person was about to get: \"my dog is dying\" / \"where do i start\"\n    has the vet, the cremation and the paw print in it, and that is the whole turn. The answer\n    is asked for on a copy of memory so nothing is written twice.\n\n    A bare ask has no thing in it, so the thing the chat is on goes in front of it: \"my dog is\n    dying\" / \"what do i do now\" is a question about the dog, and the pet-loss entry answers it\n    once the line carries the dog. Without the join the entry is never reached and the pool of\n    lines for a loss takes a turn that had a real answer.\n    ");
    let __try1 = exc::catch(|| {
    let mut answers: Py = crate::modules::answers::module_obj();
    topic = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topic")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    topic = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&topic.clone(), "get", args))(&__args) } } else { py_str("") };
    asked = if py_truthy(&topic.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&topic.clone(), &py_str(" ")), &text.clone()), "strip", args))(&__args) } } else { text.clone() };
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(asked.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("copy", "deepcopy", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answers.clone(), "respond", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
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
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut seed_text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seed_text") { v.clone() }
    else { py_str("") };
    let _ = py_str("One of the replies, not the one the bot said last time.\n\n    `seed_text` is the person's line where the caller has it, so two people asking the same open\n    question in their own words do not get the same sentence just because both are turn two.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(seed_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }]);
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
    return geti(&options.clone(), &Py::Int(0));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.grief".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADMIN" => Some(g_ADMIN()),
            "ADMIN_REPLIES" => Some(g_ADMIN_REPLIES()),
            "AGE" => Some(g_AGE()),
            "AGE_NUMBER" => Some(g_AGE_NUMBER()),
            "AGE_REPLIES" => Some(g_AGE_REPLIES()),
            "ANIMAL_PAPERWORK_REPLIES" => Some(g_ANIMAL_PAPERWORK_REPLIES()),
            "ANIMAL_SPAN" => Some(g_ANIMAL_SPAN()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ANSWERS_INSTEAD" => Some(g_ANSWERS_INSTEAD()),
            "ARRANGEMENTS" => Some(g_ARRANGEMENTS()),
            "ARRANGEMENTS_ELSE" => Some(g_ARRANGEMENTS_ELSE()),
            "ARRANGEMENTS_REPLIES" => Some(g_ARRANGEMENTS_REPLIES()),
            "ASK_OPENERS" => Some(g_ASK_OPENERS()),
            "BELONGINGS" => Some(g_BELONGINGS()),
            "BELONGINGS_REPLIES" => Some(g_BELONGINGS_REPLIES()),
            "CANT_TALK" => Some(g_CANT_TALK()),
            "CANT_TALK_REPLIES" => Some(g_CANT_TALK_REPLIES()),
            "COMPANY_REPLIES" => Some(g_COMPANY_REPLIES()),
            "COST" => Some(g_COST()),
            "COST_REPLIES" => Some(g_COST_REPLIES()),
            "CRYING" => Some(g_CRYING()),
            "CRYING_REPLIES" => Some(g_CRYING_REPLIES()),
            "DEFERS_TO" => Some(g_DEFERS_TO()),
            "DOUBT_IT" => Some(g_DOUBT_IT()),
            "DOUBT_IT_REPLIES" => Some(g_DOUBT_IT_REPLIES()),
            "DOUBT_ME" => Some(g_DOUBT_ME()),
            "DOUBT_ME_REPLIES" => Some(g_DOUBT_ME_REPLIES()),
            "EMERGENCY" => Some(g_EMERGENCY()),
            "EULOGY" => Some(g_EULOGY()),
            "EULOGY_WRITE" => Some(g_EULOGY_WRITE()),
            "EXPECTED" => Some(g_EXPECTED()),
            "EXPECTED_REPLIES" => Some(g_EXPECTED_REPLIES()),
            "FAMILY" => Some(g_FAMILY()),
            "FAMILY_REPLIES" => Some(g_FAMILY_REPLIES()),
            "GUILT" => Some(g_GUILT()),
            "GUILT_REPLIES" => Some(g_GUILT_REPLIES()),
            "HARD" => Some(g_HARD()),
            "HARD_REPLIES" => Some(g_HARD_REPLIES()),
            "LOSS_SPEAKERS" => Some(g_LOSS_SPEAKERS()),
            "MISSING" => Some(g_MISSING()),
            "MISSING_REPLIES" => Some(g_MISSING_REPLIES()),
            "OPEN_ASK" => Some(g_OPEN_ASK()),
            "OPEN_ASK_REPLIES" => Some(g_OPEN_ASK_REPLIES()),
            "OTHER_PERSON" => Some(g_OTHER_PERSON()),
            "PAPERWORK" => Some(g_PAPERWORK()),
            "PAPERWORK_REPLIES" => Some(g_PAPERWORK_REPLIES()),
            "PERSON_ONLY" => Some(g_PERSON_ONLY()),
            "PET_LOSS" => Some(g_PET_LOSS()),
            "PET_OLD_REPLIES" => Some(g_PET_OLD_REPLIES()),
            "PET_YOUNG_REPLIES" => Some(g_PET_YOUNG_REPLIES()),
            "POSSESSIVE" => Some(g_POSSESSIVE()),
            "PRACTICAL" => Some(g_PRACTICAL()),
            "PRIORITY" => Some(g_PRIORITY()),
            "PROBATE" => Some(g_PROBATE()),
            "PROBATE_REPLIES" => Some(g_PROBATE_REPLIES()),
            "REGISTER" => Some(g_REGISTER()),
            "REGISTER_REPLIES" => Some(g_REGISTER_REPLIES()),
            "REMEMBER" => Some(g_REMEMBER()),
            "SAY_TO" => Some(g_SAY_TO()),
            "SAY_TO_REPLIES" => Some(g_SAY_TO_REPLIES()),
            "SHORT" => Some(g_SHORT()),
            "SHORT_LIFE" => Some(g_SHORT_LIFE()),
            "SIGNS" => Some(g_SIGNS()),
            "SIGNS_REPLIES" => Some(g_SIGNS_REPLIES()),
            "SLEEP" => Some(g_SLEEP()),
            "SLEEP_REPLIES" => Some(g_SLEEP_REPLIES()),
            "SUDDEN" => Some(g_SUDDEN()),
            "SUDDEN_REPLIES" => Some(g_SUDDEN_REPLIES()),
            "TALK_ABOUT" => Some(g_TALK_ABOUT()),
            "TALK_ABOUT_REPLIES" => Some(g_TALK_ABOUT_REPLIES()),
            "TALK_TO_SOMEONE" => Some(g_TALK_TO_SOMEONE()),
            "TELL" => Some(g_TELL()),
            "TELLING" => Some(g_TELLING()),
            "TELLING_REPLIES" => Some(g_TELLING_REPLIES()),
            "TELL_REPLIES" => Some(g_TELL_REPLIES()),
            "THANKS_LINE" => Some(g_THANKS_LINE()),
            "WORK" => Some(g_WORK()),
            "WORK_PRESSURE" => Some(g_WORK_PRESSURE()),
            "WORK_PRESSURE_REPLIES" => Some(g_WORK_PRESSURE_REPLIES()),
            "WORK_REPLIES" => Some(g_WORK_REPLIES()),
            "WORK_WORD" => Some(g_WORK_WORD()),
            "YOUNG_AGE_REPLIES" => Some(g_YOUNG_AGE_REPLIES()),
            "_LIVING" => Some(g__LIVING()),
            "_age" => Some(py_func("_age", |args: &[Py]| _age(args))),
            "_age_pool" => Some(py_func("_age_pool", |args: &[Py]| _age_pool(args))),
            "_animal" => Some(py_func("_animal", |args: &[Py]| _animal(args))),
            "_answered_by_the_knowledge_base" => Some(py_func("_answered_by_the_knowledge_base", |args: &[Py]| _answered_by_the_knowledge_base(args))),
            "_any_animal" => Some(py_func("_any_animal", |args: &[Py]| _any_animal(args))),
            "_asks" => Some(py_func("_asks", |args: &[Py]| _asks(args))),
            "_composer_pool" => Some(py_func("_composer_pool", |args: &[Py]| _composer_pool(args))),
            "_fill" => Some(py_func("_fill", |args: &[Py]| _fill(args))),
            "_next_to_the_loss" => Some(py_func("_next_to_the_loss", |args: &[Py]| _next_to_the_loss(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_record" => Some(py_func("_record", |args: &[Py]| _record(args))),
            "_remembered" => Some(py_func("_remembered", |args: &[Py]| _remembered(args))),
            "_still_here" => Some(py_func("_still_here", |args: &[Py]| _still_here(args))),
            "_the_answer_says_the_animal" => Some(py_func("_the_answer_says_the_animal", |args: &[Py]| _the_answer_says_the_animal(args))),
            "_who" => Some(py_func("_who", |args: &[Py]| _who(args))),
            "_word" => Some(py_func("_word", |args: &[Py]| _word(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "_young" => Some(py_func("_young", |args: &[Py]| _young(args))),
            "defers_to" => Some(py_func("defers_to", |args: &[Py]| defers_to(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADMIN" => s_ADMIN(v),
            "ADMIN_REPLIES" => s_ADMIN_REPLIES(v),
            "AGE" => s_AGE(v),
            "AGE_NUMBER" => s_AGE_NUMBER(v),
            "AGE_REPLIES" => s_AGE_REPLIES(v),
            "ANIMAL_PAPERWORK_REPLIES" => s_ANIMAL_PAPERWORK_REPLIES(v),
            "ANIMAL_SPAN" => s_ANIMAL_SPAN(v),
            "ANSWERS" => s_ANSWERS(v),
            "ANSWERS_INSTEAD" => s_ANSWERS_INSTEAD(v),
            "ARRANGEMENTS" => s_ARRANGEMENTS(v),
            "ARRANGEMENTS_ELSE" => s_ARRANGEMENTS_ELSE(v),
            "ARRANGEMENTS_REPLIES" => s_ARRANGEMENTS_REPLIES(v),
            "ASK_OPENERS" => s_ASK_OPENERS(v),
            "BELONGINGS" => s_BELONGINGS(v),
            "BELONGINGS_REPLIES" => s_BELONGINGS_REPLIES(v),
            "CANT_TALK" => s_CANT_TALK(v),
            "CANT_TALK_REPLIES" => s_CANT_TALK_REPLIES(v),
            "COMPANY_REPLIES" => s_COMPANY_REPLIES(v),
            "COST" => s_COST(v),
            "COST_REPLIES" => s_COST_REPLIES(v),
            "CRYING" => s_CRYING(v),
            "CRYING_REPLIES" => s_CRYING_REPLIES(v),
            "DEFERS_TO" => s_DEFERS_TO(v),
            "DOUBT_IT" => s_DOUBT_IT(v),
            "DOUBT_IT_REPLIES" => s_DOUBT_IT_REPLIES(v),
            "DOUBT_ME" => s_DOUBT_ME(v),
            "DOUBT_ME_REPLIES" => s_DOUBT_ME_REPLIES(v),
            "EMERGENCY" => s_EMERGENCY(v),
            "EULOGY" => s_EULOGY(v),
            "EULOGY_WRITE" => s_EULOGY_WRITE(v),
            "EXPECTED" => s_EXPECTED(v),
            "EXPECTED_REPLIES" => s_EXPECTED_REPLIES(v),
            "FAMILY" => s_FAMILY(v),
            "FAMILY_REPLIES" => s_FAMILY_REPLIES(v),
            "GUILT" => s_GUILT(v),
            "GUILT_REPLIES" => s_GUILT_REPLIES(v),
            "HARD" => s_HARD(v),
            "HARD_REPLIES" => s_HARD_REPLIES(v),
            "LOSS_SPEAKERS" => s_LOSS_SPEAKERS(v),
            "MISSING" => s_MISSING(v),
            "MISSING_REPLIES" => s_MISSING_REPLIES(v),
            "OPEN_ASK" => s_OPEN_ASK(v),
            "OPEN_ASK_REPLIES" => s_OPEN_ASK_REPLIES(v),
            "OTHER_PERSON" => s_OTHER_PERSON(v),
            "PAPERWORK" => s_PAPERWORK(v),
            "PAPERWORK_REPLIES" => s_PAPERWORK_REPLIES(v),
            "PERSON_ONLY" => s_PERSON_ONLY(v),
            "PET_LOSS" => s_PET_LOSS(v),
            "PET_OLD_REPLIES" => s_PET_OLD_REPLIES(v),
            "PET_YOUNG_REPLIES" => s_PET_YOUNG_REPLIES(v),
            "POSSESSIVE" => s_POSSESSIVE(v),
            "PRACTICAL" => s_PRACTICAL(v),
            "PRIORITY" => s_PRIORITY(v),
            "PROBATE" => s_PROBATE(v),
            "PROBATE_REPLIES" => s_PROBATE_REPLIES(v),
            "REGISTER" => s_REGISTER(v),
            "REGISTER_REPLIES" => s_REGISTER_REPLIES(v),
            "REMEMBER" => s_REMEMBER(v),
            "SAY_TO" => s_SAY_TO(v),
            "SAY_TO_REPLIES" => s_SAY_TO_REPLIES(v),
            "SHORT" => s_SHORT(v),
            "SHORT_LIFE" => s_SHORT_LIFE(v),
            "SIGNS" => s_SIGNS(v),
            "SIGNS_REPLIES" => s_SIGNS_REPLIES(v),
            "SLEEP" => s_SLEEP(v),
            "SLEEP_REPLIES" => s_SLEEP_REPLIES(v),
            "SUDDEN" => s_SUDDEN(v),
            "SUDDEN_REPLIES" => s_SUDDEN_REPLIES(v),
            "TALK_ABOUT" => s_TALK_ABOUT(v),
            "TALK_ABOUT_REPLIES" => s_TALK_ABOUT_REPLIES(v),
            "TALK_TO_SOMEONE" => s_TALK_TO_SOMEONE(v),
            "TELL" => s_TELL(v),
            "TELLING" => s_TELLING(v),
            "TELLING_REPLIES" => s_TELLING_REPLIES(v),
            "TELL_REPLIES" => s_TELL_REPLIES(v),
            "THANKS_LINE" => s_THANKS_LINE(v),
            "WORK" => s_WORK(v),
            "WORK_PRESSURE" => s_WORK_PRESSURE(v),
            "WORK_PRESSURE_REPLIES" => s_WORK_PRESSURE_REPLIES(v),
            "WORK_REPLIES" => s_WORK_REPLIES(v),
            "WORK_WORD" => s_WORK_WORD(v),
            "YOUNG_AGE_REPLIES" => s_YOUNG_AGE_REPLIES(v),
            "_LIVING" => s__LIVING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.grief", module_obj);
    pyrt::xrefs::reg_fn("modules.grief", "defers_to", py_func("defers_to", |args: &[Py]| defers_to(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_who", py_func("_who", |args: &[Py]| _who(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_record", py_func("_record", |args: &[Py]| _record(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_still_here", py_func("_still_here", |args: &[Py]| _still_here(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_next_to_the_loss", py_func("_next_to_the_loss", |args: &[Py]| _next_to_the_loss(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_remembered", py_func("_remembered", |args: &[Py]| _remembered(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_age", py_func("_age", |args: &[Py]| _age(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_animal", py_func("_animal", |args: &[Py]| _animal(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_any_animal", py_func("_any_animal", |args: &[Py]| _any_animal(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_young", py_func("_young", |args: &[Py]| _young(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_age_pool", py_func("_age_pool", |args: &[Py]| _age_pool(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_word", py_func("_word", |args: &[Py]| _word(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_fill", py_func("_fill", |args: &[Py]| _fill(args)));
    pyrt::xrefs::reg_fn("modules.grief", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_asks", py_func("_asks", |args: &[Py]| _asks(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_composer_pool", py_func("_composer_pool", |args: &[Py]| _composer_pool(args)));
    pyrt::xrefs::reg_fn("modules.grief", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_the_answer_says_the_animal", py_func("_the_answer_says_the_animal", |args: &[Py]| _the_answer_says_the_animal(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_answered_by_the_knowledge_base", py_func("_answered_by_the_knowledge_base", |args: &[Py]| _answered_by_the_knowledge_base(args)));
    pyrt::xrefs::reg_fn("modules.grief", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_access("modules.grief", "ADMIN", g_ADMIN, s_ADMIN);
    pyrt::xrefs::reg_access("modules.grief", "ADMIN_REPLIES", g_ADMIN_REPLIES, s_ADMIN_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "AGE", g_AGE, s_AGE);
    pyrt::xrefs::reg_access("modules.grief", "AGE_NUMBER", g_AGE_NUMBER, s_AGE_NUMBER);
    pyrt::xrefs::reg_access("modules.grief", "AGE_REPLIES", g_AGE_REPLIES, s_AGE_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "ANIMAL_PAPERWORK_REPLIES", g_ANIMAL_PAPERWORK_REPLIES, s_ANIMAL_PAPERWORK_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "ANIMAL_SPAN", g_ANIMAL_SPAN, s_ANIMAL_SPAN);
    pyrt::xrefs::reg_access("modules.grief", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.grief", "ANSWERS_INSTEAD", g_ANSWERS_INSTEAD, s_ANSWERS_INSTEAD);
    pyrt::xrefs::reg_access("modules.grief", "ARRANGEMENTS", g_ARRANGEMENTS, s_ARRANGEMENTS);
    pyrt::xrefs::reg_access("modules.grief", "ARRANGEMENTS_ELSE", g_ARRANGEMENTS_ELSE, s_ARRANGEMENTS_ELSE);
    pyrt::xrefs::reg_access("modules.grief", "ARRANGEMENTS_REPLIES", g_ARRANGEMENTS_REPLIES, s_ARRANGEMENTS_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "ASK_OPENERS", g_ASK_OPENERS, s_ASK_OPENERS);
    pyrt::xrefs::reg_access("modules.grief", "BELONGINGS", g_BELONGINGS, s_BELONGINGS);
    pyrt::xrefs::reg_access("modules.grief", "BELONGINGS_REPLIES", g_BELONGINGS_REPLIES, s_BELONGINGS_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "CANT_TALK", g_CANT_TALK, s_CANT_TALK);
    pyrt::xrefs::reg_access("modules.grief", "CANT_TALK_REPLIES", g_CANT_TALK_REPLIES, s_CANT_TALK_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "COMPANY_REPLIES", g_COMPANY_REPLIES, s_COMPANY_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "COST", g_COST, s_COST);
    pyrt::xrefs::reg_access("modules.grief", "COST_REPLIES", g_COST_REPLIES, s_COST_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "CRYING", g_CRYING, s_CRYING);
    pyrt::xrefs::reg_access("modules.grief", "CRYING_REPLIES", g_CRYING_REPLIES, s_CRYING_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "DEFERS_TO", g_DEFERS_TO, s_DEFERS_TO);
    pyrt::xrefs::reg_access("modules.grief", "DOUBT_IT", g_DOUBT_IT, s_DOUBT_IT);
    pyrt::xrefs::reg_access("modules.grief", "DOUBT_IT_REPLIES", g_DOUBT_IT_REPLIES, s_DOUBT_IT_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "DOUBT_ME", g_DOUBT_ME, s_DOUBT_ME);
    pyrt::xrefs::reg_access("modules.grief", "DOUBT_ME_REPLIES", g_DOUBT_ME_REPLIES, s_DOUBT_ME_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "EMERGENCY", g_EMERGENCY, s_EMERGENCY);
    pyrt::xrefs::reg_access("modules.grief", "EULOGY", g_EULOGY, s_EULOGY);
    pyrt::xrefs::reg_access("modules.grief", "EULOGY_WRITE", g_EULOGY_WRITE, s_EULOGY_WRITE);
    pyrt::xrefs::reg_access("modules.grief", "EXPECTED", g_EXPECTED, s_EXPECTED);
    pyrt::xrefs::reg_access("modules.grief", "EXPECTED_REPLIES", g_EXPECTED_REPLIES, s_EXPECTED_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "FAMILY", g_FAMILY, s_FAMILY);
    pyrt::xrefs::reg_access("modules.grief", "FAMILY_REPLIES", g_FAMILY_REPLIES, s_FAMILY_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "GUILT", g_GUILT, s_GUILT);
    pyrt::xrefs::reg_access("modules.grief", "GUILT_REPLIES", g_GUILT_REPLIES, s_GUILT_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "HARD", g_HARD, s_HARD);
    pyrt::xrefs::reg_access("modules.grief", "HARD_REPLIES", g_HARD_REPLIES, s_HARD_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "LOSS_SPEAKERS", g_LOSS_SPEAKERS, s_LOSS_SPEAKERS);
    pyrt::xrefs::reg_access("modules.grief", "MISSING", g_MISSING, s_MISSING);
    pyrt::xrefs::reg_access("modules.grief", "MISSING_REPLIES", g_MISSING_REPLIES, s_MISSING_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "OPEN_ASK", g_OPEN_ASK, s_OPEN_ASK);
    pyrt::xrefs::reg_access("modules.grief", "OPEN_ASK_REPLIES", g_OPEN_ASK_REPLIES, s_OPEN_ASK_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "OTHER_PERSON", g_OTHER_PERSON, s_OTHER_PERSON);
    pyrt::xrefs::reg_access("modules.grief", "PAPERWORK", g_PAPERWORK, s_PAPERWORK);
    pyrt::xrefs::reg_access("modules.grief", "PAPERWORK_REPLIES", g_PAPERWORK_REPLIES, s_PAPERWORK_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "PERSON_ONLY", g_PERSON_ONLY, s_PERSON_ONLY);
    pyrt::xrefs::reg_access("modules.grief", "PET_LOSS", g_PET_LOSS, s_PET_LOSS);
    pyrt::xrefs::reg_access("modules.grief", "PET_OLD_REPLIES", g_PET_OLD_REPLIES, s_PET_OLD_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "PET_YOUNG_REPLIES", g_PET_YOUNG_REPLIES, s_PET_YOUNG_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "POSSESSIVE", g_POSSESSIVE, s_POSSESSIVE);
    pyrt::xrefs::reg_access("modules.grief", "PRACTICAL", g_PRACTICAL, s_PRACTICAL);
    pyrt::xrefs::reg_access("modules.grief", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.grief", "PROBATE", g_PROBATE, s_PROBATE);
    pyrt::xrefs::reg_access("modules.grief", "PROBATE_REPLIES", g_PROBATE_REPLIES, s_PROBATE_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "REGISTER", g_REGISTER, s_REGISTER);
    pyrt::xrefs::reg_access("modules.grief", "REGISTER_REPLIES", g_REGISTER_REPLIES, s_REGISTER_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "REMEMBER", g_REMEMBER, s_REMEMBER);
    pyrt::xrefs::reg_access("modules.grief", "SAY_TO", g_SAY_TO, s_SAY_TO);
    pyrt::xrefs::reg_access("modules.grief", "SAY_TO_REPLIES", g_SAY_TO_REPLIES, s_SAY_TO_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "SHORT", g_SHORT, s_SHORT);
    pyrt::xrefs::reg_access("modules.grief", "SHORT_LIFE", g_SHORT_LIFE, s_SHORT_LIFE);
    pyrt::xrefs::reg_access("modules.grief", "SIGNS", g_SIGNS, s_SIGNS);
    pyrt::xrefs::reg_access("modules.grief", "SIGNS_REPLIES", g_SIGNS_REPLIES, s_SIGNS_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "SLEEP", g_SLEEP, s_SLEEP);
    pyrt::xrefs::reg_access("modules.grief", "SLEEP_REPLIES", g_SLEEP_REPLIES, s_SLEEP_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "SUDDEN", g_SUDDEN, s_SUDDEN);
    pyrt::xrefs::reg_access("modules.grief", "SUDDEN_REPLIES", g_SUDDEN_REPLIES, s_SUDDEN_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "TALK_ABOUT", g_TALK_ABOUT, s_TALK_ABOUT);
    pyrt::xrefs::reg_access("modules.grief", "TALK_ABOUT_REPLIES", g_TALK_ABOUT_REPLIES, s_TALK_ABOUT_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "TALK_TO_SOMEONE", g_TALK_TO_SOMEONE, s_TALK_TO_SOMEONE);
    pyrt::xrefs::reg_access("modules.grief", "TELL", g_TELL, s_TELL);
    pyrt::xrefs::reg_access("modules.grief", "TELLING", g_TELLING, s_TELLING);
    pyrt::xrefs::reg_access("modules.grief", "TELLING_REPLIES", g_TELLING_REPLIES, s_TELLING_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "TELL_REPLIES", g_TELL_REPLIES, s_TELL_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "THANKS_LINE", g_THANKS_LINE, s_THANKS_LINE);
    pyrt::xrefs::reg_access("modules.grief", "WORK", g_WORK, s_WORK);
    pyrt::xrefs::reg_access("modules.grief", "WORK_PRESSURE", g_WORK_PRESSURE, s_WORK_PRESSURE);
    pyrt::xrefs::reg_access("modules.grief", "WORK_PRESSURE_REPLIES", g_WORK_PRESSURE_REPLIES, s_WORK_PRESSURE_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "WORK_REPLIES", g_WORK_REPLIES, s_WORK_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "WORK_WORD", g_WORK_WORD, s_WORK_WORD);
    pyrt::xrefs::reg_access("modules.grief", "YOUNG_AGE_REPLIES", g_YOUNG_AGE_REPLIES, s_YOUNG_AGE_REPLIES);
    pyrt::xrefs::reg_access("modules.grief", "_LIVING", g__LIVING, s__LIVING);
}
