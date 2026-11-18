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
thread_local! { static G__KITCHEN_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KITCHEN_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.kitchen: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__KITCHEN_FAULT() -> Py { ensure_init(); G__KITCHEN_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KITCHEN_FAULT(v: Py) { ensure_init(); G__KITCHEN_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__KITCHEN_THING() -> Py { ensure_init(); G__KITCHEN_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KITCHEN_THING(v: Py) { ensure_init(); G__KITCHEN_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Kitchen faults that come back as a mirror of what the person said, or as another food's answer.\n\n\"Got it - your marmalade is cloudy\" is not an answer, and three of these lines get one that is\nabout a different thing: \"my pickle is fizzy, is that ok\" was told how long an opened jar keeps,\nand \"my haricot beans are still hard\" was told to pull the string down the seam of runners, which\nis advice about a green bean. Each fault has a cause worth knowing, so each one gets the cause and\nthe fix rather than a set of steps.\n\nNothing here is a keeping, cooking or storing question: `avoid` sends those back to the entries\nthat already answer them, and a line this module does not recognise passes, so another entry keeps\nits turn. A smoking pan, a shrunk pastry and a lumpy custard already have their answers, and their\nwords are not keys here.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("jam"), py_str("marmalade"), py_str("pickle"), py_str("pickles"), py_str("pickled"), py_str("pickled onions"), py_str("pickled onion"), py_str("onions"), py_str("onion"), py_str("beetroot"), py_str("gherkin"), py_str("gherkins"), py_str("cornichon"), py_str("cornichons"), py_str("sauerkraut"), py_str("kimchi"), py_str("jalapeno"), py_str("jalapenos"), py_str("chutney"), py_str("lentils"), py_str("lentil"), py_str("chickpeas"), py_str("chickpea"), py_str("beans"), py_str("haricot"), py_str("butter beans"), py_str("split peas"), py_str("cannellini")]) }
    s__KITCHEN_THING(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("cloudy"), py_str("mouldy"), py_str("mould"), py_str("moldy"), py_str("mold"), py_str("furry"), py_str("fuzz"), py_str("white stuff"), py_str("white bits"), py_str("white film"), py_str("white bloom"), py_str("white specks"), py_str("fizzy"), py_str("bubbling"), py_str("bubbles"), py_str("slimy"), py_str("still hard"), py_str("went hard"), py_str("stay hard"), py_str("stayed hard"), py_str("wont soften"), py_str("won't soften"), py_str("didnt soften"), py_str("didn't soften"), py_str("not softening"), py_str("rock hard"), py_str("too hard")]) }
    s__KITCHEN_FAULT(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("vegetables"), py_str("veg"), py_str("lettuce"), py_str("salad"), py_str("leaves"), py_str("spinach"), py_str("kale"), py_str("cabbage"), py_str("fruit"), py_str("apples"), py_str("grapes"), py_str("tomatoes"), py_str("cucumber"), py_str("herbs"), py_str("potatoes"), py_str("carrots"), py_str("mushrooms"), py_str("berries"), py_str("strawberries")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("macaroni"), py_str("penne"), py_str("fusilli"), py_str("tagliatelle"), py_str("noodles"), py_str("rice"), py_str("how long"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("storage"), py_str("freeze"), py_str("frozen"), py_str("fridge"), py_str("how do i cook"), py_str("boil"), py_str("roast"), py_str("recipe"), py_str("my hands"), py_str("dishes"), py_str("clothes"), py_str("car"), py_str("dog"), py_str("salad dressing"), py_str("which salad"), py_str("apple watch"), py_str("watch"), py_str("watches"), py_str("phone"), py_str("iphone"), py_str("laptop"), py_str("tablet"), py_str("ipad"), py_str("kindle"), py_str("earbuds"), py_str("airpods"), py_str("headphones"), py_str("device"), py_str("remote"), py_str("mushroom"), py_str("mushrooms"), py_str("mud"), py_str("muddy"), py_str("washing up"), py_str("flatmate"), py_str("housemate"), py_str("roommate"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("never helps"), py_str("chores"), py_str("chore"), py_str("leaves it"), py_str("leaves the"), py_str("leaves them"), py_str("left the"), py_str("all the time")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("how long"), py_str("how do i cook"), py_str("how do i make"), py_str("recipe"), py_str("keep"), py_str("freeze"), py_str("fridge"), py_str("soak"), py_str("soaking"), py_str("how many"), py_str("how much"), py_str("should i"), py_str("can i"), py_str("why is my potato"), py_str("sprouted"), py_str("sprouts")]) }
    s_ENTRIES(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wash-produce")); __args.push(py_tuple(vec![__data3(), py_tuple(vec![py_str("wash"), py_str("washing"), py_str("rinse"), py_str("rinsing"), py_str("cleaned"), py_str("dirty"), py_str("muddy"), py_str("soil"), py_str("before cooking"), py_str("before eating")])])); __args.push(py_str("Rinse under cold running water and rub the surface between your hands - a colander under the tap and a shake does most of it, a salad wants its leaves separated and dried in a tea towel, and grapes want the bunch broken up so the water reaches all of them. Soap and washing-up liquid are not for food, and a sealed bag marked ready to eat is already washed.")); __kws.extend(vec![("avoid".to_string(), __data4()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kitchen-fault")); __args.push(py_tuple(vec![g__KITCHEN_THING(), g__KITCHEN_FAULT()])); __args.push(py_func("_kitchen_fault", |args: &[Py]| _kitchen_fault(args))); __kws.extend(vec![("avoid".to_string(), __data5()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _kitchen_fault(args: &[Py]) -> Py {
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
    let mut has: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Three kitchen faults, each with its own cause.\n\n    The jam is scum, the pickle is three answers in one jar - mould, a flat white film and a\n    fizz - and the pulses are about how old they are.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    said = mcall(&py_str(" "), "join", &[words.clone()]);
pub fn __nested_has_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut has: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut says: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut wanted: Py = py_list(__pos.get(7..).unwrap_or(&[]).to_vec());
    let mut want: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&wanted.clone()) { let mut want: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&word.clone(), &want.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[want.clone()]) } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    has = {{ let has = has.clone(); let memory = memory.clone(); let said = said.clone(); let says = says.clone(); let understood = understood.clone(); let word = word.clone(); let words = words.clone(); py_func("has", move |args: &[Py]| { let mut full = vec![has.clone(), memory.clone(), said.clone(), says.clone(), understood.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_has_0(&full) }) }};
pub fn __nested_says_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut has: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut says: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut phrases: Py = py_list(__pos.get(7..).unwrap_or(&[]).to_vec());
    let mut phrase: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    says = {{ let has = has.clone(); let memory = memory.clone(); let said = said.clone(); let says = says.clone(); let understood = understood.clone(); let word = word.clone(); let words = words.clone(); py_func("says", move |args: &[Py]| { let mut full = vec![has.clone(), memory.clone(), said.clone(), says.clone(), understood.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_says_1(&full) }) }};
    if py_truthy(&py_call(&has, &[py_str("jam"), py_str("marmalade"), py_str("chutney")])) {
    if py_truthy(&{ let __b = py_call(&has, &[py_str("cloudy")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("is cloudy"), py_str("looks cloudy"), py_str("went cloudy")]) } }) {
    return py_str("A cloudy jam or marmalade is scum and trapped air, not a fault: it sets and eats exactly the same. Skim the froth off with a slotted spoon as it boils, add a small knob of butter at the end to settle the bubbles, and let it stand five minutes off the heat before potting so the peel does not float. A marmalade made with the whole fruit is cloudy by nature, and only a clear jelly asks for strained juice.");
    }
    }
    if py_truthy(&{ let __b = py_call(&has, &[py_str("pickle"), py_str("pickles"), py_str("chutney")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("white stuff"), py_str("white bits"), py_str("white film")]) } }) {
    if py_truthy(&{ let __b = py_call(&has, &[py_str("fizzy"), py_str("bubbling"), py_str("bubbles")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("is fizzy"), py_str("went fizzy"), py_str("bubbling")]) } }) {
    return py_str("Bubbles and a little fizz in a brine pickle are the fermentation working, and the pickle is fine: lactic acid bacteria give off carbon dioxide as they turn the sugar to acid, and the jar settles down after a week or two. Keep the vegetables under the brine and taste a piece - a sharp, clean sourness is right. If the brine has gone thick and slimy, or it smells of rot rather than vinegar, throw that jar away.");
    }
    if py_truthy(&{ let __b = py_call(&has, &[py_str("white stuff"), py_str("white bits"), py_str("white specks"), py_str("bloom")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("white stuff"), py_str("white bits"), py_str("white specks"), py_str("white film"), py_str("white bloom"), py_str("cloudy brine")]) } }) {
    return py_str("A flat white film or speckle on top of a pickle is usually kahm yeast, which is harmless: skim it off, keep the vegetables under the brine and the pickle is fine. Fuzzy or hairy mould - blue, green, black - is a different thing: throw that jar away, because the mould's threads reach down through the brine and spooning it off the top does not make it safe. A weight that keeps everything under the liquid stops both.");
    }
    if py_truthy(&{ let __b = py_call(&has, &[py_str("mouldy"), py_str("mould"), py_str("moldy"), py_str("mold"), py_str("furry"), py_str("fuzz"), py_str("green"), py_str("blue"), py_str("black"), py_str("hairy")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("mould on"), py_str("mouldy"), py_str("gone mouldy"), py_str("went mouldy")]) } }) {
    return py_str("Throw that jar away: mould on a pickle is not something you can rescue, because the threads reach down through the brine and lifting the patch off the top leaves them behind. The one exception is a flat white film with no fuzz - that is usually kahm yeast, and skimming it off is enough, provided the pickle still smells sour and tastes clean. Keeping the vegetables under the brine with a weight is what stops mould taking hold.");
    }
    }
    if py_truthy(&{ let __b = py_call(&has, &[py_str("lentils"), py_str("lentil"), py_str("chickpeas"), py_str("chickpea"), py_str("beans"), py_str("haricot"), py_str("butter beans"), py_str("split peas")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("split peas"), py_str("haricot beans"), py_str("butter beans"), py_str("cannellini")]) } }) {
    if py_truthy(&{ let __b = py_call(&has, &[py_str("still hard"), py_str("went hard"), py_str("stay hard"), py_str("stayed hard"), py_str("wont soften"), py_str("won't soften"), py_str("didnt soften"), py_str("didn't soften"), py_str("not softening"), py_str("rock hard"), py_str("too hard")]); if py_truthy(&__b) { __b } else { py_call(&says, &[py_str("still hard"), py_str("wont soften"), py_str("won't soften"), py_str("stay hard")]) } }) {
    return py_str("Old pulses are the usual reason: lentils and beans lose the ability to soften as they age, so cook them in plenty of unsalted water, add the salt at the end, and give chickpeas and dried beans an overnight soak and 60 to 90 minutes at a bare simmer.");
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.kitchen".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_KITCHEN_FAULT" => Some(g__KITCHEN_FAULT()),
            "_KITCHEN_THING" => Some(g__KITCHEN_THING()),
            "_kitchen_fault" => Some(py_func("_kitchen_fault", |args: &[Py]| _kitchen_fault(args))),
            "entry" => Some(g_entry()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_KITCHEN_FAULT" => s__KITCHEN_FAULT(v),
            "_KITCHEN_THING" => s__KITCHEN_THING(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.kitchen", module_obj);
    pyrt::xrefs::reg_fn("knowledge.kitchen", "_kitchen_fault", py_func("_kitchen_fault", |args: &[Py]| _kitchen_fault(args)));
    pyrt::xrefs::reg_access("knowledge.kitchen", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.kitchen", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.kitchen", "_KITCHEN_FAULT", g__KITCHEN_FAULT, s__KITCHEN_FAULT);
    pyrt::xrefs::reg_access("knowledge.kitchen", "_KITCHEN_THING", g__KITCHEN_THING, s__KITCHEN_THING);
    pyrt::xrefs::reg_access("knowledge.kitchen", "entry", g_entry, s_entry);
}
