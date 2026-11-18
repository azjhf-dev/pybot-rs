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


pub static AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGAIN_HELP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANOTHER_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANY_SEPARATOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_AMOUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DETAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONEY_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SCRIPT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STILL_GOING_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CANNOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAUSE_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COLON_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETAIL_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FACT_RECALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_THE_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIKE_I_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_MORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONE_WORD_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENING_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PATTERNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RETURN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAME_ADVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEPARATOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNDERNEATH_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNDERNEATH_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_THE_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RULED_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AGAIN() -> Py { ensure_init(); AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = AGAIN.write() { *w = v; } }
pub fn g_AGAIN_HELP() -> Py { ensure_init(); AGAIN_HELP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGAIN_HELP(v: Py) { ensure_init(); if let Ok(mut w) = AGAIN_HELP.write() { *w = v; } }
pub fn g_ANOTHER_ONE() -> Py { ensure_init(); ANOTHER_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_ONE(v: Py) { ensure_init(); if let Ok(mut w) = ANOTHER_ONE.write() { *w = v; } }
pub fn g_ANY_SEPARATOR() -> Py { ensure_init(); ANY_SEPARATOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANY_SEPARATOR(v: Py) { ensure_init(); if let Ok(mut w) = ANY_SEPARATOR.write() { *w = v; } }
pub fn g_AN_AMOUNT() -> Py { ensure_init(); AN_AMOUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_AMOUNT(v: Py) { ensure_init(); if let Ok(mut w) = AN_AMOUNT.write() { *w = v; } }
pub fn g_AN_ASK() -> Py { ensure_init(); AN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = AN_ASK.write() { *w = v; } }
pub fn g_A_DETAIL() -> Py { ensure_init(); A_DETAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DETAIL(v: Py) { ensure_init(); if let Ok(mut w) = A_DETAIL.write() { *w = v; } }
pub fn g_A_MONEY_ASK() -> Py { ensure_init(); A_MONEY_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_MONEY_ASK.write() { *w = v; } }
pub fn g_A_PHRASE() -> Py { ensure_init(); A_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = A_PHRASE.write() { *w = v; } }
pub fn g_A_REPORT() -> Py { ensure_init(); A_REPORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPORT(v: Py) { ensure_init(); if let Ok(mut w) = A_REPORT.write() { *w = v; } }
pub fn g_A_SCRIPT() -> Py { ensure_init(); A_SCRIPT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SCRIPT(v: Py) { ensure_init(); if let Ok(mut w) = A_SCRIPT.write() { *w = v; } }
pub fn g_A_STILL_GOING_ON() -> Py { ensure_init(); A_STILL_GOING_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STILL_GOING_ON(v: Py) { ensure_init(); if let Ok(mut w) = A_STILL_GOING_ON.write() { *w = v; } }
pub fn g_BEING() -> Py { ensure_init(); BEING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEING(v: Py) { ensure_init(); if let Ok(mut w) = BEING.write() { *w = v; } }
pub fn g_CANNOT() -> Py { ensure_init(); CANNOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CANNOT(v: Py) { ensure_init(); if let Ok(mut w) = CANNOT.write() { *w = v; } }
pub fn g_CLAUSE_ONLY() -> Py { ensure_init(); CLAUSE_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = CLAUSE_ONLY.write() { *w = v; } }
pub fn g_COLON_LEADS() -> Py { ensure_init(); COLON_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COLON_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = COLON_LEADS.write() { *w = v; } }
pub fn g_DETAIL_AGAIN() -> Py { ensure_init(); DETAIL_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETAIL_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = DETAIL_AGAIN.write() { *w = v; } }
pub fn g_FACT_RECALL() -> Py { ensure_init(); FACT_RECALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FACT_RECALL(v: Py) { ensure_init(); if let Ok(mut w) = FACT_RECALL.write() { *w = v; } }
pub fn g_FEELING() -> Py { ensure_init(); FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = FEELING.write() { *w = v; } }
pub fn g_FOR_THE_PERSON() -> Py { ensure_init(); FOR_THE_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_THE_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = FOR_THE_PERSON.write() { *w = v; } }
pub fn g_FRAMES() -> Py { ensure_init(); FRAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAMES(v: Py) { ensure_init(); if let Ok(mut w) = FRAMES.write() { *w = v; } }
pub fn g_FRAME_WORDS() -> Py { ensure_init(); FRAME_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_WORDS.write() { *w = v; } }
pub fn g_LEADS() -> Py { ensure_init(); LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = LEADS.write() { *w = v; } }
pub fn g_LEAD_WORDS() -> Py { ensure_init(); LEAD_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = LEAD_WORDS.write() { *w = v; } }
pub fn g_LIKE_I_SAID() -> Py { ensure_init(); LIKE_I_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIKE_I_SAID(v: Py) { ensure_init(); if let Ok(mut w) = LIKE_I_SAID.write() { *w = v; } }
pub fn g_MONEY_SAID() -> Py { ensure_init(); MONEY_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY_SAID(v: Py) { ensure_init(); if let Ok(mut w) = MONEY_SAID.write() { *w = v; } }
pub fn g_NOTHING() -> Py { ensure_init(); NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING.write() { *w = v; } }
pub fn g_NOTHING_MORE() -> Py { ensure_init(); NOTHING_MORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_MORE(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_MORE.write() { *w = v; } }
pub fn g_ONE_WORD_LEAD() -> Py { ensure_init(); ONE_WORD_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONE_WORD_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = ONE_WORD_LEAD.write() { *w = v; } }
pub fn g_OPENING_FILLER() -> Py { ensure_init(); OPENING_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENING_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = OPENING_FILLER.write() { *w = v; } }
pub fn g_PATTERNS() -> Py { ensure_init(); PATTERNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PATTERNS(v: Py) { ensure_init(); if let Ok(mut w) = PATTERNS.write() { *w = v; } }
pub fn g_RETURN() -> Py { ensure_init(); RETURN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RETURN(v: Py) { ensure_init(); if let Ok(mut w) = RETURN.write() { *w = v; } }
pub fn g_SAID_BACK() -> Py { ensure_init(); SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = SAID_BACK.write() { *w = v; } }
pub fn g_SAME_ADVICE() -> Py { ensure_init(); SAME_ADVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAME_ADVICE(v: Py) { ensure_init(); if let Ok(mut w) = SAME_ADVICE.write() { *w = v; } }
pub fn g_SENTENCE() -> Py { ensure_init(); SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = SENTENCE.write() { *w = v; } }
pub fn g_SEPARATOR() -> Py { ensure_init(); SEPARATOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEPARATOR(v: Py) { ensure_init(); if let Ok(mut w) = SEPARATOR.write() { *w = v; } }
pub fn g_SHORT() -> Py { ensure_init(); SHORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); if let Ok(mut w) = SHORT.write() { *w = v; } }
pub fn g_SHORT_AGAIN() -> Py { ensure_init(); SHORT_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = SHORT_AGAIN.write() { *w = v; } }
pub fn g_SLOT() -> Py { ensure_init(); SLOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLOT(v: Py) { ensure_init(); if let Ok(mut w) = SLOT.write() { *w = v; } }
pub fn g_UNDERNEATH_CLAUSE() -> Py { ensure_init(); UNDERNEATH_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNDERNEATH_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = UNDERNEATH_CLAUSE.write() { *w = v; } }
pub fn g_UNDERNEATH_SAID() -> Py { ensure_init(); UNDERNEATH_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNDERNEATH_SAID(v: Py) { ensure_init(); if let Ok(mut w) = UNDERNEATH_SAID.write() { *w = v; } }
pub fn g__NOT_THE_END() -> Py { ensure_init(); _NOT_THE_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_END(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_THE_END.write() { *w = v; } }
pub fn g__RULED_OUT() -> Py { ensure_init(); _RULED_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RULED_OUT(v: Py) { ensure_init(); if let Ok(mut w) = _RULED_OUT.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The same paragraph is about to be said twice: say the short of it, framed as the repeat it is.\n\nSTYLE.md rules out the same answer twice in a row. `bot/memory/topics.said_again` frames a repeat\nwith a lead-in, but it stands aside when the line names something of its own: \"he keeps coming\ninto our bed at 2am\" is a new detail, so the turn is not \"the same ask again\" and the lead-in is\nwithheld - and the entry the line reaches has nothing new to say, so the paragraph goes out byte\nfor byte as before. The person has just told the bot something and gets back what they already\nread:\n\n    my son is 6 and he wont sleep\n      Walk them back every time with the same short line - \"it's sleep time, I love you\" - and ...\n    he keeps coming into our bed at 2am\n      (the same paragraph, word for word)\n\nThe answer is still right, so it is not replaced: it is cut to the sentence with the move in it\nand the lead-in says the answer has not changed, which also leaves the door open for the person\nto say what did not work.\n");
    s_SHORT(Py::Int(14));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_AGAIN(py_tuple(vec![py_str("Same answer, and it covers this: {reply}"), py_str("That is still the thing to do: {reply}"), py_str("Nothing new to add on it: {reply}")]));
    s_SHORT_AGAIN(py_tuple(vec![py_str("Same as before: {reply}"), py_str("As I said: {reply}"), py_str("Still: {reply}")]));
    s_AGAIN_HELP(py_str("Same answer I'm afraid: {reply} If that hasn't worked for you, tell me what went wrong and I'll take it from there."));
    s_DETAIL_AGAIN(py_tuple(vec![py_str("Same for {detail}: {reply}"), py_str("Same answer for {detail}: {reply}"), py_str("It is the same for {detail}: {reply}"), py_str("Same again for {detail}: {reply}")]));
    s_SAME_ADVICE(py_tuple(vec![py_str("Nothing separate for {detail} - it is the same advice: {reply}"), py_str("Nothing different for {detail} I'm afraid: the same answer covers it - {reply}"), py_str("No separate fix for {detail} - the same advice as before: {reply}"), py_str("Nothing more for {detail}: the same answer covers it - {reply}")]));
    s_RETURN(py_tuple(vec![py_str("Back to {detail}: {reply}"), py_str("Back to {detail} then: {reply}"), py_str("Back on {detail} again: {reply}"), py_str("As we said about {detail}: {reply}")]));
    s_UNDERNEATH_SAID(py_tuple(vec![py_str("{said} is the same thing underneath, and the step is the same: {reply}"), py_str("{said} is the same thing again, so the step has not changed: {reply}")]));
    s_UNDERNEATH_CLAUSE(py_tuple(vec![py_str("{said} - that is the same thing underneath, and the step is the same: {reply}"), py_str("{said}, and it comes to the same thing: {reply}")]));
    s_NOTHING_MORE(py_tuple(vec![py_str("Nothing more on it I'm afraid: {reply}"), py_str("That is all I have on it: {reply}"), py_str("There is nothing further to add: {reply}")]));
    s_LIKE_I_SAID(py_tuple(vec![py_str("Like I said: {reply}")]));
    s_FRAMES(call_builtin("tuple", &[mcall(&py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), "fromkeys", &[add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&g_AGAIN(), &g_SHORT_AGAIN()), &py_tuple(vec![g_AGAIN_HELP()])), &g_DETAIL_AGAIN()), &g_SAME_ADVICE()), &g_RETURN()), &g_UNDERNEATH_SAID()), &g_UNDERNEATH_CLAUSE()), &g_NOTHING_MORE()), &g_LIKE_I_SAID()), &crate::gen::memory::topics::g_REPEAT())])]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_THE_PERSON(py_dict(vec![(py_str("i"), py_str("you")), (py_str("me"), py_str("you")), (py_str("my"), py_str("your")), (py_str("mine"), py_str("yours")), (py_str("myself"), py_str("yourself")), (py_str("am"), py_str("are")), (py_str("i'm"), py_str("you're")), (py_str("im"), py_str("you're")), (py_str("i've"), py_str("you've")), (py_str("i'll"), py_str("you'll")), (py_str("i'd"), py_str("you'd")), (py_str("we"), py_str("you")), (py_str("our"), py_str("your")), (py_str("us"), py_str("you")), (py_str("was"), py_str("were"))]));
    s_SLOT(stdlib_call("re", "compile", &[py_str("\\{[a-z_]+\\}")]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING(stdlib_call("re", "compile", &[py_str("^(?:i\\s+feel|i\\s+felt|i'?m\\s+feeling|i\\s+am\\s+feeling|feeling)\\s+(.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CANNOT(stdlib_call("re", "compile", &[py_str("^(?:i\\s+can'?t|i\\s+cannot|i\\s+can\\s+not|i'?m\\s+not\\s+able\\s+to)\\s+(.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BEING(stdlib_call("re", "compile", &[py_str("^(?:i'?m|i\\s+am)\\s+(?:so\\s+|really\\s+|very\\s+|still\\s+|just\\s+|quite\\s+)?(.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING(stdlib_call("re", "compile", &[py_str("^nothing\\s+(\\w+)\\s+me$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE_ONLY(stdlib_call("re", "compile", &[py_str("^(?:everything|nobody|nothing|life|things|it|this)\\s+\\S"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENING_FILLER(stdlib_call("re", "compile", &[py_str("^(?:so|and|but|yeah|yeah|well|honestly|tbh|idk|dunno|just|ok|okay|hi|hey)\\b[ ,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PHRASE(Py::Int(8));
    s__NOT_THE_END(call_builtin("frozenset", &[mcall(&py_str("a an the my your our his her their its this that these those some any"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SEPARATOR(stdlib_call("re", "compile", &[py_str("\\s*(?::|\\u2014|\\u2013|-)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ANY_SEPARATOR(py_str("\\s*(?::|\\u2014|\\u2013|-)\\s*"));
    s_LEADS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FRAMES()) { let mut frame: Py = __item.clone(); __out.push(geti(&mcall(&call_builtin("str", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("{reply}")]), &Py::Int(0))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_PATTERNS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FRAMES()) { let mut frame: Py = __item.clone(); __out.push(_lead(&[frame.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD_WORDS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FRAMES()) { let mut frame: Py = __item.clone(); __out.push(call_builtin("tuple", &[crate::gen::memory::topics::words(&[geti(&mcall(&call_builtin("str", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("{")]), &Py::Int(0))])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }) { let mut lead: Py = __item.clone(); if py_truthy(&gt(&call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {  __out.push(lead.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_COLON_LEADS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FRAMES()) { let mut frame: Py = __item.clone(); __out.push(call_builtin("tuple", &[crate::gen::memory::topics::words(&[geti(&mcall(&call_builtin("str", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("{")]), &Py::Int(0))])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }) { let mut lead: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {  __out.push(geti(&lead.clone(), &Py::Int(0))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_ONE_WORD_LEAD(if py_truthy(&g_COLON_LEADS()) { stdlib_call("re", "compile", &[modulo(&py_str("^(%s)\\s*[:,]\\s"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COLON_LEADS()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None });
    s_AN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|whats|which|why|how|when|where|who|whose|should|shall|could|would|can|do|does|did|is|are|was|were|any|anything)\\b|\\?")]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_WORDS(call_builtin("frozenset", &[mcall(&py_str("back again then about as said return returning goes going go to a an the\n    of in on at for and or but so also just still now it its this that these those i me my we our\n    you your they them their he she him her please thanks thank"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DETAIL(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:long|much|many|often|far|hot|old|quickly)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANOTHER_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:what|how) about\\b|\\b(?:bigger|smaller|larger|heavier|lighter|cheaper|dearer|pricier|costlier|older|younger|worse|better|other|different|longer|shorter|higher|lower|faster|nearer|further|farther|easier|harder|stronger|weaker|hotter|colder|warmer|quicker|newer|smarter|richer|poorer|nicer|simpler|tougher|thicker|thinner|slower)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_ASK(stdlib_call("re", "compile", &[py_str("\\bhow much\\b|\\bhow many (?:pounds|dollars|euros)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:owe|owes|owed|pay|paid|pays|cost|costs|price|prices|afford|spend|spent|bill|bills|budget|save|saved|earn|earns|salary|wage|wages|rent|money|card|loan|mortgage|debt|deposit|refund|quote|quoted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AMOUNT(stdlib_call("re", "compile", &[py_str("[\\u00a3$\\u20ac]|\\b\\d+(?:\\.\\d+)?\\s?(?:pounds?|quid|dollars?|euros?|bucks|pence|cents)\\b|\\b\\d+(?:,\\d{3})+\\b|\\b[bB]\\d{2,}\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPORT(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|i've|ive|we|we're|we've|my|our|he|he's|hes|she|she's|shes|they|they're|they've)\\s+\\w"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|how|why|when|where|who|can|could|should|would|shall|is|are|do|does|did|any|anything|anyone|remind|tell|told|worth)\\b|\\?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STILL_GOING_ON(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|kept|keeps on|still|again|always|every|each|wont|won't|never|constantly|over and over|every time|all the time)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SCRIPT(stdlib_call("re", "compile", &[py_str("(?:\\\"[^\\\"]{8,}\\\"|\\u201c[^\\u201d]{8,}\\u201d|(?<![\\w])'[^']{8,}'(?![\\w]))")]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_RECALL(stdlib_call("re", "compile", &[py_str("^(?:you(?:'ve| have) (?:got|had)|you (?:live|'re|are)|your \\w[\\w' -]{0,20}? (?:is|was)|i have you as|you told me (?:you|your|it))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_BACK(stdlib_call("re", "compile", &[py_str("^(?:you said\\b|you told me\\b|i have you as\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__RULED_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:no|not|without|instead|except|never|other\\s+than)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn the_name_the_answer_says(args: &[Py]) -> Py {
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
    let mut detail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut card: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "card") { v.clone() }
    else { py_str("") };
    let mut last: Py = Py::None;
    let mut led: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The longest run of `detail`'s own words that `answer` says, or \"\" for none.\n\n    A lead-in may only name a part of the answer the answer itself names. Two ways it went wrong\n    without that: a yes/no ask puts its verb at the end, so \"would a scratching post help\" named\n    \"a scratching post help\" and came back \"Nothing different for a scratching post help I'm\n    afraid\"; and \"should i buy a bigger one\" named \"a bigger\", which is a fragment rather than a\n    thing. The answer is the only dictionary this needs - it says \"post\" and never says \"help\" -\n    so nothing has to be kept up to date, and a detail the answer does say (\"the biting\") is\n    handed back whole.\n\n    The tail is dropped one word at a time, and a word that only points at one (\"a\", \"the\") never\n    ends the name: \"Same for a: ...\" is not a sentence.\n    ");
    said = crate::gen::memory::topics::words(&[call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    words = mcall(&call_builtin("str", &[{ let __b = detail.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    while py_truthy(&words.clone()) {
    last = mcall(&geti(&words.clone(), &neg(&Py::Int(1))), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__NOT_THE_END(), &last.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &last.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[last.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "startswith", &[last.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } }) {
    return mcall(&py_str(" "), "join", &[words.clone()]);
    }
    words = slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    whole = crate::gen::memory::topics::words(&[call_builtin("str", &[{ let __b = card.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    phrase = mcall(&call_builtin("str", &[{ let __b = detail.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    last = if py_truthy(&phrase.clone()) { mcall(&geti(&phrase.clone(), &neg(&Py::Int(1))), "lower", &[]) } else { py_str("") };
    led = { let __b = gt(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&phrase.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__NOT_THE_END(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = led.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__NOT_THE_END(), &last.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&whole.clone(), &last.clone())) } } } }) {
    return mcall(&call_builtin("str", &[{ let __b = detail.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    }
    return py_str("");
    Py::None
}
pub fn as_a_phrase(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut made: Py = Py::None;
    let mut part: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The person's own words for it as a phrase, and whether it can stand as a subject.\n\n    Returns `(kind, phrase)` where kind is \"said\" for a phrase that takes \"is\" (\"Feeling empty\"),\n    \"clause\" for a whole clause that does not (\"Nothing interests you\"), or None when the line is\n    not one of the shapes this frame can be built from. The pronouns are turned round, because the\n    phrase is the bot saying their own words back to them: \"i feel empty\" is \"Feeling empty\",\n    \"nothing interests me\" is \"Nothing interests you\".\n\n    The store hands the reader the chat's last lines joined (\"i feel low, i feel empty\"), so the\n    line is read clause by clause and the newest one that is a shape is the phrase: what the person\n    has just said is the word for it now.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    whole = _a_phrase_of(&[text.clone()]);
    if py_truthy(&whole.clone()) {
    return whole.clone();
    }
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[,;]|\\band\\b|\\bbut\\b|\\bso\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(part.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[if py_truthy(&gt(&call_builtin("len", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { slice_or_raise(&clauses.clone(), &Py::Int(1), &Py::None, &Py::None) } else { py_list(vec![]) }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    clause = __item.clone();
    made = _a_phrase_of(&[clause.clone()]);
    if py_truthy(&made.clone()) {
    return made.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_phrase_of(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut turned: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`as_a_phrase` for one clause of the line.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    line = mcall(&mcall(&g_OPENING_FILLER(), "sub", &[py_str(""), mcall(&mcall(&text.clone(), "strip", &[]), "rstrip", &[py_str(".!,")])]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_A_PHRASE()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("?"))) } } } }) {
    return Py::None;
    }
    found = mcall(&g_NOTHING(), "match", &[line.clone()]);
    if py_truthy(&found.clone()) {
    verb = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&verb.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("is"), py_str("was"), py_str("has")]), &verb.clone())) } }) {
    return py_tuple(vec![py_str("clause"), add(&add(&py_str("Nothing "), &verb.clone()), &py_str(" you"))]);
    }
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_FEELING(), g_CANNOT()]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[line.clone()]);
    if py_truthy(&found.clone()) {
    rest = _for_the_person(&[mcall(&found.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &sub(&g_A_PHRASE(), &Py::Int(1))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&rest.clone(), &py_str(","))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&rest.clone(), &py_str(";"))); if py_truthy(&__b) { __b } else { _says_i(&[mcall(&found.clone(), "group", &[Py::Int(1)])]) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("that"), py_str("like"), py_str("as"), py_str("so")]), &mcall(&geti(&mcall(&rest.clone(), "split", &[]), &Py::Int(0)), "lower", &[])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&pattern.clone(), &g_CANNOT()))) {
    return py_tuple(vec![py_str("said"), add(&py_str("Not being able to "), &rest.clone())]);
    }
    return py_tuple(vec![py_str("said"), add(&py_str("Feeling "), &rest.clone())]);
    }
    }
    found = mcall(&g_CLAUSE_ONLY(), "match", &[line.clone()]);
    if py_truthy(&found.clone()) {
    turned = _for_the_person(&[line.clone()]);
    return py_tuple(vec![py_str("clause"), add(&mcall(&slice_or_raise(&turned.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&turned.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    }
    found = mcall(&g_BEING(), "match", &[line.clone()]);
    if py_truthy(&found.clone()) {
    rest = _for_the_person(&[mcall(&found.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&Py::Bool(py_contains(&rest.clone(), &py_str(",")))) {
    return Py::None;
    }
    words = mcall(&rest.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&mcall(&geti(&words.clone(), &neg(&Py::Int(1))), "lower", &[]), "endswith", &[py_str("ing")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("going"), py_str("gonna"), py_str("about"), py_str("not")]), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("your")]), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return py_tuple(vec![py_str("said"), add(&py_str("Being "), &rest.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _says_i(args: &[Py]) -> Py {
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
    let _ = py_str("True when the words hold a first person still to turn, so they are more than one clause.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_FOR_THE_PERSON(), &mcall(&mcall(&word.clone(), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))]), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _for_the_person(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut ending: Py = Py::None;
    let mut head: Py = Py::None;
    let mut turned: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`text` with the first person turned round: \"my head is banging\" -> \"your head is banging\".");
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    bare = mcall(&mcall(&word.clone(), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))]), "lower", &[]);
    ending = slice_or_raise(&word.clone(), &call_builtin("len", &[mcall(&word.clone(), "rstrip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    head = if py_truthy(&ending.clone()) { slice_or_raise(&word.clone(), &Py::None, &sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None) } else { word.clone() };
    if py_truthy(&Py::Bool(py_contains(&g_FOR_THE_PERSON(), &bare.clone()))) {
    turned = geti(&g_FOR_THE_PERSON(), &bare.clone());
    if py_truthy(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) {
    turned = mcall(&turned.clone(), "capitalize", &[]);
    }
    let _ = mcall(&words.clone(), "append", &[add(&turned.clone(), &ending.clone())]);
    } else {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn the_same_thing_underneath(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut times: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "times") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: times"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut kind: Py = Py::None;
    let mut made: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The answer behind the person's own word for it, once the frame has been said twice, or \"\".\n\n    `times` is how many times in a row the chat has had this answer, counted before this turn. One\n    lead-in saying the answer has not changed is right; the second one is about the bot, so from\n    then on the person's word takes its place. \"\" when the line is not a shape this frame can be\n    built from, which leaves the caller's own wording.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&times.clone())); if py_truthy(&__b) { __b } else { lt(&times.clone(), &Py::Int(2)) } } } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = _carries_a_lead_in(&[_flat(&[reply.clone()])]); if py_truthy(&__b) { __b } else { unframed(&[reply.clone()]) } }) {
    return py_str("");
    }
    made = as_a_phrase(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&made.clone()))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&made.clone(), 2, None);
    kind = __unpk1[0].clone();
    said = __unpk1[1].clone();
    pool = if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("said")))) { g_UNDERNEATH_SAID() } else { g_UNDERNEATH_CLAUSE() };
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)) } else { times.clone() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), said.clone()), ("reply".to_string(), mcall(&reply.clone(), "strip", &[]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&pool.clone(), &modulo(&turns.clone(), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)))), "format", args))(&__args) };
    Py::None
}
pub fn _lead(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frame: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frame"))) };
    let mut head: Py = Py::None;
    let mut part: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("A pattern for the lead-in `frame` puts in front of its answer.\n\n    Anything in front of the answer is matched word for word, except the filled slots, which\n    stand for whatever was put in them. The separator is read as either the colon the frame is\n    written with or the dash it is written with when the answer holds a colon of its own: a\n    lead-in read only in its colon form stops its turn counting as a repeat, which is the fault\n    `_times_on_the_screen` was fixed for.");
    head = geti(&mcall(&call_builtin("str", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("{reply}")]), &Py::Int(0));
    tail = py_str("");
    stripped = mcall(&g_SEPARATOR(), "sub", &[py_str(""), mcall(&head.clone(), "rstrip", &[])]);
    if py_truthy(&Py::Bool(!py_eq(&stripped.clone(), &mcall(&head.clone(), "rstrip", &[])))) {
    let __unpk1 = py_unpack(&py_tuple(vec![stripped.clone(), g_ANY_SEPARATOR()]), 2, None);
    head = __unpk1[0].clone();
    tail = __unpk1[1].clone();
    }
    return stdlib_call("re", "compile", &[add(&add(&py_str("^"), &mcall(&py_str(".+?"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SLOT(), "split", &[head.clone()])) { let mut part: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &tail.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn unframed(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The answer under a repeat's lead-in, or None when the reply carries none.");
    mine = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_PATTERNS());
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "match", &[mine.clone()]);
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&mine.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]) } }) {
    return mcall(&slice_or_raise(&mine.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    }
    return Py::None;
    Py::None
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
    let _ = py_str("A reply reduced to its words, so a repeat is a repeat whatever the punctuation.");
    return mcall(&py_str(" "), "join", &[crate::gen::memory::topics::words(&[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]);
    Py::None
}
pub fn the_cause_said_again(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut at: Py = Py::None;
    let mut found: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut other: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The sentence that names the cause, when the line asked why and the answer has one.\n\n    A card is written for the fix and keeps the reason in a sentence of its own at the end: \"Pull\n    the hair and soap out of the trap first ... The reason is hair, soap and grease caught in the\n    trap and the bend.\" Asked the same why a second time, the short of an answer is its opening,\n    so the person who asked why was handed the steps again and the one sentence that answers them\n    was the part that got cut. A why line (``memory[\"understood\"][\"wants\"] == \"reason\"``,\n    `bot/modules/intent.py`) is owed the cause whichever time it is asked, so the cause sentence is\n    what is kept - and only a sentence the card itself frames as its cause (\"the reason is ...\",\n    \"the cause is ...\"), which is the reading `bot/compose/why` already does for the bare why\n    line. A card that only explains in passing (\"... is what stops the smell coming back\") is left\n    to its opening: the opening of a card is still its answer.\n    ");
    let mut why: Py = crate::gen::compose::why::module_obj();
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("wants")]), &py_str("reason"))) } }) {
    return Py::None;
    }
    named = call_builtin("getattr", &[why.clone(), py_str("NAMED_CAUSE"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&named.clone(), &Py::None))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![reply.clone(), _the_answer_in_hand(&[reply.clone(), memory.clone()]), _a_card_behind(&[memory.clone()])]));
    for __item in __iter1 {
    said = __item.clone();
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::quotes::sentences(&[said.clone(), g_SENTENCE()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    part = __unpk3[1].clone();
    found = mcall(&named.clone(), "search", &[part.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&mcall(&found.clone(), "start", &[]), &Py::Int(2)); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[mcall(&part.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&at.clone(), &Py::Int(0)))) {
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { _led_an_earlier_answer(&[part.clone(), memory.clone()]) } }) {
    return mcall(&py_str(" "), "join", &[slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None)]);
    }
    return Py::None;
    }
    move_ = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&parts.clone(), &Py::None, &at.clone(), &Py::None)) { let mut other: Py = __item.clone(); if py_truthy(&other.clone()) {  __out.push(other.clone()); } } py_list(__out) })() };
    if py_truthy(&_led_an_earlier_answer(&[part.clone(), memory.clone()])) {
    return if py_truthy(&gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { mcall(&py_str(" "), "join", &[slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None)]) } else { Py::None };
    }
    return add(&part.clone(), &if py_truthy(&move_.clone()) { add(&py_str(" "), &geti(&move_.clone(), &Py::Int(0))) } else { py_str("") });
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_card_behind(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The newest reply in this chat that carries a cause sentence of its own, or \"\".\n\n    \"why does that happen\" is asked two turns after the card as often as one (\"... and then\" in\n    between), and the answer they were given outright is still the answer: the last reply was the\n    steps of it and names no cause, so the card behind it is where the cause is.\n    ");
    let mut why: Py = crate::gen::compose::why::module_obj();
    named = call_builtin("getattr", &[why.clone(), py_str("NAMED_CAUSE"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_is(&named.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }))) {
    continue;
    }
    said = call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&crate::gen::memory::quotes::sentences(&[said.clone(), g_SENTENCE()]));
    for __item in __iter2 {
    part = __item.clone();
    found = mcall(&named.clone(), "search", &[mcall(&part.clone(), "strip", &[])]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let __b = le(&mcall(&found.clone(), "start", &[]), &Py::Int(2)); if !py_truthy(&__b) { __b } else { ge(&call_builtin("len", &[mcall(&part.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } } } }) {
    return said.clone();
    }
    }
    let __aug3 = add(&seen.clone(), &Py::Int(1));
    seen = __aug3;
    if py_truthy(&ge(&seen.clone(), &Py::Int(3))) {
    break;
    }
    }
    return py_str("");
    Py::None
}
pub fn _led_an_earlier_answer(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether a reply in this chat opened on this sentence, so it is not news a second time.\n\n    Two whys in a row ask for the same sentence twice, and the second one is owed something it has\n    not read - the rest of the card, which is where the answer's other sentences are. An answer\n    that only *mentions* the cause somewhere in the middle does not count: the person asking why\n    was given the steps, and the cause is still the part they were not given.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    if py_truthy(&mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "startswith", &[said.clone()])) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_answer_in_hand(args: &[Py]) -> Py {
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
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The answer on the screen, when what this turn is building is the opening of it.");
    mine = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]), &Py::None, &Py::Int(8), &Py::None)]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(25)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }))) {
    continue;
    }
    said = call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]), &mine.clone()))) {
    return said.clone();
    }
    let __aug2 = add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&ge(&seen.clone(), &Py::Int(3))) {
    break;
    }
    }
    return py_str("");
    Py::None
}
pub fn _short(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut cause: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The first sentence of an answer, or the first two when the first is only a lead-in.\n\n    Never stops inside a quotation: the words an answer hands over to say have full stops of\n    their own, and half a quotation is a broken answer rather than a short one (`quotes.closed`).\n    ");
    cause = the_cause_said_again(&[reply.clone(), memory.clone()]);
    if py_truthy(&cause.clone()) {
    return cause.clone();
    }
    parts = crate::gen::memory::quotes::sentences(&[reply.clone(), g_SENTENCE()]);
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[mcall(&geti(&parts.clone(), &Py::Int(0)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT()); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    kept = slice_or_raise(&parts.clone(), &Py::None, &Py::Int(1), &Py::None);
    } else {
    kept = slice_or_raise(&parts.clone(), &Py::None, &Py::Int(2), &Py::None);
    }
    return crate::gen::memory::quotes::closed(&[mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut part: Py = __item.clone(); __out.push(mcall(&part.clone(), "strip", &[])); } py_list(__out) })() }]), slice_or_raise(&parts.clone(), &call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), reply.clone()]);
    Py::None
}
pub fn _the_first_sentence(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut cause: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The sentence that carries the answer, so a return gives the move rather than the lot.");
    cause = the_cause_said_again(&[reply.clone(), memory.clone()]);
    if py_truthy(&cause.clone()) {
    return cause.clone();
    }
    parts = crate::gen::memory::quotes::sentences(&[reply.clone(), g_SENTENCE()]);
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    kept = slice_or_raise(&parts.clone(), &Py::None, &Py::Int(1), &Py::None);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[mcall(&geti(&parts.clone(), &Py::Int(0)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    kept = slice_or_raise(&parts.clone(), &Py::None, &Py::Int(2), &Py::None);
    }
    return crate::gen::memory::quotes::closed(&[mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut part: Py = __item.clone(); __out.push(mcall(&part.clone(), "strip", &[])); } py_list(__out) })() }]), slice_or_raise(&parts.clone(), &call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), reply.clone()]);
    Py::None
}
pub fn _carries_a_lead_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut lead: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the reply opens with a repeat's lead-in (\"as i said: ...\", \"same as before: ...\").");
    said = mcall(&py_str(" "), "join", &[crate::gen::memory::topics::words(&[mine.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&add(&call_builtin("tuple", &[crate::gen::memory::topics::g_REPEAT_LEADS()]).unwrap_or_else(|e| exc::raise(e)), &g_LEAD_WORDS()));
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&mcall(&said.clone(), "startswith", &[add(&mcall(&py_str(" "), "join", &[lead.clone()]), &py_str(" "))])) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g_ONE_WORD_LEAD(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g_ONE_WORD_LEAD(), "match", &[mcall(&call_builtin("str", &[{ let __b = mine.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lstrip", &[])]) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn carries_a_lead_in(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply already opens with a lead-in, so a second one may not go in front.\n\n    The rule the frame readers keep to, \"a lead-in is never stacked on another\": an answer said\n    again twice over reads as a glitch - \"Same as before: Same answer, and it covers this: Wash\n    the bin with hot soapy water\" - and the outer lead says nothing the inner one did not.");
    return _carries_a_lead_in(&[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _times_on_the_screen(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut body: Py = Py::None;
    let mut said: Py = Py::None;
    let mut times: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("How many turns in a row already said this reply, word for word or under a lead-in.\n\n    A repeat that was already framed still counts as a repeat. It did not before: the screen then\n    held \"Same answer, and it covers this: The ambulance is on its way...\", which is not the\n    paragraph word for word, so the count started again from nothing on every turn and the same\n    lead-in was picked every time - `AGAIN` holds three leads so a person who hears an answer a\n    third time gets \"Nothing new to add on it\", and they got \"Same answer, and it covers this\"\n    three turns running instead. The body under a lead is the short of the answer, so a run of\n    framed turns is counted by the answer starting with that short.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Int(0);
    }
    times = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    break;
    }
    said = mcall(&turn.clone(), "get", &[py_str("bot")]);
    if py_truthy(&Py::Bool(!py_eq(&_flat(&[said.clone()]), &mine.clone()))) {
    body = _flat(&[{ let __b = unframed(&[said.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&body.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&mine.clone(), "startswith", &[body.clone()]))) } }) {
    break;
    }
    }
    let __aug2 = add(&times.clone(), &Py::Int(1));
    times = __aug2;
    }
    return times.clone();
    Py::None
}
pub fn _said_earlier(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("True when this answer was already given somewhere earlier in the conversation.\n\n    Only a paragraph is counted, because a paragraph is what reads as a machine when it comes\n    back word for word: a short fact (\"On Tuesday.\", \"You're Sam.\") said again on a later turn\n    is the answer, not a repeat - people ask about the same fact twice and want the fact. The\n    turn just before is left to `_times_on_the_screen` above, which numbers its lead-ins; this is\n    for the answer the chat has moved on from and come back to.\n    ");
    return Py::Bool(!py_eq(&_said_earlier_text(&[mine.clone(), memory.clone()]), &py_str("")));
    Py::None
}
pub fn _said_earlier_text(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot turn that already says this answer, or \"\" when the conversation has not had it.");
    if py_truthy(&le(&call_builtin("len", &[mcall(&mine.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT())) {
    return py_str("");
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = _flat(&[mcall(&turn.clone(), "get", &[py_str("bot")])]);
    if py_truthy(&Py::Bool(py_eq(&said.clone(), &mine.clone()))) {
    return said.clone();
    }
    if py_truthy(&mcall(&said.clone(), "startswith", &[add(&mine.clone(), &py_str(" "))])) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _names_something_the_answer_says(args: &[Py]) -> Py {
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
    let mut mine: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names a thing this answer is about.\n\n    A word of the answer's own, in the line, and not a word every line has: \"its a wheelie\n    bin\" shares \"bin\" with the answer that says what to do with one. A line about something\n    else shares none of them, so the paragraph coming back on it stays the misreading it is and\n    gets no lead-in claiming it still applies.\n\n    A word that names nothing is not a thing the line is about, which is what `ASK_ONLY` is\n    for: \"what should i do first\" shares \"first\" with the boiler answer (\"you want the pump to\n    run on for a minute first\") and nothing else, and framing the answer on that word alone took\n    the turn off the composer's own short of it (\"Start with this: Turn the boiler off at the\n    wall switch\").\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::words(&[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&crate::gen::memory::topics::g_STOPWORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::memory::topics::g_ASK_ONLY(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[bitand(&said.clone(), &call_builtin("set", &[mcall(&mine.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _returned_thing(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut record: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing the person went back to on this turn, when the line names it: \"the tap\".\n\n    Read only for the lead-in of a reply already given: the store marks the return itself, so\n    this asks it rather than reading the line for opener words. The thing has to be said in the\n    line (\"back to the tap, what tools do i need\") so \"back to it\" - which names nothing - is\n    not given the lead, and a short thing only, because a lead carrying a whole clause is longer\n    than the answer it frames.\n    ");
    record = crate::gen::memory::topics::resumed(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    thing = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("asked")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&thing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_contains(&_flat(&[text.clone()]), &thing.clone()))) {
    return py_str("");
    }
    return thing.clone();
    Py::None
}
pub fn _the_line_asks(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line asks something, read the way the store read the return.\n\n    \"back to lisbon\" names a place and asks nothing: the person has said where they want to be and\n    not yet what they want, and the answer to that is a question back, not the paragraph from the\n    top of the chat. The store reads the return itself and says whether the line asks, so this asks\n    it rather than reading the words for question marks - the line the answer sees may have been\n    rewritten with a lead of its own (\"what about back to lisbon\"), and that is the rewrite's ask.\n    ");
    record = crate::gen::memory::topics::resumed(&[memory.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&record.clone(), "get", &[py_str("asks")]), &Py::None)) } }) {
    return call_builtin("bool", &[mcall(&record.clone(), "get", &[py_str("asks")])]).unwrap_or_else(|e| exc::raise(e));
    }
    said = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    return { let __b = Py::Bool(py_eq(&mcall(&said.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&said.clone(), "get", &[py_str("asks")])]).unwrap_or_else(|e| exc::raise(e)) } };
    }
    return call_builtin("bool", &[mcall(&g_AN_ASK(), "search", &[_flat(&[text.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _adds_nothing_the_answer_did_not_say(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut added: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut said_as_typed: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the return asks again for the answer, rather than asking something it covers.\n\n    The answer to a return may already have a part that answers what the line adds. \"back to the\n    guitar, should i get lessons or an app\" comes back to a paragraph that ends \"an app is fine for\n    a month; six lessons after that fix your hands\" - the words the line adds are in the answer, so\n    the part of it that answers them is the reply, and a lead saying the whole paragraph again is\n    the wrong answer under a claim that it is the same one. The words that decide this are the\n    ones the line adds over the return itself: the thing it named and the grammar of the line come\n    out, and every word left that the answer does not say is what makes this a new ask.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_the_line_asks(&[memory.clone(), text.clone()])))) {
    return Py::Bool(false);
    }
    said_as_typed = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    typed = if py_truthy(&call_builtin("isinstance", &[said_as_typed.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&said_as_typed.clone(), "get", &[py_str("text")]) } else { Py::None };
    asked = _flat(&[if py_truthy(&{ let __b = call_builtin("isinstance", &[typed.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&typed.clone(), "strip", &[]) } }) { typed.clone() } else { text.clone() }]);
    thing = call_builtin("set", &[mcall(&mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    added = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&asked.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&word.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FRAME_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&thing.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::understanding::lexicon::g_CORE_WORDS(), &word.clone())) } } } } } } } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    return Py::Bool(!py_truthy(&bitand(&added.clone(), &call_builtin("set", &[mcall(&_flat(&[said.clone()]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))));
    Py::None
}
pub fn _places(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let _ = py_str("The place names the vocabulary knows, so a lead can write one the way a name is written.");
    let __try1 = exc::catch(|| {
    let mut PLACES: Py = crate::gen::understanding::topics::g_PLACES();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ImportError"]) {
                __handled5 = true;
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return PLACES.clone();
    Py::None
}
pub fn _as_a_name(args: &[Py]) -> Py {
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
    let mut places: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing with a place in it written as a name: \"lisbon\" -> \"Lisbon\", \"new york\" -> \"New York\".\n\n    The line the lead is built from has been flattened to lowercase, so \"Back to lisbon\" would put\n    the place in lower case next to an answer that writes \"Alfama\" and \"Belem\". Only words in the\n    vocabulary of places are touched, so \"my knee\" and \"the tap\" stay as they are, and the words\n    an ordinary thing is made of are left alone.\n    ");
    places = _places(&[]);
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_contains(&places.clone(), &word.clone()))) { add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { word.clone() }); } py_list(__out) })() }]);
    Py::None
}
pub fn _without_the_thing_in_front(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut short: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "short") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: short"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The short of the answer with a phrase naming the thing taken off the front.\n\n    \"In Rome: the Colosseum with a timed ticket, ...\" under a lead that already says \"Back to\n    rome\" says the place twice before the answer starts. The phrase comes off, and only when it\n    names the same thing the lead does, so \"For a deposit, aim for 10%\" keeps its own words.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = short.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("^(?:in|at|on|for|about|with|around)\\s+%s\\s*[:,]?\\s+"), py_str("^%s\\s*[:,]\\s+")]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = stdlib_call("re", "match", &[modulo(&pattern.clone(), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None) } }) {
    rest = slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    return add(&mcall(&geti(&rest.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    return said.clone();
    Py::None
}
pub fn _said_recently(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut look: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look") { v.clone() }
    else { Py::Int(3) };
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("1 when this reply is among the last few bot turns word for word, turns between or not.\n\n    Only a run of the same reply was counted, so one answer in between made this one new again:\n    \"i have an exam next week\" / \"how do i use a slow cooker\" / \"i havent revised\" set the whole\n    study plan out again word for word, a turn after the person read it, with no lead-in at all.\n    The answer is still on their screen; what came between was an answer to something else.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Int(0);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(_flat(&[mcall(&turn.clone(), "get", &[py_str("bot")])])); } } py_list(__out) })() }) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    return if py_truthy(&Py::Bool(py_contains(&slice_or_raise(&said.clone(), &neg(&look.clone()), &Py::None, &Py::None), &mine.clone()))) { Py::Int(1) } else { Py::Int(0) };
    Py::None
}
pub fn _says_it_came_back(args: &[Py]) -> Py {
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
    let mut says: Py = Py::None;
    let _ = py_str("Whether the line reports the thing the last answer was about coming back.\n\n    \"my back is sore again\" after \"my back is sore\", \"and the rota changed again this morning\",\n    \"its dropped again\" in a boiler chat. `bot/memory/returned` holds the one reading of it; this\n    asks there so the frame and the return rules cannot disagree.\n    ");
    let __try1 = exc::catch(|| {
    let mut returned: Py = crate::gen::memory::returned::module_obj();
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
    says = call_builtin("getattr", &[returned.clone(), py_str("says_it_happened_again"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = says.clone(); if !py_truthy(&__b) { __b } else { py_call(&says, &[text.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_figure_asked_that_the_answer_does_not_hold(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut line: Py = Py::None;
    let _ = py_str("Whether the line asks for an amount of money and the reply holds no amount.\n\n    \"how much do i owe on the card\" after \"should i clear the card first\" was framed as the card's\n    answer come round again - \"Same answer, and it covers this: Pay more than the minimum ...\" -\n    when the chat had the figure said out (\"You said 2000\") and the card holds none of it. A frame\n    claims to cover the line, so it may not answer a money question with money-less prose. Read\n    here, before the frame is built, because both callers would otherwise serve it.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_MONEY_ASK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MONEY_SAID(), "search", &[line.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_AN_AMOUNT(), "search", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn _asks_for_a_detail(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks for a time, an amount or a count.");
    return call_builtin("bool", &[mcall(&g_A_DETAIL(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_report(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line tells the bot something about the person rather than asking for a part.\n\n    A report that the chat has not heard is new information, and the paragraph that came back on a\n    shared word is a misreading. A line that says the *same* trouble is still going on is not new\n    information: it is the subject of the answer already on the screen, and standing aside there\n    prints that answer word for word a second time.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_REPORT(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_AN_ASK(), "search", &[line.clone()]) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_A_STILL_GOING_ON(), "search", &[line.clone()])));
    Py::None
}
pub fn _has_a_script(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the answer holds a sentence in quotes, which is the thing a wording ask wants.");
    return call_builtin("bool", &[mcall(&g_A_SCRIPT(), "search", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_line_that_got_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut look: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look") { v.clone() }
    else { Py::Int(3) };
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line the answer `mine` was given to, among the last few turns, or \"\".");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &neg(&look.clone()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_flat(&[mcall(&turn.clone(), "get", &[py_str("bot")])]), &mine.clone())) } }) {
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _said_and_more(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last line was this reply and then a question of its own.\n\n    A reply the bot went on from is still on the screen: \"You have an exam on friday. How are\n    you feeling about it?\" is the fact plus a question the bot put there itself, and the fact\n    said back on its own is the same answer, not a new one. Only the whole reply counts here -\n    the short form is left to the exact match above, so a longer line is not mistaken for the\n    short of itself already given.\n\n    A line that starts with the answer is not the answer: \"You don't eat meat, good to know -\n    I'll plan around it.\" opens with the sentence the store would say and then talks about the\n    greeting, so the question is what separates the two. The bot's own follow-up question means\n    it went on from the answer; anything else means it only said the answer was news. The line\n    ends in that question, so it is the end of the line that says which of the two it was.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mine.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } } } }) {
    return Py::Bool(false);
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &neg(&Py::Int(1))) } else { py_dict(vec![]) };
    raw = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    said = _flat(&[raw.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&said.clone(), "startswith", &[add(&mine.clone(), &py_str(" "))]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&raw.clone(), "endswith", &[py_str("?")]))) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn already_had(args: &[Py]) -> Py {
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
    let mut mine: Py = Py::None;
    let _ = py_str("The reply behind a lead-in that says the chat has had this answer, or None.\n\n    `instead` frames an answer for a line that *returns* to the thing it is about, and it needs\n    the thing the person came back to, because that lead reads \"Back to the tap: ...\". A line that\n    asks the same thing again without naming a return is not that: `is that dangerous` about a\n    boiler the chat answered two turns before is the question again, and the paragraph that\n    answers it is already on the screen. Standing down there hands the turn to a vaguer line\n    (`Worth doing rather than worrying about`), so the answer goes back whole behind the plain\n    lead for a repeat - and only a paragraph, because a short fact said again is the answer.\n    ");
    mine = _flat(&[reply.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mine.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _carries_a_lead_in(&[mine.clone()]); if py_truthy(&__b) { __b } else { le(&call_builtin("len", &[mcall(&mine.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT()) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_said_earlier(&[mine.clone(), memory.clone()])))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reply".to_string(), mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_AGAIN(), &Py::Int(0)), "format", args))(&__args) };
    Py::None
}
pub fn _as_said(args: &[Py]) -> Py {
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
    let mut seen: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The content words of a line in the person's own spelling, in the order they said them.");
    seen = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&crate::gen::memory::topics::words(&[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[mcall(&word.clone(), "rstrip", &[py_str("s")])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&crate::gen::memory::topics::g_STOPWORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::memory::topics::g_ASK_ONLY(), &word.clone())) } } } }) {
    if py_truthy(&Py::Bool(!py_contains(&seen.clone(), &word.clone()))) {
    let _ = mcall(&seen.clone(), "append", &[word.clone()]);
    }
    }
    }
    return seen.clone();
    Py::None
}
pub fn _content_words_in_order(args: &[Py]) -> Py {
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
    let _ = py_str("The content words of a line as the matcher compares them, in the order they were said.\n\n    The words are cut to a stem (`rstrip(\"s\")`) so \"sandwiches\" and \"sandwich\" compare equal. A stem\n    is for comparing only: what goes back on the screen is the person's own spelling, because\n    \"packed lunch idea sandwiche\" is not a phrase anybody wrote.\n    ");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_as_said(&[text.clone()])) { let mut word: Py = __item.clone(); __out.push({ let __b = mcall(&word.clone(), "rstrip", &[py_str("s")]); if py_truthy(&__b) { __b } else { word.clone() } }); } py_list(__out) })() };
    Py::None
}
pub fn _the_run_they_said_both_times(args: &[Py]) -> Py {
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
    let mut earlier: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "earlier") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: earlier"))) };
    let mut best: Py = Py::None;
    let mut run: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stop: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The longest run of the line's own words that the earlier line used too, as they said it.\n\n    A run is words that stood next to each other in this line (\"packed lunch ideas\" in \"packed\n    lunch ideas no sandwiches\"). A joining word (\"or\", \"and\") is nobody's topic and neither starts\n    nor ends a run; a word the earlier line did not use ends one; and a rule-out ends the reading\n    altogether, so the thing the person said they did not want is never the thing the lead names.\n    One word is not a topic to say back, so a single word returns \"\" and the caller falls back to\n    the subject on the screen.\n    ");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    stop = mcall(&g__RULED_OUT(), "search", &[said.clone()]);
    if py_truthy(&stop.clone()) {
    said = slice_or_raise(&said.clone(), &Py::None, &mcall(&stop.clone(), "start", &[]), &Py::None);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    best = __unpk1[0].clone();
    run = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&crate::gen::memory::topics::words(&[said.clone()]));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&earlier.clone(), &{ let __b = mcall(&word.clone(), "rstrip", &[py_str("s")]); if py_truthy(&__b) { __b } else { word.clone() } }))) {
    if py_truthy(&Py::Bool(!py_contains(&run.clone(), &word.clone()))) {
    let _ = mcall(&run.clone(), "append", &[word.clone()]);
    }
    if py_truthy(&gt(&call_builtin("len", &[run.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    best = call_builtin("list", &[run.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    } else {
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[mcall(&word.clone(), "rstrip", &[py_str("s")])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&crate::gen::memory::topics::g_STOPWORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::memory::topics::g_ASK_ONLY(), &word.clone())) } } } }) {
    run = py_list(vec![]);
    }
    }
    }
    return if py_truthy(&gt(&call_builtin("len", &[best.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { mcall(&py_str(" "), "join", &[best.clone()]) } else { py_str("") };
    Py::None
}
pub fn _the_same_question_asked_again(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut mine: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The thing the line asks about again, when an earlier turn asked the same question.\n\n    Only a near-verbatim second ask counts (\"is heat or ice better\" / \"so is heat or ice better\n    then\"): a line that adds a detail the old answer never mentioned is a new question, and its\n    old answer coming back is a misreading rather than a return.\n    ");
    asked = call_builtin("set", &[_content_words_in_order(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&lt(&call_builtin("len", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = _flat(&[mcall(&turn.clone(), "get", &[py_str("bot")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&said.clone(), &mine.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "startswith", &[add(&mine.clone(), &py_str(" "))]))) } }) {
    continue;
    }
    earlier = call_builtin("set", &[_content_words_in_order(&[mcall(&turn.clone(), "get", &[py_str("user")])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&lt(&call_builtin("len", &[earlier.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    continue;
    }
    if py_truthy(&lt(&mul(&call_builtin("len", &[bitand(&asked.clone(), &earlier.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)), &mul(&call_builtin("len", &[bitor(&asked.clone(), &earlier.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)))) {
    continue;
    }
    return _the_run_they_said_both_times(&[text.clone(), earlier.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn _the_subject_on_the_screen(args: &[Py]) -> Py {
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
    let mut record: Py = Py::None;
    let mut returned: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The subject the conversation is on, in the person's words (\"my back\"), or \"\".\n\n    A return on the turn before (\"right, back to my back\") puts the topic back, but the line\n    after it (\"so is heat or ice better then\") is a follow-up and can make its own record\n    current again. The return is what the person is on, so it is read first.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    returned = mcall(&memory.clone(), "get", &[py_str("returned")]);
    if py_truthy(&call_builtin("isinstance", &[returned.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    turn = mcall(&returned.clone(), "get", &[py_str("turn")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { ge(&turn.clone(), &sub(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) } }) {
    subject = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&returned.clone(), "get", &[py_str("asked")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&subject.clone()) {
    return _as_your(&[subject.clone()]);
    }
    }
    }
    record = mcall(&memory.clone(), "get", &[py_str("topic")]);
    subject = if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("subject")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } else { py_str("") };
    return _as_your(&[subject.clone()]);
    Py::None
}
pub fn _as_your(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut subject: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let _ = py_str("The person's own words for a thing, said back to them: \"my back\" is \"your back\".");
    if py_truthy(&mcall(&mcall(&subject.clone(), "lower", &[]), "startswith", &[py_str("my ")])) {
    return add(&py_str("your "), &slice_or_raise(&subject.clone(), &Py::Int(3), &Py::None, &Py::None));
    }
    return subject.clone();
    Py::None
}
pub fn replied(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut respond: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "respond") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: respond"))) };
    let mut respond_once: Py = Py::None;
    let _ = py_str("A module's respond, with the earlier-turn check applied once at its exit.\n\n    \"Is there an earlier turn?\" was a question every reply path had to remember to ask, and the\n    paths that forgot printed the paragraph a second time: across the deck, 74 turns in 67 of 557\n    chats said something the same chat had already said, and 59 of them were a card served again by\n    `answers` on a route that had not been taught to ask. A route added tomorrow would have been\n    the 75th.\n\n    Wrapping `respond` makes the check the module's, not the route's: whatever a module returns,\n    the last thing that happens to it is `instead`, so a new reader, a new entry or a new branch\n    inside one is covered without knowing this file exists. Module code stays the same - decorate\n    it at the bottom of the module:\n\n        respond = again.replied(respond)\n\n    It frames, it never invents: `instead` is the only thing that decides what a repeat is, and a\n    reply the chat has not had comes back exactly as the module wrote it. That is the whole point\n    of doing it here - five turns into a collapse, \"should i move him\" got the ambulance card for\n    the fourth turn running, because `safety` had never been taught to ask; a fresh card on a new\n    line is untouched, because `instead` returns None for it.\n    ");
pub fn __nested_respond_once_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut respond: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut respond_once: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "text").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut reply: Py = Py::None;
    reply = py_call(&respond, &[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return reply.clone();
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = instead(&[reply.clone(), text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { reply.clone() } }]);
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
    return reply.clone();
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
    respond_once = py_func("respond_once", move |args: &[Py]| { let mut full = vec![respond.clone(), respond_once.clone()]; full.extend_from_slice(args); __nested_respond_once_0(&full) });
    let _ = py_setattr(&respond_once.clone(), "__name__", call_builtin("getattr", &[respond.clone(), py_str("__name__"), py_str("respond_once")]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setattr(&respond_once.clone(), "__doc__", gattr(&respond.clone(), "__doc__"));
    let _ = py_setattr(&respond_once.clone(), "checks_earlier_turns", Py::Bool(true));
    return respond_once.clone();
    Py::None
}
pub fn instead(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut body: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut times: Py = Py::None;
    let _ = py_str("The reply framed as the repeat it is, or None when it is new to this conversation.\n\n    Read by the module that would otherwise serve the paragraph a second time, as the last thing\n    it does: everything above has already said whether this turn is a repeat in the bot's own\n    terms, and this is the floor under it - whatever route the answer came by, it is not printed\n    twice in the same words.\n    ");
    mine = _flat(&[reply.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mine.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&_carries_a_lead_in(&[mine.clone()])) {
    return Py::None;
    }
    said_back = mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = mcall(&g_SAID_BACK(), "match", &[said_back.clone()]); if !py_truthy(&__b) { __b } else { le(&call_builtin("len", &[mcall(&mine.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT()) } }) {
    return Py::None;
    }
    times = _times_on_the_screen(&[mine.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&times.clone()))) {
    times = _said_recently(&[mine.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&times.clone())); if !py_truthy(&__b) { __b } else { _said_and_more(&[mine.clone(), memory.clone()]) } }) {
    times = Py::Int(1);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&times.clone())); if !py_truthy(&__b) { __b } else { _said_earlier(&[mine.clone(), memory.clone()]) } }) {
    thing = _returned_thing(&[memory.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    asked = _the_same_question_asked_again(&[mine.clone(), text.clone(), memory.clone()]);
    if py_truthy(&asked.clone()) {
    detail = { let __b = _the_subject_on_the_screen(&[memory.clone()]); if py_truthy(&__b) { __b } else { asked.clone() } };
    frame = geti(&g_RETURN(), &modulo(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_RETURN()]).unwrap_or_else(|e| exc::raise(e))));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("detail".to_string(), _as_a_name(&[detail.clone()])), ("reply".to_string(), _the_first_sentence(&[reply.clone(), memory.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "format", args))(&__args) };
    }
    return Py::None;
    }
    said = _said_earlier_text(&[mine.clone(), memory.clone()]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_adds_nothing_the_answer_did_not_say(&[thing.clone(), text.clone(), said.clone(), memory.clone()]))) } }) {
    return Py::None;
    }
    thing = _as_a_name(&[thing.clone()]);
    frame = geti(&g_RETURN(), &modulo(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_RETURN()]).unwrap_or_else(|e| exc::raise(e))));
    body = _without_the_thing_in_front(&[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), thing.clone()]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("detail".to_string(), thing.clone()), ("reply".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&times.clone()))) {
    return Py::None;
    }
    return _framed_as_a_repeat(&[reply.clone(), mine.clone(), memory.clone(), times.clone(), text.clone()]);
    Py::None
}
pub fn a_thing_of_the_answer(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut mine: Py = Py::None;
    let _ = py_str("The reply framed as the repeat it is, when the line names a thing the reply is about.\n\n    Read by the module that has already found the card for the thing the line names\n    (`bot/memory/detail.py`): \"my wheelie bin smells even after i wash it\" / \"the food one is the\n    worst\" / \"its a wheelie bin\" found the bin's own card, which is the paragraph already on the\n    screen, and that route hands the card back without asking `instead` - the thing named has no\n    lead of its own, so the paragraph went out word for word a second time.\n\n    The paragraph is still the right one, so it is framed rather than replaced: the answer is the\n    same and the line is about it. What keeps this honest is `_names_something_the_answer_says` -\n    a line about something else shares none of the answer's words, and the paragraph coming back\n    on it stays the misreading it is, with no lead claiming it still applies. An answer that\n    holds the step for the fault coming back is left to `bot/memory/returned.py`, which owns\n    \"its dropped again\".\n    ");
    mine = _flat(&[reply.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mine.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&_carries_a_lead_in(&[mine.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_said_earlier(&[mine.clone(), memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_something_the_answer_says(&[text.clone(), mine.clone()])))) {
    return Py::None;
    }
    let mut returned: Py = crate::gen::memory::returned::module_obj();
    if py_truthy(&mcall(&returned.clone(), "the_next_step", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    return _framed_as_a_repeat(&[reply.clone(), mine.clone(), memory.clone(), Py::Int(1), text.clone()]);
    Py::None
}
pub fn _another_stand_down(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut o: Py = Py::None;
    let mut others: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A stand-down said a second time, in the words of another one, or None.\n\n    The hedge is the bot saying it cannot answer from here, and there are a few ways it says so.\n    Saying the same one again with \"Same answer, and it covers this\" in front of it claims the\n    thing has been answered, which it has not; saying it in another of the same words is the same\n    honesty and does not read as a machine. Only a whole reply that is one of the hedges is\n    swapped: a card that opens with a hedge has the answer in it, and it is the card.\n    ");
    let mut fallback: Py = crate::gen::compose::fallback::module_obj();
    mine = _flat(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&gattr(&fallback.clone(), "UNKNOWN"));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&_flat(&[line.clone()]), &mine.clone()))) {
    continue;
    }
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&fallback.clone(), "UNKNOWN")) { let mut o: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&_flat(&[o.clone()]), &mine.clone()))) {  __out.push(o.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&others.clone()))) {
    return Py::None;
    }
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    return geti(&others.clone(), &modulo(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[others.clone()]).unwrap_or_else(|e| exc::raise(e))));
    }
    return Py::None;
    Py::None
}
pub fn _framed_as_a_repeat(args: &[Py]) -> Py {
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
    let mut mine: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mine") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mine"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut times: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "times") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: times"))) };
    let mut text: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut lead: Py = Py::None;
    let mut said_twice: Py = Py::None;
    let mut short: Py = Py::None;
    let mut underneath: Py = Py::None;
    let _ = py_str("The reply under a lead-in saying it is the same answer, cut to the short of it.\n\n    `times` is how many times the chat has had it, which picks the lead-in. A few words is\n    already the short of itself, so it gets the short of the lead-ins instead.\n\n    `text` is the line that asked, which both callers have and this reader needs: it decides\n    whether the line asks for a part of the answer (see below). It was read here without being\n    passed in, so every framed repeat of a paragraph raised NameError, the runtime swallowed it\n    as it is meant to, and the whole knowledge base went silent behind a greeting: \"how do i cook\n    rice\" / \"sorry how do i cook rice\" was told \"I'm not sure about that one, sorry\".\n    ");
    if py_truthy(&_says_it_came_back(&[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&le(&call_builtin("len", &[mcall(&mine.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT())) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reply".to_string(), mcall(&reply.clone(), "strip", &[]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_SHORT_AGAIN(), &sub(&call_builtin("min", &[times.clone(), call_builtin("len", &[g_SHORT_AGAIN()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))), "format", args))(&__args) };
    }
    if py_truthy(&_a_report(&[text.clone()])) {
    return Py::None;
    }
    let mut fallback: Py = crate::gen::compose::fallback::module_obj();
    if py_truthy(&{ let __b = { let __b = mcall(&gattr(&fallback.clone(), "WORDING_ASKS"), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&gattr(&fallback.clone(), "WORDING_SOMEONE"), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&gattr(&fallback.clone(), "WORDING_SAYS"), "search", &[text.clone()]) } } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_has_a_script(&[reply.clone()]))) } }) {
    return Py::None;
    }
    let mut repeat: Py = crate::gen::memory::repeat::module_obj();
    if py_truthy(&mcall(&repeat.clone(), "asks_for_the_move", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&_a_figure_asked_that_the_answer_does_not_hold(&[text.clone(), reply.clone()])) {
    return Py::None;
    }
    underneath = the_same_thing_underneath(&[text.clone(), _short(&[reply.clone(), memory.clone()]), times.clone(), memory.clone()]);
    if py_truthy(&underneath.clone()) {
    return underneath.clone();
    }
    short = _short(&[reply.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&_flat(&[short.clone()]), &mine.clone())); if !py_truthy(&__b) { __b } else { _times_on_the_screen(&[_flat(&[short.clone()]), memory.clone()]) } }) {
    short = reply.clone();
    }
    if py_truthy(&{ let __b = { let __b = _asks_for_a_detail(&[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ANOTHER_ONE(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_asks_for_a_detail(&[_the_line_that_got_it(&[mine.clone(), memory.clone()])]))) } }) {
    short = reply.clone();
    }
    said_twice = _another_stand_down(&[reply.clone(), memory.clone()]);
    if py_truthy(&said_twice.clone()) {
    return said_twice.clone();
    }
    lead = geti(&g_AGAIN(), &sub(&call_builtin("min", &[times.clone(), call_builtin("len", &[g_AGAIN()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&short.clone(), &py_str(":")))) {
    lead = mcall(&lead.clone(), "replace", &[py_str(": {reply}"), py_str(" — {reply}")]);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reply".to_string(), short.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.again".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGAIN" => Some(g_AGAIN()),
            "AGAIN_HELP" => Some(g_AGAIN_HELP()),
            "ANOTHER_ONE" => Some(g_ANOTHER_ONE()),
            "ANY_SEPARATOR" => Some(g_ANY_SEPARATOR()),
            "AN_AMOUNT" => Some(g_AN_AMOUNT()),
            "AN_ASK" => Some(g_AN_ASK()),
            "A_DETAIL" => Some(g_A_DETAIL()),
            "A_MONEY_ASK" => Some(g_A_MONEY_ASK()),
            "A_PHRASE" => Some(g_A_PHRASE()),
            "A_REPORT" => Some(g_A_REPORT()),
            "A_SCRIPT" => Some(g_A_SCRIPT()),
            "A_STILL_GOING_ON" => Some(g_A_STILL_GOING_ON()),
            "BEING" => Some(g_BEING()),
            "CANNOT" => Some(g_CANNOT()),
            "CLAUSE_ONLY" => Some(g_CLAUSE_ONLY()),
            "COLON_LEADS" => Some(g_COLON_LEADS()),
            "DETAIL_AGAIN" => Some(g_DETAIL_AGAIN()),
            "FACT_RECALL" => Some(g_FACT_RECALL()),
            "FEELING" => Some(g_FEELING()),
            "FOR_THE_PERSON" => Some(g_FOR_THE_PERSON()),
            "FRAMES" => Some(g_FRAMES()),
            "FRAME_WORDS" => Some(g_FRAME_WORDS()),
            "LEADS" => Some(g_LEADS()),
            "LEAD_WORDS" => Some(g_LEAD_WORDS()),
            "LIKE_I_SAID" => Some(g_LIKE_I_SAID()),
            "MONEY_SAID" => Some(g_MONEY_SAID()),
            "NOTHING" => Some(g_NOTHING()),
            "NOTHING_MORE" => Some(g_NOTHING_MORE()),
            "ONE_WORD_LEAD" => Some(g_ONE_WORD_LEAD()),
            "OPENING_FILLER" => Some(g_OPENING_FILLER()),
            "PATTERNS" => Some(g_PATTERNS()),
            "RETURN" => Some(g_RETURN()),
            "SAID_BACK" => Some(g_SAID_BACK()),
            "SAME_ADVICE" => Some(g_SAME_ADVICE()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SEPARATOR" => Some(g_SEPARATOR()),
            "SHORT" => Some(g_SHORT()),
            "SHORT_AGAIN" => Some(g_SHORT_AGAIN()),
            "SLOT" => Some(g_SLOT()),
            "UNDERNEATH_CLAUSE" => Some(g_UNDERNEATH_CLAUSE()),
            "UNDERNEATH_SAID" => Some(g_UNDERNEATH_SAID()),
            "_NOT_THE_END" => Some(g__NOT_THE_END()),
            "_RULED_OUT" => Some(g__RULED_OUT()),
            "_a_card_behind" => Some(py_func("_a_card_behind", |args: &[Py]| _a_card_behind(args))),
            "_a_figure_asked_that_the_answer_does_not_hold" => Some(py_func("_a_figure_asked_that_the_answer_does_not_hold", |args: &[Py]| _a_figure_asked_that_the_answer_does_not_hold(args))),
            "_a_phrase_of" => Some(py_func("_a_phrase_of", |args: &[Py]| _a_phrase_of(args))),
            "_a_report" => Some(py_func("_a_report", |args: &[Py]| _a_report(args))),
            "_adds_nothing_the_answer_did_not_say" => Some(py_func("_adds_nothing_the_answer_did_not_say", |args: &[Py]| _adds_nothing_the_answer_did_not_say(args))),
            "_another_stand_down" => Some(py_func("_another_stand_down", |args: &[Py]| _another_stand_down(args))),
            "_as_a_name" => Some(py_func("_as_a_name", |args: &[Py]| _as_a_name(args))),
            "_as_said" => Some(py_func("_as_said", |args: &[Py]| _as_said(args))),
            "_as_your" => Some(py_func("_as_your", |args: &[Py]| _as_your(args))),
            "_asks_for_a_detail" => Some(py_func("_asks_for_a_detail", |args: &[Py]| _asks_for_a_detail(args))),
            "_carries_a_lead_in" => Some(py_func("_carries_a_lead_in", |args: &[Py]| _carries_a_lead_in(args))),
            "_content_words_in_order" => Some(py_func("_content_words_in_order", |args: &[Py]| _content_words_in_order(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_for_the_person" => Some(py_func("_for_the_person", |args: &[Py]| _for_the_person(args))),
            "_framed_as_a_repeat" => Some(py_func("_framed_as_a_repeat", |args: &[Py]| _framed_as_a_repeat(args))),
            "_has_a_script" => Some(py_func("_has_a_script", |args: &[Py]| _has_a_script(args))),
            "_lead" => Some(py_func("_lead", |args: &[Py]| _lead(args))),
            "_led_an_earlier_answer" => Some(py_func("_led_an_earlier_answer", |args: &[Py]| _led_an_earlier_answer(args))),
            "_names_something_the_answer_says" => Some(py_func("_names_something_the_answer_says", |args: &[Py]| _names_something_the_answer_says(args))),
            "_places" => Some(py_func("_places", |args: &[Py]| _places(args))),
            "_returned_thing" => Some(py_func("_returned_thing", |args: &[Py]| _returned_thing(args))),
            "_said_and_more" => Some(py_func("_said_and_more", |args: &[Py]| _said_and_more(args))),
            "_said_earlier" => Some(py_func("_said_earlier", |args: &[Py]| _said_earlier(args))),
            "_said_earlier_text" => Some(py_func("_said_earlier_text", |args: &[Py]| _said_earlier_text(args))),
            "_said_recently" => Some(py_func("_said_recently", |args: &[Py]| _said_recently(args))),
            "_says_i" => Some(py_func("_says_i", |args: &[Py]| _says_i(args))),
            "_says_it_came_back" => Some(py_func("_says_it_came_back", |args: &[Py]| _says_it_came_back(args))),
            "_short" => Some(py_func("_short", |args: &[Py]| _short(args))),
            "_the_answer_in_hand" => Some(py_func("_the_answer_in_hand", |args: &[Py]| _the_answer_in_hand(args))),
            "_the_first_sentence" => Some(py_func("_the_first_sentence", |args: &[Py]| _the_first_sentence(args))),
            "_the_line_asks" => Some(py_func("_the_line_asks", |args: &[Py]| _the_line_asks(args))),
            "_the_line_that_got_it" => Some(py_func("_the_line_that_got_it", |args: &[Py]| _the_line_that_got_it(args))),
            "_the_run_they_said_both_times" => Some(py_func("_the_run_they_said_both_times", |args: &[Py]| _the_run_they_said_both_times(args))),
            "_the_same_question_asked_again" => Some(py_func("_the_same_question_asked_again", |args: &[Py]| _the_same_question_asked_again(args))),
            "_the_subject_on_the_screen" => Some(py_func("_the_subject_on_the_screen", |args: &[Py]| _the_subject_on_the_screen(args))),
            "_times_on_the_screen" => Some(py_func("_times_on_the_screen", |args: &[Py]| _times_on_the_screen(args))),
            "_without_the_thing_in_front" => Some(py_func("_without_the_thing_in_front", |args: &[Py]| _without_the_thing_in_front(args))),
            "a_thing_of_the_answer" => Some(py_func("a_thing_of_the_answer", |args: &[Py]| a_thing_of_the_answer(args))),
            "already_had" => Some(py_func("already_had", |args: &[Py]| already_had(args))),
            "as_a_phrase" => Some(py_func("as_a_phrase", |args: &[Py]| as_a_phrase(args))),
            "carries_a_lead_in" => Some(py_func("carries_a_lead_in", |args: &[Py]| carries_a_lead_in(args))),
            "instead" => Some(py_func("instead", |args: &[Py]| instead(args))),
            "replied" => Some(py_func("replied", |args: &[Py]| replied(args))),
            "the_cause_said_again" => Some(py_func("the_cause_said_again", |args: &[Py]| the_cause_said_again(args))),
            "the_name_the_answer_says" => Some(py_func("the_name_the_answer_says", |args: &[Py]| the_name_the_answer_says(args))),
            "the_same_thing_underneath" => Some(py_func("the_same_thing_underneath", |args: &[Py]| the_same_thing_underneath(args))),
            "unframed" => Some(py_func("unframed", |args: &[Py]| unframed(args))),
            _ => None,
        }),
        set: None,
    }))
}
