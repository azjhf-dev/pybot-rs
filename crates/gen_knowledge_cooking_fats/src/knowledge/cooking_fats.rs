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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DECIDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALT_FAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DISH_WITH_ITS_OWN_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAT_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IN_THE_PAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_CARD() -> Py { ensure_init(); G_THE_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_CARD(v: Py) { ensure_init(); G_THE_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DECIDER() -> Py { ensure_init(); G_THE_DECIDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DECIDER(v: Py) { ensure_init(); G_THE_DECIDER.with(|c| *c.borrow_mut() = v); }
pub fn g__ALT_FAT() -> Py { ensure_init(); G__ALT_FAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALT_FAT(v: Py) { ensure_init(); G__ALT_FAT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DISH_WITH_ITS_OWN_METHOD() -> Py { ensure_init(); G__A_DISH_WITH_ITS_OWN_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DISH_WITH_ITS_OWN_METHOD(v: Py) { ensure_init(); G__A_DISH_WITH_ITS_OWN_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g__FAT_CHOICE() -> Py { ensure_init(); G__FAT_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAT_CHOICE(v: Py) { ensure_init(); G__FAT_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g__IN_THE_PAN() -> Py { ensure_init(); G__IN_THE_PAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IN_THE_PAN(v: Py) { ensure_init(); G__IN_THE_PAN.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PAN() -> Py { ensure_init(); G__NOT_A_PAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PAN(v: Py) { ensure_init(); G__NOT_A_PAN.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Which fat goes in the pan: butter, oil, or one then the other.\n\n\"Butter or oil for onions\" is a cooking question, and the bot answered it from the fridge: the\n`butter-margarine` row in `nutrition.py` keys `butter` with `or` and is about a scraping on toast,\nso every \"butter or oil\" line got the spread-versus-butter lesson. The other half of the fault was\nworse - \"should i cook onions in butter or oil\" got two recipes, shortbread and onion soup, because\nthe line names two foods and two ingredients and the cooking domain answered with one dish each.\n\nSo the choice of fat is its own question with its own answer, and the answer is a fact about heat:\nbutter burns, oil does not, and which one is better depends on how hot the pan is. A line that is\nnot about a pan passes: `avoid` holds the fridge-and-health words, so \"is butter better than\nmargarine\" and \"how much saturated fat in butter\" keep their own answers.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s__FAT_CHOICE(py_tuple(vec![py_str("butter or oil"), py_str("oil or butter"), py_str("butter or olive oil"), py_str("olive oil or butter"), py_str("butter or margarine"), py_str("margarine or butter"), py_str("which fat"), py_str("what fat"), py_str("which is the best fat")]));
    s__ALT_FAT(mcall(&py_str("|"), "join", &[g__FAT_CHOICE()]));
    fn __data1() -> Py { py_tuple(vec![py_str("fry"), py_str("fries"), py_str("fried"), py_str("frying"), py_str("cook"), py_str("cooks"), py_str("cooked"), py_str("cooking"), py_str("roast"), py_str("roasting"), py_str("bake"), py_str("baking"), py_str("pan"), py_str("sear"), py_str("searing"), py_str("saute"), py_str("sauté"), py_str("stir fry"), py_str("griddle"), py_str("skillet"), py_str("onion"), py_str("onions"), py_str("potato"), py_str("potatoes"), py_str("carrot"), py_str("carrots"), py_str("mushroom"), py_str("mushrooms"), py_str("egg"), py_str("eggs"), py_str("steak"), py_str("chicken"), py_str("sausage"), py_str("sausages"), py_str("cake"), py_str("sauce"), py_str("curry"), py_str("pancake"), py_str("pancakes"), py_str("shallow fry"), py_str("deep fry"), py_str("oven")]) }
    s__IN_THE_PAN(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("healthy"), py_str("healthier"), py_str("unhealthy"), py_str("healthiest"), py_str("cholesterol"), py_str("calories"), py_str("fattening"), py_str("saturated"), py_str("unsaturated"), py_str("good for you"), py_str("bad for you"), py_str("heart"), py_str("diet"), py_str("dieting"), py_str("spread"), py_str("toast"), py_str("on toast"), py_str("sandwich"), py_str("margarine or butter on"), py_str("lurpak"), py_str("how much"), py_str("how many"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("gone off"), py_str("off"), py_str("rancid"), py_str("mouldy"), py_str("mould"), py_str("substitute"), py_str("instead of butter"), py_str("without butter"), py_str("dairy free"), py_str("vegan"), py_str("allergy"), py_str("lactose")]) }
    s__NOT_A_PAN(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("roast potato"), py_str("roast potatoes"), py_str("roasties"), py_str("roast parsnip"), py_str("roast parsnips"), py_str("roast carrot"), py_str("roast carrots"), py_str("roast veg"), py_str("roast vegetables"), py_str("roast squash"), py_str("roast swede"), py_str("roast beetroot"), py_str("roast celeriac")]) }
    s__A_DISH_WITH_ITS_OWN_METHOD(__data3());
    s_THE_CARD(py_str("Oil takes the heat and butter gives the taste: butter starts to burn at about 150C, so for a hot pan or a hot oven use oil and add butter at the end. Butter on its own is for a low pan where nothing goes above that heat - onions, mushrooms, eggs, 15 to 20 minutes on a low heat for the onions - and in a cake oil keeps it moist for days while butter gives the flavour most people expect."));
    s_THE_DECIDER(py_tuple(vec![py_str("For onions either is fine: butter on a low heat for 15 to 20 minutes gives the taste and will not burn below 150C, and oil takes a hotter pan - so oil for a hard fry and butter for the onions."), py_str("Low heat for butter, high heat for oil: onions in butter on a low heat for 15 to 20 minutes sweeten without catching, and a pan hot enough to brown goes to the oil.")]));
    s_ENTRIES(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fat-for-the-pan")); __args.push(py_tuple(vec![g__FAT_CHOICE(), g__IN_THE_PAN()])); __args.push(py_func("_fat_for_the_pan", |args: &[Py]| _fat_for_the_pan(args))); __kws.extend(vec![("avoid".to_string(), op_add(&g__NOT_A_PAN(), &g__A_DISH_WITH_ITS_OWN_METHOD())), ("bias".to_string(), op_neg(&Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _asks_between(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is a choice between two fats for a pan, not a request for a dish.\n\n    \"should i cook onions in butter or oil\" names two foods and a cooking verb, so the meal\n    table answered it with one dish for each - shortbread for the butter and onion soup for the\n    onions - where the person had asked which fat to use. The dish table stands down on this\n    reading and the row below answers it.\n    ");
    low = op_add(&op_add(&py_str(" "), &_fold(&[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&op_add(&py_str(" "), &g__ALT_FAT()), &py_str(" "))) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__IN_THE_PAN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_PAN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _fold(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _last_reply(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("What the bot said last, so the same answer is not given word for word twice.");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns.clone(), &op_neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _fat_for_the_pan(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The card, or - when it is already on the screen - the sentence that decides the pan.");
    said = _last_reply(&[memory.clone()]);
    if py_truthy(&{ let __b = mcall(&said.clone(), "startswith", &[slice_or_raise(&geti(&g_THE_DECIDER(), &Py::Int(0)), &Py::None, &Py::Int(40), &Py::None)]); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "startswith", &[slice_or_raise(&geti(&g_THE_DECIDER(), &Py::Int(1)), &Py::None, &Py::Int(40), &Py::None)]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&said.clone(), "startswith", &[slice_or_raise(&g_THE_CARD(), &Py::None, &Py::Int(40), &Py::None)])) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_THE_DECIDER(), memory.clone()]);
    }
    return g_THE_CARD();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.cooking_fats".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "THE_CARD" => Some(g_THE_CARD()),
            "THE_DECIDER" => Some(g_THE_DECIDER()),
            "_ALT_FAT" => Some(g__ALT_FAT()),
            "_A_DISH_WITH_ITS_OWN_METHOD" => Some(g__A_DISH_WITH_ITS_OWN_METHOD()),
            "_FAT_CHOICE" => Some(g__FAT_CHOICE()),
            "_IN_THE_PAN" => Some(g__IN_THE_PAN()),
            "_NOT_A_PAN" => Some(g__NOT_A_PAN()),
            "_asks_between" => Some(py_func("_asks_between", |args: &[Py]| _asks_between(args))),
            "_fat_for_the_pan" => Some(py_func("_fat_for_the_pan", |args: &[Py]| _fat_for_the_pan(args))),
            "_fold" => Some(py_func("_fold", |args: &[Py]| _fold(args))),
            "_last_reply" => Some(py_func("_last_reply", |args: &[Py]| _last_reply(args))),
            "entry" => Some(g_entry()),
            "pick" => Some(g_pick()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "THE_CARD" => s_THE_CARD(v),
            "THE_DECIDER" => s_THE_DECIDER(v),
            "_ALT_FAT" => s__ALT_FAT(v),
            "_A_DISH_WITH_ITS_OWN_METHOD" => s__A_DISH_WITH_ITS_OWN_METHOD(v),
            "_FAT_CHOICE" => s__FAT_CHOICE(v),
            "_IN_THE_PAN" => s__IN_THE_PAN(v),
            "_NOT_A_PAN" => s__NOT_A_PAN(v),
            "entry" => s_entry(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.cooking_fats", module_obj);
    pyrt::xrefs::reg_fn("knowledge.cooking_fats", "_asks_between", py_func("_asks_between", |args: &[Py]| _asks_between(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_fats", "_fold", py_func("_fold", |args: &[Py]| _fold(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_fats", "_last_reply", py_func("_last_reply", |args: &[Py]| _last_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_fats", "_fat_for_the_pan", py_func("_fat_for_the_pan", |args: &[Py]| _fat_for_the_pan(args)));
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "THE_CARD", g_THE_CARD, s_THE_CARD);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "THE_DECIDER", g_THE_DECIDER, s_THE_DECIDER);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "_ALT_FAT", g__ALT_FAT, s__ALT_FAT);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "_A_DISH_WITH_ITS_OWN_METHOD", g__A_DISH_WITH_ITS_OWN_METHOD, s__A_DISH_WITH_ITS_OWN_METHOD);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "_FAT_CHOICE", g__FAT_CHOICE, s__FAT_CHOICE);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "_IN_THE_PAN", g__IN_THE_PAN, s__IN_THE_PAN);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "_NOT_A_PAN", g__NOT_A_PAN, s__NOT_A_PAN);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.cooking_fats", "pick", g_pick, s_pick);
}
