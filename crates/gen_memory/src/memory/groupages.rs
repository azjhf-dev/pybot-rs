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


thread_local! { static G_ASKS_THE_END_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_THE_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVES_A_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVES_A_LIST_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEYRE_A_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LIST_OF_AGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROUPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_SAME_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_SINGULARS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKS_THE_END_ONE() -> Py { ensure_init(); G_ASKS_THE_END_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_END_ONE(v: Py) { ensure_init(); G_ASKS_THE_END_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_THE_LIST() -> Py { ensure_init(); G_ASKS_THE_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_THE_LIST(v: Py) { ensure_init(); G_ASKS_THE_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVES_A_LIST() -> Py { ensure_init(); G_GIVES_A_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVES_A_LIST(v: Py) { ensure_init(); G_GIVES_A_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVES_A_LIST_BARE() -> Py { ensure_init(); G_GIVES_A_LIST_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVES_A_LIST_BARE(v: Py) { ensure_init(); G_GIVES_A_LIST_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_WORDS() -> Py { ensure_init(); G_GROUP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_WORDS(v: Py) { ensure_init(); G_GROUP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THEYRE_A_LIST() -> Py { ensure_init(); G_THEYRE_A_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEYRE_A_LIST(v: Py) { ensure_init(); G_THEYRE_A_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE() -> Py { ensure_init(); G__AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE(v: Py) { ensure_init(); G__AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LIST_OF_AGES() -> Py { ensure_init(); G__A_LIST_OF_AGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LIST_OF_AGES(v: Py) { ensure_init(); G__A_LIST_OF_AGES.with(|c| *c.borrow_mut() = v); }
pub fn g__GROUPS() -> Py { ensure_init(); G__GROUPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROUPS(v: Py) { ensure_init(); G__GROUPS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_UNIT() -> Py { ensure_init(); G__NOT_A_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_UNIT(v: Py) { ensure_init(); G__NOT_A_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_SAME_GROUP() -> Py { ensure_init(); G__THE_SAME_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_SAME_GROUP(v: Py) { ensure_init(); G__THE_SAME_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_SINGULARS() -> Py { ensure_init(); G__THE_SINGULARS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_SINGULARS(v: Py) { ensure_init(); G__THE_SINGULARS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The ages of a group of people, kept as the list the person said.\n\n    my parents are 66 and 70\n    how old are they            ->  Your parents are 66 and 70.\n\n    my two kids theyre 4 and 7\n    how old are they again      ->  Your kids are 4 and 7.\n    how old is my youngest      ->  Your youngest is 4.\n\nThe store keeps one age for the person in the chat (`memory[\"their_age\"]`), which is right for\n\"how old is she\" and wrong the moment a line gives two: `my two kids theyre 4 and 7` kept the 4\nand dropped the 7, `how old are they again` answered \"They are 4.\", and `how old are my kids`\nanswered \"You haven't told me how old they are.\" - a list of two is not one number, and it is not\nnothing either. Every plural group fails the same way, not just children, so the reading here is\nthe group word and the list beside it rather than another branch about kids.\n\nThe list is kept under `memory[\"group_ages\"]`:\n\n    {\"word\": \"kids\", \"ages\": [4, 7], \"line\": \"my two kids theyre 4 and 7\"}\n\nThe last line that gives a list for a word wins, so a correction (\"my kids are 5 and 8\" after\n\"my kids are 4 and 7\") is the list the answers are written from. What the reply may claim is only\nwhat was said: the plural ask is answered from the list, the singular ask about the store's one\nperson is left alone, and a chat with no list in it gets nothing at all from here.\n");
    s_GROUP_WORDS(py_tuple(vec![py_str("kids"), py_str("children"), py_str("sons"), py_str("daughters"), py_str("boys"), py_str("girls"), py_str("parents"), py_str("grandparents"), py_str("grandchildren"), py_str("grandkids"), py_str("brothers"), py_str("sisters"), py_str("twins"), py_str("cousins"), py_str("nieces"), py_str("nephews"), py_str("aunts"), py_str("uncles"), py_str("stepchildren"), py_str("stepkids"), py_str("in-laws"), py_str("flatmates"), py_str("housemates"), py_str("neighbours")]));
    s__THE_SAME_GROUP(py_dict(vec![(py_str("children"), py_str("children")), (py_str("child"), py_str("children")), (py_str("kid"), py_str("kids")), (py_str("kids"), py_str("kids")), (py_str("grandchild"), py_str("grandchildren")), (py_str("grandkids"), py_str("grandchildren")), (py_str("stepchild"), py_str("stepchildren")), (py_str("stepkid"), py_str("stepkids"))]));
    s__GROUPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_GROUP_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) });
    s__AGE(py_str("\\d{1,3}(?![\\d.])"));
    s__A_LIST_OF_AGES(op_add(&op_add(&op_add(&g__AGE(), &py_str("(?:\\s*(?:,|and|&)\\s*")), &g__AGE()), &py_str("){1,3}")));
    s__NOT_A_UNIT(py_str("(?!\\s*(?:times|days|weeks|months|hours|minutes|miles|metres|kilometres|km|kilos|kilograms|kg|pounds|quid|per\\b|a\\s+(?:week|month|year|day))|\\s*years(?!\\s+old))"));
    s_GIVES_A_LIST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:my|our|the|his|her)\\s+(?:\\w+\\s+){0,2}?(?P<word>"), &g__GROUPS()), &py_str(")\\b(?:\\s+(?:are|were|is|was|aged|both))?[,\\s]+(?P<ages>")), &g__A_LIST_OF_AGES()), &py_str(")(?![\\d,.])")), &g__NOT_A_UNIT())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIVES_A_LIST_BARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?P<word>"), &g__GROUPS()), &py_str(")\\s+(?:are|were|aged|both\\s+aged)\\s+(?P<ages>")), &g__A_LIST_OF_AGES()), &py_str(")(?![\\d,.])")), &g__NOT_A_UNIT())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEYRE_A_LIST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:my|our|the)?\\s*(?:\\w+\\s+){0,2}?(?P<word>"), &g__GROUPS()), &py_str(")\\b[^.!?]{0,20}?\\b(?:they'?re|they\\s+are|both\\s+aged)\\s+(?P<ages>")), &g__A_LIST_OF_AGES()), &py_str(")")), &g__NOT_A_UNIT())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_THE_LIST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\bhow\\s+old\\s+(?:are|were|is)\\s+(?:they|their|(?:\\w+\\s+){0,2}?(?:"), &g__GROUPS()), &py_str("))\\b|\\b(?:what|whats|what\\s+are|what\\s+were)\\s+(?:are\\s+|were\\s+)?(?:their|the|my|our)\\s+(?:\\w+\\s+){0,2}?(?:")), &g__GROUPS()), &py_str("|ages)\\b|\\bhow\\s+old\\s+are\\s+(?:my|our|the)\\s+(?:\\w+\\s+){0,2}?(?:")), &g__GROUPS()), &py_str(")\\b|\\b(?:their|the\\s+kids'?|my\\s+kids'?)\\s+ages\\b|\\bwhat\\s+(?:were|are)\\s+my\\s+(?:\\w+\\s+){0,2}?(?:")), &g__GROUPS()), &py_str(")\\s+ages\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_THE_END_ONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+old\\s+(?:is|was)\\s+(?:your\\s+|the\\s+|my\\s+|our\\s+)?(?P<which>youngest|oldest|elder|eldest|older|younger|little|big|little\\s+one|big\\s+one)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THE_SINGULARS(py_dict(vec![(py_str("children"), py_tuple(vec![py_str("child")])), (py_str("kids"), py_tuple(vec![py_str("kid"), py_str("child")])), (py_str("parents"), py_tuple(vec![py_str("parent")])), (py_str("people"), py_tuple(vec![py_str("person")])), (py_str("men"), py_tuple(vec![py_str("man")])), (py_str("women"), py_tuple(vec![py_str("woman")])), (py_str("twins"), py_tuple(vec![py_str("twin")])), (py_str("boys"), py_tuple(vec![py_str("boy")])), (py_str("girls"), py_tuple(vec![py_str("girl")])), (py_str("sisters"), py_tuple(vec![py_str("sister")])), (py_str("brothers"), py_tuple(vec![py_str("brother")])), (py_str("sons"), py_tuple(vec![py_str("son")])), (py_str("daughters"), py_tuple(vec![py_str("daughter")])), (py_str("grandchildren"), py_tuple(vec![py_str("grandchild")])), (py_str("grandparents"), py_tuple(vec![py_str("grandparent")])), (py_str("aunts"), py_tuple(vec![py_str("aunt")])), (py_str("uncles"), py_tuple(vec![py_str("uncle")])), (py_str("nieces"), py_tuple(vec![py_str("niece")])), (py_str("nephews"), py_tuple(vec![py_str("nephew")])), (py_str("cousins"), py_tuple(vec![py_str("cousin")])), (py_str("flatmates"), py_tuple(vec![py_str("flatmate")])), (py_str("roommates"), py_tuple(vec![py_str("roommate")])), (py_str("friends"), py_tuple(vec![py_str("friend")])), (py_str("neighbours"), py_tuple(vec![py_str("neighbour")])), (py_str("grandkids"), py_tuple(vec![py_str("grandkid")]))]));
}
pub fn _the_word(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The group word as the answer says it, or \"\" for a word that is not a group.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "strip", args))(&__args) };
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THE_SAME_GROUP(), "get", args))(&__args) };
    return if py_truthy(&Py::Bool(py_contains(&g_GROUP_WORDS(), &found.clone()))) { found.clone() } else { py_str("") };
    Py::None
}
pub fn _ages_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut number: Py = Py::None;
    let _ = py_str("The ages a match holds, as integers, in the order they were said.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d{1,3}")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut number: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() };
    Py::None
}
pub fn the_list_given(args: &[Py]) -> Py {
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
    let mut ages: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The group word and the ages the line gives, or None.\n\n    Two ages at least: a line with one age is a single person and belongs to the store's own\n    reading, which is what `memory[\"their_age\"]` already is.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_GIVES_A_LIST(), g_GIVES_A_LIST_BARE(), g_THEYRE_A_LIST()]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    continue;
    }
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_word(args))(&__args) };
    ages = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ages")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ages_in(args))(&__args) };
    if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {
    return py_tuple(vec![word.clone(), ages.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn watch(args: &[Py]) -> Py {
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
    let mut ages: Py = Py::None;
    let mut given: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Keep the list a line gives, so the ask a turn later is answered from it.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    given = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_list_given(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&given.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&given.clone(), 2, None);
    word = __unpk1[0].clone();
    ages = __unpk1[1].clone();
    let _ = py_setitem(&memory.clone(), &py_str("group_ages"), py_dict(vec![(py_str("word"), word.clone()), (py_str("ages"), ages.clone()), (py_str("line"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) })]));
    Py::None
}
pub fn _the_kept(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The group and the ages the chat holds, or (\"\" , []) when it holds none.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_tuple(vec![py_str(""), py_list(vec![])]);
    }
    kept = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("group_ages")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_tuple(vec![py_str(""), py_list(vec![])]);
    }
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_word(args))(&__args) };
    ages = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ages")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut age: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(age.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(age.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {
    return py_tuple(vec![py_str(""), py_list(vec![])]);
    }
    return py_tuple(vec![word.clone(), ages.clone()]);
    Py::None
}
pub fn _a_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ages: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ages") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ages"))) };
    let mut age: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The ages as a person says them: \"4 and 7\", \"4, 7 and 9\".");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ages.clone()) { let mut age: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(age.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return geti(&said.clone(), &Py::Int(0));
    }
    return op_modulo(&py_str("%s and %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(", "), "join", args))(&__args) }, geti(&said.clone(), &op_neg(&Py::Int(1)))]));
    Py::None
}
pub fn _the_singulars(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("The singular of the group word the chat holds, in the words a ask may put after it.");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    out = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THE_SINGULARS(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "add", args))(&__args) };
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _the_end_one_is_the_group(args: &[Py]) -> Py {
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
    let mut end_one: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end_one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end_one"))) };
    let mut word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut after: Py = Py::None;
    let mut first: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the end-of-the-list ask is about the group the chat holds.\n\n    \"how old is my oldest\", \"how old is my oldest one\" and \"how old is my oldest kid\" leave the\n    group to the chat or name it; \"how old is my oldest cat\" names a cat, and a list of the\n    kids' ages is not a cat's. A noun after the end-one word that is not the group's own\n    singular is not this reader's line.\n    ");
    after = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&end_one.clone(), "end", args))(&__args) }, &Py::None, &Py::None);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&after.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut said: Py = __item.clone(); if py_truthy(&said.clone()) {  __out.push(said.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    first = geti(&words.clone(), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("one"), py_str("ones"), py_str("of")]), &first.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_singulars(args))(&__args) }, &first.clone()));
    Py::None
}
pub fn _the_group_asked(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("The group word the ask names, \"\" when it names no group, or None when it names another.\n\n    A line that names a group the chat has no list for gets nothing: \"how old are the girls\"\n    after a line about the parents is a question about people nobody has described.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GROUP_WORDS()) { let mut found: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\b%s\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(found.clone()); } } py_list(__out) })() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut found: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_word(args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return py_str("");
    }
    return if py_truthy(&Py::Bool(py_contains(&named.clone(), &word.clone()))) { word.clone() } else { Py::None };
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut ages: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut end_one: Py = Py::None;
    let mut line: Py = Py::None;
    let mut which: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reply to an ask about the ages the chat holds, or \"\" for every other line.");
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_kept(args))(&__args) }, 2, None);
    word = __unpk1[0].clone();
    ages = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&ages.clone())) } }) {
    return py_str("");
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_group_asked(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&asked.clone(), &Py::None))) {
    return py_str("");
    }
    end_one = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_THE_END_ONE(), "search", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&end_one.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(end_one.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_end_one_is_the_group(args))(&__args) })) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&end_one.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&asked.clone(), &py_str(""))) } }) {
    which = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("which")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&end_one.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("youngest"), py_str("younger"), py_str("little"), py_str("little one")]), &which.clone()))) {
    return op_modulo(&py_str("Your youngest is %d."), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    return op_modulo(&py_str("Your oldest is %d."), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    if py_truthy(&Py::Bool(!py_is(&end_one.clone(), &Py::None))) {
    which = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("which")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&end_one.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("youngest"), py_str("younger"), py_str("little"), py_str("little one")]), &which.clone()))) {
    return op_modulo(&py_str("Your youngest %s is %d."), &py_tuple(vec![if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { word.clone() }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    }
    return op_modulo(&py_str("Your oldest %s is %d."), &py_tuple(vec![if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { word.clone() }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_THE_LIST(), "search", args))(&__args) }))) {
    return py_str("");
    }
    return op_modulo(&py_str("Your %s are %s."), &py_tuple(vec![word.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_list(args))(&__args) }]));
    Py::None
}
pub fn asks_for_the_list(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks for the ages of the group the chat holds.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reply_for(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.groupages".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_THE_END_ONE" => Some(g_ASKS_THE_END_ONE()),
            "ASKS_THE_LIST" => Some(g_ASKS_THE_LIST()),
            "GIVES_A_LIST" => Some(g_GIVES_A_LIST()),
            "GIVES_A_LIST_BARE" => Some(g_GIVES_A_LIST_BARE()),
            "GROUP_WORDS" => Some(g_GROUP_WORDS()),
            "THEYRE_A_LIST" => Some(g_THEYRE_A_LIST()),
            "_AGE" => Some(g__AGE()),
            "_A_LIST_OF_AGES" => Some(g__A_LIST_OF_AGES()),
            "_GROUPS" => Some(g__GROUPS()),
            "_NOT_A_UNIT" => Some(g__NOT_A_UNIT()),
            "_THE_SAME_GROUP" => Some(g__THE_SAME_GROUP()),
            "_THE_SINGULARS" => Some(g__THE_SINGULARS()),
            "_a_list" => Some(py_func("_a_list", |args: &[Py]| _a_list(args))),
            "_ages_in" => Some(py_func("_ages_in", |args: &[Py]| _ages_in(args))),
            "_the_end_one_is_the_group" => Some(py_func("_the_end_one_is_the_group", |args: &[Py]| _the_end_one_is_the_group(args))),
            "_the_group_asked" => Some(py_func("_the_group_asked", |args: &[Py]| _the_group_asked(args))),
            "_the_kept" => Some(py_func("_the_kept", |args: &[Py]| _the_kept(args))),
            "_the_singulars" => Some(py_func("_the_singulars", |args: &[Py]| _the_singulars(args))),
            "_the_word" => Some(py_func("_the_word", |args: &[Py]| _the_word(args))),
            "asks_for_the_list" => Some(py_func("asks_for_the_list", |args: &[Py]| asks_for_the_list(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_list_given" => Some(py_func("the_list_given", |args: &[Py]| the_list_given(args))),
            "watch" => Some(py_func("watch", |args: &[Py]| watch(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS_THE_END_ONE" => s_ASKS_THE_END_ONE(v),
            "ASKS_THE_LIST" => s_ASKS_THE_LIST(v),
            "GIVES_A_LIST" => s_GIVES_A_LIST(v),
            "GIVES_A_LIST_BARE" => s_GIVES_A_LIST_BARE(v),
            "GROUP_WORDS" => s_GROUP_WORDS(v),
            "THEYRE_A_LIST" => s_THEYRE_A_LIST(v),
            "_AGE" => s__AGE(v),
            "_A_LIST_OF_AGES" => s__A_LIST_OF_AGES(v),
            "_GROUPS" => s__GROUPS(v),
            "_NOT_A_UNIT" => s__NOT_A_UNIT(v),
            "_THE_SAME_GROUP" => s__THE_SAME_GROUP(v),
            "_THE_SINGULARS" => s__THE_SINGULARS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.groupages", module_obj);
    pyrt::xrefs::reg_fn("memory.groupages", "_the_word", py_func("_the_word", |args: &[Py]| _the_word(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "_ages_in", py_func("_ages_in", |args: &[Py]| _ages_in(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "the_list_given", py_func("the_list_given", |args: &[Py]| the_list_given(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "watch", py_func("watch", |args: &[Py]| watch(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "_the_kept", py_func("_the_kept", |args: &[Py]| _the_kept(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "_a_list", py_func("_a_list", |args: &[Py]| _a_list(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "_the_singulars", py_func("_the_singulars", |args: &[Py]| _the_singulars(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "_the_end_one_is_the_group", py_func("_the_end_one_is_the_group", |args: &[Py]| _the_end_one_is_the_group(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "_the_group_asked", py_func("_the_group_asked", |args: &[Py]| _the_group_asked(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("memory.groupages", "asks_for_the_list", py_func("asks_for_the_list", |args: &[Py]| asks_for_the_list(args)));
    pyrt::xrefs::reg_access("memory.groupages", "ASKS_THE_END_ONE", g_ASKS_THE_END_ONE, s_ASKS_THE_END_ONE);
    pyrt::xrefs::reg_access("memory.groupages", "ASKS_THE_LIST", g_ASKS_THE_LIST, s_ASKS_THE_LIST);
    pyrt::xrefs::reg_access("memory.groupages", "GIVES_A_LIST", g_GIVES_A_LIST, s_GIVES_A_LIST);
    pyrt::xrefs::reg_access("memory.groupages", "GIVES_A_LIST_BARE", g_GIVES_A_LIST_BARE, s_GIVES_A_LIST_BARE);
    pyrt::xrefs::reg_access("memory.groupages", "GROUP_WORDS", g_GROUP_WORDS, s_GROUP_WORDS);
    pyrt::xrefs::reg_access("memory.groupages", "THEYRE_A_LIST", g_THEYRE_A_LIST, s_THEYRE_A_LIST);
    pyrt::xrefs::reg_access("memory.groupages", "_AGE", g__AGE, s__AGE);
    pyrt::xrefs::reg_access("memory.groupages", "_A_LIST_OF_AGES", g__A_LIST_OF_AGES, s__A_LIST_OF_AGES);
    pyrt::xrefs::reg_access("memory.groupages", "_GROUPS", g__GROUPS, s__GROUPS);
    pyrt::xrefs::reg_access("memory.groupages", "_NOT_A_UNIT", g__NOT_A_UNIT, s__NOT_A_UNIT);
    pyrt::xrefs::reg_access("memory.groupages", "_THE_SAME_GROUP", g__THE_SAME_GROUP, s__THE_SAME_GROUP);
    pyrt::xrefs::reg_access("memory.groupages", "_THE_SINGULARS", g__THE_SINGULARS, s__THE_SINGULARS);
}
