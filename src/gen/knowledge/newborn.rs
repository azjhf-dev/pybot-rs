#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ANSWERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_NEWBORN_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BIRTH_WEIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CLUSTER_FEEDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DAYS_OLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_THE_FORTNIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OUT_AND_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SOMEONE_ELSE_BABY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_FIRST_FORTNIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WEEKS_OLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WEIGHT_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANSWERS() -> Py { ensure_init(); ANSWERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERS.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g__A_NEWBORN_WORD() -> Py { ensure_init(); _A_NEWBORN_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_NEWBORN_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _A_NEWBORN_WORD.write() { *w = v; } }
pub fn g__BIRTH_WEIGHT() -> Py { ensure_init(); _BIRTH_WEIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BIRTH_WEIGHT(v: Py) { ensure_init(); if let Ok(mut w) = _BIRTH_WEIGHT.write() { *w = v; } }
pub fn g__CLUSTER_FEEDING() -> Py { ensure_init(); _CLUSTER_FEEDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CLUSTER_FEEDING(v: Py) { ensure_init(); if let Ok(mut w) = _CLUSTER_FEEDING.write() { *w = v; } }
pub fn g__CORD() -> Py { ensure_init(); _CORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CORD(v: Py) { ensure_init(); if let Ok(mut w) = _CORD.write() { *w = v; } }
pub fn g__DAYS_OLD() -> Py { ensure_init(); _DAYS_OLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DAYS_OLD(v: Py) { ensure_init(); if let Ok(mut w) = _DAYS_OLD.write() { *w = v; } }
pub fn g__NOT_THE_FORTNIGHT() -> Py { ensure_init(); _NOT_THE_FORTNIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_FORTNIGHT(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_THE_FORTNIGHT.write() { *w = v; } }
pub fn g__OUT_AND_ABOUT() -> Py { ensure_init(); _OUT_AND_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OUT_AND_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = _OUT_AND_ABOUT.write() { *w = v; } }
pub fn g__SOMEONE_ELSE_BABY() -> Py { ensure_init(); _SOMEONE_ELSE_BABY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSE_BABY(v: Py) { ensure_init(); if let Ok(mut w) = _SOMEONE_ELSE_BABY.write() { *w = v; } }
pub fn g__THE_FIRST_FORTNIGHT() -> Py { ensure_init(); _THE_FIRST_FORTNIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_FIRST_FORTNIGHT(v: Py) { ensure_init(); if let Ok(mut w) = _THE_FIRST_FORTNIGHT.write() { *w = v; } }
pub fn g__WEEKS_OLD() -> Py { ensure_init(); _WEEKS_OLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WEEKS_OLD(v: Py) { ensure_init(); if let Ok(mut w) = _WEEKS_OLD.write() { *w = v; } }
pub fn g__WEIGHT_SAID() -> Py { ensure_init(); _WEIGHT_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WEIGHT_SAID(v: Py) { ensure_init(); if let Ok(mut w) = _WEIGHT_SAID.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The first fortnight with a newborn: the cord, going out, cluster feeding, the birth weight.\n\nThese rows were built inside `bot/knowledge/family.py` and moved out here, unchanged, because that\nfile is at the publisher's 1 MiB ceiling: a file over it cannot be uploaded at all, so a change to\nit has to make it smaller. The domain is the baby itself rather than the family in general - the\nnews, the cord stump, the first trip out, the evening that never ends, the weight at birth - and\nevery row stands down for the question that is really about a temperature, a feed, a nappy, a cot,\na jab or a name, which keeps its own card.\n\nThe guard matters more than usual: `the-first-fortnight` is the fallback for the news and the age,\nso it must take itself out of the running when the line is something else (a follow-up that names\nnothing, an older baby, somebody else's baby) rather than answer \"Congratulations\" and leave the\nreal question with nothing.\n");
    s_ANSWERS(py_tuple(vec![py_str("family-and-parenting")]));
    s_ENTRIES(py_list(vec![]));
    s__THE_FIRST_FORTNIGHT(py_str("Congratulations. The first fortnight is mostly feeding and sleep: eight to twelve feeds in 24 hours, six or more wet nappies a day, and the midwife comes on day five for the heel-prick blood spot test. The cord drops off in about a week to a fortnight, and the birth weight, down by up to a tenth in the first days, is back by two weeks. Ring the midwife or 111 the same day if the baby is too sleepy to feed, has fewer wet nappies, or looks yellow."));
    s__CORD(py_str("The cord stump dries out and drops off on its own between about five days and a fortnight - clean and dry, water only, no cream or powder, and fold the nappy below it so air gets to it, and let it come away rather than pulling. A little bleeding at the very end is normal; a red, hot or smelly patch around the base, or pus, is a call to the midwife the same day. If it is still there at three weeks, mention it."));
    s__OUT_AND_ABOUT(py_str("You can take a newborn out from the first day: a pram or a sling, one more layer than you are wearing, and out of direct sun. Keep them out of crowded rooms and away from anyone with a cough or a cold for the first six weeks, and take the pram rather than passing a newborn round a room. Go out after a feed and you get the long sleep for it."));
    s__CLUSTER_FEEDING(py_str("An evening of non-stop feeding is cluster feeding, and it is normal in the first weeks: it is not a sign your milk is short, it is how the supply is built. Keep them on, swap sides, and get someone else to take them for an hour once they settle so you can sleep. A baby who is having six or more wet nappies a day and gaining along their line is getting enough - the scales at the clinic settle it better than the length of a feed."));
    s__BIRTH_WEIGHT(py_str("The average baby weighs about 3.5 kg (7 lb 8 oz) at birth, and anything from 2.5 to 4.5 kg is normal, so 3 kg (6 lb 10 oz) is a common and perfectly normal size. The figure to watch is the change: up to a tenth of it can go in the first few days, and it is back to birth weight by two weeks. A baby who has not got back to it by then is seen again, so keep the weigh-in appointments."));
    s__WEIGHT_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:weighed|weighs|weight|kilos?|kg|pounds?|lbs?)\\b|\\b\\d+(?:\\.\\d+)?\\s*(?:kilo|kg|lbs?|pounds?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DAYS_OLD(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2})\\s+days?\\s+old\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WEEKS_OLD(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2})\\s+weeks?\\s+old\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NEWBORN_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:newborn|newborns|newly born|just born|born last week|born yesterday|born this week|born a week ago|first week|first fortnight|first few weeks|just had a baby|just had the baby|had the baby|had our baby|gave birth|given birth|born on (?:mon|tues|wednes|thurs|fri|satur|sun)day|born this morning|born today|born at (?:the|home)|home birth)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THE_FORTNIGHT(py_tuple(vec![py_str("temperature"), py_str("fever"), py_str("sick"), py_str("vomiting"), py_str("diarrhoea"), py_str("rash"), py_str("cold"), py_str("cough"), py_str("colic"), py_str("wind"), py_str("reflux"), py_str("nappy"), py_str("nappies"), py_str("poo"), py_str("sleep"), py_str("sleeps"), py_str("sleeping"), py_str("wont sleep"), py_str("wont feed"), py_str("wont take a feed"), py_str("not feeding"), py_str("isnt feeding"), py_str("breastfeed"), py_str("breastfeeding"), py_str("formula"), py_str("milk"), py_str("jab"), py_str("jabs"), py_str("vaccine"), py_str("vaccines"), py_str("vaccination"), py_str("cot"), py_str("cot death"), py_str("pram"), py_str("pushchair"), py_str("car seat"), py_str("bath"), py_str("teething"), py_str("jaundice"), py_str("pillow"), py_str("swaddle"), py_str("swaddling"), py_str("dummy"), py_str("weaning"), py_str("solids"), py_str("food"), py_str("name"), py_str("christening"), py_str("gift"), py_str("present"), py_str("visitors"), py_str("visiting"), py_str("nursery"), py_str("childminder"), py_str("weight gain"), py_str("not gaining"), py_str("losing"), py_str("early"), py_str("premature"), py_str("twins"), py_str("breath"), py_str("breathing"), py_str("cord around"), py_str("crying"), py_str("cries"), py_str("unsettled"), py_str("winding"), py_str("screen"), py_str("test"), py_str("hospital"), py_str("doctor"), py_str("gp"), py_str("health visitor"), py_str("what do i need"), py_str("need to buy"), py_str("need to get"), py_str("what to buy"), py_str("what do we need"), py_str("buy")]));
    s__SOMEONE_ELSE_BABY(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:sister|sisters|friend|friends|neighbour|neighbor|colleague|boss|cousin|niece|aunt|auntie|daughter-in-law|sister-in-law|brother|brother-in-law|stepdaughter)s?\\b|\\b(?:a friend|her baby|his baby|their baby|she had a baby|she had her baby)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-cord-stump")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cord"), py_str("umbilical cord"), py_str("cord stump"), py_str("belly button"), py_str("stump")]), py_tuple(vec![py_str("fall"), py_str("falls"), py_str("falling"), py_str("off"), py_str("come off"), py_str("comes off"), py_str("drop"), py_str("drops"), py_str("when"), py_str("how long"), py_str("still on"), py_str("heal"), py_str("heals"), py_str("clean")])])); __args.push(g__CORD()); __kws.extend(vec![("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-newborn-out-and-about")); __args.push(py_tuple(vec![py_tuple(vec![py_str("newborn"), py_str("baby"), py_str("babies"), py_str("born"), py_str("days old"), py_str("weeks old")]), py_tuple(vec![py_str("go out"), py_str("going out"), py_str("out and about"), py_str("outside"), py_str("leave the house"), py_str("first trip"), py_str("take him out"), py_str("take her out"), py_str("take them out"), py_str("take the baby out"), py_str("when can we go"), py_str("when can i take")])])); __args.push(g__OUT_AND_ABOUT()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("car"), py_str("car seat"), py_str("carseat"), py_str("pram"), py_str("pushchair"), py_str("sling"), py_str("vaccine"), py_str("vaccines"), py_str("jab"), py_str("jabs"), py_str("swim"), py_str("swimming")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cluster-feeding")); __args.push(py_tuple(vec![py_tuple(vec![py_str("baby"), py_str("babies"), py_str("newborn"), py_str("feeding"), py_str("feeds"), py_str("feed")]), py_tuple(vec![py_str("cluster"), py_str("cluster feeding"), py_str("constantly"), py_str("all evening"), py_str("all the time"), py_str("all night"), py_str("feeds constantly"), py_str("feeding constantly"), py_str("wants to feed"), py_str("non stop"), py_str("nonstop"), py_str("every hour")])])); __args.push(g__CLUSTER_FEEDING()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("wont feed"), py_str("won't feed"), py_str("not feeding"), py_str("isnt feeding"), py_str("isn't feeding"), py_str("refusing"), py_str("refuses"), py_str("formula"), py_str("supply"), py_str("mastitis"), py_str("blocked duct"), py_str("tongue tie"), py_str("losing"), py_str("not gaining"), py_str("wont take"), py_str("won't take"), py_str("how often"), py_str("how much milk"), py_str("how many feeds"), py_str("ounces"), py_str("cries"), py_str("crying"), py_str("cry"), py_str("settle"), py_str("settling"), py_str("sleep"), py_str("sleeps"), py_str("sleeping"), py_str("waking"), py_str("awake"), py_str("unsettled"), py_str("whingey")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-birth-weight")); __args.push(py_tuple(vec![py_tuple(vec![py_str("weighed"), py_str("weighs"), py_str("weigh"), py_str("weight"), py_str("kilo"), py_str("kilos"), py_str("kg"), py_str("pounds"), py_str("lb"), py_str("lbs")]), py_tuple(vec![py_str("birth"), py_str("born"), py_str("newborn"), py_str("baby"), py_str("babies"), py_str("gave birth"), py_str("days old"), py_str("last week"), py_str("at birth")])])); __args.push(g__the_birth_weight()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("month old"), py_str("months old"), py_str("years old"), py_str("centile"), py_str("percentile"), py_str("percent"), py_str("%"), py_str("gain"), py_str("gaining"), py_str("put on"), py_str("lost"), py_str("losing"), py_str("dropped"), py_str("too much"), py_str("too little"), py_str("diet")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the-first-fortnight")); __args.push(py_tuple(vec![py_tuple(vec![py_str("baby"), py_str("babies"), py_str("newborn"), py_str("newborns"), py_str("born"), py_str("birth"), py_str("gave birth")]), py_tuple(vec![py_str("last week"), py_str("yesterday"), py_str("days old"), py_str("weeks old"), py_str("day old"), py_str("week old"), py_str("first week"), py_str("first fortnight"), py_str("first few weeks"), py_str("what happens"), py_str("what now"), py_str("next"), py_str("just had"), py_str("born on"), py_str("born this"), py_str("born a week"), py_str("just born"), py_str("new arrival")])])); __args.push(g__THE_FIRST_FORTNIGHT()); __kws.extend(vec![("guard".to_string(), g__the_line_is_the_news()), ("avoid".to_string(), add(&g__NOT_THE_FORTNIGHT(), &py_tuple(vec![py_str("month old"), py_str("months old"), py_str("years old"), py_str("does nothing"), py_str("doesnt help"), py_str("does not help"), py_str("wont help"), py_str("won't help"), py_str("night feeds on my own"), py_str("no help at night")]))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
}
pub fn _the_line_said(args: &[Py]) -> Py {
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
    let mut field: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The line's own words, lowercased: the reading rewrites some lines and the text is truest.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("clean"), py_str("plain")]));
    for __item in __iter1 {
    field = __item.clone();
    said = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[field.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_line_is_the_news(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("Whether this line is the news of a new baby, or the age of one, and nothing else.\n\n    A guard rather than a check inside the reply: an entry whose reply returns None has already\n    won the line, so the card is gone and the question is left with no answer at all. A guard\n    takes the entry out of the running and the next card gets its turn, which is what \"what\n    should i get them\" after a birth needs.\n    ");
    said = _the_line_said(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:she|he|they)\\s+(?:was|is|were|has been)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:my|our)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__SOMEONE_ELSE_BABY(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:my|our)\\s+(?:baby|son|daughter|newborn)\\b|\\bi (?:had|gave)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_is_an_older_baby(&[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_THE_FORTNIGHT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _is_an_older_baby(args: &[Py]) -> Py {
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
    let mut days: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("Whether the line gives the baby an age past the first fortnight.\n\n    The card this fences answers with the first fortnight - the cord, the first trip out, the\n    weight at birth - so \"my baby is 3 weeks old\" is past it: the fortnight's own answers read\n    as yesterday's news, and the stage and feeding cards have that line.\n    ");
    days = mcall(&g__DAYS_OLD(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = days.clone(); if !py_truthy(&__b) { __b } else { gt(&call_builtin("int", &[mcall(&days.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return Py::Bool(true);
    }
    weeks = mcall(&g__WEEKS_OLD(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = weeks.clone(); if !py_truthy(&__b) { __b } else { gt(&call_builtin("int", &[mcall(&weeks.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:[2-9]|1[0-9]|2[0-4])\\s+months?\\s+old\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\byears?\\s+old\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _is_a_newborn(args: &[Py]) -> Py {
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
    let mut days: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("Whether the line says an age in days, or in the first two weeks, for a baby.");
    days = mcall(&g__DAYS_OLD(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = days.clone(); if !py_truthy(&__b) { __b } else { le(&call_builtin("int", &[mcall(&days.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return Py::Bool(true);
    }
    weeks = mcall(&g__WEEKS_OLD(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = weeks.clone(); if !py_truthy(&__b) { __b } else { le(&call_builtin("int", &[mcall(&weeks.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_birth_just_happened(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether a baby was born in the line, or in the turn or two before it.");
    let mut _BIRTH_IN_THE_CHAT: Py = crate::gen::knowledge::family::g__BIRTH_IN_THE_CHAT();
    said = py_list(vec![_the_line_said(&[understood.clone()])]);
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &neg(&Py::Int(2)), &Py::None, &Py::None) } else { py_list(vec![]) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    }
    }
    return call_builtin("bool", &[mcall(&_BIRTH_IN_THE_CHAT.clone(), "search", &[mcall(&py_str(" "), "join", &[said.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_birth_weight(args: &[Py]) -> Py {
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
    let mut days: Py = Py::None;
    let mut said: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("The range a birth weight is normal in, for a weight said in a chat with a new baby.");
    said = _the_line_said(&[understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__WEIGHT_SAID(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("birth"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("at birth"))) } }) {
    return g__BIRTH_WEIGHT();
    }
    if py_truthy(&{ let __b = mcall(&g__DAYS_OLD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__WEEKS_OLD(), "search", &[said.clone()]) } }) {
    days = mcall(&g__DAYS_OLD(), "search", &[said.clone()]);
    weeks = mcall(&g__WEEKS_OLD(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = { let __b = days.clone(); if !py_truthy(&__b) { __b } else { gt(&call_builtin("int", &[mcall(&days.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(21)) } }; if py_truthy(&__b) { __b } else { { let __b = weeks.clone(); if !py_truthy(&__b) { __b } else { gt(&call_builtin("int", &[mcall(&weeks.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } } } }) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_birth_just_happened(&[memory.clone(), understood.clone()])))) {
    return Py::None;
    }
    return g__BIRTH_WEIGHT();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.newborn".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_NEWBORN_WORD" => Some(g__A_NEWBORN_WORD()),
            "_BIRTH_WEIGHT" => Some(g__BIRTH_WEIGHT()),
            "_CLUSTER_FEEDING" => Some(g__CLUSTER_FEEDING()),
            "_CORD" => Some(g__CORD()),
            "_DAYS_OLD" => Some(g__DAYS_OLD()),
            "_NOT_THE_FORTNIGHT" => Some(g__NOT_THE_FORTNIGHT()),
            "_OUT_AND_ABOUT" => Some(g__OUT_AND_ABOUT()),
            "_SOMEONE_ELSE_BABY" => Some(g__SOMEONE_ELSE_BABY()),
            "_THE_FIRST_FORTNIGHT" => Some(g__THE_FIRST_FORTNIGHT()),
            "_WEEKS_OLD" => Some(g__WEEKS_OLD()),
            "_WEIGHT_SAID" => Some(g__WEIGHT_SAID()),
            "_a_birth_just_happened" => Some(py_func("_a_birth_just_happened", |args: &[Py]| _a_birth_just_happened(args))),
            "_is_a_newborn" => Some(py_func("_is_a_newborn", |args: &[Py]| _is_a_newborn(args))),
            "_is_an_older_baby" => Some(py_func("_is_an_older_baby", |args: &[Py]| _is_an_older_baby(args))),
            "_the_birth_weight" => Some(py_func("_the_birth_weight", |args: &[Py]| _the_birth_weight(args))),
            "_the_line_is_the_news" => Some(py_func("_the_line_is_the_news", |args: &[Py]| _the_line_is_the_news(args))),
            "_the_line_said" => Some(py_func("_the_line_said", |args: &[Py]| _the_line_said(args))),
            _ => None,
        }),
        set: None,
    }))
}
