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


thread_local! { static G_A_BODY_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BODY_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COMPARISON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_SIGHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_WITH_SOMETHING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DECISION_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HEDGED_READ: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_OWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAIR_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAYMENT_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_DID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_OR_A_BOND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUOTED_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RECEIPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SMALL_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_FAILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_FAILING_WITH_ITS_OWNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORK_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_COMPARISON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DAY_SIGHED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DAY_WITH_SOMETHING_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DECISION_PAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_DECISION_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FIGURE_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FIGURE_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_FIGURE_THEIRS_DEFAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HEDGED_READ: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HEDGED_READ_OF_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_LOAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_OWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PERSON_DID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_QUOTED_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_THING_FAILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WORK_STUCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_BODY_PART() -> Py { ensure_init(); G_A_BODY_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BODY_PART(v: Py) { ensure_init(); G_A_BODY_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BODY_QUALIFIER() -> Py { ensure_init(); G_A_BODY_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BODY_QUALIFIER(v: Py) { ensure_init(); G_A_BODY_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COMPARISON() -> Py { ensure_init(); G_A_COMPARISON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COMPARISON(v: Py) { ensure_init(); G_A_COMPARISON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_NAMED() -> Py { ensure_init(); G_A_DAY_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_NAMED(v: Py) { ensure_init(); G_A_DAY_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_SIGHED() -> Py { ensure_init(); G_A_DAY_SIGHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_SIGHED(v: Py) { ensure_init(); G_A_DAY_SIGHED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_WITH_SOMETHING_ON() -> Py { ensure_init(); G_A_DAY_WITH_SOMETHING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_WITH_SOMETHING_ON(v: Py) { ensure_init(); G_A_DAY_WITH_SOMETHING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DECISION_STUCK() -> Py { ensure_init(); G_A_DECISION_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DECISION_STUCK(v: Py) { ensure_init(); G_A_DECISION_STUCK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE_ALONE() -> Py { ensure_init(); G_A_FIGURE_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_ALONE(v: Py) { ensure_init(); G_A_FIGURE_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE_THEIRS() -> Py { ensure_init(); G_A_FIGURE_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_THEIRS(v: Py) { ensure_init(); G_A_FIGURE_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HEDGED_READ() -> Py { ensure_init(); G_A_HEDGED_READ.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HEDGED_READ(v: Py) { ensure_init(); G_A_HEDGED_READ.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOAD() -> Py { ensure_init(); G_A_LOAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOAD(v: Py) { ensure_init(); G_A_LOAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_OWED() -> Py { ensure_init(); G_A_OWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_OWED(v: Py) { ensure_init(); G_A_OWED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAIR_NAMED() -> Py { ensure_init(); G_A_PAIR_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAIR_NAMED(v: Py) { ensure_init(); G_A_PAIR_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAYMENT_NAMED() -> Py { ensure_init(); G_A_PAYMENT_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAYMENT_NAMED(v: Py) { ensure_init(); G_A_PAYMENT_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_DID() -> Py { ensure_init(); G_A_PERSON_DID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_DID(v: Py) { ensure_init(); G_A_PERSON_DID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_OR_A_BOND() -> Py { ensure_init(); G_A_PERSON_OR_A_BOND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_OR_A_BOND(v: Py) { ensure_init(); G_A_PERSON_OR_A_BOND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUOTED_PRICE() -> Py { ensure_init(); G_A_QUOTED_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUOTED_PRICE(v: Py) { ensure_init(); G_A_QUOTED_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RECEIPT() -> Py { ensure_init(); G_A_RECEIPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECEIPT(v: Py) { ensure_init(); G_A_RECEIPT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SMALL_LINE() -> Py { ensure_init(); G_A_SMALL_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SMALL_LINE(v: Py) { ensure_init(); G_A_SMALL_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_FAILING() -> Py { ensure_init(); G_A_THING_FAILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_FAILING(v: Py) { ensure_init(); G_A_THING_FAILING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_FAILING_WITH_ITS_OWNER() -> Py { ensure_init(); G_A_THING_FAILING_WITH_ITS_OWNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_FAILING_WITH_ITS_OWNER(v: Py) { ensure_init(); G_A_THING_FAILING_WITH_ITS_OWNER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORK_STUCK() -> Py { ensure_init(); G_A_WORK_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORK_STUCK(v: Py) { ensure_init(); G_A_WORK_STUCK.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION() -> Py { ensure_init(); G_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION(v: Py) { ensure_init(); G_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_BODY() -> Py { ensure_init(); G_THE_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_BODY(v: Py) { ensure_init(); G_THE_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_COMPARISON() -> Py { ensure_init(); G_THE_COMPARISON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_COMPARISON(v: Py) { ensure_init(); G_THE_COMPARISON.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DAY_SIGHED() -> Py { ensure_init(); G_THE_DAY_SIGHED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DAY_SIGHED(v: Py) { ensure_init(); G_THE_DAY_SIGHED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DAY_WITH_SOMETHING_ON() -> Py { ensure_init(); G_THE_DAY_WITH_SOMETHING_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DAY_WITH_SOMETHING_ON(v: Py) { ensure_init(); G_THE_DAY_WITH_SOMETHING_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DECISION_PAIR() -> Py { ensure_init(); G_THE_DECISION_PAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DECISION_PAIR(v: Py) { ensure_init(); G_THE_DECISION_PAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_DECISION_STUCK() -> Py { ensure_init(); G_THE_DECISION_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_DECISION_STUCK(v: Py) { ensure_init(); G_THE_DECISION_STUCK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FIGURE_ALONE() -> Py { ensure_init(); G_THE_FIGURE_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FIGURE_ALONE(v: Py) { ensure_init(); G_THE_FIGURE_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FIGURE_THEIRS() -> Py { ensure_init(); G_THE_FIGURE_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FIGURE_THEIRS(v: Py) { ensure_init(); G_THE_FIGURE_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_FIGURE_THEIRS_DEFAULT() -> Py { ensure_init(); G_THE_FIGURE_THEIRS_DEFAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_FIGURE_THEIRS_DEFAULT(v: Py) { ensure_init(); G_THE_FIGURE_THEIRS_DEFAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HEDGED_READ() -> Py { ensure_init(); G_THE_HEDGED_READ.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HEDGED_READ(v: Py) { ensure_init(); G_THE_HEDGED_READ.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HEDGED_READ_OF_A_PERSON() -> Py { ensure_init(); G_THE_HEDGED_READ_OF_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HEDGED_READ_OF_A_PERSON(v: Py) { ensure_init(); G_THE_HEDGED_READ_OF_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_LOAD() -> Py { ensure_init(); G_THE_LOAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_LOAD(v: Py) { ensure_init(); G_THE_LOAD.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_OWED() -> Py { ensure_init(); G_THE_OWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_OWED(v: Py) { ensure_init(); G_THE_OWED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PERSON_DID() -> Py { ensure_init(); G_THE_PERSON_DID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PERSON_DID(v: Py) { ensure_init(); G_THE_PERSON_DID.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_QUOTED_PRICE() -> Py { ensure_init(); G_THE_QUOTED_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_QUOTED_PRICE(v: Py) { ensure_init(); G_THE_QUOTED_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_THING_FAILING() -> Py { ensure_init(); G_THE_THING_FAILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_THING_FAILING(v: Py) { ensure_init(); G_THE_THING_FAILING.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WORK_STUCK() -> Py { ensure_init(); G_THE_WORK_STUCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WORK_STUCK(v: Py) { ensure_init(); G_THE_WORK_STUCK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The person has said something, and the bot answers with an acknowledgement and no next step.\n\nTwo replies in the composer are a receipt rather than a reply, and they are what a line that\n*says* something instead of asking gets on main:\n\n    my brakes feel spongy on my bike   -> \"I'm listening - say more if it helps.\"\n    my rent is 800                     -> \"Got it. Is there anything you'd like a hand with?\"\n    its the lower back, mostly on the right\n                                       -> \"Got it. Is there anything you'd like a hand with?\"\n    the family is coming over on sunday\n                                       -> \"Got it. Is there anything you'd like a hand with?\"\n\nA receipt is right for small talk and wrong for all of these: a thing that is failing, a figure\nabout their own money, a detail about a body, a day with something on. Nobody who says \"my brakes\nfeel spongy\" wants to hear that the bot is listening; they want the next thing to check. Nothing\nhas to be looked up to answer any of them, which is why the hedge is not the excuse.\n\nSo this file turns the receipt into one short question about the thing they just said - the\nquestion a friend asks to be useful rather than to be polite. It is the move STYLE.md asks for\nwhen the bot cannot answer the whole question: say what is known, then ask the one thing that\nnarrows it.\n\nIt is a last resort, and written to be one. `bot/modules/greeting.py` asks it after every reader\nthat knows a subject, and it only takes the turn when the reply the composer chose is one of its\ntwo receipts, so an answer of any kind keeps the turn it has. It never states a fact and never\nnames a figure of its own: the question is made from the person's words.\n\nKinds are tried narrowest first: a figure about their money, a detail about a body, a thing that\nis failing, a day with something on, and a decision with two ways out.\n");
    s_A_RECEIPT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bis there anything you'?d like a hand with\\b|\\bsay more if it helps\\b|\\bthanks for telling me\\b[^.]{0,40}\\bdig into any of that\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUESTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:is|isnt|isn't|are|arent|aren't|was|wasnt|wasn't|were|do|does|did|can|could|will|would|should|shall|may|might|have|has|had|who|what|when|where|why|how|which|whose)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_SMALL_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:hi|hiya|hey|hello|yo|good morning|morning|good evening|evening|thanks|thank you|ta|cheers|ok|okay|k|kk|cool|right|fine|yes|yeah|yep|no|nope|nah|not really|maybe|sure|alright|fair enough|lol|ha|haha|hmm+|oh|ah|i see|got it|nice|great|good|sound)\\b[\\s,.!?]*(?:\\w+(?:\\s+\\w+){0,2})?$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FIGURE_ALONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:(?:about|around|roughly|near|just|only|maybe|probably|i think|its|it's|thats|that's|like|so)\\s*)*(?:£|\\$|eur|euros?\\s*)?\\d[\\d,\\.]*\\s*(?:k|grand|thousand|a month|a week|a year|per month|per week|per year|pm|pcm|pounds?|quid)?\\W*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FIGURE_THEIRS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?:[a-z']+\\s+)?(rent|mortgage|salary|pay|wage|wages|income|pension|budget|bill|bills|loan|savings|overdraft|deposit)\\b(?:\\s+(?:is|was|are|comes to|went up to|now))?\\s*(?:about|around|roughly|only|just)?\\s*(?:£|\\$|eur\\s*)?\\d[\\d,\\.]*\\s*(?:k|grand|thousand)?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BODY_PART({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|the)\\s+(?:[a-z']+\\s+)?(back|knee|knees|hip|shoulder|neck|stomach|tummy|head|headache|ankle|wrist|elbow|chest|tooth|teeth|throat|ear|ears|eye|eyes|leg|legs|arm|arms|foot|feet|toe|toes|heel|skin|sinus|joint|joints|gut|bladder|period|periods)\\b|\\bits\\s+(?:the\\s+)?(?:[a-z']+\\s+)?(?:back|knee|hip|shoulder|neck|stomach|head|ankle|wrist|elbow|chest|foot|leg|arm)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_BODY_QUALIFIER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:lower|upper|left|right|both|sore|stiff|aching|achy|throbbing|sharp|dull|burning|swollen|tender|hurts?|hurting|killing me|giving way|gives way|clicking|locking|numb|tingling|stabbing|on the right|on the left|mostly|worse in the morning|at night)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_FAILING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:clogged|clogging|blocked|blocking|leaking|leak|broken|broke|knackered|faulty|not working|wont work|won't work|wont start|won't start|wont turn on|stopped working|packed in|given up|spongy|squeaking|squealing|rattling|smells?|smelly|damp|mould|mold|flies|ants|mice|rats|flies everywhere|beeping|flashing|jam|jammed|stuck|flooded|flooding|waterlogged|sodden|rotting|rusting|overflowing)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_FAILING_WITH_ITS_OWNER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|the)\\s+(?:[a-z']+\\s+)?(?:brakes?|bike|car|boiler|shower|tap|toilet|washing machine|dishwasher|fridge|freezer|oven|hob|kettle|hoover|vacuum|computer|laptop|phone|heating|radiator|radiators|drain|drains|sink|door|window|lock|roof|gutter|gutters|tyre|tyres|engine|gears)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON_OR_A_BOND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:marriage|relationship|friendship|husband|wife|partner|boyfriend|girlfriend|ex|mum|mom|mother|dad|father|son|daughter|child|children|kids?|family|friends?|mate|boss|colleague|job|career|business|life|trust|heart)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DAY_WITH_SOMETHING_ON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:coming over|coming round|coming to stay|visiting|arriving|staying|we're having|were having|im having|i'm having|we're hosting|hosting|party|barbecue|bbq|bday|birthday|wedding|christening|funeral|dinner party|roast|big shop|moving house|moving in|interview|first day|appointment|exam|driving test)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DAY_NAMED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:today|tomorrow|tonight|this weekend|next week|next month|saturday|sunday|monday|tuesday|wednesday|thursday|friday|january|february|march|april|may|june|july|august|september|october|november|december)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DECISION_STUCK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i (?:dont|don't|do not) know|dno|dunno|no idea)\\b[^.?!]{0,24}?\\b(?:where|what|which|who|whether|if) (?:to|i)\\b|\\b(?:cant|cannot|can't) (?:decide|choose|make up my mind)\\b|\\b(?:torn|stuck) between\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LOAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:lots on|a lot on|so much on|too much on|everything on at once|no time to|never have time|havent got time|haven't got time|run off my feet|rushed off my feet|flat out|busy week|mad week|hectic|no minute|not enough hours)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_HEDGED_READ({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i think|i feel like|i get the feeling|i suspect)\\b[^.?!]{0,40}|\\bi (?:dont|don't|do not) think i\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WORK_STUCK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:dno|dunno|dont know|don't know|stuck|fed up|sick of|had enough|had it up to here|cant stand|can't stand|dreading)\\b[^.?!]{0,32}\\b(?:job|work|shift|shifts|boss|manager|office|colleagues?|interview|payday|rota)\\b|\\b(?:job|work|shift|shifts|boss|manager|office|interview)\\b[^.?!]{0,24}\\b(?:dno|dunno|dont know|don't know|stuck|fed up|sick of|had enough|dreading|driving me)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_OWED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi (?:owe|still owe)\\b[^.?!]{0,24}?(?:\\u00a3|\\$)?\\d[\\d,]*")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PAYMENT_NAMED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:mortgage|rent|council tax|electric|electricity|gas|water|broadband|phone bill)\\b\\s*(?:is|was|are|comes to|=)\\s*(?:about|around|roughly|only|just)?\\s*(?:\\u00a3|\\$)?\\d")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_QUOTED_PRICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:wants|wanting|quoted|has quoted|said|says|asking|charging|want)\\b[^.?!]{0,18}?(?:\\u00a3|\\$)?\\d[\\d,]*\\s*(?:quid|pounds|bucks|euro|euros)?\\b[^.?!]{0,20}?\\b(?:quid|pounds|bucks|for the|for a|to do|to fit|to fix)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_OWED(py_tuple(vec![py_str("Is that what is overdue, or the bill itself?"), py_str("Is that the whole of it, or the first of it?"), py_str("What happens if it waits until next month?")]));
    s_THE_QUOTED_PRICE(py_tuple(vec![py_str("Is that in writing, and does it include the parts?"), py_str("Would a second quote be easy to get?"), py_str("Is that a firm quote or a guess they could add to?")]));
    s_A_PERSON_DID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|they|him|her|them|his|hers|their)\\b[^.?!]{0,24}?\\b(?:said|told|texted|calld|called|rang|phoned|replied|laughed|shouted|yelled|ignored|asked|sent|wrote|came|went|left|refused|agreed|apologised|turned up|walked|stormed|didnt|wouldnt|hasnt|hadnt|couldnt)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_PERSON_DID(py_tuple(vec![py_str("And how did that leave you?"), py_str("What did you say back?"), py_str("Has it been like this a while, or is it new?")]));
    s_A_COMPARISON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|was|seems|looks|feels|its|it's)\\s+(?:much\\s+)?(?:more|less|better|worse|bigger|smaller|cheaper|dearer|closer|further|farther|easier|harder|nicer|newer|older|quicker|slower)\\b[^.?!]{0,40}?\\b(?:but|though|although|whereas)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_COMPARISON(py_tuple(vec![py_str("Which of the two would you notice more in a year?"), py_str("So which way are you leaning?"), py_str("What would make it an easy choice?")]));
    s_A_DAY_SIGHED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:well,?\\s*)?(?:its |it's |and )?(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b[^.?!]{0,12}\\bagain\\b|\\b(?:back to work|another week|is it friday yet|hump day|long week ahead|weekend is over|back to reality)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_DAY_SIGHED(py_tuple(vec![py_str("What is on today that you are not looking forward to?"), py_str("Is it the day, or what is waiting on the other side of it?"), py_str("What would make today easier?")]));
    s_THE_LOAD(py_tuple(vec![py_str("What is the bit of it that is taking the time?"), py_str("Which of it could come off the list?"), py_str("What is the one thing you would drop if you could?")]));
    s_THE_HEDGED_READ(py_tuple(vec![py_str("What has made you think that?"), py_str("What would you do about it if you were sure?")]));
    s_THE_HEDGED_READ_OF_A_PERSON(op_add(&g_THE_HEDGED_READ(), &py_tuple(vec![py_str("And what has actually been said or done so far?")])));
    s_THE_WORK_STUCK(py_tuple(vec![py_str("Is it the work itself, the people, or what it pays?"), py_str("What would have to change for it to be bearable?"), py_str("How long has it been like this?")]));
    s_THE_FIGURE_ALONE(py_tuple(vec![py_str("I've got that figure. Is it what you have to spend, or what it is going to cost?"), py_str("Noted - and is that on its own, or is there more going out on top of it?"), py_str("Got that down. What is it for: the thing itself, or everything around it?")]));
    s_THE_FIGURE_THEIRS(py_dict(vec![(py_str("rent"), py_tuple(vec![py_str("Is that with the bills in it, or on top of them?"), py_str("And is that the whole of it, or is there service charge and council tax as well?")])), (py_str("mortgage"), py_tuple(vec![py_str("Is that the fixed rate, and how long is left on it?"), py_str("Is that on its own, or with the insurance and the rest?")])), (py_str("pay"), py_tuple(vec![py_str("Is that before tax or what lands in your account?"), py_str("And is that for the hours you want, or more than you want?")])), (py_str("budget"), py_tuple(vec![py_str("What does that have to cover?"), py_str("What is the one thing in it you would not cut?")])), (py_str("savings"), py_tuple(vec![py_str("Is that everything you have put by, or is there more elsewhere?"), py_str("What are you keeping it for?")])), (py_str("bills"), py_tuple(vec![py_str("Which of them is the one that has gone up?"), py_str("Is that every month, or is it a bill that landed all at once?")])), (py_str("loan"), py_tuple(vec![py_str("How long is left on it, and is the rate fixed?"), py_str("Is that the payment or what is still owed?")]))]));
    s_THE_FIGURE_THEIRS_DEFAULT(py_tuple(vec![py_str("Is that on its own, or does it cover the rest as well?"), py_str("And does that go up at any point, or is it fixed?")]));
    s_THE_BODY(py_tuple(vec![py_str("How long has it been like that, and what makes it worse?"), py_str("Does anything ease it - rest, heat, or a painkiller?"), py_str("And is it there all the time, or does it come and go?")]));
    s_THE_THING_FAILING(py_tuple(vec![py_str("What have you tried on it so far?"), py_str("How long has it been doing that?"), py_str("Is it doing it all the time, or only now and then?")]));
    s_THE_DAY_WITH_SOMETHING_ON(py_tuple(vec![py_str("How many are you having?"), py_str("What has to be ready before then?"), py_str("What is the part you are least sure about?")]));
    s_A_PAIR_NAMED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:torn\\s+between|between|the\\s+two|both|either|one\\s+or\\s+the\\s+other|which\\s+one|this\\s+or\\s+that)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_DECISION_PAIR(py_tuple(vec![py_str("What are the two you are weighing up, and what does each one cost you?"), py_str("Which of the two would you pick if nobody else had an opinion?")]));
    s_THE_DECISION_STUCK(py_tuple(vec![py_str("What is the thing that makes it hard to choose?"), py_str("Which way would you go if nobody else had an opinion?"), py_str("What would you need to know first, to pick one?")]));
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
    let _ = py_str("True only for the composer's own acknowledgements.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_RECEIPT(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn turned(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut respond: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "respond") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: respond"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("`respond`, with a receipt replaced by a question about the thing the person just said.");
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut reply: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut respond: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "text").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut said: Py = Py::None;
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&respond, args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| answer_for(args))(&__args) }; if py_truthy(&__b) { __b } else { said.clone() } };
    Py::None
}
    reply = {{ let reply = reply.clone(); let respond = respond.clone(); py_func("reply", move |args: &[Py]| { let mut full = vec![reply.clone(), respond.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) }) }};
    return reply.clone();
    Py::None
}
pub fn _money_kind(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut noun: Py = Py::None;
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FIGURE_THEIRS(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    noun = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("pay"), py_str("wage"), py_str("wages"), py_str("income"), py_str("salary")]), &noun.clone()))) {
    return py_str("pay");
    }
    return if py_truthy(&Py::Bool(py_contains(&g_THE_FIGURE_THEIRS(), &noun.clone()))) { noun.clone() } else { py_str("default") };
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
    else { Py::None };
    let mut clean: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("One short question about the thing they just said, or None.\n\n    Called with the reply the composer chose: the whole point is that no reader wanted the turn.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_receipt(args))(&__args) }))) {
    return Py::None;
    }
    memory = { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("understood")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(18)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUESTION(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", args))(&__args) }, "endswith", args))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SMALL_LINE(), "match", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_OWED(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_OWED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money_kind(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kind.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PAYMENT_NAMED(), "search", args))(&__args) } } }) {
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&clean.clone(), &py_str(" my ")), &geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" is ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }, &Py::Int(0)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money_kind(args))(&__args) };
    kind = { let __b = kind.clone(); if py_truthy(&__b) { __b } else { py_str("default") } };
    }
    if py_truthy(&kind.clone()) {
    pool = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_FIGURE_THEIRS(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { g_THE_FIGURE_THEIRS_DEFAULT() } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FIGURE_ALONE(), "match", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_FIGURE_ALONE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_WORK_STUCK(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("work")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("job")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_OR_A_BOND(), "search", args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_WORK_STUCK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_QUOTED_PRICE(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_QUOTED_PRICE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_COMPARISON(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_COMPARISON()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_DID(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_OR_A_BOND(), "search", args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_PERSON_DID()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_DAY_SIGHED(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_DAY_SIGHED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_LOAD(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_LOAD()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_HEDGED_READ(), "search", args))(&__args) }) {
    pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_OR_A_BOND(), "search", args))(&__args) }) { g_THE_HEDGED_READ_OF_A_PERSON() } else { g_THE_HEDGED_READ() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_DECISION_STUCK(), "search", args))(&__args) }) {
    pool = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PAIR_NAMED(), "search", args))(&__args) }) { op_add(&g_THE_DECISION_PAIR(), &g_THE_DECISION_STUCK()) } else { g_THE_DECISION_STUCK() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BODY_PART(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_BODY_QUALIFIER(), "search", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_BODY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING_FAILING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_PERSON_OR_A_BOND(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THING_FAILING_WITH_ITS_OWNER(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(8)) } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_THING_FAILING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_DAY_WITH_SOMETHING_ON(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_DAY_NAMED(), "search", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_DAY_WITH_SOMETHING_ON()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::pick(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.a_detail_dropped".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_BODY_PART" => Some(g_A_BODY_PART()),
            "A_BODY_QUALIFIER" => Some(g_A_BODY_QUALIFIER()),
            "A_COMPARISON" => Some(g_A_COMPARISON()),
            "A_DAY_NAMED" => Some(g_A_DAY_NAMED()),
            "A_DAY_SIGHED" => Some(g_A_DAY_SIGHED()),
            "A_DAY_WITH_SOMETHING_ON" => Some(g_A_DAY_WITH_SOMETHING_ON()),
            "A_DECISION_STUCK" => Some(g_A_DECISION_STUCK()),
            "A_FIGURE_ALONE" => Some(g_A_FIGURE_ALONE()),
            "A_FIGURE_THEIRS" => Some(g_A_FIGURE_THEIRS()),
            "A_HEDGED_READ" => Some(g_A_HEDGED_READ()),
            "A_LOAD" => Some(g_A_LOAD()),
            "A_OWED" => Some(g_A_OWED()),
            "A_PAIR_NAMED" => Some(g_A_PAIR_NAMED()),
            "A_PAYMENT_NAMED" => Some(g_A_PAYMENT_NAMED()),
            "A_PERSON_DID" => Some(g_A_PERSON_DID()),
            "A_PERSON_OR_A_BOND" => Some(g_A_PERSON_OR_A_BOND()),
            "A_QUOTED_PRICE" => Some(g_A_QUOTED_PRICE()),
            "A_RECEIPT" => Some(g_A_RECEIPT()),
            "A_SMALL_LINE" => Some(g_A_SMALL_LINE()),
            "A_THING_FAILING" => Some(g_A_THING_FAILING()),
            "A_THING_FAILING_WITH_ITS_OWNER" => Some(g_A_THING_FAILING_WITH_ITS_OWNER()),
            "A_WORK_STUCK" => Some(g_A_WORK_STUCK()),
            "QUESTION" => Some(g_QUESTION()),
            "THE_BODY" => Some(g_THE_BODY()),
            "THE_COMPARISON" => Some(g_THE_COMPARISON()),
            "THE_DAY_SIGHED" => Some(g_THE_DAY_SIGHED()),
            "THE_DAY_WITH_SOMETHING_ON" => Some(g_THE_DAY_WITH_SOMETHING_ON()),
            "THE_DECISION_PAIR" => Some(g_THE_DECISION_PAIR()),
            "THE_DECISION_STUCK" => Some(g_THE_DECISION_STUCK()),
            "THE_FIGURE_ALONE" => Some(g_THE_FIGURE_ALONE()),
            "THE_FIGURE_THEIRS" => Some(g_THE_FIGURE_THEIRS()),
            "THE_FIGURE_THEIRS_DEFAULT" => Some(g_THE_FIGURE_THEIRS_DEFAULT()),
            "THE_HEDGED_READ" => Some(g_THE_HEDGED_READ()),
            "THE_HEDGED_READ_OF_A_PERSON" => Some(g_THE_HEDGED_READ_OF_A_PERSON()),
            "THE_LOAD" => Some(g_THE_LOAD()),
            "THE_OWED" => Some(g_THE_OWED()),
            "THE_PERSON_DID" => Some(g_THE_PERSON_DID()),
            "THE_QUOTED_PRICE" => Some(g_THE_QUOTED_PRICE()),
            "THE_THING_FAILING" => Some(g_THE_THING_FAILING()),
            "THE_WORK_STUCK" => Some(g_THE_WORK_STUCK()),
            "_money_kind" => Some(py_func("_money_kind", |args: &[Py]| _money_kind(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "reads_as_a_receipt" => Some(py_func("reads_as_a_receipt", |args: &[Py]| reads_as_a_receipt(args))),
            "turned" => Some(py_func("turned", |args: &[Py]| turned(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_BODY_PART" => s_A_BODY_PART(v),
            "A_BODY_QUALIFIER" => s_A_BODY_QUALIFIER(v),
            "A_COMPARISON" => s_A_COMPARISON(v),
            "A_DAY_NAMED" => s_A_DAY_NAMED(v),
            "A_DAY_SIGHED" => s_A_DAY_SIGHED(v),
            "A_DAY_WITH_SOMETHING_ON" => s_A_DAY_WITH_SOMETHING_ON(v),
            "A_DECISION_STUCK" => s_A_DECISION_STUCK(v),
            "A_FIGURE_ALONE" => s_A_FIGURE_ALONE(v),
            "A_FIGURE_THEIRS" => s_A_FIGURE_THEIRS(v),
            "A_HEDGED_READ" => s_A_HEDGED_READ(v),
            "A_LOAD" => s_A_LOAD(v),
            "A_OWED" => s_A_OWED(v),
            "A_PAIR_NAMED" => s_A_PAIR_NAMED(v),
            "A_PAYMENT_NAMED" => s_A_PAYMENT_NAMED(v),
            "A_PERSON_DID" => s_A_PERSON_DID(v),
            "A_PERSON_OR_A_BOND" => s_A_PERSON_OR_A_BOND(v),
            "A_QUOTED_PRICE" => s_A_QUOTED_PRICE(v),
            "A_RECEIPT" => s_A_RECEIPT(v),
            "A_SMALL_LINE" => s_A_SMALL_LINE(v),
            "A_THING_FAILING" => s_A_THING_FAILING(v),
            "A_THING_FAILING_WITH_ITS_OWNER" => s_A_THING_FAILING_WITH_ITS_OWNER(v),
            "A_WORK_STUCK" => s_A_WORK_STUCK(v),
            "QUESTION" => s_QUESTION(v),
            "THE_BODY" => s_THE_BODY(v),
            "THE_COMPARISON" => s_THE_COMPARISON(v),
            "THE_DAY_SIGHED" => s_THE_DAY_SIGHED(v),
            "THE_DAY_WITH_SOMETHING_ON" => s_THE_DAY_WITH_SOMETHING_ON(v),
            "THE_DECISION_PAIR" => s_THE_DECISION_PAIR(v),
            "THE_DECISION_STUCK" => s_THE_DECISION_STUCK(v),
            "THE_FIGURE_ALONE" => s_THE_FIGURE_ALONE(v),
            "THE_FIGURE_THEIRS" => s_THE_FIGURE_THEIRS(v),
            "THE_FIGURE_THEIRS_DEFAULT" => s_THE_FIGURE_THEIRS_DEFAULT(v),
            "THE_HEDGED_READ" => s_THE_HEDGED_READ(v),
            "THE_HEDGED_READ_OF_A_PERSON" => s_THE_HEDGED_READ_OF_A_PERSON(v),
            "THE_LOAD" => s_THE_LOAD(v),
            "THE_OWED" => s_THE_OWED(v),
            "THE_PERSON_DID" => s_THE_PERSON_DID(v),
            "THE_QUOTED_PRICE" => s_THE_QUOTED_PRICE(v),
            "THE_THING_FAILING" => s_THE_THING_FAILING(v),
            "THE_WORK_STUCK" => s_THE_WORK_STUCK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.a_detail_dropped", module_obj);
    pyrt::xrefs::reg_fn("compose.a_detail_dropped", "reads_as_a_receipt", py_func("reads_as_a_receipt", |args: &[Py]| reads_as_a_receipt(args)));
    pyrt::xrefs::reg_fn("compose.a_detail_dropped", "turned", py_func("turned", |args: &[Py]| turned(args)));
    pyrt::xrefs::reg_fn("compose.a_detail_dropped", "_money_kind", py_func("_money_kind", |args: &[Py]| _money_kind(args)));
    pyrt::xrefs::reg_fn("compose.a_detail_dropped", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_BODY_PART", g_A_BODY_PART, s_A_BODY_PART);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_BODY_QUALIFIER", g_A_BODY_QUALIFIER, s_A_BODY_QUALIFIER);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_COMPARISON", g_A_COMPARISON, s_A_COMPARISON);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_DAY_NAMED", g_A_DAY_NAMED, s_A_DAY_NAMED);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_DAY_SIGHED", g_A_DAY_SIGHED, s_A_DAY_SIGHED);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_DAY_WITH_SOMETHING_ON", g_A_DAY_WITH_SOMETHING_ON, s_A_DAY_WITH_SOMETHING_ON);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_DECISION_STUCK", g_A_DECISION_STUCK, s_A_DECISION_STUCK);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_FIGURE_ALONE", g_A_FIGURE_ALONE, s_A_FIGURE_ALONE);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_FIGURE_THEIRS", g_A_FIGURE_THEIRS, s_A_FIGURE_THEIRS);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_HEDGED_READ", g_A_HEDGED_READ, s_A_HEDGED_READ);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_LOAD", g_A_LOAD, s_A_LOAD);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_OWED", g_A_OWED, s_A_OWED);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_PAIR_NAMED", g_A_PAIR_NAMED, s_A_PAIR_NAMED);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_PAYMENT_NAMED", g_A_PAYMENT_NAMED, s_A_PAYMENT_NAMED);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_PERSON_DID", g_A_PERSON_DID, s_A_PERSON_DID);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_PERSON_OR_A_BOND", g_A_PERSON_OR_A_BOND, s_A_PERSON_OR_A_BOND);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_QUOTED_PRICE", g_A_QUOTED_PRICE, s_A_QUOTED_PRICE);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_RECEIPT", g_A_RECEIPT, s_A_RECEIPT);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_SMALL_LINE", g_A_SMALL_LINE, s_A_SMALL_LINE);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_THING_FAILING", g_A_THING_FAILING, s_A_THING_FAILING);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_THING_FAILING_WITH_ITS_OWNER", g_A_THING_FAILING_WITH_ITS_OWNER, s_A_THING_FAILING_WITH_ITS_OWNER);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "A_WORK_STUCK", g_A_WORK_STUCK, s_A_WORK_STUCK);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "QUESTION", g_QUESTION, s_QUESTION);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_BODY", g_THE_BODY, s_THE_BODY);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_COMPARISON", g_THE_COMPARISON, s_THE_COMPARISON);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_DAY_SIGHED", g_THE_DAY_SIGHED, s_THE_DAY_SIGHED);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_DAY_WITH_SOMETHING_ON", g_THE_DAY_WITH_SOMETHING_ON, s_THE_DAY_WITH_SOMETHING_ON);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_DECISION_PAIR", g_THE_DECISION_PAIR, s_THE_DECISION_PAIR);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_DECISION_STUCK", g_THE_DECISION_STUCK, s_THE_DECISION_STUCK);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_FIGURE_ALONE", g_THE_FIGURE_ALONE, s_THE_FIGURE_ALONE);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_FIGURE_THEIRS", g_THE_FIGURE_THEIRS, s_THE_FIGURE_THEIRS);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_FIGURE_THEIRS_DEFAULT", g_THE_FIGURE_THEIRS_DEFAULT, s_THE_FIGURE_THEIRS_DEFAULT);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_HEDGED_READ", g_THE_HEDGED_READ, s_THE_HEDGED_READ);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_HEDGED_READ_OF_A_PERSON", g_THE_HEDGED_READ_OF_A_PERSON, s_THE_HEDGED_READ_OF_A_PERSON);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_LOAD", g_THE_LOAD, s_THE_LOAD);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_OWED", g_THE_OWED, s_THE_OWED);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_PERSON_DID", g_THE_PERSON_DID, s_THE_PERSON_DID);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_QUOTED_PRICE", g_THE_QUOTED_PRICE, s_THE_QUOTED_PRICE);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_THING_FAILING", g_THE_THING_FAILING, s_THE_THING_FAILING);
    pyrt::xrefs::reg_access("compose.a_detail_dropped", "THE_WORK_STUCK", g_THE_WORK_STUCK, s_THE_WORK_STUCK);
}
