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


thread_local! { static G_A_COMPARISON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MYSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THROUGH_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOTH_MISSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEMALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HIM_OR_HER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_I_AM_OLDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOBODY_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAME_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_AGE_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_ARE_OLDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITHOUT_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G___all__: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.missing_half: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_COMPARISON() -> Py { ensure_init(); G_A_COMPARISON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COMPARISON(v: Py) { ensure_init(); G_A_COMPARISON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GAP() -> Py { ensure_init(); G_A_GAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GAP(v: Py) { ensure_init(); G_A_GAP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GROUP() -> Py { ensure_init(); G_A_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GROUP(v: Py) { ensure_init(); G_A_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MYSELF() -> Py { ensure_init(); G_A_MYSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MYSELF(v: Py) { ensure_init(); G_A_MYSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_NAMED() -> Py { ensure_init(); G_A_PERSON_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_NAMED(v: Py) { ensure_init(); G_A_PERSON_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SOMEONE() -> Py { ensure_init(); G_A_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SOMEONE(v: Py) { ensure_init(); G_A_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_NAMED() -> Py { ensure_init(); G_A_THING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED(v: Py) { ensure_init(); G_A_THING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THROUGH_SOMEONE() -> Py { ensure_init(); G_A_THROUGH_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THROUGH_SOMEONE(v: Py) { ensure_init(); G_A_THROUGH_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_BOTH_MISSING() -> Py { ensure_init(); G_BOTH_MISSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOTH_MISSING(v: Py) { ensure_init(); G_BOTH_MISSING.with(|c| *c.borrow_mut() = v); }
pub fn g_FEMALE() -> Py { ensure_init(); G_FEMALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEMALE(v: Py) { ensure_init(); G_FEMALE.with(|c| *c.borrow_mut() = v); }
pub fn g_HIM_OR_HER() -> Py { ensure_init(); G_HIM_OR_HER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HIM_OR_HER(v: Py) { ensure_init(); G_HIM_OR_HER.with(|c| *c.borrow_mut() = v); }
pub fn g_I_AM_OLDER() -> Py { ensure_init(); G_I_AM_OLDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_I_AM_OLDER(v: Py) { ensure_init(); G_I_AM_OLDER.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_MALE() -> Py { ensure_init(); G_MALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MALE(v: Py) { ensure_init(); G_MALE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOBODY_ELSE() -> Py { ensure_init(); G_NOBODY_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOBODY_ELSE(v: Py) { ensure_init(); G_NOBODY_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAME_AGE() -> Py { ensure_init(); G_SAME_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAME_AGE(v: Py) { ensure_init(); G_SAME_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_AGE_ONLY() -> Py { ensure_init(); G_THEIR_AGE_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_AGE_ONLY(v: Py) { ensure_init(); G_THEIR_AGE_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_ARE_OLDER() -> Py { ensure_init(); G_THEY_ARE_OLDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_ARE_OLDER(v: Py) { ensure_init(); G_THEY_ARE_OLDER.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_THINGS() -> Py { ensure_init(); G_TWO_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_THINGS(v: Py) { ensure_init(); G_TWO_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_WITHOUT_MINE() -> Py { ensure_init(); G_WITHOUT_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITHOUT_MINE(v: Py) { ensure_init(); G_WITHOUT_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_MINE() -> Py { ensure_init(); G_WITH_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_MINE(v: Py) { ensure_init(); G_WITH_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g___all__() -> Py { ensure_init(); G___all__.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s___all__(v: Py) { ensure_init(); G___all__.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A comparison of two ages where only one of the ages was ever given.\n\n    I'm 34 and my wife is 32.\n    Am I older than my brother?\n      \"I'm not sure about that one, sorry - I'd rather not guess. A quick search online will get\n       you a solid answer, and I can help you make sense of what you find.\"\n\nThe store is right to keep the turn free - the wife's age is no answer to a brother whose age was\nnever given - but the search hedge is not true here either. Nobody's brother's age is on the web,\nand the person is not asking the web: they are asking whether the numbers in front of them can be\nput together, and one of the two was never said. The honest half is short and worth saying: which\nages the bot has, which one it has not got, and what would settle it.\n\nIt is asked from `bot/modules/greeting.py`, last, and only in place of a hedge, so every\ncomparison the store can do with two real ages is answered by the store before this is consulted.\n\nWhat it keeps off, because a confident wrong line is worse than the hedge it replaces:\n\n- a group (\"who is the oldest in my family\", \"who is older in my class\"): three or more people,\n  and no line here fits;\n- one made thing and a person (\"am i older than my car\"): the two halves are a birth and a year of\n  manufacture, which is not a sum, and \"when the two were made\" is false about the person;\n- a line whose other half is somebody else's (\"is he older than his cousin\"): the missing age\n  belongs to the cousin, and pointing at the son would be pointing at the wrong person;\n- a pair of made things where the line names the year itself.\n");
    s_A_COMPARISON(stdlib_call("re", "compile", &[py_str("\\bwho(?:'s|s| is| was)?\\s+(?:the\\s+)?(?:older|younger|eldest|oldest|youngest)\\b|\\b(?:am|is|are|was|were)\\b[^.!?]{0,40}\\b(?:older|younger|eldest|oldest)\\b|\\bhow (?:much|many years)\\s+(?:older|younger)\\b|\\b(?:older|younger) (?:than )?(?:me|him|her|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GAP(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:much|many\\s+years)\\s+(?:older|younger)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GROUP(stdlib_call("re", "compile", &[py_str("\\b(?:family|families|class|team|group|household|siblings|relatives|relations|kids|children|brothers|sisters|friends|everyone|everybody|all of them|the rest)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("mum"), py_str("mam"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("parent"), py_str("parents"), py_str("brother"), py_str("sister"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("girlfriend"), py_str("boyfriend"), py_str("fiance"), py_str("fiancee"), py_str("gran"), py_str("granny"), py_str("nan"), py_str("nana"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("grandfather"), py_str("aunt"), py_str("auntie"), py_str("uncle"), py_str("cousin"), py_str("nephew"), py_str("niece"), py_str("stepson"), py_str("stepdaughter"), py_str("stepdad"), py_str("stepmum"), py_str("mate"), py_str("friend"), py_str("boss"), py_str("colleague"), py_str("neighbour"), py_str("landlord"), py_str("flatmate"), py_str("housemate"), py_str("boy"), py_str("girl"), py_str("baby")]) }
    s_KIN(__data1());
    s_A_PERSON_NAMED(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our)\\s+("), &mcall(&py_str("|"), "join", &[g_KIN()])), &py_str(")(?:'s|s')?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_NAMED(stdlib_call("re", "compile", &[py_str("\\bmy\\s+([a-z][a-z'-]+)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SOMEONE(stdlib_call("re", "compile", &[py_str("\\b(?:him|her|them|they|he|she)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THROUGH_SOMEONE(stdlib_call("re", "compile", &[py_str("\\b(?:his|hers|her|their|theirs|its)\\s+[a-z][a-z'-]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MYSELF(stdlib_call("re", "compile", &[py_str("\\b(?:i|i'm|im|me|my|mine|myself|we|us|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WITH_MINE(py_tuple(vec![py_str("I've got you at {mine}, but nothing on {theirs}, so there's no two numbers to put together. Tell me that one and I'll say which of you is older."), py_str("You're {mine} in what you've told me and {theirs} has no age beside it. Give me that and the comparison takes one line.")]));
    s_WITHOUT_MINE(py_tuple(vec![py_str("There's nothing beside {theirs}: I have no age of yours either. Give me both and I'll say which of you is older."), py_str("The only age here would be {theirs}, and yours is the other half. Tell me yours and I'll do the rest.")]));
    s_BOTH_MISSING(py_tuple(vec![py_str("I have {known} and no age of yours, and nothing on {theirs} either. Those two numbers and the comparison takes one line."), py_str("What I have is {known}; what is missing is your own age and {theirs}. Give me those and I'll say which of you is older.")]));
    s_NOBODY_ELSE(py_tuple(vec![py_str("Nothing beside your own age is here to put it against. Give me theirs and I'll say which of you is older."), py_str("One number is not a comparison, and yours is the only one here. Tell me theirs and the answer is one line.")]));
    s_TWO_THINGS(py_tuple(vec![py_str("Those are two made things, so what settles it is the year each one was made - on the thing itself, on its rating label, or in its manual. Give me the two years and I'll say which is older."), py_str("The age of a made thing is the year it was made, and nothing you've told me has either one. The label or the manual will have it; two years and the comparison takes one line.")]));
    s_THEIR_AGE_ONLY(py_tuple(vec![py_str("I've got {theirs} at {their_age}, but never your age, so there is one number where there should be two. Tell me yours and I'll say which of you is older."), py_str("{theirs} is {their_age} in what you've told me and your own age is not there. Give me that and the comparison takes one line.")]));
    s_HIM_OR_HER(py_tuple(vec![py_str("I've an age here and no way to tell who you mean. Say who it is and I'll put the two side by side."), py_str("Nothing in the line says who that is, so there's no second age for me to reach. Name the person and the comparison takes one line.")]));
    s_THEY_ARE_OLDER(py_tuple(vec![py_str("{Theirs} is older - {their_age} to your {mine}.")]));
    s_I_AM_OLDER(py_tuple(vec![py_str("You're older - {mine} to {theirs}'s {their_age}.")]));
    s_SAME_AGE(py_tuple(vec![py_str("Neither of you - you're both {mine}.")]));
    s_FEMALE(call_builtin("frozenset", &[mcall(&py_str("mum mam mom mother sister wife gran granny nan nana grandma daughter aunt auntie niece stepmum stepdaughter girlfriend fiancee"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MALE(call_builtin("frozenset", &[mcall(&py_str("dad father brother husband grandad grandpa grandfather son uncle nephew stepdad stepson boyfriend fiance"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s___all__(py_list(vec![py_str("the_missing_half"), py_str("A_COMPARISON")]));
}
pub fn _choose(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut one: Py = Py::None;
    let _ = py_str("The same line for the same words, so a chat is not a slot machine.");
    return geti(&pool.clone(), &op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&text.clone()) { let mut one: Py = __item.clone(); __out.push(call_builtin("ord", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_sentence(args: &[Py]) -> Py {
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
    let _ = py_str("A line that may open on a lower-case name (\"your mum is 70 ...\") said as a sentence.");
    return if py_truthy(&line.clone()) { op_add(&mcall(&geti(&line.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&line.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { line.clone() };
    Py::None
}
pub fn _fits(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut he: Py = Py::None;
    let mut she: Py = Py::None;
    let _ = py_str("Whether a pronoun in the line can only be the person the store holds under `who`.");
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:them|they)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    she = Py::Bool(!py_is(&stdlib_call("re", "search", &[py_str("\\bher\\b|\\bshe\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    he = Py::Bool(!py_is(&stdlib_call("re", "search", &[py_str("\\bhim\\b|\\bhis\\b|\\bhe\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    if py_truthy(&{ let __b = she.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&he.clone())) } }) {
    return Py::Bool(py_contains(&g_FEMALE(), &who.clone()));
    }
    if py_truthy(&{ let __b = he.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&she.clone())) } }) {
    return Py::Bool(py_contains(&g_MALE(), &who.clone()));
    }
    return Py::Bool(true);
    Py::None
}
pub fn _given_beside(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let _ = py_str("Whether the line puts an age on this person itself (\"my brother who is 40\").\n\n    A line that does has no missing half to point out, while a line that gives somebody else's age\n    (\"my wife is 32, am i older than my brother\") still leaves the brother's missing.\n    ");
    return Py::Bool(!py_is(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\bmy\\s+"), &who.clone()), &py_str("\\b[^.!?,]{0,16}?\\b\\d{1,3}\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    Py::None
}
pub fn _the_ages(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut others: Py = Py::None;
    let mut user: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The ages the store holds: the person's own, and the ones they gave for other people.");
    mine = Py::None;
    user = { let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_is(&mcall(&user.clone(), "get", &[py_str("age")]), &Py::None))) {
    mine = call_builtin("int", &[mcall(&user.clone(), "get", &[py_str("age")])]).unwrap_or_else(|e| exc::raise(e));
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
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    mine = Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    others = py_dict(vec![]);
    let __iter6 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter6 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&entry.clone(), "get", &[py_str("age")]), &Py::None)) } }) {
    continue;
    }
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    continue;
    }
    let __try7 = exc::catch(|| {
    let _ = py_setitem(&others.clone(), &who.clone(), call_builtin("int", &[mcall(&entry.clone(), "get", &[py_str("age")])]).unwrap_or_else(|e| exc::raise(e)));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled11 = true;
    continue;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    }
    return py_tuple(vec![mine.clone(), others.clone()]);
    Py::None
}
pub fn _the_known_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut others: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "others") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: others"))) };
    let mut age: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("\"your wife at 32\" out of the ages the store holds, in the order they were given. Said\n    mid-sentence, so nothing here is capitalised: a name lifted from the store is not a sentence.");
    parts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&call_builtin("list", &[mcall(&others.clone(), "items", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(2), &Py::None));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    who = __unpk2[0].clone();
    age = __unpk2[1].clone();
    let _ = mcall(&parts.clone(), "append", &[op_add(&op_add(&op_add(&py_str("your "), &who.clone()), &py_str(" at ")), &call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e)))]);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return op_add(&op_add(&geti(&parts.clone(), &Py::Int(0)), &py_str(" and ")), &geti(&parts.clone(), &Py::Int(1)));
    }
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("nothing");
    }
    return geti(&parts.clone(), &Py::Int(0));
    Py::None
}
pub fn the_missing_half(args: &[Py]) -> Py {
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
    let mut fits: Py = Py::None;
    let mut low: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut others: Py = Py::None;
    let mut their_age: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut things: Py = Py::None;
    let mut unknown: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The age comparison said plainly when one of its two ages was never given, else \"\".");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_COMPARISON(), "search", &[low.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_GAP(), "search", &[low.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_GROUP(), "search", &[low.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_THROUGH_SOMEONE(), "search", &[low.clone()])) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&_the_ages(&[memory.clone()]), 2, None);
    mine = __unpk1[0].clone();
    others = __unpk1[1].clone();
    named = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&mcall(&g_A_PERSON_NAMED(), "findall", &[low.clone()]));
    for __item in __iter2 {
    one = __item.clone();
    one = mcall(&one.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&named.clone(), &one.clone()))) {
    let _ = mcall(&named.clone(), "append", &[one.clone()]);
    }
    }
    things = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&mcall(&g_A_THING_NAMED(), "findall", &[low.clone()]));
    for __item in __iter3 {
    one = __item.clone();
    one = mcall(&one.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_KIN(), &one.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&things.clone(), &one.clone())) } }) {
    let _ = mcall(&things.clone(), "append", &[one.clone()]);
    }
    }
    if py_truthy(&named.clone()) {
    unknown = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&others.clone(), &one.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_given_beside(&[low.clone(), one.clone()]))) } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&unknown.clone()))) {
    return py_str("");
    }
    theirs = op_add(&py_str("your "), &geti(&unknown.clone(), &Py::Int(0)));
    if py_truthy(&things.clone()) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_is(&mine.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("mine".to_string(), call_builtin("str", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e))), ("theirs".to_string(), theirs.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_WITH_MINE(), low.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&others.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("known".to_string(), _the_known_list(&[others.clone()])), ("theirs".to_string(), theirs.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_BOTH_MISSING(), low.clone()]), "format", args))(&__args) };
    }
    return _a_sentence(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("theirs".to_string(), theirs.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_WITHOUT_MINE(), low.clone()]), "format", args))(&__args) }]);
    }
    if py_truthy(&things.clone()) {
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[things.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_SOMEONE(), "search", &[low.clone()]))) } }) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b\\d{4}\\b|\\b(?:19|20)\\d\\d\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    return _a_sentence(&[_choose(&[g_TWO_THINGS(), low.clone()])]);
    }
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_SOMEONE(), "search", &[low.clone()])))) {
    if py_truthy(&{ let __b = others.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mine.clone(), &Py::None)) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_is(&mine.clone(), &Py::None))) {
    return _a_sentence(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("mine".to_string(), call_builtin("str", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_NOBODY_ELSE(), low.clone()]), "format", args))(&__args) }]);
    }
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&others.clone()))) {
    return py_str("");
    }
    fits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&others.clone()) { let mut who: Py = __item.clone(); if py_truthy(&_fits(&[low.clone(), who.clone()])) {  __out.push(who.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_is(&mine.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    who = geti(&fits.clone(), &Py::Int(0));
    return _a_sentence(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("theirs".to_string(), op_add(&py_str("your "), &who.clone())), ("their_age".to_string(), call_builtin("str", &[geti(&others.clone(), &who.clone())]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_THEIR_AGE_ONLY(), low.clone()]), "format", args))(&__args) }]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&mine.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    who = geti(&fits.clone(), &Py::Int(0));
    their_age = geti(&others.clone(), &who.clone());
    theirs = op_add(&py_str("your "), &who.clone());
    if py_truthy(&Py::Bool(py_eq(&their_age.clone(), &mine.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("mine".to_string(), call_builtin("str", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_SAME_AGE(), low.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&op_gt(&their_age.clone(), &mine.clone())) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Theirs".to_string(), op_add(&mcall(&geti(&theirs.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&theirs.clone(), &Py::Int(1), &Py::None, &Py::None))), ("their_age".to_string(), call_builtin("str", &[their_age.clone()]).unwrap_or_else(|e| exc::raise(e))), ("mine".to_string(), call_builtin("str", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_THEY_ARE_OLDER(), low.clone()]), "format", args))(&__args) };
    }
    return _a_sentence(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("theirs".to_string(), theirs.clone()), ("their_age".to_string(), call_builtin("str", &[their_age.clone()]).unwrap_or_else(|e| exc::raise(e))), ("mine".to_string(), call_builtin("str", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_I_AM_OLDER(), low.clone()]), "format", args))(&__args) }]);
    }
    return _a_sentence(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("known".to_string(), _the_known_list(&[others.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choose(&[g_HIM_OR_HER(), low.clone()]), "format", args))(&__args) }]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.missing_half".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_COMPARISON" => Some(g_A_COMPARISON()),
            "A_GAP" => Some(g_A_GAP()),
            "A_GROUP" => Some(g_A_GROUP()),
            "A_MYSELF" => Some(g_A_MYSELF()),
            "A_PERSON_NAMED" => Some(g_A_PERSON_NAMED()),
            "A_SOMEONE" => Some(g_A_SOMEONE()),
            "A_THING_NAMED" => Some(g_A_THING_NAMED()),
            "A_THROUGH_SOMEONE" => Some(g_A_THROUGH_SOMEONE()),
            "BOTH_MISSING" => Some(g_BOTH_MISSING()),
            "FEMALE" => Some(g_FEMALE()),
            "HIM_OR_HER" => Some(g_HIM_OR_HER()),
            "I_AM_OLDER" => Some(g_I_AM_OLDER()),
            "KIN" => Some(g_KIN()),
            "MALE" => Some(g_MALE()),
            "NOBODY_ELSE" => Some(g_NOBODY_ELSE()),
            "SAME_AGE" => Some(g_SAME_AGE()),
            "THEIR_AGE_ONLY" => Some(g_THEIR_AGE_ONLY()),
            "THEY_ARE_OLDER" => Some(g_THEY_ARE_OLDER()),
            "TWO_THINGS" => Some(g_TWO_THINGS()),
            "WITHOUT_MINE" => Some(g_WITHOUT_MINE()),
            "WITH_MINE" => Some(g_WITH_MINE()),
            "__all__" => Some(g___all__()),
            "_a_sentence" => Some(py_func("_a_sentence", |args: &[Py]| _a_sentence(args))),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_fits" => Some(py_func("_fits", |args: &[Py]| _fits(args))),
            "_given_beside" => Some(py_func("_given_beside", |args: &[Py]| _given_beside(args))),
            "_the_ages" => Some(py_func("_the_ages", |args: &[Py]| _the_ages(args))),
            "_the_known_list" => Some(py_func("_the_known_list", |args: &[Py]| _the_known_list(args))),
            "the_missing_half" => Some(py_func("the_missing_half", |args: &[Py]| the_missing_half(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_COMPARISON" => s_A_COMPARISON(v),
            "A_GAP" => s_A_GAP(v),
            "A_GROUP" => s_A_GROUP(v),
            "A_MYSELF" => s_A_MYSELF(v),
            "A_PERSON_NAMED" => s_A_PERSON_NAMED(v),
            "A_SOMEONE" => s_A_SOMEONE(v),
            "A_THING_NAMED" => s_A_THING_NAMED(v),
            "A_THROUGH_SOMEONE" => s_A_THROUGH_SOMEONE(v),
            "BOTH_MISSING" => s_BOTH_MISSING(v),
            "FEMALE" => s_FEMALE(v),
            "HIM_OR_HER" => s_HIM_OR_HER(v),
            "I_AM_OLDER" => s_I_AM_OLDER(v),
            "KIN" => s_KIN(v),
            "MALE" => s_MALE(v),
            "NOBODY_ELSE" => s_NOBODY_ELSE(v),
            "SAME_AGE" => s_SAME_AGE(v),
            "THEIR_AGE_ONLY" => s_THEIR_AGE_ONLY(v),
            "THEY_ARE_OLDER" => s_THEY_ARE_OLDER(v),
            "TWO_THINGS" => s_TWO_THINGS(v),
            "WITHOUT_MINE" => s_WITHOUT_MINE(v),
            "WITH_MINE" => s_WITH_MINE(v),
            "__all__" => s___all__(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.missing_half", module_obj);
    pyrt::xrefs::reg_fn("compose.missing_half", "_choose", py_func("_choose", |args: &[Py]| _choose(args)));
    pyrt::xrefs::reg_fn("compose.missing_half", "_a_sentence", py_func("_a_sentence", |args: &[Py]| _a_sentence(args)));
    pyrt::xrefs::reg_fn("compose.missing_half", "_fits", py_func("_fits", |args: &[Py]| _fits(args)));
    pyrt::xrefs::reg_fn("compose.missing_half", "_given_beside", py_func("_given_beside", |args: &[Py]| _given_beside(args)));
    pyrt::xrefs::reg_fn("compose.missing_half", "_the_ages", py_func("_the_ages", |args: &[Py]| _the_ages(args)));
    pyrt::xrefs::reg_fn("compose.missing_half", "_the_known_list", py_func("_the_known_list", |args: &[Py]| _the_known_list(args)));
    pyrt::xrefs::reg_fn("compose.missing_half", "the_missing_half", py_func("the_missing_half", |args: &[Py]| the_missing_half(args)));
    pyrt::xrefs::reg_access("compose.missing_half", "A_COMPARISON", g_A_COMPARISON, s_A_COMPARISON);
    pyrt::xrefs::reg_access("compose.missing_half", "A_GAP", g_A_GAP, s_A_GAP);
    pyrt::xrefs::reg_access("compose.missing_half", "A_GROUP", g_A_GROUP, s_A_GROUP);
    pyrt::xrefs::reg_access("compose.missing_half", "A_MYSELF", g_A_MYSELF, s_A_MYSELF);
    pyrt::xrefs::reg_access("compose.missing_half", "A_PERSON_NAMED", g_A_PERSON_NAMED, s_A_PERSON_NAMED);
    pyrt::xrefs::reg_access("compose.missing_half", "A_SOMEONE", g_A_SOMEONE, s_A_SOMEONE);
    pyrt::xrefs::reg_access("compose.missing_half", "A_THING_NAMED", g_A_THING_NAMED, s_A_THING_NAMED);
    pyrt::xrefs::reg_access("compose.missing_half", "A_THROUGH_SOMEONE", g_A_THROUGH_SOMEONE, s_A_THROUGH_SOMEONE);
    pyrt::xrefs::reg_access("compose.missing_half", "BOTH_MISSING", g_BOTH_MISSING, s_BOTH_MISSING);
    pyrt::xrefs::reg_access("compose.missing_half", "FEMALE", g_FEMALE, s_FEMALE);
    pyrt::xrefs::reg_access("compose.missing_half", "HIM_OR_HER", g_HIM_OR_HER, s_HIM_OR_HER);
    pyrt::xrefs::reg_access("compose.missing_half", "I_AM_OLDER", g_I_AM_OLDER, s_I_AM_OLDER);
    pyrt::xrefs::reg_access("compose.missing_half", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("compose.missing_half", "MALE", g_MALE, s_MALE);
    pyrt::xrefs::reg_access("compose.missing_half", "NOBODY_ELSE", g_NOBODY_ELSE, s_NOBODY_ELSE);
    pyrt::xrefs::reg_access("compose.missing_half", "SAME_AGE", g_SAME_AGE, s_SAME_AGE);
    pyrt::xrefs::reg_access("compose.missing_half", "THEIR_AGE_ONLY", g_THEIR_AGE_ONLY, s_THEIR_AGE_ONLY);
    pyrt::xrefs::reg_access("compose.missing_half", "THEY_ARE_OLDER", g_THEY_ARE_OLDER, s_THEY_ARE_OLDER);
    pyrt::xrefs::reg_access("compose.missing_half", "TWO_THINGS", g_TWO_THINGS, s_TWO_THINGS);
    pyrt::xrefs::reg_access("compose.missing_half", "WITHOUT_MINE", g_WITHOUT_MINE, s_WITHOUT_MINE);
    pyrt::xrefs::reg_access("compose.missing_half", "WITH_MINE", g_WITH_MINE, s_WITH_MINE);
    pyrt::xrefs::reg_access("compose.missing_half", "__all__", g___all__, s___all__);
}
