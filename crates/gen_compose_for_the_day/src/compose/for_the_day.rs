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


thread_local! { static G_A_COLD_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_WEIGHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_COLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_HOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_COLD_DAY() -> Py { ensure_init(); G_A_COLD_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COLD_DAY(v: Py) { ensure_init(); G_A_COLD_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_ASKED() -> Py { ensure_init(); G_A_DAY_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_ASKED(v: Py) { ensure_init(); G_A_DAY_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_WEIGHED() -> Py { ensure_init(); G_A_DAY_WEIGHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_WEIGHED(v: Py) { ensure_init(); G_A_DAY_WEIGHED.with(|c| *c.borrow_mut() = v); }
pub fn g_POOLS() -> Py { ensure_init(); G_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOLS(v: Py) { ensure_init(); G_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_COLD() -> Py { ensure_init(); G_TOO_COLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_COLD(v: Py) { ensure_init(); G_TOO_COLD.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_HOT() -> Py { ensure_init(); G_TOO_HOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_HOT(v: Py) { ensure_init(); G_TOO_HOT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A question about today's weather that is really a decision: what decides it, then the figure.\n\n`is it too hot to run` is not a forecast - the bot cannot see today's sky and never will - but it\nis not unanswerable either: what is too hot is the same in every case, and that part is worth\nsaying before naming the place that has the number. The search hedge, which was the reply, says\nnothing about heat at all. A laundry pool was written beside it and cut, because on today's main\nevery washing line is answered by a card before this is reached and the pool could never fire.\n\nIt only stands in for the hedge: it is consulted where nothing else claimed the line, and it says\nnothing at all unless the line names an activity the day is being weighed against.\n");
    s_A_DAY_WEIGHED(stdlib_call("re", "compile", &[py_str("\\b(?:too|warm enough|mild enough|dry enough|cold enough|good enough)\\s+(?:hot|cold|wet|windy|warm|cool|dry|muggy|humid)?\\s*(?:to|for)\\s+(?:go\\s+)?(?:run|running|walk|walking|cycle|cycling|jog|jogging|hike|hiking|play|playing|gardening|garden|barbecue|bbq|swim|swimming|paint|painting|the\\s+dog|a\\s+run|a\\s+walk|the\\s+wash)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_ASKED(stdlib_call("re", "compile", &[py_str("\\b(?:is|will|would|should)\\s+it\\s+(?:be\\s+)?(?:too\\s+|warm\\s+enough|dry\\s+enough|cold\\s+enough|nice\\s+enough\\s+)?(?:hot|cold|wet|windy|warm|muggy|cool|dry)\\b(?:\\s+(?:out|outside|today|tomorrow|later|this\\s+\\w+))?\\s*\\??\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TOO_HOT(py_tuple(vec![py_str("What is too hot is personal, but the thing that catches people out is the muggy air rather than the number: above the mid twenties with no breeze, go early or go shorter, and take water even if you do not feel like it. A weather app has today's figure and the humidity - what time were you thinking of going?"), py_str("Heat is judged on the air as well as the temperature: still, humid and in the sun is a lot harder than the same number with a breeze. If it is close or over about 25 the answer is early, shorter, and water. The app will have the figure for today - morning or evening for you?")]));
    s_TOO_COLD(py_tuple(vec![py_str("Cold is the easier one to get wrong: it is wind and wet that turn a cold day into a bad one, and a dry still frost is fine walking weather once you are moving. Two thin layers and something for your ears beat one thick coat. The app will have today's wind chill - how long are you out for?"), py_str("What decides a cold day is the wind, not the number: the same 3 degrees is fine in a still street and bitter on a hill. Layers you can open when you warm up, and gloves even when it does not look like it. A weather app has the wind chill for today.")]));
    s_A_COLD_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:cold|freezing|chilly|frost\\w*|ice|icy|snow\\w*|wind\\w*|gale\\w*|blowing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POOLS(py_tuple(vec![g_TOO_HOT(), g_TOO_COLD()]));
}
pub fn for_the_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { Py::None };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The pool of lines for a day weighed against an activity, or None.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), repaired.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_A_DAY_WEIGHED(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DAY_ASKED(), "search", &[mcall(&said.clone(), "strip", &[])]) } }) {
    return if py_truthy(&mcall(&g_A_COLD_DAY(), "search", &[said.clone()])) { g_TOO_COLD() } else { g_TOO_HOT() };
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.for_the_day".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_COLD_DAY" => Some(g_A_COLD_DAY()),
            "A_DAY_ASKED" => Some(g_A_DAY_ASKED()),
            "A_DAY_WEIGHED" => Some(g_A_DAY_WEIGHED()),
            "POOLS" => Some(g_POOLS()),
            "TOO_COLD" => Some(g_TOO_COLD()),
            "TOO_HOT" => Some(g_TOO_HOT()),
            "for_the_ask" => Some(py_func("for_the_ask", |args: &[Py]| for_the_ask(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_COLD_DAY" => s_A_COLD_DAY(v),
            "A_DAY_ASKED" => s_A_DAY_ASKED(v),
            "A_DAY_WEIGHED" => s_A_DAY_WEIGHED(v),
            "POOLS" => s_POOLS(v),
            "TOO_COLD" => s_TOO_COLD(v),
            "TOO_HOT" => s_TOO_HOT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.for_the_day", module_obj);
    pyrt::xrefs::reg_fn("compose.for_the_day", "for_the_ask", py_func("for_the_ask", |args: &[Py]| for_the_ask(args)));
    pyrt::xrefs::reg_access("compose.for_the_day", "A_COLD_DAY", g_A_COLD_DAY, s_A_COLD_DAY);
    pyrt::xrefs::reg_access("compose.for_the_day", "A_DAY_ASKED", g_A_DAY_ASKED, s_A_DAY_ASKED);
    pyrt::xrefs::reg_access("compose.for_the_day", "A_DAY_WEIGHED", g_A_DAY_WEIGHED, s_A_DAY_WEIGHED);
    pyrt::xrefs::reg_access("compose.for_the_day", "POOLS", g_POOLS, s_POOLS);
    pyrt::xrefs::reg_access("compose.for_the_day", "TOO_COLD", g_TOO_COLD, s_TOO_COLD);
    pyrt::xrefs::reg_access("compose.for_the_day", "TOO_HOT", g_TOO_HOT, s_TOO_HOT);
}
