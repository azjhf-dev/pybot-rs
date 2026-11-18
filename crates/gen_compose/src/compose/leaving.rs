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


thread_local! { static G_CRISIS_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KINDER_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NIGHT_GOODBYE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NIGHT_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_NEWS_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REASON_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WELL_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WISH_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ADVERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AWAY_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AWAY_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DAY_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LEAVE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SIGN_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BED_LEAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COMES_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTRAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__END_SIGN_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__END_SIGN_OFF_KEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEELS_ROTTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GOING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HURTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAVE_SHORTHAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEXT_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_BEDTIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENS_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OVERWHELM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RUN_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIGN_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIGN_OFF_VOCATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIGN_OFF_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPLIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THANKS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNWELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNWELL_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WISH_ALLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WISH_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_CRISIS_POOL() -> Py { ensure_init(); G_CRISIS_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_POOL(v: Py) { ensure_init(); G_CRISIS_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_KINDER_POOL() -> Py { ensure_init(); G_KINDER_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KINDER_POOL(v: Py) { ensure_init(); G_KINDER_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAVING() -> Py { ensure_init(); G_LEAVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAVING(v: Py) { ensure_init(); G_LEAVING.with(|c| *c.borrow_mut() = v); }
pub fn g_NIGHT_GOODBYE_POOL() -> Py { ensure_init(); G_NIGHT_GOODBYE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NIGHT_GOODBYE_POOL(v: Py) { ensure_init(); G_NIGHT_GOODBYE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_NIGHT_POOL() -> Py { ensure_init(); G_NIGHT_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NIGHT_POOL(v: Py) { ensure_init(); G_NIGHT_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_NEWS_POOL() -> Py { ensure_init(); G_OWN_NEWS_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_NEWS_POOL(v: Py) { ensure_init(); G_OWN_NEWS_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_POOL() -> Py { ensure_init(); G_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOL(v: Py) { ensure_init(); G_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_REASON_POOL() -> Py { ensure_init(); G_REASON_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REASON_POOL(v: Py) { ensure_init(); G_REASON_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_WELL_POOL() -> Py { ensure_init(); G_WELL_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WELL_POOL(v: Py) { ensure_init(); G_WELL_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_WISH_POOL() -> Py { ensure_init(); G_WISH_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WISH_POOL(v: Py) { ensure_init(); G_WISH_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g__ADVERB() -> Py { ensure_init(); G__ADVERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ADVERB(v: Py) { ensure_init(); G__ADVERB.with(|c| *c.borrow_mut() = v); }
pub fn g__ALLOWED() -> Py { ensure_init(); G__ALLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALLOWED(v: Py) { ensure_init(); G__ALLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g__ANOTHER() -> Py { ensure_init(); G__ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANOTHER(v: Py) { ensure_init(); G__ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS() -> Py { ensure_init(); G__ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS(v: Py) { ensure_init(); G__ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_ANYWHERE() -> Py { ensure_init(); G__ASKS_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_ANYWHERE(v: Py) { ensure_init(); G__ASKS_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_OPENER() -> Py { ensure_init(); G__ASKS_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_OPENER(v: Py) { ensure_init(); G__ASKS_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g__AWAY() -> Py { ensure_init(); G__AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AWAY(v: Py) { ensure_init(); G__AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__AWAY_OUT() -> Py { ensure_init(); G__AWAY_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AWAY_OUT(v: Py) { ensure_init(); G__AWAY_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__AWAY_PLAN() -> Py { ensure_init(); G__AWAY_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AWAY_PLAN(v: Py) { ensure_init(); G__AWAY_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DAY_AHEAD() -> Py { ensure_init(); G__A_DAY_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DAY_AHEAD(v: Py) { ensure_init(); G__A_DAY_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LEAVE_TIME() -> Py { ensure_init(); G__A_LEAVE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LEAVE_TIME(v: Py) { ensure_init(); G__A_LEAVE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SIGN_OFF() -> Py { ensure_init(); G__A_SIGN_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SIGN_OFF(v: Py) { ensure_init(); G__A_SIGN_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g__BED_LEAVE() -> Py { ensure_init(); G__BED_LEAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BED_LEAVE(v: Py) { ensure_init(); G__BED_LEAVE.with(|c| *c.borrow_mut() = v); }
pub fn g__BODY() -> Py { ensure_init(); G__BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BODY(v: Py) { ensure_init(); G__BODY.with(|c| *c.borrow_mut() = v); }
pub fn g__COMES_BACK() -> Py { ensure_init(); G__COMES_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COMES_BACK(v: Py) { ensure_init(); G__COMES_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTRAST() -> Py { ensure_init(); G__CONTRAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTRAST(v: Py) { ensure_init(); G__CONTRAST.with(|c| *c.borrow_mut() = v); }
pub fn g__END_SIGN_OFF() -> Py { ensure_init(); G__END_SIGN_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__END_SIGN_OFF(v: Py) { ensure_init(); G__END_SIGN_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g__END_SIGN_OFF_KEEP() -> Py { ensure_init(); G__END_SIGN_OFF_KEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__END_SIGN_OFF_KEEP(v: Py) { ensure_init(); G__END_SIGN_OFF_KEEP.with(|c| *c.borrow_mut() = v); }
pub fn g__FEELS_ROTTEN() -> Py { ensure_init(); G__FEELS_ROTTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEELS_ROTTEN(v: Py) { ensure_init(); G__FEELS_ROTTEN.with(|c| *c.borrow_mut() = v); }
pub fn g__GOING_VERB() -> Py { ensure_init(); G__GOING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GOING_VERB(v: Py) { ensure_init(); G__GOING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__HURTING() -> Py { ensure_init(); G__HURTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HURTING(v: Py) { ensure_init(); G__HURTING.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEPS() -> Py { ensure_init(); G__KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEPS(v: Py) { ensure_init(); G__KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAVE() -> Py { ensure_init(); G__LEAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAVE(v: Py) { ensure_init(); G__LEAVE.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAVE_SHORTHAND() -> Py { ensure_init(); G__LEAVE_SHORTHAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAVE_SHORTHAND(v: Py) { ensure_init(); G__LEAVE_SHORTHAND.with(|c| *c.borrow_mut() = v); }
pub fn g__MIND() -> Py { ensure_init(); G__MIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MIND(v: Py) { ensure_init(); G__MIND.with(|c| *c.borrow_mut() = v); }
pub fn g__NEXT_DAY() -> Py { ensure_init(); G__NEXT_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEXT_DAY(v: Py) { ensure_init(); G__NEXT_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__NIGHT() -> Py { ensure_init(); G__NIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NIGHT(v: Py) { ensure_init(); G__NIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__NITE() -> Py { ensure_init(); G__NITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NITE(v: Py) { ensure_init(); G__NITE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_BEDTIME() -> Py { ensure_init(); G__NOT_A_BEDTIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_BEDTIME(v: Py) { ensure_init(); G__NOT_A_BEDTIME.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENS_QUESTION() -> Py { ensure_init(); G__OPENS_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENS_QUESTION(v: Py) { ensure_init(); G__OPENS_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__OVERWHELM() -> Py { ensure_init(); G__OVERWHELM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OVERWHELM(v: Py) { ensure_init(); G__OVERWHELM.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_STATE() -> Py { ensure_init(); G__OWN_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_STATE(v: Py) { ensure_init(); G__OWN_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g__RUN_UP() -> Py { ensure_init(); G__RUN_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RUN_UP(v: Py) { ensure_init(); G__RUN_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__SIGN_OFF() -> Py { ensure_init(); G__SIGN_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIGN_OFF(v: Py) { ensure_init(); G__SIGN_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g__SIGN_OFF_VOCATIVE() -> Py { ensure_init(); G__SIGN_OFF_VOCATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIGN_OFF_VOCATIVE(v: Py) { ensure_init(); G__SIGN_OFF_VOCATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__SIGN_OFF_WORDS() -> Py { ensure_init(); G__SIGN_OFF_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIGN_OFF_WORDS(v: Py) { ensure_init(); G__SIGN_OFF_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SPLIT() -> Py { ensure_init(); G__SPLIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPLIT(v: Py) { ensure_init(); G__SPLIT.with(|c| *c.borrow_mut() = v); }
pub fn g__THANKS_WORDS() -> Py { ensure_init(); G__THANKS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THANKS_WORDS(v: Py) { ensure_init(); G__THANKS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__UNWELL() -> Py { ensure_init(); G__UNWELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNWELL(v: Py) { ensure_init(); G__UNWELL.with(|c| *c.borrow_mut() = v); }
pub fn g__UNWELL_FEELING() -> Py { ensure_init(); G__UNWELL_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNWELL_FEELING(v: Py) { ensure_init(); G__UNWELL_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g__WELL() -> Py { ensure_init(); G__WELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WELL(v: Py) { ensure_init(); G__WELL.with(|c| *c.borrow_mut() = v); }
pub fn g__WISH() -> Py { ensure_init(); G__WISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WISH(v: Py) { ensure_init(); G__WISH.with(|c| *c.borrow_mut() = v); }
pub fn g__WISH_ALLOWED() -> Py { ensure_init(); G__WISH_ALLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WISH_ALLOWED(v: Py) { ensure_init(); G__WISH_ALLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g__WISH_THING() -> Py { ensure_init(); G__WISH_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WISH_THING(v: Py) { ensure_init(); G__WISH_THING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that says the person is going: the last thing said in a chat.\n\nPeople end conversations all day, and they do it the way they start one - with a little\nfiller in front and a reason behind: \"anyway i better go, hes crying\", \"ill let you go,\ndinner is burning\", \"right im off to bed\". No knowledge entry owns a goodbye, so before this\nthe answer was whatever the reason clause matched (\"dinner is burning\" got *\"take it off the\nheat\"*), or the honest search shrug for a question nobody asked.\n\nTwo things are needed for that, and both live here so they cannot drift apart:\n\n* `reads_as_leaving(clean)` - the reading. The leaving has to be the *clause*, not a word in\n  it: \"i better go\" and \"im off to bed\" are leaves, \"i better go to the chemist for a rash\"\n  is a plan and \"should i let my son go on the trip\" is a question. A reason after the\n  leaving (\"got to go, the kids are up\") does not change what the line is.\n* `pool(clean)` - the wording. A goodbye deserves a goodbye, in the bot's own voice, with a\n  little variety so no two in a row are the same.\n\nThe arbiter reads the kind (`fallback.reading` returns it) so a topic entry cannot take the\nline back on a word of the reason clause.\n");
    s_LEAVING(py_str("leaving"));
    s__A_SIGN_OFF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:(?:catch|see)\\s+(?:you|ya|u)\\b|(?:talk|speak|chat)(?:ing)?\\s+(?:to\\s+)?(?:you|ya|u)\\b|(?:c\\s*u|c\\s*ya)\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_LEAVE_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:(?:at|around|about)\\s+(?:about\\s+|around\\s+|half\\s+)?\\d{1,2}(?::\\d{2})?\\s*(?:am|pm|o'?clock)?|(?:at|around|about)\\s+(?:about\\s+|around\\s+|half\\s+)?(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b|at\\s+(?:lunch|lunchtime|teatime|breakfast|break)\\b|after\\s+(?:work|school|college|uni|university|lunch|tea|dinner|the\\s+school\\s+run)\\b|on\\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|weds|thurs|fri|sat|sun)\\b|this\\s+(?:evening|afternoon|weekend)\\b|next\\s+week\\b|at\\s+the\\s+weekend\\b|in\\s+the\\s+(?:morning|afternoon|evening)\\b|in\\s+(?:a\\s+)?(?:bit|while|minute|sec|second|mo|moment)\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__GOING_VERB(py_str("(?:go|get\\s+off|get\\s+going|be\\s+off|be\\s+going|head\\s+off|shoot|shoot\\s+off|run|dash|fly)"));
    s__LEAVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:(?:got|gotta|have|has|need|needs|better|best|must|ought|should)\\s+(?:to\\s+)?"), &g__GOING_VERB()), &py_str("|(?:i'?m|im|i\\s+am|we'?re|we\\s+are|i'?ll|ill)\\s+(?:gonna|going\\s+to)\\s+(?:go|be\\s+off|head\\s+off|shoot(?:\\s+off)?|get\\s+off)\\b|(?:i'?m|im|i\\s+am|we'?re|we\\s+are)\\s+(?:just\\s+)?(?:gonna\\s+|going\\s+to\\s+)?(?:turn(?:ing)?\\s+in|going\\s+up\\s+to\\s+bed|signing\\s+off|up\\s+to\\s+bed)\\b|(?:i'?m|im|i\\s+am)\\s+(?:gonna\\s+|going\\s+to\\s+|just\\s+)?(?:get|getting)\\s+(?:some\\s+)?(?:kip|kips|shut\\s?eye|shuteye|sleep)\\b|(?:i'?m|im|i\\s+am|we'?re|we\\s+are)\\s+(?:just\\s+|probably\\s+)?(?:having|going\\s+to\\s+have|off\\s+for|going\\s+for)\\s+an\\s+early\\s+(?:night|one)\\b|(?:(?:an\\s+)?early\\s+night)\\b|(?:time\\s+for\\s+an\\s+early\\s+night)\\b|(?:^just\\s+having\\s+an\\s+early\\s+(?:night|one))\\b|(?:i'?m|im|i\\s+am|we'?re|we\\s+are)\\s+(?:just\\s+)?(?:going\\s+to\\s+|gonna\\s+)?(?:get|getting)\\s+my\\s+head\\s+down\\b|(?:i'?m|im|i\\s+am)\\s+(?:just\\s+)?(?:heading|going)\\s+up\\s+to\\s+(?:my\\s+)?bed\\b|(?:i'?m|im|i\\s+am)\\s+(?:just\\s+)?(?:going\\s+to\\s+|gonna\\s+)?try\\s+(?:and\\s+|to\\s+)(?:sleep|get\\s+some\\s+sleep)\\b|(?:time\\s+to\\s+turn\\s+in)\\b|(?:bed\\s*o'?\\s*clock)\\b|(?:(?:it'?s|its)\\s+)?sleep\\s*time\\b|(?:i'?m|im|i\\s+am|we'?re|we\\s+are|i'?ll|ill)\\s+(?:be\\s+)?(?:off|going|head(?:ing)?\\s+off|shoot(?:ing)?\\s+off|out\\s+of\\s+here|get\\s+(?:off|going))\\b|(?:i'?m|im|i\\s+am|we'?re|we\\s+are)\\s+(?:away|out)\\b|(?:off\\s+to\\s+(?:bed|sleep|kip|nap|work|school|the\\s+shop))\\b|(?:going\\s+for\\s+a\\s+(?:kip|nap|lie\\s+down))\\b|(?:hitting\\s+the\\s+hay)\\b|(?:(?:bed|bed\\s*time)\\s+for\\s+(?:me|us))\\b|(?:bed\\s*time)\\b|(?:call(?:ing)?\\s+it\\s+a\\s+night)\\b|(?:^going\\s+up\\s+to\\s+(?:my\\s+)?bed)\\b|(?:^just\\s+going\\s+up\\s+to\\s+(?:my\\s+)?bed)\\b|(?:\\bgood\\s?night\\b|\\bgood\\s?nite\\b|\\bnite\\b|\\bnight\\b)|(?:get(?:ting)?\\s+out\\s+of\\s+your\\s+hair)\\b|(?:time\\s+(?:for\\s+me\\s+)?to\\s+(?:go|head\\s+off|be\\s+off|shoot(?:\\s+off)?))\\b|(?:(?:that'?s|thats)\\s+me\\s+(?:away|off|out|done))\\b|(?:let\\s+you\\s+go|leave\\s+you\\s+to\\s+it|leave\\s+you\\s+be|let\\s+you\\s+get\\s+on|let\\s+you\\s+get\\s+back|stop\\s+bothering\\s+you)\\b|(?:talk|speak|chat)(?:ing)?\\s+(?:to\\s+you\\s+)?(?:later|soon|tomorrow)\\b|(?:talk|speak|chat)(?:ing)?\\s+(?:to\\s+you\\s+)?(?:")), &gattr(&g__A_LEAVE_TIME(), "pattern")), &py_str(")|(?:catch|see)\\s+(?:you|ya)\\b|(?:time\\s+(?:for\\s+me\\s+)?to\\s+go)\\b|(?:that'?s|thats)\\s+(?:me\\s+)?done\\b|(?:that'?s|thats)\\s+it\\s+from\\s+me\\b|(?:i'?m|im|i\\s+am)\\s+done\\s+for\\s+(?:the\\s+)?(?:day|today|now)\\b|(?:time\\s+(?:i\\s+was\\s+off|(?:for\\s+)?(?:my\\s+)?bed))\\b|(?:need|needs)\\s+my\\s+bed\\b|(?:my\\s+bed'?s?\\s+calling)\\b)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DAY_AHEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tomorrow|tonight|on\\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|weds|thurs|fri|sat|sun)\\b|next\\s+(?:week|weekend|month|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|this\\s+(?:evening|afternoon|weekend)|in\\s+the\\s+(?:morning|afternoon|evening)|later\\b|after\\s+(?:work|school|college|uni|university|lunch|tea|dinner|the\\s+school\\s+run)\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WISH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ok |okay |right |well |and |so |cheers |thanks |thank you |nice |great |bye )*(?:(?:i )?hope (?:you )?(?:have )?(?:a |an )?(?:really |very |lovely |nice |great |good |safe |smashing |brilliant |restful )*(?:day|evening|weekend|week|night|one|time|holiday|christmas|break|rest|trip|journey)\\b|(?:you )?have (?:a |an )?(?:really |very |lovely |nice |great |good |safe |smashing |brilliant |restful )*(?:day|evening|weekend|week|night|one|time|holiday|christmas|break|rest|trip|journey)\\b|(?:i )?hope (?:that )?(?:you\\b[^.!?,;]{0,30}?|your (?:day|evening|weekend|week|night|lunch|dinner|meal|shift|rest|break|time|one|trip|journey|holiday)\\b[^.!?,;]{0,30}?)(?:well|better|good|great|lovely|nice|fine|okay|smoothly|easily|restful|fun)\\b|(?:you )?have (?:a )?(?:good )?fun\\b|(?:you )?enjoy (?:a |an )?(?:good|great|nice|lovely|fun|happy) (?:day|evening|weekend|week|night|time|one|trip|journey|holiday|break|shift|lunch|dinner|meal|party)\\b|(?:you )?enjoy (?:your|the) (?:day|evening|weekend|night|lunch|dinner|meal|party|trip|holiday|break|shift|rest|bath|walk|flight)\\b|get well soon\\b|(?:you )?feel better(?: soon)?\\b|(?:you )?sleep (?:well|tight)\\b|(?:you )?take care\\b|all the best\\b|best wishes\\b|best regards\\b|(?:kind|warm) regards\\b|best of luck\\b|(?:good )?luck\\b|look after yourself\\b|laters\\b|laterz\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WISH_ALLOWED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("then"), py_str("now"), py_str("too"), py_str("both"), py_str("all"), py_str("and"), py_str("you"), py_str("thanks"), py_str("thank"), py_str("cheers"), py_str("bye"), py_str("byebye"), py_str("goodbye"), py_str("ok"), py_str("okay"), py_str("right"), py_str("well"), py_str("so"), py_str("x"), py_str("xx"), py_str("xxx"), py_str("mate"), py_str("guys"), py_str("folks"), py_str("everyone"), py_str("of"), py_str("yourself"), py_str("it"), py_str("to"), py_str("for"), py_str("the"), py_str("a"), py_str("have"), py_str("nice"), py_str("one"), py_str("at"), py_str("with"), py_str("in"), py_str("on"), py_str("work"), py_str("school"), py_str("class"), py_str("shift"), py_str("tonight"), py_str("tomorrow"), py_str("today"), py_str("soon"), py_str("away"), py_str("party"), py_str("interview"), py_str("meeting"), py_str("appointment"), py_str("exam"), py_str("test"), py_str("flight"), py_str("drive"), py_str("game"), py_str("match"), py_str("christmas"), py_str("new"), py_str("year"), py_str("your"), py_str("both"), py_str("as"), py_str("well"), py_str("everything"), py_str("feeling"), py_str("day"), py_str("night"), py_str("weekend"), py_str("trip"), py_str("journey"), py_str("home"), py_str("break"), py_str("lunch"), py_str("dinner"), py_str("meal"), py_str("rest"), py_str("sleep"), py_str("fun"), py_str("safe"), py_str("early"), py_str("late")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ALLOWED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ok"), py_str("okay"), py_str("right"), py_str("well"), py_str("anyway"), py_str("anyways"), py_str("alright"), py_str("oh"), py_str("so"), py_str("yeah"), py_str("yes"), py_str("and"), py_str("but"), py_str("sorry"), py_str("thanks"), py_str("thank"), py_str("ta"), py_str("hi"), py_str("hey"), py_str("listen"), py_str("look"), py_str("mm"), py_str("mhm"), py_str("i"), py_str("im"), py_str("ill"), py_str("id"), py_str("we"), py_str("us"), py_str("me"), py_str("you"), py_str("my"), py_str("the"), py_str("a"), py_str("an"), py_str("to"), py_str("for"), py_str("bit"), py_str("little"), py_str("while"), py_str("second"), py_str("minute"), py_str("sec"), py_str("now"), py_str("then"), py_str("soon"), py_str("later"), py_str("tonight"), py_str("today"), py_str("tomorrow"), py_str("morning"), py_str("night"), py_str("nite"), py_str("bed"), py_str("sleep"), py_str("work"), py_str("shop"), py_str("it"), py_str("home"), py_str("early"), py_str("one"), py_str("think"), py_str("day"), py_str("the"), py_str("that"), py_str("should"), py_str("gonna"), py_str("i'd"), py_str("i'll"), py_str("i'm"), py_str("i've"), py_str("we'd"), py_str("we'll"), py_str("we're"), py_str("it's"), py_str("that's"), py_str("love"), py_str("luv"), py_str("hun"), py_str("honey"), py_str("sweetheart"), py_str("mate"), py_str("folks"), py_str("xx"), py_str("x"), py_str("xxx"), py_str("kip"), py_str("nap"), py_str("let"), py_str("just"), py_str("about"), py_str("off")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OPENS_QUESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:can|could|should|shall|do|does|did|is|are|was|were|will|would|may|might|must|what|when|where|why|how|who|which|whose|if|whether)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|whats|why|whys|how|hows|when|whens|where|wheres|who|whos|which|should|do|does|did|can|could|would|will)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASKS_ANYWHERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:any\\s+(?:idea|ideas|tips?|advice|suggestions?|chance|thoughts?|clue|pointers?)|is\\s+there|are\\s+there|tell\\s+me|do\\s+you\\s+know|can\\s+you|could\\s+you|what\\s+do\\s+you|how\\s+do\\s+i|what\\s+should\\s+i|wondering\\s+if|if\\s+you\\s+could|what\\s+about|how\\s+about)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASKS_OPENER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:can|could|should|shall|do|does|did|is|are|was|were|will|would|may|might|must|what|whats|when|where|why|how|hows|who|whos|which|whose)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__KEEPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:dont|don't|doesnt|doesn't|didnt|didn't|not|wont|won't|never)\\b[^,.;]{0,24}\\b(?:go|going|leave|off)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SPLIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:[,;!?]|(?<!\\d)\\.|\\.(?!\\d)| - | -- |\\bbut\\b)\\s*")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POOL(py_tuple(vec![py_str("Go well, and see you when you're back."), py_str("Right, take it easy. Look after yourself."), py_str("No worries at all. Mind yourself."), py_str("Bye for now — mind how you go."), py_str("All the best — talk soon.")]));
    s_NIGHT_GOODBYE_POOL(py_tuple(vec![py_str("Night then. Sleep well."), py_str("Goodnight - sleep well."), py_str("Night night. Sleep well."), py_str("Right - goodnight, and sleep well."), py_str("Night. Sleep well, and I'll be here tomorrow.")]));
    s__NOT_A_BEDTIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:shift|shifts|bus|buses|duty|class|classes|out|club|feeds?|mare|terrors?|nurse|nursing|porter|stock)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NITE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:good\\s?night|good\\s?nite|nite|night|bed|bedtime|kip|hay|sleep|sleeping|turn(?:ing)?\\s+in|head\\s+down|shut\\s?eye|\\blie\\s+down)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REASON_POOL(py_tuple(vec![py_str("Go on then — that comes before me. Take care."), py_str("Fair enough, that matters more. Look after yourself."), py_str("Of course. Go and see to it, and mind yourself."), py_str("That first, definitely. Come back when you have a minute.")]));
    s__THANKS_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("thanks"), py_str("thank"), py_str("you"), py_str("for"), py_str("listening"), py_str("listened"), py_str("that"), py_str("helps"), py_str("helped"), py_str("so"), py_str("much"), py_str("a"), py_str("lot"), py_str("the"), py_str("chat"), py_str("talk"), py_str("talking"), py_str("it"), py_str("all"), py_str("help"), py_str("again"), py_str("was"), py_str("really"), py_str("useful"), py_str("advice"), py_str("everything"), py_str("mate"), py_str("and"), py_str("i"), py_str("appreciate"), py_str("means")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__RUN_UP(op_bitor(&op_bitor(&g__ALLOWED(), &g__THANKS_WORDS()), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ok"), py_str("okay"), py_str("right"), py_str("then"), py_str("your"), py_str("my"), py_str("our"), py_str("much")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s__SIGN_OFF_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("bye"), py_str("byebye"), py_str("goodbye"), py_str("later"), py_str("laters"), py_str("laterz"), py_str("night"), py_str("goodnight"), py_str("nite"), py_str("cheers"), py_str("ttfn"), py_str("ttyl"), py_str("cheerio"), py_str("tara"), py_str("adios"), py_str("nighty"), py_str("nites"), py_str("ok"), py_str("okay"), py_str("right"), py_str("well"), py_str("thanks"), py_str("thank"), py_str("you"), py_str("ta"), py_str("so"), py_str("and"), py_str("now"), py_str("then"), py_str("anyway"), py_str("anyways"), py_str("alright")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SIGN_OFF_VOCATIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("all"), py_str("everyone"), py_str("everybody"), py_str("folks"), py_str("guys"), py_str("love"), py_str("luv"), py_str("hun"), py_str("honey"), py_str("sweetheart"), py_str("mate"), py_str("xx"), py_str("x"), py_str("xxx"), py_str("too"), py_str("bella"), py_str("pet"), py_str("duck"), py_str("chick"), py_str("pal")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__END_SIGN_OFF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bye|byebye|goodbye|cheerio|tara|night|goodnight|nite|laters|night\\s+night|nite\\s+nite|ttfn|ttyl|adios|nighty|nites)\\b\\s*(?:all|everyone|everybody|folks|guys|too|love|hun|mate|bella|pet|duck|chick|pal)?\\s*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__END_SIGN_OFF_KEEP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ok"), py_str("okay"), py_str("right"), py_str("well"), py_str("so"), py_str("and"), py_str("then"), py_str("now"), py_str("for"), py_str("all"), py_str("everyone"), py_str("love"), py_str("luv"), py_str("hun"), py_str("honey"), py_str("sweetheart"), py_str("mate"), py_str("guys"), py_str("folks"), py_str("xx"), py_str("x"), py_str("xxx"), py_str("night"), py_str("nite"), py_str("good"), py_str("bye"), py_str("byebye"), py_str("goodbye"), py_str("cheerio"), py_str("tara"), py_str("laters"), py_str("you"), py_str("your"), py_str("i"), py_str("im"), py_str("we"), py_str("us"), py_str("me"), py_str("my"), py_str("the"), py_str("a"), py_str("an"), py_str("is"), py_str("be"), py_str("to"), py_str("have"), py_str("had")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__UNWELL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:sick|ill|poorly|unwell|hurts?|hurting|ached?|aches|aching|pain(?:ful)?|sore|stiff|headache|migraines?|toothache|cramps?|cramping|fever|temperature|vomit(?:ing|ed)?|throwing\\s+up|threw\\s+up|nausea|nauseous|dizzy|faint|flu|infection|cough(?:ing)?|rash|swollen|swelling|bleed(?:ing)?|blister(?:s|ed)?|stomach|diarr?hoea)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HURTING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:kill(?:s|ing)?(?:\\s+(?:me|my))?|agony|agonis(?:ing)?|agoniz(?:ing)?|throbbing|thumping|banging|pounding|hammering|splitting|stinging|burning|on\\s+fire|in\\s+bits|hurts?|hurting|aching|aches?|sore|stiff|tender|twinge|swollen|killing\\s+me)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:head|heads|throat|ear|ears|eye|eyes|tooth|teeth|back|backs|knee|knees|shoulder|shoulders|arm|arms|leg|legs|foot|feet|ankle|ankles|wrist|wrists|neck|hip|hips|toe|toes|hand|hands|finger|fingers|chest|stomach|belly|skin|jaw|sinus|sinuses|gum|gums|elbow|elbows|shin|shins|thigh|thighs)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__CONTRAST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:but|though|although)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COMES_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:keep|keeps|keeping|kept|cant|cannot|can'?t|still|always)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__MIND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:think|thinks|thinking|thought|thoughts|worry|worried|worrying|fret|fretting|overthink|overthinking|sleep|sleeping|asleep|awake|wake|wakes|waking|woke|dream|dreams|dreaming|night|nights|head|heads|headache|headaches|migraine|migraines|mind|brain|cry|crying|cried|tears|panic|panicking|stress|stressed|anxious|switching|switch)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OVERWHELM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i'?m|i am|i feel|im feeling|feeling|i've been feeling|ive been feeling)\\s+(?:so |really |a bit |very |totally |completely |proper )?overwhelm(?:ed|ing)\\b|\\b(?:i|we) can'?t (?:keep up|take any ?more|cope any ?more|manage any ?more)\\b|\\b(?:i'?m|i am|im) not coping\\b|\\bnot coping (?:at all|any ?more|right now)\\b|\\bit'?s more than i can (?:handle|take|cope with|manage|deal with)\\b|\\bmore than i can (?:handle|take|cope with|manage|deal with)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ADVERB(py_str("(?:still|just|really|actually|genuinely|very|so|pretty|quite|proper|sort of|kind of)"));
    s__FEELS_ROTTEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:i|we)\\s+(?:"), &g__ADVERB()), &py_str("\\s+)*feel(?:ing)?\\s+(?:")), &g__ADVERB()), &py_str("\\s+)*(?:like\\s+)?(?:rubbish|rough|awful|terrible|dreadful|poorly|crap|lousy|ill|sick|low|down|knackered|exhausted)\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SIGN_OFF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\n    take\\s+(?:it\\s+easy|care)|mind\\s+(?:yourself|how\\s+you\\s+go|out)\n    |look\\s+after\\s+yourself|see\\s+(?:you|ya)|bye|good\\s+luck|all\\s+the\\s+best\n    |go\\s+(?:well|safely|on\\s+then|gently)|talk\\s+(?:soon|later|tomorrow)|catch\\s+you\n    |come\\s+back|be\\s+safe|get\\s+home\\s+safe|on\\s+your\\s+way|off\\s+you\\s+go\n    |(?:let|leave)\\s+you\\s+(?:to\\s+it|go)|safe\\s+home|enjoy\\s+(?:it|yourself|the\\s+\\w+)\n    |have\\s+(?:a|an)\\s+(?:good|nice|great|lovely)\\s+\\w+\n    |hope\\s+(?:it|that|work|the\\s+\\w+)\\s+(?:goes|went|is)\\s+well\n)\\b")); __args.push(stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LEAVE_SHORTHAND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ttyl"), py_str("talklater"), py_str("gtg"), py_str("g2g"), py_str("cya"), py_str("l8r"), py_str("l8tr"), py_str("n8"), py_str("xx"), py_str("xxx")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AWAY_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i'?m|im|i\\s+am|we'?re|we\\s+are)\\s+(?:off|away|out)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AWAY_PLAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:next\\s+(?:week|month|year|weekend)|this\\s+(?:weekend|evening|afternoon)|tonight|tomorrow|on\\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)|at\\s+the\\s+weekend|for\\s+the\\s+weekend|in\\s+(?:june|july|august|september|october|november|december|january|february|march|april|may))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BED_LEAVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:early\\s+(?:night|one)|head\\s+down|heading\\s+up\\s+to\\s+bed|try\\s+(?:and\\s+|to\\s+)sleep)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NEXT_DAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tomorrow|tomorra|tmrw|tmoz|next\\s+(?:week|month|year|weekend)|on\\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AWAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i'?m|im|i am|we'?re|were|we are)\\s+(?:off|away)\\b|\\b(?:i'?m|im|i am)\\s+(?:heading|setting) off\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WISH_POOL(py_tuple(vec![py_str("Thanks — that's kind. Same to you."), py_str("Ta. You too — look after yourself."), py_str("Thanks, and the same back to you."), py_str("Cheers. Hope it goes well for you too."), py_str("Thank you — you too.")]));
    s_WELL_POOL(py_tuple(vec![py_str("Thanks — that's kind of you. I'm well, and I hope you are too."), py_str("Nothing wrong with me, but thank you. Look after yourself."), py_str("That's kind. I'm fine — and I hope you are too.")]));
    s_CRISIS_POOL(py_tuple(vec![py_str("That's kind of you — but you're the one I'm thinking about. How are you doing right now?"), py_str("Thank you, and I mean it. Never mind me — how is it now?"), py_str("That's kind. I'm all right, and I'd rather stay with you. What is happening right now?")]));
    s_KINDER_POOL(py_tuple(vec![py_str("Thanks — and I hope it eases soon."), py_str("Ta. Same to you — and I hope you feel better soon."), py_str("Thank you — and I hope whatever it is lets up a bit."), py_str("Thanks, and I hope you are feeling better soon.")]));
    s_NIGHT_POOL(py_tuple(vec![py_str("Thanks — and the same to you. Sleep well."), py_str("Ta. You too — get a good night."), py_str("Thank you. And I hope you sleep well too.")]));
    s_OWN_NEWS_POOL(py_tuple(vec![py_str("Never mind me — what's happening with you?"), py_str("Ah, that matters more than me. Tell me about it."), py_str("Then the wish can wait — what's going on?"), py_str("Forget the wish for a second. How is it going?")]));
    s__NIGHT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:sleep|sleeping|bed|bedtime|night's sleep|goodnight)\\b|\\bnight\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WELL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:get well|feel better|well soon)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WISH_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:day|evening|weekend|week|night|lunch|dinner|meal|party|trip|journey|holiday|break|shift|rest|bath|walk|flight|fun|well|better|tomorrow|tonight|today|sleep|tight|bed)\\b|\\bwith (?:the|your|it|that)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__UNWELL_FEELING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:awful|terrible|dreadful|rough|rubbish|lousy|miserable|cold|chills?|sniffles|sore throat|under the weather|poorly|poor thing)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OWN_STATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tired|shattered|knackered|exhausted|worn\\s+out|done\\s+in|wrecked|cream\\s+crackered|dead\\s+on\\s+my\\s+feet|sleepy|drowsy|sleepless|no\\s+sleep|broken\\s+night|up\\s+since|early\\s+start|up\\s+early|early\\s+shift|work\\s+in\\s+the\\s+morning|work\\s+tomorrow|on\\s+early|long\\s+day|long\\s+shift|busy\\s+day\\s+tomorrow|school\\s+run|need\\s+my\\s+sleep|need\\s+an\\s+early\\s+night)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ANOTHER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|they|him|her|them|his|hers|their|theirs|my\\s+(?:son|daughter|boy|girl|kids?|child|children|mum|mom|dad|husband|wife|partner|nan|grandma|grandad|dog|cat|baby|mate|friend|boss)|our\\s+\\w+)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _clauses(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SPLIT(), "split", args))(&__args) }) { let mut part: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "strip", args))(&__args) }); } } py_list(__out) })() };
    Py::None
}
pub fn _is_sign_off(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a clause is only a sign-off word or two, with who it is said to.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__SIGN_OFF_VOCATIVE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__SIGN_OFF_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _ends_in_a_sign_off(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut head: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the clause finishes on a sign-off word and nothing of its own before it.");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__END_SIGN_OFF(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&clause.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__END_SIGN_OFF_KEEP(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _is_thanks_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a clause is only a thank-you.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__THANKS_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _reads_as_a_body_complaint(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let _ = py_str("True when a clause is a body said in trouble, in any of the ways people say it.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BODY(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HURTING(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _reads_as_unwell(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let _ = py_str("True when a clause says that someone or their animal is unwell.");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__UNWELL(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_as_a_body_complaint(args))(&__args) } } };
    Py::None
}
pub fn _a_trouble_behind_a_contrast(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let _ = py_str("True when a contrast sets the person's own trouble against the leaving.\n\n    \"im going to bed but i cant switch off\", \"i should go to bed but i keep waking up\", \"got to go\n    but im overwhelmed\" and \"im off but i feel like rubbish\" are about the second half, and that\n    half is the modules' to answer, so none of them is a goodbye. A thank-you, a wish or a\n    sign-off after the contrast is still a goodbye (\"im off but nice talking to you\"), and a\n    reason for going (\"got to go but my kids are up\", \"im going to bed but im so tired\") is the\n    leave's own and stays one.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CONTRAST(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    tail = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CONTRAST(), "split", args))(&__args) }, &Py::Int(1)), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&tail.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_thanks_clause(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_sign_off(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_wish_clause(args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_their_own_trouble(args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COMES_BACK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__MIND(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _names_their_own_trouble(args: &[Py]) -> Py {
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
    let _ = py_str("True when this part of the line is the person's own trouble and not a reason for going.\n\n    One reading for both shapes, because a \"but\" is only one way of joining the two halves:\n    \"im off but i feel like rubbish\" and \"gotta go, i feel awful\" are the same line with a comma\n    in it, and only the first was read. The sign-off answers the second with \"Go on then - that\n    comes before me. Take care.\", which is about the wrong half.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OVERWHELM(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__FEELS_ROTTEN(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_a_sign_off(args: &[Py]) -> Py {
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
    let _ = py_str("True when a reply lets the person go rather than answering the reason they gave.\n\n    The arbiter asks this of a small-talk candidate on a line that says the person is leaving\n    (`bot/modules/arbiter.py`, `_leaving_reply`): the module that read the line as a goodbye keeps\n    the turn, and one that read the reason clause keeps nothing.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SIGN_OFF(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _is_wish_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this clause is a goodbye said as a wish and nothing else.");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WISH(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(slice_or_raise(&clause.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__WISH_ALLOWED(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _is_leave_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut downs: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this clause is a leave and nothing else.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_wish_clause(args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ends_in_a_sign_off(args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEAVE(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AWAY_OUT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AWAY_PLAN(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BED_LEAVE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NEXT_DAY(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEAVE(), "sub", args))(&__args) };
    downs = op_bitor(&op_bitor(&op_bitor(&g__ALLOWED(), &g__SIGN_OFF_WORDS()), &g__LEAVE_SHORTHAND()), &g__SIGN_OFF_VOCATIVE());
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&downs.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SIGN_OFF(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    trimmed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_LEAVE_TIME(), "sub", args))(&__args) };
    return { let __b = Py::Bool(!py_eq(&trimmed.clone(), &rest.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(trimmed.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&downs.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn reads_as_leaving(args: &[Py]) -> Py {
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
    let _ = py_str("True when the person is saying goodbye, with or without a reason around it.\n\n    The leave has to be a whole clause: the first clause with the reason after it\n    (\"ill let you go, dinner is burning\"), the last clause with the reason before it (\"the\n    kids are up, got to go\"), or the whole line. A question keeps its answer, and a leave\n    anyone is being talked out of (\"dont go\") is not a leave.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_as_leaving(args))(&__args) };
    Py::None
}
pub fn reads_as_a_leave_with_a_body_in_it(args: &[Py]) -> Py {
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
    let _ = py_str("True when a goodbye stops being one only because the reason names a body in trouble.\n\n    \"right, im off to bed, my tooth is agony\" is a goodbye whose reason is a symptom, so the\n    body is the turn and this module stands out of the way (see `reads_as_leaving`). That is\n    right while something answers the body. When nothing does, the person is left with their\n    own line said back to them, which is worse than the goodbye: the fallback asks this before\n    it files the line, and says the goodbye instead.\n    ");
    return { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_leaving(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); __kws.extend(vec![("unwell_blocks".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_as_leaving(args))(&__args) } } };
    Py::None
}
pub fn reads_as_a_leave_with_a_trouble_in_it(args: &[Py]) -> Py {
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
    let _ = py_str("True when a goodbye stops being one only because a contrast sets a trouble against it.\n\n    \"im off but i feel rough\" is not a goodbye while something answers the feeling, and the\n    reading stands down for it (see `_a_trouble_behind_a_contrast`). The turn then belongs to the\n    modules that answer a trouble - but a tail nobody answers leaves the person with the fallback's\n    intake form (\"Got it. Is there anything you'd like a hand with?\"), which is a worse reply than\n    letting them go. This is the same shape as `reads_as_a_leave_with_a_body_in_it`: the fallback\n    asks it before it files the line, and says the goodbye when nothing else would.\n    ");
    return { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_leaving(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); __kws.extend(vec![("trouble_blocks".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_as_leaving(args))(&__args) } } };
    Py::None
}
pub fn _reads_as_leaving(args: &[Py]) -> Py {
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
    let mut unwell_blocks: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unwell_blocks") { v.clone() }
    else { Py::Bool(true) };
    let mut trouble_blocks: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "trouble_blocks") { v.clone() }
    else { Py::Bool(true) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut first: Py = Py::None;
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("`reads_as_leaving`, with the body reason and the trouble behind a contrast blockers or not.");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OPENS_QUESTION(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__KEEPS(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = trouble_blocks.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_trouble_behind_a_contrast(args))(&__args) } } }) {
    return Py::Bool(false);
    }
    clauses = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clauses(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clauses.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clauses.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&clauses.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut clause: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_something(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&clauses.clone(), &Py::Int(0)), geti(&clauses.clone(), &op_neg(&Py::Int(1)))]), 2, None);
    first = __unpk1[0].clone();
    last = __unpk1[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) }) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&clauses.clone(), &Py::Int(1), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let __b = unwell_blocks.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_as_unwell(args))(&__args) } } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = trouble_blocks.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_their_own_trouble(args))(&__args) } } })) } } } };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) }) {
    return Py::Bool(true);
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&clauses.clone(), &Py::Int(1), &op_neg(&Py::Int(1)), &Py::None)); __kws.extend(vec![("start".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    clause = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) }))) {
    continue;
    }
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&clauses.clone(), &Py::None, &index.clone(), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&clauses.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = unwell_blocks.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_as_unwell(args))(&__args) } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_filler(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_thanks_clause(args))(&__args) } } })) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_something(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let _ = py_str("True when a clause is itself a question, with or without the question mark.\n\n    Read on its own clause and not on the whole line, because the leave and the ask are usually\n    two clauses: the wish is the first and the question is the second, and the question is what\n    the person wants answered.\n    ");
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clause.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&clause.clone(), &py_str("?")))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS_OPENER(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS_ANYWHERE(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _is_filler(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a clause is only the run-up to a goodbye: \"ok\", \"right then\", \"ok thanks\".\n\n    A thank-you is the commonest run-up there is, with the little words it carries: \"ok thanks\n    for all your help, i need to go, the bus is here\" is a goodbye and not a thanks.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__RUN_UP(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _thread_is_an_emergency(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last thing said was an emergency reply, not an ordinary one.\n\n    The safety module records the case it answered (`safety_case`) and the runtime records which\n    module answered each turn, so this reads the two facts rather than guessing at words. A minor\n    injury is answered by the same module and records no case, so a cut finger keeps the ordinary\n    wish back: it is a wish, not a crisis with a plaster on it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("safety_case")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }, &py_str("safety"))) } };
    Py::None
}
pub fn _thread_is_unwell(args: &[Py]) -> Py {
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
    let _ = py_str("True when the last thing the person said was that they (or their animal) are unwell.\n\n    Read off the memory rather than the line, because the wish is the second turn: \"i feel awful\"\n    then \"get well soon\" is a wish for the person, and the bot answering with its own health is\n    the wrong half of the exchange.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_as_unwell(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__UNWELL_FEELING(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _wish_pool(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("The wish-back lines for a goodbye said as a wish, or the leaving pool's own.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thread_is_an_emergency(args))(&__args) }) {
    return g_CRISIS_POOL();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thread_is_unwell(args))(&__args) }) {
    return g_KINDER_POOL();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WELL(), "search", args))(&__args) }) {
    return g_WELL_POOL();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NIGHT(), "search", args))(&__args) }) {
    return g_NIGHT_POOL();
    }
    return g_WISH_POOL();
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut unwell_blocks: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unwell_blocks") { v.clone() }
    else { Py::Bool(true) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut trouble_blocks: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "trouble_blocks") { v.clone() }
    else { Py::Bool(true) };
    let mut after: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut going_away: Py = Py::None;
    let mut index: Py = Py::None;
    let mut leave_pool: Py = Py::None;
    let mut leaving_too: Py = Py::None;
    let mut opens_with_a_wish: Py = Py::None;
    let mut own_news: Py = Py::None;
    let mut part: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut wishes: Py = Py::None;
    let _ = py_str("The goodbye lines for this leave, or None when the line is not one.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(words.clone()); __args.push(unwell_blocks.clone()); __args.push(trouble_blocks.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reads_as_leaving(args))(&__args) }))) {
    return Py::None;
    }
    clauses = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clauses(args))(&__args) };
    wishes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut clause: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_wish_clause(args))(&__args) }) {  __out.push(clause.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = wishes.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wishes.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WISH_THING(), "search", args))(&__args) })) } }) {
    wishes = py_list(vec![]);
    }
    leaving_too = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut clause: Py = __item.clone(); __out.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_wish_clause(args))(&__args) })) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    own_news = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut clause: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_wish_clause(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_filler(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_thanks_clause(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_sign_off(args))(&__args) })) } } } } } } } }) {  __out.push(clause.clone()); } } py_list(__out) })() };
    opens_with_a_wish = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clauses.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wishes.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&clauses.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_wish_clause(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&clauses.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WISH_THING(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } } } };
    going_away = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&own_news.clone()) { let mut clause: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AWAY(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = wishes.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&leaving_too.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&going_away.clone())) } } } }) {
    if py_truthy(&{ let __b = own_news.clone(); if !py_truthy(&__b) { __b } else { opens_with_a_wish.clone() } }) {
    return g_OWN_NEWS_POOL();
    }
    if py_truthy(&own_news.clone()) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wishes.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _wish_pool(args))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clauses.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    clause = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) }))) {
    continue;
    }
    leave_pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_a_bedtime(args))(&__args) }) { g_NIGHT_GOODBYE_POOL() } else { g_POOL() };
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&clauses.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    rest = slice_or_raise(&clauses.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut part: Py = __item.clone(); __out.push(Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_thanks_clause(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_sign_off(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_wish_clause(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) } } } } } } }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DAY_AHEAD(), "search", args))(&__args) }) {
    return leave_pool.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_state_not_an_errand(args))(&__args) }) {
    return leave_pool.clone();
    }
    return g_REASON_POOL();
    }
    return leave_pool.clone();
    }
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clauses.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_a_bedtime(args))(&__args) }) { g_NIGHT_GOODBYE_POOL() } else { g_POOL() };
    Py::None
}
pub fn _a_state_not_an_errand(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clauses: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clauses"))) };
    let mut clause: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the reasons after the leave are the person's own state or their own day.\n\n    \"goodnight, im shattered\", \"im off to bed, im worn out\", \"night, im up early\", \"im off, ive\n    got work in the morning\" are the leave with the state behind it, not a job waiting: the\n    reason pool answered them \"Of course. Go and see to it, and mind yourself.\" and there is\n    nothing to see to.\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut clause: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_thanks_clause(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_sign_off(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_wish_clause(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_leave_clause(args))(&__args) } } } } } } }))) {  __out.push(clause.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&said.clone());
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OWN_STATE(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ANOTHER(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _is_a_bedtime(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let _ = py_str("True when the leave itself is a bedtime: the bed or the night is what they are going to.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clause.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_A_BEDTIME(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NITE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn lines(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let _ = py_str("Every goodbye this module can say, for the arbiter's \"is this the sign-off\" test.");
    return op_bitor(&op_bitor(&op_bitor(&op_bitor(&op_bitor(&op_bitor(&op_bitor(&op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NIGHT_GOODBYE_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_REASON_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WISH_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WELL_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_KINDER_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NIGHT_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CRISIS_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_OWN_NEWS_POOL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.leaving".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CRISIS_POOL" => Some(g_CRISIS_POOL()),
            "KINDER_POOL" => Some(g_KINDER_POOL()),
            "LEAVING" => Some(g_LEAVING()),
            "NIGHT_GOODBYE_POOL" => Some(g_NIGHT_GOODBYE_POOL()),
            "NIGHT_POOL" => Some(g_NIGHT_POOL()),
            "OWN_NEWS_POOL" => Some(g_OWN_NEWS_POOL()),
            "POOL" => Some(g_POOL()),
            "REASON_POOL" => Some(g_REASON_POOL()),
            "WELL_POOL" => Some(g_WELL_POOL()),
            "WISH_POOL" => Some(g_WISH_POOL()),
            "_ADVERB" => Some(g__ADVERB()),
            "_ALLOWED" => Some(g__ALLOWED()),
            "_ANOTHER" => Some(g__ANOTHER()),
            "_ASKS" => Some(g__ASKS()),
            "_ASKS_ANYWHERE" => Some(g__ASKS_ANYWHERE()),
            "_ASKS_OPENER" => Some(g__ASKS_OPENER()),
            "_AWAY" => Some(g__AWAY()),
            "_AWAY_OUT" => Some(g__AWAY_OUT()),
            "_AWAY_PLAN" => Some(g__AWAY_PLAN()),
            "_A_DAY_AHEAD" => Some(g__A_DAY_AHEAD()),
            "_A_LEAVE_TIME" => Some(g__A_LEAVE_TIME()),
            "_A_SIGN_OFF" => Some(g__A_SIGN_OFF()),
            "_BED_LEAVE" => Some(g__BED_LEAVE()),
            "_BODY" => Some(g__BODY()),
            "_COMES_BACK" => Some(g__COMES_BACK()),
            "_CONTRAST" => Some(g__CONTRAST()),
            "_END_SIGN_OFF" => Some(g__END_SIGN_OFF()),
            "_END_SIGN_OFF_KEEP" => Some(g__END_SIGN_OFF_KEEP()),
            "_FEELS_ROTTEN" => Some(g__FEELS_ROTTEN()),
            "_GOING_VERB" => Some(g__GOING_VERB()),
            "_HURTING" => Some(g__HURTING()),
            "_KEEPS" => Some(g__KEEPS()),
            "_LEAVE" => Some(g__LEAVE()),
            "_LEAVE_SHORTHAND" => Some(g__LEAVE_SHORTHAND()),
            "_MIND" => Some(g__MIND()),
            "_NEXT_DAY" => Some(g__NEXT_DAY()),
            "_NIGHT" => Some(g__NIGHT()),
            "_NITE" => Some(g__NITE()),
            "_NOT_A_BEDTIME" => Some(g__NOT_A_BEDTIME()),
            "_OPENS_QUESTION" => Some(g__OPENS_QUESTION()),
            "_OVERWHELM" => Some(g__OVERWHELM()),
            "_OWN_STATE" => Some(g__OWN_STATE()),
            "_RUN_UP" => Some(g__RUN_UP()),
            "_SIGN_OFF" => Some(g__SIGN_OFF()),
            "_SIGN_OFF_VOCATIVE" => Some(g__SIGN_OFF_VOCATIVE()),
            "_SIGN_OFF_WORDS" => Some(g__SIGN_OFF_WORDS()),
            "_SPLIT" => Some(g__SPLIT()),
            "_THANKS_WORDS" => Some(g__THANKS_WORDS()),
            "_UNWELL" => Some(g__UNWELL()),
            "_UNWELL_FEELING" => Some(g__UNWELL_FEELING()),
            "_WELL" => Some(g__WELL()),
            "_WISH" => Some(g__WISH()),
            "_WISH_ALLOWED" => Some(g__WISH_ALLOWED()),
            "_WISH_THING" => Some(g__WISH_THING()),
            "_a_state_not_an_errand" => Some(py_func("_a_state_not_an_errand", |args: &[Py]| _a_state_not_an_errand(args))),
            "_a_trouble_behind_a_contrast" => Some(py_func("_a_trouble_behind_a_contrast", |args: &[Py]| _a_trouble_behind_a_contrast(args))),
            "_asks_something" => Some(py_func("_asks_something", |args: &[Py]| _asks_something(args))),
            "_clauses" => Some(py_func("_clauses", |args: &[Py]| _clauses(args))),
            "_ends_in_a_sign_off" => Some(py_func("_ends_in_a_sign_off", |args: &[Py]| _ends_in_a_sign_off(args))),
            "_is_a_bedtime" => Some(py_func("_is_a_bedtime", |args: &[Py]| _is_a_bedtime(args))),
            "_is_filler" => Some(py_func("_is_filler", |args: &[Py]| _is_filler(args))),
            "_is_leave_clause" => Some(py_func("_is_leave_clause", |args: &[Py]| _is_leave_clause(args))),
            "_is_sign_off" => Some(py_func("_is_sign_off", |args: &[Py]| _is_sign_off(args))),
            "_is_thanks_clause" => Some(py_func("_is_thanks_clause", |args: &[Py]| _is_thanks_clause(args))),
            "_is_wish_clause" => Some(py_func("_is_wish_clause", |args: &[Py]| _is_wish_clause(args))),
            "_names_their_own_trouble" => Some(py_func("_names_their_own_trouble", |args: &[Py]| _names_their_own_trouble(args))),
            "_reads_as_a_body_complaint" => Some(py_func("_reads_as_a_body_complaint", |args: &[Py]| _reads_as_a_body_complaint(args))),
            "_reads_as_leaving" => Some(py_func("_reads_as_leaving", |args: &[Py]| _reads_as_leaving(args))),
            "_reads_as_unwell" => Some(py_func("_reads_as_unwell", |args: &[Py]| _reads_as_unwell(args))),
            "_thread_is_an_emergency" => Some(py_func("_thread_is_an_emergency", |args: &[Py]| _thread_is_an_emergency(args))),
            "_thread_is_unwell" => Some(py_func("_thread_is_unwell", |args: &[Py]| _thread_is_unwell(args))),
            "_wish_pool" => Some(py_func("_wish_pool", |args: &[Py]| _wish_pool(args))),
            "lines" => Some(py_func("lines", |args: &[Py]| lines(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "reads_as_a_leave_with_a_body_in_it" => Some(py_func("reads_as_a_leave_with_a_body_in_it", |args: &[Py]| reads_as_a_leave_with_a_body_in_it(args))),
            "reads_as_a_leave_with_a_trouble_in_it" => Some(py_func("reads_as_a_leave_with_a_trouble_in_it", |args: &[Py]| reads_as_a_leave_with_a_trouble_in_it(args))),
            "reads_as_a_sign_off" => Some(py_func("reads_as_a_sign_off", |args: &[Py]| reads_as_a_sign_off(args))),
            "reads_as_leaving" => Some(py_func("reads_as_leaving", |args: &[Py]| reads_as_leaving(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CRISIS_POOL" => s_CRISIS_POOL(v),
            "KINDER_POOL" => s_KINDER_POOL(v),
            "LEAVING" => s_LEAVING(v),
            "NIGHT_GOODBYE_POOL" => s_NIGHT_GOODBYE_POOL(v),
            "NIGHT_POOL" => s_NIGHT_POOL(v),
            "OWN_NEWS_POOL" => s_OWN_NEWS_POOL(v),
            "POOL" => s_POOL(v),
            "REASON_POOL" => s_REASON_POOL(v),
            "WELL_POOL" => s_WELL_POOL(v),
            "WISH_POOL" => s_WISH_POOL(v),
            "_ADVERB" => s__ADVERB(v),
            "_ALLOWED" => s__ALLOWED(v),
            "_ANOTHER" => s__ANOTHER(v),
            "_ASKS" => s__ASKS(v),
            "_ASKS_ANYWHERE" => s__ASKS_ANYWHERE(v),
            "_ASKS_OPENER" => s__ASKS_OPENER(v),
            "_AWAY" => s__AWAY(v),
            "_AWAY_OUT" => s__AWAY_OUT(v),
            "_AWAY_PLAN" => s__AWAY_PLAN(v),
            "_A_DAY_AHEAD" => s__A_DAY_AHEAD(v),
            "_A_LEAVE_TIME" => s__A_LEAVE_TIME(v),
            "_A_SIGN_OFF" => s__A_SIGN_OFF(v),
            "_BED_LEAVE" => s__BED_LEAVE(v),
            "_BODY" => s__BODY(v),
            "_COMES_BACK" => s__COMES_BACK(v),
            "_CONTRAST" => s__CONTRAST(v),
            "_END_SIGN_OFF" => s__END_SIGN_OFF(v),
            "_END_SIGN_OFF_KEEP" => s__END_SIGN_OFF_KEEP(v),
            "_FEELS_ROTTEN" => s__FEELS_ROTTEN(v),
            "_GOING_VERB" => s__GOING_VERB(v),
            "_HURTING" => s__HURTING(v),
            "_KEEPS" => s__KEEPS(v),
            "_LEAVE" => s__LEAVE(v),
            "_LEAVE_SHORTHAND" => s__LEAVE_SHORTHAND(v),
            "_MIND" => s__MIND(v),
            "_NEXT_DAY" => s__NEXT_DAY(v),
            "_NIGHT" => s__NIGHT(v),
            "_NITE" => s__NITE(v),
            "_NOT_A_BEDTIME" => s__NOT_A_BEDTIME(v),
            "_OPENS_QUESTION" => s__OPENS_QUESTION(v),
            "_OVERWHELM" => s__OVERWHELM(v),
            "_OWN_STATE" => s__OWN_STATE(v),
            "_RUN_UP" => s__RUN_UP(v),
            "_SIGN_OFF" => s__SIGN_OFF(v),
            "_SIGN_OFF_VOCATIVE" => s__SIGN_OFF_VOCATIVE(v),
            "_SIGN_OFF_WORDS" => s__SIGN_OFF_WORDS(v),
            "_SPLIT" => s__SPLIT(v),
            "_THANKS_WORDS" => s__THANKS_WORDS(v),
            "_UNWELL" => s__UNWELL(v),
            "_UNWELL_FEELING" => s__UNWELL_FEELING(v),
            "_WELL" => s__WELL(v),
            "_WISH" => s__WISH(v),
            "_WISH_ALLOWED" => s__WISH_ALLOWED(v),
            "_WISH_THING" => s__WISH_THING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.leaving", module_obj);
    pyrt::xrefs::reg_fn("compose.leaving", "_clauses", py_func("_clauses", |args: &[Py]| _clauses(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_is_sign_off", py_func("_is_sign_off", |args: &[Py]| _is_sign_off(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_ends_in_a_sign_off", py_func("_ends_in_a_sign_off", |args: &[Py]| _ends_in_a_sign_off(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_is_thanks_clause", py_func("_is_thanks_clause", |args: &[Py]| _is_thanks_clause(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_reads_as_a_body_complaint", py_func("_reads_as_a_body_complaint", |args: &[Py]| _reads_as_a_body_complaint(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_reads_as_unwell", py_func("_reads_as_unwell", |args: &[Py]| _reads_as_unwell(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_a_trouble_behind_a_contrast", py_func("_a_trouble_behind_a_contrast", |args: &[Py]| _a_trouble_behind_a_contrast(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_names_their_own_trouble", py_func("_names_their_own_trouble", |args: &[Py]| _names_their_own_trouble(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "reads_as_a_sign_off", py_func("reads_as_a_sign_off", |args: &[Py]| reads_as_a_sign_off(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_is_wish_clause", py_func("_is_wish_clause", |args: &[Py]| _is_wish_clause(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_is_leave_clause", py_func("_is_leave_clause", |args: &[Py]| _is_leave_clause(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "reads_as_leaving", py_func("reads_as_leaving", |args: &[Py]| reads_as_leaving(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "reads_as_a_leave_with_a_body_in_it", py_func("reads_as_a_leave_with_a_body_in_it", |args: &[Py]| reads_as_a_leave_with_a_body_in_it(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "reads_as_a_leave_with_a_trouble_in_it", py_func("reads_as_a_leave_with_a_trouble_in_it", |args: &[Py]| reads_as_a_leave_with_a_trouble_in_it(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_reads_as_leaving", py_func("_reads_as_leaving", |args: &[Py]| _reads_as_leaving(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_asks_something", py_func("_asks_something", |args: &[Py]| _asks_something(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_is_filler", py_func("_is_filler", |args: &[Py]| _is_filler(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_thread_is_an_emergency", py_func("_thread_is_an_emergency", |args: &[Py]| _thread_is_an_emergency(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_thread_is_unwell", py_func("_thread_is_unwell", |args: &[Py]| _thread_is_unwell(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_wish_pool", py_func("_wish_pool", |args: &[Py]| _wish_pool(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "pool", py_func("pool", |args: &[Py]| pool(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_a_state_not_an_errand", py_func("_a_state_not_an_errand", |args: &[Py]| _a_state_not_an_errand(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "_is_a_bedtime", py_func("_is_a_bedtime", |args: &[Py]| _is_a_bedtime(args)));
    pyrt::xrefs::reg_fn("compose.leaving", "lines", py_func("lines", |args: &[Py]| lines(args)));
    pyrt::xrefs::reg_access("compose.leaving", "CRISIS_POOL", g_CRISIS_POOL, s_CRISIS_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "KINDER_POOL", g_KINDER_POOL, s_KINDER_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "LEAVING", g_LEAVING, s_LEAVING);
    pyrt::xrefs::reg_access("compose.leaving", "NIGHT_GOODBYE_POOL", g_NIGHT_GOODBYE_POOL, s_NIGHT_GOODBYE_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "NIGHT_POOL", g_NIGHT_POOL, s_NIGHT_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "OWN_NEWS_POOL", g_OWN_NEWS_POOL, s_OWN_NEWS_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "POOL", g_POOL, s_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "REASON_POOL", g_REASON_POOL, s_REASON_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "WELL_POOL", g_WELL_POOL, s_WELL_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "WISH_POOL", g_WISH_POOL, s_WISH_POOL);
    pyrt::xrefs::reg_access("compose.leaving", "_ADVERB", g__ADVERB, s__ADVERB);
    pyrt::xrefs::reg_access("compose.leaving", "_ALLOWED", g__ALLOWED, s__ALLOWED);
    pyrt::xrefs::reg_access("compose.leaving", "_ANOTHER", g__ANOTHER, s__ANOTHER);
    pyrt::xrefs::reg_access("compose.leaving", "_ASKS", g__ASKS, s__ASKS);
    pyrt::xrefs::reg_access("compose.leaving", "_ASKS_ANYWHERE", g__ASKS_ANYWHERE, s__ASKS_ANYWHERE);
    pyrt::xrefs::reg_access("compose.leaving", "_ASKS_OPENER", g__ASKS_OPENER, s__ASKS_OPENER);
    pyrt::xrefs::reg_access("compose.leaving", "_AWAY", g__AWAY, s__AWAY);
    pyrt::xrefs::reg_access("compose.leaving", "_AWAY_OUT", g__AWAY_OUT, s__AWAY_OUT);
    pyrt::xrefs::reg_access("compose.leaving", "_AWAY_PLAN", g__AWAY_PLAN, s__AWAY_PLAN);
    pyrt::xrefs::reg_access("compose.leaving", "_A_DAY_AHEAD", g__A_DAY_AHEAD, s__A_DAY_AHEAD);
    pyrt::xrefs::reg_access("compose.leaving", "_A_LEAVE_TIME", g__A_LEAVE_TIME, s__A_LEAVE_TIME);
    pyrt::xrefs::reg_access("compose.leaving", "_A_SIGN_OFF", g__A_SIGN_OFF, s__A_SIGN_OFF);
    pyrt::xrefs::reg_access("compose.leaving", "_BED_LEAVE", g__BED_LEAVE, s__BED_LEAVE);
    pyrt::xrefs::reg_access("compose.leaving", "_BODY", g__BODY, s__BODY);
    pyrt::xrefs::reg_access("compose.leaving", "_COMES_BACK", g__COMES_BACK, s__COMES_BACK);
    pyrt::xrefs::reg_access("compose.leaving", "_CONTRAST", g__CONTRAST, s__CONTRAST);
    pyrt::xrefs::reg_access("compose.leaving", "_END_SIGN_OFF", g__END_SIGN_OFF, s__END_SIGN_OFF);
    pyrt::xrefs::reg_access("compose.leaving", "_END_SIGN_OFF_KEEP", g__END_SIGN_OFF_KEEP, s__END_SIGN_OFF_KEEP);
    pyrt::xrefs::reg_access("compose.leaving", "_FEELS_ROTTEN", g__FEELS_ROTTEN, s__FEELS_ROTTEN);
    pyrt::xrefs::reg_access("compose.leaving", "_GOING_VERB", g__GOING_VERB, s__GOING_VERB);
    pyrt::xrefs::reg_access("compose.leaving", "_HURTING", g__HURTING, s__HURTING);
    pyrt::xrefs::reg_access("compose.leaving", "_KEEPS", g__KEEPS, s__KEEPS);
    pyrt::xrefs::reg_access("compose.leaving", "_LEAVE", g__LEAVE, s__LEAVE);
    pyrt::xrefs::reg_access("compose.leaving", "_LEAVE_SHORTHAND", g__LEAVE_SHORTHAND, s__LEAVE_SHORTHAND);
    pyrt::xrefs::reg_access("compose.leaving", "_MIND", g__MIND, s__MIND);
    pyrt::xrefs::reg_access("compose.leaving", "_NEXT_DAY", g__NEXT_DAY, s__NEXT_DAY);
    pyrt::xrefs::reg_access("compose.leaving", "_NIGHT", g__NIGHT, s__NIGHT);
    pyrt::xrefs::reg_access("compose.leaving", "_NITE", g__NITE, s__NITE);
    pyrt::xrefs::reg_access("compose.leaving", "_NOT_A_BEDTIME", g__NOT_A_BEDTIME, s__NOT_A_BEDTIME);
    pyrt::xrefs::reg_access("compose.leaving", "_OPENS_QUESTION", g__OPENS_QUESTION, s__OPENS_QUESTION);
    pyrt::xrefs::reg_access("compose.leaving", "_OVERWHELM", g__OVERWHELM, s__OVERWHELM);
    pyrt::xrefs::reg_access("compose.leaving", "_OWN_STATE", g__OWN_STATE, s__OWN_STATE);
    pyrt::xrefs::reg_access("compose.leaving", "_RUN_UP", g__RUN_UP, s__RUN_UP);
    pyrt::xrefs::reg_access("compose.leaving", "_SIGN_OFF", g__SIGN_OFF, s__SIGN_OFF);
    pyrt::xrefs::reg_access("compose.leaving", "_SIGN_OFF_VOCATIVE", g__SIGN_OFF_VOCATIVE, s__SIGN_OFF_VOCATIVE);
    pyrt::xrefs::reg_access("compose.leaving", "_SIGN_OFF_WORDS", g__SIGN_OFF_WORDS, s__SIGN_OFF_WORDS);
    pyrt::xrefs::reg_access("compose.leaving", "_SPLIT", g__SPLIT, s__SPLIT);
    pyrt::xrefs::reg_access("compose.leaving", "_THANKS_WORDS", g__THANKS_WORDS, s__THANKS_WORDS);
    pyrt::xrefs::reg_access("compose.leaving", "_UNWELL", g__UNWELL, s__UNWELL);
    pyrt::xrefs::reg_access("compose.leaving", "_UNWELL_FEELING", g__UNWELL_FEELING, s__UNWELL_FEELING);
    pyrt::xrefs::reg_access("compose.leaving", "_WELL", g__WELL, s__WELL);
    pyrt::xrefs::reg_access("compose.leaving", "_WISH", g__WISH, s__WISH);
    pyrt::xrefs::reg_access("compose.leaving", "_WISH_ALLOWED", g__WISH_ALLOWED, s__WISH_ALLOWED);
    pyrt::xrefs::reg_access("compose.leaving", "_WISH_THING", g__WISH_THING, s__WISH_THING);
}
