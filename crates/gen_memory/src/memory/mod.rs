#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
pub mod a_year_of_birth;
pub mod aboutthem;
pub mod ack;
pub mod again;
pub mod already;
pub mod amounts;
pub mod another;
pub mod asked_about;
pub mod asleep;
pub mod back_again;
pub mod baking;
pub mod callback;
pub mod carried_on;
pub mod carry;
pub mod cause;
pub mod change;
pub mod choice;
pub mod company;
pub mod condition;
pub mod corrected;
pub mod counts;
pub mod deadline;
pub mod decided;
pub mod detail;
pub mod doing_it;
pub mod doubt;
pub mod dream;
pub mod earlier;
pub mod enough;
pub mod facts;
pub mod favourite;
pub mod filler;
pub mod goals;
pub mod groupages;
pub mod guest;
pub mod healing;
pub mod history;
pub mod holds;
pub mod hope;
pub mod hours;
pub mod household;
pub mod kidcount;
pub mod kinds;
pub mod limit;
pub mod limits;
pub mod lineback;
pub mod matters;
pub mod meant;
pub mod measure;
pub mod more;
pub mod named;
pub mod narrow;
pub mod newborn;
pub mod not_a_trip;
pub mod not_this;
pub mod obstacle;
pub mod onset;
pub mod opening;
pub mod outfit;
pub mod outgoings;
pub mod paragraph;
pub mod passing_the_time;
pub mod pending;
pub mod people;
pub mod pile;
pub mod places;
pub mod plan;
pub mod plans;
pub mod portions;
pub mod prefer;
pub mod price;
pub mod purse;
pub mod quotes;
pub mod rates;
pub mod recall;
pub mod reject;
pub mod relations;
pub mod remedy;
pub mod reminded;
pub mod rename;
pub mod repair;
pub mod repeat;
pub mod resize;
pub mod returned;
pub mod returns;
pub mod saving;
pub mod setback;
pub mod size;
pub mod spent;
pub mod spoken;
pub mod stands;
pub mod store_lines;
pub mod store_replies;
pub mod stuck;
pub mod surname;
pub mod swap;
pub mod taking;
pub mod taste;
pub mod the_verdict;
pub mod the_years;
pub mod their_feeling;
pub mod thread;
pub mod times;
pub mod told;
pub mod too_late;
pub mod topics;
pub mod tried;
pub mod trips;
pub mod warmth;
pub mod whathurts;
pub mod when;
pub mod whereabout;
pub mod whose_body;
pub mod will_it_work;
pub mod workplace;
pub mod years_in;
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


