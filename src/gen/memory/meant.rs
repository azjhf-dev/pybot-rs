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


pub static ABOUT_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANSWERED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKING_ABOUT_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_SOMETHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDOLENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDOLENCE_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEFINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOLLOW_UP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOOK_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOOK_BACK_FAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_WRITTEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_LETTERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_TO_REPEAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_ANSWER_OF_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSONAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSONAL_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUT_IT_PLAINER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECEIPT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPEAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPEAT_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_TWICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TELLS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_YOU_ARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PERSONAL_HEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_MINE() -> Py { ensure_init(); ABOUT_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_MINE(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_MINE.write() { *w = v; } }
pub fn g_ANSWERED() -> Py { ensure_init(); ANSWERED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERED(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERED.write() { *w = v; } }
pub fn g_ASKING_ABOUT_DOING() -> Py { ensure_init(); ASKING_ABOUT_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING_ABOUT_DOING(v: Py) { ensure_init(); if let Ok(mut w) = ASKING_ABOUT_DOING.write() { *w = v; } }
pub fn g_ASKS_SOMETHING() -> Py { ensure_init(); ASKS_SOMETHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_SOMETHING(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_SOMETHING.write() { *w = v; } }
pub fn g_CLAUSE() -> Py { ensure_init(); CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = CLAUSE.write() { *w = v; } }
pub fn g_CONDOLENCE() -> Py { ensure_init(); CONDOLENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDOLENCE(v: Py) { ensure_init(); if let Ok(mut w) = CONDOLENCE.write() { *w = v; } }
pub fn g_CONDOLENCE_SENTENCE() -> Py { ensure_init(); CONDOLENCE_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDOLENCE_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = CONDOLENCE_SENTENCE.write() { *w = v; } }
pub fn g_DEFINES() -> Py { ensure_init(); DEFINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEFINES(v: Py) { ensure_init(); if let Ok(mut w) = DEFINES.write() { *w = v; } }
pub fn g_FOLLOW_UP_WORDS() -> Py { ensure_init(); FOLLOW_UP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FOLLOW_UP_WORDS.write() { *w = v; } }
pub fn g_FOR_MINE() -> Py { ensure_init(); FOR_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_MINE(v: Py) { ensure_init(); if let Ok(mut w) = FOR_MINE.write() { *w = v; } }
pub fn g_LEADS() -> Py { ensure_init(); LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = LEADS.write() { *w = v; } }
pub fn g_LEAST() -> Py { ensure_init(); LEAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAST(v: Py) { ensure_init(); if let Ok(mut w) = LEAST.write() { *w = v; } }
pub fn g_LOOK_BACK() -> Py { ensure_init(); LOOK_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); if let Ok(mut w) = LOOK_BACK.write() { *w = v; } }
pub fn g_LOOK_BACK_FAR() -> Py { ensure_init(); LOOK_BACK_FAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK_FAR(v: Py) { ensure_init(); if let Ok(mut w) = LOOK_BACK_FAR.write() { *w = v; } }
pub fn g_MEAN() -> Py { ensure_init(); MEAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEAN(v: Py) { ensure_init(); if let Ok(mut w) = MEAN.write() { *w = v; } }
pub fn g_MOST() -> Py { ensure_init(); MOST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOST(v: Py) { ensure_init(); if let Ok(mut w) = MOST.write() { *w = v; } }
pub fn g_NOTHING_WRITTEN() -> Py { ensure_init(); NOTHING_WRITTEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_WRITTEN(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_WRITTEN.write() { *w = v; } }
pub fn g_NOT_LETTERS() -> Py { ensure_init(); NOT_LETTERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_LETTERS(v: Py) { ensure_init(); if let Ok(mut w) = NOT_LETTERS.write() { *w = v; } }
pub fn g_NOT_TO_REPEAT() -> Py { ensure_init(); NOT_TO_REPEAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_TO_REPEAT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_TO_REPEAT.write() { *w = v; } }
pub fn g_NO_ANSWER_OF_MINE() -> Py { ensure_init(); NO_ANSWER_OF_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_ANSWER_OF_MINE(v: Py) { ensure_init(); if let Ok(mut w) = NO_ANSWER_OF_MINE.write() { *w = v; } }
pub fn g_OPENERS() -> Py { ensure_init(); OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = OPENERS.write() { *w = v; } }
pub fn g_PERSONAL() -> Py { ensure_init(); PERSONAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSONAL(v: Py) { ensure_init(); if let Ok(mut w) = PERSONAL.write() { *w = v; } }
pub fn g_PERSONAL_LEADS() -> Py { ensure_init(); PERSONAL_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSONAL_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = PERSONAL_LEADS.write() { *w = v; } }
pub fn g_PUT_IT_PLAINER() -> Py { ensure_init(); PUT_IT_PLAINER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUT_IT_PLAINER(v: Py) { ensure_init(); if let Ok(mut w) = PUT_IT_PLAINER.write() { *w = v; } }
pub fn g_RECEIPT() -> Py { ensure_init(); RECEIPT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECEIPT(v: Py) { ensure_init(); if let Ok(mut w) = RECEIPT.write() { *w = v; } }
pub fn g_REPEAT() -> Py { ensure_init(); REPEAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPEAT(v: Py) { ensure_init(); if let Ok(mut w) = REPEAT.write() { *w = v; } }
pub fn g_REPEAT_LEADS() -> Py { ensure_init(); REPEAT_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPEAT_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = REPEAT_LEADS.write() { *w = v; } }
pub fn g_SAID_TWICE() -> Py { ensure_init(); SAID_TWICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_TWICE(v: Py) { ensure_init(); if let Ok(mut w) = SAID_TWICE.write() { *w = v; } }
pub fn g_SENTENCE() -> Py { ensure_init(); SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = SENTENCE.write() { *w = v; } }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); SOMEONE_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSE.write() { *w = v; } }
pub fn g_SOMEONE_LEADS() -> Py { ensure_init(); SOMEONE_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_LEADS.write() { *w = v; } }
pub fn g_TELLS() -> Py { ensure_init(); TELLS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TELLS(v: Py) { ensure_init(); if let Ok(mut w) = TELLS.write() { *w = v; } }
pub fn g_WHAT_YOU_ARE() -> Py { ensure_init(); WHAT_YOU_ARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_YOU_ARE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_YOU_ARE.write() { *w = v; } }
pub fn g__HERE() -> Py { ensure_init(); _HERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HERE(v: Py) { ensure_init(); if let Ok(mut w) = _HERE.write() { *w = v; } }
pub fn g__PERSONAL_HEADS() -> Py { ensure_init(); _PERSONAL_HEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PERSONAL_HEADS(v: Py) { ensure_init(); if let Ok(mut w) = _PERSONAL_HEADS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("They ask what the bot meant, and the answer is in the line it just gave.\n\n\"what do you mean by that\" is one of the commonest follow-ups in the language, and it was\nanswered with a search hedge (\"That's beyond what I know well, sorry - searching for it will turn\nup better sources\"), as if the line the bot had just written had nothing to do with the question.\nIt has everything to do with it: the bot meant the first thing it said, or - when its own line\nended on a question - the question it asked.\n\nSo: a bare \"what do you mean\", pointed at the sentence the bot meant, with a lead-in that hands\nthe bit over (\"Just this: ...\").\n\nThe rule is narrow on purpose:\n* the line is only that question and nothing else, so \"what do you mean about the deposit\" - a\n  question with a thing of its own - never comes here;\n* there has to be a bot turn in the last turn or two to point at, so an opening \"what do you mean\n  by that\" (nothing has been meant yet) keeps the honest hedge;\n* a hedge is never cited back at itself: where the bot had no answer, the question still gets none;\n* one sentence of the answer, cut at its first clause boundary, never the paragraph again, and\n  never the line this rule already gave.\n");
    s_OPENERS(py_str("^(?:(?:and|so|but|ok|okay|well|right|sorry|wait|basically|hmm|um|er|uh|ah|oh|yeah|then|no|nope|again|hold\\s+on|alright)\\b[\\s,]+)*(?:(?:i\\s*(?:'m|m)\\s*)?not\\s+sure(?:\\s+(?:what|that|if))?[\\s,]+|i\\s+don(?:'t|t)\\s+(?:get|understand|follow)\\b[\\s,]*(?:it|that|this)?[\\s,]*|(?:i\\s+)?did(?:n'?t|nt)\\s+(?:catch|follow)(?:\\s+(?:that|you))?[\\s,]+|not\\s+following[\\s,]+|you\\s+lost\\s+me[\\s,]+)*"));
    s_MEAN(stdlib_call("re", "compile", &[add(&add(&py_str("^\\s*"), &g_OPENERS()), &py_str("(?:(?:what|wot|wat|wut|whut)\\s+(?:(?:do|did|d)\\s+)?(?:you|u|ya)\\s+mean(?:\\s+(?:by\\s+(?:that|it|this)|exactly|though|then))?|(?:whatcha|watcha|whaddya|whadya|whatdya)\\s+mean(?:\\s+by\\s+(?:that|it|this))?|how\\s+(?:do|did)\\s+(?:you|u|ya)\\s+mean(?:\\s+(?:by\\s+(?:that|it|this)|exactly))?|what(?:'s|s)?\\s+(?:(?:does|do|is)\\s+)?(?:that|it|this)\\s+(?:(?:actually|really|exactly|precisely|even|then)\\s+)?(?:supposed\\s+to\\s+)?mean(?:\\s+(?:then|though|exactly|precisely))?(?:\\s+by\\s+(?:that|it|this))?(?:\\s+for\\s+(?:me|us|him|her|them|my\\s+case|my\\s+situation|(?:my|our)\\s+[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+)?))?(?:\\s+about\\s+(?:my|our|the|this|that)\\s+[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,2})?(?:\\s+in\\s+(?:practice|reality|real\\s+terms))?(?:\\s+(?:then|though|exactly|precisely))?|meaning\\s+what|explain\\s+(?:what\\s+)?(?:you|u)\\s+mean|wdym|mean\\s+what\\s+exactly|(?:what(?:'s|s)?|what\\s+is)\\s+(?:the\\s+)?meaning\\s+of\\s+(?:that|it|this)|(?:the\\s+)?meaning\\s+of\\s+(?:that|it|this)|and\\s+that\\s+means\\?|what\\s+is\\s+meant\\s+by\\s+(?:that|it|this)|whats?\\s+the\\s+meaning|(?:that|it|this)\\s+means\\s+what(?:\\s+(?:exactly|then|though|precisely))?|what\\s+(?:you|u|ya)\\s+mean|what\\s+(?:that|it|this)\\s+means?(?:\\s+(?:exactly|then|though|precisely))?)(?:,\\s*(?:then|though|exactly|precisely|really|actually)\\b)?\\s*[?.!]*\\s*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_SOMETHING(stdlib_call("re", "compile", &[py_str(",\\s*(?:is|are|was|were|am|does|do|did|will|would|can|could|should|shall|has|have)\\s+[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,6}\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPEAT(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|but\\s+|ok\\s+|okay\\s+|well\\s+|right\\s+|sorry\\s+|er+\\s+|um+\\s+)*(?:(?:can|could|would)\\s+you\\s+(?:please\\s+)?(?:say|repeat|explain|rephrase|clarify)(?:\\s+(?:that|it))?(?:\\s+again)?(?:\\s+please)?|(?:say|repeat)\\s+(?:that|it)\\s+again|explain(?:\\s+that)?|explain\\s+again|meaning|(?:i\\s+)?(?:(?:still|really|just|honestly|actually|literally)\\s+)?(?:do\\s*n[o']?t|dont|does\\s*n[o']?t|did\\s*n[o']?t|didnt)\\s+(?:(?:really|still|quite|honestly|fully)\\s+|at\\s+all\\s+)?(?:understand|follow|get\\s+it|catch\\s+that|(?:get|see|follow)\\s+what\\s+you(?:'re|\\s+are)?\\s+(?:saying|on\\s+about|mean))(?:\\s+(?:it|that|this|at\\s+all|one\\s+bit|any\\s+of\\s+it|what\\s+you\\s+mean|a\\s+word\\s+of\\s+it|the\\s+last\\s+bit))?|(?:i'?m|im)\\s+not\\s+(?:following|with\\s+you)|(?:i'?m|im)\\s+(?:still\\s+|a\\s+bit\\s+|totally\\s+|completely\\s+)?(?:confused|lost)|(?:you\\s+)?(?:have\\s+)?lost\\s+me(?:\\s+(?:there|completely|entirely))?|(?:that|this)\\s+is\\s+confusing|(?:that'?s|thats)\\s+confusing|(?:what|pardon|eh|huh)\\?*|sorry\\s*,?\\s*(?:what|pardon|eh|huh)?\\s*\\?+|sorry\\s*,?\\s*(?:what|pardon|eh|huh)\\s*[?.!]*|(?:say|explain|put)\\s+it\\s+in\\s+(?:plain\\s+)?english(?:\\s+please)?|what(?:'s|s|\\s+is)?\\s+(?:that|this|it)\\s+in\\s+(?:plain\\s+)?english|in\\s+(?:plain\\s+)?english(?:\\s+please)?|english\\s+please|come\\s+again|say\\s+what)\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPEAT_LEADS(py_tuple(vec![py_str("I said: {said}"), py_str("Same again: {said}"), py_str("Let me put it plainer: {said}")]));
    s_LEADS(py_tuple(vec![py_str("I mean this: {said}"), py_str("Just this: {said}"), py_str("What I meant: {said}")]));
    s_PERSONAL(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:me|us|him|her|them|my\\s+case|my\\s+situation|(?:my|our)\\s+[\\w'\\u2019-]+)\\b|\\bin\\s+(?:practice|reality|real\\s+terms)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_MINE(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:my|our)\\s+(?!case\\b|situation\\b|own\\b|end\\b|side\\b|part\\b)([\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+)?)\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSONAL_LEADS(py_tuple(vec![py_str("For you, that means: {said}"), py_str("In your case: {said}")]));
    s_FOLLOW_UP_WORDS(call_builtin("frozenset", &[mcall(&py_str("what whats which that this it thats those them they\n    mean means meaning meant you u ya your i im sorry ok okay so then and but well just really\n    actually exactly precisely simply plain plainer please can could would will do does did is\n    are was were there here have has had\n    put putting say saying said tell telling explained explaining explain\n    break down again another other way words wording sense terms term practice reality real life\n    english jargon\n    technical simpler simple clearly five like of in on about to for me us him her huh pardon\n    people normal someone folk most"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_IT_PLAINER(call_builtin("frozenset", &[mcall(&py_str("mean means meaning meant explain explained explaining\n    tell telling another sense terms term practice reality english jargon technical plain words\n    plainer simpler simple simply clearly break"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_LETTERS(stdlib_call("re", "compile", &[py_str("[^a-z'\\s]")]).unwrap_or_else(|e| exc::raise(e)));
    s_TELLS(stdlib_call("re", "compile", &[py_str("\\b(?:that|this|it|those|these)\\b[^.?!]{0,20}\\btell"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_YOU_ARE(stdlib_call("re", "compile", &[py_str("\\b(?:what|who)\\s+(?:are|r)\\s+(?:you|u|ya)\\b|\\bwhat do you do\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_ABOUT_DOING(call_builtin("frozenset", &[mcall(&py_str("do does did done work works took take takes costing cost\n    last lasts hurt hurts help helps happen happens matter matters"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_MINE(stdlib_call("re", "compile", &[py_str("\\babout\\s+(?:my|our|the|this|that)\\s+([\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,2})\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(him|her|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_LEADS(py_str("For {who}, that means: {said}"));
    s__PERSONAL_HEADS(add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PERSONAL_LEADS()) { let mut lead: Py = __item.clone(); __out.push(geti(&mcall(&lead.clone(), "split", &[py_str("{")]), &Py::Int(0))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("For him, that means: "), py_str("For her, that means: "), py_str("For them, that means: ")])));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_RECEIPT(stdlib_call("re", "compile", &[py_str("^\\s*(?:sure\\s*[-\\u2014]\\s*)?(?:noted\\b|got it\\b|so far i know\\b|thanks for telling me\\b|i hear you\\b|i'?m listening\\b|say more\\b|tell me more\\b|that sounds\\b|i'?ll keep that\\b|good to know\\b|nothing (?:to do|i can add)\\b|i'?m here if\\b|(?:i\\s+)?haven'?t put anything about it into words|nothing of mine to go back over|is there anything you'?d like\\b|you'?re (?:noted|on the list)\\b)|[\\u2014-]\\s*(?:noted|good to know|got it|i'?ll remember that|i'?ve got that)\\b[.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_WRITTEN(py_tuple(vec![py_str("I haven't put anything about it into words yet - give me a bit more and I'll start from there."), py_str("Nothing of mine to go back over yet. Tell me a little more and I'll take it from the top."), py_str("I hadn't answered anything yet, so there's nothing of mine to put straighter - tell me what you were after and I'll take it on."), py_str("Nothing of mine to explain yet: I said how that landed and stopped there. Say what you were asking and I'll have a proper go at it."), py_str("That was me saying how it lands, not an answer. Ask me again and I'll take it on.")]));
    s_NO_ANSWER_OF_MINE(py_tuple(vec![py_str("I didn't have an answer there, so there's nothing of mine to explain - it just didn't know. Say a bit more about it and I'll have a proper go."), py_str("Nothing of mine to go over: I didn't know that one. Tell me more and I'll try again.")]));
    s_SAID_TWICE(py_tuple(vec![py_str("I've put that twice now, so the words in it are probably the problem - tell me which bit and I'll unpack that one."), py_str("That is the second time I've said it that way. Which word in it is the trouble?")]));
    s_CLAUSE(stdlib_call("re", "compile", &[py_str(":\\s+|\\s+-\\s+|;\\s+|,\\s+so\\s+|,\\s+and\\s+|,\\s+but\\s+|,\\s+which\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST(Py::Int(150));
    s_LEAST(Py::Int(30));
    s_ANSWERED(Py::Int(20));
    s_LOOK_BACK(Py::Int(3));
    s_LOOK_BACK_FAR(Py::Int(7));
    s_DEFINES(stdlib_call("re", "compile", &[py_str("\\bmeans?\\b|\\bis when\\b|\\bis what\\b|\\bis where\\b|\\bin other words\\b|\\brefers to\\b|\\bstands for\\b|\\bwhich means\\b|\\bthat is what\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_TO_REPEAT(call_builtin("frozenset", &[py_tuple(vec![py_str("safety"), py_str("loss"), py_str("grief")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDOLENCE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:oh|ah)\\s*,?\\s+|(?:i'?m|i\\s+am)\\s+(?:so\\s+|really\\s+|very\\s+)?)*sorry\\b|^\\s*(?:oh\\s+no|ugh|that'?s\\s+(?:rough|hard|awful|a\\s+lot\\s+to\\s+carry)|that\\s+is\\s+rough|what\\s+a\\s+shame|how\\s+awful|that\\s+sounds\\s+(?:rough|tough|hard|awful|horrible))\\s*[,:.\\u2014-]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HERE(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&add(&add(&g_LEADS(), &g_REPEAT_LEADS()), &g_PERSONAL_LEADS())) { let mut lead: Py = __item.clone(); __out.push(geti(&mcall(&lead.clone(), "split", &[py_str("{")]), &Py::Int(0))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDOLENCE_SENTENCE(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:oh|ah)\\s*,?\\s+|(?:i'?m|i\\s+am)\\s+(?:so\\s+|really\\s+|very\\s+)?)*sorry[^.!?]*[.!?]|^\\s*(?:oh\\s+no|ugh|that'?s\\s+(?:rough|hard|awful|a\\s+lot\\s+to\\s+carry)|that\\s+is\\s+rough|what\\s+a\\s+shame|how\\s+awful|that\\s+sounds\\s+(?:rough|tough|hard|awful|horrible))[^.!?]*[.!?]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _asked_plainly(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line is a pointer back at the last line or a word for plainer words.\n\n    Read by the words rather than by the shape: no word in the line can name anything, and at\n    least one of them has to ask for the thing in other words. See `FOLLOW_UP_WORDS`.\n    ");
    said = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "strip", &[py_str(" ?.!,")]);
    if py_truthy(&mcall(&g_NOT_LETTERS(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&le(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_FOLLOW_UP_WORDS())))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PUT_IT_PLAINER())))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&words.clone(), &py_str("tell"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("telling"))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TELLS(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_WHAT_YOU_ARE(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_ASKING_ABOUT_DOING(), &geti(&words.clone(), &neg(&Py::Int(1)))))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _personal(args: &[Py]) -> Py {
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
    let _ = py_str("True when this line already hands the answer over as it applies to the person.");
    return mcall(&mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "startswith", &[g__PERSONAL_HEADS()]);
    Py::None
}
pub fn _their_subject(args: &[Py]) -> Py {
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
    let _ = py_str("The thing a line names for itself in \"what does that mean for my pay\", or \"\".");
    found = mcall(&g_FOR_MINE(), "search", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return if py_truthy(&found.clone()) { mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), "split", &[])]) } else { py_str("") };
    Py::None
}
pub fn _on_the_table(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut noun: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "noun") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: noun"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut head: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when `noun` is already what the last turn or two is about.\n\n    \"so what does that mean for my pay\" after a card about an emergency tax code is the same ask\n    as \"what does that mean for me\": the card is about their pay and the word is in their own\n    line. \"what does that mean for my mortgage\" after a card about savings rates names a subject\n    nothing has said, so it is a question about that subject and not a follow-up on the bot's own\n    line - and the bot's sentence, handed back as it applies to a mortgage, would be a guess.\n    Read on the words of the last two turns both ways about, so the compound the person used\n    (\"payslip\" for \"pay\", \"boiler pressure\" for \"boiler\") counts.\n    ");
    head = mcall(&{ let __b = noun.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[]);
    head = if py_truthy(&head.clone()) { geti(&head.clone(), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    said = py_str("");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug2 = add(&said.clone(), &modulo(&py_str(" %s %s"), &py_tuple(vec![{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }, { let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }])));
    said = __aug2;
    }
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[modulo(&py_str("\\b%s"), &stdlib_call("re", "escape", &[head.clone()]).unwrap_or_else(|e| exc::raise(e))), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _under_the_lead(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The answer a hand-back is carrying, whichever module wrote the lead-in.\n\n    `_carried` reads the frames of the module that hands short forms back (`again.unframed`); this\n    module's own leads (\"I mean this: ...\", \"For you, that means: ...\") are not among them, so a\n    personal ask after one of those found nothing to apply to the person.\n    ");
    text = mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__HERE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    head = __item.clone();
    if py_truthy(&mcall(&mcall(&text.clone(), "lower", &[]), "startswith", &[mcall(&head.clone(), "lower", &[])])) {
    return mcall(&slice_or_raise(&text.clone(), &call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    }
    }
    return _carried(&[text.clone()]);
    Py::None
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
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let _ = py_str("The sentences of an answer, with a quotation kept in one piece.\n\n    The split cuts at a full stop and a space, and the words an answer hands over to say have\n    full stops of their own: \"say this: \"I'm not going to do this justice by Friday. Could I have\n    until Tuesday?\"\" is one sentence to a person and three to the splitter. Read as three, the\n    first of them - half a letter, quote left open - was handed back as the sentence the bot\n    meant, which is what \"say that again\" after the essay answer printed.\n    ");
    return crate::gen::memory::quotes::sentences(&[said.clone(), g_SENTENCE()]);
    Py::None
}
pub fn _gist(args: &[Py]) -> Py {
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
    let mut after_a_colon: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut explained: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The first clause of a sentence: the bit that carries what the bot meant.\n\n    \"Most back pain is muscular and settles in two to six weeks, and staying active is what helps\n    most: keep walking...\" is about the two to six weeks; handing the whole thing back (and\n    cutting it off with an ellipsis) says less than the gist does.\n\n    The exception is a sentence that says how the news lands and puts the explanation after a\n    colon: what was meant is on the right of it, so \"That is good news, and there is nothing to do\n    about it: a clear result means nothing was found\" hands over the second half. The test is the\n    same one `_meant` uses - the half on the right has to say what the thing means - so \"Grinding\n    means the pads are down to metal: stop driving and get them changed\" still keeps the left.\n    ");
    sentence = mcall(&{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    cut = Py::None;
    after_a_colon = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&mcall(&g_CLAUSE(), "finditer", &[sentence.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&ge(&mcall(&match_.clone(), "start", &[]), &g_LEAST())) {
    cut = mcall(&match_.clone(), "start", &[]);
    after_a_colon = mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "startswith", &[py_str(":")]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_is(&cut.clone(), &Py::None))) {
    explained = mcall(&mcall(&slice_or_raise(&sentence.clone(), &cut.clone(), &Py::None, &Py::None), "lstrip", &[py_str(":;- ")]), "strip", &[]);
    if py_truthy(&{ let __b = after_a_colon.clone(); if !py_truthy(&__b) { __b } else { { let __b = ge(&call_builtin("len", &[explained.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LEAST()); if !py_truthy(&__b) { __b } else { mcall(&g_DEFINES(), "search", &[explained.clone()]) } } } }) {
    sentence = add(&mcall(&slice_or_raise(&explained.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&explained.clone(), &Py::Int(1), &Py::None, &Py::None));
    } else {
    sentence = slice_or_raise(&sentence.clone(), &Py::None, &cut.clone(), &Py::None);
    }
    }
    sentence = mcall(&mcall(&sentence.clone(), "strip", &[]), "rstrip", &[py_str(",;:- ")]);
    if py_truthy(&gt(&call_builtin("len", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST())) {
    sentence = mcall(&geti(&mcall(&slice_or_raise(&sentence.clone(), &Py::None, &g_MOST(), &Py::None), "rsplit", &[py_str(" "), Py::Int(1)]), &Py::Int(0)), "rstrip", &[py_str(",;:- ")]);
    }
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str(".!?"), &geti(&sentence.clone(), &neg(&Py::Int(1))))) } }) {
    let __aug2 = add(&sentence.clone(), &py_str("."));
    sentence = __aug2;
    }
    return sentence.clone();
    Py::None
}
pub fn _answers(args: &[Py]) -> Py {
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
    let mut back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { g_LOOK_BACK() };
    let mut out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The bot's own lines in the last turn or two, newest first, each with the line it answered.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &neg(&back.clone()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    break;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_TO_REPEAT(), &call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    break;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![said.clone(), mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])]);
    }
    return out.clone();
    Py::None
}
pub fn _thick_enough(args: &[Py]) -> Py {
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
    let mut asked: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut theirs: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the bot's line could be explaining something, rather than saying how the news lands.\n\n    \"Ugh, that's annoying.\" is the right reply to \"my rent went up by 200\", and it is not an\n    explanation of anything: handing it back as \"I said: Ugh, that's annoying.\" tells a person who\n    asked what the bot meant that they missed something they had in front of them. A line says\n    something in the person's own subject when it is long enough to, or when it takes a word of\n    what they said into it; a short line that shares nothing with their line is a comment on it.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::words(&[said.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::gen::memory::topics::g_STOPWORDS(), &w.clone())); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&ge(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {
    return Py::Bool(true);
    }
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::topics::words(&[asked.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::gen::memory::topics::g_STOPWORDS(), &w.clone())); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    return call_builtin("bool", &[mcall(&theirs.clone(), "intersection", &[words.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _framed(args: &[Py]) -> Py {
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
    let _ = py_str("True when this line already has a hand-back lead on it, from this module or from repeat.");
    return { let __b = crate::gen::memory::repeat::framed(&[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "startswith", &[g__HERE()]) } };
    Py::None
}
pub fn _a_figure(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line states a figure, which is an answer however short it is.\n\n    \"180°C is 356°F.\" is sixteen characters and it is the whole of the answer to \"what is\n    180c in fahrenheit\", but it is under the length a line needs before it counts as an\n    explanation, so a person who said \"im confused\" or \"what?\" after it was told \"Nothing of mine\n    to go back over yet\" - as if the bot had said nothing at all. A comment on how the news lands\n    (\"Ugh, that's annoying.\") is short in the same way and answers nothing, and the figure is what\n    tells the two apart: a line that gives one has answered something, whatever its length.\n    ");
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\d"), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_question(args: &[Py]) -> Py {
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
    let _ = py_str("True when the sentence the bot would hand over is its own question.");
    return mcall(&mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "endswith", &[py_str("?")]);
    Py::None
}
pub fn _worth_handing_over(args: &[Py]) -> Py {
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
    let mut sentence: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut asked: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let _ = py_str("Whether what would come back is an answer rather than a way of saying how the news lands.\n\n    A long line can be a comment with a question after it (\"Ugh, that's a lot to carry. What would\n    make today a bit lighter?\"). Handing the comment over says the bot explained the subject when\n    all it did was say how the news lands, so the sentence itself has to be an answer: it states a\n    figure, it is the bot's own question (which is what \"what do you mean\" is asking about), or it\n    says enough in the person's own subject. The whole line still has to hold something - a\n    greeting that only asks \"what are we working on?\" has nothing to go over.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _thick_enough(&[said.clone(), asked.clone()]); if py_truthy(&__b) { __b } else { _a_figure(&[said.clone()]) } }))) {
    return Py::Bool(false);
    }
    return { let __b = _a_figure(&[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_question(&[sentence.clone()]); if py_truthy(&__b) { __b } else { _thick_enough(&[sentence.clone(), asked.clone()]) } } } };
    Py::None
}
pub fn _cite(args: &[Py]) -> Py {
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
    let mut sentence: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut asked: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { py_str("") };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("The sentence to hand over, or None when there is nothing to cite: the bot's own line was a\n    hedge (so it had no answer), a receipt for the person's line, a comment on how the news lands,\n    or a hand-back that already has a lead on it.");
    if py_truthy(&mcall(&crate::gen::memory::repeat::g_HEDGE(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_CONDOLENCE(), "match", &[said.clone()])) {
    said = _after_the_condolence(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &sentence.clone())) } }) {
    sentence = _meant(&[said.clone(), mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")])]);
    }
    }
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ANSWERED()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_figure(&[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_thick_enough(&[said.clone(), asked.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_RECEIPT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_RECEIPT(), "search", &[sentence.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&_framed(&[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("repeated")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_worth_handing_over(&[said.clone(), sentence.clone(), asked.clone()])))) {
    return Py::None;
    }
    return _gist(&[sentence.clone()]);
    Py::None
}
pub fn _meant(args: &[Py]) -> Py {
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
    let mut asked_what_it_meant: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked_what_it_meant") { v.clone() }
    else { Py::Bool(false) };
    let mut first: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentence the bot meant: its question when its own line ended on one, otherwise the\n    first thing it said that is not a hedge - or, when that first thing only tells the person\n    where to look or what to do, the sentence further on that says what the thing means.\n\n    Which one depends on the ask. \"What do you mean\" asks what the bot was getting at, and a line\n    that ended on the bot's own question was getting at the answer to that question. \"I don't\n    follow\" asks for the same content in plainer words, so where the line said something and then\n    asked, the something is what went past them: \"Then start with the smallest version of it.\n    What would the first step be?\" handed back as the question tells them nothing they did not\n    just read.\n    ");
    parts = _sentences(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if !py_truthy(&__b) { __b } else { { let __b = asked_what_it_meant.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } } } }) {
    return geti(&parts.clone(), &neg(&Py::Int(1)));
    }
    first = py_str("");
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::memory::repeat::g_HEDGE(), "search", &[sentence.clone()]))); if !py_truthy(&__b) { __b } else { ge(&call_builtin("len", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ANSWERED()) } }) {
    first = sentence.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&first.clone()))) {
    return geti(&parts.clone(), &Py::Int(0));
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_DEFINES(), "search", &[first.clone()])))) {
    let __iter2 = py_iter_or_raise(&parts.clone());
    for __item in __iter2 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&sentence.clone(), &first.clone())); if py_truthy(&__b) { __b } else { mcall(&crate::gen::memory::repeat::g_HEDGE(), "search", &[sentence.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ANSWERED()); if !py_truthy(&__b) { __b } else { mcall(&g_DEFINES(), "search", &[sentence.clone()]) } }) {
    return sentence.clone();
    }
    }
    }
    return first.clone();
    Py::None
}
pub fn _after_the_condolence(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("What the bot said after it said how the news lands, or \"\" when it only said that.\n\n    The condolence is the opener, not the whole line: \"Sorry to hear it. Give it a day before\n    deciding anything, then look at what you can control next time.\" has an answer in it, and a\n    person asking for plainer words wants that bit, without being told the bot explained the\n    subject.\n    ");
    cut = mcall(&g_CONDOLENCE_SENTENCE(), "match", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&cut.clone()))) {
    return py_str("");
    }
    rest = mcall(&py_str(" "), "join", &[mcall(&slice_or_raise(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &mcall(&cut.clone(), "end", &[]), &Py::None, &Py::None), "split", &[])]);
    return rest.clone();
    Py::None
}
pub fn _same_bit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut head: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut long: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("Whether two lines hand over the same bit, even when one is cut shorter than the other:\n    \"Most back pain is muscular\" and \"Most back pain is muscular and settles in two weeks\" are\n    the same answer to the same question.");
    one = mcall(&mcall(&mcall(&{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "rstrip", &[py_str(".?! ")]), "lower", &[]);
    other = mcall(&mcall(&mcall(&{ let __b = other.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "rstrip", &[py_str(".?! ")]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&add(&g_LEADS(), &g_REPEAT_LEADS()));
    for __item in __iter1 {
    lead = __item.clone();
    head = mcall(&geti(&mcall(&lead.clone(), "split", &[py_str("{")]), &Py::Int(0)), "lower", &[]);
    if py_truthy(&mcall(&other.clone(), "startswith", &[head.clone()])) {
    other = mcall(&slice_or_raise(&other.clone(), &call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&other.clone())) } }) {
    return Py::Bool(false);
    }
    let __unpk2 = py_unpack(&if py_truthy(&le(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![one.clone(), other.clone()]) } else { py_tuple(vec![other.clone(), one.clone()]) }, 2, None);
    short = __unpk2[0].clone();
    long = __unpk2[1].clone();
    return { let __b = mcall(&long.clone(), "startswith", &[short.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&long.clone(), &short.clone())) } };
    Py::None
}
pub fn _already(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cited: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cited") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cited"))) };
    let mut said_since: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_since") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_since"))) };
    let mut line: Py = Py::None;
    let _ = py_str("The same bit twice reads worse than moving on: once it has been handed over, the question\n    has had its answer.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_since.clone()) { let mut line: Py = __item.clone(); __out.push(_same_bit(&[cited.clone(), line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _carried(args: &[Py]) -> Py {
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
    let _ = py_str("The answer a hand-back is carrying, or \"\" when the line above is not one.\n\n    Shared with the module that writes the frames (`again.unframed`), so every lead-in the codebase\n    puts in front of an answer is read through: \"my knee hurts when i run\" twice, then \"say that\n    again\", was answered out of the line \"Same as before: Cut the distance and the speed for a week\n    or two\" - that is the frame's answer, and handing it back under a second frame put two lead-ins\n    on one sentence.\n    ");
    return { let __b = crate::gen::memory::again::unframed(&[said.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    Py::None
}
pub fn _applied_to_the_person(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut asked: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut memory: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut what_it_meant: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what_it_meant") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what_it_meant"))) };
    let mut carried: Py = Py::None;
    let mut cited: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut someone: Py = Py::None;
    let _ = py_str("The last answer handed back as it applies to the person asked about, or None.\n\n    \"what does that mean for me\" after a hand-back is the same sentence applied to them, not a\n    repeat of the sentence, and \"for him\" names the person it is applied to. This is read as soon\n    as the nearest answer is a hand-back, before the window widens: the sentence under the last\n    hand-back is the one the person is asking about, and reaching further back for a sentence that\n    `_cite` will take got \"For him, that means: 4, 6 and 11 add to 21, and 21 ÷ 3 = 7, so the mean\n    is 7\" after a chat that had also asked what an average is - the maths card, three turns back.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = what_it_meant.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_PERSONAL(), "search", &[clean.clone()]) } }))) {
    return Py::None;
    }
    carried = _under_the_lead(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&carried.clone())); if py_truthy(&__b) { __b } else { _personal(&[said.clone()]) } }) {
    return Py::None;
    }
    noun = _their_subject(&[clean.clone()]);
    if py_truthy(&{ let __b = noun.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_on_the_table(&[noun.clone(), memory.clone()]))) } }) {
    return Py::None;
    }
    cited = { let __b = _cite(&[carried.clone(), _meant(&[carried.clone(), what_it_meant.clone()]), asked.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { carried.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&cited.clone()))) {
    return Py::None;
    }
    someone = mcall(&g_SOMEONE_ELSE(), "search", &[clean.clone()]);
    if py_truthy(&someone.clone()) {
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), mcall(&mcall(&someone.clone(), "group", &[Py::Int(1)]), "lower", &[])), ("said".to_string(), cited.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_LEADS(), "format", args))(&__args) };
    } else {
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), cited.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_PERSONAL_LEADS(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return crate::gen::memory::repeat::handed_back(&[memory.clone(), lead.clone()]);
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut applied: Py = Py::None;
    let mut applied_once: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut back: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut cited: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut repaired: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_since: Py = Py::None;
    let mut what_it_meant: Py = Py::None;
    let _ = py_str("The bit the bot meant, or None when this is not that turn.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", &[]);
    repaired = mcall(&{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&repaired.clone(), &clean.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_MEAN(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REPEAT(), "match", &[clean.clone()]) } })) } } } }) {
    if py_truthy(&{ let __b = mcall(&g_MEAN(), "match", &[repaired.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REPEAT(), "match", &[repaired.clone()]) } }) {
    clean = repaired.clone();
    }
    }
    if py_truthy(&{ let __b = mcall(&g_MEAN(), "match", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_ASKS_SOMETHING(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_MEAN(), "match", &[clean.clone()])) {
    leads = g_LEADS();
    what_it_meant = Py::Bool(true);
    if py_truthy(&{ let __b = _their_subject(&[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_on_the_table(&[_their_subject(&[clean.clone()]), memory.clone()]))) } }) {
    return Py::None;
    }
    about = mcall(&g_ABOUT_MINE(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = about.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_on_the_table(&[mcall(&about.clone(), "group", &[Py::Int(1)]), memory.clone()]))) } }) {
    return Py::None;
    }
    } else {
    if py_truthy(&mcall(&g_REPEAT(), "match", &[clean.clone()])) {
    leads = g_REPEAT_LEADS();
    what_it_meant = Py::Bool(false);
    } else {
    if py_truthy(&_asked_plainly(&[clean.clone()])) {
    leads = g_REPEAT_LEADS();
    what_it_meant = Py::Bool(false);
    } else {
    return Py::None;
    }
    }
    }
    said_since = py_list(vec![]);
    lines = py_list(vec![]);
    applied_once = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_LOOK_BACK(), g_LOOK_BACK_FAR()]));
    for __item in __iter1 {
    back = __item.clone();
    let __iter2 = py_iter_or_raise(&_answers(&[memory.clone(), back.clone()]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    said = __unpk3[0].clone();
    asked = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_contains(&said_since.clone(), &said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&applied_once.clone())); if !py_truthy(&__b) { __b } else { { let __b = what_it_meant.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PERSONAL(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { _personal(&[said.clone()]) } } } } } }) {
    applied_once = Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&applied_once.clone()))) {
    applied = _applied_to_the_person(&[clean.clone(), said.clone(), asked.clone(), memory.clone(), what_it_meant.clone()]);
    if py_truthy(&applied.clone()) {
    return applied.clone();
    }
    }
    carried = _carried(&[said.clone()]);
    if py_truthy(&carried.clone()) {
    cited = _cite(&[carried.clone(), _meant(&[carried.clone(), what_it_meant.clone()]), asked.clone(), memory.clone()]);
    if py_truthy(&{ let __b = cited.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already(&[cited.clone(), said_since.clone()]))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), cited.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[leads.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    let _ = mcall(&said_since.clone(), "append", &[said.clone()]);
    let _ = mcall(&lines.clone(), "append", &[py_tuple(vec![said.clone(), asked.clone()])]);
    continue;
    }
    cited = _cite(&[said.clone(), _meant(&[said.clone(), what_it_meant.clone()]), asked.clone(), memory.clone()]);
    if py_truthy(&{ let __b = cited.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_already(&[cited.clone(), said_since.clone()]))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), cited.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[leads.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    let _ = mcall(&said_since.clone(), "append", &[said.clone()]);
    let _ = mcall(&lines.clone(), "append", &[py_tuple(vec![said.clone(), asked.clone()])]);
    }
    }
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { _framed(&[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))]) } }) {
    carried = _under_the_lead(&[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))]);
    noun = _their_subject(&[clean.clone()]);
    if py_truthy(&{ let __b = what_it_meant.clone(); if !py_truthy(&__b) { __b } else { { let __b = carried.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PERSONAL(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(!py_truthy(&noun.clone())); if py_truthy(&__b) { __b } else { _on_the_table(&[noun.clone(), memory.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_personal(&[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))]))) } } } } } } } }) {
    applied = _applied_to_the_person(&[clean.clone(), geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0)), geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(1)), memory.clone(), what_it_meant.clone()]);
    if py_truthy(&applied.clone()) {
    return applied.clone();
    }
    }
    return crate::gen::memory::repeat::handed_back(&[memory.clone(), crate::gen::compose::pick(&[g_SAID_TWICE(), memory.clone(), clean.clone()])]);
    }
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_CONDOLENCE(), "match", &[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))]) } }) {
    return crate::gen::memory::repeat::handed_back(&[memory.clone(), crate::gen::compose::pick(&[g_NOTHING_WRITTEN(), memory.clone(), clean.clone()])]);
    }
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { mcall(&crate::gen::memory::repeat::g_HEDGE(), "search", &[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))]) } }) {
    return crate::gen::memory::repeat::handed_back(&[memory.clone(), crate::gen::compose::pick(&[g_NO_ANSWER_OF_MINE(), memory.clone(), clean.clone()])]);
    }
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_RECEIPT(), "search", &[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_figure(&[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_thick_enough(&[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0)), geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(1))]))); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[_meant(&[geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))])]).unwrap_or_else(|e| exc::raise(e)), &g_ANSWERED()) } } } } } } } }) {
    return crate::gen::memory::repeat::handed_back(&[memory.clone(), crate::gen::compose::pick(&[g_NOTHING_WRITTEN(), memory.clone(), clean.clone()])]);
    }
    return Py::None;
    Py::None
}
pub fn reads_it(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let _ = py_str("Whether the line asks for the last answer again - the one ask that repeats on purpose.\n\n    The hand-back this module writes is the answer just given with a lead in front of it, so the\n    module that owns the turn has to know the repeat was asked for and not a card read out twice.\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[{ let __b = mcall(&g_MEAN(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_REPEAT(), "match", &[clean.clone()]); if py_truthy(&__b) { __b } else { _asked_plainly(&[clean.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.meant".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_MINE" => Some(g_ABOUT_MINE()),
            "ANSWERED" => Some(g_ANSWERED()),
            "ASKING_ABOUT_DOING" => Some(g_ASKING_ABOUT_DOING()),
            "ASKS_SOMETHING" => Some(g_ASKS_SOMETHING()),
            "CLAUSE" => Some(g_CLAUSE()),
            "CONDOLENCE" => Some(g_CONDOLENCE()),
            "CONDOLENCE_SENTENCE" => Some(g_CONDOLENCE_SENTENCE()),
            "DEFINES" => Some(g_DEFINES()),
            "FOLLOW_UP_WORDS" => Some(g_FOLLOW_UP_WORDS()),
            "FOR_MINE" => Some(g_FOR_MINE()),
            "LEADS" => Some(g_LEADS()),
            "LEAST" => Some(g_LEAST()),
            "LOOK_BACK" => Some(g_LOOK_BACK()),
            "LOOK_BACK_FAR" => Some(g_LOOK_BACK_FAR()),
            "MEAN" => Some(g_MEAN()),
            "MOST" => Some(g_MOST()),
            "NOTHING_WRITTEN" => Some(g_NOTHING_WRITTEN()),
            "NOT_LETTERS" => Some(g_NOT_LETTERS()),
            "NOT_TO_REPEAT" => Some(g_NOT_TO_REPEAT()),
            "NO_ANSWER_OF_MINE" => Some(g_NO_ANSWER_OF_MINE()),
            "OPENERS" => Some(g_OPENERS()),
            "PERSONAL" => Some(g_PERSONAL()),
            "PERSONAL_LEADS" => Some(g_PERSONAL_LEADS()),
            "PUT_IT_PLAINER" => Some(g_PUT_IT_PLAINER()),
            "RECEIPT" => Some(g_RECEIPT()),
            "REPEAT" => Some(g_REPEAT()),
            "REPEAT_LEADS" => Some(g_REPEAT_LEADS()),
            "SAID_TWICE" => Some(g_SAID_TWICE()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "SOMEONE_LEADS" => Some(g_SOMEONE_LEADS()),
            "TELLS" => Some(g_TELLS()),
            "WHAT_YOU_ARE" => Some(g_WHAT_YOU_ARE()),
            "_HERE" => Some(g__HERE()),
            "_PERSONAL_HEADS" => Some(g__PERSONAL_HEADS()),
            "_a_figure" => Some(py_func("_a_figure", |args: &[Py]| _a_figure(args))),
            "_a_question" => Some(py_func("_a_question", |args: &[Py]| _a_question(args))),
            "_after_the_condolence" => Some(py_func("_after_the_condolence", |args: &[Py]| _after_the_condolence(args))),
            "_already" => Some(py_func("_already", |args: &[Py]| _already(args))),
            "_answers" => Some(py_func("_answers", |args: &[Py]| _answers(args))),
            "_applied_to_the_person" => Some(py_func("_applied_to_the_person", |args: &[Py]| _applied_to_the_person(args))),
            "_asked_plainly" => Some(py_func("_asked_plainly", |args: &[Py]| _asked_plainly(args))),
            "_carried" => Some(py_func("_carried", |args: &[Py]| _carried(args))),
            "_cite" => Some(py_func("_cite", |args: &[Py]| _cite(args))),
            "_framed" => Some(py_func("_framed", |args: &[Py]| _framed(args))),
            "_gist" => Some(py_func("_gist", |args: &[Py]| _gist(args))),
            "_meant" => Some(py_func("_meant", |args: &[Py]| _meant(args))),
            "_on_the_table" => Some(py_func("_on_the_table", |args: &[Py]| _on_the_table(args))),
            "_personal" => Some(py_func("_personal", |args: &[Py]| _personal(args))),
            "_same_bit" => Some(py_func("_same_bit", |args: &[Py]| _same_bit(args))),
            "_sentences" => Some(py_func("_sentences", |args: &[Py]| _sentences(args))),
            "_their_subject" => Some(py_func("_their_subject", |args: &[Py]| _their_subject(args))),
            "_thick_enough" => Some(py_func("_thick_enough", |args: &[Py]| _thick_enough(args))),
            "_under_the_lead" => Some(py_func("_under_the_lead", |args: &[Py]| _under_the_lead(args))),
            "_worth_handing_over" => Some(py_func("_worth_handing_over", |args: &[Py]| _worth_handing_over(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "reads_it" => Some(py_func("reads_it", |args: &[Py]| reads_it(args))),
            _ => None,
        }),
        set: None,
    }))
}
