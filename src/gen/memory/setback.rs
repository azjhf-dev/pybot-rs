#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ABOUT_OCCASION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABOUT_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BLAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOES_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELPLESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MODIFIER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MORE_THAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DOER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONGOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWNER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VAGUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHOLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHOLE_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YOURS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_PLURAL_ENDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_OCCASION() -> Py { ensure_init(); ABOUT_OCCASION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_OCCASION(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_OCCASION.write() { *w = v; } }
pub fn g_ABOUT_THING() -> Py { ensure_init(); ABOUT_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THING(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THING.write() { *w = v; } }
pub fn g_BLAME() -> Py { ensure_init(); BLAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLAME(v: Py) { ensure_init(); if let Ok(mut w) = BLAME.write() { *w = v; } }
pub fn g_GOES_ON() -> Py { ensure_init(); GOES_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOES_ON(v: Py) { ensure_init(); if let Ok(mut w) = GOES_ON.write() { *w = v; } }
pub fn g_HELPLESS() -> Py { ensure_init(); HELPLESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELPLESS(v: Py) { ensure_init(); if let Ok(mut w) = HELPLESS.write() { *w = v; } }
pub fn g_JOINERS() -> Py { ensure_init(); JOINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOINERS(v: Py) { ensure_init(); if let Ok(mut w) = JOINERS.write() { *w = v; } }
pub fn g_MODIFIER() -> Py { ensure_init(); MODIFIER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MODIFIER(v: Py) { ensure_init(); if let Ok(mut w) = MODIFIER.write() { *w = v; } }
pub fn g_MORE_THAN() -> Py { ensure_init(); MORE_THAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MORE_THAN(v: Py) { ensure_init(); if let Ok(mut w) = MORE_THAN.write() { *w = v; } }
pub fn g_NOT_A_DOER() -> Py { ensure_init(); NOT_A_DOER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DOER(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DOER.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_NOW() -> Py { ensure_init(); NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOW(v: Py) { ensure_init(); if let Ok(mut w) = NOW.write() { *w = v; } }
pub fn g_ONGOING() -> Py { ensure_init(); ONGOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONGOING(v: Py) { ensure_init(); if let Ok(mut w) = ONGOING.write() { *w = v; } }
pub fn g_OWNER() -> Py { ensure_init(); OWNER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWNER(v: Py) { ensure_init(); if let Ok(mut w) = OWNER.write() { *w = v; } }
pub fn g_PEOPLE() -> Py { ensure_init(); PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE.write() { *w = v; } }
pub fn g_VAGUE() -> Py { ensure_init(); VAGUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VAGUE(v: Py) { ensure_init(); if let Ok(mut w) = VAGUE.write() { *w = v; } }
pub fn g_WHOLE() -> Py { ensure_init(); WHOLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHOLE(v: Py) { ensure_init(); if let Ok(mut w) = WHOLE.write() { *w = v; } }
pub fn g_WHOLE_NOW() -> Py { ensure_init(); WHOLE_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHOLE_NOW(v: Py) { ensure_init(); if let Ok(mut w) = WHOLE_NOW.write() { *w = v; } }
pub fn g_YOURS() -> Py { ensure_init(); YOURS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YOURS(v: Py) { ensure_init(); if let Ok(mut w) = YOURS.write() { *w = v; } }
pub fn g__NOT_A_PLURAL_ENDING() -> Py { ensure_init(); _NOT_A_PLURAL_ENDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PLURAL_ENDING(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_PLURAL_ENDING.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Something went wrong for them, and they have not said how.\n\n\"my cake was a disaster\", \"the boiler went wrong\", \"the party went badly\", \"everything went\nwrong today\": the person is handing over a setback and waiting for the one thing a friend asks,\nwhich is what happened. Every one of these lines was answered by the bot filing it away -\n\"Got it - your cake was a disaster\", \"Noted: the curry was a disaster\", \"You ruined everything,\ngood to know\" - or with a stock line that would fit any message (\"Got it. Is there anything\nyou'd like a hand with?\"). Saying a person's bad news back to them as a receipt is the coldest\nreply in STYLE.md, and on a line this open it also answers nothing.\n\nSo this reads the vague setback and the composer asks about it (bot/compose/trouble.py, which\nowns the turn for \"something of theirs has gone wrong\"): a short acknowledgement and the one\nquestion the line leaves open, in the person's own thing where the line names one (\"what\nhappened with your cake?\", not \"what happened?\").\n\nThe same line in the present tense is the other half of this: \"the deadline is killing me\",\n\"this spreadsheet is killing me\", \"the commute is a nightmare\". Nothing has happened yet, so the\nquestion is not what happened but which part of it wears most (\"what's the worst bit of it?\"),\nwhich is the reply STYLE.md gives for a spreadsheet that is killing somebody. Both were filed\naway before - \"Got it - the deadline is killing you\".\n\nGuards, because the shape is easy to over-read:\n\n* only the vague report. The line has to stop at the setback: \"the cake went wrong and it sank\"\n  or \"my laptop went wrong after the update\" says what happened, and the entry that diagnoses\n  it has the better answer, so a line that goes on is left alone.\n* never a question (\"why did my cake go wrong?\"), never a line about somebody else (\"my mum was\n  a nightmare\" - trouble.py refuses those too), never a line about their body (\"my knee is\n  killing me\" is an ache, and the body modules answer it), and never a line that names its own\n  fault (\"my cake flopped\", \"my sourdough failed\"): those are the knowledge base's diagnosis.\n* the thing named has to be theirs: \"my hair went wrong\" is \"your hair\", not \"the hair\", and a\n  day, a week or a life is spoken to as a day rather than as a gadget (\"what did your life do?\"\n  is the question this file exists to avoid).\n");
    s_VAGUE(stdlib_call("re", "compile", &[py_str("\\b(?:\n        went\\s+wrong | gone\\s+wrong\n      | (?:was|were|has\\s+been|have\\s+been|turned\\s+out)\\s+(?:a\\s+|an\\s+)?\n        (?:(?:bit\\s+of\\s+a|total|complete|real|proper|right|absolute|utter|huge|massive|\n             big)\\s+)?\n        (?:disaster|catastrophe|flop|washout|shambles|nightmare)\n      | went\\s+badly | went\\s+pear[\\s-]?shaped | came\\s+out\\s+wrong\n    )\\b"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|'s|'re)\\s+(?:really\\s+|properly\\s+|still\\s+|just\\s+)?\n    (?:\n        killing\\s+me | doing\\s+my\\s+(?:head|nut)\\s+in\n      | driving\\s+me\\s+(?:mad|crazy|nuts|insane|up\\s+the\\s+wall|round\\s+the\\s+bend)\n    )\n  | \\b(?:is|are|'s|'re)\\s+(?:really\\s+|still\\s+|all\\s+|just\\s+)?going\\s+wrong\\b\n  # The same thing said as a gradual decline rather than a fault: \"the training is going\n  # badly\", \"the lessons are not going well\", \"its going badly\". Read as a fact to file, each of\n  # these was answered \"Got it - the training is going badly\", which is the coldest reply in\n  # STYLE.md on a line that has just handed over something going wrong.\n  | \\b(?:isn['\\u2019]?t|aren['\\u2019]?t|is\\s+not|are\\s+not|not)\n    \\s+going\\s+(?:well|great|smoothly|to\\s+plan|right)\\b\n  | \\b(?:is|are|'s|'re|was|were)\\s+(?:really\\s+|still\\s+|all\\s+|just\\s+)*\n    going\\s+(?:badly|awfully|terribly|wrong)\\b\n  | \\bits\\s+(?:all\\s+|really\\s+|still\\s+|just\\s+)?\n    (?:not\\s+going\\s+(?:well|great|smoothly|to\\s+plan|right)|going\\s+(?:badly|awfully|terribly|wrong))\\b\n  | \\b(?:is|are|'s|'re)\\s+(?:a\\s+|an\\s+)?\n    (?:(?:bit\\s+of\\s+a|total|complete|real|proper|right|absolute|utter|huge|massive|big)\\s+)?\n    (?:disaster|catastrophe|nightmare|shambles)\\b\n  # A thing of theirs wearing them down over time: \"work has been getting me down lately\",\n  # \"the commute is getting me down\", \"my job gets me down\". The auxiliary is optional because\n  # people leave it out (\"work gets me down\"), and \"has been\"/\"have been\" is how the same\n  # thing is said when it has gone on a while. A person is not a thing here - \"my boss gets me\n  # down\" names somebody, and `_refused` leaves those to bot/compose/harm.py.\n  | \\b(?:(?:is|are|was|were|'s|'re)\\s+)?(?:(?:has|have|had)\\s+been\\s+|been\\s+)?\n    (?:getting|gets|got|wearing|wears|wore)\\s+(?:me|us)\\s+down\\b\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELPLESS(stdlib_call("re", "compile", &[py_str("\\s*(?:,|\\band\\b|\\bbut\\b|\\bso\\b)\\s*\n    (?:\\w+\\s+){0,3}?\n    (?:i\\s+)?(?:don['\\u2019]?t|do\\s+not|dont|can['\\u2019]?t|cannot|cant)\\s+\n    (?:really\\s+|even\\s+|honestly\\s+)?\n    (?:know\\s+what\\s+to\\s+do|know\\s+where\\s+to\\s+start|cope|keep\\s+going|take\\s+it\\s+any\\s+more)\n  | \\s*(?:,|\\band\\b|\\bbut\\b)\\s*\n    (?:\\w+\\s+){0,3}?i\\s*(?:['\\u2019]m|m\\b|am)\\s*(?:at\\s+my\\s+wits['\\u2019]?\\s*end|at\\s+the\\s+end\\s+of\\s+my\\s+tether)\n"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BLAME(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:really\\s+|totally\\s+|completely\\s+|properly\\s+|just\\s+)*\n    (?:messed|screwed|balls(?:ed)?|buggered|wrecked)\\s+(?:it\\s+)?up\\b\n    |\\b(?:i|we)\\s+(?:really\\s+|totally\\s+|completely\\s+)?(?:ruined|wrecked)\\b"), stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OWNER(stdlib_call("re", "compile", &[py_str("\\b(my|our|the|this|that|his|her|their)\\s+(?:whole\\s+|entire\\s+)?([a-z][a-z'-]{1,18})(?:\\s+([a-z][a-z'-]{1,18}))?")]).unwrap_or_else(|e| exc::raise(e)));
    s_MODIFIER(call_builtin("frozenset", &[mcall(&py_str("\n    first second third last next previous other own new old little big huge favourite favorite\n    front back spare cheap expensive birthday christmas driving washing running first\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_YOURS(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\n    day week month year weekend morning evening night time life everything anything it all\n    whole world lot bit thing way start begin end minute hour afternoon\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PEOPLE(call_builtin("frozenset", &[mcall(&py_str("\n    he she him her his hers they them their mum mam mom mummy dad daddy father mother parent\n    son daughter child kid kids baby toddler teenager boy girl husband wife partner boyfriend\n    girlfriend fiance fiancee nan nana gran granny grandma grandad grandpa grandparents brother\n    sister sibling aunt uncle cousin nephew niece friend mate flatmate housemate neighbour\n    neighbor colleague boss manager landlord landlady tenant carer nurse vet dog cat puppy\n    kitten pet\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GOES_ON(call_builtin("frozenset", &[py_tuple(vec![py_str("and"), py_str("but"), py_str("because"), py_str("cos"), py_str("cause"), py_str("so"), py_str("then"), py_str("when"), py_str("after"), py_str("before"), py_str("while"), py_str("which")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MORE_THAN(Py::Int(4));
    s_ABOUT_THING(py_tuple(vec![py_str("Oh no — what happened with {thing}?"), py_str("Ah, that's a shame. What went wrong with {thing}?"), py_str("That's rotten — what happened with {thing}?"), py_str("Ah no. What went wrong with {thing}?"), py_str("Oh no. How did {thing} go wrong?")]));
    s_ABOUT_OCCASION(add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ABOUT_THING()) { let mut line: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&line.clone(), &py_str("what happened with"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("what went wrong with"))) } }) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("Oh no. What went wrong with {thing}?")])));
    s_ONGOING(py_tuple(vec![py_str("Ugh — {thing} sounds relentless. What's the worst bit of it?"), py_str("That's a lot to be carrying. What's the hardest part of {thing}?"), py_str("Ah no. What's the worst of {thing} at the moment?"), py_str("I bet. What is it about {thing} that's wearing you down?")]));
    s_WHOLE(py_tuple(vec![py_str("Sounds like one of those days. What went wrong?"), py_str("Oh no — what happened?"), py_str("Ouch. What went wrong?"), py_str("Ah, everything at once? What happened?")]));
    s_WHOLE_NOW(py_tuple(vec![py_str("Ugh, that sounds relentless. What's the worst bit of it?"), py_str("That's a lot at the moment. What's the hardest part?"), py_str("Ah no. What is it that's wearing you down?")]));
    s_JOINERS(call_builtin("frozenset", &[py_tuple(vec![py_str("to"), py_str("for"), py_str("at"), py_str("in"), py_str("on"), py_str("onto"), py_str("into"), py_str("with"), py_str("and"), py_str("or"), py_str("but"), py_str("because"), py_str("since"), py_str("after"), py_str("before"), py_str("while"), py_str("when"), py_str("that"), py_str("which"), py_str("who"), py_str("is"), py_str("was"), py_str("were"), py_str("are"), py_str("am"), py_str("be"), py_str("been"), py_str("being"), py_str("went"), py_str("goes"), py_str("going"), py_str("gone"), py_str("has"), py_str("have"), py_str("had"), py_str("keeps"), py_str("kept"), py_str("makes"), py_str("made"), py_str("turned"), py_str("turns"), py_str("came"), py_str("comes"), py_str("last"), py_str("next"), py_str("this"), py_str("the"), py_str("a"), py_str("an"), py_str("of"), py_str("about"), py_str("from"), py_str("by"), py_str("so"), py_str("as"), py_str("then"), py_str("now"), py_str("today"), py_str("yesterday"), py_str("again"), py_str("too"), py_str("really"), py_str("still"), py_str("just"), py_str("all"), py_str("totally"), py_str("completely"), py_str("absolutely"), py_str("properly")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DOER(call_builtin("frozenset", &[py_tuple(vec![py_str("interview"), py_str("exam"), py_str("test"), py_str("party"), py_str("meeting"), py_str("date"), py_str("holiday"), py_str("essay"), py_str("presentation"), py_str("wedding"), py_str("birthday"), py_str("appointment"), py_str("shift"), py_str("lesson"), py_str("class"), py_str("review"), py_str("application"), py_str("audit"), py_str("trial"), py_str("day")])]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_PLURAL_ENDING(py_tuple(vec![py_str("ss"), py_str("us"), py_str("is"), py_str("ws")]));
}
pub fn _flat(args: &[Py]) -> Py {
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
pub fn _thing(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut qualifier: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person's own words for the thing that went wrong, or \"\" for a day or nothing.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[mcall(&g_OWNER(), "finditer", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)]), { let __b = mcall(&match_.clone(), "group", &[Py::Int(3)]); if py_truthy(&__b) { __b } else { py_str("") } }]), 3, None);
    qualifier = __unpk2[0].clone();
    word = __unpk2[1].clone();
    after = __unpk2[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("whole"), py_str("entire")]), &word.clone())); if !py_truthy(&__b) { __b } else { after.clone() } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![after.clone(), py_str("")]), 2, None);
    word = __unpk3[0].clone();
    after = __unpk3[1].clone();
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MODIFIER(), &word.clone())); if !py_truthy(&__b) { __b } else { after.clone() } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![add(&add(&word.clone(), &py_str(" ")), &after.clone()), py_str("")]), 2, None);
    word = __unpk4[0].clone();
    after = __unpk4[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_JOINERS(), &after.clone()))) {
    after = py_str("");
    }
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone()))) {
    return py_str("");
    }
    owner = if py_truthy(&Py::Bool(py_contains(&g_YOURS(), &qualifier.clone()))) { py_str("your") } else { py_str("the") };
    return mcall(&add(&add(&add(&owner.clone(), &py_str(" ")), &word.clone()), &if py_truthy(&after.clone()) { add(&py_str(" "), &after.clone()) } else { py_str("") }), "strip", &[]);
    }
    return py_str("");
    Py::None
}
pub fn _named_after(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing after a blaming verb: \"i messed up the cake\" -> \"your cake\".");
    match_ = stdlib_call("re", "search", &[py_str("\\b(?:the|my|our|this|that)\\s+(?:whole\\s+)?([a-z][a-z'-]{1,18})(?:\\s+([a-z][a-z'-]{1,18}))?"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), { let __b = mcall(&match_.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } }]), 2, None);
    word = __unpk1[0].clone();
    after = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MODIFIER(), &word.clone())); if !py_truthy(&__b) { __b } else { after.clone() } }) {
    let __unpk2 = py_unpack(&py_tuple(vec![add(&add(&word.clone(), &py_str(" ")), &after.clone()), py_str("")]), 2, None);
    word = __unpk2[0].clone();
    after = __unpk2[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_JOINERS(), &after.clone()))) {
    after = py_str("");
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone()))) {
    return py_str("");
    }
    return add(&add(&py_str("your "), &word.clone()), &if py_truthy(&after.clone()) { add(&py_str(" "), &after.clone()) } else { py_str("") });
    Py::None
}
pub fn _bare_subject(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing a line names with no \"my\"/\"the\" in front of it: \"work is killing me\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&head.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("been"), py_str("has"), py_str("have"), py_str("had"), py_str("really"), py_str("just")]), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    word = geti(&words.clone(), &neg(&Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("im"), py_str("it"), py_str("that"), py_str("this"), py_str("there"), py_str("he"), py_str("she"), py_str("they"), py_str("we"), py_str("you")]), &word.clone())) } }) {
    return py_str("");
    }
    return word.clone();
    Py::None
}
pub fn _refused(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let _ = py_str("Whether the thing the line names is not a thing to ask about.");
    return { let __b = call_builtin("bool", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PEOPLE(), &geti(&mcall(&name.clone(), "split", &[]), &neg(&Py::Int(1))))) } };
    Py::None
}
pub fn reading(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut tense: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(name, \"past\") | (name, \"now\") for a setback or a thing wearing on them, or None.\n\n    `name` is the person's own phrase for the thing (\"your cake\"), or \"\" for a whole day and\n    for a person blaming themselves without naming it, which are answered with the same short\n    question in its own tense.\n    ");
    clean = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_VAGUE(), py_str("past")]), py_tuple(vec![g_NOW(), py_str("now")]), py_tuple(vec![g_BLAME(), py_str("past")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    tense = __unpk2[1].clone();
    match_ = mcall(&pattern.clone(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    after = mcall(&slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "split", &[]);
    rest = if py_truthy(&after.clone()) { mcall(&mcall(&g_HELPLESS(), "sub", &[py_str(""), slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]), "split", &[]) } else { py_list(vec![]) };
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MORE_THAN()); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_GOES_ON(), &mcall(&word.clone(), "strip", &[py_str(".,!")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    name = _thing(&[slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&pattern.clone(), &g_BLAME())) } }) {
    name = _named_after(&[slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&pattern.clone(), &g_NOW())) } }) {
    name = _bare_subject(&[slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]);
    }
    if py_truthy(&_refused(&[name.clone()])) {
    return Py::None;
    }
    return py_tuple(vec![name.clone(), tense.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _a_plural_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut last: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the phrase for the thing reads as a plural (\"the lessons\", \"my feet\").");
    words = mcall(&call_builtin("str", &[{ let __b = name.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    last = mcall(&mcall(&geti(&words.clone(), &neg(&Py::Int(1))), "lower", &[]), "strip", &[py_str(".,!?;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&last.clone(), "endswith", &[py_str("s")])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&last.clone(), "endswith", &[g__NOT_A_PLURAL_ENDING()])));
    Py::None
}
pub fn _agrees_with_the_thing(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let _ = py_str("The pool line with its verb made to agree with the thing that fills it.");
    if py_truthy(&_a_plural_thing(&[name.clone()])) {
    return mcall(&mcall(&line.clone(), "replace", &[py_str(" sounds "), py_str(" sound ")]), "replace", &[py_str(" it sounds "), py_str(" they sound ")]);
    }
    return line.clone();
    Py::None
}
pub fn pool(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tense: Py = Py::None;
    let _ = py_str("The replies for a setback, in the person's own thing where the line named one.");
    said = reading(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_tuple(vec![]);
    }
    let __unpk1 = py_unpack(&said.clone(), 2, None);
    name = __unpk1[0].clone();
    tense = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return if py_truthy(&Py::Bool(py_eq(&tense.clone(), &py_str("now")))) { g_WHOLE_NOW() } else { g_WHOLE() };
    }
    if py_truthy(&Py::Bool(py_eq(&tense.clone(), &py_str("now")))) {
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ONGOING()) { let mut line: Py = __item.clone(); __out.push(_agrees_with_the_thing(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), name.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }, name.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    last = geti(&mcall(&name.clone(), "split", &[]), &neg(&Py::Int(1)));
    lines = if py_truthy(&Py::Bool(py_contains(&g_NOT_A_DOER(), &last.clone()))) { g_ABOUT_OCCASION() } else { g_ABOUT_THING() };
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), name.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.setback".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_OCCASION" => Some(g_ABOUT_OCCASION()),
            "ABOUT_THING" => Some(g_ABOUT_THING()),
            "BLAME" => Some(g_BLAME()),
            "GOES_ON" => Some(g_GOES_ON()),
            "HELPLESS" => Some(g_HELPLESS()),
            "JOINERS" => Some(g_JOINERS()),
            "MODIFIER" => Some(g_MODIFIER()),
            "MORE_THAN" => Some(g_MORE_THAN()),
            "NOT_A_DOER" => Some(g_NOT_A_DOER()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOW" => Some(g_NOW()),
            "ONGOING" => Some(g_ONGOING()),
            "OWNER" => Some(g_OWNER()),
            "PEOPLE" => Some(g_PEOPLE()),
            "VAGUE" => Some(g_VAGUE()),
            "WHOLE" => Some(g_WHOLE()),
            "WHOLE_NOW" => Some(g_WHOLE_NOW()),
            "YOURS" => Some(g_YOURS()),
            "_NOT_A_PLURAL_ENDING" => Some(g__NOT_A_PLURAL_ENDING()),
            "_a_plural_thing" => Some(py_func("_a_plural_thing", |args: &[Py]| _a_plural_thing(args))),
            "_agrees_with_the_thing" => Some(py_func("_agrees_with_the_thing", |args: &[Py]| _agrees_with_the_thing(args))),
            "_bare_subject" => Some(py_func("_bare_subject", |args: &[Py]| _bare_subject(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_named_after" => Some(py_func("_named_after", |args: &[Py]| _named_after(args))),
            "_refused" => Some(py_func("_refused", |args: &[Py]| _refused(args))),
            "_thing" => Some(py_func("_thing", |args: &[Py]| _thing(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "reading" => Some(py_func("reading", |args: &[Py]| reading(args))),
            _ => None,
        }),
        set: None,
    }))
}
