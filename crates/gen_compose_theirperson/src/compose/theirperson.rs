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


thread_local! { static G_A_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONDITION_STATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONDITION_WONDERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOT_COPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_OF_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ROUGH_PATCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WONDERING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BETWEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STANDS_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.theirperson: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_CONDITION() -> Py { ensure_init(); G_A_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION(v: Py) { ensure_init(); G_A_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONDITION_STATED() -> Py { ensure_init(); G_A_CONDITION_STATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION_STATED(v: Py) { ensure_init(); G_A_CONDITION_STATED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONDITION_WONDERED() -> Py { ensure_init(); G_A_CONDITION_WONDERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION_WONDERED(v: Py) { ensure_init(); G_A_CONDITION_WONDERED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOW() -> Py { ensure_init(); G_A_LOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOW(v: Py) { ensure_init(); G_A_LOW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOT_COPING() -> Py { ensure_init(); G_A_NOT_COPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOT_COPING(v: Py) { ensure_init(); G_A_NOT_COPING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_OF_THEIRS() -> Py { ensure_init(); G_A_PERSON_OF_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_OF_THEIRS(v: Py) { ensure_init(); G_A_PERSON_OF_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ROUGH_PATCH() -> Py { ensure_init(); G_A_ROUGH_PATCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ROUGH_PATCH(v: Py) { ensure_init(); G_A_ROUGH_PATCH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WONDERING() -> Py { ensure_init(); G_A_WONDERING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WONDERING(v: Py) { ensure_init(); G_A_WONDERING.with(|c| *c.borrow_mut() = v); }
pub fn g_BETWEEN() -> Py { ensure_init(); G_BETWEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BETWEEN(v: Py) { ensure_init(); G_BETWEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED() -> Py { ensure_init(); G_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED(v: Py) { ensure_init(); G_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_POOL() -> Py { ensure_init(); G_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOL(v: Py) { ensure_init(); G_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE() -> Py { ensure_init(); G_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE(v: Py) { ensure_init(); G_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_STANDS_DOWN() -> Py { ensure_init(); G_STANDS_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STANDS_DOWN(v: Py) { ensure_init(); G_STANDS_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Somebody they are close to is having a rough patch, and the store only says it back.\n\nHalf the hard things a person brings a chatbot are second-hand: \"my husband is having a hard\ntime\", \"my mate is going through a rough time\", \"my daughter is going through a rough patch\".\nNothing in this bot knows their husband, so no card takes the line, and the composer does not read\nit as a feeling either - the feeling is not theirs, and every feeling pool stands down when a line\nnames somebody else (`bot/compose/trouble.py`, `OTHERS`). What is left is the ack, so the person\ntells the bot that somebody they love is struggling and gets their own sentence handed back:\n\n    my wife is having a rough time        \"Your wife is having a rough time, good to know.\"\n    my neighbour is having a hard time    \"Noted: your neighbour is having a hard time.\"\n    my brother is having a tough time     \"Your brother is having a tough time - noted.\"\n    my daughter is going through a rough patch   \"Got it - your daughter is going through a\n                                                  rough patch.\"\n\nSTYLE.md asks a reply to answer the person in its first sentence, and to be concrete. A receipt\nis neither: it adds nothing to what they just said, and it does not notice that the thing they\nbrought was hard. The one thing this bot can honestly do with the shape is say the true thing -\nit is hard to watch - and ask the question that helps a person in that position, which is about\nthe person they are worried about.\n\nThis module reads only that shape, and only to answer; it never says a fact about anybody. No\ncause, no diagnosis, no advice about treatment: \"they are depressed\", \"it is probably the move\",\n\"they should see a doctor\" are all things this file must never say. What it knows is what the\nperson just told it, and what it asks is the next thing a friend would ask.\n\nIt checks out of everything else (`STANDS_DOWN`). A line that names a body part or an illness, a\ndeath, or that asks the bot something (\"what do i say to her\", \"how can i help\") has a card of\nits own - `bot/compose/trouble.py` for the body, `bot/compose/loss.py` for a death, the knowledge\nbase for anything asked as advice - and a fallback that took one of those turns would make the\nbot worse, not better.\n\nIt is called from `bot/modules/greeting.py`, and only where the composer's line was a hedge or\nwas the person's own words back: a module with an answer for the line has already answered.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    fn __data1() -> Py { py_tuple(vec![py_str("best friend"), py_str("best mate"), py_str("close friend"), py_str("old friend"), py_str("friend"), py_str("mate"), py_str("pal"), py_str("buddy"), py_str("flatmate"), py_str("housemate"), py_str("workmate"), py_str("schoolmate"), py_str("colleague"), py_str("boss"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("ex"), py_str("mum"), py_str("mom"), py_str("mam"), py_str("mother"), py_str("dad"), py_str("father"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("baby"), py_str("sister"), py_str("brother"), py_str("sibling"), py_str("stepson"), py_str("stepdaughter"), py_str("stepmum"), py_str("stepdad"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("grandmother"), py_str("grandfather"), py_str("aunt"), py_str("auntie"), py_str("uncle"), py_str("cousin"), py_str("nephew"), py_str("niece"), py_str("neighbour"), py_str("neighbor"), py_str("parents"), py_str("kids"), py_str("children"), py_str("sisters"), py_str("brothers"), py_str("friends"), py_str("mates")]) }
    s_SOMEONE(__data1());
    s_BETWEEN(py_str("(?:(?:best|closest|close|poor|old|little|big|lil|eldest|youngest|older|younger|grown[-\\s]?up|step|new|other)\\s+)*"));
    s_A_PERSON_OF_THEIRS(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:my|our)\\s+"), &g_BETWEEN()), &py_str("(?:")), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SOMEONE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "replace", &[py_str(" "), py_str("[\\s-]?")])); } py_list(__out) })() }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_ROUGH_PATCH(stdlib_call("re", "compile", &[py_str("\n    ^\\s*(?:\n        (?:is|are|was|were|has|have|had)\\b\n      | s\\s+been\\b\n      | been\\b\n    )\n    # \"really struggling\", \"still having a hard time\", \"been a bit down\": the adverbs a person\n    # puts in front of it, and \"been\" twice over (\"has been really down\").\n    (?:\\s+(?:really|very|so|just|still|properly|proper|completely|totally|quite|pretty|a\\s+bit|\n            well|dead|awful|genuinely|seriously|always|also|obviously|clearly|been|badly))*\\s+\n    (?:\n        having\\s+(?:a\\s+|one\\s+hell\\s+of\\s+a\\s+|such\\s+a\\s+|the\\s+worst\\s+)?\n          (?:(?:really|very|proper|properly|awful|bit\\s+of\\s+a|lot\\s+of|terrible)\\s+)?\n          (?:hard|tough|rough|bad|rubbish|awful|terrible|difficult|stressful|horrible|lousy|\n             miserable|shit|shitty|grim|dreadful)\\s+\n          (?:time|patch|spell|week|month|year|day|run)\\b\n      | struggling\\b(?!\\s+with\\b)\n      | finding\\s+(?:it|things|everything|life|work|the\\s+job)\\s+\n          (?:a\\s+bit\\s+|really\\s+|very\\s+|so\\s+|quite\\s+)?hard\\b\n      | going\\s+through\\s+(?:a\\s+lot|a\\s+(?:rough|hard|tough|bad)\\s+(?:time|patch|spell)|hell|\n                             it|something\\s+bad|a\\s+bad\\s+patch)\\b\n      | in\\s+a\\s+bad\\s+way\\b\n      | at\\s+breaking\\s+point\\b\n    )\n"), op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NOT_COPING(stdlib_call("re", "compile", &[py_str("\n    ^\\s*(?:(?:is|are|was|were)\\s+not|isn'?t|aren'?t|wasn'?t|hasn'?t|havent|weren'?t|not|\n            never)\\b\n    (?:\\s+(?:really|very|so|just|still|been|at\\s+the\\s+moment|at\\s+all))*\\s+\n    coping\\b.*$\n"), op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LOW(stdlib_call("re", "compile", &[py_str("\n    ^\\s*(?:(?:is|are|was|were|has|have)\\b|s\\s+been\\b|been\\b)\n    (?:\\s+(?:really|very|so|a\\s+bit|quite|pretty|proper|properly|just|always|still|been))*\\s+\n    (?:down|low)\\b\n    (?:\\s+(?:at\\s+the\\s+moment|lately|these\\s+days|just\\s+now|right\\s+now|again|too|still|today|\n              this\\s+week|since\\s+[a-z0-9']+|about\\s+[a-z0-9']+|in\\s+(?:him|her)sel(?:f|ves)|\n              and\\s+[a-z0-9' ]+))?\\s*[.!?]*\\s*$\n"), op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONDITION(stdlib_call("re", "compile", &[py_str("\n    ^\\s*(?:\n        (?:has|have|had|got)\n      | ['\\u2019]?s\\s+got\n      | (?:is|are|was|were)\n      | (?:could|might|may|must)\\s+be\n      | (?:seems?|appears?)\\s+(?:to\\s+be)?\n    )\n    (?:\\s+(?:really|very|probably|possibly|likely|also|now|actually|officially|just|\n             only|maybe|perhaps|definitely|already|too))*\\s*\n    (?:a\\s+bit\\s+of\\s+|a\\s+touch\\s+of\\s+|an?\\s+|mild\\s+|suspected\\s+)?\n    (?P<condition>dyslexi\\w*|dyspraxi\\w*|dyscalculi\\w*|adhd|add|autis\\w*|asperger\\w*|\n      ocd|tourette\\w*|non[\\s-]?verbal|speech\\s+delay|sensory\\s+processing|deaf|\n      hard\\s+of\\s+hearing|blind|partially\\s+sighted|down'?s?\\s+syndrome|\n      learning\\s+disabilit\\w*|on\\s+the\\s+spectrum)\\b\n"), op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WONDERING(stdlib_call("re", "compile", &[py_str("^\\s*(?:could|might|may|must)\\s+(?:be|have)\\b|\\bmaybe\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONDITION_STATED(py_tuple(vec![py_str("Thanks for telling me. Is there something about your {who} you're trying to work out?"), py_str("Good to know - I'll remember that about your {who}. Anything you're trying to sort out?"), py_str("Thanks. What's the part you're working on with your {who} just now?")]));
    s_A_CONDITION_WONDERED(py_tuple(vec![py_str("That's the hard part, the waiting. What have you noticed so far?"), py_str("It's rough sitting with a maybe like that. Write down what you see and when - that is what anyone you ask will want first. What made you start wondering?"), py_str("Not knowing yet is the worst of it. What has made you wonder?")]));
    s_STANDS_DOWN(stdlib_call("re", "compile", &[py_str("\n    \\b(?:\n        hip|knee|knees|back|shoulder|arm|arms|leg|legs|foot|feet|ankle|wrist|hand|hands|head|\n        headache|migraine|stomach|belly|chest|throat|tooth|teeth|ear|ears|eye|eyes|skin|joint|\n        joints|spine|neck|heart|lung|lungs|blood|pain|pains|hurt|hurts|hurting|ache|aches|aching|\n        ill|sick|sickness|unwell|hospital|doctor|gp|surgery|operation|chemo|cancer|tumour|tumor|\n        diagnos\\w*|scan|scans|medication|tablets|pills|dose|prescription|depress\\w*|anxiety|\n        panic|suicid\\w*|self[-\\s]?harm|addiction|drinking|drunk|sober|withdraw\\w*|\n        died|dying|die|death|dead|funeral|bereave\\w*|grief|grieving|passed\\s+away|\n        tests|exam|exams|redundancy|divorce|money|rent|bills\n    )\\b\n    | \\? | \\b(?:what|how|should|advice|tips|help|why|when|where|who)\\b\n"), op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_POOL(py_tuple(vec![py_str("That's hard, watching someone you're close to go through it. What's been happening?"), py_str("Sorry to hear it. Being the one who notices is its own weight - how are they doing in themselves?"), py_str("That's a worry to be carrying for someone else. What's going on at the moment?"), py_str("Hard when you can see it and can't fix it. Do they know you've noticed?"), py_str("That's a lot for them - and for you to watch. What would help most: someone to listen, or something practical?"), py_str("I'm sorry. It's rough being the one at the side of it. What's the hardest part for you just now?")]));
    s_NAMED(py_tuple(vec![py_str("That's hard, watching your {who} go through it. What's been happening?"), py_str("Sorry to hear it. How is your {who} in themselves?"), py_str("That's a lot for your {who} to carry, and for you to watch. What would help most?")]));
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
    let mut words: Py = Py::None;
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9'?!]+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&words.clone()) { op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[words.clone()])), &py_str(" ")) } else { py_str("") };
    Py::None
}
pub fn _who(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut person: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut phrase: Py = Py::None;
    let _ = py_str("The person as the reply names them: \"my best friend\" is \"best friend\" (the line says\n    \"your {who}\", so the possessive is not said twice).");
    phrase = mcall(&slice_or_raise(&said.clone(), &mcall(&person.clone(), "start", &[]), &mcall(&person.clone(), "end", &[]), &Py::None), "strip", &[]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:my|our)\\s+")); __args.push(py_str("")); __args.push(phrase.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    else { Py::None };
    let mut a_condition: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut person: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut who: Py = Py::None;
    let mut worded: Py = Py::None;
    let _ = py_str("The friend's reply to somebody else's rough patch, or None when the line is not that.\n\n    Only the shape is read: a person of theirs, as the subject, having a rough time. Everything\n    else - the body, illness, a death, a question, an ask for advice - is left to the module that\n    owns it.\n    ");
    said = _said(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_STANDS_DOWN(), "search", &[said.clone()])) {
    return Py::None;
    }
    person = mcall(&g_A_PERSON_OF_THEIRS(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&person.clone()))) {
    return Py::None;
    }
    tail = slice_or_raise(&said.clone(), &mcall(&person.clone(), "end", &[]), &Py::None, &Py::None);
    a_condition = mcall(&g_A_CONDITION(), "match", &[tail.clone()]);
    if py_truthy(&{ let __b = a_condition.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "match", &[py_str("^\\s*(?:could|might|may|must)\\s+have\\b"), tail.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = a_condition.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_ROUGH_PATCH(), "match", &[tail.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_NOT_COPING(), "match", &[tail.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_LOW(), "match", &[tail.clone()]) } } } } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = a_condition.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "match", &[py_str("^\\s*(?:and|&)\\s+(?:i|me)\\b"), tail.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("^\\s*(?:and|&)\\s+(?:i|me)\\b"), tail.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    read = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    read = if py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { read.clone() } else { py_dict(vec![]) };
    plural = call_builtin("bool", &[stdlib_call("re", "match", &[py_str("^\\s*(?:are|were|have|have\\s+been|were\\s+being)\\b"), tail.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&a_condition.clone()) {
    pool = if py_truthy(&mcall(&g_A_WONDERING(), "search", &[tail.clone()])) { g_A_CONDITION_WONDERED() } else { g_A_CONDITION_STATED() };
    } else {
    pool = if py_truthy(&plural.clone()) { g_POOL() } else { op_add(&g_POOL(), &g_NAMED()) };
    }
    who = _who(&[person.clone(), said.clone()]);
    worded = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "replace", &[py_str("{who}"), who.clone()])); } py_list(__out) })() };
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    last = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]) } else { Py::None };
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&worded.clone(), &last.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[worded.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } } } }) {
    worded = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&worded.clone()) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&line.clone(), &last.clone()))) {  __out.push(line.clone()); } } py_list(__out) })() };
    }
    return pyrt::xrefs::xcall("compose", "pick", &[worded.clone(), read.clone(), mcall(&said.clone(), "strip", &[])]);
    Py::None
}
pub fn may_replace(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut fallback: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("True when the bot's line for this turn says nothing about what the person brought.");
    said = mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    if py_truthy(&{ let __b = mcall(&fallback.clone(), "_is_a_hedge", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&fallback.clone(), "_in_a_pool", &[said.clone(), py_tuple(vec![gattr(&fallback.clone(), "NEUTRAL")])]) } }) {
    return Py::Bool(true);
    }
    mine = mcall(&fallback.clone(), "_the_words", &[text.clone()]);
    theirs = mcall(&fallback.clone(), "_the_words", &[said.clone()]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&theirs.clone())) } }) {
    return Py::Bool(false);
    }
    shared = op_bitand(&mine.clone(), &theirs.clone());
    return { let __b = op_ge(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { op_ge(&op_truediv(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Float(0.6)) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.theirperson".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CONDITION" => Some(g_A_CONDITION()),
            "A_CONDITION_STATED" => Some(g_A_CONDITION_STATED()),
            "A_CONDITION_WONDERED" => Some(g_A_CONDITION_WONDERED()),
            "A_LOW" => Some(g_A_LOW()),
            "A_NOT_COPING" => Some(g_A_NOT_COPING()),
            "A_PERSON_OF_THEIRS" => Some(g_A_PERSON_OF_THEIRS()),
            "A_ROUGH_PATCH" => Some(g_A_ROUGH_PATCH()),
            "A_WONDERING" => Some(g_A_WONDERING()),
            "BETWEEN" => Some(g_BETWEEN()),
            "NAMED" => Some(g_NAMED()),
            "POOL" => Some(g_POOL()),
            "SOMEONE" => Some(g_SOMEONE()),
            "STANDS_DOWN" => Some(g_STANDS_DOWN()),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "_who" => Some(py_func("_who", |args: &[Py]| _who(args))),
            "may_replace" => Some(py_func("may_replace", |args: &[Py]| may_replace(args))),
            "pick" => Some(g_pick()),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CONDITION" => s_A_CONDITION(v),
            "A_CONDITION_STATED" => s_A_CONDITION_STATED(v),
            "A_CONDITION_WONDERED" => s_A_CONDITION_WONDERED(v),
            "A_LOW" => s_A_LOW(v),
            "A_NOT_COPING" => s_A_NOT_COPING(v),
            "A_PERSON_OF_THEIRS" => s_A_PERSON_OF_THEIRS(v),
            "A_ROUGH_PATCH" => s_A_ROUGH_PATCH(v),
            "A_WONDERING" => s_A_WONDERING(v),
            "BETWEEN" => s_BETWEEN(v),
            "NAMED" => s_NAMED(v),
            "POOL" => s_POOL(v),
            "SOMEONE" => s_SOMEONE(v),
            "STANDS_DOWN" => s_STANDS_DOWN(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.theirperson", module_obj);
    pyrt::xrefs::reg_fn("compose.theirperson", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("compose.theirperson", "_who", py_func("_who", |args: &[Py]| _who(args)));
    pyrt::xrefs::reg_fn("compose.theirperson", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("compose.theirperson", "may_replace", py_func("may_replace", |args: &[Py]| may_replace(args)));
    pyrt::xrefs::reg_access("compose.theirperson", "A_CONDITION", g_A_CONDITION, s_A_CONDITION);
    pyrt::xrefs::reg_access("compose.theirperson", "A_CONDITION_STATED", g_A_CONDITION_STATED, s_A_CONDITION_STATED);
    pyrt::xrefs::reg_access("compose.theirperson", "A_CONDITION_WONDERED", g_A_CONDITION_WONDERED, s_A_CONDITION_WONDERED);
    pyrt::xrefs::reg_access("compose.theirperson", "A_LOW", g_A_LOW, s_A_LOW);
    pyrt::xrefs::reg_access("compose.theirperson", "A_NOT_COPING", g_A_NOT_COPING, s_A_NOT_COPING);
    pyrt::xrefs::reg_access("compose.theirperson", "A_PERSON_OF_THEIRS", g_A_PERSON_OF_THEIRS, s_A_PERSON_OF_THEIRS);
    pyrt::xrefs::reg_access("compose.theirperson", "A_ROUGH_PATCH", g_A_ROUGH_PATCH, s_A_ROUGH_PATCH);
    pyrt::xrefs::reg_access("compose.theirperson", "A_WONDERING", g_A_WONDERING, s_A_WONDERING);
    pyrt::xrefs::reg_access("compose.theirperson", "BETWEEN", g_BETWEEN, s_BETWEEN);
    pyrt::xrefs::reg_access("compose.theirperson", "NAMED", g_NAMED, s_NAMED);
    pyrt::xrefs::reg_access("compose.theirperson", "POOL", g_POOL, s_POOL);
    pyrt::xrefs::reg_access("compose.theirperson", "SOMEONE", g_SOMEONE, s_SOMEONE);
    pyrt::xrefs::reg_access("compose.theirperson", "STANDS_DOWN", g_STANDS_DOWN, s_STANDS_DOWN);
    pyrt::xrefs::reg_access("compose.theirperson", "pick", g_pick, s_pick);
}
