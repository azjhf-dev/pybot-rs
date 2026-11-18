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


thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAT_IN_A_CARD_SWAPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAT_OR_FISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAT_IN_A_CARD_SWAPS() -> Py { ensure_init(); G_MEAT_IN_A_CARD_SWAPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAT_IN_A_CARD_SWAPS(v: Py) { ensure_init(); G_MEAT_IN_A_CARD_SWAPS.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAT_OR_FISH() -> Py { ensure_init(); G_MEAT_OR_FISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAT_OR_FISH(v: Py) { ensure_init(); G_MEAT_OR_FISH.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The meat a card uses as an ingredient, when the vegetarian version is one word.\n\nEvery row in the kitchen is written for everyone, and a row that is about something else can still\nname meat in passing: \"how do i make gravy\" is answered by a card that starts \"Whisk the flour into\nthe meat juices off the heat before adding stock\". `cooking.fits_the_diet` drops any card with a\nmeat word in it, which is right for a card that is *about* the meat - \"how do i roast a chicken\"\nhas nothing to offer a vegetarian - and wrong for a card where the meat is the fat in the pan. The\nperson in `long-chat-05` said they were vegetarian at turn two and asked how to make gravy at turn\nten, and got nothing.\n\n`DAIRY_SWAPS` in `cooking` already does this one layer in: \"softened in butter\" is served as\n\"softened in oil\" rather than thrown away. This is the same shape at the meat layer, where the swap\nis one word and the instruction does not change:\n\n    meat juices                      -> vegetable juices     (the same pan, same deglazing)\n    chicken / beef / ham stock       -> vegetable stock       (the same liquid, same amount)\n    chicken / beef / ham stock cubes -> vegetable stock cubes\n\nTwo things are deliberately not here. Nothing is swapped on a line that names that food itself:\n\"can i freeze chicken stock\" is a question about chicken stock, which is the pot and not the\nquestion of what they eat. And nothing is swapped in a clause already talking about the plant\nversion, so \"vegetable stock instead of chicken stock\" keeps both halves.\n\nA card that is really about a cut of meat is not rescued by this: it still says \"roast the\nchicken\" after the swap, and `fits_the_diet` refuses it exactly as before.\n");
    s_ENTRIES(py_list(vec![]));
    s_MEAT_IN_A_CARD_SWAPS(py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bmeat juices\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("vegetable juices")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:chicken|beef|ham|turkey|lamb|pork|fish)\\s+stock\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("vegetable stock")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:chicken|beef|ham|turkey|lamb|pork)\\s+stocks?\\s+cubes?\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("vegetable stock cubes")])]));
    s_MEAT_OR_FISH(py_tuple(vec![py_str("meat"), py_str("fish")]));
}
pub fn swapped_out(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { py_str("") };
    let mut avoided: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "avoided") { v.clone() }
    else { Py::None };
    let mut pattern: Py = Py::None;
    let mut said_here: Py = Py::None;
    let mut swap: Py = Py::None;
    let _ = py_str("The reply with the meat it uses as an ingredient swapped, or `text` unchanged.\n\n    Only ever called for a person who does not eat meat or fish, and only ever widens what they\n    can be served: a card with no meat word in it is untouched, a clause already about the plant\n    version is untouched, and a line that named the food keeps the food.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "strip", args))(&__args) })) } }) {
    return text.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&avoided.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(avoided.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MEAT_OR_FISH()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) } }) {
    return text.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::knowledge::cooking::g_PLANT_SAID(), "search", args))(&__args) }) {
    return text.clone();
    }
    said_here = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    let __iter1 = py_iter_or_raise(&g_MEAT_IN_A_CARD_SWAPS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    swap = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said_here.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    continue;
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(pattern.clone()); __args.push(swap.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::cooking::_rewritten_outside_swaps(args))(&__args) };
    }
    return text.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.meat_in_a_card".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ENTRIES" => Some(g_ENTRIES()),
            "MEAT_IN_A_CARD_SWAPS" => Some(g_MEAT_IN_A_CARD_SWAPS()),
            "MEAT_OR_FISH" => Some(g_MEAT_OR_FISH()),
            "swapped_out" => Some(py_func("swapped_out", |args: &[Py]| swapped_out(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ENTRIES" => s_ENTRIES(v),
            "MEAT_IN_A_CARD_SWAPS" => s_MEAT_IN_A_CARD_SWAPS(v),
            "MEAT_OR_FISH" => s_MEAT_OR_FISH(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.meat_in_a_card", module_obj);
    pyrt::xrefs::reg_fn("knowledge.meat_in_a_card", "swapped_out", py_func("swapped_out", |args: &[Py]| swapped_out(args)));
    pyrt::xrefs::reg_access("knowledge.meat_in_a_card", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.meat_in_a_card", "MEAT_IN_A_CARD_SWAPS", g_MEAT_IN_A_CARD_SWAPS, s_MEAT_IN_A_CARD_SWAPS);
    pyrt::xrefs::reg_access("knowledge.meat_in_a_card", "MEAT_OR_FISH", g_MEAT_OR_FISH, s_MEAT_OR_FISH);
}