thread_local! { static G_ANOTHER_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOSTROPHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOSTROPHE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPOINTMENT_A_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPOINTMENT_A_PERSON_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPOINTMENT_A_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPOINTMENT_A_QUANTITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPOINTMENT_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPOINTMENT_NOT_A_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPOINTMENT_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_DOES_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_DOES_S: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_TO_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINED_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOURNEY_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIND_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIST_FACTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_OWNED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PETS_KEPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREPOSITIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUOTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_TO_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERBISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WERE_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_OR_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOSE_IT_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_SOMEBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DAY_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BOT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BOT_MY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DATE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKDAY_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G___all__: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANOTHER_ONE() -> Py { ensure_init(); G_ANOTHER_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_ONE(v: Py) { ensure_init(); G_ANOTHER_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_APOSTROPHE() -> Py { ensure_init(); G_APOSTROPHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOSTROPHE(v: Py) { ensure_init(); G_APOSTROPHE.with(|c| *c.borrow_mut() = v); }
pub fn g_APOSTROPHE_WORD() -> Py { ensure_init(); G_APOSTROPHE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOSTROPHE_WORD(v: Py) { ensure_init(); G_APOSTROPHE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_APPOINTMENT_A_DETERMINER() -> Py { ensure_init(); G_APPOINTMENT_A_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPOINTMENT_A_DETERMINER(v: Py) { ensure_init(); G_APPOINTMENT_A_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_APPOINTMENT_A_PERSON_VERB() -> Py { ensure_init(); G_APPOINTMENT_A_PERSON_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPOINTMENT_A_PERSON_VERB(v: Py) { ensure_init(); G_APPOINTMENT_A_PERSON_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_APPOINTMENT_A_PLURAL() -> Py { ensure_init(); G_APPOINTMENT_A_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPOINTMENT_A_PLURAL(v: Py) { ensure_init(); G_APPOINTMENT_A_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g_APPOINTMENT_A_QUANTITY() -> Py { ensure_init(); G_APPOINTMENT_A_QUANTITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPOINTMENT_A_QUANTITY(v: Py) { ensure_init(); G_APPOINTMENT_A_QUANTITY.with(|c| *c.borrow_mut() = v); }
pub fn g_APPOINTMENT_DOING() -> Py { ensure_init(); G_APPOINTMENT_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPOINTMENT_DOING(v: Py) { ensure_init(); G_APPOINTMENT_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_APPOINTMENT_NOT_A_PLURAL() -> Py { ensure_init(); G_APPOINTMENT_NOT_A_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPOINTMENT_NOT_A_PLURAL(v: Py) { ensure_init(); G_APPOINTMENT_NOT_A_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g_APPOINTMENT_PLAN() -> Py { ensure_init(); G_APPOINTMENT_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPOINTMENT_PLAN(v: Py) { ensure_init(); G_APPOINTMENT_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_TAIL() -> Py { ensure_init(); G_ASK_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_TAIL(v: Py) { ensure_init(); G_ASK_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WORDS() -> Py { ensure_init(); G_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); G_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_DOES_BARE() -> Py { ensure_init(); G_A_THING_DOES_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_DOES_BARE(v: Py) { ensure_init(); G_A_THING_DOES_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_DOES_S() -> Py { ensure_init(); G_A_THING_DOES_S.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_DOES_S(v: Py) { ensure_init(); G_A_THING_DOES_S.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTRIES() -> Py { ensure_init(); G_COUNTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTRIES(v: Py) { ensure_init(); G_COUNTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_HEAD() -> Py { ensure_init(); G_COUNT_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_HEAD(v: Py) { ensure_init(); G_COUNT_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINERS() -> Py { ensure_init(); G_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); G_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_TO_A_PERSON() -> Py { ensure_init(); G_DONE_TO_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_TO_A_PERSON(v: Py) { ensure_init(); G_DONE_TO_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON_HEADS() -> Py { ensure_init(); G_FIRST_PERSON_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON_HEADS(v: Py) { ensure_init(); G_FIRST_PERSON_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINED_SUBJECT() -> Py { ensure_init(); G_JOINED_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINED_SUBJECT(v: Py) { ensure_init(); G_JOINED_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_JOURNEY_LINE() -> Py { ensure_init(); G_JOURNEY_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOURNEY_LINE(v: Py) { ensure_init(); G_JOURNEY_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_KIND_ASK() -> Py { ensure_init(); G_KIND_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIND_ASK(v: Py) { ensure_init(); G_KIND_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LIST_FACTS() -> Py { ensure_init(); G_LIST_FACTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIST_FACTS(v: Py) { ensure_init(); G_LIST_FACTS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONTHS() -> Py { ensure_init(); G_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); G_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_OWNED() -> Py { ensure_init(); G_NOT_OWNED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_OWNED(v: Py) { ensure_init(); G_NOT_OWNED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_PLACES() -> Py { ensure_init(); G_NOT_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_PLACES(v: Py) { ensure_init(); G_NOT_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_TROUBLE() -> Py { ensure_init(); G_PERSON_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_TROUBLE(v: Py) { ensure_init(); G_PERSON_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PETS_KEPT() -> Py { ensure_init(); G_PETS_KEPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PETS_KEPT(v: Py) { ensure_init(); G_PETS_KEPT.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_OPENS() -> Py { ensure_init(); G_PLAN_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_OPENS(v: Py) { ensure_init(); G_PLAN_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_VERBS() -> Py { ensure_init(); G_PLAN_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_VERBS(v: Py) { ensure_init(); G_PLAN_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_PREPOSITIONS() -> Py { ensure_init(); G_PREPOSITIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREPOSITIONS(v: Py) { ensure_init(); G_PREPOSITIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_OPEN() -> Py { ensure_init(); G_QUESTION_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_OPEN(v: Py) { ensure_init(); G_QUESTION_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_QUOTED() -> Py { ensure_init(); G_QUOTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUOTED(v: Py) { ensure_init(); G_QUOTED.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_TO_A_PERSON() -> Py { ensure_init(); G_SAYS_TO_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_TO_A_PERSON(v: Py) { ensure_init(); G_SAYS_TO_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_HEADS() -> Py { ensure_init(); G_SUBJECT_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_HEADS(v: Py) { ensure_init(); G_SUBJECT_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WORDS() -> Py { ensure_init(); G_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WORDS(v: Py) { ensure_init(); G_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VERBISH() -> Py { ensure_init(); G_VERBISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERBISH(v: Py) { ensure_init(); G_VERBISH.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB_FORMS() -> Py { ensure_init(); G_VERB_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB_FORMS(v: Py) { ensure_init(); G_VERB_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_WERE_DOING() -> Py { ensure_init(); G_WERE_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WERE_DOING(v: Py) { ensure_init(); G_WERE_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_OR_WHERE() -> Py { ensure_init(); G_WHEN_OR_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_OR_WHERE(v: Py) { ensure_init(); G_WHEN_OR_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_WORDS() -> Py { ensure_init(); G_WHEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_WORDS(v: Py) { ensure_init(); G_WHEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOSE_IT_IS() -> Py { ensure_init(); G_WHOSE_IT_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOSE_IT_IS(v: Py) { ensure_init(); G_WHOSE_IT_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_SAID() -> Py { ensure_init(); G_WHO_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_SAID(v: Py) { ensure_init(); G_WHO_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_SOMEBODY() -> Py { ensure_init(); G_WITH_SOMEBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_SOMEBODY(v: Py) { ensure_init(); G_WITH_SOMEBODY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DAY_NUMBER() -> Py { ensure_init(); G__A_DAY_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DAY_NUMBER(v: Py) { ensure_init(); G__A_DAY_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__BOT_ME() -> Py { ensure_init(); G__BOT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BOT_ME(v: Py) { ensure_init(); G__BOT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g__BOT_MY() -> Py { ensure_init(); G__BOT_MY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BOT_MY(v: Py) { ensure_init(); G__BOT_MY.with(|c| *c.borrow_mut() = v); }
pub fn g__DATE_LEADS() -> Py { ensure_init(); G__DATE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DATE_LEADS(v: Py) { ensure_init(); G__DATE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKDAY_NAMES() -> Py { ensure_init(); G__WEEKDAY_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKDAY_NAMES(v: Py) { ensure_init(); G__WEEKDAY_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g___all__() -> Py { ensure_init(); G___all__.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s___all__(v: Py) { ensure_init(); G___all__.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The memory store: what the user and the bot have said. The conversation lane owns it.\n\nShared keys of the conversation's memory dict (list new ones in README.md):\n- memory[\"turns\"]: every earlier turn as {\"user\", \"bot\", \"module\"}; the runtime writes it.\n- memory[\"understood\"]: this turn's reading of the user's line (bot/understanding).\n- memory[\"user\"]: facts about the user, e.g. {\"name\": \"Sam\", \"city\": \"Leeds\", \"allergies\":\n  [\"nuts\"], \"pets\": [{\"animal\": \"dog\", \"name\": \"Biscuit\"}], \"budget\": \"6,000 pounds\",\n  \"diet\": \"vegetarian\" or \"no dairy\", \"condition\": [\"coeliac\"], \"age\": 34,\n  \"birthday\": \"3rd of may\", \"weight\": \"90 kilos\", \"plans\": [\"...\"]}.\n  Write it through remember().\n- memory[\"asked\"]: the fact the last recall question was about, {\"slot\", \"turn\"}, so an\n  elided ask (\"what did I say it was again?\", \"and what did I call him?\") is answered about\n  the same fact.\n- memory[\"facts_log\"]: every fact the user stated, in order: {\"key\", \"value\", \"turn\", \"text\"}.\n- memory[\"others\"]: facts about other people the user mentioned, [{\"who\": \"son\", \"allergies\":\n  [\"nuts\"]}], with who \"\" for \"he's\" when no noun is named. A fact belongs to the person it is\n  about, so \"my son is allergic to nuts\" goes here and never into memory[\"user\"].\n- memory[\"topic\"]: what the conversation is about now, {\"turn\", \"text\", \"words\"}; see\n  bot/memory/topics.py.\n- memory[\"night_out\"]: the place they said they went (\"we went to the pub last night\"),\n  {\"place\", \"turn\"}, so the next line can be answered about it (\"it was lovely\"). It only\n  counts for the turn right after the one that asked.\n- memory[\"topics\"]: every topic so far, oldest first.\n- memory[\"return_to\"], memory[\"returned\"]: set when the user goes back to an earlier topic (\"ok\n  back to the boiler\", \"anyway, the laptop\"); returned is {\"text\", \"asked\", \"asks\", \"turn\"} for\n  the turn it happened, so an answer module can greet the return or answer on that topic.\n\nHelpers every lane can use:\n- user(memory) -> the user's facts (a dict), created on first use\n- others(memory) -> facts about other people they mentioned (a list, created on first use)\n- remember(memory, key, value, text=\"\") -> keep a fact and log when it was said\n- current(memory), expand(text, memory), is_follow_up(text, memory), keywords(text)\n- resumed(memory) -> the earlier topic the user went back to on this turn, or None\n- return_target(text) -> the thing a return line names (\"the boiler\"), or \"\"\n- subject(memory) -> the noun phrase the conversation is about now (\"birthday cake\"), for\n  modules that answer a follow-up; gist(text) -> that phrase for one line\n- followup_reading(understood, memory): the reading with the current topic's subject put back in,\n  for a follow-up ('and if I stuff it?') or a correction ('sorry, I meant Florence')\n- the_thing_a_detail_belongs_to(understood, memory): an ask that names only a detail of the animal\n  the store holds, read with that animal's kind beside it ('how do I stop the chewing' after a\n  puppy is the puppy's chewing, not a pen at a desk); None when it lands on no entry\n- a_detail_of_the_thing_we_hold(understood, memory): the same reading without the entry, True when\n  the line names a detail of the animal and leaves the animal itself out\n- key_subject(text, memory), same_as_before(understood, memory), already_said(memory, text)\n- forms(word), content(text): a word with its verb endings on or off, and the words a line is\n  about, so \"saving\" finds \"save\" and a grammar word with an ending is still grammar\n- when_answer(text, memory): the date of the thing a \"when ...\" line asks about, from the line\n  that stated it (bot/memory/when.py); None when no earlier line dates it\n- remember_answer(memory, reply, keys), entry_answer(memory, topic_text): the whole answer an\n  entry gave, kept so a later follow-up can still reach a sentence a trimmed answer left out\n- recent(memory, n), last_user(memory), last_bot(memory),\n  answer_in_hand(memory): the bot turn that answered the subject the chat is on, read back over\n  an acknowledgement or two (\"ok thanks\" does not end what was being talked about, so the\n  follow-up after it is still about the answer before the thanks)\n- choice.reply_for(text, memory): when they hand the choice back (\"you pick\", \"your call\"),\n  one of the options the bot offered a turn ago, read out of its own last reply\n  (bot/memory/choice.py)\n- reject.reply_for(text, memory): the other side of that - they turn the offer down (\"nah not\n  that one\", \"ive seen it\"), and get another of the options the bot itself named\n  (bot/memory/reject.py)\n");
    s_VERB_FORMS(gen_understanding::understanding::kinds::g_VERB_FORMS());
    s_VERBISH(gen_understanding::understanding::kinds::g_VERBISH());
    s_TIME_WORDS(gen_understanding::understanding::normalize::g_TIME_WORDS());
    s_COUNTRIES(crate::memory::facts::g_COUNTRIES());
    s_DETERMINERS(crate::memory::facts::g_DETERMINERS());
    s_NOT_PLACES(crate::memory::facts::g_NOT_PLACES());
    s_MONTHS(crate::memory::facts::g_MONTHS());
    s_PREPOSITIONS(crate::memory::facts::g_PREPOSITIONS());
    s_PERSON_TROUBLE(crate::memory::topics::g_PERSON_TROUBLE());
    s_KIND_ASK(crate::memory::topics::g_KIND_ASK());
    s_WHO_SAID(crate::memory::topics::g_WHO_SAID());
    s_LIST_FACTS(py_tuple(vec![py_str("places"), py_str("allergies"), py_str("people"), py_str("plans"), py_str("condition")]));
    s_PETS_KEPT(Py::Int(12));
    s_ANOTHER_ONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("other another second third fourth youngest eldest oldest first middle\nlast"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COUNT_HEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a few|several|lots of|\\d+)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUESTION_OPEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:what|how|why|when|where|who|which|whose|should|shall|do|does|did|can|could|is|are|was|were|will|would|any)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHEN_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("january february march april may june july august september october\nnovember december jan feb mar apr jun jul aug sep sept oct nov dec\nmonday tuesday wednesday thursday friday saturday sunday weekend\nnext this coming tomorrow tonight morning afternoon evening week month year the of on in at\nfor and on"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLAN_OPENS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i i'm im i've i'll ill we we're were we've weve we'll well my our me"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_JOINED_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:[a-z'\\u2019]+\\s+){0,3}and\\s+(?:i|me)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLAN_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("planning going trying aiming hoping intending looking"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APPOINTMENT_PLAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:the\\s+)?(?P<thing>[a-z][a-z0-9' \\-]{1,28}?)\\s+(?P<when>at\\s+[\\w:.]+(?:\\s*(?:am|pm))?(?:\\s+on\\s+[a-z0-9 \\-]+)?|on\\s+[a-z0-9 \\-]+)$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APPOINTMENT_DOING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\narrange bake book bring buy call clean collect cook deliver do drop email fetch finish fix get\ngo have hire make meet message order pay phone pick post put renew rent ring see send set sort\nstart submit take text tidy visit write\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APPOINTMENT_A_DETERMINER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\na an the my your our their his her its this that these those some any\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APPOINTMENT_A_PERSON_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nmeet call ring text phone message email see visit collect fetch\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APPOINTMENT_A_QUANTITY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nbread money rent cash milk water petrol diesel fuel shopping washing ironing laundry food\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APPOINTMENT_A_PLURAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^[a-z]{3,}(?:s|es)$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_APPOINTMENT_NOT_A_PLURAL(py_tuple(vec![py_str("ss"), py_str("us"), py_str("is"), py_str("ous")]));
    s__WEEKDAY_NAMES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("christmas"), py_str("easter")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__DATE_LEADS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("in"), py_str("of"), py_str("by"), py_str("from"), py_str("until"), py_str("till"), py_str("til"), py_str("early"), py_str("late"), py_str("mid"), py_str("towards"), py_str("before"), py_str("after")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_DAY_NUMBER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\d{1,2}(?:st|nd|rd|th)?$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHOSE_IT_IS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("his"), py_str("her"), py_str("hers"), py_str("their"), py_str("theirs"), py_str("your"), py_str("yours")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_DOES_BARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("go come start end begin finish close open reopen run happen\narrive leave return fly land depart move switch change rise set turn shut take last break\nbreak up go back come round"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_DOES_S({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("is are was were due goes comes starts ends begins finishes closes\nopens reopens runs happens arrives leaves returns flies lands departs moves switches changes\nrises sets turns shuts takes lasts breaks"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_JOURNEY_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|im|i'm|ive|i've|we|were|we're|weve|we've|they|hes|he's|shes|she's)\\s+(?:am\\s+|are\\s+|is\\s+)?(?:going|off|flying|fly|moving|driving|heading|travell?ing|cycling|walking|jetting|sailing|back)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAYS_TO_A_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\ntold tell telling tellin says said asks asked ask asking calls called call calling rang ring\nringing texted texting texts text emailed emailing emails email phoned phoning phones phone\nmessaged messaging message thanked thanking thanks thank blamed blaming blame ignored ignoring\nignore hurt hurting hurts missed missing miss loved loving love needed needing need trusted\ntrusting trust woke woken woke saw seeing seen heard hearing hear helped helping help sent\nsending send showed showing show shown gave giving give given paid paying pay owed owing owe met\nmeeting meet noticed noticing notice picked picking pick chose chosen choose chose kissed\nkissing kiss hugged hugging hug\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DONE_TO_A_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nto at with for about on from of around near towards toward into over behind beside against\nacross\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUBJECT_HEADS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("i"), py_str("i'm"), py_str("i've"), py_str("i'll"), py_str("i'd"), py_str("im"), py_str("ive"), py_str("ill"), py_str("we"), py_str("we're"), py_str("we've"), py_str("we'll"), py_str("we'd"), py_str("were"), py_str("me"), py_str("my"), py_str("mine"), py_str("us"), py_str("our"), py_str("ours"), py_str("you"), py_str("your"), py_str("yours"), py_str("yourself"), py_str("he"), py_str("she"), py_str("it"), py_str("they"), py_str("him"), py_str("her"), py_str("them"), py_str("his"), py_str("their"), py_str("theirs")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FIRST_PERSON_HEADS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("i"), py_str("i'm"), py_str("i've"), py_str("i'll"), py_str("i'd"), py_str("im"), py_str("ive"), py_str("ill"), py_str("we"), py_str("we're"), py_str("we've"), py_str("we'll"), py_str("we'd"), py_str("were")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let __unpk1 = py_unpack(&py_tuple(vec![py_str("\0m"), py_str("\0M")]), 2, None);
    s__BOT_ME(__unpk1[0].clone());
    s__BOT_MY(__unpk1[1].clone());
    s_APOSTROPHE(py_dict(vec![(py_str("im"), py_str("I'm")), (py_str("ive"), py_str("I've")), (py_str("dont"), py_str("don't")), (py_str("cant"), py_str("can't")), (py_str("doesnt"), py_str("doesn't")), (py_str("didnt"), py_str("didn't")), (py_str("isnt"), py_str("isn't")), (py_str("wasnt"), py_str("wasn't")), (py_str("arent"), py_str("aren't")), (py_str("werent"), py_str("weren't")), (py_str("havent"), py_str("haven't")), (py_str("hasnt"), py_str("hasn't")), (py_str("hadnt"), py_str("hadn't")), (py_str("wouldnt"), py_str("wouldn't")), (py_str("couldnt"), py_str("couldn't")), (py_str("shouldnt"), py_str("shouldn't")), (py_str("mustnt"), py_str("mustn't")), (py_str("neednt"), py_str("needn't")), (py_str("aint"), py_str("ain't")), (py_str("shes"), py_str("she's")), (py_str("hes"), py_str("he's")), (py_str("thats"), py_str("that's")), (py_str("whats"), py_str("what's")), (py_str("theres"), py_str("there's")), (py_str("heres"), py_str("here's")), (py_str("theyre"), py_str("they're")), (py_str("youre"), py_str("you're")), (py_str("youve"), py_str("you've")), (py_str("youll"), py_str("you'll")), (py_str("youd"), py_str("you'd"))]));
    s_APOSTROPHE_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b("), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_APOSTROPHE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_QUOTED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\"[^\"]*\"|\\u201c[^\\u201d]*\\u201d|(?<!\\w)"), &py_str("'")), &py_str("[^']*")), &py_str("'")), &py_str("(?!\\w)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_OWNED(op_bitor(&op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("are were is was am and or but nor too either"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &gen_understanding::understanding::kinds::g_VERB_FORMS()), &gen_understanding::understanding::kinds::g_VERBISH()));
    s_WITH_SOMEBODY(op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("see saw seen seeing visit visited visiting meet met meeting missed\n    missing miss call called calling ring rang ringing text texted texting phone phoned phoning\n    email emailed emailing thank thanked thanking hug hugged hugging kiss kissed kissing help helped\n    helping bother bothered bothering visit stay stayed staying"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_set(vec![py_str("with")])));
    s_WHEN_OR_WHERE(op_bitor(&op_bitor(&op_bitor(&gen_understanding::understanding::normalize::g_TIME_WORDS(), &crate::memory::facts::g_PREPOSITIONS()), &crate::memory::facts::g_DETERMINERS()), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("next last later soon\n    earlier afterwards afterward today tomorrow tonight week weekend morning evening night day\n    month year couple few"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_ASK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("how what when where who why which whose should shall can could do does did\nis are was were will would am"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(",\\s*(?:and\\s+)?(?:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_ASK_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WERE_DOING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("we[r']?e?\\s+\\w+ing\\b|were\\s+\\w+ing\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s___all__(py_list(vec![py_str("action"), py_str("already_said"), py_str("asks_only"), py_str("context"), py_str("you"), py_str("current"), py_str("entry_answer"), py_str("expand"), py_str("facts"), py_str("followup_reading"), py_str("gist"), py_str("has_time"), py_str("introduction_question"), py_str("is_follow_up"), py_str("answer_in_hand"), py_str("key_subject"), py_str("keywords"), py_str("last_bot"), py_str("last_module"), py_str("last_user"), py_str("left_for_a_blank"), py_str("log"), py_str("meal_follow_up"), py_str("name"), py_str("note"), py_str("plan_line"), py_str("plan_sentence"), py_str("recent"), py_str("remember"), py_str("remember_answer"), py_str("resumed"), py_str("return_target"), py_str("said"), py_str("said_again"), py_str("same_as_before"), py_str("answer_about"), py_str("answers_about_another_creature"), py_str("the_animal_the_subject_is"), py_str("sized_reply"), py_str("stem"), py_str("subject"), py_str("the_ask_about_the_thing_it_points_at"), py_str("topic_text"), py_str("the_step_back_takes_its_words_back"), py_str("topics"), py_str("user"), py_str("bare_question"), py_str("written_out")]));
}
pub fn user(args: &[Py]) -> Py {
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
    let mut facts_: Py = Py::None;
    let _ = py_str("The user's facts (a dict), created on first use.");
    facts_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(facts_.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    facts_ = py_dict(vec![]);
    let _ = py_setitem(&memory.clone(), &py_str("user"), py_dict(vec![]));
    }
    return facts_.clone();
    Py::None
}
pub fn log(args: &[Py]) -> Py {
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
    let _ = py_str("Every fact the user stated, in the order they said it.");
    entries = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("facts_log")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entries.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    entries = py_list(vec![]);
    let _ = py_setitem(&memory.clone(), &py_str("facts_log"), py_list(vec![]));
    }
    return entries.clone();
    Py::None
}
pub fn others(args: &[Py]) -> Py {
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
    let mut seen: Py = Py::None;
    let _ = py_str("Facts about other people the user mentioned: [{\"who\": \"son\", \"allergies\": [\"nuts\"]}].\n\n    A fact belongs to the person it is about: \"my son is allergic to nuts\" is the son's, and\n    only what the user says about themselves can be recalled as their own (bot/memory/facts.py\n    decides with facts.subject()).\n    ");
    seen = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("others")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(seen.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    seen = py_list(vec![]);
    let _ = py_setitem(&memory.clone(), &py_str("others"), py_list(vec![]));
    }
    return seen.clone();
    Py::None
}
pub fn _said_another(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut earlier: Py = Py::None;
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line put one of those words in front of this person's word.\n\n    The window is two words, so \"my other little girl\" and \"the second of my sons\" are read and a\n    word from earlier in the sentence is not: \"he turns 8 the second week of june\" is one boy.\n    ");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&words.clone())) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    one = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&one.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "rstrip", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "rstrip", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&index.clone(), &Py::Int(2))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &index.clone(), &Py::None)) { let mut earlier: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ANOTHER_ONE(), &earlier.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _another_of_them(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entry: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut known: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut age: Py = Py::None;
    let mut held: Py = Py::None;
    let _ = py_str("Whether this fact is about a second person of the same kind, not the one already held.\n\n    Two people with the same word for them are told apart by a name when the chat gives one, and\n    by nothing else there is: the age is all a person gives (\"my daughter is 4\" / \"my other\n    daughter is 2\"), so a different age is what a second one looks like. Two ages for one word\n    with no telling-apart word is the ordinary correction - \"my son is 7\" / \"my son is 8\" is one\n    son of 8 - which is why the word in front of the person is what the reading turns on.\n    ");
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }]), 2, None);
    age = __unpk1[0].clone();
    held = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(age.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&age.clone(), &held.clone())) } } } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_another(args))(&__args) };
    Py::None
}
pub fn remember_other(args: &[Py]) -> Py {
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
    let mut entry: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut same: Py = Py::None;
    let mut value: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Keep a fact about someone else the user mentioned, merged into that person.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entry.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "values", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    who = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { named.clone() } }) {
    same = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| others(args))(&__args) }) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }, &named.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(same.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(geti(&same.clone(), &Py::Int(0))); __args.push(entry.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fold_into(args))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| others(args))(&__args) });
    for __item in __iter1 {
    known = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }, &who.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&entry.clone(), &py_str("name"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&known.clone(), &py_str("name"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) })) } } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entry.clone()); __args.push(known.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _another_of_them(args))(&__args) }) {
    continue;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(known.clone()); __args.push(entry.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fold_into(args))(&__args) };
    }
    kept = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "items", args))(&__args) }) { let __unpk6805 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk6805[0].clone(); let mut value: Py = __unpk6805[1].clone(); __out.push((key.clone(), if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { value.clone() })); } py_dict(__out) })() };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| others(args))(&__args) }, "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), kept.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    Py::None
}
pub fn _fold_into(args: &[Py]) -> Py {
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
    let mut known: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut entry: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut ahead: Py = Py::None;
    let mut have: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut knew: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Put one fact the line stated about a person onto the person the store already holds.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "items", args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("who"), py_str("pronoun")]), &key.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("ages")))) {
    let _ = py_setitem(&known.clone(), &py_str("ages"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to_come")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ages")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ages")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    let _ = py_setitem(&known.clone(), &py_str("age"), value.clone());
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("months")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to_come")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    ahead = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    knew = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = ahead.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&knew.clone())) } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![value.clone(), ahead.clone()]), 2, None);
    let _ = py_setitem(&known.clone(), &py_str("to_come"), __unpk3[0].clone());
    let _ = py_setitem(&known.clone(), &py_str("turns"), __unpk3[1].clone());
    continue;
    }
    if py_truthy(&{ let __b = knew.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&ahead.clone())) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to_come")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }, &value.clone())) } }) {
    let _ = py_setitem(&known.clone(), &py_str("to_come"), geti(&known.clone(), &py_str("age")));
    }
    let _ = py_setitem(&known.clone(), &py_str("age"), value.clone());
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to_come")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_le(&geti(&known.clone(), &py_str("to_come")), &value.clone()) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to_come")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    }
    continue;
    }
    if py_truthy(&{ let __b = ahead.clone(); if !py_truthy(&__b) { __b } else { { let __b = knew.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&value.clone(), &geti(&known.clone(), &py_str("age")))) } } } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![value.clone(), ahead.clone()]), 2, None);
    let _ = py_setitem(&known.clone(), &py_str("to_come"), __unpk4[0].clone());
    let _ = py_setitem(&known.clone(), &py_str("turns"), __unpk4[1].clone());
    continue;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    let _ = py_setitem(&known.clone(), &key.clone(), value.clone());
    continue;
    }
    have = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); __args.push(py_list(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "setdefault", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(have.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&have.clone(), &item.clone()))) {  __out.push(item.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&have.clone(), "extend", args))(&__args) };
    }
    }
    return known.clone();
    Py::None
}
pub fn reads_as_a_plan(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut said: Py = Py::None;
    let _ = py_str("Whether a plan kept from a line reads well back in the summary of the chat.\n\n    \"We are going to spain in july\" is a plan. \"i've got two kids\" comes out of the same slot as\n    \"Two kids\", which is a count of what they have and not something they are going to do, and\n    \"what do i need to know\" leaves the verb of the ask (\"Know\") behind, which is not a plan\n    either - and neither is the rest of that ask, so a line that asks a question keeps nothing.\n    ");
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&value.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_COUNT_HEAD(), "match", args))(&__args) }) {
    return Py::Bool(false);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "endswith", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUESTION_OPEN(), "match", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_thing_the_plan_is(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The plan without the words that say when it happens: \"my brother is coming\" for both\n    \"my brother is coming in june\" and \"my brother is coming in july\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_WHEN_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) } } else { py_str("") };
    Py::None
}
pub fn remember(args: &[Py]) -> Py {
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
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut value: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut already_named: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut known: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut one_animal: Py = Py::None;
    let mut p: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut rename: Py = Py::None;
    let mut renamed: Py = Py::None;
    let mut same: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut t: Py = Py::None;
    let mut taste: Py = Py::None;
    let mut tastes: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("Keep a fact about the user, logging when they said it.");
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str(""), py_list(vec![]), py_dict(vec![])]), &value.clone()))) {
    return Py::None;
    }
    known = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| user(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    value = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut one: Py = __item.clone(); __out.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::filler::without_filler(args))(&__args) } } else { one.clone() }); } py_list(__out) })() };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("plan"), py_str("plans")]), &key.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::filler::without_filler(args))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plans")))) {
    let __iter1 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { value.clone() } else { py_list(vec![value.clone()]) });
    for __item in __iter1 {
    one = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_plan(args))(&__args) }))) {
    continue;
    }
    plans = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plans")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plans.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    plans = py_list(vec![]);
    }
    if py_truthy(&Py::Bool(py_contains(&plans.clone(), &one.clone()))) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_the_plan_is(args))(&__args) };
    if py_truthy(&thing.clone()) {
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut kept: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_the_plan_is(args))(&__args) }, &thing.clone()))) {  __out.push(kept.clone()); } } py_list(__out) })() };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&plans.clone(), "append", args))(&__args) };
    let _ = py_setitem(&known.clone(), &py_str("plans"), slice_or_raise(&plans.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), value.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("pets")))) {
    pets = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pets")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pets.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    pets = py_list(vec![]);
    }
    value = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("not_theirs")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) } } }))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    return Py::None;
    }
    one_animal = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)));
    let __iter2 = py_iter_or_raise(&value.clone());
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    already_named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    rename = { let __b = one_animal.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("another")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&already_named.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_le(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_gt(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)))) } } } } } }) {  __out.push(p.clone()); } } py_list(__out) })() } } } } } } } } } } };
    if py_truthy(&{ let __b = rename.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rename.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))) } }) {
    renamed = geti(&rename.clone(), &Py::Int(0));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    let _ = py_setitem(&renamed.clone(), &py_str("name"), geti(&pet.clone(), &py_str("name")));
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&renamed.clone(), "get", args))(&__args) })) } }) {
    let _ = py_setitem(&renamed.clone(), &py_str("age"), geti(&pet.clone(), &py_str("age")));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&renamed.clone(), "pop", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }) {
    let _ = py_setitem(&renamed.clone(), &py_str("unit"), geti(&pet.clone(), &py_str("unit")));
    }
    }
    continue;
    }
    counted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) })) } } } }) {  __out.push(p.clone()); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    same = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("another")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })) } } } }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&counted.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&p.clone(), &counted.clone())) } } } } } } } }) {  __out.push(p.clone()); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&same.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("another")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })); if !py_truthy(&__b) { __b } else { op_le(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)) } } } }) {
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::facts::g_ANIMAL_KINDS(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breed")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) })) } }) {
    same = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coming")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = kind.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&p.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::facts::g_ANIMAL_KINDS(), "get", args))(&__args) }, &kind.clone())) } } } }) {  __out.push(p.clone()); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    if py_truthy(&Py::Bool(!py_is(&same.clone(), &Py::None))) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&same.clone(), "get", args))(&__args) })) } }) {
    let _ = py_setitem(&same.clone(), &py_str("name"), geti(&pet.clone(), &py_str("name")));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breed")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breed")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&same.clone(), "get", args))(&__args) })) } }) {
    let _ = py_setitem(&same.clone(), &py_str("breed"), geti(&pet.clone(), &py_str("breed")));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    } else {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&same.clone(), "get", args))(&__args) })) } }) {
    let _ = py_setitem(&same.clone(), &py_str("count"), geti(&pet.clone(), &py_str("count")));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&same.clone(), "get", args))(&__args) })) } }) {
    let _ = py_setitem(&same.clone(), &py_str("age"), geti(&pet.clone(), &py_str("age")));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&same.clone(), "pop", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }) {
    let _ = py_setitem(&same.clone(), &py_str("unit"), geti(&pet.clone(), &py_str("unit")));
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breed")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breed")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("breed")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&same.clone(), "get", args))(&__args) })) } }) {
    let _ = py_setitem(&same.clone(), &py_str("breed"), geti(&pet.clone(), &py_str("breed")));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pets.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    let _ = py_setitem(&known.clone(), &py_str("pets"), slice_or_raise(&pets.clone(), &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PETS_KEPT()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None, &Py::None));
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("tastes")))) {
    tastes = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("tastes")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tastes.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    tastes = py_list(vec![]);
    }
    let __iter3 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { value.clone() } else { py_list(vec![value.clone()]) });
    for __item in __iter3 {
    taste = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(taste.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&taste.clone(), "get", args))(&__args) })) } }) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&taste.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    same = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tastes.clone()) { let mut t: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&t.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &thing.clone()))) {  __out.push(t.clone()); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&same.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&same.clone(), "get", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&taste.clone(), "get", args))(&__args) }))) {
    let _ = py_setitem(&same.clone(), &py_str("how"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&taste.clone(), "get", args))(&__args) });
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("taste")), (py_str("value"), taste.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(taste.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tastes.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("taste")), (py_str("value"), taste.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    let _ = py_setitem(&known.clone(), &py_str("tastes"), slice_or_raise(&tastes.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("owns")))) {
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("owns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    held = py_dict(vec![]);
    }
    let __iter4 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "items", args))(&__args) } } else { py_list(vec![]) });
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    thing = __unpk5[0].clone();
    name = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&name.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }, &name.clone()))) {
    continue;
    }
    let _ = py_setitem(&held.clone(), &thing.clone(), name.clone());
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), py_str("owns")), (py_str("value"), py_dict(vec![(thing.clone(), name.clone())])), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    let _ = py_setitem(&known.clone(), &py_str("owns"), held.clone());
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_LIST_FACTS(), &key.clone()))) {
    seen = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(seen.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    seen = py_list(vec![]);
    }
    let __iter6 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { value.clone() } else { py_list(vec![value.clone()]) });
    for __item in __iter6 {
    item = __item.clone();
    if py_truthy(&{ let __b = item.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&seen.clone(), &item.clone())) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), item.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    }
    }
    let _ = py_setitem(&known.clone(), &key.clone(), slice_or_raise(&seen.clone(), &op_neg(&Py::Int(8)), &Py::None, &Py::None));
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }, &value.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&known.clone(), &key.clone(), value.clone());
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), value.clone()), (py_str("turn"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("text"), text.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }, "append", args))(&__args) };
    Py::None
}
pub fn forget(args: &[Py]) -> Py {
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
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut values: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "values") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: values"))) };
    let mut why: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "why") { v.clone() }
    else { Py::None };
    let mut doomed: Py = Py::None;
    let mut everything: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut had: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut known: Py = Py::None;
    let mut left: Py = Py::None;
    let mut named: Py = Py::None;
    let mut past: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut records: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut worded: Py = Py::None;
    let _ = py_str("Take values back out of a fact: \"im not coeliac\" after \"im coeliac\".\n\n    Returns the values that actually went, so the caller can say it out loud instead of leaving\n    the person to wonder whether the store heard them. The log keeps what was said earlier: a\n    fact that was true ten turns ago is still part of the conversation.\n    ");
    known = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| user(args))(&__args) };
    wanted = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(values.clone()); __args.push(py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { values.clone() } else { py_list(vec![values.clone()]) };
    doomed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut value: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }); } } py_set(__out) })() };
    had = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("pets")))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&doomed.clone())) } }) {
    return py_list(vec![]);
    }
    everything = Py::Bool(py_contains(&doomed.clone(), &py_str("*")));
    records = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut value: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(value.clone()); } } py_list(__out) })() };
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    kept = __unpk1[0].clone();
    gone = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&had.clone());
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    worded = { let __b = Py::Bool(py_contains(&doomed.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("animal")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&doomed.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) })) } };
    named = Py::Bool(py_contains(&records.clone(), &pet.clone()));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = everything.clone(); if py_truthy(&__b) { __b } else { { let __b = worded.clone(); if py_truthy(&__b) { __b } else { named.clone() } } } }))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    continue;
    }
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&everything.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&worded.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(1) } }, &Py::Int(1)) } } } } } }) {
    let _ = py_setitem(&pet.clone(), &py_str("count"), op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&pet.clone(), &py_str("count"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)));
    if py_truthy(&op_le(&geti(&pet.clone(), &py_str("count")), &Py::Int(1))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("count")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pet.clone(), "pop", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&gone.clone(), "append", args))(&__args) };
    }
    let _ = py_setitem(&known.clone(), &key.clone(), kept.clone());
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "pop", args))(&__args) };
    }
    if py_truthy(&gone.clone()) {
    past = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pets_gone")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&known.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    let __iter3 = py_iter_or_raise(&gone.clone());
    for __item in __iter3 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&past.clone(), &pet.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); __kws.extend(vec![("gone".to_string(), { let __b = why.clone(); if py_truthy(&__b) { __b } else { py_str("gone") } })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&past.clone(), "append", args))(&__args) };
    }
    }
    let _ = py_setitem(&known.clone(), &py_str("pets_gone"), past.clone());
    }
    return gone.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_LIST_FACTS(), &key.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&doomed.clone())) } }) {
    return py_list(vec![]);
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&had.clone()) { let mut value: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&doomed.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }))) {  __out.push(value.clone()); } } py_list(__out) })() };
    let _ = py_setitem(&known.clone(), &key.clone(), left.clone());
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&had.clone()) { let mut value: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&doomed.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }))) {  __out.push(value.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&had.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&doomed.clone())) } }) {
    return py_list(vec![]);
    }
    if py_truthy(&Py::Bool(py_contains(&doomed.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(had.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }))) {
    let _ = py_delitem(&known.clone(), &key.clone()).unwrap_or(());
    return py_list(vec![had.clone()]);
    }
    return py_list(vec![]);
    Py::None
}
pub fn there_is_a_chat(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the person said anything before this line.\n\n    A reply that leans on what was said earlier needs something said earlier, and on a first\n    message there is nothing. Every reader that leans on the chat asks this one question, so a\n    new one does not have to remember which key holds the turns or how the memory is shaped.\n    ");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    let __iter1 = py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn recent(args: &[Py]) -> Py {
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
    let mut n: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { Py::Int(6) };
    let mut turns: Py = Py::None;
    let _ = py_str("The last n turns, oldest first.\n\n    A reader asking about a past line with no memory to hand has no turns to look at. That\n    happens in `_about_this_chat` (`_recall_reply` and the summary readers call it with the line\n    alone, only to tell whether the line is itself a question about the chat), and the which-ask\n    rule inside it read the thing back out of the memory: `recent(None, 8)` raised\n    `AttributeError: 'NoneType' object has no attribute 'get'`, so from the first which-ask\n    onwards every recall proposed nothing and the person got the search hedge about a line they\n    had said three turns earlier. No turns is the honest answer here, not a crash.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&memory.clone()))) {
    return py_list(vec![]);
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_list(vec![]);
    }
    return slice_or_raise(&turns.clone(), &op_neg(&n.clone()), &Py::None, &Py::None);
    Py::None
}
pub fn last_user(args: &[Py]) -> Py {
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
    let mut n: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { Py::Int(1) };
    let mut lines: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The last n things the user said (before this turn), oldest first.");
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(n.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| recent(args))(&__args) }) { let mut turn: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }); } py_list(__out) })() };
    return if py_truthy(&op_gt(&n.clone(), &Py::Int(1))) { lines.clone() } else { if py_truthy(&lines.clone()) { geti(&lines.clone(), &op_neg(&Py::Int(1))) } else { py_str("") } };
    Py::None
}
pub fn last_bot(args: &[Py]) -> Py {
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
    let _ = py_str("The bot's last reply.");
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| recent(args))(&__args) };
    return if py_truthy(&turns.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) } } else { py_str("") };
    Py::None
}
pub fn last_module(args: &[Py]) -> Py {
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
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| recent(args))(&__args) };
    return if py_truthy(&turns.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) } } else { Py::None };
    Py::None
}
pub fn name(args: &[Py]) -> Py {
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
    let mut facts_: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The user's name, or ''.");
    facts_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| user(args))(&__args) };
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("name")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&facts_.clone(), "get", args))(&__args) };
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { value.clone() } else { py_str("") };
    Py::None
}
pub fn _takes_a_thing(args: &[Py]) -> Py {
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
    let _ = py_str("True when the word after the verb marks what follows as a thing, not a compound's half.");
    if py_truthy(&Py::Bool(py_contains(&g_APPOINTMENT_A_QUANTITY(), &word.clone()))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_APPOINTMENT_A_PLURAL(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_APPOINTMENT_NOT_A_PLURAL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) })) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_doing(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the plan's thing is something they are doing, not something they have on.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) }, "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_APPOINTMENT_A_PERSON_VERB(), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_APPOINTMENT_A_DETERMINER(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_APPOINTMENT_DOING(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _takes_a_thing(args))(&__args) } } } } };
    Py::None
}
pub fn named_days(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("\"on tuesday in june\" -> \"on Tuesday in June\"; \"we may go to york\" is left alone.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) };
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }, "strip", args))(&__args) };
    before = if py_truthy(&index.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))), "lower", args))(&__args) }, "strip", args))(&__args) } } else { py_str("") };
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))), "lower", args))(&__args) }, "strip", args))(&__args) } } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__WEEKDAY_NAMES(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::memory::facts::g_MONTHS(), &bare.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__DATE_LEADS(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DAY_NUMBER(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_DAY_NUMBER(), "match", args))(&__args) } } } } } } } } }) {
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&bare.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&bare.clone(), &Py::Int(1), &Py::None, &Py::None))); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "replace", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(out.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _a_name_at_the_front(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("Whether a kept plan opens with a capital that is not the article of a thing.\n\n    \"Portugal on the 12th of june\" and \"Moving to leeds on 3 june\" open on a capital the store\n    did not write for a thing: the first is a place it recognised, the second is the person's own\n    sentence. \"A cake on friday\" opens on a capital too, and that one is a thing they have.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&geti(&words.clone(), &Py::Int(0)), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "lower", args))(&__args) }, "strip", args))(&__args) }));
    Py::None
}
pub fn _a_plural(args: &[Py]) -> Py {
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
    let _ = py_str("True for the subject of a sentence said in the plural: \"clocks\", \"school holidays\".");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ss"), py_str("us"), py_str("is")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) })) } };
    Py::None
}
pub fn _has_its_own_verb(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing kept for a day already says what it does: \"clocks go back\".");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_A_THING_DOES_S(), &word.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_plural(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_THING_DOES_BARE(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn appointment_sentence(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("\"dentist at 3 on tuesday\" -> \"Your dentist is at 3 on tuesday\", or \"\" when it is not one.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .!?,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_APPOINTMENT_PLAN(), "match", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_possessive_a_relation_means(args))(&__args) };
    when = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("when")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("we"), py_str("were"), py_str("we're"), py_str("my"), py_str("our"), py_str("to"), py_str("it"), py_str("its"), py_str("he"), py_str("she"), py_str("they"), py_str("you")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) })) } }) {
    return py_str("");
    }
    when = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(when.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| named_days(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_doing(args))(&__args) }) {
    said = op_add(&op_add(&op_add(&op_add(&py_str("You're planning to "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", args))(&__args) }), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" ")), &when.clone());
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) }))) {
    said = op_add(&op_add(&op_add(&op_add(&py_str("You've got "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", args))(&__args) }), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" ")), &when.clone());
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(py_contains(&g_WHOSE_IT_IS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) }))) {
    said = op_add(&op_add(&op_add(&thing.clone(), &if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has_its_own_verb(args))(&__args) }) { py_str("") } else { py_str(" is") }), &py_str(" ")), &when.clone());
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    said = op_add(&op_add(&op_add(&op_add(&py_str("Your "), &thing.clone()), &if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has_its_own_verb(args))(&__args) }) { py_str("") } else { py_str(" is") }), &py_str(" ")), &when.clone());
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _a_journey_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said_line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_line"))) };
    let _ = py_str("True when the line the plan came from puts the person on their way somewhere.\n\n    The journey word has to be what they are doing (\"im going to spain on 3 june\", \"we fly to rome\n    on 3 june\"), not the name of the thing (\"my driving test is on 14 march\"), so it has to come\n    straight after the person.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said_line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_JOURNEY_LINE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn plan_sentence(args: &[Py]) -> Py {
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
    let mut said_line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_line") { v.clone() }
    else { Py::None };
    let _ = py_str("A plan said back, with the day a name: \"in june\" -> \"in June\".");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plan.clone()); __args.push(said_line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_plan_said_back(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| named_days(args))(&__args) };
    Py::None
}
pub fn _the_plan_said_back(args: &[Py]) -> Py {
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
    let mut said_line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_line") { v.clone() }
    else { Py::None };
    let mut appointment: Py = Py::None;
    let mut first: Py = Py::None;
    let mut out: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_sentence: Py = Py::None;
    let mut text: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("A plan said back to the person.\n\n    A plan they said as a sentence comes back as one: \"i start my new job on monday\" -> \"You\n    start your new job on monday\". A plan that is only the thing itself keeps the phrase after\n    \"you're planning\": \"The gym tomorrow\" -> \"You're planning the gym tomorrow\".\n\n    A plan kept as the bare phrase is often only half of what they said - \"we're going to spain\n    in july\" keeps \"Spain in july\", and \"You're planning spain in july\" is a machine saying it\n    back. So when the phrase alone needs the \"planning\" lead-in and the line it came from is a\n    sentence of their own (\"said_line\"), that sentence is what is said back: \"You're going to\n    spain in july\".");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .!?,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::filler::without_filler(args))(&__args) }, "strip", args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .!?,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+)?(?:also|oh|plus|by the way|btw|anyway|anyways|so|well|right|ok|okay|hey|and)\\b[\\s,]*")); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^to\\s+")); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_str("");
    }
    said_sentence = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .!?,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said_line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::filler::without_filler(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::memory::facts::no_tone(args))(&__args) }, "strip", args))(&__args) };
    theirs = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said_sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLAN_OPENS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_sentence.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said_sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_JOINED_SUBJECT(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } };
    appointment = if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said_line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_journey_line(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_name_at_the_front(args))(&__args) } } }) { py_str("") } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| appointment_sentence(args))(&__args) } };
    if py_truthy(&appointment.clone()) {
    if py_truthy(&{ let __b = theirs.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said_sentence.clone(), "lower", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) })) } }) {
    out = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said_sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| you(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _mid(args))(&__args) }, "rstrip", args))(&__args) };
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&out.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&out.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return appointment.clone();
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^to\\s+")); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^plans?\\s+to\\s+")); __args.push(py_str("planning to ")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g_PLAN_VERBS(), &first.clone()))) {
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| you(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _mid(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&op_add(&py_str("You're "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&phrase.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", args))(&__args) }), &slice_or_raise(&phrase.clone(), &Py::Int(1), &Py::None, &Py::None)), "rstrip", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLAN_OPENS(), &first.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_JOINED_SUBJECT(), "match", args))(&__args) } } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:im|ive|ill)\\b")); __args.push({ let appointment = appointment.clone(); let first = first.clone(); let out = out.clone(); let phrase = phrase.clone(); let plan = plan.clone(); let said = said.clone(); let said_line = said_line.clone(); let said_sentence = said_sentence.clone(); let text = text.clone(); let theirs = theirs.clone(); py_func("lambda", move |args: &[Py]| { let appointment = appointment.clone(); let first = first.clone(); let out = out.clone(); let phrase = phrase.clone(); let plan = plan.clone(); let said = said.clone(); let said_line = said_line.clone(); let said_sentence = said_sentence.clone(); let text = text.clone(); let theirs = theirs.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); op_add(&py_str("i'"), &slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&m.clone(), "group", args))(&__args) }, &Py::Int(1), &Py::None, &Py::None)) }) }); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwere\\b")); __args.push(py_str("we're")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bweve\\b")); __args.push(py_str("we've")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    out = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| you(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _mid(args))(&__args) }, "rstrip", args))(&__args) };
    return op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&out.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&out.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = said_sentence.clone(); if !py_truthy(&__b) { __b } else { theirs.clone() } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said_sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| plan_sentence(args))(&__args) };
    }
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| you(args))(&__args) };
    phrase = op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&phrase.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", args))(&__args) }, &slice_or_raise(&phrase.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&py_str("You're "), &phrase.clone()), "rstrip", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::memory::facts::g_DETERMINERS(), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::memory::facts::g_COUNTRIES(), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::memory::facts::g_NOT_PLACES(), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _places(args))(&__args) }, &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _not_places(args))(&__args) }, &first.clone())) } } } } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&py_str("You're planning "), &phrase.clone()), "rstrip", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "lower", args))(&__args) }, "startswith", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&py_str("You're planning "), &phrase.clone()), "rstrip", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&py_str("You're planning to "), &phrase.clone()), "rstrip", args))(&__args) };
    Py::None
}
pub fn _mid(args: &[Py]) -> Py {
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
    let _ = py_str("A \"You\" that ends up in the middle of a sentence goes back to lower case: the only reason\n    it was capitalised is that it replaced an \"I\" (\"how much should I put away\").");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[\\s,])You\\b")); __args.push(py_str("you")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _to_the_bot(args: &[Py]) -> Py {
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
    let mut head: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Mark the \"you\" that is the bot in a line about the person's own doing.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    head = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_SUBJECT_HEADS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_contains(&g_FIRST_PERSON_HEADS(), &head.clone()))) {
    return text.clone();
    }
