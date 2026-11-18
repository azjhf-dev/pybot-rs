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


thread_local! { static G_ABOUT_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALL_CLEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOOD_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SPAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TREATMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WINDOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARER_ON_THE_DAY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARER_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_PLAN_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_TREATMENT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLEARED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMERGENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_MAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFICE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRAISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRADE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAITING_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YOU_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLEAR_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_SOMEONE_ELSE() -> Py { ensure_init(); G_ABOUT_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_SOMEONE_ELSE(v: Py) { ensure_init(); G_ABOUT_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_ALL_CLEAR() -> Py { ensure_init(); G_ALL_CLEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALL_CLEAR(v: Py) { ensure_init(); G_ALL_CLEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED() -> Py { ensure_init(); G_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED(v: Py) { ensure_init(); G_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHANGE() -> Py { ensure_init(); G_A_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHANGE(v: Py) { ensure_init(); G_A_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOOD_SAID() -> Py { ensure_init(); G_A_GOOD_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOOD_SAID(v: Py) { ensure_init(); G_A_GOOD_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN() -> Py { ensure_init(); G_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN(v: Py) { ensure_init(); G_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SPAN() -> Py { ensure_init(); G_A_SPAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SPAN(v: Py) { ensure_init(); G_A_SPAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME() -> Py { ensure_init(); G_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME(v: Py) { ensure_init(); G_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TREATMENT() -> Py { ensure_init(); G_A_TREATMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TREATMENT(v: Py) { ensure_init(); G_A_TREATMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WINDOW() -> Py { ensure_init(); G_A_WINDOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WINDOW(v: Py) { ensure_init(); G_A_WINDOW.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE() -> Py { ensure_init(); G_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE(v: Py) { ensure_init(); G_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARER_ON_THE_DAY_REPLIES() -> Py { ensure_init(); G_CARER_ON_THE_DAY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARER_ON_THE_DAY_REPLIES(v: Py) { ensure_init(); G_CARER_ON_THE_DAY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CARER_REPLIES() -> Py { ensure_init(); G_CARER_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARER_REPLIES(v: Py) { ensure_init(); G_CARER_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_PLAN_REPLIES() -> Py { ensure_init(); G_CARE_PLAN_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_PLAN_REPLIES(v: Py) { ensure_init(); G_CARE_PLAN_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_REPLIES() -> Py { ensure_init(); G_CARE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_REPLIES(v: Py) { ensure_init(); G_CARE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_TREATMENT_REPLIES() -> Py { ensure_init(); G_CARE_TREATMENT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_TREATMENT_REPLIES(v: Py) { ensure_init(); G_CARE_TREATMENT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CLEARED() -> Py { ensure_init(); G_CLEARED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLEARED(v: Py) { ensure_init(); G_CLEARED.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE() -> Py { ensure_init(); G_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE(v: Py) { ensure_init(); G_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g_EMERGENCY() -> Py { ensure_init(); G_EMERGENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY(v: Py) { ensure_init(); G_EMERGENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD_NEWS() -> Py { ensure_init(); G_GOOD_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD_NEWS(v: Py) { ensure_init(); G_GOOD_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_MAN() -> Py { ensure_init(); G_MY_MAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_MAN(v: Py) { ensure_init(); G_MY_MAN.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFICE() -> Py { ensure_init(); G_OFFICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFICE(v: Py) { ensure_init(); G_OFFICE.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFICE_REPLIES() -> Py { ensure_init(); G_OFFICE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFICE_REPLIES(v: Py) { ensure_init(); G_OFFICE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHERS() -> Py { ensure_init(); G_OTHERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHERS(v: Py) { ensure_init(); G_OTHERS.with(|c| *c.borrow_mut() = v); }
pub fn g_POOLS() -> Py { ensure_init(); G_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOLS(v: Py) { ensure_init(); G_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRAISE() -> Py { ensure_init(); G_PRAISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRAISE(v: Py) { ensure_init(); G_PRAISE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN() -> Py { ensure_init(); G_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN(v: Py) { ensure_init(); G_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID() -> Py { ensure_init(); G_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID(v: Py) { ensure_init(); G_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF_SAID() -> Py { ensure_init(); G_SELF_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF_SAID(v: Py) { ensure_init(); G_SELF_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_REPLIES() -> Py { ensure_init(); G_SPAN_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_REPLIES(v: Py) { ensure_init(); G_SPAN_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRADE() -> Py { ensure_init(); G_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRADE(v: Py) { ensure_init(); G_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRADE_REPLIES() -> Py { ensure_init(); G_TRADE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRADE_REPLIES(v: Py) { ensure_init(); G_TRADE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_WAITING_REPLIES() -> Py { ensure_init(); G_WAITING_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAITING_REPLIES(v: Py) { ensure_init(); G_WAITING_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_YOU_SAID() -> Py { ensure_init(); G_YOU_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YOU_SAID(v: Py) { ensure_init(); G_YOU_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__CLEAR_WORDS() -> Py { ensure_init(); G__CLEAR_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLEAR_WORDS(v: Py) { ensure_init(); G__CLEAR_WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Somebody with authority gave them a verdict, and the turn is the decision that follows it.\n\n\"the plumber said the whole lot needs replacing\" was answered \"Got it - the plumber said the whole\nlot needs replacing\", \"the doctor said to come back in a week\" with \"Got it. Is there anything\nyou'd like a hand with?\", and the roofer's six weeks with the card for botched work. The line is\nnever the trouble: the person already knows the words, and what a friend adds is what to do with\nthem - a quote in writing, a second price, what to ask before agreeing, when to ring back.\n\nFour kinds, because the decision differs:\n\n* a trade who has priced or condemned a thing (\"the plumber\", \"the garage\", \"the roofer\") - the\n  money and the second opinion, and the bot can say whether a figure is in the usual range;\n* a clinician with a plan (\"the doctor said to come back in a week\", \"the physio\") - the wait and\n  what to do inside it, and that ringing back sooner is allowed;\n* an office with a rule (\"the council\", \"the bank\", \"the school\") - the reason in writing and the\n  step their own process has next;\n* a date or a visit (\"it would take six weeks\", \"he will be here between 8 and 12\") - the wait\n  itself and the call to make when the day goes by, whoever said it.\n\nEverything here is a guard against taking a turn that is not this:\n\n* never a question. \"the garage says it needs new discs, is 350 about right\" names a trade and a\n  verdict, and the card behind it answers the figure; asked with a question mark the turn is the\n  question, so this passes;\n* never an emergency. \"the doctor said to go to A&E now\" and \"she said to ring 999\" belong to the\n  safety module, whatever else is in the line;\n* never a clinician's verdict about somebody else (\"my mum\", \"she\", \"his bloods\"): a body that is\n  not the person's own is safety's turn;\n* never a no that names nobody (\"they said no\"): the pools that answer a refusal already do it\n  better than a stock line about a trade;\n* never the person's own words (\"i said that\", \"my sister said i should go\"): a verdict here is\n  somebody with authority, not anybody who spoke.\n");
    s_OTHERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("his"), py_str("hers"), py_str("they"), py_str("them"), py_str("their"), py_str("theirs"), py_str("mum"), py_str("mam"), py_str("mom"), py_str("mummy"), py_str("dad"), py_str("daddy"), py_str("father"), py_str("mother"), py_str("parent"), py_str("parents"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("toddler"), py_str("teenager"), py_str("boy"), py_str("girl"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("grandparents"), py_str("brother"), py_str("sister"), py_str("sibling"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("nephew"), py_str("niece"), py_str("friend"), py_str("mate"), py_str("flatmate"), py_str("housemate"), py_str("neighbour"), py_str("neighbor"), py_str("colleague"), py_str("boss"), py_str("manager"), py_str("carer"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("pet"), py_str("pup")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:said|says|say|told\\s+me|reckons|quoted|quotes)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SELF_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi\\s+(?:said|say|says|told)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_YOU_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\byou\\s+(?:said|says|say|told\\s+me)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("doctor"), py_str("doctors"), py_str("gp"), py_str("nurse"), py_str("consultant"), py_str("physio"), py_str("physiotherapist"), py_str("midwife"), py_str("paramedic"), py_str("hospital"), py_str("clinic"), py_str("surgery"), py_str("radiographer"), py_str("optician"), py_str("podiatrist"), py_str("chiropodist"), py_str("dietician"), py_str("dietitian"), py_str("111"), py_str("nhs"), py_str("dentist"), py_str("dentists"), py_str("optometrist"), py_str("optometrists")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TRADE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("plumber"), py_str("mechanic"), py_str("garage"), py_str("garages"), py_str("roofer"), py_str("builder"), py_str("electrician"), py_str("engineer"), py_str("locksmith"), py_str("dealer"), py_str("fitter"), py_str("joiner"), py_str("tiler"), py_str("plasterer"), py_str("decorator"), py_str("glazier"), py_str("man"), py_str("tradesman"), py_str("contractor"), py_str("boiler"), py_str("drain"), py_str("drains"), py_str("surveyor")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OFFICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("council"), py_str("bank"), py_str("landlord"), py_str("landlady"), py_str("insurer"), py_str("insurance"), py_str("dwp"), py_str("jobcentre"), py_str("school"), py_str("teacher"), py_str("college"), py_str("university"), py_str("letting"), py_str("agent"), py_str("mortgage"), py_str("broker"), py_str("solicitor"), py_str("conveyancer"), py_str("planning"), py_str("housing"), py_str("ombudsman"), py_str("tribunal"), py_str("assessment"), py_str("assessor"), py_str("office"), py_str("authority")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EMERGENCY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:999|911|112|116\\s*123|ambulance|a\\s*&\\s*e|a\\s+and\\s+e|casualty|resus|crash\\s+team|blue\\s+light)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\?|\\b(?:should\\s+i|shall\\s+i|is\\s+(?:that|it|this)|is\\s+\\d|how\\s+(?:much|long)|worth\\s+it|about\\s+right|too\\s+much|fair|reasonable|can\\s+i|could\\s+i)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:would\\s+take|will\\s+take|takes\\s+(?:\\w+\\s+)?(?:weeks?|months?|days?)|(?:two|three|four|five|six|seven|eight|nine|ten|twelve|\\d+)\\s+(?:weeks?|months?|days?|fortnights?)|(?:in|for|of|after)\\s+an?\\s+(?:week|fortnight|month|day|few\\s+days)|(?:coming|due|here|back|round|in)\\s+(?:on\\s+|at\\s+|in\\s+|by\\s+)?(?:the\\s+)?(?:mon|tues|wednes|thurs|fri|satur|sun)day|(?:coming|due|here|back)\\s+(?:on\\s+|at\\s+|in\\s+)?(?:the\\s+)?(?:morning|afternoon|evening|weekend|afternoon|\\d{1,2}(?:[.:]\\d\\d)?\\s*(?:am|pm)?)\\b|(?:coming|due|here|back)\\s+(?:tomorrow|tonight|today|next\\s+\\w+|this\\s+\\w+)|be\\s+here\\s+(?:between|at|on|in|by)|sometime\\s+(?:next|this)\\s+week|(?:come|comes|coming|call|call\\s+in|turn\\s+up)\\s+(?:back\\s+)?(?:in\\s+the\\s+|this\\s+)?(?:morning|afternoon|evening)|first\\s+week\\s+of\\s+\\w+)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PLAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:come\\s+back|coming\\s+back|go\\s+back|come\\s+in\\s+again|refer\\w*|review\\w*|follow\\s*-?\\s*up|check\\s*-?\\s*up|wait\\w*|keep\\s+an\\s+eye|monitor\\w*|bloods?|scan\\w*|x-?rays?|prescription|appointment|another\\s+look|see\\s+how\\s+\\w+\\s+goes)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TREATMENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:crown|crowns|filling|fillings|root\\s+canal|extraction|implant|implants|bridge|dentures?|braces|invisalign|scale\\s+and\\s+polish|glasses|lenses|varifocals|needs?\\s+(?:doing|replacing|seeing\\s+to)|has\\s+to\\s+come\\s+out)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_CHANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:changed|worse|a\\s+bit\\s+high|borderline|not\\s+where\\s+it\\s+should\\s+be|something\\s+there|wants?\\s+watching)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ALL_CLEAR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|was|were|come|comes|came)\\s+(?:back\\s+)?(?:all\\s+)?(?:fine|ok|okay|normal|clear|negative|healthy|nothing)\\b|\\bnothing\\s+(?:wrong|to\\s+worry|sinister|serious)\\b|\\ball\\s+clear\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:fixed|sorted|done|finished|put\\s+right|all\\s+working|working\\s+now|no\\s+charge|on\\s+the\\s+mend)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRAISE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:thanks?|thank\\s+you|cheers|ta|well\\s+done|good\\s+(?:work|job)|lovely|great|brilliant|excellent|fantastic|perfect|happy|pleased|delighted|ready|no\\s+charge|free\\s+of\\s+charge)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GOOD_NEWS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bsaid\\s+(?:that\\s+)?(?:i|we|you)\\s+(?:can|could|will|may)\\s+have\\b|\\b(?:approved|accepted|went\\s+through)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MY_MAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|her|his|their)\\s+(?:man|bloke|guy|fella|chap)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TRADE_REPLIES(py_tuple(vec![py_str("Get it in writing, with the parts and the labour apart, and one more quote before you say yes - two prices is the whole trick. Tell me the figure and I'll say whether it's in the usual range for the job."), py_str("Ask what the cheapest version is that would still fix it, and what happens if it turns out to be something else once it's open. Put the figure to me and I'll tell you whether it's the going rate."), py_str("Then ask for it in writing and price the job two ways - the whole lot, and the least that would get you through the year. If you give me the number I'll say whether it's fair.")]));
    s_CARE_REPLIES(py_tuple(vec![py_str("Ring back before then if it gets worse - you don't have to sit out the wait for the date. Keep a short note of what changes day to day so the next appointment is worth having."), py_str("Then go with the plan, and ring the surgery the moment something changes rather than waiting for the date. Write down what you want to ask so the ten minutes aren't wasted."), py_str("That's the plan, then. Ask what would make them bring it forward, and keep a note of what happens between now and then - that note is what makes the next appointment quick.")]));
    s_CARE_PLAN_REPLIES(py_tuple(vec![py_str("Then do as they said and keep a short note of what changes - what you noticed, when, and what made it better or worse. That note is exactly what they will ask for next time."), py_str("Keep it simple and written down: date, what you noticed, and anything that made a difference. If it changes before you are seen again, ring and say so rather than waiting."), py_str("Then watch for the change they named, and write down the first day you see it. A date and a description is what turns a worry into something they can act on.")]));
    s_CARE_TREATMENT_REPLIES(py_tuple(vec![py_str("Ask them two things before anything is agreed: what it means in plain words, and what happens if you do nothing for a while. If it is treatment, get the price both ways - on the NHS band and privately."), py_str("Get it in plain words first: what changes if you have it done, what happens if you wait, and what it costs either way. Write the answers down while they are fresh."), py_str("Worth asking what the next step is and when it would need doing by. If money comes into it, ask for the price on the NHS and privately before you book anything.")]));
    s_OFFICE_REPLIES(py_tuple(vec![py_str("Ask for it in writing, and ask what the next step in their own process is - most of these have a form or an appeal with a deadline, and whoever answers the phone will say which. Tell me what the letter says and I'll help you answer it."), py_str("Get their reason in writing and ask what would change the answer. That reply is what an appeal or a second go hangs on, and I'll help you word it."), py_str("Then ask them to confirm it in writing and how long the next step takes. Once you have their words I'll help you put your side of it.")]));
    s_WAITING_REPLIES(py_tuple(vec![py_str("Then keep the time clear and have what they need where they can get at it - paperwork, the boiler cupboard, the car keys on the table. Ring the office if nobody has turned up by the end of the slot; that call is what moves it."), py_str("Put the day aside rather than the hour, and ring the office the moment the slot goes by without a word. If it slips again, ask them to hold the next appointment for you there and then."), py_str("Then plan as if it happens and ring early if it doesn't - people who chase on the day get moved up the list. Have the details they will ask for written down: the job, the reference, and your number twice.")]));
    s_A_SPAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:take|takes|took|taking|would\\s+take|will\\s+take)\\b[^.]{0,20}?\\b(?:two|three|four|five|six|seven|eight|nine|ten|twelve|\\d+)\\s+(?:weeks?|months?|days?|fortnights?)\\b|\\b(?:a|an|one)\\s+(?:week|month|fortnight|day)\\b|\\b(?:in|for|of|after|another)\\s+(?:two|three|four|five|six|\\d+)\\s+(?:weeks?|months?)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WINDOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d{1,2}(?::\\d\\d)?\\s*(?:am|pm)\\b|\\bbetween\\s+\\d+|\\bin\\s+the\\s+(?:morning|afternoon|evening)\\b|\\bany\\s+time\\s+after\\b|\\bo'?clock\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SPAN_REPLIES(py_tuple(vec![py_str("Ask when they start, not when they finish - a start date in writing is the only part anyone can be held to, and everything after it follows. Then agree what happens if the weather or the scaffold puts it back, and keep the last payment until the work is signed off."), py_str("Get the start date and the end date in writing, with what counts as a delay and who pays for it. A job that long is worth a short weekly update by text, in one line, so nothing drifts without you knowing."), py_str("Then put the dates in your diary and ask them to confirm the start in writing a week before. If it slips, ask for a new date there and then rather than ringing back later - the date they say out loud is the one that sticks.")]));
    s_POOLS(py_tuple(vec![py_tuple(vec![g_CARE(), g_CARE_REPLIES()]), py_tuple(vec![g_TRADE(), g_TRADE_REPLIES()]), py_tuple(vec![g_OFFICE(), g_OFFICE_REPLIES()])]));
    s_PRONOUN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|him|her|they|them|his|their)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ABOUT_SOMEONE_ELSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:said|says|say|told\\s+me)\\s+(?:that\\s+)?(?:he|she)(?:'?s|'?ll|'?d)?\\b|\\b(?:said|says|say|told\\s+me)\\s+(?:that\\s+)?my\\s+[a-z]+\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CARER_REPLIES(py_tuple(vec![py_str("Ask what it is for, who does it, and what changes in the meantime - and ask them to put the answer in an email so you have it."), py_str("Ask which one they mean and who runs it, then when you would hear - a name and a date stop it being a guess."), py_str("Ask what happens if you decide to wait before agreeing to it. That is a fair question and they should be able to answer it in plain words.")]));
    s_CARER_ON_THE_DAY_REPLIES(py_tuple(vec![py_str("Put the date in your phone, and ask now what they need from you beforehand, and who to ring if it moves."), py_str("Ask what to bring and whether anything changes between now and then, and keep the number of the department that rang you.")]));
    s__CLEAR_WORDS(py_str("doing\\s+(?:well|great|fine|brilliantly)|all\\s+clear|(?:is|are|was|were|looks|look|seems|seem|sounds|sound)\\s+(?:all\\s+)?(?:fine|ok|okay|alright|sound|clear|good|healthy|safe|solid)|its\\s+(?:fine|ok|okay|alright|sound)|nothing\\s+(?:wrong|to\\s+worry\\s+about|serious|to\\s+do|to\\s+pay)|no\\s+(?:problems?|issues?|faults?|damage)|is\\s+(?:well|better)|got\\s+the\\s+all\\s+clear"));
    s_CLEARED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:"), &g__CLEAR_WORDS()), &py_str(")\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_GOOD_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:"), &g__CLEAR_WORDS()), &py_str("|happy|pleased|proud|through\\s+it|can\\s+(?:have|stay|go|come\\s+home)|is\\s+allowed|well\\s+done)\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9'&]+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_subject(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let _ = py_str("The words before the saying verb: the party is the one who spoke.\n\n    \"the garage said it needs new discs\" puts the party before the verb, and so does \"my dentist\n    told me\". The words after it are what was said, and a party word there is somebody else in\n    their words, not the speaker: \"he says nobody at school likes him\" is a boy talking about a\n    school, and reading `school` as the party gave it \"Ask for it in writing, and ask what the next\n    step in their own process is\" - office advice for a child who is being unkind to himself\n    (good-shelf on #9097).\n    ");
    head = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAID(), "split", args))(&__args) }, &Py::Int(0));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) };
    Py::None
}
pub fn _the_pool(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut parties: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("The pool for the party in the line, or None.");
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&g_POOLS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    parties = __unpk2[0].clone();
    pool = __unpk2[1].clone();
    if py_truthy(&op_bitand(&named.clone(), &parties.clone())) {
    return pool.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _last_party(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut game: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "game") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: game"))) };
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The pool for the party named in the last turn or two, or None.\n\n    `game` is the memory dict: the turns are the person's and the bot's, and either can carry the\n    party (\"my boiler keeps losing pressure\" then \"he said its two days\").\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(game.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&game.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turn.clone(), &py_str("user"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    }
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    text = __item.clone();
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_pool(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &g_TRADE_REPLIES()))) {
    return pool.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _recent_bot_lines(args: &[Py]) -> Py {
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
    let _ = py_str("The bot's last few replies, one lower-case string each.");
    said = py_list(vec![]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turn.clone(), &py_str("bot"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&memory.clone(), &py_str("bot"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push(line.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _choose(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut head: Py = Py::None;
    let mut line: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The pool's line, avoiding the ones already said in this conversation.");
    recent = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _recent_bot_lines(args))(&__args) };
    let __iter1 = py_iter_or_raise(&pool.clone());
    for __item in __iter1 {
    line = __item.clone();
    head = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &Py::None, &Py::Int(40), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&recent.clone()) { let mut said: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &head.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return line.clone();
    }
    }
    return geti(&pool.clone(), &Py::Int(0));
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
    let mut clean: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The reply for a verdict reported back to the bot, or None.");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAID(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_EMERGENCY(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SELF_SAID(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_YOU_SAID(), "search", args))(&__args) } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DONE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ALL_CLEAR(), "search", args))(&__args) } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TIME(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRAISE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TIME(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLAN(), "search", args))(&__args) } } })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MY_MAN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GOOD_NEWS(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLEARED(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TIME(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLAN(), "search", args))(&__args) } } })) } }) {
    return Py::None;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words(args))(&__args) };
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_subject(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_pool(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRONOUN(), "search", args))(&__args) }))) {
    return Py::None;
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _last_party(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_CARE_REPLIES(), g_OFFICE_REPLIES()]), &pool.clone())); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_OTHERS()) } }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ABOUT_SOMEONE_ELSE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&pool.clone(), &g_OFFICE_REPLIES())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GOOD_SAID(), "search", args))(&__args) })) } } } }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TIME(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CARER_ON_THE_DAY_REPLIES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CARER_REPLIES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&pool.clone(), &g_CARE_REPLIES()))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TIME(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CARE_REPLIES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PLAN(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CARE_PLAN_REPLIES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TREATMENT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_CHANGE(), "search", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CARE_TREATMENT_REPLIES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&pool.clone(), &g_CARE_REPLIES())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SPAN(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WINDOW(), "search", args))(&__args) })) } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SPAN_REPLIES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&pool.clone(), &g_CARE_REPLIES())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TIME(), "search", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WAITING_REPLIES()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.the_verdict".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_SOMEONE_ELSE" => Some(g_ABOUT_SOMEONE_ELSE()),
            "ALL_CLEAR" => Some(g_ALL_CLEAR()),
            "ASKED" => Some(g_ASKED()),
            "A_CHANGE" => Some(g_A_CHANGE()),
            "A_GOOD_SAID" => Some(g_A_GOOD_SAID()),
            "A_PLAN" => Some(g_A_PLAN()),
            "A_SPAN" => Some(g_A_SPAN()),
            "A_TIME" => Some(g_A_TIME()),
            "A_TREATMENT" => Some(g_A_TREATMENT()),
            "A_WINDOW" => Some(g_A_WINDOW()),
            "CARE" => Some(g_CARE()),
            "CARER_ON_THE_DAY_REPLIES" => Some(g_CARER_ON_THE_DAY_REPLIES()),
            "CARER_REPLIES" => Some(g_CARER_REPLIES()),
            "CARE_PLAN_REPLIES" => Some(g_CARE_PLAN_REPLIES()),
            "CARE_REPLIES" => Some(g_CARE_REPLIES()),
            "CARE_TREATMENT_REPLIES" => Some(g_CARE_TREATMENT_REPLIES()),
            "CLEARED" => Some(g_CLEARED()),
            "DONE" => Some(g_DONE()),
            "EMERGENCY" => Some(g_EMERGENCY()),
            "GOOD_NEWS" => Some(g_GOOD_NEWS()),
            "MY_MAN" => Some(g_MY_MAN()),
            "OFFICE" => Some(g_OFFICE()),
            "OFFICE_REPLIES" => Some(g_OFFICE_REPLIES()),
            "OTHERS" => Some(g_OTHERS()),
            "POOLS" => Some(g_POOLS()),
            "PRAISE" => Some(g_PRAISE()),
            "PRONOUN" => Some(g_PRONOUN()),
            "SAID" => Some(g_SAID()),
            "SELF_SAID" => Some(g_SELF_SAID()),
            "SPAN_REPLIES" => Some(g_SPAN_REPLIES()),
            "TRADE" => Some(g_TRADE()),
            "TRADE_REPLIES" => Some(g_TRADE_REPLIES()),
            "WAITING_REPLIES" => Some(g_WAITING_REPLIES()),
            "YOU_SAID" => Some(g_YOU_SAID()),
            "_CLEAR_WORDS" => Some(g__CLEAR_WORDS()),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_last_party" => Some(py_func("_last_party", |args: &[Py]| _last_party(args))),
            "_recent_bot_lines" => Some(py_func("_recent_bot_lines", |args: &[Py]| _recent_bot_lines(args))),
            "_the_pool" => Some(py_func("_the_pool", |args: &[Py]| _the_pool(args))),
            "_the_subject" => Some(py_func("_the_subject", |args: &[Py]| _the_subject(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_SOMEONE_ELSE" => s_ABOUT_SOMEONE_ELSE(v),
            "ALL_CLEAR" => s_ALL_CLEAR(v),
            "ASKED" => s_ASKED(v),
            "A_CHANGE" => s_A_CHANGE(v),
            "A_GOOD_SAID" => s_A_GOOD_SAID(v),
            "A_PLAN" => s_A_PLAN(v),
            "A_SPAN" => s_A_SPAN(v),
            "A_TIME" => s_A_TIME(v),
            "A_TREATMENT" => s_A_TREATMENT(v),
            "A_WINDOW" => s_A_WINDOW(v),
            "CARE" => s_CARE(v),
            "CARER_ON_THE_DAY_REPLIES" => s_CARER_ON_THE_DAY_REPLIES(v),
            "CARER_REPLIES" => s_CARER_REPLIES(v),
            "CARE_PLAN_REPLIES" => s_CARE_PLAN_REPLIES(v),
            "CARE_REPLIES" => s_CARE_REPLIES(v),
            "CARE_TREATMENT_REPLIES" => s_CARE_TREATMENT_REPLIES(v),
            "CLEARED" => s_CLEARED(v),
            "DONE" => s_DONE(v),
            "EMERGENCY" => s_EMERGENCY(v),
            "GOOD_NEWS" => s_GOOD_NEWS(v),
            "MY_MAN" => s_MY_MAN(v),
            "OFFICE" => s_OFFICE(v),
            "OFFICE_REPLIES" => s_OFFICE_REPLIES(v),
            "OTHERS" => s_OTHERS(v),
            "POOLS" => s_POOLS(v),
            "PRAISE" => s_PRAISE(v),
            "PRONOUN" => s_PRONOUN(v),
            "SAID" => s_SAID(v),
            "SELF_SAID" => s_SELF_SAID(v),
            "SPAN_REPLIES" => s_SPAN_REPLIES(v),
            "TRADE" => s_TRADE(v),
            "TRADE_REPLIES" => s_TRADE_REPLIES(v),
            "WAITING_REPLIES" => s_WAITING_REPLIES(v),
            "YOU_SAID" => s_YOU_SAID(v),
            "_CLEAR_WORDS" => s__CLEAR_WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.the_verdict", module_obj);
    pyrt::xrefs::reg_fn("memory.the_verdict", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.the_verdict", "_the_subject", py_func("_the_subject", |args: &[Py]| _the_subject(args)));
    pyrt::xrefs::reg_fn("memory.the_verdict", "_the_pool", py_func("_the_pool", |args: &[Py]| _the_pool(args)));
    pyrt::xrefs::reg_fn("memory.the_verdict", "_last_party", py_func("_last_party", |args: &[Py]| _last_party(args)));
    pyrt::xrefs::reg_fn("memory.the_verdict", "_recent_bot_lines", py_func("_recent_bot_lines", |args: &[Py]| _recent_bot_lines(args)));
    pyrt::xrefs::reg_fn("memory.the_verdict", "_choose", py_func("_choose", |args: &[Py]| _choose(args)));
    pyrt::xrefs::reg_fn("memory.the_verdict", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("memory.the_verdict", "ABOUT_SOMEONE_ELSE", g_ABOUT_SOMEONE_ELSE, s_ABOUT_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("memory.the_verdict", "ALL_CLEAR", g_ALL_CLEAR, s_ALL_CLEAR);
    pyrt::xrefs::reg_access("memory.the_verdict", "ASKED", g_ASKED, s_ASKED);
    pyrt::xrefs::reg_access("memory.the_verdict", "A_CHANGE", g_A_CHANGE, s_A_CHANGE);
    pyrt::xrefs::reg_access("memory.the_verdict", "A_GOOD_SAID", g_A_GOOD_SAID, s_A_GOOD_SAID);
    pyrt::xrefs::reg_access("memory.the_verdict", "A_PLAN", g_A_PLAN, s_A_PLAN);
    pyrt::xrefs::reg_access("memory.the_verdict", "A_SPAN", g_A_SPAN, s_A_SPAN);
    pyrt::xrefs::reg_access("memory.the_verdict", "A_TIME", g_A_TIME, s_A_TIME);
    pyrt::xrefs::reg_access("memory.the_verdict", "A_TREATMENT", g_A_TREATMENT, s_A_TREATMENT);
    pyrt::xrefs::reg_access("memory.the_verdict", "A_WINDOW", g_A_WINDOW, s_A_WINDOW);
    pyrt::xrefs::reg_access("memory.the_verdict", "CARE", g_CARE, s_CARE);
    pyrt::xrefs::reg_access("memory.the_verdict", "CARER_ON_THE_DAY_REPLIES", g_CARER_ON_THE_DAY_REPLIES, s_CARER_ON_THE_DAY_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "CARER_REPLIES", g_CARER_REPLIES, s_CARER_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "CARE_PLAN_REPLIES", g_CARE_PLAN_REPLIES, s_CARE_PLAN_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "CARE_REPLIES", g_CARE_REPLIES, s_CARE_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "CARE_TREATMENT_REPLIES", g_CARE_TREATMENT_REPLIES, s_CARE_TREATMENT_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "CLEARED", g_CLEARED, s_CLEARED);
    pyrt::xrefs::reg_access("memory.the_verdict", "DONE", g_DONE, s_DONE);
    pyrt::xrefs::reg_access("memory.the_verdict", "EMERGENCY", g_EMERGENCY, s_EMERGENCY);
    pyrt::xrefs::reg_access("memory.the_verdict", "GOOD_NEWS", g_GOOD_NEWS, s_GOOD_NEWS);
    pyrt::xrefs::reg_access("memory.the_verdict", "MY_MAN", g_MY_MAN, s_MY_MAN);
    pyrt::xrefs::reg_access("memory.the_verdict", "OFFICE", g_OFFICE, s_OFFICE);
    pyrt::xrefs::reg_access("memory.the_verdict", "OFFICE_REPLIES", g_OFFICE_REPLIES, s_OFFICE_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "OTHERS", g_OTHERS, s_OTHERS);
    pyrt::xrefs::reg_access("memory.the_verdict", "POOLS", g_POOLS, s_POOLS);
    pyrt::xrefs::reg_access("memory.the_verdict", "PRAISE", g_PRAISE, s_PRAISE);
    pyrt::xrefs::reg_access("memory.the_verdict", "PRONOUN", g_PRONOUN, s_PRONOUN);
    pyrt::xrefs::reg_access("memory.the_verdict", "SAID", g_SAID, s_SAID);
    pyrt::xrefs::reg_access("memory.the_verdict", "SELF_SAID", g_SELF_SAID, s_SELF_SAID);
    pyrt::xrefs::reg_access("memory.the_verdict", "SPAN_REPLIES", g_SPAN_REPLIES, s_SPAN_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "TRADE", g_TRADE, s_TRADE);
    pyrt::xrefs::reg_access("memory.the_verdict", "TRADE_REPLIES", g_TRADE_REPLIES, s_TRADE_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "WAITING_REPLIES", g_WAITING_REPLIES, s_WAITING_REPLIES);
    pyrt::xrefs::reg_access("memory.the_verdict", "YOU_SAID", g_YOU_SAID, s_YOU_SAID);
    pyrt::xrefs::reg_access("memory.the_verdict", "_CLEAR_WORDS", g__CLEAR_WORDS, s__CLEAR_WORDS);
}
