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


thread_local! { static G_A_NO_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NO_SEARCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NO_SEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NO_SPEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKING_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LEADING_PREPOSITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MERE_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MESSAGE_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SEARCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SPEND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FLAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEP_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THEIR_ERRAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_ADDRESSEE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_TAIL_OF_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_VERB_AND_THEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.onmybehalf: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_NO_LIST() -> Py { ensure_init(); G_A_NO_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NO_LIST(v: Py) { ensure_init(); G_A_NO_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NO_SEARCH() -> Py { ensure_init(); G_A_NO_SEARCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NO_SEARCH(v: Py) { ensure_init(); G_A_NO_SEARCH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NO_SEND() -> Py { ensure_init(); G_A_NO_SEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NO_SEND(v: Py) { ensure_init(); G_A_NO_SEND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NO_SPEND() -> Py { ensure_init(); G_A_NO_SPEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NO_SPEND(v: Py) { ensure_init(); G_A_NO_SPEND.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKING_BACK() -> Py { ensure_init(); G__ASKING_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING_BACK(v: Py) { ensure_init(); G__ASKING_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LEADING_PREPOSITION() -> Py { ensure_init(); G__A_LEADING_PREPOSITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LEADING_PREPOSITION(v: Py) { ensure_init(); G__A_LEADING_PREPOSITION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LIST() -> Py { ensure_init(); G__A_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LIST(v: Py) { ensure_init(); G__A_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MERE_PRONOUN() -> Py { ensure_init(); G__A_MERE_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MERE_PRONOUN(v: Py) { ensure_init(); G__A_MERE_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MESSAGE_OUT() -> Py { ensure_init(); G__A_MESSAGE_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MESSAGE_OUT(v: Py) { ensure_init(); G__A_MESSAGE_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SEARCH() -> Py { ensure_init(); G__A_SEARCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SEARCH(v: Py) { ensure_init(); G__A_SEARCH.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SPEND() -> Py { ensure_init(); G__A_SPEND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SPEND(v: Py) { ensure_init(); G__A_SPEND.with(|c| *c.borrow_mut() = v); }
pub fn g__FLAT() -> Py { ensure_init(); G__FLAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FLAT(v: Py) { ensure_init(); G__FLAT.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEP_OUT() -> Py { ensure_init(); G__KEEP_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEP_OUT(v: Py) { ensure_init(); G__KEEP_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__MINE() -> Py { ensure_init(); G__MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MINE(v: Py) { ensure_init(); G__MINE.with(|c| *c.borrow_mut() = v); }
pub fn g__THEIR_ERRAND() -> Py { ensure_init(); G__THEIR_ERRAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THEIR_ERRAND(v: Py) { ensure_init(); G__THEIR_ERRAND.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_ADDRESSEE() -> Py { ensure_init(); G__THE_ADDRESSEE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_ADDRESSEE(v: Py) { ensure_init(); G__THE_ADDRESSEE.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_TAIL_OF_THE_ASK() -> Py { ensure_init(); G__THE_TAIL_OF_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_TAIL_OF_THE_ASK(v: Py) { ensure_init(); G__THE_TAIL_OF_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_VERB_AND_THEN() -> Py { ensure_init(); G__THE_VERB_AND_THEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_VERB_AND_THEN(v: Py) { ensure_init(); G__THE_VERB_AND_THEN.with(|c| *c.borrow_mut() = v); }
pub fn g__YOURS() -> Py { ensure_init(); G__YOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YOURS(v: Py) { ensure_init(); G__YOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that hands the job to the bot, when the bot has no hands, no browser and no account.\n\n\"search for flights for me\", \"book the tickets\", \"send me the recipe\", \"save this for later\",\n\"add milk to my shopping list\", \"order more coffee on the app\". Each of those was answered with\none of the composer's flat readings - \"Got it. Is there anything you'd like a hand with?\", \"I'm\nlistening - say more if it helps\", \"Thanks for telling me - I'm here if you want to dig into any\nof that\" - and none of them is true, because nothing was done and nothing was even understood.\nThe person is not making small talk. They have handed over an errand.\n\nThe honest answer has three parts, in this order, because a refusal on its own leaves them where\nthey started (STYLE.md, \"Not knowing, honestly\"):\n\n1. what the bot cannot do, in a few words - no hands, no browser, no account, no card, and no\n   list that outlives the chat;\n2. what it can do instead, which is always something: get the words right, weigh the price, sort\n   the items, work out what to compare;\n3. the one place the rest happens - their app, their site, their notes, the send button.\n\nFour pools, one per kind of errand, because the way out differs: a search needs the details\nbefore the site can be opened, a booking needs the small print read before the money goes, a\nmessage needs the words, and a list needs to live somewhere that survives.\n\nSome of these errands have since been given their own answer elsewhere - a calendar line for \"save\nthis for later\", a booking line for \"book the tickets\", a printer line for \"print this out\". Those\nare better than a general pool, and the queue in `greeting.py` reaches them first; this file only\never replaces one of the composer's flat readings, so it stands down on every line that already\nhas an answer. What is left for it is the errand nobody has covered: the live look-up (\"search\nfor flights for me\", \"check the price for me online\") and the thing meant to leave here (\"send me\nthe recipe\"), both of which still come back with \"I'm listening - say more if it helps\" on main.\n\nNarrow on purpose. The ask must be an order at the front of the line (\"book the tickets\"), or the\nsame order with the asking put back in (\"can you book the tickets for me\"). A question about how\nto do the thing (\"how do i book a flight\"), a weigh-up (\"should i book now\"), a line about\nsomething already done (\"i booked the tickets\"), anything with a body, a symptom or a doctor in\nit, and the emergency verbs (call, ring, dial, 999) all keep their own answer and never reach\nthis file. Only four of the composer's flat readings are replaced.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._FLAT"); }
    s__FLAT(py_tuple(vec![pyrt::xrefs::xget("compose.fallback_data", "NEUTRAL"), pyrt::xrefs::xget("compose.fallback_data", "A_THING_SEEN_TO"), pyrt::xrefs::xget("compose.fallback_data", "A_THING_DONE"), pyrt::xrefs::xget("compose.fallback_data", "A_SMALL_DAY")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._ASKING_BACK"); }
    s__ASKING_BACK(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:please|pls|plz|just|ok|okay|now|and|also|so|then|hi|hey)\\s+)*(?:(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+|just\\s+|kindly\\s+|also\\s+)*|(?:i\\s+(?:want|need|would\\s+like)\\s+you\\s+to\\s+))?")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._A_SEARCH"); }
    s__A_SEARCH(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:please|pls|plz|just|ok|okay|now|and|also|so|then)\\s+)*(?:(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+|just\\s+|kindly\\s+|also\\s+)*)?(?:search|look\\s+(?:it\\s+)?up|look\\s+for|google|browse|compare|check\\s+(?:the\\s+)?(?:price|prices|times|flights|trains|availability))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._A_SPEND"); }
    s__A_SPEND(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:please|pls|plz|just|ok|okay|now|and|also|so|then)\\s+)*(?:(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+|just\\s+|kindly\\s+|also\\s+)*)?(?:book|reserve|order|buy|purchase|pay|top\\s+up|renew|cancel|check\\s+(?:me\\s+)?in|get\\s+me\\s+(?:a\\s+)?(?:ticket|table|seat|room))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._A_MESSAGE_OUT"); }
    s__A_MESSAGE_OUT(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:please|pls|plz|just|ok|okay|now|and|also|so|then)\\s+)*(?:(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+|just\\s+|kindly\\s+|also\\s+)*)?(?:send|email|text|forward|share|print|post|mail|message)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._A_LIST"); }
    s__A_LIST(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:please|pls|plz|just|ok|okay|now|and|also|so|then)\\s+)*(?:(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+|just\\s+|kindly\\s+|also\\s+)*)?(?:save|keep|store|note|jot|add|put|write\\s+(?:it|this|that)\\s+down|make\\s+a\\s+note|set\\s+(?:a|an)\\s+(?:reminder|alarm)|remind\\s+(?:me|us)\\s+to|remember\\s+(?:that|to))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._THEIR_ERRAND"); }
    s__THEIR_ERRAND(stdlib_call("re", "compile", &[py_str("\\b(?:me|my|mine|our|ours|us|for\\s+me|for\\s+us|this|that|these|those|it|them|(?:the|a|an)\\s+\\w+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._KEEP_OUT"); }
    s__KEEP_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+(?:do|can|should|would|to)\\b|what\\s+should|should\\s+i|shall\\s+i|is\\s+it\\s+worth|worth\\s+it|do\\s+you\\s+think|i\\s+(?:have\\s+|already\\s+)?(?:booked|ordered|bought|paid|sent|saved|added)|i\\s+(?:did|tried)|my\\s+(?:mum|dad|son|daughter|friend|husband|wife|partner|boss|carer|neighbour|brother|sister|nan|gran|grandad|child)\\s+(?:has|have|wants|asked|said)|ambulance|999|911|112|A&E|a\\s*&\\s*e|doctor|gp|surgery|nurse|pharmacist|chemist|hospital|pain|rash|lump|bleeding|blood|breath|chest|dose|medicine|tablet|an\\s+email\\s+(?:from|about)|a\\s+(?:text|letter|email)\\s+(?:from|about)|send\\s+my\\s+(?:love|regards|wishes|thanks|best)|love\\s+to\\b|(?:give|send)\\s+(?:them|him|her)\\s+my\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf.A_NO_SEARCH"); }
    s_A_NO_SEARCH(py_tuple(vec![py_str("Nothing here can search the web - no browser on this side. Say what matters most about {what} and I'll help you work out what to compare; the site or app itself has the rest."), py_str("I can't look anything up live, sorry. Tell me what you need to know about {what} and I'll say what to look for and what to ignore; whoever sells it has today's version."), py_str("No search from here - I only know what is in my own head, and none of it changes today. Name the details that matter about {what} and I'll narrow it down before you open the site.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf.A_NO_SPEND"); }
    s_A_NO_SPEND(py_tuple(vec![py_str("I can't book, order or pay for anything - no account, no card, no hands. Do it in their app or on their site, and tell me the price first: I'll say whether it looks fair and what to check before you pay."), py_str("Nothing on my side can hold a booking or take money. Read their cancellation line before you click buy - it is the one part of this that costs you later - and say what you are buying and I'll say what to watch."), py_str("That one needs your account, not me: I can't reach a basket or a booking form. Tell me what it costs and what it covers, and I'll help you work out whether the dearer one earns its money.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf.A_NO_SEND"); }
    s_A_NO_SEND(py_tuple(vec![py_str("I can't send or print anything - nothing leaves this chat. I'll get {what} right here instead, in full; copying it across is the one press on your side."), py_str("No email, no text, no printer on my side, sorry. What I can do is get {what} exactly right here; pasting it over is the last step, and it is yours."), py_str("Nothing here can post or send. I'll write {what} out so it is ready to go - the copy and the send are the two presses on your end.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf.A_NO_LIST"); }
    s_A_NO_LIST(py_tuple(vec![py_str("Nothing here keeps a list between chats - every chat starts fresh, so what you put here does not come back tomorrow. Write it where you'll see it (your phone's notes, a paper by the door) and tell me the items: I'll help you sort them."), py_str("I can't save anything, sorry - there is nowhere on my side for it to live. Say the items out loud here and I'll help you order and trim the list; then keep it in your own notes."), py_str("No memory of a list once we stop talking. Give me the lot and I'll group them by where they are in the shop; your notes app is the thing that will still have it tomorrow.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._THE_VERB_AND_THEN"); }
    s__THE_VERB_AND_THEN(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:please|pls|plz|just|ok|okay|now|and|also|so|then)\\s+)*(?:(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+|just\\s+|kindly\\s+|also\\s+)*)?(?:search|look\\s+(?:it\\s+)?up|look\\s+for|google|browse|compare|check|send|email|text|forward|share|print|post|mail|message|book|reserve|order|buy|purchase|pay|top\\s+up|renew|cancel|save|keep|store|note|jot|add|put|write\\s+(?:it|this|that)\\s+down)\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._A_LEADING_PREPOSITION"); }
    s__A_LEADING_PREPOSITION(stdlib_call("re", "compile", &[py_str("^\\s*(?:for|about|on|at|into|up|out|down|to)\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._THE_ADDRESSEE"); }
    s__THE_ADDRESSEE(stdlib_call("re", "compile", &[py_str("^\\s*(?:me|us)\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._THE_TAIL_OF_THE_ASK"); }
    s__THE_TAIL_OF_THE_ASK(stdlib_call("re", "compile", &[py_str("\\s+(?:for\\s+(?:me|us)|to\\s+(?:me|us)|please|pls|now|asap|online|thanks|ta|cheers|out|up|down|here)\\s*[.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._A_MERE_PRONOUN"); }
    s__A_MERE_PRONOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("those"), py_str("these"), py_str("one"), py_str("things")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._MINE"); }
    s__MINE(stdlib_call("re", "compile", &[py_str("\\b(my|our|mine|ours)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.onmybehalf._YOURS"); }
    s__YOURS(py_dict(vec![(py_str("my"), py_str("your")), (py_str("our"), py_str("your")), (py_str("mine"), py_str("yours")), (py_str("ours"), py_str("yours"))]));
}
pub fn _facing_them(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let _ = py_str("The person's own phrase turned to face them: \"our flight times\" -> \"your flight times\".");
    return mcall(&g__MINE(), "sub", &[{ let phrase = phrase.clone(); py_func("lambda", move |args: &[Py]| { let phrase = phrase.clone(); let found: Py = __kwp(args, 0, "found").unwrap_or(Py::None); geti(&g__YOURS(), &mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[])) }) }, phrase.clone()]);
    Py::None
}
pub fn _what_was_asked_for(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the person named in their errand, or \"it\" when they named nothing.");
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THE_VERB_AND_THEN(), "sub", args))(&__args) };
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__THE_ADDRESSEE(), "sub", args))(&__args) };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(4)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    before = after.clone();
    after = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(after.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_LEADING_PREPOSITION(), "sub", args))(&__args) }, "strip", &[]);
    after = mcall(&mcall(&g__THE_TAIL_OF_THE_ASK(), "sub", &[py_str(""), after.clone()]), "strip", &[py_str(" .!?,")]);
    if py_truthy(&Py::Bool(py_eq(&after.clone(), &before.clone()))) {
    break;
    }
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&after.clone(), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_MERE_PRONOUN(), &geti(&words.clone(), &Py::Int(0)))) } } } } } }) {
    return py_str("it");
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _is_a_flat_read(args: &[Py]) -> Py {
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
    let mut pool: Py = Py::None;
    let _ = py_str("True for the composer's flat readings this module replaces, and nothing else.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FLAT()) { let mut pool: Py = __item.clone(); __out.push(Py::Bool(py_contains(&pool.clone(), &reply.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _is_a_hedge(args: &[Py]) -> Py {
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
    let mut fallback: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut store_lines: Py = Py::None;
    let _ = py_str("True for the composer's lines that say it cannot answer, or that nothing was said yet.\n\n    A search handed to the bot is the one errand where the hedge is not good enough: \"google what\n    time the post office shuts\" was answered \"I'm not sure about that one, sorry - I'd rather not\n    guess. A quick search online will get you a solid answer\" once the line was read as the ask\n    inside it, and the honest limit below says more (what it cannot do, what the person should\n    compare, where the rest is). The same goes for the store's \"nothing said yet for that to be\"\n    lines, which answer \"google it for me\" by asking what \"it\" is - the person did say what they\n    wanted done, and the search limit is the reply that says what happens to it.\n\n    The imports are inside the call because `fallback` is the heavy one and this file is reached\n    before it, and `store_lines` is data this file does not otherwise need.\n    ");
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    store_lines = pyrt::xrefs::xmodule("memory.store_lines");
    if py_truthy(&mcall(&fallback.clone(), "is_a_hedge", &[reply.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![gattr(&store_lines.clone(), "POINTS_BACK_NOTHING"), gattr(&store_lines.clone(), "POINTING_WORD_NOTHING")])) { let mut pool: Py = __item.clone(); __out.push(Py::Bool(py_contains(&pool.clone(), &reply.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _about_an_errand(args: &[Py]) -> Py {
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
    let mut a_search: Py = Py::None;
    let _ = py_str("The pool for the errand the line hands over, or None.");
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14))) {
    return Py::None;
    }
    a_search = call_builtin("bool", &[mcall(&g__A_SEARCH(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&a_search.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__THEIR_ERRAND(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__KEEP_OUT(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&a_search.clone()) {
    return g_A_NO_SEARCH();
    }
    if py_truthy(&mcall(&g__A_SPEND(), "search", &[clean.clone()])) {
    return g_A_NO_SPEND();
    }
    if py_truthy(&mcall(&g__A_MESSAGE_OUT(), "search", &[clean.clone()])) {
    return g_A_NO_SEND();
    }
    if py_truthy(&mcall(&g__A_LIST(), "search", &[clean.clone()])) {
    return g_A_NO_LIST();
    }
    return Py::None;
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut clean: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("The honest line for an errand handed to a bot with no hands, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    pool = _about_an_errand(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_is_a_flat_read(&[reply.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_is(&pool.clone(), &g_A_NO_SEARCH())); if !py_truthy(&__b) { __b } else { _is_a_hedge(&[reply.clone()]) } })) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("what".to_string(), _facing_them(&[_what_was_asked_for(&[clean.clone()])]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), text.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.onmybehalf".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_NO_LIST" => Some(g_A_NO_LIST()),
            "A_NO_SEARCH" => Some(g_A_NO_SEARCH()),
            "A_NO_SEND" => Some(g_A_NO_SEND()),
            "A_NO_SPEND" => Some(g_A_NO_SPEND()),
            "_ASKING_BACK" => Some(g__ASKING_BACK()),
            "_A_LEADING_PREPOSITION" => Some(g__A_LEADING_PREPOSITION()),
            "_A_LIST" => Some(g__A_LIST()),
            "_A_MERE_PRONOUN" => Some(g__A_MERE_PRONOUN()),
            "_A_MESSAGE_OUT" => Some(g__A_MESSAGE_OUT()),
            "_A_SEARCH" => Some(g__A_SEARCH()),
            "_A_SPEND" => Some(g__A_SPEND()),
            "_FLAT" => Some(g__FLAT()),
            "_KEEP_OUT" => Some(g__KEEP_OUT()),
            "_MINE" => Some(g__MINE()),
            "_THEIR_ERRAND" => Some(g__THEIR_ERRAND()),
            "_THE_ADDRESSEE" => Some(g__THE_ADDRESSEE()),
            "_THE_TAIL_OF_THE_ASK" => Some(g__THE_TAIL_OF_THE_ASK()),
            "_THE_VERB_AND_THEN" => Some(g__THE_VERB_AND_THEN()),
            "_YOURS" => Some(g__YOURS()),
            "_about_an_errand" => Some(py_func("_about_an_errand", |args: &[Py]| _about_an_errand(args))),
            "_facing_them" => Some(py_func("_facing_them", |args: &[Py]| _facing_them(args))),
            "_is_a_flat_read" => Some(py_func("_is_a_flat_read", |args: &[Py]| _is_a_flat_read(args))),
            "_is_a_hedge" => Some(py_func("_is_a_hedge", |args: &[Py]| _is_a_hedge(args))),
            "_what_was_asked_for" => Some(py_func("_what_was_asked_for", |args: &[Py]| _what_was_asked_for(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "pick" => Some(g_pick()),
            "fallback_data" => Some(pyrt::xrefs::xmodule("compose.fallback_data")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_NO_LIST" => s_A_NO_LIST(v),
            "A_NO_SEARCH" => s_A_NO_SEARCH(v),
            "A_NO_SEND" => s_A_NO_SEND(v),
            "A_NO_SPEND" => s_A_NO_SPEND(v),
            "_ASKING_BACK" => s__ASKING_BACK(v),
            "_A_LEADING_PREPOSITION" => s__A_LEADING_PREPOSITION(v),
            "_A_LIST" => s__A_LIST(v),
            "_A_MERE_PRONOUN" => s__A_MERE_PRONOUN(v),
            "_A_MESSAGE_OUT" => s__A_MESSAGE_OUT(v),
            "_A_SEARCH" => s__A_SEARCH(v),
            "_A_SPEND" => s__A_SPEND(v),
            "_FLAT" => s__FLAT(v),
            "_KEEP_OUT" => s__KEEP_OUT(v),
            "_MINE" => s__MINE(v),
            "_THEIR_ERRAND" => s__THEIR_ERRAND(v),
            "_THE_ADDRESSEE" => s__THE_ADDRESSEE(v),
            "_THE_TAIL_OF_THE_ASK" => s__THE_TAIL_OF_THE_ASK(v),
            "_THE_VERB_AND_THEN" => s__THE_VERB_AND_THEN(v),
            "_YOURS" => s__YOURS(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.onmybehalf", module_obj);
    pyrt::xrefs::reg_fn("compose.onmybehalf", "_facing_them", py_func("_facing_them", |args: &[Py]| _facing_them(args)));
    pyrt::xrefs::reg_fn("compose.onmybehalf", "_what_was_asked_for", py_func("_what_was_asked_for", |args: &[Py]| _what_was_asked_for(args)));
    pyrt::xrefs::reg_fn("compose.onmybehalf", "_is_a_flat_read", py_func("_is_a_flat_read", |args: &[Py]| _is_a_flat_read(args)));
    pyrt::xrefs::reg_fn("compose.onmybehalf", "_is_a_hedge", py_func("_is_a_hedge", |args: &[Py]| _is_a_hedge(args)));
    pyrt::xrefs::reg_fn("compose.onmybehalf", "_about_an_errand", py_func("_about_an_errand", |args: &[Py]| _about_an_errand(args)));
    pyrt::xrefs::reg_fn("compose.onmybehalf", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.onmybehalf", "A_NO_LIST", g_A_NO_LIST, s_A_NO_LIST);
    pyrt::xrefs::reg_access("compose.onmybehalf", "A_NO_SEARCH", g_A_NO_SEARCH, s_A_NO_SEARCH);
    pyrt::xrefs::reg_access("compose.onmybehalf", "A_NO_SEND", g_A_NO_SEND, s_A_NO_SEND);
    pyrt::xrefs::reg_access("compose.onmybehalf", "A_NO_SPEND", g_A_NO_SPEND, s_A_NO_SPEND);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_ASKING_BACK", g__ASKING_BACK, s__ASKING_BACK);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_A_LEADING_PREPOSITION", g__A_LEADING_PREPOSITION, s__A_LEADING_PREPOSITION);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_A_LIST", g__A_LIST, s__A_LIST);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_A_MERE_PRONOUN", g__A_MERE_PRONOUN, s__A_MERE_PRONOUN);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_A_MESSAGE_OUT", g__A_MESSAGE_OUT, s__A_MESSAGE_OUT);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_A_SEARCH", g__A_SEARCH, s__A_SEARCH);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_A_SPEND", g__A_SPEND, s__A_SPEND);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_FLAT", g__FLAT, s__FLAT);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_KEEP_OUT", g__KEEP_OUT, s__KEEP_OUT);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_MINE", g__MINE, s__MINE);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_THEIR_ERRAND", g__THEIR_ERRAND, s__THEIR_ERRAND);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_THE_ADDRESSEE", g__THE_ADDRESSEE, s__THE_ADDRESSEE);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_THE_TAIL_OF_THE_ASK", g__THE_TAIL_OF_THE_ASK, s__THE_TAIL_OF_THE_ASK);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_THE_VERB_AND_THEN", g__THE_VERB_AND_THEN, s__THE_VERB_AND_THEN);
    pyrt::xrefs::reg_access("compose.onmybehalf", "_YOURS", g__YOURS, s__YOURS);
    pyrt::xrefs::reg_access("compose.onmybehalf", "pick", g_pick, s_pick);
}
