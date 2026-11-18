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


thread_local! { static G_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVICE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPTIONS_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAPERWORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAPERWORK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PICK_LINES_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REASON_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VAGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VAGUE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YESNO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YESNO_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ASK_AT_THE_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ENOUGH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DATE_ON_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DECISION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FACT_ABOUT_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIT_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FREQUENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PICK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_POINTS_AT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STORED_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STRAIGHT_YESNO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THEY_SAID_SO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TIME_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VAGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VERDICT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WHAT_WAS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WHOSE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WORK_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEANS_ON_THE_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_AN_OPTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ON_PAPER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THEIR_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHAT_TO_DO_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHICH_I_NEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ADVICE() -> Py { ensure_init(); G_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE(v: Py) { ensure_init(); G_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVICE_LINES() -> Py { ensure_init(); G_ADVICE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE_LINES(v: Py) { ensure_init(); G_ADVICE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT() -> Py { ensure_init(); G_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT(v: Py) { ensure_init(); G_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_LINES() -> Py { ensure_init(); G_AMOUNT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_LINES(v: Py) { ensure_init(); G_AMOUNT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OPTIONS() -> Py { ensure_init(); G_OPTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPTIONS(v: Py) { ensure_init(); G_OPTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPTIONS_LINES() -> Py { ensure_init(); G_OPTIONS_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPTIONS_LINES(v: Py) { ensure_init(); G_OPTIONS_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PAPERWORK() -> Py { ensure_init(); G_PAPERWORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAPERWORK(v: Py) { ensure_init(); G_PAPERWORK.with(|c| *c.borrow_mut() = v); }
pub fn g_PAPERWORK_LINES() -> Py { ensure_init(); G_PAPERWORK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAPERWORK_LINES(v: Py) { ensure_init(); G_PAPERWORK_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK() -> Py { ensure_init(); G_PICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK(v: Py) { ensure_init(); G_PICK.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_LINES() -> Py { ensure_init(); G_PICK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_LINES(v: Py) { ensure_init(); G_PICK_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PICK_LINES_NAMED() -> Py { ensure_init(); G_PICK_LINES_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PICK_LINES_NAMED(v: Py) { ensure_init(); G_PICK_LINES_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_REASON() -> Py { ensure_init(); G_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REASON(v: Py) { ensure_init(); G_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_REASON_LINES() -> Py { ensure_init(); G_REASON_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REASON_LINES(v: Py) { ensure_init(); G_REASON_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL() -> Py { ensure_init(); G_RECALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL(v: Py) { ensure_init(); G_RECALL.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_LINES() -> Py { ensure_init(); G_RECALL_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_LINES(v: Py) { ensure_init(); G_RECALL_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_STEPS() -> Py { ensure_init(); G_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEPS(v: Py) { ensure_init(); G_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME() -> Py { ensure_init(); G_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME(v: Py) { ensure_init(); G_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_LINES() -> Py { ensure_init(); G_TIME_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_LINES(v: Py) { ensure_init(); G_TIME_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_VAGUE() -> Py { ensure_init(); G_VAGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VAGUE(v: Py) { ensure_init(); G_VAGUE.with(|c| *c.borrow_mut() = v); }
pub fn g_VAGUE_LINES() -> Py { ensure_init(); G_VAGUE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VAGUE_LINES(v: Py) { ensure_init(); G_VAGUE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_YESNO() -> Py { ensure_init(); G_YESNO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YESNO(v: Py) { ensure_init(); G_YESNO.with(|c| *c.borrow_mut() = v); }
pub fn g_YESNO_LINES() -> Py { ensure_init(); G_YESNO_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YESNO_LINES(v: Py) { ensure_init(); G_YESNO_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ASK_AT_THE_FRONT() -> Py { ensure_init(); G__AN_ASK_AT_THE_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ASK_AT_THE_FRONT(v: Py) { ensure_init(); G__AN_ASK_AT_THE_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ENOUGH_ASK() -> Py { ensure_init(); G__AN_ENOUGH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ENOUGH_ASK(v: Py) { ensure_init(); G__AN_ENOUGH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DATE_ON_IT() -> Py { ensure_init(); G__A_DATE_ON_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DATE_ON_IT(v: Py) { ensure_init(); G__A_DATE_ON_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DECISION() -> Py { ensure_init(); G__A_DECISION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DECISION(v: Py) { ensure_init(); G__A_DECISION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FACT_ABOUT_TIME() -> Py { ensure_init(); G__A_FACT_ABOUT_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FACT_ABOUT_TIME(v: Py) { ensure_init(); G__A_FACT_ABOUT_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIT_STATE() -> Py { ensure_init(); G__A_FIT_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIT_STATE(v: Py) { ensure_init(); G__A_FIT_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FREQUENCY() -> Py { ensure_init(); G__A_FREQUENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FREQUENCY(v: Py) { ensure_init(); G__A_FREQUENCY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HEADS() -> Py { ensure_init(); G__A_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HEADS(v: Py) { ensure_init(); G__A_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PICK_ASK() -> Py { ensure_init(); G__A_PICK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PICK_ASK(v: Py) { ensure_init(); G__A_PICK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_POINTS_AT_A_THING() -> Py { ensure_init(); G__A_POINTS_AT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_POINTS_AT_A_THING(v: Py) { ensure_init(); G__A_POINTS_AT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STORED_COUNT() -> Py { ensure_init(); G__A_STORED_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STORED_COUNT(v: Py) { ensure_init(); G__A_STORED_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STRAIGHT_YESNO() -> Py { ensure_init(); G__A_STRAIGHT_YESNO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STRAIGHT_YESNO(v: Py) { ensure_init(); G__A_STRAIGHT_YESNO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THEY_SAID_SO() -> Py { ensure_init(); G__A_THEY_SAID_SO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THEY_SAID_SO(v: Py) { ensure_init(); G__A_THEY_SAID_SO.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME_WORD() -> Py { ensure_init(); G__A_TIME_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME_WORD(v: Py) { ensure_init(); G__A_TIME_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VAGUE() -> Py { ensure_init(); G__A_VAGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VAGUE(v: Py) { ensure_init(); G__A_VAGUE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VERDICT_WORD() -> Py { ensure_init(); G__A_VERDICT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VERDICT_WORD(v: Py) { ensure_init(); G__A_VERDICT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WHAT_WAS_IT() -> Py { ensure_init(); G__A_WHAT_WAS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WHAT_WAS_IT(v: Py) { ensure_init(); G__A_WHAT_WAS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WHOSE_THING() -> Py { ensure_init(); G__A_WHOSE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WHOSE_THING(v: Py) { ensure_init(); G__A_WHOSE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WORK_OUT() -> Py { ensure_init(); G__A_WORK_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WORK_OUT(v: Py) { ensure_init(); G__A_WORK_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_ASK() -> Py { ensure_init(); G__BARE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_ASK(v: Py) { ensure_init(); G__BARE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAVY() -> Py { ensure_init(); G__HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAVY(v: Py) { ensure_init(); G__HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_MANY() -> Py { ensure_init(); G__HOW_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_MANY(v: Py) { ensure_init(); G__HOW_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g__LEANS_ON_THE_THREAD() -> Py { ensure_init(); G__LEANS_ON_THE_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEANS_ON_THE_THREAD(v: Py) { ensure_init(); G__LEANS_ON_THE_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g__LINES() -> Py { ensure_init(); G__LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LINES(v: Py) { ensure_init(); G__LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_AN_OPTION() -> Py { ensure_init(); G__NOT_AN_OPTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_AN_OPTION(v: Py) { ensure_init(); G__NOT_AN_OPTION.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_THING() -> Py { ensure_init(); G__NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_THING(v: Py) { ensure_init(); G__NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__ON_PAPER() -> Py { ensure_init(); G__ON_PAPER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ON_PAPER(v: Py) { ensure_init(); G__ON_PAPER.with(|c| *c.borrow_mut() = v); }
pub fn g__THEIR_THINGS() -> Py { ensure_init(); G__THEIR_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THEIR_THINGS(v: Py) { ensure_init(); G__THEIR_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_THINGS() -> Py { ensure_init(); G__TWO_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_THINGS(v: Py) { ensure_init(); G__TWO_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__WHAT_TO_DO_WITH() -> Py { ensure_init(); G__WHAT_TO_DO_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHAT_TO_DO_WITH(v: Py) { ensure_init(); G__WHAT_TO_DO_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g__WHICH_I_NEED() -> Py { ensure_init(); G__WHICH_I_NEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHICH_I_NEED(v: Py) { ensure_init(); G__WHICH_I_NEED.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The shape of the answer, when the bot has not got the fact.\n\nEvery line here reached the very end of the composer: no knowledge card claimed it, no route to a\nplace fitted it, and nothing in the thread named who to ring. What the person got was the search\nhedge - \"I'm not sure about that one, sorry, I'd rather not guess. A quick search online will get\nyou a solid answer\" - which is true of every line ever typed and tells nobody anything. Pointing\nelsewhere is not a reply.\n\nThere is one thing the bot still knows about a line like that, and it is the shape of the answer:\na pick wants the thing that decides a pick, a price wants the three things that set a price, \"how\nlong\" wants the range and the thing that moves it, \"why\" wants the causes that are worth ruling\nout. None of that is a fact about the person's kitchen, and none of it needs to be: it is what a\nfriend says while getting to the detail, and it is the difference between \"go and look it up\" and\n\"here is what the answer depends on\".\n\n`fit.wants` has already read the shape and `bot/modules/intent.py` has written it to\n`memory[\"understood\"][\"wants\"]`, so this reads that and nothing else. Every line has to work on\nany subject at all - the subject is the one thing the bot has not got - so no line names the thing\nand no line assumes what kind of thing it is. The last step is always the person's own detail, in\none short question, because that is what turns the shape into an answer.\n\nWhere a line's own words do name the shape of thing (a job, a bill, a wait), the pool says so; the\nword \"it\" is never used for the subject, because a line about a garden and a line about a debt\nwould then read the same, and the bot already has pools for subjects it can name.\n");
    s_PICK(py_str("pick"));
    s_OPTIONS(py_str("options"));
    s_AMOUNT(py_str("amount"));
    s_TIME(py_str("time"));
    s_STEPS(py_str("steps"));
    s_YESNO(py_str("yesno"));
    s_VAGUE(py_str("vague"));
    s_RECALL(py_str("recall"));
    s_PAPERWORK(py_str("paperwork"));
    s_REASON(py_str("reason"));
    s_ADVICE(py_str("advice"));
    s_PICK_LINES(py_tuple(vec![py_str("I can't pick for you, but two questions settle most choices like this: which one you would still choose in a year, and which is cheaper to undo if it turns out wrong. Which way are you leaning?"), py_str("There is no right answer I can point you at. What every pick like this comes down to is what you will use most, and whether the cheaper one is good enough for that. What are you down to choosing between?"), py_str("Two ways to choose: the one that is best at the main thing, or the one that is decent at everything. Most people regret the second. What would it have to be good at?")]));
    s_PICK_LINES_NAMED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PICK_LINES()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&line.clone(), &py_str("choosing between")))) {  __out.push(line.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OPTIONS_LINES(py_tuple(vec![py_str("I have no list to hand you, and I would rather not invent one. What makes a shortlist worth anything is that the choices are actually different: one cheap, one that lasts, one that is least effort. Which of those three are you after?"), py_str("Nothing in front of me to suggest from. Tell me the two or three things that are fixed - where, when, how much - and I can say what sort of option fits them. Which is the one that cannot move?"), py_str("I can't give you a list without making it up. The useful question is what you would say no to: once that is settled there are usually only two or three options left. What is a no?")]));
    s_AMOUNT_LINES(py_tuple(vec![py_str("I haven't got a figure for that, and guessing one would be worse than none. A number like this is worked out from two or three others rather than looked up - give me those and I will do the sums with you. What have you got to go on?"), py_str("That is arithmetic more than a fact. Tell me the size of it, how many there are, and what it is per one, and I will work the total out with the steps shown so you can check them. Which of those do you have?"), py_str("No number from me without the figures it comes from - that is what stops it being a guess. What numbers do you already have?")]));
    s_TIME_LINES(py_tuple(vec![py_str("I don't know the size of the job, so any time I gave you would be a guess. Most things like this are minutes for the small version and a day or more for the big one, and which one you have is the whole question. How big is it?"), py_str("That depends on what somebody finds once they start, which is why nobody will give you a firm number before they look. Ask for the usual time and add half again. Who is doing it?"), py_str("No honest number from me without knowing the size and who does it. If it is the simple version it is quick; if anything has to be taken out first it is not. What has to happen?")]));
    s_YESNO_LINES(py_tuple(vec![py_str("I can't say yes or no to that honestly. The test I would use is the worst that happens if it goes wrong, and how easily you could undo it. If both are small, it is worth a try - how bad is the worst case?"), py_str("That is yours to weigh, and I don't have the details to weigh it for you. It comes down to what you would lose, how likely that is, and whether you could put it back. What would you lose at worst?"), py_str("No verdict from me on this one - I would be guessing. Say what would make you change your mind, and if nothing would, that is your answer already. What would change it?")]));
    s_REASON_LINES(py_tuple(vec![py_str("I don't know this one well enough to say why, and a made-up reason is worse than none. Cases like it usually have two or three causes, and the way to tell which is yours is what else happens at the same moment. What else have you noticed?"), py_str("No cause I could name honestly. The reliable way to find one is to change a single thing at a time and see whether it goes away - two changes at once tells you nothing. What have you tried?"), py_str("I would be guessing at the reason. Whatever it is, it will have a before and after: what was different the first time it happened?")]));
    s_ADVICE_LINES(py_tuple(vec![py_str("I can't tell you what to do here - I haven't got the details, and it is yours to decide. Which is the option you keep coming back to? I will argue the other side properly."), py_str("No advice worth the name from me on this. The question that usually settles it is the one you would regret not asking. Which is it?"), py_str("I don't know enough to say what is right. Put it into one sentence - the choice, and what it costs - and I will help you look at it from the outside. What is the choice?")]));
    s_VAGUE_LINES(py_tuple(vec![py_str("Give me a little more and I'll answer that part properly - what are you after?"), py_str("Happy to keep going - which part do you want next?"), py_str("That one is too open for me to answer well, and a guess would waste your time. Which bit matters most?"), py_str("One more line and I'll have something useful for you. What are you after now?")]));
    s__A_VAGUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|so|ok|okay|right|well|hmm|um)?\\s*,?\\s*(?:now\\s+what|what\\s+now|now)\\b[\\s?!.]*$|^what should i (?:think about|be doing(?: now)?|do next|do first)\\b[\\s?!.]*$|^what do i do now\\b[\\s?!.]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WHAT_TO_DO_WITH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:should|shall|do|would|can|will) (?:i|we) (?:do|use|put)\\b[^?.!]{0,20}?\\b(?:with|about)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_WHAT_WAS_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and\\s+|so\\s+)?(?:what|which)\\s+(?:was|is|were|are)?\\s*(?:the\\s+)?(?:year|one|name|number|date|day|time|age|colour|color|brand|make|model|price|word|thing)\\b|\\bwhat\\b[^?.!]{0,24}?\\b(?:did|do) (?:i|we) (?:say|tell|mention)\\b|\\bremind me\\b|\\bwhat did (?:i|we|he|she|they|you) (?:say|tell|decide|choose|pick)\\b|\\b(?:what|which)\\b[^?.!]{0,30}?\\bagain\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_THEY_SAID_SO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+(?:say|said|say\\s+it|told\\s+you|mention|mentioned)\\b|\\bagain\\b|\\bremind\\s+me\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_FREQUENCY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:many|often)\\b[^?.!]{0,24}?\\b(?:days|nights|times|weeks|months|years)\\b[^?.!]{0,12}?\\b(?:a|per|each)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AN_ASK_AT_THE_FRONT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ok\\s+|okay\\s+|and\\s+|so\\s+|but\\s+|also\\s+)?(?:which|what|should|shall|would|could|can|do|did|is|are|am|i|we|im|i'm|were|we're|cant|can't|help|how|any|got)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WHICH_I_NEED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhich\\b[^?.!]{0,24}?\\b(?:do|would|will|should|shall)\\s+(?:i|we)\\s+(?:need|use|get for|ask for|call)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RECALL_LINES(py_tuple(vec![py_str("That is one for between us rather than a search - say it once more and I will answer from what you tell me. What was it?"), py_str("I would rather have it from you than guess: say it again and I will hold on to it. Which one was it?"), py_str("Say it once more and I will keep it this time, then answer straight from that. What was it?")]));
    s_PAPERWORK_LINES(py_tuple(vec![py_str("That one is on the paperwork, not out where I can see it - the renewal date sits near the policy number on the letter or in your account online. What does it say?"), py_str("The date is in their hands rather than mine: it is on the notice they last sent you, beside the reference number. Have you still got it?"), py_str("I can't see your own documents, so the date is on the letter or in the app under the plan. What date does it give?")]));
    s__LINES(py_dict(vec![(g_PICK(), g_PICK_LINES()), (g_OPTIONS(), g_OPTIONS_LINES()), (g_AMOUNT(), g_AMOUNT_LINES()), (g_TIME(), g_TIME_LINES()), (g_YESNO(), g_YESNO_LINES()), (g_VAGUE(), g_VAGUE_LINES()), (g_RECALL(), g_RECALL_LINES()), (g_PAPERWORK(), g_PAPERWORK_LINES()), (g_REASON(), g_REASON_LINES()), (g_ADVICE(), g_ADVICE_LINES())]));
    s__HEAVY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:died|dying|death|funeral|cancer|hospice|hospital|miscarr\\w*|suicide|kill myself|abuse|assault|overdose|relapse|diagnos\\w*|terminal)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TWO_THINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(\\w[\\w'-]*)\\W{0,2}\\s+or\\s+\\W{0,2}(\\w[\\w'-]*)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_AN_OPTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i we you he she it they me us them him her my our your their this that\nthese those there then so if and but not no yes is are was were do does did can could will would\nshould shall may might have has had get got go going put take make"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HOW_MANY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|but\\s+)?how\\s+(?:many|much)\\s+(?:\\w+\\s+){0,2}?(\\w{3,})")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("more time long often far big much many old new good bad early late\nsoon quick long worth often"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_TIME_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:how\\s+far\\s+ahead|how\\s+soon|how\\s+long|how\\s+quick|when\\s+should|what\\s+time)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_FACT_ABOUT_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhen (?:does|do|did|will) (?:the|it|they|this|that|a|my|his|her|your)\\b|\\bwhen (?:does|do|did|will)\\s+(?!i\\b|we\\b|you\\b)\\w|\\bwhen (?:does|do) it\\b|\\bwhat time (?:does|do|is|are|will)\\b|\\bhow far (?:ahead|in advance)\\b|\\bhow long (?:before|in advance)\\b|\\b(?:should|do|can) i (?:need to )?book\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_STORED_COUNT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow many (?:episodes|seasons|chapter|chapters|pages|tracks|songs|albums|volumes|parts|instalments|films|series|books|tablets|doses|rounds)\\b|\\bhow many \\w+ (?:are|were) there\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_FIT_STATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:ok|okay|alright|all\\s+right|fine|safe|sound|sound\\s+enough|still\\s+good|still\\s+ok|still\\s+fine|gone\\s+off|gone\\s+bad|in\\s+date|off|spoiled|spoilt|edible|contaminated|damaged|broken|worn|leaking)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_POINTS_AT_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|that|this|those|these|they|them|the|mine|ours)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_VERDICT_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:worth|enough|too\\s+many|too\\s+much|better|worse|help|helps|work|works|risk|safe|okay|ok|fine|point|bother|afford|matter|matters)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DECISION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:should\\s+(?:i|we)|shall\\s+(?:i|we)|is\\s+it\\s+worth|worth\\s+it|worth|do\\s+you\\s+think|would\\s+(?:i|we)|am\\s+i\\s+better|enough|too\\s+much|too\\s+many|better\\s+off|bother|afford|for\\s+that|(?:would|will|does|do|did|could|can)\\s+(?:a|an|another|one|two|the)\\b[^?.!]{0,25}?\\b(?:help|be\\s+(?:better|good|worth))\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_WHOSE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|mine|his|her|their)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DATE_ON_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:when|what\\s+date)\\b[^?.!]{0,30}?\\b(?:run\\s+out|runs\\s+out|expire|expires|renew|renews|renewal|end|ends|due|give\\s+notice|notice\\s+period)\\b|\\b(?:when|what\\s+date)\\b[^?.!]{0,24}?\\b(?:do|should)\\s+(?:i|we)\\s+(?:need\\s+to\\s+)?renew\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THEIR_THINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:polic(?:y|ies)|insurance|contract|warranty|guarantee|mot|lease|tenancy|licence|license|passport|membership|subscription|pass|agreement|cover|plan|booking|reservation|permit|filing|return)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AN_ENOUGH_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|will|would|do|does|did)\\b[^?.!]{0,15}?\\benough\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PICK_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ok\\s+|and\\s+|so\\s+)?(?:what|which)\\s+(?:should|shall|do|would|can)\\s+(?:i|we)\\s+(?:go\\s+for|pick|choose|get|buy|start\\s+with|have)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_STRAIGHT_YESNO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ok\\s+|and\\s+|so\\s+)?(?:should|shall|do|does|did|would|will|is|are|can|could)\\s+(?:i|we|it|that|this)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_WORK_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:do|does|would|can|could|shall) (?:i|we)\\b[^?.!]{0,30}?\\b(?:work|figure|sort)\\s+out\\b|\\bhow (?:do|would|can|could) (?:i|we) (?:know|tell|estimate|budget)\\b[^?.!]{0,30}?\\b(?:much|many|cost|costs|afford|price|budget|total)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ON_PAPER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:polic(?:y|ies)|renewal|insurance|contract|agreement|licence|license|passport|warrant(?:y|ies)|booking|ticket|statement|invoice|receipt|letter|form|certificate|account|mortgage|tenancy|lease|quote|registration|mot|v5c)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BARE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ok\\s+|and\\s+|so\\s+)?(?:what|which|where|whats|what's)\\s+(?:should|shall|do|can|could|would)\\s+(?:i|we)\\s+(?:visit|see|do|go|eat|drink|get|buy|put|pick|choose|start|try|book)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LEANS_ON_THE_THREAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|so\\s+|but\\s+|ok\\s+)?(?:does|do|did|is|are|was|were|will|would|can|could|should)\\s+(?:that|this|it|they|those|these)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_HEADS(Py::Int(40));
}
pub fn _the_two_things_are_on_the_table(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut left: Py = Py::None;
    let mut right: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the person or the bot has already said what the choice is between.");
    said = py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    }
    let __iter2 = py_iter_or_raise(&said.clone());
    for __item in __iter2 {
    text = __item.clone();
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TWO_THINGS(), "findall", args))(&__args) });
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    left = __unpk4[0].clone();
    right = __unpk4[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__NOT_AN_OPTION(), &left.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_AN_OPTION(), &right.clone())) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_figures_are_in_the_thread(args: &[Py]) -> Py {
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
    let mut source: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person has already given the numbers, so a reply asking for them is a daft reply.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    for __item in __iter2 {
    source = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d")); __args.push(source.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_bot_just_said_a_lot(args: &[Py]) -> Py {
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
    let mut bot: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The previous answer gave the person something to work with, not just a line back.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    bot = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bot.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(25))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_count_of_the_thing_just_named(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut source: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HOW_MANY(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g__NOT_THE_THING(), &word.clone()))) {
    return Py::Bool(false);
    }
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(4)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    for __item in __iter2 {
    source = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![a-z])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "rstrip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("[a-z]{0,2}(?![a-z])"))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&source.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _on_paper(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when a paper named in the line or the last few turns holds the date or the figure.\n\n    A policy's renewal date, a licence's expiry, a bill's total: the person has the document and\n    the answer is printed on it. Nothing here is a job whose length or cost is being weighed, so\n    a shape pool that asks what the figure is built from is off the subject.\n    ");
    said = py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __aug2 = op_add(&said.clone(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    said = __aug2;
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut text: Py = __item.clone(); if py_truthy(&text.clone()) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ON_PAPER(), "search", args))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_thing_is_theirs(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the thread has a thing of their own in it that a date could belong to.\n\n    \"ok and when does it run out\" carries no noun at all, so the noun has to come from what they\n    said: \"my car insurance is up for renewal in march\". A bare \"when does the season start\" after\n    nothing of theirs stays a world fact.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WHOSE_THING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THEIR_THINGS(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_thread_is_heavy(args: &[Py]) -> Py {
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
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HEAVY(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_shape_without_the_fact(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut _turns: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("The pool for the shape of answer this line wants, or None when it wants none of these.\n\n    The shape is the one the intent module already wrote; a line with no shape of its own is left\n    to the composer, because there is nothing about it the bot could say that would be true.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    want = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wants")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_STEPS())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PICK_ASK(), "match", args))(&__args) } } }) {
    want = g_PICK();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_STEPS())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WORK_OUT(), "search", args))(&__args) } } }) {
    want = g_AMOUNT();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_STEPS())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_STRAIGHT_YESNO(), "match", args))(&__args) } } }) {
    want = g_YESNO();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_AMOUNT())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TIME_WORD(), "search", args))(&__args) } } }) {
    want = g_TIME();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DATE_ON_IT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WHOSE_THING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_is_theirs(args))(&__args) } } } } }) {
    want = g_PAPERWORK();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_AMOUNT(), g_TIME()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FREQUENCY(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_PICK(), g_OPTIONS()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_ASK_AT_THE_FRONT(), "match", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TWO_THINGS(), "search", args))(&__args) })) } } } }) {
    return Py::None;
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_TIME(), g_AMOUNT()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FACT_ABOUT_TIME(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_AMOUNT())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_STORED_COUNT(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    _turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WHAT_WAS_IT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_THEY_SAID_SO(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_turns.clone())) } } } }) {
    want = g_RECALL();
    } else {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WHAT_WAS_IT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { _turns.clone() } }) {
    return Py::None;
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_VAGUE(), "match", args))(&__args) }) {
    want = g_VAGUE();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_PICK(), g_OPTIONS()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WHICH_I_NEED(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_PICK(), g_OPTIONS()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WHAT_TO_DO_WITH(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g__LINES(), &want.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thread_is_heavy(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = memory.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEANS_ON_THE_THREAD(), "match", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_PICK(), g_STEPS(), g_OPTIONS()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BARE_ASK(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_bot_just_said_a_lot(args))(&__args) } } } } }) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_ENOUGH_ASK(), "search", args))(&__args) } } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![g_AMOUNT(), geti(&g__LINES(), &g_AMOUNT())]), 2, None);
    want = __unpk1[0].clone();
    pool = __unpk1[1].clone();
    return pool.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIT_STATE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_POINTS_AT_A_THING(), "search", args))(&__args) } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEANS_ON_THE_THREAD(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_VERDICT_WORD(), "search", args))(&__args) })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_YESNO())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DECISION(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = memory.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_AMOUNT(), g_TIME()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _on_paper(args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_AMOUNT(), g_TIME()]), &want.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_figures_are_in_the_thread(args))(&__args) } } }) {
    return Py::None;
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(want.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LINES(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&want.clone(), &g_PICK())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_two_things_are_on_the_table(args))(&__args) } } }) {
    pool = g_PICK_LINES_NAMED();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) } } }); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_count_of_the_thing_just_named(args))(&__args) }) {
    return Py::None;
    }
    return pool.clone();
    Py::None
}
pub fn _a_placeholder(args: &[Py]) -> Py {
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
    let mut fallback: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fallback") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fallback"))) };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line says nothing about this turn: a plain hedge or a bare receipt.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(py_tuple(vec![gattr(&fallback.clone(), "NEUTRAL")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "_in_a_pool", args))(&__args) }) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&fallback.clone(), "UNKNOWN")) { let mut line: Py = __item.clone(); __out.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }, &Py::None, &g__A_HEADS(), &Py::None)); } py_list(__out) })() }, &slice_or_raise(&said.clone(), &Py::None, &g__A_HEADS(), &Py::None)));
    Py::None
}
pub fn _a_thread_with_a_person_in_trouble(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fallback: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fallback") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fallback"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut reader: Py = Py::None;
    let _ = py_str("A worry about somebody's health, a death, a crisis, or a line about the body.\n\n    A general frame about how to weigh a choice is the wrong thing to say to somebody whose mother\n    is in hospital, and the bot has readers of its own for those turns.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("_in_a_care_thread"), py_str("_in_a_loss_thread"), py_str("_in_a_crisis_thread"), py_str("_thread_is_about_the_body")]));
    for __item in __iter1 {
    reader = __item.clone();
    let __try2 = exc::catch(|| {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(fallback.clone()); __args.push(reader.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("getattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, args))(&__args) }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    return Py::Bool(false);
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut clean: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The pool to say in place of a placeholder reply, or None to leave the reply alone.\n\n    `reply` is the line this turn already has, so this is asked last: after every card, place\n    route, gap reader and the composer's own lines. Only a reply that says nothing about the line\n    is replaced - one of the three plain hedges or a bare receipt - which is why the shape of an\n    answer never stands over a turn the lane can already answer.\n    ");
    let mut fallback: Py = crate::compose::fallback::module_obj();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    understood = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); __args.push(fallback.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_placeholder(args))(&__args) }))) {
    return Py::None;
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_shape_without_the_fact(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(fallback.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_thread_with_a_person_in_trouble(args))(&__args) }) {
    return Py::None;
    }
    clean = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) } } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "_choose", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.the_shape_without_the_fact".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADVICE" => Some(g_ADVICE()),
            "ADVICE_LINES" => Some(g_ADVICE_LINES()),
            "AMOUNT" => Some(g_AMOUNT()),
            "AMOUNT_LINES" => Some(g_AMOUNT_LINES()),
            "OPTIONS" => Some(g_OPTIONS()),
            "OPTIONS_LINES" => Some(g_OPTIONS_LINES()),
            "PAPERWORK" => Some(g_PAPERWORK()),
            "PAPERWORK_LINES" => Some(g_PAPERWORK_LINES()),
            "PICK" => Some(g_PICK()),
            "PICK_LINES" => Some(g_PICK_LINES()),
            "PICK_LINES_NAMED" => Some(g_PICK_LINES_NAMED()),
            "REASON" => Some(g_REASON()),
            "REASON_LINES" => Some(g_REASON_LINES()),
            "RECALL" => Some(g_RECALL()),
            "RECALL_LINES" => Some(g_RECALL_LINES()),
            "STEPS" => Some(g_STEPS()),
            "TIME" => Some(g_TIME()),
            "TIME_LINES" => Some(g_TIME_LINES()),
            "VAGUE" => Some(g_VAGUE()),
            "VAGUE_LINES" => Some(g_VAGUE_LINES()),
            "YESNO" => Some(g_YESNO()),
            "YESNO_LINES" => Some(g_YESNO_LINES()),
            "_AN_ASK_AT_THE_FRONT" => Some(g__AN_ASK_AT_THE_FRONT()),
            "_AN_ENOUGH_ASK" => Some(g__AN_ENOUGH_ASK()),
            "_A_DATE_ON_IT" => Some(g__A_DATE_ON_IT()),
            "_A_DECISION" => Some(g__A_DECISION()),
            "_A_FACT_ABOUT_TIME" => Some(g__A_FACT_ABOUT_TIME()),
            "_A_FIT_STATE" => Some(g__A_FIT_STATE()),
            "_A_FREQUENCY" => Some(g__A_FREQUENCY()),
            "_A_HEADS" => Some(g__A_HEADS()),
            "_A_PICK_ASK" => Some(g__A_PICK_ASK()),
            "_A_POINTS_AT_A_THING" => Some(g__A_POINTS_AT_A_THING()),
            "_A_STORED_COUNT" => Some(g__A_STORED_COUNT()),
            "_A_STRAIGHT_YESNO" => Some(g__A_STRAIGHT_YESNO()),
            "_A_THEY_SAID_SO" => Some(g__A_THEY_SAID_SO()),
            "_A_TIME_WORD" => Some(g__A_TIME_WORD()),
            "_A_VAGUE" => Some(g__A_VAGUE()),
            "_A_VERDICT_WORD" => Some(g__A_VERDICT_WORD()),
            "_A_WHAT_WAS_IT" => Some(g__A_WHAT_WAS_IT()),
            "_A_WHOSE_THING" => Some(g__A_WHOSE_THING()),
            "_A_WORK_OUT" => Some(g__A_WORK_OUT()),
            "_BARE_ASK" => Some(g__BARE_ASK()),
            "_HEAVY" => Some(g__HEAVY()),
            "_HOW_MANY" => Some(g__HOW_MANY()),
            "_LEANS_ON_THE_THREAD" => Some(g__LEANS_ON_THE_THREAD()),
            "_LINES" => Some(g__LINES()),
            "_NOT_AN_OPTION" => Some(g__NOT_AN_OPTION()),
            "_NOT_THE_THING" => Some(g__NOT_THE_THING()),
            "_ON_PAPER" => Some(g__ON_PAPER()),
            "_THEIR_THINGS" => Some(g__THEIR_THINGS()),
            "_TWO_THINGS" => Some(g__TWO_THINGS()),
            "_WHAT_TO_DO_WITH" => Some(g__WHAT_TO_DO_WITH()),
            "_WHICH_I_NEED" => Some(g__WHICH_I_NEED()),
            "_a_count_of_the_thing_just_named" => Some(py_func("_a_count_of_the_thing_just_named", |args: &[Py]| _a_count_of_the_thing_just_named(args))),
            "_a_placeholder" => Some(py_func("_a_placeholder", |args: &[Py]| _a_placeholder(args))),
            "_a_thread_with_a_person_in_trouble" => Some(py_func("_a_thread_with_a_person_in_trouble", |args: &[Py]| _a_thread_with_a_person_in_trouble(args))),
            "_on_paper" => Some(py_func("_on_paper", |args: &[Py]| _on_paper(args))),
            "_the_bot_just_said_a_lot" => Some(py_func("_the_bot_just_said_a_lot", |args: &[Py]| _the_bot_just_said_a_lot(args))),
            "_the_figures_are_in_the_thread" => Some(py_func("_the_figures_are_in_the_thread", |args: &[Py]| _the_figures_are_in_the_thread(args))),
            "_the_thing_is_theirs" => Some(py_func("_the_thing_is_theirs", |args: &[Py]| _the_thing_is_theirs(args))),
            "_the_thread_is_heavy" => Some(py_func("_the_thread_is_heavy", |args: &[Py]| _the_thread_is_heavy(args))),
            "_the_two_things_are_on_the_table" => Some(py_func("_the_two_things_are_on_the_table", |args: &[Py]| _the_two_things_are_on_the_table(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "the_shape_without_the_fact" => Some(py_func("the_shape_without_the_fact", |args: &[Py]| the_shape_without_the_fact(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADVICE" => s_ADVICE(v),
            "ADVICE_LINES" => s_ADVICE_LINES(v),
            "AMOUNT" => s_AMOUNT(v),
            "AMOUNT_LINES" => s_AMOUNT_LINES(v),
            "OPTIONS" => s_OPTIONS(v),
            "OPTIONS_LINES" => s_OPTIONS_LINES(v),
            "PAPERWORK" => s_PAPERWORK(v),
            "PAPERWORK_LINES" => s_PAPERWORK_LINES(v),
            "PICK" => s_PICK(v),
            "PICK_LINES" => s_PICK_LINES(v),
            "PICK_LINES_NAMED" => s_PICK_LINES_NAMED(v),
            "REASON" => s_REASON(v),
            "REASON_LINES" => s_REASON_LINES(v),
            "RECALL" => s_RECALL(v),
            "RECALL_LINES" => s_RECALL_LINES(v),
            "STEPS" => s_STEPS(v),
            "TIME" => s_TIME(v),
            "TIME_LINES" => s_TIME_LINES(v),
            "VAGUE" => s_VAGUE(v),
            "VAGUE_LINES" => s_VAGUE_LINES(v),
            "YESNO" => s_YESNO(v),
            "YESNO_LINES" => s_YESNO_LINES(v),
            "_AN_ASK_AT_THE_FRONT" => s__AN_ASK_AT_THE_FRONT(v),
            "_AN_ENOUGH_ASK" => s__AN_ENOUGH_ASK(v),
            "_A_DATE_ON_IT" => s__A_DATE_ON_IT(v),
            "_A_DECISION" => s__A_DECISION(v),
            "_A_FACT_ABOUT_TIME" => s__A_FACT_ABOUT_TIME(v),
            "_A_FIT_STATE" => s__A_FIT_STATE(v),
            "_A_FREQUENCY" => s__A_FREQUENCY(v),
            "_A_HEADS" => s__A_HEADS(v),
            "_A_PICK_ASK" => s__A_PICK_ASK(v),
            "_A_POINTS_AT_A_THING" => s__A_POINTS_AT_A_THING(v),
            "_A_STORED_COUNT" => s__A_STORED_COUNT(v),
            "_A_STRAIGHT_YESNO" => s__A_STRAIGHT_YESNO(v),
            "_A_THEY_SAID_SO" => s__A_THEY_SAID_SO(v),
            "_A_TIME_WORD" => s__A_TIME_WORD(v),
            "_A_VAGUE" => s__A_VAGUE(v),
            "_A_VERDICT_WORD" => s__A_VERDICT_WORD(v),
            "_A_WHAT_WAS_IT" => s__A_WHAT_WAS_IT(v),
            "_A_WHOSE_THING" => s__A_WHOSE_THING(v),
            "_A_WORK_OUT" => s__A_WORK_OUT(v),
            "_BARE_ASK" => s__BARE_ASK(v),
            "_HEAVY" => s__HEAVY(v),
            "_HOW_MANY" => s__HOW_MANY(v),
            "_LEANS_ON_THE_THREAD" => s__LEANS_ON_THE_THREAD(v),
            "_LINES" => s__LINES(v),
            "_NOT_AN_OPTION" => s__NOT_AN_OPTION(v),
            "_NOT_THE_THING" => s__NOT_THE_THING(v),
            "_ON_PAPER" => s__ON_PAPER(v),
            "_THEIR_THINGS" => s__THEIR_THINGS(v),
            "_TWO_THINGS" => s__TWO_THINGS(v),
            "_WHAT_TO_DO_WITH" => s__WHAT_TO_DO_WITH(v),
            "_WHICH_I_NEED" => s__WHICH_I_NEED(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.the_shape_without_the_fact", module_obj);
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_the_two_things_are_on_the_table", py_func("_the_two_things_are_on_the_table", |args: &[Py]| _the_two_things_are_on_the_table(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_the_figures_are_in_the_thread", py_func("_the_figures_are_in_the_thread", |args: &[Py]| _the_figures_are_in_the_thread(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_the_bot_just_said_a_lot", py_func("_the_bot_just_said_a_lot", |args: &[Py]| _the_bot_just_said_a_lot(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_a_count_of_the_thing_just_named", py_func("_a_count_of_the_thing_just_named", |args: &[Py]| _a_count_of_the_thing_just_named(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_on_paper", py_func("_on_paper", |args: &[Py]| _on_paper(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_the_thing_is_theirs", py_func("_the_thing_is_theirs", |args: &[Py]| _the_thing_is_theirs(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_the_thread_is_heavy", py_func("_the_thread_is_heavy", |args: &[Py]| _the_thread_is_heavy(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "the_shape_without_the_fact", py_func("the_shape_without_the_fact", |args: &[Py]| the_shape_without_the_fact(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_a_placeholder", py_func("_a_placeholder", |args: &[Py]| _a_placeholder(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "_a_thread_with_a_person_in_trouble", py_func("_a_thread_with_a_person_in_trouble", |args: &[Py]| _a_thread_with_a_person_in_trouble(args)));
    pyrt::xrefs::reg_fn("compose.the_shape_without_the_fact", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "ADVICE", g_ADVICE, s_ADVICE);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "ADVICE_LINES", g_ADVICE_LINES, s_ADVICE_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "AMOUNT", g_AMOUNT, s_AMOUNT);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "AMOUNT_LINES", g_AMOUNT_LINES, s_AMOUNT_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "OPTIONS", g_OPTIONS, s_OPTIONS);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "OPTIONS_LINES", g_OPTIONS_LINES, s_OPTIONS_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "PAPERWORK", g_PAPERWORK, s_PAPERWORK);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "PAPERWORK_LINES", g_PAPERWORK_LINES, s_PAPERWORK_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "PICK", g_PICK, s_PICK);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "PICK_LINES", g_PICK_LINES, s_PICK_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "PICK_LINES_NAMED", g_PICK_LINES_NAMED, s_PICK_LINES_NAMED);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "REASON", g_REASON, s_REASON);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "REASON_LINES", g_REASON_LINES, s_REASON_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "RECALL", g_RECALL, s_RECALL);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "RECALL_LINES", g_RECALL_LINES, s_RECALL_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "STEPS", g_STEPS, s_STEPS);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "TIME", g_TIME, s_TIME);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "TIME_LINES", g_TIME_LINES, s_TIME_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "VAGUE", g_VAGUE, s_VAGUE);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "VAGUE_LINES", g_VAGUE_LINES, s_VAGUE_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "YESNO", g_YESNO, s_YESNO);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "YESNO_LINES", g_YESNO_LINES, s_YESNO_LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_AN_ASK_AT_THE_FRONT", g__AN_ASK_AT_THE_FRONT, s__AN_ASK_AT_THE_FRONT);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_AN_ENOUGH_ASK", g__AN_ENOUGH_ASK, s__AN_ENOUGH_ASK);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_DATE_ON_IT", g__A_DATE_ON_IT, s__A_DATE_ON_IT);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_DECISION", g__A_DECISION, s__A_DECISION);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_FACT_ABOUT_TIME", g__A_FACT_ABOUT_TIME, s__A_FACT_ABOUT_TIME);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_FIT_STATE", g__A_FIT_STATE, s__A_FIT_STATE);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_FREQUENCY", g__A_FREQUENCY, s__A_FREQUENCY);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_HEADS", g__A_HEADS, s__A_HEADS);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_PICK_ASK", g__A_PICK_ASK, s__A_PICK_ASK);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_POINTS_AT_A_THING", g__A_POINTS_AT_A_THING, s__A_POINTS_AT_A_THING);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_STORED_COUNT", g__A_STORED_COUNT, s__A_STORED_COUNT);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_STRAIGHT_YESNO", g__A_STRAIGHT_YESNO, s__A_STRAIGHT_YESNO);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_THEY_SAID_SO", g__A_THEY_SAID_SO, s__A_THEY_SAID_SO);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_TIME_WORD", g__A_TIME_WORD, s__A_TIME_WORD);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_VAGUE", g__A_VAGUE, s__A_VAGUE);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_VERDICT_WORD", g__A_VERDICT_WORD, s__A_VERDICT_WORD);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_WHAT_WAS_IT", g__A_WHAT_WAS_IT, s__A_WHAT_WAS_IT);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_WHOSE_THING", g__A_WHOSE_THING, s__A_WHOSE_THING);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_A_WORK_OUT", g__A_WORK_OUT, s__A_WORK_OUT);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_BARE_ASK", g__BARE_ASK, s__BARE_ASK);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_HEAVY", g__HEAVY, s__HEAVY);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_HOW_MANY", g__HOW_MANY, s__HOW_MANY);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_LEANS_ON_THE_THREAD", g__LEANS_ON_THE_THREAD, s__LEANS_ON_THE_THREAD);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_LINES", g__LINES, s__LINES);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_NOT_AN_OPTION", g__NOT_AN_OPTION, s__NOT_AN_OPTION);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_NOT_THE_THING", g__NOT_THE_THING, s__NOT_THE_THING);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_ON_PAPER", g__ON_PAPER, s__ON_PAPER);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_THEIR_THINGS", g__THEIR_THINGS, s__THEIR_THINGS);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_TWO_THINGS", g__TWO_THINGS, s__TWO_THINGS);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_WHAT_TO_DO_WITH", g__WHAT_TO_DO_WITH, s__WHAT_TO_DO_WITH);
    pyrt::xrefs::reg_access("compose.the_shape_without_the_fact", "_WHICH_I_NEED", g__WHICH_I_NEED, s__WHICH_I_NEED);
}
