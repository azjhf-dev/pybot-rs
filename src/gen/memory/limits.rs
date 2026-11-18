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


pub static CHAIR_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STAIRS_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WALK_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_TRIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CANNOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DISLIKES_THE_WALK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FLAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FLAT_IS_A_HOME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ILLNESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _IN_A_CHAIR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LIMIT_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NO_WALKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PUSHCHAIR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _REFUSES_AID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RULE_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RULE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STAIRS_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WALK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_CHAIR_REPLIES() -> Py { ensure_init(); CHAIR_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHAIR_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = CHAIR_REPLIES.write() { *w = v; } }
pub fn g_STAIRS_REPLIES() -> Py { ensure_init(); STAIRS_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STAIRS_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = STAIRS_REPLIES.write() { *w = v; } }
pub fn g_WALK_REPLIES() -> Py { ensure_init(); WALK_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WALK_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = WALK_REPLIES.write() { *w = v; } }
pub fn g__AID() -> Py { ensure_init(); _AID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AID(v: Py) { ensure_init(); if let Ok(mut w) = _AID.write() { *w = v; } }
pub fn g__ASKS() -> Py { ensure_init(); _ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASKS(v: Py) { ensure_init(); if let Ok(mut w) = _ASKS.write() { *w = v; } }
pub fn g__A_TRIP() -> Py { ensure_init(); _A_TRIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_TRIP(v: Py) { ensure_init(); if let Ok(mut w) = _A_TRIP.write() { *w = v; } }
pub fn g__CANNOT() -> Py { ensure_init(); _CANNOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CANNOT(v: Py) { ensure_init(); if let Ok(mut w) = _CANNOT.write() { *w = v; } }
pub fn g__DISLIKES_THE_WALK() -> Py { ensure_init(); _DISLIKES_THE_WALK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DISLIKES_THE_WALK(v: Py) { ensure_init(); if let Ok(mut w) = _DISLIKES_THE_WALK.write() { *w = v; } }
pub fn g__FLAT() -> Py { ensure_init(); _FLAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FLAT(v: Py) { ensure_init(); if let Ok(mut w) = _FLAT.write() { *w = v; } }
pub fn g__FLAT_IS_A_HOME() -> Py { ensure_init(); _FLAT_IS_A_HOME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FLAT_IS_A_HOME(v: Py) { ensure_init(); if let Ok(mut w) = _FLAT_IS_A_HOME.write() { *w = v; } }
pub fn g__ILLNESS() -> Py { ensure_init(); _ILLNESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ILLNESS(v: Py) { ensure_init(); if let Ok(mut w) = _ILLNESS.write() { *w = v; } }
pub fn g__IN_A_CHAIR() -> Py { ensure_init(); _IN_A_CHAIR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__IN_A_CHAIR(v: Py) { ensure_init(); if let Ok(mut w) = _IN_A_CHAIR.write() { *w = v; } }
pub fn g__LIMIT_VERB() -> Py { ensure_init(); _LIMIT_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LIMIT_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _LIMIT_VERB.write() { *w = v; } }
pub fn g__NO_WALKS() -> Py { ensure_init(); _NO_WALKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NO_WALKS(v: Py) { ensure_init(); if let Ok(mut w) = _NO_WALKS.write() { *w = v; } }
pub fn g__PERSON() -> Py { ensure_init(); _PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PERSON(v: Py) { ensure_init(); if let Ok(mut w) = _PERSON.write() { *w = v; } }
pub fn g__PET() -> Py { ensure_init(); _PET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PET(v: Py) { ensure_init(); if let Ok(mut w) = _PET.write() { *w = v; } }
pub fn g__PUSHCHAIR() -> Py { ensure_init(); _PUSHCHAIR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PUSHCHAIR(v: Py) { ensure_init(); if let Ok(mut w) = _PUSHCHAIR.write() { *w = v; } }
pub fn g__REFUSES_AID() -> Py { ensure_init(); _REFUSES_AID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__REFUSES_AID(v: Py) { ensure_init(); if let Ok(mut w) = _REFUSES_AID.write() { *w = v; } }
pub fn g__RULE_LEAD() -> Py { ensure_init(); _RULE_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RULE_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _RULE_LEAD.write() { *w = v; } }
pub fn g__RULE_TAIL() -> Py { ensure_init(); _RULE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RULE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = _RULE_TAIL.write() { *w = v; } }
pub fn g__STAIRS_SAID() -> Py { ensure_init(); _STAIRS_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STAIRS_SAID(v: Py) { ensure_init(); if let Ok(mut w) = _STAIRS_SAID.write() { *w = v; } }
pub fn g__WALK() -> Py { ensure_init(); _WALK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WALK(v: Py) { ensure_init(); if let Ok(mut w) = _WALK.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A limit on what somebody can do, said out loud: a fact about the plan, not a symptom.\n\nPeople bring a trip or an afternoon out with a limit already in it - \"Im taking my mum to Rome in\nApril.\" / \"She cant walk far.\", \"my dad cant walk far so we need somewhere flat\", \"two adults, one\nis in a wheelchair\" - and on main every one of those lines was answered badly, in three different\nways:\n\n    She cant walk far.                     -> How long has that been going on with your mum?\n    Two adults, one is in a wheelchair.    -> Got it - two adults, one is in a wheelchair.\n    my dad cant walk far so we need ...    -> Somewhere flat - fair enough. What's stopped it\n                                              happening so far?\n    she cant walk far                      -> I'm with you. Keep going if you want to.\n\nThe first is the worst of them. A person saying what their mum can manage on a trip is not asking\nfor a health interview about her, and the question (\"how long has that been going on\") reads as if\nthe holiday had not been mentioned. A friend hears a limit and says what it means for the plan: one\nthing a morning, a sit-down between, a taxi for the last stretch, and the step-free way in.\n\nFour things keep this from answering lines it has no business in:\n\n* the shape is a limit in the words people use for one - cannot walk far, cannot manage stairs,\n  cannot stand for long, uses a stick, is in a wheelchair - and nothing else.\n* a line that reports a symptom is not a limit: \"she cant walk far and her leg is swollen\" is for\n  the health answers and safety's first. The same goes for a diagnosis or a treatment, because the\n  question there is about the illness rather than the plan.\n* an animal's limit is its own conversation (\"my dog is 14 and cant walk far\" is a question about\n  the end of a dog's life, not about a step-free entrance).\n* a line that asks something is left to the module that can answer it, so \"can my mum with a stick\n  get round the museum\" keeps its question.\n\nThe limit is also kept in `memory[\"limits\"]`, because it is a fact about the person that holds for\nthe rest of the chat: a plan offered two turns later should know the walk is short.\n");
    s__PERSON(py_str("(?:i|we|my \\w+|mum|mom|mother|dad|father|nan|nana|gran|granny|grandma|grandad|grandpa|partner|husband|wife|oh|other half|daughter|son|sister|brother|aunt|uncle|cousin|friend|mil|fil|she|he|they|her|his|their)"));
    s__CANNOT(py_str("(?:can'?t|cannot|can not|isnt able to|isn'?t able to|not able to|dont|doesn'?t|do not|does not|struggles? (?:to|with)|struggling (?:to|with)|finds? it hard to|find it hard to|has trouble|have trouble|bad (?:on|at|with))"));
    s__WALK(py_str("(?:walk(?:ing)? (?:far|much|long distances|very far|any distance)|manage (?:the )?stairs|do (?:the )?stairs|climb (?:the )?stairs|stairs|stand (?:for long|up for long|for very long|about for long|for more than \\w+ minutes?)|get (?:about|around) (?:much|easily|well)|(?:walk|move) (?:as )?(?:well|easily) as (?:she|he|they) used to)"));
    s__LIMIT_VERB(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&py_str("\\b"), &g__PERSON()), &py_str("\\s+")), &g__CANNOT()), &py_str("\\s+(?:really |even |anymore |any more )?")), &g__WALK()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AID(stdlib_call("re", "compile", &[py_str("\\b(?:uses?|using|needs?|has|got|with|on|in)\\s+(?:a |an |his |her |their |my )?(?:wheelchair|mobility scooter|scooter|walking stick|stick|walking frame|zimmer|rollator|walking aid|walker|crutches)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__IN_A_CHAIR(stdlib_call("re", "compile", &[py_str("\\b(?:is in|are in|in|into) a wheelchair\\b|\\bwheelchair[- ]?(?:user|bound)\\b|\\bon crutches\\b|cant walk at all|can'?t walk at all|non[- ]?weight ?bearing\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__FLAT(stdlib_call("re", "compile", &[py_str("\\bsomewhere flat\\b|\\bflat(?:ter)? (?:ground|walk|walking|paths?|route|town|city|seafront|promenade|terrain|streets?)\\b|\\bneed(?:s)? (?:it|somewhere|the ground) flat\\b|\\bkeep (?:it|to) (?:the )?flat\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NO_WALKS(stdlib_call("re", "compile", &[py_str("\\b(?:no|nothing|not|never|nowt)\\s+(?:(?:that\\s+)?(?:needs?|involves?|involving|means|with)\\s+)?(?:too\\s+|very\\s+|much\\s+|any\\s+|more\\s+)*(?:long\\s+|proper\\s+|big\\s+|far\\s+|steep\\s+|many\\s+)?(?:walks?|walking|hikes?|hiking|hills?|stairs|steps|climbing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__RULE_LEAD(stdlib_call("re", "compile", &[py_str("^(?:(?:and|so|but|then|also|oh|plus|ok|okay|well|just|actually|honestly|now|right)\\b[^a-z]*)*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__RULE_TAIL(stdlib_call("re", "compile", &[py_str("^[\\s,.;:!-]*(?:(?:please|thanks|ta|then|either|though|too|really|honestly|sadly|unfortunately|today|tomorrow|tonight|now|though|any ?more|at all|this time|for (?:me|us)|on (?:this|the) trip|i think|i suppose)\\b[^a-z]*)*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__STAIRS_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:stairs|steps|climbing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__REFUSES_AID(stdlib_call("re", "compile", &[py_str("\\b(?:wont|won'?t|will not|would not|refuses?|refused|refusing|hates?|hated|doesn'?t want|dont want|does not want|do not want)\\b[^.]{0,25}\\b(?:stick|frame|zimmer|rollator|walker|wheelchair|scooter|aid|rail|handle|crutch(?:es)?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DISLIKES_THE_WALK(stdlib_call("re", "compile", &[add(&py_str("\\b(?:hates?|hated|dislikes?|can'?t stand|cannot stand|cant stand|doesn'?t like|don'?t like|do not like|isn'?t keen on|not keen on|would rather not|prefers? not to)"), &py_str("\\b[^.]{0,24}\\b(?:walk(?:ing)?|hikes?|hiking|rambl(?:e|ing)|hills?|steps?|stairs)\\b|\\brather sit\\b|\\bsit in a pub\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PUSHCHAIR(stdlib_call("re", "compile", &[py_str("\\b(?:pushchair|pram|buggy|stroller|car ?seat|carrycot|cot|highchair)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__FLAT_IS_A_HOME(stdlib_call("re", "compile", &[py_str("\\b(?:to live|to rent|to buy|to move to|a flat|flat to|ground floor flat|bungalow)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ILLNESS(stdlib_call("re", "compile", &[py_str("\\b(?:pain|painful|hurts?|hurting|swollen|swelling|fell|fallen|fall|falls|dizzy|numb|weak|unwell|ill|poorly|blood|bleed|bleeding|bruise|broken|fracture|sore|stiff|doctor|gp|nurse|hospital|a&e|ambulance|111|999|since (?:her|his|the)|getting worse|arthritis|dementia|alzheimers|parkinsons|ms|stroke|surgery|operation|replacement|physio|physiotherapy|chemo|cancer|diagnosis|tablets?|medication|prescribed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PET(stdlib_call("re", "compile", &[py_str("\\b(?:dog|dogs|puppy|pup|cat|cats|kitten|pet|pets|rabbit|hamster|guinea|horse|pony|ferret|parrot)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|where|when|why|how|which|who|can|could|do|does|did|is|are|was|were|will|would|should|shall|any|anyone|anybody)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TRIP(stdlib_call("re", "compile", &[py_str("\\b(?:trip|trips|holiday|holidays|city break|weekend|days? away|nights? away|day out|days out|flight|flights|flying|abroad|visit(?:ing)?|sightsee\\w*|tour(?:ing)?|museums?|galleries|castles?|cathedral|hotels?|somewhere to stay|what should we see|where should we go|things to do|days in|nights in|weeks in|a week|a few days|\\d+ (?:days|nights)|taking \\w+(?: \\w+)? (?:to|off)|off to|going to|booked|booking|somewhere to stay|seaside|beach|coast|countryside|shopping|old town|town cent(?:re|er))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WALK_REPLIES(py_tuple(vec![py_str("Then plan for benches. One big thing a morning, a sit-down after it, and a taxi or a bus for the long hops - two sights close together beats four across town."), py_str("Good to know. Keep the days short and stop before the legs give out rather than after, and check the step-free way into the one place you most want to see."), py_str("That shapes the trip more than the sights do: one thing a day, short walks between, and no shame in a taxi for the last stretch."), py_str("Then build the day around a sit-down every hour or so, and pick your two must-sees rather than a list. Most museums have a step-free entrance if you ask before you go."), py_str("Fair enough - short hops, a stop for a coffee in the middle, and somewhere to sit at the end. That is a better trip than a marathon of churches anyway.")]));
    s_STAIRS_REPLIES(py_tuple(vec![py_str("Ask about lifts and step-free entrances before you book anything: most big places have them, and it is usually not the front door."), py_str("Then ring ahead for the ones you care about - a lift to the good floors and a loo you can get to are the two questions worth asking."), py_str("Old towns are the hard part, so keep to the flatter streets and take the bus between the bits at the top of the hill.")]));
    s_CHAIR_REPLIES(py_tuple(vec![py_str("Then ask about the step-free entrance for each place before you go - most big museums have one. Cobbles and the oldest streets are the thing to plan around."), py_str("Ring ahead for the ones that matter: step-free entry, a lift to the good floors, and an accessible loo. Most places will tell you straight if you ask."), py_str("Plan around the ground rather than the sights: smooth pavements, a bus or a taxi between stops, and one thing a day is plenty.")]));
}
pub fn _the_trip_is_on(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut known: Py = Py::None;
    let mut place: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether a trip or a day out is on the table for this limit to shape.\n\n    The line itself can bring one (\"my dad cant walk far so we need somewhere flat\", \"where should\n    we go at half term\"), or the chat already has one - a place, a plan, or a turn that named a\n    trip. The place is the strongest sign, because a person who has said where they are going does\n    not have to say it again when the limit comes up a turn later.\n    ");
    if py_truthy(&mcall(&g__A_TRIP(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g__FLAT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__FLAT_IS_A_HOME(), "search", &[clean.clone()]))) } }) {
    return Py::Bool(true);
    }
    known = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[known.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut place: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[place.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(place.clone()); } } py_list(__out) })() }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_TRIP(), "search", &[call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::Bool(true);
    }
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&_turns(&[memory.clone()]), &neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g__A_TRIP(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _kind(args: &[Py]) -> Py {
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
    let mut aid: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Which limit the line is, or None. The aid comes first: an aid decides a whole day.");
    if py_truthy(&mcall(&g__IN_A_CHAIR(), "search", &[clean.clone()])) {
    return py_str("chair");
    }
    if py_truthy(&mcall(&g__AID(), "search", &[clean.clone()])) {
    aid = mcall(&mcall(&mcall(&g__AID(), "search", &[clean.clone()]), "group", &[Py::Int(0)]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&aid.clone(), &py_str("wheelchair"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&aid.clone(), &py_str("scooter"))) } }) {
    return py_str("chair");
    }
    return py_str("walk");
    }
    if py_truthy(&mcall(&g__LIMIT_VERB(), "search", &[clean.clone()])) {
    said = mcall(&mcall(&mcall(&g__LIMIT_VERB(), "search", &[clean.clone()]), "group", &[Py::Int(0)]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("stairs")))) {
    return py_str("stairs");
    }
    return py_str("walk");
    }
    if py_truthy(&mcall(&g__DISLIKES_THE_WALK(), "search", &[clean.clone()])) {
    return py_str("walk");
    }
    if py_truthy(&mcall(&g__FLAT(), "search", &[clean.clone()])) {
    return py_str("walk");
    }
    lead = mcall(&g__RULE_LEAD(), "match", &[clean.clone()]);
    rule = mcall(&g__NO_WALKS(), "match", &[clean.clone(), if py_truthy(&lead.clone()) { mcall(&lead.clone(), "end", &[]) } else { Py::Int(0) }]);
    if py_truthy(&{ let __b = rule.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__RULE_TAIL(), "match", &[slice_or_raise(&clean.clone(), &mcall(&rule.clone(), "end", &[]), &Py::None, &Py::None)]) } }) {
    return if py_truthy(&mcall(&g__STAIRS_SAID(), "search", &[mcall(&rule.clone(), "group", &[Py::Int(0)])])) { py_str("stairs") } else { py_str("walk") };
    }
    return Py::None;
    Py::None
}
pub fn names_a_limit(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line names something the person cannot do, or None.\n\n    \"she cant walk far\", \"he is in a wheelchair\", \"two adults, one needs the stairs to himself\":\n    the kind of limit, without needing the chat to be about a trip. A line like this is a fact\n    about somebody, and a sentence of the answer already given is not the answer to it.\n    ");
    return _kind(&[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "split", &[])])]);
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut limits: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("A reply that turns a limit into a plan, or None when the line is not one.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(18)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__PET(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__ILLNESS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__REFUSES_AID(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__PUSHCHAIR(), "search", &[clean.clone()]) } } } } } }) {
    return Py::None;
    }
    kind = _kind(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&mcall(&clean.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g__ASKS(), "match", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_trip_is_on(&[clean.clone(), memory.clone()])))) {
    return Py::None;
    }
    kept = py_dict(vec![(py_str("kind"), kind.clone()), (py_str("text"), clean.clone()), (py_str("turn"), call_builtin("len", &[_turns(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)))]);
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    limits = mcall(&memory.clone(), "get", &[py_str("limits")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[limits.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    limits = py_list(vec![]);
    }
    let _ = mcall(&limits.clone(), "append", &[kept.clone()]);
    let _ = py_setitem(&memory.clone(), &py_str("limits"), slice_or_raise(&limits.clone(), &neg(&Py::Int(6)), &Py::None, &Py::None));
    }
    pool = geti(&py_dict(vec![(py_str("walk"), g_WALK_REPLIES()), (py_str("stairs"), g_STAIRS_REPLIES()), (py_str("chair"), g_CHAIR_REPLIES())]), &kind.clone());
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    Py::None
}
pub fn _turns(args: &[Py]) -> Py {
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
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    return if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { turns.clone() } else { py_list(vec![]) };
    Py::None
}
pub fn said(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut limits: Py = Py::None;
    let _ = py_str("The last limit told in this chat, or None - what a plan offered later should know about.");
    limits = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("limits")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[limits.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { limits.clone() } }) {
    last = geti(&limits.clone(), &neg(&Py::Int(1)));
    if py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    return last.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.limits".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CHAIR_REPLIES" => Some(g_CHAIR_REPLIES()),
            "STAIRS_REPLIES" => Some(g_STAIRS_REPLIES()),
            "WALK_REPLIES" => Some(g_WALK_REPLIES()),
            "_AID" => Some(g__AID()),
            "_ASKS" => Some(g__ASKS()),
            "_A_TRIP" => Some(g__A_TRIP()),
            "_CANNOT" => Some(g__CANNOT()),
            "_DISLIKES_THE_WALK" => Some(g__DISLIKES_THE_WALK()),
            "_FLAT" => Some(g__FLAT()),
            "_FLAT_IS_A_HOME" => Some(g__FLAT_IS_A_HOME()),
            "_ILLNESS" => Some(g__ILLNESS()),
            "_IN_A_CHAIR" => Some(g__IN_A_CHAIR()),
            "_LIMIT_VERB" => Some(g__LIMIT_VERB()),
            "_NO_WALKS" => Some(g__NO_WALKS()),
            "_PERSON" => Some(g__PERSON()),
            "_PET" => Some(g__PET()),
            "_PUSHCHAIR" => Some(g__PUSHCHAIR()),
            "_REFUSES_AID" => Some(g__REFUSES_AID()),
            "_RULE_LEAD" => Some(g__RULE_LEAD()),
            "_RULE_TAIL" => Some(g__RULE_TAIL()),
            "_STAIRS_SAID" => Some(g__STAIRS_SAID()),
            "_WALK" => Some(g__WALK()),
            "_kind" => Some(py_func("_kind", |args: &[Py]| _kind(args))),
            "_the_trip_is_on" => Some(py_func("_the_trip_is_on", |args: &[Py]| _the_trip_is_on(args))),
            "_turns" => Some(py_func("_turns", |args: &[Py]| _turns(args))),
            "names_a_limit" => Some(py_func("names_a_limit", |args: &[Py]| names_a_limit(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            "said" => Some(py_func("said", |args: &[Py]| said(args))),
            _ => None,
        }),
        set: None,
    }))
}
