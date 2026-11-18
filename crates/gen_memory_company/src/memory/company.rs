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


thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_FOR_A_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMING_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EITHER_SIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IS_COMING_TOO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_TO_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_COMING_ALONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TO_THE_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.company: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_FOR_A_DETAIL() -> Py { ensure_init(); G_ASKS_FOR_A_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_A_DETAIL(v: Py) { ensure_init(); G_ASKS_FOR_A_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_COMING_WITH() -> Py { ensure_init(); G_COMING_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMING_WITH(v: Py) { ensure_init(); G_COMING_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_EITHER_SIDE() -> Py { ensure_init(); G_EITHER_SIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EITHER_SIDE(v: Py) { ensure_init(); G_EITHER_SIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_IS_COMING_TOO() -> Py { ensure_init(); G_IS_COMING_TOO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IS_COMING_TOO(v: Py) { ensure_init(); G_IS_COMING_TOO.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_TO_WITH() -> Py { ensure_init(); G_OFF_TO_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_TO_WITH(v: Py) { ensure_init(); G_OFF_TO_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THEIR_OWN() -> Py { ensure_init(); G_ON_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THEIR_OWN(v: Py) { ensure_init(); G_ON_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_RIGHT() -> Py { ensure_init(); G_PUT_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_RIGHT(v: Py) { ensure_init(); G_PUT_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_WITH() -> Py { ensure_init(); G_SAID_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_WITH(v: Py) { ensure_init(); G_SAID_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKING() -> Py { ensure_init(); G_TAKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKING(v: Py) { ensure_init(); G_TAKING.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_SOMEONE_ELSE() -> Py { ensure_init(); G__ABOUT_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_SOMEONE_ELSE(v: Py) { ensure_init(); G__ABOUT_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS() -> Py { ensure_init(); G__ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS(v: Py) { ensure_init(); G__ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON() -> Py { ensure_init(); G__A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON(v: Py) { ensure_init(); G__A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__GOING() -> Py { ensure_init(); G__GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GOING(v: Py) { ensure_init(); G__GOING.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_COMING_ALONG() -> Py { ensure_init(); G__NOT_COMING_ALONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_COMING_ALONG(v: Py) { ensure_init(); G__NOT_COMING_ALONG.with(|c| *c.borrow_mut() = v); }
pub fn g__STOP() -> Py { ensure_init(); G__STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STOP(v: Py) { ensure_init(); G__STOP.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_MONTHS() -> Py { ensure_init(); G__THE_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_MONTHS(v: Py) { ensure_init(); G__THE_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g__TO_THE_TRIP() -> Py { ensure_init(); G__TO_THE_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TO_THE_TRIP(v: Py) { ensure_init(); G__TO_THE_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Who is going with them, and the ask that wants it back.\n\n\"im travelling with my mum\" was acknowledged and then dropped: \"who is coming with me\" three turns\nlater got a search shrug, and \"who am i going with\" got \"You haven't told me your name yet.\" The\ncompanion is a fact about the chat, said by the person in their own words, and a friend answers it\nfrom what they were told.\n\nThis module keeps the companion and answers the ask for it. A line that puts the companion right\n(\"actually its my sister not my wife\") replaces the one held, so the second answer is the true one.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s__GOING(py_str("(?:travelling|traveling|flying|driving|heading|off|away|on holiday|going away|going)"));
    s_SAID_WITH(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:i'?m|im|i am|we'?re|were|we are)\\s+"), &g__GOING()), &py_str("\\s+(?:off\\s+|away\\s+|there\\s+|down\\s+)?with\\s+(?P<who>[a-z][a-z'\\u2019 -]{1,40})")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_TO_WITH(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:i'?m|im|i am|we'?re|were|we are)\\s+"), &g__GOING()), &py_str("\\s+(?:to|into|for|back to)\\s+[a-z][a-z'\\u2019 -]{1,25}?\\s+with\\s+(?P<who>[a-z][a-z'\\u2019 -]{1,40})")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKING(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m|im|i am|we'?re|were|we are)\\s+(?:taking|bringing)\\s+(?P<who>[a-z][a-z'\\u2019 -]{1,40})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COMING_WITH(stdlib_call("re", "compile", &[py_str("\\b(?P<who>(?:my|our)\\s+[a-z][a-z'\\u2019-]{1,20}(?:\\s+and\\s+(?:her|his|their|my|our)\\s+[a-z][a-z'\\u2019-]{1,20})?)(?:\\s+is|\\s+are|'?s|s)?\\s+(?:coming|going|travelling|traveling)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_COMING_ALONG(stdlib_call("re", "compile", &[py_str("\\b(?:to stay|to visit|to live|to dinner|to lunch|to pick me up|home|back|round|over|out|down|up)\\b|\\bin\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__STOP(stdlib_call("re", "compile", &[py_str("\\b(?:to|from|on|at|in(?!-law)|for|next|this|tomorrow|tonight|because|but|so|then|and then|who|which|when|if|as|while|by|via|through|until|till|after|before)\\b|[,.;:!?]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PERSON(stdlib_call("re", "compile", &[py_str("^(?:(?:my|our|the|a|an|his|her|their|two|three|four|five|six|a few|some)\\s+)?(?:mum|mom|mam|mummy|mother|dad|father|daddy|papa|son|daughter|wife|husband|partner|sister|brother|kids?|children|friend|friends|mate|mates|parents|family|granny|gran|nan|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|boyfriend|girlfriend|colleague|neighbou?r|in-?laws?|mother-in-law|father-in-law)(?:\\s+and\\s+.*)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EITHER_SIDE(stdlib_call("re", "compile", &[py_str("\\b(?:me|i)\\s+and\\s+(?P<who>(?:my|our|the)\\s+[a-z][a-z'\\u2019-]{1,20}(?:\\s+and\\s+(?:her|his|their)\\s+[a-z][a-z'\\u2019-]{1,20})?)\\s+(?:are|is|were|was)\\s+(?:going|off|flying|driving|travelling|traveling|heading)\\b|\\b(?P<other>(?:my|our|the)\\s+[a-z][a-z'\\u2019-]{1,20})\\s+and\\s+(?:me|i)\\s+(?:are|were)\\s+(?:going|off|flying|driving|travelling|traveling|heading)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK(stdlib_call("re", "compile", &[py_str("\\bwho(?:'?s| is| are| else is| else)? (?:else )?(?:coming|going|travelling|traveling|flying|driving)\\s+(?:with|along with)\\s+(?:me|us)\\b|\\bwho (?:am i|are we) (?:going|travelling|traveling|flying|driving|heading)\\s+with\\b|\\bis (?:anyone|anybody|someone|somebody) (?:coming|going|travelling|traveling)\\s+with\\s+(?:me|us)\\b|\\bam i (?:going|travelling|traveling)\\s+(?:with anyone|with anybody|alone|on my own)\\b|\\bwho did (?:i|you) say (?:is |was )?(?:coming|going)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IS_COMING_TOO(stdlib_call("re", "compile", &[py_str("\\bis\\s+(?P<who>(?:my|our)\\s+[a-z][a-z'\\u2019 -]{1,20}?)\\s+(?:coming|going|travelling|traveling|flying|driving)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_FOR_A_DETAIL(stdlib_call("re", "compile", &[py_str("^\\W*(?:and |so |but |oh |right |also |ok(?:ay)?[ ,]+)*(?:when(?:'?s| is| are| was| am| do| does| did| will| would)?\\b|where\\b|why\\b|which (?:day|date)\\b|what (?:day|date|time)\\b|how (?:long|soon|many days)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ABOUT_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\bwith\\s+(?:my|our|your)\\s+\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_RIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:actually |no,? )?(?:it'?s |its |it is |they'?re |they are |that'?s |thats |it was |the one is )?(?P<new>[a-z][a-z'\\u2019 -]{1,25}?)\\s+not\\s+(?:the |my |our )?(?P<old>[a-z][a-z'\\u2019 -]{1,25})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:on my own|alone|by myself|solo|just me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS(stdlib_call("re", "compile", &[py_str("^\\W*(?:is|are|was|were|will|would|can|could|should|shall|do|does|did|who|what|when|where|why|how)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_MONTHS(stdlib_call("re", "compile", &[py_str("\\b(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sept|sep|oct|nov|dec)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TO_THE_TRIP(stdlib_call("re", "compile", &[py_str("\\b(?:going|off|flying|driving|travelling|traveling|heading|moving|off to|away to)\\s+(?:to|into|for|off to|down to|back to)\\s+(?P<rest>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _bare(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut words: Py = Py::None;
    let _ = py_str("The companion without the possessive in front: \"my wife\" -> \"wife\".");
    words = mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("the"), py_str("a"), py_str("an"), py_str("your")]), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _the_companion(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut first: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The companion a line names, as they said it, or \"\" when the line names nobody.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b([a-z]+)\\s+in[\\s-]+(laws?)\\b")); __args.push(py_str("\\1-in-\\2")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_SAID_WITH(), g_OFF_TO_WITH(), g_TAKING(), g_COMING_WITH(), g_EITHER_SIDE()]));
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    who = { let __b = mcall(&match_.clone(), "group", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("other")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    cut = mcall(&g__STOP(), "search", &[who.clone()]);
    if py_truthy(&cut.clone()) {
    who = slice_or_raise(&who.clone(), &Py::None, &mcall(&cut.clone(), "start", &[]), &Py::None);
    }
    who = mcall(&who.clone(), "strip", &[py_str(" ,.;:-'’")]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&who.clone(), &py_str(" and ")))) {
    let __unpk2 = py_unpack(&mcall(&who.clone(), "split", &[py_str(" and "), Py::Int(1)]), 2, None);
    first = __unpk2[0].clone();
    rest = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&g__A_PERSON(), "match", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_PERSON(), "match", &[op_add(&py_str("my "), &rest.clone())]) } }) {
    who = op_add(&op_add(&first.clone(), &py_str(" and ")), &rest.clone());
    } else {
    who = first.clone();
    }
    }
    words = mcall(&who.clone(), "split", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pattern.clone(), &g_COMING_WITH())); if !py_truthy(&__b) { __b } else { mcall(&g__NOT_COMING_ALONG(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__A_PERSON(), "match", &[who.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_PERSON(), "match", &[op_add(&py_str("my "), &who.clone())]) } }))) {
    continue;
    }
    return who.clone();
    }
    return py_str("");
    Py::None
}
pub fn _as_you_would_say_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut yours: Py = Py::None;
    let _ = py_str("The companion said back to the person: \"my mum\" -> \"your mum\".");
    said = mcall(&who.clone(), "strip", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("my "), py_str("your ")]), py_tuple(vec![py_str("our "), py_str("your ")]), py_tuple(vec![py_str("i "), py_str("you ")]), py_tuple(vec![py_str("me"), py_str("you")]), py_tuple(vec![py_str("we "), py_str("you ")]), py_tuple(vec![py_str("myself"), py_str("yourself")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    mine = __unpk2[0].clone();
    yours = __unpk2[1].clone();
    if py_truthy(&mcall(&said.clone(), "startswith", &[mine.clone()])) {
    return op_add(&yours.clone(), &slice_or_raise(&said.clone(), &call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None));
    }
    }
    return said.clone();
    Py::None
}
pub fn _more_than_one(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut low: Py = Py::None;
    let _ = py_str("Whether the companion is more than one person, so the verb is \"are\".");
    low = mcall(&who.clone(), "lower", &[]);
    return { let __b = Py::Bool(py_contains(&low.clone(), &py_str(" and "))); if py_truthy(&__b) { __b } else { mcall(&geti(&mcall(&low.clone(), "split", &[]), &op_neg(&Py::Int(1))), "endswith", &[py_str("s")]) } };
    Py::None
}
pub fn _the_correction(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut new: Py = Py::None;
    let mut old: Py = Py::None;
    let _ = py_str("The companion the line puts right, or \"\" when the line corrects something else.");
    held = mcall(&memory.clone(), "get", &[py_str("company")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&held.clone(), "get", &[py_str("who")]))) } }) {
    return py_str("");
    }
    match_ = mcall(&g_PUT_RIGHT(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    old = _bare(&[mcall(&match_.clone(), "group", &[py_str("old")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&old.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&old.clone(), &_bare(&[geti(&held.clone(), &py_str("who"))]))) } }) {
    return py_str("");
    }
    new = mcall(&mcall(&match_.clone(), "group", &[py_str("new")]), "strip", &[py_str(" ,.;:-")]);
    return if py_truthy(&{ let __b = mcall(&g__A_PERSON(), "match", &[new.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_PERSON(), "match", &[op_add(&py_str("my "), &new.clone())]) } }) { new.clone() } else { py_str("") };
    Py::None
}
pub fn _a_trip_is_here(args: &[Py]) -> Py {
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
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("Whether the chat holds a plan, which is what the ask for company is about.");
    user = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { Py::None };
    plans = { let __b = mcall(&{ let __b = user.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut plan: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _on_their_own(args: &[Py]) -> Py {
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
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("Whether the plan they gave says they are going alone.");
    user = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { Py::None };
    plans = { let __b = mcall(&{ let __b = user.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut plan: Py = __item.clone(); __out.push(mcall(&g_ON_THEIR_OWN(), "search", &[call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_phrase_a_frame_can_say(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut found: Py = Py::None;
    let _ = py_str("The trip as a phrase, from a plan that may be the whole line it was said in.\n\n    \"me and the kids are going to spain in july\" is how a person says the plan, and it is not\n    what a frame can put in a sentence: the two frames here read it back as \"the kids along for\n    me and the kids are going to Spain in july, then\" and \"the kids coming too. the kids and i\n    are off to Spain in july goes better shared\", which is the person's own line pasted into the\n    reply. Cut at the travel verb and the phrase is theirs: \"Spain in july\".\n    ");
    plan = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .")]);
    found = mcall(&g__TO_THE_TRIP(), "search", &[plan.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "strip", &[py_str(" .")]) } }) {
    plan = mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "strip", &[py_str(" .")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&plan.clone()))) {
    return py_str("");
    }
    plan = mcall(&g__THE_MONTHS(), "sub", &[{ let found = found.clone(); let plan = plan.clone(); py_func("lambda", move |args: &[Py]| { let found = found.clone(); let plan = plan.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); mcall(&mcall(&m.clone(), "group", &[Py::Int(0)]), "capitalize", &[]) }) }, plan.clone()]);
    return op_add(&mcall(&slice_or_raise(&plan.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&plan.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _the_trip_on_the_table(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("The trip the chat is on, in as few words as the plan was said: \"Cornwall in july\".\n\n    A plan is not always a trip: \"im making a birthday cake\" files a plan too, and \"my son is\n    coming too\" after it is about the cake, not about somebody travelling. The place is what\n    makes it a trip, and the plan has to name it.\n    ");
    user = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { Py::None };
    places = { let __b = mcall(&{ let __b = user.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&places.clone()))) {
    return py_str("");
    }
    place = mcall(&call_builtin("str", &[geti(&places.clone(), &op_neg(&Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .")]);
    plan = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = user.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&place.clone())); if py_truthy(&__b) { __b } else { { let __b = plan.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&mcall(&plan.clone(), "lower", &[]), &mcall(&place.clone(), "lower", &[]))) } } } }) {
    return py_str("");
    }
    return { let __b = _the_phrase_a_frame_can_say(&[plan.clone()]); if py_truthy(&__b) { __b } else { place.clone() } };
    Py::None
}
pub fn _new_to_the_chat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut bare: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether this is the first line of the chat that names them.\n\n    A companion said once is news; said again it is not, and a line that comes back to them\n    (\"my mum is coming too, shes 60\") is a detail about her, not the trip gaining a person.\n    ");
    bare = _bare(&[who.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return Py::Bool(false);
    }
    said = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "strip", &[]), &mcall(&call_builtin("str", &[{ let __b = mcall(&mcall(&memory.clone(), "get", &[py_str("understood"), py_dict(vec![])]), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))) } }) {
    turns = slice_or_raise(&turns.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&said.clone(), "search", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _the_one_person_asked_about(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Whether that very person is coming, or None when the line does not ask about one.\n\n    \"we are going to cornwall in july\" / \"my mum is coming too\" / \"is my mum coming too?\" got a\n    search hedge, which is the one answer a chat that has just been told cannot give. A person the\n    chat was not told about is still answered honestly - nothing was said about them.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { mcall(&g__NOT_COMING_ALONG(), "search", &[low.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ASKS_FOR_A_DETAIL(), "search", &[low.clone()])) {
    return Py::None;
    }
    found = mcall(&g_IS_COMING_TOO(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    named = _bare(&[mcall(&found.clone(), "group", &[py_str("who")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__A_PERSON(), "match", &[named.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_PERSON(), "match", &[op_add(&py_str("my "), &named.clone())]))) } } } }) {
    return Py::None;
    }
    held = mcall(&memory.clone(), "get", &[py_str("company")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&held.clone(), "get", &[py_str("who")]))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_a_trip_is_here(&[memory.clone()])))) {
    return Py::None;
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("Nothing said about your "), &named.clone()), &py_str(" - you have not said who is coming with you.")), op_add(&op_add(&py_str("You have not told me who is coming, so I cannot say about your "), &named.clone()), &py_str("."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    }
    who = _as_you_would_say_it(&[geti(&held.clone(), &py_str("who"))]);
    if py_truthy(&Py::Bool(py_eq(&named.clone(), &_bare(&[geti(&held.clone(), &py_str("who"))])))) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&py_str("Yes - you said "), &who.clone()), &py_str(" is coming with you.")), op_add(&op_add(&py_str("You said "), &who.clone()), &py_str(" is coming, so yes.")), op_add(&mcall(&who.clone(), "capitalize", &[]), &py_str(" - that is who you said was coming."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("You only said "), &who.clone()), &py_str(" was coming - nothing about your ")), &named.clone()), &py_str(".")), op_add(&op_add(&op_add(&op_add(&py_str("Nothing said about your "), &named.clone()), &py_str(" - it was ")), &who.clone()), &py_str(" you told me about."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _the_companion_joins(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut options: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The line puts somebody on the plan the chat already has, or None.\n\n    \"we are going to cornwall in july\" / \"my mum is coming too\": the store keeps the companion,\n    and the turn is the trip gaining a person. That is what a friend takes up - \"your mum as\n    well, good\". Left alone, the carry reader answered it with the tail of the answer just given\n    (\"July takes the school holidays, so the prices only go one way\"), which is the one thing a\n    second sentence of a conversation must not be.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&low.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g__ASKS(), "match", &[low.clone()]) } } } }) {
    return Py::None;
    }
    place = _the_trip_on_the_table(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return Py::None;
    }
    who = _the_companion(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_new_to_the_chat(&[who.clone(), memory.clone()]))) } }) {
    return Py::None;
    }
    said = _as_you_would_say_it(&[who.clone()]);
    if py_truthy(&_the_plan_is_this_line(&[text.clone(), memory.clone()])) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("Nice - "), &place.clone()), &py_str(" with ")), &said.clone()), &py_str(".")), op_add(&op_add(&op_add(&op_add(&py_str("Lovely - "), &place.clone()), &py_str(" with ")), &said.clone()), &py_str(".")), op_add(&op_add(&op_add(&place.clone(), &py_str(" with ")), &said.clone()), &py_str(" - good."))])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    }
    options = py_list(vec![op_add(&op_add(&op_add(&mcall(&said.clone(), "capitalize", &[]), &py_str(" as well - good. ")), &place.clone()), &py_str(" is better with company.")), op_add(&op_add(&op_add(&op_add(&py_str("Nice, "), &said.clone()), &py_str(" coming too. ")), &place.clone()), &py_str(" goes better shared.")), op_add(&op_add(&op_add(&op_add(&py_str("Good - "), &said.clone()), &py_str(" along for ")), &place.clone()), &py_str(", then."))]);
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _the_plan_is_this_line(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut plan: Py = Py::None;
    let _ = py_str("Whether the plan the chat holds is the line being answered, said in one breath.\n\n    \"me and the kids are going to spain in july\" makes the plan and names who is going: the\n    companion is not joining anything, and the reply is the trip with them on it.\n    ");
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .")]);
    plan = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .")]);
    return { let __b = call_builtin("bool", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[plan.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&plan.clone(), &line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&plan.clone(), &line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &plan.clone())) } } } } } } } };
    Py::None
}
pub fn observe(args: &[Py]) -> Py {
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
    let mut fixed: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Keep the companion the line names, or the one it puts right. None either way.");
    fixed = _the_correction(&[text.clone(), memory.clone()]);
    if py_truthy(&fixed.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("company"), py_dict(vec![(py_str("who"), fixed.clone()), (py_str("turns"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("corrected"), Py::Bool(true))]));
    return Py::None;
    }
    who = _the_companion(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&memory.clone(), &py_str("company"), py_dict(vec![(py_str("who"), who.clone()), (py_str("turns"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)))]));
    return Py::None;
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut low: Py = Py::None;
    let mut one: Py = Py::None;
    let mut options: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The companion, said back, or the addition taken up. None when the line does neither.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ASK(), "search", &[low.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g__ABOUT_SOMEONE_ELSE(), "search", &[low.clone()]) } }) {
    one = _the_one_person_asked_about(&[text.clone(), memory.clone()]);
    if py_truthy(&one.clone()) {
    return one.clone();
    }
    return _the_companion_joins(&[text.clone(), memory.clone()]);
    }
    held = mcall(&memory.clone(), "get", &[py_str("company")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&held.clone(), "get", &[py_str("who")]))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_a_trip_is_here(&[memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_on_their_own(&[memory.clone()])) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![py_str("You said you were going on your own."), py_str("On your own, you said - nobody else was named.")])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![py_str("You haven't said who's going with you."), py_str("Nobody yet - you haven't said who is going with you.")])); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::None } };
    }
    who = _as_you_would_say_it(&[geti(&held.clone(), &py_str("who"))]);
    verb = if py_truthy(&_more_than_one(&[who.clone()])) { py_str("are") } else { py_str("is") };
    if py_truthy(&mcall(&mcall(&low.clone(), "rstrip", &[]), "endswith", &[py_tuple(vec![py_str("alone"), py_str("on my own")])])) {
    return op_add(&op_add(&op_add(&op_add(&py_str("Not on your own - "), &who.clone()), &py_str(" ")), &verb.clone()), &py_str(" coming with you."));
    }
    options = py_list(vec![op_add(&op_add(&op_add(&op_add(&py_str("You said "), &who.clone()), &py_str(" ")), &verb.clone()), &py_str(" coming with you.")), op_add(&mcall(&who.clone(), "capitalize", &[]), &py_str(" - that is who you said you were going with.")), op_add(&op_add(&op_add(&op_add(&py_str("You said "), &who.clone()), &py_str(" ")), &verb.clone()), &py_str(" coming with you, and that is all I know."))]);
    if py_truthy(&mcall(&who.clone(), "startswith", &[py_str("your ")])) {
    let _ = mcall(&options.clone(), "insert", &[Py::Int(1), op_add(&op_add(&py_str("Your "), &geti(&mcall(&who.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(1))), &py_str(", you told me."))]);
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(options.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    return { let __b = reply.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.company".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK" => Some(g_ASK()),
            "ASKS_FOR_A_DETAIL" => Some(g_ASKS_FOR_A_DETAIL()),
            "COMING_WITH" => Some(g_COMING_WITH()),
            "EITHER_SIDE" => Some(g_EITHER_SIDE()),
            "IS_COMING_TOO" => Some(g_IS_COMING_TOO()),
            "OFF_TO_WITH" => Some(g_OFF_TO_WITH()),
            "ON_THEIR_OWN" => Some(g_ON_THEIR_OWN()),
            "PUT_RIGHT" => Some(g_PUT_RIGHT()),
            "SAID_WITH" => Some(g_SAID_WITH()),
            "TAKING" => Some(g_TAKING()),
            "_ABOUT_SOMEONE_ELSE" => Some(g__ABOUT_SOMEONE_ELSE()),
            "_ASKS" => Some(g__ASKS()),
            "_A_PERSON" => Some(g__A_PERSON()),
            "_GOING" => Some(g__GOING()),
            "_NOT_COMING_ALONG" => Some(g__NOT_COMING_ALONG()),
            "_STOP" => Some(g__STOP()),
            "_THE_MONTHS" => Some(g__THE_MONTHS()),
            "_TO_THE_TRIP" => Some(g__TO_THE_TRIP()),
            "_a_trip_is_here" => Some(py_func("_a_trip_is_here", |args: &[Py]| _a_trip_is_here(args))),
            "_as_you_would_say_it" => Some(py_func("_as_you_would_say_it", |args: &[Py]| _as_you_would_say_it(args))),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "_more_than_one" => Some(py_func("_more_than_one", |args: &[Py]| _more_than_one(args))),
            "_new_to_the_chat" => Some(py_func("_new_to_the_chat", |args: &[Py]| _new_to_the_chat(args))),
            "_on_their_own" => Some(py_func("_on_their_own", |args: &[Py]| _on_their_own(args))),
            "_the_companion" => Some(py_func("_the_companion", |args: &[Py]| _the_companion(args))),
            "_the_companion_joins" => Some(py_func("_the_companion_joins", |args: &[Py]| _the_companion_joins(args))),
            "_the_correction" => Some(py_func("_the_correction", |args: &[Py]| _the_correction(args))),
            "_the_one_person_asked_about" => Some(py_func("_the_one_person_asked_about", |args: &[Py]| _the_one_person_asked_about(args))),
            "_the_phrase_a_frame_can_say" => Some(py_func("_the_phrase_a_frame_can_say", |args: &[Py]| _the_phrase_a_frame_can_say(args))),
            "_the_plan_is_this_line" => Some(py_func("_the_plan_is_this_line", |args: &[Py]| _the_plan_is_this_line(args))),
            "_the_trip_on_the_table" => Some(py_func("_the_trip_on_the_table", |args: &[Py]| _the_trip_on_the_table(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "pick" => Some(g_pick()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASK" => s_ASK(v),
            "ASKS_FOR_A_DETAIL" => s_ASKS_FOR_A_DETAIL(v),
            "COMING_WITH" => s_COMING_WITH(v),
            "EITHER_SIDE" => s_EITHER_SIDE(v),
            "IS_COMING_TOO" => s_IS_COMING_TOO(v),
            "OFF_TO_WITH" => s_OFF_TO_WITH(v),
            "ON_THEIR_OWN" => s_ON_THEIR_OWN(v),
            "PUT_RIGHT" => s_PUT_RIGHT(v),
            "SAID_WITH" => s_SAID_WITH(v),
            "TAKING" => s_TAKING(v),
            "_ABOUT_SOMEONE_ELSE" => s__ABOUT_SOMEONE_ELSE(v),
            "_ASKS" => s__ASKS(v),
            "_A_PERSON" => s__A_PERSON(v),
            "_GOING" => s__GOING(v),
            "_NOT_COMING_ALONG" => s__NOT_COMING_ALONG(v),
            "_STOP" => s__STOP(v),
            "_THE_MONTHS" => s__THE_MONTHS(v),
            "_TO_THE_TRIP" => s__TO_THE_TRIP(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.company", module_obj);
    pyrt::xrefs::reg_fn("memory.company", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("memory.company", "_the_companion", py_func("_the_companion", |args: &[Py]| _the_companion(args)));
    pyrt::xrefs::reg_fn("memory.company", "_as_you_would_say_it", py_func("_as_you_would_say_it", |args: &[Py]| _as_you_would_say_it(args)));
    pyrt::xrefs::reg_fn("memory.company", "_more_than_one", py_func("_more_than_one", |args: &[Py]| _more_than_one(args)));
    pyrt::xrefs::reg_fn("memory.company", "_the_correction", py_func("_the_correction", |args: &[Py]| _the_correction(args)));
    pyrt::xrefs::reg_fn("memory.company", "_a_trip_is_here", py_func("_a_trip_is_here", |args: &[Py]| _a_trip_is_here(args)));
    pyrt::xrefs::reg_fn("memory.company", "_on_their_own", py_func("_on_their_own", |args: &[Py]| _on_their_own(args)));
    pyrt::xrefs::reg_fn("memory.company", "_the_phrase_a_frame_can_say", py_func("_the_phrase_a_frame_can_say", |args: &[Py]| _the_phrase_a_frame_can_say(args)));
    pyrt::xrefs::reg_fn("memory.company", "_the_trip_on_the_table", py_func("_the_trip_on_the_table", |args: &[Py]| _the_trip_on_the_table(args)));
    pyrt::xrefs::reg_fn("memory.company", "_new_to_the_chat", py_func("_new_to_the_chat", |args: &[Py]| _new_to_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.company", "_the_one_person_asked_about", py_func("_the_one_person_asked_about", |args: &[Py]| _the_one_person_asked_about(args)));
    pyrt::xrefs::reg_fn("memory.company", "_the_companion_joins", py_func("_the_companion_joins", |args: &[Py]| _the_companion_joins(args)));
    pyrt::xrefs::reg_fn("memory.company", "_the_plan_is_this_line", py_func("_the_plan_is_this_line", |args: &[Py]| _the_plan_is_this_line(args)));
    pyrt::xrefs::reg_fn("memory.company", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("memory.company", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_access("memory.company", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("memory.company", "ASKS_FOR_A_DETAIL", g_ASKS_FOR_A_DETAIL, s_ASKS_FOR_A_DETAIL);
    pyrt::xrefs::reg_access("memory.company", "COMING_WITH", g_COMING_WITH, s_COMING_WITH);
    pyrt::xrefs::reg_access("memory.company", "EITHER_SIDE", g_EITHER_SIDE, s_EITHER_SIDE);
    pyrt::xrefs::reg_access("memory.company", "IS_COMING_TOO", g_IS_COMING_TOO, s_IS_COMING_TOO);
    pyrt::xrefs::reg_access("memory.company", "OFF_TO_WITH", g_OFF_TO_WITH, s_OFF_TO_WITH);
    pyrt::xrefs::reg_access("memory.company", "ON_THEIR_OWN", g_ON_THEIR_OWN, s_ON_THEIR_OWN);
    pyrt::xrefs::reg_access("memory.company", "PUT_RIGHT", g_PUT_RIGHT, s_PUT_RIGHT);
    pyrt::xrefs::reg_access("memory.company", "SAID_WITH", g_SAID_WITH, s_SAID_WITH);
    pyrt::xrefs::reg_access("memory.company", "TAKING", g_TAKING, s_TAKING);
    pyrt::xrefs::reg_access("memory.company", "_ABOUT_SOMEONE_ELSE", g__ABOUT_SOMEONE_ELSE, s__ABOUT_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("memory.company", "_ASKS", g__ASKS, s__ASKS);
    pyrt::xrefs::reg_access("memory.company", "_A_PERSON", g__A_PERSON, s__A_PERSON);
    pyrt::xrefs::reg_access("memory.company", "_GOING", g__GOING, s__GOING);
    pyrt::xrefs::reg_access("memory.company", "_NOT_COMING_ALONG", g__NOT_COMING_ALONG, s__NOT_COMING_ALONG);
    pyrt::xrefs::reg_access("memory.company", "_STOP", g__STOP, s__STOP);
    pyrt::xrefs::reg_access("memory.company", "_THE_MONTHS", g__THE_MONTHS, s__THE_MONTHS);
    pyrt::xrefs::reg_access("memory.company", "_TO_THE_TRIP", g__TO_THE_TRIP, s__TO_THE_TRIP);
    pyrt::xrefs::reg_access("memory.company", "pick", g_pick, s_pick);
}
