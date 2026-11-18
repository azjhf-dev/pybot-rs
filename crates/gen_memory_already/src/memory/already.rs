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


thread_local! { static G_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_APOLOGY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_OUTCOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOLOGY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOLOGY_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIFE_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STEP_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_COMPLETION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED_NOTHING_BROKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED_OF_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTACT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DID_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DID_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_THAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINISHED_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINISHED_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINISHED_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_LINE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IRREGULAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEPT_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_PADDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOBODY_TO_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_STEP_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_SAID_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_ADJECTIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_A_JOB_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_A_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_GOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_NAMED_JOB_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_NAMED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_NEVER_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_NOT_A_THING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_NO_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_STYLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OVER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIRED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIRED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_TO_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_AS_PRESENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_APOLOGY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLED_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEPS_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TO_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLAUSE_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEGATION_STEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_there_is_a_chat: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.already: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ALREADY() -> Py { ensure_init(); G_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY(v: Py) { ensure_init(); G_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_APOLOGY() -> Py { ensure_init(); G_AN_APOLOGY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_APOLOGY(v: Py) { ensure_init(); G_AN_APOLOGY.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_OUTCOME() -> Py { ensure_init(); G_AN_OUTCOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_OUTCOME(v: Py) { ensure_init(); G_AN_OUTCOME.with(|c| *c.borrow_mut() = v); }
pub fn g_APOLOGY() -> Py { ensure_init(); G_APOLOGY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOLOGY(v: Py) { ensure_init(); G_APOLOGY.with(|c| *c.borrow_mut() = v); }
pub fn g_APOLOGY_STEP() -> Py { ensure_init(); G_APOLOGY_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOLOGY_STEP(v: Py) { ensure_init(); G_APOLOGY_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIFE_OBJECT() -> Py { ensure_init(); G_A_LIFE_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIFE_OBJECT(v: Py) { ensure_init(); G_A_LIFE_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STEP_TAKEN() -> Py { ensure_init(); G_A_STEP_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STEP_TAKEN(v: Py) { ensure_init(); G_A_STEP_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_COMPLETION() -> Py { ensure_init(); G_BARE_COMPLETION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_COMPLETION(v: Py) { ensure_init(); G_BARE_COMPLETION.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_DONE() -> Py { ensure_init(); G_BARE_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_DONE(v: Py) { ensure_init(); G_BARE_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED() -> Py { ensure_init(); G_CHANGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED(v: Py) { ensure_init(); G_CHANGED.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED_NOTHING_BROKEN() -> Py { ensure_init(); G_CHANGED_NOTHING_BROKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED_NOTHING_BROKEN(v: Py) { ensure_init(); G_CHANGED_NOTHING_BROKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED_OF_A_THING() -> Py { ensure_init(); G_CHANGED_OF_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED_OF_A_THING(v: Py) { ensure_init(); G_CHANGED_OF_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED_REPLIES() -> Py { ensure_init(); G_CHANGED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED_REPLIES(v: Py) { ensure_init(); G_CHANGED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_START() -> Py { ensure_init(); G_CONDITION_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_START(v: Py) { ensure_init(); G_CONDITION_START.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTACT() -> Py { ensure_init(); G_CONTACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTACT(v: Py) { ensure_init(); G_CONTACT.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTACT_REPLIES() -> Py { ensure_init(); G_CONTACT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTACT_REPLIES(v: Py) { ensure_init(); G_CONTACT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_DID() -> Py { ensure_init(); G_DID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DID(v: Py) { ensure_init(); G_DID.with(|c| *c.borrow_mut() = v); }
pub fn g_DID_STEPS() -> Py { ensure_init(); G_DID_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DID_STEPS(v: Py) { ensure_init(); G_DID_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_DID_TAIL() -> Py { ensure_init(); G_DID_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DID_TAIL(v: Py) { ensure_init(); G_DID_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_THAT() -> Py { ensure_init(); G_DONE_THAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_THAT(v: Py) { ensure_init(); G_DONE_THAT.with(|c| *c.borrow_mut() = v); }
pub fn g_FINISHED_ACT() -> Py { ensure_init(); G_FINISHED_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINISHED_ACT(v: Py) { ensure_init(); G_FINISHED_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g_FINISHED_ITSELF() -> Py { ensure_init(); G_FINISHED_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINISHED_ITSELF(v: Py) { ensure_init(); G_FINISHED_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_FINISHED_STEPS() -> Py { ensure_init(); G_FINISHED_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINISHED_STEPS(v: Py) { ensure_init(); G_FINISHED_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_LINE_REPLIES() -> Py { ensure_init(); G_FIRST_LINE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_LINE_REPLIES(v: Py) { ensure_init(); G_FIRST_LINE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_HARD() -> Py { ensure_init(); G_HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HARD(v: Py) { ensure_init(); G_HARD.with(|c| *c.borrow_mut() = v); }
pub fn g_IRREGULAR() -> Py { ensure_init(); G_IRREGULAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IRREGULAR(v: Py) { ensure_init(); G_IRREGULAR.with(|c| *c.borrow_mut() = v); }
pub fn g_KEPT_UP() -> Py { ensure_init(); G_KEPT_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEPT_UP(v: Py) { ensure_init(); G_KEPT_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_PADDING() -> Py { ensure_init(); G_MORE_PADDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_PADDING(v: Py) { ensure_init(); G_MORE_PADDING.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_STEPS() -> Py { ensure_init(); G_MOST_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_STEPS(v: Py) { ensure_init(); G_MOST_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOBODY_TO_ANSWER() -> Py { ensure_init(); G_NOBODY_TO_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOBODY_TO_ANSWER(v: Py) { ensure_init(); G_NOBODY_TO_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_STEP_START() -> Py { ensure_init(); G_NOT_A_STEP_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STEP_START(v: Py) { ensure_init(); G_NOT_A_STEP_START.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_SAID_BACK() -> Py { ensure_init(); G_NOT_A_THING_SAID_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_SAID_BACK(v: Py) { ensure_init(); G_NOT_A_THING_SAID_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_WORD() -> Py { ensure_init(); G_NOT_A_THING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_WORD(v: Py) { ensure_init(); G_NOT_A_THING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_ADJECTIVE() -> Py { ensure_init(); G_OVER_ADJECTIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_ADJECTIVE(v: Py) { ensure_init(); G_OVER_ADJECTIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_ASKING() -> Py { ensure_init(); G_OVER_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_ASKING(v: Py) { ensure_init(); G_OVER_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_A_JOB_DONE() -> Py { ensure_init(); G_OVER_A_JOB_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_A_JOB_DONE(v: Py) { ensure_init(); G_OVER_A_JOB_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_A_MEAL() -> Py { ensure_init(); G_OVER_A_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_A_MEAL(v: Py) { ensure_init(); G_OVER_A_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_BETWEEN() -> Py { ensure_init(); G_OVER_BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_BETWEEN(v: Py) { ensure_init(); G_OVER_BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_GOT() -> Py { ensure_init(); G_OVER_GOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_GOT(v: Py) { ensure_init(); G_OVER_GOT.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_LEAD() -> Py { ensure_init(); G_OVER_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_LEAD(v: Py) { ensure_init(); G_OVER_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_NAMED_JOB_REPLIES() -> Py { ensure_init(); G_OVER_NAMED_JOB_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_NAMED_JOB_REPLIES(v: Py) { ensure_init(); G_OVER_NAMED_JOB_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_NAMED_REPLIES() -> Py { ensure_init(); G_OVER_NAMED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_NAMED_REPLIES(v: Py) { ensure_init(); G_OVER_NAMED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_NEVER_A_THING() -> Py { ensure_init(); G_OVER_NEVER_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_NEVER_A_THING(v: Py) { ensure_init(); G_OVER_NEVER_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_NOT() -> Py { ensure_init(); G_OVER_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_NOT(v: Py) { ensure_init(); G_OVER_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_NOT_A_THING_NAMED() -> Py { ensure_init(); G_OVER_NOT_A_THING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_NOT_A_THING_NAMED(v: Py) { ensure_init(); G_OVER_NOT_A_THING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_NO_THING() -> Py { ensure_init(); G_OVER_NO_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_NO_THING(v: Py) { ensure_init(); G_OVER_NO_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_PEOPLE() -> Py { ensure_init(); G_OVER_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_PEOPLE(v: Py) { ensure_init(); G_OVER_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_REPLIES() -> Py { ensure_init(); G_OVER_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_REPLIES(v: Py) { ensure_init(); G_OVER_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_STATE() -> Py { ensure_init(); G_OVER_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_STATE(v: Py) { ensure_init(); G_OVER_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_STOP() -> Py { ensure_init(); G_OVER_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_STOP(v: Py) { ensure_init(); G_OVER_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_STYLED() -> Py { ensure_init(); G_OVER_STYLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_STYLED(v: Py) { ensure_init(); G_OVER_STYLED.with(|c| *c.borrow_mut() = v); }
pub fn g_OVER_WORD() -> Py { ensure_init(); G_OVER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OVER_WORD(v: Py) { ensure_init(); G_OVER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST() -> Py { ensure_init(); G_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST(v: Py) { ensure_init(); G_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_AT() -> Py { ensure_init(); G_POINTS_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_AT(v: Py) { ensure_init(); G_POINTS_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIRED() -> Py { ensure_init(); G_REPAIRED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIRED(v: Py) { ensure_init(); G_REPAIRED.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIRED_REPLIES() -> Py { ensure_init(); G_REPAIRED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIRED_REPLIES(v: Py) { ensure_init(); G_REPAIRED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_FILLER() -> Py { ensure_init(); G_REPORT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_FILLER(v: Py) { ensure_init(); G_REPORT_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_REPLIES() -> Py { ensure_init(); G_REPORT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_REPLIES(v: Py) { ensure_init(); G_REPORT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_TO_ME() -> Py { ensure_init(); G_SAID_TO_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_TO_ME(v: Py) { ensure_init(); G_SAID_TO_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_AS_PRESENT() -> Py { ensure_init(); G_SAME_AS_PRESENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_AS_PRESENT(v: Py) { ensure_init(); G_SAME_AS_PRESENT.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_APOLOGY() -> Py { ensure_init(); G_SAYS_APOLOGY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_APOLOGY(v: Py) { ensure_init(); G_SAYS_APOLOGY.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLED() -> Py { ensure_init(); G_SETTLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLED(v: Py) { ensure_init(); G_SETTLED.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLED_REPLIES() -> Py { ensure_init(); G_SETTLED_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLED_REPLIES(v: Py) { ensure_init(); G_SETTLED_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_STEPS_LEADS() -> Py { ensure_init(); G_STEPS_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEPS_LEADS(v: Py) { ensure_init(); G_STEPS_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT() -> Py { ensure_init(); G_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT(v: Py) { ensure_init(); G_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_TELLING() -> Py { ensure_init(); G_TELLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELLING(v: Py) { ensure_init(); G_TELLING.with(|c| *c.borrow_mut() = v); }
pub fn g_TO_THE_BOT() -> Py { ensure_init(); G_TO_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TO_THE_BOT(v: Py) { ensure_init(); G_TO_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAILING() -> Py { ensure_init(); G_TRAILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAILING(v: Py) { ensure_init(); G_TRAILING.with(|c| *c.borrow_mut() = v); }
pub fn g__CLAUSE_BREAK() -> Py { ensure_init(); G__CLAUSE_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLAUSE_BREAK(v: Py) { ensure_init(); G__CLAUSE_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g__NEGATION_STEM() -> Py { ensure_init(); G__NEGATION_STEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEGATION_STEM(v: Py) { ensure_init(); G__NEGATION_STEM.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_there_is_a_chat() -> Py { ensure_init(); G_there_is_a_chat.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_there_is_a_chat(v: Py) { ensure_init(); G_there_is_a_chat.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that says the step was already taken is answered with the next turn of the thread.\n\n\"someone used my card online\" / \"i already called the bank\" is the most useful thing a person\ncan say in an advice thread: they did the first thing, and what they want now is what comes\nafter it. Nothing read the line, so the store kept the words and the composer answered the\nshape instead of the person: \"Got it. Is there anything you'd like a hand with?\", and a bare\n\"i already called the plumber\" got \"I'm listening - say more if it helps.\", which is what\nSTYLE.md calls the therapist pattern - a line that would fit any message at all.\n\nSo this reads the report and answers the next turn of the thread. Which reply that is depends\non the answer behind it:\n\n* where the answer still has a step after theirs, that step is the reply, read from the answer\n  itself: \"turn the water off at the isolating valve under the sink, take the handle off, and\n  take the old part to a hardware shop\" is answered with \"take the handle off, and take the old\n  part to a hardware shop\". A reply of our own there would be one sentence behind the card and\n  would ask about a result that cannot exist yet - turning the water off cannot make a dripping\n  tap better, and bleeding the radiators does not stop a boiler losing pressure;\n* where the answer has nothing left to say, the report gets the one thing a friend asks next:\n  how it went. That question is named by the kind of step, because the question that follows a\n  call is not the question that follows a repair - \"what did they say?\" against \"is it any\n  better since?\".\n\nThe rule is narrow on purpose:\n* the line has to say \"already\" and then a step that was taken, as a past-tense verb\n  (\"called\", \"tried\", \"bled\", \"turned\"), so \"i already have a dog\" - a fact about their\n  house - is not a report on anything and is left to the store;\n* the line must not ask anything of its own (\"i already bled the radiators what next\" wants\n  the next step, not a question about this one), and it must be short: a report, not a story;\n* a hard thread is left alone: after a death, \"i already told my mum\" is about the person who\n  died, and a cheerful \"what did they say?\" would be grotesque.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_there_is_a_chat(py_func("there_is_a_chat", |args: &[Py]| pyrt::xrefs::xcall("memory", "there_is_a_chat", args)));
    s_FIRST_LINE_REPLIES(pyrt::xrefs::xget("memory.warmth", "FIRST_LINE_REPLIES"));
    s_SUBJECT(py_str("(?:i|we|ive|i've|weve|we've|i have|we have|i had|we had)"));
    s_ALREADY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:(?:ok|okay|so|well|yeah|yep|yes|and|but|oh|right|alright|no)\\W+)*(?:"), &g_SUBJECT()), &py_str("\\W+)?already\\W+(\\w+)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DID(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|so|well|yeah|yep|yes|and|but|oh|right|alright|no)\\W+)*(?:i|we|ive|i've|weve|we've|i have|we have)\\W+did\\W+(\\w+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DID_STEPS(call_builtin("frozenset", &[mcall(&py_str("call ring phone email text message speak talk ask tell contact write\nsend pay go see make take put book order buy fix bleed clean reset turn change check clear\ntighten replace restart unplug soak scrub defrost hoover paint apply cut start stop give\ncomplain report cancel register fill submit"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DID_TAIL(call_builtin("frozenset", &[mcall(&py_str("that this it them him her me us the a an one ones thing stuff though tho\nalthough again already twice once too well just still even really very much more some any all\nand but so oh ok okay yeah yep yes no not yet since yesterday today tomorrow morning afternoon\nevening last night week month year now then before after first both either\noff out on up down around round through over back away"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRAILING(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|so|well|yeah|yep|yes|and|but|oh|right|alright|no)\\W+)*(?:i|we|ive|i've|weve|we've)\\W+(\\w+)\\b.*\\balready\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST(call_builtin("frozenset", &[mcall(&py_str("called rang phoned emailed texted messaged spoke talked asked told contacted\nwrote sent paid went seen saw made took put booked ordered bought tried fixed bled\ncleaned reset turned changed checked cleared tightened replaced restarted unplugged soaked\nscrubbed defrosted hoovered painted applied cut started stopped gave complained reported\ncancelled registered filled submitted"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTACT(call_builtin("frozenset", &[mcall(&py_str("called rang phoned emailed texted messaged spoke talked asked told\ncontacted wrote complained reported"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOBODY_TO_ANSWER(call_builtin("frozenset", &[mcall(&py_str("booked ordered registered submitted cancelled filled sent\npaid"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_APOLOGY(stdlib_call("re", "compile", &[py_str("\\b(?:sorry|apolog\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIRED(call_builtin("frozenset", &[mcall(&py_str("fixed bled cleaned reset turned checked cleared tightened replaced\nrestarted unplugged soaked scrubbed defrosted hoovered painted applied"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHANGED(call_builtin("frozenset", &[mcall(&py_str("tried started stopped gave paid ordered bought made took put went\nchanged cut"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHANGED_OF_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("changed")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LIFE_OBJECT(call_builtin("frozenset", &[mcall(&py_str("job jobs shift shifts hours routine routines plan plans mind diet\nschool course habit habits career way ways approach lifestyle life body sleep mood name names\naddress phone number provider tariff deal supplier gym class club doctor dentist"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHANGED_NOTHING_BROKEN(call_builtin("frozenset", &[mcall(&py_str("baby babies boy girl kid kids child children little one\nhim her them me you us everyone everybody nappy nappies pad pads sheet sheets bedding bed cot\nquilt duvet pillow pillowcase towel towels flannel bag bags bin bins water loo toilet roll tea\ndog cat puppy kitten pup hamster rabbit guinea budgie bird birds fish pet pets"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|what's|how|when|where|which|who|why|should|shall|can|could|will|would|may|might|do|does|did|is|are|any|anything)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TO_THE_BOT(stdlib_call("re", "compile", &[py_str("\\b(?:you|u|your|yours|youse)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_WORDS(Py::Int(9));
    s_SAID_TO_ME(call_builtin("frozenset", &[py_tuple(vec![py_str("answered"), py_str("asked"), py_str("discussed"), py_str("explained"), py_str("mentioned"), py_str("said"), py_str("talked"), py_str("told")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HARD(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|death|passed away|funeral|burial|cremation|miscarriage|stillborn|hospital|hospice|cancer|diagnosis|suicide)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTACT_REPLIES(py_tuple(vec![py_str("Then it's with them now - what did they say?"), py_str("Right. Did they give you an answer?"), py_str("Fair enough. What did they say?"), py_str("Good. Have they said anything back yet?")]));
    s_OVER_REPLIES(py_tuple(vec![py_str("Good - that's the job done. Did somebody come out in the end, or did you sort it yourself?"), py_str("That's a relief. What was it in the end?"), py_str("Good to hear it. Is it holding, or is it one of those that comes back?"), py_str("Right - so that's out of the way. What did it take?")]));
    s_OVER_A_JOB_DONE(call_builtin("frozenset", &[py_tuple(vec![py_str("done"), py_str("finished"), py_str("sorted")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_NAMED_REPLIES(py_tuple(vec![py_str("Good to hear it. Is the {thing} holding, or is it one of those that comes back?"), py_str("Good - that is the {thing} sorted then. What was it in the end?"), py_str("That's a relief. What was wrong with the {thing}, in the end?"), py_str("Right - so the {thing} is out of the way. Did it take long?")]));
    s_OVER_NAMED_JOB_REPLIES(py_tuple(vec![py_str("Right - the {thing} done then. What was the fiddly part?"), py_str("Good. How long did the {thing} take?"), py_str("That's the {thing} out of the way. Was it as big a job as it looked?"), py_str("Nice one. What was the {thing} like to do?")]));
    s_REPAIRED_REPLIES(py_tuple(vec![py_str("Then that's the first thing done. Is it any better since?"), py_str("Right - has it made a difference?"), py_str("Good. If it's still the same tomorrow, say and we'll go a step further."), py_str("Then give it a day and tell me if it settles.")]));
    s_CHANGED_REPLIES(py_tuple(vec![py_str("Right - has it made any difference yet?"), py_str("Early days, then - what has changed so far?"), py_str("Good. How are you finding it?")]));
    s_REPORT_REPLIES(py_tuple(vec![py_str("Right - how did that go?"), py_str("Good. What happened when you did?"), py_str("Then what came of it?")]));
    s_SETTLED_REPLIES(py_tuple(vec![py_str("Good — that's that sorted, then."), py_str("Nice one. One less thing on the list."), py_str("Good. That is that one off your mind.")]));
    s_FINISHED_ACT(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|so|well|yeah|yep|yes|and|but|oh|right|alright|no)\\W+)*(?:i|we|ive|i've|weve|we've|i have|we have)\\W+(?:just|already|finally|now|nearly)?\\W*(\\w+)\\W+(?:it|them|one|that|this|everything|the lot|all of it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FINISHED_ITSELF(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|so|well|yeah|yep|yes|and|but|oh|right|alright|no)\\W+)*(?:it|its|it's|that|thats|that's|all)\\W+(?:is\\s+|was\\s+|were\\s+|got\\s+|gets\\s+|now\\s+)?(done|sorted|fixed|finished|working|mended|booked|ordered|paid|collected|replaced)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_DONE(stdlib_call("re", "compile", &[py_str("^\\W*(?:all\\s+)?(done|sorted|finished|fixed)\\W*(?:,?\\s*(?:thanks|ta))?\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_THAT(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|so|well|yeah|yep|yes|and|but|oh|right|alright)\\W+)*(done|sorted|fixed|finished|tried)\\W+(?:that|this|it|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FINISHED_STEPS(call_builtin("frozenset", &[mcall(&py_str("\n    fixed done finished sorted mended repaired replaced cleaned painted hoovered tidied\n    found bought ordered booked returned collected installed cleared changed\n    washed rang called emailed texted paid sent submitted filled cancelled cancelled\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SETTLED(call_builtin("frozenset", &[mcall(&py_str("\n    done finished sorted found bought ordered booked returned collected sent submitted\n    filled cancelled canceled paid\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_COMPLETION(call_builtin("frozenset", &[py_tuple(vec![py_str("done"), py_str("finished")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MORE_PADDING(op_bitor(&g_DID_TAIL(), &call_builtin("frozenset", &[mcall(&py_str("thanks ta cheers brilliant great good nice lovely\nperfect out off up properly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_KEPT_UP(stdlib_call("re", "compile", &[py_str("\\b(?:keep\\s+(?:walking|going|doing|at\\s+it|it\\s+up|an?\\s+eye)|carry\\s+on|every\\s+day|each\\s+day|twice\\s+a\\s+(?:day|week)|a\\s+day|a\\s+week|stick\\s+(?:with|to)|a\\s+habit|one\\s+day\\s+at\\s+a\\s+time)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TELLING(call_builtin("frozenset", &[py_tuple(vec![py_str("told"), py_str("said"), py_str("spoken"), py_str("texted"), py_str("messaged"), py_str("written"), py_str("apologised")])]).unwrap_or_else(|e| exc::raise(e)));
    s_APOLOGY_STEP(py_str("apologise"));
    s_APOLOGY(stdlib_call("re", "compile", &[py_str("\\b(?:apolog\\w*|sorry)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_AT(call_builtin("frozenset", &[mcall(&py_str("it its it's them that this these those one ones him her all everything\n    both lot"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_dict(vec![(py_str("bled"), py_str("bleed")), (py_str("tried"), py_str("try")), (py_str("rang"), py_str("ring")), (py_str("told"), py_str("tell")), (py_str("paid"), py_str("pay")), (py_str("made"), py_str("make")), (py_str("took"), py_str("take")), (py_str("went"), py_str("go")), (py_str("saw"), py_str("see")), (py_str("spoke"), py_str("speak")), (py_str("wrote"), py_str("write")), (py_str("sent"), py_str("send")), (py_str("bought"), py_str("buy")), (py_str("gave"), py_str("give")), (py_str("put"), py_str("put")), (py_str("ran"), py_str("run")), (py_str("came"), py_str("come")), (py_str("did"), py_str("do")), (py_str("fed"), py_str("feed")), (py_str("led"), py_str("lead")), (py_str("set"), py_str("set")), (py_str("cut"), py_str("cut")), (py_str("read"), py_str("read")), (py_str("hit"), py_str("hit"))]) }
    s_IRREGULAR(__data1());
    s_A_STEP_TAKEN(op_sub(&call_builtin("frozenset", &[g_IRREGULAR()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[py_tuple(vec![py_str("did"), py_str("said")])]).unwrap_or_else(|e| exc::raise(e))));
    s_SAME_AS_PRESENT(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_IRREGULAR(), "items", &[])) { let __unpk1937 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk1937[0].clone(); let mut now: Py = __unpk1937[1].clone(); if py_truthy(&Py::Bool(py_eq(&word.clone(), &now.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_OUTCOME(call_builtin("frozenset", &[mcall(&py_str("fixed mended repaired sorted done finished"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_WORD(stdlib_call("re", "compile", &[py_str("\\b(fixed|mended|repaired|sorted|done|working|works|up\\s+and\\s+running)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|'s|been|got|now|still|its|it's)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_GOT(call_builtin("frozenset", &[py_tuple(vec![py_str("got")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_PEOPLE(call_builtin("frozenset", &[mcall(&py_str("mates friends everyone everybody people kids children family folks\nlads girls boys others us them they we i life lives"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__NEGATION_STEM(py_str("(?:is|are|was|were|do|does|did|have|has|had|wo|ca|ai|nee|must|could|would|should)"));
    s_OVER_NOT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:not|never|nothing|none|until|till|til|before|when|if|unless|once|yet|should|need|needs|want|wants)\\b|\\b"), &g__NEGATION_STEM()), &py_str("n'?t\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_ADJECTIVE(call_builtin("frozenset", &[mcall(&py_str("rate rates price prices term terms fee fees cost costs charge\ncharges nights night hours hours salary contract deal amount penalty budget rent appointment\ntime day week month year in up point order place"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_NOT_A_THING_NAMED(call_builtin("frozenset", &[mcall(&py_str("match matches game games race races fight fight vote votes\nelection elections draw draw lottery raffle bet bets trial trials contest competition fixture\ncompetitions tournament score result results odds price prices"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_A_MEAL(call_builtin("frozenset", &[mcall(&py_str("dinner tea lunch breakfast supper brunch food meal meals roast cake\ncakes bread pastry curry stew soup pizza"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_STYLED(call_builtin("frozenset", &[mcall(&py_str("hair hairs nails nail lashes brows eyebrows makeup make-up face\nbeard fringe colour color dye highlights"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_LEAD(call_builtin("frozenset", &[mcall(&py_str("my our your his her their its it's the a an this that these those both\nwhole old new little big other same own"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_NO_THING(call_builtin("frozenset", &[mcall(&py_str("i we he she they you me us them it its it's that thats that's this\neverything all nothing something anything someone somebody"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_BETWEEN(call_builtin("frozenset", &[mcall(&py_str("is are was were s been got gets get now still finally again just\nat last always never"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_STOP(call_builtin("frozenset", &[mcall(&py_str("and but so because after when if until since although though while\nthan then also plus with"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING_SAID_BACK(stdlib_call("re", "compile", &[py_str("^(?:s?he'?s|i'?m|i'?ve|they'?re|they'?ve|you'?re|you'?ve|we'?re|we'?ve|it'?s|that'?s|there'?s|theres|whats|what'?s|we|us|there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_NEVER_A_THING(call_builtin("frozenset", &[mcall(&py_str("is are was were be been being am do does did have has had will\nwould can cant could shall should may might must not never isnt arent wasnt werent dont doesnt\ndidnt havent hasnt hadnt wont wouldnt couldnt shouldnt mustnt aint"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OVER_ASKING(stdlib_call("re", "compile", &[py_str("^\\W*(?:is|are|was|were|do|does|did|has|have|had|will|would|shall|can|could|should|may|might|when|what|whats|what's|how|why|where|who|which|if|isnt|isn't|wasnt|wasn't)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CLAUSE_BREAK(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("[,;:\\u2014\\u2013]|\\b(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_OVER_STOP()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEPS_LEADS(py_tuple(vec![py_str("Then {said}."), py_str("Right - {said}."), py_str("Then that part is out of the way: {said}."), py_str("Next: {said}.")]));
    s_NOT_A_THING_WORD(call_builtin("frozenset", &[mcall(&py_str("already i ive we ve have had the a an it them that this those these\nand but so oh ok okay well yeah yep no nah right alright then my our your his her their of to in\non at for with off out up down just still even really very much more some any all"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_FILLER(call_builtin("frozenset", &[mcall(&py_str("did done do does though tho although again twice once back away around\nround through over really actually honestly literally anyway anyways too also even just still\nyet since today yesterday tomorrow morning afternoon evening night last week month year now\nthen before after first both either one ones thing stuff much many lot bits bit little quite\nrather sort kind suppose guess think know said says told tell mean meant"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_STEPS(Py::Int(4));
    s_NOT_A_STEP_START(call_builtin("frozenset", &[mcall(&py_str("less though although but and or so plus with without unless\nwhereas while which that who also still both either neither than then-as-i"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_START(stdlib_call("re", "compile", &[py_str("^(?:if|when|once|should)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_APOLOGY(stdlib_call("re", "compile", &[py_str("\\b(?:apolog\\w*|say sorry|says sorry|say you'?re sorry|i'?m sorry|i am sorry|sorry for)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_step_after_did(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut match_: Py = Py::None;
    let mut step: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The step in a line that says it with \"did\": \"i did ring them though, twice\", or None.\n\n    The word after \"did\" is in the base form (\"ring\", not \"rang\"), so it is read against the\n    steps this module knows rather than against a past tense: only a step of `DID_STEPS` is\n    claimed here, and what follows it has to be one of the little words a report is padded with,\n    so \"i did ring the council about the bins\" keeps its own turn for the entries to read.\n    ");
    match_ = mcall(&g_DID(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    step = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&Py::Bool(!py_contains(&g_DID_STEPS(), &step.clone()))) {
    return Py::None;
    }
    tail = stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&low.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_DID_TAIL(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return step.clone();
    Py::None
}
pub fn _a_crisis_line_said(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the bot's own last words in this stretch named a crisis line.");
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(2)), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_hard_thread(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the conversation is about a death or an illness, where this reader stands down.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = mcall(&g_HARD(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { mcall(&g_HARD(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut match_: Py = Py::None;
    let mut step: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut thing_after: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The step the line says was already taken, or None.\n\n    Only the word after \"already\" decides that it is a step: a past-tense verb, and not one of\n    the ones that only ever says what somebody has (\"got\", \"had\", \"been\"), because \"i already\n    got one\" is a fact about their house and not a report on anything the bot said.\n    ");
    match_ = { let __b = mcall(&g_ALREADY(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_TRAILING(), "search", &[low.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    thing_after = Py::Bool(py_is(&gattr(&match_.clone(), "re"), &g_ALREADY()));
    step = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TELLING(), &step.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_APOLOGY(), "search", &[low.clone()]) } }) {
    return g_APOLOGY_STEP();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_PAST(), &step.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&step.clone(), "endswith", &[py_str("ed")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_STEP_TAKEN(), &step.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SAME_AS_PRESENT(), &step.clone())); if !py_truthy(&__b) { __b } else { thing_after.clone() } }) {
    tail = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), slice_or_raise(&low.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_POINTS_AT(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&tail.clone()))) {
    return Py::None;
    }
    }
    return step.clone();
    Py::None
}
pub fn _nothing_more(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the job is all the line says, so the report has no second half to answer.");
    tail = stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&low.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_MORE_PADDING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_job_with_an_end(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut first: Py = Py::None;
    let _ = py_str("True when the answer's first sentence hands over a job that finishes, not one kept up.");
    first = geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    return Py::Bool(!py_truthy(&mcall(&g_KEPT_UP(), "search", &[first.clone()])));
    Py::None
}
pub fn _the_finished_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The step a report of a job finished names, or None.\n\n    The same reading as `_the_step` for the line that says the job is over without the word\n    \"already\": \"ive fixed it\" names the fix, \"ive found one\" the search, and \"its done\" says the\n    whole of it is over without naming a step at all, which is why it is read as \"done\".\n    ");
    step = Py::None;
    match_ = mcall(&g_FINISHED_ACT(), "search", &[low.clone()]);
    if py_truthy(&match_.clone()) {
    found = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&g_FINISHED_STEPS(), &found.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&found.clone(), &py_str("did"))) } }; if !py_truthy(&__b) { __b } else { _nothing_more(&[low.clone(), match_.clone()]) } }) {
    step = if py_truthy(&Py::Bool(py_eq(&found.clone(), &py_str("did")))) { py_str("done") } else { found.clone() };
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    match_ = { let __b = mcall(&g_FINISHED_ITSELF(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BARE_DONE(), "match", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DONE_THAT(), "match", &[low.clone()]) } } } };
    if py_truthy(&match_.clone()) {
    if py_truthy(&Py::Bool(!py_truthy(&_nothing_more(&[low.clone(), match_.clone()])))) {
    return Py::None;
    }
    step = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BARE_COMPLETION(), &step.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_job_with_an_end(&[_the_last_answer(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])]))) } }) {
    return Py::None;
    }
    return step.clone();
    Py::None
}
pub fn the_step_they_took(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut over: Py = Py::None;
    let mut step: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The word for the step the person says they already took, or None.\n\n    Published so the store and the composer can tell this report from a fact about their house\n    with one reading, the same way the other readers in this package are shared.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    words = mcall(&low.clone(), "split", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS())) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_TO_THE_BOT(), "search", &[low.clone()])) {
    return Py::None;
    }
    over = the_trouble_is_over(&[low.clone()]);
    if py_truthy(&over.clone()) {
    if py_truthy(&_a_hard_thread(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])) {
    return Py::None;
    }
    return over.clone();
    }
    if py_truthy(&{ let __b = mcall(&g_ASKING(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DID(), "search", &[low.clone()]))) } }) {
    return Py::None;
    }
    step = { let __b = _the_step(&[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = _the_step_after_did(&[low.clone()]); if py_truthy(&__b) { __b } else { _the_finished_step(&[low.clone(), memory.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return Py::None;
    }
    if py_truthy(&_a_hard_thread(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])) {
    return Py::None;
    }
    return step.clone();
    Py::None
}
pub fn _the_thing_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut bare: Py = Py::None;
    let mut head: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing named in front of the word at `at`, or None when the line names no thing.\n\n    \"my boiler is fixed\" names the boiler, \"the washing machine is repaired\" the washing machine,\n    and \"its sorted now\" names nothing at all. Only a thing, not a person or an event: the classes\n    in the frozensets above read the word as something else entirely.\n    ");
    words = mcall(&slice_or_raise(&low.clone(), &Py::None, &at.clone(), &Py::None), "split", &[]);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OVER_BETWEEN(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!'")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OVER_STOP(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!'")]))) } } } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    thing = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    raw = __item.clone();
    bare = mcall(&raw.clone(), "strip", &[py_str(".,!'")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OVER_NO_THING(), &bare.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OVER_LEAD(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OVER_STOP(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OVER_BETWEEN(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OVER_NEVER_A_THING(), &bare.clone())) } } } } } }) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g_OVER_NO_THING(), &bare.clone()))) {
    return Py::None;
    }
    let _ = mcall(&thing.clone(), "append", &[bare.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    thing = mcall(&py_str(" "), "join", &[call_builtin("reversed", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = mcall(&g_NOT_A_THING_SAID_BACK(), "match", &[thing.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&py_str(""), "join", &[mcall(&thing.clone(), "split", &[])]), "isdigit", &[]) } }) {
    return Py::None;
    }
    head = geti(&mcall(&thing.clone(), "split", &[]), &op_neg(&Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OVER_NOT_A_THING_NAMED(), &head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OVER_A_MEAL(), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OVER_STYLED(), &head.clone())) } } } }) {
    return Py::Bool(false);
    }
    return thing.clone();
    Py::None
}
pub fn the_thing_that_is_mended(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut low: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing a line says is fixed, or None. Published so a caller can say it back.");
    let __unpk1 = py_unpack(&_the_over_word(&[text.clone()]), 3, None);
    word = __unpk1[0].clone();
    at = __unpk1[1].clone();
    low = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::None;
    }
    thing = _the_thing_said(&[low.clone(), at.clone()]);
    return { let __b = thing.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _the_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut end: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The clause that holds the character at `at`, cut at the joins either side of it.\n\n    A negation stands a report down only in the clause that carries the state word. Left\n    standing down the whole line, \"the sink is sorted, wasnt it\" and \"the boiler is fixed now,\n    dont worry\" lost the report of the trouble being over entirely and came back as the search\n    hedge and the boiler card.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::Int(0), call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    start = __unpk1[0].clone();
    end = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g__CLAUSE_BREAK(), "finditer", &[low.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    if py_truthy(&op_le(&mcall(&match_.clone(), "end", &[]), &at.clone())) {
    start = mcall(&match_.clone(), "end", &[]);
    } else {
    if py_truthy(&op_ge(&mcall(&match_.clone(), "start", &[]), &at.clone())) {
    end = mcall(&match_.clone(), "start", &[]);
    break;
    }
    }
    }
    return slice_or_raise(&low.clone(), &start.clone(), &end.clone(), &Py::None);
    Py::None
}
pub fn _the_over_word(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut first: Py = Py::None;
    let mut i: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut near: Py = Py::None;
    let mut w: Py = Py::None;
    let mut window: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The accepted word, where it starts, and the line, or (None, -1, \"\").");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&low.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } } } }) {
    return py_tuple(vec![Py::None, op_neg(&Py::Int(1)), low.clone()]);
    }
    if py_truthy(&mcall(&g_OVER_ASKING(), "search", &[low.clone()])) {
    return py_tuple(vec![Py::None, op_neg(&Py::Int(1)), low.clone()]);
    }
    words = mcall(&low.clone(), "split", &[]);
    let __iter1 = py_iter_or_raise(&mcall(&g_OVER_WORD(), "finditer", &[low.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    word = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    at = call_builtin("len", &[mcall(&slice_or_raise(&low.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    after = if py_truthy(&op_lt(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &op_add(&at.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_OVER_ADJECTIVE(), &mcall(&after.clone(), "strip", &[py_str(".,!")])))) {
    continue;
    }
    near = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&at.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None)) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!'")])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&near.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OVER_GOT(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    first = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[near.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk3249 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk3249[0].clone(); let mut w: Py = __unpk3249[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_OVER_GOT(), &w.clone()))) {  __out.push(i.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    near = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[near.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk4017 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk4017[0].clone(); let mut w: Py = __unpk4017[1].clone(); if py_truthy(&{ let __b = op_ge(&i.clone(), &first.clone()); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_OVER_PEOPLE(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    }
    window = mcall(&py_str(" "), "join", &[near.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&near.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_OVER_PEOPLE(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&mcall(&g_OVER_NOT(), "search", &[_the_clause(&[low.clone(), mcall(&match_.clone(), "start", &[])])])) {
    continue;
    }
    if py_truthy(&mcall(&g_OVER_STATE(), "search", &[window.clone()])) {
    if py_truthy(&Py::Bool(py_is(&_the_thing_said(&[low.clone(), mcall(&match_.clone(), "start", &[])]), &Py::Bool(false)))) {
    continue;
    }
    return py_tuple(vec![word.clone(), mcall(&match_.clone(), "start", &[]), low.clone()]);
    }
    }
    return py_tuple(vec![Py::None, op_neg(&Py::Int(1)), low.clone()]);
    Py::None
}
pub fn the_trouble_is_over(args: &[Py]) -> Py {
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
    let _ = py_str("The word that says the trouble is over, or None.\n\n    Only the state of a thing reads here, and only when the line is a report: short, no question,\n    nothing asked of the trouble (\"can it be fixed\", \"how long until it is fixed\" are the asks the\n    entries answer), and nothing left over of it (\"still not fixed\", \"the landlord hasnt fixed it\").\n    ");
    return geti(&_the_over_word(&[text.clone()]), &Py::Int(0));
    Py::None
}
pub fn _gave_a_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut verdict: Py = Py::None;
    let _ = py_str("True when the answer handed a move over, whatever the move was.\n\n    The composer's own reading of an answer's first instruction, because the question here is\n    only whether there was a step at all: a report of the outcome is about that step, and an\n    answer that gave none is a new ask the entries should answer.\n    ");
    let __try1 = exc::catch(|| {
    verdict = pyrt::xrefs::xmodule("compose.verdict");
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&verdict.clone(), "_an_instruction_sentence", &[answer.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn _the_opening_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut char: Py = Py::None;
    let mut index: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The first sentence of an answer, which is where the step it hands over is named.\n\n    A later sentence can use the same word in another sense: \"my cat has gone missing\" tells you\n    to \"leave something that smells of home and water at the door ... kneel down and let it come\n    to you\", and \"come\" there is the cat coming, not a step the person has taken. Read over the\n    whole answer it made \"i already came home\" a report on the bot's step.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    char = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str(".!?"), &char.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&text.clone(), &op_add(&index.clone(), &Py::Int(1))), &py_str(" "))) } } } }) {
    return slice_or_raise(&text.clone(), &Py::None, &op_add(&index.clone(), &Py::Int(1)), &Py::None);
    }
    }
    return text.clone();
    Py::None
}
pub fn _says_the_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut other: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the last answer is where the person got the step they have already taken.");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&step.clone(), &g_APOLOGY_STEP())); if py_truthy(&__b) { __b } else { mcall(&step.clone(), "startswith", &[py_str("apolog")]) } }) {
    return call_builtin("bool", &[mcall(&g_SAYS_APOLOGY(), "search", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(py_contains(&g_AN_OUTCOME(), &step.clone()))) {
    return _gave_a_step(&[answer.clone()]);
    }
    word = mcall(&g_IRREGULAR(), "get", &[step.clone(), step.clone()]);
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ed")]) } }) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    } else {
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("e")]) } }) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    } else {
    stem = word.clone();
    }
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    other = __item.clone();
    if py_truthy(&{ let __b = mcall(&other.clone(), "startswith", &[stem.clone()]); if py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[other.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let mut step: Py = Py::None;
    let _ = py_str("True when this report should be answered as it comes, whatever else proposed.\n\n    A report that the step was already taken is not an ask, and the entry that gave the step\n    reads it as the same question again: \"my boiler is making a noise\" / \"i already bled the\n    radiators\" got the bleeding instructions a second time. So the line is claimed - but only\n    when the step is one the last answer itself named. Every other report is a new ask (\"my\n    landlord wont fix the heating\" / \"ive already rung the council about it\" is still the\n    council's answer to give) and is left to the entries.\n    ");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&reply_for(&[text.clone(), memory.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    step = the_step_they_took(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    if py_truthy(&the_trouble_is_over(&[text.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    return py_tuple(vec![Py::Bool(true), _says_the_step(&[step.clone(), _the_last_answer(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])])]);
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
pub fn _the_steps_after(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { py_str("") };
    let mut _u: Py = Py::None;
    let mut a_limit: Py = Py::None;
    let mut after: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut in_condition: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut score: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut steps: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The answer's steps after the one they took, in order, or an empty list.\n\n    The step they took is found by the words they used rather than by its verb alone: \"i already\n    turned the heating off\" holds \"turned\", and the answer's own \"a quarter turn at a time\" holds\n    it too, so the verb alone picks a step they have not done. The thing and the state (\"water\",\n    \"off\") say which step it was.\n    ");
    a_limit = pyrt::xrefs::xmodule("memory.limit");
    let __try1 = exc::catch(|| {
    steps = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&a_limit.clone(), "_steps", &[answer.clone()])) { let __unpk5701 = py_unpack(&__item.clone(), 2, None); let mut one: Py = __unpk5701[0].clone(); let mut _u: Py = __unpk5701[1].clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
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
    return py_list(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&steps.clone()))) {
    return py_list(vec![]);
    }
    word = mcall(&mcall(&g_IRREGULAR(), "get", &[step.clone(), step.clone()]), "lower", &[]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ed")]) } }) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    } else {
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("e")]) } }) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    } else {
    stem = word.clone();
    }
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut other: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NOT_A_THING_WORD(), &other.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_REPORT_FILLER(), &other.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&other.clone(), "startswith", &[stem.clone()]))); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } } } } } }) {  __out.push(other.clone()); } } py_set(__out) })() };
    let __unpk6 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0)]), 2, None);
    best = __unpk6[0].clone();
    best_score = __unpk6[1].clone();
    let __iter7 = py_iter_or_raise(&call_builtin("enumerate", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter7 {
    let __unpk8 = py_unpack(&__item.clone(), 2, None);
    index = __unpk8[0].clone();
    sentence = __unpk8[1].clone();
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&sentence.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    score = call_builtin("len", &[op_bitand(&words.clone(), &wanted.clone())]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&mcall(&sentence.clone(), "lower", &[]), &stem.clone()))) {
    let __aug9 = op_add(&score.clone(), &Py::Int(1));
    score = __aug9;
    }
    if py_truthy(&op_gt(&score.clone(), &best_score.clone())) {
    let __unpk10 = py_unpack(&py_tuple(vec![index.clone(), score.clone()]), 2, None);
    best = __unpk10[0].clone();
    best_score = __unpk10[1].clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = wanted.clone(); if !py_truthy(&__b) { __b } else { op_lt(&best_score.clone(), &Py::Int(2)) } } } }) {
    return py_list(vec![]);
    }
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&steps.clone(), &op_add(&best.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); __out.push(_the_instruction_of(&[mcall(&mcall(&one.clone(), "strip", &[]), "rstrip", &[py_str(" .,;")])])); } py_list(__out) })() };
    kept = py_list(vec![]);
    in_condition = Py::Bool(false);
    let __iter11 = py_iter_or_raise(&call_builtin("enumerate", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter11 {
    let __unpk12 = py_unpack(&__item.clone(), 2, None);
    index = __unpk12[0].clone();
    one = __unpk12[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&one.clone()))) {
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_STEP_START(), &mcall(&mcall(&geti(&mcall(&one.clone(), "split", &[]), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",:")]))) } }) {
    break;
    }
    if py_truthy(&mcall(&slice_or_raise(&one.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) {
    if py_truthy(&{ let __b = in_condition.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CONDITION_START(), "match", &[one.clone()]))) } }) {
    break;
    }
    in_condition = Py::Bool(true);
    }
    let _ = mcall(&kept.clone(), "append", &[one.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_STEPS())) {
    break;
    }
    }
    return kept.clone();
    Py::None
}
pub fn _the_instruction_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut colon: Py = Py::None;
    let mut head: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("A step without the explanation hung on the end of it.\n\n    The steps are cut from the answer at its own punctuation, and an instruction often carries\n    its reason after a colon: \"ask for an affordable payment plan: most councils will spread the\n    arrears over the rest of the year\". Handing that back says the whole paragraph again - the\n    person asked what to do next, not why - so the step is the part before the colon, and the\n    reason stays in the answer they were already given.\n    ");
    let __unpk1 = py_unpack(&mcall(&call_builtin("str", &[{ let __b = step.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "partition", &[py_str(":")]), 3, None);
    head = __unpk1[0].clone();
    colon = __unpk1[1].clone();
    tail = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&colon.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return step.clone();
    }
    return mcall(&mcall(&head.clone(), "strip", &[]), "rstrip", &[py_str(" .,;")]);
    Py::None
}
pub fn _with_steps(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut fallback: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The remaining steps said as what to do next.");
    said = mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut step: Py = __item.clone(); __out.push(if py_truthy(&mcall(&geti(&step.clone(), &Py::Int(0)), "isupper", &[])) { op_add(&mcall(&geti(&step.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { step.clone() }); } py_list(__out) })() }]);
    said = mcall(&said.clone(), "rstrip", &[py_str(" .")]);
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+)?then\\s+")); __args.push(py_str("")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mcall(&fallback.clone(), "_choose", &[g_STEPS_LEADS(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, said.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _the_last_answer(args: &[Py]) -> Py {
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
    let mut plan_steps: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's last reply, which is the answer a report is about.\n\n    When that reply was one step of an answer rather than the answer (\"Start with this: Turn the\n    water off at the isolating valve under the sink. There are three steps in all.\"), the answer\n    it came from is what is read: the step alone has nothing after it, and \"ive already turned the\n    water off\" would be told there is nothing left to do.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&t.clone(), "get", &[py_str("bot")]) } }) {  __out.push(t.clone()); } } py_list(__out) })() };
    last = if py_truthy(&turns.clone()) { call_builtin("str", &[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")])]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    plan_steps = pyrt::xrefs::xmodule("memory.plan");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = mcall(&plan_steps.clone(), "the_answer_in_hand", &[memory.clone()]); if py_truthy(&__b) { __b } else { last.clone() } }]);
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
    return last.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn tells_them_the_step_again(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut step: Py = Py::None;
    let _ = py_str("True when `reply` still tells the person to do the step they have just reported doing.\n\n    Published for the arbiter, which has to decide whether a card on the line's topic has read the\n    report or answered the question above it. After \"my boiler is making a noise\" / \"i already\n    bled the radiators\" the radiators entry read the bleeding instructions out a second time, and\n    a card that still says \"bleed the radiators\" has not seen the line at all: the person has just\n    told it they did. The reading is this module's own (`the_step_they_took` is the same reading\n    that claims the turn), so the arbiter keeps no vocabulary of its own.\n    ");
    let __try1 = exc::catch(|| {
    step = the_step_they_took(&[text.clone(), memory.clone()]);
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
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&the_trouble_is_over(&[text.clone()])) {
    return Py::Bool(true);
    }
    let __try6 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), _says_the_step(&[step.clone(), reply.clone()])]);
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
    Py::None
}
pub fn _of_a_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the step was done to something in the house rather than to the person's life.\n\n    The step words themselves say nothing about it - \"changed\" is in `CHANGED` for \"i changed my\n    routine\" and it read \"i changed the washer\" the same way - so the object decides. A life\n    object named after the step makes it a life change; anything else (\"the washer\", \"the\n    filter\", \"the bulb\") is a thing that got mended, and the question after it is whether it\n    is any better since. A person, a creature or the laundry is the third case: nothing was\n    broken, so nothing is asked after it (`CHANGED_NOTHING_BROKEN`).\n    ");
    if py_truthy(&Py::Bool(!py_contains(&g_CHANGED_OF_A_THING(), &step.clone()))) {
    return Py::Bool(false);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    at = mcall(&words.clone(), "index", &[step.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ValueError"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    after = slice_or_raise(&words.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CHANGED_NOTHING_BROKEN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_LIFE_OBJECT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _only_finished(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the finished reading is the only reading of the line, and not the word \"already\".");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = _the_step(&[low.clone()]); if py_truthy(&__b) { __b } else { _the_step_after_did(&[low.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[_the_finished_step(&[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_part_after(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut step_text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step_text"))) };
    let mut _u: Py = Py::None;
    let mut a_limit: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut score: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The answer's own next instruction after the step it has just handed back, or [].\n\n    `_the_steps_after` finds the step from the verb the person used (\"bleed\"), and that is the\n    right reading when they say the step themselves. Here the person says only that it has not\n    worked (\"its still noisy\"), so the step is the one the bot handed back last turn - the\n    pressure gauge, not the bleeding - and the piece that follows it is a whole instruction of\n    the answer, often opening on \"and\" (\"and if it carries on get a heating engineer\"). Read as\n    prose, that \"and\" is the answer wandering off, so this one matches the instruction by its\n    words and takes the piece after it, minus the joining word.\n    ");
    a_limit = pyrt::xrefs::xmodule("memory.limit");
    let __try1 = exc::catch(|| {
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&a_limit.clone(), "_steps", &[answer.clone()])) { let __unpk8660 = py_unpack(&__item.clone(), 2, None); let mut one: Py = __unpk8660[0].clone(); let mut _u: Py = __unpk8660[1].clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
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
    return py_list(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&pieces.clone()))) {
    return py_list(vec![]);
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = step_text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING_WORD(), &word.clone())) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    let __unpk6 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0)]), 2, None);
    best = __unpk6[0].clone();
    best_score = __unpk6[1].clone();
    let __iter7 = py_iter_or_raise(&call_builtin("enumerate", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter7 {
    let __unpk8 = py_unpack(&__item.clone(), 2, None);
    index = __unpk8[0].clone();
    piece = __unpk8[1].clone();
    score = call_builtin("len", &[op_bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&piece.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &wanted.clone())]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&score.clone(), &best_score.clone())) {
    let __unpk9 = py_unpack(&py_tuple(vec![index.clone(), score.clone()]), 2, None);
    best = __unpk9[0].clone();
    best_score = __unpk9[1].clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&best_score.clone(), &Py::Int(2)) } }) {
    return py_list(vec![]);
    }
    let __iter10 = py_iter_or_raise(&slice_or_raise(&pieces.clone(), &op_add(&best.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter10 {
    piece = __item.clone();
    piece = mcall(&mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|then|but|so)\\s+")); __args.push(py_str("")); __args.push(piece.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", &[]), "rstrip", &[py_str(" .,;")]);
    if py_truthy(&Py::Bool(!py_truthy(&piece.clone()))) {
    continue;
    }
    if py_truthy(&mcall(&g_CONDITION_START(), "match", &[piece.clone()])) {
    return py_list(vec![piece.clone()]);
    }
    if py_truthy(&mcall(&slice_or_raise(&piece.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) {
    break;
    }
    }
    return py_list(vec![]);
    Py::None
}
pub fn after_the_step_handed_back(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut step: Py = Py::None;
    let mut t: Py = Py::None;
    let mut tried: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The answer's own next step when the step it handed back did not do it, or None.\n\n    \"the boiler is making a noise\" / \"i bled the radiators\" / \"its still noisy\": the answer gave\n    the bleeding, then the pressure, and \"still noisy\" says the pressure was not the end of it.\n    Nothing here knows a better step than the answer did, so the answer's own next sentence is\n    the reply (\"if it carries on get a heating engineer to look at the pump\") - where the store\n    would otherwise ask the person to watch it happen again.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    step = call_builtin("str", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("handed_back_text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    answer = call_builtin("str", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("handed_back_answer")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&memory.clone(), "get", &[py_str("handed_back_step")]), &op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) } } } }) {
    return Py::None;
    }
    tried = pyrt::xrefs::xmodule("memory.tried");
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&tried.clone(), "still_the_state", &[text.clone(), memory.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
    after = _the_part_after(&[answer.clone(), step.clone()]);
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
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return Py::None;
    }
    return _with_steps(&[after.clone(), memory.clone()]);
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut job: Py = Py::None;
    let mut last: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The next turn of the thread for a report that the step was already taken, or None.");
    let __try1 = exc::catch(|| {
    step = the_step_they_took(&[text.clone(), memory.clone()]);
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
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return after_the_step_handed_back(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_the_last_answer(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]))); if !py_truthy(&__b) { __b } else { _only_finished(&[text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&the_trouble_is_over(&[text.clone()])) {
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    if py_truthy(&_a_crisis_line_said(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, gattr(&fallback.clone(), "CRISIS_WORDS")])) {
    return Py::None;
    }
    thing = the_thing_that_is_mended(&[text.clone()]);
    if py_truthy(&thing.clone()) {
    job = Py::Bool(py_contains(&g_OVER_A_JOB_DONE(), &the_trouble_is_over(&[text.clone()])));
    pool = if py_truthy(&job.clone()) { g_OVER_NAMED_JOB_REPLIES() } else { g_OVER_NAMED_REPLIES() };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(mcall(&fallback.clone(), "memory_bot", &[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "_choose", args))(&__args) };
    return mcall(&said.clone(), "replace", &[py_str("{thing}"), thing.clone()]);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_OVER_REPLIES()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(mcall(&fallback.clone(), "memory_bot", &[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "_choose", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SAID_TO_ME(), &step.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory", "there_is_a_chat", &[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.warmth", "FIRST_LINE_REPLIES")); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    let __try6 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_BARE_COMPLETION(), &step.clone())); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("memory.repeat", "_tried", &[text.clone()]); if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.repeat", "_later_part", &[text.clone(), _the_last_answer(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])]) } } } }) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    let __try11 = exc::catch(|| {
    if py_truthy(&Py::Bool(py_contains(&g_BARE_COMPLETION(), &step.clone()))) {
    after = py_list(vec![]);
    } else {
    after = _the_steps_after(&[step.clone(), _the_last_answer(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret12: Py = Py::Bool(false);
    let mut __val13: Py = Py::None;
    let mut __unhandled14: Option<exc::PyExc> = None;
    match __try11 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret12 = __p[0].clone(); __val13 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled15 = false;
            if !__handled15 && exc::matches_any(&__e, &["Exception"]) {
                __handled15 = true;
    after = py_list(vec![]);
            }
            if !__handled15 { __unhandled14 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled14 { exc::raise(__e); }
    if py_truthy(&__ret12) { return __val13; }
    if py_truthy(&after.clone()) {
    return _with_steps(&[after.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_STEP_TAKEN(), &step.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_CONTACT(), &step.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_TELLING(), &step.clone())); if !py_truthy(&__b) { __b } else { _the_last_answer(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]) } } } } } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_says_the_step(&[step.clone(), _the_opening_of(&[_the_last_answer(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])])])))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOBODY_TO_ANSWER(), &step.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_CONTACT(), &step.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_AN_APOLOGY(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } }) {
    pool = g_REPORT_REPLIES();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_CONTACT(), &step.clone()))) {
    pool = g_CONTACT_REPLIES();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_REPAIRED(), &step.clone())); if py_truthy(&__b) { __b } else { _of_a_thing(&[step.clone(), text.clone()]) } }) {
    pool = g_REPAIRED_REPLIES();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_CHANGED(), &step.clone()))) {
    pool = g_CHANGED_REPLIES();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_SETTLED(), &step.clone()))) {
    pool = g_SETTLED_REPLIES();
    } else {
    pool = g_REPORT_REPLIES();
    }
    }
    }
    }
    }
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    last = mcall(&fallback.clone(), "memory_bot", &[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]);
    return mcall(&fallback.clone(), "_choose", &[pool.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, last.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.already".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALREADY" => Some(g_ALREADY()),
            "AN_APOLOGY" => Some(g_AN_APOLOGY()),
            "AN_OUTCOME" => Some(g_AN_OUTCOME()),
            "APOLOGY" => Some(g_APOLOGY()),
            "APOLOGY_STEP" => Some(g_APOLOGY_STEP()),
            "ASKING" => Some(g_ASKING()),
            "A_LIFE_OBJECT" => Some(g_A_LIFE_OBJECT()),
            "A_STEP_TAKEN" => Some(g_A_STEP_TAKEN()),
            "BARE_COMPLETION" => Some(g_BARE_COMPLETION()),
            "BARE_DONE" => Some(g_BARE_DONE()),
            "CHANGED" => Some(g_CHANGED()),
            "CHANGED_NOTHING_BROKEN" => Some(g_CHANGED_NOTHING_BROKEN()),
            "CHANGED_OF_A_THING" => Some(g_CHANGED_OF_A_THING()),
            "CHANGED_REPLIES" => Some(g_CHANGED_REPLIES()),
            "CONDITION_START" => Some(g_CONDITION_START()),
            "CONTACT" => Some(g_CONTACT()),
            "CONTACT_REPLIES" => Some(g_CONTACT_REPLIES()),
            "DID" => Some(g_DID()),
            "DID_STEPS" => Some(g_DID_STEPS()),
            "DID_TAIL" => Some(g_DID_TAIL()),
            "DONE_THAT" => Some(g_DONE_THAT()),
            "FINISHED_ACT" => Some(g_FINISHED_ACT()),
            "FINISHED_ITSELF" => Some(g_FINISHED_ITSELF()),
            "FINISHED_STEPS" => Some(g_FINISHED_STEPS()),
            "FIRST_LINE_REPLIES" => Some(g_FIRST_LINE_REPLIES()),
            "HARD" => Some(g_HARD()),
            "IRREGULAR" => Some(g_IRREGULAR()),
            "KEPT_UP" => Some(g_KEPT_UP()),
            "MORE_PADDING" => Some(g_MORE_PADDING()),
            "MOST_STEPS" => Some(g_MOST_STEPS()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "NOBODY_TO_ANSWER" => Some(g_NOBODY_TO_ANSWER()),
            "NOT_A_STEP_START" => Some(g_NOT_A_STEP_START()),
            "NOT_A_THING_SAID_BACK" => Some(g_NOT_A_THING_SAID_BACK()),
            "NOT_A_THING_WORD" => Some(g_NOT_A_THING_WORD()),
            "OVER_ADJECTIVE" => Some(g_OVER_ADJECTIVE()),
            "OVER_ASKING" => Some(g_OVER_ASKING()),
            "OVER_A_JOB_DONE" => Some(g_OVER_A_JOB_DONE()),
            "OVER_A_MEAL" => Some(g_OVER_A_MEAL()),
            "OVER_BETWEEN" => Some(g_OVER_BETWEEN()),
            "OVER_GOT" => Some(g_OVER_GOT()),
            "OVER_LEAD" => Some(g_OVER_LEAD()),
            "OVER_NAMED_JOB_REPLIES" => Some(g_OVER_NAMED_JOB_REPLIES()),
            "OVER_NAMED_REPLIES" => Some(g_OVER_NAMED_REPLIES()),
            "OVER_NEVER_A_THING" => Some(g_OVER_NEVER_A_THING()),
            "OVER_NOT" => Some(g_OVER_NOT()),
            "OVER_NOT_A_THING_NAMED" => Some(g_OVER_NOT_A_THING_NAMED()),
            "OVER_NO_THING" => Some(g_OVER_NO_THING()),
            "OVER_PEOPLE" => Some(g_OVER_PEOPLE()),
            "OVER_REPLIES" => Some(g_OVER_REPLIES()),
            "OVER_STATE" => Some(g_OVER_STATE()),
            "OVER_STOP" => Some(g_OVER_STOP()),
            "OVER_STYLED" => Some(g_OVER_STYLED()),
            "OVER_WORD" => Some(g_OVER_WORD()),
            "PAST" => Some(g_PAST()),
            "POINTS_AT" => Some(g_POINTS_AT()),
            "REPAIRED" => Some(g_REPAIRED()),
            "REPAIRED_REPLIES" => Some(g_REPAIRED_REPLIES()),
            "REPORT_FILLER" => Some(g_REPORT_FILLER()),
            "REPORT_REPLIES" => Some(g_REPORT_REPLIES()),
            "SAID_TO_ME" => Some(g_SAID_TO_ME()),
            "SAME_AS_PRESENT" => Some(g_SAME_AS_PRESENT()),
            "SAYS_APOLOGY" => Some(g_SAYS_APOLOGY()),
            "SETTLED" => Some(g_SETTLED()),
            "SETTLED_REPLIES" => Some(g_SETTLED_REPLIES()),
            "STEPS_LEADS" => Some(g_STEPS_LEADS()),
            "SUBJECT" => Some(g_SUBJECT()),
            "TELLING" => Some(g_TELLING()),
            "TO_THE_BOT" => Some(g_TO_THE_BOT()),
            "TRAILING" => Some(g_TRAILING()),
            "_CLAUSE_BREAK" => Some(g__CLAUSE_BREAK()),
            "_NEGATION_STEM" => Some(g__NEGATION_STEM()),
            "_a_crisis_line_said" => Some(py_func("_a_crisis_line_said", |args: &[Py]| _a_crisis_line_said(args))),
            "_a_hard_thread" => Some(py_func("_a_hard_thread", |args: &[Py]| _a_hard_thread(args))),
            "_a_job_with_an_end" => Some(py_func("_a_job_with_an_end", |args: &[Py]| _a_job_with_an_end(args))),
            "_gave_a_step" => Some(py_func("_gave_a_step", |args: &[Py]| _gave_a_step(args))),
            "_nothing_more" => Some(py_func("_nothing_more", |args: &[Py]| _nothing_more(args))),
            "_of_a_thing" => Some(py_func("_of_a_thing", |args: &[Py]| _of_a_thing(args))),
            "_only_finished" => Some(py_func("_only_finished", |args: &[Py]| _only_finished(args))),
            "_says_the_step" => Some(py_func("_says_the_step", |args: &[Py]| _says_the_step(args))),
            "_the_clause" => Some(py_func("_the_clause", |args: &[Py]| _the_clause(args))),
            "_the_finished_step" => Some(py_func("_the_finished_step", |args: &[Py]| _the_finished_step(args))),
            "_the_instruction_of" => Some(py_func("_the_instruction_of", |args: &[Py]| _the_instruction_of(args))),
            "_the_last_answer" => Some(py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args))),
            "_the_opening_of" => Some(py_func("_the_opening_of", |args: &[Py]| _the_opening_of(args))),
            "_the_over_word" => Some(py_func("_the_over_word", |args: &[Py]| _the_over_word(args))),
            "_the_part_after" => Some(py_func("_the_part_after", |args: &[Py]| _the_part_after(args))),
            "_the_step" => Some(py_func("_the_step", |args: &[Py]| _the_step(args))),
            "_the_step_after_did" => Some(py_func("_the_step_after_did", |args: &[Py]| _the_step_after_did(args))),
            "_the_steps_after" => Some(py_func("_the_steps_after", |args: &[Py]| _the_steps_after(args))),
            "_the_thing_said" => Some(py_func("_the_thing_said", |args: &[Py]| _the_thing_said(args))),
            "_with_steps" => Some(py_func("_with_steps", |args: &[Py]| _with_steps(args))),
            "after_the_step_handed_back" => Some(py_func("after_the_step_handed_back", |args: &[Py]| after_the_step_handed_back(args))),
            "pick" => Some(g_pick()),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            "tells_them_the_step_again" => Some(py_func("tells_them_the_step_again", |args: &[Py]| tells_them_the_step_again(args))),
            "the_step_they_took" => Some(py_func("the_step_they_took", |args: &[Py]| the_step_they_took(args))),
            "the_thing_that_is_mended" => Some(py_func("the_thing_that_is_mended", |args: &[Py]| the_thing_that_is_mended(args))),
            "the_trouble_is_over" => Some(py_func("the_trouble_is_over", |args: &[Py]| the_trouble_is_over(args))),
            "there_is_a_chat" => Some(g_there_is_a_chat()),
            "repeat" => Some(pyrt::xrefs::xmodule("memory.repeat")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALREADY" => s_ALREADY(v),
            "AN_APOLOGY" => s_AN_APOLOGY(v),
            "AN_OUTCOME" => s_AN_OUTCOME(v),
            "APOLOGY" => s_APOLOGY(v),
            "APOLOGY_STEP" => s_APOLOGY_STEP(v),
            "ASKING" => s_ASKING(v),
            "A_LIFE_OBJECT" => s_A_LIFE_OBJECT(v),
            "A_STEP_TAKEN" => s_A_STEP_TAKEN(v),
            "BARE_COMPLETION" => s_BARE_COMPLETION(v),
            "BARE_DONE" => s_BARE_DONE(v),
            "CHANGED" => s_CHANGED(v),
            "CHANGED_NOTHING_BROKEN" => s_CHANGED_NOTHING_BROKEN(v),
            "CHANGED_OF_A_THING" => s_CHANGED_OF_A_THING(v),
            "CHANGED_REPLIES" => s_CHANGED_REPLIES(v),
            "CONDITION_START" => s_CONDITION_START(v),
            "CONTACT" => s_CONTACT(v),
            "CONTACT_REPLIES" => s_CONTACT_REPLIES(v),
            "DID" => s_DID(v),
            "DID_STEPS" => s_DID_STEPS(v),
            "DID_TAIL" => s_DID_TAIL(v),
            "DONE_THAT" => s_DONE_THAT(v),
            "FINISHED_ACT" => s_FINISHED_ACT(v),
            "FINISHED_ITSELF" => s_FINISHED_ITSELF(v),
            "FINISHED_STEPS" => s_FINISHED_STEPS(v),
            "FIRST_LINE_REPLIES" => s_FIRST_LINE_REPLIES(v),
            "HARD" => s_HARD(v),
            "IRREGULAR" => s_IRREGULAR(v),
            "KEPT_UP" => s_KEPT_UP(v),
            "MORE_PADDING" => s_MORE_PADDING(v),
            "MOST_STEPS" => s_MOST_STEPS(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "NOBODY_TO_ANSWER" => s_NOBODY_TO_ANSWER(v),
            "NOT_A_STEP_START" => s_NOT_A_STEP_START(v),
            "NOT_A_THING_SAID_BACK" => s_NOT_A_THING_SAID_BACK(v),
            "NOT_A_THING_WORD" => s_NOT_A_THING_WORD(v),
            "OVER_ADJECTIVE" => s_OVER_ADJECTIVE(v),
            "OVER_ASKING" => s_OVER_ASKING(v),
            "OVER_A_JOB_DONE" => s_OVER_A_JOB_DONE(v),
            "OVER_A_MEAL" => s_OVER_A_MEAL(v),
            "OVER_BETWEEN" => s_OVER_BETWEEN(v),
            "OVER_GOT" => s_OVER_GOT(v),
            "OVER_LEAD" => s_OVER_LEAD(v),
            "OVER_NAMED_JOB_REPLIES" => s_OVER_NAMED_JOB_REPLIES(v),
            "OVER_NAMED_REPLIES" => s_OVER_NAMED_REPLIES(v),
            "OVER_NEVER_A_THING" => s_OVER_NEVER_A_THING(v),
            "OVER_NOT" => s_OVER_NOT(v),
            "OVER_NOT_A_THING_NAMED" => s_OVER_NOT_A_THING_NAMED(v),
            "OVER_NO_THING" => s_OVER_NO_THING(v),
            "OVER_PEOPLE" => s_OVER_PEOPLE(v),
            "OVER_REPLIES" => s_OVER_REPLIES(v),
            "OVER_STATE" => s_OVER_STATE(v),
            "OVER_STOP" => s_OVER_STOP(v),
            "OVER_STYLED" => s_OVER_STYLED(v),
            "OVER_WORD" => s_OVER_WORD(v),
            "PAST" => s_PAST(v),
            "POINTS_AT" => s_POINTS_AT(v),
            "REPAIRED" => s_REPAIRED(v),
            "REPAIRED_REPLIES" => s_REPAIRED_REPLIES(v),
            "REPORT_FILLER" => s_REPORT_FILLER(v),
            "REPORT_REPLIES" => s_REPORT_REPLIES(v),
            "SAID_TO_ME" => s_SAID_TO_ME(v),
            "SAME_AS_PRESENT" => s_SAME_AS_PRESENT(v),
            "SAYS_APOLOGY" => s_SAYS_APOLOGY(v),
            "SETTLED" => s_SETTLED(v),
            "SETTLED_REPLIES" => s_SETTLED_REPLIES(v),
            "STEPS_LEADS" => s_STEPS_LEADS(v),
            "SUBJECT" => s_SUBJECT(v),
            "TELLING" => s_TELLING(v),
            "TO_THE_BOT" => s_TO_THE_BOT(v),
            "TRAILING" => s_TRAILING(v),
            "_CLAUSE_BREAK" => s__CLAUSE_BREAK(v),
            "_NEGATION_STEM" => s__NEGATION_STEM(v),
            "pick" => s_pick(v),
            "there_is_a_chat" => s_there_is_a_chat(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.already", module_obj);
    pyrt::xrefs::reg_fn("memory.already", "_the_step_after_did", py_func("_the_step_after_did", |args: &[Py]| _the_step_after_did(args)));
    pyrt::xrefs::reg_fn("memory.already", "_a_crisis_line_said", py_func("_a_crisis_line_said", |args: &[Py]| _a_crisis_line_said(args)));
    pyrt::xrefs::reg_fn("memory.already", "_a_hard_thread", py_func("_a_hard_thread", |args: &[Py]| _a_hard_thread(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_step", py_func("_the_step", |args: &[Py]| _the_step(args)));
    pyrt::xrefs::reg_fn("memory.already", "_nothing_more", py_func("_nothing_more", |args: &[Py]| _nothing_more(args)));
    pyrt::xrefs::reg_fn("memory.already", "_a_job_with_an_end", py_func("_a_job_with_an_end", |args: &[Py]| _a_job_with_an_end(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_finished_step", py_func("_the_finished_step", |args: &[Py]| _the_finished_step(args)));
    pyrt::xrefs::reg_fn("memory.already", "the_step_they_took", py_func("the_step_they_took", |args: &[Py]| the_step_they_took(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_thing_said", py_func("_the_thing_said", |args: &[Py]| _the_thing_said(args)));
    pyrt::xrefs::reg_fn("memory.already", "the_thing_that_is_mended", py_func("the_thing_that_is_mended", |args: &[Py]| the_thing_that_is_mended(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_clause", py_func("_the_clause", |args: &[Py]| _the_clause(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_over_word", py_func("_the_over_word", |args: &[Py]| _the_over_word(args)));
    pyrt::xrefs::reg_fn("memory.already", "the_trouble_is_over", py_func("the_trouble_is_over", |args: &[Py]| the_trouble_is_over(args)));
    pyrt::xrefs::reg_fn("memory.already", "_gave_a_step", py_func("_gave_a_step", |args: &[Py]| _gave_a_step(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_opening_of", py_func("_the_opening_of", |args: &[Py]| _the_opening_of(args)));
    pyrt::xrefs::reg_fn("memory.already", "_says_the_step", py_func("_says_the_step", |args: &[Py]| _says_the_step(args)));
    pyrt::xrefs::reg_fn("memory.already", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_steps_after", py_func("_the_steps_after", |args: &[Py]| _the_steps_after(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_instruction_of", py_func("_the_instruction_of", |args: &[Py]| _the_instruction_of(args)));
    pyrt::xrefs::reg_fn("memory.already", "_with_steps", py_func("_with_steps", |args: &[Py]| _with_steps(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_last_answer", py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args)));
    pyrt::xrefs::reg_fn("memory.already", "tells_them_the_step_again", py_func("tells_them_the_step_again", |args: &[Py]| tells_them_the_step_again(args)));
    pyrt::xrefs::reg_fn("memory.already", "_of_a_thing", py_func("_of_a_thing", |args: &[Py]| _of_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.already", "_only_finished", py_func("_only_finished", |args: &[Py]| _only_finished(args)));
    pyrt::xrefs::reg_fn("memory.already", "_the_part_after", py_func("_the_part_after", |args: &[Py]| _the_part_after(args)));
    pyrt::xrefs::reg_fn("memory.already", "after_the_step_handed_back", py_func("after_the_step_handed_back", |args: &[Py]| after_the_step_handed_back(args)));
    pyrt::xrefs::reg_fn("memory.already", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.already", "ALREADY", g_ALREADY, s_ALREADY);
    pyrt::xrefs::reg_access("memory.already", "AN_APOLOGY", g_AN_APOLOGY, s_AN_APOLOGY);
    pyrt::xrefs::reg_access("memory.already", "AN_OUTCOME", g_AN_OUTCOME, s_AN_OUTCOME);
    pyrt::xrefs::reg_access("memory.already", "APOLOGY", g_APOLOGY, s_APOLOGY);
    pyrt::xrefs::reg_access("memory.already", "APOLOGY_STEP", g_APOLOGY_STEP, s_APOLOGY_STEP);
    pyrt::xrefs::reg_access("memory.already", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("memory.already", "A_LIFE_OBJECT", g_A_LIFE_OBJECT, s_A_LIFE_OBJECT);
    pyrt::xrefs::reg_access("memory.already", "A_STEP_TAKEN", g_A_STEP_TAKEN, s_A_STEP_TAKEN);
    pyrt::xrefs::reg_access("memory.already", "BARE_COMPLETION", g_BARE_COMPLETION, s_BARE_COMPLETION);
    pyrt::xrefs::reg_access("memory.already", "BARE_DONE", g_BARE_DONE, s_BARE_DONE);
    pyrt::xrefs::reg_access("memory.already", "CHANGED", g_CHANGED, s_CHANGED);
    pyrt::xrefs::reg_access("memory.already", "CHANGED_NOTHING_BROKEN", g_CHANGED_NOTHING_BROKEN, s_CHANGED_NOTHING_BROKEN);
    pyrt::xrefs::reg_access("memory.already", "CHANGED_OF_A_THING", g_CHANGED_OF_A_THING, s_CHANGED_OF_A_THING);
    pyrt::xrefs::reg_access("memory.already", "CHANGED_REPLIES", g_CHANGED_REPLIES, s_CHANGED_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "CONDITION_START", g_CONDITION_START, s_CONDITION_START);
    pyrt::xrefs::reg_access("memory.already", "CONTACT", g_CONTACT, s_CONTACT);
    pyrt::xrefs::reg_access("memory.already", "CONTACT_REPLIES", g_CONTACT_REPLIES, s_CONTACT_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "DID", g_DID, s_DID);
    pyrt::xrefs::reg_access("memory.already", "DID_STEPS", g_DID_STEPS, s_DID_STEPS);
    pyrt::xrefs::reg_access("memory.already", "DID_TAIL", g_DID_TAIL, s_DID_TAIL);
    pyrt::xrefs::reg_access("memory.already", "DONE_THAT", g_DONE_THAT, s_DONE_THAT);
    pyrt::xrefs::reg_access("memory.already", "FINISHED_ACT", g_FINISHED_ACT, s_FINISHED_ACT);
    pyrt::xrefs::reg_access("memory.already", "FINISHED_ITSELF", g_FINISHED_ITSELF, s_FINISHED_ITSELF);
    pyrt::xrefs::reg_access("memory.already", "FINISHED_STEPS", g_FINISHED_STEPS, s_FINISHED_STEPS);
    pyrt::xrefs::reg_access("memory.already", "FIRST_LINE_REPLIES", g_FIRST_LINE_REPLIES, s_FIRST_LINE_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "HARD", g_HARD, s_HARD);
    pyrt::xrefs::reg_access("memory.already", "IRREGULAR", g_IRREGULAR, s_IRREGULAR);
    pyrt::xrefs::reg_access("memory.already", "KEPT_UP", g_KEPT_UP, s_KEPT_UP);
    pyrt::xrefs::reg_access("memory.already", "MORE_PADDING", g_MORE_PADDING, s_MORE_PADDING);
    pyrt::xrefs::reg_access("memory.already", "MOST_STEPS", g_MOST_STEPS, s_MOST_STEPS);
    pyrt::xrefs::reg_access("memory.already", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("memory.already", "NOBODY_TO_ANSWER", g_NOBODY_TO_ANSWER, s_NOBODY_TO_ANSWER);
    pyrt::xrefs::reg_access("memory.already", "NOT_A_STEP_START", g_NOT_A_STEP_START, s_NOT_A_STEP_START);
    pyrt::xrefs::reg_access("memory.already", "NOT_A_THING_SAID_BACK", g_NOT_A_THING_SAID_BACK, s_NOT_A_THING_SAID_BACK);
    pyrt::xrefs::reg_access("memory.already", "NOT_A_THING_WORD", g_NOT_A_THING_WORD, s_NOT_A_THING_WORD);
    pyrt::xrefs::reg_access("memory.already", "OVER_ADJECTIVE", g_OVER_ADJECTIVE, s_OVER_ADJECTIVE);
    pyrt::xrefs::reg_access("memory.already", "OVER_ASKING", g_OVER_ASKING, s_OVER_ASKING);
    pyrt::xrefs::reg_access("memory.already", "OVER_A_JOB_DONE", g_OVER_A_JOB_DONE, s_OVER_A_JOB_DONE);
    pyrt::xrefs::reg_access("memory.already", "OVER_A_MEAL", g_OVER_A_MEAL, s_OVER_A_MEAL);
    pyrt::xrefs::reg_access("memory.already", "OVER_BETWEEN", g_OVER_BETWEEN, s_OVER_BETWEEN);
    pyrt::xrefs::reg_access("memory.already", "OVER_GOT", g_OVER_GOT, s_OVER_GOT);
    pyrt::xrefs::reg_access("memory.already", "OVER_LEAD", g_OVER_LEAD, s_OVER_LEAD);
    pyrt::xrefs::reg_access("memory.already", "OVER_NAMED_JOB_REPLIES", g_OVER_NAMED_JOB_REPLIES, s_OVER_NAMED_JOB_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "OVER_NAMED_REPLIES", g_OVER_NAMED_REPLIES, s_OVER_NAMED_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "OVER_NEVER_A_THING", g_OVER_NEVER_A_THING, s_OVER_NEVER_A_THING);
    pyrt::xrefs::reg_access("memory.already", "OVER_NOT", g_OVER_NOT, s_OVER_NOT);
    pyrt::xrefs::reg_access("memory.already", "OVER_NOT_A_THING_NAMED", g_OVER_NOT_A_THING_NAMED, s_OVER_NOT_A_THING_NAMED);
    pyrt::xrefs::reg_access("memory.already", "OVER_NO_THING", g_OVER_NO_THING, s_OVER_NO_THING);
    pyrt::xrefs::reg_access("memory.already", "OVER_PEOPLE", g_OVER_PEOPLE, s_OVER_PEOPLE);
    pyrt::xrefs::reg_access("memory.already", "OVER_REPLIES", g_OVER_REPLIES, s_OVER_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "OVER_STATE", g_OVER_STATE, s_OVER_STATE);
    pyrt::xrefs::reg_access("memory.already", "OVER_STOP", g_OVER_STOP, s_OVER_STOP);
    pyrt::xrefs::reg_access("memory.already", "OVER_STYLED", g_OVER_STYLED, s_OVER_STYLED);
    pyrt::xrefs::reg_access("memory.already", "OVER_WORD", g_OVER_WORD, s_OVER_WORD);
    pyrt::xrefs::reg_access("memory.already", "PAST", g_PAST, s_PAST);
    pyrt::xrefs::reg_access("memory.already", "POINTS_AT", g_POINTS_AT, s_POINTS_AT);
    pyrt::xrefs::reg_access("memory.already", "REPAIRED", g_REPAIRED, s_REPAIRED);
    pyrt::xrefs::reg_access("memory.already", "REPAIRED_REPLIES", g_REPAIRED_REPLIES, s_REPAIRED_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "REPORT_FILLER", g_REPORT_FILLER, s_REPORT_FILLER);
    pyrt::xrefs::reg_access("memory.already", "REPORT_REPLIES", g_REPORT_REPLIES, s_REPORT_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "SAID_TO_ME", g_SAID_TO_ME, s_SAID_TO_ME);
    pyrt::xrefs::reg_access("memory.already", "SAME_AS_PRESENT", g_SAME_AS_PRESENT, s_SAME_AS_PRESENT);
    pyrt::xrefs::reg_access("memory.already", "SAYS_APOLOGY", g_SAYS_APOLOGY, s_SAYS_APOLOGY);
    pyrt::xrefs::reg_access("memory.already", "SETTLED", g_SETTLED, s_SETTLED);
    pyrt::xrefs::reg_access("memory.already", "SETTLED_REPLIES", g_SETTLED_REPLIES, s_SETTLED_REPLIES);
    pyrt::xrefs::reg_access("memory.already", "STEPS_LEADS", g_STEPS_LEADS, s_STEPS_LEADS);
    pyrt::xrefs::reg_access("memory.already", "SUBJECT", g_SUBJECT, s_SUBJECT);
    pyrt::xrefs::reg_access("memory.already", "TELLING", g_TELLING, s_TELLING);
    pyrt::xrefs::reg_access("memory.already", "TO_THE_BOT", g_TO_THE_BOT, s_TO_THE_BOT);
    pyrt::xrefs::reg_access("memory.already", "TRAILING", g_TRAILING, s_TRAILING);
    pyrt::xrefs::reg_access("memory.already", "_CLAUSE_BREAK", g__CLAUSE_BREAK, s__CLAUSE_BREAK);
    pyrt::xrefs::reg_access("memory.already", "_NEGATION_STEM", g__NEGATION_STEM, s__NEGATION_STEM);
    pyrt::xrefs::reg_access("memory.already", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("memory.already", "there_is_a_chat", g_there_is_a_chat, s_there_is_a_chat);
}
