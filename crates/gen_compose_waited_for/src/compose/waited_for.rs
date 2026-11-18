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


thread_local! { static G_A_WAIT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAIT_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOKING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_OURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFFER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPLY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAITING_A_DECISION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAITING_FOR_AN_OFFER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAITING_FOR_AN_ORDER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAITING_FOR_A_BOOKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAITING_FOR_A_REPAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WAITING_FOR_A_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.waited_for: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_WAIT_ASK() -> Py { ensure_init(); G_A_WAIT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAIT_ASK(v: Py) { ensure_init(); G_A_WAIT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAIT_LENGTH() -> Py { ensure_init(); G_A_WAIT_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAIT_LENGTH(v: Py) { ensure_init(); G_A_WAIT_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOKING_WORDS() -> Py { ensure_init(); G_BOOKING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOKING_WORDS(v: Py) { ensure_init(); G_BOOKING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_HELD() -> Py { ensure_init(); G_NOTHING_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_HELD(v: Py) { ensure_init(); G_NOTHING_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_OURS() -> Py { ensure_init(); G_NOT_OURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_OURS(v: Py) { ensure_init(); G_NOT_OURS.with(|c| *c.borrow_mut() = v); }
pub fn g_OFFER_WORDS() -> Py { ensure_init(); G_OFFER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFFER_WORDS(v: Py) { ensure_init(); G_OFFER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDER_WORDS() -> Py { ensure_init(); G_ORDER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDER_WORDS(v: Py) { ensure_init(); G_ORDER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_POOLS() -> Py { ensure_init(); G_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOLS(v: Py) { ensure_init(); G_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR_WORDS() -> Py { ensure_init(); G_REPAIR_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR_WORDS(v: Py) { ensure_init(); G_REPAIR_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_REPLY_WORDS() -> Py { ensure_init(); G_REPLY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPLY_WORDS(v: Py) { ensure_init(); G_REPLY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ASK() -> Py { ensure_init(); G_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ASK(v: Py) { ensure_init(); G_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WAITING_A_DECISION() -> Py { ensure_init(); G_WAITING_A_DECISION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAITING_A_DECISION(v: Py) { ensure_init(); G_WAITING_A_DECISION.with(|c| *c.borrow_mut() = v); }
pub fn g_WAITING_FOR_AN_OFFER() -> Py { ensure_init(); G_WAITING_FOR_AN_OFFER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAITING_FOR_AN_OFFER(v: Py) { ensure_init(); G_WAITING_FOR_AN_OFFER.with(|c| *c.borrow_mut() = v); }
pub fn g_WAITING_FOR_AN_ORDER() -> Py { ensure_init(); G_WAITING_FOR_AN_ORDER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAITING_FOR_AN_ORDER(v: Py) { ensure_init(); G_WAITING_FOR_AN_ORDER.with(|c| *c.borrow_mut() = v); }
pub fn g_WAITING_FOR_A_BOOKING() -> Py { ensure_init(); G_WAITING_FOR_A_BOOKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAITING_FOR_A_BOOKING(v: Py) { ensure_init(); G_WAITING_FOR_A_BOOKING.with(|c| *c.borrow_mut() = v); }
pub fn g_WAITING_FOR_A_REPAIR() -> Py { ensure_init(); G_WAITING_FOR_A_REPAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAITING_FOR_A_REPAIR(v: Py) { ensure_init(); G_WAITING_FOR_A_REPAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_WAITING_FOR_A_REPLY() -> Py { ensure_init(); G_WAITING_FOR_A_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WAITING_FOR_A_REPLY(v: Py) { ensure_init(); G_WAITING_FOR_A_REPLY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A date somebody else holds: when the thing they are waiting on will be done.\n\n\"i ordered a new bed\" / \"how long do i have to wait\", \"the engineer is coming tuesday\" / \"will it\nbe fixed by friday\", \"i want a winter coat\" / \"will it still be on sale next week\" all got the\nsearch shrug (\"A quick search online will get you a solid answer, and I can help you make sense of\nwhat you find\"), which names neither the thing nor the place. None of these is a fact about the\nworld that the bot could learn: they are the shop's date, the repairer's date, the offer's end\ndate. So the reply says what is knowable about that kind of wait - what decides it, what the desk\nwill ask for, what the paperwork has on it - and then names the one who settles the rest.\n\nIt only stands in for a hedge (`pool is UNKNOWN`) and only where the thread says what is being\nwaited on, so a line with its own answer never reaches it.\n");
    s_A_WAIT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:when|whenabouts|how\\s+long|how\\s+soon|what\\s+day|by\\s+when)\\b[^?.!]{0,30}?\\b(?:get(?:s|ting)?\\s+here|arriv\\w*|turn(?:s|ing)?\\s+up|show(?:s|ing)?\\s+up|come(?:s|ing)?(?:\\s+round)?|be\\s+(?:here|done|ready|fixed|back|sorted|delivered|repaired|installed|finished|in\\s+stock|on\\s+sale)|have\\s+\\w+|hear(?:d)?(?:\\s+back)?|let\\s+(?:me|us)\\s+know|get\\s+back\\s+to\\s+(?:me|us)|tell\\s+(?:me|us)|waiting?)\\b|\\bwill\\s+(?:it|that|this|they)\\s+(?:still\\s+)?(?:have\\s+\\w+|be\\s+(?:fixed|done|ready|back|delivered|here|sorted|repaired|finished|installed|on\\s+sale|in\\s+stock|available|open)|come|arrive|turn\\s+up|show\\s+up|get\\s+here)\\b[^?.!]{0,30}?\\b(?:by|before|in\\s+time|next|this\\s+(?:week|month|weekend)|then|friday|monday|tuesday|wednesday|thursday|saturday|sunday|tomorrow|the\\s+weekend)\\b|\\bwill\\s+(?:they|it)\\s+still\\s+have\\s+(?:it|them|any|one)\\b|\\bis\\s+it\\s+worth\\s+waiting\\b|\\b(?:is|has)\\s+there\\s+(?:a|any)\\s+(?:date|day|delivery\\s+date|deadline)\\b|\\bany\\s+(?:date|word\\s+(?:yet|on\\s+when)|news\\s+(?:yet|on\\s+when))\\b|\\b(?:any|no)\\s+idea\\s+(?:of\\s+)?(?:when|how\\s+long|what\\s+day)\\b|\\bdate\\s+(?:yet|in\\s+sight|at\\s+all)\\b|\\bshould\\s+(?:i|we)\\s+wait\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WAIT_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:wait|hold off|hold on)\\b|\\bfor\\s+(?:the\\s+)?(?:sales?|payday|christmas|black\\s+friday|january|next\\s+(?:week|month))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ORDER_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:ordered|order|ordering|parcel|package|delivery|deliver|delivered|dispatch|dispatched|courier|tracking|arriving|on\\s+its\\s+way|coming\\s+(?:on|by|next|tomorrow)|bought|purchased|refund|returns?|sent\\s+(?:it\\s+)?back)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPAIR_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:broken|broke|not\\s+working|stopped\\s+working|repair|repairing|fixed|fixing|engineer|technician|repairman|plumber|electrician|boiler|fuse|spare\\s+part|part\\s+order|quote|callout|call\\s+out|warranty|leak\\w*|drip\\w*|flood\\w*|fault\\w*|cracked|smashed|packed\\s+up|on\\s+the\\s+blink|won'?t\\s+(?:start|work|drain|spin|heat|turn\\s+on)|stopped\\s+(?:working|turning|spinning|heating))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BOOKING_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:booked|booking|appointment|mot|service\\s+slot|viewing|quote\\s+visit|haircut|reservation|table\\s+booked|van\\s+hire|court|lesson)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPLY_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:cv|application|applied|applying|interview|recruiter|agency|hiring|hear\\s+back|heard\\s+back|reply|replied|emailed|phoned|rang|rang\\s+them|left\\s+a\\s+message|waiting\\s+to\\s+hear|offer\\s+letter|references?|landlord|letting|estate\\s+agent|boss|manager|council|DWP|benefits?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OFFER_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:sale|sales|discount|reduced|offer|deal|in\\s+stock|out\\s+of\\s+stock|cheaper|price|prices|bargain|coat|jacket|sofa|tv|laptop|phone|bike|pram|furniture|present|gift|looking\\s+at|thinking\\s+of\\s+buying|want\\s+to\\s+buy|going\\s+to\\s+buy)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_OURS(stdlib_call("re", "compile", &[py_str("\\b(?:hospital|doctor|gp|nurse|surgery|clinic|a&e|ambulance|diagnos\\w*|cancer|tumour|tumor|chemo|operation|surgery|infection|pregnan\\w*|baby|babies|miscarriage|funeral|died|dead|death|dying|grief|bereave\\w*|vet|vets|the\\s+dog|the\\s+cat|my\\s+dog|my\\s+cat)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_ASK(py_tuple(vec![py_str("how long do i have to wait"), py_str("how long will it be"), py_str("how long does it take"), py_str("will it be ready by then"), py_str("when will it be done")]));
    s_NOTHING_HELD(py_tuple(vec![py_str("A date like that is nearly always written down somewhere - a confirmation, an order page, a letter, an email - and that is where to look first: it carries a reference, the date they gave you, and what they promised. Then one short chase, quoting the reference and asking for a day rather than \"soon\", is what moves it. What is it you are waiting to hear about?"), py_str("Whichever date it is, three things settle it: find the paper or page with the reference on it, read what it actually promises, and ask the one who gave it to you for a day in writing. A date in writing is what you can hold them to if it slips. Tell me which wait this is and I will take it apart with you.")]));
    s_WAITING_FOR_AN_ORDER(py_tuple(vec![py_str("When it lands is the shop's to say, mine to help you plan around. The date they gave you is on the confirmation email or the order page, and it moves when they dispatch it; the quickest settling of it is a message to them with the order number, asking whether the date still holds and what happens if it slips."), py_str("Delivery dates are the shop's, not mine - and they are usually on the confirmation email, which also carries a tracking link once it ships. Ring or message them with the order number and ask them to confirm the day in writing; that gives you something to point at if it moves."), py_str("The shop holds that date, and the tracking link is the closest thing to it you have: it usually says what is happening today and when they expect to deliver. Ask them with the order number for a day rather than a window - a day you can plan a morning around.")]));
    s_WAITING_FOR_A_REPAIR(py_tuple(vec![py_str("Whether it is done by then is the repairer's call, not mine - what you can pin down is the part that decides it: whether they have the part, and whether they can give you a slot rather than a day. Ring them with the job reference and ask for both, and ask what happens to the price if the part is late."), py_str("That date belongs to whoever is doing the work, and what decides it is usually a part or a second visit. Ask them straight: is the part in, is the job booked in, and what would make it slip - then you know whether to plan around it or chase it."), py_str("Nobody can promise you the day from here, and a repairer can: they know the part and the diary. Ask them for the date in writing and what happens if the part is late, and keep the quote handy - a repair that slips is the moment a quote gets re-quoted.")]));
    s_WAITING_FOR_A_REPLY(py_tuple(vec![py_str("When they come back to you is theirs to decide, and what you can do is make it easy to say yes: one short chase, naming the date you applied and asking when they expect to decide. If there is a date in their own advert or letter, that is the one to hold them to."), py_str("Nobody can promise you their answer, and the polite version of chasing is the thing that moves it: ring or email once, say what you sent and when, and ask for a date rather than an answer. A week to ten days is the usual ask; after that a second note is reasonable."), py_str("That one is theirs to give, not mine - and it is worth asking them what their timeline is rather than whether they have decided. One chase, with the date you applied and the reference, is enough; if a decision date was written down anywhere, that is what to quote.")]));
    s_WAITING_FOR_A_BOOKING(py_tuple(vec![py_str("The place holds that date, and their own booking is the record of it: the time on the confirmation text or email is what they will go by. Ring them with the booking name and ask how long the slot really takes - the appointment time is usually the start of it, not the whole of it."), py_str("How long it takes is theirs to say, and it varies with what they find on the day. The confirmation will have the start time; ring and ask how long to allow and whether you should bring anything, and you will not be guessing on the morning.")]));
    s_WAITING_FOR_AN_OFFER(py_tuple(vec![py_str("Whether it is still on offer next week is the shop's call - nobody outside it knows when a sale ends, and the date is usually on the page under the price or the small print. If it is a thing you want, ask them before the day: a shop can often hold a price for a day or tell you the day it ends."), py_str("Sale dates are the shop's, and the page usually says when it finishes - look under the price for an end date, and ring the branch if it does not. Nobody can promise stock either: a popular size goes before the sale ends, so the thing worth asking is whether they will hold one, not whether the discount lasts."), py_str("That one is the shop's to answer: the price page normally carries the end date, and a phone call to the branch settles the rest - whether it is in stock in your size and whether they will put one by. A discount you cannot get in your size is no discount at all.")]));
    s_WAITING_A_DECISION(py_tuple(vec![py_str("Waiting is a real choice here, and the trade is easy to say: sales come round on set dates and prices do drop, but the thing you want can go out of stock first, and a wait on something you need now costs more than the discount saves. The shop's page carries the date a sale ends; what only you can answer is how much the wait is worth to you."), py_str("That is your call, and the parts of it are knowable: when sales usually land, whether the thing is likely to be in stock at the end of one, and what it costs you to go without in the meantime. If it is something you need this week, the wait has already lost; if it is a treat, a month is cheap. The shop's own page will tell you the date, and I can help you weigh it.")]));
    s_POOLS(py_tuple(vec![g_WAITING_FOR_AN_ORDER(), g_WAITING_FOR_A_REPAIR(), g_WAITING_FOR_A_BOOKING(), g_WAITING_FOR_A_REPLY(), g_WAITING_FOR_AN_OFFER(), g_WAITING_A_DECISION(), g_NOTHING_HELD()]));
}
pub fn asks_for_a_date(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut line: Py = Py::None;
    let _ = py_str("True when the line asks how long a wait is, or whether a date holds.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), repaired.clone()]));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_WAIT_ASK(), "search", &[line.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_thread(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let _ = py_str("What the person has said lately, this line included, as one lowercase string.\n\n    Only the person's turns: the bot's last answer is written in its own words, and \"a guaranteed\n    service such as next-day delivery\" about a late sofa put a repairer in a thread about a shop.\n    ");
    said = py_list(vec![call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    let _ = mcall(&said.clone(), "extend", &[slice_or_raise(&pyrt::xrefs::xcall("memory.holds", "lines", &[memory.clone()]), &op_neg(&Py::Int(4)), &Py::None, &Py::None)]);
    return mcall(&mcall(&py_str(" "), "join", &[said.clone()]), "lower", &[]);
    Py::None
}
pub fn _the_kind(args: &[Py]) -> Py {
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
    let mut thread: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thread") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thread"))) };
    let mut kind: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("'order' | 'repair' | 'booking' | 'offer' | 'decision' | None for what is being waited on.\n\n    The line itself wins: \"should i wait for the sales\" is a decision however the thread reads.\n    A repair outranks an order, because the thing that is broken is the thing the date is about.\n    ");
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:should\\s+(?:i|we)\\s+wait|is\\s+it\\s+worth\\s+waiting|wait\\s+(?:for|till|until)\\s+the\\s+(?:sales?|payday|christmas|black\\s+friday))\\b"), { let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("decision");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("repair"), g_REPAIR_WORDS()]), py_tuple(vec![py_str("order"), g_ORDER_WORDS()]), py_tuple(vec![py_str("booking"), g_BOOKING_WORDS()]), py_tuple(vec![py_str("reply"), g_REPLY_WORDS()]), py_tuple(vec![py_str("offer"), g_OFFER_WORDS()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&mcall(&words.clone(), "search", &[thread.clone()])) {
    return kind.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn for_the_ask(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut thread: Py = Py::None;
    let _ = py_str("The pool of lines for this wait, or None when the thread names nothing to wait for.\n\n    None is the answer for most lines: a wait about a body, a death, a pet or a safety thread is\n    somebody else's to settle and is read by the bands above this one.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&asks_for_a_date(&[clean.clone(), repaired.clone()])))) {
    return Py::None;
    }
    thread = _the_thread(&[clean.clone(), memory.clone()]);
    if py_truthy(&mcall(&g_NOT_OURS(), "search", &[thread.clone()])) {
    return Py::None;
    }
    kind = _the_kind(&[clean.clone(), thread.clone()]);
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("decision")))) {
    return g_WAITING_A_DECISION();
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("repair")))) {
    return g_WAITING_FOR_A_REPAIR();
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("order")))) {
    return g_WAITING_FOR_AN_ORDER();
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("booking")))) {
    return g_WAITING_FOR_A_BOOKING();
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("reply")))) {
    return g_WAITING_FOR_A_REPLY();
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("offer")))) {
    return g_WAITING_FOR_AN_OFFER();
    }
    return g_NOTHING_HELD();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.waited_for".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_WAIT_ASK" => Some(g_A_WAIT_ASK()),
            "A_WAIT_LENGTH" => Some(g_A_WAIT_LENGTH()),
            "BOOKING_WORDS" => Some(g_BOOKING_WORDS()),
            "NOTHING_HELD" => Some(g_NOTHING_HELD()),
            "NOT_OURS" => Some(g_NOT_OURS()),
            "OFFER_WORDS" => Some(g_OFFER_WORDS()),
            "ORDER_WORDS" => Some(g_ORDER_WORDS()),
            "POOLS" => Some(g_POOLS()),
            "REPAIR_WORDS" => Some(g_REPAIR_WORDS()),
            "REPLY_WORDS" => Some(g_REPLY_WORDS()),
            "THE_ASK" => Some(g_THE_ASK()),
            "WAITING_A_DECISION" => Some(g_WAITING_A_DECISION()),
            "WAITING_FOR_AN_OFFER" => Some(g_WAITING_FOR_AN_OFFER()),
            "WAITING_FOR_AN_ORDER" => Some(g_WAITING_FOR_AN_ORDER()),
            "WAITING_FOR_A_BOOKING" => Some(g_WAITING_FOR_A_BOOKING()),
            "WAITING_FOR_A_REPAIR" => Some(g_WAITING_FOR_A_REPAIR()),
            "WAITING_FOR_A_REPLY" => Some(g_WAITING_FOR_A_REPLY()),
            "_the_kind" => Some(py_func("_the_kind", |args: &[Py]| _the_kind(args))),
            "_the_thread" => Some(py_func("_the_thread", |args: &[Py]| _the_thread(args))),
            "asks_for_a_date" => Some(py_func("asks_for_a_date", |args: &[Py]| asks_for_a_date(args))),
            "for_the_ask" => Some(py_func("for_the_ask", |args: &[Py]| for_the_ask(args))),
            "earlier" => Some(pyrt::xrefs::xmodule("memory.holds")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_WAIT_ASK" => s_A_WAIT_ASK(v),
            "A_WAIT_LENGTH" => s_A_WAIT_LENGTH(v),
            "BOOKING_WORDS" => s_BOOKING_WORDS(v),
            "NOTHING_HELD" => s_NOTHING_HELD(v),
            "NOT_OURS" => s_NOT_OURS(v),
            "OFFER_WORDS" => s_OFFER_WORDS(v),
            "ORDER_WORDS" => s_ORDER_WORDS(v),
            "POOLS" => s_POOLS(v),
            "REPAIR_WORDS" => s_REPAIR_WORDS(v),
            "REPLY_WORDS" => s_REPLY_WORDS(v),
            "THE_ASK" => s_THE_ASK(v),
            "WAITING_A_DECISION" => s_WAITING_A_DECISION(v),
            "WAITING_FOR_AN_OFFER" => s_WAITING_FOR_AN_OFFER(v),
            "WAITING_FOR_AN_ORDER" => s_WAITING_FOR_AN_ORDER(v),
            "WAITING_FOR_A_BOOKING" => s_WAITING_FOR_A_BOOKING(v),
            "WAITING_FOR_A_REPAIR" => s_WAITING_FOR_A_REPAIR(v),
            "WAITING_FOR_A_REPLY" => s_WAITING_FOR_A_REPLY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.waited_for", module_obj);
    pyrt::xrefs::reg_fn("compose.waited_for", "asks_for_a_date", py_func("asks_for_a_date", |args: &[Py]| asks_for_a_date(args)));
    pyrt::xrefs::reg_fn("compose.waited_for", "_the_thread", py_func("_the_thread", |args: &[Py]| _the_thread(args)));
    pyrt::xrefs::reg_fn("compose.waited_for", "_the_kind", py_func("_the_kind", |args: &[Py]| _the_kind(args)));
    pyrt::xrefs::reg_fn("compose.waited_for", "for_the_ask", py_func("for_the_ask", |args: &[Py]| for_the_ask(args)));
    pyrt::xrefs::reg_access("compose.waited_for", "A_WAIT_ASK", g_A_WAIT_ASK, s_A_WAIT_ASK);
    pyrt::xrefs::reg_access("compose.waited_for", "A_WAIT_LENGTH", g_A_WAIT_LENGTH, s_A_WAIT_LENGTH);
    pyrt::xrefs::reg_access("compose.waited_for", "BOOKING_WORDS", g_BOOKING_WORDS, s_BOOKING_WORDS);
    pyrt::xrefs::reg_access("compose.waited_for", "NOTHING_HELD", g_NOTHING_HELD, s_NOTHING_HELD);
    pyrt::xrefs::reg_access("compose.waited_for", "NOT_OURS", g_NOT_OURS, s_NOT_OURS);
    pyrt::xrefs::reg_access("compose.waited_for", "OFFER_WORDS", g_OFFER_WORDS, s_OFFER_WORDS);
    pyrt::xrefs::reg_access("compose.waited_for", "ORDER_WORDS", g_ORDER_WORDS, s_ORDER_WORDS);
    pyrt::xrefs::reg_access("compose.waited_for", "POOLS", g_POOLS, s_POOLS);
    pyrt::xrefs::reg_access("compose.waited_for", "REPAIR_WORDS", g_REPAIR_WORDS, s_REPAIR_WORDS);
    pyrt::xrefs::reg_access("compose.waited_for", "REPLY_WORDS", g_REPLY_WORDS, s_REPLY_WORDS);
    pyrt::xrefs::reg_access("compose.waited_for", "THE_ASK", g_THE_ASK, s_THE_ASK);
    pyrt::xrefs::reg_access("compose.waited_for", "WAITING_A_DECISION", g_WAITING_A_DECISION, s_WAITING_A_DECISION);
    pyrt::xrefs::reg_access("compose.waited_for", "WAITING_FOR_AN_OFFER", g_WAITING_FOR_AN_OFFER, s_WAITING_FOR_AN_OFFER);
    pyrt::xrefs::reg_access("compose.waited_for", "WAITING_FOR_AN_ORDER", g_WAITING_FOR_AN_ORDER, s_WAITING_FOR_AN_ORDER);
    pyrt::xrefs::reg_access("compose.waited_for", "WAITING_FOR_A_BOOKING", g_WAITING_FOR_A_BOOKING, s_WAITING_FOR_A_BOOKING);
    pyrt::xrefs::reg_access("compose.waited_for", "WAITING_FOR_A_REPAIR", g_WAITING_FOR_A_REPAIR, s_WAITING_FOR_A_REPAIR);
    pyrt::xrefs::reg_access("compose.waited_for", "WAITING_FOR_A_REPLY", g_WAITING_FOR_A_REPLY, s_WAITING_FOR_A_REPLY);
}
