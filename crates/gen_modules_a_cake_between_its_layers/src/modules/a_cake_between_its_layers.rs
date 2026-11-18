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


thread_local! { static G_ANSWERS_INSTEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_CAKE_ROW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FILLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_find: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_understand: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.a_cake_between_its_layers: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS_INSTEAD() -> Py { ensure_init(); G_ANSWERS_INSTEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_INSTEAD(v: Py) { ensure_init(); G_ANSWERS_INSTEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CAKE() -> Py { ensure_init(); G_A_CAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CAKE(v: Py) { ensure_init(); G_A_CAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_BETWEEN() -> Py { ensure_init(); G_BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BETWEEN(v: Py) { ensure_init(); G_BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ASKING() -> Py { ensure_init(); G_NOT_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ASKING(v: Py) { ensure_init(); G_NOT_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_CAKE_ROW() -> Py { ensure_init(); G_NOT_THE_CAKE_ROW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_CAKE_ROW(v: Py) { ensure_init(); G_NOT_THE_CAKE_ROW.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FILLING() -> Py { ensure_init(); G_THE_FILLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FILLING(v: Py) { ensure_init(); G_THE_FILLING.with(|c| *c.borrow_mut() = v); }
pub fn g_find() -> Py { ensure_init(); G_find.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_find(v: Py) { ensure_init(); G_find.with(|c| *c.borrow_mut() = v); }
pub fn g_understand() -> Py { ensure_init(); G_understand.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_understand(v: Py) { ensure_init(); G_understand.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A cake asked what goes between its layers is the filling question, not five other cards.\n\nSix wordings of the same everyday question reached five other rows on main: \"what can i put\nbetween the layers of a cake\" was a punt, \"what cream goes in a victoria sponge\" and \"what goes in\nthe middle of a lemon cake\" were the sponge weights, \"what filling for a coffee cake\" and \"how do\ni fill a cake\" were how long buttercream keeps, and the two splitting lines were worse than punts -\n\"how do i layer a cake\" was the haircut card, whose key holds a bare \"layers\", and \"how do i cut a\ncake in half\" was \"Divide it by 2: half of 46 is 23\".\n\nThe cake is in every one of those lines, so the reading is the two halves of the question: a cake\nnamed, and a word that says *between* - filling, middle, layers, split, half, which cream, which\njam. Asking is part of it: a cake that sank, domed or is raw in the middle is a fault, and \"i cut\nmyself a slice of cake\" is a cut with a knife in it.\n\nThen the matcher is asked what the line reaches, and a row that answers a cake keeps the turn: the\nrecipe for a named cake (\"how do i make a cream cake\"), how long it keeps, how many calories, the\nbuttercream in grams, the tin for a birthday cake. Only the rows that answered the wrong thing\nstand down, listed once in `NOT_THE_CAKE_ROW`, and a punt has nothing to lose.\n\nA module rather than a row because the rows that took these lines - the haircut, the halving sum,\nthe sponge weights - are in files at the repository's size cap and each would need its own avoid.\nThe card is written once here and answers every way of asking it.\n");
    s_find(py_func("find", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "find", args)));
    s_understand(py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_cake_between_its_layers.PRIORITY"); }
    s_PRIORITY(Py::Int(1200));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_cake_between_its_layers.ANSWERS_INSTEAD"); }
    s_ANSWERS_INSTEAD(py_tuple(vec![py_str("cooking"), py_str("food-and-drink"), py_str("home-and-diy")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_cake_between_its_layers.A_CAKE"); }
    s_A_CAKE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:cake|cakes|sponge|gateau|cupcake|cupcakes|victoria sponge|victoria sandwich|madeira|loaf cake|sandwich cake)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_cake_between_its_layers.BETWEEN"); }
    s_BETWEEN(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:filling|fill|filled|between the layers|between the two|in the middle|the middle of|middle of it|layer|layers|layer it|two layers|three layers|split|split it|sandwich it|halve|in half|what cream|which cream|what jam|cream goes|jam goes|goes between|goes in the middle)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_cake_between_its_layers.NOT_ASKING"); }
    s_NOT_ASKING(stdlib_call("re", "compile", &[py_str("\\b(?:why|sank|sink|sunk|sinking|stodgy|dense|heavy|dry|burn|burnt|undercooked|raw|gummy|domed|dropped|didnt rise|didn't rise|flat|stale|birthday|vegan|gluten|egg free|no eggs|cupcake cases|icing|frosting|whip|whipping|myself|himself|herself|a slice|a piece of|how long|keep|keeps|last|freeze|fridge|calorie|calories|how many|many people|serves|feed|tin size|what size tin|instead of the egg)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_cake_between_its_layers.NOT_THE_CAKE_ROW"); }
    s_NOT_THE_CAKE_ROW(call_builtin("frozenset", &[py_tuple(vec![py_str("recipe-lemon-drizzle"), py_str("amount-in-a-dish"), py_str("plan-followup"), py_str("hair-cut-yourself"), py_str("half-of-a-number"), py_str("bakes")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET modules.a_cake_between_its_layers.THE_FILLING"); }
    s_THE_FILLING(py_str("Jam and buttercream, jam first: a thin layer of jam stops the two halves sliding, then buttercream on top of it - about 3 tablespoons of jam and 150g of buttercream for a 20cm cake. Split the sponge by pushing cocktail sticks in at the height you want all the way round, then cut to them with a long serrated knife, turning the cake as you go so the line meets itself. A chocolate cake takes ganache instead - 100g dark chocolate melted into 100ml double cream, cooled until it holds - and fresh cream with fruit is the best filling there is, though it wants making on the morning you eat it."));
}
pub fn asks_about_its_layers(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks where the filling or the split goes in a cake.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![Py::None, py_str("question"), py_str("request"), py_str("")]), &mcall(&said.clone(), "get", &[py_str("kind")])))) {
    return Py::Bool(false);
    }
    text = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![mcall(&said.clone(), "get", &[py_str("clean")]), mcall(&said.clone(), "get", &[py_str("text")])])) { let mut part: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]);
    if py_truthy(&call_builtin("isinstance", &[mcall(&said.clone(), "get", &[py_str("words")]), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    text = op_add(&op_add(&text.clone(), &py_str(" ")), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&said.clone(), &py_str("words"))) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]));
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[])))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_A_CAKE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_BETWEEN(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_ASKING(), "search", &[text.clone()]))) } } } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    said = pyrt::xrefs::xcall("understanding", "understand", &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&asks_about_its_layers(&[said.clone()])))) {
    return Py::None;
    }
    row = _the_row(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&row.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_THE_CAKE_ROW(), &call_builtin("getattr", &[row.clone(), py_str("id"), py_str("")]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    return g_THE_FILLING();
    Py::None
}
pub fn _the_row(args: &[Py]) -> Py {
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
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), pyrt::xrefs::xcall("knowledge.lookup", "find", &[pyrt::xrefs::xcall("understanding", "understand", &[text.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }])])]);
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
    return Py::None;
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
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_cake_between_its_layers".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS_INSTEAD" => Some(g_ANSWERS_INSTEAD()),
            "A_CAKE" => Some(g_A_CAKE()),
            "BETWEEN" => Some(g_BETWEEN()),
            "NOT_ASKING" => Some(g_NOT_ASKING()),
            "NOT_THE_CAKE_ROW" => Some(g_NOT_THE_CAKE_ROW()),
            "PRIORITY" => Some(g_PRIORITY()),
            "THE_FILLING" => Some(g_THE_FILLING()),
            "_the_row" => Some(py_func("_the_row", |args: &[Py]| _the_row(args))),
            "asks_about_its_layers" => Some(py_func("asks_about_its_layers", |args: &[Py]| asks_about_its_layers(args))),
            "find" => Some(g_find()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "understand" => Some(g_understand()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS_INSTEAD" => s_ANSWERS_INSTEAD(v),
            "A_CAKE" => s_A_CAKE(v),
            "BETWEEN" => s_BETWEEN(v),
            "NOT_ASKING" => s_NOT_ASKING(v),
            "NOT_THE_CAKE_ROW" => s_NOT_THE_CAKE_ROW(v),
            "PRIORITY" => s_PRIORITY(v),
            "THE_FILLING" => s_THE_FILLING(v),
            "find" => s_find(v),
            "understand" => s_understand(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_cake_between_its_layers", module_obj);
    pyrt::xrefs::reg_fn("modules.a_cake_between_its_layers", "asks_about_its_layers", py_func("asks_about_its_layers", |args: &[Py]| asks_about_its_layers(args)));
    pyrt::xrefs::reg_fn("modules.a_cake_between_its_layers", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.a_cake_between_its_layers", "_the_row", py_func("_the_row", |args: &[Py]| _the_row(args)));
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "ANSWERS_INSTEAD", g_ANSWERS_INSTEAD, s_ANSWERS_INSTEAD);
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "A_CAKE", g_A_CAKE, s_A_CAKE);
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "BETWEEN", g_BETWEEN, s_BETWEEN);
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "NOT_ASKING", g_NOT_ASKING, s_NOT_ASKING);
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "NOT_THE_CAKE_ROW", g_NOT_THE_CAKE_ROW, s_NOT_THE_CAKE_ROW);
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "THE_FILLING", g_THE_FILLING, s_THE_FILLING);
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "find", g_find, s_find);
    pyrt::xrefs::reg_access("modules.a_cake_between_its_layers", "understand", g_understand, s_understand);
}
