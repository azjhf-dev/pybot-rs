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


thread_local! { static G_CALENDAR_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLANNING_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_A_DEATH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLAUSE_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIRST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LENGTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DENIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MARRYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEW_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ORDER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PART_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLANNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPAN_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TURNED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORK_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_CALENDAR_NOW() -> Py { ensure_init(); G_CALENDAR_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_NOW(v: Py) { ensure_init(); G_CALENDAR_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_PLANNING_KEYS() -> Py { ensure_init(); G_PLANNING_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLANNING_KEYS(v: Py) { ensure_init(); G_PLANNING_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_A_DEATH() -> Py { ensure_init(); G__ABOUT_A_DEATH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_A_DEATH(v: Py) { ensure_init(); G__ABOUT_A_DEATH.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKING() -> Py { ensure_init(); G__ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING(v: Py) { ensure_init(); G__ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_FOR() -> Py { ensure_init(); G__ASK_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_FOR(v: Py) { ensure_init(); G__ASK_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_WORDS() -> Py { ensure_init(); G__ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_WORDS(v: Py) { ensure_init(); G__ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLAUSE_BREAK() -> Py { ensure_init(); G__A_CLAUSE_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLAUSE_BREAK(v: Py) { ensure_init(); G__A_CLAUSE_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIRST_ASK() -> Py { ensure_init(); G__A_FIRST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIRST_ASK(v: Py) { ensure_init(); G__A_FIRST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LENGTH_ASK() -> Py { ensure_init(); G__A_LENGTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LENGTH_ASK(v: Py) { ensure_init(); G__A_LENGTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__DENIED() -> Py { ensure_init(); G__DENIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DENIED(v: Py) { ensure_init(); G__DENIED.with(|c| *c.borrow_mut() = v); }
pub fn g__MARRYING() -> Py { ensure_init(); G__MARRYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MARRYING(v: Py) { ensure_init(); G__MARRYING.with(|c| *c.borrow_mut() = v); }
pub fn g__MOVING() -> Py { ensure_init(); G__MOVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOVING(v: Py) { ensure_init(); G__MOVING.with(|c| *c.borrow_mut() = v); }
pub fn g__NEW_JOB() -> Py { ensure_init(); G__NEW_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEW_JOB(v: Py) { ensure_init(); G__NEW_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g__ORDER_WORDS() -> Py { ensure_init(); G__ORDER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ORDER_WORDS(v: Py) { ensure_init(); G__ORDER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PART_WORDS() -> Py { ensure_init(); G__PART_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PART_WORDS(v: Py) { ensure_init(); G__PART_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PLANNING() -> Py { ensure_init(); G__PLANNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLANNING(v: Py) { ensure_init(); G__PLANNING.with(|c| *c.borrow_mut() = v); }
pub fn g__PLAN_ASK() -> Py { ensure_init(); G__PLAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLAN_ASK(v: Py) { ensure_init(); G__PLAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__SELF() -> Py { ensure_init(); G__SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SELF(v: Py) { ensure_init(); G__SELF.with(|c| *c.borrow_mut() = v); }
pub fn g__SPAN_HEAD() -> Py { ensure_init(); G__SPAN_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPAN_HEAD(v: Py) { ensure_init(); G__SPAN_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__TURNED() -> Py { ensure_init(); G__TURNED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TURNED(v: Py) { ensure_init(); G__TURNED.with(|c| *c.borrow_mut() = v); }
pub fn g__WHO() -> Py { ensure_init(); G__WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHO(v: Py) { ensure_init(); G__WHO.with(|c| *c.borrow_mut() = v); }
pub fn g__WORK_WHEN() -> Py { ensure_init(); G__WORK_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORK_WHEN(v: Py) { ensure_init(); G__WORK_WHEN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A plan announced is the question the plan implies.\n\nPeople do not ask for help with a plan, they announce it:\n\n    im planning a trip           -> Noted: you're planning a trip.\n    im getting married in june   -> Noted: you're getting married in june.\n    im moving house next month   -> Noted: you're moving house next month.\n    i start a new job on monday  -> Noted: you start a new job on monday.\n\nFiling it is right - the follow-up turns need the plan remembered - but it is not an answer, and\nthe knowledge base already holds the answer to the question the announcement implies:\n\n    how do i plan a trip             -> Two months out: check the passport has six months left...\n    how do i plan a wedding          -> Guest list first, because it sets the cost of everything...\n    what do i need to do to move house -> Do the paper trail in one afternoon: tell the council...\n    first day at a new job           -> Dress one step smarter than the job's everyday clothes...\n\nSo the announcement is read as that question, and answered from the entry it reaches. Two rules\nkeep it honest: the line must be the person's own plan, said as a statement (not a question, not a\nplan they are arguing with, not somebody else's), and the rewrite must reach an entry - a plan the\nknowledge base has no answer for is left to the reply that files it, exactly as before.\n");
    s_CALENDAR_NOW(crate::memory::when::g_CALENDAR_NOW());
    s__SELF(py_str("^(?:(?:so|ok|okay|and|also|well|right|hmm)[, ]+)*"));
    s__WHO(py_str("(?:i'm|im|i am|we're|we are|(?:me|i) and [a-z]+ (?:are|'re)|[a-z]+ and i (?:are|'re))"));
    s__PLANNING(stdlib_call("re", "compile", &[op_add(&op_add(&g__SELF(), &g__WHO()), &py_str("\\s+(?:planning|sorting out|organising|organizing|arranging)\\b(?:\\s+(?:a|an|the|my|our|some))?\\s+(?P<thing>[a-z][a-z ']{2,40})")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MOVING(stdlib_call("re", "compile", &[op_add(&op_add(&g__SELF(), &g__WHO()), &py_str("\\s+moving\\s+(?:house|home|flat|out|in\\b|to\\s+[a-z]+|next|soon|this|in\\s)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MARRYING(stdlib_call("re", "compile", &[op_add(&op_add(&g__SELF(), &g__WHO()), &py_str("\\s+getting married\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NEW_JOB(stdlib_call("re", "compile", &[op_add(&g__SELF(), &py_str("(?:i'm|im|i am|i|we're|we are|we)?\\s*(?:start|starts|starting|begin|beginning)\\s+(?:a |my |our )?(?:new )?(?P<what>job|work)\\b(?P<after>.*)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WORK_WHEN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s*(?:(?:on|by)\\s+(?:the\\s+)?(?:\\d+|first|last|"), &mcall(&py_str("|"), "join", &[py_list(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("mon"), py_str("tues"), py_str("tue"), py_str("weds"), py_str("wed"), py_str("thurs"), py_str("thu"), py_str("thur"), py_str("fri"), py_str("sat"), py_str("sun")])])), &py_str(")(?:st|nd|rd|th)?|next\\s+(?:week|month|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|tomorrow|today|this\\s+(?:week|month|monday|tuesday|wednesday|thursday|friday|saturday|sunday))\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_FOR(py_tuple(vec![py_tuple(vec![gattr(&g__MARRYING(), "search"), py_str("how do i plan a wedding")]), py_tuple(vec![gattr(&g__MOVING(), "search"), py_str("what do i need to do to move house")]), py_tuple(vec![g__a_first_day(), py_str("first day at a new job")])]));
    s__ASKING(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:what|how|when|where|which|who|why|should|shall|can|could|would|do i need|do we need|any (?:tips|ideas|advice))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TURNED(stdlib_call("re", "compile", &[py_str("\\bbut\\b|\\bthough\\b|\\bexcept\\b|\\bonly\\b|\\bhowever\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DENIED(stdlib_call("re", "compile", &[py_str("\\b(?:not|never|no longer|arent|isnt|wasnt|dont|doesnt)\\b|n't\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ABOUT_A_DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:grave|graves|graveside|cemetery|ashes|funeral|funerals|burial|buried|cremation|cremated|crematorium|memorial|hospice|dying|died|death|passed away|say goodbye|last goodbye)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLANNING_KEYS(call_builtin("frozenset", &[mcall(&py_str("\nplan plans planning planned prepare preparing preparation organise organises organising\norganize organized organizing arrange arranging countdown checklist need needs booking book\nbudget budgeting\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__PART_WORDS(call_builtin("frozenset", &[mcall(&py_str("\na an the and or but if so to of in on at for with about from by as is are was were be been do\ndoes did doing done i im me my we our you your it its that this these those what which how when\nwhere who why should could would can will shall may might must not no yes please just really\nany some more most first next then now again also too very get got getting go going want need\nneed to know tell say said help good best way ways thing things\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_LENGTH_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long\\b|\\bhow soon\\b|\\bhow far ahead\\b|\\bhow many (?:weeks|days|months|hours)\\b|\\bwhat order\\b|\\bbefore i go\\b|\\btimeline\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SPAN_HEAD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[py_list(vec![py_str("\\d+"), py_str("a"), py_str("an"), py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("eleven"), py_str("twelve")])])), &py_str(")\\s+(?:day|week|month|year)s?\\s+(?:out|before|ahead)\\b|\\bthe (?:day|week|month|year) before\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CLAUSE_BREAK(stdlib_call("re", "compile", &[py_str("(?:,|;|\\s-\\s|:\\s)")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("long soon far ahead many much weeks week days day months month hours\n    hour minutes minute time take takes last before order timeline go going get"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FIRST_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what(?:'s|s| is)? (?:the )?(?:first|next)|what (?:should |will |does )?comes? first|what do i do(?: first)?|what should i do(?: first)?|where (?:do|should) i (?:start|begin)|how do i (?:start|begin)|first (?:thing|step|bit)|what first)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ORDER_WORDS(call_builtin("frozenset", &[mcall(&py_str("first comes start begin next thing things step steps order point\n    anything else now again should could would can will might must the and then so about\n    whats what come"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__PLAN_ASK(stdlib_call("re", "compile", &[py_str("how\\s+(?:do|can|should)\\s+i\\s+(?:plan|prepare for|get ready for|start planning)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_first_day(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("True when the line is a first day at a new job, not a shift or a job on the house.");
    found = mcall(&g__NEW_JOB(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[py_str("what")]), "lower", &[]), &py_str("job")))) {
    return Py::Bool(true);
    }
    after = stdlib_call("re", "sub", &[py_str("[\\s.,!?;:'\\\"()]"), py_str(""), { let __b = mcall(&found.clone(), "group", &[py_str("after")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g__WORK_WHEN(), "match", &[{ let __b = mcall(&found.clone(), "group", &[py_str("after")]); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn as_question(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut question: Py = Py::None;
    let _ = py_str("The question a plan announcement implies, or None when the line is not one.");
    let __unpk1 = py_unpack(&_implied(&[text.clone()]), 2, None);
    question = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    return question.clone();
    Py::None
}
pub fn _implied(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut found: Py = Py::None;
    let mut test: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The question the line implies, and whether it is one of the kinds read by hand.\n\n    A kind read by hand names its own entry (\"what do i need to do to move house\"), so that entry\n    answers it. The general kind (\"im planning a kitchen\") only has \"how do i plan a ...\", and the\n    entry that reaches can be about the thing rather than about planning it: kitchen-routine\n    cleans a kitchen, and the person was talking about planning one.\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ASKING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__TURNED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__DENIED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__ABOUT_A_DEATH(), "search", &[clean.clone()]) } } } } } } } }) {
    return py_tuple(vec![Py::None, Py::Bool(false)]);
    }
    let __iter1 = py_iter_or_raise(&g__ASK_FOR());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    test = __unpk2[0].clone();
    ask = __unpk2[1].clone();
    if py_truthy(&py_call(&test, &[clean.clone()])) {
    return py_tuple(vec![ask.clone(), Py::Bool(true)]);
    }
    }
    found = mcall(&g__PLANNING(), "search", &[clean.clone()]);
    if py_truthy(&found.clone()) {
    thing = mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "strip", &[py_str(" .!,")]);
    if py_truthy(&mcall(&thing.clone(), "startswith", &[py_str("to ")])) {
    return py_tuple(vec![Py::None, Py::Bool(false)]);
    }
    thing = mcall(&stdlib_call("re", "sub", &[py_str("\\s+(?:next|this|in|on|for)\\s+\\w+.*$"), py_str(""), thing.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&thing.clone()) {
    if py_truthy(&mcall(&thing.clone(), "startswith", &[py_tuple(vec![py_str("my "), py_str("our "), py_str("the ")])])) {
    return py_tuple(vec![op_add(&py_str("how do i plan "), &thing.clone()), Py::Bool(false)]);
    }
    return py_tuple(vec![op_add(&py_str("how do i plan a "), &thing.clone()), Py::Bool(false)]);
    }
    }
    return py_tuple(vec![Py::None, Py::Bool(false)]);
    Py::None
}
pub fn _the_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entry: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut question: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "question") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: question"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("The entry's answer said, the way the composer will say it.\n\n    An entry's reply is a string, a list of phrasings or a function of the line and the store\n    (`bot/compose`), and the last two are most of the knowledge base. Reading only the string left\n    every such entry looking as if it had no answer at all, so a plan announced reached nothing\n    and fell back to \"Noted: ...\" on any main where the entry had moved to a function.\n    ");
    reply = call_builtin("getattr", &[entry.clone(), py_str("reply"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __try1 = exc::catch(|| {
    reply = py_call(&reply, &[if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, gen_understanding::understanding::understand(&[question.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }])]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    } else {
    if py_truthy(&call_builtin("isinstance", &[reply.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    reply = pyrt::xrefs::xcall("compose", "pick", &[reply.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }]);
    }
    }
    return if py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reply.clone() } else { py_str("") };
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    let mut by_hand: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut question: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The entry's answer to the question the plan implies, or None to leave the line as it was.");
    let __unpk1 = py_unpack(&_implied(&[text.clone()]), 2, None);
    question = __unpk1[0].clone();
    by_hand = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&question.clone()))) {
    return Py::None;
    }
    let __try2 = exc::catch(|| {
    entry = pyrt::xrefs::xcall("knowledge.lookup", "find", &[gen_understanding::understanding::understand(&[question.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }])]);
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
    return Py::None;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&Py::Bool(py_is(&entry.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&by_hand.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_about_planning(&[entry.clone()]))) } }) {
    return Py::None;
    }
    reply = _the_reply(&[entry.clone(), question.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&slice_or_raise(&reply.clone(), &Py::None, &Py::Int(1), &Py::None), "islower", &[])) {
    return Py::None;
    }
    return mcall(&reply.clone(), "strip", &[]);
    Py::None
}
pub fn _about_planning(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entry: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut part: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the entry was written for a question about planning the thing.");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[entry.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    piece = __item.clone();
    let __iter3 = py_iter_or_raise(&mcall(&mcall(&mcall(&call_builtin("str", &[piece.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("'"), py_str("")]), "split", &[]));
    for __item in __iter3 {
    word = __item.clone();
    let _ = mcall(&words.clone(), "add", &[word.clone()]);
    }
    }
    }
    return call_builtin("bool", &[op_bitand(&words.clone(), &g_PLANNING_KEYS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_clauses(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut part: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",\\s*|;\\s*|\\s+and\\s+"), detail.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[py_str(" .;")])) {  __out.push(mcall(&part.clone(), "strip", &[py_str(" .;")])); } } py_list(__out) })() };
    Py::None
}
pub fn _the_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut first_only: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "first_only") { v.clone() }
    else { Py::Bool(false) };
    let mut best: Py = Py::None;
    let mut best_hit: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut hit: Py = Py::None;
    let _ = py_str("The clause of a sentence that names the part asked about, else the first clause.");
    if py_truthy(&first_only.clone()) {
    clauses = _the_clauses(&[detail.clone()]);
    return if py_truthy(&clauses.clone()) { geti(&clauses.clone(), &Py::Int(0)) } else { mcall(&detail.clone(), "strip", &[py_str(" .")]) };
    }
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0)]), 2, None);
    best = __unpk1[0].clone();
    best_hit = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&_the_clauses(&[detail.clone()]));
    for __item in __iter2 {
    clause = __item.clone();
    hit = call_builtin("len", &[op_bitand(&wanted.clone(), &_part_words(&[clause.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&hit.clone(), &best_hit.clone())) {
    let __unpk3 = py_unpack(&py_tuple(vec![clause.clone(), hit.clone()]), 2, None);
    best = __unpk3[0].clone();
    best_hit = __unpk3[1].clone();
    }
    }
    clauses = _the_clauses(&[detail.clone()]);
    return { let __b = best.clone(); if py_truthy(&__b) { __b } else { if py_truthy(&clauses.clone()) { geti(&clauses.clone(), &Py::Int(0)) } else { mcall(&detail.clone(), "strip", &[py_str(" .")]) } } };
    Py::None
}
pub fn _the_first_step(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut step: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The opening step of the paragraph, said as a step rather than as the paragraph again.\n\n    \"what do i do first\" after \"Work out how many nights you have first, then give each place at\n    least three of them: a base with day trips beats a list of cities...\" wants the first thing to\n    do, and the opening sentence handed back is the paragraph they have just read - word for word,\n    so the arbiter takes it for a repeat and nobody answers the question. The first step of that\n    sentence, in front of a word that says it is the start, is both.\n    ");
    text = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "rstrip", &[py_str(".")]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__A_CLAUSE_BREAK(), "split", &[text.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    step = py_str("");
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    part = __item.clone();
    step = if py_truthy(&step.clone()) { op_add(&op_add(&step.clone(), &py_str(", ")), &part.clone()) } else { part.clone() };
    if py_truthy(&op_ge(&call_builtin("len", &[mcall(&step.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return Py::None;
    }
    if py_truthy(&op_ge(&call_builtin("len", &[mcall(&step.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))) {
    step = text.clone();
    }
    return op_add(&op_add(&op_add(&py_str("Start with this: "), &mcall(&geti(&step.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn _a_countdown_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut head: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: head"))) };
    let mut detail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut wanted: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut about_the_start: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "about_the_start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: about_the_start"))) };
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("A span heading with its step, short enough not to be the paragraph read out again.\n\n    The heading is the answer to \"when should i book\" and to \"how long will it take\": the step\n    alone is \"a week out\", and the whole sentence is the sentence they have just read.\n    ");
    step = _the_step(&[detail.clone(), wanted.clone(), about_the_start.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return Py::None;
    }
    said = op_add(&mcall(&mcall(&head.clone(), "strip", &[]), "rstrip", &[py_str(":")]), &if py_truthy(&Py::Bool(!py_truthy(&about_the_start.clone()))) { py_str(":") } else { py_str(" is when to start:") });
    return op_add(&op_add(&op_add(&op_add(&said.clone(), &py_str(" ")), &mcall(&geti(&step.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn _another_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut wanted: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut said_words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names something the paragraph itself never mentions.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(!py_contains(&g__ASK_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said_words.clone(), &word.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _part_words(args: &[Py]) -> Py {
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
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__PART_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _was_a_plan(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let _ = py_str("True when the person's line was a plan announced or a question about planning one.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&geti(&_implied(&[clean.clone()]), &Py::Int(0))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g__PLAN_ASK(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn follow_up(args: &[Py]) -> Py {
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
    let mut announced: Py = Py::None;
    let mut asked_about: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_hit: Py = Py::None;
    let mut colon: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut head: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut last: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut part: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut span: Py = Py::None;
    let mut told: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The sentence of the plan answer this line asks about, or None.\n\n    Only the turn straight after the announcement, and only while the paragraph the store gave is\n    still the last reply: the whole paragraph has just been read, so a question about one part of\n    it wants that sentence and not the paragraph again (which would be the same words twice) and\n    not a search hedge either.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&crate::memory::when::g_CALENDAR_NOW(), "match", &[mcall(&mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]), "strip", &[py_str(" .!?;,")])])) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    said = py_str("");
    reply = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    kept = mcall(&memory.clone(), "get", &[py_str("plan_answer")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kept.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _was_a_plan(&[mcall(&kept.clone(), "get", &[py_str("user")])]) } }) {
    announced = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&kept.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = announced.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&announced.clone(), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))) } }) {
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&kept.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    told = mcall(&memory.clone(), "get", &[py_str("entry_answer")]);
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = told.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    whole = mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&reply.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&reply.clone(), &slice_or_raise(&whole.clone(), &Py::None, &Py::Int(60), &Py::None))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&whole.clone(), &slice_or_raise(&reply.clone(), &Py::None, &Py::Int(60), &Py::None))) } })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("memory_store"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_was_a_plan(&[mcall(&last.clone(), "get", &[py_str("user")])]))) } }) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&mcall(&text.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ASKING(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_FIRST_ASK(), "search", &[text.clone()]) } } } }))) {
    return Py::None;
    }
    wanted = _part_words(&[text.clone()]);
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&{ let __b = mcall(&g__A_FIRST_ASK(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_another_thing(&[op_sub(&wanted.clone(), &g__ORDER_WORDS()), _part_words(&[said.clone()])]))) } }) {
    let __unpk1 = py_unpack(&if py_truthy(&sentences.clone()) { mcall(&geti(&sentences.clone(), &Py::Int(0)), "partition", &[py_str(":")]) } else { py_tuple(vec![py_str(""), py_str(""), py_str("")]) }, 3, None);
    head = __unpk1[0].clone();
    colon = __unpk1[1].clone();
    detail = __unpk1[2].clone();
    if py_truthy(&{ let __b = colon.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__SPAN_HEAD(), "search", &[head.clone()]); if !py_truthy(&__b) { __b } else { mcall(&detail.clone(), "strip", &[]) } } } }) {
    return _a_countdown_step(&[head.clone(), detail.clone(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), Py::Bool(false)]);
    }
    return if py_truthy(&sentences.clone()) { _the_first_step(&[geti(&sentences.clone(), &Py::Int(0))]) } else { Py::None };
    }
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&if py_truthy(&sentences.clone()) { mcall(&geti(&sentences.clone(), &Py::Int(0)), "partition", &[py_str(":")]) } else { py_tuple(vec![py_str(""), py_str(""), py_str("")]) }, 3, None);
    head = __unpk2[0].clone();
    colon = __unpk2[1].clone();
    detail = __unpk2[2].clone();
    span = call_builtin("bool", &[{ let __b = colon.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__SPAN_HEAD(), "search", &[head.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    opening = Py::None;
    asked_about = py_list(vec![]);
    if py_truthy(&{ let __b = colon.clone(); if !py_truthy(&__b) { __b } else { mcall(&detail.clone(), "strip", &[]) } }) {
    opening = op_add(&mcall(&geti(&mcall(&detail.clone(), "strip", &[]), &Py::Int(0)), "upper", &[]), &slice_or_raise(&mcall(&detail.clone(), "strip", &[]), &Py::Int(1), &Py::None, &Py::None));
    let _ = mcall(&asked_about.clone(), "append", &[py_tuple(vec![opening.clone(), op_bitor(&_part_words(&[detail.clone()]), &_part_words(&[head.clone()]))])]);
    }
    let __aug3 = op_add(&asked_about.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&sentences.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut sentence: Py = __item.clone(); __out.push(py_tuple(vec![sentence.clone(), _part_words(&[sentence.clone()])])); } py_list(__out) })() });
    asked_about = __aug3;
    let __unpk4 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0)]), 2, None);
    best = __unpk4[0].clone();
    best_hit = __unpk4[1].clone();
    let __iter5 = py_iter_or_raise(&asked_about.clone());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    sentence = __unpk6[0].clone();
    words = __unpk6[1].clone();
    if py_truthy(&mcall(&reply.clone(), "startswith", &[slice_or_raise(&mcall(&py_str(" "), "join", &[mcall(&mcall(&sentence.clone(), "lower", &[]), "split", &[])]), &Py::None, &Py::Int(40), &Py::None)])) {
    continue;
    }
    hit = call_builtin("len", &[op_bitand(&wanted.clone(), &words.clone())]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = hit.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_ge(&hit.clone(), &best_hit.clone()) } } } }) {
    let __unpk7 = py_unpack(&py_tuple(vec![sentence.clone(), hit.clone()]), 2, None);
    best = __unpk7[0].clone();
    best_hit = __unpk7[1].clone();
    }
    }
    if py_truthy(&Py::Bool(py_is(&best.clone(), &Py::None))) {
    if py_truthy(&{ let __b = span.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_LENGTH_ASK(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_another_thing(&[wanted.clone(), _part_words(&[said.clone()])]))) } } } }) {
    return _a_countdown_step(&[head.clone(), detail.clone(), wanted.clone(), Py::Bool(true)]);
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&best.clone(), &opening.clone())); if !py_truthy(&__b) { __b } else { span.clone() } }) {
    return _a_countdown_step(&[head.clone(), detail.clone(), wanted.clone(), call_builtin("bool", &[mcall(&g__A_LENGTH_ASK(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return best.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.plans".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CALENDAR_NOW" => Some(g_CALENDAR_NOW()),
            "PLANNING_KEYS" => Some(g_PLANNING_KEYS()),
            "_ABOUT_A_DEATH" => Some(g__ABOUT_A_DEATH()),
            "_ASKING" => Some(g__ASKING()),
            "_ASK_FOR" => Some(g__ASK_FOR()),
            "_ASK_WORDS" => Some(g__ASK_WORDS()),
            "_A_CLAUSE_BREAK" => Some(g__A_CLAUSE_BREAK()),
            "_A_FIRST_ASK" => Some(g__A_FIRST_ASK()),
            "_A_LENGTH_ASK" => Some(g__A_LENGTH_ASK()),
            "_DENIED" => Some(g__DENIED()),
            "_MARRYING" => Some(g__MARRYING()),
            "_MOVING" => Some(g__MOVING()),
            "_NEW_JOB" => Some(g__NEW_JOB()),
            "_ORDER_WORDS" => Some(g__ORDER_WORDS()),
            "_PART_WORDS" => Some(g__PART_WORDS()),
            "_PLANNING" => Some(g__PLANNING()),
            "_PLAN_ASK" => Some(g__PLAN_ASK()),
            "_SELF" => Some(g__SELF()),
            "_SPAN_HEAD" => Some(g__SPAN_HEAD()),
            "_TURNED" => Some(g__TURNED()),
            "_WHO" => Some(g__WHO()),
            "_WORK_WHEN" => Some(g__WORK_WHEN()),
            "_a_countdown_step" => Some(py_func("_a_countdown_step", |args: &[Py]| _a_countdown_step(args))),
            "_a_first_day" => Some(py_func("_a_first_day", |args: &[Py]| _a_first_day(args))),
            "_about_planning" => Some(py_func("_about_planning", |args: &[Py]| _about_planning(args))),
            "_another_thing" => Some(py_func("_another_thing", |args: &[Py]| _another_thing(args))),
            "_implied" => Some(py_func("_implied", |args: &[Py]| _implied(args))),
            "_part_words" => Some(py_func("_part_words", |args: &[Py]| _part_words(args))),
            "_the_clauses" => Some(py_func("_the_clauses", |args: &[Py]| _the_clauses(args))),
            "_the_first_step" => Some(py_func("_the_first_step", |args: &[Py]| _the_first_step(args))),
            "_the_reply" => Some(py_func("_the_reply", |args: &[Py]| _the_reply(args))),
            "_the_step" => Some(py_func("_the_step", |args: &[Py]| _the_step(args))),
            "_was_a_plan" => Some(py_func("_was_a_plan", |args: &[Py]| _was_a_plan(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "as_question" => Some(py_func("as_question", |args: &[Py]| as_question(args))),
            "follow_up" => Some(py_func("follow_up", |args: &[Py]| follow_up(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CALENDAR_NOW" => s_CALENDAR_NOW(v),
            "PLANNING_KEYS" => s_PLANNING_KEYS(v),
            "_ABOUT_A_DEATH" => s__ABOUT_A_DEATH(v),
            "_ASKING" => s__ASKING(v),
            "_ASK_FOR" => s__ASK_FOR(v),
            "_ASK_WORDS" => s__ASK_WORDS(v),
            "_A_CLAUSE_BREAK" => s__A_CLAUSE_BREAK(v),
            "_A_FIRST_ASK" => s__A_FIRST_ASK(v),
            "_A_LENGTH_ASK" => s__A_LENGTH_ASK(v),
            "_DENIED" => s__DENIED(v),
            "_MARRYING" => s__MARRYING(v),
            "_MOVING" => s__MOVING(v),
            "_NEW_JOB" => s__NEW_JOB(v),
            "_ORDER_WORDS" => s__ORDER_WORDS(v),
            "_PART_WORDS" => s__PART_WORDS(v),
            "_PLANNING" => s__PLANNING(v),
            "_PLAN_ASK" => s__PLAN_ASK(v),
            "_SELF" => s__SELF(v),
            "_SPAN_HEAD" => s__SPAN_HEAD(v),
            "_TURNED" => s__TURNED(v),
            "_WHO" => s__WHO(v),
            "_WORK_WHEN" => s__WORK_WHEN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.plans", module_obj);
    pyrt::xrefs::reg_fn("memory.plans", "_a_first_day", py_func("_a_first_day", |args: &[Py]| _a_first_day(args)));
    pyrt::xrefs::reg_fn("memory.plans", "as_question", py_func("as_question", |args: &[Py]| as_question(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_implied", py_func("_implied", |args: &[Py]| _implied(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_the_reply", py_func("_the_reply", |args: &[Py]| _the_reply(args)));
    pyrt::xrefs::reg_fn("memory.plans", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_about_planning", py_func("_about_planning", |args: &[Py]| _about_planning(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_the_clauses", py_func("_the_clauses", |args: &[Py]| _the_clauses(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_the_step", py_func("_the_step", |args: &[Py]| _the_step(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_the_first_step", py_func("_the_first_step", |args: &[Py]| _the_first_step(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_a_countdown_step", py_func("_a_countdown_step", |args: &[Py]| _a_countdown_step(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_another_thing", py_func("_another_thing", |args: &[Py]| _another_thing(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_part_words", py_func("_part_words", |args: &[Py]| _part_words(args)));
    pyrt::xrefs::reg_fn("memory.plans", "_was_a_plan", py_func("_was_a_plan", |args: &[Py]| _was_a_plan(args)));
    pyrt::xrefs::reg_fn("memory.plans", "follow_up", py_func("follow_up", |args: &[Py]| follow_up(args)));
    pyrt::xrefs::reg_access("memory.plans", "CALENDAR_NOW", g_CALENDAR_NOW, s_CALENDAR_NOW);
    pyrt::xrefs::reg_access("memory.plans", "PLANNING_KEYS", g_PLANNING_KEYS, s_PLANNING_KEYS);
    pyrt::xrefs::reg_access("memory.plans", "_ABOUT_A_DEATH", g__ABOUT_A_DEATH, s__ABOUT_A_DEATH);
    pyrt::xrefs::reg_access("memory.plans", "_ASKING", g__ASKING, s__ASKING);
    pyrt::xrefs::reg_access("memory.plans", "_ASK_FOR", g__ASK_FOR, s__ASK_FOR);
    pyrt::xrefs::reg_access("memory.plans", "_ASK_WORDS", g__ASK_WORDS, s__ASK_WORDS);
    pyrt::xrefs::reg_access("memory.plans", "_A_CLAUSE_BREAK", g__A_CLAUSE_BREAK, s__A_CLAUSE_BREAK);
    pyrt::xrefs::reg_access("memory.plans", "_A_FIRST_ASK", g__A_FIRST_ASK, s__A_FIRST_ASK);
    pyrt::xrefs::reg_access("memory.plans", "_A_LENGTH_ASK", g__A_LENGTH_ASK, s__A_LENGTH_ASK);
    pyrt::xrefs::reg_access("memory.plans", "_DENIED", g__DENIED, s__DENIED);
    pyrt::xrefs::reg_access("memory.plans", "_MARRYING", g__MARRYING, s__MARRYING);
    pyrt::xrefs::reg_access("memory.plans", "_MOVING", g__MOVING, s__MOVING);
    pyrt::xrefs::reg_access("memory.plans", "_NEW_JOB", g__NEW_JOB, s__NEW_JOB);
    pyrt::xrefs::reg_access("memory.plans", "_ORDER_WORDS", g__ORDER_WORDS, s__ORDER_WORDS);
    pyrt::xrefs::reg_access("memory.plans", "_PART_WORDS", g__PART_WORDS, s__PART_WORDS);
    pyrt::xrefs::reg_access("memory.plans", "_PLANNING", g__PLANNING, s__PLANNING);
    pyrt::xrefs::reg_access("memory.plans", "_PLAN_ASK", g__PLAN_ASK, s__PLAN_ASK);
    pyrt::xrefs::reg_access("memory.plans", "_SELF", g__SELF, s__SELF);
    pyrt::xrefs::reg_access("memory.plans", "_SPAN_HEAD", g__SPAN_HEAD, s__SPAN_HEAD);
    pyrt::xrefs::reg_access("memory.plans", "_TURNED", g__TURNED, s__TURNED);
    pyrt::xrefs::reg_access("memory.plans", "_WHO", g__WHO, s__WHO);
    pyrt::xrefs::reg_access("memory.plans", "_WORK_WHEN", g__WORK_WHEN, s__WORK_WHEN);
}
