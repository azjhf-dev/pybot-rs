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


thread_local! { static G_A_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEWS_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REPORT_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.news_ask: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_FRAME() -> Py { ensure_init(); G_A_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FRAME(v: Py) { ensure_init(); G_A_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEWS_DOING() -> Py { ensure_init(); G_A_NEWS_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEWS_DOING(v: Py) { ensure_init(); G_A_NEWS_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REPORT_VERB() -> Py { ensure_init(); G_A_REPORT_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REPORT_VERB(v: Py) { ensure_init(); G_A_REPORT_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT() -> Py { ensure_init(); G_A_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT(v: Py) { ensure_init(); G_A_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_YEAR() -> Py { ensure_init(); G_A_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_YEAR(v: Py) { ensure_init(); G_A_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_OWN() -> Py { ensure_init(); G_THEIR_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN(v: Py) { ensure_init(); G_THEIR_OWN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that names the news and nothing else is the ask with the words left out.\n\n\"anything in the news\", \"the headlines\", \"news about the strike\" carry no question mark, so the\nasking gate in `bot/compose/fallback.py` dropped them and the line was met with a nod (\"I'm\nlistening - say more if it helps\") as though the person had told the bot something, or with the\nnothing-much pool's \"None that I know of\", which claims the bot has looked and found nothing.\n\nWhat makes a line this ask is its shape, not a list of phrasings: an asking frame, then the news\nword, then nothing but where or when it is, or one thing it is about. A line that puts anything\nelse after the word is about the person's own business and keeps its own answer - \"i have news\"\nis an announcement the nod answers, \"news from the hospital\" is about someone they know, \"no\nnews yet\" is a verdict they have already reached.\n\nThis module answers only the lines it is good at and says nothing about any other line.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.news_ask.A_FRAME"); }
    s_A_FRAME(stdlib_call("re", "compile", &[py_str("^(?:so|and|ok|okay|well|but)?\\s*(?:any|anything|got|get|is\\s+there|are\\s+there|whats?|what\\s+(?:is|are)|tell\\s+me|you\\s+got|have\\s+you\\s+got|more|the|some)?\\s*(?:in\\s+the\\s+|from\\s+the\\s+|on\\s+the\\s+|the\\s+)?(?:news|headlines?)(?:\\s+(?:about|on|of|regarding)\\s+.*|\\s+(?:today|tonight|this\\s+(?:morning|afternoon|evening)|at\\s+the\\s+moment|right\\s+now|please|then)\\b.*)?$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.news_ask.A_NEWS_DOING"); }
    s_A_NEWS_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:news|headlines?)\\s+(?:is|are|was|were|has|have|had|gets?|got|makes?|made|keeps?|puts?|bothered|upsets?|worries)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.news_ask.A_VERDICT"); }
    s_A_VERDICT(stdlib_call("re", "compile", &[py_str("\\b(?:no|some|good|bad|great|awful|sad|terrible|nice|exciting|brilliant|lovely|fantastic|amazing|grim|big)\\s+(?:news|headlines?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.news_ask.THEIR_OWN"); }
    s_THEIR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|my|our|his|her|their)\\b.*\\b(?:news|headlines?)\\b|\\b(?:news|headlines?)\\b.*\\b(?:my|our|his|her|their)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.news_ask.A_REPORT_VERB"); }
    s_A_REPORT_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:watched|watching|read|reading|heard|hear|told|saw|seen|made|missed|followed|caught)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.news_ask.A_YEAR"); }
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
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_FRAME" => s_A_FRAME(v),
            "A_NEWS_DOING" => s_A_NEWS_DOING(v),
            "A_REPORT_VERB" => s_A_REPORT_VERB(v),
            "A_VERDICT" => s_A_VERDICT(v),
            "A_YEAR" => s_A_YEAR(v),
            "THEIR_OWN" => s_THEIR_OWN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.news_ask", module_obj);
    pyrt::xrefs::reg_fn("compose.news_ask", "reads_as_an_ask_for_the_news", py_func("reads_as_an_ask_for_the_news", |args: &[Py]| reads_as_an_ask_for_the_news(args)));
    pyrt::xrefs::reg_access("compose.news_ask", "A_FRAME", g_A_FRAME, s_A_FRAME);
    pyrt::xrefs::reg_access("compose.news_ask", "A_NEWS_DOING", g_A_NEWS_DOING, s_A_NEWS_DOING);
    pyrt::xrefs::reg_access("compose.news_ask", "A_REPORT_VERB", g_A_REPORT_VERB, s_A_REPORT_VERB);
    pyrt::xrefs::reg_access("compose.news_ask", "A_VERDICT", g_A_VERDICT, s_A_VERDICT);
    pyrt::xrefs::reg_access("compose.news_ask", "A_YEAR", g_A_YEAR, s_A_YEAR);
    pyrt::xrefs::reg_access("compose.news_ask", "THEIR_OWN", g_THEIR_OWN, s_THEIR_OWN);
}
