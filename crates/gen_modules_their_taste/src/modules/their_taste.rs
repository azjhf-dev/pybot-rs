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


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DISLIKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DISLIKE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAN_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEAVY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__I_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIKES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIKE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACEHOLDER_OPENINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STORY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THEIRS_BUT_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TYPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__obstacle: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON() -> Py { ensure_init(); G__A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON(v: Py) { ensure_init(); G__A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__DISLIKES() -> Py { ensure_init(); G__DISLIKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DISLIKES(v: Py) { ensure_init(); G__DISLIKES.with(|c| *c.borrow_mut() = v); }
pub fn g__DISLIKE_VERBS() -> Py { ensure_init(); G__DISLIKE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DISLIKE_VERBS(v: Py) { ensure_init(); G__DISLIKE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__FAN_OF() -> Py { ensure_init(); G__FAN_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAN_OF(v: Py) { ensure_init(); G__FAN_OF.with(|c| *c.borrow_mut() = v); }
pub fn g__HEAVY() -> Py { ensure_init(); G__HEAVY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEAVY(v: Py) { ensure_init(); G__HEAVY.with(|c| *c.borrow_mut() = v); }
pub fn g__I_SAY() -> Py { ensure_init(); G__I_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__I_SAY(v: Py) { ensure_init(); G__I_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD() -> Py { ensure_init(); G__LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); G__LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__LIKES() -> Py { ensure_init(); G__LIKES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIKES(v: Py) { ensure_init(); G__LIKES.with(|c| *c.borrow_mut() = v); }
pub fn g__LIKE_VERBS() -> Py { ensure_init(); G__LIKE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIKE_VERBS(v: Py) { ensure_init(); G__LIKE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TASTE() -> Py { ensure_init(); G__NOT_A_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TASTE(v: Py) { ensure_init(); G__NOT_A_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_THING() -> Py { ensure_init(); G__NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING(v: Py) { ensure_init(); G__NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACEHOLDER_OPENINGS() -> Py { ensure_init(); G__PLACEHOLDER_OPENINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACEHOLDER_OPENINGS(v: Py) { ensure_init(); G__PLACEHOLDER_OPENINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__STORY() -> Py { ensure_init(); G__STORY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STORY(v: Py) { ensure_init(); G__STORY.with(|c| *c.borrow_mut() = v); }
pub fn g__THEIRS_BUT_NOT_A_THING() -> Py { ensure_init(); G__THEIRS_BUT_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THEIRS_BUT_NOT_A_THING(v: Py) { ensure_init(); G__THEIRS_BUT_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__THING() -> Py { ensure_init(); G__THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THING(v: Py) { ensure_init(); G__THING.with(|c| *c.borrow_mut() = v); }
pub fn g__TONE() -> Py { ensure_init(); G__TONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TONE(v: Py) { ensure_init(); G__TONE.with(|c| *c.borrow_mut() = v); }
pub fn g__TYPED() -> Py { ensure_init(); G__TYPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TYPED(v: Py) { ensure_init(); G__TYPED.with(|c| *c.borrow_mut() = v); }
pub fn g__obstacle() -> Py { ensure_init(); G__obstacle.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__obstacle(v: Py) { ensure_init(); G__obstacle.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A taste of their own, said plainly, gets a reply that answers back.\n\nPeople say what they like and what they cannot stand in the middle of a conversation, usually as\na line on its own: \"i love thai food\", \"i really like jazz\", \"im not a fan of football\", \"i cant\nstand mushrooms\". No module reads them, so they reach the composer, and the composer hands the\nsentence back with a receipt on it:\n\n    i love thai food            -> \"Got it — you love thai food.\"\n    i like a girl at work       -> \"Noted: you like a girl at work.\"\n    i really like jazz          -> nothing at all from any module above the composer\n\nA friend does not hand a person their own sentence. They say what they think of the thing\n(\"Can't argue with thai food\"), and then ask the one thing only the person can answer: what got\nthem into it, how often they get it, what it is about it they like. That is all this module does,\nfor a like and for a dislike, because the two take different lines.\n\nOnly a whole line is one of these: the taste has to be the line and not the beginning of a story\n(\"i love thai food but it's so expensive to make\" is a story with a taste in it). A thing that is\na person (\"i like a girl at work\", \"i love you\"), a thing that is the bot's own, or a word that\nstands in for a thing (\"i love it here\", \"i like that\") is not a taste this can answer, and\nneither is a line about their own life going wrong (\"i hate my life\", \"i hate my job\" after \"my\nboss has it in for me\") - those keep whatever turn they have. A taste said straight after heavy\nnews is not a question back, either: the module stands down for the turn before.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    let __try1 = exc::catch(|| {
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
    s__obstacle(Py::None);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    s_PRIORITY(op_neg(&Py::Int(960)));
    s__LIKES(py_tuple(vec![py_str("Can't argue with {thing}. What do you like most?"), py_str("{Thing} - good taste. What is it about {thing} that you like?"), py_str("That is a good thing to like. What do you like most about {thing}?")]));
    s__DISLIKES(py_tuple(vec![py_str("{Thing} - fair enough. What is it you don't like about {thing}?"), py_str("Nobody is going to talk you round on {thing}, and nobody needs to. What is it that puts you off?"), py_str("Plenty of people would say the same about {thing}. What is it you cannot take about {thing}?")]));
    fn __data6() -> Py { py_tuple(vec![py_str("really like"), py_str("do like"), py_str("love"), py_str("like"), py_str("adore"), py_str("am mad about"), py_str("m mad about"), py_str("am into"), py_str("m into"), py_str("can't get enough of"), py_str("cant get enough of"), py_str("can not get enough of"), py_str("am a fan of"), py_str("m a fan of"), py_str("am obsessed with"), py_str("m obsessed with"), py_str("have always loved")]) }
    s__LIKE_VERBS(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("really don't like"), py_str("really dont like"), py_str("don't like"), py_str("dont like"), py_str("hate"), py_str("dislike"), py_str("can't stand"), py_str("cant stand"), py_str("can not stand"), py_str("am not a fan of"), py_str("m not a fan of"), py_str("am not keen on"), py_str("m not keen on"), py_str("have never liked"), py_str("have gone off")]) }
    s__DISLIKE_VERBS(__data7());
    s__FAN_OF(stdlib_call("re", "compile", &[py_str("(?:am|m)\\s+(?:a\\s+)?(?:\\w+\\s+){0,2}fan\\s+of"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TONE(stdlib_call("re", "compile", &[py_str("(?:\\s+(?:tbh|tho|though|really|honestly|actually|literally|so\\s+much|a\\s+lot|as\\s+well|too|now|anyway|i\\s+suppose|i\\s+guess|lol|ha|haha|btw|atm|lately|these\\s+days|at\\s+the\\s+moment|right\\s+now|still|again))+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LEAD(py_str("(?:(?:oh|god|god,|honestly|yeah|well|actually|man|christ|jesus|blimey)\\s+)*"));
    s__THING(stdlib_call("re", "compile", &[py_str("^(?P<thing>[a-z][a-z'\\-]*(?:\\s+[a-z][a-z'\\-]*){0,4})$")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data8() -> Py { py_tuple(vec![py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("these"), py_str("those"), py_str("him"), py_str("her"), py_str("you"), py_str("me"), py_str("us"), py_str("here"), py_str("there"), py_str("everything"), py_str("anything"), py_str("nothing"), py_str("everyone"), py_str("someone"), py_str("it all"), py_str("that one"), py_str("this one"), py_str("one"), py_str("ones"), py_str("things"), py_str("stuff"), py_str("life"), py_str("my life"), py_str("myself"), py_str("my job"), py_str("my work"), py_str("people"), py_str("the world"), py_str("the news"), py_str("him too"), py_str("her too"), py_str("your advice"), py_str("advice"), py_str("your help"), py_str("this recipe"), py_str("the recipe"), py_str("your recipes"), py_str("this place"), py_str("this app"), py_str("chatting to you"), py_str("talking to you"), py_str("you too"), py_str("you both"), py_str("your voice"), py_str("much more of this"), py_str("more of this"), py_str("much more"), py_str("any more of this"), py_str("it any more"), py_str("it anymore"), py_str("this any more"), py_str("this anymore")]) }
    s__NOT_A_THING(call_builtin("frozenset", &[__data8()]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:girl|boy|woman|man|guy|lad|lass|bloke|fella|someone|somebody|people|person|her|him|them|she|he|they)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THEIRS_BUT_NOT_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:mum|mom|mother|dad|father|son|daughter|kid|kids|child|children|wife|husband|partner|baby|gran|grandma|grandmother|nan|nana|grandad|grandpa|grandfather|aunt|uncle|cousin|sister|brother|parents|family|dog|cat|puppy|kitten|pets?|bird|horse|rabbit|hamster|boss|friend|mate|neighbours?|self|life|job|work|body|looks|health|mind|sleep)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_TASTE(stdlib_call("re", "compile", &[py_str("\\b(?:my life|myself|my job|my work|my body|my looks|the world|people|mondays?\\s+morning)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__I_SAY(py_str("^(?:i|i'm|i've)\\s+"));
    s__TYPED(py_tuple(vec![py_tuple(vec![py_str("im "), py_str("i am ")]), py_tuple(vec![py_str("i m "), py_str("i am ")]), py_tuple(vec![py_str("ive "), py_str("i have ")])]));
    s__STORY(stdlib_call("re", "compile", &[py_str("\\b(?:but|because|so|though|although|when|whenever|if|since|which|that|keeps?|makes?|gets?|goes?|go|went|got|costs?|cost|is|are|was|were|will|would|can|could|have|has|had|need|needs|want|wants|helps?|works?|tastes?|smells?)\\b|\\band\\s+(?:i|it|its|it's|they|we|you|he|she|there|thats|that's)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HEAVY(stdlib_call("re", "compile", &[py_str("\\b(?:ill|unwell|poorly|sick|dying|died|death|passed away|funeral|bereave\\w*|griev\\w*|cancer|chemo\\w*|radiotherap\\w*|tumou?r|dementia|alzheim\\w*|stroke|sepsis|surgery|operation|transplant|hospice|palliative|terminal|hospital|icu|ambulance|diagnos\\w*|bully\\w*|bullied|depress\\w*|suicid\\w*|self harm|miscarr\\w*|stillborn|addict\\w*|relapse\\w*|overdose\\w*|sectioned|in pain|suffering)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data9() -> Py { py_tuple(vec![py_str("noted"), py_str("got it"), py_str("i'm listening"), py_str("i am listening"), py_str("thanks for telling me"), py_str("that's noted"), py_str("thats noted"), py_str("ok."), py_str("okay."), py_str("i don't know enough"), py_str("i dont know enough"), py_str("that's beyond what i know"), py_str("thats beyond what i know"), py_str("i'm not sure about that one"), py_str("im not sure about that one")]) }
    s__PLACEHOLDER_OPENINGS(__data9());
}
pub fn _the_composer_would_answer(args: &[Py]) -> Py {
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
    let mut opening: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the composer has a line of its own for this line, rather than a placeholder.");
    let __try1 = exc::catch(|| {
    reply = py_call(&pyrt::xrefs::xget("compose.fallback", "reply_for"), &[text.clone(), memory.clone()]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PLACEHOLDER_OPENINGS()) { let mut opening: Py = __item.clone(); __out.push(mcall(&said.clone(), "startswith", &[opening.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_greeting_would_answer(args: &[Py]) -> Py {
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
    let mut greeting: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the band below this module has a line of its own, rather than a placeholder.\n\n    `fallback.reply_for` is not the whole floor. `greeting` sits one band down and has a reader of\n    its own for a taste which names the thing and asks about it in the person's own terms - \"Ah,\n    thai food. What got you into that?\", \"Your new flat - I can see why. What do you like most\n    about it?\", \"The sea, nice one. What is the draw for you?\". Asking only the composer meant a\n    tailored line was replaced by one of three templates: a receipt is dead, but a flat template\n    over a good line is worse, and it is the same mistake in the other direction.\n\n    The receipts it also gives (\"Noted: you love thai food\") are the placeholder openings above,\n    so those still stand this module up.\n    ");
    let __try1 = exc::catch(|| {
    greeting = pyrt::xrefs::xmodule("modules.greeting");
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try6 = exc::catch(|| {
    reply = mcall(&greeting.clone(), "respond", &[text.clone(), memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::Bool(false);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PLACEHOLDER_OPENINGS()) { let mut opening: Py = __item.clone(); __out.push(mcall(&said.clone(), "startswith", &[opening.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_line_before(args: &[Py]) -> Py {
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
    let _ = py_str("What the person said on the turn before this one, or \"\" on the first turn.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    return if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    Py::None
}
pub fn _the_thread_is_heavy(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let _ = py_str("True when the turn before this one was heavy news rather than ordinary talk.");
    before = _the_line_before(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_reads_as_heavy_news", &[before.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return call_builtin("bool", &[mcall(&g__HEAVY(), "search", &[before.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_plainly(args: &[Py]) -> Py {
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
    let mut changed: Py = Py::None;
    let mut proper: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut typed: Py = Py::None;
    let _ = py_str("The line without the throat-clearing and the trailing tone words.");
    said = clean.clone();
    let __iter1 = py_iter_or_raise(&g__TYPED());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    typed = __unpk2[0].clone();
    proper = __unpk2[1].clone();
    if py_truthy(&mcall(&said.clone(), "startswith", &[typed.clone()])) {
    said = op_add(&proper.clone(), &slice_or_raise(&said.clone(), &call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None));
    }
    }
    said = stdlib_call("re", "sub", &[op_add(&py_str("^"), &g__LEAD()), py_str(""), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    changed = Py::Bool(true);
    while py_truthy(&changed.clone()) {
    stripped = mcall(&g__TONE(), "sub", &[py_str(""), said.clone()]);
    changed = Py::Bool(!py_eq(&stripped.clone(), &said.clone()));
    said = stripped.clone();
    }
    return mcall(&said.clone(), "strip", &[]);
    Py::None
}
pub fn _a_taste(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut head: Py = Py::None;
    let mut held: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut verbs: Py = Py::None;
    let _ = py_str("(kind, the thing as they said it) for a line that is a taste on its own, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("like"), g__LIKE_VERBS(), g__LIKES()]), py_tuple(vec![py_str("dislike"), g__DISLIKE_VERBS(), g__DISLIKES()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    kind = __unpk2[0].clone();
    verbs = __unpk2[1].clone();
    pool = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&op_add(&verbs.clone(), &py_tuple(vec![gattr(&g__FAN_OF(), "pattern")])));
    for __item in __iter3 {
    verb = __item.clone();
    head = op_add(&op_add(&g__I_SAY(), &if py_truthy(&Py::Bool(py_is(&verb.clone(), &gattr(&g__FAN_OF(), "pattern")))) { verb.clone() } else { stdlib_call("re", "escape", &[verb.clone()]).unwrap_or_else(|e| exc::raise(e)) }), &py_str("\\s+"));
    match_ = stdlib_call("re", "match", &[head.clone(), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    rest = mcall(&slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { mcall(&g__STORY(), "search", &[rest.clone()]) } }) {
    continue;
    }
    thing = mcall(&g__THING(), "match", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    continue;
    }
    rest = mcall(&thing.clone(), "group", &[py_str("thing")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__NOT_A_THING(), &rest.clone())); if py_truthy(&__b) { __b } else { mcall(&g__A_PERSON(), "search", &[rest.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("dislike"))); if !py_truthy(&__b) { __b } else { mcall(&g__NOT_A_TASTE(), "search", &[rest.clone()]) } }) {
    continue;
    }
    first = geti(&mcall(&rest.clone(), "split", &[]), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("your"), py_str("our"), py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("here"), py_str("there")]), &first.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&first.clone(), &py_str("my")))) {
    held = mcall(&slice_or_raise(&rest.clone(), &Py::Int(3), &Py::None, &Py::None), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&held.clone())); if py_truthy(&__b) { __b } else { mcall(&g__THEIRS_BUT_NOT_A_THING(), "search", &[held.clone()]) } }) {
    continue;
    }
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("^(?:your|this|that|these|those)\\s+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    return py_tuple(vec![kind.clone(), rest.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_thing_said(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The thing in the person's own words, with their \"my\" turned round.");
    words = mcall(&thing.clone(), "split", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("my"))) } }) {
    let _ = py_setitem(&words.clone(), &Py::Int(0), py_str("your"));
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _pick(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let _ = py_str("One line of the pool, seeded by the person's line and never the one just given.\n\n    The composer's own pick is used rather than the first unseen line, because a pool of three\n    read out in order means every person who says a taste on their first turn gets the same\n    sentence: eleven deck chats changed to \"Can't argue with ...\", word for word.\n    ");
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), { let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    Py::None
}
pub fn _the_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut kind: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    thing = _the_thing_said(&[thing.clone()]);
    pool = if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("like")))) { g__LIKES() } else { g__DISLIKES() };
    line = _pick(&[pool.clone(), memory.clone(), mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    return mcall(&mcall(&line.clone(), "replace", &[py_str("{Thing}"), op_add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))]), "replace", &[py_str("{thing}"), thing.clone()]);
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut taste: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The line, when it is a taste on its own and the thread is not a heavy one.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = _said_plainly(&[mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    taste = _a_taste(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&taste.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    memory = py_dict(vec![]);
    }
    if py_truthy(&_the_thread_is_heavy(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_is(&g__obstacle(), &Py::None))) {
    let __try1 = exc::catch(|| {
    if py_truthy(&pyrt::xrefs::xcall("memory.obstacle", "answer_for", &[text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
    }
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    if py_truthy(&_the_composer_would_answer(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&_the_greeting_would_answer(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    let __unpk6 = py_unpack(&taste.clone(), 2, None);
    kind = __unpk6[0].clone();
    thing = __unpk6[1].clone();
    return _the_reply(&[kind.clone(), thing.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.their_taste".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "_A_PERSON" => Some(g__A_PERSON()),
            "_DISLIKES" => Some(g__DISLIKES()),
            "_DISLIKE_VERBS" => Some(g__DISLIKE_VERBS()),
            "_FAN_OF" => Some(g__FAN_OF()),
            "_HEAVY" => Some(g__HEAVY()),
            "_I_SAY" => Some(g__I_SAY()),
            "_LEAD" => Some(g__LEAD()),
            "_LIKES" => Some(g__LIKES()),
            "_LIKE_VERBS" => Some(g__LIKE_VERBS()),
            "_NOT_A_TASTE" => Some(g__NOT_A_TASTE()),
            "_NOT_A_THING" => Some(g__NOT_A_THING()),
            "_PLACEHOLDER_OPENINGS" => Some(g__PLACEHOLDER_OPENINGS()),
            "_STORY" => Some(g__STORY()),
            "_THEIRS_BUT_NOT_A_THING" => Some(g__THEIRS_BUT_NOT_A_THING()),
            "_THING" => Some(g__THING()),
            "_TONE" => Some(g__TONE()),
            "_TYPED" => Some(g__TYPED()),
            "_a_taste" => Some(py_func("_a_taste", |args: &[Py]| _a_taste(args))),
            "_obstacle" => Some(g__obstacle()),
            "_pick" => Some(py_func("_pick", |args: &[Py]| _pick(args))),
            "_said_plainly" => Some(py_func("_said_plainly", |args: &[Py]| _said_plainly(args))),
            "_the_composer_would_answer" => Some(py_func("_the_composer_would_answer", |args: &[Py]| _the_composer_would_answer(args))),
            "_the_greeting_would_answer" => Some(py_func("_the_greeting_would_answer", |args: &[Py]| _the_greeting_would_answer(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_reply" => Some(py_func("_the_reply", |args: &[Py]| _the_reply(args))),
            "_the_thing_said" => Some(py_func("_the_thing_said", |args: &[Py]| _the_thing_said(args))),
            "_the_thread_is_heavy" => Some(py_func("_the_thread_is_heavy", |args: &[Py]| _the_thread_is_heavy(args))),
            "pick" => Some(g_pick()),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "_A_PERSON" => s__A_PERSON(v),
            "_DISLIKES" => s__DISLIKES(v),
            "_DISLIKE_VERBS" => s__DISLIKE_VERBS(v),
            "_FAN_OF" => s__FAN_OF(v),
            "_HEAVY" => s__HEAVY(v),
            "_I_SAY" => s__I_SAY(v),
            "_LEAD" => s__LEAD(v),
            "_LIKES" => s__LIKES(v),
            "_LIKE_VERBS" => s__LIKE_VERBS(v),
            "_NOT_A_TASTE" => s__NOT_A_TASTE(v),
            "_NOT_A_THING" => s__NOT_A_THING(v),
            "_PLACEHOLDER_OPENINGS" => s__PLACEHOLDER_OPENINGS(v),
            "_STORY" => s__STORY(v),
            "_THEIRS_BUT_NOT_A_THING" => s__THEIRS_BUT_NOT_A_THING(v),
            "_THING" => s__THING(v),
            "_TONE" => s__TONE(v),
            "_TYPED" => s__TYPED(v),
            "_obstacle" => s__obstacle(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.their_taste", module_obj);
    pyrt::xrefs::reg_fn("modules.their_taste", "_the_composer_would_answer", py_func("_the_composer_would_answer", |args: &[Py]| _the_composer_would_answer(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "_the_greeting_would_answer", py_func("_the_greeting_would_answer", |args: &[Py]| _the_greeting_would_answer(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "_the_thread_is_heavy", py_func("_the_thread_is_heavy", |args: &[Py]| _the_thread_is_heavy(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "_said_plainly", py_func("_said_plainly", |args: &[Py]| _said_plainly(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "_a_taste", py_func("_a_taste", |args: &[Py]| _a_taste(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "_the_thing_said", py_func("_the_thing_said", |args: &[Py]| _the_thing_said(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "_pick", py_func("_pick", |args: &[Py]| _pick(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "_the_reply", py_func("_the_reply", |args: &[Py]| _the_reply(args)));
    pyrt::xrefs::reg_fn("modules.their_taste", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.their_taste", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.their_taste", "_A_PERSON", g__A_PERSON, s__A_PERSON);
    pyrt::xrefs::reg_access("modules.their_taste", "_DISLIKES", g__DISLIKES, s__DISLIKES);
    pyrt::xrefs::reg_access("modules.their_taste", "_DISLIKE_VERBS", g__DISLIKE_VERBS, s__DISLIKE_VERBS);
    pyrt::xrefs::reg_access("modules.their_taste", "_FAN_OF", g__FAN_OF, s__FAN_OF);
    pyrt::xrefs::reg_access("modules.their_taste", "_HEAVY", g__HEAVY, s__HEAVY);
    pyrt::xrefs::reg_access("modules.their_taste", "_I_SAY", g__I_SAY, s__I_SAY);
    pyrt::xrefs::reg_access("modules.their_taste", "_LEAD", g__LEAD, s__LEAD);
    pyrt::xrefs::reg_access("modules.their_taste", "_LIKES", g__LIKES, s__LIKES);
    pyrt::xrefs::reg_access("modules.their_taste", "_LIKE_VERBS", g__LIKE_VERBS, s__LIKE_VERBS);
    pyrt::xrefs::reg_access("modules.their_taste", "_NOT_A_TASTE", g__NOT_A_TASTE, s__NOT_A_TASTE);
    pyrt::xrefs::reg_access("modules.their_taste", "_NOT_A_THING", g__NOT_A_THING, s__NOT_A_THING);
    pyrt::xrefs::reg_access("modules.their_taste", "_PLACEHOLDER_OPENINGS", g__PLACEHOLDER_OPENINGS, s__PLACEHOLDER_OPENINGS);
    pyrt::xrefs::reg_access("modules.their_taste", "_STORY", g__STORY, s__STORY);
    pyrt::xrefs::reg_access("modules.their_taste", "_THEIRS_BUT_NOT_A_THING", g__THEIRS_BUT_NOT_A_THING, s__THEIRS_BUT_NOT_A_THING);
    pyrt::xrefs::reg_access("modules.their_taste", "_THING", g__THING, s__THING);
    pyrt::xrefs::reg_access("modules.their_taste", "_TONE", g__TONE, s__TONE);
    pyrt::xrefs::reg_access("modules.their_taste", "_TYPED", g__TYPED, s__TYPED);
    pyrt::xrefs::reg_access("modules.their_taste", "_obstacle", g__obstacle, s__obstacle);
    pyrt::xrefs::reg_access("modules.their_taste", "pick", g_pick, s_pick);
}
