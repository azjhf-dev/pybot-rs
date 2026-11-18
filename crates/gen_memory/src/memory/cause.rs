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


thread_local! { static G_A_BODY_OR_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GUESS_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HESITATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_AT_THE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REASON_NOBODY_HAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REASON_TOOK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STOCK_LOW_POOLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SWAP_TO_YOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_REASON_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEATED_ON_PURPOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_BODY_OR_A_PERSON() -> Py { ensure_init(); G_A_BODY_OR_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BODY_OR_A_PERSON(v: Py) { ensure_init(); G_A_BODY_OR_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GUESS_IN_FRONT() -> Py { ensure_init(); G_A_GUESS_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GUESS_IN_FRONT(v: Py) { ensure_init(); G_A_GUESS_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HESITATION() -> Py { ensure_init(); G_A_HESITATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HESITATION(v: Py) { ensure_init(); G_A_HESITATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOD() -> Py { ensure_init(); G_A_NOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOD(v: Py) { ensure_init(); G_A_NOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_AT_THE_END() -> Py { ensure_init(); G_A_QUESTION_AT_THE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_AT_THE_END(v: Py) { ensure_init(); G_A_QUESTION_AT_THE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_WORD() -> Py { ensure_init(); G_A_QUESTION_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_WORD(v: Py) { ensure_init(); G_A_QUESTION_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REASON_NOBODY_HAS() -> Py { ensure_init(); G_A_REASON_NOBODY_HAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REASON_NOBODY_HAS(v: Py) { ensure_init(); G_A_REASON_NOBODY_HAS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REASON_TOOK() -> Py { ensure_init(); G_A_REASON_TOOK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REASON_TOOK(v: Py) { ensure_init(); G_A_REASON_TOOK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STOCK_LOW_POOLS() -> Py { ensure_init(); G_A_STOCK_LOW_POOLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STOCK_LOW_POOLS(v: Py) { ensure_init(); G_A_STOCK_LOW_POOLS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SWAP_TO_YOURS() -> Py { ensure_init(); G_A_SWAP_TO_YOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SWAP_TO_YOURS(v: Py) { ensure_init(); G_A_SWAP_TO_YOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING() -> Py { ensure_init(); G_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING(v: Py) { ensure_init(); G_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERB() -> Py { ensure_init(); G_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERB(v: Py) { ensure_init(); G_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_SAID() -> Py { ensure_init(); G_NOTHING_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_SAID(v: Py) { ensure_init(); G_NOTHING_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_REASON() -> Py { ensure_init(); G_NOT_A_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_REASON(v: Py) { ensure_init(); G_NOT_A_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_REASON_PHRASES() -> Py { ensure_init(); G_NOT_A_REASON_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_REASON_PHRASES(v: Py) { ensure_init(); G_NOT_A_REASON_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_IDEA() -> Py { ensure_init(); G_NO_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_IDEA(v: Py) { ensure_init(); G_NO_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_REASON() -> Py { ensure_init(); G_NO_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_REASON(v: Py) { ensure_init(); G_NO_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEATED_ON_PURPOSE() -> Py { ensure_init(); G_REPEATED_ON_PURPOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEATED_ON_PURPOSE(v: Py) { ensure_init(); G_REPEATED_ON_PURPOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE() -> Py { ensure_init(); G_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE(v: Py) { ensure_init(); G_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_PHRASES() -> Py { ensure_init(); G_STATE_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_PHRASES(v: Py) { ensure_init(); G_STATE_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_WORDS() -> Py { ensure_init(); G_STATE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_WORDS(v: Py) { ensure_init(); G_STATE_WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A bare thing, offered as the reason for the state they just reported.\n\nPeople talk in shorthands with someone who was listening a moment ago. A woman says she is\nknackered, asks the bot why it thinks so, and answers the bot's own question with two words:\n\n```\nim knackered\n  -> Long one? Give yourself half an hour doing nothing before the next thing.\nwhy\n  -> I'd be guessing at the reason - the cause is not something I can see from here. Which part\n     of that do you want the why for?\nthe baby\n  -> I'm listening — say more if it helps.\n```\n\n\"The baby\" is the answer to the question the bot just asked. It is not a fragment nobody can\nuse: it is a reason, and one the bot has plenty to say about. Said as the full statement it is\n(\"im knackered, the baby\"), the knowledge base answers it with the broken nights and the shift\nthat gets each of them a block of sleep. The whole of the failure is that the words the person\nleft out were the words that made it a sentence.\n\nSo this reads the line against the state they reported: the phrase is joined to it, and the\nline that comes out is answered as a line of its own.\n\nThe rule is narrow on purpose:\n* the line has to be a thing and nothing else - no verb, no question, at most four words - so\n  \"and the baby needs a feed at 3\" stays a line of its own;\n* it has to follow the bot's own turn, so a bare \"the garden\" as the opening line - or straight\n  after the person's own words - is not read as a reason for anything;\n* there has to be a state reported in the last three turns (\"im knackered\", \"i feel low\",\n  \"ive been stressed\"), and the state has to be one of the words people use for how they are,\n  so \"im thinking of a dog\" is not a state to be given a reason for;\n* the answer has to be one the bot has not just given, so this cannot turn a shrug into the\n  same line twice.\n");
    s_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m|i am|i feel|i'?ve been|i have been|i'?ve had|i have had|i had|it'?s been|its been|feel|feeling|been)\\s+(?:(?:so|very|really|proper|properly|totally|completely|absolutely|a bit|a little|well|fairly|quite|pretty|dead|bloody|seriously|still|just|always|knackered|feeling)\\s+)*(?:(?:having|had)\\s+)?(?:(?:a|an|the)\\s+)?([a-z']+(?:\\s+up)?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\ntired knackered shattered exhausted drained worn weary stressed stressed-out anxious worried\nnervous low down depressed miserable unhappy sad lonely bored fed up hacked off angry cross\nannoyed upset rubbish awful terrible rough ill sick poorly unwell sore achy achey hurting\nskint broke strapped poor lost stuck overwhelmed snowed swamped\nflat empty numb crap\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_PHRASES(py_tuple(vec![py_str("in a state"), py_str("at the end of my tether"), py_str("at the end"), py_str("under the weather"), py_str("run down"), py_str("worn out"), py_str("fed up"), py_str("hacked off"), py_str("on my knees"), py_str("all over the place"), py_str("not sleeping"), py_str("not been sleeping"), py_str("short of money"), py_str("hard up"), py_str("out of work"), py_str("off sick"), py_str("bad day"), py_str("rough day"), py_str("hard day"), py_str("long day"), py_str("tough day"), py_str("off day"), py_str("down day"), py_str("flat day"), py_str("rubbish week"), py_str("bad week"), py_str("rough week"), py_str("hard week"), py_str("long week")]));
    s_A_QUESTION_AT_THE_END(stdlib_call("re", "compile", &[py_str("\\?\\s*[\\\"'”’)]?\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING(stdlib_call("re", "compile", &[py_str("^(?:(?:just|only|mostly|mainly|probably|definitely|honestly|mostly just|just the|probably the|i think|i suppose|suppose|maybe)\\s+)*(?:the|my|our|his|her|their|this|that|a|an|no|not having|having|being|work|money|sleep|kids|family|everything|it|me|us)?\\s*([a-z' ]{1,40})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GUESS_IN_FRONT(stdlib_call("re", "compile", &[py_str("^(?:(?:maybe|probably|likely|definitely|honestly|really|just|only|actually|possibly|must be|its|it's|i think|i suppose|suppose|i reckon|reckon|though|but|and|also)\\s+)+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_REASON(call_builtin("frozenset", &[mcall(&py_str("\neverything everyone everybody anything nothing something life stuff things\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_REASON_PHRASES(py_tuple(vec![py_str("it all"), py_str("all of it"), py_str("the world"), py_str("the lot"), py_str("everything else"), py_str("everything at once")]));
    s_A_SWAP_TO_YOURS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bmy\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("your")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bour\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("your")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bhis\\b|\\bher\\b|\\btheir\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("your")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bme\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("you")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bi\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("you")])]));
    s_NO_IDEA(stdlib_call("re", "compile", &[py_str("\\b(?:do(?:n'?t| not)|doesnt|doesn'?t|didnt|didn'?t|dunno|dun+o)\\s+(?:really\\s+)?know\\b|\\bno idea\\b|\\bnot (?:sure|a clue)\\b|\\bwho knows\\b|\\bgod knows\\b|\\bbeats me\\b|\\bno clue\\b|\\bsearch me\\b|\\bhaven'?t a clue\\b|\\bnot got a clue\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BODY_OR_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:(?:my|his|her|their|the)\\s+)?(?:head|heads|back|knee|knees|shoulder|shoulders|hip|hips|neck|stomach|belly|gut|tooth|teeth|gum|gums|foot|feet|ankle|ankles|arm|arms|leg|legs|hand|hands|wrist|elbow|ear|ears|throat|chest|skin|eye|eyes|migraine|migraines|cough|cold|flu|toothache|backache|sciatica|arthritis)\\b|\\b(?:my|our|his|her|their)\\s+(?:husband|wife|partner|boyfriend|girlfriend|mum|mom|mam|mother|dad|father|son|daughter|child|kid|kids|baby|brother|sister|friend|mate|boss|colleague|nan|gran|grandma|grandad|aunt|uncle|cousin)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nyes yeh yeah yep ok okay k cool nice right sure no nope nah nothing none nothing much\nthanks thank you ta cheers hi hello hey hmm mm mmm oh ah sorry alright fine definitely\nmaybe probably dont know dunno what why how who when where l don't know not sure\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NOD(op_bitor(&op_bitor(&crate::memory::topics::g_ACKS(), &crate::memory::topics::g_STOPWORDS()), &call_builtin("frozenset", &[mcall(&py_str("\ngood great brilliant brill ace excellent amazing wonderful lovely loverly luvly fab fabulous\nsuper top sorted sound grand mint quality class legend star stuff things one\nmate matey fella boss hun love dude bud pal thats that thank ta cheers much many lot lots\nha haha lol lols hehe x xx xxx aye yeh yep yes yeah ok okay kk alright then so well too anyway\nanyhow though also still just only really honestly actually truly right fine\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_NO_REASON(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:(?:i\\s+)?(?:really|honestly|actually|genuinely)\\s+)?(?:i\\s+)?(?:(?:dont|don'?t|do not)\\s+(?:really\\s+)?know|(?:have\\s+)?no\\s+idea|(?:i\\s*(?:'m|m| am)\\s+)?(?:not|cant|can'?t|cannot)\\s+(?:really\\s+)?(?:sure|explain|say|tell)|dunno)\\s+why\\b|(?:but\\s+|and\\s+|so\\s+)?(?:nothing|nowt)(?:\\s+(?:much|really|at all))?\\s+(?:has\\s+|had\\s+)?happened\\b|(?:for\\s+)?(?:no|not\\s+any|without\\s+a)\\s+(?:particular\\s+|real\\s+|obvious\\s+)?reason\\b|(?:i\\s+)?(?:dont|don'?t|do not)\\s+(?:really\\s+)?know|(?:have\\s+)?no\\s+idea|dunno)(?:\\s+(?:really|at all|though|today|honestly|maybe|i suppose))*\\s*[.!?]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|be|been|am|do|does|did|have|has|had|will|would|can|could|should|shall|may|might|must|need|needs|needed|want|wants|wanted|got|get|gets|getting|go|goes|went|make|makes|made|take|takes|took|keep|keeps|kept|wake|wakes|woke|feed|feeds|cries|cried|come|comes|came|start|starts|started|stop|stops|stopped|work|works|working|left|leave|leaves|lost|lose|loses|hate|hates|love|loves|said|says|tell|told)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUESTION_WORD(stdlib_call("re", "compile", &[py_str("^\\W*(?:what|whats|what's|why|how|hows|how's|when|whens|where|who|whos|which|whose|do|does|did|is|are|was|were|can|could|would|should|will|shall|have|has|had|am)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPEATED_ON_PURPOSE(stdlib_call("re", "compile", &[py_str("^\\W*(?:same (?:answer|as|one)|it'?s what i said|as i said|said already|that is the same|the same as|again:)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m listening|say more|not sure about that one|don'?t know enough|beyond what i know|rather not guess|everything i have on that one|not much (?:i can add|to go on)|which bit do you|tell me more|i'?m here if you want|anything you'?d like a hand with|thanks for telling me)\\b|\\u2014\\s*noted\\.|\\s-\\s*noted\\."), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HESITATION(stdlib_call("re", "compile", &[py_str("^\\W*(?:i\\s*(?:'m|m| am)\\s+)?(?:not\\s+(?:really\\s+|too\\s+|so\\s+)?sure|unsure|not\\s+(?:really\\s+)?certain|(?:i\\s+)?(?:dont|don'?t|do not)\\s+(?:really\\s+)?know|dunno|no\\s+idea)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REASON_TOOK(py_tuple(vec![py_str("{reason} can do that to you. What is the worst of it at the moment?"), py_str("That is a lot to carry, and {reason} would wear anyone down."), py_str("{reason} - that is the usual one. Is it worse now than it was a month ago?"), py_str("Anyone would be worn down by {reason}. What is the worst part of it just now?")]));
    s_A_REASON_NOBODY_HAS(py_tuple(vec![py_str("There's often no reason you could point at. It's still worth telling me."), py_str("That's alright — a lot of it arrives without a why."), py_str("You don't have to have the reason for it. Say what it's like and we'll go from there."), py_str("Plenty of it has no cause you could name. It still counts.")]));
    s_A_STOCK_LOW_POOLS(py_tuple(vec![py_str("sad")]));
}
pub fn _flat(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn _turns(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _the_state_they_reported(args: &[Py]) -> Py {
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
    let mut back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "back") { v.clone() }
    else { Py::Int(4) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The state line, said as it was said, from the last few turns (newest first).");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&_turns(&[memory.clone()]), &op_neg(&back.clone()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = _flat(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&g_STATE_PHRASES());
    for __item in __iter2 {
    phrase = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![line.clone(), phrase.clone()]);
    }
    }
    found = mcall(&g_STATE(), "search", &[line.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STATE_WORDS(), &mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]))) } }) {
    return py_tuple(vec![line.clone(), mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[])]);
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _the_bot_just_spoke(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The bot's line on the turn before, or \"\" when it said nothing.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&_turns(&[memory.clone()]), &op_neg(&Py::Int(2)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _already_has_it(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut line: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The bot's own line already names the thing: \"The baby - so that is the part behind it.\"\n\n    It has been understood, and a question of its own was asked about it, so the reason is not\n    something the bot is missing. Answering the state line again over the top of that question\n    talks past the person instead of with them.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    line = op_add(&op_add(&py_str(" "), &_flat(&[said.clone()])), &py_str(" "));
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&_flat(&[thing.clone()]), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("a"), py_str("an"), py_str("this"), py_str("that"), py_str("just"), py_str("only"), py_str("mostly"), py_str("mainly"), py_str("probably"), py_str("definitely")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), line.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _only_a_nod(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is nothing but a nod: the store's own ack reading, or every word of it\n    a nod word (\"good stuff\", \"thats grand\", \"brill thanks\", \"loverly\"). It names no thing.");
    if py_truthy(&crate::memory::topics::is_ack(&[text.clone()])) {
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&_flat(&[text.clone()]), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?'")])); } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(Py::Bool(py_contains(&g_A_NOD(), &word.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _a_thing(args: &[Py]) -> Py {
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
    let mut body: Py = Py::None;
    let mut low: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing itself, said the way they said it, or None when the line is more than a thing.");
    low = mcall(&_flat(&[text.clone()]), "strip", &[py_str(" .!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { mcall(&low.clone(), "endswith", &[py_str("?")]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_HESITATION(), "match", &[low.clone()])) {
    low = mcall(&mcall(&g_A_HESITATION(), "sub", &[py_str(""), low.clone()]), "strip", &[py_str(" .,:;-")]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    }
    if py_truthy(&mcall(&g_A_QUESTION_WORD(), "match", &[low.clone()])) {
    return Py::None;
    }
    body = { let __b = mcall(&mcall(&g_A_GUESS_IN_FRONT(), "sub", &[py_str(""), low.clone()]), "strip", &[]); if py_truthy(&__b) { __b } else { low.clone() } };
    body = { let __b = mcall(&mcall(&g_A_GUESS_IN_FRONT(), "sub", &[py_str(""), body.clone()]), "strip", &[]); if py_truthy(&__b) { __b } else { body.clone() } };
    words = mcall(&body.clone(), "split", &[]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &body.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &mcall(&py_str(" "), "join", &[words.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { mcall(&g_A_VERB(), "search", &[body.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_THING(), "match", &[body.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_only_a_nod(&[body.clone()])) {
    return Py::None;
    }
    low = body.clone();
    words = mcall(&low.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("this"), py_str("that"), py_str("a"), py_str("an"), py_str("it"), py_str("just"), py_str("only"), py_str("mostly"), py_str("mainly"), py_str("probably"), py_str("definitely"), py_str("honestly"), py_str("maybe"), py_str("i"), py_str("think"), py_str("suppose")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }))) {
    return Py::None;
    }
    return low.clone();
    Py::None
}
pub fn expand(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut spoke: Py = Py::None;
    let mut state: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The full statement this bare thing is the reason for, or None.\n\n    None whenever the line is not a thing said as a reason: a line of its own, a line with no\n    state behind it to be a reason for, or a line after a bot turn that asked nothing.\n    ");
    thing = _a_thing(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    spoke = _the_bot_just_spoke(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&spoke.clone()))) {
    return Py::None;
    }
    if py_truthy(&_already_has_it(&[spoke.clone(), thing.clone()])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_state_they_reported(&[memory.clone()]), 2, None);
    said = __unpk1[0].clone();
    word = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&word.clone())) } }) {
    return Py::None;
    }
    state = if py_truthy(&op_le(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) { said.clone() } else { op_add(&py_str("im "), &word.clone()) };
    return op_add(&op_add(&state.clone(), &py_str(", ")), &thing.clone());
    Py::None
}
pub fn _the_bot_would_only_shrug(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut probe: Py = Py::None;
    let mut said: Py = Py::None;
    let mut state: Py = Py::None;
    let _ = py_str("True when the bot's own reply to these words on their own is a line that says nothing.\n\n    The composer already answers some of these: straight after \"im knackered\", \"the baby\" comes\n    back as \"The baby - so that is the part behind it. What has been hardest about it?\", which is\n    a real reply and not something to be talked over. But two turns later, after the bot's own\n    \"which part do you want the why for?\", nothing picks it up and the person gets \"I'm\n    listening - say more if it helps\". Only the second is a reply to fix, so the composer is\n    asked what it would say and only its shrugs count as a gap.\n    ");
    let __try1 = exc::catch(|| {
    let mut fallback: Py = pyrt::xrefs::xmodule("compose.fallback");
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
    probe = stdlib_call("copy", "deepcopy", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    state = stdlib_call("random", "getstate", &[]).unwrap_or_else(|e| exc::raise(e));
    let __try6 = exc::catch(|| {
    said = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(probe.clone()); __kws.extend(vec![("verdict".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fallback.clone(), "reply_for", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
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
    let _ = stdlib_call("random", "setstate", &[state.clone()]).unwrap_or_else(|e| exc::raise(e));
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    return call_builtin("bool", &[mcall(&g_NOTHING_SAID(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asked(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut names: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { py_tuple(vec![py_str("answers"), py_str("chat")]) };
    let mut module: Py = Py::None;
    let mut name: Py = Py::None;
    let mut reading_memory: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("What the bot would say to this line, from the knowledge base first, then the chat.\n\n    A reason is often a feeling rather than a fact (\"im stressed, money\"), and the feelings are\n    answered by the chat's own readers; a reason with something to know in it (\"im tired, my\n    back\", \"im knackered, the baby\") is answered from the knowledge base. The first one with\n    something to say wins. `names` narrows it to one reader: a feeling with no reason in it is\n    a mood, and the knowledge base's answer to the words \"nothing happened\" is the insomnia\n    entry, which is not about a mood at all.\n    ");
    reading_memory = stdlib_call("copy", "deepcopy", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&reading_memory.clone(), &py_str("understood"), gen_understanding::understanding::understand(&[line.clone(), reading_memory.clone()]));
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    let __try2 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[op_add(&py_str("..modules."), &name.clone()), g___package__()]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&module.clone(), "respond", &[line.clone(), reading_memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOTHING_SAID(), "search", &[said.clone()]))) } } } }) {
    return mcall(&said.clone(), "strip", &[]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_stock_low_line(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pool_line: Py = Py::None;
    let _ = py_str("True when the reply is one of the general lines a low mood is answered with.");
    let __try1 = exc::catch(|| {
    let mut chat: Py = pyrt::xrefs::xmodule("modules.chat");
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
    line = _flat(&[said.clone()]);
    let __iter6 = py_iter_or_raise(&g_A_STOCK_LOW_POOLS());
    for __item in __iter6 {
    name = __item.clone();
    let __iter7 = py_iter_or_raise(&{ let __b = mcall(&gattr(&chat.clone(), "REPLIES"), "get", &[name.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter7 {
    pool_line = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[pool_line.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_flat(&[pool_line.clone()]), &line.clone())) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_the_reason(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut line: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply names the reason at all, so it is a reply to it and not to the mood.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&_flat(&[thing.clone()]), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("a"), py_str("an"), py_str("this"), py_str("that"), py_str("just"), py_str("only"), py_str("mostly"), py_str("mainly")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    line = op_add(&op_add(&py_str(" "), &_flat(&[said.clone()])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    stem = if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) { slice_or_raise(&word.clone(), &Py::None, &Py::Int(5), &Py::None) } else { word.clone() };
    if py_truthy(&stdlib_call("re", "search", &[op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e))), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_nothing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reason: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reason") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reason"))) };
    let mut line: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the reason is a word for every reason at once (\"everything\", \"life\", \"it all\").");
    line = op_add(&op_add(&py_str(" "), &_flat(&[reason.clone()])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_REASON_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&_flat(&[reason.clone()]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("a"), py_str("an"), py_str("this"), py_str("that")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_REASON(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_persons_own_reason(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reason: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reason") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reason"))) };
    let mut pattern: Py = Py::None;
    let mut replacement: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The reason said in the bot's mouth: theirs, so \"my sleep\" and \"her job\" are \"your\".");
    said = _flat(&[reason.clone()]);
    let __iter1 = py_iter_or_raise(&g_A_SWAP_TO_YOURS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    replacement = __unpk2[1].clone();
    said = mcall(&pattern.clone(), "sub", &[replacement.clone(), said.clone()]);
    }
    return said.clone();
    Py::None
}
pub fn _asks_for_the_rest(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks for another option or the rest of the answer, naming nothing.");
    let __try1 = exc::catch(|| {
    let mut another_module: Py = crate::memory::another::module_obj();
    return py_tuple(vec![Py::Bool(true), mcall(&another_module.clone(), "names_nothing", &[text.clone()])]);
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
    Py::None
}
pub fn _a_reason_for_how_they_are(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut body: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut line: Py = Py::None;
    let mut reason: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A warm line about the reason they named, from the reason itself, or None.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&_flat(&[thing.clone()]), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("a"), py_str("an"), py_str("this"), py_str("that"), py_str("just"), py_str("only"), py_str("mostly"), py_str("mainly")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::None;
    }
    if py_truthy(&_names_nothing(&[thing.clone()])) {
    return Py::None;
    }
    if py_truthy(&_asks_for_the_rest(&[thing.clone()])) {
    return Py::None;
    }
    body = pyrt::xrefs::xcall("compose", "pick", &[g_A_REASON_TOOK(), memory.clone(), thing.clone()]);
    reason = _the_persons_own_reason(&[thing.clone()]);
    said = mcall(&body.clone(), "replace", &[py_str("{reason}"), if py_truthy(&mcall(&body.clone(), "startswith", &[py_str("{reason}")])) { op_add(&mcall(&geti(&reason.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&reason.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { reason.clone() }]);
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    kept = mcall(&memory.clone(), "get", &[py_str("repeated")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kept.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&kept.clone(), &said.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&_turns(&[memory.clone()]), &op_neg(&Py::Int(2)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&line.clone()) {
    if py_truthy(&Py::Bool(py_eq(&_flat(&[line.clone()]), &_flat(&[said.clone()])))) {
    return Py::None;
    }
    break;
    }
    }
    return said.clone();
    Py::None
}
pub fn _without_what_was_just_said(args: &[Py]) -> Py {
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
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut kept: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentences of an answer the person has not just read, in order.");
    seen = _flat(&[last.clone()]);
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&said.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&seen.clone(), &_flat(&[sentence.clone()]))) } }) {
    let _ = mcall(&kept.clone(), "append", &[mcall(&sentence.clone(), "strip", &[])]);
    }
    }
    return mcall(&py_str(" "), "join", &[kept.clone()]);
    Py::None
}
pub fn _a_new_answer(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The same line, unless the bot has just said it or is framing it as a repeat.");
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    last = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&_turns(&[memory.clone()]), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    last = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&last.clone()) {
    break;
    }
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_flat(&[said.clone()]), &_flat(&[last.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&last.clone()) {
    said = _without_what_was_just_said(&[said.clone(), last.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    }
    if py_truthy(&Py::Bool(py_contains(&{ let __b = mcall(&memory.clone(), "get", &[py_str("repeated")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &said.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_REPEATED_ON_PURPOSE(), "match", &[said.clone()])) {
    return Py::None;
    }
    return said.clone();
    Py::None
}
pub fn _the_feeling_without_a_reason(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut said: Py = Py::None;
    let mut state: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The state they reported, answered again with the missing reason said out loud.\n\n    \"im feeling really down today\" / \"i dont know why\" is not a fragment nobody can use: the\n    second line is the answer to the question a friend would ask, and the bot already knows the\n    state it belongs to. Joined to that state it is a line the ordinary readers can answer -\n    the feeling pool for a mood, and the knowledge base for tiredness - instead of the stock\n    line the floor gives a line that names nothing.\n    ");
    let __unpk1 = py_unpack(&_the_state_they_reported(&[memory.clone()]), 2, None);
    said = __unpk1[0].clone();
    word = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&word.clone())) } }) {
    return Py::None;
    }
    state = if py_truthy(&op_le(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) { said.clone() } else { op_add(&py_str("im "), &word.clone()) };
    answer = _a_new_answer(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&state.clone(), &py_str(", and i don't know why"))); __args.push(memory.clone()); __kws.extend(vec![("names".to_string(), py_tuple(vec![py_str("answers"), py_str("chat")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked(args))(&__args) }, memory.clone()]);
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    return _the_reason_nobody_has(&[memory.clone()]);
    Py::None
}
pub fn _the_reason_nobody_has(args: &[Py]) -> Py {
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
    let mut here: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut last: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("A line about there being no reason, not the one just said.");
    said = pyrt::xrefs::xcall("compose", "pick", &[g_A_REASON_NOBODY_HAS(), memory.clone(), py_str("no reason")]);
    kept = mcall(&memory.clone(), "get", &[py_str("repeated")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kept.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&kept.clone(), &said.clone())) } }) {
    return Py::None;
    }
    last = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&_turns(&[memory.clone()]), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    last = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&last.clone()) {
    break;
    }
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_flat(&[said.clone()]), &_flat(&[last.clone()]))) } }) {
    pool = call_builtin("list", &[g_A_REASON_NOBODY_HAS()]).unwrap_or_else(|e| exc::raise(e));
    here = if py_truthy(&Py::Bool(py_contains(&pool.clone(), &said.clone()))) { mcall(&pool.clone(), "index", &[said.clone()]) } else { Py::Int(0) };
    said = geti(&pool.clone(), &op_modulo(&op_add(&here.clone(), &Py::Int(1)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e))));
    }
    return said.clone();
    Py::None
}
pub fn _the_knowledge_base_answers_it(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut module: Py = Py::None;
    let mut reading_memory: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the knowledge base has an answer to the bare thing on its own.\n\n    The join is for words nothing answers (\"the baby\", \"work\", \"money\"). A thing with an entry\n    of its own is a line of its own, and the entry is the answer the person should get.\n    ");
    let __try1 = exc::catch(|| {
    module = stdlib_call("importlib", "import_module", &[py_str("..modules.answers"), g___package__()]).unwrap_or_else(|e| exc::raise(e));
    reading_memory = stdlib_call("copy", "deepcopy", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&reading_memory.clone(), &py_str("understood"), gen_understanding::understanding::understand(&[thing.clone(), reading_memory.clone()]));
    said = mcall(&module.clone(), "respond", &[thing.clone(), reading_memory.clone()]);
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
    return call_builtin("bool", &[{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&said.clone(), "strip", &[]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_about_the_bot(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let _ = py_str("True when the line is the question about the bot's last line, and the store can answer it.\n\n    Both halves matter. The reading says the line is about the bot's last line rather than a\n    reason for how they feel; the question only stands this reader down when the meant reader\n    has something to say back, because a question it cannot answer is still a line somebody\n    typed, and standing down for it left \"im knackered\" / \"wdym\" with the search hedge where\n    main answered the tiredness card.\n    ");
    let __try1 = exc::catch(|| {
    reading = gen_understanding::understanding::understand(&[text.clone(), memory.clone()]);
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
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("topic")]), &py_str("about-the-bot"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("kind")]), &py_str("question"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reading.clone(), "get", &[py_str("entities")]))) } } } }))) {
    return Py::Bool(false);
    }
    let __try6 = exc::catch(|| {
    let mut meant: Py = crate::memory::meant::module_obj();
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&meant.clone(), "answer_for", &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut plain: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut warmer: Py = Py::None;
    let _ = py_str("The reply to the reason they named, or None when there is nothing to answer it with.");
    if py_truthy(&_asks_about_the_bot(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_NO_REASON(), "match", &[_flat(&[text.clone()])])) {
    return _the_feeling_without_a_reason(&[memory.clone()]);
    }
    reading = expand(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&reading.clone()))) {
    return Py::None;
    }
    thing = _a_thing(&[text.clone()]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { _the_knowledge_base_answers_it(&[thing.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_bot_would_only_shrug(&[thing.clone(), memory.clone()]))) } }) {
    return Py::None;
    }
    said = _asked(&[reading.clone(), memory.clone()]);
    plain = { let __b = call_builtin("bool", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NO_IDEA(), "search", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]))) } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return if py_truthy(&plain.clone()) { _a_reason_for_how_they_are(&[thing.clone(), memory.clone()]) } else { Py::None };
    }
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_BODY_OR_A_PERSON(), "search", &[thing.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = _a_stock_low_line(&[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_the_reason(&[said.clone(), thing.clone()]))) } } } } } }) {
    warmer = _a_reason_for_how_they_are(&[thing.clone(), memory.clone()]);
    if py_truthy(&warmer.clone()) {
    return warmer.clone();
    }
    }
    return _a_new_answer(&[said.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.cause".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BODY_OR_A_PERSON" => Some(g_A_BODY_OR_A_PERSON()),
            "A_GUESS_IN_FRONT" => Some(g_A_GUESS_IN_FRONT()),
            "A_HESITATION" => Some(g_A_HESITATION()),
            "A_NOD" => Some(g_A_NOD()),
            "A_QUESTION_AT_THE_END" => Some(g_A_QUESTION_AT_THE_END()),
            "A_QUESTION_WORD" => Some(g_A_QUESTION_WORD()),
            "A_REASON_NOBODY_HAS" => Some(g_A_REASON_NOBODY_HAS()),
            "A_REASON_TOOK" => Some(g_A_REASON_TOOK()),
            "A_STOCK_LOW_POOLS" => Some(g_A_STOCK_LOW_POOLS()),
            "A_SWAP_TO_YOURS" => Some(g_A_SWAP_TO_YOURS()),
            "A_THING" => Some(g_A_THING()),
            "A_VERB" => Some(g_A_VERB()),
            "NOTHING_SAID" => Some(g_NOTHING_SAID()),
            "NOT_A_REASON" => Some(g_NOT_A_REASON()),
            "NOT_A_REASON_PHRASES" => Some(g_NOT_A_REASON_PHRASES()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NO_IDEA" => Some(g_NO_IDEA()),
            "NO_REASON" => Some(g_NO_REASON()),
            "REPEATED_ON_PURPOSE" => Some(g_REPEATED_ON_PURPOSE()),
            "STATE" => Some(g_STATE()),
            "STATE_PHRASES" => Some(g_STATE_PHRASES()),
            "STATE_WORDS" => Some(g_STATE_WORDS()),
            "_a_new_answer" => Some(py_func("_a_new_answer", |args: &[Py]| _a_new_answer(args))),
            "_a_reason_for_how_they_are" => Some(py_func("_a_reason_for_how_they_are", |args: &[Py]| _a_reason_for_how_they_are(args))),
            "_a_stock_low_line" => Some(py_func("_a_stock_low_line", |args: &[Py]| _a_stock_low_line(args))),
            "_a_thing" => Some(py_func("_a_thing", |args: &[Py]| _a_thing(args))),
            "_already_has_it" => Some(py_func("_already_has_it", |args: &[Py]| _already_has_it(args))),
            "_asked" => Some(py_func("_asked", |args: &[Py]| _asked(args))),
            "_asks_about_the_bot" => Some(py_func("_asks_about_the_bot", |args: &[Py]| _asks_about_the_bot(args))),
            "_asks_for_the_rest" => Some(py_func("_asks_for_the_rest", |args: &[Py]| _asks_for_the_rest(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_names_nothing" => Some(py_func("_names_nothing", |args: &[Py]| _names_nothing(args))),
            "_only_a_nod" => Some(py_func("_only_a_nod", |args: &[Py]| _only_a_nod(args))),
            "_says_the_reason" => Some(py_func("_says_the_reason", |args: &[Py]| _says_the_reason(args))),
            "_the_bot_just_spoke" => Some(py_func("_the_bot_just_spoke", |args: &[Py]| _the_bot_just_spoke(args))),
            "_the_bot_would_only_shrug" => Some(py_func("_the_bot_would_only_shrug", |args: &[Py]| _the_bot_would_only_shrug(args))),
            "_the_feeling_without_a_reason" => Some(py_func("_the_feeling_without_a_reason", |args: &[Py]| _the_feeling_without_a_reason(args))),
            "_the_knowledge_base_answers_it" => Some(py_func("_the_knowledge_base_answers_it", |args: &[Py]| _the_knowledge_base_answers_it(args))),
            "_the_persons_own_reason" => Some(py_func("_the_persons_own_reason", |args: &[Py]| _the_persons_own_reason(args))),
            "_the_reason_nobody_has" => Some(py_func("_the_reason_nobody_has", |args: &[Py]| _the_reason_nobody_has(args))),
            "_the_state_they_reported" => Some(py_func("_the_state_they_reported", |args: &[Py]| _the_state_they_reported(args))),
            "_turns" => Some(py_func("_turns", |args: &[Py]| _turns(args))),
            "_without_what_was_just_said" => Some(py_func("_without_what_was_just_said", |args: &[Py]| _without_what_was_just_said(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "expand" => Some(py_func("expand", |args: &[Py]| expand(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BODY_OR_A_PERSON" => s_A_BODY_OR_A_PERSON(v),
            "A_GUESS_IN_FRONT" => s_A_GUESS_IN_FRONT(v),
            "A_HESITATION" => s_A_HESITATION(v),
            "A_NOD" => s_A_NOD(v),
            "A_QUESTION_AT_THE_END" => s_A_QUESTION_AT_THE_END(v),
            "A_QUESTION_WORD" => s_A_QUESTION_WORD(v),
            "A_REASON_NOBODY_HAS" => s_A_REASON_NOBODY_HAS(v),
            "A_REASON_TOOK" => s_A_REASON_TOOK(v),
            "A_STOCK_LOW_POOLS" => s_A_STOCK_LOW_POOLS(v),
            "A_SWAP_TO_YOURS" => s_A_SWAP_TO_YOURS(v),
            "A_THING" => s_A_THING(v),
            "A_VERB" => s_A_VERB(v),
            "NOTHING_SAID" => s_NOTHING_SAID(v),
            "NOT_A_REASON" => s_NOT_A_REASON(v),
            "NOT_A_REASON_PHRASES" => s_NOT_A_REASON_PHRASES(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NO_IDEA" => s_NO_IDEA(v),
            "NO_REASON" => s_NO_REASON(v),
            "REPEATED_ON_PURPOSE" => s_REPEATED_ON_PURPOSE(v),
            "STATE" => s_STATE(v),
            "STATE_PHRASES" => s_STATE_PHRASES(v),
            "STATE_WORDS" => s_STATE_WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.cause", module_obj);
    pyrt::xrefs::reg_fn("memory.cause", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_turns", py_func("_turns", |args: &[Py]| _turns(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_the_state_they_reported", py_func("_the_state_they_reported", |args: &[Py]| _the_state_they_reported(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_the_bot_just_spoke", py_func("_the_bot_just_spoke", |args: &[Py]| _the_bot_just_spoke(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_already_has_it", py_func("_already_has_it", |args: &[Py]| _already_has_it(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_only_a_nod", py_func("_only_a_nod", |args: &[Py]| _only_a_nod(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_a_thing", py_func("_a_thing", |args: &[Py]| _a_thing(args)));
    pyrt::xrefs::reg_fn("memory.cause", "expand", py_func("expand", |args: &[Py]| expand(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_the_bot_would_only_shrug", py_func("_the_bot_would_only_shrug", |args: &[Py]| _the_bot_would_only_shrug(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_asked", py_func("_asked", |args: &[Py]| _asked(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_a_stock_low_line", py_func("_a_stock_low_line", |args: &[Py]| _a_stock_low_line(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_says_the_reason", py_func("_says_the_reason", |args: &[Py]| _says_the_reason(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_names_nothing", py_func("_names_nothing", |args: &[Py]| _names_nothing(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_the_persons_own_reason", py_func("_the_persons_own_reason", |args: &[Py]| _the_persons_own_reason(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_asks_for_the_rest", py_func("_asks_for_the_rest", |args: &[Py]| _asks_for_the_rest(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_a_reason_for_how_they_are", py_func("_a_reason_for_how_they_are", |args: &[Py]| _a_reason_for_how_they_are(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_without_what_was_just_said", py_func("_without_what_was_just_said", |args: &[Py]| _without_what_was_just_said(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_a_new_answer", py_func("_a_new_answer", |args: &[Py]| _a_new_answer(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_the_feeling_without_a_reason", py_func("_the_feeling_without_a_reason", |args: &[Py]| _the_feeling_without_a_reason(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_the_reason_nobody_has", py_func("_the_reason_nobody_has", |args: &[Py]| _the_reason_nobody_has(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_the_knowledge_base_answers_it", py_func("_the_knowledge_base_answers_it", |args: &[Py]| _the_knowledge_base_answers_it(args)));
    pyrt::xrefs::reg_fn("memory.cause", "_asks_about_the_bot", py_func("_asks_about_the_bot", |args: &[Py]| _asks_about_the_bot(args)));
    pyrt::xrefs::reg_fn("memory.cause", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("memory.cause", "A_BODY_OR_A_PERSON", g_A_BODY_OR_A_PERSON, s_A_BODY_OR_A_PERSON);
    pyrt::xrefs::reg_access("memory.cause", "A_GUESS_IN_FRONT", g_A_GUESS_IN_FRONT, s_A_GUESS_IN_FRONT);
    pyrt::xrefs::reg_access("memory.cause", "A_HESITATION", g_A_HESITATION, s_A_HESITATION);
    pyrt::xrefs::reg_access("memory.cause", "A_NOD", g_A_NOD, s_A_NOD);
    pyrt::xrefs::reg_access("memory.cause", "A_QUESTION_AT_THE_END", g_A_QUESTION_AT_THE_END, s_A_QUESTION_AT_THE_END);
    pyrt::xrefs::reg_access("memory.cause", "A_QUESTION_WORD", g_A_QUESTION_WORD, s_A_QUESTION_WORD);
    pyrt::xrefs::reg_access("memory.cause", "A_REASON_NOBODY_HAS", g_A_REASON_NOBODY_HAS, s_A_REASON_NOBODY_HAS);
    pyrt::xrefs::reg_access("memory.cause", "A_REASON_TOOK", g_A_REASON_TOOK, s_A_REASON_TOOK);
    pyrt::xrefs::reg_access("memory.cause", "A_STOCK_LOW_POOLS", g_A_STOCK_LOW_POOLS, s_A_STOCK_LOW_POOLS);
    pyrt::xrefs::reg_access("memory.cause", "A_SWAP_TO_YOURS", g_A_SWAP_TO_YOURS, s_A_SWAP_TO_YOURS);
    pyrt::xrefs::reg_access("memory.cause", "A_THING", g_A_THING, s_A_THING);
    pyrt::xrefs::reg_access("memory.cause", "A_VERB", g_A_VERB, s_A_VERB);
    pyrt::xrefs::reg_access("memory.cause", "NOTHING_SAID", g_NOTHING_SAID, s_NOTHING_SAID);
    pyrt::xrefs::reg_access("memory.cause", "NOT_A_REASON", g_NOT_A_REASON, s_NOT_A_REASON);
    pyrt::xrefs::reg_access("memory.cause", "NOT_A_REASON_PHRASES", g_NOT_A_REASON_PHRASES, s_NOT_A_REASON_PHRASES);
    pyrt::xrefs::reg_access("memory.cause", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.cause", "NO_IDEA", g_NO_IDEA, s_NO_IDEA);
    pyrt::xrefs::reg_access("memory.cause", "NO_REASON", g_NO_REASON, s_NO_REASON);
    pyrt::xrefs::reg_access("memory.cause", "REPEATED_ON_PURPOSE", g_REPEATED_ON_PURPOSE, s_REPEATED_ON_PURPOSE);
    pyrt::xrefs::reg_access("memory.cause", "STATE", g_STATE, s_STATE);
    pyrt::xrefs::reg_access("memory.cause", "STATE_PHRASES", g_STATE_PHRASES, s_STATE_PHRASES);
    pyrt::xrefs::reg_access("memory.cause", "STATE_WORDS", g_STATE_WORDS, s_STATE_WORDS);
}
