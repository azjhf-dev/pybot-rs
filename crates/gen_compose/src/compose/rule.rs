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


thread_local! { static G_A_COURT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FOLLOW_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GAME_COURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOCAL_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PENALTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAW_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_RULE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERMISSION_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_A_COURT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_A_LOCAL_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNKNOWN_THE_LAW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_OWNS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_COURT_THING() -> Py { ensure_init(); G_A_COURT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COURT_THING(v: Py) { ensure_init(); G_A_COURT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FOLLOW_UP() -> Py { ensure_init(); G_A_FOLLOW_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FOLLOW_UP(v: Py) { ensure_init(); G_A_FOLLOW_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GAME_COURT() -> Py { ensure_init(); G_A_GAME_COURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GAME_COURT(v: Py) { ensure_init(); G_A_GAME_COURT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOCAL_RULE() -> Py { ensure_init(); G_A_LOCAL_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOCAL_RULE(v: Py) { ensure_init(); G_A_LOCAL_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PENALTY() -> Py { ensure_init(); G_A_PENALTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PENALTY(v: Py) { ensure_init(); G_A_PENALTY.with(|c| *c.borrow_mut() = v); }
pub fn g_LAW_ASKED() -> Py { ensure_init(); G_LAW_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAW_ASKED(v: Py) { ensure_init(); G_LAW_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_RULE_ASK() -> Py { ensure_init(); G_NOT_A_RULE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_RULE_ASK(v: Py) { ensure_init(); G_NOT_A_RULE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PERMISSION_ASKED() -> Py { ensure_init(); G_PERMISSION_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERMISSION_ASKED(v: Py) { ensure_init(); G_PERMISSION_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_A_COURT_THING() -> Py { ensure_init(); G_UNKNOWN_A_COURT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_A_COURT_THING(v: Py) { ensure_init(); G_UNKNOWN_A_COURT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_A_LOCAL_RULE() -> Py { ensure_init(); G_UNKNOWN_A_LOCAL_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_A_LOCAL_RULE(v: Py) { ensure_init(); G_UNKNOWN_A_LOCAL_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNKNOWN_THE_LAW() -> Py { ensure_init(); G_UNKNOWN_THE_LAW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNKNOWN_THE_LAW(v: Py) { ensure_init(); G_UNKNOWN_THE_LAW.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_OWNS_IT() -> Py { ensure_init(); G_WHO_OWNS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_OWNS_IT(v: Py) { ensure_init(); G_WHO_OWNS_IT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The rule is not the bot's to state: whose it is, and where it is written.\n\nSTYLE.md draws the line this module sits on. Asked whether something is allowed, the bot must\nnot invent the rule, and it must not shrug either:\n\n    is it against the law to park on the pavement\n    Different councils, different rules - yours is the only one that counts here. It is on their\n    site, and the council themselves will tell you where you stand if you ring.\n\nTwo things make that reply worth more than \"a quick search online will get you a solid answer\":\nit names **who owns the rule** (the council, the court's office, the government) and it does not\nguess at the wording. Nothing in this file states a rule, a penalty or a deadline, and nothing in\nit is a fact about the world that could go out of date - the same reason a live price or a\nsurgery's opening hours are not here.\n\nThree owners are read, and no more:\n\n* a **court thing** - a court date, an ignored letter, a summons, a tribunal. The office named on\n  the letter knows its own dates, and the reference is what gets a person through.\n* a **local rule** - parking, bins, a shed, planning permission, a licence. The council writes it,\n  it changes from one street to the next, and their own site has the exact wording.\n* **the law in general** - cash, recording a call, points on a licence. It is published on gov.uk\n  before it is guessed at, and Citizens Advice reads it against the person's own case.\n\nWhat is *not* here is as deliberate. A body or a medicine stands the whole file down, so\n\"am i allowed to take these tablets with my antibiotics\" stays a pharmacist's question. A court\nyou play on is not a court of law, so \"how long is the tennis court open\" keeps its own answer.\n\nThis is a last resort, and it is asked like one: `bot/modules/greeting.py` asks it only when the\ncomposer's reply is already a hedge, so a card, a module or a line of the composer's own never\nloses a turn to it. What it can lose is a search hedge nobody wanted, and it cannot invent a fact\nto fill the gap.\n");
    s_A_COURT_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bcourt\\b|\\bsummons\\b|\\btribunal\\b|\\bhearing\\s+(?:date|on|for)\\b|\\bmagistrates?\\b|\\bsmall\\s+claims\\b|\\bbailiffs?\\b|\\bsolicitors?\\s+letter\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_GAME_COURT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tennis|squash|badminton|basketball|netball|volleyball|football|padel|pickleball|handball|five-a-side|bowls)\\s+courts?\\b|\\b(?:food|courtyard|shopping|outlet)\\s+courts?\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LOCAL_RULE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:council|local\\s+authority|parking|pavement|kerb|driveway|bins?|bin\\s+day|fly\\s*tipping|noise\\s+nuisance|planning\\s+permission|permission\\s+for|shed|fence|conservatory|loft|extension|licen[cs]e|permit|bus\\s+lane|allotment|dog\\s+warden|hedge|garden|chickens?|hens?|bees?|hive|bonfire|barbeque|barbecue)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LAW_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|isnt|is\\s+not|was|wasnt)\\s+(?:it|that|this)\\s+(?:actually\\s+|really\\s+|even\\s+)?(?:legal|illegal|lawful|unlawful|against\\s+the\\s+law|allowed|a\\s+crime)\\b|\\bam\\s+i\\s+(?:legally\\s+)?allowed\\s+to\\b|\\bdo\\s+i\\s+have\\s+to\\s+(?:by\\s+law|legally)\\b|\\bcan\\s+(?:i|they|he|she|you|we)\\s+be\\s+fined\\b|\\bwill\\s+(?:i|they|he|she|we)\\s+get\\s+(?:done|fined|prosecuted|points|a\\s+fine)\\b|\\bwhats?\\s+the\\s+law\\s+(?:on|about|say)\\b|\\bdoes\\s+the\\s+law\\s+say\\b|\\bwhat\\s+does\\s+the\\s+law\\s+say\\b|\\bis\\s+there\\s+a\\s+law\\s+(?:against|on|about)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_RULE_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tablets?|pills?|painkillers?|paracetamol|ibuprofen|antibiotics?|insulin|medicines?|dose|dosage|prescription|symptoms?|bleeding|pregnant|pregnancy|baby|babies|child|kids?|sick|ill|hurt|pain|doctor|gp|nurse|hospital)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FOLLOW_UP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:will|would|can|could|do|does|did)\\s+(?:they|it|he|she|the\\s+council|the\\s+court)\\b|\\bwhat\\s+(?:happens|happened|now|next)\\b|\\bhow\\s+long\\b|\\bwhat\\s+do\\s+i\\s+do\\b|\\bam\\s+i\\s+(?:in\\s+trouble|stuck|liable)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_UNKNOWN_THE_LAW(py_tuple(vec![py_str("There is an answer in law to that, and it turns on the exact situation rather than the general idea - which is the part I would get wrong. gov.uk has the wording in plain English, and Citizens Advice will read it against your own case for free."), py_str("The general rule is written down, but where it lands depends on your side of it, and that is exactly the bit I would only be guessing at. gov.uk has the wording, and Citizens Advice is free and will apply it to your situation."), py_str("I can tell you where this one is kept rather than what it says: law like this is published in full, and a summary from me could be wrong on a detail that matters. It is on gov.uk in plain words, and Citizens Advice will answer the same question about your own case free of charge.")]));
    s_UNKNOWN_A_LOCAL_RULE(py_tuple(vec![py_str("Rules like this are set by your council rather than nationally, so two roads can answer differently - which is why I would only be guessing at yours. Their own site has the exact wording and any charge, and their phone line settles it in a minute."), py_str("This one is your council's to set and enforce, and it changes from place to place. Their site has it in writing, and Citizens Advice is free if the page is a maze, which it often is."), py_str("The council writes this one, so theirs is the only wording that counts - and it varies by street, so a general answer would be no use to you. It is on their site with the details; ringing them is quicker than reading, and the number is on the page.")]));
    s_UNKNOWN_A_COURT_THING(py_tuple(vec![py_str("The letter is the thing that matters here: it names the date and the office that can answer for it, and that office is quicker than the paperwork looks. Ring it with the reference to hand, sooner rather than later - and Citizens Advice will read the letter with you first, free."), py_str("Whoever sent it knows what happens next and by when, and nothing I say changes a date that is already set. Ring the office named on the letter with the reference to hand; Citizens Advice will go through the wording with you first at no cost."), py_str("A court's next step is on the paperwork rather than in a general answer, and the office that sent it can tell you where you stand. The reference on the letter is what gets you through - and Citizens Advice will read it with you for free if the wording is unclear.")]));
    s_PERMISSION_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:can|could|may)\\s+(?:i|we|you|they|he|she)\\b|\\b(?:do|does|did)\\s+(?:i|we|you|they|he|she)\\s+(?:need|have\\s+to|require|apply)\\b|\\b(?:i|we|they|he|she)\\s+(?:need|needs|have\\s+to|has\\s+to|required|requires)\\b|\\bam\\s+i\\s+(?:allowed|supposed)\\b|\\bis\\s+it\\s+(?:allowed|legal|illegal|against\\s+the\\s+law)\\b|\\bwhat\\s+(?:are|is)\\s+the\\s+rules?\\b|\\b(?:rules?|regulations?|bylaws?|by-laws?|permission|permit|licen[cs]e)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHO_OWNS_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:council|local\\s+authority|town\\s+hall|court|tribunal|planning|enforcement|warden|inspector|officer)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PENALTY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:fines?|fined|penalt(?:y|ies)|prosecut\\w*|summons|points|asbos?|notice|tribunal|evict\\w*|bailiffs?)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _said(args: &[Py]) -> Py {
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
    let _ = py_str("The line, lowercased and with its spacing tidied.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _a_court_of_law(args: &[Py]) -> Py {
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
    let _ = py_str("True when a court word in the line means a court of law rather than a place you play.\n\n    \"how long is the tennis court open\" and \"how far is the tennis court\" are the leisure\n    centre's hours and the walk there. A line that asks about the law as well keeps the court\n    arm, so a tennis court that has brought the council round is still read as both.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_COURT_THING(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GAME_COURT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LAW_ASKED(), "search", args))(&__args) })) } }));
    Py::None
}
pub fn _the_thread_said(args: &[Py]) -> Py {
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
    else { Py::Int(2) };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when one of the last few things the person said matched `pattern`.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_court_of_law_in_the_thread(args: &[Py]) -> Py {
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
    else { Py::Int(2) };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when one of the last few lines named a court of law.\n\n    \"my ex is taking me to court\" then \"what happens if i ignore the letter\": the line says only\n    \"the letter\", and the letter is the court's.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_court_of_law(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _said_a_local_rule_recently(args: &[Py]) -> Py {
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
    else { Py::Int(2) };
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last few lines named something a council sets a rule about.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LOCAL_RULE(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_rule_ask_in_the_thread(args: &[Py]) -> Py {
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
    else { Py::Int(2) };
    let mut earlier: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when one of the last few lines was itself a rule ask.\n\n    \"i put my bin out early\" then \"will they fine me\" is the same rule. \"small back garden with\n    clay soil\" then \"can i do it from seed\" is gardening, and the word that matched is \"garden\".\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    earlier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&earlier.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_RULE_ASK(), "search", args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LAW_ASKED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHO_OWNS_IT(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LOCAL_RULE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PERMISSION_ASKED(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn pool_for(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The pool that names who owns this rule, or None when the line is not about one.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_RULE_ASK(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_court_of_law(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_court_of_law_in_the_thread(args))(&__args) } } }) {
    return g_UNKNOWN_A_COURT_THING();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LAW_ASKED(), "search", args))(&__args) }) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LOCAL_RULE(), "search", args))(&__args) }) {
    return g_UNKNOWN_A_LOCAL_RULE();
    }
    return g_UNKNOWN_THE_LAW();
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FOLLOW_UP(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_a_local_rule_recently(args))(&__args) } } }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PENALTY(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_rule_ask_in_the_thread(args))(&__args) } } }) {
    return g_UNKNOWN_A_LOCAL_RULE();
    }
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
    else { Py::None };
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("One line naming the rule's owner, or None when the line is not a rule ask.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) };
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pool_for(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { memory.clone() } else { py_dict(vec![]) }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.rule".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_COURT_THING" => Some(g_A_COURT_THING()),
            "A_FOLLOW_UP" => Some(g_A_FOLLOW_UP()),
            "A_GAME_COURT" => Some(g_A_GAME_COURT()),
            "A_LOCAL_RULE" => Some(g_A_LOCAL_RULE()),
            "A_PENALTY" => Some(g_A_PENALTY()),
            "LAW_ASKED" => Some(g_LAW_ASKED()),
            "NOT_A_RULE_ASK" => Some(g_NOT_A_RULE_ASK()),
            "PERMISSION_ASKED" => Some(g_PERMISSION_ASKED()),
            "UNKNOWN_A_COURT_THING" => Some(g_UNKNOWN_A_COURT_THING()),
            "UNKNOWN_A_LOCAL_RULE" => Some(g_UNKNOWN_A_LOCAL_RULE()),
            "UNKNOWN_THE_LAW" => Some(g_UNKNOWN_THE_LAW()),
            "WHO_OWNS_IT" => Some(g_WHO_OWNS_IT()),
            "_a_court_of_law" => Some(py_func("_a_court_of_law", |args: &[Py]| _a_court_of_law(args))),
            "_a_court_of_law_in_the_thread" => Some(py_func("_a_court_of_law_in_the_thread", |args: &[Py]| _a_court_of_law_in_the_thread(args))),
            "_a_rule_ask_in_the_thread" => Some(py_func("_a_rule_ask_in_the_thread", |args: &[Py]| _a_rule_ask_in_the_thread(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_said_a_local_rule_recently" => Some(py_func("_said_a_local_rule_recently", |args: &[Py]| _said_a_local_rule_recently(args))),
            "_the_thread_said" => Some(py_func("_the_thread_said", |args: &[Py]| _the_thread_said(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "pool_for" => Some(py_func("pool_for", |args: &[Py]| pool_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_COURT_THING" => s_A_COURT_THING(v),
            "A_FOLLOW_UP" => s_A_FOLLOW_UP(v),
            "A_GAME_COURT" => s_A_GAME_COURT(v),
            "A_LOCAL_RULE" => s_A_LOCAL_RULE(v),
            "A_PENALTY" => s_A_PENALTY(v),
            "LAW_ASKED" => s_LAW_ASKED(v),
            "NOT_A_RULE_ASK" => s_NOT_A_RULE_ASK(v),
            "PERMISSION_ASKED" => s_PERMISSION_ASKED(v),
            "UNKNOWN_A_COURT_THING" => s_UNKNOWN_A_COURT_THING(v),
            "UNKNOWN_A_LOCAL_RULE" => s_UNKNOWN_A_LOCAL_RULE(v),
            "UNKNOWN_THE_LAW" => s_UNKNOWN_THE_LAW(v),
            "WHO_OWNS_IT" => s_WHO_OWNS_IT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.rule", module_obj);
    pyrt::xrefs::reg_fn("compose.rule", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("compose.rule", "_a_court_of_law", py_func("_a_court_of_law", |args: &[Py]| _a_court_of_law(args)));
    pyrt::xrefs::reg_fn("compose.rule", "_the_thread_said", py_func("_the_thread_said", |args: &[Py]| _the_thread_said(args)));
    pyrt::xrefs::reg_fn("compose.rule", "_a_court_of_law_in_the_thread", py_func("_a_court_of_law_in_the_thread", |args: &[Py]| _a_court_of_law_in_the_thread(args)));
    pyrt::xrefs::reg_fn("compose.rule", "_said_a_local_rule_recently", py_func("_said_a_local_rule_recently", |args: &[Py]| _said_a_local_rule_recently(args)));
    pyrt::xrefs::reg_fn("compose.rule", "_a_rule_ask_in_the_thread", py_func("_a_rule_ask_in_the_thread", |args: &[Py]| _a_rule_ask_in_the_thread(args)));
    pyrt::xrefs::reg_fn("compose.rule", "pool_for", py_func("pool_for", |args: &[Py]| pool_for(args)));
    pyrt::xrefs::reg_fn("compose.rule", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.rule", "A_COURT_THING", g_A_COURT_THING, s_A_COURT_THING);
    pyrt::xrefs::reg_access("compose.rule", "A_FOLLOW_UP", g_A_FOLLOW_UP, s_A_FOLLOW_UP);
    pyrt::xrefs::reg_access("compose.rule", "A_GAME_COURT", g_A_GAME_COURT, s_A_GAME_COURT);
    pyrt::xrefs::reg_access("compose.rule", "A_LOCAL_RULE", g_A_LOCAL_RULE, s_A_LOCAL_RULE);
    pyrt::xrefs::reg_access("compose.rule", "A_PENALTY", g_A_PENALTY, s_A_PENALTY);
    pyrt::xrefs::reg_access("compose.rule", "LAW_ASKED", g_LAW_ASKED, s_LAW_ASKED);
    pyrt::xrefs::reg_access("compose.rule", "NOT_A_RULE_ASK", g_NOT_A_RULE_ASK, s_NOT_A_RULE_ASK);
    pyrt::xrefs::reg_access("compose.rule", "PERMISSION_ASKED", g_PERMISSION_ASKED, s_PERMISSION_ASKED);
    pyrt::xrefs::reg_access("compose.rule", "UNKNOWN_A_COURT_THING", g_UNKNOWN_A_COURT_THING, s_UNKNOWN_A_COURT_THING);
    pyrt::xrefs::reg_access("compose.rule", "UNKNOWN_A_LOCAL_RULE", g_UNKNOWN_A_LOCAL_RULE, s_UNKNOWN_A_LOCAL_RULE);
    pyrt::xrefs::reg_access("compose.rule", "UNKNOWN_THE_LAW", g_UNKNOWN_THE_LAW, s_UNKNOWN_THE_LAW);
    pyrt::xrefs::reg_access("compose.rule", "WHO_OWNS_IT", g_WHO_OWNS_IT, s_WHO_OWNS_IT);
}
