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


thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAGGAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAGGAGE_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEANING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PLACE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_METHOD() -> Py { ensure_init(); G_A_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_METHOD(v: Py) { ensure_init(); G_A_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_BAGGAGE() -> Py { ensure_init(); G_BAGGAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAGGAGE(v: Py) { ensure_init(); G_BAGGAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_BAGGAGE_POOL() -> Py { ensure_init(); G_BAGGAGE_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAGGAGE_POOL(v: Py) { ensure_init(); G_BAGGAGE_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_LEANING() -> Py { ensure_init(); G_LEANING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEANING(v: Py) { ensure_init(); G_LEANING.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_HEADS() -> Py { ensure_init(); G_PLAIN_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_HEADS(v: Py) { ensure_init(); G_PLAIN_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT() -> Py { ensure_init(); G_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); G_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PLACE_ASK() -> Py { ensure_init(); G_THE_PLACE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PLACE_ASK(v: Py) { ensure_init(); G_THE_PLACE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN() -> Py { ensure_init(); G_UNKNOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN(v: Py) { ensure_init(); G_UNKNOWN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The place an answer is kept, read from the line above when the follow-up names no kind of its own.\n\n`fallback_data.the_place_the_answer_is_kept` turns a line into the place its answer is kept -\ntheir own account, a shop's stock, an official rule, a timetable, an airline's baggage terms -\nfrom the words on that line. It is read inside the composer only for a line that is about to get\nthe plain hedge, and it asks the line to name its own kind, because a wrong place is worse than\nno place.\n\nA follow-up does not name its kind. \"im on benefits\" then \"can i save anything\", \"one bag is 23\nkilos\" then \"and the kids, do they get one each\", \"is a backpack ok as hand luggage\" - the noun\nis one line up, where the person already said it, and the bot had just answered that line. Those\nturns came out as \"I'm not sure about that one, sorry - I'd rather not guess. A quick search\nonline will get you a solid answer\": a pointer with nothing in front of it, to a person whose own\nwords name the place.\n\nSo the kind is read from the conversation as well as from the line: a short follow-up that names\nno kind of its own takes the kind from the person's own last line, and the reply is that kind's\nown line - the limit in a few words, then where the answer lives, then what the bot can do with\nit. Only the person's own lines are read for it: the bot's answer to the turn above names half\nthe kinds in the table on its way past (\"a pharmacist will answer that one\" is a shop as well as\na medicine), and inheriting a warranty from it put \"the maker's page has the cover\" in front of\n\"will it be alright\" about a phone dropped in the sink.\n\nThis runs at the end of the hedge gate, after every reader that can answer a line of its own, so\na line with an answer of its own - a rule, a timetable, a medicine, a person to ring - never\nreaches it. It returns None when the thread names no kind either, and the composer's own hedge\nstands.\n");
    s_THE_PLACE_ASK(crate::compose::fallback_data::g_THE_PLACE_ASK());
    s_UNKNOWN(crate::compose::fallback_data::g_UNKNOWN());
    s_PLAIN_HEADS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::compose::fallback_data::g_UNKNOWN()) { let mut line: Py = __item.clone(); __out.push(slice_or_raise(&line.clone(), &Py::None, &Py::Int(40), &Py::None)); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEANING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:and|also|so|but|ok|okay|then|plus)\\b|\\W*(?:what|how|where|when|who|which|is|are|do|does|can|could|should|shall|will|would|have|has|did)\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SHORT(Py::Int(11));
    s_ASKING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\na an the and or but so then if is are was were be been being am do does did done have has had\ni you we they he she it me us them him her my your our their its this that these those there\nwhat which who whom whose where when why how whether\ncan could would should shall will may might must need needs needed\none ones each both either any some all more most much many few less least other else also too\nstill just only again ever never not no yes ok okay please thanks\nlong far often soon old big heavy quick fast early late near\nto of in on at for from with about into over under up down out off back by as than\nitn t ll re ve don doesnt didnt isnt arent wasnt wont cant couldnt shouldnt wouldnt\nget gets got give gives gave put puts take takes keep keeps save saves share shares use uses\nbuy buys pay pays book books order orders renew renews change changes try tries go goes come\ncomes help helps tell tells say says ask asks know knows work works start starts stop stops\ndo make makes fix fixes clean cleans wash washes cut cuts add adds pour pours boil boils\ncook cooks freeze freezes store stores wear wears eat eats drink drinks run runs walk walks\nwait waits ring rings call calls check checks send sends bring brings\nanything something everything nothing anyone someone anyone nobody none somewhere anywhere\nelsewhere whichever whatever whenever however lot lots bit bits piece pieces way ways\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_METHOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:how\\s+(?:do|can|could|should|would|shall)\\s+(?:i|you|we)\\b|how\\s+to\\b|(?:what|whats|what'?s)\\s+the\\s+(?:best|easiest|quickest|right)\\s+way\\b|the\\s+(?:best|easiest|quickest|right)\\s+way\\s+to\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BAGGAGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:baggage|luggage|hand luggage|cabin bag|carry[- ]on|hold bag|checked bag|holdall|suitcase|allowance|weight limit|kilos?|kilograms?|oversize|oversized|extra bag|bag drop|legroom)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BAGGAGE_POOL(py_tuple(vec![py_str("That is your airline's rather than mine, sorry - the allowance is set by the fare, so two people on the same plane can have different ones. Your booking's own page shows the kilos you have paid for, and adding a bag before the airport is always the cheaper way. Tell me what it says and I'll help you pack to it."), py_str("The airline sets that and the fare decides it, sorry, so it is your booking that answers it: open it and look for the baggage line rather than the airline's general page. Bring me the figure and I'll help you fit everything in."), py_str("I can't call that one - it belongs to the ticket rather than to the plane, sorry. Your booking shows what you have paid for and the desk weighs it to the kilo; tell me the number and I'll help you sort the rest.")]));
}
pub fn _turns(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _without_plurals(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with the s taken off every word, so a table written in the singular reads it.\n\n    The table's own words are singular where people are not - `\\bbenefit\\b` is \"on benefit\" and\n    the person says \"im on benefits\", `\\border\\b` is one order and they say \"my orders\" - and a\n    kind missed by a letter is a hedge for the whole turn. Whole words only, so \"orders\" becomes\n    \"order\" and \"passport\" stays \"passport\".\n    ");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ss"), py_str("us"), py_str("is")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) })) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "append", args))(&__args) };
    } else {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "append", args))(&__args) };
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn is_the_plain_hedge(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply is the plain pointer and not one of the hedges that already say more.");
    return Py::Bool(py_contains(&g_PLAIN_HEADS(), &slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::Int(40), &Py::None)));
    Py::None
}
pub fn _inheritable(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("Whether this kind's own line can stand on a kind borrowed from the turn above.\n\n    The officials' pool is written for a council's page (\"the council's own page, or the number on\n    the letter\"), which is the right place only where the person said council: borrowed for a\n    passport renewal it sent them to the wrong office. The rest name a place that belongs to the\n    kind itself - their own account, the operator's app, a pharmacist, a free advice service - so\n    borrowing them is safe.\n    ");
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }, &py_str("council")))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn _the_kind(args: &[Py]) -> Py {
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
    let mut borrowable: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "borrowable") { v.clone() }
    else { Py::Bool(false) };
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The kind a line names itself, as the table or this file knows it.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![line.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_plurals(args))(&__args) }]));
    for __item in __iter1 {
    said = __item.clone();
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback_data::the_place_the_answer_is_kept(args))(&__args) };
    if py_truthy(&pool.clone()) {
    if py_truthy(&{ let __b = borrowable.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _inheritable(args))(&__args) })) } }) {
    continue;
    }
    return pool.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BAGGAGE(), "search", args))(&__args) }) {
    return g_BAGGAGE_POOL();
    }
    }
    return Py::None;
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut line: Py = Py::None;
    let mut near: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place this follow-up's answer is kept, or None when the thread names no kind.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::fallback_data::the_place_the_answer_is_kept(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BAGGAGE(), "search", args))(&__args) }) {
    return g_BAGGAGE_POOL();
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEANING(), "search", args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_METHOD(), "search", args))(&__args) }) {
    return Py::None;
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z']+")); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_SHORT())) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_ASKING(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _turns(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    near = py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }]);
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEANING(), "search", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(2))), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&near.clone(), "append", args))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&near.clone());
    for __item in __iter1 {
    said = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _clean(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &line.clone())) } }) {
    continue;
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("borrowable".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_kind(args))(&__args) };
    if py_truthy(&pool.clone()) {
    return pool.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.kept_nearby".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKING" => Some(g_ASKING()),
            "A_METHOD" => Some(g_A_METHOD()),
            "BAGGAGE" => Some(g_BAGGAGE()),
            "BAGGAGE_POOL" => Some(g_BAGGAGE_POOL()),
            "LEANING" => Some(g_LEANING()),
            "PLAIN_HEADS" => Some(g_PLAIN_HEADS()),
            "SHORT" => Some(g_SHORT()),
            "THE_PLACE_ASK" => Some(g_THE_PLACE_ASK()),
            "UNKNOWN" => Some(g_UNKNOWN()),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_inheritable" => Some(py_func("_inheritable", |args: &[Py]| _inheritable(args))),
            "_the_kind" => Some(py_func("_the_kind", |args: &[Py]| _the_kind(args))),
            "_turns" => Some(py_func("_turns", |args: &[Py]| _turns(args))),
            "_without_plurals" => Some(py_func("_without_plurals", |args: &[Py]| _without_plurals(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "is_the_plain_hedge" => Some(py_func("is_the_plain_hedge", |args: &[Py]| is_the_plain_hedge(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKING" => s_ASKING(v),
            "A_METHOD" => s_A_METHOD(v),
            "BAGGAGE" => s_BAGGAGE(v),
            "BAGGAGE_POOL" => s_BAGGAGE_POOL(v),
            "LEANING" => s_LEANING(v),
            "PLAIN_HEADS" => s_PLAIN_HEADS(v),
            "SHORT" => s_SHORT(v),
            "THE_PLACE_ASK" => s_THE_PLACE_ASK(v),
            "UNKNOWN" => s_UNKNOWN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.kept_nearby", module_obj);
    pyrt::xrefs::reg_fn("compose.kept_nearby", "_turns", py_func("_turns", |args: &[Py]| _turns(args)));
    pyrt::xrefs::reg_fn("compose.kept_nearby", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("compose.kept_nearby", "_without_plurals", py_func("_without_plurals", |args: &[Py]| _without_plurals(args)));
    pyrt::xrefs::reg_fn("compose.kept_nearby", "is_the_plain_hedge", py_func("is_the_plain_hedge", |args: &[Py]| is_the_plain_hedge(args)));
    pyrt::xrefs::reg_fn("compose.kept_nearby", "_inheritable", py_func("_inheritable", |args: &[Py]| _inheritable(args)));
    pyrt::xrefs::reg_fn("compose.kept_nearby", "_the_kind", py_func("_the_kind", |args: &[Py]| _the_kind(args)));
    pyrt::xrefs::reg_fn("compose.kept_nearby", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.kept_nearby", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("compose.kept_nearby", "A_METHOD", g_A_METHOD, s_A_METHOD);
    pyrt::xrefs::reg_access("compose.kept_nearby", "BAGGAGE", g_BAGGAGE, s_BAGGAGE);
    pyrt::xrefs::reg_access("compose.kept_nearby", "BAGGAGE_POOL", g_BAGGAGE_POOL, s_BAGGAGE_POOL);
    pyrt::xrefs::reg_access("compose.kept_nearby", "LEANING", g_LEANING, s_LEANING);
    pyrt::xrefs::reg_access("compose.kept_nearby", "PLAIN_HEADS", g_PLAIN_HEADS, s_PLAIN_HEADS);
    pyrt::xrefs::reg_access("compose.kept_nearby", "SHORT", g_SHORT, s_SHORT);
    pyrt::xrefs::reg_access("compose.kept_nearby", "THE_PLACE_ASK", g_THE_PLACE_ASK, s_THE_PLACE_ASK);
    pyrt::xrefs::reg_access("compose.kept_nearby", "UNKNOWN", g_UNKNOWN, s_UNKNOWN);
}
