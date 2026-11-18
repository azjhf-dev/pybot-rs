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
thread_local! { static G_a_thing_has_stopped: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.pensions: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_a_thing_has_stopped() -> Py { ensure_init(); G_a_thing_has_stopped.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_thing_has_stopped(v: Py) { ensure_init(); G_a_thing_has_stopped.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Money nobody explains out loud: when the state pension starts, and what it pays.\n\nThe state pension is one number in two forms - the age it starts and the years behind it - and\nthe money domain answers neither: \"what is a state pension\" got the workplace-pension answer and\n\"when do i get my state pension\" got a MoneyHelper pointer, which is the question restated. The\nfigures change every April, so each answer gives the current figure and where the live one is.\n\nThis is a domain of its own because it is the one money table that nothing else shares: the\nbudget, the house deposit, the order to clear debts, a credit file and a mortgage are money.py,\nand a private pension is money.py too, so this file says where it stops.\n");
    s_a_thing_has_stopped(py_func("a_thing_has_stopped", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "a_thing_has_stopped", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pensions.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("money")]));
    fn __data1() -> Py { py_tuple(vec![py_str("state pension"), py_str("pension age"), py_str("retirement age"), py_str("national insurance record"), py_str("national insurance years"), py_str("ni years"), py_str("ni record"), py_str("pension forecast"), py_str("qualifying years"), py_str("my state pension"), py_str("stop work"), py_str("stopping work"), py_str("years of national insurance"), py_str("national insurance contributions"), py_str("ni contributions"), py_str("years of ni"), py_str("national insurance so far"), py_str("until i retire"), py_str("until i can retire"), py_str("until retirement"), py_str("how long until i retire"), py_str("years until i retire"), py_str("stop working"), py_str("stopping working")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("when"), py_str("how old"), py_str("what age"), py_str("how much"), py_str("how long"), py_str("how many years"), py_str("do i get"), py_str("qualify"), py_str("check"), py_str("forecast"), py_str("how many"), py_str("get"), py_str("am i"), py_str("will i"), py_str("can i"), py_str("should i"), py_str("want"), py_str("what happens"), py_str("what")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("workplace"), py_str("opt out"), py_str("opting out"), py_str("contribution"), py_str("contribute"), py_str("contributing"), py_str("pay in"), py_str("put in"), py_str("paying in"), py_str("auto enrolment"), py_str("auto-enrolment"), py_str("scheme"), py_str("should i pay"), py_str("join"), py_str("put it in"), py_str("or save"), py_str("save it"), py_str("pension or"), py_str("put away"), py_str("invest"), py_str("worth"), py_str("debt"), py_str("pay off"), py_str("pay it off"), py_str("save or"), py_str("overdraft"), py_str("loan"), py_str("tax free"), py_str("tax-free"), py_str("lump sum"), py_str("25%"), py_str("take my pension"), py_str("take it early"), py_str("take my pot"), py_str("pot"), py_str("draw down"), py_str("drawdown"), py_str("private pension"), py_str("personal pension"), py_str("sipp"), py_str("how can i retire"), py_str("how do i retire"), py_str("how can i retire early"), py_str("retire early"), py_str("retiring early")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pensions.ENTRIES"); }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("state-pension")); __args.push(py_tuple(vec![__data1(), __data2()])); __args.push(py_func("_state_pension", |args: &[Py]| _state_pension(args))); __kws.extend(vec![("avoid".to_string(), __data3()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "a_thing_has_stopped", &[understood.clone()]))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
}
pub fn _state_pension(args: &[Py]) -> Py {
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
    let mut asks_amount: Py = Py::None;
    let mut asks_record: Py = Py::None;
    let mut asks_when: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut record: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut text: Py = Py::None;
    let mut when: Py = Py::None;
    let mut years: Py = Py::None;
    let _ = py_str("Two questions, both about a number: when it starts, and how much.\n\n    A \"when\" line leads with the age, and the amount follows; a line about retiring early gets\n    one more sentence, because stopping work and claiming are not the same event and people mix\n    them up more than they mix up anything else here.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    record = py_str("Your record is on gov.uk under \"check your National Insurance record\": it lists each year and whether it counts, and gaps can often be filled by paying voluntary contributions.");
    asks_record = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\brecord\\b|\\bcheck\\b.*\\b(?:national insurance|\\bni\\b)"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    when = py_str("The state pension starts at 66 now, and at 67 for anyone reaching that age from 2028, with 68 to come later, so check your own date on gov.uk. It is not automatic: a letter should come about two months before, and if it does not, claim it on gov.uk or by phone.");
    years = py_str("The full new state pension is about £230 a week, £11,500 a year, in 2025-26, on 35 qualifying years of National Insurance; each year short pays about a thirtieth less, and you need at least 10 years to get anything at all.");
    counted = stdlib_call("re", "search", &[py_str("\\b\\d+\\s*years?\\b|\\bhow many years\\b|\\byears? of (?:national insurance|ni)\\b|\\bqualifying\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    asks_when = stdlib_call("re", "search", &[py_str("\\bwhen\\b|how old|what age|\\bdate\\b|\\bretir(?:e|ing|ement)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    asks_amount = stdlib_call("re", "search", &[py_str("\\bhow much\\b|\\bamount\\b|\\bper week\\b|\\ba week\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = asks_record.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_when.clone())) } }) {
    reply = op_add(&op_add(&op_add(&op_add(&record.clone(), &py_str(" ")), &years.clone()), &py_str(" ")), &when.clone());
    } else {
    if py_truthy(&{ let __b = { let __b = asks_amount.clone(); if py_truthy(&__b) { __b } else { counted.clone() } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_when.clone())) } }) {
    reply = op_add(&op_add(&years.clone(), &py_str(" ")), &when.clone());
    } else {
    reply = op_add(&op_add(&when.clone(), &py_str(" ")), &years.clone());
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bretir(?:e|ing|ement)\\b|\\bstop(?:ping)?\\s+work"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_add(&reply.clone(), &py_str(" Stopping work and claiming it are two different things, so retiring early is a question about your own money rather than about the pension."));
    reply = __aug1;
    }
    return reply.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.pensions".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_state_pension" => Some(py_func("_state_pension", |args: &[Py]| _state_pension(args))),
            "a_thing_has_stopped" => Some(g_a_thing_has_stopped()),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ENTRIES" => s_ENTRIES(v),
            "a_thing_has_stopped" => s_a_thing_has_stopped(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.pensions", module_obj);
    pyrt::xrefs::reg_fn("knowledge.pensions", "_state_pension", py_func("_state_pension", |args: &[Py]| _state_pension(args)));
    pyrt::xrefs::reg_access("knowledge.pensions", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.pensions", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.pensions", "a_thing_has_stopped", g_a_thing_has_stopped, s_a_thing_has_stopped);
    pyrt::xrefs::reg_access("knowledge.pensions", "entry", g_entry, s_entry);
}
