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


thread_local! { static G_ABOUT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_A_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALWAYS_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALWAYS_DOES_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARRIVES_LATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHARED_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COURTESY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COURTESY_DENIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CREDIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CREDIT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRITICISED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRITICISED_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRITICISED_MY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_TO_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMAIL_HABIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVERY_TIME_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_HABIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_HABIT_APPRECIATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_HABIT_INCLUDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOLD_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPS_THE_ARRANGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LATE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MESSY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_GRUMBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_HELPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_SINCERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_SET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RAISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_IT_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_ABOUT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_DOUBT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKEN_FOR_GRANTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THANKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THREAD_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNSEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORDS_NOT_MEANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.habit: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_ME() -> Py { ensure_init(); G_ABOUT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_ME(v: Py) { ensure_init(); G_ABOUT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_A_SAYING() -> Py { ensure_init(); G_AFTER_A_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_A_SAYING(v: Py) { ensure_init(); G_AFTER_A_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g_ALWAYS_COUNT() -> Py { ensure_init(); G_ALWAYS_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALWAYS_COUNT(v: Py) { ensure_init(); G_ALWAYS_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_ALWAYS_DOES_IT() -> Py { ensure_init(); G_ALWAYS_DOES_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALWAYS_DOES_IT(v: Py) { ensure_init(); G_ALWAYS_DOES_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_ARRIVES_LATE() -> Py { ensure_init(); G_ARRIVES_LATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARRIVES_LATE(v: Py) { ensure_init(); G_ARRIVES_LATE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED() -> Py { ensure_init(); G_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED(v: Py) { ensure_init(); G_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_FOR() -> Py { ensure_init(); G_ASK_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_FOR(v: Py) { ensure_init(); G_ASK_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON() -> Py { ensure_init(); G_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); G_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHARED_OCCASION() -> Py { ensure_init(); G_A_SHARED_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHARED_OCCASION(v: Py) { ensure_init(); G_A_SHARED_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_BAILING() -> Py { ensure_init(); G_BAILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAILING(v: Py) { ensure_init(); G_BAILING.with(|c| *c.borrow_mut() = v); }
pub fn g_BAILS() -> Py { ensure_init(); G_BAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAILS(v: Py) { ensure_init(); G_BAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_PEOPLE() -> Py { ensure_init(); G_CHILD_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_PEOPLE(v: Py) { ensure_init(); G_CHILD_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_COURTESY() -> Py { ensure_init(); G_COURTESY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COURTESY(v: Py) { ensure_init(); G_COURTESY.with(|c| *c.borrow_mut() = v); }
pub fn g_COURTESY_DENIED() -> Py { ensure_init(); G_COURTESY_DENIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COURTESY_DENIED(v: Py) { ensure_init(); G_COURTESY_DENIED.with(|c| *c.borrow_mut() = v); }
pub fn g_CREDIT() -> Py { ensure_init(); G_CREDIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CREDIT(v: Py) { ensure_init(); G_CREDIT.with(|c| *c.borrow_mut() = v); }
pub fn g_CREDIT_LINES() -> Py { ensure_init(); G_CREDIT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CREDIT_LINES(v: Py) { ensure_init(); G_CREDIT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CRITICISED() -> Py { ensure_init(); G_CRITICISED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRITICISED(v: Py) { ensure_init(); G_CRITICISED.with(|c| *c.borrow_mut() = v); }
pub fn g_CRITICISED_LINES() -> Py { ensure_init(); G_CRITICISED_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRITICISED_LINES(v: Py) { ensure_init(); G_CRITICISED_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CRITICISED_MY() -> Py { ensure_init(); G_CRITICISED_MY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRITICISED_MY(v: Py) { ensure_init(); G_CRITICISED_MY.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_TO_ME() -> Py { ensure_init(); G_DONE_TO_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_TO_ME(v: Py) { ensure_init(); G_DONE_TO_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT_POOL() -> Py { ensure_init(); G_DOUBT_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT_POOL(v: Py) { ensure_init(); G_DOUBT_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_EMAIL_HABIT() -> Py { ensure_init(); G_EMAIL_HABIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMAIL_HABIT(v: Py) { ensure_init(); G_EMAIL_HABIT.with(|c| *c.borrow_mut() = v); }
pub fn g_EVERY_TIME_POOL() -> Py { ensure_init(); G_EVERY_TIME_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVERY_TIME_POOL(v: Py) { ensure_init(); G_EVERY_TIME_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING_ON() -> Py { ensure_init(); G_GOING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING_ON(v: Py) { ensure_init(); G_GOING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_HABIT() -> Py { ensure_init(); G_GOOD_HABIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_HABIT(v: Py) { ensure_init(); G_GOOD_HABIT.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_HABIT_APPRECIATED() -> Py { ensure_init(); G_GOOD_HABIT_APPRECIATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_HABIT_APPRECIATED(v: Py) { ensure_init(); G_GOOD_HABIT_APPRECIATED.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_HABIT_INCLUDED() -> Py { ensure_init(); G_GOOD_HABIT_INCLUDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_HABIT_INCLUDED(v: Py) { ensure_init(); G_GOOD_HABIT_INCLUDED.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_WORDS() -> Py { ensure_init(); G_GOOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_WORDS(v: Py) { ensure_init(); G_GOOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOLD_THE_LINE() -> Py { ensure_init(); G_HOLD_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOLD_THE_LINE(v: Py) { ensure_init(); G_HOLD_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPS_THE_ARRANGEMENT() -> Py { ensure_init(); G_KEEPS_THE_ARRANGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPS_THE_ARRANGEMENT(v: Py) { ensure_init(); G_KEEPS_THE_ARRANGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_LATE_LINES() -> Py { ensure_init(); G_LATE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LATE_LINES(v: Py) { ensure_init(); G_LATE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT() -> Py { ensure_init(); G_LEFT_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT(v: Py) { ensure_init(); G_LEFT_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_MESS() -> Py { ensure_init(); G_MESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MESS(v: Py) { ensure_init(); G_MESS.with(|c| *c.borrow_mut() = v); }
pub fn g_MESSY() -> Py { ensure_init(); G_MESSY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MESSY(v: Py) { ensure_init(); G_MESSY.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_POOLS() -> Py { ensure_init(); G_MY_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_POOLS(v: Py) { ensure_init(); G_MY_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_THING() -> Py { ensure_init(); G_MY_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_THING(v: Py) { ensure_init(); G_MY_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ASKED() -> Py { ensure_init(); G_NOT_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ASKED(v: Py) { ensure_init(); G_NOT_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_GRUMBLE() -> Py { ensure_init(); G_NOT_A_GRUMBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_GRUMBLE(v: Py) { ensure_init(); G_NOT_A_GRUMBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_HELPED() -> Py { ensure_init(); G_NOT_HELPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_HELPED(v: Py) { ensure_init(); G_NOT_HELPED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_SINCERE() -> Py { ensure_init(); G_NOT_SINCERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_SINCERE(v: Py) { ensure_init(); G_NOT_SINCERE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THIS() -> Py { ensure_init(); G_NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THIS(v: Py) { ensure_init(); G_NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE() -> Py { ensure_init(); G_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE(v: Py) { ensure_init(); G_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_SET() -> Py { ensure_init(); G_PEOPLE_SET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_SET(v: Py) { ensure_init(); G_PEOPLE_SET.with(|c| *c.borrow_mut() = v); }
pub fn g_RAISE() -> Py { ensure_init(); G_RAISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RAISE(v: Py) { ensure_init(); G_RAISE.with(|c| *c.borrow_mut() = v); }
pub fn g_RECURS() -> Py { ensure_init(); G_RECURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECURS(v: Py) { ensure_init(); G_RECURS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_IT_POOL() -> Py { ensure_init(); G_SAID_IT_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_IT_POOL(v: Py) { ensure_init(); G_SAID_IT_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_ABOUT_ME() -> Py { ensure_init(); G_SAYS_ABOUT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_ABOUT_ME(v: Py) { ensure_init(); G_SAYS_ABOUT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_DOUBT() -> Py { ensure_init(); G_SELF_DOUBT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_DOUBT(v: Py) { ensure_init(); G_SELF_DOUBT.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE() -> Py { ensure_init(); G_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE(v: Py) { ensure_init(); G_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_STRAIN() -> Py { ensure_init(); G_STRAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRAIN(v: Py) { ensure_init(); G_STRAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_AGAIN() -> Py { ensure_init(); G_SUBJECT_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_AGAIN(v: Py) { ensure_init(); G_SUBJECT_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKEN() -> Py { ensure_init(); G_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKEN(v: Py) { ensure_init(); G_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKEN_FOR_GRANTED() -> Py { ensure_init(); G_TAKEN_FOR_GRANTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKEN_FOR_GRANTED(v: Py) { ensure_init(); G_TAKEN_FOR_GRANTED.with(|c| *c.borrow_mut() = v); }
pub fn g_THANKED() -> Py { ensure_init(); G_THANKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THANKED(v: Py) { ensure_init(); G_THANKED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_SAYING() -> Py { ensure_init(); G_THE_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_SAYING(v: Py) { ensure_init(); G_THE_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g_THREAD_TURNS() -> Py { ensure_init(); G_THREAD_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THREAD_TURNS(v: Py) { ensure_init(); G_THREAD_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNSEEN() -> Py { ensure_init(); G_UNSEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNSEEN(v: Py) { ensure_init(); G_UNSEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_WORDS_NOT_MEANT() -> Py { ensure_init(); G_WORDS_NOT_MEANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORDS_NOT_MEANT(v: Py) { ensure_init(); G_WORDS_NOT_MEANT.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_LINES() -> Py { ensure_init(); G_WORK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_LINES(v: Py) { ensure_init(); G_WORK_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_PEOPLE() -> Py { ensure_init(); G_WORK_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_PEOPLE(v: Py) { ensure_init(); G_WORK_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_WORDS() -> Py { ensure_init(); G_WORK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_WORDS(v: Py) { ensure_init(); G_WORK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Somebody else's habit, said out loud, with nothing else in the line.\n\nThis is the fallback's fallback. The repository's floor is three lines that fit any message\n(\"Got it. Is there anything you'd like a hand with?\"), and on main a person who has just\ndescribed the thing that wears them down is asked whether there is anything they would like a\nhand with - which is the one thing the line has just told you.\n\n    my brother keeps asking me for money        the neutral nod\n    my son keeps leaving his shoes in the hall  the neutral nod\n    my mate never invites me to anything        the neutral nod\n    my boss keeps moving the goalposts          the neutral nod\n    my partner keeps criticising me             the neutral nod\n\nThe composer already has a pool for this shape (`GRIEVANCE` in `bot/compose/fallback.py`), and it\nreaches the lines whose verb is in its list - borrows, cancels, criticises, rings. What it misses\nare the two commonest ways a person says it: *imposition* (\"asks me for money\", \"asks me to\nbabysit\", \"uses my toothbrush\", \"leaves her dishes in the sink\") and *neglect* (\"never invites me\",\n\"never thanks me\"). Both are the same complaint - the other person keeps doing something and the\nspeaker is the one paying for it - so they are read here, and the reply is the only help a\nfallback can honestly give for them: not a fact about the world, but the words to raise it.\n\nThe reply says the one thing that is true of every one of these lines, because it is the thing\npeople get wrong: say it once, about the pattern rather than the last time, and ask for the one\nchange you want by name. STYLE.md's \"Not knowing, honestly\" is the shape - the bot cannot see the\nbrother or the hall, so it does not pretend to know what happened; it hands over the sentence.\n\nTwo rules keep it away from everybody else's turns:\n\n- `bot/modules/greeting.py` only asks this when the reply it already has is one of the pools that\n  means nothing was answered (`fallback._is_a_hedge`, or one of the three NEUTRAL lines), so a\n  line any entry or module answered never reaches here and cannot change;\n- reading is narrow on purpose. There has to be a person in the line (\"my son\", \"the boss\"), the\n  line has to say it *keeps* happening, and one of the shapes below has to be in it. Everything\n  else - a body, an illness, a harm, a child's sleep, a pet, a good thing - stands down and keeps\n  the reply it had, because those want a different answer and other modules know them.\n\nNot every line this file can read is one it gets the turn on, and that is the gate doing its job:\n`my childminder keeps cancelling` is read as an arrangement they keep breaking, but the reply it\nalready has is the composer's \"Ugh, those are fiddly. How long has it been like that?\", which is\nneither a hedge nor a nod, so it keeps its turn. When a line here is measured, measure what the\nbot says, not what `answer_for` returns.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    fn __data1() -> Py { py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("parent"), py_str("parents"), py_str("sister"), py_str("brother"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("flatmate"), py_str("housemate"), py_str("roommate"), py_str("landlord"), py_str("landlady"), py_str("boss"), py_str("manager"), py_str("colleague"), py_str("coworker"), py_str("co-worker"), py_str("teammate"), py_str("team mate"), py_str("workmate"), py_str("neighbour"), py_str("neighbor"), py_str("son"), py_str("daughter"), py_str("kid"), py_str("child"), py_str("children"), py_str("friend"), py_str("mate"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("ex"), py_str("stepson"), py_str("stepdaughter"), py_str("stepdad"), py_str("stepmum"), py_str("mother-in-law"), py_str("father-in-law"), py_str("brother-in-law"), py_str("sister-in-law"), py_str("daughter-in-law"), py_str("son-in-law"), py_str("in-laws"), py_str("household"), py_str("family"), py_str("childminder"), py_str("babysitter"), py_str("nanny"), py_str("carer"), py_str("cleaner"), py_str("hairdresser"), py_str("tutor"), py_str("twin"), py_str("grandkid"), py_str("boy"), py_str("girl"), py_str("lad"), py_str("relative"), py_str("sibling"), py_str("stepkid")]) }
    s_PEOPLE(__data1());
    s_PEOPLE_SET(call_builtin("frozenset", &[g_PEOPLE()]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our|the)\\s+(?:[a-z-]+\\s+){0,2}?(?:"), &mcall(&py_str("|"), "join", &[g_PEOPLE()])), &py_str(")(?:s|es)?\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE(stdlib_call("re", "compile", &[py_str("\\b(?:someone|somebody|people|everyone|everybody)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_RECURS(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|kept|always|never|constantly|continually|forever|again and again|every (?:single |blessed )?(?:day|night|morning|week|weekend|time|month)|all the time|time after time|over and over|wont stop|won'?t stop|doesnt stop|doesn'?t stop|never stops)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_TO_ME(stdlib_call("re", "compile", &[py_str("\\b(?:ask|asks|asked|asking|tell|tells|told|telling|invite|invites|invited|inviting|thank|thanks|thanked|thanking|owe|owes|owed|blame|blames|blamed|criticis\\w*|criticiz\\w*|interrupt|interrupts|interrupted|interrupting|nag|nags|nagged|nagging|moan|moans|moaned|complain|complains|complained|ring|rings|ringing|rang|call|calls|calling|called|text|texts|texted|texting|message|messages|messaged|messaging|phone|phones|phoned|phoning|lecture|lectures|lectured|lecturing|mock|mocks|mocked|belittle|belittles|belittled|ignore|ignores|ignored|ignoring|talk|talks|talked|talking|shout|shouts|shouted|shouting|yell|yells|yelled|yelling|complain|moan|pick|picks|picked|picking)\\b(?:\\s+[a-z]+)?\\s+(?:me|us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FOR(stdlib_call("re", "compile", &[py_str("\\b(?:ask|asks|asked|asking)\\s+(?:me|us)\\s+(?:for|to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_ASKED(stdlib_call("re", "compile", &[py_str("\\b(?:never|doesn'?t|doesnt|don'?t|dont|won'?t|wont|not)\\b[^.?!]{0,26}?\\b(?:invit\\w*|includ\\w*|asks? me|ask me|asked me|picks? me|picks? up|leaves? me out|left me out)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_UNSEEN(stdlib_call("re", "compile", &[py_str("\\b(?:never|doesn'?t|doesnt|don'?t|dont|won'?t|wont|not)\\b[^.?!]{0,26}?\\b(?:thank\\w*|notic\\w*|appreciat\\w*|listen\\w*|consider\\w*|repl\\w*|respond\\w*|visits?|visit me|talks? to me|talked to me)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_HELPED(stdlib_call("re", "compile", &[py_str("\\b(?:never|doesn'?t|doesnt|don'?t|dont|won'?t|wont)\\b[^.?!]{0,26}?\\b(?:helps?|helping|pulls? (?:his|her|their) weight|does (?:his|her|their) share)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MY_THING(stdlib_call("re", "compile", &[py_str("\\b(?:borrow|borrows|borrowed|borrowing|take|takes|took|taking|use|uses|used|using|nick|nicks|nicked|nicking|eat|eats|ate|eating|drink|drinks|drank|drinking|spend|spends|spent|spending|wear|wears|wearing|wore|hide|hides|hid|hoard|hoards|fill|fills|filled|move|moves|moved|break|breaks|broke|break|throw|throws|threw|smash|smashes)\\b[^.?!]{0,20}?\\b(?:my|our)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MESS(stdlib_call("re", "compile", &[py_str("\\b(?:everywhere|all over|all over the place|lying around|left around|in the way|in (?:my|our) way|in the (?:hall|hallway|kitchen|sink|living room|lounge|bathroom|bath|bedroom|fridge|bin|garden|yard|car|sofa|cupboard|garage|utility|passage|porch)|on the (?:floor|table|sofa|bed|counter|stairs|side)|up the stairs|on the stairs|out the front|outside the front|in the passage)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("drives me mad"), py_str("drives me nuts"), py_str("drives me crazy"), py_str("drives me insane"), py_str("drives me up the wall"), py_str("does my head in"), py_str("does my nut in"), py_str("gets to me"), py_str("getting to me"), py_str("winds me up"), py_str("wound me up"), py_str("it annoys me"), py_str("annoys me"), py_str("annoying me"), py_str("sick of"), py_str("fed up"), py_str("cant stand"), py_str("can't stand"), py_str("wearing me down"), py_str("wears me down"), py_str("worn me out"), py_str("wears me out"), py_str("boils my blood"), py_str("grates on me"), py_str("had enough of"), py_str("draining me")]) }
    s_STRAIN(__data2());
    s_CRITICISED(stdlib_call("re", "compile", &[py_str("\\b(?:criticis\\w*|criticiz\\w*|comment\\w*|picks? on|picked on|picking on|puts? me down|putting me down|makes? fun of|made fun of|moan\\w*|complain\\w* about|lectur\\w*|tells? me how|tells? me what|remind\\w* me|nag\\w*|judge\\w*|belittl\\w*|mocks?|mocked|rubs? it in)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CRITICISED_MY(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:[a-z]+\\s+){0,2}?(?:weight|looks?|cooking|house|parenting|driving|job|money|hair|clothes|lifestyle|accent|english|kids?|children|body|figure|age)\\b|\\b(?:how|the way)\\s+(?:i|we)\\s+(?:look|dress|speak|talk|walk|eat|spend|parent|drive)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BAILS(stdlib_call("re", "compile", &[py_str("\\b(?:cancel\\w*|cancell\\w*|bails?|bailed|bailing|pulls? out|pulled out|pulling out|drops? out|dropped out|dropping out|lets? me down|let me down|letting me down|messes? me about|messing me about|mess(?:es)? me around|messing me around|not turn(?:ing)? up|doesn'?t turn up|didn'?t turn up|stands? me up|stood me up|no[- ]?shows?|no[- ]?showed|changes? (?:the|our) plans?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_THE_ARRANGEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:never|doesn'?t|doesnt|don'?t|dont|won'?t|wont)\\b[^.?!]{0,20}?\\b(?:cancel\\w*|cancell\\w*|bails?|lets? me down|stands? me up|no[- ]?shows?|late|brings? (?:it|that) up)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ALWAYS_DOES_IT(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|kept|always|constantly|continually|every\\s+(?:single\\s+)?(?:day|week|time))\\b[^.?!]{0,26}?\\b(?:invit\\w*|includ\\w*|thank\\w*|say\\w*\\s+thank\\w*|ask(?:s|ing)? me (?:over|round|out|along|to)\\b|asks? after me\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_THANKED(stdlib_call("re", "compile", &[py_str("\\bthank\\w*|\\bthank you\\b|\\bsays?\\s+(?:a\\s+)?thank")]).unwrap_or_else(|e| exc::raise(e)));
    s_COURTESY(stdlib_call("re", "compile", &[py_str("\\b(?:thank\\w*|invit\\w*|includ\\w*|asks? me (?:over|round|out|along))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COURTESY_DENIED(stdlib_call("re", "compile", &[py_str("\\b(?:never|not|doesn'?t|doesnt|don'?t|dont|won'?t|wont|hardly|rarely|only when)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_SINCERE(stdlib_call("re", "compile", &[py_str("\\b(?:never|doesn'?t|doesnt|didn'?t|didnt|don'?t|dont|won'?t|wont|not)\\s+(?:really\\s+|ever\\s+|actually\\s+|truly\\s+)?mean(?:s|t)?\\s+(?:it|a word of it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SHARED_OCCASION(stdlib_call("re", "compile", &[py_str("\\bask(?:s|ing)? me (?:to |round for |over for )?(?:dinner|lunch|supper|tea|a meal|a drink|drinks|a party|a barbecue|a bbq|a picnic|a night out|a wedding|a do)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ARRIVES_LATE(stdlib_call("re", "compile", &[py_str("\\b(?:turns? up late|turning up late|shows? up late|showing up late|arriv\\w* late|arrives? late|always late|never on time|keeps? me waiting|every time we meet| timekeeping)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_ON(stdlib_call("re", "compile", &[py_str("\\b(?:goes? on about|going on about|goes? on and on|going on and on|keeps? on about|bangs? on about|harks? on about|keeps? bringing (?:it|that) up|brings? (?:it|that) up|wont drop it|won'?t drop it|never drops it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EMAIL_HABIT(stdlib_call("re", "compile", &[py_str("\\b(?:reply(?:ing|ies)? all|replies all|cc'?s? (?:everyone|the whole|us all)|copies? (?:in )?(?:everyone|the whole team|the lot)|forwarding everything|forwards everything)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORK_PEOPLE(call_builtin("frozenset", &[py_tuple(vec![py_str("boss"), py_str("manager"), py_str("colleague"), py_str("coworker"), py_str("co-worker"), py_str("teammate"), py_str("team mate"), py_str("workmate")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("goalpost"), py_str("goal"), py_str("goals"), py_str("deadline"), py_str("rota"), py_str("shift"), py_str("1:1"), py_str("one to one"), py_str("one-to-one"), py_str("meeting"), py_str("brief"), py_str("priority"), py_str("priorities"), py_str("workload"), py_str("target"), py_str("overtime"), py_str("annual leave"), py_str("holiday"), py_str("pay rise"), py_str("promotion"), py_str("review"), py_str("appraisal"), py_str("micromanage"), py_str("micromanaging"), py_str("email"), py_str("inbox"), py_str("projects?")]) }
    s_WORK_WORDS(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("fall"), py_str("falls"), py_str("falling"), py_str("fell"), py_str("fallen"), py_str("cough"), py_str("coughs"), py_str("coughing"), py_str("cold"), py_str("flu"), py_str("sick"), py_str("ill"), py_str("unwell"), py_str("pain"), py_str("pains"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("ache"), py_str("aches"), py_str("dizzy"), py_str("faint"), py_str("fainting"), py_str("forget"), py_str("forgets"), py_str("forgetting"), py_str("forgot"), py_str("memory"), py_str("confused"), py_str("dementia"), py_str("alzheimers"), py_str("alzheimer's"), py_str("hospital"), py_str("doctor"), py_str("gp"), py_str("nurse"), py_str("tablets"), py_str("medication"), py_str("medicine"), py_str("chemo"), py_str("cancer"), py_str("stroke"), py_str("operation"), py_str("surgery"), py_str("bleed"), py_str("bleeding"), py_str("blood"), py_str("bruise"), py_str("bruises"), py_str("lump"), py_str("rash"), py_str("fever"), py_str("vomit"), py_str("vomiting"), py_str("hospital"), py_str("hit"), py_str("hits"), py_str("hitting"), py_str("punch"), py_str("punches"), py_str("kick"), py_str("kicks"), py_str("slap"), py_str("slaps"), py_str("grabbed"), py_str("grabs"), py_str("touched"), py_str("touches"), py_str("touching"), py_str("threaten"), py_str("threatens"), py_str("threatened"), py_str("abuse"), py_str("abusive"), py_str("bullies"), py_str("bullying"), py_str("bullied"), py_str("drunk"), py_str("drinks"), py_str("smokes"), py_str("smoking"), py_str("suicide"), py_str("suicidal"), py_str("self harm"), py_str("kill"), py_str("starving"), py_str("anorexic"), py_str("depressed"), py_str("depression"), py_str("anxious"), py_str("anxiety"), py_str("panic"), py_str("therapy"), py_str("therapist"), py_str("counselling"), py_str("counseling"), py_str("appointment"), py_str("appointments")]) }
    s_NOT_THIS(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("waking"), py_str("wakes"), py_str("woke"), py_str("sleep"), py_str("sleeps"), py_str("sleeping"), py_str("crying"), py_str("cries"), py_str("cried"), py_str("nappy"), py_str("nappies"), py_str("feeding"), py_str("feeds"), py_str("tantrum"), py_str("tantrums"), py_str("teething"), py_str("barking"), py_str("barks"), py_str("howling"), py_str("scratching"), py_str("litter"), py_str("bites"), py_str("biting"), py_str("chewing"), py_str("jumping up"), py_str("soiling"), py_str("weeing"), py_str("pooing")]) }
    s_NOT_A_GRUMBLE(call_builtin("frozenset", &[__data5()]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_PEOPLE(call_builtin("frozenset", &[py_tuple(vec![py_str("son"), py_str("daughter"), py_str("kid"), py_str("child"), py_str("children"), py_str("stepson"), py_str("stepdaughter"), py_str("teenager"), py_str("teenagers"), py_str("toddler"), py_str("baby")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_tuple(vec![py_str("nicely"), py_str("kindly"), py_str("sweet"), py_str("lovely"), py_str("proud"), py_str("funny"), py_str("jokes"), py_str("joking"), py_str("hilarious"), py_str("thoughtful"), py_str("supportive"), py_str("helpful"), py_str("amazing"), py_str("brilliant"), py_str("wonderful"), py_str("affectionate"), py_str("cuddly"), py_str("cuddles")]) }
    s_GOOD_WORDS(call_builtin("frozenset", &[__data6()]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKED(py_tuple(vec![py_str("Decide your line before the next ask rather than in the moment, and make it one sentence: what you can do and what you can't. 'I can do Saturday, but not every Saturday' is a thing somebody can plan around, and you don't owe a reason."), py_str("Say it once, calmly, and make it about the thing rather than the person: 'I'd rather you asked me first.' One clear line lands where a hint does not, and it is kinder than going quiet about it."), py_str("Give them an answer they can work with: what you can do, and what you can't. 'I can help with the school run, but I can't lend money again' is a line somebody can plan around.")]));
    s_TAKEN(py_tuple(vec![py_str("Make it one rule rather than one row, said once and warmly the next time they ask: 'of course, if it's back by Sunday.' A rule they agree to is easier to hold than a complaint about the last time."), py_str("Ask before the next time rather than after: 'can we agree I'm asked first, even if the answer is going to be yes?' It costs nothing when they would have asked anyway."), py_str("Keep the two or three things you actually mind out of reach and let the rest go. Then the one sentence - 'anything of mine, just ask' - covers the lot without a list of every time.")]));
    s_MESSY(py_tuple(vec![py_str("Give the thing one place to go rather than asking for tidiness in general: 'the shoes live in the cupboard by the door.' A named place is something a person can do; 'clear up after yourself' is not."), py_str("Ask for the one job, once, at a calm moment - 'can the dishes go in the dishwasher before bed?' - rather than for a change in character. One job named is one job done."), py_str("Say it as the house rule rather than as a complaint about them: 'in this house the boots come off at the door.' A rule is easier to hear than a list of the times they didn't.")]));
    s_LEFT_OUT(py_tuple(vec![py_str("Say it once, plainly, as a wish rather than an accusation: 'I'd love to be asked next time, even at short notice.' People often don't know they've left somebody out, and one sentence is usually enough."), py_str("Ask for the next one rather than for the reason: 'can I come to the next one?' A straight yes or no tells you more than any hint will, and it is easy for them to say yes to."), py_str("Worth saying out loud once - 'I've noticed I don't get asked any more, and it does get to me' - and then letting them answer. The answer is the useful part, whatever it is.")]));
    s_TAKEN_FOR_GRANTED(py_tuple(vec![py_str("Say what you would like noticed rather than what they didn't do: 'a thanks when I've done the shopping would go a long way.' People often don't know which bit you want acknowledged, and a thing named is something they can do."), py_str("Ask for it once, plainly and without a score kept: 'it would help if you said when it's good.' Then notice it when it comes, because the thanks you asked for is the one they are learning to give."), py_str("Worth saying what the effect is, once: 'I do a lot round here and it goes unnoticed, and that is what wears me down.' It is easier to hear than 'you never thank me'.")]));
    s_CRITICISED_LINES(py_tuple(vec![py_str("Say it once, about one thing, in the moment: 'please don't comment on that - it isn't helping.' You don't have to justify it, and not liking it is reason enough."), py_str("Name the thing and the effect in one sentence - 'when you say that, I stop wanting to tell you things' - and leave it there. It is a statement, not an argument to win."), py_str("Answer flatly and warmly the first time, then give the same short answer every time after: 'I heard you, and I'm not changing it.' Repetition is what ends it, not a better argument.")]));
    s_WORK_LINES(py_tuple(vec![py_str("Get it in writing and ask what moved: 'just to confirm, it's Tuesday now - what changed?' A written thread is what you can point at the next time it shifts, and it costs one email."), py_str("Ask what moved and what comes off instead of why it moved: 'if this is the new priority, what drops?' That hands the choice to the person who can actually make it."), py_str("Raise it once, at the end of a meeting, in plain words: 'when the plan changes twice a week I lose time on the rest.' Say what it costs you, not that it is annoying.")]));
    s_CREDIT_LINES(py_tuple(vec![py_str("Say what you did, in one sentence, in the room where it comes up: 'I did the analysis behind that.' Credit follows a clear claim, and the quiet hope that somebody notices is what leaves it with them."), py_str("Ask for the part you want named rather than the whole thing: 'can the write-up say who did what?' It is a small, reasonable request, and it is much easier to say yes to than a row about taking credit."), py_str("Keep a short note of what you did and when - not to send to anyone, but so you can say it plainly the next time it matters. Then say it once, warmly and without the charge: 'that was mine, and I'd like that known.'")]));
    s_GOOD_HABIT_APPRECIATED(py_tuple(vec![py_str("That is rarer than people think, and worth saying out loud once: 'I noticed you always say thank you, and it makes a difference.' People hear the complaint version far more often than the good one."), py_str("It is worth naming to their face rather than just enjoying: 'you always notice, and I appreciate it.' A thank-you that goes unmentioned tends to stop, because nobody can tell it landed."), py_str("Say it back in the same plain way, in the moment: 'I appreciate that you always think to say it.' One sentence is the whole of what it takes.")]));
    s_GOOD_HABIT_INCLUDED(py_tuple(vec![py_str("That is a good habit to be on the receiving end of, and worth naming to their face: 'I noticed you always think to ask me.' People rarely hear the good version of a habit, and it is usually never said."), py_str("Say yes, and say why once: 'I like that you always think to include me.' The person who keeps asking is usually the one who never gets told that it lands."), py_str("Enjoy it and say so in the moment rather than leaving it as a thing you only notice - 'I noticed, and I'm glad of it.' It costs nothing and it usually lands better than people expect.")]));
    s_GOOD_HABIT(op_add(&g_GOOD_HABIT_APPRECIATED(), &g_GOOD_HABIT_INCLUDED()));
    s_WORDS_NOT_MEANT(py_tuple(vec![py_str("Weigh what they do against what they say, because a thank-you nobody means is the cheapest thing there is to give. If nothing else in it matches the words, stop going out of your way for a while and see what they notice."), py_str("Say the plain version once, without heat: 'I'd rather you said nothing than said it out of habit.' Either answer tells you more than the words did."), py_str("Ask for the thing rather than the words: 'it would mean more if you helped with it than if you thanked me for it.' Somebody who won't do the thing has told you what the words are worth.")]));
    s_RAISE(py_tuple(vec![py_str("Say it once, at a calm time, about the pattern rather than today: 'this keeps happening and it's starting to get to me - can we sort it?' Name the thing they do and the one change you want; a list of every time turns it into a row."), py_str("Raise it early rather than on the tenth time, and ask for the one change you actually want. 'I'd rather you asked first' is something a person can act on; 'stop it' is not."), py_str("Worth one plain sentence to them, said warmly and once. People can act on a request - 'can we agree the boots go by the door?' - not on a sigh.")]));
    s_BAILING(py_tuple(vec![py_str("Say it plainly once, and name the pattern rather than the last time: 'you've cancelled three times now and it leaves me stuck - can we agree a week's notice?' Then say what you will do if it happens again, and do that."), py_str("Put a fallback in place before the next time, so the cost lands on them rather than on you: a backup for the childcare, somebody else who can drive, a plan that does not need them to turn up. Nothing about this changes until cancelling costs them something."), py_str("It is fair to ask why, once: 'is something making it hard at the moment?' If there is a reason you can both work with, say what would help; if there is not, it is worth saying the arrangement no longer works for you.")]));
    s_LATE_LINES(py_tuple(vec![py_str("Agree the time out loud, then say what happens if it slips: 'if you're not here by seven I'll start without you.' A consequence you actually keep is what changes a habit; being annoyed on the night does not."), py_str("Say it once, without the sarcasm, in the calmest voice you have: 'when you're late it means I am waiting, and I'd rather we set a time we both mean.' Then stop being the person who waits quietly."), py_str("Ask for a message rather than for punctuality, because it is the waiting that costs you: 'if you'll be more than ten minutes, text me.' That is easy to say yes to, and it gets you the thing you actually wanted.")]));
    s_SUBJECT_AGAIN(py_tuple(vec![py_str("Ask what they want from the conversation: 'when you bring it up, is there something you want me to say?' Somebody who goes over the same ground is usually after an answer they have not had yet, and naming that ends it quicker than changing the subject."), py_str("Say where the line is once, in a warm voice: 'I've said all I can about that one - I'd rather talk about something else tonight.' Then steer, gently, to something you both enjoy.")]));
    fn __data7() -> Py { py_tuple(vec![py_str("went quiet"), py_str("didnt say anything"), py_str("didn't say anything"), py_str("said nothing"), py_str("got annoyed"), py_str("got defensive"), py_str("got funny"), py_str("got upset"), py_str("shrugged"), py_str("laughed it off"), py_str("laughed"), py_str("said sorry"), py_str("said they were sorry"), py_str("agreed"), py_str("promised"), py_str("said ok"), py_str("said okay"), py_str("didnt listen"), py_str("didn't listen"), py_str("ignored it"), py_str("did it again"), py_str("kept doing it"), py_str("went mad"), py_str("kicked off"), py_str("stormed off"), py_str("hung up"), py_str("walked off"), py_str("denied it")]) }
    s_AFTER_A_SAYING(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("i said"), py_str("i told"), py_str("i asked"), py_str("ive said"), py_str("i've said"), py_str("ive told"), py_str("i've told"), py_str("i explained"), py_str("i mentioned"), py_str("i brought it up"), py_str("i tried"), py_str("i texted"), py_str("i wrote")]) }
    s_THE_SAYING(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("every single day"), py_str("every single time"), py_str("every day"), py_str("every time"), py_str("all the time"), py_str("constantly"), py_str("again and again"), py_str("over and over"), py_str("non stop"), py_str("non-stop"), py_str("daily"), py_str("every week"), py_str("every night"), py_str("without fail"), py_str("for years"), py_str("so often")]) }
    s_ALWAYS_COUNT(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("am i overreacting"), py_str("am i being too sensitive"), py_str("am i being sensitive"), py_str("am i over thinking"), py_str("am i overthinking"), py_str("am i being unreasonable"), py_str("is it me"), py_str("should i just give up"), py_str("should i give up"), py_str("am i mad"), py_str("am i wrong to"), py_str("is that normal"), py_str("am i being daft"), py_str("am i being silly"), py_str("too sensitive")]) }
    s_SELF_DOUBT(__data10());
    s_ABOUT_ME(py_tuple(vec![py_str("Ask for one example with a time and a place - 'when was the last time?' - instead of arguing about all of you. A real instance tells you what to change; a verdict does not."), py_str("Take the grain of truth and say what you would like instead: 'you're right that I've been on my phone at dinner - I'll put it down.' That ends it faster than a defence does, and it does not mean agreeing with all of it."), py_str("Ask what they would like more of rather than whether they are right: 'what would a good evening look like for you?' That is a question with an answer, and a verdict is not."), py_str("Say it back before you answer, so you both know what is being said: 'so you feel like I'm not really here - is that it?' A fair amount of these turn out to be about one bad week."), py_str("Give it a hearing without signing up to all of it: 'some of that lands and some of it doesn't - can we talk about the bit that matters most?' Then do that one thing.")]));
    s_SAID_IT_POOL(py_tuple(vec![py_str("Then you have said it, which is the hard half. Say the same sentence again next time, in the same words - a line that comes back identical is what makes it a rule rather than a mood, and it is the explaining that gives them something to argue with."), py_str("The first time is usually the one that gets argued with, so say it once more the same way and then let it sit. Nothing changes in the moment; what changes is that they now know what you will say."), py_str("A quiet reply to that is normal, and it is not the answer yet. Hold the line exactly as you put it - 'I can't do that' - and leave the door open for the next time: 'ask me again about Saturday and I'll say yes'.")]));
    s_EVERY_TIME_POOL(py_tuple(vec![py_str("Then it is a pattern rather than a bad week, and worth naming as one: 'this happens most days' lands differently from 'you did it again'. Ask for the smallest version of it - one thing, one place - because a whole list is easy to refuse and one thing is not."), py_str("Each time it happens, say the same short line and nothing else. It feels repetitive to you because you are the one saying it; being boring about it is exactly what makes it stick.")]));
    s_DOUBT_POOL(py_tuple(vec![py_str("It is not you being sensitive: a small thing that happens every day is a different thing from a small thing that happens once. Being worn down by the repetition is the honest response to it, and it is worth saying as its own sentence."), py_str("You are not asking for much - you are asking for the same small thing twice - so it is worth one clear go before you decide anything. If nothing changes after you have said it plainly, that is information about them rather than about you."), py_str("A thing can be small and still wear you down when it lands every day. Say the one sentence you would say to a friend in the same spot, and take your own side in it.")]));
    s_HOLD_THE_LINE(py_tuple(vec![py_str("The line stays the same, and it is worth saying it once more the same way: what you can do, and what you can't. You don't have to give a reason twice."), py_str("Then it is still the one plain sentence, said warmly and once: 'I can't do that.' After that the answer is no, and it does not need explaining again."), py_str("Worth naming it once, plainly, rather than hinting: 'this is happening a lot and I'd like it to change.' Then leave the next move with them, which is where it belongs.")]));
    fn __data11() -> Py { py_tuple(vec![g_ASKED(), g_TAKEN(), g_MESSY(), g_LEFT_OUT(), g_TAKEN_FOR_GRANTED(), g_CRITICISED_LINES(), g_WORK_LINES(), g_BAILING(), g_LATE_LINES(), g_SUBJECT_AGAIN(), g_RAISE(), g_ABOUT_ME(), g_GOOD_HABIT(), g_WORDS_NOT_MEANT()]) }
    s_MY_POOLS(__data11());
    s_THREAD_TURNS(Py::Int(3));
    s_SAYS_ABOUT_ME(stdlib_call("re", "compile", &[py_str("\\b(?:says?|said|tells?|told|thinks?|reckons?|calls?)\\s+(?:me\\s+|everyone\\s+|people\\s+)?(?:that\\s+)?(?:i'?m|im|i am|i never|i always|i dont|i don'?t|i only|i just)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CREDIT(stdlib_call("re", "compile", &[py_str("\\b(?:take|takes|took|taking|get|gets|got|getting)\\s+(?:all\\s+)?(?:the\\s+)?credit\\b|\\b(?:takes|took|getting|gets)\\s+the\\s+blame\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _in_my_pools(args: &[Py]) -> Py {
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
    let mut pool: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a bot line is one of this file's own, so the thread is still open.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MY_POOLS()) { let mut pool: Py = __item.clone(); for __item in py_iter_or_raise(&pool.clone()) { let mut reply: Py = __item.clone(); __out.push(mcall(&said.clone(), "startswith", &[slice_or_raise(&reply.clone(), &Py::None, &Py::Int(30), &Py::None)])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _thread_open(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last line in this conversation came out of one of these pools.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&g_THREAD_TURNS()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    return _in_my_pools(&[line.clone()]);
    }
    return Py::Bool(false);
    Py::None
}
pub fn holds_the_thread(args: &[Py]) -> Py {
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
    let _ = py_str("True when the bot's last line was one of this file's, so a short reply is still this line.\n\n    The receipt the composer gives a second turn (\"Got it - it's every single time\") is the\n    person's own sentence handed back, which is the same class of placeholder as the nod: it puts\n    nothing on the table. `bot/modules/greeting.py` asks this before replacing one, so a receipt\n    in any other kind of thread keeps the turn.\n    ");
    return _thread_open(&[memory.clone()]);
    Py::None
}
pub fn _follow_up(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asking_about_itself: Py = Py::None;
    let mut outcome: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut tried: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The second turn of the same complaint, or None when it is not one.");
    if py_truthy(&Py::Bool(!py_truthy(&_thread_open(&[memory.clone()])))) {
    return Py::None;
    }
    words = mcall(&clean.clone(), "split", &[]);
    asking_about_itself = stdlib_call("re", "search", &[py_str("\\b(?:you|your|the bot|this bot)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&asking_about_itself.clone()) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SELF_DOUBT()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g_DOUBT_POOL();
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ALWAYS_COUNT()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return g_EVERY_TIME_POOL();
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:what|how)\\b[^?]{0,30}\\b(?:say|tell|word|phrase|put it|bring it up)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    tried = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_SAYING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    outcome = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AFTER_A_SAYING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = { let __b = tried.clone(); if py_truthy(&__b) { __b } else { outcome.clone() } }; if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } }) {
    return g_SAID_IT_POOL();
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:says? (?:she|he|they)'?ll|said (?:she|he|they)'?d|promised|says? (?:she|he|they) (?:will|would)|doesn'?t|didn'?t|won'?t)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } }) {
    return g_SAID_IT_POOL();
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:he|she|they|him|her|them)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&clean.clone(), &py_str("?"))) } } } }) {
    return g_HOLD_THE_LINE();
    }
    return Py::None;
    Py::None
}
pub fn _the_one(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut cut: Py = Py::None;
    let _ = py_str("The person with a plural taken off, so \"my kids\" is read as a kid and a boss as a boss.\n\n    The child and work readings are keyed on the noun, and a line that says \"the kids keep going\n    on about minecraft\" is the same child as \"my kid keeps going on about it\".\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CHILD_PEOPLE(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WORK_PEOPLE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PEOPLE_SET(), &word.clone())) } } } }) {
    return word.clone();
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None)]));
    for __item in __iter1 {
    cut = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CHILD_PEOPLE(), &cut.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WORK_PEOPLE(), &cut.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PEOPLE_SET(), &cut.clone())) } } } }) {
    return cut.clone();
    }
    }
    return word.clone();
    Py::None
}
pub fn _person_named(args: &[Py]) -> Py {
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
    let _ = py_str("The person the line is about, as they were named, or \"\".\n\n    Kept so the work reading can tell a boss from a brother without matching the line twice.\n    ");
    match_ = mcall(&g_A_PERSON(), "search", &[clean.clone()]);
    if py_truthy(&match_.clone()) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    return py_str("");
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
    let mut person: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut last: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Which pool answers this line, or None when it is not this file's turn.\n\n    The order matters: work first (a boss is not a brother), then being criticised, then the\n    three ways of not being asked, then the thing in the line itself, and the general raising-it\n    pool last, so a line that names nothing beyond the pattern still gets the words.\n    ");
    last = if py_truthy(&person.clone()) { _the_one(&[geti(&mcall(&person.clone(), "split", &[]), &op_neg(&Py::Int(1)))]) } else { py_str("") };
    if py_truthy(&mcall(&g_NOT_SINCERE(), "search", &[clean.clone()])) {
    return g_WORDS_NOT_MEANT();
    }
    if py_truthy(&{ let __b = mcall(&g_ALWAYS_DOES_IT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_BAILS(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_SHARED_OCCASION(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASK_FOR(), "search", &[clean.clone()]))) } } } } } }) {
    return if py_truthy(&mcall(&g_THANKED(), "search", &[clean.clone()])) { g_GOOD_HABIT_APPRECIATED() } else { g_GOOD_HABIT_INCLUDED() };
    }
    if py_truthy(&{ let __b = mcall(&g_COURTESY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_COURTESY_DENIED(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_BAILS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = _strained(&[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CRITICISED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_ASKED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_UNSEEN(), "search", &[clean.clone()]) } } } } } } } })) } } } }) {
    return if py_truthy(&mcall(&g_THANKED(), "search", &[clean.clone()])) { g_GOOD_HABIT_APPRECIATED() } else { g_GOOD_HABIT_INCLUDED() };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WORK_PEOPLE(), &last.clone())); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WORK_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&g_DONE_TO_ME(), "search", &[clean.clone()]) } } } }) {
    return g_WORK_LINES();
    }
    if py_truthy(&mcall(&g_SAYS_ABOUT_ME(), "search", &[clean.clone()])) {
    return g_ABOUT_ME();
    }
    if py_truthy(&{ let __b = mcall(&g_CRITICISED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_DONE_TO_ME(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CRITICISED_MY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SAYS_ABOUT_ME(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { _strained(&[clean.clone()]) } } } } } } } }) {
    return g_CRITICISED_LINES();
    }
    if py_truthy(&mcall(&g_CREDIT(), "search", &[clean.clone()])) {
    return g_CREDIT_LINES();
    }
    if py_truthy(&mcall(&g_NOT_ASKED(), "search", &[clean.clone()])) {
    return g_LEFT_OUT();
    }
    if py_truthy(&mcall(&g_UNSEEN(), "search", &[clean.clone()])) {
    return g_TAKEN_FOR_GRANTED();
    }
    if py_truthy(&mcall(&g_ASK_FOR(), "search", &[clean.clone()])) {
    return g_ASKED();
    }
    if py_truthy(&{ let __b = mcall(&g_MESS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:leaves?|left|leaving|drops?|dropped|dumping|dumps|put|puts|putting)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return g_MESSY();
    }
    if py_truthy(&mcall(&g_MY_THING(), "search", &[clean.clone()])) {
    return g_TAKEN();
    }
    if py_truthy(&mcall(&g_NOT_HELPED(), "search", &[clean.clone()])) {
    return g_RAISE();
    }
    if py_truthy(&{ let __b = mcall(&g_BAILS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_KEEPS_THE_ARRANGEMENT(), "search", &[clean.clone()]))) } }) {
    return g_BAILING();
    }
    if py_truthy(&{ let __b = mcall(&g_ARRIVES_LATE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_KEEPS_THE_ARRANGEMENT(), "search", &[clean.clone()]))) } }) {
    return g_LATE_LINES();
    }
    if py_truthy(&{ let __b = mcall(&g_GOING_ON(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_CHILD_PEOPLE(), &last.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_KEEPS_THE_ARRANGEMENT(), "search", &[clean.clone()]))) } } } }) {
    return g_SUBJECT_AGAIN();
    }
    if py_truthy(&{ let __b = mcall(&g_EMAIL_HABIT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WORK_PEOPLE(), &last.clone())) } }) {
    return g_RAISE();
    }
    if py_truthy(&mcall(&g_DONE_TO_ME(), "search", &[clean.clone()])) {
    return g_CRITICISED_LINES();
    }
    if py_truthy(&_strained(&[clean.clone()])) {
    return g_RAISE();
    }
    return Py::None;
    Py::None
}
pub fn _said_at_work(args: &[Py]) -> Py {
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
    let mut person: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut last: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a line about the boss or a colleague names a thing that changes at work.");
    last = if py_truthy(&person.clone()) { _the_one(&[geti(&mcall(&person.clone(), "split", &[]), &op_neg(&Py::Int(1)))]) } else { py_str("") };
    return { let __b = Py::Bool(py_contains(&g_WORK_PEOPLE(), &last.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WORK_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _strained(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STRAIN()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut follow: Py = Py::None;
    let mut person: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut someone: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words to raise it with somebody, or None when this is not that shape.\n\n    Asked by `bot/modules/greeting.py` only where the composer's reply is a placeholder, so a\n    None here keeps the reply the person would have had.\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(30)) } }) {
    return Py::None;
    }
    memory = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) };
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&words.clone(), &g_NOT_THIS())) {
    return Py::None;
    }
    follow = _follow_up(&[clean.clone(), memory.clone()]);
    if py_truthy(&follow.clone()) {
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[follow.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_RECURS(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_SINCERE(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    someone = mcall(&g_SOMEONE(), "search", &[clean.clone()]);
    person = { let __b = _person_named(&[clean.clone()]); if py_truthy(&__b) { __b } else { if py_truthy(&someone.clone()) { mcall(&someone.clone(), "group", &[Py::Int(0)]) } else { py_str("") } } };
    if py_truthy(&Py::Bool(!py_truthy(&person.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_NOT_A_GRUMBLE()); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_GOOD_WORDS()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&clean.clone(), "endswith", &[py_str("?")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:what|how)\\b[^?]{0,24}\\b(?:say|tell|word|phrase|put it|raise it)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_DONE_TO_ME(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MY_THING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MESS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_ASKED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_UNSEEN(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_HELPED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CRITICISED_MY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SAYS_ABOUT_ME(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CREDIT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = _strained(&[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_SINCERE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BAILS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ARRIVES_LATE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_GOING_ON(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_EMAIL_HABIT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_COURTESY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = _said_at_work(&[clean.clone(), person.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ALWAYS_DOES_IT(), "search", &[clean.clone()]) } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }))) {
    return Py::None;
    }
    pool = _shape(&[clean.clone(), person.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.habit".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_ME" => Some(g_ABOUT_ME()),
            "AFTER_A_SAYING" => Some(g_AFTER_A_SAYING()),
            "ALWAYS_COUNT" => Some(g_ALWAYS_COUNT()),
            "ALWAYS_DOES_IT" => Some(g_ALWAYS_DOES_IT()),
            "ARRIVES_LATE" => Some(g_ARRIVES_LATE()),
            "ASKED" => Some(g_ASKED()),
            "ASK_FOR" => Some(g_ASK_FOR()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_SHARED_OCCASION" => Some(g_A_SHARED_OCCASION()),
            "BAILING" => Some(g_BAILING()),
            "BAILS" => Some(g_BAILS()),
            "CHILD_PEOPLE" => Some(g_CHILD_PEOPLE()),
            "COURTESY" => Some(g_COURTESY()),
            "COURTESY_DENIED" => Some(g_COURTESY_DENIED()),
            "CREDIT" => Some(g_CREDIT()),
            "CREDIT_LINES" => Some(g_CREDIT_LINES()),
            "CRITICISED" => Some(g_CRITICISED()),
            "CRITICISED_LINES" => Some(g_CRITICISED_LINES()),
            "CRITICISED_MY" => Some(g_CRITICISED_MY()),
            "DONE_TO_ME" => Some(g_DONE_TO_ME()),
            "DOUBT_POOL" => Some(g_DOUBT_POOL()),
            "EMAIL_HABIT" => Some(g_EMAIL_HABIT()),
            "EVERY_TIME_POOL" => Some(g_EVERY_TIME_POOL()),
            "GOING_ON" => Some(g_GOING_ON()),
            "GOOD_HABIT" => Some(g_GOOD_HABIT()),
            "GOOD_HABIT_APPRECIATED" => Some(g_GOOD_HABIT_APPRECIATED()),
            "GOOD_HABIT_INCLUDED" => Some(g_GOOD_HABIT_INCLUDED()),
            "GOOD_WORDS" => Some(g_GOOD_WORDS()),
            "HOLD_THE_LINE" => Some(g_HOLD_THE_LINE()),
            "KEEPS_THE_ARRANGEMENT" => Some(g_KEEPS_THE_ARRANGEMENT()),
            "LATE_LINES" => Some(g_LATE_LINES()),
            "LEFT_OUT" => Some(g_LEFT_OUT()),
            "MESS" => Some(g_MESS()),
            "MESSY" => Some(g_MESSY()),
            "MY_POOLS" => Some(g_MY_POOLS()),
            "MY_THING" => Some(g_MY_THING()),
            "NOT_ASKED" => Some(g_NOT_ASKED()),
            "NOT_A_GRUMBLE" => Some(g_NOT_A_GRUMBLE()),
            "NOT_HELPED" => Some(g_NOT_HELPED()),
            "NOT_SINCERE" => Some(g_NOT_SINCERE()),
            "NOT_THIS" => Some(g_NOT_THIS()),
            "PEOPLE" => Some(g_PEOPLE()),
            "PEOPLE_SET" => Some(g_PEOPLE_SET()),
            "RAISE" => Some(g_RAISE()),
            "RECURS" => Some(g_RECURS()),
            "SAID_IT_POOL" => Some(g_SAID_IT_POOL()),
            "SAYS_ABOUT_ME" => Some(g_SAYS_ABOUT_ME()),
            "SELF_DOUBT" => Some(g_SELF_DOUBT()),
            "SOMEONE" => Some(g_SOMEONE()),
            "STRAIN" => Some(g_STRAIN()),
            "SUBJECT_AGAIN" => Some(g_SUBJECT_AGAIN()),
            "TAKEN" => Some(g_TAKEN()),
            "TAKEN_FOR_GRANTED" => Some(g_TAKEN_FOR_GRANTED()),
            "THANKED" => Some(g_THANKED()),
            "THE_SAYING" => Some(g_THE_SAYING()),
            "THREAD_TURNS" => Some(g_THREAD_TURNS()),
            "UNSEEN" => Some(g_UNSEEN()),
            "WORDS_NOT_MEANT" => Some(g_WORDS_NOT_MEANT()),
            "WORK_LINES" => Some(g_WORK_LINES()),
            "WORK_PEOPLE" => Some(g_WORK_PEOPLE()),
            "WORK_WORDS" => Some(g_WORK_WORDS()),
            "_follow_up" => Some(py_func("_follow_up", |args: &[Py]| _follow_up(args))),
            "_in_my_pools" => Some(py_func("_in_my_pools", |args: &[Py]| _in_my_pools(args))),
            "_person_named" => Some(py_func("_person_named", |args: &[Py]| _person_named(args))),
            "_said_at_work" => Some(py_func("_said_at_work", |args: &[Py]| _said_at_work(args))),
            "_shape" => Some(py_func("_shape", |args: &[Py]| _shape(args))),
            "_strained" => Some(py_func("_strained", |args: &[Py]| _strained(args))),
            "_the_one" => Some(py_func("_the_one", |args: &[Py]| _the_one(args))),
            "_thread_open" => Some(py_func("_thread_open", |args: &[Py]| _thread_open(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "holds_the_thread" => Some(py_func("holds_the_thread", |args: &[Py]| holds_the_thread(args))),
            "pick" => Some(g_pick()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_ME" => s_ABOUT_ME(v),
            "AFTER_A_SAYING" => s_AFTER_A_SAYING(v),
            "ALWAYS_COUNT" => s_ALWAYS_COUNT(v),
            "ALWAYS_DOES_IT" => s_ALWAYS_DOES_IT(v),
            "ARRIVES_LATE" => s_ARRIVES_LATE(v),
            "ASKED" => s_ASKED(v),
            "ASK_FOR" => s_ASK_FOR(v),
            "A_PERSON" => s_A_PERSON(v),
            "A_SHARED_OCCASION" => s_A_SHARED_OCCASION(v),
            "BAILING" => s_BAILING(v),
            "BAILS" => s_BAILS(v),
            "CHILD_PEOPLE" => s_CHILD_PEOPLE(v),
            "COURTESY" => s_COURTESY(v),
            "COURTESY_DENIED" => s_COURTESY_DENIED(v),
            "CREDIT" => s_CREDIT(v),
            "CREDIT_LINES" => s_CREDIT_LINES(v),
            "CRITICISED" => s_CRITICISED(v),
            "CRITICISED_LINES" => s_CRITICISED_LINES(v),
            "CRITICISED_MY" => s_CRITICISED_MY(v),
            "DONE_TO_ME" => s_DONE_TO_ME(v),
            "DOUBT_POOL" => s_DOUBT_POOL(v),
            "EMAIL_HABIT" => s_EMAIL_HABIT(v),
            "EVERY_TIME_POOL" => s_EVERY_TIME_POOL(v),
            "GOING_ON" => s_GOING_ON(v),
            "GOOD_HABIT" => s_GOOD_HABIT(v),
            "GOOD_HABIT_APPRECIATED" => s_GOOD_HABIT_APPRECIATED(v),
            "GOOD_HABIT_INCLUDED" => s_GOOD_HABIT_INCLUDED(v),
            "GOOD_WORDS" => s_GOOD_WORDS(v),
            "HOLD_THE_LINE" => s_HOLD_THE_LINE(v),
            "KEEPS_THE_ARRANGEMENT" => s_KEEPS_THE_ARRANGEMENT(v),
            "LATE_LINES" => s_LATE_LINES(v),
            "LEFT_OUT" => s_LEFT_OUT(v),
            "MESS" => s_MESS(v),
            "MESSY" => s_MESSY(v),
            "MY_POOLS" => s_MY_POOLS(v),
            "MY_THING" => s_MY_THING(v),
            "NOT_ASKED" => s_NOT_ASKED(v),
            "NOT_A_GRUMBLE" => s_NOT_A_GRUMBLE(v),
            "NOT_HELPED" => s_NOT_HELPED(v),
            "NOT_SINCERE" => s_NOT_SINCERE(v),
            "NOT_THIS" => s_NOT_THIS(v),
            "PEOPLE" => s_PEOPLE(v),
            "PEOPLE_SET" => s_PEOPLE_SET(v),
            "RAISE" => s_RAISE(v),
            "RECURS" => s_RECURS(v),
            "SAID_IT_POOL" => s_SAID_IT_POOL(v),
            "SAYS_ABOUT_ME" => s_SAYS_ABOUT_ME(v),
            "SELF_DOUBT" => s_SELF_DOUBT(v),
            "SOMEONE" => s_SOMEONE(v),
            "STRAIN" => s_STRAIN(v),
            "SUBJECT_AGAIN" => s_SUBJECT_AGAIN(v),
            "TAKEN" => s_TAKEN(v),
            "TAKEN_FOR_GRANTED" => s_TAKEN_FOR_GRANTED(v),
            "THANKED" => s_THANKED(v),
            "THE_SAYING" => s_THE_SAYING(v),
            "THREAD_TURNS" => s_THREAD_TURNS(v),
            "UNSEEN" => s_UNSEEN(v),
            "WORDS_NOT_MEANT" => s_WORDS_NOT_MEANT(v),
            "WORK_LINES" => s_WORK_LINES(v),
            "WORK_PEOPLE" => s_WORK_PEOPLE(v),
            "WORK_WORDS" => s_WORK_WORDS(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.habit", module_obj);
    pyrt::xrefs::reg_fn("compose.habit", "_in_my_pools", py_func("_in_my_pools", |args: &[Py]| _in_my_pools(args)));
    pyrt::xrefs::reg_fn("compose.habit", "_thread_open", py_func("_thread_open", |args: &[Py]| _thread_open(args)));
    pyrt::xrefs::reg_fn("compose.habit", "holds_the_thread", py_func("holds_the_thread", |args: &[Py]| holds_the_thread(args)));
    pyrt::xrefs::reg_fn("compose.habit", "_follow_up", py_func("_follow_up", |args: &[Py]| _follow_up(args)));
    pyrt::xrefs::reg_fn("compose.habit", "_the_one", py_func("_the_one", |args: &[Py]| _the_one(args)));
    pyrt::xrefs::reg_fn("compose.habit", "_person_named", py_func("_person_named", |args: &[Py]| _person_named(args)));
    pyrt::xrefs::reg_fn("compose.habit", "_shape", py_func("_shape", |args: &[Py]| _shape(args)));
    pyrt::xrefs::reg_fn("compose.habit", "_said_at_work", py_func("_said_at_work", |args: &[Py]| _said_at_work(args)));
    pyrt::xrefs::reg_fn("compose.habit", "_strained", py_func("_strained", |args: &[Py]| _strained(args)));
    pyrt::xrefs::reg_fn("compose.habit", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.habit", "ABOUT_ME", g_ABOUT_ME, s_ABOUT_ME);
    pyrt::xrefs::reg_access("compose.habit", "AFTER_A_SAYING", g_AFTER_A_SAYING, s_AFTER_A_SAYING);
    pyrt::xrefs::reg_access("compose.habit", "ALWAYS_COUNT", g_ALWAYS_COUNT, s_ALWAYS_COUNT);
    pyrt::xrefs::reg_access("compose.habit", "ALWAYS_DOES_IT", g_ALWAYS_DOES_IT, s_ALWAYS_DOES_IT);
    pyrt::xrefs::reg_access("compose.habit", "ARRIVES_LATE", g_ARRIVES_LATE, s_ARRIVES_LATE);
    pyrt::xrefs::reg_access("compose.habit", "ASKED", g_ASKED, s_ASKED);
    pyrt::xrefs::reg_access("compose.habit", "ASK_FOR", g_ASK_FOR, s_ASK_FOR);
    pyrt::xrefs::reg_access("compose.habit", "A_PERSON", g_A_PERSON, s_A_PERSON);
    pyrt::xrefs::reg_access("compose.habit", "A_SHARED_OCCASION", g_A_SHARED_OCCASION, s_A_SHARED_OCCASION);
    pyrt::xrefs::reg_access("compose.habit", "BAILING", g_BAILING, s_BAILING);
    pyrt::xrefs::reg_access("compose.habit", "BAILS", g_BAILS, s_BAILS);
    pyrt::xrefs::reg_access("compose.habit", "CHILD_PEOPLE", g_CHILD_PEOPLE, s_CHILD_PEOPLE);
    pyrt::xrefs::reg_access("compose.habit", "COURTESY", g_COURTESY, s_COURTESY);
    pyrt::xrefs::reg_access("compose.habit", "COURTESY_DENIED", g_COURTESY_DENIED, s_COURTESY_DENIED);
    pyrt::xrefs::reg_access("compose.habit", "CREDIT", g_CREDIT, s_CREDIT);
    pyrt::xrefs::reg_access("compose.habit", "CREDIT_LINES", g_CREDIT_LINES, s_CREDIT_LINES);
    pyrt::xrefs::reg_access("compose.habit", "CRITICISED", g_CRITICISED, s_CRITICISED);
    pyrt::xrefs::reg_access("compose.habit", "CRITICISED_LINES", g_CRITICISED_LINES, s_CRITICISED_LINES);
    pyrt::xrefs::reg_access("compose.habit", "CRITICISED_MY", g_CRITICISED_MY, s_CRITICISED_MY);
    pyrt::xrefs::reg_access("compose.habit", "DONE_TO_ME", g_DONE_TO_ME, s_DONE_TO_ME);
    pyrt::xrefs::reg_access("compose.habit", "DOUBT_POOL", g_DOUBT_POOL, s_DOUBT_POOL);
    pyrt::xrefs::reg_access("compose.habit", "EMAIL_HABIT", g_EMAIL_HABIT, s_EMAIL_HABIT);
    pyrt::xrefs::reg_access("compose.habit", "EVERY_TIME_POOL", g_EVERY_TIME_POOL, s_EVERY_TIME_POOL);
    pyrt::xrefs::reg_access("compose.habit", "GOING_ON", g_GOING_ON, s_GOING_ON);
    pyrt::xrefs::reg_access("compose.habit", "GOOD_HABIT", g_GOOD_HABIT, s_GOOD_HABIT);
    pyrt::xrefs::reg_access("compose.habit", "GOOD_HABIT_APPRECIATED", g_GOOD_HABIT_APPRECIATED, s_GOOD_HABIT_APPRECIATED);
    pyrt::xrefs::reg_access("compose.habit", "GOOD_HABIT_INCLUDED", g_GOOD_HABIT_INCLUDED, s_GOOD_HABIT_INCLUDED);
    pyrt::xrefs::reg_access("compose.habit", "GOOD_WORDS", g_GOOD_WORDS, s_GOOD_WORDS);
    pyrt::xrefs::reg_access("compose.habit", "HOLD_THE_LINE", g_HOLD_THE_LINE, s_HOLD_THE_LINE);
    pyrt::xrefs::reg_access("compose.habit", "KEEPS_THE_ARRANGEMENT", g_KEEPS_THE_ARRANGEMENT, s_KEEPS_THE_ARRANGEMENT);
    pyrt::xrefs::reg_access("compose.habit", "LATE_LINES", g_LATE_LINES, s_LATE_LINES);
    pyrt::xrefs::reg_access("compose.habit", "LEFT_OUT", g_LEFT_OUT, s_LEFT_OUT);
    pyrt::xrefs::reg_access("compose.habit", "MESS", g_MESS, s_MESS);
    pyrt::xrefs::reg_access("compose.habit", "MESSY", g_MESSY, s_MESSY);
    pyrt::xrefs::reg_access("compose.habit", "MY_POOLS", g_MY_POOLS, s_MY_POOLS);
    pyrt::xrefs::reg_access("compose.habit", "MY_THING", g_MY_THING, s_MY_THING);
    pyrt::xrefs::reg_access("compose.habit", "NOT_ASKED", g_NOT_ASKED, s_NOT_ASKED);
    pyrt::xrefs::reg_access("compose.habit", "NOT_A_GRUMBLE", g_NOT_A_GRUMBLE, s_NOT_A_GRUMBLE);
    pyrt::xrefs::reg_access("compose.habit", "NOT_HELPED", g_NOT_HELPED, s_NOT_HELPED);
    pyrt::xrefs::reg_access("compose.habit", "NOT_SINCERE", g_NOT_SINCERE, s_NOT_SINCERE);
    pyrt::xrefs::reg_access("compose.habit", "NOT_THIS", g_NOT_THIS, s_NOT_THIS);
    pyrt::xrefs::reg_access("compose.habit", "PEOPLE", g_PEOPLE, s_PEOPLE);
    pyrt::xrefs::reg_access("compose.habit", "PEOPLE_SET", g_PEOPLE_SET, s_PEOPLE_SET);
    pyrt::xrefs::reg_access("compose.habit", "RAISE", g_RAISE, s_RAISE);
    pyrt::xrefs::reg_access("compose.habit", "RECURS", g_RECURS, s_RECURS);
    pyrt::xrefs::reg_access("compose.habit", "SAID_IT_POOL", g_SAID_IT_POOL, s_SAID_IT_POOL);
    pyrt::xrefs::reg_access("compose.habit", "SAYS_ABOUT_ME", g_SAYS_ABOUT_ME, s_SAYS_ABOUT_ME);
    pyrt::xrefs::reg_access("compose.habit", "SELF_DOUBT", g_SELF_DOUBT, s_SELF_DOUBT);
    pyrt::xrefs::reg_access("compose.habit", "SOMEONE", g_SOMEONE, s_SOMEONE);
    pyrt::xrefs::reg_access("compose.habit", "STRAIN", g_STRAIN, s_STRAIN);
    pyrt::xrefs::reg_access("compose.habit", "SUBJECT_AGAIN", g_SUBJECT_AGAIN, s_SUBJECT_AGAIN);
    pyrt::xrefs::reg_access("compose.habit", "TAKEN", g_TAKEN, s_TAKEN);
    pyrt::xrefs::reg_access("compose.habit", "TAKEN_FOR_GRANTED", g_TAKEN_FOR_GRANTED, s_TAKEN_FOR_GRANTED);
    pyrt::xrefs::reg_access("compose.habit", "THANKED", g_THANKED, s_THANKED);
    pyrt::xrefs::reg_access("compose.habit", "THE_SAYING", g_THE_SAYING, s_THE_SAYING);
    pyrt::xrefs::reg_access("compose.habit", "THREAD_TURNS", g_THREAD_TURNS, s_THREAD_TURNS);
    pyrt::xrefs::reg_access("compose.habit", "UNSEEN", g_UNSEEN, s_UNSEEN);
    pyrt::xrefs::reg_access("compose.habit", "WORDS_NOT_MEANT", g_WORDS_NOT_MEANT, s_WORDS_NOT_MEANT);
    pyrt::xrefs::reg_access("compose.habit", "WORK_LINES", g_WORK_LINES, s_WORK_LINES);
    pyrt::xrefs::reg_access("compose.habit", "WORK_PEOPLE", g_WORK_PEOPLE, s_WORK_PEOPLE);
    pyrt::xrefs::reg_access("compose.habit", "WORK_WORDS", g_WORK_WORDS, s_WORK_WORDS);
    pyrt::xrefs::reg_access("compose.habit", "pick", g_pick, s_pick);
}
