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


thread_local! { static G_DOUBLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IRREGULAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVING_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_DOUBLES() -> Py { ensure_init(); G_DOUBLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBLES(v: Py) { ensure_init(); G_DOUBLES.with(|c| *c.borrow_mut() = v); }
pub fn g_IRREGULAR() -> Py { ensure_init(); G_IRREGULAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IRREGULAR(v: Py) { ensure_init(); G_IRREGULAR.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVING_TO() -> Py { ensure_init(); G_MOVING_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVING_TO(v: Py) { ensure_init(); G_MOVING_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_STEPS() -> Py { ensure_init(); G_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEPS(v: Py) { ensure_init(); G_STEPS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The words after a plan's \"to\": the step itself, or the place it goes to.\n\nTwo lines open the same way and are not the same sentence:\n\n    im going to dye my hair pink        a step, so the phrase said back is \"dyeing your hair pink\"\n    im going to the gym                 a place, so the phrase said back is \"going to the gym\"\n\nTurning whatever follows a \"to\" into an -ing word gave people nonsense, because most of what\nfollows a \"to\" is not a verb: \"is that a good idea\" after \"im going to bed early tonight\" was\nanswered \"Tell me what matters most about bedding early tonight\", \"im going to the gym\" became\n\"Theing gym - that is your call, not mine\", and \"im going to town later\" became \"about towning\nlater\". A noun is not a step, and the fold only belongs on a step.\n\nThe list is a list of steps and not a dictionary of every verb: a word it does not know is said\nback with its own \"going to\" in front (\"going to spain in may\", \"going to the gym\"), which is\nalways the words the person used and always a phrase a sentence can hold. The failure is then a\nslightly longer phrase, never a word that does not exist.\n");
    s_STEPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    abandon adopt apply arrange ask attend bake be begin board book borrow bring build buy call\n    cancel catch change check choose clean clear climb close come continue cook cover cut decorate\n    decide do drink drive drop eat email empty enter feed fight find finish fix fly follow forget\n    get give go grow hand have head help hire hold hunt improve increase install invest invite\n    join keep knit launch learn leave lend let listen live log look lose make marry meet move open\n    order organise organize paint pass pay phone pick plan plant play post practise prepare put\n    quit read record redecorate reduce register remember remove renovate rent repair replace\n    report rescue retire return review ride ring run save say see sell send set sew shave shop\n    sign sit slow sort speak spend split start stay stop stream study submit swap switch take talk\n    teach tell test text throw tidy train transfer travel treat trim try tune turn uncover upgrade\n    use visit volunteer wait walk wash watch wear win withdraw work write accept appeal babysit\n    celebrate claim cycle decline dye insure jog meditate renew reply respond retrain rewire swim\n    complain apologise apologize argue browse cheat chill confess cry cuddle dance delay delete\n    download dress forgive gamble hide hug joke kiss laugh lay lie nap panic postpone pray print\n    relax scroll shower sing skip sleep snooze stall stretch sulk tease worry yawn\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DOUBLES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    begin forget regret admit commit prefer refer occur submit permit upset control patrol compel\n    propel equip\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IRREGULAR(py_dict(vec![(py_str("be"), py_str("being"))]));
    s_MOVING_TO(py_str("going to|gonna|off to|out to|away to|heading to|heading off to|about to"));
}
pub fn _ing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut short: Py = Py::None;
    let _ = py_str("The -ing form of a step: \"learn\" -> \"learning\", \"run\" -> \"running\", \"dye\" -> \"dyeing\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) {
    return word.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_IRREGULAR(), &word.clone()))) {
    return geti(&g_IRREGULAR(), &word.clone());
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ie")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), &py_str("ying"));
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("e")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ee"), py_str("ye"), py_str("oe")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) })) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing"));
    }
    short = { let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DOUBLES(), &word.clone())) } };
    if py_truthy(&{ let __b = short.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&py_str("aeiouwxy"), &geti(&word.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_str("aeiou"), &geti(&word.clone(), &op_neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&py_str("aeiou"), &geti(&word.clone(), &op_neg(&Py::Int(3))))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&word.clone(), &op_neg(&Py::Int(3))), &py_str("u"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&word.clone(), &op_neg(&Py::Int(4)), &op_neg(&Py::Int(3)), &Py::None), &py_str("q"))) } } } } } } } } } } } }) {
    return op_add(&op_add(&word.clone(), &geti(&word.clone(), &op_neg(&Py::Int(1)))), &py_str("ing"));
    }
    return op_add(&word.clone(), &py_str("ing"));
    Py::None
}
pub fn a_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("True when the word after a plan's \"to\" is the step itself rather than a place or a thing.");
    return Py::Bool(py_contains(&g_STEPS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "lower", args))(&__args) }, "strip", args))(&__args) }));
    Py::None
}
pub fn said_after_a_to(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut ing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ing") { v.clone() }
    else { Py::None };
    let mut fold: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words after a plan's \"to\", said back as a phrase a sentence can hold.\n\n    \"dye my hair pink\" -> \"dyeing my hair pink\"; \"the gym\" -> \"going to the gym\"; \"bed early\n    tonight\" -> \"going to bed early tonight\". The second is what every phrase the reader cannot\n    vouch for gets, because \"going to\" takes a noun as happily as a verb.\n    ");
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?:;")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "split", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_step(args))(&__args) }) {
    fold = { let __b = ing.clone(); if py_truthy(&__b) { __b } else { py_func("_ing", |args: &[Py]| _ing(args)) } };
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&fold, args))(&__args) }, &if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) { op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }) } else { py_str("") });
    }
    return op_add(&py_str("going to "), &phrase.clone());
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.stepwords".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "DOUBLES" => Some(g_DOUBLES()),
            "IRREGULAR" => Some(g_IRREGULAR()),
            "MOVING_TO" => Some(g_MOVING_TO()),
            "STEPS" => Some(g_STEPS()),
            "_ing" => Some(py_func("_ing", |args: &[Py]| _ing(args))),
            "a_step" => Some(py_func("a_step", |args: &[Py]| a_step(args))),
            "said_after_a_to" => Some(py_func("said_after_a_to", |args: &[Py]| said_after_a_to(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "DOUBLES" => s_DOUBLES(v),
            "IRREGULAR" => s_IRREGULAR(v),
            "MOVING_TO" => s_MOVING_TO(v),
            "STEPS" => s_STEPS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.stepwords", module_obj);
    pyrt::xrefs::reg_fn("compose.stepwords", "_ing", py_func("_ing", |args: &[Py]| _ing(args)));
    pyrt::xrefs::reg_fn("compose.stepwords", "a_step", py_func("a_step", |args: &[Py]| a_step(args)));
    pyrt::xrefs::reg_fn("compose.stepwords", "said_after_a_to", py_func("said_after_a_to", |args: &[Py]| said_after_a_to(args)));
    pyrt::xrefs::reg_access("compose.stepwords", "DOUBLES", g_DOUBLES, s_DOUBLES);
    pyrt::xrefs::reg_access("compose.stepwords", "IRREGULAR", g_IRREGULAR, s_IRREGULAR);
    pyrt::xrefs::reg_access("compose.stepwords", "MOVING_TO", g_MOVING_TO, s_MOVING_TO);
    pyrt::xrefs::reg_access("compose.stepwords", "STEPS", g_STEPS, s_STEPS);
}
