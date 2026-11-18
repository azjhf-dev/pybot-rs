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


thread_local! { static G_A_BARE_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_RATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_AND_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTS_A_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_ASK_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_ASK_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_CORRECTION_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_CORRECTION_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_FILLER_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_FIX_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_IGNORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RATES_TEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_BARE_COUNT() -> Py { ensure_init(); G_A_BARE_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_COUNT(v: Py) { ensure_init(); G_A_BARE_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_RATE() -> Py { ensure_init(); G_A_BARE_RATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_RATE(v: Py) { ensure_init(); G_A_BARE_RATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_AND_PERIOD() -> Py { ensure_init(); G_A_COUNT_AND_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_AND_PERIOD(v: Py) { ensure_init(); G_A_COUNT_AND_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTS_A_PERIOD() -> Py { ensure_init(); G_COUNTS_A_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTS_A_PERIOD(v: Py) { ensure_init(); G_COUNTS_A_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_ADVICE() -> Py { ensure_init(); G_RATES_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_ADVICE(v: Py) { ensure_init(); G_RATES_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_ASK() -> Py { ensure_init(); G_RATES_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_ASK(v: Py) { ensure_init(); G_RATES_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_ASK_FILLER() -> Py { ensure_init(); G_RATES_ASK_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_ASK_FILLER(v: Py) { ensure_init(); G_RATES_ASK_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_ASK_LEAD() -> Py { ensure_init(); G_RATES_ASK_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_ASK_LEAD(v: Py) { ensure_init(); G_RATES_ASK_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_CORRECTION_LEAD() -> Py { ensure_init(); G_RATES_CORRECTION_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_CORRECTION_LEAD(v: Py) { ensure_init(); G_RATES_CORRECTION_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_CORRECTION_TAIL() -> Py { ensure_init(); G_RATES_CORRECTION_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_CORRECTION_TAIL(v: Py) { ensure_init(); G_RATES_CORRECTION_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_END() -> Py { ensure_init(); G_RATES_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_END(v: Py) { ensure_init(); G_RATES_END.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_FILLER_LEAD() -> Py { ensure_init(); G_RATES_FILLER_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_FILLER_LEAD(v: Py) { ensure_init(); G_RATES_FILLER_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_FIX_LEAD() -> Py { ensure_init(); G_RATES_FIX_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_FIX_LEAD(v: Py) { ensure_init(); G_RATES_FIX_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_IGNORE() -> Py { ensure_init(); G_RATES_IGNORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_IGNORE(v: Py) { ensure_init(); G_RATES_IGNORE.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_LEAD() -> Py { ensure_init(); G_RATES_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_LEAD(v: Py) { ensure_init(); G_RATES_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_PRONOUN() -> Py { ensure_init(); G_RATES_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_PRONOUN(v: Py) { ensure_init(); G_RATES_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_SAID() -> Py { ensure_init(); G_RATES_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_SAID(v: Py) { ensure_init(); G_RATES_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_SUBJECT() -> Py { ensure_init(); G_RATES_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_SUBJECT(v: Py) { ensure_init(); G_RATES_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_RATES_TEXT() -> Py { ensure_init(); G_RATES_TEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RATES_TEXT(v: Py) { ensure_init(); G_RATES_TEXT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How often a thing they do happens, said of themselves.\n\n    i go to the gym twice a week\n    how often was that again\n      \"You go to the gym twice a week.\"\n\nThe store kept the hours a person's day runs on (`bot/modules/memory_store.py`, `_the_times`)\nbut nothing about how often they do a thing, so a late \"how often was that again\" reached the\ndetail ask-back and asked them the question they were asking the bot (\"You haven't said - how\noften was that?\"), and \"how often do i go to the gym\" was answered from the exercise entry's\nguideline rather than from the twice they had told it. This is the same shape of fact as a time\nof day with a different unit, so it is filed the same way: against the doing they named, newest\nwinning, and read back in their own words.\n\nOnly a first-person line is filed, so \"my husband goes to the gym twice a week\" is not put on\nthem; a pronoun in the ask never names a doing (\"how often should i feed them\" after \"i water\nthem every day\" is about feeding, which nothing here holds); and a rate asked for as advice\n(\"how often should i water them\") is the knowledge base's, not this reader's.\n");
    s_RATES_TEXT(py_str("(?:once|twice)(?:\\s+(?:a|per|each)\\s+(?:hour|day|week|fortnight|month|year))?|(?:(?:\\d+|two|three|four|five|six|seven|eight)\\s+times?)\\s+(?:a|per|each)\\s+(?:hour|day|week|fortnight|month|year)|every\\s+(?:\\d+|a|an|one|two|three|four|five|six|few)\\s+(?:hours?|minutes?|days?|weeks?|months?|nights?)|every\\s+(?:hour|day|week|morning|evening|night|weekend|other\\s+day|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|most\\s+(?:days|weeks|mornings|evenings)"));
    s_RATES_END(py_str("(?![a-z])"));
    s_RATES_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("\\b(?P<how>"), &g_RATES_TEXT()), &py_str(")")), &g_RATES_END())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<lead>and\\s+|so\\s+|also\\s+|oh\\s+|well\\s+|but\\s+|actually\\s+|no\\s+|sorry\\s+|make it\\s+|i mean\\s+|i meant\\s+|rather\\s+|instead\\s+)*(?P<subj>i|we|i'm|im|i've|ive|we're|were)\\b(?P<rest>.*)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_FIX_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("actually"), py_str("no"), py_str("sorry"), py_str("make"), py_str("i mean"), py_str("i meant"), py_str("rather"), py_str("instead")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_IGNORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i we my our me the a an at on in to for of it that this and but so\njust now only really still usually normally about around im ive were don't dont"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_ASK_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("how what when where why who which whose should shall can could\nwould will do does did is are was were if whether whats hows remind tell"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|so|but|also|oh|well|then|just|now|ok|okay|plus)\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_CORRECTION_LEAD(py_str("(?:actually|no\\s+wait|no|sorry|wait|hang\\s+on|make\\s+it|i\\s+mean|i\\s+meant|rather|instead)"));
    s_RATES_CORRECTION_TAIL(py_str("(?:\\s+(?:now|instead|actually|these\\s+days|not))?\\s*[.!]*\\s*$"));
    s_A_BARE_COUNT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("^\\s*"), &g_RATES_CORRECTION_LEAD()), &py_str("\\b[,\\s:]*(?P<num>\\d+|two|three|four|five|six|seven|eight|nine|ten)\\s+times?(?:\\s+(?:a|per|each)\\s+(?P<period>day|week|fortnight|month|year))?")), &g_RATES_CORRECTION_TAIL())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BARE_RATE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^\\s*"), &g_RATES_CORRECTION_LEAD()), &py_str("\\b[,\\s:]*(?P<how>")), &g_RATES_TEXT()), &py_str(")")), &g_RATES_CORRECTION_TAIL())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_COUNT_AND_PERIOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?P<num>\\d+|two|three|four|five|six|seven|eight|nine|ten)\\s+times?\\s+(?:a|per|each)\\s+(?P<period>day|week|fortnight|month|year)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_FILLER_LEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("and so but also oh well then plus just still only now really\nactually normally usually today tomorrow tonight yesterday"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_ADVICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:should|shall|ought|need(?:s)?|best|recommend\\w*|advice)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("^\\s*(?:and |so |ok |okay |oh |also |then )*(?:remind me\\s+)?(?:(?P<a>how often|how many times)\\b(?P<rest>.*)|(?P<c>how (?:many|much)\\b(?P<rest3>[^?]*))|(?P<b>was|is|were|are)\\s+(?:that|it|this)\\s+(?P<how>"), &g_RATES_TEXT()), &py_str(")")), &g_RATES_END()), &py_str("|what(?:'s|s| is| was)\\s+the\\s+(?:pattern|rate|schedule)\\b(?P<rest2>.*))\\s*[?.!]*\\s*$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COUNTS_A_PERIOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:a|per|each)\\s+(?:hour|day|week|fortnight|month|year|morning|evening|night)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_PRONOUN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("them they it its it's that this these those him her his hers me us\none ones thing stuff"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RATES_ASK_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("do does did was is were i we my our me the a an you go going\nagain then was that it this now still say said tell once twice a week day month year times\nusually normally have has had get gets got take takes make makes use uses need needs\ni'm im've"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _the_rates(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rates")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { held.clone() } else { py_list(vec![]) };
    Py::None
}
pub fn _keep_a_rate_of_a_doing(args: &[Py]) -> Py {
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
    let mut rate: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rate") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rate"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut lead: Py = Py::None;
    let mut w: Py = Py::None;
    let mut what: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("File a rate a line says of a doing that is not put as their own: \"the boiler gets serviced\n    once a year\", \"our 7 month old is waking every 2 hours\".\n\n    A question (\"how often should i water them\") and advice to them (\"you should water them twice\n    a week\") are not things to keep; a line whose rate is the whole of it (\"twice a week\") is an\n    answer to a question the bot asked, not a fact about a doing.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_RATES_ASK_LEAD(), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATES_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATES_ADVICE(), "search", args))(&__args) } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("you"), py_str("your"), py_str("yours"), py_str("themselves")]), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::None;
    }
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATES_SAID(), "sub", args))(&__args) };
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&what.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    lead = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&what.clone(), "split", args))(&__args) }, &Py::None, &Py::Int(1), &Py::None);
    if py_truthy(&{ let __b = lead.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_RATES_FILLER_LEAD(), &geti(&lead.clone(), &Py::Int(0)))) } }) {
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&what.clone(), "split", args))(&__args) }, &Py::Int(1), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&what.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(what.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_RATES_IGNORE(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() })) } }) {
    return Py::None;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(what.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rate.clone(), "group", args))(&__args) }); __args.push(said.clone()); __kws.extend(vec![("own".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keep_a_rate(args))(&__args) };
    Py::None
}
pub fn _keep_a_rate(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut how: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how"))) };
    let mut said: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut own: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { Py::Bool(true) };
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("File one rate they said of a doing, newest winning for that doing.\n\n    `own` is whether the doing is theirs (\"i go to the gym twice a week\") or somebody else's\n    (\"our 7 month old is waking every 2 hours\"); only the first is said back as what they do.\n    ");
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = what.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    how = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!;:-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = how.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&what.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&how.clone())) } }) {
    return Py::None;
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_rates(args))(&__args) };
    turn = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&held.clone());
    let mut __else2 = false;
    for __item in __iter1 {
    item = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }, &what.clone())) } }) {
    let _ = py_setitem(&item.clone(), &py_str("how"), how.clone());
    let _ = py_setitem(&item.clone(), &py_str("said"), said.clone());
    let _ = py_setitem(&item.clone(), &py_str("turn"), turn.clone());
    let _ = py_setitem(&item.clone(), &py_str("own"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(own.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    break;
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("what"), what.clone()), (py_str("how"), how.clone()), (py_str("said"), said.clone()), (py_str("turn"), turn.clone()), (py_str("own"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(own.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "append", args))(&__args) };
    let _ = py_setitem(&memory.clone(), &py_str("rates"), slice_or_raise(&held.clone(), &op_neg(&Py::Int(8)), &Py::None, &Py::None));
    Py::None
}
pub fn _the_newest_recent_rate(args: &[Py]) -> Py {
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
    else { Py::Int(2) };
    let mut item: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The newest rate on file said a turn or two ago, which a bare correction can be about.");
    turn = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_rates(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    item = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&op_le(&op_sub(&turn.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &within.clone())) {
    return item.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_rate_put_right(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut period: Py = Py::None;
    let mut said: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("File a correction of a rate that names no doing of its own, when there is one to correct.\n\n    \"actually 5 times\" and \"no wait, twice a week\" are the line before, put right. The count alone\n    keeps the period the rate was said with, because the person is changing how many and not how\n    often the period runs: \"i walk 3 times a week\" then \"actually 5 times\" is five times a week,\n    not five times ever.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    count = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BARE_COUNT(), "match", args))(&__args) };
    whole = if py_truthy(&count.clone()) { Py::None } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BARE_RATE(), "match", args))(&__args) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&count.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&whole.clone())) } }) {
    return Py::Bool(false);
    }
    item = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_newest_recent_rate(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&item.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&count.clone()) {
    period = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("period")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&count.clone(), "group", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&period.clone()))) {
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_COUNT_AND_PERIOD(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&held.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    period = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("period")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "group", args))(&__args) };
    }
    let _ = py_setitem(&item.clone(), &py_str("how"), op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("num")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&count.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, &py_str(" times a ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&period.clone(), "lower", args))(&__args) }));
    } else {
    let _ = py_setitem(&item.clone(), &py_str("how"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&whole.clone(), "group", args))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) });
    }
    let _ = py_setitem(&item.clone(), &py_str("said"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "lower", args))(&__args) });
    let _ = py_setitem(&item.clone(), &py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let _ = py_setitem(&memory.clone(), &py_str("rates"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_rates(args))(&__args) });
    return Py::Bool(true);
    Py::None
}
pub fn keep(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut held: Py = Py::None;
    let mut inner: Py = Py::None;
    let mut item: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut what: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Keep a rate a line says of a doing: \"i go to the gym twice a week\".\n\n    A rate said of somebody else's doing (\"our 7 month old is waking every 2 hours\") is kept\n    too, filed against the doing as the line put it, because the late ask is the same ask (\"how\n    often was he waking\") and it was being answered with the first line's card again.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_rate_put_right(args))(&__args) }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[.;!?,]|\\band\\b|\\bbut\\b")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    clause = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clause.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    rate = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATES_SAID(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&rate.clone()))) {
    continue;
    }
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATES_SUBJECT(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(rate.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keep_a_rate_of_a_doing(args))(&__args) };
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("i"), py_str("we"), py_str("i'm"), py_str("im"), py_str("i've"), py_str("ive"), py_str("we're"), py_str("were")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subj")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&subject.clone(), "group", args))(&__args) }, "lower", args))(&__args) }))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(rate.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keep_a_rate_of_a_doing(args))(&__args) };
    continue;
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rest")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&subject.clone(), "group", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    inner = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATES_SAID(), "search", args))(&__args) };
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!;:-!")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&if py_truthy(&inner.clone()) { slice_or_raise(&rest.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&inner.clone(), "start", args))(&__args) }, &Py::None) } else { py_str("") }, "strip", args))(&__args) };
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&what.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(word.clone()); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(what.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_RATES_IGNORE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    continue;
    }
    lead = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("lead")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&subject.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&{ let __b = lead.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_RATES_FIX_LEAD(), &geti(&lead.clone(), &Py::Int(0)))) } }) {
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_rates(args))(&__args) };
    turn = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let mut __else3 = false;
    for __item in __iter2 {
    item = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) })) } }) {
    continue;
    }
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    theirs = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&what.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = op_bitand(&mine.clone(), &theirs.clone()); if py_truthy(&__b) { __b } else { op_le(&op_sub(&turn.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(2)) } }) {
    let _ = py_setitem(&item.clone(), &py_str("what"), { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { what.clone() } });
    let _ = py_setitem(&item.clone(), &py_str("how"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rate.clone(), "group", args))(&__args) });
    let _ = py_setitem(&item.clone(), &py_str("said"), said.clone());
    let _ = py_setitem(&item.clone(), &py_str("turn"), turn.clone());
    let _ = py_setitem(&memory.clone(), &py_str("rates"), held.clone());
    break;
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(what.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rate.clone(), "group", args))(&__args) }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keep_a_rate(args))(&__args) };
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(what.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rate.clone(), "group", args))(&__args) }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _keep_a_rate(args))(&__args) };
    }
    Py::None
}
pub fn is_a_rate_ask(args: &[Py]) -> Py {
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
    let _ = py_str("Is this line only the ask for one of the rates kept here?");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| answer(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut held: Py = Py::None;
    let mut held_item: Py = Py::None;
    let mut how: Py = Py::None;
    let mut item: Py = Py::None;
    let mut lean_back: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut what: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The rate of their own doing a late ask names, said back in their own words.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_rates(args))(&__args) }) { let mut item: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) } } }) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return Py::None;
    }
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATES_ASK(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RATES_ADVICE(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("c")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("c")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_COUNTS_A_PERIOD(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rest")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rest3")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rest2")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } } } }, "lower", args))(&__args) };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_RATES_ASK_FILLER(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_RATES_PRONOUN(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } } } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    item = Py::None;
    if py_truthy(&named.clone()) {
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    held_item = __item.clone();
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held_item.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_RATES_PRONOUN(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    if py_truthy(&op_bitand(&named.clone(), &words.clone())) {
    item = held_item.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&item.clone(), &Py::None))) {
    return Py::None;
    }
    } else {
    lean_back = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:that|it|this|again|remind|said|say|told|tell)\\b")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&lean_back.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("c")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "group", args))(&__args) })) } } } }) {
    return Py::None;
    }
    item = geti(&held.clone(), &op_neg(&Py::Int(1)));
    }
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    how = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&what.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&how.clone())) } }) {
    return Py::None;
    }
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bmy\\b")); __args.push(py_str("your")); __args.push(what.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bour\\b")); __args.push(py_str("your")); __args.push(what.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("own")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&item.clone(), "get", args))(&__args) }, &Py::Bool(false)))) {
    return op_add(&op_add(&op_add(&op_add(&py_str("You said "), &what.clone()), &py_str(" ")), &how.clone()), &py_str("."));
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![Py::Int(0), Py::Int(1)])); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, &Py::Int(0)))) {
    said = op_add(&op_add(&op_add(&op_add(&py_str("You "), &what.clone()), &py_str(" ")), &how.clone()), &py_str("."));
    } else {
    said = op_add(&op_add(&op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&how.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&how.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" — you ")), &what.clone()), &py_str("."));
    }
    return said.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.rates".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BARE_COUNT" => Some(g_A_BARE_COUNT()),
            "A_BARE_RATE" => Some(g_A_BARE_RATE()),
            "A_COUNT_AND_PERIOD" => Some(g_A_COUNT_AND_PERIOD()),
            "COUNTS_A_PERIOD" => Some(g_COUNTS_A_PERIOD()),
            "RATES_ADVICE" => Some(g_RATES_ADVICE()),
            "RATES_ASK" => Some(g_RATES_ASK()),
            "RATES_ASK_FILLER" => Some(g_RATES_ASK_FILLER()),
            "RATES_ASK_LEAD" => Some(g_RATES_ASK_LEAD()),
            "RATES_CORRECTION_LEAD" => Some(g_RATES_CORRECTION_LEAD()),
            "RATES_CORRECTION_TAIL" => Some(g_RATES_CORRECTION_TAIL()),
            "RATES_END" => Some(g_RATES_END()),
            "RATES_FILLER_LEAD" => Some(g_RATES_FILLER_LEAD()),
            "RATES_FIX_LEAD" => Some(g_RATES_FIX_LEAD()),
            "RATES_IGNORE" => Some(g_RATES_IGNORE()),
            "RATES_LEAD" => Some(g_RATES_LEAD()),
            "RATES_PRONOUN" => Some(g_RATES_PRONOUN()),
            "RATES_SAID" => Some(g_RATES_SAID()),
            "RATES_SUBJECT" => Some(g_RATES_SUBJECT()),
            "RATES_TEXT" => Some(g_RATES_TEXT()),
            "_keep_a_rate" => Some(py_func("_keep_a_rate", |args: &[Py]| _keep_a_rate(args))),
            "_keep_a_rate_of_a_doing" => Some(py_func("_keep_a_rate_of_a_doing", |args: &[Py]| _keep_a_rate_of_a_doing(args))),
            "_the_newest_recent_rate" => Some(py_func("_the_newest_recent_rate", |args: &[Py]| _the_newest_recent_rate(args))),
            "_the_rate_put_right" => Some(py_func("_the_rate_put_right", |args: &[Py]| _the_rate_put_right(args))),
            "_the_rates" => Some(py_func("_the_rates", |args: &[Py]| _the_rates(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "is_a_rate_ask" => Some(py_func("is_a_rate_ask", |args: &[Py]| is_a_rate_ask(args))),
            "keep" => Some(py_func("keep", |args: &[Py]| keep(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BARE_COUNT" => s_A_BARE_COUNT(v),
            "A_BARE_RATE" => s_A_BARE_RATE(v),
            "A_COUNT_AND_PERIOD" => s_A_COUNT_AND_PERIOD(v),
            "COUNTS_A_PERIOD" => s_COUNTS_A_PERIOD(v),
            "RATES_ADVICE" => s_RATES_ADVICE(v),
            "RATES_ASK" => s_RATES_ASK(v),
            "RATES_ASK_FILLER" => s_RATES_ASK_FILLER(v),
            "RATES_ASK_LEAD" => s_RATES_ASK_LEAD(v),
            "RATES_CORRECTION_LEAD" => s_RATES_CORRECTION_LEAD(v),
            "RATES_CORRECTION_TAIL" => s_RATES_CORRECTION_TAIL(v),
            "RATES_END" => s_RATES_END(v),
            "RATES_FILLER_LEAD" => s_RATES_FILLER_LEAD(v),
            "RATES_FIX_LEAD" => s_RATES_FIX_LEAD(v),
            "RATES_IGNORE" => s_RATES_IGNORE(v),
            "RATES_LEAD" => s_RATES_LEAD(v),
            "RATES_PRONOUN" => s_RATES_PRONOUN(v),
            "RATES_SAID" => s_RATES_SAID(v),
            "RATES_SUBJECT" => s_RATES_SUBJECT(v),
            "RATES_TEXT" => s_RATES_TEXT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.rates", module_obj);
    pyrt::xrefs::reg_fn("memory.rates", "_the_rates", py_func("_the_rates", |args: &[Py]| _the_rates(args)));
    pyrt::xrefs::reg_fn("memory.rates", "_keep_a_rate_of_a_doing", py_func("_keep_a_rate_of_a_doing", |args: &[Py]| _keep_a_rate_of_a_doing(args)));
    pyrt::xrefs::reg_fn("memory.rates", "_keep_a_rate", py_func("_keep_a_rate", |args: &[Py]| _keep_a_rate(args)));
    pyrt::xrefs::reg_fn("memory.rates", "_the_newest_recent_rate", py_func("_the_newest_recent_rate", |args: &[Py]| _the_newest_recent_rate(args)));
    pyrt::xrefs::reg_fn("memory.rates", "_the_rate_put_right", py_func("_the_rate_put_right", |args: &[Py]| _the_rate_put_right(args)));
    pyrt::xrefs::reg_fn("memory.rates", "keep", py_func("keep", |args: &[Py]| keep(args)));
    pyrt::xrefs::reg_fn("memory.rates", "is_a_rate_ask", py_func("is_a_rate_ask", |args: &[Py]| is_a_rate_ask(args)));
    pyrt::xrefs::reg_fn("memory.rates", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_access("memory.rates", "A_BARE_COUNT", g_A_BARE_COUNT, s_A_BARE_COUNT);
    pyrt::xrefs::reg_access("memory.rates", "A_BARE_RATE", g_A_BARE_RATE, s_A_BARE_RATE);
    pyrt::xrefs::reg_access("memory.rates", "A_COUNT_AND_PERIOD", g_A_COUNT_AND_PERIOD, s_A_COUNT_AND_PERIOD);
    pyrt::xrefs::reg_access("memory.rates", "COUNTS_A_PERIOD", g_COUNTS_A_PERIOD, s_COUNTS_A_PERIOD);
    pyrt::xrefs::reg_access("memory.rates", "RATES_ADVICE", g_RATES_ADVICE, s_RATES_ADVICE);
    pyrt::xrefs::reg_access("memory.rates", "RATES_ASK", g_RATES_ASK, s_RATES_ASK);
    pyrt::xrefs::reg_access("memory.rates", "RATES_ASK_FILLER", g_RATES_ASK_FILLER, s_RATES_ASK_FILLER);
    pyrt::xrefs::reg_access("memory.rates", "RATES_ASK_LEAD", g_RATES_ASK_LEAD, s_RATES_ASK_LEAD);
    pyrt::xrefs::reg_access("memory.rates", "RATES_CORRECTION_LEAD", g_RATES_CORRECTION_LEAD, s_RATES_CORRECTION_LEAD);
    pyrt::xrefs::reg_access("memory.rates", "RATES_CORRECTION_TAIL", g_RATES_CORRECTION_TAIL, s_RATES_CORRECTION_TAIL);
    pyrt::xrefs::reg_access("memory.rates", "RATES_END", g_RATES_END, s_RATES_END);
    pyrt::xrefs::reg_access("memory.rates", "RATES_FILLER_LEAD", g_RATES_FILLER_LEAD, s_RATES_FILLER_LEAD);
    pyrt::xrefs::reg_access("memory.rates", "RATES_FIX_LEAD", g_RATES_FIX_LEAD, s_RATES_FIX_LEAD);
    pyrt::xrefs::reg_access("memory.rates", "RATES_IGNORE", g_RATES_IGNORE, s_RATES_IGNORE);
    pyrt::xrefs::reg_access("memory.rates", "RATES_LEAD", g_RATES_LEAD, s_RATES_LEAD);
    pyrt::xrefs::reg_access("memory.rates", "RATES_PRONOUN", g_RATES_PRONOUN, s_RATES_PRONOUN);
    pyrt::xrefs::reg_access("memory.rates", "RATES_SAID", g_RATES_SAID, s_RATES_SAID);
    pyrt::xrefs::reg_access("memory.rates", "RATES_SUBJECT", g_RATES_SUBJECT, s_RATES_SUBJECT);
    pyrt::xrefs::reg_access("memory.rates", "RATES_TEXT", g_RATES_TEXT, s_RATES_TEXT);
}
