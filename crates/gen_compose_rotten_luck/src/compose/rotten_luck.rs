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


thread_local! { static G_A_BILL_THAT_SAYS_OTHERWISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BLOW_FROM_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHORE_PILE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHORE_WAITING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FALL_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FRIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FRIGHT_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB_SOMEBODY_ELSE_DOES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LACK_THAT_IS_STILL_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEAL_THEY_WENT_WITHOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEAR_MISS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEAR_MISS_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NO_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NO_FROM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REMARK_THAT_STUNG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_THAT_DID_NOT_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAIT_THAT_WENT_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLANK_OR_THIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAMAGED_OR_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLAD_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLAD_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISHAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEAR_MISS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FRIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_GOOD_DESPITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESOLVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ASK_OPENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BARE_NEAR_MISS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BODY_CAN_BE_HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DISPUTED_CHARGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_JOB_NEEDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_JOB_THAT_IS_SORTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LONG_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEAL_MISSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEAL_ON_PURPOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NO_STILL_BEING_WEIGHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON_NEEDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PLAN_THAT_FELL_THROUGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_REFUSAL_IN_THE_PASSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_REFUSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_REMARK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SAID_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THING_WITH_NO_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHORE_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FROM_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MINE_OR_OURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PILE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHAKE_IS_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEBODY_ELSES_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STILL_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRIED_AND_IT_DID_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.rotten_luck: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_BILL_THAT_SAYS_OTHERWISE() -> Py { ensure_init(); G_A_BILL_THAT_SAYS_OTHERWISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BILL_THAT_SAYS_OTHERWISE(v: Py) { ensure_init(); G_A_BILL_THAT_SAYS_OTHERWISE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BLOW_FROM_SOMEONE() -> Py { ensure_init(); G_A_BLOW_FROM_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BLOW_FROM_SOMEONE(v: Py) { ensure_init(); G_A_BLOW_FROM_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHORE_PILE() -> Py { ensure_init(); G_A_CHORE_PILE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHORE_PILE(v: Py) { ensure_init(); G_A_CHORE_PILE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHORE_WAITING() -> Py { ensure_init(); G_A_CHORE_WAITING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHORE_WAITING(v: Py) { ensure_init(); G_A_CHORE_WAITING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FALL() -> Py { ensure_init(); G_A_FALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FALL(v: Py) { ensure_init(); G_A_FALL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FALL_ELSE() -> Py { ensure_init(); G_A_FALL_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FALL_ELSE(v: Py) { ensure_init(); G_A_FALL_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FRIGHT() -> Py { ensure_init(); G_A_FRIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FRIGHT(v: Py) { ensure_init(); G_A_FRIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FRIGHT_ELSE() -> Py { ensure_init(); G_A_FRIGHT_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FRIGHT_ELSE(v: Py) { ensure_init(); G_A_FRIGHT_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB_SOMEBODY_ELSE_DOES() -> Py { ensure_init(); G_A_JOB_SOMEBODY_ELSE_DOES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB_SOMEBODY_ELSE_DOES(v: Py) { ensure_init(); G_A_JOB_SOMEBODY_ELSE_DOES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LACK_THAT_IS_STILL_GOING() -> Py { ensure_init(); G_A_LACK_THAT_IS_STILL_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LACK_THAT_IS_STILL_GOING(v: Py) { ensure_init(); G_A_LACK_THAT_IS_STILL_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEAL_THEY_WENT_WITHOUT() -> Py { ensure_init(); G_A_MEAL_THEY_WENT_WITHOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEAL_THEY_WENT_WITHOUT(v: Py) { ensure_init(); G_A_MEAL_THEY_WENT_WITHOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEAR_MISS() -> Py { ensure_init(); G_A_NEAR_MISS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEAR_MISS(v: Py) { ensure_init(); G_A_NEAR_MISS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEAR_MISS_ELSE() -> Py { ensure_init(); G_A_NEAR_MISS_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEAR_MISS_ELSE(v: Py) { ensure_init(); G_A_NEAR_MISS_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NO_ALREADY() -> Py { ensure_init(); G_A_NO_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NO_ALREADY(v: Py) { ensure_init(); G_A_NO_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NO_FROM() -> Py { ensure_init(); G_A_NO_FROM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NO_FROM(v: Py) { ensure_init(); G_A_NO_FROM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REMARK_THAT_STUNG() -> Py { ensure_init(); G_A_REMARK_THAT_STUNG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REMARK_THAT_STUNG(v: Py) { ensure_init(); G_A_REMARK_THAT_STUNG.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_THAT_DID_NOT_WORK() -> Py { ensure_init(); G_A_THING_THAT_DID_NOT_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_THAT_DID_NOT_WORK(v: Py) { ensure_init(); G_A_THING_THAT_DID_NOT_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAIT_THAT_WENT_ON() -> Py { ensure_init(); G_A_WAIT_THAT_WENT_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAIT_THAT_WENT_ON(v: Py) { ensure_init(); G_A_WAIT_THAT_WENT_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_BLANK_OR_THIN() -> Py { ensure_init(); G_BLANK_OR_THIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLANK_OR_THIN(v: Py) { ensure_init(); G_BLANK_OR_THIN.with(|c| *c.borrow_mut() = v); }
pub fn g_DAMAGED_OR_TAKEN() -> Py { ensure_init(); G_DAMAGED_OR_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAMAGED_OR_TAKEN(v: Py) { ensure_init(); G_DAMAGED_OR_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_FRIGHT() -> Py { ensure_init(); G_FRIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRIGHT(v: Py) { ensure_init(); G_FRIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_GLAD_ANYWHERE() -> Py { ensure_init(); G_GLAD_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLAD_ANYWHERE(v: Py) { ensure_init(); G_GLAD_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_GLAD_OPENER() -> Py { ensure_init(); G_GLAD_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLAD_OPENER(v: Py) { ensure_init(); G_GLAD_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_MISHAP() -> Py { ensure_init(); G_MISHAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISHAP(v: Py) { ensure_init(); G_MISHAP.with(|c| *c.borrow_mut() = v); }
pub fn g_NEAR_MISS() -> Py { ensure_init(); G_NEAR_MISS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEAR_MISS(v: Py) { ensure_init(); G_NEAR_MISS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FRIGHT() -> Py { ensure_init(); G_NOT_A_FRIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FRIGHT(v: Py) { ensure_init(); G_NOT_A_FRIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_GOOD() -> Py { ensure_init(); G_NOT_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_GOOD(v: Py) { ensure_init(); G_NOT_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_GOOD_DESPITE() -> Py { ensure_init(); G_NOT_GOOD_DESPITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_GOOD_DESPITE(v: Py) { ensure_init(); G_NOT_GOOD_DESPITE.with(|c| *c.borrow_mut() = v); }
pub fn g_RESOLVED() -> Py { ensure_init(); G_RESOLVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESOLVED(v: Py) { ensure_init(); G_RESOLVED.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_PLACE() -> Py { ensure_init(); G_THEIR_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_PLACE(v: Py) { ensure_init(); G_THEIR_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_THING() -> Py { ensure_init(); G_THEIR_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_THING(v: Py) { ensure_init(); G_THEIR_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ASK_OPENING() -> Py { ensure_init(); G__AN_ASK_OPENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ASK_OPENING(v: Py) { ensure_init(); G__AN_ASK_OPENING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BARE_NEAR_MISS() -> Py { ensure_init(); G__A_BARE_NEAR_MISS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BARE_NEAR_MISS(v: Py) { ensure_init(); G__A_BARE_NEAR_MISS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BODY_CAN_BE_HURT() -> Py { ensure_init(); G__A_BODY_CAN_BE_HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BODY_CAN_BE_HURT(v: Py) { ensure_init(); G__A_BODY_CAN_BE_HURT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DISPUTED_CHARGE() -> Py { ensure_init(); G__A_DISPUTED_CHARGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DISPUTED_CHARGE(v: Py) { ensure_init(); G__A_DISPUTED_CHARGE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_JOB_NEEDED() -> Py { ensure_init(); G__A_JOB_NEEDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_JOB_NEEDED(v: Py) { ensure_init(); G__A_JOB_NEEDED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_JOB_THAT_IS_SORTED() -> Py { ensure_init(); G__A_JOB_THAT_IS_SORTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_JOB_THAT_IS_SORTED(v: Py) { ensure_init(); G__A_JOB_THAT_IS_SORTED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LONG_WAIT() -> Py { ensure_init(); G__A_LONG_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LONG_WAIT(v: Py) { ensure_init(); G__A_LONG_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEAL_MISSED() -> Py { ensure_init(); G__A_MEAL_MISSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEAL_MISSED(v: Py) { ensure_init(); G__A_MEAL_MISSED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEAL_ON_PURPOSE() -> Py { ensure_init(); G__A_MEAL_ON_PURPOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEAL_ON_PURPOSE(v: Py) { ensure_init(); G__A_MEAL_ON_PURPOSE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NO_STILL_BEING_WEIGHED() -> Py { ensure_init(); G__A_NO_STILL_BEING_WEIGHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NO_STILL_BEING_WEIGHED(v: Py) { ensure_init(); G__A_NO_STILL_BEING_WEIGHED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON_NEEDING() -> Py { ensure_init(); G__A_PERSON_NEEDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON_NEEDING(v: Py) { ensure_init(); G__A_PERSON_NEEDING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PLAN_THAT_FELL_THROUGH() -> Py { ensure_init(); G__A_PLAN_THAT_FELL_THROUGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PLAN_THAT_FELL_THROUGH(v: Py) { ensure_init(); G__A_PLAN_THAT_FELL_THROUGH.with(|c| *c.borrow_mut() = v); }
pub fn g__A_REFUSAL_IN_THE_PASSIVE() -> Py { ensure_init(); G__A_REFUSAL_IN_THE_PASSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_REFUSAL_IN_THE_PASSIVE(v: Py) { ensure_init(); G__A_REFUSAL_IN_THE_PASSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_REFUSED() -> Py { ensure_init(); G__A_REFUSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_REFUSED(v: Py) { ensure_init(); G__A_REFUSED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_REMARK() -> Py { ensure_init(); G__A_REMARK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_REMARK(v: Py) { ensure_init(); G__A_REMARK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SAID_NO() -> Py { ensure_init(); G__A_SAID_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SAID_NO(v: Py) { ensure_init(); G__A_SAID_NO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THING_WITH_NO_BODY() -> Py { ensure_init(); G__A_THING_WITH_NO_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THING_WITH_NO_BODY(v: Py) { ensure_init(); G__A_THING_WITH_NO_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WAIT() -> Py { ensure_init(); G__A_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WAIT(v: Py) { ensure_init(); G__A_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g__CHORE_THINGS() -> Py { ensure_init(); G__CHORE_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHORE_THINGS(v: Py) { ensure_init(); G__CHORE_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__FALL() -> Py { ensure_init(); G__FALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FALL(v: Py) { ensure_init(); G__FALL.with(|c| *c.borrow_mut() = v); }
pub fn g__FROM_SOMEONE() -> Py { ensure_init(); G__FROM_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FROM_SOMEONE(v: Py) { ensure_init(); G__FROM_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g__MINE() -> Py { ensure_init(); G__MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MINE(v: Py) { ensure_init(); G__MINE.with(|c| *c.borrow_mut() = v); }
pub fn g__MINE_OR_OURS() -> Py { ensure_init(); G__MINE_OR_OURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MINE_OR_OURS(v: Py) { ensure_init(); G__MINE_OR_OURS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TRADE() -> Py { ensure_init(); G__NOT_A_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TRADE(v: Py) { ensure_init(); G__NOT_A_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g__PILE_WORDS() -> Py { ensure_init(); G__PILE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PILE_WORDS(v: Py) { ensure_init(); G__PILE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SHAKE_IS_THEIRS() -> Py { ensure_init(); G__SHAKE_IS_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHAKE_IS_THEIRS(v: Py) { ensure_init(); G__SHAKE_IS_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEBODY_ELSES_MEAL() -> Py { ensure_init(); G__SOMEBODY_ELSES_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEBODY_ELSES_MEAL(v: Py) { ensure_init(); G__SOMEBODY_ELSES_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_ELSE() -> Py { ensure_init(); G__SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE(v: Py) { ensure_init(); G__SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__STILL_NO() -> Py { ensure_init(); G__STILL_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STILL_NO(v: Py) { ensure_init(); G__STILL_NO.with(|c| *c.borrow_mut() = v); }
pub fn g__TRIED_AND_IT_DID_NOT() -> Py { ensure_init(); G__TRIED_AND_IT_DID_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRIED_AND_IT_DID_NOT(v: Py) { ensure_init(); G__TRIED_AND_IT_DID_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing of theirs that has been damaged, taken or refused is not a day that went well.\n\n\"We had a flood\", \"someone keyed my car\", \"my son got suspended\", \"i came off my bike\" and \"my\nvisa was refused\" all reach the composer's small-day pool on main, which is written for an\nordinary account of an ordinary afternoon (\"a takeaway, a haircut, work being alright\"):\n\n    we had a flood                -> \"Nice one. How did the rest of it go?\"\n    someone keyed my car          -> \"Good to hear. What was the best bit?\"\n    my son got suspended          -> \"Nice one. How did the rest of it go?\"\n    i came off my bike            -> \"Nice one. How did the rest of it go?\"\n\nThat is the worst thing the bot can say to somebody, and it is not a knowledge gap: nothing needs\nlooking up to answer it. `bot/compose/badnews.py` reads the hard news - a diagnosis, a death, the\nmoney running out - but a flooded kitchen, a keyed car and a child sent home are a smaller kind\nof bad day, and no reader has them, so they fall through to the pleasantry.\n\nWhat a friend does with one of these is say it is rotten and ask one thing about it, which is\nwhat this file does. Three pools, because the three shapes want different questions: a thing of\ntheirs that was damaged or taken wants \"what needs sorting\", a refusal or a punishment that\nlanded on them wants \"what did they say\", and coming off a bike wants \"are you alright\".\n\nThis is a last resort and it is written to be one. `bot/modules/greeting.py` asks it at the very\nend, and it only takes the turn when the reply the composer chose is one of its glad lines - so\nan answer of any kind, from any module, keeps the turn it has. Nothing here invents a fact: the\nreply says the day is bad and asks about it.\n\nThe words that are ordinary in another line are left out: \"a flood of orders\", \"flooded with\napplications\", a \"fire drill\", a code \"keyed in\", the show somebody \"stole\". A line that says\none of those is not this.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_THEIR_PLACE(py_str("(?:my|our|the)\\s+(?:[a-z']+\\s+)?(?:house|flat|place|home|kitchen|bathroom|cellar|basement|loft|garage|shed|garden|hall|hallway|landing|roof|bedroom|attic|utility|conservatory)"));
    s_THEIR_THING(py_str("(?:my|our)\\s+(?:[a-z']+\\s+){0,2}"));
    s_NOT_GOOD(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let mut __s = String::new(); __s.push_str("\\b(?:we|i|they)\\s+(?:just\\s+)?had\\s+(?:a|an)\\s+(?:flood|flooding|leak|burst\\s+pipe|burst\\s+radiator|break[- ]?in|burglary|fire)\\b"); py_string(__s) }, { let mut __s = String::new(); __s.push_str(&fstr(&g_THEIR_PLACE(), -1, "")); __s.push_str("[^.!?,]{0,24}\\b(?:flooded|flooding|is flooding|has flooded|caught fire|went on fire|is leaking|was leaking|has water coming in|came through the ceiling)\\b"); py_string(__s) }, py_str("\\b(?:got|been|were|was)\\s+(?:burgled|broken into|robbed|vandalised|vandalized|smashed up|turned over)\\b"), { let mut __s = String::new(); __s.push_str(&fstr(&g_THEIR_THING(), -1, "")); __s.push_str("\\b(?:got|was|were|has been|had been|is|are|got\\s+)\\s*(?:keyed|scratched|dented|clamped|towed(?:\\s+away)?|smashed|nicked|stolen|vandalised|vandalized|broken into|pinched|slashed|punctured|kicked in|wrenched off|snapped off|ripped|torn|torched|set (?:on )?fire to)\\b"); py_string(__s) }, py_str("\\b(?:someone|somebody|a kid|the kids|next door|a neighbour|a neighbor|my neighbours?|our neighbours?|the neighbours?)\\s+(?:keyed|scratched|dented|smashed|nicked|pinched|stole|took|vandalised|vandalized)\\s+(?:my|our)\\b"), { let mut __s = String::new(); __s.push_str(&fstr(&g_THEIR_THING(), -1, "")); __s.push_str("\\b(?:son|daughter|kid|child|boy|girl|lad|lass|teenager|eldest|youngest)\\b[^.!?,]{0,30}\\b(?:suspended|excluded|expelled)\\b"); py_string(__s) }, { let mut __s = String::new(); __s.push_str(&fstr(&g_THEIR_THING(), -1, "")); __s.push_str("\\b(?:visa|application|claim|mortgage|loan|benefits?|pip|dla|passport|offer|request)\\b[^.!?,]{0,24}\\b(?:refused|rejected|turned down|declined|denied|turned me down|turned them down)\\b"); py_string(__s) }, py_str("\\bcame off\\s+(?:my|his|her|the|a|their)\\s+(?:bike|motorbike|moped|scooter|e-scooter|cycle|horse|ladder|roof|skateboard|board)\\b"), { let mut __s = String::new(); __s.push_str(&fstr(&g_THEIR_THING(), -1, "")); __s.push_str("\\b(?:car|van|bike|motorbike)\\b[^.!?,]{0,20}\\b(?:got|was|were)\\s+(?:clamped|towed(?:\\s+away)?|keyed|scratched|dented|scraped)\\b"); py_string(__s) }, py_str("\\bpranged\\b|\\bscraped the (?:car|van|wing mirror)\\b"), { let mut __s = String::new(); __s.push_str("(?:"); __s.push_str(&fstr(&g_THEIR_PLACE(), -1, "")); __s.push_str("|\\b(?:ceiling|roof|fence|gate|wall|greenhouse|tree|branch|tiles?|guttering|gutter|chimney pot|carport|awning))\\b[^.!?,]{0,24}\\b(?:came down|collapsed|caved in|blew (?:away|down|off|over)|got blown (?:away|down|off))\\b"); py_string(__s) }])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_GOOD_DESPITE(stdlib_call("re", "compile", &[py_str("\\bflood(?:ed)?\\s+(?:of|with)\\b|\\bfire\\s+(?:drill|alarm|test|work|escape|station|place|pits?)\\b|\\bkeyed\\s+in\\b|\\bstole\\s+the\\s+(?:show|limelight|scene)\\b|\\bnicked\\s+(?:off|out)\\b|\\brobbed\\s+(?:blind|us of)\\b|\\be?xcluded\\s+from\\s+the\\s+pub\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GLAD_OPENER(stdlib_call("re", "compile", &[py_str("^(?:nice one|good to hear|sounds alright|well done|that'?s good|lovely|glad to hear|how was it, overall)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BLANK_OR_THIN(stdlib_call("re", "compile", &[py_str("^(?:got it\\b|thanks for telling me|i'?m listening|noted:|right\\.|okay\\.|ok\\.|rotten luck\\.?$|that'?s a pain\\.?$|that'?s rough\\.?$|fair enough\\b|ugh, that'?s annoying\\.?$|that'?s annoying\\.?$)")]).unwrap_or_else(|e| exc::raise(e)));
    s_GLAD_ANYWHERE(stdlib_call("re", "compile", &[py_str("\\b(?:how did the rest of it go|what was the best bit|anything good in it|are you pleased with it|is it what you were after|how did that feel)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SAID_NO(stdlib_call("re", "compile", &[py_str("\\b(?:she|he|they|my\\s+[a-z]+|our\\s+[a-z]+|the\\s+[a-z]+|his\\s+[a-z]+|her\\s+[a-z]+|their\\s+[a-z]+|mum|dad|boss|landlord|manager|school|council|bank|surgery|gp|doctor|nurse|hr)\\b(?:\\s+[a-z']+){0,4}?\\s(?:said|says|saying|told\\s+me|telling\\s+me)\\s+(?:me\\s+)?no\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_REFUSED(stdlib_call("re", "compile", &[py_str("\\b(?:she|he|they|my\\s+[a-z]+|our\\s+[a-z]+|the\\s+[a-z]+|his\\s+[a-z]+|her\\s+[a-z]+|their\\s+[a-z]+|mum|dad|boss|landlord|manager|school|council|bank|surgery|gp|doctor|hr)\\b(?:\\s+[a-z']+){0,3}?\\s(?:refused|has\\s+refused|had\\s+refused|turned\\s+(?:it|me)\\s+down|won'?t\\s+(?:budge|have\\s+it|agree|allow|consider|do\\s+it|move|shift|bend)|isn'?t\\s+keen|wasn'?t\\s+keen|is\\s+not\\s+keen|said\\s+(?:it|that)\\s+(?:isn'?t|wasn'?t|is\\s+not|was\\s+not)\\s+(?:possible|allowed|an\\s+option|on))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NO_ALREADY(py_tuple(vec![py_str("Then you know where you stand, and that is worth something. Was the no to that exact ask, or to the idea of it?"), py_str("Then the answer is no for now. What would have to change for it to be a yes?"), py_str("Ask once more with your reason in the same sentence, and take the second no as the answer."), py_str("One no is worth hearing again only if something has changed since. Say what has, in a line, and go back with it."), py_str("That is a no to the ask as you put it, which is not a no to you. Change one thing about it and try again."), py_str("Go for the smallest version of it that would still help, and build from that if it comes.")]));
    s_A_NO_FROM(py_tuple(vec![py_str("Then {who} has given you an answer. Was it a no to that ask, or to the whole thing?"), py_str("{who} has said no once, so ask for the smallest version of it that would still help."), py_str("A no from {who} is worth one more ask, with what you want and why in a single line - then leave it with them."), py_str("Go back to {who} with what has changed since and nothing else. That is what moves a no."), py_str("What would {who} have said yes to? People who refuse usually know.")]));
    s__MINE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NO_STILL_BEING_WEIGHED(stdlib_call("re", "compile", &[py_str("\\b(?:what\\s+if|if\\s+(?:she|he|they|my|the|it)|suppose|supposing|in\\s+case|unless|should\\s+i|shall\\s+i|do\\s+you\\s+think|would\\s+they|will\\s+they)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_REFUSAL_IN_THE_PASSIVE(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|got|been|is|are|being)\\s+(?:refused|turned\\s+down|rejected|turned\\s+away|declined|knocked\\s+back)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RESOLVED(stdlib_call("re", "compile", &[py_str("\\b(?:but|and|although|though)\\s+(?:\\w+\\s+){0,3}?(?:sorted|sorted out|fixed|resolved|dried out|paid out|refunded|replaced|recovered|got it back|got them back|got them|back at school|back to normal|back home|all fine|all ok|ok now|okay now|fine now|im fine|i'?m fine|was fine|no harm|nobody hurt|no one hurt|nothing was taken|only a scratch|turned out (?:fine|ok)|got it on appeal|won the appeal|appeal worked|only took (?:a|the|my|our|one)\\b)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAMAGED_OR_TAKEN(py_tuple(vec![py_str("Rotten luck. What needs sorting first?"), py_str("That's a rotten one. How bad is it?"), py_str("Ugh, that's the last thing you needed. Is it sorted, or still going on?"), py_str("That's a mess to land on you. What's the damage?")]));
    s_A_BLOW_FROM_SOMEONE(py_tuple(vec![py_str("That's a blow. Sorry to hear it."), py_str("Ugh, that's harsh. That's a rotten thing to have land on you."), py_str("That's a rotten thing to land on you.")]));
    s_A_FALL(py_tuple(vec![py_str("That's a shock. Are you alright?"), py_str("Ouch. Are you hurt?"), py_str("That would rattle anyone. Are you alright?")]));
    s_A_FALL_ELSE(py_tuple(vec![py_str("That's a shock. Are they alright?"), py_str("That's a bad one. How are they doing?"), py_str("That's a shock to hear. Is everyone alright?")]));
    s__SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their)\\s+(?:brother|sister|son|daughter|mum|mom|dad|husband|wife|partner|mate|friend|neighbour|neighbor|nephew|niece|grandad|granddad|grandma|gran|nan|colleague|boss|cousin|little (?:boy|girl|lad|one))\\b|\\b(?:he|she|they)\\s+(?:came off|fell)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__FALL(stdlib_call("re", "compile", &[py_str("\\bcame off|\\bpranged\\b|\\bscraped the (?:car|van|wing mirror)\\b|\\bclamped\\b|\\btowed\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MISHAP(py_str("(?:fell|fall|fallen|slipped|slip|tripped|trip|came\\s+off|come\\s+off|knocked\\s+off|knocked\\s+(?:me|us)\\s+off|hit\\s+(?:me|us|by)|ran\\s+(?:me|us)\\s+over|run\\s+over|drove\\s+(?:into|at)\\s+(?:me|us)|clipped\\s+(?:me|us)|collided|crashed|crash|went\\s+over|tumbled|skidded|smashed\\s+into)\\b"));
    s_NEAR_MISS(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:nearly|almost|close\\s+to|just\\s+missed|just\\s+about)\\b[^.!?,]{0,26}\\b"), &g_MISHAP()), &py_str("|\\b(?:near\\s+miss|close\\s+call|narrow\\s+escape|just\\s+missed\\s+(?:me|us))\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:shaken\\s+up|shaken|shook\\s+up|shook\\s+(?:me|us)|shakes\\s+(?:me|us)\\s+up|shaking\\s+(?:me|us)\\b|shaking\\s+(?:me|us)\\s+up|still\\s+shaking|shaking\\s+a\\s+bit|(?:got|had|gave\\s+me)\\s+a\\s+fright|a\\s+fright|bit\\s+of\\s+a\\s+scare|scared\\s+me|frightened\\s+me|spooked\\s+me|in\\s+shock)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FRIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:shaking\\s+with\\s+laughter|shaken\\s+not\\s+stirred|shaking\\s+like\\s+a\\s+leaf\\s+of\\s+lettuce|(?:scared|afraid|frightened|terrified)\\s+(?:of|to|about|that|it\\s+would)\\b|scared\\s+(?:of|to|about)\\b|frightened\\s+(?:of|to)\\b|in\\s+shock\\s+that\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PLAN_THAT_FELL_THROUGH(stdlib_call("re", "compile", &[py_str("\\bf(?:ell|all|allen|alling)\\s+through\\b(?!\\s+(?:the\\s+)?(?:ice|floor|roof|glass|window|boards?|ceiling|decking|net|crust|hole)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BARE_NEAR_MISS(stdlib_call("re", "compile", &[py_str("\\b(?:near\\s+miss|close\\s+call|narrow\\s+escape)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BODY_CAN_BE_HURT(stdlib_call("re", "compile", &[py_str("\\b(?:bike|cycle|bicycle|car|van|lorry|truck|bus|coach|train|tram|taxi|cab|scooter|motorbike|road|motorway|roundabout|kerb|junction|crossing|pavement|street|stairs|staircase|steps|ladder|scaffolding|roof|balcony|wall|window|glass|ice|snow|kitchen|bath|shower|pool|sea|cliff|horse|path|track|ferry|boat)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_THING_WITH_NO_BODY(stdlib_call("re", "compile", &[py_str("\\b(?:deadline|deal|sale|plan|plans|trip|booking|offer|order|mortgage|application|interview|contract|flight|agreement|purchase|quote|bid|tender|funding|visa|exam|essay|project|launch)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FRIGHT(py_tuple(vec![py_str("Glad you're okay. That stays with you for a while."), py_str("That gets the heart going. Sit down for a minute if you can."), py_str("A fright like that sits in the body for a while. Are you okay now?"), py_str("God, that shakes you up. Do you want to sit down for a minute?")]));
    s_A_NEAR_MISS(py_tuple(vec![py_str("Close one. Are you hurt at all?"), py_str("God, a near miss like that shakes you up. Are you alright?"), py_str("That gets the heart going. Sit down for a minute if you can."), py_str("Glad you're in one piece. That sort of thing stays with you.")]));
    s_A_FRIGHT_ELSE(py_tuple(vec![py_str("That shakes anyone up. Are they alright?"), py_str("A fright like that stays with you. How are they now?"), py_str("That gets the heart going. Is everyone alright?")]));
    s_A_NEAR_MISS_ELSE(py_tuple(vec![py_str("Close call. Is everyone alright?"), py_str("Glad they're in one piece. That sort of thing shakes you up."), py_str("That would rattle anyone. Are they hurt at all?")]));
    s__FROM_SOMEONE(stdlib_call("re", "compile", &[py_str("\\b(?:suspended|excluded|expelled|refused|rejected|turned down|declined|denied)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_THAT_DID_NOT_WORK(py_tuple(vec![py_str("That's annoying. What does it do when you try it?"), py_str("Ugh, one of those. Where are you up to with it now?"), py_str("That's a pain. What have you got in front of you at the moment?"), py_str("Rotten. Does it do the same thing every time?")]));
    s_A_WAIT_THAT_WENT_ON(py_tuple(vec![py_str("That's a long time to sit there. Did you get seen in the end?"), py_str("Ugh, two hours of that. Was it worth staying for?"), py_str("That's a lot of waiting. Did anyone tell you what was going on?")]));
    s_A_REMARK_THAT_STUNG(py_tuple(vec![py_str("That would sting. What did they say?"), py_str("Ugh, that's not a kind thing to say. How did you take it?"), py_str("That's the sort of thing that sits with you. Did you say anything back?")]));
    s_A_BILL_THAT_SAYS_OTHERWISE(py_tuple(vec![py_str("That's worth checking before you pay again. What does the bill say?"), py_str("Ugh. What does their paperwork say you owe?"), py_str("That's easily done on their side. Have you got the receipt?")]));
    s_A_MEAL_THEY_WENT_WITHOUT(py_tuple(vec![py_str("That's a day with a hole in the middle of it. Is it no time to stop, or nothing in?"), py_str("A day with a meal missing in it drags from there on. What stopped you - the day, or the cupboard?"), py_str("Ugh, that leaves you running on nothing. Get something in you now if you can - even toast counts."), py_str("That's a miserable way to get through a day. Get something in you soon, whatever is quickest.")]));
    s__A_JOB_NEEDED(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the)\\s+(?:[\\w']+\\s+){0,2}?(?:needs?|needed)\\s+(?:a\\s+|an\\s+|some\\s+)?(?:new\\s+[\\w']+|doing|fixing|replacing|repairing|painting|servicing|cleaning|sorting|resurfacing|cutting|work|doing\\s+up|service|look|quote|check|part|parts|job)\\b|\\b(?:said|says|told\\s+me)\\s+(?:it|the\\s+[\\w']+)\\s+(?:needs?|would\\s+need)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_TRADE(stdlib_call("re", "compile", &[py_str("\\b(?:homework|schoolwork|coursework|essay|essays|dissertation|revision|assignment|assignments|reading|forms?|paperwork|admin|tax\\s+return|shopping|washing|laundry|ironing|dishes|washing[-\\s]?up|sink|hoovering|vacuuming|tidying|housework|cleaning|bins|bin|cv|applications?|invoices?|accounts|bookkeeping|packing|unpacking|bathroom|bedroom|beds?|cooking|dinner|tea|supper|lunch|inbox|emails?|photos?|backups?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PERSON_NEEDING(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their)\\s+(?:mum|mom|mother|dad|father|son|daughter|kid|kids|child|children|wife|husband|partner|baby|boy|girl|mate|friend|boss|brother|sister|nan|gran|grandad|grandma|dog|cat|puppy|kitten|teeth|tooth|gums|hair|nails|knee|back|hip|shoulder|ankle|wrist|elbow|neck|eyes|skin)\\b\\s+(?:needs?|needed|wants?|has|have)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_JOB_THAT_IS_SORTED(stdlib_call("re", "compile", &[py_str("\\b(?:sorted|done|fixed|replaced|booked|arranged|coming\\s+(?:on|tomorrow|next)|quoted|priced|paid\\s+for|had\\s+a\\s+quote|got\\s+a\\s+quote|got\\s+a\\s+price|had\\s+a\\s+price|been\\s+quoted)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHORE_WAITING(py_tuple(vec![py_str("It is one job rather than a day's work. Ten minutes on it now, and stop when the timer goes - starting is the part that was making it feel bigger than it is."), py_str("Give it a quarter of an hour and see where you are. One job of that size fits in an evening, and whatever is left of it will keep until tomorrow."), py_str("Put a timer on and do the part you can see. It is one thing, not a verdict on your week, and you can stop at ten minutes with it half done.")]));
    s_A_CHORE_PILE(py_tuple(vec![py_str("One of those jobs that never gets shorter. Pick the one you would notice most if it were done and do that first; the rest will still be there tonight, and that is allowed."), py_str("That sort of pile is normal and it is not a moral failing. Do the smallest one on the list now, while the kettle boils, and it stops feeling like a wall."), py_str("Nothing broken, just a pile. Write them down, do the one that takes ten minutes, and leave the rest for a day when you have the energy.")]));
    s__PILE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:pile|piles|mountain|backlog|list|mess|state|everything|all\\s+of\\s+it|loads|loads\\s+of|lots|chores|housework|catch(?:ing)?\\s+up|behind\\s+on)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__CHORE_THINGS(stdlib_call("re", "compile", &[py_str("\\b(?:homework|schoolwork|coursework|essays?|dissertation|revision|assignments?|forms?|paperwork|admin|shopping|washing|laundry|ironing|dishes|washing[-\\s]?up|hoovering|vacuuming|tidying|cleaning|bins?|cv|applications?|invoices?|packing|unpacking|bathroom|bedroom|beds?|cooking|inbox|emails?|photos?|backups?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_JOB_SOMEBODY_ELSE_DOES(py_tuple(vec![py_str("That's a job for a trade rather than a guess. Ask for the price in writing first, and what it includes."), py_str("Worth two quotes on that one. The written quote that says what is included is the one to compare."), py_str("That's a cost to land on you. Have you had a price for it yet, or is that the next step?"), py_str("Ugh, one of those. Get what they would do and what it costs in writing before anybody starts.")]));
    s__TRIED_AND_IT_DID_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:ive|i'?ve|we'?ve|weve|i|we)\\s+(?:already\\s+)?(?:tried|attempted)\\b|\\btried\\s+(?:everything|it all|all of it|that|this|it|both|two|three|\\w+\\s+\\w+(?:es|s))\\b|\\bnothing\\s+(?:works|worked|has worked|i try|i do)\\b|\\b(?:didn'?t|doesn'?t|won'?t|wont|not)\\s+work(?:ed)?\\b|\\bno\\s+luck\\b|\\bstill\\s+(?:nothing|not|the same|doing it|happening)\\b|\\bkeeps?\\s+(?:doing|going|coming back|happening)\\b|\\bgave up on it\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__STILL_NO(stdlib_call("re", "compile", &[py_str("\\bstill\\s+(?:no\\s+[a-z]|without\\s+[a-z]|not\\s+got\\b|havent\\b|haven'?t\\b|waiting\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LACK_THAT_IS_STILL_GOING(py_tuple(vec![py_str("That's dragging on. How are you holding up with it?"), py_str("That's a long stretch of waiting for it. Is anything moving on it at all?"), py_str("Still nothing, then. That wears you down - what's the part you can move?"), py_str("That's hard going. Is it something you can chase, or does it just take time?")]));
    s__A_WAIT(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|he|she|they)\\s+(?:was|were|have been|had been)?\\s*wait(?:ed|ing)?\\b|\\bqueue(?:d|ds|ing|s)?\\b|\\bin\\s+the\\s+queue\\b|\\bsat\\s+in\\s+(?:a|the)\\s+waiting\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_LONG_WAIT(stdlib_call("re", "compile", &[py_str("\\b(?:hours?|hrs?|ages|half an hour|an hour|two hours|all morning|all afternoon|\\d+\\s*(?:hours?|hrs?))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_REMARK(stdlib_call("re", "compile", &[py_str("\\bpassed\\s+(?:a\\s+)?(?:comment|remark|comments|remarks)\\b|\\b(?:had|having)\\s+(?:a\\s+)?go\\s+at\\s+me\\b|\\b(?:moaned|went on|nagged|lectured)\\s+(?:at|to)\\s+me\\b|\\bsaid\\s+something\\s+(?:about|on)\\s+my\\b|\\bmade\\s+a\\s+(?:comment|remark|face)\\s+about\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DISPUTED_CHARGE(stdlib_call("re", "compile", &[py_str("\\bbut\\s+i\\s+(?:thought|was\\s+told|understood|believed)\\b[^.!?]{0,44}\\b(?:paid|pay|included|free|covered|booked|settled|refund(?:ed)?)\\b|\\b(?:they|it)\\s+says?\\s+i\\s+owe\\b|\\bcharged\\s+me\\s+again\\b|\\bi\\s+was\\s+told\\s+it\\s+was\\s+(?:free|included)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MEAL_MISSED(stdlib_call("re", "compile", &[py_str("\\b(?:went|gone|go|going|was)\\s+without\\s+(?:my\\s+|any\\s+|a\\s+|the\\s+|our\\s+)?(?:lunch|dinner|tea|breakfast|supper|food|a\\s+meal|meals|my\\s+meal)\\b|\\b(?:skipped|missed|skipping|missing)\\s+(?:my\\s+|a\\s+|the\\s+|our\\s+|any\\s+)?(?:lunch|dinner|tea|breakfast|supper|a\\s+meal|meals)\\b|\\bno\\s+(?:lunch|dinner|tea|breakfast|proper\\s+meal)\\b[^.!?]{0,14}\\b(?:again|today|yesterday|either)\\b|\\b(?:worked|working)\\s+through\\s+(?:my\\s+|a\\s+|the\\s+)?(?:lunch|dinner)\\b|\\bforgot\\s+(?:to\\s+eat|my\\s+(?:lunch|dinner|tea|breakfast|food))\\b|\\bdidn'?t\\s+(?:get|have|manage)\\s+(?:any\\s+|a\\s+|my\\s+)?(?:lunch|dinner|tea|breakfast|(?:proper\\s+)?meal)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MEAL_ON_PURPOSE(stdlib_call("re", "compile", &[py_str("\\b(?:on\\s+purpose|deliberately|on\\s+purpose|by\\s+choice|fasting|fast\\s+day|intermittent|diet(?:ing)?|cutting\\s+(?:out|down)|trying\\s+to\\s+(?:lose|lose\\s+weight)|skipping\\s+meals\\s+to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SOMEBODY_ELSES_MEAL(stdlib_call("re", "compile", &[py_str("\\b(?:the\\s+kids?|the\\s+children|my\\s+(?:son|daughter|boy|girl|lad|lass|husband|wife|partner|mum|mom|dad|brother|sister|friend|mate|flatmate|housemate)|he|she|they)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__MINE_OR_OURS(stdlib_call("re", "compile", &[py_str("\\b(?:i|ive|i'?m|im|i'?ve|we|weve|we'?ve|myself)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ASK_OPENING(stdlib_call("re", "compile", &[py_str("^(?:what|how|why|when|where|which|who|whose|is|are|was|were|do|does|did|can|could|should|would|will|shall|am|may|might|any\\s+chance|isnt|arent|doesnt|dont|didnt)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SHAKE_IS_THEIRS(stdlib_call("re", "compile", &[py_str("^\\s*(?:he|she|they|him|them|his|her|their)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_pile_of_chores(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line named more than one job, or said the whole of it is a state.");
    if py_truthy(&mcall(&g__PILE_WORDS(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    return op_gt(&call_builtin("len", &[call_builtin("set", &[mcall(&g__CHORE_THINGS(), "findall", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn reads_as_stuck_with_it(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line reports an attempt that failed, a wait, a remark that stung or a bill.\n\n    Whole shapes only, and never an ask: \"what if i tried again\" and \"how long should i wait\"\n    are questions with an answer of their own, and they are answered where they are read.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_GOOD_DESPITE(), "search", &[clean.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__AN_ASK_OPENING(), "match", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_DISPUTED_CHARGE(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_REMARK(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__TRIED_AND_IT_DID_NOT(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__A_WAIT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_LONG_WAIT(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_not_good(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("True when the line is a thing of theirs that has been damaged, taken, refused or fallen.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { mcall(&g_NOT_GOOD_DESPITE(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&reads_the_shake(&[clean.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_GOOD()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_the_shake(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a near miss or a fright - nothing landed, they are still shaken.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_FRIGHT(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_PLAN_THAT_FELL_THROUGH(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__A_BARE_NEAR_MISS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_THING_WITH_NO_BODY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_BODY_CAN_BE_HURT(), "search", &[clean.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_NEAR_MISS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FRIGHT(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_the_shake_of_someone_else(args: &[Py]) -> Py {
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
    let _ = py_str("True when the near miss or the fright happened to somebody the person is telling us about.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_the_shake(&[clean.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__SOMEONE_ELSE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__SHAKE_IS_THEIRS(), "match", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_glad(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the reply congratulates the news, or is a receipt of it with nothing after.");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    line = mcall(&mcall(&reply.clone(), "strip", &[]), "lower", &[]);
    return call_builtin("bool", &[{ let __b = mcall(&g_GLAD_OPENER(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_GLAD_ANYWHERE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BLANK_OR_THIN(), "match", &[line.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_meal_missed(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says they went without a meal of their own.\n\n    A meal has to be named (\"i missed the bus\", \"i skipped the gym\" are the same verb about\n    something else) and it has to be the person in front of the bot who went without it: \"my son\n    missed lunch at school\" happened to somebody else, and \"i skip lunch on purpose\" is a plan\n    rather than a bad day.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_MEAL_ON_PURPOSE(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_MEAL_MISSED(), "search", &[clean.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__MINE_OR_OURS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__SOMEBODY_ELSES_MEAL(), "search", &[clean.clone()]))) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_no_they_had(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line reports somebody refusing the person something.\n\n    A question is not a report (\"should i say no to her\"), a line the person said no in is their\n    own limit (\"i said no\", \"we said no to the quote\"), and a machine that will not start is not a\n    person: \"my car won't start\" is the car's answer.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&call_builtin("str", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_NO_STILL_BEING_WEIGHED(), "search", &[call_builtin("str", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = stdlib_call("re", "match", &[py_str("^(?:i|we)\\b"), call_builtin("str", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:she|he|they|my|our|the)\\b"), call_builtin("str", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__A_SAID_NO(), "search", &[call_builtin("str", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { mcall(&g__A_REFUSED(), "search", &[call_builtin("str", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e))]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _who_said_no(args: &[Py]) -> Py {
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
    let mut lean: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Who refused, said the way the person would hear it back: \"your boss\", \"the council\".");
    if py_truthy(&mcall(&g__A_REFUSAL_IN_THE_PASSIVE(), "search", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return py_str("");
    }
    who = py_str("");
    match_ = stdlib_call("re", "search", &[py_str("\\b((?:my|our|the)\\s+[a-z]+|mum|dad)\\b"), call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    who = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    lean = { let __b = mcall(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("leans_on")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    subject = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&lean.clone(), "get", &[py_str("subject")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&lean.clone(), "get", &[py_str("subject_type")]), &py_str("person"))) } }) {
    who = subject.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("your")); __args.push(who.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__MINE(), "sub", args))(&__args) }, "split", &[])]);
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
    else { Py::None };
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { Py::None };
    let mut clean: Py = Py::None;
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("A rotten thing answered as rotten, or None.\n\n    Called with the reply the composer chose, because the whole point is that no other module\n    wanted the turn: a line that has an answer of its own keeps it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_glad(&[reply.clone()])))) {
    return Py::None;
    }
    memory = { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    clean = mcall(&{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    clean = mcall(&py_str(" "), "join", &[mcall(&clean.clone(), "split", &[])]);
    if py_truthy(&reads_the_shake(&[clean.clone()])) {
    if py_truthy(&reads_the_shake_of_someone_else(&[clean.clone()])) {
    mine = if py_truthy(&mcall(&g_NEAR_MISS(), "search", &[clean.clone()])) { g_A_NEAR_MISS_ELSE() } else { g_A_FRIGHT_ELSE() };
    } else {
    if py_truthy(&mcall(&g_NEAR_MISS(), "search", &[clean.clone()])) {
    mine = g_A_NEAR_MISS();
    } else {
    mine = g_A_FRIGHT();
    }
    }
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&g_RESOLVED(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&reads_as_a_meal_missed(&[clean.clone()])) {
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_A_MEAL_THEY_WENT_WITHOUT()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g__A_JOB_NEEDED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__A_PERSON_NEEDING(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_JOB_THAT_IS_SORTED(), "search", &[clean.clone()]))) } } } }) {
    if py_truthy(&mcall(&g__NOT_A_TRADE(), "search", &[clean.clone()])) {
    pool = if py_truthy(&_a_pile_of_chores(&[clean.clone()])) { g_A_CHORE_PILE() } else { g_A_CHORE_WAITING() };
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_A_JOB_SOMEBODY_ELSE_DOES()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g__STILL_NO(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_WAIT(), "search", &[clean.clone()]))) } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_A_LACK_THAT_IS_STILL_GOING()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    }
    if py_truthy(&reads_as_stuck_with_it(&[clean.clone()])) {
    if py_truthy(&mcall(&g__A_DISPUTED_CHARGE(), "search", &[clean.clone()])) {
    pool = g_A_BILL_THAT_SAYS_OTHERWISE();
    } else {
    if py_truthy(&mcall(&g__A_REMARK(), "search", &[clean.clone()])) {
    pool = g_A_REMARK_THAT_STUNG();
    } else {
    if py_truthy(&{ let __b = mcall(&g__A_WAIT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_LONG_WAIT(), "search", &[clean.clone()]) } }) {
    pool = g_A_WAIT_THAT_WENT_ON();
    } else {
    pool = g_A_THING_THAT_DID_NOT_WORK();
    }
    }
    }
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_not_good(&[clean.clone()])))) {
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_no_they_had(&[clean.clone()])))) {
    return Py::None;
    }
    who = _who_said_no(&[clean.clone(), memory.clone()]);
    pool = if py_truthy(&who.clone()) { g_A_NO_FROM() } else { g_A_NO_ALREADY() };
    line = pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), op_add(&op_add(&clean.clone(), &py_str("|")), &who.clone())]);
    return if py_truthy(&who.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) } } else { line.clone() };
    }
    if py_truthy(&mcall(&g__FALL(), "search", &[clean.clone()])) {
    pool = if py_truthy(&mcall(&g__SOMEONE_ELSE(), "search", &[clean.clone()])) { g_A_FALL_ELSE() } else { g_A_FALL() };
    } else {
    if py_truthy(&mcall(&g__FROM_SOMEONE(), "search", &[clean.clone()])) {
    pool = g_A_BLOW_FROM_SOMEONE();
    } else {
    pool = g_DAMAGED_OR_TAKEN();
    }
    }
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.rotten_luck".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BILL_THAT_SAYS_OTHERWISE" => Some(g_A_BILL_THAT_SAYS_OTHERWISE()),
            "A_BLOW_FROM_SOMEONE" => Some(g_A_BLOW_FROM_SOMEONE()),
            "A_CHORE_PILE" => Some(g_A_CHORE_PILE()),
            "A_CHORE_WAITING" => Some(g_A_CHORE_WAITING()),
            "A_FALL" => Some(g_A_FALL()),
            "A_FALL_ELSE" => Some(g_A_FALL_ELSE()),
            "A_FRIGHT" => Some(g_A_FRIGHT()),
            "A_FRIGHT_ELSE" => Some(g_A_FRIGHT_ELSE()),
            "A_JOB_SOMEBODY_ELSE_DOES" => Some(g_A_JOB_SOMEBODY_ELSE_DOES()),
            "A_LACK_THAT_IS_STILL_GOING" => Some(g_A_LACK_THAT_IS_STILL_GOING()),
            "A_MEAL_THEY_WENT_WITHOUT" => Some(g_A_MEAL_THEY_WENT_WITHOUT()),
            "A_NEAR_MISS" => Some(g_A_NEAR_MISS()),
            "A_NEAR_MISS_ELSE" => Some(g_A_NEAR_MISS_ELSE()),
            "A_NO_ALREADY" => Some(g_A_NO_ALREADY()),
            "A_NO_FROM" => Some(g_A_NO_FROM()),
            "A_REMARK_THAT_STUNG" => Some(g_A_REMARK_THAT_STUNG()),
            "A_THING_THAT_DID_NOT_WORK" => Some(g_A_THING_THAT_DID_NOT_WORK()),
            "A_WAIT_THAT_WENT_ON" => Some(g_A_WAIT_THAT_WENT_ON()),
            "BLANK_OR_THIN" => Some(g_BLANK_OR_THIN()),
            "DAMAGED_OR_TAKEN" => Some(g_DAMAGED_OR_TAKEN()),
            "FRIGHT" => Some(g_FRIGHT()),
            "GLAD_ANYWHERE" => Some(g_GLAD_ANYWHERE()),
            "GLAD_OPENER" => Some(g_GLAD_OPENER()),
            "MISHAP" => Some(g_MISHAP()),
            "NEAR_MISS" => Some(g_NEAR_MISS()),
            "NOT_A_FRIGHT" => Some(g_NOT_A_FRIGHT()),
            "NOT_GOOD" => Some(g_NOT_GOOD()),
            "NOT_GOOD_DESPITE" => Some(g_NOT_GOOD_DESPITE()),
            "RESOLVED" => Some(g_RESOLVED()),
            "THEIR_PLACE" => Some(g_THEIR_PLACE()),
            "THEIR_THING" => Some(g_THEIR_THING()),
            "_AN_ASK_OPENING" => Some(g__AN_ASK_OPENING()),
            "_A_BARE_NEAR_MISS" => Some(g__A_BARE_NEAR_MISS()),
            "_A_BODY_CAN_BE_HURT" => Some(g__A_BODY_CAN_BE_HURT()),
            "_A_DISPUTED_CHARGE" => Some(g__A_DISPUTED_CHARGE()),
            "_A_JOB_NEEDED" => Some(g__A_JOB_NEEDED()),
            "_A_JOB_THAT_IS_SORTED" => Some(g__A_JOB_THAT_IS_SORTED()),
            "_A_LONG_WAIT" => Some(g__A_LONG_WAIT()),
            "_A_MEAL_MISSED" => Some(g__A_MEAL_MISSED()),
            "_A_MEAL_ON_PURPOSE" => Some(g__A_MEAL_ON_PURPOSE()),
            "_A_NO_STILL_BEING_WEIGHED" => Some(g__A_NO_STILL_BEING_WEIGHED()),
            "_A_PERSON_NEEDING" => Some(g__A_PERSON_NEEDING()),
            "_A_PLAN_THAT_FELL_THROUGH" => Some(g__A_PLAN_THAT_FELL_THROUGH()),
            "_A_REFUSAL_IN_THE_PASSIVE" => Some(g__A_REFUSAL_IN_THE_PASSIVE()),
            "_A_REFUSED" => Some(g__A_REFUSED()),
            "_A_REMARK" => Some(g__A_REMARK()),
            "_A_SAID_NO" => Some(g__A_SAID_NO()),
            "_A_THING_WITH_NO_BODY" => Some(g__A_THING_WITH_NO_BODY()),
            "_A_WAIT" => Some(g__A_WAIT()),
            "_CHORE_THINGS" => Some(g__CHORE_THINGS()),
            "_FALL" => Some(g__FALL()),
            "_FROM_SOMEONE" => Some(g__FROM_SOMEONE()),
            "_MINE" => Some(g__MINE()),
            "_MINE_OR_OURS" => Some(g__MINE_OR_OURS()),
            "_NOT_A_TRADE" => Some(g__NOT_A_TRADE()),
            "_PILE_WORDS" => Some(g__PILE_WORDS()),
            "_SHAKE_IS_THEIRS" => Some(g__SHAKE_IS_THEIRS()),
            "_SOMEBODY_ELSES_MEAL" => Some(g__SOMEBODY_ELSES_MEAL()),
            "_SOMEONE_ELSE" => Some(g__SOMEONE_ELSE()),
            "_STILL_NO" => Some(g__STILL_NO()),
            "_TRIED_AND_IT_DID_NOT" => Some(g__TRIED_AND_IT_DID_NOT()),
            "_a_pile_of_chores" => Some(py_func("_a_pile_of_chores", |args: &[Py]| _a_pile_of_chores(args))),
            "_who_said_no" => Some(py_func("_who_said_no", |args: &[Py]| _who_said_no(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "pick" => Some(g_pick()),
            "reads_as_a_meal_missed" => Some(py_func("reads_as_a_meal_missed", |args: &[Py]| reads_as_a_meal_missed(args))),
            "reads_as_a_no_they_had" => Some(py_func("reads_as_a_no_they_had", |args: &[Py]| reads_as_a_no_they_had(args))),
            "reads_as_glad" => Some(py_func("reads_as_glad", |args: &[Py]| reads_as_glad(args))),
            "reads_as_not_good" => Some(py_func("reads_as_not_good", |args: &[Py]| reads_as_not_good(args))),
            "reads_as_stuck_with_it" => Some(py_func("reads_as_stuck_with_it", |args: &[Py]| reads_as_stuck_with_it(args))),
            "reads_the_shake" => Some(py_func("reads_the_shake", |args: &[Py]| reads_the_shake(args))),
            "reads_the_shake_of_someone_else" => Some(py_func("reads_the_shake_of_someone_else", |args: &[Py]| reads_the_shake_of_someone_else(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BILL_THAT_SAYS_OTHERWISE" => s_A_BILL_THAT_SAYS_OTHERWISE(v),
            "A_BLOW_FROM_SOMEONE" => s_A_BLOW_FROM_SOMEONE(v),
            "A_CHORE_PILE" => s_A_CHORE_PILE(v),
            "A_CHORE_WAITING" => s_A_CHORE_WAITING(v),
            "A_FALL" => s_A_FALL(v),
            "A_FALL_ELSE" => s_A_FALL_ELSE(v),
            "A_FRIGHT" => s_A_FRIGHT(v),
            "A_FRIGHT_ELSE" => s_A_FRIGHT_ELSE(v),
            "A_JOB_SOMEBODY_ELSE_DOES" => s_A_JOB_SOMEBODY_ELSE_DOES(v),
            "A_LACK_THAT_IS_STILL_GOING" => s_A_LACK_THAT_IS_STILL_GOING(v),
            "A_MEAL_THEY_WENT_WITHOUT" => s_A_MEAL_THEY_WENT_WITHOUT(v),
            "A_NEAR_MISS" => s_A_NEAR_MISS(v),
            "A_NEAR_MISS_ELSE" => s_A_NEAR_MISS_ELSE(v),
            "A_NO_ALREADY" => s_A_NO_ALREADY(v),
            "A_NO_FROM" => s_A_NO_FROM(v),
            "A_REMARK_THAT_STUNG" => s_A_REMARK_THAT_STUNG(v),
            "A_THING_THAT_DID_NOT_WORK" => s_A_THING_THAT_DID_NOT_WORK(v),
            "A_WAIT_THAT_WENT_ON" => s_A_WAIT_THAT_WENT_ON(v),
            "BLANK_OR_THIN" => s_BLANK_OR_THIN(v),
            "DAMAGED_OR_TAKEN" => s_DAMAGED_OR_TAKEN(v),
            "FRIGHT" => s_FRIGHT(v),
            "GLAD_ANYWHERE" => s_GLAD_ANYWHERE(v),
            "GLAD_OPENER" => s_GLAD_OPENER(v),
            "MISHAP" => s_MISHAP(v),
            "NEAR_MISS" => s_NEAR_MISS(v),
            "NOT_A_FRIGHT" => s_NOT_A_FRIGHT(v),
            "NOT_GOOD" => s_NOT_GOOD(v),
            "NOT_GOOD_DESPITE" => s_NOT_GOOD_DESPITE(v),
            "RESOLVED" => s_RESOLVED(v),
            "THEIR_PLACE" => s_THEIR_PLACE(v),
            "THEIR_THING" => s_THEIR_THING(v),
            "_AN_ASK_OPENING" => s__AN_ASK_OPENING(v),
            "_A_BARE_NEAR_MISS" => s__A_BARE_NEAR_MISS(v),
            "_A_BODY_CAN_BE_HURT" => s__A_BODY_CAN_BE_HURT(v),
            "_A_DISPUTED_CHARGE" => s__A_DISPUTED_CHARGE(v),
            "_A_JOB_NEEDED" => s__A_JOB_NEEDED(v),
            "_A_JOB_THAT_IS_SORTED" => s__A_JOB_THAT_IS_SORTED(v),
            "_A_LONG_WAIT" => s__A_LONG_WAIT(v),
            "_A_MEAL_MISSED" => s__A_MEAL_MISSED(v),
            "_A_MEAL_ON_PURPOSE" => s__A_MEAL_ON_PURPOSE(v),
            "_A_NO_STILL_BEING_WEIGHED" => s__A_NO_STILL_BEING_WEIGHED(v),
            "_A_PERSON_NEEDING" => s__A_PERSON_NEEDING(v),
            "_A_PLAN_THAT_FELL_THROUGH" => s__A_PLAN_THAT_FELL_THROUGH(v),
            "_A_REFUSAL_IN_THE_PASSIVE" => s__A_REFUSAL_IN_THE_PASSIVE(v),
            "_A_REFUSED" => s__A_REFUSED(v),
            "_A_REMARK" => s__A_REMARK(v),
            "_A_SAID_NO" => s__A_SAID_NO(v),
            "_A_THING_WITH_NO_BODY" => s__A_THING_WITH_NO_BODY(v),
            "_A_WAIT" => s__A_WAIT(v),
            "_CHORE_THINGS" => s__CHORE_THINGS(v),
            "_FALL" => s__FALL(v),
            "_FROM_SOMEONE" => s__FROM_SOMEONE(v),
            "_MINE" => s__MINE(v),
            "_MINE_OR_OURS" => s__MINE_OR_OURS(v),
            "_NOT_A_TRADE" => s__NOT_A_TRADE(v),
            "_PILE_WORDS" => s__PILE_WORDS(v),
            "_SHAKE_IS_THEIRS" => s__SHAKE_IS_THEIRS(v),
            "_SOMEBODY_ELSES_MEAL" => s__SOMEBODY_ELSES_MEAL(v),
            "_SOMEONE_ELSE" => s__SOMEONE_ELSE(v),
            "_STILL_NO" => s__STILL_NO(v),
            "_TRIED_AND_IT_DID_NOT" => s__TRIED_AND_IT_DID_NOT(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.rotten_luck", module_obj);
    pyrt::xrefs::reg_fn("compose.rotten_luck", "_a_pile_of_chores", py_func("_a_pile_of_chores", |args: &[Py]| _a_pile_of_chores(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "reads_as_stuck_with_it", py_func("reads_as_stuck_with_it", |args: &[Py]| reads_as_stuck_with_it(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "reads_as_not_good", py_func("reads_as_not_good", |args: &[Py]| reads_as_not_good(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "reads_the_shake", py_func("reads_the_shake", |args: &[Py]| reads_the_shake(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "reads_the_shake_of_someone_else", py_func("reads_the_shake_of_someone_else", |args: &[Py]| reads_the_shake_of_someone_else(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "reads_as_glad", py_func("reads_as_glad", |args: &[Py]| reads_as_glad(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "reads_as_a_meal_missed", py_func("reads_as_a_meal_missed", |args: &[Py]| reads_as_a_meal_missed(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "reads_as_a_no_they_had", py_func("reads_as_a_no_they_had", |args: &[Py]| reads_as_a_no_they_had(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "_who_said_no", py_func("_who_said_no", |args: &[Py]| _who_said_no(args)));
    pyrt::xrefs::reg_fn("compose.rotten_luck", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_BILL_THAT_SAYS_OTHERWISE", g_A_BILL_THAT_SAYS_OTHERWISE, s_A_BILL_THAT_SAYS_OTHERWISE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_BLOW_FROM_SOMEONE", g_A_BLOW_FROM_SOMEONE, s_A_BLOW_FROM_SOMEONE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_CHORE_PILE", g_A_CHORE_PILE, s_A_CHORE_PILE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_CHORE_WAITING", g_A_CHORE_WAITING, s_A_CHORE_WAITING);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_FALL", g_A_FALL, s_A_FALL);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_FALL_ELSE", g_A_FALL_ELSE, s_A_FALL_ELSE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_FRIGHT", g_A_FRIGHT, s_A_FRIGHT);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_FRIGHT_ELSE", g_A_FRIGHT_ELSE, s_A_FRIGHT_ELSE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_JOB_SOMEBODY_ELSE_DOES", g_A_JOB_SOMEBODY_ELSE_DOES, s_A_JOB_SOMEBODY_ELSE_DOES);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_LACK_THAT_IS_STILL_GOING", g_A_LACK_THAT_IS_STILL_GOING, s_A_LACK_THAT_IS_STILL_GOING);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_MEAL_THEY_WENT_WITHOUT", g_A_MEAL_THEY_WENT_WITHOUT, s_A_MEAL_THEY_WENT_WITHOUT);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_NEAR_MISS", g_A_NEAR_MISS, s_A_NEAR_MISS);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_NEAR_MISS_ELSE", g_A_NEAR_MISS_ELSE, s_A_NEAR_MISS_ELSE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_NO_ALREADY", g_A_NO_ALREADY, s_A_NO_ALREADY);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_NO_FROM", g_A_NO_FROM, s_A_NO_FROM);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_REMARK_THAT_STUNG", g_A_REMARK_THAT_STUNG, s_A_REMARK_THAT_STUNG);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_THING_THAT_DID_NOT_WORK", g_A_THING_THAT_DID_NOT_WORK, s_A_THING_THAT_DID_NOT_WORK);
    pyrt::xrefs::reg_access("compose.rotten_luck", "A_WAIT_THAT_WENT_ON", g_A_WAIT_THAT_WENT_ON, s_A_WAIT_THAT_WENT_ON);
    pyrt::xrefs::reg_access("compose.rotten_luck", "BLANK_OR_THIN", g_BLANK_OR_THIN, s_BLANK_OR_THIN);
    pyrt::xrefs::reg_access("compose.rotten_luck", "DAMAGED_OR_TAKEN", g_DAMAGED_OR_TAKEN, s_DAMAGED_OR_TAKEN);
    pyrt::xrefs::reg_access("compose.rotten_luck", "FRIGHT", g_FRIGHT, s_FRIGHT);
    pyrt::xrefs::reg_access("compose.rotten_luck", "GLAD_ANYWHERE", g_GLAD_ANYWHERE, s_GLAD_ANYWHERE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "GLAD_OPENER", g_GLAD_OPENER, s_GLAD_OPENER);
    pyrt::xrefs::reg_access("compose.rotten_luck", "MISHAP", g_MISHAP, s_MISHAP);
    pyrt::xrefs::reg_access("compose.rotten_luck", "NEAR_MISS", g_NEAR_MISS, s_NEAR_MISS);
    pyrt::xrefs::reg_access("compose.rotten_luck", "NOT_A_FRIGHT", g_NOT_A_FRIGHT, s_NOT_A_FRIGHT);
    pyrt::xrefs::reg_access("compose.rotten_luck", "NOT_GOOD", g_NOT_GOOD, s_NOT_GOOD);
    pyrt::xrefs::reg_access("compose.rotten_luck", "NOT_GOOD_DESPITE", g_NOT_GOOD_DESPITE, s_NOT_GOOD_DESPITE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "RESOLVED", g_RESOLVED, s_RESOLVED);
    pyrt::xrefs::reg_access("compose.rotten_luck", "THEIR_PLACE", g_THEIR_PLACE, s_THEIR_PLACE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "THEIR_THING", g_THEIR_THING, s_THEIR_THING);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_AN_ASK_OPENING", g__AN_ASK_OPENING, s__AN_ASK_OPENING);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_BARE_NEAR_MISS", g__A_BARE_NEAR_MISS, s__A_BARE_NEAR_MISS);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_BODY_CAN_BE_HURT", g__A_BODY_CAN_BE_HURT, s__A_BODY_CAN_BE_HURT);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_DISPUTED_CHARGE", g__A_DISPUTED_CHARGE, s__A_DISPUTED_CHARGE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_JOB_NEEDED", g__A_JOB_NEEDED, s__A_JOB_NEEDED);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_JOB_THAT_IS_SORTED", g__A_JOB_THAT_IS_SORTED, s__A_JOB_THAT_IS_SORTED);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_LONG_WAIT", g__A_LONG_WAIT, s__A_LONG_WAIT);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_MEAL_MISSED", g__A_MEAL_MISSED, s__A_MEAL_MISSED);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_MEAL_ON_PURPOSE", g__A_MEAL_ON_PURPOSE, s__A_MEAL_ON_PURPOSE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_NO_STILL_BEING_WEIGHED", g__A_NO_STILL_BEING_WEIGHED, s__A_NO_STILL_BEING_WEIGHED);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_PERSON_NEEDING", g__A_PERSON_NEEDING, s__A_PERSON_NEEDING);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_PLAN_THAT_FELL_THROUGH", g__A_PLAN_THAT_FELL_THROUGH, s__A_PLAN_THAT_FELL_THROUGH);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_REFUSAL_IN_THE_PASSIVE", g__A_REFUSAL_IN_THE_PASSIVE, s__A_REFUSAL_IN_THE_PASSIVE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_REFUSED", g__A_REFUSED, s__A_REFUSED);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_REMARK", g__A_REMARK, s__A_REMARK);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_SAID_NO", g__A_SAID_NO, s__A_SAID_NO);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_THING_WITH_NO_BODY", g__A_THING_WITH_NO_BODY, s__A_THING_WITH_NO_BODY);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_A_WAIT", g__A_WAIT, s__A_WAIT);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_CHORE_THINGS", g__CHORE_THINGS, s__CHORE_THINGS);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_FALL", g__FALL, s__FALL);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_FROM_SOMEONE", g__FROM_SOMEONE, s__FROM_SOMEONE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_MINE", g__MINE, s__MINE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_MINE_OR_OURS", g__MINE_OR_OURS, s__MINE_OR_OURS);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_NOT_A_TRADE", g__NOT_A_TRADE, s__NOT_A_TRADE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_PILE_WORDS", g__PILE_WORDS, s__PILE_WORDS);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_SHAKE_IS_THEIRS", g__SHAKE_IS_THEIRS, s__SHAKE_IS_THEIRS);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_SOMEBODY_ELSES_MEAL", g__SOMEBODY_ELSES_MEAL, s__SOMEBODY_ELSES_MEAL);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_SOMEONE_ELSE", g__SOMEONE_ELSE, s__SOMEONE_ELSE);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_STILL_NO", g__STILL_NO, s__STILL_NO);
    pyrt::xrefs::reg_access("compose.rotten_luck", "_TRIED_AND_IT_DID_NOT", g__TRIED_AND_IT_DID_NOT, s__TRIED_AND_IT_DID_NOT);
    pyrt::xrefs::reg_access("compose.rotten_luck", "pick", g_pick, s_pick);
}
