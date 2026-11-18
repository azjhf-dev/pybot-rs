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
thread_local! { static G_ASKS_A_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAMED_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PUDDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRY_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKS_AROUND_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECIDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_FROM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREE_FROM_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PUDDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_AS_A_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALITY_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALITY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_MOST_THINGS_A_VERDICT_TRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.carry: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_A_VERDICT() -> Py { ensure_init(); G_ASKS_A_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_VERDICT(v: Py) { ensure_init(); G_ASKS_A_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE() -> Py { ensure_init(); G_A_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE(v: Py) { ensure_init(); G_A_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB() -> Py { ensure_init(); G_A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB(v: Py) { ensure_init(); G_A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEAL() -> Py { ensure_init(); G_A_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEAL(v: Py) { ensure_init(); G_A_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAMED_THING() -> Py { ensure_init(); G_A_NAMED_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAMED_THING(v: Py) { ensure_init(); G_A_NAMED_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN() -> Py { ensure_init(); G_A_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN(v: Py) { ensure_init(); G_A_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PUDDING() -> Py { ensure_init(); G_A_PUDDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PUDDING(v: Py) { ensure_init(); G_A_PUDDING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE() -> Py { ensure_init(); G_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE(v: Py) { ensure_init(); G_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRY_STOP() -> Py { ensure_init(); G_CARRY_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRY_STOP(v: Py) { ensure_init(); G_CARRY_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION() -> Py { ensure_init(); G_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION(v: Py) { ensure_init(); G_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKS_AROUND_IT() -> Py { ensure_init(); G_COOKS_AROUND_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKS_AROUND_IT(v: Py) { ensure_init(); G_COOKS_AROUND_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_DECIDING() -> Py { ensure_init(); G_DECIDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECIDING(v: Py) { ensure_init(); G_DECIDING.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_FROM() -> Py { ensure_init(); G_FREE_FROM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_FROM(v: Py) { ensure_init(); G_FREE_FROM.with(|c| *c.borrow_mut() = v); }
pub fn g_FREE_FROM_FOODS() -> Py { ensure_init(); G_FREE_FROM_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREE_FROM_FOODS(v: Py) { ensure_init(); G_FREE_FROM_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_MINE() -> Py { ensure_init(); G_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINE(v: Py) { ensure_init(); G_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PUDDING() -> Py { ensure_init(); G_NOT_A_PUDDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PUDDING(v: Py) { ensure_init(); G_NOT_A_PUDDING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_AS_A_MEASURE() -> Py { ensure_init(); G_NOT_A_THING_AS_A_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_AS_A_MEASURE(v: Py) { ensure_init(); G_NOT_A_THING_AS_A_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALITY_LEAD() -> Py { ensure_init(); G_QUALITY_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALITY_LEAD(v: Py) { ensure_init(); G_QUALITY_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALITY_WORDS() -> Py { ensure_init(); G_QUALITY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALITY_WORDS(v: Py) { ensure_init(); G_QUALITY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_MOST_THINGS_A_VERDICT_TRIES() -> Py { ensure_init(); G_THE_MOST_THINGS_A_VERDICT_TRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_MOST_THINGS_A_VERDICT_TRIES(v: Py) { ensure_init(); G_THE_MOST_THINGS_A_VERDICT_TRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT() -> Py { ensure_init(); G_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT(v: Py) { ensure_init(); G_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_FILLER() -> Py { ensure_init(); G_VERDICT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_FILLER(v: Py) { ensure_init(); G_VERDICT_FILLER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A condition added to a choice is answered about the choice, not about the condition.\n\nThe person has just been given something to think about - a pet, a car, a move, a course - and\nanswers with the fact from their own life that decides it: \"but i work long hours\", \"and i have a\nsmall flat\". The line names no thing of its own, so the knowledge entries for the choice cannot see\nit, and whatever the condition reaches on its own wins the turn: \"but i work long hours\" after a\nquestion about getting a cat was answered with the card about working hours and the rota, which is\na different conversation, while the pet entries hold exactly the answer wanted (\"An adult cat is\nfine with a working day...\").\n\nThis module carries the thing in hand into the line and asks the knowledge base the question the\nperson would have made of it. It answers only when\n\n- the line opens like an addition (\"but\", \"and\", \"though\", \"only\", \"the thing is\") and asks nothing,\n- the store has nothing of its own for it, and\n- the answer that comes back names the thing in hand, so an entry about something else cannot win.\n\nSo a condition that has an answer of its own keeps it, and a line that opens a new subject never\nreaches this at all.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.CONDITION"); }
    s_CONDITION(stdlib_call("re", "compile", &[py_str("^(?:but|and|though|although|however|only|except|also|plus|mind you|the thing is|thing is|the problem is|problem is|the catch is|my worry is)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.MINE"); }
    s_MINE(stdlib_call("re", "compile", &[py_str("^(?:im|i'm|i am|i have|i've|ive|i only|i just|i usually|i always|i never|i work|i live|we have|we've|we live|we work|my \\w+ is)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.A_PLAN"); }
    s_A_PLAN(stdlib_call("re", "compile", &[py_str("^(?:going to|gonna|about to|planning to|planning on|off to)\\s+(?!be\\b|have\\b|need\\b|want\\b)\\w")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.A_JOB"); }
    s_A_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:need|needs|have|has|got|want|wants|ought|must)\\s+to\\s+(?:fix|mend|repair|replace|service|sort|clean|build|install|fit|change|bleed|adjust|tune|patch|pump|oil|grease)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.DECIDING"); }
    s_DECIDING(stdlib_call("re", "compile", &[py_str("\\b(?:cook|cooking|bake|baking|make|making|eat|eating|dinner|tea|supper|lunch)\\b|\\b(?:thinking (?:of|about)|should i|shall i|do i want|considering|im after|i'?m after|im going to|i'?m going to|im about to|i'?m about to|im planning|planning to|going to get|i want to get|what should i get|which one|worth it|do i need)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.A_STATE"); }
    s_A_STATE(stdlib_call("re", "compile", &[py_str("^(?:im|i'm|i am|ive|i've)\\s+(?:been|felt|feeling|still|not|so|really|like\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.A_DATE"); }
    s_A_DATE(stdlib_call("re", "compile", &[py_str("\\b(?:january|february|march|april|may|june|july|august|september|october|november|december|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.A_FIGURE"); }
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\d|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand|half|a few|a couple)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.ASKS"); }
    s_ASKS(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:can|could|should|would|will|do|does|did|is|are|was|were|what|how|why|when|where|which|who)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("bit"), py_str("few"), py_str("good"), py_str("great"), py_str("little"), py_str("load"), py_str("lot"), py_str("many"), py_str("moment"), py_str("quick"), py_str("right"), py_str("same"), py_str("sort"), py_str("thing"), py_str("time"), py_str("while"), py_str("whole")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.NOT_A_THING"); }
    s_NOT_A_THING(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.A_NAMED_THING"); }
    s_A_NAMED_THING(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|the|my|our|your|his|her|their|another|no|two|three)\\s+([a-z][a-z'\\-]{2,})")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.THE_MOST_THINGS_A_VERDICT_TRIES"); }
    s_THE_MOST_THINGS_A_VERDICT_TRIES(Py::Int(2));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.VERDICT"); }
    s_VERDICT(stdlib_call("re", "compile", &[mcall(&py_str("\\b(?:fair|ok|okay|alright|right|fine|enough|acceptable|reasonable|sensible|safe|legal|\\n    allowed|normal|typical|usual|weird|rude|polite|necessary|worth|expensive|cheap|\\n    bad|good|wise|risky|dangerous|healthy|a problem|an issue|a mistake|a good idea|\\n    a bad idea|too much|too many|too long|too short|too big|too small|too early|too late)\\b"), "replace", &[py_str("\n"), py_str("")]), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.VERDICT_FILLER"); }
    s_VERDICT_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an the that this it they these those there is are was were be\nbeen being would will shall do does did should can could i we you me us my our your his her their\ntoo so really very quite all then still just much many long short big small early late often\nnow good bad ok okay fine right thing one to of in on for with get getting have has had if what\nhow more less time way idea problem issue mistake deal worth going go at as by from up out and\nthe"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.ASKS_A_VERDICT"); }
    s_ASKS_A_VERDICT(stdlib_call("re", "compile", &[py_str("^(?:is|are|was|were|would|will|shall|do|does|did|should|can|could|have|has)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("day"), py_str("days"), py_str("week"), py_str("weeks"), py_str("month"), py_str("months"), py_str("year"), py_str("years"), py_str("hour"), py_str("hours"), py_str("minute"), py_str("minutes"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("night"), py_str("nights"), py_str("weekend"), py_str("time"), py_str("times"), py_str("while"), py_str("moment"), py_str("second"), py_str("seconds"), py_str("chance"), py_str("lot"), py_str("lotta")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.NOT_A_THING_AS_A_MEASURE"); }
    s_NOT_A_THING_AS_A_MEASURE(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.QUALITY_LEAD"); }
    s_QUALITY_LEAD(stdlib_call("re", "compile", &[py_str("^(?:(?:how|hows|what)\\s+about\\s+(?:a|an\\s+)?|how\\s+about\\s+|whats\\s+|what\\s+is\\s+|and\\s+|so\\s+|ok(?:ay)?\\s*,?\\s*)*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.QUALITY_WORDS"); }
    s_QUALITY_WORDS(call_builtin("frozenset", &[mcall(&py_str("\ncold warm hot chilled filling fillin light lighter sweet savoury savory salty spicy mild creamy\ncheesy quick quickish easy simple healthy fresher fresh veggie vegetarian vegan meaty\nfishy hearty substantial summery wintery\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.A_PUDDING"); }
    s_A_PUDDING(stdlib_call("re", "compile", &[py_str("\\b(?:pudding|puddings|pud|dessert|desserts|afters|sweet\\s+course)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.A_MEAL"); }
    s_A_MEAL(stdlib_call("re", "compile", &[py_str("\\b(?:dinner|supper|lunch|brunch|meal|meals|cook|cooking|cooked|bake|baking|baked|recipe|dish|dishes|food|eat|eating|roast|hob|oven|fryer)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.NOT_A_PUDDING"); }
    s_NOT_A_PUDDING(call_builtin("frozenset", &[mcall(&py_str("spicy savoury salty meaty fishy hearty filling substantial "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("almond"), py_str("almonds"), py_str("cashew"), py_str("cashews"), py_str("walnut"), py_str("walnuts"), py_str("pecan"), py_str("pecans"), py_str("hazelnut"), py_str("hazelnuts"), py_str("pistachio"), py_str("pistachios"), py_str("peanut"), py_str("peanuts"), py_str("praline"), py_str("marzipan")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("almond"), py_str("almonds"), py_str("cashew"), py_str("cashews"), py_str("walnut"), py_str("walnuts"), py_str("pecan"), py_str("pecans"), py_str("hazelnut"), py_str("hazelnuts"), py_str("pistachio"), py_str("pistachios"), py_str("peanut"), py_str("peanuts")]) }
    fn __data3() -> Py { py_dict(vec![(py_str("dairy"), py_tuple(vec![py_str("milk"), py_str("cheese"), py_str("butter"), py_str("cream"), py_str("yogurt"), py_str("yoghurt"), py_str("custard"), py_str("ghee"), py_str("fromage"), py_str("kefir"), py_str("buttermilk")])), (py_str("gluten"), py_tuple(vec![py_str("bread"), py_str("pasta"), py_str("flour"), py_str("couscous"), py_str("barley"), py_str("rye"), py_str("wheat"), py_str("noodle"), py_str("noodles"), py_str("cracker"), py_str("pastry")])), (py_str("wheat"), py_tuple(vec![py_str("bread"), py_str("pasta"), py_str("flour"), py_str("couscous"), py_str("barley"), py_str("noodle"), py_str("noodles"), py_str("pastry")])), (py_str("nuts"), __data4()), (py_str("nut"), __data5()), (py_str("peanuts"), py_tuple(vec![py_str("peanut"), py_str("peanuts"), py_str("groundnut"), py_str("satay")])), (py_str("peanut"), py_tuple(vec![py_str("peanut"), py_str("peanuts"), py_str("groundnut"), py_str("satay")])), (py_str("eggs"), py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("mayonnaise"), py_str("meringue"), py_str("custard")])), (py_str("egg"), py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("mayonnaise"), py_str("meringue"), py_str("custard")])), (py_str("meat"), py_tuple(vec![py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("bacon"), py_str("ham"), py_str("sausage"), py_str("sausages"), py_str("mince"), py_str("turkey"), py_str("chorizo"), py_str("steak")])), (py_str("pork"), py_tuple(vec![py_str("bacon"), py_str("ham"), py_str("sausage"), py_str("sausages"), py_str("chorizo"), py_str("gammon"), py_str("prosciutto")])), (py_str("beef"), py_tuple(vec![py_str("steak"), py_str("mince"), py_str("burger"), py_str("burgers"), py_str("brisket")])), (py_str("fish"), py_tuple(vec![py_str("salmon"), py_str("tuna"), py_str("cod"), py_str("haddock"), py_str("prawn"), py_str("prawns"), py_str("anchovy"), py_str("anchovies"), py_str("sardine"), py_str("sardines"), py_str("mackerel"), py_str("trout")])), (py_str("shellfish"), py_tuple(vec![py_str("prawn"), py_str("prawns"), py_str("crab"), py_str("mussel"), py_str("mussels"), py_str("lobster"), py_str("scallop"), py_str("scallops"), py_str("prawn"), py_str("squid")])), (py_str("soya"), py_tuple(vec![py_str("soy"), py_str("soya"), py_str("tofu"), py_str("edamame"), py_str("tempeh")])), (py_str("soy"), py_tuple(vec![py_str("soy"), py_str("soya"), py_str("tofu"), py_str("edamame"), py_str("tempeh")])), (py_str("sugar"), py_tuple(vec![py_str("honey"), py_str("syrup"), py_str("jam"), py_str("chocolate"), py_str("biscuit"), py_str("cake"), py_str("sweets")])), (py_str("salt"), py_tuple(vec![py_str("salted"), py_str("crisps"), py_str("bacon"), py_str("soy"), py_str("soya"), py_str("stock cube")]))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.FREE_FROM_FOODS"); }
    s_FREE_FROM_FOODS(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.FREE_FROM"); }
    s_FREE_FROM(call_builtin("frozenset", &[mcall(&py_str("\ndairy gluten wheat nuts nut peanuts peanut eggs egg milk cheese butter cream meat pork beef fish\nshellfish soya soy sugar salt spice spices chilli alcohol\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.CARRY_STOP"); }
    s_CARRY_STOP(call_builtin("frozenset", &[mcall(&py_str("\nsomething anything cook cooking make making tonight dinner lunch tea supper idea ideas meal meals\nhave want need what can could should recipes recipe please\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.carry.COOKS_AROUND_IT"); }
    s_COOKS_AROUND_IT(stdlib_call("re", "compile", &[py_str("\\b(?:in place of|instead of|swap(?:ped)?|replace|leave (?:it|that|them) out|free[- ]from|free of|\\w+[- ]free\\b|without\\b|use more|go easy on)\\b")]).unwrap_or_else(|e| exc::raise(e)));
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
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    Py::None
}
pub fn _the_thing_in_hand(args: &[Py]) -> Py {
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
    let mut other: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut topics_seen: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The subject of the chat before this line: the newest topic that is not this line itself.");
    said = mcall(&mcall(&_flat(&[text.clone()]), "lower", &[]), "rstrip", &[py_str(".")]);
    seen = py_list(vec![]);
    record = mcall(&memory.clone(), "get", &[py_str("topic")]);
    if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&seen.clone(), "append", &[record.clone()]);
    }
    topics_seen = mcall(&memory.clone(), "get", &[py_str("topics")]);
    if py_truthy(&call_builtin("isinstance", &[topics_seen.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&seen.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[topics_seen.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut other: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[other.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(other.clone()); } } py_list(__out) })() }]);
    }
    let __iter1 = py_iter_or_raise(&seen.clone());
    for __item in __iter1 {
    other = __item.clone();
    words = _flat(&[mcall(&other.clone(), "get", &[py_str("text")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&words.clone(), "lower", &[]), "rstrip", &[py_str(".")]), &said.clone())) } }) {
    continue;
    }
    thing = pyrt::xrefs::xcall("memory.repair", "_the_thing_the_last_line_named", &[words.clone()]);
    if py_truthy(&thing.clone()) {
    return py_tuple(vec![words.clone(), thing.clone()]);
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn _a_fresh_memory(args: &[Py]) -> Py {
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
    let mut fresh: Py = Py::None;
    let mut key: Py = Py::None;
    let mut understand: Py = Py::None;
    let _ = py_str("The facts the person stated, without the chat: the ask is read as a line of its own.\n\n    `aside` says so to the modules that read the store's own bookkeeping: the chat is not here, so\n    nothing in it was just said, and a name in the facts is one the person gave turns ago\n    (`bot/compose/__init__.py: _name_just_given`).\n    ");
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    fresh = py_dict(vec![(py_str("turns"), py_list(vec![])), (py_str("aside"), Py::Bool(true))]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("user"), py_str("others"), py_str("facts_log")]));
    for __item in __iter1 {
    key = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&memory.clone(), &key.clone()))) {
    let _ = py_setitem(&fresh.clone(), &key.clone(), geti(&memory.clone(), &key.clone()));
    }
    }
    return fresh.clone();
    Py::None
}
pub fn _the_answer_without_the_condition(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut subject_text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject_text"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answers: Py = Py::None;
    let _ = py_str("What the knowledge base says to this line as it was said, on its own.");
    answers = pyrt::xrefs::xmodule("modules.answers");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), { let __b = mcall(&answers.clone(), "respond", &[subject_text.clone(), _a_fresh_memory(&[memory.clone()])]); if py_truthy(&__b) { __b } else { py_str("") } }]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _named_in(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut subject: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Does the answer talk about the thing in hand? A plural counts as its own word.");
    said = mcall(&_flat(&[reply.clone()]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[subject.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    forms = { let __b = pyrt::xrefs::xcall("memory.topics", "forms", &[word.clone()]); if py_truthy(&__b) { __b } else { py_set(vec![word.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &thing.clone())); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.topics", "alike", &[word.clone(), thing.clone()]) } }) {
    forms = op_bitor(&op_bitor(&call_builtin("set", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "forms", &[thing.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), &py_set(vec![thing.clone()]));
    }
    let __iter2 = py_iter_or_raise(&forms.clone());
    for __item in __iter2 {
    form = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[form.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _already_on_the_screen(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut opening: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Is this the answer the person has just read? Then the condition added nothing to it.\n\n    \"but we go away a lot\" after \"should i get a cat or a dog\" reaches the same comparison the\n    person has just been given, word for word, and a second copy of it framed as \"same answer\"\n    is worse than the plain acknowledgement the composer would have said. The check is on the\n    opening of the answer, which every entry writes its own way, and it searches the replies\n    rather than matching them exactly, because a repeat is framed with a lead-in in front of the\n    same body.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    said = mcall(&_flat(&[reply.clone()]), "lower", &[]);
    opening = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&said.clone(), "split", &[]), &Py::None, &Py::Int(10), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&_flat(&[mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")])]), "lower", &[]), &opening.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_verdict_on_the_other_thing(args: &[Py]) -> Py {
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
    let mut condition: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "condition") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: condition"))) };
    let mut thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut subject: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut held: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the answer weighs the thing in hand against a thing the condition named.\n\n    \"im thinking of getting a rescue dog\" / \"but i already have a cat\" is a question about a dog\n    in a house with a cat. The condition names a thing of its own - a cat - and the answer that\n    came back is the cat-or-dog verdict: \"Dogs, by a whisker... A cat is the kinder choice if\n    you're out all day.\" That is an answer to a different question, so this module stands down\n    and lets the chat say what it was going to say. A condition that names nothing (\"but i work\n    long hours\", \"but its busy in term time\") is not this, and keeps its carried answer.\n    ");
    said = mcall(&_flat(&[reply.clone()]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = pyrt::xrefs::xcall("memory.topics", "keywords", &[subject.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&mcall(&g_A_NAMED_THING(), "findall", &[mcall(&_flat(&[condition.clone()]), "lower", &[])]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut one: Py = __item.clone(); __out.push(pyrt::xrefs::xcall("memory.topics", "alike", &[word.clone(), one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_decision_in_the_chat(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when what the chat is about is something the person is weighing up.");
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("user")]) } }) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    topic = mcall(&memory.clone(), "get", &[py_str("topic")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&topic.clone(), "get", &[py_str("text")]) } }) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[geti(&topic.clone(), &py_str("text"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&g_DECIDING(), "search", &[one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_verdict_asked(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the whole line is a verdict asked about the chat, not a thing of its own.");
    low = mcall(&mcall(&clean.clone(), "lower", &[]), "strip", &[py_str(" .!")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_VERDICT(), "search", &[low.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&clean.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_A_VERDICT(), "match", &[low.clone()]))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), low.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VERDICT_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&g_VERDICT(), "search", &[word.clone()]) } }) {
    continue;
    }
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn the_verdict_in_the_chat(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let mut since: Py = Py::None;
    let mut subject_text: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The verdict on the thing in hand, asked with what the person has since said, or None.\n\n    Only the facts they have added since the thing was named go in front of it (\"we both work\n    full time\"), because those are what the verdict is about: a chat about a dog and then a fact\n    about working hours is a question about a dog left alone all day. The ask is read as a line\n    of its own, and the answer has to name the thing in hand, so an entry that scored on the\n    words of the fact alone cannot win.\n    ");
    line = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_verdict_asked(&[line.clone()]))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&_the_things_the_verdict_is_about(&[memory.clone(), line.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    subject_text = __unpk2[0].clone();
    thing = __unpk2[1].clone();
    since = __unpk2[2].clone();
    reply = _the_verdict_answer(&[memory.clone(), line.clone(), subject_text.clone(), thing.clone(), since.clone()]);
    if py_truthy(&reply.clone()) {
    return reply.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_verdict_answer(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut subject_text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject_text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject_text"))) };
    let mut thing: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut since: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "since") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: since"))) };
    let mut _again: Py = Py::None;
    let mut answers: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut framed: Py = Py::None;
    let mut one: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The answer to a verdict about one thing the chat named, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&subject_text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&thing.clone())) } }) {
    return Py::None;
    }
    answers = pyrt::xrefs::xmodule("modules.answers");
    asked = mcall(&py_str(", "), "join", &[op_add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&since.clone()) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() }, &py_list(vec![subject_text.clone(), line.clone()]))]);
    let __try1 = exc::catch(|| {
    reply = mcall(&answers.clone(), "respond", &[asked.clone(), _a_fresh_memory(&[memory.clone()])]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_named_in(&[reply.clone(), thing.clone(), subject_text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_already_on_the_screen(&[reply.clone(), memory.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&_the_answer_just_given(&[reply.clone(), memory.clone()])))) {
    let __try6 = exc::catch(|| {
    _again = pyrt::xrefs::xmodule("memory.again");
    framed = mcall(&_again.clone(), "already_had", &[reply.clone(), memory.clone()]);
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
    framed = Py::None;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    if py_truthy(&framed.clone()) {
    return framed.clone();
    }
    }
    return Py::None;
    }
    return mcall(&reply.clone(), "strip", &[]);
    Py::None
}
pub fn _the_answer_just_given(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when this reply is the last answer word for word, or the opening of it.");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::Bool(false);
    }
    mine = mcall(&_flat(&[reply.clone()]), "lower", &[]);
    last = mcall(&_flat(&[mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")])]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mine.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&last.clone())) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_eq(&mine.clone(), &last.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&last.clone(), "startswith", &[mine.clone()]); if py_truthy(&__b) { __b } else { mcall(&mine.clone(), "startswith", &[mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&last.clone(), "split", &[]), &Py::None, &Py::Int(10), &Py::None)])]) } } } };
    Py::None
}
pub fn _the_things_the_verdict_is_about(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut order: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The things the chat named that the verdict could be about, newest first.\n\n    The thing is the last one the chat named with a word in front of it - \"a dog\", \"the kitchen\" -\n    and a line that names no thing (\"we both work full time\") is what the verdict is about rather\n    than the thing it is about, so it is carried along in front of it and never becomes one.\n\n    More than one is offered because the newest thing named can be the person's own plan for the\n    chat's subject - \"my boiler keeps losing pressure\" / \"im going to bleed the radiators\" /\n    \"is that dangerous\" - and no entry answers a verdict about a plan: the question is still about\n    the boiler, so the thing named before it is tried in its place.\n    ");
    said = mcall(&mcall(&_flat(&[line.clone()]), "lower", &[]), "rstrip", &[py_str(".")]);
    order = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&_the_lines_of_the_chat(&[memory.clone()]));
    for __item in __iter1 {
    other = __item.clone();
    words = _flat(&[other.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&words.clone(), "lower", &[]), "rstrip", &[py_str(".")]), &said.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = order.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&geti(&order.clone(), &op_neg(&Py::Int(1))), "lower", &[]), "rstrip", &[py_str(".")]), &mcall(&mcall(&words.clone(), "lower", &[]), "rstrip", &[py_str(".")]))) } }) {
    continue;
    }
    let _ = mcall(&order.clone(), "append", &[words.clone()]);
    }
    found = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[order.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    index = __item.clone();
    one = geti(&order.clone(), &index.clone());
    thing = _the_thing_they_named(&[one.clone()]);
    if py_truthy(&thing.clone()) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![one.clone(), thing.clone(), slice_or_raise(&order.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)])]);
    if py_truthy(&op_ge(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_THE_MOST_THINGS_A_VERDICT_TRIES())) {
    break;
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_thing_they_named(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the line names with a word in front of it: \"a dog\", \"my dog\", \"the kitchen\".\n\n    A thing comes with a determiner, and that is what tells the thing a verdict is about from a\n    word that is only part of a figure or a way they live: \"i take 1000 units of vitamin d\" names\n    no thing this way, and the question about it is the store's, not a verdict on a vitamin.\n    ");
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_NAMED_THING(), "findall", &[mcall(&_flat(&[line.clone()]), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NOT_A_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_THING_AS_A_MEASURE(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return if py_truthy(&found.clone()) { geti(&found.clone(), &op_neg(&Py::Int(1))) } else { Py::None };
    Py::None
}
pub fn _the_lines_of_the_chat(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut one: Py = Py::None;
    let mut order: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Every line the person said, oldest first.\n\n    The history is the whole chat, including the lines nothing kept a topic for: \"we go away a\n    lot\" after a question about a cat is answered by the greeting pool, so the topic record never\n    moved, and reading the topics alone lost the fact the verdict is about.\n    ");
    order = py_list(vec![]);
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&order.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")])); } } py_list(__out) })() }]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&order.clone()) { let mut line: Py = __item.clone(); __out.push(_flat(&[line.clone()])); } py_list(__out) })() }) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn the_subject_in_the_condition(args: &[Py]) -> Py {
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
    let mut answers: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut condition: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut job_thing: Py = Py::None;
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut own: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut subject_text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The answer to the choice with the person's own condition in it, or None.");
    line = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&g_ASKS(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    found = mcall(&g_CONDITION(), "match", &[mcall(&line.clone(), "lower", &[])]);
    mine = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    mine = mcall(&g_MINE(), "match", &[mcall(&line.clone(), "lower", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mine.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_FIGURE(), "search", &[mcall(&line.clone(), "lower", &[])]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_STATE(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DATE(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_PLAN(), "match", &[mcall(&slice_or_raise(&mcall(&line.clone(), "lower", &[]), &mcall(&mine.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[])])) {
    return Py::None;
    }
    found = mine.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_decision_in_the_chat(&[memory.clone()])))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_thing_in_hand(&[text.clone(), memory.clone()]), 2, None);
    subject_text = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    answers = pyrt::xrefs::xmodule("modules.answers");
    condition = mcall(&slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,.?!")]);
    if py_truthy(&Py::Bool(!py_truthy(&condition.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_JOB(), "search", &[condition.clone()])) {
    job_thing = pyrt::xrefs::xcall("memory.repair", "_the_thing_the_last_line_named", &[condition.clone()]);
    if py_truthy(&{ let __b = job_thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&job_thing.clone(), &_flat(&[thing.clone()]))) } }) {
    return Py::None;
    }
    }
    ask = op_modulo(&py_str("%s, %s"), &py_tuple(vec![condition.clone(), subject_text.clone()]));
    let __try2 = exc::catch(|| {
    reply = mcall(&answers.clone(), "respond", &[ask.clone(), _a_fresh_memory(&[memory.clone()])]);
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
    return Py::None;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    held = call_builtin("list", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "keywords", &[condition.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    own = _the_answer_without_the_condition(&[line.clone(), memory.clone()]);
    if py_truthy(&{ let __b = own.clone(); if !py_truthy(&__b) { __b } else { { let __b = held.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&held.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), mcall(&own.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_named_in(&[reply.clone(), thing.clone(), subject_text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_a_verdict_on_the_other_thing(&[reply.clone(), condition.clone(), thing.clone(), subject_text.clone()])) {
    return Py::None;
    }
    if py_truthy(&_already_on_the_screen(&[reply.clone(), memory.clone()])) {
    return Py::None;
    }
    return mcall(&reply.clone(), "strip", &[]);
    Py::None
}
pub fn _the_quality(args: &[Py]) -> Py {
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
    let mut lead: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The quality the line asks for, as \"something cold\", or None.");
    said = mcall(&mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), line.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" ,.!?")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    lead = mcall(&g_QUALITY_LEAD(), "match", &[said.clone()]);
    if py_truthy(&lead.clone()) {
    said = mcall(&slice_or_raise(&said.clone(), &mcall(&lead.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,.!?")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "startswith", &[py_tuple(vec![py_str("something "), py_str("anything "), py_str("summat ")])])))) {
    return Py::None;
    }
    words = mcall(&said.clone(), "split", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_QUALITY_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return op_add(&py_str("something "), &mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]));
    Py::None
}
pub fn _the_course(args: &[Py]) -> Py {
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
    let mut quality: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "quality") { v.clone() }
    else { py_str("") };
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("\"pudding\" when the chat is about the sweet, \"dinner\" when it is about the meal, else None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    record = mcall(&memory.clone(), "get", &[py_str("topic")]);
    if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&lines.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&lines.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "strip", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[lines.clone()]);
    words = call_builtin("set", &[mcall(&call_builtin("str", &[{ let __b = quality.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_A_PUDDING(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_NOT_A_PUDDING()))) } }) {
    return py_str("pudding");
    }
    if py_truthy(&mcall(&g_A_MEAL(), "search", &[said.clone()])) {
    return py_str("dinner");
    }
    return Py::None;
    Py::None
}
pub fn _is_a_hedge(args: &[Py]) -> Py {
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
    let mut fallback: Py = Py::None;
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[mcall(&fallback.clone(), "is_a_hedge", &[reply.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn the_quality_in_the_chat(args: &[Py]) -> Py {
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
    let mut answers: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut course: Py = Py::None;
    let mut direct: Py = Py::None;
    let mut line: Py = Py::None;
    let mut probe: Py = Py::None;
    let mut quality: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The dish that answers the quality the person asked for, or None.");
    line = _flat(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    quality = _the_quality(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&quality.clone()))) {
    return Py::None;
    }
    course = _the_course(&[memory.clone(), quality.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&course.clone()))) {
    return Py::None;
    }
    answers = pyrt::xrefs::xmodule("modules.answers");
    probe = call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    direct = mcall(&answers.clone(), "respond", &[line.clone(), probe.clone()]);
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
    direct = Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = direct.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_a_hedge(&[direct.clone()]))) } }) {
    return Py::None;
    }
    ask = op_modulo(&py_str("%s for %s"), &py_tuple(vec![quality.clone(), course.clone()]));
    let __try6 = exc::catch(|| {
    reply = mcall(&answers.clone(), "respond", &[ask.clone(), _a_fresh_memory(&[memory.clone()])]);
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
    return Py::None;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&_is_a_hedge(&[reply.clone()])) {
    return Py::None;
    }
    if py_truthy(&_already_on_the_screen(&[reply.clone(), memory.clone()])) {
    return Py::None;
    }
    return mcall(&reply.clone(), "strip", &[]);
    Py::None
}
pub fn _the_last_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lines: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut line: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The last line of the person's own that asked for something, near enough to still be on.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    words = mcall(&_flat(&[line.clone()]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&words.clone(), "lower", &[]), "rstrip", &[py_str(".")]), &said.clone())) } }) {
    continue;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&words.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14))) {
    return Py::None;
    }
    return words.clone();
    }
    return Py::None;
    Py::None
}
pub fn _speaks_of(args: &[Py]) -> Py {
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
    let mut things: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "things") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: things"))) };
    let mut food: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("True when the answer carries the thing that was ruled out, by its name or as a food.");
    said = mcall(&_flat(&[reply.clone()]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&things.clone());
    for __item in __iter1 {
    thing = __item.clone();
    forms = op_bitor(&py_set(vec![thing.clone()]), &call_builtin("set", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "forms", &[thing.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    let __iter2 = py_iter_or_raise(&mcall(&g_FREE_FROM_FOODS(), "get", &[mcall(&thing.clone(), "lower", &[]), py_tuple(vec![])]));
    for __item in __iter2 {
    food = __item.clone();
    let __aug3 = op_bitor(&forms.clone(), &op_bitor(&py_set(vec![food.clone()]), &call_builtin("set", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "forms", &[food.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))));
    forms = __aug3;
    }
    let __iter4 = py_iter_or_raise(&forms.clone());
    for __item in __iter4 {
    form = __item.clone();
    if py_truthy(&{ let __b = form.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[form.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_free_from(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut things: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "things") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: things"))) };
    let mut known: Py = Py::None;
    let mut word: Py = Py::None;
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&things.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FREE_FROM()) { let mut known: Py = __item.clone(); __out.push(pyrt::xrefs::xcall("memory.topics", "alike", &[word.clone(), known.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_subject_named(args: &[Py]) -> Py {
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
    let mut base: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "base") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: base"))) };
    let mut form: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Does the answer name the thing the ask was about? Then the ask was answered, not another.");
    said = mcall(&_flat(&[reply.clone()]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[base.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_CARRY_STOP(), &word.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&op_bitor(&py_set(vec![word.clone()]), &call_builtin("set", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "forms", &[word.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter2 {
    form = __item.clone();
    if py_truthy(&{ let __b = form.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[form.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_limit_in_the_chat(args: &[Py]) -> Py {
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
    let mut answers: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut asks: Py = Py::None;
    let mut base: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut read: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut things: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The dish or list the person still wants, with what they ruled out, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&_flat(&[text.clone()]), "strip", &[]);
    read = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&read.clone(), "get", &[py_str("constraint")]))) } }) {
    return Py::None;
    }
    things = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&read.clone(), "get", &[py_str("excludes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut thing: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&things.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_free_from(&[things.clone()]))) } } } }) {
    return Py::None;
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    base = _the_last_ask(&[lines.clone(), mcall(&said.clone(), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&base.clone()))) {
    return Py::None;
    }
    answers = pyrt::xrefs::xmodule("modules.answers");
    asks = py_list(vec![op_modulo(&py_str("%s, no %s"), &py_tuple(vec![base.clone(), mcall(&py_str(" or "), "join", &[things.clone()])]))]);
    subject = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[base.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_CARRY_STOP(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&subject.clone()) {
    let _ = mcall(&asks.clone(), "append", &[op_modulo(&py_str("what can i cook with %s, no %s"), &py_tuple(vec![mcall(&py_str(" "), "join", &[slice_or_raise(&subject.clone(), &Py::None, &Py::Int(2), &Py::None)]), mcall(&py_str(" or "), "join", &[things.clone()])]))]);
    }
    let __iter1 = py_iter_or_raise(&asks.clone());
    for __item in __iter1 {
    ask = __item.clone();
    let __try2 = exc::catch(|| {
    reply = mcall(&answers.clone(), "respond", &[ask.clone(), _a_fresh_memory(&[memory.clone()])]);
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = _is_a_hedge(&[reply.clone()]); if py_truthy(&__b) { __b } else { _already_on_the_screen(&[reply.clone(), memory.clone()]) } }) {
    continue;
    }
    if py_truthy(&_speaks_of(&[reply.clone(), things.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&_cooks_around_it(&[reply.clone()])))) {
    continue;
    }
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&_the_subject_named(&[reply.clone(), base.clone()])))) {
    continue;
    }
    }
    return mcall(&reply.clone(), "strip", &[]);
    }
    return Py::None;
    Py::None
}
pub fn _cooks_around_it(args: &[Py]) -> Py {
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
    let _ = py_str("True when the answer names the thing ruled out in order to say what goes in its place.");
    return call_builtin("bool", &[mcall(&g_COOKS_AROUND_IT(), "search", &[mcall(&_flat(&[reply.clone()]), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.carry".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKS" => Some(g_ASKS()),
            "ASKS_A_VERDICT" => Some(g_ASKS_A_VERDICT()),
            "A_DATE" => Some(g_A_DATE()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_JOB" => Some(g_A_JOB()),
            "A_MEAL" => Some(g_A_MEAL()),
            "A_NAMED_THING" => Some(g_A_NAMED_THING()),
            "A_PLAN" => Some(g_A_PLAN()),
            "A_PUDDING" => Some(g_A_PUDDING()),
            "A_STATE" => Some(g_A_STATE()),
            "CARRY_STOP" => Some(g_CARRY_STOP()),
            "CONDITION" => Some(g_CONDITION()),
            "COOKS_AROUND_IT" => Some(g_COOKS_AROUND_IT()),
            "DECIDING" => Some(g_DECIDING()),
            "FREE_FROM" => Some(g_FREE_FROM()),
            "FREE_FROM_FOODS" => Some(g_FREE_FROM_FOODS()),
            "MINE" => Some(g_MINE()),
            "NOT_A_PUDDING" => Some(g_NOT_A_PUDDING()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_A_THING_AS_A_MEASURE" => Some(g_NOT_A_THING_AS_A_MEASURE()),
            "QUALITY_LEAD" => Some(g_QUALITY_LEAD()),
            "QUALITY_WORDS" => Some(g_QUALITY_WORDS()),
            "THE_MOST_THINGS_A_VERDICT_TRIES" => Some(g_THE_MOST_THINGS_A_VERDICT_TRIES()),
            "VERDICT" => Some(g_VERDICT()),
            "VERDICT_FILLER" => Some(g_VERDICT_FILLER()),
            "_a_decision_in_the_chat" => Some(py_func("_a_decision_in_the_chat", |args: &[Py]| _a_decision_in_the_chat(args))),
            "_a_free_from" => Some(py_func("_a_free_from", |args: &[Py]| _a_free_from(args))),
            "_a_fresh_memory" => Some(py_func("_a_fresh_memory", |args: &[Py]| _a_fresh_memory(args))),
            "_a_verdict_asked" => Some(py_func("_a_verdict_asked", |args: &[Py]| _a_verdict_asked(args))),
            "_a_verdict_on_the_other_thing" => Some(py_func("_a_verdict_on_the_other_thing", |args: &[Py]| _a_verdict_on_the_other_thing(args))),
            "_already_on_the_screen" => Some(py_func("_already_on_the_screen", |args: &[Py]| _already_on_the_screen(args))),
            "_cooks_around_it" => Some(py_func("_cooks_around_it", |args: &[Py]| _cooks_around_it(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_is_a_hedge" => Some(py_func("_is_a_hedge", |args: &[Py]| _is_a_hedge(args))),
            "_named_in" => Some(py_func("_named_in", |args: &[Py]| _named_in(args))),
            "_speaks_of" => Some(py_func("_speaks_of", |args: &[Py]| _speaks_of(args))),
            "_the_answer_just_given" => Some(py_func("_the_answer_just_given", |args: &[Py]| _the_answer_just_given(args))),
            "_the_answer_without_the_condition" => Some(py_func("_the_answer_without_the_condition", |args: &[Py]| _the_answer_without_the_condition(args))),
            "_the_course" => Some(py_func("_the_course", |args: &[Py]| _the_course(args))),
            "_the_last_ask" => Some(py_func("_the_last_ask", |args: &[Py]| _the_last_ask(args))),
            "_the_lines_of_the_chat" => Some(py_func("_the_lines_of_the_chat", |args: &[Py]| _the_lines_of_the_chat(args))),
            "_the_quality" => Some(py_func("_the_quality", |args: &[Py]| _the_quality(args))),
            "_the_subject_named" => Some(py_func("_the_subject_named", |args: &[Py]| _the_subject_named(args))),
            "_the_thing_in_hand" => Some(py_func("_the_thing_in_hand", |args: &[Py]| _the_thing_in_hand(args))),
            "_the_thing_they_named" => Some(py_func("_the_thing_they_named", |args: &[Py]| _the_thing_they_named(args))),
            "_the_things_the_verdict_is_about" => Some(py_func("_the_things_the_verdict_is_about", |args: &[Py]| _the_things_the_verdict_is_about(args))),
            "_the_verdict_answer" => Some(py_func("_the_verdict_answer", |args: &[Py]| _the_verdict_answer(args))),
            "the_limit_in_the_chat" => Some(py_func("the_limit_in_the_chat", |args: &[Py]| the_limit_in_the_chat(args))),
            "the_quality_in_the_chat" => Some(py_func("the_quality_in_the_chat", |args: &[Py]| the_quality_in_the_chat(args))),
            "the_subject_in_the_condition" => Some(py_func("the_subject_in_the_condition", |args: &[Py]| the_subject_in_the_condition(args))),
            "the_verdict_in_the_chat" => Some(py_func("the_verdict_in_the_chat", |args: &[Py]| the_verdict_in_the_chat(args))),
            "a_repair" => Some(pyrt::xrefs::xmodule("memory.repair")),
            "topics" => Some(pyrt::xrefs::xmodule("memory.topics")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKS" => s_ASKS(v),
            "ASKS_A_VERDICT" => s_ASKS_A_VERDICT(v),
            "A_DATE" => s_A_DATE(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_JOB" => s_A_JOB(v),
            "A_MEAL" => s_A_MEAL(v),
            "A_NAMED_THING" => s_A_NAMED_THING(v),
            "A_PLAN" => s_A_PLAN(v),
            "A_PUDDING" => s_A_PUDDING(v),
            "A_STATE" => s_A_STATE(v),
            "CARRY_STOP" => s_CARRY_STOP(v),
            "CONDITION" => s_CONDITION(v),
            "COOKS_AROUND_IT" => s_COOKS_AROUND_IT(v),
            "DECIDING" => s_DECIDING(v),
            "FREE_FROM" => s_FREE_FROM(v),
            "FREE_FROM_FOODS" => s_FREE_FROM_FOODS(v),
            "MINE" => s_MINE(v),
            "NOT_A_PUDDING" => s_NOT_A_PUDDING(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_A_THING_AS_A_MEASURE" => s_NOT_A_THING_AS_A_MEASURE(v),
            "QUALITY_LEAD" => s_QUALITY_LEAD(v),
            "QUALITY_WORDS" => s_QUALITY_WORDS(v),
            "THE_MOST_THINGS_A_VERDICT_TRIES" => s_THE_MOST_THINGS_A_VERDICT_TRIES(v),
            "VERDICT" => s_VERDICT(v),
            "VERDICT_FILLER" => s_VERDICT_FILLER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.carry", module_obj);
    pyrt::xrefs::reg_fn("memory.carry", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_thing_in_hand", py_func("_the_thing_in_hand", |args: &[Py]| _the_thing_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_a_fresh_memory", py_func("_a_fresh_memory", |args: &[Py]| _a_fresh_memory(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_answer_without_the_condition", py_func("_the_answer_without_the_condition", |args: &[Py]| _the_answer_without_the_condition(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_named_in", py_func("_named_in", |args: &[Py]| _named_in(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_already_on_the_screen", py_func("_already_on_the_screen", |args: &[Py]| _already_on_the_screen(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_a_verdict_on_the_other_thing", py_func("_a_verdict_on_the_other_thing", |args: &[Py]| _a_verdict_on_the_other_thing(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_a_decision_in_the_chat", py_func("_a_decision_in_the_chat", |args: &[Py]| _a_decision_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_a_verdict_asked", py_func("_a_verdict_asked", |args: &[Py]| _a_verdict_asked(args)));
    pyrt::xrefs::reg_fn("memory.carry", "the_verdict_in_the_chat", py_func("the_verdict_in_the_chat", |args: &[Py]| the_verdict_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_verdict_answer", py_func("_the_verdict_answer", |args: &[Py]| _the_verdict_answer(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_answer_just_given", py_func("_the_answer_just_given", |args: &[Py]| _the_answer_just_given(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_things_the_verdict_is_about", py_func("_the_things_the_verdict_is_about", |args: &[Py]| _the_things_the_verdict_is_about(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_thing_they_named", py_func("_the_thing_they_named", |args: &[Py]| _the_thing_they_named(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_lines_of_the_chat", py_func("_the_lines_of_the_chat", |args: &[Py]| _the_lines_of_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.carry", "the_subject_in_the_condition", py_func("the_subject_in_the_condition", |args: &[Py]| the_subject_in_the_condition(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_quality", py_func("_the_quality", |args: &[Py]| _the_quality(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_course", py_func("_the_course", |args: &[Py]| _the_course(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_is_a_hedge", py_func("_is_a_hedge", |args: &[Py]| _is_a_hedge(args)));
    pyrt::xrefs::reg_fn("memory.carry", "the_quality_in_the_chat", py_func("the_quality_in_the_chat", |args: &[Py]| the_quality_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_last_ask", py_func("_the_last_ask", |args: &[Py]| _the_last_ask(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_speaks_of", py_func("_speaks_of", |args: &[Py]| _speaks_of(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_a_free_from", py_func("_a_free_from", |args: &[Py]| _a_free_from(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_the_subject_named", py_func("_the_subject_named", |args: &[Py]| _the_subject_named(args)));
    pyrt::xrefs::reg_fn("memory.carry", "the_limit_in_the_chat", py_func("the_limit_in_the_chat", |args: &[Py]| the_limit_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.carry", "_cooks_around_it", py_func("_cooks_around_it", |args: &[Py]| _cooks_around_it(args)));
    pyrt::xrefs::reg_access("memory.carry", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("memory.carry", "ASKS_A_VERDICT", g_ASKS_A_VERDICT, s_ASKS_A_VERDICT);
    pyrt::xrefs::reg_access("memory.carry", "A_DATE", g_A_DATE, s_A_DATE);
    pyrt::xrefs::reg_access("memory.carry", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("memory.carry", "A_JOB", g_A_JOB, s_A_JOB);
    pyrt::xrefs::reg_access("memory.carry", "A_MEAL", g_A_MEAL, s_A_MEAL);
    pyrt::xrefs::reg_access("memory.carry", "A_NAMED_THING", g_A_NAMED_THING, s_A_NAMED_THING);
    pyrt::xrefs::reg_access("memory.carry", "A_PLAN", g_A_PLAN, s_A_PLAN);
    pyrt::xrefs::reg_access("memory.carry", "A_PUDDING", g_A_PUDDING, s_A_PUDDING);
    pyrt::xrefs::reg_access("memory.carry", "A_STATE", g_A_STATE, s_A_STATE);
    pyrt::xrefs::reg_access("memory.carry", "CARRY_STOP", g_CARRY_STOP, s_CARRY_STOP);
    pyrt::xrefs::reg_access("memory.carry", "CONDITION", g_CONDITION, s_CONDITION);
    pyrt::xrefs::reg_access("memory.carry", "COOKS_AROUND_IT", g_COOKS_AROUND_IT, s_COOKS_AROUND_IT);
    pyrt::xrefs::reg_access("memory.carry", "DECIDING", g_DECIDING, s_DECIDING);
    pyrt::xrefs::reg_access("memory.carry", "FREE_FROM", g_FREE_FROM, s_FREE_FROM);
    pyrt::xrefs::reg_access("memory.carry", "FREE_FROM_FOODS", g_FREE_FROM_FOODS, s_FREE_FROM_FOODS);
    pyrt::xrefs::reg_access("memory.carry", "MINE", g_MINE, s_MINE);
    pyrt::xrefs::reg_access("memory.carry", "NOT_A_PUDDING", g_NOT_A_PUDDING, s_NOT_A_PUDDING);
    pyrt::xrefs::reg_access("memory.carry", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.carry", "NOT_A_THING_AS_A_MEASURE", g_NOT_A_THING_AS_A_MEASURE, s_NOT_A_THING_AS_A_MEASURE);
    pyrt::xrefs::reg_access("memory.carry", "QUALITY_LEAD", g_QUALITY_LEAD, s_QUALITY_LEAD);
    pyrt::xrefs::reg_access("memory.carry", "QUALITY_WORDS", g_QUALITY_WORDS, s_QUALITY_WORDS);
    pyrt::xrefs::reg_access("memory.carry", "THE_MOST_THINGS_A_VERDICT_TRIES", g_THE_MOST_THINGS_A_VERDICT_TRIES, s_THE_MOST_THINGS_A_VERDICT_TRIES);
    pyrt::xrefs::reg_access("memory.carry", "VERDICT", g_VERDICT, s_VERDICT);
    pyrt::xrefs::reg_access("memory.carry", "VERDICT_FILLER", g_VERDICT_FILLER, s_VERDICT_FILLER);
}
