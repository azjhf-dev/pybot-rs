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


pub static ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TIMED_KIND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAIM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAND_BACKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONTHS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_PLAIN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_RECALL_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRONOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNITS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VALUE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASKS() -> Py { ensure_init(); ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS.write() { *w = v; } }
pub fn g_A_RATE() -> Py { ensure_init(); A_RATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RATE(v: Py) { ensure_init(); if let Ok(mut w) = A_RATE.write() { *w = v; } }
pub fn g_A_TIMED_KIND() -> Py { ensure_init(); A_TIMED_KIND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TIMED_KIND(v: Py) { ensure_init(); if let Ok(mut w) = A_TIMED_KIND.write() { *w = v; } }
pub fn g_CLAIM() -> Py { ensure_init(); CLAIM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAIM(v: Py) { ensure_init(); if let Ok(mut w) = CLAIM.write() { *w = v; } }
pub fn g_DAYS() -> Py { ensure_init(); DAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAYS(v: Py) { ensure_init(); if let Ok(mut w) = DAYS.write() { *w = v; } }
pub fn g_HAND_BACK() -> Py { ensure_init(); HAND_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_BACK(v: Py) { ensure_init(); if let Ok(mut w) = HAND_BACK.write() { *w = v; } }
pub fn g_HAND_BACKS() -> Py { ensure_init(); HAND_BACKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAND_BACKS(v: Py) { ensure_init(); if let Ok(mut w) = HAND_BACKS.write() { *w = v; } }
pub fn g_KINDS() -> Py { ensure_init(); KINDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS(v: Py) { ensure_init(); if let Ok(mut w) = KINDS.write() { *w = v; } }
pub fn g_MONTHS() -> Py { ensure_init(); MONTHS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); if let Ok(mut w) = MONTHS.write() { *w = v; } }
pub fn g_NOT_A_PLAIN_ASK() -> Py { ensure_init(); NOT_A_PLAIN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PLAIN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_PLAIN_ASK.write() { *w = v; } }
pub fn g_NOT_A_RECALL_ASK() -> Py { ensure_init(); NOT_A_RECALL_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_RECALL_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_RECALL_ASK.write() { *w = v; } }
pub fn g_PRONOUNS() -> Py { ensure_init(); PRONOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRONOUNS(v: Py) { ensure_init(); if let Ok(mut w) = PRONOUNS.write() { *w = v; } }
pub fn g_QUESTION() -> Py { ensure_init(); QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION.write() { *w = v; } }
pub fn g_RECALL() -> Py { ensure_init(); RECALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL(v: Py) { ensure_init(); if let Ok(mut w) = RECALL.write() { *w = v; } }
pub fn g_STOP() -> Py { ensure_init(); STOP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); if let Ok(mut w) = STOP.write() { *w = v; } }
pub fn g_THEIR_OWN_WORDS() -> Py { ensure_init(); THEIR_OWN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN_WORDS.write() { *w = v; } }
pub fn g_UNITS() -> Py { ensure_init(); UNITS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNITS(v: Py) { ensure_init(); if let Ok(mut w) = UNITS.write() { *w = v; } }
pub fn g_VALUE_WORDS() -> Py { ensure_init(); VALUE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VALUE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = VALUE_WORDS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("One check for what the conversation already holds: is there a line of theirs that carried it?\n\nTwo things in this bot depend on the same question, and both used to answer it anew each time it\ncame up:\n\n* a reply that claims the person has not said something (\"You haven't told me how old she is\"),\n  and\n* a recall ask (\"how old did i say she was\", \"what size did i say\") that has to find the value\n  in the turns behind it.\n\nThe claim is the worse of the two. `shes 7 going on 8` was answered \"Got it\", and a turn later\n\"how old did i say she was\" was answered \"You haven't told me how old she is\" - a flat\ncontradiction of the line above it. `mtl-26` gave `shes called jane and shes marrying tom` and\nwas later told \"You haven't told me your sister's name\". The reader for the slot had found\nnothing, and nothing checked whether the chat held a word for it before saying it did not.\n\nSo the check lives here, once: `carried()` answers \"which of their lines has a value of this\nkind\", and `put_right()` is the one gate every store reply goes through - if the reply denies a\nthing the chat says, the line is handed back instead of the denial. The kinds are data\n(`KINDS`), so a new kind is a line here rather than a rule in each module.\n\nThe subject matters only when the claim names one. \"You haven't told me how old your daughter is\"\nhas to find a line about the daughter; \"how old did i say she was\" is a pronoun, and a pronoun\npoints at the line just said, so the latest line of that kind is the one it means.\n");
    s_MONTHS(py_str("january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|sept|oct|nov|dec"));
    s_DAYS(py_str("monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|tonight|today"));
    s_UNITS(py_str("kg|g|l|ml|kilo|kilos|kilogram|kilograms|gram|grams|lb|lbs|pound|pounds|ounce|ounces|cm|mm|metre|metres|meter|meters|foot|feet|inch|inches|litre|litres|pint|pints"));
    s_KINDS(py_dict(vec![(py_str("age"), stdlib_call("re", "compile", &[py_str("(?<!year )(?<!size )\\b\\d{1,3}\\b|\\b(?:\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)\\s*(?:years? old|years?|yrs?|yo)\\b|\\baged?\\s+\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("name"), stdlib_call("re", "compile", &[py_str("\\b(?:called|named|calls|name(?:'s| is| was)?)\\s+[a-z]"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("when"), stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:"), &g_MONTHS()), &py_str("|")), &g_DAYS()), &py_str("|\\d{1,2}(?:st|nd|rd|th)\\b|\\d{1,2}(?:[:.]\\d{2})?\\s*(?:am|pm)\\b|\\d{1,2}[:.]\\d{2})\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("size"), stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:\\d+(?:\\.\\d+)?\\s*(?:"), &g_UNITS()), &py_str(")|size\\s+\\d+|\\d+\\s*(?:by|x)\\s*\\d+)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("often"), stdlib_call("re", "compile", &[py_str("\\b(?:\\d+\\s+times|once|twice|three times|every\\s+(?:day|week|month|morning|night|year)|a\\s+(?:day|week|month)|most\\s+(?:days|nights|weeks)|alternate)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("money"), stdlib_call("re", "compile", &[py_str("£\\s*\\d|\\b\\d+(?:,\\d{3})*\\s*(?:pounds?|quid|k\\b|grand)|\\bbudget\\b|\\b\\d{3,}\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("duration"), stdlib_call("re", "compile", &[py_str("\\b(?:\\d+(?:\\.\\d+)?|an?|one|two|three|four|five|six|seven|eight|nine|ten|twelve|fifteen|twenty|half an)\\s*(?:hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("count"), stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+(?:(?:of|in)\\s+)?(?:[a-z]{2,}s|people|children|men|women|us|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("place"), stdlib_call("re", "compile", &[py_str("\\b(?:in|at|to|from|live|living|moved)\\s+[A-Za-z][a-z]{2,}"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]));
    s_ASKS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bhow old\\b|\\bage\\b|\\bhow many years\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("age")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s| is| was)?\\s+(?:it |they |he |she )?called\\b|\\bname\\b|\\bcalled\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("name")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bwhen\\b|\\bwhat day\\b|\\bwhat date\\b|\\bwhat time\\b|\\bwhich day\\b|\\bwhat month\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("when")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bhow often\\b|\\bhow many times\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("often")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bsize\\b|\\bhow big\\b|\\bhow tall\\b|\\bhow heavy\\b|\\bhow long is\\b|\\bhow wide\\b|\\bhow deep\\b|\\bhow far\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("size")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bhow much\\b|\\bbudget\\b|\\bhow much did i\\b|\\bwhat did i say i (?:paid|spent)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("money")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bhow long\\b|\\bhow many (?:days|weeks|months|years|hours|minutes)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("duration")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bhow many\\b|\\bhow many of\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("count")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bwhere\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("place")])]));
    s_CLAIM(stdlib_call("re", "compile", &[py_str("\\b(?:you (?:haven'?t|have not) (?:told|said|mentioned|given)|you'?ve not (?:said|told|mentioned)|you never (?:said|told|mentioned)|i don'?t have (?:your|the|their|his|her)\\b[^.]*\\bfrom you|i (?:do not|don'?t) know (?:what|how|when|where|who)\\b[^.]*\\bsaid)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VALUE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("heavy"), py_str("weigh"), py_str("weighs"), py_str("weight"), py_str("big"), py_str("tall"), py_str("height"), py_str("wide"), py_str("deep"), py_str("far"), py_str("long"), py_str("length"), py_str("size"), py_str("old"), py_str("age"), py_str("aged"), py_str("name"), py_str("called"), py_str("date"), py_str("day"), py_str("month"), py_str("time"), py_str("often"), py_str("times"), py_str("cost"), py_str("costs"), py_str("price"), py_str("paid"), py_str("spend"), py_str("spent"), py_str("much"), py_str("many"), py_str("number"), py_str("count"), py_str("amount"), py_str("budget"), py_str("how"), py_str("what")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUNS(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("me"), py_str("my"), py_str("we"), py_str("us"), py_str("our"), py_str("you"), py_str("your"), py_str("he"), py_str("him"), py_str("his"), py_str("she"), py_str("her"), py_str("they"), py_str("them"), py_str("their"), py_str("it"), py_str("its"), py_str("the"), py_str("a"), py_str("an"), py_str("that"), py_str("this"), py_str("one"), py_str("there")])]).unwrap_or_else(|e| exc::raise(e)));
    s_STOP(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("and"), py_str("or"), py_str("but"), py_str("it"), py_str("its"), py_str("is"), py_str("was"), py_str("were"), py_str("are"), py_str("am"), py_str("be"), py_str("to"), py_str("of"), py_str("for"), py_str("in"), py_str("on"), py_str("at"), py_str("by"), py_str("with"), py_str("about"), py_str("again"), py_str("said"), py_str("say"), py_str("saying"), py_str("told"), py_str("tell"), py_str("you"), py_str("i"), py_str("me"), py_str("my"), py_str("have"), py_str("has"), py_str("had"), py_str("do"), py_str("did"), py_str("does"), py_str("not"), py_str("what"), py_str("when"), py_str("where"), py_str("which"), py_str("who"), py_str("how"), py_str("many"), py_str("much"), py_str("old"), py_str("size"), py_str("long"), py_str("often"), py_str("times"), py_str("time"), py_str("that"), py_str("this"), py_str("your"), py_str("yours"), py_str("called"), py_str("name"), py_str("names"), py_str("remember"), py_str("remind"), py_str("know"), py_str("think"), py_str("get"), py_str("got"), py_str("there"), py_str("their"), py_str("his"), py_str("her"), py_str("they"), py_str("them"), py_str("we"), py_str("our"), py_str("us"), py_str("one"), py_str("ones"), py_str("haven"), py_str("havent"), py_str("isnt"), py_str("wasnt"), py_str("didnt"), py_str("dont"), py_str("doesnt"), py_str("never"), py_str("mentioned"), py_str("given"), py_str("should"), py_str("would"), py_str("could"), py_str("will"), py_str("shall"), py_str("can"), py_str("any"), py_str("some"), py_str("just"), py_str("so"), py_str("if"), py_str("then"), py_str("than"), py_str("as"), py_str("from"), py_str("out"), py_str("up"), py_str("down")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_BACK(py_tuple(vec![py_str("You said: \"{line}\"."), py_str("That one you did say: \"{line}\"."), py_str("You told me: \"{line}\".")]));
    s_A_RATE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d[\\d,.]*|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s*(?:times?\\s*)?(?:a|an|per|/|each|every)\\s*(?:month|week|year|day|hour)\\b|£?\\s*\\d[\\d,.]*\\s*(?:pounds?|quid|grand|k)?\\s*(?:a|an|per|/|each|every)\\s*(?:month|week|year|day|hour)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIMED_KIND(py_tuple(vec![py_str("duration"), py_str("count")]));
    s_RECALL(stdlib_call("re", "compile", &[py_str("\\b(?:did i (?:say|tell|mention)|what did i say|what was it i said|remind me|what did we say|how did i say|again)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAND_BACKS(stdlib_call("re", "compile", &[py_str("^(?:You said|That one you did say|You told me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|so|but|or|oh|also|then)?\\s*(?:how|what|when|where|which|who|whose|why|do|does|did|is|are|was|were|am|can|could|should|would|will|shall|have|has|had|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:(?:did|do|does|have|had|was|were) i (?:say|tell|mention|said|go|use|have|had|get|got|buy|bought|pay|paid|book|pick|choose)|remind me|what was my|what did i|where did (?:i|we) say|again)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_RECALL_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:should|could|would|will|shall|might|can|may|roughly|maybe|probably|need|needs|got to|have to|havent got|is it worth|better off|do you think|any (?:idea|chance)|recommend)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_PLAIN_ASK(py_tuple(vec![py_str("when"), py_str("duration")]));
}
pub fn _a_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("A word that can be the subject of a claim: not a pronoun, not a verb of saying, and not a\n    contraction (\"haven't\" is the claim itself, not a thing it is about).");
    return { let __b = Py::Bool(!py_contains(&g_STOP(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_PRONOUNS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_VALUE_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&word.clone(), &py_str("'"))) } } } } } } } };
    Py::None
}
pub fn _turns(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    turns = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn said_lines(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut number: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What the person said, oldest first, as (turn number, line).\n\n    The turn being answered is not in `memory[\"turns\"]` yet, so this is the conversation so far.\n    Numbered from one because that is how the deck and the person talk about turns.\n\n    A question of theirs is not a line that says anything: asked `do i have a brother` after\n    `do i have any brothers or sisters`, handing the question back is worse than the store's\n    honest \"you haven't told me about a brother yet\". So an ask they typed is skipped here -\n    the bot's reply to it is a line they can be answered from, not their own question.\n    ");
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[_turns(&[memory.clone()]), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    number = __unpk2[0].clone();
    turn = __unpk2[1].clone();
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUESTION(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&line.clone(), "endswith", &[py_str("?")]) } } } }) {
    continue;
    }
    let _ = mcall(&said.clone(), "append", &[py_tuple(vec![number.clone(), line.clone()])]);
    }
    return said.clone();
    Py::None
}
pub fn kind_of(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The kind of value this line asks for, or None.\n\n    Read off the ask, not the subject: \"how old did i say she was\" and \"what size did i say\" are\n    the same question about different values, and the reader below is the same for both.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_ASKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    kind = __unpk2[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[low.clone()])) {
    return kind.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn carries(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether this line holds a value of that kind.");
    pattern = mcall(&g_KINDS(), "get", &[{ let __b = kind.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&pattern.clone()))) {
    return Py::Bool(false);
    }
    said = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&g_A_TIMED_KIND(), &kind.clone()))) {
    said = mcall(&g_A_RATE(), "sub", &[py_str(" "), said.clone()]);
    }
    return call_builtin("bool", &[mcall(&pattern.clone(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _about(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The words a claim names as its subject, none of them pronouns.");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&_a_word(&[word.clone()])) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _said_words(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of a line that could name the thing it is about, longest first.");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e));
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_STOP(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_PRONOUNS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_VALUE_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&word.clone(), &py_str("'"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&out.clone(), &word.clone())) } } } } } } } } } }) {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _mentions(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line mentions any of these words, in any of its forms.\n\n    \"how often did i say i go\" has to find \"im going three times a week\", so a plural, a\n    possessive and the -ing and -ed forms all count as the word itself. The word has to start a\n    new word, so \"go\" does not find \"dog\", and \"a dog walk every day\" is not about the gym.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:s|es|'s|ing|ed)?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn kinds_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut ask: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let _ = py_str("The kinds a value asked for could be in, the likeliest first.\n\n    \"how much\" is a sum far more often than an amount, but it is an amount when the chat said the\n    thing in grams or litres: \"i usually do 500g of mince for 4\" then \"and how much mince was that\"\n    was handed the search hedge while the 500g was two turns up. A weight or a volume is a size,\n    so the money kind is followed by the size kind and nothing else changes.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("money"))); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow much\\b"), call_builtin("str", &[{ let __b = ask.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_tuple(vec![py_str("money"), py_str("size")]);
    }
    return py_tuple(vec![kind.clone()]);
    Py::None
}
pub fn carried(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut about: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "about") { v.clone() }
    else { py_tuple(vec![]) };
    let mut window: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "window") { v.clone() }
    else { Py::None };
    let mut prefer: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "prefer") { v.clone() }
    else { py_tuple(vec![]) };
    let mut after: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::Int(0) };
    let mut best: Py = Py::None;
    let mut best_line: Py = Py::None;
    let mut best_number: Py = Py::None;
    let mut hits: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut number: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut score: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The latest line of theirs that carried a value of this kind, or None.\n\n    `about` is the subject a claim named: a line has to mention it, else nothing of that kind\n    counts. `prefer` is what the ask itself named (\"what size did i say the chicken was\" is about\n    the chicken): a line that says one of those words wins over one that only carries the value,\n    so a chat that has said both \"three times a week\" and \"every day\" answers the gym question\n    with the gym line. `after` leaves out the lines before a turn, which is how a pronoun is\n    followed back to what it points at.\n    ");
    lines = said_lines(&[memory.clone()]);
    if py_truthy(&window.clone()) {
    lines = slice_or_raise(&lines.clone(), &neg(&window.clone()), &Py::None, &Py::None);
    }
    if py_truthy(&after.clone()) {
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut pair: Py = __item.clone(); if py_truthy(&gt(&geti(&pair.clone(), &Py::Int(0)), &after.clone())) {  __out.push(pair.clone()); } } py_list(__out) })() };
    }
    hits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let __unpk2549 = py_unpack(&__item.clone(), 2, None); let mut number: Py = __unpk2549[0].clone(); let mut line: Py = __unpk2549[1].clone(); if py_truthy(&carries(&[kind.clone(), line.clone()])) {  __out.push(py_tuple(vec![number.clone(), line.clone()])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&hits.clone()))) {
    return Py::None;
    }
    if py_truthy(&about.clone()) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[hits.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    number = __unpk2[0].clone();
    line = __unpk2[1].clone();
    if py_truthy(&_mentions(&[line.clone(), about.clone()])) {
    return py_tuple(vec![number.clone(), line.clone()]);
    }
    }
    }
    if py_truthy(&prefer.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![Py::Int(0), Py::None, Py::None]), 3, None);
    best = __unpk3[0].clone();
    best_number = __unpk3[1].clone();
    best_line = __unpk3[2].clone();
    let __iter4 = py_iter_or_raise(&hits.clone());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    number = __unpk5[0].clone();
    line = __unpk5[1].clone();
    score = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&prefer.clone()) { let mut word: Py = __item.clone(); if py_truthy(&_mentions(&[line.clone(), py_tuple(vec![word.clone()])])) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&ge(&score.clone(), &best.clone())) {
    let __unpk6 = py_unpack(&py_tuple(vec![score.clone(), number.clone(), line.clone()]), 3, None);
    best = __unpk6[0].clone();
    best_number = __unpk6[1].clone();
    best_line = __unpk6[2].clone();
    }
    }
    if py_truthy(&best.clone()) {
    return py_tuple(vec![best_number.clone(), best_line.clone()]);
    }
    }
    if py_truthy(&about.clone()) {
    return Py::None;
    }
    return geti(&hits.clone(), &neg(&Py::Int(1)));
    Py::None
}
pub fn _pointing_back(args: &[Py]) -> Py {
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
    let mut about: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "about") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: about"))) };
    let mut _u: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("Whether a line of the chat names what the ask names, so the ask is pointing at it.\n\n    \"back to the bedroom, what size was it again\" names the bedroom and the chat has the bedroom,\n    so the one size line in the chat is what it means. \"and remind me where we said wed stay\"\n    names staying and the chat never said it, so the one place line in the chat is not that.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&about.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_lines(&[memory.clone()])) { let __unpk7796 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk7796[0].clone(); let mut line: Py = __unpk7796[1].clone(); __out.push(_mentions(&[line.clone(), about.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn carried_alone(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut _u: Py = Py::None;
    let mut hits: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("The one line of the chat that carries a value of this kind, when there is exactly one.\n\n    A chat that has said one size and nothing else has answered \"what size was it again\" whatever\n    the ask calls the thing - the wallpaper chat's \"back to the bedroom, what size was it again\" is\n    about the 3 metres by 4 it gave a few turns before. Two values of the kind and there is nothing\n    to hand back, so nothing is guessed.\n    ");
    hits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_lines(&[memory.clone()])) { let __unpk7892 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk7892[0].clone(); let mut line: Py = __unpk7892[1].clone(); if py_truthy(&carries(&[kind.clone(), line.clone()])) {  __out.push(line.clone()); } } py_list(__out) })() };
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[hits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&hits.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn hand_back(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut low: Py = Py::None;
    let _ = py_str("Their own line, quoted, in the plainest words there are for it.");
    low = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("line".to_string(), low.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_HAND_BACK(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, add(&add(&low.clone(), &py_str(" ")), &call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]), "format", args))(&__args) };
    Py::None
}
pub fn a_claim(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The part of a reply that claims the person never said something, or None.");
    found = mcall(&g_CLAIM(), "search", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return slice_or_raise(&call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)), &mcall(&found.clone(), "start", &[]), &Py::None, &Py::None);
    Py::None
}
pub fn _values(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut piece: Py = Py::None;
    let _ = py_str("The values of a kind one line carries, as plain words, for telling two lines apart.");
    pattern = mcall(&g_KINDS(), "get", &[call_builtin("str", &[{ let __b = kind.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(py_is(&pattern.clone(), &Py::None))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[low.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    piece = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("[^a-z0-9. ]"), py_str(" "), mcall(&match_.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&piece.clone()) {
    let _ = mcall(&out.clone(), "add", &[piece.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _the_line_the_ask_names(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("The line a recall ask means, when it names the thing that line is about.\n\n    `how often did i say i go` names the gym, so the dog walk said after it is not the line meant.\n    Only an ask about their own words or habit can name a line this way; every other line is the\n    store's to answer as it likes, and None says so.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_RECALL(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { their_own_words(&[low.clone()]) } }))) {
    return py_tuple(vec![Py::None, py_tuple(vec![]), Py::None]);
    }
    kind = kind_of(&[low.clone()]);
    named = _said_words(&[low.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kind.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&named.clone())) } }) {
    return py_tuple(vec![Py::None, named.clone(), Py::None]);
    }
    return py_tuple(vec![kind.clone(), named.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(kind.clone()); __args.push(py_tuple(vec![])); __kws.extend(vec![("prefer".to_string(), named.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| carried(args))(&__args) }]);
    Py::None
}
pub fn gives_the_named_value(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("Whether the reply gives the value of the line the ask names, not another line's value.");
    let __unpk1 = py_unpack(&_the_line_the_ask_names(&[text.clone(), memory.clone()]), 3, None);
    kind = __unpk1[0].clone();
    named = __unpk1[1].clone();
    found = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[bitand(&_values(&[kind.clone(), reply.clone()]), &_values(&[kind.clone(), geti(&found.clone(), &Py::Int(1))]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn another_line_was_asked_for(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("True when a recall ask that names something was answered from a different line.\n\n    A recall ask says which line it means: `how often did i say i go` is about the gym, not about\n    the dog walk said after it. The store answered that line with \"You also walk the dog every\n    day.\" - a line of the right kind, and not the one the ask named - so a reply that hands back\n    the line the ask names is the answer to the question that was asked. A reply that gives the\n    value the named line gives is answering from that line, however few of its words it repeats:\n    `and when does he start again` was answered \"In September.\" and that is the same September.\n    ");
    let __unpk1 = py_unpack(&_the_line_the_ask_names(&[text.clone(), memory.clone()]), 3, None);
    kind = __unpk1[0].clone();
    named = __unpk1[1].clone();
    found = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_mentions(&[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), named.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&gives_the_named_value(&[text.clone(), memory.clone(), reply.clone()])));
    Py::None
}
pub fn a_misquoted_line(args: &[Py]) -> Py {
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
    let mut ask: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { py_str("") };
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut quoted: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The line to hand back when the reply quotes a line that does not carry what was asked.\n\n    `what day did i say my appointment was` was answered \"You said: \"i finally got my hospital\n    appointment through\"\" - a real line of theirs, quoted back, with no day in it at all, while\n    `its on the 12th of march at half nine` sat two turns up. A quotation is only an answer when\n    the line it quotes carries the kind of value the ask is asking about; when another line does,\n    that line is the answer and this returns it.\n    ");
    text = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_HAND_BACKS(), "match", &[text.clone()])))) {
    return Py::None;
    }
    quoted = stdlib_call("re", "search", &[py_str("\"([^\"]{2,300})\""), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    kind = kind_of(&[ask.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&quoted.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&kind.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&_values(&[kind.clone(), mcall(&quoted.clone(), "group", &[Py::Int(1)])])) {
    return Py::None;
    }
    found = carried(&[memory.clone(), kind.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&geti(&found.clone(), &Py::Int(1)), "lower", &[]), "split", &[])]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&quoted.clone(), "group", &[Py::Int(1)]), "lower", &[]), "split", &[])])))) {
    return Py::None;
    }
    return hand_back(&[geti(&found.clone(), &Py::Int(1)), memory.clone(), { let __b = a_claim(&[text.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
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
    let mut alone: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("A recall ask nothing else answered: their own line, handed back, when it carries the value.\n\n    \"what size did i say\", \"when did i say we fly\" and \"how often did i say i go\" were handed to\n    the search hedge while the size, the date and the number of times were in the line above.\n    This is the same check as `put_right`, asked from the other side: the kind comes from the ask,\n    and the line comes from the chat.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_RECALL(), "search", &[low.clone()]))) } }) {
    return Py::None;
    }
    kind = kind_of(&[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    about = _about(&[low.clone()]);
    named = _said_words(&[low.clone()]);
    found = if py_truthy(&about.clone()) { carried(&[memory.clone(), kind.clone(), about.clone()]) } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(kind.clone()); __args.push(py_tuple(vec![])); __kws.extend(vec![("prefer".to_string(), named.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| carried(args))(&__args) }; if py_truthy(&__b) { __b } else { if py_truthy(&named.clone()) { Py::None } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(kind.clone()); __args.push(py_tuple(vec![])); __kws.extend(vec![("window".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| carried(args))(&__args) } } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { _pointing_back(&[memory.clone(), about.clone()]) } } } }) {
    alone = carried_alone(&[memory.clone(), kind.clone()]);
    found = if py_truthy(&alone.clone()) { py_tuple(vec![call_builtin("len", &[said_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)), alone.clone()]) } else { Py::None };
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return hand_back(&[geti(&found.clone(), &Py::Int(1)), memory.clone(), low.clone()]);
    Py::None
}
pub fn their_own_words(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the ask is about something the person themselves said or does.");
    return call_builtin("bool", &[mcall(&g_THEIR_OWN_WORDS(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn from_the_line(args: &[Py]) -> Py {
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
    let mut alone: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("The answer a line of theirs holds for this question, when nothing else had one.\n\n    `remind me how old she is` and `how old is she`, two turns after \"shes 7 going on 8\", were both\n    handed the search hedge: the store's slot reader files an age only when it is told whose it is\n    (\"my daughter is 6\"), and nothing read the figure that was on the screen. The kind is read\n    from the question and the line from the chat, exactly as `answer_for` does it; the difference\n    is only that no recall cue is needed, because the caller has already found that nothing else\n    answered.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_QUESTION(), "match", &[low.clone()]))) } }) {
    return Py::None;
    }
    kind = kind_of(&[low.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kind.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_PLAIN_ASK(), &kind.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_A_RECALL_ASK(), "search", &[low.clone()])) {
    return Py::None;
    }
    about = _about(&[low.clone()]);
    named = _said_words(&[low.clone()]);
    found = Py::None;
    let __iter1 = py_iter_or_raise(&kinds_for(&[kind.clone(), low.clone()]));
    for __item in __iter1 {
    kind = __item.clone();
    found = if py_truthy(&about.clone()) { carried(&[memory.clone(), kind.clone(), about.clone()]) } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(kind.clone()); __args.push(py_tuple(vec![])); __kws.extend(vec![("prefer".to_string(), named.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| carried(args))(&__args) }; if py_truthy(&__b) { __b } else { if py_truthy(&named.clone()) { Py::None } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(kind.clone()); __args.push(py_tuple(vec![])); __kws.extend(vec![("window".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| carried(args))(&__args) } } } } };
    if py_truthy(&found.clone()) {
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { _pointing_back(&[memory.clone(), about.clone()]) } }) {
    alone = carried_alone(&[memory.clone(), kind.clone()]);
    found = if py_truthy(&alone.clone()) { py_tuple(vec![call_builtin("len", &[said_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)), alone.clone()]) } else { Py::None };
    if py_truthy(&found.clone()) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return hand_back(&[geti(&found.clone(), &Py::Int(1)), memory.clone(), low.clone()]);
    Py::None
}
pub fn put_right(args: &[Py]) -> Py {
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
    let mut ask: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { py_str("") };
    let mut asked: Py = Py::None;
    let mut claim: Py = Py::None;
    let mut found: Py = Py::None;
    let mut introduced: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut kinds: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_here: Py = Py::None;
    let mut number: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut window: Py = Py::None;
    let _ = py_str("The reply with a false claim of not knowing replaced by their own line, or None.\n\n    One gate, and it is the whole rule: a reply may say the bot does not know a thing only if no\n    line of theirs carries it. When one does, the line is handed back - it cannot contradict the\n    conversation, and it is the answer they asked for. A claim with nothing behind it in the\n    chat is left exactly as it was, because then it is true.\n    ");
    claim = a_claim(&[reply.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&claim.clone()))) {
    return Py::None;
    }
    asked = { let __b = kind_of(&[claim.clone()]); if py_truthy(&__b) { __b } else { kind_of(&[ask.clone()]) } };
    kinds = if py_truthy(&asked.clone()) { py_tuple(vec![asked.clone()]) } else { py_tuple(vec![py_str("age"), py_str("name"), py_str("when"), py_str("size"), py_str("money"), py_str("often"), py_str("duration")]) };
    named = _about(&[claim.clone()]);
    subject = { let __b = named.clone(); if py_truthy(&__b) { __b } else { _about(&[ask.clone()]) } };
    named_here = { let __b = call_builtin("bool", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _mentions(&[call_builtin("str", &[{ let __b = ask.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), named.clone()]) } };
    window = if py_truthy(&subject.clone()) { Py::None } else { Py::Int(3) };
    let __iter1 = py_iter_or_raise(&kinds.clone());
    for __item in __iter1 {
    kind = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(kind.clone()); __args.push(subject.clone()); __kws.extend(vec![("window".to_string(), window.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| carried(args))(&__args) };
    if py_truthy(&found.clone()) {
    return hand_back(&[geti(&found.clone(), &Py::Int(1)), memory.clone(), claim.clone()]);
    }
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&named.clone())) } }) {
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(kind.clone()); __args.push(py_tuple(vec![])); __kws.extend(vec![("window".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| carried(args))(&__args) };
    if py_truthy(&found.clone()) {
    return hand_back(&[geti(&found.clone(), &Py::Int(1)), memory.clone(), claim.clone()]);
    }
    } else {
    if py_truthy(&named.clone()) {
    introduced = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said_lines(&[memory.clone()])) { let __unpk8016 = py_unpack(&__item.clone(), 2, None); let mut number: Py = __unpk8016[0].clone(); let mut line: Py = __unpk8016[1].clone(); if py_truthy(&_mentions(&[line.clone(), named.clone()])) {  __out.push(number.clone()); } } py_list(__out) })() };
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(kind.clone()); __args.push(py_tuple(vec![py_str("she"), py_str("he"), py_str("they")])); __kws.extend(vec![("after".to_string(), if py_truthy(&named_here.clone()) { call_builtin("max", &[{ let __b = introduced.clone(); if py_truthy(&__b) { __b } else { py_list(vec![Py::Int(0)]) } }]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| carried(args))(&__args) };
    if py_truthy(&found.clone()) {
    return hand_back(&[geti(&found.clone(), &Py::Int(1)), memory.clone(), claim.clone()]);
    }
    }
    }
    }
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asked.clone())) } }) {
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[said_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    number = __unpk3[0].clone();
    line = __unpk3[1].clone();
    if py_truthy(&_mentions(&[line.clone(), named.clone()])) {
    return hand_back(&[line.clone(), memory.clone(), claim.clone()]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.earlier".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS" => Some(g_ASKS()),
            "A_RATE" => Some(g_A_RATE()),
            "A_TIMED_KIND" => Some(g_A_TIMED_KIND()),
            "CLAIM" => Some(g_CLAIM()),
            "DAYS" => Some(g_DAYS()),
            "HAND_BACK" => Some(g_HAND_BACK()),
            "HAND_BACKS" => Some(g_HAND_BACKS()),
            "KINDS" => Some(g_KINDS()),
            "MONTHS" => Some(g_MONTHS()),
            "NOT_A_PLAIN_ASK" => Some(g_NOT_A_PLAIN_ASK()),
            "NOT_A_RECALL_ASK" => Some(g_NOT_A_RECALL_ASK()),
            "PRONOUNS" => Some(g_PRONOUNS()),
            "QUESTION" => Some(g_QUESTION()),
            "RECALL" => Some(g_RECALL()),
            "STOP" => Some(g_STOP()),
            "THEIR_OWN_WORDS" => Some(g_THEIR_OWN_WORDS()),
            "UNITS" => Some(g_UNITS()),
            "VALUE_WORDS" => Some(g_VALUE_WORDS()),
            "_a_word" => Some(py_func("_a_word", |args: &[Py]| _a_word(args))),
            "_about" => Some(py_func("_about", |args: &[Py]| _about(args))),
            "_mentions" => Some(py_func("_mentions", |args: &[Py]| _mentions(args))),
            "_pointing_back" => Some(py_func("_pointing_back", |args: &[Py]| _pointing_back(args))),
            "_said_words" => Some(py_func("_said_words", |args: &[Py]| _said_words(args))),
            "_the_line_the_ask_names" => Some(py_func("_the_line_the_ask_names", |args: &[Py]| _the_line_the_ask_names(args))),
            "_turns" => Some(py_func("_turns", |args: &[Py]| _turns(args))),
            "_values" => Some(py_func("_values", |args: &[Py]| _values(args))),
            "a_claim" => Some(py_func("a_claim", |args: &[Py]| a_claim(args))),
            "a_misquoted_line" => Some(py_func("a_misquoted_line", |args: &[Py]| a_misquoted_line(args))),
            "another_line_was_asked_for" => Some(py_func("another_line_was_asked_for", |args: &[Py]| another_line_was_asked_for(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "carried" => Some(py_func("carried", |args: &[Py]| carried(args))),
            "carried_alone" => Some(py_func("carried_alone", |args: &[Py]| carried_alone(args))),
            "carries" => Some(py_func("carries", |args: &[Py]| carries(args))),
            "from_the_line" => Some(py_func("from_the_line", |args: &[Py]| from_the_line(args))),
            "gives_the_named_value" => Some(py_func("gives_the_named_value", |args: &[Py]| gives_the_named_value(args))),
            "hand_back" => Some(py_func("hand_back", |args: &[Py]| hand_back(args))),
            "kind_of" => Some(py_func("kind_of", |args: &[Py]| kind_of(args))),
            "kinds_for" => Some(py_func("kinds_for", |args: &[Py]| kinds_for(args))),
            "put_right" => Some(py_func("put_right", |args: &[Py]| put_right(args))),
            "said_lines" => Some(py_func("said_lines", |args: &[Py]| said_lines(args))),
            "their_own_words" => Some(py_func("their_own_words", |args: &[Py]| their_own_words(args))),
            _ => None,
        }),
        set: None,
    }))
}
