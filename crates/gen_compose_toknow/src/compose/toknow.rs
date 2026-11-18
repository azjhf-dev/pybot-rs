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


thread_local! { static G_A_FEW_WORDS_OF_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRONOUN_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_YOU_AGREE_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHAT_TO_KNOW_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_YOU_AGREE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WORDS_THAT_END_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_THE_BOT_CAN_SAY_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_TO_KNOW_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.toknow: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_FEW_WORDS_OF_THING() -> Py { ensure_init(); G_A_FEW_WORDS_OF_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEW_WORDS_OF_THING(v: Py) { ensure_init(); G_A_FEW_WORDS_OF_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRONOUN_NOT_A_THING() -> Py { ensure_init(); G_A_PRONOUN_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRONOUN_NOT_A_THING(v: Py) { ensure_init(); G_A_PRONOUN_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_YOU_AGREE_TO() -> Py { ensure_init(); G_A_THING_YOU_AGREE_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_YOU_AGREE_TO(v: Py) { ensure_init(); G_A_THING_YOU_AGREE_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHAT_TO_KNOW_ASKED() -> Py { ensure_init(); G_A_WHAT_TO_KNOW_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHAT_TO_KNOW_ASKED(v: Py) { ensure_init(); G_A_WHAT_TO_KNOW_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_YOU_AGREE() -> Py { ensure_init(); G_BEFORE_YOU_AGREE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_YOU_AGREE(v: Py) { ensure_init(); G_BEFORE_YOU_AGREE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WORDS_THAT_END_THE_THING() -> Py { ensure_init(); G_THE_WORDS_THAT_END_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WORDS_THAT_END_THE_THING(v: Py) { ensure_init(); G_THE_WORDS_THAT_END_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_THE_BOT_CAN_SAY_ABOUT_IT() -> Py { ensure_init(); G_WHAT_THE_BOT_CAN_SAY_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_THE_BOT_CAN_SAY_ABOUT_IT(v: Py) { ensure_init(); G_WHAT_THE_BOT_CAN_SAY_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_TO_KNOW_FIRST() -> Py { ensure_init(); G_WHAT_TO_KNOW_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_KNOW_FIRST(v: Py) { ensure_init(); G_WHAT_TO_KNOW_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What to look out for before agreeing to something, or before going somewhere.\n\n`_WAITING` in `fallback_chain.py` reads the person's own knowing as the thing they are waiting for\n- \"i should know by friday\", \"we should know next week\", \"i should know in a few days\" - and it is\nright about those. The same four words sit inside an ordinary question, and the wait reading took\nthem:\n\n    i got a job offer from the garden centre\n    anything i should know about the job\n      before  So it is out of your hands for now. When do you hear back?\n\n    we are looking at a flat to rent on saturday\n    anything i should know about the flat\n      before  Fingers crossed then. When will you know?\n\nNeither is news in the post. What separates them is the frame the words sit in, so the frame is\nwhat is read here: `anything / something / everything / nothing / what ... (i|we|you) should know`\nis a question *about a thing*, and the wait reading stands down for it.\n\nThe ask then keeps a real answer, because a pointer on its own is not one (`STYLE.md`, \"Not\nknowing, honestly\" - lead with what the bot does know, then say where the rest is):\n\n- a thing the knowledge base holds gets its card: \"about lisbon\" gets the Lisbon card, \"about a\n  heat pump\" the heat pump one, \"about pregnancy\" the card about telling an employer;\n- something a person *agrees to* - a job, a flat, an offer, a mortgage, a tenancy, a loan - gets\n  the three things worth checking and the fact that all three are written down before they sign;\n- anything else is named back with the honest shape: what decides it, where the details live, and\n  one question. No fact is invented, and no absence is claimed that the bot cannot back;\n- a pronoun (\"anything i should know about it\") names nothing, so nothing fires.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_A_WHAT_TO_KNOW_ASKED(stdlib_call("re", "compile", &[py_str("\\b(?:anything|something|everything|nothing|what)(?:\\s+else)?\\s+(?:that\\s+)?(?:i|we|you)\\s+(?:should|ought to|need to|have to|must)\\s+know\\b|\\b(?:is|are) there (?:anything|something|much)\\b[^?.!]{0,25}\\b(?:i|we)\\s+know\\b|\\bwhat\\s+(?:should|do|would)\\s+(?:i|we)\\s+know\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_YOU_AGREE_TO(stdlib_call("re", "compile", &[py_str("\\b(?:job|jobs|role|position|post|offer|contract|permanent|probation|shift|rota|hours|pay|salary|apprenticeship|placement|course|uni|university|college|degree|tenancy|lease|landlord|flat|house|mortgage|rent|deposit|move|mover|buying|purchase|loan|credit|insurance|policy|car|van|bike|warranty|gym|membership|subscription|phone contract|broadband|nursery|childcare|school place|deal|signing|sign|agreement|terms)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BEFORE_YOU_AGREE(py_tuple(vec![py_str("The things people are glad they checked are the money after the first month, what it commits you to, and how you get out of it - and all three are written down somewhere before you agree to anything. Which of those is the one you are unsure about?"), py_str("It is nearly always the small print that decides it: what it costs over time, what you can and cannot do, and the notice you have to give. Do you have the paperwork in front of you?"), py_str("Ask what happens if it goes wrong, because that is the part nobody mentions until it has. What is the thing you would be agreeing to?")]));
    s_WHAT_TO_KNOW_FIRST(py_tuple(vec![py_str("Tell me the part you want to get right about {thing} and I will tell you what I know. The general rule is that what you want from it and the one limit you cannot move decide everything else, and the details sit on the official page for it. Which part is it?"), py_str("What I can be useful on is the part you are unsure about, rather than a list: say which bit of {thing} you are weighing and I will take it from there. What is worrying you?")]));
    s_WHAT_THE_BOT_CAN_SAY_ABOUT_IT(py_tuple(vec![py_str("what do you know about {}"), py_str("is {} worth it"), py_str("tell me about {}")]));
    s_A_PRONOUN_NOT_A_THING(stdlib_call("re", "compile", &[py_str("^(?:it|this|that|them|those|these|him|her|me|us|you|one|ones|things|stuff)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WORDS_THAT_END_THE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:before|after|when|if|so|and|but|because|or|for|with|to|in|on|at|from)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEW_WORDS_OF_THING(Py::Int(5));
}
pub fn _the_thing_asked_about(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("\"Anything I should know about the job\" -> \"the job\", for the reply to name it back.");
    match_ = stdlib_call("re", "search", &[py_str("\\bknow\\b[^?.!]{0,10}?\\babout\\b\\s*([^?.!]{1,60})"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    match_ = stdlib_call("re", "search", &[py_str("\\babout\\b\\s*([^?.!]{1,60})\\b(?:before|first)\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    thing = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ?.!,")]);
    cut = mcall(&g_THE_WORDS_THAT_END_THE_THING(), "search", &[thing.clone()]);
    if py_truthy(&cut.clone()) {
    thing = mcall(&slice_or_raise(&thing.clone(), &Py::None, &mcall(&cut.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ?.!,")]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_PRONOUN_NOT_A_THING(), "match", &[thing.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&thing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_A_FEW_WORDS_OF_THING())) {
    return py_str("");
    }
    return thing.clone();
    Py::None
}
pub fn _a_card_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut answers: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The knowledge base's own answer about a named thing, if it has one.");
    let __try1 = exc::catch(|| {
    answers = pyrt::xrefs::xmodule("modules.answers");
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&g_WHAT_THE_BOT_CAN_SAY_ABOUT_IT());
    for __item in __iter6 {
    frame = __item.clone();
    let __try7 = exc::catch(|| {
    said = mcall(&answers.clone(), "respond", &[mcall(&frame.clone(), "format", &[thing.clone()]), py_dict(vec![])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["Exception"]) {
                __handled11 = true;
    said = Py::None;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn ask_about_what_to_look_out_for(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut asked_about: Py = Py::None;
    let mut card: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shaped: Py = Py::None;
    let _ = py_str("None when the line is not this ask, \"\" when it is and there is nothing to say, else a reply.\n\n    The empty string keeps the two apart: a question about a thing is not news in the post even\n    when the bot has nothing to add, so the caller lets the line go on to whatever it was before\n    this reading existed rather than answer it as a wait.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_WHAT_TO_KNOW_ASKED(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_THING_YOU_AGREE_TO(), "search", &[said.clone()])) {
    return { let __b = pyrt::xrefs::xcall("compose", "pick", &[g_BEFORE_YOU_AGREE(), memory.clone(), said.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    }
    asked_about = _the_thing_asked_about(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&asked_about.clone()))) {
    return py_str("");
    }
    card = _a_card_about(&[asked_about.clone()]);
    if py_truthy(&card.clone()) {
    return card.clone();
    }
    shaped = pyrt::xrefs::xcall("compose", "pick", &[g_WHAT_TO_KNOW_FIRST(), memory.clone(), said.clone()]);
    if py_truthy(&shaped.clone()) {
    return mcall(&shaped.clone(), "replace", &[py_str("{thing}"), asked_about.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.toknow".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_FEW_WORDS_OF_THING" => Some(g_A_FEW_WORDS_OF_THING()),
            "A_PRONOUN_NOT_A_THING" => Some(g_A_PRONOUN_NOT_A_THING()),
            "A_THING_YOU_AGREE_TO" => Some(g_A_THING_YOU_AGREE_TO()),
            "A_WHAT_TO_KNOW_ASKED" => Some(g_A_WHAT_TO_KNOW_ASKED()),
            "BEFORE_YOU_AGREE" => Some(g_BEFORE_YOU_AGREE()),
            "THE_WORDS_THAT_END_THE_THING" => Some(g_THE_WORDS_THAT_END_THE_THING()),
            "WHAT_THE_BOT_CAN_SAY_ABOUT_IT" => Some(g_WHAT_THE_BOT_CAN_SAY_ABOUT_IT()),
            "WHAT_TO_KNOW_FIRST" => Some(g_WHAT_TO_KNOW_FIRST()),
            "_a_card_about" => Some(py_func("_a_card_about", |args: &[Py]| _a_card_about(args))),
            "_the_thing_asked_about" => Some(py_func("_the_thing_asked_about", |args: &[Py]| _the_thing_asked_about(args))),
            "ask_about_what_to_look_out_for" => Some(py_func("ask_about_what_to_look_out_for", |args: &[Py]| ask_about_what_to_look_out_for(args))),
            "pick" => Some(g_pick()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_FEW_WORDS_OF_THING" => s_A_FEW_WORDS_OF_THING(v),
            "A_PRONOUN_NOT_A_THING" => s_A_PRONOUN_NOT_A_THING(v),
            "A_THING_YOU_AGREE_TO" => s_A_THING_YOU_AGREE_TO(v),
            "A_WHAT_TO_KNOW_ASKED" => s_A_WHAT_TO_KNOW_ASKED(v),
            "BEFORE_YOU_AGREE" => s_BEFORE_YOU_AGREE(v),
            "THE_WORDS_THAT_END_THE_THING" => s_THE_WORDS_THAT_END_THE_THING(v),
            "WHAT_THE_BOT_CAN_SAY_ABOUT_IT" => s_WHAT_THE_BOT_CAN_SAY_ABOUT_IT(v),
            "WHAT_TO_KNOW_FIRST" => s_WHAT_TO_KNOW_FIRST(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.toknow", module_obj);
    pyrt::xrefs::reg_fn("compose.toknow", "_the_thing_asked_about", py_func("_the_thing_asked_about", |args: &[Py]| _the_thing_asked_about(args)));
    pyrt::xrefs::reg_fn("compose.toknow", "_a_card_about", py_func("_a_card_about", |args: &[Py]| _a_card_about(args)));
    pyrt::xrefs::reg_fn("compose.toknow", "ask_about_what_to_look_out_for", py_func("ask_about_what_to_look_out_for", |args: &[Py]| ask_about_what_to_look_out_for(args)));
    pyrt::xrefs::reg_access("compose.toknow", "A_FEW_WORDS_OF_THING", g_A_FEW_WORDS_OF_THING, s_A_FEW_WORDS_OF_THING);
    pyrt::xrefs::reg_access("compose.toknow", "A_PRONOUN_NOT_A_THING", g_A_PRONOUN_NOT_A_THING, s_A_PRONOUN_NOT_A_THING);
    pyrt::xrefs::reg_access("compose.toknow", "A_THING_YOU_AGREE_TO", g_A_THING_YOU_AGREE_TO, s_A_THING_YOU_AGREE_TO);
    pyrt::xrefs::reg_access("compose.toknow", "A_WHAT_TO_KNOW_ASKED", g_A_WHAT_TO_KNOW_ASKED, s_A_WHAT_TO_KNOW_ASKED);
    pyrt::xrefs::reg_access("compose.toknow", "BEFORE_YOU_AGREE", g_BEFORE_YOU_AGREE, s_BEFORE_YOU_AGREE);
    pyrt::xrefs::reg_access("compose.toknow", "THE_WORDS_THAT_END_THE_THING", g_THE_WORDS_THAT_END_THE_THING, s_THE_WORDS_THAT_END_THE_THING);
    pyrt::xrefs::reg_access("compose.toknow", "WHAT_THE_BOT_CAN_SAY_ABOUT_IT", g_WHAT_THE_BOT_CAN_SAY_ABOUT_IT, s_WHAT_THE_BOT_CAN_SAY_ABOUT_IT);
    pyrt::xrefs::reg_access("compose.toknow", "WHAT_TO_KNOW_FIRST", g_WHAT_TO_KNOW_FIRST, s_WHAT_TO_KNOW_FIRST);
    pyrt::xrefs::reg_access("compose.toknow", "pick", g_pick, s_pick);
}
