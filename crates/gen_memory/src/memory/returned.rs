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


thread_local! { static G_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITIONAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTINUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADING_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEXT_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECURRENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECURRING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMIND_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETURNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_EVIDENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AGAIN() -> Py { ensure_init(); G_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGAIN(v: Py) { ensure_init(); G_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ASK() -> Py { ensure_init(); G_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); G_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITIONAL_ASK() -> Py { ensure_init(); G_CONDITIONAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL_ASK(v: Py) { ensure_init(); G_CONDITIONAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTINUES() -> Py { ensure_init(); G_CONTINUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTINUES(v: Py) { ensure_init(); G_CONTINUES.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADING_CONDITION() -> Py { ensure_init(); G_LEADING_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADING_CONDITION(v: Py) { ensure_init(); G_LEADING_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_NEXT_STEP() -> Py { ensure_init(); G_NEXT_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEXT_STEP(v: Py) { ensure_init(); G_NEXT_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_ANSWER() -> Py { ensure_init(); G_NOT_AN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_ANSWER(v: Py) { ensure_init(); G_NOT_AN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_RECURRENCE() -> Py { ensure_init(); G_RECURRENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECURRENCE(v: Py) { ensure_init(); G_RECURRENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_RECURRING() -> Py { ensure_init(); G_RECURRING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECURRING(v: Py) { ensure_init(); G_RECURRING.with(|c| *c.borrow_mut() = v); }
pub fn g_REMIND_ME() -> Py { ensure_init(); G_REMIND_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMIND_ME(v: Py) { ensure_init(); G_REMIND_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_RETURNING() -> Py { ensure_init(); G_RETURNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETURNING(v: Py) { ensure_init(); G_RETURNING.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_EVIDENT() -> Py { ensure_init(); G_SELF_EVIDENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_EVIDENT(v: Py) { ensure_init(); G_SELF_EVIDENT.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that says the trouble came back: the answer's own next step is the reply.\n\n\"the boiler keeps losing pressure\" gets the answer, and the next line is \"top it up and it came\nback after a week\". Nothing reads that line as being about the answer, so it fell to the\nfallback pools - \"I'm listening, say more if it helps\", \"Got it - it came back after a week\" -\nwhich throw away the answer the person is telling you has not held. The answer already says what\nto do at that point (\"If it keeps dropping, there's a leak somewhere: stop topping it up and get\na heating engineer out\"), and that sentence is the reply: nothing is invented, and the person\ngets the step they are owed instead of their own words handed back.\n\nThe rule is narrow on purpose:\n* the line has to say the thing itself came back (\"it came back\", \"its back\", \"back again\"), not\n  the person going somewhere (\"i went back to work today\") and not a thing being handed over\n  (\"should i give it back to the shop\");\n* the turn before has to be an answer, so nothing is read out of a greeting, a hedge or an\n  emergency;\n* the answer has to hold a sentence that says what to do if it recurs. Most answers do not, and\n  then this module says nothing and the rest of the bot answers as it did.\n");
    s_RETURNING(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|this|they|the (?:same )?(?:problem|trouble|issue|fault|leak|smell|noise|pain|spot|mould|damp|drip|mark|stain|rash|ache|cough|itch|swelling|spots|noises|pains|leaks|smells|rashes|aches|coughs|itches)s?)\\s+(?:(?:'s|is|was|has|had|have|came|comes|come|keeps coming|kept coming|keep coming)\\s+)(?:\\w+\\s+){0,2}back\\b|\\b(?:its|it's|they're|theyre)\\s+back\\b|\\b(?:it|that|this)\\s+has\\s+(?:come|came)\\s+back\\b|\\bback again\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:it|its|it's|that|thats|that's|they|theyre|they're|this|the (?:same )?(?:problem|trouble|issue|fault|leak|smell|noise|pain|ache|rash|drip|mould|damp|blockage|spots?|pains?|noises?|leaks?|spots?))\\s+(?:has |have |had |is |was |keeps? |kept |just |only |still |all |already |again )*(?:dropped|drop|drops|dropping|started|starting|starts|happened|happens|happening|come back|came back|comes back|come|came|gone|gone off|returned|return|done it|does it|doing it|played up|plays up|playing up|acting up|acts up|acted up|flared up|flares up|flaring up|stopped working|broken|broke|failed|fails|failing|blocking|blocked|back|no better|not better|not clearing|not cleared|just as bad|as bad|worse|better)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECURRING(stdlib_call("re", "compile", &[py_str("\\b(?:again|still|no better|not better|worse than before|just as bad|as bad as before)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_EVIDENT(stdlib_call("re", "compile", &[py_str("\\b(?:returned|return|came back|come back|coming back|keeps? coming|kept coming|keeps? (?:dropping|failing|blocking|breaking|playing up|acting up|flaring up)|flared up|flaring up|back again)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING(stdlib_call("re", "compile", &[py_str("^(?:will|would|does|do|did|is|are|was|were|can|could|should|why|when|how|what|where|who|shall)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEXT_STEP(stdlib_call("re", "compile", &[py_str("\\b(?:if|when|once)\\s+(?:it|that|this|they|things)\\s+(?:keeps?|still|comes? back|come back|returns?|happens? again|does it again|doesn'?t|does not|hasn'?t|has not|has been|have been|lasts?|persists?|keeps up|won'?t|will not|is no better|isn'?t better|isn'?t clearing|is not clearing|gets worse)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_ANSWER(py_tuple(vec![py_str("safety"), py_str("urgent"), py_str("grief"), py_str("loss")]));
    s_CONDITIONAL_ASK(stdlib_call("re", "compile", &[py_str("^(?:and|but|so|then)?\\s*(?:what\\s+)?if\\s+(?:it|that|this|they|theyre|it's|things|the\\s+\\w+)\\s+(?:does\\s?n[o']?t|does not|did\\s?n[o']?t|did not|is\\s?n[o']?t|is not|has\\s?n[o']?t|has not|have\\s?n[o']?t|gets?|stays?|keeps?|lasts?|still|stops?|eases?|no better|not better|not clearing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADING_CONDITION(stdlib_call("re", "compile", &[py_str("^(?:if|when|once)\\s[^,.;:]{0,80},\\s*(?=[A-Za-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTINUES(py_tuple(vec![py_str("or"), py_str("and"), py_str("keeps"), py_str("keep"), py_str("kept"), py_str("comes"), py_str("come"), py_str("still"), py_str("lasts"), py_str("persists"), py_str("gets"), py_str("goes"), py_str("happens"), py_str("does"), py_str("hasn't"), py_str("has not"), py_str("isn't"), py_str("is not"), py_str("won't"), py_str("will not"), py_str("stops"), py_str("swells"), py_str("spreads"), py_str("bleeds"), py_str("hurts"), py_str("wakes")]));
    s_RECURRENCE(stdlib_call("re", "compile", &[py_str("\\b(?:again|still)\\b\\s*(?:(?:today|tonight|this morning|this afternoon|this evening|this week|already|now|too|though)\\b\\s*)*(?:[.!]*$|of\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMIND_ME(stdlib_call("re", "compile", &[py_str("\\b(?:remind|refresh my memory|what (?:was|is|were|did))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ASK(stdlib_call("re", "compile", &[py_str("^(?:how|what|when|where|which|who|why|can|could|should|shall|will|would|may|might|do|does|did|is|are|am|was|were|have|has|any|is there|are there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn reads_as_returning(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line reports that the thing the answer was about has come back.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(16)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_ASKING(), "match", &[line.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_RETURNING(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_AGAIN(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_RECURRING(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SELF_EVIDENT(), "search", &[line.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_turn_before(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return if py_truthy(&turns.clone()) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { Py::None };
    Py::None
}
pub fn _the_answer_before(args: &[Py]) -> Py {
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
    let mut keep: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keep") { v.clone() }
    else { Py::Int(4) };
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The last thing the bot said that holds a next step, or None.\n\n    Not simply the turn before: a nod in between (\"Got it - you topped it up yesterday\") is not\n    the answer, and reading the recurrence against it lost the step the person has earned -\n    \"my boiler keeps losing pressure\" / \"i topped it up yesterday\" / \"its dropped again\" said\n    \"It's dropped again - noted\" instead of \"there's a leak somewhere: stop topping it up\". An\n    emergency answer still owns the turns after it, so the walk stops there.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&keep.clone()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_NOT_AN_ANSWER(), &call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&the_next_step(&[mcall(&turn.clone(), "get", &[py_str("bot")])])) {
    return turn.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn the_next_step(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentence of an answer that says what to do if the trouble recurs, or None.\n\n    The conditional is taken off the front of it, because the person has already read the\n    sentence it belonged to. A conditional in the middle of a sentence (\"see a GP if it is no\n    better in a fortnight\") is left whole: cutting that would leave a fragment.\n    ");
    let __iter1 = py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]));
    for __item in __iter1 {
    sentence = __item.clone();
    sentence = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&sentence.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NEXT_STEP(), "search", &[sentence.clone()]))) } }) {
    continue;
    }
    bare = mcall(&g_LEADING_CONDITION(), "sub", &[py_str(""), sentence.clone()]);
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&bare.clone(), &sentence.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[mcall(&bare.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_CONTINUES(), &mcall(&mcall(&geti(&mcall(&bare.clone(), "split", &[]), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",.;:")]))) } } } } } }) {
    return op_add(&mcall(&slice_or_raise(&bare.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&bare.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&mcall(&mcall(&sentence.clone(), "lower", &[]), "startswith", &[py_tuple(vec![py_str("if "), py_str("when "), py_str("once ")])])) {
    continue;
    }
    return sentence.clone();
    }
    return Py::None;
    Py::None
}
pub fn says_it_happened_again(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line reports the thing happening once more, so a repeat frame is wrong.\n\n    A repeat frame says \"same answer as before\", and that is a wrong thing to say to news: the\n    person is telling the bot the trouble has come back, not asking it to read the old answer out.\n    Standing down hands the line to the modules that answer a return - which have the next step\n    the first answer could not know about (\"Then say it once, plainly, about the next change\").\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_REMIND_ME(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_AN_ASK(), "match", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&reads_as_returning(&[said.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_RECURRENCE(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_conditional(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks what happens if the trouble is still there.\n\n    Not the same as a recurrence report: nothing has come back yet, the person is asking about the\n    branch the answer named - \"and if it does not get better in two weeks\" after a rash answer.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[mcall(&g_CONDITIONAL_ASK(), "match", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn pool_for(args: &[Py]) -> Py {
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
    let mut around: Py = Py::None;
    let mut before: Py = Py::None;
    let mut conditional: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The next step the answer already names, or None when this line is not that follow-up.\n\n    Returned as the sentence itself: it is the bot's own advice, said again because it now\n    applies, and a lead-in in front of it (\"same answer, and it covers this\") would claim the\n    whole answer still stands when the part that matters is the branch the person has reached.\n    ");
    conditional = reads_as_a_conditional(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reads_as_returning(&[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&conditional.clone())) } }) {
    return Py::None;
    }
    before = _the_answer_before(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return Py::None;
    }
    said = call_builtin("str", &[{ let __b = mcall(&before.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    step = the_next_step(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&step.clone(), "lower", &[]), "split", &[])]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])])))) {
    return Py::None;
    }
    if py_truthy(&conditional.clone()) {
    around = mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "replace", &[step.clone(), py_str(" ")]), "split", &[])]);
    if py_truthy(&op_le(&call_builtin("len", &[mcall(&around.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) {
    return Py::None;
    }
    }
    return step.clone();
    Py::None
}
pub fn the_whole_of_it(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The answer the step was cut from, for the store's record of what has been said.\n\n    The person has now read every part of that answer: the fix on the first turn, and the branch\n    for the trouble coming back on this one. Keeping only the branch lets the rest of it be handed\n    over as new a turn later, which reads as the bot forgetting itself.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_returning(&[text.clone()])))) {
    return Py::None;
    }
    before = _the_answer_before(&[memory.clone()]);
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&the_next_step(&[said.clone()])) { said.clone() } else { Py::None };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.returned".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGAIN" => Some(g_AGAIN()),
            "AN_ASK" => Some(g_AN_ASK()),
            "ASKING" => Some(g_ASKING()),
            "CONDITIONAL_ASK" => Some(g_CONDITIONAL_ASK()),
            "CONTINUES" => Some(g_CONTINUES()),
            "LEADING_CONDITION" => Some(g_LEADING_CONDITION()),
            "NEXT_STEP" => Some(g_NEXT_STEP()),
            "NOT_AN_ANSWER" => Some(g_NOT_AN_ANSWER()),
            "RECURRENCE" => Some(g_RECURRENCE()),
            "RECURRING" => Some(g_RECURRING()),
            "REMIND_ME" => Some(g_REMIND_ME()),
            "RETURNING" => Some(g_RETURNING()),
            "SELF_EVIDENT" => Some(g_SELF_EVIDENT()),
            "SENTENCE" => Some(g_SENTENCE()),
            "_the_answer_before" => Some(py_func("_the_answer_before", |args: &[Py]| _the_answer_before(args))),
            "_the_turn_before" => Some(py_func("_the_turn_before", |args: &[Py]| _the_turn_before(args))),
            "pool_for" => Some(py_func("pool_for", |args: &[Py]| pool_for(args))),
            "reads_as_a_conditional" => Some(py_func("reads_as_a_conditional", |args: &[Py]| reads_as_a_conditional(args))),
            "reads_as_returning" => Some(py_func("reads_as_returning", |args: &[Py]| reads_as_returning(args))),
            "says_it_happened_again" => Some(py_func("says_it_happened_again", |args: &[Py]| says_it_happened_again(args))),
            "the_next_step" => Some(py_func("the_next_step", |args: &[Py]| the_next_step(args))),
            "the_whole_of_it" => Some(py_func("the_whole_of_it", |args: &[Py]| the_whole_of_it(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGAIN" => s_AGAIN(v),
            "AN_ASK" => s_AN_ASK(v),
            "ASKING" => s_ASKING(v),
            "CONDITIONAL_ASK" => s_CONDITIONAL_ASK(v),
            "CONTINUES" => s_CONTINUES(v),
            "LEADING_CONDITION" => s_LEADING_CONDITION(v),
            "NEXT_STEP" => s_NEXT_STEP(v),
            "NOT_AN_ANSWER" => s_NOT_AN_ANSWER(v),
            "RECURRENCE" => s_RECURRENCE(v),
            "RECURRING" => s_RECURRING(v),
            "REMIND_ME" => s_REMIND_ME(v),
            "RETURNING" => s_RETURNING(v),
            "SELF_EVIDENT" => s_SELF_EVIDENT(v),
            "SENTENCE" => s_SENTENCE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.returned", module_obj);
    pyrt::xrefs::reg_fn("memory.returned", "reads_as_returning", py_func("reads_as_returning", |args: &[Py]| reads_as_returning(args)));
    pyrt::xrefs::reg_fn("memory.returned", "_the_turn_before", py_func("_the_turn_before", |args: &[Py]| _the_turn_before(args)));
    pyrt::xrefs::reg_fn("memory.returned", "_the_answer_before", py_func("_the_answer_before", |args: &[Py]| _the_answer_before(args)));
    pyrt::xrefs::reg_fn("memory.returned", "the_next_step", py_func("the_next_step", |args: &[Py]| the_next_step(args)));
    pyrt::xrefs::reg_fn("memory.returned", "says_it_happened_again", py_func("says_it_happened_again", |args: &[Py]| says_it_happened_again(args)));
    pyrt::xrefs::reg_fn("memory.returned", "reads_as_a_conditional", py_func("reads_as_a_conditional", |args: &[Py]| reads_as_a_conditional(args)));
    pyrt::xrefs::reg_fn("memory.returned", "pool_for", py_func("pool_for", |args: &[Py]| pool_for(args)));
    pyrt::xrefs::reg_fn("memory.returned", "the_whole_of_it", py_func("the_whole_of_it", |args: &[Py]| the_whole_of_it(args)));
    pyrt::xrefs::reg_access("memory.returned", "AGAIN", g_AGAIN, s_AGAIN);
    pyrt::xrefs::reg_access("memory.returned", "AN_ASK", g_AN_ASK, s_AN_ASK);
    pyrt::xrefs::reg_access("memory.returned", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("memory.returned", "CONDITIONAL_ASK", g_CONDITIONAL_ASK, s_CONDITIONAL_ASK);
    pyrt::xrefs::reg_access("memory.returned", "CONTINUES", g_CONTINUES, s_CONTINUES);
    pyrt::xrefs::reg_access("memory.returned", "LEADING_CONDITION", g_LEADING_CONDITION, s_LEADING_CONDITION);
    pyrt::xrefs::reg_access("memory.returned", "NEXT_STEP", g_NEXT_STEP, s_NEXT_STEP);
    pyrt::xrefs::reg_access("memory.returned", "NOT_AN_ANSWER", g_NOT_AN_ANSWER, s_NOT_AN_ANSWER);
    pyrt::xrefs::reg_access("memory.returned", "RECURRENCE", g_RECURRENCE, s_RECURRENCE);
    pyrt::xrefs::reg_access("memory.returned", "RECURRING", g_RECURRING, s_RECURRING);
    pyrt::xrefs::reg_access("memory.returned", "REMIND_ME", g_REMIND_ME, s_REMIND_ME);
    pyrt::xrefs::reg_access("memory.returned", "RETURNING", g_RETURNING, s_RETURNING);
    pyrt::xrefs::reg_access("memory.returned", "SELF_EVIDENT", g_SELF_EVIDENT, s_SELF_EVIDENT);
    pyrt::xrefs::reg_access("memory.returned", "SENTENCE", g_SENTENCE, s_SENTENCE);
}
