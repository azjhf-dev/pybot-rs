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


thread_local! { static G_ABOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BELOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMPTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_CASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_LETTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUNS_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOVE() -> Py { ensure_init(); G_ABOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOVE(v: Py) { ensure_init(); G_ABOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_IN_IT() -> Py { ensure_init(); G_A_PERSON_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_IN_IT(v: Py) { ensure_init(); G_A_PERSON_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN() -> Py { ensure_init(); G_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN(v: Py) { ensure_init(); G_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_BELOW() -> Py { ensure_init(); G_BELOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BELOW(v: Py) { ensure_init(); G_BELOW.with(|c| *c.borrow_mut() = v); }
pub fn g_BREAK() -> Py { ensure_init(); G_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREAK(v: Py) { ensure_init(); G_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_WORD() -> Py { ensure_init(); G_CONDITION_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_WORD(v: Py) { ensure_init(); G_CONDITION_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_EMPTY() -> Py { ensure_init(); G_EMPTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMPTY(v: Py) { ensure_init(); G_EMPTY.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_CASE() -> Py { ensure_init(); G_MOST_CASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_CASE(v: Py) { ensure_init(); G_MOST_CASE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_LETTERS() -> Py { ensure_init(); G_MOST_LETTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_LETTERS(v: Py) { ensure_init(); G_MOST_LETTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS_BEFORE() -> Py { ensure_init(); G_MOST_WORDS_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS_BEFORE(v: Py) { ensure_init(); G_MOST_WORDS_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_PERSON() -> Py { ensure_init(); G_MY_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_PERSON(v: Py) { ensure_init(); G_MY_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBERS() -> Py { ensure_init(); G_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBERS(v: Py) { ensure_init(); G_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENS() -> Py { ensure_init(); G_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENS(v: Py) { ensure_init(); G_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON() -> Py { ensure_init(); G_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); G_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE() -> Py { ensure_init(); G_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE(v: Py) { ensure_init(); G_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_RUNS_ON() -> Py { ensure_init(); G_RUNS_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUNS_ON(v: Py) { ensure_init(); G_RUNS_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNITS() -> Py { ensure_init(); G_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNITS(v: Py) { ensure_init(); G_UNITS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The branch of the last answer that a short line of the person's own words chooses.\n\nAn answer often offers a side and names the case it is for: \"Dogs, by a whisker ... A cat is the\nkinder choice if you're out all day\". When the person then says \"i work all day\", the answer has\nalready said what that means, and repeating their line back to them (\"Got it - you work all day\")\nthrows the answer away. This reads the conditions out of the answer we just gave and hands back\nthe consequence of the one their line matches, so the reply can be the choice, not an echo.\n\nThe match is on a pair of words, not a topic: \"all day\" is in both lines, where \"i had a long day\"\nshares only \"day\" with \"out all day\" and is a different thing to say.\n\nA case is about the person who asked. \"my neighbour works all day\" shares \"all day\" with \"if\nyou're out all day\" and says nothing about the flat the cat would live in, so it takes no case:\nthe branch is theirs only when the line is about them.\n");
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?:(?:\\d\\.\\d)|[^.!?])+[.!?]?")]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENS(stdlib_call("re", "compile", &[py_str("^(?:if|when|unless|in case)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITION_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:if|when|unless|in case)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BREAK(stdlib_call("re", "compile", &[py_str("\\s*(?:,|;|:|\\u2014|\\s-\\s|\\band then\\b|\\bthen\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EMPTY(call_builtin("frozenset", &[mcall(&py_str("a an the i im you your youre we us our it its that this there\nhere to of in on at for with and or but if then so as is are was were be been am do does did\nhave has had got get all just really very too not no now some any my me him her his hers them\nthey their what which who whom whose how why when where"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_LETTERS(Py::Int(90));
    s_MOST_CASE(Py::Int(60));
    s_RUNS_ON(stdlib_call("re", "compile", &[py_str("\\b(?:get|ring|call|take|keep|see|use|put|leave|buy|stop|go|try|check|ask|tell|give|make|book|contact|wash|clean|apply|move|add|rest|ice|eat|drink)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE(call_builtin("frozenset", &[mcall(&py_str("out outside away outdoors home house indoors inside office in"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON(op_bitor(&call_builtin("frozenset", &[pyrt::xrefs::xget("memory.facts", "RELATIONS")]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[mcall(&py_str("neighbour neighbor friend boss manager\ncolleague coworker roommate flatmate housemate landlord partner kids kid child children baby\ntoddler twins sister brother son daughter"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_MY_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_IN_IT(op_bitor(&call_builtin("frozenset", &[g_PERSON()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[mcall(&py_str("someone anyone somebody anybody child children\n    baby toddler guest visitor family relatives"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_A_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)(?:m|re| am| are)?\\s+(?:want|wanna|need|plan|intend|have to|must|should|will|would like|am going|are going|going to|think i|ll|will)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_UNITS(call_builtin("frozenset", &[mcall(&py_str("year month week day hour minute mile km kilo stone pound percent"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBERS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[mcall(&py_str("zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty thirty forty fifty sixty seventy eighty ninety hundred"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5957 = py_unpack(&__item.clone(), 2, None); let mut value: Py = __unpk5957[0].clone(); let mut word: Py = __unpk5957[1].clone(); __out.push((word.clone(), value.clone())); } py_dict(__out) })() });
    s_BELOW(py_tuple(vec![py_str("under"), py_str("below"), py_str("less than"), py_str("fewer than"), py_str("up to"), py_str("at most"), py_str("no more than"), py_str("within"), py_str("younger than"), py_str("shorter than")]));
    s_ABOVE(py_tuple(vec![py_str("over"), py_str("above"), py_str("more than"), py_str("past"), py_str("older than"), py_str("longer than"), py_str("beyond"), py_str("at least"), py_str("from"), py_str("at")]));
    s_MOST_WORDS_BEFORE(Py::Int(3));
}
pub fn _places(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_PLACE(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    let _ = py_str("The words of a line, with the apostrophe taken out so \"you're\" meets \"youre\".");
    return stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _pairs(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut first: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("Every two words that sit next to each other and are not both empty ones.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[words.clone(), slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))) { let __unpk2274 = py_unpack(&__item.clone(), 2, None); let mut first: Py = __unpk2274[0].clone(); let mut second: Py = __unpk2274[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_EMPTY(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_EMPTY(), &second.clone())) } }) {  __out.push(op_add(&op_add(&first.clone(), &py_str(" ")), &second.clone())); } } py_set(__out) })() };
    Py::None
}
pub fn about_someone_else(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line is about one of the people in it rather than the person typing.\n\n    A family word after \"my\" is not proof on its own - \"my wife and i are out all day\" is the\n    household and does take the case - but a short line matched on a pair of words is worth less\n    than the echo it replaces when the pair belongs to somebody else, so the reading refuses.\n    The pronoun is the other half: \"she works all day\" is her day.\n    ");
    low = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_MY_PERSON(), "findall", &[low.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PERSON(), &mcall(&word.clone(), "rstrip", &[py_str("'s")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _sentences(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "findall", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    Py::None
}
pub fn _one_move(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut consequence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "consequence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: consequence"))) };
    let mut kept: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut text: Py = Py::None;
    let mut trial: Py = Py::None;
    let _ = py_str("The first move of a consequence: a long one is cut at a clause boundary, not dropped.\n\n    \"pay a local pest controller, usually 50 to 80 pounds, rather than going up a ladder with a\n    spray - and do not block the entrance, because they will find another way into the house\" is\n    one consequence of a case and three sentences' worth of text: what the case is worth saying is\n    its first move, and dropping it for the length of its tail lost the case altogether.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = consequence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" ,;:-—.")]);
    kept = text.clone();
    if py_truthy(&op_gt(&call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_LETTERS())) {
    kept = py_str("");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",\\s+|\\s+-\\s+|\\s+\\u2014\\s+"), text.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    piece = __item.clone();
    trial = if py_truthy(&kept.clone()) { op_add(&op_add(&kept.clone(), &py_str(", ")), &piece.clone()) } else { piece.clone() };
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[trial.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_LETTERS()); if !py_truthy(&__b) { __b } else { kept.clone() } }) {
    break;
    }
    kept = trial.clone();
    }
    }
    kept = mcall(&kept.clone(), "strip", &[py_str(" ,;:-—")]);
    kept = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:[\\s,;:]+)(?:and|but|or|so|then|with|plus)$")); __args.push(py_str("")); __args.push(kept.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return mcall(&kept.clone(), "strip", &[py_str(" ,;:-—")]);
    Py::None
}
pub fn conditions(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut case: Py = Py::None;
    let mut consequence: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut found: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut opening_at: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("[(the case, what the answer says happens in that case)] for every \"if\" in the answer.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&_sentences(&[said.clone()]));
    for __item in __iter1 {
    sentence = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_CONDITION_WORD(), "finditer", &[sentence.clone()]));
    for __item in __iter2 {
    opening = __item.clone();
    before = mcall(&slice_or_raise(&sentence.clone(), &Py::None, &mcall(&opening.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,;:-—")]);
    if py_truthy(&mcall(&g_CONDITION_WORD(), "search", &[before.clone()])) {
    continue;
    }
    rest = mcall(&slice_or_raise(&sentence.clone(), &mcall(&opening.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    rest = mcall(&mcall(&g_OPENS(), "sub", &[py_str(""), rest.clone()]), "strip", &[]);
    cut = mcall(&g_BREAK(), "search", &[rest.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&cut.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = before.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_RUNS_ON(), "search", &[rest.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&cut.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&before.clone())) } }) {
    opening_at = stdlib_call("re", "search", &[py_str("\\s+(?=(?:a|an|the)\\s)"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&opening_at.clone()) {
    cut = opening_at.clone();
    }
    }
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("[:;]"), before.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    case = mcall(&if py_truthy(&cut.clone()) { slice_or_raise(&rest.clone(), &Py::None, &mcall(&cut.clone(), "start", &[]), &Py::None) } else { rest.clone() }, "strip", &[py_str(" ,;:-—.")]);
    after = if py_truthy(&cut.clone()) { slice_or_raise(&rest.clone(), &mcall(&cut.clone(), "start", &[]), &Py::None, &Py::None) } else { py_str("") };
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:,|;|:|\\u2014|\\s-\\s|\\band\\b|\\bthen\\b)\\s*")); __args.push(py_str("")); __args.push(after.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    consequence = if py_truthy(&op_ge(&call_builtin("len", &[_words(&[before.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { before.clone() } else { mcall(&after.clone(), "strip", &[py_str(" ,;:-—.")]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&case.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&consequence.clone())) } }) {
    continue;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[case.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_CASE())) {
    continue;
    }
    consequence = _one_move(&[consequence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&consequence.clone()))) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![case.clone(), consequence.clone()])]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _number(args: &[Py]) -> Py {
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
    word = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,")]);
    if py_truthy(&mcall(&word.clone(), "isdigit", &[])) {
    return call_builtin("float", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return if py_truthy(&Py::Bool(py_contains(&g_NUMBERS(), &word.clone()))) { call_builtin("float", &[geti(&g_NUMBERS(), &word.clone())]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None };
    Py::None
}
pub fn _measures(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("{unit: number} for every \"<number> <unit>\" in a line, digits or words alike.");
    found = py_dict(vec![]);
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9.]+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    unit = mcall(&word.clone(), "rstrip", &[py_str("s")]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_UNITS(), &unit.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&index.clone())) } }) {
    continue;
    }
    value = _number(&[geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1)))]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&value.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&index.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } } } }) {
    value = Py::Float(1.0);
    }
    if py_truthy(&Py::Bool(!py_is(&value.clone(), &Py::None))) {
    let _ = mcall(&found.clone(), "setdefault", &[unit.clone(), value.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _tests(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut case: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "case") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: case"))) };
    let mut around: Py = Py::None;
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut side: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("[(unit, number, \"below\"|\"above\")] for every number a case tests.");
    found = py_list(vec![]);
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9.]+"), mcall(&call_builtin("str", &[{ let __b = case.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    unit = mcall(&word.clone(), "rstrip", &[py_str("s")]);
    value = if py_truthy(&index.clone()) { _number(&[geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1)))]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_UNITS(), &unit.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&value.clone(), &Py::None)) } }) {
    continue;
    }
    around = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&op_sub(&index.clone(), &Py::Int(1)), &g_MOST_WORDS_BEFORE())]).unwrap_or_else(|e| exc::raise(e)), &op_sub(&index.clone(), &Py::Int(1)), &Py::None)]);
    side = if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BELOW()) { let mut lead: Py = __item.clone(); __out.push(Py::Bool(py_contains(&around.clone(), &lead.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { py_str("below") } else { if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ABOVE()) { let mut lead: Py = __item.clone(); __out.push(Py::Bool(py_contains(&around.clone(), &lead.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { py_str("above") } else { py_str("") } };
    if py_truthy(&side.clone()) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![unit.clone(), value.clone(), side.clone()])]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _meets(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut case: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "case") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: case"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut judged: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut side: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Whether the numbers the line gives pass the numbers the case tests.\n\n    True when every test is passed, False when one is failed, and None when the case tests\n    nothing this line gives a number for - which leaves the case to the word match alone. The\n    line itself is read, not its words alone: a number is not a word.\n    ");
    mine = _measures(&[text.clone()]);
    judged = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&_tests(&[case.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    unit = __unpk2[0].clone();
    value = __unpk2[1].clone();
    side = __unpk2[2].clone();
    if py_truthy(&Py::Bool(!py_contains(&mine.clone(), &unit.clone()))) {
    continue;
    }
    judged = Py::Bool(true);
    theirs = geti(&mine.clone(), &unit.clone());
    if py_truthy(&{ let __b = Py::Bool(py_eq(&side.clone(), &py_str("below"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_lt(&theirs.clone(), &value.clone()))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&side.clone(), &py_str("above"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_gt(&theirs.clone(), &value.clone()))) } }) {
    return Py::Bool(false);
    }
    }
    return if py_truthy(&judged.clone()) { Py::Bool(true) } else { Py::None };
    Py::None
}
pub fn chosen(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut case: Py = Py::None;
    let mut consequence: Py = Py::None;
    let _ = py_str("The consequence the last answer gave for the case this line states, or None.\n\n    Only one reading is allowed: the line has to share a pair of words with the case, so \"i work\n    all day\" takes the \"out all day\" branch while \"i had a long day\" takes nothing. A case that\n    tests a number also has to be passed by their number, because a shared unit says nothing\n    about which side of the test they are on.\n    ");
    let __iter1 = py_iter_or_raise(&_candidates(&[text.clone(), said.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    case = __unpk2[0].clone();
    consequence = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_is(&_meets(&[case.clone(), text.clone()]), &Py::Bool(false)))) {
    continue;
    }
    return consequence.clone();
    }
    return Py::None;
    Py::None
}
pub fn refused(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut _u: Py = Py::None;
    let mut case: Py = Py::None;
    let _ = py_str("Whether the last answer put a test on the number this line gives, and their number fails.\n\n    The other half of the same reading: \"repair it if it is under five years old\" is answered\n    \"its 8 years old\", and the answer's verdict does not apply to them. The caller says the fact\n    back and stops, which is the honest reply - the answer itself already said what to do about\n    the two cases it named, and neither of them is this one.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_candidates(&[text.clone(), said.clone()])) { let __unpk3548 = py_unpack(&__item.clone(), 2, None); let mut case: Py = __unpk3548[0].clone(); let mut _u: Py = __unpk3548[1].clone(); __out.push(Py::Bool(py_is(&_meets(&[case.clone(), text.clone()]), &Py::Bool(false)))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _carried(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut case_words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "case_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: case_words"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut met: Py = Py::None;
    let mut w: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("Whether the line carries what the case turns on, when no pair of words can be shared.\n\n    A case is often one word that matters - \"if it is by a door\", \"if someone in the house is\n    allergic\" - and a line that says it in its own words (\"its right by the back door\", \"my son\n    is allergic\") shares no pair with it. The words that carry no subject are dropped, and what is\n    left of the case has to be met: all of a short case, and most of a long one, whose tail has\n    picked up the rest of its sentence.\n    ");
    want = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&case_words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_EMPTY(), &w.clone()))) {  __out.push(w.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&want.clone()))) {
    return Py::Bool(false);
    }
    met = op_bitand(&want.clone(), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&Py::Bool(!py_truthy(&met.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_eq(&call_builtin("len", &[met.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[want.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[want.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { op_ge(&op_mul(&call_builtin("len", &[met.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[want.clone()]).unwrap_or_else(|e| exc::raise(e))) } } } };
    Py::None
}
pub fn _about_a_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut case_words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "case_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: case_words"))) };
    let mut w: Py = Py::None;
    let _ = py_str("Whether the case itself names somebody, so a line about theirs can be that case.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&case_words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_PERSON_IN_IT(), &mcall(&w.clone(), "rstrip", &[py_str("'s")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _candidates(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut case: Py = Py::None;
    let mut case_words: Py = Py::None;
    let mut consequence: Py = Py::None;
    let mut found: Py = Py::None;
    let mut half: Py = Py::None;
    let mut halves: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut one: Py = Py::None;
    let mut ours: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The (case, consequence) pairs of the answer this line can be matched to in words.");
    if py_truthy(&mcall(&g_A_PLAN(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return py_list(vec![]);
    }
    words = _words(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return py_list(vec![]);
    }
    mine = _pairs(&[words.clone()]);
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&conditions(&[said.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    case = __unpk2[0].clone();
    consequence = __unpk2[1].clone();
    case_words = _words(&[case.clone()]);
    halves = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bor\\b|\\bwhen\\b")); __args.push(case.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[])) {  __out.push(_words(&[one.clone()])); } } py_list(__out) })() };
    hit = Py::Bool(false);
    let __iter3 = py_iter_or_raise(&halves.clone());
    for __item in __iter3 {
    half = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_bitand(&mine.clone(), &_pairs(&[half.clone()])); if py_truthy(&__b) { __b } else { _carried(&[half.clone(), words.clone()]) } }))) {
    continue;
    }
    let __unpk4 = py_unpack(&py_tuple(vec![_places(&[half.clone()]), _places(&[words.clone()])]), 2, None);
    theirs = __unpk4[0].clone();
    ours = __unpk4[1].clone();
    if py_truthy(&{ let __b = theirs.clone(); if !py_truthy(&__b) { __b } else { { let __b = ours.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&theirs.clone(), &ours.clone()))) } } } }) {
    continue;
    }
    hit = Py::Bool(true);
    break;
    }
    if py_truthy(&Py::Bool(!py_truthy(&hit.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = about_someone_else(&[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_a_person(&[case_words.clone()]))) } }) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![case.clone(), consequence.clone()])]);
    }
    return found.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.condition".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOVE" => Some(g_ABOVE()),
            "A_PERSON_IN_IT" => Some(g_A_PERSON_IN_IT()),
            "A_PLAN" => Some(g_A_PLAN()),
            "BELOW" => Some(g_BELOW()),
            "BREAK" => Some(g_BREAK()),
            "CONDITION_WORD" => Some(g_CONDITION_WORD()),
            "EMPTY" => Some(g_EMPTY()),
            "MOST_CASE" => Some(g_MOST_CASE()),
            "MOST_LETTERS" => Some(g_MOST_LETTERS()),
            "MOST_WORDS_BEFORE" => Some(g_MOST_WORDS_BEFORE()),
            "MY_PERSON" => Some(g_MY_PERSON()),
            "NUMBERS" => Some(g_NUMBERS()),
            "OPENS" => Some(g_OPENS()),
            "PERSON" => Some(g_PERSON()),
            "PLACE" => Some(g_PLACE()),
            "RUNS_ON" => Some(g_RUNS_ON()),
            "SENTENCE" => Some(g_SENTENCE()),
            "UNITS" => Some(g_UNITS()),
            "_about_a_person" => Some(py_func("_about_a_person", |args: &[Py]| _about_a_person(args))),
            "_candidates" => Some(py_func("_candidates", |args: &[Py]| _candidates(args))),
            "_carried" => Some(py_func("_carried", |args: &[Py]| _carried(args))),
            "_measures" => Some(py_func("_measures", |args: &[Py]| _measures(args))),
            "_meets" => Some(py_func("_meets", |args: &[Py]| _meets(args))),
            "_number" => Some(py_func("_number", |args: &[Py]| _number(args))),
            "_one_move" => Some(py_func("_one_move", |args: &[Py]| _one_move(args))),
            "_pairs" => Some(py_func("_pairs", |args: &[Py]| _pairs(args))),
            "_places" => Some(py_func("_places", |args: &[Py]| _places(args))),
            "_sentences" => Some(py_func("_sentences", |args: &[Py]| _sentences(args))),
            "_tests" => Some(py_func("_tests", |args: &[Py]| _tests(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "about_someone_else" => Some(py_func("about_someone_else", |args: &[Py]| about_someone_else(args))),
            "chosen" => Some(py_func("chosen", |args: &[Py]| chosen(args))),
            "conditions" => Some(py_func("conditions", |args: &[Py]| conditions(args))),
            "refused" => Some(py_func("refused", |args: &[Py]| refused(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOVE" => s_ABOVE(v),
            "A_PERSON_IN_IT" => s_A_PERSON_IN_IT(v),
            "A_PLAN" => s_A_PLAN(v),
            "BELOW" => s_BELOW(v),
            "BREAK" => s_BREAK(v),
            "CONDITION_WORD" => s_CONDITION_WORD(v),
            "EMPTY" => s_EMPTY(v),
            "MOST_CASE" => s_MOST_CASE(v),
            "MOST_LETTERS" => s_MOST_LETTERS(v),
            "MOST_WORDS_BEFORE" => s_MOST_WORDS_BEFORE(v),
            "MY_PERSON" => s_MY_PERSON(v),
            "NUMBERS" => s_NUMBERS(v),
            "OPENS" => s_OPENS(v),
            "PERSON" => s_PERSON(v),
            "PLACE" => s_PLACE(v),
            "RUNS_ON" => s_RUNS_ON(v),
            "SENTENCE" => s_SENTENCE(v),
            "UNITS" => s_UNITS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.condition", module_obj);
    pyrt::xrefs::reg_fn("memory.condition", "_places", py_func("_places", |args: &[Py]| _places(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_pairs", py_func("_pairs", |args: &[Py]| _pairs(args)));
    pyrt::xrefs::reg_fn("memory.condition", "about_someone_else", py_func("about_someone_else", |args: &[Py]| about_someone_else(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_sentences", py_func("_sentences", |args: &[Py]| _sentences(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_one_move", py_func("_one_move", |args: &[Py]| _one_move(args)));
    pyrt::xrefs::reg_fn("memory.condition", "conditions", py_func("conditions", |args: &[Py]| conditions(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_number", py_func("_number", |args: &[Py]| _number(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_measures", py_func("_measures", |args: &[Py]| _measures(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_tests", py_func("_tests", |args: &[Py]| _tests(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_meets", py_func("_meets", |args: &[Py]| _meets(args)));
    pyrt::xrefs::reg_fn("memory.condition", "chosen", py_func("chosen", |args: &[Py]| chosen(args)));
    pyrt::xrefs::reg_fn("memory.condition", "refused", py_func("refused", |args: &[Py]| refused(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_carried", py_func("_carried", |args: &[Py]| _carried(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_about_a_person", py_func("_about_a_person", |args: &[Py]| _about_a_person(args)));
    pyrt::xrefs::reg_fn("memory.condition", "_candidates", py_func("_candidates", |args: &[Py]| _candidates(args)));
    pyrt::xrefs::reg_access("memory.condition", "ABOVE", g_ABOVE, s_ABOVE);
    pyrt::xrefs::reg_access("memory.condition", "A_PERSON_IN_IT", g_A_PERSON_IN_IT, s_A_PERSON_IN_IT);
    pyrt::xrefs::reg_access("memory.condition", "A_PLAN", g_A_PLAN, s_A_PLAN);
    pyrt::xrefs::reg_access("memory.condition", "BELOW", g_BELOW, s_BELOW);
    pyrt::xrefs::reg_access("memory.condition", "BREAK", g_BREAK, s_BREAK);
    pyrt::xrefs::reg_access("memory.condition", "CONDITION_WORD", g_CONDITION_WORD, s_CONDITION_WORD);
    pyrt::xrefs::reg_access("memory.condition", "EMPTY", g_EMPTY, s_EMPTY);
    pyrt::xrefs::reg_access("memory.condition", "MOST_CASE", g_MOST_CASE, s_MOST_CASE);
    pyrt::xrefs::reg_access("memory.condition", "MOST_LETTERS", g_MOST_LETTERS, s_MOST_LETTERS);
    pyrt::xrefs::reg_access("memory.condition", "MOST_WORDS_BEFORE", g_MOST_WORDS_BEFORE, s_MOST_WORDS_BEFORE);
    pyrt::xrefs::reg_access("memory.condition", "MY_PERSON", g_MY_PERSON, s_MY_PERSON);
    pyrt::xrefs::reg_access("memory.condition", "NUMBERS", g_NUMBERS, s_NUMBERS);
    pyrt::xrefs::reg_access("memory.condition", "OPENS", g_OPENS, s_OPENS);
    pyrt::xrefs::reg_access("memory.condition", "PERSON", g_PERSON, s_PERSON);
    pyrt::xrefs::reg_access("memory.condition", "PLACE", g_PLACE, s_PLACE);
    pyrt::xrefs::reg_access("memory.condition", "RUNS_ON", g_RUNS_ON, s_RUNS_ON);
    pyrt::xrefs::reg_access("memory.condition", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("memory.condition", "UNITS", g_UNITS, s_UNITS);
}
