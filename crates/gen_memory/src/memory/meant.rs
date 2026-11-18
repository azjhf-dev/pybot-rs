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


thread_local! { static G_ABOUT_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_ABOUT_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_SOMETHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDOLENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDOLENCE_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEFINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK_FAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_WRITTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_LETTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_TO_REPEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_ANSWER_OF_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSONAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSONAL_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUT_IT_PLAINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECEIPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_TWICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_YOU_ARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSONAL_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_MINE() -> Py { ensure_init(); G_ABOUT_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_MINE(v: Py) { ensure_init(); G_ABOUT_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERED() -> Py { ensure_init(); G_ANSWERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERED(v: Py) { ensure_init(); G_ANSWERED.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_ABOUT_DOING() -> Py { ensure_init(); G_ASKING_ABOUT_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_ABOUT_DOING(v: Py) { ensure_init(); G_ASKING_ABOUT_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_SOMETHING() -> Py { ensure_init(); G_ASKS_SOMETHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_SOMETHING(v: Py) { ensure_init(); G_ASKS_SOMETHING.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE() -> Py { ensure_init(); G_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE(v: Py) { ensure_init(); G_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDOLENCE() -> Py { ensure_init(); G_CONDOLENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDOLENCE(v: Py) { ensure_init(); G_CONDOLENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDOLENCE_SENTENCE() -> Py { ensure_init(); G_CONDOLENCE_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDOLENCE_SENTENCE(v: Py) { ensure_init(); G_CONDOLENCE_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_DEFINES() -> Py { ensure_init(); G_DEFINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEFINES(v: Py) { ensure_init(); G_DEFINES.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP_WORDS() -> Py { ensure_init(); G_FOLLOW_UP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_WORDS(v: Py) { ensure_init(); G_FOLLOW_UP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_MINE() -> Py { ensure_init(); G_FOR_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_MINE(v: Py) { ensure_init(); G_FOR_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS() -> Py { ensure_init(); G_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); G_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAST() -> Py { ensure_init(); G_LEAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAST(v: Py) { ensure_init(); G_LEAST.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK() -> Py { ensure_init(); G_LOOK_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); G_LOOK_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK_FAR() -> Py { ensure_init(); G_LOOK_BACK_FAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK_FAR(v: Py) { ensure_init(); G_LOOK_BACK_FAR.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAN() -> Py { ensure_init(); G_MEAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAN(v: Py) { ensure_init(); G_MEAN.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST() -> Py { ensure_init(); G_MOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST(v: Py) { ensure_init(); G_MOST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_WRITTEN() -> Py { ensure_init(); G_NOTHING_WRITTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_WRITTEN(v: Py) { ensure_init(); G_NOTHING_WRITTEN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_LETTERS() -> Py { ensure_init(); G_NOT_LETTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_LETTERS(v: Py) { ensure_init(); G_NOT_LETTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_TO_REPEAT() -> Py { ensure_init(); G_NOT_TO_REPEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_TO_REPEAT(v: Py) { ensure_init(); G_NOT_TO_REPEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_ANSWER_OF_MINE() -> Py { ensure_init(); G_NO_ANSWER_OF_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_ANSWER_OF_MINE(v: Py) { ensure_init(); G_NO_ANSWER_OF_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENERS() -> Py { ensure_init(); G_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENERS(v: Py) { ensure_init(); G_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSONAL() -> Py { ensure_init(); G_PERSONAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSONAL(v: Py) { ensure_init(); G_PERSONAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSONAL_LEADS() -> Py { ensure_init(); G_PERSONAL_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSONAL_LEADS(v: Py) { ensure_init(); G_PERSONAL_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_PUT_IT_PLAINER() -> Py { ensure_init(); G_PUT_IT_PLAINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUT_IT_PLAINER(v: Py) { ensure_init(); G_PUT_IT_PLAINER.with(|c| *c.borrow_mut() = v); }
pub fn g_RECEIPT() -> Py { ensure_init(); G_RECEIPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECEIPT(v: Py) { ensure_init(); G_RECEIPT.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT() -> Py { ensure_init(); G_REPEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT(v: Py) { ensure_init(); G_REPEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_LEADS() -> Py { ensure_init(); G_REPEAT_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_LEADS(v: Py) { ensure_init(); G_REPEAT_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_TWICE() -> Py { ensure_init(); G_SAID_TWICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_TWICE(v: Py) { ensure_init(); G_SAID_TWICE.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); G_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); G_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_LEADS() -> Py { ensure_init(); G_SOMEONE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_LEADS(v: Py) { ensure_init(); G_SOMEONE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_TELLS() -> Py { ensure_init(); G_TELLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELLS(v: Py) { ensure_init(); G_TELLS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_YOU_ARE() -> Py { ensure_init(); G_WHAT_YOU_ARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_YOU_ARE(v: Py) { ensure_init(); G_WHAT_YOU_ARE.with(|c| *c.borrow_mut() = v); }
pub fn g__HERE() -> Py { ensure_init(); G__HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HERE(v: Py) { ensure_init(); G__HERE.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSONAL_HEADS() -> Py { ensure_init(); G__PERSONAL_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSONAL_HEADS(v: Py) { ensure_init(); G__PERSONAL_HEADS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("They ask what the bot meant, and the answer is in the line it just gave.\n\n\"what do you mean by that\" is one of the commonest follow-ups in the language, and it was\nanswered with a search hedge (\"That's beyond what I know well, sorry - searching for it will turn\nup better sources\"), as if the line the bot had just written had nothing to do with the question.\nIt has everything to do with it: the bot meant the first thing it said, or - when its own line\nended on a question - the question it asked.\n\nSo: a bare \"what do you mean\", pointed at the sentence the bot meant, with a lead-in that hands\nthe bit over (\"Just this: ...\").\n\nThe rule is narrow on purpose:\n* the line is only that question and nothing else, so \"what do you mean about the deposit\" - a\n  question with a thing of its own - never comes here;\n* there has to be a bot turn in the last turn or two to point at, so an opening \"what do you mean\n  by that\" (nothing has been meant yet) keeps the honest hedge;\n* a hedge is never cited back at itself: where the bot had no answer, the question still gets none;\n* one sentence of the answer, cut at its first clause boundary, never the paragraph again, and\n  never the line this rule already gave.\n");
    s_OPENERS(py_str("^(?:(?:and|so|but|ok|okay|well|right|sorry|wait|basically|hmm|um|er|uh|ah|oh|yeah|then|no|nope|again|hold\\s+on|alright)\\b[\\s,]+)*(?:(?:i\\s*(?:'m|m)\\s*)?not\\s+sure(?:\\s+(?:what|that|if))?[\\s,]+|i\\s+don(?:'t|t)\\s+(?:get|understand|follow)\\b[\\s,]*(?:it|that|this)?[\\s,]*|(?:i\\s+)?did(?:n'?t|nt)\\s+(?:catch|follow)(?:\\s+(?:that|you))?[\\s,]+|not\\s+following[\\s,]+|you\\s+lost\\s+me[\\s,]+)*"));
    s_MEAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^\\s*"), &g_OPENERS()), &py_str("(?:(?:what|wot|wat|wut|whut)\\s+(?:(?:do|did|d)\\s+)?(?:you|u|ya)\\s+mean(?:\\s+(?:by\\s+(?:that|it|this)|exactly|though|then))?|(?:whatcha|watcha|whaddya|whadya|whatdya)\\s+mean(?:\\s+by\\s+(?:that|it|this))?|how\\s+(?:do|did)\\s+(?:you|u|ya)\\s+mean(?:\\s+(?:by\\s+(?:that|it|this)|exactly))?|what(?:'s|s)?\\s+(?:(?:does|do|is)\\s+)?(?:that|it|this)\\s+(?:(?:actually|really|exactly|precisely|even|then)\\s+)?(?:supposed\\s+to\\s+)?mean(?:\\s+(?:then|though|exactly|precisely))?(?:\\s+by\\s+(?:that|it|this))?(?:\\s+for\\s+(?:me|us|him|her|them|my\\s+case|my\\s+situation|(?:my|our)\\s+[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+)?))?(?:\\s+about\\s+(?:my|our|the|this|that)\\s+[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,2})?(?:\\s+in\\s+(?:practice|reality|real\\s+terms))?(?:\\s+(?:then|though|exactly|precisely))?|meaning\\s+what|explain\\s+(?:what\\s+)?(?:you|u)\\s+mean|wdym|mean\\s+what\\s+exactly|(?:what(?:'s|s)?|what\\s+is)\\s+(?:the\\s+)?meaning\\s+of\\s+(?:that|it|this)|(?:the\\s+)?meaning\\s+of\\s+(?:that|it|this)|and\\s+that\\s+means\\?|what\\s+is\\s+meant\\s+by\\s+(?:that|it|this)|whats?\\s+the\\s+meaning|(?:that|it|this)\\s+means\\s+what(?:\\s+(?:exactly|then|though|precisely))?|what\\s+(?:you|u|ya)\\s+mean|what\\s+(?:that|it|this)\\s+means?(?:\\s+(?:exactly|then|though|precisely))?)(?:,\\s*(?:then|though|exactly|precisely|really|actually)\\b)?\\s*[?.!]*\\s*$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_SOMETHING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",\\s*(?:is|are|was|were|am|does|do|did|will|would|can|could|should|shall|has|have)\\s+[\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,6}\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REPEAT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:and\\s+|so\\s+|but\\s+|ok\\s+|okay\\s+|well\\s+|right\\s+|sorry\\s+|er+\\s+|um+\\s+)*(?:(?:can|could|would)\\s+you\\s+(?:please\\s+)?(?:say|repeat|explain|rephrase|clarify)(?:\\s+(?:that|it))?(?:\\s+again)?(?:\\s+please)?|(?:say|repeat)\\s+(?:that|it)\\s+again|explain(?:\\s+that)?|explain\\s+again|meaning|(?:i\\s+)?(?:(?:still|really|just|honestly|actually|literally)\\s+)?(?:do\\s*n[o']?t|dont|does\\s*n[o']?t|did\\s*n[o']?t|didnt)\\s+(?:(?:really|still|quite|honestly|fully)\\s+|at\\s+all\\s+)?(?:understand|follow|get\\s+it|catch\\s+that|(?:get|see|follow)\\s+what\\s+you(?:'re|\\s+are)?\\s+(?:saying|on\\s+about|mean))(?:\\s+(?:it|that|this|at\\s+all|one\\s+bit|any\\s+of\\s+it|what\\s+you\\s+mean|a\\s+word\\s+of\\s+it|the\\s+last\\s+bit))?|(?:i'?m|im)\\s+not\\s+(?:following|with\\s+you)|(?:i'?m|im)\\s+(?:still\\s+|a\\s+bit\\s+|totally\\s+|completely\\s+)?(?:confused|lost)|(?:you\\s+)?(?:have\\s+)?lost\\s+me(?:\\s+(?:there|completely|entirely))?|(?:that|this)\\s+is\\s+confusing|(?:that'?s|thats)\\s+confusing|(?:what|pardon|eh|huh)\\?*|sorry\\s*,?\\s*(?:what|pardon|eh|huh)?\\s*\\?+|sorry\\s*,?\\s*(?:what|pardon|eh|huh)\\s*[?.!]*|(?:say|explain|put)\\s+it\\s+in\\s+(?:plain\\s+)?english(?:\\s+please)?|what(?:'s|s|\\s+is)?\\s+(?:that|this|it)\\s+in\\s+(?:plain\\s+)?english|in\\s+(?:plain\\s+)?english(?:\\s+please)?|english\\s+please|come\\s+again|say\\s+what)\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REPEAT_LEADS(py_tuple(vec![py_str("I said: {said}"), py_str("Same again: {said}"), py_str("Let me put it plainer: {said}")]));
    s_LEADS(py_tuple(vec![py_str("I mean this: {said}"), py_str("Just this: {said}"), py_str("What I meant: {said}")]));
    s_PERSONAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfor\\s+(?:me|us|him|her|them|my\\s+case|my\\s+situation|(?:my|our)\\s+[\\w'\\u2019-]+)\\b|\\bin\\s+(?:practice|reality|real\\s+terms)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FOR_MINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfor\\s+(?:my|our)\\s+(?!case\\b|situation\\b|own\\b|end\\b|side\\b|part\\b)([\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+)?)\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PERSONAL_LEADS(py_tuple(vec![py_str("For you, that means: {said}"), py_str("In your case: {said}")]));
    s_FOLLOW_UP_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("what whats which that this it thats those them they\n    mean means meaning meant you u ya your i im sorry ok okay so then and but well just really\n    actually exactly precisely simply plain plainer please can could would will do does did is\n    are was were there here have has had\n    put putting say saying said tell telling explained explaining explain\n    break down again another other way words wording sense terms term practice reality real life\n    english jargon\n    technical simpler simple clearly five like of in on about to for me us him her huh pardon\n    people normal someone folk most"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PUT_IT_PLAINER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("mean means meaning meant explain explained explaining\n    tell telling another sense terms term practice reality english jargon technical plain words\n    plainer simpler simple simply clearly break"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_LETTERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z'\\s]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TELLS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:that|this|it|those|these)\\b[^.?!]{0,20}\\btell")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHAT_YOU_ARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|who)\\s+(?:are|r)\\s+(?:you|u|ya)\\b|\\bwhat do you do\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKING_ABOUT_DOING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("do does did done work works took take takes costing cost\n    last lasts hurt hurts help helps happen happens matter matters"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ABOUT_MINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\babout\\s+(?:my|our|the|this|that)\\s+([\\w'\\u2019-]+(?:\\s+[\\w'\\u2019-]+){0,2})\\s*[?.!]*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SOMEONE_ELSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfor\\s+(him|her|them)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SOMEONE_LEADS(py_str("For {who}, that means: {said}"));
    s__PERSONAL_HEADS(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PERSONAL_LEADS()) { let mut lead: Py = __item.clone(); __out.push(geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "split", args))(&__args) }, &Py::Int(0))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_tuple(vec![py_str("For him, that means: "), py_str("For her, that means: "), py_str("For them, that means: ")])));
    s_SENTENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RECEIPT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:sure\\s*[-\\u2014]\\s*)?(?:noted\\b|got it\\b|so far i know\\b|thanks for telling me\\b|i hear you\\b|i'?m listening\\b|say more\\b|tell me more\\b|that sounds\\b|i'?ll keep that\\b|good to know\\b|nothing (?:to do|i can add)\\b|i'?m here if\\b|(?:i\\s+)?haven'?t put anything about it into words|nothing of mine to go back over|is there anything you'?d like\\b|you'?re (?:noted|on the list)\\b)|[\\u2014-]\\s*(?:noted|good to know|got it|i'?ll remember that|i'?ve got that)\\b[.!]*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOTHING_WRITTEN(py_tuple(vec![py_str("I haven't put anything about it into words yet - give me a bit more and I'll start from there."), py_str("Nothing of mine to go back over yet. Tell me a little more and I'll take it from the top."), py_str("I hadn't answered anything yet, so there's nothing of mine to put straighter - tell me what you were after and I'll take it on."), py_str("Nothing of mine to explain yet: I said how that landed and stopped there. Say what you were asking and I'll have a proper go at it."), py_str("That was me saying how it lands, not an answer. Ask me again and I'll take it on.")]));
    s_NO_ANSWER_OF_MINE(py_tuple(vec![py_str("I didn't have an answer there, so there's nothing of mine to explain - it just didn't know. Say a bit more about it and I'll have a proper go."), py_str("Nothing of mine to go over: I didn't know that one. Tell me more and I'll try again.")]));
    s_SAID_TWICE(py_tuple(vec![py_str("I've put that twice now, so the words in it are probably the problem - tell me which bit and I'll unpack that one."), py_str("That is the second time I've said it that way. Which word in it is the trouble?")]));
    s_CLAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":\\s+|\\s+-\\s+|;\\s+|,\\s+so\\s+|,\\s+and\\s+|,\\s+but\\s+|,\\s+which\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOST(Py::Int(150));
    s_LEAST(Py::Int(30));
    s_ANSWERED(Py::Int(20));
    s_LOOK_BACK(Py::Int(3));
    s_LOOK_BACK_FAR(Py::Int(7));
    s_DEFINES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bmeans?\\b|\\bis when\\b|\\bis what\\b|\\bis where\\b|\\bin other words\\b|\\brefers to\\b|\\bstands for\\b|\\bwhich means\\b|\\bthat is what\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_TO_REPEAT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("safety"), py_str("loss"), py_str("grief")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDOLENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:oh|ah)\\s*,?\\s+|(?:i'?m|i\\s+am)\\s+(?:so\\s+|really\\s+|very\\s+)?)*sorry\\b|^\\s*(?:oh\\s+no|ugh|that'?s\\s+(?:rough|hard|awful|a\\s+lot\\s+to\\s+carry)|that\\s+is\\s+rough|what\\s+a\\s+shame|how\\s+awful|that\\s+sounds\\s+(?:rough|tough|hard|awful|horrible))\\s*[,:.\\u2014-]")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&op_add(&g_LEADS(), &g_REPEAT_LEADS()), &g_PERSONAL_LEADS())) { let mut lead: Py = __item.clone(); __out.push(geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "split", args))(&__args) }, &Py::Int(0))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDOLENCE_SENTENCE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:oh|ah)\\s*,?\\s+|(?:i'?m|i\\s+am)\\s+(?:so\\s+|really\\s+|very\\s+)?)*sorry[^.!?]*[.!?]|^\\s*(?:oh\\s+no|ugh|that'?s\\s+(?:rough|hard|awful|a\\s+lot\\s+to\\s+carry)|that\\s+is\\s+rough|what\\s+a\\s+shame|how\\s+awful|that\\s+sounds\\s+(?:rough|tough|hard|awful|horrible))[^.!?]*[.!?]")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_LETTERS(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(9)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FOLLOW_UP_WORDS())))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_PUT_IT_PLAINER())))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&words.clone(), &py_str("tell"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("telling"))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TELLS(), "search", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHAT_YOU_ARE(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_ASKING_ABOUT_DOING(), &geti(&words.clone(), &op_neg(&Py::Int(1)))))) {
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__PERSONAL_HEADS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "startswith", args))(&__args) };
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
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FOR_MINE(), "search", args))(&__args) };
    return if py_truthy(&found.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) } } else { py_str("") };
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
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = noun.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "split", args))(&__args) };
    head = if py_truthy(&head.clone()) { geti(&head.clone(), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::Bool(false);
    }
    said = py_str("");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __aug2 = op_add(&said.clone(), &op_modulo(&py_str(" %s %s"), &py_tuple(vec![{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }])));
    said = __aug2;
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\b%s"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__HERE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    head = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&head.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }, "startswith", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _carried(args))(&__args) };
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_SENTENCE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::quotes::sentences(args))(&__args) };
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
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    cut = Py::None;
    after_a_colon = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAUSE(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &g_LEAST())) {
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) };
    after_a_colon = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "startswith", args))(&__args) };
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_is(&cut.clone(), &Py::None))) {
    explained = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(":;- ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&sentence.clone(), &cut.clone(), &Py::None, &Py::None), "lstrip", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = after_a_colon.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(explained.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_LEAST()); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(explained.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DEFINES(), "search", args))(&__args) } } } } }) {
    sentence = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&explained.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&explained.clone(), &Py::Int(1), &Py::None, &Py::None));
    } else {
    sentence = slice_or_raise(&sentence.clone(), &Py::None, &cut.clone(), &Py::None);
    }
    }
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:- ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&sentence.clone(), "strip", args))(&__args) }, "rstrip", args))(&__args) };
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_MOST())) {
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",;:- ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&sentence.clone(), &Py::None, &g_MOST(), &Py::None), "rsplit", args))(&__args) }, &Py::Int(0)), "rstrip", args))(&__args) };
    }
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str(".!?"), &geti(&sentence.clone(), &op_neg(&Py::Int(1))))) } }) {
    let __aug2 = op_add(&sentence.clone(), &py_str("."));
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
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&back.clone()), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    break;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_TO_REPEAT(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![said.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) }) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::memory::topics::g_STOPWORDS(), &w.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5))) {
    return Py::Bool(true);
    }
    theirs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::words(args))(&__args) }) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::memory::topics::g_STOPWORDS(), &w.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(w.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&theirs.clone(), "intersection", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::repeat::framed(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__HERE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "startswith", args))(&__args) } } };
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "endswith", args))(&__args) };
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thick_enough(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_figure(args))(&__args) } } }))) {
    return Py::Bool(false);
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_figure(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_question(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thick_enough(args))(&__args) } } } } };
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
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::repeat::g_HEDGE(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDOLENCE(), "match", args))(&__args) }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _after_the_condolence(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &sentence.clone())) } }) {
    sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "rstrip", args))(&__args) }, "endswith", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _meant(args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_ANSWERED()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_figure(args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thick_enough(args))(&__args) })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECEIPT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECEIPT(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _framed(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("repeated")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(sentence.clone()); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _worth_handing_over(args))(&__args) }))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gist(args))(&__args) };
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
    parts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sentences(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "rstrip", args))(&__args) }, "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = asked_what_it_meant.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) } } } }) {
    return geti(&parts.clone(), &op_neg(&Py::Int(1)));
    }
    first = py_str("");
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::repeat::g_HEDGE(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_ANSWERED()) } }) {
    first = sentence.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&first.clone()))) {
    return geti(&parts.clone(), &Py::Int(0));
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DEFINES(), "search", args))(&__args) }))) {
    let __iter2 = py_iter_or_raise(&parts.clone());
    for __item in __iter2 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&sentence.clone(), &first.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::repeat::g_HEDGE(), "search", args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_ANSWERED()); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DEFINES(), "search", args))(&__args) } } }) {
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
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDOLENCE_SENTENCE(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&cut.clone()))) {
    return py_str("");
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
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
    one = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".?! ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "rstrip", args))(&__args) }, "lower", args))(&__args) };
    other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".?! ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = other.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "rstrip", args))(&__args) }, "lower", args))(&__args) };
    let __iter1 = py_iter_or_raise(&op_add(&g_LEADS(), &g_REPEAT_LEADS()));
    for __item in __iter1 {
    lead = __item.clone();
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&other.clone(), "startswith", args))(&__args) }) {
    other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&other.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&other.clone())) } }) {
    return Py::Bool(false);
    }
    let __unpk2 = py_unpack(&if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(other.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { py_tuple(vec![one.clone(), other.clone()]) } else { py_tuple(vec![other.clone(), one.clone()]) }, 2, None);
    short = __unpk2[0].clone();
    long = __unpk2[1].clone();
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(short.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&long.clone(), "startswith", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&long.clone(), &short.clone())) } };
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_since.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cited.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _same_bit(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::again::unframed(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = what_it_meant.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PERSONAL(), "search", args))(&__args) } } }))) {
    return Py::None;
    }
    carried = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _under_the_lead(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&carried.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _personal(args))(&__args) } } }) {
    return Py::None;
    }
    noun = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _their_subject(args))(&__args) };
    if py_truthy(&{ let __b = noun.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(noun.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _on_the_table(args))(&__args) })) } }) {
    return Py::None;
    }
    cited = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(carried.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(carried.clone()); __args.push(what_it_meant.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _meant(args))(&__args) }); __args.push(asked.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cite(args))(&__args) }; if py_truthy(&__b) { __b } else { carried.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&cited.clone()))) {
    return Py::None;
    }
    someone = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_ELSE(), "search", args))(&__args) };
    if py_truthy(&someone.clone()) {
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&someone.clone(), "group", args))(&__args) }, "lower", args))(&__args) }), ("said".to_string(), cited.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_LEADS(), "format", args))(&__args) };
    } else {
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), cited.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PERSONAL_LEADS()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::repeat::handed_back(args))(&__args) };
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", args))(&__args) };
    repaired = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plain")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = repaired.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&repaired.clone(), &clean.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MEAN(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPEAT(), "match", args))(&__args) } } })) } } } }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MEAN(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(repaired.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPEAT(), "match", args))(&__args) } } }) {
    clean = repaired.clone();
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MEAN(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_SOMETHING(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MEAN(), "match", args))(&__args) }) {
    leads = g_LEADS();
    what_it_meant = Py::Bool(true);
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _their_subject(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _their_subject(args))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _on_the_table(args))(&__args) })) } }) {
    return Py::None;
    }
    about = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ABOUT_MINE(), "search", args))(&__args) };
    if py_truthy(&{ let __b = about.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&about.clone(), "group", args))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _on_the_table(args))(&__args) })) } }) {
    return Py::None;
    }
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPEAT(), "match", args))(&__args) }) {
    leads = g_REPEAT_LEADS();
    what_it_meant = Py::Bool(false);
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_plainly(args))(&__args) }) {
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
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(back.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _answers(args))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    said = __unpk3[0].clone();
    asked = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_contains(&said_since.clone(), &said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&applied_once.clone())); if !py_truthy(&__b) { __b } else { { let __b = what_it_meant.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PERSONAL(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _personal(args))(&__args) } } } } } } }) {
    applied_once = Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&applied_once.clone()))) {
    applied = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(said.clone()); __args.push(asked.clone()); __args.push(memory.clone()); __args.push(what_it_meant.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _applied_to_the_person(args))(&__args) };
    if py_truthy(&applied.clone()) {
    return applied.clone();
    }
    }
    carried = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _carried(args))(&__args) };
    if py_truthy(&carried.clone()) {
    cited = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(carried.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(carried.clone()); __args.push(what_it_meant.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _meant(args))(&__args) }); __args.push(asked.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cite(args))(&__args) };
    if py_truthy(&{ let __b = cited.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cited.clone()); __args.push(said_since.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _already(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), cited.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(leads.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_since.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![said.clone(), asked.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lines.clone(), "append", args))(&__args) };
    continue;
    }
    cited = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(what_it_meant.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _meant(args))(&__args) }); __args.push(asked.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cite(args))(&__args) };
    if py_truthy(&{ let __b = cited.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cited.clone()); __args.push(said_since.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _already(args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), cited.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(leads.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_since.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![said.clone(), asked.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lines.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _framed(args))(&__args) } } }) {
    carried = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _under_the_lead(args))(&__args) };
    noun = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _their_subject(args))(&__args) };
    if py_truthy(&{ let __b = what_it_meant.clone(); if !py_truthy(&__b) { __b } else { { let __b = carried.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PERSONAL(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(!py_truthy(&noun.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(noun.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _on_the_table(args))(&__args) } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _personal(args))(&__args) })) } } } } } } } }) {
    applied = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(1))); __args.push(memory.clone()); __args.push(what_it_meant.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _applied_to_the_person(args))(&__args) };
    if py_truthy(&applied.clone()) {
    return applied.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SAID_TWICE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::repeat::handed_back(args))(&__args) };
    }
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDOLENCE(), "match", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NOTHING_WRITTEN()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::repeat::handed_back(args))(&__args) };
    }
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::repeat::g_HEDGE(), "search", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NO_ANSWER_OF_MINE()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::repeat::handed_back(args))(&__args) };
    }
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECEIPT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_figure(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thick_enough(args))(&__args) })); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&geti(&lines.clone(), &Py::Int(0)), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _meant(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_ANSWERED()) } } } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NOTHING_WRITTEN()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::repeat::handed_back(args))(&__args) };
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
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MEAN(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REPEAT(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_plainly(args))(&__args) } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_MINE" => s_ABOUT_MINE(v),
            "ANSWERED" => s_ANSWERED(v),
            "ASKING_ABOUT_DOING" => s_ASKING_ABOUT_DOING(v),
            "ASKS_SOMETHING" => s_ASKS_SOMETHING(v),
            "CLAUSE" => s_CLAUSE(v),
            "CONDOLENCE" => s_CONDOLENCE(v),
            "CONDOLENCE_SENTENCE" => s_CONDOLENCE_SENTENCE(v),
            "DEFINES" => s_DEFINES(v),
            "FOLLOW_UP_WORDS" => s_FOLLOW_UP_WORDS(v),
            "FOR_MINE" => s_FOR_MINE(v),
            "LEADS" => s_LEADS(v),
            "LEAST" => s_LEAST(v),
            "LOOK_BACK" => s_LOOK_BACK(v),
            "LOOK_BACK_FAR" => s_LOOK_BACK_FAR(v),
            "MEAN" => s_MEAN(v),
            "MOST" => s_MOST(v),
            "NOTHING_WRITTEN" => s_NOTHING_WRITTEN(v),
            "NOT_LETTERS" => s_NOT_LETTERS(v),
            "NOT_TO_REPEAT" => s_NOT_TO_REPEAT(v),
            "NO_ANSWER_OF_MINE" => s_NO_ANSWER_OF_MINE(v),
            "OPENERS" => s_OPENERS(v),
            "PERSONAL" => s_PERSONAL(v),
            "PERSONAL_LEADS" => s_PERSONAL_LEADS(v),
            "PUT_IT_PLAINER" => s_PUT_IT_PLAINER(v),
            "RECEIPT" => s_RECEIPT(v),
            "REPEAT" => s_REPEAT(v),
            "REPEAT_LEADS" => s_REPEAT_LEADS(v),
            "SAID_TWICE" => s_SAID_TWICE(v),
            "SENTENCE" => s_SENTENCE(v),
            "SOMEONE_ELSE" => s_SOMEONE_ELSE(v),
            "SOMEONE_LEADS" => s_SOMEONE_LEADS(v),
            "TELLS" => s_TELLS(v),
            "WHAT_YOU_ARE" => s_WHAT_YOU_ARE(v),
            "_HERE" => s__HERE(v),
            "_PERSONAL_HEADS" => s__PERSONAL_HEADS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.meant", module_obj);
    pyrt::xrefs::reg_fn("memory.meant", "_asked_plainly", py_func("_asked_plainly", |args: &[Py]| _asked_plainly(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_personal", py_func("_personal", |args: &[Py]| _personal(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_their_subject", py_func("_their_subject", |args: &[Py]| _their_subject(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_on_the_table", py_func("_on_the_table", |args: &[Py]| _on_the_table(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_under_the_lead", py_func("_under_the_lead", |args: &[Py]| _under_the_lead(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_sentences", py_func("_sentences", |args: &[Py]| _sentences(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_gist", py_func("_gist", |args: &[Py]| _gist(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_answers", py_func("_answers", |args: &[Py]| _answers(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_thick_enough", py_func("_thick_enough", |args: &[Py]| _thick_enough(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_framed", py_func("_framed", |args: &[Py]| _framed(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_a_figure", py_func("_a_figure", |args: &[Py]| _a_figure(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_a_question", py_func("_a_question", |args: &[Py]| _a_question(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_worth_handing_over", py_func("_worth_handing_over", |args: &[Py]| _worth_handing_over(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_cite", py_func("_cite", |args: &[Py]| _cite(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_meant", py_func("_meant", |args: &[Py]| _meant(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_after_the_condolence", py_func("_after_the_condolence", |args: &[Py]| _after_the_condolence(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_same_bit", py_func("_same_bit", |args: &[Py]| _same_bit(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_already", py_func("_already", |args: &[Py]| _already(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_carried", py_func("_carried", |args: &[Py]| _carried(args)));
    pyrt::xrefs::reg_fn("memory.meant", "_applied_to_the_person", py_func("_applied_to_the_person", |args: &[Py]| _applied_to_the_person(args)));
    pyrt::xrefs::reg_fn("memory.meant", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_fn("memory.meant", "reads_it", py_func("reads_it", |args: &[Py]| reads_it(args)));
    pyrt::xrefs::reg_access("memory.meant", "ABOUT_MINE", g_ABOUT_MINE, s_ABOUT_MINE);
    pyrt::xrefs::reg_access("memory.meant", "ANSWERED", g_ANSWERED, s_ANSWERED);
    pyrt::xrefs::reg_access("memory.meant", "ASKING_ABOUT_DOING", g_ASKING_ABOUT_DOING, s_ASKING_ABOUT_DOING);
    pyrt::xrefs::reg_access("memory.meant", "ASKS_SOMETHING", g_ASKS_SOMETHING, s_ASKS_SOMETHING);
    pyrt::xrefs::reg_access("memory.meant", "CLAUSE", g_CLAUSE, s_CLAUSE);
    pyrt::xrefs::reg_access("memory.meant", "CONDOLENCE", g_CONDOLENCE, s_CONDOLENCE);
    pyrt::xrefs::reg_access("memory.meant", "CONDOLENCE_SENTENCE", g_CONDOLENCE_SENTENCE, s_CONDOLENCE_SENTENCE);
    pyrt::xrefs::reg_access("memory.meant", "DEFINES", g_DEFINES, s_DEFINES);
    pyrt::xrefs::reg_access("memory.meant", "FOLLOW_UP_WORDS", g_FOLLOW_UP_WORDS, s_FOLLOW_UP_WORDS);
    pyrt::xrefs::reg_access("memory.meant", "FOR_MINE", g_FOR_MINE, s_FOR_MINE);
    pyrt::xrefs::reg_access("memory.meant", "LEADS", g_LEADS, s_LEADS);
    pyrt::xrefs::reg_access("memory.meant", "LEAST", g_LEAST, s_LEAST);
    pyrt::xrefs::reg_access("memory.meant", "LOOK_BACK", g_LOOK_BACK, s_LOOK_BACK);
    pyrt::xrefs::reg_access("memory.meant", "LOOK_BACK_FAR", g_LOOK_BACK_FAR, s_LOOK_BACK_FAR);
    pyrt::xrefs::reg_access("memory.meant", "MEAN", g_MEAN, s_MEAN);
    pyrt::xrefs::reg_access("memory.meant", "MOST", g_MOST, s_MOST);
    pyrt::xrefs::reg_access("memory.meant", "NOTHING_WRITTEN", g_NOTHING_WRITTEN, s_NOTHING_WRITTEN);
    pyrt::xrefs::reg_access("memory.meant", "NOT_LETTERS", g_NOT_LETTERS, s_NOT_LETTERS);
    pyrt::xrefs::reg_access("memory.meant", "NOT_TO_REPEAT", g_NOT_TO_REPEAT, s_NOT_TO_REPEAT);
    pyrt::xrefs::reg_access("memory.meant", "NO_ANSWER_OF_MINE", g_NO_ANSWER_OF_MINE, s_NO_ANSWER_OF_MINE);
    pyrt::xrefs::reg_access("memory.meant", "OPENERS", g_OPENERS, s_OPENERS);
    pyrt::xrefs::reg_access("memory.meant", "PERSONAL", g_PERSONAL, s_PERSONAL);
    pyrt::xrefs::reg_access("memory.meant", "PERSONAL_LEADS", g_PERSONAL_LEADS, s_PERSONAL_LEADS);
    pyrt::xrefs::reg_access("memory.meant", "PUT_IT_PLAINER", g_PUT_IT_PLAINER, s_PUT_IT_PLAINER);
    pyrt::xrefs::reg_access("memory.meant", "RECEIPT", g_RECEIPT, s_RECEIPT);
    pyrt::xrefs::reg_access("memory.meant", "REPEAT", g_REPEAT, s_REPEAT);
    pyrt::xrefs::reg_access("memory.meant", "REPEAT_LEADS", g_REPEAT_LEADS, s_REPEAT_LEADS);
    pyrt::xrefs::reg_access("memory.meant", "SAID_TWICE", g_SAID_TWICE, s_SAID_TWICE);
    pyrt::xrefs::reg_access("memory.meant", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("memory.meant", "SOMEONE_ELSE", g_SOMEONE_ELSE, s_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("memory.meant", "SOMEONE_LEADS", g_SOMEONE_LEADS, s_SOMEONE_LEADS);
    pyrt::xrefs::reg_access("memory.meant", "TELLS", g_TELLS, s_TELLS);
    pyrt::xrefs::reg_access("memory.meant", "WHAT_YOU_ARE", g_WHAT_YOU_ARE, s_WHAT_YOU_ARE);
    pyrt::xrefs::reg_access("memory.meant", "_HERE", g__HERE, s__HERE);
    pyrt::xrefs::reg_access("memory.meant", "_PERSONAL_HEADS", g__PERSONAL_HEADS, s__PERSONAL_HEADS);
}
