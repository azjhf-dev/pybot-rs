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


thread_local! { static G_ASKS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_BACK_A_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_AMOUNT_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ATTRIBUTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_KIND_FOR_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MY_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PAST_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SAID_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TELLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THE_CALENDAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VALUE_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_VALUE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INTENTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIND_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEMORY_READERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHORT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VAGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKS_BACK() -> Py { ensure_init(); G_ASKS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_BACK(v: Py) { ensure_init(); G_ASKS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_BACK_A_KIND() -> Py { ensure_init(); G_ASKS_BACK_A_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_BACK_A_KIND(v: Py) { ensure_init(); G_ASKS_BACK_A_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_AMOUNT_ASKED() -> Py { ensure_init(); G__AN_AMOUNT_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_AMOUNT_ASKED(v: Py) { ensure_init(); G__AN_AMOUNT_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ATTRIBUTE() -> Py { ensure_init(); G__AN_ATTRIBUTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ATTRIBUTE(v: Py) { ensure_init(); G__AN_ATTRIBUTE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIGURE() -> Py { ensure_init(); G__A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); G__A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_KIND() -> Py { ensure_init(); G__A_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_KIND(v: Py) { ensure_init(); G__A_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__A_KIND_FOR_WORD() -> Py { ensure_init(); G__A_KIND_FOR_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_KIND_FOR_WORD(v: Py) { ensure_init(); G__A_KIND_FOR_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MY_VALUE() -> Py { ensure_init(); G__A_MY_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MY_VALUE(v: Py) { ensure_init(); G__A_MY_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PAST_ACT() -> Py { ensure_init(); G__A_PAST_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PAST_ACT(v: Py) { ensure_init(); G__A_PAST_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SAID_AGAIN() -> Py { ensure_init(); G__A_SAID_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SAID_AGAIN(v: Py) { ensure_init(); G__A_SAID_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TELLING() -> Py { ensure_init(); G__A_TELLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TELLING(v: Py) { ensure_init(); G__A_TELLING.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THE_CALENDAR() -> Py { ensure_init(); G__A_THE_CALENDAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THE_CALENDAR(v: Py) { ensure_init(); G__A_THE_CALENDAR.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VALUE_START() -> Py { ensure_init(); G__A_VALUE_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VALUE_START(v: Py) { ensure_init(); G__A_VALUE_START.with(|c| *c.borrow_mut() = v); }
pub fn g__A_VALUE_VERB() -> Py { ensure_init(); G__A_VALUE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_VALUE_VERB(v: Py) { ensure_init(); G__A_VALUE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__INTENTION() -> Py { ensure_init(); G__INTENTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INTENTION(v: Py) { ensure_init(); G__INTENTION.with(|c| *c.borrow_mut() = v); }
pub fn g__KIND_WORDS() -> Py { ensure_init(); G__KIND_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIND_WORDS(v: Py) { ensure_init(); G__KIND_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__MEMORY_READERS() -> Py { ensure_init(); G__MEMORY_READERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEMORY_READERS(v: Py) { ensure_init(); G__MEMORY_READERS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_DETAIL() -> Py { ensure_init(); G__NOT_A_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_DETAIL(v: Py) { ensure_init(); G__NOT_A_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__QUESTION() -> Py { ensure_init(); G__QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__QUESTION(v: Py) { ensure_init(); G__QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__SHORT_WORD() -> Py { ensure_init(); G__SHORT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHORT_WORD(v: Py) { ensure_init(); G__SHORT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_ASK() -> Py { ensure_init(); G__THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_ASK(v: Py) { ensure_init(); G__THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__VAGUE() -> Py { ensure_init(); G__VAGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VAGUE(v: Py) { ensure_init(); G__VAGUE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A detail the person gave earlier, found again from the words of the ask that wants it back.\n\nLate in a chat people ask for a thing they said themselves:\n\n    its on the outside of the knee, just under the bone      (turn 2)\n    ...\n    and which knee was it                                   -> \"You said: it's on the outside\n                                                               of the knee, just under the bone\"\n\n`bot/memory/recall.py` reads the asks whose subject is a fact the store keeps (a name, a job, a\nbudget), and the store says that fact back. Every other detail - which side a knee is on, what\nsubject they are revising, the shift pattern they work - is in no fact key, so those asks were\nleft to the entries and answered with a card about something else or with a search hedge.\n\nThis reads them instead, without filing anything: the person's own lines are in\n`memory[\"turns\"]`, so the ask's content words are matched against what they typed, and the line\nthat carries them is said back. It is deliberately narrow:\n\n* the line has to be a recall ask as people phrase it (\"what subject did i say i was revising\",\n  \"which knee was it\", \"what was my deposit target again\", \"remind me what i was going to ask\n  for\"), so a knowledge question (\"which knee supports the body\") is never this module's;\n* a line that is itself a recall ask is never the answer - \"what did i say about the days we\n  have\" is answered from the days, not from an earlier \"remind me how many days we have\";\n* one content word of three letters or more has to be shared, and the words the ask is made of\n  (\"what\", \"did\", \"say\", \"again\") are not content, so \"what did i say\" on its own finds nothing\n  and the line goes to whoever answered it before.\n");
    s__KIND_WORDS(py_tuple(vec![py_str("size"), py_str("time"), py_str("date"), py_str("day"), py_str("price"), py_str("cost"), py_str("number"), py_str("weight"), py_str("height"), py_str("dose")]));
    s__A_KIND(py_dict(vec![(py_str("size"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:in\\s+(?:a|an)|size)\\s+(xs|s|m|l|xl|xxl|small|medium|large|extra large)\\b|\\b(?:was|is|its|it's|make\\s+it|actually|instead)\\s+(?:a\\s+)?(xs|xl|xxl|small|medium|large|extra\\s+large)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("time"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:at|from)\\s+(\\d{1,2}(?:[:.]\\d{2})?\\s?(?:am|pm)?|half\\s+\\w+|midday|noon|midnight)\\b|\\b(\\d{1,2}(?:[:.]\\d{2})?\\s?(?:am|pm))\\b|\\b(half\\s+(?:past\\s+)?(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\\b|\\b(midday|noon|midnight)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("day"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\b(?:%s)\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("tomorrow"), py_str("tonight")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) })); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("date"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:the\\s+)?((?:0?[1-9]|[12][0-9]|3[01])(?:st|nd|rd|th))\\b|\\b(?:in|on)\\s+(january|february|march|april|may|june|july|august|september|october|november|december)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("price"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:£|\\$|€)\\s?\\d[\\d,]*(?:\\.\\d+)?|\\b\\d[\\d,]*(?:\\.\\d+)?\\s?(?:quid|pounds|euros|dollars|grand)\\b|\\b(?:quoted|paid|charged|cost)\\s+(?:me\\s+)?(?:£|\\$|€)?\\s?\\d[\\d,]*")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("weight"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:\\.\\d+)?\\s?(?:kg|kilos?|stone|st|lbs?|pounds?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("height"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:\\.\\d+)?\\s?(?:cm|ft|feet|foot|inches|in)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("dose"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:\\.\\d+)?\\s?(?:mg|ml|mcg|micrograms?|g)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s__A_KIND_FOR_WORD(py_dict(vec![(py_str("size"), py_str("size")), (py_str("time"), py_str("time")), (py_str("date"), py_str("date")), (py_str("day"), py_str("day")), (py_str("birthday"), py_str("date")), (py_str("price"), py_str("price")), (py_str("cost"), py_str("price")), (py_str("number"), py_str("price")), (py_str("weight"), py_str("weight")), (py_str("paid"), py_str("price")), (py_str("pay"), py_str("price")), (py_str("spend"), py_str("price")), (py_str("spent"), py_str("price")), (py_str("height"), py_str("height")), (py_str("dose"), py_str("dose"))]));
    s_ASKS_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|which|who|when|where|how\\s+\\w+)\\b[^?!]{0,40}?\\bdid\\s+i\\s+(?:say|said|tell\\s+you|told\\s+you|mention|mentioned)\\b|\\b(?:what|which)\\s+(?:[a-z']+\\s+){1,3}?(?:was|is|were)\\s+it\\b|\\bwhat(?:'s|\\s+is|\\s+was)\\s+my\\s+(?:[a-z']+\\s+){1,3}?again\\b|\\bremind\\s+me\\s+(?:what|which|when|where|who|how\\s+many|how\\s+much)\\b|\\bwhat\\s+did\\s+i\\s+say\\b|\\bwhat\\s+(?:was|were|am|is)\\s+(?:i|we)\\s+going\\s+to\\s+\\w+|\\bwhat\\s+(?:was|is)\\s+it\\s+(?:he|she|they|we|i|you)\\b|\\bhow\\s+(?:much|many)\\b[^?!]{0,25}?\\bdid\\s+i\\s+(?:say|said|need|want|ask)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AN_ATTRIBUTE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:age|ages|old|size|name|price|cost|weight|height|colour|color|birthday|date|time|number|address|surname|title)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nwhat which who whom whose when where why how remind me again say said saying says tell told\ntelling mention mentioned i you my our we us it its that this these those was were is are be\nbeen being did do does done about of to the a an and or for with in on at by as if so but not\nany some please just really thing things one ones kind sort name called exact exactly already\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__VAGUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\ngo going went get got getting do doing done make making take taking put thing things way bit\nlot time now then well good bad sure really quite maybe first last next other another same\nback again more most much many little big small old new long short like want wanted need\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_A_DETAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+(?:old|many|much|long|often|far|heavy|tall)\\b|\\bwhat\\s+time\\b|\\bwhat\\s+(?:day|date)\\b|\\bwhen\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SHORT_WORD(Py::Int(3));
    s__QUESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:what|which|who|when|where|why|how|should|shall|can|could|would|will|do|does|did|is|are|am|have|has|if|any)\\b|\\?\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__INTENTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:was|were|am|is|going)\\s+(?:i\\s+)?(?:going\\s+)?to\\s+(?:ask|say|do|tell|mention)\\b|\\bwhat\\s+(?:i\\s+)?was\\s+going\\s+to\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_BACK_A_KIND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\b(?:what|which)\\s+(?:%(kind)s)\\b[^?!]{0,22}?\\bagain\\b|\\bthe\\s+(?:%(kind)s)\\s+(?:(?:that\\s+)?(?:i|we)\\s+(?:said|gave|told\\s+you|mentioned)|again)\\b|\\b(?:did|have)\\s+(?:i|we)\\b[^?!]{0,30}?\\b(?:%(kind)s)\\b|\\bhow\\s+(?:much|many)\\b[^?!]{0,25}?\\bagain\\b"), &py_dict(vec![(py_str("kind"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__A_KIND_FOR_WORD()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) })]))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_TELLING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:was|is|were|are|should\\s+be|came\\s+in|goes|needs|will\\s+be)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PAST_ACT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:did|have)\\s+(?:i|we)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MY_VALUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bmy\\s+(?:[a-z']+\\s+){0,3}?(?:is|was)\\s+(?P<value>[^.;,?]+?)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_VALUE_START({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:[0-9]|a\\b|an\\b|the\\b|on\\b|in\\b|at\\b|for\\b|about\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_VALUE_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:quoted|paid|charged|cost|costs|size|in\\s+(?:a|an)|at|from)\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_THE_CALENDAR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:what|which)\\s+(?:day|date|time)\\s+is\\s+it\\s*\\??$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_SAID_AGAIN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:again|before|earlier|did\\s+(?:i|we)|have\\s+i|you\\s+know|tell\\s+me|remind)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_FIGURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\u00a3$\\u20ac]\\s?\\d[\\d.,]*|\\b\\d[\\d.,]*\\s?(?:%|g|kg|grams?|kilos?|ml|cl|litres?|liters?|pints?|oz|lbs?|tbsp|tsp|cups?|mugs?|slices?|sheets?|hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?|miles?|km|metres?|meters?|pounds?|quid|dollars?|euros?|degrees?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AN_AMOUNT_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+(?:much|many)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__MEMORY_READERS(py_tuple(vec![g__the_value_of_the_kind(), g__the_figure_for_a_thing(), g__the_line_itself()]));
}
pub fn flat(args: &[Py]) -> Py {
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
    let _ = py_str("The line as typed, with the spacing evened out.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut word: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    Py::None
}
pub fn _stem(args: &[Py]) -> Py {
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
    let mut suffix: Py = Py::None;
    let _ = py_str("A crude shared stem, so \"revising\" answers \"revise\" and \"shifts\" answers \"shift\".");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ies"), py_str("es"), py_str("ed"), py_str("s")]));
    for __item in __iter1 {
    suffix = __item.clone();
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(suffix.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g__SHORT_WORD())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(suffix.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(suffix.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None);
    }
    }
    return word.clone();
    Py::None
}
pub fn about(args: &[Py]) -> Py {
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
    let _ = py_str("The content words of a recall ask, as stems: what the line wants said back.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__THE_ASK(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__VAGUE(), &word.clone())) } }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stem(args))(&__args) }); } } py_set(__out) })() };
    Py::None
}
pub fn _a_kind_named(args: &[Py]) -> Py {
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
    let _ = py_str("The first attribute word the line names, or None.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__A_KIND_FOR_WORD(), &word.clone()))) {
    return word.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn is_a_recall_ask(args: &[Py]) -> Py {
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
    let mut flat_text: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("Whether the line asks for something said earlier in this chat.\n\n    One check for every memory-based reply. The shapes that name what they want said back\n    (`what did i say about the boiler`), the shapes that name the attribute and say `again`\n    (`what size again`, `the size i said`), and the attribute named with the person's own past act\n    (`what size did i get`, `how much did i pay`) are all this one question.\n    ");
    flat_text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(flat_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_BACK(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(flat_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_BACK_A_KIND(), "search", args))(&__args) };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&flat_text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_TELLING(), "match", args))(&__args) })) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(flat_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PAST_ACT(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(flat_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_kind_named(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+much\\b")); __args.push(flat_text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_last_reply(args: &[Py]) -> Py {
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
    let _ = py_str("The bot's own last line, in the same shape as the person's lines.");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { turns.clone() } else { py_tuple(vec![]) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    }
    }
    return py_str("");
    Py::None
}
pub fn the_lines(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own lines this chat, oldest first, without the line being answered.");
    said = py_list(vec![]);
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    let __iter1 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { turns.clone() } else { py_tuple(vec![]) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    if py_truthy(&line.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    }
    return said.clone();
    Py::None
}
pub fn _the_best_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut wanted: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut questions: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "questions") { v.clone() }
    else { Py::Bool(false) };
    let mut best: Py = Py::None;
    let mut line: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut place: Py = Py::None;
    let mut score: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The earlier line that carries most of what the ask is about, or None.");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    place = __unpk2[0].clone();
    line = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| is_a_recall_ask(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__QUESTION(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&questions.clone())) } }) {
    continue;
    }
    shared = op_bitand(&wanted.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about_any(args))(&__args) });
    if py_truthy(&Py::Bool(!py_truthy(&shared.clone()))) {
    continue;
    }
    longest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&shared.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_lt(&longest.clone(), &g__SHORT_WORD())) {
    continue;
    }
    score = py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(shared.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, longest.clone(), place.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&score.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![score.clone(), line.clone()]);
    }
    }
    return if py_truthy(&best.clone()) { geti(&best.clone(), &Py::Int(1)) } else { Py::None };
    Py::None
}
pub fn about_any(args: &[Py]) -> Py {
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
    let _ = py_str("Every stem the line carries, content word or not: what it can be found by.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__THE_ASK(), &word.clone())); if !py_truthy(&__b) { __b } else { op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g__SHORT_WORD()) } }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stem(args))(&__args) }); } } py_set(__out) })() };
    Py::None
}
pub fn _the_kind_asked(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The kind of value the ask is about, when the ask is about the attribute and nothing else.");
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_THE_CALENDAR(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SAID_AGAIN(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g__A_KIND_FOR_WORD(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow\\s+much\\b")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about(args))(&__args) } } }) {
    return Py::None;
    }
    return py_str("price");
    }
    if py_truthy(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about(args))(&__args) }, &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stem(args))(&__args) }); } py_set(__out) })() })) {
    return Py::None;
    }
    return geti(&g__A_KIND_FOR_WORD(), &geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0)));
    Py::None
}
pub fn _a_value_of_the_kind(args: &[Py]) -> Py {
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
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut group: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The last value of that kind the person gave, said back, or None when they gave none.");
    pattern = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_KIND(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&pattern.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    line = __item.clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    groups = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "groups", args))(&__args) }) { let mut group: Py = __item.clone(); if py_truthy(&group.clone()) {  __out.push(group.clone()); } } py_list(__out) })() };
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,;")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&if py_truthy(&groups.clone()) { geti(&groups.clone(), &Py::Int(0)) } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } }, "strip", args))(&__args) };
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,;")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_VALUE_VERB(), "sub", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9]")); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&value.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } }) {
    return value.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_value(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The value of a \"my <thing> is <value>\" line, or None when the line is not that shape.");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MY_VALUE(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9]")); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&value.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)) } }) {
    return Py::None;
    }
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_VALUE_START(), "match", args))(&__args) }) { value.clone() } else { Py::None };
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut holding: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The sentence of the line that carries what the ask is about, for a long line.");
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(120))) {
    return line.clone();
    }
    parts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?;])\\s+|,\\s+(?=and\\b|but\\b)")); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    holding = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&op_bitand(&wanted.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about_any(args))(&__args) })) {  __out.push(part.clone()); } } py_list(__out) })() };
    return if py_truthy(&holding.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ,.;")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&holding.clone(), &Py::Int(0)), "strip", args))(&__args) } } else { line.clone() };
    Py::None
}
pub fn _the_value_of_the_kind(args: &[Py]) -> Py {
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
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut kind: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The value of the attribute the ask names, when the ask names the attribute and nothing else.\n\n    This is the reader that needs no word in common with the line: \"what size did i say\" and the\n    earlier \"my coat came in a medium\" share nothing, and the answer is still medium.\n    ");
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_kind_asked(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_value_of_the_kind(args))(&__args) };
    return if py_truthy(&value.clone()) { op_add(&op_add(&py_str("You said "), &value.clone()), &py_str(".")) } else { Py::None };
    Py::None
}
pub fn _the_figure_for_a_thing(args: &[Py]) -> Py {
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
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The figure they gave for the thing the ask names, when the ask is \"how much <thing>\".\n\n    \"how much mince did i say i needed\" has no kind of its own - mince is not a size or a time -\n    so the kind table cannot answer it, and the line itself is the wrong answer because the figure\n    is buried in it. The last line that carries the thing and a figure is the one they mean.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_AMOUNT_ASKED(), "search", args))(&__args) }))) {
    return Py::None;
    }
    wanted = op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about(args))(&__args) }, &g__VAGUE());
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&wanted.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about_any(args))(&__args) })))) {
    continue;
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIGURE(), "search", args))(&__args) };
    if py_truthy(&found.clone()) {
    return op_add(&op_add(&py_str("You said "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "rstrip", args))(&__args) }), &py_str("."));
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_line_itself(args: &[Py]) -> Py {
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
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answered: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The earlier line that carries what the ask is about, quoted back or said as its value.\n\n    A figure or a date is not a detail said back by quoting a line: \"how old did i say she was\"\n    would quote whichever line shared a word with it, so a kind of value with no reader of its own\n    is left to whoever answers for it.\n    ");
    figure = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_A_DETAIL(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = figure.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_kind_asked(args))(&__args) })) } }) {
    return Py::None;
    }
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    line = if py_truthy(&figure.clone()) { Py::None } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(wanted.clone()); __args.push(lines.clone()); __kws.extend(vec![("questions".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__INTENTION(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_best_line(args))(&__args) } };
    if py_truthy(&{ let __b = figure.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&line.clone())) } }) {
    return Py::None;
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_ATTRIBUTE(), "fullmatch", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _stem(args))(&__args) }); } } py_set(__out) })() };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&named.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about_any(args))(&__args) }))) } }) {
    return Py::None;
    }
    answered = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_last_reply(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about_any(args))(&__args) };
    if py_truthy(&op_bitand(&op_bitand(&wanted.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about_any(args))(&__args) }), &answered.clone())) {
    return Py::None;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_value(args))(&__args) };
    if py_truthy(&value.clone()) {
    return op_add(&op_add(&py_str("You said "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&value.clone(), "rstrip", args))(&__args) }), &py_str("."));
    }
    return op_add(&op_add(&py_str("You said: \""), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(wanted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _carried(args))(&__args) }), &py_str("\""));
    Py::None
}
pub fn _the_kind_they_gave(args: &[Py]) -> Py {
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
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let _ = py_str("The earlier line that holds the kind of detail this line asks for, said back.\n\n    The readers above are matched on the words an ask shares with their lines, and a whole family\n    of asks shares none: \"and what make is the boiler\" against \"its a worcester bosch\", \"what\n    colour was it\" against \"its a dulux one, the blue tin\". The ask names the kind of detail it\n    wants and the line gives it without the word. The table for that is `bot/memory/kinds.py`; the\n    line it finds is quoted in the voice the other readers use.\n    ");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::kinds::the_line_that_gives(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return op_add(&op_add(&py_str("You said: \""), &geti(&found.clone(), &py_str("line"))), &py_str("\""));
    Py::None
}
pub fn the_kind_they_gave(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The earlier line of theirs that gives the kind of detail this line asks for, or None.\n\n    The reading behind `said_back`'s reply, for a caller that has to know whether the chat answers\n    the line before it lets something else answer it.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_THE_CALENDAR(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SAID_AGAIN(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::kinds::the_line_that_gives(args))(&__args) };
    Py::None
}
pub fn an_earlier_turn(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut reader: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Is this line asking for something said earlier, and what was said?\n\n    Returns the reply that says it back, or None when the line is not asking that. Every\n    memory-based reply in the bot is meant to come through here, so \"is there an earlier turn?\"\n    is one check with one answer rather than a rule in each module.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| flat(args))(&__args) };
    lines = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines(args))(&__args) };
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&lines.clone(), &op_neg(&Py::Int(1))), &said.clone())) } }) {
    lines = slice_or_raise(&lines.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| is_a_recall_ask(args))(&__args) }))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(lines.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_kind_they_gave(args))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&g__MEMORY_READERS());
    for __item in __iter1 {
    reader = __item.clone();
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(lines.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&reader, args))(&__args) };
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(lines.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_kind_they_gave(args))(&__args) };
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let _ = py_str("What to say when the line asks again for something the person gave earlier, or None.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| an_earlier_turn(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.lineback".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS_BACK" => Some(g_ASKS_BACK()),
            "ASKS_BACK_A_KIND" => Some(g_ASKS_BACK_A_KIND()),
            "_AN_AMOUNT_ASKED" => Some(g__AN_AMOUNT_ASKED()),
            "_AN_ATTRIBUTE" => Some(g__AN_ATTRIBUTE()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_A_KIND" => Some(g__A_KIND()),
            "_A_KIND_FOR_WORD" => Some(g__A_KIND_FOR_WORD()),
            "_A_MY_VALUE" => Some(g__A_MY_VALUE()),
            "_A_PAST_ACT" => Some(g__A_PAST_ACT()),
            "_A_SAID_AGAIN" => Some(g__A_SAID_AGAIN()),
            "_A_TELLING" => Some(g__A_TELLING()),
            "_A_THE_CALENDAR" => Some(g__A_THE_CALENDAR()),
            "_A_VALUE_START" => Some(g__A_VALUE_START()),
            "_A_VALUE_VERB" => Some(g__A_VALUE_VERB()),
            "_INTENTION" => Some(g__INTENTION()),
            "_KIND_WORDS" => Some(g__KIND_WORDS()),
            "_MEMORY_READERS" => Some(g__MEMORY_READERS()),
            "_NOT_A_DETAIL" => Some(g__NOT_A_DETAIL()),
            "_QUESTION" => Some(g__QUESTION()),
            "_SHORT_WORD" => Some(g__SHORT_WORD()),
            "_THE_ASK" => Some(g__THE_ASK()),
            "_VAGUE" => Some(g__VAGUE()),
            "_a_kind_named" => Some(py_func("_a_kind_named", |args: &[Py]| _a_kind_named(args))),
            "_a_value_of_the_kind" => Some(py_func("_a_value_of_the_kind", |args: &[Py]| _a_value_of_the_kind(args))),
            "_carried" => Some(py_func("_carried", |args: &[Py]| _carried(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_the_best_line" => Some(py_func("_the_best_line", |args: &[Py]| _the_best_line(args))),
            "_the_figure_for_a_thing" => Some(py_func("_the_figure_for_a_thing", |args: &[Py]| _the_figure_for_a_thing(args))),
            "_the_kind_asked" => Some(py_func("_the_kind_asked", |args: &[Py]| _the_kind_asked(args))),
            "_the_kind_they_gave" => Some(py_func("_the_kind_they_gave", |args: &[Py]| _the_kind_they_gave(args))),
            "_the_last_reply" => Some(py_func("_the_last_reply", |args: &[Py]| _the_last_reply(args))),
            "_the_line_itself" => Some(py_func("_the_line_itself", |args: &[Py]| _the_line_itself(args))),
            "_the_value" => Some(py_func("_the_value", |args: &[Py]| _the_value(args))),
            "_the_value_of_the_kind" => Some(py_func("_the_value_of_the_kind", |args: &[Py]| _the_value_of_the_kind(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "about" => Some(py_func("about", |args: &[Py]| about(args))),
            "about_any" => Some(py_func("about_any", |args: &[Py]| about_any(args))),
            "an_earlier_turn" => Some(py_func("an_earlier_turn", |args: &[Py]| an_earlier_turn(args))),
            "flat" => Some(py_func("flat", |args: &[Py]| flat(args))),
            "is_a_recall_ask" => Some(py_func("is_a_recall_ask", |args: &[Py]| is_a_recall_ask(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_kind_they_gave" => Some(py_func("the_kind_they_gave", |args: &[Py]| the_kind_they_gave(args))),
            "the_lines" => Some(py_func("the_lines", |args: &[Py]| the_lines(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS_BACK" => s_ASKS_BACK(v),
            "ASKS_BACK_A_KIND" => s_ASKS_BACK_A_KIND(v),
            "_AN_AMOUNT_ASKED" => s__AN_AMOUNT_ASKED(v),
            "_AN_ATTRIBUTE" => s__AN_ATTRIBUTE(v),
            "_A_FIGURE" => s__A_FIGURE(v),
            "_A_KIND" => s__A_KIND(v),
            "_A_KIND_FOR_WORD" => s__A_KIND_FOR_WORD(v),
            "_A_MY_VALUE" => s__A_MY_VALUE(v),
            "_A_PAST_ACT" => s__A_PAST_ACT(v),
            "_A_SAID_AGAIN" => s__A_SAID_AGAIN(v),
            "_A_TELLING" => s__A_TELLING(v),
            "_A_THE_CALENDAR" => s__A_THE_CALENDAR(v),
            "_A_VALUE_START" => s__A_VALUE_START(v),
            "_A_VALUE_VERB" => s__A_VALUE_VERB(v),
            "_INTENTION" => s__INTENTION(v),
            "_KIND_WORDS" => s__KIND_WORDS(v),
            "_MEMORY_READERS" => s__MEMORY_READERS(v),
            "_NOT_A_DETAIL" => s__NOT_A_DETAIL(v),
            "_QUESTION" => s__QUESTION(v),
            "_SHORT_WORD" => s__SHORT_WORD(v),
            "_THE_ASK" => s__THE_ASK(v),
            "_VAGUE" => s__VAGUE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.lineback", module_obj);
    pyrt::xrefs::reg_fn("memory.lineback", "flat", py_func("flat", |args: &[Py]| flat(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_stem", py_func("_stem", |args: &[Py]| _stem(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "about", py_func("about", |args: &[Py]| about(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_a_kind_named", py_func("_a_kind_named", |args: &[Py]| _a_kind_named(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "is_a_recall_ask", py_func("is_a_recall_ask", |args: &[Py]| is_a_recall_ask(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_the_last_reply", py_func("_the_last_reply", |args: &[Py]| _the_last_reply(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "the_lines", py_func("the_lines", |args: &[Py]| the_lines(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_the_best_line", py_func("_the_best_line", |args: &[Py]| _the_best_line(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "about_any", py_func("about_any", |args: &[Py]| about_any(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_the_kind_asked", py_func("_the_kind_asked", |args: &[Py]| _the_kind_asked(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_a_value_of_the_kind", py_func("_a_value_of_the_kind", |args: &[Py]| _a_value_of_the_kind(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_the_value", py_func("_the_value", |args: &[Py]| _the_value(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_carried", py_func("_carried", |args: &[Py]| _carried(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_the_value_of_the_kind", py_func("_the_value_of_the_kind", |args: &[Py]| _the_value_of_the_kind(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_the_figure_for_a_thing", py_func("_the_figure_for_a_thing", |args: &[Py]| _the_figure_for_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_the_line_itself", py_func("_the_line_itself", |args: &[Py]| _the_line_itself(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "_the_kind_they_gave", py_func("_the_kind_they_gave", |args: &[Py]| _the_kind_they_gave(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "the_kind_they_gave", py_func("the_kind_they_gave", |args: &[Py]| the_kind_they_gave(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "an_earlier_turn", py_func("an_earlier_turn", |args: &[Py]| an_earlier_turn(args)));
    pyrt::xrefs::reg_fn("memory.lineback", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.lineback", "ASKS_BACK", g_ASKS_BACK, s_ASKS_BACK);
    pyrt::xrefs::reg_access("memory.lineback", "ASKS_BACK_A_KIND", g_ASKS_BACK_A_KIND, s_ASKS_BACK_A_KIND);
    pyrt::xrefs::reg_access("memory.lineback", "_AN_AMOUNT_ASKED", g__AN_AMOUNT_ASKED, s__AN_AMOUNT_ASKED);
    pyrt::xrefs::reg_access("memory.lineback", "_AN_ATTRIBUTE", g__AN_ATTRIBUTE, s__AN_ATTRIBUTE);
    pyrt::xrefs::reg_access("memory.lineback", "_A_FIGURE", g__A_FIGURE, s__A_FIGURE);
    pyrt::xrefs::reg_access("memory.lineback", "_A_KIND", g__A_KIND, s__A_KIND);
    pyrt::xrefs::reg_access("memory.lineback", "_A_KIND_FOR_WORD", g__A_KIND_FOR_WORD, s__A_KIND_FOR_WORD);
    pyrt::xrefs::reg_access("memory.lineback", "_A_MY_VALUE", g__A_MY_VALUE, s__A_MY_VALUE);
    pyrt::xrefs::reg_access("memory.lineback", "_A_PAST_ACT", g__A_PAST_ACT, s__A_PAST_ACT);
    pyrt::xrefs::reg_access("memory.lineback", "_A_SAID_AGAIN", g__A_SAID_AGAIN, s__A_SAID_AGAIN);
    pyrt::xrefs::reg_access("memory.lineback", "_A_TELLING", g__A_TELLING, s__A_TELLING);
    pyrt::xrefs::reg_access("memory.lineback", "_A_THE_CALENDAR", g__A_THE_CALENDAR, s__A_THE_CALENDAR);
    pyrt::xrefs::reg_access("memory.lineback", "_A_VALUE_START", g__A_VALUE_START, s__A_VALUE_START);
    pyrt::xrefs::reg_access("memory.lineback", "_A_VALUE_VERB", g__A_VALUE_VERB, s__A_VALUE_VERB);
    pyrt::xrefs::reg_access("memory.lineback", "_INTENTION", g__INTENTION, s__INTENTION);
    pyrt::xrefs::reg_access("memory.lineback", "_KIND_WORDS", g__KIND_WORDS, s__KIND_WORDS);
    pyrt::xrefs::reg_access("memory.lineback", "_MEMORY_READERS", g__MEMORY_READERS, s__MEMORY_READERS);
    pyrt::xrefs::reg_access("memory.lineback", "_NOT_A_DETAIL", g__NOT_A_DETAIL, s__NOT_A_DETAIL);
    pyrt::xrefs::reg_access("memory.lineback", "_QUESTION", g__QUESTION, s__QUESTION);
    pyrt::xrefs::reg_access("memory.lineback", "_SHORT_WORD", g__SHORT_WORD, s__SHORT_WORD);
    pyrt::xrefs::reg_access("memory.lineback", "_THE_ASK", g__THE_ASK, s__THE_ASK);
    pyrt::xrefs::reg_access("memory.lineback", "_VAGUE", g__VAGUE, s__VAGUE);
}
