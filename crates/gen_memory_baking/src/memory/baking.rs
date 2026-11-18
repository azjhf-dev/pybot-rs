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


thread_local! { static G_APPLIANCES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LACKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_BAKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.baking: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_APPLIANCES() -> Py { ensure_init(); G_APPLIANCES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPLIANCES(v: Py) { ensure_init(); G_APPLIANCES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BAKE() -> Py { ensure_init(); G_BAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAKE(v: Py) { ensure_init(); G_BAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_LACKS() -> Py { ensure_init(); G_LACKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LACKS(v: Py) { ensure_init(); G_LACKS.with(|c| *c.borrow_mut() = v); }
pub fn g_LONGEST() -> Py { ensure_init(); G_LONGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONGEST(v: Py) { ensure_init(); G_LONGEST.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_BAKES() -> Py { ensure_init(); G_NAMED_BAKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_BAKES(v: Py) { ensure_init(); G_NAMED_BAKES.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_WORD() -> Py { ensure_init(); G_NAME_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_WORD(v: Py) { ensure_init(); G_NAME_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL() -> Py { ensure_init(); G_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL(v: Py) { ensure_init(); G_TAIL.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that names a kind of cake is a question about that cake, not about its vegetable.\n\n\"im thinking of baking a cake\" then \"what about a carrot cake\" was answered with green\nvegetables, because the word the entries key on is \"carrot\" and the entry for carrots is a\nside dish. The same line about a coffee cake fell to the search hedge, and \"what about lemon\ndrizzle\" reached the entry about drizzling a sauce over a joint. The person is still in the\ncake they named three words ago: written out as \"how do i make a carrot cake\" the line\nreaches the cake entries, which answer it.\n\nOnly an ask takes this route - a report (\"i baked a carrot cake\") or a taste (\"i love carrot\ncake\") is not a how-to question, and the turn stays with whatever answers a report. Anything\nthat is not a compound name of a bake has no line here and passes to the entries untouched.\n");
    s_NAMED_BAKES(stdlib_call("re", "compile", &[py_str("\\b(?:lemon|orange)\\s+drizzle\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BAKE(stdlib_call("re", "compile", &[py_str("\\b(cakes?|loaves|loaf|bread|muffins?|cupcakes?|sponge|buns?|scones?|biscuits?|traybake|traybakes?|brownies?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK(stdlib_call("re", "compile", &[py_str("^(?:so\\s+|ok\\s+|okay\\s+|and\\s+)?(?:(?:what|how)\\s+about\\s+|(?:and|or|plus)\\s+|(?:can|could|should|shall|would)\\s+(?:i|we)\\s+(?:make|bake|do|cook)\\s+|what\\s+if\\s+(?:i|we)\\s+(?:made|make|bake|baked|cooked|cook)\\s+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL(stdlib_call("re", "compile", &[py_str("\\s+(?:instead|too|as well|then|please|later|tonight|now)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LONGEST(Py::Int(4));
    s_NAME_WORD(stdlib_call("re", "compile", &[py_str("^[a-z][a-z'-]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_APPLIANCES(py_str("oven|grill|hob|microwave|air\\s*fryer|cooker|stove|range|pressure\\s*cooker|slow\\s*cooker|toaster"));
    s_LACKS(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:no|without|not\\s+got|dont\\s+have|don't\\s+have|cant\\s+use|can't\\s+use)\\s+(?:a\\s+|an\\s+|any\\s+|working\\s+|my\\s+)?(?:"), &g_APPLIANCES()), &py_str(")\\b|\\b(?:")), &g_APPLIANCES()), &py_str(")\\s+(?:is\\s+|has\\s+|has\\s+gone\\s+)?(?:broken|gone|died|had\\s+it|packed\\s+in)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut noun: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "noun") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: noun"))) };
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The name of the bake in the line without the ask around it, or \"\" when it is not one.\n\n    \"a carrot cake\" is kept as it stands so the written-out ask reads as English. Words that\n    cannot be part of a bake's name - a verb, a place, a clause - take the whole thing out,\n    because \"how do i make bake it for an hour\" is not a question any entry answers.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&noun.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_TAIL(), "match", &[op_add(&py_str(" "), &geti(&words.clone(), &op_neg(&Py::Int(1))))]) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    named = call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("some"), py_str("another")]), &mcall(&geti(&named.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    let _ = mcall(&named.clone(), "pop", &[Py::Int(0)]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LONGEST()) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&named.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NAME_WORD(), "match", &[mcall(&word.clone(), "strip", &[py_str(".,!?;")])])))) {
    return py_str("");
    }
    }
    return mcall(&mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(".,!?;:")]), "lower", &[]);
    Py::None
}
pub fn written_out(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut line: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The line written out as a how-to about the bake it names, or None to leave it alone.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "strip", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::None;
    }
    asked = mcall(&g_ASK(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    rest = slice_or_raise(&line.clone(), &mcall(&asked.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_BAKE(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAMED_BAKES(), "search", &[rest.clone()]) } }))) {
    return Py::None;
    }
    noun = _the_name(&[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&noun.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_LACKS(), "search", &[noun.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_BAKE(), "search", &[noun.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAMED_BAKES(), "search", &[noun.clone()]) } }))) {
    return Py::None;
    }
    return op_add(&py_str("how do i make "), &noun.clone());
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.baking".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "APPLIANCES" => Some(g_APPLIANCES()),
            "ASK" => Some(g_ASK()),
            "BAKE" => Some(g_BAKE()),
            "LACKS" => Some(g_LACKS()),
            "LONGEST" => Some(g_LONGEST()),
            "NAMED_BAKES" => Some(g_NAMED_BAKES()),
            "NAME_WORD" => Some(g_NAME_WORD()),
            "TAIL" => Some(g_TAIL()),
            "_the_name" => Some(py_func("_the_name", |args: &[Py]| _the_name(args))),
            "written_out" => Some(py_func("written_out", |args: &[Py]| written_out(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "APPLIANCES" => s_APPLIANCES(v),
            "ASK" => s_ASK(v),
            "BAKE" => s_BAKE(v),
            "LACKS" => s_LACKS(v),
            "LONGEST" => s_LONGEST(v),
            "NAMED_BAKES" => s_NAMED_BAKES(v),
            "NAME_WORD" => s_NAME_WORD(v),
            "TAIL" => s_TAIL(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.baking", module_obj);
    pyrt::xrefs::reg_fn("memory.baking", "_the_name", py_func("_the_name", |args: &[Py]| _the_name(args)));
    pyrt::xrefs::reg_fn("memory.baking", "written_out", py_func("written_out", |args: &[Py]| written_out(args)));
    pyrt::xrefs::reg_access("memory.baking", "APPLIANCES", g_APPLIANCES, s_APPLIANCES);
    pyrt::xrefs::reg_access("memory.baking", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("memory.baking", "BAKE", g_BAKE, s_BAKE);
    pyrt::xrefs::reg_access("memory.baking", "LACKS", g_LACKS, s_LACKS);
    pyrt::xrefs::reg_access("memory.baking", "LONGEST", g_LONGEST, s_LONGEST);
    pyrt::xrefs::reg_access("memory.baking", "NAMED_BAKES", g_NAMED_BAKES, s_NAMED_BAKES);
    pyrt::xrefs::reg_access("memory.baking", "NAME_WORD", g_NAME_WORD, s_NAME_WORD);
    pyrt::xrefs::reg_access("memory.baking", "TAIL", g_TAIL, s_TAIL);
}
