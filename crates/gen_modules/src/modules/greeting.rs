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


thread_local! { static G_A_DETAIL_OF_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DOING_SOON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FACT_OF_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEW_FACT_ABOUT_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REFERS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STEP_IN_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_A_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_BEFORE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_TENSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUSHING_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTE_MARK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESTART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGNING_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES_AS_IT_COMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEN_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GO_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_respond: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_DETAIL_OF_IT() -> Py { ensure_init(); G_A_DETAIL_OF_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DETAIL_OF_IT(v: Py) { ensure_init(); G_A_DETAIL_OF_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DOING_SOON() -> Py { ensure_init(); G_A_DOING_SOON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DOING_SOON(v: Py) { ensure_init(); G_A_DOING_SOON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FACT_OF_THEIRS() -> Py { ensure_init(); G_A_FACT_OF_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FACT_OF_THEIRS(v: Py) { ensure_init(); G_A_FACT_OF_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEW_FACT_ABOUT_SOMEONE() -> Py { ensure_init(); G_A_NEW_FACT_ABOUT_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEW_FACT_ABOUT_SOMEONE(v: Py) { ensure_init(); G_A_NEW_FACT_ABOUT_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REFERS_BACK() -> Py { ensure_init(); G_A_REFERS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REFERS_BACK(v: Py) { ensure_init(); G_A_REFERS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STEP_IN_IT() -> Py { ensure_init(); G_A_STEP_IN_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STEP_IN_IT(v: Py) { ensure_init(); G_A_STEP_IN_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_AHEAD() -> Py { ensure_init(); G_A_TIME_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_AHEAD(v: Py) { ensure_init(); G_A_TIME_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_A_STEP() -> Py { ensure_init(); G_DONE_A_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_A_STEP(v: Py) { ensure_init(); G_DONE_A_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_BEFORE_IT() -> Py { ensure_init(); G_NOTHING_BEFORE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_BEFORE_IT(v: Py) { ensure_init(); G_NOTHING_BEFORE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_TENSE() -> Py { ensure_init(); G_PAST_TENSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_TENSE(v: Py) { ensure_init(); G_PAST_TENSE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN() -> Py { ensure_init(); G_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN(v: Py) { ensure_init(); G_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_PUSHING_BACK() -> Py { ensure_init(); G_PUSHING_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUSHING_BACK(v: Py) { ensure_init(); G_PUSHING_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTE_MARK() -> Py { ensure_init(); G_QUOTE_MARK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTE_MARK(v: Py) { ensure_init(); G_QUOTE_MARK.with(|c| *c.borrow_mut() = v); }
pub fn g_RESTART() -> Py { ensure_init(); G_RESTART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESTART(v: Py) { ensure_init(); G_RESTART.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGNING_OFF() -> Py { ensure_init(); G_SIGNING_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGNING_OFF(v: Py) { ensure_init(); G_SIGNING_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEN_TAIL() -> Py { ensure_init(); G_THEN_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEN_TAIL(v: Py) { ensure_init(); G_THEN_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__GO_ON() -> Py { ensure_init(); G__GO_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GO_ON(v: Py) { ensure_init(); G__GO_ON.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_BACK() -> Py { ensure_init(); G__POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_BACK(v: Py) { ensure_init(); G__POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_respond() -> Py { ensure_init(); G_respond.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_respond(v: Py) { ensure_init(); G_respond.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    s_respond(py_func("respond", |args: &[Py]| respond__wrapped(args)));
    let _ = py_str("The seed's fallback: it only answers when no other module does.");
    s_PRIORITY(op_neg(&Py::Int(1000)));
    s_A_REFERS_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:you|u|yous)\\s+(?:said|told me|mentioned|suggested|advised|recommended|agreed|promised|told me to)\\b|^(?:(?:o+|oh|and|so|ok|okay|right|now|but|well|anyway|erm|um)\\s+)*(?:back|going back|return|coming back)\\s+to\\s+(?:what|that|it|this|where|our|the)\\b|^(?:(?:o+|oh|and|so|ok|okay|right|now|but|well|anyway|erm|um)\\s+)*(?:what|which)\\s+(?:was|were)\\s+we\\b|\\bwhat we (?:were|was) (?:saying|talking about|on about)\\b|\\b(?:the )?same (?:as )?(?:before|last time)\\b|\\bthat thing i (?:told|mentioned|said to|asked) you\\b|\\bas (?:we|you) (?:said|were saying|agreed|discussed)\\b|\\bas i (?:said|mentioned|told you)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_BEFORE_IT(py_tuple(vec![py_str("There's nothing before this line for me to go on - say what it was about and I'll pick it up."), py_str("I haven't said anything yet, so give me the thing itself: what are we going back to?"), py_str("Nothing on my side yet - tell me what it was and I'll take it from there."), py_str("That's the first thing you've said to me, so I'll need it in full: what was it about?")]));
    s_A_STEP_IN_IT(py_tuple(vec![py_str("check"), py_str("clear"), py_str("tr"), py_str("us"), py_str("put"), py_str("add"), py_str("set"), py_str("switch"), py_str("turn"), py_str("leav"), py_str("keep"), py_str("ask"), py_str("tak"), py_str("replace"), py_str("restart"), py_str("update"), py_str("mov"), py_str("clean"), py_str("fix"), py_str("buy"), py_str("get"), py_str("call"), py_str("ring"), py_str("book"), py_str("look"), py_str("mak"), py_str("run"), py_str("wash"), py_str("charg"), py_str("delet"), py_str("start"), py_str("install"), py_str("plug"), py_str("test"), py_str("measur"), py_str("wip"), py_str("help"), py_str("need"), py_str("work"), py_str("cost"), py_str("sav")]));
    s_PUSHING_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:though|but|actually|still|to be honest|honestly)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIGNING_OFF(py_tuple(vec![py_str("hope"), py_str("good luck"), py_str("sorry"), py_str("take care"), py_str("all the best"), py_str("let me know"), py_str("good question"), py_str("that said")]));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUOTE_MARK(stdlib_call("re", "compile", &[py_str("[\\u2018\\u2019\\u201c\\u201d\\\"]|(?<![A-Za-z])'|'(?![A-Za-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_BACK(stdlib_call("re", "compile", &[py_str("^(?:so[, ]+)?(?:where were we|where did we get to|what were you saying|what were we saying|what were we talking about|what was i saying)[?.!]*$|^(?:and )?back to (?:it|that|what we were saying|what we were talking about)[.!]*$|^(?:as|like) (?:i|we) (?:said|were saying|mentioned)[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__GO_ON(py_str("Go on - what has been happening?"));
    s_THEN_TAIL(stdlib_call("re", "compile", &[py_str("\\bthen\\b,?\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN(call_builtin("frozenset", &[mcall(&py_str("about after again all already also and any are because been before being but\nboth can could did does doing done dont down each even ever every for from get got had has have\nhaving her here hers him his how into its just know like made make many more most much must my\nnever not now off on once one only other our out over own said same says she should since some\nstill such than that the their them then there these they thing things think this those though\nthrough time too took under until very was way were what when where which while who why will with\nwould yes yet you your youre ive im id ill youve weve theyre isnt dont cant didnt wasnt\nhavent hasnt wouldnt couldnt shouldnt thats whats lets okay ok min little bit lot thing\nstuff"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_RESTART(stdlib_call("re", "compile", &[py_str("\\b(?:turn\\w*|switch\\w*|restart\\w*|reboot\\w*|reset\\w*|unplug\\w*|plug\\w*|power\\w*|off)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DETAIL_OF_IT(stdlib_call("re", "compile", &[py_str("\\b(?:in|on|at|by|near|under|over|behind|inside|outside|next to)\\s+(?:the|a|an|my|our|his|her|their)?\\s*[a-z]+|\\b(?:it|its|it's|they|theyre|they're|he|she|that|this|there)\\s*(?:'s|s| is| are| was| were| arent| isn't| isnt)\\s+(?:a|an|the|not|quite|just|still|only)?\\s*[a-z]+|\\b(?:they|he|she|we|you|it)\\s+(?:only\\s+|just\\s+|already\\s+|also\\s+)?[a-z]+ed\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FACT_OF_THEIRS(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|the)\\s+[a-z]+\\s+(?:is|are|was|were)\\s+[a-z0-9]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_A_STEP(stdlib_call("re", "compile", &[py_str("\\b(?:i|ive|i've|i've)\\s*(?:have\\s+)?(?:already\\s+)?(?:[a-z]+ed|done|tried|did)\\b|\\b(?:did|tried|done) (?:that|it|this)(?: already)?\\b|\\balready (?:did|done|tried)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME_AHEAD(stdlib_call("re", "compile", &[py_str("\\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|tonight|later|soon|weekend|next\\s+(?:week|month|year|time)|this\\s+(?:morning|afternoon|evening|week|month)|in\\s+the\\s+(?:morning|afternoon|evening)|in\\s+(?:a|an)\\s+(?:bit|while|minute|hour)|in\\s+a\\s+few\\s+days|after\\s+work|day\\s+after\\s+tomorrow)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DOING_SOON(stdlib_call("re", "compile", &[py_str("\\b(?:call\\w*|ring\\w*|phon\\w*|get\\w*|sort\\w*|tak\\w*|book\\w*|do|doing|fix\\w*|go|going|see|seeing|bring\\w*|order\\w*|ask\\w*|contact\\w*|chas\\w*|email\\w*|messag\\w*|text\\w*|arrang\\w*|hir\\w*|report\\w*|pop\\w*|drop\\w*|look\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_TENSE(stdlib_call("re", "compile", &[py_str("ed$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEW_FACT_ABOUT_SOMEONE(stdlib_call("re", "compile", &[py_str("\\ballerg\\w*|\\bintoleran\\w*|\\bcoeliac|\\bceliac|\\blactose\\b|\\b(?:is|im|i'm|are|we're|were)\\s+(?:a\\s+)?(?:vegetarian|vegan|pescatarian)\\b|\\b(?:doesnt|does not|dont|do not|wont|will not|never|can'?t|cant)\\s+(?:eat|drink|have|touch)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKES_AS_IT_COMES(g_takes_as_it_comes());
    s_respond(gen_memory::memory::again::replied(&[py_func("respond", |args: &[Py]| respond__wrapped(args))]));
    s_respond(gen_compose::compose::a_detail_dropped::turned(&[g_respond()]));
}
pub fn _a_line_pointing_back_at_nothing(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let _ = py_str("The pool for an opening line that points at an earlier turn, or None.\n\n    Only when the chat has no turns: after one turn there is something to point at, and every\n    reading that answers a real callback keeps the line.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_REFERS_BACK(), "search", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::None;
    }
    return py_call(&gen_compose::compose::fallback::g__choose(), &[g_NOTHING_BEFORE_IT(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn respond__wrapped(args: &[Py]) -> Py {
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
    let mut a_comment: Py = Py::None;
    let mut a_place: Py = Py::None;
    let mut about_them: Py = Py::None;
    let mut accepted: Py = Py::None;
    let mut after: Py = Py::None;
    let mut again_asked: Py = Py::None;
    let mut against: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut asked_back: Py = Py::None;
    let mut back: Py = Py::None;
    let mut back_at_nothing: Py = Py::None;
    let mut came_back: Py = Py::None;
    let mut cannot_use: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut checks: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut corrected: Py = Py::None;
    let mut course: Py = Py::None;
    let mut deciding: Py = Py::None;
    let mut described: Py = Py::None;
    let mut elsewhere: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut gap_lines: Py = Py::None;
    let mut gave: Py = Py::None;
    let mut glad: Py = Py::None;
    let mut going_on: Py = Py::None;
    let mut handed: Py = Py::None;
    let mut hands_reply: Py = Py::None;
    let mut held: Py = Py::None;
    let mut here: Py = Py::None;
    let mut in_the_room: Py = Py::None;
    let mut judged: Py = Py::None;
    let mut judged_line: Py = Py::None;
    let mut keeping_going: Py = Py::None;
    let mut known: Py = Py::None;
    let mut knows: Py = Py::None;
    let mut late: Py = Py::None;
    let mut line: Py = Py::None;
    let mut looked: Py = Py::None;
    let mut low_day: Py = Py::None;
    let mut named: Py = Py::None;
    let mut not_a_body: Py = Py::None;
    let mut not_good: Py = Py::None;
    let mut nothing_yet: Py = Py::None;
    let mut objected: Py = Py::None;
    let mut of_mine: Py = Py::None;
    let mut off: Py = Py::None;
    let mut off_to: Py = Py::None;
    let mut on_screen: Py = Py::None;
    let mut one_half: Py = Py::None;
    let mut only: Py = Py::None;
    let mut outcome: Py = Py::None;
    let mut own: Py = Py::None;
    let mut own_life: Py = Py::None;
    let mut owned: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut past: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut refused: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut reported: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut retoned: Py = Py::None;
    let mut serious: Py = Py::None;
    let mut stretch: Py = Py::None;
    let mut tasted: Py = Py::None;
    let mut taught: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut thinner: Py = Py::None;
    let mut thread: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut verdict: Py = Py::None;
    let mut verdict_answer: Py = Py::None;
    let mut who: Py = Py::None;
    let mut worth: Py = Py::None;
    let mut years_back: Py = Py::None;
    let mut not_moved: Py = gen_memory::memory::tried::module_obj();
    going_on = mcall(&not_moved.clone(), "reply_for", &[text.clone(), memory.clone()]);
    if py_truthy(&going_on.clone()) {
    return going_on.clone();
    }
    who = gen_compose::compose::who_to_see::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&who.clone()) {
    return who.clone();
    }
    came_back = gen_memory::memory::back_again::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&came_back.clone()) {
    return came_back.clone();
    }
    about_them = gen_memory::memory::their_feeling::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&about_them.clone()) {
    return about_them.clone();
    }
    back_at_nothing = _a_line_pointing_back_at_nothing(&[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), memory.clone()]);
    if py_truthy(&back_at_nothing.clone()) {
    return back_at_nothing.clone();
    }
    against = gen_compose::compose::quantity::the_figure_against_the_answer(&[text.clone(), memory.clone()]);
    if py_truthy(&against.clone()) {
    return against.clone();
    }
    accepted = gen_compose::compose::fallback::a_doable(&[text.clone(), memory.clone()]);
    if py_truthy(&accepted.clone()) {
    return accepted.clone();
    }
    verdict = { let __b = gen_compose::compose::fallback::reads_as_a_verdict(&[text.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_compose::compose::fallback::a_verdict_stands_down(&[memory.clone()]))) } };
    a_comment = gen_compose::compose::fallback::reads_as_a_verdict(&[text.clone(), memory.clone()]);
    carried = if py_truthy(&a_comment.clone()) { Py::None } else { gen_memory::memory::detail::answer_for(&[text.clone(), memory.clone()]) };
    if py_truthy(&carried.clone()) {
    return carried.clone();
    }
    objected = if py_truthy(&a_comment.clone()) { Py::None } else { gen_memory::memory::obstacle::answer_for(&[text.clone(), memory.clone()]) };
    if py_truthy(&objected.clone()) {
    return objected.clone();
    }
    described = gen_compose::compose::no_eyes::what_they_described(&[text.clone(), memory.clone()]);
    if py_truthy(&described.clone()) {
    return { let __b = py_call(&gen_compose::compose::fallback::g__choose(), &[described.clone(), memory.clone(), text.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    }
    reported = gen_memory::memory::detail::a_report_about_the_thing(&[text.clone(), memory.clone()]);
    if py_truthy(&reported.clone()) {
    return reported.clone();
    }
    corrected = gen_memory::memory::matters::a_corrected_age(&[text.clone(), memory.clone()]);
    if py_truthy(&corrected.clone()) {
    return corrected.clone();
    }
    off_to = if py_truthy(&a_comment.clone()) { Py::None } else { gen_compose::compose::moving::reply_for(&[text.clone(), memory.clone()]) };
    if py_truthy(&off_to.clone()) {
    return off_to.clone();
    }
    tasted = gen_compose::compose::taste::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&tasted.clone()) {
    return tasted.clone();
    }
    let mut retone: Py = py_func("retone", |args: &[Py]| gen_compose::compose::tone_again::retone(args));
    retoned = py_call(&retone, &[memory.clone(), text.clone()]);
    if py_truthy(&retoned.clone()) {
    return retoned.clone();
    }
    serious = gen_compose::compose::safety_doubt::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&serious.clone()) {
    return serious.clone();
    }
    reply = py_call(&gen_compose::compose::fallback::g_reply_for(), &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&a_comment.clone()))) {
    in_the_room = gen_compose::compose::no_eyes::a_thing_in_the_room_over_a_placeholder(&[text.clone(), reply.clone()]);
    if py_truthy(&in_the_room.clone()) {
    return { let __b = py_call(&gen_compose::compose::fallback::g__choose(), &[in_the_room.clone(), memory.clone(), text.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    }
    if py_truthy(&{ let __b = gen_compose::compose::no_eyes::_is_a_placeholder(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::is_a_hedge(&[reply.clone()]) } }) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![gen_compose::compose::no_eyes::a_contact_they_handed_over(&[text.clone()]), gen_compose::compose::no_eyes::a_recording_they_offer(&[text.clone()])]));
    for __item in __iter1 {
    pool = __item.clone();
    if py_truthy(&pool.clone()) {
    return { let __b = py_call(&gen_compose::compose::fallback::g__choose(), &[pool.clone(), memory.clone(), text.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    }
    }
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_in_a_pool(&[reply.clone(), py_tuple(vec![gen_compose::compose::fallback::g_A_SMALL_DAY()])]); if py_truthy(&__b) { __b } else { gen_memory::memory::ack::reads_as_a_receipt(&[reply.clone()]) } } } }) {
    of_mine = gen_compose::compose::remark::a_thing_of_theirs_reply_for(&[text.clone(), mcall(&text.clone(), "split", &[]), memory.clone()]);
    if py_truthy(&of_mine.clone()) {
    return of_mine.clone();
    }
    }
    judged_line = gen_compose::compose::said_about_me::reads_as_reported_judgement(&[text.clone()]);
    nothing_yet = { let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_nothing_to_stand_behind(&[reply.clone()]); if py_truthy(&__b) { __b } else { { let __b = judged_line.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::said_about_me::speaks_to_a_self_judgement(&[reply.clone()]) } } } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&a_comment.clone())); if !py_truthy(&__b) { __b } else { nothing_yet.clone() } }) {
    judged = { let __b = gen_compose::compose::said_about_me::answer_for(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::said_about_me::answer_followup(&[text.clone(), memory.clone()]) } };
    if py_truthy(&judged.clone()) {
    return judged.clone();
    }
    }
    not_a_body = gen_compose::compose::breakup::answer_for(&[text.clone(), reply.clone(), memory.clone()]);
    if py_truthy(&not_a_body.clone()) {
    return not_a_body.clone();
    }
    keeping_going = gen_compose::compose::griefplea::reply_for(&[text.clone(), reply.clone(), memory.clone()]);
    if py_truthy(&keeping_going.clone()) {
    return keeping_going.clone();
    }
    reply = first_line_points_back_at_nothing(&[reply.clone(), memory.clone()]);
    years_back = gen_compose::compose::goodnews::reply_for_a_year_ago(&[text.clone(), reply.clone(), memory.clone()]);
    if py_truthy(&years_back.clone()) {
    return years_back.clone();
    }
    stretch = gen_compose::compose::stretches::the_stretch_said_back(&[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&stretch.clone()) {
    return stretch.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&a_comment.clone())); if !py_truthy(&__b) { __b } else { { let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_nothing_to_stand_behind(&[reply.clone()]) } } } }) {
    refused = gen_memory::memory::obstacle::a_move_refused(&[text.clone(), memory.clone()]);
    if py_truthy(&refused.clone()) {
    return refused.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_nothing_to_stand_behind(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } } } }) {
    cannot_use = gen_memory::memory::obstacle::answer_for_a_thing_they_cannot_use(&[text.clone(), memory.clone()]);
    if py_truthy(&cannot_use.clone()) {
    return cannot_use.clone();
    }
    }
    rest = the_step_they_have_not_had(&[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&rest.clone()) {
    return rest.clone();
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_nothing_to_stand_behind(&[reply.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_in_a_pool(&[reply.clone(), py_tuple(vec![gen_compose::compose::fallback::g_A_SMALL_DAY()])]) } } } } } }) {
    let mut _verdict_for: Py = py_func("answer_for", |args: &[Py]| gen_memory::memory::the_verdict::answer_for(args));
    verdict_answer = py_call(&_verdict_for, &[text.clone(), memory.clone()]);
    if py_truthy(&verdict_answer.clone()) {
    return verdict_answer.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    again_asked = gen_memory::memory::repeat::the_same_question_again(&[text.clone(), memory.clone()]);
    if py_truthy(&again_asked.clone()) {
    return again_asked.clone();
    }
    outcome = gen_compose::compose::fallback::an_outcome_asked_about_someone(&[text.clone(), memory.clone()]);
    if py_truthy(&outcome.clone()) {
    return outcome.clone();
    }
    figure = gen_memory::memory::repeat::the_figure_asked(&[text.clone(), memory.clone()]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    figure = gen_memory::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    let mut is_it_worth_it: Py = gen_compose::compose::is_it_worth_it::module_obj();
    worth = mcall(&is_it_worth_it.clone(), "is_it_worth_it", &[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&worth.clone()) {
    return worth.clone();
    }
    }
    back = gen_memory::memory::repeat::asked_back(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = back.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_memory::memory::repeat::carries_the_figure(&[back.clone(), reply.clone()]))) } }) {
    return back.clone();
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    late = gen_memory::memory::too_late::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&late.clone()) {
    return late.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_in_a_pool(&[reply.clone(), py_tuple(vec![gen_compose::compose::fallback::g_NEUTRAL()])]) } } } }) {
    checks = gen_compose::compose::checking::the_checks_already_named(&[text.clone(), memory.clone()]);
    if py_truthy(&checks.clone()) {
    return checks.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    theirs = gen_compose::compose::how_it_went::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    own = gen_compose::compose::own_facts::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&own.clone()) {
    return own.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    past = gen_compose::compose::own_facts::about_earlier_chats(&[text.clone(), memory.clone()]);
    if py_truthy(&past.clone()) {
    return past.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    taught = gen_compose::compose::own_facts::learns_from_the_chat(&[text.clone(), memory.clone()]);
    if py_truthy(&taught.clone()) {
    return taught.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    plan = gen_compose::compose::plans::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&plan.clone()) {
    return plan.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_nothing_to_stand_behind(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::setting_off::is_a_pointer(&[reply.clone()]) } } } } } }) {
    off = gen_compose::compose::setting_off::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&off.clone()) {
    return off.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_nothing_to_stand_behind(&[reply.clone()]) } } } }) {
    gave = if py_truthy(&{ let __b = gen_compose::compose::fallback::_thread_is_about_the_body(&[memory.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_the_thread_is_heavy(&[memory.clone()]) } }) { Py::None } else { gen_compose::compose::how_it_went::the_answer_they_gave(&[text.clone(), memory.clone()]) };
    if py_truthy(&gave.clone()) {
    return gave.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    owned = gen_compose::compose::rule::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&owned.clone()) {
    return owned.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    named = gen_compose::compose::checking::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    glad = gen_compose::compose::pleased::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&glad.clone()) {
    return glad.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    thread = gen_compose::compose::fallback::the_threads_answer(&[text.clone(), memory.clone()]);
    if py_truthy(&thread.clone()) {
    return thread.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { mcall(&reply.clone(), "startswith", &[gen_compose::compose::their_day::g_WHO_TO_ASK()]) } } } }) {
    asked_back = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("care".to_string(), gen_compose::compose::fallback::_the_thread_is_heavy(&[memory.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::their_day::pool_for(args))(&__args) };
    if py_truthy(&asked_back.clone()) {
    return py_call(&gen_compose::compose::fallback::g__choose(), &[asked_back.clone(), memory.clone(), text.clone()]);
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_nothing_to_stand_behind(&[reply.clone()]) } } } }) {
    after = gen_compose::compose::the_turn_after::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&after.clone()) {
    return after.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    own = gen_compose::compose::their_day::own_fact_for(&[text.clone(), memory.clone()]);
    if py_truthy(&own.clone()) {
    return own.clone();
    }
    parts = gen_compose::compose::no_page::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&parts.clone()) {
    return parts.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_an_ask_which_bit(&[reply.clone()]) } } } }) {
    course = gen_compose::compose::the_course_of_it::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&course.clone()) {
    return course.clone();
    }
    }
    deciding = gen_compose::compose::liveask::answer_for(&[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&deciding.clone()) {
    return deciding.clone();
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    one_half = gen_compose::compose::missing_half::the_missing_half(&[text.clone(), memory.clone()]);
    if py_truthy(&one_half.clone()) {
    return one_half.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&a_comment.clone()))) {
    plain = gen_memory::memory::detail::the_plain_ask(&[text.clone(), memory.clone()]);
    if py_truthy(&plain.clone()) {
    return plain.clone();
    }
    }
    on_screen = gen_compose::compose::onscreen::the_answer_on_screen(&[text.clone(), memory.clone()]);
    if py_truthy(&on_screen.clone()) {
    return on_screen.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    let mut asked_about: Py = gen_memory::memory::asked_about::module_obj();
    asked = mcall(&asked_about.clone(), "the_answer_this_chat_gave", &[text.clone(), memory.clone()]);
    if py_truthy(&asked.clone()) {
    return asked.clone();
    }
    }
    if py_truthy(&reply.clone()) {
    not_good = gen_compose::compose::rotten_luck::answer_for(&[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&not_good.clone()) {
    return not_good.clone();
    }
    }
    low_day = gen_compose::compose::nottheirbest::answer_for(&[text.clone(), reply.clone(), memory.clone()]);
    if py_truthy(&low_day.clone()) {
    return low_day.clone();
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gen_compose::compose::fallback::g_UNKNOWN()) { let mut line: Py = __item.clone(); __out.push(slice_or_raise(&line.clone(), &Py::None, &Py::Int(40), &Py::None)); } py_list(__out) })() }, &slice_or_raise(&reply.clone(), &Py::None, &Py::Int(40), &Py::None))) } }) {
    only = gen_compose::compose::only_they_know::answer_for(&[text.clone(), memory.clone()]);
    if py_truthy(&only.clone()) {
    return only.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::a_document_lives_with_an_office::a_map_app_line(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::a_document_lives_with_an_office::a_lost_record(&[text.clone(), reply.clone()]) } } } } } }) {
    held = gen_compose::compose::a_document_lives_with_an_office::answer_for(&[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&held.clone()) {
    return held.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&gen_compose::compose::find_a_place::g_A_NOD(), &reply.clone())) } } } }) {
    here = if py_truthy(&gen_compose::compose::find_a_place::already_a_route(&[reply.clone()])) { Py::None } else { gen_compose::compose::find_a_place::answer_for(&[text.clone(), memory.clone()]) };
    if py_truthy(&here.clone()) {
    return here.clone();
    }
    }
    if py_truthy(&reply.clone()) {
    handed = gen_compose::compose::onmybehalf::answer_for(&[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&handed.clone()) {
    return handed.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    judged = gen_compose::compose::a_place_nobody_can_see::answer_for(&[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&judged.clone()) {
    return judged.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    let mut what_i_do_know: Py = gen_compose::compose::what_i_do_know::module_obj();
    known = mcall(&what_i_do_know.clone(), "reply_for", &[text.clone(), memory.clone(), reply.clone()]);
    if py_truthy(&known.clone()) {
    return known.clone();
    }
    }
    if py_truthy(&gen_memory::memory::repeat::answers_nothing(&[reply.clone()])) {
    part = gen_memory::memory::repeat::the_part_asked(&[text.clone(), memory.clone()]);
    if py_truthy(&part.clone()) {
    return part.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&gen_memory::memory::repeat::the_advice_was_to_get_seen(&[memory.clone()])))) {
    knows = gen_memory::memory::repeat::what_it_knows(&[text.clone(), memory.clone()]);
    if py_truthy(&knows.clone()) {
    return knows.clone();
    }
    }
    }
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    clean = { let __b = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("clean")]) } else { Py::None }; if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]) } };
    thinner = { let __b = call_builtin("bool", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_in_a_pool(&[reply.clone(), py_tuple(vec![gen_compose::compose::fallback::g_NEUTRAL()])]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if !py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::findout::reads_as_busy(&[clean.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_compose::compose::findout::says_where_to_look(&[reply.clone()]))) } } } } } } } };
    if py_truthy(&thinner.clone()) {
    elsewhere = { let __b = gen_compose::compose::findout::pool(&[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::findout::more_about_the_journey(&[clean.clone(), memory.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&elsewhere.clone())); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_in_a_pool(&[reply.clone(), py_tuple(vec![gen_compose::compose::fallback::g_NEUTRAL()])]) } }) {
    elsewhere = gen_compose::compose::findout::pool_over_a_receipt(&[clean.clone()]);
    }
    if py_truthy(&elsewhere.clone()) {
    return py_call(&gen_compose::compose::fallback::g__choose(), &[elsewhere.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    looked = gen_compose::compose::findout::on_the_thing(&[text.clone(), memory.clone(), mcall(&memory.clone(), "get", &[py_str("understood")])]);
    if py_truthy(&looked.clone()) {
    return looked.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::gap::names_only_a_page(&[reply.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&gen_compose::compose::gap::names_the_paper(&[reply.clone()]))) } } } }) {
    gap_lines = gen_compose::compose::gap::pool(&[clean.clone()]);
    if py_truthy(&gap_lines.clone()) {
    return py_call(&gen_compose::compose::fallback::g__choose(), &[gap_lines.clone(), memory.clone(), clean.clone()]);
    }
    }
    a_place = { let __b = gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::_in_a_pool(&[reply.clone(), py_tuple(vec![gen_compose::compose::fallback::g_NEUTRAL(), gen_compose::compose::fallback::g_A_SMALL_DAY()])]); if py_truthy(&__b) { __b } else { gen_memory::memory::ack::reads_as_a_receipt(&[reply.clone()]) } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&a_place.clone())); if !py_truthy(&__b) { __b } else { gen_compose::compose::hands::writes_the_words(&[text.clone()]) } }) {
    a_place = gen_compose::compose::fallback::_in_a_pool(&[reply.clone(), py_tuple(vec![gen_compose::compose::fallback::g_ACTS_FOR_ME()])]);
    }
    if py_truthy(&a_place.clone()) {
    hands_reply = gen_compose::compose::hands::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&hands_reply.clone()) {
    return hands_reply.clone();
    }
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { gen_compose::compose::fallback::_is_a_hedge(&[reply.clone()]) } }) {
    own_life = gen_compose::compose::their_life::reply_for(&[text.clone(), memory.clone()]);
    if py_truthy(&own_life.clone()) {
    return own_life.clone();
    }
    }
    return reply.clone();
    Py::None
}
pub fn first_line_points_back_at_nothing(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut kept: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut sentences: Py = Py::None;
    let _ = py_str("The reply, with any \"where were we\" taken out when nothing has been said before it.\n\n    A chat's first line has no earlier thread: an apology for being quiet (\"sorry ive been quiet,\n    its been a rough week\") is the first thing the person has said, and \"Where were we?\" asks them\n    to resume a conversation that does not exist. Only a reply whose last sentence points back is\n    touched, only when the bot has not answered anything yet, and the rest of the reply is kept as\n    it is, so a card that answers something keeps its whole answer.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    memory = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) };
    said_before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_before.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&reply.clone(), "strip", &[])])) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[])) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&sentences.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__POINTS_BACK(), "match", &[mcall(&geti(&sentences.clone(), &op_neg(&Py::Int(1))), "strip", &[])]))) } }) {
    return reply.clone();
    }
    kept = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&sentences.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[])) {  __out.push(one.clone()); } } py_list(__out) })() }]);
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    return g__GO_ON();
    }
    return if py_truthy(&op_gt(&call_builtin("len", &[mcall(&kept.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))) { kept.clone() } else { op_add(&op_add(&kept.clone(), &py_str(" ")), &g__GO_ON()) };
    Py::None
}
pub fn sentences_of(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut out: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The answer's sentences, with a boundary inside an open quotation left alone.\n\n    A card often quotes the words a person can say (\"Say it as a preference and not as a\n    rejection, once, warmly: 'I love that you think of me - could we do a coffee and a walk\n    instead of a top? I only wear two of them.' Then make it easy for her...\"). The question mark\n    is inside the quotation, so the card's sentence runs on past it, and splitting there handed\n    the person \"I only wear two of them.' Then make it easy for her...\" - a leftover that opens\n    mid-quote, which can never read right.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    out = __unpk1[0].clone();
    start = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_SENTENCE(), "finditer", &[answer.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    piece = slice_or_raise(&answer.clone(), &start.clone(), &mcall(&match_.clone(), "start", &[]), &Py::None);
    if py_truthy(&op_modulo(&call_builtin("len", &[mcall(&g_QUOTE_MARK(), "findall", &[piece.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[mcall(&piece.clone(), "strip", &[])]);
    start = mcall(&match_.clone(), "end", &[]);
    }
    let _ = mcall(&out.clone(), "append", &[mcall(&slice_or_raise(&answer.clone(), &start.clone(), &Py::None, &Py::None), "strip", &[])]);
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _a_step_of_this_answer(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut restarting: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the step they report is one of the answer's own, so its next step is theirs.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PLAIN(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(slice_or_raise(&word.clone(), &Py::None, &Py::Int(5), &Py::None)); } py_set(__out) })() };
    restarting = { let __b = call_builtin("bool", &[mcall(&g_RESTART(), "search", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_RESTART(), "search", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)) } };
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&theirs.clone(), &slice_or_raise(&word.clone(), &Py::None, &Py::Int(5), &Py::None))); if py_truthy(&__b) { __b } else { { let __b = restarting.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_RESTART(), "search", &[word.clone()]) } } } }) {
    continue;
    }
    return Py::Bool(false);
    }
    return call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_plan_for_later(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let _ = py_str("True when the line says what they will do and when, rather than how things are.\n\n    Both halves are needed. A time on its own is a detail (\"it started in the garden on\n    saturday\") and a doing verb on its own is not ahead of them (\"check the queue\") - it is the\n    two together, with the doing not yet in the past, that puts the line in the future: \"im\n    taking it back tomorrow\", \"i'll do it at the weekend\", \"get someone in next week\".\n    ");
    if py_truthy(&mcall(&g_DONE_A_STEP(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_TIME_AHEAD(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_A_DOING_SOON(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    doing = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PAST_TENSE(), "search", &[mcall(&doing.clone(), "group", &[Py::Int(0)])])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _answers_the_line(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut turns: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { py_tuple(vec![]) };
    let mut asked_before: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let _ = py_str("True when the line is a detail of that answer rather than news of its own.\n\n    A place or a thing named, a pronoun said about it, or a step reported as done is about the\n    answer whatever the answer is. A fact of theirs (\"my review is next month\") is only about\n    the answer when the answer is about the thing it names: \"my daughter is allergic to milk\"\n    came after an answer about nuts, and the leftover of that answer - \"check the labels on\n    chocolate, muesli, pesto and satay, and anything marked 'may contain nuts'\" - was handed back\n    as advice about milk, which is a wrong fact about a child's allergy. Nothing in the answer was\n    about the daughter or the milk, so the line is news and keeps the composer's own reply.\n    ");
    if py_truthy(&{ let __b = mcall(&g_A_DETAIL_OF_IT(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DONE_A_STEP(), "search", &[text.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_FACT_OF_THEIRS(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    named = call_builtin("set", &[gen_compose::compose::fallback::content_words(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&named.clone(), &call_builtin("set", &[gen_compose::compose::fallback::content_words(&[answer.clone()])]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(true);
    }
    asked_before = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    return Py::Bool(!py_truthy(&op_bitand(&named.clone(), &call_builtin("set", &[gen_compose::compose::fallback::content_words(&[asked_before.clone()])]).unwrap_or_else(|e| exc::raise(e)))));
    Py::None
}
pub fn _said_back(args: &[Py]) -> Py {
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
    let mut mine: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("True when the reply is the person's own words back, with no claim added.\n\n    The composer's ack (\"Got it - you've turned it off and on\") keeps the person's words and\n    adds a receipt, so the test is on the person's words: three quarters of what they said is in\n    the reply, and at least two of their words are. A reply that says something else shares\n    nothing with the line, which is how a real answer is told from a receipt.\n    ");
    mine = gen_compose::compose::fallback::_the_words(&[text.clone()]);
    theirs = gen_compose::compose::fallback::_the_words(&[said.clone()]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&theirs.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&gen_compose::compose::fallback::_is_a_hedge(&[said.clone()])) {
    return Py::Bool(false);
    }
    shared = op_bitand(&mine.clone(), &theirs.clone());
    return { let __b = op_ge(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { op_ge(&op_truediv(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Float(0.6)) } };
    Py::None
}
pub fn the_step_they_have_not_had(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut a_step_done: Py = Py::None;
    let mut after: Py = Py::None;
    let mut an_echo: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut candidates: Py = Py::None;
    let mut did_a_step: Py = Py::None;
    let mut leftover: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut step: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The part of the last answer the person has not had yet, or \"\" when there is none.\n\n    \"the wifi keeps dropping in the back bedroom\" is answered with two sentences, the second of\n    which is the branch for one room still dropping; \"its fine in the kitchen\" then got the\n    composer's echo back - \"Got it - it's fine in the kitchen\" - with the branch still on the\n    screen. The person has answered the answer, so the reply is the answer's second sentence, not\n    their own words.\n\n    It is asked only of a reply that is an echo: a module that can answer for itself has already\n    answered, and a real line from the composer is never replaced. The line has to be short and\n    not a question, and the leftover has to hold a step, so a sign-off is not said back. A line\n    that says what they will do about it, and when, is a plan for later: \"call the plumber on\n    monday\" has read the answer, so the rest of it is not what they are waiting for.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    an_echo = _said_back(&[text.clone(), said.clone()]);
    a_step_done = { let __b = call_builtin("bool", &[mcall(&g_DONE_A_STEP(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { py_call(&gen_compose::compose::fallback::g__a_small_day(), &[text.clone(), memory.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = an_echo.clone(); if py_truthy(&__b) { __b } else { a_step_done.clone() } })) } }) {
    return py_str("");
    }
    if py_truthy(&gen_compose::compose::fallback::is_a_forecast(&[said.clone()])) {
    return py_str("");
    }
    if py_truthy(&gen_compose::compose::fallback::is_a_forecast(&[said.clone()])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = a_step_done.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&an_echo.clone())) } })) } }) {
    return py_str("");
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("?"))) } } } }) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(one.clone()); } } py_list(__out) })() };
    answer = if py_truthy(&turns.clone()) { mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_answers_the_line(&[text.clone(), answer.clone(), turns.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&_the_answer_again(&[answer.clone(), turns.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_PUSHING_BACK(), "search", &[text.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_NEW_FACT_ABOUT_SOMEONE(), "search", &[text.clone()])) {
    return py_str("");
    }
    if py_truthy(&_a_plan_for_later(&[text.clone()])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = gen_compose::compose::fallback::reads_as_a_verdict(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { gen_compose::compose::fallback::a_verdict_stands_down(&[memory.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = a_step_done.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&an_echo.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_step_of_this_answer(&[text.clone(), answer.clone()]))) } } } }) {
    return py_str("");
    }
    sentences = sentences_of(&[answer.clone()]);
    candidates = py_list(vec![]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[sentences.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = a_step_done.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&an_echo.clone())) } })) } }) {
    let _ = mcall(&candidates.clone(), "append", &[mcall(&py_str(" "), "join", &[slice_or_raise(&sentences.clone(), &Py::Int(1), &Py::Int(3), &Py::None)])]);
    }
    did_a_step = call_builtin("bool", &[mcall(&g_DONE_A_STEP(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_THEN_TAIL(), "finditer", &[answer.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&did_a_step.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_said_back(&[text.clone(), slice_or_raise(&answer.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]))) } }) {
    continue;
    }
    after = mcall(&slice_or_raise(&answer.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&after.clone()) {
    let _ = mcall(&candidates.clone(), "append", &[op_add(&mcall(&geti(&after.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    }
    break;
    }
    let __iter2 = py_iter_or_raise(&candidates.clone());
    for __item in __iter2 {
    leftover = __item.clone();
    low = mcall(&leftover.clone(), "lower", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&leftover.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { mcall(&low.clone(), "startswith", &[g_SIGNING_OFF()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&gen_memory::memory::detail::stands_alone(&[leftover.clone()])))) {
    continue;
    }
    if py_truthy(&mcall(&mcall(&leftover.clone(), "rstrip", &[]), "endswith", &[py_str("?")])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_STEP_IN_IT()) { let mut step: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &step.clone()), &py_str("\\w*")), low.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    return leftover.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_answer_again(args: &[Py]) -> Py {
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
    let mut turns: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut earlier: Py = Py::None;
    let mut low: Py = Py::None;
    let mut one: Py = Py::None;
    let _ = py_str("True when the last reply is an earlier answer of the bot's, said again.\n\n    The memory store hands the last answer back with a short lead-in (\"Sure - ...\") when the person\n    asks to hear it again, and nothing in that is un-had: it is the answer they have just read.\n    After \"come again?\" over the Lisbon answer, the next line - a new fact about Sintra - was\n    answered with \"May is warm and busy, so book the tower and the popular restaurants ahead\", the\n    tail of the answer they had been given a moment before, instead of the fact being taken down.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None));
    for __item in __iter1 {
    one = __item.clone();
    earlier = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[mcall(&earlier.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &earlier.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let _ = py_str("True when the composer's own line is the answer, whatever else proposed.");
    return { let __b = gen_compose::compose::fallback::reads_as_about_itself(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::reads_as_a_tight_week(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::a_continuation_of_the_tight_week(&[memory.clone(), text.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::fallback::reads_as_small_talk(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&gen_compose::compose::quantity::the_figure_against_the_answer(&[text.clone(), memory.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::moving::reads_as_a_move_cheered(&[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = gen_compose::compose::who_to_see::replies_to_a_who_to_see(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_call(&gen_compose::compose::fallback::g_TAKES_AS_IT_COMES(), &[text.clone(), memory.clone()]) } } } } } } } } } } } } } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.greeting".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_DETAIL_OF_IT" => Some(g_A_DETAIL_OF_IT()),
            "A_DOING_SOON" => Some(g_A_DOING_SOON()),
            "A_FACT_OF_THEIRS" => Some(g_A_FACT_OF_THEIRS()),
            "A_NEW_FACT_ABOUT_SOMEONE" => Some(g_A_NEW_FACT_ABOUT_SOMEONE()),
            "A_REFERS_BACK" => Some(g_A_REFERS_BACK()),
            "A_STEP_IN_IT" => Some(g_A_STEP_IN_IT()),
            "A_TIME_AHEAD" => Some(g_A_TIME_AHEAD()),
            "DONE_A_STEP" => Some(g_DONE_A_STEP()),
            "NOTHING_BEFORE_IT" => Some(g_NOTHING_BEFORE_IT()),
            "PAST_TENSE" => Some(g_PAST_TENSE()),
            "PLAIN" => Some(g_PLAIN()),
            "PRIORITY" => Some(g_PRIORITY()),
            "PUSHING_BACK" => Some(g_PUSHING_BACK()),
            "QUOTE_MARK" => Some(g_QUOTE_MARK()),
            "RESTART" => Some(g_RESTART()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SIGNING_OFF" => Some(g_SIGNING_OFF()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "THEN_TAIL" => Some(g_THEN_TAIL()),
            "_GO_ON" => Some(g__GO_ON()),
            "_POINTS_BACK" => Some(g__POINTS_BACK()),
            "_a_line_pointing_back_at_nothing" => Some(py_func("_a_line_pointing_back_at_nothing", |args: &[Py]| _a_line_pointing_back_at_nothing(args))),
            "_a_plan_for_later" => Some(py_func("_a_plan_for_later", |args: &[Py]| _a_plan_for_later(args))),
            "_a_step_of_this_answer" => Some(py_func("_a_step_of_this_answer", |args: &[Py]| _a_step_of_this_answer(args))),
            "_answers_the_line" => Some(py_func("_answers_the_line", |args: &[Py]| _answers_the_line(args))),
            "_said_back" => Some(py_func("_said_back", |args: &[Py]| _said_back(args))),
            "_the_answer_again" => Some(py_func("_the_answer_again", |args: &[Py]| _the_answer_again(args))),
            "first_line_points_back_at_nothing" => Some(py_func("first_line_points_back_at_nothing", |args: &[Py]| first_line_points_back_at_nothing(args))),
            "respond" => Some(g_respond()),
            "sentences_of" => Some(py_func("sentences_of", |args: &[Py]| sentences_of(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            "the_step_they_have_not_had" => Some(py_func("the_step_they_have_not_had", |args: &[Py]| the_step_they_have_not_had(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_DETAIL_OF_IT" => s_A_DETAIL_OF_IT(v),
            "A_DOING_SOON" => s_A_DOING_SOON(v),
            "A_FACT_OF_THEIRS" => s_A_FACT_OF_THEIRS(v),
            "A_NEW_FACT_ABOUT_SOMEONE" => s_A_NEW_FACT_ABOUT_SOMEONE(v),
            "A_REFERS_BACK" => s_A_REFERS_BACK(v),
            "A_STEP_IN_IT" => s_A_STEP_IN_IT(v),
            "A_TIME_AHEAD" => s_A_TIME_AHEAD(v),
            "DONE_A_STEP" => s_DONE_A_STEP(v),
            "NOTHING_BEFORE_IT" => s_NOTHING_BEFORE_IT(v),
            "PAST_TENSE" => s_PAST_TENSE(v),
            "PLAIN" => s_PLAIN(v),
            "PRIORITY" => s_PRIORITY(v),
            "PUSHING_BACK" => s_PUSHING_BACK(v),
            "QUOTE_MARK" => s_QUOTE_MARK(v),
            "RESTART" => s_RESTART(v),
            "SENTENCE" => s_SENTENCE(v),
            "SIGNING_OFF" => s_SIGNING_OFF(v),
            "TAKES_AS_IT_COMES" => s_TAKES_AS_IT_COMES(v),
            "THEN_TAIL" => s_THEN_TAIL(v),
            "_GO_ON" => s__GO_ON(v),
            "_POINTS_BACK" => s__POINTS_BACK(v),
            "respond" => s_respond(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.greeting", module_obj);
    pyrt::xrefs::reg_fn("modules.greeting", "_a_line_pointing_back_at_nothing", py_func("_a_line_pointing_back_at_nothing", |args: &[Py]| _a_line_pointing_back_at_nothing(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "respond", py_func("respond", |args: &[Py]| py_call(&g_respond(), args)));
    pyrt::xrefs::reg_fn("modules.greeting", "first_line_points_back_at_nothing", py_func("first_line_points_back_at_nothing", |args: &[Py]| first_line_points_back_at_nothing(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "sentences_of", py_func("sentences_of", |args: &[Py]| sentences_of(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "_a_step_of_this_answer", py_func("_a_step_of_this_answer", |args: &[Py]| _a_step_of_this_answer(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "_a_plan_for_later", py_func("_a_plan_for_later", |args: &[Py]| _a_plan_for_later(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "_answers_the_line", py_func("_answers_the_line", |args: &[Py]| _answers_the_line(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "_said_back", py_func("_said_back", |args: &[Py]| _said_back(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "the_step_they_have_not_had", py_func("the_step_they_have_not_had", |args: &[Py]| the_step_they_have_not_had(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "_the_answer_again", py_func("_the_answer_again", |args: &[Py]| _the_answer_again(args)));
    pyrt::xrefs::reg_fn("modules.greeting", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_access("modules.greeting", "A_DETAIL_OF_IT", g_A_DETAIL_OF_IT, s_A_DETAIL_OF_IT);
    pyrt::xrefs::reg_access("modules.greeting", "A_DOING_SOON", g_A_DOING_SOON, s_A_DOING_SOON);
    pyrt::xrefs::reg_access("modules.greeting", "A_FACT_OF_THEIRS", g_A_FACT_OF_THEIRS, s_A_FACT_OF_THEIRS);
    pyrt::xrefs::reg_access("modules.greeting", "A_NEW_FACT_ABOUT_SOMEONE", g_A_NEW_FACT_ABOUT_SOMEONE, s_A_NEW_FACT_ABOUT_SOMEONE);
    pyrt::xrefs::reg_access("modules.greeting", "A_REFERS_BACK", g_A_REFERS_BACK, s_A_REFERS_BACK);
    pyrt::xrefs::reg_access("modules.greeting", "A_STEP_IN_IT", g_A_STEP_IN_IT, s_A_STEP_IN_IT);
    pyrt::xrefs::reg_access("modules.greeting", "A_TIME_AHEAD", g_A_TIME_AHEAD, s_A_TIME_AHEAD);
    pyrt::xrefs::reg_access("modules.greeting", "DONE_A_STEP", g_DONE_A_STEP, s_DONE_A_STEP);
    pyrt::xrefs::reg_access("modules.greeting", "NOTHING_BEFORE_IT", g_NOTHING_BEFORE_IT, s_NOTHING_BEFORE_IT);
    pyrt::xrefs::reg_access("modules.greeting", "PAST_TENSE", g_PAST_TENSE, s_PAST_TENSE);
    pyrt::xrefs::reg_access("modules.greeting", "PLAIN", g_PLAIN, s_PLAIN);
    pyrt::xrefs::reg_access("modules.greeting", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.greeting", "PUSHING_BACK", g_PUSHING_BACK, s_PUSHING_BACK);
    pyrt::xrefs::reg_access("modules.greeting", "QUOTE_MARK", g_QUOTE_MARK, s_QUOTE_MARK);
    pyrt::xrefs::reg_access("modules.greeting", "RESTART", g_RESTART, s_RESTART);
    pyrt::xrefs::reg_access("modules.greeting", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("modules.greeting", "SIGNING_OFF", g_SIGNING_OFF, s_SIGNING_OFF);
    pyrt::xrefs::reg_access("modules.greeting", "TAKES_AS_IT_COMES", g_TAKES_AS_IT_COMES, s_TAKES_AS_IT_COMES);
    pyrt::xrefs::reg_access("modules.greeting", "THEN_TAIL", g_THEN_TAIL, s_THEN_TAIL);
    pyrt::xrefs::reg_access("modules.greeting", "_GO_ON", g__GO_ON, s__GO_ON);
    pyrt::xrefs::reg_access("modules.greeting", "_POINTS_BACK", g__POINTS_BACK, s__POINTS_BACK);
    pyrt::xrefs::reg_access("modules.greeting", "respond", g_respond, s_respond);
}
