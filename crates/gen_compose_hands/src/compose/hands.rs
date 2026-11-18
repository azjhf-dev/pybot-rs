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


thread_local! { static G_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOW_TO_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOBS_UNAVAILABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOKUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_FOR_HELP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THROAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TO_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.hands: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASKED() -> Py { ensure_init(); G_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED(v: Py) { ensure_init(); G_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOW_TO_TAIL() -> Py { ensure_init(); G_A_HOW_TO_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOW_TO_TAIL(v: Py) { ensure_init(); G_A_HOW_TO_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPOSE() -> Py { ensure_init(); G_COMPOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPOSE(v: Py) { ensure_init(); G_COMPOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_FLAT() -> Py { ensure_init(); G_FLAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLAT(v: Py) { ensure_init(); G_FLAT.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_ME() -> Py { ensure_init(); G_FOR_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_ME(v: Py) { ensure_init(); G_FOR_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_JOBS() -> Py { ensure_init(); G_JOBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOBS(v: Py) { ensure_init(); G_JOBS.with(|c| *c.borrow_mut() = v); }
pub fn g_JOBS_UNAVAILABLE() -> Py { ensure_init(); G_JOBS_UNAVAILABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOBS_UNAVAILABLE(v: Py) { ensure_init(); G_JOBS_UNAVAILABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOKUP() -> Py { ensure_init(); G_LOOKUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOKUP(v: Py) { ensure_init(); G_LOOKUP.with(|c| *c.borrow_mut() = v); }
pub fn g_MAIL() -> Py { ensure_init(); G_MAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAIL(v: Py) { ensure_init(); G_MAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JOB() -> Py { ensure_init(); G_NOT_A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOB(v: Py) { ensure_init(); G_NOT_A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_FOR_HELP() -> Py { ensure_init(); G_NOT_FOR_HELP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_FOR_HELP(v: Py) { ensure_init(); G_NOT_FOR_HELP.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_PLAN() -> Py { ensure_init(); G_OWN_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_PLAN(v: Py) { ensure_init(); G_OWN_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_HERE() -> Py { ensure_init(); G_POINTS_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_HERE(v: Py) { ensure_init(); G_POINTS_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g_THROAT() -> Py { ensure_init(); G_THROAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THROAT(v: Py) { ensure_init(); G_THROAT.with(|c| *c.borrow_mut() = v); }
pub fn g_TO_ME() -> Py { ensure_init(); G_TO_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TO_ME(v: Py) { ensure_init(); G_TO_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that asks the bot to do something with its hands, or with what is on the person's screen.\n\nThe composer already has the limit for a job in the world: \"take the rubbish out\", \"put the kettle\non\", \"book me a table for two\" are read by `fallback._acts_for_me` and answered with the plain\ntruth - no hands, no phone, nothing outside the chat - plus the half of the job the bot can do.\nIts openers and its objects are the house's and the errand's, so the jobs that need a screen\nrather than a pair of legs fell straight through:\n\n    set a timer for 20 mins      -> I'm listening - say more if it helps.\n    add it to my calendar        -> I'm listening - say more if it helps.\n    print it off for me          -> Got it. Is there anything you'd like a hand with?\n    can you read my emails       -> I don't know enough to answer that honestly. A search should\n                                    sort you out quickly.\n    can you google it for me     -> I'm not sure about that one, sorry - I'd rather not guess.\n\nEvery one of those is wrong in the same way. A search cannot read a person's post or set their\ntimer, so the pointer is not merely useless, it promises that somebody else can do what nobody\nelse can; and the stock nod leaves the person believing the thing was heard, which for a job said\nto a machine reads as done. What a friend says is the limit in a few words and then the part that\ncan happen here: the words. \"Can you email my boss\" is a real ask with a real half - the message\ncan be written in the chat and sent from the person's phone.\n\nThree readings keep this off the lines that are not the ask:\n\n  - who it is said to. \"can you send this to my boss\" is an ask, \"i need to send this to my boss\"\n    is the person's own next five minutes, and answering the second with a refusal is refusing a\n    job nobody gave;\n  - what is being done. \"write a note to my mate\", \"draft an email to my boss\" and \"write a text\n    to my mum\" are the words themselves, which the bot can write, so they belong to the composer\n    that drafts (`bot/compose/draft.py`) and not to this limit - while \"write it up\" and \"type it\n    up\" are the same words going onto somebody's screen, which the bot cannot touch;\n  - whether the ask points at something here. \"print it off\", \"send this\", \"add it to my\n    calendar\" point back at the chat; \"text him or call\" points at a person and is the person's\n    own choice, so it keeps whatever the composer says.\n\nIt only ever speaks where the composer's own line was a place: the honest hedge pools (`UNKNOWN`\nand its family) or the stock nod (`NEUTRAL`), and the \"job in the world\" pool for the asks whose\nreal half is the writing. A line the composer answers keeps its answer, and the one ask it already\nanswers well - \"can you search the internet\" is read as about-the-bot and told that nothing is\nlive on this end - is not said twice in different words.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.THROAT"); }
    s_THROAT(py_str("(?:(?:ok|okay|right|so|hi|hello|hey|oi|and|oh|ah|er|um|umm|hmm|quick|just|also|sorry|now|then|again)\\s+)*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.ASKED"); }
    s_ASKED(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^"), &g_THROAT()), &py_str("(?:(?:can|could|would|will|might)\\s+(?:you|u|ya)\\b|(?:please|pls|kindly)\\b|i\\s+(?:want|need|would\\s+like|would\\s+love)\\s+(?:you|u)\\s+to\\b)"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.FOR_ME"); }
    s_FOR_ME(stdlib_call("re", "compile", &[py_str("\\b(?:for me|for us|on my behalf)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.FLAT"); }
    s_FLAT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^"), &g_THROAT()), &py_str("(?:set|start|put|add|note|jot|print|scan|send|text|email|forward|upload|download|attach|sign|fill|read|translate|record|play|pause|skip|cue|turn|switch|stick|keep|write|type)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.POINTS_HERE"); }
    s_POINTS_HERE(stdlib_call("re", "compile", &[py_str("\\b(?:it|this|that|these|those)\\b|\\bfor me\\b|\\b(?:my|the)\\s+(?:calendar|diary|planner|timer|notes?|alarm|phone|minutes|transcript|report|letter|essay|forms?|pages?|list|draft|script)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.COMPOSE"); }
    s_COMPOSE(stdlib_call("re", "compile", &[py_str("\\b(?:draft|compose|word|reword|put\\s+together)\\b|\\bwrite\\s+(?:\\w+\\s+){0,2}(?:text|message|email|letter|note|reply|card|email)\\b|\\bnotes?\\s+(?:for|to)\\s")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.JOBS"); }
    s_JOBS(py_tuple(vec![py_tuple(vec![py_str("timer"), stdlib_call("re", "compile", &[py_str("\\b(?:timer|countdown|stopwatch|alarm)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("diary"), stdlib_call("re", "compile", &[py_str("\\b(?:calendar|diary|planner)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("note"), stdlib_call("re", "compile", &[py_str("\\b(?:note\\s+(?:that|it|this)|jot\\s+(?:that|it|this)\\s+down|write\\s+(?:it|this|that)\\s+down|make\\s+a\\s+note|keep\\s+a\\s+note|type\\s+up\\s+(?:my|the|this|these|those)\\s+(?:notes?|minutes|transcript))\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("send"), stdlib_call("re", "compile", &[py_str("\\b(?:send|sending|forward|post|share|print|scan|upload|download|attach|sign|fill\\s+in|(?:text|email|message)\\s+(?!to\\b)|write\\s+(?:it|this|that)\\s+up|type\\s+(?:it|this|that|these|those)\\s+up|type\\s+up\\s+(?:my|the|this|that|these|those|your)\\b)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("play"), stdlib_call("re", "compile", &[py_str("\\b(?:play|playing|pause|unpause|skip|stop|start|cue|turn|switch|stick|put\\s+on)\\b(?:\\s+\\w+){0,3}?\\s+(?:music|song|songs|playlist|track|album|radio|telly|tv|video|podcast|episode|record)\\b|\\b(?:telly|tv|radio|music|volume|sound)\\s+(?:up|down|on|off|louder|quieter)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("read"), stdlib_call("re", "compile", &[py_str("\\b(?:read\\s+(?:it|this|that|them)\\s+(?:out|aloud|to\\s+me|for\\s+me)|read\\s+(?:my|the|this|these|those)\\s+(?:emails?|messages?|letter|page|screen|file|document|text)\\b|read\\s+(?:it\\s+)?out\\s+loud|translate|summari[sz]e|proofread)\\b")]).unwrap_or_else(|e| exc::raise(e))])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.TO_ME"); }
    s_TO_ME(stdlib_call("re", "compile", &[py_str("\\b(?:send|text|email|message|post|share|read|write|print)\\s+(?:me|us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.LOOKUP"); }
    s_LOOKUP(stdlib_call("re", "compile", &[py_str("\\b(?:(?:google|search)\\s+(?:it|that|this|something)\\b|look\\s+(?:it|that|this|something)\\s+up\\b|look\\s+up\\s+(?:the|a|an|some|my)\\b|check\\s+(?:it|that|this)\\s+(?:online|up)\\b|read\\s+(?:me\\s+)?(?:the|today'?s|this\\s+morning'?s)\\s+(?:news|headlines|papers?|weather|scores?|forecast)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.A_HOW_TO_TAIL"); }
    s_A_HOW_TO_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+to|how\\s+do|how\\s+can|what|why|when|where|which|who|if|whether)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.NOT_FOR_HELP"); }
    s_NOT_FOR_HELP(stdlib_call("re", "compile", &[py_str("\\b(?:send|get|call|fetch|find)\\s+(?:help|someone|somebody|anyone|anybody|the\\s+police|an?\\s+ambulance|a\\s+doctor|medical)\\b|\\b(?:send|get|call|fetch|ring)\\s+for\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.OWN_PLAN"); }
    s_OWN_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:in the morning|tomorrow|tonight|later on|next week|when i get (?:home|back)|before i forget|after (?:work|lunch|tea))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.NOT_A_JOB"); }
    s_NOT_A_JOB(stdlib_call("re", "compile", &[py_str("^(?:how\\b|whats\\b|what\\s+(?:is|are|should)\\b|where\\b|when\\b|why\\b|who\\b|which\\b|(?:can|could|would|will)\\s+(?:you|u)\\s+(?:tell|show|explain|teach|help|suggest|recommend|advise|remind|check|look|find|search|google|see|hear|watch|open|remember|know|think|guess|say|give|list|make|do|work|count|spell)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.JOBS_UNAVAILABLE"); }
    s_JOBS_UNAVAILABLE(py_dict(vec![(py_str("timer"), py_tuple(vec![py_str("Set that on the phone's timer - nothing on this end can ring or buzz, so the phone is the one to trust. What's it timing?"), py_str("I can't set a timer, no - no hands and nothing that buzzes. Your phone's clock app will, and tell me what it's for and I'll help you with the rest of it.")])), (py_str("diary"), py_tuple(vec![py_str("That has to go in your calendar rather than mine - I can't put anything in there. Tell me the date and what it's for, and I'll help you work out what has to happen before it."), py_str("Nothing I can add to a diary from this end, sorry. Say when it is and I'll help you plan backwards from it.")])), (py_str("note"), py_tuple(vec![py_str("I can't keep a note past this chat, sorry. Say the line here and I'll help you get it short and plain, so it is worth saving the moment you copy it out."), py_str("I can't save a note anywhere you'd see it later, sorry. Say it here and I'll hold it while we talk; copy it into your notes app before you go.")])), (py_str("send"), py_tuple(vec![py_str("I can't send, print or attach anything - I'm words in this chat, with nothing to post from. Paste the text here and I'll get the wording right, ready for you to send."), py_str("That needs hands I haven't got, sorry. The words I can do: give me the message and I'll tidy it, shorten it or say it better.")])), (py_str("play"), py_tuple(vec![py_str("I can't play anything - there is no speaker on this end, only words. Put the radio or the telly on, and tell me what you're in the mood for and I'll suggest something."), py_str("Nothing here to play it on, sorry. Say what kind of thing you fancy and I'll help you pick, then it's the remote or your own speaker that does the rest.")])), (py_str("read"), py_tuple(vec![py_str("The page is on your side of the chat, not mine - paste the bit you want and I'll read it, shorten it, or put it in other words."), py_str("I can't see the file or the screen, sorry. Copy the words into the chat and I'll take it from there."), py_str("No inbox on this end, sorry - nothing arrives here. Your mail app has it open in a tap, and paste me the message you want to answer and I'll help you word the reply.")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.hands.MAIL"); }
    s_MAIL(stdlib_call("re", "compile", &[py_str("\\b(?:e-?mails?|inbox|mailbox|mail|messages?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _bare(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The line as these readings see it: lowercase, no punctuation, no apostrophes.");
    said = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "replace", &[py_str("'"), py_str("")]);
    return mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("[^a-z0-9 ]+"), py_str(" "), said.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    Py::None
}
pub fn reads(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("The job this line asks the bot to do, or None.\n\n    One of \"timer\", \"diary\", \"note\", \"send\", \"play\", \"read\" or \"lookup\", and only for a line that asks\n    the bot itself. A how-to, the person's own errand, a report of a job already done, an ask for\n    the words rather than for the sending of them, and anything the bot can answer from what it\n    knows are all left to the part that knows about them.\n    ");
    line = _bare(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(18)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_TO_ME(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_COMPOSE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_FOR_HELP(), "search", &[line.clone()]) } } } }) {
    return Py::None;
    }
    asked = { let __b = call_builtin("bool", &[mcall(&g_ASKED(), "match", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_FOR_ME(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_LOOKUP(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_HOW_TO_TAIL(), "search", &[line.clone()]))) } } } }) {
    return py_str("lookup");
    }
    if py_truthy(&mcall(&g_NOT_A_JOB(), "match", &[line.clone()])) {
    return Py::None;
    }
    flat = call_builtin("bool", &[mcall(&g_FLAT(), "match", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&flat.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_JOBS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    shape = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&shape.clone(), "search", &[line.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = flat.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asked.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("send"), py_str("note")]), &kind.clone())); if py_truthy(&__b) { __b } else { mcall(&g_OWN_PLAN(), "search", &[line.clone()]) } } } } } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_POINTS_HERE(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_OWN_PLAN(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    }
    return kind.clone();
    }
    return Py::None;
    Py::None
}
pub fn writes_the_words(args: &[Py]) -> Py {
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
    let _ = py_str("True for the asks whose half the bot can do is the writing: a message, a note, a page.\n\n    `_acts_for_me` in the composer answers a job in the world with \"what to say when you ring\",\n    which is the right half of a phone call and the wrong one of an email: the thing asked for\n    there is the words themselves.\n    ");
    return Py::Bool(py_contains(&py_tuple(vec![py_str("send"), py_str("note"), py_str("read"), py_str("play")]), &reads(&[text.clone()])));
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
    let mut fallback: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("The reply for an ask the bot has no hands for, or None when the line is not one.");
    kind = reads(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("lookup")))) {
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    return pyrt::xrefs::xcall("compose", "pick", &[geti(&gattr(&fallback.clone(), "ABOUT_SELF"), &py_str("online")), memory.clone(), _bare(&[text.clone()])]);
    }
    pool = geti(&g_JOBS_UNAVAILABLE(), &kind.clone());
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("read")))) {
    pool = if py_truthy(&mcall(&g_MAIL(), "search", &[_bare(&[text.clone()])])) { py_tuple(vec![geti(&pool.clone(), &Py::Int(2))]) } else { slice_or_raise(&pool.clone(), &Py::None, &Py::Int(2), &Py::None) };
    }
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), _bare(&[text.clone()])]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.hands".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKED" => Some(g_ASKED()),
            "A_HOW_TO_TAIL" => Some(g_A_HOW_TO_TAIL()),
            "COMPOSE" => Some(g_COMPOSE()),
            "FLAT" => Some(g_FLAT()),
            "FOR_ME" => Some(g_FOR_ME()),
            "JOBS" => Some(g_JOBS()),
            "JOBS_UNAVAILABLE" => Some(g_JOBS_UNAVAILABLE()),
            "LOOKUP" => Some(g_LOOKUP()),
            "MAIL" => Some(g_MAIL()),
            "NOT_A_JOB" => Some(g_NOT_A_JOB()),
            "NOT_FOR_HELP" => Some(g_NOT_FOR_HELP()),
            "OWN_PLAN" => Some(g_OWN_PLAN()),
            "POINTS_HERE" => Some(g_POINTS_HERE()),
            "THROAT" => Some(g_THROAT()),
            "TO_ME" => Some(g_TO_ME()),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "pick" => Some(g_pick()),
            "reads" => Some(py_func("reads", |args: &[Py]| reads(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "writes_the_words" => Some(py_func("writes_the_words", |args: &[Py]| writes_the_words(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKED" => s_ASKED(v),
            "A_HOW_TO_TAIL" => s_A_HOW_TO_TAIL(v),
            "COMPOSE" => s_COMPOSE(v),
            "FLAT" => s_FLAT(v),
            "FOR_ME" => s_FOR_ME(v),
            "JOBS" => s_JOBS(v),
            "JOBS_UNAVAILABLE" => s_JOBS_UNAVAILABLE(v),
            "LOOKUP" => s_LOOKUP(v),
            "MAIL" => s_MAIL(v),
            "NOT_A_JOB" => s_NOT_A_JOB(v),
            "NOT_FOR_HELP" => s_NOT_FOR_HELP(v),
            "OWN_PLAN" => s_OWN_PLAN(v),
            "POINTS_HERE" => s_POINTS_HERE(v),
            "THROAT" => s_THROAT(v),
            "TO_ME" => s_TO_ME(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.hands", module_obj);
    pyrt::xrefs::reg_fn("compose.hands", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("compose.hands", "reads", py_func("reads", |args: &[Py]| reads(args)));
    pyrt::xrefs::reg_fn("compose.hands", "writes_the_words", py_func("writes_the_words", |args: &[Py]| writes_the_words(args)));
    pyrt::xrefs::reg_fn("compose.hands", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.hands", "ASKED", g_ASKED, s_ASKED);
    pyrt::xrefs::reg_access("compose.hands", "A_HOW_TO_TAIL", g_A_HOW_TO_TAIL, s_A_HOW_TO_TAIL);
    pyrt::xrefs::reg_access("compose.hands", "COMPOSE", g_COMPOSE, s_COMPOSE);
    pyrt::xrefs::reg_access("compose.hands", "FLAT", g_FLAT, s_FLAT);
    pyrt::xrefs::reg_access("compose.hands", "FOR_ME", g_FOR_ME, s_FOR_ME);
    pyrt::xrefs::reg_access("compose.hands", "JOBS", g_JOBS, s_JOBS);
    pyrt::xrefs::reg_access("compose.hands", "JOBS_UNAVAILABLE", g_JOBS_UNAVAILABLE, s_JOBS_UNAVAILABLE);
    pyrt::xrefs::reg_access("compose.hands", "LOOKUP", g_LOOKUP, s_LOOKUP);
    pyrt::xrefs::reg_access("compose.hands", "MAIL", g_MAIL, s_MAIL);
    pyrt::xrefs::reg_access("compose.hands", "NOT_A_JOB", g_NOT_A_JOB, s_NOT_A_JOB);
    pyrt::xrefs::reg_access("compose.hands", "NOT_FOR_HELP", g_NOT_FOR_HELP, s_NOT_FOR_HELP);
    pyrt::xrefs::reg_access("compose.hands", "OWN_PLAN", g_OWN_PLAN, s_OWN_PLAN);
    pyrt::xrefs::reg_access("compose.hands", "POINTS_HERE", g_POINTS_HERE, s_POINTS_HERE);
    pyrt::xrefs::reg_access("compose.hands", "THROAT", g_THROAT, s_THROAT);
    pyrt::xrefs::reg_access("compose.hands", "TO_ME", g_TO_ME, s_TO_ME);
    pyrt::xrefs::reg_access("compose.hands", "pick", g_pick, s_pick);
}
