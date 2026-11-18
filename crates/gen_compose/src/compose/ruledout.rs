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


thread_local! { static G_A_ROW_BETWEEN_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ROW_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIM_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIM_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIM_LEADS_IF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIM_OF_FAILURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIM_SMALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KINDS_A_LIST_CAN_SWAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MANNERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_INSTANCES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_KEEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_KEEP_WHOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_NOT_THEN_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_NO_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_NO_THEN_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_OPENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_OPENS_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_PARTICLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_PREP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_QUALITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_REPLIES_A_ROW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_REPLIES_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_REPLIES_THE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_SOFTENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_SYMPATHY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_THEN_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_WHOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RULED_OUT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TITLES_IN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTED_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_ROW_BETWEEN_PEOPLE() -> Py { ensure_init(); G_A_ROW_BETWEEN_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ROW_BETWEEN_PEOPLE(v: Py) { ensure_init(); G_A_ROW_BETWEEN_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ROW_NAMES() -> Py { ensure_init(); G_A_ROW_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ROW_NAMES(v: Py) { ensure_init(); G_A_ROW_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIM_CONDITION() -> Py { ensure_init(); G_CLAIM_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIM_CONDITION(v: Py) { ensure_init(); G_CLAIM_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIM_LEADS() -> Py { ensure_init(); G_CLAIM_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIM_LEADS(v: Py) { ensure_init(); G_CLAIM_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIM_LEADS_IF() -> Py { ensure_init(); G_CLAIM_LEADS_IF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIM_LEADS_IF(v: Py) { ensure_init(); G_CLAIM_LEADS_IF.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIM_OF_FAILURE() -> Py { ensure_init(); G_CLAIM_OF_FAILURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIM_OF_FAILURE(v: Py) { ensure_init(); G_CLAIM_OF_FAILURE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIM_SMALL() -> Py { ensure_init(); G_CLAIM_SMALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIM_SMALL(v: Py) { ensure_init(); G_CLAIM_SMALL.with(|c| *c.borrow_mut() = v); }
pub fn g_KINDS_A_LIST_CAN_SWAP() -> Py { ensure_init(); G_KINDS_A_LIST_CAN_SWAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KINDS_A_LIST_CAN_SWAP(v: Py) { ensure_init(); G_KINDS_A_LIST_CAN_SWAP.with(|c| *c.borrow_mut() = v); }
pub fn g_MANNERS() -> Py { ensure_init(); G_MANNERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MANNERS(v: Py) { ensure_init(); G_MANNERS.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_NAMED() -> Py { ensure_init(); G_PET_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_NAMED(v: Py) { ensure_init(); G_PET_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT() -> Py { ensure_init(); G_RULED_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT(v: Py) { ensure_init(); G_RULED_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_CLASS() -> Py { ensure_init(); G_RULED_OUT_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_CLASS(v: Py) { ensure_init(); G_RULED_OUT_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_INSTANCES() -> Py { ensure_init(); G_RULED_OUT_INSTANCES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_INSTANCES(v: Py) { ensure_init(); G_RULED_OUT_INSTANCES.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_KEEPS() -> Py { ensure_init(); G_RULED_OUT_KEEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_KEEPS(v: Py) { ensure_init(); G_RULED_OUT_KEEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_KEEP_WHOLE() -> Py { ensure_init(); G_RULED_OUT_KEEP_WHOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_KEEP_WHOLE(v: Py) { ensure_init(); G_RULED_OUT_KEEP_WHOLE.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_NAMED() -> Py { ensure_init(); G_RULED_OUT_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_NAMED(v: Py) { ensure_init(); G_RULED_OUT_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_NOT_A_THING() -> Py { ensure_init(); G_RULED_OUT_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_NOT_A_THING(v: Py) { ensure_init(); G_RULED_OUT_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_NOT_THEN_ELSE() -> Py { ensure_init(); G_RULED_OUT_NOT_THEN_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_NOT_THEN_ELSE(v: Py) { ensure_init(); G_RULED_OUT_NOT_THEN_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_NO_ANSWER() -> Py { ensure_init(); G_RULED_OUT_NO_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_NO_ANSWER(v: Py) { ensure_init(); G_RULED_OUT_NO_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_NO_THEN_ELSE() -> Py { ensure_init(); G_RULED_OUT_NO_THEN_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_NO_THEN_ELSE(v: Py) { ensure_init(); G_RULED_OUT_NO_THEN_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_OPENING() -> Py { ensure_init(); G_RULED_OUT_OPENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_OPENING(v: Py) { ensure_init(); G_RULED_OUT_OPENING.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_OPENS_WITH() -> Py { ensure_init(); G_RULED_OUT_OPENS_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_OPENS_WITH(v: Py) { ensure_init(); G_RULED_OUT_OPENS_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_PARTICLES() -> Py { ensure_init(); G_RULED_OUT_PARTICLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_PARTICLES(v: Py) { ensure_init(); G_RULED_OUT_PARTICLES.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_PREP() -> Py { ensure_init(); G_RULED_OUT_PREP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_PREP(v: Py) { ensure_init(); G_RULED_OUT_PREP.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_QUALITY() -> Py { ensure_init(); G_RULED_OUT_QUALITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_QUALITY(v: Py) { ensure_init(); G_RULED_OUT_QUALITY.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_REPLIES_A_ROW() -> Py { ensure_init(); G_RULED_OUT_REPLIES_A_ROW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_REPLIES_A_ROW(v: Py) { ensure_init(); G_RULED_OUT_REPLIES_A_ROW.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_REPLIES_PLAIN() -> Py { ensure_init(); G_RULED_OUT_REPLIES_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_REPLIES_PLAIN(v: Py) { ensure_init(); G_RULED_OUT_REPLIES_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_REPLIES_THE() -> Py { ensure_init(); G_RULED_OUT_REPLIES_THE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_REPLIES_THE(v: Py) { ensure_init(); G_RULED_OUT_REPLIES_THE.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_SOFTENING() -> Py { ensure_init(); G_RULED_OUT_SOFTENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_SOFTENING(v: Py) { ensure_init(); G_RULED_OUT_SOFTENING.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_SYMPATHY() -> Py { ensure_init(); G_RULED_OUT_SYMPATHY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_SYMPATHY(v: Py) { ensure_init(); G_RULED_OUT_SYMPATHY.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_TAIL() -> Py { ensure_init(); G_RULED_OUT_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_TAIL(v: Py) { ensure_init(); G_RULED_OUT_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_THEN_ELSE() -> Py { ensure_init(); G_RULED_OUT_THEN_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_THEN_ELSE(v: Py) { ensure_init(); G_RULED_OUT_THEN_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_WHOLE() -> Py { ensure_init(); G_RULED_OUT_WHOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_WHOLE(v: Py) { ensure_init(); G_RULED_OUT_WHOLE.with(|c| *c.borrow_mut() = v); }
pub fn g_RULED_OUT_WORDS() -> Py { ensure_init(); G_RULED_OUT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RULED_OUT_WORDS(v: Py) { ensure_init(); G_RULED_OUT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_ELSEWHERE() -> Py { ensure_init(); G_STEP_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_ELSEWHERE(v: Py) { ensure_init(); G_STEP_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_TITLES_IN_ANSWER() -> Py { ensure_init(); G_TITLES_IN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TITLES_IN_ANSWER(v: Py) { ensure_init(); G_TITLES_IN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTED_ONE() -> Py { ensure_init(); G_WANTED_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTED_ONE(v: Py) { ensure_init(); G_WANTED_ONE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing the person rules out, answered against the answer they just got.\n\n\"not a series\" after three films, \"i dont like the gym\" after an answer about half a plate of\nvegetables, \"not tablets\" after a sleep answer. The person is steering, and what they ruled out was\nnever in the answer. What those lines got on main was the flat echo back (\"Got it - not the\nchemical stuff\"), \"Thanks for telling me - I'm here if you want to dig into any of that.\", or a\nwrong entry entirely: \"not a diesel\" reached the misfuelling card and \"not tablets\" the repeat\nprescription one.\n\nThe reply says the thing was never in the answer and hands back the stretch of it that still\nstands, so the person can go on from there without reading the paragraph again.\n\nThis module only reads: the choosing helper and the shrug list it avoids live in\n`bot/compose/fallback.py` and are used here as they are, with no change to them.\n");
    s_RULED_OUT(py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?not\\s+(?P<thing>(?:a|an|any|the)\\s+[a-z][a-z' ]{0,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(true)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?not\\s+(?P<thing>[a-z][a-z' ]{0,20}s)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?not\\s+(?P<thing>[a-z][a-z' ]{0,20}\\s+related)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?not\\s+(?:too|much)\\s+(?P<thing>[a-z][a-z' ]{1,20})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?nothing\\s+(?P<thing>[a-z][a-z' ]{1,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?(?:i|we)\\s+(?:really\\s+|just\\s+|actually\\s+|personally\\s+)?(?:dont|don't|do\\s+not|didnt|did\\s+not|never)\\s+want\\s+(?P<thing>trouble|hassle|aggro|drama|upset|bother)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?(?:i|we)\\s+(?:really\\s+|just\\s+|actually\\s+|personally\\s+)?(?:dont|don't|do\\s+not|didnt|did\\s+not|never)\\s+want\\s+to\\s+(?P<thing>(?:fall\\s+out|falling\\s+out|argue|fight|quarrel|kick\\s+up\\s+a\\s+fuss|make\\s+a\\s+fuss|get\\s+into\\s+it))$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?(?:i|we)\\s+(?:really\\s+|just\\s+|actually\\s+|personally\\s+)?(?:dont|don't|do\\s+not|didnt|did\\s+not|never)\\s+(?:like|want|have|own|use|need|rate|fancy)\\s+(?P<thing>(?:a|an|any|the|my|another)\\s+[a-z][a-z' ]{0,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(true)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+|no,?\\s+|nah,?\\s+)?(?:it|that|this)(?:'s|s)?\\s+(?:is\\s+not|isnt|isn't|not)\\s+(?P<thing>(?:a|an|any|the|my|another)\\s+[a-z][a-z' ]{0,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(true)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+|no,?\\s+|nah,?\\s+)?(?:it|that|this)(?:'s|s)?\\s+(?:is\\s+not|isnt|isn't|not)\\s+(?P<thing>[a-z][a-z' ]{1,20}s)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)])]));
    s_RULED_OUT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("thing"), py_str("things"), py_str("anything"), py_str("something"), py_str("that"), py_str("this"), py_str("which"), py_str("what"), py_str("any"), py_str("more"), py_str("much"), py_str("needs"), py_str("need"), py_str("uses"), py_str("use"), py_str("takes"), py_str("take"), py_str("gets"), py_str("get"), py_str("has"), py_str("have"), py_str("requires"), py_str("require"), py_str("wants"), py_str("want"), py_str("please"), py_str("thanks"), py_str("really"), py_str("very")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_QUALITY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("bad"), py_str("good"), py_str("sad"), py_str("happy"), py_str("awful"), py_str("terrible"), py_str("horrible"), py_str("weird"), py_str("wrong"), py_str("fine"), py_str("ok"), py_str("okay"), py_str("sure"), py_str("nice"), py_str("lovely"), py_str("great"), py_str("better"), py_str("worse"), py_str("best"), py_str("worst"), py_str("big"), py_str("small"), py_str("little"), py_str("long"), py_str("short"), py_str("new"), py_str("old"), py_str("easy"), py_str("hard"), py_str("difficult"), py_str("expensive"), py_str("cheap"), py_str("different"), py_str("same"), py_str("busy"), py_str("tired"), py_str("stressed"), py_str("ill"), py_str("sick")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_NOT_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("case"), py_str("point"), py_str("problem"), py_str("issue"), py_str("fact"), py_str("joke"), py_str("idea"), py_str("whole"), py_str("kind"), py_str("sort"), py_str("type"), py_str("way"), py_str("matter"), py_str("question"), py_str("answer"), py_str("reason"), py_str("truth"), py_str("deal"), py_str("end"), py_str("yet"), py_str("either"), py_str("though"), py_str("well"), py_str("thanks"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been"), py_str("going"), py_str("happening"), py_str("happened"), py_str("here"), py_str("there"), py_str("now"), py_str("ever"), py_str("again"), py_str("else"), py_str("right"), py_str("left"), py_str("much"), py_str("more"), py_str("less"), py_str("least"), py_str("still"), py_str("even"), py_str("already"), py_str("quite"), py_str("rather"), py_str("able"), py_str("possible"), py_str("worth"), py_str("enough"), py_str("true"), py_str("false"), py_str("sure"), py_str("wrong"), py_str("fine")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:related|based|ish|esque)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_PREP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:of|to|on|for|by|from|about)\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_OPENS_WITH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:with|without|in|on|at|for|about|of|to)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_SOFTENING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?nothing\\s+(?:too|very|that|which|of)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_KEEPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|any|the|my|another)\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_REPLIES_PLAIN(py_tuple(vec![py_str("There's no {thing} in that answer, so it still stands. {opening}"), py_str("That one never mentions {thing} at all, so nothing changes. {opening}"), py_str("Nothing there was about {thing}, so the answer holds. {opening}")]));
    s_RULED_OUT_REPLIES_THE(py_tuple(vec![py_str("None of that was {thing} - the answer still stands. {opening}"), py_str("That one never mentions {thing} at all, so nothing changes. {opening}"), py_str("Nothing there was {thing}, so the answer holds. {opening}")]));
    s_A_ROW_BETWEEN_PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("row"), py_str("fuss"), py_str("scene"), py_str("argument"), py_str("argy bargy"), py_str("argie bargie"), py_str("fight"), py_str("drama"), py_str("falling out"), py_str("fall out"), py_str("confrontation"), py_str("trouble"), py_str("hassle"), py_str("aggro"), py_str("upset"), py_str("bad feeling"), py_str("bad blood"), py_str("stink"), py_str("carry on"), py_str("atmosphere")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_ROW_NAMES(py_dict(vec![(py_str("fall out"), py_str("falling out")), (py_str("falling out"), py_str("falling out")), (py_str("argue"), py_str("argument")), (py_str("quarrel"), py_str("argument")), (py_str("get into it"), py_str("argument")), (py_str("fight"), py_str("fight")), (py_str("kick up a fuss"), py_str("fuss")), (py_str("make a fuss"), py_str("fuss")), (py_str("row"), py_str("row")), (py_str("trouble"), py_str("trouble")), (py_str("hassle"), py_str("hassle")), (py_str("bother"), py_str("bother")), (py_str("aggro"), py_str("aggro")), (py_str("drama"), py_str("drama")), (py_str("upset"), py_str("upset"))]));
    s_RULED_OUT_REPLIES_A_ROW(py_tuple(vec![py_str("Fair enough - no {thing} needed. {opening}"), py_str("Quite right, no {thing} wanted. {opening}"), py_str("Fair enough: one calm sentence, no {thing}. {opening}")]));
    s_RULED_OUT_CLASS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("tablets"), py_str("pills"), py_str("painkillers"), py_str("painkiller"), py_str("medication"), py_str("medicine"), py_str("medicines"), py_str("drugs"), py_str("antibiotics"), py_str("supplements"), py_str("vitamins"), py_str("inhalers"), py_str("creams"), py_str("series"), py_str("episodes"), py_str("thriller"), py_str("thrillers"), py_str("horror"), py_str("comedies"), py_str("documentaries"), py_str("meat"), py_str("dairy"), py_str("gluten"), py_str("carbs"), py_str("carbohydrates"), py_str("sugar"), py_str("chemicals"), py_str("screens"), py_str("toys")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_INSTANCES(py_dict(vec![(py_str("tablets"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("aspirin"), py_str("codeine"), py_str("antihistamine"), py_str("melatonin"), py_str("sertraline"), py_str("tablet"), py_str("pill"), py_str("painkiller"), py_str("dose")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("pills"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("aspirin"), py_str("codeine"), py_str("antihistamine"), py_str("melatonin"), py_str("pill"), py_str("tablet"), py_str("painkiller")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("painkillers"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("aspirin"), py_str("codeine"), py_str("painkiller")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("painkiller"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("aspirin"), py_str("codeine"), py_str("painkiller")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("medication"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("tablet"), py_str("pill"), py_str("dose"), py_str("prescription")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("medicine"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("tablet"), py_str("pill"), py_str("dose")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("medicines"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("tablet"), py_str("pill"), py_str("dose")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("drugs"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("paracetamol"), py_str("ibuprofen"), py_str("antibiotic"), py_str("painkiller"), py_str("tablet")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("antibiotics"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("antibiotic"), py_str("penicillin"), py_str("amoxicillin")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("supplements"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("supplement"), py_str("vitamin"), py_str("iron"), py_str("magnesium"), py_str("omega")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("vitamins"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("vitamin"), py_str("supplement"), py_str("iron"), py_str("magnesium"), py_str("omega")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("creams"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("cream"), py_str("ointment"), py_str("lotion"), py_str("gel")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("series"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("series"), py_str("episode"), py_str("episodes"), py_str("season"), py_str("seasons"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("episodes"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("episode"), py_str("series"), py_str("season"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("thriller"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("thriller"), py_str("murder"), py_str("mystery"), py_str("crime"), py_str("detective"), py_str("suspense"), py_str("serial"), py_str("killer")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("thrillers"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("thriller"), py_str("murder"), py_str("mystery"), py_str("crime"), py_str("detective"), py_str("suspense"), py_str("serial"), py_str("killer")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("horror"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("horror"), py_str("scary"), py_str("ghost"), py_str("haunted"), py_str("frightening")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("meat"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("meat"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("bacon"), py_str("sausage"), py_str("mince"), py_str("steak"), py_str("ham"), py_str("turkey")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("dairy"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("dairy"), py_str("milk"), py_str("cheese"), py_str("butter"), py_str("cream"), py_str("yoghurt"), py_str("yogurt")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("gluten"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("gluten"), py_str("bread"), py_str("pasta"), py_str("flour"), py_str("wheat"), py_str("couscous"), py_str("barley")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("carbs"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("rice"), py_str("pasta"), py_str("bread"), py_str("potato"), py_str("potatoes"), py_str("flour")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("carbohydrates"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("rice"), py_str("pasta"), py_str("bread"), py_str("potato"), py_str("potatoes"), py_str("flour")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("sugar"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("sugar"), py_str("honey"), py_str("syrup"), py_str("sweetener"), py_str("jam")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("chemicals"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("vinegar"), py_str("bleach"), py_str("ammonia"), py_str("caustic"), py_str("solvent"), py_str("spray"), py_str("bicarbonate")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("screens"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("screen"), py_str("phone"), py_str("television"), py_str("tv"), py_str("laptop"), py_str("monitor"), py_str("tablet")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("toys"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("toy"), py_str("lego"), py_str("blocks"), py_str("play")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s_RULED_OUT_NAMED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\*[^*]{2,}\\*|\\\"[^\\\"]{2,}\\\"|\\u201c[^\\u201d]{2,}\\u201d")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_KEEP_WHOLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:999|911|112|111|ambulance|a&e|casualty|emergency|urgent care|999 line)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_SYMPATHY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:sorry|sad news|passed away|grief|grieving|bereave\\w*|condolence\\w*|miss (?:him|her|them))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_OPENING(Py::Int(120));
    s_RULED_OUT_WHOLE(Py::Int(210));
    s_MANNERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:[\\s,!.?]*\\b(?:pls+|plz|please|pleas|plse|thx|thanx|thanks|thankyou|thank\\s+you|ta|ty|cheers|cheerio|mate|m8|fella|buddy|bud|pal|x|xx|xxx)\\b)+[\\s,!.?]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_THEN_ELSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\s,;:-]+(?:then\\s+|and\\s+)?(?:so\\s+)?(?:what|anything|something)\\s+else(?:\\s+[a-z']+){0,4}\\s*[.?!]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_NOT_THEN_ELSE(py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?not\\s+(?P<thing>(?:a|an|any|the)\\s+[a-z][a-z' ]{0,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(true)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+|but\\s+)?not\\s+(?P<thing>[a-z][a-z' ]{1,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)])]));
    s_RULED_OUT_NO_THEN_ELSE(py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:no|nah|nope|no,|nah,|nope,)\\s+(?P<thing>(?:a|an|any|the)\\s+[a-z][a-z' ]{0,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(true)]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:no|nah|nope|no,|nah,|nope,)\\s+(?P<thing>[a-z][a-z' ]{1,24})$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, Py::Bool(false)])]));
    s_RULED_OUT_PARTICLES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("out off up down away back over through around about along\ntogether home here there it self selves"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RULED_OUT_NO_ANSWER(py_tuple(vec![py_str("sorry"), py_str("that sounds"), py_str("sounds like"), py_str("i'm listening"), py_str("i am listening"), py_str("got it"), py_str("thanks for telling"), py_str("no worries"), py_str("of course"), py_str("noted"), py_str("fair enough"), py_str("that's rough"), py_str("thats rough")]));
    s_CLAIM_OF_FAILURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:ok|okay|so|well|yeah|yep|yes|and|but|oh|right|alright|no|nope|nah|still|now)\\W+)*(?:nothing|nowt|none\\s+of\\s+(?:it|that|them|these|those)|neither\\s+of\\s+(?:them|those|it))(?:\\s+\\w+){0,5}\\W*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAIM_SMALL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("at all so far yet again now still really just about on with me my it\nthis that them those these any more much either the a an of in for to day days week weeks moment\nwhile times time one bit lot lots out up down been has have had is was are were am do does did"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAIM_LEADS(py_tuple(vec![py_str("Then it is the next part that matters: {step}"), py_str("Then take it a step further: {step}"), py_str("Then the next step: {step}"), py_str("Next: {step}")]));
    s_CLAIM_CONDITION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:if|when|once|unless|before|after|until)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STEP_ELSEWHERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gp|doctor|nurse|pharmacist|chemist|dentist|vet|optician|midwife|health visitor|111|999|911|helpline|a&e|hospital|council|citizens advice|plumber|electrician|engineer|gas safe|mechanic|locksmith|roofer|builder|technician|specialist|repair shop)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAIM_LEADS_IF(py_tuple(vec![py_str("The next thing to try: {step}"), py_str("Next: {step}"), py_str("Then the next check: {step}")]));
    s_PET_NAMED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:dog|dogs|puppy|puppies|pup|pups|cat|cats|kitten|kittens|rabbit|rabbits|bunny|bunnies|hamster|hamsters|gerbil|gerbils|guinea pig|guinea pigs|parrot|parrots|budgie|budgies|bird|birds|horse|horses|pony|ponies|fish|goldfish|tortoise|tortoises|snake|snakes|lizard|lizards|ferret|ferrets|pet|pets)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WANTED_ONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:want|wants|wanted|would like|'d like|after|thinking (?:of|about)|considering|planning to get|should (?:i|we) get|can (?:i|we) get|shall we get|get (?:a|an)|getting (?:a|an)|looking at getting)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TITLES_IN_ANSWER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\*[^*]{2,60}\\*")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_KINDS_A_LIST_CAN_SWAP(py_tuple(vec![py_str("horror"), py_str("scary"), py_str("frightening"), py_str("creepy"), py_str("comedy"), py_str("comedies"), py_str("funny"), py_str("romance"), py_str("romantic"), py_str("sad"), py_str("documentary"), py_str("gory")]));
}
pub fn _named_picks(args: &[Py]) -> Py {
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
    let _ = py_str("True when the answer offers two or more named things rather than describing them.");
    return op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_NAMED(), "findall", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1));
    Py::None
}
pub fn _class_of(args: &[Py]) -> Py {
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
    let mut classes: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut key: Py = Py::None;
    let mut members: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The classes a ruled-out word belongs to, read from the word itself.\n\n    One spelling is not one class. \"not the chemical stuff\" is the same class as \"not chemicals\"\n    with a different ending, and \"not a film\" is a member of the class the watching answer offers\n    as \"series\"/\"episodes\" - the answer's own suggestions are the instances, so the line names one\n    of them rather than the class.\n    ");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    forms = py_set(vec![word.clone(), op_add(&word.clone(), &py_str("s"))]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("es")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&forms.clone(), "add", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&forms.clone(), "add", args))(&__args) };
    }
    classes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_RULED_OUT_CLASS(), &form.clone()))) {  __out.push(form.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&classes.clone()))) {
    stem = slice_or_raise(&word.clone(), &Py::None, &Py::Int(7), &Py::None);
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stem.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6))) {
    classes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RULED_OUT_CLASS()) { let mut key: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stem.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&key.clone(), "startswith", args))(&__args) }) {  __out.push(key.clone()); } } py_set(__out) })() };
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&classes.clone()))) {
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_INSTANCES(), "items", args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    members = __unpk2[1].clone();
    if py_truthy(&op_bitand(&forms.clone(), &members.clone())) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&classes.clone(), "add", args))(&__args) };
    }
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(classes.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_class_word(args: &[Py]) -> Py {
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
    let mut forms: Py = Py::None;
    let mut key: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("True when the word is one of the class names itself, in any spelling.");
    forms = py_set(vec![word.clone(), op_add(&word.clone(), &py_str("s"))]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("es")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&forms.clone(), "add", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&forms.clone(), "add", args))(&__args) };
    }
    if py_truthy(&op_bitand(&forms.clone(), &g_RULED_OUT_CLASS())) {
    return Py::Bool(true);
    }
    stem = slice_or_raise(&word.clone(), &Py::None, &Py::Int(7), &Py::None);
    return { let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stem.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RULED_OUT_CLASS()) { let mut key: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stem.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&key.clone(), "startswith", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _an_answer_of_that_class(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut classes: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_word: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the answer names the ruled-out class or one of its members.\n\n    \"not tablets\" against an answer that says paracetamol, \"not a thriller\" against a murder\n    mystery: the answer's own suggestion is an instance of what was ruled out, and saying \"that\n    one never mentions tablets at all\" while handing two of them back reads as not listening.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_KEEPS(), "sub", args))(&__args) }, "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    classes = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _class_of(args))(&__args) };
    let __iter2 = py_iter_or_raise(&classes.clone());
    for __item in __iter2 {
    key = __item.clone();
    forms = py_set(vec![key.clone(), op_add(&key.clone(), &py_str("s"))]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&key.clone(), "endswith", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&key.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&forms.clone(), "add", args))(&__args) };
    }
    if py_truthy(&op_bitand(&forms.clone(), &said.clone())) {
    return Py::Bool(true);
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_INSTANCES(), "get", args))(&__args) }, &said.clone())) {
    return Py::Bool(true);
    }
    stem = slice_or_raise(&key.clone(), &Py::None, &Py::Int(7), &Py::None);
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stem.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut said_word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stem.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_word.clone(), "startswith", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = classes.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_class_word(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named_picks(args))(&__args) } } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _without_the_manners(args: &[Py]) -> Py {
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
    let _ = py_str("The line with the politeness it ends on taken off.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MANNERS(), "sub", args))(&__args) }, "strip", args))(&__args) };
    Py::None
}
pub fn _ruled_out_thing(args: &[Py]) -> Py {
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
    let mut asked_for_the_other: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut patterns: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut with_determiner: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing a line rules out, in the person's own words, and whether it kept its \"a\"/\"the\".");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_the_manners(args))(&__args) };
    asked_for_the_other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_THEN_ELSE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&asked_for_the_other.clone()) {
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_THEN_ELSE(), "sub", args))(&__args) }, "strip", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(7)) } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_SOFTENING(), "match", args))(&__args) }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_str(""), Py::Bool(false)]), 2, None);
    thing = __unpk1[0].clone();
    kept = __unpk1[1].clone();
    patterns = op_add(&g_RULED_OUT(), &if py_truthy(&asked_for_the_other.clone()) { op_add(&g_RULED_OUT_NOT_THEN_ELSE(), &g_RULED_OUT_NO_THEN_ELSE()) } else { py_tuple(vec![]) });
    let __iter2 = py_iter_or_raise(&patterns.clone());
    let mut __else3 = false;
    for __item in __iter2 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk4[0].clone();
    with_determiner = __unpk4[1].clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "match", args))(&__args) };
    if py_truthy(&found.clone()) {
    let __unpk5 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, with_determiner.clone()]), 2, None);
    thing = __unpk5[0].clone();
    kept = __unpk5[1].clone();
    break;
    }
    }
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_RULED_OUT_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?'\"")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_TAIL(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_PREP(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_OPENS_WITH(), "search", args))(&__args) } } } } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_RULED_OUT_QUALITY(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_RULED_OUT_NOT_A_THING(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nothing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "startswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::kinds::names_a_claim(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    kept = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_KEEPS(), "match", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "isalpha", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    return py_tuple(vec![thing.clone(), kept.clone()]);
    Py::None
}
pub fn _a_row_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut bare: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("The row in a ruled-out thing, or \"\" - trouble between people is how, not what.");
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_KEEPS(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g_A_ROW_NAMES(), &bare.clone()))) {
    return geti(&g_A_ROW_NAMES(), &bare.clone());
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_ROW_NAMES()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&bare.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&phrase.clone(), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "startswith", args))(&__args) } } }) {
    return geti(&g_A_ROW_NAMES(), &phrase.clone());
    }
    }
    return py_str("");
    Py::None
}
pub fn _names_it(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut carrying: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when an answer already names the thing, in any form it might be written in.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_KEEPS(), "sub", args))(&__args) }, "split", args))(&__args) };
    carrying = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_RULED_OUT_PARTICLES(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&carrying.clone()) {
    words = carrying.clone();
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    forms = py_set(vec![word.clone(), op_add(&word.clone(), &py_str("s")), op_add(&word.clone(), &py_str("es")), op_add(&word.clone(), &py_str("ing")), op_add(&word.clone(), &py_str("ed"))]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&forms.clone(), "add", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("es")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&forms.clone(), "add", args))(&__args) };
    }
    if py_truthy(&op_bitand(&forms.clone(), &said.clone())) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_whole_answer(args: &[Py]) -> Py {
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
    let mut punt: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's last line when it was an answer, not a nod, a question back or a shrug.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(60)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "endswith", args))(&__args) } } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::compose::fallback::g_PUNT()) { let mut punt: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "lower", args))(&__args) }, &punt.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_RULED_OUT_NO_ANSWER()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "lower", args))(&__args) }, "startswith", args))(&__args) }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_KEEP_WHOLE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_SYMPATHY(), "search", args))(&__args) } } }) {
    return py_str("");
    }
    return last.clone();
    Py::None
}
pub fn _the_standing_part(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut first: Py = Py::None;
    let mut head: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut stretch: Py = Py::None;
    let mut stretches: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The stretch of an answer worth handing back: its first step, not the whole paragraph.\n\n    A short answer is handed back whole, because in a short answer every sentence is doing work\n    (\"and don't drive yourself\" is the second one). A longer one keeps its first step: the\n    lead-in is dropped (\"If you want something easy and good tonight: *Groundhog Day*...\"),\n    because a lead-in hands nothing back, and then the first clause long enough to carry a step.\n    \"\" when there is nothing short enough to be a reminder.\n    ");
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_RULED_OUT_WHOLE())) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    first = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "partition", args))(&__args) }, 3, None);
    head = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    tail = __unpk1[2].clone();
    if py_truthy(&{ let __b = tail.clone(); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&head.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(8)) } }) {
    first = tail.clone();
    }
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_RULED_OUT_OPENING())) {
    return first.clone();
    }
    cut = slice_or_raise(&first.clone(), &Py::None, &g_RULED_OUT_OPENING(), &Py::None);
    stretches = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("; "), py_str(", "), py_str(" - "), py_str(" and ")])) { let mut mark: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&cut.clone(), &mark.clone()))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&cut.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mark.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "rfind", args))(&__args) }, &Py::None), "strip", args))(&__args) }); } } py_list(__out) })() };
    stretches = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stretches.clone()) { let mut stretch: Py = __item.clone(); if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&stretch.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4))) {  __out.push(stretch.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&stretches.clone()))) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stretches.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_claim_of_failure(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut conditional: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut step: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The next part of the answer, for a line that says the answer's steps did not take.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAIM_OF_FAILURE(), "match", args))(&__args) }))) {
    return py_str("");
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("nothing"), py_str("nowt"), py_str("none"), py_str("neither"), py_str("of"), py_str("ok"), py_str("okay"), py_str("so"), py_str("well"), py_str("yeah"), py_str("yep"), py_str("yes"), py_str("and"), py_str("but"), py_str("oh"), py_str("right"), py_str("alright"), py_str("no"), py_str("nope"), py_str("nah"), py_str("still"), py_str("now")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "pop", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::kinds::names_a_claim(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(!py_contains(&g_CLAIM_SMALL(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::kinds::names_a_claim(args))(&__args) })) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("");
    }
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_whole_answer(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "rstrip", args))(&__args) }, "endswith", args))(&__args) }) {
    return py_str("");
    }
    let __try1 = exc::catch(|| {
    let mut _later_part: Py = py_func("_later_part", |args: &[Py]| gen_memory::memory::repeat::_later_part(args));
    step = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&_later_part, args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&step.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&step.clone())) } }) {
    return py_str("");
    }
    conditional = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(step.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAIM_CONDITION(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&conditional.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(step.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STEP_ELSEWHERE(), "search", args))(&__args) })) } }) {
    return py_str("");
    }
    step = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    pool = if py_truthy(&conditional.clone()) { g_CLAIM_LEADS_IF() } else { g_CLAIM_LEADS() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    Py::None
}
pub fn _a_pet_turned_down(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the line turns down an animal the line above asked about getting one of.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PET_NAMED(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&before.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PET_NAMED(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WANTED_ONE(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_list_that_can_be_swapped(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the answer is a list of picks and the thing ruled out is the kind in it.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_KEEPS(), "sub", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_KINDS_A_LIST_CAN_SWAP(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    return op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TITLES_IN_ANSWER(), "findall", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2));
    Py::None
}
pub fn _rules_out_what_i_never_said(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing ruled out, whether it kept its \"a\"/\"the\", and the part that still stands.");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ruled_out_thing(args))(&__args) }, 2, None);
    thing = __unpk1[0].clone();
    kept = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_pet_turned_down(args))(&__args) }) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_whole_answer(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_it(args))(&__args) } } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_list_that_can_be_swapped(args))(&__args) }) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_answer_of_that_class(args))(&__args) }) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    opening = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_standing_part(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&opening.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(opening.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_it(args))(&__args) } } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false), py_str("")]);
    }
    return py_tuple(vec![if py_truthy(&kept.clone()) { thing.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RULED_OUT_KEEPS(), "sub", args))(&__args) } }, kept.clone(), opening.clone()]);
    Py::None
}
pub fn _ruled_out_reply(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut kept: Py = Py::None;
    let mut line: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut row: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("One line saying the thing just ruled out was never in the answer, and what still stands.");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _rules_out_what_i_never_said(args))(&__args) }, 3, None);
    thing = __unpk1[0].clone();
    kept = __unpk1[1].clone();
    opening = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return py_str("");
    }
    opening = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&opening.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&opening.clone(), &Py::Int(1), &Py::None, &Py::None));
    row = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_row_thing(args))(&__args) };
    if py_truthy(&row.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RULED_OUT_REPLIES_A_ROW()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), row.clone()), ("opening".to_string(), opening.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    }
    pool = if py_truthy(&kept.clone()) { g_RULED_OUT_REPLIES_THE() } else { g_RULED_OUT_REPLIES_PLAIN() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1))), "endswith", args))(&__args) } } }) {
    pool = slice_or_raise(&g_RULED_OUT_REPLIES_PLAIN(), &Py::Int(1), &Py::None, &Py::None);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("opening".to_string(), opening.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
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
    let mut claim: Py = Py::None;
    let mut clean: Py = Py::None;
    let _ = py_str("The rule-out answered against the answer above, or \"\" when this is not that shape.");
    let __try1 = exc::catch(|| {
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    claim = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_claim_of_failure(args))(&__args) };
    if py_truthy(&claim.clone()) {
    return py_tuple(vec![Py::Bool(true), claim.clone()]);
    }
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ruled_out_reply(args))(&__args) }]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
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
    let mut _kept: Py = Py::None;
    let mut _opening: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("Does this line steer the answer it followed? A hook is asked before the reply is made.");
    let __try1 = exc::catch(|| {
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_claim_of_failure(args))(&__args) }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    let __unpk6 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _rules_out_what_i_never_said(args))(&__args) }, 3, None);
    thing = __unpk6[0].clone();
    _kept = __unpk6[1].clone();
    _opening = __unpk6[2].clone();
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
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
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.ruledout".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_ROW_BETWEEN_PEOPLE" => Some(g_A_ROW_BETWEEN_PEOPLE()),
            "A_ROW_NAMES" => Some(g_A_ROW_NAMES()),
            "CLAIM_CONDITION" => Some(g_CLAIM_CONDITION()),
            "CLAIM_LEADS" => Some(g_CLAIM_LEADS()),
            "CLAIM_LEADS_IF" => Some(g_CLAIM_LEADS_IF()),
            "CLAIM_OF_FAILURE" => Some(g_CLAIM_OF_FAILURE()),
            "CLAIM_SMALL" => Some(g_CLAIM_SMALL()),
            "KINDS_A_LIST_CAN_SWAP" => Some(g_KINDS_A_LIST_CAN_SWAP()),
            "MANNERS" => Some(g_MANNERS()),
            "PET_NAMED" => Some(g_PET_NAMED()),
            "RULED_OUT" => Some(g_RULED_OUT()),
            "RULED_OUT_CLASS" => Some(g_RULED_OUT_CLASS()),
            "RULED_OUT_INSTANCES" => Some(g_RULED_OUT_INSTANCES()),
            "RULED_OUT_KEEPS" => Some(g_RULED_OUT_KEEPS()),
            "RULED_OUT_KEEP_WHOLE" => Some(g_RULED_OUT_KEEP_WHOLE()),
            "RULED_OUT_NAMED" => Some(g_RULED_OUT_NAMED()),
            "RULED_OUT_NOT_A_THING" => Some(g_RULED_OUT_NOT_A_THING()),
            "RULED_OUT_NOT_THEN_ELSE" => Some(g_RULED_OUT_NOT_THEN_ELSE()),
            "RULED_OUT_NO_ANSWER" => Some(g_RULED_OUT_NO_ANSWER()),
            "RULED_OUT_NO_THEN_ELSE" => Some(g_RULED_OUT_NO_THEN_ELSE()),
            "RULED_OUT_OPENING" => Some(g_RULED_OUT_OPENING()),
            "RULED_OUT_OPENS_WITH" => Some(g_RULED_OUT_OPENS_WITH()),
            "RULED_OUT_PARTICLES" => Some(g_RULED_OUT_PARTICLES()),
            "RULED_OUT_PREP" => Some(g_RULED_OUT_PREP()),
            "RULED_OUT_QUALITY" => Some(g_RULED_OUT_QUALITY()),
            "RULED_OUT_REPLIES_A_ROW" => Some(g_RULED_OUT_REPLIES_A_ROW()),
            "RULED_OUT_REPLIES_PLAIN" => Some(g_RULED_OUT_REPLIES_PLAIN()),
            "RULED_OUT_REPLIES_THE" => Some(g_RULED_OUT_REPLIES_THE()),
            "RULED_OUT_SOFTENING" => Some(g_RULED_OUT_SOFTENING()),
            "RULED_OUT_SYMPATHY" => Some(g_RULED_OUT_SYMPATHY()),
            "RULED_OUT_TAIL" => Some(g_RULED_OUT_TAIL()),
            "RULED_OUT_THEN_ELSE" => Some(g_RULED_OUT_THEN_ELSE()),
            "RULED_OUT_WHOLE" => Some(g_RULED_OUT_WHOLE()),
            "RULED_OUT_WORDS" => Some(g_RULED_OUT_WORDS()),
            "STEP_ELSEWHERE" => Some(g_STEP_ELSEWHERE()),
            "TITLES_IN_ANSWER" => Some(g_TITLES_IN_ANSWER()),
            "WANTED_ONE" => Some(g_WANTED_ONE()),
            "_a_claim_of_failure" => Some(py_func("_a_claim_of_failure", |args: &[Py]| _a_claim_of_failure(args))),
            "_a_class_word" => Some(py_func("_a_class_word", |args: &[Py]| _a_class_word(args))),
            "_a_list_that_can_be_swapped" => Some(py_func("_a_list_that_can_be_swapped", |args: &[Py]| _a_list_that_can_be_swapped(args))),
            "_a_pet_turned_down" => Some(py_func("_a_pet_turned_down", |args: &[Py]| _a_pet_turned_down(args))),
            "_a_row_thing" => Some(py_func("_a_row_thing", |args: &[Py]| _a_row_thing(args))),
            "_an_answer_of_that_class" => Some(py_func("_an_answer_of_that_class", |args: &[Py]| _an_answer_of_that_class(args))),
            "_class_of" => Some(py_func("_class_of", |args: &[Py]| _class_of(args))),
            "_named_picks" => Some(py_func("_named_picks", |args: &[Py]| _named_picks(args))),
            "_names_it" => Some(py_func("_names_it", |args: &[Py]| _names_it(args))),
            "_ruled_out_reply" => Some(py_func("_ruled_out_reply", |args: &[Py]| _ruled_out_reply(args))),
            "_ruled_out_thing" => Some(py_func("_ruled_out_thing", |args: &[Py]| _ruled_out_thing(args))),
            "_rules_out_what_i_never_said" => Some(py_func("_rules_out_what_i_never_said", |args: &[Py]| _rules_out_what_i_never_said(args))),
            "_the_standing_part" => Some(py_func("_the_standing_part", |args: &[Py]| _the_standing_part(args))),
            "_the_whole_answer" => Some(py_func("_the_whole_answer", |args: &[Py]| _the_whole_answer(args))),
            "_without_the_manners" => Some(py_func("_without_the_manners", |args: &[Py]| _without_the_manners(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_ROW_BETWEEN_PEOPLE" => s_A_ROW_BETWEEN_PEOPLE(v),
            "A_ROW_NAMES" => s_A_ROW_NAMES(v),
            "CLAIM_CONDITION" => s_CLAIM_CONDITION(v),
            "CLAIM_LEADS" => s_CLAIM_LEADS(v),
            "CLAIM_LEADS_IF" => s_CLAIM_LEADS_IF(v),
            "CLAIM_OF_FAILURE" => s_CLAIM_OF_FAILURE(v),
            "CLAIM_SMALL" => s_CLAIM_SMALL(v),
            "KINDS_A_LIST_CAN_SWAP" => s_KINDS_A_LIST_CAN_SWAP(v),
            "MANNERS" => s_MANNERS(v),
            "PET_NAMED" => s_PET_NAMED(v),
            "RULED_OUT" => s_RULED_OUT(v),
            "RULED_OUT_CLASS" => s_RULED_OUT_CLASS(v),
            "RULED_OUT_INSTANCES" => s_RULED_OUT_INSTANCES(v),
            "RULED_OUT_KEEPS" => s_RULED_OUT_KEEPS(v),
            "RULED_OUT_KEEP_WHOLE" => s_RULED_OUT_KEEP_WHOLE(v),
            "RULED_OUT_NAMED" => s_RULED_OUT_NAMED(v),
            "RULED_OUT_NOT_A_THING" => s_RULED_OUT_NOT_A_THING(v),
            "RULED_OUT_NOT_THEN_ELSE" => s_RULED_OUT_NOT_THEN_ELSE(v),
            "RULED_OUT_NO_ANSWER" => s_RULED_OUT_NO_ANSWER(v),
            "RULED_OUT_NO_THEN_ELSE" => s_RULED_OUT_NO_THEN_ELSE(v),
            "RULED_OUT_OPENING" => s_RULED_OUT_OPENING(v),
            "RULED_OUT_OPENS_WITH" => s_RULED_OUT_OPENS_WITH(v),
            "RULED_OUT_PARTICLES" => s_RULED_OUT_PARTICLES(v),
            "RULED_OUT_PREP" => s_RULED_OUT_PREP(v),
            "RULED_OUT_QUALITY" => s_RULED_OUT_QUALITY(v),
            "RULED_OUT_REPLIES_A_ROW" => s_RULED_OUT_REPLIES_A_ROW(v),
            "RULED_OUT_REPLIES_PLAIN" => s_RULED_OUT_REPLIES_PLAIN(v),
            "RULED_OUT_REPLIES_THE" => s_RULED_OUT_REPLIES_THE(v),
            "RULED_OUT_SOFTENING" => s_RULED_OUT_SOFTENING(v),
            "RULED_OUT_SYMPATHY" => s_RULED_OUT_SYMPATHY(v),
            "RULED_OUT_TAIL" => s_RULED_OUT_TAIL(v),
            "RULED_OUT_THEN_ELSE" => s_RULED_OUT_THEN_ELSE(v),
            "RULED_OUT_WHOLE" => s_RULED_OUT_WHOLE(v),
            "RULED_OUT_WORDS" => s_RULED_OUT_WORDS(v),
            "STEP_ELSEWHERE" => s_STEP_ELSEWHERE(v),
            "TITLES_IN_ANSWER" => s_TITLES_IN_ANSWER(v),
            "WANTED_ONE" => s_WANTED_ONE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.ruledout", module_obj);
    pyrt::xrefs::reg_fn("compose.ruledout", "_named_picks", py_func("_named_picks", |args: &[Py]| _named_picks(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_class_of", py_func("_class_of", |args: &[Py]| _class_of(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_a_class_word", py_func("_a_class_word", |args: &[Py]| _a_class_word(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_an_answer_of_that_class", py_func("_an_answer_of_that_class", |args: &[Py]| _an_answer_of_that_class(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_without_the_manners", py_func("_without_the_manners", |args: &[Py]| _without_the_manners(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_ruled_out_thing", py_func("_ruled_out_thing", |args: &[Py]| _ruled_out_thing(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_a_row_thing", py_func("_a_row_thing", |args: &[Py]| _a_row_thing(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_names_it", py_func("_names_it", |args: &[Py]| _names_it(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_the_whole_answer", py_func("_the_whole_answer", |args: &[Py]| _the_whole_answer(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_the_standing_part", py_func("_the_standing_part", |args: &[Py]| _the_standing_part(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_a_claim_of_failure", py_func("_a_claim_of_failure", |args: &[Py]| _a_claim_of_failure(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_a_pet_turned_down", py_func("_a_pet_turned_down", |args: &[Py]| _a_pet_turned_down(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_a_list_that_can_be_swapped", py_func("_a_list_that_can_be_swapped", |args: &[Py]| _a_list_that_can_be_swapped(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_rules_out_what_i_never_said", py_func("_rules_out_what_i_never_said", |args: &[Py]| _rules_out_what_i_never_said(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "_ruled_out_reply", py_func("_ruled_out_reply", |args: &[Py]| _ruled_out_reply(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("compose.ruledout", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_access("compose.ruledout", "A_ROW_BETWEEN_PEOPLE", g_A_ROW_BETWEEN_PEOPLE, s_A_ROW_BETWEEN_PEOPLE);
    pyrt::xrefs::reg_access("compose.ruledout", "A_ROW_NAMES", g_A_ROW_NAMES, s_A_ROW_NAMES);
    pyrt::xrefs::reg_access("compose.ruledout", "CLAIM_CONDITION", g_CLAIM_CONDITION, s_CLAIM_CONDITION);
    pyrt::xrefs::reg_access("compose.ruledout", "CLAIM_LEADS", g_CLAIM_LEADS, s_CLAIM_LEADS);
    pyrt::xrefs::reg_access("compose.ruledout", "CLAIM_LEADS_IF", g_CLAIM_LEADS_IF, s_CLAIM_LEADS_IF);
    pyrt::xrefs::reg_access("compose.ruledout", "CLAIM_OF_FAILURE", g_CLAIM_OF_FAILURE, s_CLAIM_OF_FAILURE);
    pyrt::xrefs::reg_access("compose.ruledout", "CLAIM_SMALL", g_CLAIM_SMALL, s_CLAIM_SMALL);
    pyrt::xrefs::reg_access("compose.ruledout", "KINDS_A_LIST_CAN_SWAP", g_KINDS_A_LIST_CAN_SWAP, s_KINDS_A_LIST_CAN_SWAP);
    pyrt::xrefs::reg_access("compose.ruledout", "MANNERS", g_MANNERS, s_MANNERS);
    pyrt::xrefs::reg_access("compose.ruledout", "PET_NAMED", g_PET_NAMED, s_PET_NAMED);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT", g_RULED_OUT, s_RULED_OUT);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_CLASS", g_RULED_OUT_CLASS, s_RULED_OUT_CLASS);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_INSTANCES", g_RULED_OUT_INSTANCES, s_RULED_OUT_INSTANCES);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_KEEPS", g_RULED_OUT_KEEPS, s_RULED_OUT_KEEPS);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_KEEP_WHOLE", g_RULED_OUT_KEEP_WHOLE, s_RULED_OUT_KEEP_WHOLE);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_NAMED", g_RULED_OUT_NAMED, s_RULED_OUT_NAMED);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_NOT_A_THING", g_RULED_OUT_NOT_A_THING, s_RULED_OUT_NOT_A_THING);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_NOT_THEN_ELSE", g_RULED_OUT_NOT_THEN_ELSE, s_RULED_OUT_NOT_THEN_ELSE);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_NO_ANSWER", g_RULED_OUT_NO_ANSWER, s_RULED_OUT_NO_ANSWER);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_NO_THEN_ELSE", g_RULED_OUT_NO_THEN_ELSE, s_RULED_OUT_NO_THEN_ELSE);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_OPENING", g_RULED_OUT_OPENING, s_RULED_OUT_OPENING);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_OPENS_WITH", g_RULED_OUT_OPENS_WITH, s_RULED_OUT_OPENS_WITH);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_PARTICLES", g_RULED_OUT_PARTICLES, s_RULED_OUT_PARTICLES);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_PREP", g_RULED_OUT_PREP, s_RULED_OUT_PREP);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_QUALITY", g_RULED_OUT_QUALITY, s_RULED_OUT_QUALITY);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_REPLIES_A_ROW", g_RULED_OUT_REPLIES_A_ROW, s_RULED_OUT_REPLIES_A_ROW);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_REPLIES_PLAIN", g_RULED_OUT_REPLIES_PLAIN, s_RULED_OUT_REPLIES_PLAIN);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_REPLIES_THE", g_RULED_OUT_REPLIES_THE, s_RULED_OUT_REPLIES_THE);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_SOFTENING", g_RULED_OUT_SOFTENING, s_RULED_OUT_SOFTENING);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_SYMPATHY", g_RULED_OUT_SYMPATHY, s_RULED_OUT_SYMPATHY);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_TAIL", g_RULED_OUT_TAIL, s_RULED_OUT_TAIL);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_THEN_ELSE", g_RULED_OUT_THEN_ELSE, s_RULED_OUT_THEN_ELSE);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_WHOLE", g_RULED_OUT_WHOLE, s_RULED_OUT_WHOLE);
    pyrt::xrefs::reg_access("compose.ruledout", "RULED_OUT_WORDS", g_RULED_OUT_WORDS, s_RULED_OUT_WORDS);
    pyrt::xrefs::reg_access("compose.ruledout", "STEP_ELSEWHERE", g_STEP_ELSEWHERE, s_STEP_ELSEWHERE);
    pyrt::xrefs::reg_access("compose.ruledout", "TITLES_IN_ANSWER", g_TITLES_IN_ANSWER, s_TITLES_IN_ANSWER);
    pyrt::xrefs::reg_access("compose.ruledout", "WANTED_ONE", g_WANTED_ONE, s_WANTED_ONE);
}
