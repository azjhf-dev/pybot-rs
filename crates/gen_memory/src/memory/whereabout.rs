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


thread_local! { static G_ASKS_FOR_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALIFIED_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKS_FOR_A_PLACE() -> Py { ensure_init(); G_ASKS_FOR_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_A_PLACE(v: Py) { ensure_init(); G_ASKS_FOR_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); G_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); G_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_LINE() -> Py { ensure_init(); G_PLACE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_LINE(v: Py) { ensure_init(); G_PLACE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALIFIED_LINE() -> Py { ensure_init(); G_QUALIFIED_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALIFIED_LINE(v: Py) { ensure_init(); G_QUALIFIED_LINE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A place said on its own is the question just asked, in that place.\n\n    any ideas for a cheap day out with kids\n      -> Free ones that work: a walk with a purpose (a park, a canal, the woods) ...\n    near leeds\n      -> I'm listening - say more if it helps.                              (main)\n\n\"near leeds\" is not news about the person, and it is not a question of its own. It is the question\nthey asked a turn ago with a place put on it, and the knowledge base answers that question: \"Day\ntrips from Leeds: York (25 minutes by train), the Yorkshire Dales (an hour by car), Harrogate for\na spa afternoon\". Main had nothing to do with the place, so it handed the line to the filler that\nasks for more, which is the one thing a place is not.\n\nSo this reads the place out of the line, puts the question back with the place in it, and hands\nthat back as the line to answer. It decides nothing about the reply: the caller answers the\nquestion it returns the way it answers any other question, and drops it when the answer it gets is\nthe hedge or the answer already on screen. That last part is what keeps a place said after a\nquestion no place can narrow (\"im knackered\" then \"near leeds\") from being answered as if it were\nthe question.\n\nA place is not only a name. \"somewhere warm and cheap\" after \"i need a holiday\" is the same\nshape said as a wish: it is the question with what they want put on it, and the knowledge base\nanswers the two together (\"The cheapest sun is the Algarve or the Costa Blanca outside July and\nAugust\"). Main answered the wish with \"Thanks for telling me - I'm here if you want to dig into\nany of that.\"\n\nThe rule is narrow on purpose:\n* the line has to be a place and nothing else - \"near leeds\", \"somewhere near leeds\", \"in\n  birmingham\", \"round here\" - so \"its near leeds and the traffic was awful\" stays a line of its\n  own, and a bare \"leeds\" is left alone because a town on its own is as likely to be a plan as a\n  place to go;\n* the place has to be a place, so \"in june\", \"in the morning\", \"near the oven\" and \"in bed\" are\n  not read as one;\n* there has to be a question in the turn just finished, and one a place narrows (\"where\", \"any\n  ideas\", \"what should i do\", \"can you recommend\"), so a place said after a statement is left\n  alone;\n* the place must be one the answer on screen does not already say, so an answer that named Leeds\n  is not the whole question asked again.\n");
    s_PLACE_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:its|it'?s|it is|were|we'?re|we are|im|i'?m|i am|somewhere|anywhere|out)\\s+)?(?:right\\s+)?(?P<lead>near|round|around|close to|not far from|outside of|outside|by|in)\\s+(?P<place>[a-z][a-z' .-]{1,28})$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUALIFIED_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:somewhere|anywhere|out somewhere|away somewhere)\\s+(?P<place>[a-z][a-z' -]{2,28})$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_PLACE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\njanuary february march april may june july august september october november december\njan feb mar apr jun jul aug sep sept oct nov dec\nmonday tuesday wednesday thursday friday saturday sunday\nmorning afternoon evening night nights week weekend month year\nday days time times moment minute hour\nbed home house garden kitchen oven bath bedroom lounge hall office car van toilet sink\nhere there it this that\nfact general particular short long end total all once first last\nfuture public private secret common advance writing person people\nprogress practice tears pain shock doubt error trouble\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_FOR_A_PLACE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:where|somewhere|anywhere|place|places|ideas?|recommend|suggest(?:ion)?|days? out|day trip|holiday|holidays|weekend away|getaway|trip|travel|visit|stay|eat out|restaurant|pub|cafe|coffee shop|walk|see|explore|go out|going out|things to do|whats on)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn the_place(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut place: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The place a line that is only a place says, or \"\" for any other line.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5)) } }) {
    return py_str("");
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PLACE_LINE(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUALIFIED_LINE(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    }
    place = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("place")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return py_str("");
    }
    first = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&place.clone(), "split", args))(&__args) }, &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_PLACE(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_PLACE(), &place.clone())) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &first.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&place.clone(), "startswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_PLACE(), &slice_or_raise(&place.clone(), &Py::Int(4), &Py::None, &Py::None))) } }) {
    return py_str("");
    }
    return place.clone();
    Py::None
}
pub fn the_question_with_the_place(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The question this line puts a place on, with the place in it; or \"\".\n\n    The question is looked for in the three turns before this one, not only the last, because a\n    place is often said after something else about the answer: \"any ideas for a cheap day out with\n    kids\" then \"its raining\" then \"near leeds\" is still the day-out question with a place on it,\n    and the answer the bot gave to the rain (\"I'm listening - say more if it helps\") is the one\n    thing a place is not.\n    ");
    place = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_place(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return py_str("");
    }
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_FOR_A_PLACE(), "search", args))(&__args) })) } }) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&place.clone(), "split", args))(&__args) }, &Py::Int(0))))) {
    return py_str("");
    }
    return op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "rstrip", args))(&__args) }, &py_str(" ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .!")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "strip", args))(&__args) });
    }
    return py_str("");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.whereabout".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_FOR_A_PLACE" => Some(g_ASKS_FOR_A_PLACE()),
            "NOT_A_PLACE" => Some(g_NOT_A_PLACE()),
            "PLACE_LINE" => Some(g_PLACE_LINE()),
            "QUALIFIED_LINE" => Some(g_QUALIFIED_LINE()),
            "the_place" => Some(py_func("the_place", |args: &[Py]| the_place(args))),
            "the_question_with_the_place" => Some(py_func("the_question_with_the_place", |args: &[Py]| the_question_with_the_place(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS_FOR_A_PLACE" => s_ASKS_FOR_A_PLACE(v),
            "NOT_A_PLACE" => s_NOT_A_PLACE(v),
            "PLACE_LINE" => s_PLACE_LINE(v),
            "QUALIFIED_LINE" => s_QUALIFIED_LINE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.whereabout", module_obj);
    pyrt::xrefs::reg_fn("memory.whereabout", "the_place", py_func("the_place", |args: &[Py]| the_place(args)));
    pyrt::xrefs::reg_fn("memory.whereabout", "the_question_with_the_place", py_func("the_question_with_the_place", |args: &[Py]| the_question_with_the_place(args)));
    pyrt::xrefs::reg_access("memory.whereabout", "ASKS_FOR_A_PLACE", g_ASKS_FOR_A_PLACE, s_ASKS_FOR_A_PLACE);
    pyrt::xrefs::reg_access("memory.whereabout", "NOT_A_PLACE", g_NOT_A_PLACE, s_NOT_A_PLACE);
    pyrt::xrefs::reg_access("memory.whereabout", "PLACE_LINE", g_PLACE_LINE, s_PLACE_LINE);
    pyrt::xrefs::reg_access("memory.whereabout", "QUALIFIED_LINE", g_QUALIFIED_LINE, s_QUALIFIED_LINE);
}
