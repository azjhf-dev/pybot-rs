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


thread_local! { static G__ABBREVIATION_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_HOW_OFTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAUSE_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIFTABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENS_WITH_A_WARNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENS_WITH_SYMPATHY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_BACKWARDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SENTENCE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARED_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SIGNPOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THINKING_IT_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOTAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERDICT_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.lead: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g__ABBREVIATION_END() -> Py { ensure_init(); G__ABBREVIATION_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABBREVIATION_END(v: Py) { ensure_init(); G__ABBREVIATION_END.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_HOW_OFTEN() -> Py { ensure_init(); G__ASKS_HOW_OFTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_HOW_OFTEN(v: Py) { ensure_init(); G__ASKS_HOW_OFTEN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RATE() -> Py { ensure_init(); G__A_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RATE(v: Py) { ensure_init(); G__A_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g__CAUSE_SENTENCE() -> Py { ensure_init(); G__CAUSE_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAUSE_SENTENCE(v: Py) { ensure_init(); G__CAUSE_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__HARD() -> Py { ensure_init(); G__HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HARD(v: Py) { ensure_init(); G__HARD.with(|c| *c.borrow_mut() = v); }
pub fn g__LIFTABLE() -> Py { ensure_init(); G__LIFTABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIFTABLE(v: Py) { ensure_init(); G__LIFTABLE.with(|c| *c.borrow_mut() = v); }
pub fn g__METHOD() -> Py { ensure_init(); G__METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__METHOD(v: Py) { ensure_init(); G__METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENING() -> Py { ensure_init(); G__OPENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENING(v: Py) { ensure_init(); G__OPENING.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENS_WITH_A_WARNING() -> Py { ensure_init(); G__OPENS_WITH_A_WARNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENS_WITH_A_WARNING(v: Py) { ensure_init(); G__OPENS_WITH_A_WARNING.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENS_WITH_SYMPATHY() -> Py { ensure_init(); G__OPENS_WITH_SYMPATHY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENS_WITH_SYMPATHY(v: Py) { ensure_init(); G__OPENS_WITH_SYMPATHY.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_BACKWARDS() -> Py { ensure_init(); G__POINTS_BACKWARDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_BACKWARDS(v: Py) { ensure_init(); G__POINTS_BACKWARDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_WORD() -> Py { ensure_init(); G__SAID_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_WORD(v: Py) { ensure_init(); G__SAID_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__SENTENCE_END() -> Py { ensure_init(); G__SENTENCE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SENTENCE_END(v: Py) { ensure_init(); G__SENTENCE_END.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARED_WORDS() -> Py { ensure_init(); G__SHARED_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARED_WORDS(v: Py) { ensure_init(); G__SHARED_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SIGNPOST() -> Py { ensure_init(); G__SIGNPOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SIGNPOST(v: Py) { ensure_init(); G__SIGNPOST.with(|c| *c.borrow_mut() = v); }
pub fn g__THINKING_IT_OVER() -> Py { ensure_init(); G__THINKING_IT_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THINKING_IT_OVER(v: Py) { ensure_init(); G__THINKING_IT_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g__TOTAL() -> Py { ensure_init(); G__TOTAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOTAL(v: Py) { ensure_init(); G__TOTAL.with(|c| *c.borrow_mut() = v); }
pub fn g__VERDICT_OPENS() -> Py { ensure_init(); G__VERDICT_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERDICT_OPENS(v: Py) { ensure_init(); G__VERDICT_OPENS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The answer in the first sentence, for every answer module at once.\n\nTwo rules live here, both about the same thing: STYLE.md's \"the answer in the first sentence\",\nread from `memory[\"understood\"][\"wants\"]`. The first turns a method round when a card written for\n\"how long\" is asked \"how\"; the second puts a row's own declared opening (`entry(..., lead=...)`)\nin front of a card that is written for the harder question and has the plain one to answer too\n(`entry(..., lead=(\"yesno\", \"Usually, yes\"))`: the shape of line, then the sentence owed to it).\nBoth are read by the one place every knowledge row is rendered (`bot/modules/answers.py`), so a\nrow in any domain gets them without a module changing.\n\nSTYLE.md asks for the answer in the first sentence. A row that holds a dish holds one answer for\nevery way the dish is asked about, so it is written the way the clock asks for it: \"About 40\nminutes start to finish. Fry 500g mince hard ...\". That is the answer to \"how long does chilli\ntake\" and it is not the answer to \"how do i make chilli\", which is the method the person came\nfor: read out, the first thing they get is a duration they did not ask for, and the paragraph\nthat tells them what to do starts in the second sentence.\n\nSo when the line asks for the method (`memory[\"understood\"][\"wants\"]` is `steps`), a reply whose\nopening sentence is only the total time of the job is turned round: the method leads and the\ntotal time closes the answer, where it is still useful and no longer in the way of it. Nothing is\nadded, cut or reworded - the same sentence moves to the end - and every other ask is left exactly\nas it is, because \"how long\" wants the clock first and a limit or a fault is not this file's.\n\nThe move is safe because the sentence it moves says only how long the whole thing takes: it\ncarries \"start to finish\", \"from start to finish\", \"in total\", \"altogether\" or \"all in\", so it is\na summary of the job and not one step's own time (\"simmer 30 minutes\" stays where it is, inside\nthe method it belongs to).\n");
    s__TOTAL(stdlib_call("re", "compile", &[py_str("^(?:about|around|roughly|just|only|under|over|about an?|an?|a)\\s*[^.]{0,60}\\b(?:minutes?|mins?|hours?|hrs?|days?)\\b[^.]{0,60}?\\b(?:start to finish|from start to finish|in total|altogether|all in|end to end)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OPENING(stdlib_call("re", "compile", &[py_str("^\\s*(\\S.*?[.!?])\\s+(\\S.*)$"), stdlib_attr("re", "S").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__METHOD(call_builtin("frozenset", &[py_tuple(vec![py_str("steps")])]).unwrap_or_else(|e| exc::raise(e)));
    s__SIGNPOST(stdlib_call("re", "compile", &[py_str("^(?:(?:a few|a couple of|two|three|four|five|six|\\d+)\\s+(?:things?|tests?|steps?|options?|ideas?|ways?|rules?|checks?|questions?|tips?|notes?|points?|parts?)(?:\\s*,?\\s*(?:in this order|to know|to try|to check|to keep in mind|first|for you|to start with|before you start))?|here(?:'?s| is| are)\\s+(?:the|a|an|what|how|my|our|it|this)\\b[^:]{0,60}|the following\\b[^:]{0,40}|these are\\b[^:]{0,40}|(?:great|good|fair|interesting|nice|lovely) question|(?:that'?s|it'?s) (?:a )?(?:great|good|fair|interesting|nice) question|you (?:asked|want(?:ed)? to know|were asking|are asking)\\b[^:]{0,80}|so (?:you'?re|you are) asking\\b[^:]{0,80}|the question (?:is|was)\\b[^:]{0,80}|let me (?:start|explain|answer|walk you through|take you through|think)|i'?ll (?:start|explain|answer|walk you through|take you through)|i can (?:help|answer|explain|tell you))$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SENTENCE_END(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s__ABBREVIATION_END(stdlib_call("re", "compile", &[py_str("\\b(?:e\\.g|i\\.e|approx|incl|etc|vs|cf|fig|dept|est|ref|tel|mr|mrs|ms|dr|prof)\\.$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_HOW_OFTEN(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:often|frequently)\\b|\\bhow\\s+many\\s+times\\b|\\bwhat\\s+interval\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_RATE(stdlib_call("re", "compile", &[py_str("\\b(?:once|twice|(?:three|four|five|six|\\d+)\\s+times|every(?:\\s+other)?(?:\\s+\\w+)?\\s+(?:day|week|month|year|morning|evening|night)|daily|weekly|monthly|yearly|nightly|fortnightly|\\d+\\s+times\\s+(?:a|an|per|each)\\s+(?:day|week|month|year)|(?:two|three|four|five)\\s+times\\s+(?:a|an|per|each)\\s+(?:day|week|month|year))(?:\\s+(?:a|an|per|each)\\s+(?:day|week|month|year))?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__VERDICT_OPENS(stdlib_call("re", "compile", &[py_str("^(?:yes|yeah|yep|no|nope)\\b(?=\\s*[,.;:!?-]|\\s*$)|^(?:not really|not usually|not always)\\b|^(?:worth it|not worth it|worth the money)\\b|^(?:it is|it's) (?:worth|not worth|fine|safe|okay|ok|not safe|not fine)\\b|^(?:it depends|usually|definitely|in most cases|most of the time)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THINKING_IT_OVER(stdlib_call("re", "compile", &[py_str("\\bworth\\b[^.]{0,40}\\b(?:whether|asking|thinking|a thought|considering|wondering|talking about it|a conversation|a talk|checking first)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CAUSE_SENTENCE(stdlib_call("re", "compile", &[py_str("^\\s*(?:the|a|an)\\s+(?:usual|main|most likely|likely|commonest|common|real|other|whole)?\\s*(?:reason|cause)s?\\b(?![?!])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HARD(call_builtin("frozenset", &[py_tuple(vec![pyrt::xrefs::xget("compose.fit", "TIME"), pyrt::xrefs::xget("compose.fit", "AMOUNT"), pyrt::xrefs::xget("compose.fit", "PLACE"), pyrt::xrefs::xget("compose.fit", "PERSON"), pyrt::xrefs::xget("compose.fit", "MEANING"), pyrt::xrefs::xget("compose.fit", "YESNO"), pyrt::xrefs::xget("compose.fit", "PICK"), pyrt::xrefs::xget("compose.fit", "STEPS"), pyrt::xrefs::xget("compose.fit", "OPTIONS"), pyrt::xrefs::xget("compose.fit", "REASON")])]).unwrap_or_else(|e| exc::raise(e)));
    s__SAID_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9]+")]).unwrap_or_else(|e| exc::raise(e)));
    s__LIFTABLE(call_builtin("frozenset", &[py_tuple(vec![pyrt::xrefs::xget("compose.fit", "TIME"), pyrt::xrefs::xget("compose.fit", "AMOUNT"), pyrt::xrefs::xget("compose.fit", "PLACE")])]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_BACKWARDS(stdlib_call("re", "compile", &[py_str("^\\s*(?:it|its|it's|that|that's|this|these|those|they|them|their|there\\s+(?:it|they|you|we|that|those|these)\\b|then|so|and|but|also|otherwise|still|though|however|either|neither|both|such|the same|which|who|if|unless|in that case|at that point|after that|for now)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OPENS_WITH_SYMPATHY(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:i'm|i am|so|very|truly)\\s+)*(?:sorry|apologies)\\b|^\\s*that'?s\\s+(?:hard|awful|terrible|rough|grim|rotten|a lot)\\b|^\\s*i'?m\\s+so\\s+sorry\\b|^\\s*my\\s+(?:heart|sympathies)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OPENS_WITH_A_WARNING(stdlib_call("re", "compile", &[py_str("^\\s*(?:there'?s?\\s+(?:is\\s+)?no\\s+safe\\b|no\\s+safe\\b|never\\b|do\\s+not\\b|don'?t\\b|(?:call|ring|phone)\\s+999\\b|999\\b|go\\s+to\\s+(?:a&e|the\\s+hospital)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("the"), py_str("and"), py_str("for"), py_str("that"), py_str("this"), py_str("with"), py_str("you"), py_str("your"), py_str("have"), py_str("has"), py_str("are"), py_str("was"), py_str("were"), py_str("what"), py_str("when"), py_str("where"), py_str("which"), py_str("who"), py_str("why"), py_str("how"), py_str("should"), py_str("could"), py_str("would"), py_str("will"), py_str("been"), py_str("being"), py_str("about"), py_str("there"), py_str("their"), py_str("them"), py_str("they"), py_str("from"), py_str("into"), py_str("than"), py_str("then"), py_str("some"), py_str("any"), py_str("can"), py_str("does"), py_str("did"), py_str("doing"), py_str("not"), py_str("but"), py_str("one"), py_str("out"), py_str("over"), py_str("very"), py_str("just"), py_str("like"), py_str("get"), py_str("gets"), py_str("got"), py_str("much"), py_str("many"), py_str("more"), py_str("most"), py_str("long"), py_str("take"), py_str("takes"), py_str("need"), py_str("needs"), py_str("want"), py_str("wants"), py_str("make"), py_str("made"), py_str("is"), py_str("it")]) }
    s__SHARED_WORDS(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut before: Py = Py::None;
    let mut end: Py = Py::None;
    let mut out: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut start: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The reply's sentences, in order, with their own punctuation kept.");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_list(vec![]);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    out = __unpk1[0].clone();
    start = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g__SENTENCE_END(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    end = __item.clone();
    before = slice_or_raise(&text.clone(), &start.clone(), &mcall(&end.clone(), "start", &[]), &Py::None);
    if py_truthy(&mcall(&g__ABBREVIATION_END(), "search", &[before.clone()])) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[mcall(&slice_or_raise(&text.clone(), &start.clone(), &mcall(&end.clone(), "start", &[]), &Py::None), "strip", &[])]);
    start = mcall(&end.clone(), "end", &[]);
    }
    if py_truthy(&mcall(&slice_or_raise(&text.clone(), &start.clone(), &Py::None, &Py::None), "strip", &[])) {
    let _ = mcall(&out.clone(), "append", &[mcall(&slice_or_raise(&text.clone(), &start.clone(), &Py::None, &Py::None), "strip", &[])]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut sentence: Py = __item.clone(); if py_truthy(&sentence.clone()) {  __out.push(sentence.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _wants_the_method(args: &[Py]) -> Py {
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
    let mut want: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    want = mcall(&understood.clone(), "get", &[py_str("wants")]);
    if py_truthy(&call_builtin("isinstance", &[want.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    want = mcall(&mcall(&want.clone(), "strip", &[]), "lower", &[]);
    }
    return Py::Bool(py_contains(&g__METHOD(), &want.clone()));
    Py::None
}
pub fn rate_first(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut at: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The rate sentence leads when the line asked how often and the row puts it second.\n\n    A row holds one answer for every way its subject is asked about, and a row written for the job\n    answers the frequency later: \"Cut them straight across rather than down the sides, leave a\n    millimetre or two of white ... Once a fortnight is plenty\". Read out to someone who asked\n    \"how often should i trim my toenails\", that answers a different question for a sentence and\n    reaches the rate last.\n\n    Only a sentence that opens on the rate is moved - \"Once a fortnight is plenty\", \"Every six\n    months\", \"Two or three times a week\" - because a rate named inside a sentence about something\n    else (\"most freezers need this once or twice a year\") is that other sentence's fact, not the\n    rate on its own. Nothing is added, cut or reworded: the same sentence moves, and the sentence\n    it passes keeps its place behind it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__ASKS_HOW_OFTEN(), "search", &[line.clone()]))) } }) {
    return reply.clone();
    }
    parts = _sentences(&[reply.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return reply.clone();
    }
    if py_truthy(&mcall(&g__A_RATE(), "search", &[geti(&parts.clone(), &Py::Int(0))])) {
    return reply.clone();
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    sentence = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&g__A_RATE(), "match", &[sentence.clone()]); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(22)) } }) {
    return mcall(&py_str(" "), "join", &[op_add(&op_add(&py_list(vec![sentence.clone()]), &slice_or_raise(&parts.clone(), &Py::None, &at.clone(), &Py::None)), &slice_or_raise(&parts.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None))]);
    }
    }
    return reply.clone();
    Py::None
}
pub fn answer_first(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut first: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The answer leads: a sentence that only announces the answer is not the answer.\n\n    A row, a card or a composer line is often written to be read on a screen, where a signpost\n    helps: \"Three tests, in this order.\", \"Two things to know.\", \"Here's the short version.\",\n    \"Good question.\", \"You asked how long rice takes.\" Read out as the first thing the person\n    gets, it answers nothing. STYLE.md puts the answer in the first sentence, so that sentence is\n    dropped and the sentence it was announcing leads.\n\n    The same reading is used for every shape of answer: a signpost is not a time, a figure, a\n    list, a comparison, a verdict or a method, so no table of labels is kept here. The sentence is\n    dropped only when there is another sentence to lead, and nothing else is reordered, added or\n    reworded. A count in a signpost (\"three tests\") is still carried by the tests that follow.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    reply = rate_first(&[reply.clone(), understood.clone(), line.clone()]);
    parts = _sentences(&[reply.clone()]);
    if py_truthy(&op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    first = mcall(&mcall(&geti(&parts.clone(), &Py::Int(0)), "rstrip", &[py_str(".!? ")]), "strip", &[]);
    if py_truthy(&{ let __b = first.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__SIGNPOST(), "match", &[first.clone()]) } }) {
    reply = mcall(&py_str(" "), "join", &[slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    }
    }
    reply = verdict_first(&[reply.clone(), understood.clone(), line.clone()]);
    if py_truthy(&_wants_the_method(&[understood.clone()])) {
    return method_first(&[reply.clone(), understood.clone()]);
    }
    return cause_first(&[reply.clone(), understood.clone()]);
    Py::None
}
pub fn verdict_first(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut bare: Py = Py::None;
    let mut bare_opening: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut place: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The verdict leads when the line asked yes or no and the card gave it later.\n\n    A row holds one answer for every way its subject is asked about, and the worth-it cards are\n    written to be read whole: what the thing is, who it suits, what to pay. Asked \"is it worth\n    it\", the first thing the person gets is a description, and the sentence that answers them is\n    the second one. STYLE.md puts the answer in the first sentence, so the sentence that opens\n    with a verdict is moved to the front and the rest follows in the order it was written.\n\n    Nothing is added or reworded, and the move is only made when the shape check can see that the\n    opening sentence is *not* a yes or no: `fit.gives(opening, \"yesno\", line) is False`. So a card\n    that already answers first is left alone, a card whose opening the check cannot read is left\n    alone, and a line that did not ask yes or no never comes here.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    if py_truthy(&Py::Bool(!py_eq(&_the_want(&[understood.clone()]), &pyrt::xrefs::xget("compose.fit", "YESNO")))) {
    return reply.clone();
    }
    parts = _sentences(&[reply.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return reply.clone();
    }
    bare_opening = mcall(&mcall(&geti(&parts.clone(), &Py::Int(0)), "rstrip", &[py_str(".!? ")]), "strip", &[]);
    if py_truthy(&mcall(&g__VERDICT_OPENS(), "match", &[bare_opening.clone()])) {
    return reply.clone();
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    place = __unpk2[0].clone();
    sentence = __unpk2[1].clone();
    bare = mcall(&mcall(&sentence.clone(), "rstrip", &[py_str(".!? ")]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__VERDICT_OPENS(), "match", &[bare.clone()]))) } }) {
    continue;
    }
    if py_truthy(&mcall(&g__THINKING_IT_OVER(), "search", &[bare.clone()])) {
    continue;
    }
    let __try3 = exc::catch(|| {
    if py_truthy(&Py::Bool(py_is(&pyrt::xrefs::xcall("compose.fit", "gives", &[bare.clone(), pyrt::xrefs::xget("compose.fit", "YESNO"), line.clone()]), &Py::Bool(false)))) {
    return py_tuple(vec![Py::Int(2), Py::None]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["Exception"]) {
                __handled7 = true;
    continue;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_eq(&__ret4, &Py::Int(2)) { continue; }
    if py_truthy(&__ret4) { return __val5; }
    rest = mcall(&py_str(" "), "join", &[op_add(&slice_or_raise(&parts.clone(), &Py::None, &place.clone(), &Py::None), &slice_or_raise(&parts.clone(), &op_add(&place.clone(), &Py::Int(1)), &Py::None, &Py::None))]);
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![sentence.clone(), rest.clone()]));
    }
    return reply.clone();
    Py::None
}
pub fn method_first(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut found: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The reply with its total time moved to the end when the line asked for the method.\n\n    Returns the reply unchanged for every other line, for a reply that is one sentence (there is\n    nothing to lead with), and for a reply whose opening sentence is about more than its total\n    time.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&_wants_the_method(&[understood.clone()])))) {
    return reply.clone();
    }
    found = mcall(&g__OPENING(), "match", &[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return reply.clone();
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]), mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "strip", &[])]), 2, None);
    opening = __unpk1[0].clone();
    rest = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__TOTAL(), "match", &[opening.clone()])))) {
    return reply.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return reply.clone();
    }
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![rest.clone(), opening.clone()]));
    Py::None
}
pub fn _wants_the_cause(args: &[Py]) -> Py {
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
    let mut want: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    want = mcall(&understood.clone(), "get", &[py_str("wants")]);
    if py_truthy(&call_builtin("isinstance", &[want.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    want = mcall(&mcall(&want.clone(), "strip", &[]), "lower", &[]);
    }
    return Py::Bool(py_eq(&want.clone(), &pyrt::xrefs::xget("compose.fit", "REASON")));
    Py::None
}
pub fn cause_first(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut index: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The cause sentence leads when the line asked why and the card puts it last.\n\n    A card is written for the screen and for every way its subject is asked about, so the fix\n    comes first and the cause closes the paragraph: \"Pull the hair and soap out of the trap first\n    ... The reason is hair, soap and grease caught in the trap and the bend.\" Read out to someone\n    who asked \"why is my drain slow\", that answers a different question for two sentences and\n    reaches their answer last.\n\n    So when `memory[\"understood\"][\"wants\"]` is `reason`, a sentence that names itself as the cause\n    is moved to the front. Nothing is added, cut or reworded - the same sentence moves - and a\n    reply whose first sentence is already a cause is left exactly as it is. Every other shape of\n    line is left alone: \"how do i unblock my drain\" wants the fix first whatever the card says\n    later, and `method_first` above owns the `steps` case.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&_wants_the_cause(&[understood.clone()])))) {
    return reply.clone();
    }
    parts = _sentences(&[reply.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return reply.clone();
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&mcall(&g__CAUSE_SENTENCE(), "match", &[geti(&parts.clone(), &index.clone())])) {
    moved = geti(&parts.clone(), &index.clone());
    rest = op_add(&slice_or_raise(&parts.clone(), &Py::None, &index.clone(), &Py::None), &slice_or_raise(&parts.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None));
    return mcall(&py_str(" "), "join", &[op_add(&py_list(vec![moved.clone()]), &rest.clone())]);
    }
    }
    return reply.clone();
    Py::None
}
pub fn _the_want(args: &[Py]) -> Py {
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
    let mut want: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    want = mcall(&understood.clone(), "get", &[py_str("wants")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[want.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    return mcall(&mcall(&want.clone(), "strip", &[]), "lower", &[]);
    Py::None
}
pub fn _the_opening(args: &[Py]) -> Py {
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
    return pyrt::xrefs::xcall("compose.fit", "first_sentence", &[reply.clone()]);
    Py::None
}
pub fn _already_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lead_said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lead_said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lead_said"))) };
    let mut opening: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opening"))) };
    let mut mine: Py = Py::None;
    let mut there: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the opening sentence already says the declared one, in its own words.");
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__SAID_WORD(), "findall", &[mcall(&lead_said.clone(), "lower", &[])])) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::Bool(py_eq(&mcall(&mcall(&lead_said.clone(), "lower", &[]), "strip", &[py_str(" .,!?")]), &mcall(&py_str(" "), "join", &[mcall(&g__SAID_WORD(), "findall", &[mcall(&opening.clone(), "lower", &[])])])));
    }
    there = call_builtin("set", &[mcall(&g__SAID_WORD(), "findall", &[mcall(&opening.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&there.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_own_sentence_holding(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut parts: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "parts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: parts"))) };
    let mut bare: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bare") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bare"))) };
    let mut at: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut part: Py = Py::None;
    let mut there: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place of the reply's own sentence that says the declared words, or None.\n\n    A row written for the harder question puts the plain answer in its second or third sentence\n    (\"... and it booms loudest when you accelerate. It is not an emergency, but fumes can get\n    into the cabin ...\"). `bare` is usually the opening of that sentence rather than all of it,\n    so the test is whether every word they share is in it - the sentence's own trailing detail\n    (\"but fumes can get into the cabin\") is carried along with it.\n    ");
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__SAID_WORD(), "findall", &[mcall(&bare.clone(), "lower", &[])])) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    part = __unpk2[1].clone();
    there = call_builtin("set", &[mcall(&g__SAID_WORD(), "findall", &[mcall(&part.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&there.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return at.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn the_sentence_owed(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut declared: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "declared") { v.clone() }
    else { Py::None };
    let mut at: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut line: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("The sentence the line asked for, put in front, when the row declares one for its shape.\n\n    A row holds one answer for every way its subject is asked about, and it is written for the\n    harder question: the side-effects card was written for \"my medication is making me feel odd\"\n    and answers \"are these side effects normal\" with a paragraph about the leaflet that reaches\n    the verdict only in its third clause. STYLE.md asks for the answer in the first sentence, so\n    a row declares the sentence it owes and the shape of line it owes it to -\n    `entry(..., lead=(\"yesno\", \"Usually, yes\"))`, or a callable in place of the sentence for a row\n    whose key is asked from both sides (\"No\" for \"is a cold the same as the flu\", \"\" for \"can i\n    get flu twice\") - and this puts it in front.\n\n    What decides is the line, not a reading of the card: the line's `wants` has to be the shape\n    the row declared, and only then does the sentence go in front. So a line that wants a time is\n    left alone by a row whose sentence is a verdict, a card that already opens on the same words\n    is left exactly as it was, and a sentence the shape check can read as another shape is never\n    used. Nothing is reworded: the sentence is the row's own words.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[declared.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("len", &[declared.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) } }) {
    return reply.clone();
    }
    let __unpk1 = py_unpack(&declared.clone(), 2, None);
    shape = __unpk1[0].clone();
    said = __unpk1[1].clone();
    shape = if py_truthy(&call_builtin("isinstance", &[shape.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&mcall(&shape.clone(), "strip", &[]), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__HARD(), &shape.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&_the_want(&[understood.clone()]), &shape.clone())) } }) {
    return reply.clone();
    }
    if py_truthy(&call_builtin("callable", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __try2 = exc::catch(|| {
    said = py_call(&said, &[understood.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    return reply.clone();
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    bare = mcall(&mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]), "rstrip", &[py_str(" .!?")]);
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return reply.clone();
    }
    opening = _the_opening(&[reply.clone()]);
    if py_truthy(&mcall(&mcall(&mcall(&reply.clone(), "lstrip", &[]), "lower", &[]), "startswith", &[mcall(&bare.clone(), "lower", &[])])) {
    return reply.clone();
    }
    if py_truthy(&_already_said(&[bare.clone(), opening.clone()])) {
    return reply.clone();
    }
    line = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("text")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    line = { let __b = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("clean")]) } else { py_str("") }; if py_truthy(&__b) { __b } else { py_str("") } };
    }
    let __try7 = exc::catch(|| {
    if py_truthy(&Py::Bool(py_is(&pyrt::xrefs::xcall("compose.fit", "gives", &[bare.clone(), shape.clone(), line.clone()]), &Py::Bool(false)))) {
    return py_tuple(vec![Py::Bool(true), reply.clone()]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["Exception"]) {
                __handled11 = true;
    return reply.clone();
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    parts = _sentences(&[reply.clone()]);
    at = _the_own_sentence_holding(&[parts.clone(), bare.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&at.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_gt(&at.clone(), &Py::Int(0)) } }) {
    return mcall(&py_str(" "), "join", &[op_add(&slice_or_raise(&parts.clone(), &at.clone(), &Py::None, &Py::None), &slice_or_raise(&parts.clone(), &Py::None, &at.clone(), &Py::None))]);
    }
    return op_modulo(&py_str("%s. %s"), &py_tuple(vec![bare.clone(), mcall(&reply.clone(), "lstrip", &[])]));
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
    let mut word: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]{4,}"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__SHARED_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _names_the_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut asked: Py = Py::None;
    let _ = py_str("True when the sentence shares a word with the line that asked.");
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(true);
    }
    asked = _words(&[line.clone()]);
    return call_builtin("bool", &[op_bitand(&asked.clone(), &_words(&[sentence.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _sentence_stands_alone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut text: Py = Py::None;
    let _ = py_str("True when a sentence reads as an answer with nothing before it.");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__POINTS_BACKWARDS(), "match", &[text.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&text.clone(), "endswith", &[py_str("?")])));
    Py::None
}
pub fn shape(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut index: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut want: Py = Py::None;
    let mut when_asked: Py = Py::None;
    let _ = py_str("The reply with the sentence that answers the wanted kind of question moved to the front.\n\n    `understood` is the turn's reading, whose `wants` names the kind of answer the line asked for\n    (`bot/compose/fit.py`). A method asks for its own move (`method_first`); a time, an amount or a\n    place lifts the first sentence that gives that kind of answer, and only when the reply's\n    opening does not already give it. A yes or no and a pick are left alone: their checks cannot\n    be sure enough that a sentence is the answer rather than a remark that mentions one.\n    Everything else is returned unchanged, and a sentence is never reworded, only moved.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return reply.clone();
    }
    want = mcall(&understood.clone(), "get", &[py_str("wants")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[want.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return reply.clone();
    }
    want = mcall(&mcall(&want.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g__METHOD(), &want.clone()))) {
    return method_first(&[reply.clone(), understood.clone()]);
    }
    if py_truthy(&Py::Bool(!py_contains(&g__LIFTABLE(), &want.clone()))) {
    return reply.clone();
    }
    parts = _sentences(&[reply.clone()]);
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return reply.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pyrt::xrefs::xcall("compose.fit", "gives", &[geti(&parts.clone(), &Py::Int(0)), want.clone(), line.clone()]), &Py::Bool(true))); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fit", "is_a_punt", &[geti(&parts.clone(), &Py::Int(0))]) } }) {
    return reply.clone();
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\d"), geti(&parts.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _names_the_line(&[geti(&parts.clone(), &Py::Int(0)), line.clone()]) } }) {
    return reply.clone();
    }
    if py_truthy(&mcall(&g__OPENS_WITH_SYMPATHY(), "search", &[geti(&parts.clone(), &Py::Int(0))])) {
    return reply.clone();
    }
    if py_truthy(&mcall(&g__OPENS_WITH_A_WARNING(), "search", &[geti(&parts.clone(), &Py::Int(0))])) {
    return reply.clone();
    }
    when_asked = { let __b = call_builtin("bool", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&stdlib_call("re", "search", &[py_str("\\bwhen\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::None)) } };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    sentence = geti(&parts.clone(), &index.clone());
    if py_truthy(&{ let __b = when_asked.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pyrt::xrefs::xget("compose.fit", "CLOCK"), "search", &[sentence.clone()]))) } }) {
    continue;
    }
    if py_truthy(&mcall(&g__OPENS_WITH_A_WARNING(), "search", &[sentence.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_sentence_stands_alone(&[sentence.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![pyrt::xrefs::xget("compose.fit", "TIME"), pyrt::xrefs::xget("compose.fit", "AMOUNT")]), &want.clone())); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("compose.fit", "_a_time", &[sentence.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.fit", "_an_amount", &[sentence.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_is(&pyrt::xrefs::xcall("compose.fit", "gives", &[sentence.clone(), want.clone(), line.clone()]), &Py::Bool(true)))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_the_line(&[sentence.clone(), line.clone()])))) {
    continue;
    }
    moved = op_add(&op_add(&py_list(vec![sentence.clone()]), &slice_or_raise(&parts.clone(), &Py::None, &index.clone(), &Py::None)), &slice_or_raise(&parts.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None));
    return mcall(&py_str(" "), "join", &[moved.clone()]);
    }
    return reply.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.lead".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "_ABBREVIATION_END" => Some(g__ABBREVIATION_END()),
            "_ASKS_HOW_OFTEN" => Some(g__ASKS_HOW_OFTEN()),
            "_A_RATE" => Some(g__A_RATE()),
            "_CAUSE_SENTENCE" => Some(g__CAUSE_SENTENCE()),
            "_HARD" => Some(g__HARD()),
            "_LIFTABLE" => Some(g__LIFTABLE()),
            "_METHOD" => Some(g__METHOD()),
            "_OPENING" => Some(g__OPENING()),
            "_OPENS_WITH_A_WARNING" => Some(g__OPENS_WITH_A_WARNING()),
            "_OPENS_WITH_SYMPATHY" => Some(g__OPENS_WITH_SYMPATHY()),
            "_POINTS_BACKWARDS" => Some(g__POINTS_BACKWARDS()),
            "_SAID_WORD" => Some(g__SAID_WORD()),
            "_SENTENCE_END" => Some(g__SENTENCE_END()),
            "_SHARED_WORDS" => Some(g__SHARED_WORDS()),
            "_SIGNPOST" => Some(g__SIGNPOST()),
            "_THINKING_IT_OVER" => Some(g__THINKING_IT_OVER()),
            "_TOTAL" => Some(g__TOTAL()),
            "_VERDICT_OPENS" => Some(g__VERDICT_OPENS()),
            "_already_said" => Some(py_func("_already_said", |args: &[Py]| _already_said(args))),
            "_names_the_line" => Some(py_func("_names_the_line", |args: &[Py]| _names_the_line(args))),
            "_sentence_stands_alone" => Some(py_func("_sentence_stands_alone", |args: &[Py]| _sentence_stands_alone(args))),
            "_sentences" => Some(py_func("_sentences", |args: &[Py]| _sentences(args))),
            "_the_opening" => Some(py_func("_the_opening", |args: &[Py]| _the_opening(args))),
            "_the_own_sentence_holding" => Some(py_func("_the_own_sentence_holding", |args: &[Py]| _the_own_sentence_holding(args))),
            "_the_want" => Some(py_func("_the_want", |args: &[Py]| _the_want(args))),
            "_wants_the_cause" => Some(py_func("_wants_the_cause", |args: &[Py]| _wants_the_cause(args))),
            "_wants_the_method" => Some(py_func("_wants_the_method", |args: &[Py]| _wants_the_method(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "answer_first" => Some(py_func("answer_first", |args: &[Py]| answer_first(args))),
            "cause_first" => Some(py_func("cause_first", |args: &[Py]| cause_first(args))),
            "method_first" => Some(py_func("method_first", |args: &[Py]| method_first(args))),
            "rate_first" => Some(py_func("rate_first", |args: &[Py]| rate_first(args))),
            "shape" => Some(py_func("shape", |args: &[Py]| shape(args))),
            "the_sentence_owed" => Some(py_func("the_sentence_owed", |args: &[Py]| the_sentence_owed(args))),
            "verdict_first" => Some(py_func("verdict_first", |args: &[Py]| verdict_first(args))),
            "fit" => Some(pyrt::xrefs::xmodule("compose.fit")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "_ABBREVIATION_END" => s__ABBREVIATION_END(v),
            "_ASKS_HOW_OFTEN" => s__ASKS_HOW_OFTEN(v),
            "_A_RATE" => s__A_RATE(v),
            "_CAUSE_SENTENCE" => s__CAUSE_SENTENCE(v),
            "_HARD" => s__HARD(v),
            "_LIFTABLE" => s__LIFTABLE(v),
            "_METHOD" => s__METHOD(v),
            "_OPENING" => s__OPENING(v),
            "_OPENS_WITH_A_WARNING" => s__OPENS_WITH_A_WARNING(v),
            "_OPENS_WITH_SYMPATHY" => s__OPENS_WITH_SYMPATHY(v),
            "_POINTS_BACKWARDS" => s__POINTS_BACKWARDS(v),
            "_SAID_WORD" => s__SAID_WORD(v),
            "_SENTENCE_END" => s__SENTENCE_END(v),
            "_SHARED_WORDS" => s__SHARED_WORDS(v),
            "_SIGNPOST" => s__SIGNPOST(v),
            "_THINKING_IT_OVER" => s__THINKING_IT_OVER(v),
            "_TOTAL" => s__TOTAL(v),
            "_VERDICT_OPENS" => s__VERDICT_OPENS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.lead", module_obj);
    pyrt::xrefs::reg_fn("compose.lead", "_sentences", py_func("_sentences", |args: &[Py]| _sentences(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_wants_the_method", py_func("_wants_the_method", |args: &[Py]| _wants_the_method(args)));
    pyrt::xrefs::reg_fn("compose.lead", "rate_first", py_func("rate_first", |args: &[Py]| rate_first(args)));
    pyrt::xrefs::reg_fn("compose.lead", "answer_first", py_func("answer_first", |args: &[Py]| answer_first(args)));
    pyrt::xrefs::reg_fn("compose.lead", "verdict_first", py_func("verdict_first", |args: &[Py]| verdict_first(args)));
    pyrt::xrefs::reg_fn("compose.lead", "method_first", py_func("method_first", |args: &[Py]| method_first(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_wants_the_cause", py_func("_wants_the_cause", |args: &[Py]| _wants_the_cause(args)));
    pyrt::xrefs::reg_fn("compose.lead", "cause_first", py_func("cause_first", |args: &[Py]| cause_first(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_the_want", py_func("_the_want", |args: &[Py]| _the_want(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_the_opening", py_func("_the_opening", |args: &[Py]| _the_opening(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_already_said", py_func("_already_said", |args: &[Py]| _already_said(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_the_own_sentence_holding", py_func("_the_own_sentence_holding", |args: &[Py]| _the_own_sentence_holding(args)));
    pyrt::xrefs::reg_fn("compose.lead", "the_sentence_owed", py_func("the_sentence_owed", |args: &[Py]| the_sentence_owed(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_names_the_line", py_func("_names_the_line", |args: &[Py]| _names_the_line(args)));
    pyrt::xrefs::reg_fn("compose.lead", "_sentence_stands_alone", py_func("_sentence_stands_alone", |args: &[Py]| _sentence_stands_alone(args)));
    pyrt::xrefs::reg_fn("compose.lead", "shape", py_func("shape", |args: &[Py]| shape(args)));
    pyrt::xrefs::reg_access("compose.lead", "_ABBREVIATION_END", g__ABBREVIATION_END, s__ABBREVIATION_END);
    pyrt::xrefs::reg_access("compose.lead", "_ASKS_HOW_OFTEN", g__ASKS_HOW_OFTEN, s__ASKS_HOW_OFTEN);
    pyrt::xrefs::reg_access("compose.lead", "_A_RATE", g__A_RATE, s__A_RATE);
    pyrt::xrefs::reg_access("compose.lead", "_CAUSE_SENTENCE", g__CAUSE_SENTENCE, s__CAUSE_SENTENCE);
    pyrt::xrefs::reg_access("compose.lead", "_HARD", g__HARD, s__HARD);
    pyrt::xrefs::reg_access("compose.lead", "_LIFTABLE", g__LIFTABLE, s__LIFTABLE);
    pyrt::xrefs::reg_access("compose.lead", "_METHOD", g__METHOD, s__METHOD);
    pyrt::xrefs::reg_access("compose.lead", "_OPENING", g__OPENING, s__OPENING);
    pyrt::xrefs::reg_access("compose.lead", "_OPENS_WITH_A_WARNING", g__OPENS_WITH_A_WARNING, s__OPENS_WITH_A_WARNING);
    pyrt::xrefs::reg_access("compose.lead", "_OPENS_WITH_SYMPATHY", g__OPENS_WITH_SYMPATHY, s__OPENS_WITH_SYMPATHY);
    pyrt::xrefs::reg_access("compose.lead", "_POINTS_BACKWARDS", g__POINTS_BACKWARDS, s__POINTS_BACKWARDS);
    pyrt::xrefs::reg_access("compose.lead", "_SAID_WORD", g__SAID_WORD, s__SAID_WORD);
    pyrt::xrefs::reg_access("compose.lead", "_SENTENCE_END", g__SENTENCE_END, s__SENTENCE_END);
    pyrt::xrefs::reg_access("compose.lead", "_SHARED_WORDS", g__SHARED_WORDS, s__SHARED_WORDS);
    pyrt::xrefs::reg_access("compose.lead", "_SIGNPOST", g__SIGNPOST, s__SIGNPOST);
    pyrt::xrefs::reg_access("compose.lead", "_THINKING_IT_OVER", g__THINKING_IT_OVER, s__THINKING_IT_OVER);
    pyrt::xrefs::reg_access("compose.lead", "_TOTAL", g__TOTAL, s__TOTAL);
    pyrt::xrefs::reg_access("compose.lead", "_VERDICT_OPENS", g__VERDICT_OPENS, s__VERDICT_OPENS);
}
