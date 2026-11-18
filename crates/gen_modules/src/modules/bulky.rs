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


thread_local! { static G_DISPOSAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ELECTRICAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINISHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLOOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FURNITURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GARDEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPS_ITS_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_OURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPLACING: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_DISPOSAL() -> Py { ensure_init(); G_DISPOSAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISPOSAL(v: Py) { ensure_init(); G_DISPOSAL.with(|c| *c.borrow_mut() = v); }
pub fn g_ELECTRICAL() -> Py { ensure_init(); G_ELECTRICAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ELECTRICAL(v: Py) { ensure_init(); G_ELECTRICAL.with(|c| *c.borrow_mut() = v); }
pub fn g_FINISHED() -> Py { ensure_init(); G_FINISHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINISHED(v: Py) { ensure_init(); G_FINISHED.with(|c| *c.borrow_mut() = v); }
pub fn g_FLOOR() -> Py { ensure_init(); G_FLOOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLOOR(v: Py) { ensure_init(); G_FLOOR.with(|c| *c.borrow_mut() = v); }
pub fn g_FURNITURE() -> Py { ensure_init(); G_FURNITURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FURNITURE(v: Py) { ensure_init(); G_FURNITURE.with(|c| *c.borrow_mut() = v); }
pub fn g_GARDEN() -> Py { ensure_init(); G_GARDEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GARDEN(v: Py) { ensure_init(); G_GARDEN.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPS_ITS_ANSWER() -> Py { ensure_init(); G_KEEPS_ITS_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPS_ITS_ANSWER(v: Py) { ensure_init(); G_KEEPS_ITS_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_OURS() -> Py { ensure_init(); G_NOT_OURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_OURS(v: Py) { ensure_init(); G_NOT_OURS.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING() -> Py { ensure_init(); G_POINTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING(v: Py) { ensure_init(); G_POINTING.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_REPLACING() -> Py { ensure_init(); G_REPLACING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPLACING(v: Py) { ensure_init(); G_REPLACING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing too big for the bin is a bulky waste job, and the composer had nothing for it.\n\n\"What do i do with old carpet\" and \"how do i get rid of an old trampoline\" reached nothing: the\nknowledge base has the mattress, the wardrobe, the washing machine and the paint tin, but not the\ncarpet or the trampoline, so the composer fell back to \"I'm not sure about that one, sorry - I'd\nrather not guess. A quick search online will get you a solid answer\". The person is left holding\nthe thing and a search box.\n\nThe answer is the same three routes any bulky thing takes: the council's bulky waste collection,\nthe recycling centre, or a charity that collects if the thing is still usable. The first step is\nthe one that makes it moveable - roll a carpet, take a wardrobe apart - and the fee is the\ncouncil's, so the reply says where to check it rather than inventing a number.\n\nThis sits above the greeting band (PRIORITY -1000) and below every knowledge module, so it only\never answers a line no entry reached. A mattress, a wardrobe, a washing machine and a tin of paint\nkeep their own answers. The thing can be in the line or in the line before it (\"we are replacing\nthe carpet\" / \"what do i do with the old one\"), because a pronoun names no thing on its own.\n");
    s_PRIORITY(op_neg(&Py::Int(999)));
    s_FLOOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:carpets?|carpet\\s+tiles?|underlay|lino(?:leum)?|vinyl\\s+flooring|vinyl\\s+tiles?|flooring|doormats?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FURNITURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:sofas?|settees?|armchairs?|mattress(?:es)?|beds?|wardrobes?|tables?|chairs?|desks?|bookcases?|bookshelves?|shelving|cabinets?|sideboards?|chests?\\s+of\\s+drawers|furniture)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ELECTRICAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:fridges?|freezers?|washing\\s+machines?|dishwashers?|tumble\\s+dryers?|ovens?|cookers?|microwaves?|tvs?|televisions?|monitors?|computers?|laptops?|printers?|kettles?|hoovers?|vacuum\\s+cleaners?|boilers?|radiators?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GARDEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:trampolines?|sheds?|fences?|patios?|slabs?|soil|bushes|bush|trees?|garden\\s+waste|green\\s+waste)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DISPOSAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what\\s+(?:do|should|can)\\s+(?:i|we)\\s+do\\s+with|how\\s+(?:do|can|should)\\s+(?:i|we)\\s+(?:get\\s+rid\\s+of|dispose\\s+of|throw\\s+(?:away|out))|where\\s+(?:can|do|should)\\s+(?:i|we)\\s+(?:take|put|get\\s+rid\\s+of)|get\\s+rid\\s+of|dispose\\s+of|throw\\s+(?:away|out)|take\\s+(?:it\\s+)?to\\s+the\\s+tip|goes?\\s+in\\s+the\\s+bin|recycle)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REPLACING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:replacing|pulling\\s+up|taking\\s+up|ripping\\s+up|(?:having|getting)\\s+(?:the\\s+|a\\s+)?[a-z ]{0,12}(?:replaced|lifted))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FINISHED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:old|broken|worn|worn\\s+out|tired|replaced|replacing|ripped|stained|damp|smelly|had\\s+it|no\\s+good|unwanted|spare|left\\s?over|pulled\\s+up)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POINTING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|that|this|one|them|those|these)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_OURS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:asbestos|needles?|syringe|chemicals?|acid|gas\\s+(?:bottle|cylinder)|oil|paint|solvent|pesticide|weedkiller|medical|clinical)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_KEEPS_ITS_ANSWER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:clean|cleaning|wash|washing\\s+it|stain|smell|moth|moths|remove|mend|repair|fit|lay|choose|choosing|buy|buying)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
pub fn _group(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("floor"), g_FLOOR()]), py_tuple(vec![py_str("furniture"), g_FURNITURE()]), py_tuple(vec![py_str("electrical"), g_ELECTRICAL()]), py_tuple(vec![py_str("garden"), g_GARDEN()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&found.clone()) {
    return py_tuple(vec![name.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut group: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The thing the line is about, from this line or the one before it.");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _group(args))(&__args) }, 2, None);
    group = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    if py_truthy(&thing.clone()) {
    return py_tuple(vec![group.clone(), thing.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_POINTING(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    said = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    let __unpk3 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _group(args))(&__args) }, 2, None);
    group = __unpk3[0].clone();
    thing = __unpk3[1].clone();
    if py_truthy(&thing.clone()) {
    return py_tuple(vec![group.clone(), thing.clone()]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
}
pub fn _reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut group: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "group") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: group"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    if py_truthy(&Py::Bool(py_eq(&group.clone(), &py_str("floor")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("Roll the {0} up and cut it into lengths you can lift, then take it to the recycling centre - most tips take {0} in the general waste skip. If it will not fit in a car, book a bulky waste collection with your council; most charge twenty to forty pounds for a doorstep pickup. Check your council's site for the fee and the booking, because I can't see your council's charges from here."), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&group.clone(), &py_str("furniture")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("Take the {0} apart if it will come apart, then book a bulky waste collection with your council - most charge twenty to forty pounds - or take it to the recycling centre. If it is still usable, a charity or a local reuse group may collect it; the British Heart Foundation collects furniture in many areas. Check your council's site for the fee and the collection, because I can't see your council's charges from here."), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_eq(&group.clone(), &py_str("electrical")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("Anything with a plug or a battery is electrical waste, so it cannot go in the bin: a shop that sells the same kind of thing has to take it back, and the recycling centre has an electricals point. A {0} is also a bulky collection if you cannot move it - ask the shop delivering a new one to take the old one. Check your council's site for the fee, because I can't see your council's charges from here."), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("Dismantle the {0} as far as it will come apart and take it to the recycling centre, or book a bulky waste collection with your council; most charge twenty to forty pounds. The metal goes in the scrap metal skip and the rest in general waste, and green garden waste goes in the council's green bin. Check your council's site for the fee and any permit, because I can't see your council's charges from here."), "format", args))(&__args) };
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
    let mut clean: Py = Py::None;
    let mut group: Py = Py::None;
    let mut thing: Py = Py::None;
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DISPOSAL(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPLACING(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_OURS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_KEEPS_ITS_ANSWER(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing(args))(&__args) }, 2, None);
    group = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPLACING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DISPOSAL(), "search", args))(&__args) })) } }) {
    if py_truthy(&Py::Bool(!py_eq(&group.clone(), &py_str("floor")))) {
    return Py::None;
    }
    return py_str("Good. What are you doing with the old one - tip or a council collection?");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FINISHED(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_POINTING(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(group.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reply(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.bulky".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "DISPOSAL" => Some(g_DISPOSAL()),
            "ELECTRICAL" => Some(g_ELECTRICAL()),
            "FINISHED" => Some(g_FINISHED()),
            "FLOOR" => Some(g_FLOOR()),
            "FURNITURE" => Some(g_FURNITURE()),
            "GARDEN" => Some(g_GARDEN()),
            "KEEPS_ITS_ANSWER" => Some(g_KEEPS_ITS_ANSWER()),
            "NOT_OURS" => Some(g_NOT_OURS()),
            "POINTING" => Some(g_POINTING()),
            "PRIORITY" => Some(g_PRIORITY()),
            "REPLACING" => Some(g_REPLACING()),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_group" => Some(py_func("_group", |args: &[Py]| _group(args))),
            "_reply" => Some(py_func("_reply", |args: &[Py]| _reply(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "DISPOSAL" => s_DISPOSAL(v),
            "ELECTRICAL" => s_ELECTRICAL(v),
            "FINISHED" => s_FINISHED(v),
            "FLOOR" => s_FLOOR(v),
            "FURNITURE" => s_FURNITURE(v),
            "GARDEN" => s_GARDEN(v),
            "KEEPS_ITS_ANSWER" => s_KEEPS_ITS_ANSWER(v),
            "NOT_OURS" => s_NOT_OURS(v),
            "POINTING" => s_POINTING(v),
            "PRIORITY" => s_PRIORITY(v),
            "REPLACING" => s_REPLACING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.bulky", module_obj);
    pyrt::xrefs::reg_fn("modules.bulky", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("modules.bulky", "_group", py_func("_group", |args: &[Py]| _group(args)));
    pyrt::xrefs::reg_fn("modules.bulky", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("modules.bulky", "_reply", py_func("_reply", |args: &[Py]| _reply(args)));
    pyrt::xrefs::reg_fn("modules.bulky", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.bulky", "DISPOSAL", g_DISPOSAL, s_DISPOSAL);
    pyrt::xrefs::reg_access("modules.bulky", "ELECTRICAL", g_ELECTRICAL, s_ELECTRICAL);
    pyrt::xrefs::reg_access("modules.bulky", "FINISHED", g_FINISHED, s_FINISHED);
    pyrt::xrefs::reg_access("modules.bulky", "FLOOR", g_FLOOR, s_FLOOR);
    pyrt::xrefs::reg_access("modules.bulky", "FURNITURE", g_FURNITURE, s_FURNITURE);
    pyrt::xrefs::reg_access("modules.bulky", "GARDEN", g_GARDEN, s_GARDEN);
    pyrt::xrefs::reg_access("modules.bulky", "KEEPS_ITS_ANSWER", g_KEEPS_ITS_ANSWER, s_KEEPS_ITS_ANSWER);
    pyrt::xrefs::reg_access("modules.bulky", "NOT_OURS", g_NOT_OURS, s_NOT_OURS);
    pyrt::xrefs::reg_access("modules.bulky", "POINTING", g_POINTING, s_POINTING);
    pyrt::xrefs::reg_access("modules.bulky", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.bulky", "REPLACING", g_REPLACING, s_REPLACING);
}
