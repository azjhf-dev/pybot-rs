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


thread_local! { static G_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DANGLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINISHED_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FORBADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TENSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_DID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.carried_on: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_QUESTION() -> Py { ensure_init(); G_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION(v: Py) { ensure_init(); G_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE() -> Py { ensure_init(); G_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE(v: Py) { ensure_init(); G_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_DANGLING() -> Py { ensure_init(); G_DANGLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DANGLING(v: Py) { ensure_init(); G_DANGLING.with(|c| *c.borrow_mut() = v); }
pub fn g_FINISHED_IT() -> Py { ensure_init(); G_FINISHED_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINISHED_IT(v: Py) { ensure_init(); G_FINISHED_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_FORBADE() -> Py { ensure_init(); G_FORBADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FORBADE(v: Py) { ensure_init(); G_FORBADE.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME() -> Py { ensure_init(); G_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME(v: Py) { ensure_init(); G_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO() -> Py { ensure_init(); G_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO(v: Py) { ensure_init(); G_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINING() -> Py { ensure_init(); G_JOINING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINING(v: Py) { ensure_init(); G_JOINING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_STEP() -> Py { ensure_init(); G_NOT_A_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STEP(v: Py) { ensure_init(); G_NOT_A_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALIFIER() -> Py { ensure_init(); G_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALIFIER(v: Py) { ensure_init(); G_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT() -> Py { ensure_init(); G_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); G_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT() -> Py { ensure_init(); G_SOFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT(v: Py) { ensure_init(); G_SOFT.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_WORDS() -> Py { ensure_init(); G_STEP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_WORDS(v: Py) { ensure_init(); G_STEP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TENSE() -> Py { ensure_init(); G_TENSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TENSE(v: Py) { ensure_init(); G_TENSE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_DID() -> Py { ensure_init(); G_THEY_DID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_DID(v: Py) { ensure_init(); G_THEY_DID.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB() -> Py { ensure_init(); G_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB(v: Py) { ensure_init(); G_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that says which step of the last answer they have done gets the step after it.\n\n\"how do i get red wine out of a cream carpet\" / \"it happened last night and i blotted it\" was\nanswered \"Thanks for telling me - I'm here if you want to dig into any of that\", which is the one\nreply a follow-up must never get. The person has not asked anything new: they have told the bot\nwhere they are in the job, and the useful thing a friend says next is the next bit of the job -\nin the words the answer already used, because those words are the ones they are following.\n\n    how do i get red wine out of a cream carpet\n      Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, ...\n    it happened last night and i blotted it\n      Good - blotting first is right. Now pour on cold water and blot again.\n\nThe reading is deliberately narrow, because the same words after a different kind of answer are\nnot a report on it:\n\n* the turn before must have been an answer from `answers` that gave instructions, and the line\n  before it must have been a how-to ask (\"how do i\", \"how to\", \"what's the best way\");\n* the line must be short and carry no question of its own, so \"i blotted it, will that be\n  enough?\" is still a question and is answered as one;\n* the person has to be in the line, saying what they did (\"i blotted it\", \"we gave it a go\"): \"it\n  boiled over\" and \"it happened last night\" report the thing, not a step of theirs;\n* it must name the step, in one of the two ways a person does that: an action in the past\n  (\"blotted\", \"scrubbed\") that the answer used, or a thing they finished with a completion verb\n  in front of it (\"i did the bicarbonate bit\", \"i used the vinegar\"). The words are matched\n  against the answer, never a list of jobs here;\n* it must not be a different kind of line wearing the same words - a lack (\"i dont have any\n  vinegar\"), a mishap (\"i boiled it dry\", \"it boiled over\"), or a feeling (\"i hate this kettle\")\n  is something to answer, never praise for a step nobody took;\n* the clause the action came from must not be one the answer forbade (\"don't rub\"), so a person\n  who rubbed it is not told they did the right thing;\n* something must come after that step in the answer, and it must be an instruction rather than\n  the other half of a pair (\"or a sachet of citric acid\" belongs to the step before it). If they\n  have done the last one, this has nothing to say and passes the turn back.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.WORD"); }
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.HOW_TO"); }
    s_HOW_TO(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+(?:do|can|should|could)\\s+(?:i|we|you)\\b|how\\s+to\\b|what'?s\\s+the\\s+best\\s+way\\b|whats\\s+the\\s+best\\s+way\\b|best\\s+way\\s+to\\b|any\\s+tips\\b|how\\s+would\\s+you\\b|what\\s+should\\s+i\\s+use\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.A_QUESTION"); }
    s_A_QUESTION(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:what|why|how|when|where|who|whom|whose|which|will|would|should|could|can|shall|may|might)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.FRAME"); }
    s_FRAME(call_builtin("frozenset", &[mcall(&py_str("\n    it its it's that this them they those these there here\n    i we he she you ive i've weve we've id i'd\n    the a an and then so but also just already now still only even\n    happened happen went come came got get had has have was were is are been being\n    last night morning afternoon evening today yesterday week weekend month year\n    ago later earlier since about around nearly almost bit thing one ones all\n    on in at to of with out up down off back through over again just finally first\n    tried try did done doing do does\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.SHORT"); }
    s_SHORT(Py::Int(14));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.STEP_WORDS"); }
    s_STEP_WORDS(Py::Int(22));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.DANGLING"); }
    s_DANGLING(call_builtin("frozenset", &[mcall(&py_str("a an the and or but nor so yet for with without to of in on at by from\nas if when while because which that than then also plus into onto over under about after before\nduring up down out off back through it its their his her your my our this these those there\nhere"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.FORBADE"); }
    s_FORBADE(stdlib_call("re", "compile", &[py_str("^\\s*(?:don'?t|do not|never|avoid|not)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.CLAUSE"); }
    s_CLAUSE(stdlib_call("re", "compile", &[py_str(";|,\\s*(?:then\\s+)?|(?<=\\.)\\s+|\\s+then\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.THEY_DID"); }
    s_THEY_DID(stdlib_call("re", "compile", &[py_str("\\b(?:i|i've|ive|i'd|id|we|we've|weve|we'd|wed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.FINISHED_IT"); }
    s_FINISHED_IT(stdlib_call("re", "compile", &[py_str("\\b(?:i|i've|ive|i'd|id|we|we've|weve|we'd|wed)\\s+(?:already\\s+|just\\s+|now\\s+|only\\s+)?(?:did|done|used|tried|gave|had|finished|started)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.NOT_A_STEP"); }
    s_NOT_A_STEP(stdlib_call("re", "compile", &[py_str("\\b(?:dont|don't|doesnt|doesn't|didnt|didn't|havent|haven't|cant|can't|couldnt|couldn't|wont|won't|ran out|run out|no|none|nothing|dry|over|broke|broken|breaks|burns?|burnt|burned|spilt|spilled|ruined|stuck|stopped|hate|hates|fed up|sick of|gave up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.SOFT"); }
    s_SOFT(call_builtin("frozenset", &[mcall(&py_str("\n    leave left wait keep let give make use put take check see look wipe soak boil\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.TENSE"); }
    s_TENSE(stdlib_call("re", "compile", &[py_str("(?:ed|ing|s|d)$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.POINTS_BACK"); }
    s_POINTS_BACK(call_builtin("frozenset", &[mcall(&py_str("above below under over that this these those it them there here\nsame other one ones already again first last next then now"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.QUALIFIER"); }
    s_QUALIFIER(stdlib_call("re", "compile", &[py_str("^(?:don'?t|do not|never|avoid|not|which|that|with|about|for|in|on|until|repeating|again|and repeat|using|leave it|keep|or|nor|as|at|by)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.VERB"); }
    s_VERB(stdlib_call("re", "compile", &[py_str("\\b[a-z]{3,}(?:e|ed|ing|s)?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carried_on.JOINING"); }
    s_JOINING(call_builtin("frozenset", &[mcall(&py_str("\n    and or but nor so yet plus then also next\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_whole_step(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The step, cut to STEP_WORDS words without cutting a sentence in the middle.\n\n    The cut lands on a clause boundary when the step has one (a colon, a semicolon or a comma in\n    the first STEP_WORDS words), which keeps a whole instruction; otherwise it walks back to the\n    last word that could end a sentence. The caller's frame supplies the full stop.\n    ");
    words = mcall(&said.clone(), "split", &[]);
    if py_truthy(&op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS())) {
    return mcall(&said.clone(), "strip", &[py_str(" ,;:")]);
    }
    words = slice_or_raise(&words.clone(), &Py::None, &g_STEP_WORDS(), &Py::None);
    let mut __broke1 = false;
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), Py::Int(2), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    at = __item.clone();
    if py_truthy(&mcall(&geti(&words.clone(), &op_sub(&at.clone(), &Py::Int(1))), "endswith", &[py_tuple(vec![py_str(":"), py_str(";"), py_str(",")])])) {
    words = slice_or_raise(&words.clone(), &Py::None, &at.clone(), &Py::None);
    let _ = py_setitem(&words.clone(), &op_sub(&at.clone(), &Py::Int(1)), mcall(&geti(&words.clone(), &op_sub(&at.clone(), &Py::Int(1))), "rstrip", &[py_str(":;,")]));
    __broke1 = true;
    break;
    }
    }
    if !__broke1 {
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DANGLING(), &mcall(&mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "lower", &[]), "strip", &[py_str(",;:")]))); if py_truthy(&__b) { __b } else { mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "endswith", &[py_tuple(vec![py_str(","), py_str(":"), py_str(";")])]) } } } }) {
    words = slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    }
    return mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" ,;:")]);
    Py::None
}
pub fn _stem(args: &[Py]) -> Py {
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
    let _ = py_str("\"blotted\" -> \"blot\", \"pouring\" -> \"pour\", \"boils\" -> \"boil\".");
    word = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ied")]) } }) {
    word = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    }
    word = mcall(&g_TENSE(), "sub", &[py_str(""), word.clone()]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&word.clone(), &op_neg(&Py::Int(1))), &geti(&word.clone(), &op_neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("lsz"), &geti(&word.clone(), &op_neg(&Py::Int(1))))) } } } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
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
    let mut w: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])])) { let mut w: Py = __item.clone(); __out.push(w.clone()); } py_list(__out) })() };
    Py::None
}
pub fn _steps(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut out: Py = Py::None;
    let mut part: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The answer cut at its clause breaks, which is where one instruction stops and the next\n    starts: \"blot it, don't rub, with kitchen paper, then pour on cold water and blot again\".");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_CLAUSE(), "split", &[call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    part = __item.clone();
    part = mcall(&part.clone(), "strip", &[py_str(" .;,-")]);
    words = mcall(&part.clone(), "split", &[]);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_JOINING(), &mcall(&mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "lower", &[]), "strip", &[py_str(" ,;:")]))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    part = mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" ,;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[part.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _doing(args: &[Py]) -> Py {
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
    let mut base: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("\"blotted\" -> \"blotting\", so the lead-in reads like a person wrote it.");
    low = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return py_str("");
    }
    if py_truthy(&mcall(&low.clone(), "endswith", &[py_str("ing")])) {
    return low.clone();
    }
    if py_truthy(&{ let __b = mcall(&low.clone(), "endswith", &[py_str("ied")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    base = op_add(&slice_or_raise(&low.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    } else {
    if py_truthy(&{ let __b = mcall(&low.clone(), "endswith", &[py_str("ed")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    base = slice_or_raise(&low.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    } else {
    if py_truthy(&{ let __b = mcall(&low.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    base = slice_or_raise(&low.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    } else {
    base = low.clone();
    }
    }
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[base.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&base.clone(), &op_neg(&Py::Int(1))), &geti(&base.clone(), &op_neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("lsz"), &geti(&base.clone(), &op_neg(&Py::Int(1))))) } } } }) {
    return op_add(&base.clone(), &py_str("ing"));
    }
    if py_truthy(&{ let __b = mcall(&base.clone(), "endswith", &[py_str("e")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[base.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return op_add(&slice_or_raise(&base.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing"));
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[base.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&py_str("aeiouwxy"), &geti(&base.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_str("aeiou"), &geti(&base.clone(), &op_neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&base.clone(), &op_neg(&Py::Int(3))))) } } } } } }) {
    return op_add(&op_add(&base.clone(), &geti(&base.clone(), &op_neg(&Py::Int(1)))), &py_str("ing"));
    }
    return op_add(&base.clone(), &py_str("ing"));
    Py::None
}
pub fn _the_action(args: &[Py]) -> Py {
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
    let mut stem: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The action the person says they have done, as a stem, or None.\n\n    \"it happened last night and i blotted it\" -> \"blot\". The named thing has to be a verb in the\n    past that is not one of the words the frame around it is made of.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[line.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FRAME(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TENSE(), "search", &[word.clone()]))) } }) {
    continue;
    }
    stem = _stem(&[word.clone()]);
    if py_truthy(&{ let __b = stem.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FRAME(), &stem.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return stem.clone();
    }
    }
    return Py::None;
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
    let mut action: Py = Py::None;
    let mut after: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut at: Py = Py::None;
    let mut done: Py = Py::None;
    let mut first: Py = Py::None;
    let mut first_word: Py = Py::None;
    let mut last: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_word: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shapes: Py = Py::None;
    let mut step: Py = Py::None;
    let mut steps: Py = Py::None;
    let mut t: Py = Py::None;
    let mut things: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut w: Py = Py::None;
    let mut which: Py = Py::None;
    let _ = py_str("The next step of the last answer, when the line says which step they are on.");
    line = mcall(&py_str(" "), "join", &[_words(&[text.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT()) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_QUESTION(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("answers")))) {
    return Py::None;
    }
    asked = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_HOW_TO(), "search", &[asked.clone()])))) {
    return Py::None;
    }
    steps = _steps(&[mcall(&last.clone(), "get", &[py_str("bot")])]);
    if py_truthy(&op_lt(&call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THEY_DID(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_A_STEP(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    action = _the_action(&[text.clone()]);
    things = if py_truthy(&mcall(&g_FINISHED_IT(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[text.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FRAME(), &w.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {  __out.push(_stem(&[w.clone()])); } } py_set(__out) })() } } else { call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&action.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&things.clone())) } }) {
    return Py::None;
    }
    done = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    step = __unpk2[1].clone();
    if py_truthy(&mcall(&g_FORBADE(), "search", &[step.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = action.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[step.clone()])) { let mut w: Py = __item.clone(); __out.push(_stem(&[w.clone()])); } py_set(__out) })() }, &action.clone())) } }) {
    done = at.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&done.clone(), &Py::None))) {
    let __iter3 = py_iter_or_raise(&call_builtin("enumerate", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    at = __unpk4[0].clone();
    step = __unpk4[1].clone();
    if py_truthy(&mcall(&g_FORBADE(), "search", &[step.clone()])) {
    continue;
    }
    if py_truthy(&op_bitand(&things.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[step.clone()])) { let mut w: Py = __item.clone(); __out.push(_stem(&[w.clone()])); } py_set(__out) })() })) {
    done = at.clone();
    break;
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&done.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_ge(&done.clone(), &op_sub(&call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    return Py::None;
    }
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&steps.clone(), &op_add(&done.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut step: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_QUALIFIER(), "search", &[step.clone()])))) {  __out.push(step.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return Py::None;
    }
    said = geti(&after.clone(), &Py::Int(0));
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|then|but|so|also|plus|next)\\s+")); __args.push(py_str("")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[said.clone()])) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTS_BACK(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    said = _a_whole_step(&[said.clone()]);
    first_word = geti(&mcall(&said.clone(), "split", &[]), &Py::Int(0));
    named_word = { let __b = mcall(&first_word.clone(), "isupper", &[]); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[first_word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&first_word.clone(), &py_str("I"))) } } } };
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&named_word.clone())) } }) {
    said = op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    first = slice_or_raise(&_words(&[geti(&steps.clone(), &done.clone())]), &Py::None, &Py::Int(1), &Py::None);
    named = py_str("");
    if py_truthy(&{ let __b = action.clone(); if !py_truthy(&__b) { __b } else { { let __b = first.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_stem(&[geti(&first.clone(), &Py::Int(0))]), &action.clone())) } } } }) {
    named = _doing(&[action.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("do"), py_str("doing"), py_str("go"), py_str("going"), py_str("did")]), &named.clone()))) {
    named = py_str("");
    }
    shapes = py_tuple(vec![py_str("Good - {doing} first is right. Now {step}."), py_str("Right - {doing} is the first bit done. Now {step}."), py_str("That was the right first move. Next, {step}.")]);
    which = if py_truthy(&named.clone()) { geti(&shapes.clone(), &op_modulo(&call_builtin("sum", &[call_builtin("map", &[py_func("ord", |args: &[Py]| call_builtin("ord", args).unwrap_or_else(|e| exc::raise(e))), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[shapes.clone()]).unwrap_or_else(|e| exc::raise(e)))) } else { geti(&shapes.clone(), &Py::Int(2)) };
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("doing".to_string(), named.clone()), ("step".to_string(), said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&which.clone(), "format", args))(&__args) };
    lead = mcall(&lead.clone(), "replace", &[py_str("Now then "), py_str("Now ")]);
    return lead.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.carried_on".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_QUESTION" => Some(g_A_QUESTION()),
            "CLAUSE" => Some(g_CLAUSE()),
            "DANGLING" => Some(g_DANGLING()),
            "FINISHED_IT" => Some(g_FINISHED_IT()),
            "FORBADE" => Some(g_FORBADE()),
            "FRAME" => Some(g_FRAME()),
            "HOW_TO" => Some(g_HOW_TO()),
            "JOINING" => Some(g_JOINING()),
            "NOT_A_STEP" => Some(g_NOT_A_STEP()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "QUALIFIER" => Some(g_QUALIFIER()),
            "SHORT" => Some(g_SHORT()),
            "SOFT" => Some(g_SOFT()),
            "STEP_WORDS" => Some(g_STEP_WORDS()),
            "TENSE" => Some(g_TENSE()),
            "THEY_DID" => Some(g_THEY_DID()),
            "VERB" => Some(g_VERB()),
            "WORD" => Some(g_WORD()),
            "_a_whole_step" => Some(py_func("_a_whole_step", |args: &[Py]| _a_whole_step(args))),
            "_doing" => Some(py_func("_doing", |args: &[Py]| _doing(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_steps" => Some(py_func("_steps", |args: &[Py]| _steps(args))),
            "_the_action" => Some(py_func("_the_action", |args: &[Py]| _the_action(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "tried" => Some(pyrt::xrefs::xmodule("memory.tried")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_QUESTION" => s_A_QUESTION(v),
            "CLAUSE" => s_CLAUSE(v),
            "DANGLING" => s_DANGLING(v),
            "FINISHED_IT" => s_FINISHED_IT(v),
            "FORBADE" => s_FORBADE(v),
            "FRAME" => s_FRAME(v),
            "HOW_TO" => s_HOW_TO(v),
            "JOINING" => s_JOINING(v),
            "NOT_A_STEP" => s_NOT_A_STEP(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "QUALIFIER" => s_QUALIFIER(v),
            "SHORT" => s_SHORT(v),
            "SOFT" => s_SOFT(v),
            "STEP_WORDS" => s_STEP_WORDS(v),
            "TENSE" => s_TENSE(v),
            "THEY_DID" => s_THEY_DID(v),
            "VERB" => s_VERB(v),
            "WORD" => s_WORD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.carried_on", module_obj);
    pyrt::xrefs::reg_fn("memory.carried_on", "_a_whole_step", py_func("_a_whole_step", |args: &[Py]| _a_whole_step(args)));
    pyrt::xrefs::reg_fn("memory.carried_on", "_stem", py_func("_stem", |args: &[Py]| _stem(args)));
    pyrt::xrefs::reg_fn("memory.carried_on", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.carried_on", "_steps", py_func("_steps", |args: &[Py]| _steps(args)));
    pyrt::xrefs::reg_fn("memory.carried_on", "_doing", py_func("_doing", |args: &[Py]| _doing(args)));
    pyrt::xrefs::reg_fn("memory.carried_on", "_the_action", py_func("_the_action", |args: &[Py]| _the_action(args)));
    pyrt::xrefs::reg_fn("memory.carried_on", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.carried_on", "A_QUESTION", g_A_QUESTION, s_A_QUESTION);
    pyrt::xrefs::reg_access("memory.carried_on", "CLAUSE", g_CLAUSE, s_CLAUSE);
    pyrt::xrefs::reg_access("memory.carried_on", "DANGLING", g_DANGLING, s_DANGLING);
    pyrt::xrefs::reg_access("memory.carried_on", "FINISHED_IT", g_FINISHED_IT, s_FINISHED_IT);
    pyrt::xrefs::reg_access("memory.carried_on", "FORBADE", g_FORBADE, s_FORBADE);
    pyrt::xrefs::reg_access("memory.carried_on", "FRAME", g_FRAME, s_FRAME);
    pyrt::xrefs::reg_access("memory.carried_on", "HOW_TO", g_HOW_TO, s_HOW_TO);
    pyrt::xrefs::reg_access("memory.carried_on", "JOINING", g_JOINING, s_JOINING);
    pyrt::xrefs::reg_access("memory.carried_on", "NOT_A_STEP", g_NOT_A_STEP, s_NOT_A_STEP);
    pyrt::xrefs::reg_access("memory.carried_on", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("memory.carried_on", "QUALIFIER", g_QUALIFIER, s_QUALIFIER);
    pyrt::xrefs::reg_access("memory.carried_on", "SHORT", g_SHORT, s_SHORT);
    pyrt::xrefs::reg_access("memory.carried_on", "SOFT", g_SOFT, s_SOFT);
    pyrt::xrefs::reg_access("memory.carried_on", "STEP_WORDS", g_STEP_WORDS, s_STEP_WORDS);
    pyrt::xrefs::reg_access("memory.carried_on", "TENSE", g_TENSE, s_TENSE);
    pyrt::xrefs::reg_access("memory.carried_on", "THEY_DID", g_THEY_DID, s_THEY_DID);
    pyrt::xrefs::reg_access("memory.carried_on", "VERB", g_VERB, s_VERB);
    pyrt::xrefs::reg_access("memory.carried_on", "WORD", g_WORD, s_WORD);
}
