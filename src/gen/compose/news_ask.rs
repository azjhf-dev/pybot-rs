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


pub static A_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NEWS_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPORT_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_YEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_FRAME() -> Py { ensure_init(); A_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = A_FRAME.write() { *w = v; } }
pub fn g_A_NEWS_DOING() -> Py { ensure_init(); A_NEWS_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NEWS_DOING(v: Py) { ensure_init(); if let Ok(mut w) = A_NEWS_DOING.write() { *w = v; } }
pub fn g_A_REPORT_VERB() -> Py { ensure_init(); A_REPORT_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPORT_VERB(v: Py) { ensure_init(); if let Ok(mut w) = A_REPORT_VERB.write() { *w = v; } }
pub fn g_A_VERDICT() -> Py { ensure_init(); A_VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = A_VERDICT.write() { *w = v; } }
pub fn g_A_YEAR() -> Py { ensure_init(); A_YEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_YEAR(v: Py) { ensure_init(); if let Ok(mut w) = A_YEAR.write() { *w = v; } }
pub fn g_THEIR_OWN() -> Py { ensure_init(); THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A line that names the news and nothing else is the ask with the words left out.\n\n\"anything in the news\", \"the headlines\", \"news about the strike\" carry no question mark, so the\nasking gate in `bot/compose/fallback.py` dropped them and the line was met with a nod (\"I'm\nlistening - say more if it helps\") as though the person had told the bot something, or with the\nnothing-much pool's \"None that I know of\", which claims the bot has looked and found nothing.\n\nWhat makes a line this ask is its shape, not a list of phrasings: an asking frame, then the news\nword, then nothing but where or when it is, or one thing it is about. A line that puts anything\nelse after the word is about the person's own business and keeps its own answer - \"i have news\"\nis an announcement the nod answers, \"news from the hospital\" is about someone they know, \"no\nnews yet\" is a verdict they have already reached.\n\nThis module answers only the lines it is good at and says nothing about any other line.\n");
    s_A_FRAME(stdlib_call("re", "compile", &[py_str("^(?:so|and|ok|okay|well|but)?\\s*(?:any|anything|got|get|is\\s+there|are\\s+there|whats?|what\\s+(?:is|are)|tell\\s+me|you\\s+got|have\\s+you\\s+got|more|the|some)?\\s*(?:in\\s+the\\s+|from\\s+the\\s+|on\\s+the\\s+|the\\s+)?(?:news|headlines?)(?:\\s+(?:about|on|of|regarding)\\s+.*|\\s+(?:today|tonight|this\\s+(?:morning|afternoon|evening)|at\\s+the\\s+moment|right\\s+now|please|then)\\b.*)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEWS_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:news|headlines?)\\s+(?:is|are|was|were|has|have|had|gets?|got|makes?|made|keeps?|puts?|bothered|upsets?|worries)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VERDICT(stdlib_call("re", "compile", &[py_str("\\b(?:no|some|good|bad|great|awful|sad|terrible|nice|exciting|brilliant|lovely|fantastic|amazing|grim|big)\\s+(?:news|headlines?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|my|our|his|her|their)\\b.*\\b(?:news|headlines?)\\b|\\b(?:news|headlines?)\\b.*\\b(?:my|our|his|her|their)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPORT_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:watched|watching|read|reading|heard|hear|told|saw|seen|made|missed|followed|caught)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_YEAR(stdlib_call("re", "compile", &[py_str("\\b(?:19|20)\\d\\d\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn reads_as_an_ask_for_the_news(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks for the news and says nothing else about it.");
    said = mcall(&mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "strip", &[py_str(" .!")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_FRAME(), "match", &[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_NEWS_DOING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_VERDICT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIR_OWN(), "search", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_A_REPORT_VERB(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_YEAR(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.news_ask".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_FRAME" => Some(g_A_FRAME()),
            "A_NEWS_DOING" => Some(g_A_NEWS_DOING()),
            "A_REPORT_VERB" => Some(g_A_REPORT_VERB()),
            "A_VERDICT" => Some(g_A_VERDICT()),
            "A_YEAR" => Some(g_A_YEAR()),
            "THEIR_OWN" => Some(g_THEIR_OWN()),
            "reads_as_an_ask_for_the_news" => Some(py_func("reads_as_an_ask_for_the_news", |args: &[Py]| reads_as_an_ask_for_the_news(args))),
            _ => None,
        }),
        set: None,
    }))
}
