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


thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DROPPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_WANTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_CHANGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WISHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WISHES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED() -> Py { ensure_init(); G_CHANGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED(v: Py) { ensure_init(); G_CHANGED.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED_ASK() -> Py { ensure_init(); G_CHANGED_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED_ASK(v: Py) { ensure_init(); G_CHANGED_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DROPPING() -> Py { ensure_init(); G_DROPPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DROPPING(v: Py) { ensure_init(); G_DROPPING.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER() -> Py { ensure_init(); G_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); G_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO() -> Py { ensure_init(); G_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO(v: Py) { ensure_init(); G_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_KEPT() -> Py { ensure_init(); G_KEPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEPT(v: Py) { ensure_init(); G_KEPT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_WANTING() -> Py { ensure_init(); G_NOT_WANTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_WANTING(v: Py) { ensure_init(); G_NOT_WANTING.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_CHANGED() -> Py { ensure_init(); G_SUBJECT_CHANGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_CHANGED(v: Py) { ensure_init(); G_SUBJECT_CHANGED.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS() -> Py { ensure_init(); G_WANTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS(v: Py) { ensure_init(); G_WANTS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_TAIL() -> Py { ensure_init(); G_WHEN_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_TAIL(v: Py) { ensure_init(); G_WHEN_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WISHED() -> Py { ensure_init(); G_WISHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WISHED(v: Py) { ensure_init(); G_WISHED.with(|c| *c.borrow_mut() = v); }
pub fn g_WISHES() -> Py { ensure_init(); G_WISHES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WISHES(v: Py) { ensure_init(); G_WISHES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that reports a change to a fact the store keeps is news, not a question about it.\n\n\"i want to lose a bit of weight\" / \"ive changed my diet\" was answered \"You haven't told me about\nyour diet.\" The person had just told the store about their diet, and the store denied the lot: the\nask pattern reads the words \"my diet\" as a request for the fact, and a line that reports a change\nto the fact is not a request for it.\n\nWhat is missing after \"ive changed my diet\" is not whether they changed it, it is what it is now,\nso the honest reply asks for that (or runs with it when the store holds the value). The same\nshape said as a wish (\"i want to change my diet\", \"should i change my diet\", \"im thinking of\nchanging my diet\") wants a hand with the change rather than a denial.\n\nThe subject is the caller's: it passes the word the fact is kept under (\"diet\", \"budget\", \"my\nhours\") and gets back how the line stands to it, so this is one reading for every fact the store\ncan be asked about rather than a rule for the diet.\n\nTwo things are left alone. A line that gives the new value (\"i changed jobs, im in a hospital\nnow\", \"ive changed my diet to vegan\") is a fact the readers that keep facts have already taken,\nand this reader stands down so they keep it. A how-to ask (\"how do i change my diet\") is a\nrequest for advice, which the advice modules answer with something concrete.\n");
    s_CHANGED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:change|changed|changes|changing|swapped|swapping|switched|switching|given up|give up|gave up|gone off|went off|cut out|cut down|cutting out|cutting down|stopped|stopping|quit|quitting|dropped|ditched|overhauled|tweaked|reworked)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUBJECT_CHANGED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|has|have|had|got)\\s+(?:all\\s+)?(?:changed|different|new)\\b|\\bis\\s+not\\s+the\\s+same\\b|\\bnot the same\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WANTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:want(?:s|ed)?|wanna|need(?:s|ed)?|should|shall|ought|could|would|might|may|thinking|thought|considering|planning|plan|trying|try|hoping|hope|ready|about|worth|idea|better)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOW_TO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+(?:do|does|can|could|should|would|might|to)\\b|\\bwhat(?:'s| is) the best way\\b|\\bany (?:tips|advice|ways)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHEN_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|an|one|two|three|four|five|six|few|couple(?: of)?|\\d+)\\s+(?:day|week|month|year|while)s?(?:\\s+or so)?\\s+ago\\b|\\b(?:last|this)\\s+(?:week|month|year)\\b|\\b(?:recently|lately|these days|nowadays)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    i me my our we you your it its the a an to of and but so or now just really also too very\n    much lot bit still again already even actually all right ok okay well oh ah yeah yes no\n    have has had ve ive weve youve theyve im am is are was were be been do does did done\n    dont doesnt didnt cant\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS(py_dict(vec![(py_str("diet"), py_str("What does a normal day look like now?")), (py_str("budget"), py_str("What are you working with now?")), (py_str("hours"), py_str("What are your hours now?")), (py_str("job"), py_str("What are you doing now?"))]));
    s_WISHES(py_dict(vec![(py_str("diet"), py_str("What is the change for - weight, energy, or something a doctor said? That decides which swaps are worth it.")), (py_str("budget"), py_str("What is the change for - a bill, a save, or paying something off?"))]));
    s_KEPT(py_str("Good to know. "));
    s_DROPPING(py_str("{was}, so I'll drop that - {ask}"));
    s_CHANGED_ASK(py_str("What is it now?"));
    s_WISHED(py_str("What are you trying to change about it? Tell me and I'll be concrete."));
    s_NOT_WANTING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:don'?t|doesn'?t|didn'?t|not|never|no need|no plans|no intention)\\b[^.!?]{0,20}\\b(?:want|wanna|need|going to|gonna|plan|plans|thinking|planning|change|changing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _names(args: &[Py]) -> Py {
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
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut part: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line names the fact: the subject's own words, in any case.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = subject.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    pattern = op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) }) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\\s+"), "join", args))(&__args) }), &py_str("\\b"));
    return Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(line.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None));
    Py::None
}
pub fn _left_over(args: &[Py]) -> Py {
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
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut mine: Py = Py::None;
    let mut part: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of the line that are neither the fact, nor the change, nor filler.");
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = subject.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&words.clone()) {
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) }) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\\s+"), "join", args))(&__args) }), &py_str("\\b"))); __args.push(py_str(" ")); __args.push(mine.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHEN_TAIL(), "sub", args))(&__args) };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_CHANGED(), g_SUBJECT_CHANGED(), g_WANTS()]));
    for __item in __iter1 {
    pattern = __item.clone();
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "sub", args))(&__args) };
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&mine.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FILLER(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "lower", args))(&__args) }, "split", args))(&__args) }, &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn reported(args: &[Py]) -> Py {
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
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut before: Py = Py::None;
    let mut line: Py = Py::None;
    let mut moved: Py = Py::None;
    let _ = py_str("How the line stands to the fact: \"changed\", \"wanting\", or None for anything else.\n\n    `subject` is the fact's own word, as the store keeps it (\"diet\", \"budget\", \"my hours\"), and\n    the line has to name it: \"ive changed my diet\" is about the diet, \"ive changed the bedding\"\n    is about something else and is left to the modules that answer it. A line with a new value in\n    it (\"ive changed my diet to vegan\") is left to the readers that keep facts.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names(args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_TO(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_WANTING(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    moved = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CHANGED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SUBJECT_CHANGED(), "search", args))(&__args) } } };
    if py_truthy(&Py::Bool(!py_truthy(&moved.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(subject.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _left_over(args))(&__args) }) {
    return Py::None;
    }
    before = slice_or_raise(&line.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&moved.clone(), "end", args))(&__args) }, &Py::None);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WANTS(), "search", args))(&__args) }) {
    return py_str("wanting");
    }
    return py_str("changed");
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut was: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "was") { v.clone() }
    else { Py::None };
    let mut ask: Py = Py::None;
    let mut key: Py = Py::None;
    let mut settled: Py = Py::None;
    let _ = py_str("What to say to a line that reports a change to the fact, or asks for one.\n\n    `was` is the value the store holds, when it holds one: the reply names it as the value it\n    stops using, so the person can see the store has moved on rather than being asked to say it\n    all again.\n    ");
    key = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = subject.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("wanting")))) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WISHES(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { g_WISHED() } };
    }
    ask = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { g_CHANGED_ASK() } };
    if py_truthy(&Py::Bool(!py_truthy(&was.clone()))) {
    return op_add(&g_KEPT(), &ask.clone());
    }
    settled = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(was.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("no ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&settled.clone(), "startswith", args))(&__args) }) {
    settled = op_add(&py_str("You don't eat "), &slice_or_raise(&settled.clone(), &Py::Int(3), &Py::None, &Py::None));
    } else {
    settled = op_add(&py_str("You were "), &settled.clone());
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("was".to_string(), settled.clone()), ("ask".to_string(), op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&ask.clone(), &Py::Int(0)), "lower", args))(&__args) }, &slice_or_raise(&ask.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DROPPING(), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.change".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS" => Some(g_ASKS()),
            "CHANGED" => Some(g_CHANGED()),
            "CHANGED_ASK" => Some(g_CHANGED_ASK()),
            "DROPPING" => Some(g_DROPPING()),
            "FILLER" => Some(g_FILLER()),
            "HOW_TO" => Some(g_HOW_TO()),
            "KEPT" => Some(g_KEPT()),
            "NOT_WANTING" => Some(g_NOT_WANTING()),
            "SUBJECT_CHANGED" => Some(g_SUBJECT_CHANGED()),
            "WANTS" => Some(g_WANTS()),
            "WHEN_TAIL" => Some(g_WHEN_TAIL()),
            "WISHED" => Some(g_WISHED()),
            "WISHES" => Some(g_WISHES()),
            "_left_over" => Some(py_func("_left_over", |args: &[Py]| _left_over(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            "reported" => Some(py_func("reported", |args: &[Py]| reported(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS" => s_ASKS(v),
            "CHANGED" => s_CHANGED(v),
            "CHANGED_ASK" => s_CHANGED_ASK(v),
            "DROPPING" => s_DROPPING(v),
            "FILLER" => s_FILLER(v),
            "HOW_TO" => s_HOW_TO(v),
            "KEPT" => s_KEPT(v),
            "NOT_WANTING" => s_NOT_WANTING(v),
            "SUBJECT_CHANGED" => s_SUBJECT_CHANGED(v),
            "WANTS" => s_WANTS(v),
            "WHEN_TAIL" => s_WHEN_TAIL(v),
            "WISHED" => s_WISHED(v),
            "WISHES" => s_WISHES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.change", module_obj);
    pyrt::xrefs::reg_fn("memory.change", "_names", py_func("_names", |args: &[Py]| _names(args)));
    pyrt::xrefs::reg_fn("memory.change", "_left_over", py_func("_left_over", |args: &[Py]| _left_over(args)));
    pyrt::xrefs::reg_fn("memory.change", "reported", py_func("reported", |args: &[Py]| reported(args)));
    pyrt::xrefs::reg_fn("memory.change", "reply", py_func("reply", |args: &[Py]| reply(args)));
    pyrt::xrefs::reg_access("memory.change", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("memory.change", "CHANGED", g_CHANGED, s_CHANGED);
    pyrt::xrefs::reg_access("memory.change", "CHANGED_ASK", g_CHANGED_ASK, s_CHANGED_ASK);
    pyrt::xrefs::reg_access("memory.change", "DROPPING", g_DROPPING, s_DROPPING);
    pyrt::xrefs::reg_access("memory.change", "FILLER", g_FILLER, s_FILLER);
    pyrt::xrefs::reg_access("memory.change", "HOW_TO", g_HOW_TO, s_HOW_TO);
    pyrt::xrefs::reg_access("memory.change", "KEPT", g_KEPT, s_KEPT);
    pyrt::xrefs::reg_access("memory.change", "NOT_WANTING", g_NOT_WANTING, s_NOT_WANTING);
    pyrt::xrefs::reg_access("memory.change", "SUBJECT_CHANGED", g_SUBJECT_CHANGED, s_SUBJECT_CHANGED);
    pyrt::xrefs::reg_access("memory.change", "WANTS", g_WANTS, s_WANTS);
    pyrt::xrefs::reg_access("memory.change", "WHEN_TAIL", g_WHEN_TAIL, s_WHEN_TAIL);
    pyrt::xrefs::reg_access("memory.change", "WISHED", g_WISHED, s_WISHED);
    pyrt::xrefs::reg_access("memory.change", "WISHES", g_WISHES, s_WISHES);
}
