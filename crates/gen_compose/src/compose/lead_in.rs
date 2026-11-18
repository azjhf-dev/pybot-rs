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


thread_local! { static G_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEWEST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_HELP_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FLOOR_OR_A_NOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_QUESTION() -> Py { ensure_init(); G_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION(v: Py) { ensure_init(); G_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_FEWEST_WORDS() -> Py { ensure_init(); G_FEWEST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEWEST_WORDS(v: Py) { ensure_init(); G_FEWEST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_IN() -> Py { ensure_init(); G_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_IN(v: Py) { ensure_init(); G_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_HELP_NOW() -> Py { ensure_init(); G_NAMED_HELP_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_HELP_NOW(v: Py) { ensure_init(); G_NAMED_HELP_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_NEWS() -> Py { ensure_init(); G_NO_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_NEWS(v: Py) { ensure_init(); G_NO_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FLOOR_OR_A_NOD() -> Py { ensure_init(); G_THE_FLOOR_OR_A_NOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FLOOR_OR_A_NOD(v: Py) { ensure_init(); G_THE_FLOOR_OR_A_NOD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A lead-in in front of the ask is not part of the ask.\n\nPeople chain a question onto what was just said, and the word they chain it with is not part of\nit: \"and\", \"also\", \"ok so\", \"but\", \"then\". Main answers some of those and loses others, because\nthe reading of a line as a question is anchored at its first word:\n\n    is b&q open on sunday\n      -> \"The opening hours are the first thing a venue changes and the last thing it tells\n          anyone ... Their own page or map listing has the current ones.\"\n    and is b&q open on sunday\n      -> \"Ask away, I'm listening. What's going on?\"\n\nThe same ask, two letters in front of it, and the person gets the floor - a line that would fit\nany message, which is the one thing this lane exists to keep off their screen. It reads as noise\nrather than as a shrug, which is why nothing caught it: the line is not a hedge, so every pool\nthat stands down for a hedge kept it.\n\nWhat this module does, and only this: a line whose first words are a lead-in and whose *rest* is\na question, where what the bot would say to the whole line is only the floor or a hedge, is\nanswered as the question it is - by the same composer, asked about the rest of the line. The\ncomposer is not reimplemented here; `fallback.reply_for` is the one that answers, so the pool the\nperson gets is the pool they would have got without the two letters in front.\n\nIt stands down on everything else:\n\n- a line that is not a question after the lead-in (\"and my mum is in hospital\", \"and it was\n  lovely\") keeps whatever answered it - stripping the lead-in from a report would make the bot\n  answer a question nobody asked;\n- a line the bot can already answer keeps its answer: the floor-and-hedge test is what decides,\n  so \"and can you smell gas\" still gets the gas card, and a card is never replaced;\n- a line whose stripped ask gets only the floor too (\"and what about it\") gains nothing, so\n  nothing is said;\n- a first turn pointing back at nothing (\"and what about that\") is left to the modules that read\n  a conversation that has not started.\n\nThe composer is asked twice - once about the whole line, once about the rest - so the rule is\ngeneral rather than a list of lead-ins bolted onto one reader: any reader that is anchored at the\nfirst word is covered, and a reader that already tolerates a lead-in gives the same answer both\ntimes and this module stands down.\n");
    s_LEAD_IN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:and|but|so|also|plus|then|ok|okay|right|well|oh|ah|now|anyway|still)\\b[\\s,.!\\-]*)+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_QUESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:is|are|was|were|am|do|does|did|can|could|will|would|should|shall|may|might|have|has|had|what|when|where|which|who|whose|why|how)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FEWEST_WORDS(Py::Int(3));
    s_THE_FLOOR_OR_A_NOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:ask away|i'?m listening|i am listening|go on\\b|tell me more|say more\\b|i'?m here\\b|take your time|over to you|spill it|i'?m all ears|got it\\b|noted\\b|thanks for telling me|fair enough|right then|okay then|sure\\b|of course\\b|go ahead\\b|i'?ll do my best|good luck with it|anything else\\b|what'?s on your mind|ask me anything)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMED_HELP_NOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:999|112|111|ambulance|a&e|casualty|emergency|suicid\\w*|overdos\\w*|poison\\w*|bleeding|unconscious|not breathing|chest pain|anaphyla\\w*)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_NEWS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an and the i me my we us our you your it its is was were be been being am\nare do does did done can could would should will shall may might must have has had having to of in\non at for with from about as that this these those there here then now so but or if not no yes\nwell oh ah ok okay right sure got noted thanks thank you please just very really quite too also\nstill any some all more most much many long far off soon quickly quick plenty time times diary\nforward looking look round around coming come going go ahead mind anything something nothing else\none ones what whats when where which who why how say said tell told"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _only_the_floor_or_a_nod(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the reply is nothing but the floor, a nod or a receipt.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback::is_a_hedge(args))(&__args) }) {
    return Py::Bool(true);
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::ack::reads_as_a_receipt(args))(&__args) }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FLOOR_OR_A_NOD(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_rest_after_the_lead_in(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The question the line asks once the lead-in is off it, or \"\" when there is no lead-in.");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEAD_IN(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_QUESTION(), "match", args))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FEWEST_WORDS())) {
    return py_str("");
    }
    return rest.clone();
    Py::None
}
pub fn _a_copy(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A copy of memory to ask a question of, the way the runtime makes one for a module.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "items", args))(&__args) }) { let __unpk9684 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk9684[0].clone(); let mut value: Py = __unpk9684[1].clone(); __out.push((key.clone(), if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("turns")))) { value.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("copy", "deepcopy", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } })); } py_dict(__out) })() }]);
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
    return memory.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _read_as_its_own_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut mine: Py = Py::None;
    let _ = py_str("The conversation's memory, with the understanding reread for the shorter line.\n\n    Everything in the composer reads `memory[\"understood\"]`, and by the time a module is asked\n    that is the reading of the *whole* line - `and is b&q open on sunday` is a statement because\n    it opens with `and`. Asking the composer about `is b&q open on sunday` with that reading in\n    place gives the same floor line again, which is how the first cut of this module answered\n    nothing at all. The reading is redone here for the words that are left, and only that: the\n    understanding writes `understood` and nothing else, so no fact is filed twice.\n    ");
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_copy(args))(&__args) };
    let __try1 = exc::catch(|| {
    let mut understand: Py = pyrt::xrefs::xmodule("modules.understand");
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understand.clone(), "observe", args))(&__args) };
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
    return mine.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return mine.clone();
    Py::None
}
pub fn _an_echo_of_the_ask(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut asked: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply says nothing the ask did not already say.\n\n    \"Is b&q open on sunday — noted.\" and \"Is b&q open on sunday. Not long now, then.\" are the\n    person's own words in a frame. They are not the floor, so the floor test let them through, and\n    in a conversation that is the shape the composer answers a chained ask with: the module stood\n    down and the person kept the receipt. A card says something the ask did not (\"the opening\n    hours are the first thing a venue changes\"), and that stays the answer.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9&']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9&']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NO_NEWS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&asked.clone(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn _a_reply_of_its_own(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply says something of its own rather than handing the turn back.");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_the_floor_or_a_nod(args))(&__args) })) } };
    Py::None
}
pub fn the_answer_the_lead_in_hid(args: &[Py]) -> Py {
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
    let mut alone: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut key: Py = Py::None;
    let mut line: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut value: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut without: Py = Py::None;
    let _ = py_str("The reply the ask would have had without the lead-in, or None.\n\n    `memory` is the module's own copy, so the second question is asked of the same conversation.\n    The first question gets a copy of its own: a call that reads memory must not leave a trace\n    that changes what the second call sees.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_rest_after_the_lead_in(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    whole = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_copy(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g_reply_for(), args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_the_floor_or_a_nod(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(whole.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_echo_of_the_ask(args))(&__args) })) } }) {
    return Py::None;
    }
    without = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_as_its_own_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g_reply_for(), args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(without.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_reply_of_its_own(args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&without.clone(), &whole.clone())) } }) {
    alone = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "items", args))(&__args) }) { let __unpk1645 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk1645[0].clone(); let mut value: Py = __unpk1645[1].clone(); if py_truthy(&Py::Bool(!py_eq(&key.clone(), &py_str("turns")))) {  __out.push((key.clone(), value.clone())); } } py_dict(__out) })() };
    fresh = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); __args.push(alone.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _read_as_its_own_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g_reply_for(), args))(&__args) };
    if py_truthy(&{ let __b = fresh.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(fresh.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_reply_of_its_own(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&fresh.clone(), &whole.clone())) } } } }) {
    without = fresh.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&without.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&without.clone(), &whole.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = whole.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_HELP_NOW(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = without.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_HELP_NOW(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(without.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_reply_of_its_own(args))(&__args) }))) {
    return Py::None;
    }
    return without.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.lead_in".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_QUESTION" => Some(g_A_QUESTION()),
            "FEWEST_WORDS" => Some(g_FEWEST_WORDS()),
            "LEAD_IN" => Some(g_LEAD_IN()),
            "NAMED_HELP_NOW" => Some(g_NAMED_HELP_NOW()),
            "NO_NEWS" => Some(g_NO_NEWS()),
            "THE_FLOOR_OR_A_NOD" => Some(g_THE_FLOOR_OR_A_NOD()),
            "_a_copy" => Some(py_func("_a_copy", |args: &[Py]| _a_copy(args))),
            "_a_reply_of_its_own" => Some(py_func("_a_reply_of_its_own", |args: &[Py]| _a_reply_of_its_own(args))),
            "_an_echo_of_the_ask" => Some(py_func("_an_echo_of_the_ask", |args: &[Py]| _an_echo_of_the_ask(args))),
            "_only_the_floor_or_a_nod" => Some(py_func("_only_the_floor_or_a_nod", |args: &[Py]| _only_the_floor_or_a_nod(args))),
            "_read_as_its_own_line" => Some(py_func("_read_as_its_own_line", |args: &[Py]| _read_as_its_own_line(args))),
            "_the_rest_after_the_lead_in" => Some(py_func("_the_rest_after_the_lead_in", |args: &[Py]| _the_rest_after_the_lead_in(args))),
            "the_answer_the_lead_in_hid" => Some(py_func("the_answer_the_lead_in_hid", |args: &[Py]| the_answer_the_lead_in_hid(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_QUESTION" => s_A_QUESTION(v),
            "FEWEST_WORDS" => s_FEWEST_WORDS(v),
            "LEAD_IN" => s_LEAD_IN(v),
            "NAMED_HELP_NOW" => s_NAMED_HELP_NOW(v),
            "NO_NEWS" => s_NO_NEWS(v),
            "THE_FLOOR_OR_A_NOD" => s_THE_FLOOR_OR_A_NOD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.lead_in", module_obj);
    pyrt::xrefs::reg_fn("compose.lead_in", "_only_the_floor_or_a_nod", py_func("_only_the_floor_or_a_nod", |args: &[Py]| _only_the_floor_or_a_nod(args)));
    pyrt::xrefs::reg_fn("compose.lead_in", "_the_rest_after_the_lead_in", py_func("_the_rest_after_the_lead_in", |args: &[Py]| _the_rest_after_the_lead_in(args)));
    pyrt::xrefs::reg_fn("compose.lead_in", "_a_copy", py_func("_a_copy", |args: &[Py]| _a_copy(args)));
    pyrt::xrefs::reg_fn("compose.lead_in", "_read_as_its_own_line", py_func("_read_as_its_own_line", |args: &[Py]| _read_as_its_own_line(args)));
    pyrt::xrefs::reg_fn("compose.lead_in", "_an_echo_of_the_ask", py_func("_an_echo_of_the_ask", |args: &[Py]| _an_echo_of_the_ask(args)));
    pyrt::xrefs::reg_fn("compose.lead_in", "_a_reply_of_its_own", py_func("_a_reply_of_its_own", |args: &[Py]| _a_reply_of_its_own(args)));
    pyrt::xrefs::reg_fn("compose.lead_in", "the_answer_the_lead_in_hid", py_func("the_answer_the_lead_in_hid", |args: &[Py]| the_answer_the_lead_in_hid(args)));
    pyrt::xrefs::reg_access("compose.lead_in", "A_QUESTION", g_A_QUESTION, s_A_QUESTION);
    pyrt::xrefs::reg_access("compose.lead_in", "FEWEST_WORDS", g_FEWEST_WORDS, s_FEWEST_WORDS);
    pyrt::xrefs::reg_access("compose.lead_in", "LEAD_IN", g_LEAD_IN, s_LEAD_IN);
    pyrt::xrefs::reg_access("compose.lead_in", "NAMED_HELP_NOW", g_NAMED_HELP_NOW, s_NAMED_HELP_NOW);
    pyrt::xrefs::reg_access("compose.lead_in", "NO_NEWS", g_NO_NEWS, s_NO_NEWS);
    pyrt::xrefs::reg_access("compose.lead_in", "THE_FLOOR_OR_A_NOD", g_THE_FLOOR_OR_A_NOD, s_THE_FLOOR_OR_A_NOD);
}
