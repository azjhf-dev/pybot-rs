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


thread_local! { static G_AN_AGE_UNDER_A_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_EARLY_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_VISITOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COMING_THAT_IS_NOT_A_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COMING_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FEED_FIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FEED_FIGHT_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEDS_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEDS_CHANGE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEDS_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEDS_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAST_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAY_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAY_SOMEBODY_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_IN_THE_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_ANYWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHORT_PAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STANDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STANDING_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBJECT_BESIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBJECT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBJECT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRADE_VISITOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VISIT_CARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VISIT_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VISIT_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VISIT_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WATER_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EARLY_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSTURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_PAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNDER_A_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VISITORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATER_IN_THE_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATER_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AN_AGE_UNDER_A_YEAR() -> Py { ensure_init(); G_AN_AGE_UNDER_A_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AGE_UNDER_A_YEAR(v: Py) { ensure_init(); G_AN_AGE_UNDER_A_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_EARLY_HOUR() -> Py { ensure_init(); G_AN_EARLY_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_EARLY_HOUR(v: Py) { ensure_init(); G_AN_EARLY_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BABY() -> Py { ensure_init(); G_A_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BABY(v: Py) { ensure_init(); G_A_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_VISITOR() -> Py { ensure_init(); G_A_CARE_VISITOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_VISITOR(v: Py) { ensure_init(); G_A_CARE_VISITOR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COMING_THAT_IS_NOT_A_VISIT() -> Py { ensure_init(); G_A_COMING_THAT_IS_NOT_A_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COMING_THAT_IS_NOT_A_VISIT(v: Py) { ensure_init(); G_A_COMING_THAT_IS_NOT_A_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COMING_VISIT() -> Py { ensure_init(); G_A_COMING_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COMING_VISIT(v: Py) { ensure_init(); G_A_COMING_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FEED() -> Py { ensure_init(); G_A_FEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEED(v: Py) { ensure_init(); G_A_FEED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FEED_FIGHT() -> Py { ensure_init(); G_A_FEED_FIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEED_FIGHT(v: Py) { ensure_init(); G_A_FEED_FIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FEED_FIGHT_REPLY() -> Py { ensure_init(); G_A_FEED_FIGHT_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEED_FIGHT_REPLY(v: Py) { ensure_init(); G_A_FEED_FIGHT_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEDS() -> Py { ensure_init(); G_A_MEDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEDS(v: Py) { ensure_init(); G_A_MEDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEDS_CHANGE() -> Py { ensure_init(); G_A_MEDS_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEDS_CHANGE(v: Py) { ensure_init(); G_A_MEDS_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEDS_CHANGE_REPLY() -> Py { ensure_init(); G_A_MEDS_CHANGE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEDS_CHANGE_REPLY(v: Py) { ensure_init(); G_A_MEDS_CHANGE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEDS_MINE() -> Py { ensure_init(); G_A_MEDS_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEDS_MINE(v: Py) { ensure_init(); G_A_MEDS_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEDS_SYMPTOM() -> Py { ensure_init(); G_A_MEDS_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEDS_SYMPTOM(v: Py) { ensure_init(); G_A_MEDS_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAST_VISIT() -> Py { ensure_init(); G_A_PAST_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAST_VISIT(v: Py) { ensure_init(); G_A_PAST_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAY_OWN() -> Py { ensure_init(); G_A_PAY_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAY_OWN(v: Py) { ensure_init(); G_A_PAY_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAY_SOMEBODY_ELSE() -> Py { ensure_init(); G_A_PAY_SOMEBODY_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAY_SOMEBODY_ELSE(v: Py) { ensure_init(); G_A_PAY_SOMEBODY_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_IN_THE_VISIT() -> Py { ensure_init(); G_A_PERSON_IN_THE_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_IN_THE_VISIT(v: Py) { ensure_init(); G_A_PERSON_IN_THE_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_ANYWHERE() -> Py { ensure_init(); G_A_QUESTION_ANYWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_ANYWHERE(v: Py) { ensure_init(); G_A_QUESTION_ANYWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_OPENER() -> Py { ensure_init(); G_A_QUESTION_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_OPENER(v: Py) { ensure_init(); G_A_QUESTION_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHIFT() -> Py { ensure_init(); G_A_SHIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHIFT(v: Py) { ensure_init(); G_A_SHIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHORT_PAY() -> Py { ensure_init(); G_A_SHORT_PAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHORT_PAY(v: Py) { ensure_init(); G_A_SHORT_PAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STANDING() -> Py { ensure_init(); G_A_STANDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STANDING(v: Py) { ensure_init(); G_A_STANDING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STANDING_UP() -> Py { ensure_init(); G_A_STANDING_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STANDING_UP(v: Py) { ensure_init(); G_A_STANDING_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBJECT_BESIDE() -> Py { ensure_init(); G_A_SUBJECT_BESIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT_BESIDE(v: Py) { ensure_init(); G_A_SUBJECT_BESIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBJECT_FAMILY() -> Py { ensure_init(); G_A_SUBJECT_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT_FAMILY(v: Py) { ensure_init(); G_A_SUBJECT_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBJECT_FILLER() -> Py { ensure_init(); G_A_SUBJECT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT_FILLER(v: Py) { ensure_init(); G_A_SUBJECT_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRADE_VISITOR() -> Py { ensure_init(); G_A_TRADE_VISITOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRADE_VISITOR(v: Py) { ensure_init(); G_A_TRADE_VISITOR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VISIT_CARE() -> Py { ensure_init(); G_A_VISIT_CARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VISIT_CARE(v: Py) { ensure_init(); G_A_VISIT_CARE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VISIT_DAY() -> Py { ensure_init(); G_A_VISIT_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VISIT_DAY(v: Py) { ensure_init(); G_A_VISIT_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VISIT_PLACE() -> Py { ensure_init(); G_A_VISIT_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VISIT_PLACE(v: Py) { ensure_init(); G_A_VISIT_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VISIT_TRADE() -> Py { ensure_init(); G_A_VISIT_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VISIT_TRADE(v: Py) { ensure_init(); G_A_VISIT_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WATER_TROUBLE() -> Py { ensure_init(); G_A_WATER_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WATER_TROUBLE(v: Py) { ensure_init(); G_A_WATER_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_EARLY_START() -> Py { ensure_init(); G_EARLY_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EARLY_START(v: Py) { ensure_init(); G_EARLY_START.with(|c| *c.borrow_mut() = v); }
pub fn g_POSTURAL() -> Py { ensure_init(); G_POSTURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSTURAL(v: Py) { ensure_init(); G_POSTURAL.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_PAY() -> Py { ensure_init(); G_SHORT_PAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_PAY(v: Py) { ensure_init(); G_SHORT_PAY.with(|c| *c.borrow_mut() = v); }
pub fn g_UNDER_A_YEAR() -> Py { ensure_init(); G_UNDER_A_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNDER_A_YEAR(v: Py) { ensure_init(); G_UNDER_A_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_VISITORS() -> Py { ensure_init(); G_VISITORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VISITORS(v: Py) { ensure_init(); G_VISITORS.with(|c| *c.borrow_mut() = v); }
pub fn g_WATER_IN_THE_HOUSE() -> Py { ensure_init(); G_WATER_IN_THE_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATER_IN_THE_HOUSE(v: Py) { ensure_init(); G_WATER_IN_THE_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_WATER_THINGS() -> Py { ensure_init(); G_WATER_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATER_THINGS(v: Py) { ensure_init(); G_WATER_THINGS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Nothing in the store knows the line: say the one thing that is true about what it names.\n\nA plain account of something with nothing on the end of it - \"the tap has been running all\nmorning\", \"i think theyre underpaying me\", \"my shift starts at half 5 in the morning\", \"my\nvision is gone when i stand up\" - is not a question, so no pool answers it, and it reached the\nintake form: \"Got it. Is there anything you'd like a hand with?\". That reply offers help with a\ntask nobody mentioned and could have come before any message at all.\n\nWhat a friend does with a line like that is say the one thing they know about it: that water\nwhere it should not be is a today job and the stopcock comes first, that a short payslip is\nusually one of three things, that a half-five start is decided the night before. So the family\nof thing the line names decides the reply - water in the house, pay, an early shift, a body, a\nchild under a year, a visit that is booked - and each pool says the part that is true whatever\nthe details are, with no number of theirs invented and no promise about how it ends.\n\nOnly ever a reply where nothing answered. The caller runs this last, before the intake form,\nso a pool with something of its own has already had its turn.\n\nThere is no general body family here on purpose. \"my dad is having chest pain\" and \"my head is\nbleeding badly\" are statements about a body too, and no line this pool could write is right for\nthem; the health and urgent pools read those, and a wrong reply there is worse than the intake\nform. The one body shape here is standing up and the vision going, where the usual cause is\nordinary and the safe advice is the same for everybody.\n");
    s_A_QUESTION_OPENER(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|when|where|which|who|why|how|can|could|should|would|will|is|are|was|were|do|does|did|have|has|am)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUESTION_ANYWHERE(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|should|would|shall|is it|are they|is that|does it|do i|did i|too young|too old|allowed|normal)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WATER_THINGS(call_builtin("frozenset", &[mcall(&py_str("\n    tap taps faucet faucets pipe pipes cistern toilet sink bath shower boiler radiator\n    radiators stopcock valve overflow drain drains damp\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WATER_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:running|runs|leak|leaks|leaked|leaking|drip|drips|dripped|dripping|overflow\\w*|flood\\w*|burst|wont stop|won't stop|not stopping|wet|puddle|soaked|on all day|left on|left running|been on|on since|still on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PAY_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:payslip|payslips|wages|salary|overtime|hourly|underpaid|underpay|underpaying|my pay|short(?:er)? this month|short again|paid short)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PAY_SOMEBODY_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:rent|tenant|tenants|landlord|customer|customers|client|clients|invoice|invoices|refund|bill|bills|mate|friend|brother|sister|mum|dad|he|she|they|owes?|owed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SHORT_PAY(stdlib_call("re", "compile", &[py_str("\\b(?:underpay\\w*|underpaid|short|less than|miss(?:ed|ing)?|wrong|not paid|didn'?t pay|didnt pay|short-?changed|cheated|deduct\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SHIFT(stdlib_call("re", "compile", &[py_str("\\b(?:shift|shifts|rota|alarm)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_EARLY_HOUR(stdlib_call("re", "compile", &[py_str("(?:\\bhalf\\s+[3-7]\\b|\\b[3-7](?::\\d\\d)?\\s*(?:am|a\\.m\\.)|\\bup at (?:[3-7]|half)|\\bstart(?:s|ing)? (?:at |by )?[3-7]\\b|\\bearly start\\b|\\bearly shift\\b|\\bbefore (?:6|7|eight|seven|six)\\b|\\bdark\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POSTURAL(call_builtin("frozenset", &[mcall(&py_str("\n    vision eyes sight black blacks blacked dizzy dizziness faint faints fainting lightheaded\n    light-headed headrush head-rush stars ears ringing\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STANDING(stdlib_call("re", "compile", &[py_str("\\b(?:stand|stands|standing|stood|get up|gets up|getting up|got up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BABY(stdlib_call("re", "compile", &[py_str("\\b(?:(?:little|small|baby|newborn)\\s+(?:one|boy|girl|laddie|lad|fella)|baby|babies|newborn|infant|toddler)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AGE_UNDER_A_YEAR(stdlib_call("re", "compile", &[py_str("\\b(?:\\d{1,2}\\s*(?:months?|weeks?|month|week)|\\d{1,2}\\s*(?:mo|wk)s?\\b|months? old|weeks? old|a few months|six months|seven months|eight months|nine months|ten months|eleven months)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WATER_IN_THE_HOUSE(py_tuple(vec![py_str("Water where it should not be is a today job. Stopcock off first - under the kitchen sink or where the pipe comes into the house - and a tap that will not close is usually the washer, which costs pennies."), py_str("Turn the water off at the stopcock before anything else, then look: a tap that runs on is nearly always the washer, and once it is isolated a leak stops being an emergency.")]));
    s_SHORT_PAY(py_tuple(vec![py_str("Get the payslip out before you say anything to anyone. The gap is nearly always a break taken off twice, an hour that was not counted, or a tax code that changed without anybody telling you."), py_str("Compare the payslip against your own hours, then raise it with the number in your hand: it is usually one unpaid hour, a break deducted that you did not take, or the wrong tax code.")]));
    s_EARLY_START(py_tuple(vec![py_str("An early start is decided the night before: bag by the door, clothes out, and lights off early rather than catching up at the weekend - the lie-in is what makes the next morning worse."), py_str("The people who cope with an early start go to bed early rather than making up for it later. The same bedtime on days off, if you can: the lie-in is what makes the next week harder.")]));
    s_A_STANDING_UP(py_tuple(vec![py_str("Vision going as you stand is usually blood pressure dipping for a second. Go up in two stages - sit on the edge, then stand - and get it looked at if it keeps happening or you feel faint."), py_str("That is the ordinary stand-up drop in blood pressure. Two stages getting up, water rather than coffee first thing, and have it checked if it happens again today.")]));
    s_UNDER_A_YEAR(py_tuple(vec![py_str("Under a year, the next thing arrives before you have got used to the last one - sitting up, then lumps of food, then pulling to stand. Whatever stage they are at will be gone in a month."), py_str("That age is mostly a run of small milestones: sitting, then lumps, then pulling up. It changes faster than anybody gets comfortable with, so nothing odd lasts.")]));
    s_A_FEED(stdlib_call("re", "compile", &[py_str("\\b(?:bottle|bottles|breast|breastfeed\\w*|formula|feeding|feeds|feed|milk)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEED_FIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:fights|fighting|refuses|refusing|refused|wont|won't|keeps?|screams?|screaming|cries|howling|colic|colicky|reflux|sick|posset\\w*|not taking|turns away)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEED_FIGHT_REPLY(py_tuple(vec![py_str("A feed that is a fight is usually pace, wind or the teat: sit them up, keep the bottle level so the milk is in the teat and not pouring in, and stop halfway to burp. If they are still fighting it after that, the health visitor is the person to ask."), py_str("Try paced feeding before anything else - upright, bottle level, a pause to burp in the middle - and check the teat: too fast a flow makes a baby gulp and pull away. A health visitor can watch a feed if it carries on.")]));
    s_VISITORS(call_builtin("frozenset", &[mcall(&py_str("\nsomebody someone man bloke bloke lady guy fella chap\nboiler man boiler bloke plumber electrician gas man gas engineer engineer heating engineer\nbuilder joiner carpenter decorator painer painter plasterer tiler roofer glazier locksmith\nchimney sweep pest control drain man drain bloke surveyor cleaner window cleaner gardener\nmeter reader delivery man delivery driver courier van man removal man removals man fitter\ntechnician repair man washing machine man dishwasher man oven man tv man sky man\nbroadband man bt man openreach man virgin man internet man\nnurse district nurse community nurse health visitor midwife physio physiotherapist occupational\ntherapist carer support worker social worker housing officer housing association landlord\nletting agent agent estate agent vet vet nurse groom farrier\ncouncil the council council inspector environmental health housing housing officer\nscaffolder scaffolder scaffolder mechanic mot man aa man rac man\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COMING_VISIT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|am|will be|going to be|should be|due|booked|scheduled|expected)\\b[^.!?]{0,24}?\\b(?:coming|come|due|arriving|booked|booked in|scheduled|expected|turning up|coming round|coming over|coming out|coming to|visiting|seeing)\\b|\\b(?:comes|come|arrives|visits)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VISIT_DAY(stdlib_call("re", "compile", &[py_str("\\b(today|tonight|tomorrow|this morning|this afternoon|this evening|(?:mon|tues|wednes|thurs|fri|satur|sun)day|monday|tuesday|wednesday|thursday|friday|saturday|sunday|next week|in the morning|in the afternoon|in the evening|first thing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CARE_VISITOR(call_builtin("frozenset", &[mcall(&py_str("\nnurse nurses district community health visitor midwife midwives physio physiotherapist\noccupational therapist ot carer carers care worker support worker social worker gp doctor\nconsultant specialist matron paramedic phlebotomist chiropodist podiatrist dentist hygienist\naudiologist optician speech therapist slt\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRADE_VISITOR(call_builtin("frozenset", &[mcall(&py_str("\nboiler plumber plumbing electrician gas engineer heating engineer engineer builder joiner\ncarpenter decorator painter plasterer tiler roofer glazier locksmith sweep pest control drain\ncleaner gardener fitter technician repair man washing machine dishwasher oven tv sky man\nbroadband bt openreach virgin internet man chimney scaffolder mechanic mot man aa man rac man\nvan man removal man removals surveyor vet farrier groom\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VISIT_CARE(py_tuple(vec![py_str("{Day} then. Is it the usual check, or has something changed since the last one?"), py_str("Right - {Day}. Do you know what they are coming for, or is it a general check?"), py_str("Good to have it in the diary. Is somebody going to be with you for it?"), py_str("{Day} it is. Worth writing down the two or three things you want to ask while you think of them.")]));
    s_A_VISIT_TRADE(py_tuple(vec![py_str("Right - {day} then. Are they coming to look at it, or to do the job?"), py_str("{Day} it is. Is it the same one as last time, or somebody new?"), py_str("Good, that's in the diary. Do you have to be in for them, or can they get in?"), py_str("So {day} is the visit. Have they said how long they will be, or is it a wait in all morning?"), py_str("{Day} then. Is there anything you need to have ready for them before they get there?")]));
    s_A_VISIT_PLACE(py_tuple(vec![py_str("{Day} then. Do you need to be in for them, or can they get in?"), py_str("Right - {Day}. Have they said what they want to see, or is it just a look round?"), py_str("{Day} it is. Worth having whatever paperwork they asked about to hand."), py_str("Good. Is it the first time they have been out, or do you know who you are getting?")]));
    s_A_PAST_VISIT(stdlib_call("re", "compile", &[py_str("\\b(?:came|came round|came over|turned up|showed up|been round|was here|were here|visited|did come)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_IN_THE_VISIT(call_builtin("frozenset", &[mcall(&py_str("\nsomebody someone man bloke lady guy fella chap woman girl boy\nplumber electrician engineer builder joiner carpenter decorator painter plasterer tiler roofer\nglazier locksmith sweep cleaner gardener surveyor fitter technician mechanic\nvisitor therapist assessor inspector\nnurse midwife physio physiotherapist carer worker officer landlord agent vet groom farrier\ncouncil mechanic dentist hygienist optician consultant specialist matron paramedic\nphlebotomist chiropodist podiatrist driver reader\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COMING_THAT_IS_NOT_A_VISIT(stdlib_call("re", "compile", &[py_str("\\b(?:coming|come|comes|coming\\s+round|coming\\s+over)\\s+(?:back|out|up|off|through)\\b|\\b(?:back|out|up)\\s+from\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEDS(stdlib_call("re", "compile", &[py_str("\\b(?:meds|medication|medications|medicine|medicines|tablets?|pills?|capsules?|dose|doses|dosage|prescription|inhaler|inhalers|insulin|antidepressants?|antibiotics?|painkillers?|statins?|steroids?|hormones?|patch|patches|injections?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEDS_CHANGE(stdlib_call("re", "compile", &[py_str("\\b(?:changed|change|changes|changing|new|started|starting|start|switched|switch|moved|put me on|now on|instead of|upped|increased|lowered|reduced|stopped|come off|came off|different|put on|been put|gone up|gone down|went up|went down|twice a day|once a day|three times a day|swapped|swap|halved|halve|doubled|higher|lower|stronger|weaker)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEDS_MINE(stdlib_call("re", "compile", &[py_str("^(?:so |and |but |ok |okay |well |anyway |yeah |yeh |also )*(?:my\\s+(?:\\w+\\s+)?(?:meds|medications?|medicines?|tablets?|pills?|dose|dosage|prescription|inhaler|insulin|antidepressants?|antibiotics?|painkillers?|statins?|steroids?|hormones?|patch|patches|injections?)|i(?:'?m| am)\\s+(?:now\\s+)?(?:on|taking)|ive|i(?:'?ve)?\\s+been\\s+(?:put\\s+)?on|i\\s+(?:started|start|switched|changed|stopped|come off|came off)|i\\s+take\\b|i(?:'?ve|\\s+have|\\s+got)\\s+(?:some\\s+)?(?:new\\s+|different\\s+|another\\s+)?(?:meds|medication|medicines?|tablets?|pills?|capsules?|inhaler|insulin)(?:\\s+to\\s+take)?|(?:the|my)\\s+(?:doctor|gp|consultant|nurse|psychiatrist)\\s+(?:has\\s+)?(?:changed|put me on|switched|started me|gave me)|i(?:'?m| am)\\s+\\w+\\s+and\\s+my\\s+\\w+\\s+(?:have|has|was|were|is)\\s+\\w+ed|(?:they|he|she|the\\s+(?:doctor|gp|consultant|nurse|psychiatrist|chemist|pharmacist|hospital|surgery))\\s+(?:have\\s+|has\\s+)?(?:changed|switched|swapped|put me on|started me on|given me|gave me|moved me))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEDS_SYMPTOM(stdlib_call("re", "compile", &[py_str("\\b(?:wheez\\w*|breath\\w*|chest|bleed\\w*|rash|swollen|swelling|pain|hurts?|hurting|dizzy|faint\\w*|sick|vomit\\w*|temperature|fever|numb|collapse\\w*|seizure|fit|too many|double|twice the|overdose|took \\d|swallow\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEDS_CHANGE_REPLY(py_tuple(vec![py_str("A changed dose or a new tablet usually takes a week or two before you can tell what it is doing, so the pharmacist is worth one question: what to expect, and what to do about a missed one. They know more about the timing than anybody else on the high street."), py_str("Give it a week or two before you judge it - most medicines take that long to settle - and ask the pharmacist about anything that bothers you in between rather than waiting for the next appointment. A missed dose is their kind of question too.")]));
    s_A_SUBJECT_FAMILY(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:mortgages?|remortgag\\w*|re-?mortgag\\w*)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("A remortgage turns on the rate you can get now against the one you are on, plus the fee on top. Your lender's own figures and a broker's quote are the two to put side by side. Is this the borrowing side, picking a deal, or a letter from a lender?"), py_str("Mortgage paperwork comes down to the offer, the terms and the early repayment charge, and the pack they sent has each one written out. What are you trying to pin down - the amount, the deal, or something you have been sent?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:pensions?|state pension|workplace pension|company pension|personal pension|annuit\\w+)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Pensions split into the state one, a workplace pot and one you may have lost track of, and each has its own place to look: the state forecast comes off your National Insurance record, and a workplace pot sends a statement each year. Which one is yours?"), py_str("A pension question is usually about how much is in it, when it can be taken, or what it is invested in, and the statement answers the first. Is this one you are paying into now, one you are about to draw, or one you are trying to find?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:council tax|energy bills?|electric(?:ity)? bills?|gas bills?|water bills?|utility bills?|bills?)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("A bill comes down to the reading it was worked out from, the tariff you are on and any discount you are owed, and your supplier's account page shows all three for your address. Is it the amount that looks wrong, or more than you can carry this month?"), py_str("Energy, water and council tax each have their own account page and their own discount scheme, and the reading behind the figure is printed on the bill. Which bill is it?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:tax returns?|tax codes?|tax rebates?|tax refunds?|self[- ]assessment|p45s?|p60s?|tax|taxes|hmrc)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Tax codes, returns and rebates are all worked out from your own record, and HMRC's personal tax account shows what they hold for you. Is it what you owe, something you can claim, or the form itself?"), py_str("A tax question is usually about what you owe, what you can claim back, or the form itself, and the record behind it is yours to see. Is this a first return, or one you have done before and something has changed?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:life insurance|life cover|term insurance|car insurance|home insurance|contents insurance|travel insurance|insurance|insurances)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("An insurance question comes down to what is covered, the excess, and what you had to tell them, and all three are in the policy wording they sent you. Is it what it covers, what it costs, or a claim you are in the middle of?"), py_str("The cover, the excess and the exclusions are the three that decide an insurance question, and the wording you were sent names all three. Is this a quote, a claim, or something you are not sure you have to declare?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:cv|cvs|curriculum vitae|cover letters?|resume|job applications?)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("A CV is read in about a minute, so the last job, what you did in it and one line of what came of it do most of the work. Is it what to put in, what to cut, or shaping it for one advert?"), py_str("For most jobs one page and the recent years are enough, and the advert tells you which of your jobs it wants first. Is this starting one from nothing, or reworking one you already have?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:flat[- ]?pack\\w*|flatpack\\w*|ikea|self[- ]assembly|assembl(?:y|ing)|billy bookcase)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Flat pack goes together in the order the sheet gives, and the parts are lettered, so a part missing from the box is usually a letter you have not got to yet. Which bit - the parts, the instructions, or getting it square on the wall?"), py_str("With flat pack the sheet is the authority: parts are lettered, and the fixings are in a bag with a count on it. Is it what should be in the box, the order, or one step that is not going in?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:woodworm|wood worm|damp|fence|fencing|guttering|boiler service|loft insulation)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Damp, woodworm and a leaking gutter are usually one job with a cause above it, and a trade tells you which in one visit. Is it what it is, how to treat it yourself, or who to get in?"), py_str("The cause is usually higher up than the symptom - a gutter, a roof edge, a bridged damp course - so a photo from outside is worth taking first. Is it a job you want to do yourself, or one for somebody with the right kit?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:sourdough|yoghurt|yogurt|kefir|kombucha|kimchi|ferment\\w*|starter culture)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("A ferment is alive, so the look and the smell of it tell you more than the clock does. Is this getting one going from nothing, or one that has stopped working?"), py_str("A starter is flour and water and time, and what it does in the jar is the whole report. Is it the first few days, or the baking side of it?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:weaning|potty training|sleep training|teething|nappies|(?:baby|infant|first|stage\\s+\\d|ready[- ]made|powdered)\\s+formula|formula\\s+(?:milk|feed|feeds|feeding))(?:\\s+(?:advice|help))?\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("With a baby it is usually readiness that decides the timing, and the signs show before anything needs doing. Is this when to start, or what to do now it has started?"), py_str("Most of these come down to what the child is doing already rather than the calendar. Is it the timing you are unsure of, or something that is already not working?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:warning light|warning lights|engine light|light on the dash)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("The handbook has a page for every light on the dash and says which ones mean stop now, and a garage can read the code behind it in minutes. Which light is it, and is the car still driving normally?"), py_str("Every light on that dash has one meaning for your model, printed in the handbook, and the code behind it is what a garage plugs in to read. Is it on all the time, or only now and then?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:mot|mots|car service|bike chains?|brakes?|clutch)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("An MOT is the safety check and a service is the upkeep, and the garage's sheet lists what each one covers for your car. Is it the date, the price, or doing it at home?"), py_str("The garage's own sheet says what a check or a repair covers and what it charges, and the handbook says how often the maker wants each one. Is it whether it needs doing, what it should cost, or how long the car will be off the road?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:(?:passport|licence|license|visa)\\s+renew(?:al|als|ing)?|renew(?:al|als|ing)?\\s+(?:of\\s+)?(?:my\\s+|your\\s+|the\\s+)?(?:passport|licence|license|visa)|renew(?:ing)?\\s+(?:my\\s+|your\\s+|the\\s+)?(?:passport|licence|license|visa)|passports?|visas?|driving licences?|driving license|licences?|license|provisional)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("A renewal runs off your old document and a photo that meets the rules, and the official site takes you through the rest. Is this applying, renewing, or how long it takes?"), py_str("The form is the same one every time; what changes is the photo and what you have to send with the old document. Is it the form, the timing, or what they will want from you?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:childcare|nursery|childmind\\w*|childminder|wrap[- ]around care|after[- ]school club)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("Nursery places are booked months ahead and the inspection report for each one is public, so the visit and the report are what decide it. Is it the cost, finding a place, or what they will actually do with them?"), py_str("A childminder and a nursery are held to the same inspection, and the report says what the last visit found. Is this choosing the place, or the money side of it?")])]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:first aid|first[- ]aid|a first aid kit|resuscitat\\w+|cpr)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("First aid is the same few steps every time - danger, response, breathing, then help - and a course is where the practice comes from. Is this a situation you are in now, or getting ready for one?"), py_str("The steps are the easy part; doing them on a person is what a course gives you. Is it now, or the kit and the course?")])])]));
    s_A_SUBJECT_FILLER(call_builtin("frozenset", &[mcall(&py_str("\n    a an the my our your his her their its some any this that these those new old first second\n    help advice question info information please thanks thank re again about on for of to with\n    and or so just quick quickly bit little lot too more most starter\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUBJECT_BESIDE(call_builtin("frozenset", &[mcall(&py_str("\n    wardrobe wardrobes bookcase bookcases desk desks drawer drawers table tables chair chairs\n    bed beds shelves shelving unit units kitchen bedroom office\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_visit(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says somebody is turning up on a day.");
    if py_truthy(&Py::Bool(!py_truthy(&_has(&[text.clone(), g_VISITORS()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_has(&[text.clone(), g_A_PERSON_IN_THE_VISIT()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_VISIT_DAY(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_PAST_VISIT(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_COMING_THAT_IS_NOT_A_VISIT(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_COMING_VISIT(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_visit_family(args: &[Py]) -> Py {
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
    let _ = py_str("The pool for who is coming: a person's care, a thing in the house, or the place itself.");
    if py_truthy(&_has(&[text.clone(), g_A_CARE_VISITOR()])) {
    return g_A_VISIT_CARE();
    }
    if py_truthy(&_has(&[text.clone(), g_A_TRADE_VISITOR()])) {
    return g_A_VISIT_TRADE();
    }
    return g_A_VISIT_PLACE();
    Py::None
}
pub fn _has(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    return call_builtin("bool", &[op_bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_subject_alone(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The family for a line that is nothing but the name of a subject, else None.\n\n    Every family word is stripped out of the line first, so \"ikea wardrobe\" and \"tax return\"\n    are two subjects and not a subject with a word left over. What is left has to be filler\n    (\"my cv\", \"help with the boiler service\") or the line names a verb or a number and belongs\n    to the readings above.\n    ");
    stripped = text.clone();
    family = Py::None;
    let __iter1 = py_iter_or_raise(&g_A_SUBJECT_FAMILY());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    lines = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&pattern.clone(), "search", &[stripped.clone()])))) {
    continue;
    }
    family = { let __b = family.clone(); if py_truthy(&__b) { __b } else { lines.clone() } };
    stripped = mcall(&pattern.clone(), "sub", &[py_str(" "), stripped.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&family.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), stripped.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_A_SUBJECT_FILLER(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_SUBJECT_BESIDE(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() }) {
    return Py::None;
    }
    return family.clone();
    Py::None
}
pub fn _a_statement(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("The line when it is an account and not a question, else None.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_A_QUESTION_OPENER(), "match", &[text.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_QUESTION_ANYWHERE(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&text.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_MEDS(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_MEDS_CHANGE(), "search", &[text.clone()]) } })) } }) {
    return Py::None;
    }
    return text.clone();
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut day: Py = Py::None;
    let mut family: Py = Py::None;
    let mut line: Py = Py::None;
    let mut text: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("One thing that is true about what the line names, or None when it names nothing.");
    whole = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = whole.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&whole.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_QUESTION_OPENER(), "match", &[whole.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_QUESTION_ANYWHERE(), "search", &[whole.clone()]))) } } } } } }) {
    family = _a_subject_alone(&[whole.clone()]);
    if py_truthy(&family.clone()) {
    return family.clone();
    }
    }
    text = _a_statement(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&text.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_A_STANDING(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { _has(&[text.clone(), g_POSTURAL()]) } }; if py_truthy(&__b) { __b } else { { let __b = _has(&[text.clone(), py_tuple(vec![py_str("vision"), py_str("eyes"), py_str("sight")])]); if !py_truthy(&__b) { __b } else { mcall(&g_A_STANDING(), "search", &[text.clone()]) } } } }) {
    return g_A_STANDING_UP();
    }
    if py_truthy(&{ let __b = _has(&[text.clone(), g_WATER_THINGS()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_WATER_TROUBLE(), "search", &[text.clone()]) } }) {
    return g_WATER_IN_THE_HOUSE();
    }
    if py_truthy(&{ let __b = mcall(&g_A_PAY_OWN(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_SHORT_PAY(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_PAY_SOMEBODY_ELSE(), "search", &[text.clone()]))) } } } }) {
    return g_SHORT_PAY();
    }
    if py_truthy(&{ let __b = mcall(&g_A_BABY(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_FEED(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_FEED_FIGHT(), "search", &[text.clone()]) } } } }) {
    return g_A_FEED_FIGHT_REPLY();
    }
    if py_truthy(&{ let __b = mcall(&g_A_BABY(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_AN_AGE_UNDER_A_YEAR(), "search", &[text.clone()]) } }) {
    return g_UNDER_A_YEAR();
    }
    if py_truthy(&{ let __b = mcall(&g_A_SHIFT(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_AN_EARLY_HOUR(), "search", &[text.clone()]) } }) {
    return g_EARLY_START();
    }
    if py_truthy(&{ let __b = mcall(&g_A_MEDS(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MEDS_CHANGE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_MEDS_MINE(), "match", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_MEDS_SYMPTOM(), "search", &[text.clone()]))) } } } } } }) {
    return g_A_MEDS_CHANGE_REPLY();
    }
    if py_truthy(&_a_visit(&[text.clone()])) {
    day = mcall(&g_A_VISIT_DAY(), "search", &[text.clone()]);
    word = mcall(&mcall(&day.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_visit_family(&[text.clone()])) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), word.clone()), ("Day".to_string(), op_add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.onething".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_AGE_UNDER_A_YEAR" => Some(g_AN_AGE_UNDER_A_YEAR()),
            "AN_EARLY_HOUR" => Some(g_AN_EARLY_HOUR()),
            "A_BABY" => Some(g_A_BABY()),
            "A_CARE_VISITOR" => Some(g_A_CARE_VISITOR()),
            "A_COMING_THAT_IS_NOT_A_VISIT" => Some(g_A_COMING_THAT_IS_NOT_A_VISIT()),
            "A_COMING_VISIT" => Some(g_A_COMING_VISIT()),
            "A_FEED" => Some(g_A_FEED()),
            "A_FEED_FIGHT" => Some(g_A_FEED_FIGHT()),
            "A_FEED_FIGHT_REPLY" => Some(g_A_FEED_FIGHT_REPLY()),
            "A_MEDS" => Some(g_A_MEDS()),
            "A_MEDS_CHANGE" => Some(g_A_MEDS_CHANGE()),
            "A_MEDS_CHANGE_REPLY" => Some(g_A_MEDS_CHANGE_REPLY()),
            "A_MEDS_MINE" => Some(g_A_MEDS_MINE()),
            "A_MEDS_SYMPTOM" => Some(g_A_MEDS_SYMPTOM()),
            "A_PAST_VISIT" => Some(g_A_PAST_VISIT()),
            "A_PAY_OWN" => Some(g_A_PAY_OWN()),
            "A_PAY_SOMEBODY_ELSE" => Some(g_A_PAY_SOMEBODY_ELSE()),
            "A_PERSON_IN_THE_VISIT" => Some(g_A_PERSON_IN_THE_VISIT()),
            "A_QUESTION_ANYWHERE" => Some(g_A_QUESTION_ANYWHERE()),
            "A_QUESTION_OPENER" => Some(g_A_QUESTION_OPENER()),
            "A_SHIFT" => Some(g_A_SHIFT()),
            "A_SHORT_PAY" => Some(g_A_SHORT_PAY()),
            "A_STANDING" => Some(g_A_STANDING()),
            "A_STANDING_UP" => Some(g_A_STANDING_UP()),
            "A_SUBJECT_BESIDE" => Some(g_A_SUBJECT_BESIDE()),
            "A_SUBJECT_FAMILY" => Some(g_A_SUBJECT_FAMILY()),
            "A_SUBJECT_FILLER" => Some(g_A_SUBJECT_FILLER()),
            "A_TRADE_VISITOR" => Some(g_A_TRADE_VISITOR()),
            "A_VISIT_CARE" => Some(g_A_VISIT_CARE()),
            "A_VISIT_DAY" => Some(g_A_VISIT_DAY()),
            "A_VISIT_PLACE" => Some(g_A_VISIT_PLACE()),
            "A_VISIT_TRADE" => Some(g_A_VISIT_TRADE()),
            "A_WATER_TROUBLE" => Some(g_A_WATER_TROUBLE()),
            "EARLY_START" => Some(g_EARLY_START()),
            "POSTURAL" => Some(g_POSTURAL()),
            "SHORT_PAY" => Some(g_SHORT_PAY()),
            "UNDER_A_YEAR" => Some(g_UNDER_A_YEAR()),
            "VISITORS" => Some(g_VISITORS()),
            "WATER_IN_THE_HOUSE" => Some(g_WATER_IN_THE_HOUSE()),
            "WATER_THINGS" => Some(g_WATER_THINGS()),
            "_a_statement" => Some(py_func("_a_statement", |args: &[Py]| _a_statement(args))),
            "_a_subject_alone" => Some(py_func("_a_subject_alone", |args: &[Py]| _a_subject_alone(args))),
            "_a_visit" => Some(py_func("_a_visit", |args: &[Py]| _a_visit(args))),
            "_has" => Some(py_func("_has", |args: &[Py]| _has(args))),
            "_the_visit_family" => Some(py_func("_the_visit_family", |args: &[Py]| _the_visit_family(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_AGE_UNDER_A_YEAR" => s_AN_AGE_UNDER_A_YEAR(v),
            "AN_EARLY_HOUR" => s_AN_EARLY_HOUR(v),
            "A_BABY" => s_A_BABY(v),
            "A_CARE_VISITOR" => s_A_CARE_VISITOR(v),
            "A_COMING_THAT_IS_NOT_A_VISIT" => s_A_COMING_THAT_IS_NOT_A_VISIT(v),
            "A_COMING_VISIT" => s_A_COMING_VISIT(v),
            "A_FEED" => s_A_FEED(v),
            "A_FEED_FIGHT" => s_A_FEED_FIGHT(v),
            "A_FEED_FIGHT_REPLY" => s_A_FEED_FIGHT_REPLY(v),
            "A_MEDS" => s_A_MEDS(v),
            "A_MEDS_CHANGE" => s_A_MEDS_CHANGE(v),
            "A_MEDS_CHANGE_REPLY" => s_A_MEDS_CHANGE_REPLY(v),
            "A_MEDS_MINE" => s_A_MEDS_MINE(v),
            "A_MEDS_SYMPTOM" => s_A_MEDS_SYMPTOM(v),
            "A_PAST_VISIT" => s_A_PAST_VISIT(v),
            "A_PAY_OWN" => s_A_PAY_OWN(v),
            "A_PAY_SOMEBODY_ELSE" => s_A_PAY_SOMEBODY_ELSE(v),
            "A_PERSON_IN_THE_VISIT" => s_A_PERSON_IN_THE_VISIT(v),
            "A_QUESTION_ANYWHERE" => s_A_QUESTION_ANYWHERE(v),
            "A_QUESTION_OPENER" => s_A_QUESTION_OPENER(v),
            "A_SHIFT" => s_A_SHIFT(v),
            "A_SHORT_PAY" => s_A_SHORT_PAY(v),
            "A_STANDING" => s_A_STANDING(v),
            "A_STANDING_UP" => s_A_STANDING_UP(v),
            "A_SUBJECT_BESIDE" => s_A_SUBJECT_BESIDE(v),
            "A_SUBJECT_FAMILY" => s_A_SUBJECT_FAMILY(v),
            "A_SUBJECT_FILLER" => s_A_SUBJECT_FILLER(v),
            "A_TRADE_VISITOR" => s_A_TRADE_VISITOR(v),
            "A_VISIT_CARE" => s_A_VISIT_CARE(v),
            "A_VISIT_DAY" => s_A_VISIT_DAY(v),
            "A_VISIT_PLACE" => s_A_VISIT_PLACE(v),
            "A_VISIT_TRADE" => s_A_VISIT_TRADE(v),
            "A_WATER_TROUBLE" => s_A_WATER_TROUBLE(v),
            "EARLY_START" => s_EARLY_START(v),
            "POSTURAL" => s_POSTURAL(v),
            "SHORT_PAY" => s_SHORT_PAY(v),
            "UNDER_A_YEAR" => s_UNDER_A_YEAR(v),
            "VISITORS" => s_VISITORS(v),
            "WATER_IN_THE_HOUSE" => s_WATER_IN_THE_HOUSE(v),
            "WATER_THINGS" => s_WATER_THINGS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.onething", module_obj);
    pyrt::xrefs::reg_fn("compose.onething", "_a_visit", py_func("_a_visit", |args: &[Py]| _a_visit(args)));
    pyrt::xrefs::reg_fn("compose.onething", "_the_visit_family", py_func("_the_visit_family", |args: &[Py]| _the_visit_family(args)));
    pyrt::xrefs::reg_fn("compose.onething", "_has", py_func("_has", |args: &[Py]| _has(args)));
    pyrt::xrefs::reg_fn("compose.onething", "_a_subject_alone", py_func("_a_subject_alone", |args: &[Py]| _a_subject_alone(args)));
    pyrt::xrefs::reg_fn("compose.onething", "_a_statement", py_func("_a_statement", |args: &[Py]| _a_statement(args)));
    pyrt::xrefs::reg_fn("compose.onething", "pool", py_func("pool", |args: &[Py]| pool(args)));
    pyrt::xrefs::reg_access("compose.onething", "AN_AGE_UNDER_A_YEAR", g_AN_AGE_UNDER_A_YEAR, s_AN_AGE_UNDER_A_YEAR);
    pyrt::xrefs::reg_access("compose.onething", "AN_EARLY_HOUR", g_AN_EARLY_HOUR, s_AN_EARLY_HOUR);
    pyrt::xrefs::reg_access("compose.onething", "A_BABY", g_A_BABY, s_A_BABY);
    pyrt::xrefs::reg_access("compose.onething", "A_CARE_VISITOR", g_A_CARE_VISITOR, s_A_CARE_VISITOR);
    pyrt::xrefs::reg_access("compose.onething", "A_COMING_THAT_IS_NOT_A_VISIT", g_A_COMING_THAT_IS_NOT_A_VISIT, s_A_COMING_THAT_IS_NOT_A_VISIT);
    pyrt::xrefs::reg_access("compose.onething", "A_COMING_VISIT", g_A_COMING_VISIT, s_A_COMING_VISIT);
    pyrt::xrefs::reg_access("compose.onething", "A_FEED", g_A_FEED, s_A_FEED);
    pyrt::xrefs::reg_access("compose.onething", "A_FEED_FIGHT", g_A_FEED_FIGHT, s_A_FEED_FIGHT);
    pyrt::xrefs::reg_access("compose.onething", "A_FEED_FIGHT_REPLY", g_A_FEED_FIGHT_REPLY, s_A_FEED_FIGHT_REPLY);
    pyrt::xrefs::reg_access("compose.onething", "A_MEDS", g_A_MEDS, s_A_MEDS);
    pyrt::xrefs::reg_access("compose.onething", "A_MEDS_CHANGE", g_A_MEDS_CHANGE, s_A_MEDS_CHANGE);
    pyrt::xrefs::reg_access("compose.onething", "A_MEDS_CHANGE_REPLY", g_A_MEDS_CHANGE_REPLY, s_A_MEDS_CHANGE_REPLY);
    pyrt::xrefs::reg_access("compose.onething", "A_MEDS_MINE", g_A_MEDS_MINE, s_A_MEDS_MINE);
    pyrt::xrefs::reg_access("compose.onething", "A_MEDS_SYMPTOM", g_A_MEDS_SYMPTOM, s_A_MEDS_SYMPTOM);
    pyrt::xrefs::reg_access("compose.onething", "A_PAST_VISIT", g_A_PAST_VISIT, s_A_PAST_VISIT);
    pyrt::xrefs::reg_access("compose.onething", "A_PAY_OWN", g_A_PAY_OWN, s_A_PAY_OWN);
    pyrt::xrefs::reg_access("compose.onething", "A_PAY_SOMEBODY_ELSE", g_A_PAY_SOMEBODY_ELSE, s_A_PAY_SOMEBODY_ELSE);
    pyrt::xrefs::reg_access("compose.onething", "A_PERSON_IN_THE_VISIT", g_A_PERSON_IN_THE_VISIT, s_A_PERSON_IN_THE_VISIT);
    pyrt::xrefs::reg_access("compose.onething", "A_QUESTION_ANYWHERE", g_A_QUESTION_ANYWHERE, s_A_QUESTION_ANYWHERE);
    pyrt::xrefs::reg_access("compose.onething", "A_QUESTION_OPENER", g_A_QUESTION_OPENER, s_A_QUESTION_OPENER);
    pyrt::xrefs::reg_access("compose.onething", "A_SHIFT", g_A_SHIFT, s_A_SHIFT);
    pyrt::xrefs::reg_access("compose.onething", "A_SHORT_PAY", g_A_SHORT_PAY, s_A_SHORT_PAY);
    pyrt::xrefs::reg_access("compose.onething", "A_STANDING", g_A_STANDING, s_A_STANDING);
    pyrt::xrefs::reg_access("compose.onething", "A_STANDING_UP", g_A_STANDING_UP, s_A_STANDING_UP);
    pyrt::xrefs::reg_access("compose.onething", "A_SUBJECT_BESIDE", g_A_SUBJECT_BESIDE, s_A_SUBJECT_BESIDE);
    pyrt::xrefs::reg_access("compose.onething", "A_SUBJECT_FAMILY", g_A_SUBJECT_FAMILY, s_A_SUBJECT_FAMILY);
    pyrt::xrefs::reg_access("compose.onething", "A_SUBJECT_FILLER", g_A_SUBJECT_FILLER, s_A_SUBJECT_FILLER);
    pyrt::xrefs::reg_access("compose.onething", "A_TRADE_VISITOR", g_A_TRADE_VISITOR, s_A_TRADE_VISITOR);
    pyrt::xrefs::reg_access("compose.onething", "A_VISIT_CARE", g_A_VISIT_CARE, s_A_VISIT_CARE);
    pyrt::xrefs::reg_access("compose.onething", "A_VISIT_DAY", g_A_VISIT_DAY, s_A_VISIT_DAY);
    pyrt::xrefs::reg_access("compose.onething", "A_VISIT_PLACE", g_A_VISIT_PLACE, s_A_VISIT_PLACE);
    pyrt::xrefs::reg_access("compose.onething", "A_VISIT_TRADE", g_A_VISIT_TRADE, s_A_VISIT_TRADE);
    pyrt::xrefs::reg_access("compose.onething", "A_WATER_TROUBLE", g_A_WATER_TROUBLE, s_A_WATER_TROUBLE);
    pyrt::xrefs::reg_access("compose.onething", "EARLY_START", g_EARLY_START, s_EARLY_START);
    pyrt::xrefs::reg_access("compose.onething", "POSTURAL", g_POSTURAL, s_POSTURAL);
    pyrt::xrefs::reg_access("compose.onething", "SHORT_PAY", g_SHORT_PAY, s_SHORT_PAY);
    pyrt::xrefs::reg_access("compose.onething", "UNDER_A_YEAR", g_UNDER_A_YEAR, s_UNDER_A_YEAR);
    pyrt::xrefs::reg_access("compose.onething", "VISITORS", g_VISITORS, s_VISITORS);
    pyrt::xrefs::reg_access("compose.onething", "WATER_IN_THE_HOUSE", g_WATER_IN_THE_HOUSE, s_WATER_IN_THE_HOUSE);
    pyrt::xrefs::reg_access("compose.onething", "WATER_THINGS", g_WATER_THINGS, s_WATER_THINGS);
}
