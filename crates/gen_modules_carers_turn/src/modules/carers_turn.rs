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


thread_local! { static G_ABOUT_THE_INCIDENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABOUT_THE_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSER_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ABUSE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_THE_APOLOGY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_THE_DRINK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_THE_KINDNESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS_INSTEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_THE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_THE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS_BETTER_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EASING_NOT_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EASING_UNDONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ITS_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IT_IS_NOT_YOUR_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NODDING_ALONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_OUT_OF_THE_WOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_RIGHT_AFTER_THE_WORST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOWHERE_TO_TURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_WAY_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECOVERING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROUTE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_THE_SAME_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWALLOWED_BETTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWALLOWED_NOT_RELIEF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_OWN_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEM_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_ARE_EASING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_ARE_RECOVERING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_SAY_ITS_FINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ANSWER_ASKED_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_APOLOGY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_BAG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DRINK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FEAR_IT_REPEATS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_KINDNESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_LOAD_IS_TOO_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_NEXT_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PLAN_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_TROUBLE_IS_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THREAT_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_MUCH_TO_CARRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_THE_INCIDENT() -> Py { ensure_init(); G_ABOUT_THE_INCIDENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_INCIDENT(v: Py) { ensure_init(); G_ABOUT_THE_INCIDENT.with(|c| *c.borrow_mut() = v); }
pub fn g_ABOUT_THE_THREAD() -> Py { ensure_init(); G_ABOUT_THE_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_THREAD(v: Py) { ensure_init(); G_ABOUT_THE_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSER_ANSWER() -> Py { ensure_init(); G_ABUSER_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSER_ANSWER(v: Py) { ensure_init(); G_ABUSER_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_ABUSE_ANSWER() -> Py { ensure_init(); G_ABUSE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABUSE_ANSWER(v: Py) { ensure_init(); G_ABUSE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_THE_APOLOGY() -> Py { ensure_init(); G_AFTER_THE_APOLOGY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_THE_APOLOGY(v: Py) { ensure_init(); G_AFTER_THE_APOLOGY.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_THE_DRINK() -> Py { ensure_init(); G_AFTER_THE_DRINK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_THE_DRINK(v: Py) { ensure_init(); G_AFTER_THE_DRINK.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_THE_KINDNESS() -> Py { ensure_init(); G_AFTER_THE_KINDNESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_THE_KINDNESS(v: Py) { ensure_init(); G_AFTER_THE_KINDNESS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERED() -> Py { ensure_init(); G_ANSWERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERED(v: Py) { ensure_init(); G_ANSWERED.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS_INSTEAD() -> Py { ensure_init(); G_ANSWERS_INSTEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_INSTEAD(v: Py) { ensure_init(); G_ANSWERS_INSTEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_THE_ANSWER() -> Py { ensure_init(); G_BEFORE_THE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_THE_ANSWER(v: Py) { ensure_init(); G_BEFORE_THE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED_SUBJECT() -> Py { ensure_init(); G_CHANGED_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED_SUBJECT(v: Py) { ensure_init(); G_CHANGED_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_THE_SUBJECT() -> Py { ensure_init(); G_CHANGE_THE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_THE_SUBJECT(v: Py) { ensure_init(); G_CHANGE_THE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS_BETTER_THEM() -> Py { ensure_init(); G_CRISIS_BETTER_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_BETTER_THEM(v: Py) { ensure_init(); G_CRISIS_BETTER_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS_WORDS() -> Py { ensure_init(); G_CRISIS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS_WORDS(v: Py) { ensure_init(); G_CRISIS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_EASING_NOT_ONE() -> Py { ensure_init(); G_EASING_NOT_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EASING_NOT_ONE(v: Py) { ensure_init(); G_EASING_NOT_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_EASING_UNDONE() -> Py { ensure_init(); G_EASING_UNDONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EASING_UNDONE(v: Py) { ensure_init(); G_EASING_UNDONE.with(|c| *c.borrow_mut() = v); }
pub fn g_ITS_FINE() -> Py { ensure_init(); G_ITS_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ITS_FINE(v: Py) { ensure_init(); G_ITS_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_IT_IS_NOT_YOUR_FAULT() -> Py { ensure_init(); G_IT_IS_NOT_YOUR_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IT_IS_NOT_YOUR_FAULT(v: Py) { ensure_init(); G_IT_IS_NOT_YOUR_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_ON() -> Py { ensure_init(); G_MOVE_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_ON(v: Py) { ensure_init(); G_MOVE_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_NODDING_ALONG() -> Py { ensure_init(); G_NODDING_ALONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NODDING_ALONG(v: Py) { ensure_init(); G_NODDING_ALONG.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_OUT_OF_THE_WOODS() -> Py { ensure_init(); G_NOT_OUT_OF_THE_WOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_OUT_OF_THE_WOODS(v: Py) { ensure_init(); G_NOT_OUT_OF_THE_WOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_RIGHT_AFTER_THE_WORST() -> Py { ensure_init(); G_NOT_RIGHT_AFTER_THE_WORST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_RIGHT_AFTER_THE_WORST(v: Py) { ensure_init(); G_NOT_RIGHT_AFTER_THE_WORST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOWHERE_TO_TURN() -> Py { ensure_init(); G_NOWHERE_TO_TURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOWHERE_TO_TURN(v: Py) { ensure_init(); G_NOWHERE_TO_TURN.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_WAY_OUT() -> Py { ensure_init(); G_NO_WAY_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_WAY_OUT(v: Py) { ensure_init(); G_NO_WAY_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_PACKING() -> Py { ensure_init(); G_PACKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACKING(v: Py) { ensure_init(); G_PACKING.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_RECOVERING() -> Py { ensure_init(); G_RECOVERING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECOVERING(v: Py) { ensure_init(); G_RECOVERING.with(|c| *c.borrow_mut() = v); }
pub fn g_ROUTE_NAMED() -> Py { ensure_init(); G_ROUTE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROUTE_NAMED(v: Py) { ensure_init(); G_ROUTE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_THE_SAME_THREAD() -> Py { ensure_init(); G_STILL_THE_SAME_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_THE_SAME_THREAD(v: Py) { ensure_init(); G_STILL_THE_SAME_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g_SWALLOWED_BETTER() -> Py { ensure_init(); G_SWALLOWED_BETTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWALLOWED_BETTER(v: Py) { ensure_init(); G_SWALLOWED_BETTER.with(|c| *c.borrow_mut() = v); }
pub fn g_SWALLOWED_NOT_RELIEF() -> Py { ensure_init(); G_SWALLOWED_NOT_RELIEF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWALLOWED_NOT_RELIEF(v: Py) { ensure_init(); G_SWALLOWED_NOT_RELIEF.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_OWN_FAULT() -> Py { ensure_init(); G_THEIR_OWN_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_FAULT(v: Py) { ensure_init(); G_THEIR_OWN_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEM_ASKING() -> Py { ensure_init(); G_THEM_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEM_ASKING(v: Py) { ensure_init(); G_THEM_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_ARE_EASING() -> Py { ensure_init(); G_THEY_ARE_EASING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_ARE_EASING(v: Py) { ensure_init(); G_THEY_ARE_EASING.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_ARE_RECOVERING() -> Py { ensure_init(); G_THEY_ARE_RECOVERING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_ARE_RECOVERING(v: Py) { ensure_init(); G_THEY_ARE_RECOVERING.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_SAY_ITS_FINE() -> Py { ensure_init(); G_THEY_SAY_ITS_FINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_SAY_ITS_FINE(v: Py) { ensure_init(); G_THEY_SAY_ITS_FINE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ANSWER_ASKED_A_QUESTION() -> Py { ensure_init(); G_THE_ANSWER_ASKED_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ANSWER_ASKED_A_QUESTION(v: Py) { ensure_init(); G_THE_ANSWER_ASKED_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_APOLOGY() -> Py { ensure_init(); G_THE_APOLOGY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_APOLOGY(v: Py) { ensure_init(); G_THE_APOLOGY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_BAG() -> Py { ensure_init(); G_THE_BAG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_BAG(v: Py) { ensure_init(); G_THE_BAG.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DRINK() -> Py { ensure_init(); G_THE_DRINK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DRINK(v: Py) { ensure_init(); G_THE_DRINK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FEAR_IT_REPEATS() -> Py { ensure_init(); G_THE_FEAR_IT_REPEATS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FEAR_IT_REPEATS(v: Py) { ensure_init(); G_THE_FEAR_IT_REPEATS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_KINDNESS() -> Py { ensure_init(); G_THE_KINDNESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_KINDNESS(v: Py) { ensure_init(); G_THE_KINDNESS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_LOAD_IS_TOO_MUCH() -> Py { ensure_init(); G_THE_LOAD_IS_TOO_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_LOAD_IS_TOO_MUCH(v: Py) { ensure_init(); G_THE_LOAD_IS_TOO_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_NEXT_ONE() -> Py { ensure_init(); G_THE_NEXT_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_NEXT_ONE(v: Py) { ensure_init(); G_THE_NEXT_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PLAN_ITSELF() -> Py { ensure_init(); G_THE_PLAN_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PLAN_ITSELF(v: Py) { ensure_init(); G_THE_PLAN_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_TROUBLE_IS_OVER() -> Py { ensure_init(); G_THE_TROUBLE_IS_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_TROUBLE_IS_OVER(v: Py) { ensure_init(); G_THE_TROUBLE_IS_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_THREAT_AGAIN() -> Py { ensure_init(); G_THREAT_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THREAT_AGAIN(v: Py) { ensure_init(); G_THREAT_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_MUCH_TO_CARRY() -> Py { ensure_init(); G_TOO_MUCH_TO_CARRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_MUCH_TO_CARRY(v: Py) { ensure_init(); G_TOO_MUCH_TO_CARRY.with(|c| *c.borrow_mut() = v); }
pub fn g__END() -> Py { ensure_init(); G__END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__END(v: Py) { ensure_init(); G__END.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTHS() -> Py { ensure_init(); G__MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTHS(v: Py) { ensure_init(); G__MONTHS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The turn after a safety answer that was about somebody else.\n\nWhen safety.py has just answered about a person the speaker is looking after, the next line is\nnearly always one of six things, and the composer had no reader for any of them, so they were\nanswered as if the safety card had been about the person typing:\n\n* the reasurance in the carer's own words - \"he says hes fine\", \"she swears its nothing\", \"the\n  doctor says its just bruising\";\n* a recovery that has lasted - \"hes been sober six months\", \"she hasnt used in two years\";\n* the easing - \"hes come round now\", \"shes sitting up and talking\", \"hes asleep now\";\n* the easing with a tail that undoes it - \"hes come round but hes confused\";\n* an abuse thread's own continuation - \"he says hes sorry\", \"he promised he will change\", \"he\n  says its my fault\", \"i dont have anywhere to go\";\n* the apology, the promise, the flowers, the \"it was my fault\" that follows.\n\nEach of them was answered with the instruction for a crisis in the room. After \"my boyfriend hit\nme\", \"he says hes sorry\" got \"Keep your boyfriend close tonight and ask him again tomorrow.\nRinging 116 123 with him there is easier than ringing it for him\" - the person who did the hitting\nput in the room as somebody to look after. After \"my son had a fit on the sofa\", \"hes asleep now\"\ngot \"If you haven't already, ring 999 or 911 now\" said again to somebody who had just said the\nworst was passing. After \"my friend tried to kill himself\", \"he says hes fine\" got \"take anything\nhe could take out of the room\".\n\nTwo rules run through every pool here. The line is answered about the person it is about - \"your\ndad\", \"him\", never \"you\" - and the safety card above is never simply said again. Nothing is\ninvented: where the answer depends on facts the bot cannot see (whether the next one is really\ncoming, which refuge has space tonight), the reply names the person or the line that can tell.\n\nThis is a module beside the fallback rather than a change inside it, because `bot/compose/\nfallback.py` is past the world's per-file size limit and no change to it can be submitted. It sits\nat PRIORITY -998, above the greeting band (PRIORITY -1000) and below every knowledge module, so it\nonly ever answers a line that nothing else has answered, and only when the safety turn directly\nabove was about somebody else.\n\nIt declares `ANSWERS_INSTEAD`, because a line it reads whole is a line no entry can answer: the\nthread's own subject is not a topic of the world. Without the declaration the arbiter's specialist\nrule hands the third turn to whichever row matched a word of the turn before it - after \"my\nhusband hit me\" / \"hes been drinking again\", \"should i leave\" was answered with the UK drinking\nlimits.\n");
    s_CRISIS_BETTER_THEM(pyrt::xrefs::xget("compose.fallback_data", "CRISIS_BETTER_THEM"));
    s_CRISIS_WORDS(pyrt::xrefs::xget("compose.fallback_data", "CRISIS_WORDS"));
    s_PRIORITY(op_neg(&Py::Int(998)));
    s_ANSWERS_INSTEAD(py_tuple(vec![py_str("abuse"), py_str("carers"), py_str("safety")]));
    s_ROUTE_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:999|911|112|111|a\\s?&\\s?e|a and e|accident and emergency|ambulance|paramedic|urgent care|out of hours|x-ray)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEY_ARE_EASING(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|dad|mum|mom|nan|gran|granny|grandma|grandad|grandpa|son|daughter|husband|wife|partner|brother|sister|baby|kid|child|boy|girl|little one|friend|neighbour)(?:'s|s| is| are| was| were| has| had)?\\b\\s+(?:(?:come|came|coming)\\s+(?:round|to|back round|back to)\\b|(?:awake|conscious|alert|with it|responsive|breathing (?:ok|okay|normally|fine|easier|better|properly))\\b|(?:sat|sitting) up\\b|(?:talking|chatting|eating|drinking) (?:now|again)\\b|(?:asleep|sleeping|snoozing|dozing|resting)\\b|(?:up and about|on the mend|out of the woods|over the worst)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_EASING_UNDONE(stdlib_call("re", "compile", &[py_str("\\b(?:confus\\w*|not (?:making sense|himself|herself|themselves|with it|right)|blue|grey|gray|pale|clammy|shaking|shaky|dizzy|feverish|burning up|bleeding|sick|vomit\\w*|wont wake|won't wake|cant wake|can't wake|unresponsive|limp|floppy|slurr\\w*|breathing (?:oddly|strangely|badly|funny))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEM_ASKING(stdlib_call("re", "compile", &[py_str("^(?:is|are|was|were|has|have|had|did|does|do|will|would|can|could|should|shall|any)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EASING_NOT_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:not|isnt|isn't|arent|aren't|wasnt|wasn't|wont|won't|cant|can't|couldnt|didnt|doesnt|hasnt|havent|still|worse|but|except|though|shaking|shaky|confused|blue|grey|pale|burning|hot|fever|bleeding|sick|vomit\\w*|dizzy|gone)\\b|\\?")]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVE_ON(stdlib_call("re", "compile", &[py_str("\\b(?:anyway|what about|how about|what if|should i|shall i|do i|can i|im going to|ill (?:go|ring|call|take)|what do i (?:do|say))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEY_SAY_ITS_FINE(stdlib_call("re", "compile", &[py_str("\\b(?:says?|said|telling|tells?|told|insists?|insisted|reckons?|thinks?|swears?|claims?|assures?|keeping\\s+on\\s+about|keeps?\\s+saying)\\b[^.!?]{0,26}?\\b(?:its|it's|hes|he's|shes|she's|theyre|they're|im|i'm)\\b\\s*(?:nothing|fine|ok|okay|alright|all\\s+right|not\\s+(?:that|too|so)\\s+bad|no\\s+big\\s+deal|nothing\\s+to\\s+worry\\s+about|just|only|a\\s+bit|a\\s+little|bruis\\w*|a\\s+scratch|a\\s+bump)\\b|\\b(?:told|asked|telling|asking|begged)\\s+me\\s+not\\s+to\\s+(?:worry|fuss|make\\s+a\\s+fuss|bother|think\\s+about\\s+it)\\b|\\b(?:said|says|told\\s+me|kept\\s+saying|keeps?\\s+saying)\\s+nothing\\s+(?:to\\s+worry|was\\s+wrong|is\\s+wrong|wrong)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__MONTHS(py_str("(?:\\d+|a|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a\\s+few|several)\\s+(?:days?|weeks?|months?|years?)"));
    s_THEY_ARE_RECOVERING(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:sober|clean|dry|straight|drugs?[- ]?free|off\\s+(?:it|the\\s+drink|the\\s+booze)|self[- ]harm\\s+free)\\b[\\w\\s,'-]{0,14}?\\b"), &g__MONTHS()), &py_str("|\\b")), &g__MONTHS()), &py_str("\\s+(?:sober|clean|dry)\\b|\\b(?:hasnt|has\\s+not|havent|have\\s+not|hadnt)\\s+(?:touched|used|drunk|drank|self[- ]harmed|cut|relapsed)\\b[\\w\\s,'-]{0,14}?\\b(?:in|for)\\s+")), &g__MONTHS())]).unwrap_or_else(|e| exc::raise(e)));
    s_ABUSE_ANSWER(stdlib_call("re", "compile", &[py_str("domestic abuse|abuse helpline|0808 2000 247|1-800-799-7233|what's happening to you is abuse|what is happening to you is abuse")]).unwrap_or_else(|e| exc::raise(e)));
    s_ABUSER_ANSWER(stdlib_call("re", "compile", &[py_str("0808 802 4040|people\\s+who\\s+are\\s+violent|hitting\\s+someone\\s+you\\s+live\\s+with\\s+is\\s+abuse|that\\s+is\\s+abuse,\\s+and")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_APOLOGY(stdlib_call("re", "compile", &[py_str("\\b(?:sorry|apolog\\w+|wont\\s+happen\\s+again|won't\\s+happen\\s+again|promis(?:e|ed|es)|will\\s+change|has\\s+changed|is\\s+changing|getting\\s+help|doing\\s+something\\s+about\\s+it|didn'?t\\s+mean|didnt\\s+mean|not\\s+like\\s+(?:him|her|that)|only\\s+did\\s+it\\s+once)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_KINDNESS(stdlib_call("re", "compile", &[py_str("(?:says|said|told\\s+me)\\s+(?:that\\s+)?(?:he|she)\\s+loves\\s+me|(?:he|she)\\s+loves\\s+me\\s+really|bought\\s+me|flowers|being\\s+(?:really\\s+)?(?:nice|sweet|lovely)\\s+now|making\\s+up\\s+for\\s+it|only\\s+does\\s+it\\s+when|been\\s+(?:really\\s+)?(?:good|lovely)\\s+(?:since|this\\s+week)")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_DRINK(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|is|gets?|been)\\s+(?:drunk|drinking|on\\s+something)\\b|\\bhe\\s+drinks\\b|\\bwhen\\s+he\\s+drinks\\b|\\bdrinks\\s+every\\s+(?:day|night)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN_FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:was|is|wasnt|isnt)\\s+it\\s+(?:my|mine)\\b|\\bdid\\s+i\\s+(?:do\\s+something|make|cause|provoke|push|wind)\\b|\\bmy\\s+fault\\b|\\b(?:i|we)\\s+provoked\\b|\\bwhat\\s+did\\s+i\\s+do\\b|\\b(?:i|we)\\s+(?:made|make|caused|provoked|wound\\s+up|annoyed)\\s+(?:him|her|them|my\\s+\\w+)\\s+(?:angry|mad|furious|do\\s+it|lose\\s+it|lash\\s+out|hit|upset)\\b|\\bmaybe\\s+it\\s+was\\s+me\\b|\\bprobably\\s+my\\b|\\bhow\\s+(?:do|can|should)\\s+i\\s+(?:make\\s+(?:it|things)\\s+up|make\\s+it\\s+right|put\\s+it\\s+right|make\\s+amends)\\b|\\b(?:should|shall|do)\\s+i\\s+(?:apologise|apologize|say\\s+sorry)\\b|\\bi\\s+(?:should|ought\\s+to)\\s+(?:apologise|apologize|say\\s+sorry)\\b|\\bmake\\s+(?:it|things)\\s+up\\s+to\\s+(?:him|her|them|my\\s+\\w+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THREAT_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:will|would|going\\s+to|gonna|wont|is\\s+going\\s+to)\\s+(?:do\\s+it|kill|hurt|hit|beat)\\s+(?:me\\s+)?again\\b|\\bnext\\s+time\\b|\\bthreaten\\w*|\\bhe\\s+said\\s+he.?d\\s+(?:kill|hurt|hit)\\b|\\bsaid\\s+hed\\s+(?:kill|hurt|hit)\\b|\\bhell\\s+do\\s+it\\s+again\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NODDING_ALONG(stdlib_call("re", "compile", &[py_str("^\\W*(?:ok|okay|k|kk|thanks|thank\\s+you|cheers|ta|right|yeah|yes|no|bye|goodnight|night|fair\\s+enough|got\\s+it)\\W*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_TROUBLE_IS_OVER(stdlib_call("re", "compile", &[py_str("\\b(?:(?:we(?:'re| are)?|were|weve)\\s+(?:okay|ok|fine|good)|all\\s+good\\b|its\\s+fine\\s+now\\b|(?:he|she|they)(?:'s|s|\\s+is|\\s+are)?\\s+(?:fine|okay|ok|alright)\\s+now\\b|no\\s+worries\\b|sorted\\s+now\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_WAY_OUT(stdlib_call("re", "compile", &[py_str("\\bnowhere\\s+(?:to\\s+go|to\\s+stay|else\\s+to\\s+go|safe)\\b|\\bno\\s+way\\s+out\\b|\\bno\\s+money\\s+to\\s+(?:leave|go)\\b|\\bcan'?t\\s+(?:leave|afford\\s+to\\s+leave)\\b|\\bcant\\s+(?:leave|afford\\s+to\\s+leave)\\b|\\bno\\s+one\\s+to\\s+turn\\s+to\\b|\\bdont\\s+have\\s+anywhere\\b|\\bdon'?t\\s+have\\s+anywhere\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SWALLOWED_NOT_RELIEF(stdlib_call("re", "compile", &[py_str("\\b(?:overdos\\w*|poison\\w*|swallow\\w*|paracetamol|ibuprofen|bleach|antifreeze|pills|tablets|capsules|carbon\\s+monoxide|took\\s+(?:too|a\\s+load|a\\s+bunch|loads))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TOO_MUCH_TO_CARRY(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:can'?t|cant|couldn'?t|couldnt)\\s+(?:take|do|keep\\s+(?:doing\\s+)?|carry|handle)\\s*(?:it|this|any\\s*more|anymore|much\\s+more)\\b|\\b(?:i|we)\\s+(?:can'?t|cant)\\s+(?:take|do)\\s+(?:any\\s*)?more\\b|\\bi\\s+(?:can'?t|cant)\\s+cope\\b|\\b(?:its|it'?s|it\\s+is)\\s+too\\s+much\\s+(?:for\\s+me|to\\s+(?:take|carry|bear))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_ANSWER_ASKED_A_QUESTION(stdlib_call("re", "compile", &[py_str("^[^.!]*\\?")]).unwrap_or_else(|e| exc::raise(e)));
    s_ANSWERED(stdlib_call("re", "compile", &[py_str("^\\W*(?:yes|yeah|yep|no|nope|not|its\\s+just|it\\s+is\\s+just|just|only|nothing|bruis\\w*|she\\s+says|he\\s+says|they\\s+say|i\\s+think\\s+it|i\\s+checked)")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_RIGHT_AFTER_THE_WORST(py_tuple(vec![py_str("That is 999 or 911 rather than a wait and see - coming round and then not being right is the thing they want to hear about, so say what you saw when it started. Stay with {who} and keep {obj} still until they come.")]));
    s_NOT_OUT_OF_THE_WOODS(py_tuple(vec![py_str("That it passed is not the same as being safe. Stay with {who} and get {obj} seen today, and say what happened when it started - a fit, a fall or a collapse wants a doctor the same day."), py_str("Good that it eased, and it does not cancel the check: {who} needs to be seen today, not left to sleep it off. Ring 111 or the number you were given and say what happened, and stay until somebody has looked.")]));
    s_SWALLOWED_BETTER(py_tuple(vec![py_str("With something swallowed the harm can go on for hours after it starts to look better, and that is why it is the line now rather than later. Ring 999 or 911, say what {subj} took and when, and keep the packet to read from."), py_str("That it eased does not mean it is over - what was swallowed goes on working after somebody starts to feel better, and the amount and the time are what the line needs. Ring 999 or 911 now, and stay with {who} until somebody has looked.")]));
    s_BEFORE_THE_ANSWER(py_tuple(vec![py_str("That is the better sign, and it does not answer what you were asked: it is the confusion, the grey colour or the not-making-sense that decides this one, not how settled {subj} looks. If any of that is there, or comes back, it is 999 or 911 - otherwise get {obj} seen today."), py_str("Glad to hear it, and keep the question open rather than take it as answered: what decides it is whether {subj} is properly with it, not how settled {subj} looks. Watch for the change - drowsy, confused, or not right - and ring 999 or 911 if it comes.")]));
    s_ITS_FINE(py_tuple(vec![py_str("Take it as a good sign rather than the whole of it. Keep the number where {who} can see it, and ask again tomorrow - the question is easier the second time."), py_str("Good that {subj} said it out loud, and what someone says when they are asked is often the hope rather than the plan. Keep the question open and ask again when it is quiet."), py_str("It is worth hearing, and it does not settle it - people play it down at first. What helps is staying close tonight and asking {obj} again in the morning.")]));
    s_RECOVERING(py_tuple(vec![py_str("That is a real stretch behind {obj}, and worth saying out loud. Nothing to do about it now except keep the door open: the crisis line on a bad night, and the GP if {subj} would like the help to keep going."), py_str("That is good news, and it is the kind that holds with company rather than on its own. Keep the number where {subj} can find it, and take the good months as they come.")]));
    s_AFTER_THE_KINDNESS(py_tuple(vec![py_str("It can be meant, and it is the part most people are offered at this stage. Keep it alongside the rest rather than instead of it: somewhere you could go tonight, your keys and phone where you can reach them, and the abuse line's number in the phone."), py_str("Good weeks between the bad ones are what makes this hard to name, and they do not cancel the bad ones out. Note down what happened and when, keep the number 0808 2000 247 in the UK (1-800-799-7233 in the US) close, and let somebody hear all of it rather than the good part.")]));
    s_AFTER_THE_APOLOGY(py_tuple(vec![py_str("An apology after it happens is usual, and it is not the part that changes anything - a change is something you would see over months, not in the hour afterwards. A domestic abuse line (0808 2000 247 in the UK, 1-800-799-7233 in the US) will plan the next step with you when it is safe to talk."), py_str("It can be meant, and it is the part most people are offered at this stage. Keep it alongside the rest rather than instead of it: somewhere you could go tonight, your keys and phone where you can reach them, and the abuse line's number in the phone.")]));
    s_IT_IS_NOT_YOUR_FAULT(py_tuple(vec![py_str("No. Nothing you said or did makes it your fault, and the choice to hit was theirs. If it would help to hear it from somebody who hears this all day, the abuse line is free and answers any hour."), py_str("It is not your fault, and what happened was not a reaction to something you did - hitting is a thing a person chooses to do. A domestic abuse line (0808 2000 247 in the UK, 1-800-799-7233 in the US) will say that out loud with you and plan from there.")]));
    s_THE_NEXT_ONE(py_tuple(vec![py_str("Saying it will happen again is the thing to act on rather than wait out: ring 999 or 911 if you are in danger now, and take what you would need to leave with you - keys, phone, medication. The abuse line (0808 2000 247 in the UK) will plan the rest with you."), py_str("That is a threat, and a threat like that is what turns into the next one. Get somewhere with other people in it tonight, ring 999 or 911 the moment you are frightened, and 0808 2000 247 will work out where you could go if you would rather not go alone.")]));
    s_NOWHERE_TO_TURN(py_tuple(vec![py_str("There is somewhere to go, and it is arranged through the abuse line rather than looked up tonight: 0808 2000 247 in the UK (1-800-799-7233 in the US) will say what is free near you, and refuges keep their addresses off the open internet for the obvious reason. Take your keys, your phone, any medication, and whatever papers you can reach."), py_str("Refuges exist for exactly this, and the abuse line (0808 2000 247 in the UK, 1-800-799-7233 in the US) knows which ones have space tonight - that is the thing it does all day, and it does not need you to have decided anything. If getting to a phone is not safe, one person you trust is enough of a start.")]));
    s_ABOUT_THE_THREAD(stdlib_call("re", "compile", &[py_str("\\b(?:he|him|his|she|her|hers|they|them|their|we|us|our|my\\s+(?:partner|husband|wife|boyfriend|girlfriend|ex|dad|mum|father|mother|son|daughter|brother|sister|friend)\\b)|\\b(?:what\\s+should\\s+i\\s+do|what\\s+do\\s+i\\s+do|how\\s+do\\s+i\\s+get\\s+out|who\\s+can\\s+i\\s+call|where\\s+can\\s+i\\s+go|should\\s+i\\s+leave)\\b(?:\\s+(?:now|next|though|then))?\\s*(?:[.?!,]|$)")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHANGED_SUBJECT(py_tuple(vec![py_str("Fair enough - what is on your mind instead?"), py_str("Of course. Anything you would rather talk about? I am here for it."), py_str("That is all right, we can leave it there. What else is going on with you?")]));
    s_CHANGE_THE_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|shall|would)\\s+(?:we|i)\\s+(?:please\\s+|just\\s+)?(?:talk|chat|speak)\\s+about\\s+(?:something|anything|summat|another\\s+thing|a\\s+different\\s+(?:thing|subject))\\b|\\b(?:can|could|shall)\\s+we\\s+(?:please\\s+|just\\s+)?(?:change|drop|leave)\\s+(?:the\\s+subject|it|this|that)\\b|\\bi\\s*(?:'| a)?m\\s+not\\s+(?:sure\\s+i\\s+want|going)\\s+to\\s+talk\\s+about\\b|\\bi\\s+do\\s?n'?o?t\\s+want\\s+to\\s+talk\\s+about\\s+(?:it|this|that|him|her|them)\\b|\\blet'?s\\s+(?:not\\s+)?(?:talk|chat)\\s+about\\s+(?:something|anything)\\s+else\\b|\\blet'?s\\s+not\\s+(?:talk|chat)\\s+about\\s+(?:it|this|that)\\b|\\benough\\s+about\\s+(?:that|it|this)\\b|\\bmoving\\s+on\\s+from\\s+(?:this|that|it)\\b|\\b(?:something|anything)\\s+else\\s+(?:instead|for\\s+a\\s+bit|for\\s+now)\\b|\\b(?:im|i\\s+am|ive)\\s+(?:done|finished)\\s+talking\\s+about\\b|\\b(?:drop|leave|park)\\s+it\\s+(?:for\\s+now|please)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THE_INCIDENT(stdlib_call("re", "compile", &[py_str("\\b(?:its\\s+not\\s+that\\s+bad|not\\s+that\\s+bad|it\\s+wasn't\\s+that\\s+bad|only\\s+once|it\\s+was\\s+my\\s+fault|it\\s+was\\s+an\\s+accident|he\\s+didn'?t\\s+mean|she\\s+didn'?t\\s+mean|it\\s+just\\s+happened)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_THE_DRINK(py_tuple(vec![py_str("The drink is the part that makes the rest of it more likely rather than less, and it is not the reason it is allowed to happen. A domestic abuse line (0808 2000 247 in the UK, 1-800-799-7233 in the US) hears this version of it all day: they will plan around the nights it is worse, and the number is free at any hour."), py_str("A night that goes that way when {subj} has been drinking is still {subj} choosing it, and the pattern is what to plan for - where you would go, what you would take, who would come. The abuse line (0808 2000 247 in the UK) will do that planning with you, sober hours or not.")]));
    s_THE_BAG(py_tuple(vec![py_str("Phone, keys, medication, and the papers you can reach - passport, bank cards, birth certificates - plus a change of clothes. One bag you can carry, and nothing you would have to go back for. The abuse line (0808 2000 247 in the UK, 1-800-799-7233 in the US) will say what a refuge has already, so you are not packing for a fortnight."), py_str("Smaller than people expect: phone and charger, keys, medication, papers, cards, and whatever the children cannot sleep without if they are coming. A bag by the door is also the thing that makes tonight's decision a smaller one.")]));
    s_THE_LOAD_IS_TOO_MUCH(py_tuple(vec![py_str("You have been holding that on your own, and it is too much for one person to hold. Is there one person you could tell today - a friend, a sister, a neighbour - even if it is only that you are not coping?"), py_str("Of course you are worn out: nothing about this is small, and saying so is not a failure. The abuse line (0808 2000 247 in the UK, 1-800-799-7233 in the US) answers at any hour, and you do not have to have decided anything to ring it."), py_str("That is a lot to carry with no let-up, and you do not have to work out where it ends tonight. Tell one person you trust today, and keep 0808 2000 247 (1-800-799-7233 in the US) in the phone for the hour it gets to be too much.")]));
    s_STILL_THE_SAME_THREAD(py_tuple(vec![py_str("What you do next is your call, and it is an easier call with somebody who hears this all day: 0808 2000 247 in the UK, 1-800-799-7233 in the US, free and any hour. Tonight: keys, phone and money where you can reach them, and one person you could go to."), py_str("It does not have to have been the worst night of it to count - these threads mostly start small and get worse. Keep a note of what happened and when, somewhere only you can open, and keep the abuse line's number where you can find it in a hurry.")]));
    s__END(py_str("(?:\\s+(?:now|next|though|then))?\\s*(?:[.?!,]|$)"));
    s_THE_PLAN_ITSELF(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:should|shall|do|can|could|will) (?:i|we) (?:leave|leaves|go|stay|move|get out|walk out)\\b"), &g__END()), &py_str("|\\bwhere (?:do|can|should) i (?:go|stay|sleep tonight)\\b")), &g__END()), &py_str("|\\bshould i leave\\b")), &g__END()), &py_str("|\\bcan i leave\\b")), &g__END()), &py_str("|\\bdo i need to (?:leave|go|get out)\\b|\\bwhat (?:should|do|can|could) i do\\b")), &g__END()), &py_str("|\\bwhat now\\b")), &g__END()), &py_str("|\\bwhat do i do next\\b")), &g__END()), &py_str("|\\bshould i (?:go back|stay with (?:him|her|them))\\b|\\b(?:how|when) (?:do|can|should|would) i (?:get|collect|pick up|take|move) (?:my|the|our)\\s+(?:stuff|things|belongings|clothes|papers|paperwork|documents|passport)\\b|\\bcan i take (?:the|my) (?:kids|children)\\b|\\bwhat (?:do|should|can|could) i (?:tell|say to) (?:the|my|our)\\s+(?:kids|children|son|daughter|family|mum|mom|mother|parents|boss)\\b")), &g__END()), &py_str("|\\bhow long (?:will|does|is) (?:this|it|all this) (?:take|going to take|going to go on)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PACKING(stdlib_call("re", "compile", &[py_str("\\bwhat (?:should|do|can|could|shall) i (?:take|bring|pack)\\b|\\bwhat (?:do|will|should) i (?:need|want) to (?:take|bring|pack)\\b|\\bpack a bag\\b|\\bwhat (?:should|do) i (?:put|get) in (?:the|my) bag\\b|\\bwhat do i take with me\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_FEAR_IT_REPEATS(stdlib_call("re", "compile", &[py_str("\\bwill (?:it|this|he|she|they) (?:get|be) worse\\b|\\bis it going to get worse\\b|\\bwill (?:it|this) (?:happen|get) again\\b|\\bwill (?:he|she) do it again\\b|\\bdoes it get worse\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _clean(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _turns(args: &[Py]) -> Py {
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
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() }]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled5 = true;
    return py_list(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _the_answer_above(args: &[Py]) -> Py {
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
    let mut module: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The turn directly above this line, or None when it is not one this file answers after.\n\n    The safety card, and then this file's own answers for as long as the thread stays an abuse\n    thread. The second is what keeps a third and fourth turn in the same reading: the card was\n    answered two turns up, the turn directly above was this file's reply to an apology, and the\n    line was \"should i leave\" - which stood down here because the module above was not safety's,\n    and was then answered from whichever pool held the apology, so the person who had been hit\n    was asked to name what they did and say why it was wrong (magenta-pine, #13295). A thread\n    that is not about abuse still has to have safety's answer directly above: \"my dad collapsed\"\n    then \"hes come round now\" keeps the carer reading.\n    ");
    turns = _turns(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    module = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("safety"), py_str("safety_cards")]), &module.clone()))) {
    return last.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&module.clone(), &py_str("carers_turn"))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut turn: Py = __item.clone(); __out.push(_an_abuse_thread(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return last.clone();
    }
    return Py::None;
    Py::None
}
pub fn _reads_as_easing(args: &[Py]) -> Py {
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
    let _ = py_str("A report that their person is through the worst, and nothing in the line that undoes it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_THEY_ARE_EASING(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_EASING_NOT_ONE(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MOVE_ON(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEM_ASKING(), "match", &[clean.clone()]) } } } }));
    Py::None
}
pub fn _reads_as_easing_but_worse(args: &[Py]) -> Py {
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
    let _ = py_str("The easing with a tail that needs a person: \"hes come round but hes confused\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_THEY_ARE_EASING(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_EASING_UNDONE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_abuse_thread(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut text: Py = Py::None;
    text = mcall(&call_builtin("str", &[{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return { let __b = call_builtin("bool", &[mcall(&g_ABUSE_ANSWER(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ABUSER_ANSWER(), "search", &[text.clone()]))) } };
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
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut char: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("One line from a pool, filled with the words for the person it is about.\n\n    Two things are read before the line is chosen. The line the bot said last turn is dropped from\n    the pool, because the same paragraph twice in a row reads as a machine that is not listening:\n    \"what do i do\" then \"should i go back\" came back identical, because the two questions add up to\n    the same number.\n    ");
    lines = if py_truthy(&forms.clone()) { pyrt::xrefs::xcall("compose.fallback", "_wrote_for_them", &[pool.clone(), forms.clone()]) } else { call_builtin("tuple", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    if py_truthy(&Py::Bool(!py_truthy(&lines.clone()))) {
    return Py::None;
    }
    turns = _turns(&[memory.clone()]);
    said = mcall(&call_builtin("str", &[{ let __b = if py_truthy(&turns.clone()) { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]) } else { py_str("") }; if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&line.clone(), "strip", &[]), &said.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() };
    if py_truthy(&fresh.clone()) {
    lines = fresh.clone();
    }
    }
    start = op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clean.clone()) { let mut char: Py = __item.clone(); __out.push(call_builtin("ord", &[char.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return geti(&lines.clone(), &start.clone());
    Py::None
}
pub fn _the_thread(args: &[Py]) -> Py {
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
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut past: Py = Py::None;
    let _ = py_str("Everything said in this thread, lower-cased, for a reading to search.");
    lines = py_list(vec![call_builtin("str", &[{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    let __iter1 = py_iter_or_raise(&_turns(&[memory.clone()]));
    for __item in __iter1 {
    past = __item.clone();
    let _ = mcall(&lines.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&past.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    let _ = mcall(&lines.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&past.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push(mcall(&line.clone(), "lower", &[])); } } py_list(__out) })() }]);
    Py::None
}
pub fn _the_answer_sent_them_somewhere(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let _ = py_str("True when the answer above asked for a person to be seen, not for a watch and wait.");
    return call_builtin("bool", &[mcall(&g_ROUTE_NAMED(), "search", &[mcall(&call_builtin("str", &[{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_words_for_the_one_who_did_it(args: &[Py]) -> Py {
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
    let mut forms: Py = Py::None;
    let mut relation: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The words for the person the abuse came from, for the pools written about them.\n\n    `_about_somebody_else` cannot supply these: it stands down on the speaker's own crisis on\n    purpose (`SOMEBODY_HURT_ME` - \"my boyfriend hit me\" is the person typing, not somebody they\n    care for), and asking it first is what made every pool below unreachable for exactly the\n    thread the module's own docstring is about: after \"my boyfriend hit me\", \"he says hes sorry\"\n    fell to the greeting because `forms` was None before the abuse branch was ever reached.\n\n    The pools are written without placeholders bar one line that reads `{subj}`, so what this has\n    to get right is the pronoun, and `memory[\"safety_person\"]` is the one the safety answer above\n    used (\"he\" for \"my boyfriend hit me\"). Nothing here guesses who they are.\n    ");
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("safety_person")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    relation = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("person")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&mcall(&said.clone(), "startswith", &[py_str("she")])) {
    forms = py_dict(vec![(py_str("subject"), py_str("she")), (py_str("object"), py_str("her")), (py_str("possessive"), py_str("her")), (py_str("be"), py_str("is"))]);
    } else {
    if py_truthy(&mcall(&said.clone(), "startswith", &[py_str("they")])) {
    forms = py_dict(vec![(py_str("subject"), py_str("they")), (py_str("object"), py_str("them")), (py_str("possessive"), py_str("their")), (py_str("be"), py_str("are"))]);
    } else {
    forms = py_dict(vec![(py_str("subject"), py_str("he")), (py_str("object"), py_str("him")), (py_str("possessive"), py_str("his")), (py_str("be"), py_str("is"))]);
    }
    }
    let _ = py_setitem(&forms.clone(), &py_str("who"), if py_truthy(&relation.clone()) { op_add(&py_str("your "), &relation.clone()) } else { geti(&forms.clone(), &py_str("object")) });
    return forms.clone();
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
    let mut above: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut in_abuse: Py = Py::None;
    let mut in_crisis: Py = Py::None;
    let mut last: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    clean = _clean(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    above = _the_answer_above(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&above.clone(), &Py::None))) {
    return Py::None;
    }
    last = mcall(&call_builtin("str", &[{ let __b = mcall(&above.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&g_NODDING_ALONG(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_TROUBLE_IS_OVER(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_CHANGE_THE_SUBJECT(), "search", &[clean.clone()])) {
    return _pick(&[g_CHANGED_SUBJECT(), Py::None, clean.clone(), memory.clone()]);
    }
    in_abuse = { let __b = _an_abuse_thread(&[last.clone()]); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_turns(&[memory.clone()])) { let mut turn: Py = __item.clone(); __out.push(_an_abuse_thread(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&in_abuse.clone()) {
    forms = _the_words_for_the_one_who_did_it(&[memory.clone()]);
    } else {
    forms = pyrt::xrefs::xcall("compose.fallback", "_about_somebody_else", &[py_dict(vec![(py_str("user"), call_builtin("str", &[{ let __b = mcall(&above.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&forms.clone()))) {
    return Py::None;
    }
    }
    if py_truthy(&in_abuse.clone()) {
    if py_truthy(&mcall(&g_PACKING(), "search", &[clean.clone()])) {
    return _pick(&[g_THE_BAG(), Py::None, clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_THE_PLAN_ITSELF(), "search", &[clean.clone()])) {
    return _pick(&[pyrt::xrefs::xget("compose.fallback", "ABUSE_AFTER"), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_THEIR_OWN_FAULT(), "search", &[clean.clone()])) {
    return _pick(&[g_IT_IS_NOT_YOUR_FAULT(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_NO_WAY_OUT(), "search", &[clean.clone()])) {
    return _pick(&[g_NOWHERE_TO_TURN(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_THREAT_AGAIN(), "search", &[clean.clone()])) {
    return _pick(&[g_THE_NEXT_ONE(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_THE_FEAR_IT_REPEATS(), "search", &[clean.clone()])) {
    return _pick(&[g_THE_NEXT_ONE(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_THE_DRINK(), "search", &[clean.clone()])) {
    return _pick(&[g_AFTER_THE_DRINK(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_THE_KINDNESS(), "search", &[clean.clone()])) {
    return _pick(&[g_AFTER_THE_KINDNESS(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_THE_APOLOGY(), "search", &[clean.clone()])) {
    return _pick(&[g_AFTER_THE_APOLOGY(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_TOO_MUCH_TO_CARRY(), "search", &[clean.clone()])) {
    return _pick(&[g_THE_LOAD_IS_TOO_MUCH(), Py::None, clean.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_ABOUT_THE_THREAD(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ABOUT_THE_INCIDENT(), "search", &[clean.clone()]) } }))) {
    return Py::None;
    }
    return _pick(&[g_STILL_THE_SAME_THREAD(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&_reads_as_easing_but_worse(&[clean.clone()])) {
    return _pick(&[g_NOT_RIGHT_AFTER_THE_WORST(), forms.clone(), clean.clone(), memory.clone()]);
    }
    asked = mcall(&g_THE_ANSWER_ASKED_A_QUESTION(), "search", &[mcall(&call_builtin("str", &[{ let __b = mcall(&above.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_ANSWERED(), "match", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { _reads_as_easing(&[clean.clone()]) } } } }) {
    return _pick(&[g_BEFORE_THE_ANSWER(), forms.clone(), clean.clone(), memory.clone()]);
    }
    in_crisis = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("compose.fallback_data", "CRISIS_WORDS")) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&last.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&in_crisis.clone()) {
    if py_truthy(&mcall(&g_THEY_SAY_ITS_FINE(), "search", &[clean.clone()])) {
    return _pick(&[g_ITS_FINE(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&mcall(&g_THEY_ARE_RECOVERING(), "search", &[clean.clone()])) {
    return _pick(&[g_RECOVERING(), forms.clone(), clean.clone(), memory.clone()]);
    }
    if py_truthy(&_reads_as_easing(&[clean.clone()])) {
    if py_truthy(&mcall(&g_SWALLOWED_NOT_RELIEF(), "search", &[_the_thread(&[memory.clone(), last.clone()])])) {
    return _pick(&[g_SWALLOWED_BETTER(), forms.clone(), clean.clone(), memory.clone()]);
    }
    return _pick(&[pyrt::xrefs::xget("compose.fallback_data", "CRISIS_BETTER_THEM"), forms.clone(), clean.clone(), memory.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = _reads_as_easing(&[clean.clone()]); if !py_truthy(&__b) { __b } else { _the_answer_sent_them_somewhere(&[last.clone()]) } }) {
    if py_truthy(&mcall(&g_SWALLOWED_NOT_RELIEF(), "search", &[_the_thread(&[memory.clone(), last.clone()])])) {
    return _pick(&[g_SWALLOWED_BETTER(), forms.clone(), clean.clone(), memory.clone()]);
    }
    return _pick(&[g_NOT_OUT_OF_THE_WOODS(), forms.clone(), clean.clone(), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.carers_turn".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_INCIDENT" => Some(g_ABOUT_THE_INCIDENT()),
            "ABOUT_THE_THREAD" => Some(g_ABOUT_THE_THREAD()),
            "ABUSER_ANSWER" => Some(g_ABUSER_ANSWER()),
            "ABUSE_ANSWER" => Some(g_ABUSE_ANSWER()),
            "AFTER_THE_APOLOGY" => Some(g_AFTER_THE_APOLOGY()),
            "AFTER_THE_DRINK" => Some(g_AFTER_THE_DRINK()),
            "AFTER_THE_KINDNESS" => Some(g_AFTER_THE_KINDNESS()),
            "ANSWERED" => Some(g_ANSWERED()),
            "ANSWERS_INSTEAD" => Some(g_ANSWERS_INSTEAD()),
            "BEFORE_THE_ANSWER" => Some(g_BEFORE_THE_ANSWER()),
            "CHANGED_SUBJECT" => Some(g_CHANGED_SUBJECT()),
            "CHANGE_THE_SUBJECT" => Some(g_CHANGE_THE_SUBJECT()),
            "CRISIS_BETTER_THEM" => Some(g_CRISIS_BETTER_THEM()),
            "CRISIS_WORDS" => Some(g_CRISIS_WORDS()),
            "EASING_NOT_ONE" => Some(g_EASING_NOT_ONE()),
            "EASING_UNDONE" => Some(g_EASING_UNDONE()),
            "ITS_FINE" => Some(g_ITS_FINE()),
            "IT_IS_NOT_YOUR_FAULT" => Some(g_IT_IS_NOT_YOUR_FAULT()),
            "MOVE_ON" => Some(g_MOVE_ON()),
            "NODDING_ALONG" => Some(g_NODDING_ALONG()),
            "NOT_OUT_OF_THE_WOODS" => Some(g_NOT_OUT_OF_THE_WOODS()),
            "NOT_RIGHT_AFTER_THE_WORST" => Some(g_NOT_RIGHT_AFTER_THE_WORST()),
            "NOWHERE_TO_TURN" => Some(g_NOWHERE_TO_TURN()),
            "NO_WAY_OUT" => Some(g_NO_WAY_OUT()),
            "PACKING" => Some(g_PACKING()),
            "PRIORITY" => Some(g_PRIORITY()),
            "RECOVERING" => Some(g_RECOVERING()),
            "ROUTE_NAMED" => Some(g_ROUTE_NAMED()),
            "STILL_THE_SAME_THREAD" => Some(g_STILL_THE_SAME_THREAD()),
            "SWALLOWED_BETTER" => Some(g_SWALLOWED_BETTER()),
            "SWALLOWED_NOT_RELIEF" => Some(g_SWALLOWED_NOT_RELIEF()),
            "THEIR_OWN_FAULT" => Some(g_THEIR_OWN_FAULT()),
            "THEM_ASKING" => Some(g_THEM_ASKING()),
            "THEY_ARE_EASING" => Some(g_THEY_ARE_EASING()),
            "THEY_ARE_RECOVERING" => Some(g_THEY_ARE_RECOVERING()),
            "THEY_SAY_ITS_FINE" => Some(g_THEY_SAY_ITS_FINE()),
            "THE_ANSWER_ASKED_A_QUESTION" => Some(g_THE_ANSWER_ASKED_A_QUESTION()),
            "THE_APOLOGY" => Some(g_THE_APOLOGY()),
            "THE_BAG" => Some(g_THE_BAG()),
            "THE_DRINK" => Some(g_THE_DRINK()),
            "THE_FEAR_IT_REPEATS" => Some(g_THE_FEAR_IT_REPEATS()),
            "THE_KINDNESS" => Some(g_THE_KINDNESS()),
            "THE_LOAD_IS_TOO_MUCH" => Some(g_THE_LOAD_IS_TOO_MUCH()),
            "THE_NEXT_ONE" => Some(g_THE_NEXT_ONE()),
            "THE_PLAN_ITSELF" => Some(g_THE_PLAN_ITSELF()),
            "THE_TROUBLE_IS_OVER" => Some(g_THE_TROUBLE_IS_OVER()),
            "THREAT_AGAIN" => Some(g_THREAT_AGAIN()),
            "TOO_MUCH_TO_CARRY" => Some(g_TOO_MUCH_TO_CARRY()),
            "_END" => Some(g__END()),
            "_MONTHS" => Some(g__MONTHS()),
            "_an_abuse_thread" => Some(py_func("_an_abuse_thread", |args: &[Py]| _an_abuse_thread(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_reads_as_easing" => Some(py_func("_reads_as_easing", |args: &[Py]| _reads_as_easing(args))),
            "_reads_as_easing_but_worse" => Some(py_func("_reads_as_easing_but_worse", |args: &[Py]| _reads_as_easing_but_worse(args))),
            "_the_answer_above" => Some(py_func("_the_answer_above", |args: &[Py]| _the_answer_above(args))),
            "_the_answer_sent_them_somewhere" => Some(py_func("_the_answer_sent_them_somewhere", |args: &[Py]| _the_answer_sent_them_somewhere(args))),
            "_the_thread" => Some(py_func("_the_thread", |args: &[Py]| _the_thread(args))),
            "_the_words_for_the_one_who_did_it" => Some(py_func("_the_words_for_the_one_who_did_it", |args: &[Py]| _the_words_for_the_one_who_did_it(args))),
            "_turns" => Some(py_func("_turns", |args: &[Py]| _turns(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THE_INCIDENT" => s_ABOUT_THE_INCIDENT(v),
            "ABOUT_THE_THREAD" => s_ABOUT_THE_THREAD(v),
            "ABUSER_ANSWER" => s_ABUSER_ANSWER(v),
            "ABUSE_ANSWER" => s_ABUSE_ANSWER(v),
            "AFTER_THE_APOLOGY" => s_AFTER_THE_APOLOGY(v),
            "AFTER_THE_DRINK" => s_AFTER_THE_DRINK(v),
            "AFTER_THE_KINDNESS" => s_AFTER_THE_KINDNESS(v),
            "ANSWERED" => s_ANSWERED(v),
            "ANSWERS_INSTEAD" => s_ANSWERS_INSTEAD(v),
            "BEFORE_THE_ANSWER" => s_BEFORE_THE_ANSWER(v),
            "CHANGED_SUBJECT" => s_CHANGED_SUBJECT(v),
            "CHANGE_THE_SUBJECT" => s_CHANGE_THE_SUBJECT(v),
            "CRISIS_BETTER_THEM" => s_CRISIS_BETTER_THEM(v),
            "CRISIS_WORDS" => s_CRISIS_WORDS(v),
            "EASING_NOT_ONE" => s_EASING_NOT_ONE(v),
            "EASING_UNDONE" => s_EASING_UNDONE(v),
            "ITS_FINE" => s_ITS_FINE(v),
            "IT_IS_NOT_YOUR_FAULT" => s_IT_IS_NOT_YOUR_FAULT(v),
            "MOVE_ON" => s_MOVE_ON(v),
            "NODDING_ALONG" => s_NODDING_ALONG(v),
            "NOT_OUT_OF_THE_WOODS" => s_NOT_OUT_OF_THE_WOODS(v),
            "NOT_RIGHT_AFTER_THE_WORST" => s_NOT_RIGHT_AFTER_THE_WORST(v),
            "NOWHERE_TO_TURN" => s_NOWHERE_TO_TURN(v),
            "NO_WAY_OUT" => s_NO_WAY_OUT(v),
            "PACKING" => s_PACKING(v),
            "PRIORITY" => s_PRIORITY(v),
            "RECOVERING" => s_RECOVERING(v),
            "ROUTE_NAMED" => s_ROUTE_NAMED(v),
            "STILL_THE_SAME_THREAD" => s_STILL_THE_SAME_THREAD(v),
            "SWALLOWED_BETTER" => s_SWALLOWED_BETTER(v),
            "SWALLOWED_NOT_RELIEF" => s_SWALLOWED_NOT_RELIEF(v),
            "THEIR_OWN_FAULT" => s_THEIR_OWN_FAULT(v),
            "THEM_ASKING" => s_THEM_ASKING(v),
            "THEY_ARE_EASING" => s_THEY_ARE_EASING(v),
            "THEY_ARE_RECOVERING" => s_THEY_ARE_RECOVERING(v),
            "THEY_SAY_ITS_FINE" => s_THEY_SAY_ITS_FINE(v),
            "THE_ANSWER_ASKED_A_QUESTION" => s_THE_ANSWER_ASKED_A_QUESTION(v),
            "THE_APOLOGY" => s_THE_APOLOGY(v),
            "THE_BAG" => s_THE_BAG(v),
            "THE_DRINK" => s_THE_DRINK(v),
            "THE_FEAR_IT_REPEATS" => s_THE_FEAR_IT_REPEATS(v),
            "THE_KINDNESS" => s_THE_KINDNESS(v),
            "THE_LOAD_IS_TOO_MUCH" => s_THE_LOAD_IS_TOO_MUCH(v),
            "THE_NEXT_ONE" => s_THE_NEXT_ONE(v),
            "THE_PLAN_ITSELF" => s_THE_PLAN_ITSELF(v),
            "THE_TROUBLE_IS_OVER" => s_THE_TROUBLE_IS_OVER(v),
            "THREAT_AGAIN" => s_THREAT_AGAIN(v),
            "TOO_MUCH_TO_CARRY" => s_TOO_MUCH_TO_CARRY(v),
            "_END" => s__END(v),
            "_MONTHS" => s__MONTHS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.carers_turn", module_obj);
    pyrt::xrefs::reg_fn("modules.carers_turn", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_turns", py_func("_turns", |args: &[Py]| _turns(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_the_answer_above", py_func("_the_answer_above", |args: &[Py]| _the_answer_above(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_reads_as_easing", py_func("_reads_as_easing", |args: &[Py]| _reads_as_easing(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_reads_as_easing_but_worse", py_func("_reads_as_easing_but_worse", |args: &[Py]| _reads_as_easing_but_worse(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_an_abuse_thread", py_func("_an_abuse_thread", |args: &[Py]| _an_abuse_thread(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_the_thread", py_func("_the_thread", |args: &[Py]| _the_thread(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_the_answer_sent_them_somewhere", py_func("_the_answer_sent_them_somewhere", |args: &[Py]| _the_answer_sent_them_somewhere(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "_the_words_for_the_one_who_did_it", py_func("_the_words_for_the_one_who_did_it", |args: &[Py]| _the_words_for_the_one_who_did_it(args)));
    pyrt::xrefs::reg_fn("modules.carers_turn", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.carers_turn", "ABOUT_THE_INCIDENT", g_ABOUT_THE_INCIDENT, s_ABOUT_THE_INCIDENT);
    pyrt::xrefs::reg_access("modules.carers_turn", "ABOUT_THE_THREAD", g_ABOUT_THE_THREAD, s_ABOUT_THE_THREAD);
    pyrt::xrefs::reg_access("modules.carers_turn", "ABUSER_ANSWER", g_ABUSER_ANSWER, s_ABUSER_ANSWER);
    pyrt::xrefs::reg_access("modules.carers_turn", "ABUSE_ANSWER", g_ABUSE_ANSWER, s_ABUSE_ANSWER);
    pyrt::xrefs::reg_access("modules.carers_turn", "AFTER_THE_APOLOGY", g_AFTER_THE_APOLOGY, s_AFTER_THE_APOLOGY);
    pyrt::xrefs::reg_access("modules.carers_turn", "AFTER_THE_DRINK", g_AFTER_THE_DRINK, s_AFTER_THE_DRINK);
    pyrt::xrefs::reg_access("modules.carers_turn", "AFTER_THE_KINDNESS", g_AFTER_THE_KINDNESS, s_AFTER_THE_KINDNESS);
    pyrt::xrefs::reg_access("modules.carers_turn", "ANSWERED", g_ANSWERED, s_ANSWERED);
    pyrt::xrefs::reg_access("modules.carers_turn", "ANSWERS_INSTEAD", g_ANSWERS_INSTEAD, s_ANSWERS_INSTEAD);
    pyrt::xrefs::reg_access("modules.carers_turn", "BEFORE_THE_ANSWER", g_BEFORE_THE_ANSWER, s_BEFORE_THE_ANSWER);
    pyrt::xrefs::reg_access("modules.carers_turn", "CHANGED_SUBJECT", g_CHANGED_SUBJECT, s_CHANGED_SUBJECT);
    pyrt::xrefs::reg_access("modules.carers_turn", "CHANGE_THE_SUBJECT", g_CHANGE_THE_SUBJECT, s_CHANGE_THE_SUBJECT);
    pyrt::xrefs::reg_access("modules.carers_turn", "CRISIS_BETTER_THEM", g_CRISIS_BETTER_THEM, s_CRISIS_BETTER_THEM);
    pyrt::xrefs::reg_access("modules.carers_turn", "CRISIS_WORDS", g_CRISIS_WORDS, s_CRISIS_WORDS);
    pyrt::xrefs::reg_access("modules.carers_turn", "EASING_NOT_ONE", g_EASING_NOT_ONE, s_EASING_NOT_ONE);
    pyrt::xrefs::reg_access("modules.carers_turn", "EASING_UNDONE", g_EASING_UNDONE, s_EASING_UNDONE);
    pyrt::xrefs::reg_access("modules.carers_turn", "ITS_FINE", g_ITS_FINE, s_ITS_FINE);
    pyrt::xrefs::reg_access("modules.carers_turn", "IT_IS_NOT_YOUR_FAULT", g_IT_IS_NOT_YOUR_FAULT, s_IT_IS_NOT_YOUR_FAULT);
    pyrt::xrefs::reg_access("modules.carers_turn", "MOVE_ON", g_MOVE_ON, s_MOVE_ON);
    pyrt::xrefs::reg_access("modules.carers_turn", "NODDING_ALONG", g_NODDING_ALONG, s_NODDING_ALONG);
    pyrt::xrefs::reg_access("modules.carers_turn", "NOT_OUT_OF_THE_WOODS", g_NOT_OUT_OF_THE_WOODS, s_NOT_OUT_OF_THE_WOODS);
    pyrt::xrefs::reg_access("modules.carers_turn", "NOT_RIGHT_AFTER_THE_WORST", g_NOT_RIGHT_AFTER_THE_WORST, s_NOT_RIGHT_AFTER_THE_WORST);
    pyrt::xrefs::reg_access("modules.carers_turn", "NOWHERE_TO_TURN", g_NOWHERE_TO_TURN, s_NOWHERE_TO_TURN);
    pyrt::xrefs::reg_access("modules.carers_turn", "NO_WAY_OUT", g_NO_WAY_OUT, s_NO_WAY_OUT);
    pyrt::xrefs::reg_access("modules.carers_turn", "PACKING", g_PACKING, s_PACKING);
    pyrt::xrefs::reg_access("modules.carers_turn", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.carers_turn", "RECOVERING", g_RECOVERING, s_RECOVERING);
    pyrt::xrefs::reg_access("modules.carers_turn", "ROUTE_NAMED", g_ROUTE_NAMED, s_ROUTE_NAMED);
    pyrt::xrefs::reg_access("modules.carers_turn", "STILL_THE_SAME_THREAD", g_STILL_THE_SAME_THREAD, s_STILL_THE_SAME_THREAD);
    pyrt::xrefs::reg_access("modules.carers_turn", "SWALLOWED_BETTER", g_SWALLOWED_BETTER, s_SWALLOWED_BETTER);
    pyrt::xrefs::reg_access("modules.carers_turn", "SWALLOWED_NOT_RELIEF", g_SWALLOWED_NOT_RELIEF, s_SWALLOWED_NOT_RELIEF);
    pyrt::xrefs::reg_access("modules.carers_turn", "THEIR_OWN_FAULT", g_THEIR_OWN_FAULT, s_THEIR_OWN_FAULT);
    pyrt::xrefs::reg_access("modules.carers_turn", "THEM_ASKING", g_THEM_ASKING, s_THEM_ASKING);
    pyrt::xrefs::reg_access("modules.carers_turn", "THEY_ARE_EASING", g_THEY_ARE_EASING, s_THEY_ARE_EASING);
    pyrt::xrefs::reg_access("modules.carers_turn", "THEY_ARE_RECOVERING", g_THEY_ARE_RECOVERING, s_THEY_ARE_RECOVERING);
    pyrt::xrefs::reg_access("modules.carers_turn", "THEY_SAY_ITS_FINE", g_THEY_SAY_ITS_FINE, s_THEY_SAY_ITS_FINE);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_ANSWER_ASKED_A_QUESTION", g_THE_ANSWER_ASKED_A_QUESTION, s_THE_ANSWER_ASKED_A_QUESTION);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_APOLOGY", g_THE_APOLOGY, s_THE_APOLOGY);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_BAG", g_THE_BAG, s_THE_BAG);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_DRINK", g_THE_DRINK, s_THE_DRINK);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_FEAR_IT_REPEATS", g_THE_FEAR_IT_REPEATS, s_THE_FEAR_IT_REPEATS);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_KINDNESS", g_THE_KINDNESS, s_THE_KINDNESS);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_LOAD_IS_TOO_MUCH", g_THE_LOAD_IS_TOO_MUCH, s_THE_LOAD_IS_TOO_MUCH);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_NEXT_ONE", g_THE_NEXT_ONE, s_THE_NEXT_ONE);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_PLAN_ITSELF", g_THE_PLAN_ITSELF, s_THE_PLAN_ITSELF);
    pyrt::xrefs::reg_access("modules.carers_turn", "THE_TROUBLE_IS_OVER", g_THE_TROUBLE_IS_OVER, s_THE_TROUBLE_IS_OVER);
    pyrt::xrefs::reg_access("modules.carers_turn", "THREAT_AGAIN", g_THREAT_AGAIN, s_THREAT_AGAIN);
    pyrt::xrefs::reg_access("modules.carers_turn", "TOO_MUCH_TO_CARRY", g_TOO_MUCH_TO_CARRY, s_TOO_MUCH_TO_CARRY);
    pyrt::xrefs::reg_access("modules.carers_turn", "_END", g__END, s__END);
    pyrt::xrefs::reg_access("modules.carers_turn", "_MONTHS", g__MONTHS, s__MONTHS);
}
