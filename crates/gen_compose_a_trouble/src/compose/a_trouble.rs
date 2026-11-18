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


thread_local! { static G_A_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ITS_THE_WEATHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALREADY_NARROWING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BARE_OFFER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FUNCTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STOCK_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERBING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__a_hedge: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_restates_the_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_unlike_the_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_TROUBLE() -> Py { ensure_init(); G_A_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TROUBLE(v: Py) { ensure_init(); G_A_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRE_WORD() -> Py { ensure_init(); G_FIRE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRE_WORD(v: Py) { ensure_init(); G_FIRE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ITS_THE_WEATHER() -> Py { ensure_init(); G_ITS_THE_WEATHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ITS_THE_WEATHER(v: Py) { ensure_init(); G_ITS_THE_WEATHER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_MACHINE() -> Py { ensure_init(); G_NOT_A_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MACHINE(v: Py) { ensure_init(); G_NOT_A_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g_STALL() -> Py { ensure_init(); G_STALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STALL(v: Py) { ensure_init(); G_STALL.with(|c| *c.borrow_mut() = v); }
pub fn g__ALREADY_NARROWING() -> Py { ensure_init(); G__ALREADY_NARROWING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALREADY_NARROWING(v: Py) { ensure_init(); G__ALREADY_NARROWING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BARE_OFFER() -> Py { ensure_init(); G__A_BARE_OFFER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BARE_OFFER(v: Py) { ensure_init(); G__A_BARE_OFFER.with(|c| *c.borrow_mut() = v); }
pub fn g__DETERMINERS() -> Py { ensure_init(); G__DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DETERMINERS(v: Py) { ensure_init(); G__DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g__FUNCTION() -> Py { ensure_init(); G__FUNCTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FUNCTION(v: Py) { ensure_init(); G__FUNCTION.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_THING() -> Py { ensure_init(); G__NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_THING(v: Py) { ensure_init(); G__NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__REPLIES() -> Py { ensure_init(); G__REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REPLIES(v: Py) { ensure_init(); G__REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g__STOCK_LENGTH() -> Py { ensure_init(); G__STOCK_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STOCK_LENGTH(v: Py) { ensure_init(); G__STOCK_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__STOP() -> Py { ensure_init(); G__STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STOP(v: Py) { ensure_init(); G__STOP.with(|c| *c.borrow_mut() = v); }
pub fn g__VERBING() -> Py { ensure_init(); G__VERBING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERBING(v: Py) { ensure_init(); G__VERBING.with(|c| *c.borrow_mut() = v); }
pub fn g__a_hedge() -> Py { ensure_init(); G__a_hedge.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__a_hedge(v: Py) { ensure_init(); G__a_hedge.with(|c| *c.borrow_mut() = v); }
pub fn g_restates_the_line() -> Py { ensure_init(); G_restates_the_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_restates_the_line(v: Py) { ensure_init(); G_restates_the_line.with(|c| *c.borrow_mut() = v); }
pub fn g_unlike_the_line() -> Py { ensure_init(); G_unlike_the_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_unlike_the_line(v: Py) { ensure_init(); G_unlike_the_line.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("One question back about a trouble, named, instead of a stall that fits any message.\n\nThe census turn (`notes/tools/stockline.py`) that leaves people nowhere is a plain statement of a\ntrouble with no card behind it:\n\n    my airpods are not showing up          -> Got it. Is there anything you'd like a hand with?\n    my electric blanket is smoking          -> the same stall, on main and after this change\n\nThe first is answerable with one question, and it is the same question for a boiler, a car\nand a laptop: is it every time or only sometimes, and did anything change. A friend who cannot\nsee the thing asks that rather than \"tell me more\", because the answer decides what to ask next.\n\nSo the reply names the thing from the person's own line - one word of it, the most specific - and\nasks one of the three questions a troubleshooter asks first. It holds no fact, so it cannot be\nwrong, and it leaves the thread somewhere to go.\n\nIt is not for a decision (\"im chucking up my evening class\"), a feeling, or anything a card\nanswers: `A_TROUBLE` is the words that mean a thing is not doing its job, and the module stands\ndown unless the composer's own reply says nothing about the line.\n");
    s__a_hedge(py_func("is_a_hedge", |args: &[Py]| pyrt::xrefs::xcall("compose.aboutness", "is_a_hedge", args)));
    s_restates_the_line(py_func("restates_the_line", |args: &[Py]| pyrt::xrefs::xcall("compose.aboutness", "restates_the_line", args)));
    s_unlike_the_line(py_func("unlike_the_line", |args: &[Py]| pyrt::xrefs::xcall("compose.aboutness", "unlike_the_line", args)));
    s__VERBING(py_str("(?:work\\w*|show\\w*|connect\\w*|pair\\w*|charg\\w*|turn\\w*|start\\w*|boot\\w*|open\\w*|load\\w*|sync\\w*|respond\\w*|detect\\w*|heat\\w*|drain\\w*|fill\\w*|print\\w*|spin\\w*|run\\w*|play\\w*|record\\w*|boil\\w*|flush\\w*|close\\w*|lock\\w*|los\\w*|drop\\w*|cut\\w*|jamm\\w*|fall\\w*|slip\\w*|stall\\w*|stall?|squeal\\w*|whistl\\w*|smell\\w*|smoke|smoking|overheat\\w*|beep\\w*|go\\w*|going|gone|drip\\w*|clog\\w*)"));
    s_A_TROUBLE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:\\b(?:is\\s+not|isn'?t|are\\s+not|aren'?t|was\\s+not|wasn'?t|does\\s+not|doesn'?t|do\\s+not|don'?t|did\\s+not|didn'?t)\\s+"), &g__VERBING()), &py_str("|\\b(?:wont|won'?t|will\\s+not|wouldn'?t|refuses?\\s+to)\\s+")), &g__VERBING()), &py_str("|\\b(?:keeps?|kept|stopped|stops)\\s+")), &g__VERBING()), &py_str("|\\b(?:is|are|it'?s)\\s+(?:broken|bust|busted|dead|faulty|knackered|on\\s+the\\s+blink)\\b|\\b(?:mak\\w*|made)\\s+(?:a\\s+|an\\s+|that\\s+)?(?:noise|racket|din|humming|buzzing|whirring|clicking)\\b|\\b(?:broke|broken|leaks?|leaking|leaked|stuck|sticking|noisy|noise|rattl\\w*|slow|sluggish|laggy|freez\\w*|crash\\w*|error|glitch\\w*|bug|bugs|buggy|acting\\s+up|playing\\s+up|gone\\s+wrong|doing\\s+something\\s+weird)\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_MACHINE(stdlib_call("re", "compile", &[py_str("\\b(?:son|daughter|child|kid|baby|mum|mom|mother|dad|father|wife|husband|partner|boyfriend|girlfriend|brother|sister|friend|mate|nan|gran|grandad|grandma|aunt|uncle|cousin|family|relationship|marriage|heart|trust|friendship|things)\\b.{0,20}?\\b(?:is|are|was|were|it'?s)\\s+(?:broken|bust|busted|dead|gone|over|finished|ruined)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:spark\\w*|scorch\\w*|burn(?:s|ing|t)?|catch(?:es|ing)?\\s+fire|flames?|melt(?:s|ing|ed)?|fizz\\w*|smoulder\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THE_THING(call_builtin("frozenset", &[mcall(&py_str("\nworking work works worked showing show shows shown connecting connected pairing paired\ncharging charged turning turned booting booted starting started opening opened loading loaded\nsyncing synced updating updated playing played running ran doing done going stuck slow\nwrong broken bust faulty weird strange dead fine better worse again still always never\nkeep keeps kept isnt arent wasnt doesnt dont didnt wont wont stop stops stopped making makes\nnoise leaking leak making enough too quite really\ntoday tonight tomorrow yesterday morning afternoon evening night daytime weekend now later\nmeantime outside indoors upstairs downstairs somewhere anywhere nowhere everywhere here there\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ITS_THE_WEATHER(stdlib_call("re", "compile", &[py_str("^\\s*(?:oh\\s+)?(?:it'?s|its|it\\s+is)\\s+(?:so\\s+|very\\s+|really\\s+|properly\\s+|absolutely\\s+|bloody\\s+|still\\s+|turned\\s+|got\\s+)?(?:freezing|boiling|roasting|baltic|chilly|nippy|pouring|raining|snowing|sleeting|hailing|blowing|howling|mild|warm|hot|cold|wet|dry|damp|dark|grey|gray|grim|miserable|lovely|nice|glorious)\\b|\\bchucking\\s+it\\s+down\\b|\\bblowing\\s+a\\s+gale\\b|\\brain(?:ing|y)\\s+(?:again|all\\s+day|now)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__REPLIES(py_tuple(vec![py_str("%s - every time, or only sometimes?"), py_str("Has anything changed since %s last worked - an update, a new one, a move?"), py_str("%s - how long has it been doing that?")]));
    s__DETERMINERS(py_tuple(vec![py_str("my"), py_str("our"), py_str("the"), py_str("this"), py_str("that"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("its")]));
    s__FUNCTION(call_builtin("frozenset", &[mcall(&py_str("\nthe a an my our your his her their its this that these those that's thats some any something\nanything nothing everything all most more less very just really quite still always never again\nthere here out up off down over back with without from than then about into onto and but so\nwhen since while if or of to in on at for as it is are was were be been being got have has had\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__STOP(call_builtin("frozenset", &[mcall(&py_str("\nis are was were has have had wont will does doesnt didnt not never keeps kept still always\nin on at to of for with and but so when since about all just really very much more less\nisnt arent wasnt dont wont cant couldnt shouldnt wouldnt\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__STOCK_LENGTH(Py::Int(14));
    s__ALREADY_NARROWING(stdlib_call("re", "compile", &[py_str("\\b(?:every\\s+time|now\\s+and\\s+then|now\\s+and\\s+again|when\\s+did\\s+it\\s+start|how\\s+long\\s+has|what\\s+does\\s+it\\s+do|what'?s\\s+it\\s+doing|what\\s+happens\\s+when|what\\s+were\\s+you\\s+doing|since\\s+when|what\\s+it\\s+does|what\\s+have\\s+you\\s+tried|tell\\s+me\\s+what|say\\s+more)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BARE_OFFER(stdlib_call("re", "compile", &[py_str("^\\s*(?:got it[.,]?\\s*|okay[.,]?\\s*|right[.,]?\\s*)?(?:is\\s+there\\s+anything|anything\\s+else|what\\s+else|can\\s+i\\s+help)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STALL(stdlib_call("re", "compile", &[py_str("(?:\\bi'?m listening\\b|\\bsay more if it helps\\b|\\btell me more\\b|\\banything you'?d like a hand with\\b|\\bwhat'?s on your mind\\b|\\bi'?m here if you want to\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn the_thing(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut one: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut start: Py = Py::None;
    let mut trouble_words: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The most specific thing the line is about, with its determiner, or \"\" when there is none.");
    clean = call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    words = mcall(&clean.clone(), "split", &[]);
    spans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_TROUBLE(), "finditer", &[clean.clone()])) { let mut one: Py = __item.clone(); __out.push(py_tuple(vec![mcall(&one.clone(), "start", &[]), mcall(&one.clone(), "end", &[])])); } py_list(__out) })() };
    trouble_words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&spans.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    start = __unpk2[0].clone();
    end = __unpk2[1].clone();
    let _ = mcall(&trouble_words.clone(), "update", &[mcall(&slice_or_raise(&clean.clone(), &start.clone(), &end.clone(), &Py::None), "split", &[])]);
    }
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__FUNCTION(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__NOT_THE_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&trouble_words.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "isalpha", &[]) } } } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    return _named(&[clean.clone(), words.clone(), found.clone(), spans.clone()]);
    Py::None
}
pub fn _named(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut found: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut spans: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: spans"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut best: Py = Py::None;
    let mut name: Py = Py::None;
    best = geti(&found.clone(), &Py::Int(0));
    at = mcall(&words.clone(), "index", &[best.clone()]);
    name = best.clone();
    after = if py_truthy(&op_lt(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &op_add(&at.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&after.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&found.clone(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__STOP(), &after.clone())) } } } } } } } }) {
    name = op_modulo(&py_str("%s %s"), &py_tuple(vec![best.clone(), after.clone()]));
    }
    before = if py_truthy(&at.clone()) { geti(&words.clone(), &op_sub(&at.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g__DETERMINERS(), &before.clone()))) {
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![before.clone(), name.clone()]));
    }
    return op_modulo(&py_str("the %s"), &name.clone());
    Py::None
}
pub fn _heavy(args: &[Py]) -> Py {
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
    let mut _fallback: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the thread is heavy news, read through the composer's own test for it.");
    _fallback = pyrt::xrefs::xmodule("compose.fallback");
    turns = call_builtin("list", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&_fallback.clone(), "reads_as_heavy_news", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]); if py_truthy(&__b) { __b } else { mcall(&_fallback.clone(), "_the_thread_is_heavy", &[py_dict(vec![(py_str("turns"), turns.clone())])]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn trouble_pool(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _fallback: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The one question this trouble raises, or None when this is not a trouble.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&_heavy(&[memory.clone()])) {
    return Py::None;
    }
    _fallback = pyrt::xrefs::xmodule("compose.fallback");
    if py_truthy(&mcall(&_fallback.clone(), "reads_as_heavy_news", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_MACHINE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FIRE_WORD(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ITS_THE_WEATHER(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_TROUBLE(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    thing = the_thing(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    pool = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__REPLIES()) { let mut line: Py = __item.clone(); __out.push(op_add(&mcall(&geti(&op_modulo(&line.clone(), &thing.clone()), &Py::Int(0)), "upper", &[]), &slice_or_raise(&op_modulo(&line.clone(), &thing.clone()), &Py::Int(1), &Py::None, &Py::None))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let __cmp11: Py = call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_lt(&Py::Int(2), &__cmp11)) { op_lt(&__cmp11, &Py::Int(22)) } else { Py::Bool(false) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { pool.clone() } else { Py::None };
    Py::None
}
pub fn _a_bare_offer(args: &[Py]) -> Py {
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
    return call_builtin("bool", &[mcall(&g__A_BARE_OFFER(), "search", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn says_nothing_about_the_line(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the composer's own reply is a hedge, the line given back, or a stock line.");
    reply = mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[])))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_STALL(), "search", &[reply.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&reply.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__STOCK_LENGTH())) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&mcall(&reply.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_bare_offer(&[reply.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__ALREADY_NARROWING(), "search", &[reply.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("compose.aboutness", "is_a_hedge", &[reply.clone()]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("compose.aboutness", "restates_the_line", &[reply.clone(), text.clone()]) } }) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&pyrt::xrefs::xcall("compose.aboutness", "unlike_the_line", &[reply.clone(), text.clone()]), 2, None);
    about = __unpk1[0].clone();
    said = __unpk1[1].clone();
    return { let __b = call_builtin("bool", &[about.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_trouble".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_TROUBLE" => Some(g_A_TROUBLE()),
            "FIRE_WORD" => Some(g_FIRE_WORD()),
            "ITS_THE_WEATHER" => Some(g_ITS_THE_WEATHER()),
            "NOT_A_MACHINE" => Some(g_NOT_A_MACHINE()),
            "STALL" => Some(g_STALL()),
            "_ALREADY_NARROWING" => Some(g__ALREADY_NARROWING()),
            "_A_BARE_OFFER" => Some(g__A_BARE_OFFER()),
            "_DETERMINERS" => Some(g__DETERMINERS()),
            "_FUNCTION" => Some(g__FUNCTION()),
            "_NOT_THE_THING" => Some(g__NOT_THE_THING()),
            "_REPLIES" => Some(g__REPLIES()),
            "_STOCK_LENGTH" => Some(g__STOCK_LENGTH()),
            "_STOP" => Some(g__STOP()),
            "_VERBING" => Some(g__VERBING()),
            "_a_bare_offer" => Some(py_func("_a_bare_offer", |args: &[Py]| _a_bare_offer(args))),
            "_a_hedge" => Some(g__a_hedge()),
            "_heavy" => Some(py_func("_heavy", |args: &[Py]| _heavy(args))),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "restates_the_line" => Some(g_restates_the_line()),
            "says_nothing_about_the_line" => Some(py_func("says_nothing_about_the_line", |args: &[Py]| says_nothing_about_the_line(args))),
            "the_thing" => Some(py_func("the_thing", |args: &[Py]| the_thing(args))),
            "trouble_pool" => Some(py_func("trouble_pool", |args: &[Py]| trouble_pool(args))),
            "unlike_the_line" => Some(g_unlike_the_line()),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_TROUBLE" => s_A_TROUBLE(v),
            "FIRE_WORD" => s_FIRE_WORD(v),
            "ITS_THE_WEATHER" => s_ITS_THE_WEATHER(v),
            "NOT_A_MACHINE" => s_NOT_A_MACHINE(v),
            "STALL" => s_STALL(v),
            "_ALREADY_NARROWING" => s__ALREADY_NARROWING(v),
            "_A_BARE_OFFER" => s__A_BARE_OFFER(v),
            "_DETERMINERS" => s__DETERMINERS(v),
            "_FUNCTION" => s__FUNCTION(v),
            "_NOT_THE_THING" => s__NOT_THE_THING(v),
            "_REPLIES" => s__REPLIES(v),
            "_STOCK_LENGTH" => s__STOCK_LENGTH(v),
            "_STOP" => s__STOP(v),
            "_VERBING" => s__VERBING(v),
            "_a_hedge" => s__a_hedge(v),
            "restates_the_line" => s_restates_the_line(v),
            "unlike_the_line" => s_unlike_the_line(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_trouble", module_obj);
    pyrt::xrefs::reg_fn("compose.a_trouble", "the_thing", py_func("the_thing", |args: &[Py]| the_thing(args)));
    pyrt::xrefs::reg_fn("compose.a_trouble", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("compose.a_trouble", "_heavy", py_func("_heavy", |args: &[Py]| _heavy(args)));
    pyrt::xrefs::reg_fn("compose.a_trouble", "trouble_pool", py_func("trouble_pool", |args: &[Py]| trouble_pool(args)));
    pyrt::xrefs::reg_fn("compose.a_trouble", "_a_bare_offer", py_func("_a_bare_offer", |args: &[Py]| _a_bare_offer(args)));
    pyrt::xrefs::reg_fn("compose.a_trouble", "says_nothing_about_the_line", py_func("says_nothing_about_the_line", |args: &[Py]| says_nothing_about_the_line(args)));
    pyrt::xrefs::reg_access("compose.a_trouble", "A_TROUBLE", g_A_TROUBLE, s_A_TROUBLE);
    pyrt::xrefs::reg_access("compose.a_trouble", "FIRE_WORD", g_FIRE_WORD, s_FIRE_WORD);
    pyrt::xrefs::reg_access("compose.a_trouble", "ITS_THE_WEATHER", g_ITS_THE_WEATHER, s_ITS_THE_WEATHER);
    pyrt::xrefs::reg_access("compose.a_trouble", "NOT_A_MACHINE", g_NOT_A_MACHINE, s_NOT_A_MACHINE);
    pyrt::xrefs::reg_access("compose.a_trouble", "STALL", g_STALL, s_STALL);
    pyrt::xrefs::reg_access("compose.a_trouble", "_ALREADY_NARROWING", g__ALREADY_NARROWING, s__ALREADY_NARROWING);
    pyrt::xrefs::reg_access("compose.a_trouble", "_A_BARE_OFFER", g__A_BARE_OFFER, s__A_BARE_OFFER);
    pyrt::xrefs::reg_access("compose.a_trouble", "_DETERMINERS", g__DETERMINERS, s__DETERMINERS);
    pyrt::xrefs::reg_access("compose.a_trouble", "_FUNCTION", g__FUNCTION, s__FUNCTION);
    pyrt::xrefs::reg_access("compose.a_trouble", "_NOT_THE_THING", g__NOT_THE_THING, s__NOT_THE_THING);
    pyrt::xrefs::reg_access("compose.a_trouble", "_REPLIES", g__REPLIES, s__REPLIES);
    pyrt::xrefs::reg_access("compose.a_trouble", "_STOCK_LENGTH", g__STOCK_LENGTH, s__STOCK_LENGTH);
    pyrt::xrefs::reg_access("compose.a_trouble", "_STOP", g__STOP, s__STOP);
    pyrt::xrefs::reg_access("compose.a_trouble", "_VERBING", g__VERBING, s__VERBING);
    pyrt::xrefs::reg_access("compose.a_trouble", "_a_hedge", g__a_hedge, s__a_hedge);
    pyrt::xrefs::reg_access("compose.a_trouble", "restates_the_line", g_restates_the_line, s_restates_the_line);
    pyrt::xrefs::reg_access("compose.a_trouble", "unlike_the_line", g_unlike_the_line, s_unlike_the_line);
}
