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


thread_local! { static G_DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRINK_PAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXTRA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GENERIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MY_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_FOOD_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_FOR_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_X_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KNOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.guest: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_DISLIKE() -> Py { ensure_init(); G_DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISLIKE(v: Py) { ensure_init(); G_DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g_DRINK_PAIR() -> Py { ensure_init(); G_DRINK_PAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRINK_PAIR(v: Py) { ensure_init(); G_DRINK_PAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_EXTRA() -> Py { ensure_init(); G_EXTRA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXTRA(v: Py) { ensure_init(); G_EXTRA.with(|c| *c.borrow_mut() = v); }
pub fn g_GENERIC() -> Py { ensure_init(); G_GENERIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GENERIC(v: Py) { ensure_init(); G_GENERIC.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MY_THING() -> Py { ensure_init(); G_NOT_MY_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MY_THING(v: Py) { ensure_init(); G_NOT_MY_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_FOOD_ALONE() -> Py { ensure_init(); G_NO_FOOD_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_FOOD_ALONE(v: Py) { ensure_init(); G_NO_FOOD_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_FOR_ME() -> Py { ensure_init(); G_NO_FOR_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_FOR_ME(v: Py) { ensure_init(); G_NO_FOR_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_X_ALONE() -> Py { ensure_init(); G_NO_X_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_X_ALONE(v: Py) { ensure_init(); G_NO_X_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN() -> Py { ensure_init(); G_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN(v: Py) { ensure_init(); G_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP() -> Py { ensure_init(); G_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); G_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__KNOWN() -> Py { ensure_init(); G__KNOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KNOWN(v: Py) { ensure_init(); G__KNOWN.with(|c| *c.borrow_mut() = v); }
pub fn g__WHEN() -> Py { ensure_init(); G__WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHEN(v: Py) { ensure_init(); G__WHEN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What a guest will not eat, and the answer it changes.\n\nA line such as \"and she hates mushrooms\" is not a new subject: it is one more constraint\non the dinner we were already planning. The memory store keeps it against the person, and\nthis module revises the suggestion the bot has just given so the reply is concrete rather\nthan a nod. The same shape serves \"he can't stand olives\" and \"she isn't keen on fish\".\n\nThe knowledge base is still the authority on food; this module only needs to tell a food\napart from the other things a person might hate (\"she hates her job\"). The words come from\nthe understanding lane's cooking vocabulary and from the pantry table, so a new ingredient\nthere is recognised here without a second list to maintain.\n");
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s_STOP(stdlib_call("re", "compile", &[py_str("\\b(?:but|so|because|though|although|which|when|if|and then)\\b|[,.;!?]")]).unwrap_or_else(|e| exc::raise(e)));
    s_DISLIKE(stdlib_call("re", "compile", &[py_str("\\b(?:hates?|dislikes?|can'?t stand|cannot stand|(?:do|does|did)(?:n'?t| not) (?:like|fancy|enjoy|want|wants?)|(?:isn'?t|aren'?t|is not|are not|amn'?t) (?:(?:that|really|very|so) )?(?:much of )?(?:a |the )?(?:(?:big|huge|massive|great) )?(?:fan of|keen on|fond of|fussed on|my thing|my kind of thing)|not (?:(?:that|really|very|so|all that) )?(?:much of )?(?:a |the )?(?:(?:big|huge|massive|great) )?(?:fan of|keen on|fond of|fussed on|my thing|my kind of thing)|avoids?|avoiding)\\s+(?:the\\s+|any\\s+)?(?P<food>[a-z][a-z' -]{1,40})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_MY_THING(stdlib_call("re", "compile", &[py_str("\\b(?P<food>[a-z][a-z' -]{1,30}?)\\s+(?:isn'?t|is not|aren'?t|are not)\\s+(?:(?:really|that|exactly|quite)\\s+)?(?:my|our)\\s+(?:(?:kind|sort)\\s+of\\s+|cup\\s+of\\s+tea|thing)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_FOR_ME(stdlib_call("re", "compile", &[py_str("\\b(?:no|not|none of the)\\s+(?:any\\s+|more\\s+|the\\s+)?(?P<food>[a-z][a-z' -]{1,40}?)\\s+(?:for me|thanks|thank you|please|today|tonight|tomorrow|later)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_FOOD_ALONE(stdlib_call("re", "compile", &[py_str("^\\s*(?:no|not|none(?=[\\s,])|nothing with|anything but)\\s+(?P<food>[a-z][a-z' ,-]{3,60}?\\s+(?:or|and|,)\\s+[a-z][a-z' ,-]{1,40})\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GENERIC(call_builtin("frozenset", &[mcall(&py_str("\ncooking cook food dinner tea lunch breakfast eating eat meal meals work job school housework\ncleaning shopping mornings mondays tuesdays wednesdays thursdays fridays saturdays sundays\nwinter summer rain traffic flying travel travelling exercise sport gym crowds spiders\neverything anything nothing it that this them him her\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN(call_builtin("frozenset", &[mcall(&py_str("her his their my your our its it them him this that these those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_EXTRA(call_builtin("frozenset", &[mcall(&py_str("\nolive feta couscous halloumi paneer quinoa avocado broccoli cauliflower cabbage leek celery\ncucumber lettuce peas sweetcorn beetroot aubergine courgette pumpkin berries banana apple\norange lemon lime grape melon peach pear plum mango pineapple strawberry blueberry raspberry\nwalnut almond cashew peanut hazelnut pistachio sesame tofu prawn salmon tuna cod haddock\nmackerel sardine anchovy sausage bacon ham chorizo steak mince turkey duck lamb pork beef\nchicken fish egg eggs cheese butter milk cream yoghurt bread pasta rice potato tomato onion\ngarlic carrot pepper spinach mushroom lentil chickpea bean noodle soup curry lasagne pizza\ncake chocolate sugar salt\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__KNOWN(Py::None);
    s__WHEN(stdlib_call("re", "compile", &[py_str("(?:\\s+(?:(?:this|next|last|on)\\s+(?:week|weekend|friday|saturday|sunday|monday|tuesday|wednesday|thursday|morning|evening|afternoon|night|weekday|birthday|christmas)|today|tonight|tomorrow|tomorrow night|now|later|again|then|instead|for (?:tea|dinner|lunch|breakfast|supper)|at (?:lunch|dinner|breakfast|tea)|any\\s?more))+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DRINK_PAIR(call_builtin("frozenset", &[mcall(&py_str("\ntea coffee beer wine lager cider ale stout juice squash cordial water milk lemonade\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_X_ALONE(stdlib_call("re", "compile", &[py_str("^\\s*(?:no|not|none of the|nothing with)\\s+(?:any\\s+|more\\s+|the\\s+)?(?P<food>[a-z][a-z' -]{1,40}?)\\s*(?:please|thanks|thank you)?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("A word with its plural undone, so mushrooms/mushroom are one food.");
    word = mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str(".,!?;:'\"")]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ies")]) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("oes")]) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("es")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str("sxzh"), &geti(&word.clone(), &op_neg(&Py::Int(3))))) } } } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ss")]))) } } } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _known(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut STRONG: Py = Py::None;
    let mut _reply: Py = Py::None;
    let mut aliases: Py = Py::None;
    let mut knowledge: Py = Py::None;
    let mut name: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every food word the rest of the bot already knows, built once.");
    if py_truthy(&Py::Bool(!py_is(&g__KNOWN(), &Py::None))) {
    return g__KNOWN();
    }
    words = call_builtin("set", &[g_EXTRA()]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    STRONG = pyrt::xrefs::xget("understanding.topics", "STRONG");
    let __iter6 = py_iter_or_raise(&py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    for __item in __iter6 {
    topic = __item.clone();
    let _ = mcall(&words.clone(), "update", &[mcall(&mcall(&STRONG.clone(), "get", &[topic.clone(), py_str("")]), "split", &[])]);
    }
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try7 = exc::catch(|| {
    knowledge = pyrt::xrefs::xmodule("knowledge.cooking");
    let __iter12 = py_iter_or_raise(&mcall(&gattr(&knowledge.clone(), "PANTRY"), "items", &[]));
    for __item in __iter12 {
    let __unpk13 = py_unpack(&__item.clone(), 2, None);
    name = __unpk13[0].clone();
    let __unpk14 = py_unpack(&__unpk13[1].clone(), 2, None);
    aliases = __unpk14[0].clone();
    _reply = __unpk14[1].clone();
    let _ = mcall(&words.clone(), "add", &[name.clone()]);
    let _ = mcall(&words.clone(), "update", &[aliases.clone()]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["Exception"]) {
                __handled11 = true;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    s__KNOWN({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(_fold(&[word.clone()])); } } py_set(__out) })() });
    return g__KNOWN();
    Py::None
}
pub fn _clean(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    item = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = item.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,!?;:'\"")]), "lower", &[]), "split", &[])]);
    item = mcall(&stdlib_call("re", "sub", &[py_str("^(?:the|any|some|more|all|a|an)\\s+"), py_str(""), item.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    item = mcall(&stdlib_call("re", "sub", &[py_str("\\s+(?:too|either|as well|much|really|actually|at all|though|tho|anyway)$"), py_str(""), item.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    item = mcall(&mcall(&g__WHEN(), "sub", &[py_str(""), item.clone()]), "strip", &[py_str(" ,")]);
    return item.clone();
    Py::None
}
pub fn _one_half(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("One half of a pair that was turned down: a food, or a drink named as one.");
    if py_truthy(&_is_food(&[piece.clone()])) {
    return Py::Bool(true);
    }
    words = mcall(&g_WORD(), "findall", &[piece.clone()]);
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DRINK_PAIR(), &_fold(&[word.clone()])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _is_food(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut known: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&item.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GENERIC(), &item.clone())) } }) {
    return Py::Bool(false);
    }
    words = mcall(&g_WORD(), "findall", &[item.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PRONOUN(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    known = _known(&[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&known.clone(), &_fold(&[word.clone()])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn plain(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut a_pair: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a_pair") { v.clone() }
    else { Py::Bool(false) };
    let mut known: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The food itself for a dislike of the user's own, or None when the phrase only contains one.\n\n    `_is_food` is right for editing a suggestion that mentions the food (\"mushrooms\" in a\n    mushroom lasagne) and too loose for a promise about everything the bot suggests: \"no noisy\n    restaurants\", \"no cheap wine\" and \"no mushrooms on pizza\" are not foods. A plain phrase has\n    only known food words in it; \"the texture of tofu\" is read as the tofu.\n    ");
    item = _clean(&[item.clone()]);
    item = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:the\\s+)?(?:taste|texture|smell|look|sight)\\s+of\\s+(?:the\\s+)?")); __args.push(py_str("")); __args.push(item.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = _is_food(&[item.clone()]); if py_truthy(&__b) { __b } else { { let __b = a_pair.clone(); if !py_truthy(&__b) { __b } else { _one_half(&[item.clone()]) } } } })); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:on|with|of|in|at)\\b"), item.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    known = _known(&[]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[item.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&known.clone(), &_fold(&[word.clone()])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return item.clone();
    Py::None
}
pub fn found(args: &[Py]) -> Py {
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
    let mut _at: Py = Py::None;
    let mut a_pair: Py = Py::None;
    let mut chunk: Py = Py::None;
    let mut food: Py = Py::None;
    let mut item: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let _ = py_str("[(food, where)] in the line: [(\"mushrooms\", 15)] for \"and she hates mushrooms\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_DISLIKE(), g_NOT_MY_THING(), g_NO_FOR_ME(), g_NO_FOOD_ALONE()]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    chunk = geti(&mcall(&g_STOP(), "split", &[mcall(&match_.clone(), "group", &[py_str("food")])]), &Py::Int(0));
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",|\\band\\b|\\bor\\b"), chunk.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut piece: Py = __item.clone(); __out.push(_clean(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:no|not)\\s+")); __args.push(py_str("")); __args.push(mcall(&piece.clone(), "strip", &[])); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])); } py_list(__out) })() };
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&piece.clone()) {  __out.push(piece.clone()); } } py_list(__out) })() };
    a_pair = { let __b = op_ge(&call_builtin("len", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); __out.push(_one_half(&[piece.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = Py::Bool(py_is(&pattern.clone(), &g_NO_FOOD_ALONE())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&a_pair.clone())) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&pieces.clone());
    for __item in __iter3 {
    item = __item.clone();
    if py_truthy(&{ let __b = { let __b = _is_food(&[item.clone()]); if py_truthy(&__b) { __b } else { a_pair.clone() } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let __unpk7722 = py_unpack(&__item.clone(), 2, None); let mut food: Py = __unpk7722[0].clone(); let mut _at: Py = __unpk7722[1].clone(); __out.push(food.clone()); } py_list(__out) })() }, &item.clone())) } }) {
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![item.clone(), mcall(&match_.clone(), "start", &[py_str("food")])])]);
    }
    }
    }
    }
    return out.clone();
    Py::None
}
pub fn turned_down_alone(args: &[Py]) -> Py {
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
    let mut item: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The foods a bare \"no mushrooms\" names, or [] when the line says anything more.");
    match_ = mcall(&g_NO_X_ALONE(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_list(vec![]);
    }
    item = _clean(&[mcall(&match_.clone(), "group", &[py_str("food")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&item.clone())); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:or|and)\\b"), item.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_list(vec![]);
    }
    return if py_truthy(&_is_food(&[item.clone()])) { py_list(vec![item.clone()]) } else { py_list(vec![]) };
    Py::None
}
pub fn _contains(args: &[Py]) -> Py {
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
    let mut food: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut root: Py = Py::None;
    let mut word: Py = Py::None;
    root = _fold(&[food.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_eq(&_fold(&[word.clone()]), &root.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _without_article(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|the)\\s+")); __args.push(py_str("")); __args.push(mcall(&item.clone(), "strip", &[])); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _split_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut item: Py = Py::None;
    let mut items: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The options in \"a curry, a lasagne, or a traybake\".");
    parts = stdlib_call("re", "split", &[py_str(",\\s*|\\s+or\\s+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[py_str(" .")])) {  __out.push(stdlib_call("re", "sub", &[py_str("^(?:or|and)\\s+"), py_str(""), mcall(&part.clone(), "strip", &[py_str(" .")])]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut item: Py = __item.clone(); __out.push(op_gt(&call_builtin("len", &[mcall(&g_WORD(), "findall", &[item.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return items.clone();
    Py::None
}
pub fn _kept_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut items: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "items") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: items"))) };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[items.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return op_add(&geti(&items.clone(), &Py::Int(0)), &py_str(" still works"));
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[items.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return op_add(&op_add(&op_add(&geti(&items.clone(), &Py::Int(0)), &py_str(" or ")), &geti(&items.clone(), &Py::Int(1))), &py_str(" both still work"));
    }
    return op_add(&op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&items.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" or ")), &geti(&items.clone(), &op_neg(&Py::Int(1)))), &py_str(" all still work"));
    Py::None
}
pub fn revise(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut foods: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "foods") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: foods"))) };
    let mut food: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut head: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut item: Py = Py::None;
    let mut items: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut removed: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sep: Py = Py::None;
    let _ = py_str("The last answer with the disliked food taken out, or None when it does not mention it.\n\n    The reply is edited only where the food is one option in a list (\"a curry, a lasagne, or a\n    traybake\"), because that is where a friend would say \"then leave that one out\". A single\n    recipe that needs it has no alternative to keep, so the caller lets another module answer.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&foods.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&reply.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    hit = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&foods.clone()) { let mut food: Py = __item.clone(); if py_truthy(&_contains(&[sentence.clone(), food.clone()])) {  __out.push(food.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&hit.clone()))) {
    continue;
    }
    let __unpk2 = py_unpack(&mcall(&sentence.clone(), "partition", &[py_str(": ")]), 3, None);
    head = __unpk2[0].clone();
    sep = __unpk2[1].clone();
    rest = __unpk2[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&sep.clone()))) {
    continue;
    }
    items = _split_list(&[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&items.clone()))) {
    continue;
    }
    removed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut item: Py = __item.clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&hit.clone()) { let mut food: Py = __item.clone(); __out.push(_contains(&[item.clone(), food.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(item.clone()); } } py_list(__out) })() };
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&removed.clone(), &item.clone()))) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&removed.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&kept.clone())) } }) {
    continue;
    }
    gone = op_add(&py_str("the "), &mcall(&py_str(" and the "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&removed.clone()) { let mut item: Py = __item.clone(); __out.push(_without_article(&[item.clone()])); } py_list(__out) })() }]));
    let __aug3 = op_add(&gone.clone(), &if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[removed.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { py_str(" is out") } else { py_str(" are out") });
    gone = __aug3;
    return op_add(&op_add(&op_add(&op_add(&py_str("Then "), &gone.clone()), &py_str(" — ")), &_kept_phrase(&[kept.clone()])), &py_str("."));
    }
    return Py::None;
    Py::None
}
pub fn left_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut foods: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "foods") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: foods"))) };
    let mut food: Py = Py::None;
    let mut items: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A short reply to a food the user themselves will not eat, with no suggestion to edit.");
    items = call_builtin("list", &[mcall(&py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), "fromkeys", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&foods.clone()) { let mut food: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[food.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[food.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() }])]).unwrap_or_else(|e| exc::raise(e));
    said = if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[items.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) { mcall(&py_str(" or "), "join", &[items.clone()]) } else { mcall(&py_str(", "), "join", &[items.clone()]) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    return op_add(&op_add(&py_str("Fair enough — no "), &said.clone()), &py_str(" in anything I suggest."));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.guest".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "DISLIKE" => Some(g_DISLIKE()),
            "DRINK_PAIR" => Some(g_DRINK_PAIR()),
            "EXTRA" => Some(g_EXTRA()),
            "GENERIC" => Some(g_GENERIC()),
            "NOT_MY_THING" => Some(g_NOT_MY_THING()),
            "NO_FOOD_ALONE" => Some(g_NO_FOOD_ALONE()),
            "NO_FOR_ME" => Some(g_NO_FOR_ME()),
            "NO_X_ALONE" => Some(g_NO_X_ALONE()),
            "PRONOUN" => Some(g_PRONOUN()),
            "STOP" => Some(g_STOP()),
            "WORD" => Some(g_WORD()),
            "_KNOWN" => Some(g__KNOWN()),
            "_WHEN" => Some(g__WHEN()),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_contains" => Some(py_func("_contains", |args: &[Py]| _contains(args))),
            "_fold" => Some(py_func("_fold", |args: &[Py]| _fold(args))),
            "_is_food" => Some(py_func("_is_food", |args: &[Py]| _is_food(args))),
            "_kept_phrase" => Some(py_func("_kept_phrase", |args: &[Py]| _kept_phrase(args))),
            "_known" => Some(py_func("_known", |args: &[Py]| _known(args))),
            "_one_half" => Some(py_func("_one_half", |args: &[Py]| _one_half(args))),
            "_split_list" => Some(py_func("_split_list", |args: &[Py]| _split_list(args))),
            "_without_article" => Some(py_func("_without_article", |args: &[Py]| _without_article(args))),
            "found" => Some(py_func("found", |args: &[Py]| found(args))),
            "left_out" => Some(py_func("left_out", |args: &[Py]| left_out(args))),
            "plain" => Some(py_func("plain", |args: &[Py]| plain(args))),
            "revise" => Some(py_func("revise", |args: &[Py]| revise(args))),
            "turned_down_alone" => Some(py_func("turned_down_alone", |args: &[Py]| turned_down_alone(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "DISLIKE" => s_DISLIKE(v),
            "DRINK_PAIR" => s_DRINK_PAIR(v),
            "EXTRA" => s_EXTRA(v),
            "GENERIC" => s_GENERIC(v),
            "NOT_MY_THING" => s_NOT_MY_THING(v),
            "NO_FOOD_ALONE" => s_NO_FOOD_ALONE(v),
            "NO_FOR_ME" => s_NO_FOR_ME(v),
            "NO_X_ALONE" => s_NO_X_ALONE(v),
            "PRONOUN" => s_PRONOUN(v),
            "STOP" => s_STOP(v),
            "WORD" => s_WORD(v),
            "_KNOWN" => s__KNOWN(v),
            "_WHEN" => s__WHEN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.guest", module_obj);
    pyrt::xrefs::reg_fn("memory.guest", "_fold", py_func("_fold", |args: &[Py]| _fold(args)));
    pyrt::xrefs::reg_fn("memory.guest", "_known", py_func("_known", |args: &[Py]| _known(args)));
    pyrt::xrefs::reg_fn("memory.guest", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("memory.guest", "_one_half", py_func("_one_half", |args: &[Py]| _one_half(args)));
    pyrt::xrefs::reg_fn("memory.guest", "_is_food", py_func("_is_food", |args: &[Py]| _is_food(args)));
    pyrt::xrefs::reg_fn("memory.guest", "plain", py_func("plain", |args: &[Py]| plain(args)));
    pyrt::xrefs::reg_fn("memory.guest", "found", py_func("found", |args: &[Py]| found(args)));
    pyrt::xrefs::reg_fn("memory.guest", "turned_down_alone", py_func("turned_down_alone", |args: &[Py]| turned_down_alone(args)));
    pyrt::xrefs::reg_fn("memory.guest", "_contains", py_func("_contains", |args: &[Py]| _contains(args)));
    pyrt::xrefs::reg_fn("memory.guest", "_without_article", py_func("_without_article", |args: &[Py]| _without_article(args)));
    pyrt::xrefs::reg_fn("memory.guest", "_split_list", py_func("_split_list", |args: &[Py]| _split_list(args)));
    pyrt::xrefs::reg_fn("memory.guest", "_kept_phrase", py_func("_kept_phrase", |args: &[Py]| _kept_phrase(args)));
    pyrt::xrefs::reg_fn("memory.guest", "revise", py_func("revise", |args: &[Py]| revise(args)));
    pyrt::xrefs::reg_fn("memory.guest", "left_out", py_func("left_out", |args: &[Py]| left_out(args)));
    pyrt::xrefs::reg_access("memory.guest", "DISLIKE", g_DISLIKE, s_DISLIKE);
    pyrt::xrefs::reg_access("memory.guest", "DRINK_PAIR", g_DRINK_PAIR, s_DRINK_PAIR);
    pyrt::xrefs::reg_access("memory.guest", "EXTRA", g_EXTRA, s_EXTRA);
    pyrt::xrefs::reg_access("memory.guest", "GENERIC", g_GENERIC, s_GENERIC);
    pyrt::xrefs::reg_access("memory.guest", "NOT_MY_THING", g_NOT_MY_THING, s_NOT_MY_THING);
    pyrt::xrefs::reg_access("memory.guest", "NO_FOOD_ALONE", g_NO_FOOD_ALONE, s_NO_FOOD_ALONE);
    pyrt::xrefs::reg_access("memory.guest", "NO_FOR_ME", g_NO_FOR_ME, s_NO_FOR_ME);
    pyrt::xrefs::reg_access("memory.guest", "NO_X_ALONE", g_NO_X_ALONE, s_NO_X_ALONE);
    pyrt::xrefs::reg_access("memory.guest", "PRONOUN", g_PRONOUN, s_PRONOUN);
    pyrt::xrefs::reg_access("memory.guest", "STOP", g_STOP, s_STOP);
    pyrt::xrefs::reg_access("memory.guest", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("memory.guest", "_KNOWN", g__KNOWN, s__KNOWN);
    pyrt::xrefs::reg_access("memory.guest", "_WHEN", g__WHEN, s__WHEN);
}
