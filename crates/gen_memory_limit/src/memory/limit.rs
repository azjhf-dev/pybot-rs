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


thread_local! { static G_ALSO_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AND_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPLIANCES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DETAIL_OF_THE_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FAULT_IN_A_NAMED_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HEATING_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LEAKING_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PET_IN_THE_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_STARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOLDS_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_THE_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_THE_HOUSE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURED_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_WITH_A_PET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_APPLIANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_OF_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_IT_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_TO_CALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORDS_IN_A_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAVES_IT_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIMIT_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOVE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_understanding: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ALSO_THERE() -> Py { ensure_init(); G_ALSO_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALSO_THERE(v: Py) { ensure_init(); G_ALSO_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_AND_STEP() -> Py { ensure_init(); G_AND_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AND_STEP(v: Py) { ensure_init(); G_AND_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_AMOUNT() -> Py { ensure_init(); G_AN_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AMOUNT(v: Py) { ensure_init(); G_AN_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_APPLIANCES() -> Py { ensure_init(); G_APPLIANCES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPLIANCES(v: Py) { ensure_init(); G_APPLIANCES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_A_CHOICE() -> Py { ensure_init(); G_ASKS_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_CHOICE(v: Py) { ensure_init(); G_ASKS_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DETAIL_OF_THE_FAULT() -> Py { ensure_init(); G_A_DETAIL_OF_THE_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DETAIL_OF_THE_FAULT(v: Py) { ensure_init(); G_A_DETAIL_OF_THE_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FAULT_IN_A_NAMED_THING() -> Py { ensure_init(); G_A_FAULT_IN_A_NAMED_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FAULT_IN_A_NAMED_THING(v: Py) { ensure_init(); G_A_FAULT_IN_A_NAMED_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HEATING_FAULT() -> Py { ensure_init(); G_A_HEATING_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HEATING_FAULT(v: Py) { ensure_init(); G_A_HEATING_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LEAKING_FAULT() -> Py { ensure_init(); G_A_LEAKING_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LEAKING_FAULT(v: Py) { ensure_init(); G_A_LEAKING_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PET_IN_THE_REASON() -> Py { ensure_init(); G_A_PET_IN_THE_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PET_IN_THE_REASON(v: Py) { ensure_init(); G_A_PET_IN_THE_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REASON() -> Py { ensure_init(); G_A_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REASON(v: Py) { ensure_init(); G_A_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME() -> Py { ensure_init(); G_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME(v: Py) { ensure_init(); G_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_STARTS() -> Py { ensure_init(); G_CLAUSE_STARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_STARTS(v: Py) { ensure_init(); G_CLAUSE_STARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOLDS_ON() -> Py { ensure_init(); G_HOLDS_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOLDS_ON(v: Py) { ensure_init(); G_HOLDS_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_THE_HOUSE() -> Py { ensure_init(); G_IN_THE_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_THE_HOUSE(v: Py) { ensure_init(); G_IN_THE_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_THE_HOUSE_WORDS() -> Py { ensure_init(); G_IN_THE_HOUSE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_THE_HOUSE_WORDS(v: Py) { ensure_init(); G_IN_THE_HOUSE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINING() -> Py { ensure_init(); G_JOINING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINING(v: Py) { ensure_init(); G_JOINING.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS() -> Py { ensure_init(); G_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); G_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_LONGEST() -> Py { ensure_init(); G_LONGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONGEST(v: Py) { ensure_init(); G_LONGEST.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURED() -> Py { ensure_init(); G_MEASURED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURED(v: Py) { ensure_init(); G_MEASURED.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURED_PARTS() -> Py { ensure_init(); G_MEASURED_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURED_PARTS(v: Py) { ensure_init(); G_MEASURED_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE() -> Py { ensure_init(); G_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE(v: Py) { ensure_init(); G_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_ASKS() -> Py { ensure_init(); G_MOVE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_ASKS(v: Py) { ensure_init(); G_MOVE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_HAVE() -> Py { ensure_init(); G_NOT_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_HAVE(v: Py) { ensure_init(); G_NOT_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_WITH_A_PET() -> Py { ensure_init(); G_NOT_WITH_A_PET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_WITH_A_PET(v: Py) { ensure_init(); G_NOT_WITH_A_PET.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_APPLIANCE() -> Py { ensure_init(); G_NO_APPLIANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_APPLIANCE(v: Py) { ensure_init(); G_NO_APPLIANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_OF() -> Py { ensure_init(); G_OUT_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_OF(v: Py) { ensure_init(); G_OUT_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_OF_IT() -> Py { ensure_init(); G_SAID_OF_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_OF_IT(v: Py) { ensure_init(); G_SAID_OF_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP() -> Py { ensure_init(); G_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP(v: Py) { ensure_init(); G_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_TAIL() -> Py { ensure_init(); G_TIME_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_TAIL(v: Py) { ensure_init(); G_TIME_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_IT_IS() -> Py { ensure_init(); G_WHERE_IT_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_IT_IS(v: Py) { ensure_init(); G_WHERE_IT_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_TO_CALL() -> Py { ensure_init(); G_WHO_TO_CALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_TO_CALL(v: Py) { ensure_init(); G_WHO_TO_CALL.with(|c| *c.borrow_mut() = v); }
pub fn g_WORDS_IN_A_REPLY() -> Py { ensure_init(); G_WORDS_IN_A_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORDS_IN_A_REPLY(v: Py) { ensure_init(); G_WORDS_IN_A_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAVES_IT_OUT() -> Py { ensure_init(); G__LEAVES_IT_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAVES_IT_OUT(v: Py) { ensure_init(); G__LEAVES_IT_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__LIMIT_FRAME() -> Py { ensure_init(); G__LIMIT_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIMIT_FRAME(v: Py) { ensure_init(); G__LIMIT_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g__MOVE_WORDS() -> Py { ensure_init(); G__MOVE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOVE_WORDS(v: Py) { ensure_init(); G__MOVE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_understanding() -> Py { ensure_init(); G_understanding.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_understanding(v: Py) { ensure_init(); G_understanding.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that puts a limit on the answer just given.\n\n\"how do i unblock the sink\" is answered with a plunger and then the trap underneath. The next line\n- \"i dont have a plunger\" - is not news about the person and not a new question: it is about the\nanswer they have just been given, and the answer already holds the steps that still work. The same\ngoes for \"i ran out of oven cleaner\" after the oven answer, and for \"ive got a cat\" after the ants\nanswer, where nothing is missing but one step has become unsafe.\n\nSo this module answers from the words of the previous answer: it cuts the answer into steps, drops\nthe ones the limit rules out, and says the rest back with the limit as the reason. It never asks the\nknowledge base again with the limit on the end of the question - \"how do i get rid of ants with a\ncat in the house\" reaches the rehoming-a-pet entry, which is not what the person asked.\n\nIt answers only a short line that names a limit, and only the turn after an answer. Everything else\nis passed on, unchanged, to the modules that were going to answer it.\n");
    s_understanding(py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args)));
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_OUT_OF(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)(?:\\s*(?:have|'ve|ve|had|'m|'re|am|are))?\\s*(?:just\\s+|now\\s+|actually\\s+)?(?:ran out of|run out of|ran outta|am out of|are out of|m out of|'m out of|'re out of|re out of|out of)\\s+(?:any\\s+|a\\s+|an\\s+)?(?P<thing>[a-z][a-z' -]{1,24})|\\b(?:no|without)\\s+(?:a\\s+|an\\s+|any\\s+)?(?P<thing3>[a-z][a-z' -]{1,24})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_HAVE(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:(?:do|did|have)\\s+not|do\\s*n'?t|don'?t|dont|did\\s*n'?t|didnt|haven'?t|havent|hadn'?t|hadnt)\\s+(?:(?:got|have|own|any)\\s+)?(?:any\\s+|a\\s+|an\\s+)?(?P<thing2>[a-z][a-z' -]{1,24})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DETAIL_OF_THE_FAULT(stdlib_call("re", "compile", &[py_str("^(?:(?:hot|cold|running)\\s+water)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HEATING_FAULT(stdlib_call("re", "compile", &[py_str("boiler|heating|radiator|combi|immersion|cylinder|pilot light|water pressure|thermostat"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LEAKING_FAULT(stdlib_call("re", "compile", &[py_str("leak|drip|burst|pour|flood|water everywhere|water coming|spray|seep|smok|fume|soot|flame|on fire"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:idea|clue|time|money|the foggiest|chance|choice|room|option|ants anymore|anymore|any more|no more|a life|a job|job|work|a car|way|luck|bother|worries|worry|problem|doubt|wonder|rush|need|joke|point|sense|harm|thanks|matter)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_THE_HOUSE_WORDS(py_str("cat|cats|kitten|kittens|dog|dogs|puppy|puppies|rabbit|rabbits|hamster|hamsters|guinea pig|budgie|parrot|tortoise|child|children|kid|kids|toddler|baby|son|daughter"));
    s_IN_THE_HOUSE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:(?:i|we)(?:\\s+have|'ve|ve|got)?\\s+(?:got\\s+|have\\s+)?(?:a|an|two|three|our|my)\\s+(?P<who>"), &g_IN_THE_HOUSE_WORDS()), &py_str(")\\b|(?:there'?s|theres)\\s+(?:a|an)\\s+(?P<who2>")), &g_IN_THE_HOUSE_WORDS()), &py_str(")\\s+(?:in the house|here|at home))")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALSO_THERE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\s*(?:,|and|plus)\\s+(?:a|an|the|my|our|two)\\s+(?P<who3>"), &g_IN_THE_HOUSE_WORDS()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_WITH_A_PET(stdlib_call("re", "compile", &[py_str("\\b(?:bait|baits|baited|poison|poisoned|pellets?|killer|insecticide|rodenticide|spring ?traps?|mousetraps?|rat traps?|glue ?boards?|ant powder|ant spray|blue grains)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:how|what|why|when|which|should|can|could|would|do i|is there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_A_CHOICE(stdlib_call("re", "compile", &[py_str("\\b(?:ideas?|dinners?|meals?|lunches?|suppers?|breakfasts?|recipes?|suggestions?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE_STARTS(call_builtin("frozenset", &[mcall(&py_str("i im i'm ive iv we you it its it's that this there they he she\n    meant mean wants want wanted needs need think thought know knew said say says told ask asked\n    just really actually yes no thanks sorry"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP(stdlib_call("re", "compile", &[py_str("^(?:start|begin|use|put|place|leave|let|keep|wipe|rub|mix|make|add|take|coat|scrub|brush|soak|pour|drain|wash|rinse|blot|hang|air|open|close|undo|unscrew|pull|lift|fill|top|cut|trim|cover|seal|spray|check|turn|hold|run|go|get|do|give|ask|book|call|wait|set|sit|stand|watch|pick|dig|move|remove|replace|clean|unplug|vacuum|sweep|boil|simmer|bake|rest|chill|store|freeze|test|look|try|screw|tighten|loosen|slide)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AND_STEP(stdlib_call("re", "compile", &[py_str("\\s+(?:and|then|so|but)\\s+(?=(?:start|begin|use|put|place|leave|let|keep|wipe|rub|mix|make|add|take|coat|scrub|brush|soak|pour|drain|wash|rinse|blot|hang|air|open|close|undo|unscrew|pull|lift|fill|top|cut|trim|cover|seal|spray|check|turn|hold|run|go|get|do|give|ask|book|call|wait|set|sit|stand|watch|pick|dig|move|remove|replace|clean|unplug|vacuum|sweep|boil|simmer|bake|rest|chill|store|freeze|test|look|try|screw|tighten|loosen|slide)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADS(py_dict(vec![(py_str("pet"), py_str("With {who} in the house, "))]));
    s_LONGEST(Py::Int(9));
    s_A_FAULT_IN_A_NAMED_THING(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their|your|its|this|that|these|those)\\s+[a-z][a-z'-]*(?:\\s+[a-z][a-z'-]*){0,2}\\s+(?:has|have|had|'s|is|are|was|were|gets?|got|does|do|keeps?|comes?|shows?|gives?|leaves?|makes?|runs?)\\s+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REASON(stdlib_call("re", "compile", &[py_str("\\s+(?:because|cos|coz|cus|coz of|cos of|because of|seeing as)\\b.*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_IT_IS(stdlib_call("re", "compile", &[py_str("\\s+(?:in|at|on)\\s+(?:the|my|our)\\s+[a-z]+(?: [a-z]+)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PET_IN_THE_REASON(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:because|cos|coz|cus|since|with|of)\\s+(?:it'?s\\s+)?(?:the|my|our|a|an)\\s+(?P<who>"), &g_IN_THE_HOUSE_WORDS()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MOVE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\ntiring strenuous energetic demanding exhausting exhaustive walking walk walks hike hikes hiking\nhill hills stair stairs steps standing stand climbing climb lifting lift far long running run\ncycling cycle bike bikes swimming swim carrying carry bending bend kneeling kneel squat squats\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__LIMIT_FRAME(call_builtin("frozenset", &[mcall(&py_str("\nno not nothing never nowt without too very much any more most lot lots really just only still\ncant cannot can can't able manage do does did done get gets up down and or but the a an my our\nyour his her its it that this those these there here of for in on at with to i we im me us\nbe being is are was were have has had about even anymore either please thanks\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__LEAVES_IT_OUT(stdlib_call("re", "compile", &[py_str("^(?:and |also |but |so |then )*(?:no|nothing|not|never|nowt|without)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JOINING(call_builtin("frozenset", &[mcall(&py_str("of the a an my your our their its some any"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_OF_IT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|be|been|has|have|had|does|do|did|will|can|makes?|keeps?|works?|takes?|needs?|gives?|goes|go|comes?|gets?|stays?|holds?|sits?|starts?|ends?|means?|sounds?|clears?|pulls?|dries?|helps?|turns?|leaves?|leaving|adding|mixing|cooks?|removes?|kills?|stops?|lasts?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURED(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:[.,]\\d+)?\\s*(?:g|kg|ml|l|cl|oz|lb|grams?|kilos?|litres?|millilitres?|tbsp|tsp|tablespoons?|teaspoons?|cups?|pints?|pinch(?:es)?|handfuls?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURED_PARTS(stdlib_call("re", "compile", &[py_str(",|\\band\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AMOUNT(call_builtin("frozenset", &[mcall(&py_str("\nweight amount quantity part parts half quarter third fifth pinch splash dash drizzle\nspoonful spoon spoons tablespoon tablespoons tbsp teaspoon teaspoons tsp cup cups gram grams g kg\nml litre litres ounce ounces oz pound pounds lb handful knob\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVE(stdlib_call("re", "compile", &[py_str("\\b(?:first|start|begin|next|step|steps|change|try)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVE_ASKS(call_builtin("frozenset", &[mcall(&py_str("what whats where when which who why how first start begin next step steps\n    change try do i we you should shall can could would the a an it that this there and then so\n    now best thing things makes make much about on in to"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_APPLIANCES(py_tuple(vec![py_str("oven"), py_str("hob"), py_str("microwave"), py_str("cooker"), py_str("stove")]));
    s_NO_APPLIANCE(py_tuple(vec![py_str("No {thing}, then. {answer}"), py_str("Right, no {thing}. {answer}"), py_str("Without a {thing}, then. {answer}")]));
    s_HOLDS_ON(stdlib_call("re", "compile", &[py_str("\\b(?:it|them|this|that|these|those|the rest)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORDS_IN_A_REPLY(Py::Int(30));
    s_WHO_TO_CALL(stdlib_call("re", "compile", &[py_str("\\b(?:engineer|plumber|electrician|gas safe|corgi|professional|qualified|specialist|expert|call (?:someone|somebody|out)|get (?:someone|somebody|an? [a-z]+ )?(?:in|out))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_TAIL(stdlib_call("re", "compile", &[py_str("^\\d+(?:\\s*(?:-|to)\\s*\\d+)?\\s*(?:minutes?|mins?|hours?|hrs?|seconds?|secs?)(?:\\s*,?\\s*(?:give or take|or so|roughly|about|max|at most))?[.!]?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:\\s*(?:-|to)\\s*\\d+)?\\s*(?:minutes?|mins?|hours?|hrs?|seconds?|secs?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn words_of(args: &[Py]) -> Py {
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
    return stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_fault_in_the_thing(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let _ = py_str("True when the words before the lack name the thing that lacks it.\n\n    \"my shower has no water pressure\" is a shower that is doing something wrong, and the shower's\n    own card is the answer. It is not a person running out of a product, so nothing is cut out of\n    the answer above.\n    ");
    return call_builtin("bool", &[mcall(&g_A_FAULT_IN_A_NAMED_THING(), "search", &[slice_or_raise(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None)])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _shown(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The thing a limit names, as said, without a trailing weight, and as it is named.\n\n    The three are not the same: \"i ran out of oven cleaner for that\" says the thing as \"oven\n    cleaner\", and \"i dont have ants anymore\" is a phrase whose last word says the line is not about\n    having a thing at all, so that word has to be read before it is thrown away.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("thing"), py_str("thing2"), py_str("thing3")]));
    for __item in __iter1 {
    group = __item.clone();
    if py_truthy(&mcall(&mcall(&match_.clone(), "groupdict", &[]), "get", &[group.clone()])) {
    raw = mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[group.clone()]), "split", &[])]);
    said = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:left|anyway|though|tho|either|now|anymore|for that|for it)$")); __args.push(py_str("")); __args.push(raw.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", &[py_str(" .,!?;:'\"")]);
    said = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:but|and|though|although|because|so|unless|whereas)\\s+(?=(?:i|we|you|they|it|there|he|she)\\b)")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    said = _without_a_reason(&[said.clone()]);
    said = mcall(&said.clone(), "strip", &[py_str(" .,!?;:'\"")]);
    said = mcall(&mcall(&g_WHERE_IT_IS(), "sub", &[py_str(""), said.clone()]), "strip", &[py_str(" .,!?;:'\"")]);
    return py_tuple(vec![said.clone(), _bare(&[said.clone()]), raw.clone()]);
    }
    }
    return py_tuple(vec![py_str(""), py_str(""), py_str("")]);
    Py::None
}
pub fn _without_a_reason(args: &[Py]) -> Py {
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
    let _ = py_str("The thing without the reason they said after it: \"fan\" from \"fan because the dog chews\".");
    return mcall(&mcall(&g_A_REASON(), "sub", &[py_str(""), said.clone()]), "strip", &[py_str(" .,!?;:'\"")]);
    Py::None
}
pub fn _a_pet_in_the_reason(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut hit: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The animal or child the limit was explained by, or \"\" when the reason names none.");
    rest = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    hit = mcall(&g_A_PET_IN_THE_REASON(), "search", &[rest.clone()]);
    return if py_truthy(&hit.clone()) { mcall(&hit.clone(), "group", &[py_str("who")]) } else { py_str("") };
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
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("\"oven cleaner\" from \"the oven cleaner\", \"some white spirit\" from \"white spirit\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("any"), py_str("some"), py_str("my"), py_str("our"), py_str("of"), py_str("it")]), &mcall(&word.clone(), "lower", &[])))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = mcall(&py_str(" "), "join", &[words.clone()]); if py_truthy(&__b) { __b } else { said.clone() } };
    Py::None
}
pub fn _the_read(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let _ = py_str("The reading of the line, from the turn that already made one, or a fresh one.");
    read = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("understood")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .,!;:")]), &said.clone())) } }) {
    return read.clone();
    }
    return { let __b = pyrt::xrefs::xcall("understanding", "understand", &[said.clone(), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    Py::None
}
pub fn reads_as_a_limit(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_as: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("(kind, the thing as said, the thing as named) for a line that limits the last answer.");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .,!;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words_of(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_LONGEST()) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&{ let __b = mcall(&_the_read(&[said.clone(), memory.clone()]), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &py_str("recommend")))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("ran_out"), g_OUT_OF()]), py_tuple(vec![py_str("not_have"), g_NOT_HAVE()]), py_tuple(vec![py_str("pet"), g_IN_THE_HOUSE()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    found = mcall(&pattern.clone(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("pet")))) {
    who = _everyone_in_the_house(&[said.clone(), found.clone()]);
    return py_tuple(vec![py_str("pet"), who.clone(), who.clone()]);
    }
    if py_truthy(&mcall(&g_ASKS(), "search", &[mcall(&said.clone(), "replace", &[mcall(&found.clone(), "group", &[Py::Int(0)]), py_str(" ")])])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ASKS_A_CHOICE(), "search", &[mcall(&said.clone(), "replace", &[mcall(&found.clone(), "group", &[Py::Int(0)]), py_str(" ")])])) {
    return Py::None;
    }
    if py_truthy(&_a_fault_in_the_thing(&[said.clone(), found.clone()])) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&_shown(&[found.clone()]), 3, None);
    shown = __unpk3[0].clone();
    bare = __unpk3[1].clone();
    raw = __unpk3[2].clone();
    if py_truthy(&{ let __b = raw.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CLAUSE_STARTS(), &mcall(&mcall(&geti(&mcall(&raw.clone(), "split", &[]), &Py::Int(0)), "lower", &[]), "strip", &[py_str(".,!")]))) } }) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:is|are|was|were|isn'?t|aren'?t)\\b"), { let __b = bare.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bare.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_A_THING(), "search", &[bare.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_THING(), "search", &[raw.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_ASKS(), "search", &[bare.clone()])) {
    return Py::None;
    }
    who = _a_pet_in_the_reason(&[said.clone(), found.clone()]);
    if py_truthy(&who.clone()) {
    answer = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&_last_answer(&[memory.clone()]), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&{ let __b = answer.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_NOT_WITH_A_PET(), "search", &[answer.clone()]) } }) {
    said_as = if py_truthy(&{ let __b = mcall(&who.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&who.clone(), &py_str("children"))) } }) { who.clone() } else { op_add(&py_str("a "), &who.clone()) };
    return py_tuple(vec![py_str("pet"), said_as.clone(), said_as.clone()]);
    }
    }
    return py_tuple(vec![kind.clone(), shown.clone(), bare.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn only_a_limit(args: &[Py]) -> Py {
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
    let mut a_limit_on_the_plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when every word of the line is the limit and none of it is a subject of its own.");
    let __try1 = exc::catch(|| {
    a_limit_on_the_plan = py_func("a_limit_on_the_plan", |args: &[Py]| pyrt::xrefs::xcall("memory.facts", "a_limit_on_the_plan", args));
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
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .,!;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words_of(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_LONGEST()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__LEAVES_IT_OUT(), "match", &[said.clone()])))) {
    return Py::Bool(false);
    }
    let __try6 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_eq(&py_call(&a_limit_on_the_plan, &[said.clone()]), &py_str("move")))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("'")])); } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__MOVE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__LIMIT_FRAME(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _everyone_in_the_house(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut more: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut out: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut seen: Py = Py::None;
    let _ = py_str("\"a dog and a toddler\" from a line that names both of them.");
    names = py_list(vec![{ let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("who2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]);
    rest = slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    while py_truthy(&Py::Bool(true)) {
    more = mcall(&g_ALSO_THERE(), "match", &[rest.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&more.clone()))) {
    break;
    }
    let _ = mcall(&names.clone(), "append", &[mcall(&more.clone(), "group", &[py_str("who3")])]);
    rest = slice_or_raise(&rest.clone(), &mcall(&more.clone(), "end", &[]), &Py::None, &Py::None);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), py_list(vec![])]), 2, None);
    seen = __unpk1[0].clone();
    out = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&names.clone());
    for __item in __iter2 {
    name = __item.clone();
    name = mcall(&py_str(" "), "join", &[mcall(&name.clone(), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&seen.clone(), &mcall(&name.clone(), "lower", &[]))) } }) {
    continue;
    }
    let _ = mcall(&seen.clone(), "add", &[mcall(&name.clone(), "lower", &[])]);
    let _ = mcall(&out.clone(), "append", &[if py_truthy(&{ let __b = mcall(&name.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("children")]), &name.clone())) } }) { name.clone() } else { op_add(&py_str("a "), &name.clone()) }]);
    }
    return mcall(&py_str(" and "), "join", &[out.clone()]);
    Py::None
}
pub fn _pieces(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer as sentences, each split at the commas that open a new step.");
    pieces = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    sentence = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&sentence.clone()))) {
    continue;
    }
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str(",\\s+(?=[a-z])"), sentence.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut clause: Py = __item.clone(); if py_truthy(&mcall(&clause.clone(), "strip", &[])) {  __out.push(mcall(&clause.clone(), "strip", &[])); } } py_list(__out) })() };
    let _ = mcall(&pieces.clone(), "append", &[py_tuple(vec![sentence.clone(), clauses.clone()])]);
    }
    return pieces.clone();
    Py::None
}
pub fn _steps(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut out: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("[(clause, sentence)] - the steps of an answer, in the order they were given.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&_pieces(&[answer.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    sentence = __unpk2[0].clone();
    clauses = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&clauses.clone());
    for __item in __iter3 {
    clause = __item.clone();
    let __iter4 = py_iter_or_raise(&stdlib_call("re", "split", &[g_AND_STEP(), clause.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    step = __item.clone();
    step = mcall(&step.clone(), "strip", &[py_str(" .,;:")]);
    if py_truthy(&step.clone()) {
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![step.clone(), sentence.clone()])]);
    }
    }
    }
    }
    return out.clone();
    Py::None
}
pub fn _names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut forms: Py = Py::None;
    let mut got: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut start: Py = Py::None;
    let mut want: Py = Py::None;
    let mut window: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a clause names the thing, by that thing's own forms: \"oven cleaner\" and \"cleaner\".\n\n    A two-word thing is also found with the joining word left out, and with the words the other way\n    round, because a person says \"bicarbonate of soda\" and a thing's bare form drops the \"of\".\n    ");
    forms = pyrt::xrefs::xcall("memory.topics", "forms", &[thing.clone()]);
    words = call_builtin("set", &[words_of(&[clause.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&forms.clone(), &words.clone())) {
    return Py::Bool(true);
    }
    want = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[thing.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_JOINING(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[want.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    got = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[clause.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_JOINING(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[got.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[want.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    start = __item.clone();
    window = slice_or_raise(&got.clone(), &start.clone(), &op_add(&start.clone(), &call_builtin("len", &[want.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[window.clone(), want.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk9823 = py_unpack(&__item.clone(), 2, None); let mut one: Py = __unpk9823[0].clone(); let mut other: Py = __unpk9823[1].clone(); __out.push({ let __b = Py::Bool(py_eq(&one.clone(), &other.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xcall("memory.topics", "forms", &[other.clone()]), &one.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _first_with(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut _u: Py = Py::None;
    let mut clause: Py = Py::None;
    let _ = py_str("The first step of the answer that names the thing, or \"\" if the answer never does.");
    let __iter1 = py_iter_or_raise(&_steps(&[answer.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    clause = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    if py_truthy(&_names(&[clause.clone(), thing.clone()])) {
    return clause.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_amount_of_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut before: Py = Py::None;
    let mut one: Py = Py::None;
    let mut place: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing is said as an amount: \"the same weight of soft butter\", \"325ml water\".");
    words = words_of(&[clause.clone()]);
    wanted = op_sub(&call_builtin("set", &[words_of(&[thing.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_JOINING());
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    place = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&wanted.clone(), &word.clone()))) {
    continue;
    }
    before = slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&place.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &place.clone(), &Py::None);
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_AN_AMOUNT(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), mcall(&py_str(" "), "join", &[before.clone()])]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _among_measured_things(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing is one item of a measured list: \"500g flour, 7g yeast, 10g salt\".\n\n    The shopping list a recipe opens with reads as a step - \"Mix 500g strong white flour, 7g yeast,\n    10g salt and 325ml warm water into a soft dough\" starts with the verb \"mix\" - so a limit that\n    landed on the yeast was answered \"skip the yeast\" and handed back the rest of a yeasted loaf,\n    which is not a bread anyone can make. The verb is about the list, not about the thing: three or\n    more measured items with the thing among them make it a list, and the knowledge base keeps the\n    turn.\n    ");
    words = mcall(&call_builtin("str", &[{ let __b = clause.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MEASURED(), "search", &[words.clone()]))) } }) {
    return Py::Bool(false);
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_MEASURED_PARTS(), "split", &[words.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(_names(&[part.clone(), thing.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn is_a_thing_on_a_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { py_str("") };
    let mut said: Py = Py::None;
    let mut tidied: Py = Py::None;
    let _ = py_str("True for a step of an answer that only names a thing: \"7g dried yeast\", \"350ml warm water\".\n\n    A list of what a job is made of is not a set of steps: bread without the yeast is not bread\n    with one step fewer, and the answer to \"i havent got any yeast\" is the one that says what to\n    make instead. So a limit that lands on a thing on a list is not answered from the steps; the\n    knowledge base keeps the turn and answers the line itself.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = clause.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    tidied = _tidy(&[said.clone()]);
    if py_truthy(&{ let __b = tidied.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MEASURED(), "search", &[tidied.clone()]))) } }) {
    said = tidied.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _among_measured_things(&[said.clone(), thing.clone()]); if py_truthy(&__b) { __b } else { _the_amount_of_it(&[said.clone(), thing.clone()]) } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(!py_truthy(&mcall(&g_STEP(), "match", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SAID_OF_IT(), "search", &[said.clone()]))) } };
    Py::None
}
pub fn after_an_answer(args: &[Py]) -> Py {
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
    let _ = py_str("True when the turn before this one was an answer to a how-to, not the bot's own chatter.");
    return pyrt::xrefs::xcall("memory.tried", "after_an_answer", &[memory.clone()]);
    Py::None
}
pub fn _last_answer(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut last: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    asked = if py_truthy(&op_gt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(2))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("last_answer")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("question")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) };
    return py_tuple(vec![call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), asked.clone()]);
    Py::None
}
pub fn reads_the_answer(args: &[Py]) -> Py {
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
    let mut _asked: Py = Py::None;
    let mut _thing: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("True when this limit is answered from the last answer's own steps.\n\n    `answers` stands down on this and not on `reads_as_a_limit`, because not every limit is about\n    the answer just given: \"how do i clean my oven\" then \"i dont have any eggs\" is a cooking\n    question, and the answer that helps it is about what replaces an egg in a cake. Only a thing\n    the answer itself asks for, or a product missing after a how-to, belongs here.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&after_an_answer(&[memory.clone()])))) {
    return Py::Bool(false);
    }
    limit = reads_as_a_limit(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&limit.clone()))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&limit.clone(), 3, None);
    kind = __unpk1[0].clone();
    _thing = __unpk1[1].clone();
    bare = __unpk1[2].clone();
    let __unpk2 = py_unpack(&_last_answer(&[memory.clone()]), 2, None);
    answer = __unpk2[0].clone();
    _asked = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("pet")))) {
    return Py::Bool(!py_is(&_with_a_pet(&[answer.clone(), bare.clone()]), &Py::None));
    }
    named = _first_with(&[answer.clone(), bare.clone()]);
    if py_truthy(&named.clone()) {
    return Py::Bool(!py_truthy(&is_a_thing_on_a_list(&[named.clone(), bare.clone()])));
    }
    if py_truthy(&_without_the_appliance(&[bare.clone(), answer.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = pyrt::xrefs::xcall("memory.tried", "asks_for_a_fix", &[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = _about_the_job(&[bare.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { _a_fault_in_the_line_answered(&[memory.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&a_detail_of_a_fault(&[memory.clone(), bare.clone()]))) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_line_answered(args: &[Py]) -> Py {
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
    let _ = py_str("The person's line that the answer above was written for.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&turn.clone(), "get", &[py_str("bot")])) {
    return call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("last_answer")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("question")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_fault_in_the_line_answered(args: &[Py]) -> Py {
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
    let mut reported: Py = Py::None;
    let _ = py_str("True when the answer above was written for a thing that has gone wrong.");
    reported = _the_line_answered(&[memory.clone()]);
    return { let __b = call_builtin("bool", &[reported.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&pyrt::xrefs::xget("memory.tried", "FAULT"), "search", &[mcall(&reported.clone(), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _about_the_job(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut bare: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bare") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bare"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut named_words: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the thing they lack is part of the job the answer above was for.\n\n    \"my shower has no water pressure\" and \"i have no eggs\" are not about the fruit fly answer\n    above them, and saying \"You don't need water pressure for that\" hung an answer about flies on\n    a fault in a shower. What a how-to may call unnecessary is a product of its own job, so the\n    thing has to be named by the line that was answered or by the answer itself.\n    ");
    named_words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&words_of(&[_the_line_answered(&[memory.clone()])]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())) } }) {
    let __aug2 = op_bitor(&named_words.clone(), &pyrt::xrefs::xcall("memory.topics", "forms", &[word.clone()]));
    named_words = __aug2;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&named_words.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[bare.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())) } }) {  __out.push(op_bitand(&named_words.clone(), &pyrt::xrefs::xcall("memory.topics", "forms", &[word.clone()]))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut reply: Py = Py::None;
    let _ = py_str("The reply a limit on the last answer deserves, or None to leave the turn alone.");
    reply = _limit_reply(&[text.clone(), memory.clone()]);
    if py_truthy(&reply.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("limit"), py_dict(vec![(py_str("said"), reply.clone()), (py_str("steps"), _steps_the_limit_left(&[text.clone(), memory.clone()]))]));
    }
    return reply.clone();
    Py::None
}
pub fn move_after_a_limit(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut record: Py = Py::None;
    let mut step: Py = Py::None;
    let mut steps: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The first step the limit left, when the line right after it asks for the move.\n\n    A limit reply is read from the answer behind it, so the answer is no longer the turn on the\n    screen: the move ask would find nothing to pick from and stop at a search hedge, when the step\n    it wants is the first one the limit kept.\n    ");
    record = mcall(&memory.clone(), "get", &[py_str("limit")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), &mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("said")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))) } }) {
    let _ = mcall(&memory.clone(), "pop", &[py_str("limit"), Py::None]);
    return Py::None;
    }
    line = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_MOVE(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words_of(&[line.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = pyrt::xrefs::xcall("memory.topics", "pairing", &[line.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_no_thing(&[line.clone()]))) } }) {
    return Py::None;
    }
    steps = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("steps")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut step: Py = __item.clone(); if py_truthy(&step.clone()) {  __out.push(step.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&steps.clone()))) {
    return Py::None;
    }
    step = mcall(&mcall(&geti(&steps.clone(), &Py::Int(0)), "strip", &[]), "rstrip", &[py_str(". ")]);
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return Py::None;
    }
    return op_add(&op_add(&op_add(&py_str("I would start here: "), &mcall(&geti(&step.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn _names_no_thing(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names nothing of its own: \"whats the first thing to change\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[line.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.topics", "STOPWORDS"), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_MOVE_ASKS(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return Py::Bool(!py_truthy(&words.clone()));
    Py::None
}
pub fn _steps_the_limit_left(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut _asked: Py = Py::None;
    let mut _thing: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The steps of the last answer that the limit does not rule out.");
    limit = reads_as_a_limit(&[text.clone(), memory.clone()]);
    let __unpk1 = py_unpack(&_last_answer(&[memory.clone()]), 2, None);
    answer = __unpk1[0].clone();
    _asked = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&limit.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&answer.clone())) } }) {
    return py_list(vec![]);
    }
    let __unpk2 = py_unpack(&limit.clone(), 3, None);
    kind = __unpk2[0].clone();
    _thing = __unpk2[1].clone();
    bare = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("pet")))) {
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_steps(&[answer.clone()])) { let __unpk6235 = py_unpack(&__item.clone(), 2, None); let mut step: Py = __unpk6235[0].clone(); let mut _u: Py = __unpk6235[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NOT_WITH_A_PET(), "search", &[step.clone()])))) {  __out.push(step.clone()); } } py_list(__out) })() };
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_steps(&[answer.clone()])) { let __unpk7781 = py_unpack(&__item.clone(), 2, None); let mut step: Py = __unpk7781[0].clone(); let mut _u: Py = __unpk7781[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&_names(&[step.clone(), bare.clone()])))) {  __out.push(step.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn a_detail_of_a_fault(args: &[Py]) -> Py {
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
    let mut bare: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bare") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bare"))) };
    let mut line: Py = Py::None;
    let mut reported: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the line says the fault again, in the words of the answer that was just given.\n\n    \"my boiler is broken\" then \"no hot water\" is the same fault twice: the person is not telling the\n    answer that it may leave the water out, they are telling it what the boiler is doing wrong. The\n    knowledge base holds the boiler card, so this reader stands down where a product would have been\n    cut out of the answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_DETAIL_OF_THE_FAULT(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = bare.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])))) {
    return Py::Bool(false);
    }
    reported = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&turn.clone(), "get", &[py_str("bot")])) {
    reported = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&reported.clone()))) {
    return Py::Bool(false);
    }
    line = mcall(&reported.clone(), "lower", &[]);
    return { let __b = call_builtin("bool", &[mcall(&g_A_HEATING_FAULT(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&pyrt::xrefs::xget("memory.tried", "FAULT"), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_LEAKING_FAULT(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.tried", "buying", &[memory.clone()]))) } } } } } };
    Py::None
}
pub fn _without_the_appliance(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut alternative: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut cooking: Py = Py::None;
    let mut name: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("What to cook without it, when the answer in front needs the thing that has gone.");
    name = mcall(&mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_APPLIANCES(), &name.clone()))) {
    return Py::None;
    }
    cooking = pyrt::xrefs::xmodule("knowledge.cooking");
    asked = op_add(&py_str("what can i cook with no "), &name.clone());
    understood = py_dict(vec![(py_str("clean"), asked.clone()), (py_str("words"), mcall(&asked.clone(), "split", &[])), (py_str("excludes"), py_list(vec![name.clone()])), (py_str("constraint"), Py::Bool(true))]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&cooking.clone(), "missing", &[understood.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&cooking.clone(), "unusable", &[answer.clone(), understood.clone()])))) {
    return Py::None;
    }
    alternative = mcall(&cooking.clone(), "without", &[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&alternative.clone()))) {
    return Py::None;
    }
    alternative = op_add(&mcall(&slice_or_raise(&alternative.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&alternative.clone(), &Py::Int(1), &Py::None, &Py::None));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), name.clone()), ("answer".to_string(), alternative.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_NO_APPLIANCE(), memory.clone(), name.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _limit_reply(args: &[Py]) -> Py {
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
    let mut _asked: Py = Py::None;
    let mut alternative: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut named: Py = Py::None;
    let mut steps: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut verbatim: Py = Py::None;
    let _ = py_str("The reply a limit on the last answer deserves, or None to leave the turn alone.");
    if py_truthy(&Py::Bool(!py_truthy(&after_an_answer(&[memory.clone()])))) {
    return Py::None;
    }
    limit = reads_as_a_limit(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&limit.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&limit.clone(), 3, None);
    kind = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    bare = __unpk1[2].clone();
    let __unpk2 = py_unpack(&_last_answer(&[memory.clone()]), 2, None);
    answer = __unpk2[0].clone();
    _asked = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("pet")))) {
    return _with_a_pet(&[answer.clone(), bare.clone()]);
    }
    if py_truthy(&a_detail_of_a_fault(&[memory.clone(), bare.clone()])) {
    return Py::None;
    }
    steps = _steps(&[answer.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&steps.clone()))) {
    return Py::None;
    }
    named = _first_with(&[answer.clone(), bare.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    alternative = _without_the_appliance(&[bare.clone(), answer.clone(), memory.clone()]);
    if py_truthy(&alternative.clone()) {
    return alternative.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_a_fault_in_the_line_answered(&[memory.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.tried", "asks_for_a_fix", &[memory.clone()]))) } }) {
    return Py::None;
    }
    return _say(&[op_add(&op_add(&py_str("You don't need "), &thing.clone()), &py_str(" for that: ")), _up_to_words(&[_kept(&[answer.clone(), bare.clone()])])]);
    }
    if py_truthy(&is_a_thing_on_a_list(&[named.clone(), bare.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _names(&[geti(&geti(&steps.clone(), &Py::Int(0)), &Py::Int(0)), bare.clone()]); if !py_truthy(&__b) { __b } else { _holds_on(&[answer.clone(), bare.clone()]) } }) {
    return _needs_it(&[thing.clone(), answer.clone()]);
    }
    kept = _without_a_repeated_time(&[_kept(&[answer.clone(), bare.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    return Py::None;
    }
    verbatim = if py_truthy(&_names(&[named.clone(), thing.clone()])) { thing.clone() } else { _bare_phrase(&[named.clone(), bare.clone()]) };
    return _say(&[op_add(&op_add(&py_str("Skip the "), &verbatim.clone()), &py_str(": ")), _up_to_words(&[kept.clone()])]);
    Py::None
}
pub fn _kept(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut bare: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bare") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bare"))) };
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut out: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The answer with the steps that name the thing cut out, one entry for each sentence left.\n\n    Whole sentences and not single clauses, because a clause of a repair is not a repair: cutting\n    the white wine out of \"blot it, don't rub, with kitchen paper, then pour on cold water\" and\n    stopping at two clauses left \"blot it, don't rub\", which is half of the first thing to do.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&_pieces(&[answer.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    sentence = __unpk2[0].clone();
    clauses = __unpk2[1].clone();
    kept = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&clauses.clone());
    for __item in __iter3 {
    clause = __item.clone();
    let __iter4 = py_iter_or_raise(&stdlib_call("re", "split", &[g_AND_STEP(), clause.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    step = __item.clone();
    step = mcall(&step.clone(), "strip", &[py_str(" .,;:")]);
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names(&[step.clone(), bare.clone()]))) } }) {
    let _ = mcall(&kept.clone(), "append", &[step.clone()]);
    }
    }
    }
    if py_truthy(&kept.clone()) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(", "), "join", &[kept.clone()])]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _without_a_repeated_time(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut out: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("Drop a closing time estimate when the steps left already say how long something takes.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_TIME_TAIL(), "match", &[mcall(&sentence.clone(), "strip", &[])]); if !py_truthy(&__b) { __b } else { mcall(&g_A_TIME(), "search", &[mcall(&py_str(" "), "join", &[out.clone()])]) } } } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[sentence.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _up_to_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentences: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentences") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentences"))) };
    let mut count: Py = Py::None;
    let mut out: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first of them, and as many more as a reply can carry.");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    out = __unpk1[0].clone();
    count = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&sentences.clone());
    for __item in __iter2 {
    sentence = __item.clone();
    words = call_builtin("len", &[words_of(&[sentence.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { op_gt(&op_add(&count.clone(), &words.clone()), &g_WORDS_IN_A_REPLY()) } }) {
    break;
    }
    let _ = mcall(&out.clone(), "append", &[sentence.clone()]);
    let __aug3 = op_add(&count.clone(), &words.clone());
    count = __aug3;
    }
    return out.clone();
    Py::None
}
pub fn _holds_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut bare: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bare") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bare"))) };
    let mut kept: Py = Py::None;
    let _ = py_str("True when what the answer has left is done to what the missing step makes.");
    kept = _kept(&[answer.clone(), bare.clone()]);
    return { let __b = call_builtin("bool", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_HOLDS_ON(), "search", &[geti(&kept.clone(), &Py::Int(0))])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _needs_it(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut _clauses: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The reply when the step that is out is the one the rest of the answer is done to.");
    lead = op_add(&op_add(&py_str("You can't do the first step without the "), &thing.clone()), &py_str(", and the rest of that answer is done after it. "));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[_pieces(&[answer.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    sentence = __unpk2[0].clone();
    _clauses = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&g_WHO_TO_CALL(), "search", &[sentence.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names(&[sentence.clone(), thing.clone()]))) } }) {
    return op_add(&lead.clone(), &mcall(&sentence.clone(), "strip", &[]));
    }
    }
    return mcall(&lead.clone(), "rstrip", &[]);
    Py::None
}
pub fn _bare_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut named: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut bare: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bare") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bare"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("\"plunger\" from \"Start with a plunger\", so the step can be named in six words.");
    words = words_of(&[named.clone()]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xcall("memory.topics", "forms", &[bare.clone()]), &word.clone()))) {
    return word.clone();
    }
    }
    return bare.clone();
    Py::None
}
pub fn _with_a_pet(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut _u: Py = Py::None;
    let mut hazard: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut named: Py = Py::None;
    let mut safe: Py = Py::None;
    let mut step: Py = Py::None;
    let mut steps: Py = Py::None;
    let _ = py_str("The steps a cat or a child does not rule out, said with the reason they are missing.");
    steps = _steps(&[answer.clone()]);
    hazard = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&steps.clone()) { let __unpk6673 = py_unpack(&__item.clone(), 2, None); let mut step: Py = __unpk6673[0].clone(); let mut _u: Py = __unpk6673[1].clone(); if py_truthy(&mcall(&g_NOT_WITH_A_PET(), "search", &[step.clone()])) {  __out.push(step.clone()); } } py_list(__out) })() };
    safe = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&steps.clone()) { let __unpk2050 = py_unpack(&__item.clone(), 2, None); let mut step: Py = __unpk2050[0].clone(); let mut _u: Py = __unpk2050[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NOT_WITH_A_PET(), "search", &[step.clone()])))) {  __out.push(step.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&hazard.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&safe.clone())) } }) {
    return Py::None;
    }
    named = mcall(&mcall(&mcall(&g_NOT_WITH_A_PET(), "search", &[geti(&hazard.clone(), &Py::Int(0))]), "group", &[Py::Int(0)]), "lower", &[]);
    lead = op_add(&op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_LEADS(), &py_str("pet")), "format", args))(&__args) }, &py_str("skip the ")), &named.clone()), &py_str(": "));
    return _say(&[lead.clone(), safe.clone()]);
    Py::None
}
pub fn _say(args: &[Py]) -> Py {
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
    let mut steps: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "steps") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: steps"))) };
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("The lead and up to two steps, said as one sentence somebody would want to read.");
    said = _tidy(&[geti(&steps.clone(), &Py::Int(0))]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])]))) } }) {
    second = _tidy(&[geti(&steps.clone(), &Py::Int(1))]);
    let __aug1 = op_add(&said.clone(), &if py_truthy(&mcall(&slice_or_raise(&second.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) { op_add(&py_str(". "), &second.clone()) } else { op_add(&py_str(", "), &second.clone()) });
    said = __aug1;
    }
    said = mcall(&said.clone(), "rstrip", &[py_str(" ,;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])))) {
    let __aug2 = op_add(&said.clone(), &py_str("."));
    said = __aug2;
    }
    return if py_truthy(&lead.clone()) { op_add(&op_add(&lead.clone(), &mcall(&geti(&said.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)) };
    Py::None
}
pub fn _tidy(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut said: Py = Py::None;
    let _ = py_str("\"undo the trap underneath (bucket first)\" reads as it was said, without the joining word.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|then|so|but|also)\\s+")); __args.push(py_str("")); __args.push(mcall(&step.clone(), "strip", &[])); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return mcall(&said.clone(), "strip", &[]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.limit".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ALSO_THERE" => Some(g_ALSO_THERE()),
            "AND_STEP" => Some(g_AND_STEP()),
            "AN_AMOUNT" => Some(g_AN_AMOUNT()),
            "APPLIANCES" => Some(g_APPLIANCES()),
            "ASKS" => Some(g_ASKS()),
            "ASKS_A_CHOICE" => Some(g_ASKS_A_CHOICE()),
            "A_DETAIL_OF_THE_FAULT" => Some(g_A_DETAIL_OF_THE_FAULT()),
            "A_FAULT_IN_A_NAMED_THING" => Some(g_A_FAULT_IN_A_NAMED_THING()),
            "A_HEATING_FAULT" => Some(g_A_HEATING_FAULT()),
            "A_LEAKING_FAULT" => Some(g_A_LEAKING_FAULT()),
            "A_PET_IN_THE_REASON" => Some(g_A_PET_IN_THE_REASON()),
            "A_REASON" => Some(g_A_REASON()),
            "A_TIME" => Some(g_A_TIME()),
            "CLAUSE_STARTS" => Some(g_CLAUSE_STARTS()),
            "HOLDS_ON" => Some(g_HOLDS_ON()),
            "IN_THE_HOUSE" => Some(g_IN_THE_HOUSE()),
            "IN_THE_HOUSE_WORDS" => Some(g_IN_THE_HOUSE_WORDS()),
            "JOINING" => Some(g_JOINING()),
            "LEADS" => Some(g_LEADS()),
            "LONGEST" => Some(g_LONGEST()),
            "MEASURED" => Some(g_MEASURED()),
            "MEASURED_PARTS" => Some(g_MEASURED_PARTS()),
            "MOVE" => Some(g_MOVE()),
            "MOVE_ASKS" => Some(g_MOVE_ASKS()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_HAVE" => Some(g_NOT_HAVE()),
            "NOT_WITH_A_PET" => Some(g_NOT_WITH_A_PET()),
            "NO_APPLIANCE" => Some(g_NO_APPLIANCE()),
            "OUT_OF" => Some(g_OUT_OF()),
            "SAID_OF_IT" => Some(g_SAID_OF_IT()),
            "STEP" => Some(g_STEP()),
            "TIME_TAIL" => Some(g_TIME_TAIL()),
            "WHERE_IT_IS" => Some(g_WHERE_IT_IS()),
            "WHO_TO_CALL" => Some(g_WHO_TO_CALL()),
            "WORDS_IN_A_REPLY" => Some(g_WORDS_IN_A_REPLY()),
            "_LEAVES_IT_OUT" => Some(g__LEAVES_IT_OUT()),
            "_LIMIT_FRAME" => Some(g__LIMIT_FRAME()),
            "_MOVE_WORDS" => Some(g__MOVE_WORDS()),
            "_a_fault_in_the_line_answered" => Some(py_func("_a_fault_in_the_line_answered", |args: &[Py]| _a_fault_in_the_line_answered(args))),
            "_a_fault_in_the_thing" => Some(py_func("_a_fault_in_the_thing", |args: &[Py]| _a_fault_in_the_thing(args))),
            "_a_pet_in_the_reason" => Some(py_func("_a_pet_in_the_reason", |args: &[Py]| _a_pet_in_the_reason(args))),
            "_about_the_job" => Some(py_func("_about_the_job", |args: &[Py]| _about_the_job(args))),
            "_among_measured_things" => Some(py_func("_among_measured_things", |args: &[Py]| _among_measured_things(args))),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "_bare_phrase" => Some(py_func("_bare_phrase", |args: &[Py]| _bare_phrase(args))),
            "_everyone_in_the_house" => Some(py_func("_everyone_in_the_house", |args: &[Py]| _everyone_in_the_house(args))),
            "_first_with" => Some(py_func("_first_with", |args: &[Py]| _first_with(args))),
            "_holds_on" => Some(py_func("_holds_on", |args: &[Py]| _holds_on(args))),
            "_kept" => Some(py_func("_kept", |args: &[Py]| _kept(args))),
            "_last_answer" => Some(py_func("_last_answer", |args: &[Py]| _last_answer(args))),
            "_limit_reply" => Some(py_func("_limit_reply", |args: &[Py]| _limit_reply(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_names_no_thing" => Some(py_func("_names_no_thing", |args: &[Py]| _names_no_thing(args))),
            "_needs_it" => Some(py_func("_needs_it", |args: &[Py]| _needs_it(args))),
            "_pieces" => Some(py_func("_pieces", |args: &[Py]| _pieces(args))),
            "_say" => Some(py_func("_say", |args: &[Py]| _say(args))),
            "_shown" => Some(py_func("_shown", |args: &[Py]| _shown(args))),
            "_steps" => Some(py_func("_steps", |args: &[Py]| _steps(args))),
            "_steps_the_limit_left" => Some(py_func("_steps_the_limit_left", |args: &[Py]| _steps_the_limit_left(args))),
            "_the_amount_of_it" => Some(py_func("_the_amount_of_it", |args: &[Py]| _the_amount_of_it(args))),
            "_the_line_answered" => Some(py_func("_the_line_answered", |args: &[Py]| _the_line_answered(args))),
            "_the_read" => Some(py_func("_the_read", |args: &[Py]| _the_read(args))),
            "_tidy" => Some(py_func("_tidy", |args: &[Py]| _tidy(args))),
            "_up_to_words" => Some(py_func("_up_to_words", |args: &[Py]| _up_to_words(args))),
            "_with_a_pet" => Some(py_func("_with_a_pet", |args: &[Py]| _with_a_pet(args))),
            "_without_a_reason" => Some(py_func("_without_a_reason", |args: &[Py]| _without_a_reason(args))),
            "_without_a_repeated_time" => Some(py_func("_without_a_repeated_time", |args: &[Py]| _without_a_repeated_time(args))),
            "_without_the_appliance" => Some(py_func("_without_the_appliance", |args: &[Py]| _without_the_appliance(args))),
            "a_detail_of_a_fault" => Some(py_func("a_detail_of_a_fault", |args: &[Py]| a_detail_of_a_fault(args))),
            "after_an_answer" => Some(py_func("after_an_answer", |args: &[Py]| after_an_answer(args))),
            "is_a_thing_on_a_list" => Some(py_func("is_a_thing_on_a_list", |args: &[Py]| is_a_thing_on_a_list(args))),
            "move_after_a_limit" => Some(py_func("move_after_a_limit", |args: &[Py]| move_after_a_limit(args))),
            "only_a_limit" => Some(py_func("only_a_limit", |args: &[Py]| only_a_limit(args))),
            "pick" => Some(g_pick()),
            "reads_as_a_limit" => Some(py_func("reads_as_a_limit", |args: &[Py]| reads_as_a_limit(args))),
            "reads_the_answer" => Some(py_func("reads_the_answer", |args: &[Py]| reads_the_answer(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "understanding" => Some(g_understanding()),
            "words_of" => Some(py_func("words_of", |args: &[Py]| words_of(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALSO_THERE" => s_ALSO_THERE(v),
            "AND_STEP" => s_AND_STEP(v),
            "AN_AMOUNT" => s_AN_AMOUNT(v),
            "APPLIANCES" => s_APPLIANCES(v),
            "ASKS" => s_ASKS(v),
            "ASKS_A_CHOICE" => s_ASKS_A_CHOICE(v),
            "A_DETAIL_OF_THE_FAULT" => s_A_DETAIL_OF_THE_FAULT(v),
            "A_FAULT_IN_A_NAMED_THING" => s_A_FAULT_IN_A_NAMED_THING(v),
            "A_HEATING_FAULT" => s_A_HEATING_FAULT(v),
            "A_LEAKING_FAULT" => s_A_LEAKING_FAULT(v),
            "A_PET_IN_THE_REASON" => s_A_PET_IN_THE_REASON(v),
            "A_REASON" => s_A_REASON(v),
            "A_TIME" => s_A_TIME(v),
            "CLAUSE_STARTS" => s_CLAUSE_STARTS(v),
            "HOLDS_ON" => s_HOLDS_ON(v),
            "IN_THE_HOUSE" => s_IN_THE_HOUSE(v),
            "IN_THE_HOUSE_WORDS" => s_IN_THE_HOUSE_WORDS(v),
            "JOINING" => s_JOINING(v),
            "LEADS" => s_LEADS(v),
            "LONGEST" => s_LONGEST(v),
            "MEASURED" => s_MEASURED(v),
            "MEASURED_PARTS" => s_MEASURED_PARTS(v),
            "MOVE" => s_MOVE(v),
            "MOVE_ASKS" => s_MOVE_ASKS(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_HAVE" => s_NOT_HAVE(v),
            "NOT_WITH_A_PET" => s_NOT_WITH_A_PET(v),
            "NO_APPLIANCE" => s_NO_APPLIANCE(v),
            "OUT_OF" => s_OUT_OF(v),
            "SAID_OF_IT" => s_SAID_OF_IT(v),
            "STEP" => s_STEP(v),
            "TIME_TAIL" => s_TIME_TAIL(v),
            "WHERE_IT_IS" => s_WHERE_IT_IS(v),
            "WHO_TO_CALL" => s_WHO_TO_CALL(v),
            "WORDS_IN_A_REPLY" => s_WORDS_IN_A_REPLY(v),
            "_LEAVES_IT_OUT" => s__LEAVES_IT_OUT(v),
            "_LIMIT_FRAME" => s__LIMIT_FRAME(v),
            "_MOVE_WORDS" => s__MOVE_WORDS(v),
            "pick" => s_pick(v),
            "understanding" => s_understanding(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.limit", module_obj);
    pyrt::xrefs::reg_fn("memory.limit", "words_of", py_func("words_of", |args: &[Py]| words_of(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_a_fault_in_the_thing", py_func("_a_fault_in_the_thing", |args: &[Py]| _a_fault_in_the_thing(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_shown", py_func("_shown", |args: &[Py]| _shown(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_without_a_reason", py_func("_without_a_reason", |args: &[Py]| _without_a_reason(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_a_pet_in_the_reason", py_func("_a_pet_in_the_reason", |args: &[Py]| _a_pet_in_the_reason(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_the_read", py_func("_the_read", |args: &[Py]| _the_read(args)));
    pyrt::xrefs::reg_fn("memory.limit", "reads_as_a_limit", py_func("reads_as_a_limit", |args: &[Py]| reads_as_a_limit(args)));
    pyrt::xrefs::reg_fn("memory.limit", "only_a_limit", py_func("only_a_limit", |args: &[Py]| only_a_limit(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_everyone_in_the_house", py_func("_everyone_in_the_house", |args: &[Py]| _everyone_in_the_house(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_pieces", py_func("_pieces", |args: &[Py]| _pieces(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_steps", py_func("_steps", |args: &[Py]| _steps(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_names", py_func("_names", |args: &[Py]| _names(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_first_with", py_func("_first_with", |args: &[Py]| _first_with(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_the_amount_of_it", py_func("_the_amount_of_it", |args: &[Py]| _the_amount_of_it(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_among_measured_things", py_func("_among_measured_things", |args: &[Py]| _among_measured_things(args)));
    pyrt::xrefs::reg_fn("memory.limit", "is_a_thing_on_a_list", py_func("is_a_thing_on_a_list", |args: &[Py]| is_a_thing_on_a_list(args)));
    pyrt::xrefs::reg_fn("memory.limit", "after_an_answer", py_func("after_an_answer", |args: &[Py]| after_an_answer(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_last_answer", py_func("_last_answer", |args: &[Py]| _last_answer(args)));
    pyrt::xrefs::reg_fn("memory.limit", "reads_the_answer", py_func("reads_the_answer", |args: &[Py]| reads_the_answer(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_the_line_answered", py_func("_the_line_answered", |args: &[Py]| _the_line_answered(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_a_fault_in_the_line_answered", py_func("_a_fault_in_the_line_answered", |args: &[Py]| _a_fault_in_the_line_answered(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_about_the_job", py_func("_about_the_job", |args: &[Py]| _about_the_job(args)));
    pyrt::xrefs::reg_fn("memory.limit", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("memory.limit", "move_after_a_limit", py_func("move_after_a_limit", |args: &[Py]| move_after_a_limit(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_names_no_thing", py_func("_names_no_thing", |args: &[Py]| _names_no_thing(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_steps_the_limit_left", py_func("_steps_the_limit_left", |args: &[Py]| _steps_the_limit_left(args)));
    pyrt::xrefs::reg_fn("memory.limit", "a_detail_of_a_fault", py_func("a_detail_of_a_fault", |args: &[Py]| a_detail_of_a_fault(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_without_the_appliance", py_func("_without_the_appliance", |args: &[Py]| _without_the_appliance(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_limit_reply", py_func("_limit_reply", |args: &[Py]| _limit_reply(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_kept", py_func("_kept", |args: &[Py]| _kept(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_without_a_repeated_time", py_func("_without_a_repeated_time", |args: &[Py]| _without_a_repeated_time(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_up_to_words", py_func("_up_to_words", |args: &[Py]| _up_to_words(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_holds_on", py_func("_holds_on", |args: &[Py]| _holds_on(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_needs_it", py_func("_needs_it", |args: &[Py]| _needs_it(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_bare_phrase", py_func("_bare_phrase", |args: &[Py]| _bare_phrase(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_with_a_pet", py_func("_with_a_pet", |args: &[Py]| _with_a_pet(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_say", py_func("_say", |args: &[Py]| _say(args)));
    pyrt::xrefs::reg_fn("memory.limit", "_tidy", py_func("_tidy", |args: &[Py]| _tidy(args)));
    pyrt::xrefs::reg_access("memory.limit", "ALSO_THERE", g_ALSO_THERE, s_ALSO_THERE);
    pyrt::xrefs::reg_access("memory.limit", "AND_STEP", g_AND_STEP, s_AND_STEP);
    pyrt::xrefs::reg_access("memory.limit", "AN_AMOUNT", g_AN_AMOUNT, s_AN_AMOUNT);
    pyrt::xrefs::reg_access("memory.limit", "APPLIANCES", g_APPLIANCES, s_APPLIANCES);
    pyrt::xrefs::reg_access("memory.limit", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("memory.limit", "ASKS_A_CHOICE", g_ASKS_A_CHOICE, s_ASKS_A_CHOICE);
    pyrt::xrefs::reg_access("memory.limit", "A_DETAIL_OF_THE_FAULT", g_A_DETAIL_OF_THE_FAULT, s_A_DETAIL_OF_THE_FAULT);
    pyrt::xrefs::reg_access("memory.limit", "A_FAULT_IN_A_NAMED_THING", g_A_FAULT_IN_A_NAMED_THING, s_A_FAULT_IN_A_NAMED_THING);
    pyrt::xrefs::reg_access("memory.limit", "A_HEATING_FAULT", g_A_HEATING_FAULT, s_A_HEATING_FAULT);
    pyrt::xrefs::reg_access("memory.limit", "A_LEAKING_FAULT", g_A_LEAKING_FAULT, s_A_LEAKING_FAULT);
    pyrt::xrefs::reg_access("memory.limit", "A_PET_IN_THE_REASON", g_A_PET_IN_THE_REASON, s_A_PET_IN_THE_REASON);
    pyrt::xrefs::reg_access("memory.limit", "A_REASON", g_A_REASON, s_A_REASON);
    pyrt::xrefs::reg_access("memory.limit", "A_TIME", g_A_TIME, s_A_TIME);
    pyrt::xrefs::reg_access("memory.limit", "CLAUSE_STARTS", g_CLAUSE_STARTS, s_CLAUSE_STARTS);
    pyrt::xrefs::reg_access("memory.limit", "HOLDS_ON", g_HOLDS_ON, s_HOLDS_ON);
    pyrt::xrefs::reg_access("memory.limit", "IN_THE_HOUSE", g_IN_THE_HOUSE, s_IN_THE_HOUSE);
    pyrt::xrefs::reg_access("memory.limit", "IN_THE_HOUSE_WORDS", g_IN_THE_HOUSE_WORDS, s_IN_THE_HOUSE_WORDS);
    pyrt::xrefs::reg_access("memory.limit", "JOINING", g_JOINING, s_JOINING);
    pyrt::xrefs::reg_access("memory.limit", "LEADS", g_LEADS, s_LEADS);
    pyrt::xrefs::reg_access("memory.limit", "LONGEST", g_LONGEST, s_LONGEST);
    pyrt::xrefs::reg_access("memory.limit", "MEASURED", g_MEASURED, s_MEASURED);
    pyrt::xrefs::reg_access("memory.limit", "MEASURED_PARTS", g_MEASURED_PARTS, s_MEASURED_PARTS);
    pyrt::xrefs::reg_access("memory.limit", "MOVE", g_MOVE, s_MOVE);
    pyrt::xrefs::reg_access("memory.limit", "MOVE_ASKS", g_MOVE_ASKS, s_MOVE_ASKS);
    pyrt::xrefs::reg_access("memory.limit", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.limit", "NOT_HAVE", g_NOT_HAVE, s_NOT_HAVE);
    pyrt::xrefs::reg_access("memory.limit", "NOT_WITH_A_PET", g_NOT_WITH_A_PET, s_NOT_WITH_A_PET);
    pyrt::xrefs::reg_access("memory.limit", "NO_APPLIANCE", g_NO_APPLIANCE, s_NO_APPLIANCE);
    pyrt::xrefs::reg_access("memory.limit", "OUT_OF", g_OUT_OF, s_OUT_OF);
    pyrt::xrefs::reg_access("memory.limit", "SAID_OF_IT", g_SAID_OF_IT, s_SAID_OF_IT);
    pyrt::xrefs::reg_access("memory.limit", "STEP", g_STEP, s_STEP);
    pyrt::xrefs::reg_access("memory.limit", "TIME_TAIL", g_TIME_TAIL, s_TIME_TAIL);
    pyrt::xrefs::reg_access("memory.limit", "WHERE_IT_IS", g_WHERE_IT_IS, s_WHERE_IT_IS);
    pyrt::xrefs::reg_access("memory.limit", "WHO_TO_CALL", g_WHO_TO_CALL, s_WHO_TO_CALL);
    pyrt::xrefs::reg_access("memory.limit", "WORDS_IN_A_REPLY", g_WORDS_IN_A_REPLY, s_WORDS_IN_A_REPLY);
    pyrt::xrefs::reg_access("memory.limit", "_LEAVES_IT_OUT", g__LEAVES_IT_OUT, s__LEAVES_IT_OUT);
    pyrt::xrefs::reg_access("memory.limit", "_LIMIT_FRAME", g__LIMIT_FRAME, s__LIMIT_FRAME);
    pyrt::xrefs::reg_access("memory.limit", "_MOVE_WORDS", g__MOVE_WORDS, s__MOVE_WORDS);
    pyrt::xrefs::reg_access("memory.limit", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("memory.limit", "understanding", g_understanding, s_understanding);
}
