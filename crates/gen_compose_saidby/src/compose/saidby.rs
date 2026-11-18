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


thread_local! { static G_ASKING_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DECLARATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRONOUN_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_SPOKEN_TO_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_TO_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_DECLARED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_SAID_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.saidby: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASKING_ABOUT_IT() -> Py { ensure_init(); G_ASKING_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_ABOUT_IT(v: Py) { ensure_init(); G_ASKING_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DECLARATION() -> Py { ensure_init(); G_A_DECLARATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DECLARATION(v: Py) { ensure_init(); G_A_DECLARATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRONOUN_HEAD() -> Py { ensure_init(); G_A_PRONOUN_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRONOUN_HEAD(v: Py) { ensure_init(); G_A_PRONOUN_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WISH() -> Py { ensure_init(); G_A_WISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WISH(v: Py) { ensure_init(); G_A_WISH.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_SPOKEN_TO_THEM() -> Py { ensure_init(); G_NOT_SPOKEN_TO_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_SPOKEN_TO_THEM(v: Py) { ensure_init(); G_NOT_SPOKEN_TO_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_FILLER() -> Py { ensure_init(); G_OWN_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_FILLER(v: Py) { ensure_init(); G_OWN_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SAYING() -> Py { ensure_init(); G_OWN_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SAYING(v: Py) { ensure_init(); G_OWN_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SUBJECT() -> Py { ensure_init(); G_OWN_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SUBJECT(v: Py) { ensure_init(); G_OWN_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTED() -> Py { ensure_init(); G_QUOTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTED(v: Py) { ensure_init(); G_QUOTED.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID() -> Py { ensure_init(); G_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID(v: Py) { ensure_init(); G_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_TO_ME() -> Py { ensure_init(); G_SAID_TO_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_TO_ME(v: Py) { ensure_init(); G_SAID_TO_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_DECLARED() -> Py { ensure_init(); G_THEY_DECLARED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_DECLARED(v: Py) { ensure_init(); G_THEY_DECLARED.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_SAID() -> Py { ensure_init(); G_THEY_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_SAID(v: Py) { ensure_init(); G_THEY_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_SAID_AGAIN() -> Py { ensure_init(); G_THEY_SAID_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_SAID_AGAIN(v: Py) { ensure_init(); G_THEY_SAID_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO() -> Py { ensure_init(); G_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO(v: Py) { ensure_init(); G_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A wish inside quotation marks belongs to the person who said it, not to the one who typed it.\n\nPeople bring other people's words to the bot all the time, and the line looks like a wish of\ntheir own because the words of a wish are in it:\n\n    she texted me \"we need to talk\" and nothing else\n        -> \"Talk\" and nothing else — fair enough. What's stopped it happening so far?\"\n    he texted \"i want to see you tonight\" and i panicked\n        -> \"Worth doing. What would you like help with first?\"\n    my mum keeps saying \"i want to see the grandkids more\"\n        -> \"See the grandkids more\" — fair enough. What's stopped it happening so far?\"\n\nEach of those is a plan pool reading somebody else's `i want` as the person's own, and the first\nand third say a fragment of a quotation back - half a sentence with a stray quote mark in it.\nThe person is not asking for help doing the thing; they are holding what was said to them.\n\nTwo things live here so they cannot drift apart:\n\n* `reads_as_somebody_elses_wish(text)` - the reading. It needs a quotation of at least two words,\n  a verb of saying with a person in front of it, and a wish or a hard thing to say inside the\n  quotation. Anything else stands down: a word quoted to be picked on (\"i want something called\n  \"sourdough\"\") is not this, and neither is a sign or a note (\"the note said \"no parking after\n  6\"\"), because nobody said it to the person.\n* `reply_for(text, memory)` - the wording. It asks about the person holding it, never about the\n  wish: the thing to help with is what they are left feeling, not the quotation.\n\nThe chat module reads both (`bot/modules/chat.py`): the plan reply (\"Worth doing\") and the thing\nreply (\"fair enough\") both stand down for a line this answers, and a card the knowledge base can\nanswer keeps its turn, because this is read from inside chat, not in front of it. That last part\nwas a promise this module did not keep: the reading only asked whether the line was somebody\nelse's words, never whether something else could answer them, so two lines with a card of their\nown were asked what went through the person's head instead:\n\n    my son said \"i hate you\" and went to his room\n        -> \"That's a big thing to have said to you.\"  where the child card says not to argue\n    my wife said \"i want a divorce\" last night\n        -> \"Hard to be on the end of that one.\"  where the money card has the first move\n\n`_the_card_keeps_its_turn` asks the module that owns the base, so the rule is the base's answer\nand not a list of subjects kept here (cream-flag, #routing 179).\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.QUOTED"); }
    s_QUOTED(stdlib_call("re", "compile", &[py_str("\"([^\"]{3,300})\"|\\u201c([^\\u201d]{3,300})\\u201d")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.SAID"); }
    s_SAID(stdlib_call("re", "compile", &[py_str("\\b(?P<who>(?:my|his|her|their|our|the)\\s+[a-z][a-z'-]*(?:\\s+(?:in[-\\s]?laws?|wife|husband))?|he|she|they|someone|somebody|everyone|everybody|a\\s+friend|a\\s+mate)\\b(?:\\s+[a-z']+){0,3}?\\s+(?:said|says|saying|texted|texts|text|messaged|messages|emailed|emails|wrote|writes|goes|went|was\\s+like|shouted|shouts|yelled|yells|screamed|screams|whispered|whispers|keeps?\\s+(?:saying|shouting|telling\\s+me)|kept\\s+(?:saying|shouting|telling\\s+me)|has\\s+said|had\\s+said|told\\s+me|tells\\s+me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.SAID_TO_ME"); }
    s_SAID_TO_ME(stdlib_call("re", "compile", &[py_str("\\b(?:said|says|texted|texts|messaged|emailed|told\\s+me)\\s+(?:to\\s+)?(?:me)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.A_WISH"); }
    s_A_WISH(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:want|wants|wanted|need|needs|needed|would\\s+like|d\\s+like|dont|don'?t)\\b|\\b(?:i|im|i'm|we|we're|weve|we've)\\s+(?:am\\s+|are\\s+|is\\s+)?(?:going\\s+to|leaving|done|out)\\b|\\b(?:i|we)\\s+miss\\b|\\bi\\s+(?:want|need)\\s+out\\b|\\bwe\\s+need\\s+to\\s+talk\\b|\\b(?:its|it'?s)\\s+over\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.A_DECLARATION"); }
    s_A_DECLARATION(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:really\\s+|still\\s+)?(?:love|hate)\\s+(?:you|him|her|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.NOT_SPOKEN_TO_THEM"); }
    s_NOT_SPOKEN_TO_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:note|notes|sign|signs|label|notice|letter|email\\s+says|form|ticket|invoice|bill|receipt|menu|website|app|screen|message\\s+on|instructions|recipe|book|leaflet|poster|song|songs|lyric|lyrics|radio|telly|tv|advert|advertisement|film|movie|poem|poems|show|podcast|headline|article|quote)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.OWN_SAYING"); }
    s_OWN_SAYING(stdlib_call("re", "compile", &[py_str("(?:said|say|says|texted|text|texts|messaged|message|emailed|email|wrote|told|replied|answered|sent)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.OWN_SUBJECT"); }
    s_OWN_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("i i'd id we we'd weve i've ive"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.OWN_FILLER"); }
    s_OWN_FILLER(call_builtin("frozenset", &[mcall(&py_str("just also only then never always still even actually already finally once today yesterday"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.THEY_SAID"); }
    s_THEY_SAID(py_tuple(vec![py_str("{Who}'s saying that is a lot to sit with. How are you taking it?"), py_str("That's a big thing to have said to you. What's going round your head?"), py_str("Hard to be on the end of that one. Do you want to talk it through?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.THEY_SAID_AGAIN"); }
    s_THEY_SAID_AGAIN(py_tuple(vec![py_str("{Who} keeps coming back to it, and that wears you down more than one saying does. What do you say back?"), py_str("Said often enough it stops being about the words. What do you think is behind it?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.THEY_DECLARED"); }
    s_THEY_DECLARED(py_tuple(vec![py_str("That's a big thing to have said to you. What went through your head?"), py_str("Words like that change the room. How do you feel about it?"), py_str("Hard to know what to do with being told that. What do you want to say back?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.ASKING_ABOUT_IT"); }
    s_ASKING_ABOUT_IT(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:what does that mean|what do i say|what should i say|what do i reply|what should i reply|can (?:he|she|they) do that|is that allowed|what do i do|should i)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.WHO"); }
    s_WHO(stdlib_call("re", "compile", &[py_str("\\b(my|his|her|their|our|the)\\s+([a-z][a-z'-]*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.saidby.A_PRONOUN_HEAD"); }
    s_A_PRONOUN_HEAD(call_builtin("frozenset", &[mcall(&py_str("he she they someone somebody everyone everybody"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn the_quotation(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The first sentence somebody said, or \"\" when the line quotes nobody.");
    found = mcall(&g_QUOTED(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "split", &[])]);
    Py::None
}
pub fn _the_one_who_said_it(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The words for the person who said it: \"my wife\", \"he\", or \"\" when nobody is named.");
    found = mcall(&g_SAID(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    who = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("who")]), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("someone"), py_str("somebody"), py_str("everyone"), py_str("everybody")]), &who.clone()))) {
    return who.clone();
    }
    return who.clone();
    Py::None
}
pub fn _before_the_quotation(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("The words in front of the first quotation: \"so i texted him \" of \"so i texted him \"i want...\"\".");
    found = mcall(&g_QUOTED(), "search", &[text.clone()]);
    return if py_truthy(&found.clone()) { slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None) } else { py_str("") };
    Py::None
}
pub fn said_by_the_person(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the quotation is the person's own message: \"so i texted him \"i want to see you\"\".\n\n    The verb of saying's own subject decides it. `i said`, `i texted him`, `we replied` mean the\n    words came from the person typing; `he said`, `my wife texted` mean they came at them. Only\n    the words in front of the quotation are read, so a line that reports one and sends one is\n    still read by whichever came first.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&_before_the_quotation(&[text.clone()]), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(",.!?;:")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_OWN_SAYING(), "fullmatch", &[geti(&words.clone(), &index.clone())])))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&words.clone(), &Py::None, &index.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_OWN_FILLER(), &word.clone()))) {
    continue;
    }
    return Py::Bool(py_contains(&g_OWN_SUBJECT(), &word.clone()));
    }
    return Py::Bool(false);
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_somebody_elses_wish(args: &[Py]) -> Py {
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
    let mut quoted: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line brings somebody else's wish or hard words, quoted, to the bot.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&said.clone(), &py_str("\""))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &py_str("“"))) } } } }) {
    return Py::Bool(false);
    }
    quoted = the_quotation(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&quoted.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[mcall(&quoted.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_WISH(), "search", &[quoted.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DECLARATION(), "search", &[quoted.clone()]) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_SPOKEN_TO_THEM(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_ABOUT_IT(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&said_by_the_person(&[said.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_SAID(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SAID_TO_ME(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _who_phrase(args: &[Py]) -> Py {
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
    let mut who: Py = Py::None;
    let _ = py_str("The person as the subject of the reply when the head is a pronoun, else \"\".");
    who = _the_one_who_said_it(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_PRONOUN_HEAD(), &geti(&mcall(&who.clone(), "split", &[]), &Py::Int(0)))) } }) {
    return py_str("");
    }
    return op_add(&mcall(&slice_or_raise(&who.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&who.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _fits_the_head(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let _ = py_str("Drop the lines that open on a person the line did not name that way.");
    return { let __b = Py::Bool(!py_contains(&line.clone(), &py_str("{Who}"))); if py_truthy(&__b) { __b } else { call_builtin("bool", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_card_keeps_its_turn(args: &[Py]) -> Py {
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
    let mut answers: Py = Py::None;
    let _ = py_str("True when the knowledge base has a card for the line, which then keeps its turn.\n\n    The reply here asks about the person holding the words, which is right when nothing else can\n    answer them. When the base does answer them it is the better reply by every rule in STYLE.md -\n    it says what to do about the thing, not how the person might be feeling about it - so this\n    reading stands down and `bot/modules/answers.py` says its piece. Asked of the module that\n    owns the base rather than of a subject list here, so a card added there needs nothing added\n    here.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try6 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&answers.clone(), "respond", &[text.clone(), if py_truthy(&Py::Bool(py_is(&memory.clone(), &Py::None))) { py_dict(vec![]) } else { memory.clone() }])]).unwrap_or_else(|e| exc::raise(e))]);
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
    let mut again: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("One reply that asks about the person holding somebody else's words, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_somebody_elses_wish(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_the_card_keeps_its_turn(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    who = _who_phrase(&[said.clone()]);
    if py_truthy(&{ let __b = mcall(&g_A_DECLARATION(), "search", &[the_quotation(&[said.clone()])]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_WISH(), "search", &[the_quotation(&[said.clone()])]))) } }) {
    pool = call_builtin("list", &[g_THEY_DECLARED()]).unwrap_or_else(|e| exc::raise(e));
    } else {
    again = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:keeps?\\s+(?:saying|shouting|on)|kept\\s+(?:saying|shouting|on)|always\\s+says|says\\s+it\\s+again|over\\s+and\\s+over)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    lines = if py_truthy(&again.clone()) { g_THEY_SAID_AGAIN() } else { g_THEY_SAID() };
    pool = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); if py_truthy(&_fits_the_head(&[line.clone(), who.clone()])) {  __out.push(line.clone()); } } py_list(__out) })() };
    }
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), said.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.saidby".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKING_ABOUT_IT" => Some(g_ASKING_ABOUT_IT()),
            "A_DECLARATION" => Some(g_A_DECLARATION()),
            "A_PRONOUN_HEAD" => Some(g_A_PRONOUN_HEAD()),
            "A_WISH" => Some(g_A_WISH()),
            "NOT_SPOKEN_TO_THEM" => Some(g_NOT_SPOKEN_TO_THEM()),
            "OWN_FILLER" => Some(g_OWN_FILLER()),
            "OWN_SAYING" => Some(g_OWN_SAYING()),
            "OWN_SUBJECT" => Some(g_OWN_SUBJECT()),
            "QUOTED" => Some(g_QUOTED()),
            "SAID" => Some(g_SAID()),
            "SAID_TO_ME" => Some(g_SAID_TO_ME()),
            "THEY_DECLARED" => Some(g_THEY_DECLARED()),
            "THEY_SAID" => Some(g_THEY_SAID()),
            "THEY_SAID_AGAIN" => Some(g_THEY_SAID_AGAIN()),
            "WHO" => Some(g_WHO()),
            "_before_the_quotation" => Some(py_func("_before_the_quotation", |args: &[Py]| _before_the_quotation(args))),
            "_fits_the_head" => Some(py_func("_fits_the_head", |args: &[Py]| _fits_the_head(args))),
            "_the_card_keeps_its_turn" => Some(py_func("_the_card_keeps_its_turn", |args: &[Py]| _the_card_keeps_its_turn(args))),
            "_the_one_who_said_it" => Some(py_func("_the_one_who_said_it", |args: &[Py]| _the_one_who_said_it(args))),
            "_who_phrase" => Some(py_func("_who_phrase", |args: &[Py]| _who_phrase(args))),
            "pick" => Some(g_pick()),
            "reads_as_somebody_elses_wish" => Some(py_func("reads_as_somebody_elses_wish", |args: &[Py]| reads_as_somebody_elses_wish(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "said_by_the_person" => Some(py_func("said_by_the_person", |args: &[Py]| said_by_the_person(args))),
            "the_quotation" => Some(py_func("the_quotation", |args: &[Py]| the_quotation(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKING_ABOUT_IT" => s_ASKING_ABOUT_IT(v),
            "A_DECLARATION" => s_A_DECLARATION(v),
            "A_PRONOUN_HEAD" => s_A_PRONOUN_HEAD(v),
            "A_WISH" => s_A_WISH(v),
            "NOT_SPOKEN_TO_THEM" => s_NOT_SPOKEN_TO_THEM(v),
            "OWN_FILLER" => s_OWN_FILLER(v),
            "OWN_SAYING" => s_OWN_SAYING(v),
            "OWN_SUBJECT" => s_OWN_SUBJECT(v),
            "QUOTED" => s_QUOTED(v),
            "SAID" => s_SAID(v),
            "SAID_TO_ME" => s_SAID_TO_ME(v),
            "THEY_DECLARED" => s_THEY_DECLARED(v),
            "THEY_SAID" => s_THEY_SAID(v),
            "THEY_SAID_AGAIN" => s_THEY_SAID_AGAIN(v),
            "WHO" => s_WHO(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.saidby", module_obj);
    pyrt::xrefs::reg_fn("compose.saidby", "the_quotation", py_func("the_quotation", |args: &[Py]| the_quotation(args)));
    pyrt::xrefs::reg_fn("compose.saidby", "_the_one_who_said_it", py_func("_the_one_who_said_it", |args: &[Py]| _the_one_who_said_it(args)));
    pyrt::xrefs::reg_fn("compose.saidby", "_before_the_quotation", py_func("_before_the_quotation", |args: &[Py]| _before_the_quotation(args)));
    pyrt::xrefs::reg_fn("compose.saidby", "said_by_the_person", py_func("said_by_the_person", |args: &[Py]| said_by_the_person(args)));
    pyrt::xrefs::reg_fn("compose.saidby", "reads_as_somebody_elses_wish", py_func("reads_as_somebody_elses_wish", |args: &[Py]| reads_as_somebody_elses_wish(args)));
    pyrt::xrefs::reg_fn("compose.saidby", "_who_phrase", py_func("_who_phrase", |args: &[Py]| _who_phrase(args)));
    pyrt::xrefs::reg_fn("compose.saidby", "_fits_the_head", py_func("_fits_the_head", |args: &[Py]| _fits_the_head(args)));
    pyrt::xrefs::reg_fn("compose.saidby", "_the_card_keeps_its_turn", py_func("_the_card_keeps_its_turn", |args: &[Py]| _the_card_keeps_its_turn(args)));
    pyrt::xrefs::reg_fn("compose.saidby", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.saidby", "ASKING_ABOUT_IT", g_ASKING_ABOUT_IT, s_ASKING_ABOUT_IT);
    pyrt::xrefs::reg_access("compose.saidby", "A_DECLARATION", g_A_DECLARATION, s_A_DECLARATION);
    pyrt::xrefs::reg_access("compose.saidby", "A_PRONOUN_HEAD", g_A_PRONOUN_HEAD, s_A_PRONOUN_HEAD);
    pyrt::xrefs::reg_access("compose.saidby", "A_WISH", g_A_WISH, s_A_WISH);
    pyrt::xrefs::reg_access("compose.saidby", "NOT_SPOKEN_TO_THEM", g_NOT_SPOKEN_TO_THEM, s_NOT_SPOKEN_TO_THEM);
    pyrt::xrefs::reg_access("compose.saidby", "OWN_FILLER", g_OWN_FILLER, s_OWN_FILLER);
    pyrt::xrefs::reg_access("compose.saidby", "OWN_SAYING", g_OWN_SAYING, s_OWN_SAYING);
    pyrt::xrefs::reg_access("compose.saidby", "OWN_SUBJECT", g_OWN_SUBJECT, s_OWN_SUBJECT);
    pyrt::xrefs::reg_access("compose.saidby", "QUOTED", g_QUOTED, s_QUOTED);
    pyrt::xrefs::reg_access("compose.saidby", "SAID", g_SAID, s_SAID);
    pyrt::xrefs::reg_access("compose.saidby", "SAID_TO_ME", g_SAID_TO_ME, s_SAID_TO_ME);
    pyrt::xrefs::reg_access("compose.saidby", "THEY_DECLARED", g_THEY_DECLARED, s_THEY_DECLARED);
    pyrt::xrefs::reg_access("compose.saidby", "THEY_SAID", g_THEY_SAID, s_THEY_SAID);
    pyrt::xrefs::reg_access("compose.saidby", "THEY_SAID_AGAIN", g_THEY_SAID_AGAIN, s_THEY_SAID_AGAIN);
    pyrt::xrefs::reg_access("compose.saidby", "WHO", g_WHO, s_WHO);
    pyrt::xrefs::reg_access("compose.saidby", "pick", g_pick, s_pick);
}
