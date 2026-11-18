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
thread_local! { static G_a_detail_of_the_thing_we_hold: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_about: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_action: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_already_said: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_answer_about: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_answer_in_hand: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_answers_about_another_creature: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_asks_only: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_bare_question: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_content: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_context: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_current: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_answer: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_expand: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_followup_reading: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_forms: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_gist: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_has_time: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_introduction_question: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_is_follow_up: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_key_subject: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_keywords: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_left_for_a_blank: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_meal_follow_up: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_note: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_person: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_person_named: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_possessive_me: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_remember_answer: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_resumed: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_return_target: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_said_again: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_same_as_before: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_sized_reply: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_stem: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_subject: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_animal_the_subject_is: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_ask_about_the_thing_it_points_at: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_step_back_takes_its_words_back: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_thing_a_detail_belongs_to: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_thing_it_is: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_topic_text: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_trouble: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_without_filler: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_written_out: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_a_detail_of_the_thing_we_hold() -> Py { ensure_init(); G_a_detail_of_the_thing_we_hold.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_detail_of_the_thing_we_hold(v: Py) { ensure_init(); G_a_detail_of_the_thing_we_hold.with(|c| *c.borrow_mut() = v); }
pub fn g_about() -> Py { ensure_init(); G_about.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_about(v: Py) { ensure_init(); G_about.with(|c| *c.borrow_mut() = v); }
pub fn g_action() -> Py { ensure_init(); G_action.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_action(v: Py) { ensure_init(); G_action.with(|c| *c.borrow_mut() = v); }
pub fn g_already_said() -> Py { ensure_init(); G_already_said.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_already_said(v: Py) { ensure_init(); G_already_said.with(|c| *c.borrow_mut() = v); }
pub fn g_answer_about() -> Py { ensure_init(); G_answer_about.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_answer_about(v: Py) { ensure_init(); G_answer_about.with(|c| *c.borrow_mut() = v); }
pub fn g_answer_in_hand() -> Py { ensure_init(); G_answer_in_hand.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_answer_in_hand(v: Py) { ensure_init(); G_answer_in_hand.with(|c| *c.borrow_mut() = v); }
pub fn g_answers_about_another_creature() -> Py { ensure_init(); G_answers_about_another_creature.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_answers_about_another_creature(v: Py) { ensure_init(); G_answers_about_another_creature.with(|c| *c.borrow_mut() = v); }
pub fn g_asks_only() -> Py { ensure_init(); G_asks_only.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_asks_only(v: Py) { ensure_init(); G_asks_only.with(|c| *c.borrow_mut() = v); }
pub fn g_bare_question() -> Py { ensure_init(); G_bare_question.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_bare_question(v: Py) { ensure_init(); G_bare_question.with(|c| *c.borrow_mut() = v); }
pub fn g_content() -> Py { ensure_init(); G_content.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_content(v: Py) { ensure_init(); G_content.with(|c| *c.borrow_mut() = v); }
pub fn g_context() -> Py { ensure_init(); G_context.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_context(v: Py) { ensure_init(); G_context.with(|c| *c.borrow_mut() = v); }
pub fn g_current() -> Py { ensure_init(); G_current.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_current(v: Py) { ensure_init(); G_current.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_answer() -> Py { ensure_init(); G_entry_answer.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_answer(v: Py) { ensure_init(); G_entry_answer.with(|c| *c.borrow_mut() = v); }
pub fn g_expand() -> Py { ensure_init(); G_expand.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_expand(v: Py) { ensure_init(); G_expand.with(|c| *c.borrow_mut() = v); }
pub fn g_followup_reading() -> Py { ensure_init(); G_followup_reading.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_followup_reading(v: Py) { ensure_init(); G_followup_reading.with(|c| *c.borrow_mut() = v); }
pub fn g_forms() -> Py { ensure_init(); G_forms.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_forms(v: Py) { ensure_init(); G_forms.with(|c| *c.borrow_mut() = v); }
pub fn g_gist() -> Py { ensure_init(); G_gist.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_gist(v: Py) { ensure_init(); G_gist.with(|c| *c.borrow_mut() = v); }
pub fn g_has_time() -> Py { ensure_init(); G_has_time.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_has_time(v: Py) { ensure_init(); G_has_time.with(|c| *c.borrow_mut() = v); }
pub fn g_introduction_question() -> Py { ensure_init(); G_introduction_question.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_introduction_question(v: Py) { ensure_init(); G_introduction_question.with(|c| *c.borrow_mut() = v); }
pub fn g_is_follow_up() -> Py { ensure_init(); G_is_follow_up.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_is_follow_up(v: Py) { ensure_init(); G_is_follow_up.with(|c| *c.borrow_mut() = v); }
pub fn g_key_subject() -> Py { ensure_init(); G_key_subject.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_key_subject(v: Py) { ensure_init(); G_key_subject.with(|c| *c.borrow_mut() = v); }
pub fn g_keywords() -> Py { ensure_init(); G_keywords.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_keywords(v: Py) { ensure_init(); G_keywords.with(|c| *c.borrow_mut() = v); }
pub fn g_left_for_a_blank() -> Py { ensure_init(); G_left_for_a_blank.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_left_for_a_blank(v: Py) { ensure_init(); G_left_for_a_blank.with(|c| *c.borrow_mut() = v); }
pub fn g_meal_follow_up() -> Py { ensure_init(); G_meal_follow_up.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_meal_follow_up(v: Py) { ensure_init(); G_meal_follow_up.with(|c| *c.borrow_mut() = v); }
pub fn g_note() -> Py { ensure_init(); G_note.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_note(v: Py) { ensure_init(); G_note.with(|c| *c.borrow_mut() = v); }
pub fn g_person() -> Py { ensure_init(); G_person.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_person(v: Py) { ensure_init(); G_person.with(|c| *c.borrow_mut() = v); }
pub fn g_person_named() -> Py { ensure_init(); G_person_named.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_person_named(v: Py) { ensure_init(); G_person_named.with(|c| *c.borrow_mut() = v); }
pub fn g_possessive_me() -> Py { ensure_init(); G_possessive_me.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_possessive_me(v: Py) { ensure_init(); G_possessive_me.with(|c| *c.borrow_mut() = v); }
pub fn g_remember_answer() -> Py { ensure_init(); G_remember_answer.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_remember_answer(v: Py) { ensure_init(); G_remember_answer.with(|c| *c.borrow_mut() = v); }
pub fn g_resumed() -> Py { ensure_init(); G_resumed.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_resumed(v: Py) { ensure_init(); G_resumed.with(|c| *c.borrow_mut() = v); }
pub fn g_return_target() -> Py { ensure_init(); G_return_target.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_return_target(v: Py) { ensure_init(); G_return_target.with(|c| *c.borrow_mut() = v); }
pub fn g_said_again() -> Py { ensure_init(); G_said_again.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_said_again(v: Py) { ensure_init(); G_said_again.with(|c| *c.borrow_mut() = v); }
pub fn g_same_as_before() -> Py { ensure_init(); G_same_as_before.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_same_as_before(v: Py) { ensure_init(); G_same_as_before.with(|c| *c.borrow_mut() = v); }
pub fn g_sized_reply() -> Py { ensure_init(); G_sized_reply.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_sized_reply(v: Py) { ensure_init(); G_sized_reply.with(|c| *c.borrow_mut() = v); }
pub fn g_stem() -> Py { ensure_init(); G_stem.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_stem(v: Py) { ensure_init(); G_stem.with(|c| *c.borrow_mut() = v); }
pub fn g_subject() -> Py { ensure_init(); G_subject.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_subject(v: Py) { ensure_init(); G_subject.with(|c| *c.borrow_mut() = v); }
pub fn g_the_animal_the_subject_is() -> Py { ensure_init(); G_the_animal_the_subject_is.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_animal_the_subject_is(v: Py) { ensure_init(); G_the_animal_the_subject_is.with(|c| *c.borrow_mut() = v); }
pub fn g_the_ask_about_the_thing_it_points_at() -> Py { ensure_init(); G_the_ask_about_the_thing_it_points_at.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_ask_about_the_thing_it_points_at(v: Py) { ensure_init(); G_the_ask_about_the_thing_it_points_at.with(|c| *c.borrow_mut() = v); }
pub fn g_the_step_back_takes_its_words_back() -> Py { ensure_init(); G_the_step_back_takes_its_words_back.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_step_back_takes_its_words_back(v: Py) { ensure_init(); G_the_step_back_takes_its_words_back.with(|c| *c.borrow_mut() = v); }
pub fn g_the_thing_a_detail_belongs_to() -> Py { ensure_init(); G_the_thing_a_detail_belongs_to.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_thing_a_detail_belongs_to(v: Py) { ensure_init(); G_the_thing_a_detail_belongs_to.with(|c| *c.borrow_mut() = v); }
pub fn g_the_thing_it_is() -> Py { ensure_init(); G_the_thing_it_is.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_thing_it_is(v: Py) { ensure_init(); G_the_thing_it_is.with(|c| *c.borrow_mut() = v); }
pub fn g_topic_text() -> Py { ensure_init(); G_topic_text.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_topic_text(v: Py) { ensure_init(); G_topic_text.with(|c| *c.borrow_mut() = v); }
pub fn g_trouble() -> Py { ensure_init(); G_trouble.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_trouble(v: Py) { ensure_init(); G_trouble.with(|c| *c.borrow_mut() = v); }
pub fn g_without_filler() -> Py { ensure_init(); G_without_filler.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_without_filler(v: Py) { ensure_init(); G_without_filler.with(|c| *c.borrow_mut() = v); }
pub fn g_written_out() -> Py { ensure_init(); G_written_out.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_written_out(v: Py) { ensure_init(); G_written_out.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The memory store: what the user and the bot have said. The conversation lane owns it.\n\nShared keys of the conversation's memory dict (list new ones in README.md):\n- memory[\"turns\"]: every earlier turn as {\"user\", \"bot\", \"module\"}; the runtime writes it.\n- memory[\"understood\"]: this turn's reading of the user's line (bot/understanding).\n- memory[\"user\"]: facts about the user, e.g. {\"name\": \"Sam\", \"city\": \"Leeds\", \"allergies\":\n  [\"nuts\"], \"pets\": [{\"animal\": \"dog\", \"name\": \"Biscuit\"}], \"budget\": \"6,000 pounds\",\n  \"diet\": \"vegetarian\" or \"no dairy\", \"condition\": [\"coeliac\"], \"age\": 34,\n  \"birthday\": \"3rd of may\", \"weight\": \"90 kilos\", \"plans\": [\"...\"]}.\n  Write it through remember().\n- memory[\"asked\"]: the fact the last recall question was about, {\"slot\", \"turn\"}, so an\n  elided ask (\"what did I say it was again?\", \"and what did I call him?\") is answered about\n  the same fact.\n- memory[\"facts_log\"]: every fact the user stated, in order: {\"key\", \"value\", \"turn\", \"text\"}.\n- memory[\"others\"]: facts about other people the user mentioned, [{\"who\": \"son\", \"allergies\":\n  [\"nuts\"]}], with who \"\" for \"he's\" when no noun is named. A fact belongs to the person it is\n  about, so \"my son is allergic to nuts\" goes here and never into memory[\"user\"].\n- memory[\"topic\"]: what the conversation is about now, {\"turn\", \"text\", \"words\"}; see\n  bot/memory/topics.py.\n- memory[\"night_out\"]: the place they said they went (\"we went to the pub last night\"),\n  {\"place\", \"turn\"}, so the next line can be answered about it (\"it was lovely\"). It only\n  counts for the turn right after the one that asked.\n- memory[\"topics\"]: every topic so far, oldest first.\n- memory[\"return_to\"], memory[\"returned\"]: set when the user goes back to an earlier topic (\"ok\n  back to the boiler\", \"anyway, the laptop\"); returned is {\"text\", \"asked\", \"asks\", \"turn\"} for\n  the turn it happened, so an answer module can greet the return or answer on that topic.\n\nHelpers every lane can use:\n- user(memory) -> the user's facts (a dict), created on first use\n- others(memory) -> facts about other people they mentioned (a list, created on first use)\n- remember(memory, key, value, text=\"\") -> keep a fact and log when it was said\n- current(memory), expand(text, memory), is_follow_up(text, memory), keywords(text)\n- resumed(memory) -> the earlier topic the user went back to on this turn, or None\n- return_target(text) -> the thing a return line names (\"the boiler\"), or \"\"\n- subject(memory) -> the noun phrase the conversation is about now (\"birthday cake\"), for\n  modules that answer a follow-up; gist(text) -> that phrase for one line\n- followup_reading(understood, memory): the reading with the current topic's subject put back in,\n  for a follow-up ('and if I stuff it?') or a correction ('sorry, I meant Florence')\n- the_thing_a_detail_belongs_to(understood, memory): an ask that names only a detail of the animal\n  the store holds, read with that animal's kind beside it ('how do I stop the chewing' after a\n  puppy is the puppy's chewing, not a pen at a desk); None when it lands on no entry\n- a_detail_of_the_thing_we_hold(understood, memory): the same reading without the entry, True when\n  the line names a detail of the animal and leaves the animal itself out\n- key_subject(text, memory), same_as_before(understood, memory), already_said(memory, text)\n- forms(word), content(text): a word with its verb endings on or off, and the words a line is\n  about, so \"saving\" finds \"save\" and a grammar word with an ending is still grammar\n- when_answer(text, memory): the date of the thing a \"when ...\" line asks about, from the line\n  that stated it (bot/memory/when.py); None when no earlier line dates it\n- remember_answer(memory, reply, keys), entry_answer(memory, topic_text): the whole answer an\n  entry gave, kept so a later follow-up can still reach a sentence a trimmed answer left out\n- recent(memory, n), last_user(memory), last_bot(memory),\n  answer_in_hand(memory): the bot turn that answered the subject the chat is on, read back over\n  an acknowledgement or two (\"ok thanks\" does not end what was being talked about, so the\n  follow-up after it is still about the answer before the thanks)\n- choice.reply_for(text, memory): when they hand the choice back (\"you pick\", \"your call\"),\n  one of the options the bot offered a turn ago, read out of its own last reply\n  (bot/memory/choice.py)\n- reject.reply_for(text, memory): the other side of that - they turn the offer down (\"nah not\n  that one\", \"ive seen it\"), and get another of the options the bot itself named\n  (bot/memory/reject.py)\n");
    s_VERB_FORMS(pyrt::xrefs::xget("understanding.kinds", "VERB_FORMS"));
    s_VERBISH(pyrt::xrefs::xget("understanding.kinds", "VERBISH"));
    s_TIME_WORDS(pyrt::xrefs::xget("understanding.normalize", "TIME_WORDS"));
    s_possessive_me(py_func("possessive_me", |args: &[Py]| pyrt::xrefs::xcall("understanding.normalize", "possessive_me", args)));
    s_without_filler(py_func("without_filler", |args: &[Py]| pyrt::xrefs::xcall("memory.filler", "without_filler", args)));
    s_COUNTRIES(pyrt::xrefs::xget("memory.facts", "COUNTRIES"));
    s_DETERMINERS(pyrt::xrefs::xget("memory.facts", "DETERMINERS"));
    s_NOT_PLACES(pyrt::xrefs::xget("memory.facts", "NOT_PLACES"));
    s_MONTHS(pyrt::xrefs::xget("memory.facts", "MONTHS"));
    s_PREPOSITIONS(pyrt::xrefs::xget("memory.facts", "PREPOSITIONS"));
    s_PERSON_TROUBLE(pyrt::xrefs::xget("memory.topics", "PERSON_TROUBLE"));
    s_trouble(py_func("trouble", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "trouble", args)));
    s_KIND_ASK(pyrt::xrefs::xget("memory.topics", "KIND_ASK"));
    s_WHO_SAID(pyrt::xrefs::xget("memory.topics", "WHO_SAID"));
    s_a_detail_of_the_thing_we_hold(py_func("a_detail_of_the_thing_we_hold", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "a_detail_of_the_thing_we_hold", args)));
    s_about(py_func("about", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "about", args)));
    s_action(py_func("action", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "action", args)));
    s_already_said(py_func("already_said", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "already_said", args)));
    s_answer_in_hand(py_func("answer_in_hand", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "answer_in_hand", args)));
    s_asks_only(py_func("asks_only", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "asks_only", args)));
    s_bare_question(py_func("bare_question", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "bare_question", args)));
    s_content(py_func("content", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "content", args)));
    s_context(py_func("context", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "context", args)));
    s_current(py_func("current", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "current", args)));
    s_entry_answer(py_func("entry_answer", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "entry_answer", args)));
    s_expand(py_func("expand", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "expand", args)));
    s_forms(py_func("forms", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "forms", args)));
    s_followup_reading(py_func("followup_reading", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "followup_reading", args)));
    s_gist(py_func("gist", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "gist", args)));
    s_has_time(py_func("has_time", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "has_time", args)));
    s_introduction_question(py_func("introduction_question", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "introduction_question", args)));
    s_is_follow_up(py_func("is_follow_up", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "is_follow_up", args)));
    s_key_subject(py_func("key_subject", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "key_subject", args)));
    s_keywords(py_func("keywords", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "keywords", args)));
    s_left_for_a_blank(py_func("left_for_a_blank", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "left_for_a_blank", args)));
    s_meal_follow_up(py_func("meal_follow_up", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "meal_follow_up", args)));
    s_note(py_func("note", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "note", args)));
    s_person(py_func("person", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "person", args)));
    s_person_named(py_func("person_named", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "person_named", args)));
    s_remember_answer(py_func("remember_answer", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "remember_answer", args)));
    s_resumed(py_func("resumed", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "resumed", args)));
    s_return_target(py_func("return_target", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "return_target", args)));
    s_said_again(py_func("said_again", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "said_again", args)));
    s_answer_about(py_func("answer_about", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "answer_about", args)));
    s_answers_about_another_creature(py_func("answers_about_another_creature", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "answers_about_another_creature", args)));
    s_same_as_before(py_func("same_as_before", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "same_as_before", args)));
    s_sized_reply(py_func("sized_reply", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "sized_reply", args)));
    s_stem(py_func("stem", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "stem", args)));
    s_subject(py_func("subject", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "subject", args)));
    s_the_animal_the_subject_is(py_func("the_animal_the_subject_is", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "the_animal_the_subject_is", args)));
    s_the_ask_about_the_thing_it_points_at(py_func("the_ask_about_the_thing_it_points_at", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "the_ask_about_the_thing_it_points_at", args)));
    s_the_thing_a_detail_belongs_to(py_func("the_thing_a_detail_belongs_to", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "the_thing_a_detail_belongs_to", args)));
    s_the_step_back_takes_its_words_back(py_func("the_step_back_takes_its_words_back", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "the_step_back_takes_its_words_back", args)));
    s_the_thing_it_is(py_func("the_thing_it_is", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "the_thing_it_is", args)));
    s_topic_text(py_func("topic_text", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "topic_text", args)));
    s_written_out(py_func("written_out", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "written_out", args)));
    s_LIST_FACTS(py_tuple(vec![py_str("places"), py_str("allergies"), py_str("people"), py_str("plans"), py_str("condition")]));
    s_PETS_KEPT(Py::Int(12));
    s_ANOTHER_ONE(call_builtin("frozenset", &[mcall(&py_str("other another second third fourth youngest eldest oldest first middle\nlast"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_HEAD(stdlib_call("re", "compile", &[py_str("^\\W*(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a few|several|lots of|\\d+)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:what|how|why|when|where|who|which|whose|should|shall|do|does|did|can|could|is|are|was|were|will|would|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_WORDS(call_builtin("frozenset", &[mcall(&py_str("january february march april may june july august september october\nnovember december jan feb mar apr jun jul aug sep sept oct nov dec\nmonday tuesday wednesday thursday friday saturday sunday weekend\nnext this coming tomorrow tonight morning afternoon evening week month year the of on in at\nfor and on"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_OPENS(call_builtin("frozenset", &[mcall(&py_str("i i'm im i've i'll ill we we're were we've weve we'll well my our me"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JOINED_SUBJECT(stdlib_call("re", "compile", &[py_str("^(?:[a-z'\\u2019]+\\s+){0,3}and\\s+(?:i|me)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_VERBS(call_builtin("frozenset", &[mcall(&py_str("planning going trying aiming hoping intending looking"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_APPOINTMENT_PLAN(stdlib_call("re", "compile", &[py_str("^(?:the\\s+)?(?P<thing>[a-z][a-z0-9' \\-]{1,28}?)\\s+(?P<when>at\\s+[\\w:.]+(?:\\s*(?:am|pm))?(?:\\s+on\\s+[a-z0-9 \\-]+)?|on\\s+[a-z0-9 \\-]+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_APPOINTMENT_DOING(call_builtin("frozenset", &[mcall(&py_str("\narrange bake book bring buy call clean collect cook deliver do drop email fetch finish fix get\ngo have hire make meet message order pay phone pick post put renew rent ring see send set sort\nstart submit take text tidy visit write\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_APPOINTMENT_A_DETERMINER(call_builtin("frozenset", &[mcall(&py_str("\na an the my your our their his her its this that these those some any\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_APPOINTMENT_A_PERSON_VERB(call_builtin("frozenset", &[mcall(&py_str("\nmeet call ring text phone message email see visit collect fetch\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_APPOINTMENT_A_QUANTITY(call_builtin("frozenset", &[mcall(&py_str("\nbread money rent cash milk water petrol diesel fuel shopping washing ironing laundry food\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_APPOINTMENT_A_PLURAL(stdlib_call("re", "compile", &[py_str("^[a-z]{3,}(?:s|es)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_APPOINTMENT_NOT_A_PLURAL(py_tuple(vec![py_str("ss"), py_str("us"), py_str("is"), py_str("ous")]));
    s__WEEKDAY_NAMES(call_builtin("frozenset", &[py_tuple(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("christmas"), py_str("easter")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("in"), py_str("of"), py_str("by"), py_str("from"), py_str("until"), py_str("till"), py_str("til"), py_str("early"), py_str("late"), py_str("mid"), py_str("towards"), py_str("before"), py_str("after")]) }
    s__DATE_LEADS(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DAY_NUMBER(stdlib_call("re", "compile", &[py_str("^\\d{1,2}(?:st|nd|rd|th)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHOSE_IT_IS(call_builtin("frozenset", &[py_tuple(vec![py_str("his"), py_str("her"), py_str("hers"), py_str("their"), py_str("theirs"), py_str("your"), py_str("yours")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_DOES_BARE(call_builtin("frozenset", &[mcall(&py_str("go come start end begin finish close open reopen run happen\narrive leave return fly land depart move switch change rise set turn shut take last break\nbreak up go back come round"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_DOES_S(call_builtin("frozenset", &[mcall(&py_str("is are was were due goes comes starts ends begins finishes closes\nopens reopens runs happens arrives leaves returns flies lands departs moves switches changes\nrises sets turns shuts takes lasts breaks"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JOURNEY_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|ive|i've|we|were|we're|weve|we've|they|hes|he's|shes|she's)\\s+(?:am\\s+|are\\s+|is\\s+)?(?:going|off|flying|fly|moving|driving|heading|travell?ing|cycling|walking|jetting|sailing|back)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_TO_A_PERSON(call_builtin("frozenset", &[mcall(&py_str("\ntold tell telling tellin says said asks asked ask asking calls called call calling rang ring\nringing texted texting texts text emailed emailing emails email phoned phoning phones phone\nmessaged messaging message thanked thanking thanks thank blamed blaming blame ignored ignoring\nignore hurt hurting hurts missed missing miss loved loving love needed needing need trusted\ntrusting trust woke woken woke saw seeing seen heard hearing hear helped helping help sent\nsending send showed showing show shown gave giving give given paid paying pay owed owing owe met\nmeeting meet noticed noticing notice picked picking pick chose chosen choose chose kissed\nkissing kiss hugged hugging hug\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_TO_A_PERSON(call_builtin("frozenset", &[mcall(&py_str("\nto at with for about on from of around near towards toward into over behind beside against\nacross\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("i"), py_str("i'm"), py_str("i've"), py_str("i'll"), py_str("i'd"), py_str("im"), py_str("ive"), py_str("ill"), py_str("we"), py_str("we're"), py_str("we've"), py_str("we'll"), py_str("we'd"), py_str("were"), py_str("me"), py_str("my"), py_str("mine"), py_str("us"), py_str("our"), py_str("ours"), py_str("you"), py_str("your"), py_str("yours"), py_str("yourself"), py_str("he"), py_str("she"), py_str("it"), py_str("they"), py_str("him"), py_str("her"), py_str("them"), py_str("his"), py_str("their"), py_str("theirs")]) }
    s_SUBJECT_HEADS(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("i"), py_str("i'm"), py_str("i've"), py_str("i'll"), py_str("i'd"), py_str("im"), py_str("ive"), py_str("ill"), py_str("we"), py_str("we're"), py_str("we've"), py_str("we'll"), py_str("we'd"), py_str("were")]) }
    s_FIRST_PERSON_HEADS(call_builtin("frozenset", &[__data3()]).unwrap_or_else(|e| exc::raise(e)));
    let __unpk4 = py_unpack(&py_tuple(vec![py_str("\0m"), py_str("\0M")]), 2, None);
    s__BOT_ME(__unpk4[0].clone());
    s__BOT_MY(__unpk4[1].clone());
    fn __data5() -> Py { py_dict(vec![(py_str("im"), py_str("I'm")), (py_str("ive"), py_str("I've")), (py_str("dont"), py_str("don't")), (py_str("cant"), py_str("can't")), (py_str("doesnt"), py_str("doesn't")), (py_str("didnt"), py_str("didn't")), (py_str("isnt"), py_str("isn't")), (py_str("wasnt"), py_str("wasn't")), (py_str("arent"), py_str("aren't")), (py_str("werent"), py_str("weren't")), (py_str("havent"), py_str("haven't")), (py_str("hasnt"), py_str("hasn't")), (py_str("hadnt"), py_str("hadn't")), (py_str("wouldnt"), py_str("wouldn't")), (py_str("couldnt"), py_str("couldn't")), (py_str("shouldnt"), py_str("shouldn't")), (py_str("mustnt"), py_str("mustn't")), (py_str("neednt"), py_str("needn't")), (py_str("aint"), py_str("ain't")), (py_str("shes"), py_str("she's")), (py_str("hes"), py_str("he's")), (py_str("thats"), py_str("that's")), (py_str("whats"), py_str("what's")), (py_str("theres"), py_str("there's")), (py_str("heres"), py_str("here's")), (py_str("theyre"), py_str("they're")), (py_str("youre"), py_str("you're")), (py_str("youve"), py_str("you've")), (py_str("youll"), py_str("you'll")), (py_str("youd"), py_str("you'd"))]) }
    s_APOSTROPHE(__data5());
    s_APOSTROPHE_WORD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b("), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_APOSTROPHE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUOTED(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\"[^\"]*\"|\\u201c[^\\u201d]*\\u201d|(?<!\\w)"), &py_str("'")), &py_str("[^']*")), &py_str("'")), &py_str("(?!\\w)"))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_OWNED(op_bitor(&op_bitor(&call_builtin("frozenset", &[mcall(&py_str("are were is was am and or but nor too either"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &pyrt::xrefs::xget("understanding.kinds", "VERB_FORMS")), &pyrt::xrefs::xget("understanding.kinds", "VERBISH")));
    s_WITH_SOMEBODY(op_bitor(&call_builtin("frozenset", &[mcall(&py_str("see saw seen seeing visit visited visiting meet met meeting missed\n    missing miss call called calling ring rang ringing text texted texting phone phoned phoning\n    email emailed emailing thank thanked thanking hug hugged hugging kiss kissed kissing help helped\n    helping bother bothered bothering visit stay stayed staying"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("with")])));
    s_WHEN_OR_WHERE(op_bitor(&op_bitor(&op_bitor(&pyrt::xrefs::xget("understanding.normalize", "TIME_WORDS"), &pyrt::xrefs::xget("memory.facts", "PREPOSITIONS")), &pyrt::xrefs::xget("memory.facts", "DETERMINERS")), &call_builtin("frozenset", &[mcall(&py_str("next last later soon\n    earlier afterwards afterward today tomorrow tonight week weekend morning evening night day\n    month year couple few"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("how what when where who why which whose should shall can could do does did\nis are was were will would am"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_TAIL(stdlib_call("re", "compile", &[op_add(&op_add(&py_str(",\\s*(?:and\\s+)?(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_ASK_WORDS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WERE_DOING(stdlib_call("re", "compile", &[py_str("we[r']?e?\\s+\\w+ing\\b|were\\s+\\w+ing\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_list(vec![py_str("action"), py_str("already_said"), py_str("asks_only"), py_str("context"), py_str("you"), py_str("current"), py_str("entry_answer"), py_str("expand"), py_str("facts"), py_str("followup_reading"), py_str("gist"), py_str("has_time"), py_str("introduction_question"), py_str("is_follow_up"), py_str("answer_in_hand"), py_str("key_subject"), py_str("keywords"), py_str("last_bot"), py_str("last_module"), py_str("last_user"), py_str("left_for_a_blank"), py_str("log"), py_str("meal_follow_up"), py_str("name"), py_str("note"), py_str("plan_line"), py_str("plan_sentence"), py_str("recent"), py_str("remember"), py_str("remember_answer"), py_str("resumed"), py_str("return_target"), py_str("said"), py_str("said_again"), py_str("same_as_before"), py_str("answer_about"), py_str("answers_about_another_creature"), py_str("the_animal_the_subject_is"), py_str("sized_reply"), py_str("stem"), py_str("subject"), py_str("the_ask_about_the_thing_it_points_at"), py_str("topic_text"), py_str("the_step_back_takes_its_words_back"), py_str("topics"), py_str("user"), py_str("bare_question"), py_str("written_out")]) }
    s___all__(__data6());
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
    facts_ = mcall(&memory.clone(), "get", &[py_str("user")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[facts_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
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
    entries = mcall(&memory.clone(), "get", &[py_str("facts_log")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entries.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
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
    seen = mcall(&memory.clone(), "get", &[py_str("others")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[seen.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
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
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&words.clone())) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    one = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&one.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&one.clone(), "rstrip", &[py_str("s")]), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&index.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)) { let mut earlier: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ANOTHER_ONE(), &earlier.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
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
    if py_truthy(&{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { mcall(&known.clone(), "get", &[py_str("name")]) } }) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&entry.clone(), "get", &[py_str("age")]), mcall(&known.clone(), "get", &[py_str("age")])]), 2, None);
    age = __unpk1[0].clone();
    held = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[age.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&age.clone(), &held.clone())) } } } }) {
    return Py::Bool(false);
    }
    return _said_another(&[text.clone(), { let __b = mcall(&known.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("who")]) } }]);
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[mcall(&entry.clone(), "values", &[])]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    who = { let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } };
    named = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { named.clone() } }) {
    same = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&others(&[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &named.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[same.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    return _fold_into(&[memory.clone(), geti(&same.clone(), &Py::Int(0)), entry.clone(), text.clone()]);
    }
    let __iter1 = py_iter_or_raise(&others(&[memory.clone()]));
    for __item in __iter1 {
    known = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&known.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&call_builtin("str", &[geti(&entry.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&call_builtin("str", &[geti(&known.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } } } }) {
    continue;
    }
    if py_truthy(&_another_of_them(&[entry.clone(), known.clone(), text.clone()])) {
    continue;
    }
    return _fold_into(&[memory.clone(), known.clone(), entry.clone(), text.clone()]);
    }
    kept = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&entry.clone(), "items", &[])) { let __unpk3175 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk3175[0].clone(); let mut value: Py = __unpk3175[1].clone(); __out.push((key.clone(), if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("list", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { value.clone() })); } py_dict(__out) })() };
    let _ = mcall(&others(&[memory.clone()]), "append", &[kept.clone()]);
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), kept.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
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
    let __iter1 = py_iter_or_raise(&mcall(&entry.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("who"), py_str("pronoun")]), &key.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("ages")))) {
    let _ = py_setitem(&known.clone(), &py_str("ages"), call_builtin("list", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&known.clone(), "pop", &[py_str("age"), Py::None]);
    let _ = mcall(&known.clone(), "pop", &[py_str("to_come"), Py::None]);
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { mcall(&known.clone(), "get", &[py_str("ages")]) } }) {
    let _ = mcall(&known.clone(), "pop", &[py_str("ages"), Py::None]);
    let _ = py_setitem(&known.clone(), &py_str("age"), value.clone());
    let _ = mcall(&known.clone(), "pop", &[py_str("months"), Py::None]);
    let _ = mcall(&known.clone(), "pop", &[py_str("to_come"), Py::None]);
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[value.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&known.clone(), "get", &[py_str("age")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    ahead = { let __b = mcall(&entry.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } };
    knew = { let __b = mcall(&known.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = ahead.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&knew.clone())) } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![value.clone(), ahead.clone()]), 2, None);
    let _ = py_setitem(&known.clone(), &py_str("to_come"), __unpk3[0].clone());
    let _ = py_setitem(&known.clone(), &py_str("turns"), __unpk3[1].clone());
    continue;
    }
    if py_truthy(&{ let __b = knew.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&ahead.clone())) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[mcall(&known.clone(), "get", &[py_str("to_come")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&known.clone(), "get", &[py_str("age")]), &value.clone())) } }) {
    let _ = py_setitem(&known.clone(), &py_str("to_come"), geti(&known.clone(), &py_str("age")));
    }
    let _ = py_setitem(&known.clone(), &py_str("age"), value.clone());
    if py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&known.clone(), "get", &[py_str("to_come")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_le(&geti(&known.clone(), &py_str("to_come")), &value.clone()) } }) {
    let _ = mcall(&known.clone(), "pop", &[py_str("to_come"), Py::None]);
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
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
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))))) {
    let _ = py_setitem(&known.clone(), &key.clone(), value.clone());
    continue;
    }
    have = mcall(&known.clone(), "setdefault", &[key.clone(), py_list(vec![])]);
    if py_truthy(&call_builtin("isinstance", &[have.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&have.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&have.clone(), &item.clone()))) {  __out.push(item.clone()); } } py_list(__out) })() }]);
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
    value = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&value.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_COUNT_HEAD(), "match", &[value.clone()])) {
    return Py::Bool(false);
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_QUESTION_OPEN(), "match", &[said.clone()]) } } } }) {
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_WHEN_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { mcall(&py_str(" "), "join", &[words.clone()]) } else { py_str("") };
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
    known = user(&[memory.clone()]);
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    value = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut one: Py = __item.clone(); __out.push(if py_truthy(&call_builtin("isinstance", &[one.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { pyrt::xrefs::xcall("memory.filler", "without_filler", &[one.clone()]) } else { one.clone() }); } py_list(__out) })() };
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("plan"), py_str("plans")]), &key.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    value = pyrt::xrefs::xcall("memory.filler", "without_filler", &[value.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plans")))) {
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { py_list(vec![value.clone()]) });
    for __item in __iter1 {
    one = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_plan(&[one.clone(), text.clone()])))) {
    continue;
    }
    plans = mcall(&known.clone(), "get", &[py_str("plans")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[plans.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    plans = py_list(vec![]);
    }
    if py_truthy(&Py::Bool(py_contains(&plans.clone(), &one.clone()))) {
    continue;
    }
    thing = _the_thing_the_plan_is(&[one.clone()]);
    if py_truthy(&thing.clone()) {
    plans = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut kept: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&_the_thing_the_plan_is(&[kept.clone()]), &thing.clone()))) {  __out.push(kept.clone()); } } py_list(__out) })() };
    }
    let _ = mcall(&plans.clone(), "append", &[one.clone()]);
    let _ = py_setitem(&known.clone(), &py_str("plans"), slice_or_raise(&plans.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    }
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), value.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("pets")))) {
    pets = mcall(&known.clone(), "get", &[py_str("pets")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pets.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    pets = py_list(vec![]);
    }
    value = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("not_theirs")]) } }))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    return Py::None;
    }
    one_animal = Py::Bool(py_eq(&call_builtin("len", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    let __iter2 = py_iter_or_raise(&value.clone());
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    already_named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&p.clone(), "get", &[py_str("animal")]), &mcall(&pet.clone(), "get", &[py_str("animal")]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&p.clone(), "get", &[py_str("name")]), &mcall(&pet.clone(), "get", &[py_str("name")]))) } }) {  __out.push(p.clone()); } } py_list(__out) })() };
    rename = { let __b = one_animal.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("another")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&already_named.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_le(&{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&p.clone(), "get", &[py_str("animal")]), &mcall(&pet.clone(), "get", &[py_str("animal")]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&p.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&p.clone(), "get", &[py_str("name")]), &mcall(&pet.clone(), "get", &[py_str("name")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_gt(&{ let __b = mcall(&p.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)))) } } } } } }) {  __out.push(p.clone()); } } py_list(__out) })() } } } } } } } } } } };
    if py_truthy(&{ let __b = rename.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[rename.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    renamed = geti(&rename.clone(), &Py::Int(0));
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    let _ = py_setitem(&renamed.clone(), &py_str("name"), geti(&pet.clone(), &py_str("name")));
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("age")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&pet.clone(), "get", &[py_str("age")]), &mcall(&renamed.clone(), "get", &[py_str("age")]))) } }) {
    let _ = py_setitem(&renamed.clone(), &py_str("age"), geti(&pet.clone(), &py_str("age")));
    let _ = mcall(&renamed.clone(), "pop", &[py_str("unit"), Py::None]);
    if py_truthy(&mcall(&pet.clone(), "get", &[py_str("unit")])) {
    let _ = py_setitem(&renamed.clone(), &py_str("unit"), geti(&pet.clone(), &py_str("unit")));
    }
    }
    continue;
    }
    counted = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&p.clone(), "get", &[py_str("animal")]), &mcall(&pet.clone(), "get", &[py_str("animal")]))); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let __b = mcall(&p.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&p.clone(), "get", &[py_str("name")]))) } } } }) {  __out.push(p.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    same = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&p.clone(), "get", &[py_str("animal")]), &mcall(&pet.clone(), "get", &[py_str("animal")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&p.clone(), "get", &[py_str("name")]), &mcall(&pet.clone(), "get", &[py_str("name")]))); if py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("another")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&p.clone(), "get", &[py_str("name")]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("name")]))) } } } }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&counted.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&p.clone(), &counted.clone())) } } } } } } } }) {  __out.push(p.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_is(&same.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("another")]))); if !py_truthy(&__b) { __b } else { op_le(&{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(1)) } } } }) {
    kind = mcall(&pyrt::xrefs::xget("memory.facts", "ANIMAL_KINDS"), "get", &[mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), py_str("")]);
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("name")]))) } }) {
    same = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut p: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&p.clone(), "get", &[py_str("coming")]); if !py_truthy(&__b) { __b } else { { let __b = kind.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&pyrt::xrefs::xget("memory.facts", "ANIMAL_KINDS"), "get", &[mcall(&call_builtin("str", &[{ let __b = mcall(&p.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), py_str("")]), &kind.clone())) } } } }) {  __out.push(p.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    if py_truthy(&Py::Bool(!py_is(&same.clone(), &Py::None))) {
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&same.clone(), "get", &[py_str("name")]))) } }) {
    let _ = py_setitem(&same.clone(), &py_str("name"), geti(&pet.clone(), &py_str("name")));
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&same.clone(), "get", &[py_str("breed")]))) } }) {
    let _ = py_setitem(&same.clone(), &py_str("breed"), geti(&pet.clone(), &py_str("breed")));
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    } else {
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&pet.clone(), "get", &[py_str("count")]), &mcall(&same.clone(), "get", &[py_str("count")]))) } }) {
    let _ = py_setitem(&same.clone(), &py_str("count"), geti(&pet.clone(), &py_str("count")));
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    }
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("age")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&pet.clone(), "get", &[py_str("age")]), &mcall(&same.clone(), "get", &[py_str("age")]))) } }) {
    let _ = py_setitem(&same.clone(), &py_str("age"), geti(&pet.clone(), &py_str("age")));
    let _ = mcall(&same.clone(), "pop", &[py_str("unit"), Py::None]);
    if py_truthy(&mcall(&pet.clone(), "get", &[py_str("unit")])) {
    let _ = py_setitem(&same.clone(), &py_str("unit"), geti(&pet.clone(), &py_str("unit")));
    }
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&pet.clone(), "get", &[py_str("breed")]), &mcall(&same.clone(), "get", &[py_str("breed")]))) } }) {
    let _ = py_setitem(&same.clone(), &py_str("breed"), geti(&pet.clone(), &py_str("breed")));
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    continue;
    }
    let _ = mcall(&pets.clone(), "append", &[pet.clone()]);
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), pet.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    let _ = py_setitem(&known.clone(), &py_str("pets"), slice_or_raise(&pets.clone(), &op_neg(&call_builtin("max", &[g_PETS_KEPT(), call_builtin("len", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None));
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("tastes")))) {
    tastes = mcall(&known.clone(), "get", &[py_str("tastes")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[tastes.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    tastes = py_list(vec![]);
    }
    let __iter3 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { py_list(vec![value.clone()]) });
    for __item in __iter3 {
    taste = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[taste.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&taste.clone(), "get", &[py_str("thing")]))) } }) {
    continue;
    }
    thing = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&taste.clone(), "get", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    same = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tastes.clone()) { let mut t: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &thing.clone()))) {  __out.push(t.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_is(&same.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_eq(&mcall(&same.clone(), "get", &[py_str("how")]), &mcall(&taste.clone(), "get", &[py_str("how")])))) {
    let _ = py_setitem(&same.clone(), &py_str("how"), mcall(&taste.clone(), "get", &[py_str("how")]));
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("taste")), (py_str("value"), taste.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    continue;
    }
    let _ = mcall(&tastes.clone(), "append", &[taste.clone()]);
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("taste")), (py_str("value"), taste.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    let _ = py_setitem(&known.clone(), &py_str("tastes"), slice_or_raise(&tastes.clone(), &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("owns")))) {
    held = mcall(&known.clone(), "get", &[py_str("owns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    held = py_dict(vec![]);
    }
    let __iter4 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "items", &[]) } else { py_list(vec![]) });
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    thing = __unpk5[0].clone();
    name = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&name.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&held.clone(), "get", &[thing.clone()]), &name.clone()))) {
    continue;
    }
    let _ = py_setitem(&held.clone(), &thing.clone(), name.clone());
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("owns")), (py_str("value"), py_dict(vec![(thing.clone(), name.clone())])), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    let _ = py_setitem(&known.clone(), &py_str("owns"), held.clone());
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_LIST_FACTS(), &key.clone()))) {
    seen = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[seen.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    seen = py_list(vec![]);
    }
    let __iter6 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { py_list(vec![value.clone()]) });
    for __item in __iter6 {
    item = __item.clone();
    if py_truthy(&{ let __b = item.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&seen.clone(), &item.clone())) } }) {
    let _ = mcall(&seen.clone(), "append", &[item.clone()]);
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), item.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    }
    }
    let _ = py_setitem(&known.clone(), &key.clone(), slice_or_raise(&seen.clone(), &op_neg(&Py::Int(8)), &Py::None, &Py::None));
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&known.clone(), "get", &[key.clone()]), &value.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&known.clone(), &key.clone(), value.clone());
    let _ = mcall(&log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), value.clone()), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
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
    known = user(&[memory.clone()]);
    wanted = if py_truthy(&call_builtin("isinstance", &[values.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { values.clone() } else { py_list(vec![values.clone()]) };
    doomed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut value: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&mcall(&call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])); } } py_set(__out) })() };
    had = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("pets")))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[had.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&doomed.clone())) } }) {
    return py_list(vec![]);
    }
    everything = Py::Bool(py_contains(&doomed.clone(), &py_str("*")));
    records = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut value: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(value.clone()); } } py_list(__out) })() };
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    kept = __unpk1[0].clone();
    gone = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&had.clone());
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    worded = { let __b = Py::Bool(py_contains(&doomed.clone(), &mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&doomed.clone(), &mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]))) } };
    named = Py::Bool(py_contains(&records.clone(), &pet.clone()));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = everything.clone(); if py_truthy(&__b) { __b } else { { let __b = worded.clone(); if py_truthy(&__b) { __b } else { named.clone() } } } }))) {
    let _ = mcall(&kept.clone(), "append", &[pet.clone()]);
    continue;
    }
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&everything.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&worded.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(1) } }, &Py::Int(1)) } } } } } }) {
    let _ = py_setitem(&pet.clone(), &py_str("count"), op_sub(&call_builtin("int", &[geti(&pet.clone(), &py_str("count"))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    if py_truthy(&op_le(&geti(&pet.clone(), &py_str("count")), &Py::Int(1))) {
    let _ = mcall(&pet.clone(), "pop", &[py_str("count"), Py::None]);
    }
    let _ = mcall(&kept.clone(), "append", &[pet.clone()]);
    continue;
    }
    let _ = mcall(&gone.clone(), "append", &[pet.clone()]);
    }
    let _ = py_setitem(&known.clone(), &key.clone(), kept.clone());
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    let _ = mcall(&known.clone(), "pop", &[key.clone(), Py::None]);
    }
    if py_truthy(&gone.clone()) {
    past = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("pets_gone")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    let __iter3 = py_iter_or_raise(&gone.clone());
    for __item in __iter3 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&past.clone(), &pet.clone()))) {
    let _ = mcall(&past.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pet.clone()); __kws.extend(vec![("gone".to_string(), { let __b = why.clone(); if py_truthy(&__b) { __b } else { py_str("gone") } })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    }
    }
    let _ = py_setitem(&known.clone(), &py_str("pets_gone"), past.clone());
    }
    return gone.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_LIST_FACTS(), &key.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[had.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&doomed.clone())) } }) {
    return py_list(vec![]);
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&had.clone()) { let mut value: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&doomed.clone(), &mcall(&mcall(&call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])))) {  __out.push(value.clone()); } } py_list(__out) })() };
    let _ = py_setitem(&known.clone(), &key.clone(), left.clone());
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&had.clone()) { let mut value: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&doomed.clone(), &mcall(&mcall(&call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])))) {  __out.push(value.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&had.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&doomed.clone())) } }) {
    return py_list(vec![]);
    }
    if py_truthy(&Py::Bool(py_contains(&doomed.clone(), &mcall(&mcall(&call_builtin("str", &[had.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])))) {
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
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
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
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
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
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&recent(&[memory.clone(), n.clone()])) { let mut turn: Py = __item.clone(); __out.push(mcall(&turn.clone(), "get", &[py_str("user"), py_str("")])); } py_list(__out) })() };
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
    turns = recent(&[memory.clone(), Py::Int(1)]);
    return if py_truthy(&turns.clone()) { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot"), py_str("")]) } else { py_str("") };
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
    turns = recent(&[memory.clone(), Py::Int(1)]);
    return if py_truthy(&turns.clone()) { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("module")]) } else { Py::None };
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
    facts_ = user(&[memory.clone()]);
    value = mcall(&facts_.clone(), "get", &[py_str("name")]);
    return if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { py_str("") };
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
    return call_builtin("bool", &[{ let __b = mcall(&g_APPOINTMENT_A_PLURAL(), "match", &[word.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[g_APPOINTMENT_NOT_A_PLURAL()]))) } }]).unwrap_or_else(|e| exc::raise(e));
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&thing.clone(), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(",.;:")])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&geti(&words.clone(), &Py::Int(0)), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[geti(&words.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_APPOINTMENT_A_PERSON_VERB(), &geti(&words.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_APPOINTMENT_A_DETERMINER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_APPOINTMENT_DOING(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { _takes_a_thing(&[geti(&words.clone(), &Py::Int(1))]) } } } };
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
    words = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    bare = mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(",.;:!?")]);
    before = if py_truthy(&index.clone()) { mcall(&mcall(&geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))), "lower", &[]), "strip", &[py_str(",.;:!?")]) } else { py_str("") };
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { mcall(&mcall(&geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))), "lower", &[]), "strip", &[py_str(",.;:!?")]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__WEEKDAY_NAMES(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "MONTHS"), &bare.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__DATE_LEADS(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_DAY_NUMBER(), "match", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_DAY_NUMBER(), "match", &[after.clone()]) } } } } } } } }) {
    word = mcall(&word.clone(), "replace", &[bare.clone(), op_add(&mcall(&slice_or_raise(&bare.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&bare.clone(), &Py::Int(1), &Py::None, &Py::None)), Py::Int(1)]);
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
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
    words = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&geti(&words.clone(), &Py::Int(0)), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",.!?")])));
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
    word = mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("ss"), py_str("us"), py_str("is")])]))) } };
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
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(",.;:!?")])); } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_A_THING_DOES_S(), &word.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = _a_plural(&[geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1)))]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_THING_DOES_BARE(), &word.clone())) } }) {
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
    found = mcall(&g_APPOINTMENT_PLAN(), "match", &[mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .!?,")])]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    thing = _the_possessive_a_relation_means(&[mcall(&mcall(&found.clone(), "group", &[py_str("thing")]), "strip", &[])]);
    when = mcall(&mcall(&found.clone(), "group", &[py_str("when")]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("we"), py_str("were"), py_str("we're"), py_str("my"), py_str("our"), py_str("to"), py_str("it"), py_str("its"), py_str("he"), py_str("she"), py_str("they"), py_str("you")]), &mcall(&geti(&mcall(&thing.clone(), "split", &[]), &Py::Int(0)), "lower", &[]))) } }) {
    return py_str("");
    }
    when = named_days(&[when.clone()]);
    if py_truthy(&_a_doing(&[thing.clone()])) {
    said = op_add(&op_add(&op_add(&op_add(&py_str("You're planning to "), &mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" ")), &when.clone());
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &mcall(&geti(&mcall(&thing.clone(), "split", &[]), &Py::Int(0)), "lower", &[])))) {
    said = op_add(&op_add(&op_add(&op_add(&py_str("You've got "), &mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" ")), &when.clone());
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(py_contains(&g_WHOSE_IT_IS(), &mcall(&geti(&mcall(&thing.clone(), "split", &[]), &Py::Int(0)), "lower", &[])))) {
    said = op_add(&op_add(&op_add(&thing.clone(), &if py_truthy(&_has_its_own_verb(&[thing.clone()])) { py_str("") } else { py_str(" is") }), &py_str(" ")), &when.clone());
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    said = op_add(&op_add(&op_add(&op_add(&py_str("Your "), &thing.clone()), &if py_truthy(&_has_its_own_verb(&[thing.clone()])) { py_str("") } else { py_str(" is") }), &py_str(" ")), &when.clone());
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
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
    return call_builtin("bool", &[mcall(&g_JOURNEY_LINE(), "search", &[call_builtin("str", &[{ let __b = said_line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
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
    return named_days(&[_the_plan_said_back(&[plan.clone(), said_line.clone()])]);
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
    text = mcall(&pyrt::xrefs::xcall("memory.filler", "without_filler", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]), "strip", &[py_str(" .!?,")]);
    text = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and\\s+)?(?:also|oh|plus|by the way|btw|anyway|anyways|so|well|right|ok|okay|hey|and)\\b[\\s,]*")); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", &[py_str(" .!?,")]);
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^to\\s+")); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_str("");
    }
    said_sentence = mcall(&pyrt::xrefs::xcall("memory.facts", "no_tone", &[pyrt::xrefs::xcall("memory.filler", "without_filler", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said_line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])]), "strip", &[py_str(" .!?,")]);
    theirs = { let __b = call_builtin("bool", &[said_sentence.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLAN_OPENS(), &mcall(&geti(&mcall(&said_sentence.clone(), "split", &[]), &Py::Int(0)), "lower", &[]))); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_JOINED_SUBJECT(), "match", &[said_sentence.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
    appointment = if py_truthy(&{ let __b = _a_journey_line(&[said_line.clone()]); if py_truthy(&__b) { __b } else { _a_name_at_the_front(&[text.clone()]) } }) { py_str("") } else { appointment_sentence(&[text.clone()]) };
    if py_truthy(&appointment.clone()) {
    if py_truthy(&{ let __b = theirs.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&said_sentence.clone(), "lower", &[]), &mcall(&text.clone(), "lower", &[]))) } }) {
    out = mcall(&_mid(&[you(&[said_sentence.clone()])]), "rstrip", &[py_str(" .")]);
    return op_add(&mcall(&slice_or_raise(&out.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&out.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return appointment.clone();
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^to\\s+")); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^plans?\\s+to\\s+")); __args.push(py_str("planning to ")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    first = mcall(&mcall(&geti(&mcall(&text.clone(), "split", &[]), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",.;:")]);
    if py_truthy(&Py::Bool(py_contains(&g_PLAN_VERBS(), &first.clone()))) {
    phrase = _mid(&[you(&[text.clone()])]);
    return mcall(&op_add(&op_add(&py_str("You're "), &mcall(&slice_or_raise(&phrase.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&phrase.clone(), &Py::Int(1), &Py::None, &Py::None)), "rstrip", &[py_str(" .")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLAN_OPENS(), &first.clone())); if py_truthy(&__b) { __b } else { mcall(&g_JOINED_SUBJECT(), "match", &[text.clone()]) } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:im|ive|ill)\\b")); __args.push({ let appointment = appointment.clone(); let first = first.clone(); let out = out.clone(); let phrase = phrase.clone(); let plan = plan.clone(); let said = said.clone(); let said_line = said_line.clone(); let said_sentence = said_sentence.clone(); let text = text.clone(); let theirs = theirs.clone(); py_func("lambda", move |args: &[Py]| { let appointment = appointment.clone(); let first = first.clone(); let out = out.clone(); let phrase = phrase.clone(); let plan = plan.clone(); let said = said.clone(); let said_line = said_line.clone(); let said_sentence = said_sentence.clone(); let text = text.clone(); let theirs = theirs.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); op_add(&py_str("i'"), &slice_or_raise(&mcall(&m.clone(), "group", &[Py::Int(0)]), &Py::Int(1), &Py::None, &Py::None)) }) }); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwere\\b")); __args.push(py_str("we're")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bweve\\b")); __args.push(py_str("we've")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    out = mcall(&_mid(&[you(&[said.clone()])]), "rstrip", &[py_str(" .")]);
    return op_add(&mcall(&slice_or_raise(&out.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&out.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = said_sentence.clone(); if !py_truthy(&__b) { __b } else { theirs.clone() } }) {
    return plan_sentence(&[said_sentence.clone()]);
    }
    phrase = you(&[text.clone()]);
    phrase = op_add(&mcall(&slice_or_raise(&phrase.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&phrase.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&{ let __b = mcall(&first.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return mcall(&op_add(&py_str("You're "), &phrase.clone()), "rstrip", &[py_str(" .")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "DETERMINERS"), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "COUNTRIES"), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "NOT_PLACES"), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&_places(&[]), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&_not_places(&[]), &first.clone())) } } } } } } } }) {
    return mcall(&op_add(&py_str("You're planning "), &phrase.clone()), "rstrip", &[py_str(" .")]);
    }
    if py_truthy(&mcall(&mcall(&phrase.clone(), "lower", &[]), "startswith", &[py_str("to ")])) {
    return mcall(&op_add(&py_str("You're planning "), &phrase.clone()), "rstrip", &[py_str(" .")]);
    }
    return mcall(&op_add(&py_str("You're planning to "), &phrase.clone()), "rstrip", &[py_str(" .")]);
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
    return stdlib_call("re", "sub", &[py_str("(?<=[\\s,])You\\b"), py_str("you"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
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
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&text.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    head = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_SUBJECT_HEADS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
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
    word = mcall(&match_.clone(), "group", &[Py::Int(2)]);
    if py_truthy(&Py::Bool(!py_contains(&op_bitor(&g_SAYS_TO_A_PERSON(), &g_DONE_TO_A_PERSON()), &mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[])))) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("thank"), py_str("thanks")]), &mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[])))) {
    after = stdlib_call("re", "match", &[py_str("[\\s,]*([a-z']+)"), slice_or_raise(&gattr(&match_.clone(), "string"), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("for"), py_str("again"), py_str("so"), py_str("too")]), &mcall(&mcall(&after.clone(), "group", &[Py::Int(1)]), "lower", &[]))) } }) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    }
    new = if py_truthy(&Py::Bool(py_eq(&mcall(&word.clone(), "lower", &[]), &py_str("your")))) { py_str("my") } else { py_str("me") };
    marker = if py_truthy(&Py::Bool(py_eq(&new.clone(), &py_str("my")))) { g__BOT_MY() } else { g__BOT_ME() };
    return op_add(&op_add(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &py_str(" ")), &if py_truthy(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) { mcall(&marker.clone(), "upper", &[]) } else { marker.clone() });
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
    return mcall(&mcall(&text.clone(), "replace", &[g__BOT_ME(), py_str("me")]), "replace", &[g__BOT_MY(), py_str("my")]);
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
    let mut PLACES: Py = Py::None;
    let __try1 = exc::catch(|| {
    PLACES = pyrt::xrefs::xget("understanding.topics", "PLACES");
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
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
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
    let mut NOT_PLACES: Py = Py::None;
    let __try1 = exc::catch(|| {
    NOT_PLACES = pyrt::xrefs::xget("understanding.topics", "NOT_PLACES");
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
    return call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_QUOTED(), "search", &[text.clone()]))) } }) {
    return py_call(&repair, &[text.clone()]);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    out = __unpk1[0].clone();
    last = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_QUOTED(), "finditer", &[text.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    let _ = mcall(&out.clone(), "append", &[py_call(&repair, &[slice_or_raise(&text.clone(), &last.clone(), &mcall(&found.clone(), "start", &[]), &Py::None)])]);
    let _ = mcall(&out.clone(), "append", &[mcall(&found.clone(), "group", &[Py::Int(0)])]);
    last = mcall(&found.clone(), "end", &[]);
    }
    let _ = mcall(&out.clone(), "append", &[py_call(&repair, &[slice_or_raise(&text.clone(), &last.clone(), &Py::None, &Py::None)])]);
    return mcall(&py_str(""), "join", &[out.clone()]);
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
    word = mcall(&match_.clone(), "group", &[Py::Int(0)]);
    other = mcall(&g_APOSTROPHE(), "get", &[mcall(&word.clone(), "lower", &[]), word.clone()]);
    return if py_truthy(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) { op_add(&mcall(&slice_or_raise(&other.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&other.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { other.clone() };
    Py::None
}
    put_back = {{ let put_back = put_back.clone(); let text = text.clone(); py_func("put_back", move |args: &[Py]| { let mut full = vec![put_back.clone(), text.clone()]; full.extend_from_slice(args); __nested_put_back_0(&full) }) }};
    return _outside_quotes(&[text.clone(), { let put_back = put_back.clone(); let text = text.clone(); py_func("lambda", move |args: &[Py]| { let put_back = put_back.clone(); let text = text.clone(); let part: Py = __kwp(args, 0, "part").unwrap_or(Py::None); mcall(&g_APOSTROPHE_WORD(), "sub", &[put_back.clone(), part.clone()]) }) }]);
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
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(0)]), mcall(&match_.clone(), "group", &[Py::Int(2)]), mcall(&match_.clone(), "group", &[Py::Int(3)])]), 3, None);
    whole = __unpk1[0].clone();
    relation = __unpk1[1].clone();
    thing = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &mcall(&slice_or_raise(&relation.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_OWNED(), &mcall(&thing.clone(), "lower", &[]))) } }) {
    return whole.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WHEN_OR_WHERE(), &mcall(&thing.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { _with_somebody(&[text.clone(), mcall(&match_.clone(), "start", &[])]) } }) {
    return whole.clone();
    }
    return op_add(&op_add(&op_add(&op_add(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &py_str(" ")), &slice_or_raise(&relation.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)), &py_str("'s ")), &thing.clone());
    Py::None
}
    own = {{ let own = own.clone(); let text = text.clone(); py_func("own", move |args: &[Py]| { let mut full = vec![own.clone(), text.clone()]; full.extend_from_slice(args); __nested_own_0(&full) }) }};
    return stdlib_call("re", "sub", &[py_str("\\b((?:[Mm]y|[Oo]ur)) ([a-z]+s) ([A-Za-z0-9']+)"), own.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e));
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
    match_ = stdlib_call("re", "match", &[py_str("([A-Za-z]+s)\\s+([A-Za-z0-9']+)"), mcall(&call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return phrase.clone();
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)])]), 2, None);
    relation = __unpk1[0].clone();
    owned = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &mcall(&slice_or_raise(&relation.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_OWNED(), &mcall(&owned.clone(), "lower", &[]))) } }) {
    return phrase.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_WHEN_OR_WHERE(), &mcall(&owned.clone(), "lower", &[])))) {
    return phrase.clone();
    }
    return op_add(&op_add(&slice_or_raise(&relation.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("'s ")), &mcall(&slice_or_raise(&mcall(&phrase.clone(), "strip", &[]), &call_builtin("len", &[relation.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "lstrip", &[]));
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
    before = mcall(&mcall(&slice_or_raise(&text.clone(), &Py::None, &start.clone(), &Py::None), "rstrip", &[]), "split", &[]);
    return { let __b = call_builtin("bool", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WITH_SOMEBODY(), &mcall(&geti(&before.clone(), &op_neg(&Py::Int(1))), "lower", &[]))) } };
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
    return _outside_quotes(&[text.clone(), { let text = text.clone(); py_func("lambda", move |args: &[Py]| { let text = text.clone(); let part: Py = __kwp(args, 0, "part").unwrap_or(Py::None); _flip_to_the_second_person(&[_the_relation_possessive(&[part.clone()])]) }) }]);
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
    word = mcall(&match_.clone(), "group", &[Py::Int(0)]);
    other = mcall(&swaps.clone(), "get", &[mcall(&word.clone(), "lower", &[]), word.clone()]);
    return if py_truthy(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) { op_add(&mcall(&slice_or_raise(&other.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&other.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { other.clone() };
    Py::None
}
    swap = {{ let found = found.clone(); let index = index.clone(); let mine = mine.clone(); let owns = owns.clone(); let place = place.clone(); let said_me = said_me.clone(); let seen = seen.clone(); let swap = swap.clone(); let swaps = swaps.clone(); let text = text.clone(); let word = word.clone(); let words = words.clone(); let yours = yours.clone(); py_func("swap", move |args: &[Py]| { let mut full = vec![found.clone(), index.clone(), mine.clone(), owns.clone(), place.clone(), said_me.clone(), seen.clone(), swap.clone(), swaps.clone(), text.clone(), word.clone(), words.clone(), yours.clone()]; full.extend_from_slice(args); __nested_swap_0(&full) }) }};
    text = _the_relation_possessive(&[text.clone()]);
    text = _to_the_bot(&[text.clone()]);
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&text.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    mine = pyrt::xrefs::xcall("understanding.normalize", "possessive_me", &[words.clone()]);
    said_me = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8379 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk8379[0].clone(); let mut word: Py = __unpk8379[1].clone(); if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("me")))) {  __out.push(index.clone()); } } py_list(__out) })() };
    owns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[said_me.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk1467 = py_unpack(&__item.clone(), 2, None); let mut place: Py = __unpk1467[0].clone(); let mut index: Py = __unpk1467[1].clone(); if py_truthy(&{ let __b = op_lt(&index.clone(), &call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&mine.clone(), &index.clone()), &py_str("my"))) } }) {  __out.push(place.clone()); } } py_set(__out) })() };
    let __aug2 = op_sub(&owns.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[stdlib_call("re", "finditer", &[py_str("\\bme\\b"), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let __unpk2508 = py_unpack(&__item.clone(), 2, None); let mut place: Py = __unpk2508[0].clone(); let mut found: Py = __unpk2508[1].clone(); if py_truthy(&stdlib_call("re", "match", &[py_str("\\s*,"), slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(place.clone()); } } py_set(__out) })() });
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
    word = mcall(&match_.clone(), "group", &[Py::Int(0)]);
    if py_truthy(&Py::Bool(py_contains(&owns.clone(), &geti(&seen.clone(), &Py::Int(0))))) {
    return if py_truthy(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) { py_str("Your") } else { py_str("your") };
    }
    return word.clone();
    Py::None
}
    yours = {{ let found = found.clone(); let index = index.clone(); let mine = mine.clone(); let owns = owns.clone(); let place = place.clone(); let said_me = said_me.clone(); let seen = seen.clone(); let swap = swap.clone(); let swaps = swaps.clone(); let text = text.clone(); let word = word.clone(); let words = words.clone(); let yours = yours.clone(); py_func("yours", move |args: &[Py]| { let mut full = vec![found.clone(), index.clone(), mine.clone(), owns.clone(), place.clone(), said_me.clone(), seen.clone(), swap.clone(), swaps.clone(), text.clone(), word.clone(), words.clone(), yours.clone()]; full.extend_from_slice(args); __nested_yours_2(&full) }) }};
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bme\\b")); __args.push(yours.clone()); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(i'm|i am|i've|i have|i was not|i wasn't|i wasnt|i was|i were|we wasnt|we was|im|ive|my|mine|myself|me|we|our|ourselves|us|i)\\b")); __args.push(swap.clone()); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return without_the_apostrophe(&[_from_the_bot(&[text.clone()])]);
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
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .!?,")]);
    words = mcall(&line.clone(), "split", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ASK_WORDS(), &mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",.;:")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WERE_DOING(), "match", &[line.clone()]))) } } } }) {
    return py_str("");
    }
    cut = mcall(&g_ASK_TAIL(), "search", &[line.clone()]);
    if py_truthy(&cut.clone()) {
    line = slice_or_raise(&line.clone(), &Py::None, &mcall(&cut.clone(), "start", &[]), &Py::None);
    }
    return mcall(&line.clone(), "strip", &[py_str(" .,!?")]);
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
    wanted = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[log(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("plan"), py_str("plans")]), &mcall(&entry.clone(), "get", &[py_str("key")])))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("value")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &wanted.clone()))) {
    continue;
    }
    said_line = _statement(&[mcall(&entry.clone(), "get", &[py_str("text")])]);
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
    let __iter1 = py_iter_or_raise(&log(&[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &key.clone()))) {
    return py_tuple(vec![mcall(&entry.clone(), "get", &[py_str("value")]), mcall(&entry.clone(), "get", &[py_str("turn")])]);
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
            "a_detail_of_the_thing_we_hold" => Some(g_a_detail_of_the_thing_we_hold()),
            "about" => Some(g_about()),
            "action" => Some(g_action()),
            "already_said" => Some(g_already_said()),
            "answer_about" => Some(g_answer_about()),
            "answer_in_hand" => Some(g_answer_in_hand()),
            "answers_about_another_creature" => Some(g_answers_about_another_creature()),
            "appointment_sentence" => Some(py_func("appointment_sentence", |args: &[Py]| appointment_sentence(args))),
            "asks_only" => Some(g_asks_only()),
            "bare_question" => Some(g_bare_question()),
            "content" => Some(g_content()),
            "context" => Some(g_context()),
            "current" => Some(g_current()),
            "entry_answer" => Some(g_entry_answer()),
            "expand" => Some(g_expand()),
            "followup_reading" => Some(g_followup_reading()),
            "forget" => Some(py_func("forget", |args: &[Py]| forget(args))),
            "forms" => Some(g_forms()),
            "gist" => Some(g_gist()),
            "has_time" => Some(g_has_time()),
            "introduction_question" => Some(g_introduction_question()),
            "is_follow_up" => Some(g_is_follow_up()),
            "key_subject" => Some(g_key_subject()),
            "keywords" => Some(g_keywords()),
            "last_bot" => Some(py_func("last_bot", |args: &[Py]| last_bot(args))),
            "last_module" => Some(py_func("last_module", |args: &[Py]| last_module(args))),
            "last_user" => Some(py_func("last_user", |args: &[Py]| last_user(args))),
            "left_for_a_blank" => Some(g_left_for_a_blank()),
            "log" => Some(py_func("log", |args: &[Py]| log(args))),
            "meal_follow_up" => Some(g_meal_follow_up()),
            "name" => Some(py_func("name", |args: &[Py]| name(args))),
            "named_days" => Some(py_func("named_days", |args: &[Py]| named_days(args))),
            "note" => Some(g_note()),
            "others" => Some(py_func("others", |args: &[Py]| others(args))),
            "person" => Some(g_person()),
            "person_named" => Some(g_person_named()),
            "plan_line" => Some(py_func("plan_line", |args: &[Py]| plan_line(args))),
            "plan_sentence" => Some(py_func("plan_sentence", |args: &[Py]| plan_sentence(args))),
            "possessive_me" => Some(g_possessive_me()),
            "reads_as_a_plan" => Some(py_func("reads_as_a_plan", |args: &[Py]| reads_as_a_plan(args))),
            "recent" => Some(py_func("recent", |args: &[Py]| recent(args))),
            "remember" => Some(py_func("remember", |args: &[Py]| remember(args))),
            "remember_answer" => Some(g_remember_answer()),
            "remember_other" => Some(py_func("remember_other", |args: &[Py]| remember_other(args))),
            "resumed" => Some(g_resumed()),
            "return_target" => Some(g_return_target()),
            "said" => Some(py_func("said", |args: &[Py]| said(args))),
            "said_again" => Some(g_said_again()),
            "same_as_before" => Some(g_same_as_before()),
            "sized_reply" => Some(g_sized_reply()),
            "stem" => Some(g_stem()),
            "subject" => Some(g_subject()),
            "the_animal_the_subject_is" => Some(g_the_animal_the_subject_is()),
            "the_ask_about_the_thing_it_points_at" => Some(g_the_ask_about_the_thing_it_points_at()),
            "the_step_back_takes_its_words_back" => Some(g_the_step_back_takes_its_words_back()),
            "the_thing_a_detail_belongs_to" => Some(g_the_thing_a_detail_belongs_to()),
            "the_thing_it_is" => Some(g_the_thing_it_is()),
            "there_is_a_chat" => Some(py_func("there_is_a_chat", |args: &[Py]| there_is_a_chat(args))),
            "topic_text" => Some(g_topic_text()),
            "trouble" => Some(g_trouble()),
            "user" => Some(py_func("user", |args: &[Py]| user(args))),
            "without_filler" => Some(g_without_filler()),
            "without_the_apostrophe" => Some(py_func("without_the_apostrophe", |args: &[Py]| without_the_apostrophe(args))),
            "written_out" => Some(g_written_out()),
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
            "a_detail_of_the_thing_we_hold" => s_a_detail_of_the_thing_we_hold(v),
            "about" => s_about(v),
            "action" => s_action(v),
            "already_said" => s_already_said(v),
            "answer_about" => s_answer_about(v),
            "answer_in_hand" => s_answer_in_hand(v),
            "answers_about_another_creature" => s_answers_about_another_creature(v),
            "asks_only" => s_asks_only(v),
            "bare_question" => s_bare_question(v),
            "content" => s_content(v),
            "context" => s_context(v),
            "current" => s_current(v),
            "entry_answer" => s_entry_answer(v),
            "expand" => s_expand(v),
            "followup_reading" => s_followup_reading(v),
            "forms" => s_forms(v),
            "gist" => s_gist(v),
            "has_time" => s_has_time(v),
            "introduction_question" => s_introduction_question(v),
            "is_follow_up" => s_is_follow_up(v),
            "key_subject" => s_key_subject(v),
            "keywords" => s_keywords(v),
            "left_for_a_blank" => s_left_for_a_blank(v),
            "meal_follow_up" => s_meal_follow_up(v),
            "note" => s_note(v),
            "person" => s_person(v),
            "person_named" => s_person_named(v),
            "possessive_me" => s_possessive_me(v),
            "remember_answer" => s_remember_answer(v),
            "resumed" => s_resumed(v),
            "return_target" => s_return_target(v),
            "said_again" => s_said_again(v),
            "same_as_before" => s_same_as_before(v),
            "sized_reply" => s_sized_reply(v),
            "stem" => s_stem(v),
            "subject" => s_subject(v),
            "the_animal_the_subject_is" => s_the_animal_the_subject_is(v),
            "the_ask_about_the_thing_it_points_at" => s_the_ask_about_the_thing_it_points_at(v),
            "the_step_back_takes_its_words_back" => s_the_step_back_takes_its_words_back(v),
            "the_thing_a_detail_belongs_to" => s_the_thing_a_detail_belongs_to(v),
            "the_thing_it_is" => s_the_thing_it_is(v),
            "topic_text" => s_topic_text(v),
            "trouble" => s_trouble(v),
            "without_filler" => s_without_filler(v),
            "written_out" => s_written_out(v),
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
    pyrt::xrefs::reg_access("memory", "a_detail_of_the_thing_we_hold", g_a_detail_of_the_thing_we_hold, s_a_detail_of_the_thing_we_hold);
    pyrt::xrefs::reg_access("memory", "about", g_about, s_about);
    pyrt::xrefs::reg_access("memory", "action", g_action, s_action);
    pyrt::xrefs::reg_access("memory", "already_said", g_already_said, s_already_said);
    pyrt::xrefs::reg_access("memory", "answer_about", g_answer_about, s_answer_about);
    pyrt::xrefs::reg_access("memory", "answer_in_hand", g_answer_in_hand, s_answer_in_hand);
    pyrt::xrefs::reg_access("memory", "answers_about_another_creature", g_answers_about_another_creature, s_answers_about_another_creature);
    pyrt::xrefs::reg_access("memory", "asks_only", g_asks_only, s_asks_only);
    pyrt::xrefs::reg_access("memory", "bare_question", g_bare_question, s_bare_question);
    pyrt::xrefs::reg_access("memory", "content", g_content, s_content);
    pyrt::xrefs::reg_access("memory", "context", g_context, s_context);
    pyrt::xrefs::reg_access("memory", "current", g_current, s_current);
    pyrt::xrefs::reg_access("memory", "entry_answer", g_entry_answer, s_entry_answer);
    pyrt::xrefs::reg_access("memory", "expand", g_expand, s_expand);
    pyrt::xrefs::reg_access("memory", "followup_reading", g_followup_reading, s_followup_reading);
    pyrt::xrefs::reg_access("memory", "forms", g_forms, s_forms);
    pyrt::xrefs::reg_access("memory", "gist", g_gist, s_gist);
    pyrt::xrefs::reg_access("memory", "has_time", g_has_time, s_has_time);
    pyrt::xrefs::reg_access("memory", "introduction_question", g_introduction_question, s_introduction_question);
    pyrt::xrefs::reg_access("memory", "is_follow_up", g_is_follow_up, s_is_follow_up);
    pyrt::xrefs::reg_access("memory", "key_subject", g_key_subject, s_key_subject);
    pyrt::xrefs::reg_access("memory", "keywords", g_keywords, s_keywords);
    pyrt::xrefs::reg_access("memory", "left_for_a_blank", g_left_for_a_blank, s_left_for_a_blank);
    pyrt::xrefs::reg_access("memory", "meal_follow_up", g_meal_follow_up, s_meal_follow_up);
    pyrt::xrefs::reg_access("memory", "note", g_note, s_note);
    pyrt::xrefs::reg_access("memory", "person", g_person, s_person);
    pyrt::xrefs::reg_access("memory", "person_named", g_person_named, s_person_named);
    pyrt::xrefs::reg_access("memory", "possessive_me", g_possessive_me, s_possessive_me);
    pyrt::xrefs::reg_access("memory", "remember_answer", g_remember_answer, s_remember_answer);
    pyrt::xrefs::reg_access("memory", "resumed", g_resumed, s_resumed);
    pyrt::xrefs::reg_access("memory", "return_target", g_return_target, s_return_target);
    pyrt::xrefs::reg_access("memory", "said_again", g_said_again, s_said_again);
    pyrt::xrefs::reg_access("memory", "same_as_before", g_same_as_before, s_same_as_before);
    pyrt::xrefs::reg_access("memory", "sized_reply", g_sized_reply, s_sized_reply);
    pyrt::xrefs::reg_access("memory", "stem", g_stem, s_stem);
    pyrt::xrefs::reg_access("memory", "subject", g_subject, s_subject);
    pyrt::xrefs::reg_access("memory", "the_animal_the_subject_is", g_the_animal_the_subject_is, s_the_animal_the_subject_is);
    pyrt::xrefs::reg_access("memory", "the_ask_about_the_thing_it_points_at", g_the_ask_about_the_thing_it_points_at, s_the_ask_about_the_thing_it_points_at);
    pyrt::xrefs::reg_access("memory", "the_step_back_takes_its_words_back", g_the_step_back_takes_its_words_back, s_the_step_back_takes_its_words_back);
    pyrt::xrefs::reg_access("memory", "the_thing_a_detail_belongs_to", g_the_thing_a_detail_belongs_to, s_the_thing_a_detail_belongs_to);
    pyrt::xrefs::reg_access("memory", "the_thing_it_is", g_the_thing_it_is, s_the_thing_it_is);
    pyrt::xrefs::reg_access("memory", "topic_text", g_topic_text, s_topic_text);
    pyrt::xrefs::reg_access("memory", "trouble", g_trouble, s_trouble);
    pyrt::xrefs::reg_access("memory", "without_filler", g_without_filler, s_without_filler);
    pyrt::xrefs::reg_access("memory", "written_out", g_written_out, s_written_out);
}
