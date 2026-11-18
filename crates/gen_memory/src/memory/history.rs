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


thread_local! { static G_ANSWERS_AN_EARLIER_TURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_YES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THEY_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIMS_A_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_SAID_BY_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREETING_WITH_AN_ADDRESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GREETS_A_RETURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_SAID_ABOUT_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_CONTENT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMIND_ME_RECAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_ABOUT_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STORE_SUMMARY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TALKED_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_WE_WERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS_AN_EARLIER_TURN() -> Py { ensure_init(); G_ANSWERS_AN_EARLIER_TURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS_AN_EARLIER_TURN(v: Py) { ensure_init(); G_ANSWERS_AN_EARLIER_TURN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_YES() -> Py { ensure_init(); G_A_BARE_YES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_YES(v: Py) { ensure_init(); G_A_BARE_YES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION() -> Py { ensure_init(); G_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION(v: Py) { ensure_init(); G_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THEY_ASKED() -> Py { ensure_init(); G_A_THEY_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THEY_ASKED(v: Py) { ensure_init(); G_A_THEY_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WANT() -> Py { ensure_init(); G_A_WANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WANT(v: Py) { ensure_init(); G_A_WANT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_QUESTION() -> Py { ensure_init(); G_CHAT_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_QUESTION(v: Py) { ensure_init(); G_CHAT_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIMS_A_PAST() -> Py { ensure_init(); G_CLAIMS_A_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIMS_A_PAST(v: Py) { ensure_init(); G_CLAIMS_A_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_SAID_BY_BOT() -> Py { ensure_init(); G_FIRST_SAID_BY_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_SAID_BY_BOT(v: Py) { ensure_init(); G_FIRST_SAID_BY_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_THING() -> Py { ensure_init(); G_FIRST_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_THING(v: Py) { ensure_init(); G_FIRST_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_GREETING_WITH_AN_ADDRESS() -> Py { ensure_init(); G_GREETING_WITH_AN_ADDRESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREETING_WITH_AN_ADDRESS(v: Py) { ensure_init(); G_GREETING_WITH_AN_ADDRESS.with(|c| *c.borrow_mut() = v); }
pub fn g_GREETS_A_RETURN() -> Py { ensure_init(); G_GREETS_A_RETURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GREETS_A_RETURN(v: Py) { ensure_init(); G_GREETS_A_RETURN.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK() -> Py { ensure_init(); G_LOOK_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); G_LOOK_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_SAID_ABOUT_HEAD() -> Py { ensure_init(); G_MY_SAID_ABOUT_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_SAID_ABOUT_HEAD(v: Py) { ensure_init(); G_MY_SAID_ABOUT_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_SAID() -> Py { ensure_init(); G_NOTHING_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_SAID(v: Py) { ensure_init(); G_NOTHING_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_CONTENT_WORDS() -> Py { ensure_init(); G_NO_CONTENT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_CONTENT_WORDS(v: Py) { ensure_init(); G_NO_CONTENT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_ABOUT() -> Py { ensure_init(); G_RECALL_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_ABOUT(v: Py) { ensure_init(); G_RECALL_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL_SHAPE() -> Py { ensure_init(); G_RECALL_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL_SHAPE(v: Py) { ensure_init(); G_RECALL_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_REMIND_ME_RECAP() -> Py { ensure_init(); G_REMIND_ME_RECAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMIND_ME_RECAP(v: Py) { ensure_init(); G_REMIND_ME_RECAP.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_ABOUT_HEAD() -> Py { ensure_init(); G_SAID_ABOUT_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_ABOUT_HEAD(v: Py) { ensure_init(); G_SAID_ABOUT_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_STORE_SUMMARY() -> Py { ensure_init(); G_STORE_SUMMARY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STORE_SUMMARY(v: Py) { ensure_init(); G_STORE_SUMMARY.with(|c| *c.borrow_mut() = v); }
pub fn g_TALKED_ABOUT() -> Py { ensure_init(); G_TALKED_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TALKED_ABOUT(v: Py) { ensure_init(); G_TALKED_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_NEWS() -> Py { ensure_init(); G_THEIR_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_NEWS(v: Py) { ensure_init(); G_THEIR_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_WE_WERE() -> Py { ensure_init(); G_WHERE_WE_WERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_WE_WERE(v: Py) { ensure_init(); G_WHERE_WE_WERE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Questions about the conversation as a whole: what have we been on, and what came first.\n\nThe conversation lane owns this. Two things people ask about a chat they have been having have\nno other home on main:\n\n- what have we talked about (\"what have we covered?\", \"summarise our chat\")\n- what came first (\"what was the first thing I said?\", \"where did we start?\")\n\n\"What did I say about X?\" is read from the fact by the store's own recall (the `what did i say\nabout the boiler` path), so nothing here quotes a single line back.\n\nThe data is already there: memory[\"turns\"], memory[\"topics\"], written by\nbot/modules/memory_store.py. Nothing here replies; memory_store words the answer.\n");
    s_TALKED_ABOUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:else )?(?:have|did|were) (?:we|you and i|you and me) (?:been )?(?:talking|talked|talk|chatting|chatted|on|covering|covered|go over|through|about|discussed|discussing)\\b|\\bwhat (?:topics|things|stuff) (?:have|did) we\\b|\\bwhat did we (?:cover|discuss|mention|go over)(?:\\s+(?:so far|today|just now|again))?\\s*[?.!]*$|\\bremind me (?:what|of what) (?:we|you and i|you and me)(?:'ve|'d)? (?:have )?(?:been )?(?:talked|spoken|chatted|discussed|covered|been on|gone over|been over)\\b|\\bwhat did we go over(?:(?:\\s+so far|\\s+today|\\s+just now))?\\s*[?.!]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FIRST_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:was|were|is) (?:the|my) first (?:thing|question|topic|bit|line|message|words?)(?:\\s+(?:i|we)\\b|\\s*[?.!]*$)|\\bwhat did (?:i|we) (?:say|ask|mention|talk about) first\\b|\\b(?:where|how) did (?:we|i|you and i) (?:start|begin)\\s*[?.!]*$|\\b(?:where|how) did (?:this|our|the) (?:chat|conversation|talk) (?:start|begin|go)\\b|\\bhow did we (?:get|come) (?:on|to) (?:to )?(?:this|that|it)\\b|\\bwhat did (?:i|we) (?:open|start) with\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FIRST_SAID_BY_BOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:was|were|is) the first (?:thing|question|line|message|words?|answer|reply) (?:that )?you\\b|\\bwhat (?:was|were|is) your first (?:thing|question|line|message|words?|answer|reply)\\b|\\bwhat did you (?:say|ask) first\\b|\\bhow did you start\\b|\\bwhat did you (?:say|tell me|answer|ask)(?: \\w+){0,2} (?:at the (?:very )?(?:start|beginning)|in the beginning|early on)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RECALL_ABOUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:did|have|do) (?:i|we) (?:say|said|tell you|told you|mention|mentioned)(?: to you)? about\\b|\\b(?:remind me|tell me again) (?:what|of what) (?:i|we)(?:'ve)? (?:said|told you|mentioned|talked about|been on about)\\b[^?.!]*\\babout\\s+\\w")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAID_ABOUT_HEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and |so |ok |okay |also |but |oh )*(?:what did you (?:say|tell me|mention|suggest|recommend)|remind me what you (?:said|told me|mentioned)|what was (?:it|that) you (?:said|told me|mentioned)|what was your (?:answer|advice|reply|take)|what did you tell me)\\s*(?:about|on|regarding|for)\\s+(?P<thing>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHAT_QUESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|which|anything) (?:else )?(?:have|did|were|do) (?:we|i|you and i|you and me)\\b[^?.!]*\\b(?:talk|talked|talking|chat|chatted|chatting|discuss|discussed|discussing|cover|covered|covering|say|said|mention|mentioned|go over|been on|spoken)\\b|\\b(?:remind me|tell me again|recap)\\b[^?.!]*\\b(?:talked|talk|spoken|spoke|chatted|discussed|covered|said|been on|conversation|chat)\\b|\\bsummar(?:y|ise|ize)\\b.*\\b(?:chat|conversation|talk|said)\\b|\\bwhere (?:did|do) (?:we|i) (?:start|begin|get to|leave off|started)\\b|\\bwhat (?:was|is) (?:the )?(?:first|last) (?:thing|question|topic|bit|line|message|words?)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_REMIND_ME_RECAP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bremind me (?:what|of what) (?:we|you and i|you and me)(?:'ve|'d)? (?:have )?(?:been )?(?:covered|been on about|been on|discussed)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHERE_WE_WERE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:were|are|was) (?:we|i|you) (?:talking|chatting|on) about\\b|\\bwhere were we\\b|\\bwhat was i saying\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STORE_SUMMARY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bsummar(?:y|ise|ize)\\b|\\brecap\\b|\\bcatch me up\\b|\\bwhere (?:did|do) we (?:leave|left|get|got)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RECALL_SHAPE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:does|do|did|is|are|was|were) (?:my|his|her|our|their)\\b|\\b(?:what|where|who|when|how many) (?:do|did|am|is|are) (?:i|we)\\b|\\bam i\\b|\\bdo i (?:have|live|work|need|eat)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MY_SAID_ABOUT_HEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and |so |ok |okay |also |but |oh )*(?:what did (?:i|we) (?:say|tell you|mention)|(?:what|anything) have (?:i|we) (?:said|told you|mentioned)|remind me what (?:i|we) (?:said|told you|mentioned)|what was (?:it|that) (?:i|we) (?:said|told you|mentioned)|did (?:i|we) (?:say|mention))\\s*(?:anything\\s+)?(?:about|on|regarding|for)\\s+(?P<thing>.+)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOTHING_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:hi|hiya|hello|hey|yo|morning|evening|good (?:morning|afternoon|evening)|\\s*how (?:are you|r you|are u)(?:\\s+doing)?(?:\\s+today)?|how(?:'s| is|s) it going|how are things|what(?:'s| is|s) up|thanks?|thank you|cheers|ta|nice(?: one)?|great|lovely|ok(?:ay)?|cool|wow|tell me (?:a|another) (?:joke|story|fact)|tell me something|say something|sing me\\b[^?.!]*|give me a joke)\\s*[.!?]*\\s*")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GREETING_WITH_AN_ADDRESS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:hi|hiya|hello|hey|yo|morning|evening|good (?:morning|afternoon|evening))(?:\\s+(?:there|all|everyone|everybody|guys|folks|mate|team|both|you|y'all))+\\s*[.!?]*\\s*")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAIMS_A_PAST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:where\\s+we\\s+(?:were|left\\s+off|got\\s+to)|what\\s+were\\s+we\\s+(?:saying|on\\s+about|up\\s+to)|what\\s+were\\s+you\\s+saying|as\\s+(?:i|we)\\s+said|as\\s+before|same\\s+as\\s+before|you\\s+mentioned|earlier\\s+(?:you|we)|last\\s+time\\s+(?:we|you|i)\\b|what\\s+did\\s+i\\s+say|remind\\s+me\\s+(?:what|where|which|how)|carry\\s+on\\s+where|before\\s+we\\s+(?:broke\\s+off|stopped)|still\\s+here\\s*,?\\s+(?:if|so)|what\\s+(?:are|were)\\s+we\\s+(?:working\\s+on|on|talking\\s+about)|whatever\\s+you(?:'ve|\\s+have)\\s+been\\s+(?:doing|up\\s+to)|(?:a\\s+)?break\\s+does\\s+(?:a\\s+lot\\s+of\\s+|you\\s+)?good|keep\\s+it\\s+going\\s+(?:for\\s+)?another|since\\s+we\\s+last\\s+spoke|last\\s+we\\s+spoke|(?:we|i)\\s+(?:talked|spoke)\\s+about\\s+(?:it|that|this)|where\\s+(?:were\\s+we|did\\s+we\\s+get\\s+to|did\\s+we\\s+leave|have\\s+we\\s+got\\s+to))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GREETS_A_RETURN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:oh|ah|well|hello|hi|hey|right|ok|okay|and|so)\\s*[,!.]?\\s*)?(?:welcome\\s+back|you(?:'re|\\s+are)\\s+back|back\\s+again|(?:good|nice)\\s+to\\s+(?:see|have)\\s+you\\s+(?:again|back)|(?:hello|hi|hey)\\s+again)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANSWERS_AN_EARLIER_TURN(py_tuple(vec![py_str("good to hear"), py_str("glad to hear"), py_str("sounds alright"), py_str("sounds good"), py_str("nice one"), py_str("thanks for telling me"), py_str("thanks for that"), py_str("say more"), py_str("tell me more about that"), py_str("how did the rest of it go"), py_str("anything good in it"), py_str("same as before"), py_str("same answer"), py_str("same again"), py_str("nothing new to add"), py_str("that is still the thing to do"), py_str("as i said"), py_str("as you said"), py_str("like i said"), py_str("you said"), py_str("you told me"), py_str("you mentioned"), py_str("back to what we"), py_str("where we were"), py_str("where were we"), py_str("the same goes for"), py_str("how did that go"), py_str("what was the best")]));
    s_NO_CONTENT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\na an the as i we you he she it they me my our your us them and or but so if then than that this\nthese those is was were be been being am are do does did done have has had got get gets go goes\ngoing went gone on off up down out back again please sorry said say saying says tell told earlier\nbefore after now still just really very fine ok okay right well yes no yeah nope hi hey hello\nthanks thank nothing new same add to for of in at with about what where when who how why which\ncarry carrying never again anyway anyways so oh ah hmm um eh sort kind lot bit one cut\nmissed missed replies reply replied message messages quiet silence paused pause interrupted\ndistracted resume continue thread thought half unfinished midway apologise apologize apologies\ngone away mid\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THEY_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\?\\s*$|^(?:what|when|where|who|why|how|which|do|does|did|can|could|should|would|is|are|was|were|have|has|will|remind|tell me|any idea|any chance)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LOOK_BACK(Py::Int(8));
    s_THEIR_NEWS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we|my|our|me|mine|us)\\b|\\bive\\b|\\bim\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_QUESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:what|where|when|who|why|how|which|do|does|did|is|are|was|were|can|could|should|would|will|shall|have|has|any)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WANT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:want|wants|wanting|need|needs|needed|hoping|hope to|planning|plan to|going to|thinking of|thinking about)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BARE_YES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:yes|yeah|yep|no|nope|ok|okay|sure|thanks|thank you|ta|right|cool|nice|hmm+|ah+|oh+)\\W*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    let _ = py_str("\"What have we talked about?\" as plain words, punctuation stripped from the ends.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    Py::None
}
pub fn the_thing_named(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing a \"what did you say about X\" names, or \"\" when there is no such thing.");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _flat(args))(&__args) }, "strip", args))(&__args) };
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAID_ABOUT_HEAD(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("those"), py_str("these")]), &thing.clone())) } }) {
    return py_str("");
    }
    return thing.clone();
    Py::None
}
pub fn the_thing_they_named(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing a \"what did i say about X\" names, or \"\" when there is no such thing.\n\n    The mirror of `the_thing_named`, which reads the bot's side. Kept apart from it because the\n    two are answered from different turns: what they said about the tablets is their own line,\n    and what the bot said about the tablets is its answer, and reading one as the other handed\n    back the bot's advice where the person's line was asked for.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _flat(args))(&__args) }, "strip", args))(&__args) };
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MY_SAID_ABOUT_HEAD(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ?.!,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+")); __args.push(py_str(" ")); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("those"), py_str("these")]), &thing.clone())) } }) {
    return py_str("");
    }
    return thing.clone();
    Py::None
}
pub fn asked(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut low: Py = Py::None;
    let _ = py_str("Which conversation question this is: 'topics', 'first' or None.\n\n    `memory` is not needed for the answers here; it is passed so a caller can ask about the\n    line in the store's own kept state without a second signature.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _flat(args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHERE_WE_WERE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STORE_SUMMARY(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_REMIND_ME_RECAP(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECALL_ABOUT(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_thing_named(args))(&__args) }) {
    return py_str("said_about");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FIRST_SAID_BY_BOT(), "search", args))(&__args) }) {
    return py_str("first_bot");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FIRST_THING(), "search", args))(&__args) }) {
    return py_str("first");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TALKED_ABOUT(), "search", args))(&__args) }) {
    return py_str("topics");
    }
    return Py::None;
    Py::None
}
pub fn _nothing_said(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("A greeting, a thank you or a request to be entertained: not a subject.");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _flat(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOTHING_SAID(), "fullmatch", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GREETING_WITH_AN_ADDRESS(), "fullmatch", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn about_the_chat(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True for anything that asks about the conversation itself, answered or not.\n\n    A line like \"what else have we talked about\" that nothing answers is still a question about\n    the chat, so it must not come back later quoted as a subject we have been on.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _flat(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asked(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECALL_ABOUT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CHAT_QUESTION(), "search", args))(&__args) } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn not_a_subject(args: &[Py]) -> Py {
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
    let _ = py_str("Nothing we can say we have been on: a chat question, an ask about what they told you,\n    a greeting, or a joke request.");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| about_the_chat(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _nothing_said(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _flat(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RECALL_SHAPE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } };
    Py::None
}
pub fn brings_nothing_of_its_own(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line has no subject of its own, only a point back at the conversation.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NO_CONTENT_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    let mut memory: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let _ = py_str("Whether the conversation has a turn before this one.\n\n    The runtime appends a turn after the modules answer it, so an empty history is the opening\n    line. One reading of that, so every reply that leans on the chat asks the same question of the\n    same place. `text` is the line being answered: a turn carrying those words is this line and\n    not an earlier one, which matters to a reader that cannot rely on the store having recorded\n    the line yet.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _earlier_turns(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn a_past_that_is_not_there(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { Py::None };
    let mut frame: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when a reply leans on a conversation that has not happened yet.\n\n    One question, asked by every reply that speaks about the chat: is there an earlier turn? The\n    replies that say there is come in two kinds - the ones that point back at it (\"welcome back\",\n    \"where were we\", \"you mentioned\", \"as I said\") and the ones that answer it (\"good to hear\",\n    \"nice one, how did the rest of it go\", \"thanks for telling me\"), and the second kind is only\n    wrong when `said` - the person's line - brings nothing of its own. A caller that does not have\n    the line to hand (`said` left out) gets the pointing-back list only, which needs no gate.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| an_earlier_turn(args))(&__args) }) {
    return Py::Bool(false);
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAIMS_A_PAST(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GREETS_A_RETURN(), "match", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_is(&said.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| brings_nothing_of_its_own(args))(&__args) }))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ANSWERS_AN_EARLIER_TURN()) { let mut frame: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &frame.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn first_bot_line(args: &[Py]) -> Py {
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
    let _ = py_str("The first thing the bot said in this conversation, or ''.");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turn.clone(), &py_str("bot"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _earlier_turns(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut out: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("Every turn of theirs before this line, oldest first.\n\n    The store writes the line being answered into memory[\"turns\"] as it observes it, so the\n    newest turn is this line itself and is not an earlier one. Comparing the words rather than\n    dropping the last turn keeps this right for a reader called with a line the store has not\n    seen yet.\n    ");
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None }; if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    row = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(row.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&row.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }, &mine.clone())) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(row.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    return out.clone();
    Py::None
}
pub fn _a_line_they_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut row: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "row") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: row"))) };
    let _ = py_str("True when the turn is a question of theirs, not something they told the chat.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = row.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THEY_ASKED(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words and the verb forms of each - \"walking\" asks about a line that says \"walk\".\n\n    `topics.content` takes the ending off what it keeps, so the ask and the line can miss each\n    other (\"walking\" against \"walk\"), and the ask then fell back to the newest line and handed\n    back something said about another thing.\n    ");
    out = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    let __aug2 = op_bitor(&out.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::forms(args))(&__args) });
    out = __aug2;
    }
    return out.clone();
    Py::None
}
pub fn the_earlier_turn(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut about: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "about") { v.clone() }
    else { py_str("") };
    let mut _row: Py = Py::None;
    let mut best: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut row: Py = Py::None;
    let mut scored: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut told: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The turn of theirs this line is about, or None when the chat holds no earlier one.\n\n    The one check for \"is there an earlier turn?\", and for which one. An ask that names a thing\n    (\"what did i say about lisbon\") is about the turn that names it, not about the newest line:\n    a chat that says \"im saving 3000 for a trip to lisbon\" and then \"the flights are about 120\n    each\" answers the lisbon ask from the first line, where reading the newest line handed back\n    a figure given for something else. When the line names nothing, or names something no turn\n    has, the newest turn is the honest one.\n    ");
    earlier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _earlier_turns(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&earlier.clone()))) {
    return Py::None;
    }
    told = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&earlier.clone()) { let mut row: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(row.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_line_they_asked(args))(&__args) }))) {  __out.push(row.clone()); } } py_list(__out) })() };
    if py_truthy(&told.clone()) {
    earlier = told.clone();
    }
    thing = { let __b = about.clone(); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_thing_they_named(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_thing_named(args))(&__args) } } } } };
    wanted = if py_truthy(&thing.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::content(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_forms(args))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return geti(&earlier.clone(), &op_neg(&Py::Int(1)));
    }
    scored = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&earlier.clone()) { let mut row: Py = __item.clone(); __out.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&wanted.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&row.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::topics::content(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_forms(args))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, row.clone()])); } py_list(__out) })() };
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&scored.clone()) { let __unpk6966 = py_unpack(&__item.clone(), 2, None); let mut hit: Py = __unpk6966[0].clone(); let mut _row: Py = __unpk6966[1].clone(); __out.push(hit.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    return geti(&earlier.clone(), &op_neg(&Py::Int(1)));
    }
    return geti(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&scored.clone()) { let __unpk2454 = py_unpack(&__item.clone(), 2, None); let mut hit: Py = __unpk2454[0].clone(); let mut row: Py = __unpk2454[1].clone(); if py_truthy(&Py::Bool(py_eq(&hit.clone(), &best.clone()))) {  __out.push(row.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(1)));
    Py::None
}
pub fn first_line(args: &[Py]) -> Py {
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
    let _ = py_str("The first thing the user said in this conversation, or ''.");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turn.clone(), &py_str("user"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn topic_lines(args: &[Py]) -> Py {
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
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Int(3) };
    let mut last: Py = Py::None;
    let mut out: Py = Py::None;
    let mut record: Py = Py::None;
    let mut records: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The subjects the conversation has been on, oldest first, as the user put them.");
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_list(vec![])]), 2, None);
    seen = __unpk1[0].clone();
    out = __unpk1[1].clone();
    records = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topics")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    let __iter2 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(records.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { records.clone() } else { py_list(vec![]) });
    for __item in __iter2 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(record.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&record.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&seen.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| not_a_subject(args))(&__args) } } } } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    last = py_str("");
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { turns.clone() } else { py_list(vec![]) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    turn = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }, &py_str("memory_store")))) {
    continue;
    }
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turn.clone(), &py_str("user"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    break;
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&seen.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "lower", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| not_a_subject(args))(&__args) })) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    return slice_or_raise(&out.clone(), &op_neg(&limit.clone()), &Py::None, &Py::None);
    Py::None
}
pub fn turns_before(args: &[Py]) -> Py {
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
    let _ = py_str("Every turn the chat has had before this one, oldest first. Never raises.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_list(vec![]);
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_list(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn the_turn_before(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The newest turn before this one, as a dict, or None when the chat has none.\n\n    `an_earlier_turn` above is the yes-or-no that most readers want; this is the same question\n    answered with the turn itself, for a reader that wants to read it.\n    ");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| turns_before(args))(&__args) };
    return if py_truthy(&turns.clone()) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { Py::None };
    Py::None
}
pub fn nothing_said_before(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this line is the first thing they have said.\n\n    Read from `turns_before` rather than from `an_earlier_turn`: a `turns` list of things that are\n    not turns holds nothing a reader can lean on, so it is as good as no earlier turn at all.\n    ");
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| turns_before(args))(&__args) }));
    Py::None
}
pub fn the_lines_before(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { g_LOOK_BACK() };
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What they said in the last `look_back` turns, oldest first, empties dropped.");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| turns_before(args))(&__args) }, &op_neg(&look_back.clone()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); } py_list(__out) })() };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push(line.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn the_line_before(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { g_LOOK_BACK() };
    let mut lines: Py = Py::None;
    let _ = py_str("What they said in the newest earlier turn, or ''.");
    lines = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(look_back.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines_before(args))(&__args) };
    return if py_truthy(&lines.clone()) { geti(&lines.clone(), &op_neg(&Py::Int(1))) } else { py_str("") };
    Py::None
}
pub fn is_their_news(args: &[Py]) -> Py {
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
    else { Py::Int(3) };
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether a line is news of theirs, rather than a question, a want or a bare \"yes\".\n\n    A reader that asks back a detail the person left out needs a line that could have left one\n    out: \"i got the job\" has, \"how much\" has not, and \"i want to go to rome\" has a price tag\n    rather than a detail.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &words.clone())) {
    return Py::Bool(false);
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "endswith", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_QUESTION(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BARE_YES(), "match", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WANT(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_NEWS(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_news_turn_before(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::Int(3) };
    let mut line: Py = Py::None;
    let _ = py_str("What they said in the newest earlier turn that was their news, or ''.\n\n    \"is it worth getting a flu jab\" / \"how much does it cost\" has no news in it to leave a detail\n    out of, and neither has the bare \"yes\" after a plan, so both answer '' and the reader that\n    asked declines instead of asking the person their own question back.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines_before(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| is_their_news(args))(&__args) }) {
    return line.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn the_earlier_line(args: &[Py]) -> Py {
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
    let mut pattern: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let mut look_back: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { g_LOOK_BACK() };
    let mut line: Py = Py::None;
    let _ = py_str("The newest earlier line of theirs that matches `pattern`, or ''.\n\n    The pattern may be compiled or a string. This is how a follow-up finds its antecedent: \"what\n    are their names\" looks back for the line that named more than one person, and finds it however\n    the person happened to word it.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    pattern = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(look_back.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines_before(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    return line.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn the_earlier_lines_that(args: &[Py]) -> Py {
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
    let mut pattern: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let mut look_back: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { g_LOOK_BACK() };
    let mut line: Py = Py::None;
    let _ = py_str("Every earlier line of theirs that matches, newest last.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    pattern = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(look_back.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_lines_before(args))(&__args) }) { let mut line: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {  __out.push(line.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn only_with_an_earlier_turn(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reader: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reader") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reader"))) };
    let mut guarded: Py = Py::None;
    let _ = py_str("Wrap a store reader so it answers only when the chat has a turn before this one.\n\n    A reader that hands back what was said cannot be right on the first line of a chat, whatever\n    its patterns say; used as a decorator it saves each one from its own copy of the check.\n    ");
pub fn __nested_guarded_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut guarded: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reader: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut args: Py = py_list(__pos.get(3..).unwrap_or(&[]).to_vec());
    let mut kwargs: Py = py_dict(__kw.iter().filter(|(n, _)| !["memory"].contains(&n.as_str())).map(|(n, v)| (py_str(n), v.clone())).collect());
    let mut args: Py = Py::None;
    let mut kwargs: Py = Py::None;
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| nothing_said_before(args))(&__args) }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.extend(py_iter_or_raise(&args.clone())); for (k, v) in kw_pairs(&kwargs.clone()) { __kws.push((k, v)); } if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&reader, args))(&__args) };
    Py::None
}
    guarded = {{ let guarded = guarded.clone(); let reader = reader.clone(); py_func("guarded", move |args: &[Py]| { let mut full = vec![guarded.clone(), reader.clone()]; full.extend_from_slice(args); __nested_guarded_0(&full) }) }};
    let _ = py_setattr(&guarded.clone(), "__name__", { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reader.clone()); __args.push(py_str("__name__")); __args.push(py_str("guarded")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("getattr", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    return guarded.clone();
    Py::None
}
pub fn the_last_answer(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The bot's last line as it was written, or \"\" when there is no earlier turn.\n\n    This is the raw line, notes and hedges included: what the person can see. A caller that wants\n    the answer rather than the last line asks `the_answer_on_screen`, which steps back over the\n    turns that are not answers.\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| turns_before(args))(&__args) }) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "get", args))(&__args) }); } } py_list(__out) })() };
    return if py_truthy(&said.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&said.clone(), &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) } } else { py_str("") };
    Py::None
}
pub fn _a_note_not_an_answer(args: &[Py]) -> Py {
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
    let mut fallback: Py = pyrt::xrefs::xmodule("compose.fallback");
    let mut verdict: Py = pyrt::xrefs::xmodule("compose.verdict");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&verdict.clone(), "takes_a_fact_back", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "is_said_it_already", args))(&__args) } } };
    Py::None
}
pub fn _a_hedge(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut punt: Py = Py::None;
    let mut fallback: Py = pyrt::xrefs::xmodule("compose.fallback");
    flat = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "_is_a_hedge", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&fallback.clone(), "PUNT")) { let mut punt: Py = __item.clone(); __out.push(Py::Bool(py_contains(&flat.clone(), &punt.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn the_answer_on_screen(args: &[Py]) -> Py {
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
    let mut within: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "within") { v.clone() }
    else { Py::Int(3) };
    let mut modules: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "modules") { v.clone() }
    else { py_tuple(vec![py_str("answers"), py_str("safety")]) };
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The knowledge answer the person is looking at, or \"\" when the last turns hold none.\n\n    This is the answer a follow-up is read from, so it steps back past the composer's notes and\n    past a hedge - neither is an answer - and it stops at the first real answer only when that\n    answer came from the knowledge base or a safety card. A turn of the store's own, or of small\n    talk, is not an answer to come back with, and \"\" is what the caller needs to pass.\n    ");
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| turns_before(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_gt(&seen.clone(), &within.clone())) {
    break;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_note_not_an_answer(args))(&__args) }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&modules.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_hedge(args))(&__args) }) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    return py_str("");
    Py::None
}
pub fn the_turn_the_answer_came_from(args: &[Py]) -> Py {
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
    let mut within: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "within") { v.clone() }
    else { Py::Int(3) };
    let mut answer: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The turn dict `the_answer_on_screen` read, or None - for callers that need the module.");
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(within.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_answer_on_screen(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| turns_before(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &answer.clone()))) {
    return turn.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn the_answer_is_a_card(args: &[Py]) -> Py {
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
    let mut within: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "within") { v.clone() }
    else { Py::Int(3) };
    let mut turn: Py = Py::None;
    let _ = py_str("Whether the answer on the screen is a safety card rather than a knowledge entry.");
    turn = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(within.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_turn_the_answer_came_from(args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_str("safety"))) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.history".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS_AN_EARLIER_TURN" => Some(g_ANSWERS_AN_EARLIER_TURN()),
            "A_BARE_YES" => Some(g_A_BARE_YES()),
            "A_QUESTION" => Some(g_A_QUESTION()),
            "A_THEY_ASKED" => Some(g_A_THEY_ASKED()),
            "A_WANT" => Some(g_A_WANT()),
            "CHAT_QUESTION" => Some(g_CHAT_QUESTION()),
            "CLAIMS_A_PAST" => Some(g_CLAIMS_A_PAST()),
            "FIRST_SAID_BY_BOT" => Some(g_FIRST_SAID_BY_BOT()),
            "FIRST_THING" => Some(g_FIRST_THING()),
            "GREETING_WITH_AN_ADDRESS" => Some(g_GREETING_WITH_AN_ADDRESS()),
            "GREETS_A_RETURN" => Some(g_GREETS_A_RETURN()),
            "LOOK_BACK" => Some(g_LOOK_BACK()),
            "MY_SAID_ABOUT_HEAD" => Some(g_MY_SAID_ABOUT_HEAD()),
            "NOTHING_SAID" => Some(g_NOTHING_SAID()),
            "NO_CONTENT_WORDS" => Some(g_NO_CONTENT_WORDS()),
            "RECALL_ABOUT" => Some(g_RECALL_ABOUT()),
            "RECALL_SHAPE" => Some(g_RECALL_SHAPE()),
            "REMIND_ME_RECAP" => Some(g_REMIND_ME_RECAP()),
            "SAID_ABOUT_HEAD" => Some(g_SAID_ABOUT_HEAD()),
            "STORE_SUMMARY" => Some(g_STORE_SUMMARY()),
            "TALKED_ABOUT" => Some(g_TALKED_ABOUT()),
            "THEIR_NEWS" => Some(g_THEIR_NEWS()),
            "WHERE_WE_WERE" => Some(g_WHERE_WE_WERE()),
            "_a_hedge" => Some(py_func("_a_hedge", |args: &[Py]| _a_hedge(args))),
            "_a_line_they_asked" => Some(py_func("_a_line_they_asked", |args: &[Py]| _a_line_they_asked(args))),
            "_a_note_not_an_answer" => Some(py_func("_a_note_not_an_answer", |args: &[Py]| _a_note_not_an_answer(args))),
            "_earlier_turns" => Some(py_func("_earlier_turns", |args: &[Py]| _earlier_turns(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_nothing_said" => Some(py_func("_nothing_said", |args: &[Py]| _nothing_said(args))),
            "_the_forms" => Some(py_func("_the_forms", |args: &[Py]| _the_forms(args))),
            "a_past_that_is_not_there" => Some(py_func("a_past_that_is_not_there", |args: &[Py]| a_past_that_is_not_there(args))),
            "about_the_chat" => Some(py_func("about_the_chat", |args: &[Py]| about_the_chat(args))),
            "an_earlier_turn" => Some(py_func("an_earlier_turn", |args: &[Py]| an_earlier_turn(args))),
            "asked" => Some(py_func("asked", |args: &[Py]| asked(args))),
            "brings_nothing_of_its_own" => Some(py_func("brings_nothing_of_its_own", |args: &[Py]| brings_nothing_of_its_own(args))),
            "first_bot_line" => Some(py_func("first_bot_line", |args: &[Py]| first_bot_line(args))),
            "first_line" => Some(py_func("first_line", |args: &[Py]| first_line(args))),
            "is_their_news" => Some(py_func("is_their_news", |args: &[Py]| is_their_news(args))),
            "not_a_subject" => Some(py_func("not_a_subject", |args: &[Py]| not_a_subject(args))),
            "nothing_said_before" => Some(py_func("nothing_said_before", |args: &[Py]| nothing_said_before(args))),
            "only_with_an_earlier_turn" => Some(py_func("only_with_an_earlier_turn", |args: &[Py]| only_with_an_earlier_turn(args))),
            "the_answer_is_a_card" => Some(py_func("the_answer_is_a_card", |args: &[Py]| the_answer_is_a_card(args))),
            "the_answer_on_screen" => Some(py_func("the_answer_on_screen", |args: &[Py]| the_answer_on_screen(args))),
            "the_earlier_line" => Some(py_func("the_earlier_line", |args: &[Py]| the_earlier_line(args))),
            "the_earlier_lines_that" => Some(py_func("the_earlier_lines_that", |args: &[Py]| the_earlier_lines_that(args))),
            "the_earlier_turn" => Some(py_func("the_earlier_turn", |args: &[Py]| the_earlier_turn(args))),
            "the_last_answer" => Some(py_func("the_last_answer", |args: &[Py]| the_last_answer(args))),
            "the_line_before" => Some(py_func("the_line_before", |args: &[Py]| the_line_before(args))),
            "the_lines_before" => Some(py_func("the_lines_before", |args: &[Py]| the_lines_before(args))),
            "the_news_turn_before" => Some(py_func("the_news_turn_before", |args: &[Py]| the_news_turn_before(args))),
            "the_thing_named" => Some(py_func("the_thing_named", |args: &[Py]| the_thing_named(args))),
            "the_thing_they_named" => Some(py_func("the_thing_they_named", |args: &[Py]| the_thing_they_named(args))),
            "the_turn_before" => Some(py_func("the_turn_before", |args: &[Py]| the_turn_before(args))),
            "the_turn_the_answer_came_from" => Some(py_func("the_turn_the_answer_came_from", |args: &[Py]| the_turn_the_answer_came_from(args))),
            "topic_lines" => Some(py_func("topic_lines", |args: &[Py]| topic_lines(args))),
            "turns_before" => Some(py_func("turns_before", |args: &[Py]| turns_before(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS_AN_EARLIER_TURN" => s_ANSWERS_AN_EARLIER_TURN(v),
            "A_BARE_YES" => s_A_BARE_YES(v),
            "A_QUESTION" => s_A_QUESTION(v),
            "A_THEY_ASKED" => s_A_THEY_ASKED(v),
            "A_WANT" => s_A_WANT(v),
            "CHAT_QUESTION" => s_CHAT_QUESTION(v),
            "CLAIMS_A_PAST" => s_CLAIMS_A_PAST(v),
            "FIRST_SAID_BY_BOT" => s_FIRST_SAID_BY_BOT(v),
            "FIRST_THING" => s_FIRST_THING(v),
            "GREETING_WITH_AN_ADDRESS" => s_GREETING_WITH_AN_ADDRESS(v),
            "GREETS_A_RETURN" => s_GREETS_A_RETURN(v),
            "LOOK_BACK" => s_LOOK_BACK(v),
            "MY_SAID_ABOUT_HEAD" => s_MY_SAID_ABOUT_HEAD(v),
            "NOTHING_SAID" => s_NOTHING_SAID(v),
            "NO_CONTENT_WORDS" => s_NO_CONTENT_WORDS(v),
            "RECALL_ABOUT" => s_RECALL_ABOUT(v),
            "RECALL_SHAPE" => s_RECALL_SHAPE(v),
            "REMIND_ME_RECAP" => s_REMIND_ME_RECAP(v),
            "SAID_ABOUT_HEAD" => s_SAID_ABOUT_HEAD(v),
            "STORE_SUMMARY" => s_STORE_SUMMARY(v),
            "TALKED_ABOUT" => s_TALKED_ABOUT(v),
            "THEIR_NEWS" => s_THEIR_NEWS(v),
            "WHERE_WE_WERE" => s_WHERE_WE_WERE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.history", module_obj);
    pyrt::xrefs::reg_fn("memory.history", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_thing_named", py_func("the_thing_named", |args: &[Py]| the_thing_named(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_thing_they_named", py_func("the_thing_they_named", |args: &[Py]| the_thing_they_named(args)));
    pyrt::xrefs::reg_fn("memory.history", "asked", py_func("asked", |args: &[Py]| asked(args)));
    pyrt::xrefs::reg_fn("memory.history", "_nothing_said", py_func("_nothing_said", |args: &[Py]| _nothing_said(args)));
    pyrt::xrefs::reg_fn("memory.history", "about_the_chat", py_func("about_the_chat", |args: &[Py]| about_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.history", "not_a_subject", py_func("not_a_subject", |args: &[Py]| not_a_subject(args)));
    pyrt::xrefs::reg_fn("memory.history", "brings_nothing_of_its_own", py_func("brings_nothing_of_its_own", |args: &[Py]| brings_nothing_of_its_own(args)));
    pyrt::xrefs::reg_fn("memory.history", "an_earlier_turn", py_func("an_earlier_turn", |args: &[Py]| an_earlier_turn(args)));
    pyrt::xrefs::reg_fn("memory.history", "a_past_that_is_not_there", py_func("a_past_that_is_not_there", |args: &[Py]| a_past_that_is_not_there(args)));
    pyrt::xrefs::reg_fn("memory.history", "first_bot_line", py_func("first_bot_line", |args: &[Py]| first_bot_line(args)));
    pyrt::xrefs::reg_fn("memory.history", "_earlier_turns", py_func("_earlier_turns", |args: &[Py]| _earlier_turns(args)));
    pyrt::xrefs::reg_fn("memory.history", "_a_line_they_asked", py_func("_a_line_they_asked", |args: &[Py]| _a_line_they_asked(args)));
    pyrt::xrefs::reg_fn("memory.history", "_the_forms", py_func("_the_forms", |args: &[Py]| _the_forms(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_earlier_turn", py_func("the_earlier_turn", |args: &[Py]| the_earlier_turn(args)));
    pyrt::xrefs::reg_fn("memory.history", "first_line", py_func("first_line", |args: &[Py]| first_line(args)));
    pyrt::xrefs::reg_fn("memory.history", "topic_lines", py_func("topic_lines", |args: &[Py]| topic_lines(args)));
    pyrt::xrefs::reg_fn("memory.history", "turns_before", py_func("turns_before", |args: &[Py]| turns_before(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_turn_before", py_func("the_turn_before", |args: &[Py]| the_turn_before(args)));
    pyrt::xrefs::reg_fn("memory.history", "nothing_said_before", py_func("nothing_said_before", |args: &[Py]| nothing_said_before(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_lines_before", py_func("the_lines_before", |args: &[Py]| the_lines_before(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_line_before", py_func("the_line_before", |args: &[Py]| the_line_before(args)));
    pyrt::xrefs::reg_fn("memory.history", "is_their_news", py_func("is_their_news", |args: &[Py]| is_their_news(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_news_turn_before", py_func("the_news_turn_before", |args: &[Py]| the_news_turn_before(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_earlier_line", py_func("the_earlier_line", |args: &[Py]| the_earlier_line(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_earlier_lines_that", py_func("the_earlier_lines_that", |args: &[Py]| the_earlier_lines_that(args)));
    pyrt::xrefs::reg_fn("memory.history", "only_with_an_earlier_turn", py_func("only_with_an_earlier_turn", |args: &[Py]| only_with_an_earlier_turn(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_last_answer", py_func("the_last_answer", |args: &[Py]| the_last_answer(args)));
    pyrt::xrefs::reg_fn("memory.history", "_a_note_not_an_answer", py_func("_a_note_not_an_answer", |args: &[Py]| _a_note_not_an_answer(args)));
    pyrt::xrefs::reg_fn("memory.history", "_a_hedge", py_func("_a_hedge", |args: &[Py]| _a_hedge(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_answer_on_screen", py_func("the_answer_on_screen", |args: &[Py]| the_answer_on_screen(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_turn_the_answer_came_from", py_func("the_turn_the_answer_came_from", |args: &[Py]| the_turn_the_answer_came_from(args)));
    pyrt::xrefs::reg_fn("memory.history", "the_answer_is_a_card", py_func("the_answer_is_a_card", |args: &[Py]| the_answer_is_a_card(args)));
    pyrt::xrefs::reg_access("memory.history", "ANSWERS_AN_EARLIER_TURN", g_ANSWERS_AN_EARLIER_TURN, s_ANSWERS_AN_EARLIER_TURN);
    pyrt::xrefs::reg_access("memory.history", "A_BARE_YES", g_A_BARE_YES, s_A_BARE_YES);
    pyrt::xrefs::reg_access("memory.history", "A_QUESTION", g_A_QUESTION, s_A_QUESTION);
    pyrt::xrefs::reg_access("memory.history", "A_THEY_ASKED", g_A_THEY_ASKED, s_A_THEY_ASKED);
    pyrt::xrefs::reg_access("memory.history", "A_WANT", g_A_WANT, s_A_WANT);
    pyrt::xrefs::reg_access("memory.history", "CHAT_QUESTION", g_CHAT_QUESTION, s_CHAT_QUESTION);
    pyrt::xrefs::reg_access("memory.history", "CLAIMS_A_PAST", g_CLAIMS_A_PAST, s_CLAIMS_A_PAST);
    pyrt::xrefs::reg_access("memory.history", "FIRST_SAID_BY_BOT", g_FIRST_SAID_BY_BOT, s_FIRST_SAID_BY_BOT);
    pyrt::xrefs::reg_access("memory.history", "FIRST_THING", g_FIRST_THING, s_FIRST_THING);
    pyrt::xrefs::reg_access("memory.history", "GREETING_WITH_AN_ADDRESS", g_GREETING_WITH_AN_ADDRESS, s_GREETING_WITH_AN_ADDRESS);
    pyrt::xrefs::reg_access("memory.history", "GREETS_A_RETURN", g_GREETS_A_RETURN, s_GREETS_A_RETURN);
    pyrt::xrefs::reg_access("memory.history", "LOOK_BACK", g_LOOK_BACK, s_LOOK_BACK);
    pyrt::xrefs::reg_access("memory.history", "MY_SAID_ABOUT_HEAD", g_MY_SAID_ABOUT_HEAD, s_MY_SAID_ABOUT_HEAD);
    pyrt::xrefs::reg_access("memory.history", "NOTHING_SAID", g_NOTHING_SAID, s_NOTHING_SAID);
    pyrt::xrefs::reg_access("memory.history", "NO_CONTENT_WORDS", g_NO_CONTENT_WORDS, s_NO_CONTENT_WORDS);
    pyrt::xrefs::reg_access("memory.history", "RECALL_ABOUT", g_RECALL_ABOUT, s_RECALL_ABOUT);
    pyrt::xrefs::reg_access("memory.history", "RECALL_SHAPE", g_RECALL_SHAPE, s_RECALL_SHAPE);
    pyrt::xrefs::reg_access("memory.history", "REMIND_ME_RECAP", g_REMIND_ME_RECAP, s_REMIND_ME_RECAP);
    pyrt::xrefs::reg_access("memory.history", "SAID_ABOUT_HEAD", g_SAID_ABOUT_HEAD, s_SAID_ABOUT_HEAD);
    pyrt::xrefs::reg_access("memory.history", "STORE_SUMMARY", g_STORE_SUMMARY, s_STORE_SUMMARY);
    pyrt::xrefs::reg_access("memory.history", "TALKED_ABOUT", g_TALKED_ABOUT, s_TALKED_ABOUT);
    pyrt::xrefs::reg_access("memory.history", "THEIR_NEWS", g_THEIR_NEWS, s_THEIR_NEWS);
    pyrt::xrefs::reg_access("memory.history", "WHERE_WE_WERE", g_WHERE_WE_WERE, s_WHERE_WE_WERE);
}
