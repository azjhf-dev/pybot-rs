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
thread_local! { static G_ASKS_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GENERIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKS_ADVICE() -> Py { ensure_init(); G_ASKS_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_ADVICE(v: Py) { ensure_init(); G_ASKS_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_A_LENGTH() -> Py { ensure_init(); G_ASKS_A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_LENGTH(v: Py) { ensure_init(); G_ASKS_A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH() -> Py { ensure_init(); G_A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH(v: Py) { ensure_init(); G_A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_REPLY() -> Py { ensure_init(); G_A_LENGTH_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_REPLY(v: Py) { ensure_init(); G_A_LENGTH_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_SAID() -> Py { ensure_init(); G_A_LENGTH_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_SAID(v: Py) { ensure_init(); G_A_LENGTH_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_GENERIC() -> Py { ensure_init(); G_GENERIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GENERIC(v: Py) { ensure_init(); G_GENERIC.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_LENGTH() -> Py { ensure_init(); G_NO_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_LENGTH(v: Py) { ensure_init(); G_NO_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_STAY() -> Py { ensure_init(); G_STAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAY(v: Py) { ensure_init(); G_STAY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A length asked of a stay: \"how long will it take\" after \"my mum is coming to stay\".\n\nA visit announced is answered with a card about making a guest comfortable, and the card says\nnothing about time. The very next line is the one a person asks about it - how long the stay is,\nhow long the visit wants - and it names no subject of its own, so the reading hands the knowledge\nbase the topic of the conversation and the same card comes back, cut to its first sentence:\n\n    my mum is coming to stay next week\n      -> \"Give them the best bed and plan one shared outing a day, so the flat is mainly for\n          sleeping and everyone gets a reason to leave it. Agree meal times on the first evening\n          and keep one evening for yourself; a week goes better when nobody is guessing.\"\n    how long will it take\n      -> \"Give them the best bed and plan one shared outing a day, so the flat is mainly for\n          sleeping and everyone gets a reason to leave it.\"\n\nThe person has read that sentence already and still does not know how long anything takes. What\nthey need is either the length the chat already said (\"she is here for four days\") put back to\nthem with what it means for the plan, or - when the chat never said one - the question asked,\nbecause the length is the thing that decides how many outings the card's advice means.\n\nThis module only takes that one turn: a stay in the chat, a length asked with no subject of its\nown, and a last answer that gave no length. Everything else is left as it was, which is why\n\"i want to repaint the hall\" / \"how long will it take\" still gets the painting card's day's work,\nand a trip (\"im going to portugal in july\") is not a stay and is never touched.\n");
    s_PRIORITY(Py::Int(1050));
    s_STAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:coming|comes|came|coming over|coming round|getting here|arriving|arrives|visiting)\\b[^.!?]{0,40}?\\b(?:stay|staying|stays|visit|to us|over|round|for (?:the )?weekend|for \\w+ (?:days|nights|weeks))\\b|\\b(?:is|are|was|were)\\s+(?:coming\\s+)?(?:to\\s+)?staying\\b|\\bstaying\\s+(?:with\\s+us|here|over|for)\\b|\\bstopping\\s+(?:with\\s+us|over|here)\\b|\\b(?:a|the)\\s+visit\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LENGTH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfor\\s+(?:about\\s+|just\\s+|only\\s+)?(\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|a\\s+few|a\\s+couple\\s+of)\\s*(day|days|night|nights|week|weeks)\\b|\\b(?:for\\s+)?(?:the|this|that|next|at\\s+the)\\s+weekend\\b|\\b(?:a|one)\\s+week\\b|\\b(?:a|one)\\s+fortnight\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LENGTH_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\d+|two|three|four|five|six|seven|eight|nine|ten|twenty)\\s*(?:full|whole|more|extra)?\\s*(?:days?|nights?|weeks?|hours?|minutes?)\\b|\\ba\\s+(?:week|fortnight|month)\\b|\\bweekend\\b|\\bfortnight\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_A_LENGTH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+long\\b|\\bhow\\s+many\\s+(?:days|nights|weeks)\\b|\\bhow\\s+much\\s+time\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GENERIC({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nabout actually again all also and an any are around as at be been before both but by can could do\ndoes doing else even ever exactly for from get getting go going got had has have he her here hers\nhim his how i if in into is it its just keep know like long may me might more much my nearly no\nnot now of off on once one only or our ours out over own please really right said say she should\nso some still take takes taking than that the their theirs them then there these they thing this\nthose to too up us very want was way we well were what when where which while who why will with\nwould you your yours sorted stuff bit bits stay stays staying visit visiting long weekend days\nnights\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LENGTH_REPLY(py_tuple(vec![py_str("{length} is a comfortable stay: one shared outing a day is enough for that long, and keep one evening back for yourself."), py_str("For {length}, plan one outing a day rather than a schedule - and leave one evening free."), py_str("{length} gives you room for an outing a day. The rest of the card is what matters more: the bed and the meal times.")]));
    s_ADVICE(py_tuple(vec![py_str("Three or four days is the sweet spot for a first stay: long enough to settle in, short enough that nobody runs out of things to say."), py_str("A weekend to a week. Past a week, plan one evening apart or the visit starts to feel like a duty for both of you."), py_str("Up to a week is the usual answer - long enough to settle in, and the length where an outing a day and one evening kept back still feel like a visit.")]));
    s_ASKS_ADVICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+long\\s+(?:should|do\\s+you\\s+think|would\\s+you|is\\s+(?:it\\s+)?(?:right|normal))\\b|\\bhow\\s+many\\s+(?:days|nights)\\s+should\\b|\\bhow\\s+long\\s+(?:is\\s+)?(?:too\\s+long|long\\s+enough)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NO_LENGTH(py_tuple(vec![py_str("You have not said how long they are staying, and that is what decides it: a weekend is one outing, a week is three or four. How long is it?"), py_str("How long is the stay? A weekend wants one thing planned and a week wants two or three, so the answer changes with it."), py_str("How long are they with you for? Tell me and I will say how much to plan - the bed and the meal times are the same either way.")]));
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_stay(args: &[Py]) -> Py {
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
    let _ = py_str("The line that announced the stay, or \"\" when the chat has none.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STAY(), "search", args))(&__args) }) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_length_said(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The length the person gave for the stay (\"four days\", \"the weekend\"), or \"\".");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_stay(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LENGTH(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_modulo(&py_str("%s %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }])), "lower", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    Py::None
}
pub fn _the_opening(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The first sentence of a reply: the part that answers the question it was given.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[.!?](?:\\s|$)")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return if py_truthy(&cut.clone()) { slice_or_raise(&text.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "end", args))(&__args) }, &Py::None) } else { text.clone() };
    Py::None
}
pub fn _the_last_answer(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The bot's last reply and the module that gave it.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&reply.clone()) {
    return py_tuple(vec![reply.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
}
pub fn _a_subject_of_its_own(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("Whether the length asked names something of its own (\"the drive\", \"the crumble\").");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GENERIC(), &word.clone())); if py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "isdigit", args))(&__args) }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
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
    let mut last: Py = Py::None;
    let mut length: Py = Py::None;
    let mut module: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stay: Py = Py::None;
    let _ = py_str("The reply to a length asked of a stay in this chat, or None for every other line.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_A_LENGTH(), "search", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_subject_of_its_own(args))(&__args) }) {
    return Py::None;
    }
    stay = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_stay(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&stay.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_last_answer(args))(&__args) }, 2, None);
    last = __unpk1[0].clone();
    module = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&module.clone(), &py_str("answers"))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_opening(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LENGTH_SAID(), "search", args))(&__args) }) {
    return Py::None;
    }
    length = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_length_said(args))(&__args) };
    if py_truthy(&length.clone()) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("length".to_string(), length.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_LENGTH_REPLY()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), length.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }, "format", args))(&__args) };
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_ADVICE(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ADVICE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), stay.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NO_LENGTH()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), stay.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.how_long_a_stay".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADVICE" => Some(g_ADVICE()),
            "ASKS_ADVICE" => Some(g_ASKS_ADVICE()),
            "ASKS_A_LENGTH" => Some(g_ASKS_A_LENGTH()),
            "A_LENGTH" => Some(g_A_LENGTH()),
            "A_LENGTH_REPLY" => Some(g_A_LENGTH_REPLY()),
            "A_LENGTH_SAID" => Some(g_A_LENGTH_SAID()),
            "GENERIC" => Some(g_GENERIC()),
            "NO_LENGTH" => Some(g_NO_LENGTH()),
            "PRIORITY" => Some(g_PRIORITY()),
            "STAY" => Some(g_STAY()),
            "_a_subject_of_its_own" => Some(py_func("_a_subject_of_its_own", |args: &[Py]| _a_subject_of_its_own(args))),
            "_the_last_answer" => Some(py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args))),
            "_the_length_said" => Some(py_func("_the_length_said", |args: &[Py]| _the_length_said(args))),
            "_the_opening" => Some(py_func("_the_opening", |args: &[Py]| _the_opening(args))),
            "_the_stay" => Some(py_func("_the_stay", |args: &[Py]| _the_stay(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADVICE" => s_ADVICE(v),
            "ASKS_ADVICE" => s_ASKS_ADVICE(v),
            "ASKS_A_LENGTH" => s_ASKS_A_LENGTH(v),
            "A_LENGTH" => s_A_LENGTH(v),
            "A_LENGTH_REPLY" => s_A_LENGTH_REPLY(v),
            "A_LENGTH_SAID" => s_A_LENGTH_SAID(v),
            "GENERIC" => s_GENERIC(v),
            "NO_LENGTH" => s_NO_LENGTH(v),
            "PRIORITY" => s_PRIORITY(v),
            "STAY" => s_STAY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.how_long_a_stay", module_obj);
    pyrt::xrefs::reg_fn("modules.how_long_a_stay", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("modules.how_long_a_stay", "_the_stay", py_func("_the_stay", |args: &[Py]| _the_stay(args)));
    pyrt::xrefs::reg_fn("modules.how_long_a_stay", "_the_length_said", py_func("_the_length_said", |args: &[Py]| _the_length_said(args)));
    pyrt::xrefs::reg_fn("modules.how_long_a_stay", "_the_opening", py_func("_the_opening", |args: &[Py]| _the_opening(args)));
    pyrt::xrefs::reg_fn("modules.how_long_a_stay", "_the_last_answer", py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args)));
    pyrt::xrefs::reg_fn("modules.how_long_a_stay", "_a_subject_of_its_own", py_func("_a_subject_of_its_own", |args: &[Py]| _a_subject_of_its_own(args)));
    pyrt::xrefs::reg_fn("modules.how_long_a_stay", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "ADVICE", g_ADVICE, s_ADVICE);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "ASKS_ADVICE", g_ASKS_ADVICE, s_ASKS_ADVICE);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "ASKS_A_LENGTH", g_ASKS_A_LENGTH, s_ASKS_A_LENGTH);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "A_LENGTH", g_A_LENGTH, s_A_LENGTH);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "A_LENGTH_REPLY", g_A_LENGTH_REPLY, s_A_LENGTH_REPLY);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "A_LENGTH_SAID", g_A_LENGTH_SAID, s_A_LENGTH_SAID);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "GENERIC", g_GENERIC, s_GENERIC);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "NO_LENGTH", g_NO_LENGTH, s_NO_LENGTH);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.how_long_a_stay", "STAY", g_STAY, s_STAY);
}
