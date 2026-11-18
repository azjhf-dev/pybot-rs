#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static AN_ASK_AT_THE_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CONDITION_IN_FRONT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EMERGENCY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ILLNESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_LOSS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POOLS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REACTION_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STUCK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static URGENT_ALREADY_GIVEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ANIMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AN_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_BODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ENDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FLAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ILLNESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MAKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOTHING_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OWED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SKINT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _START_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STATE_SOURCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AN_ASK_AT_THE_END() -> Py { ensure_init(); AN_ASK_AT_THE_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ASK_AT_THE_END(v: Py) { ensure_init(); if let Ok(mut w) = AN_ASK_AT_THE_END.write() { *w = v; } }
pub fn g_A_CONDITION_IN_FRONT() -> Py { ensure_init(); A_CONDITION_IN_FRONT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION_IN_FRONT(v: Py) { ensure_init(); if let Ok(mut w) = A_CONDITION_IN_FRONT.write() { *w = v; } }
pub fn g_DEATH() -> Py { ensure_init(); DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATH(v: Py) { ensure_init(); if let Ok(mut w) = DEATH.write() { *w = v; } }
pub fn g_DOING() -> Py { ensure_init(); DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOING(v: Py) { ensure_init(); if let Ok(mut w) = DOING.write() { *w = v; } }
pub fn g_EMERGENCY() -> Py { ensure_init(); EMERGENCY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EMERGENCY(v: Py) { ensure_init(); if let Ok(mut w) = EMERGENCY.write() { *w = v; } }
pub fn g_ENDING() -> Py { ensure_init(); ENDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENDING(v: Py) { ensure_init(); if let Ok(mut w) = ENDING.write() { *w = v; } }
pub fn g_FEELING() -> Py { ensure_init(); FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = FEELING.write() { *w = v; } }
pub fn g_FIRST_LINE() -> Py { ensure_init(); FIRST_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_LINE(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_LINE.write() { *w = v; } }
pub fn g_FIRST_SENTENCE() -> Py { ensure_init(); FIRST_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_SENTENCE.write() { *w = v; } }
pub fn g_HARD() -> Py { ensure_init(); HARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HARD(v: Py) { ensure_init(); if let Ok(mut w) = HARD.write() { *w = v; } }
pub fn g_ILLNESS() -> Py { ensure_init(); ILLNESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ILLNESS(v: Py) { ensure_init(); if let Ok(mut w) = ILLNESS.write() { *w = v; } }
pub fn g_JOB() -> Py { ensure_init(); JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOB(v: Py) { ensure_init(); if let Ok(mut w) = JOB.write() { *w = v; } }
pub fn g_MONEY() -> Py { ensure_init(); MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = MONEY.write() { *w = v; } }
pub fn g_NAMES_A_THING() -> Py { ensure_init(); NAMES_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_A_THING.write() { *w = v; } }
pub fn g_PET_LOSS() -> Py { ensure_init(); PET_LOSS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_LOSS(v: Py) { ensure_init(); if let Ok(mut w) = PET_LOSS.write() { *w = v; } }
pub fn g_PLAN() -> Py { ensure_init(); PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = PLAN.write() { *w = v; } }
pub fn g_POOLS() -> Py { ensure_init(); POOLS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POOLS(v: Py) { ensure_init(); if let Ok(mut w) = POOLS.write() { *w = v; } }
pub fn g_REACTION_FRAME() -> Py { ensure_init(); REACTION_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REACTION_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = REACTION_FRAME.write() { *w = v; } }
pub fn g_SELF() -> Py { ensure_init(); SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF(v: Py) { ensure_init(); if let Ok(mut w) = SELF.write() { *w = v; } }
pub fn g_STUCK() -> Py { ensure_init(); STUCK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STUCK(v: Py) { ensure_init(); if let Ok(mut w) = STUCK.write() { *w = v; } }
pub fn g_URGENT_ALREADY_GIVEN() -> Py { ensure_init(); URGENT_ALREADY_GIVEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_URGENT_ALREADY_GIVEN(v: Py) { ensure_init(); if let Ok(mut w) = URGENT_ALREADY_GIVEN.write() { *w = v; } }
pub fn g__ANIMAL() -> Py { ensure_init(); _ANIMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ANIMAL(v: Py) { ensure_init(); if let Ok(mut w) = _ANIMAL.write() { *w = v; } }
pub fn g__AN_OBJECT() -> Py { ensure_init(); _AN_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AN_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = _AN_OBJECT.write() { *w = v; } }
pub fn g__ASK_BODY() -> Py { ensure_init(); _ASK_BODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_BODY(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_BODY.write() { *w = v; } }
pub fn g__DEATH() -> Py { ensure_init(); _DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEATH(v: Py) { ensure_init(); if let Ok(mut w) = _DEATH.write() { *w = v; } }
pub fn g__ENDING() -> Py { ensure_init(); _ENDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ENDING(v: Py) { ensure_init(); if let Ok(mut w) = _ENDING.write() { *w = v; } }
pub fn g__FEELING() -> Py { ensure_init(); _FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FEELING(v: Py) { ensure_init(); if let Ok(mut w) = _FEELING.write() { *w = v; } }
pub fn g__FLAT() -> Py { ensure_init(); _FLAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FLAT(v: Py) { ensure_init(); if let Ok(mut w) = _FLAT.write() { *w = v; } }
pub fn g__ILLNESS() -> Py { ensure_init(); _ILLNESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ILLNESS(v: Py) { ensure_init(); if let Ok(mut w) = _ILLNESS.write() { *w = v; } }
pub fn g__JOB() -> Py { ensure_init(); _JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__JOB(v: Py) { ensure_init(); if let Ok(mut w) = _JOB.write() { *w = v; } }
pub fn g__LEADS() -> Py { ensure_init(); _LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LEADS(v: Py) { ensure_init(); if let Ok(mut w) = _LEADS.write() { *w = v; } }
pub fn g__MAKING() -> Py { ensure_init(); _MAKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MAKING(v: Py) { ensure_init(); if let Ok(mut w) = _MAKING.write() { *w = v; } }
pub fn g__MONEY() -> Py { ensure_init(); _MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MONEY(v: Py) { ensure_init(); if let Ok(mut w) = _MONEY.write() { *w = v; } }
pub fn g__NOTHING_ELSE() -> Py { ensure_init(); _NOTHING_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOTHING_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = _NOTHING_ELSE.write() { *w = v; } }
pub fn g__OWED() -> Py { ensure_init(); _OWED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OWED(v: Py) { ensure_init(); if let Ok(mut w) = _OWED.write() { *w = v; } }
pub fn g__PLAN() -> Py { ensure_init(); _PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PLAN(v: Py) { ensure_init(); if let Ok(mut w) = _PLAN.write() { *w = v; } }
pub fn g__SELF() -> Py { ensure_init(); _SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SELF(v: Py) { ensure_init(); if let Ok(mut w) = _SELF.write() { *w = v; } }
pub fn g__SKINT() -> Py { ensure_init(); _SKINT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SKINT(v: Py) { ensure_init(); if let Ok(mut w) = _SKINT.write() { *w = v; } }
pub fn g__START_ASK() -> Py { ensure_init(); _START_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__START_ASK(v: Py) { ensure_init(); if let Ok(mut w) = _START_ASK.write() { *w = v; } }
pub fn g__STATE() -> Py { ensure_init(); _STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STATE(v: Py) { ensure_init(); if let Ok(mut w) = _STATE.write() { *w = v; } }
pub fn g__STATE_SOURCE() -> Py { ensure_init(); _STATE_SOURCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STATE_SOURCE(v: Py) { ensure_init(); if let Ok(mut w) = _STATE_SOURCE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Where to start, when the person says they don't know and the hard thing is already on the table.\n\n    my wife left me        -> That sounds rough. If you want, I can help you think through ...\n    i dont know what to do -> Got it. Is there anything you'd like a hand with?\n\nThe second line names nothing new: it carries the first one over. The stock nod answers it with\n\"is there anything you'd like a hand with\", which is exactly what the person has just said they\ncannot do, and it would fit any message in the conversation. What they are asking for is the next\nstep, and the only thing that can say what the next step is, is the line before it - which the\nstore keeps in memory[\"turns\"]. So this reads that line and answers with the first thing a friend\nwould say to that kind of news: money has dates, a death has paperwork and none of it is today, a\ndiagnosis has a question for the doctor.\n\nOnly the line that carries nothing of its own comes here, and only when the turn before it was one\nof those kinds. Anything else returns None, so \"i dont know what to do with my life\" still reaches\nthe direction pool in bot/compose/fallback.py, which is written for it.\n");
    s__NOTHING_ELSE(py_str("(?:\\s+(?:now|next|anymore|any more|at all))?"));
    s__STATE_SOURCE(py_str("(?:im|i am|were|we are)\\s+(?:just\\s+)?(?:so\\s+)?(?:lost|stuck)\\b(?:\\s+(?:in\\s+a\\s+rut|on|with|about)\\s+(?:this|it|that))?"));
    s__STATE(stdlib_call("re", "compile", &[add(&add(&py_str("^\\W*(?:"), &g__STATE_SOURCE()), &py_str(")\\s*(?:\\?|\\.|!)*\\s*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s__LEADS(py_str("(?:(?:and|but|so|well|oh|honestly|then|i mean|like|just|seriously|really)\\s+)*"));
    s__ASK_BODY(add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&add(&py_str("(?:(?:i|we)(?:'ve| have|'m| am)?\\s+(?:just\\s+|really\\s+|still\\s+|honestly\\s+|actually\\s+)*(?:dont|don'?t|do not|doesnt|does not)\\s+know\\s+what\\s+to\\s+do"), &g__NOTHING_ELSE()), &py_str("|(?:(?:i|we)(?:'ve| have|'m| am)?\\s+)?(?:got\\s+|have\\s+)?no\\s+idea\\s+what\\s+to\\s+do")), &g__NOTHING_ELSE()), &py_str("|(?:i|we)(?:'ve| have)?\\s+(?:just\\s+|really\\s+|honestly\\s+)*(?:dont|don'?t|do not)\\s+know\\s+where\\s+to\\s+(?:start|begin|turn)")), &g__NOTHING_ELSE()), &py_str("|(?:i|we)(?:'ve| have)?\\s+(?:no\\s+idea|dont|don'?t|do not)\\s+what\\s+(?:comes\\s+)?next\\b")), &g__NOTHING_ELSE()), &py_str("|what\\s+(?:do|should|shall|can|could)\\s+(?:i|we)\\s+do")), &g__NOTHING_ELSE()), &py_str("|what\\s+(?:am|are)\\s+(?:i|we)\\s+supposed\\s+to\\s+do")), &g__NOTHING_ELSE()), &py_str("|(?:")), &g__STATE_SOURCE()), &py_str("))")));
    s_STUCK(stdlib_call("re", "compile", &[add(&add(&add(&py_str("^\\W*"), &g__LEADS()), &g__ASK_BODY()), &py_str("\\s*(?:\\?|\\.|!)*\\s*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ASK_AT_THE_END(stdlib_call("re", "compile", &[add(&add(&add(&py_str("(?:^|[\\s,;:.!?\\u2014-]+)"), &g__LEADS()), &g__ASK_BODY()), &py_str("\\s*(?:\\?|\\.|!)*\\s*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_REACTION_FRAME(stdlib_call("re", "compile", &[py_str("^(?:(?:oh|well|god|wow|yeah|ok|okay|hmm|blimey|cor|ah|yes|no)\\s+)*(?:that sounds|this sounds|that looks|this looks|that seems|that feels|that must be|that must have been|thats|that's|this is|that is|it is|its|it's|sounds|looks|seems|feels|that|this|it)\\s+(?P<about>(?:[a-z']+\\s*){1,3})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMES_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:like|as|than|a|an|the|my|your|his|her|their|our|some|any|this|that|these|those|one|two|three)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|passed (?:away|on)|funeral|buried|cremat\\w*|death|widow\\w*|put (?:him|her|it|them)? ?down|euthanis\\w*|we lost (?:her|him|them|mum|dad|my))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_OBJECT(py_str("(?!\\s+(?:a|an|the|my|your|his|her|their|our|some|this|that|these|those|no|one|two|three|four|five|another|all|[a-z]+ing)\\b)"));
    s__ENDING(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:left me"), &g__AN_OBJECT()), &py_str("|walked out|threw me out|kicked me out|(?:broke|split) up")), &g__AN_OBJECT()), &py_str("|separated|divorc\\w*|she'?s gone|he'?s gone|affair|cheated)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__JOB(stdlib_call("re", "compile", &[py_str("\\b(?:lost my (?:job|work|business|contract)|redundant|redundancy|laid off|sacked|(?:got|been|were|was|im|i am|i'?m|getting|get|being)\\s+fired\\b(?!\\s+up\\b)|let (?:me|him|her|us|them) go|resigned|walked out of (?:my|the) job|contract not renewed|hours (?:were )?cut|(?:my|our|the|a)\\s+pay cut\\b|pay cut\\b[^.!?]*\\b(?:job|work|hours|salary|wage|boss|employer|shift|contract)\\b|\\b(?:job|work|hours|salary|wage|boss|employer|shift|contract)\\b[^.!?]*\\bpay cut)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__OWED(py_str("(?:the |my |our )?(?:rent|bills?|payments?|mortgage|council tax|card|loan|gas|electric(?:ity)?|water|dues|money|everything|childcare|nursery|school meals?)"));
    s__MONEY(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:in debt|arrears|behind on "), &g__OWED()), &py_str("|behind with ")), &g__OWED()), &py_str("|can'?t afford|cannot afford|bailiffs|evicted|eviction|repossession|bankrupt|overdraft|credit cards? maxed|money is tight|low on (?:money|cash|funds|dosh)|short of money|skint|brokest|(?:i|we)(?:'ve| have)?\\s+no money|down to my last (?:\\d+|tenner|fiver|pound|quid|twenty|fifty))\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__SKINT(stdlib_call("re", "compile", &[py_str("\\b(?:im|i am|i'?m|we are|we'?re|were)\\s+(?:so\\s+|totally\\s+|completely\\s+|proper\\s+)?broke\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ILLNESS(stdlib_call("re", "compile", &[py_str("\\b(?:diagnos\\w*|cancer|tumour|tumor|chemo\\w*|radiotherapy|biopsy|hospice|terminally ill|(?:had|having|a|another|suspected)\\s+stroke\\b(?!\\s+of\\b)|heart attack|dementia|alzheimers|parkinsons|multiple sclerosis|(?:has|have|got|with)\\s+ms\\b(?!\\s+(?:word|teams|office|excel|project|outlook|access|paint|edge|windows))|in (?:the )?hospital|intensive care)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__FEELING(stdlib_call("re", "compile", &[py_str("\\b(?:im|i am|i'?m|i feel|i'?ve been|ive been|feeling)\\b(?:\\s+\\w+){0,4}\\s+(?:stressed|overwhelmed|exhausted|knackered|burnt out|burned out|anxious|panicking|depressed|miserable|drowning|struggling|at the end of my tether|spiralling|all over the place|low\\b(?!\\s+(?:on|in|down)\\b)|down\\b(?!\\s+to\\b)|flat\\b(?!\\s+(?:tire|tyre|battery|mate|screen|white|bread|out|packed))|blue\\b|rubbish|awful|hopeless|empty)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__FLAT(stdlib_call("re", "compile", &[py_str("\\b(?:im|i am|i'?m|ive|i'?ve|i feel|i'?ve been|ive been|feeling)\\b(?:\\s+\\w+){0,4}\\s+(?:tired of (?:it all|everything|this|all this|life)|fed up|had enough|sick of (?:it|everything|all this)|worn out|worn down|running on empty|drained|no energy)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:i want to|i wanna|im thinking of|i'?m thinking of|im going to|i'?m going to|i plan to|i'?d like to|i would like to|ive decided to|i'?ve decided to|we want to|we'?re thinking of)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__MAKING(stdlib_call("re", "compile", &[py_str("\\b(?:cook|cooking|bake|baking|make|making|build|building|grow|growing|plant|planting|sew|sewing|knit|knitting|paint|painting|brew|brewing|roast|roasting|grill|grilling|barbecue|smoke|smoking|pickle|pickling|ferment|fermenting)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SELF(stdlib_call("re", "compile", &[py_str("\\b(?:learn|learning|study|studying|studies|practi[cs]e|practi[cs]ing|take (?:it |that )?up|get (?:fit|fitter|healthier|stronger|in shape)|lose (?:some )?weight|start(?:ing)? (?:running|swimming|cycling|walking|yoga|pilates|climbing|the gym|going to the gym|lessons?|classes?)|guitar|piano|drums|violin|ukulele|singing|choir|running|swimming|cycling|yoga|pilates|climbing|tennis|football|rugby|drawing|sketching|painting|photography|pottery|write|writing|a book|a novel|a blog|code|coding|programming|a language|spanish|french|italian|german|welsh|uni|university|college|a course|a degree|an evening class|open university)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH(py_tuple(vec![py_str("Nothing has to be decided today. When something does, it is usually one dull thing - a phone call, a form - so take whichever is in front of you and let the rest wait."), py_str("There's no right order for this, and no rush to find one. Pick the smallest thing that has to happen this week and leave the rest of it."), py_str("The next step is usually paperwork, and it keeps. If it helps more, tell me about them instead.")]));
    s_ENDING(py_tuple(vec![py_str("Then start with tonight rather than the whole thing: somewhere to sleep, and what you say to the people who will ask."), py_str("Nothing about the why has to be settled now. The next day or two is the only real question - where you stay, what you eat, who you ring."), py_str("Take it in the order it hurts least: a roof and money first, the rest after. Is either of those already sorted?")]));
    s_JOB(py_tuple(vec![py_str("Money has dates, so start there: what the last pay and any notice money come to, and when they land."), py_str("One small thing tomorrow, one the day after - a CV refreshed, a single application. Momentum does more than a plan here."), py_str("First find out what you are owed and when: final pay, holiday, notice. Work out the next month before the next year.")]));
    s_MONEY(py_tuple(vec![py_str("Start with whatever has a deadline - the rent, or the bill that is actually overdue - then ring the people owed it. They are used to that call."), py_str("Free advice beats guessing: StepChange or Citizens Advice will work it out with you for nothing. Ring one of them this week."), py_str("Do the sums in one sitting - what comes in, what has to go out, what is left - and ring whoever you owe before they ring you.")]));
    s_ILLNESS(py_tuple(vec![py_str("The next thing is a question for the doctor rather than for you: what happens next, and what to do if. Write those two down before the appointment."), py_str("Nothing has to be understood today. Take one practical thing - the appointment, the form, who drives you - and let the rest wait."), py_str("Ask what the treatment is for and what it is meant to do, and take someone with you to write the answers down.")]));
    s_FEELING(py_tuple(vec![py_str("Then take the smallest thing that would ease it - one call made, one thing dropped, half an hour of nothing. Which is easiest?"), py_str("You don't have to sort the whole week. Pick the one thing that would make today lighter and do only that."), py_str("Start with the basics - food, sleep, one thing off the list - and let the rest wait until tomorrow.")]));
    s_PLAN(py_tuple(vec![py_str("Then pick the smallest first move - one website, one phone call, one list - and do only that today."), py_str("Start with the piece you can finish in an hour: a list of what it costs, or the one call you have been putting off."), py_str("Do the boring first step, whatever it is - the form, the quote, the phone call. The rest gets clearer after it."), py_str("Find one person who has done it and ask what their first week looked like. Then do the smallest thing off that list today.")]));
    s_SELF(py_tuple(vec![py_str("Then make the first go small enough to be easy: twenty minutes of it today, and nothing promised beyond that."), py_str("Pick the beginner's version and do it once this week, badly. It gets clearer once there is something to improve."), py_str("Start with the hour you would actually do it in, not the plan for it. What would that hour look like?")]));
    s_HARD(py_tuple(vec![py_str("One thing at a time, then. What is the smallest thing that would make today easier?"), py_str("You don't have to have a plan. Pick the single next step and ignore the rest of it for now."), py_str("Start with whatever has a deadline, if anything does; if nothing does, start with eating something.")]));
    s__ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(?:dog|dogs|puppy|pup|cat|cats|kitten|pet|rabbit|bunny|hamster|guinea pig|horse|pony|budgie|parrot|ferret|tortoise|goldfish|fish)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_LOSS(py_tuple(vec![py_str("Nothing has to be decided today, and nothing has to be tidied away either. What is the house like without them?"), py_str("There is no step to take today. If it helps, tell me about them - what they were like."), py_str("The quiet is the hard part, and it takes the time it takes. Take today as slowly as it comes.")]));
    s_EMERGENCY(stdlib_call("re", "compile", &[py_str("\\b(?:999|911|112|111|a&e|a and e|casualty|ambulance|emergency department|out of hours|urgent care|walk-in cent(?:re|er))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_URGENT_ALREADY_GIVEN(py_tuple(vec![py_str("The advice you just had is the first thing to do. Nothing else needs a plan until someone has seen you."), py_str("What you were just told to do is the whole of the list today - the rest keeps until someone has seen you."), py_str("Get seen first. Nothing on the list is worth doing before that, and it will all still be there afterwards.")]));
    s_FIRST_SENTENCE(stdlib_call("re", "compile", &[py_str("[.!?](?:\\s|$)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONDITION_IN_FRONT(stdlib_call("re", "compile", &[py_str("\\b(?:if|unless|when|whether|in case)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_LINE(py_tuple(vec![py_str("Nothing looks like a first step when it is all in one heap. Say the situation in a line and we'll find the smallest piece of it."), py_str("It is usually one thing pretending to be ten. What is the part that keeps coming back?"), py_str("The fog is normal with this much on. Name one thing that happened today and we will start there.")]));
    s_POOLS(py_tuple(vec![py_tuple(vec![g__DEATH(), g_DEATH()]), py_tuple(vec![g__ENDING(), g_ENDING()]), py_tuple(vec![g__JOB(), g_JOB()]), py_tuple(vec![g__MONEY(), g_MONEY()]), py_tuple(vec![g__SKINT(), g_MONEY()]), py_tuple(vec![g__ILLNESS(), g_ILLNESS()]), py_tuple(vec![g__FEELING(), g_FEELING()]), py_tuple(vec![g__FLAT(), g_FEELING()]), py_tuple(vec![g__PLAN(), g_PLAN()])]));
    s_DOING(call_builtin("frozenset", &[mcall(&py_str("start starts begin pick do try ask ring call phone write book get go take read\nwatch cook bake keep put set use buy check plan split choose make find look learn practice practise\nsend fill sign download join tell speak say warm rinse boil cut add mix stop ease slow wear walk run\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__START_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:so|and|but|well|oh|honestly|then|i mean|like|just|seriously|really|the thing is|see|look)\\s+)*(?:(?:(?:i|we)\\s+)?(?:(?:really|honestly|just|even|still)\\s+)*(?:(?:dont|don'?t|do not)\\s+(?:even\\s+)?know|dunno|(?:have\\s+)?no\\s+(?:idea|clue))\\s+where\\s+to\\s+(?:start|begin)|(?:where|how)\\s+(?:do|should|shall|can|could)\\s+(?:i|we)\\s+(?:even\\s+)?(?:start|begin)|where\\s+to\\s+(?:start|begin))(?:\\s+(?:now|first|today|at\\s+all))?\\s*(?:\\?|\\.|!)*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_reaction_to_what_was_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lead: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lead") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lead"))) };
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether a clause in front of the ask is only the person's reaction to the last turn.");
    text = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = lead.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" ,;:.!?-—")]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    found = mcall(&g_REACTION_FRAME(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_NAMES_A_THING(), "search", &[mcall(&found.clone(), "group", &[py_str("about")])])));
    Py::None
}
pub fn the_ask_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut found: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The ask in the line: the whole of it, or the ask at the end of a reaction to the last turn.\n\n    \"\" when the line is not a stuck ask at all, which is most lines.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&mcall(&g_STUCK(), "match", &[text.clone()])) {
    return text.clone();
    }
    found = mcall(&g_AN_ASK_AT_THE_END(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    lead = mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,;:.!?-—")]);
    if py_truthy(&{ let __b = lead.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_reaction_to_what_was_said(&[lead.clone()]))) } }) {
    return py_str("");
    }
    return mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[py_str(" ,;:.!?-—")]);
    Py::None
}
pub fn _opens_with_an_emergency(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the reply's first sentence is the emergency, said without a condition on it.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    found = mcall(&g_EMERGENCY(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    end = mcall(&g_FIRST_SENTENCE(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = end.clone(); if !py_truthy(&__b) { __b } else { ge(&mcall(&found.clone(), "start", &[]), &mcall(&end.clone(), "end", &[])) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_A_CONDITION_IN_FRONT(), "search", &[slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None)])));
    Py::None
}
pub fn the_advice_was_to_get_seen(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the bot's last reply opened by telling them to get seen today.\n\n    A reply that says \"ring 111\" or \"go to A&E\" in its first sentence has already given the step,\n    and a line that asks where to start after it is asking about everything else. Read from the\n    last thing the bot said, so a chat that never reached one of those answers is untouched.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    return _opens_with_an_emergency(&[call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    return Py::Bool(false);
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut ask: Py = Py::None;
    let mut before: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut replies: Py = Py::None;
    let _ = py_str("The pool of next steps for a stuck line, or None when the line is not that or nothing hard\n    came before it.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    ask = the_ask_in(&[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&ask.clone()))) {
    return Py::None;
    }
    before = _before(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    if py_truthy(&mcall(&g__START_ASK(), "match", &[ask.clone()])) {
    return _not_said_again(&[g_HARD(), memory.clone()]);
    }
    if py_truthy(&mcall(&g__STATE(), "match", &[low.clone()])) {
    return Py::None;
    }
    return _not_said_again(&[g_FIRST_LINE(), memory.clone()]);
    }
    if py_truthy(&the_advice_was_to_get_seen(&[memory.clone()])) {
    return _not_said_again(&[g_URGENT_ALREADY_GIVEN(), memory.clone()]);
    }
    let __iter1 = py_iter_or_raise(&g_POOLS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    replies = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&pattern.clone(), &g__PLAN())); if !py_truthy(&__b) { __b } else { mcall(&g__MAKING(), "search", &[before.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&pattern.clone(), &g__PLAN())); if !py_truthy(&__b) { __b } else { mcall(&g__SELF(), "search", &[before.clone()]) } }) {
    if py_truthy(&_the_answer_gave_a_step(&[memory.clone()])) {
    continue;
    }
    return _not_said_again(&[g_SELF(), memory.clone()]);
    }
    if py_truthy(&mcall(&pattern.clone(), "search", &[before.clone()])) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&pattern.clone(), &g__DEATH())); if !py_truthy(&__b) { __b } else { mcall(&g__ANIMAL(), "search", &[before.clone()]) } }) {
    continue;
    }
    return _not_said_again(&[replies.clone(), memory.clone()]);
    }
    }
    if py_truthy(&crate::gen::compose::badnews::reads_as_bad_news(&[before.clone()])) {
    return _not_said_again(&[g_HARD(), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _the_answer_gave_a_step(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("Whether the bot's last reply already told the person something to do.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    said = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\d"), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str(":"))) } }) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(6), &Py::None)) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOING(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _not_said_again(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut replies: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "replies") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: replies"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The pool with the steps the bot has already said in the last few turns pushed back.\n\n    \"im drained\" is answered \"give yourself half an hour doing nothing before the next thing\", and\n    the stuck line after it was answered with the same half hour: two turns, one idea. The step\n    that shares most of its words with what the bot just said goes to the back, and the freshest\n    one comes first; nothing is dropped, because a repeated step is still better than the nod.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() }]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return replies.clone();
    }
pub fn __nested_written_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut memory: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut replies: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut turn: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut turns: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut written: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "reply").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut shared: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&reply.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::memory::topics::g_STOPWORDS(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    shared = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said.clone(), &mcall(&w.clone(), "rstrip", &[py_str("s")])))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = ge(&shared.clone(), &Py::Int(3)); if !py_truthy(&__b) { __b } else { ge(&truediv(&shared.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Float(0.35)) } };
    Py::None
}
    written = py_func("written", move |args: &[Py]| { let mut full = vec![memory.clone(), replies.clone(), said.clone(), turn.clone(), turns.clone(), written.clone()]; full.extend_from_slice(args); __nested_written_0(&full) });
    return call_builtin("tuple", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(replies.clone()); __kws.extend(vec![("key".to_string(), written.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _before(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line before this one, cleaned, or '' when there is none.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_str("");
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.stuck".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_ASK_AT_THE_END" => Some(g_AN_ASK_AT_THE_END()),
            "A_CONDITION_IN_FRONT" => Some(g_A_CONDITION_IN_FRONT()),
            "DEATH" => Some(g_DEATH()),
            "DOING" => Some(g_DOING()),
            "EMERGENCY" => Some(g_EMERGENCY()),
            "ENDING" => Some(g_ENDING()),
            "FEELING" => Some(g_FEELING()),
            "FIRST_LINE" => Some(g_FIRST_LINE()),
            "FIRST_SENTENCE" => Some(g_FIRST_SENTENCE()),
            "HARD" => Some(g_HARD()),
            "ILLNESS" => Some(g_ILLNESS()),
            "JOB" => Some(g_JOB()),
            "MONEY" => Some(g_MONEY()),
            "NAMES_A_THING" => Some(g_NAMES_A_THING()),
            "PET_LOSS" => Some(g_PET_LOSS()),
            "PLAN" => Some(g_PLAN()),
            "POOLS" => Some(g_POOLS()),
            "REACTION_FRAME" => Some(g_REACTION_FRAME()),
            "SELF" => Some(g_SELF()),
            "STUCK" => Some(g_STUCK()),
            "URGENT_ALREADY_GIVEN" => Some(g_URGENT_ALREADY_GIVEN()),
            "_ANIMAL" => Some(g__ANIMAL()),
            "_AN_OBJECT" => Some(g__AN_OBJECT()),
            "_ASK_BODY" => Some(g__ASK_BODY()),
            "_DEATH" => Some(g__DEATH()),
            "_ENDING" => Some(g__ENDING()),
            "_FEELING" => Some(g__FEELING()),
            "_FLAT" => Some(g__FLAT()),
            "_ILLNESS" => Some(g__ILLNESS()),
            "_JOB" => Some(g__JOB()),
            "_LEADS" => Some(g__LEADS()),
            "_MAKING" => Some(g__MAKING()),
            "_MONEY" => Some(g__MONEY()),
            "_NOTHING_ELSE" => Some(g__NOTHING_ELSE()),
            "_OWED" => Some(g__OWED()),
            "_PLAN" => Some(g__PLAN()),
            "_SELF" => Some(g__SELF()),
            "_SKINT" => Some(g__SKINT()),
            "_START_ASK" => Some(g__START_ASK()),
            "_STATE" => Some(g__STATE()),
            "_STATE_SOURCE" => Some(g__STATE_SOURCE()),
            "_a_reaction_to_what_was_said" => Some(py_func("_a_reaction_to_what_was_said", |args: &[Py]| _a_reaction_to_what_was_said(args))),
            "_before" => Some(py_func("_before", |args: &[Py]| _before(args))),
            "_not_said_again" => Some(py_func("_not_said_again", |args: &[Py]| _not_said_again(args))),
            "_opens_with_an_emergency" => Some(py_func("_opens_with_an_emergency", |args: &[Py]| _opens_with_an_emergency(args))),
            "_the_answer_gave_a_step" => Some(py_func("_the_answer_gave_a_step", |args: &[Py]| _the_answer_gave_a_step(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "the_advice_was_to_get_seen" => Some(py_func("the_advice_was_to_get_seen", |args: &[Py]| the_advice_was_to_get_seen(args))),
            "the_ask_in" => Some(py_func("the_ask_in", |args: &[Py]| the_ask_in(args))),
            _ => None,
        }),
        set: None,
    }))
}
