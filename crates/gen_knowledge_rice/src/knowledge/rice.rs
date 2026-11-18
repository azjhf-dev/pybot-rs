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
thread_local! { static G_ANY_SORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISHES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISHE_DEFAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SWAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWAP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.rice: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANY_SORT() -> Py { ensure_init(); G_ANY_SORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANY_SORT(v: Py) { ensure_init(); G_ANY_SORT.with(|c| *c.borrow_mut() = v); }
pub fn g_DISHES() -> Py { ensure_init(); G_DISHES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISHES(v: Py) { ensure_init(); G_DISHES.with(|c| *c.borrow_mut() = v); }
pub fn g_DISHE_DEFAULT() -> Py { ensure_init(); G_DISHE_DEFAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISHE_DEFAULT(v: Py) { ensure_init(); G_DISHE_DEFAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_ASK() -> Py { ensure_init(); G_DISH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_ASK(v: Py) { ensure_init(); G_DISH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMES() -> Py { ensure_init(); G_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMES(v: Py) { ensure_init(); G_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SWAP() -> Py { ensure_init(); G_NOT_A_SWAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SWAP(v: Py) { ensure_init(); G_NOT_A_SWAP.with(|c| *c.borrow_mut() = v); }
pub fn g_SORTS() -> Py { ensure_init(); G_SORTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORTS(v: Py) { ensure_init(); G_SORTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SWAP_ASK() -> Py { ensure_init(); G_SWAP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWAP_ASK(v: Py) { ensure_init(); G_SWAP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Rice, on its own: which sort for a dish, and what stands in for what.\n\nRice was the one everyday food where main gave the same answer to every question. Asking about\nthe rice for a paella got \"Try egg fried rice: fry cold cooked rice in hot oil for 5 minutes...\",\nasking whether brown rice could stand in for white got a paragraph on fibre, and\n\"can i use arborio instead of risotto rice\" got \"That's beyond what I know well, sorry.\"\n\nThe differences are real and they are two numbers each - how much water, how long - so they live\nhere as one table of sorts. A question about a swap or about a dish asks for the two numbers of\nthe sort that is coming out of the cupboard, so one table answers every way of asking it.\n\nThis is its own domain rather than a part of bot/knowledge/cooking.py: the cooking table is the\nbusiest list in the repository, and bot/knowledge/__init__.py takes a domain as one file with\nENTRIES and ANSWERS.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_SORTS(py_dict(vec![(py_str("basmati"), py_dict(vec![(py_str("label"), py_str("basmati")), (py_str("time"), py_str("10 to 12 minutes")), (py_str("water"), py_str("1 part rice to 1.5 parts water")), (py_str("what"), py_str("a long grain that cooks up dry and separate, which is why it suits curry")), (py_str("for"), py_str("curry, biryani and pilau")), (py_str("stands_in"), py_str("If you do not have basmati, jasmine or any long grain cooks the same way at the same water and time."))])), (py_str("jasmine"), py_dict(vec![(py_str("label"), py_str("jasmine rice")), (py_str("time"), py_str("10 to 12 minutes")), (py_str("water"), py_str("1 part rice to 1.5 parts water")), (py_str("what"), py_str("a long grain, softer and a little sweeter than basmati")), (py_str("for"), py_str("Thai food, stir-fries and curry")), (py_str("stands_in"), py_str("If you do not have jasmine, basmati or any long grain cooks the same way at the same water and time."))])), (py_str("long grain"), py_dict(vec![(py_str("label"), py_str("white rice")), (py_str("time"), py_str("10 to 12 minutes")), (py_str("water"), py_str("1 part rice to 1.5 parts water")), (py_str("what"), py_str("the everyday one and the least fussy")), (py_str("for"), py_str("anything")), (py_str("stands_in"), py_str("Basmati or jasmine will do just as well at the same water and time."))])), (py_str("arborio"), py_dict(vec![(py_str("label"), py_str("arborio")), (py_str("time"), py_str("18 to 20 minutes, with hot stock added a ladle at a time")), (py_str("water"), py_str("3 to 4 parts hot stock")), (py_str("what"), py_str("a short grain that sheds starch, so it goes creamy as you stir it")), (py_str("for"), py_str("risotto")), (py_str("stands_in"), py_str("If you do not have arborio, carnaroli is better still, or short grain pudding rice at a push."))])), (py_str("carnaroli"), py_dict(vec![(py_str("label"), py_str("carnaroli")), (py_str("time"), py_str("18 to 20 minutes, with hot stock added a ladle at a time")), (py_str("water"), py_str("3 to 4 parts hot stock")), (py_str("what"), py_str("the best risotto rice: it holds its shape better than arborio and takes a little more stock")), (py_str("for"), py_str("risotto")), (py_str("stands_in"), py_str("If you do not have carnaroli, arborio works, though it goes a little softer."))])), (py_str("bomba"), py_dict(vec![(py_str("label"), py_str("bomba rice")), (py_str("time"), py_str("18 to 20 minutes")), (py_str("water"), py_str("1 part rice to 2.5 or 3 parts stock")), (py_str("what"), py_str("a short grain that swells without bursting, so it stays separate")), (py_str("for"), py_str("paella")), (py_str("stands_in"), py_str("If you do not have bomba, arborio works, though it goes creamier, so do not stir it."))])), (py_str("sushi"), py_dict(vec![(py_str("label"), py_str("sushi rice")), (py_str("time"), py_str("12 to 15 minutes")), (py_str("water"), py_str("1 part rice to 1.2 parts water")), (py_str("what"), py_str("a short grain that goes sticky enough to hold together, then seasoned with vinegar, sugar and salt")), (py_str("for"), py_str("sushi and onigiri")), (py_str("stands_in"), py_str("If you do not have sushi rice, pudding rice will do at a push, though it is stickier still."))])), (py_str("pudding"), py_dict(vec![(py_str("label"), py_str("pudding rice")), (py_str("time"), py_str("25 to 30 minutes")), (py_str("water"), py_str("1 part rice to about 4 parts milk, or half milk and half water")), (py_str("what"), py_str("a short grain that goes soft and creamy in milk")), (py_str("for"), py_str("rice pudding")), (py_str("stands_in"), py_str("If you do not have pudding rice, arborio is very close."))])), (py_str("brown"), py_dict(vec![(py_str("label"), py_str("brown rice")), (py_str("time"), py_str("25 to 30 minutes, then 10 minutes off the heat with the lid on")), (py_str("water"), py_str("1 part rice to 2 parts water")), (py_str("what"), py_str("white rice with the bran left on, which makes it nuttier, chewier and slower to cook")), (py_str("for"), py_str("anything white rice does, if you have the extra twenty minutes")), (py_str("stands_in"), py_str("If you do not have brown rice, white rice works at 1 to 1.5 for 10 to 12 minutes, but it will not taste the same."))])), (py_str("wild"), py_dict(vec![(py_str("label"), py_str("wild rice")), (py_str("time"), py_str("40 to 45 minutes")), (py_str("water"), py_str("1 part rice to 2.5 parts water")), (py_str("what"), py_str("not rice at all but a grass seed: dark, chewy, and usually mixed into a rice rather than eaten alone")), (py_str("for"), py_str("mixing into long grain rice for colour and bite")), (py_str("stands_in"), py_str("There is no real stand-in for wild rice, so leave it out rather than swap it."))])), (py_str("sticky"), py_dict(vec![(py_str("label"), py_str("sticky rice")), (py_str("time"), py_str("20 to 25 minutes in a steamer after an hour's soak")), (py_str("water"), py_str("1 part rice to 1 part water, then steamed rather than boiled")), (py_str("what"), py_str("glutinous rice, which is sticky enough to eat with your fingers")), (py_str("for"), py_str("Thai sticky rice and rice balls")), (py_str("stands_in"), py_str("If you do not have sticky rice, sushi rice is the nearest thing in a supermarket."))]))]));
    s_NAMES(py_dict(vec![(py_str("basmati"), py_tuple(vec![py_str("basmati")])), (py_str("jasmine"), py_tuple(vec![py_str("jasmine"), py_str("jasmine rice"), py_str("thai rice")])), (py_str("long grain"), py_tuple(vec![py_str("long grain"), py_str("long-grain"), py_str("plain rice"), py_str("white rice"), py_str("regular rice")])), (py_str("arborio"), py_tuple(vec![py_str("arborio"), py_str("risotto rice"), py_str("risotto")])), (py_str("carnaroli"), py_tuple(vec![py_str("carnaroli")])), (py_str("bomba"), py_tuple(vec![py_str("bomba"), py_str("paella rice")])), (py_str("sushi"), py_tuple(vec![py_str("sushi rice"), py_str("sushi")])), (py_str("pudding"), py_tuple(vec![py_str("pudding rice"), py_str("rice pudding"), py_str("pudding")])), (py_str("brown"), py_tuple(vec![py_str("brown rice"), py_str("wholegrain rice"), py_str("wholegrain"), py_str("brown")])), (py_str("wild"), py_tuple(vec![py_str("wild rice"), py_str("wild")])), (py_str("sticky"), py_tuple(vec![py_str("sticky rice"), py_str("glutinous rice")]))]));
    s_DISHE_DEFAULT(Py::None);
    s_DISHES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("paella"), py_str("paellas")]), py_str("bomba")]), py_tuple(vec![py_tuple(vec![py_str("risotto"), py_str("risottos")]), py_str("arborio")]), py_tuple(vec![py_tuple(vec![py_str("sushi"), py_str("onigiri"), py_str("maki")]), py_str("sushi")]), py_tuple(vec![py_tuple(vec![py_str("rice pudding"), py_str("pudding")]), py_str("pudding")]), py_tuple(vec![py_tuple(vec![py_str("biryani"), py_str("biriyani"), py_str("pilau"), py_str("pilaf"), py_str("pullao")]), py_str("basmati")]), py_tuple(vec![py_tuple(vec![py_str("curry"), py_str("curries"), py_str("kedgeree"), py_str("kheer"), py_str("dhal"), py_str("dal")]), py_str("basmati")]), py_tuple(vec![py_tuple(vec![py_str("stir fry"), py_str("stir-fry"), py_str("stirfry")]), py_str("jasmine")]), py_tuple(vec![py_tuple(vec![py_str("jollof"), py_str("nasi goreng")]), py_str("long grain")]), py_tuple(vec![py_tuple(vec![py_str("sticky rice"), py_str("mango sticky rice")]), py_str("sticky")])]));
    fn __data1() -> Py { py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("jasmine"), py_str("long grain"), py_str("arborio"), py_str("carnaroli"), py_str("bomba"), py_str("sushi rice"), py_str("pudding rice"), py_str("brown rice"), py_str("wild rice"), py_str("sticky rice"), py_str("glutinous rice"), py_str("paella rice"), py_str("risotto rice"), py_str("wholegrain rice")]) }
    s_ANY_SORT(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("instead"), py_str("instead of"), py_str("swap"), py_str("swaps"), py_str("swapped"), py_str("swap for"), py_str("substitute"), py_str("substitute for"), py_str("stand in"), py_str("stand-in"), py_str("replace"), py_str("replacement"), py_str("alternative"), py_str("or can i use"), py_str("can i use"), py_str("could i use"), py_str("what can i use"), py_str("use for"), py_str("work for"), py_str("works for"), py_str("will it work"), py_str("does it work"), py_str("ok to use"), py_str("okay to use"), py_str("fine to use")]) }
    s_SWAP_ASK(__data2());
    s_DISH_ASK(py_tuple(vec![py_str("what rice"), py_str("which rice"), py_str("best rice"), py_str("right rice"), py_str("rice for"), py_str("rice should i"), py_str("rice do i"), py_str("rice to use"), py_str("rice goes"), py_str("rice with")]));
    fn __data3() -> Py { py_tuple(vec![py_str("how long"), py_str("how much water"), py_str("minutes"), py_str("time"), py_str("keep"), py_str("kept"), py_str("leftover"), py_str("left over"), py_str("reheat"), py_str("freeze"), py_str("frozen"), py_str("healthy"), py_str("healthier"), py_str("health"), py_str("fibre"), py_str("fiber"), py_str("calories"), py_str("calorie"), py_str("weight"), py_str("blood sugar"), py_str("diet"), py_str("diabetes"), py_str("recipe"), py_str("how do i make"), py_str("how do i cook"), py_str("cook it"), py_str("wash"), py_str("rinse"), py_str("mushy"), py_str("soggy"), py_str("burnt"), py_str("burned"), py_str("rice cooker"), py_str("microwave"), py_str("pasta"), py_str("spaghetti"), py_str("noodles"), py_str("noodle"), py_str("potato"), py_str("potatoes"), py_str("quinoa"), py_str("couscous"), py_str("bulgur"), py_str("barley"), py_str("bread"), py_str("cauliflower"), py_str("orzo"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("puppy"), py_str("kitten"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("parrot"), py_str("rabbit"), py_str("hamster"), py_str("fish"), py_str("tortoise"), py_str("horse")]) }
    s_NOT_A_SWAP(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("rice"), py_str("paella"), py_str("paellas"), py_str("risotto"), py_str("risottos"), py_str("sushi"), py_str("onigiri"), py_str("maki"), py_str("rice pudding"), py_str("biryani"), py_str("biriyani"), py_str("pilau"), py_str("pilaf"), py_str("pullao"), py_str("sticky rice"), py_str("mango sticky rice"), py_str("curry"), py_str("curries"), py_str("kedgeree"), py_str("kheer"), py_str("dhal"), py_str("dal"), py_str("stir fry"), py_str("stir-fry"), py_str("stirfry"), py_str("jollof"), py_str("nasi goreng")]) }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rice-swap")); __args.push(py_tuple(vec![g_ANY_SORT(), call_builtin("tuple", &[g_SWAP_ASK()]).unwrap_or_else(|e| exc::raise(e))])); __args.push(py_func("_swap_reply", |args: &[Py]| _swap_reply(args))); __kws.extend(vec![("avoid".to_string(), g_NOT_A_SWAP()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rice-for-dish")); __args.push(py_tuple(vec![__data4(), call_builtin("tuple", &[g_DISH_ASK()]).unwrap_or_else(|e| exc::raise(e))])); __args.push(py_func("_dish_reply", |args: &[Py]| _dish_reply(args))); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_A_SWAP(), &py_tuple(vec![py_str("how do i make"), py_str("how do i cook a"), py_str("recipe for"), py_str("how much"), py_str("per person"), py_str("people"), py_str("each")]))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The line as one lowercase string, for the dish names, which are phrases.");
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]);
    }
    return op_add(&op_add(&py_str(" "), &mcall(&said.clone(), "lower", &[])), &py_str(" "));
    Py::None
}
pub fn _sort_named(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut at: Py = Py::None;
    let mut form: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every sort the line names, in the order the line names them.\n\n    The order matters: in \"can i use jasmine instead of basmati\" the jasmine is what the person\n    is holding and the basmati is what it has to replace, so the first one named is the one the\n    reply has to describe.\n    ");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&{ let __b = names.clone(); if py_truthy(&__b) { __b } else { g_NAMES() } }, "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    words = __unpk2[1].clone();
    at = call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    let __iter4 = py_iter_or_raise(&py_tuple(vec![op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")), op_add(&op_add(&py_str(" "), &word.clone()), &py_str("s "))]));
    for __item in __iter4 {
    form = __item.clone();
    where_ = mcall(&said.clone(), "find", &[form.clone()]);
    if py_truthy(&{ let __b = op_ge(&where_.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { op_lt(&where_.clone(), &at.clone()) } }) {
    at = where_.clone();
    break;
    }
    }
    }
    if py_truthy(&op_lt(&at.clone(), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![at.clone(), key.clone()])]);
    }
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("sorted", &[found.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk6927 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk6927[0].clone(); let mut key: Py = __unpk6927[1].clone(); __out.push(key.clone()); } py_list(__out) })() };
    Py::None
}
pub fn _dish_recent(args: &[Py]) -> Py {
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
    let mut dish: Py = Py::None;
    let mut dish_word: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The dish the last few turns were about, for a follow-up that names no dish of its own.\n\n    \"im making a paella tonight\" then \"what rice should i use\" is one question in two lines: the\n    second turn has no dish word in it at all, so without this it got cooking.py's plain rice pan\n    method and the paella was lost.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    let __unpk2 = py_unpack(&_dish_named(&[said.clone()]), 2, None);
    dish = __unpk2[0].clone();
    dish_word = __unpk2[1].clone();
    if py_truthy(&dish.clone()) {
    return py_tuple(vec![dish.clone(), dish_word.clone()]);
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _dish_named(args: &[Py]) -> Py {
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
    let mut sort: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let __iter1 = py_iter_or_raise(&g_DISHES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    sort = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str("s ")))) } }) {
    return py_tuple(vec![sort.clone(), word.clone()]);
    }
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _swap(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut dish: Py = Py::None;
    let mut dish_word: Py = Py::None;
    let mut h: Py = Py::None;
    let mut held: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut wanted_for_dish: Py = Py::None;
    let _ = py_str("One sort standing in for another, or one sort named for a dish: the numbers that change.");
    said = _said(&[understood.clone()]);
    named = _sort_named(&[said.clone()]);
    let __unpk1 = py_unpack(&_dish_named(&[said.clone()]), 2, None);
    dish = __unpk1[0].clone();
    dish_word = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&dish.clone())); if !py_truthy(&__b) { __b } else { { let __b = named.clone(); if py_truthy(&__b) { __b } else { geti(&_dish_recent(&[memory.clone()]), &Py::Int(0)) } } } }) {
    let __unpk2 = py_unpack(&_dish_recent(&[memory.clone()]), 2, None);
    dish = __unpk2[0].clone();
    dish_word = __unpk2[1].clone();
    }
    wanted_for_dish = mcall(&g_SORTS(), "get", &[dish.clone(), py_dict(vec![])]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&named.clone(), &Py::Int(0)), &geti(&named.clone(), &Py::Int(1)))) } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![geti(&named.clone(), &Py::Int(0)), geti(&named.clone(), &Py::Int(1))]), 2, None);
    held = __unpk3[0].clone();
    wanted = __unpk3[1].clone();
    let __unpk4 = py_unpack(&py_tuple(vec![geti(&g_SORTS(), &held.clone()), geti(&g_SORTS(), &wanted.clone())]), 2, None);
    h = __unpk4[0].clone();
    w = __unpk4[1].clone();
    if py_truthy(&Py::Bool(py_eq(&py_tuple(vec![geti(&h.clone(), &py_str("water")), geti(&h.clone(), &py_str("time"))]), &py_tuple(vec![geti(&w.clone(), &py_str("water")), geti(&w.clone(), &py_str("time"))])))) {
    return op_modulo(&py_str("%s will stand in for %s on the same water and time - %s, %s - and the difference is the grain: it is %s."), &py_tuple(vec![mcall(&geti(&h.clone(), &py_str("label")), "capitalize", &[]), geti(&w.clone(), &py_str("label")), geti(&h.clone(), &py_str("water")), geti(&h.clone(), &py_str("time")), geti(&h.clone(), &py_str("what"))]));
    }
    return op_modulo(&py_str("%s will stand in for %s: it wants %s and %s, where %s wants %s and %s. It is %s, so the dish comes out with that grain instead."), &py_tuple(vec![mcall(&geti(&h.clone(), &py_str("label")), "capitalize", &[]), geti(&w.clone(), &py_str("label")), geti(&h.clone(), &py_str("water")), geti(&h.clone(), &py_str("time")), geti(&w.clone(), &py_str("label")), geti(&w.clone(), &py_str("water")), geti(&w.clone(), &py_str("time")), geti(&h.clone(), &py_str("what"))]));
    }
    if py_truthy(&{ let __b = dish.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&named.clone(), &Py::Int(0)), &dish.clone())) } } } }) {
    w = wanted_for_dish.clone();
    said = op_modulo(&py_str("For %s, %s is the one: %s, %s. It is %s."), &py_tuple(vec![dish_word.clone(), geti(&w.clone(), &py_str("label")), geti(&w.clone(), &py_str("water")), geti(&w.clone(), &py_str("time")), geti(&w.clone(), &py_str("what"))]));
    if py_truthy(&mcall(&w.clone(), "get", &[py_str("stands_in")])) {
    let __aug5 = op_add(&said.clone(), &op_add(&py_str(" "), &geti(&w.clone(), &py_str("stands_in"))));
    said = __aug5;
    }
    return said.clone();
    }
    if py_truthy(&{ let __b = dish.clone(); if !py_truthy(&__b) { __b } else { named.clone() } }) {
    held = geti(&named.clone(), &Py::Int(0));
    let __unpk6 = py_unpack(&py_tuple(vec![geti(&g_SORTS(), &held.clone()), wanted_for_dish.clone()]), 2, None);
    h = __unpk6[0].clone();
    w = __unpk6[1].clone();
    return op_modulo(&py_str("You can use %s in %s, but %s is the one for it: %s, %s. %s is %s, where %s is %s, so the dish comes out with that grain in it."), &py_tuple(vec![geti(&h.clone(), &py_str("label")), dish_word.clone(), geti(&w.clone(), &py_str("label")), geti(&w.clone(), &py_str("water")), geti(&w.clone(), &py_str("time")), mcall(&geti(&w.clone(), &py_str("label")), "capitalize", &[]), geti(&w.clone(), &py_str("what")), geti(&h.clone(), &py_str("label")), geti(&h.clone(), &py_str("what"))]));
    }
    if py_truthy(&dish.clone()) {
    w = wanted_for_dish.clone();
    said = op_modulo(&py_str("For %s, %s is the one: %s, %s. It is %s."), &py_tuple(vec![dish_word.clone(), geti(&w.clone(), &py_str("label")), geti(&w.clone(), &py_str("water")), geti(&w.clone(), &py_str("time")), geti(&w.clone(), &py_str("what"))]));
    if py_truthy(&mcall(&w.clone(), "get", &[py_str("stands_in")])) {
    let __aug7 = op_add(&said.clone(), &op_add(&py_str(" "), &geti(&w.clone(), &py_str("stands_in"))));
    said = __aug7;
    }
    return said.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return op_modulo(&py_str("Plain long grain rice is the safe one for most things: %s, %s. Basmati and jasmine are the same water and time with a different grain, and brown rice wants %s for %s."), &py_tuple(vec![geti(&geti(&g_SORTS(), &py_str("long grain")), &py_str("water")), geti(&geti(&g_SORTS(), &py_str("long grain")), &py_str("time")), geti(&geti(&g_SORTS(), &py_str("brown")), &py_str("water")), geti(&geti(&g_SORTS(), &py_str("brown")), &py_str("time"))]));
    }
    held = geti(&named.clone(), &Py::Int(0));
    h = geti(&g_SORTS(), &held.clone());
    return op_modulo(&py_str("%s works in place of white rice: %s, %s. It is %s."), &py_tuple(vec![mcall(&geti(&h.clone(), &py_str("label")), "capitalize", &[]), geti(&h.clone(), &py_str("water")), geti(&h.clone(), &py_str("time")), geti(&h.clone(), &py_str("what"))]));
    Py::None
}
pub fn _dish(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("Which sort a named dish wants, and the numbers for it.");
    return _swap(&[memory.clone(), understood.clone()]);
    Py::None
}
pub fn _swap_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    return _swap(&[memory.clone(), understood.clone()]);
    Py::None
}
pub fn _dish_reply(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    return _swap(&[memory.clone(), understood.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.rice".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ANY_SORT" => Some(g_ANY_SORT()),
            "DISHES" => Some(g_DISHES()),
            "DISHE_DEFAULT" => Some(g_DISHE_DEFAULT()),
            "DISH_ASK" => Some(g_DISH_ASK()),
            "ENTRIES" => Some(g_ENTRIES()),
            "NAMES" => Some(g_NAMES()),
            "NOT_A_SWAP" => Some(g_NOT_A_SWAP()),
            "SORTS" => Some(g_SORTS()),
            "SWAP_ASK" => Some(g_SWAP_ASK()),
            "_dish" => Some(py_func("_dish", |args: &[Py]| _dish(args))),
            "_dish_named" => Some(py_func("_dish_named", |args: &[Py]| _dish_named(args))),
            "_dish_recent" => Some(py_func("_dish_recent", |args: &[Py]| _dish_recent(args))),
            "_dish_reply" => Some(py_func("_dish_reply", |args: &[Py]| _dish_reply(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_sort_named" => Some(py_func("_sort_named", |args: &[Py]| _sort_named(args))),
            "_swap" => Some(py_func("_swap", |args: &[Py]| _swap(args))),
            "_swap_reply" => Some(py_func("_swap_reply", |args: &[Py]| _swap_reply(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ANY_SORT" => s_ANY_SORT(v),
            "DISHES" => s_DISHES(v),
            "DISHE_DEFAULT" => s_DISHE_DEFAULT(v),
            "DISH_ASK" => s_DISH_ASK(v),
            "ENTRIES" => s_ENTRIES(v),
            "NAMES" => s_NAMES(v),
            "NOT_A_SWAP" => s_NOT_A_SWAP(v),
            "SORTS" => s_SORTS(v),
            "SWAP_ASK" => s_SWAP_ASK(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.rice", module_obj);
    pyrt::xrefs::reg_fn("knowledge.rice", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.rice", "_sort_named", py_func("_sort_named", |args: &[Py]| _sort_named(args)));
    pyrt::xrefs::reg_fn("knowledge.rice", "_dish_recent", py_func("_dish_recent", |args: &[Py]| _dish_recent(args)));
    pyrt::xrefs::reg_fn("knowledge.rice", "_dish_named", py_func("_dish_named", |args: &[Py]| _dish_named(args)));
    pyrt::xrefs::reg_fn("knowledge.rice", "_swap", py_func("_swap", |args: &[Py]| _swap(args)));
    pyrt::xrefs::reg_fn("knowledge.rice", "_dish", py_func("_dish", |args: &[Py]| _dish(args)));
    pyrt::xrefs::reg_fn("knowledge.rice", "_swap_reply", py_func("_swap_reply", |args: &[Py]| _swap_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.rice", "_dish_reply", py_func("_dish_reply", |args: &[Py]| _dish_reply(args)));
    pyrt::xrefs::reg_access("knowledge.rice", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.rice", "ANY_SORT", g_ANY_SORT, s_ANY_SORT);
    pyrt::xrefs::reg_access("knowledge.rice", "DISHES", g_DISHES, s_DISHES);
    pyrt::xrefs::reg_access("knowledge.rice", "DISHE_DEFAULT", g_DISHE_DEFAULT, s_DISHE_DEFAULT);
    pyrt::xrefs::reg_access("knowledge.rice", "DISH_ASK", g_DISH_ASK, s_DISH_ASK);
    pyrt::xrefs::reg_access("knowledge.rice", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.rice", "NAMES", g_NAMES, s_NAMES);
    pyrt::xrefs::reg_access("knowledge.rice", "NOT_A_SWAP", g_NOT_A_SWAP, s_NOT_A_SWAP);
    pyrt::xrefs::reg_access("knowledge.rice", "SORTS", g_SORTS, s_SORTS);
    pyrt::xrefs::reg_access("knowledge.rice", "SWAP_ASK", g_SWAP_ASK, s_SWAP_ASK);
    pyrt::xrefs::reg_access("knowledge.rice", "entry", g_entry, s_entry);
}