pub fn __nested_mark_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut head: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut mark: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut after: Py = Py::None;
    let mut marker: Py = Py::None;
    let mut new: Py = Py::None;
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&Py::Bool(!py_contains(&op_bitor(&g_SAYS_TO_A_PERSON(), &g_DONE_TO_A_PERSON()), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("thank"), py_str("thanks")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) }))) {
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[\\s,]*([a-z']+)")); __args.push(slice_or_raise(&gattr(&match_.clone(), "string"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("for"), py_str("again"), py_str("so"), py_str("too")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&after.clone(), "group", args))(&__args) }, "lower", args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    }
    }
    new = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }, &py_str("your")))) { py_str("my") } else { py_str("me") };
    marker = if py_truthy(&Py::Bool(py_eq(&new.clone(), &py_str("my")))) { g__BOT_MY() } else { g__BOT_ME() };
    return op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, &py_str(" ")), &if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&marker.clone(), "upper", args))(&__args) } } else { marker.clone() });
    Py::None
}
    mark = {{ let head = head.clone(); let mark = mark.clone(); let text = text.clone(); let word = word.clone(); let words = words.clone(); py_func("mark", move |args: &[Py]| { let mut full = vec![head.clone(), mark.clone(), text.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_mark_0(&full) }) }};
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b([a-z']+) +\\b(you|your)\\b")); __args.push(mark.clone()); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _from_the_bot(args: &[Py]) -> Py {
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
    let _ = py_str("The marked words go back to the bot's own person: \"you will always be\" -> \"me\".");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__BOT_MY()); __args.push(py_str("my")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__BOT_ME()); __args.push(py_str("me")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "replace", args))(&__args) }, "replace", args))(&__args) };
    Py::None
}
pub fn _places(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let __try1 = exc::catch(|| {
    let mut PLACES: Py = gen_understanding::understanding::topics::g_PLACES();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ImportError"]) {
                __handled5 = true;
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return PLACES.clone();
    Py::None
}
pub fn _not_places(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let __try1 = exc::catch(|| {
    let mut NOT_PLACES: Py = gen_understanding::understanding::topics::g_NOT_PLACES();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ImportError"]) {
                __handled5 = true;
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return NOT_PLACES.clone();
    Py::None
}
pub fn _outside_quotes(args: &[Py]) -> Py {
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
    let mut repair: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "repair") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: repair"))) };
    let mut found: Py = Py::None;
    let mut last: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("`repair` over the parts of the line that are the person's own words, quotes left alone.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUOTED(), "search", args))(&__args) })) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&repair, args))(&__args) };
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    out = __unpk1[0].clone();
    last = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_QUOTED(), "finditer", args))(&__args) });
    for __item in __iter2 {
    found = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &last.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "start", args))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&repair, args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) };
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &last.clone(), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&repair, args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(out.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(""), "join", args))(&__args) };
    Py::None
}
pub fn without_the_apostrophe(args: &[Py]) -> Py {
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
    let mut put_back: Py = Py::None;
    let _ = py_str("The person's words with the apostrophes they left out put back: \"shes 72\" -> \"she's 72\".\n\n    Only the store's own sentence is written this way; a line handed back in quotation marks is\n    their words as they typed them, and is left alone.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return text.clone();
    }
pub fn __nested_put_back_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut put_back: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_APOSTROPHE(), "get", args))(&__args) };
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) }) { op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&other.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&other.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { other.clone() };
    Py::None
}
    put_back = {{ let put_back = put_back.clone(); let text = text.clone(); py_func("put_back", move |args: &[Py]| { let mut full = vec![put_back.clone(), text.clone()]; full.extend_from_slice(args); __nested_put_back_0(&full) }) }};
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push({ let put_back = put_back.clone(); let text = text.clone(); py_func("lambda", move |args: &[Py]| { let put_back = put_back.clone(); let text = text.clone(); let part: Py = __kwp(args, 0, "part").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(put_back.clone()); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_APOSTROPHE_WORD(), "sub", args))(&__args) } }) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _outside_quotes(args))(&__args) };
    Py::None
}
pub fn _the_relation_possessive(args: &[Py]) -> Py {
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
    let mut own: Py = Py::None;
    let _ = py_str("The \"s\" of a relation with something after it becomes the possessive it means: \"my\n    mums birthday\" -> \"my mum's birthday\", and \"my daughters school\" -> \"my daughter's school\".\n\n    Only a relation the store knows, and only when the word after it is a thing it could own:\n    \"my sisters are coming\" is two people, not a possessive, and neither is \"seeing my sisters\n    tomorrow\".\n    ");
pub fn __nested_own_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut own: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut relation: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut whole: Py = Py::None;
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }]), 3, None);
    whole = __unpk1[0].clone();
    relation = __unpk1[1].clone();
    thing = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::memory::facts::g_RELATIONS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&relation.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_OWNED(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) })) } }) {
    return whole.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WHEN_OR_WHERE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_somebody(args))(&__args) } } }) {
    return whole.clone();
    }
    return op_add(&op_add(&op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, &py_str(" ")), &slice_or_raise(&relation.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)), &py_str("'s ")), &thing.clone());
    Py::None
}
    own = {{ let own = own.clone(); let text = text.clone(); py_func("own", move |args: &[Py]| { let mut full = vec![own.clone(), text.clone()]; full.extend_from_slice(args); __nested_own_0(&full) }) }};
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b((?:[Mm]y|[Oo]ur)) ([a-z]+s) ([A-Za-z0-9']+)")); __args.push(own.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_possessive_a_relation_means(args: &[Py]) -> Py {
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
    let mut owned: Py = Py::None;
    let mut relation: Py = Py::None;
    let _ = py_str("The possessive a relation means when its determiner is not in the phrase: \"sons parents\n    evening\" -> \"son's parents evening\".\n\n    The store keeps a plan as the thing itself, and the \"my\" that said whose it is stays with\n    the line: \"my sons parents evening is on thursday\" is kept as \"sons parents evening on\n    thursday\". Said back whole it is the line that says it (\"your son's parents evening is on\n    Thursday\"), but a plan said back as a thing of theirs has only its own words to stand on,\n    and \"your sons parents evening\" reads as a typo.\n\n    The same guards as the determiner's version: a relation the store knows, a thing it could\n    own after it, so \"sons are coming\" and \"sons tonight\" keep their plural.\n    ");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("([A-Za-z]+s)\\s+([A-Za-z0-9']+)")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return phrase.clone();
    }
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }]), 2, None);
    relation = __unpk1[0].clone();
    owned = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::memory::facts::g_RELATIONS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&relation.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_OWNED(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&owned.clone(), "lower", args))(&__args) })) } }) {
    return phrase.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_WHEN_OR_WHERE(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&owned.clone(), "lower", args))(&__args) }))) {
    return phrase.clone();
    }
    return op_add(&op_add(&slice_or_raise(&relation.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("'s ")), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "strip", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(relation.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None), "lstrip", args))(&__args) });
    Py::None
}
pub fn _with_somebody(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let _ = py_str("True when the relation is somebody the line says they are with or going to see.\n\n    The verb of being with somebody has to stand next to the relation: \"seeing my sisters\n    tomorrow\", \"text my sisters later\". A \"to\" in between names the place they are going to, and\n    a place is somebody's (\"off to my brother's tonight\"), so the possessive is right there.\n    ");
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &Py::None, &start.clone(), &Py::None), "rstrip", args))(&__args) }, "split", args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WITH_SOMEBODY(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&before.clone(), &op_neg(&Py::Int(1))), "lower", args))(&__args) })) } };
    Py::None
}
pub fn you(args: &[Py]) -> Py {
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
    let _ = py_str("A fact in the user's own words, reworded to say it back to them: \"my daughter's\n    birthday\" -> \"your daughter's birthday\". A quotation in the line is somebody else's words\n    and is left as it was said; outside one, the \"s\" a relation was typed with becomes the\n    possessive it means (\"your mums birthday\" -> \"your mum's birthday\").");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push({ let text = text.clone(); py_func("lambda", move |args: &[Py]| { let text = text.clone(); let part: Py = __kwp(args, 0, "part").unwrap_or(Py::None); { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_relation_possessive(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _flip_to_the_second_person(args))(&__args) } }) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _outside_quotes(args))(&__args) };
    Py::None
}
pub fn _flip_to_the_second_person(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut owns: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said_me: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut swap: Py = Py::None;
    let mut swaps: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let mut yours: Py = Py::None;
    let _ = py_str("`you`'s reading, over one stretch of the line: the first person becomes the second.");
    swaps = py_dict(vec![(py_str("i'm"), py_str("you're")), (py_str("i am"), py_str("you are")), (py_str("i've"), py_str("you've")), (py_str("i have"), py_str("you have")), (py_str("i'll"), py_str("you'll")), (py_str("we'll"), py_str("you'll")), (py_str("im"), py_str("you're")), (py_str("ive"), py_str("you've")), (py_str("i was not"), py_str("you were not")), (py_str("i wasn't"), py_str("you weren't")), (py_str("i wasnt"), py_str("you weren't")), (py_str("i was"), py_str("you were")), (py_str("i were"), py_str("you were")), (py_str("we was"), py_str("you were")), (py_str("we wasnt"), py_str("you weren't")), (py_str("my"), py_str("your")), (py_str("mine"), py_str("yours")), (py_str("myself"), py_str("yourself")), (py_str("me"), py_str("you")), (py_str("we"), py_str("you")), (py_str("our"), py_str("your")), (py_str("ourselves"), py_str("yourselves")), (py_str("us"), py_str("you")), (py_str("i"), py_str("you"))]);
pub fn __nested_swap_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut found: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut index: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut mine: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut owns: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut place: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut said_me: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut seen: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut swap: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut swaps: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut yours: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(13).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut other: Py = Py::None;
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&swaps.clone(), "get", args))(&__args) };
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) }) { op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&other.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", args))(&__args) }, &slice_or_raise(&other.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { other.clone() };
    Py::None
}
    swap = {{ let found = found.clone(); let index = index.clone(); let mine = mine.clone(); let owns = owns.clone(); let place = place.clone(); let said_me = said_me.clone(); let seen = seen.clone(); let swap = swap.clone(); let swaps = swaps.clone(); let text = text.clone(); let word = word.clone(); let words = words.clone(); let yours = yours.clone(); py_func("swap", move |args: &[Py]| { let mut full = vec![found.clone(), index.clone(), mine.clone(), owns.clone(), place.clone(), said_me.clone(), seen.clone(), swap.clone(), swaps.clone(), text.clone(), word.clone(), words.clone(), yours.clone()]; full.extend_from_slice(args); __nested_swap_0(&full) }) }};
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_relation_possessive(args))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _to_the_bot(args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    mine = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::normalize::possessive_me(args))(&__args) };
    said_me = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk8008 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk8008[0].clone(); let mut word: Py = __unpk8008[1].clone(); if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("me")))) {  __out.push(index.clone()); } } py_list(__out) })() };
    owns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said_me.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk6008 = py_unpack(&__item.clone(), 2, None); let mut place: Py = __unpk6008[0].clone(); let mut index: Py = __unpk6008[1].clone(); if py_truthy(&{ let __b = op_lt(&index.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(mine.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&mine.clone(), &index.clone()), &py_str("my"))) } }) {  __out.push(place.clone()); } } py_set(__out) })() };
    let __aug2 = op_sub(&owns.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bme\\b")); __args.push(text.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk6632 = py_unpack(&__item.clone(), 2, None); let mut place: Py = __unpk6632[0].clone(); let mut found: Py = __unpk6632[1].clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*,")); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "match", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(place.clone()); } } py_set(__out) })() });
    owns = __aug2;
    if py_truthy(&owns.clone()) {
    seen = py_list(vec![op_neg(&Py::Int(1))]);
pub fn __nested_yours_2(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut found: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut index: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut mine: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut owns: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut place: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut said_me: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut seen: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut swap: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut swaps: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut yours: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(13).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let __aug1 = op_add(&geti(&seen.clone(), &Py::Int(0)), &Py::Int(1));
    let _ = py_setitem(&seen.clone(), &Py::Int(0), __aug1);
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&owns.clone(), &geti(&seen.clone(), &Py::Int(0))))) {
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) }) { py_str("Your") } else { py_str("your") };
    }
    return word.clone();
    Py::None
}
    yours = {{ let found = found.clone(); let index = index.clone(); let mine = mine.clone(); let owns = owns.clone(); let place = place.clone(); let said_me = said_me.clone(); let seen = seen.clone(); let swap = swap.clone(); let swaps = swaps.clone(); let text = text.clone(); let word = word.clone(); let words = words.clone(); let yours = yours.clone(); py_func("yours", move |args: &[Py]| { let mut full = vec![found.clone(), index.clone(), mine.clone(), owns.clone(), place.clone(), said_me.clone(), seen.clone(), swap.clone(), swaps.clone(), text.clone(), word.clone(), words.clone(), yours.clone()]; full.extend_from_slice(args); __nested_yours_2(&full) }) }};
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bme\\b")); __args.push(yours.clone()); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(i'm|i am|i've|i have|i was not|i wasn't|i wasnt|i was|i were|we wasnt|we was|im|ive|my|mine|myself|me|we|our|ourselves|us|i)\\b")); __args.push(swap.clone()); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _from_the_bot(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| without_the_apostrophe(args))(&__args) };
    Py::None
}
pub fn _statement(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut line: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The statement half of a line, or \"\" when the line is a question.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .!?,")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "strip", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "split", args))(&__args) };
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ASK_WORDS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",.;:")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&words.clone(), &Py::Int(0)), "lower", args))(&__args) }, "strip", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WERE_DOING(), "match", args))(&__args) })) } } } }) {
    return py_str("");
    }
    cut = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK_TAIL(), "search", args))(&__args) };
    if py_truthy(&cut.clone()) {
    line = slice_or_raise(&line.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cut.clone(), "start", args))(&__args) }, &Py::None);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" .,!?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) };
    Py::None
}
pub fn plan_line(args: &[Py]) -> Py {
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
    let mut said_line: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The line the person said that a plan came from, if it names that plan.\n\n    \"we're going to spain in july\" keeps the plan \"Spain in july\", and the phrase alone cannot\n    say who is going or that anybody is: the sentence it came from can. Only the statement part\n    of that line is a plan said back, so a question hanging off it is cut away.");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("plan"), py_str("plans")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("key")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &wanted.clone()))) {
    continue;
    }
    said_line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _statement(args))(&__args) };
    if py_truthy(&said_line.clone()) {
    return said_line.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn said(args: &[Py]) -> Py {
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
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut entry: Py = Py::None;
    let _ = py_str("The first value the user gave for a fact key, as (value, turn) or (None, None).");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| log(args))(&__args) });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("key")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }, &key.clone()))) {
    return py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entry.clone(), "get", args))(&__args) }]);
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANOTHER_ONE" => Some(g_ANOTHER_ONE()),
            "APOSTROPHE" => Some(g_APOSTROPHE()),
            "APOSTROPHE_WORD" => Some(g_APOSTROPHE_WORD()),
            "APPOINTMENT_A_DETERMINER" => Some(g_APPOINTMENT_A_DETERMINER()),
            "APPOINTMENT_A_PERSON_VERB" => Some(g_APPOINTMENT_A_PERSON_VERB()),
            "APPOINTMENT_A_PLURAL" => Some(g_APPOINTMENT_A_PLURAL()),
            "APPOINTMENT_A_QUANTITY" => Some(g_APPOINTMENT_A_QUANTITY()),
            "APPOINTMENT_DOING" => Some(g_APPOINTMENT_DOING()),
            "APPOINTMENT_NOT_A_PLURAL" => Some(g_APPOINTMENT_NOT_A_PLURAL()),
            "APPOINTMENT_PLAN" => Some(g_APPOINTMENT_PLAN()),
            "ASK_TAIL" => Some(g_ASK_TAIL()),
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "A_THING_DOES_BARE" => Some(g_A_THING_DOES_BARE()),
            "A_THING_DOES_S" => Some(g_A_THING_DOES_S()),
            "COUNTRIES" => Some(g_COUNTRIES()),
            "COUNT_HEAD" => Some(g_COUNT_HEAD()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "DONE_TO_A_PERSON" => Some(g_DONE_TO_A_PERSON()),
            "FIRST_PERSON_HEADS" => Some(g_FIRST_PERSON_HEADS()),
            "JOINED_SUBJECT" => Some(g_JOINED_SUBJECT()),
            "JOURNEY_LINE" => Some(g_JOURNEY_LINE()),
            "KIND_ASK" => Some(g_KIND_ASK()),
            "LIST_FACTS" => Some(g_LIST_FACTS()),
            "MONTHS" => Some(g_MONTHS()),
            "NOT_OWNED" => Some(g_NOT_OWNED()),
            "NOT_PLACES" => Some(g_NOT_PLACES()),
            "PERSON_TROUBLE" => Some(g_PERSON_TROUBLE()),
            "PETS_KEPT" => Some(g_PETS_KEPT()),
            "PLAN_OPENS" => Some(g_PLAN_OPENS()),
            "PLAN_VERBS" => Some(g_PLAN_VERBS()),
            "PREPOSITIONS" => Some(g_PREPOSITIONS()),
            "QUESTION_OPEN" => Some(g_QUESTION_OPEN()),
            "QUOTED" => Some(g_QUOTED()),
            "SAYS_TO_A_PERSON" => Some(g_SAYS_TO_A_PERSON()),
            "SUBJECT_HEADS" => Some(g_SUBJECT_HEADS()),
            "TIME_WORDS" => Some(g_TIME_WORDS()),
            "VERBISH" => Some(g_VERBISH()),
            "VERB_FORMS" => Some(g_VERB_FORMS()),
            "WERE_DOING" => Some(g_WERE_DOING()),
            "WHEN_OR_WHERE" => Some(g_WHEN_OR_WHERE()),
            "WHEN_WORDS" => Some(g_WHEN_WORDS()),
            "WHOSE_IT_IS" => Some(g_WHOSE_IT_IS()),
            "WHO_SAID" => Some(g_WHO_SAID()),
            "WITH_SOMEBODY" => Some(g_WITH_SOMEBODY()),
            "_A_DAY_NUMBER" => Some(g__A_DAY_NUMBER()),
            "_BOT_ME" => Some(g__BOT_ME()),
            "_BOT_MY" => Some(g__BOT_MY()),
            "_DATE_LEADS" => Some(g__DATE_LEADS()),
            "_WEEKDAY_NAMES" => Some(g__WEEKDAY_NAMES()),
            "__all__" => Some(g___all__()),
            "_a_doing" => Some(py_func("_a_doing", |args: &[Py]| _a_doing(args))),
            "_a_journey_line" => Some(py_func("_a_journey_line", |args: &[Py]| _a_journey_line(args))),
            "_a_name_at_the_front" => Some(py_func("_a_name_at_the_front", |args: &[Py]| _a_name_at_the_front(args))),
            "_a_plural" => Some(py_func("_a_plural", |args: &[Py]| _a_plural(args))),
            "_another_of_them" => Some(py_func("_another_of_them", |args: &[Py]| _another_of_them(args))),
            "_flip_to_the_second_person" => Some(py_func("_flip_to_the_second_person", |args: &[Py]| _flip_to_the_second_person(args))),
            "_fold_into" => Some(py_func("_fold_into", |args: &[Py]| _fold_into(args))),
            "_from_the_bot" => Some(py_func("_from_the_bot", |args: &[Py]| _from_the_bot(args))),
            "_has_its_own_verb" => Some(py_func("_has_its_own_verb", |args: &[Py]| _has_its_own_verb(args))),
            "_mid" => Some(py_func("_mid", |args: &[Py]| _mid(args))),
            "_not_places" => Some(py_func("_not_places", |args: &[Py]| _not_places(args))),
            "_outside_quotes" => Some(py_func("_outside_quotes", |args: &[Py]| _outside_quotes(args))),
            "_places" => Some(py_func("_places", |args: &[Py]| _places(args))),
            "_said_another" => Some(py_func("_said_another", |args: &[Py]| _said_another(args))),
            "_statement" => Some(py_func("_statement", |args: &[Py]| _statement(args))),
            "_takes_a_thing" => Some(py_func("_takes_a_thing", |args: &[Py]| _takes_a_thing(args))),
            "_the_plan_said_back" => Some(py_func("_the_plan_said_back", |args: &[Py]| _the_plan_said_back(args))),
            "_the_possessive_a_relation_means" => Some(py_func("_the_possessive_a_relation_means", |args: &[Py]| _the_possessive_a_relation_means(args))),
            "_the_relation_possessive" => Some(py_func("_the_relation_possessive", |args: &[Py]| _the_relation_possessive(args))),
            "_the_thing_the_plan_is" => Some(py_func("_the_thing_the_plan_is", |args: &[Py]| _the_thing_the_plan_is(args))),
            "_to_the_bot" => Some(py_func("_to_the_bot", |args: &[Py]| _to_the_bot(args))),
            "_with_somebody" => Some(py_func("_with_somebody", |args: &[Py]| _with_somebody(args))),
            "appointment_sentence" => Some(py_func("appointment_sentence", |args: &[Py]| appointment_sentence(args))),
            "forget" => Some(py_func("forget", |args: &[Py]| forget(args))),
            "last_bot" => Some(py_func("last_bot", |args: &[Py]| last_bot(args))),
            "last_module" => Some(py_func("last_module", |args: &[Py]| last_module(args))),
            "last_user" => Some(py_func("last_user", |args: &[Py]| last_user(args))),
            "log" => Some(py_func("log", |args: &[Py]| log(args))),
            "name" => Some(py_func("name", |args: &[Py]| name(args))),
            "named_days" => Some(py_func("named_days", |args: &[Py]| named_days(args))),
            "others" => Some(py_func("others", |args: &[Py]| others(args))),
            "plan_line" => Some(py_func("plan_line", |args: &[Py]| plan_line(args))),
            "plan_sentence" => Some(py_func("plan_sentence", |args: &[Py]| plan_sentence(args))),
            "reads_as_a_plan" => Some(py_func("reads_as_a_plan", |args: &[Py]| reads_as_a_plan(args))),
            "recent" => Some(py_func("recent", |args: &[Py]| recent(args))),
            "remember" => Some(py_func("remember", |args: &[Py]| remember(args))),
            "remember_other" => Some(py_func("remember_other", |args: &[Py]| remember_other(args))),
            "said" => Some(py_func("said", |args: &[Py]| said(args))),
            "there_is_a_chat" => Some(py_func("there_is_a_chat", |args: &[Py]| there_is_a_chat(args))),
            "user" => Some(py_func("user", |args: &[Py]| user(args))),
            "without_the_apostrophe" => Some(py_func("without_the_apostrophe", |args: &[Py]| without_the_apostrophe(args))),
            "you" => Some(py_func("you", |args: &[Py]| you(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANOTHER_ONE" => s_ANOTHER_ONE(v),
            "APOSTROPHE" => s_APOSTROPHE(v),
            "APOSTROPHE_WORD" => s_APOSTROPHE_WORD(v),
            "APPOINTMENT_A_DETERMINER" => s_APPOINTMENT_A_DETERMINER(v),
            "APPOINTMENT_A_PERSON_VERB" => s_APPOINTMENT_A_PERSON_VERB(v),
            "APPOINTMENT_A_PLURAL" => s_APPOINTMENT_A_PLURAL(v),
            "APPOINTMENT_A_QUANTITY" => s_APPOINTMENT_A_QUANTITY(v),
            "APPOINTMENT_DOING" => s_APPOINTMENT_DOING(v),
            "APPOINTMENT_NOT_A_PLURAL" => s_APPOINTMENT_NOT_A_PLURAL(v),
            "APPOINTMENT_PLAN" => s_APPOINTMENT_PLAN(v),
            "ASK_TAIL" => s_ASK_TAIL(v),
            "ASK_WORDS" => s_ASK_WORDS(v),
            "A_THING_DOES_BARE" => s_A_THING_DOES_BARE(v),
            "A_THING_DOES_S" => s_A_THING_DOES_S(v),
            "COUNTRIES" => s_COUNTRIES(v),
            "COUNT_HEAD" => s_COUNT_HEAD(v),
            "DETERMINERS" => s_DETERMINERS(v),
            "DONE_TO_A_PERSON" => s_DONE_TO_A_PERSON(v),
            "FIRST_PERSON_HEADS" => s_FIRST_PERSON_HEADS(v),
            "JOINED_SUBJECT" => s_JOINED_SUBJECT(v),
            "JOURNEY_LINE" => s_JOURNEY_LINE(v),
            "KIND_ASK" => s_KIND_ASK(v),
            "LIST_FACTS" => s_LIST_FACTS(v),
            "MONTHS" => s_MONTHS(v),
            "NOT_OWNED" => s_NOT_OWNED(v),
            "NOT_PLACES" => s_NOT_PLACES(v),
            "PERSON_TROUBLE" => s_PERSON_TROUBLE(v),
            "PETS_KEPT" => s_PETS_KEPT(v),
            "PLAN_OPENS" => s_PLAN_OPENS(v),
            "PLAN_VERBS" => s_PLAN_VERBS(v),
            "PREPOSITIONS" => s_PREPOSITIONS(v),
            "QUESTION_OPEN" => s_QUESTION_OPEN(v),
            "QUOTED" => s_QUOTED(v),
            "SAYS_TO_A_PERSON" => s_SAYS_TO_A_PERSON(v),
            "SUBJECT_HEADS" => s_SUBJECT_HEADS(v),
            "TIME_WORDS" => s_TIME_WORDS(v),
            "VERBISH" => s_VERBISH(v),
            "VERB_FORMS" => s_VERB_FORMS(v),
            "WERE_DOING" => s_WERE_DOING(v),
            "WHEN_OR_WHERE" => s_WHEN_OR_WHERE(v),
            "WHEN_WORDS" => s_WHEN_WORDS(v),
            "WHOSE_IT_IS" => s_WHOSE_IT_IS(v),
            "WHO_SAID" => s_WHO_SAID(v),
            "WITH_SOMEBODY" => s_WITH_SOMEBODY(v),
            "_A_DAY_NUMBER" => s__A_DAY_NUMBER(v),
            "_BOT_ME" => s__BOT_ME(v),
            "_BOT_MY" => s__BOT_MY(v),
            "_DATE_LEADS" => s__DATE_LEADS(v),
            "_WEEKDAY_NAMES" => s__WEEKDAY_NAMES(v),
            "__all__" => s___all__(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory", module_obj);
    pyrt::xrefs::reg_fn("memory", "user", py_func("user", |args: &[Py]| user(args)));
    pyrt::xrefs::reg_fn("memory", "log", py_func("log", |args: &[Py]| log(args)));
    pyrt::xrefs::reg_fn("memory", "others", py_func("others", |args: &[Py]| others(args)));
    pyrt::xrefs::reg_fn("memory", "_said_another", py_func("_said_another", |args: &[Py]| _said_another(args)));
    pyrt::xrefs::reg_fn("memory", "_another_of_them", py_func("_another_of_them", |args: &[Py]| _another_of_them(args)));
    pyrt::xrefs::reg_fn("memory", "remember_other", py_func("remember_other", |args: &[Py]| remember_other(args)));
    pyrt::xrefs::reg_fn("memory", "_fold_into", py_func("_fold_into", |args: &[Py]| _fold_into(args)));
    pyrt::xrefs::reg_fn("memory", "reads_as_a_plan", py_func("reads_as_a_plan", |args: &[Py]| reads_as_a_plan(args)));
    pyrt::xrefs::reg_fn("memory", "_the_thing_the_plan_is", py_func("_the_thing_the_plan_is", |args: &[Py]| _the_thing_the_plan_is(args)));
    pyrt::xrefs::reg_fn("memory", "remember", py_func("remember", |args: &[Py]| remember(args)));
    pyrt::xrefs::reg_fn("memory", "forget", py_func("forget", |args: &[Py]| forget(args)));
    pyrt::xrefs::reg_fn("memory", "there_is_a_chat", py_func("there_is_a_chat", |args: &[Py]| there_is_a_chat(args)));
    pyrt::xrefs::reg_fn("memory", "recent", py_func("recent", |args: &[Py]| recent(args)));
    pyrt::xrefs::reg_fn("memory", "last_user", py_func("last_user", |args: &[Py]| last_user(args)));
    pyrt::xrefs::reg_fn("memory", "last_bot", py_func("last_bot", |args: &[Py]| last_bot(args)));
    pyrt::xrefs::reg_fn("memory", "last_module", py_func("last_module", |args: &[Py]| last_module(args)));
    pyrt::xrefs::reg_fn("memory", "name", py_func("name", |args: &[Py]| name(args)));
    pyrt::xrefs::reg_fn("memory", "_takes_a_thing", py_func("_takes_a_thing", |args: &[Py]| _takes_a_thing(args)));
    pyrt::xrefs::reg_fn("memory", "_a_doing", py_func("_a_doing", |args: &[Py]| _a_doing(args)));
    pyrt::xrefs::reg_fn("memory", "named_days", py_func("named_days", |args: &[Py]| named_days(args)));
    pyrt::xrefs::reg_fn("memory", "_a_name_at_the_front", py_func("_a_name_at_the_front", |args: &[Py]| _a_name_at_the_front(args)));
    pyrt::xrefs::reg_fn("memory", "_a_plural", py_func("_a_plural", |args: &[Py]| _a_plural(args)));
    pyrt::xrefs::reg_fn("memory", "_has_its_own_verb", py_func("_has_its_own_verb", |args: &[Py]| _has_its_own_verb(args)));
    pyrt::xrefs::reg_fn("memory", "appointment_sentence", py_func("appointment_sentence", |args: &[Py]| appointment_sentence(args)));
    pyrt::xrefs::reg_fn("memory", "_a_journey_line", py_func("_a_journey_line", |args: &[Py]| _a_journey_line(args)));
    pyrt::xrefs::reg_fn("memory", "plan_sentence", py_func("plan_sentence", |args: &[Py]| plan_sentence(args)));
    pyrt::xrefs::reg_fn("memory", "_the_plan_said_back", py_func("_the_plan_said_back", |args: &[Py]| _the_plan_said_back(args)));
    pyrt::xrefs::reg_fn("memory", "_mid", py_func("_mid", |args: &[Py]| _mid(args)));
    pyrt::xrefs::reg_fn("memory", "_to_the_bot", py_func("_to_the_bot", |args: &[Py]| _to_the_bot(args)));
    pyrt::xrefs::reg_fn("memory", "_from_the_bot", py_func("_from_the_bot", |args: &[Py]| _from_the_bot(args)));
    pyrt::xrefs::reg_fn("memory", "_places", py_func("_places", |args: &[Py]| _places(args)));
    pyrt::xrefs::reg_fn("memory", "_not_places", py_func("_not_places", |args: &[Py]| _not_places(args)));
    pyrt::xrefs::reg_fn("memory", "_outside_quotes", py_func("_outside_quotes", |args: &[Py]| _outside_quotes(args)));
    pyrt::xrefs::reg_fn("memory", "without_the_apostrophe", py_func("without_the_apostrophe", |args: &[Py]| without_the_apostrophe(args)));
    pyrt::xrefs::reg_fn("memory", "_the_relation_possessive", py_func("_the_relation_possessive", |args: &[Py]| _the_relation_possessive(args)));
    pyrt::xrefs::reg_fn("memory", "_the_possessive_a_relation_means", py_func("_the_possessive_a_relation_means", |args: &[Py]| _the_possessive_a_relation_means(args)));
    pyrt::xrefs::reg_fn("memory", "_with_somebody", py_func("_with_somebody", |args: &[Py]| _with_somebody(args)));
    pyrt::xrefs::reg_fn("memory", "you", py_func("you", |args: &[Py]| you(args)));
    pyrt::xrefs::reg_fn("memory", "_flip_to_the_second_person", py_func("_flip_to_the_second_person", |args: &[Py]| _flip_to_the_second_person(args)));
    pyrt::xrefs::reg_fn("memory", "_statement", py_func("_statement", |args: &[Py]| _statement(args)));
    pyrt::xrefs::reg_fn("memory", "plan_line", py_func("plan_line", |args: &[Py]| plan_line(args)));
    pyrt::xrefs::reg_fn("memory", "said", py_func("said", |args: &[Py]| said(args)));
    pyrt::xrefs::reg_access("memory", "ANOTHER_ONE", g_ANOTHER_ONE, s_ANOTHER_ONE);
    pyrt::xrefs::reg_access("memory", "APOSTROPHE", g_APOSTROPHE, s_APOSTROPHE);
    pyrt::xrefs::reg_access("memory", "APOSTROPHE_WORD", g_APOSTROPHE_WORD, s_APOSTROPHE_WORD);
    pyrt::xrefs::reg_access("memory", "APPOINTMENT_A_DETERMINER", g_APPOINTMENT_A_DETERMINER, s_APPOINTMENT_A_DETERMINER);
    pyrt::xrefs::reg_access("memory", "APPOINTMENT_A_PERSON_VERB", g_APPOINTMENT_A_PERSON_VERB, s_APPOINTMENT_A_PERSON_VERB);
    pyrt::xrefs::reg_access("memory", "APPOINTMENT_A_PLURAL", g_APPOINTMENT_A_PLURAL, s_APPOINTMENT_A_PLURAL);
    pyrt::xrefs::reg_access("memory", "APPOINTMENT_A_QUANTITY", g_APPOINTMENT_A_QUANTITY, s_APPOINTMENT_A_QUANTITY);
    pyrt::xrefs::reg_access("memory", "APPOINTMENT_DOING", g_APPOINTMENT_DOING, s_APPOINTMENT_DOING);
    pyrt::xrefs::reg_access("memory", "APPOINTMENT_NOT_A_PLURAL", g_APPOINTMENT_NOT_A_PLURAL, s_APPOINTMENT_NOT_A_PLURAL);
    pyrt::xrefs::reg_access("memory", "APPOINTMENT_PLAN", g_APPOINTMENT_PLAN, s_APPOINTMENT_PLAN);
    pyrt::xrefs::reg_access("memory", "ASK_TAIL", g_ASK_TAIL, s_ASK_TAIL);
    pyrt::xrefs::reg_access("memory", "ASK_WORDS", g_ASK_WORDS, s_ASK_WORDS);
    pyrt::xrefs::reg_access("memory", "A_THING_DOES_BARE", g_A_THING_DOES_BARE, s_A_THING_DOES_BARE);
    pyrt::xrefs::reg_access("memory", "A_THING_DOES_S", g_A_THING_DOES_S, s_A_THING_DOES_S);
    pyrt::xrefs::reg_access("memory", "COUNTRIES", g_COUNTRIES, s_COUNTRIES);
    pyrt::xrefs::reg_access("memory", "COUNT_HEAD", g_COUNT_HEAD, s_COUNT_HEAD);
    pyrt::xrefs::reg_access("memory", "DETERMINERS", g_DETERMINERS, s_DETERMINERS);
    pyrt::xrefs::reg_access("memory", "DONE_TO_A_PERSON", g_DONE_TO_A_PERSON, s_DONE_TO_A_PERSON);
    pyrt::xrefs::reg_access("memory", "FIRST_PERSON_HEADS", g_FIRST_PERSON_HEADS, s_FIRST_PERSON_HEADS);
    pyrt::xrefs::reg_access("memory", "JOINED_SUBJECT", g_JOINED_SUBJECT, s_JOINED_SUBJECT);
    pyrt::xrefs::reg_access("memory", "JOURNEY_LINE", g_JOURNEY_LINE, s_JOURNEY_LINE);
    pyrt::xrefs::reg_access("memory", "KIND_ASK", g_KIND_ASK, s_KIND_ASK);
    pyrt::xrefs::reg_access("memory", "LIST_FACTS", g_LIST_FACTS, s_LIST_FACTS);
    pyrt::xrefs::reg_access("memory", "MONTHS", g_MONTHS, s_MONTHS);
    pyrt::xrefs::reg_access("memory", "NOT_OWNED", g_NOT_OWNED, s_NOT_OWNED);
    pyrt::xrefs::reg_access("memory", "NOT_PLACES", g_NOT_PLACES, s_NOT_PLACES);
    pyrt::xrefs::reg_access("memory", "PERSON_TROUBLE", g_PERSON_TROUBLE, s_PERSON_TROUBLE);
    pyrt::xrefs::reg_access("memory", "PETS_KEPT", g_PETS_KEPT, s_PETS_KEPT);
    pyrt::xrefs::reg_access("memory", "PLAN_OPENS", g_PLAN_OPENS, s_PLAN_OPENS);
    pyrt::xrefs::reg_access("memory", "PLAN_VERBS", g_PLAN_VERBS, s_PLAN_VERBS);
    pyrt::xrefs::reg_access("memory", "PREPOSITIONS", g_PREPOSITIONS, s_PREPOSITIONS);
    pyrt::xrefs::reg_access("memory", "QUESTION_OPEN", g_QUESTION_OPEN, s_QUESTION_OPEN);
    pyrt::xrefs::reg_access("memory", "QUOTED", g_QUOTED, s_QUOTED);
    pyrt::xrefs::reg_access("memory", "SAYS_TO_A_PERSON", g_SAYS_TO_A_PERSON, s_SAYS_TO_A_PERSON);
    pyrt::xrefs::reg_access("memory", "SUBJECT_HEADS", g_SUBJECT_HEADS, s_SUBJECT_HEADS);
    pyrt::xrefs::reg_access("memory", "TIME_WORDS", g_TIME_WORDS, s_TIME_WORDS);
    pyrt::xrefs::reg_access("memory", "VERBISH", g_VERBISH, s_VERBISH);
    pyrt::xrefs::reg_access("memory", "VERB_FORMS", g_VERB_FORMS, s_VERB_FORMS);
    pyrt::xrefs::reg_access("memory", "WERE_DOING", g_WERE_DOING, s_WERE_DOING);
    pyrt::xrefs::reg_access("memory", "WHEN_OR_WHERE", g_WHEN_OR_WHERE, s_WHEN_OR_WHERE);
    pyrt::xrefs::reg_access("memory", "WHEN_WORDS", g_WHEN_WORDS, s_WHEN_WORDS);
    pyrt::xrefs::reg_access("memory", "WHOSE_IT_IS", g_WHOSE_IT_IS, s_WHOSE_IT_IS);
    pyrt::xrefs::reg_access("memory", "WHO_SAID", g_WHO_SAID, s_WHO_SAID);
    pyrt::xrefs::reg_access("memory", "WITH_SOMEBODY", g_WITH_SOMEBODY, s_WITH_SOMEBODY);
    pyrt::xrefs::reg_access("memory", "_A_DAY_NUMBER", g__A_DAY_NUMBER, s__A_DAY_NUMBER);
    pyrt::xrefs::reg_access("memory", "_BOT_ME", g__BOT_ME, s__BOT_ME);
    pyrt::xrefs::reg_access("memory", "_BOT_MY", g__BOT_MY, s__BOT_MY);
    pyrt::xrefs::reg_access("memory", "_DATE_LEADS", g__DATE_LEADS, s__DATE_LEADS);
    pyrt::xrefs::reg_access("memory", "_WEEKDAY_NAMES", g__WEEKDAY_NAMES, s__WEEKDAY_NAMES);
    pyrt::xrefs::reg_access("memory", "__all__", g___all__, s___all__);
}
