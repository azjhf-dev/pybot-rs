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


pub static AN_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_WHAT_TO_DO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RECALL_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_REF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNNAMED_KINDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AN_END() -> Py { ensure_init(); AN_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_END(v: Py) { ensure_init(); if let Ok(mut w) = AN_END.write() { *w = v; } }
pub fn g_ASKS() -> Py { ensure_init(); ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS.write() { *w = v; } }
pub fn g_ASKS_WHAT_TO_DO() -> Py { ensure_init(); ASKS_WHAT_TO_DO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHAT_TO_DO(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_WHAT_TO_DO.write() { *w = v; } }
pub fn g_A_QUESTION() -> Py { ensure_init(); A_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = A_QUESTION.write() { *w = v; } }
pub fn g_A_RECALL_SAID() -> Py { ensure_init(); A_RECALL_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RECALL_SAID(v: Py) { ensure_init(); if let Ok(mut w) = A_RECALL_SAID.write() { *w = v; } }
pub fn g_A_START() -> Py { ensure_init(); A_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_START(v: Py) { ensure_init(); if let Ok(mut w) = A_START.write() { *w = v; } }
pub fn g_BACK_REF() -> Py { ensure_init(); BACK_REF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_REF(v: Py) { ensure_init(); if let Ok(mut w) = BACK_REF.write() { *w = v; } }
pub fn g_FRAME() -> Py { ensure_init(); FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = FRAME.write() { *w = v; } }
pub fn g_GIVES() -> Py { ensure_init(); GIVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVES(v: Py) { ensure_init(); if let Ok(mut w) = GIVES.write() { *w = v; } }
pub fn g_LEAD() -> Py { ensure_init(); LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = LEAD.write() { *w = v; } }
pub fn g_UNNAMED_KINDS() -> Py { ensure_init(); UNNAMED_KINDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNNAMED_KINDS(v: Py) { ensure_init(); if let Ok(mut w) = UNNAMED_KINDS.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The kind of detail a line asks for, and the earlier line of theirs that gives that kind.\n\n`bot/memory/lineback.py` is the one check for \"is there an earlier turn this line is about\", and\nits readers are matched on the words an ask shares with their lines. A whole family of asks shares\nno word with the line that answers them:\n\n    the plumber is coming tuesday at 10                              what time again\n    its a worcester bosch, about six years old                       and what make is the boiler\n    its a dulox one, the blue tin                                     what colour was it\n    we paid 28 pounds for it                                          how much was it again\n\nThe ask names the *kind* of detail it wants - the time, the make, the colour, the price - and the\nline gives that kind without ever using the word. So the question underneath is not \"which line\nshares a word with this ask\" but \"does an earlier line of theirs hold the kind of detail this line\nasks for, about the thing this line names (or, when it names nothing, about anything the chat has\nbeen on)\".\n\nThis module is the table for that question and nothing else: it reads, and never decides a reply.\n`bot/memory/lineback.py` asks it last, after the readers that match on words, and says the line\nback in its own voice.\n");
    s_ASKS(py_tuple(vec![py_tuple(vec![py_str("time"), stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+(?:time|hour)\\b|\\bthe\\s+(?:time|hour)\\b|\\b(?:what\\s+time|the\\s+time|the\\s+hour)\\s+(?:again|was\\s+it|is\\s+it|did\\s+i)\\b|\\bo'?clock\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("day"), stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+(?:day|date)\\b|\\bthe\\s+(?:day|date)\\b|\\b(?:what|which)\\s+(?:day|date)\\s+(?:again|was\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("when"), stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|also\\s+)?when\\b|\\bwhen\\s+(?:again|was\\s+it|is\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("make"), stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+(?:make|brand|model|manufacturer)\\b|\\b(?:make|brand|model)\\s+(?:again|is\\s+it|was\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("price"), stdlib_call("re", "compile", &[py_str("\\bhow\\s+much\\b|\\b(?:what|which)\\s+(?:price|cost)\\b|\\bthe\\s+(?:price|cost)\\b|\\b(?:what|how\\s+much)\\s+did\\s+(?:i|we|you)\\s+pay\\b|\\bhow\\s+much\\s+(?:again|was\\s+it|did\\s+i)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("duration"), stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("size"), stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+size\\b|\\bhow\\s+(?:big|tall|wide|heavy|far)\\b|\\b(?:what|which)\\s+size\\s+(?:again|was\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("age"), stdlib_call("re", "compile", &[py_str("\\bhow\\s+old\\b|\\b(?:what|which)\\s+age\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("colour"), stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+colou?r\\b|\\bcolou?r\\s+(?:again|was\\s+it|is\\s+it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("place"), stdlib_call("re", "compile", &[py_str("\\bwhere\\b|\\b(?:what|which)\\s+(?:place|town|city|airport)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("name"), stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+(?:name|make\\s+is\\s+it\\s+called)\\b|\\bwho\\s+(?:made|makes|built)\\b|\\bwhat'?s\\s+it\\s+called\\b|\\bwhat\\s+was\\s+it\\s+called\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))])]));
    s_GIVES(py_dict(vec![(py_str("time"), stdlib_call("re", "compile", &[py_str("\\b(?:at|by|around|about|from|until|till|before|after)\\s+\\d{1,2}(?::\\d\\d)?(?:\\s*(?:am|pm|a\\.m\\.|p\\.m\\.))?\\b|\\b\\d{1,2}(?::\\d\\d)?\\s*(?:am|pm)\\b|\\b(?:noon|midday|midnight)\\b|\\b(?:half|quarter)\\s+(?:past|to)\\s+\\w+|\\bo'?clock\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("day"), stdlib_call("re", "compile", &[py_str("\\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|tue|weds|wed|thurs|thu|fri|sat|sun|tomorrow|today|tonight|weekend|next\\s+(?:week|month|year)|this\\s+week|the\\s+\\d{1,2}(?:st|nd|rd|th)|\\d{1,2}(?:st|nd|rd|th)\\s+(?:of\\s+)?[a-z]+)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("make"), stdlib_call("re", "compile", &[py_str("\\b(?:it'?s|its|it\\s+is|i\\s+got|we\\s+got|i\\s+bought|we\\s+bought|they\\s+installed)\\s+(?:a|an)\\s+[a-z][\\w-]*(?:\\s+[a-z][\\w-]*)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("price"), stdlib_call("re", "compile", &[py_str("[£$€]\\s*\\d|\\b\\d[\\d,.]*\\s*(?:pounds?|quid|euros?|dollars?)\\b|\\b(?:costs?|cost\\s+me|paid|paying|charges?|quote[ds]?)\\b|\\b\\d[\\d,.]*\\s*(?:pounds?|quid|euros?|dollars?)\\s+a\\s+(?:month|week|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("size"), stdlib_call("re", "compile", &[py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:mm|cm|m|metres?|meters?|feet|foot|ft|inches|inch|in|kg|kilos?|grams?|g|stone|lb|lbs|litres?|liters?|miles?|ml)\\b|\\bsize\\s+\\w"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("age"), stdlib_call("re", "compile", &[py_str("\\b(?:\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty)\\s*(?:years?|months?|weeks?)\\s*(?:old)?\\s*(?=$|[.,;!?]|\\b(?:and|or|in|now|so)\\b)|\\baged\\s+\\d"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("place"), stdlib_call("re", "compile", &[py_str("\\b(?:in|at|from|to|near|outside|inside|via)\\s+(?!night|morning|afternoon|evening|all|least|once|times?|moment|end|same|other|top|bottom|front|back|side|way|case|fact|past|future|total|order|general|particular|short|full|half|the\\s+(?:end|moment|same|other|top|bottom|front|back|side|way|case|fact|total|order))[a-z][\\w'-]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("when"), stdlib_call("re", "compile", &[py_str("\\b(?:at|by|around)\\s+\\d{1,2}(?::\\d\\d)?(?!\\d)|\\b\\d{1,2}\\s*(?:am|pm)\\b|\\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|today|tonight|next\\s+week|this\\s+week)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("colour"), stdlib_call("re", "compile", &[py_str("\\b(?:red|blue|green|yellow|black|white|grey|gray|brown|pink|purple|orange|beige|cream|navy|silver|gold|teal|turquoise|maroon|olive|grey|charcoal)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("name"), stdlib_call("re", "compile", &[py_str("\\b(?:called|named)\\s+[a-z]|\\bname'?s?\\s+[a-z]|\\b(?:it'?s|its)\\s+[A-Z]"), Py::Int(0)]).unwrap_or_else(|e| exc::raise(e)))]));
    s_BACK_REF(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:it'?s|its|it\\s+is|they'?re|they\\s+are|that'?s|there'?s|there\\s+are|it\\s+was|was|is|are)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright|sorry|hmm|hi|hiya|hey|hello|now|then|again|quick\\s+one|mate)[ ,.!-]+)*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME(call_builtin("frozenset", &[mcall(&py_str("\na an the and but so or if my our your their his her its it this that these those of to in on at\nfor with from about again was were is are am be been being do does did done have has had\nwhat which who whom whose when where why how much many long old big tall wide heavy far\ntime hour day date make brand model manufacturer price cost size age number place town city\nairport name called colour color count weight height length say said tell told mention mentioned\npay paid paying cost costs costed charge charged spend spent buy bought\ni we you me us them they he she\njust only even ever before already first last next other another else more most still yet now\nplease pls can could would will shall may might should must cant cannot dont doesnt didnt\nremember recall forget forgot know knew knews remind mind think thought reckon guess\ngo goes going went gone come comes coming came been\none ones thing things stuff kind sort like\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RECALL_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:again|before|earlier|was\\s+(?:it|that)|did\\s+(?:i|we|you)|have\\s+i|you\\s+know|tell\\s+me|remind(?:ed)?\\s+me|the\\s+(?:time|hour|day|date|make|model|brand|price|cost|size|name|place|colour|color|number))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUESTION(stdlib_call("re", "compile", &[py_str("\\?\\s*$|^\\W*(?:what|which|who|whom|whose|when|where|why|how|is|are|was|were|do|does|did|can|could|should|would|will|shall|have|has|am|any|remind|tell\\s+me)(?:'?s)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_UNNAMED_KINDS(call_builtin("frozenset", &[mcall(&py_str("price colour size age make"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_END(stdlib_call("re", "compile", &[py_str("\\b(?:end|ends|ending|finish|finishes|finishing|last|lasts|leaves|leaving|close|closes|closing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_START(stdlib_call("re", "compile", &[py_str("\\b(?:start|starts|starting|started|begin|begins|beginning|began|open|opens|opening)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_WHAT_TO_DO(stdlib_call("re", "compile", &[py_str("\\b(?:do|does|should|can|could|would|shall|will|might)\\s+(?:i|we|you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_other_end(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut asking_end: Py = Py::None;
    let mut asking_start: Py = Py::None;
    let _ = py_str("Whether the ask and the line are about different ends of the same plan.");
    asking_end = call_builtin("bool", &[mcall(&g_AN_END(), "search", &[ask.clone()])]).unwrap_or_else(|e| exc::raise(e));
    asking_start = call_builtin("bool", &[mcall(&g_A_START(), "search", &[ask.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&asking_end.clone(), &asking_start.clone()))) {
    return Py::Bool(false);
    }
    return if py_truthy(&asking_end.clone()) { call_builtin("bool", &[mcall(&g_A_START(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("bool", &[mcall(&g_AN_END(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)) };
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    let mut length: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "length") { v.clone() }
    else { Py::Int(2) };
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of a line worth matching on, stemmed the way people vary them.");
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z][a-z'\\-]*"), _flat(&[line.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str("'-")]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FRAME(), &word.clone())) } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "add", &[word.clone()]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("s")]) } }) {
    let _ = mcall(&out.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
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
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut flat: Py = Py::None;
    let mut have: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line names one of the words, in the shape a line would hold it.");
    have = _words(&[line.clone()]);
    flat = _flat(&[line.clone()]);
    let __iter1 = py_iter_or_raise(&wanted.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&have.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat.clone(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&have.clone(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _their_lines(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Every line of theirs in the chat, oldest first, without their own questions.\n\n    Their question is what they asked, not something they told: quoting it back answers a question\n    with the same question (\"what size chicken now\" came back as You said: \"what size chicken do i\n    need\"). A count asked for is left out of the table for the same reason: how many of something\n    is worked out from the facts on file, and the line that carries one of the figures is not the\n    count that was asked for.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_QUESTION(), "match", &[line.clone()]) } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[line.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn the_kind_asked(args: &[Py]) -> Py {
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
    let _ = py_str("The kinds of detail the line asks for, in the order the table reads them.");
    low = _flat(&[text.clone()]);
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASKS()) { let __unpk3812 = py_unpack(&__item.clone(), 2, None); let mut kind: Py = __unpk3812[0].clone(); let mut pattern: Py = __unpk3812[1].clone(); if py_truthy(&mcall(&pattern.clone(), "search", &[low.clone()])) {  __out.push(kind.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn the_line_that_gives(args: &[Py]) -> Py {
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
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut asking_again: Py = Py::None;
    let mut gives: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut kinds: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The earlier line of theirs that holds the kind of detail the line asks for, or None.\n\n    Returned as a small reading - `{\"line\": ..., \"kind\": ...}` - so the caller phrases it. Nothing\n    comes back when no earlier line of theirs gives that kind, which is what keeps a question about\n    the world (\"how much does a new boiler cost\", in a chat that never gave a price) out of it.\n    ");
    kinds = the_kind_asked(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kinds.clone()))) {
    return Py::None;
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = lines.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut line: Py = __item.clone(); if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_QUESTION(), "match", &[line.clone()]))) } }) {  __out.push(line.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&lines.clone()))) {
    return Py::None;
    }
    low = mcall(&g_LEAD(), "sub", &[py_str(""), _flat(&[text.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_QUESTION(), "match", &[low.clone()])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ASKS_WHAT_TO_DO(), "search", &[low.clone()])) {
    return Py::None;
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z][a-z'\\-]*"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FRAME(), &word.clone())); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    asking_again = call_builtin("bool", &[mcall(&g_A_RECALL_SAID(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asking_again.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kinds.clone()) { let mut kind: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_UNNAMED_KINDS(), &kind.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&kinds.clone());
    for __item in __iter1 {
    kind = __item.clone();
    gives = mcall(&g_GIVES(), "get", &[kind.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&gives.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asking_again.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_UNNAMED_KINDS(), &kind.clone())) } } } } } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    index = __item.clone();
    line = geti(&lines.clone(), &index.clone());
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&gives.clone(), "search", &[line.clone()])))) {
    continue;
    }
    if py_truthy(&_the_other_end(&[low.clone(), line.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { _mentions(&[line.clone(), named.clone()]) } }) {
    return py_dict(vec![(py_str("line"), line.clone()), (py_str("kind"), kind.clone())]);
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_BACK_REF(), "match", &[line.clone()]); if !py_truthy(&__b) { __b } else { _mentions(&[geti(&lines.clone(), &sub(&index.clone(), &Py::Int(1))), named.clone()]) } } } }) {
    return py_dict(vec![(py_str("line"), line.clone()), (py_str("kind"), kind.clone())]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.kinds".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_END" => Some(g_AN_END()),
            "ASKS" => Some(g_ASKS()),
            "ASKS_WHAT_TO_DO" => Some(g_ASKS_WHAT_TO_DO()),
            "A_QUESTION" => Some(g_A_QUESTION()),
            "A_RECALL_SAID" => Some(g_A_RECALL_SAID()),
            "A_START" => Some(g_A_START()),
            "BACK_REF" => Some(g_BACK_REF()),
            "FRAME" => Some(g_FRAME()),
            "GIVES" => Some(g_GIVES()),
            "LEAD" => Some(g_LEAD()),
            "UNNAMED_KINDS" => Some(g_UNNAMED_KINDS()),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_mentions" => Some(py_func("_mentions", |args: &[Py]| _mentions(args))),
            "_the_other_end" => Some(py_func("_the_other_end", |args: &[Py]| _the_other_end(args))),
            "_their_lines" => Some(py_func("_their_lines", |args: &[Py]| _their_lines(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "the_kind_asked" => Some(py_func("the_kind_asked", |args: &[Py]| the_kind_asked(args))),
            "the_line_that_gives" => Some(py_func("the_line_that_gives", |args: &[Py]| the_line_that_gives(args))),
            _ => None,
        }),
        set: None,
    }))
}
