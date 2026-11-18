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


thread_local! { static G_A_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANNOT_USE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANNOT_USE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_ORDERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_THING_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_TO_DO_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREPOSITIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFUSAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFUSAL_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFUSAL_THING_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFUSAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFUSED_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_ORDERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_SAME_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNABLE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANY_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEDICINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TREATMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_DOING() -> Py { ensure_init(); G_A_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DOING(v: Py) { ensure_init(); G_A_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_CANNOT_USE() -> Py { ensure_init(); G_CANNOT_USE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANNOT_USE(v: Py) { ensure_init(); G_CANNOT_USE.with(|c| *c.borrow_mut() = v); }
pub fn g_CANNOT_USE_ASKS() -> Py { ensure_init(); G_CANNOT_USE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANNOT_USE_ASKS(v: Py) { ensure_init(); G_CANNOT_USE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_DISLIKE() -> Py { ensure_init(); G_DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISLIKE(v: Py) { ensure_init(); G_DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME() -> Py { ensure_init(); G_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME(v: Py) { ensure_init(); G_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME_BREAK() -> Py { ensure_init(); G_FRAME_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME_BREAK(v: Py) { ensure_init(); G_FRAME_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME_WORDS() -> Py { ensure_init(); G_FRAME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME_WORDS(v: Py) { ensure_init(); G_FRAME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS() -> Py { ensure_init(); G_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); G_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_ORDERS() -> Py { ensure_init(); G_LEAD_ORDERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_ORDERS(v: Py) { ensure_init(); G_LEAD_ORDERS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_THING_LEADS() -> Py { ensure_init(); G_NOTHING_THING_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_THING_LEADS(v: Py) { ensure_init(); G_NOTHING_THING_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_TO_DO_LEADS() -> Py { ensure_init(); G_NOTHING_TO_DO_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_TO_DO_LEADS(v: Py) { ensure_init(); G_NOTHING_TO_DO_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_LIMIT() -> Py { ensure_init(); G_NOT_A_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_LIMIT(v: Py) { ensure_init(); G_NOT_A_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PREPOSITIONS() -> Py { ensure_init(); G_PREPOSITIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREPOSITIONS(v: Py) { ensure_init(); G_PREPOSITIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_REFUSAL() -> Py { ensure_init(); G_REFUSAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFUSAL(v: Py) { ensure_init(); G_REFUSAL.with(|c| *c.borrow_mut() = v); }
pub fn g_REFUSAL_LEADS() -> Py { ensure_init(); G_REFUSAL_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFUSAL_LEADS(v: Py) { ensure_init(); G_REFUSAL_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_REFUSAL_THING_LEADS() -> Py { ensure_init(); G_REFUSAL_THING_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFUSAL_THING_LEADS(v: Py) { ensure_init(); G_REFUSAL_THING_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_REFUSAL_WORDS() -> Py { ensure_init(); G_REFUSAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFUSAL_WORDS(v: Py) { ensure_init(); G_REFUSAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_REFUSED_VERBS() -> Py { ensure_init(); G_REFUSED_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFUSED_VERBS(v: Py) { ensure_init(); G_REFUSED_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORTER() -> Py { ensure_init(); G_SHORTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORTER(v: Py) { ensure_init(); G_SHORTER.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_ORDERS() -> Py { ensure_init(); G_STEP_ORDERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_ORDERS(v: Py) { ensure_init(); G_STEP_ORDERS.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_WORDS() -> Py { ensure_init(); G_STEP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_WORDS(v: Py) { ensure_init(); G_STEP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP() -> Py { ensure_init(); G_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); G_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_NOUN() -> Py { ensure_init(); G_THE_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_NOUN(v: Py) { ensure_init(); G_THE_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_SAME_MOVE() -> Py { ensure_init(); G_THE_SAME_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_SAME_MOVE(v: Py) { ensure_init(); G_THE_SAME_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNABLE_VERB() -> Py { ensure_init(); G_UNABLE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNABLE_VERB(v: Py) { ensure_init(); G_UNABLE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__ANY_MOVE() -> Py { ensure_init(); G__ANY_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANY_MOVE(v: Py) { ensure_init(); G__ANY_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g__MEDICINE() -> Py { ensure_init(); G__MEDICINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEDICINE(v: Py) { ensure_init(); G__MEDICINE.with(|c| *c.borrow_mut() = v); }
pub fn g__TREATMENT() -> Py { ensure_init(); G__TREATMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TREATMENT(v: Py) { ensure_init(); G__TREATMENT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A stated dislike of the thing the last answer leaned on.\n\n\"i want to lose weight\" gets the plan, and the next line is \"i hate exercise\". Nothing answers\nit, so the composer echoes the person's own words back (\"Got it - you hate exercise\"), which is a\nreceipt for a sentence rather than a reply to it, and it ends the conversation: the person has\njust said what does not fit, and the bot has said nothing about it.\n\nWhat a friend does with \"i hate exercise\" is leave it out and say which part of the plan still\nholds: \"Fair enough - this part doesn't need exercise: Fill up on protein and fibre first...\". The\nstep is the one they were already given, so it is honest without any new knowledge, and it answers\nthe objection instead of filing it.\n\nThe rule is narrow on purpose:\n* the line has to be nothing but a stated dislike or a lack of a thing (\"i hate exercise\", \"i\n  cant stand the gym\", \"i dont like cooking\"): `ack.only_a_dislike` is the same reading the\n  composer uses to keep a dislike from being read as a bad day, so \"i hate my boss shouting at me\"\n  and \"i hate my job\" stay the composer's;\n* the answer being objected to has to be a knowledge answer with a step in it, and it has to be\n  about the same thing the chat is still on, so a line that changes the subject (\"how do i fix the\n  tap\" then \"i hate exercise\") is left alone;\n* a step that itself names the thing they dislike is not handed back as the thing to do - the\n  answer is searched for the first step that does not need it, and when every step needs it the\n  line is left to the composer;\n* the step is not said a third time: what has already come back once comes back in its first\n  clause or not at all.\n\nThe same answer is on the screen for a refusal of the move rather than a dislike of a thing:\n\"theres a mouse in my kitchen\" gets the way in, the traps and the poison, and \"i dont want to\nkill it\" was met with the floor (\"Got it. Is there anything you'd like a hand with?\") - a line\nthat would fit any message, one turn after being told how. What a friend does with that is not\nargue for the trap: it is to say that the move is not owed at all, and hand over the part of the\nanswer that never needed it (\"You don't have to kill it: Find the way in first: ... block those\nwith wire wool and filler\"). No new knowledge goes into it - the part handed over is a sentence of\nthe answer already given, so it is honest by construction.\n\nThat reading is narrower than it looks. The refused move has to be one of a short list of words a\nknowledge answer actually prescribes (`kill`, `trap`, `poison`, `bait`, `take`), together with the\nwords the answer uses for the same move (`THE_SAME_MOVE`), and the answer has to contain one of\nthem: \"i dont want to upset her\" and \"i dont want to make it a big thing\" name nothing the answer\nprescribes, so those lines keep the composer's reply. The step handed over has to contain none of\nthe family at all, or the reply would hand back the very thing they refused.\n\nA refusal names its move either as a doing (\"i dont want to kill it\", \"i dont want to take pills\")\nor as the thing itself (\"i dont want pellets\", \"i dont want medication\"), and the lead is written\nfor the one it is: a noun put where a doing goes reads as \"You don't have to pellets\".\n");
    s_DISLIKE(py_tuple(vec![stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*i\\s+(?:(?:just|really|properly|actually|kind of|sort of)\\s+)*(?:hate|hates|dislike|dislikes)\\s+(?P<thing>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*i\\s+(?:(?:just|really|properly|actually)\\s+)*(?:cant|cannot|can\\s*not|can'?t)\\s+stand\\s+(?P<thing>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*i\\s*(?:'m|m|am)?\\s*(?:not|n't)\\s+(?:a\\s+)?(?:fan|keen)\\s+(?:of|on)\\s+(?P<thing>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*i\\s+(?:(?:just|really|properly|actually)\\s+)*(?:dont|do\\s*not|don'?t)\\s+(?:like|love|fancy)\\s+(?P<thing>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("it its it's that this these those them him her you me everything\n    anything something stuff things thing more any all\n    myself yourself himself herself itself ourselves themselves"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_WORDS(Py::Int(12));
    s_STEP_WORDS(Py::Int(16));
    s_SHORTER(Py::Int(10));
    s_LEADS(py_tuple(vec![py_str("Fair enough - this part doesn't need {thing}: {step}"), py_str("That's fine, this works without {thing}: {step}"), py_str("You can leave {thing} out of this one: {step}"), py_str("This one works without {thing}: {step}"), py_str("Then skip {thing} - the rest of it stands: {step}")]));
    s_CANNOT_USE_ASKS(py_tuple(vec![py_str("No {bare}, then - what could you do instead?"), py_str("So {thing} won't work - what else is there?"), py_str("Okay, so not {thing}: what could you do instead?")]));
    s_REFUSAL(py_tuple(vec![stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright|honestly)\\W+)*i\\W+(?:(?:just|really|actually|honestly|properly|definitely|absolutely|kind\\s+of|sort\\s+of)\\W+)*(?:dont|do\\s*not|don'?t|wouldnt|would\\s*not|won'?t|wont)\\s+(?:want\\s+(?:to\\s+)?|wanna\\s+)?(?P<move>[a-z].*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright|honestly)\\W+)*i(?:['\\u2019]?d|\\s+would)\\s+(?:really\\s+|much\\s+)?rather\\s+(?:not\\s+)?(?P<move>[a-z].*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright|honestly)\\W+)*i\\s+(?:refuse\\s+to|wouldnt\\s+be\\s+able\\s+to|wont\\s+be\\s+able\\s+to)\\s+(?P<move>[a-z].*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s__TREATMENT(py_tuple(vec![py_str("spot on"), py_str("flea treatment"), py_str("flea product"), py_str("flea collar"), py_str("flea tablet"), py_str("flea drop"), py_str("wormer"), py_str("dewormer"), py_str("shampoo"), py_str("permethrin"), py_str("fipronil"), py_str("imidacloprid"), py_str("frontline"), py_str("advocate"), py_str("drops")]));
    s__MEDICINE(py_tuple(vec![py_str("pill"), py_str("tablet"), py_str("medicine"), py_str("medication"), py_str("dose"), py_str("drug"), py_str("supplement"), py_str("paracetamol"), py_str("ibuprofen"), py_str("aspirin"), py_str("painkiller"), py_str("antibiotic"), py_str("antihistamine"), py_str("steroid"), py_str("inhaler"), py_str("melatonin")]));
    s_THE_SAME_MOVE(py_dict(vec![(py_str("kill"), py_tuple(vec![py_str("kill"), py_str("trap"), py_str("snap"), py_str("poison"), py_str("bait"), py_str("pest"), py_str("exterminat")])), (py_str("trap"), py_tuple(vec![py_str("trap"), py_str("snap"), py_str("bait"), py_str("kill")])), (py_str("poison"), op_add(&py_tuple(vec![py_str("poison"), py_str("bait"), py_str("pesticide"), py_str("chemical"), py_str("spray"), py_str("fogger"), py_str("insecticide")]), &g__TREATMENT())), (py_str("bait"), py_tuple(vec![py_str("bait"), py_str("poison"), py_str("trap")])), (py_str("chemical"), op_add(&py_tuple(vec![py_str("chemical"), py_str("pesticide"), py_str("spray"), py_str("insecticide"), py_str("fogger"), py_str("poison")]), &g__TREATMENT())), (py_str("spray"), op_add(&py_tuple(vec![py_str("spray"), py_str("pesticide"), py_str("chemical"), py_str("fogger"), py_str("insecticide")]), &g__TREATMENT())), (py_str("fogger"), op_add(&py_tuple(vec![py_str("fogger"), py_str("spray"), py_str("pesticide"), py_str("chemical"), py_str("insecticide")]), &g__TREATMENT())), (py_str("pellet"), py_tuple(vec![py_str("pellet"), py_str("bait"), py_str("poison"), py_str("slug")])), (py_str("pill"), g__MEDICINE()), (py_str("tablet"), g__MEDICINE()), (py_str("medicine"), g__MEDICINE()), (py_str("medication"), g__MEDICINE()), (py_str("drug"), g__MEDICINE()), (py_str("take"), g__MEDICINE()), (py_str("swallow"), g__MEDICINE())]));
    s_REFUSAL_LEADS(py_tuple(vec![py_str("You don't have to {move} to start. {step}"), py_str("Start with the part that does not need you to {move}. {step}"), py_str("That comes later, if at all - this first: {step}")]));
    s_LEAD_ORDERS(stdlib_call("re", "compile", &[py_str("\\bthis first\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_ORDERS(stdlib_call("re", "compile", &[py_str("^(?:first|firstly|find|start|begin|sort)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_TO_DO_LEADS(py_tuple(vec![py_str("You don't have to {move}. {step}"), py_str("None of this needs you to {move}. {step}"), py_str("Nobody is asking you to {move}. {step}")]));
    s_REFUSAL_THING_LEADS(py_tuple(vec![py_str("You don't have to start with the {thing}. {step}"), py_str("Start with the part that needs no {thing}. {step}")]));
    s_NOTHING_THING_LEADS(py_tuple(vec![py_str("None of this needs {thing}. {step}"), py_str("Nobody is asking for {thing}. {step}"), py_str("You can leave {thing} out of this: {step}")]));
    s_THE_NOUN(stdlib_call("re", "compile", &[py_str("^(?:the|a|an|any|my|your|our|their|his|her|some|those|these)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_WORDS(call_builtin("frozenset", &[mcall(&py_str("i dont don't do not want wanna to really just actually honestly\n    properly definitely absolutely kind of sort would much rather though tho too either at all as\n    well anymore now any and but so oh also yeah yep ok okay right alright"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSAL_WORDS(Py::Int(60));
    s_STOP(call_builtin("frozenset", &[mcall(&py_str("the a an my our your their his her its some any and but so of to for at on\n    in with about really very quite just too"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNABLE_VERB(py_str("do|use|afford|eat|have|get\\s+to|go\\s+to|join|manage|handle|attend|access|buy|pay\\s+for|reach|keep\\s+up"));
    s_CANNOT_USE(py_tuple(vec![stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:the\\s+problem\\s+is\\s+|its\\s+just\\s+that\\s+)?i\\s+(?:(?:just|really|actually|properly|kind\\s+of|sort\\s+of)\\s+)*(?:cant|cannot|can\\s*not|can'?t)\\s+(?:"), &g_UNABLE_VERB()), &py_str(")\\s+(?P<thing>.+)$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:the\\s+problem\\s+is\\s+|its\\s+just\\s+that\\s+)?(?:i|we)\\s+(?:(?:just|really|actually)\\s+)*(?:dont|do\\s*not|don'?t|havent|have\\s*not|haven'?t)\\s+(?:got\\s+)?(?:have\\s+|own\\s+)?(?P<thing>(?:a|an|any|the|my|our)\\s+[a-z][a-z' ]{0,24})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_NOT_A_LIMIT(call_builtin("frozenset", &[mcall(&py_str("time clue idea choice chance point reason way word answer option\n    money cash budget it that this them those these"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__ANY_MOVE(call_builtin("tuple", &[call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_THE_SAME_MOVE(), "values", &[])) { let mut family: Py = __item.clone(); for __item in py_iter_or_raise(&family.clone()) { let mut stem: Py = __item.clone(); __out.push(stem.clone()); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DOING(op_bitor(&crate::memory::topics::g_IMPERATIVE(), &call_builtin("frozenset", &[mcall(&py_str("\nempty vacuum hoover dust brush scrub bleach seal strip snip trap bait sprinkle hang launder soak\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_REFUSED_VERBS(op_bitor(&g_A_DOING(), &call_builtin("frozenset", &[mcall(&py_str("\nkill trap bait poison spray use take put set call pay buy hire throw burn bury dig cut wash fill\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_PREPOSITIONS(call_builtin("frozenset", &[mcall(&py_str("in on at round around under over by near behind inside outside from\n    with about across through along"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME(stdlib_call("re", "compile", &[py_str("^(?:the things? that (?:work|help|matter)|what (?:works|helps)|the fix|the trick|the plan|the short version|what i would do|the basics)\\b[^:.!?]{0,40}:\\s+\\S"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_BREAK(stdlib_call("re", "compile", &[py_str("\\s*(?::|\\u2014|\\u2013|--|;)\\s*")]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
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
    return stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&_flat(&[text.clone()]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_thing(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing the person has just said they cannot stand, or None.\n\n    None means the line is not a stated dislike on its own: a question, a longer sentence about\n    someone's behaviour, or a dislike of \"it\", which is about what was already said.\n    ");
    said = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::memory::ack::only_a_dislike(&[said.clone()])))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_DISLIKE());
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    thing = mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "strip", &[py_str(" .!,;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &mcall(&thing.clone(), "lower", &[]))) } }) {
    return Py::None;
    }
    return thing.clone();
    }
    return Py::None;
    Py::None
}
pub fn the_thing_they_cannot_use(args: &[Py]) -> Py {
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
    let mut core: Py = Py::None;
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the person has just said they cannot do or have not got, or None.\n\n    \"i cant do gyms\" after the plan for losing a stone is the same shape as \"i hate exercise\": one\n    part of the plan is out - except that nothing in it can be handed back without the thing, so\n    the answer has to ask what is left. The line has to be short and to name the thing itself, so\n    \"i cant do it\" and \"i cant believe it\" are not this.\n    ");
    said = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_CANNOT_USE());
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    thing = mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "strip", &[py_str(" .!,;:")]);
    core = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[thing.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_STOP(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &core.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_LIMIT(), &core.clone())) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:because|since|as|though|although|but|unless|if|when)\\b"), mcall(&thing.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return thing.clone();
    }
    return Py::None;
    Py::None
}
pub fn the_move(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The move the person has just refused, in their own words, or None.\n\n    None means the line is not a refusal of a move on its own: a question, a longer sentence, or a\n    \"i dont want to\" about something other than a thing to do (\"i dont want a row\").\n    ");
    said = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_REFUSAL());
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    move_ = mcall(&mcall(&found.clone(), "group", &[py_str("move")]), "strip", &[py_str(" .!,;:")]);
    move_ = stdlib_call("re", "sub", &[py_str("^(?:be\\s+|have\\s+to\\s+|keep\\s+)"), py_str(""), move_.clone()]).unwrap_or_else(|e| exc::raise(e));
    move_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\s,]+(?:though|tho|too|either|really|honestly|at\\s+all|as\\s+well|any\\s+more|now)$")); __args.push(py_str("")); __args.push(move_.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&move_.clone()))) {
    return Py::None;
    }
    family = _the_family(&[move_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&family.clone()))) {
    return Py::None;
    }
    return move_.clone();
    }
    return Py::None;
    Py::None
}
pub fn _the_family(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut move_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "move") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: move"))) };
    let mut family: Py = Py::None;
    let mut key: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words the answer uses for the move the person has refused, or ().\n\n    The key is the first word of the move that a knowledge answer prescribes, read by its stem so\n    \"chemicals\" and \"pellets\" are the words they are: \"i dont want to use chemicals\" is a refusal\n    of the chemicals, not of using.\n    ");
    let __iter1 = py_iter_or_raise(&_words(&[move_.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_STOP(), &word.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_THE_SAME_MOVE(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    key = __unpk3[0].clone();
    family = __unpk3[1].clone();
    if py_truthy(&mcall(&word.clone(), "startswith", &[key.clone()])) {
    return call_builtin("tuple", &[family.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _has_family(args: &[Py]) -> Py {
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
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut stem: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether any of these words is one the answer uses for the move (or a form of one).");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    let __iter2 = py_iter_or_raise(&family.clone());
    for __item in __iter2 {
    stem = __item.clone();
    if py_truthy(&mcall(&word.clone(), "startswith", &[stem.clone()])) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _carries_a_move(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut said: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("Whether this sentence names the move, by a word of the family or by a name of two words.");
    if py_truthy(&_has_family(&[_words(&[sentence.clone()]), family.clone()])) {
    return Py::Bool(true);
    }
    said = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9]+"), py_str(" "), mcall(&_flat(&[sentence.clone()]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&family.clone()) { let mut stem: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&stem.clone(), &py_str(" ")))) {  __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &stem.clone()), &py_str(" "))))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _opens_on_a_verb(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut move_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "move") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: move"))) };
    let mut words: Py = Py::None;
    let _ = py_str("True when the refusal names the doing (\"kill it\", \"spray anything\"), not the thing.\n\n    One word is a thing: \"i dont want pellets\" and \"i dont want poison\" both name the substance. A\n    second word that is a preposition leaves the first one a thing too (\"poison in the house\"),\n    while an object after it makes it the doing (\"poison it\", \"spray chemicals about\"). Without the\n    second half of this, \"i dont want to spray anything\" - a deck row - was read as a noun and the\n    lead became \"You don't have to start with the spray anything\".\n    ");
    words = _words(&[move_.clone()]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_REFUSED_VERBS(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&g_PREPOSITIONS(), &geti(&words.clone(), &Py::Int(1))));
    Py::None
}
pub fn _the_noun(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut move_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "move") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: move"))) };
    let _ = py_str("The move as a bare noun: \"the pellets\" and \"pellets at all\" are both \"pellets\".");
    return mcall(&mcall(&g_THE_NOUN(), "sub", &[py_str(""), _flat(&[move_.clone()])]), "strip", &[py_str(" .!,;:")]);
    Py::None
}
pub fn _a_list_under_a_frame(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("True when the sentence is a framed list of dos: \"The things that work: ..., ..., and ...\".");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_FRAME(), "match", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    tail = geti(&mcall(&said.clone(), "split", &[py_str(":"), Py::Int(1)]), &Py::Int(1));
    return op_ge(&mcall(&tail.clone(), "count", &[py_str(",")]), &Py::Int(2));
    Py::None
}
pub fn _a_doing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut first: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the sentence tells the person to do something, rather than saying what a thing is.\n\n    `detail._a_step` answers the same question for a sentence it can read as one move, but it\n    stands aside on a long one and on a list: \"Empty the wardrobe, wash or dry-clean everything\n    woollen, then vacuum the drawer ...\" is 46 words and is one instruction, and it is exactly the\n    sentence a person who will not use chemicals needs. The first word is the reliable part - a\n    reason opens on its subject (\"Waking in the night is normal\", \"Brown tips are usually dry\n    air\"), and a move opens on its verb.\n    ");
    if py_truthy(&crate::memory::detail::_a_step(&[one.clone()])) {
    return Py::Bool(true);
    }
    words = mcall(&call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    first = mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(",;:.-—–'\"")]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_A_DOING(), &first.clone()))) {
    return Py::Bool(true);
    }
    return _a_list_under_a_frame(&[one.clone()]);
    Py::None
}
pub fn _a_whole_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut cut: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut said: Py = Py::None;
    let mut trial: Py = Py::None;
    let _ = py_str("The sentence as one move, capped at `REFUSAL_WORDS` words at a clause boundary.\n\n    The punctuation is kept as the answer wrote it: \"Find the way in first: mice follow pipes and\n    cables\" is a frame and its content, and joined with a comma it reads as two instructions in a\n    row.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    if py_truthy(&op_le(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_REFUSAL_WORDS())) {
    return if py_truthy(&mcall(&said.clone(), "endswith", &[py_str(".")])) { said.clone() } else { op_add(&said.clone(), &py_str(".")) };
    }
    cut = py_str("");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[,;:])\\s+"), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    piece = __item.clone();
    trial = mcall(&op_add(&op_add(&cut.clone(), &py_str(" ")), &piece.clone()), "strip", &[]);
    if py_truthy(&{ let __b = cut.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&trial.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_REFUSAL_WORDS()) } }) {
    break;
    }
    cut = trial.clone();
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&cut.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_REFUSAL_WORDS())) {
    cut = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&cut.clone(), "split", &[]), &Py::None, &g_REFUSAL_WORDS(), &Py::None)]);
    }
    cut = mcall(&cut.clone(), "strip", &[py_str(" ,;:-")]);
    return if py_truthy(&cut.clone()) { op_add(&cut.clone(), &py_str(".")) } else { py_str("") };
    Py::None
}
pub fn a_move_refused(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut family: Py = Py::None;
    let mut gives_it: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut only_the_move: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_move: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut step: Py = Py::None;
    let mut there: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The reply for a refusal of the move the last answer gave, or None.\n\n    Only called once the composer's reply for the turn is a receipt or the floor, so a line the\n    composer has an answer for keeps it (bot/modules/greeting.py).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    move_ = the_move(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&move_.clone()))) {
    return Py::None;
    }
    family = _the_family(&[move_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&family.clone()))) {
    return Py::None;
    }
    about = mcall(&{ let __b = gen_understanding::understanding::understand(&[_flat(&[text.clone()]), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic_own")]);
    mine = call_builtin("set", &[_words(&[move_.clone()])]).unwrap_or_else(|e| exc::raise(e));
    only_the_move = Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[text.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&mine.clone(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FRAME_WORDS(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() }));
    let __iter1 = py_iter_or_raise(&crate::memory::repeat::_recent_answers(&[_before_this_turn(&[memory.clone(), text.clone()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    asked = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&op_gt(&back.clone(), &Py::Int(2))) {
    continue;
    }
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::memory::repeat::_about_the_same_thing(&[asked.clone(), said.clone(), memory.clone()]))) } }) {
    continue;
    }
    there = mcall(&{ let __b = gen_understanding::understanding::understand(&[_flat(&[asked.clone()]), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic_own")]);
    if py_truthy(&{ let __b = about.clone(); if !py_truthy(&__b) { __b } else { { let __b = there.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&about.clone(), &there.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&only_the_move.clone())) } } } } } }) {
    continue;
    }
    gives_it = _carries_a_move(&[said.clone(), family.clone()]);
    step = py_str("");
    sentence = py_str("");
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    one = __item.clone();
    if py_truthy(&_carries_a_move(&[one.clone(), family.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&gives_it.clone())); if !py_truthy(&__b) { __b } else { _carries_a_move(&[one.clone(), g__ANY_MOVE()]) } }) {
    continue;
    }
    piece = _a_whole_step(&[one.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&piece.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_doing(&[one.clone()])))) {
    continue;
    }
    let __unpk4 = py_unpack(&py_tuple(vec![piece.clone(), one.clone()]), 2, None);
    step = __unpk4[0].clone();
    sentence = __unpk4[1].clone();
    break;
    }
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    continue;
    }
    if py_truthy(&crate::memory::repeat::_already_repeated(&[step.clone(), memory.clone()])) {
    step = crate::memory::detail::_first_clause(&[step.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if py_truthy(&__b) { __b } else { crate::memory::repeat::_already_repeated(&[step.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    }
    pool = if py_truthy(&gives_it.clone()) { g_REFUSAL_LEADS() } else { g_NOTHING_TO_DO_LEADS() };
    said_move = move_.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_opens_on_a_verb(&[move_.clone()])))) {
    pool = if py_truthy(&gives_it.clone()) { g_REFUSAL_THING_LEADS() } else { g_NOTHING_THING_LEADS() };
    said_move = { let __b = _the_noun(&[move_.clone()]); if py_truthy(&__b) { __b } else { move_.clone() } };
    }
    if py_truthy(&mcall(&g_STEP_ORDERS(), "match", &[step.clone()])) {
    pool = { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut lead: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_LEAD_ORDERS(), "search", &[lead.clone()])))) {  __out.push(lead.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { pool.clone() } };
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("move".to_string(), said_move.clone()), ("thing".to_string(), said_move.clone()), ("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_call(&crate::memory::repeat::g_pick(), &[pool.clone(), memory.clone(), text.clone()]), "format", args))(&__args) };
    if py_truthy(&sentence.clone()) {
    let _ = crate::memory::repeat::_handed_back(&[memory.clone(), sentence.clone()]);
    }
    return crate::memory::repeat::_handed_back(&[memory.clone(), reply.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _under_its_frame(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut head: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The move inside a framed sentence, taken from the clause after its colon or dash.\n\n    `detail._a_step` stands aside on a sentence that opens on a frame (\"Change a few things you\n    can keep doing rather than going on a diet: fill half the plate with vegetables, put protein in\n    every meal ...\"): it is long, and the part before the colon is not the move. The move is the\n    clause after it, and without this the weight answer had no steps at all, so\n    `i want to lose weight` / `i hate exercise` was answered \"Got it - you hate exercise.\" - the\n    receipt this whole module exists to replace. The tail has to be a move in its own right\n    (`_a_doing`), so a frame followed by a fact is left alone: \"Find the way in first: mice follow\n    pipes and cables\" is not a doing under a frame, it is the why of one.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    parts = mcall(&g_FRAME_BREAK(), "split", &[said.clone(), Py::Int(1)]);
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&geti(&parts.clone(), &Py::Int(0)), "strip", &[]), mcall(&geti(&parts.clone(), &Py::Int(1)), "strip", &[])]), 2, None);
    head = __unpk1[0].clone();
    tail = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&tail.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_REFUSAL_WORDS()) } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_doing(&[tail.clone()])))) {
    return py_str("");
    }
    return _a_whole_step(&[tail.clone()]);
    Py::None
}
pub fn _the_frame(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut parts: Py = Py::None;
    let _ = py_str("The lead-in of a framed sentence (\"Change a few things you can keep doing...\"), or \"\".");
    parts = mcall(&g_FRAME_BREAK(), "split", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), Py::Int(1)]);
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) { mcall(&geti(&parts.clone(), &Py::Int(0)), "strip", &[]) } else { py_str("") };
    Py::None
}
pub fn _steps(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut out: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut step: Py = Py::None;
    let mut under: Py = Py::None;
    let _ = py_str("Every instruction in the answer, in order, as (the sentence, the step, under a frame).\n\n    The third item is True when the step was cut from under the sentence's colon rather than read\n    off the sentence itself. The sentence is then still mostly unsaid - the frame is the lead-in\n    and the rest of its list has not been handed over - so `answer_for` does not count the whole\n    of it said, which is what kept `what else` from reaching the next clause of the list.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    step = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __kws.extend(vec![("questions_are_steps".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::detail::_a_step(args))(&__args) };
    if py_truthy(&step.clone()) {
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![sentence.clone(), step.clone(), Py::Bool(false)])]);
    continue;
    }
    under = _under_its_frame(&[sentence.clone()]);
    if py_truthy(&under.clone()) {
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![sentence.clone(), under.clone(), Py::Bool(true)])]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _needs_it(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut wanted: Py = Py::None;
    let _ = py_str("Whether this step leans on the thing the person has just ruled out.");
    wanted = op_sub(&call_builtin("set", &[_words(&[thing.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_STOP());
    return call_builtin("bool", &[op_bitand(&wanted.clone(), &call_builtin("set", &[_words(&[step.clone()])]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _leans_on_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut asked: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { Py::None };
    let mut mine: Py = Py::None;
    let mut there: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("Whether the answer being objected to names the thing at all.\n\n    The dislike has to be of something the last answer actually leaned on, or every grumble in a\n    chat is answered as a part of the plan: `i want to lose weight` / `i hate waiting`, `i hate\n    mondays` and `i hate people` all came back as \"this works without waiting/mondays/people\",\n    which is a nonsense reply to a person who was only letting off steam. A thing the answer never\n    mentions is not what they are objecting to, so that line is left to the composer.\n    ");
    wanted = op_sub(&call_builtin("set", &[_words(&[thing.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_STOP());
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&wanted.clone(), &call_builtin("set", &[_words(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(true);
    }
    there = mcall(&{ let __b = gen_understanding::understanding::understand(&[_flat(&[thing.clone()]), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic_own")]);
    mine = mcall(&{ let __b = gen_understanding::understanding::understand(&[_flat(&[{ let __b = asked.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic_own")]);
    return call_builtin("bool", &[{ let __b = there.clone(); if !py_truthy(&__b) { __b } else { { let __b = mine.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&there.clone(), &mine.clone())) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _before_this_turn(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut mine: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The memory as it stood before the line being answered, when this turn is already recorded.\n\n    A module that runs after another has answered can see its own turn in the history, which\n    counts the answer it is objecting to as two turns back rather than one - and two turns back\n    has to be about the same thing the chat is still on, which by then it is not (\"i want to lose\n    weight\" / \"i cant stand the gym\" was refused for that reason). The turn being answered is the\n    one to drop, so the distance is counted in answers.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    mine = _flat(&[text.clone()]);
    while py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_flat(&[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]), &mine.clone())) } }) {
    let _ = mcall(&turns.clone(), "pop", &[]);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return memory.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("turns".to_string(), turns.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_reply_for(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut limited: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limited") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: limited"))) };
    let mut about: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut framed: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut step: Py = Py::None;
    let mut there: Py = Py::None;
    let mut under: Py = Py::None;
    let _ = py_str("The reply for a thing the line rules out of the answer above it, or None.");
    about = mcall(&{ let __b = gen_understanding::understanding::understand(&[_flat(&[text.clone()]), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic_own")]);
    let __iter1 = py_iter_or_raise(&crate::memory::repeat::_recent_answers(&[_before_this_turn(&[memory.clone(), text.clone()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    asked = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&op_gt(&back.clone(), &Py::Int(2))) {
    continue;
    }
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::memory::repeat::_about_the_same_thing(&[asked.clone(), said.clone(), memory.clone()]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&limited.clone()))) {
    there = mcall(&{ let __b = gen_understanding::understanding::understand(&[_flat(&[asked.clone()]), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic_own")]);
    if py_truthy(&{ let __b = about.clone(); if !py_truthy(&__b) { __b } else { { let __b = there.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&about.clone(), &there.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_leans_on_it(&[said.clone(), thing.clone(), asked.clone()])))) {
    continue;
    }
    }
    step = Py::None;
    sentence = py_str("");
    framed = Py::Bool(false);
    let __iter3 = py_iter_or_raise(&_steps(&[said.clone()]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 3, None);
    sentence = __unpk4[0].clone();
    piece = __unpk4[1].clone();
    under = __unpk4[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_needs_it(&[piece.clone(), thing.clone()])))) {
    step = piece.clone();
    framed = under.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    if py_truthy(&limited.clone()) {
    ask = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("bare".to_string(), mcall(&g_THE_NOUN(), "sub", &[py_str(""), thing.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_call(&crate::memory::repeat::g_pick(), &[g_CANNOT_USE_ASKS(), memory.clone(), text.clone()]), "format", args))(&__args) };
    return crate::memory::repeat::_handed_back(&[memory.clone(), ask.clone()]);
    }
    continue;
    }
    if py_truthy(&crate::memory::repeat::_already_repeated(&[step.clone(), memory.clone()])) {
    step = crate::memory::detail::_first_clause(&[step.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if py_truthy(&__b) { __b } else { crate::memory::repeat::_already_repeated(&[step.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&step.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS())) {
    step = crate::memory::detail::_first_clause(&[step.clone()]);
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_call(&crate::memory::repeat::g_pick(), &[g_LEADS(), memory.clone(), text.clone()]), "format", args))(&__args) };
    if py_truthy(&framed.clone()) {
    let _ = crate::memory::repeat::_handed_back(&[memory.clone(), step.clone()]);
    } else {
    if py_truthy(&sentence.clone()) {
    let _ = crate::memory::repeat::_handed_back(&[memory.clone(), sentence.clone()]);
    }
    }
    return crate::memory::repeat::_handed_back(&[memory.clone(), reply.clone()]);
    }
    return Py::None;
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
    let mut thing: Py = Py::None;
    let _ = py_str("The reply for a stated dislike of something the last answer leaned on, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    thing = the_thing(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    return _the_reply_for(&[text.clone(), memory.clone(), thing.clone(), Py::Bool(false)]);
    Py::None
}
pub fn answer_for_a_thing_they_cannot_use(args: &[Py]) -> Py {
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
    let mut thing: Py = Py::None;
    let _ = py_str("The reply for a thing the person cannot use, or None.\n\n    Asked only where the composer has answered with nothing - the nod that fits any message\n    (\"Thanks for telling me - I'm here if you want to dig into any of that\") - because an answer\n    that adapts to a thing they have not got (\"Skip the cheese: a frittata: whisk 6 eggs...\")\n    already does better than any question this module could ask about it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    thing = the_thing_they_cannot_use(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    return _the_reply_for(&[text.clone(), memory.clone(), thing.clone(), Py::Bool(true)]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.obstacle".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_DOING" => Some(g_A_DOING()),
            "CANNOT_USE" => Some(g_CANNOT_USE()),
            "CANNOT_USE_ASKS" => Some(g_CANNOT_USE_ASKS()),
            "DISLIKE" => Some(g_DISLIKE()),
            "FRAME" => Some(g_FRAME()),
            "FRAME_BREAK" => Some(g_FRAME_BREAK()),
            "FRAME_WORDS" => Some(g_FRAME_WORDS()),
            "LEADS" => Some(g_LEADS()),
            "LEAD_ORDERS" => Some(g_LEAD_ORDERS()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "NOTHING_THING_LEADS" => Some(g_NOTHING_THING_LEADS()),
            "NOTHING_TO_DO_LEADS" => Some(g_NOTHING_TO_DO_LEADS()),
            "NOT_A_LIMIT" => Some(g_NOT_A_LIMIT()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "PREPOSITIONS" => Some(g_PREPOSITIONS()),
            "REFUSAL" => Some(g_REFUSAL()),
            "REFUSAL_LEADS" => Some(g_REFUSAL_LEADS()),
            "REFUSAL_THING_LEADS" => Some(g_REFUSAL_THING_LEADS()),
            "REFUSAL_WORDS" => Some(g_REFUSAL_WORDS()),
            "REFUSED_VERBS" => Some(g_REFUSED_VERBS()),
            "SHORTER" => Some(g_SHORTER()),
            "STEP_ORDERS" => Some(g_STEP_ORDERS()),
            "STEP_WORDS" => Some(g_STEP_WORDS()),
            "STOP" => Some(g_STOP()),
            "THE_NOUN" => Some(g_THE_NOUN()),
            "THE_SAME_MOVE" => Some(g_THE_SAME_MOVE()),
            "UNABLE_VERB" => Some(g_UNABLE_VERB()),
            "_ANY_MOVE" => Some(g__ANY_MOVE()),
            "_MEDICINE" => Some(g__MEDICINE()),
            "_TREATMENT" => Some(g__TREATMENT()),
            "_a_doing" => Some(py_func("_a_doing", |args: &[Py]| _a_doing(args))),
            "_a_list_under_a_frame" => Some(py_func("_a_list_under_a_frame", |args: &[Py]| _a_list_under_a_frame(args))),
            "_a_whole_step" => Some(py_func("_a_whole_step", |args: &[Py]| _a_whole_step(args))),
            "_before_this_turn" => Some(py_func("_before_this_turn", |args: &[Py]| _before_this_turn(args))),
            "_carries_a_move" => Some(py_func("_carries_a_move", |args: &[Py]| _carries_a_move(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_has_family" => Some(py_func("_has_family", |args: &[Py]| _has_family(args))),
            "_leans_on_it" => Some(py_func("_leans_on_it", |args: &[Py]| _leans_on_it(args))),
            "_needs_it" => Some(py_func("_needs_it", |args: &[Py]| _needs_it(args))),
            "_opens_on_a_verb" => Some(py_func("_opens_on_a_verb", |args: &[Py]| _opens_on_a_verb(args))),
            "_steps" => Some(py_func("_steps", |args: &[Py]| _steps(args))),
            "_the_family" => Some(py_func("_the_family", |args: &[Py]| _the_family(args))),
            "_the_frame" => Some(py_func("_the_frame", |args: &[Py]| _the_frame(args))),
            "_the_noun" => Some(py_func("_the_noun", |args: &[Py]| _the_noun(args))),
            "_the_reply_for" => Some(py_func("_the_reply_for", |args: &[Py]| _the_reply_for(args))),
            "_under_its_frame" => Some(py_func("_under_its_frame", |args: &[Py]| _under_its_frame(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "a_move_refused" => Some(py_func("a_move_refused", |args: &[Py]| a_move_refused(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "answer_for_a_thing_they_cannot_use" => Some(py_func("answer_for_a_thing_they_cannot_use", |args: &[Py]| answer_for_a_thing_they_cannot_use(args))),
            "the_move" => Some(py_func("the_move", |args: &[Py]| the_move(args))),
            "the_thing" => Some(py_func("the_thing", |args: &[Py]| the_thing(args))),
            "the_thing_they_cannot_use" => Some(py_func("the_thing_they_cannot_use", |args: &[Py]| the_thing_they_cannot_use(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_DOING" => s_A_DOING(v),
            "CANNOT_USE" => s_CANNOT_USE(v),
            "CANNOT_USE_ASKS" => s_CANNOT_USE_ASKS(v),
            "DISLIKE" => s_DISLIKE(v),
            "FRAME" => s_FRAME(v),
            "FRAME_BREAK" => s_FRAME_BREAK(v),
            "FRAME_WORDS" => s_FRAME_WORDS(v),
            "LEADS" => s_LEADS(v),
            "LEAD_ORDERS" => s_LEAD_ORDERS(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "NOTHING_THING_LEADS" => s_NOTHING_THING_LEADS(v),
            "NOTHING_TO_DO_LEADS" => s_NOTHING_TO_DO_LEADS(v),
            "NOT_A_LIMIT" => s_NOT_A_LIMIT(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "PREPOSITIONS" => s_PREPOSITIONS(v),
            "REFUSAL" => s_REFUSAL(v),
            "REFUSAL_LEADS" => s_REFUSAL_LEADS(v),
            "REFUSAL_THING_LEADS" => s_REFUSAL_THING_LEADS(v),
            "REFUSAL_WORDS" => s_REFUSAL_WORDS(v),
            "REFUSED_VERBS" => s_REFUSED_VERBS(v),
            "SHORTER" => s_SHORTER(v),
            "STEP_ORDERS" => s_STEP_ORDERS(v),
            "STEP_WORDS" => s_STEP_WORDS(v),
            "STOP" => s_STOP(v),
            "THE_NOUN" => s_THE_NOUN(v),
            "THE_SAME_MOVE" => s_THE_SAME_MOVE(v),
            "UNABLE_VERB" => s_UNABLE_VERB(v),
            "_ANY_MOVE" => s__ANY_MOVE(v),
            "_MEDICINE" => s__MEDICINE(v),
            "_TREATMENT" => s__TREATMENT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.obstacle", module_obj);
    pyrt::xrefs::reg_fn("memory.obstacle", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "the_thing", py_func("the_thing", |args: &[Py]| the_thing(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "the_thing_they_cannot_use", py_func("the_thing_they_cannot_use", |args: &[Py]| the_thing_they_cannot_use(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "the_move", py_func("the_move", |args: &[Py]| the_move(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_the_family", py_func("_the_family", |args: &[Py]| _the_family(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_has_family", py_func("_has_family", |args: &[Py]| _has_family(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_carries_a_move", py_func("_carries_a_move", |args: &[Py]| _carries_a_move(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_opens_on_a_verb", py_func("_opens_on_a_verb", |args: &[Py]| _opens_on_a_verb(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_the_noun", py_func("_the_noun", |args: &[Py]| _the_noun(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_a_list_under_a_frame", py_func("_a_list_under_a_frame", |args: &[Py]| _a_list_under_a_frame(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_a_doing", py_func("_a_doing", |args: &[Py]| _a_doing(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_a_whole_step", py_func("_a_whole_step", |args: &[Py]| _a_whole_step(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "a_move_refused", py_func("a_move_refused", |args: &[Py]| a_move_refused(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_under_its_frame", py_func("_under_its_frame", |args: &[Py]| _under_its_frame(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_the_frame", py_func("_the_frame", |args: &[Py]| _the_frame(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_steps", py_func("_steps", |args: &[Py]| _steps(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_needs_it", py_func("_needs_it", |args: &[Py]| _needs_it(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_leans_on_it", py_func("_leans_on_it", |args: &[Py]| _leans_on_it(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_before_this_turn", py_func("_before_this_turn", |args: &[Py]| _before_this_turn(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "_the_reply_for", py_func("_the_reply_for", |args: &[Py]| _the_reply_for(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_fn("memory.obstacle", "answer_for_a_thing_they_cannot_use", py_func("answer_for_a_thing_they_cannot_use", |args: &[Py]| answer_for_a_thing_they_cannot_use(args)));
    pyrt::xrefs::reg_access("memory.obstacle", "A_DOING", g_A_DOING, s_A_DOING);
    pyrt::xrefs::reg_access("memory.obstacle", "CANNOT_USE", g_CANNOT_USE, s_CANNOT_USE);
    pyrt::xrefs::reg_access("memory.obstacle", "CANNOT_USE_ASKS", g_CANNOT_USE_ASKS, s_CANNOT_USE_ASKS);
    pyrt::xrefs::reg_access("memory.obstacle", "DISLIKE", g_DISLIKE, s_DISLIKE);
    pyrt::xrefs::reg_access("memory.obstacle", "FRAME", g_FRAME, s_FRAME);
    pyrt::xrefs::reg_access("memory.obstacle", "FRAME_BREAK", g_FRAME_BREAK, s_FRAME_BREAK);
    pyrt::xrefs::reg_access("memory.obstacle", "FRAME_WORDS", g_FRAME_WORDS, s_FRAME_WORDS);
    pyrt::xrefs::reg_access("memory.obstacle", "LEADS", g_LEADS, s_LEADS);
    pyrt::xrefs::reg_access("memory.obstacle", "LEAD_ORDERS", g_LEAD_ORDERS, s_LEAD_ORDERS);
    pyrt::xrefs::reg_access("memory.obstacle", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("memory.obstacle", "NOTHING_THING_LEADS", g_NOTHING_THING_LEADS, s_NOTHING_THING_LEADS);
    pyrt::xrefs::reg_access("memory.obstacle", "NOTHING_TO_DO_LEADS", g_NOTHING_TO_DO_LEADS, s_NOTHING_TO_DO_LEADS);
    pyrt::xrefs::reg_access("memory.obstacle", "NOT_A_LIMIT", g_NOT_A_LIMIT, s_NOT_A_LIMIT);
    pyrt::xrefs::reg_access("memory.obstacle", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.obstacle", "PREPOSITIONS", g_PREPOSITIONS, s_PREPOSITIONS);
    pyrt::xrefs::reg_access("memory.obstacle", "REFUSAL", g_REFUSAL, s_REFUSAL);
    pyrt::xrefs::reg_access("memory.obstacle", "REFUSAL_LEADS", g_REFUSAL_LEADS, s_REFUSAL_LEADS);
    pyrt::xrefs::reg_access("memory.obstacle", "REFUSAL_THING_LEADS", g_REFUSAL_THING_LEADS, s_REFUSAL_THING_LEADS);
    pyrt::xrefs::reg_access("memory.obstacle", "REFUSAL_WORDS", g_REFUSAL_WORDS, s_REFUSAL_WORDS);
    pyrt::xrefs::reg_access("memory.obstacle", "REFUSED_VERBS", g_REFUSED_VERBS, s_REFUSED_VERBS);
    pyrt::xrefs::reg_access("memory.obstacle", "SHORTER", g_SHORTER, s_SHORTER);
    pyrt::xrefs::reg_access("memory.obstacle", "STEP_ORDERS", g_STEP_ORDERS, s_STEP_ORDERS);
    pyrt::xrefs::reg_access("memory.obstacle", "STEP_WORDS", g_STEP_WORDS, s_STEP_WORDS);
    pyrt::xrefs::reg_access("memory.obstacle", "STOP", g_STOP, s_STOP);
    pyrt::xrefs::reg_access("memory.obstacle", "THE_NOUN", g_THE_NOUN, s_THE_NOUN);
    pyrt::xrefs::reg_access("memory.obstacle", "THE_SAME_MOVE", g_THE_SAME_MOVE, s_THE_SAME_MOVE);
    pyrt::xrefs::reg_access("memory.obstacle", "UNABLE_VERB", g_UNABLE_VERB, s_UNABLE_VERB);
    pyrt::xrefs::reg_access("memory.obstacle", "_ANY_MOVE", g__ANY_MOVE, s__ANY_MOVE);
    pyrt::xrefs::reg_access("memory.obstacle", "_MEDICINE", g__MEDICINE, s__MEDICINE);
    pyrt::xrefs::reg_access("memory.obstacle", "_TREATMENT", g__TREATMENT, s__TREATMENT);
}
