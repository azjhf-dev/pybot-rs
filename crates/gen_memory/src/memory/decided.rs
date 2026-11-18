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


thread_local! { static G_ACTIVITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOOSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPANION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPANION_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTING_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECIDING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GUESSING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDIOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INNER_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENING_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPTION_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REACHES_FOR_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TO_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ACTIVITY() -> Py { ensure_init(); G_ACTIVITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACTIVITY(v: Py) { ensure_init(); G_ACTIVITY.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_FRAME() -> Py { ensure_init(); G_ASK_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_FRAME(v: Py) { ensure_init(); G_ASK_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOOSING() -> Py { ensure_init(); G_CHOOSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOOSING(v: Py) { ensure_init(); G_CHOOSING.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_CHOICE() -> Py { ensure_init(); G_CLAUSE_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_CHOICE(v: Py) { ensure_init(); G_CLAUSE_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPANION() -> Py { ensure_init(); G_COMPANION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPANION(v: Py) { ensure_init(); G_COMPANION.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPANION_VERBS() -> Py { ensure_init(); G_COMPANION_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPANION_VERBS(v: Py) { ensure_init(); G_COMPANION_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTING() -> Py { ensure_init(); G_CORRECTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTING(v: Py) { ensure_init(); G_CORRECTING.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTING_LINE() -> Py { ensure_init(); G_CORRECTING_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTING_LINE(v: Py) { ensure_init(); G_CORRECTING_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_DECIDING_WORD() -> Py { ensure_init(); G_DECIDING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECIDING_WORD(v: Py) { ensure_init(); G_DECIDING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_DOING() -> Py { ensure_init(); G_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOING(v: Py) { ensure_init(); G_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_GOING() -> Py { ensure_init(); G_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOING(v: Py) { ensure_init(); G_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_GUESSING_ASK() -> Py { ensure_init(); G_GUESSING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GUESSING_ASK(v: Py) { ensure_init(); G_GUESSING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_IDIOM() -> Py { ensure_init(); G_IDIOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDIOM(v: Py) { ensure_init(); G_IDIOM.with(|c| *c.borrow_mut() = v); }
pub fn g_INNER_DECIDE() -> Py { ensure_init(); G_INNER_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INNER_DECIDE(v: Py) { ensure_init(); G_INNER_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_IN() -> Py { ensure_init(); G_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_IN(v: Py) { ensure_init(); G_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SUBJECT() -> Py { ensure_init(); G_NOT_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SUBJECT(v: Py) { ensure_init(); G_NOT_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENING_QUESTION() -> Py { ensure_init(); G_OPENING_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENING_QUESTION(v: Py) { ensure_init(); G_OPENING_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_OPTIONS() -> Py { ensure_init(); G_OPTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPTIONS(v: Py) { ensure_init(); G_OPTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPTION_FILLER() -> Py { ensure_init(); G_OPTION_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPTION_FILLER(v: Py) { ensure_init(); G_OPTION_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_KEYS() -> Py { ensure_init(); G_PLAN_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_KEYS(v: Py) { ensure_init(); G_PLAN_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g_REACHES_FOR_A_CHOICE() -> Py { ensure_init(); G_REACHES_FOR_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REACHES_FOR_A_CHOICE(v: Py) { ensure_init(); G_REACHES_FOR_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLING() -> Py { ensure_init(); G_SETTLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLING(v: Py) { ensure_init(); G_SETTLING.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL() -> Py { ensure_init(); G_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL(v: Py) { ensure_init(); G_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_TO_YOU() -> Py { ensure_init(); G_TO_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TO_YOU(v: Py) { ensure_init(); G_TO_YOU.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What they settled on, and the ask that wants it back.\n\nA chat that weighs two things and then picks one keeps the choosing line (\"actually a dog\",\n\"lets go with spain\") only as a line they said: \"what did i decide to get\" found no reader at all\nand answered with a search hedge, \"what did we settle on\" and \"which one did i pick\" were taken\nby the topic's advice (dog walking, holiday packing), and \"what am i getting\" got the advice\nagain. The choosing line is a small shape, and the answer to the ask is the thing it named, so\nthe shape is read here and kept as a decision of its own.\n");
    s_LEAD_IN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ok(?:ay)?|so|right|well|then|hmm|oh|and|but)[, ]+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SETTLING(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:actually|instead|in the end|in that case|on second thoughts?|on second thought)[,]?\\s+)(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we|i've|ive|we've|weve)\\s+(?:have\\s+|had\\s+)?(?:decided|settled)\\s+(?:on|for)?\\s*(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+(?:picked|chose|landed on|ended up with)\\s+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+(?:go|went|goes)\\s+with\\s+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_GOING(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i'?m|im|i am|we'?re|were)\\s+(?:going|gonna)\\s+(?:with|for)\\s+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i'?m|im|i am|we'?re|were)\\s+going to\\s+go\\s+(?:with|for)\\s+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_ACTIVITY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("run jog walk stroll hike swim shower bath drive ride cycle coffee\n    drink pint curry dinner lunch breakfast supper nap kip sleep sit look wander shop shopping\n    haircut massage break rest smoke fag wee pee leak"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTING_LINE(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:no|nah|nope)[,:]\\s*(?:i mean(?:t)?[, ]+)?(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:no|nah|nope)\\s+i mean(?:t)?[, ]+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_CHOOSING(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:lets|let's|i'?ll|ill|we'?ll|well|i'?m going to|im going to|i'?m gonna|im gonna|we'?re going to|were going to)\\s+(?:go|going|get|getting|have|having|take|taking|do|doing|pick|picking|choose|choosing|book|booking|buy|buying)\\s+(?:with|for)?\\s*(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+went for\\s+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<what>.{2,40}?),?\\s+i (?:think|reckon|suppose)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<what>(?:a|an|the|some)\\s+[\\w'’-]+(?:\\s+[\\w'’-]+)?)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bmake it\\s+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:lets|let's|i'?ll|ill|we'?ll|well|i will|we will)\\s+(?:take|do|have)\\s+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+said\\s+(?P<what>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_OPTIONS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bor\\b|\\bbetween\\b|\\btorn\\b|\\bthinking of\\b|\\bthinking about\\b|\\bweighing up\\b|\\bshould i\\b|\\beither\\b|\\bwhich one\\b|\\bdecid(?:e|ed|ing)\\b|cannot decide|can'?t decide|\\bchoose\\b|\\bchoices?\\b|\\boptions\\b|\\bhelp me pick\\b|\\bvs\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\ni im i'm we we're you you're it its it's that this which what who they he she there here\nme my our your his her their is are am was were do does did dont don't doesnt didn't\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAUSE_CHOICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:actually|instead|in the end|turns out|it turns out)[,]?\\s+(?:i|we)(?:'ve|ve|'d|d)?\\s+(?:have\\s+|had\\s+|just\\s+)?(?:booked|bought|got|ordered|chosen|chose|picked|reserved|taken|took|signed up for|settled on|decided on|gone with|went with|landed on)\\s+(?P<what>.+)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:no|nah|nope|i mean|i meant|wait|sorry)[, ]+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:instead|after all|i think|i reckon|i suppose|then|actually|last night|last week|yesterday|today|this morning)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IDIOM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_list(vec![py_str("the flow"), py_str("the grain"), py_str("the crowd"), py_str("thank you"), py_str("thanks"), py_str("the rest")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("it that this them worries way idea chance problem doubt quick fast work happen up right\n    better worse soon now snappy official count brief short clear easy fun nice worth cheaper\n    bigger smaller louder quieter sense money"), "split", args))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOST_WORDS(Py::Int(4));
    s_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:did|do|have|had) (?:i|we) (?:decide|decided|settle(?:d)? on|choose|chose|pick|picked|go for|went for|end up with)\\b|\\bwhat (?:am|are) (?:i|we) (?:getting|going with|going for|ending up with|having)\\b|\\bwhich (?:one|of (?:them|those))? (?:did (?:i|we) (?:pick|choose|go for|decide|settle on|go with|end up with)|(?:am|are) (?:i|we) (?:getting|going with))\\b|\\bwhich (?:one|of (?:them|those)) (?:i|we) (?:picked|chose|went with|went for|settled on|decided on)\\b|\\bwhere did (?:i|we) (?:decide|settle) (?:to go|on)\\b|\\bso what (?:am|are) (?:i|we) (?:getting|going with|doing)\\b|\\bwhat (?:i|we) (?:decided|chose|picked|settled on|went with|went for)\\b|\\b(?:have|has|had|did) (?:i|we) (?:decide|decided|picked|chosen|chose|settled)\\b|\\bare (?:we|you) decided\\b|\\bwhat was (?:decided|chosen|picked|settled on)\\b|\\bwhat (?:did|do) (?:i|we) (?:land|opt) (?:on|for)\\b|\\bwhich did (?:i|we) (?:go for|pick|choose|decide on|settle on)\\b|\\bwhat did (?:i|we) say (?:i|we)(?:'d| would) (?:get|choose|pick|go for|have)\\b|\\bremind me (?:of|about) (?:the|our) choice\\b|\\bdid (?:i|we) (?:go with|go for|settle on|pick|choose|decide on|end up with)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GUESSING_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bdid (?:i|we) (?:go with|go for|settle on|pick|choose|decide on|end up with)\\s+(?P<what>[^?.!,]+)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TO_YOU(py_dict(vec![(py_str("i"), py_str("you")), (py_str("im"), py_str("you're")), (py_str("i'm"), py_str("you're")), (py_str("me"), py_str("you")), (py_str("my"), py_str("your")), (py_str("mine"), py_str("yours")), (py_str("myself"), py_str("yourself")), (py_str("we"), py_str("you")), (py_str("our"), py_str("your")), (py_str("ours"), py_str("yours")), (py_str("ive"), py_str("you've")), (py_str("i've"), py_str("you've"))]));
    s_INNER_DECIDE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:i|we)(?:'ve|'d|ve|d| have| had)?\\s+(?:decided|settled|chosen|chose|picked|gone|went|going|go)\\b\\s*(?:on|with|for)?\\s*")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OPTION_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("im i we you they should shall do does did get gets getting got\n    go goes going gonna want wants wanted need needs like rather prefer thinking thought\n    about of between torn pick picking choose choosing decide decided deciding help me if\n    maybe well so and then also still just were was are is am been being cant can't cannot\n    unsure whether if better best worse worth it to up weighing"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REACHES_FOR_A_CHOICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:go|goes|going|went)\\s+(?:with|for)\\b|\\b(?:settled|decided|landed|ended up)\\s+(?:on|for|with)\\b|\\b(?:(?:going|gonna)\\s+to\\s+)?(?:take|taking|took|book|booking|pick|picked|choose|chose|choosing)\\s+(?:the|a|an|my|that|this)\\b|\\b(?:picked|chose|choosing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OPENING_QUESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:is|are|was|were|should|shall|do|does|did|can|could|would|will|what|which|how|where|when|why|any|if)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DECIDING_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:decid|settl|pick|cho(?:o|s)|decided|deciding)\\w*")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLAN_KEYS(py_tuple(vec![py_str("plans"), py_str("plan")]));
    s_ASK_FRAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("what which one ones did do does have has had i we you me us my our\n    decide decided deciding settle settled settling on choose chose chosen pick picked picking\n    go went going for get gets getting got have having take took end ended up with am are was\n    were so about of to the a an it that this them those remind remember again the choice\n    our land landed opt opted for say said would will should and then there where when why\n    how who whose much many long kind sort time place"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("one ones one's best better worse right thing things way again now\n    then really please just also still ever again"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DOING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("make makes made making cook cooks cooked cooking eat eats eating drink\n    drinks drinking buy buys buying wear wears wearing watch watches watching learn learns\n    learning study studies studying grow grows growing plant plants planting build builds\n    building serve serves serving"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COMPANION_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("drink drinks drinking serve serves serving have has having\n    order orders ordering pair pairs pairing pour pours pouring"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COMPANION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\b(?:%s)\\s+(?:with|alongside)\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_COMPANION_VERBS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut trimmed: Py = Py::None;
    let mut verb_phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The chosen thing as the store should keep it, or None when the phrase is not one.");
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    while py_truthy(&Py::Bool(true)) {
    trimmed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTING(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&trimmed.clone(), &phrase.clone()))) {
    break;
    }
    phrase = trimmed.clone();
    }
    while py_truthy(&Py::Bool(true)) {
    trimmed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INNER_DECIDE(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&trimmed.clone(), &phrase.clone()))) {
    break;
    }
    phrase = trimmed.clone();
    }
    while py_truthy(&Py::Bool(true)) {
    trimmed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TAIL(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&trimmed.clone(), &phrase.clone()))) {
    break;
    }
    phrase = trimmed.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_IDIOM(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "lower", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\d.,]+")); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "fullmatch", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "split", args))(&__args) };
    verb_phrase = { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "lower", args))(&__args) }, &py_str("to"))) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &if py_truthy(&verb_phrase.clone()) { op_add(&g_MOST_WORDS(), &Py::Int(4)) } else { g_MOST_WORDS() }) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "lower", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_THING(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    return phrase.clone();
    Py::None
}
pub fn _two_things(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut head: Py = Py::None;
    let mut left: Py = Py::None;
    let mut left_words: Py = Py::None;
    let mut low: Py = Py::None;
    let mut right: Py = Py::None;
    let mut right_words: Py = Py::None;
    let _ = py_str("The two things a line put side by side, as \"X or Y\", or None.\n\n    Used by the ask that finds nothing settled: naming the two things the chat left open is an\n    answer, and \"Nothing settled yet.\" on its own tells the person less than they told us.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    left = py_str("");
    right = py_str("");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&low.clone(), &py_str(" between "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" between ")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "split", args))(&__args) }, &Py::Int(1)), &py_str(" and "))) } }) {
    head = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" between ")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "split", args))(&__args) }, &Py::Int(1));
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" and ")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&head.clone(), "split", args))(&__args) }, 2, None);
    left = __unpk1[0].clone();
    right = __unpk1[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&low.clone(), &py_str(" or ")))) {
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" or ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "rfind", args))(&__args) };
    let __unpk2 = py_unpack(&py_tuple(vec![slice_or_raise(&low.clone(), &Py::None, &cut.clone(), &Py::None), slice_or_raise(&low.clone(), &op_add(&cut.clone(), &Py::Int(4)), &Py::None, &Py::None)]), 2, None);
    left = __unpk2[0].clone();
    right = __unpk2[1].clone();
    } else {
    return Py::None;
    }
    }
    left_words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&left.clone(), "split", args))(&__args) };
    while py_truthy(&{ let __b = left_words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OPTION_FILLER(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&left_words.clone(), &Py::Int(0)), "strip", args))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&left_words.clone(), "pop", args))(&__args) };
    }
    right_words = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&right.clone(), "split", args))(&__args) }, &Py::None, &Py::Int(3), &Py::None);
    while py_truthy(&{ let __b = right_words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OPTION_FILLER(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&right_words.clone(), &op_neg(&Py::Int(1))), "strip", args))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&right_words.clone(), "pop", args))(&__args) };
    }
    left = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&left_words.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    right = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(right_words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&left.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&right.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_IDIOM(), &left.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_IDIOM(), &right.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &left.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &right.clone())) } } } } } }) {
    return Py::None;
    }
    return op_add(&op_add(&left.clone(), &py_str(" or ")), &right.clone());
    Py::None
}
pub fn _options_since(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut key: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The pair put on the table after the choice on the book was made, or None.\n\n    \"im thinking of getting a cat\" / \"actually a dog\" is a choice; a later \"we were thinking italy\n    or spain\" is a new question, and answering it with the dog answers a chat that is over.\n    ");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    key = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    return Py::None;
    }
    seen = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, &key.clone()))) {
    seen = Py::Bool(true);
    break;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPTIONS(), "search", args))(&__args) })) } }) {
    continue;
    }
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _two_things(args))(&__args) };
    if py_truthy(&pair.clone()) {
    return pair.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_pair_left_open(args: &[Py]) -> Py {
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
    let mut pair: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("\"italy or spain\" for the last line that put two things in front of them, or None.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPTIONS(), "search", args))(&__args) })) } } } }) {
    continue;
    }
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _two_things(args))(&__args) };
    if py_truthy(&pair.clone()) {
    return pair.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _as_typed(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing with the case the person typed it in: \"Spain\" stays, \"spain\" stays lower.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return thing.clone();
    }
    pattern = op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\\s+"), "join", args))(&__args) }), &py_str("\\b"));
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return if py_truthy(&match_.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } } else { thing.clone() };
    Py::None
}
pub fn read(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut patterns: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let _ = py_str("The choice a line makes: {\"what\": thing, \"settled\": bool}, or None.\n\n    \"settled\" says whether the line settles the choice by itself (\"i've decided on the blue one\",\n    \"actually a dog\") or only when the chat had options on the table (\"lets go with spain\").\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(14)) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks(args))(&__args) }) {
    return Py::None;
    }
    while py_truthy(&Py::Bool(true)) {
    trimmed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD_IN(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&trimmed.clone(), &line.clone()))) {
    break;
    }
    line = trimmed.clone();
    }
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAUSE_CHOICE(), "search", args))(&__args) };
    if py_truthy(&clause.clone()) {
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) };
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_activity(args))(&__args) })) } }) {
    return py_dict(vec![(py_str("what"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _as_typed(args))(&__args) }), (py_str("settled"), Py::Bool(true))]);
    }
    }
    let __iter1 = py_iter_or_raise(&g_CORRECTING_LINE());
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&match_.clone()) {
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) };
    if py_truthy(&thing.clone()) {
    return py_dict(vec![(py_str("what"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _as_typed(args))(&__args) }), (py_str("settled"), Py::Bool(false)), (py_str("corrects"), Py::Bool(true))]);
    }
    }
    }
    let __iter2 = py_iter_or_raise(&g_GOING());
    for __item in __iter2 {
    pattern = __item.clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) };
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_activity(args))(&__args) })) } }) {
    return py_dict(vec![(py_str("what"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _as_typed(args))(&__args) }), (py_str("settled"), Py::Bool(true))]);
    }
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_SETTLING(), Py::Bool(true)]), py_tuple(vec![g_CHOOSING(), Py::Bool(false)])]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    patterns = __unpk4[0].clone();
    settled = __unpk4[1].clone();
    let __iter5 = py_iter_or_raise(&patterns.clone());
    for __item in __iter5 {
    pattern = __item.clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) };
    if py_truthy(&thing.clone()) {
    return py_dict(vec![(py_str("what"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _as_typed(args))(&__args) }), (py_str("settled"), settled.clone())]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn extract(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let _ = py_str("The thing a choosing line settled on, or None.\n\n    \"actually a dog\" is one; so is \"lets go with spain\", \"ive decided on the blue one\" and \"i\n    think ill go with fixing mine\". \"actually i live in leeds\" is a correction, \"actually its\n    tuesday\" puts a day in place, and neither settles anything, so both are left alone.\n    ");
    reading = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| read(args))(&__args) };
    return if py_truthy(&reading.clone()) { geti(&reading.clone(), &py_str("what")) } else { Py::None };
    Py::None
}
pub fn _an_activity(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing after a going verb is something a person does, not something chosen.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "strip", args))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("some"), py_str("my"), py_str("our")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ACTIVITY(), &geti(&words.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _a_bare_thing(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a line is one short thing and nothing else: \"cat\", \"the blue one\".");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_THING(), &w.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(false);
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "isalpha", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } };
    Py::None
}
pub fn _options_on_the_table(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the last thing said put a choice in front of them.\n\n    Only the two turns before the choosing line count. A choice is made right after the options\n    are named (\"i cant decide between spain and italy\" then \"lets go with spain\"), and a word like\n    \"between\" from three turns back is about something else by then: it is what made \"im torn\n    between the bike and the train\", \"ill go with the train\" and then \"i went for a walk\" file the\n    walk as a choice.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_bare_thing(args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPTIONS(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_choice_reached_for(args: &[Py]) -> Py {
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
    let mut kept_line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept_line") { v.clone() }
    else { Py::None };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the last thing they said is a choice this reader could not name.\n\n    A line the store already holds is the one exception: it was named when it was said.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks(args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OPENING_QUESTION(), "match", args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = kept_line.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept_line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REACHES_FOR_A_CHOICE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_plan_since(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the last thing they said was a plan still going: \"im going for a run\".\n\n    A plan is read and refused rather than unreadable, so the ask after it can be answered with\n    the truth (\"Nothing settled yet.\") instead of the module standing down.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } } } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&g_GOING());
    for __item in __iter2 {
    pattern = __item.clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&match_.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_activity(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return Py::Bool(false);
    }
    return Py::Bool(false);
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
    let mut reading: Py = Py::None;
    let _ = py_str("Keep the thing they settled on. A later choice is the one they settled on.");
    reading = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| read(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reading.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("corrects")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_decision(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _options_on_the_table(args))(&__args) } } })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&geti(&reading.clone(), &py_str("settled")))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("corrects")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _options_on_the_table(args))(&__args) })) } } } }) {
    return Py::None;
    }
    let _ = py_setitem(&memory.clone(), &py_str("decided"), py_dict(vec![(py_str("what"), geti(&reading.clone(), &py_str("what"))), (py_str("line"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) })]));
    return geti(&memory.clone(), &py_str("decided"));
    Py::None
}
pub fn _the_plan_on_record(args: &[Py]) -> Py {
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
    let mut p: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut user: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The plan the chat has already said back to them, latest first, or None.\n\n    A plan the bot never said back is not one the chat settled: \"im learning to swim\" files the\n    plan \"Swim\", and \"you settled on Swim\" is a decision they never made. A plan the bot has\n    already confirmed - \"Got it - you are going to Cornwall in September.\" - is the chat settled,\n    and the ask about it must not be told the chat settled nothing.\n    ");
    user = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(user.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    plans = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&user.clone(), "get", args))(&__args) }) { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&user.clone(), "get", args))(&__args) }]) } else { py_list(vec![]) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plans")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&user.clone(), "get", args))(&__args) }); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    plans = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&user.clone(), &py_str("plans"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &plans.clone());
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut p: Py = __item.clone(); if py_truthy(&p.clone()) {  __out.push(p.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    plan = __item.clone();
    plan = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&plan.clone(), "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| said_back(args))(&__args) } } }) {
    return plan.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn said_back(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("Whether an earlier reply of the bot already said these words.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &w.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_decision(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let _ = py_str("The choice on record, or None.");
    held = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("decided")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) } } }) {
    return held.clone();
    }
    return Py::None;
    Py::None
}
pub fn asks(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is asking back what the chat settled on.");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_guess(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The thing an ask expects to hear back (\"did we settle on spain\"), or None.");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GUESSING_ASK(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) };
    Py::None
}
pub fn _the_same_thing(args: &[Py]) -> Py {
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
    let mut held: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: held"))) };
    let mut first: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut second: Py = Py::None;
    let mut verb_one: Py = Py::None;
    let mut verb_two: Py = Py::None;
    let _ = py_str("Whether the thing guessed is the thing on record: \"spain\" is \"spain in may\".");
    one = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = held.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&other.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "startswith", args))(&__args) }) {
    one = slice_or_raise(&one.clone(), &Py::Int(3), &Py::None, &Py::None);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&other.clone(), "startswith", args))(&__args) }) {
    other = slice_or_raise(&other.clone(), &Py::Int(3), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&one.clone(), &other.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&other.clone(), &one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&one.clone(), &other.clone())) } } } }) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "split", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&other.clone(), "split", args))(&__args) }]), 2, None);
    first = __unpk1[0].clone();
    second = __unpk1[1].clone();
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(second.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&first.clone(), &Py::Int(1), &Py::None, &Py::None), &slice_or_raise(&second.clone(), &Py::Int(1), &Py::None, &Py::None))) } } } }) {
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&first.clone(), &Py::Int(0)), geti(&second.clone(), &Py::Int(0))]), 2, None);
    verb_one = __unpk2[0].clone();
    verb_two = __unpk2[1].clone();
    return { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(verb_one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(verb_two.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&verb_one.clone(), &Py::None, &Py::Int(3), &Py::None), &slice_or_raise(&verb_two.clone(), &Py::None, &Py::Int(3), &Py::None))) } } } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _said_to_you(args: &[Py]) -> Py {
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
    let mut swapped: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing in second person, in the case they typed it: \"fixing mine\" -> \"fixing yours\".");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    swapped = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TO_YOU(), "get", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&swapped.clone()) { swapped.clone() } else { word.clone() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "append", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _a_verb_phrase(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True for a decision stated as what they will do: \"to give up sugar\", \"to take the job\".");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) };
    return { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "lower", args))(&__args) }, &py_str("to"))) } };
    Py::None
}
pub fn _the_choice_said(args: &[Py]) -> Py {
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
    let _ = py_str("The choice in a sentence: \"the blue one\" -> \"You went with the blue one.\".");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_verb_phrase(args))(&__args) }) {
    return op_add(&op_add(&py_str("You decided "), &thing.clone()), &py_str("."));
    }
    return op_add(&op_add(&py_str("You went with "), &thing.clone()), &py_str("."));
    Py::None
}
pub fn _at_the_start(args: &[Py]) -> Py {
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
    let _ = py_str("The thing where it opens the sentence: \"fixing yours\" -> \"Fixing yours\".");
    return if py_truthy(&thing.clone()) { op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { thing.clone() };
    Py::None
}
pub fn _asks_about_another_thing(args: &[Py]) -> Py {
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
    let mut held: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: held"))) };
    let mut held_text: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the ask names a thing the choice on record is not about.\n\n    \"my boiler is playing up\" / \"what did i decide about the boiler\": the person is asking about\n    the boiler, and the chat never settled on one, so naming the dog would be a false memory and\n    \"nothing settled yet\" would throw away the boiler answer. The module stands down instead.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    held_text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = held.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_COMPANION(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = w.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_ASK_FRAME(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_SUBJECT(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_DOING(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "isdigit", args))(&__args) })) } } } } } } } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&held_text.clone(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&w.clone(), &Py::None, &Py::Int(4), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held_text.clone(), "startswith", args))(&__args) } } } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    let mut guess: Py = Py::None;
    let mut held: Py = Py::None;
    let mut moved_on: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The reply to an ask that wants the choice back, or None when the line is not one.\n\n    With nothing settled the answer says so: a choice the chat never made is not a fact, and\n    \"you haven't picked one yet\" is the one true thing to say.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks(args))(&__args) }))) {
    return Py::None;
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_decision(args))(&__args) };
    if py_truthy(&held.clone()) {
    moved_on = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("line")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _options_since(args))(&__args) };
    if py_truthy(&{ let __b = moved_on.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("line")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_choice_reached_for(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("Nothing settled yet: "), &moved_on.clone()), &py_str(".")), op_add(&op_add(&py_str("Not yet: "), &moved_on.clone()), &py_str(".")), op_add(&op_add(&py_str("Neither one yet: "), &moved_on.clone()), &py_str("."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_guess(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_about_another_thing(args))(&__args) } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = held.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("line")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_choice_reached_for(args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_about_another_thing(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_guess(args))(&__args) })) } }) {
    return Py::None;
    }
    plan = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_plan_on_record(args))(&__args) };
    if py_truthy(&{ let __b = plan.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DECIDING_WORD(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_dict(vec![(py_str("what"), plan.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_about_another_thing(args))(&__args) })) } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("You settled on "), &plan.clone()), &py_str(".")), op_add(&op_add(&py_str("That's settled: "), &plan.clone()), &py_str(".")), op_add(&op_add(&py_str("The chat settled it: "), &plan.clone()), &py_str("."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_choice_reached_for(args))(&__args) }) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DECIDING_WORD(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_plan_since(args))(&__args) } } }))) {
    return Py::None;
    }
    }
    pair = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_pair_left_open(args))(&__args) };
    if py_truthy(&pair.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("Nothing settled yet: "), &pair.clone()), &py_str(".")), op_add(&op_add(&py_str("Not yet: "), &pair.clone()), &py_str(".")), op_add(&op_add(&py_str("Neither one yet: "), &pair.clone()), &py_str("."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![py_str("Nothing settled yet."), py_str("You haven't picked one yet."), py_str("You haven't settled on one yet.")])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_to_you(args))(&__args) };
    guess = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_guess(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_verb_phrase(args))(&__args) }) {
    if py_truthy(&guess.clone()) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(guess.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_same_thing(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("That's right: you decided "), &thing.clone()), &py_str(".")), op_add(&op_add(&py_str("Yes - you decided "), &thing.clone()), &py_str("."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("Not quite: you decided "), &thing.clone()), &py_str(".")), op_add(&op_add(&py_str("The other one: you decided "), &thing.clone()), &py_str("."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("You decided "), &thing.clone()), &py_str(".")), op_add(&op_add(&py_str("You decided "), &thing.clone()), &py_str(", in the end."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&guess.clone()) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(guess.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_same_thing(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("Yes - "), &thing.clone()), &py_str(", in the end.")), op_add(&op_add(&py_str("That's right: "), &thing.clone()), &py_str(".")), op_add(&op_add(&py_str("Yes, "), &thing.clone()), &py_str("."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("No - "), &thing.clone()), &py_str(", in the end.")), op_add(&op_add(&py_str("Not quite: "), &thing.clone()), &py_str(", in the end.")), op_add(&op_add(&py_str("The other one: "), &thing.clone()), &py_str("."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _at_the_start(args))(&__args) }, &py_str(", in the end.")), op_add(&op_add(&py_str("You settled on "), &thing.clone()), &py_str(".")), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_choice_said(args))(&__args) }])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.decided".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACTIVITY" => Some(g_ACTIVITY()),
            "ASK" => Some(g_ASK()),
            "ASK_FRAME" => Some(g_ASK_FRAME()),
            "CHOOSING" => Some(g_CHOOSING()),
            "CLAUSE_CHOICE" => Some(g_CLAUSE_CHOICE()),
            "COMPANION" => Some(g_COMPANION()),
            "COMPANION_VERBS" => Some(g_COMPANION_VERBS()),
            "CORRECTING" => Some(g_CORRECTING()),
            "CORRECTING_LINE" => Some(g_CORRECTING_LINE()),
            "DECIDING_WORD" => Some(g_DECIDING_WORD()),
            "DOING" => Some(g_DOING()),
            "GOING" => Some(g_GOING()),
            "GUESSING_ASK" => Some(g_GUESSING_ASK()),
            "IDIOM" => Some(g_IDIOM()),
            "INNER_DECIDE" => Some(g_INNER_DECIDE()),
            "LEAD_IN" => Some(g_LEAD_IN()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "NOT_A_SUBJECT" => Some(g_NOT_A_SUBJECT()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "OPENING_QUESTION" => Some(g_OPENING_QUESTION()),
            "OPTIONS" => Some(g_OPTIONS()),
            "OPTION_FILLER" => Some(g_OPTION_FILLER()),
            "PLAN_KEYS" => Some(g_PLAN_KEYS()),
            "REACHES_FOR_A_CHOICE" => Some(g_REACHES_FOR_A_CHOICE()),
            "SETTLING" => Some(g_SETTLING()),
            "TAIL" => Some(g_TAIL()),
            "TO_YOU" => Some(g_TO_YOU()),
            "_a_bare_thing" => Some(py_func("_a_bare_thing", |args: &[Py]| _a_bare_thing(args))),
            "_a_choice_reached_for" => Some(py_func("_a_choice_reached_for", |args: &[Py]| _a_choice_reached_for(args))),
            "_a_plan_since" => Some(py_func("_a_plan_since", |args: &[Py]| _a_plan_since(args))),
            "_a_verb_phrase" => Some(py_func("_a_verb_phrase", |args: &[Py]| _a_verb_phrase(args))),
            "_an_activity" => Some(py_func("_an_activity", |args: &[Py]| _an_activity(args))),
            "_as_typed" => Some(py_func("_as_typed", |args: &[Py]| _as_typed(args))),
            "_asks_about_another_thing" => Some(py_func("_asks_about_another_thing", |args: &[Py]| _asks_about_another_thing(args))),
            "_at_the_start" => Some(py_func("_at_the_start", |args: &[Py]| _at_the_start(args))),
            "_options_on_the_table" => Some(py_func("_options_on_the_table", |args: &[Py]| _options_on_the_table(args))),
            "_options_since" => Some(py_func("_options_since", |args: &[Py]| _options_since(args))),
            "_said_to_you" => Some(py_func("_said_to_you", |args: &[Py]| _said_to_you(args))),
            "_the_choice_said" => Some(py_func("_the_choice_said", |args: &[Py]| _the_choice_said(args))),
            "_the_pair_left_open" => Some(py_func("_the_pair_left_open", |args: &[Py]| _the_pair_left_open(args))),
            "_the_plan_on_record" => Some(py_func("_the_plan_on_record", |args: &[Py]| _the_plan_on_record(args))),
            "_the_same_thing" => Some(py_func("_the_same_thing", |args: &[Py]| _the_same_thing(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_two_things" => Some(py_func("_two_things", |args: &[Py]| _two_things(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "asks" => Some(py_func("asks", |args: &[Py]| asks(args))),
            "extract" => Some(py_func("extract", |args: &[Py]| extract(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "read" => Some(py_func("read", |args: &[Py]| read(args))),
            "said_back" => Some(py_func("said_back", |args: &[Py]| said_back(args))),
            "the_decision" => Some(py_func("the_decision", |args: &[Py]| the_decision(args))),
            "the_guess" => Some(py_func("the_guess", |args: &[Py]| the_guess(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACTIVITY" => s_ACTIVITY(v),
            "ASK" => s_ASK(v),
            "ASK_FRAME" => s_ASK_FRAME(v),
            "CHOOSING" => s_CHOOSING(v),
            "CLAUSE_CHOICE" => s_CLAUSE_CHOICE(v),
            "COMPANION" => s_COMPANION(v),
            "COMPANION_VERBS" => s_COMPANION_VERBS(v),
            "CORRECTING" => s_CORRECTING(v),
            "CORRECTING_LINE" => s_CORRECTING_LINE(v),
            "DECIDING_WORD" => s_DECIDING_WORD(v),
            "DOING" => s_DOING(v),
            "GOING" => s_GOING(v),
            "GUESSING_ASK" => s_GUESSING_ASK(v),
            "IDIOM" => s_IDIOM(v),
            "INNER_DECIDE" => s_INNER_DECIDE(v),
            "LEAD_IN" => s_LEAD_IN(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "NOT_A_SUBJECT" => s_NOT_A_SUBJECT(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "OPENING_QUESTION" => s_OPENING_QUESTION(v),
            "OPTIONS" => s_OPTIONS(v),
            "OPTION_FILLER" => s_OPTION_FILLER(v),
            "PLAN_KEYS" => s_PLAN_KEYS(v),
            "REACHES_FOR_A_CHOICE" => s_REACHES_FOR_A_CHOICE(v),
            "SETTLING" => s_SETTLING(v),
            "TAIL" => s_TAIL(v),
            "TO_YOU" => s_TO_YOU(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.decided", module_obj);
    pyrt::xrefs::reg_fn("memory.decided", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_two_things", py_func("_two_things", |args: &[Py]| _two_things(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_options_since", py_func("_options_since", |args: &[Py]| _options_since(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_the_pair_left_open", py_func("_the_pair_left_open", |args: &[Py]| _the_pair_left_open(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_as_typed", py_func("_as_typed", |args: &[Py]| _as_typed(args)));
    pyrt::xrefs::reg_fn("memory.decided", "read", py_func("read", |args: &[Py]| read(args)));
    pyrt::xrefs::reg_fn("memory.decided", "extract", py_func("extract", |args: &[Py]| extract(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_an_activity", py_func("_an_activity", |args: &[Py]| _an_activity(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_a_bare_thing", py_func("_a_bare_thing", |args: &[Py]| _a_bare_thing(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_options_on_the_table", py_func("_options_on_the_table", |args: &[Py]| _options_on_the_table(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_a_choice_reached_for", py_func("_a_choice_reached_for", |args: &[Py]| _a_choice_reached_for(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_a_plan_since", py_func("_a_plan_since", |args: &[Py]| _a_plan_since(args)));
    pyrt::xrefs::reg_fn("memory.decided", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_the_plan_on_record", py_func("_the_plan_on_record", |args: &[Py]| _the_plan_on_record(args)));
    pyrt::xrefs::reg_fn("memory.decided", "said_back", py_func("said_back", |args: &[Py]| said_back(args)));
    pyrt::xrefs::reg_fn("memory.decided", "the_decision", py_func("the_decision", |args: &[Py]| the_decision(args)));
    pyrt::xrefs::reg_fn("memory.decided", "asks", py_func("asks", |args: &[Py]| asks(args)));
    pyrt::xrefs::reg_fn("memory.decided", "the_guess", py_func("the_guess", |args: &[Py]| the_guess(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_the_same_thing", py_func("_the_same_thing", |args: &[Py]| _the_same_thing(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_said_to_you", py_func("_said_to_you", |args: &[Py]| _said_to_you(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_a_verb_phrase", py_func("_a_verb_phrase", |args: &[Py]| _a_verb_phrase(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_the_choice_said", py_func("_the_choice_said", |args: &[Py]| _the_choice_said(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_at_the_start", py_func("_at_the_start", |args: &[Py]| _at_the_start(args)));
    pyrt::xrefs::reg_fn("memory.decided", "_asks_about_another_thing", py_func("_asks_about_another_thing", |args: &[Py]| _asks_about_another_thing(args)));
    pyrt::xrefs::reg_fn("memory.decided", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_access("memory.decided", "ACTIVITY", g_ACTIVITY, s_ACTIVITY);
    pyrt::xrefs::reg_access("memory.decided", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("memory.decided", "ASK_FRAME", g_ASK_FRAME, s_ASK_FRAME);
    pyrt::xrefs::reg_access("memory.decided", "CHOOSING", g_CHOOSING, s_CHOOSING);
    pyrt::xrefs::reg_access("memory.decided", "CLAUSE_CHOICE", g_CLAUSE_CHOICE, s_CLAUSE_CHOICE);
    pyrt::xrefs::reg_access("memory.decided", "COMPANION", g_COMPANION, s_COMPANION);
    pyrt::xrefs::reg_access("memory.decided", "COMPANION_VERBS", g_COMPANION_VERBS, s_COMPANION_VERBS);
    pyrt::xrefs::reg_access("memory.decided", "CORRECTING", g_CORRECTING, s_CORRECTING);
    pyrt::xrefs::reg_access("memory.decided", "CORRECTING_LINE", g_CORRECTING_LINE, s_CORRECTING_LINE);
    pyrt::xrefs::reg_access("memory.decided", "DECIDING_WORD", g_DECIDING_WORD, s_DECIDING_WORD);
    pyrt::xrefs::reg_access("memory.decided", "DOING", g_DOING, s_DOING);
    pyrt::xrefs::reg_access("memory.decided", "GOING", g_GOING, s_GOING);
    pyrt::xrefs::reg_access("memory.decided", "GUESSING_ASK", g_GUESSING_ASK, s_GUESSING_ASK);
    pyrt::xrefs::reg_access("memory.decided", "IDIOM", g_IDIOM, s_IDIOM);
    pyrt::xrefs::reg_access("memory.decided", "INNER_DECIDE", g_INNER_DECIDE, s_INNER_DECIDE);
    pyrt::xrefs::reg_access("memory.decided", "LEAD_IN", g_LEAD_IN, s_LEAD_IN);
    pyrt::xrefs::reg_access("memory.decided", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("memory.decided", "NOT_A_SUBJECT", g_NOT_A_SUBJECT, s_NOT_A_SUBJECT);
    pyrt::xrefs::reg_access("memory.decided", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.decided", "OPENING_QUESTION", g_OPENING_QUESTION, s_OPENING_QUESTION);
    pyrt::xrefs::reg_access("memory.decided", "OPTIONS", g_OPTIONS, s_OPTIONS);
    pyrt::xrefs::reg_access("memory.decided", "OPTION_FILLER", g_OPTION_FILLER, s_OPTION_FILLER);
    pyrt::xrefs::reg_access("memory.decided", "PLAN_KEYS", g_PLAN_KEYS, s_PLAN_KEYS);
    pyrt::xrefs::reg_access("memory.decided", "REACHES_FOR_A_CHOICE", g_REACHES_FOR_A_CHOICE, s_REACHES_FOR_A_CHOICE);
    pyrt::xrefs::reg_access("memory.decided", "SETTLING", g_SETTLING, s_SETTLING);
    pyrt::xrefs::reg_access("memory.decided", "TAIL", g_TAIL, s_TAIL);
    pyrt::xrefs::reg_access("memory.decided", "TO_YOU", g_TO_YOU, s_TO_YOU);
}
