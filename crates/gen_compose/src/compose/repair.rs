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


thread_local! { static G_AMENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_FOR_THE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_WHAT_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEEDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEEDS_WITH_A_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAPSES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOBODY_TO_REPAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_OR_PROFESSIONAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PHYSICAL_HARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FEELING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FEELING_NAMED_DATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEELING_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRIEF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LAPSE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REGRET_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_THING_THEY_WERE_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AMENDS() -> Py { ensure_init(); G_AMENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMENDS(v: Py) { ensure_init(); G_AMENDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_FOR_THE_WORDS() -> Py { ensure_init(); G_ASKS_FOR_THE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_THE_WORDS(v: Py) { ensure_init(); G_ASKS_FOR_THE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_WHAT_TO_DO() -> Py { ensure_init(); G_ASKS_WHAT_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHAT_TO_DO(v: Py) { ensure_init(); G_ASKS_WHAT_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_DEEDS() -> Py { ensure_init(); G_DEEDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEEDS(v: Py) { ensure_init(); G_DEEDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DEEDS_WITH_A_CARD() -> Py { ensure_init(); G_DEEDS_WITH_A_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEEDS_WITH_A_CARD(v: Py) { ensure_init(); G_DEEDS_WITH_A_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_LAPSES() -> Py { ensure_init(); G_LAPSES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAPSES(v: Py) { ensure_init(); G_LAPSES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOBODY_TO_REPAIR() -> Py { ensure_init(); G_NOBODY_TO_REPAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOBODY_TO_REPAIR(v: Py) { ensure_init(); G_NOBODY_TO_REPAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DEED() -> Py { ensure_init(); G_NOT_A_DEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DEED(v: Py) { ensure_init(); G_NOT_A_DEED.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE() -> Py { ensure_init(); G_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE(v: Py) { ensure_init(); G_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_OR_PROFESSIONAL() -> Py { ensure_init(); G_PET_OR_PROFESSIONAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_OR_PROFESSIONAL(v: Py) { ensure_init(); G_PET_OR_PROFESSIONAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PHYSICAL_HARM() -> Py { ensure_init(); G_PHYSICAL_HARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PHYSICAL_HARM(v: Py) { ensure_init(); G_PHYSICAL_HARM.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FEELING_NAMED() -> Py { ensure_init(); G_THE_FEELING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FEELING_NAMED(v: Py) { ensure_init(); G_THE_FEELING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FEELING_NAMED_DATED() -> Py { ensure_init(); G_THE_FEELING_NAMED_DATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FEELING_NAMED_DATED(v: Py) { ensure_init(); G_THE_FEELING_NAMED_DATED.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO() -> Py { ensure_init(); G_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO(v: Py) { ensure_init(); G_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g__AMENDS() -> Py { ensure_init(); G__AMENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMENDS(v: Py) { ensure_init(); G__AMENDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BABY() -> Py { ensure_init(); G__A_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BABY(v: Py) { ensure_init(); G__A_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g__BETWEEN() -> Py { ensure_init(); G__BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BETWEEN(v: Py) { ensure_init(); G__BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g__FEELING_FIRST() -> Py { ensure_init(); G__FEELING_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEELING_FIRST(v: Py) { ensure_init(); G__FEELING_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g__GRIEF() -> Py { ensure_init(); G__GRIEF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRIEF(v: Py) { ensure_init(); G__GRIEF.with(|c| *c.borrow_mut() = v); }
pub fn g__LAPSE_VERBS() -> Py { ensure_init(); G__LAPSE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LAPSE_VERBS(v: Py) { ensure_init(); G__LAPSE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__REGRET_FIRST() -> Py { ensure_init(); G__REGRET_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REGRET_FIRST(v: Py) { ensure_init(); G__REGRET_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_THING_THEY_WERE_IN() -> Py { ensure_init(); G__THE_THING_THEY_WERE_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_THING_THEY_WERE_IN(v: Py) { ensure_init(); G__THE_THING_THEY_WERE_IN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The reading of a line where the person is the one who did it to somebody.\n\nMost lines that name a person are about that person: \"my mum is upset with me\" is a question\nabout the mum, \"my wife and i argue about money\" one about the money, \"my mums birthday\" one\nabout a party. A line where the person reports a deed of their own is the opposite of those. The\nperson named is who it was done *to*, and what the line wants is what to do about it now.\n\n    i let my mum down, what do i do             the friend, on how to put it right\n    i upset my dad what should i do             not \"you seem a bit off with me - have I done\n                                                something?\", which asks the dad's question\n    i forgot my mums birthday, what do i do     not a party to plan: the day has gone\n    i lied to my wife about the money           not how to split the bills\n\nSo the reading has one job: say whether the deed in the line is the person's own. It is asked by\nthe two parts that would otherwise answer the person named - the composer's repair pool\n(`bot/modules/chat.py`, \"something they said that they wish they had not said\") and the rows\nwhose key names a person with the other direction in mind (`guard=` in the knowledge base).\n\nEvery shape here is a deed people do to somebody they live with, followed by a person. The verb\nis what decides it, never a word like \"upset\" on its own: \"my mum is upset\" has no deed in it,\n\"i upset my mum\" has one, and \"i upset my stomach\" has a body part rather than a person. What a\ndeed is followed by is a person in the possessive (\"my wife\", \"his brother\") or a pronoun\n(\"i let her down\", \"i lied to them\") - a plain \"the waiter\" or \"the dog\" is not the shape this\nreading is for, and the rows that answer them keep those lines.\n");
    s_PEOPLE(py_str("(?:mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|partner|husband|wife|boyfriend|girlfriend|fiance|fiancee|ex|sister|brother|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|uncle|cousin|stepmum|stepdad|stepson|stepdaughter|mother in law|father in law|in laws|friend|friends|mate|best friend|bestie|flatmate|housemate|neighbour|neighbor|boss|manager|colleague|coworker|team|staff)"));
    s_WHO(op_modulo(&py_str("(?:(?:my|his|her|their|our|the)\\s+%s|him|her|them|someone|somebody)"), &g_PEOPLE()));
    s__BETWEEN(py_str("(?:(?:have|had|ve|d|just|really|actually|totally|completely|already|also|then|kind of|sort of|so|still|again|keep|keeps|kept|always|often|usually|sometimes|constantly|managed to|went and)\\s+){0,4}"));
    s__THE_THING_THEY_WERE_IN(op_modulo(&py_str("(?:%s)?(?:play|concert|recital|match|game|show|performance|assembly|sports day|parents evening|parents' evening|nativity|graduation|prize giving|prizegiving|first day|open day|appointment)\\b"), &py_str("(?:school|first|big|little|football|netball|swimming|music|dance|christmas|xmas|summer|end of term|leavers?)\\s+")));
    s_DEEDS(py_tuple(vec![py_tuple(vec![py_str("let down"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("(?:\\bi\\s+%s|\\b(?:about|for|after|of)\\s+%s)(?:let|lets|letting)\\s+%s\\s+down\\b"), &py_tuple(vec![g__BETWEEN(), g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("lied"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:lied|lie|lying|lies)\\b[^.!?]{0,20}?\\bto\\s+%s\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("lied"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:told|tell|have told|ve told)\\s+(?:%s\\s+)?(?:a\\s+)?lie\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("forgot a date"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi\\s+(?:(?:keep|always|completely|totally|just|nearly|almost|went and|went)\\s+){0,3}(?:forgot|forget|forgetting|forgotten)\\b[^.!?]{0,40}?\\b(?:birthday|anniversary)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("forgot a date"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:missed|miss)\\s+(?:%s'?s?\\s+)?(?:birthday|anniversary)\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("lost my temper"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:lost|lose|losing|have lost|ve lost)\\s+my\\s+temper\\b"), &g__BETWEEN())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("upset them"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:upset|hurt|offended|insulted|embarrassed|annoyed|disappointed|angered)\\s+%s\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("upset them"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:hurt|upset|offended|annoyed)\\s+%s'?s?\\s+feelings\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("was unkind to them"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:was|been|am|have been|ve been|were)\\s+(?:a bit |a little |really |very |so |quite |pretty |just |being |really being |so being )*(?:rude|horrible|mean|cruel|unkind|nasty|sharp|short|unfair|dismissive|snappy|off|offhand|harsh|awful|dreadful|terrible|disrespectful|cheeky|abrupt)\\s+(?:to|with|towards|toward)\\s+%s\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("shouted at them"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:shouted|yelled|screamed|snapped|swore|sworn)\\s+(?:at|with)\\s+%s\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("said the wrong thing"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+(?:%s)?(?:said|say|saying|told|tell|telling)\\s+(?:the wrong thing|the wrong words|something\\s+(?:stupid|awful|daft|silly|dumb|mean|horrible|wrong|embarrassing|terrible|harsh|unkind))\\b"), &g__BETWEEN())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("said the wrong thing"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+(?:%s)?(?:said|told)\\b[^.!?]{0,40}\\b(?:was|were|is) (?:awful|horrible|stupid|mean|rude|unkind|dreadful|terrible|nasty|rubbish|disgusting|pathetic)\\b"), &g__BETWEEN())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("missed their thing"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:missed|miss)\\s+(?:(?:%s)'?s?\\s+|(?:my|his|her|their|our)\\s+)%s"), &py_tuple(vec![g__BETWEEN(), g_WHO(), g__THE_THING_THEY_WERE_IN()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("broke a promise"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:broke|broken|break|breaks|have broken|ve broken)\\s+(?:a\\s+|my\\s+|the\\s+)?promise\\b"), &g__BETWEEN())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("cancelled on them"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:stood|stand|standing)\\s+%s\\s+up\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("cancelled on them"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:cancelled|canceled|cancel|cancelling|canceling)\\s+(?:on\\s+)?%s\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("ignored them"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:ignored|ignore|ignoring|blanked|blank)\\s+%s\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("ignored them"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:ignored|ignore|ignoring|blanked|blank)\\s+%s'?s?\\s+(?:calls?|texts?|messages?|emails?|letters?|invites?)\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("made them cry"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:made|make|making|have made|ve made)\\s+%s\\s+(?:cry|cried|crying|sad|miserable|upset|tearful)\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])]));
    s__LAPSE_VERBS(py_str("(?:call(?:ed|ing)?|ring|rang|rung|ringing|text(?:ed|ing)?|messag(?:e|ed|ing)|repl(?:y|ied|ying)|writ(?:e|ing|ten)|visit(?:ed|ing)?|wish(?:ed|ing)?|send|sent|sending)"));
    s_LAPSES(py_tuple(vec![py_tuple(vec![py_str("left it too long"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:forgot|forget|forgetting|have forgotten|ve forgotten)\\s+to\\s+%s\\b[^.!?]{0,30}?%s"), &py_tuple(vec![g__BETWEEN(), g__LAPSE_VERBS(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("left it too long"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:didnt|didn'?t|did not|never|havent|haven'?t|have not|hadnt|hadn'?t|had not|hasnt|hasn'?t|has not)\\s+%s(?:\\s+to)?\\s+%s'?s?\\b"), &py_tuple(vec![g__BETWEEN(), g__LAPSE_VERBS(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("left it too long"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:havent|haven'?t|have not|hadnt|hadn'?t|had not)\\s+(?:seen|saw|visited|been\\s+to\\s+see)\\s+%s\\s+in\\b"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("left it too long"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\bi\\s+%s(?:didnt|didn'?t|did not|never)\\s+(?:get|got|buy|bought|send|sent)\\s+%s\\s+(?:anything|nothing|a\\s+(?:present|gift|card))"), &py_tuple(vec![g__BETWEEN(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])]));
    s_NOT_A_DEED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:let|letting)\\s+(?:my|her|his|their|your)\\s+hair\\s+down\\b|\\bi (?:hurt|upset|pulled|broke|twisted|strained|cut|burnt|burned|dislocated|sprained)\\s+my\\s+(?:back|knee|ankle|shoulder|arm|leg|foot|feet|hand|finger|wrist|neck|head|hip|stomach|toe|ribs?|thumb|elbow|eye|teeth|tooth|ear|throat|chest|spine|hamstring|calves|calf|shin|jaw|skin|appetite|weight|sleep|hair)\\b|\\bi (?:lost|lose|losing)\\s+my\\s+(?:keys|phone|wallet|purse|job|way|mind|trainers|glasses|vape|earbuds?)\\b|\\bi (?:am|m|feel|felt|been)\\s+(?:so |really |very |a bit )?(?:lost|stuck|short of|out of)\\b|\\bi (?:never|don'?t|didnt|didn'?t|hardly ever|rarely)\\s+(?:let|lied|lie|forget|lose|lost|shout|yell|snap|swear|cancel|ignore|blank)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOBODY_TO_REPAIR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:didnt|didn'?t|did not|never)\\s+(?:get|got|win|won|pass|passed|make|made)\\b|\\b(?:got|been) (?:fired|sacked|dumped|rejected|turned down)\\b|\\bthere'?s no fixing\\b|\\bit'?s too late\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PHYSICAL_HARM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:hit|hits|hitting|punch|punched|slap|slapped|push|pushed|shove|shoved|shook|shake|shaking|kick|kicked|throw|threw|thumped|strangle|strangled|bit|biting|dragged|grabbed)\\b|\\b(?:hurt|hurting|injured|burned|burnt|bruised)\\s+(?:my|her|his|their|our)\\s+(?:baby|child|kid|kids|son|daughter|wife|husband|partner|girlfriend|boyfriend|mum|mom|dad|mother|father|sister|brother)\\b(?!'?s?\\s+feelings)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PET_OR_PROFESSIONAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:to|with|at)\\s+(?:my|our|the|a)\\s+(?:dog|cat|puppy|kitten|horse|pony|rabbit|hamster|guinea pig|budgie|parrot|pet|fish|bird|chickens?|sheep|cow|goat|lamb|doctor|gp|nurse|consultant|surgeon|midwife|health visitor|dentist|orthodontist|physio(?:therapist)?|pharmacist|optician|vet|solicitor|accountant|landlord|letting agent|bank)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DEEDS_WITH_A_CARD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_set(vec![py_str("lost my temper"), py_str("shouted at them"), py_str("forgot a date"), py_str("cancelled on them")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__REGRET_FIRST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:honestly|god|tbh|ugh|so|ok|okay)\\s+)*i\\s+(?:just\\s+|really\\s+|deeply\\s+|so\\s+|still\\s+)*regret\\w*\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__FEELING_FIRST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:honestly|god|tbh|ugh|sometimes|now|ok|okay|so)\\s+)*i\\s+(?:just\\s+|really\\s+|still\\s+|properly\\s+|honestly\\s+)*(?:feel|felt|feels)\\b[^.?!]{0,40}?\\b(?:that|about|for|because|when|after|of|at|like)\\b\\s*(?:i|i'?ve|im|i'?m|me|my|not|letting|cancelling|missing|going|being|doing)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_FEELING_NAMED(py_str("Guilt like that is doing its job if it moves on: "));
    s_THE_FEELING_NAMED_DATED(py_str("Guilt about a date does not make it right, so put it right: "));
    s__GRIEF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:died|dies|dying|death|deaths|passed away|passed on|funeral|burial|cremation|grief|grieving|bereav\\w*|widow|widower|widowed|estate|the will|his will|her will)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_BABY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:baby|babies|newborn|newborns|infant|infants)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AMENDS(py_str("\\b(?:make|making)\\s+(?:[\\w']+\\s+){0,6}up\\s+to\\s+%s\\b|\\b(?:make|making|made)\\s+(?:it\\s+)?up\\s+to\\s+%s\\b|\\bmake\\s+(?:it\\s+)?up\\s+with\\s+%s\\b|\\bmake\\s+(?:some\\s+)?amends\\b|\\bput\\s+(?:it|things)\\s+right\\s+(?:with|to)\\s+%s\\b"));
    s_AMENDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&g__AMENDS(), &py_tuple(vec![g_WHO(), g_WHO(), g_WHO(), g_WHO()]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_FOR_THE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:do|should|shall|can|could) i (?:actually |really )?(?:say|tell)\\b|\\bhow (?:do|should) i (?:say it|word it|put it|phrase it)\\b|\\bwhat (?:would|should) i say\\b|\\bwhat do i say to\\b|\\bhow do i tell (?:her|him|them)\\b|\\bi (?:just |really )?dont know what to say\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASKS_WHAT_TO_DO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat (?:do|should|shall|can|could) i do\\b|\\bhow (?:do|can|should) i (?:fix|make|put|smooth|sort)\\b|\\bhow do i (?:fix it|make it right|make up for it)\\b|\\bwhere do i start\\b|\\bwhat now\\b|\\bwhats the next step\\b|\\bany advice\\b|\\bwhat would you do\\b|\\bhow do i say sorry\\b|\\bhow do i apologise\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn deed_of(args: &[Py]) -> Py {
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
    let mut deed: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The deed the person reports doing to somebody, or None.\n\n    `clean` is the line as the understanding engine hands it on, or a reading with \"clean\" in it.\n    A deed with a pet, a doctor or a professional in it is not this reading: a dog does not read\n    an apology and a briefing that went wrong is a mess rather than a wrong. Those keep whatever\n    answer their own row gives them (`SAID_WRONG_TO_AN_ANIMAL`, `SAID_WRONG_TO_A_PROFESSIONAL`).\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    clean = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PET_OR_PROFESSIONAL(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PHYSICAL_HARM(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__GRIEF(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_BABY(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__REGRET_FIRST(), "search", args))(&__args) }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_LAPSES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    deed = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    return deed.clone();
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_A_DEED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOBODY_TO_REPAIR(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    let __iter3 = py_iter_or_raise(&g_DEEDS());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    deed = __unpk4[0].clone();
    pattern = __unpk4[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    return deed.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn owns_the_deed(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line reports something the person did to somebody close. A `guard=` test.");
    return Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| deed_of(args))(&__args) }, &Py::None));
    Py::None
}
pub fn with_the_feeling_named(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut lead: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The deed's card under a sentence that names the feeling the line led with.\n\n    \"i feel sick that i lied to her\" is a confession that says how they feel before it says what\n    they did, and the card alone answers as though nobody had. The name goes in front and the card\n    follows, because the deed still wants its step; a line with the feeling somewhere in the middle\n    (\"i lied and i feel awful\") keeps the card alone.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return reply.clone();
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__FEELING_FIRST(), "search", args))(&__args) }))) {
    return reply.clone();
    }
    lead = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| deed_of(args))(&__args) }, &py_str("forgot a date")))) { g_THE_FEELING_NAMED_DATED() } else { g_THE_FEELING_NAMED() };
    return op_add(&op_add(&lead.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&reply.clone(), &Py::Int(0)), "lower", args))(&__args) }), &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn line_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line as a reading holds it, for a guard that has to read the words themselves.\n\n    A guard is handed the reading, which is a dict for the line as typed and a re-pointed one\n    when the store has put back what an \"it\" stood for. Both carry the words, so a guard about\n    the shape of the line reads `clean` and falls back to `text` or the tokens.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return reading.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text")]));
    for __item in __iter1 {
    key = __item.clone();
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { value.clone() } }) {
    return value.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn amends_to(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line asks how to make amends to somebody, without naming a deed.\n\n    \"how do i make it up to my mum\" has no verb of doing in it, so `deed_of` is right to say\n    there is no deed; what it asks for is the same turn's answer - the specific thing that would\n    put it right. A pet or a professional stands it down for the same reason it stands the deed\n    down, and a part of the body is not somebody to make amends to.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    clean = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_AMENDS(), "search", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PET_OR_PROFESSIONAL(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PHYSICAL_HARM(), "search", args))(&__args) } } }));
    Py::None
}
pub fn a_repair_is_open(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut told: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the turns just said are one repair, with the asks in the middle of it.\n\n    The line that follows two follow-ups still belongs to the repair the conversation opened\n    with: \"i made my mum cry\", \"i dont know what to say\", \"she wont talk to me\". Only the asks\n    that lean on the turn before them are walked through, so a chat that has moved on to a fresh\n    subject is not pulled back into an apology from three turns ago.\n\n    The knowledge base asks this too. \"hes not speaking to me\" the day after an apology has the\n    words of a small child who is late talking, and the row for that child answered it with the\n    speech milestones; the silence here is the repair's, so the row stands down.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    told = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&told.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    said = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| owns_the_deed(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| amends_to(args))(&__args) } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(9)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_FOR_THE_WORDS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKS_WHAT_TO_DO(), "search", args))(&__args) } } })) } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.repair".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AMENDS" => Some(g_AMENDS()),
            "ASKS_FOR_THE_WORDS" => Some(g_ASKS_FOR_THE_WORDS()),
            "ASKS_WHAT_TO_DO" => Some(g_ASKS_WHAT_TO_DO()),
            "DEEDS" => Some(g_DEEDS()),
            "DEEDS_WITH_A_CARD" => Some(g_DEEDS_WITH_A_CARD()),
            "LAPSES" => Some(g_LAPSES()),
            "NOBODY_TO_REPAIR" => Some(g_NOBODY_TO_REPAIR()),
            "NOT_A_DEED" => Some(g_NOT_A_DEED()),
            "PEOPLE" => Some(g_PEOPLE()),
            "PET_OR_PROFESSIONAL" => Some(g_PET_OR_PROFESSIONAL()),
            "PHYSICAL_HARM" => Some(g_PHYSICAL_HARM()),
            "THE_FEELING_NAMED" => Some(g_THE_FEELING_NAMED()),
            "THE_FEELING_NAMED_DATED" => Some(g_THE_FEELING_NAMED_DATED()),
            "WHO" => Some(g_WHO()),
            "_AMENDS" => Some(g__AMENDS()),
            "_A_BABY" => Some(g__A_BABY()),
            "_BETWEEN" => Some(g__BETWEEN()),
            "_FEELING_FIRST" => Some(g__FEELING_FIRST()),
            "_GRIEF" => Some(g__GRIEF()),
            "_LAPSE_VERBS" => Some(g__LAPSE_VERBS()),
            "_REGRET_FIRST" => Some(g__REGRET_FIRST()),
            "_THE_THING_THEY_WERE_IN" => Some(g__THE_THING_THEY_WERE_IN()),
            "a_repair_is_open" => Some(py_func("a_repair_is_open", |args: &[Py]| a_repair_is_open(args))),
            "amends_to" => Some(py_func("amends_to", |args: &[Py]| amends_to(args))),
            "deed_of" => Some(py_func("deed_of", |args: &[Py]| deed_of(args))),
            "line_of" => Some(py_func("line_of", |args: &[Py]| line_of(args))),
            "owns_the_deed" => Some(py_func("owns_the_deed", |args: &[Py]| owns_the_deed(args))),
            "with_the_feeling_named" => Some(py_func("with_the_feeling_named", |args: &[Py]| with_the_feeling_named(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AMENDS" => s_AMENDS(v),
            "ASKS_FOR_THE_WORDS" => s_ASKS_FOR_THE_WORDS(v),
            "ASKS_WHAT_TO_DO" => s_ASKS_WHAT_TO_DO(v),
            "DEEDS" => s_DEEDS(v),
            "DEEDS_WITH_A_CARD" => s_DEEDS_WITH_A_CARD(v),
            "LAPSES" => s_LAPSES(v),
            "NOBODY_TO_REPAIR" => s_NOBODY_TO_REPAIR(v),
            "NOT_A_DEED" => s_NOT_A_DEED(v),
            "PEOPLE" => s_PEOPLE(v),
            "PET_OR_PROFESSIONAL" => s_PET_OR_PROFESSIONAL(v),
            "PHYSICAL_HARM" => s_PHYSICAL_HARM(v),
            "THE_FEELING_NAMED" => s_THE_FEELING_NAMED(v),
            "THE_FEELING_NAMED_DATED" => s_THE_FEELING_NAMED_DATED(v),
            "WHO" => s_WHO(v),
            "_AMENDS" => s__AMENDS(v),
            "_A_BABY" => s__A_BABY(v),
            "_BETWEEN" => s__BETWEEN(v),
            "_FEELING_FIRST" => s__FEELING_FIRST(v),
            "_GRIEF" => s__GRIEF(v),
            "_LAPSE_VERBS" => s__LAPSE_VERBS(v),
            "_REGRET_FIRST" => s__REGRET_FIRST(v),
            "_THE_THING_THEY_WERE_IN" => s__THE_THING_THEY_WERE_IN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.repair", module_obj);
    pyrt::xrefs::reg_fn("compose.repair", "deed_of", py_func("deed_of", |args: &[Py]| deed_of(args)));
    pyrt::xrefs::reg_fn("compose.repair", "owns_the_deed", py_func("owns_the_deed", |args: &[Py]| owns_the_deed(args)));
    pyrt::xrefs::reg_fn("compose.repair", "with_the_feeling_named", py_func("with_the_feeling_named", |args: &[Py]| with_the_feeling_named(args)));
    pyrt::xrefs::reg_fn("compose.repair", "line_of", py_func("line_of", |args: &[Py]| line_of(args)));
    pyrt::xrefs::reg_fn("compose.repair", "amends_to", py_func("amends_to", |args: &[Py]| amends_to(args)));
    pyrt::xrefs::reg_fn("compose.repair", "a_repair_is_open", py_func("a_repair_is_open", |args: &[Py]| a_repair_is_open(args)));
    pyrt::xrefs::reg_access("compose.repair", "AMENDS", g_AMENDS, s_AMENDS);
    pyrt::xrefs::reg_access("compose.repair", "ASKS_FOR_THE_WORDS", g_ASKS_FOR_THE_WORDS, s_ASKS_FOR_THE_WORDS);
    pyrt::xrefs::reg_access("compose.repair", "ASKS_WHAT_TO_DO", g_ASKS_WHAT_TO_DO, s_ASKS_WHAT_TO_DO);
    pyrt::xrefs::reg_access("compose.repair", "DEEDS", g_DEEDS, s_DEEDS);
    pyrt::xrefs::reg_access("compose.repair", "DEEDS_WITH_A_CARD", g_DEEDS_WITH_A_CARD, s_DEEDS_WITH_A_CARD);
    pyrt::xrefs::reg_access("compose.repair", "LAPSES", g_LAPSES, s_LAPSES);
    pyrt::xrefs::reg_access("compose.repair", "NOBODY_TO_REPAIR", g_NOBODY_TO_REPAIR, s_NOBODY_TO_REPAIR);
    pyrt::xrefs::reg_access("compose.repair", "NOT_A_DEED", g_NOT_A_DEED, s_NOT_A_DEED);
    pyrt::xrefs::reg_access("compose.repair", "PEOPLE", g_PEOPLE, s_PEOPLE);
    pyrt::xrefs::reg_access("compose.repair", "PET_OR_PROFESSIONAL", g_PET_OR_PROFESSIONAL, s_PET_OR_PROFESSIONAL);
    pyrt::xrefs::reg_access("compose.repair", "PHYSICAL_HARM", g_PHYSICAL_HARM, s_PHYSICAL_HARM);
    pyrt::xrefs::reg_access("compose.repair", "THE_FEELING_NAMED", g_THE_FEELING_NAMED, s_THE_FEELING_NAMED);
    pyrt::xrefs::reg_access("compose.repair", "THE_FEELING_NAMED_DATED", g_THE_FEELING_NAMED_DATED, s_THE_FEELING_NAMED_DATED);
    pyrt::xrefs::reg_access("compose.repair", "WHO", g_WHO, s_WHO);
    pyrt::xrefs::reg_access("compose.repair", "_AMENDS", g__AMENDS, s__AMENDS);
    pyrt::xrefs::reg_access("compose.repair", "_A_BABY", g__A_BABY, s__A_BABY);
    pyrt::xrefs::reg_access("compose.repair", "_BETWEEN", g__BETWEEN, s__BETWEEN);
    pyrt::xrefs::reg_access("compose.repair", "_FEELING_FIRST", g__FEELING_FIRST, s__FEELING_FIRST);
    pyrt::xrefs::reg_access("compose.repair", "_GRIEF", g__GRIEF, s__GRIEF);
    pyrt::xrefs::reg_access("compose.repair", "_LAPSE_VERBS", g__LAPSE_VERBS, s__LAPSE_VERBS);
    pyrt::xrefs::reg_access("compose.repair", "_REGRET_FIRST", g__REGRET_FIRST, s__REGRET_FIRST);
    pyrt::xrefs::reg_access("compose.repair", "_THE_THING_THEY_WERE_IN", g__THE_THING_THEY_WERE_IN, s__THE_THING_THEY_WERE_IN);
}
