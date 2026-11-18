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


thread_local! { static G_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEMALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEMALE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRETCH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_OK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.no_page: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_BODY() -> Py { ensure_init(); G_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY(v: Py) { ensure_init(); G_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_DOING() -> Py { ensure_init(); G_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOING(v: Py) { ensure_init(); G_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_FEMALE() -> Py { ensure_init(); G_FEMALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEMALE(v: Py) { ensure_init(); G_FEMALE.with(|c| *c.borrow_mut() = v); }
pub fn g_FEMALE_WORDS() -> Py { ensure_init(); G_FEMALE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEMALE_WORDS(v: Py) { ensure_init(); G_FEMALE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MALE() -> Py { ensure_init(); G_MALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MALE(v: Py) { ensure_init(); G_MALE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THIS() -> Py { ensure_init(); G_NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THIS(v: Py) { ensure_init(); G_NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASION() -> Py { ensure_init(); G_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASION(v: Py) { ensure_init(); G_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASION_WORDS() -> Py { ensure_init(); G_OCCASION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASION_WORDS(v: Py) { ensure_init(); G_OCCASION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON() -> Py { ensure_init(); G_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON(v: Py) { ensure_init(); G_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL() -> Py { ensure_init(); G_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL(v: Py) { ensure_init(); G_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_WORDS() -> Py { ensure_init(); G_PLURAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_WORDS(v: Py) { ensure_init(); G_PLURAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PROSE() -> Py { ensure_init(); G_PROSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROSE(v: Py) { ensure_init(); G_PROSE.with(|c| *c.borrow_mut() = v); }
pub fn g_STRETCH_WORDS() -> Py { ensure_init(); G_STRETCH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRETCH_WORDS(v: Py) { ensure_init(); G_STRETCH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_OK() -> Py { ensure_init(); G_TAIL_OK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_OK(v: Py) { ensure_init(); G_TAIL_OK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE() -> Py { ensure_init(); G_WHOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE(v: Py) { ensure_init(); G_WHOLE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How to handle a person or a day of their own: there is no page, so name the parts.\n\nThree lines from the fallback census, all the same shape:\n\n    how do i handle christmas        -> \"I don't know enough to answer that honestly. A search\n    how do i manage my mum           ->  should sort you out quickly\"\n    how do i sort my life out        -> the same hedge\n\nA search cannot answer any of them, and that is not a gap in the search: how to handle Christmas\ndepends on the money, the days and who is in the room, and how to manage somebody's mum depends\non the mum and on the person asking. There is nothing written down to go and read. The hedge is\nworse than useless here - it sends a person looking for a page that does not exist.\n\nWhat a friend does is the one thing that is always true: say the ask is too big for one answer,\nname the two or three parts it is really made of, and ask which one to take first. That is a\nquestion a person can answer in four words, and it turns a shrug into a conversation.\n\nThree kinds, and only these:\n  - \"person\"   - a person of theirs (\"how do i deal with my ex\", \"how do i manage my mum\")\n  - \"occasion\" - a named day or season (\"how do i handle christmas\", \"how do i get through\n                 half term\")\n  - \"whole\"    - no thing at all (\"how do i sort my life out\", \"how do i get through the week\")\n\nIt never invents the answer, and it never guesses at a thing it was not asked about: an ask with\nno \"handle/deal with/manage/cope with/get through/sort out\" in it is left alone, so every how-to\nwith a real method (`how do i clean the filter`, `how do i drain the pasta`) and every body or\ndose question (`how do i manage the pain`) keep the reply they have.\n");
    s_DOING(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:do|can|could|should|would)\\s+(?:i|we|you)\\s+(?:handle|deal\\s+with|cope\\s+with|manage|get\\s+through|get\\s+past|sort\\s+(?:[\\w']+\\s+){0,3}out|live\\s+with|put\\s+up\\s+with|survive)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEMALE(py_str("(?:mum|mom|mam|mummy|mother|sister|daughter|wife|girlfriend|nan|nana|grandma|granny|aunt|aunty|niece|stepmum|stepmother)"));
    s_MALE(py_str("(?:dad|daddy|father|brother|son|husband|boyfriend|grandad|granddad|grandpa|uncle|nephew|stepdad|stepfather)"));
    s_PLURAL(py_str("(?:kids|children|parents|in-laws|brothers|sisters|twins|boys|girls|neighbours|neighbors)"));
    s_PERSON(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:handle|deal\\s+with|cope\\s+with|manage|live\\s+with|put\\s+up\\s+with|survive|sort\\s+out)\\s+(?P<who>(?:my|our|his|her|their)\\s+(?:"), &g_FEMALE()), &py_str("|")), &g_MALE()), &py_str("|")), &g_PLURAL()), &py_str(")(?:\\s+in\\s*-?\\s*law)?|my\\s+ex|her\\s+ex|his\\s+ex|the\\s+ex|my\\s+partner|the\\s+kids|the\\s+children)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OCCASION_WORDS(py_str("christmas|xmas|the\\s+holidays|half\\s+term|the\\s+summer\\s+holidays|birthdays?|the\\s+birthday|a\\s+birthday|the\\s+wedding|a\\s+wedding|mother'?s\\s+day|father'?s\\s+day|new\\s+year|new\\s+years\\s+eve"));
    s_STRETCH_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:half\\s+term|the\\s+holidays|the\\s+summer\\s+holidays|the\\s+school\\s+holidays|the\\s+week\\s+off)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OCCASION(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:handle|deal\\s+with|cope\\s+with|manage|get\\s+through|get\\s+past|sort\\s+out|live\\s+with|survive)\\s+(?:the\\s+|a\\s+|my\\s+|this\\s+)?(?P<what>"), &g_OCCASION_WORDS()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHOLE(stdlib_call("re", "compile", &[py_str("\\b(?:it\\s+all|everything|my\\s+life|life|the\\s+week|the\\s+month|the\\s+days|all\\s+of\\s+it|another\\s+week|the\\s+whole\\s+thing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("pain"), py_str("ache"), py_str("hurts"), py_str("hurting"), py_str("sore"), py_str("symptoms"), py_str("dose"), py_str("dosage"), py_str("tablets"), py_str("meds"), py_str("medication"), py_str("antibiotics"), py_str("insulin"), py_str("bleeding"), py_str("bleed"), py_str("rash"), py_str("lump"), py_str("breath"), py_str("breathing"), py_str("chest"), py_str("sleep"), py_str("insomnia"), py_str("withdrawal"), py_str("detox"), py_str("alcohol"), py_str("drinking"), py_str("grief"), py_str("grieving"), py_str("bereavement"), py_str("suicidal"), py_str("self harm"), py_str("panic")]) }
    s_BODY(__data1());
    s_NOT_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:clean|cleaning|fix|fixing|fit|fitting|wire|wiring|bleed\\s+the|install|assemble|unblock|drain|paint|paint\\s+over|plumb|tile|grout|service|reset|reinstall|renew|apply|book|claim|fill\\s+in|cancel|return|refund|get\\s+through\\s+to|get\\s+(?:to|in\\s+touch\\s+with))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PROSE(py_dict(vec![(py_str("person"), py_tuple(vec![py_str("There's no way to handle a person whole, so take one behaviour: the thing {who} will keep doing, and what you'll do when it happens again. Say it once, at a calm moment, about the time together rather than about {them} ('I need to be home by nine'), and keep that time shorter than feels natural - it is the part that helps most. Where the answer has to come from someone professional, the GP surgery and Citizens Advice are the two doors."), py_str("Nobody has written the page for {who}, but the working part is the same for everyone: pick the one thing that actually goes wrong, say it plainly once, and decide beforehand what you'll do if it happens again. Shorter stretches together, with something to do, work better than a long one you both dread. A carers' helpline or your GP surgery will take the specific version."), py_str("What usually helps is to stop trying to change {who} and change the shape of the time together instead: shorter, with something to do, one subject you can both stand, and one warm ordinary thing in it. Say your limit once, in advance, rather than in the moment. If it needs a professional - money, a diagnosis, a social worker - Citizens Advice and the GP surgery are where to ask.")])), (py_str("occasion"), py_tuple(vec![py_str("There's no one answer for {what}, because it is really three questions: what you can spend, whose place it is at, and which people you can't be in a room with. Settle the first two this week and the third gets much smaller. If money is the tight part, the budgeting pages at Citizens Advice walk through it free."), py_str("Most of {what} comes down to money, place and who is in the room, so take them in that order: a figure you can spend without flinching, where it happens, and the one or two people you'd rather not sit next to. Then pick one thing you actually want from the day and protect it - that is what makes it feel like yours."), py_str("Most of {what} is settled by three things: the money, the days themselves, and who is in the room. Fix a budget first, then decide the days, and shorten the part with the people you find hard. Citizens Advice goes through the money side free if it is the tight bit.")])), (py_str("stretch"), py_tuple(vec![py_str("With {what}, it is mostly about days rather than plans: pick two of them for something out of the house, leave the rest loose, and line up the one thing that gets everyone up in the morning. Ask whoever might help - a grandparent, a friend - this week rather than on the Monday, and your council lists the free activities by area."), py_str("What makes {what} work is a shape to the days: mornings out, afternoons quiet, and screens at a stated time rather than an argument. Decide the two days you'll go somewhere and check the free stuff now - the council's holiday pages and the local leisure centre are where those lists are."), py_str("Nobody gets {what} perfect, but the days go better with the mornings fixed: somewhere to be by ten, lunch sorted the night before, and one calm thing for the afternoon. The rest can be loose. If money is the tight part, the council's free-activity lists and Citizens Advice are the two places to look.")])), (py_str("whole"), py_tuple(vec![py_str("That is too big to answer as one thing, so start with the load: write down everything that's on you, then mark the two that make the rest worse - for most people that is money and sleep. Fix one of those this week with the smallest real step, a budget you can see or a bedtime you keep. If it is money, housing or a benefit, Citizens Advice will go through your actual numbers free."), py_str("Nobody can hand you the whole of that, but the order is the same for most people: sleep, money, then the thing you keep putting off. Pick the loudest one, take the smallest real step this week, and tell one person what you're doing - that is what keeps it moving. Your GP is the right next stop if it is your mood as much as the practical side."), py_str("The honest version is that there is no plan for all of it at once, but there is an order: sleep, money, then one thing you can finish. Write the list, cross off what is not actually yours to do, and pick one item small enough to do this week. Your GP or Citizens Advice are where the parts that need a professional go.")]))]));
    s_FEMALE_WORDS(call_builtin("tuple", &[stdlib_call("re", "findall", &[py_str("[a-z-]+"), g_FEMALE()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLURAL_WORDS(call_builtin("tuple", &[stdlib_call("re", "findall", &[py_str("[a-z-]+"), g_PLURAL()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str(""), py_str("on my own"), py_str("on my own now"), py_str("by myself"), py_str("alone"), py_str("now"), py_str("anymore"), py_str("any more"), py_str("these days"), py_str("at the moment"), py_str("this year"), py_str("this time"), py_str("again"), py_str("all of it"), py_str("and my job"), py_str("and work")]) }
    s_TAIL_OK(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    return mcall(&mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .!,")]), "lower", &[]);
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
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut phrase: Py = Py::None;
    let mut them: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The person as they were named, and the pronoun that goes with them.");
    phrase = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    phrase = stdlib_call("re", "sub", &[py_str("^(?:my|our|the)\\s+"), py_str("your "), phrase.clone()]).unwrap_or_else(|e| exc::raise(e));
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z-]+"), mcall(&phrase.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); __out.push(w.clone()); } py_list(__out) })() };
    them = py_str("them");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PLURAL_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_FEMALE_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    them = py_str("her");
    } else {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("ex"), py_str("partner")]), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    them = py_str("them");
    } else {
    them = py_str("him");
    }
    }
    }
    return py_tuple(vec![phrase.clone(), them.clone()]);
    Py::None
}
pub fn _tail_ok(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut tail: Py = Py::None;
    tail = mcall(&slice_or_raise(&line.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" .!,")]);
    return { let __b = Py::Bool(py_contains(&g_TAIL_OK(), &tail.clone())); if py_truthy(&__b) { __b } else { mcall(&tail.clone(), "startswith", &[py_str("when ")]) } };
    Py::None
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut line: Py = Py::None;
    let mut occasion: Py = Py::None;
    let mut person: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The kind of ask this is - \"person\", \"occasion\", \"whole\" - or None.");
    line = _bare(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DOING(), "search", &[line.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NOT_THIS(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BODY()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b")), line.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    person = mcall(&g_PERSON(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { _tail_ok(&[person.clone(), line.clone()]) } }) {
    return py_str("person");
    }
    occasion = mcall(&g_OCCASION(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = occasion.clone(); if !py_truthy(&__b) { __b } else { _tail_ok(&[occasion.clone(), line.clone()]) } }) {
    if py_truthy(&mcall(&g_STRETCH_WORDS(), "search", &[line.clone()])) {
    return py_str("stretch");
    }
    return py_str("occasion");
    }
    if py_truthy(&mcall(&g_WHOLE(), "search", &[line.clone()])) {
    return py_str("whole");
    }
    return Py::None;
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
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut them: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut what: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The parts it is made of and the one question back, or None when this is not the ask.");
    kind = reads(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    turns = call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    line = geti(&geti(&g_PROSE(), &kind.clone()), &op_modulo(&turns.clone(), &call_builtin("len", &[geti(&g_PROSE(), &kind.clone())]).unwrap_or_else(|e| exc::raise(e))));
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("person")))) {
    let __unpk1 = py_unpack(&_who(&[mcall(&g_PERSON(), "search", &[_bare(&[text.clone()])])]), 2, None);
    who = __unpk1[0].clone();
    them = __unpk1[1].clone();
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone()), ("them".to_string(), them.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("occasion"), py_str("stretch")]), &kind.clone()))) {
    what = mcall(&mcall(&g_OCCASION(), "search", &[_bare(&[text.clone()])]), "group", &[py_str("what")]);
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("what".to_string(), what.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) };
    return op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return line.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.no_page".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BODY" => Some(g_BODY()),
            "DOING" => Some(g_DOING()),
            "FEMALE" => Some(g_FEMALE()),
            "FEMALE_WORDS" => Some(g_FEMALE_WORDS()),
            "MALE" => Some(g_MALE()),
            "NOT_THIS" => Some(g_NOT_THIS()),
            "OCCASION" => Some(g_OCCASION()),
            "OCCASION_WORDS" => Some(g_OCCASION_WORDS()),
            "PERSON" => Some(g_PERSON()),
            "PLURAL" => Some(g_PLURAL()),
            "PLURAL_WORDS" => Some(g_PLURAL_WORDS()),
            "PROSE" => Some(g_PROSE()),
            "STRETCH_WORDS" => Some(g_STRETCH_WORDS()),
            "TAIL_OK" => Some(g_TAIL_OK()),
            "WHOLE" => Some(g_WHOLE()),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "_tail_ok" => Some(py_func("_tail_ok", |args: &[Py]| _tail_ok(args))),
            "_who" => Some(py_func("_who", |args: &[Py]| _who(args))),
            "reads" => Some(py_func("reads", |args: &[Py]| reads(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BODY" => s_BODY(v),
            "DOING" => s_DOING(v),
            "FEMALE" => s_FEMALE(v),
            "FEMALE_WORDS" => s_FEMALE_WORDS(v),
            "MALE" => s_MALE(v),
            "NOT_THIS" => s_NOT_THIS(v),
            "OCCASION" => s_OCCASION(v),
            "OCCASION_WORDS" => s_OCCASION_WORDS(v),
            "PERSON" => s_PERSON(v),
            "PLURAL" => s_PLURAL(v),
            "PLURAL_WORDS" => s_PLURAL_WORDS(v),
            "PROSE" => s_PROSE(v),
            "STRETCH_WORDS" => s_STRETCH_WORDS(v),
            "TAIL_OK" => s_TAIL_OK(v),
            "WHOLE" => s_WHOLE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.no_page", module_obj);
    pyrt::xrefs::reg_fn("compose.no_page", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("compose.no_page", "_who", py_func("_who", |args: &[Py]| _who(args)));
    pyrt::xrefs::reg_fn("compose.no_page", "_tail_ok", py_func("_tail_ok", |args: &[Py]| _tail_ok(args)));
    pyrt::xrefs::reg_fn("compose.no_page", "reads", py_func("reads", |args: &[Py]| reads(args)));
    pyrt::xrefs::reg_fn("compose.no_page", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.no_page", "BODY", g_BODY, s_BODY);
    pyrt::xrefs::reg_access("compose.no_page", "DOING", g_DOING, s_DOING);
    pyrt::xrefs::reg_access("compose.no_page", "FEMALE", g_FEMALE, s_FEMALE);
    pyrt::xrefs::reg_access("compose.no_page", "FEMALE_WORDS", g_FEMALE_WORDS, s_FEMALE_WORDS);
    pyrt::xrefs::reg_access("compose.no_page", "MALE", g_MALE, s_MALE);
    pyrt::xrefs::reg_access("compose.no_page", "NOT_THIS", g_NOT_THIS, s_NOT_THIS);
    pyrt::xrefs::reg_access("compose.no_page", "OCCASION", g_OCCASION, s_OCCASION);
    pyrt::xrefs::reg_access("compose.no_page", "OCCASION_WORDS", g_OCCASION_WORDS, s_OCCASION_WORDS);
    pyrt::xrefs::reg_access("compose.no_page", "PERSON", g_PERSON, s_PERSON);
    pyrt::xrefs::reg_access("compose.no_page", "PLURAL", g_PLURAL, s_PLURAL);
    pyrt::xrefs::reg_access("compose.no_page", "PLURAL_WORDS", g_PLURAL_WORDS, s_PLURAL_WORDS);
    pyrt::xrefs::reg_access("compose.no_page", "PROSE", g_PROSE, s_PROSE);
    pyrt::xrefs::reg_access("compose.no_page", "STRETCH_WORDS", g_STRETCH_WORDS, s_STRETCH_WORDS);
    pyrt::xrefs::reg_access("compose.no_page", "TAIL_OK", g_TAIL_OK, s_TAIL_OK);
    pyrt::xrefs::reg_access("compose.no_page", "WHOLE", g_WHOLE, s_WHOLE);
}
