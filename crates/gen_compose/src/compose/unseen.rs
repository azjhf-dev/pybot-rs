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


thread_local! { static G_BARE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IGNORED_BY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_REGARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROFESSIONAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RISK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_SEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BARE_VERB() -> Py { ensure_init(); G_BARE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_VERB(v: Py) { ensure_init(); G_BARE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_DENIED() -> Py { ensure_init(); G_DENIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENIED(v: Py) { ensure_init(); G_DENIED.with(|c| *c.borrow_mut() = v); }
pub fn g_IGNORED_BY() -> Py { ensure_init(); G_IGNORED_BY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IGNORED_BY(v: Py) { ensure_init(); G_IGNORED_BY.with(|c| *c.borrow_mut() = v); }
pub fn g_LINES() -> Py { ensure_init(); G_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINES(v: Py) { ensure_init(); G_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_ME() -> Py { ensure_init(); G_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ME(v: Py) { ensure_init(); G_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED() -> Py { ensure_init(); G_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED(v: Py) { ensure_init(); G_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_OVER() -> Py { ensure_init(); G_NAMED_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_OVER(v: Py) { ensure_init(); G_NAMED_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_PEOPLE() -> Py { ensure_init(); G_NAMED_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_PEOPLE(v: Py) { ensure_init(); G_NAMED_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_REGARD() -> Py { ensure_init(); G_NAMED_REGARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_REGARD(v: Py) { ensure_init(); G_NAMED_REGARD.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_TOLD() -> Py { ensure_init(); G_NAMED_TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_TOLD(v: Py) { ensure_init(); G_NAMED_TOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ME() -> Py { ensure_init(); G_NOT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ME(v: Py) { ensure_init(); G_NOT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER() -> Py { ensure_init(); G_OTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER(v: Py) { ensure_init(); G_OTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE() -> Py { ensure_init(); G_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE(v: Py) { ensure_init(); G_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PROFESSIONAL() -> Py { ensure_init(); G_PROFESSIONAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROFESSIONAL(v: Py) { ensure_init(); G_PROFESSIONAL.with(|c| *c.borrow_mut() = v); }
pub fn g_RISK() -> Py { ensure_init(); G_RISK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RISK(v: Py) { ensure_init(); G_RISK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PERSON() -> Py { ensure_init(); G_THE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PERSON(v: Py) { ensure_init(); G_THE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_SEEN() -> Py { ensure_init(); G__NOT_SEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_SEEN(v: Py) { ensure_init(); G__NOT_SEEN.with(|c| *c.borrow_mut() = v); }
pub fn g__SEEN() -> Py { ensure_init(); G__SEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SEEN(v: Py) { ensure_init(); G__SEEN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Being around people and not being seen by them.\n\n\"i feel like nobody cares\", \"nobody ever asks how i am\", \"i feel invisible at work\" - the person\nis not saying how they feel about a thing, a plan or a fact, so none of the modules that answer\nthose lines wants it, and what was left was the stock nod (\"Got it. Is there anything you'd like\na hand with?\", \"I'm listening - say more if it helps\"), which answers nothing at all to somebody\nsaying they are lonely in a room full of people.\n\nOne reading, one pool: what a friend says to that is that it is a lonely way to feel, and then\none question. No advice - a weekly club is not the answer to \"nobody at home asks how i am\" -\nand never the line said back.\n\nThe shapes are the verb people use for the thing they are missing, with the person as its\nobject: nobody *listens to me*, no one *asks how i am*, nobody *is on my side*. A verb with a\nthing after it is not this (\"nobody cares about the bins\", \"nobody asked for a receipt\"), which\nis why the object is part of every shape rather than a word list alone.\n");
    s__SEEN(py_str("(?:cares?|cared|listens?|listened|understands?|understood|notices?|noticed|checks? (?:in|on me)|invites?|invited|includes?|included|sees? me|wants? me|likes? me|loves? me|stands? up for me|is (?:there|around) for me|are (?:there|around) for me|is on my side|are on my side|has time for me|knows? (?:my name|me|i exist)|have time for me|talks? to me|spoke to me|speaks to me|texts? me|rings? me|calls? me)"));
    s__NOT_SEEN(py_str("(?:invisible|unseen|unheard|overlooked|ignored|left out|an afterthought|a spare part|like furniture|like a ghost|like a nobody|like i am not there|like i'?m not there|like i do not exist|like i don'?t exist)"));
    s_OTHER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|they|him|her|them|his|their|my (?:mum|mom|mam|dad|boss|son|daughter|partner|husband|wife|friend|sister|brother|colleague|manager|neighbour|neighbor|carer))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_ME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:the (?:planet|climate|environment|government|club|team|company|shop|bins|bin|road|park|beach|library|bus|train)|them|it all|about (?:the|this|that)\\s+(?:company|job|place|team|club))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PROFESSIONAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:doctor|gp|surgery|consultant|nurse|midwife|health visitor|dentist|optician|teacher|head ?teacher|tutor|lecturer|social worker|council|landlord|letting agent|housing officer|jobcentre|dwp|bank|insurer)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RISK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:nobody|no ?one)\\s+would\\b|\\bif i (?:wasn'?t|was not|weren'?t|were not|am not|was |were )?(?:here|gone)\\b|\\bif i died\\b|\\bbetter off (?:dead|without me)\\b|\\bkill myself\\b|\\bend (?:it all|my life)\\b|\\bno reason to (?:live|go on)\\b|\\bnot be here any ?more\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DENIED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:im|i['\\u2019]?m|i\\s+(?:feel|felt|am|get|end up))\\s+(?:not|never|hardly|barely)\\s+(?:like\\s+)?"), &g__NOT_SEEN()), &py_str("\\b|\\b(?:nobody|no ?one|no-one|none of (?:my|them|the lot))\\b(?:\\s+\\w+){0,3}\\s+(?:not|never|does?nt|do not|didnt|isnt|is not|wont|will not)\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:nobody|no ?one|no-one|not one person|none of (?:my|them|the lot))(?:\\s+\\w+){0,4}\\s+(?:(?:cares?|cared|listens?|listened|understands?|understood|notices?|noticed)|(?:checks?|checked) (?:in|up))\\b(?P<tail>[^.!?,]{0,40})")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:$|(?:me|us)\\b|(?:in|up|on|with)\\s+(?:me|us)\\b|(?:about|to|for|of|at|with)\\s+(?:me|us)\\b|(?:about|to|for|of)\\s+(?:how|what|whether|if|anything|nothing|whatever|any\\s+of\\s+it)\\s+(?:i|we)\\b|(?:how|whether|if)\\s+(?:i|we)\\b|(?:in|up)\\s+(?:on|with)\\s+(?:me|us)\\b|(?:anymore|any ?more|at all|either|ever)\\b|(?:in|up)?\\s*$)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IGNORED_BY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:ignored|overlooked|left out|unseen|unheard|not noticed)\\s+by\\s+(?:the|a|an|my|our|his|her|their)\\s+(\\w+)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mam"), py_str("mother"), py_str("dad"), py_str("father"), py_str("parents"), py_str("family"), py_str("friends"), py_str("friend"), py_str("mates"), py_str("mate"), py_str("people"), py_str("everyone"), py_str("everybody"), py_str("anyone"), py_str("anybody"), py_str("someone"), py_str("somebody"), py_str("colleagues"), py_str("colleague"), py_str("coworkers"), py_str("neighbours"), py_str("neighbors"), py_str("kids"), py_str("children"), py_str("son"), py_str("daughter"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("brother"), py_str("sister"), py_str("siblings"), py_str("group"), py_str("others"), py_str("crowd"), py_str("team"), py_str("class"), py_str("work"), py_str("housemates"), py_str("flatmates"), py_str("school"), py_str("them"), py_str("him"), py_str("her")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:nobody|no ?one|no-one|not one person|none of (?:my|them|the lot))(?:\\s+\\w+){0,4}\\s+"), &g__SEEN()), &py_str("\\b|\\b(?:nobody|no ?one|no-one)\\s+(?:cares?|cared)\\b(?=\\s*(?:[.!?,]|$))|\\b(?:nobody|no ?one|no-one)(?:\\s+\\w+){0,2}\\s+(?:asks?|asked|notices?|noticed)\\s+(?:me\\s+|us\\s+)?(?:how|whether|if)\\s+(?:i|we)\\b|\\b(?:nobody|no ?one|no-one)(?:\\s+\\w+){0,4}\\s+(?:asks?|asked)\\s+(?:about|after)\\s+me\\b|\\b(?:nobody|no ?one|no-one)(?:\\s+\\w+){0,4}\\s+bothers?\\s+(?:with|about)\\s+me\\b|\\b(?:i\\s+(?:feel|felt|am|get|end up)|im|i['\\u2019]m)\\b(?:\\s+\\w+){0,2}\\s+(?:like\\s+)?")), &g__NOT_SEEN()), &py_str("\\b|\\b(?:feel|feels|felt|feeling)\\s+(?:like\\s+)?")), &g__NOT_SEEN()), &py_str("\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMED_PEOPLE(py_str("(?:my|our)\\s+(?:best\\s+|old\\s+|oldest\\s+|closest\\s+|closer\\s+|own\\s+|new\\s+|eldest\\s+|older\\s+|younger\\s+|little\\s+|big\\s+)?(?:friends?|mates?|family|mum|mom|mam|mother|dad|father|parents|sister|brother|siblings|husband|wife|partner|boyfriend|girlfriend|kids|children|son|daughter|flatmates?|housemates?|colleagues?|coworkers?|workmates?|neighbours?|neighbors?|relatives|cousins?|in-?laws)|the\\s+(?:girls|boys|lads|lasses|others|kids|team|group|people|family|mums|parents)|everyone|everybody"));
    s_NAMED_REGARD(py_str("(?:likes?|loves?|wants?|cares?(?:\\s+about)?|notices?|needs?|listens?\\s+to|bothers?\\s+with|invites?|includes?|misses?|respects?|trusts?|fancies|talks?\\s+to|texts?|rings?|calls?)"));
    s_NAMED_OVER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:want|love|need|miss|fancy)\\s+me\\s+(?:back|any\\s?more)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMED_TOLD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:likes?|loves?|wants?|needs?|asks?)\\s+(?:me|us)\\s+to\\s+\\w+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAMED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("people".to_string(), g_NAMED_PEOPLE()), ("regard".to_string(), g_NAMED_REGARD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\\b(?:{people})\\b(?:\\s+\\w+){{0,2}}\\s+(?:dont|don'?t|doesnt|doesn'?t|didnt|didn'?t|never|wont|won'?t|do not|does not|did not|will not)\\s+(?:\\w+\\s+){{0,2}}{regard}\\s+(?:me|us)\\b"), "format", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LINES(py_tuple(vec![py_str("That is a lonely place to be, and being surrounded by people does not fix it. Who is one person who does see you?"), py_str("I hear you. Talking and getting nothing back wears a person down. How long has it felt like this?"), py_str("Not being noticed by the people near you is its own kind of hard. Where does it bite most - home, work, or with friends?"), py_str("That would get to anyone, and it is not about being easy to overlook. Is there one person you could say this to?")]));
}
pub fn reads(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut by: Py = Py::None;
    let mut named: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the line is about not being seen by the people around them.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RISK(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    named = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_OVER(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED_TOLD(), "search", args))(&__args) })) } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ME(), "search", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DENIED(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&named.clone()) {
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PROFESSIONAL(), "search", args))(&__args) }));
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_ME(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_OTHER(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PROFESSIONAL(), "search", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_VERB(), "search", args))(&__args) };
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tail")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_PERSON(), "match", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    by = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IGNORED_BY(), "search", args))(&__args) };
    if py_truthy(&{ let __b = by.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PEOPLE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&by.clone(), "group", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.unseen".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BARE_VERB" => Some(g_BARE_VERB()),
            "DENIED" => Some(g_DENIED()),
            "IGNORED_BY" => Some(g_IGNORED_BY()),
            "LINES" => Some(g_LINES()),
            "ME" => Some(g_ME()),
            "NAMED" => Some(g_NAMED()),
            "NAMED_OVER" => Some(g_NAMED_OVER()),
            "NAMED_PEOPLE" => Some(g_NAMED_PEOPLE()),
            "NAMED_REGARD" => Some(g_NAMED_REGARD()),
            "NAMED_TOLD" => Some(g_NAMED_TOLD()),
            "NOT_ME" => Some(g_NOT_ME()),
            "OTHER" => Some(g_OTHER()),
            "PEOPLE" => Some(g_PEOPLE()),
            "PROFESSIONAL" => Some(g_PROFESSIONAL()),
            "RISK" => Some(g_RISK()),
            "THE_PERSON" => Some(g_THE_PERSON()),
            "_NOT_SEEN" => Some(g__NOT_SEEN()),
            "_SEEN" => Some(g__SEEN()),
            "reads" => Some(py_func("reads", |args: &[Py]| reads(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BARE_VERB" => s_BARE_VERB(v),
            "DENIED" => s_DENIED(v),
            "IGNORED_BY" => s_IGNORED_BY(v),
            "LINES" => s_LINES(v),
            "ME" => s_ME(v),
            "NAMED" => s_NAMED(v),
            "NAMED_OVER" => s_NAMED_OVER(v),
            "NAMED_PEOPLE" => s_NAMED_PEOPLE(v),
            "NAMED_REGARD" => s_NAMED_REGARD(v),
            "NAMED_TOLD" => s_NAMED_TOLD(v),
            "NOT_ME" => s_NOT_ME(v),
            "OTHER" => s_OTHER(v),
            "PEOPLE" => s_PEOPLE(v),
            "PROFESSIONAL" => s_PROFESSIONAL(v),
            "RISK" => s_RISK(v),
            "THE_PERSON" => s_THE_PERSON(v),
            "_NOT_SEEN" => s__NOT_SEEN(v),
            "_SEEN" => s__SEEN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.unseen", module_obj);
    pyrt::xrefs::reg_fn("compose.unseen", "reads", py_func("reads", |args: &[Py]| reads(args)));
    pyrt::xrefs::reg_access("compose.unseen", "BARE_VERB", g_BARE_VERB, s_BARE_VERB);
    pyrt::xrefs::reg_access("compose.unseen", "DENIED", g_DENIED, s_DENIED);
    pyrt::xrefs::reg_access("compose.unseen", "IGNORED_BY", g_IGNORED_BY, s_IGNORED_BY);
    pyrt::xrefs::reg_access("compose.unseen", "LINES", g_LINES, s_LINES);
    pyrt::xrefs::reg_access("compose.unseen", "ME", g_ME, s_ME);
    pyrt::xrefs::reg_access("compose.unseen", "NAMED", g_NAMED, s_NAMED);
    pyrt::xrefs::reg_access("compose.unseen", "NAMED_OVER", g_NAMED_OVER, s_NAMED_OVER);
    pyrt::xrefs::reg_access("compose.unseen", "NAMED_PEOPLE", g_NAMED_PEOPLE, s_NAMED_PEOPLE);
    pyrt::xrefs::reg_access("compose.unseen", "NAMED_REGARD", g_NAMED_REGARD, s_NAMED_REGARD);
    pyrt::xrefs::reg_access("compose.unseen", "NAMED_TOLD", g_NAMED_TOLD, s_NAMED_TOLD);
    pyrt::xrefs::reg_access("compose.unseen", "NOT_ME", g_NOT_ME, s_NOT_ME);
    pyrt::xrefs::reg_access("compose.unseen", "OTHER", g_OTHER, s_OTHER);
    pyrt::xrefs::reg_access("compose.unseen", "PEOPLE", g_PEOPLE, s_PEOPLE);
    pyrt::xrefs::reg_access("compose.unseen", "PROFESSIONAL", g_PROFESSIONAL, s_PROFESSIONAL);
    pyrt::xrefs::reg_access("compose.unseen", "RISK", g_RISK, s_RISK);
    pyrt::xrefs::reg_access("compose.unseen", "THE_PERSON", g_THE_PERSON, s_THE_PERSON);
    pyrt::xrefs::reg_access("compose.unseen", "_NOT_SEEN", g__NOT_SEEN, s__NOT_SEEN);
    pyrt::xrefs::reg_access("compose.unseen", "_SEEN", g__SEEN, s__SEEN);
}
