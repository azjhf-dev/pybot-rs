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


thread_local! { static G_AGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_AGE_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_OF_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HABIT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PER_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SIZE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLOUR_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRACTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_COST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_WITH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_WITH_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFTEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OF_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAY_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SALARY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIZE_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIZE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIZE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLOT_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES_AS_IT_COMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_SPAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VALUE_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEIGHT_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHATS_MY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AGE_ASK() -> Py { ensure_init(); G_AGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_ASK(v: Py) { ensure_init(); G_AGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_VALUE() -> Py { ensure_init(); G_AGE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_VALUE(v: Py) { ensure_init(); G_AGE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_AGE_CLAUSE() -> Py { ensure_init(); G_AN_AGE_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AGE_CLAUSE(v: Py) { ensure_init(); G_AN_AGE_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_OF_PEOPLE() -> Py { ensure_init(); G_A_COUNT_OF_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_OF_PEOPLE(v: Py) { ensure_init(); G_A_COUNT_OF_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HABIT_SAID() -> Py { ensure_init(); G_A_HABIT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HABIT_SAID(v: Py) { ensure_init(); G_A_HABIT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY() -> Py { ensure_init(); G_A_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY(v: Py) { ensure_init(); G_A_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PER_YEAR() -> Py { ensure_init(); G_A_PER_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PER_YEAR(v: Py) { ensure_init(); G_A_PER_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SIZE() -> Py { ensure_init(); G_A_SIZE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SIZE(v: Py) { ensure_init(); G_A_SIZE.with(|c| *c.borrow_mut() = v); }
pub fn g_COLOUR_VALUE() -> Py { ensure_init(); G_COLOUR_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLOUR_VALUE(v: Py) { ensure_init(); G_COLOUR_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRACTIONS() -> Py { ensure_init(); G_CONTRACTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRACTIONS(v: Py) { ensure_init(); G_CONTRACTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_COST_ASK() -> Py { ensure_init(); G_COST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COST_ASK(v: Py) { ensure_init(); G_COST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER() -> Py { ensure_init(); G_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); G_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_KINDS() -> Py { ensure_init(); G_HOW_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_KINDS(v: Py) { ensure_init(); G_HOW_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_VALUE() -> Py { ensure_init(); G_LENGTH_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_VALUE(v: Py) { ensure_init(); G_LENGTH_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_ASK() -> Py { ensure_init(); G_MONEY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_ASK(v: Py) { ensure_init(); G_MONEY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_QUALIFIER() -> Py { ensure_init(); G_MONEY_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_QUALIFIER(v: Py) { ensure_init(); G_MONEY_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_WORDS() -> Py { ensure_init(); G_MONEY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_WORDS(v: Py) { ensure_init(); G_MONEY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_COST() -> Py { ensure_init(); G_NOT_A_COST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_COST(v: Py) { ensure_init(); G_NOT_A_COST.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_WITH_ASK() -> Py { ensure_init(); G_OFF_WITH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_WITH_ASK(v: Py) { ensure_init(); G_OFF_WITH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_WITH_SAID() -> Py { ensure_init(); G_OFF_WITH_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_WITH_SAID(v: Py) { ensure_init(); G_OFF_WITH_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_OFTEN_ASK() -> Py { ensure_init(); G_OFTEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFTEN_ASK(v: Py) { ensure_init(); G_OFTEN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OF_MINE() -> Py { ensure_init(); G_OF_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OF_MINE(v: Py) { ensure_init(); G_OF_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAY_ASK() -> Py { ensure_init(); G_PAY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAY_ASK(v: Py) { ensure_init(); G_PAY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PAY_ASK_WORDS() -> Py { ensure_init(); G_PAY_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAY_ASK_WORDS(v: Py) { ensure_init(); G_PAY_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE() -> Py { ensure_init(); G_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE(v: Py) { ensure_init(); G_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_LINE() -> Py { ensure_init(); G_PRONOUN_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_LINE(v: Py) { ensure_init(); G_PRONOUN_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_WORDS() -> Py { ensure_init(); G_QUESTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_WORDS(v: Py) { ensure_init(); G_QUESTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_ASK() -> Py { ensure_init(); G_SAID_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_ASK(v: Py) { ensure_init(); G_SAID_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SALARY_WORDS() -> Py { ensure_init(); G_SALARY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SALARY_WORDS(v: Py) { ensure_init(); G_SALARY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SIZE_ADVICE() -> Py { ensure_init(); G_SIZE_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIZE_ADVICE(v: Py) { ensure_init(); G_SIZE_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_SIZE_ASK() -> Py { ensure_init(); G_SIZE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIZE_ASK(v: Py) { ensure_init(); G_SIZE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SIZE_VALUE() -> Py { ensure_init(); G_SIZE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIZE_VALUE(v: Py) { ensure_init(); G_SIZE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_SLOT_STOP() -> Py { ensure_init(); G_SLOT_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLOT_STOP(v: Py) { ensure_init(); G_SLOT_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEBODY() -> Py { ensure_init(); G_SOMEBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEBODY(v: Py) { ensure_init(); G_SOMEBODY.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS() -> Py { ensure_init(); G_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS(v: Py) { ensure_init(); G_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_QUESTION() -> Py { ensure_init(); G_THEIR_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_QUESTION(v: Py) { ensure_init(); G_THEIR_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_SPAN() -> Py { ensure_init(); G_TIME_SPAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_SPAN(v: Py) { ensure_init(); G_TIME_SPAN.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WORDS() -> Py { ensure_init(); G_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WORDS(v: Py) { ensure_init(); G_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VALUE_KINDS() -> Py { ensure_init(); G_VALUE_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VALUE_KINDS(v: Py) { ensure_init(); G_VALUE_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAYS() -> Py { ensure_init(); G_WEEKDAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAYS(v: Py) { ensure_init(); G_WEEKDAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEIGHT_VALUE() -> Py { ensure_init(); G_WEIGHT_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_VALUE(v: Py) { ensure_init(); G_WEIGHT_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHATS_MY() -> Py { ensure_init(); G_WHATS_MY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHATS_MY(v: Py) { ensure_init(); G_WHATS_MY.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ASK() -> Py { ensure_init(); G_WHEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASK(v: Py) { ensure_init(); G_WHEN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_ASK() -> Py { ensure_init(); G_WHERE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_ASK(v: Py) { ensure_init(); G_WHERE_ASK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A fact about themselves they gave, asked for again by name, and answered from their own line.\n\n    im looking for a new winter coat\n    im a size 12\n    my budget is about 100 pounds\n    ...\n    what size am i again    -> \"I don't know enough to answer that honestly. A search ...\"\n    and whats my budget     -> \"Your budget is 100 pounds.\"\n\nThe store has the size the whole time - on the same chat `do you know my size` answers\n\"You said you're a size 12.\" - so this is one ask shape reaching nobody, not a fact that was\nnever kept. The ask is about the person, and the attribute words (\"size\", \"weight\", \"colour\")\nare not words in the line that gives them (\"im a size 12\", \"90 kilos\"), so no entry about the\nworld can look it up and no reader of what was said was asked.\n\nThe reading here is narrow on purpose, and every part of it is needed:\n\n* the ask has to be about them - `what <attribute> am i`, `whats my <attribute>` - so\n  \"what size are the boxes\" and \"what time am i leaving\" are untouched;\n* the attribute has to be one of the short list below, so a line about anything else passes;\n* **and the attribute has to be in one of their own lines.** That is what makes it a recall\n  rather than a question about the world: `what size am i` with nothing said before it is\n  still the size-conversion card, and `what size am i again` after \"my knee hurts\" finds\n  nothing and the turn stays where it was.\n\nThe wording is the store's own restating (`memory_store._told_phrase`), so the fact comes back\nin the shape a friend would say it - \"You told me you're a size 12.\" - and never as a quotation\nof the line.\n\nThe same shape of ask, with the slot named instead of the fact, is answered here too:\n\n    we are going to edinburgh for a long weekend in october\n    we want to be near the royal mile\n    ...\n    did i say when we were going        -> \"I don't know enough to answer that honestly.\"\n    did i say where we wanted to stay   -> \"That's beyond what I know well, sorry.\"\n\nThe month is in their first line and the area in their second, and the store answers\n\"what did i say about october\" from the same lines. What is missing is the ask that names the\nslot: `did i say when ...`, `when did i say ...`, and the same with \"where\". The slot word says\nwhat kind of value is wanted, and the rest of the ask names the thing it belongs to - so the\nline has to hold both a time (for when) or a place (for where) **and** share a word with the\nsubject of the ask. Nothing else is read: \"did i say when the boiler was serviced\" with no time\nsaid anywhere passes, and so does every line that is not one of these asks.\n\nThe same shape again, with a size instead of a time or a place:\n\n    we're redecorating the kitchen next month\n    its small, about 3 by 2 metres\n    ...\n    and how big is the kitchen     -> \"Clear the surfaces, then work clockwise ...\" (a cleaning card)\n\nThe size is in their second line, said about the kitchen it does not name, and the ask for it\nreached a card about cleaning. A measure is read here only with a unit (metres, cm, feet, litres)\nso a count (\"two cats\") and a price (\"20 pounds\") are not sizes, and the line has to be about the\nthing the ask names - or be a line about it alone (\"its small, about 3 by 2 metres\") directly\nafter the line that does name it.\n");
    s_PRIORITY(Py::Int(6900));
    fn __data1() -> Py { py_tuple(vec![py_str("size"), py_str("shoe size"), py_str("ring size"), py_str("weight"), py_str("height"), py_str("colour"), py_str("color"), py_str("budget"), py_str("age"), py_str("brand"), py_str("make"), py_str("model"), py_str("number"), py_str("plate"), py_str("breed"), py_str("name")]) }
    s_THEIRS(__data1());
    s_QUESTION_WORDS(call_builtin("frozenset", &[mcall(&py_str("best better option options chance chances odds next plan plans point\n    problem issue deal story time day week month year"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bwhat(?:\\s+is|\\s*'s|s)?\\s+(?P<attr>"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\s+(?:am\\s+i)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OF_MINE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bwhat(?:\\s+is|\\s*'s|s)?\\s+my\\s+(?P<attr>"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THEIRS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FILLER(call_builtin("frozenset", &[mcall(&py_str("again now then today still also please again? so and but ok okay oh"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("(?:[\\u00a3$\\u20ac]\\s*\\d[\\d,.]*|\\b\\d[\\d,.]*\\s*(?:a|per|each)\\s+(?:month|week|year|fortnight|day|week|month)\\b|\\b\\d[\\d,.]*\\s*(?:quid|pounds?|dollars?|euros?|pm)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_COST(stdlib_call("re", "compile", &[py_str("\\b(?:earn|earns|earned|earning|salary|wage|wages|take[- ]home|save|saves|saved|saving|work|works|worked|working|shift|shifts|eat|eats|ate|eating|drink|drinks|drank|drinking|run|runs|ran|running|walk|walks|walked|walking|swim|swims|swam|swimming|sleep|sleeps|slept|sleeping|study|studies|studied|studying|read|reads|reading)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|how\\s+much)\\b[^.!?]*?\\b(?:am|was|are|were|do|did)\\s+i\\b[^.!?]*?\\b(?:pay|paying|paid|cost|costs|costing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COST_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|so\\s+|ok\\s+|okay\\s+)?how\\s+much\\s+(?:is|was)\\s+it\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHATS_MY(stdlib_call("re", "compile", &[py_str("\\bwhat(?:\\s+is|\\s*'s|s)?\\s+my\\s+(?P<attr>[a-z][a-z' -]{1,24}?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRACTIONS(py_tuple(vec![py_tuple(vec![py_str("you are"), py_str("you're")]), py_tuple(vec![py_str("you have"), py_str("you've")]), py_tuple(vec![py_str("it is"), py_str("it's")]), py_tuple(vec![py_str("that is"), py_str("that's")]), py_tuple(vec![py_str("i am"), py_str("i'm")])]));
    s_WHEN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:did|do)\\s+(?:i|we)\\s+say\\s+when\\b(?P<rest>.*)|\\bwhen\\s+did\\s+(?:i|we)\\s+say\\b(?P<rest2>.*)|\\bwhen\\s+(?:was|is)\\s+it\\s+(?:that\\s+)?(?:i|we)\\s+said\\b(?P<rest3>.*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:did|do)\\s+(?:i|we)\\s+say\\s+where\\b(?P<rest>.*)|\\bwhere\\s+did\\s+(?:i|we)\\s+say\\b(?P<rest2>.*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTHS(mcall(&py_str("january february march april may june july august september october november december"), "split", &[]));
    s_WEEKDAYS(mcall(&py_str("monday tuesday wednesday thursday friday saturday sunday"), "split", &[]));
    s_TIME_WORDS(call_builtin("frozenset", &[op_add(&op_add(&g_MONTHS(), &g_WEEKDAYS()), &mcall(&py_str("weekend tonight tomorrow today morning\nafternoon evening night midday noon christmas easter summer winter spring autumn"), "split", &[]))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_SPAN(stdlib_call("re", "compile", &[py_str("\\b(?:next|last|this)\\s+(?:week|month|year|summer|winter|spring|autumn)\\b|\\bin\\s+(?:about\\s+)?\\d+\\s+(?:days?|weeks?|months?|years?)\\b|\\b\\d{1,2}(?:[.:]\\d\\d)?\\s*(?:am|pm)\\b|\\bat\\s+\\d{1,2}\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:near|next to|close to|round the corner from|opposite)\\s+(?P<place>[a-z][a-z'\\u2019]*(?:\\s+[a-z][a-z'\\u2019]*){0,3})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIZE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+(?:big|large|wide|long|tall|high|deep)|what\\s+(?:size|dimensions?|measurements?)|how\\s+many\\s+(?:metres|meters|feet|foot|ft|centimetres|centimeters|cm|square\\s+(?:metres|meters|feet)))\\b(?P<rest>[^?.!]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIZE_ADVICE(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|would|could|can|might|will|need|needs|enough|recommend|best)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SIZE(stdlib_call("re", "compile", &[py_str("(?P<measure>\\b\\d+(?:[.,]\\d+)?\\s*(?:by|x|\\u00d7)\\s*\\d+(?:[.,]\\d+)?\\s*(?:m|metres?|meters?|ft|feet|foot|cm|centimetres?|centimeters?|inches|inch)\\b|\\b\\d+(?:[.,]\\d+)?\\s*(?:square|sq)\\s*(?:metres?|meters?|feet|foot|ft|m)\\b|\\b\\d+(?:[.,]\\d+)?\\s*(?:m|metres?|meters?|cm|centimetres?|centimeters?|ft|feet|foot|inches|inch|litres?|liters?|ml)\\b)(?P<after>\\s+[a-z][a-z'-]*)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SALARY_WORDS(py_tuple(vec![py_str("salary"), py_str("pay"), py_str("wage"), py_str("wages"), py_str("income"), py_str("earnings")]));
    s_MONEY_WORDS(op_add(&g_SALARY_WORDS(), &py_tuple(vec![py_str("budget"), py_str("price"), py_str("cost"), py_str("rent"), py_str("savings"), py_str("deposit")])));
    fn __data2() -> Py { py_tuple(vec![py_str("current"), py_str("currently"), py_str("annual"), py_str("annually"), py_str("monthly"), py_str("weekly"), py_str("hourly"), py_str("yearly"), py_str("real"), py_str("actual"), py_str("actually"), py_str("basic"), py_str("gross"), py_str("net"), py_str("total"), py_str("usual"), py_str("normal"), py_str("present"), py_str("own")]) }
    s_MONEY_QUALIFIER(__data2());
    s_MONEY_ASK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\bwhat(?:'s| is| was|s)?\\s+(?:my|our|their|his|her)\\s+(?:(?:"), &mcall(&py_str("|"), "join", &[g_MONEY_QUALIFIER()])), &py_str(")\\s+)?(?P<what>")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MONEY_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b|\\bhow much\\s+(?:was|is|were|did\\s+(?:i|we)\\s+say)\\s+(?:my|our|the|their|his|her)\\s+(?:(?:")), &mcall(&py_str("|"), "join", &[g_MONEY_QUALIFIER()])), &py_str(")\\s+)?(?P<what2>")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MONEY_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PER_YEAR(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:\\.\\d+)?\\s*k\\b(?:\\s*(?:a|per)\\s*(?:year|annum))?|\\u00a3?\\s?\\d[\\d,]{3,}\\s*(?:a|per)\\s*(?:year|annum)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY(stdlib_call("re", "compile", &[py_str("(?:\\u00a3\\s?\\d[\\d,]*(?:\\.\\d\\d)?|\\b\\d[\\d,]*(?:\\.\\d+)?\\s*(?:pounds?|quid|dollars?|euros?|grand)\\b|\\b\\d[\\d,]{2,}\\b)(?:\\s*(?:a|per)\\s*(?:month|week|year|fortnight))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SLOT_STOP(call_builtin("frozenset", &[mcall(&py_str("i we you me us my our it its the a an to be been being was were are is\nam do did does say said about that this of for in on at and but so then just again please oh ok\nokay actually"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|how\\s+old)\\b[^.?!]{0,40}?\\b(?:did|do)\\s+i\\s+(?:say|tell\\s+you|told\\s+you|think|thought|mention|mentioned)\\b[^.?!]{0,30}?\\b(?:age|old)\\b|\\bwhat\\s+(?:was|is)\\s+(?P<whose>his|her|their|my|our|its)\\s+age\\b|\\bhow\\s+old\\s+(?:did|do)\\s+i\\s+(?:say|tell\\s+you|told\\s+you|think|mention)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OFTEN_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+often\\b[^.?!]{0,30}?\\b(?:did|do)\\s+i\\s+(?:say|tell\\s+you|told\\s+you|mention)\\b|\\bhow\\s+many\\s+times\\b[^.?!]{0,30}?\\b(?:did|do)\\s+i\\s+(?:say|tell\\s+you|mention)\\b|\\bhow\\s+often\\s+(?:was|is)\\s+it\\b[^.?!]{0,12}?\\bagain\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AGE_CLAUSE(stdlib_call("re", "compile", &[py_str("(?:^|[,;]|\\bis\\b|\\bwas\\b|\\bim\\b|\\bi'm\\b|\\bam\\b|\\b-\\s*)\\s*(?P<n>\\d{1,3})\\s*(?=$|[,.;!?]|\\s+and\\b|\\s+but\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEBODY(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|me|myself|he|she|him|her|his|they|them|their|my|our|son|daughter|lad|girl|boy|kid|kids|baby|toddler|mum|mom|dad|husband|wife|partner|brother|sister|nan|gran|grandad|dog|cat|pup|puppy|labrador|retriever|terrier|shepherd|spaniel|collie|hound|mutt|rabbit|hamster|horse)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COUNT_OF_PEOPLE(stdlib_call("re", "compile", &[py_str("\\b\\d+\\s+(?:of\\s+us|people|adults)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HABIT_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:(?:once|twice|three|four|five|six|seven|eight|nine|ten|\\d+)\\s+times?\\s+(?:a|per|each)\\s+(?:day|week|month|year)|(?:once|twice)\\s+(?:a|per)\\s+(?:day|week|month|year)|every\\s+(?:day|week|weekend|morning|evening|night)|most\\s+(?:days|weeks|mornings|evenings)|\\bdaily\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN_LINE(stdlib_call("re", "compile", &[py_str("^\\s*(?:its|it is|it's|that's|thats|theres|there's|theyre|they are|hes|shes|were|we are)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_ASK(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|so\\s+|but\\s+|also\\s+|oh\\s+|ok\\s+|okay\\s+)?(?:(?:what|which)\\s+(?P<attr>[a-z][a-z' -]{1,20}?)\\s+(?:did|do|have)\\s+(?:i|we)\\s+(?:say|give|mention|tell\\s+you)|how\\s+(?P<how>[a-z]+)\\s+did\\s+(?:i|we)\\s+say)\\b[^?.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SIZE_VALUE(stdlib_call("re", "compile", &[py_str("(?:(?<![\\w'])(?:xs|s|m|l|xl|xxl|xxxl)(?![\\w'])|\\b(?:small|medium|large|extra\\s+large|size\\s+\\d{1,2}|(?:a\\s+)?\\d{1,2}\\s*(?:-|to)\\s*\\d{1,2})\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_VALUE(stdlib_call("re", "compile", &[py_str("\\b(?:long|short|shoulder[- ]length|past\\s+(?:my\\s+)?shoulders|bob|crop|pixie|just\\s+(?:above|below)\\s+\\w+|\\d+\\s*(?:cm|inches|inch))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEIGHT_VALUE(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:[.,]\\d+)?\\s*(?:kg|kilo|kilos|kilograms|st|stone|stones|lbs?|pounds)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_VALUE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d{1,2}\\s*(?:years?\\s*old|yr|yrs)?|aged\\s+\\d{1,2})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COLOUR_VALUE(stdlib_call("re", "compile", &[py_str("\\b(?:red|blue|green|black|white|grey|gray|brown|beige|cream|navy|pink|purple|orange|yellow|gold|silver|teal|turquoise|burgundy|maroon|khaki|olive|charcoal)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VALUE_KINDS(py_dict(vec![(py_str("size"), g_SIZE_VALUE()), (py_str("shoe size"), g_SIZE_VALUE()), (py_str("ring size"), g_SIZE_VALUE()), (py_str("colour"), g_COLOUR_VALUE()), (py_str("color"), g_COLOUR_VALUE()), (py_str("weight"), g_WEIGHT_VALUE()), (py_str("age"), g_AGE_VALUE()), (py_str("length"), g_LENGTH_VALUE()), (py_str("hair"), g_LENGTH_VALUE())]));
    s_HOW_KINDS(py_dict(vec![(py_str("short"), g_LENGTH_VALUE()), (py_str("long"), g_LENGTH_VALUE()), (py_str("tall"), g_LENGTH_VALUE()), (py_str("big"), g_SIZE_VALUE()), (py_str("large"), g_SIZE_VALUE()), (py_str("small"), g_SIZE_VALUE()), (py_str("heavy"), g_WEIGHT_VALUE()), (py_str("old"), g_AGE_VALUE())]));
    s_THEIR_QUESTION(stdlib_call("re", "compile", &[py_str("^\\W*(?:what|whats|what's|how|when|where|which|who|why|can|could|should|shall|would|will|does|do|did|is|are|was|were|have|has|any|anything|tell\\s+me|remind\\s+me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_WITH_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:was|is)\\s+i\\s+off\\b|\\bwhat\\s+did\\s+i\\s+say\\s+i\\s+was\\s+off\\b|\\bwhat\\s+was\\s+i\\s+off\\s+(?:sick\\s+)?with\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_WITH_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:off\\s+(?:sick|work|ill)|been\\s+off|signed\\s+off)\\b[^.?!]*?\\bwith\\s+(?P<thing>an?\\s+)?(?P<what>[a-z][a-z' -]{3,30})\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("much"), py_str("many"), py_str("do"), py_str("did"), py_str("does"), py_str("i"), py_str("im"), py_str("my"), py_str("the"), py_str("a"), py_str("an"), py_str("is"), py_str("was"), py_str("are"), py_str("were"), py_str("for"), py_str("of"), py_str("to"), py_str("in"), py_str("on"), py_str("at"), py_str("it"), py_str("that"), py_str("this"), py_str("me"), py_str("you"), py_str("your"), py_str("get"), py_str("got"), py_str("getting"), py_str("pay"), py_str("paid"), py_str("pays"), py_str("paying"), py_str("payment"), py_str("payments"), py_str("wage"), py_str("wages"), py_str("salary"), py_str("salaries"), py_str("earn"), py_str("earns"), py_str("earned"), py_str("earning"), py_str("make"), py_str("makes"), py_str("made"), py_str("making"), py_str("take"), py_str("home"), py_str("takehome"), py_str("and"), py_str("or"), py_str("per"), py_str("plus"), py_str("now"), py_str("again"), py_str("current"), py_str("currently"), py_str("exact"), py_str("exactly"), py_str("actual"), py_str("actually"), py_str("latest"), py_str("today"), py_str("so"), py_str("far"), py_str("have"), py_str("has"), py_str("had"), py_str("been"), py_str("like"), py_str("think"), py_str("about"), py_str("guess"), py_str("roughly"), py_str("all"), py_str("just"), py_str("really"), py_str("total"), py_str("figure"), py_str("number"), py_str("amount"), py_str("sum"), py_str("much"), py_str("there"), py_str("then")]) }
    s_PAY_ASK_WORDS(call_builtin("frozenset", &[__data3()]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKES_AS_IT_COMES(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
}
pub fn _their_attribute(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The attribute they are asking after, or \"\" when the line is not that ask.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_ASK(), g_OF_MINE()]));
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "search", &[low.clone()]);
    if py_truthy(&match_.clone()) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[py_str("attr")]), "lower", &[]), "split", &[])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _stem(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut suffix: Py = Py::None;
    let _ = py_str("The word with a plural or a participle taken off, so \"colour\" and \"colouring\" match.");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str("'")]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    suffix = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[suffix.clone()]) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return word.clone();
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
    let mut tokens: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tokens") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tokens"))) };
    let mut attribute: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "attribute") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: attribute"))) };
    let mut stems: Py = Py::None;
    let mut token: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when every word of the attribute is one of these tokens, in any of its forms.");
    stems = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tokens.clone()) { let mut token: Py = __item.clone(); __out.push(_stem(&[token.clone()])); } py_set(__out) })() };
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&attribute.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&stems.clone(), &_stem(&[word.clone()])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_phrase_of_theirs(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut attribute: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "attribute") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: attribute"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut needs_a_figure: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "needs_a_figure") { v.clone() }
    else { Py::Bool(false) };
    let mut line: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The newest thing they said about this attribute, said back, or \"\".\n\n    Their own asks are skipped: \"what size am i\" a turn earlier has the word in it and states\n    nothing, and reading it back would answer the question with the question. The first line\n    that states a fact with the attribute in it is the one answered from.\n\n    `needs_a_figure` is for the ask that wants a number: the line only answers it when it holds\n    one, so a line about the thing and nothing else passes.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = { let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names(&[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&line.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), attribute.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = needs_a_figure.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_FIGURE(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_COST(), "search", &[line.clone()]))) } })) } }) {
    continue;
    }
    phrase = _phrase(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&phrase.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names(&[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&phrase.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), attribute.clone()]))) } }) {
    continue;
    }
    return phrase.clone();
    }
    return py_str("");
    Py::None
}
pub fn _a_cost_line(args: &[Py]) -> Py {
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
    let _ = py_str("True when this line of theirs names a figure for something they pay.");
    line = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_A_FIGURE(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_COST(), "search", &[line.clone()]))) } } } };
    Py::None
}
pub fn _the_newest_cost(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The newest thing they said with a figure in it, said back, or \"\".");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = { let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&_a_cost_line(&[line.clone()])) {
    return _phrase(&[line.clone()]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_cost_just_before(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The figure said on the line before this one, said back, or \"\".\n\n    For \"how much is it\", which names nothing of its own: only the line immediately before it can\n    be the it, and with no figure there the ask is a price question about the world.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    line = { let __b = mcall(&{ let __b = geti(&turns.clone(), &op_neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } };
    return if py_truthy(&_a_cost_line(&[line.clone()])) { _phrase(&[line.clone()]) } else { py_str("") };
    Py::None
}
pub fn _phrase(args: &[Py]) -> Py {
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
    let mut memory_store: Py = Py::None;
    let _ = py_str("The line as a fact about them, in the store's own words.");
    memory_store = pyrt::xrefs::xmodule("modules.memory_store");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), mcall(&memory_store.clone(), "_told_phrase", &[line.clone()])]);
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
pub fn _the_value(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The fact without the frame: \"your budget is about 100 pounds\" -> \"about 100 pounds\".");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("^you(?:'re| are| were) (.+)$"), py_str("^you(?:'ve| have) (.+)$"), py_str("^your .+? is (.+)$"), py_str("^your (.+)$"), py_str("^it(?:'s|s) (.+)$"), py_str("^you (?:pay|paid) (.+)$"), py_str("^(?:it )?costs? (.+)$")]));
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = stdlib_call("re", "match", &[pattern.clone(), phrase.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    return mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "strip", &[]), "rstrip", &[py_str(".")]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_slot_ask(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut slot: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("(\"when\", \"where\" or \"size\", the subject words of the ask) or (\"\", set()) for a line\n    that asks after no slot at all.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("when"), g_WHEN_ASK()]), py_tuple(vec![py_str("where"), g_WHERE_ASK()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    slot = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    match_ = mcall(&pattern.clone(), "search", &[low.clone()]);
    if py_truthy(&match_.clone()) {
    rest = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&match_.clone(), "groups", &[])) { let mut group: Py = __item.clone(); if py_truthy(&group.clone()) {  __out.push(group.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    return py_tuple(vec![slot.clone(), _the_subject(&[rest.clone()])]);
    }
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("age"), g_AGE_ASK()]), py_tuple(vec![py_str("often"), g_OFTEN_ASK()]), py_tuple(vec![py_str("off_with"), g_OFF_WITH_ASK()])]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    slot = __unpk4[0].clone();
    pattern = __unpk4[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[low.clone()])) {
    return py_tuple(vec![slot.clone(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    if py_truthy(&_their_attribute(&[low.clone()])) {
    return py_tuple(vec![py_str(""), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    match_ = mcall(&g_SIZE_ASK(), "search", &[low.clone()]);
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SIZE_ADVICE(), "search", &[low.clone()]))) } }) {
    return py_tuple(vec![py_str("size"), _the_subject(&[mcall(&match_.clone(), "group", &[py_str("rest")])])]);
    }
    match_ = mcall(&g_MONEY_ASK(), "search", &[low.clone()]);
    if py_truthy(&match_.clone()) {
    what = mcall(&call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&match_.clone(), "groups", &[])) { let mut group: Py = __item.clone(); if py_truthy(&group.clone()) {  __out.push(group.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return py_tuple(vec![op_add(&py_str("money:"), &what.clone()), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return py_tuple(vec![py_str(""), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _the_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The words the ask names the thing by: \"when we were going\" -> {\"go\"}.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&rest.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_SLOT_STOP(), &word.clone()))) {  __out.push(_stem(&[word.clone()])); } } py_set(__out) })() };
    Py::None
}
pub fn _a_time(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let mut span: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The time said in this line, as they said it, or \"\".\n\n    The month or the day comes first: \"a long weekend in october\" is about october, and the\n    \"weekend\" in it says less. A span (\"next week\", \"at 7\") is read before a loose word\n    (\"weekend\", \"morning\"), because a span is what the person meant when they gave both.\n    ");
    low = mcall(&line.clone(), "lower", &[]);
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z'\\u2019]+"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&op_add(&g_MONTHS(), &g_WEEKDAYS()), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&named.clone()) {
    return mcall(&geti(&named.clone(), &Py::Int(0)), "capitalize", &[]);
    }
    span = mcall(&g_TIME_SPAN(), "search", &[low.clone()]);
    if py_truthy(&span.clone()) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&span.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z'\\u2019]+"), low.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_TIME_WORDS(), &word.clone()))) {
    return word.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_place(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut place: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place said in this line with its preposition - \"near the royal mile\" - or \"\".");
    match_ = mcall(&g_PLACE(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    place = mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("place")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&place.clone())); if py_truthy(&__b) { __b } else { op_le(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&place.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() }, &g_SLOT_STOP()) } }) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    Py::None
}
pub fn _a_size(args: &[Py]) -> Py {
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
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut after: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut measure: Py = Py::None;
    let _ = py_str("The size said in this line, as they said it, or \"\".\n\n    The noun the ask names is carried with the measure when the line puts it there: \"a chocolate\n    sponge in a 20cm tin\" is \"20cm tin\", which is what a person would say back.\n    ");
    let __iter1 = py_iter_or_raise(&mcall(&g_A_SIZE(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    measure = mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("measure")]), "split", &[])]);
    after = mcall(&mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("after")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&subject.clone(), &_stem(&[after.clone()]))) } } } }) {
    return op_add(&op_add(&measure.clone(), &py_str(" ")), &after.clone());
    }
    return measure.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_size_recall(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut subject: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut index: Py = Py::None;
    let mut line: Py = Py::None;
    let mut older: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The size of the thing the ask names, from their own lines, or \"\".");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    line = geti(&lines.clone(), &index.clone());
    value = _a_size(&[line.clone(), subject.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    continue;
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&line.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() };
    if py_truthy(&subject.clone()) {
    if py_truthy(&op_bitand(&said.clone(), &subject.clone())) {
    return value.clone();
    }
    older = if py_truthy(&index.clone()) { geti(&lines.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = mcall(&g_PRONOUN_LINE(), "match", &[line.clone()]); if !py_truthy(&__b) { __b } else { op_bitand(&subject.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&older.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() }) } }) {
    return value.clone();
    }
    continue;
    }
    if py_truthy(&op_le(&op_sub(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &index.clone()), &Py::Int(2))) {
    return value.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_money_recall(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut what: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut lines: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lines") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lines"))) };
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The figure they gave for this money word, as they said it, or \"\".");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    low = mcall(&line.clone(), "lower", &[]);
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&g_SALARY_WORDS(), &what.clone()))) {
    match_ = mcall(&g_A_PER_YEAR(), "search", &[line.clone()]);
    if py_truthy(&match_.clone()) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    }
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&said.clone(), &_stem(&[what.clone()])))) {
    continue;
    }
    match_ = mcall(&g_A_MONEY(), "search", &[line.clone()]);
    if py_truthy(&match_.clone()) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_value_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut attribute: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "attribute") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: attribute"))) };
    let mut how: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how"))) };
    let _ = py_str("The pattern a line has to hold to answer this ask, or None.");
    attribute = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = attribute.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&attribute.clone()) {
    if py_truthy(&Py::Bool(py_contains(&g_VALUE_KINDS(), &attribute.clone()))) {
    return geti(&g_VALUE_KINDS(), &attribute.clone());
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("time"), py_str("day"), py_str("date"), py_str("month"), py_str("year"), py_str("week")]), &attribute.clone()))) {
    return py_func("_a_time", |args: &[Py]| _a_time(args));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MONEY_WORDS(), &attribute.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("price"), py_str("cost")]), &attribute.clone())) } }) {
    return { let attribute = attribute.clone(); let how = how.clone(); py_func("lambda", move |args: &[Py]| { let attribute = attribute.clone(); let how = how.clone(); let line: Py = __kwp(args, 0, "line").unwrap_or(Py::None); mcall(&g_A_MONEY(), "search", &[line.clone()]) }) };
    }
    return Py::None;
    }
    return mcall(&g_HOW_KINDS(), "get", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = how.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    Py::None
}
pub fn _the_line_that_says_it(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Their own line that holds the value asked for, or \"\".\n\n    Read newest first, and only the lines of theirs that are not themselves asks: reading back\n    \"what size did i say\" a turn later would answer the ask with an ask.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    match_ = mcall(&g_SAID_ASK(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    kind = _the_value_kind(&[mcall(&match_.clone(), "group", &[py_str("attr")]), mcall(&match_.clone(), "group", &[py_str("how")])]);
    if py_truthy(&Py::Bool(py_is(&kind.clone(), &Py::None))) {
    return py_str("");
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&if py_truthy(&call_builtin("callable", &[kind.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_call(&kind, &[line.clone()]) } else { mcall(&kind.clone(), "search", &[line.clone()]) })) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_THEIR_QUESTION(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SAID_ASK(), "match", &[mcall(&line.clone(), "lower", &[])]) } }) {
    continue;
    }
    if py_truthy(&_their_attribute(&[mcall(&line.clone(), "lower", &[])])) {
    continue;
    }
    return line.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_values_the_kind_takes(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut found: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut key: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Every value of the kind the ask names that their line holds, as they said it.\n\n    One value is the answer on its own - \"i ordered a coat in a medium\" answers \"what size did i\n    say\" with \"medium\", which is what the store's own reader says - and more than one leaves the\n    ask ambiguous (\"long and curly, past my shoulders\"), so the line itself is the safest answer.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    match_ = mcall(&g_SAID_ASK(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_list(vec![]);
    }
    kind = _the_value_kind(&[mcall(&match_.clone(), "group", &[py_str("attr")]), mcall(&match_.clone(), "group", &[py_str("how")])]);
    if py_truthy(&Py::Bool(py_is(&kind.clone(), &Py::None))) {
    return py_list(vec![]);
    }
    if py_truthy(&call_builtin("callable", &[kind.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    value = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = py_call(&kind, &[line.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return if py_truthy(&value.clone()) { py_list(vec![value.clone()]) } else { py_list(vec![]) };
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    found = __unpk1[0].clone();
    seen = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&kind.clone(), "finditer", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter2 {
    hit = __item.clone();
    value = mcall(&py_str(" "), "join", &[mcall(&mcall(&hit.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    key = _stem(&[value.clone()]);
    if py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&seen.clone(), &key.clone())) } }) {
    let _ = mcall(&seen.clone(), "add", &[key.clone()]);
    let _ = mcall(&found.clone(), "append", &[value.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_value_said(args: &[Py]) -> Py {
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
    let mut _the_store_says: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut values: Py = Py::None;
    let _ = py_str("The reply for an attribute asked for again with nothing named beside it, or None.\n\n    The store answers the asks it files a fact for - \"how old did i say he was\" is \"Your son is\n    14.\" - and a quotation of the line is a worse answer than that, so this reader only speaks\n    when the store has nothing to say, and it says the value in the store's own voice whenever\n    one value is what the ask was for.\n    ");
    line = _the_line_that_says_it(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    _the_store_says = py_func("respond", |args: &[Py]| pyrt::xrefs::xcall("modules.memory_store", "respond", args));
    if py_truthy(&py_call(&_the_store_says, &[text.clone(), memory.clone()])) {
    return Py::None;
    }
    values = _the_values_the_kind_takes(&[text.clone(), line.clone()]);
    said = if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[values.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&values.clone(), &Py::Int(0)) } else { line.clone() };
    reply = if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[values.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { op_add(&op_add(&py_str("You said: \""), &said.clone()), &py_str("\"")) } else { op_add(&op_add(&py_str("You said "), &said.clone()), &py_str(".")) };
    last = _the_last_reply(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_contains(&mcall(&last.clone(), "lower", &[]), &mcall(&mcall(&reply.clone(), "lower", &[]), "rstrip", &[py_str(".")])))) {
    return op_add(&py_str("Still the same: "), &said.clone());
    }
    return reply.clone();
    Py::None
}
pub fn _the_slot_recall(args: &[Py]) -> Py {
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
    let mut lines: Py = Py::None;
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut slot: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reply for a line that asks after a slot of what they said, or None.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_slot_ask(&[low.clone()]), 2, None);
    slot = __unpk1[0].clone();
    subject = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&slot.clone()))) {
    return Py::None;
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("age"), py_str("often"), py_str("off_with")]), &slot.clone()))) {
    value = py_str("");
    if py_truthy(&Py::Bool(py_eq(&slot.clone(), &py_str("age")))) {
    value = _the_age_they_gave(&[turns.clone(), low.clone()]);
    } else {
    if py_truthy(&Py::Bool(py_eq(&slot.clone(), &py_str("often")))) {
    value = _the_habit_they_gave(&[turns.clone()]);
    } else {
    value = _the_off_with_they_gave(&[turns.clone()]);
    }
    }
    return if py_truthy(&value.clone()) { _the_slot_reply(&[value.clone(), memory.clone()]) } else { Py::None };
    }
    if py_truthy(&mcall(&slot.clone(), "startswith", &[py_str("money:")])) {
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    value = _the_money_recall(&[geti(&mcall(&slot.clone(), "split", &[py_str(":"), Py::Int(1)]), &Py::Int(1)), lines.clone()]);
    return if py_truthy(&value.clone()) { _the_slot_reply(&[value.clone(), memory.clone()]) } else { Py::None };
    }
    if py_truthy(&Py::Bool(py_eq(&slot.clone(), &py_str("size")))) {
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    value = _the_size_recall(&[subject.clone(), lines.clone()]);
    return if py_truthy(&value.clone()) { _the_slot_reply(&[value.clone(), memory.clone()]) } else { Py::None };
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    turn = __item.clone();
    line = { let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    value = if py_truthy(&Py::Bool(py_eq(&slot.clone(), &py_str("when")))) { _a_time(&[line.clone()]) } else { _a_place(&[line.clone()]) };
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    continue;
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&line.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() };
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&said.clone(), &subject.clone()))) } }) {
    continue;
    }
    return _the_slot_reply(&[value.clone(), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _the_age_they_gave(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_person: Py = Py::None;
    let mut number: Py = Py::None;
    let mut said_of: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut whose: Py = Py::None;
    let _ = py_str("The age said as a number of its own, with the owner from the ask, or \"\".\n\n    The newest line that gives one is the one answered from, and a line that counts people\n    (\"theres 6 of us\") is not an age: the number has to stand as a clause of its own.\n    ");
    whose = py_str("their");
    named = stdlib_call("re", "search", &[py_str("\\b(his|her|their|my|our|its)\\b[^.?!]{0,12}?\\b(?:age|old)\\b"), low.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&named.clone()) {
    whose = mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    } else {
    said_of = stdlib_call("re", "search", &[py_str("\\b(he|she|they)\\b"), low.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&said_of.clone()) {
    whose = geti(&py_dict(vec![(py_str("he"), py_str("his")), (py_str("she"), py_str("her")), (py_str("they"), py_str("their"))]), &mcall(&mcall(&said_of.clone(), "group", &[Py::Int(1)]), "lower", &[]));
    }
    }
    if py_truthy(&Py::Bool(py_eq(&whose.clone(), &py_str("my")))) {
    whose = py_str("your");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_COUNT_OF_PEOPLE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SOMEBODY(), "search", &[line.clone()]))) } } } }) {
    continue;
    }
    found = mcall(&g_AN_AGE_CLAUSE(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    number = mcall(&found.clone(), "group", &[py_str("n")]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp21: Py = call_builtin("int", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_lt(&Py::Int(0), &__cmp21)) { op_lt(&__cmp21, &Py::Int(121)) } else { Py::Bool(false) } }))) {
    continue;
    }
    named_person = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:my|our)\\s+(?P<who>[a-z]+)\\s+(?P<was>is|was)\\s+"), &stdlib_call("re", "escape", &[number.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&named_person.clone()) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&py_str("your "), &mcall(&mcall(&named_person.clone(), "group", &[py_str("who")]), "lower", &[])), &py_str(" ")), &mcall(&mcall(&named_person.clone(), "group", &[py_str("was")]), "lower", &[])), &py_str(" ")), &number.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("their"), py_str("its")]), &whose.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:i|im|i'm|am)\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return op_add(&py_str("your age was "), &number.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&whose.clone(), &py_str("your")))) {
    return op_add(&py_str("your age was "), &number.clone());
    }
    return op_add(&op_add(&whose.clone(), &py_str(" age was ")), &number.clone());
    }
    return py_str("");
    Py::None
}
pub fn _the_off_with_they_gave(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut article: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What they said they were off sick with, or \"\".");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    found = mcall(&g_OFF_WITH_SAID(), "search", &[line.clone()]);
    if py_truthy(&found.clone()) {
    thing = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("what")]), "lower", &[]), "split", &[])]);
    article = if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&{ let __b = mcall(&found.clone(), "group", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]), &py_str("an")))) { py_str("an ") } else { py_str("a ") };
    return op_add(&op_add(&py_str("you were off with "), &article.clone()), &thing.clone());
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_habit_they_gave(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("How often they said they do it, as they said it, or \"\".");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    continue;
    }
    found = mcall(&g_A_HABIT_SAID(), "search", &[line.clone()]);
    if py_truthy(&found.clone()) {
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]), "split", &[])]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_slot_reply(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("The slot value as a friend says it, never as the same sentence twice.");
    reply = op_add(&op_add(&py_str("You said "), &value.clone()), &py_str("."));
    if py_truthy(&Py::Bool(py_contains(&mcall(&_the_last_reply(&[memory.clone()]), "lower", &[]), &mcall(&reply.clone(), "lower", &[])))) {
    return op_add(&op_add(&py_str("Still "), &value.clone()), &py_str("."));
    }
    return reply.clone();
    Py::None
}
pub fn _the_last_reply(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns.clone(), &op_neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _spoken(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut full: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("The phrase the way a person says it: the store writes contractions out.");
    phrase = stdlib_call("re", "sub", &[py_str("^its\\b"), py_str("it's"), call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_CONTRACTIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    full = __unpk2[0].clone();
    short = __unpk2[1].clone();
    phrase = stdlib_call("re", "sub", &[op_add(&op_add(&py_str("\\b"), &full.clone()), &py_str("\\b")), short.clone(), phrase.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return phrase.clone();
    Py::None
}
pub fn _the_ask_names_something_new(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the ask names a thing not one of their own lines ever mentioned.\n\n    \"how much do i pay for rent\" names the rent, and nothing in the chat is about rent, so the\n    wage they gave is a figure for something else. \"how much is my phone bill\" names the phone\n    bill, which the chat does hold, and that figure is the answer they want.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    said = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __aug2 = op_bitor(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() });
    said = __aug2;
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() };
    let __aug3 = op_sub(&asked.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PAY_ASK_WORDS()) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() });
    asked = __aug3;
    return call_builtin("bool", &[op_sub(&asked.clone(), &said.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_recall(args: &[Py]) -> Py {
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
    let mut attribute: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut last: Py = Py::None;
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_value: Py = Py::None;
    let mut slot: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wants_a_figure: Py = Py::None;
    let _ = py_str("The reply for a line that asks for a fact of theirs by name, or None.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    said_value = _the_value_said(&[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&said_value.clone(), &Py::None))) {
    return said_value.clone();
    }
    slot = _the_slot_recall(&[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&slot.clone(), &Py::None))) {
    return slot.clone();
    }
    attribute = _their_attribute(&[low.clone()]);
    wants_a_figure = Py::Bool(false);
    if py_truthy(&Py::Bool(!py_truthy(&attribute.clone()))) {
    if py_truthy(&mcall(&g_PAY_ASK(), "search", &[low.clone()])) {
    wants_a_figure = Py::Bool(true);
    } else {
    if py_truthy(&mcall(&g_COST_ASK(), "match", &[low.clone()])) {
    wants_a_figure = Py::Bool(true);
    } else {
    named = mcall(&g_WHATS_MY(), "search", &[low.clone()]);
    candidate = if py_truthy(&named.clone()) { mcall(&py_str(" "), "join", &[mcall(&mcall(&named.clone(), "group", &[py_str("attr")]), "split", &[])]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&candidate.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&candidate.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_QUESTION_WORDS(), &geti(&mcall(&candidate.clone(), "split", &[]), &Py::Int(0)))) } } } }) {
    return Py::None;
    }
    attribute = candidate.clone();
    wants_a_figure = Py::Bool(true);
    }
    }
    }
    if py_truthy(&attribute.clone()) {
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(attribute.clone()); __args.push(memory.clone()); __kws.extend(vec![("needs_a_figure".to_string(), wants_a_figure.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_phrase_of_theirs(args))(&__args) };
    } else {
    if py_truthy(&mcall(&g_COST_ASK(), "match", &[low.clone()])) {
    phrase = _the_cost_just_before(&[memory.clone()]);
    } else {
    phrase = _the_newest_cost(&[memory.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&phrase.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_PAY_ASK(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _the_ask_names_something_new(&[low.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.repeat", "the_figure_they_gave", &[text.clone(), memory.clone()]))) } } } }) {
    return Py::None;
    }
    said = _spoken(&[phrase.clone()]);
    value = _the_value(&[said.clone()]);
    last = _the_last_reply(&[memory.clone()]);
    if py_truthy(&{ let __b = value.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&mcall(&last.clone(), "lower", &[]), &mcall(&value.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&last.clone(), "lower", &[]), &py_str("you told me"))) } } } }) {
    return op_add(&op_add(&py_str("Still "), &value.clone()), &py_str("."));
    }
    return op_add(&op_add(&py_str("You told me "), &said.clone()), &py_str("."));
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut slot: Py = Py::None;
    let _ = py_str("True on exactly the lines `respond` answers, and nowhere else.\n\n    The clothes-size card declares the topic a coat chat is read as, and a card that declares\n    the line takes it off a module underneath it whatever the bands say (`arbiter._as_it_comes`).\n    On these lines the ask is about the person, so nothing about the world can answer it: the\n    module says so itself rather than raising `PRIORITY` over every other line it reads.\n\n    The three shapes read off a line that gave the value in other words - an age, a habit, an\n    illness - are the exception, and they are not declared. The store keeps those facts itself\n    and answers those asks at 7000 in its own words (\"how old did i say he was\" -> \"Your son is\n    14.\"), which is the answer a person already gets; declaring would take that turn off it and\n    say \"You said his age was 14.\" only because this module read the same number twice. So the\n    new shapes answer only where nothing above them does, which is every line they were written\n    for: the ask the store, the cards and the store's own recall all passed on.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __unpk1 = py_unpack(&_the_slot_ask(&[low.clone()]), 2, None);
    slot = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("age"), py_str("often"), py_str("off_with")]), &slot.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&_the_recall(&[text.clone(), memory.clone()]), &Py::None));
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    return _the_recall(&[text.clone(), memory.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.attribute_said_again".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGE_ASK" => Some(g_AGE_ASK()),
            "AGE_VALUE" => Some(g_AGE_VALUE()),
            "AN_AGE_CLAUSE" => Some(g_AN_AGE_CLAUSE()),
            "ASK" => Some(g_ASK()),
            "A_COUNT_OF_PEOPLE" => Some(g_A_COUNT_OF_PEOPLE()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_HABIT_SAID" => Some(g_A_HABIT_SAID()),
            "A_MONEY" => Some(g_A_MONEY()),
            "A_PER_YEAR" => Some(g_A_PER_YEAR()),
            "A_SIZE" => Some(g_A_SIZE()),
            "COLOUR_VALUE" => Some(g_COLOUR_VALUE()),
            "CONTRACTIONS" => Some(g_CONTRACTIONS()),
            "COST_ASK" => Some(g_COST_ASK()),
            "FILLER" => Some(g_FILLER()),
            "HOW_KINDS" => Some(g_HOW_KINDS()),
            "LENGTH_VALUE" => Some(g_LENGTH_VALUE()),
            "MONEY_ASK" => Some(g_MONEY_ASK()),
            "MONEY_QUALIFIER" => Some(g_MONEY_QUALIFIER()),
            "MONEY_WORDS" => Some(g_MONEY_WORDS()),
            "MONTHS" => Some(g_MONTHS()),
            "NOT_A_COST" => Some(g_NOT_A_COST()),
            "OFF_WITH_ASK" => Some(g_OFF_WITH_ASK()),
            "OFF_WITH_SAID" => Some(g_OFF_WITH_SAID()),
            "OFTEN_ASK" => Some(g_OFTEN_ASK()),
            "OF_MINE" => Some(g_OF_MINE()),
            "PAY_ASK" => Some(g_PAY_ASK()),
            "PAY_ASK_WORDS" => Some(g_PAY_ASK_WORDS()),
            "PLACE" => Some(g_PLACE()),
            "PRIORITY" => Some(g_PRIORITY()),
            "PRONOUN_LINE" => Some(g_PRONOUN_LINE()),
            "QUESTION_WORDS" => Some(g_QUESTION_WORDS()),
            "SAID_ASK" => Some(g_SAID_ASK()),
            "SALARY_WORDS" => Some(g_SALARY_WORDS()),
            "SIZE_ADVICE" => Some(g_SIZE_ADVICE()),
            "SIZE_ASK" => Some(g_SIZE_ASK()),
            "SIZE_VALUE" => Some(g_SIZE_VALUE()),
            "SLOT_STOP" => Some(g_SLOT_STOP()),
            "SOMEBODY" => Some(g_SOMEBODY()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "THEIRS" => Some(g_THEIRS()),
            "THEIR_QUESTION" => Some(g_THEIR_QUESTION()),
            "TIME_SPAN" => Some(g_TIME_SPAN()),
            "TIME_WORDS" => Some(g_TIME_WORDS()),
            "VALUE_KINDS" => Some(g_VALUE_KINDS()),
            "WEEKDAYS" => Some(g_WEEKDAYS()),
            "WEIGHT_VALUE" => Some(g_WEIGHT_VALUE()),
            "WHATS_MY" => Some(g_WHATS_MY()),
            "WHEN_ASK" => Some(g_WHEN_ASK()),
            "WHERE_ASK" => Some(g_WHERE_ASK()),
            "_a_cost_line" => Some(py_func("_a_cost_line", |args: &[Py]| _a_cost_line(args))),
            "_a_place" => Some(py_func("_a_place", |args: &[Py]| _a_place(args))),
            "_a_size" => Some(py_func("_a_size", |args: &[Py]| _a_size(args))),
            "_a_time" => Some(py_func("_a_time", |args: &[Py]| _a_time(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_phrase" => Some(py_func("_phrase", |args: &[Py]| _phrase(args))),
            "_spoken" => Some(py_func("_spoken", |args: &[Py]| _spoken(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_the_age_they_gave" => Some(py_func("_the_age_they_gave", |args: &[Py]| _the_age_they_gave(args))),
            "_the_ask_names_something_new" => Some(py_func("_the_ask_names_something_new", |args: &[Py]| _the_ask_names_something_new(args))),
            "_the_cost_just_before" => Some(py_func("_the_cost_just_before", |args: &[Py]| _the_cost_just_before(args))),
            "_the_habit_they_gave" => Some(py_func("_the_habit_they_gave", |args: &[Py]| _the_habit_they_gave(args))),
            "_the_last_reply" => Some(py_func("_the_last_reply", |args: &[Py]| _the_last_reply(args))),
            "_the_line_that_says_it" => Some(py_func("_the_line_that_says_it", |args: &[Py]| _the_line_that_says_it(args))),
            "_the_money_recall" => Some(py_func("_the_money_recall", |args: &[Py]| _the_money_recall(args))),
            "_the_newest_cost" => Some(py_func("_the_newest_cost", |args: &[Py]| _the_newest_cost(args))),
            "_the_off_with_they_gave" => Some(py_func("_the_off_with_they_gave", |args: &[Py]| _the_off_with_they_gave(args))),
            "_the_phrase_of_theirs" => Some(py_func("_the_phrase_of_theirs", |args: &[Py]| _the_phrase_of_theirs(args))),
            "_the_recall" => Some(py_func("_the_recall", |args: &[Py]| _the_recall(args))),
            "_the_size_recall" => Some(py_func("_the_size_recall", |args: &[Py]| _the_size_recall(args))),
            "_the_slot_ask" => Some(py_func("_the_slot_ask", |args: &[Py]| _the_slot_ask(args))),
            "_the_slot_recall" => Some(py_func("_the_slot_recall", |args: &[Py]| _the_slot_recall(args))),
            "_the_slot_reply" => Some(py_func("_the_slot_reply", |args: &[Py]| _the_slot_reply(args))),
            "_the_subject" => Some(py_func("_the_subject", |args: &[Py]| _the_subject(args))),
            "_the_value" => Some(py_func("_the_value", |args: &[Py]| _the_value(args))),
            "_the_value_kind" => Some(py_func("_the_value_kind", |args: &[Py]| _the_value_kind(args))),
            "_the_value_said" => Some(py_func("_the_value_said", |args: &[Py]| _the_value_said(args))),
            "_the_values_the_kind_takes" => Some(py_func("_the_values_the_kind_takes", |args: &[Py]| _the_values_the_kind_takes(args))),
            "_their_attribute" => Some(py_func("_their_attribute", |args: &[Py]| _their_attribute(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGE_ASK" => s_AGE_ASK(v),
            "AGE_VALUE" => s_AGE_VALUE(v),
            "AN_AGE_CLAUSE" => s_AN_AGE_CLAUSE(v),
            "ASK" => s_ASK(v),
            "A_COUNT_OF_PEOPLE" => s_A_COUNT_OF_PEOPLE(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_HABIT_SAID" => s_A_HABIT_SAID(v),
            "A_MONEY" => s_A_MONEY(v),
            "A_PER_YEAR" => s_A_PER_YEAR(v),
            "A_SIZE" => s_A_SIZE(v),
            "COLOUR_VALUE" => s_COLOUR_VALUE(v),
            "CONTRACTIONS" => s_CONTRACTIONS(v),
            "COST_ASK" => s_COST_ASK(v),
            "FILLER" => s_FILLER(v),
            "HOW_KINDS" => s_HOW_KINDS(v),
            "LENGTH_VALUE" => s_LENGTH_VALUE(v),
            "MONEY_ASK" => s_MONEY_ASK(v),
            "MONEY_QUALIFIER" => s_MONEY_QUALIFIER(v),
            "MONEY_WORDS" => s_MONEY_WORDS(v),
            "MONTHS" => s_MONTHS(v),
            "NOT_A_COST" => s_NOT_A_COST(v),
            "OFF_WITH_ASK" => s_OFF_WITH_ASK(v),
            "OFF_WITH_SAID" => s_OFF_WITH_SAID(v),
            "OFTEN_ASK" => s_OFTEN_ASK(v),
            "OF_MINE" => s_OF_MINE(v),
            "PAY_ASK" => s_PAY_ASK(v),
            "PAY_ASK_WORDS" => s_PAY_ASK_WORDS(v),
            "PLACE" => s_PLACE(v),
            "PRIORITY" => s_PRIORITY(v),
            "PRONOUN_LINE" => s_PRONOUN_LINE(v),
            "QUESTION_WORDS" => s_QUESTION_WORDS(v),
            "SAID_ASK" => s_SAID_ASK(v),
            "SALARY_WORDS" => s_SALARY_WORDS(v),
            "SIZE_ADVICE" => s_SIZE_ADVICE(v),
            "SIZE_ASK" => s_SIZE_ASK(v),
            "SIZE_VALUE" => s_SIZE_VALUE(v),
            "SLOT_STOP" => s_SLOT_STOP(v),
            "SOMEBODY" => s_SOMEBODY(v),
            "TAKES_AS_IT_COMES" => s_TAKES_AS_IT_COMES(v),
            "THEIRS" => s_THEIRS(v),
            "THEIR_QUESTION" => s_THEIR_QUESTION(v),
            "TIME_SPAN" => s_TIME_SPAN(v),
            "TIME_WORDS" => s_TIME_WORDS(v),
            "VALUE_KINDS" => s_VALUE_KINDS(v),
            "WEEKDAYS" => s_WEEKDAYS(v),
            "WEIGHT_VALUE" => s_WEIGHT_VALUE(v),
            "WHATS_MY" => s_WHATS_MY(v),
            "WHEN_ASK" => s_WHEN_ASK(v),
            "WHERE_ASK" => s_WHERE_ASK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.attribute_said_again", module_obj);
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_their_attribute", py_func("_their_attribute", |args: &[Py]| _their_attribute(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_stem", py_func("_stem", |args: &[Py]| _stem(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_names", py_func("_names", |args: &[Py]| _names(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_phrase_of_theirs", py_func("_the_phrase_of_theirs", |args: &[Py]| _the_phrase_of_theirs(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_a_cost_line", py_func("_a_cost_line", |args: &[Py]| _a_cost_line(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_newest_cost", py_func("_the_newest_cost", |args: &[Py]| _the_newest_cost(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_cost_just_before", py_func("_the_cost_just_before", |args: &[Py]| _the_cost_just_before(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_phrase", py_func("_phrase", |args: &[Py]| _phrase(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_value", py_func("_the_value", |args: &[Py]| _the_value(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_slot_ask", py_func("_the_slot_ask", |args: &[Py]| _the_slot_ask(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_subject", py_func("_the_subject", |args: &[Py]| _the_subject(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_a_time", py_func("_a_time", |args: &[Py]| _a_time(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_a_place", py_func("_a_place", |args: &[Py]| _a_place(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_a_size", py_func("_a_size", |args: &[Py]| _a_size(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_size_recall", py_func("_the_size_recall", |args: &[Py]| _the_size_recall(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_money_recall", py_func("_the_money_recall", |args: &[Py]| _the_money_recall(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_value_kind", py_func("_the_value_kind", |args: &[Py]| _the_value_kind(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_line_that_says_it", py_func("_the_line_that_says_it", |args: &[Py]| _the_line_that_says_it(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_values_the_kind_takes", py_func("_the_values_the_kind_takes", |args: &[Py]| _the_values_the_kind_takes(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_value_said", py_func("_the_value_said", |args: &[Py]| _the_value_said(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_slot_recall", py_func("_the_slot_recall", |args: &[Py]| _the_slot_recall(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_age_they_gave", py_func("_the_age_they_gave", |args: &[Py]| _the_age_they_gave(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_off_with_they_gave", py_func("_the_off_with_they_gave", |args: &[Py]| _the_off_with_they_gave(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_habit_they_gave", py_func("_the_habit_they_gave", |args: &[Py]| _the_habit_they_gave(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_slot_reply", py_func("_the_slot_reply", |args: &[Py]| _the_slot_reply(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_last_reply", py_func("_the_last_reply", |args: &[Py]| _the_last_reply(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_spoken", py_func("_spoken", |args: &[Py]| _spoken(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_ask_names_something_new", py_func("_the_ask_names_something_new", |args: &[Py]| _the_ask_names_something_new(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "_the_recall", py_func("_the_recall", |args: &[Py]| _the_recall(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_fn("modules.attribute_said_again", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.attribute_said_again", "AGE_ASK", g_AGE_ASK, s_AGE_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "AGE_VALUE", g_AGE_VALUE, s_AGE_VALUE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "AN_AGE_CLAUSE", g_AN_AGE_CLAUSE, s_AN_AGE_CLAUSE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "A_COUNT_OF_PEOPLE", g_A_COUNT_OF_PEOPLE, s_A_COUNT_OF_PEOPLE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "A_HABIT_SAID", g_A_HABIT_SAID, s_A_HABIT_SAID);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "A_MONEY", g_A_MONEY, s_A_MONEY);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "A_PER_YEAR", g_A_PER_YEAR, s_A_PER_YEAR);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "A_SIZE", g_A_SIZE, s_A_SIZE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "COLOUR_VALUE", g_COLOUR_VALUE, s_COLOUR_VALUE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "CONTRACTIONS", g_CONTRACTIONS, s_CONTRACTIONS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "COST_ASK", g_COST_ASK, s_COST_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "FILLER", g_FILLER, s_FILLER);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "HOW_KINDS", g_HOW_KINDS, s_HOW_KINDS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "LENGTH_VALUE", g_LENGTH_VALUE, s_LENGTH_VALUE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "MONEY_ASK", g_MONEY_ASK, s_MONEY_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "MONEY_QUALIFIER", g_MONEY_QUALIFIER, s_MONEY_QUALIFIER);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "MONEY_WORDS", g_MONEY_WORDS, s_MONEY_WORDS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "NOT_A_COST", g_NOT_A_COST, s_NOT_A_COST);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "OFF_WITH_ASK", g_OFF_WITH_ASK, s_OFF_WITH_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "OFF_WITH_SAID", g_OFF_WITH_SAID, s_OFF_WITH_SAID);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "OFTEN_ASK", g_OFTEN_ASK, s_OFTEN_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "OF_MINE", g_OF_MINE, s_OF_MINE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "PAY_ASK", g_PAY_ASK, s_PAY_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "PAY_ASK_WORDS", g_PAY_ASK_WORDS, s_PAY_ASK_WORDS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "PLACE", g_PLACE, s_PLACE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "PRONOUN_LINE", g_PRONOUN_LINE, s_PRONOUN_LINE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "QUESTION_WORDS", g_QUESTION_WORDS, s_QUESTION_WORDS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "SAID_ASK", g_SAID_ASK, s_SAID_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "SALARY_WORDS", g_SALARY_WORDS, s_SALARY_WORDS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "SIZE_ADVICE", g_SIZE_ADVICE, s_SIZE_ADVICE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "SIZE_ASK", g_SIZE_ASK, s_SIZE_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "SIZE_VALUE", g_SIZE_VALUE, s_SIZE_VALUE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "SLOT_STOP", g_SLOT_STOP, s_SLOT_STOP);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "SOMEBODY", g_SOMEBODY, s_SOMEBODY);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "TAKES_AS_IT_COMES", g_TAKES_AS_IT_COMES, s_TAKES_AS_IT_COMES);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "THEIRS", g_THEIRS, s_THEIRS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "THEIR_QUESTION", g_THEIR_QUESTION, s_THEIR_QUESTION);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "TIME_SPAN", g_TIME_SPAN, s_TIME_SPAN);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "TIME_WORDS", g_TIME_WORDS, s_TIME_WORDS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "VALUE_KINDS", g_VALUE_KINDS, s_VALUE_KINDS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "WEEKDAYS", g_WEEKDAYS, s_WEEKDAYS);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "WEIGHT_VALUE", g_WEIGHT_VALUE, s_WEIGHT_VALUE);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "WHATS_MY", g_WHATS_MY, s_WHATS_MY);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "WHEN_ASK", g_WHEN_ASK, s_WHEN_ASK);
    pyrt::xrefs::reg_access("modules.attribute_said_again", "WHERE_ASK", g_WHERE_ASK, s_WHERE_ASK);
}
