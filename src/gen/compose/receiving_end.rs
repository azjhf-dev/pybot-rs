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


pub static A_BODY_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BRUSH_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COMPLAINT_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERSON_WHO_KEEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WARM_MEMORY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_DOING_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_WHO_KEEPS_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROLE_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROLE_WITH_SOMEBODY_ABOVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_BRUSH_OFF_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_A_BODY_LINE() -> Py { ensure_init(); A_BODY_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BODY_LINE(v: Py) { ensure_init(); if let Ok(mut w) = A_BODY_LINE.write() { *w = v; } }
pub fn g_A_BRUSH_OFF() -> Py { ensure_init(); A_BRUSH_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BRUSH_OFF(v: Py) { ensure_init(); if let Ok(mut w) = A_BRUSH_OFF.write() { *w = v; } }
pub fn g_A_COMPLAINT_WORD() -> Py { ensure_init(); A_COMPLAINT_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COMPLAINT_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_COMPLAINT_WORD.write() { *w = v; } }
pub fn g_A_PERSON_WHO_KEEPS() -> Py { ensure_init(); A_PERSON_WHO_KEEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_WHO_KEEPS(v: Py) { ensure_init(); if let Ok(mut w) = A_PERSON_WHO_KEEPS.write() { *w = v; } }
pub fn g_A_WARM_MEMORY() -> Py { ensure_init(); A_WARM_MEMORY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WARM_MEMORY(v: Py) { ensure_init(); if let Ok(mut w) = A_WARM_MEMORY.write() { *w = v; } }
pub fn g_KEEPS_DOING_IT() -> Py { ensure_init(); KEEPS_DOING_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_DOING_IT(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_DOING_IT.write() { *w = v; } }
pub fn g_PERSON_WHO_KEEPS_REPLIES() -> Py { ensure_init(); PERSON_WHO_KEEPS_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_WHO_KEEPS_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_WHO_KEEPS_REPLIES.write() { *w = v; } }
pub fn g_ROLE_SAID_BACK() -> Py { ensure_init(); ROLE_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROLE_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = ROLE_SAID_BACK.write() { *w = v; } }
pub fn g_ROLE_WITH_SOMEBODY_ABOVE() -> Py { ensure_init(); ROLE_WITH_SOMEBODY_ABOVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROLE_WITH_SOMEBODY_ABOVE(v: Py) { ensure_init(); if let Ok(mut w) = ROLE_WITH_SOMEBODY_ABOVE.write() { *w = v; } }
pub fn g_THEIR() -> Py { ensure_init(); THEIR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR(v: Py) { ensure_init(); if let Ok(mut w) = THEIR.write() { *w = v; } }
pub fn g_THE_BRUSH_OFF_REPLIES() -> Py { ensure_init(); THE_BRUSH_OFF_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_BRUSH_OFF_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = THE_BRUSH_OFF_REPLIES.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A person in a role who keeps doing the same thing, or was brushed off: \"my boss keeps moving\nthe goalposts\", \"my manager says its just how it is\".\n\nThese lines name no ask at all, so on main they reached the two pools that fit any message (\"Got it.\nIs there anything you'd like a hand with?\", \"I'm listening - say more if it helps\"). The person has\nsaid the whole of it and is waiting to hear whether they are being unreasonable. Nothing in the line\ncan be looked up - it is between them and somebody else - so the reply says that in a few words and\nasks the one thing that decides the next step: said to them, or taken higher.\n\nEvery reply leads with the step the bot does know - put it in writing once, dates and what you want,\nkeep a copy - and then names where the rest of it lives: one rung above, ACAS, the complaints\nprocedure, the ombudsman. A pointer on its own, or a question back on its own, is not an answer, and\nneither is a warm line that says nothing.\n\nThe reading lives here rather than in `bot/compose/fallback.py` because that file is over the size\nthe world accepts, so a change to it cannot be submitted at all. `bot/modules/receiving_end.py` is\nits band: `PRIORITY` -980, above the greeting fallback (-1000) and below the nudge (-950), so it can\nonly ever replace the two pools that fit any message. It returns None on every other line.\n");
    s_ROLE_WITH_SOMEBODY_ABOVE(py_str("(?:boss|manager|line\\s+manager|team\\s+lead|team\\s+leader|team\\s+manager|supervisor|shift\\s+lead|foreman|colleague|co-?worker|teacher|head\\s?teacher|tutor|lecturer|landlord|landlady|letting\\s+agent|social\\s+worker|social\\s+services|housing\\s+officer|housing\\s+association|jobcentre|council|school|nursery|college|university|gp\\s+surgery|doctors\\s+surgery|surgery|practice|hr|payroll|recruiter|agency)"));
    s_A_COMPLAINT_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:never|not|no|wont|won'?t|doesnt|doesn'?t|didnt|didn'?t|ignores?|ignored|forgets?|forgot|losing|loses|lost|lies|lying|lied|shouts?|shouted|yell\\w*|blames?|blamed|criticis\\w*|undermin\\w*|gaslight\\w*|cancel\\w*|clos\\w*|chang\\w*|changes|moves?|moved|goalposts|overrules?|shuts? me down|snaps? at me|rolls? (?:his|her|their) eyes|leaves? me out|rules|last minute|wrong|unfair|credit|favourites?|piling|pile|micromanag\\w*|hiding|refuses?|refused|picks on|picked on|talks over|talked over|belittles?|belittled|mess(?:ed)? up|muck\\w*|stops? me|stopped me|keeps me|kept me|delay\\w*|chase\\w*|impossible|nightmare|stress\\w*|awful|horrible|useless|lazy|rude|bully\\w*|unprofessional)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_DOING_IT(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?|always|never|constantly|every\\s+(?:single\\s+)?(?:time|week|day|shift|month|term)|again\\s+and\\s+again|over\\s+and\\s+over|all\\s+the\\s+time|ignores?|ignored|picks?\\s+on\\s+me|undermines?\\s+me|talks?\\s+over\\s+me|belittles?\\s+me|gaslights?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_WHO_KEEPS(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:(?:my|our|his|her|their|the)\\s+(?:new\\s+|old\\s+|current\\s+|ex\\s+)?)?"), &g_ROLE_WITH_SOMEBODY_ABOVE()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WARM_MEMORY(stdlib_call("re", "compile", &[py_str("\\b(?:always|used\\s+to)\\s+(?:said|told|taught|gave|made\\s+me|let\\s+me|wanted|encouraged)\\b|\\b(?:was|were)\\s+(?:great|lovely|kind|good|brilliant|fair|helpful)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BRUSH_OFF(stdlib_call("re", "compile", &[py_str("\\b(?:says?\\s+(?:that\\s+)?(?:its|it'?s|thats|that'?s)\\s+(?:just\\s+)?(?:how|the\\s+way)\\b|says?\\s+(?:that\\s+)?(?:its|it'?s|thats|that'?s)\\s+life\\b|shrugs?\\s+(?:it\\s+off|it|and)\\b|says?\\s+(?:to\\s+)?(?:just\\s+)?(?:get\\s+on\\s+with\\s+it|put\\s+up\\s+with\\s+it)|says?\\s+(?:everyone|everybody|we\\s+all)\\b[^.?!]{0,40}?(?:put[s]?\\s+up\\s+with|has\\s+to)|keeps?\\s+saying\\s+(?:we'?ll\\s+see|next\\s+(?:month|week|year)|soon|in\\s+a\\s+(?:bit|minute|while))|won'?t\\s+even\\s+(?:say|answer|reply|discuss))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_BRUSH_OFF_REPLIES(py_tuple(vec![py_str("That is a brush-off rather than an answer. Nothing to look up here, but the move that works is a short email: say what they said, what you want changed, and ask for a reply by a date. If the date passes, that email is what you take one rung above them, or to ACAS if it is a workplace."), py_str("When the answer is that is just how it is, ask once more in writing - what you want changed and by when - and keep the reply. That email is what the next rung reads: ACAS for a workplace, or the complaints procedure every school, council and surgery has to follow."), py_str("Nothing to look up here, but do not leave it spoken: put the request in writing with a date on it and keep a copy. If {role} will not answer that, the next step is one rung above them, and after that the ombudsman, which is free to use and is where a case finally gets looked at."), py_str("There is a shape to these answers and it is worth naming: ask for the reason in writing, once, with the date you want it by. A recorded refusal is something you can challenge, and the complaints procedure is the route - it has to end somewhere, usually with the ombudsman.")]));
    s_PERSON_WHO_KEEPS_REPLIES(py_tuple(vec![py_str("Nothing in that to look up, but the step is the same as for any pattern like this: put it in writing once - the date, what happened, what you want - and keep a copy. If it does not come back in a week, it goes one rung above {role}, and ACAS gives free advice on the wording if it is a workplace."), py_str("When it happens again and again the wording decides it: dates, what you asked for, what happened next. Send that once to {role} and ask for an answer by a date. If it does not come, ask for the written complaints or grievance procedure and follow the step it names."), py_str("Keep a short record - the date, what happened, what you asked for. A record is what makes {role} have to answer, and it is what the next rung reads. Every school, council and GP surgery has a complaints procedure with named steps, and asking for a copy is the first one."), py_str("That is a pattern rather than one bad day, so put it in writing rather than say it again: what you want changed, with dates. Nothing to look up here, but the complaints procedure is where this goes next - ask for a copy of it and follow the step it names.")]));
    s_ROLE_SAID_BACK(py_dict(vec![(py_str("hr"), py_str("HR")), (py_str("gp surgery"), py_str("the surgery")), (py_str("doctors surgery"), py_str("the surgery")), (py_str("surgery"), py_str("the surgery")), (py_str("school"), py_str("the school")), (py_str("nursery"), py_str("the nursery")), (py_str("council"), py_str("the council")), (py_str("university"), py_str("the university")), (py_str("college"), py_str("the college")), (py_str("jobcentre"), py_str("the jobcentre"))]));
    s_THEIR(py_dict(vec![(py_str("my"), py_str("your")), (py_str("our"), py_str("your")), (py_str("his"), py_str("their")), (py_str("her"), py_str("their")), (py_str("their"), py_str("their"))]));
    s_A_BODY_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:knife|gun|knives|weapon|throat|strangl\\w*|chok\\w*|punch\\w*|beat (?:me|him|her)|hit (?:me|him|her|my)|hits? me|hurt(?:ing)? me|hurts? me|threaten\\w*|threats?|kill me|kill(?:ed)? (?:him|her|them)|bruis\\w*|abus\\w*|assault\\w*|rape\\w*|black eye|broken (?:arm|nose|ribs?))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_role_said_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut _u: Py = Py::None;
    let mut first: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut role: Py = Py::None;
    let _ = py_str("The role as the person wrote it, facing them the right way round.");
    role = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = found.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_ROLE_SAID_BACK(), &role.clone()))) {
    return geti(&g_ROLE_SAID_BACK(), &role.clone());
    }
    if py_truthy(&mcall(&role.clone(), "startswith", &[py_tuple(vec![py_str("the "), py_str("a ")])])) {
    return role.clone();
    }
    let __unpk1 = py_unpack(&mcall(&role.clone(), "partition", &[py_str(" ")]), 3, None);
    first = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    rest = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THEIR(), &first.clone())); if !py_truthy(&__b) { __b } else { rest.clone() } }) {
    return add(&add(&geti(&g_THEIR(), &first.clone()), &py_str(" ")), &rest.clone());
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("your"), py_str("their"), py_str("the"), py_str("a")]), &first.clone()))) {
    return role.clone();
    }
    return add(&py_str("the "), &role.clone());
    Py::None
}
pub fn _a_body_line_above(args: &[Py]) -> Py {
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
    let _ = py_str("True when one of the last two things the person said was about a body or a threat.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&said.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut line: Py = __item.clone(); __out.push(mcall(&g_A_BODY_LINE(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&line.clone(), "lower", &[]), "split", &[])])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_person_who_keeps(args: &[Py]) -> Py {
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
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut after: Py = Py::None;
    let mut brush: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut role: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says: Py = Py::None;
    let _ = py_str("The question back to \"my boss keeps moving the goalposts\", or None.\n\n    Both halves of the turn are read here because they are the same turn: the thing happens\n    again and again, or the person said it once and was brushed off. In either case nothing in\n    the line can be looked up and the next step is the same question.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    role = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![said.clone(), mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = repaired.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]));
    for __item in __iter1 {
    line = __item.clone();
    match_ = mcall(&g_A_PERSON_WHO_KEEPS(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    after = slice_or_raise(&line.clone(), &mcall(&match_.clone(), "end", &[]), &add(&mcall(&match_.clone(), "end", &[]), &Py::Int(70)), &Py::None);
    brush = mcall(&g_A_BRUSH_OFF(), "search", &[after.clone()]);
    if py_truthy(&{ let __b = { let __b = brush.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_KEEPS_DOING_IT(), "search", &[after.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_COMPLAINT_WORD(), "search", &[line.clone()]) } } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_WARM_MEMORY(), "search", &[line.clone()]))) } }) {
    role = _the_role_said_back(&[slice_or_raise(&line.clone(), &mcall(&match_.clone(), "start", &[]), &mcall(&match_.clone(), "end", &[]), &Py::None)]);
    pool = if py_truthy(&brush.clone()) { g_THE_BRUSH_OFF_REPLIES() } else { g_PERSON_WHO_KEEPS_REPLIES() };
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&role.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = crate::gen::compose::fallback::in_a_safety_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_grief_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_in_a_loss_thread(&[memory.clone()]); if py_truthy(&__b) { __b } else { _a_body_line_above(&[memory.clone()]) } } } } } }) {
    return Py::None;
    }
    says = call_builtin("str", &[{ let __b = crate::gen::compose::fallback::_the_line_before(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = says.clone(); if !py_truthy(&__b) { __b } else { crate::gen::compose::fallback::_in_a_pool(&[says.clone(), py_tuple(vec![g_PERSON_WHO_KEEPS_REPLIES(), g_THE_BRUSH_OFF_REPLIES()])]) } }) {
    return Py::None;
    }
    return py_call(&crate::gen::compose::fallback::g__choose(), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("role".to_string(), role.clone()), ("Role".to_string(), add(&mcall(&geti(&role.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&role.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), clean.clone()]);
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut repaired: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repaired") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repaired"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("The reply to a role that keeps doing it, or none for every other line.");
    return _a_person_who_keeps(&[clean.clone(), repaired.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.receiving_end".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BODY_LINE" => Some(g_A_BODY_LINE()),
            "A_BRUSH_OFF" => Some(g_A_BRUSH_OFF()),
            "A_COMPLAINT_WORD" => Some(g_A_COMPLAINT_WORD()),
            "A_PERSON_WHO_KEEPS" => Some(g_A_PERSON_WHO_KEEPS()),
            "A_WARM_MEMORY" => Some(g_A_WARM_MEMORY()),
            "KEEPS_DOING_IT" => Some(g_KEEPS_DOING_IT()),
            "PERSON_WHO_KEEPS_REPLIES" => Some(g_PERSON_WHO_KEEPS_REPLIES()),
            "ROLE_SAID_BACK" => Some(g_ROLE_SAID_BACK()),
            "ROLE_WITH_SOMEBODY_ABOVE" => Some(g_ROLE_WITH_SOMEBODY_ABOVE()),
            "THEIR" => Some(g_THEIR()),
            "THE_BRUSH_OFF_REPLIES" => Some(g_THE_BRUSH_OFF_REPLIES()),
            "_a_body_line_above" => Some(py_func("_a_body_line_above", |args: &[Py]| _a_body_line_above(args))),
            "_a_person_who_keeps" => Some(py_func("_a_person_who_keeps", |args: &[Py]| _a_person_who_keeps(args))),
            "_the_role_said_back" => Some(py_func("_the_role_said_back", |args: &[Py]| _the_role_said_back(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: None,
    }))
}
