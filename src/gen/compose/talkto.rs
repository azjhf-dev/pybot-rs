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


pub static BLANK_SUBJECTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INVITED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POOL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POOL_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALK_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEIGHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEIGHING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BARE_INVITATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_LEAD_IN_FRONT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_TIME_BEFORE_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CLAUSE_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HEDGED_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OWNERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POLITE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _REPEATED_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SOMETHING_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SUBJECT_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SUBJECT_END_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SUBJECT_HEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SUBJECT_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TALK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TYPED_SHORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TYPED_SHORT_DICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WHEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_BLANK_SUBJECTS() -> Py { ensure_init(); BLANK_SUBJECTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BLANK_SUBJECTS(v: Py) { ensure_init(); if let Ok(mut w) = BLANK_SUBJECTS.write() { *w = v; } }
pub fn g_INVITED() -> Py { ensure_init(); INVITED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INVITED(v: Py) { ensure_init(); if let Ok(mut w) = INVITED.write() { *w = v; } }
pub fn g_POOL() -> Py { ensure_init(); POOL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POOL(v: Py) { ensure_init(); if let Ok(mut w) = POOL.write() { *w = v; } }
pub fn g_POOL_CLAUSE() -> Py { ensure_init(); POOL_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POOL_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = POOL_CLAUSE.write() { *w = v; } }
pub fn g_TALK_ASKS() -> Py { ensure_init(); TALK_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALK_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = TALK_ASKS.write() { *w = v; } }
pub fn g_WEIGHING() -> Py { ensure_init(); WEIGHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEIGHING(v: Py) { ensure_init(); if let Ok(mut w) = WEIGHING.write() { *w = v; } }
pub fn g_WEIGHING_WORDS() -> Py { ensure_init(); WEIGHING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEIGHING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = WEIGHING_WORDS.write() { *w = v; } }
pub fn g__A_BARE_INVITATION() -> Py { ensure_init(); _A_BARE_INVITATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BARE_INVITATION(v: Py) { ensure_init(); if let Ok(mut w) = _A_BARE_INVITATION.write() { *w = v; } }
pub fn g__A_LEAD_IN_FRONT() -> Py { ensure_init(); _A_LEAD_IN_FRONT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_LEAD_IN_FRONT(v: Py) { ensure_init(); if let Ok(mut w) = _A_LEAD_IN_FRONT.write() { *w = v; } }
pub fn g__A_TIME_BEFORE_THE_THING() -> Py { ensure_init(); _A_TIME_BEFORE_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_TIME_BEFORE_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = _A_TIME_BEFORE_THE_THING.write() { *w = v; } }
pub fn g__CLAUSE_START() -> Py { ensure_init(); _CLAUSE_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CLAUSE_START(v: Py) { ensure_init(); if let Ok(mut w) = _CLAUSE_START.write() { *w = v; } }
pub fn g__HEDGED_END() -> Py { ensure_init(); _HEDGED_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HEDGED_END(v: Py) { ensure_init(); if let Ok(mut w) = _HEDGED_END.write() { *w = v; } }
pub fn g__LEAD() -> Py { ensure_init(); _LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _LEAD.write() { *w = v; } }
pub fn g__OWNERS() -> Py { ensure_init(); _OWNERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OWNERS(v: Py) { ensure_init(); if let Ok(mut w) = _OWNERS.write() { *w = v; } }
pub fn g__POLITE() -> Py { ensure_init(); _POLITE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POLITE(v: Py) { ensure_init(); if let Ok(mut w) = _POLITE.write() { *w = v; } }
pub fn g__REPEATED_WORD() -> Py { ensure_init(); _REPEATED_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__REPEATED_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _REPEATED_WORD.write() { *w = v; } }
pub fn g__SOMETHING_ELSE() -> Py { ensure_init(); _SOMETHING_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SOMETHING_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = _SOMETHING_ELSE.write() { *w = v; } }
pub fn g__SUBJECT_END() -> Py { ensure_init(); _SUBJECT_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_END(v: Py) { ensure_init(); if let Ok(mut w) = _SUBJECT_END.write() { *w = v; } }
pub fn g__SUBJECT_END_CLAUSE() -> Py { ensure_init(); _SUBJECT_END_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_END_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = _SUBJECT_END_CLAUSE.write() { *w = v; } }
pub fn g__SUBJECT_HEAD() -> Py { ensure_init(); _SUBJECT_HEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_HEAD(v: Py) { ensure_init(); if let Ok(mut w) = _SUBJECT_HEAD.write() { *w = v; } }
pub fn g__SUBJECT_TAIL() -> Py { ensure_init(); _SUBJECT_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = _SUBJECT_TAIL.write() { *w = v; } }
pub fn g__TALK() -> Py { ensure_init(); _TALK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TALK(v: Py) { ensure_init(); if let Ok(mut w) = _TALK.write() { *w = v; } }
pub fn g__TYPED_SHORT() -> Py { ensure_init(); _TYPED_SHORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TYPED_SHORT(v: Py) { ensure_init(); if let Ok(mut w) = _TYPED_SHORT.write() { *w = v; } }
pub fn g__TYPED_SHORT_DICT() -> Py { ensure_init(); _TYPED_SHORT_DICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TYPED_SHORT_DICT(v: Py) { ensure_init(); if let Ok(mut w) = _TYPED_SHORT_DICT.write() { *w = v; } }
pub fn g__VERB() -> Py { ensure_init(); _VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__VERB(v: Py) { ensure_init(); if let Ok(mut w) = _VERB.write() { *w = v; } }
pub fn g__WHEN() -> Py { ensure_init(); _WHEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WHEN(v: Py) { ensure_init(); if let Ok(mut w) = _WHEN.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Somebody asks this bot for room to talk about a thing of theirs, and names the thing.\n\n\"can you talk to me about my anxiety\" got the search hedge (\"I don't know enough to answer that\nhonestly. A search should sort you out quickly\") - which is no answer at all: nobody wants a search\nengine for that. The bot can do the one thing asked for, which is talk with them, and the honest\nreply is a yes in a few words and the one question that starts it: STYLE.md's \"Not knowing,\nhonestly\" and its one short question in the same breath.\n\nThe ask is read, not listed: \"can you talk to me about X\", \"can we talk about X\", \"can i talk to\nyou about X\", \"i want / need / would like to talk about X\", \"talk to me about X\", and the same\nframes with \"think\" and \"work\" (\"can you help me think about my options\"). The thing named is said\nback in the person's own words with the possessive turned round (\"my job\" is \"your job\"), because\nthe thing named is the thing they want to talk about.\n\nNarrow on purpose. It wants the bot addressed - a person in the line asking for the bot's ear - so\n\"he wont talk to me about it\" and \"my daughter wont talk about it\" are somebody else's refusal and\nkeep the reply they had. It only ever replaces one of the composer's own shrugs, so a card that\nanswers the line stands, and a crisis line never reaches here.\n");
    s__LEAD(py_str("(?:(?:ok|okay|so|right|hi|hey|well)\\s+)?"));
    s__POLITE(py_str("(?:(?:please|just)\\s+)?"));
    s__TALK(py_str("(?:talk|chat|speak)"));
    s__VERB(py_str("(?:want|wanna|need|needed|would\\s+like|'?d\\s+like|have\\s+to|got\\s+to|gotta|have|got)"));
    s_TALK_ASKS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![add(&add(&add(&py_str("(?:can|could|would|will)\\s+you\\s+"), &g__POLITE()), &g__TALK()), &py_str("\\s+(?:to|with)\\s+me\\s+(?:about|through)\\s+(?P<subject>.+)$")), add(&add(&py_str("(?:can|could|may)\\s+(?:i|we)\\s+(?:please\\s+)?"), &g__TALK()), &py_str("(?:\\s+(?:to|with)\\s+you)?\\s+(?:about|through)\\s+(?P<subject>.+)$")), add(&add(&add(&add(&py_str("(?:i|we)\\s+(?:just\\s+|really\\s+|honestly\\s+)?"), &g__VERB()), &py_str("\\s+(?:to\\s+)?")), &g__TALK()), &py_str("(?:\\s+(?:to|with)\\s+you)?\\s+(?:about|through)\\s+(?P<subject>.+)$")), add(&add(&g__POLITE(), &g__TALK()), &py_str("\\s+to\\s+me\\s+(?:about|through)\\s+(?P<subject>.+)$")), add(&add(&py_str("(?:can|could|would|will)\\s+you\\s+"), &g__POLITE()), &py_str("help\\s+me\\s+(?:think|talk|work)\\s+(?:about|through|out)\\s+(?P<subject>.+)$")), add(&add(&py_str("(?:i|we)\\s+(?:just\\s+|really\\s+)?"), &g__VERB()), &py_str("\\s+(?:to\\s+)?(?:think|work)\\s+(?:about|through|out)\\s+(?P<subject>.+)$"))])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[add(&g__LEAD(), &pattern.clone())]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_BLANK_SUBJECTS(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("them"), py_str("something"), py_str("somethin"), py_str("anything"), py_str("everything"), py_str("stuff"), py_str("a thing"), py_str("one thing"), py_str("it all"), py_str("something else")])]).unwrap_or_else(|e| exc::raise(e)));
    s__SUBJECT_TAIL(stdlib_call("re", "compile", &[py_str("\\s+(?:with|to)\\s+(?:you|me)\\s*$|\\s+(?:again|please|now|today|tonight|later|tomorrow|sometime|soon|this\\s+(?:evening|afternoon|morning|weekend)|next\\s+week)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__WHEN(py_str("(?:again|now|today|tonight|tomorrow|later|sometime|soon|this\\s+(?:evening|afternoon|morning|weekend)|next\\s+week|when\\s+(?:you\\s+)?(?:have|get|'ve\\s+got)\\s+a\\s+(?:minute|mo|moment))"));
    s__A_TIME_BEFORE_THE_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(talk|chat|speak|think|work)\\s+(?:to\\s+(?:you|me)\\s+)?(?:"), &g__WHEN()), &py_str("\\s+)+(?=(?:about|through|out)\\b)"))]).unwrap_or_else(|e| exc::raise(e)));
    s__SUBJECT_HEAD(stdlib_call("re", "compile", &[py_str("^(?:the|that|this)\\s+thing\\s+(?:about|with)\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s__SOMETHING_ELSE(stdlib_call("re", "compile", &[py_str("^(?:something|anything|somebody|anybody|a\\s+different\\s+thing|something\\s+different|another\\s+thing|anything\\s+else\\s+at\\s+all)\\s+else\\s*[,;:\\-\\u2013\\u2014]?\\s*")]).unwrap_or_else(|e| exc::raise(e)));
    s__SUBJECT_END(stdlib_call("re", "compile", &[py_str("\\s+(?:its|it's|it is|it was|it's|and|but|because|cos|cuz|coz|so|which|that|who|i|ive|i've|we|were|we're|he|she|they|my|our)\\b.*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__CLAUSE_START(stdlib_call("re", "compile", &[py_str("^(?:how|what|why|whether|if|when|where|which|who|whoever|whatever)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SUBJECT_END_CLAUSE(stdlib_call("re", "compile", &[py_str("\\s+(?:its|it's|it is|it was|and|but|because|cos|cuz|coz|which|that)\\b.*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__HEDGED_END(stdlib_call("re", "compile", &[py_str("\\s*,?\\s*(?:maybe|perhaps|i\\s+think|i\\s+reckon|if\\s+that\\s+is\\s+ok)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__OWNERS(py_tuple(vec![py_tuple(vec![py_str("my"), py_str("your")]), py_tuple(vec![py_str("our"), py_str("your")])]));
    s__A_BARE_INVITATION(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&add(&g__LEAD(), &py_str("(?:(?:can|could|may|shall|should)\\s+(?:i|we)\\s+")), &g__POLITE()), &py_str("|(?:i|we)\\s+")), &g__VERB()), &py_str("\\s+(?:to\\s+)?|")), &g__POLITE()), &py_str(")(?:talk|chat|speak)(?:\\s+(?:to|with)\\s+(?:you|me))?(?:\\s+")), &g__WHEN()), &py_str(")+\\s*[?.!]*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_LEAD_IN_FRONT(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&g__LEAD(), &py_str("(?:(?:can|could|may|shall|should)\\s+(?:i|we)\\s+")), &g__POLITE()), &py_str("|(?:i|we)\\s+")), &g__VERB()), &py_str("\\s+(?:to\\s+)?)(?:talk|chat|speak)"))]).unwrap_or_else(|e| exc::raise(e)));
    s_INVITED(py_tuple(vec![py_str("Yes - I'm here whenever you are. What's on your mind?"), py_str("Any time. What is it you want to talk about?"), py_str("Of course. What's going on, and where do you want to start?")]));
    s_POOL(py_tuple(vec![py_str("Of course. Tell me about {thing} - what's the part that's bothering you most?"), py_str("{Thing} it is. What's on your mind about it?"), py_str("Happy to. Where do you want to start with {thing}?"), py_str("Right, {thing} - go on. What's the bit you want to think out loud about?")]));
    s_POOL_CLAUSE(py_tuple(vec![geti(&g_POOL(), &Py::Int(0)), geti(&g_POOL(), &Py::Int(3))]));
    s_WEIGHING(py_tuple(vec![py_str("Yes. What are the options you're weighing, as they stand now?"), py_str("Let's lay them out. What's on each side of it?"), py_str("Of course. What would you lose with each one?")]));
    s_WEIGHING_WORDS(call_builtin("frozenset", &[mcall(&py_str("\noptions option choice choices decision decisions pros cons\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__TYPED_SHORT(py_tuple(vec![py_tuple(vec![py_str("u"), py_str("you")]), py_tuple(vec![py_str("ya"), py_str("you")]), py_tuple(vec![py_str("ur"), py_str("your")]), py_tuple(vec![py_str("yr"), py_str("your")]), py_tuple(vec![py_str("wana"), py_str("want")]), py_tuple(vec![py_str("wanna"), py_str("want")]), py_tuple(vec![py_str("wanta"), py_str("want")])]));
    s__REPEATED_WORD(stdlib_call("re", "compile", &[py_str("\\b(\\w+)(\\s+\\1\\b)+")]).unwrap_or_else(|e| exc::raise(e)));
    s__TYPED_SHORT_DICT(call_builtin("dict", &[g__TYPED_SHORT()]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _as_typed(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with the chat-speak the frames use spelled out and a doubled word collapsed.");
    line = mcall(&g__REPEATED_WORD(), "sub", &[py_str("\\1"), line.clone()]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&line.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&g__TYPED_SHORT_DICT(), "get", &[word.clone(), word.clone()])); } py_list(__out) })() };
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn the_line_as_read(args: &[Py]) -> Py {
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
    let _ = py_str("The line the frames are matched against: chat-speak spelled out, a named day dropped.\n\n    `bot/modules/talkto.py` looks the line up in the knowledge base to see whether a card answers\n    the thing named, and it has to ask about the same line this reader read - otherwise a day in\n    the ask (\"can we talk tomorrow about my mum\") makes the lookup miss the carers card and this\n    module takes a turn a card answers better.\n    ");
    return mcall(&g__A_TIME_BEFORE_THE_THING(), "sub", &[py_str("\\1 "), _as_typed(&[text.clone()])]);
    Py::None
}
pub fn _subject(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing named after the talk frame, or \"\" when none is named.");
    text = the_line_as_read(&[text.clone()]);
    let __iter1 = py_iter_or_raise(&g_TALK_ASKS());
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    subject = add(&py_str(" "), &mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("subject")]), "strip", &[]), "rstrip", &[py_str(".!?,")]));
    subject = mcall(&mcall(&g__SUBJECT_TAIL(), "sub", &[py_str(""), subject.clone()]), "strip", &[]);
    subject = mcall(&mcall(&g__SUBJECT_HEAD(), "sub", &[py_str(""), subject.clone()]), "strip", &[]);
    subject = mcall(&mcall(&g__SOMETHING_ELSE(), "sub", &[py_str(""), subject.clone()]), "strip", &[]);
    clause = call_builtin("bool", &[mcall(&g__CLAUSE_START(), "match", &[subject.clone()])]).unwrap_or_else(|e| exc::raise(e));
    subject = mcall(&mcall(&if py_truthy(&clause.clone()) { g__SUBJECT_END_CLAUSE() } else { g__SUBJECT_END() }, "sub", &[py_str(""), subject.clone()]), "strip", &[]);
    subject = mcall(&mcall(&g__HEDGED_END(), "sub", &[py_str(""), subject.clone()]), "strip", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_BLANK_SUBJECTS(), &subject.clone()))) {
    return py_str("");
    }
    words = mcall(&subject.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &if py_truthy(&clause.clone()) { Py::Int(9) } else { Py::Int(5) }) } }) {
    return py_str("");
    }
    let __iter2 = py_iter_or_raise(&g__OWNERS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    mine = __unpk3[0].clone();
    theirs = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &mine.clone()))) {
    let _ = py_setitem(&words.clone(), &Py::Int(0), theirs.clone());
    break;
    }
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    }
    return py_str("");
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
    let mut frames: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut one: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut typed: Py = Py::None;
    let _ = py_str("A yes and the one question that starts it, or \"\" when the line is not this ask.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_str("");
    }
    subject = _subject(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    typed = _as_typed(&[line.clone()]);
    if py_truthy(&{ let __b = mcall(&g__A_BARE_INVITATION(), "match", &[typed.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_LEAD_IN_FRONT(), "match", &[typed.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&typed.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } } } }) {
    return crate::gen::compose::pick(&[g_INVITED(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, text.clone()]);
    }
    return py_str("");
    }
    if py_truthy(&bitand(&call_builtin("set", &[mcall(&subject.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_WEIGHING_WORDS())) {
    return crate::gen::compose::pick(&[g_WEIGHING(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, text.clone()]);
    }
    frames = if py_truthy(&mcall(&g__CLAUSE_START(), "match", &[subject.clone()])) { g_POOL_CLAUSE() } else { g_POOL() };
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut one: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), subject.clone()), ("Thing".to_string(), add(&mcall(&slice_or_raise(&subject.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&subject.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    return crate::gen::compose::pick(&[lines.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, text.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.talkto".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BLANK_SUBJECTS" => Some(g_BLANK_SUBJECTS()),
            "INVITED" => Some(g_INVITED()),
            "POOL" => Some(g_POOL()),
            "POOL_CLAUSE" => Some(g_POOL_CLAUSE()),
            "TALK_ASKS" => Some(g_TALK_ASKS()),
            "WEIGHING" => Some(g_WEIGHING()),
            "WEIGHING_WORDS" => Some(g_WEIGHING_WORDS()),
            "_A_BARE_INVITATION" => Some(g__A_BARE_INVITATION()),
            "_A_LEAD_IN_FRONT" => Some(g__A_LEAD_IN_FRONT()),
            "_A_TIME_BEFORE_THE_THING" => Some(g__A_TIME_BEFORE_THE_THING()),
            "_CLAUSE_START" => Some(g__CLAUSE_START()),
            "_HEDGED_END" => Some(g__HEDGED_END()),
            "_LEAD" => Some(g__LEAD()),
            "_OWNERS" => Some(g__OWNERS()),
            "_POLITE" => Some(g__POLITE()),
            "_REPEATED_WORD" => Some(g__REPEATED_WORD()),
            "_SOMETHING_ELSE" => Some(g__SOMETHING_ELSE()),
            "_SUBJECT_END" => Some(g__SUBJECT_END()),
            "_SUBJECT_END_CLAUSE" => Some(g__SUBJECT_END_CLAUSE()),
            "_SUBJECT_HEAD" => Some(g__SUBJECT_HEAD()),
            "_SUBJECT_TAIL" => Some(g__SUBJECT_TAIL()),
            "_TALK" => Some(g__TALK()),
            "_TYPED_SHORT" => Some(g__TYPED_SHORT()),
            "_TYPED_SHORT_DICT" => Some(g__TYPED_SHORT_DICT()),
            "_VERB" => Some(g__VERB()),
            "_WHEN" => Some(g__WHEN()),
            "_as_typed" => Some(py_func("_as_typed", |args: &[Py]| _as_typed(args))),
            "_subject" => Some(py_func("_subject", |args: &[Py]| _subject(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "the_line_as_read" => Some(py_func("the_line_as_read", |args: &[Py]| the_line_as_read(args))),
            _ => None,
        }),
        set: None,
    }))
}
