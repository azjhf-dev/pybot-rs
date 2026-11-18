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


thread_local! { static G_ASKING_IF_IM_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INVITED_AT_THE_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_INVITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_PRESENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_WANTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENCE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOGETHER_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOGETHER_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_compose_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASKING_IF_IM_HERE() -> Py { ensure_init(); G_ASKING_IF_IM_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_IF_IM_HERE(v: Py) { ensure_init(); G_ASKING_IF_IM_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITED() -> Py { ensure_init(); G_INVITED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITED(v: Py) { ensure_init(); G_INVITED.with(|c| *c.borrow_mut() = v); }
pub fn g_INVITED_AT_THE_START() -> Py { ensure_init(); G_INVITED_AT_THE_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INVITED_AT_THE_START(v: Py) { ensure_init(); G_INVITED_AT_THE_START.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_INVITE() -> Py { ensure_init(); G_NOT_AN_INVITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_INVITE(v: Py) { ensure_init(); G_NOT_AN_INVITE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_PRESENCE() -> Py { ensure_init(); G_NOT_PRESENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_PRESENCE(v: Py) { ensure_init(); G_NOT_PRESENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THEIRS() -> Py { ensure_init(); G_NOT_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THEIRS(v: Py) { ensure_init(); G_NOT_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_WANTED() -> Py { ensure_init(); G_NOT_WANTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_WANTED(v: Py) { ensure_init(); G_NOT_WANTED.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE() -> Py { ensure_init(); G_PRESENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE(v: Py) { ensure_init(); G_PRESENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENCE_REPLIES() -> Py { ensure_init(); G_PRESENCE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENCE_REPLIES(v: Py) { ensure_init(); G_PRESENCE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_TOGETHER_AT() -> Py { ensure_init(); G_TOGETHER_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOGETHER_AT(v: Py) { ensure_init(); G_TOGETHER_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_TOGETHER_REPLIES() -> Py { ensure_init(); G_TOGETHER_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOGETHER_REPLIES(v: Py) { ensure_init(); G_TOGETHER_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_compose_pick() -> Py { ensure_init(); G_compose_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_compose_pick(v: Py) { ensure_init(); G_compose_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("An invitation to the bot is a person asking for company, and nothing in the composer heard it.\n\n\"do you fancy a walk\" is not a question about anything in the world: it is one person asking\nanother along. Every reading in `bot/compose/fallback.py` hears it as a question anyway, so it\nreached the search hedge (\"that's beyond what I know well, sorry\") or, said the other way, the\nintake form (\"Got it. Is there anything you'd like a hand with?\"). Neither is what a person hears\nwhen they invite someone out, and neither is what a friend says back.\n\nA friend who cannot come does not do either. They say no to the coming - plainly, without\napologising for existing and without pretending they could - and they say yes to the thing: where\nwould you go, what is on, who is going. So the refusal comes first and the next question is about\nthe thing invited to, which is what `TOGETHER_AT` picks the pool by.\n\nOnly a line spoken to the bot is an invitation. \"i want to go for a pint\" is their own plan and\n\"we went for a walk\" is their news, so the shape has to be addressed (\"do you fancy\", \"shall we\")\nor start at the verb with nothing in front of it (\"fancy a brew\", \"want to grab a coffee\").\n\n`bot/modules/invitation.py` is the turn: it asks this reading and proposes the line, at a\nPRIORITY below the knowledge base, so a thing the answers do know keeps its answer (\"shall we get\na takeaway\" still gets the fish and chips, \"you up for a curry on friday\" the quick curry).\n");
    s_compose_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_INVITED(stdlib_call("re", "compile", &[py_str("\\b(?:do|would|will|can)\\s+you\\s+(?:want|fancy|feel\\s+like|be\\s+up\\s+for|come|join)\\b|\\b(?:do|would|will)\\s+you\\s+(?:like\\s+to\\s+)?(?:come|join)\\b|\\bfancy\\b|\\bup\\s+for\\b|\\bwanna\\b|\\bshall\\s+we\\b|\\bwe\\s+could\\b|\\bwant\\s+to\\s+(?:go|get|grab|have|see|watch|come|join)\\b|\\bcare\\s+to\\s+(?:come|join)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_INVITED_AT_THE_START(stdlib_call("re", "compile", &[py_str("^(?:fancy|wanna|up\\s+for|shall\\s+we|we\\s+could|want\\s+to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THEIRS(stdlib_call("re", "compile", &[py_str("^(?:i|we|he|she|they|my|our|her|his|their)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_WANTED(stdlib_call("re", "compile", &[py_str("\\b(?:know|knew|learn|ask|asked|tell|say|hear|check|explain|guess|help|try)\\b|\\b(?:see|check|find\\s+out)\\s+if\\b|\\bwhether\\b|\\bhow\\s+to\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_INVITE(stdlib_call("re", "compile", &[py_str("\\b(?:anything|something|a\\s+hand|a\\s+lift|a\\s+receipt|a\\s+piece|me\\s+come|her|him|them|die|dying|kill)\\b|\\b(?:want|like|need|fancy)\\s+(?:me|us)\\s+to\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRESENCE(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|would|will)\\s+(?:you|u)\\s+(?:please\\s+|possibly\\s+)?(?:come|pop|stop|drop|visit|join|meet|attend|be)\\b|\\bare\\s+(?:you|u)\\s+(?:coming|free|around|about)\\b|\\b(?:do|would|will)\\s+(?:you|u)\\s+(?:be\\s+)?(?:coming|there|around)\\b|\\b(?:you|u)\\s+(?:coming|joining\\s+(?:me|us))\\b|\\bwhy\\s+don'?t\\s+(?:you|u)\\s+(?:come|join)\\b|\\b(?:come|join)\\s+(?:me|us)\\s+(?:for|to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_PRESENCE(stdlib_call("re", "compile", &[py_str("\\bcome\\s+(?:up|back|across|down\\s+to|off)\\b|\\b(?:be|being)\\s+there\\s+for\\s+(?:me|us|him|her|them)\\b|\\bmeet\\s+(?:the|a|an|my|your|our)\\b|\\b(?:be|is|are|was)\\s+(?:there|around)\\s+(?:when|if|to\\s+(?:help|talk|listen))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_IF_IM_HERE(stdlib_call("re", "compile", &[py_str("^(?:hi|hey|hello|ok|okay|so|right|and|thanks|ta|sorry|please)?\\s*(?:are\\s+(?:you|u)|you|r\\s+u)\\s+(?:there|here|about|around)\\s*[?.!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_TOGETHER_AT(py_tuple(vec![py_tuple(vec![py_str("drink"), stdlib_call("re", "compile", &[py_str("\\b(?:coffee|cuppa|brew|pint|beer|wine|drink|juice|tea\\s+out|pub|bar|cafe|coffee\\s+shop)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("meal"), stdlib_call("re", "compile", &[py_str("\\b(?:lunch|dinner|brunch|breakfast|takeaway|take\\s+away|curry|pizza|chinese|indian|bite|meal|restaurant|nando'?s|kebab)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("walk"), stdlib_call("re", "compile", &[py_str("\\b(?:walk|stroll|hike|wander|run|jog|cycle|bike\\s+ride|swim|gym|park\\s+run)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("screen"), stdlib_call("re", "compile", &[py_str("\\b(?:film|movie|show|play|gig|concert|match|telly|cinema|theatre|festival)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("shopping"), stdlib_call("re", "compile", &[py_str("\\b(?:shopping|shops|market|city\\s+centre|high\\s+street|boot\\s+sale)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("out"), stdlib_call("re", "compile", &[py_str("\\b(?:day\\s+out|weekend\\s+away|trip|beach|countryside|seaside|coast|museum|stately\\s+home|picnic)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("do"), stdlib_call("re", "compile", &[py_str("\\b(?:party|bbq|barbecue|barbeque|wedding|birthday|christening|housewarming|house-warming|quiz|karaoke|reunion|funeral|wake|service|graduation|engagement|anniversary|get-together|get\\s+together|celebration|naming\\s+ceremony)\\b|\\b(?:a|the|this|that|my|his|her|their|big|fancy|work|office|leaving|christmas|charity|fundraising|retirement|birthday)\\s+do\\b")]).unwrap_or_else(|e| exc::raise(e))])]));
    s_TOGETHER_REPLIES(py_dict(vec![(py_str("drink"), py_tuple(vec![py_str("I can't come along, but that sounds like a good idea. Where do you go for it?"), py_str("Wish I could join you. Somewhere near you, or the place you always go?")])), (py_str("meal"), py_tuple(vec![py_str("I can't make it, but that sounds good. Is it a treat, or an ordinary weeknight?"), py_str("Wish I could be there. Somewhere new, or the place you always end up in?")])), (py_str("walk"), py_tuple(vec![py_str("I can't come with you, but that sounds like a good idea. Is there a route you like, or somewhere new to try?"), py_str("Wish I could join you. It clears the head more than people expect - where do you go?")])), (py_str("screen"), py_tuple(vec![py_str("I can't come along, but that is a good shout. What is on?"), py_str("Wish I could be there. Something you have been meaning to see, or picking on the night?")])), (py_str("shopping"), py_tuple(vec![py_str("I can't come along, but that sounds like a good afternoon. Is it a browse, or are you after something in particular?"), py_str("Wish I could be there. Do you know what you want, or will it find you?")])), (py_str("out"), py_tuple(vec![py_str("I can't come along, but that sounds like a good day out. Where are you thinking?"), py_str("Wish I could be there. Have you been before, or would it be the first time?")])), (py_str("do"), py_tuple(vec![py_str("I can't be there - I'm words on a screen - but it sounds a good do. What do you need help with for it: what to take, what to wear, or what to say to the host?"), py_str("No legs on this end, so I can't make it, sorry. Tell me how many are going and what you've got to sort out, and I'll do the planning side of it.")]))]));
    s_PRESENCE_REPLIES(py_tuple(vec![py_str("I can't be there in person - nothing here leaves the chat - but tell me the plan and I'll do the useful half: what to take, what to cook, how to get there."), py_str("No body on this end, so coming along isn't mine, sorry. Say what the day looks like and I'll help with the bits that are words - the list, the timing, what to say."), py_str("I'd be there if I could. I'm not, so let me help from here: how many are coming, and what have you got left to sort?")]));
}
pub fn _the_invitation(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut present: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Which pool answers an invitation to the bot, or None when the line is not one.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_IF_IM_HERE(), "match", &[text.clone()]) } }) {
    return Py::None;
    }
    present = { let __b = call_builtin("bool", &[mcall(&g_PRESENCE(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_PRESENCE(), "search", &[text.clone()]))) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&present.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_INVITED(), "search", &[text.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_INVITED_AT_THE_START(), "match", &[text.clone()]))); if !py_truthy(&__b) { __b } else { mcall(&g_NOT_THEIRS(), "match", &[text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&present.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_WANTED(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_AN_INVITE(), "search", &[text.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = present.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_NOT_AN_INVITE(), "search", &[text.clone()]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_TOGETHER_AT());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[text.clone()])) {
    return geti(&g_TOGETHER_REPLIES(), &kind.clone());
    }
    }
    if py_truthy(&present.clone()) {
    return g_PRESENCE_REPLIES();
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("The line an invitation to the bot gets, or None when the line is not one.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    pool = _the_invitation(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.invitation".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKING_IF_IM_HERE" => Some(g_ASKING_IF_IM_HERE()),
            "INVITED" => Some(g_INVITED()),
            "INVITED_AT_THE_START" => Some(g_INVITED_AT_THE_START()),
            "NOT_AN_INVITE" => Some(g_NOT_AN_INVITE()),
            "NOT_PRESENCE" => Some(g_NOT_PRESENCE()),
            "NOT_THEIRS" => Some(g_NOT_THEIRS()),
            "NOT_WANTED" => Some(g_NOT_WANTED()),
            "PRESENCE" => Some(g_PRESENCE()),
            "PRESENCE_REPLIES" => Some(g_PRESENCE_REPLIES()),
            "TOGETHER_AT" => Some(g_TOGETHER_AT()),
            "TOGETHER_REPLIES" => Some(g_TOGETHER_REPLIES()),
            "_the_invitation" => Some(py_func("_the_invitation", |args: &[Py]| _the_invitation(args))),
            "compose_pick" => Some(g_compose_pick()),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKING_IF_IM_HERE" => s_ASKING_IF_IM_HERE(v),
            "INVITED" => s_INVITED(v),
            "INVITED_AT_THE_START" => s_INVITED_AT_THE_START(v),
            "NOT_AN_INVITE" => s_NOT_AN_INVITE(v),
            "NOT_PRESENCE" => s_NOT_PRESENCE(v),
            "NOT_THEIRS" => s_NOT_THEIRS(v),
            "NOT_WANTED" => s_NOT_WANTED(v),
            "PRESENCE" => s_PRESENCE(v),
            "PRESENCE_REPLIES" => s_PRESENCE_REPLIES(v),
            "TOGETHER_AT" => s_TOGETHER_AT(v),
            "TOGETHER_REPLIES" => s_TOGETHER_REPLIES(v),
            "compose_pick" => s_compose_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.invitation", module_obj);
    pyrt::xrefs::reg_fn("compose.invitation", "_the_invitation", py_func("_the_invitation", |args: &[Py]| _the_invitation(args)));
    pyrt::xrefs::reg_fn("compose.invitation", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.invitation", "ASKING_IF_IM_HERE", g_ASKING_IF_IM_HERE, s_ASKING_IF_IM_HERE);
    pyrt::xrefs::reg_access("compose.invitation", "INVITED", g_INVITED, s_INVITED);
    pyrt::xrefs::reg_access("compose.invitation", "INVITED_AT_THE_START", g_INVITED_AT_THE_START, s_INVITED_AT_THE_START);
    pyrt::xrefs::reg_access("compose.invitation", "NOT_AN_INVITE", g_NOT_AN_INVITE, s_NOT_AN_INVITE);
    pyrt::xrefs::reg_access("compose.invitation", "NOT_PRESENCE", g_NOT_PRESENCE, s_NOT_PRESENCE);
    pyrt::xrefs::reg_access("compose.invitation", "NOT_THEIRS", g_NOT_THEIRS, s_NOT_THEIRS);
    pyrt::xrefs::reg_access("compose.invitation", "NOT_WANTED", g_NOT_WANTED, s_NOT_WANTED);
    pyrt::xrefs::reg_access("compose.invitation", "PRESENCE", g_PRESENCE, s_PRESENCE);
    pyrt::xrefs::reg_access("compose.invitation", "PRESENCE_REPLIES", g_PRESENCE_REPLIES, s_PRESENCE_REPLIES);
    pyrt::xrefs::reg_access("compose.invitation", "TOGETHER_AT", g_TOGETHER_AT, s_TOGETHER_AT);
    pyrt::xrefs::reg_access("compose.invitation", "TOGETHER_REPLIES", g_TOGETHER_REPLIES, s_TOGETHER_REPLIES);
    pyrt::xrefs::reg_access("compose.invitation", "compose_pick", g_compose_pick, s_compose_pick);
}
