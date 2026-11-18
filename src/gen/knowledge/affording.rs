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


pub static ANSWERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PERSON_OR_AN_INVITATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NO_MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PLANNING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_OCCASION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ANSWERS() -> Py { ensure_init(); ANSWERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERS.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g__A_PERSON_OR_AN_INVITATION() -> Py { ensure_init(); _A_PERSON_OR_AN_INVITATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PERSON_OR_AN_INVITATION(v: Py) { ensure_init(); if let Ok(mut w) = _A_PERSON_OR_AN_INVITATION.write() { *w = v; } }
pub fn g__NO_MONEY() -> Py { ensure_init(); _NO_MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NO_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = _NO_MONEY.write() { *w = v; } }
pub fn g__PLANNING() -> Py { ensure_init(); _PLANNING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PLANNING(v: Py) { ensure_init(); if let Ok(mut w) = _PLANNING.write() { *w = v; } }
pub fn g__THE_OCCASION() -> Py { ensure_init(); _THE_OCCASION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_OCCASION(v: Py) { ensure_init(); if let Ok(mut w) = _THE_OCCASION.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Saying no to something you cannot afford: the invitation, not the price.\n\nThe answers lane owns this domain. Everything here is data: the row is keyed by meaning (an\ninvitation that costs more than the person has) and phrased the way a sensible friend would\nanswer, not the way the money or the event happens to be worded.\n\nWhy the row is here rather than in `money` or `family`: those two domains already hold\n`gift-budget`, which keys `friend` beside `afford` because it answers what a present should\ncost. Any line that says \"I cannot afford it\" and names a friend therefore reads as a question\nabout a price, and the answer a person got was \"Roughly £10 to £15 covers a secret santa or a\ncolleague\" - a figure, to somebody who has just said they cannot go. `family`'s\n`friend-cant-afford-wedding` is the right card and answered weddings only. This row is the\nother half of the subject in one place, so a holiday, a stag, a hen, a birthday dinner and a\nshare of a trip all reach the same answer, the price rows keep their own lines, and a wedding\nkeeps `friend-cant-afford-wedding`, which already says it well.\n");
    s_ANSWERS(py_tuple(vec![py_str("friends-and-relationships"), py_str("money"), py_str("family-and-parenting")]));
    s__NO_MONEY(py_tuple(vec![py_str("cant afford"), py_str("can't afford"), py_str("cannot afford"), py_str("couldnt afford"), py_str("couldn't afford"), py_str("cant go"), py_str("can't go"), py_str("cant come"), py_str("can't come"), py_str("too expensive for me"), py_str("too expensive for us"), py_str("thats too expensive"), py_str("that's too expensive"), py_str("expensive things"), py_str("expensive stuff"), py_str("priced out"), py_str("money is tight"), py_str("moneys tight"), py_str("money's tight"), py_str("no money for"), py_str("cant stretch to"), py_str("can't stretch to")]));
    s__THE_OCCASION(py_tuple(vec![py_str("holiday"), py_str("holidays"), py_str("trip"), py_str("trips"), py_str("weekend away"), py_str("long weekend"), py_str("city break"), py_str("stag"), py_str("stag do"), py_str("stags"), py_str("hen"), py_str("hen do"), py_str("hens"), py_str("birthday dinner"), py_str("birthday meal"), py_str("birthday drinks"), py_str("birthday do"), py_str("birthday party"), py_str("dinner"), py_str("meal"), py_str("night out"), py_str("party"), py_str("parties"), py_str("bridesmaid"), py_str("best man"), py_str("godparent"), py_str("to come"), py_str("to go"), py_str("come along"), py_str("be there"), py_str("my share"), py_str("my half"), py_str("the share"), py_str("my ticket"), py_str("the ticket"), py_str("inviting me"), py_str("invited me"), py_str("invites me"), py_str("invitation")]));
    s__A_PERSON_OR_AN_INVITATION(stdlib_call("re", "compile", &[py_str("\\b(?:friend|friends|mate|mates|pal|pals|mum|mam|mom|mother|dad|father|sister|brother|sibling|cousin|colleague|workmate|family|she|he|they|her|him|them|invit(?:e|es|ed|ing|ation|ations)|asking me|asked me|keep asking|to come|to go|come along|be there|my share|my half)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PLANNING(stdlib_call("re", "compile", &[py_str("\\b(?:planning|guest list|how much should|how much do i|budget(?:ing)? for|cheapest|cheap|save for|saving for|my own wedding|our wedding|we are getting married|afford a holiday|afford a trip|afford a wedding)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("an-invitation-you-cannot-afford")); __args.push(py_tuple(vec![g__NO_MONEY(), g__THE_OCCASION()])); __args.push(py_str("Say it early, to the person who asked you, and keep it about the money rather than about them: 'I'd love to be there, and I can't do the whole thing this year.' Offer the part you can do - the meal, the one night, a smaller gift - and name a number if you have one. Don't invent a reason and don't leave it to the week before: one honest no, said kindly, is much easier to hear than a yes you cannot afford.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), g__an_invitation_is_what_they_cannot_afford()), ("avoid".to_string(), py_tuple(vec![py_str("wedding"), py_str("weddings"), py_str("bride"), py_str("groom"), py_str("bridesmaid"), py_str("best man"), py_str("godparent"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("secret santa"), py_str("christmas"), py_str("xmas"), py_str("rent"), py_str("mortgage"), py_str("bill"), py_str("bills"), py_str("energy"), py_str("electric"), py_str("gas"), py_str("council tax"), py_str("insurance"), py_str("car"), py_str("van"), py_str("petrol"), py_str("fuel"), py_str("childcare"), py_str("nursery"), py_str("school trip"), py_str("uniform"), py_str("food"), py_str("shopping"), py_str("dental"), py_str("dentist"), py_str("glasses")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn _an_invitation_is_what_they_cannot_afford(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut one: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the money that is not there was for something somebody asked them to.\n\n    True means this row may answer. A line with no \"cant afford\" in it is untouched, so every\n    other way of saying the money is short keeps whatever row it had. A line with one has to\n    name the person or the invitation, and must not be a line about planning or pricing.\n    ");
    reading = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    words = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    text = { let __b = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { words.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_PERSON_OR_AN_INVITATION(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g__PLANNING(), "search", &[text.clone()])));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.affording".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "_A_PERSON_OR_AN_INVITATION" => Some(g__A_PERSON_OR_AN_INVITATION()),
            "_NO_MONEY" => Some(g__NO_MONEY()),
            "_PLANNING" => Some(g__PLANNING()),
            "_THE_OCCASION" => Some(g__THE_OCCASION()),
            "_an_invitation_is_what_they_cannot_afford" => Some(py_func("_an_invitation_is_what_they_cannot_afford", |args: &[Py]| _an_invitation_is_what_they_cannot_afford(args))),
            _ => None,
        }),
        set: None,
    }))
}
