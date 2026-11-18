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


thread_local! { static G_ANIMALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALL_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEPING_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEPING_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANIMALS() -> Py { ensure_init(); G_ANIMALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMALS(v: Py) { ensure_init(); G_ANIMALS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__ALL_NAMES() -> Py { ensure_init(); G__ALL_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALL_NAMES(v: Py) { ensure_init(); G__ALL_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS() -> Py { ensure_init(); G__ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS(v: Py) { ensure_init(); G__ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__AVOID() -> Py { ensure_init(); G__AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AVOID(v: Py) { ensure_init(); G__AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEPING_ASKS() -> Py { ensure_init(); G__KEEPING_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEPING_ASKS(v: Py) { ensure_init(); G__KEEPING_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEPING_AVOID() -> Py { ensure_init(); G__KEEPING_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEPING_AVOID(v: Py) { ensure_init(); G__KEEPING_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("An animal somebody keeps is a pet, not a cut of meat.\n\nThe pantry answers a staple by name and the storage rows answer a \"keep\", so the lines about a\nkept animal were answered as if it were dinner: \"how do i look after a rabbit\" got the braise\n(\"a rabbit wants either a long braise or a fast roast\"), \"what do i feed a rabbit\" a method for\ncooking one, \"how do i keep chickens\" the shelf life of cooked chicken, \"how do i look after a\nduck\" the watering of a houseplant, and \"how do i look after a goat\" a punt.\n\nWhat makes the line a keeper's is the pair: an animal people keep, and one of the care words that\nbelongs to keeping it. Each animal has its own basics card - what to house it in, what to feed it\nand the one thing a new keeper gets wrong - and the pan words are stood aside so \"how do i cook a\nrabbit\" and \"rabbit stew\" stay with the kitchen.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    fn __data1() -> Py { py_tuple(vec![py_str("look after"), py_str("looking after"), py_str("care"), py_str("caring"), py_str("care for"), py_str("keep"), py_str("keeping"), py_str("own"), py_str("owning"), py_str("feed"), py_str("feeding"), py_str("fed"), py_str("what to feed"), py_str("hutch"), py_str("coop"), py_str("pen"), py_str("shed"), py_str("house"), py_str("housing"), py_str("run"), py_str("need"), py_str("needs"), py_str("new"), py_str("starter"), py_str("basic"), py_str("basics"), py_str("setup"), py_str("set up"), py_str("want"), py_str("wants"), py_str("getting"), py_str("start with"), py_str("how many"), py_str("how much"), py_str("space"), py_str("room"), py_str("size"), py_str("licence"), py_str("license"), py_str("register"), py_str("permit"), py_str("as a pet"), py_str("a pet"), py_str("my pet")]) }
    s__KEEPING_ASKS(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("cooked"), py_str("roast"), py_str("roasted"), py_str("recipe"), py_str("recipes"), py_str("curry"), py_str("stew"), py_str("stews"), py_str("braise"), py_str("bake"), py_str("baked"), py_str("grill"), py_str("fry"), py_str("fried"), py_str("raw"), py_str("sick"), py_str("sickness"), py_str("ill"), py_str("illness"), py_str("unwell"), py_str("vet"), py_str("dying"), py_str("died"), py_str("dead"), py_str("injury"), py_str("injured"), py_str("limping"), py_str("lice"), py_str("mites"), py_str("worms"), py_str("worming"), py_str("price"), py_str("cost"), py_str("costs"), py_str("buy"), py_str("buying"), py_str("sell"), py_str("selling"), py_str("worth"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("left out"), py_str("leftover"), py_str("leftovers"), py_str("store"), py_str("storage"), py_str("eating"), py_str("eat it"), py_str("to eat"), py_str("for dinner"), py_str("on the menu")]) }
    s__KEEPING_AVOID(__data2());
    s__AVOID(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__KEEPING_AVOID()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__KEEPING_ASKS()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("braise"), py_str("braised"), py_str("roast"), py_str("jointed"), py_str("saddle"), py_str("bacon"), py_str("wine"), py_str("stock"), py_str("bones"), py_str("feed"), py_str("feeding"), py_str("fed"), py_str("what to feed"), py_str("diet")]) }
    s_ANIMALS(py_list(vec![py_tuple(vec![py_tuple(vec![py_str("rabbit"), py_str("rabbits"), py_str("bunny"), py_str("bunnies"), py_str("doe"), py_str("buck")]), py_str("rabbit-care"), py_str("A hutch the size of a small shed, not the one in the shop: a rabbit needs a run it can stand up in and stretch out in, plus a covered sleeping box, and it wants company - two rabbits, a neutered pair, is kinder than one. Hay all day, a handful of greens, a spoon of pellets, and water in a bottle rather than a bowl. Handle it every day from the start: a rabbit that is picked up rarely will scratch to get down."), __data3()]), py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("chickens"), py_str("hen"), py_str("hens"), py_str("ex battery hens"), py_str("ex-battery"), py_str("cockerel"), py_str("rooster"), py_str("coop"), py_str("hen house"), py_str("henhouse")]), py_str("hen-basics"), py_str("Three hens is the smallest sensible flock, in a coop with a nest box and a perch each, and a fox-proof run of about a square metre per hen. Layers' pellets, grit and clean water, and shut them in at dusk - the fox comes at night, and a hen that is out is a hen that is gone."), py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("laying"), py_str("lay"), py_str("lays"), py_str("broody"), py_str("meat"), py_str("breast"), py_str("thigh"), py_str("skin")])]), py_tuple(vec![py_tuple(vec![py_str("duck"), py_str("ducks"), py_str("drake"), py_str("drakes")]), py_str("duck-basics"), py_str("Ducks want a house to sleep in and water deep enough to dip a whole beak, so a tub they can climb out of does a pond's job. Layers' pellets and grit, bedding changed often because they are far messier than hens, and the same shutting in at dusk."), py_tuple(vec![py_str("confit"), py_str("pancake"), py_str("pancakes"), py_str("orange")])]), py_tuple(vec![py_tuple(vec![py_str("goose"), py_str("geese"), py_str("gosling"), py_str("goslings")]), py_str("goose-basics"), py_str("Geese want grass to graze, a shelter they can walk into and a lot of space, and two at the least. They are good watch animals and very messy, and a gander will chase people in the breeding season, so keep them away from a path small children use."), py_tuple(vec![py_str("christmas"), py_str("foie"), py_str("down"), py_str("fat")])]), py_tuple(vec![py_tuple(vec![py_str("goat"), py_str("goats"), py_str("kid goat"), py_str("kid goats"), py_str("pygmy goat"), py_str("billy"), py_str("nanny goat")]), py_str("goat-basics"), py_str("Goats are herd animals, so two at the least: one on its own is a distressed goat. They want a dry shed, a fence they cannot lean on and something to climb, hay and browse all day, a mineral lick, and their feet trimmed every couple of months."), py_tuple(vec![py_str("milk"), py_str("cheese"), py_str("curd"), py_str("meat"), py_str("butter"), py_str("yoghurt"), py_str("yogurt")])]), py_tuple(vec![py_tuple(vec![py_str("quail"), py_str("quails")]), py_str("quail-basics"), py_str("Quails want a hutch or aviary with a solid floor, a deep layer of bedding and somewhere to hide, and a pair or a small group rather than one. Gamebird crumb with extra calcium for layers, and a low roof: they fly straight up when startled."), py_tuple(vec![py_str("meat"), py_str("roast")])]), py_tuple(vec![py_tuple(vec![py_str("pigeon"), py_str("pigeons"), py_str("racing pigeons"), py_str("fantails")]), py_str("pigeon-basics"), py_str("Pigeons want a loft with perches, a landing board and open sky in front of it, and a pair is happier than one. A pigeon mix, grit and clean water, and out at the same time each day so they learn the way home."), py_tuple(vec![py_str("shoot"), py_str("shooting"), py_str("pest"), py_str("roof"), py_str("spikes"), py_str("poo"), py_str("droppings")])])]));
    s__ALL_NAMES({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ANIMALS()) { let __unpk5231 = py_unpack(&__item.clone(), 4, None); let mut names: Py = __unpk5231[0].clone(); let mut _card: Py = __unpk5231[1].clone(); let mut _reply: Py = __unpk5231[2].clone(); let mut _extra: Py = __unpk5231[3].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(name.clone()); } } } py_list(__out) })() });
    s_ENTRIES({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ANIMALS()) { let __unpk3407 = py_unpack(&__item.clone(), 4, None); let mut names: Py = __unpk3407[0].clone(); let mut card: Py = __unpk3407[1].clone(); let mut reply: Py = __unpk3407[2].clone(); let mut extra: Py = __unpk3407[3].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(card.clone()); __args.push(py_tuple(vec![names.clone(), g__KEEPING_ASKS()])); __args.push(reply.clone()); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g__KEEPING_AVOID(), &call_builtin("tuple", &[extra.clone()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALL_NAMES()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&names.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))), ("bias".to_string(), Py::Int(2)), ("guard".to_string(), py_func("keeping_an_animal", |args: &[Py]| keeping_an_animal(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() });
}
pub fn keeping_an_animal(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line asks how to keep an animal, rather than how to cook one.\n\n        how do i look after a rabbit   -> a keeper's question, not the braise\n        what do i feed a rabbit        -> a keeper's question\n        how do i keep chickens         -> not the shelf life of cooked chicken\n        how do i cook a rabbit         -> the kitchen keeps this line\n        rabbit stew                    -> and this one\n        how long can i keep chicken in the fridge -> cooked chicken, not a hen\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    text = mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__AVOID(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__ASKS(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.kept_animals".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANIMALS" => Some(g_ANIMALS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ALL_NAMES" => Some(g__ALL_NAMES()),
            "_ASKS" => Some(g__ASKS()),
            "_AVOID" => Some(g__AVOID()),
            "_KEEPING_ASKS" => Some(g__KEEPING_ASKS()),
            "_KEEPING_AVOID" => Some(g__KEEPING_AVOID()),
            "entry" => Some(g_entry()),
            "keeping_an_animal" => Some(py_func("keeping_an_animal", |args: &[Py]| keeping_an_animal(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANIMALS" => s_ANIMALS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ALL_NAMES" => s__ALL_NAMES(v),
            "_ASKS" => s__ASKS(v),
            "_AVOID" => s__AVOID(v),
            "_KEEPING_ASKS" => s__KEEPING_ASKS(v),
            "_KEEPING_AVOID" => s__KEEPING_AVOID(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.kept_animals", module_obj);
    pyrt::xrefs::reg_fn("knowledge.kept_animals", "keeping_an_animal", py_func("keeping_an_animal", |args: &[Py]| keeping_an_animal(args)));
    pyrt::xrefs::reg_access("knowledge.kept_animals", "ANIMALS", g_ANIMALS, s_ANIMALS);
    pyrt::xrefs::reg_access("knowledge.kept_animals", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.kept_animals", "_ALL_NAMES", g__ALL_NAMES, s__ALL_NAMES);
    pyrt::xrefs::reg_access("knowledge.kept_animals", "_ASKS", g__ASKS, s__ASKS);
    pyrt::xrefs::reg_access("knowledge.kept_animals", "_AVOID", g__AVOID, s__AVOID);
    pyrt::xrefs::reg_access("knowledge.kept_animals", "_KEEPING_ASKS", g__KEEPING_ASKS, s__KEEPING_ASKS);
    pyrt::xrefs::reg_access("knowledge.kept_animals", "_KEEPING_AVOID", g__KEEPING_AVOID, s__KEEPING_AVOID);
    pyrt::xrefs::reg_access("knowledge.kept_animals", "entry", g_entry, s_entry);
}
