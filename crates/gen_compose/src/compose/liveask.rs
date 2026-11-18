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


thread_local! { static G_LIVE_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVE_DECIDE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTE_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_MARK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_LIVE_CHECK() -> Py { ensure_init(); G_LIVE_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_CHECK(v: Py) { ensure_init(); G_LIVE_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_DECIDE() -> Py { ensure_init(); G_LIVE_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_DECIDE(v: Py) { ensure_init(); G_LIVE_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVE_DECIDE_ASKS() -> Py { ensure_init(); G_LIVE_DECIDE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVE_DECIDE_ASKS(v: Py) { ensure_init(); G_LIVE_DECIDE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTE_KIND() -> Py { ensure_init(); G_NOTE_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTE_KIND(v: Py) { ensure_init(); G_NOTE_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_MARK() -> Py { ensure_init(); G_NO_MARK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_MARK(v: Py) { ensure_init(); G_NO_MARK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A decision that turns on live information: the one-tap check, then the weighing.\n\nThe composer's live note is the honest answer to \"what time does the shop close today\" - it says\nthe bot cannot see live hours and names the page that can - and it is the wrong answer on its own\nto \"should i go before it shuts\". That line asks for a decision, and the decision turns on the one\nthing the bot cannot see, so a reply that only says where to look leaves the question on the floor:\n\n    should i take a coat tomorrow\n    No live weather here, sorry. A forecast app or site will tell you in one tap, and the averages\n    are worth a look for a trip later in the year.\n\nNothing in that is wrong and nothing in it is an answer. STYLE.md's \"Not knowing, honestly\" has\nthe shape that is: say what the bot cannot see, name the one place that knows, and stay with the\nperson for the rest of it. So the kind's own check is kept, in one step, and the weighing is handed\nback in words the person can answer:\n\n    I can't see the weather from here, sorry. A weather app has the next few days in one tap, with\n    the feels-like temperature. Nobody can call that but you - say what you'd lose either way and\n    I'll lay both out with you.\n\nWhat is written here is where each kind of live information is found and how a decision is offered;\nno fact about the world is added, because the bot was asked precisely the thing it cannot see.\n\nThis is a last resort: `bot/modules/greeting.py` asks it after the composer, and it stands down\nunless the composer's reply is one of its own live notes (`fallback.LIVE_KINDS`), so a line the\ncomposer answers with anything else - or a line any entry or module answered - never reaches it.\nAdding the note rather than replacing it would be the safer-looking change and the worse one: two\ntips at the end of one reply (\"a forecast app will tell you in one tap\" ... \"a weather app has the\nnext few days in one tap\") reads as a machine talking to itself.\n");
    s_LIVE_DECIDE(py_tuple(vec![py_str("I can't see the {kind} from here, sorry. {check} Then it's your call, and I can help you weigh it: what would each way cost you?"), py_str("I can't see the {kind} from here, sorry. {check} Nobody can call that but you — say what you'd lose either way and I'll lay both out with you.")]));
    s_LIVE_CHECK(py_dict(vec![(py_str("weather"), py_tuple(vec![py_str("A weather app has the next few days in one tap, with the feels-like temperature."), py_str("weather")])), (py_str("hours"), py_tuple(vec![py_str("The venue's own page or its map listing has today's hours, holiday changes and all."), py_str("opening times")])), (py_str("prices"), py_tuple(vec![py_str("What it costs turns on which one it is, where you buy it and the day, and the seller's own site has today's figure."), py_str("prices")])), (py_str("news"), py_tuple(vec![py_str("A news site has the latest."), py_str("news")])), (py_str("travel"), py_tuple(vec![py_str("The operator's app or a journey planner has the next one and any disruption."), py_str("times or delays")]))]));
    s_LIVE_DECIDE_ASKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:should i|shall i|is it worth|is it better|would it be better|do i need|do you think i should|what would you do|not sure|unsure|dont know|do not know|no idea|cant decide|torn between|in two minds|whether to|if i can)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOTE_KIND({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&crate::compose::fallback::g_LIVE_KINDS()) { let __unpk6776 = py_unpack(&__item.clone(), 3, None); let mut kind: Py = __unpk6776[0].clone(); let mut _phrases: Py = __unpk6776[1].clone(); let mut pool: Py = __unpk6776[2].clone(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push((line.clone(), kind.clone())); } } py_dict(__out) })() });
    s_NO_MARK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("['\\u2019`]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    else { py_str("") };
    let mut check: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut options: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The check and the weighing for a decision about live information, or None.\n\n    `reply` is what the composer has already said. Only one of its live notes is replaced, so the\n    file can neither take a turn off a better answer nor answer a line nobody asked a decision\n    about.\n    ");
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOTE_KIND(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); __args.push(py_tuple(vec![py_str(""), py_str("")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LIVE_CHECK(), "get", args))(&__args) }, 2, None);
    check = __unpk1[0].clone();
    word = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&check.clone()))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NO_MARK(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LIVE_DECIDE_ASKS(), "search", args))(&__args) }))) {
    return Py::None;
    }
    options = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LIVE_DECIDE()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("check".to_string(), check.clone()), ("kind".to_string(), word.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    read = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); __args.push(read.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.liveask".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "LIVE_CHECK" => Some(g_LIVE_CHECK()),
            "LIVE_DECIDE" => Some(g_LIVE_DECIDE()),
            "LIVE_DECIDE_ASKS" => Some(g_LIVE_DECIDE_ASKS()),
            "NOTE_KIND" => Some(g_NOTE_KIND()),
            "NO_MARK" => Some(g_NO_MARK()),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "LIVE_CHECK" => s_LIVE_CHECK(v),
            "LIVE_DECIDE" => s_LIVE_DECIDE(v),
            "LIVE_DECIDE_ASKS" => s_LIVE_DECIDE_ASKS(v),
            "NOTE_KIND" => s_NOTE_KIND(v),
            "NO_MARK" => s_NO_MARK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.liveask", module_obj);
    pyrt::xrefs::reg_fn("compose.liveask", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.liveask", "LIVE_CHECK", g_LIVE_CHECK, s_LIVE_CHECK);
    pyrt::xrefs::reg_access("compose.liveask", "LIVE_DECIDE", g_LIVE_DECIDE, s_LIVE_DECIDE);
    pyrt::xrefs::reg_access("compose.liveask", "LIVE_DECIDE_ASKS", g_LIVE_DECIDE_ASKS, s_LIVE_DECIDE_ASKS);
    pyrt::xrefs::reg_access("compose.liveask", "NOTE_KIND", g_NOTE_KIND, s_NOTE_KIND);
    pyrt::xrefs::reg_access("compose.liveask", "NO_MARK", g_NO_MARK, s_NO_MARK);
}
