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


thread_local! { static G_AN_ATTEMPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HISTORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MACHINE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERIOUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__QUESTION_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SECOND_PERSON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIDY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_hedge: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_restates_the_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_unlike_the_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.thread_detail: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AN_ATTEMPT() -> Py { ensure_init(); G_AN_ATTEMPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ATTEMPT(v: Py) { ensure_init(); G_AN_ATTEMPT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HISTORY() -> Py { ensure_init(); G_A_HISTORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HISTORY(v: Py) { ensure_init(); G_A_HISTORY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIMIT() -> Py { ensure_init(); G_A_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIMIT(v: Py) { ensure_init(); G_A_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MACHINE_VERB() -> Py { ensure_init(); G_A_MACHINE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MACHINE_VERB(v: Py) { ensure_init(); G_A_MACHINE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON() -> Py { ensure_init(); G_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON(v: Py) { ensure_init(); G_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SERIOUS() -> Py { ensure_init(); G_SERIOUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERIOUS(v: Py) { ensure_init(); G_SERIOUS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SAYING() -> Py { ensure_init(); G__A_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SAYING(v: Py) { ensure_init(); G__A_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD() -> Py { ensure_init(); G__LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); G__LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__QUESTION_OPEN() -> Py { ensure_init(); G__QUESTION_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__QUESTION_OPEN(v: Py) { ensure_init(); G__QUESTION_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g__REPLIES() -> Py { ensure_init(); G__REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REPLIES(v: Py) { ensure_init(); G__REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g__SECOND_PERSON_WORDS() -> Py { ensure_init(); G__SECOND_PERSON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SECOND_PERSON_WORDS(v: Py) { ensure_init(); G__SECOND_PERSON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__TIDY() -> Py { ensure_init(); G__TIDY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIDY(v: Py) { ensure_init(); G__TIDY.with(|c| *c.borrow_mut() = v); }
pub fn g__a_hedge() -> Py { ensure_init(); G__a_hedge.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_hedge(v: Py) { ensure_init(); G__a_hedge.with(|c| *c.borrow_mut() = v); }
pub fn g_restates_the_line() -> Py { ensure_init(); G_restates_the_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_restates_the_line(v: Py) { ensure_init(); G_restates_the_line.with(|c| *c.borrow_mut() = v); }
pub fn g_unlike_the_line() -> Py { ensure_init(); G_unlike_the_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_unlike_the_line(v: Py) { ensure_init(); G_unlike_the_line.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A detail added to a thread, answered as a detail of that thread.\n\nThe turn that goes wrong in the census is not the question, it is the sentence before it. A\nperson gives the bot something to work with - \"she doesnt like surprises\" at a party being\nplanned, \"it was fine for two years\" about a boiler, \"ive said no once already\" about a boss -\nand the bot answers \"I'm listening - say more if it helps\", or says the line back (\"Got it -\nyou've said no once already\"), or shrugs at a search. The next turn, phrased as a question, gets\na real card: the bot had the thread all along and spent one turn of it saying nothing.\n\nSo this reading takes that turn. Three kinds of detail are worth one question each, and the\nquestion is the one that decides what happens next:\n\n* a **limit** somebody in the plan has (\"she doesnt like surprises\", \"my dad wont eat pork\",\n  \"one of us doesnt drink\") - the plan works round it, and what matters is whether it is a hard\n  no or a thing to keep off the plan;\n* an **attempt** already made (\"ive said no once already\", \"we tried that\") - what matters is\n  how it went down;\n* a **history** (\"it was fine for two years\", \"it started in march\") - what matters is what\n  changed, or how often it is happening now.\n\nNothing here holds a fact about the world. The reply is the person's own clause said back once,\nshort, with the question that follows from it, which is what a friend does with a detail dropped\ninto a conversation. It speaks only where the composer would otherwise have said nothing about\nthe line - a hedge, the line given back, or a stock line that names none of it - so any reader\nthat found its own answer keeps the turn.\n");
    s__a_hedge(py_func("is_a_hedge", |args: &[Py]| pyrt::xrefs::xcall("compose.aboutness", "is_a_hedge", args)));
    s_restates_the_line(py_func("restates_the_line", |args: &[Py]| pyrt::xrefs::xcall("compose.aboutness", "restates_the_line", args)));
    s_unlike_the_line(py_func("unlike_the_line", |args: &[Py]| pyrt::xrefs::xcall("compose.aboutness", "unlike_the_line", args)));
    s_A_LIMIT(stdlib_call("re", "compile", &[py_str("(?:doesn'?t|does\\s+not|won'?t|will\\s+not|can'?t|cannot|isn'?t\\s+keen|not\\s+keen|hates?|allergic|refuses?|avoids?|no\\s+(?:meat|alcohol|drink|drinking|stairs|loud\\s+\\w+|sugar|gluten|dairy|fish|pork|nuts))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|him|her|them|we|us|i|i'?m|im|i'?ve|you|one\\s+of\\s+us|somebody|someone|everybody|everyone|(?:my|our|his|her|their|your)\\s+(?:mum|mom|mother|dad|father|son|daughter|child|kid|baby|wife|husband|partner|boyfriend|girlfriend|brother|sister|friend|mate|nan|gran|grandad|grandma|aunt|uncle|cousin|boss|colleague|family|in-laws?|parents?|niece|nephew|flatmate|housemate|guests?))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MACHINE_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:boils?|starts?|charges?|connects?|turns?|shuts?|opens?|closes?|fires?|spins?|prints?|works?|drains?|heats?|pairs?|loads?|syncs?|boots?|scans?|locks?|rings?|beeps?|plays?|records?|downloads?|uploads?|updates?|runs?|shows?|detects?|responds?|freezes?|crashes?|drips?|flushes?|restarts?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SAYING(py_str("said|tried|asked|told|rung|rang|called|emailed|texted|written|complained"));
    s_AN_ATTEMPT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:\\b(?:i|we|he|she|they|you)(?:'?ve|\\s+have|\\s+had)?\\s+(?:already\\s+|just\\s+)?(?:"), &g__A_SAYING()), &py_str(")\\b|\\b(?:")), &g__A_SAYING()), &py_str(")\\s+(?:that|it|him|her|them)\\b|\\b(?:already|just)\\s+(?:")), &g__A_SAYING()), &py_str(")\\b|\\bi\\s+did\\s+that\\b|\\bsaid\\s+no\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HISTORY(stdlib_call("re", "compile", &[py_str("(?:\\bwas\\s+(?:fine|ok|okay|all\\s+right)\\b|\\bused\\s+to\\b|\\bsince\\s+(?:last|the|march|april|may|june|july|august|september|october|november|december|january|february)\\b|\\bfor\\s+(?:\\d+|two|three|four|five|six|seven|eight|nine|ten|a\\s+few|years|months|weeks|days)\\b|\\bit\\s+started\\b|\\bstarted\\s+(?:in|last|when)\\b|\\bevery\\s+(?:day|week|morning|night|time)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__REPLIES(py_dict(vec![(py_str("limit"), py_tuple(vec![py_str("%s - is that a hard no, or just something to keep off the plan?"), py_str("%s, then. Does that rule the whole thing out, or only that part of it?"), py_str("%s - how far does the plan have to go round it?")])), (py_str("attempt"), py_tuple(vec![py_str("%s - how did that go down?"), py_str("%s, so what did they say?"), py_str("%s - what came of it?")])), (py_str("history"), py_tuple(vec![py_str("%s - how often is it happening now?"), py_str("%s, so something changed. What was it like just before that?"), py_str("%s - has anything else changed since?")]))]));
    fn __data1() -> Py { py_tuple(vec![py_tuple(vec![py_str("\\bi'?m\\b"), py_str("I'm")]), py_tuple(vec![py_str("\\bi'?ve\\b"), py_str("I've")]), py_tuple(vec![py_str("\\bi'?ll\\b"), py_str("I'll")]), py_tuple(vec![py_str("\\bi'?d\\b"), py_str("I'd")]), py_tuple(vec![py_str("\\bdont\\b"), py_str("don't")]), py_tuple(vec![py_str("\\bdoesnt\\b"), py_str("doesn't")]), py_tuple(vec![py_str("\\bdidnt\\b"), py_str("didn't")]), py_tuple(vec![py_str("\\bcant\\b"), py_str("can't")]), py_tuple(vec![py_str("\\bwont\\b"), py_str("won't")]), py_tuple(vec![py_str("\\bim\\b"), py_str("I'm")]), py_tuple(vec![py_str("\\bive\\b"), py_str("I've")]), py_tuple(vec![py_str("\\bhavent\\b"), py_str("haven't")]), py_tuple(vec![py_str("\\bisnt\\b"), py_str("isn't")]), py_tuple(vec![py_str("\\bwasnt\\b"), py_str("wasn't")]), py_tuple(vec![py_str("\\btheyre\\b"), py_str("they're")]), py_tuple(vec![py_str("\\bthats\\b"), py_str("that's")]), py_tuple(vec![py_str("\\bits\\b"), py_str("it's")]), py_tuple(vec![py_str("\\bi\\b"), py_str("I")])]) }
    s__TIDY(op_add(&__data1(), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday")])) { let mut word: Py = __item.clone(); __out.push(py_tuple(vec![op_modulo(&py_str("\\b%s\\b"), &word.clone()), mcall(&word.clone(), "capitalize", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    s__LEAD(stdlib_call("re", "compile", &[py_str("^(?:and|but|so|also|well|ok|okay|oh|ah|right|yeah|yes)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__QUESTION_OPEN(stdlib_call("re", "compile", &[py_str("^(?:what|why|how|when|where|who|which|whose|is|are|was|were|do|does|did|can|could|should|would|will|shall|have|has|had|am|if)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SERIOUS(stdlib_call("re", "compile", &[py_str("\\b(?:bull(?:y|ied|ies|ying)|cancer|chemo|tumour|biopsy|died|dying|death|funeral|bereave\\w*|grief|grieving|abuse[ds]?|assault\\w*|miscarr\\w*|stillborn|self[-\\s]?harm|suicide|suicidal|custody|social\\s+services|evict\\w*|homeless|bankrupt\\w*|debt\\s+collector|depress\\w*|anxiety|panic\\s+attack|addict\\w*|drinking\\s+too\\s+much|hospital|sectioned)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_dict(vec![(py_str("i"), py_str("you")), (py_str("i'm"), py_str("you're")), (py_str("i've"), py_str("you've")), (py_str("i'll"), py_str("you'll")), (py_str("i'd"), py_str("you'd")), (py_str("im"), py_str("you're")), (py_str("ive"), py_str("you've")), (py_str("ill"), py_str("you'll")), (py_str("id"), py_str("you'd")), (py_str("my"), py_str("your")), (py_str("mine"), py_str("yours")), (py_str("me"), py_str("you")), (py_str("myself"), py_str("yourself"))]) }
    s__SECOND_PERSON_WORDS(__data2());
}
pub fn _clean_line(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn the_thread_is_serious(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { py_str("") };
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when this thread is about somebody's health, safety or grief rather than a plan.");
    said = op_add(&op_add(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), &py_str(" ")), &call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)));
    return call_builtin("bool", &[mcall(&g_SERIOUS(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn fallback_heavy(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let _ = py_str("True when the thread is heavy news, read through the composer's own test for it.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = py_call(&pyrt::xrefs::xget("compose.fallback", "reads_as_heavy_news"), &[_clean_line(&[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")])])]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fallback", "_the_thread_is_heavy", &[py_dict(vec![(py_str("turns"), turns.clone())])]) } }]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn the_clause_said_back(args: &[Py]) -> Py {
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
    let mut instead: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shorter: Py = Py::None;
    let _ = py_str("The person's clause, as the bot would write it: leading join words off, punctuation off.");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .,!?-")]);
    while py_truthy(&Py::Bool(true)) {
    shorter = mcall(&g__LEAD(), "sub", &[py_str(""), said.clone()]);
    if py_truthy(&Py::Bool(py_eq(&shorter.clone(), &said.clone()))) {
    break;
    }
    said = shorter.clone();
    }
    let __iter1 = py_iter_or_raise(&g__TIDY());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    instead = __unpk2[1].clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(instead.clone()); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    return op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn the_kind_of_detail(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let _ = py_str("Which kind of detail the line drops into the thread, or None when it drops none.");
    if py_truthy(&{ let __b = mcall(&g_A_LIMIT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PERSON(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_MACHINE_VERB(), "search", &[clean.clone()]))) } } } }) {
    return py_str("limit");
    }
    if py_truthy(&mcall(&g_AN_ATTEMPT(), "search", &[clean.clone()])) {
    return py_str("attempt");
    }
    if py_truthy(&mcall(&g_A_HISTORY(), "search", &[clean.clone()])) {
    return py_str("history");
    }
    return Py::None;
    Py::None
}
pub fn detail_pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clause: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The one question the detail raises, or None when this turn is not that turn.");
    clean = _clean_line(&[clean.clone()]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g__QUESTION_OPEN(), "match", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = fallback_heavy(&[turns.clone()]); if py_truthy(&__b) { __b } else { the_thread_is_serious(&[turns.clone()]) } }) {
    return Py::None;
    }
    kind = the_kind_of_detail(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    clause = the_clause_said_back(&[clean.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::None;
    }
    said = _second_person(&[clause.clone()]);
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&g__REPLIES(), &kind.clone())) { let mut line: Py = __item.clone(); __out.push(op_modulo(&line.clone(), &said.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _second_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut at: Py = Py::None;
    let mut core: Py = Py::None;
    let mut swap: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[mcall(&call_builtin("str", &[{ let __b = clause.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    core = mcall(&word.clone(), "rstrip", &[py_str(",.?!")]);
    swap = mcall(&g__SECOND_PERSON_WORDS(), "get", &[mcall(&mcall(&mcall(&core.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "replace", &[py_str("'"), py_str("'")])]);
    if py_truthy(&Py::Bool(!py_truthy(&swap.clone()))) {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&at.clone(), &Py::Int(0)))) {
    swap = op_add(&mcall(&slice_or_raise(&swap.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&swap.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    let _ = mcall(&words.clone(), "append", &[op_add(&swap.clone(), &slice_or_raise(&word.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))]);
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn says_nothing_about_the_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut about: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a reply is a hedge, the line given back, or a stock line about nothing.");
    reply = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[])))) {
    return Py::Bool(true);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&reply.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(22))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.aboutness", "is_a_hedge", &[reply.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.aboutness", "restates_the_line", &[reply.clone(), text.clone()]) } }) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&pyrt::xrefs::xcall("compose.aboutness", "unlike_the_line", &[reply.clone(), text.clone()]), 2, None);
    about = __unpk1[0].clone();
    said = __unpk1[1].clone();
    return { let __b = call_builtin("bool", &[about.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.thread_detail".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_ATTEMPT" => Some(g_AN_ATTEMPT()),
            "A_HISTORY" => Some(g_A_HISTORY()),
            "A_LIMIT" => Some(g_A_LIMIT()),
            "A_MACHINE_VERB" => Some(g_A_MACHINE_VERB()),
            "A_PERSON" => Some(g_A_PERSON()),
            "SERIOUS" => Some(g_SERIOUS()),
            "_A_SAYING" => Some(g__A_SAYING()),
            "_LEAD" => Some(g__LEAD()),
            "_QUESTION_OPEN" => Some(g__QUESTION_OPEN()),
            "_REPLIES" => Some(g__REPLIES()),
            "_SECOND_PERSON_WORDS" => Some(g__SECOND_PERSON_WORDS()),
            "_TIDY" => Some(g__TIDY()),
            "_a_hedge" => Some(g__a_hedge()),
            "_clean_line" => Some(py_func("_clean_line", |args: &[Py]| _clean_line(args))),
            "_second_person" => Some(py_func("_second_person", |args: &[Py]| _second_person(args))),
            "detail_pool" => Some(py_func("detail_pool", |args: &[Py]| detail_pool(args))),
            "fallback_heavy" => Some(py_func("fallback_heavy", |args: &[Py]| fallback_heavy(args))),
            "restates_the_line" => Some(g_restates_the_line()),
            "says_nothing_about_the_line" => Some(py_func("says_nothing_about_the_line", |args: &[Py]| says_nothing_about_the_line(args))),
            "the_clause_said_back" => Some(py_func("the_clause_said_back", |args: &[Py]| the_clause_said_back(args))),
            "the_kind_of_detail" => Some(py_func("the_kind_of_detail", |args: &[Py]| the_kind_of_detail(args))),
            "the_thread_is_serious" => Some(py_func("the_thread_is_serious", |args: &[Py]| the_thread_is_serious(args))),
            "unlike_the_line" => Some(g_unlike_the_line()),
            "_fallback" => Some(pyrt::xrefs::xmodule("compose.fallback")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_ATTEMPT" => s_AN_ATTEMPT(v),
            "A_HISTORY" => s_A_HISTORY(v),
            "A_LIMIT" => s_A_LIMIT(v),
            "A_MACHINE_VERB" => s_A_MACHINE_VERB(v),
            "A_PERSON" => s_A_PERSON(v),
            "SERIOUS" => s_SERIOUS(v),
            "_A_SAYING" => s__A_SAYING(v),
            "_LEAD" => s__LEAD(v),
            "_QUESTION_OPEN" => s__QUESTION_OPEN(v),
            "_REPLIES" => s__REPLIES(v),
            "_SECOND_PERSON_WORDS" => s__SECOND_PERSON_WORDS(v),
            "_TIDY" => s__TIDY(v),
            "_a_hedge" => s__a_hedge(v),
            "restates_the_line" => s_restates_the_line(v),
            "unlike_the_line" => s_unlike_the_line(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.thread_detail", module_obj);
    pyrt::xrefs::reg_fn("compose.thread_detail", "_clean_line", py_func("_clean_line", |args: &[Py]| _clean_line(args)));
    pyrt::xrefs::reg_fn("compose.thread_detail", "the_thread_is_serious", py_func("the_thread_is_serious", |args: &[Py]| the_thread_is_serious(args)));
    pyrt::xrefs::reg_fn("compose.thread_detail", "fallback_heavy", py_func("fallback_heavy", |args: &[Py]| fallback_heavy(args)));
    pyrt::xrefs::reg_fn("compose.thread_detail", "the_clause_said_back", py_func("the_clause_said_back", |args: &[Py]| the_clause_said_back(args)));
    pyrt::xrefs::reg_fn("compose.thread_detail", "the_kind_of_detail", py_func("the_kind_of_detail", |args: &[Py]| the_kind_of_detail(args)));
    pyrt::xrefs::reg_fn("compose.thread_detail", "detail_pool", py_func("detail_pool", |args: &[Py]| detail_pool(args)));
    pyrt::xrefs::reg_fn("compose.thread_detail", "_second_person", py_func("_second_person", |args: &[Py]| _second_person(args)));
    pyrt::xrefs::reg_fn("compose.thread_detail", "says_nothing_about_the_line", py_func("says_nothing_about_the_line", |args: &[Py]| says_nothing_about_the_line(args)));
    pyrt::xrefs::reg_access("compose.thread_detail", "AN_ATTEMPT", g_AN_ATTEMPT, s_AN_ATTEMPT);
    pyrt::xrefs::reg_access("compose.thread_detail", "A_HISTORY", g_A_HISTORY, s_A_HISTORY);
    pyrt::xrefs::reg_access("compose.thread_detail", "A_LIMIT", g_A_LIMIT, s_A_LIMIT);
    pyrt::xrefs::reg_access("compose.thread_detail", "A_MACHINE_VERB", g_A_MACHINE_VERB, s_A_MACHINE_VERB);
    pyrt::xrefs::reg_access("compose.thread_detail", "A_PERSON", g_A_PERSON, s_A_PERSON);
    pyrt::xrefs::reg_access("compose.thread_detail", "SERIOUS", g_SERIOUS, s_SERIOUS);
    pyrt::xrefs::reg_access("compose.thread_detail", "_A_SAYING", g__A_SAYING, s__A_SAYING);
    pyrt::xrefs::reg_access("compose.thread_detail", "_LEAD", g__LEAD, s__LEAD);
    pyrt::xrefs::reg_access("compose.thread_detail", "_QUESTION_OPEN", g__QUESTION_OPEN, s__QUESTION_OPEN);
    pyrt::xrefs::reg_access("compose.thread_detail", "_REPLIES", g__REPLIES, s__REPLIES);
    pyrt::xrefs::reg_access("compose.thread_detail", "_SECOND_PERSON_WORDS", g__SECOND_PERSON_WORDS, s__SECOND_PERSON_WORDS);
    pyrt::xrefs::reg_access("compose.thread_detail", "_TIDY", g__TIDY, s__TIDY);
    pyrt::xrefs::reg_access("compose.thread_detail", "_a_hedge", g__a_hedge, s__a_hedge);
    pyrt::xrefs::reg_access("compose.thread_detail", "restates_the_line", g_restates_the_line, s_restates_the_line);
    pyrt::xrefs::reg_access("compose.thread_detail", "unlike_the_line", g_unlike_the_line, s_unlike_the_line);
}
