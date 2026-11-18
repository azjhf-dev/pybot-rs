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


thread_local! { static G_ANYBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_OBJECT_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LONG_CALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SIGN_MEANS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBSET_OF_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_PHONE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANYBODY() -> Py { ensure_init(); G_ANYBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANYBODY(v: Py) { ensure_init(); G_ANYBODY.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_OBJECT_OF() -> Py { ensure_init(); G_AN_OBJECT_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_OBJECT_OF(v: Py) { ensure_init(); G_AN_OBJECT_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LONG_CALL() -> Py { ensure_init(); G_A_LONG_CALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LONG_CALL(v: Py) { ensure_init(); G_A_LONG_CALL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON() -> Py { ensure_init(); G_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); G_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE() -> Py { ensure_init(); G_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); G_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SIGN_MEANS() -> Py { ensure_init(); G_A_SIGN_MEANS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SIGN_MEANS(v: Py) { ensure_init(); G_A_SIGN_MEANS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBSET_OF_TIME() -> Py { ensure_init(); G_A_SUBSET_OF_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBSET_OF_TIME(v: Py) { ensure_init(); G_A_SUBSET_OF_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_LINES() -> Py { ensure_init(); G_CONDITION_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_LINES(v: Py) { ensure_init(); G_CONDITION_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_PHONE_LINES() -> Py { ensure_init(); G_ON_THE_PHONE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_PHONE_LINES(v: Py) { ensure_init(); G_ON_THE_PHONE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_LINES() -> Py { ensure_init(); G_PLACE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_LINES(v: Py) { ensure_init(); G_PLACE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_TO_DO() -> Py { ensure_init(); G_SAYS_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_TO_DO(v: Py) { ensure_init(); G_SAYS_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_LINES() -> Py { ensure_init(); G_SOMEONE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_LINES(v: Py) { ensure_init(); G_SOMEONE_LINES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A bare \"do i need to see anyone\" is answered by the someone the answer already named.\n\n    my dog ate chocolate\n      \"Yes, chocolate is poisonous to dogs: the darker the chocolate, the worse it is. Ring your\n       vet with the dog's weight and roughly how much and what kind of chocolate it ate; don't\n       wait for symptoms.\"\n    do i need to see anyone\n      before: \"That's beyond what I know well, sorry. Searching for it will turn up better sources\n               than I can offer; tell me what you find and I'll help you weigh it up.\"\n      after:  \"Yes - your vet. Ring them with the dog's weight and roughly how much and what kind\n               of chocolate it ate.\"\n\nThe answer had already said who to ring, in an instruction with no fence around it, and the line\nasks only whether somebody should be seen. Nothing in the knowledge base matches those words -\n\"anyone\" is not a topic - so the turn fell to the don't-know line, which is the one reply that\nthrows away an answer the bot had just given.\n\nWhat this file is careful about:\n\n* The ask must be **bare**. A line that names what it is about (\"do i need to see anyone about my\n  tax return\", \"i need to see a doctor tomorrow\") is somebody else's, and is left alone.\n* The answer in hand must name a **person to contact**, in an instruction with no \"if\", \"when\" or\n  \"most days\" in front of it - the reading `verdict._referral` already makes, so the fence rules\n  written there are the ones used here. An answer that says \"see a doctor if it is still there in\n  three weeks\" is advice for later, and the doubt about it is answered by the condition, not by\n  this.\n* The reply is the answer's own words, cut to the instruction. Nothing is added about the thing\n  the answer was about, so the bot cannot say more than it already did.\n");
    s_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:do|should|shall|must|need|will|would|can|could|have)\\s+(?:i|we|you)\\s+(?:(?:need|have|got|want)\\s+to\\s+|really\\s+|actually\\s+|even\\s+|just\\s+)?(?:see|ring|call|visit|contact|speak\\s+to|talk\\s+to|go\\s+(?:to|and\\s+see)|get\\s+seen\\s+by|book\\s+(?:in\\s+with|an?\\s+appointment\\s+with)|get\\s+(?:an?\\s+)?appointment\\s+with)\\s+(?:somebody|someone|anybody|anyone|a\\s+professional|a\\s+(?:doctor|vet|dentist|nurse|gp|pharmacist|optician|midwife|lawyer|plumber|electrician|specialist)|the\\s+(?:doctor|vet|dentist|nurse|gp|pharmacist|optician))\\b\\s*[?.!]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:your|the|a|an|their|his|her)?\\s*(vet|doctor|gp|dentist|nurse|pharmacist|optician|midwife|health visitor|paramedic|physio(?:therapist)?|practice nurse|a&e|111|999)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PLACE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:a&e|the\\s+emergency\\s+department|walk-?in\\s+centre|minor\\s+injuries(?:\\s+unit)?|urgent\\s+care|out\\s+of\\s+hours|999|911|111|112)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SOMEONE_LINES(py_tuple(vec![py_str("Yes - {person}. {call}"), py_str("Yes, {person} - {call_lower}"), py_str("The answer named one: {person}. {call}"), py_str("Yes: {person}. {call}")]));
    s_PLACE_LINES(py_tuple(vec![py_str("Yes - {call_lower}"), py_str("{call}"), py_str("Yes, that one: {call_lower}")]));
    s_ON_THE_PHONE_LINES(py_tuple(vec![py_str("{call}"), py_str("Yes - {call_lower}"), py_str("The answer said it plainly: {call_lower}")]));
    s_A_LONG_CALL(Py::Int(16));
    s_A_SIGN_MEANS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:if|unless|when|whenever|means|would\\s+mean|otherwise|in\\s+case)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANYBODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:someone|somebody|anyone|anybody|a\\s+professional)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AN_OBJECT_OF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:see|ring|call|contact|visit|ask|speak\\s+to|talk\\s+to|say\\s+to|saying\\s+to|go\\s+to|book\\s+in\\s+with|appointments?\\s+with|with|to)\\s*$")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDITION_LINES(py_tuple(vec![py_str("Not always. {clause}"), py_str("It depends. {clause}"), py_str("Go by this. {clause}"), py_str("Only if it comes to that. {clause}")]));
    s_A_SUBSET_OF_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:that|which|who)\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAYS_TO_DO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:worth|should|needs?|must|ring|call|see|speak|talk|book|get|go|make an appointment|appointment|check(?:ed)?|look(?:ed)? at)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _the_answer_in_hand(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The reply the bot gave last, or \"\".");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "strip", args))(&__args) } } }) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_condition(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut person: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own sentence making a referral depend on a sign, cut to the sign.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?]) ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    sentence = __item.clone();
    person = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ANYBODY(), "search", args))(&__args) } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&person.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::verdict::g_NAMES_A_CLINICIAN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ANYBODY(), "search", args))(&__args) } } })) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SIGN_MEANS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(person.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_sign_not_a_step(args))(&__args) } } }))) {
    continue;
    }
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) };
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(", but ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "rfind", args))(&__args) };
    if py_truthy(&{ let __b = op_gt(&cut.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); __args.push(person.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_sign_not_a_step(args))(&__args) } } }) {
    clause = slice_or_raise(&clause.clone(), &op_add(&cut.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(", but ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None, &Py::None);
    clause = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&clause.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return clause.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_question(args: &[Py]) -> Py {
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
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { py_str("") };
    let mut said_now: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own question, if it asked one the person can answer.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?]) ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    sentence = __item.clone();
    said_now = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = said_now.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&clause.clone(), &said_now.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_now.clone(), "endswith", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said_now.clone(), &py_str("? "))) } } } } } }) {
    if py_truthy(&Py::Bool(py_contains(&said_now.clone(), &py_str("?")))) {
    return slice_or_raise(&said_now.clone(), &Py::None, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_now.clone(), "index", args))(&__args) }, &Py::Int(1)), &Py::None);
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_condition_reply(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clause: Py = Py::None;
    let mut question: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut written: Py = Py::None;
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_condition(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    return py_str("");
    }
    written = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "rstrip", args))(&__args) }, &py_str("."));
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{clause}")); __args.push(written.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CONDITION_LINES()); __args.push(memory.clone()); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) }, "replace", args))(&__args) };
    question = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_question(args))(&__args) };
    if py_truthy(&{ let __b = question.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&reply.clone(), &question.clone())) } }) {
    reply = op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "rstrip", args))(&__args) }, &py_str(" ")), &question.clone());
    }
    return reply.clone();
    Py::None
}
pub fn _a_sign_not_a_step(args: &[Py]) -> Py {
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
    let mut person: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut before: Py = Py::None;
    let _ = py_str("True when what comes before the named person makes the referral depend on a sign.");
    if py_truthy(&Py::Bool(py_is(&person.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    before = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "start", args))(&__args) }, &Py::None);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SIGN_MEANS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SUBSET_OF_TIME(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_sentence_that_names_someone(args: &[Py]) -> Py {
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
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's own sentence naming somebody to contact, if it is advice and not a condition.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?]) ")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::verdict::g_NAMES_A_CLINICIAN(), "search", args))(&__args) }))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::verdict::g_FENCED_IN_SENTENCE(), "search", args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_sign_not_a_step(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAYS_TO_DO(), "search", args))(&__args) }) {
    return sentence.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_call(args: &[Py]) -> Py {
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
    let mut breaks: Py = Py::None;
    let mut call: Py = Py::None;
    let mut head: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut person: Py = Py::None;
    let mut person_in_it: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The instruction in the answer that names somebody to contact, and who it names.");
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::verdict::_the_referral_sentence(args))(&__args) };
    person_in_it = if py_truthy(&sentence.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&person_in_it.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(person_in_it.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_sign_not_a_step(args))(&__args) } } } } }) {
    sentence = py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&sentence.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::verdict::_referral(args))(&__args) }, &Py::None)) } }) {
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_sentence_that_names_someone(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&sentence.clone()))) {
    return py_tuple(vec![py_str(""), Py::None]);
    }
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    call = text.clone();
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(";")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }, &Py::Int(0)), "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) }) {
    call = head.clone();
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&call.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_A_LONG_CALL())) {
    person = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(call.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) };
    tail = if py_truthy(&person.clone()) { slice_or_raise(&call.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "end", args))(&__args) }, &Py::None, &Py::None) } else { py_str("") };
    breaks = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(","), py_str(":"), py_str(";")])) { let mut mark: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&tail.clone(), &mark.clone()))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mark.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tail.clone(), "index", args))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&breaks.clone()) {
    call = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&call.clone(), &Py::None, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "end", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(breaks.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None), "strip", args))(&__args) };
    }
    }
    call = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .;:,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&call.clone(), "rstrip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(call.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) })) } }) {
    return py_tuple(vec![py_str(""), Py::None]);
    }
    return py_tuple(vec![op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&call.clone(), &Py::Int(0)), "lower", args))(&__args) }, &slice_or_raise(&call.clone(), &Py::Int(1), &Py::None, &Py::None)), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(call.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON(), "search", args))(&__args) }]);
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
    let mut a_place: Py = Py::None;
    let mut after: Py = Py::None;
    let mut call: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut on_the_phone: Py = Py::None;
    let mut person: Py = Py::None;
    let mut place: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_again: Py = Py::None;
    let _ = py_str("The reply to a bare \"do i need to see someone\", or None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK(), "match", args))(&__args) }))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_answer_in_hand(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_call(args))(&__args) }, 2, None);
    call = __unpk1[0].clone();
    person = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&person.clone(), &Py::None)) } }) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_condition_reply(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "group", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    on_the_phone = Py::Bool(py_contains(&py_tuple(vec![py_str("999"), py_str("911"), py_str("111"), py_str("112")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "strip", args))(&__args) }, "lower", args))(&__args) }));
    a_place = { let __b = on_the_phone.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLACE(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    if py_truthy(&a_place.clone()) {
    said_again = call.clone();
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&call.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "start", args))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AN_OBJECT_OF(), "search", args))(&__args) }))) {
    said_again = call.clone();
    } else {
    after = slice_or_raise(&call.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "end", args))(&__args) }, &Py::None, &Py::None);
    place = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(appointments?|visits?|checks?)\\b")); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&place.clone()) {
    noun = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&place.clone(), "group", args))(&__args) };
    pronoun = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("appointment")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&noun.clone(), "startswith", args))(&__args) }) { py_str("an appointment") } else { op_add(&py_str("a "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&noun.clone(), "rstrip", args))(&__args) }) };
    said_again = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&call.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "start", args))(&__args) }, &Py::None), "rstrip", args))(&__args) }, &py_str(" ")), &pronoun.clone()), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&after.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&place.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "rstrip", args))(&__args) }), "strip", args))(&__args) };
    } else {
    said_again = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&call.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "start", args))(&__args) }, &Py::None), "rstrip", args))(&__args) }, &py_str(" them ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&after.clone(), "lstrip", args))(&__args) }), "strip", args))(&__args) };
    }
    }
    }
    pool = if py_truthy(&on_the_phone.clone()) { g_ON_THE_PHONE_LINES() } else { if py_truthy(&a_place.clone()) { g_PLACE_LINES() } else { g_SOMEONE_LINES() } };
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{person}")); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __args.push(call.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) }, "replace", args))(&__args) };
    said_again = op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said_again.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said_again.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{call_lower}")); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said_again.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", args))(&__args) }, &slice_or_raise(&said_again.clone(), &Py::Int(1), &Py::None, &Py::None))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{call}")); __args.push(said_again.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    Py::None
}
pub fn replies_to_a_who_to_see(args: &[Py]) -> Py {
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
    let _ = py_str("True when this file has the line, for the module that declares the claim.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reply_for(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
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
    Py::Module(Rc::new(PyModule { name: "bot.compose.who_to_see".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANYBODY" => Some(g_ANYBODY()),
            "AN_OBJECT_OF" => Some(g_AN_OBJECT_OF()),
            "ASK" => Some(g_ASK()),
            "A_LONG_CALL" => Some(g_A_LONG_CALL()),
            "A_PERSON" => Some(g_A_PERSON()),
            "A_PLACE" => Some(g_A_PLACE()),
            "A_SIGN_MEANS" => Some(g_A_SIGN_MEANS()),
            "A_SUBSET_OF_TIME" => Some(g_A_SUBSET_OF_TIME()),
            "CONDITION_LINES" => Some(g_CONDITION_LINES()),
            "ON_THE_PHONE_LINES" => Some(g_ON_THE_PHONE_LINES()),
            "PLACE_LINES" => Some(g_PLACE_LINES()),
            "SAYS_TO_DO" => Some(g_SAYS_TO_DO()),
            "SOMEONE_LINES" => Some(g_SOMEONE_LINES()),
            "_a_sentence_that_names_someone" => Some(py_func("_a_sentence_that_names_someone", |args: &[Py]| _a_sentence_that_names_someone(args))),
            "_a_sign_not_a_step" => Some(py_func("_a_sign_not_a_step", |args: &[Py]| _a_sign_not_a_step(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_the_answer_in_hand" => Some(py_func("_the_answer_in_hand", |args: &[Py]| _the_answer_in_hand(args))),
            "_the_call" => Some(py_func("_the_call", |args: &[Py]| _the_call(args))),
            "_the_condition" => Some(py_func("_the_condition", |args: &[Py]| _the_condition(args))),
            "_the_condition_reply" => Some(py_func("_the_condition_reply", |args: &[Py]| _the_condition_reply(args))),
            "_the_question" => Some(py_func("_the_question", |args: &[Py]| _the_question(args))),
            "replies_to_a_who_to_see" => Some(py_func("replies_to_a_who_to_see", |args: &[Py]| replies_to_a_who_to_see(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANYBODY" => s_ANYBODY(v),
            "AN_OBJECT_OF" => s_AN_OBJECT_OF(v),
            "ASK" => s_ASK(v),
            "A_LONG_CALL" => s_A_LONG_CALL(v),
            "A_PERSON" => s_A_PERSON(v),
            "A_PLACE" => s_A_PLACE(v),
            "A_SIGN_MEANS" => s_A_SIGN_MEANS(v),
            "A_SUBSET_OF_TIME" => s_A_SUBSET_OF_TIME(v),
            "CONDITION_LINES" => s_CONDITION_LINES(v),
            "ON_THE_PHONE_LINES" => s_ON_THE_PHONE_LINES(v),
            "PLACE_LINES" => s_PLACE_LINES(v),
            "SAYS_TO_DO" => s_SAYS_TO_DO(v),
            "SOMEONE_LINES" => s_SOMEONE_LINES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.who_to_see", module_obj);
    pyrt::xrefs::reg_fn("compose.who_to_see", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "_the_answer_in_hand", py_func("_the_answer_in_hand", |args: &[Py]| _the_answer_in_hand(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "_the_condition", py_func("_the_condition", |args: &[Py]| _the_condition(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "_the_question", py_func("_the_question", |args: &[Py]| _the_question(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "_the_condition_reply", py_func("_the_condition_reply", |args: &[Py]| _the_condition_reply(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "_a_sign_not_a_step", py_func("_a_sign_not_a_step", |args: &[Py]| _a_sign_not_a_step(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "_a_sentence_that_names_someone", py_func("_a_sentence_that_names_someone", |args: &[Py]| _a_sentence_that_names_someone(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "_the_call", py_func("_the_call", |args: &[Py]| _the_call(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("compose.who_to_see", "replies_to_a_who_to_see", py_func("replies_to_a_who_to_see", |args: &[Py]| replies_to_a_who_to_see(args)));
    pyrt::xrefs::reg_access("compose.who_to_see", "ANYBODY", g_ANYBODY, s_ANYBODY);
    pyrt::xrefs::reg_access("compose.who_to_see", "AN_OBJECT_OF", g_AN_OBJECT_OF, s_AN_OBJECT_OF);
    pyrt::xrefs::reg_access("compose.who_to_see", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("compose.who_to_see", "A_LONG_CALL", g_A_LONG_CALL, s_A_LONG_CALL);
    pyrt::xrefs::reg_access("compose.who_to_see", "A_PERSON", g_A_PERSON, s_A_PERSON);
    pyrt::xrefs::reg_access("compose.who_to_see", "A_PLACE", g_A_PLACE, s_A_PLACE);
    pyrt::xrefs::reg_access("compose.who_to_see", "A_SIGN_MEANS", g_A_SIGN_MEANS, s_A_SIGN_MEANS);
    pyrt::xrefs::reg_access("compose.who_to_see", "A_SUBSET_OF_TIME", g_A_SUBSET_OF_TIME, s_A_SUBSET_OF_TIME);
    pyrt::xrefs::reg_access("compose.who_to_see", "CONDITION_LINES", g_CONDITION_LINES, s_CONDITION_LINES);
    pyrt::xrefs::reg_access("compose.who_to_see", "ON_THE_PHONE_LINES", g_ON_THE_PHONE_LINES, s_ON_THE_PHONE_LINES);
    pyrt::xrefs::reg_access("compose.who_to_see", "PLACE_LINES", g_PLACE_LINES, s_PLACE_LINES);
    pyrt::xrefs::reg_access("compose.who_to_see", "SAYS_TO_DO", g_SAYS_TO_DO, s_SAYS_TO_DO);
    pyrt::xrefs::reg_access("compose.who_to_see", "SOMEONE_LINES", g_SOMEONE_LINES, s_SOMEONE_LINES);
}
