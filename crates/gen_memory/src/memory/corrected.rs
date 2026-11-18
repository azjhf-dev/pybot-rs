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


thread_local! { static G_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANY_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AT_TIME_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_BEFORE_A_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DAY_SAID_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOURNEY_GOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_UNIT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_NEW_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MADE_OF_A_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_ALT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FACT_OF_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_SUBJECT_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_DIGITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_BARE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_KIND_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATIVE_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLING_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_THE_SAME_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEEKDAY_IN_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AMOUNT() -> Py { ensure_init(); G_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT(v: Py) { ensure_init(); G_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_WORD() -> Py { ensure_init(); G_AMOUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_WORD(v: Py) { ensure_init(); G_AMOUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ANY_NUMBER() -> Py { ensure_init(); G_ANY_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANY_NUMBER(v: Py) { ensure_init(); G_ANY_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_START() -> Py { ensure_init(); G_ASK_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_START(v: Py) { ensure_init(); G_ASK_START.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_TIME() -> Py { ensure_init(); G_AT_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_TIME(v: Py) { ensure_init(); G_AT_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_AT_TIME_VALUE() -> Py { ensure_init(); G_AT_TIME_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AT_TIME_VALUE(v: Py) { ensure_init(); G_AT_TIME_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_BEFORE_A_MONTH() -> Py { ensure_init(); G_A_DAY_BEFORE_A_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_BEFORE_A_MONTH(v: Py) { ensure_init(); G_A_DAY_BEFORE_A_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DAY_SAID_LONG() -> Py { ensure_init(); G_A_DAY_SAID_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DAY_SAID_LONG(v: Py) { ensure_init(); G_A_DAY_SAID_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOURNEY_GOING() -> Py { ensure_init(); G_A_JOURNEY_GOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOURNEY_GOING(v: Py) { ensure_init(); G_A_JOURNEY_GOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_UNIT_SAID() -> Py { ensure_init(); G_A_UNIT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_UNIT_SAID(v: Py) { ensure_init(); G_A_UNIT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_HOUR() -> Py { ensure_init(); G_BARE_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_HOUR(v: Py) { ensure_init(); G_BARE_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_NEW_COUNT() -> Py { ensure_init(); G_BARE_NEW_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_NEW_COUNT(v: Py) { ensure_init(); G_BARE_NEW_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTABLE() -> Py { ensure_init(); G_CORRECTABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTABLE(v: Py) { ensure_init(); G_CORRECTABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_BACK() -> Py { ensure_init(); G_CORRECTION_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_BACK(v: Py) { ensure_init(); G_CORRECTION_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_SAID() -> Py { ensure_init(); G_COUNT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_SAID(v: Py) { ensure_init(); G_COUNT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_WORDS() -> Py { ensure_init(); G_COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORDS(v: Py) { ensure_init(); G_COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CURRENCY() -> Py { ensure_init(); G_CURRENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURRENCY(v: Py) { ensure_init(); G_CURRENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_DATE_DAY() -> Py { ensure_init(); G_DATE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATE_DAY(v: Py) { ensure_init(); G_DATE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_DATE_PHRASE() -> Py { ensure_init(); G_DATE_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATE_PHRASE(v: Py) { ensure_init(); G_DATE_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_WORD() -> Py { ensure_init(); G_DAY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_WORD(v: Py) { ensure_init(); G_DAY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_FAMILY() -> Py { ensure_init(); G_FACT_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_FAMILY(v: Py) { ensure_init(); G_FACT_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_MADE_OF_A_VALUE() -> Py { ensure_init(); G_MADE_OF_A_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MADE_OF_A_VALUE(v: Py) { ensure_init(); G_MADE_OF_A_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_ALT() -> Py { ensure_init(); G_MONTH_ALT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_ALT(v: Py) { ensure_init(); G_MONTH_ALT.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_WORD() -> Py { ensure_init(); G_MONTH_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_WORD(v: Py) { ensure_init(); G_MONTH_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTH_WORDS() -> Py { ensure_init(); G_MONTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTH_WORDS(v: Py) { ensure_init(); G_MONTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMING_VERB() -> Py { ensure_init(); G_NAMING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMING_VERB(v: Py) { ensure_init(); G_NAMING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FACT_OF_THEIRS() -> Py { ensure_init(); G_NOT_A_FACT_OF_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FACT_OF_THEIRS(v: Py) { ensure_init(); G_NOT_A_FACT_OF_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_UNIT() -> Py { ensure_init(); G_NOT_A_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_UNIT(v: Py) { ensure_init(); G_NOT_A_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_SUBJECT_OF_ITS_OWN() -> Py { ensure_init(); G_NO_SUBJECT_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_SUBJECT_OF_ITS_OWN(v: Py) { ensure_init(); G_NO_SUBJECT_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_DIGITS() -> Py { ensure_init(); G_NUMBER_DIGITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_DIGITS(v: Py) { ensure_init(); G_NUMBER_DIGITS.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_VALUE() -> Py { ensure_init(); G_NUMBER_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_VALUE(v: Py) { ensure_init(); G_NUMBER_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORD() -> Py { ensure_init(); G_NUMBER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORD(v: Py) { ensure_init(); G_NUMBER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_BARE_NUMBER() -> Py { ensure_init(); G_OTHER_BARE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_BARE_NUMBER(v: Py) { ensure_init(); G_OTHER_BARE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_KIND_SAY() -> Py { ensure_init(); G_OTHER_KIND_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_KIND_SAY(v: Py) { ensure_init(); G_OTHER_KIND_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_SAID() -> Py { ensure_init(); G_OTHER_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_SAID(v: Py) { ensure_init(); G_OTHER_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_PETS() -> Py { ensure_init(); G_PLURAL_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_PETS(v: Py) { ensure_init(); G_PLURAL_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATIVE_MONTH() -> Py { ensure_init(); G_RELATIVE_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATIVE_MONTH(v: Py) { ensure_init(); G_RELATIVE_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g_SPELLING_APART() -> Py { ensure_init(); G_SPELLING_APART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPELLING_APART(v: Py) { ensure_init(); G_SPELLING_APART.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_THE_SAME_PERSON() -> Py { ensure_init(); G_STILL_THE_SAME_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_THE_SAME_PERSON(v: Py) { ensure_init(); G_STILL_THE_SAME_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_FAMILY() -> Py { ensure_init(); G_UNIT_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_FAMILY(v: Py) { ensure_init(); G_UNIT_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_WEEKDAY_IN_PLAN() -> Py { ensure_init(); G_WEEKDAY_IN_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_IN_PLAN(v: Py) { ensure_init(); G_WEEKDAY_IN_PLAN.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Moved out of bot/modules/memory_store.py whole, to keep that file small.\n\nNothing here is new: the same readers and pools, imported back by the module that\nused to hold them. See bot/memory/store_lines.py for the same move made earlier.\n");
    s_AMOUNT_WORD(crate::memory::store_lines::g_AMOUNT_WORD());
    s_AT_TIME_VALUE(crate::memory::store_lines::g_AT_TIME_VALUE());
    s_CORRECTION_BACK(crate::memory::store_lines::g_CORRECTION_BACK());
    s_COUNT_WORDS(crate::memory::store_lines::g_COUNT_WORDS());
    s_FACT_FAMILY(crate::memory::store_lines::g_FACT_FAMILY());
    s_NOT_A_UNIT(crate::memory::store_lines::g_NOT_A_UNIT());
    s_OTHER_KIND_SAY(crate::memory::store_lines::g_OTHER_KIND_SAY());
    s_OTHER_SAID(crate::memory::store_lines::g_OTHER_SAID());
    s_PLURAL_PETS(crate::memory::store_lines::g_PLURAL_PETS());
    s_SPELLING_APART(crate::memory::store_lines::g_SPELLING_APART());
    s_UNIT_FAMILY(crate::memory::store_lines::g_UNIT_FAMILY());
    s_OTHER_BARE_NUMBER(stdlib_call("re", "compile", &[py_str("\\d{1,3}")]).unwrap_or_else(|e| exc::raise(e)));
    s_MADE_OF_A_VALUE(stdlib_call("re", "compile", &[py_str("^(?:[\\u00a3$\\u20ac]?\\s?\\d|the\\s+\\d{1,2}(?:st|nd|rd|th)|\\d{1,2}(?::\\d{2})?\\s*(?:am|pm)|(?:next\\s+|this\\s+|on\\s+|until\\s+|till\\s+)?(?:mon|tues|wednes|thurs|fri|satur|sun)day|(?:next\\s+|this\\s+|in\\s+)?(?:january|february|march|april|may|june|july|august|september|october|november|december))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_UNIT_SAID(stdlib_call("re", "compile", &[py_str("(?<![\\w.])\\d[\\d,]*(?:\\.\\d+)?\\s*([a-z]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMING_VERB(stdlib_call("re", "compile", &[py_str("^(?:(?:called|named)\\s+|(?:he|she|it|they)(?:'s|s|\\s+is|\\s+was|\\s+are)\\s+(?:(?:called|named)\\s+)?)(?=\\S)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FACT_OF_THEIRS(call_builtin("frozenset", &[mcall(&py_str("who pronoun names name"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_SAID_LONG(stdlib_call("re", "compile", &[py_str("^(?:the\\s+)?(\\d{1,2}(?:st|nd|rd|th)?)\\s+(?:of\\s+)?([a-z]+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_VALUE(stdlib_call("re", "compile", &[py_str("^(\\d+(?:\\.\\d+)?)\\s*([a-z]+)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_DIGITS(stdlib_call("re", "compile", &[py_str("[£$€]?\\d+(?:[.,]\\d+)?")]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_HOUR(stdlib_call("re", "compile", &[py_str("^(\\d{1,2})$")]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_SAID(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b("), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::memory::facts::g_PET_COUNT_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str("|\\d{1,2})\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANY_NUMBER(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\w+teen|twenty|thirty|forty|fifty|hundred)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_NEW_COUNT(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|ah|oh|right)\\b[,.]?\\s*)*(?:(?:no|nope|sorry|actually|oops|wait|hold\\s+on|hold\\s+up|hang\\s+on)\\b[,.]?\\s*|i\\s+mean(?:t)?\\s+|make\\s+(?:that|it)\\s+)+(?:(?:just|only|about|around|roughly|nearly|exactly)\\s+(?:the\\s+)?)?(?:(?:it'?s|its|that'?s|thats)\\s+)?(?P<count>\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\w+teen)\\b\\s*(?:now\\b)?\\s*[.!]?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_JOURNEY_GOING(stdlib_call("re", "compile", &[py_str("\\b(?:fly|flies|flying|flight|flights|travel|travels|travelling|traveling|trip|trips|holiday|holidays|booking|booked|visit|visiting|journey|outing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DATE_DAY(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2})(?:st|nd|rd|th)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_BEFORE_A_MONTH(stdlib_call("re", "compile", &[op_modulo(&py_str("\\b(?:the\\s+)?(\\d{1,2})\\s+(?:of\\s+)?(?:%s)\\b"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::facts::g_MONTHS()) { let mut name: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEEKDAY_IN_PLAN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:(?:this|next|last|coming|the)\\s+)?(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&crate::memory::facts::g_DAYS(), &py_set(vec![py_str("weekend")]))); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RELATIVE_MONTH(stdlib_call("re", "compile", &[py_str("\\b(?:next|this|that|the\\s+following|the\\s+coming|the\\s+next)\\s+month\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTABLE(call_builtin("frozenset", &[py_tuple(vec![py_str("name"), py_str("age"), py_str("city"), py_str("places"), py_str("job"), py_str("diet"), py_str("allergies"), py_str("height"), py_str("weight"), py_str("budget"), py_str("pet"), py_str("pets"), py_str("plan"), py_str("birthday")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AMOUNT(stdlib_call("re", "compile", &[py_str("(?:[£$\\u20ac]\\s?)?\\d[\\d,]*(?:\\.\\d+)?(?:\\s*(?:(?:hundred|thousand|million|grand)(?![a-z])|k(?![a-z])))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_WORD(stdlib_call("re", "compile", &[py_str("(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|noon|midday|midnight)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTHS(call_builtin("frozenset", &[mcall(&py_str("january february march april may june july august september october november december jan feb mar apr jun jul aug sep sept oct nov dec"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTH_WORD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MONTHS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTH_ALT(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MONTHS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_DATE_PHRASE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:the\\s+)?\\d{1,2}(?:st|nd|rd|th)\\b(?:\\s+of\\s+(?:"), &g_MONTH_ALT()), &py_str("))?|\\b(?:the\\s+)?\\d{1,2}\\s+(?:")), &g_MONTH_ALT()), &py_str(")\\b|\\b(?:")), &g_MONTH_ALT()), &py_str(")\\s+(?:the\\s+)?\\d{1,2}(?:st|nd|rd|th)?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AT_TIME(stdlib_call("re", "compile", &[op_add(&py_str("\\bat\\s+"), &crate::memory::store_lines::g_AT_TIME_VALUE()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_WORD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::memory::facts::g_DAYS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CURRENCY(stdlib_call("re", "compile", &[py_str("[£$\\u20ac]\\s?")]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_SUBJECT_OF_ITS_OWN(call_builtin("frozenset", &[mcall(&py_str("\na about actually ah all also am an and anyway as at be been but by can could did do does doing\nfor from get gets got had has have i i'd i'll i'm i've if in into is it it's its just like made\nmake makes me meant meanto maybe might my no nope not of oh ok okay on or own please probably\nput puts rather right said say says so sorry than that that's thats the then there these think\nthis those to too um us was we well were what when where which who will with would yeah yeh yes\nyou your\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_THE_SAME_PERSON(call_builtin("frozenset", &[mcall(&py_str("\nbirthday bday dob born her hers he him his she their them they\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_START(stdlib_call("re", "compile", &[py_str("^\\W*(?:does|did|do|is|are|was|were|can|could|would|will|has|have)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTH_WORDS(call_builtin("frozenset", &[mcall(&py_str("january february march april may june july august september october\nnovember december"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_START(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:anyway|and|but|so|oh|right|ok|okay|sorry|just|also|well|then|actually|anyhow|plus)[\\s,]+)*(?:what|whats|why|whys|how|hows|when|whens|where|wheres|who|whos|which|whose|can|could|should|would|do|does|did|is|are|(?:was|were)(?!\\s+\\w+ing\\b)|will|shall|have|has|any\\b|anyone|anything)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _number_belongs_to_that_age(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stated: Py = Py::None;
    let _ = py_str("Whether a bare number can only be the age of the person named before it.\n\n    A number said on its own belongs to the fact the line before it stated. When that line stated\n    that person's age (\"my son is 7\" then \"no, 8\") or asked something they want kept (\"what should\n    I get her\" then \"no, 6\"), the age is what moves. When it stated something else about the user\n    (\"i weigh 80 kilos\", \"my budget is 30\"), the number is that fact's correction, and reading it\n    as the child's age files a weight against the child: \"my son is 7\", \"i weigh 80 kilos\", \"no,\n    75\" used to keep the son at 75 years old.\n    ");
    last = mcall(&call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::Bool(true);
    }
    stated = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &py_str("other"))) } }) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = stated.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&stated.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), &last.clone())) } }) {
    return Py::Bool(true);
    }
    fixed = { let __b = mcall(&memory.clone(), "get", &[py_str("correction")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[fixed.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("said")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), &last.clone())) } }) {
    said = call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("line")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&crate::memory::facts::extract(&[said.clone()]), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } } } }) {
    return Py::Bool(true);
    }
    }
    return call_builtin("bool", &[_asks(&[last.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_name_for_a_person(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a corrected value can be somebody's name at all.\n\n    A rename is only read when what follows is a name: `_not_a_name` turns down the words people\n    use about somebody (\"fine\", \"complicated\") and a month or a weekday is never one, so the\n    answer to \"what is my sister called\" is never \"Sunday\".\n    ");
    word = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    low = mcall(&word.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::memory::facts::g_MONTHS(), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_DAYS(), &low.clone())) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __kws.extend(vec![("strict_ly".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::facts::_not_a_name(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_NAMES_THAT_END_LIKE_A_VERB(), &low.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&low.clone(), "endswith", &[crate::memory::facts::g_NOT_NAME_ENDING()])));
    Py::None
}
pub fn _a_new_name_was_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut target: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut parts: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the correction gives somebody a name at all, in one of the three shapes.\n\n    The naming word says it in as many words (\"no hes called dave\"), and so does a denial that\n    gives the name back (\"no he isn't, it's Dave\") - the pronoun and the negation are the frame\n    there, and the name is what follows them. Away from those, people put a name right with the\n    name and nothing else: \"no its dave\", \"i meant dave\", \"sorry, dave\".\n\n    So the word itself is what has to hold up, and `facts.a_name_could_be` is the test: one word\n    of a name's shape, not a word the bot reads as language (\"its fine\", \"its complicated\", \"its\n    the plumber\"), and not a month or a day (\"its sunday\" is what day it is).\n\n    A name that runs to two words is a name too, and it is the commonest one people have:\n    \"no its mrs jones\", \"no its elsa bell\". Both words have to be a name or the title in front\n    of one, so \"no its mrs\" is still not a name and \"no it was very\" still is not.\n    ");
    word = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = crate::memory::named::carries_a_name(&[text.clone()]); if py_truthy(&__b) { __b } else { _denies_and_names(&[text.clone()]) } }) {
    return Py::Bool(true);
    }
    parts = mcall(&word.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if !py_truthy(&__b) { __b } else { { let __b = _a_word_of_a_name(&[geti(&parts.clone(), &Py::Int(0))]); if !py_truthy(&__b) { __b } else { crate::memory::facts::a_name_could_be(&[geti(&parts.clone(), &Py::Int(1))]) } } } }) {
    return Py::Bool(true);
    }
    return crate::memory::facts::a_name_could_be(&[word.clone()]);
    Py::None
}
pub fn _a_word_of_a_name(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("Whether a word can lead a name: a title (\"mr\", \"dr\") or a name itself.\n\n    Only the first half of a two-word name is checked with this, because the second half is\n    checked as a name, and \"no its mr\" is a title with nothing behind it.\n    ");
    low = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".")]), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_TITLE_WORDS(), &low.clone()))) {
    return Py::Bool(true);
    }
    return crate::memory::facts::a_name_could_be(&[low.clone()]);
    Py::None
}
pub fn _denies_and_names(args: &[Py]) -> Py {
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
    let mut denial: Py = Py::None;
    let _ = py_str("Whether the line denies the old name and gives the new one: \"no he isn't, it's Dave\".\n\n    The pronoun and the negation are the frame here, where the other routes look for \"called\".\n    Only the shape that gives a name back counts: \"no he isn't Dave\" denies one and names none, and\n    neither does \"no he isn't called Dave\" - the naming word there belongs to the clause being\n    denied, so the store keeps the name it had rather than the one the person ruled out.\n    ");
    if py_truthy(&crate::memory::facts::a_name_is_taken_back(&[text.clone()])) {
    return Py::Bool(false);
    }
    denial = mcall(&crate::memory::facts::g_DENIAL_OF_A_VALUE(), "match", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    return call_builtin("bool", &[{ let __b = denial.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&denial.clone(), "group", &[py_str("says")]); if !py_truthy(&__b) { __b } else { mcall(&denial.clone(), "group", &[py_str("value")]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _correct_other(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut _u: Py = Py::None;
    let mut age: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let mut item: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut known: Py = Py::None;
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut number: Py = Py::None;
    let mut saying: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The correction of a fact kept against someone else, as the line it lands on plus a note.\n\n    \"My mum is allergic to nuts\" then \"no, peanuts\": the fact is the mum's, so the user's own\n    allergies are not what is being corrected, and the correction has to be re-said in her shape\n    to reach it. Returns {\"line\", \"note\"}, or None when nothing is kept against anyone that fits.\n\n    `text` is the line the person said, because a name is only corrected by a line that says it\n    is the name (\"no shes called dana\"), not by the bare word the correction was read from.\n    ");
    known = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::others(&[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("who")]) } }) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&known.clone()))) {
    return Py::None;
    }
    number = mcall(&g_OTHER_BARE_NUMBER(), "match", &[mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_number_belongs_to_that_age(&[memory.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&number.clone()) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[known.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[mcall(&entry.clone(), "get", &[py_str("age")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    age = call_builtin("int", &[mcall(&number.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __cmp21: Py = age.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp21)) { op_le(&__cmp21, &Py::Int(120)) } else { Py::Bool(false) } })) } }) {
    return Py::None;
    }
    let _ = crate::memory::remember_other(&[memory.clone(), py_dict(vec![(py_str("who"), who.clone()), (py_str("age"), age.clone())])]);
    return py_dict(vec![(py_str("line"), op_modulo(&py_str("my %s is %d"), &py_tuple(vec![who.clone(), age.clone()]))), (py_str("note"), op_modulo(&py_str("%d — got it, your %s is %d from now on."), &py_tuple(vec![age.clone(), who.clone(), age.clone()])))]);
    }
    return Py::None;
    }
    let __iter3 = py_iter_or_raise(&call_builtin("reversed", &[known.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    entry = __item.clone();
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    continue;
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::store_lines::g_OTHER_SAID()) { let __unpk5784 = py_unpack(&__item.clone(), 2, None); let mut kind: Py = __unpk5784[0].clone(); let mut _u: Py = __unpk5784[1].clone(); if py_truthy(&mcall(&entry.clone(), "get", &[kind.clone()])) {  __out.push(kind.clone()); } } py_list(__out) })() };
    let __iter4 = py_iter_or_raise(&crate::memory::store_lines::g_OTHER_SAID());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk5[0].clone();
    saying = __unpk5[1].clone();
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&kept.clone(), &kind.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("name"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_name_for_a_person(&[target.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_new_name_was_said(&[target.clone(), text.clone()]))) } } } }) {
    continue;
    }
    line = op_modulo(&py_str("my %s %s %s"), &py_tuple(vec![who.clone(), saying.clone(), target.clone()]));
    found = crate::memory::facts::extract(&[line.clone()]);
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[item.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&item.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&who.clone(), "lower", &[]))) } }) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut item: Py = __item.clone(); __out.push(mcall(&item.clone(), "get", &[kind.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let _ = mcall(&entry.clone(), "pop", &[kind.clone(), Py::None]);
    let __iter6 = py_iter_or_raise(&mine.clone());
    for __item in __iter6 {
    item = __item.clone();
    let _ = crate::memory::remember_other(&[memory.clone(), item.clone(), line.clone()]);
    }
    shown = op_add(&mcall(&slice_or_raise(&call_builtin("str", &[target.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&call_builtin("str", &[target.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1), &Py::None, &Py::None));
    shape = mcall(&crate::memory::store_lines::g_OTHER_KIND_SAY(), "get", &[kind.clone(), py_str("{v} it is, then.")]);
    return py_dict(vec![(py_str("line"), line.clone()), (py_str("note"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("v".to_string(), shown.clone()), ("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&shape.clone(), "format", args))(&__args) })]);
    }
    }
    Py::None
}
pub fn _make_corrects(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut target: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("Whether \"make it ...\" puts a value where a value was.\n\n    \"make it 900\" after \"my budget is 500\" and \"make it 3 cats\" after \"we have 2 cats\" are\n    corrections, and a number, a date, a day or a time is one on its own. A word is one only when\n    the line it puts right holds a fact to put it in (\"we're going to lisbon\" / \"make it porto\").\n    \"my boiler is broken\" / \"make it stop\" and \"a new sofa\" / \"make it bigger\" are the rest of a\n    sentence, and folding \"Stop\" into the topic would colour every turn after it.\n    ");
    if py_truthy(&mcall(&g_MADE_OF_A_VALUE(), "match", &[mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[crate::memory::facts::extract(&[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_unit(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The family of the unit a value is said in, or \"\" when it names no unit.");
    match_ = mcall(&g_A_UNIT_SAID(), "search", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    return mcall(&crate::memory::store_lines::g_UNIT_FAMILY(), "get", &[mcall(&match_.clone(), "group", &[Py::Int(1)]), py_str("")]);
    Py::None
}
pub fn _stated_families(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut before: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut families: Py = Py::None;
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The families of unit a line is said in: the unit it names, and the facts it states.");
    families = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    unit = _the_unit(&[before.clone()]);
    if py_truthy(&unit.clone()) {
    let _ = mcall(&families.clone(), "add", &[unit.clone()]);
    }
    let __try1 = exc::catch(|| {
    found = crate::memory::facts::extract(&[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
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
    found = py_dict(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { found.clone() } else { py_tuple(vec![]) });
    for __item in __iter6 {
    key = __item.clone();
    family = mcall(&crate::memory::store_lines::g_FACT_FAMILY(), "get", &[key.clone()]);
    if py_truthy(&family.clone()) {
    let _ = mcall(&families.clone(), "add", &[family.clone()]);
    }
    }
    return families.clone();
    Py::None
}
pub fn _measured_the_same(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut family: Py = Py::None;
    let _ = py_str("True when the line before is measured in the unit the new value is said in.\n\n    Only values that name a unit are checked: a bare number (\"make that 800\") takes its meaning\n    from the fact it corrects, which is what the readers below decide. The line before answers\n    for it either by naming the same unit (\"lose 5 kilos\" / \"8 kilos\") or by stating a fact that\n    is measured in that family at all (\"my budget is 400\" / \"no, 600 pounds\").\n    ");
    family = _the_unit(&[value.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&family.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[_date_said(&[mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&_stated_families(&[before.clone()]), &family.clone()));
    Py::None
}
pub fn _without_the_naming_verb(args: &[Py]) -> Py {
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
    let mut fixed: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("\"no its called max\": the value read off that line is the verb and the name both.\n\n    The correction reader takes up to two words for a value, so a name given with its verb\n    (\"called max\") arrived with the verb in it - and the store then kept a dog called \"Called\n    max\", which is what the bot said back. The name is the word after the verb.\n\n    The pronoun in front is the same fault in another shape: \"no hes max\" was kept as a cat\n    called \"Hes max\", and \"no shes called milly\" as \"Shes called milly\". The pronoun names the\n    pet the correction is about, so it is never part of the name either.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    fixed = mcall(&g_NAMING_VERB(), "sub", &[py_str(""), said.clone()]);
    if py_truthy(&Py::Bool(py_eq(&fixed.clone(), &said.clone()))) {
    return said.clone();
    }
    return op_add(&mcall(&slice_or_raise(&fixed.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&fixed.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _the_plan_on_the_table(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The plan as the chat holds it right now, in the words the store keeps it in.\n\n    A second correction has no line in `facts_log` to lean on: the log holds the plan as it was\n    first said (\"sons birthday on 3 may\"), while the chat has already moved it to \"on 4 may\", so\n    \"no the 5th\" would find nothing to stand in place of. The plan on the table is the one the\n    correction is about to rewrite, and it is read here before that happens.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("plan")]));
    for __item in __iter1 {
    key = __item.clone();
    value = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    return value.clone();
    }
    }
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    value = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) {
    return value.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_plan_before(args: &[Py]) -> Py {
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
    let mut plan_after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan_after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan_after"))) };
    let mut after: Py = Py::None;
    let mut e: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The plan a correction came from: the newest one in the log that is not the new one.\n\n    A plan keeps no key of its own that can be read back, so the plan the person put right is\n    found where it was written down: the log holds every plan in the words it arrived in, and\n    the one the correction replaced is the newest that is not the value they now mean.\n    ");
    after = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = plan_after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut e: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[e.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(e.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("plan"), py_str("plans")]), &mcall(&entry.clone(), "get", &[py_str("key")])))) {
    continue;
    }
    value = mcall(&entry.clone(), "get", &[py_str("value")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&value.clone(), "strip", &[]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&value.clone(), "lower", &[]), "split", &[])]), &after.clone()))) {
    return value.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _what_took_its_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut before: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut value: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut change_end: Py = Py::None;
    let mut change_start: Py = Py::None;
    let mut end: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut head: Py = Py::None;
    let mut kept_head: Py = Py::None;
    let mut kept_tail: Py = Py::None;
    let mut new_middle: Py = Py::None;
    let mut start: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The words of `after` that stand where `value` stood in `before`, or None.\n\n    \"sons birthday on 3 may\" corrected to \"sons birthday on 4 may\" says the fact \"3 may\" is now\n    \"4 may\": the plan changed inside that fact, so the part of the new plan between the common\n    start and the common end is what the fact now says, with the words the change left alone\n    kept on either side of it. A plan that changed somewhere else returns None, so a value the\n    correction did not touch is never rewritten.\n    ");
    before = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    after = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    value = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&value.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&before.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&before.clone(), &value.clone())) } } } } } }) {
    return Py::None;
    }
    start = mcall(&before.clone(), "index", &[value.clone()]);
    end = op_add(&start.clone(), &call_builtin("len", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)));
    head = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&head.clone(), &call_builtin("min", &[call_builtin("len", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&before.clone(), &head.clone()), &geti(&after.clone(), &head.clone()))) } }) {
    let __aug1 = op_add(&head.clone(), &Py::Int(1));
    head = __aug1;
    }
    tail = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&tail.clone(), &op_sub(&call_builtin("min", &[call_builtin("len", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &head.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&before.clone(), &op_sub(&op_sub(&call_builtin("len", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &tail.clone())), &geti(&after.clone(), &op_sub(&op_sub(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &tail.clone())))) } }) {
    let __aug2 = op_add(&tail.clone(), &Py::Int(1));
    tail = __aug2;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![head.clone(), op_sub(&call_builtin("len", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &tail.clone())]), 2, None);
    change_start = __unpk3[0].clone();
    change_end = __unpk3[1].clone();
    if py_truthy(&{ let __b = op_le(&change_end.clone(), &start.clone()); if py_truthy(&__b) { __b } else { op_ge(&change_start.clone(), &end.clone()) } }) {
    return Py::None;
    }
    kept_head = slice_or_raise(&before.clone(), &start.clone(), &call_builtin("min", &[change_start.clone(), end.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None);
    kept_tail = if py_truthy(&op_lt(&change_end.clone(), &end.clone())) { slice_or_raise(&before.clone(), &call_builtin("max", &[change_end.clone(), start.clone()]).unwrap_or_else(|e| exc::raise(e)), &end.clone(), &Py::None) } else { py_str("") };
    new_middle = slice_or_raise(&after.clone(), &change_start.clone(), &call_builtin("max", &[change_start.clone(), op_sub(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &tail.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::None);
    fixed = mcall(&op_add(&op_add(&kept_head.clone(), &new_middle.clone()), &kept_tail.clone()), "strip", &[]);
    return { let __b = fixed.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _a_day_written_the_way_the_store_writes_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fact: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fact") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fact"))) };
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The day without the little words it was said with, when the fact is a day.");
    text = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = fact.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    found = mcall(&g_A_DAY_SAID_LONG(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return text.clone();
    }
    return op_modulo(&py_str("%s %s"), &py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(1)]), mcall(&found.clone(), "group", &[Py::Int(2)])]));
    Py::None
}
pub fn _the_person_the_plan_is_about(args: &[Py]) -> Py {
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
    let mut correction: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "correction") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: correction"))) };
    let mut before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { Py::None };
    let mut entry: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut plan_after: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut value: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("A fact kept against a person moves with the plan the person put right.\n\n    \"my sons birthday is 3 may\" then \"actually its on 4 may\" corrects the plan to \"sons birthday\n    on 4 may\" and leaves the son's birthday at 3 May, so \"when is my sons birthday\" reads back\n    the date they took back. The plan is the same fact the person carries, so a value the plan\n    held and no longer holds is written on the person with what took its place. Nothing is moved\n    on a plan that mentions nobody, so a trip keeps to itself.\n    ");
    plan_after = if py_truthy(&call_builtin("isinstance", &[correction.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&correction.clone(), "get", &[py_str("value")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    before = _the_plan_before(&[memory.clone(), plan_after.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return Py::Int(0);
    }
    sentence = mcall(&call_builtin("str", &[{ let __b = mcall(&correction.clone(), "get", &[py_str("sentence")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    moved = Py::Int(0);
    let __iter1 = py_iter_or_raise(&{ let __b = crate::memory::others(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&entry.clone(), "get", &[py_str("pronoun")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&who.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\w*\\b")), before.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = plan_after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&who.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\w*\\b")), mcall(&call_builtin("str", &[plan_after.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&call_builtin("list", &[mcall(&entry.clone(), "items", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    key = __unpk3[0].clone();
    value = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_FACT_OF_THEIRS(), &key.clone()))) {
    continue;
    }
    said = if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { py_str("") };
    taken = _what_took_its_place(&[before.clone(), plan_after.clone(), said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&taken.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&taken.clone(), &mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]))) } }) {
    continue;
    }
    taken = _a_day_written_the_way_the_store_writes_it(&[taken.clone()]);
    let _ = py_setitem(&entry.clone(), &key.clone(), taken.clone());
    kept = py_dict(vec![(if py_truthy(&mcall(&entry.clone(), "get", &[py_str("who")])) { py_str("who") } else { py_str("pronoun") }, { let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("pronoun")]) } }), (key.clone(), taken.clone())]);
    let _ = mcall(&crate::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), kept.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), { let __b = sentence.clone(); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = plan_after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]) } })])]);
    let __aug4 = op_add(&moved.clone(), &Py::Int(1));
    moved = __aug4;
    }
    }
    return moved.clone();
    Py::None
}
pub fn _a_correction_is_not_a_plan(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut body: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut known: Py = Py::None;
    let mut one: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A line that put a value right is not also a plan of its own.\n\n    \"im going to spain in june\" then \"actually we fly in july now\": the line says which month the\n    trip moved to, and the trip it moved is the one already in the store. Read as a plan of its\n    own as well, the list held \"Spain in July\" and \"we fly in july now\", and the reply said the\n    second one back - \"Got it - July.\" where the person had named the trip, not a month on its\n    own. The words the line corrects with are not part of what it says, so the plan to drop is\n    the line without them.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    body = mcall(&py_str(" "), "join", &[mcall(&mcall(&crate::memory::facts::g_CORRECTION_LEAD_RE(), "sub", &[py_str(""), said.clone()]), "split", &[])]);
    known = crate::memory::user(&[memory.clone()]);
    plans = mcall(&known.clone(), "get", &[py_str("plans")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[plans.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![said.clone(), body.clone()]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), kept.clone());
    }
    Py::None
}
pub fn _a_value_already_the_plans(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut held: Py = Py::None;
    let mut known: Py = Py::None;
    let mut latest: Py = Py::None;
    let mut line: Py = Py::None;
    let mut one: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the value a line corrects to is already what the person's own plan says.\n\n    A correction is applied once per turn but the store is written two or three times, and each\n    later pass found the old month still in a twin of the plan: \"im booking a trip to croatia in\n    july\" / \"my mums birthday is 2 april\" / \"actually we fly out in october\" moved \"Croatia in\n    july\" to October and then moved \"A trip to croatia in july\" as well, and the value the answer\n    reads the place from became the twin - so \"whats the food like\" lost Croatia and answered the\n    birthday as a party. The line's own words do not count: the store files the correction as a\n    plan on the way in (\"we fly out in october\"), and that is not a second thing to move.\n    ");
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    known = crate::memory::user(&[memory.clone()]);
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&p.clone(), "strip", &[]) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    latest = mcall(&known.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[latest.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&latest.clone(), "strip", &[]) } }) {
    let _ = mcall(&plans.clone(), "append", &[latest.clone()]);
    }
    let __iter1 = py_iter_or_raise(&plans.clone());
    for __item in __iter1 {
    one = __item.clone();
    held = mcall(&py_str(" "), "join", &[mcall(&mcall(&one.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&line.clone(), &held.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&held.clone(), &line.clone())) } } } }) {
    continue;
    }
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), held.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _apply_correction(args: &[Py]) -> Py {
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
    let mut correction: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "correction") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: correction"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut before: Py = Py::None;
    let mut both_numbers: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut hated: Py = Py::None;
    let mut held: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut line: Py = Py::None;
    let mut month: Py = Py::None;
    let mut moved_off_the_topic: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut part: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plan_on_the_table: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut target: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut topics: Py = Py::None;
    let mut wrong: Py = Py::None;
    let _ = py_str("'Sorry, I meant Florence, not Rome' replaces Rome in the topic it corrects.");
    target = if py_truthy(&call_builtin("isinstance", &[correction.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&correction.clone(), "get", &[py_str("to")]) } else { Py::None };
    if py_truthy(&target.clone()) {
    target = _without_the_naming_verb(&[target.clone()]);
    let _ = py_setitem(&correction.clone(), &py_str("to"), target.clone());
    }
    wrong = if py_truthy(&call_builtin("isinstance", &[correction.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&correction.clone(), "get", &[py_str("from")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&target.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[correction.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&correction.clone(), "get", &[py_str("applied")]) } }) {
    return Py::None;
    }
    if py_truthy(&_a_value_already_the_plans(&[memory.clone(), target.clone(), text.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&wrong.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_measured_the_same(&[target.clone(), { let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[correction.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&correction.clone(), "get", &[py_str("make")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_make_corrects(&[target.clone(), memory.clone()]))) } } } }) {
    return Py::None;
    }
    both_numbers = call_builtin("bool", &[{ let __b = Py::Bool(!py_truthy(&crate::memory::facts::a_clock_time(&[target.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_NUMBER_VALUE(), "match", &[{ let __b = mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if py_truthy(&__b) { __b } else { py_str("-") } }]); if !py_truthy(&__b) { __b } else { mcall(&g_NUMBER_VALUE(), "match", &[{ let __b = mcall(&call_builtin("str", &[{ let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if py_truthy(&__b) { __b } else { py_str("-") } }]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    plan_on_the_table = _the_plan_on_the_table(&[memory.clone()]);
    fixed = _correct_plan(&[memory.clone(), target.clone(), wrong.clone(), text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&fixed.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = both_numbers.clone(); if !py_truthy(&__b) { __b } else { _stated_plan(&[memory.clone()]) } })) } }) {
    fixed = { let __b = _correct_date(&[memory.clone(), target.clone(), text.clone(), { let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { { let __b = _correct_clock(&[memory.clone(), target.clone(), text.clone(), { let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { { let __b = _correct_number(&[memory.clone(), target.clone(), text.clone()]); if py_truthy(&__b) { __b } else { { let __b = _correct_count(&[memory.clone(), target.clone(), text.clone()]); if py_truthy(&__b) { __b } else { { let __b = _correct_fact(&[memory.clone(), target.clone(), wrong.clone(), text.clone()]); if py_truthy(&__b) { __b } else { { let __b = _correct_a_persons_name(&[memory.clone(), target.clone(), text.clone(), { let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { { let __b = _correct_when(&[memory.clone(), target.clone(), text.clone()]); if py_truthy(&__b) { __b } else { _correct_the_count_on_the_table(&[memory.clone(), target.clone(), text.clone()]) } } } } } } } } } } } } } };
    }
    if py_truthy(&{ let __b = fixed.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&fixed.clone(), "get", &[py_str("from")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&wrong.clone())) } } } }) {
    wrong = geti(&fixed.clone(), &py_str("from"));
    let _ = py_setitem(&correction.clone(), &py_str("from"), wrong.clone());
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    other = if py_truthy(&fixed.clone()) { Py::None } else { _correct_other(&[memory.clone(), target.clone(), text.clone()]) };
    if py_truthy(&other.clone()) {
    let _ = py_setitem(&other.clone(), &py_str("said"), said.clone());
    let _ = py_setitem(&other.clone(), &py_str("applied"), Py::Bool(true));
    let _ = py_setitem(&memory.clone(), &py_str("correction"), other.clone());
    }
    if py_truthy(&fixed.clone()) {
    if py_truthy(&Py::Bool(py_eq(&mcall(&fixed.clone(), "get", &[py_str("key")]), &py_str("plan")))) {
    let _ = _the_person_the_plan_is_about(&[memory.clone(), fixed.clone(), plan_on_the_table.clone()]);
    let _ = _a_correction_is_not_a_plan(&[memory.clone(), text.clone()]);
    }
    let _ = py_setitem(&fixed.clone(), &py_str("said"), said.clone());
    let _ = py_setitem(&fixed.clone(), &py_str("applied"), Py::Bool(true));
    let _ = py_setitem(&memory.clone(), &py_str("correction"), fixed.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&fixed.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&wrong.clone())) } }) {
    return Py::None;
    }
    record = py_call(&crate::memory::g_current(), &[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&record.clone()))) {
    return Py::None;
    }
    text = geti(&record.clone(), &py_str("text"));
    moved_off_the_topic = call_builtin("bool", &[{ let __b = fixed.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&fixed.clone(), "get", &[py_str("key")]), &py_str("plan"))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&fixed.clone(), "get", &[py_str("from")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[call_builtin("str", &[geti(&fixed.clone(), &py_str("from"))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b"))); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&moved_off_the_topic.clone()) {
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { text.clone() } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&line.clone()) {
    fresh = py_dict(vec![(py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), line.clone()), (py_str("words"), py_call(&crate::memory::g_keywords(), &[line.clone()]))]);
    let _ = py_setitem(&memory.clone(), &py_str("topic"), fresh.clone());
    topics = mcall(&memory.clone(), "get", &[py_str("topics")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[topics.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    topics = py_list(vec![]);
    let _ = py_setitem(&memory.clone(), &py_str("topics"), py_list(vec![]));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&topics.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&geti(&topics.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("text")]), &line.clone())) } }) {
    let _ = mcall(&topics.clone(), "append", &[fresh.clone()]);
    }
    }
    return Py::None;
    } else {
    if py_truthy(&other.clone()) {
    text = { let __b = mcall(&other.clone(), "get", &[py_str("line")]); if py_truthy(&__b) { __b } else { text.clone() } };
    } else {
    if py_truthy(&{ let __b = fixed.clone(); if !py_truthy(&__b) { __b } else { mcall(&fixed.clone(), "get", &[py_str("line")]) } }) {
    text = geti(&fixed.clone(), &py_str("line"));
    } else {
    if py_truthy(&{ let __b = wrong.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[wrong.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b"))); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    text = op_add(&op_add(&op_add(&text.clone(), &py_str(" (mean ")), &target.clone()), &py_str(")"));
    } else {
    if py_truthy(&{ let __b = fixed.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&fixed.clone(), "get", &[py_str("key")]), &py_str("plan"))); if !py_truthy(&__b) { __b } else { _the_topic_with_the_new_clock(&[text.clone(), call_builtin("str", &[{ let __b = mcall(&crate::memory::user(&[memory.clone()]), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } }) {
    text = _the_topic_with_the_new_clock(&[text.clone(), call_builtin("str", &[{ let __b = mcall(&crate::memory::user(&[memory.clone()]), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    } else {
    if py_truthy(&Py::Bool(!py_contains(&mcall(&text.clone(), "lower", &[]), &mcall(&target.clone(), "lower", &[])))) {
    text = op_add(&op_add(&op_add(&text.clone(), &py_str(" (mean ")), &target.clone()), &py_str(")"));
    }
    }
    }
    }
    }
    }
    let _ = py_setitem(&record.clone(), &py_str("text"), text.clone());
    let _ = py_setitem(&record.clone(), &py_str("words"), py_call(&crate::memory::g_keywords(), &[text.clone()]));
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*\\(mean [^)]*\\)\\s*$")); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    stated = { let __b = if py_truthy(&wrong.clone()) { _swap_in(&[before.clone(), target.clone(), wrong.clone()]) } else { Py::None }; if py_truthy(&__b) { __b } else { before.clone() } };
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&crate::memory::facts::extract(&[{ let __b = stated.clone(); if py_truthy(&__b) { __b } else { text.clone() } }]), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&op_sub(&call_builtin("set", &[entry.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("who"), py_str("pronoun")]))))) {
    continue;
    }
    fresh = mcall(&entry.clone(), "get", &[py_str("birthday")]);
    theirs = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::others(&[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &mcall(&entry.clone(), "get", &[py_str("who")])))) {  __out.push(one.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    kept = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = theirs.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("birthday")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = fresh.clone(); if !py_truthy(&__b) { __b } else { { let __b = wrong.clone(); if !py_truthy(&__b) { __b } else { kept.clone() } } } }) {
    hated = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z0-9]+"), mcall(&call_builtin("str", &[wrong.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("of"), py_str("on"), py_str("in")]), &part.clone())) } }) {  __out.push(part.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = hated.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&hated.clone()) { let mut part: Py = __item.clone(); __out.push(Py::Bool(py_contains(&kept.clone(), &part.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    }
    if py_truthy(&fresh.clone()) {
    held = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::others(&[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &mcall(&entry.clone(), "get", &[py_str("who")])))) {  __out.push(one.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    before = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = held.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("birthday")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    month = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::facts::g_MONTHS()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&before.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = month.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::memory::facts::g_MONTHS()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&call_builtin("str", &[fresh.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &name.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    let _ = py_setitem(&entry.clone(), &py_str("birthday"), op_modulo(&py_str("%s %s"), &py_tuple(vec![fresh.clone(), month.clone()])));
    }
    }
    let _ = crate::memory::remember_other(&[memory.clone(), call_builtin("dict", &[entry.clone()]).unwrap_or_else(|e| exc::raise(e)), { let __b = stated.clone(); if py_truthy(&__b) { __b } else { text.clone() } }]);
    }
    places = mcall(&crate::memory::user(&[memory.clone()]), "get", &[py_str("places")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[places.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = wrong.clone(); if !py_truthy(&__b) { __b } else { crate::memory::facts::place_value(&[target.clone()]) } } } }) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut place: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&call_builtin("str", &[place.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&target.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    let _ = mcall(&places.clone(), "append", &[target.clone()]);
    }
    }
    topics = mcall(&memory.clone(), "get", &[py_str("topics")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[topics.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = topics.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&geti(&topics.clone(), &op_neg(&Py::Int(1))), &record.clone())); if !py_truthy(&__b) { __b } else { mcall(&mcall(&geti(&topics.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("text"), py_str("")]), "startswith", &[slice_or_raise(&geti(&record.clone(), &py_str("text")), &Py::None, &Py::Int(10), &Py::None)]) } } } } } }) {
    let _ = py_setitem(&topics.clone(), &op_neg(&Py::Int(1)), record.clone());
    }
    let _ = py_call(&crate::memory::g_note(), &[memory.clone(), text.clone()]);
    Py::None
}
pub fn _stated_plan(args: &[Py]) -> Py {
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
    let mut entries: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The plan the line before this one stated, or None.\n\n    A plan has no fact key of its own (\"plan\", \"plans\"), so the correction path cannot read it\n    the way it reads a name or an age; the value it replaces is a number inside the plan.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    entries = mcall(&memory.clone(), "get", &[py_str("facts_log")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[entries.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("plan"), py_str("plans")]), &mcall(&entry.clone(), "get", &[py_str("key")])))) {
    value = mcall(&entry.clone(), "get", &[py_str("value")]);
    return if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&value.clone(), "strip", &[]) } }) { value.clone() } else { Py::None };
    }
    return Py::None;
    }
    return Py::None;
    Py::None
}
pub fn _an_amount_on_the_measure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut measure: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The plan with the amount they named now in it: \"Lose a stone\" / \"i meant half a stone\".\n\n    The correction of a plan was read only when both values were figures (\"lose 5 kilos\" / \"i\n    meant 8 kilos\"), so an amount said in words was dropped without a word: \"i want to lose a\n    stone\" then \"i meant half a stone\" kept \"Lose a stone\" and answered \"Thanks for telling me -\n    I'm here if you want to dig into any of that\", which says the correction did not land.\n\n    What makes the two amounts the same fact is the measure after them, so the measure is what is\n    matched: the word the new amount counts (\"stone\" in \"half a stone\", \"minutes\" in \"30 minutes\")\n    has to be a measure the plan already says, and the amount in front of it is what moves. A plan\n    that does not say that measure is a different fact and is left alone (\"lose a stone\" with\n    \"i meant a week\" corrects nothing), and so is a measure with no amount in front of it.\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    measure = geti(&words.clone(), &op_neg(&Py::Int(1)));
    stem = if py_truthy(&{ let __b = mcall(&measure.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[measure.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) { slice_or_raise(&measure.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None) } else { measure.clone() };
    if py_truthy(&op_lt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::None;
    }
    found = stdlib_call("re", "search", &[op_add(&op_add(&op_add(&op_add(&py_str("(?<![\\w])("), &crate::memory::store_lines::g_AMOUNT_WORD()), &py_str("\\s+)(")), &stdlib_call("re", "escape", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:s|es)?)(?![\\w])")), call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::None;
    }
    before = slice_or_raise(&plan.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None);
    said = mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&mcall(&before.clone(), "strip", &[])) {
    said = op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_dict(vec![(py_str("plan"), op_add(&op_add(&before.clone(), &said.clone()), &slice_or_raise(&plan.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None))), (py_str("value"), mcall(&call_builtin("str", &[target.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])), (py_str("in_the_sentence"), said.clone()), (py_str("was"), mcall(&found.clone(), "group", &[Py::Int(0)]))]);
    Py::None
}
pub fn _correct_plan(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut wrong: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wrong"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut amount: Py = Py::None;
    let mut count: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut known: Py = Py::None;
    let mut number: Py = Py::None;
    let mut origin: Py = Py::None;
    let mut p: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut right: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut swap: Py = Py::None;
    let mut there: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("A number corrected inside a plan: \"lose 5 kilos\" / \"i meant 8 kilos not 5\".\n\n    Both values have to be numbers, and the plan has to say the wrong one, or nothing is\n    touched: \"no, sesame\" is a list fact and \"no, thursday not tuesday\" is a day. The unit is\n    the plan's own, so \"i meant 8\" after \"lose 5 kilos\" gives \"lose 8 kilos\".\n    ");
    plan = _stated_plan(&[memory.clone()]);
    if py_truthy(&{ let __b = plan.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&wrong.clone())) } }) {
    amount = _an_amount_on_the_measure(&[plan.clone(), target.clone()]);
    if py_truthy(&amount.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&amount.clone(), &py_str("plan")), geti(&amount.clone(), &py_str("value"))]), 2, None);
    fixed = __unpk1[0].clone();
    shown = __unpk1[1].clone();
    known = crate::memory::user(&[memory.clone()]);
    let _ = py_setitem(&known.clone(), &py_str("plan"), fixed.clone());
    plans = mcall(&known.clone(), "get", &[py_str("plans")]);
    if py_truthy(&call_builtin("isinstance", &[plans.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut p: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&p.clone(), &plan.clone()))) { fixed.clone() } else { p.clone() }); } py_list(__out) })() });
    }
    let _ = py_call(&crate::memory::g_note(), &[memory.clone(), fixed.clone()]);
    origin = crate::memory::plan_line(&[memory.clone(), plan.clone()]);
    if py_truthy(&origin.clone()) {
    was = geti(&amount.clone(), &py_str("was"));
    origin = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![\\w])"), &stdlib_call("re", "escape", &[was.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![\\w])"))); __args.push(geti(&amount.clone(), &py_str("in_the_sentence"))); __args.push(origin.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    sentence = crate::memory::plan_sentence(&[fixed.clone(), origin.clone()]);
    return py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), shown.clone()), (py_str("sentence"), { let __b = sentence.clone(); if py_truthy(&__b) { __b } else { fixed.clone() } }), (py_str("line"), { let __b = origin.clone(); if py_truthy(&__b) { __b } else { Py::None } }), (py_str("quiet"), Py::Bool(true))]);
    }
    }
    right = mcall(&g_NUMBER_VALUE(), "match", &[mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&plan.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&right.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&wrong.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = crate::memory::facts::a_clock_time(&[target.clone()]); if py_truthy(&__b) { __b } else { crate::memory::facts::a_clock_time(&[wrong.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_NUMBER_VALUE(), "match", &[mcall(&call_builtin("str", &[wrong.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![mcall(&right.clone(), "group", &[Py::Int(1)]), mcall(&{ let __b = mcall(&right.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]), 2, None);
    number = __unpk2[0].clone();
    unit = __unpk2[1].clone();
    pattern = op_add(&py_str("(?<![\\d.])"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[wrong.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&unit.clone()) {
    let __aug3 = op_add(&pattern.clone(), &op_add(&op_add(&op_add(&op_add(&py_str("(\\s+"), &stdlib_call("re", "escape", &[unit.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b|\\s+(?!")), &crate::memory::store_lines::g_NOT_A_UNIT()), &py_str("\\b)[a-z]+\\b)?")));
    pattern = __aug3;
    }
    shown = py_list(vec![]);
pub fn __nested_swap_3(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut amount: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut count: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut fixed: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut known: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut number: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut origin: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut p: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut pattern: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut plan: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut plans: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut right: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut sentence: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut shown: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut swap: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut target: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut there: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut unit: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut was: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut wrong: Py = __pos.get(20).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(21).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut found: Py = Py::None;
    found = if py_truthy(&unit.clone()) { op_add(&op_add(&number.clone(), &py_str(" ")), &unit.clone()) } else { number.clone() };
    let _ = mcall(&shown.clone(), "append", &[found.clone()]);
    return found.clone();
    Py::None
}
    swap = {{ let amount = amount.clone(); let count = count.clone(); let fixed = fixed.clone(); let known = known.clone(); let memory = memory.clone(); let number = number.clone(); let origin = origin.clone(); let p = p.clone(); let pattern = pattern.clone(); let plan = plan.clone(); let plans = plans.clone(); let right = right.clone(); let sentence = sentence.clone(); let shown = shown.clone(); let swap = swap.clone(); let target = target.clone(); let text = text.clone(); let there = there.clone(); let unit = unit.clone(); let was = was.clone(); let wrong = wrong.clone(); py_func("swap", move |args: &[Py]| { let mut full = vec![amount.clone(), count.clone(), fixed.clone(), known.clone(), memory.clone(), number.clone(), origin.clone(), p.clone(), pattern.clone(), plan.clone(), plans.clone(), right.clone(), sentence.clone(), shown.clone(), swap.clone(), target.clone(), text.clone(), there.clone(), unit.clone(), was.clone(), wrong.clone()]; full.extend_from_slice(args); __nested_swap_3(&full) }) }};
    let __unpk5 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pattern.clone()); __args.push(swap.clone()); __args.push(plan.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "subn", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 2, None);
    fixed = __unpk5[0].clone();
    count = __unpk5[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&unit.clone()))) {
    there = stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![\\d.])"), &stdlib_call("re", "escape", &[number.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(\\s+[a-z]+)?")), fixed.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = there.clone(); if !py_truthy(&__b) { __b } else { mcall(&there.clone(), "group", &[Py::Int(1)]) } }) {
    let _ = py_setitem(&shown.clone(), &Py::Int(0), op_add(&number.clone(), &mcall(&there.clone(), "group", &[Py::Int(1)])));
    }
    }
    known = crate::memory::user(&[memory.clone()]);
    let _ = py_setitem(&known.clone(), &py_str("plan"), fixed.clone());
    plans = mcall(&known.clone(), "get", &[py_str("plans")]);
    if py_truthy(&call_builtin("isinstance", &[plans.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut p: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&p.clone(), &plan.clone()))) { fixed.clone() } else { p.clone() }); } py_list(__out) })() });
    }
    let _ = py_call(&crate::memory::g_note(), &[memory.clone(), fixed.clone()]);
    return py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), geti(&shown.clone(), &Py::Int(0))), (py_str("wrong"), mcall(&call_builtin("str", &[wrong.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))]);
    Py::None
}
pub fn _number_in(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The digits of a value, so \"£500\" and \"500\" are the same number.");
    match_ = mcall(&g_NUMBER_DIGITS(), "search", &[call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    return if py_truthy(&match_.clone()) { mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lstrip", &[py_str("£$€")]), "strip", &[]) } else { py_str("") };
    Py::None
}
pub fn _correct_a_persons_name(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut wrong: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { py_str("") };
    let mut entry: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut named: Py = Py::None;
    let mut new: Py = Py::None;
    let mut o: Py = Py::None;
    let mut old: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut taken_back: Py = Py::None;
    let mut was: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("A name given for somebody else is put right when the right name is said.\n\n    \"my sister is called ana\" and then \"actually its anna\" is the same sister spelled right, not\n    a second person: the name is filed against the person, and \"what is my sister called\" answers\n    from it, so it is the filed name that moves. Only a line that gave one person a name and\n    nothing else is corrected, and only with a single word that reads as a name, so \"my sister is\n    called ana\" then \"i meant the plumber\" corrects nothing, and no pet or the person typing is\n    touched here - those have their own paths and are answered before this one.\n    ");
    new = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&new.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&crate::memory::facts::g_NAME_WORD(), "match", &[new.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_NOT_A_NAME(), &mcall(&new.clone(), "lower", &[]))) } } } }) {
    return Py::None;
    }
    taken_back = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = taken_back.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&taken_back.clone(), &mcall(&new.clone(), "lower", &[]))) } }) {
    moved = _a_name_the_store_keeps(&[memory.clone(), taken_back.clone(), new.clone()]);
    if py_truthy(&Py::Bool(!py_is(&moved.clone(), &Py::None))) {
    return moved.clone();
    }
    }
    said = call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&crate::memory::facts::extract(&[said.clone()]), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("name")]) } } } }) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    who = call_builtin("str", &[geti(&geti(&named.clone(), &Py::Int(0)), &py_str("who"))]).unwrap_or_else(|e| exc::raise(e));
    old = call_builtin("str", &[geti(&geti(&named.clone(), &Py::Int(0)), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e));
    was = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&geti(&named.clone(), &Py::Int(0)), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&mcall(&old.clone(), "lower", &[]), &mcall(&new.clone(), "lower", &[])))) {
    return Py::None;
    }
    taken_back = mcall(&mcall(&call_builtin("str", &[{ let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_a_spelling_of(&[new.clone(), old.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&taken_back.clone(), &mcall(&old.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&was.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "lower", &[])); } py_list(__out) })() }, &taken_back.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = _a_new_name_was_said(&[new.clone(), text.clone()]); if !py_truthy(&__b) { __b } else { _a_name_for_a_person(&[new.clone()]) } })) } } } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&crate::memory::others(&[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("name")]) } } } }) {
    let _ = py_setitem(&entry.clone(), &py_str("name"), new.clone());
    let _ = py_setitem(&entry.clone(), &py_str("was"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut o: Py = __item.clone(); __out.push(call_builtin("str", &[o.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }, &py_list(vec![old.clone()]))) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&one.clone(), "lower", &[]), &mcall(&new.clone(), "lower", &[])))) {  __out.push(one.clone()); } } py_list(__out) })() });
    said_back = op_add(&mcall(&slice_or_raise(&old.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&old.clone(), &Py::Int(1), &Py::None, &Py::None));
    return py_dict(vec![(py_str("key"), py_str("person_name")), (py_str("value"), new.clone()), (py_str("wrong"), said_back.clone()), (py_str("from"), old.clone())]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_spelling_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut new: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let _ = py_str("Whether a word reads as another go at the same name: the same first letter, and close.\n\n    A month or a weekday is never a name, so a word the date readers already take is refused\n    even when it is a letter or two from the name.\n    ");
    one = mcall(&mcall(&call_builtin("str", &[{ let __b = new.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    other = mcall(&mcall(&call_builtin("str", &[{ let __b = old.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&one.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&other.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&one.clone(), &Py::Int(0)), &geti(&other.clone(), &Py::Int(0)))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::memory::facts::g_MONTHS(), &one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::memory::facts::g_DAYS(), &one.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&one.clone(), "startswith", &[other.clone()]); if py_truthy(&__b) { __b } else { mcall(&other.clone(), "startswith", &[one.clone()]) } }) {
    return Py::Bool(true);
    }
    return op_le(&_letters_apart(&[one.clone(), other.clone()]), &crate::memory::store_lines::g_SPELLING_APART());
    Py::None
}
pub fn _letters_apart(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut letter: Py = Py::None;
    let mut other_letter: Py = Py::None;
    let _ = py_str("How many single letters have to be added, dropped or changed to turn one into other.");
    if py_truthy(&op_lt(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    let __unpk1 = py_unpack(&py_tuple(vec![other.clone(), one.clone()]), 2, None);
    one = __unpk1[0].clone();
    other = __unpk1[1].clone();
    }
    before = call_builtin("list", &[call_builtin("range", &[op_add(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[one.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    i = __unpk3[0].clone();
    letter = __unpk3[1].clone();
    after = py_list(vec![i.clone()]);
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[other.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    j = __unpk5[0].clone();
    other_letter = __unpk5[1].clone();
    let _ = mcall(&after.clone(), "append", &[call_builtin("min", &[op_add(&geti(&before.clone(), &j.clone()), &Py::Int(1)), op_add(&geti(&after.clone(), &op_sub(&j.clone(), &Py::Int(1))), &Py::Int(1)), op_add(&geti(&before.clone(), &op_sub(&j.clone(), &Py::Int(1))), &Py::Bool(!py_eq(&letter.clone(), &other_letter.clone())))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    before = after.clone();
    }
    return geti(&before.clone(), &op_neg(&Py::Int(1)));
    Py::None
}
pub fn _correct_clock(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut wrong: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { py_str("") };
    let mut bare: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut known: Py = Py::None;
    let mut latest: Py = Py::None;
    let mut named: Py = Py::None;
    let mut new: Py = Py::None;
    let mut old: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let _ = py_str("A clock inside a plan is put right: \"the meeting is at 10am\" then \"make it 2pm\".\n\n    The plan keeps everything else it says and takes the new time, so \"what time is the\n    meeting\" answers with the hour they corrected it to. It could not be done by putting the\n    figure right: a figure in a plan was swapped on a word boundary, and `\u{08}10\u{08}` does not find\n    the 10 in \"10am\", so the store said \"Meeting at 10am - got it\" over a plan it had left as\n    it was. A bare figure is a clock only where the plan's own clock names no half of the day\n    (\"dinner at 8\" then \"make it 9\"): beside \"at 10am\", \"make it 2\" could be either.\n    ");
    new = crate::memory::facts::a_clock_time(&[target.clone()]);
    bare = if py_truthy(&new.clone()) { py_str("") } else { _a_bare_hour(&[target.clone()]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&new.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bare.clone())) } }) {
    return Py::None;
    }
    known = crate::memory::user(&[memory.clone()]);
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&p.clone(), "strip", &[]) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    latest = mcall(&known.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[latest.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&latest.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&plans.clone(), &latest.clone())) } } } }) {
    let _ = mcall(&plans.clone(), "append", &[latest.clone()]);
    }
    if py_truthy(&wrong.clone()) {
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut p: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&_clock_key(&[crate::memory::facts::the_clock_in(&[p.clone()])]), &_clock_key(&[wrong.clone()])))) {  __out.push(p.clone()); } } py_list(__out) })() };
    plans = { let __b = named.clone(); if py_truthy(&__b) { __b } else { plans.clone() } };
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    plan = __item.clone();
    found = crate::memory::facts::the_clock_in(&[plan.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    old = mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&new.clone()))) {
    if py_truthy(&_names_a_half_of_the_day(&[old.clone()])) {
    continue;
    }
    new = bare.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&_clock_key(&[old.clone()]), &_clock_key(&[new.clone()])))) {
    return Py::None;
    }
    fixed = mcall(&op_add(&op_add(&slice_or_raise(&plan.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &new.clone()), &slice_or_raise(&plan.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    let _ = crate::memory::remember(&[memory.clone(), py_str("plan"), fixed.clone(), text.clone()]);
    held = mcall(&known.clone(), "get", &[py_str("plans")]);
    if py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut p: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&call_builtin("str", &[p.clone()]).unwrap_or_else(|e| exc::raise(e)), &plan.clone()))) { fixed.clone() } else { p.clone() }); } py_list(__out) })() });
    }
    return py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), fixed.clone()), (py_str("from"), old.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _the_topic_with_the_new_clock(args: &[Py]) -> Py {
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
    let mut plan_now: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan_now") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan_now"))) };
    let mut fixed: Py = Py::None;
    let mut now: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut was: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The topic's own line with the plan's new clock in it, or None when the two are not the\n    same line.\n\n    \"im flying at 6am\" beside a plan that now reads \"im flying at 7am\" is the same sentence with\n    the hour they meant: the topic takes the new hour and keeps the words the person used, so the\n    next turn is read against one line and not against the hour that was taken back. A plan about\n    something else (\"Meeting at 2pm\" beside \"the meeting is at 10am\") is not the topic's line, and\n    nothing is moved: the answer to that one comes from the plan itself (\"At 2pm.\").\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    plan = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = plan_now.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&plan.clone())) } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![crate::memory::facts::the_clock_in(&[said.clone()]), crate::memory::facts::the_clock_in(&[plan.clone()])]), 2, None);
    was = __unpk1[0].clone();
    now = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&was.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&now.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_clock_key(&[mcall(&was.clone(), "group", &[Py::Int(0)])]), &_clock_key(&[mcall(&now.clone(), "group", &[Py::Int(0)])]))) } } } }) {
    return Py::None;
    }
pub fn __nested_words_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut fixed: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut now: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut plan: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut plan_now: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut was: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(8).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "line").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut found: Py = __pos.get(9).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "found").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    rest = mcall(&py_str(" "), "join", &[mcall(&op_add(&slice_or_raise(&line.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)), "split", &[])]);
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&rest.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("my"), py_str("your"), py_str("our"), py_str("the"), py_str("a"), py_str("an"), py_str("im"), py_str("i"), py_str("you"), py_str("is"), py_str("are"), py_str("its"), py_str("at")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
    words = {{ let fixed = fixed.clone(); let now = now.clone(); let plan = plan.clone(); let plan_now = plan_now.clone(); let said = said.clone(); let text = text.clone(); let was = was.clone(); let words = words.clone(); py_func("words", move |args: &[Py]| { let mut full = vec![fixed.clone(), now.clone(), plan.clone(), plan_now.clone(), said.clone(), text.clone(), was.clone(), words.clone()]; full.extend_from_slice(args); __nested_words_1(&full) }) }};
    if py_truthy(&Py::Bool(!py_eq(&py_call(&words, &[said.clone(), was.clone()]), &py_call(&words, &[plan.clone(), now.clone()])))) {
    return Py::None;
    }
    fixed = mcall(&op_add(&op_add(&slice_or_raise(&text.clone(), &Py::None, &mcall(&was.clone(), "start", &[]), &Py::None), &mcall(&now.clone(), "group", &[Py::Int(0)])), &slice_or_raise(&text.clone(), &mcall(&was.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    return fixed.clone();
    Py::None
}
pub fn _a_bare_hour(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The hour a bare figure could be, or \"\" when it is not one.");
    match_ = mcall(&g_BARE_HOUR(), "match", &[mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&match_.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __cmp11: Py = call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(24)) } else { Py::Bool(false) } })) } }) {
    return py_str("");
    }
    return mcall(&match_.clone(), "group", &[Py::Int(1)]);
    Py::None
}
pub fn _names_a_half_of_the_day(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clock: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clock") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clock"))) };
    let _ = py_str("True when a clock says which half of the day it is in (\"10am\", \"noon\").");
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("am|pm|o'?clock|noon|midday|midnight"), call_builtin("str", &[{ let __b = clock.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _clock_key(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clock: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clock") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clock"))) };
    let _ = py_str("A clock with everything but its figures and letters taken out, so 10am is 10am.");
    return stdlib_call("re", "sub", &[py_str("[^0-9a-z]"), py_str(""), mcall(&call_builtin("str", &[{ let __b = clock.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _correct_number(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut digits: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut n: Py = Py::None;
    let mut new: Py = Py::None;
    let mut old: Py = Py::None;
    let mut olds: Py = Py::None;
    let mut origin: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut said: Py = Py::None;
    let mut same: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut told: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A number is not a fact of its own either: it belongs to whatever the line before it\n    was about, so \"my budget is 500\" then \"make that 800\" moves the budget, and the user does\n    not have to name the thing again. Only a field that really holds the old number is moved,\n    so a stray digit somewhere in the line corrects nothing.\n    ");
    new = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    digits = _number_in(&[new.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&digits.clone()))) {
    return Py::None;
    }
    if py_truthy(&_names_a_date(&[new.clone()])) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { _a_birthday_on_the_last_line(&[memory.clone()]) } }) {
    return Py::None;
    }
    olds = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("\\d+(?:[.,]\\d+)?"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut n: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&n.clone(), &digits.clone()))) {  __out.push(n.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&olds.clone()))) {
    return Py::None;
    }
    known = crate::memory::user(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&olds.clone());
    for __item in __iter1 {
    old = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("budget"), py_str("age"), py_str("height"), py_str("weight")]));
    for __item in __iter2 {
    key = __item.clone();
    value = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&value.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_number_in(&[value.clone()]), &old.clone())) } }) {
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), value.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    same = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d+(?:[.,]\\d+)?")); __args.push(digits.clone()); __args.push(value.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    } else {
    same = if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("int", &[digits.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { digits.clone() };
    }
    let _ = crate::memory::remember(&[memory.clone(), key.clone(), same.clone(), text.clone()]);
    kept = mcall(&known.clone(), "get", &[key.clone()]);
    shown = if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![Py::None, py_str("")]), &kept.clone()))) { kept.clone() } else { same.clone() };
    return py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), shown.clone()), (py_str("from"), old.clone())]);
    }
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_str("plan"), py_str("job")]));
    for __item in __iter3 {
    key = __item.clone();
    value = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&value.clone(), &old.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&value.clone(), &digits.clone())) } } } }) {
    fixed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &old.clone()), &py_str("\\b"))); __args.push(digits.clone()); __args.push(value.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&fixed.clone(), &value.clone()))) {
    continue;
    }
    told = text.clone();
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plan")))) {
    origin = crate::memory::plan_line(&[memory.clone(), value.clone()]);
    told = { let __b = if py_truthy(&origin.clone()) { _swap_in(&[origin.clone(), digits.clone(), old.clone()]) } else { py_str("") }; if py_truthy(&__b) { __b } else { text.clone() } };
    }
    let _ = crate::memory::remember(&[memory.clone(), key.clone(), fixed.clone(), told.clone()]);
    plans = mcall(&known.clone(), "get", &[py_str("plans")]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("plan"))); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[plans.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut p: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&mcall(&call_builtin("str", &[p.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&value.clone(), "lower", &[])))) { fixed.clone() } else { p.clone() }); } py_list(__out) })() });
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plan")))) {
    return py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), fixed.clone()), (py_str("sentence"), told.clone()), (py_str("from"), old.clone())]);
    }
    return py_dict(vec![(py_str("key"), py_str("line")), (py_str("value"), op_add(&mcall(&slice_or_raise(&fixed.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&fixed.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("from"), old.clone())]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _count_said(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The number a line says, as a number: \"3\", \"three cats\", \"a couple of dogs\".");
    match_ = mcall(&g_COUNT_SAID(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Int(0);
    }
    word = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&mcall(&word.clone(), "isdigit", &[])) {
    return call_builtin("int", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return call_builtin("int", &[{ let __b = mcall(&crate::memory::facts::g_PET_COUNT_WORDS(), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_count(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut count: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut animal: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut many: Py = Py::None;
    let _ = py_str("A count of an animal said back as a person says it: \"three cats\", \"two fish\".");
    animal = mcall(&mcall(&call_builtin("str", &[{ let __b = animal.clone(); if py_truthy(&__b) { __b } else { py_str("pet") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&animal.clone(), "endswith", &[py_tuple(vec![py_str("fish"), py_str("sheep"), py_str("deer")])]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&count.clone(), &Py::Int(1))) } }) {
    many = animal.clone();
    } else {
    many = op_add(&animal.clone(), &py_str("s"));
    }
    return mcall(&py_str("{} {}"), "format", &[count.clone(), many.clone()]);
    Py::None
}
pub fn _correct_count(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut animal: Py = Py::None;
    let mut held: Py = Py::None;
    let mut named: Py = Py::None;
    let mut new: Py = Py::None;
    let mut old: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut told: Py = Py::None;
    let _ = py_str("A count of an animal already on the store is put right by saying the new count.\n\n    \"we have two cats\" then \"sorry, 3 cats\" is the same house with one more cat, not a second\n    animal: the store keeps one record per animal, so the number in that record is what moves.\n    A bare \"no, 3\" moves the count of the animal the line before it named, the way \"make that\n    800\" moves a budget, and only when that line gave a count for the same animal to replace -\n    \"i have a cat\" then \"no, 3\" says nothing about how many, so nothing is guessed at.\n    ");
    new = _count_said(&[target.clone()]);
    if py_truthy(&{ let __b = op_lt(&new.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_gt(&new.clone(), &Py::Int(30)) } }) {
    return Py::None;
    }
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&crate::memory::user(&[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&crate::memory::facts::extract(&[call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&crate::memory::facts::extract(&[said.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut animal: Py = __item.clone(); if py_truthy(&animal.clone()) {  __out.push(animal.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::None;
    }
    told = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&crate::memory::facts::extract(&[said.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&mcall(&pet.clone(), "get", &[py_str("count")])) {  __out.push(pet.clone()); } } py_list(__out) })() };
    old = if py_truthy(&told.clone()) { call_builtin("int", &[geti(&geti(&told.clone(), &Py::Int(0)), &py_str("count"))]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&old.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&old.clone(), &new.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&crate::memory::relations::_animal_family(&[mcall(&pet.clone(), "get", &[py_str("animal")])]), &crate::memory::relations::_animal_family(&[geti(&named.clone(), &Py::Int(0))])))) {
    continue;
    }
    let _ = py_setitem(&pet.clone(), &py_str("count"), new.clone());
    return py_dict(vec![(py_str("key"), py_str("pets_count")), (py_str("value"), _a_count(&[new.clone(), mcall(&pet.clone(), "get", &[py_str("animal")])])), (py_str("from"), _a_count(&[old.clone(), mcall(&pet.clone(), "get", &[py_str("animal")])]))]);
    }
    return Py::None;
    Py::None
}
pub fn _correct_the_count_on_the_table(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut counted: Py = Py::None;
    let mut known: Py = Py::None;
    let mut line: Py = Py::None;
    let mut new: Py = Py::None;
    let mut old: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Put a corrected count on what the line before it counted.\n\n    \"i have two kids\" then \"actually three\" is the same household with one more child: the store\n    already keeps the count under the thing counted, so a line that only says the new number\n    moves that number and the person does not have to name the thing again. Only a line that is\n    nothing but the new number counts, and only when the line before it gave one number - \"i have\n    two kids and 3 dogs\" then \"no, 4\" says nothing about which of them is 4. A count of an animal\n    never gets here: `_correct_count` reads it first and knows the animal the line named.\n    ");
    line = mcall(&g_BARE_NEW_COUNT(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    new = _count_said(&[mcall(&line.clone(), "group", &[py_str("count")])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = new.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(30)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&g_ANY_NUMBER(), "findall", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    return Py::None;
    }
    known = crate::memory::user(&[memory.clone()]);
    counted = mcall(&crate::memory::facts::extract(&[said.clone()]), "get", &[py_str("kids")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[counted.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[mcall(&known.clone(), "get", &[py_str("kids")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&counted.clone(), &new.clone()))) {
    return Py::None;
    }
    old = geti(&known.clone(), &py_str("kids"));
    let _ = crate::memory::remember(&[memory.clone(), py_str("kids"), new.clone(), text.clone()]);
    return py_dict(vec![(py_str("key"), py_str("kids")), (py_str("value"), _a_count(&[new.clone(), py_str("kid")])), (py_str("from"), _a_count(&[old.clone(), py_str("kid")]))]);
    Py::None
}
pub fn _a_fact_of_somebody_else(args: &[Py]) -> Py {
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
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut entry: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a plan is a date kept for another person (\"mums birthday on 2 april\"), not a plan.\n\n    The store says whose it is in `others`, so the person's own word for them (\"mum\", \"son\") in\n    the plan is what marks it: their birthday is a fact about them, and a trip of the person's own\n    is not.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = entry.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    continue;
    }
    word = geti(&mcall(&who.clone(), "split", &[]), &Py::Int(0));
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?:'?s)?\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_plans_a_journey_moves(args: &[Py]) -> Py {
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
    let mut plans: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plans"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut one: Py = Py::None;
    let _ = py_str("The plans a line about a journey can move, or None when the line names no journey.\n\n    None means the line names no going of that kind, so every date held is a candidate the way it\n    always was (\"make it july\", \"no, in august\").\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_JOURNEY_GOING(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])))) {
    return Py::None;
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_a_fact_of_somebody_else(&[memory.clone(), one.clone()])))) {  __out.push(one.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _correct_when(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut fixed: Py = Py::None;
    let mut group: Py = Py::None;
    let mut journeys: Py = Py::None;
    let mut known: Py = Py::None;
    let mut latest: Py = Py::None;
    let mut low: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut plans_list: Py = Py::None;
    let mut read_back: Py = Py::None;
    let mut told: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A month or a day is not a fact of its own: it lives inside the plan the line before it\n    stated, so \"my holiday is in june\" then \"make that july\" moves the holiday to July. The\n    old month is the one that plan holds, which is why the user does not have to name it.\n    ");
    low = mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::memory::facts::g_MONTHS(), &low.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::memory::facts::g_DAYS(), &low.clone())) } }) {
    return Py::None;
    }
    known = crate::memory::user(&[memory.clone()]);
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&p.clone(), "strip", &[]) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    latest = mcall(&known.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[latest.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&latest.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&plans.clone(), &latest.clone())) } } } }) {
    let _ = mcall(&plans.clone(), "append", &[latest.clone()]);
    }
    group = if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_MONTHS(), &low.clone()))) { crate::memory::facts::g_MONTHS() } else { crate::memory::facts::g_DAYS() };
    journeys = _the_plans_a_journey_moves(&[memory.clone(), plans.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_is(&journeys.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&journeys.clone()))) {
    return Py::None;
    }
    plans = journeys.clone();
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    plan = __item.clone();
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&plan.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&group.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&word.clone(), &low.clone())) } }) {
    fixed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\b"))); __args.push(target.clone()); __args.push(plan.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    told = _plan_told_line(&[memory.clone(), plan.clone(), fixed.clone(), text.clone()]);
    let _ = crate::memory::remember(&[memory.clone(), py_str("plan"), fixed.clone(), told.clone()]);
    plans_list = mcall(&known.clone(), "get", &[py_str("plans")]);
    if py_truthy(&call_builtin("isinstance", &[plans_list.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans_list.clone()) { let mut p: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&mcall(&call_builtin("str", &[p.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&plan.clone(), "lower", &[])))) { fixed.clone() } else { p.clone() }); } py_list(__out) })() });
    }
    read_back = crate::memory::plan_sentence(&[fixed.clone(), told.clone()]);
    if py_truthy(&mcall(&mcall(&read_back.clone(), "lower", &[]), "startswith", &[py_str("you're planning ")])) {
    read_back = py_str("");
    }
    return py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), mcall(&fixed.clone(), "rstrip", &[py_str(" .")])), (py_str("from"), op_add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("sentence"), read_back.clone())]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _date_said(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut day: Py = Py::None;
    let mut month: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The day and the month a value states, either of them None.");
    day = mcall(&g_DATE_DAY(), "search", &[low.clone()]);
    month = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::memory::facts::g_MONTHS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &name.clone()), &py_str("\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    month = name.clone();
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&day.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { month.clone() } }) {
    bare = mcall(&g_A_DAY_BEFORE_A_MONTH(), "search", &[low.clone()]);
    if py_truthy(&bare.clone()) {
    return py_tuple(vec![mcall(&bare.clone(), "group", &[Py::Int(1)]), month.clone()]);
    }
    }
    return py_tuple(vec![if py_truthy(&day.clone()) { mcall(&day.clone(), "group", &[Py::Int(0)]) } else { Py::None }, month.clone()]);
    Py::None
}
pub fn _a_day_in_the_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut old_day: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old_day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old_day"))) };
    let mut day: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut count: Py = Py::None;
    let mut number: Py = Py::None;
    let mut suffix: Py = Py::None;
    let _ = py_str("A day written the way the plan already writes its days: \"3 may\" put right stays a number.\n\n    \"my sons birthday is 3 may\" then \"sorry, its the 4th of may\" kept the plan's day as \"4th\",\n    which reads fine but is a different kind of day from the one the plan had, so the store's\n    own value changed shape for no reason. An ordinal plan keeps the ordinal, a plain one the\n    number.\n    ");
    number = stdlib_call("re", "match", &[py_str("(\\d{1,2})"), call_builtin("str", &[{ let __b = day.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&number.clone()))) {
    return _date_word_case(&[day.clone()]);
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\d{1,2}(?:st|nd|rd|th)$"), call_builtin("str", &[{ let __b = old_day.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    count = call_builtin("int", &[mcall(&number.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    suffix = if py_truthy(&{ let __cmp11: Py = op_modulo(&count.clone(), &Py::Int(100)); if py_truthy(&op_le(&Py::Int(11), &__cmp11)) { op_le(&__cmp11, &Py::Int(13)) } else { Py::Bool(false) } }) { py_str("th") } else { mcall(&py_dict(vec![(Py::Int(1), py_str("st")), (Py::Int(2), py_str("nd")), (Py::Int(3), py_str("rd"))]), "get", &[op_modulo(&count.clone(), &Py::Int(10)), py_str("th")]) };
    return op_add(&mcall(&number.clone(), "group", &[Py::Int(1)]), &suffix.clone());
    }
    return mcall(&number.clone(), "group", &[Py::Int(1)]);
    Py::None
}
pub fn _date_word_case(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("\"july\" is written \"July\", and a day said as \"13th\" is left as it is said.");
    text = call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&mcall(&slice_or_raise(&text.clone(), &Py::None, &Py::Int(1), &Py::None), "isalpha", &[])) { op_add(&mcall(&slice_or_raise(&text.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&text.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { text.clone() };
    Py::None
}
pub fn _date_in_plan(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let _ = py_str("The day and the month the plan already says, either of them None.");
    return _date_said(&[mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    Py::None
}
pub fn _the_weekday_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let _ = py_str("The weekday a plan says its day as, or None (\"the dentist is on tuesday at 9\").");
    return mcall(&g_WEEKDAY_IN_PLAN(), "search", &[call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _names_a_date(args: &[Py]) -> Py {
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
    let mut day: Py = Py::None;
    let mut month: Py = Py::None;
    let _ = py_str("True when a value states a date: a month, or a day of the month (\"5 june\", \"the 5th\").");
    let __unpk1 = py_unpack(&_date_said(&[mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]), 2, None);
    day = __unpk1[0].clone();
    month = __unpk1[1].clone();
    return call_builtin("bool", &[{ let __b = day.clone(); if py_truthy(&__b) { __b } else { month.clone() } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_relative_month(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut day: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut month: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut fixed: Py = Py::None;
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The plan with a relative month replaced by the month they named, and what moved.\n\n    \"Rome next month\" and \"no in july\" gives \"Rome in July\": the preposition is added with\n    the month, because the plan said when with no preposition of its own. A day with it\n    (\"no, the 12th of july\") writes both, and the relative words go.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&month.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[plan.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    found = mcall(&g_RELATIVE_MONTH(), "search", &[plan.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::None;
    }
    said = _date_word_case(&[month.clone()]);
    if py_truthy(&day.clone()) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), day.clone()), ("month".to_string(), said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("on the {day} of {month}"), "format", args))(&__args) };
    } else {
    said = op_add(&py_str("in "), &said.clone());
    }
    fixed = mcall(&op_add(&op_add(&slice_or_raise(&plan.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &said.clone()), &slice_or_raise(&plan.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    fixed = mcall(&py_str(" "), "join", &[mcall(&mcall(&fixed.clone(), "replace", &[py_str(" ,"), py_str(",")]), "split", &[])]);
    return py_tuple(vec![fixed.clone(), mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]), said.clone()]);
    Py::None
}
pub fn _a_relative_line(args: &[Py]) -> Py {
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
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut was: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "was") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: was"))) };
    let mut said: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut told: Py = Py::None;
    let _ = py_str("The line the plan was stated in, with the relative date said as the date they mean.\n\n    \"im going to rome next month\" and \"no in july\" gives \"im going to rome in July\", which\n    plan_sentence says back as \"You're going to rome in July\".\n    ");
    told = crate::memory::plan_line(&[memory.clone(), plan.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&told.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&was.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } } } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[was.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b"))); __args.push(said.clone()); __args.push(told.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _plan_saying(args: &[Py]) -> Py {
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
    let mut day: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut month: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut found_day: Py = Py::None;
    let mut found_month: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let _ = py_str("The plan in the store that says the date being taken back, or None.\n\n    \"im going to portugal in june\" / \"my sons birthday is in august\" / \"sorry, july not\n    june\" is the trip that is wrong, not the birthday: the value they name as wrong says\n    which plan the correction is about. The newest plan that says it wins.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&day.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&month.clone())) } }) {
    return Py::None;
    }
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(p.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    plan = __item.clone();
    let __unpk2 = py_unpack(&_date_in_plan(&[plan.clone()]), 2, None);
    found_day = __unpk2[0].clone();
    found_month = __unpk2[1].clone();
    if py_truthy(&{ let __b = month.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&found_month.clone(), &month.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = day.clone(); if !py_truthy(&__b) { __b } else { { let __b = found_day.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&found_day.clone(), &day.clone())) } } } }) {
    continue;
    }
    return plan.clone();
    }
    return Py::None;
    Py::None
}
pub fn _correct_date(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut wrong: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { py_str("") };
    let mut day: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut found: Py = Py::None;
    let mut gone_day: Py = Py::None;
    let mut gone_month: Py = Py::None;
    let mut join: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut month: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut old_day: Py = Py::None;
    let mut old_month: Py = Py::None;
    let mut p: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans_list: Py = Py::None;
    let mut read_back: Py = Py::None;
    let mut relative: Py = Py::None;
    let mut said: Py = Py::None;
    let mut saying: Py = Py::None;
    let mut told: Py = Py::None;
    let mut was: Py = Py::None;
    let mut weekday: Py = Py::None;
    let mut with_a_day: Py = Py::None;
    let _ = py_str("A date said again moves the date in the plan it belongs to, part by part.\n\n    \"im going to portugal on the 12th of june\" / \"sorry i meant the 13th of june\" moves the day;\n    \"sorry i meant the 13th\" moves the day and keeps June, because a month nobody took back is\n    still the month. Nothing is moved on a value that names no day and no month, and a plan that\n    says neither is not a plan this reader can fix.\n    ");
    low = mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __unpk1 = py_unpack(&_date_said(&[low.clone()]), 2, None);
    day = __unpk1[0].clone();
    month = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&day.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&month.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&_a_birthday_on_the_last_line(&[memory.clone()])) {
    return Py::None;
    }
    plan = { let __b = _stated_plan(&[memory.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::memory::user(&[memory.clone()]), "get", &[py_str("plan")]) } };
    let __unpk2 = py_unpack(&_date_said(&[mcall(&call_builtin("str", &[{ let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]), 2, None);
    gone_day = __unpk2[0].clone();
    gone_month = __unpk2[1].clone();
    saying = _plan_saying(&[memory.clone(), gone_day.clone(), gone_month.clone()]);
    if py_truthy(&saying.clone()) {
    plan = saying.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[plan.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&plan.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&_date_in_plan(&[plan.clone()]), 2, None);
    old_day = __unpk3[0].clone();
    old_month = __unpk3[1].clone();
    let __unpk4 = py_unpack(&py_tuple(vec![plan.clone(), py_list(vec![])]), 2, None);
    fixed = __unpk4[0].clone();
    moved = __unpk4[1].clone();
    told = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&old_day.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&old_month.clone())) } }) {
    weekday = _the_weekday_in(&[plan.clone()]);
    if py_truthy(&{ let __b = day.clone(); if !py_truthy(&__b) { __b } else { { let __b = month.clone(); if !py_truthy(&__b) { __b } else { weekday.clone() } } } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("day".to_string(), _date_word_case(&[day.clone()])), ("month".to_string(), _date_word_case(&[month.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("the {day} of {month}"), "format", args))(&__args) };
    fixed = mcall(&py_str(" "), "join", &[mcall(&op_add(&op_add(&slice_or_raise(&plan.clone(), &Py::None, &mcall(&weekday.clone(), "start", &[]), &Py::None), &said.clone()), &slice_or_raise(&plan.clone(), &mcall(&weekday.clone(), "end", &[]), &Py::None, &Py::None)), "split", &[])]);
    was = mcall(&mcall(&weekday.clone(), "group", &[Py::Int(0)]), "lower", &[]);
    let _ = mcall(&moved.clone(), "append", &[was.clone()]);
    } else {
    relative = _a_relative_month(&[plan.clone(), day.clone(), month.clone()]);
    if py_truthy(&Py::Bool(py_is(&relative.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk5 = py_unpack(&relative.clone(), 3, None);
    fixed = __unpk5[0].clone();
    was = __unpk5[1].clone();
    said = __unpk5[2].clone();
    let _ = mcall(&moved.clone(), "append", &[was.clone()]);
    }
    told = _a_relative_line(&[memory.clone(), plan.clone(), was.clone(), said.clone()]);
    }
    if py_truthy(&{ let __b = day.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&old_day.clone())); if !py_truthy(&__b) { __b } else { old_month.clone() } } } }) {
    with_a_day = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(in|on|for|by|until|till|from|around|before|after)\\s+(?:the\\s+)?"), &stdlib_call("re", "escape", &[old_month.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    found = mcall(&with_a_day.clone(), "search", &[fixed.clone()]);
    if py_truthy(&found.clone()) {
    join = if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), &py_str("in")))) { py_str("on") } else { mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]) };
    fixed = op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&slice_or_raise(&fixed.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &join.clone()), &py_str(" the ")), &_date_word_case(&[day.clone()])), &py_str(" of ")), &_date_word_case(&[old_month.clone()])), &slice_or_raise(&fixed.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None));
    let _ = mcall(&moved.clone(), "append", &[old_month.clone()]);
    }
    }
    if py_truthy(&{ let __b = day.clone(); if !py_truthy(&__b) { __b } else { { let __b = old_day.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&day.clone(), "lower", &[]), &mcall(&old_day.clone(), "lower", &[]))) } } } }) {
    fixed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &old_day.clone()), &py_str("\\b"))); __args.push(_a_day_in_the_kind(&[old_day.clone(), day.clone()])); __args.push(fixed.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let _ = mcall(&moved.clone(), "append", &[old_day.clone()]);
    }
    if py_truthy(&{ let __b = month.clone(); if !py_truthy(&__b) { __b } else { { let __b = old_month.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&month.clone(), &old_month.clone())) } } } }) {
    fixed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &old_month.clone()), &py_str("\\b"))); __args.push(_date_word_case(&[month.clone()])); __args.push(fixed.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let _ = mcall(&moved.clone(), "append", &[old_month.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&fixed.clone(), &plan.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&told.clone(), &Py::None))) {
    told = _plan_told_line(&[memory.clone(), plan.clone(), fixed.clone(), text.clone()]);
    }
    let _ = crate::memory::remember(&[memory.clone(), py_str("plan"), fixed.clone(), told.clone()]);
    known = crate::memory::user(&[memory.clone()]);
    plans_list = mcall(&known.clone(), "get", &[py_str("plans")]);
    if py_truthy(&call_builtin("isinstance", &[plans_list.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans_list.clone()) { let mut p: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&mcall(&call_builtin("str", &[p.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&plan.clone(), "lower", &[])))) { fixed.clone() } else { p.clone() }); } py_list(__out) })() });
    }
    read_back = crate::memory::plan_sentence(&[fixed.clone(), told.clone()]);
    if py_truthy(&mcall(&mcall(&read_back.clone(), "lower", &[]), "startswith", &[py_str("you're planning ")])) {
    read_back = py_str("");
    }
    return py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), mcall(&fixed.clone(), "rstrip", &[py_str(" .")])), (py_str("from"), mcall(&py_str(" "), "join", &[moved.clone()])), (py_str("sentence"), read_back.clone())]);
    Py::None
}
pub fn _a_name_the_store_keeps(args: &[Py]) -> Py {
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
    let mut taken_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "taken_back") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: taken_back"))) };
    let mut new: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut before: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut old: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("The correction that moves the name of the one person the store keeps under it.\n\n    \"my sister is called anna and my brother is called tom\" then \"its hannah not anna\": the name\n    taken back is on record for the sister, so it is her name that moves - the person named last\n    in the chat is not the one being put right.\n    ");
    let __iter1 = py_iter_or_raise(&crate::memory::others(&[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&entry.clone(), "get", &[py_str("name")]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&call_builtin("str", &[geti(&entry.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &taken_back.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_name_for_a_person(&[new.clone()])))) {
    return Py::None;
    }
    before = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&entry.clone(), &py_str("name"), new.clone());
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&before.clone(), "lower", &[]), &mcall(&new.clone(), "lower", &[]))) } }) {
    was = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut old: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&new.clone(), "lower", &[])))) {  __out.push(call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    let _ = py_setitem(&entry.clone(), &py_str("was"), op_add(&was.clone(), &py_list(vec![before.clone()])));
    }
    let _ = mcall(&entry.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&crate::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[entry.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), op_modulo(&py_str("my %s is called %s"), &py_tuple(vec![mcall(&entry.clone(), "get", &[py_str("who")]), new.clone()])))])]);
    return py_dict(vec![(py_str("key"), py_str("person_name")), (py_str("value"), new.clone()), (py_str("wrong"), op_add(&mcall(&slice_or_raise(&taken_back.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&taken_back.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("from"), taken_back.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _stated_last(args: &[Py]) -> Py {
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
    let mut by_line: Py = Py::None;
    let mut entries: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The key of the fact the correction fixes, or None.\n\n    A correction fixes the thing just said, and that is usually the previous line. It is not\n    always: people ask about a fact first and correct it once they hear it back (\"im a nurse\",\n    \"whats my job?\", \"no i mean a teacher\"). So the lines are walked back over the questions\n    between, and the search stops at the first line that said something of its own - a fact that\n    cannot be corrected, or a deed - because the person has moved on and an older age is not what\n    \"no I mean 43\" is about.\n    ");
    entries = mcall(&memory.clone(), "get", &[py_str("facts_log")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entries.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    by_line = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&entries.clone());
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let _ = mcall(&mcall(&by_line.clone(), "setdefault", &[said.clone(), py_list(vec![])]), "append", &[mcall(&entry.clone(), "get", &[py_str("key")])]);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = crate::memory::last_user(&[memory.clone(), crate::memory::store_lines::g_CORRECTION_BACK()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    line = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    keys = mcall(&by_line.clone(), "get", &[said.clone()]);
    if py_truthy(&keys.clone()) {
    let __iter3 = py_iter_or_raise(&call_builtin("reversed", &[keys.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    key = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_CORRECTABLE(), &key.clone()))) {
    return key.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&_asks(&[said.clone()])))) {
    return Py::None;
    }
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_asks(&[said.clone()])))) {
    return Py::None;
    }
    }
    return Py::None;
    Py::None
}
pub fn _plan_told_line(args: &[Py]) -> Py {
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
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: old"))) };
    let mut new: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "new") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: new"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut now: Py = Py::None;
    let mut told: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("The line to keep with a plan whose date was changed.\n\n    A plan is said back in the words it was stated in (\"You're going to portugal in July\", not\n    \"You're planning portugal in July\"), and the line they stated it in still holds the date\n    they said first, so the date moves in the line too. Only a change of the date carries the\n    original line: correcting the thing itself (\"sorry, i meant porto\") is a new line, and the\n    old one would name the place they have just corrected.\n    ");
    told = crate::memory::plan_line(&[memory.clone(), old.clone()]);
    let __unpk1 = py_unpack(&py_tuple(vec![_date_word(&[old.clone()]), _date_word(&[new.clone()])]), 2, None);
    was = __unpk1[0].clone();
    now = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&told.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&was.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&now.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&was.clone(), "lower", &[]), &mcall(&now.clone(), "lower", &[]))) } } } } } }) {
    return text.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[was.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b"))); __args.push(now.clone()); __args.push(told.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _date_word(args: &[Py]) -> Py {
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
    let _ = py_str("The month or day a phrase names, as it was written there, or \"\" - the value a change moves.");
    match_ = { let __b = mcall(&g_MONTH_WORD(), "search", &[call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { mcall(&g_DAY_WORD(), "search", &[call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } };
    return if py_truthy(&match_.clone()) { mcall(&match_.clone(), "group", &[Py::Int(0)]) } else { py_str("") };
    Py::None
}
pub fn _kind_of_value(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut target: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut told: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "told") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: told"))) };
    let mut low: Py = Py::None;
    let _ = py_str("What part of the fact's sentence a corrected value is a value of.\n\n    \"August\" after \"we're going to Lisbon in May\" is the month, so the month moves and Lisbon\n    stays. \"500\" after \"400 pounds\" is the amount, so the pounds stay. A bare number after\n    \"leave for the airport at 4\" is the time. Anything else names the thing the line was\n    about, and the place it replaces is the one the sentence carries.\n    ");
    low = mcall(&target.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_MONTHS(), &low.clone()))) {
    return py_str("month");
    }
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_DAYS(), &low.clone()))) {
    return py_str("day");
    }
    if py_truthy(&{ let __b = mcall(&g_AMOUNT(), "fullmatch", &[target.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_AT_TIME(), "search", &[told.clone()]))) } }) {
    return py_str("amount");
    }
    if py_truthy(&{ let __b = mcall(&g_AT_TIME(), "search", &[told.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_AMOUNT(), "fullmatch", &[target.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NUMBER_WORD(), "fullmatch", &[target.clone()]) } } } }) {
    return py_str("time");
    }
    if py_truthy(&mcall(&g_DATE_PHRASE(), "search", &[target.clone()])) {
    return py_str("date");
    }
    return Py::None;
    Py::None
}
pub fn _swap_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut told: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "told") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: told"))) };
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut wrong: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut half: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut last: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The fact's own sentence with the corrected value in place of the one it corrects.\n\n    \"A trip to Lisbon in May\" and \"sorry, I meant Porto\" (or \"no, Porto\") tells the whole\n    sentence: only the place moved. \"400 pounds\" and \"I meant 500\" is 500 pounds, because the\n    pounds are still how the budget is counted. A value only moves the part of the sentence it\n    is a value of: \"I meant August\" moves the month, not the city. Nothing is swapped when the\n    sentence does not carry the thing being corrected, so a correction about something else\n    leaves the fact be.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[told.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&told.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&target.clone())) } } } }) {
    return Py::None;
    }
    target = mcall(&call_builtin("str", &[target.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    kind = _kind_of_value(&[target.clone(), told.clone()]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("month"), py_str("day")]), &kind.clone()))) {
    wanted = if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("month")))) { g_MONTH_WORD() } else { g_DAY_WORD() };
    found = call_builtin("list", &[mcall(&wanted.clone(), "finditer", &[told.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    last = geti(&found.clone(), &op_neg(&Py::Int(1)));
    return op_add(&op_add(&slice_or_raise(&told.clone(), &Py::None, &mcall(&last.clone(), "start", &[]), &Py::None), &target.clone()), &slice_or_raise(&told.clone(), &mcall(&last.clone(), "end", &[]), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("amount")))) {
    found = call_builtin("list", &[mcall(&g_AMOUNT(), "finditer", &[told.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    last = geti(&found.clone(), &op_neg(&Py::Int(1)));
    unit = mcall(&g_CURRENCY(), "match", &[slice_or_raise(&told.clone(), &mcall(&last.clone(), "start", &[]), &Py::None, &Py::None)]);
    tail = slice_or_raise(&told.clone(), &mcall(&last.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&{ let __b = unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CURRENCY(), "match", &[target.clone()]))) } }) {
    target = op_add(&op_add(&mcall(&mcall(&unit.clone(), "group", &[Py::Int(0)]), "strip", &[]), &if py_truthy(&mcall(&mcall(&unit.clone(), "group", &[Py::Int(0)]), "endswith", &[py_str(" ")])) { py_str(" ") } else { py_str("") }), &target.clone());
    } else {
    if py_truthy(&mcall(&g_CURRENCY(), "match", &[target.clone()])) {
    tail = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:pounds?|dollars?|euros?|quid|bucks?)\\b")); __args.push(py_str("")); __args.push(tail.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return op_add(&op_add(&slice_or_raise(&told.clone(), &Py::None, &mcall(&last.clone(), "start", &[]), &Py::None), &target.clone()), &tail.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("date")))) {
    found = call_builtin("list", &[mcall(&g_DATE_PHRASE(), "finditer", &[told.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    last = geti(&found.clone(), &op_neg(&Py::Int(1)));
    said = if py_truthy(&mcall(&mcall(&target.clone(), "lower", &[]), "startswith", &[py_tuple(vec![py_str("the "), py_str("a "), py_str("an ")])])) { mcall(&target.clone(), "lower", &[]) } else { target.clone() };
    return op_add(&op_add(&slice_or_raise(&told.clone(), &Py::None, &mcall(&last.clone(), "start", &[]), &Py::None), &said.clone()), &slice_or_raise(&told.clone(), &mcall(&last.clone(), "end", &[]), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("time")))) {
    found = call_builtin("list", &[mcall(&g_AT_TIME(), "finditer", &[told.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    last = geti(&found.clone(), &op_neg(&Py::Int(1)));
    said = mcall(&last.clone(), "group", &[Py::Int(0)]);
    half = stdlib_call("re", "search", &[py_str("(am|pm|o'?clock)\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = half.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("am|pm|o'?clock|noon|midday|midnight"), target.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    target = op_add(&op_add(&target.clone(), &if py_truthy(&mcall(&slice_or_raise(&target.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None), "isdigit", &[])) { py_str("") } else { py_str(" ") }), &mcall(&half.clone(), "group", &[Py::Int(1)]));
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("^at\\s+"), &crate::memory::store_lines::g_AT_TIME_VALUE())); __args.push(op_add(&py_str("at "), &target.clone())); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return op_add(&op_add(&slice_or_raise(&told.clone(), &Py::None, &mcall(&last.clone(), "start", &[]), &Py::None), &rest.clone()), &slice_or_raise(&told.clone(), &mcall(&last.clone(), "end", &[]), &Py::None, &Py::None));
    }
    if py_truthy(&wrong.clone()) {
    named = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[wrong.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&named.clone(), "search", &[told.clone()])))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(target.clone()); __args.push(told.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "sub", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_AMOUNT(), "fullmatch", &[target.clone()])))) {
    return Py::None;
    }
    found = call_builtin("list", &[mcall(&g_AMOUNT(), "finditer", &[told.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    last = geti(&found.clone(), &op_neg(&Py::Int(1)));
    return op_add(&op_add(&slice_or_raise(&told.clone(), &Py::None, &mcall(&last.clone(), "start", &[]), &Py::None), &target.clone()), &slice_or_raise(&told.clone(), &mcall(&last.clone(), "end", &[]), &Py::None, &Py::None));
    Py::None
}
pub fn _plan_the_line_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plans: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plans"))) };
    let mut told: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "told") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: told"))) };
    let mut low: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The plan in the list that the corrected sentence was kept as, or None.\n\n    The plan is stored as the phrase the person used (\"Portugal in june\"), while the line in the\n    log is the whole sentence (\"im going to portugal in june\"), so the one that came from this\n    sentence is the entry one holds the other. Left in the list, it is a plan they have already\n    corrected, and \"what am I up to?\" reads both.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = told.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&plans.clone());
    for __item in __iter1 {
    plan = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[plan.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&said.clone(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&low.clone(), &said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &low.clone())) } } } } } }) {
    return plan.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _carries(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut said: Py = Py::None;
    let _ = py_str("Whether a kept value's own wording carries `word` as a word.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    word = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = word.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _fact_naming(args: &[Py]) -> Py {
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
    let mut wrong: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wrong"))) };
    let mut entries: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("(key, sentence) of the fact whose own wording carries the value being corrected.\n\n    \"im going to portugal in june\" / \"my sons birthday is in august\" / \"sorry, july not june\":\n    the month they named as wrong is in the trip, so the trip is what moves. The newest stated\n    fact is the birthday, and correcting the newest one moves a plan the line never mentioned.\n    Only a value the person actually names gets this reading; \"make that july\" names nothing and\n    the fact to correct is the one just said.\n    ");
    value = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    entries = mcall(&memory.clone(), "get", &[py_str("facts_log")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&value.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[entries.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    pattern = stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_CORRECTABLE(), &mcall(&entry.clone(), "get", &[py_str("key")]))) } }) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&pattern.clone(), "search", &[said.clone()])) {
    return py_tuple(vec![mcall(&entry.clone(), "get", &[py_str("key")]), said.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_birthday_on_the_last_line(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line before this one stated a birthday, whichever entry it was logged as.\n\n    \"my birthday is in may\" is read as their birthday and as the date it names, so the log holds\n    both. A correction of the date is a correction of the birthday, and the plan the same line was\n    kept as is not what moves.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &said.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &py_str("birthday")))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _correct_birthday(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut day: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let mut known: Py = Py::None;
    let mut month: Py = Py::None;
    let mut old_day: Py = Py::None;
    let mut old_month: Py = Py::None;
    let mut told: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The birthday a person gives again is the one the store keeps.\n\n    \"my birthday is 3rd may\" / \"sorry, its the 4th may\" left the day as it first said it, so\n    \"when is my birthday\" answered with the date they had just taken back. The date said now\n    replaces the one said before, and a line that names only the month or only the day moves that\n    part of it and keeps the rest - \"sorry, the 4th\" keeps May, because a month nobody took back\n    is still the month. A line that names no date at all leaves the birthday alone.\n    ");
    known = crate::memory::user(&[memory.clone()]);
    held = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("birthday")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    let __unpk1 = py_unpack(&_date_said(&[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]), 2, None);
    day = __unpk1[0].clone();
    month = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&day.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&month.clone())) } }) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&_date_said(&[mcall(&held.clone(), "lower", &[])]), 2, None);
    old_day = __unpk2[0].clone();
    old_month = __unpk2[1].clone();
    day = { let __b = day.clone(); if py_truthy(&__b) { __b } else { old_day.clone() } };
    month = { let __b = month.clone(); if py_truthy(&__b) { __b } else { old_month.clone() } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&day.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&month.clone())) } }) {
    return Py::None;
    }
    value = if py_truthy(&{ let __b = day.clone(); if !py_truthy(&__b) { __b } else { month.clone() } }) { op_add(&op_add(&day.clone(), &py_str(" of ")), &month.clone()) } else { { let __b = month.clone(); if py_truthy(&__b) { __b } else { day.clone() } } };
    if py_truthy(&Py::Bool(py_eq(&mcall(&value.clone(), "lower", &[]), &mcall(&held.clone(), "lower", &[])))) {
    return Py::None;
    }
    told = text.clone();
    let __iter3 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("facts_log")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    entry = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &py_str("birthday"))) } }) {
    told = { let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { text.clone() } };
    break;
    }
    }
    let _ = crate::memory::remember(&[memory.clone(), py_str("birthday"), value.clone(), told.clone()]);
    return py_dict(vec![(py_str("key"), py_str("birthday")), (py_str("value"), value.clone()), (py_str("sentence"), _birthday_words(&[value.clone()]))]);
    Py::None
}
pub fn _a_birthday_for_somebody(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The person the line before this one gave a birthday to, as the store keeps them, or None.\n\n    \"my daughters birthday is the 3 may\" is filed on the daughter rather than as a plan, so a\n    date said again after it has to be moved on the person - the person's own birthday goes\n    through `_correct_birthday`, and this is that same seam one person along.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&said.clone(), &py_str("birthday"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &py_str("born"))) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[crate::memory::others(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&entry.clone(), "get", &[py_str("birthday")]))) } }) {
    continue;
    }
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    return entry.clone();
    }
    return Py::None;
    Py::None
}
pub fn _the_line_is_about_that_birthday(args: &[Py]) -> Py {
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
    let mut held: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: held"))) };
    let mut target: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut left: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut value: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a line naming a date is about the birthday the line before it gave.\n\n    \"no the 4th\" and \"sorry, september\" say nothing but the value, so the birthday on the table\n    is what they move. A line with a subject and a verb of its own is about that thing instead:\n    \"my sons birthday is 3 may\" then \"actually we fly in september\" moved the birthday to 3\n    September and the flight stayed in June, because a month said again was read as a correction\n    of the last date in the chat whatever the line was about. Naming the person or the birthday\n    again keeps the birthday the subject.\n    ");
    value = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NO_SUBJECT_OF_ITS_OWN(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&value.clone(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&left.clone()))) {
    return Py::Bool(true);
    }
    who = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = held.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    theirs = if py_truthy(&who.clone()) { py_set(vec![who.clone(), op_add(&who.clone(), &py_str("s"))]) } else { call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)) };
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_STILL_THE_SAME_PERSON(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&theirs.clone(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _correct_their_birthday(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut wrong: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wrong"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut bare: Py = Py::None;
    let mut day: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut gone_day: Py = Py::None;
    let mut gone_month: Py = Py::None;
    let mut held: Py = Py::None;
    let mut known: Py = Py::None;
    let mut line: Py = Py::None;
    let mut link: Py = Py::None;
    let mut low: Py = Py::None;
    let mut month: Py = Py::None;
    let mut old_day: Py = Py::None;
    let mut old_month: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut value: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("A date said again after somebody else's birthday is that person's birthday, moved.\n\n    \"my daughters birthday is the 3 may\" / \"no the 4th\" kept the 3rd and answered \"Got it. Is\n    there anything you'd like a hand with?\", because a bare day names no fact of its own: the\n    birthday is kept on the daughter, and the reader for it only knew the person using the bot.\n    The day said here moves the day and keeps the month nobody took back - the same rule the\n    person's own birthday follows - and the line is said back about them, not about the user.\n\n    Nothing moves when the value they take back is not part of that birthday: \"im going to\n    portugal in june\" / \"my sons birthday is in august\" / \"sorry, july not june\" is the trip\n    that is wrong, and the birthday the line after it gave is not the thing being corrected.\n    ");
    held = _a_birthday_for_somebody(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return Py::None;
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = target.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __unpk1 = py_unpack(&_date_said(&[low.clone()]), 2, None);
    day = __unpk1[0].clone();
    month = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&day.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&month.clone())) } }) {
    return Py::None;
    }
    known = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("birthday")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __unpk2 = py_unpack(&_date_said(&[known.clone()]), 2, None);
    old_day = __unpk2[0].clone();
    old_month = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&old_day.clone()))) {
    bare = stdlib_call("re", "search", &[py_str("\\b(\\d{1,2})\\b"), known.clone()]).unwrap_or_else(|e| exc::raise(e));
    old_day = if py_truthy(&bare.clone()) { mcall(&bare.clone(), "group", &[Py::Int(1)]) } else { Py::None };
    }
    let __unpk3 = py_unpack(&_date_said(&[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = wrong.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]), 2, None);
    gone_day = __unpk3[0].clone();
    gone_month = __unpk3[1].clone();
    if py_truthy(&{ let __b = { let __b = gone_day.clone(); if py_truthy(&__b) { __b } else { gone_month.clone() } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let __b = gone_day.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&gone_day.clone(), &old_day.clone())) } }; if py_truthy(&__b) { __b } else { { let __b = gone_month.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&gone_month.clone(), &old_month.clone())) } } } })) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_line_is_about_that_birthday(&[text.clone(), held.clone(), target.clone()])))) {
    return Py::None;
    }
    day = { let __b = day.clone(); if py_truthy(&__b) { __b } else { old_day.clone() } };
    month = { let __b = month.clone(); if py_truthy(&__b) { __b } else { old_month.clone() } };
    value = if py_truthy(&{ let __b = day.clone(); if !py_truthy(&__b) { __b } else { month.clone() } }) { op_add(&op_add(&day.clone(), &if py_truthy(&Py::Bool(py_contains(&known.clone(), &py_str(" of ")))) { py_str(" of ") } else { py_str(" ") }), &month.clone()) } else { { let __b = month.clone(); if py_truthy(&__b) { __b } else { day.clone() } } };
    if py_truthy(&Py::Bool(py_eq(&mcall(&value.clone(), "lower", &[]), &mcall(&known.clone(), "lower", &[])))) {
    return Py::None;
    }
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    entry = py_dict(vec![(py_str("who"), who.clone()), (py_str("birthday"), value.clone())]);
    if py_truthy(&mcall(&held.clone(), "get", &[py_str("name")])) {
    let _ = py_setitem(&entry.clone(), &py_str("name"), mcall(&held.clone(), "get", &[py_str("name")]));
    }
    let _ = crate::memory::remember_other(&[memory.clone(), entry.clone(), { let __b = crate::memory::last_user(&[memory.clone()]); if py_truthy(&__b) { __b } else { text.clone() } }]);
    line = op_add(&op_add(&op_add(&op_add(&py_str("my "), &who.clone()), &py_str("s birthday is ")), &if py_truthy(&day.clone()) { py_str("on ") } else { py_str("in ") }), &value.clone());
    link = if py_truthy(&day.clone()) { py_str("on ") } else { py_str("") };
    said_back = op_add(&op_add(&op_add(&op_add(&py_str("your "), &who.clone()), &py_str("s birthday is ")), &link.clone()), &_birthday_words(&[value.clone()]));
    return py_dict(vec![(py_str("key"), py_str("birthday")), (py_str("value"), value.clone()), (py_str("from"), known.clone()), (py_str("line"), line.clone()), (py_str("note"), op_add(&op_add(&py_str("Got it — "), &said_back.clone()), &py_str(".")))]);
    Py::None
}
pub fn _correct_fact(args: &[Py]) -> Py {
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
    let mut target: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut wrong: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wrong"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut counted: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut holder: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut named: Py = Py::None;
    let mut note: Py = Py::None;
    let mut old: Py = Py::None;
    let mut origin: Py = Py::None;
    let mut p: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut replaced: Py = Py::None;
    let mut said: Py = Py::None;
    let mut swapped: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut told: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Replace the fact the previous line stated with the value the user corrected it to.\n\n    The value is read in the shape its fact arrives in (\"i live in Bristol\", \"i am 43\", \"my\n    name is Jen\"), and kept only when it reads as that same kind of fact. A wrong first value\n    stays in the list of places, because \"which did I say first?\" still has an answer.\n    ");
    theirs = _correct_their_birthday(&[memory.clone(), target.clone(), wrong.clone(), text.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    known = crate::memory::user(&[memory.clone()]);
    counted = crate::memory::facts::pet_counts(&[target.clone()]);
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(p.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = counted.clone(); if !py_truthy(&__b) { __b } else { pets.clone() } }) {
    moved = py_str("");
    let __iter1 = py_iter_or_raise(&counted.clone());
    for __item in __iter1 {
    said = __item.clone();
    let __iter2 = py_iter_or_raise(&pets.clone());
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&crate::memory::relations::_animal_family(&[mcall(&pet.clone(), "get", &[py_str("animal")])]), &crate::memory::relations::_animal_family(&[geti(&said.clone(), &py_str("animal"))])))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&pet.clone(), "get", &[py_str("count")]), &geti(&said.clone(), &py_str("count"))))) {
    continue;
    }
    let _ = py_setitem(&pet.clone(), &py_str("count"), geti(&said.clone(), &py_str("count")));
    moved = _many(&[geti(&said.clone(), &py_str("count")), { let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("pet") } }]);
    }
    }
    if py_truthy(&moved.clone()) {
    let _ = py_setitem(&known.clone(), &py_str("pets"), pets.clone());
    return py_dict(vec![(py_str("key"), py_str("pets")), (py_str("value"), moved.clone())]);
    }
    }
    named = _fact_naming(&[memory.clone(), wrong.clone()]);
    key = if py_truthy(&named.clone()) { geti(&named.clone(), &Py::Int(0)) } else { _stated_last(&[memory.clone()]) };
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("city"), py_str("places")]), &key.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::memory::facts::place_value(&[target.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&key.clone(), &py_str("birthday"))); if !py_truthy(&__b) { __b } else { _a_birthday_on_the_last_line(&[memory.clone()]) } }) {
    key = py_str("birthday");
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("birthday")))) {
    return _correct_birthday(&[memory.clone(), target.clone(), text.clone()]);
    }
    known = crate::memory::user(&[memory.clone()]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("pet"), py_str("pets")]), &key.clone()))) {
    pets = mcall(&known.clone(), "get", &[py_str("pets")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pets.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter3 = py_iter_or_raise(&call_builtin("reversed", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("name")]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = wrong.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&call_builtin("str", &[mcall(&pet.clone(), "get", &[py_str("name")])]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&wrong.clone(), "lower", &[]))) } }) {
    continue;
    }
    let _ = py_setitem(&pet.clone(), &py_str("name"), crate::memory::relations::_tidy_name(&[target.clone()]));
    return py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), geti(&pet.clone(), &py_str("name"))), (py_str("animal"), { let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("pet") } })]);
    }
    return Py::None;
    }
    told = Py::None;
    held = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_carries(&[held.clone(), wrong.clone()]))) } }) {
    told = geti(&named.clone(), &Py::Int(1));
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("budget"), py_str("plan")]), &key.clone()))) {
    replaced = wrong.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&replaced.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&key.clone(), &py_str("plan"))) } }) {
    places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(p.clone()); } } py_list(__out) })() };
    replaced = if py_truthy(&places.clone()) { geti(&places.clone(), &op_neg(&Py::Int(1))) } else { Py::None };
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("plan"))); if !py_truthy(&__b) { __b } else { { let __b = replaced.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::memory::facts::place_value(&[call_builtin("str", &[target.clone()]).unwrap_or_else(|e| exc::raise(e))]))) } } } }) {
    return Py::None;
    }
    old = if py_truthy(&Py::Bool(!py_is(&told.clone(), &Py::None))) { told.clone() } else { mcall(&known.clone(), "get", &[key.clone()]) };
    swapped = _swap_in(&[old.clone(), target.clone(), replaced.clone()]);
    if py_truthy(&swapped.clone()) {
    said = text.clone();
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plan")))) {
    origin = crate::memory::plan_line(&[memory.clone(), old.clone()]);
    said = { let __b = _swap_in(&[origin.clone(), target.clone(), replaced.clone()]); if py_truthy(&__b) { __b } else { _plan_told_line(&[memory.clone(), old.clone(), swapped.clone(), text.clone()]) } };
    }
    let _ = crate::memory::remember(&[memory.clone(), key.clone(), swapped.clone(), said.clone()]);
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plan")))) {
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(p.clone()); } } py_list(__out) })() };
    holder = _plan_the_line_named(&[plans.clone(), old.clone()]);
    if py_truthy(&Py::Bool(!py_is(&holder.clone(), &Py::None))) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut p: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&p.clone(), &holder.clone()))) { swapped.clone() } else { p.clone() }); } py_list(__out) })() });
    } else {
    if py_truthy(&Py::Bool(!py_contains(&plans.clone(), &swapped.clone()))) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), slice_or_raise(&op_add(&plans.clone(), &py_list(vec![swapped.clone()])), &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    }
    }
    places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(p.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = replaced.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&_kind_of_value(&[target.clone(), old.clone()]), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut p: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&p.clone(), "lower", &[]), &mcall(&call_builtin("str", &[replaced.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut p: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&p.clone(), "lower", &[]), &mcall(&target.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } } } }) {
    let _ = py_setitem(&known.clone(), &py_str("places"), op_add(&places.clone(), &py_list(vec![target.clone()])));
    }
    }
    note = swapped.clone();
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plan")))) {
    note = { let __b = crate::memory::plan_sentence(&[swapped.clone(), said.clone()]); if py_truthy(&__b) { __b } else { swapped.clone() } };
    if py_truthy(&mcall(&mcall(&note.clone(), "lower", &[]), "startswith", &[py_str("you're planning ")])) {
    note = swapped.clone();
    }
    }
    return py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), target.clone()), (py_str("sentence"), note.clone())]);
    }
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[held.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&_kind_of_value(&[target.clone(), held.clone()]), &Py::None)) } }) {
    return Py::None;
    }
    found = crate::memory::facts::reread(&[target.clone(), key.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("city"), py_str("places")]), &key.clone()))) {
    place = { let __b = mcall(&found.clone(), "get", &[py_str("city")]); if py_truthy(&__b) { __b } else { geti(&{ let __b = mcall(&found.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![Py::None]) } }, &Py::Int(0)) } };
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&known.clone(), &py_str("places"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(p.clone()); } } py_list(__out) })() });
    let _ = crate::memory::remember(&[memory.clone(), py_str("places"), py_list(vec![place.clone()]), text.clone()]);
    let _ = crate::memory::remember(&[memory.clone(), py_str("city"), place.clone(), text.clone()]);
    return py_dict(vec![(py_str("key"), py_str("city")), (py_str("value"), place.clone())]);
    }
    value = mcall(&found.clone(), "get", &[key.clone()]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    value = if py_truthy(&value.clone()) { geti(&value.clone(), &Py::Int(0)) } else { Py::None };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str(""), py_list(vec![]), py_dict(vec![])]), &value.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("budget")))) {
    value = _keep_unit(&[value.clone(), mcall(&known.clone(), "get", &[py_str("budget")])]);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("allergies"), py_str("people"), py_str("conditions")]), &key.clone()))) {
    let _ = _drop_wrong(&[known.clone(), key.clone(), wrong.clone(), target.clone()]);
    }
    let _ = crate::memory::remember(&[memory.clone(), key.clone(), value.clone(), text.clone()]);
    return py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), value.clone()), (py_str("from"), wrong.clone())]);
    Py::None
}
pub fn _keep_unit(args: &[Py]) -> Py {
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
    let mut was: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "was") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: was"))) };
    let mut unit: Py = Py::None;
    let _ = py_str("\"no, i mean 500\" after \"my budget is 300 pounds\" is still pounds: a bare number takes the\n    unit the fact it replaces was said in.");
    value = mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&value.clone())); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("[a-z]"), value.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return value.clone();
    }
    unit = stdlib_call("re", "search", &[py_str("([a-z]+)\\s*$"), mcall(&call_builtin("str", &[{ let __b = was.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&unit.clone()) { op_add(&op_add(&value.clone(), &py_str(" ")), &mcall(&mcall(&unit.clone(), "group", &[Py::Int(1)]), "lower", &[])) } else { value.clone() };
    Py::None
}
pub fn _drop_wrong(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut known: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut wrong: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrong") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wrong"))) };
    let mut target: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "target") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: target"))) };
    let mut doomed: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut left: Py = Py::None;
    let mut v: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("A correction said which value was wrong (\"tree nuts, not peanuts\"): that one goes.\n\n    It is the difference between replacing a fact and adding to it, and it only ever touches a\n    list of things the person named, never the history of where they have lived.\n    ");
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut v: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[v.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(v.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    return Py::None;
    }
    doomed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![wrong.clone(), target.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(mcall(&call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_set(__out) })() };
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut v: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&doomed.clone(), &mcall(&v.clone(), "lower", &[])))) {  __out.push(v.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&left.clone(), &kept.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    left = py_list(vec![]);
    }
    let _ = py_setitem(&known.clone(), &key.clone(), left.clone());
    Py::None
}
pub fn _birthday_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut date: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "date") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: date"))) };
    let mut day: Py = Py::None;
    let mut month: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The date the way it reads in a sentence: \"3rd of may\" -> \"the 3rd of May\".");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[date.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    parts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&said.clone(), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    let _ = mcall(&parts.clone(), "append", &[if py_truthy(&Py::Bool(py_contains(&g_MONTH_WORDS(), &word.clone()))) { mcall(&word.clone(), "capitalize", &[]) } else { word.clone() }]);
    }
    said = mcall(&py_str(" "), "join", &[parts.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\d"), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return op_add(&py_str("in "), &said.clone());
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("\\d{1,2}(?:st|nd|rd|th)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return said.clone();
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\d{1,2}(?:st|nd|rd|th)\\s+[A-Z][a-z]+$"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __unpk2 = py_unpack(&mcall(&said.clone(), "split", &[]), 2, None);
    day = __unpk2[0].clone();
    month = __unpk2[1].clone();
    return op_modulo(&py_str("the %s of %s"), &py_tuple(vec![day.clone(), month.clone()]));
    }
    return op_add(&py_str("the "), &said.clone());
    Py::None
}
pub fn _many(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut count: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut animal: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut noun: Py = Py::None;
    let _ = py_str("\"two cats\": how many animals, said the way a person says it.");
    noun = mcall(&crate::memory::store_lines::g_PLURAL_PETS(), "get", &[mcall(&call_builtin("str", &[animal.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), op_add(&call_builtin("str", &[animal.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_str("s"))]);
    return op_add(&op_add(&mcall(&crate::memory::store_lines::g_COUNT_WORDS(), "get", &[count.clone(), call_builtin("str", &[count.clone()]).unwrap_or_else(|e| exc::raise(e))]), &py_str(" ")), &noun.clone());
    Py::None
}
pub fn _asks(args: &[Py]) -> Py {
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
    let _ = py_str("True when the user's line is a question rather than something they told us.");
    return call_builtin("bool", &[mcall(&g_ASK_START(), "match", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.corrected".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AMOUNT" => Some(g_AMOUNT()),
            "AMOUNT_WORD" => Some(g_AMOUNT_WORD()),
            "ANY_NUMBER" => Some(g_ANY_NUMBER()),
            "ASK_START" => Some(g_ASK_START()),
            "AT_TIME" => Some(g_AT_TIME()),
            "AT_TIME_VALUE" => Some(g_AT_TIME_VALUE()),
            "A_DAY_BEFORE_A_MONTH" => Some(g_A_DAY_BEFORE_A_MONTH()),
            "A_DAY_SAID_LONG" => Some(g_A_DAY_SAID_LONG()),
            "A_JOURNEY_GOING" => Some(g_A_JOURNEY_GOING()),
            "A_UNIT_SAID" => Some(g_A_UNIT_SAID()),
            "BARE_HOUR" => Some(g_BARE_HOUR()),
            "BARE_NEW_COUNT" => Some(g_BARE_NEW_COUNT()),
            "CORRECTABLE" => Some(g_CORRECTABLE()),
            "CORRECTION_BACK" => Some(g_CORRECTION_BACK()),
            "COUNT_SAID" => Some(g_COUNT_SAID()),
            "COUNT_WORDS" => Some(g_COUNT_WORDS()),
            "CURRENCY" => Some(g_CURRENCY()),
            "DATE_DAY" => Some(g_DATE_DAY()),
            "DATE_PHRASE" => Some(g_DATE_PHRASE()),
            "DAY_WORD" => Some(g_DAY_WORD()),
            "FACT_FAMILY" => Some(g_FACT_FAMILY()),
            "MADE_OF_A_VALUE" => Some(g_MADE_OF_A_VALUE()),
            "MONTHS" => Some(g_MONTHS()),
            "MONTH_ALT" => Some(g_MONTH_ALT()),
            "MONTH_WORD" => Some(g_MONTH_WORD()),
            "MONTH_WORDS" => Some(g_MONTH_WORDS()),
            "NAMING_VERB" => Some(g_NAMING_VERB()),
            "NOT_A_FACT_OF_THEIRS" => Some(g_NOT_A_FACT_OF_THEIRS()),
            "NOT_A_UNIT" => Some(g_NOT_A_UNIT()),
            "NO_SUBJECT_OF_ITS_OWN" => Some(g_NO_SUBJECT_OF_ITS_OWN()),
            "NUMBER_DIGITS" => Some(g_NUMBER_DIGITS()),
            "NUMBER_VALUE" => Some(g_NUMBER_VALUE()),
            "NUMBER_WORD" => Some(g_NUMBER_WORD()),
            "OTHER_BARE_NUMBER" => Some(g_OTHER_BARE_NUMBER()),
            "OTHER_KIND_SAY" => Some(g_OTHER_KIND_SAY()),
            "OTHER_SAID" => Some(g_OTHER_SAID()),
            "PLURAL_PETS" => Some(g_PLURAL_PETS()),
            "RELATIVE_MONTH" => Some(g_RELATIVE_MONTH()),
            "SPELLING_APART" => Some(g_SPELLING_APART()),
            "STILL_THE_SAME_PERSON" => Some(g_STILL_THE_SAME_PERSON()),
            "UNIT_FAMILY" => Some(g_UNIT_FAMILY()),
            "WEEKDAY_IN_PLAN" => Some(g_WEEKDAY_IN_PLAN()),
            "_a_bare_hour" => Some(py_func("_a_bare_hour", |args: &[Py]| _a_bare_hour(args))),
            "_a_birthday_for_somebody" => Some(py_func("_a_birthday_for_somebody", |args: &[Py]| _a_birthday_for_somebody(args))),
            "_a_birthday_on_the_last_line" => Some(py_func("_a_birthday_on_the_last_line", |args: &[Py]| _a_birthday_on_the_last_line(args))),
            "_a_correction_is_not_a_plan" => Some(py_func("_a_correction_is_not_a_plan", |args: &[Py]| _a_correction_is_not_a_plan(args))),
            "_a_count" => Some(py_func("_a_count", |args: &[Py]| _a_count(args))),
            "_a_day_in_the_kind" => Some(py_func("_a_day_in_the_kind", |args: &[Py]| _a_day_in_the_kind(args))),
            "_a_day_written_the_way_the_store_writes_it" => Some(py_func("_a_day_written_the_way_the_store_writes_it", |args: &[Py]| _a_day_written_the_way_the_store_writes_it(args))),
            "_a_fact_of_somebody_else" => Some(py_func("_a_fact_of_somebody_else", |args: &[Py]| _a_fact_of_somebody_else(args))),
            "_a_name_for_a_person" => Some(py_func("_a_name_for_a_person", |args: &[Py]| _a_name_for_a_person(args))),
            "_a_name_the_store_keeps" => Some(py_func("_a_name_the_store_keeps", |args: &[Py]| _a_name_the_store_keeps(args))),
            "_a_new_name_was_said" => Some(py_func("_a_new_name_was_said", |args: &[Py]| _a_new_name_was_said(args))),
            "_a_relative_line" => Some(py_func("_a_relative_line", |args: &[Py]| _a_relative_line(args))),
            "_a_relative_month" => Some(py_func("_a_relative_month", |args: &[Py]| _a_relative_month(args))),
            "_a_spelling_of" => Some(py_func("_a_spelling_of", |args: &[Py]| _a_spelling_of(args))),
            "_a_value_already_the_plans" => Some(py_func("_a_value_already_the_plans", |args: &[Py]| _a_value_already_the_plans(args))),
            "_a_word_of_a_name" => Some(py_func("_a_word_of_a_name", |args: &[Py]| _a_word_of_a_name(args))),
            "_an_amount_on_the_measure" => Some(py_func("_an_amount_on_the_measure", |args: &[Py]| _an_amount_on_the_measure(args))),
            "_apply_correction" => Some(py_func("_apply_correction", |args: &[Py]| _apply_correction(args))),
            "_asks" => Some(py_func("_asks", |args: &[Py]| _asks(args))),
            "_birthday_words" => Some(py_func("_birthday_words", |args: &[Py]| _birthday_words(args))),
            "_carries" => Some(py_func("_carries", |args: &[Py]| _carries(args))),
            "_clock_key" => Some(py_func("_clock_key", |args: &[Py]| _clock_key(args))),
            "_correct_a_persons_name" => Some(py_func("_correct_a_persons_name", |args: &[Py]| _correct_a_persons_name(args))),
            "_correct_birthday" => Some(py_func("_correct_birthday", |args: &[Py]| _correct_birthday(args))),
            "_correct_clock" => Some(py_func("_correct_clock", |args: &[Py]| _correct_clock(args))),
            "_correct_count" => Some(py_func("_correct_count", |args: &[Py]| _correct_count(args))),
            "_correct_date" => Some(py_func("_correct_date", |args: &[Py]| _correct_date(args))),
            "_correct_fact" => Some(py_func("_correct_fact", |args: &[Py]| _correct_fact(args))),
            "_correct_number" => Some(py_func("_correct_number", |args: &[Py]| _correct_number(args))),
            "_correct_other" => Some(py_func("_correct_other", |args: &[Py]| _correct_other(args))),
            "_correct_plan" => Some(py_func("_correct_plan", |args: &[Py]| _correct_plan(args))),
            "_correct_the_count_on_the_table" => Some(py_func("_correct_the_count_on_the_table", |args: &[Py]| _correct_the_count_on_the_table(args))),
            "_correct_their_birthday" => Some(py_func("_correct_their_birthday", |args: &[Py]| _correct_their_birthday(args))),
            "_correct_when" => Some(py_func("_correct_when", |args: &[Py]| _correct_when(args))),
            "_count_said" => Some(py_func("_count_said", |args: &[Py]| _count_said(args))),
            "_date_in_plan" => Some(py_func("_date_in_plan", |args: &[Py]| _date_in_plan(args))),
            "_date_said" => Some(py_func("_date_said", |args: &[Py]| _date_said(args))),
            "_date_word" => Some(py_func("_date_word", |args: &[Py]| _date_word(args))),
            "_date_word_case" => Some(py_func("_date_word_case", |args: &[Py]| _date_word_case(args))),
            "_denies_and_names" => Some(py_func("_denies_and_names", |args: &[Py]| _denies_and_names(args))),
            "_drop_wrong" => Some(py_func("_drop_wrong", |args: &[Py]| _drop_wrong(args))),
            "_fact_naming" => Some(py_func("_fact_naming", |args: &[Py]| _fact_naming(args))),
            "_keep_unit" => Some(py_func("_keep_unit", |args: &[Py]| _keep_unit(args))),
            "_kind_of_value" => Some(py_func("_kind_of_value", |args: &[Py]| _kind_of_value(args))),
            "_letters_apart" => Some(py_func("_letters_apart", |args: &[Py]| _letters_apart(args))),
            "_make_corrects" => Some(py_func("_make_corrects", |args: &[Py]| _make_corrects(args))),
            "_many" => Some(py_func("_many", |args: &[Py]| _many(args))),
            "_measured_the_same" => Some(py_func("_measured_the_same", |args: &[Py]| _measured_the_same(args))),
            "_names_a_date" => Some(py_func("_names_a_date", |args: &[Py]| _names_a_date(args))),
            "_names_a_half_of_the_day" => Some(py_func("_names_a_half_of_the_day", |args: &[Py]| _names_a_half_of_the_day(args))),
            "_number_belongs_to_that_age" => Some(py_func("_number_belongs_to_that_age", |args: &[Py]| _number_belongs_to_that_age(args))),
            "_number_in" => Some(py_func("_number_in", |args: &[Py]| _number_in(args))),
            "_plan_saying" => Some(py_func("_plan_saying", |args: &[Py]| _plan_saying(args))),
            "_plan_the_line_named" => Some(py_func("_plan_the_line_named", |args: &[Py]| _plan_the_line_named(args))),
            "_plan_told_line" => Some(py_func("_plan_told_line", |args: &[Py]| _plan_told_line(args))),
            "_stated_families" => Some(py_func("_stated_families", |args: &[Py]| _stated_families(args))),
            "_stated_last" => Some(py_func("_stated_last", |args: &[Py]| _stated_last(args))),
            "_stated_plan" => Some(py_func("_stated_plan", |args: &[Py]| _stated_plan(args))),
            "_swap_in" => Some(py_func("_swap_in", |args: &[Py]| _swap_in(args))),
            "_the_line_is_about_that_birthday" => Some(py_func("_the_line_is_about_that_birthday", |args: &[Py]| _the_line_is_about_that_birthday(args))),
            "_the_person_the_plan_is_about" => Some(py_func("_the_person_the_plan_is_about", |args: &[Py]| _the_person_the_plan_is_about(args))),
            "_the_plan_before" => Some(py_func("_the_plan_before", |args: &[Py]| _the_plan_before(args))),
            "_the_plan_on_the_table" => Some(py_func("_the_plan_on_the_table", |args: &[Py]| _the_plan_on_the_table(args))),
            "_the_plans_a_journey_moves" => Some(py_func("_the_plans_a_journey_moves", |args: &[Py]| _the_plans_a_journey_moves(args))),
            "_the_topic_with_the_new_clock" => Some(py_func("_the_topic_with_the_new_clock", |args: &[Py]| _the_topic_with_the_new_clock(args))),
            "_the_unit" => Some(py_func("_the_unit", |args: &[Py]| _the_unit(args))),
            "_the_weekday_in" => Some(py_func("_the_weekday_in", |args: &[Py]| _the_weekday_in(args))),
            "_what_took_its_place" => Some(py_func("_what_took_its_place", |args: &[Py]| _what_took_its_place(args))),
            "_without_the_naming_verb" => Some(py_func("_without_the_naming_verb", |args: &[Py]| _without_the_naming_verb(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AMOUNT" => s_AMOUNT(v),
            "AMOUNT_WORD" => s_AMOUNT_WORD(v),
            "ANY_NUMBER" => s_ANY_NUMBER(v),
            "ASK_START" => s_ASK_START(v),
            "AT_TIME" => s_AT_TIME(v),
            "AT_TIME_VALUE" => s_AT_TIME_VALUE(v),
            "A_DAY_BEFORE_A_MONTH" => s_A_DAY_BEFORE_A_MONTH(v),
            "A_DAY_SAID_LONG" => s_A_DAY_SAID_LONG(v),
            "A_JOURNEY_GOING" => s_A_JOURNEY_GOING(v),
            "A_UNIT_SAID" => s_A_UNIT_SAID(v),
            "BARE_HOUR" => s_BARE_HOUR(v),
            "BARE_NEW_COUNT" => s_BARE_NEW_COUNT(v),
            "CORRECTABLE" => s_CORRECTABLE(v),
            "CORRECTION_BACK" => s_CORRECTION_BACK(v),
            "COUNT_SAID" => s_COUNT_SAID(v),
            "COUNT_WORDS" => s_COUNT_WORDS(v),
            "CURRENCY" => s_CURRENCY(v),
            "DATE_DAY" => s_DATE_DAY(v),
            "DATE_PHRASE" => s_DATE_PHRASE(v),
            "DAY_WORD" => s_DAY_WORD(v),
            "FACT_FAMILY" => s_FACT_FAMILY(v),
            "MADE_OF_A_VALUE" => s_MADE_OF_A_VALUE(v),
            "MONTHS" => s_MONTHS(v),
            "MONTH_ALT" => s_MONTH_ALT(v),
            "MONTH_WORD" => s_MONTH_WORD(v),
            "MONTH_WORDS" => s_MONTH_WORDS(v),
            "NAMING_VERB" => s_NAMING_VERB(v),
            "NOT_A_FACT_OF_THEIRS" => s_NOT_A_FACT_OF_THEIRS(v),
            "NOT_A_UNIT" => s_NOT_A_UNIT(v),
            "NO_SUBJECT_OF_ITS_OWN" => s_NO_SUBJECT_OF_ITS_OWN(v),
            "NUMBER_DIGITS" => s_NUMBER_DIGITS(v),
            "NUMBER_VALUE" => s_NUMBER_VALUE(v),
            "NUMBER_WORD" => s_NUMBER_WORD(v),
            "OTHER_BARE_NUMBER" => s_OTHER_BARE_NUMBER(v),
            "OTHER_KIND_SAY" => s_OTHER_KIND_SAY(v),
            "OTHER_SAID" => s_OTHER_SAID(v),
            "PLURAL_PETS" => s_PLURAL_PETS(v),
            "RELATIVE_MONTH" => s_RELATIVE_MONTH(v),
            "SPELLING_APART" => s_SPELLING_APART(v),
            "STILL_THE_SAME_PERSON" => s_STILL_THE_SAME_PERSON(v),
            "UNIT_FAMILY" => s_UNIT_FAMILY(v),
            "WEEKDAY_IN_PLAN" => s_WEEKDAY_IN_PLAN(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.corrected", module_obj);
    pyrt::xrefs::reg_fn("memory.corrected", "_number_belongs_to_that_age", py_func("_number_belongs_to_that_age", |args: &[Py]| _number_belongs_to_that_age(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_name_for_a_person", py_func("_a_name_for_a_person", |args: &[Py]| _a_name_for_a_person(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_new_name_was_said", py_func("_a_new_name_was_said", |args: &[Py]| _a_new_name_was_said(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_word_of_a_name", py_func("_a_word_of_a_name", |args: &[Py]| _a_word_of_a_name(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_denies_and_names", py_func("_denies_and_names", |args: &[Py]| _denies_and_names(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_other", py_func("_correct_other", |args: &[Py]| _correct_other(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_make_corrects", py_func("_make_corrects", |args: &[Py]| _make_corrects(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_the_unit", py_func("_the_unit", |args: &[Py]| _the_unit(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_stated_families", py_func("_stated_families", |args: &[Py]| _stated_families(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_measured_the_same", py_func("_measured_the_same", |args: &[Py]| _measured_the_same(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_without_the_naming_verb", py_func("_without_the_naming_verb", |args: &[Py]| _without_the_naming_verb(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_the_plan_on_the_table", py_func("_the_plan_on_the_table", |args: &[Py]| _the_plan_on_the_table(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_the_plan_before", py_func("_the_plan_before", |args: &[Py]| _the_plan_before(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_what_took_its_place", py_func("_what_took_its_place", |args: &[Py]| _what_took_its_place(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_day_written_the_way_the_store_writes_it", py_func("_a_day_written_the_way_the_store_writes_it", |args: &[Py]| _a_day_written_the_way_the_store_writes_it(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_the_person_the_plan_is_about", py_func("_the_person_the_plan_is_about", |args: &[Py]| _the_person_the_plan_is_about(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_correction_is_not_a_plan", py_func("_a_correction_is_not_a_plan", |args: &[Py]| _a_correction_is_not_a_plan(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_value_already_the_plans", py_func("_a_value_already_the_plans", |args: &[Py]| _a_value_already_the_plans(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_apply_correction", py_func("_apply_correction", |args: &[Py]| _apply_correction(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_stated_plan", py_func("_stated_plan", |args: &[Py]| _stated_plan(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_an_amount_on_the_measure", py_func("_an_amount_on_the_measure", |args: &[Py]| _an_amount_on_the_measure(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_plan", py_func("_correct_plan", |args: &[Py]| _correct_plan(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_number_in", py_func("_number_in", |args: &[Py]| _number_in(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_a_persons_name", py_func("_correct_a_persons_name", |args: &[Py]| _correct_a_persons_name(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_spelling_of", py_func("_a_spelling_of", |args: &[Py]| _a_spelling_of(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_letters_apart", py_func("_letters_apart", |args: &[Py]| _letters_apart(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_clock", py_func("_correct_clock", |args: &[Py]| _correct_clock(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_the_topic_with_the_new_clock", py_func("_the_topic_with_the_new_clock", |args: &[Py]| _the_topic_with_the_new_clock(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_bare_hour", py_func("_a_bare_hour", |args: &[Py]| _a_bare_hour(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_names_a_half_of_the_day", py_func("_names_a_half_of_the_day", |args: &[Py]| _names_a_half_of_the_day(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_clock_key", py_func("_clock_key", |args: &[Py]| _clock_key(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_number", py_func("_correct_number", |args: &[Py]| _correct_number(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_count_said", py_func("_count_said", |args: &[Py]| _count_said(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_count", py_func("_a_count", |args: &[Py]| _a_count(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_count", py_func("_correct_count", |args: &[Py]| _correct_count(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_the_count_on_the_table", py_func("_correct_the_count_on_the_table", |args: &[Py]| _correct_the_count_on_the_table(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_fact_of_somebody_else", py_func("_a_fact_of_somebody_else", |args: &[Py]| _a_fact_of_somebody_else(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_the_plans_a_journey_moves", py_func("_the_plans_a_journey_moves", |args: &[Py]| _the_plans_a_journey_moves(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_when", py_func("_correct_when", |args: &[Py]| _correct_when(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_date_said", py_func("_date_said", |args: &[Py]| _date_said(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_day_in_the_kind", py_func("_a_day_in_the_kind", |args: &[Py]| _a_day_in_the_kind(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_date_word_case", py_func("_date_word_case", |args: &[Py]| _date_word_case(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_date_in_plan", py_func("_date_in_plan", |args: &[Py]| _date_in_plan(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_the_weekday_in", py_func("_the_weekday_in", |args: &[Py]| _the_weekday_in(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_names_a_date", py_func("_names_a_date", |args: &[Py]| _names_a_date(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_relative_month", py_func("_a_relative_month", |args: &[Py]| _a_relative_month(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_relative_line", py_func("_a_relative_line", |args: &[Py]| _a_relative_line(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_plan_saying", py_func("_plan_saying", |args: &[Py]| _plan_saying(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_date", py_func("_correct_date", |args: &[Py]| _correct_date(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_name_the_store_keeps", py_func("_a_name_the_store_keeps", |args: &[Py]| _a_name_the_store_keeps(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_stated_last", py_func("_stated_last", |args: &[Py]| _stated_last(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_plan_told_line", py_func("_plan_told_line", |args: &[Py]| _plan_told_line(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_date_word", py_func("_date_word", |args: &[Py]| _date_word(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_kind_of_value", py_func("_kind_of_value", |args: &[Py]| _kind_of_value(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_swap_in", py_func("_swap_in", |args: &[Py]| _swap_in(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_plan_the_line_named", py_func("_plan_the_line_named", |args: &[Py]| _plan_the_line_named(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_carries", py_func("_carries", |args: &[Py]| _carries(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_fact_naming", py_func("_fact_naming", |args: &[Py]| _fact_naming(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_birthday_on_the_last_line", py_func("_a_birthday_on_the_last_line", |args: &[Py]| _a_birthday_on_the_last_line(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_birthday", py_func("_correct_birthday", |args: &[Py]| _correct_birthday(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_a_birthday_for_somebody", py_func("_a_birthday_for_somebody", |args: &[Py]| _a_birthday_for_somebody(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_the_line_is_about_that_birthday", py_func("_the_line_is_about_that_birthday", |args: &[Py]| _the_line_is_about_that_birthday(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_their_birthday", py_func("_correct_their_birthday", |args: &[Py]| _correct_their_birthday(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_correct_fact", py_func("_correct_fact", |args: &[Py]| _correct_fact(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_keep_unit", py_func("_keep_unit", |args: &[Py]| _keep_unit(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_drop_wrong", py_func("_drop_wrong", |args: &[Py]| _drop_wrong(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_birthday_words", py_func("_birthday_words", |args: &[Py]| _birthday_words(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_many", py_func("_many", |args: &[Py]| _many(args)));
    pyrt::xrefs::reg_fn("memory.corrected", "_asks", py_func("_asks", |args: &[Py]| _asks(args)));
    pyrt::xrefs::reg_access("memory.corrected", "AMOUNT", g_AMOUNT, s_AMOUNT);
    pyrt::xrefs::reg_access("memory.corrected", "AMOUNT_WORD", g_AMOUNT_WORD, s_AMOUNT_WORD);
    pyrt::xrefs::reg_access("memory.corrected", "ANY_NUMBER", g_ANY_NUMBER, s_ANY_NUMBER);
    pyrt::xrefs::reg_access("memory.corrected", "ASK_START", g_ASK_START, s_ASK_START);
    pyrt::xrefs::reg_access("memory.corrected", "AT_TIME", g_AT_TIME, s_AT_TIME);
    pyrt::xrefs::reg_access("memory.corrected", "AT_TIME_VALUE", g_AT_TIME_VALUE, s_AT_TIME_VALUE);
    pyrt::xrefs::reg_access("memory.corrected", "A_DAY_BEFORE_A_MONTH", g_A_DAY_BEFORE_A_MONTH, s_A_DAY_BEFORE_A_MONTH);
    pyrt::xrefs::reg_access("memory.corrected", "A_DAY_SAID_LONG", g_A_DAY_SAID_LONG, s_A_DAY_SAID_LONG);
    pyrt::xrefs::reg_access("memory.corrected", "A_JOURNEY_GOING", g_A_JOURNEY_GOING, s_A_JOURNEY_GOING);
    pyrt::xrefs::reg_access("memory.corrected", "A_UNIT_SAID", g_A_UNIT_SAID, s_A_UNIT_SAID);
    pyrt::xrefs::reg_access("memory.corrected", "BARE_HOUR", g_BARE_HOUR, s_BARE_HOUR);
    pyrt::xrefs::reg_access("memory.corrected", "BARE_NEW_COUNT", g_BARE_NEW_COUNT, s_BARE_NEW_COUNT);
    pyrt::xrefs::reg_access("memory.corrected", "CORRECTABLE", g_CORRECTABLE, s_CORRECTABLE);
    pyrt::xrefs::reg_access("memory.corrected", "CORRECTION_BACK", g_CORRECTION_BACK, s_CORRECTION_BACK);
    pyrt::xrefs::reg_access("memory.corrected", "COUNT_SAID", g_COUNT_SAID, s_COUNT_SAID);
    pyrt::xrefs::reg_access("memory.corrected", "COUNT_WORDS", g_COUNT_WORDS, s_COUNT_WORDS);
    pyrt::xrefs::reg_access("memory.corrected", "CURRENCY", g_CURRENCY, s_CURRENCY);
    pyrt::xrefs::reg_access("memory.corrected", "DATE_DAY", g_DATE_DAY, s_DATE_DAY);
    pyrt::xrefs::reg_access("memory.corrected", "DATE_PHRASE", g_DATE_PHRASE, s_DATE_PHRASE);
    pyrt::xrefs::reg_access("memory.corrected", "DAY_WORD", g_DAY_WORD, s_DAY_WORD);
    pyrt::xrefs::reg_access("memory.corrected", "FACT_FAMILY", g_FACT_FAMILY, s_FACT_FAMILY);
    pyrt::xrefs::reg_access("memory.corrected", "MADE_OF_A_VALUE", g_MADE_OF_A_VALUE, s_MADE_OF_A_VALUE);
    pyrt::xrefs::reg_access("memory.corrected", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("memory.corrected", "MONTH_ALT", g_MONTH_ALT, s_MONTH_ALT);
    pyrt::xrefs::reg_access("memory.corrected", "MONTH_WORD", g_MONTH_WORD, s_MONTH_WORD);
    pyrt::xrefs::reg_access("memory.corrected", "MONTH_WORDS", g_MONTH_WORDS, s_MONTH_WORDS);
    pyrt::xrefs::reg_access("memory.corrected", "NAMING_VERB", g_NAMING_VERB, s_NAMING_VERB);
    pyrt::xrefs::reg_access("memory.corrected", "NOT_A_FACT_OF_THEIRS", g_NOT_A_FACT_OF_THEIRS, s_NOT_A_FACT_OF_THEIRS);
    pyrt::xrefs::reg_access("memory.corrected", "NOT_A_UNIT", g_NOT_A_UNIT, s_NOT_A_UNIT);
    pyrt::xrefs::reg_access("memory.corrected", "NO_SUBJECT_OF_ITS_OWN", g_NO_SUBJECT_OF_ITS_OWN, s_NO_SUBJECT_OF_ITS_OWN);
    pyrt::xrefs::reg_access("memory.corrected", "NUMBER_DIGITS", g_NUMBER_DIGITS, s_NUMBER_DIGITS);
    pyrt::xrefs::reg_access("memory.corrected", "NUMBER_VALUE", g_NUMBER_VALUE, s_NUMBER_VALUE);
    pyrt::xrefs::reg_access("memory.corrected", "NUMBER_WORD", g_NUMBER_WORD, s_NUMBER_WORD);
    pyrt::xrefs::reg_access("memory.corrected", "OTHER_BARE_NUMBER", g_OTHER_BARE_NUMBER, s_OTHER_BARE_NUMBER);
    pyrt::xrefs::reg_access("memory.corrected", "OTHER_KIND_SAY", g_OTHER_KIND_SAY, s_OTHER_KIND_SAY);
    pyrt::xrefs::reg_access("memory.corrected", "OTHER_SAID", g_OTHER_SAID, s_OTHER_SAID);
    pyrt::xrefs::reg_access("memory.corrected", "PLURAL_PETS", g_PLURAL_PETS, s_PLURAL_PETS);
    pyrt::xrefs::reg_access("memory.corrected", "RELATIVE_MONTH", g_RELATIVE_MONTH, s_RELATIVE_MONTH);
    pyrt::xrefs::reg_access("memory.corrected", "SPELLING_APART", g_SPELLING_APART, s_SPELLING_APART);
    pyrt::xrefs::reg_access("memory.corrected", "STILL_THE_SAME_PERSON", g_STILL_THE_SAME_PERSON, s_STILL_THE_SAME_PERSON);
    pyrt::xrefs::reg_access("memory.corrected", "UNIT_FAMILY", g_UNIT_FAMILY, s_UNIT_FAMILY);
    pyrt::xrefs::reg_access("memory.corrected", "WEEKDAY_IN_PLAN", g_WEEKDAY_IN_PLAN, s_WEEKDAY_IN_PLAN);
}
