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


thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STAY_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMETHING_WRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_BOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WARD_KNOWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.ward: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PET() -> Py { ensure_init(); G_A_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PET(v: Py) { ensure_init(); G_A_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STAY_IN_THE_LINE() -> Py { ensure_init(); G_A_STAY_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STAY_IN_THE_LINE(v: Py) { ensure_init(); G_A_STAY_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_DETAIL() -> Py { ensure_init(); G_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETAIL(v: Py) { ensure_init(); G_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME_AGAIN() -> Py { ensure_init(); G_HOME_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME_AGAIN(v: Py) { ensure_init(); G_HOME_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON() -> Py { ensure_init(); G_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); G_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE() -> Py { ensure_init(); G_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE(v: Py) { ensure_init(); G_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_SELF() -> Py { ensure_init(); G_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SELF(v: Py) { ensure_init(); G_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMETHING_WRONG() -> Py { ensure_init(); G_SOMETHING_WRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMETHING_WRONG(v: Py) { ensure_init(); G_SOMETHING_WRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_STAFF() -> Py { ensure_init(); G_STAFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAFF(v: Py) { ensure_init(); G_STAFF.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_BOT() -> Py { ensure_init(); G_THE_BOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_BOT(v: Py) { ensure_init(); G_THE_BOT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WARD_KNOWS() -> Py { ensure_init(); G_THE_WARD_KNOWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WARD_KNOWS(v: Py) { ensure_init(); G_THE_WARD_KNOWS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOSE() -> Py { ensure_init(); G_WHOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOSE(v: Py) { ensure_init(); G_WHOSE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A question about somebody's care is the ward's to answer, not a search.\n\nOn main the questions families actually ask while somebody is in hospital all reach the\ncomposer's search hedge - \"I don't know enough to answer that honestly. A search should sort you\nout quickly\":\n\n    how long will she be in hospital        -> the search hedge\n    when can she come home from hospital    -> \"That's beyond what I know well, sorry\"\n    what ward is she on                     -> the search hedge\n    is he allowed visitors                  -> the search hedge\n    what time is her operation              -> the search hedge\n    has she had the scan results yet        -> \"That's beyond what I know well, sorry\"\n\nNo search settles any of them. How long a stay is, when somebody is discharged, which ward they\nare on, when the theatre list runs and whether the results are back are all in that person's\nnotes, and the only people holding the notes are the ones looking after them. Telling somebody\nto look it up sends them away from the one place that can answer, which is the failure STYLE.md's\n\"Not knowing, honestly\" is written against: say what the bot cannot do, then name who knows.\n\nSo every reply does two things and nothing else: it says plainly that nothing here can see the\nnotes, and it names the ward and the one thing to do - ring, ask for the nurse looking after them\nand put the question as it was put here. Nothing is guessed: not a number of days, not a ward\nname, not a visiting hour. The reading is the same for all of them because the reply is the same:\nthis is somebody else's patient and the notes are not here.\n\nWhat is read, in the line, is\n\n  - somebody it is about: he/she/they, or \"my/our/his/her/their <person>\" (mum, dad, son,\n    daughter, wife, husband, partner, gran, grandad, sister, brother, friend, baby, ...), or the\n    speaker with a place of care in the line - \"how long will i be in hospital\" is their own\n    notes as much as anybody's;\n  - a place or a person of care - hospital, ward, hospice, care home, clinic, unit, theatre,\n    consultant, doctor, nurse, midwife - or one of the care facts that live in the notes: the\n    operation, the scan, the results, the discharge, the visitors;\n  - and a practical ask about it: how long, when, what time, which, where, whether they are in\n    or out, have they had, can i, is he allowed.\n\nIt stands down where another reading owns the line, because those are answers and this file is\nthe last resort: a symptom or an injury (\"is her chest infection serious\", \"her wound is\nbleeding\") is the health cards', a how-to about doing something (\"should i drive after the\noperation\") is knowledge the bot has, a thing of theirs to tell (\"how did the scan go\") is\nhow_it_went.py, and a pet's stay is the vet's line, which the composer already has. Like the\nother readers of a hedge, `bot/modules/greeting.py` asks this only when the composer has nothing\nbut a shrug, so a line with an answer of its own never reaches this file.\n");
    s_PERSON(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:he|she|they|him|her|them|his|their)(?![a-z])|(?<![a-z])(?:my|our|his|her|their)\\s+(?:mum|mom|mother|dad|father|son|daughter|child|kid|wife|husband|partner|brother|sister|gran|granny|grandma|grandad|grandpa|nana|nan|aunt|uncle|cousin|friend|mate|neighbour|neighbor|baby|boy|girl|lad|boss|colleague)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:i|me|my|mine|myself)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:hospital|ward|hospice|care home|nursing home|residential home|clinic|infirmary|a&e|theatre|theatres)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STAFF(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:consultant|doctor|nurse|midwife|anaesthetist|anesthetist|surgeon|registrar|duty nurse)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DETAIL(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:operation|surgery|op|procedure|scan|x-ray|xray|mri|ct scan|ultrasound|results|test results|notes|chart|discharge|discharged|visitors|visiting|visiting hours)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how long|how soon|how many days|when|what time|what ward|what room|what unit|what bed|what hospital|which|where|has (?:she|he|they)|have they|is (?:she|he|they)|are they|will (?:she|he|they)|can i|can we|could i|am i allowed|are we allowed|did they)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMETHING_WRONG(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:pain|painful|hurts?|hurting|aching|aches?|sore|swollen|swelling|bleeding|bled|fever|temperature|rash|infection|infected|vomit|vomiting|dizzy|breathless|bruise|wound|symptom|diagnos\\w*)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PET(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:vet|vets|veterinary|puppy|kitten|rabbit|hamster|guinea|budgie|parrot|dog|cat|horse|pony)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOME_AGAIN(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:is (?:now )?home|are (?:now )?home|was home|were home|got home|back home|came home|has come home|had come home|left (?:the )?hospital|came out of (?:the )?hospital|got out of (?:the )?hospital|out of (?:the )?hospital now|(?<!come )(?<!coming )(?<!comes )(?<!get )(?<!getting )(?<!go )(?<!going )(?<!going to )home from (?:the )?hospital|(?:has|have|had) been discharged|was discharged|is discharged)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STAY_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:in for|in there|in here|still in|be in|is (?:she|he|they) in|are they in|staying|stay overnight|staying overnight|can i stay|out of (?:there|hospital)|discharg\\w*|visitors?|visiting|room|bed|bay|ward|(?:coming|came|come) home|(?:going|went) home|get home|getting home|back on (?:her|his|their) feet)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_BOT(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:you|your|yours|yourself)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WARD_KNOWS(py_tuple(vec![py_str("Nobody here can see {whose} notes, so the ward is the way to find out: ring and ask for the nurse looking after {whom}, and put the question as you put it to me."), py_str("That sits in {whose} notes, and nothing here can read them. Ring the ward and ask for the nurse on duty - they will tell you what is happening and what happens next."), py_str("Only the people looking after {whom} can say. Ring the ward, say who you are and who you are asking about, and ask them straight."), py_str("I can't see {whose} chart from here. Ring the ward and ask for the nurse looking after {whom}: they have the notes in front of them and they answer this all day."), py_str("The answer is in {whose} notes, so it has to come from the ward. Ring them, ask for the nurse on duty, and say what you need to know - they can look it up while you are on the phone.")]));
    s_WHOSE(py_dict(vec![(py_str("her"), py_str("her")), (py_str("him"), py_str("his")), (py_str("them"), py_str("their")), (py_str("you"), py_str("your"))]));
}
pub fn _whom(args: &[Py]) -> Py {
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
    let _ = py_str("The pronoun for the person, from the words of the line: her, him, them or you.");
    if py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])she(?![a-z])"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("her");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])he(?![a-z])"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("him");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PERSON(), "search", &[said.clone()])))) {
    return py_str("you");
    }
    return py_str("them");
    Py::None
}
pub fn _the_place_before(args: &[Py]) -> Py {
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
    let _ = py_str("The care words of the last few turns, or \"\" on the first turn.\n\n    A family talks in two parts: \"my mum went into hospital last night\" and then \"how long will\n    she be in for\", \"what room is she in\", \"can i stay overnight with her\". The second turn names\n    the person and the question, and the place is one turn back. Only the place is read from the\n    earlier lines, never the answer, and three turns is enough for a person to have changed the\n    subject.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]);
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
    let mut room: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut whom: Py = Py::None;
    let _ = py_str("The ward line for a question about somebody's care, or None to leave the reply alone.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { mcall(&g_THE_BOT(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    room = mcall(&py_str(" "), "join", &[py_tuple(vec![said.clone(), _the_place_before(&[memory.clone()])])]);
    if py_truthy(&{ let __b = mcall(&g_A_PET(), "search", &[room.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SOMETHING_WRONG(), "search", &[room.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOME_AGAIN(), "search", &[room.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_PLACE(), "search", &[room.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_STAFF(), "search", &[room.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DETAIL(), "search", &[room.clone()]) } } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = mcall(&g_PLACE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_STAFF(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DETAIL(), "search", &[said.clone()]) } } } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_STAY_IN_THE_LINE(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_PERSON(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SELF(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_PLACE(), "search", &[room.clone()]) } } } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ASK(), "search", &[said.clone()])))) {
    return Py::None;
    }
    whom = _whom(&[said.clone()]);
    seed = op_add(&op_mul(&call_builtin("len", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(31)), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("whose".to_string(), geti(&g_WHOSE(), &whom.clone())), ("whom".to_string(), whom.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_THE_WARD_KNOWS(), &op_modulo(&seed.clone(), &call_builtin("len", &[g_THE_WARD_KNOWS()]).unwrap_or_else(|e| exc::raise(e)))), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.ward".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK" => Some(g_ASK()),
            "A_PET" => Some(g_A_PET()),
            "A_STAY_IN_THE_LINE" => Some(g_A_STAY_IN_THE_LINE()),
            "DETAIL" => Some(g_DETAIL()),
            "HOME_AGAIN" => Some(g_HOME_AGAIN()),
            "PERSON" => Some(g_PERSON()),
            "PLACE" => Some(g_PLACE()),
            "SELF" => Some(g_SELF()),
            "SOMETHING_WRONG" => Some(g_SOMETHING_WRONG()),
            "STAFF" => Some(g_STAFF()),
            "THE_BOT" => Some(g_THE_BOT()),
            "THE_WARD_KNOWS" => Some(g_THE_WARD_KNOWS()),
            "WHOSE" => Some(g_WHOSE()),
            "_the_place_before" => Some(py_func("_the_place_before", |args: &[Py]| _the_place_before(args))),
            "_whom" => Some(py_func("_whom", |args: &[Py]| _whom(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASK" => s_ASK(v),
            "A_PET" => s_A_PET(v),
            "A_STAY_IN_THE_LINE" => s_A_STAY_IN_THE_LINE(v),
            "DETAIL" => s_DETAIL(v),
            "HOME_AGAIN" => s_HOME_AGAIN(v),
            "PERSON" => s_PERSON(v),
            "PLACE" => s_PLACE(v),
            "SELF" => s_SELF(v),
            "SOMETHING_WRONG" => s_SOMETHING_WRONG(v),
            "STAFF" => s_STAFF(v),
            "THE_BOT" => s_THE_BOT(v),
            "THE_WARD_KNOWS" => s_THE_WARD_KNOWS(v),
            "WHOSE" => s_WHOSE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.ward", module_obj);
    pyrt::xrefs::reg_fn("compose.ward", "_whom", py_func("_whom", |args: &[Py]| _whom(args)));
    pyrt::xrefs::reg_fn("compose.ward", "_the_place_before", py_func("_the_place_before", |args: &[Py]| _the_place_before(args)));
    pyrt::xrefs::reg_fn("compose.ward", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.ward", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("compose.ward", "A_PET", g_A_PET, s_A_PET);
    pyrt::xrefs::reg_access("compose.ward", "A_STAY_IN_THE_LINE", g_A_STAY_IN_THE_LINE, s_A_STAY_IN_THE_LINE);
    pyrt::xrefs::reg_access("compose.ward", "DETAIL", g_DETAIL, s_DETAIL);
    pyrt::xrefs::reg_access("compose.ward", "HOME_AGAIN", g_HOME_AGAIN, s_HOME_AGAIN);
    pyrt::xrefs::reg_access("compose.ward", "PERSON", g_PERSON, s_PERSON);
    pyrt::xrefs::reg_access("compose.ward", "PLACE", g_PLACE, s_PLACE);
    pyrt::xrefs::reg_access("compose.ward", "SELF", g_SELF, s_SELF);
    pyrt::xrefs::reg_access("compose.ward", "SOMETHING_WRONG", g_SOMETHING_WRONG, s_SOMETHING_WRONG);
    pyrt::xrefs::reg_access("compose.ward", "STAFF", g_STAFF, s_STAFF);
    pyrt::xrefs::reg_access("compose.ward", "THE_BOT", g_THE_BOT, s_THE_BOT);
    pyrt::xrefs::reg_access("compose.ward", "THE_WARD_KNOWS", g_THE_WARD_KNOWS, s_THE_WARD_KNOWS);
    pyrt::xrefs::reg_access("compose.ward", "WHOSE", g_WHOSE, s_WHOSE);
}
