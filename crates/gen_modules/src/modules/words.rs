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


thread_local! { static G_ASKS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXAMPLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKS_WORDS() -> Py { ensure_init(); G_ASKS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WORDS(v: Py) { ensure_init(); G_ASKS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_FILLER() -> Py { ensure_init(); G_ASK_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_FILLER(v: Py) { ensure_init(); G_ASK_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_OBJECT() -> Py { ensure_init(); G_ASK_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OBJECT(v: Py) { ensure_init(); G_ASK_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_EXAMPLED() -> Py { ensure_init(); G_EXAMPLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXAMPLED(v: Py) { ensure_init(); G_EXAMPLED.with(|c| *c.borrow_mut() = v); }
pub fn g_LINES() -> Py { ensure_init(); G_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINES(v: Py) { ensure_init(); G_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTED() -> Py { ensure_init(); G_QUOTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTED(v: Py) { ensure_init(); G_QUOTED.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The words to say, when the answer just gave them.\n\nA person who has been told what to do often asks for the words next: \"what should i say to\nher\", \"how do i word it\", \"what do i text him\", \"what should i put in the email\". By then the\nanswer has usually already put the sentence in their mouth - \"Mention it lightly instead of\nletting it fester: 'I missed hearing from you on my birthday, did it slip your mind?'\", \"ask\nfor a specific figure with the reason attached: \"I'd like to move to 34,000, given...\"\" - and\nthe bot was answering all of those with \"Keep it to two sentences... send me the gist and I'll\ndraft it\", asking again for what it had already written. A wording ask is answered with the\nwords the last answer quoted, so the person can copy them.\n\nSo this module answers only when all three hold: the line asks for the words themselves, the\nbot has just said something, and that something quoted a sentence of three words or more. It\nnever invents a sentence and never guesses at one: no quote in the last answer, no reply.\n");
    s_PRIORITY(Py::Int(1200));
    s_ASKS_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|how)\\s+(?:should|do|would|shall|can|could|might)\\s+i\\s+(?:say|text|email|message|write|reply|answer|word|phrase|tell|open|start)\\b|\\b(?:what|how)\\s+(?:should|do|would|shall|can|could|might)\\s+i\\s+put\\s+(?:it|this|that|these|those|them)\\b|\\b(?:what|how)\\s+(?:should|do|would|shall|can|could|might)\\s+i\\s+put\\s+(?:[^?.]{0,30}?\\b(?:in|into|on)\\s+)?(?:the|my|his|her|their|your|our|a|an)\\s+(?:e-?mail|letter|card|note|text|message|msg|reply|answer|application|invitation|invite|whatsapp|bio|post|comment|review|essay|report)\\b|\\bhow\\s+(?:do|should|would|can|could)\\s+i\\s+(?:word|phrase|put)\\s+(?:it|this|that)\\b|\\bhow\\s+(?:do|should|would|can|could)\\s+i\\s+ask\\s+(?:for\\s+)?(?:it|that|this|them|him|her|someone|somebody)\\b|\\b(?:can|could|should)\\s+i\\s+ask\\s+(?:for\\s+)?(?:it|that|this|them|him|her)\\b(?:\\s+(?:back|again))?[\\s.?!]*$|\\b(?:can|could|would)\\s+you\\s+(?:word|phrase)\\s+(?:it|this|that)\\b|\\b(?:give|gimme)\\s+me\\s+the\\s+words\\b|\\bwhat\\s+are\\s+the\\s+words\\b|\\bwhat(?:'s| is)\\s+the\\s+wording\\b|\\b(?:don'?t|do\\s+not|dont|no\\s+idea|not\\s+sure|never\\s+know\\s+how|can'?t\\s+work\\s+out)\\s+(?:know\\s+)?how\\s+to\\s+(?:tell|say|word|phrase|put|ask|bring|raise|mention|open|start|write|text|email|message)\\b|\\b(?:don'?t|do\\s+not|dont|no\\s+idea|not\\s+sure)\\s+what\\s+to\\s+(?:say|text|write|reply|message|email|call|tell)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUOTED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![A-Za-z0-9])'((?:[^']|'(?=[A-Za-z])){12,300})'(?![A-Za-z0-9])|(?<![A-Za-z0-9])\\\"((?:[^\\\"]|\\\"(?=[A-Za-z])){12,300})\\\"(?![A-Za-z0-9])|\\u201c([^\\u201d]{12,300})\\u201d|\\u2018([^\\u2019]{12,300})\\u2019")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LINES(py_tuple(vec![py_str("{words} Say that, and leave it there - it gives them an easy way to answer."), py_str("{words} That is the whole of it, so don't add a second explanation after it."), py_str("{words} Say it once, early, and let the silence do the rest."), py_str("{words} Keep to those words and let them reply in their own time."), py_str("{words} The short version lands better than a speech, so stop there.")]));
    s_EXAMPLED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\b(?:a|an|the|one|like|such\\s+as|e\\.g\\.))\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_OBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:it|this|that|them|those)\\b|(?:to|for|at|with|in|on)\\s+(?:him|her|them|it|this|that|me|us|everyone|somebody|anybody)\\b|(?:to|for|at|with|in|on)\\s+(?:my|his|her|their|our|your|the|a|an)\\s+[a-z']+|(?:him|her|them|everyone|somebody)\\b|(?:my|his|her|their|our|your|the)\\s+[a-z']+)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:please|then|instead|just|really|up|out|loud|back)\\b[,.]?")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _quoted(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first quoted sentence of three words or more in the bot's last reply, with its marks.\n\n    The marks come back with it because the answer chose them: a sentence the answer set in\n    double quotes is handed back in double quotes, so it looks on screen as it did a turn ago.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUOTED(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "groups", args))(&__args) }) { let mut group: Py = __item.clone(); if py_truthy(&group.clone()) {  __out.push(group.clone()); } } py_list(__out) })() }); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::Int(18))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EXAMPLED(), "search", args))(&__args) }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[?!.]$")); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&words.clone(), &py_str(","))) } }) {
    continue;
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    }
    return py_str("");
    Py::None
}
pub fn _names_its_own(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut filler: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("True when the line asks for the words *and* says what they are for.\n\n    \"what should i say to him\" is the ask and nothing else, so the words of the last answer are\n    the answer. \"how do i say sorry\" and \"how do i say it without a row\" carry their own subject,\n    and the answer two turns back is not it.\n    ");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_WORDS(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(true);
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    step = __item.clone();
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "strip", args))(&__args) };
    moved = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK_OBJECT(), "match", args))(&__args) };
    if py_truthy(&moved.clone()) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&rest.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&moved.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    _u = __item.clone();
    filler = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK_FILLER(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&filler.clone()))) {
    break;
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&rest.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&filler.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9]")); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_words(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("The quoted words this line is asking for, or \"\" when this line is not that ask.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(14)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_WORDS(), "search", args))(&__args) })) } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_its_own(args))(&__args) }) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::last_bot(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _quoted(args))(&__args) };
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let _ = py_str("This line is asking for the words the last answer already gave, so it is this module's.\n\n    The reply is a quote of the answer, so on the surface it looks like the bot saying the same\n    thing twice, and a general module that answers the same line about something else - a\n    birthday-card line for \"what should i say to her\" - is preferred to it by the specialist\n    fit. Declaring the line instead of raising PRIORITY says that for this reading only.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut words: Py = Py::None;
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_words(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("words".to_string(), words.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LINES()); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_compose::compose::pick(args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.words".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_WORDS" => Some(g_ASKS_WORDS()),
            "ASK_FILLER" => Some(g_ASK_FILLER()),
            "ASK_OBJECT" => Some(g_ASK_OBJECT()),
            "EXAMPLED" => Some(g_EXAMPLED()),
            "LINES" => Some(g_LINES()),
            "PRIORITY" => Some(g_PRIORITY()),
            "QUOTED" => Some(g_QUOTED()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "_names_its_own" => Some(py_func("_names_its_own", |args: &[Py]| _names_its_own(args))),
            "_quoted" => Some(py_func("_quoted", |args: &[Py]| _quoted(args))),
            "_the_words" => Some(py_func("_the_words", |args: &[Py]| _the_words(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS_WORDS" => s_ASKS_WORDS(v),
            "ASK_FILLER" => s_ASK_FILLER(v),
            "ASK_OBJECT" => s_ASK_OBJECT(v),
            "EXAMPLED" => s_EXAMPLED(v),
            "LINES" => s_LINES(v),
            "PRIORITY" => s_PRIORITY(v),
            "QUOTED" => s_QUOTED(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.words", module_obj);
    pyrt::xrefs::reg_fn("modules.words", "_quoted", py_func("_quoted", |args: &[Py]| _quoted(args)));
    pyrt::xrefs::reg_fn("modules.words", "_names_its_own", py_func("_names_its_own", |args: &[Py]| _names_its_own(args)));
    pyrt::xrefs::reg_fn("modules.words", "_the_words", py_func("_the_words", |args: &[Py]| _the_words(args)));
    pyrt::xrefs::reg_fn("modules.words", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_fn("modules.words", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.words", "ASKS_WORDS", g_ASKS_WORDS, s_ASKS_WORDS);
    pyrt::xrefs::reg_access("modules.words", "ASK_FILLER", g_ASK_FILLER, s_ASK_FILLER);
    pyrt::xrefs::reg_access("modules.words", "ASK_OBJECT", g_ASK_OBJECT, s_ASK_OBJECT);
    pyrt::xrefs::reg_access("modules.words", "EXAMPLED", g_EXAMPLED, s_EXAMPLED);
    pyrt::xrefs::reg_access("modules.words", "LINES", g_LINES, s_LINES);
    pyrt::xrefs::reg_access("modules.words", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.words", "QUOTED", g_QUOTED, s_QUOTED);
}
