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
thread_local! { static G__ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK() -> Py { ensure_init(); G__ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK(v: Py) { ensure_init(); G__ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__DATES() -> Py { ensure_init(); G__DATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DATES(v: Py) { ensure_init(); G__DATES.with(|c| *c.borrow_mut() = v); }
pub fn g__LONG() -> Py { ensure_init(); G__LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LONG(v: Py) { ensure_init(); G__LONG.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMES() -> Py { ensure_init(); G__NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMES(v: Py) { ensure_init(); G__NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The dates of famous things: when it was built, discovered, introduced or first happened.\n\n@lemon-wasp measured five `when` lines on main and each of them fell over a different way: \"when\nwas the eiffel tower built\" and \"when was penicillin discovered\" were a search punt, \"when did the\nempire windrush arrive\" was told the bot cannot see the news, \"when did the poll tax start\" got\nthe self-assessment card because of the word tax, and \"when was the last ice age\" was answered\n\"You haven't told me about anything coming up yet\".\n\nThe history table in `history.py` already answers `who was brunel` and `when did the titanic sink`\nby the thing the line names, so the mechanism was right and the rows were missing: a date is a\nfact about a thing, and the thing is the key. Every reply opens with the date, because that is\nthe half a `when` line asks about. A domain of its own rather than more rows in `history.py`,\nwhich is at the publisher's size ceiling.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("history"), py_str("science")]));
    fn __data1() -> Py { py_tuple(vec![py_str("dentist"), py_str("waiting"), py_str("prescription"), py_str("appointment"), py_str("gp"), py_str("job"), py_str("work for"), py_str("complaint"), py_str("complain"), py_str("charging"), py_str("charge"), py_str("private"), py_str("pay"), py_str("cost"), py_str("free"), py_str("eye"), py_str("eye test"), py_str("dental"), py_str("ambulance"), py_str("waiting list")]) }
    s__DATES(py_dict(vec![(py_str("eiffel-tower"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("eiffel tower"), py_str("eiffel")])), (py_str("tell"), py_str("The Eiffel Tower was finished in March 1889 for the Paris world's fair, the Exposition Universelle, and opened to the public in May that year. It is 300 metres to the top of the flagpole, was the tallest building in the world until 1930, and was meant to stand for twenty years."))])), (py_str("penicillin"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("penicillin"), py_str("fleming")])), (py_str("off"), py_tuple(vec![py_str("who discovered"), py_str("who found"), py_str("who invented"), py_str("who created")])), (py_str("tell"), py_str("Alexander Fleming noticed the mould killing his bacteria at St Mary's Hospital in London in September 1928, and published it the next year. Howard Florey and Ernst Chain turned it into a medicine at Oxford in 1940-41, and the first patient was treated in 1941."))])), (py_str("last-ice-age"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("ice age"), py_str("ice ages"), py_str("last ice age"), py_str("glaciation"), py_str("glacial period")])), (py_str("off"), py_tuple(vec![py_str("film"), py_str("films"), py_str("movie"), py_str("mammoth"), py_str("the croods"), py_str("netflix"), py_str("cinema"), py_str("dvd"), py_str("sequel"), py_str("release")])), (py_str("off_re"), stdlib_call("re", "compile", &[py_str("\\bice ages?\\b(?:\\s+\\d|\\s+(?:out|on)\\b|\\s+(?:on|at) (?:tv|netflix|the cinema|bbc|itv|channel|disney))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("tell"), py_str("The last ice age ended about 11,700 years ago. The coldest part of it, the last glacial maximum, was about 20,000 years ago, when ice sheets reached as far south as the Thames and the Midlands.")), (py_str("long"), py_str("The last ice age ran for roughly 100,000 years and ended about 11,700 years ago, which is when the glaciers melted back and farming began. The coldest stretch was about 20,000 years ago."))])), (py_str("poll-tax"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("poll tax"), py_str("community charge")])), (py_str("tell"), py_str("The poll tax, officially the community charge, started in Scotland in April 1989 and in England and Wales in April 1990, and was replaced by the council tax in 1993. It charged every adult a flat amount instead of a rate on the value of the home, and the protests against it helped bring down Margaret Thatcher."))])), (py_str("empire-windrush"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("empire windrush"), py_str("windrush")])), (py_str("tell"), py_str("The Empire Windrush docked at Tilbury on 22 June 1948, bringing 1,027 passengers from the Caribbean, most of them Jamaican, who had answered an ad for workers in Britain. It was one ship among many, but the date became the shorthand for the start of that generation's arrival."))])), (py_str("statue-of-liberty"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("statue of liberty"), py_str("liberty island")])), (py_str("tell"), py_str("The Statue of Liberty was dedicated on 28 October 1886 in New York Harbour, a gift from France for the centenary of American independence. It was designed by Frederic Auguste Bartholdi with a iron frame by Gustave Eiffel, and the pedestal was paid for by public subscription."))])), (py_str("nhs-founded"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("nhs"), py_str("national health service")])), (py_str("off"), __data1()), (py_str("tell"), py_str("The NHS started on 5 July 1948, when the National Health Service Act came into force in England and Wales under Aneurin Bevan, the health minister. It was the first health service anywhere to be free at the point of use and paid for out of taxation."))]))]));
    s__NAMES(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__DATES(), "values", &[])) { let mut row: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&row.clone(), &py_str("names"))) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("when"), py_str("when was"), py_str("when did"), py_str("what year"), py_str("which year"), py_str("how old"), py_str("how long ago"), py_str("how long did"), py_str("how long was"), py_str("how long has"), py_str("how long have"), py_str("lasted"), py_str("who built"), py_str("built"), py_str("discovered"), py_str("invented"), py_str("founded"), py_str("started")]) }
    s__ASK(__data2());
    s__LONG(stdlib_call("re", "compile", &[py_str("\\b(?:how long|long|lasted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("the-date-of-a-thing")); __args.push(py_tuple(vec![g__NAMES(), g__ASK()])); __args.push(py_func("the_date", |args: &[Py]| the_date(args))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("guard".to_string(), py_func("names_a_row", |args: &[Py]| names_a_row(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
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
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The line as typed and as read, so a name survives whatever the reader repairs.");
    reading = { let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    parts = py_tuple(vec![mcall(&reading.clone(), "get", &[py_str("plain")]), mcall(&reading.clone(), "get", &[py_str("clean")]), mcall(&reading.clone(), "get", &[py_str("text")])]);
    return mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }]), "lower", &[]);
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
    let mut best: Py = Py::None;
    let mut best_len: Py = Py::None;
    let mut name: Py = Py::None;
    let mut row: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The row a line names, longest name first, or None when it names none of them.");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0)]), 2, None);
    best = __unpk1[0].clone();
    best_len = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g__DATES(), "values", &[]));
    for __item in __iter2 {
    row = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&row.clone(), "get", &[py_str("off"), py_tuple(vec![])])) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&mcall(&row.clone(), "get", &[py_str("off_re")]), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&geti(&row.clone(), &py_str("off_re")), "search", &[text.clone()]) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&geti(&row.clone(), &py_str("names")));
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &best_len.clone()); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![row.clone(), call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    best = __unpk4[0].clone();
    best_len = __unpk4[1].clone();
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _named(args: &[Py]) -> Py {
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
    return _the_row(&[text.clone()]);
    Py::None
}
pub fn the_date(args: &[Py]) -> Py {
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
    let mut row: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The date of the thing the line names, or None so another row can answer.");
    text = _said(&[reading.clone()]);
    row = _the_row(&[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&row.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&row.clone(), "get", &[py_str("long")]); if !py_truthy(&__b) { __b } else { mcall(&g__LONG(), "search", &[text.clone()]) } }) {
    return geti(&row.clone(), &py_str("long"));
    }
    return geti(&row.clone(), &py_str("tell"));
    Py::None
}
pub fn names_a_row(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let _ = py_str("The guard is asked while the line is scored, so a row this table does not hold is not\n    written off by the table's own bias: \"why did world war one start\" is not a date.");
    return Py::Bool(!py_is(&_the_row(&[_said(&[reading.clone()])]), &Py::None));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.dates".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_ASK" => Some(g__ASK()),
            "_DATES" => Some(g__DATES()),
            "_LONG" => Some(g__LONG()),
            "_NAMES" => Some(g__NAMES()),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_the_row" => Some(py_func("_the_row", |args: &[Py]| _the_row(args))),
            "entry" => Some(g_entry()),
            "names_a_row" => Some(py_func("names_a_row", |args: &[Py]| names_a_row(args))),
            "the_date" => Some(py_func("the_date", |args: &[Py]| the_date(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "_ASK" => s__ASK(v),
            "_DATES" => s__DATES(v),
            "_LONG" => s__LONG(v),
            "_NAMES" => s__NAMES(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.dates", module_obj);
    pyrt::xrefs::reg_fn("knowledge.dates", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("knowledge.dates", "_the_row", py_func("_the_row", |args: &[Py]| _the_row(args)));
    pyrt::xrefs::reg_fn("knowledge.dates", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("knowledge.dates", "the_date", py_func("the_date", |args: &[Py]| the_date(args)));
    pyrt::xrefs::reg_fn("knowledge.dates", "names_a_row", py_func("names_a_row", |args: &[Py]| names_a_row(args)));
    pyrt::xrefs::reg_access("knowledge.dates", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.dates", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.dates", "_ASK", g__ASK, s__ASK);
    pyrt::xrefs::reg_access("knowledge.dates", "_DATES", g__DATES, s__DATES);
    pyrt::xrefs::reg_access("knowledge.dates", "_LONG", g__LONG, s__LONG);
    pyrt::xrefs::reg_access("knowledge.dates", "_NAMES", g__NAMES, s__NAMES);
    pyrt::xrefs::reg_access("knowledge.dates", "entry", g_entry, s_entry);
}
