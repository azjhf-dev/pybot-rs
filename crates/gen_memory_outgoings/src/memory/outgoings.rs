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


thread_local! { static G_AFTER_THE_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CODE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIRST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HAND_HELD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LEFT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STEP_ASK_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_YEAR_AND_A_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIXED_COST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HIGH_SHARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INCOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOW_SHARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_BUDGET_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_COST_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_COST: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.outgoings: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AFTER_THE_FIGURE() -> Py { ensure_init(); G_AFTER_THE_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_THE_FIGURE(v: Py) { ensure_init(); G_AFTER_THE_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_ASK() -> Py { ensure_init(); G_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_ASK(v: Py) { ensure_init(); G_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CODE() -> Py { ensure_init(); G_A_CODE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CODE(v: Py) { ensure_init(); G_A_CODE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIRST_ASK() -> Py { ensure_init(); G_A_FIRST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIRST_ASK(v: Py) { ensure_init(); G_A_FIRST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HAND_HELD() -> Py { ensure_init(); G_A_HAND_HELD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HAND_HELD(v: Py) { ensure_init(); G_A_HAND_HELD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LEFT_ASK() -> Py { ensure_init(); G_A_LEFT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LEFT_ASK(v: Py) { ensure_init(); G_A_LEFT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEASURE() -> Py { ensure_init(); G_A_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEASURE(v: Py) { ensure_init(); G_A_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHARE() -> Py { ensure_init(); G_A_SHARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHARE(v: Py) { ensure_init(); G_A_SHARE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STEP_ASK_POINTS_BACK() -> Py { ensure_init(); G_A_STEP_ASK_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STEP_ASK_POINTS_BACK(v: Py) { ensure_init(); G_A_STEP_ASK_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_YEAR_AND_A_MONTH() -> Py { ensure_init(); G_A_YEAR_AND_A_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_YEAR_AND_A_MONTH(v: Py) { ensure_init(); G_A_YEAR_AND_A_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_NUMBER() -> Py { ensure_init(); G_BARE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_NUMBER(v: Py) { ensure_init(); G_BARE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_MOVE() -> Py { ensure_init(); G_FIRST_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_MOVE(v: Py) { ensure_init(); G_FIRST_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIXED_COST() -> Py { ensure_init(); G_FIXED_COST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIXED_COST(v: Py) { ensure_init(); G_FIXED_COST.with(|c| *c.borrow_mut() = v); }
pub fn g_HIGH_SHARE() -> Py { ensure_init(); G_HIGH_SHARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HIGH_SHARE(v: Py) { ensure_init(); G_HIGH_SHARE.with(|c| *c.borrow_mut() = v); }
pub fn g_INCOME() -> Py { ensure_init(); G_INCOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INCOME(v: Py) { ensure_init(); G_INCOME.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT() -> Py { ensure_init(); G_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT(v: Py) { ensure_init(); G_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_AGAIN() -> Py { ensure_init(); G_LEFT_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_AGAIN(v: Py) { ensure_init(); G_LEFT_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_LOW_SHARE() -> Py { ensure_init(); G_LOW_SHARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOW_SHARE(v: Py) { ensure_init(); G_LOW_SHARE.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH() -> Py { ensure_init(); G_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH(v: Py) { ensure_init(); G_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_BUDGET_LINE() -> Py { ensure_init(); G_NOT_A_BUDGET_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_BUDGET_LINE(v: Py) { ensure_init(); G_NOT_A_BUDGET_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_COST_NAME() -> Py { ensure_init(); G_NOT_A_COST_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_COST_NAME(v: Py) { ensure_init(); G_NOT_A_COST_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_SUM() -> Py { ensure_init(); G_SUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUM(v: Py) { ensure_init(); G_SUM.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEK() -> Py { ensure_init(); G_WEEK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEK(v: Py) { ensure_init(); G_WEEK.with(|c| *c.borrow_mut() = v); }
pub fn g_YEAR() -> Py { ensure_init(); G_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YEAR(v: Py) { ensure_init(); G_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_COST() -> Py { ensure_init(); G__THE_COST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_COST(v: Py) { ensure_init(); G__THE_COST.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What comes in and what goes out: \"im on 1800 a month and rent is 700\".\n\nA person setting out their month says the two figures together, because together they are the one\nthing worth knowing: what is left. Nothing in the bot read the pair. Each figure found a card of\nits own, or a receipt:\n\n```\nim on 1800 a month and rent is 700        -> \"Tell the landlord in writing before the next rent\n                                              day, and ask the council about a Discretionary\n                                              Housing Payment...\"   (the arrears card)\ni earn 1800 and my rent is 700            -> \"Thanks for telling me - I'm here if you want to dig\n                                              into any of that.\"\ni take home 1800 and rent is 700          -> \"Coming into the UK you can bring 4 litres of\n                                              spirits...\"   (the duty-free entry, off \"take home\")\nwe bring in 2400 and the mortgage is 900  -> \"Got it. Is there anything you'd like a hand with?\"\nrent is 700                               -> the arrears card again, to somebody who is not behind\n```\n\nThe arrears card is right for \"i cant pay my rent\" and wrong for \"rent is 700\". The duty-free card\nis right for nothing here at all: the words \"take home\" found it. The two figures are the line, so\nthe two figures are what the reply is about.\n\nThree shapes, each answered with arithmetic on the person's own numbers:\n\n* what comes in and what goes out in one line - \"im on 1800 a month and rent is 700\" - and what is\n  left after the rent is the answer;\n* the cost on its own after the income was said a turn or two before (\"i earn about 1800 a month\" /\n  \"and rent is 900\"), which is the same sum with the income read off the chat;\n* the income on its own when the chat is already about putting money by (\"how much should i save\n  each month\" / \"i earn about 1800 a month\"), which is the share of *their* figure rather than the\n  share of the example figure the money entry uses.\n\nNothing is invented: every figure in the reply is one the person gave, or a share the money entry\nalready uses (\"put away 10 to 20 percent of your take-home\", \"a tenth to a fifth\"). What is not a\nbudget line is left alone - arrears, an eviction notice, a rent that is going up, a tax code, and\nevery ask (\"should i overpay\", \"how do i budget\") belong to the readings that answer them.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.SUM"); }
    s_SUM(stdlib_call("re", "compile", &[py_str("£\\s*(?P<marked>\\d[\\d,]*(?:\\.\\d+)?)\\s*(?P<marked_unit>k|grand|thousand)?|(?P<bare>\\d[\\d,]*(?:\\.\\d+)?)\\s*(?P<bare_unit>k|grand|thousand|pounds?|quid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.BARE_NUMBER"); }
    s_BARE_NUMBER(stdlib_call("re", "compile", &[py_str("\\d[\\d,]*(?:\\.\\d+)?")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.INCOME"); }
    s_INCOME(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)(?:'m|m|'re| are)?\\s+(?:on|earn|earns|make|makes|bring\\s+in|brings\\s+in|get|gets)\\b|\\btake[\\s-]?home\\b|\\b(?:my|our)\\s+(?:pay|wage|wages|salary|pension|income|earnings)\\b|\\bafter\\s+tax\\b|\\b(?:partner|husband|wife)\\s+(?:earns?|makes?|brings?\\s+in|gets?|takes?\\s+home)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.FIXED_COST"); }
    s_FIXED_COST(stdlib_call("re", "compile", &[py_str("\\b(?:rents?|mortgage|childcare|child\\s*care|childminder|nursery|nursery\\s+fees|school\\s+(?:fees|dinners|clubs?)|breakfast\\s+club|after[\\s-]school\\s+club|council\\s+tax|bills?|utility|utilities|energy|electric(?:ity)?|water|gas|heating|broadband|internet|phone|mobile|tv\\s+licen[cs]e|car\\s+finance|car\\s+payment|car\\s+insurance|loan|loans|repayments?|insurance|commut\\w*|travel|fares?|transport|petrol|diesel|fuel|parking|tolls?|train\\s+fare|season\\s+ticket|bus\\s+pass|subscriptions?|gym|clothes|uniforms?|nappies|food|groceries|shopping|car|van|pets?|vets?|vet\\s+bills?|medicines?|prescriptions?|dentist|child\\s+maintenance|maintenance)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.AFTER_THE_FIGURE"); }
    s_AFTER_THE_FIGURE(stdlib_call("re", "compile", &[py_str("^\\s*(?:a\\s+(?:month|week|year|fortnight)\\s*)?(?:goes?|went|is\\s+spent|are\\s+spent)?\\s*(?:on|for|towards|into)\\s+(?P<noun>[a-z][a-z'\\-]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.NOT_A_COST_NAME"); }
    s_NOT_A_COST_NAME(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|it|them|that|this|me|us|him|her|his|my|our|their|its|and|or|so|but|if|when|top|all|everything|nothing|stuff|things?)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.MONTH"); }
    s_MONTH(stdlib_call("re", "compile", &[py_str("\\b(?:a|per|each|every)\\s+month\\b|\\bmonthly\\b|\\bpcm\\b|\\b(?:a|per)\\s+4\\s+weeks\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.WEEK"); }
    s_WEEK(stdlib_call("re", "compile", &[py_str("\\b(?:a|per|each|every)\\s+week\\b|\\bweekly\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.YEAR"); }
    s_YEAR(stdlib_call("re", "compile", &[py_str("\\b(?:a|per|each|every)\\s+year\\b|\\byearly\\b|\\bannually\\b|\\ba\\s+year\\b|\\bp\\.?a\\.?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.NOT_A_BUDGET_LINE"); }
    s_NOT_A_BUDGET_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:arrears?|behind|overdue|evict\\w*|notice|can'?t\\s+pay|cant\\s+pay|couldn'?t\\s+pay|not\\s+paid|unpaid|short\\s+this\\s+month|owing|owe|owed|debt|debts|default\\w*|bailiff\\w*|due|going\\s+up|went\\s+up|go\\s+up|gone\\s+up|goes\\s+up|rising|increas\\w*|raise[sd]?|cheaper|reduce|reducing|lower|negotiate|landlord|housing\\s+benefit|universal\\s+credit)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.AN_ASK"); }
    s_AN_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:should|could|can|do|does|is|are|was|were|what|how|when|where|which|why|would|will)\\b|[,;]\\s*(?:should|could|can|do|does|is|are|was|were|what|how|when|where|which|why|would|will)\\s+\\w|\\?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.A_MEASURE"); }
    s_A_MEASURE(stdlib_call("re", "compile", &[py_str("^\\s*(?:mins?|minutes?|hours?|hrs?|days?|weeks?|months?|years?|miles?|km|kg|bed(?:room)?s?|kids?|people|guests?|percent|%)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.A_CODE"); }
    s_A_CODE(stdlib_call("re", "compile", &[py_str("\\b\\d{3,}[A-Za-z]{1,2}\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.A_HAND_HELD"); }
    s_A_HAND_HELD(stdlib_call("re", "compile", &[py_str("\\b(?:put|set|stick)\\s+(?:\\u00a3?\\d[\\d,]*(?:\\.\\d+)?\\s+)?(?:by|aside|away)\\b|\\b(?:save[sd]?|saving|savings|pot|putting\\s+by)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings._THE_COST"); }
    s__THE_COST(py_dict(vec![(py_str("rents"), py_str("rent")), (py_str("bills"), py_str("bills")), (py_str("loans"), py_str("loan payments")), (py_str("repayments"), py_str("repayment"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.LOW_SHARE"); }
    s_LOW_SHARE(Py::Float(0.1));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.HIGH_SHARE"); }
    s_HIGH_SHARE(Py::Float(0.2));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.LEFT"); }
    s_LEFT(py_tuple(vec![py_str("That leaves {left} after the {named}. Food, travel and the rest come out of that, and whatever is over at the end is what you can put by."), py_str("{left} left after the {named}, then. A standing order for the amount you want to keep, on the day after payday, takes it out before you can spend it."), py_str("So {in_amount} in and {out_amount} out on the {named} leaves {left}. Pick the figure you want to keep and move it the day you are paid.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.A_YEAR_AND_A_MONTH"); }
    s_A_YEAR_AND_A_MONTH(py_tuple(vec![py_str("{in_amount} a year is about {a_month} a month, so the {named} leaves {left}."), py_str("{in_amount} a year works out at {a_month} a month, and after the {named} there is {left}.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.FIRST_MOVE"); }
    s_FIRST_MOVE(py_tuple(vec![py_str("Set the standing order first: choose the figure you want to keep and move it the day after you are paid, so the month runs on what is left rather than the other way round."), py_str("Start with the standing order on payday for the figure you want to keep. Everything else in the month can move around it.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.A_FIRST_ASK"); }
    s_A_FIRST_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|where)\\s+(?:should|do|shall)\\s+i\\s+(?:do|start|begin)\\b[^.?!]*\\b(?:first|next|now|start)\\b|\\bhow\\s+do\\s+i\\s+(?:start|begin)\\b|\\bwhere\\s+(?:do|should)\\s+i\\s+(?:even\\s+)?(?:start|begin)\\b|\\bwhat'?s\\s+(?:the\\s+)?first\\s+(?:step|thing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.A_LEFT_ASK"); }
    s_A_LEFT_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat'?s\\s+left\\b|\\bwhat\\s+is\\s+left\\b|\\bwhat'?s\\s+left\\s+over\\b|\\bhow\\s+much\\s+(?:is\\s+left|left|do\\s+i\\s+have\\s+left|have\\s+i\\s+(?:got\\s+)?left)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.LEFT_AGAIN"); }
    s_LEFT_AGAIN(py_tuple(vec![py_str("{left} {unit}, on the figures you gave."), py_str("Same sum, on the figures you gave: {left} {unit}.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.A_STEP_ASK_POINTS_BACK"); }
    s_A_STEP_ASK_POINTS_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:that|this|it|them|those|these|now|next|first|then|month|months|money|save|saves|saving|savings|budget|budgeting|pay|paid|payday|salary|wage|income|rent|bills?|pot|aside|afford)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.outgoings.A_SHARE"); }
    s_A_SHARE(py_tuple(vec![py_str("A tenth to a fifth of {in_amount} is {low} to {high} {period}, which is the usual amount to put by."), py_str("Put by a tenth to a fifth of it: {low} to {high} {period}, out of {in_amount}.")]));
}
pub fn _choice(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut shape: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shape") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: shape"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut candidate: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut start: Py = Py::None;
    let mut step: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("A shape for this line, never the one just used.\n\n    `hash()` of a string is salted per run, so the index comes from the letters; and the ending the\n    bot used on the turn before is skipped, because two arithmetic replies in a row that close with\n    the same sentence read as one sentence said twice (\"the childcare is 400 a month on top\").\n    ");
    was = py_str("");
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    was = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    start = op_modulo(&call_builtin("sum", &[call_builtin("map", &[py_func("ord", |args: &[Py]| call_builtin("ord", args).unwrap_or_else(|e| exc::raise(e))), call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[shape.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[shape.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    candidate = geti(&shape.clone(), &op_modulo(&op_add(&start.clone(), &step.clone()), &call_builtin("len", &[shape.clone()]).unwrap_or_else(|e| exc::raise(e))));
    opener = mcall(&geti(&mcall(&candidate.clone(), "split", &[py_str("{")]), &Py::Int(0)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&opener.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&was.clone(), &slice_or_raise(&opener.clone(), &Py::None, &Py::Int(28), &Py::None))) } }) {
    return candidate.clone();
    }
    }
    return geti(&shape.clone(), &start.clone());
    Py::None
}
pub fn _the_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut costs: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "costs") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: costs"))) };
    let mut _value: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let _ = py_str("The costs as one phrase: \"rent\", \"rent and the childcare\", \"rent, the nursery and the loan\".");
    names = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&costs.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    _value = __unpk2[1].clone();
    name = mcall(&g__THE_COST(), "get", &[name.clone(), name.clone()]);
    if py_truthy(&Py::Bool(!py_contains(&names.clone(), &name.clone()))) {
    let _ = mcall(&names.clone(), "append", &[name.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&names.clone(), &Py::Int(0));
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    return op_modulo(&py_str("%s and the %s"), &py_tuple(vec![geti(&names.clone(), &Py::Int(0)), geti(&names.clone(), &Py::Int(1))]));
    }
    return op_modulo(&py_str("%s and the %s"), &py_tuple(vec![mcall(&py_str(", "), "join", &[slice_or_raise(&names.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), geti(&names.clone(), &op_neg(&Py::Int(1)))]));
    Py::None
}
pub fn _sums(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut start: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("(value, start, end) for every figure in the line that can be money.\n\n    Both passes run, not one after the other only if the first found nothing: \"i earn 1800 a month\n    and my rent is 700 pounds\" pairs \"700 pounds\" with its money word, and stopping there lost the\n    1800 and the whole line with it.\n    ");
    found = py_list(vec![]);
    taken = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_SUM(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    value = { let __b = mcall(&match_.clone(), "group", &[py_str("marked")]); if py_truthy(&__b) { __b } else { mcall(&match_.clone(), "group", &[py_str("bare")]) } };
    unit = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("marked_unit")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("bare_unit")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    let __try2 = exc::catch(|| {
    number = call_builtin("float", &[mcall(&value.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("k"), py_str("grand"), py_str("thousand")]), &unit.clone()))) {
    let __aug7 = op_mul(&number.clone(), &Py::Int(1000));
    number = __aug7;
    }
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &op_add(&mcall(&match_.clone(), "end", &[]), &Py::Int(1)), &Py::None), "isalpha", &[]); if py_truthy(&__b) { __b } else { mcall(&g_A_MEASURE(), "match", &[slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]) } }) {
    continue;
    }
    if py_truthy(&op_ge(&number.clone(), &Py::Int(20))) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![number.clone(), mcall(&match_.clone(), "start", &[]), mcall(&match_.clone(), "end", &[])])]);
    let _ = mcall(&taken.clone(), "append", &[py_tuple(vec![mcall(&match_.clone(), "start", &[]), mcall(&match_.clone(), "end", &[])])]);
    }
    }
    let __iter8 = py_iter_or_raise(&mcall(&g_BARE_NUMBER(), "finditer", &[text.clone()]));
    for __item in __iter8 {
    match_ = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&taken.clone()) { let __unpk3267 = py_unpack(&__item.clone(), 2, None); let mut start: Py = __unpk3267[0].clone(); let mut end: Py = __unpk3267[1].clone(); __out.push({ let __cmp11: Py = mcall(&match_.clone(), "start", &[]); if py_truthy(&op_le(&start.clone(), &__cmp11)) { op_lt(&__cmp11, &end.clone()) } else { Py::Bool(false) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &op_add(&mcall(&match_.clone(), "end", &[]), &Py::Int(1)), &Py::None), "isalpha", &[]); if py_truthy(&__b) { __b } else { mcall(&g_A_MEASURE(), "match", &[slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]) } }) {
    continue;
    }
    let __try9 = exc::catch(|| {
    number = call_builtin("float", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret10: Py = Py::Bool(false);
    let mut __val11: Py = Py::None;
    let mut __unhandled12: Option<exc::PyExc> = None;
    match __try9 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret10 = __p[0].clone(); __val11 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled13 = false;
            if !__handled13 && exc::matches_any(&__e, &["ValueError"]) {
                __handled13 = true;
    continue;
            }
            if !__handled13 { __unhandled12 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled12 { exc::raise(__e); }
    if py_truthy(&__ret10) { return __val11; }
    if py_truthy(&op_ge(&number.clone(), &Py::Int(20))) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![number.clone(), mcall(&match_.clone(), "start", &[]), mcall(&match_.clone(), "end", &[])])]);
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let end = end.clone(); let found = found.clone(); let match_ = match_.clone(); let number = number.clone(); let start = start.clone(); let taken = taken.clone(); let text = text.clone(); let unit = unit.clone(); let value = value.clone(); py_func("lambda", move |args: &[Py]| { let end = end.clone(); let found = found.clone(); let match_ = match_.clone(); let number = number.clone(); let start = start.clone(); let taken = taken.clone(); let text = text.clone(); let unit = unit.clone(); let value = value.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); geti(&item.clone(), &Py::Int(1)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "sort", args))(&__args) };
    return found.clone();
    Py::None
}
pub fn _period(args: &[Py]) -> Py {
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
    let _ = py_str("The unit a figure is said in: \"a month\" unless the line says a week or a year.");
    if py_truthy(&mcall(&g_WEEK(), "search", &[text.clone()])) {
    return py_str("a week");
    }
    if py_truthy(&mcall(&g_YEAR(), "search", &[text.clone()])) {
    return py_str("a year");
    }
    if py_truthy(&mcall(&g_MONTH(), "search", &[text.clone()])) {
    return py_str("a month");
    }
    return py_str("");
    Py::None
}
pub fn _units_in(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The units a line names, as a set: {\"a month\"}, or {\"a month\", \"a week\"} when it mixes two.");
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_MONTH(), py_str("a month")]), py_tuple(vec![g_WEEK(), py_str("a week")]), py_tuple(vec![g_YEAR(), py_str("a year")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    unit = __unpk2[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[text.clone()])) {
    let _ = mcall(&found.clone(), "add", &[unit.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _pounds(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let _ = py_str("£1,100 / £180.50 - the mark on, and the figure the way people write it.");
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.005))) {
    return op_modulo(&py_str("£%s"), &call_builtin("format", &[call_builtin("int", &[call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str(",")]).unwrap_or_else(|e| exc::raise(e)));
    }
    return op_modulo(&py_str("£%s"), &call_builtin("format", &[call_builtin("round", &[value.clone(), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e)), py_str(",.2f")]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _with_unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut figure: Py = Py::None;
    let _ = py_str("\"£1,100 a month\", or \"£1,100\" when the line never said which month it is.");
    figure = _pounds(&[value.clone()]);
    return if py_truthy(&unit.clone()) { op_modulo(&py_str("%s %s"), &py_tuple(vec![figure.clone(), unit.clone()])) } else { figure.clone() };
    Py::None
}
pub fn _in_phrase(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut pattern: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let mut span: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "span") { v.clone() }
    else { Py::Int(18) };
    let _ = py_str("Whether `pattern` runs around the figure, over it: \"put 200 aside\" has the 200 inside it.");
    return call_builtin("bool", &[mcall(&pattern.clone(), "search", &[slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&start.clone(), &span.clone())]).unwrap_or_else(|e| exc::raise(e)), &op_add(&end.clone(), &span.clone()), &Py::None)])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _near(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut pattern: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pattern") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pattern"))) };
    let mut before: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { Py::Int(30) };
    let mut after: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::Int(30) };
    let _ = py_str("Whether `pattern` sits within those characters of the figure, either side of it.");
    return { let __b = call_builtin("bool", &[mcall(&pattern.clone(), "search", &[slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&start.clone(), &before.clone())]).unwrap_or_else(|e| exc::raise(e)), &start.clone(), &Py::None)])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&pattern.clone(), "search", &[slice_or_raise(&text.clone(), &end.clone(), &op_add(&end.clone(), &after.clone()), &Py::None)])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_nearest(args: &[Py]) -> Py {
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
    let mut cue: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut sums: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sums") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sums"))) };
    let mut after: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::Int(20) };
    let mut before: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { Py::Int(6) };
    let mut best: Py = Py::None;
    let mut distance: Py = Py::None;
    let mut end: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut start: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The sum a word is talking about: the first one after it, else the last one before it.\n\n    \"i take home 1800 and rent is 700\" has two figures, and the one the income words are about is\n    the one they are next to - 1800. A window on both sides and no order to it took the rent, and\n    then the sum came out as the same figure twice: £1,300 left after the rent, out of £850.\n    ");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&sums.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    value = __unpk2[0].clone();
    start = __unpk2[1].clone();
    end = __unpk2[2].clone();
    gap = op_sub(&start.clone(), &mcall(&cue.clone(), "end", &[]));
    if py_truthy(&{ let __cmp31: Py = gap.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp31)) { op_le(&__cmp31, &after.clone()) } else { Py::Bool(false) } }) {
    distance = gap.clone();
    } else {
    gap = op_sub(&mcall(&cue.clone(), "start", &[]), &end.clone());
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp41: Py = gap.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp41)) { op_le(&__cmp41, &before.clone()) } else { Py::Bool(false) } }))) {
    continue;
    }
    distance = gap.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&distance.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![distance.clone(), value.clone(), start.clone(), end.clone()]);
    }
    }
    return best.clone();
    Py::None
}
pub fn _the_incomes_in_the_line(args: &[Py]) -> Py {
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
    let mut sums: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sums") { v.clone() }
    else { Py::None };
    let mut _distance: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut nearest: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut start: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("[(value, start, end)] for every figure the line calls money coming in.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_INCOME(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_A_CODE(), "search", &[line.clone()]) } } } }) {
    return py_list(vec![]);
    }
    if py_truthy(&Py::Bool(py_is(&sums.clone(), &Py::None))) {
    sums = _sums(&[line.clone()]);
    }
    found = py_list(vec![]);
    seen = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_INCOME(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    cue = __item.clone();
    nearest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(cue.clone()); __args.push(sums.clone()); __kws.extend(vec![("after".to_string(), Py::Int(20)), ("before".to_string(), Py::Int(8))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_nearest(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&nearest.clone()))) {
    continue;
    }
    let __unpk2 = py_unpack(&nearest.clone(), 4, None);
    _distance = __unpk2[0].clone();
    value = __unpk2[1].clone();
    start = __unpk2[2].clone();
    end = __unpk2[3].clone();
    if py_truthy(&Py::Bool(py_contains(&seen.clone(), &start.clone()))) {
    continue;
    }
    let _ = mcall(&seen.clone(), "append", &[start.clone()]);
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![value.clone(), start.clone(), end.clone()])]);
    }
    return found.clone();
    Py::None
}
pub fn the_income_in_the_line(args: &[Py]) -> Py {
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
    let mut _end: Py = Py::None;
    let mut _start: Py = Py::None;
    let mut incomes: Py = Py::None;
    let mut line: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("(value, period) for \"im on 1800 a month\" and \"i take home 1800\", or None.\n\n    Every wage the line names is money coming in. \"i make 1200 and my partner makes 800 and the\n    rent is 900\" is a household of £2,000; reading only the first wage left £800 of it off the\n    month and said \"£300 left after the rent\".\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    incomes = _the_incomes_in_the_line(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&incomes.clone()))) {
    return Py::None;
    }
    return py_tuple(vec![call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&incomes.clone()) { let __unpk7518 = py_unpack(&__item.clone(), 3, None); let mut value: Py = __unpk7518[0].clone(); let mut _start: Py = __unpk7518[1].clone(); let mut _end: Py = __unpk7518[2].clone(); __out.push(value.clone()); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), _period(&[line.clone()])]);
    Py::None
}
pub fn _the_cost_the_figure_is_for(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut best: Py = Py::None;
    let mut distance: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut on_shape: Py = Py::None;
    let _ = py_str("The cost a figure belongs to, or \"\" when nothing beside it says money is going out.\n\n    Either order: \"rent is 700\" and \"the childcare is 400\" name the cost before the figure, while\n    \"400 goes on childcare\" and \"600 on food\" name it after. The nearest cost word wins, so a line\n    with two of them - \"rent is 700 and food is 300\" - puts each figure with its own word.\n    ");
    before = slice_or_raise(&line.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&start.clone(), &Py::Int(26))]).unwrap_or_else(|e| exc::raise(e)), &start.clone(), &Py::None);
    after = slice_or_raise(&line.clone(), &end.clone(), &op_add(&end.clone(), &Py::Int(26)), &Py::None);
    best = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_FIXED_COST(), "finditer", &[before.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    distance = op_sub(&call_builtin("len", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &mcall(&match_.clone(), "end", &[]));
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&distance.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![distance.clone(), mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[])]);
    }
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_FIXED_COST(), "finditer", &[after.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    distance = mcall(&match_.clone(), "start", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&distance.clone(), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![distance.clone(), mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[])]);
    }
    }
    if py_truthy(&best.clone()) {
    return geti(&best.clone(), &Py::Int(1));
    }
    on_shape = mcall(&g_AFTER_THE_FIGURE(), "match", &[after.clone()]);
    if py_truthy(&{ let __b = on_shape.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_COST_NAME(), "match", &[mcall(&on_shape.clone(), "group", &[py_str("noun")])]))) } }) {
    return mcall(&mcall(&on_shape.clone(), "group", &[py_str("noun")]), "lower", &[]);
    }
    return py_str("");
    Py::None
}
pub fn _the_unit_beside(args: &[Py]) -> Py {
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
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The period said right after a figure: \"900 a month\" is a month, \"900\" says nothing.");
    tail = slice_or_raise(&line.clone(), &end.clone(), &op_add(&end.clone(), &Py::Int(16)), &Py::None);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_MONTH(), py_str("a month")]), py_tuple(vec![g_WEEK(), py_str("a week")]), py_tuple(vec![g_YEAR(), py_str("a year")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    unit = __unpk2[1].clone();
    match_ = mcall(&pattern.clone(), "search", &[tail.clone()]);
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&tail.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,-")]))) } }) {
    return unit.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn the_costs_with_units_in_the_line(args: &[Py]) -> Py {
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
    let mut _end: Py = Py::None;
    let mut _value: Py = Py::None;
    let mut end: Py = Py::None;
    let mut found: Py = Py::None;
    let mut incomes: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut start: Py = Py::None;
    let mut sums: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("[(name, value, unit)] for the costs the line names, each with the period it was said in.\n\n    \"and rent is 900\" is one. \"rent is due on the 2nd\" names none, and \"rent is 750 and i put 200\n    aside\" is one cost and one pot, so the pot is not read as a cost.\n\n    Every figure that is not the income comes off, not just the last one: \"i earn 2000 and rent is\n    700 and food is 300\" left £1,300 when only the rent was counted, and the food was gone from the\n    sentence, which is the one thing a money answer cannot do. A figure nothing in the line\n    explains - neither the income nor a cost - takes the whole reading with it: half a sum is worse\n    than none.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_list(vec![]);
    }
    sums = _sums(&[line.clone()]);
    incomes = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_incomes_in_the_line(&[line.clone(), sums.clone()])) { let __unpk2963 = py_unpack(&__item.clone(), 3, None); let mut _value: Py = __unpk2963[0].clone(); let mut start: Py = __unpk2963[1].clone(); let mut _end: Py = __unpk2963[2].clone(); __out.push(start.clone()); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&sums.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    value = __unpk2[0].clone();
    start = __unpk2[1].clone();
    end = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_contains(&incomes.clone(), &start.clone()))) {
    continue;
    }
    if py_truthy(&_in_phrase(&[line.clone(), start.clone(), end.clone(), g_A_HAND_HELD()])) {
    continue;
    }
    name = _the_cost_the_figure_is_for(&[line.clone(), start.clone(), end.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return py_list(vec![]);
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![name.clone(), value.clone(), _the_unit_beside(&[line.clone(), end.clone()])])]);
    }
    return found.clone();
    Py::None
}
pub fn the_outgoings_in_the_line(args: &[Py]) -> Py {
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
    let mut _unit: Py = Py::None;
    let mut name: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("[(name, value)] for the costs the line names with a figure beside them, all of them.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&the_costs_with_units_in_the_line(&[text.clone()])) { let __unpk8675 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk8675[0].clone(); let mut value: Py = __unpk8675[1].clone(); let mut _unit: Py = __unpk8675[2].clone(); __out.push(py_tuple(vec![name.clone(), value.clone()])); } py_list(__out) })() };
    Py::None
}
pub fn _there_is_memory(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the chat has said anything before this line.\n\n    The runtime writes memory[\"turns\"] after the turn, so during a first line there is nothing\n    behind it: a reply built on the chat needs the chat, and on the first line only the line itself\n    can answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_income_before(args: &[Py]) -> Py {
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
    let mut income: Py = Py::None;
    let mut line: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("(value, period) from the two lines before, when this one says only what goes out.\n\n    \"i earn about 1800 a month\" / \"and rent is 900\": the second line is the first with the income\n    off the screen, and it was answered \"Got it - rent is 900.\" Only the two lines before, so an\n    income said ten turns ago in another subject is not brought back as this month's.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))) } }) {
    continue;
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_gt(&seen.clone(), &Py::Int(2))) {
    return Py::None;
    }
    income = the_income_in_the_line(&[line.clone()]);
    if py_truthy(&income.clone()) {
    return income.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_costs_before(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("[(name, value)] for the costs the two lines before named with a figure.\n\n    \"we bring in 2400 a month and the mortgage is 900\" / \"the nursery is 600 on top\": the second\n    line is not a fresh month with only a nursery in it. Read on its own the sum came out as £1,800\n    left, which is £600 more than the person has - the first line's mortgage had gone off the\n    screen. What is already off the month stays off it, and the names of both are said.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    found = py_list(vec![]);
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))) } }) {
    continue;
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_gt(&seen.clone(), &Py::Int(2))) {
    break;
    }
    found = op_add(&the_outgoings_in_the_line(&[line.clone()]), &found.clone());
    }
    return found.clone();
    Py::None
}
pub fn _remember(args: &[Py]) -> Py {
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
    let mut left: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "left") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: left"))) };
    let mut unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let _ = py_str("Leave a note that this turn was the month set out, for the line that asks what comes next.");
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&memory.clone(), &py_str("the_month_set_out"), py_dict(vec![(py_str("left"), left.clone()), (py_str("unit"), unit.clone())]));
    }
    Py::None
}
pub fn _the_pot_in_the_line(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut line: Py = Py::None;
    let mut start: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The figure the line puts by, or 0: \"rent is 750 and i put 200 aside\".");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&_sums(&[line.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    value = __unpk2[0].clone();
    start = __unpk2[1].clone();
    end = __unpk2[2].clone();
    if py_truthy(&_in_phrase(&[line.clone(), start.clone(), end.clone(), g_A_HAND_HELD()])) {
    return value.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _the_left_again(args: &[Py]) -> Py {
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
    let mut left: Py = Py::None;
    let mut month: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The figure the month left, said again to the line that asks what is left of it.");
    month = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("the_month_set_out")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[month.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    left = mcall(&month.clone(), "get", &[py_str("left")]);
    unit = { let __b = mcall(&month.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("a month") } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&left.clone())); if py_truthy(&__b) { __b } else { op_le(&left.clone(), &Py::Int(0)) } }) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("left".to_string(), mcall(&py_str("£{:,}"), "format", &[call_builtin("int", &[call_builtin("round", &[call_builtin("float", &[left.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))])), ("unit".to_string(), unit.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choice(&[g_LEFT_AGAIN(), line.clone(), memory.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _the_ask_points_back(args: &[Py]) -> Py {
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
    let _ = py_str("Whether \"where do i start\" is about the month or about something else entirely.");
    if py_truthy(&mcall(&g_A_STEP_ASK_POINTS_BACK(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    return op_le(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("[A-Za-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4));
    Py::None
}
pub fn _the_month_was_just_set_out(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the turn before this one was the sum, so \"what should i do first\" has a subject.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&memory.clone(), "get", &[py_str("the_month_set_out")]))) } }) {
    return Py::Bool(false);
    }
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    said = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bleaves \\u00a3|\\bleft after the\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_unit_in_the_chat(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The unit the chat is speaking in: \"how much should i save a month\" sets the month.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(2)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { py_str("") };
    if py_truthy(&call_builtin("isinstance", &[line.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    unit = _period(&[line.clone()]);
    if py_truthy(&unit.clone()) {
    return unit.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_chat_is_about_saving(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the last two turns were about putting money by.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:save|saves|saving|savings|put\\s+(?:by|aside|away)|budget|deposit|putting\\s+by)\\b"), mcall(&py_str(" "), "join", &[said.clone()]), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
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
    let mut _name: Py = Py::None;
    let mut _said: Py = Py::None;
    let mut _value: Py = Py::None;
    let mut a_month: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut before: Py = Py::None;
    let mut cost: Py = Py::None;
    let mut cost_unit: Py = Py::None;
    let mut costs: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut income: Py = Py::None;
    let mut left: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut own_month: Py = Py::None;
    let mut pot: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut spent_all: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut with_units: Py = Py::None;
    let _ = py_str("The reply a line about the month gets, or \"\" when it is not one.\n\n    A reply only where the arithmetic is the answer: what is left, or the share of a take-home in a\n    chat that is already about saving. Everything else - arrears, an ask, a rent that is going up, a\n    tax code - keeps the reply it already had.\n    ");
    line = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(220)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_A_LEFT_ASK(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { _the_month_was_just_set_out(&[memory.clone()]) } }) {
    return _the_left_again(&[line.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_A_FIRST_ASK(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _the_month_was_just_set_out(&[memory.clone()]); if !py_truthy(&__b) { __b } else { _the_ask_points_back(&[line.clone()]) } } } }) {
    return _choice(&[g_FIRST_MOVE(), line.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_A_BUDGET_LINE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_AN_ASK(), "search", &[line.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&op_gt(&call_builtin("len", &[_units_in(&[line.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return py_str("");
    }
    unit = _period(&[line.clone()]);
    income = the_income_in_the_line(&[line.clone()]);
    with_units = the_costs_with_units_in_the_line(&[line.clone()]);
    costs = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&with_units.clone()) { let __unpk2314 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk2314[0].clone(); let mut value: Py = __unpk2314[1].clone(); let mut _said: Py = __unpk2314[2].clone(); __out.push(py_tuple(vec![name.clone(), value.clone()])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_there_is_memory(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = income.clone(); if !py_truthy(&__b) { __b } else { costs.clone() } })) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = income.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&unit.clone())) } }) {
    income = py_tuple(vec![geti(&income.clone(), &Py::Int(0)), _the_unit_in_the_chat(&[memory.clone()])]);
    }
    own_month = call_builtin("bool", &[income.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&income.clone()))) {
    earlier = _the_income_before(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&earlier.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&costs.clone())) } }) {
    return py_str("");
    }
    income = py_tuple(vec![geti(&earlier.clone(), &Py::Int(0)), { let __b = geti(&earlier.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { { let __b = unit.clone(); if py_truthy(&__b) { __b } else { _the_unit_in_the_chat(&[memory.clone()]) } } } }]);
    }
    let __unpk1 = py_unpack(&income.clone(), 2, None);
    value = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    if py_truthy(&costs.clone()) {
    before = py_list(vec![]);
    if py_truthy(&Py::Bool(!py_truthy(&own_month.clone()))) {
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_costs_before(&[memory.clone()])) { let mut cost: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&costs.clone()) { let __unpk165 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk165[0].clone(); let mut _value: Py = __unpk165[1].clone(); __out.push(name.clone()); } py_list(__out) })() }, &geti(&cost.clone(), &Py::Int(0))))) {  __out.push(cost.clone()); } } py_list(__out) })() };
    }
    costs = op_add(&before.clone(), &costs.clone());
    cost = geti(&geti(&costs.clone(), &op_neg(&Py::Int(1))), &Py::Int(1));
    named = _the_names(&[costs.clone()]);
    cost_unit = if py_truthy(&with_units.clone()) { geti(&geti(&with_units.clone(), &op_neg(&Py::Int(1))), &Py::Int(2)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[costs.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let __b = cost_unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&cost_unit.clone(), &unit.clone())) } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&unit.clone(), &py_str("a year"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&cost_unit.clone(), &py_str("a year"))) } }) {
    a_month = call_builtin("round", &[op_truediv(&value.clone(), &Py::Float(12.0))]).unwrap_or_else(|e| exc::raise(e));
    left = op_sub(&a_month.clone(), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&costs.clone()) { let __unpk7236 = py_unpack(&__item.clone(), 2, None); let mut _name: Py = __unpk7236[0].clone(); let mut cost: Py = __unpk7236[1].clone(); __out.push(cost.clone()); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&{ let __b = op_le(&left.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_lt(&left.clone(), &op_mul(&a_month.clone(), &Py::Float(0.05))) } }) {
    return py_str("");
    }
    let _ = _remember(&[memory.clone(), left.clone(), py_str("a month")]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("in_amount".to_string(), _pounds(&[value.clone()])), ("a_month".to_string(), _pounds(&[a_month.clone()])), ("named".to_string(), named.clone()), ("left".to_string(), _with_unit(&[left.clone(), py_str("a month")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choice(&[g_A_YEAR_AND_A_MONTH(), line.clone(), memory.clone()]), "format", args))(&__args) };
    }
    left = op_sub(&value.clone(), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&costs.clone()) { let __unpk5220 = py_unpack(&__item.clone(), 2, None); let mut _name: Py = __unpk5220[0].clone(); let mut cost: Py = __unpk5220[1].clone(); __out.push(cost.clone()); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&{ let __b = op_le(&left.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_lt(&left.clone(), &op_mul(&value.clone(), &Py::Float(0.05))) } }) {
    return py_str("");
    }
    spent_all = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&costs.clone()) { let __unpk7680 = py_unpack(&__item.clone(), 2, None); let mut _name: Py = __unpk7680[0].clone(); let mut amount: Py = __unpk7680[1].clone(); __out.push(amount.clone()); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    pot = _the_pot_in_the_line(&[line.clone()]);
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("left".to_string(), _with_unit(&[left.clone(), unit.clone()])), ("named".to_string(), named.clone()), ("in_amount".to_string(), _pounds(&[value.clone()])), ("out_amount".to_string(), _pounds(&[spent_all.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choice(&[g_LEFT(), line.clone(), memory.clone()]), "format", args))(&__args) };
    if py_truthy(&pot.clone()) {
    let __aug2 = op_add(&reply.clone(), &op_modulo(&py_str(" The %s you put by comes out of that."), &_pounds(&[pot.clone()])));
    reply = __aug2;
    }
    let _ = _remember(&[memory.clone(), left.clone(), unit.clone()]);
    return reply.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_chat_is_about_saving(&[memory.clone()])))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&unit.clone()))) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("in_amount".to_string(), _pounds(&[value.clone()])), ("low".to_string(), _pounds(&[op_mul(&value.clone(), &g_LOW_SHARE())])), ("high".to_string(), _pounds(&[op_mul(&value.clone(), &g_HIGH_SHARE())])), ("period".to_string(), unit.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_choice(&[g_A_SHARE(), line.clone(), memory.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.outgoings".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFTER_THE_FIGURE" => Some(g_AFTER_THE_FIGURE()),
            "AN_ASK" => Some(g_AN_ASK()),
            "A_CODE" => Some(g_A_CODE()),
            "A_FIRST_ASK" => Some(g_A_FIRST_ASK()),
            "A_HAND_HELD" => Some(g_A_HAND_HELD()),
            "A_LEFT_ASK" => Some(g_A_LEFT_ASK()),
            "A_MEASURE" => Some(g_A_MEASURE()),
            "A_SHARE" => Some(g_A_SHARE()),
            "A_STEP_ASK_POINTS_BACK" => Some(g_A_STEP_ASK_POINTS_BACK()),
            "A_YEAR_AND_A_MONTH" => Some(g_A_YEAR_AND_A_MONTH()),
            "BARE_NUMBER" => Some(g_BARE_NUMBER()),
            "FIRST_MOVE" => Some(g_FIRST_MOVE()),
            "FIXED_COST" => Some(g_FIXED_COST()),
            "HIGH_SHARE" => Some(g_HIGH_SHARE()),
            "INCOME" => Some(g_INCOME()),
            "LEFT" => Some(g_LEFT()),
            "LEFT_AGAIN" => Some(g_LEFT_AGAIN()),
            "LOW_SHARE" => Some(g_LOW_SHARE()),
            "MONTH" => Some(g_MONTH()),
            "NOT_A_BUDGET_LINE" => Some(g_NOT_A_BUDGET_LINE()),
            "NOT_A_COST_NAME" => Some(g_NOT_A_COST_NAME()),
            "SUM" => Some(g_SUM()),
            "WEEK" => Some(g_WEEK()),
            "YEAR" => Some(g_YEAR()),
            "_THE_COST" => Some(g__THE_COST()),
            "_choice" => Some(py_func("_choice", |args: &[Py]| _choice(args))),
            "_in_phrase" => Some(py_func("_in_phrase", |args: &[Py]| _in_phrase(args))),
            "_near" => Some(py_func("_near", |args: &[Py]| _near(args))),
            "_period" => Some(py_func("_period", |args: &[Py]| _period(args))),
            "_pounds" => Some(py_func("_pounds", |args: &[Py]| _pounds(args))),
            "_remember" => Some(py_func("_remember", |args: &[Py]| _remember(args))),
            "_sums" => Some(py_func("_sums", |args: &[Py]| _sums(args))),
            "_the_ask_points_back" => Some(py_func("_the_ask_points_back", |args: &[Py]| _the_ask_points_back(args))),
            "_the_chat_is_about_saving" => Some(py_func("_the_chat_is_about_saving", |args: &[Py]| _the_chat_is_about_saving(args))),
            "_the_cost_the_figure_is_for" => Some(py_func("_the_cost_the_figure_is_for", |args: &[Py]| _the_cost_the_figure_is_for(args))),
            "_the_costs_before" => Some(py_func("_the_costs_before", |args: &[Py]| _the_costs_before(args))),
            "_the_income_before" => Some(py_func("_the_income_before", |args: &[Py]| _the_income_before(args))),
            "_the_incomes_in_the_line" => Some(py_func("_the_incomes_in_the_line", |args: &[Py]| _the_incomes_in_the_line(args))),
            "_the_left_again" => Some(py_func("_the_left_again", |args: &[Py]| _the_left_again(args))),
            "_the_month_was_just_set_out" => Some(py_func("_the_month_was_just_set_out", |args: &[Py]| _the_month_was_just_set_out(args))),
            "_the_names" => Some(py_func("_the_names", |args: &[Py]| _the_names(args))),
            "_the_nearest" => Some(py_func("_the_nearest", |args: &[Py]| _the_nearest(args))),
            "_the_pot_in_the_line" => Some(py_func("_the_pot_in_the_line", |args: &[Py]| _the_pot_in_the_line(args))),
            "_the_unit_beside" => Some(py_func("_the_unit_beside", |args: &[Py]| _the_unit_beside(args))),
            "_the_unit_in_the_chat" => Some(py_func("_the_unit_in_the_chat", |args: &[Py]| _the_unit_in_the_chat(args))),
            "_there_is_memory" => Some(py_func("_there_is_memory", |args: &[Py]| _there_is_memory(args))),
            "_units_in" => Some(py_func("_units_in", |args: &[Py]| _units_in(args))),
            "_with_unit" => Some(py_func("_with_unit", |args: &[Py]| _with_unit(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_costs_with_units_in_the_line" => Some(py_func("the_costs_with_units_in_the_line", |args: &[Py]| the_costs_with_units_in_the_line(args))),
            "the_income_in_the_line" => Some(py_func("the_income_in_the_line", |args: &[Py]| the_income_in_the_line(args))),
            "the_outgoings_in_the_line" => Some(py_func("the_outgoings_in_the_line", |args: &[Py]| the_outgoings_in_the_line(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AFTER_THE_FIGURE" => s_AFTER_THE_FIGURE(v),
            "AN_ASK" => s_AN_ASK(v),
            "A_CODE" => s_A_CODE(v),
            "A_FIRST_ASK" => s_A_FIRST_ASK(v),
            "A_HAND_HELD" => s_A_HAND_HELD(v),
            "A_LEFT_ASK" => s_A_LEFT_ASK(v),
            "A_MEASURE" => s_A_MEASURE(v),
            "A_SHARE" => s_A_SHARE(v),
            "A_STEP_ASK_POINTS_BACK" => s_A_STEP_ASK_POINTS_BACK(v),
            "A_YEAR_AND_A_MONTH" => s_A_YEAR_AND_A_MONTH(v),
            "BARE_NUMBER" => s_BARE_NUMBER(v),
            "FIRST_MOVE" => s_FIRST_MOVE(v),
            "FIXED_COST" => s_FIXED_COST(v),
            "HIGH_SHARE" => s_HIGH_SHARE(v),
            "INCOME" => s_INCOME(v),
            "LEFT" => s_LEFT(v),
            "LEFT_AGAIN" => s_LEFT_AGAIN(v),
            "LOW_SHARE" => s_LOW_SHARE(v),
            "MONTH" => s_MONTH(v),
            "NOT_A_BUDGET_LINE" => s_NOT_A_BUDGET_LINE(v),
            "NOT_A_COST_NAME" => s_NOT_A_COST_NAME(v),
            "SUM" => s_SUM(v),
            "WEEK" => s_WEEK(v),
            "YEAR" => s_YEAR(v),
            "_THE_COST" => s__THE_COST(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.outgoings", module_obj);
    pyrt::xrefs::reg_fn("memory.outgoings", "_choice", py_func("_choice", |args: &[Py]| _choice(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_names", py_func("_the_names", |args: &[Py]| _the_names(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_sums", py_func("_sums", |args: &[Py]| _sums(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_period", py_func("_period", |args: &[Py]| _period(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_units_in", py_func("_units_in", |args: &[Py]| _units_in(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_pounds", py_func("_pounds", |args: &[Py]| _pounds(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_with_unit", py_func("_with_unit", |args: &[Py]| _with_unit(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_in_phrase", py_func("_in_phrase", |args: &[Py]| _in_phrase(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_near", py_func("_near", |args: &[Py]| _near(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_nearest", py_func("_the_nearest", |args: &[Py]| _the_nearest(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_incomes_in_the_line", py_func("_the_incomes_in_the_line", |args: &[Py]| _the_incomes_in_the_line(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "the_income_in_the_line", py_func("the_income_in_the_line", |args: &[Py]| the_income_in_the_line(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_cost_the_figure_is_for", py_func("_the_cost_the_figure_is_for", |args: &[Py]| _the_cost_the_figure_is_for(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_unit_beside", py_func("_the_unit_beside", |args: &[Py]| _the_unit_beside(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "the_costs_with_units_in_the_line", py_func("the_costs_with_units_in_the_line", |args: &[Py]| the_costs_with_units_in_the_line(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "the_outgoings_in_the_line", py_func("the_outgoings_in_the_line", |args: &[Py]| the_outgoings_in_the_line(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_there_is_memory", py_func("_there_is_memory", |args: &[Py]| _there_is_memory(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_income_before", py_func("_the_income_before", |args: &[Py]| _the_income_before(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_costs_before", py_func("_the_costs_before", |args: &[Py]| _the_costs_before(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_remember", py_func("_remember", |args: &[Py]| _remember(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_pot_in_the_line", py_func("_the_pot_in_the_line", |args: &[Py]| _the_pot_in_the_line(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_left_again", py_func("_the_left_again", |args: &[Py]| _the_left_again(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_ask_points_back", py_func("_the_ask_points_back", |args: &[Py]| _the_ask_points_back(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_month_was_just_set_out", py_func("_the_month_was_just_set_out", |args: &[Py]| _the_month_was_just_set_out(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_unit_in_the_chat", py_func("_the_unit_in_the_chat", |args: &[Py]| _the_unit_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "_the_chat_is_about_saving", py_func("_the_chat_is_about_saving", |args: &[Py]| _the_chat_is_about_saving(args)));
    pyrt::xrefs::reg_fn("memory.outgoings", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.outgoings", "AFTER_THE_FIGURE", g_AFTER_THE_FIGURE, s_AFTER_THE_FIGURE);
    pyrt::xrefs::reg_access("memory.outgoings", "AN_ASK", g_AN_ASK, s_AN_ASK);
    pyrt::xrefs::reg_access("memory.outgoings", "A_CODE", g_A_CODE, s_A_CODE);
    pyrt::xrefs::reg_access("memory.outgoings", "A_FIRST_ASK", g_A_FIRST_ASK, s_A_FIRST_ASK);
    pyrt::xrefs::reg_access("memory.outgoings", "A_HAND_HELD", g_A_HAND_HELD, s_A_HAND_HELD);
    pyrt::xrefs::reg_access("memory.outgoings", "A_LEFT_ASK", g_A_LEFT_ASK, s_A_LEFT_ASK);
    pyrt::xrefs::reg_access("memory.outgoings", "A_MEASURE", g_A_MEASURE, s_A_MEASURE);
    pyrt::xrefs::reg_access("memory.outgoings", "A_SHARE", g_A_SHARE, s_A_SHARE);
    pyrt::xrefs::reg_access("memory.outgoings", "A_STEP_ASK_POINTS_BACK", g_A_STEP_ASK_POINTS_BACK, s_A_STEP_ASK_POINTS_BACK);
    pyrt::xrefs::reg_access("memory.outgoings", "A_YEAR_AND_A_MONTH", g_A_YEAR_AND_A_MONTH, s_A_YEAR_AND_A_MONTH);
    pyrt::xrefs::reg_access("memory.outgoings", "BARE_NUMBER", g_BARE_NUMBER, s_BARE_NUMBER);
    pyrt::xrefs::reg_access("memory.outgoings", "FIRST_MOVE", g_FIRST_MOVE, s_FIRST_MOVE);
    pyrt::xrefs::reg_access("memory.outgoings", "FIXED_COST", g_FIXED_COST, s_FIXED_COST);
    pyrt::xrefs::reg_access("memory.outgoings", "HIGH_SHARE", g_HIGH_SHARE, s_HIGH_SHARE);
    pyrt::xrefs::reg_access("memory.outgoings", "INCOME", g_INCOME, s_INCOME);
    pyrt::xrefs::reg_access("memory.outgoings", "LEFT", g_LEFT, s_LEFT);
    pyrt::xrefs::reg_access("memory.outgoings", "LEFT_AGAIN", g_LEFT_AGAIN, s_LEFT_AGAIN);
    pyrt::xrefs::reg_access("memory.outgoings", "LOW_SHARE", g_LOW_SHARE, s_LOW_SHARE);
    pyrt::xrefs::reg_access("memory.outgoings", "MONTH", g_MONTH, s_MONTH);
    pyrt::xrefs::reg_access("memory.outgoings", "NOT_A_BUDGET_LINE", g_NOT_A_BUDGET_LINE, s_NOT_A_BUDGET_LINE);
    pyrt::xrefs::reg_access("memory.outgoings", "NOT_A_COST_NAME", g_NOT_A_COST_NAME, s_NOT_A_COST_NAME);
    pyrt::xrefs::reg_access("memory.outgoings", "SUM", g_SUM, s_SUM);
    pyrt::xrefs::reg_access("memory.outgoings", "WEEK", g_WEEK, s_WEEK);
    pyrt::xrefs::reg_access("memory.outgoings", "YEAR", g_YEAR, s_YEAR);
    pyrt::xrefs::reg_access("memory.outgoings", "_THE_COST", g__THE_COST, s__THE_COST);
}
