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


thread_local! { static G_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SMALL_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTBURST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTBURST_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTBURST_WALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEELINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IDIOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IN_LAW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.harm: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_CHILD() -> Py { ensure_init(); G_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD(v: Py) { ensure_init(); G_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SMALL_CHILD() -> Py { ensure_init(); G_A_SMALL_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SMALL_CHILD(v: Py) { ensure_init(); G_A_SMALL_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY_LINES() -> Py { ensure_init(); G_BABY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY_LINES(v: Py) { ensure_init(); G_BABY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_LINES() -> Py { ensure_init(); G_CHILD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_LINES(v: Py) { ensure_init(); G_CHILD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_HURTING() -> Py { ensure_init(); G_HURTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURTING(v: Py) { ensure_init(); G_HURTING.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED() -> Py { ensure_init(); G_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED(v: Py) { ensure_init(); G_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTBURST() -> Py { ensure_init(); G_OUTBURST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTBURST(v: Py) { ensure_init(); G_OUTBURST.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTBURST_VERB() -> Py { ensure_init(); G_OUTBURST_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTBURST_VERB(v: Py) { ensure_init(); G_OUTBURST_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTBURST_WALL() -> Py { ensure_init(); G_OUTBURST_WALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTBURST_WALL(v: Py) { ensure_init(); G_OUTBURST_WALL.with(|c| *c.borrow_mut() = v); }
pub fn g_POOL() -> Py { ensure_init(); G_POOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POOL(v: Py) { ensure_init(); G_POOL.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_LEAD() -> Py { ensure_init(); G__BARE_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_LEAD(v: Py) { ensure_init(); G__BARE_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__BODY() -> Py { ensure_init(); G__BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BODY(v: Py) { ensure_init(); G__BODY.with(|c| *c.borrow_mut() = v); }
pub fn g__FEELINGS() -> Py { ensure_init(); G__FEELINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEELINGS(v: Py) { ensure_init(); G__FEELINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__HURT() -> Py { ensure_init(); G__HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HURT(v: Py) { ensure_init(); G__HURT.with(|c| *c.borrow_mut() = v); }
pub fn g__IDIOM() -> Py { ensure_init(); G__IDIOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IDIOM(v: Py) { ensure_init(); G__IDIOM.with(|c| *c.borrow_mut() = v); }
pub fn g__IN_LAW() -> Py { ensure_init(); G__IN_LAW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IN_LAW(v: Py) { ensure_init(); G__IN_LAW.with(|c| *c.borrow_mut() = v); }
pub fn g__JOINERS() -> Py { ensure_init(); G__JOINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOINERS(v: Py) { ensure_init(); G__JOINERS.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSON() -> Py { ensure_init(); G__PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSON(v: Py) { ensure_init(); G__PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_PERSON() -> Py { ensure_init(); G__THE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_PERSON(v: Py) { ensure_init(); G__THE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__WISH() -> Py { ensure_init(); G__WISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WISH(v: Py) { ensure_init(); G__WISH.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that says they want to hurt somebody, and the reply it deserves.\n\nPeople say this all day and they almost never mean it as a plan: \"i want to kill my boss\", \"i\ncould strangle him\", \"punch your brother\". It is anger with a person in it. The plan pool read\nit as a project and handed it back as approval -\n\n    i want to kill my boss       -> \"Worth doing. What would you like help with first?\"\n    i want to punch my brother   -> \"Punch your brother - fair enough. What's stopped it happening so far?\"\n    i want to hit my wife        -> \"Hit your wife - fair enough. What's stopped it happening so far?\"\n\n- which is the bot agreeing to a killing, in its own voice. Nothing in the line asked for help\nwith it; what it wants is to be heard, and the anger behind it is what the bot can answer.\n\nTwo things live here, so they cannot drift apart:\n\n* `reads_as_hurting_someone(clean)` - the reading. Two parts are needed and both have to be\n  there: a verb of hurting, and a person after it. `kill time`, `could murder a curry`, `hit the\n  sack`, `beat the traffic` and `kill two birds` have the verb and no person, so they keep their\n  own answers. A dog or a cat is somebody's too, so a pet counts as a person here. The line is\n  the speaker's own wish (`i want`, `i could`, `im going to`) or a bare instruction to them\n  (\"punch your brother\"); a question (\"should i hit him?\") is a question and is left alone.\n* `reply_for(clean, memory)` - the wording. It says no to the doing in the plainest way without\n  a lecture, then asks about the anger, which is the thing the person can actually talk about.\n  One of the four names the person they are angry with, so two turns in a row are not the same\n  sentence.\n\nThe chat module reads both (`bot/modules/chat.py`, `_their_thing` and `_intent`): a line this\nanswers is not a plan to get on with, and it is not a thing to ask \"which kind?\" about.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_HURTING(py_str("hurting_someone"));
    s__HURT(stdlib_call("re", "compile", &[py_str("\\b(?:kill|killing|murder|murdering|strangle|strangling|throttle|throttling|choke|choking|stab|stabbing|shoot|shooting|poison|poisoning|punch|punching|hit|hitting|smack|smacking|thump|thumping|whack|whacking|slap|slapping|kick|kicking|drown|drowning|deck|decking|bash|bashing|batter|battering|gouge|gouging|clobber|clobbering|pummel|pummelling|headbutt|headbutting|garrotte|garrote|hurt|hurting|smash|smashing|shake|shaking|shook|shaken|beat\\s+up|beat\\s+(?:the\\s+)?(?:living\\s+)?daylights\\s+out\\s+of|do\\s+(?:him|her|them)\\s+in|knock\\s+(?:him|her|them)\\s+out|run\\s+(?:him|her|them)\\s+over|set\\s+fire\\s+to|burn\\s+down|get\\s+(?:him|her|them)\\s+back|get\\s+my\\s+own\\s+back\\s+on)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:him|her|them|(?:his|her|their|your|my|our)\\s+(?:face|head|teeth|nose|jaw)|(?:(?:my|your|his|her|their|our|the)\\s+)?(?:boss|manager|husband|wife|partner|boyfriend|girlfriend|ex|fiance|fiancee|mum|mom|dad|mother|father|brother|sister|sons?|daughters?|child(?:ren)?|kids?|bab(?:y|ies)|toddlers?|infants?|newborns?|gran|nan|grandma|grandad|grandmother|grandfather|aunt|uncle|cousin|in-laws?|neighbour|neighbor|landlord|landlady|tenant|flatmate|housemate|roommate|mate|friend|colleague|coworker|co-worker|teacher|headteacher|classmate|stepmum|stepdad|stepson|stepdaughter|mother-in-law|father-in-law|careworker|carer|nurse|doctor|dr|gp|dentist|social\\s+worker|referee|coach|vicar|priest|imam|rabbi|officer|policeman|policewoman|cop|driver|cyclist|bus\\s+driver|security\\s+guard|bouncer|celebrity|politician|(?:that|the)\\s+(?:man|woman|bloke|guy|girl|lady|person|kid)|those\\s+(?:people|kids|children|men|women)|everyone|everybody))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_PERSON(stdlib_call("re", "compile", &[py_str("\\b((?:my|your|his|her|their|our)\\s+[a-z][a-z'-]*(?:\\s+in[-\\s]?laws?)?|(?:that|the)\\s+(?:man|woman|bloke|guy|lady|person|kid)|him|her|them)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__IDIOM(stdlib_call("re", "compile", &[py_str("\\b(?:kill\\s+(?:time|the\\s+(?:lights?|mood|music|engine|conversation))|kill\\s+two\\s+birds|kill\\s+it|killed\\s+it|killing\\s+(?:time|the\\s+time)|(?:could|can)\\s+murder\\s+a\\b|murder\\s+(?:a\\s+)?(?:song|tune|curry|pint|coffee|brew)|shake\\s+it\\s+off|shake\\s+a\\s+leg|shake\\s+hands?|shake\\s+(?:the\\s+)?dust|shake\\s+up|shake\\s+(?:some\\s+)?sense\\s+into|shake\\s+a\\s+fist|no\\s+great\\s+shakes|hit\\s+the\\s+(?:road|sack|hay|shops?|gym|bottle|roof|ground|deck|bar|pub|town)|hit\\s+the\\s+nail|hit\\s+a\\s+wall|hit\\s+it\\s+off|beat\\s+(?:the\\s+traffic|the\\s+queue|the\\s+rush|the\\s+clock|a\\s+retreat|the\\s+heat)|beat\\s+(?:the\\s+)?(?:eggs|cream|batter)|(?:slap|smack)\\s+(?:some\\s+)?(?:butter|sauce|paint)|kick\\s+(?:the\\s+tyres|off|back\\s+and\\s+relax|it)|kick\\s+(?:the\\s+)?(?:habit|bucket)|kick\\s+(?:\\w+\\s+){0,3}out\\b|shoot\\s+(?:hoops|some\\s+hoops|the\\s+shit)|shoot\\s+through|strangle\\s+a\\s+yawn|strangled\\s+a\\s+yawn)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WISH(stdlib_call("re", "compile", &[py_str("\\b(?:want|wanna|wants|wanting|going|gonna|about|ready|tempted|could|would|might|may|will|shall|should|need|needs|have|gotta|dying|half|like|feel)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("just"), py_str("please"), py_str("honestly"), py_str("god"), py_str("and"), py_str("then"), py_str("so"), py_str("ok"), py_str("okay"), py_str("right"), py_str("look"), py_str("mate"), py_str("now"), py_str("sometimes"), py_str("nearly"), py_str("almost")]) }
    s__BARE_LEAD(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s__FEELINGS(stdlib_call("re", "compile", &[py_str("\\bhurt(?:ing)?\\s+(?:[a-z']+\\s+){0,3}feelings?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POOL(py_tuple(vec![py_str("I'm not going to help with that, but something has pushed you a long way. What happened?"), py_str("That is a lot of anger to be carrying. What has actually happened?"), py_str("Fair to be that angry. What has been going on?")]));
    s_NAMED(py_str("Sounds like {person} has worn you down. What has been the worst of it?"));
    s_A_SMALL_CHILD(stdlib_call("re", "compile", &[py_str("\\b(?:bab(?:y|ies)|infants?|newborns?|toddlers?|little one)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILD(stdlib_call("re", "compile", &[py_str("\\b(?:child|children|kid|kids|stepchild|stepchildren|sons?|daughters?)\\b(?![\\s-]+in[\\s-]?law)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__IN_LAW(stdlib_call("re", "compile", &[py_str("^[\\s-]+in[\\s-]?laws?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("because"), py_str("who"), py_str("that"), py_str("so"), py_str("and"), py_str("but"), py_str("or"), py_str("if"), py_str("when"), py_str("whenever"), py_str("while"), py_str("at"), py_str("on"), py_str("in"), py_str("to"), py_str("for"), py_str("with"), py_str("about"), py_str("over"), py_str("again"), py_str("today"), py_str("tonight"), py_str("now"), py_str("last"), py_str("this"), py_str("every"), py_str("is"), py_str("was"), py_str("has"), py_str("had"), py_str("does"), py_str("did"), py_str("will"), py_str("would"), py_str("keeps"), py_str("kept"), py_str("said"), py_str("says"), py_str("told"), py_str("tells"), py_str("called"), py_str("texted"), py_str("right"), py_str("still"), py_str("just"), py_str("tomorrow"), py_str("yesterday"), py_str("later"), py_str("soon")]) }
    s__JOINERS(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    s_BABY_LINES(py_tuple(vec![py_str("Put some distance between you and them before anything else: out of the room, and put them down somewhere safe first. Then ring someone today - a partner, a friend, your health visitor or your GP - and say the words you just typed. Being this close to the edge is not a thing to sit on alone."), py_str("Step out of the room before anything else, and put them down somewhere safe first. Then ring somebody today rather than later: a health visitor, a GP, or a friend who will come over. Every parent gets to the end of their rope, and the ones who say it out loud are the ones who get help.")]));
    s_CHILD_LINES(py_tuple(vec![py_str("Step away from them before you say another word: out of the room, and a walk if you can. Then ring somebody today - a partner, a friend, your GP - and say the words you just typed. Wanting to hurt your own child is not a thing to carry on your own."), py_str("Put some distance between you and them now, and give it an hour before you speak to them again. Then tell one person today what you have just written - someone who will take it seriously. Being this far past the end of your rope is not something to sit on alone.")]));
    s_OUTBURST(stdlib_call("re", "compile", &[py_str("\\b(?:something|anything|whatever)\\b[.,!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OUTBURST_WALL(stdlib_call("re", "compile", &[py_str("\\b(?:a|the) wall\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OUTBURST_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:hit|punch|smack|slap|kick|smash|thump|whack|headbutt|scream|shout|throw)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__BODY(call_builtin("frozenset", &[py_tuple(vec![py_str("face"), py_str("head"), py_str("teeth"), py_str("nose"), py_str("jaw")])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _person_said(args: &[Py]) -> Py {
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
    let mut hurt: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("\"your boss\" - the person they are angry with, said in the second person.\n\n    The reply talks to them, not for them: \"my mate is a nightmare, i want to kill him\" got\n    \"Sounds like my mate has worn you down\", which is the bot putting the person's own word in\n    its own mouth.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    hurt = mcall(&g__HURT(), "search", &[text.clone()]);
    let __iter1 = py_iter_or_raise(&op_add(&if py_truthy(&hurt.clone()) { py_list(vec![slice_or_raise(&text.clone(), &mcall(&hurt.clone(), "start", &[]), &Py::None, &Py::None)]) } else { py_list(vec![]) }, &py_list(vec![text.clone()])));
    for __item in __iter1 {
    span = __item.clone();
    said = _said_back(&[span.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _said_back(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut swaps: Py = Py::None;
    let _ = py_str("The person in this piece of the line, said in the second person, or \"\".");
    found = mcall(&g__THE_PERSON(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_stands_on_its_own(&[text.clone(), found.clone()])))) {
    return py_str("");
    }
    said = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]);
    if py_truthy(&Py::Bool(py_contains(&g__BODY(), &geti(&mcall(&said.clone(), "partition", &[py_str(" ")]), &Py::Int(2))))) {
    return py_str("");
    }
    let __unpk1 = py_unpack(&mcall(&said.clone(), "partition", &[py_str(" ")]), 3, None);
    head = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    rest = __unpk1[2].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("that")]), &head.clone()))) {
    return py_str("");
    }
    swaps = py_dict(vec![(py_str("my"), py_str("your")), (py_str("our"), py_str("your")), (py_str("his"), py_str("their")), (py_str("her"), py_str("their")), (py_str("its"), py_str("their"))]);
    if py_truthy(&Py::Bool(py_contains(&swaps.clone(), &head.clone()))) {
    said = mcall(&op_add(&op_add(&geti(&swaps.clone(), &head.clone()), &py_str(" ")), &rest.clone()), "strip", &[]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("him"), py_str("her")]), &said.clone()))) {
    said = py_str("them");
    }
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("their"), py_str("them"), py_str("your")]), &said.clone()))) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn reads_as_hurting_someone(args: &[Py]) -> Py {
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
    let mut hurt: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says they want to hurt a person.\n\n    A question is not this (\"should i hit him?\" asks for an opinion and gets one), and neither is\n    the same verb said of somebody else (\"she wants to kill her boss\" is news about her, and \"my\n    brother punched a man\" is a story). The wish has to be the speaker's - with or without the \"i\n    want to\" in front of it, because \"punch your brother\" is the same wish left bare.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__IDIOM(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bkill(?:ing)?\\s+(?:my|your|him|her)sel(?:f|ves)?\\b|\\bkill(?:ing)?\\s+me\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    hurt = mcall(&g__HURT(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&hurt.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__FEELINGS(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__PERSON(), "search", &[slice_or_raise(&text.clone(), &mcall(&hurt.clone(), "start", &[]), &Py::None, &Py::None)])))) {
    tail = slice_or_raise(&text.clone(), &mcall(&hurt.clone(), "start", &[]), &op_add(&mcall(&hurt.clone(), "start", &[]), &Py::Int(40)), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_OUTBURST(), "search", &[tail.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_OUTBURST_WALL(), "search", &[tail.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_OUTBURST_VERB(), "search", &[tail.clone()]) } } } }))) {
    return Py::Bool(false);
    }
    }
    opening = mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&hurt.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,;:-–—")]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:dont|don'?t|do not|never|no longer|wouldnt|wouldn'?t|couldnt|couldn'?t|cant|can'?t|wont|won'?t)\\b(?:\\s+\\w+){0,4}\\s*$"), opening.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:he|she|they|we|you|his|her|their|its)\\b(?:\\s+\\w+){0,3}\\s+(?:wants?|wanted|is|was|keeps|kept|might|may|will|would)\\s*$"), opening.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:i|i'm|im|i'd|id|ive)\\b"), opening.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return call_builtin("bool", &[mcall(&g__WISH(), "search", &[opening.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    return { let __b = Py::Bool(!py_truthy(&opening.clone())); if py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&opening.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__BARE_LEAD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn reads_as_our_reply(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut head: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether this reply is one of the lines from here.\n\n    The arbiter recognises the harm reading by its wording rather than by the module that\n    proposed it: the reply is written here and only passed on by chat, and which module carries\n    it is chat's business. Naming a person (\"your brother\") is matched by its two fixed halves.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_POOL(), &text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BABY_LINES(), &text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CHILD_LINES(), &text.clone())) } } } }) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&mcall(&g_NAMED(), "partition", &[py_str("{person}")]), 3, None);
    head = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    tail = __unpk1[2].clone();
    return call_builtin("bool", &[{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { { let __b = tail.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&text.clone(), "startswith", &[head.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&text.clone(), "endswith", &[tail.clone()]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _child_at_the_hurt(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut flat: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "flat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: flat"))) };
    let mut first: Py = Py::None;
    let mut hurt: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("\"small\" or \"child\" when the doing is aimed at their own child, else \"\".\n\n    The word alone is not enough: `when i was a kid` is the speaker as a child, \"my son's\n    teacher\" is somebody else, and \"the man who hurt my daughter\" is a parent being protective.\n    So the child has to be the first person after the doing - the one it is aimed at - and it\n    has to be the whole of that person (\"my son\", not \"my son's teacher\").\n    ");
    hurt = mcall(&g__HURT(), "search", &[flat.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&hurt.clone()))) {
    return py_str("");
    }
    tail = slice_or_raise(&flat.clone(), &mcall(&hurt.clone(), "start", &[]), &Py::None, &Py::None);
    first = mcall(&g__PERSON(), "search", &[tail.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&first.clone()))) {
    return py_str("");
    }
    phrase = mcall(&first.clone(), "group", &[Py::Int(0)]);
    if py_truthy(&mcall(&g__IN_LAW(), "match", &[slice_or_raise(&tail.clone(), &mcall(&first.clone(), "end", &[]), &Py::None, &Py::None)])) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_stands_on_its_own(&[tail.clone(), first.clone()])))) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_A_SMALL_CHILD(), "search", &[phrase.clone()])) {
    return py_str("small");
    }
    return if py_truthy(&mcall(&g_A_CHILD(), "search", &[phrase.clone()])) { py_str("child") } else { py_str("") };
    Py::None
}
pub fn _stands_on_its_own(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the person phrase is the whole person, not the start of a longer noun phrase.");
    rest = mcall(&slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "lstrip", &[py_str(" ,.;:!?-")]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(true);
    }
    word = stdlib_call("re", "match", &[py_str("[a-z']+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__JOINERS(), &mcall(&word.clone(), "group", &[Py::Int(0)]))) } };
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut child: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut options: Py = Py::None;
    let mut person: Py = Py::None;
    let _ = py_str("The reply to a line that says they want to hurt somebody, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_hurting_someone(&[clean.clone()])))) {
    return Py::None;
    }
    flat = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    child = _child_at_the_hurt(&[flat.clone()]);
    if py_truthy(&Py::Bool(py_eq(&child.clone(), &py_str("small")))) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_BABY_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&child.clone()) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_CHILD_LINES(), memory.clone(), clean.clone()]);
    }
    person = _person_said(&[clean.clone()]);
    options = call_builtin("list", &[g_POOL()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("him"), py_str("her"), py_str("them")]), &person.clone())) } }) {
    let _ = mcall(&options.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("person".to_string(), person.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NAMED(), "format", args))(&__args) }]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&person.clone()))) {
    options = call_builtin("list", &[g_POOL()]).unwrap_or_else(|e| exc::raise(e));
    }
    return pyrt::xrefs::xcall("compose", "pick", &[options.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.harm".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CHILD" => Some(g_A_CHILD()),
            "A_SMALL_CHILD" => Some(g_A_SMALL_CHILD()),
            "BABY_LINES" => Some(g_BABY_LINES()),
            "CHILD_LINES" => Some(g_CHILD_LINES()),
            "HURTING" => Some(g_HURTING()),
            "NAMED" => Some(g_NAMED()),
            "OUTBURST" => Some(g_OUTBURST()),
            "OUTBURST_VERB" => Some(g_OUTBURST_VERB()),
            "OUTBURST_WALL" => Some(g_OUTBURST_WALL()),
            "POOL" => Some(g_POOL()),
            "_BARE_LEAD" => Some(g__BARE_LEAD()),
            "_BODY" => Some(g__BODY()),
            "_FEELINGS" => Some(g__FEELINGS()),
            "_HURT" => Some(g__HURT()),
            "_IDIOM" => Some(g__IDIOM()),
            "_IN_LAW" => Some(g__IN_LAW()),
            "_JOINERS" => Some(g__JOINERS()),
            "_PERSON" => Some(g__PERSON()),
            "_THE_PERSON" => Some(g__THE_PERSON()),
            "_WISH" => Some(g__WISH()),
            "_child_at_the_hurt" => Some(py_func("_child_at_the_hurt", |args: &[Py]| _child_at_the_hurt(args))),
            "_person_said" => Some(py_func("_person_said", |args: &[Py]| _person_said(args))),
            "_said_back" => Some(py_func("_said_back", |args: &[Py]| _said_back(args))),
            "_stands_on_its_own" => Some(py_func("_stands_on_its_own", |args: &[Py]| _stands_on_its_own(args))),
            "pick" => Some(g_pick()),
            "reads_as_hurting_someone" => Some(py_func("reads_as_hurting_someone", |args: &[Py]| reads_as_hurting_someone(args))),
            "reads_as_our_reply" => Some(py_func("reads_as_our_reply", |args: &[Py]| reads_as_our_reply(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CHILD" => s_A_CHILD(v),
            "A_SMALL_CHILD" => s_A_SMALL_CHILD(v),
            "BABY_LINES" => s_BABY_LINES(v),
            "CHILD_LINES" => s_CHILD_LINES(v),
            "HURTING" => s_HURTING(v),
            "NAMED" => s_NAMED(v),
            "OUTBURST" => s_OUTBURST(v),
            "OUTBURST_VERB" => s_OUTBURST_VERB(v),
            "OUTBURST_WALL" => s_OUTBURST_WALL(v),
            "POOL" => s_POOL(v),
            "_BARE_LEAD" => s__BARE_LEAD(v),
            "_BODY" => s__BODY(v),
            "_FEELINGS" => s__FEELINGS(v),
            "_HURT" => s__HURT(v),
            "_IDIOM" => s__IDIOM(v),
            "_IN_LAW" => s__IN_LAW(v),
            "_JOINERS" => s__JOINERS(v),
            "_PERSON" => s__PERSON(v),
            "_THE_PERSON" => s__THE_PERSON(v),
            "_WISH" => s__WISH(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.harm", module_obj);
    pyrt::xrefs::reg_fn("compose.harm", "_person_said", py_func("_person_said", |args: &[Py]| _person_said(args)));
    pyrt::xrefs::reg_fn("compose.harm", "_said_back", py_func("_said_back", |args: &[Py]| _said_back(args)));
    pyrt::xrefs::reg_fn("compose.harm", "reads_as_hurting_someone", py_func("reads_as_hurting_someone", |args: &[Py]| reads_as_hurting_someone(args)));
    pyrt::xrefs::reg_fn("compose.harm", "reads_as_our_reply", py_func("reads_as_our_reply", |args: &[Py]| reads_as_our_reply(args)));
    pyrt::xrefs::reg_fn("compose.harm", "_child_at_the_hurt", py_func("_child_at_the_hurt", |args: &[Py]| _child_at_the_hurt(args)));
    pyrt::xrefs::reg_fn("compose.harm", "_stands_on_its_own", py_func("_stands_on_its_own", |args: &[Py]| _stands_on_its_own(args)));
    pyrt::xrefs::reg_fn("compose.harm", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.harm", "A_CHILD", g_A_CHILD, s_A_CHILD);
    pyrt::xrefs::reg_access("compose.harm", "A_SMALL_CHILD", g_A_SMALL_CHILD, s_A_SMALL_CHILD);
    pyrt::xrefs::reg_access("compose.harm", "BABY_LINES", g_BABY_LINES, s_BABY_LINES);
    pyrt::xrefs::reg_access("compose.harm", "CHILD_LINES", g_CHILD_LINES, s_CHILD_LINES);
    pyrt::xrefs::reg_access("compose.harm", "HURTING", g_HURTING, s_HURTING);
    pyrt::xrefs::reg_access("compose.harm", "NAMED", g_NAMED, s_NAMED);
    pyrt::xrefs::reg_access("compose.harm", "OUTBURST", g_OUTBURST, s_OUTBURST);
    pyrt::xrefs::reg_access("compose.harm", "OUTBURST_VERB", g_OUTBURST_VERB, s_OUTBURST_VERB);
    pyrt::xrefs::reg_access("compose.harm", "OUTBURST_WALL", g_OUTBURST_WALL, s_OUTBURST_WALL);
    pyrt::xrefs::reg_access("compose.harm", "POOL", g_POOL, s_POOL);
    pyrt::xrefs::reg_access("compose.harm", "_BARE_LEAD", g__BARE_LEAD, s__BARE_LEAD);
    pyrt::xrefs::reg_access("compose.harm", "_BODY", g__BODY, s__BODY);
    pyrt::xrefs::reg_access("compose.harm", "_FEELINGS", g__FEELINGS, s__FEELINGS);
    pyrt::xrefs::reg_access("compose.harm", "_HURT", g__HURT, s__HURT);
    pyrt::xrefs::reg_access("compose.harm", "_IDIOM", g__IDIOM, s__IDIOM);
    pyrt::xrefs::reg_access("compose.harm", "_IN_LAW", g__IN_LAW, s__IN_LAW);
    pyrt::xrefs::reg_access("compose.harm", "_JOINERS", g__JOINERS, s__JOINERS);
    pyrt::xrefs::reg_access("compose.harm", "_PERSON", g__PERSON, s__PERSON);
    pyrt::xrefs::reg_access("compose.harm", "_THE_PERSON", g__THE_PERSON, s__THE_PERSON);
    pyrt::xrefs::reg_access("compose.harm", "_WISH", g__WISH, s__WISH);
    pyrt::xrefs::reg_access("compose.harm", "pick", g_pick, s_pick);
}
