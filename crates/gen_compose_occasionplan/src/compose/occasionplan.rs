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


thread_local! { static G_CHILD_PLANS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLANS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOON_PLANS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANOTHER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_FOR_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MONTH_ON_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NAME_BEFORE_THE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_POSSESSIVE_BEFORE_THE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRTHDAY_IN_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_TO_COME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MINE_BEFORE_THE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PLAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEONE_ELSES_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPOKEN_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STILL_TALKING_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TURNS_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_CHILD_PLANS() -> Py { ensure_init(); G_CHILD_PLANS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_PLANS(v: Py) { ensure_init(); G_CHILD_PLANS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLANS() -> Py { ensure_init(); G_PLANS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLANS(v: Py) { ensure_init(); G_PLANS.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATIONS() -> Py { ensure_init(); G_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATIONS(v: Py) { ensure_init(); G_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_SOON_PLANS() -> Py { ensure_init(); G_SOON_PLANS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOON_PLANS(v: Py) { ensure_init(); G_SOON_PLANS.with(|c| *c.borrow_mut() = v); }
pub fn g__ANOTHER_PERSON() -> Py { ensure_init(); G__ANOTHER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANOTHER_PERSON(v: Py) { ensure_init(); G__ANOTHER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_FOR_A_PLAN() -> Py { ensure_init(); G__ASKS_FOR_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_FOR_A_PLAN(v: Py) { ensure_init(); G__ASKS_FOR_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MONTH_ON_ITS_OWN() -> Py { ensure_init(); G__A_MONTH_ON_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MONTH_ON_ITS_OWN(v: Py) { ensure_init(); G__A_MONTH_ON_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NAME_BEFORE_THE_DAY() -> Py { ensure_init(); G__A_NAME_BEFORE_THE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NAME_BEFORE_THE_DAY(v: Py) { ensure_init(); G__A_NAME_BEFORE_THE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_POSSESSIVE_BEFORE_THE_DAY() -> Py { ensure_init(); G__A_POSSESSIVE_BEFORE_THE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_POSSESSIVE_BEFORE_THE_DAY(v: Py) { ensure_init(); G__A_POSSESSIVE_BEFORE_THE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRTHDAY_IN_PLAN() -> Py { ensure_init(); G__BIRTHDAY_IN_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRTHDAY_IN_PLAN(v: Py) { ensure_init(); G__BIRTHDAY_IN_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_SAID() -> Py { ensure_init(); G__DAY_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_SAID(v: Py) { ensure_init(); G__DAY_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_TO_COME() -> Py { ensure_init(); G__DAY_TO_COME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_TO_COME(v: Py) { ensure_init(); G__DAY_TO_COME.with(|c| *c.borrow_mut() = v); }
pub fn g__MINE_BEFORE_THE_DAY() -> Py { ensure_init(); G__MINE_BEFORE_THE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MINE_BEFORE_THE_DAY(v: Py) { ensure_init(); G__MINE_BEFORE_THE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PLAN_ASK() -> Py { ensure_init(); G__NOT_A_PLAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PLAN_ASK(v: Py) { ensure_init(); G__NOT_A_PLAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_SUBJECT() -> Py { ensure_init(); G__OWN_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_SUBJECT(v: Py) { ensure_init(); G__OWN_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEONE_ELSES_DAY() -> Py { ensure_init(); G__SOMEONE_ELSES_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEONE_ELSES_DAY(v: Py) { ensure_init(); G__SOMEONE_ELSES_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__SOON() -> Py { ensure_init(); G__SOON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOON(v: Py) { ensure_init(); G__SOON.with(|c| *c.borrow_mut() = v); }
pub fn g__SPOKEN_FOR() -> Py { ensure_init(); G__SPOKEN_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPOKEN_FOR(v: Py) { ensure_init(); G__SPOKEN_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__STILL_TALKING_ABOUT() -> Py { ensure_init(); G__STILL_TALKING_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STILL_TALKING_ABOUT(v: Py) { ensure_init(); G__STILL_TALKING_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g__TURNS_AGE() -> Py { ensure_init(); G__TURNS_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TURNS_AGE(v: Py) { ensure_init(); G__TURNS_AGE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A birthday already on file, and the line that asks what to do for it.\n\n\"my sister turns 30 on saturday\" is stored as a person with an age and a day still to come\n(`memory[\"others\"]`, `{\"who\": \"sister\", \"age\": 30, \"turns\": \"on saturday\"}`). The next line,\n\"i want to plan something\", asks for a plan for exactly that, and on main the store was not read\nat all: the line got a question back (\"Good thing to take on. What's the first step from where\nyou are?\") or a plan for a plain Saturday (\"Pick two things and put them in the diary by Friday:\n... a market on a Saturday morning\"). Neither is a 30th, and the person had already said whose\nday it was.\n\nSo this reads the occasion the store holds and answers for it: the one thing worth booking, the\none thing worth doing today, and the reason. Nothing is invented about the person - only their\nname for them, the age and the day they gave - and the plan is the ordinary one that works for a\ngroup at short notice (a table with a time on it, the guest list told today).\n\nTwo gates, so it cannot take a turn it does not know:\n\n* the line asks for a plan and names nothing else (\"i want to plan something\", \"any ideas\",\n  \"what should we do\", \"help me sort something out\"). A line that carries its own subject\n  (\"how do i plan my diet\", \"i want to plan the route\", \"ideas for tea\") is somebody else's,\n  because what it is about is on the line and not in the store.\n* the store holds exactly one person with an age and a day still to come. Two is not one thing\n  to plan, and an age with no day (\"my mum is 70\") is a fact rather than a date.\n");
    s__DAY_TO_COME(stdlib_call("re", "compile", &[py_str("\\b(?:on\\s+)?(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|tonight|tomorrow|today|this weekend|this saturday|the weekend|next week|next weekend|next month|next year|in \\d+ (?:days?|weeks?|months?)|\\d{1,2}(?:st|nd|rd|th)? (?:of )?(?:january|february|march|april|may|june|july|august|september|october|november|december)|(?:january|february|march|april|may|june|july|august|september|october|november|december)(?: \\d{1,2}(?:st|nd|rd|th)?)?|(?:on )?the \\d{1,2}(?:st|nd|rd|th))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_FOR_A_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:plan|plans|planning|arrange|arranging|organi[sz]e|organi[sz]ing|ideas?|something|anything|celebrate|throw (?:her|him|them|a)|do (?:for|to mark))\\b|\\b(?:what|how) (?:should|shall|can|could) (?:we|i) do\\b|\\bwhat do (?:we|i) do\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__OWN_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:diet|workout|exercise|training|budget|money|finances?|meals?|food|tea|dinner|lunch|breakfast|shopping|garden|shed|loft|move|moving|house|holiday|trip|route|journey|revision|study|work|job|business|wedding|funeral|birth|baby|contract|rota|shift|packing|pack|list|calendar|week|month|year|term)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_PLAN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what day|which day|when is|when's|where is|where's|how old|what time|who is|how much|how many|why|what do you mean|what did you mean)\\b|\\bwould (?:actually |really )?(?:like|love|enjoy|prefer|want)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLANS(py_tuple(vec![py_str("For {day}, book the one thing that needs a table or a ticket, because everything else on the night can be decided on the spot. Then send the time and place to everyone you want there today, so nobody is left waiting to hear."), py_str("Get one firm thing in the diary for {day} first - a table for the group at seven, or tickets - and keep the rest of it loose. Send the details round today; a birthday falls apart at the 'so what are we doing' stage, not on the night."), py_str("The thing that makes {day} work is one booking with a time on it: a table at seven, then somewhere with music after. Text the people you want there today with both, and ask them who else they would want in the group."), py_str("For {day}, put a time on it and tell people today: a table booked for seven, and a second place to go after that needs no booking. Ask what they would actually like to eat first, so the booking is somewhere they want rather than somewhere free.")]));
    s_CHILD_PLANS(py_tuple(vec![py_str("For {day}, book the thing that fills up first - a hall, a soft play slot, a bowling lane - and send the invitations the same day, because a children's party is decided by who can come. Order the cake when you book, so it is not the thing left to Friday."), py_str("Get the slot booked for {day} before you think about anything else: the weekend ones go first. Then ask them which two friends matter most, invite those and a few more, and keep it to two hours - that is as long as a party that size lasts."), py_str("For {day}, one booked place and a short guest list beats a big plan: a hall or a soft play slot for two hours, food that is easy to hand round, and a cake ordered today. Ask the parents to stay if the children are small.")]));
    s__SOON(stdlib_call("re", "compile", &[py_str("\\b(?:tonight|tomorrow|today|this weekend|this saturday|in \\d+ days?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOON_PLANS(py_tuple(vec![py_str("It is close, so keep it small and certain for {day}: book a table for the people who can actually come, tell them the time today, and let the rest be drinks after. A short-notice birthday works on who turns up, not on the plan."), py_str("For {day}, ring the place rather than booking online - a table for six at short notice is usually a phone call - and send one message with the time and the address to everyone today.")]));
    s__A_MONTH_ON_ITS_OWN(stdlib_call("re", "compile", &[py_str("january|february|march|april|may|june|july|august|september|october|november|december")]).unwrap_or_else(|e| exc::raise(e)));
    s__BIRTHDAY_IN_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:birthday|bday|turning|turns?|\\d{1,2}\\s*-?\\s*(?:this|next)\\s*(?:month|week|year))\\b|\\b(?:i'?m|im|i am) (?:\\d{1,2}|turning \\d{1,2})\\b|\\bmy \\d{1,2}(?:st|nd|rd|th)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__TURNS_AGE(stdlib_call("re", "compile", &[py_str("\\bturns? (\\d{1,2})\\b|\\bturning (\\d{1,2})\\b|\\bmy (\\d{1,2})(?:st|nd|rd|th)\\b|\\b(\\d{1,2})(?:st|nd|rd|th)\\s+(?:birthday|bday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__STILL_TALKING_ABOUT(Py::Int(6));
    fn __data1() -> Py { py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mam"), py_str("mother"), py_str("dad"), py_str("father"), py_str("son"), py_str("daughter"), py_str("boy"), py_str("girl"), py_str("baby"), py_str("kid"), py_str("child"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("sister"), py_str("brother"), py_str("friend"), py_str("mate"), py_str("nephew"), py_str("niece"), py_str("cousin"), py_str("aunt"), py_str("auntie"), py_str("uncle"), py_str("nan"), py_str("granny"), py_str("gran"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("boss"), py_str("colleague"), py_str("neighbour"), py_str("neighbor"), py_str("flatmate"), py_str("housemate"), py_str("lass"), py_str("lad"), py_str("fella"), py_str("eldest"), py_str("youngest"), py_str("twins")]) }
    s_RELATIONS(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s__SPOKEN_FOR(py_dict(vec![(py_str("half"), py_str("partner")), (py_str("one"), py_str("child"))]));
    s__MINE_BEFORE_THE_DAY(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?:[a-z]{2,12}\\s+){0,2}(?P<who>(?!(?:is|are|was|will|turns?|turning)\\b)[a-z]{2,12})(?:'s|s)?\\s+(?:(?:is|are)\\s+)?(?:\\d{1,2}(?:st|nd|rd|th)\\s+)?(?:birthday|bday|turns?\\b|turning\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_POSSESSIVE_BEFORE_THE_DAY(stdlib_call("re", "compile", &[py_str("\\b(?P<who>[a-z]{2,12})(?:'s|s)\\s+(?:\\d{1,2}(?:st|nd|rd|th)\\s+)?(?:birthday|bday|turns?\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NAME_BEFORE_THE_DAY(stdlib_call("re", "compile", &[py_str("\\bmy\\s+[a-z]{2,12}(?:'s|s)?\\b[^.!?]{0,24}\\b(?:birthday|bday|turn(?:s|ing)?\\s+\\d{1,2}\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__SOMEONE_ELSES_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:his|her|their|its)\\s+(?:birthday|bday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__DAY_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ANOTHER_PERSON(stdlib_call("re", "compile", &[py_str("\\bfor (?:my|our|his|her|their|the) (?:[a-z]+ )?(?:mum|mom|dad|father|mother|sister|brother|wife|husband|partner|son|daughter|friend|mate|nan|gran|grandma|grandad|boss|colleague|neighbours?|kids?|children)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _ordinal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut age: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "age") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: age"))) };
    let mut suffix: Py = Py::None;
    let _ = py_str("30 -> \"30th\", 21 -> \"21st\", 22 -> \"22nd\", 23 -> \"23rd\".");
    if py_truthy(&{ let __cmp11: Py = op_modulo(&age.clone(), &Py::Int(100)); if py_truthy(&op_le(&Py::Int(10), &__cmp11)) { op_le(&__cmp11, &Py::Int(20)) } else { Py::Bool(false) } }) {
    suffix = py_str("th");
    } else {
    suffix = mcall(&py_dict(vec![(Py::Int(1), py_str("st")), (Py::Int(2), py_str("nd")), (Py::Int(3), py_str("rd"))]), "get", &[op_modulo(&age.clone(), &Py::Int(10)), py_str("th")]);
    }
    return op_modulo(&py_str("%d%s"), &py_tuple(vec![age.clone(), suffix.clone()]));
    Py::None
}
pub fn _whose(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut low: Py = Py::None;
    let _ = py_str("\"sister\" -> \"your sister's\"; a name is said as it is: \"Jess\" -> \"Jess's\".");
    who = mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return py_str("");
    }
    low = mcall(&who.clone(), "lower", &[]);
    if py_truthy(&mcall(&low.clone(), "startswith", &[py_tuple(vec![py_str("your "), py_str("my ")])])) {
    low = geti(&mcall(&low.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(1));
    }
    if py_truthy(&mcall(&slice_or_raise(&who.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) {
    return op_add(&who.clone(), &py_str("'s"));
    }
    return op_add(&op_add(&py_str("your "), &low.clone()), &if py_truthy(&mcall(&low.clone(), "endswith", &[py_str("s")])) { py_str("'") } else { py_str("'s") });
    Py::None
}
pub fn _the_day(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut when: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut found: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("Just the day phrase out of a birthday line, as the reply says it.\n\n    \"my daughters birthday is next month she turns 7\" keeps the whole clause in the store, and\n    the reply needs \"next month\" from it and not the clause; \"3rd may\" is said with an \"on\" in\n    front because a reply reads \"your birthday on 3rd May\" and not \"your birthday 3rd May\".\n    ");
    when = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = when.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = mcall(&g__DAY_TO_COME(), "search", &[when.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    phrase = mcall(&g__DAY_SAID(), "sub", &[py_func("_cap", |args: &[Py]| _cap(args)), mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[])]);
    if py_truthy(&mcall(&g__A_MONTH_ON_ITS_OWN(), "fullmatch", &[mcall(&phrase.clone(), "lower", &[])])) {
    return op_add(&py_str("in "), &phrase.clone());
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&phrase.clone(), "startswith", &[py_tuple(vec![py_str("on "), py_str("in "), py_str("next "), py_str("this "), py_str("the "), py_str("tomorrow"), py_str("tonight"), py_str("today"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday")])])))) {
    phrase = op_add(&py_str("on "), &phrase.clone());
    }
    return phrase.clone();
    Py::None
}
pub fn _the_age(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut found: Py = Py::None;
    let mut group: Py = Py::None;
    let _ = py_str("The age a birthday line states, \"she turns 7\" / \"turning 40\", or None.\n\n    A figure on its own is not an age here: \"my birthday is on the 3rd of May\" has a 3 in it and\n    the person is not three.\n    ");
    found = mcall(&g__TURNS_AGE(), "search", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&found.clone(), "groups", &[]));
    for __item in __iter1 {
    group = __item.clone();
    if py_truthy(&group.clone()) {
    age = call_builtin("int", &[group.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __cmp21: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(1), &__cmp21)) { op_lt(&__cmp21, &Py::Int(120)) } else { Py::Bool(false) } }) {
    return age.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _said_how_long_ago(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { Py::None };
    let mut markers: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "markers") { v.clone() }
    else { py_str("") };
    let mut best: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut now: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Turns since the line that filed this birthday, or None when nothing filed it.");
    turns = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("turns")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { py_list(vec![]) };
    now = op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    best = Py::None;
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[mcall(&entry.clone(), "get", &[py_str("turn")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![mcall(&entry.clone(), "get", &[py_str("value")]), call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    value = __unpk2[0].clone();
    text = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_is(&who.clone(), &Py::None))) {
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &py_str("other"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&value.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&call_builtin("str", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])))) {
    continue;
    }
    } else {
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("plan"), py_str("plans"), py_str("other"), py_str("people"), py_str("birthday")]), &mcall(&entry.clone(), "get", &[py_str("key")])))) {
    continue;
    }
    if py_truthy(&{ let __b = markers.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&markers.clone(), "search", &[mcall(&text.clone(), "lower", &[])]))) } }) {
    continue;
    }
    }
    gap = op_sub(&now.clone(), &geti(&entry.clone(), &py_str("turn")));
    best = if py_truthy(&Py::Bool(py_is(&best.clone(), &Py::None))) { gap.clone() } else { call_builtin("min", &[best.clone(), gap.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    }
    return best.clone();
    Py::None
}
pub fn _the_person(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut low: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Who the birthday is for, read off the line: \"son\" for \"my sons 5th birthday\", else None.\n\n    Empty string means somebody is named and cannot be placed: \"my flatmate's birthday\" is not a\n    person this can address, and \"your birthday\" would be the wrong person, so the plan stands\n    aside instead.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&mcall(&g__SOMEONE_ELSES_DAY(), "search", &[low.clone()])) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g__MINE_BEFORE_THE_DAY(), g__A_POSSESSIVE_BEFORE_THE_DAY()]));
    for __item in __iter1 {
    shape = __item.clone();
    found = mcall(&shape.clone(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    who = mcall(&mcall(&found.clone(), "group", &[py_str("who")]), "strip", &[py_str("'")]);
    if py_truthy(&Py::Bool(py_contains(&g__SPOKEN_FOR(), &who.clone()))) {
    return geti(&g__SPOKEN_FOR(), &who.clone());
    }
    if py_truthy(&Py::Bool(py_contains(&g_RELATIONS(), &who.clone()))) {
    return who.clone();
    }
    if py_truthy(&{ let __b = mcall(&who.clone(), "endswith", &[py_str("'s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_RELATIONS(), &slice_or_raise(&who.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None))) } }) {
    return slice_or_raise(&who.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = mcall(&who.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_RELATIONS(), &slice_or_raise(&who.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return slice_or_raise(&who.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return if py_truthy(&mcall(&g__A_NAME_BEFORE_THE_DAY(), "search", &[low.clone()])) { py_str("") } else { Py::None };
    }
    return if py_truthy(&mcall(&g__A_NAME_BEFORE_THE_DAY(), "search", &[low.clone()])) { py_str("") } else { Py::None };
    Py::None
}
pub fn _the_recent_age(args: &[Py]) -> Py {
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
    let mut upto: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "upto") { v.clone() }
    else { Py::Int(6) };
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut found: Py = Py::None;
    let mut g: Py = Py::None;
    let mut last: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The one age said over the last few turns: \"shes turning 8\" after the plan was filed.\n\n    The plan the store keeps is the line that named the day, so an age said on a later turn\n    (\"im trying to sort out my daughters birthday party next weekend\" / \"shes turning 8\") is only\n    there. Two different ages in the window are not one birthday, so nothing is read.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), Py::Int(0)]), 2, None);
    ages = __unpk1[0].clone();
    turns = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __aug3 = op_add(&turns.clone(), &Py::Int(1));
    turns = __aug3;
    if py_truthy(&op_gt(&turns.clone(), &upto.clone())) {
    break;
    }
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter4 = py_iter_or_raise(&mcall(&g__TURNS_AGE(), "finditer", &[text.clone()]));
    for __item in __iter4 {
    found = __item.clone();
    age = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&found.clone(), "groups", &[])) { let mut g: Py = __item.clone(); if py_truthy(&g.clone()) {  __out.push(g.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&age.clone()) {
    let _ = mcall(&ages.clone(), "add", &[call_builtin("int", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[ages.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    age = mcall(&ages.clone(), "pop", &[]);
    if py_truthy(&{ let __cmp51: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(1), &__cmp51)) { op_lt(&__cmp51, &Py::Int(120)) } else { Py::Bool(false) } }) {
    return age.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_occasion(args: &[Py]) -> Py {
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
    let mut birthday: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut last: Py = Py::None;
    let mut met: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut one: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut said: Py = Py::None;
    let mut told: Py = Py::None;
    let mut when: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The one birthday the store holds with a day still to come, else None.\n\n    Two shapes, one reading. Somebody else's day is a person in `memory[\"others\"]` with the age\n    and the day kept apart (\"my sister turns 30 on saturday\") or together in the birthday text\n    (\"my daughters birthday is next month she turns 7\"). The person's own is their own record, and\n    \"im turning 40 next month\" reaches it as a plan in their own words, with no figure filed\n    beside it, so the figure is read out of that plan.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    told = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&last.clone(), "get", &[py_str("user")]) } }) {
    told = call_builtin("str", &[mcall(&last.clone(), "get", &[py_str("user")])]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    met = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter2 {
    one = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { mcall(&one.clone(), "get", &[py_str("pronoun")]) } }) {
    continue;
    }
    who = { let __b = mcall(&one.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { _the_person(&[told.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    continue;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![mcall(&one.clone(), "get", &[py_str("age")]), _the_day(&[mcall(&one.clone(), "get", &[py_str("turns")])])]), 2, None);
    age = __unpk3[0].clone();
    when = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    birthday = call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("birthday")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    when = _the_day(&[birthday.clone()]);
    if py_truthy(&when.clone()) {
    age = _the_age(&[birthday.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[age.clone(), py_func("bool", |args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __cmp41: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(1), &__cmp41)) { op_lt(&__cmp41, &Py::Int(120)) } else { Py::Bool(false) } })) } } } }) {
    age = Py::None;
    }
    gap = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_how_long_ago(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&gap.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_gt(&gap.clone(), &g__STILL_TALKING_ABOUT()) } }) {
    continue;
    }
    let _ = mcall(&met.clone(), "append", &[py_tuple(vec![py_str("another"), who.clone(), age.clone(), when.clone()])]);
    }
    mine = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&met.clone())); if !py_truthy(&__b) { __b } else { mine.clone() } }) {
    plans = if py_truthy(&call_builtin("isinstance", &[mcall(&mine.clone(), "get", &[py_str("plans")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&mine.clone(), "get", &[py_str("plans")]) } else { py_list(vec![]) };
    said = { let __b = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&plans.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)) { let mut p: Py = __item.clone(); __out.push(call_builtin("str", &[p.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]); if py_truthy(&__b) { __b } else { call_builtin("str", &[{ let __b = mcall(&mine.clone(), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } };
    when = if py_truthy(&mcall(&g__BIRTHDAY_IN_PLAN(), "search", &[said.clone()])) { _the_day(&[said.clone()]) } else { py_str("") };
    who = if py_truthy(&when.clone()) { _the_person(&[op_add(&op_add(&said.clone(), &py_str(" ")), &told.clone())]) } else { Py::None };
    if py_truthy(&{ let __b = when.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&who.clone(), &py_str(""))) } }) {
    return Py::None;
    }
    if py_truthy(&when.clone()) {
    age = mcall(&mine.clone(), "get", &[py_str("age")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[age.clone(), py_func("bool", |args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __cmp51: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(1), &__cmp51)) { op_lt(&__cmp51, &Py::Int(120)) } else { Py::Bool(false) } })) } } } }) {
    age = { let __b = _the_age(&[said.clone()]); if py_truthy(&__b) { __b } else { _the_recent_age(&[memory.clone()]) } };
    }
    gap = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("markers".to_string(), g__BIRTHDAY_IN_PLAN())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_how_long_ago(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&gap.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_le(&gap.clone(), &g__STILL_TALKING_ABOUT()) } }) {
    let _ = mcall(&met.clone(), "append", &[py_tuple(vec![py_str("you"), { let __b = who.clone(); if py_truthy(&__b) { __b } else { Py::None } }, age.clone(), when.clone()])]);
    }
    }
    }
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[met.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&met.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _cap(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let _ = py_str("A day word said back: \"on saturday\" reads as a day and not as a stray lower case word.");
    return op_add(&mcall(&slice_or_raise(&mcall(&match_.clone(), "group", &[Py::Int(0)]), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&mcall(&match_.clone(), "group", &[Py::Int(0)]), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _reads_as_a_plan_ask(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut subject: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks for a plan for the thing in hand and names nothing else.\n\n    A follow-up leans on the turn before it, so \"i want to plan something\" carries no subject of\n    its own and the store is the only place the subject can come from. The subject words are\n    looked for only on the line itself: a word the earlier turn used (\"my sister turns 30 on\n    saturday\") is what the store already made of it.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    words = mcall(&mcall(&mcall(&mcall(&said.clone(), "replace", &[py_str("?"), py_str(" ")]), "replace", &[py_str("!"), py_str(" ")]), "replace", &[py_str("."), py_str(" ")]), "split", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__ASKS_FOR_A_PLAN(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__NOT_A_PLAN_ASK(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    subject = mcall(&g__OWN_SUBJECT(), "findall", &[said.clone()]);
    if py_truthy(&subject.clone()) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__ANOTHER_PERSON(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
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
    let _ = py_str("The hook a module declares (`TAKES_AS_IT_COMES`): this line is the plan for the day on file.\n\n    Nothing in the entries knows whose birthday is in the store, so the card that answers a plan\n    ask in general plans a plain Saturday - \"pick two things and put them in the diary by Friday\"\n    - and the day the person named is lost. Said only on the lines `reply_for` answers, so every\n    other line this module proposes on is settled by the ordinary rules.\n    ");
    return Py::Bool(!py_is(&reply_for(&[text.clone(), memory.clone()]), &Py::None));
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
    let mut age: Py = Py::None;
    let mut c: Py = Py::None;
    let mut day: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut occasion: Py = Py::None;
    let mut part: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut when: Py = Py::None;
    let mut who: Py = Py::None;
    let mut whose: Py = Py::None;
    let _ = py_str("A plan for the birthday the store holds, or None for every other line.");
    occasion = _the_occasion(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&occasion.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_reads_as_a_plan_ask(&[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&occasion.clone(), 4, None);
    kind = __unpk1[0].clone();
    who = __unpk1[1].clone();
    age = __unpk1[2].clone();
    when = __unpk1[3].clone();
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("you")))) {
    whose = if py_truthy(&who.clone()) { _whose(&[who.clone()]) } else { py_str("your") };
    } else {
    whose = _whose(&[who.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&whose.clone()))) {
    return Py::None;
    }
    }
    day = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![whose.clone(), if py_truthy(&age.clone()) { _ordinal(&[age.clone()]) } else { py_str("birthday") }, when.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(16)) } }) {
    pool = g_CHILD_PLANS();
    } else {
    if py_truthy(&mcall(&g__SOON(), "search", &[when.clone()])) {
    pool = g_SOON_PLANS();
    } else {
    pool = g_PLANS();
    }
    }
    turns = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("turns")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { py_list(vec![]) };
    seed = op_add(&op_mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    line = mcall(&stdlib_call("random", "Random", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e)), "choice", &[pool.clone()]);
    return mcall(&line.clone(), "replace", &[py_str("{day}"), day.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.occasionplan".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CHILD_PLANS" => Some(g_CHILD_PLANS()),
            "PLANS" => Some(g_PLANS()),
            "RELATIONS" => Some(g_RELATIONS()),
            "SOON_PLANS" => Some(g_SOON_PLANS()),
            "_ANOTHER_PERSON" => Some(g__ANOTHER_PERSON()),
            "_ASKS_FOR_A_PLAN" => Some(g__ASKS_FOR_A_PLAN()),
            "_A_MONTH_ON_ITS_OWN" => Some(g__A_MONTH_ON_ITS_OWN()),
            "_A_NAME_BEFORE_THE_DAY" => Some(g__A_NAME_BEFORE_THE_DAY()),
            "_A_POSSESSIVE_BEFORE_THE_DAY" => Some(g__A_POSSESSIVE_BEFORE_THE_DAY()),
            "_BIRTHDAY_IN_PLAN" => Some(g__BIRTHDAY_IN_PLAN()),
            "_DAY_SAID" => Some(g__DAY_SAID()),
            "_DAY_TO_COME" => Some(g__DAY_TO_COME()),
            "_MINE_BEFORE_THE_DAY" => Some(g__MINE_BEFORE_THE_DAY()),
            "_NOT_A_PLAN_ASK" => Some(g__NOT_A_PLAN_ASK()),
            "_OWN_SUBJECT" => Some(g__OWN_SUBJECT()),
            "_SOMEONE_ELSES_DAY" => Some(g__SOMEONE_ELSES_DAY()),
            "_SOON" => Some(g__SOON()),
            "_SPOKEN_FOR" => Some(g__SPOKEN_FOR()),
            "_STILL_TALKING_ABOUT" => Some(g__STILL_TALKING_ABOUT()),
            "_TURNS_AGE" => Some(g__TURNS_AGE()),
            "_cap" => Some(py_func("_cap", |args: &[Py]| _cap(args))),
            "_ordinal" => Some(py_func("_ordinal", |args: &[Py]| _ordinal(args))),
            "_reads_as_a_plan_ask" => Some(py_func("_reads_as_a_plan_ask", |args: &[Py]| _reads_as_a_plan_ask(args))),
            "_said_how_long_ago" => Some(py_func("_said_how_long_ago", |args: &[Py]| _said_how_long_ago(args))),
            "_the_age" => Some(py_func("_the_age", |args: &[Py]| _the_age(args))),
            "_the_day" => Some(py_func("_the_day", |args: &[Py]| _the_day(args))),
            "_the_occasion" => Some(py_func("_the_occasion", |args: &[Py]| _the_occasion(args))),
            "_the_person" => Some(py_func("_the_person", |args: &[Py]| _the_person(args))),
            "_the_recent_age" => Some(py_func("_the_recent_age", |args: &[Py]| _the_recent_age(args))),
            "_whose" => Some(py_func("_whose", |args: &[Py]| _whose(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CHILD_PLANS" => s_CHILD_PLANS(v),
            "PLANS" => s_PLANS(v),
            "RELATIONS" => s_RELATIONS(v),
            "SOON_PLANS" => s_SOON_PLANS(v),
            "_ANOTHER_PERSON" => s__ANOTHER_PERSON(v),
            "_ASKS_FOR_A_PLAN" => s__ASKS_FOR_A_PLAN(v),
            "_A_MONTH_ON_ITS_OWN" => s__A_MONTH_ON_ITS_OWN(v),
            "_A_NAME_BEFORE_THE_DAY" => s__A_NAME_BEFORE_THE_DAY(v),
            "_A_POSSESSIVE_BEFORE_THE_DAY" => s__A_POSSESSIVE_BEFORE_THE_DAY(v),
            "_BIRTHDAY_IN_PLAN" => s__BIRTHDAY_IN_PLAN(v),
            "_DAY_SAID" => s__DAY_SAID(v),
            "_DAY_TO_COME" => s__DAY_TO_COME(v),
            "_MINE_BEFORE_THE_DAY" => s__MINE_BEFORE_THE_DAY(v),
            "_NOT_A_PLAN_ASK" => s__NOT_A_PLAN_ASK(v),
            "_OWN_SUBJECT" => s__OWN_SUBJECT(v),
            "_SOMEONE_ELSES_DAY" => s__SOMEONE_ELSES_DAY(v),
            "_SOON" => s__SOON(v),
            "_SPOKEN_FOR" => s__SPOKEN_FOR(v),
            "_STILL_TALKING_ABOUT" => s__STILL_TALKING_ABOUT(v),
            "_TURNS_AGE" => s__TURNS_AGE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.occasionplan", module_obj);
    pyrt::xrefs::reg_fn("compose.occasionplan", "_ordinal", py_func("_ordinal", |args: &[Py]| _ordinal(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_whose", py_func("_whose", |args: &[Py]| _whose(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_the_day", py_func("_the_day", |args: &[Py]| _the_day(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_the_age", py_func("_the_age", |args: &[Py]| _the_age(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_said_how_long_ago", py_func("_said_how_long_ago", |args: &[Py]| _said_how_long_ago(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_the_person", py_func("_the_person", |args: &[Py]| _the_person(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_the_recent_age", py_func("_the_recent_age", |args: &[Py]| _the_recent_age(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_the_occasion", py_func("_the_occasion", |args: &[Py]| _the_occasion(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_cap", py_func("_cap", |args: &[Py]| _cap(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "_reads_as_a_plan_ask", py_func("_reads_as_a_plan_ask", |args: &[Py]| _reads_as_a_plan_ask(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_fn("compose.occasionplan", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.occasionplan", "CHILD_PLANS", g_CHILD_PLANS, s_CHILD_PLANS);
    pyrt::xrefs::reg_access("compose.occasionplan", "PLANS", g_PLANS, s_PLANS);
    pyrt::xrefs::reg_access("compose.occasionplan", "RELATIONS", g_RELATIONS, s_RELATIONS);
    pyrt::xrefs::reg_access("compose.occasionplan", "SOON_PLANS", g_SOON_PLANS, s_SOON_PLANS);
    pyrt::xrefs::reg_access("compose.occasionplan", "_ANOTHER_PERSON", g__ANOTHER_PERSON, s__ANOTHER_PERSON);
    pyrt::xrefs::reg_access("compose.occasionplan", "_ASKS_FOR_A_PLAN", g__ASKS_FOR_A_PLAN, s__ASKS_FOR_A_PLAN);
    pyrt::xrefs::reg_access("compose.occasionplan", "_A_MONTH_ON_ITS_OWN", g__A_MONTH_ON_ITS_OWN, s__A_MONTH_ON_ITS_OWN);
    pyrt::xrefs::reg_access("compose.occasionplan", "_A_NAME_BEFORE_THE_DAY", g__A_NAME_BEFORE_THE_DAY, s__A_NAME_BEFORE_THE_DAY);
    pyrt::xrefs::reg_access("compose.occasionplan", "_A_POSSESSIVE_BEFORE_THE_DAY", g__A_POSSESSIVE_BEFORE_THE_DAY, s__A_POSSESSIVE_BEFORE_THE_DAY);
    pyrt::xrefs::reg_access("compose.occasionplan", "_BIRTHDAY_IN_PLAN", g__BIRTHDAY_IN_PLAN, s__BIRTHDAY_IN_PLAN);
    pyrt::xrefs::reg_access("compose.occasionplan", "_DAY_SAID", g__DAY_SAID, s__DAY_SAID);
    pyrt::xrefs::reg_access("compose.occasionplan", "_DAY_TO_COME", g__DAY_TO_COME, s__DAY_TO_COME);
    pyrt::xrefs::reg_access("compose.occasionplan", "_MINE_BEFORE_THE_DAY", g__MINE_BEFORE_THE_DAY, s__MINE_BEFORE_THE_DAY);
    pyrt::xrefs::reg_access("compose.occasionplan", "_NOT_A_PLAN_ASK", g__NOT_A_PLAN_ASK, s__NOT_A_PLAN_ASK);
    pyrt::xrefs::reg_access("compose.occasionplan", "_OWN_SUBJECT", g__OWN_SUBJECT, s__OWN_SUBJECT);
    pyrt::xrefs::reg_access("compose.occasionplan", "_SOMEONE_ELSES_DAY", g__SOMEONE_ELSES_DAY, s__SOMEONE_ELSES_DAY);
    pyrt::xrefs::reg_access("compose.occasionplan", "_SOON", g__SOON, s__SOON);
    pyrt::xrefs::reg_access("compose.occasionplan", "_SPOKEN_FOR", g__SPOKEN_FOR, s__SPOKEN_FOR);
    pyrt::xrefs::reg_access("compose.occasionplan", "_STILL_TALKING_ABOUT", g__STILL_TALKING_ABOUT, s__STILL_TALKING_ABOUT);
    pyrt::xrefs::reg_access("compose.occasionplan", "_TURNS_AGE", g__TURNS_AGE, s__TURNS_AGE);
}
