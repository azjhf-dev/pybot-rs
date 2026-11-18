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


thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH() -> Py { ensure_init(); G_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH(v: Py) { ensure_init(); G_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_POOL() -> Py { ensure_init(); G_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOL(v: Py) { ensure_init(); G_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS() -> Py { ensure_init(); G_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS(v: Py) { ensure_init(); G_THEIRS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A figure about something the person made is not a look-up (PRIORITY -997).\n\n\"how many calories in my stew\", \"how many grams of sugar in a flapjack\", \"how many grams in an\napple pie\", \"how many carbs in my dinner\", \"how many calories in a quiche\" all reached the seed\nfallback - \"I'm not sure about that one, sorry - I'd rather not guess. A quick search online will\nget you a solid answer\". Nothing can know those numbers: the recipe, the portion and what went\ninto it are what decide them, and the figures are on the recipe card or the label on the pack.\nA search engine is the wrong instrument for somebody's own stew.\n\nThis is a module rather than a kind in bot/compose/gap.py because that file is shared with every\nlane, and a new file cannot conflict. PRIORITY -997 puts it just above the seed fallback (-1000)\nand below every module that can answer, so it only ever speaks where nothing else did: the food\nrows that know their figures keep their turn.\n\nThe measure shape alone is never enough, which is the whole fence here. A conversion (\"how many\nml in a bath\"), a brand (\"how many calories in a big mac\"), a meal deal and a staple (\"how many\ngrams of rice for two people\") all have the same shape and are all answered elsewhere or not\nthis kind at all, so a line needs a dish word or the person's own cooking and portion. \"How much\nsugar is in a flapjack\" is the same ask with \"much\" in it, so both words are read.\n");
    s_PRIORITY(op_neg(&Py::Int(997)));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\bhow (?:many|much)\\s+(?:grams?|g|kg|calories?|cals?|carbs?|carbohydrate|sugar|salt|protein|fat|fibre|fiber|ml|millilitres?|litres?|ounces?|oz|slices?|portions?|servings?|spoons?|tablespoons?|teaspoons?)\\b[^?!]{0,40}?\\b(?:in|of|per|for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DISH(stdlib_call("re", "compile", &[py_str("\\b(?:stew|casserole|pie|quiche|tart|cake|flapjack|brownie|biscuit|scone|pudding|crumble|lasagne|lasagna|curry|chilli|soup|broth|risotto|paella|pizza|sandwich|wrap|salad|roast|stir ?fry|noodle|dumpling|loaf|bake|smoothie|porridge|granola|muffin|pastry|gravy|sauce|dressing|hummus|dip|pancake|omelette|frittata)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:my|the one i|i made|i baked|i cooked|homemade|home made|left ?overs?)\\b|\\b(?:portion|serving|slice|bowl|plate|helping)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POOL(py_tuple(vec![py_str("The recipe and the portion are what set that, not a general figure: the weights on your own recipe card or the label on the pack, divided by how many it served. Tell me what went in and how many it made, and I will work the sum out with you."), py_str("Nothing here can know that one - it depends what went into it and how much of it you had. Your recipe or the pack has the figures for the whole thing, so halve it or quarter it; give me the amounts and I will do the division.")]));
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
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_FIGURE(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_DISH(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIRS(), "search", &[clean.clone()]) } }))) {
    return Py::None;
    }
    return geti(&g_POOL(), &Py::Int(0));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_figure_of_a_dish".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_FIGURE" => Some(g_A_FIGURE()),
            "DISH" => Some(g_DISH()),
            "POOL" => Some(g_POOL()),
            "PRIORITY" => Some(g_PRIORITY()),
            "THEIRS" => Some(g_THEIRS()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_FIGURE" => s_A_FIGURE(v),
            "DISH" => s_DISH(v),
            "POOL" => s_POOL(v),
            "PRIORITY" => s_PRIORITY(v),
            "THEIRS" => s_THEIRS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_figure_of_a_dish", module_obj);
    pyrt::xrefs::reg_fn("modules.a_figure_of_a_dish", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.a_figure_of_a_dish", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("modules.a_figure_of_a_dish", "DISH", g_DISH, s_DISH);
    pyrt::xrefs::reg_access("modules.a_figure_of_a_dish", "POOL", g_POOL, s_POOL);
    pyrt::xrefs::reg_access("modules.a_figure_of_a_dish", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_figure_of_a_dish", "THEIRS", g_THEIRS, s_THEIRS);
}
