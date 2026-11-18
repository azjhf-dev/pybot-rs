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


thread_local! { static G_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_ME_AND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_MY_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOULD_I_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOULD_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_NOT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_NOT_SAID_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS_WITH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_ALONE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_HOUSE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_HOUSE_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEN_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEN_NOT_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ANSWER_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_LIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_AND_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.household: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ALONE() -> Py { ensure_init(); G_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALONE(v: Py) { ensure_init(); G_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_ME() -> Py { ensure_init(); G_JUST_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_ME(v: Py) { ensure_init(); G_JUST_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_ME_AND() -> Py { ensure_init(); G_JUST_ME_AND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_ME_AND(v: Py) { ensure_init(); G_JUST_ME_AND.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_SAID() -> Py { ensure_init(); G_NOT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_SAID(v: Py) { ensure_init(); G_NOT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_MY_OWN() -> Py { ensure_init(); G_ON_MY_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_MY_OWN(v: Py) { ensure_init(); G_ON_MY_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOULD_I_MORE() -> Py { ensure_init(); G_SHOULD_I_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOULD_I_MORE(v: Py) { ensure_init(); G_SHOULD_I_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOULD_MORE() -> Py { ensure_init(); G_SHOULD_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOULD_MORE(v: Py) { ensure_init(); G_SHOULD_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR() -> Py { ensure_init(); G_THEIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR(v: Py) { ensure_init(); G_THEIR.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_ALONE() -> Py { ensure_init(); G_THEIRS_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_ALONE(v: Py) { ensure_init(); G_THEIRS_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_NOT_SAID() -> Py { ensure_init(); G_THEIRS_NOT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_NOT_SAID(v: Py) { ensure_init(); G_THEIRS_NOT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_NOT_SAID_WHO() -> Py { ensure_init(); G_THEIRS_NOT_SAID_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_NOT_SAID_WHO(v: Py) { ensure_init(); G_THEIRS_NOT_SAID_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS_WITH() -> Py { ensure_init(); G_THEIRS_WITH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS_WITH(v: Py) { ensure_init(); G_THEIRS_WITH.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_ALONE_SAID() -> Py { ensure_init(); G_THEIR_ALONE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_ALONE_SAID(v: Py) { ensure_init(); G_THEIR_ALONE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_HOUSE() -> Py { ensure_init(); G_THEIR_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_HOUSE(v: Py) { ensure_init(); G_THEIR_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_HOUSE_ASK() -> Py { ensure_init(); G_THEIR_HOUSE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_HOUSE_ASK(v: Py) { ensure_init(); G_THEIR_HOUSE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_HOUSE_WHO() -> Py { ensure_init(); G_THEIR_HOUSE_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_HOUSE_WHO(v: Py) { ensure_init(); G_THEIR_HOUSE_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_THEN_ALONE() -> Py { ensure_init(); G_THEN_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEN_ALONE(v: Py) { ensure_init(); G_THEN_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEN_NOT_ALONE() -> Py { ensure_init(); G_THEN_NOT_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEN_NOT_ALONE(v: Py) { ensure_init(); G_THEN_NOT_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ANSWER_ASKED() -> Py { ensure_init(); G_THE_ANSWER_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ANSWER_ASKED(v: Py) { ensure_init(); G_THE_ANSWER_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_LIVES() -> Py { ensure_init(); G_WHO_LIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_LIVES(v: Py) { ensure_init(); G_WHO_LIVES.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_SOMEONE() -> Py { ensure_init(); G_WITH_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_SOMEONE(v: Py) { ensure_init(); G_WITH_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_AND_NO() -> Py { ensure_init(); G_YES_AND_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_AND_NO(v: Py) { ensure_init(); G_YES_AND_NO.with(|c| *c.borrow_mut() = v); }
pub fn g__OBJECTS() -> Py { ensure_init(); G__OBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OBJECTS(v: Py) { ensure_init(); G__OBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Who lives with the person: \"its just me and my daughter\", and the ask that follows it.\n\nThe store keeps the people a person names and the plans they make, but not the household itself,\nso the two plainest lines about it had nowhere to land:\n\n    i moved here in march\n    its just me and my daughter\n      -> \"Tell them as soon as it is certain, and keep it concrete: which room is theirs...\"\n\nThe line names a child and the word \"just\", and the only reading that fitted was the one for\ntelling a child about a move, so a person saying who they live with was told how to break the\nnews to their daughter. And the ask about it was answered by the store's place reader, which\nlooked for a town and found none:\n\n    who lives with us\n      -> \"You haven't mentioned a place yet.\"\n\nA friend answers both from what they have been told, and asks when they have not been told.\n\nThe household is filed on the chat's own memory as `memory[\"household\"]`, so every module can see\nit: {\"people\": [\"daughter\"], \"alone\": False} or {\"people\": [], \"alone\": True}. Somebody else\nliving alone is filed beside it as `memory[\"their_households\"]` ({\"mum\": True}), because \"my mum\nlives on her own\" is a fact about her and \"does she live on her own\" is the ask about it. Only the shape\nthat says who is at home is read - \"its just me and my daughter\", \"the two of us\", \"just me\" -\nbecause \"i live with my mum\" is also a place (\"i live with my mum in leeds\") and the store reads\nthat one.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.WHO_LIVES"); }
    s_WHO_LIVES(stdlib_call("re", "compile", &[py_str("\\bwho (?:else )?(?:lives|live|living|is living) (?:with (?:you|u|us|me)|there|here|at home)\\b|\\bwho (?:do|does) (?:you|u) live with\\b|\\bwho am i living with\\b|\\bwho (?:do|did) i live with\\b|\\banyone (?:else )?(?:live|living|lives) (?:with (?:you|u|us|me)|there|here|at home)\\b|\\banybody (?:else )?(?:live|living|lives) (?:with (?:you|u|us|me)|there|here)\\b|\\bdoes anyone (?:else )?live with (?:you|u)\\b|\\bwho(?:'s| is| s) (?:at home|there with you|living there)\\b|\\bwho else is (?:at home|there)\\b|\\bhow many of (?:you|us) (?:are|is) there\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.JUST_ME_AND"); }
    s_JUST_ME_AND(stdlib_call("re", "compile", &[py_str("^(?:well |ok |okay |so |and )*(?:its |it's |it is )?(?:just |only )?(?:me|us) (?:and|&) (?:my |our )?(?P<who>[a-z]+(?: [a-z]+)?)(?: and (?:my |our )?(?P<more>[a-z]+(?: [a-z]+)?))?(?: at home| living here| here)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.JUST_ME"); }
    s_JUST_ME(stdlib_call("re", "compile", &[py_str("^(?:well |ok |okay |so |and )*(?:its |it's |it is )?(?:just |only )?(?:me|the two of us|the three of us|us)(?: at home| here| on my own)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.ON_MY_OWN"); }
    s_ON_MY_OWN(stdlib_call("re", "compile", &[py_str("\\bi (?:live|am living|'m living|m living) (?:on my own|by myself|alone)\\b|\\bi (?:live|'m|am) (?:on my own|by myself)\\b|\\bliving alone\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.SHOULD_I_MORE"); }
    s_SHOULD_I_MORE(stdlib_call("re", "compile", &[py_str("\\bshould i\\s+(?:\\w+\\s+){0,2}(?:more|more often|again|every day|every week)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.YES_AND_NO"); }
    s_YES_AND_NO(stdlib_call("re", "compile", &[py_str("(?:yes|yeah|yep|yup|she does|he does|they do|she lives alone|he lives alone|no|nope|not really|she doesnt|she doesn't|he doesnt|he doesn't|they dont|they don't|she isnt|she isn't|he isnt|he isn't|they arent|they aren't)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIR_HOUSE_ASK"); }
    s_THEIR_HOUSE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|is|are)\\s+(?P<who>she|he|they|your\\s+[a-z]+)(?:\\s+still)?\\s+(?:live|living|be\\s+living)\\b[^?]*?\\b(?:alone|on\\s+(?:her|his|their)\\s+own|by\\s+(?:her|him|their)self)\\b|\\b(?:is|are)\\s+(?P<who2>she|he|they|your\\s+[a-z]+)(?:\\s+still)?\\s+(?:on\\s+(?:her|his|their)\\s+own|alone|by\\s+(?:her|him|their)self)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIR_ALONE_SAID"); }
    s_THEIR_ALONE_SAID(stdlib_call("re", "compile", &[py_str("\\bmy (?P<who>mum|mom|mam|dad|father|mother|gran|granny|nan|nana|grandma|grandad|grandpa|sister|brother|aunt|uncle|cousin|friend|neighbour|neighbor|wife|husband|partner)\\s+(?:lives|living|is living|'s living|s living)\\s+(?:on (?:her|his|their) own|alone|by (?:her|him|their)self)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIR_HOUSE_WHO"); }
    s_THEIR_HOUSE_WHO(stdlib_call("re", "compile", &[py_str("\\bwho (?:does|do) (?P<who>she|he|they|your\\s+[a-z]+)(?:\\s+actually)?\\s+live with\\b|\\bwho (?:is|'s|s) (?P<who2>she|he|they|your\\s+[a-z]+) living with\\b|\\b(?:does|do) (?:anyone|anybody|someone|somebody)(?:\\s+else)?\\s+live with (?P<who3>her|him|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIR"); }
    s_THEIR(py_str("{who}"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.WITH_SOMEONE"); }
    s_WITH_SOMEONE(py_tuple(vec![py_str("As far as you've told me, it's you and {who}."), py_str("You and {who} — that's who you've mentioned."), py_str("Just you and {who}, the way you told me.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.ALONE"); }
    s_ALONE(py_tuple(vec![py_str("On your own, you told me."), py_str("Just you, then — I'm glad of the company."), py_str("You live on your own, as far as I know.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.NOT_SAID"); }
    s_NOT_SAID(py_tuple(vec![py_str("You haven't said who lives with you — who's there?"), py_str("I don't know that yet. Who do you live with?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIR_HOUSE"); }
    s_THEIR_HOUSE(py_tuple(vec![py_str("Just you and {who}, then — noted."), py_str("You and {who}, then. What's an average evening like there?"), py_str("{Who} and you, then — that's the household.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIRS_NOT_SAID"); }
    s_THEIRS_NOT_SAID(py_tuple(vec![py_str("I can't tell from here — but if {who} is living alone, a standing call or visit beats a one-off grand gesture."), py_str("You'd know better than me. If {who} is living alone and it's getting harder, the council's free needs assessment is the first thing to ask about."), py_str("I don't know that one. If you're worried about {who} living alone, it's the small regular contact that helps most.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIRS_NOT_SAID_WHO"); }
    s_THEIRS_NOT_SAID_WHO(py_tuple(vec![py_str("I don't know — you haven't told me who lives with {who}."), py_str("Not something you've said. If it's company for {who} you're wondering about, a weekly class or club is the usual way in.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.SHOULD_MORE"); }
    s_SHOULD_MORE(py_tuple(vec![py_str("Little and often beats long and rare: a short call on the same day each week is easier to keep than a long one whenever there is time, and it is the one that gets counted on."), py_str("Yes — the regular short one does more than the long catch-up. Put it in the diary for the same day each week and let it be dull; dull is what makes it last.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEN_ALONE"); }
    s_THEN_ALONE(py_tuple(vec![py_str("Then a standing time to ring {who} is worth more than a long catch-up now and then — the same day each week, ten minutes, and it becomes a fixture rather than a good intention."), py_str("That is the hard part, the ordinary week on their own. A short call at the same time each week does more than a long one whenever it can be fitted in.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEN_NOT_ALONE"); }
    s_THEN_NOT_ALONE(py_tuple(vec![py_str("That is good — then it is the ordinary days that want the visits rather than the occasions, and a standing one is the easiest kind to keep."), py_str("Good. Then what helps most is turning up on a dull Tuesday rather than a birthday — the ordinary days are the ones that get long.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIRS_WITH"); }
    s_THEIRS_WITH(py_tuple(vec![py_str("You said so yourself — someone lives with {who}."), py_str("You told me — {who} has company.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THEIRS_ALONE"); }
    s_THEIRS_ALONE(py_tuple(vec![py_str("You told me — {who} lives alone."), py_str("You said so yourself: {who} lives alone.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household._OBJECTS"); }
    s__OBJECTS(py_dict(vec![(py_str("she"), py_str("her")), (py_str("he"), py_str("him")), (py_str("they"), py_str("them"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.household.THE_ANSWER_ASKED"); }
    s_THE_ANSWER_ASKED(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|is|are) (?P<who>she|he|they|your\\s+[a-z]+)(?:\\s+still)?\\s+(?:live|living|alone|on (?:her|his|their) own)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _who_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut said: Py = Py::None;
    let _ = py_str("The relation as it goes back to them: \"daughter\" -> \"your daughter\".");
    said = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    return op_add(&py_str("your "), &said.clone());
    Py::None
}
pub fn _who_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut alone: Py = Py::None;
    let mut held: Py = Py::None;
    let mut name: Py = Py::None;
    let mut only: Py = Py::None;
    let mut person: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The person an ask about somebody else is about: (\"your mum\", \"mum\") or (\"she\", \"\").\n\n    A line that names the relation (\"is your dad on his own\") says who it means. A bare \"she\" is\n    the person the chat was last about, which the store keeps in `memory[\"others\"]`.\n    ");
    said = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[]), "lower", &[]);
    if py_truthy(&mcall(&said.clone(), "startswith", &[py_str("your ")])) {
    return py_tuple(vec![said.clone(), slice_or_raise(&said.clone(), &Py::Int(5), &Py::None, &Py::None)]);
    }
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("others")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    person = __item.clone();
    name = if py_truthy(&call_builtin("isinstance", &[person.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&person.clone(), "get", &[py_str("who")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[name.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&name.clone(), "strip", &[]) } }) {
    name = mcall(&mcall(&name.clone(), "strip", &[]), "lower", &[]);
    return py_tuple(vec![op_add(&py_str("your "), &name.clone()), name.clone()]);
    }
    }
    }
    alone = _their_households(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[alone.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    only = call_builtin("next", &[call_builtin("iter", &[alone.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return py_tuple(vec![op_add(&py_str("your "), &call_builtin("str", &[only.clone()]).unwrap_or_else(|e| exc::raise(e))), call_builtin("str", &[only.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return py_tuple(vec![said.clone(), py_str("")]);
    Py::None
}
pub fn _who_held(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let _ = py_str("The person as the object of a sentence: \"she\" -> \"her\", \"your mum\" -> \"your mum\".");
    return if py_truthy(&key.clone()) { who.clone() } else { mcall(&g__OBJECTS(), "get", &[who.clone(), who.clone()]) };
    Py::None
}
pub fn _the_ask_it_answers(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The question about this on the last turn, if this line answers it.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_YES_AND_NO(), "match", &[said.clone()])))) {
    return Py::None;
    }
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    asked = mcall(&memory.clone(), "get", &[py_str("household_asked")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[asked.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&asked.clone(), "get", &[py_str("ask")]), &py_str("alone"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("household"))) } } } }) {
    return asked.clone();
    }
    found = mcall(&g_THE_ANSWER_ASKED(), "search", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_who_asked(&[mcall(&found.clone(), "group", &[py_str("who")]), memory.clone()]), 2, None);
    who = __unpk1[0].clone();
    key = __unpk1[1].clone();
    return py_dict(vec![(py_str("who"), who.clone()), (py_str("key"), key.clone()), (py_str("ask"), py_str("alone"))]);
    Py::None
}
pub fn _their_households(args: &[Py]) -> Py {
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
    let _ = py_str("Who else the chat says lives alone, by relation: {\"mum\": True}.");
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("their_households")]) } else { Py::None };
    return if py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { held.clone() } else { py_dict(vec![]) };
    Py::None
}
pub fn _held(args: &[Py]) -> Py {
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
    let mut people: Py = Py::None;
    let mut person: Py = Py::None;
    let _ = py_str("The household the chat has been told about, or None.");
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("household")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    people = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&held.clone(), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut person: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[person.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { person.clone() } }) {  __out.push(person.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&people.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&held.clone(), "get", &[py_str("alone")]))) } }) {
    return Py::None;
    }
    return py_dict(vec![(py_str("people"), people.clone()), (py_str("alone"), call_builtin("bool", &[mcall(&held.clone(), "get", &[py_str("alone")])]).unwrap_or_else(|e| exc::raise(e)))]);
    Py::None
}
pub fn observe(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut more: Py = Py::None;
    let mut people: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("File who lives with them, and say nothing: the reply is this module's `respond`.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!,")]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_THEIR_ALONE_SAID(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    theirs = __item.clone();
    kept = mcall(&memory.clone(), "setdefault", &[py_str("their_households"), py_dict(vec![])]);
    if py_truthy(&call_builtin("isinstance", &[kept.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&kept.clone(), &mcall(&mcall(&theirs.clone(), "group", &[py_str("who")]), "lower", &[]), Py::Bool(true));
    }
    }
    if py_truthy(&mcall(&g_ON_MY_OWN(), "search", &[said.clone()])) {
    let _ = py_setitem(&memory.clone(), &py_str("household"), py_dict(vec![(py_str("people"), py_list(vec![])), (py_str("alone"), Py::Bool(true)), (py_str("said"), said.clone())]));
    return Py::None;
    }
    found = mcall(&g_JUST_ME_AND(), "match", &[said.clone()]);
    if py_truthy(&found.clone()) {
    people = py_list(vec![_who_said(&[mcall(&found.clone(), "group", &[py_str("who")])])]);
    more = mcall(&found.clone(), "group", &[py_str("more")]);
    if py_truthy(&more.clone()) {
    second = _who_said(&[more.clone()]);
    if py_truthy(&{ let __b = second.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&people.clone(), &second.clone())) } }) {
    let _ = mcall(&people.clone(), "append", &[second.clone()]);
    }
    }
    let _ = py_setitem(&memory.clone(), &py_str("household"), py_dict(vec![(py_str("people"), people.clone()), (py_str("alone"), Py::Bool(false)), (py_str("said"), said.clone())]));
    return Py::None;
    }
    if py_truthy(&mcall(&g_JUST_ME(), "match", &[said.clone()])) {
    let _ = py_setitem(&memory.clone(), &py_str("household"), py_dict(vec![(py_str("people"), py_list(vec![])), (py_str("alone"), Py::Bool(true)), (py_str("said"), said.clone())]));
    }
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
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut last: Py = Py::None;
    let mut named: Py = Py::None;
    let mut people: Py = Py::None;
    let mut said: Py = Py::None;
    let mut them: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut who: Py = Py::None;
    let mut yes: Py = Py::None;
    let _ = py_str("The ask, or the household said back and nothing else, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!,")]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    asked = _the_ask_it_answers(&[said.clone(), memory.clone()]);
    if py_truthy(&asked.clone()) {
    let _ = mcall(&memory.clone(), "pop", &[py_str("household_asked"), Py::None]);
    yes = Py::Bool(!py_truthy(&{ let __b = mcall(&said.clone(), "startswith", &[py_tuple(vec![py_str("no"), py_str("nope"), py_str("not")])]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" doesnt"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" doesn't"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" dont"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" don't"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" isnt"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" isn't"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str(" arent"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str(" aren't"))) } } } } } } } } } } } } } } } }));
    kept = mcall(&memory.clone(), "setdefault", &[py_str("their_households"), py_dict(vec![])]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kept.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&asked.clone(), "get", &[py_str("key")]) } }) {
    let _ = py_setitem(&kept.clone(), &geti(&asked.clone(), &py_str("key")), yes.clone());
    }
    who = _who_held(&[{ let __b = mcall(&asked.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("them") } }, { let __b = mcall(&asked.clone(), "get", &[py_str("key")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&yes.clone()) { g_THEN_ALONE() } else { g_THEN_NOT_ALONE() }); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    if py_truthy(&mcall(&g_SHOULD_I_MORE(), "search", &[said.clone()])) {
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("household")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SHOULD_MORE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    }
    found = mcall(&g_THEIR_HOUSE_WHO(), "search", &[said.clone()]);
    if py_truthy(&found.clone()) {
    named = { let __b = mcall(&found.clone(), "group", &[py_str("who")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "group", &[py_str("who2")]) } };
    if py_truthy(&named.clone()) {
    let __unpk1 = py_unpack(&_who_asked(&[named.clone(), memory.clone()]), 2, None);
    who = __unpk1[0].clone();
    key = __unpk1[1].clone();
    them = _who_held(&[who.clone(), key.clone()]);
    } else {
    them = mcall(&found.clone(), "group", &[py_str("who3")]);
    }
    let _ = py_setitem(&memory.clone(), &py_str("household_asked"), py_dict(vec![(py_str("who"), if py_truthy(&named.clone()) { who.clone() } else { them.clone() }), (py_str("key"), if py_truthy(&named.clone()) { key.clone() } else { py_str("") }), (py_str("ask"), py_str("who"))]));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), them.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_NOT_SAID_WHO()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    found = mcall(&g_THEIR_HOUSE_ASK(), "search", &[said.clone()]);
    if py_truthy(&found.clone()) {
    let __unpk2 = py_unpack(&_who_asked(&[{ let __b = mcall(&found.clone(), "group", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("who2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, memory.clone()]), 2, None);
    who = __unpk2[0].clone();
    key = __unpk2[1].clone();
    held = if py_truthy(&key.clone()) { mcall(&_their_households(&[memory.clone()]), "get", &[key.clone()]) } else { Py::None };
    if py_truthy(&Py::Bool(py_is(&held.clone(), &Py::Bool(true)))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_ALONE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_is(&held.clone(), &Py::Bool(false)))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), _who_held(&[who.clone(), key.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_WITH()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    let _ = py_setitem(&memory.clone(), &py_str("household_asked"), py_dict(vec![(py_str("who"), who.clone()), (py_str("key"), key.clone()), (py_str("ask"), py_str("alone"))]));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS_NOT_SAID()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    if py_truthy(&mcall(&g_WHO_LIVES(), "search", &[said.clone()])) {
    held = _held(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&held.clone(), &Py::None))) {
    if py_truthy(&pyrt::xrefs::xcall("memory.counts", "answer", &[said.clone(), memory.clone()])) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NOT_SAID()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&geti(&held.clone(), &py_str("alone"))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ALONE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    people = geti(&held.clone(), &py_str("people"));
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[people.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    who = geti(&people.clone(), &Py::Int(0));
    } else {
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[people.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    who = op_add(&op_add(&geti(&people.clone(), &Py::Int(0)), &py_str(" and ")), &geti(&people.clone(), &Py::Int(1)));
    } else {
    who = op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&people.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&people.clone(), &op_neg(&Py::Int(1))));
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WITH_SOMEONE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    found = { let __b = mcall(&g_JUST_ME_AND(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_JUST_ME(), "match", &[said.clone()]) } };
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } })) } }) {
    return Py::None;
    }
    if py_truthy(&found.clone()) {
    who = _who_said(&[mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("who")])]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone()), ("Who".to_string(), op_add(&mcall(&slice_or_raise(&who.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&who.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIR_HOUSE()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("This line, because the household is the whole of it.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!,")]);
    if py_truthy(&mcall(&g_WHO_LIVES(), "search", &[said.clone()])) {
    return Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.counts", "answer", &[said.clone(), memory.clone()])));
    }
    if py_truthy(&{ let __b = mcall(&g_THEIR_HOUSE_ASK(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIR_HOUSE_WHO(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_ask_it_answers(&[said.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_SHOULD_I_MORE(), "search", &[said.clone()])) {
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    return Py::Bool(py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("household")));
    }
    found = { let __b = mcall(&g_JUST_ME_AND(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_JUST_ME(), "match", &[said.clone()]) } };
    return { let __b = call_builtin("bool", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("who")])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.household".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALONE" => Some(g_ALONE()),
            "JUST_ME" => Some(g_JUST_ME()),
            "JUST_ME_AND" => Some(g_JUST_ME_AND()),
            "NOT_SAID" => Some(g_NOT_SAID()),
            "ON_MY_OWN" => Some(g_ON_MY_OWN()),
            "SHOULD_I_MORE" => Some(g_SHOULD_I_MORE()),
            "SHOULD_MORE" => Some(g_SHOULD_MORE()),
            "THEIR" => Some(g_THEIR()),
            "THEIRS_ALONE" => Some(g_THEIRS_ALONE()),
            "THEIRS_NOT_SAID" => Some(g_THEIRS_NOT_SAID()),
            "THEIRS_NOT_SAID_WHO" => Some(g_THEIRS_NOT_SAID_WHO()),
            "THEIRS_WITH" => Some(g_THEIRS_WITH()),
            "THEIR_ALONE_SAID" => Some(g_THEIR_ALONE_SAID()),
            "THEIR_HOUSE" => Some(g_THEIR_HOUSE()),
            "THEIR_HOUSE_ASK" => Some(g_THEIR_HOUSE_ASK()),
            "THEIR_HOUSE_WHO" => Some(g_THEIR_HOUSE_WHO()),
            "THEN_ALONE" => Some(g_THEN_ALONE()),
            "THEN_NOT_ALONE" => Some(g_THEN_NOT_ALONE()),
            "THE_ANSWER_ASKED" => Some(g_THE_ANSWER_ASKED()),
            "WHO_LIVES" => Some(g_WHO_LIVES()),
            "WITH_SOMEONE" => Some(g_WITH_SOMEONE()),
            "YES_AND_NO" => Some(g_YES_AND_NO()),
            "_OBJECTS" => Some(g__OBJECTS()),
            "_held" => Some(py_func("_held", |args: &[Py]| _held(args))),
            "_the_ask_it_answers" => Some(py_func("_the_ask_it_answers", |args: &[Py]| _the_ask_it_answers(args))),
            "_their_households" => Some(py_func("_their_households", |args: &[Py]| _their_households(args))),
            "_who_asked" => Some(py_func("_who_asked", |args: &[Py]| _who_asked(args))),
            "_who_held" => Some(py_func("_who_held", |args: &[Py]| _who_held(args))),
            "_who_said" => Some(py_func("_who_said", |args: &[Py]| _who_said(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "pick" => Some(g_pick()),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            "counts" => Some(pyrt::xrefs::xmodule("memory.counts")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALONE" => s_ALONE(v),
            "JUST_ME" => s_JUST_ME(v),
            "JUST_ME_AND" => s_JUST_ME_AND(v),
            "NOT_SAID" => s_NOT_SAID(v),
            "ON_MY_OWN" => s_ON_MY_OWN(v),
            "SHOULD_I_MORE" => s_SHOULD_I_MORE(v),
            "SHOULD_MORE" => s_SHOULD_MORE(v),
            "THEIR" => s_THEIR(v),
            "THEIRS_ALONE" => s_THEIRS_ALONE(v),
            "THEIRS_NOT_SAID" => s_THEIRS_NOT_SAID(v),
            "THEIRS_NOT_SAID_WHO" => s_THEIRS_NOT_SAID_WHO(v),
            "THEIRS_WITH" => s_THEIRS_WITH(v),
            "THEIR_ALONE_SAID" => s_THEIR_ALONE_SAID(v),
            "THEIR_HOUSE" => s_THEIR_HOUSE(v),
            "THEIR_HOUSE_ASK" => s_THEIR_HOUSE_ASK(v),
            "THEIR_HOUSE_WHO" => s_THEIR_HOUSE_WHO(v),
            "THEN_ALONE" => s_THEN_ALONE(v),
            "THEN_NOT_ALONE" => s_THEN_NOT_ALONE(v),
            "THE_ANSWER_ASKED" => s_THE_ANSWER_ASKED(v),
            "WHO_LIVES" => s_WHO_LIVES(v),
            "WITH_SOMEONE" => s_WITH_SOMEONE(v),
            "YES_AND_NO" => s_YES_AND_NO(v),
            "_OBJECTS" => s__OBJECTS(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.household", module_obj);
    pyrt::xrefs::reg_fn("memory.household", "_who_said", py_func("_who_said", |args: &[Py]| _who_said(args)));
    pyrt::xrefs::reg_fn("memory.household", "_who_asked", py_func("_who_asked", |args: &[Py]| _who_asked(args)));
    pyrt::xrefs::reg_fn("memory.household", "_who_held", py_func("_who_held", |args: &[Py]| _who_held(args)));
    pyrt::xrefs::reg_fn("memory.household", "_the_ask_it_answers", py_func("_the_ask_it_answers", |args: &[Py]| _the_ask_it_answers(args)));
    pyrt::xrefs::reg_fn("memory.household", "_their_households", py_func("_their_households", |args: &[Py]| _their_households(args)));
    pyrt::xrefs::reg_fn("memory.household", "_held", py_func("_held", |args: &[Py]| _held(args)));
    pyrt::xrefs::reg_fn("memory.household", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("memory.household", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_fn("memory.household", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_access("memory.household", "ALONE", g_ALONE, s_ALONE);
    pyrt::xrefs::reg_access("memory.household", "JUST_ME", g_JUST_ME, s_JUST_ME);
    pyrt::xrefs::reg_access("memory.household", "JUST_ME_AND", g_JUST_ME_AND, s_JUST_ME_AND);
    pyrt::xrefs::reg_access("memory.household", "NOT_SAID", g_NOT_SAID, s_NOT_SAID);
    pyrt::xrefs::reg_access("memory.household", "ON_MY_OWN", g_ON_MY_OWN, s_ON_MY_OWN);
    pyrt::xrefs::reg_access("memory.household", "SHOULD_I_MORE", g_SHOULD_I_MORE, s_SHOULD_I_MORE);
    pyrt::xrefs::reg_access("memory.household", "SHOULD_MORE", g_SHOULD_MORE, s_SHOULD_MORE);
    pyrt::xrefs::reg_access("memory.household", "THEIR", g_THEIR, s_THEIR);
    pyrt::xrefs::reg_access("memory.household", "THEIRS_ALONE", g_THEIRS_ALONE, s_THEIRS_ALONE);
    pyrt::xrefs::reg_access("memory.household", "THEIRS_NOT_SAID", g_THEIRS_NOT_SAID, s_THEIRS_NOT_SAID);
    pyrt::xrefs::reg_access("memory.household", "THEIRS_NOT_SAID_WHO", g_THEIRS_NOT_SAID_WHO, s_THEIRS_NOT_SAID_WHO);
    pyrt::xrefs::reg_access("memory.household", "THEIRS_WITH", g_THEIRS_WITH, s_THEIRS_WITH);
    pyrt::xrefs::reg_access("memory.household", "THEIR_ALONE_SAID", g_THEIR_ALONE_SAID, s_THEIR_ALONE_SAID);
    pyrt::xrefs::reg_access("memory.household", "THEIR_HOUSE", g_THEIR_HOUSE, s_THEIR_HOUSE);
    pyrt::xrefs::reg_access("memory.household", "THEIR_HOUSE_ASK", g_THEIR_HOUSE_ASK, s_THEIR_HOUSE_ASK);
    pyrt::xrefs::reg_access("memory.household", "THEIR_HOUSE_WHO", g_THEIR_HOUSE_WHO, s_THEIR_HOUSE_WHO);
    pyrt::xrefs::reg_access("memory.household", "THEN_ALONE", g_THEN_ALONE, s_THEN_ALONE);
    pyrt::xrefs::reg_access("memory.household", "THEN_NOT_ALONE", g_THEN_NOT_ALONE, s_THEN_NOT_ALONE);
    pyrt::xrefs::reg_access("memory.household", "THE_ANSWER_ASKED", g_THE_ANSWER_ASKED, s_THE_ANSWER_ASKED);
    pyrt::xrefs::reg_access("memory.household", "WHO_LIVES", g_WHO_LIVES, s_WHO_LIVES);
    pyrt::xrefs::reg_access("memory.household", "WITH_SOMEONE", g_WITH_SOMEONE, s_WITH_SOMEONE);
    pyrt::xrefs::reg_access("memory.household", "YES_AND_NO", g_YES_AND_NO, s_YES_AND_NO);
    pyrt::xrefs::reg_access("memory.household", "_OBJECTS", g__OBJECTS, s__OBJECTS);
    pyrt::xrefs::reg_access("memory.household", "pick", g_pick, s_pick);
}
