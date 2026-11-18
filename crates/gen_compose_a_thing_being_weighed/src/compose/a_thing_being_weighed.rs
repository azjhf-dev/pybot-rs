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


thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DECISION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECEIPTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALREADY_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ARTICLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NOT_AN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BOUGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUTS_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUTS_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECIDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ITS_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIND_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LOOKING_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VOWELS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEIGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEIGHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__kind: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__word: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__words: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.a_thing_being_weighed: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_KINDS() -> Py { ensure_init(); G_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KINDS(v: Py) { ensure_init(); G_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DECISION() -> Py { ensure_init(); G_NOT_A_DECISION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DECISION(v: Py) { ensure_init(); G_NOT_A_DECISION.with(|c| *c.borrow_mut() = v); }
pub fn g_RECEIPTS() -> Py { ensure_init(); G_RECEIPTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECEIPTS(v: Py) { ensure_init(); G_RECEIPTS.with(|c| *c.borrow_mut() = v); }
pub fn g__ALREADY_HAVE() -> Py { ensure_init(); G__ALREADY_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALREADY_HAVE(v: Py) { ensure_init(); G__ALREADY_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g__ARTICLES() -> Py { ensure_init(); G__ARTICLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ARTICLES(v: Py) { ensure_init(); G__ARTICLES.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NOT_AN() -> Py { ensure_init(); G__A_NOT_AN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NOT_AN(v: Py) { ensure_init(); G__A_NOT_AN.with(|c| *c.borrow_mut() = v); }
pub fn g__BOUGHT() -> Py { ensure_init(); G__BOUGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BOUGHT(v: Py) { ensure_init(); G__BOUGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__CUTS_HEAD() -> Py { ensure_init(); G__CUTS_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUTS_HEAD(v: Py) { ensure_init(); G__CUTS_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__CUTS_NAME() -> Py { ensure_init(); G__CUTS_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUTS_NAME(v: Py) { ensure_init(); G__CUTS_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g__DECIDING() -> Py { ensure_init(); G__DECIDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECIDING(v: Py) { ensure_init(); G__DECIDING.with(|c| *c.borrow_mut() = v); }
pub fn g__EVENT() -> Py { ensure_init(); G__EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EVENT(v: Py) { ensure_init(); G__EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g__ITS_FOR() -> Py { ensure_init(); G__ITS_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ITS_FOR(v: Py) { ensure_init(); G__ITS_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__JOB() -> Py { ensure_init(); G__JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOB(v: Py) { ensure_init(); G__JOB.with(|c| *c.borrow_mut() = v); }
pub fn g__KIND_OF() -> Py { ensure_init(); G__KIND_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIND_OF(v: Py) { ensure_init(); G__KIND_OF.with(|c| *c.borrow_mut() = v); }
pub fn g__LOOKING_FOR() -> Py { ensure_init(); G__LOOKING_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LOOKING_FOR(v: Py) { ensure_init(); G__LOOKING_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENERS() -> Py { ensure_init(); G__OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENERS(v: Py) { ensure_init(); G__OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g__POOLS() -> Py { ensure_init(); G__POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POOLS(v: Py) { ensure_init(); G__POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g__POSSESSIVE() -> Py { ensure_init(); G__POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POSSESSIVE(v: Py) { ensure_init(); G__POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__VOWELS() -> Py { ensure_init(); G__VOWELS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VOWELS(v: Py) { ensure_init(); G__VOWELS.with(|c| *c.borrow_mut() = v); }
pub fn g__WEIGH() -> Py { ensure_init(); G__WEIGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEIGH(v: Py) { ensure_init(); G__WEIGH.with(|c| *c.borrow_mut() = v); }
pub fn g__WEIGHING() -> Py { ensure_init(); G__WEIGHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEIGHING(v: Py) { ensure_init(); G__WEIGHING.with(|c| *c.borrow_mut() = v); }
pub fn g__kind() -> Py { ensure_init(); G__kind.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__kind(v: Py) { ensure_init(); G__kind.with(|c| *c.borrow_mut() = v); }
pub fn g__word() -> Py { ensure_init(); G__word.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__word(v: Py) { ensure_init(); G__word.with(|c| *c.borrow_mut() = v); }
pub fn g__words() -> Py { ensure_init(); G__words.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__words(v: Py) { ensure_init(); G__words.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A statement that names something the person is weighing deserves more than a receipt.\n\nPeople do not always arrive with a question. Half of the turns that reach the bot are somebody\nthinking out loud about a thing they are about to decide:\n\n    im looking at an electric blanket for the winter\n    we are looking at a venue for the wedding\n    im going for an interview at a bank\n    its for a warehouse supervisor role\n\nEvery one of those named a real thing, and the bot answered with the same three words each time:\n\n    Got it. Is there anything you'd like a hand with?\n    Thanks for telling me - I'm here if you want to dig into any of that.\n    I'm listening - say more if it helps.\n\nThe three receipts in `fallback_data.NEUTRAL` are the right answer to \"the dog did a funny thing\ntoday\". They are the wrong answer to somebody standing in front of a decision, because the person\nhas said which decision and the bot has said nothing about it. STYLE.md's \"Not knowing, honestly\"\ngives the shape that works: the part the bot does know, said plainly, then one question.\n\nThe part it knows here is not a fact about the thing - it has never seen the blanket, the venue or\nthe job - it is what that *kind* of decision turns on, which is true of every one of them: what\nhappens after you pay, which round of interviews, how many people are coming. So this file reads\nthe kind off the words the person used, names their thing back to them, gives the two or three\nthings that settle that kind of decision, and asks the one question a friend would ask next. It\nnames no price, no model and no date, because those are the facts it does not have.\n\nTwo rules kept it honest while it was written. Nothing is answered unless the line names a thing:\n\"its a diesel\" and \"its a supermarket\" are left alone, because the kind cannot be read off them and\na frame for the wrong kind is worse than the receipt. And the kind has to be one of the three the\nfile knows (`bought`, `job`, `event`); anything else - a symptom, a move, a person - belongs to a\nmodule that reads it properly, so the receipt stands rather than a guess.\n\nIt is a last resort: `bot/modules/greeting.py` calls it only where the reply it already has is one\nof the three receipts, so any card, any entry and any real answer all keep the turn.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_RECEIPTS(py_tuple(vec![py_str("got it. is there anything you'd like a hand with?"), py_str("thanks for telling me — i'm here if you want to dig into any of that."), py_str("i'm listening — say more if it helps.")]));
    s__WEIGHING(py_str("(?:looking at|looking into|thinking about|thinking of|considering|weighing up|deciding on|deciding between|going for|applying for|trying for|after)"));
    s__WEIGH(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s*(?:so|and|but|ok|okay|well|oh|right)?\\s*(?:(?:i'?m|im|i am|we'?re|we are|she'?s|he'?s|they'?re|my (?:wife|husband|partner|mum|dad|son|daughter|friend))\\s+)?(?:still\\s+|also\\s+|just\\s+|now\\s+|actually\\s+)?"), &g__WEIGHING()), &py_str("\\s+(?P<thing>[a-z0-9' -]{3,45}?)\\s*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s__DECIDING(stdlib_call("re", "compile", &[py_str("\\b(?:deciding|torn|choosing|stuck) (?:between|over)\\s+(?:a|an|two|three|some|the)?\\s*(?P<thing>[a-z0-9' -]{3,45}?)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__LOOKING_FOR(stdlib_call("re", "compile", &[py_str("\\b(?:looking for|shopping for|hunting for|asking about|going to see|getting a quote for|getting quotes for)\\s+(?:a|an|some|a new|new|some new)\\s+(?P<thing>[a-z0-9' -]{3,45}?)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__ITS_FOR(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|oh\\s+|well\\s+)?(?:it'?s|its|they'?re|the one i want is)\\s+(?:for|to be for)\\s+(?:a|an|the|our|my|some)\\s+(?P<thing>[a-z0-9' -]{3,45}?)\\s*$|^\\s*(?:and\\s+|oh\\s+|well\\s+)?(?:it'?s|its)\\s+(?:a|an)\\s+(?P<bare>[a-z0-9' -]{3,45}?)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("wedding"), py_str("venue"), py_str("party"), py_str("birthday"), py_str("funeral"), py_str("christening"), py_str("anniversary"), py_str("reception"), py_str("celebration"), py_str("gathering"), py_str("conference"), py_str("shower"), py_str("bash")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("boiler"), py_str("blanket"), py_str("car"), py_str("van"), py_str("bike"), py_str("bicycle"), py_str("laptop"), py_str("computer"), py_str("phone"), py_str("mobile"), py_str("tablet"), py_str("mattress"), py_str("sofa"), py_str("settee"), py_str("chair"), py_str("table"), py_str("desk"), py_str("pram"), py_str("pushchair"), py_str("cot"), py_str("seat"), py_str("washer"), py_str("machine"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("cooker"), py_str("dishwasher"), py_str("dryer"), py_str("vacuum"), py_str("hoover"), py_str("kettle"), py_str("tv"), py_str("television"), py_str("monitor"), py_str("printer"), py_str("watch"), py_str("coat"), py_str("jacket"), py_str("boots"), py_str("boot"), py_str("tent"), py_str("caravan"), py_str("scooter"), py_str("mower"), py_str("lawnmower"), py_str("trimmer"), py_str("dehumidifier"), py_str("fryer"), py_str("microwave"), py_str("radiator"), py_str("pump"), py_str("panel"), py_str("burner")]) }
    s_KINDS(py_dict(vec![(py_str("job"), py_tuple(vec![py_str("job"), py_str("role"), py_str("position"), py_str("interview"), py_str("apprenticeship"), py_str("placement"), py_str("internship"), py_str("secondment"), py_str("promotion"), py_str("shift")])), (py_str("event"), __data1()), (py_str("bought"), __data2())]));
    s__KIND_OF(py_dict(vec![]));
    let __iter3 = py_iter_or_raise(&mcall(&g_KINDS(), "items", &[]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    s__kind(__unpk4[0].clone());
    s__words(__unpk4[1].clone());
    let __iter5 = py_iter_or_raise(&g__words());
    for __item in __iter5 {
    s__word(__item.clone());
    let _ = py_setitem(&g__KIND_OF(), &g__word(), g__kind());
    let _ = py_setitem(&g__KIND_OF(), &op_add(&g__word(), &py_str("s")), g__kind());
    }
    }
    s_NOT_A_DECISION(stdlib_call("re", "compile", &[py_str("\\b(?:pain|ache|hurts?|hurting|blood|bleeding|lump|rash|doctor|gp|nurse|hospital|surgery|symptom|dose|doses|tablet|tablets|antibiotic|prescription|period|pregnant|pregnancy|rent|rent arrears|evict\\w*|deposit|landlord|council tax|benefits?|universal credit|salary|wage|wages|payslip|court|solicitor|police|dead|died|dying|divorce|my (?:mum|dad|son|daughter|husband|wife|partner|friend|brother|sister|baby))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS(stdlib_call("re", "compile", &[py_str("^\\s*(?:so|but|and|ok|okay)?\\s*(?:what|which|how|when|where|who|why|is|are|was|were|do|does|did|can|could|should|would|will|shall|am)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("some"), py_str("my"), py_str("our"), py_str("a new"), py_str("new"), py_str("some new"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("any")]) }
    s__OPENERS(__data6());
    s__VOWELS(call_builtin("frozenset", &[py_str("aeiou")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NOT_AN(py_tuple(vec![py_str("used"), py_str("unused"), py_str("useful"), py_str("one"), py_str("once"), py_str("usual"), py_str("uniform"), py_str("university"), py_str("euro"), py_str("european"), py_str("ubiquitous")]));
    fn __data7() -> Py { py_tuple(vec![py_str("i think"), py_str("i reckon"), py_str("i suppose"), py_str("apparently"), py_str("though"), py_str("actually"), py_str("maybe"), py_str("probably"), py_str("instead"), py_str("than"), py_str("from"), py_str("on"), py_str("at"), py_str("in"), py_str("with"), py_str("about"), py_str("to"), py_str("and"), py_str("or"), py_str("for"), py_str("this week"), py_str("this weekend"), py_str("next week"), py_str("next month"), py_str("for now")]) }
    s__CUTS_HEAD(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("i think"), py_str("i reckon"), py_str("i suppose"), py_str("apparently"), py_str("though"), py_str("than i"), py_str("and"), py_str("or"), py_str("this week"), py_str("this weekend"), py_str("next week"), py_str("next month"), py_str("for now")]) }
    s__CUTS_NAME(__data8());
    s__ALREADY_HAVE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|mine|your|their)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__POSSESSIVE(py_tuple(vec![py_str("my"), py_str("our"), py_str("mine"), py_str("your"), py_str("their"), py_str("his"), py_str("her"), py_str("its")]));
    s__ARTICLES(py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("some"), py_str("any")]));
    s__BOUGHT(py_tuple(vec![py_str("With {thing}, what happens after you pay is what decides it: who repairs {it}, what the guarantee actually covers, and what the running cost is against the price. Get those three in writing, then compare. What will you be asking of {it}?"), py_str("For {thing}, the three questions a shop will not put on the label are who repairs {it}, what the guarantee covers and what the running cost is. Ask all three before you look at the price. Is it for every day or now and then?"), py_str("Buying {thing} comes down to use and after-sales: what you will ask of {it}, and who takes {it} back if something goes wrong. Settle those two and the shopping is the easy part. What is it for, mainly?")]));
    s__JOB(py_tuple(vec![py_str("For {thing}, preparation decides more than the day itself: one story for each line of your CV, and two questions about what the job actually has to deal with. Then ask what the hours and the first three months look like. Which stage are you at?"), py_str("Before {thing}, the work that pays is two or three stories you could tell a friend, and one question you genuinely want answered - the person who asks it is the one they remember. Do you know who you will be talking to?"), py_str("With {thing}, find out what the job has to deal with before you decide what to talk about: the hours, the team, and what happened to the last person in the role. Have you seen the job description, or just the advert?")]));
    s__EVENT(py_tuple(vec![py_str("For {thing}, the order that saves arguments is the date, then how many people, then what each one can cost - places quote by the head and by the day. Settle the first two before you look at any room, or the room decides them for you. How many are you expecting?"), py_str("With {thing}, the two numbers to pin down first are how many people and what each of them can cost, before you fall for a place; the date then follows whatever is left. Roughly how many?"), py_str("For {thing}, read it backwards: what you can spend, divided by the people coming, is what any place has to beat. Do that sum before you visit anywhere. How many are coming?")]));
    s__POOLS(py_dict(vec![(py_str("bought"), g__BOUGHT()), (py_str("job"), g__JOB()), (py_str("event"), g__EVENT())]));
}
pub fn _plain(args: &[Py]) -> Py {
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
    return mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    Py::None
}
pub fn _bare(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut opener: Py = Py::None;
    let mut said: Py = Py::None;
    said = mcall(&_plain(&[phrase.clone()]), "strip", &[py_str(" .,!?-")]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__OPENERS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    opener = __item.clone();
    if py_truthy(&mcall(&said.clone(), "startswith", &[op_add(&opener.clone(), &py_str(" "))])) {
    said = slice_or_raise(&said.clone(), &op_add(&call_builtin("len", &[opener.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &Py::None, &Py::None);
    break;
    }
    }
    return mcall(&said.clone(), "strip", &[py_str(" .,!-")]);
    Py::None
}
pub fn _as_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut head: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: head"))) };
    let mut first: Py = Py::None;
    let _ = py_str("The thing in the person's own words, with an article only if they gave none.");
    first = if py_truthy(&mcall(&name.clone(), "split", &[])) { geti(&mcall(&name.clone(), "split", &[]), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("an"), py_str("a")]), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&first.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__POSSESSIVE(), &first.clone())) } } } }) {
    return name.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("some"), py_str("any")]), &first.clone()))) {
    name = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&name.clone(), "split", &[]), &Py::Int(1), &Py::None, &Py::None)]);
    first = if py_truthy(&mcall(&name.clone(), "split", &[])) { geti(&mcall(&name.clone(), "split", &[]), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return py_str("");
    }
    }
    if py_truthy(&{ let __b = mcall(&first.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__POSSESSIVE(), &first.clone())); if py_truthy(&__b) { __b } else { _is_plural(&[head.clone()]) } } } }) {
    return name.clone();
    }
    return _with_article(&[name.clone()]);
    Py::None
}
pub fn _with_article(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut said: Py = Py::None;
    let _ = py_str("\"electric blanket\" -> \"an electric blanket\"; \"two laptops\" stays as it is.\n\n    Every template puts the phrase after a word rather than in the subject of a verb, so a plural\n    only has to keep its own shape instead of agreeing with anything.\n    ");
    said = _bare(&[phrase.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&geti(&said.clone(), &Py::Int(0)), "isdigit", &[]); if py_truthy(&__b) { __b } else { _is_plural(&[geti(&mcall(&said.clone(), "split", &[]), &op_neg(&Py::Int(1)))]) } } } }) {
    return said.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g__A_NOT_AN(), &geti(&mcall(&said.clone(), "split", &[]), &Py::Int(0))))) {
    return op_add(&py_str("a "), &said.clone());
    }
    if py_truthy(&{ let __b = _is_plural(&[geti(&mcall(&said.clone(), "split", &[]), &Py::Int(0))]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    return said.clone();
    }
    return op_add(&if py_truthy(&Py::Bool(py_contains(&g__VOWELS(), &geti(&said.clone(), &Py::Int(0))))) { py_str("an ") } else { py_str("a ") }, &said.clone());
    Py::None
}
pub fn _is_plural(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    return { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ss")]))) } };
    Py::None
}
pub fn _cut_at(args: &[Py]) -> Py {
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
    let mut cuts: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cuts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cuts"))) };
    let mut at: Py = Py::None;
    let mut found: Py = Py::None;
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    low = op_add(&op_add(&py_str(" "), &said.clone()), &py_str(" "));
    at = call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&cuts.clone());
    for __item in __iter1 {
    word = __item.clone();
    found = mcall(&low.clone(), "find", &[op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))]);
    if py_truthy(&{ let __cmp21: Py = found.clone(); if py_truthy(&op_lt(&op_neg(&Py::Int(1)), &__cmp21)) { op_lt(&__cmp21, &at.clone()) } else { Py::Bool(false) } }) {
    at = found.clone();
    }
    }
    return if py_truthy(&op_ge(&at.clone(), &call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))) { said.clone() } else { mcall(&slice_or_raise(&said.clone(), &Py::None, &at.clone(), &Py::None), "strip", &[]) };
    Py::None
}
pub fn _head_and_name(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut head_part: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("\"a second hand bike for commuting\" -> (\"bike\", \"a second hand bike\").");
    raw = mcall(&_plain(&[thing.clone()]), "strip", &[py_str(" .,!?-")]);
    said = _bare(&[raw.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_tuple(vec![py_str(""), py_str(""), py_str("")]);
    }
    head_part = _cut_at(&[said.clone(), g__CUTS_HEAD()]);
    head = if py_truthy(&mcall(&head_part.clone(), "split", &[])) { mcall(&mcall(&geti(&mcall(&head_part.clone(), "split", &[]), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?-")]), "lower", &[]) } else { py_str("") };
    return py_tuple(vec![head.clone(), _cut_at(&[raw.clone(), g__CUTS_NAME()]), head_part.clone()]);
    Py::None
}
pub fn _read(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut head_part: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("(kind, the thing as the person said it) or (None, None).");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASKS(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_DECISION(), "search", &[said.clone()]) } } } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    thing = Py::None;
    match_ = { let __b = mcall(&g__WEIGH(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__DECIDING(), "search", &[said.clone()]) } };
    if py_truthy(&match_.clone()) {
    thing = mcall(&match_.clone(), "group", &[py_str("thing")]);
    } else {
    match_ = mcall(&g__LOOKING_FOR(), "search", &[said.clone()]);
    if py_truthy(&match_.clone()) {
    thing = mcall(&match_.clone(), "group", &[py_str("thing")]);
    } else {
    match_ = mcall(&g__ITS_FOR(), "search", &[said.clone()]);
    if py_truthy(&match_.clone()) {
    thing = { let __b = mcall(&match_.clone(), "group", &[py_str("thing")]); if py_truthy(&__b) { __b } else { mcall(&match_.clone(), "group", &[py_str("bare")]) } };
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bthan\\b"), _plain(&[thing.clone()])]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    let __unpk1 = py_unpack(&_head_and_name(&[thing.clone()]), 3, None);
    head = __unpk1[0].clone();
    name = __unpk1[1].clone();
    head_part = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&name.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } } } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&geti(&mcall(&head.clone(), "split", &[]), &op_neg(&Py::Int(1))), "endswith", &[py_str("ing")]); if py_truthy(&__b) { __b } else { mcall(&geti(&mcall(&name.clone(), "split", &[]), &Py::Int(0)), "endswith", &[py_str("ing")]) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    kind = mcall(&g__KIND_OF(), "get", &[head.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("job"), py_str("bought")]), &kind.clone())); if !py_truthy(&__b) { __b } else { mcall(&g__ALREADY_HAVE(), "search", &[head_part.clone()]) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    return py_tuple(vec![kind.clone(), name.clone()]);
    Py::None
}
pub fn pool(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut head: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut template: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("Every phrasing for the kind of decision the line names, or None.");
    let __unpk1 = py_unpack(&_read(&[_plain(&[line.clone()])]), 2, None);
    kind = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&_head_and_name(&[thing.clone()]), 3, None);
    head = __unpk2[0].clone();
    name = __unpk2[1].clone();
    _u = __unpk2[2].clone();
    said = _as_said(&[name.clone(), head.clone()]);
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&g__POOLS(), &kind.clone())) { let mut template: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), said.clone()), ("it".to_string(), if py_truthy(&_is_plural(&[head.clone()])) { py_str("them") } else { py_str("it") })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&template.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_receipt(args: &[Py]) -> Py {
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
    let mut receipt: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the reply is one of the three receipts, with or without a question after it.");
    said = _plain(&[reply.clone()]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RECEIPTS()) { let mut receipt: Py = __item.clone(); __out.push(mcall(&said.clone(), "startswith", &[receipt.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
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
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { py_str("") };
    let mut options: Py = Py::None;
    let _ = py_str("A decision the person has just named, helped with, or None.\n\n    `reply` is what the bot was going to say. Only a receipt is replaced, so a card, an entry, a\n    module's own answer and the composer's own line all keep the turn.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_receipt(&[reply.clone()])))) {
    return Py::None;
    }
    options = pool(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&options.clone()))) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("compose", "pick", &[options.clone(), if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) }, text.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_thing_being_weighed".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS" => Some(g_ASKS()),
            "KINDS" => Some(g_KINDS()),
            "NOT_A_DECISION" => Some(g_NOT_A_DECISION()),
            "RECEIPTS" => Some(g_RECEIPTS()),
            "_ALREADY_HAVE" => Some(g__ALREADY_HAVE()),
            "_ARTICLES" => Some(g__ARTICLES()),
            "_A_NOT_AN" => Some(g__A_NOT_AN()),
            "_BOUGHT" => Some(g__BOUGHT()),
            "_CUTS_HEAD" => Some(g__CUTS_HEAD()),
            "_CUTS_NAME" => Some(g__CUTS_NAME()),
            "_DECIDING" => Some(g__DECIDING()),
            "_EVENT" => Some(g__EVENT()),
            "_ITS_FOR" => Some(g__ITS_FOR()),
            "_JOB" => Some(g__JOB()),
            "_KIND_OF" => Some(g__KIND_OF()),
            "_LOOKING_FOR" => Some(g__LOOKING_FOR()),
            "_OPENERS" => Some(g__OPENERS()),
            "_POOLS" => Some(g__POOLS()),
            "_POSSESSIVE" => Some(g__POSSESSIVE()),
            "_VOWELS" => Some(g__VOWELS()),
            "_WEIGH" => Some(g__WEIGH()),
            "_WEIGHING" => Some(g__WEIGHING()),
            "_as_said" => Some(py_func("_as_said", |args: &[Py]| _as_said(args))),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "_cut_at" => Some(py_func("_cut_at", |args: &[Py]| _cut_at(args))),
            "_head_and_name" => Some(py_func("_head_and_name", |args: &[Py]| _head_and_name(args))),
            "_is_plural" => Some(py_func("_is_plural", |args: &[Py]| _is_plural(args))),
            "_kind" => Some(g__kind()),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_read" => Some(py_func("_read", |args: &[Py]| _read(args))),
            "_with_article" => Some(py_func("_with_article", |args: &[Py]| _with_article(args))),
            "_word" => Some(g__word()),
            "_words" => Some(g__words()),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "pick" => Some(g_pick()),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "reads_as_a_receipt" => Some(py_func("reads_as_a_receipt", |args: &[Py]| reads_as_a_receipt(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS" => s_ASKS(v),
            "KINDS" => s_KINDS(v),
            "NOT_A_DECISION" => s_NOT_A_DECISION(v),
            "RECEIPTS" => s_RECEIPTS(v),
            "_ALREADY_HAVE" => s__ALREADY_HAVE(v),
            "_ARTICLES" => s__ARTICLES(v),
            "_A_NOT_AN" => s__A_NOT_AN(v),
            "_BOUGHT" => s__BOUGHT(v),
            "_CUTS_HEAD" => s__CUTS_HEAD(v),
            "_CUTS_NAME" => s__CUTS_NAME(v),
            "_DECIDING" => s__DECIDING(v),
            "_EVENT" => s__EVENT(v),
            "_ITS_FOR" => s__ITS_FOR(v),
            "_JOB" => s__JOB(v),
            "_KIND_OF" => s__KIND_OF(v),
            "_LOOKING_FOR" => s__LOOKING_FOR(v),
            "_OPENERS" => s__OPENERS(v),
            "_POOLS" => s__POOLS(v),
            "_POSSESSIVE" => s__POSSESSIVE(v),
            "_VOWELS" => s__VOWELS(v),
            "_WEIGH" => s__WEIGH(v),
            "_WEIGHING" => s__WEIGHING(v),
            "_kind" => s__kind(v),
            "_word" => s__word(v),
            "_words" => s__words(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_thing_being_weighed", module_obj);
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "_plain", py_func("_plain", |args: &[Py]| _plain(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "_as_said", py_func("_as_said", |args: &[Py]| _as_said(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "_with_article", py_func("_with_article", |args: &[Py]| _with_article(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "_is_plural", py_func("_is_plural", |args: &[Py]| _is_plural(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "_cut_at", py_func("_cut_at", |args: &[Py]| _cut_at(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "_head_and_name", py_func("_head_and_name", |args: &[Py]| _head_and_name(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "_read", py_func("_read", |args: &[Py]| _read(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "pool", py_func("pool", |args: &[Py]| pool(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "reads_as_a_receipt", py_func("reads_as_a_receipt", |args: &[Py]| reads_as_a_receipt(args)));
    pyrt::xrefs::reg_fn("compose.a_thing_being_weighed", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "KINDS", g_KINDS, s_KINDS);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "NOT_A_DECISION", g_NOT_A_DECISION, s_NOT_A_DECISION);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "RECEIPTS", g_RECEIPTS, s_RECEIPTS);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_ALREADY_HAVE", g__ALREADY_HAVE, s__ALREADY_HAVE);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_ARTICLES", g__ARTICLES, s__ARTICLES);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_A_NOT_AN", g__A_NOT_AN, s__A_NOT_AN);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_BOUGHT", g__BOUGHT, s__BOUGHT);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_CUTS_HEAD", g__CUTS_HEAD, s__CUTS_HEAD);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_CUTS_NAME", g__CUTS_NAME, s__CUTS_NAME);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_DECIDING", g__DECIDING, s__DECIDING);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_EVENT", g__EVENT, s__EVENT);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_ITS_FOR", g__ITS_FOR, s__ITS_FOR);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_JOB", g__JOB, s__JOB);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_KIND_OF", g__KIND_OF, s__KIND_OF);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_LOOKING_FOR", g__LOOKING_FOR, s__LOOKING_FOR);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_OPENERS", g__OPENERS, s__OPENERS);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_POOLS", g__POOLS, s__POOLS);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_POSSESSIVE", g__POSSESSIVE, s__POSSESSIVE);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_VOWELS", g__VOWELS, s__VOWELS);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_WEIGH", g__WEIGH, s__WEIGH);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_WEIGHING", g__WEIGHING, s__WEIGHING);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_kind", g__kind, s__kind);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_word", g__word, s__word);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "_words", g__words, s__words);
    pyrt::xrefs::reg_access("compose.a_thing_being_weighed", "pick", g_pick, s_pick);
}
