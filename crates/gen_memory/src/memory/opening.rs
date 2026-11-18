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


thread_local! { static G_A_CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CRUSH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_FOR_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DOING_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIKED_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_AS_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STANDING_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TASTE_OF_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_CORRECTION() -> Py { ensure_init(); G_A_CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CORRECTION(v: Py) { ensure_init(); G_A_CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CRUSH() -> Py { ensure_init(); G_A_CRUSH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CRUSH(v: Py) { ensure_init(); G_A_CRUSH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_FOR_SOMEONE() -> Py { ensure_init(); G_A_DAY_FOR_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_FOR_SOMEONE(v: Py) { ensure_init(); G_A_DAY_FOR_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_TAILS() -> Py { ensure_init(); G_A_DAY_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_TAILS(v: Py) { ensure_init(); G_A_DAY_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DOING_TAILS() -> Py { ensure_init(); G_A_DOING_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DOING_TAILS(v: Py) { ensure_init(); G_A_DOING_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIKED_TAILS() -> Py { ensure_init(); G_A_LIKED_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIKED_TAILS(v: Py) { ensure_init(); G_A_LIKED_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEASURE() -> Py { ensure_init(); G_A_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEASURE(v: Py) { ensure_init(); G_A_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONTHS() -> Py { ensure_init(); G_A_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONTHS(v: Py) { ensure_init(); G_A_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAST() -> Py { ensure_init(); G_A_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAST(v: Py) { ensure_init(); G_A_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_AS_OBJECT() -> Py { ensure_init(); G_A_PERSON_AS_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_AS_OBJECT(v: Py) { ensure_init(); G_A_PERSON_AS_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STANDING_STATE() -> Py { ensure_init(); G_A_STANDING_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STANDING_STATE(v: Py) { ensure_init(); G_A_STANDING_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_START() -> Py { ensure_init(); G_A_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_START(v: Py) { ensure_init(); G_A_START.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE_TAILS() -> Py { ensure_init(); G_A_STATE_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE_TAILS(v: Py) { ensure_init(); G_A_STATE_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TASTE() -> Py { ensure_init(); G_A_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TASTE(v: Py) { ensure_init(); G_A_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TASTE_OF_A_PERSON() -> Py { ensure_init(); G_A_TASTE_OF_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TASTE_OF_A_PERSON(v: Py) { ensure_init(); G_A_TASTE_OF_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME() -> Py { ensure_init(); G_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME(v: Py) { ensure_init(); G_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_HE() -> Py { ensure_init(); G_HE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HE(v: Py) { ensure_init(); G_HE.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY() -> Py { ensure_init(); G_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); G_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_OCCASION() -> Py { ensure_init(); G_NOT_AN_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_OCCASION(v: Py) { ensure_init(); G_NOT_AN_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_SHE() -> Py { ensure_init(); G_SHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHE(v: Py) { ensure_init(); G_SHE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY() -> Py { ensure_init(); G_THEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY(v: Py) { ensure_init(); G_THEY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The first line of a chat is answered, not filed.\n\n`bot/memory/ack.py` says the fact back, which is the right thing on turn five: the person has\njust added a detail to something the chat is already about, and \"Noted: your daughter's 6 next\nmonth\" reads as the bot keeping it. As the *first* thing anybody says it is a filing cabinet -\nnothing comes back about the six-year-old, the revision or the week. The person typed a line\nexpecting a reply, and a receipt of their own sentence is not one.\n\nSo on an opener the plain receipt frames are swapped for the one question a friend asks about\nthat fact. Which question follows the fact, not the words, and there are three shapes:\n\n* a day coming for somebody - an age, a birthday, a start date: what the day will be like;\n* something they are doing at a time - revising tonight, exams in May: how it is going;\n* a standing state - six days a week, living with parents: how it is working out.\n\nThe subject's pronoun comes from the person the fact is about, so a line about their daughter\nasks \"how is she\" and a line about themselves asks \"how are you\". The pools are read through\nbot.compose.pick, so two openers of the same shape do not get the same sentence.\n");
    s_A_DAY_FOR_SOMEONE(stdlib_call("re", "compile", &[py_str("(?:\\b(?:turns?|is|'s|am|'m|are|'re|re) \\d+\\b|\\b(?:im|i'm|i am) \\d+\\b|\\b\\d+ (?:next|this|in|on)\\b|\\bbirthday\\b|\\bbday\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:tonight|this evening|this morning|this afternoon|tomorrow|the day after tomorrow|next (?:week|month|year|weekend|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|this weekend|at the weekend|on monday|on tuesday|on wednesday|on thursday|on friday|on saturday|on sunday|in (?:january|february|march|april|may|june|july|august|september|october|november|december)|in the (?:morning|afternoon|evening)|later today|for the rest of the (?:day|week))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STANDING_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:every (?:day|week|night|morning|month)|(?:twice|three times|four times|five times|six times) a (?:day|week)|\\b(?:six|seven|five|four) days a week|live with|live(s)? with|living with|work(s)? (?:in|at|for|from|nights|shifts|weekends)|have (?:two|three|four|five) (?:kids|children|jobs))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TASTE(stdlib_call("re", "compile", &[py_str("^(?:and |so |but )?(?P<who>i|he|she|they|we|my (?:[\\w']+ )?[\\w']+)\\s+(?:really |just |absolutely |also |still )*(?:loves?|likes?|enjoys?|prefers?|(?:is|are) (?:really |very |well )?(?:into|a (?:big )?fan of))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TASTE_OF_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:you|me|him|her|them|us)\\s*[.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CRUSH(stdlib_call("re", "compile", &[py_str("\\b(?:a (?:girl|boy|man|woman|lady|lad)|someone|somebody|anyone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_AS_OBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:mum|mom|mother|dad|father|son|daughter|kids?|children|boys?|girls?|wife|husband|partner|brother|sister|nana|gran|grandma|grandad|granny|family|parents|baby|friend|mates?|neighbours?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_START(stdlib_call("re", "compile", &[py_str("\\b(?:starts?|starting|begins?|goes? back|going back|finish(?:es)?|leaves?|left) (?:year|school|college|uni|university|nursery|reception)\\b|\\b(?:has|sits?|taking|takes) (?:his|her|their|the) (?:gcses?|exams?|a levels?|mocks?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHE(py_tuple(vec![py_str("daughter"), py_str("sister"), py_str("mum"), py_str("mother"), py_str("mam"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("aunt"), py_str("auntie"), py_str("wife"), py_str("girlfriend"), py_str("partner"), py_str("niece"), py_str("cousin"), py_str("girl"), py_str("granddaughter"), py_str("stepmum"), py_str("stepdaughter"), py_str("mother in law"), py_str("daughter in law"), py_str("sister in law")]));
    s_HE(py_tuple(vec![py_str("son"), py_str("brother"), py_str("dad"), py_str("father"), py_str("grandad"), py_str("granddad"), py_str("uncle"), py_str("husband"), py_str("boyfriend"), py_str("nephew"), py_str("cousin"), py_str("boy"), py_str("grandson"), py_str("stepdad"), py_str("stepson"), py_str("father in law"), py_str("son in law"), py_str("brother in law")]));
    s_THEY(py_tuple(vec![py_str("kids"), py_str("children"), py_str("parents"), py_str("friends"), py_str("twins"), py_str("boys"), py_str("girls"), py_str("little one"), py_str("little ones"), py_str("toddler"), py_str("baby"), py_str("child")]));
    s_A_DAY_TAILS(py_tuple(vec![py_str("{Thing} — what are you planning for it?"), py_str("{Thing}. What would {subj} like to do?"), py_str("{Thing} — is there anything {subj} {has} asked for?"), py_str("{Thing}. What {be} {subj} thinking of doing?")]));
    s_A_LIKED_TAILS(py_tuple(vec![py_str("{Thing} — what got {them} into it?"), py_str("{Thing}. Is it a new thing, or {has} {subj} always been keen?"), py_str("{Thing} — what's the best bit of it for {them}?"), py_str("{Thing}. How long has that been going on?"), py_str("{Thing} — is that a recent thing, or a lifelong one?")]));
    s_A_DOING_TAILS(py_tuple(vec![py_str("{Thing} — how {be} {subj} feeling about it?"), py_str("{Thing} — how {be} {subj} feeling about it?"), py_str("{Thing}. How's it going so far?"), py_str("{Thing}. How's it going so far?"), py_str("{Thing} — {be} {subj} looking forward to it?")]));
    s_A_STATE_TAILS(py_tuple(vec![py_str("{Thing} — how's that working out?"), py_str("{Thing}. How's that treating you?"), py_str("{Thing} — what's that like?")]));
    s_A_PAST(stdlib_call("re", "compile", &[py_str("\\b(?:yesterday|last (?:night|week|weekend|month|year|summer|christmas|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|this (?:morning|afternoon)|earlier|ago|used to|was going to|were going to|had|saw|met|went|got married|moved|spent|asked|rang|kicked|took|cooked|bought|ate|paid|came|left|started|finished|arrived)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:rent|savings|bill|bills|budget|wage|wages|salary|debt|loan|cost|price|payment|money|credit card|overdraft|mortgage)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEASURE(stdlib_call("re", "compile", &[py_str("\\b(?:foot|feet|ft|cm|centimetres?|metres?|stone|kilos?|kilograms?|pounds?|inches|inch)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONTHS(stdlib_call("re", "compile", &[py_str("\\b\\d+ months?\\b|\\bmonths? old\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_OCCASION(stdlib_call("re", "compile", &[py_str("\\b(?:age|date of birth|dob|birthday|rent|savings)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CORRECTION(stdlib_call("re", "compile", &[py_str("(?:^\\s*(?:no|nope|sorry)\\b|\\b(?:i (?:said|meant|told you|didn't say|did not say))\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_day_is_the_point(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let _ = py_str("Whether the age is what the line is about rather than background: \"my daughter is 14 and\n    sleeping till noon\" is a complaint with an age in it, not a birthday, and \"i'm 34 and my wife\n    is 32\" has two days in it, so \"it\" would stand for neither of them.");
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&g_A_DAY_FOR_SOMEONE(), "findall", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("\\b\\d{1,3}\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:and|but|n)\\b"), slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _names_my(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lower: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lower") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lower"))) };
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("Whether the line names that relation of theirs, however it is typed: \"my daughter\",\n    \"my daughters\", \"my daughter's 6\", \"my daughters' party\".");
    return call_builtin("bool", &[stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\bmy (?:\\w+ )?"), &word.clone()), &py_str("(?:s|'s|s')?\\b")), lower.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _who(args: &[Py]) -> Py {
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
    let mut lower: Py = Py::None;
    let mut starts: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"you\" for a fact about the speaker, and the third person for one about somebody named.");
    lower = mcall(&clean.clone(), "lower", &[]);
    lower = stdlib_call("re", "sub", &[py_str("^\\s*(?:and |so |but |well )?(?:they|he|she|people|someone|everyone)\\s+(?:said|says|say|reckons?|thinks?|told me|tell me)\\s+"), py_str(""), lower.clone()]).unwrap_or_else(|e| exc::raise(e));
    starts = stdlib_call("re", "match", &[py_str("^\\s*(?:and |so |but |well )?(he|she|they|theyre|they're)\\b"), lower.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&starts.clone()) {
    who = mcall(&starts.clone(), "group", &[Py::Int(1)]);
    who = if py_truthy(&mcall(&who.clone(), "startswith", &[py_str("they")])) { py_str("they") } else { who.clone() };
    return py_tuple(vec![who.clone(), if py_truthy(&Py::Bool(py_eq(&who.clone(), &py_str("they")))) { py_str("are") } else { py_str("is") }]);
    }
    let __iter1 = py_iter_or_raise(&g_THEY());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&_names_my(&[lower.clone(), word.clone()])) {
    return py_tuple(vec![py_str("they"), py_str("are")]);
    }
    }
    let __iter2 = py_iter_or_raise(&g_SHE());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&_names_my(&[lower.clone(), word.clone()])) {
    return py_tuple(vec![py_str("she"), py_str("is")]);
    }
    }
    let __iter3 = py_iter_or_raise(&g_HE());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&_names_my(&[lower.clone(), word.clone()])) {
    return py_tuple(vec![py_str("he"), py_str("is")]);
    }
    }
    return py_tuple(vec![py_str("you"), py_str("are")]);
    Py::None
}
pub fn _said_before(args: &[Py]) -> Py {
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
    let _ = py_str("Whether anything has been answered yet: the current line is already in the turns with no\n    reply beside it, so the test is for a turn the bot has answered.");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_opening(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut be: Py = Py::None;
    let mut day: Py = Py::None;
    let mut has: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subj: Py = Py::None;
    let mut t: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut taste: Py = Py::None;
    let mut them: Py = Py::None;
    let _ = py_str("The question about the fact, where the plain receipt would be the whole reply.\n\n    Only the first line of a chat: once anything has been said the receipt is a detail landing on\n    a conversation, which is what bot/memory/ack.py's frames are for.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&clean.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&_said_before(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_PAST(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MEASURE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_MONTHS(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_CORRECTION(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_MONEY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MONEY(), "search", &[thing.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_AN_OCCASION(), "search", &[thing.clone()]) } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_who(&[clean.clone()]), 2, None);
    subj = __unpk1[0].clone();
    be = __unpk1[1].clone();
    has = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("you"), py_str("they")]), &subj.clone()))) { py_str("have") } else { py_str("has") };
    them = mcall(&py_dict(vec![(py_str("you"), py_str("you")), (py_str("she"), py_str("her")), (py_str("he"), py_str("him")), (py_str("they"), py_str("them"))]), "get", &[subj.clone(), py_str("you")]);
    taste = mcall(&g_A_TASTE(), "match", &[clean.clone()]);
    if py_truthy(&{ let __b = taste.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_TASTE_OF_A_PERSON(), "search", &[thing.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_CRUSH(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PERSON_AS_OBJECT(), "search", &[slice_or_raise(&clean.clone(), &mcall(&taste.clone(), "end", &[]), &Py::None, &Py::None)]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&slice_or_raise(&clean.clone(), &mcall(&taste.clone(), "end", &[]), &Py::None, &Py::None), &py_str(" but "))) } } } } } } } }) {
    taste = Py::None;
    }
    if py_truthy(&taste.clone()) {
    pool = g_A_LIKED_TAILS();
    }
    day = if py_truthy(&taste.clone()) { Py::None } else { mcall(&g_A_DAY_FOR_SOMEONE(), "search", &[clean.clone()]) };
    if py_truthy(&{ let __b = day.clone(); if !py_truthy(&__b) { __b } else { { let __b = _the_day_is_the_point(&[clean.clone(), day.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_AN_OCCASION(), "search", &[thing.clone()]))) } } } }) {
    pool = if py_truthy(&Py::Bool(py_eq(&subj.clone(), &py_str("you")))) { call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_A_DAY_TAILS()) { let mut t: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&t.clone(), &py_str("asked for")))) {  __out.push(t.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } else { g_A_DAY_TAILS() };
    if py_truthy(&Py::Bool(py_eq(&subj.clone(), &py_str("you")))) {
    pool = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut tail: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_contains(&tail.clone(), &py_str("would {subj} like to do")))) { py_str("{Thing} — anything you're marking it with?") } else { tail.clone() }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    } else {
    if py_truthy(&day.clone()) {
    return Py::None;
    } else {
    if py_truthy(&taste.clone()) {
    } else {
    if py_truthy(&{ let __b = mcall(&g_A_TIME(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_START(), "search", &[clean.clone()]) } }) {
    pool = g_A_DOING_TAILS();
    } else {
    if py_truthy(&mcall(&g_A_STANDING_STATE(), "search", &[clean.clone()])) {
    pool = g_A_STATE_TAILS();
    } else {
    return Py::None;
    }
    }
    }
    }
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Thing".to_string(), op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))), ("subj".to_string(), subj.clone()), ("be".to_string(), be.clone()), ("has".to_string(), has.clone()), ("them".to_string(), them.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    return mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.opening".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CORRECTION" => Some(g_A_CORRECTION()),
            "A_CRUSH" => Some(g_A_CRUSH()),
            "A_DAY_FOR_SOMEONE" => Some(g_A_DAY_FOR_SOMEONE()),
            "A_DAY_TAILS" => Some(g_A_DAY_TAILS()),
            "A_DOING_TAILS" => Some(g_A_DOING_TAILS()),
            "A_LIKED_TAILS" => Some(g_A_LIKED_TAILS()),
            "A_MEASURE" => Some(g_A_MEASURE()),
            "A_MONTHS" => Some(g_A_MONTHS()),
            "A_PAST" => Some(g_A_PAST()),
            "A_PERSON_AS_OBJECT" => Some(g_A_PERSON_AS_OBJECT()),
            "A_STANDING_STATE" => Some(g_A_STANDING_STATE()),
            "A_START" => Some(g_A_START()),
            "A_STATE_TAILS" => Some(g_A_STATE_TAILS()),
            "A_TASTE" => Some(g_A_TASTE()),
            "A_TASTE_OF_A_PERSON" => Some(g_A_TASTE_OF_A_PERSON()),
            "A_TIME" => Some(g_A_TIME()),
            "HE" => Some(g_HE()),
            "MONEY" => Some(g_MONEY()),
            "NOT_AN_OCCASION" => Some(g_NOT_AN_OCCASION()),
            "SHE" => Some(g_SHE()),
            "THEY" => Some(g_THEY()),
            "_names_my" => Some(py_func("_names_my", |args: &[Py]| _names_my(args))),
            "_said_before" => Some(py_func("_said_before", |args: &[Py]| _said_before(args))),
            "_the_day_is_the_point" => Some(py_func("_the_day_is_the_point", |args: &[Py]| _the_day_is_the_point(args))),
            "_who" => Some(py_func("_who", |args: &[Py]| _who(args))),
            "the_opening" => Some(py_func("the_opening", |args: &[Py]| the_opening(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CORRECTION" => s_A_CORRECTION(v),
            "A_CRUSH" => s_A_CRUSH(v),
            "A_DAY_FOR_SOMEONE" => s_A_DAY_FOR_SOMEONE(v),
            "A_DAY_TAILS" => s_A_DAY_TAILS(v),
            "A_DOING_TAILS" => s_A_DOING_TAILS(v),
            "A_LIKED_TAILS" => s_A_LIKED_TAILS(v),
            "A_MEASURE" => s_A_MEASURE(v),
            "A_MONTHS" => s_A_MONTHS(v),
            "A_PAST" => s_A_PAST(v),
            "A_PERSON_AS_OBJECT" => s_A_PERSON_AS_OBJECT(v),
            "A_STANDING_STATE" => s_A_STANDING_STATE(v),
            "A_START" => s_A_START(v),
            "A_STATE_TAILS" => s_A_STATE_TAILS(v),
            "A_TASTE" => s_A_TASTE(v),
            "A_TASTE_OF_A_PERSON" => s_A_TASTE_OF_A_PERSON(v),
            "A_TIME" => s_A_TIME(v),
            "HE" => s_HE(v),
            "MONEY" => s_MONEY(v),
            "NOT_AN_OCCASION" => s_NOT_AN_OCCASION(v),
            "SHE" => s_SHE(v),
            "THEY" => s_THEY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.opening", module_obj);
    pyrt::xrefs::reg_fn("memory.opening", "_the_day_is_the_point", py_func("_the_day_is_the_point", |args: &[Py]| _the_day_is_the_point(args)));
    pyrt::xrefs::reg_fn("memory.opening", "_names_my", py_func("_names_my", |args: &[Py]| _names_my(args)));
    pyrt::xrefs::reg_fn("memory.opening", "_who", py_func("_who", |args: &[Py]| _who(args)));
    pyrt::xrefs::reg_fn("memory.opening", "_said_before", py_func("_said_before", |args: &[Py]| _said_before(args)));
    pyrt::xrefs::reg_fn("memory.opening", "the_opening", py_func("the_opening", |args: &[Py]| the_opening(args)));
    pyrt::xrefs::reg_access("memory.opening", "A_CORRECTION", g_A_CORRECTION, s_A_CORRECTION);
    pyrt::xrefs::reg_access("memory.opening", "A_CRUSH", g_A_CRUSH, s_A_CRUSH);
    pyrt::xrefs::reg_access("memory.opening", "A_DAY_FOR_SOMEONE", g_A_DAY_FOR_SOMEONE, s_A_DAY_FOR_SOMEONE);
    pyrt::xrefs::reg_access("memory.opening", "A_DAY_TAILS", g_A_DAY_TAILS, s_A_DAY_TAILS);
    pyrt::xrefs::reg_access("memory.opening", "A_DOING_TAILS", g_A_DOING_TAILS, s_A_DOING_TAILS);
    pyrt::xrefs::reg_access("memory.opening", "A_LIKED_TAILS", g_A_LIKED_TAILS, s_A_LIKED_TAILS);
    pyrt::xrefs::reg_access("memory.opening", "A_MEASURE", g_A_MEASURE, s_A_MEASURE);
    pyrt::xrefs::reg_access("memory.opening", "A_MONTHS", g_A_MONTHS, s_A_MONTHS);
    pyrt::xrefs::reg_access("memory.opening", "A_PAST", g_A_PAST, s_A_PAST);
    pyrt::xrefs::reg_access("memory.opening", "A_PERSON_AS_OBJECT", g_A_PERSON_AS_OBJECT, s_A_PERSON_AS_OBJECT);
    pyrt::xrefs::reg_access("memory.opening", "A_STANDING_STATE", g_A_STANDING_STATE, s_A_STANDING_STATE);
    pyrt::xrefs::reg_access("memory.opening", "A_START", g_A_START, s_A_START);
    pyrt::xrefs::reg_access("memory.opening", "A_STATE_TAILS", g_A_STATE_TAILS, s_A_STATE_TAILS);
    pyrt::xrefs::reg_access("memory.opening", "A_TASTE", g_A_TASTE, s_A_TASTE);
    pyrt::xrefs::reg_access("memory.opening", "A_TASTE_OF_A_PERSON", g_A_TASTE_OF_A_PERSON, s_A_TASTE_OF_A_PERSON);
    pyrt::xrefs::reg_access("memory.opening", "A_TIME", g_A_TIME, s_A_TIME);
    pyrt::xrefs::reg_access("memory.opening", "HE", g_HE, s_HE);
    pyrt::xrefs::reg_access("memory.opening", "MONEY", g_MONEY, s_MONEY);
    pyrt::xrefs::reg_access("memory.opening", "NOT_AN_OCCASION", g_NOT_AN_OCCASION, s_NOT_AN_OCCASION);
    pyrt::xrefs::reg_access("memory.opening", "SHE", g_SHE, s_SHE);
    pyrt::xrefs::reg_access("memory.opening", "THEY", g_THEY, s_THEY);
}
