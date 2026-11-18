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


thread_local! { static G_A_CHORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CASH_POINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARBY_ONE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEAR_ME_PAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_STEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ADJECTIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PRONOUN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.nearby_one: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_CHORE() -> Py { ensure_init(); G_A_CHORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHORE(v: Py) { ensure_init(); G_A_CHORE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRADE() -> Py { ensure_init(); G_A_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRADE(v: Py) { ensure_init(); G_A_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERB() -> Py { ensure_init(); G_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERB(v: Py) { ensure_init(); G_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_CASH_POINT() -> Py { ensure_init(); G_CASH_POINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CASH_POINT(v: Py) { ensure_init(); G_CASH_POINT.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARBY_ONE_POOL() -> Py { ensure_init(); G_NEARBY_ONE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARBY_ONE_POOL(v: Py) { ensure_init(); G_NEARBY_ONE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_NEAR_ME_PAIR() -> Py { ensure_init(); G_NEAR_ME_PAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEAR_ME_PAIR(v: Py) { ensure_init(); G_NEAR_ME_PAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PLACE() -> Py { ensure_init(); G_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLACE(v: Py) { ensure_init(); G_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_HEADS() -> Py { ensure_init(); G_PLACE_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_HEADS(v: Py) { ensure_init(); G_PLACE_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_STEMS() -> Py { ensure_init(); G_PLACE_STEMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_STEMS(v: Py) { ensure_init(); G_PLACE_STEMS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_TAILS() -> Py { ensure_init(); G_PLACE_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_TAILS(v: Py) { ensure_init(); G_PLACE_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ADJECTIVE() -> Py { ensure_init(); G_THE_ADJECTIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ADJECTIVE(v: Py) { ensure_init(); G_THE_ADJECTIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ARTICLE() -> Py { ensure_init(); G_THE_ARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ARTICLE(v: Py) { ensure_init(); G_THE_ARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_END() -> Py { ensure_init(); G_THE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_END(v: Py) { ensure_init(); G_THE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FRAME() -> Py { ensure_init(); G_THE_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FRAME(v: Py) { ensure_init(); G_THE_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PRONOUN_ASK() -> Py { ensure_init(); G_THE_PRONOUN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PRONOUN_ASK(v: Py) { ensure_init(); G_THE_PRONOUN_ASK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A near-me ask with the thing left out: \"is there one close by\".\n\nThe named form of this question - \"is there a launderette near me\" - has a pool of its own, and it\nanswers well: it says there is no local list here, that a map app shows what is close, and that a\nphone call settles the rest. The same question asked with the thing left out was not read at all:\n\n    im looking for a launderette\n    is there one close by\n    -> \"I'm not sure about that one, sorry - I'd rather not guess. A quick search online will get\n        you a solid answer, and I can help you make sense of what you find.\"\n\nThe person has just said what they are looking for. \"One\" is that thing. So the second line is the\nnamed ask, and the answer is the named answer with the words put back in: \"a map app search for\nlaunderette near you\". Nothing here is invented - the thing named is the person's own word from\ntheir own line, and the advice is the advice the named ask already gets.\n\nTwo things are read and one is refused:\n\n- the shape: \"is there one near me / near here / nearby / close by / round here / in my area\",\n  \"is there any near me\", \"where is the nearest one\", \"where can i find one\", \"can you find one\".\n- the thing: the last thing the person said they were looking for, with the frame dropped\n  (\"im looking for a launderette\" -> \"launderette\", \"i need a cash machine for the car park\" ->\n  \"cash machine\").\n- a state is not a thing to find. \"im exhausted\" then \"is there one near me\" is not a search for\n  an exhausted, and the module stands down.\n- a pronoun ask whose thread this cannot read. Main has pools for the near-me ask as well, and a\n  question back (\"which thing is it?\") is not an improvement on them: with no thing found, the\n  line is left exactly as it was.\n\nEverything else returns None, and the fallback keeps the line it has.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.THE_PRONOUN_ASK"); }
    s_THE_PRONOUN_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:is|are)\\s+there\\s+(?:one|any)\\s+(?:near(?:\\s+(?:me|here|us))?|nearby|close\\s+by|close\\s+to\\s+(?:me|us|here)|round\\s+here|around\\s+here|in\\s+(?:my|the|our)\\s+(?:area|town|village|end|city))\\b|(?<![a-z])(?:is|are)\\s+there\\s+(?:one|any)\\s+(?:close\\s+by|nearby)\\b|(?<![a-z])where(?:'s|\\s+is)\\s+the\\s+(?:nearest|closest)\\s+one\\b|(?<![a-z])(?:where|how)\\s+(?:can|do|would|should)\\s+i\\s+(?:find|get|locate)\\s+(?:one|any)\\b|(?<![a-z])(?:can|could|will)\\s+you\\s+(?:find|point\\s+me\\s+to|tell\\s+me\\s+where\\s+to\\s+find)\\s+(?:one|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.THE_FRAME"); }
    s_THE_FRAME(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:i\\s*(?:'m|m)?\\s*(?:looking\\s+for|after|in\\s+need\\s+of|trying\\s+to\\s+find)|i\\s+(?:really\\s+)?(?:need|want|would\\s+like|am\\s+after)|i\\s+could\\s+(?:really\\s+)?(?:do\\s+with|use|find)|(?:im|i\\s+am)\\s+(?:desperate|dying)\\s+for|looking\\s+for|(?:where|how)\\s+(?:can|do|would)\\s+i\\s+(?:get|find|buy|locate)|(?:is|are)\\s+there\\s+(?:a|an|any)|(?:can|could)\\s+you\\s+(?:find|recommend|point\\s+me\\s+to)|ive\\s+been\\s+(?:trying\\s+to\\s+find|looking\\s+for|desperate\\s+for)|(?:do\\s+you\\s+know\\s+(?:where|of)\\s+(?:a|an|any)?)|(?:any\\s+idea|any\\s+suggestions?)\\s+(?:where|of|for)\\s+(?:i\\s+(?:can|could|might)\\s+(?:find|get|buy))?)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.THE_END"); }
    s_THE_END(call_builtin("frozenset", &[mcall(&py_str("\nfor from that which who whom to in on at into onto with without near nearest nearby around round\nabout before after because but and or so as if when while during since until by through over under\nbetween among per via than then there here this these those some any all my your our their his her\nits it me us them him week weekend today tomorrow tonight morning afternoon evening night time\nmonth year day hour minute minute please thanks thank soon asap now later\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.THE_ARTICLE"); }
    s_THE_ARTICLE(call_builtin("frozenset", &[mcall(&py_str("a an the some another any other one two three couple few"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.THE_ADJECTIVE"); }
    s_THE_ADJECTIVE(call_builtin("frozenset", &[mcall(&py_str("\ngood nice decent cheap local proper reliable great best better closest nearest nearby quick\nhalf-decent real actual special favourite favorite little big small large new old open\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.A_VERB"); }
    s_A_VERB(call_builtin("frozenset", &[mcall(&py_str("\nget stop be boil cover drink eat give go keep leave learn move order register reinstall remember\nservice shower sleep switch change use wake water cry open buy find start try make save spend\nsort fix call book check\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.NOT_A_THING"); }
    s_NOT_A_THING(op_bitor(&call_builtin("frozenset", &[mcall(&py_str("\nmidwifery series batteries battery account card induction session job clothes course degree\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[mcall(&py_str("\none any it thing something anything anyone someone everything nothing\nlonely tired exhausted sad happy angry upset worried anxious stressed ill sick unwell better worse\nok okay fine well hungry thirsty bored cold hot warm busy free awake asleep asleep dead alive\nworking broken fixed sorted ready done finished lost stuck late early right wrong sure certain\ntrue false real possible likely easy hard difficult cheap expensive worth allowed legal illegal\nallowed normal strange weird fair unfair safe dangerous\ngood bad great nice best worst better worse\nsomewhere anywhere nowhere everywhere someplace anyplace\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.NEARBY_ONE_POOL"); }
    s_NEARBY_ONE_POOL(py_tuple(vec![py_str("Nothing here knows what is around you, sorry - no map, no local list. A map app search for {thing} near you is the quickest way to one, and the reviews on the listing beat anything I could recall about a place I have never been."), py_str("I can't see your area, sorry. Put \"{thing}\" and your town into a map app and it shows what is close, with the opening hours and the distance; the place's own page or a phone call settles anything else about it.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.NOT_A_PLACE"); }
    s_NOT_A_PLACE(call_builtin("frozenset", &[mcall(&py_str("\nchance news way idea reason point need risk history proof difference problem question law rule\nlift taxi bus train tram coach tube flight ferry delivery parcel refund appointment cure treatment\nvaccine price cost quote answer opinion view\nlist colour color salary wage pay jewellery jewelry trousers pants jeans shoes shirt coat\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.PLACE_HEADS"); }
    s_PLACE_HEADS(call_builtin("frozenset", &[mcall(&py_str("\nstore stores station stations stop stops office offices garage garages bank banks centre center\nmarket yard park pool hall hotel cafe pub clinic surgery pharmacy chemist hospital library\nsupermarket launderette launderettes laundrette tip depot studio salon agency firm company gym\ntakeaway barber hairdresser optician\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.A_CHORE"); }
    s_A_CHORE(call_builtin("frozenset", &[mcall(&py_str("shop shops store stores market markets"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.A_TRADE"); }
    s_A_TRADE(call_builtin("frozenset", &[mcall(&py_str("\nplumber electrician decorator plasterer roofer builder joiner carpenter painter gardener cleaner\nlocksmith glazier tiler bricklayer mechanic groomer vet dentist doctor nurse optician physio\nphysiotherapist chiropodist podiatrist therapist counsellor psychotherapist solicitor barrister\naccountant surveyor architect tutor instructor tuner cutter tailor barber hairdresser chef\ncaterer photographer director midwife osteopath acupuncturist notary\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.PLACE_TAILS"); }
    s_PLACE_TAILS(py_tuple(vec![py_str("ette"), py_str("ettes"), py_str("ery"), py_str("ary"), py_str("eum"), py_str("eums")]));
    fn __data1() -> Py { py_tuple(vec![py_str("shop"), py_str("store"), py_str("market"), py_str("station"), py_str("garage"), py_str("cafe"), py_str("park"), py_str("hall"), py_str("bank"), py_str("pool"), py_str("clinic"), py_str("centre"), py_str("center"), py_str("chemist"), py_str("pub"), py_str("hotel"), py_str("studio"), py_str("salon"), py_str("agency"), py_str("firm"), py_str("gym"), py_str("kiosk"), py_str("stall"), py_str("chippy"), py_str("chippie"), py_str("deli"), py_str("caff"), py_str("diner"), py_str("grill"), py_str("bistro"), py_str("launderette"), py_str("laundrette"), py_str("hospital"), py_str("surgery")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.PLACE_STEMS"); }
    s_PLACE_STEMS(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.CASH_POINT"); }
    s_CASH_POINT(call_builtin("frozenset", &[py_tuple(vec![py_str("cash point"), py_str("cash points"), py_str("cashpoint"), py_str("cashpoints"), py_str("cash machine"), py_str("cash machines"), py_str("atm"), py_str("atms")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("post box"), py_str("postbox"), py_str("pillar box"), py_str("phone box"), py_str("bottle bank"), py_str("recycling point"), py_str("public toilet"), py_str("public toilets"), py_str("litter bin"), py_str("lost property office"), py_str("petrol station"), py_str("bus stop"), py_str("taxi rank")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.nearby_one.NEAR_ME_PAIR"); }
    s_NEAR_ME_PAIR(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_thing(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the person said they were looking for, in their own words, or \"\".");
    found = mcall(&g_THE_FRAME(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    words = mcall(&slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "split", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(".,!?;:'\"()")]), "lower", &[]), &py_str("to"))) } }) {
    return py_str("");
    }
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    bare = mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:'\"()")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THE_END(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THE_ARTICLE(), &bare.clone())) } }) {
    if py_truthy(&kept.clone()) {
    break;
    }
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THE_ADJECTIVE(), &bare.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&kept.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("^[a-z][a-z'&-]*$"), bare.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    break;
    }
    let _ = mcall(&kept.clone(), "append", &[bare.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)))) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &geti(&kept.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &mcall(&py_str(" "), "join", &[kept.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_A_VERB(), &geti(&kept.clone(), &Py::Int(0))))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CASH_POINT(), &mcall(&py_str(" "), "join", &[kept.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NEAR_ME_PAIR(), &mcall(&py_str(" "), "join", &[kept.clone()]))) } }) {
    return mcall(&py_str(" "), "join", &[kept.clone()]);
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_PLACE())) {
    return py_str("");
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_THING())) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_place(&[kept.clone()])))) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[kept.clone()]);
    Py::None
}
pub fn _a_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kept: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kept"))) };
    let mut last: Py = Py::None;
    let _ = py_str("True when the thing the person named is somewhere a map app could show them.");
    last = geti(&kept.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_CHORE(), &last.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACE_HEADS(), &last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_TRADE(), &last.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&last.clone(), "endswith", &[g_PLACE_STEMS()]); if py_truthy(&__b) { __b } else { mcall(&last.clone(), "endswith", &[g_PLACE_TAILS()]) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_thread_thing(args: &[Py]) -> Py {
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
    let mut cleaned: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cleaned") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cleaned"))) };
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The thing the person said they were looking for, newest first, or \"\" if that was not a place.\n\n    The last thing they asked for decides, because that is what \"one\" and \"the nearest one\" mean:\n    four turns back is a different part of the conversation. A turn that asks for no thing at all\n    (\"its for a broken key\") is a detail and does not decide anything, so the scan keeps going.\n    ");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&mcall(&{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&said.clone(), &cleaned.clone())); if py_truthy(&__b) { __b } else { mcall(&g_THE_PRONOUN_ASK(), "search", &[said.clone()]) } } } }) {
    continue;
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_gt(&seen.clone(), &Py::Int(4))) {
    break;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THE_FRAME(), "search", &[said.clone()])))) {
    continue;
    }
    return _the_thing(&[said.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut letter: Py = Py::None;
    let _ = py_str("One of two wordings, the same one for the same line.");
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return py_str("");
    }
    return geti(&pool.clone(), &op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&line.clone()) { let mut letter: Py = __item.clone(); __out.push(call_builtin("ord", &[letter.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut cleaned: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The map-app answer to a pronoun near-me ask, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    cleaned = mcall(&py_str(" "), "join", &[mcall(&mcall(&line.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THE_PRONOUN_ASK(), "search", &[cleaned.clone()])))) {
    return Py::None;
    }
    thing = _the_thread_thing(&[memory.clone(), cleaned.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_pick(&[g_NEARBY_ONE_POOL(), cleaned.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.nearby_one".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CHORE" => Some(g_A_CHORE()),
            "A_TRADE" => Some(g_A_TRADE()),
            "A_VERB" => Some(g_A_VERB()),
            "CASH_POINT" => Some(g_CASH_POINT()),
            "NEARBY_ONE_POOL" => Some(g_NEARBY_ONE_POOL()),
            "NEAR_ME_PAIR" => Some(g_NEAR_ME_PAIR()),
            "NOT_A_PLACE" => Some(g_NOT_A_PLACE()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "PLACE_HEADS" => Some(g_PLACE_HEADS()),
            "PLACE_STEMS" => Some(g_PLACE_STEMS()),
            "PLACE_TAILS" => Some(g_PLACE_TAILS()),
            "THE_ADJECTIVE" => Some(g_THE_ADJECTIVE()),
            "THE_ARTICLE" => Some(g_THE_ARTICLE()),
            "THE_END" => Some(g_THE_END()),
            "THE_FRAME" => Some(g_THE_FRAME()),
            "THE_PRONOUN_ASK" => Some(g_THE_PRONOUN_ASK()),
            "_a_place" => Some(py_func("_a_place", |args: &[Py]| _a_place(args))),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_the_thread_thing" => Some(py_func("_the_thread_thing", |args: &[Py]| _the_thread_thing(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CHORE" => s_A_CHORE(v),
            "A_TRADE" => s_A_TRADE(v),
            "A_VERB" => s_A_VERB(v),
            "CASH_POINT" => s_CASH_POINT(v),
            "NEARBY_ONE_POOL" => s_NEARBY_ONE_POOL(v),
            "NEAR_ME_PAIR" => s_NEAR_ME_PAIR(v),
            "NOT_A_PLACE" => s_NOT_A_PLACE(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "PLACE_HEADS" => s_PLACE_HEADS(v),
            "PLACE_STEMS" => s_PLACE_STEMS(v),
            "PLACE_TAILS" => s_PLACE_TAILS(v),
            "THE_ADJECTIVE" => s_THE_ADJECTIVE(v),
            "THE_ARTICLE" => s_THE_ARTICLE(v),
            "THE_END" => s_THE_END(v),
            "THE_FRAME" => s_THE_FRAME(v),
            "THE_PRONOUN_ASK" => s_THE_PRONOUN_ASK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.nearby_one", module_obj);
    pyrt::xrefs::reg_fn("compose.nearby_one", "_the_thing", py_func("_the_thing", |args: &[Py]| _the_thing(args)));
    pyrt::xrefs::reg_fn("compose.nearby_one", "_a_place", py_func("_a_place", |args: &[Py]| _a_place(args)));
    pyrt::xrefs::reg_fn("compose.nearby_one", "_the_thread_thing", py_func("_the_thread_thing", |args: &[Py]| _the_thread_thing(args)));
    pyrt::xrefs::reg_fn("compose.nearby_one", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_fn("compose.nearby_one", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.nearby_one", "A_CHORE", g_A_CHORE, s_A_CHORE);
    pyrt::xrefs::reg_access("compose.nearby_one", "A_TRADE", g_A_TRADE, s_A_TRADE);
    pyrt::xrefs::reg_access("compose.nearby_one", "A_VERB", g_A_VERB, s_A_VERB);
    pyrt::xrefs::reg_access("compose.nearby_one", "CASH_POINT", g_CASH_POINT, s_CASH_POINT);
    pyrt::xrefs::reg_access("compose.nearby_one", "NEARBY_ONE_POOL", g_NEARBY_ONE_POOL, s_NEARBY_ONE_POOL);
    pyrt::xrefs::reg_access("compose.nearby_one", "NEAR_ME_PAIR", g_NEAR_ME_PAIR, s_NEAR_ME_PAIR);
    pyrt::xrefs::reg_access("compose.nearby_one", "NOT_A_PLACE", g_NOT_A_PLACE, s_NOT_A_PLACE);
    pyrt::xrefs::reg_access("compose.nearby_one", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("compose.nearby_one", "PLACE_HEADS", g_PLACE_HEADS, s_PLACE_HEADS);
    pyrt::xrefs::reg_access("compose.nearby_one", "PLACE_STEMS", g_PLACE_STEMS, s_PLACE_STEMS);
    pyrt::xrefs::reg_access("compose.nearby_one", "PLACE_TAILS", g_PLACE_TAILS, s_PLACE_TAILS);
    pyrt::xrefs::reg_access("compose.nearby_one", "THE_ADJECTIVE", g_THE_ADJECTIVE, s_THE_ADJECTIVE);
    pyrt::xrefs::reg_access("compose.nearby_one", "THE_ARTICLE", g_THE_ARTICLE, s_THE_ARTICLE);
    pyrt::xrefs::reg_access("compose.nearby_one", "THE_END", g_THE_END, s_THE_END);
    pyrt::xrefs::reg_access("compose.nearby_one", "THE_FRAME", g_THE_FRAME, s_THE_FRAME);
    pyrt::xrefs::reg_access("compose.nearby_one", "THE_PRONOUN_ASK", g_THE_PRONOUN_ASK, s_THE_PRONOUN_ASK);
}
