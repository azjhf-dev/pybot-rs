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


thread_local! { static G_A_DEATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VIEW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ASK_LATER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_A_VIEW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DECLINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECLINED_LITERAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JUST_LISTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WANT_TO_BE_HEARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_DEATH() -> Py { ensure_init(); G_A_DEATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEATH(v: Py) { ensure_init(); G_A_DEATH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VIEW() -> Py { ensure_init(); G_A_VIEW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VIEW(v: Py) { ensure_init(); G_A_VIEW.with(|c| *c.borrow_mut() = v); }
pub fn g_HEARD() -> Py { ensure_init(); G_HEARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEARD(v: Py) { ensure_init(); G_HEARD.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ASK() -> Py { ensure_init(); G__AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ASK(v: Py) { ensure_init(); G__AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ASK_LATER() -> Py { ensure_init(); G__AN_ASK_LATER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ASK_LATER(v: Py) { ensure_init(); G__AN_ASK_LATER.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_A_VIEW() -> Py { ensure_init(); G__ASKS_A_VIEW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_A_VIEW(v: Py) { ensure_init(); G__ASKS_A_VIEW.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DECLINE() -> Py { ensure_init(); G__A_DECLINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DECLINE(v: Py) { ensure_init(); G__A_DECLINE.with(|c| *c.borrow_mut() = v); }
pub fn g__DECLINED_LITERAL() -> Py { ensure_init(); G__DECLINED_LITERAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECLINED_LITERAL(v: Py) { ensure_init(); G__DECLINED_LITERAL.with(|c| *c.borrow_mut() = v); }
pub fn g__JUST_LISTEN() -> Py { ensure_init(); G__JUST_LISTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JUST_LISTEN(v: Py) { ensure_init(); G__JUST_LISTEN.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_AN_ASK() -> Py { ensure_init(); G__NOT_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_AN_ASK(v: Py) { ensure_init(); G__NOT_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THIS() -> Py { ensure_init(); G__NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THIS(v: Py) { ensure_init(); G__NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_ADVICE() -> Py { ensure_init(); G__NO_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_ADVICE(v: Py) { ensure_init(); G__NO_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g__WANT_TO_BE_HEARD() -> Py { ensure_init(); G__WANT_TO_BE_HEARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WANT_TO_BE_HEARD(v: Py) { ensure_init(); G__WANT_TO_BE_HEARD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that says it does not want advice wants to be heard, not fixed.\n\nNobody types \"and i dont want advice\" to a machine unless they have had advice they did not ask\nfor, and the worst thing to do with it is hand them a card. Yet every one of these reached a card\nor a search pointer:\n\n    my son was in an accident and i dont want advice     -> \"a search should sort you out\"\n    my dog is ill and i dont want advice                 -> \"What would help most right now?\"\n    i dont want advice, i just want to talk              -> \"That's beyond what I know well\"\n    not looking for a solution, just want to know what you think\n                                                         -> the chemistry definition of a solution\n    i dont need advice, just someone to listen           -> a card about listening to people\n    im not looking for advice, only to say it out loud   -> \"I'm not sure about that one\"\n    i know what to do, i just wanted to tell someone     -> a place to visit\n\nThe words of the line are the words of the thing they do not want, so each of them was a lookup\nthat found exactly the wrong entry. Nothing about it is a question to look up. What the bot knows\nis the shape of the turn itself: somebody is telling it something, and the useful reply is to say\nthey have been heard and make it easy to go on - no advice, no card, and no \"search for it\".\n\nTwo shapes, because they ask for different things. Most decline the fixing altogether. A few\ndecline the fix and ask for a view instead (\"just want to know what you think\"), and a view is\nsomething the bot can give once it knows the situation, so that shape asks for the situation.\n\nThe matcher needs a decline, in the same clause as the thing declined, so a line that asks for\nadvice keeps its answer (\"what advice would you give my sister\", \"should i take advice from a\nbroker\"), and \"i dont want to give advice\" - about somebody else - is not this line either.\n");
    s__DECLINED_LITERAL(py_str("(?:advice|advise|solutions?|tips?|suggestions?|a\\s+lecture|lectures?|to\\s+be\\s+told\\s+what\\s+to\\s+do|telling\\s+me\\s+what\\s+to\\s+do)"));
    s__NO_ADVICE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:(?:i|we|he|she|they)\\s+(?:dont|don't|do\\s+not|doesnt|doesn't|didnt|didn't|wouldnt|wouldn't)\\s+(?:really\\s+|actually\\s+|even\\s+)?(?:want|need|wanted|needed|asked\\s+for|ask\\s+for)\\s+(?:(?:any|no|your|more|further|the|medical|financial|legal|diet|parenting|career)\\s+){0,2}(?:"), &g__DECLINED_LITERAL()), &py_str(")|(?:i|we|im|i'm)\\s+(?:am\\s+)?(?:not|n't)\\s+(?:looking|asking|after|here)\\s+for\\s+(?:a\\s+|any\\s+|the\\s+)?(?:advice|solutions?|fix|fixing|tips?|suggestions?)|(?:not|n't)\\s+looking\\s+for\\s+(?:a\\s+|any\\s+)?(?:solution|advice|fix)|no\\s+(?:advice|solutions?|tips?|lectures?)(?:\\s+(?:please|thanks|thank\\s+you))?|without\\s+(?:the\\s+)?(?:advice|lecture|tips)|(?:dont|don't|do\\s+not|no)\\s+(?:give|giving)\\s+me\\s+(?:any\\s+)?(?:advice|tips)|not\\s+(?:asking|looking)\\s+for\\s+(?:advice|a\\s+solution)|(?:dont|don't|do\\s+not|stop)\\s+tell(?:ing)?\\s+me\\s+what\\s+to\\s+do|(?:i|i've|ive)\\s+(?:already\\s+)?know\\s+what\\s+to\\s+do,\\s+(?:i\\s+)?(?:just|only)\\s+(?:want|wanted|need|needed))\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WANT_TO_BE_HEARD(stdlib_call("re", "compile", &[py_str("\\b(?:(?:just|only|really)\\s+(?:want|wanted|need|needed)\\s+(?:to\\s+)?(?:talk|vent|be\\s+heard|get\\s+(?:it|this|that|something)\\s+off\\s+(?:my|our)\\s+chest)|(?:just|only)\\s+(?:wanted|want|needed)\\s+to\\s+(?:say\\s+(?:it|this|that|something)|tell\\s+(?:you|u|someone|somebody|them|him|her)\\s+something)|(?:just|only)\\s+(?:wanted|want)\\s+(?:to\\s+)?(?:say|tell)\\s+(?:it\\s+)?out\\s+loud|wanted\\s+to\\s+say\\s+it\\s+out\\s+loud|i\\s+(?:just\\s+)?(?:want|wanted)\\s+to\\s+be\\s+heard|(?:just\\s+|only\\s+)?(?:want|wanted|need|needed)\\s+(?:someone|somebody|a\\s+person)\\s+(?:who\\s+)?(?:to\\s+)?listen|just\\s+(?:talk|listen)\\s+to\\s+me|(?:let|allow)\\s+me\\s+(?:to\\s+)?(?:talk|vent|rant)|(?:just\\s+|only\\s+)?(?:want|need|wanted|needed)\\s+to\\s+(?:vent|rant)|can\\s+i\\s+(?:just\\s+)?(?:vent|rant)|i\\s+need\\s+(?:a\\s+)?(?:vent|rant)|can\\s+i\\s+just\\s+talk)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__JUST_LISTEN(stdlib_call("re", "compile", &[py_str("\\b(?:just|only)\\s+listen(?:\\s+to\\s+me)?(?:\\s+please)?\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_A_VIEW(stdlib_call("re", "compile", &[py_str("\\b(?:what\\s+(?:do\\s+)?(?:you|u)\\s+(?:think|reckon|make\\s+of\\s+it)|whats\\s+your\\s+(?:honest\\s+)?(?:opinion|view|take)|what\\s+is\\s+your\\s+(?:opinion|view|take)|your\\s+honest\\s+opinion|your\\s+view\\s+on\\s+it|(?:must|would\\s+you)\\s+think\\s+of\\s+me|does\\s+that\\s+(?:sound|seem)\\s+?(?:mad|silly|wrong|bad)|am\\s+i\\s+(?:being\\s+)?(?:mad|silly|wrong|unreasonable))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:what|some|good|the\\s+best|better)\\s+advice\\b|should\\s+i\\s+take\\s+(?:her|his|their|my|any|some)\\s+advice|(?:give|giving|offer|offering)\\s+(?:her|him|them|my|his|your)\\s+advice|(?:dont|don't|do\\s+not)\\s+want\\s+to\\s+give|asked\\s+(?:for|about)\\s+(?:her|his|their|my)\\s+advice"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ASK(stdlib_call("re", "compile", &[py_str("(?:\\b(?:what|how|which|when|where|who|why|should|could|would|can|is|are|does|do)\\b[^?.!]{0,60}\\?|\\b(?:what|how|which|where|who)\\b\\s+(?:do|should|can|could|would)\\b|\\bwhats\\b|\\bwhat's\\b|\\bshould\\s+i\\b|\\bcan\\s+i\\b|\\bcould\\s+i\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DECLINE(stdlib_call("re", "compile", &[op_modulo(&py_str("(?:%s)|(?:%s)|(?:%s)"), &py_tuple(vec![gattr(&g__NO_ADVICE(), "pattern"), gattr(&g__WANT_TO_BE_HEARD(), "pattern"), gattr(&g__JUST_LISTEN(), "pattern")])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ASK_LATER(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|what's|how|which|when|where|who|why|should|could|would|can|is|are|was|were|do|does|did|will|want\\s+to\\s+know|wondering|wonder\\s+if|tell\\s+me\\s+(?:if|whether|what|how)|let\\s+me\\s+know\\s+(?:if|whether|what|how))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_AN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:is\\s+that\\s+(?:ok|okay|alright|fine)|if\\s+thats?\\s+(?:ok|okay|alright|fine)|does\\s+that\\s+make\\s+sense|is\\s+that\\s+alright\\s+with\\s+you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|dead|death|killed|suicide|passed\\s+away|passed\\s+on|funeral|burial|cremation|took\\s+(?:his|her|their)\\s+own\\s+life|buried|burying|bereav\\w*|grief|grieving|widow|widowed|mourning)\\b|\\b(?:lost|bury|lose|losing)\\s+(?:my|our|her|his|their|the)?\\s*(?:mum|mom|mother|dad|father|wife|husband|baby|child|son|daughter|brother|sister|partner|gran|granny|nan|grandad|grandpa|friend|mate)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEARD(py_tuple(vec![py_str("Fair enough - I'll keep the advice to myself. Tell me the rest of it."), py_str("Understood. I'm listening - how are you doing with it?"), py_str("Right, no fixing from me then. What's on your mind?"), py_str("That's fair. Say it all and I'll just listen."), py_str("No advice, then. Go on - I'm here."), py_str("Okay - no advice from me. Where are you with it all?")]));
    s_A_VIEW(py_tuple(vec![py_str("Then tell me the whole of it and I'll say what I think - no fixing, just my honest view."), py_str("Okay - the situation as it is, and I'll give you my straight view of it."), py_str("I'll tell you what I think, then. What's the situation?"), py_str("Fair enough. Lay it out and I'll tell you honestly what I make of it.")]));
}
pub fn _what_is_left_after_the_decline(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The rest of the line once the decline is taken out of it.");
    said = mcall(&g__A_DECLINE(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return line.clone();
    }
    return slice_or_raise(&line.clone(), &mcall(&said.clone(), "end", &[]), &Py::None, &Py::None);
    Py::None
}
pub fn asks_something_of_its_own(args: &[Py]) -> Py {
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
    let mut left: Py = Py::None;
    let _ = py_str("True when the line asks something besides the decline, so the ask is the turn.");
    left = mcall(&g__NOT_AN_ASK(), "sub", &[py_str(" "), _what_is_left_after_the_decline(&[line.clone()])]);
    return call_builtin("bool", &[mcall(&g__AN_ASK_LATER(), "search", &[left.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _plain(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[])]);
    Py::None
}
pub fn says_a_death(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line carries a death, so a loss reader has the better answer.");
    return call_builtin("bool", &[mcall(&g_A_DEATH(), "search", &[_plain(&[text.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_for_a_view(args: &[Py]) -> Py {
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
    let _ = py_str("True when the same line asks for the bot's view instead of a fix.");
    return call_builtin("bool", &[mcall(&g__ASKS_A_VIEW(), "search", &[_plain(&[text.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn says_no_advice(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line declines advice, or asks only to be heard.");
    line = _plain(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    said = call_builtin("bool", &[{ let __b = mcall(&g__NO_ADVICE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__WANT_TO_BE_HEARD(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__JUST_LISTEN(), "search", &[line.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&asks_for_a_view(&[line.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__NOT_THIS(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    rest = mcall(&g__JUST_LISTEN(), "sub", &[py_str(" "), mcall(&g__WANT_TO_BE_HEARD(), "sub", &[py_str(" "), line.clone()])]);
    if py_truthy(&mcall(&g__AN_ASK(), "search", &[rest.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&asks_something_of_its_own(&[line.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn pool_name(args: &[Py]) -> Py {
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
    let _ = py_str("Which reply pool the line wants: the view one, or the plain \"I'm listening\" one, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&says_no_advice(&[text.clone()])))) {
    return Py::None;
    }
    return if py_truthy(&asks_for_a_view(&[text.clone()])) { py_str("a_view") } else { py_str("no_advice") };
    Py::None
}
pub fn lines_for(args: &[Py]) -> Py {
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
    let _ = py_str("The pool of lines for the turn: the listening ones or the view ones.");
    return if py_truthy(&Py::Bool(py_eq(&pool.clone(), &py_str("a_view")))) { g_A_VIEW() } else { g_HEARD() };
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
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("The line for this turn, varied on the person's own words so two lines do not read alike.");
    let mut pick: Py = py_func("pick", |args: &[Py]| crate::compose::pick(args));
    return py_call(&pick, &[call_builtin("list", &[lines_for(&[pool.clone()])]).unwrap_or_else(|e| exc::raise(e)), memory.clone(), text.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.no_advice".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_DEATH" => Some(g_A_DEATH()),
            "A_VIEW" => Some(g_A_VIEW()),
            "HEARD" => Some(g_HEARD()),
            "_AN_ASK" => Some(g__AN_ASK()),
            "_AN_ASK_LATER" => Some(g__AN_ASK_LATER()),
            "_ASKS_A_VIEW" => Some(g__ASKS_A_VIEW()),
            "_A_DECLINE" => Some(g__A_DECLINE()),
            "_DECLINED_LITERAL" => Some(g__DECLINED_LITERAL()),
            "_JUST_LISTEN" => Some(g__JUST_LISTEN()),
            "_NOT_AN_ASK" => Some(g__NOT_AN_ASK()),
            "_NOT_THIS" => Some(g__NOT_THIS()),
            "_NO_ADVICE" => Some(g__NO_ADVICE()),
            "_WANT_TO_BE_HEARD" => Some(g__WANT_TO_BE_HEARD()),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_what_is_left_after_the_decline" => Some(py_func("_what_is_left_after_the_decline", |args: &[Py]| _what_is_left_after_the_decline(args))),
            "asks_for_a_view" => Some(py_func("asks_for_a_view", |args: &[Py]| asks_for_a_view(args))),
            "asks_something_of_its_own" => Some(py_func("asks_something_of_its_own", |args: &[Py]| asks_something_of_its_own(args))),
            "lines_for" => Some(py_func("lines_for", |args: &[Py]| lines_for(args))),
            "pool_name" => Some(py_func("pool_name", |args: &[Py]| pool_name(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "says_a_death" => Some(py_func("says_a_death", |args: &[Py]| says_a_death(args))),
            "says_no_advice" => Some(py_func("says_no_advice", |args: &[Py]| says_no_advice(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_DEATH" => s_A_DEATH(v),
            "A_VIEW" => s_A_VIEW(v),
            "HEARD" => s_HEARD(v),
            "_AN_ASK" => s__AN_ASK(v),
            "_AN_ASK_LATER" => s__AN_ASK_LATER(v),
            "_ASKS_A_VIEW" => s__ASKS_A_VIEW(v),
            "_A_DECLINE" => s__A_DECLINE(v),
            "_DECLINED_LITERAL" => s__DECLINED_LITERAL(v),
            "_JUST_LISTEN" => s__JUST_LISTEN(v),
            "_NOT_AN_ASK" => s__NOT_AN_ASK(v),
            "_NOT_THIS" => s__NOT_THIS(v),
            "_NO_ADVICE" => s__NO_ADVICE(v),
            "_WANT_TO_BE_HEARD" => s__WANT_TO_BE_HEARD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.no_advice", module_obj);
    pyrt::xrefs::reg_fn("compose.no_advice", "_what_is_left_after_the_decline", py_func("_what_is_left_after_the_decline", |args: &[Py]| _what_is_left_after_the_decline(args)));
    pyrt::xrefs::reg_fn("compose.no_advice", "asks_something_of_its_own", py_func("asks_something_of_its_own", |args: &[Py]| asks_something_of_its_own(args)));
    pyrt::xrefs::reg_fn("compose.no_advice", "_plain", py_func("_plain", |args: &[Py]| _plain(args)));
    pyrt::xrefs::reg_fn("compose.no_advice", "says_a_death", py_func("says_a_death", |args: &[Py]| says_a_death(args)));
    pyrt::xrefs::reg_fn("compose.no_advice", "asks_for_a_view", py_func("asks_for_a_view", |args: &[Py]| asks_for_a_view(args)));
    pyrt::xrefs::reg_fn("compose.no_advice", "says_no_advice", py_func("says_no_advice", |args: &[Py]| says_no_advice(args)));
    pyrt::xrefs::reg_fn("compose.no_advice", "pool_name", py_func("pool_name", |args: &[Py]| pool_name(args)));
    pyrt::xrefs::reg_fn("compose.no_advice", "lines_for", py_func("lines_for", |args: &[Py]| lines_for(args)));
    pyrt::xrefs::reg_fn("compose.no_advice", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.no_advice", "A_DEATH", g_A_DEATH, s_A_DEATH);
    pyrt::xrefs::reg_access("compose.no_advice", "A_VIEW", g_A_VIEW, s_A_VIEW);
    pyrt::xrefs::reg_access("compose.no_advice", "HEARD", g_HEARD, s_HEARD);
    pyrt::xrefs::reg_access("compose.no_advice", "_AN_ASK", g__AN_ASK, s__AN_ASK);
    pyrt::xrefs::reg_access("compose.no_advice", "_AN_ASK_LATER", g__AN_ASK_LATER, s__AN_ASK_LATER);
    pyrt::xrefs::reg_access("compose.no_advice", "_ASKS_A_VIEW", g__ASKS_A_VIEW, s__ASKS_A_VIEW);
    pyrt::xrefs::reg_access("compose.no_advice", "_A_DECLINE", g__A_DECLINE, s__A_DECLINE);
    pyrt::xrefs::reg_access("compose.no_advice", "_DECLINED_LITERAL", g__DECLINED_LITERAL, s__DECLINED_LITERAL);
    pyrt::xrefs::reg_access("compose.no_advice", "_JUST_LISTEN", g__JUST_LISTEN, s__JUST_LISTEN);
    pyrt::xrefs::reg_access("compose.no_advice", "_NOT_AN_ASK", g__NOT_AN_ASK, s__NOT_AN_ASK);
    pyrt::xrefs::reg_access("compose.no_advice", "_NOT_THIS", g__NOT_THIS, s__NOT_THIS);
    pyrt::xrefs::reg_access("compose.no_advice", "_NO_ADVICE", g__NO_ADVICE, s__NO_ADVICE);
    pyrt::xrefs::reg_access("compose.no_advice", "_WANT_TO_BE_HEARD", g__WANT_TO_BE_HEARD, s__WANT_TO_BE_HEARD);
}
