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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASLEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MARK_NOT_A_GRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHAT_TO_WEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAREER_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CATCHING_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLASS_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLASS_BAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLASS_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLASS_SAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONCEPTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONCEPT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTED_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COURSE_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENGLISH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ESSAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXAM_NERVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXAM_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXTENSION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRACTIONS_ADD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRACTIONS_DIVIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRACTIONS_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRACTIONS_MULTIPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRACTIONS_SIMPLIFY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRADE_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HALF_OF_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB_LENGTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LANGUAGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LANGUAGE_HOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LANGUAGE_LEARNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LANGUAGE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEARNING_OBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LETTER_GRADES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOSING_TRACK_OF_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NERVES_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_EXAM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_PERSONS_NOTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_START_OF_EXAMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_REVISION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASS_MARK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEST_AS_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_GENERAL_CAREER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_AFTER_TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOP_GRADES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WELSH_AND_IRISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_THE_PLATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKED_OUTRIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKED_TO_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_THE_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CONCLUSION_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DEGREE_OF_HEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEANING_IS_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CAREER_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CORRECTION_FURNITURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CORRECTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COURSE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DONE_IT_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ENGLISH_FURNITURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ENGLISH_TOO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXAM_IN_THE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAILING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEELS_WRONGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FORGETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FORGET_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRADES_ROUTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_TO_REMEMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IS_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LAST_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LATER_STUDY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEANING_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEMORISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_AGE_STUDY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OTHER_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAST_PAPERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAST_PAPERS_ASK_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAST_PAPERS_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PHRASE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REMEMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAY_OR_TRANSLATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SCHOOL_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMETHING_WRITTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TEMPERATURE_DEGREES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_ANNOUNCEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_PAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WRITING_HAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__by_language: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__concept: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__concept_name: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__language: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__phrase: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__plain: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__said_in: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__say: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__word: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_c: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_nerves_opening: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.study: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASLEEP() -> Py { ensure_init(); G_ASLEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASLEEP(v: Py) { ensure_init(); G_ASLEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MARK_NOT_A_GRADE() -> Py { ensure_init(); G_A_MARK_NOT_A_GRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MARK_NOT_A_GRADE(v: Py) { ensure_init(); G_A_MARK_NOT_A_GRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBJECT() -> Py { ensure_init(); G_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT(v: Py) { ensure_init(); G_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHAT_TO_WEAR() -> Py { ensure_init(); G_A_WHAT_TO_WEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHAT_TO_WEAR(v: Py) { ensure_init(); G_A_WHAT_TO_WEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_CAREER_SUBJECTS() -> Py { ensure_init(); G_CAREER_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAREER_SUBJECTS(v: Py) { ensure_init(); G_CAREER_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_CATCHING_UP() -> Py { ensure_init(); G_CATCHING_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CATCHING_UP(v: Py) { ensure_init(); G_CATCHING_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_CLASS_ALONE() -> Py { ensure_init(); G_CLASS_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLASS_ALONE(v: Py) { ensure_init(); G_CLASS_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLASS_BAND() -> Py { ensure_init(); G_CLASS_BAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLASS_BAND(v: Py) { ensure_init(); G_CLASS_BAND.with(|c| *c.borrow_mut() = v); }
pub fn g_CLASS_NAMES() -> Py { ensure_init(); G_CLASS_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLASS_NAMES(v: Py) { ensure_init(); G_CLASS_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_CLASS_SAYS() -> Py { ensure_init(); G_CLASS_SAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLASS_SAYS(v: Py) { ensure_init(); G_CLASS_SAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONCEPTS() -> Py { ensure_init(); G_CONCEPTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONCEPTS(v: Py) { ensure_init(); G_CONCEPTS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONCEPT_WORDS() -> Py { ensure_init(); G_CONCEPT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONCEPT_WORDS(v: Py) { ensure_init(); G_CONCEPT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTED_AFTER() -> Py { ensure_init(); G_COUNTED_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTED_AFTER(v: Py) { ensure_init(); G_COUNTED_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_COURSE_HOURS() -> Py { ensure_init(); G_COURSE_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COURSE_HOURS(v: Py) { ensure_init(); G_COURSE_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENGLISH_WORDS() -> Py { ensure_init(); G_ENGLISH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENGLISH_WORDS(v: Py) { ensure_init(); G_ENGLISH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_ESSAY_WORDS() -> Py { ensure_init(); G_ESSAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ESSAY_WORDS(v: Py) { ensure_init(); G_ESSAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_EXAM_NERVES() -> Py { ensure_init(); G_EXAM_NERVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXAM_NERVES(v: Py) { ensure_init(); G_EXAM_NERVES.with(|c| *c.borrow_mut() = v); }
pub fn g_EXAM_WHEN() -> Py { ensure_init(); G_EXAM_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXAM_WHEN(v: Py) { ensure_init(); G_EXAM_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_EXTENSION_ASK() -> Py { ensure_init(); G_EXTENSION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXTENSION_ASK(v: Py) { ensure_init(); G_EXTENSION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_FRACTIONS_ADD() -> Py { ensure_init(); G_FRACTIONS_ADD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRACTIONS_ADD(v: Py) { ensure_init(); G_FRACTIONS_ADD.with(|c| *c.borrow_mut() = v); }
pub fn g_FRACTIONS_DIVIDE() -> Py { ensure_init(); G_FRACTIONS_DIVIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRACTIONS_DIVIDE(v: Py) { ensure_init(); G_FRACTIONS_DIVIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_FRACTIONS_FIRST() -> Py { ensure_init(); G_FRACTIONS_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRACTIONS_FIRST(v: Py) { ensure_init(); G_FRACTIONS_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_FRACTIONS_MULTIPLY() -> Py { ensure_init(); G_FRACTIONS_MULTIPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRACTIONS_MULTIPLY(v: Py) { ensure_init(); G_FRACTIONS_MULTIPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_FRACTIONS_SIMPLIFY() -> Py { ensure_init(); G_FRACTIONS_SIMPLIFY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRACTIONS_SIMPLIFY(v: Py) { ensure_init(); G_FRACTIONS_SIMPLIFY.with(|c| *c.borrow_mut() = v); }
pub fn g_GRADE_NAME() -> Py { ensure_init(); G_GRADE_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRADE_NAME(v: Py) { ensure_init(); G_GRADE_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_HALF_OF_CLASS() -> Py { ensure_init(); G_HALF_OF_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HALF_OF_CLASS(v: Py) { ensure_init(); G_HALF_OF_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_ASK() -> Py { ensure_init(); G_HOW_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_ASK(v: Py) { ensure_init(); G_HOW_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB_LENGTHS() -> Py { ensure_init(); G_JOB_LENGTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB_LENGTHS(v: Py) { ensure_init(); G_JOB_LENGTHS.with(|c| *c.borrow_mut() = v); }
pub fn g_LANGUAGES() -> Py { ensure_init(); G_LANGUAGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LANGUAGES(v: Py) { ensure_init(); G_LANGUAGES.with(|c| *c.borrow_mut() = v); }
pub fn g_LANGUAGE_HOURS() -> Py { ensure_init(); G_LANGUAGE_HOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LANGUAGE_HOURS(v: Py) { ensure_init(); G_LANGUAGE_HOURS.with(|c| *c.borrow_mut() = v); }
pub fn g_LANGUAGE_LEARNING() -> Py { ensure_init(); G_LANGUAGE_LEARNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LANGUAGE_LEARNING(v: Py) { ensure_init(); G_LANGUAGE_LEARNING.with(|c| *c.borrow_mut() = v); }
pub fn g_LANGUAGE_NAMES() -> Py { ensure_init(); G_LANGUAGE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LANGUAGE_NAMES(v: Py) { ensure_init(); G_LANGUAGE_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_LEARNING_OBJECTS() -> Py { ensure_init(); G_LEARNING_OBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEARNING_OBJECTS(v: Py) { ensure_init(); G_LEARNING_OBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_LETTER_GRADES() -> Py { ensure_init(); G_LETTER_GRADES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LETTER_GRADES(v: Py) { ensure_init(); G_LETTER_GRADES.with(|c| *c.borrow_mut() = v); }
pub fn g_LOSING_TRACK_OF_MONEY() -> Py { ensure_init(); G_LOSING_TRACK_OF_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOSING_TRACK_OF_MONEY(v: Py) { ensure_init(); G_LOSING_TRACK_OF_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_MOCK() -> Py { ensure_init(); G_MOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOCK(v: Py) { ensure_init(); G_MOCK.with(|c| *c.borrow_mut() = v); }
pub fn g_NATIVE() -> Py { ensure_init(); G_NATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NATIVE(v: Py) { ensure_init(); G_NATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NERVES_BODY() -> Py { ensure_init(); G_NERVES_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NERVES_BODY(v: Py) { ensure_init(); G_NERVES_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_EXAM() -> Py { ensure_init(); G_NOT_AN_EXAM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_EXAM(v: Py) { ensure_init(); G_NOT_AN_EXAM.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_PERSONS_NOTE() -> Py { ensure_init(); G_NOT_A_PERSONS_NOTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PERSONS_NOTE(v: Py) { ensure_init(); G_NOT_A_PERSONS_NOTE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_START_OF_EXAMS() -> Py { ensure_init(); G_NOT_A_START_OF_EXAMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_START_OF_EXAMS(v: Py) { ensure_init(); G_NOT_A_START_OF_EXAMS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_REVISION() -> Py { ensure_init(); G_NOT_REVISION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_REVISION(v: Py) { ensure_init(); G_NOT_REVISION.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_CLASS() -> Py { ensure_init(); G_NOT_THE_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_CLASS(v: Py) { ensure_init(); G_NOT_THE_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_PASS_MARK() -> Py { ensure_init(); G_PASS_MARK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASS_MARK(v: Py) { ensure_init(); G_PASS_MARK.with(|c| *c.borrow_mut() = v); }
pub fn g_PHRASES() -> Py { ensure_init(); G_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PHRASES(v: Py) { ensure_init(); G_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_REST_ASK() -> Py { ensure_init(); G_REST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REST_ASK(v: Py) { ensure_init(); G_REST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SCORE() -> Py { ensure_init(); G_SCORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCORE(v: Py) { ensure_init(); G_SCORE.with(|c| *c.borrow_mut() = v); }
pub fn g_TEST_AS_A_VERB() -> Py { ensure_init(); G_TEST_AS_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEST_AS_A_VERB(v: Py) { ensure_init(); G_TEST_AS_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_GENERAL_CAREER() -> Py { ensure_init(); G_THE_GENERAL_CAREER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_GENERAL_CAREER(v: Py) { ensure_init(); G_THE_GENERAL_CAREER.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_AFTER_TEST() -> Py { ensure_init(); G_TIME_AFTER_TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_AFTER_TEST(v: Py) { ensure_init(); G_TIME_AFTER_TEST.with(|c| *c.borrow_mut() = v); }
pub fn g_TOP_GRADES() -> Py { ensure_init(); G_TOP_GRADES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOP_GRADES(v: Py) { ensure_init(); G_TOP_GRADES.with(|c| *c.borrow_mut() = v); }
pub fn g_WELSH_AND_IRISH() -> Py { ensure_init(); G_WELSH_AND_IRISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WELSH_AND_IRISH(v: Py) { ensure_init(); G_WELSH_AND_IRISH.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_THE_PLATE() -> Py { ensure_init(); G__ABOUT_THE_PLATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_THE_PLATE(v: Py) { ensure_init(); G__ABOUT_THE_PLATE.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_ASK() -> Py { ensure_init(); G__AGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_ASK(v: Py) { ensure_init(); G__AGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_NUMBER() -> Py { ensure_init(); G__AGE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_NUMBER(v: Py) { ensure_init(); G__AGE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_SAID() -> Py { ensure_init(); G__AGE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_SAID(v: Py) { ensure_init(); G__AGE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED_OUTRIGHT() -> Py { ensure_init(); G__ASKED_OUTRIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED_OUTRIGHT(v: Py) { ensure_init(); G__ASKED_OUTRIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED_TO_MOVE() -> Py { ensure_init(); G__ASKED_TO_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED_TO_MOVE(v: Py) { ensure_init(); G__ASKED_TO_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_THE_LENGTH() -> Py { ensure_init(); G__ASKS_THE_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_THE_LENGTH(v: Py) { ensure_init(); G__ASKS_THE_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CONCLUSION_SAID() -> Py { ensure_init(); G__A_CONCLUSION_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CONCLUSION_SAID(v: Py) { ensure_init(); G__A_CONCLUSION_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DEGREE_OF_HEAT() -> Py { ensure_init(); G__A_DEGREE_OF_HEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DEGREE_OF_HEAT(v: Py) { ensure_init(); G__A_DEGREE_OF_HEAT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEANING_IS_ASKED() -> Py { ensure_init(); G__A_MEANING_IS_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEANING_IS_ASKED(v: Py) { ensure_init(); G__A_MEANING_IS_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__CAREER_CHANGE() -> Py { ensure_init(); G__CAREER_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CAREER_CHANGE(v: Py) { ensure_init(); G__CAREER_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILD() -> Py { ensure_init(); G__CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILD(v: Py) { ensure_init(); G__CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__CORRECTION_FURNITURE() -> Py { ensure_init(); G__CORRECTION_FURNITURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CORRECTION_FURNITURE(v: Py) { ensure_init(); G__CORRECTION_FURNITURE.with(|c| *c.borrow_mut() = v); }
pub fn g__CORRECTION_WORDS() -> Py { ensure_init(); G__CORRECTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CORRECTION_WORDS(v: Py) { ensure_init(); G__CORRECTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__COURSE_TIME() -> Py { ensure_init(); G__COURSE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COURSE_TIME(v: Py) { ensure_init(); G__COURSE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__DONE_IT_BEFORE() -> Py { ensure_init(); G__DONE_IT_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DONE_IT_BEFORE(v: Py) { ensure_init(); G__DONE_IT_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__ENGLISH_FURNITURE() -> Py { ensure_init(); G__ENGLISH_FURNITURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ENGLISH_FURNITURE(v: Py) { ensure_init(); G__ENGLISH_FURNITURE.with(|c| *c.borrow_mut() = v); }
pub fn g__ENGLISH_TOO() -> Py { ensure_init(); G__ENGLISH_TOO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ENGLISH_TOO(v: Py) { ensure_init(); G__ENGLISH_TOO.with(|c| *c.borrow_mut() = v); }
pub fn g__EXAM_IN_THE_WORDS() -> Py { ensure_init(); G__EXAM_IN_THE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXAM_IN_THE_WORDS(v: Py) { ensure_init(); G__EXAM_IN_THE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__FAILING() -> Py { ensure_init(); G__FAILING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAILING(v: Py) { ensure_init(); G__FAILING.with(|c| *c.borrow_mut() = v); }
pub fn g__FEELS_WRONGED() -> Py { ensure_init(); G__FEELS_WRONGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEELS_WRONGED(v: Py) { ensure_init(); G__FEELS_WRONGED.with(|c| *c.borrow_mut() = v); }
pub fn g__FORGETTING() -> Py { ensure_init(); G__FORGETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FORGETTING(v: Py) { ensure_init(); G__FORGETTING.with(|c| *c.borrow_mut() = v); }
pub fn g__FORGET_TO_DO() -> Py { ensure_init(); G__FORGET_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FORGET_TO_DO(v: Py) { ensure_init(); G__FORGET_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g__GRADES_ROUTE() -> Py { ensure_init(); G__GRADES_ROUTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRADES_ROUTE(v: Py) { ensure_init(); G__GRADES_ROUTE.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_TO_REMEMBER() -> Py { ensure_init(); G__HOW_TO_REMEMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_TO_REMEMBER(v: Py) { ensure_init(); G__HOW_TO_REMEMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__IS_CHILD() -> Py { ensure_init(); G__IS_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IS_CHILD(v: Py) { ensure_init(); G__IS_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__LAST_TIME() -> Py { ensure_init(); G__LAST_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LAST_TIME(v: Py) { ensure_init(); G__LAST_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__LATER_STUDY() -> Py { ensure_init(); G__LATER_STUDY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LATER_STUDY(v: Py) { ensure_init(); G__LATER_STUDY.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD_IN() -> Py { ensure_init(); G__LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD_IN(v: Py) { ensure_init(); G__LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g__MEANING_FRAME() -> Py { ensure_init(); G__MEANING_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEANING_FRAME(v: Py) { ensure_init(); G__MEANING_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g__MEMORISE() -> Py { ensure_init(); G__MEMORISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEMORISE(v: Py) { ensure_init(); G__MEMORISE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_AGE_STUDY() -> Py { ensure_init(); G__NOT_AGE_STUDY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_AGE_STUDY(v: Py) { ensure_init(); G__NOT_AGE_STUDY.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_WORDS() -> Py { ensure_init(); G__NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_WORDS(v: Py) { ensure_init(); G__NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__OTHER_PEOPLE() -> Py { ensure_init(); G__OTHER_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OTHER_PEOPLE(v: Py) { ensure_init(); G__OTHER_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__PAST_PAPERS() -> Py { ensure_init(); G__PAST_PAPERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAST_PAPERS(v: Py) { ensure_init(); G__PAST_PAPERS.with(|c| *c.borrow_mut() = v); }
pub fn g__PAST_PAPERS_ASK_WHEN() -> Py { ensure_init(); G__PAST_PAPERS_ASK_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAST_PAPERS_ASK_WHEN(v: Py) { ensure_init(); G__PAST_PAPERS_ASK_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g__PAST_PAPERS_WHEN() -> Py { ensure_init(); G__PAST_PAPERS_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAST_PAPERS_WHEN(v: Py) { ensure_init(); G__PAST_PAPERS_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g__PHRASE_ASK() -> Py { ensure_init(); G__PHRASE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PHRASE_ASK(v: Py) { ensure_init(); G__PHRASE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__REMEMBERS() -> Py { ensure_init(); G__REMEMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REMEMBERS(v: Py) { ensure_init(); G__REMEMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g__SAY_OR_TRANSLATE() -> Py { ensure_init(); G__SAY_OR_TRANSLATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAY_OR_TRANSLATE(v: Py) { ensure_init(); G__SAY_OR_TRANSLATE.with(|c| *c.borrow_mut() = v); }
pub fn g__SCHOOL_THINGS() -> Py { ensure_init(); G__SCHOOL_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SCHOOL_THINGS(v: Py) { ensure_init(); G__SCHOOL_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMETHING_WRITTEN() -> Py { ensure_init(); G__SOMETHING_WRITTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMETHING_WRITTEN(v: Py) { ensure_init(); G__SOMETHING_WRITTEN.with(|c| *c.borrow_mut() = v); }
pub fn g__TEMPERATURE_DEGREES() -> Py { ensure_init(); G__TEMPERATURE_DEGREES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TEMPERATURE_DEGREES(v: Py) { ensure_init(); G__TEMPERATURE_DEGREES.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_ANNOUNCEMENT() -> Py { ensure_init(); G__THE_ANNOUNCEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_ANNOUNCEMENT(v: Py) { ensure_init(); G__THE_ANNOUNCEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_PAGE() -> Py { ensure_init(); G__THE_PAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_PAGE(v: Py) { ensure_init(); G__THE_PAGE.with(|c| *c.borrow_mut() = v); }
pub fn g__WRITING_HAND() -> Py { ensure_init(); G__WRITING_HAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WRITING_HAND(v: Py) { ensure_init(); G__WRITING_HAND.with(|c| *c.borrow_mut() = v); }
pub fn g__by_language() -> Py { ensure_init(); G__by_language.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__by_language(v: Py) { ensure_init(); G__by_language.with(|c| *c.borrow_mut() = v); }
pub fn g__concept() -> Py { ensure_init(); G__concept.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__concept(v: Py) { ensure_init(); G__concept.with(|c| *c.borrow_mut() = v); }
pub fn g__concept_name() -> Py { ensure_init(); G__concept_name.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__concept_name(v: Py) { ensure_init(); G__concept_name.with(|c| *c.borrow_mut() = v); }
pub fn g__language() -> Py { ensure_init(); G__language.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__language(v: Py) { ensure_init(); G__language.with(|c| *c.borrow_mut() = v); }
pub fn g__phrase() -> Py { ensure_init(); G__phrase.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__phrase(v: Py) { ensure_init(); G__phrase.with(|c| *c.borrow_mut() = v); }
pub fn g__plain() -> Py { ensure_init(); G__plain.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__plain(v: Py) { ensure_init(); G__plain.with(|c| *c.borrow_mut() = v); }
pub fn g__said_in() -> Py { ensure_init(); G__said_in.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__said_in(v: Py) { ensure_init(); G__said_in.with(|c| *c.borrow_mut() = v); }
pub fn g__say() -> Py { ensure_init(); G__say.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__say(v: Py) { ensure_init(); G__say.with(|c| *c.borrow_mut() = v); }
pub fn g__word() -> Py { ensure_init(); G__word.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__word(v: Py) { ensure_init(); G__word.with(|c| *c.borrow_mut() = v); }
pub fn g_c() -> Py { ensure_init(); G_c.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_c(v: Py) { ensure_init(); G_c.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_nerves_opening() -> Py { ensure_init(); G_nerves_opening.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_nerves_opening(v: Py) { ensure_init(); G_nerves_opening.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    s__concept(py_func("_concept", |args: &[Py]| _concept__wrapped(args)));
    let _ = py_str("School and study: revising, remembering, essays, notes, and learning a language.\n\nEverything here is data on the merged `lookup.entry` contract: one entry per thing people\nask about, keyed by the words that identify it (revising, an essay, a words-in-Spanish\nquestion) with a reply phrased the way a helpful older student would say it. The matcher\nin lookup.py picks the entry; bot/modules/answers.py turns it into a reply.\n\nTwo general mechanisms live here:\n- START / ONE THING style advice is always concrete (do this today, for ten minutes),\n  because \"make a timetable\" on its own is what the person has already failed at.\n- PHRASES is a small table of everyday phrases in four languages, read by a callable\n  reply, so one entry answers every \"how do you say X in Y\" question instead of one\n  entry per phrase.\n");
    s_A_WHAT_TO_WEAR(pyrt::xrefs::xget("knowledge.lookup", "A_WHAT_TO_WEAR"));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ENGLISH_WORDS(pyrt::xrefs::xget("understanding.normalize", "BASE"));
    s_NERVES_BODY(pyrt::xrefs::xget("knowledge.wording", "NERVES_BODY"));
    s_nerves_opening(py_func("nerves_opening", |args: &[Py]| pyrt::xrefs::xcall("knowledge.wording", "nerves_opening", args)));
    s_ANSWERS(py_tuple(vec![py_str("school-and-study"), py_str("language-and-writing")]));
    s_FRACTIONS_DIVIDE(py_str("Flip the second fraction upside down and multiply: 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9. Multiplying is top times top and bottom times bottom (2/3 x 3/5 = 6/15, which simplifies to 2/5), and adding or subtracting needs the same bottom number first, so 1/3 + 1/4 = 4/12 + 3/12 = 7/12."));
    s_FRACTIONS_MULTIPLY(py_str("Top times top, bottom times bottom: 2/3 x 3/5 = 6/15, which simplifies to 2/5. Dividing is the same trick with the second one flipped over, so 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9. Adding and subtracting need the same bottom number first: 1/3 + 1/4 = 4/12 + 3/12 = 7/12."));
    s_FRACTIONS_ADD(py_str("Get the same bottom number first: 1/3 + 1/4 becomes 4/12 + 3/12 = 7/12, and subtracting works the same way. Multiplying is top times top and bottom times bottom (2/3 x 3/5 = 6/15, which simplifies to 2/5); dividing flips the second one over and multiplies, so 2/3 divided by 3/5 is 2/3 x 5/3 = 10/9."));
    s_FRACTIONS_SIMPLIFY(py_str("Divide the top and the bottom by the same number until nothing goes into both: 6/15 divide by 3 and you get 2/5. For sums, multiply top times top and bottom times bottom (2/3 x 3/5 = 6/15, then simplify), divide by flipping the second fraction over (2/3 divided by 3/5 is 2/3 x 5/3 = 10/9), and add or subtract once the bottom numbers match (1/3 + 1/4 = 7/12)."));
    s_FRACTIONS_FIRST(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("divide"), py_str("dividing"), py_str("divided")]), g_FRACTIONS_DIVIDE()]), py_tuple(vec![py_tuple(vec![py_str("multiply"), py_str("multiplying"), py_str("multiplied"), py_str("times")]), g_FRACTIONS_MULTIPLY()]), py_tuple(vec![py_tuple(vec![py_str("add"), py_str("adding"), py_str("plus"), py_str("subtract"), py_str("subtracting"), py_str("subtracted"), py_str("minus")]), g_FRACTIONS_ADD()]), py_tuple(vec![py_tuple(vec![py_str("simplify"), py_str("simplifying"), py_str("cancel"), py_str("lowest")]), g_FRACTIONS_SIMPLIFY()])]));
    s__PAST_PAPERS(py_str("Do them under timed conditions, then mark yourself against the real mark scheme - the marking is where most of the learning is. Keep a list of the question types you lose marks on and redo only those next time. If you are short of time, do the paper in your head and write full answers only for the weak sections."));
    s__PAST_PAPERS_WHEN(py_str("Start about six to eight weeks before the exam, one paper a week, then two a week in the last month, so the early marks still leave you time to fix what they show. Do each under timed conditions and mark it against the real mark scheme, and redo only the question types you lose marks on."));
    fn __data1() -> Py { py_tuple(vec![py_str("when"), py_str("should"), py_str("start"), py_str("starting"), py_str("begin"), py_str("leave"), py_str("leaving"), py_str("left"), py_str("soon"), py_str("late"), py_str("early"), py_str("before"), py_str("how long"), py_str("how many weeks"), py_str("how many months")]) }
    s__PAST_PAPERS_ASK_WHEN(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_dict(vec![(py_str("portuguese"), py_tuple(vec![py_str("Onde fica a casa de banho?"), py_str("ON-dy FEE-ka a KA-za dy BAN-yo")])), (py_str("greek"), py_tuple(vec![py_str("Poú eínai i toualéta? (πού είναι η τουαλέτα)"), py_str("poo EE-neh ee too-a-LEH-ta")])), (py_str("turkish"), py_tuple(vec![py_str("Tuvalet nerede?"), py_str("too-va-LET NEH-reh-deh")])), (py_str("welsh"), py_tuple(vec![py_str("Ble mae’r toiled?"), py_str("bleh mire TOY-led")])), (py_str("japanese"), py_tuple(vec![py_str("Toire wa doko desu ka (トイレはどこですか)"), py_str("TOY-reh wah DOH-koh dess kah")])), (py_str("chinese"), py_tuple(vec![py_str("Xǐshǒujiān zài nǎr? (洗手间在哪儿)"), py_str("shee-show-jyen dzai nar")])), (py_str("dutch"), py_tuple(vec![py_str("Waar is het toilet?"), py_str("vaar is hut twa-LET")])), (py_str("spanish"), py_tuple(vec![py_str("¿Dónde está el baño?"), py_str("DON-day es-TA el BAN-yo")])), (py_str("french"), py_tuple(vec![py_str("Où sont les toilettes ?"), py_str("oo son lay twa-LET")])), (py_str("italian"), py_tuple(vec![py_str("Dov'è il bagno?"), py_str("do-VEH eel BAN-yo")])), (py_str("german"), py_tuple(vec![py_str("Wo ist die Toilette?"), py_str("vo ist dee toy-LET-tuh")])), (py_str("welsh"), py_tuple(vec![py_str("Ble mae'r toiled?"), py_str("blay my-re TOY-led")])), (py_str("irish"), py_tuple(vec![py_str("Cá bhfuil an leithreas?"), py_str("kaw will an LEH-rass")]))]) }
    fn __data4() -> Py { py_dict(vec![(py_str("portuguese"), py_tuple(vec![py_str("Obrigado (a woman says obrigada)"), py_str("oh-bree-GAH-doo")])), (py_str("greek"), py_tuple(vec![py_str("Efcharistó (ευχαριστώ)"), py_str("ef-kha-ree-STOH")])), (py_str("turkish"), py_tuple(vec![py_str("Teşekkürler"), py_str("teh-shek-kewr-LER")])), (py_str("welsh"), py_tuple(vec![py_str("Diolch"), py_str("DEE-olch")])), (py_str("japanese"), py_tuple(vec![py_str("Arigatou (ありがとう)"), py_str("a-ree-GA-toh")])), (py_str("chinese"), py_tuple(vec![py_str("Xièxie (谢谢)"), py_str("shyeh-shyeh")])), (py_str("dutch"), py_tuple(vec![py_str("Dank je wel"), py_str("dank yuh vel")])), (py_str("spanish"), py_tuple(vec![py_str("Gracias"), py_str("GRA-thee-as")])), (py_str("french"), py_tuple(vec![py_str("Merci"), py_str("mair-SEE")])), (py_str("italian"), py_tuple(vec![py_str("Grazie"), py_str("GRAT-zee-eh")])), (py_str("german"), py_tuple(vec![py_str("Danke"), py_str("DAN-kuh")])), (py_str("welsh"), py_tuple(vec![py_str("Diolch"), py_str("DEE-olkh")])), (py_str("irish"), py_tuple(vec![py_str("Go raibh maith agat"), py_str("guh rev MAH a-gut")])), (py_str("gaelic"), py_tuple(vec![py_str("Tapadh leat"), py_str("TA-pah lat")]))]) }
    fn __data5() -> Py { py_dict(vec![(py_str("portuguese"), py_tuple(vec![py_str("Por favor"), py_str("por fa-VOR")])), (py_str("greek"), py_tuple(vec![py_str("Parakaló (παρακαλώ)"), py_str("pa-ra-ka-LOH")])), (py_str("turkish"), py_tuple(vec![py_str("Lütfen"), py_str("LEWT-fen")])), (py_str("welsh"), py_tuple(vec![py_str("Os gwelwch yn dda"), py_str("oss gwel-ooch un thah")])), (py_str("japanese"), py_tuple(vec![py_str("Onegaishimasu (お願いします)"), py_str("o-neh-GUY-shee-mas")])), (py_str("chinese"), py_tuple(vec![py_str("Qǐng (请)"), py_str("ching")])), (py_str("dutch"), py_tuple(vec![py_str("Alstublieft"), py_str("als-too-BLEEFT")])), (py_str("spanish"), py_tuple(vec![py_str("Por favor"), py_str("por fa-VOR")])), (py_str("french"), py_tuple(vec![py_str("S'il vous plaît"), py_str("seel voo PLEH")])), (py_str("italian"), py_tuple(vec![py_str("Per favore"), py_str("per fa-VO-reh")])), (py_str("german"), py_tuple(vec![py_str("Bitte"), py_str("BIT-tuh")])), (py_str("welsh"), py_tuple(vec![py_str("Os gwelwch yn dda"), py_str("os GWEL-ookh un thah")])), (py_str("irish"), py_tuple(vec![py_str("Le do thoil"), py_str("leh duh HULL")])), (py_str("gaelic"), py_tuple(vec![py_str("Mas e do thoil e"), py_str("mas eh daw HOLL eh")]))]) }
    fn __data6() -> Py { py_dict(vec![(py_str("portuguese"), py_tuple(vec![py_str("Olá"), py_str("oh-LAH")])), (py_str("greek"), py_tuple(vec![py_str("Kaliméra (καλημέρα)"), py_str("ka-lee-MEH-ra")])), (py_str("turkish"), py_tuple(vec![py_str("Merhaba"), py_str("mer-ha-BAH")])), (py_str("welsh"), py_tuple(vec![py_str("Helo (Bore da in the morning)"), py_str("HEH-loh")])), (py_str("japanese"), py_tuple(vec![py_str("Konnichiwa (こんにちは)"), py_str("kon-nee-CHEE-wah")])), (py_str("chinese"), py_tuple(vec![py_str("Nǐ hǎo (你好)"), py_str("nee HOW")])), (py_str("dutch"), py_tuple(vec![py_str("Hallo"), py_str("HAL-oh")])), (py_str("spanish"), py_tuple(vec![py_str("Hola"), py_str("OH-la")])), (py_str("french"), py_tuple(vec![py_str("Bonjour"), py_str("bon-ZHOOR")])), (py_str("italian"), py_tuple(vec![py_str("Ciao"), py_str("chow")])), (py_str("german"), py_tuple(vec![py_str("Hallo"), py_str("HAL-oh")])), (py_str("welsh"), py_tuple(vec![py_str("Helo"), py_str("HEH-lo")])), (py_str("irish"), py_tuple(vec![py_str("Dia dhuit"), py_str("DEE-a gwit")])), (py_str("gaelic"), py_tuple(vec![py_str("Halò"), py_str("ha-LAW")]))]) }
    fn __data7() -> Py { py_dict(vec![(py_str("portuguese"), py_tuple(vec![py_str("Adeus"), py_str("a-DEHS")])), (py_str("greek"), py_tuple(vec![py_str("Antío (αντίο)"), py_str("an-DEE-oh")])), (py_str("turkish"), py_tuple(vec![py_str("Hoşça kalın"), py_str("hosh-cha ka-LUN")])), (py_str("welsh"), py_tuple(vec![py_str("Hwyl fawr"), py_str("hooil vowr")])), (py_str("japanese"), py_tuple(vec![py_str("Sayounara (さようなら)"), py_str("sa-YOH-na-ra")])), (py_str("chinese"), py_tuple(vec![py_str("Zàijiàn (再见)"), py_str("dzai-jyen")])), (py_str("dutch"), py_tuple(vec![py_str("Tot ziens"), py_str("tot zeens")])), (py_str("spanish"), py_tuple(vec![py_str("Adiós"), py_str("a-DYOS")])), (py_str("french"), py_tuple(vec![py_str("Au revoir"), py_str("oh ruh-VWAR")])), (py_str("italian"), py_tuple(vec![py_str("Arrivederci"), py_str("a-ree-veh-DAIR-chee")])), (py_str("german"), py_tuple(vec![py_str("Auf Wiedersehen"), py_str("owf VEE-der-zayn")])), (py_str("welsh"), py_tuple(vec![py_str("Hwyl fawr"), py_str("hooil vower")])), (py_str("irish"), py_tuple(vec![py_str("Slán"), py_str("slawn")])), (py_str("gaelic"), py_tuple(vec![py_str("Mar sin leat"), py_str("mar shin lat")]))]) }
    fn __data8() -> Py { py_dict(vec![(py_str("portuguese"), py_tuple(vec![py_str("Quanto custa?"), py_str("KWAN-too KOOSH-ta")])), (py_str("greek"), py_tuple(vec![py_str("Póso kánei? (πόσο κάνει)"), py_str("POH-so KA-nee")])), (py_str("turkish"), py_tuple(vec![py_str("Ne kadar?"), py_str("neh ka-DAR")])), (py_str("welsh"), py_tuple(vec![py_str("Faint yw hwn?"), py_str("fynt yoo hoon")])), (py_str("japanese"), py_tuple(vec![py_str("Ikura desu ka (いくらですか)"), py_str("ee-KOO-rah dess kah")])), (py_str("chinese"), py_tuple(vec![py_str("Duōshao qián? (多少钱)"), py_str("dwaw-shau chyen")])), (py_str("dutch"), py_tuple(vec![py_str("Hoeveel kost het?"), py_str("hoo-VAYL kost hut")])), (py_str("spanish"), py_tuple(vec![py_str("¿Cuánto cuesta?"), py_str("KWAN-to KWES-ta")])), (py_str("french"), py_tuple(vec![py_str("Combien ça coûte ?"), py_str("kom-BYAN sa KOOT")])), (py_str("italian"), py_tuple(vec![py_str("Quanto costa?"), py_str("KWAN-to KOS-ta")])), (py_str("german"), py_tuple(vec![py_str("Was kostet das?"), py_str("vas KOS-tet das")])), (py_str("welsh"), py_tuple(vec![py_str("Faint yw hyn?"), py_str("fynt yoo hin")])), (py_str("irish"), py_tuple(vec![py_str("Cé mhéad?"), py_str("kay vayd")]))]) }
    fn __data2() -> Py { py_dict(vec![(py_str("toilet"), __data3()), (py_str("thank you"), __data4()), (py_str("please"), __data5()), (py_str("hello"), __data6()), (py_str("goodbye"), __data7()), (py_str("how much"), __data8()), (py_str("the bill"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("La cuenta, por favor"), py_str("la KWEN-ta por fa-VOR")])), (py_str("french"), py_tuple(vec![py_str("L'addition, s'il vous plaît"), py_str("la-dee-SYON seel voo PLEH")])), (py_str("italian"), py_tuple(vec![py_str("Il conto, per favore"), py_str("eel KON-to per fa-VO-reh")])), (py_str("german"), py_tuple(vec![py_str("Die Rechnung, bitte"), py_str("dee REKH-noong BIT-tuh")])), (py_str("welsh"), py_tuple(vec![py_str("Y bil, os gwelwch yn dda"), py_str("uh bill os GWEL-ookh un thah")]))])), (py_str("sorry"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("Perdón"), py_str("per-DON")])), (py_str("french"), py_tuple(vec![py_str("Pardon"), py_str("par-DON")])), (py_str("italian"), py_tuple(vec![py_str("Scusi"), py_str("SKOO-zee")])), (py_str("german"), py_tuple(vec![py_str("Entschuldigung"), py_str("ent-SHOOL-dee-goong")])), (py_str("welsh"), py_tuple(vec![py_str("Mae'n ddrwg gen i"), py_str("mine throog gen ee")])), (py_str("irish"), py_tuple(vec![py_str("Tá brón orm"), py_str("taw brone or-um")]))])), (py_str("i don't understand"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("No entiendo"), py_str("no en-TYEN-do")])), (py_str("french"), py_tuple(vec![py_str("Je ne comprends pas"), py_str("zhuh nuh kom-PRAN pa")])), (py_str("italian"), py_tuple(vec![py_str("Non capisco"), py_str("non ka-PEES-ko")])), (py_str("german"), py_tuple(vec![py_str("Ich verstehe nicht"), py_str("ikh fair-SHTAY-uh nikht")])), (py_str("welsh"), py_tuple(vec![py_str("Dw i ddim yn deall"), py_str("doo ee thim un day-al")])), (py_str("irish"), py_tuple(vec![py_str("Ní thuigim"), py_str("nee HIG-im")]))])), (py_str("do you speak english"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("¿Habla inglés?"), py_str("AB-la een-GLES")])), (py_str("french"), py_tuple(vec![py_str("Parlez-vous anglais ?"), py_str("par-lay VOO on-GLEH")])), (py_str("italian"), py_tuple(vec![py_str("Parla inglese?"), py_str("PAR-la een-GLEH-zeh")])), (py_str("german"), py_tuple(vec![py_str("Sprechen Sie Englisch?"), py_str("SHPREKH-en zee ENG-lish")])), (py_str("welsh"), py_tuple(vec![py_str("Ydych chi'n siarad Saesneg?"), py_str("UD-ikh kheen SHAR-ad SICE-neg")]))])), (py_str("water"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("Agua"), py_str("AH-gwa")])), (py_str("french"), py_tuple(vec![py_str("De l'eau"), py_str("duh loh")])), (py_str("italian"), py_tuple(vec![py_str("Acqua"), py_str("AH-kwa")])), (py_str("german"), py_tuple(vec![py_str("Wasser"), py_str("VAS-ser")])), (py_str("welsh"), py_tuple(vec![py_str("Dŵr"), py_str("door")])), (py_str("irish"), py_tuple(vec![py_str("Uisce"), py_str("ISH-ka")])), (py_str("gaelic"), py_tuple(vec![py_str("Uisge"), py_str("OOSH-ka")]))])), (py_str("help"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("Socorro"), py_str("so-KO-ro")])), (py_str("french"), py_tuple(vec![py_str("Au secours"), py_str("oh suh-KOOR")])), (py_str("italian"), py_tuple(vec![py_str("Aiuto"), py_str("a-YOO-to")])), (py_str("german"), py_tuple(vec![py_str("Hilfe"), py_str("HIL-fuh")])), (py_str("welsh"), py_tuple(vec![py_str("Help!"), py_str("help")])), (py_str("irish"), py_tuple(vec![py_str("Cabhair!"), py_str("KOW-ir")])), (py_str("gaelic"), py_tuple(vec![py_str("Cuideachadh!"), py_str("KOO-jokh-ug")]))])), (py_str("good morning"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("Buenos días"), py_str("BWEH-nos DEE-as")])), (py_str("french"), py_tuple(vec![py_str("Bonjour"), py_str("bon-ZHOOR")])), (py_str("italian"), py_tuple(vec![py_str("Buongiorno"), py_str("bwon-JOR-no")])), (py_str("german"), py_tuple(vec![py_str("Guten Morgen"), py_str("GOO-ten MOR-gen")])), (py_str("welsh"), py_tuple(vec![py_str("Bore da"), py_str("BOR-eh da")])), (py_str("irish"), py_tuple(vec![py_str("Maidin mhaith"), py_str("MA-jin vah")])), (py_str("gaelic"), py_tuple(vec![py_str("Madainn mhath"), py_str("MA-din vah")]))])), (py_str("good night"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("Buenas noches"), py_str("BWEH-nas NO-chess")])), (py_str("french"), py_tuple(vec![py_str("Bonne nuit"), py_str("bon NWEE")])), (py_str("italian"), py_tuple(vec![py_str("Buonanotte"), py_str("bwo-na-NOT-teh")])), (py_str("german"), py_tuple(vec![py_str("Gute Nacht"), py_str("GOO-tuh nakht")])), (py_str("welsh"), py_tuple(vec![py_str("Nos da"), py_str("noss da")])), (py_str("irish"), py_tuple(vec![py_str("Oíche mhaith"), py_str("EE-ha vah")]))])), (py_str("cheers"), py_dict(vec![(py_str("spanish"), py_tuple(vec![py_str("Salud"), py_str("sa-LOOD")])), (py_str("french"), py_tuple(vec![py_str("Santé"), py_str("son-TAY")])), (py_str("italian"), py_tuple(vec![py_str("Salute"), py_str("sa-LOO-teh")])), (py_str("german"), py_tuple(vec![py_str("Prost"), py_str("prohst")])), (py_str("welsh"), py_tuple(vec![py_str("Iechyd da"), py_str("YAKH-id da")])), (py_str("irish"), py_tuple(vec![py_str("Sláinte"), py_str("SLAWN-cha")])), (py_str("gaelic"), py_tuple(vec![py_str("Slàinte mhath"), py_str("SLAWN-cha vah")]))]))]) }
    s_PHRASES(__data2());
    s_WELSH_AND_IRISH(py_dict(vec![(py_str("thank you"), py_dict(vec![(py_str("welsh"), py_tuple(vec![py_str("Diolch"), py_str("DEE-olch")])), (py_str("irish"), py_tuple(vec![py_str("Go raibh maith agat"), py_str("guh rev MAH-ut uh-gut")]))])), (py_str("please"), py_dict(vec![(py_str("welsh"), py_tuple(vec![py_str("Os gwelwch yn dda"), py_str("os GWEL-ookh un thah")])), (py_str("irish"), py_tuple(vec![py_str("Le do thoil"), py_str("leh duh HULL")]))])), (py_str("hello"), py_dict(vec![(py_str("welsh"), py_tuple(vec![py_str("Helo"), py_str("HEH-loh")])), (py_str("irish"), py_tuple(vec![py_str("Dia duit"), py_str("DEE-uh gwit")]))])), (py_str("goodbye"), py_dict(vec![(py_str("welsh"), py_tuple(vec![py_str("Hwyl fawr"), py_str("hoo-il vowr")])), (py_str("irish"), py_tuple(vec![py_str("Slán"), py_str("slawn")]))])), (py_str("sorry"), py_dict(vec![(py_str("welsh"), py_tuple(vec![py_str("Mae'n ddrwg gen i"), py_str("mine throog gen ee")])), (py_str("irish"), py_tuple(vec![py_str("Tá brón orm"), py_str("taw brone or-um")]))])), (py_str("water"), py_dict(vec![(py_str("welsh"), py_tuple(vec![py_str("Dŵr"), py_str("door")])), (py_str("irish"), py_tuple(vec![py_str("Uisce"), py_str("ISH-keh")]))]))]));
    let __iter9 = py_iter_or_raise(&mcall(&g_WELSH_AND_IRISH(), "items", &[]));
    for __item in __iter9 {
    let __unpk10 = py_unpack(&__item.clone(), 2, None);
    s__concept(__unpk10[0].clone());
    s__by_language(__unpk10[1].clone());
    let _ = mcall(&mcall(&g_PHRASES(), "setdefault", &[py_func("_concept", |args: &[Py]| _concept__wrapped(args)), py_dict(vec![])]), "update", &[g__by_language()]);
    }
    fn __data11() -> Py { py_dict(vec![(py_str("spanish"), py_str("Spanish")), (py_str("espanol"), py_str("Spanish")), (py_str("español"), py_str("Spanish")), (py_str("french"), py_str("French")), (py_str("francais"), py_str("French")), (py_str("français"), py_str("French")), (py_str("italian"), py_str("Italian")), (py_str("italiano"), py_str("Italian")), (py_str("german"), py_str("German")), (py_str("deutsch"), py_str("German")), (py_str("welsh"), py_str("Welsh")), (py_str("cymraeg"), py_str("Welsh")), (py_str("wales"), py_str("Welsh")), (py_str("irish"), py_str("Irish")), (py_str("gaeilge"), py_str("Irish")), (py_str("gailge"), py_str("Irish")), (py_str("as gaeilge"), py_str("Irish")), (py_str("gaelic"), py_str("Scottish Gaelic")), (py_str("scottish gaelic"), py_str("Scottish Gaelic")), (py_str("portuguese"), py_str("Portuguese")), (py_str("portugues"), py_str("Portuguese")), (py_str("português"), py_str("Portuguese")), (py_str("greek"), py_str("Greek")), (py_str("turkish"), py_str("Turkish")), (py_str("japanese"), py_str("Japanese")), (py_str("chinese"), py_str("Chinese")), (py_str("mandarin"), py_str("Chinese")), (py_str("dutch"), py_str("Dutch"))]) }
    s_LANGUAGES(__data11());
    fn __data12() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("toilet"), py_str("loo"), py_str("bathroom"), py_str("restroom"), py_str("bog")]), py_str("toilet")]), py_tuple(vec![py_tuple(vec![py_str("thank"), py_str("thanks")]), py_str("thank you")]), py_tuple(vec![py_tuple(vec![py_str("please")]), py_str("please")]), py_tuple(vec![py_tuple(vec![py_str("hello"), py_str("hi")]), py_str("hello")]), py_tuple(vec![py_tuple(vec![py_str("good morning")]), py_str("good morning")]), py_tuple(vec![py_tuple(vec![py_str("good night"), py_str("goodnight")]), py_str("good night")]), py_tuple(vec![py_tuple(vec![py_str("cheers")]), py_str("cheers")]), py_tuple(vec![py_tuple(vec![py_str("goodbye"), py_str("bye")]), py_str("goodbye")]), py_tuple(vec![py_tuple(vec![py_str("how much"), py_str("cost"), py_str("price")]), py_str("how much")]), py_tuple(vec![py_tuple(vec![py_str("bill"), py_str("check"), py_str("cheque")]), py_str("the bill")]), py_tuple(vec![py_tuple(vec![py_str("sorry"), py_str("excuse me")]), py_str("sorry")]), py_tuple(vec![py_tuple(vec![py_str("don't understand"), py_str("dont understand"), py_str("do not understand")]), py_str("i don't understand")]), py_tuple(vec![py_tuple(vec![py_str("speak english"), py_str("english")]), py_str("do you speak english")]), py_tuple(vec![py_tuple(vec![py_str("water")]), py_str("water")]), py_tuple(vec![py_tuple(vec![py_str("help")]), py_str("help")])]) }
    s_CONCEPTS(__data12());
    fn __data13() -> Py { py_tuple(vec![py_str("thank you"), py_str("thanks"), py_str("thank"), py_str("cheers"), py_str("hello"), py_str("hi"), py_str("please"), py_str("goodbye"), py_str("bye"), py_str("sorry"), py_str("excuse me"), py_str("water"), py_str("help"), py_str("toilet"), py_str("loo"), py_str("bathroom"), py_str("good morning"), py_str("good night"), py_str("goodnight"), py_str("how much"), py_str("the bill"), py_str("bill"), py_str("dont understand"), py_str("don't understand"), py_str("do you speak english"), py_str("speak english")]) }
    s_CONCEPT_WORDS(__data13());
    s_NATIVE(py_dict(vec![]));
    let __iter14 = py_iter_or_raise(&mcall(&g_PHRASES(), "items", &[]));
    for __item in __iter14 {
    let __unpk15 = py_unpack(&__item.clone(), 2, None);
    s__concept_name(__unpk15[0].clone());
    s__said_in(__unpk15[1].clone());
    let __iter16 = py_iter_or_raise(&mcall(&g__said_in(), "items", &[]));
    for __item in __iter16 {
    let __unpk17 = py_unpack(&__item.clone(), 2, None);
    s__language(__unpk17[0].clone());
    let __unpk18 = py_unpack(&__unpk17[1].clone(), 2, None);
    s__phrase(__unpk18[0].clone());
    s__say(__unpk18[1].clone());
    let __iter19 = py_iter_or_raise(&py_tuple(vec![g__phrase(), mcall(&g__phrase(), "strip", &[py_str("!?¿")])]));
    for __item in __iter19 {
    s__word(__item.clone());
    s__word(mcall(&mcall(&g__word(), "strip", &[py_str("!?¿.,")]), "lower", &[]));
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[g__word()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__word(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&mcall(&g__concept_name(), "split", &[]), &g__word())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.normalize", "BASE"), &g__word())) } } } } } }) {
    let _ = mcall(&g_NATIVE(), "setdefault", &[g__word(), py_tuple(vec![g__language(), g__concept_name()])]);
    s__plain(mcall(&py_str(""), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("unicodedata", "normalize", &[py_str("NFKD"), g__word()]).unwrap_or_else(|e| exc::raise(e))) { let mut c: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("unicodedata", "combining", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(c.clone()); } } py_list(__out) })() }]));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&g__plain(), &g__word())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.normalize", "BASE"), &g__plain())) } }) {
    let _ = mcall(&g_NATIVE(), "setdefault", &[g__plain(), py_tuple(vec![g__language(), g__concept_name()])]);
    }
    }
    }
    }
    }
    s__concept_name(Py::None);
    s__said_in(Py::None);
    s__language(Py::None);
    s__phrase(Py::None);
    s__say(Py::None);
    s__word(Py::None);
    s__plain(Py::None);
    s__A_MEANING_IS_ASKED(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:does|do|did)\\b[^?.!]{0,40}\\bmean(?:s|ing)?\\b|\\bmean(?:s|ing|ings)?\\b|\\bhow\\s+(?:do|can|would|should|could)\\s+(?:you|i|we)\\s+say\\b|\\btranslat(?:e|ion|ions)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data20() -> Py { py_tuple(vec![py_str("german"), py_str("french"), py_str("spanish"), py_str("italian"), py_str("welsh"), py_str("irish"), py_str("gaelic"), py_str("portuguese"), py_str("greek"), py_str("polish"), py_str("dutch"), py_str("turkish"), py_str("japanese"), py_str("chinese"), py_str("mandarin"), py_str("cantonese"), py_str("arabic"), py_str("russian"), py_str("hindi"), py_str("urdu"), py_str("bengali"), py_str("punjabi"), py_str("somali"), py_str("romanian"), py_str("czech"), py_str("swedish"), py_str("norwegian"), py_str("danish"), py_str("finnish"), py_str("korean"), py_str("thai"), py_str("vietnamese"), py_str("hebrew"), py_str("swahili"), py_str("tagalog"), py_str("ukrainian"), py_str("latin"), py_str("afrikaans"), py_str("albanian"), py_str("bulgarian"), py_str("croatian"), py_str("serbian"), py_str("slovak"), py_str("slovenian"), py_str("estonian"), py_str("latvian"), py_str("lithuanian"), py_str("hungarian"), py_str("icelandic"), py_str("malay"), py_str("indonesian"), py_str("persian"), py_str("farsi"), py_str("kurdish"), py_str("amharic"), py_str("yoruba"), py_str("zulu"), py_str("xhosa"), py_str("creole"), py_str("filipino"), py_str("sign language")]) }
    s_LANGUAGE_NAMES(__data20());
    fn __data21() -> Py { py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("am"), py_str("what"), py_str("whats"), py_str("what's"), py_str("which"), py_str("it"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("in"), py_str("on"), py_str("of"), py_str("to"), py_str("for"), py_str("and"), py_str("or"), py_str("do"), py_str("does"), py_str("did"), py_str("you"), py_str("i"), py_str("we"), py_str("they"), py_str("he"), py_str("she"), py_str("mean"), py_str("means"), py_str("meaning"), py_str("say"), py_str("please"), py_str("so"), py_str("my"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("there"), py_str("here"), py_str("s")]) }
    s__ENGLISH_FURNITURE(call_builtin("frozenset", &[__data21()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data22() -> Py { py_tuple(vec![py_str("essay"), py_str("essays"), py_str("dissertation"), py_str("coursework"), py_str("assignment"), py_str("assignments"), py_str("report"), py_str("reports"), py_str("paragraph"), py_str("paragraphs"), py_str("write"), py_str("writing"), py_str("written"), py_str("thesis"), py_str("paper"), py_str("papers"), py_str("project"), py_str("projects"), py_str("chapter"), py_str("chapters"), py_str("story"), py_str("stories"), py_str("mark"), py_str("marks"), py_str("grade"), py_str("grades"), py_str("tutor"), py_str("teacher"), py_str("lecturer"), py_str("uni"), py_str("university"), py_str("college"), py_str("gcse"), py_str("a level"), py_str("a-level"), py_str("aqa"), py_str("edexcel"), py_str("english"), py_str("history"), py_str("politics"), py_str("sociology"), py_str("psychology"), py_str("biology"), py_str("geography"), py_str("question"), py_str("questions"), py_str("title"), py_str("speech"), py_str("speeches"), py_str("talk"), py_str("talks")]) }
    s__SOMETHING_WRITTEN(__data22());
    s__A_CONCLUSION_SAID(py_tuple(vec![py_str("conclusion"), py_str("conclusions"), py_str("conclude"), py_str("sum up"), py_str("summarise"), py_str("summarize"), py_str("last paragraph"), py_str("final paragraph"), py_str("wrap up")]));
    s__MEANING_FRAME(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+does\\b|\\bwhat'?s\\b|\\bwhat\\s+is\\b|\\bmean(s|ing)?\\b|\\bstand\\s+for\\b|\\btranslate[sd]?\\b|\\btranslation\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKED_OUTRIGHT(stdlib_call("re", "compile", &[py_str("\\bmean(s|ing)?\\b|\\bstand\\s+for\\b|\\btranslate[sd]?\\b|\\bhow\\s+do\\s+you\\s+say\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data23() -> Py { py_tuple(vec![py_str("was"), py_str("die"), py_str("non"), py_str("tot"), py_str("mas"), py_str("nos"), py_str("dia"), py_str("per"), py_str("mar"), py_str("faint"), py_str("bore"), py_str("dank"), py_str("salute"), py_str("sante"), py_str("santé"), py_str("wasser"), py_str("ble")]) }
    s__ENGLISH_TOO(call_builtin("frozenset", &[__data23()]).unwrap_or_else(|e| exc::raise(e)));
    s__ABOUT_THE_PLATE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:food|foods|cuisine|dish|dishes|eat|eating|meals?|restaurants?)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SAY_OR_TRANSLATE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:say|says|saying|pronounce|pronounced|translate|translated|translation|mean|means|how do you say)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PHRASE_ASK(stdlib_call("re", "compile", &[op_modulo(&py_str("\\bhow\\b[^?.!]{0,30}\\b(?:say|pronounce|translate)\\b|\\bsay\\b|\\bpronounce\\b|\\btranslat(?:e|ion|ions)\\b|\\bwords?\\s+for\\b|\\bphrase(?:s)?\\s+for\\b|\\bin\\s+(?:%s)\\b"), &mcall(&py_str("|"), "join", &[g_LANGUAGE_NAMES()])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data24() -> Py { py_tuple(vec![py_str("actually"), py_str("no"), py_str("not"), py_str("sorry"), py_str("im"), py_str("i'm"), py_str("meant"), py_str("mean"), py_str("instead"), py_str("rather"), py_str("then"), py_str("make"), py_str("change"), py_str("changed"), py_str("changing"), py_str("switch"), py_str("switched"), py_str("let"), py_str("lets"), py_str("let's"), py_str("learn"), py_str("learning"), py_str("learnt"), py_str("studying"), py_str("study"), py_str("pick"), py_str("picking"), py_str("choose"), py_str("choosing"), py_str("language"), py_str("languages"), py_str("one"), py_str("ones"), py_str("about"), py_str("also"), py_str("too"), py_str("both"), py_str("them"), py_str("other"), py_str("another"), py_str("re"), py_str("well"), py_str("decide"), py_str("deciding"), py_str("decided")]) }
    s__CORRECTION_WORDS(call_builtin("frozenset", &[__data24()]).unwrap_or_else(|e| exc::raise(e)));
    s__CORRECTION_FURNITURE(op_bitor(&g__ENGLISH_FURNITURE(), &g__CORRECTION_WORDS()));
    fn __data25() -> Py { py_tuple(vec![py_str("heat"), py_str("warm"), py_str("hot"), py_str("cold"), py_str("weather"), py_str("celsius"), py_str("centigrade"), py_str("fahrenheit"), py_str("temperature"), py_str("thermostat"), py_str("fever"), py_str("oven"), py_str("washing machine"), py_str("angle"), py_str("bend"), py_str("slope"), py_str("latitude"), py_str("gas mark"), py_str("convert"), py_str("conversion"), py_str("degrees c"), py_str("degrees f")]) }
    s__A_DEGREE_OF_HEAT(__data25());
    s__TEMPERATURE_DEGREES(stdlib_call("re", "compile", &[py_str("(?<![\\w:.\\d])(?<!\\d\\s)(?:[1-9]|[1-3][0-9]|40)\\s*(?:degrees?\\b|deg\\b|°)")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data26() -> Py { py_tuple(vec![py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("eleven"), py_str("twelve"), py_str("thirteen"), py_str("fourteen"), py_str("fifteen"), py_str("sixteen"), py_str("seventeen"), py_str("eighteen"), py_str("nineteen"), py_str("twenty"), py_str("thirty"), py_str("forty"), py_str("fifty"), py_str("sixty"), py_str("seventy"), py_str("eighty"), py_str("ninety"), py_str("hundred")]) }
    s__NUMBER_WORDS(call_builtin("frozenset", &[__data26()]).unwrap_or_else(|e| exc::raise(e)));
    s__FAILING(call_builtin("frozenset", &[py_tuple(vec![py_str("fail"), py_str("failed"), py_str("failing"), py_str("fails"), py_str("flunk"), py_str("flunked"), py_str("flunking")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data27() -> Py { py_tuple(vec![py_str("exam"), py_str("exams"), py_str("examination"), py_str("gcse"), py_str("gcses"), py_str("a-level"), py_str("a-levels"), py_str("alevel"), py_str("alevels"), py_str("as-level"), py_str("as-levels"), py_str("mock"), py_str("mocks"), py_str("coursework"), py_str("resit"), py_str("resits"), py_str("retake"), py_str("retakes"), py_str("remark"), py_str("remarks"), py_str("module"), py_str("modules"), py_str("degree"), py_str("semester"), py_str("dissertation"), py_str("grade"), py_str("grades"), py_str("uni"), py_str("university"), py_str("college"), py_str("school"), py_str("sixth"), py_str("year"), py_str("paper"), py_str("papers"), py_str("results"), py_str("sats"), py_str("btec"), py_str("ib")]) }
    s__EXAM_IN_THE_WORDS(call_builtin("frozenset", &[__data27()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data28() -> Py { py_tuple(vec![py_str("blood"), py_str("driving"), py_str("pregnant"), py_str("pregnancy"), py_str("swab"), py_str("urine"), py_str("hearing"), py_str("eye"), py_str("eyes"), py_str("sight"), py_str("allergy"), py_str("allergies"), py_str("covid"), py_str("pcr"), py_str("lateral"), py_str("theory"), py_str("smear"), py_str("cv"), py_str("resume"), py_str("résumé"), py_str("cricket"), py_str("cricketer"), py_str("cricketers"), py_str("test match"), py_str("test matches"), py_str("wicket"), py_str("wickets"), py_str("innings"), py_str("bowler"), py_str("bowlers"), py_str("bowling"), py_str("batting"), py_str("batter"), py_str("batsman"), py_str("stumps"), py_str("ashes"), py_str("lbw"), py_str("motor"), py_str("motors"), py_str("engine"), py_str("engines"), py_str("car"), py_str("cars"), py_str("van"), py_str("vans"), py_str("battery"), py_str("batteries"), py_str("lawnmower"), py_str("mower"), py_str("boiler"), py_str("starter motor"), py_str("changing the subject"), py_str("change the subject"), py_str("changed the subject"), py_str("change of subject"), py_str("off topic"), py_str("on a different subject"), py_str("onto a different subject")]) }
    s_NOT_AN_EXAM(__data28());
    fn __data29() -> Py { py_tuple(vec![py_str("thank you note"), py_str("thank-you note"), py_str("thankyou note"), py_str("thanks note"), py_str("apology note"), py_str("apologise note"), py_str("apologize note"), py_str("apologising note"), py_str("apologizing note"), py_str("sorry note"), py_str("condolence note"), py_str("sympathy note"), py_str("get well note"), py_str("welcome note"), py_str("short note"), py_str("little note"), py_str("note saying"), py_str("a note to my"), py_str("note to my"), py_str("note for my"), py_str("note to the"), py_str("note for the"), py_str("note to a"), py_str("note for a")]) }
    s_NOT_A_PERSONS_NOTE(__data29());
    fn __data30() -> Py { py_tuple(vec![py_str("money"), py_str("spending"), py_str("spent"), py_str("spend"), py_str("budget"), py_str("budgets"), py_str("outgoings"), py_str("goes out"), py_str("going out of my account"), py_str("bank"), py_str("bills"), py_str("paycheck"), py_str("payday"), py_str("salary"), py_str("wages"), py_str("direct debit"), py_str("direct debits"), py_str("standing order"), py_str("statement"), py_str("account"), py_str("what i earn"), py_str("savings"), py_str("overdrawn"), py_str("debt"), py_str("debts")]) }
    s_LOSING_TRACK_OF_MONEY(py_tuple(vec![__data30()]));
    s_TEST_AS_A_VERB(stdlib_call("re", "compile", &[py_str("\\btest(?:ed|ing|s)?\\s+(?:if|whether|that|the|a|an|my|your|our|his|her|its|this|these|those|any|some|for|out|with|by)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_AFTER_TEST(stdlib_call("re", "compile", &[py_str("\\btest\\s+(?:a|an|one|two|three|\\d+)\\s+(?:week|weeks|day|days|month|months|year|years|fortnight|hour|hours|minute|minutes)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEARNING_OBJECTS(call_builtin("frozenset", &[mcall(&py_str("knowledge memory learning revise revised revising revision\nexam exams mock mocks vocab vocabulary spelling spellings grammar tenses reading maths math\ngcse alevel a-level subject subjects topic topics syllabus paper comprehension\nknow knows knew remember remembers remembered learnt learned french spanish german italian\nenglish history geography science biology chemistry physics chinese"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASLEEP(py_tuple(vec![py_str("asleep"), py_str("falling asleep"), py_str("sleepy"), py_str("tired"), py_str("drowsy"), py_str("nodding off"), py_str("sleep")]));
    fn __data31() -> Py { py_tuple(vec![py_str("dad"), py_str("dads"), py_str("father"), py_str("mum"), py_str("mums"), py_str("mother"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("spouse"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandmother"), py_str("grandad"), py_str("granddad"), py_str("grandfather"), py_str("brother"), py_str("sister"), py_str("aunt"), py_str("auntie"), py_str("uncle"), py_str("father in law"), py_str("mother in law"), py_str("in laws"), py_str("other half"), py_str("boyfriend"), py_str("girlfriend")]) }
    s__OTHER_PEOPLE(call_builtin("frozenset", &[__data31()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data32() -> Py { py_tuple(vec![py_str("forget"), py_str("forgets"), py_str("forgetting"), py_str("forgot"), py_str("forgotten"), py_str("remember"), py_str("remembers"), py_str("remembering"), py_str("memory"), py_str("recall"), py_str("names"), py_str("name"), py_str("words")]) }
    s__FORGETTING(call_builtin("frozenset", &[__data32()]).unwrap_or_else(|e| exc::raise(e)));
    s_EXAM_NERVES(py_str("Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing."));
    fn __data33() -> Py { py_tuple(vec![py_str("take a break"), py_str("take breaks"), py_str("take my break"), py_str("take a rest"), py_str("a break from"), py_str("break from studying"), py_str("break from revision"), py_str("break from revising"), py_str("breaks from"), py_str("rest from studying"), py_str("how often break"), py_str("how often should i break"), py_str("how long a break"), py_str("how long should i break"), py_str("break between"), py_str("breaks between"), py_str("should i break"), py_str("should i take a break")]) }
    s_REST_ASK(__data33());
    s_NOT_A_START_OF_EXAMS(py_tuple(vec![py_str("running out of time"), py_str("run out of time"), py_str("ran out of time"), py_str("out of time"), py_str("no time left"), py_str("time in the exam"), py_str("time in exams"), py_str("finish the paper")]));
    fn __data34() -> Py { py_tuple(vec![py_str("instructions"), py_str("assembling"), py_str("manual"), py_str("flat pack"), py_str("flatpack"), py_str("ikea"), py_str("rota"), py_str("rotas"), py_str("advert"), py_str("adverts"), py_str("job advert"), py_str("the wording"), py_str("its wording"), py_str("policy document"), py_str("policy wording"), py_str("terms and conditions"), py_str("the invoice"), py_str("an invoice"), py_str("before signing"), py_str("revise the contract"), py_str("revise a contract"), py_str("revise my contract"), py_str("revising the contract"), py_str("revise the lease"), py_str("revise the agreement"), py_str("revise the budget"), py_str("revise my budget"), py_str("revise a budget"), py_str("revise my essay"), py_str("revise the essay"), py_str("revise my cv"), py_str("revise my resume")]) }
    s_NOT_REVISION(__data34());
    fn __data35() -> Py { py_tuple(vec![py_str("language"), py_str("languages"), py_str("spanish"), py_str("french"), py_str("italian"), py_str("german"), py_str("portuguese"), py_str("japanese"), py_str("chinese"), py_str("mandarin"), py_str("cantonese"), py_str("welsh"), py_str("korean"), py_str("russian"), py_str("arabic"), py_str("dutch"), py_str("greek"), py_str("polish"), py_str("turkish"), py_str("urdu"), py_str("punjabi"), py_str("hindi"), py_str("british sign language"), py_str("sign language"), py_str("fluent"), py_str("fluency"), py_str("fluently"), py_str("duolingo"), py_str("bilingual")]) }
    s_LANGUAGE_LEARNING(__data35());
    s_MOCK(py_tuple(vec![py_str("mock"), py_str("mocks"), py_str("practice paper"), py_str("practice papers"), py_str("predicted"), py_str("predicted grade"), py_str("predicted grades")]));
    s_HOW_ASK(py_tuple(vec![py_str("good way"), py_str("best way"), py_str("right way"), py_str("proper way"), py_str("way"), py_str("ways"), py_str("tips"), py_str("tip"), py_str("advice"), py_str("the trick")]));
    s_A_SUBJECT(py_tuple(vec![py_str("maths"), py_str("math"), py_str("mathematics"), py_str("algebra"), py_str("geometry"), py_str("trigonometry"), py_str("physics"), py_str("chemistry")]));
    s_A_MARK_NOT_A_GRADE(py_tuple(vec![py_str("out"), py_str("of"), py_str("than"), py_str("per"), py_str("over"), py_str("under")]));
    fn __data36() -> Py { py_tuple(vec![py_str("marks"), py_str("mark"), py_str("out"), py_str("%"), py_str("percent"), py_str("subjects"), py_str("hours"), py_str("years"), py_str("days"), py_str("weeks"), py_str("months"), py_str("kids"), py_str("children"), py_str("grandchildren"), py_str("questions"), py_str("pages"), py_str("miles"), py_str("minutes"), py_str("times"), py_str("weeks"), py_str("gcse"), py_str("gcses")]) }
    s_COUNTED_AFTER(__data36());
    s_GRADE_NAME(stdlib_call("re", "compile", &[py_str("^(?:a[\\s*-]?stars?|a\\*s?|grades?|gcses?|a[\\s-]?levels?|btecs?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LETTER_GRADES(stdlib_call("re", "compile", &[py_str("^(?:as|bs|cs|ds|es|fs|us)(?:$|\\s+(?:and|in)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_TOP_GRADES(stdlib_call("re", "compile", &[py_str("\\b(?:[2-9]|1[0-2])\\s+a[\\s*-]?stars?\\b|\\b(?:straight|all)\\s+(?:9s|8s)|\\b[2-9]\\s+a\\*s\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SCORE(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,3})\\b[^0-9,]{0,30}?\\b(?:out of|/)\\s*(\\d{1,3})\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__FEELS_WRONGED(stdlib_call("re", "compile", &[py_str("\\bhate[sd]?\\b|\\bhating\\b|doesn'?t like me|does not like me|dont like me|picks? on me|picked on|shouts? at me|\\bunfair\\b|\\bmean\\b|\\bharsh\\b|\\bganging\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__IS_CHILD(stdlib_call("re", "compile", &[py_str("\\bson\\b|\\bdaughter\\b|\\bchild\\b|\\bkids?\\b|\\bstepson\\b|\\bstepdaughter\\b|\\bniece\\b|\\bnephew\\b|\\bgrandson\\b|\\bgranddaughter\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SCHOOL_THINGS(stdlib_call("re", "compile", &[py_str("\\bteacher|\\bclass\\b|\\bclasses\\b|\\blesson|\\bschool|\\bform tutor\\b|\\btutor\\b|\\bhead of year\\b|\\bheadteacher|\\bpupil|\\bhomework|\\bmarks\\b|\\bdetention|\\bexcluded\\b|\\bmaths\\b|\\bmath\\b|\\bmathematics\\b|\\benglish\\b|\\bscience\\b|\\bhistory\\b|\\bgeography\\b|\\bchemistry\\b|\\bphysics\\b|\\bbiology\\b|\\bfrench\\b|\\bspanish\\b|\\bgerman\\b|\\bart\\b|\\bmusic\\b|\\bdrama\\b|\\bcomputing\\b|\\bict\\b|\\bgcses?\\b|\\ba levels?\\b|\\bsats\\b|\\bform\\b|\\bassembly\\b|\\bplayground\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data37() -> Py { py_dict(vec![(py_str("spanish"), Py::Int(600)), (py_str("french"), Py::Int(600)), (py_str("italian"), Py::Int(600)), (py_str("portuguese"), Py::Int(600)), (py_str("dutch"), Py::Int(600)), (py_str("afrikaans"), Py::Int(600)), (py_str("swedish"), Py::Int(600)), (py_str("norwegian"), Py::Int(600)), (py_str("danish"), Py::Int(600)), (py_str("romanian"), Py::Int(600)), (py_str("german"), Py::Int(750)), (py_str("welsh"), Py::Int(900)), (py_str("irish"), Py::Int(900)), (py_str("greek"), Py::Int(1100)), (py_str("polish"), Py::Int(1100)), (py_str("russian"), Py::Int(1100)), (py_str("turkish"), Py::Int(1100)), (py_str("hebrew"), Py::Int(1100)), (py_str("hindi"), Py::Int(1100)), (py_str("urdu"), Py::Int(1100)), (py_str("punjabi"), Py::Int(1100)), (py_str("vietnamese"), Py::Int(1100)), (py_str("thai"), Py::Int(1100)), (py_str("persian"), Py::Int(1100)), (py_str("farsi"), Py::Int(1100)), (py_str("japanese"), Py::Int(2200)), (py_str("chinese"), Py::Int(2200)), (py_str("mandarin"), Py::Int(2200)), (py_str("cantonese"), Py::Int(2200)), (py_str("korean"), Py::Int(2200)), (py_str("arabic"), Py::Int(2200))]) }
    s_LANGUAGE_HOURS(__data37());
    s__FORGET_TO_DO(stdlib_call("re", "compile", &[py_str("\\bforget(?:s|ting)?\\s+to\\s+(?:ring|call|text|reply|post|book|pay|take|drink|eat|feed|walk|charge|water|tidy|clean|put|send|bring|buy|collect|cancel|renew|order|email|message|pick|do|go|get|check|ask|tell|remind|turn|lock|switch|defrost|batch|prep)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data38() -> Py { py_tuple(vec![py_str("read"), py_str("reading"), py_str("book"), py_str("books"), py_str("chapter"), py_str("page"), py_str("pages"), py_str("textbook"), py_str("article"), py_str("paragraph"), py_str("notes"), py_str("revision"), py_str("revise"), py_str("study"), py_str("studying")]) }
    s__THE_PAGE(__data38());
    s__LAST_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:cant|dont|couldnt|doesnt|didnt|did not|do not|cannot)\\s+remember\\s+(?:(?:the|a)\\s+)?last\\s+(?:time|night|day|week|month|year)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__HOW_TO_REMEMBER(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:can|do|should|would|could)?\\s*(?:i|you|we|to)\\s+(?:better\\s+|best\\s+)?(?:remember|memorise|memorize|retain|take in)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLASS_BAND(py_dict(vec![(py_str("first"), Py::Int(70)), (py_str("2:1"), Py::Int(60)), (py_str("2:2"), Py::Int(50)), (py_str("third"), Py::Int(40))]));
    s_CLASS_SAYS(py_dict(vec![(py_str("first"), py_str("a first is a first-class honours degree: a weighted average of 70% or more, the top class, and the one PhD funding usually asks for. Below it, a 2:1 is 60 to 69, a 2:2 is 50 to 59, a third is 40 to 49, and a degree passes at 40% overall.")), (py_str("2:1"), py_str("a 2:1 is an upper second-class honours degree: a weighted average of 60 to 69%, the class most graduate schemes and master's courses ask for. Above it a first is 70% and up, below it a 2:2 is 50 to 59 and a third is 40 to 49, and a degree passes at 40% overall.")), (py_str("2:2"), py_str("a 2:2 is a lower second-class honours degree: a weighted average of 50 to 59%, one band below a 2:1 (60 to 69) and one above a third (40 to 49). A degree passes at 40% overall, and plenty of jobs and master's courses still take a 2:2.")), (py_str("third"), py_str("a third is a third-class honours degree: a weighted average of 40 to 49%, still honours but the lowest class, one band below a 2:2 (50 to 59). A degree passes at 40% overall.")), (py_str("distinction"), py_str("a distinction is the top award on a master's: an average of 70% or more, with a merit at 60 to 69 and a pass at 50 (a bachelor's degree passes at 40%).")), (py_str("merit"), py_str("a merit is the middle award on a master's: an average of 60 to 69%, between a distinction (70 and up) and a pass at 50 (a bachelor's degree passes at 40%)."))]));
    s_CLASS_NAMES(py_tuple(vec![py_tuple(vec![py_str("2:1"), py_tuple(vec![py_str("\\b2\\s*:\\s*1\\b"), py_str("\\b2\\.1\\b"), py_str("\\b2\\s1\\b"), py_str("\\bupper second\\b"), py_str("\\bupper-second\\b")])]), py_tuple(vec![py_str("2:2"), py_tuple(vec![py_str("\\b2\\s*:\\s*2\\b"), py_str("\\b2\\.2\\b"), py_str("\\b2\\s2\\b"), py_str("\\blower second\\b"), py_str("\\blower-second\\b")])]), py_tuple(vec![py_str("first"), py_tuple(vec![py_str("\\bfirst class\\b"), py_str("\\bfirst-class\\b"), py_str("\\b1st class\\b")])]), py_tuple(vec![py_str("third"), py_tuple(vec![py_str("\\bthird class\\b"), py_str("\\bthird-class\\b"), py_str("\\b3rd class\\b")])]), py_tuple(vec![py_str("distinction"), py_tuple(vec![py_str("\\bdistinction\\b")])]), py_tuple(vec![py_str("merit"), py_tuple(vec![py_str("\\bmerit\\b")])])]));
    s_CLASS_ALONE(py_tuple(vec![py_tuple(vec![py_str("first"), stdlib_call("re", "compile", &[py_str("\\b(?:a|the|my|get|got|need|want)\\s+first\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("third"), stdlib_call("re", "compile", &[py_str("\\b(?:a|the|my|get|got|need|want)\\s+third\\b")]).unwrap_or_else(|e| exc::raise(e))])]));
    fn __data39() -> Py { py_tuple(vec![py_str("first year"), py_str("first term"), py_str("first attempt"), py_str("first time"), py_str("first day"), py_str("first week"), py_str("first draft"), py_str("first choice"), py_str("first job"), py_str("first child"), py_str("first baby"), py_str("first home"), py_str("first house"), py_str("first car"), py_str("third year"), py_str("third term"), py_str("third attempt"), py_str("third time"), py_str("third week"), py_str("third child"), py_str("third baby"), py_str("third party")]) }
    s_NOT_THE_CLASS(__data39());
    s_HALF_OF_CLASS(py_tuple(vec![py_tuple(vec![py_str("low"), py_str("\\blow\\s+(?:2\\s*:\\s*1|2\\.1|upper second)\\b"), py_str("2:1"), Py::Int(60), Py::Int(64)]), py_tuple(vec![py_str("high"), py_str("\\bhigh\\s+(?:2\\s*:\\s*1|2\\.1|upper second)\\b"), py_str("2:1"), Py::Int(65), Py::Int(69)]), py_tuple(vec![py_str("low"), py_str("\\blow\\s+(?:2\\s*:\\s*2|2\\.2|lower second)\\b"), py_str("2:2"), Py::Int(50), Py::Int(54)]), py_tuple(vec![py_str("high"), py_str("\\bhigh\\s+(?:2\\s*:\\s*2|2\\.2|lower second)\\b"), py_str("2:2"), Py::Int(55), Py::Int(59)])]));
    s_PASS_MARK(py_str("A degree passes at 40% in most UK universities (50% in Scotland, and 50% for a taught masters), and the class comes from your average, weighted by credits: 70% and up is a first, 60 to 69 a 2:1, 50 to 59 a 2:2 and 40 to 49 a third. Modules usually need 40% to count, a resit is often capped at the pass mark, and some courses need a pass in every year to carry credits forward, so check your course handbook."));
    fn __data41() -> Py { py_tuple(vec![py_str("good idea"), py_str("a good idea"), py_str("worth it"), py_str("worthwhile"), py_str("useful"), py_str("point"), py_str("bad"), py_str("benefit"), py_str("help"), py_str("helps"), py_str("think"), py_str("opinion"), py_str("too much"), py_str("actually"), py_str("effective"), py_str("any good"), py_str("necessary"), py_str("good"), py_str("helps"), py_str("kids"), py_str("children")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("good idea"), py_str("worth it"), py_str("worthwhile"), py_str("useful"), py_str("actually"), py_str("think"), py_str("opinion"), py_str("too much"), py_str("point"), py_str("benefit"), py_str("necessary"), py_str("effective"), py_str("any good"), py_str("helps")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to get"), py_str("get him to"), py_str("get her to"), py_str("get them to"), py_str("wont do"), py_str("won t do"), py_str("refuses"), py_str("refusing"), py_str("refuse"), py_str("every night"), py_str("routine"), py_str("reward"), py_str("chart"), py_str("sticker"), py_str("timetable"), py_str("times tables"), py_str("spelling"), py_str("maths"), py_str("english"), py_str("revision"), py_str("revise"), py_str("exam"), py_str("concentrate"), py_str("focus"), py_str("adhd"), py_str("dyslex"), py_str("teacher"), py_str("school said")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("how"), py_str("do"), py_str("use"), py_str("practice"), py_str("best"), py_str("worth"), py_str("marks"), py_str("when"), py_str("should"), py_str("start"), py_str("starting"), py_str("begin"), py_str("leave"), py_str("left"), py_str("soon"), py_str("many"), py_str("much"), py_str("often")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("remember"), py_str("remembering"), py_str("forget"), py_str("forgetting"), py_str("keep forgetting"), py_str("how"), py_str("best"), py_str("trick"), py_str("tip"), py_str("tips"), py_str("help"), py_str("never"), py_str("cant remember"), py_str("can't remember")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("remember"), py_str("remembering"), py_str("forget"), py_str("forgetting"), py_str("keep forgetting"), py_str("how"), py_str("best"), py_str("trick"), py_str("tip"), py_str("tips"), py_str("help"), py_str("never"), py_str("cant remember"), py_str("can't remember")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("baby"), py_str("babies"), py_str("dog"), py_str("cat"), py_str("puppy"), py_str("kitten"), py_str("company"), py_str("business"), py_str("domain"), py_str("website"), py_str("username"), py_str("password"), py_str("rename"), py_str("change my name"), py_str("middle name"), py_str("surname"), py_str("spell my name"), py_str("write my name"), py_str("sign my name"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("song"), py_str("songs"), py_str("tune"), py_str("book"), py_str("books"), py_str("novel"), py_str("show"), py_str("series"), py_str("actor"), py_str("actress"), py_str("band"), py_str("album"), py_str("name our"), py_str("name the team"), py_str("name the quiz"), py_str("name the club"), py_str("naming our"), py_str("team name"), py_str("team names"), py_str("quiz name"), py_str("quiz names"), py_str("club name"), py_str("club names"), py_str("call our team"), py_str("call the team"), py_str("call the quiz"), py_str("call our quiz"), py_str("call the club"), py_str("call our club"), py_str("needs a name"), py_str("need a name"), py_str("name ideas"), py_str("name for our"), py_str("names for our"), py_str("dementia"), py_str("alzheimers"), py_str("alzheimer's"), py_str("sign of"), py_str("symptom"), py_str("worried"), py_str("worry"), py_str("should i be concerned"), py_str("is it normal")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("explain"), py_str("teach"), py_str("simple"), py_str("simply"), py_str("method"), py_str("child"), py_str("year old"), py_str("works"), py_str("stuck"), py_str("remember"), py_str("steps"), py_str("step by step"), py_str("by hand"), py_str("without a calculator"), py_str("show me"), py_str("work out"), py_str("help"), py_str("do long division"), py_str("long division method"), py_str("learn"), py_str("homework"), py_str("practice"), py_str("practise"), py_str("how do i do"), py_str("how do you do"), py_str("how to do"), py_str("what is"), py_str("whats"), py_str("mean"), py_str("meaning"), py_str("big number"), py_str("big numbers")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("decimal"), py_str("decimals"), py_str("3 by 8"), py_str("as a decimal"), py_str("two decimal places"), py_str("bill"), py_str("bills"), py_str("chores"), py_str("pizza"), py_str("taxi"), py_str("rent"), py_str("custody"), py_str("excel"), py_str("spreadsheet"), py_str("formula"), py_str("assets"), py_str("inheritance"), py_str("profits"), py_str("bonus"), py_str("holiday"), py_str("money between"), py_str("between us"), py_str("dividing up")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("multiply"), py_str("multiplying"), py_str("multiplied"), py_str("times"), py_str("divide"), py_str("dividing"), py_str("divided"), py_str("add"), py_str("adding"), py_str("subtract"), py_str("subtracting"), py_str("plus"), py_str("minus"), py_str("common denominator"), py_str("same denominator"), py_str("simplify"), py_str("cancel down")]) }
    fn __data51() -> Py { py_tuple(vec![py_str("build"), py_str("make"), py_str("model"), py_str("ideas"), py_str("school"), py_str("project"), py_str("science project"), py_str("school project"), py_str("kids"), py_str("child"), py_str("daughter"), py_str("son"), py_str("erupt"), py_str("erupts"), py_str("eruption"), py_str("explain"), py_str("why"), py_str("how")]) }
    fn __data52() -> Py { py_tuple(vec![py_str("timeline"), py_str("gantt"), py_str("project plan"), py_str("deadline"), py_str("schedule"), py_str("sprint"), py_str("client"), py_str("stakeholder"), py_str("at work"), py_str("for work"), py_str("milestone"), py_str("roadmap"), py_str("what makes"), py_str("what causes"), py_str("which makes"), py_str("that makes"), py_str("causes it"), py_str("the cause"), py_str("the reason")]) }
    fn __data53() -> Py { py_tuple(vec![py_str("daughter"), py_str("son"), py_str("child"), py_str("kid"), py_str("kids"), py_str("practise"), py_str("practice"), py_str("help"), py_str("how"), py_str("ready"), py_str("test"), py_str("school"), py_str("ten words"), py_str("prepare")]) }
    fn __data54() -> Py { py_tuple(vec![py_str("my son"), py_str("my daughter"), py_str("my kid"), py_str("my child"), py_str("bedtime"), py_str("learn to read"), py_str("read to me"), py_str("reading age"), py_str("phonics"), py_str("sounding out"), py_str("music"), py_str("sheet music"), py_str("sight read"), py_str("sight-read"), py_str("sight reading")]) }
    fn __data55() -> Py { py_tuple(vec![py_str("my son"), py_str("my daughter"), py_str("my kid"), py_str("my child"), py_str("bedtime"), py_str("learn to read"), py_str("reading age"), py_str("every night"), py_str("refuses"), py_str("reluctant"), py_str("help him"), py_str("help her"), py_str("count as reading"), py_str("counts as reading"), py_str("count as a book"), py_str("reading practice"), py_str("reading practise"), py_str("ok for reading"), py_str("good for reading"), py_str("for reading practice")]) }
    fn __data56() -> Py { py_tuple(vec![py_str("my eyes"), py_str("eyes hurt"), py_str("eyes ache"), py_str("eyes are sore"), py_str("eye strain"), py_str("eyestrain"), py_str("in bed"), py_str("my eyesight"), py_str("tired eyes"), py_str("sore eyes"), py_str("music"), py_str("sheet music"), py_str("sight read"), py_str("sight-read"), py_str("sight reading"), py_str("notation"), py_str("an instrument"), py_str("piano"), py_str("guitar"), py_str("violin")]) }
    fn __data57() -> Py { py_tuple(vec![py_str("comic"), py_str("comics"), py_str("comic book"), py_str("comic books"), py_str("graphic novel"), py_str("graphic novels"), py_str("manga"), py_str("audio book"), py_str("audio books"), py_str("audiobook"), py_str("audiobooks"), py_str("magazine"), py_str("magazines"), py_str("picture book"), py_str("picture books"), py_str("kindle"), py_str("e-reader"), py_str("ereader")]) }
    fn __data58() -> Py { py_tuple(vec![py_str("presentation"), py_str("talk"), py_str("speech"), py_str("pitch"), py_str("slides"), py_str("audience"), py_str("wedding"), py_str("best man"), py_str("bride"), py_str("groom"), py_str("interview"), py_str("assembly"), py_str("meeting"), py_str("conference")]) }
    fn __data59() -> Py { py_tuple(vec![py_str("1"), py_str("2"), py_str("3"), py_str("4"), py_str("5"), py_str("6"), py_str("7"), py_str("8"), py_str("9"), py_str("bad"), py_str("good"), py_str("mean"), py_str("pass"), py_str("passed"), py_str("average"), py_str("standard"), py_str("decent"), py_str("changed"), py_str("compare")]) }
    fn __data60() -> Py { py_tuple(vec![py_str("credit score"), py_str("credit rating"), py_str("credit file"), py_str("credit report"), py_str("credit history"), py_str("piano"), py_str("guitar"), py_str("violin"), py_str("ballet"), py_str("dance"), py_str("swimming"), py_str("abrsm"), py_str("trinity"), py_str("instrument"), py_str("music"), py_str("singing"), py_str("choir"), py_str("theory test"), py_str("driving test"), py_str("football"), py_str("rugby"), py_str("netball"), py_str("cricket"), py_str("hockey"), py_str("team"), py_str("league"), py_str("club"), py_str("martial arts"), py_str("karate"), py_str("judo")]) }
    fn __data61() -> Py { py_tuple(vec![py_str("1"), py_str("2"), py_str("3"), py_str("4"), py_str("5"), py_str("6"), py_str("7"), py_str("8"), py_str("9"), py_str("grade"), py_str("grades"), py_str("bad"), py_str("good"), py_str("mean"), py_str("worried"), py_str("worry"), py_str("failed"), py_str("gutted"), py_str("matter"), py_str("count"), py_str("what should")]) }
    fn __data62() -> Py { py_tuple(vec![py_str("putting off"), py_str("procrastinat"), py_str("get started"), py_str("started"), py_str("start"), py_str("begin"), py_str("behind"), py_str("timetable"), py_str("schedule"), py_str("overwhelmed"), py_str("how"), py_str("tips"), py_str("advice"), py_str("best"), py_str("help"), py_str("should"), py_str("weeks"), py_str("soon"), py_str("coming"), py_str("prepare"), py_str("preparing"), py_str("prep"), py_str("get ready"), py_str("havent revised"), py_str("havent started"), py_str("havent done"), py_str("havent begun"), py_str("not revised"), py_str("not started"), py_str("not begun"), py_str("no revision"), py_str("done nothing"), py_str("nothing yet"), py_str("done none"), py_str("havent touched"), py_str("havent opened"), py_str("left it"), py_str("left everything"), py_str("not done any"), py_str("in two days"), py_str("in 2 days"), py_str("in three days"), py_str("in 3 days"), py_str("in a few days"), py_str("days time"), py_str("days left"), py_str("run out of time"), py_str("no time")]) }
    fn __data63() -> Py { py_tuple(vec![py_str("time of day"), py_str("what time"), py_str("best time"), py_str("better time"), py_str("when is the best"), py_str("at night or"), py_str("or in the morning"), py_str("or in the evening"), py_str("in the morning or"), py_str("morning or night"), py_str("night or morning"), py_str("evening or morning"), py_str("morning or evening")]) }
    fn __data64() -> Py { py_tuple(vec![py_str("remember"), py_str("sticks"), py_str("stays"), py_str("stuck"), py_str("head"), py_str("brain"), py_str("memory"), py_str("memorise"), py_str("memorize"), py_str("reread"), py_str("rereading"), py_str("notes"), py_str("method"), py_str("technique"), py_str("effective"), py_str("how"), py_str("tips"), py_str("best"), py_str("help")]) }
    fn __data65() -> Py { py_tuple(vec![py_str("morning"), py_str("night"), py_str("evening"), py_str("time of day"), py_str("better time"), py_str("best time"), py_str("best time to"), py_str("best time of day"), py_str("good time"), py_str("right time"), py_str("when in the day"), py_str("what time of day"), py_str("what time"), py_str("which time"), py_str("what hour"), py_str("when")]) }
    fn __data66() -> Py { py_tuple(vec![py_str("nervous"), py_str("nerves"), py_str("nerve"), py_str("panic"), py_str("panicking"), py_str("blank"), py_str("anxious"), py_str("freeze"), py_str("freaking"), py_str("scared"), py_str("stress"), py_str("stressed"), py_str("dread"), py_str("dreading"), py_str("worried"), pyrt::xrefs::xget("knowledge.wording", "NERVES_BODY")]) }
    fn __data67() -> Py { py_tuple(vec![py_str("eat"), py_str("eating"), py_str("ate"), py_str("food"), py_str("breakfast"), py_str("lunch"), py_str("drink"), py_str("drinking"), py_str("water"), py_str("snack"), py_str("snacks"), py_str("coffee"), py_str("caffeine"), py_str("hungry"), py_str("sugary"), py_str("energy drink")]) }
    fn __data68() -> Py { py_tuple(vec![py_str("blood"), py_str("diabetes"), py_str("diabetic"), py_str("my diet"), py_str("lose weight"), py_str("thinking of food"), py_str("overeating"), py_str("appetite"), py_str("nausea"), py_str("sick"), py_str("vomit"), py_str("allergic"), py_str("allergy"), py_str("bring water"), py_str("take water"), py_str("into the exam"), py_str("into an exam"), py_str("allowed in"), py_str("bring in"), py_str("take in")]) }
    fn __data69() -> Py { py_tuple(vec![py_str("lecture"), py_str("lectures"), py_str("class"), py_str("lesson"), py_str("better"), py_str("take"), py_str("taking"), py_str("handwriting"), py_str("laptop"), py_str("book"), py_str("books"), py_str("textbook"), py_str("textbooks"), py_str("chapter"), py_str("chapters"), py_str("reading"), py_str("seminar"), py_str("write"), py_str("writing"), py_str("good"), py_str("good notes")]) }
    fn __data70() -> Py { py_tuple(vec![py_str("focus"), py_str("focusing"), py_str("concentrate"), py_str("concentrating"), py_str("concentrated"), py_str("distracted"), py_str("distraction"), py_str("concentration"), py_str("mind wanders"), py_str("wanders"), py_str("zone out"), py_str("zoning out"), py_str("lose track")]) }
    fn __data71() -> Py { py_tuple(vec![py_str("study"), py_str("studying"), py_str("revising"), py_str("revision"), py_str("homework"), py_str("sit down"), py_str("end up"), py_str("tiktok"), py_str("youtube"), py_str("better"), py_str("worse"), py_str("work"), py_str("office"), py_str("job"), py_str("desk"), py_str("task"), py_str("tasks"), py_str("mind"), py_str("wanders"), py_str("keep losing"), py_str("losing track"), py_str("get on with"), py_str("get anything"), py_str("get distracted"), py_str("cant"), py_str("can t"), py_str("read"), py_str("reading"), py_str("book"), py_str("books"), py_str("meeting"), py_str("meetings"), py_str("conversation")]) }
    fn __data72() -> Py { py_tuple(vec![py_str("concentrate"), py_str("concentrating"), py_str("concentration"), py_str("focus"), py_str("focusing"), py_str("focused"), py_str("distracted"), py_str("distracting"), py_str("distraction"), py_str("wanders"), py_str("attention"), py_str("procrastination"), py_str("procrastinating"), py_str("procrastinate")]) }
    fn __data73() -> Py { py_tuple(vec![py_str("how do i focus"), py_str("how can i focus"), py_str("how do i concentrate"), py_str("how can i concentrate"), py_str("how do i stay"), py_str("how do i keep"), py_str("how do i improve"), py_str("how do i pay"), py_str("how do i stop"), py_str("how do i beat"), py_str("better"), py_str("improve"), py_str("improving"), py_str("stay"), py_str("keep"), py_str("cant"), py_str("cannot"), py_str("why"), py_str("help"), py_str("class"), py_str("mind"), py_str("anything"), py_str("beat"), py_str("pay")]) }
    fn __data74() -> Py { py_tuple(vec![py_str("learning"), py_str("learn"), py_str("learnt"), py_str("study"), py_str("picking up"), py_str("memorise"), py_str("memorize"), py_str("memorising"), py_str("memorizing"), py_str("fluent"), py_str("fluency"), py_str("fluently"), py_str("speak"), py_str("speaking")]) }
    fn __data75() -> Py { py_tuple(vec![py_str("language"), py_str("spanish"), py_str("french"), py_str("italian"), py_str("german"), py_str("portuguese"), py_str("japanese"), py_str("chinese"), py_str("welsh"), py_str("korean"), py_str("russian"), py_str("arabic"), py_str("dutch"), py_str("greek"), py_str("polish")]) }
    fn __data76() -> Py { py_tuple(vec![py_str("which language should"), py_str("what language should"), py_str("language should i"), py_str("languages should i"), py_str("what language to"), py_str("which language to"), py_str("what language do i"), py_str("which language do i"), py_str("best language to"), py_str("easiest language to"), py_str("hardest language to"), py_str("good language to"), py_str("useful language to"), py_str("most useful language"), py_str("language worth"), py_str("languages worth"), py_str("which language is"), py_str("what language is"), py_str("pick a language"), py_str("choose a language"), py_str("which language first"), py_str("what language first"), py_str("easiest language"), py_str("hardest language"), py_str("best language")]) }
    fn __data77() -> Py { py_tuple(vec![py_str("learn"), py_str("learning"), py_str("learnt"), py_str("study"), py_str("studying"), py_str("pick"), py_str("picking"), py_str("choose"), py_str("choosing"), py_str("start"), py_str("starting"), py_str("first"), py_str("worth"), py_str("recommend"), py_str("english speaker"), py_str("english speakers")]) }
    fn __data78() -> Py { py_tuple(vec![py_str("say"), py_str("says"), py_str("saying"), py_str("pronounce"), py_str("translate"), py_str("translation"), py_str("phrase"), py_str("phrases"), py_str("speak"), py_str("speaks"), py_str("speaking"), py_str("spoken"), py_str("in spanish"), py_str("in french"), py_str("in italian"), py_str("in german"), py_str("in welsh"), py_str("in irish"), py_str("in gaelic"), py_str("in portuguese"), py_str("sign language"), py_str("duolingo"), py_str("app"), py_str("apps"), py_str("lesson"), py_str("lessons"), py_str("class"), py_str("classes"), py_str("course"), py_str("courses"), py_str("gcse"), py_str("a level"), py_str("a-level"), py_str("degree"), py_str("university"), py_str("abroad"), py_str("moving to"), py_str("live in"), py_str("holiday"), py_str("my child"), py_str("my son"), py_str("my daughter")]) }
    fn __data79() -> Py { py_tuple(vec![py_str("how long"), py_str("how many hours"), py_str("how many years"), py_str("how many months"), py_str("how many weeks"), py_str("how soon"), py_str("how much time"), py_str("how long until"), py_str("how long till"), py_str("how long before"), py_str("until i'm"), py_str("until im"), py_str("to become"), py_str("does it take")]) }
    fn __data80() -> Py { py_tuple(vec![py_str("a week"), py_str("per week"), py_str("each week"), py_str("every week"), py_str("hours a week"), py_str("how often"), py_str("sign language"), py_str("translate"), py_str("translation"), py_str("pronounce"), py_str("pronunciation"), py_str("spell"), py_str("spelling"), py_str("grammar"), py_str("lesson"), py_str("lessons"), py_str("class"), py_str("classes"), py_str("course"), py_str("courses"), py_str("app"), py_str("apps"), py_str("duolingo"), py_str("gcse"), py_str("a level"), py_str("a-level"), py_str("ielts"), py_str("degree"), py_str("university"), py_str("have you been"), py_str("did it take you"), py_str("you been"), py_str("how long have you")]) }
    fn __data81() -> Py { py_tuple(vec![py_str("spanish"), py_str("french"), py_str("italian"), py_str("german"), py_str("espanol"), py_str("español"), py_str("francais"), py_str("français"), py_str("italiano"), py_str("deutsch"), py_str("portuguese"), py_str("portugues"), py_str("português"), py_str("greek"), py_str("turkish"), py_str("japanese"), py_str("chinese"), py_str("mandarin"), py_str("dutch"), py_str("welsh"), py_str("cymraeg"), py_str("irish"), py_str("gaeilge"), py_str("gaelic")]) }
    fn __data82() -> Py { py_tuple(vec![py_str("spanish"), py_str("french"), py_str("italian"), py_str("german"), py_str("espanol"), py_str("español"), py_str("francais"), py_str("français"), py_str("italiano"), py_str("deutsch"), py_str("welsh"), py_str("cymraeg"), py_str("irish"), py_str("gaeilge"), py_str("gaelic")]) }
    fn __data83() -> Py { py_tuple(vec![py_str("whats"), py_str("what is"), py_str("what's"), py_str("how"), py_str("i need"), py_str("translate"), py_str("actually"), py_str("instead"), py_str("make that"), py_str("not"), py_str("no"), py_str("then"), py_str("what about"), py_str("about"), py_str("or"), py_str("rather"), py_str("i mean"), py_str("mean")]) }
    fn __data84() -> Py { py_tuple(vec![py_str("worth"), py_str("going"), py_str("go"), py_str("should"), py_str("apprenticeship"), py_str("debt"), py_str("better"), py_str("need"), py_str("needed"), py_str("necessary"), py_str("alternative"), py_str("alternatives"), py_str("without a degree")]) }
    fn __data85() -> Py { py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("should"), py_str("paid"), py_str("pay"), py_str("pays"), py_str("earn"), py_str("earning"), py_str("money"), py_str("wage"), py_str("wages"), py_str("much"), py_str("better"), py_str("instead"), py_str("uni"), py_str("university"), py_str("degree"), py_str("what"), py_str("whats"), py_str("good")]) }
    fn __data86() -> Py { py_tuple(vec![py_str("apply"), py_str("applying"), py_str("get one"), py_str("get an"), py_str("get a"), py_str("how do i get"), py_str("how to get"), py_str("find one"), py_str("find an"), py_str("where do i"), py_str("search"), py_str("become an"), py_str("become a"), py_str("start one"), py_str("start an"), py_str("vacancies"), py_str("vacancy"), py_str("looking"), py_str("look for"), py_str("start looking")]) }
    fn __data87() -> Py { py_tuple(vec![py_str("didn't"), py_str("didnt"), py_str("bad"), py_str("worse"), py_str("failed"), py_str("disappointed"), py_str("resit"), py_str("clearing"), py_str("not what"), py_str("missed"), py_str("offer"), py_str("rejected"), py_str("options"), py_str("now what")]) }
    fn __data88() -> Py { py_tuple(vec![py_str("study"), py_str("studying"), py_str("revising"), py_str("revision"), py_str("homework"), py_str("essay"), py_str("uni"), py_str("university"), py_str("college"), py_str("school"), py_str("coursework"), py_str("dissertation"), py_str("thesis"), py_str("assignment"), py_str("reading"), py_str("deadline"), py_str("stop")]) }
    fn __data89() -> Py { py_tuple(vec![py_str("revision"), py_str("revise"), py_str("revising"), py_str("exam"), py_str("exams"), py_str("subject"), py_str("subjects"), py_str("study"), py_str("studying"), py_str("gcse"), py_str("gcses"), py_str("a-level"), py_str("a levels"), py_str("mocks")]) }
    fn __data90() -> Py { py_tuple(vec![py_str("how long"), py_str("how many hours"), py_str("hours"), py_str("long"), py_str("blocks"), py_str("pomodoro"), py_str("breaks"), py_str("break"), py_str("take a break"), py_str("take breaks"), py_str("a break from"), py_str("break from"), py_str("break between"), py_str("breaks between"), py_str("how often break"), py_str("how often should i break"), py_str("how long a break"), py_str("how long should i break")]) }
    fn __data91() -> Py { py_tuple(vec![py_str("wont"), py_str("won't"), py_str("will not"), py_str("refuses"), py_str("refusing"), py_str("hates"), py_str("hate"), py_str("battle"), py_str("fight"), py_str("fights"), py_str("fighting"), py_str("nightmare"), py_str("sit down"), py_str("every night"), py_str("help"), py_str("struggling"), py_str("struggles"), py_str("routine"), py_str("table")]) }
    fn __data92() -> Py { py_tuple(vec![py_str("revise"), py_str("revising"), py_str("revision"), py_str("study"), py_str("studying"), py_str("learn"), py_str("learned"), py_str("learnt"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("school"), py_str("college"), py_str("uni"), py_str("university"), py_str("homework"), py_str("re-read"), py_str("reread")]) }
    fn __data93() -> Py { py_tuple(vec![py_str("english"), py_str("english literature"), py_str("english language"), py_str("english lit"), py_str("history"), py_str("geography"), py_str("psychology"), py_str("sociology"), py_str("economics"), py_str("business studies"), py_str("religious studies"), py_str("politics"), py_str("media studies"), py_str("classics"), py_str("law a level")]) }
    fn __data94() -> Py { py_tuple(vec![py_str("revise"), py_str("revising"), py_str("revision"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("failing"), py_str("fail"), py_str("failed"), py_str("behind"), py_str("bad at"), py_str("rubbish at"), py_str("struggling"), py_str("stuck"), py_str("how"), py_str("better"), py_str("improve"), py_str("improving"), py_str("marks"), py_str("grade"), py_str("grades")]) }
    fn __data95() -> Py { py_tuple(vec![py_str("revise"), py_str("revising"), py_str("revision"), py_str("exam"), py_str("exams"), py_str("study"), py_str("studying"), py_str("learn"), py_str("school"), py_str("college"), py_str("a level"), py_str("a levels"), py_str("gcse"), py_str("gcses"), py_str("test"), py_str("tests"), py_str("how"), py_str("better"), py_str("failing"), py_str("fail"), py_str("behind"), py_str("help"), py_str("tips")]) }
    fn __data96() -> Py { py_tuple(vec![py_str("browser"), py_str("browser history"), py_str("search history"), py_str("watch history"), py_str("clear my"), py_str("delete my"), py_str("cookies"), py_str("cache"), py_str("incognito"), py_str("medical history"), py_str("family history"), py_str("ancestry"), py_str("genealogy"), py_str("ancestors"), py_str("my gp"), py_str("symptoms"), py_str("speak english"), py_str("speaking english"), py_str("pronounce"), py_str("pronunciation"), py_str("accent"), py_str("learn english"), py_str("improve my english"), py_str("fluent"), py_str("english speaking"), py_str("native speaker"), py_str("another language")]) }
    fn __data97() -> Py { py_tuple(vec![py_str("revise"), py_str("revising"), py_str("revision"), py_str("exam"), py_str("exams"), py_str("test"), py_str("learn"), py_str("practise"), py_str("practice"), py_str("marks"), py_str("better"), py_str("how"), py_str("failing"), py_str("fail"), py_str("failed"), py_str("fails"), py_str("struggling"), py_str("struggle"), py_str("behind"), py_str("bad at"), py_str("rubbish at"), py_str("slipping"), py_str("stuck")]) }
    fn __data98() -> Py { py_tuple(vec![py_str("cant sleep"), py_str("can't sleep"), py_str("insomnia"), py_str("wont wake"), py_str("wakes me"), py_str("nap"), py_str("tired all"), py_str("cant get up"), py_str("can't get up"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("pet"), py_str("pets"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("toddlers"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("year old"), py_str("year olds"), py_str("year-old"), py_str("yr old"), py_str("yrs old"), py_str("time of day"), py_str("better time"), py_str("best time"), py_str("morning or"), py_str("evening or"), py_str("or evening"), py_str("or night"), py_str("or morning"), py_str("which is better"), py_str("what time of day"), py_str("better at night"), py_str("better in the morning"), py_str("better in the evening"), py_str("better in the day"), py_str("study better"), py_str("revise better"), py_str("really better"), py_str("at night or"), py_str("or in the morning"), py_str("or in the evening"), py_str("or in the day"), py_str("in the morning or"), py_str("in the evening or"), py_str("night or morning"), py_str("evening or morning"), py_str("morning or night"), py_str("morning or in"), py_str("sharper"), py_str("feel"), py_str("feeling"), py_str("feels"), py_str("felt"), py_str("sick"), py_str("nauseous"), py_str("nausea"), py_str("anxious"), py_str("anxiety"), py_str("panic"), py_str("panicky"), py_str("dread"), py_str("dreading"), py_str("exhausted"), py_str("tired"), py_str("stress"), py_str("stressed"), py_str("worry"), py_str("worried"), py_str("unhappy"), py_str("depressed"), py_str("miserable"), py_str("low mood")]) }
    fn __data99() -> Py { py_tuple(vec![py_str("opening line"), py_str("opening lines"), py_str("first line"), py_str("first words"), py_str("opener"), py_str("openers"), py_str("icebreaker"), py_str("ice breaker"), py_str("one liner"), py_str("one-liner"), py_str("hook"), py_str("open my speech"), py_str("open a speech"), py_str("open my talk"), py_str("open a talk"), py_str("open my presentation"), py_str("open a presentation")]) }
    fn __data100() -> Py { py_tuple(vec![py_str("speech"), py_str("talk"), py_str("presentation"), py_str("toast"), py_str("audience"), py_str("room"), py_str("stage"), py_str("best man"), py_str("maid of honour"), py_str("wedding"), py_str("assembly"), py_str("work do"), py_str("leaving do")]) }
    fn __data101() -> Py { py_tuple(vec![py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("kids"), py_str("children"), py_str("teenager"), py_str("wont"), py_str("won't"), py_str("refuses"), py_str("refusing"), py_str("refused"), py_str("battle"), py_str("battles"), py_str("nightmare"), py_str("help"), py_str("struggling"), py_str("struggles"), py_str("fights"), py_str("every night"), py_str("year old"), py_str("years old")]) }
    fn __data102() -> Py { py_tuple(vec![py_str("wont"), py_str("won't"), py_str("refuses"), py_str("refusing"), py_str("refused"), py_str("hates"), py_str("battle"), py_str("battles"), py_str("fights"), py_str("fighting"), py_str("nightmare"), py_str("struggling"), py_str("struggles"), py_str("every night"), py_str("will not"), py_str("wont sit"), py_str("will not sit"), py_str("sit down"), py_str("get him to"), py_str("get her to"), py_str("get them to"), py_str("get my"), py_str("hates homework"), py_str("hate homework"), py_str("how do i help"), py_str("how can i help"), py_str("how to help"), py_str("how do i get him"), py_str("how do i get her"), py_str("what should i do about")]) }
    fn __data103() -> Py { py_tuple(vec![py_str("ok il try"), py_str("ok i'll try"), py_str("ill try that"), py_str("i'll try that"), py_str("i will try that"), py_str("ok thanks"), py_str("thanks that helps"), py_str("that helps"), py_str("will do"), py_str("ok will do"), py_str("sounds good"), py_str("okay i will"), py_str("ok i will")]) }
    fn __data104() -> Py { py_tuple(vec![py_str("essay"), py_str("book"), py_str("article"), py_str("website"), py_str("harvard"), py_str("style"), py_str("write"), py_str("list"), py_str("how"), py_str("dissertation"), py_str("assignment"), py_str("coursework"), py_str("sources")]) }
    fn __data105() -> Py { py_tuple(vec![py_str("essay"), py_str("assignment"), py_str("coursework"), py_str("dissertation"), py_str("university"), py_str("college"), py_str("school"), py_str("harvard"), py_str("apa"), py_str("mla"), py_str("book"), py_str("website"), py_str("article"), py_str("sources")]) }
    fn __data106() -> Py { py_tuple(vec![py_str("piano"), py_str("guitar"), py_str("violin"), py_str("instrument"), py_str("instruments"), py_str("keyboard"), py_str("drums"), py_str("clarinet"), py_str("flute"), py_str("saxophone"), py_str("sax"), py_str("cello"), py_str("trumpet"), py_str("ukulele"), py_str("banjo"), py_str("recorder"), py_str("harmonica"), py_str("trombone"), py_str("viola"), py_str("double bass"), py_str("oboe"), py_str("bassoon"), py_str("harp")]) }
    fn __data107() -> Py { py_tuple(vec![py_str("evening course"), py_str("evening courses"), py_str("evening class"), py_str("evening classes"), py_str("night class"), py_str("night classes"), py_str("adult education"), py_str("adult course"), py_str("adult courses"), py_str("adult learning"), py_str("part time course"), py_str("part-time course"), py_str("part time degree"), py_str("part-time degree"), py_str("access course"), py_str("access to higher education"), py_str("college course"), py_str("community course"), py_str("evening college")]) }
    fn __data108() -> Py { py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("hours"), py_str("how much work"), py_str("how much time"), py_str("how long"), py_str("start"), py_str("starting"), py_str("enrol"), py_str("enroll"), py_str("sign up"), py_str("beginner"), py_str("level"), py_str("fees"), py_str("fee"), py_str("cost"), py_str("commitment"), py_str("a week"), py_str("per week"), py_str("workload"), py_str("homework"), py_str("fit"), py_str("manage"), py_str("should i"), py_str("shall i"), py_str("can i"), py_str("do i need"), py_str("is it")]) }
    fn __data109() -> Py { py_tuple(vec![py_str("course"), py_str("courses"), py_str("class"), py_str("classes"), py_str("college"), py_str("adult"), py_str("evening"), py_str("night"), py_str("worth"), py_str("hours"), py_str("workload"), py_str("should i"), py_str("shall i"), py_str("is it worth")]) }
    fn __data110() -> Py { py_tuple(vec![py_str("essay"), py_str("essays"), py_str("coursework"), py_str("assignment"), py_str("dissertation"), py_str("exam"), py_str("exams"), py_str("revision"), py_str("revise"), py_str("boil"), py_str("oven"), py_str("swim"), py_str("swimming"), py_str("running"), py_str("race"), py_str("golf"), py_str("dinner"), py_str("meal"), py_str("online course"), py_str("free course online")]) }
    fn __data111() -> Py { py_tuple(vec![py_str("start"), py_str("starting"), py_str("beginner"), py_str("first"), py_str("learn"), py_str("learning"), py_str("app"), py_str("apps"), py_str("lessons"), py_str("teacher"), py_str("cheap"), py_str("buy"), py_str("practice"), py_str("practise"), py_str("good way"), py_str("how do i")]) }
    fn __data112() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("which"), py_str("should"), py_str("is it"), py_str("is a"), py_str("is an"), py_str("any"), py_str("tip"), py_str("tips"), py_str("advice"), py_str("best"), py_str("good way"), py_str("worth"), py_str("start"), py_str("beginner"), py_str("cheap"), py_str("cost")]) }
    fn __data113() -> Py { py_tuple(vec![py_str("improve"), py_str("better"), py_str("messy"), py_str("neat"), py_str("neater"), py_str("legible"), py_str("fix"), py_str("practice"), py_str("practise"), py_str("slow"), py_str("read"), py_str("help"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("awful"), py_str("terrible"), py_str("bad"), py_str("poor"), py_str("school")]) }
    fn __data114() -> Py { py_tuple(vec![py_str("school bag"), py_str("schoolbag"), py_str("book bag"), py_str("backpack"), py_str("rucksack"), py_str("bag for school"), py_str("bag ready for school"), py_str("bag tidy"), py_str("keep my bag"), py_str("my bag tidy"), py_str("my bag is a mess"), py_str("bag is a mess"), py_str("in my bag"), py_str("my bag is always"), py_str("pack my bag"), py_str("packing my bag"), py_str("my bag packed"), py_str("organise my bag"), py_str("organize my bag"), py_str("sort my bag"), py_str("sorting my bag")]) }
    fn __data115() -> Py { py_tuple(vec![py_str("tidy"), py_str("tidy up"), py_str("keep tidy"), py_str("organise"), py_str("organised"), py_str("organize"), py_str("mess"), py_str("messy"), py_str("lost"), py_str("losing"), py_str("forget"), py_str("forgot"), py_str("pack"), py_str("packing"), py_str("everything")]) }
    fn __data116() -> Py { py_tuple(vec![py_str("ask for help"), py_str("struggling"), py_str("behind"), py_str("help"), py_str("fail"), py_str("talk"), py_str("email"), py_str("home"), py_str("dont get it"), py_str("don't get it"), py_str("shy"), py_str("scared"), py_str("nervous"), py_str("afraid"), py_str("embarrassed"), py_str("dont understand"), py_str("don't understand"), py_str("confused"), py_str("stuck"), py_str("ask"), py_str("asked"), py_str("asking"), py_str("a question"), py_str("questions"), py_str("the question"), py_str("message"), py_str("send an email"), py_str("get help"), py_str("extra help"), py_str("more help"), py_str("hand up"), py_str("raise my hand"), py_str("raised hand"), py_str("wasnt picked"), py_str("wasn't picked"), py_str("never get picked"), py_str("not picked"), py_str("answer in class"), py_str("answering in class"), py_str("speak up in class"), py_str("one to one"), py_str("reply"), py_str("behind on"), py_str("ask for feedback"), py_str("asking for feedback"), py_str("for feedback"), py_str("what to improve")]) }
    fn __data117() -> Py { py_tuple(vec![py_str("take notes"), py_str("taking notes"), py_str("make notes"), py_str("notes in class"), py_str("note taking"), py_str("notes"), py_str("focus"), py_str("focusing"), py_str("pay attention"), py_str("paying attention"), py_str("attention"), py_str("concentrate"), py_str("concentrating"), py_str("concentration"), py_str("listen better"), py_str("revise"), py_str("revising"), py_str("revision"), py_str("revising for"), py_str("exam"), py_str("exams"), py_str("mock"), py_str("mock exam"), py_str("marks"), py_str("mark scheme"), py_str("stop talking"), py_str("talking in class"), py_str("chatting"), py_str("chat"), py_str("distracting"), py_str("call out"), py_str("calling out"), py_str("shouting out"), py_str("loud in class"), py_str("noisy class"), py_str("emailed me"), py_str("email me"), py_str("emailing me"), py_str("rang me"), py_str("called me"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("my kids"), py_str("his teacher"), py_str("her teacher"), py_str("their teacher"), py_str("parents evening"), py_str("parent evening"), py_str("parent teacher"), py_str("parent-teacher"), py_str("head teacher"), py_str("headteacher"), py_str("my boy"), py_str("my girl")]) }
    fn __data118() -> Py { py_tuple(vec![py_str("cost"), py_str("costs"), py_str("how much"), py_str("fees"), py_str("fee"), py_str("tuition"), py_str("price"), py_str("expensive"), py_str("afford"), py_str("worth it"), py_str("worth"), py_str("loan"), py_str("bursary"), py_str("scholarship"), py_str("funding"), py_str("save up"), py_str("part time"), py_str("part-time")]) }
    fn __data119() -> Py { py_tuple(vec![py_str("personal statement"), py_str("statement of purpose"), py_str("cover letter"), py_str("essay"), py_str("undergraduate"), py_str("bachelors"), py_str("bachelor"), py_str("a level"), py_str("a-level"), py_str("student loan"), py_str("maintenance loan"), py_str("rent"), py_str("deposit")]) }
    fn __data120() -> Py { py_tuple(vec![py_str("how long"), py_str("how many words"), py_str("how many characters"), py_str("word count"), py_str("characters"), py_str("maximum length"), py_str("how many lines"), py_str("should i go"), py_str("shall i go"), py_str("is it worth"), py_str("worth it"), py_str("worth going"), py_str("do i need"), py_str("not worth"), py_str("alternatives"), py_str("if i go"), py_str("whether to go")]) }
    fn __data121() -> Py { py_tuple(vec![py_str("helping my"), py_str("my son"), py_str("my daughter"), py_str("my kid"), py_str("my child"), py_str("my children"), py_str("wont do"), py_str("won't do"), py_str("refuses"), py_str("battle"), py_str("every night"), py_str("nagging"), py_str("homework-child")]) }
    fn __data122() -> Py { py_tuple(vec![py_str("stop talking"), py_str("talking too much"), py_str("talk too much"), py_str("chatting"), py_str("chatty"), py_str("chatter"), py_str("chatterbox"), py_str("blurt"), py_str("blurting"), py_str("keep talking"), py_str("distracting"), py_str("nobody talks"), py_str("no one talks"), py_str("nobody speaks"), py_str("no one speaks"), py_str("talks to me"), py_str("speaks to me"), py_str("talk to me"), py_str("speak to me"), py_str("nobody will talk"), py_str("no one will talk")]) }
    fn __data123() -> Py { py_tuple(vec![py_str("class"), py_str("classes"), py_str("classroom"), py_str("lesson"), py_str("lessons"), py_str("school"), py_str("teacher"), py_str("lecture"), py_str("lectures"), py_str("seminar"), py_str("seminars"), py_str("assembly"), py_str("assemblies"), py_str("tutor"), py_str("tutors"), py_str("tutorial"), py_str("tutorials"), py_str("playground"), py_str("playgrounds")]) }
    fn __data124() -> Py { py_tuple(vec![py_str("stop talking"), py_str("talk too much"), py_str("talking too much"), py_str("talk to much"), py_str("chat too much"), py_str("chatting"), py_str("chatty"), py_str("chatter"), py_str("chatterbox"), py_str("blurt"), py_str("blurting"), py_str("keep talking"), py_str("quiet"), py_str("quieter"), py_str("distract"), py_str("distracting"), py_str("shouting out"), py_str("too loud")]) }
    fn __data125() -> Py { py_tuple(vec![py_str("should"), py_str("worth"), py_str("take"), py_str("better"), py_str("do"), py_str("plan"), py_str("uni"), py_str("university"), py_str("offer"), py_str("money"), py_str("work"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("mean"), py_str("means"), py_str("definition"), py_str("define"), py_str("explain")]) }
    fn __data126() -> Py { py_tuple(vec![py_str("marked me down"), py_str("marks me down"), py_str("marking me down"), py_str("marks down"), py_str("low marks"), py_str("bad marks"), py_str("marks are low"), py_str("marked down"), py_str("unfair mark"), py_str("unfair marks"), py_str("marks dont make sense"), py_str("dont understand my marks"), py_str("teacher marks")]) }
    fn __data127() -> Py { py_tuple(vec![py_str("remember what i read"), py_str("remember what i've read"), py_str("remember a book"), py_str("remember a chapter"), py_str("remember reading"), py_str("take notes from a book"), py_str("make notes from a book"), py_str("read and remember"), py_str("reading for study"), py_str("how do i remember"), py_str("retain"), py_str("take in"), py_str("takes in"), py_str("sink in"), py_str("sinks in"), py_str("sticks"), py_str("cant remember what i read"), py_str("can't remember what i read"), py_str("dont remember what i read"), py_str("don't remember what i read"), py_str("forget what i read"), py_str("nothing i read"), py_str("cant remember"), py_str("can't remember"), py_str("dont remember"), py_str("don't remember"), py_str("remember any of it"), py_str("forget"), py_str("forgetting"), py_str("forgot")]) }
    fn __data128() -> Py { py_tuple(vec![py_str("read"), py_str("reading"), py_str("book"), py_str("chapter"), py_str("textbook"), py_str("notes"), py_str("remember"), py_str("memory"), py_str("revision"), py_str("revise"), py_str("study"), py_str("keep"), py_str("how"), py_str("page"), py_str("pages"), py_str("article"), py_str("paragraph")]) }
    fn __data129() -> Py { py_tuple(vec![py_str("phone number"), py_str("pin"), py_str("password"), py_str("birthday"), py_str("name of"), py_str("surname"), py_str("my ex"), py_str("get over"), py_str("move on"), py_str("grief"), py_str("grieving"), py_str("bereaved"), py_str("died"), py_str("death"), py_str("passed away"), py_str("him"), py_str("her"), py_str("someone"), py_str("somebody"), py_str("breakup"), py_str("break up"), py_str("affair"), py_str("cheated"), py_str("how to"), py_str("names"), py_str("anniversary"), py_str("word"), py_str("spelling"), py_str("where i put"), py_str("where i left"), py_str("appointment"), py_str("medication"), py_str("tablet"), py_str("keys"), py_str("glasses"), py_str("wallet"), py_str("barbecue"), py_str("barbecues"), py_str("bbq"), py_str("barbeque"), py_str("grill"), py_str("grilling"), py_str("pan"), py_str("pans"), py_str("wok"), py_str("pasta"), py_str("rice"), py_str("dough"), py_str("pastry"), py_str("meat"), py_str("chicken"), py_str("fish"), py_str("pancake"), py_str("pancakes"), py_str("crepe"), py_str("crepes"), py_str("batter"), py_str("waffle"), py_str("waffles"), py_str("yorkshire"), py_str("yorkshires"), py_str("yorkshire pudding"), py_str("pikelet"), py_str("pikelets"), py_str("cooking"), py_str("cook"), py_str("recipe"), py_str("oven"), py_str("sauce"), py_str("gravy"), py_str("soup"), py_str("bread"), py_str("loaf"), py_str("loaves"), py_str("cake"), py_str("cakes"), py_str("baking"), py_str("bake"), py_str("batter"), py_str("sponge"), py_str("souffle"), py_str("risen"), py_str("curtain"), py_str("curtains"), py_str("shower"), py_str("shower curtain"), py_str("screen protector"), py_str("phone"), py_str("laptop"), py_str("phone screen"), py_str("cling film"), py_str("bin bag"), py_str("zip"), py_str("drawer"), py_str("door"), py_str("window"), py_str("sticker"), py_str("stickers"), py_str("tape"), py_str("plaster"), py_str("saucepan"), py_str("frying pan"), py_str("pasta"), py_str("rice"), py_str("pan"), py_str("pans"), py_str("dough"), py_str("pastry"), py_str("our plans"), py_str("our dates"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("marriage"), py_str("subscription"), py_str("subscriptions"), py_str("cancel"), py_str("cancelling"), py_str("canceling"), py_str("direct debit"), py_str("insurance"), py_str("bill"), py_str("bills"), py_str("renewal"), py_str("renew"), py_str("payment"), py_str("payments"), py_str("policy"), py_str("make it stick"), py_str("make it stick this time"), py_str("stick with it"), py_str("stick at it"), py_str("keep at it"), py_str("keep it up"), py_str("keep going")]) }
    fn __data130() -> Py { py_tuple(vec![py_str("fail my exams"), py_str("failed my exams"), py_str("fail my exam"), py_str("failed my exam"), py_str("fail my a levels"), py_str("failed my a levels"), py_str("fail my gcses"), py_str("failed my gcses"), py_str("fail the year"), py_str("failed the year"), py_str("failed my degree"), py_str("fail my degree"), py_str("did badly in my exams"), py_str("did badly in my exam"), py_str("flunked"), py_str("i failed"), py_str("get a remark"), py_str("ask for a remark"), py_str("remark my exam"), py_str("a remark")]) }
    fn __data131() -> Py { py_tuple(vec![py_str("what"), py_str("should"), py_str("do"), py_str("now"), py_str("next"), py_str("resit"), py_str("retake"), py_str("remark"), py_str("appeal"), py_str("university"), py_str("college"), py_str("sixth form"), py_str("year"), py_str("help"), py_str("i"), py_str("my")]) }
    fn __data132() -> Py { py_tuple(vec![py_str("difference"), py_str("better"), py_str("worth"), py_str("should"), py_str("which"), py_str("or"), py_str("need"), py_str("instead of"), py_str("what"), py_str("why"), py_str("choose"), py_str("decide"), py_str("go to"), py_str("matter")]) }
    fn __data133() -> Py { py_tuple(vec![py_str("conclusion"), py_str("conclusions"), py_str("conclude"), py_str("ending"), py_str("sum up"), py_str("summarise"), py_str("summarize"), py_str("wrap up"), py_str("last paragraph"), py_str("final paragraph"), py_str("strong conclusion"), py_str("good conclusion"), py_str("essay conclusion"), py_str("write a conclusion"), py_str("conclusion for"), py_str("conclusion of"), py_str("conclusion to")]) }
    fn __data134() -> Py { py_tuple(vec![py_str("essay"), py_str("essays"), py_str("dissertation"), py_str("coursework"), py_str("assignment"), py_str("report"), py_str("write"), py_str("writing"), py_str("how"), py_str("start"), py_str("phrases"), py_str("words"), py_str("should"), py_str("what"), py_str("need"), py_str("good"), py_str("strong"), py_str("help")]) }
    fn __data135() -> Py { py_tuple(vec![py_str("i came to the conclusion"), py_str("came to the conclusion"), py_str("test"), py_str("experiment"), py_str("war"), py_str("wars"), py_str("battle"), py_str("battles"), py_str("revolution"), py_str("armistice"), py_str("trench"), py_str("trenches"), py_str("empire"), py_str("1914"), py_str("1918"), py_str("1939"), py_str("1945"), py_str("chat"), py_str("chats"), py_str("chatting"), py_str("conversation"), py_str("conversations"), py_str("phone call"), py_str("phone calls"), py_str("party"), py_str("parties"), py_str("relationship"), py_str("tenancy"), py_str("lease"), py_str("contract"), py_str("subscription"), py_str("argument"), py_str("friendship"), py_str("end things"), py_str("boyfriend"), py_str("girlfriend"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("break up"), py_str("breaking up"), py_str("divorce"), py_str("marriage"), py_str("my marriage"), py_str("fiance"), py_str("fiancee"), py_str("lab report"), py_str("results"), py_str("data"), py_str("conclusion of my"), py_str("doctor"), py_str("gp"), py_str("diagnosis"), py_str("end up"), py_str("ended up"), py_str("ends up"), py_str("ending up"), py_str("homework"), py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kids"), py_str("kid"), py_str("maths"), py_str("times tables"), py_str("end of life"), py_str("end-of-life"), py_str("life care"), py_str("dying"), py_str("died"), py_str("dies"), py_str("dying"), py_str("palliative"), py_str("hospice"), py_str("hospices"), py_str("terminally"), py_str("terminal"), py_str("bereavement"), py_str("grief"), py_str("funeral"), py_str("my mum is"), py_str("my dad is"), py_str("my mother is"), py_str("my father is"), py_str("end a fast"), py_str("end my fast"), py_str("end the fast"), py_str("end this fast"), py_str("end a diet"), py_str("end my diet"), py_str("end the diet"), py_str("end a poem"), py_str("end my poem"), py_str("end the poem"), py_str("end a verse"), py_str("end a sentence"), py_str("end my sentence"), py_str("end a paragraph"), py_str("end my paragraph"), py_str("end a chapter"), py_str("end my chapter"), py_str("end a book"), py_str("end my book"), py_str("end a story"), py_str("end my story"), py_str("end a novel"), py_str("end my novel"), py_str("interview"), py_str("interviews"), py_str("interviewing"), py_str("interviewer"), py_str("job interview"), py_str("panel"), py_str("appraisal"), py_str("one to one"), py_str("1:1"), py_str("end a meeting"), py_str("end the meeting"), py_str("end my meeting"), py_str("meeting"), py_str("end a shift"), py_str("end my shift"), py_str("end the shift"), py_str("end a call"), py_str("end the call")]) }
    fn __data136() -> Py { py_tuple(vec![py_str("go back to uni"), py_str("go back to university"), py_str("going back to uni"), py_str("going back to university"), py_str("back to uni"), py_str("back to university"), py_str("back to studying"), py_str("back to study"), py_str("mature student"), py_str("mature students"), py_str("go back to college"), py_str("back to college"), py_str("going back to college"), py_str("college at 30"), py_str("college at 40"), py_str("college at 50"), py_str("evening class"), py_str("evening classes"), py_str("study as an adult"), py_str("degree as an adult"), py_str("evening degree"), py_str("part time degree"), py_str("open university"), py_str("study at night"), py_str("university at 30"), py_str("university at 40"), py_str("university at 50"), py_str("uni at 30"), py_str("uni at 40"), py_str("uni at 50"), py_str("degree at 30"), py_str("degree at 40"), py_str("in my 30s"), py_str("in my 40s"), py_str("in my 50s"), py_str("late in life")]) }
    fn __data137() -> Py { py_tuple(vec![py_str("40"), py_str("30"), py_str("35"), py_str("45"), py_str("50"), py_str("older"), py_str("later"), py_str("at my age"), py_str("worth"), py_str("should"), py_str("how"), py_str("as an adult"), py_str("after kids"), py_str("after children"), py_str("career change"), py_str("money"), py_str("cost"), py_str("afford")]) }
    fn __data138() -> Py { py_tuple(vec![py_str("pass mark"), py_str("passing mark"), py_str("pass"), py_str("passes"), py_str("percent"), py_str("percentage"), py_str("grade"), py_str("grades"), py_str("classification"), py_str("2:1"), py_str("first class"), py_str("ucas"), py_str("personal statement"), py_str("student loan"), py_str("tuition fee"), py_str("tuition fees"), py_str("how do i apply"), py_str("open day")]) }
    fn __data139() -> Py { py_tuple(vec![py_str("university"), py_str("uni"), py_str("degree"), py_str("degrees"), py_str("undergraduate"), py_str("bachelors"), py_str("masters"), py_str("msc"), py_str("bsc"), py_str("postgrad"), py_str("postgraduate"), py_str("module"), py_str("modules"), py_str("semester"), py_str("2:1"), py_str("2.1"), py_str("2:2"), py_str("2 1"), py_str("2 2"), py_str("first class"), py_str("upper second"), py_str("lower second"), py_str("distinction"), py_str("merit"), py_str("classification"), py_str("pass mark"), py_str("passing mark")]) }
    fn __data140() -> Py { py_tuple(vec![py_str("pass mark"), py_str("pass"), py_str("passing"), py_str("passed"), py_str("fail"), py_str("fails"), py_str("failing"), py_str("mark"), py_str("marks"), py_str("40"), py_str("50"), py_str("60"), py_str("70"), py_str("percent"), py_str("percentage"), py_str("average"), py_str("need"), py_str("what"), py_str("how much"), py_str("grade"), py_str("grades"), py_str("worth"), py_str("count"), py_str("counts"), py_str("resit"), py_str("resubmit"), py_str("good"), py_str("is"), py_str("are"), py_str("dont"), py_str("mean"), py_str("means")]) }
    fn __data141() -> Py { py_tuple(vec![py_str("gcse"), py_str("gcses"), py_str("a level"), py_str("a-level"), py_str("alevel"), py_str("alevels"), py_str("sixth form"), py_str("year 11"), py_str("year 10"), py_str("sats"), py_str("11+"), py_str("secondary school"), py_str("high school"), py_str("what should i study"), py_str("should i study"), py_str("what to study"), py_str("what should i take"), py_str("what should i do at uni"), py_str("what should i do at university"), py_str("tuition fee"), py_str("fees"), py_str("student loan"), py_str("student finance"), py_str("maintenance loan"), py_str("how do i apply"), py_str("ucas"), py_str("personal statement"), py_str("open day"), py_str("is university"), py_str("is uni"), py_str("worth going"), py_str("how much does"), py_str("worth it"), py_str("is it worth"), py_str("worth doing"), py_str("worth the money"), py_str("worth the debt"), py_str("at 40"), py_str("at 30"), py_str("at 50"), py_str("at 45"), py_str("in my 40s"), py_str("in my 30s"), py_str("in my 40's"), py_str("go back to uni"), py_str("back to uni"), py_str("back to university"), py_str("mature student"), g__A_DEGREE_OF_HEAT(), py_str("do i need a degree"), py_str("need a degree to"), py_str("degree to be"), py_str("to be a"), py_str("to be an"), py_str("become a"), py_str("become an"), py_str("get into"), py_str("qualify as"), py_str("train as"), py_str("stamp"), py_str("stamps"), py_str("postage"), py_str("post office"), py_str("postbox"), py_str("post box"), py_str("royal mail"), py_str("envelope"), py_str("envelopes"), py_str("signed for"), py_str("recorded delivery"), py_str("first class stamp"), py_str("second class stamp"), py_str("first class post"), py_str("second class post"), py_str("train ticket"), py_str("plane ticket"), py_str("flight"), py_str("seat"), py_str("carriage"), py_str("railcard")]) }
    fn __data142() -> Py { py_tuple(vec![py_str("which course"), py_str("what course"), py_str("course to study"), py_str("choose a course"), py_str("pick a course"), py_str("choosing a course"), py_str("which degree"), py_str("what degree"), py_str("choose a degree"), py_str("university course"), py_str("which university"), py_str("choose a university"), py_str("what to study at university"), py_str("what to study at uni"), py_str("what should i study"), py_str("what should i do at university"), py_str("what should i do at uni"), py_str("subject to study"), py_str("what subject")]) }
    fn __data143() -> Py { py_tuple(vec![py_str("should i"), py_str("how do i"), py_str("help"), py_str("advice"), py_str("choose"), py_str("pick"), py_str("decide"), py_str("between"), py_str("better"), py_str("best"), py_str("worth"), py_str("difference"), py_str("study"), py_str("do")]) }
    fn __data144() -> Py { py_tuple(vec![py_str("should i"), py_str("how do i"), py_str("help"), py_str("advice"), py_str("choose"), py_str("pick"), py_str("decide"), py_str("between"), py_str("better"), py_str("best"), py_str("worth"), py_str("difference"), py_str("study")]) }
    fn __data145() -> Py { py_tuple(vec![py_str("university worth"), py_str("worth it"), py_str("ucas"), py_str("personal statement"), py_str("apply"), py_str("clearing"), py_str("gap year"), py_str("a level subject"), py_str("a levels to"), py_str("gcse"), py_str("a level"), py_str("a levels"), py_str("a-level"), py_str("as level"), py_str("gcse"), py_str("gcses")]) }
    fn __data146() -> Py { py_tuple(vec![py_str("what is"), py_str("how does"), py_str("university"), py_str("ucas"), py_str("uni"), py_str("college"), py_str("course"), py_str("apply"), py_str("get in"), py_str("get a place"), py_str("get onto"), py_str("find"), py_str("place"), py_str("vacancy"), py_str("vacancies"), py_str("missed"), py_str("grades"), py_str("worse than"), py_str("not got in"), py_str("offer"), py_str("adjustment"), py_str("open"), py_str("opens"), py_str("start"), py_str("starts"), py_str("begin"), py_str("begins"), py_str("still")]) }
    fn __data147() -> Py { py_tuple(vec![py_str("what is"), py_str("how does"), py_str("how do i"), py_str("apply"), py_str("get"), py_str("find"), py_str("place"), py_str("vacancy"), py_str("missed"), py_str("grades"), py_str("when"), py_str("still"), py_str("open")]) }
    fn __data148() -> Py { py_tuple(vec![py_str("failed my"), py_str("resit"), py_str("remark"), py_str("a level results day"), py_str("my teacher"), py_str("cache"), py_str("cached"), py_str("cookies"), py_str("browser"), py_str("chrome"), py_str("safari"), py_str("app"), py_str("clear my"), py_str("clear the"), py_str("clear a"), py_str("clear an"), py_str("clear our"), py_str("clear your"), py_str("clear out"), py_str("clear up"), py_str("clearing my"), py_str("clearing the"), py_str("clearing out"), py_str("clear space"), py_str("clear some space"), py_str("clear storage"), py_str("clear enough"), py_str("cleared my"), py_str("cleared the"), py_str("clear some"), py_str("clear any"), py_str("clear debt"), py_str("clear debts"), py_str("clear arrears"), py_str("clear balance"), py_str("clear credit card"), py_str("clear loan"), py_str("clear loans"), py_str("clear mortgage"), py_str("clear overdraft"), py_str("clear it"), py_str("condensation"), py_str("condensed"), py_str("steamed up"), py_str("steaming up"), py_str("misted up"), py_str("fogged up"), py_str("fogging up"), py_str("pane"), py_str("panes"), py_str("squeegee"), py_str("windscreen"), py_str("clear room"), py_str("clear some room"), py_str("room on my"), py_str("room for"), py_str("laptop"), py_str("computer"), py_str("junk files"), py_str("downloads")]) }
    fn __data149() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("get"), py_str("aim"), py_str("need"), py_str("what"), py_str("difference"), py_str("improve"), py_str("better"), py_str("marks"), py_str("grade"), py_str("degree"), py_str("honours"), py_str("worth"), py_str("matter")]) }
    fn __data150() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("get"), py_str("aim"), py_str("need"), py_str("what"), py_str("difference"), py_str("improve"), py_str("better"), py_str("matter"), py_str("want"), py_str("want to"), py_str("hoping"), py_str("aiming"), py_str("trying")]) }
    fn __data151() -> Py { py_tuple(vec![py_str("a level"), py_str("gcse"), py_str("exam"), py_str("revision"), py_str("essay"), py_str("reference"), py_str("notes"), py_str("first aid"), py_str("first-aid"), py_str("aid"), py_str("certificate"), py_str("course"), py_str("stamp"), py_str("stamps"), py_str("postage"), py_str("post office"), py_str("letter"), py_str("parcel")]) }
    fn __data152() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("write"), py_str("writing"), py_str("sections"), py_str("layout"), py_str("format"), py_str("include"), py_str("need"), py_str("goes in"), py_str("as level"), py_str("a level"), py_str("gcse"), py_str("university")]) }
    fn __data153() -> Py { py_tuple(vec![py_str("university"), py_str("uni"), py_str("ucas"), py_str("degree"), py_str("offer"), py_str("clearing"), py_str("what does"), py_str("what do my grades mean"), py_str("my grades mean"), py_str("licence"), py_str("license"), py_str("driving"), py_str("motorway"), py_str("speeding"), py_str("insurance"), py_str("how many a levels"), py_str("how many a level"), py_str("how many subjects"), py_str("apply for college"), py_str("applying for college"), py_str("apply to college"), py_str("college application"), py_str("apply for sixth form"), py_str("nurse"), py_str("nursing"), py_str("teacher"), py_str("teaching"), py_str("vet"), py_str("veterinary"), py_str("doctor"), py_str("lawyer"), py_str("solicitor"), py_str("midwife"), py_str("electrician"), py_str("to be a")]) }
    fn __data154() -> Py { py_tuple(vec![py_str("leave school"), py_str("leaves school"), py_str("left school"), py_str("leaving school"), py_str("quit school"), py_str("quitting school"), py_str("drop out of school"), py_str("dropping out of school"), py_str("not stay on"), py_str("stay on at school"), py_str("stay at school"), py_str("sixth form"), py_str("college instead")]) }
    fn __data155() -> Py { py_tuple(vec![py_str("16"), py_str("sixteen"), py_str("at 16"), py_str("year 11"), py_str("year 12"), py_str("after gcses"), py_str("after gcse"), py_str("wont go"), py_str("won't go"), py_str("doesnt want"), py_str("refuses"), py_str("refusing"), py_str("should i let"), py_str("let him"), py_str("let her"), py_str("is it ok"), py_str("is it legal"), py_str("allowed"), py_str("have to"), py_str("must"), py_str("law")]) }
    fn __data156() -> Py { py_tuple(vec![py_str("can they"), py_str("can my"), py_str("is it"), py_str("should"), py_str("do they have to"), py_str("have to"), py_str("what are"), py_str("what can"), py_str("options"), py_str("instead"), py_str("wants to"), py_str("want to"), py_str("doesnt want"), py_str("does not want"), py_str("wont"), py_str("refuses"), py_str("leave"), py_str("leaving"), py_str("quit"), py_str("drop out")]) }
    fn __data157() -> Py { py_tuple(vec![py_str("what grades"), py_str("grades do i need"), py_str("how do i apply"), py_str("apply for college"), py_str("apply to college"), py_str("ucas"), py_str("university"), py_str("uni"), py_str("a level results"), py_str("gcse results"), py_str("results day"), py_str("birthday"), py_str("16th"), py_str("driving"), py_str("licence"), py_str("license"), py_str("moped"), py_str("cigarettes"), py_str("alcohol"), py_str("consent"), py_str("minimum wage"), py_str("national insurance")]) }
    fn __data158() -> Py { py_tuple(vec![py_str("strict teacher"), py_str("strict"), py_str("harsh teacher"), py_str("mean teacher"), py_str("unfair teacher"), py_str("teacher hates me"), py_str("teacher doesnt like me"), py_str("teacher doesn't like me"), py_str("does not like me"), py_str("doesnt like me"), py_str("does not like my"), py_str("dont like me"), py_str("picks on me"), py_str("picking on me"), py_str("picked on by"), py_str("shouted at by teacher"), py_str("teacher keeps"), py_str("hates"), py_str("hated"), py_str("hate my teacher")]) }
    fn __data159() -> Py { py_tuple(vec![py_str("what do i do"), py_str("how do i"), py_str("deal with"), py_str("handle"), py_str("cope"), py_str("help"), py_str("should i"), py_str("talk to"), py_str("complain"), py_str("tell"), py_str("my teacher"), py_str("teacher"), py_str("class"), py_str("lesson")]) }
    fn __data160() -> Py { py_tuple(vec![py_str("what do i do"), py_str("how do i"), py_str("deal with"), py_str("handle"), py_str("help"), py_str("should i"), py_str("talk to"), py_str("complain"), py_str("tell"), py_str("hate"), py_str("hates"), py_str("picks on"), py_str("picked on"), py_str("shouts"), py_str("unfair"), py_str("mean"), py_str("harsh"), py_str("like me"), py_str("not like")]) }
    fn __data161() -> Py { py_tuple(vec![py_str("boss"), py_str("manager"), py_str("work"), py_str("my job"), py_str("colleague"), py_str("bully"), py_str("bullied"), py_str("bullying"), py_str("homework"), py_str("exam"), py_str("the hate u give"), py_str("read after"), py_str("read next"), py_str("reading next"), py_str("what to read"), py_str("books like"), py_str("book like"), py_str("book after"), py_str("novel"), py_str("novels")]) }
    fn __data40() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("homework-worth-it")); __args.push(py_tuple(vec![py_tuple(vec![py_str("homework"), py_str("home work"), py_str("homeworks")]), __data41(), __data42()])); __args.push(py_str("Homework earns its place in small doses: about ten minutes a night in the first years of primary, half an hour by the end of it and an hour or so at secondary, and the evidence for it is weakest for the youngest children. What makes it work is the same each night - a fixed time, a clear table, and you nearby without taking over. If it is regularly taking much longer than the school's own limit, or ending in tears, that is worth one short message to the teacher rather than a battle every evening.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("homework"), py_str("home work"), py_str("homeworks")])), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data43())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("past-papers"), py_tuple(vec![py_tuple(vec![py_str("past paper"), py_str("past papers"), py_str("mock"), py_str("exam paper"), py_str("mark scheme")]), __data44()]), py_func("_past_papers_reply", |args: &[Py]| _past_papers_reply(args))]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("flashcards"), py_tuple(vec![py_tuple(vec![py_str("flashcard"), py_str("flashcards"), py_str("anki"), py_str("quizlet")]), py_tuple(vec![py_str("how"), py_str("make"), py_str("use"), py_str("best"), py_str("worth"), py_str("many")])]), py_str("One fact per card, question on one side, answer on the other, and no paragraphs: \"What year was the Battle of Hastings?\" not \"notes on 1066\". Review only the cards you get wrong, in short daily sessions, and make them yourself - writing them is half the memory work. Anki does the spacing for you if you prefer an app.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("mnemonics"), py_tuple(vec![py_tuple(vec![py_str("mnemonic"), py_str("mnemonics"), py_str("memory trick"), py_str("remember a list")]), py_tuple(vec![py_str("how"), py_str("remember"), py_str("best"), py_str("use"), py_str("lists"), py_str("what"), py_str("whats"), py_str("mean"), py_str("meaning")])]), py_str("Build a sentence from the first letters (Never Eat Shredded Wheat for the compass), or tie items to places on a walk round your house - the sillier the image, the better it sticks. Mnemonics are good for lists and orders, useless for understanding; use them for the facts you keep dropping.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("remember-names")); __args.push(py_tuple(vec![py_tuple(vec![py_str("names"), py_str("name"), py_str("forget names"), py_str("remembering names"), py_str("forget peoples names")]), __data45()])); __args.push(py_str("Use the name out loud in the first minute - \"Nice to meet you, Sarah\" - and hang it on something physical: alliteration (Tall Tom), a rhyme, or a face feature. Repeating it twice early beats any trick later, and if it has gone, say so straight away: \"Sorry, your name's gone\" costs less than guessing.")); __kws.extend(vec![("ask".to_string(), __data46()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&_somebody_else_forgets(&[understood.clone()]))) })), ("avoid".to_string(), __data47())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("memorise-dates")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dates"), py_str("timeline"), py_str("history")]), py_tuple(vec![py_str("remember"), py_str("memorise"), py_str("memorize"), py_str("learn"), py_str("keep forgetting")])])); __args.push(py_str("Put the dates on a blank timeline you draw yourself, and group them around a few anchor events you already know. Then link each new date to a story or a person rather than a bare number, and test yourself by covering the dates and filling them in. Ten dates you can place beat fifty you half-remember.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("my wife"), py_str("my husband"), py_str("my partner"), py_str("our dates"), py_str("our anniversary"), py_str("our plans"), py_str("marriage")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fraction-meaning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fraction"), py_str("fractions"), py_str("numerator"), py_str("denominator"), py_str("decimal")]), py_tuple(vec![py_str("what"), py_str("whats"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("is a"), py_str("are")])])); __args.push(py_str("A fraction is a part of a whole written as one number over another: the bottom says how many equal pieces the whole is cut into and the top says how many of those you have, so 3/4 is three pieces out of four. The one thing that trips people up is that the same amount has lots of names - 2/4 is 1/2, and 0.5 is 1/2 too - which is why you can simplify and compare them by putting them over the same bottom number.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("minutes"), py_str("seconds"), py_str("as a fraction"), py_str("in a fraction"), py_str("out of"), py_str("of my"), py_str("of the"), py_str("fraction of an hour is"), py_str("fraction of a day is"), py_str("fraction of a week is"), py_str("fraction of a minute is")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fractions-explain")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fraction"), py_str("fractions"), py_str("numerator"), py_str("denominator")]), py_tuple(vec![py_str("explain"), py_str("simple"), py_str("simply"), py_str("struggling"), py_str("understand"), py_str("child"), py_str("son"), py_str("daughter"), py_str("kid"), py_str("teach"), py_str("help")])])); __args.push(py_str("Start with something they can eat: cut an apple into four, take one piece - that is one quarter. Then show that the bottom number is how many pieces the whole was cut into and the top is how many you have. Do halves, quarters and thirds with real objects before any writing, then move to drawing them as bars of chocolate.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("times-tables")); __args.push(py_tuple(vec![py_tuple(vec![py_str("times tables"), py_str("times table"), py_str("multiplication"), py_str("times by"), py_str("6 times 7")]), py_tuple(vec![py_str("learn"), py_str("memorise"), py_str("remember"), py_str("how"), py_str("help"), py_str("child"), py_str("quick")])])); __args.push(py_str("Do them out loud, in order, then shuffled, in two-minute bursts a few times a day - short and often beats long and rare. Learn the easy ones first (2s, 5s, 10s, then the squares) so more than half the table is familiar. The hard ones are 6x7, 7x8, 8x9; make up a silly rhyme for just those.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("long-division")); __args.push(py_tuple(vec![py_tuple(vec![py_str("long division"), py_str("division"), py_str("divide"), py_str("dividing"), py_str("short division"), py_str("bus stop method"), py_str("bus stop"), py_str("remainder"), py_str("remainders")]), __data48()])); __args.push(py_str("Long division is the same four steps over and over: divide, multiply, subtract, bring down. Take 492 divided by 4: 4 goes into 4 once (write 1), 1 x 4 = 4, subtract to get 0, bring down the 9. 4 into 9 goes twice (write 2), 2 x 4 = 8, 9 - 8 = 1, bring down the 2 to make 12. 4 into 12 is 3, so the answer is 123. Say each step out loud the first few times, and use real objects for the first example so the place value makes sense.")); __kws.extend(vec![("avoid".to_string(), __data49())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fractions-sums")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fraction"), py_str("fractions")]), __data50()])); __args.push(py_func("_fractions_reply", |args: &[Py]| _fractions_reply(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("decimal"), py_str("decimals"), py_str("percent"), py_str("percentage"), py_str("percentages"), py_str("what is a"), py_str("whats a"), py_str("mean")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("maths-word-problems"), py_tuple(vec![py_tuple(vec![py_str("word problem"), py_str("word problems"), py_str("maths problem"), py_str("maths question")]), py_tuple(vec![py_str("solve"), py_str("how"), py_str("stuck"), py_str("approach"), py_str("help"), py_str("cant")])]), py_str("Underline what the question actually asks, then write down the numbers it gives you and what units they are in. Guess the answer roughly before calculating - if your working is far off the guess, the sum is wrong, not the guess. Draw it if there is any shape, distance or time in it.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("science-project-build")); __args.push(py_tuple(vec![py_tuple(vec![py_str("volcano"), py_str("volcanoes"), py_str("model volcano"), py_str("volcano model")]), __data51()])); __args.push(py_str("For a volcano: a bottle in a cone of foil and clay, then bicarbonate of soda with vinegar and a little washing-up liquid for the eruption (red food colouring helps). The part that earns marks is the write-up: label the bottle as the magma chamber, and explain that the gas is what pushes it out. It takes an afternoon and a tray.")); __kws.extend(vec![("avoid".to_string(), __data52())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("spelling-test")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spelling test"), py_str("spelling bee"), py_str("spellings"), py_str("spelling list"), py_str("weekly spellings"), py_str("cant spell"), py_str("can't spell"), py_str("cannot spell"), py_str("bad at spelling"), py_str("struggles to spell"), py_str("struggle to spell"), py_str("spelling")]), __data53()])); __args.push(py_str("Five words a night, not the whole list: look at the word, say it, cover it, write it, then check it, and put the ones she gets wrong to the back of the queue. Say the word in a sentence as you test her, because a spelling she can only write in a list is not learnt yet. Ten minutes is enough, and the night before is for the five she keeps missing, not for the twenty she knows.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how do you spell"), py_str("how do i spell"), py_str("how to spell"), py_str("spell the word"), py_str("spell it"), py_str("spelling of"), py_str("improve"), py_str("improving")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("essay-plan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("essay"), py_str("coursework"), py_str("assignment")]), op_add(&py_tuple(vec![py_str("plan"), py_str("start"), py_str("stuck"), py_str("blank"), py_str("deadline"), py_str("how"), py_str("write"), py_str("writing")]), &g_HOW_ASK())])); __args.push(py_str("Make the plan first and write the essay from it: list three or four points that answer the question, put one piece of evidence under each, and put them in the order that makes the argument grow. Then write the body paragraphs first and the introduction last, when you know what you argued. Two hours of planning saves a day of staring.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("mistakes"), py_str("mistake"), py_str("grammar"), py_str("spell"), py_str("spelling"), py_str("proofread"), py_str("typo"), py_str("typos"), py_str("check"), py_str("unclear"), py_str("structure of the sentence")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("proofread-help")); __args.push(py_tuple(vec![py_tuple(vec![py_str("spell check"), py_str("spellcheck"), py_str("proofread"), py_str("proof read"), py_str("check this email"), py_str("check my email"), py_str("check this essay"), py_str("check my essay"), py_str("check my cv")]), py_tuple(vec![py_str("can"), py_str("could"), py_str("you"), py_str("please"), py_str("how"), py_str("help"), py_str("this"), py_str("my")])])); __args.push(py_str("I cannot read it for you, but the check that catches most of it is reading it aloud: your ear stops on the missing word and the doubled one. Then look at four things in order - the name at the top, the ask in the first line, any number or date, and the last line. Shorten any sentence you have to read twice, and leave it for ten minutes before the last read if you can.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("essay"), py_str("essays"), py_str("dissertation"), py_str("coursework"), py_str("assignment"), py_str("thesis")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("reading-comprehension")); __args.push(py_tuple(vec![py_tuple(vec![py_str("comprehension"), py_str("reading"), py_str("passage")]), py_tuple(vec![py_str("understand"), py_str("answer"), py_str("questions"), py_str("help"), py_str("improve"), py_str("exam"), py_str("better"), py_str("get better"), py_str("struggling"), py_str("weakest")])])); __args.push(py_str("Read the questions before the passage, so you know what to look for. Then read once for sense and go back to the lines the questions point at, quoting a few words in your answer. For \"why\" questions, answer with because plus something from the text; for \"how do you know\", quote it.")); __kws.extend(vec![("avoid".to_string(), __data54())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("reading-with-a-child")); __args.push(py_tuple(vec![py_tuple(vec![py_str("reading"), py_str("read"), py_str("books"), py_str("phonics")]), __data55()])); __args.push(py_str("Ten minutes a night, the same time, with them reading out loud to you: stop at the first hard word, sound it out together, and let them finish the sentence. Read the same short book twice, because the second read is where the confidence comes from, and keep a book in the car or bag for dead time. Ask one question about the story afterwards rather than testing words.")); __kws.extend(vec![("avoid".to_string(), __data56())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-comic-counts-as-reading")); __args.push(py_tuple(vec![py_tuple(vec![py_str("count as reading"), py_str("counts as reading"), py_str("count as a book"), py_str("count as books"), py_str("still reading"), py_str("proper reading")]), __data57()])); __args.push(py_str("Yes, it counts. Comics, graphic novels, magazines and audio books all build the same thing a book does - the habit and the vocabulary - and a child who reads what they like reads far more of it than one handed a book they do not. Keep one chapter book alongside it rather than banning the comics, and ask what happened in the story afterwards. For audio books, same idea: the words go in by ear, and a paper copy of the same book helps once they can follow along.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-breaks")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pomodoro"), py_str("study breaks"), py_str("study break"), py_str("how long should i study"), py_str("how long to study"), py_str("breaks when studying")]), py_tuple(vec![py_str("how"), py_str("long"), py_str("often"), py_str("best"), py_str("study"), py_str("studying"), py_str("revision"), py_str("technique"), py_str("method"), py_str("mean"), py_str("work")])])); __args.push(py_str("25 minutes on, 5 off, and a longer break every four blocks: the short block keeps quality up, the break is what makes it repeatable. In the break, stand up and move rather than scrolling, because a phone break turns five minutes into twenty. Stop while it is still going well, so tomorrow is easier to start.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("instructions"), py_str("assembling"), py_str("manual"), py_str("recipe"), py_str("flat pack"), py_str("flatpack"), py_str("how long should i study"), py_str("how long should i revise"), py_str("how many hours")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("study-with-adhd"), py_tuple(vec![py_tuple(vec![py_str("adhd"), py_str("attention deficit")]), py_tuple(vec![py_str("study"), py_str("revise"), py_str("focus"), py_str("concentrate"), py_str("help"), py_str("tips")])]), py_str("Make the task visible and small: one page, one timer, everything else out of the room. Body-doubling works for a lot of people - study alongside someone, in person or on a video call with mics off. Move between two or three subjects rather than forcing one, and use a paper checklist you tick, because visible progress is the fuel.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("motivation-to-study"), py_tuple(vec![py_tuple(vec![py_str("motivation"), py_str("motivated"), py_str("pointless"), py_str("cant be bothered"), py_str("no point")]), py_tuple(vec![py_str("study"), py_str("revise"), py_str("school"), py_str("exam"), py_str("how"), py_str("help"), py_str("why")])]), py_str("Motivation follows action, not the other way round: do ten minutes and decide after. Pick the subject you least dread but find easiest to start with, because a bad day is about getting one thing done, not all of it. If it has felt pointless for weeks, tell someone at school or work; that is a different problem from laziness.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("learn-to-the-test")); __args.push(py_tuple(vec![py_tuple(vec![py_str("learn"), py_str("memorise")]), py_tuple(vec![py_str("fast"), py_str("quickly"), py_str("overnight"), py_str("in a day"), py_str("last minute"), py_str("exam")])])); __args.push(py_str("Last-minute learning works for facts and structures, not skills: take the syllabus or mark scheme, list what is actually examinable, and learn the highest-mark items with self-testing. Sleep before the exam and skip the things you already know. Be honest with yourself - this buys a pass, not understanding.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_LANGUAGE_LEARNING(), &py_tuple(vec![py_str("the job"), py_str("my job"), py_str("new job"), py_str("the role"), py_str("my role"), py_str("at work"), py_str("on the job"), py_str("job quickly"), py_str("the ropes"), py_str("ropes"), py_str("up to speed")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("stop-cramming")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cram"), py_str("cramming"), py_str("crammed"), py_str("all nighter"), py_str("all-nighter"), py_str("all-nighters"), py_str("last minute revision"), py_str("last-minute revision"), py_str("leave it to the last minute"), py_str("leaving it late"), py_str("leaving everything"), py_str("leaving revision")]), py_tuple(vec![py_str("stop"), py_str("stops"), py_str("how"), py_str("help"), py_str("tips"), py_str("avoid"), py_str("not"), py_str("always"), py_str("every"), py_str("why"), py_str("end up"), py_str("keep")])])); __args.push(py_str("Make the first session tiny and today: open the topic list, pick the one you know least, and test yourself on it for ten minutes, because starting is the part that is failing rather than the studying. Then put three short sessions in the week you actually have, each ending with a self-test instead of a reread, and give each one a stopping time so it cannot drift to midnight. Testing yourself is what makes the last day calm instead of terrible; if the exam is tomorrow, pack the bag, read your one-page summary once, and sleep.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("no sleep"), py_str("bad sleep")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("practise-a-talk")); __args.push(py_tuple(vec![py_tuple(vec![py_str("practise"), py_str("practice"), py_str("practising"), py_str("practicing"), py_str("rehearse"), py_str("rehearsal"), py_str("rehearsing"), py_str("run through"), py_str("run it through"), py_str("go through it out loud")]), __data58()])); __args.push(py_str("Say it out loud three times, standing up, with the slides in front of you: the first run finds the sentences that only looked fine on paper, the second is against a timer, and the third is to one person who will tell you where they lost the thread. Time it against the slot you were given and cut material rather than speed up. Write the opening sentence and the closing one out in full and keep everything between them to a few words per card, then practise stopping for a second after your last point instead of trailing off.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("group-project"), py_tuple(vec![py_tuple(vec![py_str("group project"), py_str("group work"), py_str("team project"), py_str("presentation with"), py_str("work in a group"), py_str("work well in a group"), py_str("work as a group"), py_str("group of us"), py_str("work with a group")]), py_tuple(vec![py_str("how"), py_str("help"), py_str("lazy"), py_str("fair"), py_str("split"), py_str("work"), py_str("handle")])]), py_str("At the first meeting, split the work in writing with names and dates, and pick one person to hold the whole thing together. Swap drafts before the deadline so no one is the bottleneck. If someone does nothing, tell them early what you need by when; chasing at the end is where the resentment comes from.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("teach-someone"), py_tuple(vec![py_tuple(vec![py_str("explain it to someone"), py_str("teach someone"), py_str("teach it to someone"), py_str("teaching someone"), py_str("feynman")]), py_tuple(vec![py_str("how"), py_str("best way to learn"), py_str("understand"), py_str("check"), py_str("learn")])]), py_str("Explain it out loud to someone who does not know it, without notes. Where you stumble or reach for jargon is exactly what you do not understand yet, so go back to that bit and try again. Ten minutes of teaching yourself in the mirror is worth an hour of rereading.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("grade-meaning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("grade"), py_str("grades"), py_str("gcse"), py_str("gcses"), py_str("a level"), py_str("a-level"), py_str("alevel"), py_str("alevels"), py_str("marks"), py_str("score")]), __data59()])); __args.push(py_func("_grade_answer", |args: &[Py]| _grade_answer(args))); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_NOT_AN_EXAM(), &g_MOCK()), &__data60())), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mock-grade")); __args.push(py_tuple(vec![g_MOCK(), __data61()])); __args.push(py_func("_mock_answer", |args: &[Py]| _mock_answer(args))); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("revision-start")); __args.push(py_tuple(vec![py_tuple(vec![py_str("revising"), py_str("revise"), py_str("revision"), py_str("exam"), py_str("exams"), py_str("test"), py_str("mocks"), py_str("mock")]), __data62()])); __args.push(_unless_testing_a_thing(&[py_str("Start with something tiny today: open the past paper or the topic list and write down what you already know about one topic — ten minutes, that is the whole job. Then work backwards from the exam date and give each topic a day, in 25-minute blocks with a 5-minute break. The last week should be past papers under time, not reading notes.")])); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_AN_EXAM(), &g_NOT_REVISION()), &g_ASLEEP()), &g_REST_ASK()), &__data63()), &g_NOT_A_START_OF_EXAMS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("revision-active")); __args.push(py_tuple(vec![py_tuple(vec![py_str("revising"), py_str("revise"), py_str("revision"), py_str("studying"), py_str("study"), py_str("revision notes"), py_str("make revision notes"), py_str("revision methods"), py_str("revision techniques")]), op_add(&__data64(), &g_HOW_ASK())])); __args.push(py_str("Testing yourself is what makes things stick; rereading notes mostly feels productive without being so. Turn each topic into five questions and answer them with the book shut, then check, and redo the ones you got wrong tomorrow and again in a week. Reread only the bits you could not answer.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_AN_EXAM(), &g_NOT_REVISION()), &g_ASLEEP()), &g_REST_ASK()), &g_A_SUBJECT()), &py_tuple(vec![py_str("time of day"), py_str("best time"), py_str("better time"), py_str("what time"), py_str("when is the best")])), &g_NOT_A_START_OF_EXAMS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("revision-method-names")); __args.push(py_tuple(vec![py_tuple(vec![py_str("active recall"), py_str("spaced repetition"), py_str("spaced practice"), py_str("retrieval practice"), py_str("testing yourself"), py_str("interleaving"), py_str("blurting"), py_str("leitner")]), py_tuple(vec![py_str("what"), py_str("whats"), py_str("mean"), py_str("means"), py_str("is"), py_str("explain"), py_str("about"), py_str("definition"), py_str("define"), py_str("work"), py_str("does"), py_str("do")])])); __args.push(py_str("Testing yourself is what makes things stick; rereading notes mostly feels productive without being so. Turn each topic into five questions and answer them with the book shut, then check, and redo the ones you got wrong tomorrow and again in a week. Reread only the bits you could not answer.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-time-of-day")); __args.push(py_tuple(vec![py_tuple(vec![py_str("study"), py_str("studying"), py_str("revise"), py_str("revising"), py_str("revision")]), __data65()])); __args.push(py_str("The time you will actually do it beats the perfect time, and for anything that needs concentration most people are sharper in the morning. Hard topics first, drilling and past papers later, and keep the last half hour before bed off screens if you can. If you are a night owl and it works, do not fight it — just do it at the same time each day.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_NOT_AN_EXAM(), &g_ASLEEP()), &py_tuple(vec![py_str("shift"), py_str("shifts"), py_str("rota"), py_str("work nights"), py_str("night shift")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("vocab-test")); __args.push(py_tuple(vec![py_tuple(vec![py_str("vocabulary"), py_str("vocab"), py_str("words")]), py_tuple(vec![py_str("remember"), py_str("learn"), py_str("memorise"), py_str("memorize"), py_str("test"), py_str("language"), py_str("french"), py_str("spanish"), py_str("german"), py_str("italian")])])); __args.push(py_str("Learn seven to ten words at a time, out loud, and test yourself instead of reading the list: cover the English side and say the word, then swap. Put each new word in a short sentence you would actually say, and go back over yesterday's words for two minutes before adding new ones — spacing is what makes them stay.")); __kws.extend(vec![("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&_somebody_else_forgets(&[understood.clone()]))) })), ("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &g_ASLEEP())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("exam-night-before")); __args.push(py_tuple(vec![py_tuple(vec![py_str("exam"), py_str("exams"), py_str("test")]), py_tuple(vec![py_str("night before"), py_str("tomorrow"), py_str("cram"), py_str("cramming"), py_str("evening"), py_str("sleep"), py_str("bedtime")])])); __args.push(py_str("Sleep is worth more than the extra hours of cramming: a rested brain recalls what you already know, a tired one does not. Pack your bag and the pens you need, read your one-page summary once, then stop by nine and do something that is not revision. If you really cannot settle, write down the three things you are worried about forgetting.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("exam-nerves")); __args.push(py_tuple(vec![py_tuple(vec![py_str("exam"), py_str("exams"), py_str("test")]), __data66()])); __args.push(py_func("_exam_nerves", |args: &[Py]| _exam_nerves(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), g_NOT_AN_EXAM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("exam-day-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("mock"), py_str("mocks"), py_str("exam day"), py_str("exam morning")]), __data67()])); __args.push(py_str("Something slow and ordinary an hour or two before, with water: porridge, toast and egg, or cereal and a banana. A sugary breakfast spikes and drops you by the second hour, which is exactly when the paper gets hard, so keep the sweet thing for the break. Take a bottle of water and a plain snack in, and if the exam is in the afternoon eat a light lunch rather than a big one - a full stomach is what makes people sleepy at 2pm. Keep the coffee you usually drink and nothing more: a double espresso you never have is what sends people to the loo twice in the first half hour.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &__data68())), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("essay-opening")); __args.push(py_tuple(vec![py_str("essay"), py_tuple(vec![py_str("opening"), py_str("first sentence"), py_str("first line"), py_str("introduction"), py_str("start"), py_str("begin"), py_str("sentence"), py_str("hook")])])); __args.push(py_str("Answer the question in your first sentence, plainly, and let the rest of the essay back it up: \"Prisons in the 1800s did more to hide poverty than to fix it.\" No \"In this essay I will discuss\" and no dictionary definition — a marker wants to see your answer straight away.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("essay-structure")); __args.push(py_tuple(vec![py_str("essay"), py_tuple(vec![py_str("structure"), py_str("paragraph"), py_str("paragraphs"), py_str("conclusion"), py_str("argue"), py_str("argument"), py_str("organise"), py_str("organize")])])); __args.push(py_str("One idea per paragraph: a sentence that says what it argues, the evidence for it, then a line saying why that matters to the question. Your conclusion should answer the question as asked, not repeat the introduction. If you are stuck, write the paragraphs' first sentences as a list first and check they add up to an argument.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("essay-check")); __args.push(py_tuple(vec![py_str("essay"), py_tuple(vec![py_str("proofread"), py_str("check"), py_str("mistakes"), py_str("shorter"), py_str("word count"), py_str("edit"), py_str("cut"), py_str("improve")])])); __args.push(py_str("Read it out loud, or have your computer read it aloud: the places you stumble are the places to fix. Cut every \"very\", \"really\" and \"basically\", make sure each paragraph's first sentence says its point, and check names, dates and quotes — those are what get marked down.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("notes-lecture")); __args.push(py_tuple(vec![py_tuple(vec![py_str("note"), py_str("note-taking"), py_str("notetaking"), py_str("notes from")]), __data69()])); __args.push(py_str("Write in your own words, in short lines, and do not try to transcribe: you are making a map to come back to, not a transcript. Leave a wide margin for questions after the lesson and add them the same evening, in five minutes, while it is still fresh. Handwriting beats typing for remembering, if it is a choice.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &g_NOT_A_PERSONS_NOTE())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-distracted")); __args.push(py_tuple(vec![__data70(), __data71()])); __args.push(py_str("Put the phone in another room — not face down, another room — and work in 25-minute blocks with a 5-minute break where you get up. Before each block, write the one thing you will finish on a scrap of paper so you do not have to decide where to start. Same desk, same time, same little ritual helps more than willpower.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &g_LOSING_TRACK_OF_MONEY())), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("concentration")); __args.push(py_tuple(vec![__data72(), __data73()])); __args.push(py_str("Mostly it is the setup, not willpower: enough sleep, food, the phone in another room and one thing open at a time. Work in 25-minute blocks on a single task written down first, because choosing where to start is half the distraction, and stand up for the break. If your mind wanders when you read, take notes by hand and say the page back to yourself in one line at the end of it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("camera"), py_str("lens"), py_str("autofocus"), py_str("photograph"), py_str("photo"), py_str("diet"), py_str("weight"), py_str("good things"), py_str("road"), py_str("driving"), py_str("meeting"), py_str("meetings")])), ("bias".to_string(), op_neg(&Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("learn-language-daily")); __args.push(py_tuple(vec![__data74(), __data75()])); __args.push(py_str("Split your time in three: a few minutes going over yesterday's phrases out loud, most of it on new phrases said aloud and written down once, and a few minutes listening to something short at normal speed. Learn whole phrases you will use rather than grammar tables, and speak from the first week — even to yourself in the kitchen. One lesson or one conversation partner a week is what makes the rest work.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-a-language")); __args.push(py_tuple(vec![__data76(), __data77()])); __args.push(py_str("Pick by what you will actually use it for, not by which is easiest on paper: the language of the country you visit, or of the family you are marrying into, beats any ranking. With no reason to choose, Spanish is the usual bet - about 500 million speakers, spelling that matches the sound, and a lot of cheap material - and if you already have French or Italian, the other one of the pair is the cheapest start of all. The clock is the real cost: about 600 to 750 hours of class for Spanish, French or Italian, but roughly 2,200 for Arabic, Chinese, Japanese or Korean.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &__data78())), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("language-hours")); __args.push(py_tuple(vec![op_add(&g_LANGUAGE_LEARNING(), &py_tuple(vec![py_str("learn"), py_str("learning"), py_str("learnt"), py_str("learned"), py_str("become fluent"), py_str("get fluent"), py_str("pick up")])), py_tuple(vec![py_str("long"), py_str("hours"), py_str("years"), py_str("months"), py_str("weeks"), py_str("until"), py_str("soon"), py_str("time")])])); __args.push(py_func("_language_hours", |args: &[Py]| _language_hours(args))); __kws.extend(vec![("ask".to_string(), __data79()), ("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &__data80())), ("guard".to_string(), py_func("_names_a_language", |args: &[Py]| _names_a_language(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("trip-phrases")); __args.push(py_tuple(vec![py_tuple(vec![py_str("phrases"), py_str("phrasebook")]), py_tuple(vec![py_str("learn"), py_str("before"), py_str("holiday"), py_str("trip"), py_str("italy"), py_str("spain"), py_str("france"), py_str("language"), py_str("useful")])])); __args.push(py_str("Learn these twelve and you can get through most of a holiday: hello, goodbye, please, thank you, sorry, excuse me, do you speak English, I don't understand, how much is this, the bill please, where is the toilet, and help. Say them out loud until they are automatic, because under pressure you will use what you can say without thinking.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("say-language")); __args.push(py_tuple(vec![py_tuple(vec![py_str("say"), py_str("pronounce"), py_str("translate"), py_str("translation")]), __data81()])); __args.push(py_func("_say_in_language", |args: &[Py]| _say_in_language(args))); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("guard".to_string(), py_func("_a_phrase_is_asked", |args: &[Py]| _a_phrase_is_asked(args))), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("say-welsh-irish")); __args.push(py_tuple(vec![py_tuple(vec![py_str("say"), py_str("pronounce"), py_str("translate"), py_str("translation"), py_str("whats"), py_str("what is"), py_str("what's")]), py_tuple(vec![py_str("welsh"), py_str("cymraeg"), py_str("wales"), py_str("irish"), py_str("gaeilge")])])); __args.push(py_func("_say_in_language", |args: &[Py]| _say_in_language(args))); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("guard".to_string(), py_func("_a_phrase_is_asked", |args: &[Py]| _a_phrase_is_asked(args))), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("say-language-phrase")); __args.push(py_tuple(vec![__data82(), g_CONCEPT_WORDS(), __data83()])); __args.push(py_func("_say_in_language", |args: &[Py]| _say_in_language(args))); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("guard".to_string(), py_func("_a_phrase_is_asked", |args: &[Py]| _a_phrase_is_asked(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-does-that-mean")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mean"), py_str("means"), py_str("meaning"), py_str("what does"), py_str("whats")]), call_builtin("tuple", &[call_builtin("sorted", &[g_NATIVE()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))])); __args.push(py_func("_what_does_it_mean", |args: &[Py]| _what_does_it_mean(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("mean"), py_str("means"), py_str("meaning"), py_str("translate"), py_str("translation"), py_str("stand for"), py_str("stands for"), py_str("what does it say"), py_str("in english")])), ("avoid".to_string(), py_tuple(vec![py_str("i mean"), py_str("i meant"), py_str("meaning to say")])), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_a_word_is_being_asked_about", |args: &[Py]| _a_word_is_being_asked_about(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subjects-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("choose"), py_str("choosing"), py_str("pick"), py_str("picking")]), py_tuple(vec![py_str("a-levels"), py_str("alevels"), py_str("a levels"), py_str("gcses"), py_str("subjects"), py_str("degree"), py_str("course"), py_str("university"), py_str("college")])])); __args.push(py_str("Pick the subjects you will still work at on a bad week, then check the entry requirements for two or three courses you might want later, because that is what closes doors rather than the subjects themselves. Keeping one essay subject and one numerical one keeps more options open than people expect. Ask a teacher you trust what they think you would enjoy, not just what you are good at.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("university-worth")); __args.push(py_tuple(vec![py_tuple(vec![py_str("university"), py_str("uni"), py_str("degree")]), __data84()])); __args.push(py_str("It depends on what you want to do: for nursing, engineering, law or teaching the degree is the door, and for plenty of other jobs experience gets you further for less money. Compare a degree with a degree apprenticeship in the same field — earn while you learn, no fees — and look at what people actually doing the job you want studied. Ask two of them before deciding.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &g__A_DEGREE_OF_HEAT()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("apprenticeship-worth")); __args.push(py_tuple(vec![py_tuple(vec![py_str("apprenticeship"), py_str("apprenticeships"), py_str("apprentice"), py_str("degree apprenticeship"), py_str("higher apprenticeship"), py_str("level 2 apprenticeship"), py_str("level 3 apprenticeship")]), __data85()])); __args.push(py_str("An apprenticeship is a paid job with the training built into the week: most of the time you work, about a fifth of it you study, and there are no tuition fees. They run from GCSE level up to degree apprenticeships in engineering, digital, nursing and accountancy, so one can stand in place of a degree - you earn while you learn and finish with experience and no student debt, but with less choice of where you study. Pay is at least the apprentice minimum wage and more on most schemes.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &__data86()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("results-day")); __args.push(py_tuple(vec![py_tuple(vec![py_str("results"), py_str("grades"), py_str("gcse"), py_str("gcses"), py_str("a-level"), py_str("a level"), py_str("a levels"), py_str("alevels")]), __data87()])); __args.push(py_str("First, it is not the end of anything: there are more routes from here than there were before results day. If you have missed a university offer, ring the university yourself today — courses take people who call — and look at Clearing the same afternoon. For school exams, ask about a remark and about resitting in November or next summer; both are normal and neither sets you back a year if you use the time.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &g_MOCK())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-procrastinate")); __args.push(py_tuple(vec![py_tuple(vec![py_str("procrastinat"), py_str("put off"), py_str("putting off"), py_str("put it off"), py_str("put things off"), py_str("cant start"), py_str("can't start"), py_str("cannot start"), py_str("keep putting"), py_str("get started"), py_str("get going")]), __data88()])); __args.push(py_str("Do the smallest possible first step, not the whole job: open the book at the page and write the title and today's date, or set a timer for five minutes and let yourself do it badly. Putting it off is mostly about the task feeling too big to start, and it shrinks as soon as you are in it. Decide tonight what tomorrow's first step is, so then it is an instruction to follow rather than a choice to make.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &py_tuple(vec![py_str("exam"), py_str("exams"), py_str("gcse"), py_str("gcses"), py_str("a-level"), py_str("a levels"), py_str("mocks"), py_str("mock"), py_str("test"), py_str("tests")]))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thesis-statement")); __args.push(py_tuple(vec![py_tuple(vec![py_str("thesis statement"), py_str("topic sentence"), py_str("thesis")]), py_tuple(vec![py_str("what"), py_str("write"), py_str("how"), py_str("example"), py_str("essay"), py_str("introduction"), py_str("start"), py_str("good"), py_str("should"), py_str("my")])])); __args.push(py_str("A thesis statement is one sentence that says the claim your essay will prove, and why it matters: \"Cheaper public transport would cut city emissions more than electric cars, because most journeys are short.\" Put it at the end of your first paragraph, make it something a reasonable person could disagree with, and check that every paragraph after it is evidence for that one sentence. If a paragraph is not, either cut it or change the thesis.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("phd"), py_str("masters"), py_str("supervisor"), py_str("submitted"), py_str("binding"), py_str("library")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-timetable")); __args.push(py_tuple(vec![py_tuple(vec![py_str("timetable"), py_str("revision timetable"), py_str("schedule"), py_str("planner"), py_str("spreadsheet"), py_str("plan"), py_str("planning")]), __data89()])); __args.push(py_str("Count the days you actually have (sleep and school nights are not revision), list the topics per subject, and give each topic one block rather than one subject one day: three blocks a day is a real day's work. Put the subject you dread first, in the morning. Leave the last three days for past papers under time, and leave one day empty, because something always slips.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-how-long")); __args.push(py_tuple(vec![py_tuple(vec![py_str("study"), py_str("studying"), py_str("revise"), py_str("revising"), py_str("revision"), py_str("homework")]), __data90()])); __args.push(py_str("About 25 to 45 minutes at a time, then a proper 5 to 10 minutes away from the desk: two or three hours of that is a good day, and four is a very long one. What counts is testing yourself in the block, not the hours you sat there. If you keep losing track, set a timer for the block and stop when it goes, even mid-sentence — stopping on a problem you can pick up again makes the next start easy.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_NOT_AN_EXAM(), &g_NOT_REVISION()), &py_tuple(vec![py_str("how long before"), py_str("how long until"), py_str("how far ahead"), py_str("how early"), py_str("weeks before"), py_str("months before"), py_str("days before"), py_str("how long do i need"), py_str("how long does it take to revise"), py_str("how long should i revise for the exam")]))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("homework-amount")); __args.push(py_tuple(vec![py_tuple(vec![py_str("homework"), py_str("home work")]), py_tuple(vec![py_str("how much"), py_str("much homework"), py_str("how long should homework"), py_str("how many minutes"), py_str("how many hours of homework"), py_str("how long homework")])])); __args.push(py_str("Most primary homework is 20 to 30 minutes a night by the last years at primary, and about an hour a night in year 7 and up, with a break part way. If a piece runs well past that, write a note to the teacher rather than pushing through it, because homework that takes an hour and ends in tears is usually the wrong homework.")); __kws.extend(vec![("avoid".to_string(), __data91()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-forgetting")); __args.push(py_tuple(vec![py_tuple(vec![py_str("forget"), py_str("forgetting"), py_str("forgot"), py_str("cant remember"), py_str("can't remember"), py_str("wont stick"), py_str("won't stick"), py_str("doesnt stick"), py_str("doesn't stick")]), py_tuple(vec![py_str("revise"), py_str("revising"), py_str("revision"), py_str("study"), py_str("studying"), py_str("learn"), py_str("learned"), py_str("learnt"), py_str("memory"), py_str("remember"), py_str("everything")])])); __args.push(py_str("That is normal and it has a fix: what you re-read fades in a day or two, and what you test yourself on comes back. Same day, look at the topic and cover it and say it out loud; then again after two days, after a week, and after a month. Turn each topic into five questions and keep them on cards, and go through yesterday's cards for two minutes before starting anything new.")); __kws.extend(vec![("ask".to_string(), __data92()), ("avoid".to_string(), g_NOT_AN_EXAM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("revision-essay-subjects")); __args.push(py_tuple(vec![__data93(), __data94()])); __args.push(py_str("Essay subjects are learned by writing answers, not by reading the notes again: get the mark scheme for your board, write three timed past questions a week and plan the other two in a spider diagram. Mark your own answer against the criteria and count what you missed - a point you did not make, a quote, the evaluation sentence at the end - because the marks are in the structure as much as the knowledge. Learn five specific examples or quotes per topic rather than twenty vague ones, and keep them on cards: one named case with a date beats a paragraph about 'many historians'.")); __kws.extend(vec![("ask".to_string(), __data95()), ("avoid".to_string(), __data96())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("revision-maths")); __args.push(py_tuple(vec![py_tuple(vec![py_str("maths"), py_str("math"), py_str("mathematics"), py_str("algebra"), py_str("geometry"), py_str("trigonometry"), py_str("physics"), py_str("chemistry"), py_str("biology"), py_str("science"), py_str("calculations")]), __data97()])); __args.push(py_str("Maths and the sciences are learned by doing, not reading: work through the examples with the book open, then close it and do them again, then do past papers with the mark scheme next to you. Mark them honestly and write down the topics that lost marks; those get redone three days later. Learn the formulas by using them in questions rather than copying them out ten times, and if a topic still will not work, ask a teacher or a classmate to watch you do one question.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("memorise-lines")); __args.push(py_tuple(vec![py_tuple(vec![py_str("memorise"), py_str("memorize"), py_str("memorising"), py_str("memorizing"), py_str("learn by heart"), py_str("off by heart"), py_str("learn lines"), py_str("my lines"), py_str("lines")]), py_tuple(vec![py_str("speech"), py_str("poem"), py_str("lines"), py_str("script"), py_str("play"), py_str("part"), py_str("presentation"), py_str("talk"), py_str("wedding"), py_str("words"), py_str("monologue")])])); __args.push(py_str("Split it into chunks of three or four lines and learn the first word of each chunk, because that is what you will forget first. Read the chunk aloud, look away and say it, then add the next chunk and say both; record yourself and listen while walking or washing up. Say the whole thing out loud twice a day and it settles in about a week.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &py_tuple(vec![py_str("opening line"), py_str("opening lines"), py_str("first line"), py_str("first words"), py_str("opener"), py_str("icebreaker"), py_str("ice breaker"), py_str("hook"), py_str("joke"), py_str("jokes"), py_str("funny")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-mornings")); __args.push(py_tuple(vec![py_tuple(vec![py_str("get up"), py_str("getting up"), py_str("get out of bed"), py_str("wake up"), py_str("waking up"), py_str("mornings"), py_str("morning"), py_str("alarm")]), py_tuple(vec![py_str("early"), py_str("study"), py_str("studying"), py_str("revise"), py_str("revising"), py_str("revision"), py_str("5am"), py_str("6am"), py_str("before school"), py_str("before work")])])); __args.push(py_str("Put the alarm across the room, and set it for the same time every day including weekends, because a lie-in on Sunday is what makes Monday impossible. Be in bed eight hours before it, and leave the book open at the page the night before so the first ten minutes need no decision. Then get up when it rings: negotiating with yourself is the part that fails.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data98())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("speech-opener")); __args.push(py_tuple(vec![__data99(), __data100()])); __args.push(py_str("Ask the room something it already knows the answer to, or name the one detail everyone there has seen - \"hands up who has been handed a handbag and told to hold it\" - then wait a beat before your name. Keep it under ten seconds and make the joke about you rather than anyone in the room, and if nothing comes, open with the strangest real detail of the story instead of a joke.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("poem"), py_str("poems"), py_str("script"), py_str("play"), py_str("monologue"), py_str("card"), py_str("cards"), py_str("vows"), py_str("eulogy"), py_str("thank you letter")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-awake")); __args.push(py_tuple(vec![py_tuple(vec![py_str("awake"), py_str("tired"), py_str("sleepy"), py_str("nodding off"), py_str("drowsy"), py_str("exhausted"), py_str("asleep"), py_str("falling asleep"), py_str("sleep")]), py_tuple(vec![py_str("study"), py_str("studying"), py_str("revise"), py_str("revising"), py_str("revision"), py_str("homework"), py_str("lecture"), py_str("class"), py_str("stay"), py_str("staying")])])); __args.push(py_str("Stand up and move for two minutes every half hour, drink water, and put a bright light on your face — sleepiness at a desk is often boredom plus warm air. Do the hardest thing first and leave the reading for when you are flagging. If you are falling asleep at 9pm you need sleep, not coffee: an early night and an hour in the morning beats two hours of fighting it.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &py_tuple(vec![py_str("homework battle"), py_str("battle over homework"), py_str("battles over homework"), py_str("homework is a battle"), py_str("homework every night"), py_str("every night is a battle"), py_str("the homework battle")]))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("homework-child")); __args.push(py_tuple(vec![py_tuple(vec![py_str("homework")]), __data101()])); __args.push(py_str("Same time and same table every night, twenty minutes then a break, and the phone in another room — the fight is usually about the phone and the sudden start, not the work. Sit nearby doing your own quiet job rather than over their shoulder, and only help when they ask. If it is still a battle after a week, tell the teacher what you are seeing: homework that takes an hour and ends in tears is usually the wrong homework.")); __kws.extend(vec![("ask".to_string(), __data102()), ("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &__data103()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("essay-reference")); __args.push(py_tuple(vec![py_tuple(vec![py_str("reference"), py_str("references"), py_str("referencing"), py_str("cite"), py_str("citation"), py_str("bibliography"), py_str("sources")]), __data104()])); __args.push(py_str("Every claim that is not yours needs the author, the year and the page, and every source you used goes in the list at the end: author, year, title, publisher (or URL and the date you looked at it). Use the exact style your course asks for — Harvard, APA and MLA differ — and copy the format from your course guide rather than from memory. Quote sparingly and put the page number in, because a quote without one is the thing markers flag.")); __kws.extend(vec![("ask".to_string(), __data105()), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &py_tuple(vec![py_str("job"), py_str("jobs"), py_str("application"), py_str("applications"), py_str("interview"), py_str("interviews"), py_str("cv"), py_str("employer"), py_str("referee"), py_str("boss"), py_str("manager")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("learn-an-instrument-as-an-adult")); __args.push(py_tuple(vec![__data106(), py_tuple(vec![py_str("adult"), py_str("adults"), py_str("learning"), py_str("learn"), py_str("taking up"), py_str("take up"), py_str("late"), py_str("start"), py_str("starting"), py_str("old")])])); __args.push(py_str("It is not too late: adults pick up the theory faster than children, they just need shorter and more regular practice. Twenty minutes a day gets you a simple song in two or three months, and four chords in about a month. Have six lessons with a teacher first, so nobody has to unlearn your posture.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("too old"), py_str("too late"), py_str("start"), py_str("starting"), py_str("take up"), py_str("taking up"), py_str("begin"), py_str("worth it"), py_str("should i"), py_str("can i"), py_str("advice"), py_str("think about")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("evening-course")); __args.push(py_tuple(vec![__data107(), __data108()])); __args.push(py_str("Most evening courses are two or three hours of class a week plus about the same again at home, so plan on one evening and four or five hours a week. Ask for the guided learning hours before you enrol, because the college has to publish them, and check the fee: in England, English and maths up to GCSE are free for anyone who does not already have them, and a first level 2 or 3 qualification is often free if you are under 23 or on a low income. At the open evening, ask what the last group went on to do - that is what tells you whether it is worth the year.")); __kws.extend(vec![("ask".to_string(), __data109()), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data110())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("learn-guitar-first-steps")); __args.push(py_tuple(vec![py_tuple(vec![py_str("guitar"), py_str("guitars"), py_str("ukulele"), py_str("bass guitar"), py_str("learning guitar"), py_str("play guitar")]), __data111()])); __args.push(py_str("Buy a used acoustic from a shop that sets it up for you, 80 to 150 pounds: a 30 pound supermarket one sits too high off the neck, hurts your fingers and is why people stop after a fortnight. Practise fifteen minutes a day, not an hour once a week, and learn G, C, D and Em, which is most of the songs you already like. An app is fine for a month; six lessons after that fix your hands before the habits set.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("old"), py_str("late"), py_str("sell"), py_str("selling"), py_str("repair"), py_str("string"), py_str("strings"), py_str("broken"), py_str("tuning"), py_str("amp"), py_str("amplifier")])), ("ask".to_string(), __data112())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("handwriting"), py_tuple(vec![py_tuple(vec![py_str("handwriting"), py_str("hand writing"), py_str("write neatly"), py_str("neat writing"), py_str("messy writing"), py_str("illegible"), py_str("joined up writing"), py_str("cursive")]), __data113()]), py_str("Slow down by about a third for a week and use lined paper: most messy handwriting is speed, not hand shape. Then pick the two or three letters you join badly (r, n, m and your a's) and copy a paragraph a day, ten minutes, saying each letter as you write it. Check the shelf: a pen with a grip you actually hold, sitting up with both feet down and the paper turned slightly. If it is a child, check the grip first - a fist grip or a thumb over the top makes neat writing almost impossible - and use a pencil long enough to reach the table.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("school-bag"), py_tuple(vec![__data114(), __data115()]), py_str("Pack it the night before, not in the morning: the timetable, then the books for those lessons, then the kit or the letter that has to go back. One pocket for pens, one for paper, one for the phone, and nothing loose at the bottom - a bag you can empty in one look is a bag you stop losing things in.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ask-teacher")); __args.push(py_tuple(vec![py_tuple(vec![py_str("teacher"), py_str("tutor"), py_str("lecturer"), py_str("in class"), py_str("after class"), py_str("in the lesson"), py_str("in a lesson"), py_str("office hours"), py_str("my class"), py_str("in lectures")]), __data116()])); __args.push(py_str("Ask in person after class with one specific thing (\"I get the formula but I can't do the last question on the sheet\"), and say what you've already tried. Teachers help far more often than students expect, and an email the same day saying thank you makes them remember you next time.")); __kws.extend(vec![("avoid".to_string(), __data117())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("masters-cost")); __args.push(py_tuple(vec![py_tuple(vec![py_str("masters"), py_str("master's"), py_str("msc"), py_str("ma "), py_str("mba"), py_str("postgraduate"), py_str("postgrad"), py_str("postgraduate degree"), py_str("taught masters")]), __data118()])); __args.push(py_str("For a UK student, a taught masters is usually £9,000 to £15,000 for the year - around £8,000 to £12,000 for a classroom subject, £12,000 to £20,000 for a lab one, and an MBA can be £25,000 or more - plus living costs for a full-time year. A postgraduate master's loan from Student Finance covers up to about £12,500 of it, paid in three instalments, and it does not depend on your income. Ask the department about scholarships and fee waivers before you ask a bank, and check whether the course runs part time, because that spreads the cost over two years while you work.")); __kws.extend(vec![("avoid".to_string(), __data119()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("university-apply")); __args.push(py_tuple(vec![py_tuple(vec![py_str("university"), py_str("uni"), py_str("college application"), py_str("personal statement"), py_str("ucas")]), py_tuple(vec![py_str("apply"), py_str("application"), py_str("write"), py_str("how"), py_str("should"), py_str("start"), py_str("deadline")])])); __args.push(py_str("Apply on UCAS: one form, up to five courses, the personal statement and a reference, with 15 October for Oxford, Cambridge, medicine and veterinary science and mid-January for everything else. The statement is the part people worry about, and the rule is to start with the subject, not your childhood: what you have read, built or done that made you want it, then what you want to learn next. Two drafts and one reader who knows the subject beats five drafts alone.")); __kws.extend(vec![("avoid".to_string(), __data120()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-is-ucas")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ucas"), py_str("ucas form"), py_str("ucas application"), py_str("ucas account")]), py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("what does"), py_str("stand for"), py_str("mean"), py_str("means"), py_str("how does it work"), py_str("explain"), py_str("what do i need")])])); __args.push(py_str("UCAS is the one form that covers almost every full-time UK degree: you fill it in once, add up to five courses, write a personal statement, and a teacher or employer adds a reference. The fee is about £28. The early deadline is 15 October for Oxford, Cambridge, medicine, dentistry and veterinary science, and the main one is in mid-January. Offers come back between then and May, and you pick a firm choice and an insurance one.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("points"), py_str("tariff"), py_str("how many points"), py_str("clearing"), py_str("personal statement length"), py_str("how long"), py_str("how many words")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ucas-statement-length")); __args.push(py_tuple(vec![py_tuple(vec![py_str("personal statement")]), py_tuple(vec![py_str("how long"), py_str("how many words"), py_str("how many characters"), py_str("word count"), py_str("words"), py_str("characters"), py_str("maximum"), py_str("limit"), py_str("length")])])); __args.push(py_str("Four thousand characters including spaces, or forty-seven lines - whichever you hit first - and that is one side of A4 in plain prose. Write the draft long, then cut: the sentences that say nothing (\"ever since I was young\") go first, then any description of the course you can read on the university's own page. Paste it into a character counter rather than guessing, and leave room for the edit.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("cv"), py_str("c v"), py_str("resume"), py_str("résumé"), py_str("tax return"), py_str("taxes")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("maths-homework")); __args.push(py_tuple(vec![py_tuple(vec![py_str("maths"), py_str("math"), py_str("algebra"), py_str("equation"), py_str("equations"), py_str("geometry"), py_str("homework problem")]), py_tuple(vec![py_str("help"), py_str("solve"), py_str("understand"), py_str("explain"), py_str("how"), py_str("stuck")])])); __args.push(py_str("Ask them to show you the first step they'd try, then work one example together and let them do the next alone: that's the bit that teaches. If they're stuck on why, not how, draw it; almost every school maths idea has a picture (a bar for fractions, a graph for equations).")); __kws.extend(vec![("avoid".to_string(), __data121())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spelling-improve"), py_tuple(vec![py_tuple(vec![py_str("spelling"), py_str("spell"), py_str("spelling mistakes"), py_str("grammar")]), py_tuple(vec![py_str("improve"), py_str("better"), py_str("how"), py_str("fix"), py_str("help")])]), py_str("Keep a list of the words you actually get wrong (not a generic list), and test yourself on those five at a time by writing them, saying them and covering them. For the ones that refuse to stick, break them into a silly phrase or find the small word inside the big one (there's \"a rat\" in separate).")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-presentation")); __args.push(py_tuple(vec![py_tuple(vec![py_str("presentation"), py_str("talk"), py_str("speech")]), py_tuple(vec![py_str("school"), py_str("class"), py_str("students"), py_str("assembly"), py_str("nervous")])])); __args.push(py_str("Write the first sentence and the last sentence first, then three points in between, one card each. Practise out loud standing up, once to a person and once to a wall, and print the cards in big letters: looking up at people is what makes a school talk go well, and you can't look up at a phone.")); __kws.extend(vec![("avoid".to_string(), __data122())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("talkative-in-class")); __args.push(py_tuple(vec![__data123(), __data124()])); __args.push(py_str("Sit where the chat cannot start: front row, or beside someone you do not talk to, and ask the teacher to move you, which is a normal request and not a confession. Keep a corner of the page for the things you want to say and hold them to the end of the lesson - most of them keep, and the good ones come back to you. If it is answers blurting out, count to three and write the answer down first; the urge drops once it is on the page.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &py_tuple(vec![py_str("classmate"), py_str("classmates"), py_str("presentation"), py_str("presentations"), py_str("speech"), py_str("exam"), py_str("exams"), py_str("revision"), py_str("revise"), py_str("test"), py_str("grade"), py_str("grades")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("memorise-quotes")); __args.push(py_tuple(vec![py_tuple(vec![py_str("quotes"), py_str("quote"), py_str("quotations"), py_str("key quotes")]), py_tuple(vec![py_str("memorise"), py_str("memorize"), py_str("memorising"), py_str("learn"), py_str("remember"), py_str("english lit"), py_str("essay"), py_str("exam"), py_str("revise"), py_str("revision"), py_str("how")])])); __args.push(py_str("Pick eight to ten short ones you can use for more than one theme, and learn them in fours to six words rather than whole speeches - \"a plague o' both your houses\" is learnable, a fifteen-line speech is not. Say each aloud, look away and say it, then write it from memory; do three a day and cycle back through the rest. Practise the sentence you will build around it, because the quote alone earns nothing without the two lines of analysis after it.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("exam-timing")); __args.push(py_tuple(vec![py_tuple(vec![py_str("running out of time"), py_str("out of time"), py_str("time in exams"), py_str("didnt finish"), py_str("didn't finish"), py_str("no time left"), py_str("ran out of time"), py_str("finish the paper")]), py_tuple(vec![py_str("exam"), py_str("exams"), py_str("paper"), py_str("test"), py_str("revision"), py_str("how"), py_str("help"), py_str("tips"), py_str("stop")])])); __args.push(py_str("Give each question the minutes it is worth: one mark is about one minute, so a ten-mark question gets ten. Write the clock time you must move on by in the margin before you start, and move on when it arrives even if the answer is unfinished - a half answer plus the next question beats a perfect first one and three blanks. Read the whole paper first, do the question you know best, and keep the last five minutes for checking names, units and whether every question has an answer.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gap-year")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gap year"), py_str("year out"), py_str("take a year off"), py_str("defer"), py_str("deferring")]), __data125()])); __args.push(py_str("It is worth it when there is a reason you can name: a paid job that builds a skill, a season abroad with the cost worked out, or time for something you could not do later. Ask your university in writing to defer - most hold the offer for one year if you ask before the start date - and keep the place in mind, because reapplying is harder than deferring. Work out the money first: a year abroad is cheap only if the flights and the first month are paid for before you go.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("marks-question")); __args.push(py_tuple(vec![__data126(), py_tuple(vec![py_str("why"), py_str("teacher"), py_str("ask"), py_str("unfair"), py_str("lower"), py_str("explain"), py_str("dont know"), py_str("appeal"), py_str("grade"), py_str("grades"), py_str("feedback"), py_str("marked")])])); __args.push(py_str("Ask for the mark scheme, not for a favour: \"could I see which part of question 3 lost the marks?\" That is a normal request, and a good teacher will show you. Then ask for one example of what a full-mark answer looked like and rewrite that paragraph with it in front of you. If the marks still look wrong against the published scheme, ask the school for a remark in writing rather than arguing in the corridor.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &py_tuple(vec![py_str("stain"), py_str("washing"), py_str("shirt"), py_str("carpet"), py_str("wood"), py_str("paint")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("uni-cost")); __args.push(py_tuple(vec![py_tuple(vec![py_str("uni cost"), py_str("university cost"), py_str("cost of uni"), py_str("cost of university"), py_str("tuition fee"), py_str("tuition fees"), py_str("student debt"), py_str("how much does uni"), py_str("how much does university")]), py_tuple(vec![py_str("much"), py_str("total"), py_str("cost"), py_str("costs"), py_str("pay back"), py_str("worth"), py_str("how"), py_str("year"), py_str("all")])])); __args.push(py_str("In England the fee is £9,250 a year, so a three-year course is about £27,750, and living costs add roughly £9,000 to £12,000 a year on top. Nobody pays it up front: it is a loan repaid at 9% of what you earn above about £25,000, and what is left is written off after 40 years, so the monthly figure matters far more than the headline. Check the maintenance loan for your household income, and pay the deposit for accommodation out of the first instalment rather than a credit card.")); __kws.extend(vec![("avoid".to_string(), g_NOT_AN_EXAM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("remember-reading")); __args.push(py_tuple(vec![__data127(), __data128()])); __args.push(py_str("Read a section, then close the book and say its three points out loud before going on - rereading feels like study but the recall is what sticks. Turn each heading into a question first (\"what is a supply curve?\") and answer it from memory, then write one line per page in your own words. Come back to those lines after ten minutes, the next day and a week later, and you keep most of it with far less total time.")); __kws.extend(vec![("guard".to_string(), py_func("_the_page_is_what_was_forgotten", |args: &[Py]| _the_page_is_what_was_forgotten(args))), ("avoid".to_string(), __data129())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fail-exams")); __args.push(py_tuple(vec![__data130(), __data131()])); __args.push(py_str("Speak to your school or college on results day, not next month: they can tell you the resit date (usually November for maths and English, the following summer for most others) and whether the university or college will hold your place, which plenty will once they know. If you were a mark or two off a grade boundary, a remark can be worth it, but it can also go down, so ask the exams officer what the risk is. Failing one paper is not failing the year, and a plan made this week beats a summer of dread.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &py_tuple(vec![py_str("missed my offer"), py_str("missed the offer"), py_str("missed my place"), py_str("clearing"), py_str("my offer"), py_str("firm choice"), py_str("insurance choice"), py_str("rejected by"), py_str("rejected me")]))), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&_a_failure_of_something_else(&[understood.clone()]))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("college-or-university")); __args.push(py_tuple(vec![py_tuple(vec![py_str("college or university"), py_str("university or college"), py_str("difference between college"), py_str("college and university"), py_str("should i go to college"), py_str("college instead of university"), py_str("is college better than university"), py_str("btec or a levels"), py_str("sixth form or college"), py_str("what is college")]), __data132()])); __args.push(py_str("College is further education: A-levels, BTECs, T-levels, apprenticeships and access courses, usually for 16 to 18 year olds and mostly free until 19. University is a degree, three or four years, with tuition loans and maintenance loans to pay for it. The route that works for most people is whichever gets you the qualification the job you want actually asks for: a trade, nursing or design can go straight from college into work, while law, medicine and teaching need the degree. College first is not a step down - an access or foundation year costs less than a wasted degree and keeps the university route open.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how do i apply"), py_str("ucas"), py_str("student loan"), py_str("student finance"), py_str("tuition fee"), py_str("how much does university cost"), py_str("open day"), py_str("personal statement")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("essay-conclusion")); __args.push(py_tuple(vec![__data133(), __data134()])); __args.push(py_str("The last paragraph is where you say what it adds up to, not what you have already said: one sentence that answers the question in the title, one on the strongest evidence that got you there, then the wider point or what should happen next. Four or five sentences is enough, with no new evidence and no new quotation - if a fact belongs in the conclusion, it belongs in the body first.")); __kws.extend(vec![("guard".to_string(), py_func("_the_ending_is_written", |args: &[Py]| _the_ending_is_written(args))), ("avoid".to_string(), __data135()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mature-student")); __args.push(py_tuple(vec![__data136(), __data137()])); __args.push(py_str("Going back later is common, and the money is the first thing to check: a full-time degree is three years without a salary, so most people do it part time, one module at a time, or with the Open University. Student finance counts you as an independent student, so your own income decides the loan, and a course can credit what you already know. Do one evening class first, before you commit to three years.")); __kws.extend(vec![("avoid".to_string(), __data138())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("university-pass-mark")); __args.push(py_tuple(vec![__data139(), __data140()])); __args.push(py_func("_university_pass_mark", |args: &[Py]| _university_pass_mark(args))); __kws.extend(vec![("avoid".to_string(), op_add(&__data141(), &pyrt::xrefs::xget("knowledge.lookup", "A_WHAT_TO_WEAR"))), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); { let __b = Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.quantity", "a_figure_with_a_unit", &[understood.clone()]))); if !py_truthy(&__b) { __b } else { _the_degrees_are_heat(&[understood.clone()]) } } }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("course-choice")); __args.push(py_tuple(vec![__data142(), __data143()])); __args.push(py_str("Choose on the modules in years two and three, not the course title: read them, and if half of them bore you the course is wrong whatever it is called. Then check the entry requirements against your predicted grades, the contact hours and whether a placement or a year in industry is offered, and the graduate outcomes page for where last year's students went. Go to an open day and sit in one lecture if you can, because a department feels like something on the day. Apply to five with one dream and two safe choices, and keep the one you would be happy at if the grades go wrong.")); __kws.extend(vec![("ask".to_string(), __data144()), ("avoid".to_string(), __data145())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clearing")); __args.push(py_tuple(vec![py_tuple(vec![py_str("clearing"), py_str("clearing place"), py_str("clearing course"), py_str("a place at university"), py_str("which university"), py_str("adjustment")]), __data146()])); __args.push(py_str("Clearing is how you get a place when you have no offer or your grades missed: from July to October, search the official UCAS Clearing service for courses with vacancies, then ring the university yourself with your UCAS personal ID, your grades and the course code - an offer made on that call is a real offer. Have the list and your grades in front of you, ring three or four in one morning, and add the one you want in UCAS Track before you relax. If you already have a place and want a better one, that is adjustment, which is the same search with a deadline of five days.")); __kws.extend(vec![("ask".to_string(), __data147()), ("avoid".to_string(), __data148())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("first-class")); __args.push(py_tuple(vec![py_tuple(vec![py_str("first class"), py_str("first in my degree"), py_str("get a first"), py_str("2:1"), py_str("2.1"), py_str("second class"), py_str("degree classification"), py_str("final year"), py_str("dissertation")]), __data149()])); __args.push(py_str("The marks come from the same three things in every module: answer the exact question that was set, argue one clear line rather than describe everything you know, and quote the reading to back it. Get the marking criteria for each module and treat it as the checklist, then use the feedback on your first essay - the same weakness costs marks three times if nobody fixes it. Go to the lectures and seminars, because the person marking it notices, and start the dissertation in the summer before final year: a first is usually decided by the dissertation and two strong modules, not by all of them.")); __kws.extend(vec![("ask".to_string(), __data150()), ("avoid".to_string(), __data151())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("lab-report")); __args.push(py_tuple(vec![py_tuple(vec![py_str("lab report"), py_str("lab reports"), py_str("write up"), py_str("write-up"), py_str("practical report"), py_str("experiment write up"), py_str("practical write up"), py_str("biology practical"), py_str("chemistry practical"), py_str("physics practical")]), __data152()])); __args.push(py_str("Write it in this order and the marks look after themselves: aim in one sentence, method in numbered steps someone else could follow, results as a table plus one graph with labelled axes and units, then a conclusion that says whether the prediction was right, then an evaluation of what went wrong. Keep the method past tense and passive ('the solution was heated'), put numbers to the precision the instrument allows, and have one paragraph on sources of error with what you would change - that paragraph is the difference between a pass and a good mark. Do the graph and the evaluation first when you are short of time, because they carry the most marks.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("write"), py_str("writing"), py_str("sections"), py_str("layout"), py_str("format"), py_str("include"), py_str("need")])), ("avoid".to_string(), py_tuple(vec![py_str("essay"), py_str("coursework"), py_str("dissertation"), py_str("thesis"), py_str("book report"), py_str("report on"), py_str("business report"), py_str("annual report")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sixth-form-grades")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sixth form"), py_str("sixth-form"), py_str("college place"), py_str("a level place"), py_str("a level entry"), py_str("get into sixth form"), py_str("year 12"), py_str("year 11"), py_str("college"), py_str("gcse"), py_str("gcses"), py_str("a level")]), py_tuple(vec![py_str("what grades"), py_str("grades do i need"), py_str("grades"), py_str("requirements"), py_str("need"), py_str("entry"), py_str("points"), py_str("average"), py_str("enough"), py_str("to do"), py_str("get in"), py_str("apply")])])); __args.push(py_str("Most sixth forms and colleges ask for five GCSEs at grade 4 or above, including English and maths, and then a 6 or a 7 in the subjects you want to take at A level - so check the subject page, not the general prospectus, because the maths and science requirement is usually higher than the rest. If you miss the grades, ask about a level 3 course or a mixed programme, which often lets you move on to A level after a year, and apply early with a backup choice: places are offered from the autumn of year 11, not on results day.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what grades"), py_str("grades"), py_str("requirements"), py_str("need"), py_str("entry"), py_str("points"), py_str("enough"), py_str("get in"), py_str("apply")])), ("avoid".to_string(), __data153()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leaving-school-at-16")); __args.push(py_tuple(vec![__data154(), __data155()])); __args.push(py_str("In England they can leave school at 16 but not stop learning: education or training has to continue until 18, and a college course, an apprenticeship or a job with accredited training in the week all count, so it does not have to be more school. The realistic routes are A levels at a sixth form, a level 2 or 3 vocational course (BTEC, T level, NVQ) at college, or an apprenticeship, which is a paid job with the training built in and usually one day a week at college. Visit one college open day together before deciding, and ask about a level 2 course if the GCSEs are shaky, because that is a stepping stone to level 3 rather than a dead end.")); __kws.extend(vec![("ask".to_string(), __data156()), ("report".to_string(), Py::Bool(true)), ("avoid".to_string(), __data157())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("strict-teacher")); __args.push(py_tuple(vec![__data158(), __data159()])); __args.push(py_func("_strict_teacher", |args: &[Py]| _strict_teacher(args))); __kws.extend(vec![("ask".to_string(), __data160()), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); _a_school_line(&[understood.clone()]) })), ("avoid".to_string(), __data161())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    s_ENTRIES(__data40());
    fn __data162() -> Py { py_tuple(vec![py_str("memorise"), py_str("memorize"), py_str("memorising"), py_str("memorizing"), py_str("remember"), py_str("remembering"), py_str("learn"), py_str("learning"), py_str("memory"), py_str("brain"), py_str("forget"), py_str("forgetting"), py_str("forgetful"), py_str("revise"), py_str("revising"), py_str("revision")]) }
    s__REMEMBERS(__data162());
    s__MEMORISE(py_str("Test yourself instead of rereading: cover the page, say or write what you remember, then check. That one change does more than any technique. Then space it out - the same thing tomorrow, in three days, in a week - because the forgetting and the effort of getting it back is what makes it stay. Say it out loud or write it by hand rather than only reading, link a new fact to one you already know, and sleep on it: a night's sleep is when it gets filed."));
    fn __data163() -> Py { py_tuple(vec![py_str("faster"), py_str("quickly"), py_str("quick"), py_str("better"), py_str("stick"), py_str("sticks"), py_str("sticking"), py_str("stuck"), py_str("forget"), py_str("forgetting"), py_str("keeps forgetting"), py_str("keep forgetting"), py_str("more"), py_str("improve"), py_str("sharper"), py_str("technique"), py_str("techniques"), py_str("trick"), py_str("tricks"), py_str("tips"), py_str("how"), py_str("best"), py_str("effective"), py_str("way")]) }
    fn __data164() -> Py { py_tuple(vec![py_str("dates"), py_str("timeline"), py_str("history"), py_str("flashcard"), py_str("flashcards"), py_str("anki"), py_str("quizlet"), py_str("the job"), py_str("my job"), py_str("new job"), py_str("the role"), py_str("my role"), py_str("at work"), py_str("on the job"), py_str("the ropes"), py_str("ropes"), py_str("up to speed"), py_str("mnemonic"), py_str("mnemonics"), py_str("spelling"), py_str("spell"), py_str("password"), py_str("passwords"), py_str("names and faces"), py_str("faces and names"), py_str("dream"), py_str("dreams"), py_str("where i put"), py_str("where i left"), py_str("remember to"), py_str("remind me"), py_str("reminder"), py_str("birthday"), py_str("anniversary"), py_str("asleep"), py_str("tired"), py_str("sleepy"), py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("parent"), py_str("parents"), py_str("grandma"), py_str("grandmother"), py_str("grandad"), py_str("grandfather"), py_str("gran"), py_str("nan"), py_str("nana"), py_str("mother in law"), py_str("father in law"), py_str("touch type"), py_str("touch-type"), py_str("touch typing"), py_str("touch-typing"), py_str("typing"), py_str("keyboard"), py_str("wpm"), py_str("words a minute"), py_str("drive"), py_str("driving"), py_str("cook"), py_str("cooking"), py_str("bake"), py_str("baking"), py_str("swim"), py_str("swimming"), py_str("knit"), py_str("knitting"), py_str("sew"), py_str("sewing"), py_str("draw"), py_str("drawing"), py_str("dance"), py_str("dancing"), py_str("guitar"), py_str("piano"), py_str("drums"), py_str("violin"), py_str("ride a bike"), py_str("cycle"), py_str("cycling"), py_str("code"), py_str("coding"), py_str("programming"), py_str("touch typing"), py_str("juggle"), py_str("whistle"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("granny"), py_str("grandpa"), py_str("getting forgetful"), py_str("gets forgetful"), py_str("memory loss"), py_str("losing his memory"), py_str("losing her memory"), py_str("dementia"), py_str("alzheimers"), py_str("alzheimer's"), py_str("as you get older"), py_str("getting older"), py_str("my age"), py_str("oven"), py_str("cooked"), py_str("recipe"), py_str("recipes"), py_str("dish"), py_str("dishes"), py_str("tray"), py_str("trays"), py_str("tin"), py_str("tins"), py_str("roast"), py_str("kitchen")]) }
    fn __data165() -> Py { py_tuple(vec![py_str("faster"), py_str("quickly"), py_str("quick"), py_str("better"), py_str("stick"), py_str("sticks"), py_str("sticking"), py_str("stuck"), py_str("forget"), py_str("forgetting"), py_str("keeps forgetting"), py_str("keep forgetting"), py_str("more"), py_str("improve"), py_str("sharper"), py_str("technique"), py_str("techniques"), py_str("trick"), py_str("tricks"), py_str("tips"), py_str("how"), py_str("best"), py_str("effective"), py_str("way")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("memorise-faster")); __args.push(py_tuple(vec![py_tuple(vec![py_str("memorise"), py_str("memorize"), py_str("memorising"), py_str("memorizing"), py_str("remember"), py_str("remembering"), py_str("learn"), py_str("learning"), py_str("memory"), py_str("brain")]), __data163()])); __args.push(py_func("_memorise", |args: &[Py]| _memorise(args))); __kws.extend(vec![("avoid".to_string(), __data164()), ("ask".to_string(), __data165())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data166() -> Py { py_tuple(vec![py_str("wrong letters"), py_str("wrong letter"), py_str("types the wrong"), py_str("typing the wrong"), py_str("not working"), py_str("broken"), py_str("broke"), py_str("spilled"), py_str("sticky"), py_str("missing keys"), py_str("virtual"), py_str("on screen"), py_str("clean"), py_str("cleaning"), py_str("cleaned"), py_str("wipe"), py_str("dirty"), py_str("crumbs"), py_str("grubby"), py_str("disinfect"), py_str("phone keyboard"), py_str("autocorrect"), py_str("swipe"), py_str("symbol"), py_str("symbols"), py_str("special character"), py_str("special characters"), py_str("emoji"), py_str("emojis"), py_str("accent"), py_str("accents"), py_str("capital letter"), py_str("capital letters"), py_str("uppercase"), py_str("underscore"), py_str("hashtag"), py_str("slash"), py_str("asterisk"), py_str("tilde"), py_str("key combination"), py_str("shortcut"), py_str("shortcuts")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("learn-touch-typing")); __args.push(py_tuple(vec![py_tuple(vec![py_str("touch type"), py_str("touch-type"), py_str("touch typing"), py_str("touch-typing"), py_str("typing"), py_str("wpm"), py_str("words a minute")]), py_tuple(vec![py_str("learn"), py_str("learning"), py_str("teach myself"), py_str("improve"), py_str("quicker"), py_str("speed"), py_str("properly"), py_str("proper"), py_str("start"), py_str("practice"), py_str("practise")])])); __args.push(py_str("Start on the home keys and do not look down: left hand on a s d f, right hand on j k l and the semicolon, thumbs on the space bar, and cover your hands with a cloth if you keep peeking. Ten to fifteen minutes a day on a free drill site, typing real sentences rather than random letters, is enough to build it. Accuracy first and speed second, because a wrong key costs more time than a slow one: most people go from about 30 to 50 words a minute in a month of daily practice, and the last stretch comes from typing your own messages rather than the drills.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("how do i"), py_str("how can i"), py_str("how to"), py_str("whats the best way"), py_str("can i"), py_str("help"), py_str("tips"), py_str("faster"), py_str("better")])), ("avoid".to_string(), __data166()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__LATER_STUDY(stdlib_call("re", "compile", &[py_str("\\b(?:course|courses|college|university|uni|degree|gcse|gcses|a[ -]?levels?|night school|evening class|evening classes|access course|study|studying|lessons?|retrain|retraining|qualification|qualifications|apprenticeship|masters|phd|bookkeeping|nursing|tutor|lectures?|homework|exams?|revision|traineeship|nvq)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__CAREER_CHANGE(stdlib_call("re", "compile", &[py_str("\\b(?:too old|too late)\\b[^.?!]*\\b(?:become|becoming|train as|training as|retrain as|retraining as|qualify as|qualifying as|career change|change career|change jobs|switch careers?|switch to|move into|go into|get into)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_AGE_STUDY(stdlib_call("re", "compile", &[py_str("\\b(?:baby|babies|pregnan\\w*|ivf|fertility|children|kids|puppy|puppies|dog|dogs|cat|cats|driving|drink\\w*|tattoo|piercing|marathon|gym|weight|surgery|operation|piano|guitar|violin|viola|cello|drums|instrument|instruments|keyboard|ukulele|flute|business|businesses|startup|startups|company|shop|restaurant|cafe|dad|father|mum|mom|mother|parent|parents|grandma|grandmother|grandad|grandfather|footballer?|athlete|olympic|rugby|boxer|boxing|singer|dancer|actor|actress|model|influencer|youtuber|streamer)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AGE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:too old|too late|at my age|my age|the age|an age|age matter|age matters|age stop|age a problem|age hold|can i still|could i still|is it still|still worth|worth it at|at \\d{2}|in my (?:thirties|forties|fifties|sixties|seventies))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AGE_NUMBER(stdlib_call("re", "compile", &[py_str("\\b(\\d{2})\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data167() -> Py { py_tuple(vec![py_str("how much work"), py_str("how much time"), py_str("how much reading"), py_str("how many hours"), py_str("much study"), py_str("workload"), py_str("hours a week"), py_str("a week"), py_str("per week"), py_str("each week"), py_str("evenings a week"), py_str("commitment"), py_str("much effort"), py_str("the hours"), py_str("what hours"), py_str("hours be"), py_str("hours like")]) }
    s__COURSE_TIME(__data167());
    s_COURSE_HOURS(py_str("Budget two to three hours of reading and homework for every hour in class: a three-hour evening class is usually 9 to 12 hours a week including the class, and a part-time degree is 16 to 20 hours a week because it is half of a full-time course. The college has to publish those guided-learning hours on the course page, so ask for them before you enrol and pick the two evenings you would protect. Reading is what gets dropped first, so put it in the diary as an appointment rather than a hope."));
    fn __data168() -> Py { py_tuple(vec![py_str("at my age"), py_str("my age"), py_str("too old"), py_str("too old for"), py_str("too late"), py_str("mature student"), py_str("mature students"), py_str("older student"), py_str("am i too old"), py_str("is it too late"), py_str("age matter"), py_str("does age"), py_str("age stop me"), py_str("age too late")]) }
    fn __data169() -> Py { py_tuple(vec![py_str("sensible"), py_str("worth"), py_str("wise"), py_str("should i"), py_str("do it"), py_str("thing to do"), py_str("go for it"), py_str("go back"), py_str("study"), py_str("studying"), py_str("university"), py_str("uni"), py_str("college"), py_str("course"), py_str("retrain"), py_str("learn"), py_str("start"), py_str("doing"), py_str("become"), py_str("becoming"), py_str("be a"), py_str("qualify"), py_str("qualifying"), py_str("career"), py_str("careers"), py_str("change career"), py_str("switch")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("study-age")); __args.push(py_tuple(vec![__data168(), __data169()])); __args.push(py_func("_study_later", |args: &[Py]| _study_later(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("retire"), py_str("retiring"), py_str("retirement"), py_str("pension"), py_str("mortgage"), py_str("bus pass"), py_str("state pension"), py_str("driving licence"), py_str("insurance"), py_str("job interview"), py_str("cv")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data170() -> Py { py_tuple(vec![py_str("course"), py_str("courses"), py_str("college"), py_str("university"), py_str("uni"), py_str("evening class"), py_str("evening classes"), py_str("night school"), py_str("part time"), py_str("part-time"), py_str("degree"), py_str("study"), py_str("studying"), py_str("would it"), py_str("would that"), py_str("would i need"), py_str("do you think"), py_str("be"), py_str("is that"), py_str("is it"), py_str("that"), py_str("it")]) }
    fn __data171() -> Py { py_tuple(vec![py_str("sleep"), py_str("overtime"), py_str("shift"), py_str("shifts"), py_str("salary"), py_str("pay"), py_str("job"), py_str("jobs"), py_str("driving"), py_str("revise"), py_str("revising"), py_str("revision"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("gcse"), py_str("gcses"), py_str("a level"), py_str("a levels"), py_str("mocks"), py_str("homework"), py_str("work out my"), py_str("do i work")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("course-hours")); __args.push(py_tuple(vec![g__COURSE_TIME(), __data170()])); __args.push(py_func("_course_hours", |args: &[Py]| _course_hours(args))); __kws.extend(vec![("avoid".to_string(), __data171())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data172() -> Py { py_tuple(vec![py_str("english"), py_str("spanish"), py_str("french"), py_str("italian"), py_str("german"), py_str("portuguese"), py_str("polish"), py_str("welsh"), py_str("irish"), py_str("dutch"), py_str("a language"), py_str("languages"), py_str("language")]) }
    fn __data173() -> Py { py_tuple(vec![py_str("improve"), py_str("better"), py_str("learn"), py_str("learning"), py_str("fluent"), py_str("fluently"), py_str("speak"), py_str("speaking"), py_str("practice"), py_str("practise"), py_str("practising"), py_str("practicing"), py_str("study"), py_str("studying"), py_str("get better"), py_str("conversation"), py_str("conversational")]) }
    fn __data174() -> Py { py_tuple(vec![py_str("spell"), py_str("spelt"), py_str("spelling"), py_str("grammar"), py_str("comma"), py_str("apostrophe"), py_str("semicolon"), py_str("synonym"), py_str("meaning"), py_str("means"), py_str("difference between"), py_str("pronounce"), py_str("pronunciation"), py_str("translate"), py_str("translation"), py_str("essay"), py_str("coursework"), py_str("gcse"), py_str("a level"), py_str("ielts"), py_str("exam"), py_str("test"), py_str("do you speak"), py_str("can you speak"), py_str("why is")]) }
    fn __data175() -> Py { py_tuple(vec![py_str("improve"), py_str("better"), py_str("learn"), py_str("learning"), py_str("fluent"), py_str("fluently"), py_str("speak"), py_str("speaking"), py_str("practice"), py_str("practise"), py_str("practising"), py_str("practicing"), py_str("study"), py_str("studying"), py_str("get better"), py_str("conversation"), py_str("conversational"), py_str("how")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("improve-english")); __args.push(py_tuple(vec![__data172(), __data173()])); __args.push(py_str("Understand more before you try to speak, and speak badly early: ten minutes of listening or reading you mostly follow, every day, does more in a month than a grammar book. Learn whole phrases rather than single words - \"could I have...\", \"I've just...\" - because they come out in the right order without you building them. Then say them: out loud on your own, to a tutor once a week if you can, or to anyone patient. Get corrected on the mistakes you make every time, not on every mistake, and keep a list of your own bad habits. Watching something you already know with subtitles in the new language is the cheapest hour there is.")); __kws.extend(vec![("avoid".to_string(), __data174()), ("ask".to_string(), __data175())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data176() -> Py { py_tuple(vec![py_str("how many"), py_str("how much"), py_str("need"), py_str("needs"), py_str("needed"), py_str("college"), py_str("sixth form"), py_str("a level"), py_str("a levels"), py_str("course"), py_str("get in"), py_str("get into"), py_str("require"), py_str("required"), py_str("points"), py_str("university")]) }
    fn __data177() -> Py { py_tuple(vec![py_str("driving"), py_str("licence"), py_str("license"), py_str("theory test"), py_str("penalty"), py_str("speeding"), py_str("blood"), py_str("grade 4 mean"), py_str("what is a grade"), py_str("what does a grade"), py_str("oil"), py_str("oils"), py_str("engine"), py_str("engines"), py_str("car"), py_str("cars"), py_str("petrol"), py_str("diesel"), py_str("fuel"), py_str("tyre"), py_str("tyres"), py_str("brake"), py_str("brakes"), py_str("coolant"), py_str("wood"), py_str("timber"), py_str("steel"), py_str("meat"), py_str("beef"), py_str("steak"), py_str("coffee"), py_str("paint"), py_str("milk"), py_str("eggs"), py_str("whisky"), py_str("wine"), py_str("nurse"), py_str("nursing"), py_str("teacher"), py_str("teaching"), py_str("vet"), py_str("veterinary"), py_str("doctor"), py_str("lawyer"), py_str("solicitor"), py_str("midwife"), py_str("electrician"), py_str("ucas points"), py_str("ucas tariff"), py_str("ucs points")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gcse-points")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gcse points"), py_str("points for college"), py_str("points for a levels"), py_str("how many gcses"), py_str("how many gcse"), py_str("grades do i need"), py_str("grades to get in"), py_str("entry requirements"), py_str("entry grades"), py_str("what grades"), py_str("which grades"), py_str("gcse grades")]), __data176()])); __args.push(py_str("Colleges usually ask for five GCSEs at grade 4 or above, including maths and English, and a sixth form taking A levels often wants a 6 in the subjects you pick - a 6 in maths for maths, for instance. The points people talk about are just that arithmetic: a grade 4 is worth 4, so five grade 4s is 20 points, and each college publishes what it asks for. If English or maths is below 4, most colleges will let you take it alongside the course rather than turn you away, so ring admissions with the exact course and ask.")); __kws.extend(vec![("avoid".to_string(), __data177())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data178() -> Py { py_tuple(vec![py_str("how many"), py_str("how much"), py_str("need"), py_str("needed"), py_str("do i need"), py_str("are"), py_str("is"), py_str("does"), py_str("my"), py_str("points"), py_str("level"), py_str("a level"), py_str("btec"), py_str("university"), py_str("course"), py_str("offer"), py_str("get")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ucas-points")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ucas points"), py_str("ucas tariff"), py_str("ucas tariff points"), py_str("ucs points"), py_str("tariff points")]), __data178()])); __args.push(py_str("UCAS tariff points come from level 3 qualifications only, so GCSEs carry none: an A level is 48 points at grade A, 40 at B, 32 at C, 24 at D and 16 at E, and a BTEC Extended Diploma can score more than three A levels. Add them up across everything you have and compare with the course page, which usually prints both the points and the grades it asks for - when the grades are the offer, the points are only the summary.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("gcse"), py_str("gcses"), py_str("sixth form"), py_str("college place")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data179() -> Py { py_tuple(vec![py_str("how many"), py_str("should i take"), py_str("should i do"), py_str("need"), py_str("take"), py_str("do"), py_str("three"), py_str("four"), py_str("a level"), py_str("a levels"), py_str("subjects"), py_str("sixth form"), py_str("college")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-levels-count")); __args.push(py_tuple(vec![py_tuple(vec![py_str("how many a levels"), py_str("how many a level"), py_str("how many subjects at a level"), py_str("how many subjects for a level"), py_str("three or four a levels"), py_str("four a levels"), py_str("how many a levels should i take")]), __data179()])); __args.push(py_str("Three A levels is the standard full programme and what nearly every offer is written on; take four only if one is further maths, or if you are keeping a fourth for a course that asks for it, because a fourth A level rarely improves an offer and always costs marks in the other three. Two A levels plus a BTEC or an EPQ is a normal shape too - check the entry requirements for two or three courses before you choose the number.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("university"), py_str("uni"), py_str("ucas"), py_str("grade"), py_str("grades"), py_str("drop"), py_str("results day"), py_str("fail")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s__GRADES_ROUTE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("doctor"), py_str("medical school"), py_str("medicine")]), py_str("Medicine wants strong GCSEs, usually 5s and 6s including maths, English and a science, then A level chemistry plus one more science or maths, and some work experience in a care setting before you apply.")]), py_tuple(vec![py_tuple(vec![py_str("pharmacist"), py_str("pharmacy")]), py_str("Pharmacy wants A level chemistry and one other science, then the four-year MPharm and the year of training after it.")]), py_tuple(vec![py_tuple(vec![py_str("physiotherapist"), py_str("physiotherapy")]), py_str("Physiotherapy wants A level biology, most courses ask for a B or above at A level, and then the degree.")]), py_tuple(vec![py_tuple(vec![py_str("architect"), py_str("architecture")]), py_str("Architecture wants A level maths or physics, then the degree, then two years of practice before the final exam.")]), py_tuple(vec![py_tuple(vec![py_str("social worker"), py_str("social work")]), py_str("Social work wants GCSEs in English and maths, then a degree in social work, then registration with Social Work England.")]), py_tuple(vec![py_tuple(vec![py_str("police"), py_str("police officer")]), py_str("The police ask for a level 3 qualification - A levels or a BTEC both count - plus the fitness test and the background checks.")])]));
    fn __data180() -> Py { py_tuple(vec![py_str("what grades do i need to be a"), py_str("what gcses do i need to be a"), py_str("grades do i need to be a"), py_str("gcses do i need to be a"), py_str("how many gcses do i need to be a"), py_str("gcses do i need for"), py_str("gcses to be a"), py_str("grades to be a"), py_str("what grades for"), py_str("what gcses for"), py_str("what do i need to be a"), py_str("grades do i need for"), py_str("grades do i need to be an"), py_str("gcses do i need to be an"), py_str("what grades do i need to be an"), py_str("grades to be an"), py_str("how many gcses to be a"), py_str("how many gcses for")]) }
    fn __data181() -> Py { py_tuple(vec![py_str("doctor"), py_str("medicine"), py_str("medical school"), py_str("pharmacist"), py_str("pharmacy"), py_str("physiotherapist"), py_str("physiotherapy"), py_str("architect"), py_str("architecture"), py_str("social worker"), py_str("social work"), py_str("police"), py_str("police officer")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("job-grades")); __args.push(py_tuple(vec![__data180(), __data181()])); __args.push(py_func("_job_grades", |args: &[Py]| _job_grades(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data182() -> Py { py_tuple(vec![py_str("become"), py_str("becoming"), py_str("get into"), py_str("getting into"), py_str("qualify as"), py_str("train as"), py_str("want to be"), py_str("how do i become"), py_str("how i become"), py_str("what do i need to"), py_str("do i need a degree"), py_str("need a degree to"), py_str("degree to be"), py_str("degree to become"), py_str("a degree to"), py_str("to be a"), py_str("to be an")]) }
    fn __data183() -> Py { py_tuple(vec![py_str("doctor"), py_str("medical school"), py_str("medicine"), py_str("pharmacist"), py_str("pharmacy"), py_str("physiotherapist"), py_str("physiotherapy"), py_str("architect"), py_str("architecture"), py_str("social worker"), py_str("social work"), py_str("police"), py_str("police officer")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("job-route")); __args.push(py_tuple(vec![__data182(), __data183()])); __args.push(py_func("_job_grades", |args: &[Py]| _job_grades(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("pension"), py_str("mortgage"), py_str("insurance"), py_str("credit"), py_str("ucl"), py_str("an apprentice"), py_str("apprenticeship"), py_str("my teacher"), py_str("my son"), py_str("my daughter"), py_str("my mum"), py_str("my dad")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sixth-form-application")); __args.push(py_tuple(vec![py_tuple(vec![py_str("apply for college"), py_str("applying for college"), py_str("apply to college"), py_str("college application"), py_str("apply for sixth form"), py_str("apply to sixth form"), py_str("how do i apply for college")]), py_tuple(vec![py_str("how do i"), py_str("how can i"), py_str("do i"), py_str("when do i"), py_str("apply"), py_str("application"), py_str("deadline"), py_str("college"), py_str("sixth form"), py_str("place")])])); __args.push(py_str("Apply straight to the college, not through your school: every college has an online form on its own site, and for a September start the deadline is usually the spring before, with places filling from January. You apply before your GCSE results, the offer is conditional on the grades, and you confirm it on results day - so apply to two colleges if you are not sure, and ring admissions the week results come out if anything goes wrong.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("university"), py_str("uni"), py_str("ucas"), py_str("degree"), py_str("student finance"), py_str("job"), py_str("visa")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data184() -> Py { py_tuple(vec![py_str("get"), py_str("getting"), py_str("find"), py_str("finding"), py_str("apply"), py_str("applying"), py_str("start"), py_str("starting"), py_str("look"), py_str("looking"), py_str("search"), py_str("searching"), py_str("how"), py_str("where"), py_str("become"), py_str("becoming"), py_str("want"), py_str("wanted"), py_str("want to"), py_str("should")]) }
    fn __data185() -> Py { py_tuple(vec![py_str("degree apprenticeship"), py_str("degree apprenticeships"), py_str("worth it"), py_str("worth doing"), py_str("how many hours"), py_str("holidays"), py_str("time off"), py_str("overtime"), py_str("part time"), py_str("shift"), py_str("better than"), py_str("instead of university"), py_str("rather than university"), py_str("levy"), py_str("my employer"), py_str("our company"), py_str("we want to hire"), py_str("i employ")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("apprenticeship-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("apprenticeship"), py_str("apprenticeships"), py_str("apprentice"), py_str("apprentices")]), __data184()])); __args.push(py_str("Apply to them like jobs, because that is what they are: the vacancies are on the government Find an Apprenticeship service (gov.uk in England, apprenticeships.scot in Scotland, careerswales in Wales, nidirect in Northern Ireland), and big employers also advertise on their own sites with closing dates in the autumn and winter before a September start, so apply months early and to several. They come at four levels: GCSE-level, A-level, higher (the first two years of a degree) and degree, and the degree ones are competitive and want A-levels or a good BTEC. You need English and maths at GCSE for most of them, and if you missed those the employer usually puts you through them alongside the job. Pay is at least the apprentice minimum wage and rises with each year, so check the travel before you accept: a low wage plus a long commute is the thing people regret.")); __kws.extend(vec![("avoid".to_string(), __data185()), ("ask".to_string(), py_tuple(vec![py_str("how"), py_str("where"), py_str("what"), py_str("can i"), py_str("do i"), py_str("should i"), py_str("want to"), py_str("looking for"), py_str("help me"), py_str("advice")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_EXAM_WHEN(py_str("GCSE and A-level exams run from the middle of May to the middle of June, and the exact timetable for your subjects is published by the exam board - AQA, Edexcel, OCR or WJEC - about a year ahead, with the month-by-month dates on the board's site for the year you are sitting. Your school gives you your own timetable with the room and the start time once the entries are confirmed in February, and that is the one to go by. Results come out in the middle of August on a Thursday, with A-levels a week before GCSEs, and mocks sit in November or December before that."));
    fn __data186() -> Py { py_tuple(vec![py_str("predicted"), py_str("predicted grade"), py_str("predicted grades"), py_str("practice paper"), py_str("practice papers"), py_str("results day"), py_str("my results"), py_str("got my results"), py_str("failed"), py_str("fail"), py_str("clearing"), py_str("remark"), py_str("resit"), py_str("revise"), py_str("revising"), py_str("revision"), py_str("timetable for revision"), py_str("how do i"), py_str("best way")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("exam-dates")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gcse"), py_str("gcses"), py_str("a level"), py_str("a levels"), py_str("alevels"), py_str("national 5"), py_str("higher"), py_str("highers"), py_str("mock"), py_str("mocks"), py_str("exams")]), py_tuple(vec![py_str("when"), py_str("what date"), py_str("which date"), py_str("start"), py_str("starts"), py_str("starting"), py_str("begin"), py_str("begins"), py_str("timetable"), py_str("timings"), py_str("how long until")])])); __args.push(py_func("_exam_when", |args: &[Py]| _exam_when(args))); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_AN_EXAM(), &__data186())), ("ask".to_string(), py_tuple(vec![py_str("when"), py_str("what date"), py_str("which date"), py_str("dates"), py_str("how long until"), py_str("timetable")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_CAREER_SUBJECTS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("nurse"), py_str("nursing")]), py_str("Nursing wants five GCSEs at grade 4 or above, including English, maths and a science, and then a nursing degree, a nursing associate apprenticeship or a degree apprenticeship at a trust. Most universities also ask for A-levels or a BTEC in a science or health subject, and care experience - a care home, a hospice, a hospital volunteer - is what separates people with the same grades. Check the entry requirements for each course, because they differ.")]), py_tuple(vec![py_tuple(vec![py_str("midwife"), py_str("midwifery")]), py_str("Midwifery asks for the same five GCSEs at grade 4 or above, English, maths and a science among them, and then a midwifery degree or one of the new midwifery apprenticeships. It is one of the most competitive courses there is, so care or maternity experience and a week spent shadowing a midwife are worth more than an extra A-level.")]), py_tuple(vec![py_tuple(vec![py_str("doctor"), py_str("medicine"), py_str("medical school"), py_str("surgeon")]), py_str("Medicine wants strong GCSEs, usually 5s and 6s including maths, English and science, then A-level chemistry plus one more science or maths, because nearly every medical school requires chemistry. Then it is a five-year degree, the UCAT test in the summer before you apply, and work experience or volunteering in a care setting; the interview is about what you saw there.")]), py_tuple(vec![py_tuple(vec![py_str("teacher"), py_str("teaching"), py_str("primary school teacher"), py_str("secondary school teacher")]), py_str("Teaching wants GCSEs at grade 4 or above in English and maths, plus a science for primary, then a degree in a subject schools teach, then a PGCE or a school-based training place - or a teaching apprenticeship, where you are paid while you train. Primary teaching takes any degree but checks your curriculum subjects, and secondary wants the subject you will teach.")]), py_tuple(vec![py_tuple(vec![py_str("vet"), py_str("veterinary"), py_str("vet school")]), py_str("Veterinary medicine wants A-level chemistry and usually biology, grades A or A*, and weeks of work experience with animals - a farm, a stable, a vet practice - before you apply. The degree is five years and hard to get into, so apply to two or three vet schools and have a second choice you would genuinely take.")]), py_tuple(vec![py_tuple(vec![py_str("lawyer"), py_str("law"), py_str("solicitor"), py_str("barrister"), py_str("law degree")]), py_str("Law takes no particular A-levels: grades and a degree matter, and any subject is fine, so pick what you will do well in. The law degree (or any degree plus the conversion course) is followed by the SQE exams for solicitors or the bar course for barristers, and the thing that gets you a training place is experience - a law centre, a solicitor's office, mooting at university.")]), py_tuple(vec![py_tuple(vec![py_str("engineer"), py_str("engineering"), py_str("mechanical engineer"), py_str("civil engineer"), py_str("electrical engineer")]), py_str("Engineering wants A-level maths and physics, then an accredited MEng or BEng degree, and the accreditation is the part to check because it is what leads to chartership. The other route is a degree apprenticeship with an employer, which pays from day one and ends with the same qualification, or a technician apprenticeship if you would rather start with your hands.")]), py_tuple(vec![py_tuple(vec![py_str("accountant"), py_str("accountancy"), py_str("accounting"), py_str("chartered accountant")]), py_str("Accountancy asks for no fixed subjects, though maths and a numerical subject help, and there are three ways in: A-levels then an AAT or ACCA/ACA training contract, a degree then the same exams, or an accountancy degree apprenticeship where the employer pays for the exams. The exams are the qualification either way, so the route with the least debt is usually the apprenticeship.")]), py_tuple(vec![py_tuple(vec![py_str("paramedic"), py_str("paramedic science")]), py_str("Paramedic science wants a science A-level or an access course, and usually GCSEs at grade 4 or above in English, maths and science, then a three-year paramedic science degree - or the technician route inside an ambulance service, which pays while you train. Get a first aid certificate and some time in a care or St John Ambulance role first, because the courses and the interviews both ask what you have seen.")]), py_tuple(vec![py_tuple(vec![py_str("electrician"), py_str("plumber"), py_str("plumbing"), py_str("mechanic"), py_str("chef"), py_str("carpenter"), py_str("bricklayer"), py_str("joiner"), py_str("gas engineer")]), py_str("The trades ask for no GCSEs to start, though English and maths help with the theory and employers usually want 4s in them. The way in is an apprenticeship - level 2 then level 3 with an NVQ - or a college course alongside finding an employer yourself, and for gas, electrics and heating there is a registration scheme at the end that you cannot skip. Ring local firms and ask for a trial week: that is how most people get taken on.")])]));
    s_THE_GENERAL_CAREER(py_str("Check the entry requirements on the university or college page for the exact course, because they differ more than people expect - most want five GCSEs at grade 4 or above including English and maths, and A-levels or a BTEC in a subject that fits. Pick the subjects that keep two doors open rather than one, and ask a teacher or someone doing the job now what they actually needed."));
    fn __data187() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("nurse"), py_str("nursing")]), py_str("Three years for a nursing degree, two for a nursing associate course, or four on a degree apprenticeship where you are paid while you train."), py_str("A return-to-practice course, not the degree again: about 450 hours of supervised practice and a short theory module, which most trusts run over three to six months, and they tell you which one they take at the interview.")]), py_tuple(vec![py_tuple(vec![py_str("midwife"), py_str("midwifery")]), py_str("Three years for a midwifery degree, or four years on a midwifery apprenticeship, which pays while you train."), py_str("A return-to-practice course of a few months supervised in a maternity unit, rather than the degree again, if you have let your registration lapse.")]), py_tuple(vec![py_tuple(vec![py_str("dentist"), py_str("dentistry"), py_str("dental school"), py_str("dental surgeon")]), py_str("Five years at dental school, then a year of dental foundation training in a practice before you can work on your own, so six years from starting to being paid as a dentist."), py_str("A return-to-practice or the overseas registration exam rather than the degree again: around a year of study and supervised practice, and the General Dental Council says which route your qualification needs.")]), py_tuple(vec![py_tuple(vec![py_str("doctor"), py_str("medicine"), py_str("medical school"), py_str("surgeon")]), py_str("Five years at medical school, then two foundation years in a hospital before you can apply for specialty training, which runs from three to eight years depending on the field."), py_str("If you qualified as a doctor abroad, the route is the PLAB exams and then a couple of years in a training post, not a second degree.")]), py_tuple(vec![py_tuple(vec![py_str("teacher"), py_str("teaching"), py_str("primary school teacher"), py_str("secondary school teacher")]), py_str("Three years for a degree and then one more for a PGCE or a school-based training year, so four in all, or about the same on a teaching apprenticeship, which pays."), py_str("A refresher course of a few weeks rather than a second degree, if you taught before and let the registration go.")]), py_tuple(vec![py_tuple(vec![py_str("vet"), py_str("veterinary"), py_str("vet school")]), py_str("Five years at vet school, and there is no shorter route into it."), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("lawyer"), py_str("law"), py_str("solicitor"), py_str("barrister"), py_str("law degree")]), py_str("Three years for a law degree, then two years of qualifying work experience while you sit the SQE exams - so four to five years in all, or a year more if you do a conversion course instead of the law degree."), py_str("A conversion course of a year, then the same two years of qualifying work experience.")]), py_tuple(vec![py_tuple(vec![py_str("engineer"), py_str("engineering"), py_str("mechanical engineer"), py_str("civil engineer"), py_str("electrical engineer")]), py_str("Four years for an MEng, or three for a BEng and then a few years of supervised work to chartership."), py_str("A degree apprenticeship of four to six years, paid throughout, which is the usual way back in if you already work in the industry.")]), py_tuple(vec![py_tuple(vec![py_str("accountant"), py_str("accountancy"), py_str("accounting"), py_str("chartered accountant")]), py_str("Three to four years of exams, AAT first and then ACCA or ACA, taken while you work, so you are paid from the first year."), py_str("The same three to four years of exams, starting at the level you left off at, and a firm will usually take you on as a trainee while you sit them.")]), py_tuple(vec![py_tuple(vec![py_str("paramedic"), py_str("paramedic science")]), py_str("Three years for a paramedic science degree, or about two years as an associate ambulance practitioner inside a trust, which pays."), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("pharmacist"), py_str("pharmacy")]), py_str("Four years for an MPharm, then a year of foundation training in a pharmacy before you register."), py_str("A refresher placement and the registration assessment rather than the degree again.")]), py_tuple(vec![py_tuple(vec![py_str("physiotherapist"), py_str("physiotherapy")]), py_str("Three years for a physiotherapy degree, or two on an accelerated Masters if you already have a degree in a science."), py_str("A return-to-practice course of a few weeks, run by the Chartered Society of Physiotherapy, if you have been out of the register.")]), py_tuple(vec![py_tuple(vec![py_str("architect"), py_str("architecture")]), py_str("Three years for a degree, then two years of practical experience and the Part 3 exam - seven years in all, and the Part 3 is the part people underestimate."), py_str("The Part 2 and Part 3 years, on top of the experience you already have, rather than the whole degree again.")]), py_tuple(vec![py_tuple(vec![py_str("social worker"), py_str("social work")]), py_str("Three years for a social work degree, or two on a Masters if your first degree is in something related, and there are fast-track programmes that pay a salary while you train."), py_str("A return-to-practice course of a few weeks, and the register tells you what you need.")]), py_tuple(vec![py_tuple(vec![py_str("police"), py_str("police officer")]), py_str("No degree is needed: you apply to a force and train on the job, or take the three-year police constable degree apprenticeship, which is paid throughout."), py_str("You go back in at your previous rank where the force allows it, with a short refresher rather than the full training again.")]), py_tuple(vec![py_tuple(vec![py_str("electrician"), py_str("plumber"), py_str("plumbing"), py_str("gas engineer")]), py_str("Three to four years for a level 3 apprenticeship, which is how nearly everyone gets in, and the gas or electrics registration adds a year or two after that."), py_str("A level 2 refresher and the registration assessment rather than the whole apprenticeship.")]), py_tuple(vec![py_tuple(vec![py_str("chef")]), py_str("Two to three years as a commis chef through an apprenticeship or a college course, and you are paid from the first week."), py_str("A short refresher in a kitchen and the level 2 assessment rather than the whole thing again.")]), py_tuple(vec![py_tuple(vec![py_str("mechanic"), py_str("carpenter"), py_str("bricklayer"), py_str("joiner")]), py_str("Three to four years for a level 3 apprenticeship, which is how nearly everyone gets in."), py_str("A short refresher and the level 2 assessment rather than the whole apprenticeship.")])]) }
    s_JOB_LENGTHS(__data187());
    s__ASKS_THE_LENGTH(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how many years|how many months|how long|what length|years of study|length of the course|length of the training)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s__DONE_IT_BEFORE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:retrain|retraining|retrained|go back to|going back to|went back to|return to|returning to|returned to|years ago|used to be|i was a|i used to work|lapsed|let it lapse|on a break|career break|years out|years off|years away)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data188() -> Py { py_tuple(vec![py_str("gcse"), py_str("gcses"), py_str("a level"), py_str("a levels"), py_str("alevels"), py_str("subjects"), py_str("grades"), py_str("qualifications"), py_str("entry requirements"), py_str("requirements"), py_str("ucas points"), py_str("how many points"), py_str("points do i need"), py_str("points needed"), py_str("points for"), py_str("nurse"), py_str("nursing"), py_str("midwife"), py_str("midwifery"), py_str("doctor"), py_str("medicine"), py_str("a teacher"), py_str("teachers"), py_str("primary teacher"), py_str("secondary teacher"), py_str("teacher training"), py_str("teach primary"), py_str("teach secondary"), py_str("teaching"), py_str("vet"), py_str("veterinary"), py_str("lawyer"), py_str("solicitor"), py_str("barrister"), py_str("engineer"), py_str("engineering"), py_str("accountant"), py_str("accountancy"), py_str("accounting"), py_str("electrician"), py_str("plumber"), py_str("mechanic"), py_str("chef"), py_str("carpenter"), py_str("bricklayer"), py_str("joiner"), py_str("paramedic")]) }
    fn __data189() -> Py { py_tuple(vec![py_str("need"), py_str("needs"), py_str("needed"), py_str("require"), py_str("requires"), py_str("required"), py_str("be a"), py_str("be an"), py_str("to be a"), py_str("to be an"), py_str("become"), py_str("becoming"), py_str("get into"), py_str("getting into"), py_str("get on"), py_str("take"), py_str("want"), py_str("go back to"), py_str("going back to"), py_str("return to"), py_str("returning to"), py_str("should i take"), py_str("what"), py_str("which"), py_str("for"), py_str("how"), py_str("good career"), py_str("a good career"), py_str("career"), py_str("careers"), py_str("retrain"), py_str("retraining"), py_str("retrained"), py_str("go back to"), py_str("back to"), py_str("return to"), py_str("job"), py_str("jobs"), py_str("worth it"), py_str("going back to"), py_str("returning to"), py_str("after 10 years"), py_str("after five years"), py_str("after 5 years"), py_str("after ten years"), py_str("years off"), py_str("years out"), py_str("years away")]) }
    fn __data190() -> Py { py_tuple(vec![py_str("point of any"), py_str("point of it"), py_str("point of this"), py_str("point of all"), py_str("point of that"), py_str("point anymore"), py_str("no point"), py_str("any point"), py_str("f1"), py_str("formula 1"), py_str("formula one"), py_str("grand prix"), py_str("motogp"), py_str("podium"), py_str("pole position"), py_str("try"), py_str("tries"), py_str("goal"), py_str("goals"), py_str("touchdown"), py_str("conversion"), py_str("rugby"), py_str("football"), py_str("soccer"), py_str("cricket"), py_str("netball"), py_str("basketball"), py_str("hockey"), py_str("tennis"), py_str("league"), py_str("score"), py_str("scored"), py_str("scoring"), py_str("season"), py_str("licence"), py_str("license"), py_str("penalty points"), py_str("speeding"), py_str("driving"), py_str("banned"), py_str("parked"), py_str("road"), py_str("motorway"), py_str("insurance"), py_str("ask a teacher"), py_str("ask my teacher"), py_str("ask the teacher"), py_str("ask a lecturer"), py_str("ask my lecturer"), py_str("ask the lecturer"), py_str("ask my tutor"), py_str("ask a tutor"), py_str("email my teacher"), py_str("email a teacher"), py_str("email my lecturer"), py_str("email my tutor"), py_str("ask for help"), py_str("for help"), py_str("struggling"), py_str("how do i ask"), py_str("a question for"), py_str("ask a question"), py_str("choose"), py_str("choosing"), py_str("choice of"), py_str("pick"), py_str("picking"), py_str("my options"), py_str("options evening"), py_str("options form"), py_str("complain"), py_str("complaint"), py_str("complaining"), py_str("rude"), py_str("unfair"), py_str("shouted at me"), py_str("picked on"), py_str("my grades are"), py_str("results day"), py_str("i got a"), py_str("got a grade"), py_str("bad grade"), py_str("failed"), py_str("resit"), py_str("remark"), py_str("revise"), py_str("revising"), py_str("revision"), py_str("my dog"), py_str("my cat"), py_str("my pet"), py_str("the dog"), py_str("the cat"), py_str("puppy"), py_str("teach my"), py_str("teaching my"), py_str("teach a dog"), py_str("teach the dog"), py_str("teach him"), py_str("teach her"), py_str("teach them"), py_str("teach it"), py_str("how to teach"), py_str("teach someone"), py_str("teach me"), py_str("kitten"), py_str("rabbit"), py_str("hamster"), py_str("horse"), py_str("pony"), py_str("livestock"), py_str("boiler"), py_str("leak"), py_str("leaking"), py_str("dripping"), py_str("blocked"), py_str("fuse"), py_str("fuse box"), py_str("rewire"), py_str("wiring"), py_str("socket"), py_str("sockets"), py_str("radiator"), py_str("drain"), py_str("toilet"), py_str("tap"), py_str("shower"), py_str("washing machine"), py_str("find a"), py_str("finding a"), py_str("how do i find"), py_str("where do i find"), py_str("recommend"), py_str("recommended"), py_str("recommendation"), py_str("hire"), py_str("hiring"), py_str("a good plumber"), py_str("a good electrician"), py_str("a good builder"), py_str("a good decorator"), py_str("a good roofer"), py_str("trustworthy"), py_str("reputable"), py_str("cowboy"), py_str("quote"), py_str("quotes"), py_str("tradesperson"), py_str("tradesman"), py_str("a decent plumber"), py_str("a reliable plumber"), py_str("find a good"), py_str("find a reliable"), py_str("find a decent"), py_str("i need a plumber"), py_str("i need an electrician"), py_str("i need a mechanic"), py_str("call a plumber"), py_str("call an electrician"), py_str("call a mechanic"), py_str("car needs"), py_str("car wont"), py_str("car won't"), py_str("team"), py_str("teams"), py_str("league"), py_str("netball"), py_str("football"), py_str("rugby"), py_str("hockey"), py_str("cricket"), py_str("climbing"), py_str("bouldering"), py_str("piano"), py_str("guitar"), py_str("violin"), py_str("flute"), py_str("music"), py_str("ballet"), py_str("dancing"), py_str("drums"), py_str("singing"), py_str("road"), py_str("path"), py_str("flight"), py_str("hotel"), py_str("restaurant"), py_str("steak"), py_str("coffee"), py_str("beef"), py_str("petrol"), py_str("diesel"), py_str("what does a"), py_str("what do a"), py_str("do all day"), py_str("day to day"), py_str("a day in the life"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("shopping"), py_str("shop for"), py_str("bank"), py_str("joint"), py_str("savings"), py_str("current account"), py_str("my account"), py_str("the account"), py_str("an account"), py_str("account number"), py_str("account details"), py_str("my accounts"), py_str("online accounts"), py_str("google"), py_str("facebook"), py_str("instagram"), py_str("snapchat"), py_str("tiktok"), py_str("twitter"), py_str("linkedin"), py_str("social media"), py_str("gmail"), py_str("outlook"), py_str("see a doctor"), py_str("my doctor"), py_str("the doctor"), py_str("book a doctor"), py_str("book"), py_str("booking"), py_str("register with"), py_str("doctor's"), py_str("gp"), py_str("surgery"), py_str("nhs"), py_str("hospital"), py_str("appointment"), py_str("waiting list"), py_str("referral"), py_str("i feel"), py_str("im feeling"), py_str("i am feeling")]) }
    fn __data191() -> Py { py_tuple(vec![py_str("become"), py_str("becoming"), py_str("be a"), py_str("be an"), py_str("to be a"), py_str("to be an"), py_str("get into"), py_str("getting into"), py_str("qualify"), py_str("qualified"), py_str("train"), py_str("training"), py_str("study"), py_str("studying"), py_str("subjects"), py_str("subject"), py_str("grades"), py_str("grade"), py_str("gcse"), py_str("gcses"), py_str("a level"), py_str("a levels"), py_str("alevels"), py_str("qualifications"), py_str("qualification"), py_str("requirements"), py_str("entry requirements"), py_str("ucas"), py_str("ucas points"), py_str("points"), py_str("degree"), py_str("university"), py_str("college"), py_str("apprenticeship"), py_str("course"), py_str("career"), py_str("careers"), py_str("work experience"), py_str("what does"), py_str("what do i need"), py_str("what will i need"), py_str("how long does"), py_str("go back to"), py_str("going back to"), py_str("return to"), py_str("returning to"), py_str("job"), py_str("jobs"), py_str("worth it")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("career-subjects")); __args.push(py_tuple(vec![__data188(), __data189()])); __args.push(py_func("_career_subjects", |args: &[Py]| _career_subjects(args))); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_NOT_AN_EXAM(), &g_MOCK()), &__data190())), ("ask".to_string(), __data191()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data192() -> Py { py_tuple(vec![py_str("what"), py_str("whats"), py_str("what is"), py_str("mean"), py_str("means"), py_str("definition"), py_str("how long"), py_str("how many words"), py_str("how do i"), py_str("how to"), py_str("start"), py_str("starting"), py_str("begin"), py_str("structure"), py_str("write"), py_str("writing"), py_str("word count"), py_str("length"), py_str("supervisor"), py_str("plan"), py_str("planning"), py_str("is a"), py_str("for")]) }
    fn __data193() -> Py { py_tuple(vec![py_str("marks"), py_str("mark"), py_str("grade"), py_str("grades"), py_str("extension"), py_str("extend"), py_str("deadline"), py_str("late"), py_str("penalty"), py_str("plagiarism"), py_str("reference"), py_str("supervisor never"), py_str("supervisor wont"), py_str("supervisor does not"), py_str("supervisor doesnt"), py_str("supervisor ignores"), py_str("supervisor is useless"), py_str("supervisor is awful"), py_str("my supervisor"), py_str("my tutor never")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dissertation")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dissertation"), py_str("dissertations"), py_str("final year project"), py_str("final project"), py_str("undergraduate thesis"), py_str("masters thesis"), py_str("doctoral thesis"), py_str("capstone project")]), __data192()])); __args.push(py_str("A dissertation is one long piece of work - usually 8,000 to 12,000 words for a bachelor's and up to 20,000 for a master's - where you answer one question of your own with evidence, rather than showing what you know about a subject. It is marked on the question, the reading and the argument, in that order. Start with a question narrow enough to answer in the word count, then a plan of four or five chapters, then read for that plan only. Book two supervisor meetings before Christmas and take a written note of what was agreed at each one: the students who fall behind are the ones who go quiet, not the ones who ask.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_REVISION(), &__data193())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data194() -> Py { py_tuple(vec![py_str("what"), py_str("whats"), py_str("what is"), py_str("mean"), py_str("means"), py_str("definition"), py_str("how long"), py_str("how many years"), py_str("worth it"), py_str("should i"), py_str("funding"), py_str("funded"), py_str("cost"), py_str("how do i"), py_str("apply"), py_str("difference"), py_str("is a"), py_str("do i need"), py_str("for")]) }
    fn __data195() -> Py { py_tuple(vec![py_str("marks"), py_str("grade"), py_str("grades"), py_str("essay"), py_str("revision"), py_str("exam"), py_str("gcse"), py_str("a level"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("my son"), py_str("my daughter"), py_str("or get a job"), py_str("or a job"), py_str("phd or"), py_str("masters or"), py_str("or a masters"), py_str("worth it"), py_str("worth the money"), py_str("worth the cost"), py_str("worth the debt"), py_str("interview"), py_str("should i do"), py_str("should i"), py_str("is it worth"), py_str("should i bother"), py_str("interview for"), py_str("prepare for an interview")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("phd")); __args.push(py_tuple(vec![py_tuple(vec![py_str("phd"), py_str("ph.d"), py_str("doctorate"), py_str("doctoral"), py_str("dphil"), py_str("postgraduate research"), py_str("phd thesis")]), __data194()])); __args.push(py_str("A PhD is three to four years of full-time original research under a supervisor, ending in a thesis of about 60,000 to 80,000 words and a spoken exam called a viva: it trains you to produce new knowledge, and it is not required for most jobs outside research. Look for a funded place first - a studentship pays fees and a stipend, and the funded ones are advertised on FindAPhD and on university pages - because self-funding a PhD is expensive for the career it buys. Write to a named supervisor with a two-page proposal and ask what they have funding for, rather than applying to a department in general.")); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_REVISION(), &__data195())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data196() -> Py { py_tuple(vec![py_str("read"), py_str("reading"), py_str("study"), py_str("studying"), py_str("revision"), py_str("revise"), py_str("exam"), py_str("exams"), py_str("uni"), py_str("university"), py_str("school"), py_str("college"), py_str("how do i"), py_str("help"), py_str("cope"), py_str("takes forever"), py_str("slow"), py_str("takes me"), py_str("struggle"), py_str("struggling")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dyslexia-reading")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dyslexic"), py_str("dyslexia"), py_str("reading is slow"), py_str("reading takes forever"), py_str("reading takes me ages"), py_str("words jump"), py_str("words move on the page"), py_str("letters move"), py_str("read slowly"), py_str("slow reader")]), __data196()])); __args.push(py_str("Ask for the formal help first: a dyslexia assessment at university goes through the disability service, and it unlocks extra time in exams, a reader or text-to-speech software, and often a grant for equipment - at school it is the SENCO, and it costs you nothing to ask. For the reading itself, use text-to-speech so the page reads to you while you follow with a finger, take the chapter in 20-minute blocks with the summary read first, and ask for the reading list in advance so you are never reading against a deadline. Being slow at decoding is not being slow at thinking, and the extra time is the standard fix, not a favour.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my kid"), py_str("year old"), py_str("toddler"), py_str("baby"), py_str("spelling test"), py_str("handwriting")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_EXTENSION_ASK(py_tuple(vec![py_str("extension"), py_str("extensions"), py_str("extend"), py_str("mitigating circumstances"), py_str("mitigation"), py_str("extenuating circumstances"), py_str("special circumstances"), py_str("more time"), py_str("late submission"), py_str("submit late"), py_str("hand in late"), py_str("extra time")]));
    fn __data197() -> Py { py_tuple(vec![py_str("essay"), py_str("essays"), py_str("coursework"), py_str("assignment"), py_str("assignments"), py_str("dissertation"), py_str("project write-up"), py_str("lab report"), py_str("thesis"), py_str("submission"), py_str("portfolio"), py_str("exam"), py_str("exams"), py_str("module")]) }
    s_ESSAY_WORDS(__data197());
    fn __data198() -> Py { py_tuple(vec![py_str("how do i"), py_str("ask"), py_str("asking"), py_str("get"), py_str("apply"), py_str("tutor"), py_str("lecturer"), py_str("teacher"), py_str("professor"), py_str("module leader"), py_str("uni"), py_str("university"), py_str("school"), py_str("college"), py_str("who do i"), py_str("can i"), py_str("what do i"), py_str("email"), py_str("form")]) }
    fn __data199() -> Py { py_tuple(vec![py_str("boss"), py_str("manager"), py_str("client"), py_str("at work"), py_str("my job"), py_str("council"), py_str("tax return"), py_str("visa"), py_str("rent"), py_str("landlord"), py_str("planning permission"), py_str("mortgage"), py_str("insurance")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("essay-extension")); __args.push(py_tuple(vec![op_add(&g_EXTENSION_ASK(), &py_tuple(vec![py_str("deadline"), py_str("deadlines")])), op_add(&g_ESSAY_WORDS(), &__data198())])); __args.push(py_str("Ask the module leader or tutor before the deadline, in writing, with the date you need - and ask what the evidence rule is at your place, because it is a form with a name: mitigating circumstances, extenuating circumstances or special circumstances. Send the form and the evidence (a GP note, a letter, a hospital appointment) in one email, and keep a copy. The standard extension is a week or two for a short illness; longer or repeated problems go to the mitigating circumstances panel rather than to your tutor. Ask early: most universities will not accept a claim made after the deadline has passed, and asking costs nothing.")); __kws.extend(vec![("avoid".to_string(), __data199()), ("ask".to_string(), op_add(&g_ESSAY_WORDS(), &py_tuple(vec![py_str("mitigating circumstances"), py_str("mitigating"), py_str("extenuating"), py_str("tutor"), py_str("lecturer"), py_str("module leader"), py_str("professor"), py_str("uni"), py_str("university"), py_str("school"), py_str("college")]))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data200() -> Py { py_tuple(vec![py_str("missed the deadline"), py_str("missed my deadline"), py_str("miss a deadline"), py_str("missed a deadline"), py_str("handed in late"), py_str("submitted late"), py_str("late submission"), py_str("missed the submission"), py_str("deadline passed"), py_str("deadline has passed"), py_str("deadline gone"), py_str("didnt submit"), py_str("didn't submit"), py_str("wasnt in on time"), py_str("wasn't in on time"), py_str("missed my coursework"), py_str("missed the coursework"), py_str("missed my essay"), py_str("missed the essay"), py_str("missed my assignment"), py_str("missed my submission"), py_str("coursework deadline"), py_str("essay deadline"), py_str("deadline for my essay"), py_str("deadline for the essay"), py_str("deadline for my coursework")]) }
    fn __data201() -> Py { py_tuple(vec![py_str("uni"), py_str("university"), py_str("school"), py_str("college"), py_str("module"), py_str("tutor"), py_str("lecturer"), py_str("teacher"), py_str("what happens"), py_str("what now"), py_str("what do i do"), py_str("will i fail"), py_str("fail"), py_str("penalty"), py_str("capped"), py_str("resit"), py_str("appeal"), py_str("talk to"), py_str("email")]) }
    fn __data202() -> Py { py_tuple(vec![py_str("boss"), py_str("manager"), py_str("client"), py_str("at work"), py_str("my job"), py_str("council"), py_str("tax return"), py_str("visa"), py_str("rent"), py_str("landlord"), py_str("planning permission"), py_str("mortgage"), py_str("insurance"), py_str("car"), py_str("mot"), py_str("report"), py_str("the report"), py_str("the figures"), py_str("client"), py_str("board"), py_str("audit"), py_str("invoice")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("missed-essay-deadline")); __args.push(py_tuple(vec![__data200(), op_add(&g_ESSAY_WORDS(), &__data201())])); __args.push(py_str("Email the module leader today, before they chase you: say the piece is late, give the reason in one sentence and name the date you will submit. Most places take 10% off a day up to a cap of a pass mark for a few days, and the late penalty is usually shown on your marks breakdown - check your module handbook for the exact rule rather than guessing. If the reason was illness or something serious, ask for mitigating circumstances now: it can remove the late penalty and give you a resit as if for the first time. Silence is the only version where nothing can be fixed.")); __kws.extend(vec![("avoid".to_string(), __data202()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data203() -> Py { py_tuple(vec![py_str("hours"), py_str("hour"), py_str("how many hours"), py_str("how long"), py_str("time off"), py_str("holiday"), py_str("holidays"), py_str("overtime"), py_str("week"), py_str("a week"), py_str("work"), py_str("can i work"), py_str("allowed to work"), py_str("part time"), py_str("evenings"), py_str("weekends"), py_str("shifts")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("apprentice-hours")); __args.push(py_tuple(vec![py_tuple(vec![py_str("apprentice"), py_str("apprentices"), py_str("apprenticeship"), py_str("apprenticeships")]), __data203()])); __args.push(py_str("At least 30 hours a week of paid work, and an apprentice under 18 cannot be asked to do more than 40 hours a week or more than 8 hours a day. Everyone still counts the 28 days' paid holiday a year, bank holidays on top if the contract says so, and the off-the-job training - about a fifth of the hours - happens inside your paid time rather than after it. Overtime is allowed if you are over 18 and it is in the contract, and the 48-hour average week limit applies as it does to any job.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data204() -> Py { py_tuple(vec![py_str("due"), py_str("deadline"), py_str("deadlines"), py_str("hand in"), py_str("hand-in"), py_str("handing in"), py_str("submit"), py_str("submission"), py_str("last minute"), py_str("behind"), py_str("havent started"), py_str("haven t started"), py_str("not started"), py_str("havent done"), py_str("haven t done"), py_str("left it"), py_str("running out of time"), py_str("no time")]) }
    fn __data205() -> Py { py_tuple(vec![py_str("due"), py_str("deadline"), py_str("hand in"), py_str("hand-in"), py_str("submit"), py_str("submission"), py_str("last minute"), py_str("behind"), py_str("havent started"), py_str("haven t started"), py_str("not started"), py_str("havent done"), py_str("haven t done"), py_str("left it"), py_str("running out of time"), py_str("no time")]) }
    fn __data206() -> Py { py_tuple(vec![py_str("revise"), py_str("revising"), py_str("revision"), py_str("exam"), py_str("exams"), py_str("mock"), py_str("past paper"), py_str("how do i"), py_str("how to"), py_str("how can i"), py_str("what should i write"), py_str("structure"), py_str("introduction"), py_str("conclusion"), py_str("reference"), py_str("references"), py_str("citation"), py_str("cite"), py_str("plagiaris"), py_str("spelling"), py_str("grammar"), py_str("word count"), py_str("grade"), py_str("marks"), py_str("my son"), py_str("my daughter"), py_str("my kid"), py_str("my child"), py_str("my boy"), py_str("my girl"), py_str("my children"), py_str("his teacher"), py_str("her teacher"), py_str("the teacher"), py_str("school report"), py_str("at work"), py_str("my boss"), py_str("to my boss"), py_str("the boss"), py_str("for work"), py_str("client")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-deadline-and-nothing-written")); __args.push(py_tuple(vec![py_tuple(vec![py_str("essay"), py_str("essays"), py_str("assignment"), py_str("assignments"), py_str("coursework"), py_str("dissertation"), py_str("piece of work"), py_str("thesis"), py_str("words"), py_str("word count")]), __data204(), __data205()])); __args.push(py_str("Work backwards from the deadline: decide the three or four points that answer the question, put a heading for each, write the middle sections badly and fast, then the introduction and the conclusion last, then read it out loud once. Ask for a short extension today rather than after the deadline and put the request in writing, because most tutors will say yes before the date and almost none can after it. A complete plain answer scores more than a perfect first half, so hand in every section, even thin.")); __kws.extend(vec![("avoid".to_string(), __data206()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data207() -> Py { py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("boy"), py_str("girl"), py_str("toddler"), py_str("youngest"), py_str("eldest"), py_str("reception"), py_str("reception class"), py_str("preschool"), py_str("nursery"), py_str("year old"), py_str("year-old"), py_str("4 year old"), py_str("5 year old"), py_str("6 year old"), py_str("7 year old")]) }
    s__CHILD(__data207());
    s__AGE_SAID(py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2})\\s*(?:year|yr)s?[\\s-]*(?:old|young)\\b")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:is|at|aged|age|turned|nearly)\\s+(\\d{1,2})\\b")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2})\\s*(?:and|,)\\s*(?:cant|can't|cannot|doesnt|doesn't|still|only|hasnt|hasn't)\\b")]).unwrap_or_else(|e| exc::raise(e))]));
    s__WRITING_HAND(py_str("Give the hand other work first - threading, cutting, playdough, big chalk on the path - and practise the name in short bursts with a chunky pencil and a dotted version to trace, five minutes at a time and no more."));
    fn __data208() -> Py { py_tuple(vec![py_str("write his name"), py_str("write her name"), py_str("write their name"), py_str("write his own name"), py_str("write her own name"), py_str("writes his name"), py_str("writes her name"), py_str("writing his name"), py_str("writing her name"), py_str("write the name"), py_str("write name"), py_str("write his"), py_str("write her"), py_str("cant write"), py_str("can't write"), py_str("cannot write"), py_str("cant hold a pencil"), py_str("cant hold the pencil"), py_str("pencil grip"), py_str("holds a pencil"), py_str("hold a pencil"), py_str("holding a pencil"), py_str("pencil properly"), py_str("letter formation"), py_str("write letters"), py_str("writing letters"), py_str("recognise his name"), py_str("recognise her name"), py_str("know his letters"), py_str("know her letters"), py_str("learn to write"), py_str("scribbles"), py_str("wont write"), py_str("hates writing"), py_str("hate writing")]) }
    fn __data209() -> Py { py_tuple(vec![py_str("my handwriting"), py_str("handwriting"), py_str("my writing"), py_str("neater"), py_str("neatly"), py_str("spelling"), py_str("grammar"), py_str("signature"), py_str("copy"), py_str("font"), py_str("illegible"), py_str("my notes"), py_str("my essay"), py_str("school place"), py_str("choose a school"), py_str("apply for a school"), py_str("adult"), py_str("i cant write")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("child-cant-write-his-name")); __args.push(py_tuple(vec![g__CHILD(), __data208()])); __args.push(py_func("the_writing_card", |args: &[Py]| the_writing_card(args))); __kws.extend(vec![("avoid".to_string(), __data209()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data210() -> Py { py_tuple(vec![py_str("cant sleep"), py_str("can't sleep"), py_str("not sleeping"), py_str("wont sleep"), py_str("won't sleep"), py_str("up late"), py_str("staying up"), py_str("awake"), py_str("tired"), py_str("nervous"), py_str("worried"), py_str("worry"), py_str("stressed"), py_str("stress"), py_str("anxious"), py_str("panic"), py_str("crying"), py_str("cries"), py_str("not eating"), py_str("off their food"), py_str("revision")]) }
    fn __data211() -> Py { py_tuple(vec![py_str("cant sleep"), py_str("can't sleep"), py_str("not sleeping"), py_str("wont sleep"), py_str("won't sleep"), py_str("up late"), py_str("staying up"), py_str("awake"), py_str("tired"), py_str("nervous"), py_str("worried"), py_str("worry"), py_str("stressed"), py_str("stress"), py_str("anxious"), py_str("panic"), py_str("crying"), py_str("cries"), py_str("not eating"), py_str("off their food"), py_str("nerves")]) }
    fn __data212() -> Py { py_tuple(vec![py_str("my exams"), py_str("my revision"), py_str("im revising"), py_str("i cant sleep"), py_str("i can't sleep"), py_str("i am revising"), py_str("toddler"), py_str("baby"), py_str("cot"), py_str("crib"), py_str("nursery"), py_str("my exam"), py_str("my test"), py_str("my driving test"), py_str("blood test"), py_str("pregnancy test"), py_str("urine")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("child-exam-nerves")); __args.push(py_tuple(vec![py_tuple(vec![py_str("exam"), py_str("exams"), py_str("tests"), py_str("mocks"), py_str("revision"), py_str("revising"), py_str("gcses"), py_str("gcse"), py_str("a levels"), py_str("a level"), py_str("sats"), py_str("school tests")]), op_add(&g__CHILD(), &py_tuple(vec![py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("year old"), py_str("year-old"), py_str("15 year old"), py_str("16 year old"), py_str("17 year old"), py_str("she"), py_str("he")])), __data210()])); __args.push(py_str("Take the pressure off the night, not the work: the revision is done by then, so the job is a fixed stop time, the phone out of the room, and something that is not revision for the hour before bed. Keep the worry concrete - ask what they are most afraid of forgetting, write it down and leave it on the desk, because a written worry stops the circling. If the exams are weeks away and sleep is still gone, or they are not eating or cannot get to school, tell their tutor, because that is worth more than any technique.")); __kws.extend(vec![("ask".to_string(), __data211()), ("avoid".to_string(), __data212()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data213() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("when"), py_str("which"), py_str("why"), py_str("can i"), py_str("could i"), py_str("should"), py_str("should i"), py_str("do i"), py_str("is it"), py_str("too late"), py_str("wrong"), py_str("regret"), py_str("hate"), py_str("want to"), py_str("worth it"), py_str("thinking of"), py_str("considering"), py_str("decided"), py_str("allowed"), py_str("possible"), py_str("advice"), py_str("help"), py_str("best"), py_str("first year"), py_str("second year"), py_str("third year"), py_str("year 10"), py_str("year 11"), py_str("year 12"), py_str("year 13"), py_str("halfway through"), py_str("just started"), py_str("already started")]) }
    s__ASKED_TO_MOVE(__data213());
    s__THE_ANNOUNCEMENT(py_tuple(vec![py_str("changing subject"), py_str("changing subjects"), py_str("changing the subject"), py_str("changing topic"), py_str("changing the topic"), py_str("change topic"), py_str("change of subject"), py_str("moving on"), py_str("next subject"), py_str("different subject"), py_str("different topic")]));
    fn __data214() -> Py { py_tuple(vec![py_str("what about"), py_str("how about"), py_str("what"), py_str("and"), py_str("but"), py_str("so"), py_str("ok"), py_str("okay"), py_str("anyway"), py_str("right"), py_str("well"), py_str("now"), py_str("then"), py_str("also"), py_str("plus"), py_str("just"), py_str("let us"), py_str("lets"), py_str("let's"), py_str("can we"), py_str("can i"), py_str("could we"), py_str("shall we"), py_str("maybe")]) }
    s__LEAD_IN(__data214());
    fn __data215() -> Py { py_tuple(vec![py_str("change"), py_str("changing"), py_str("changed"), py_str("switch"), py_str("switching"), py_str("swap"), py_str("swapping"), py_str("transfer"), py_str("transferring"), py_str("move"), py_str("moving"), py_str("drop"), py_str("dropping"), py_str("quit"), py_str("quitting"), py_str("hate"), py_str("hating"), py_str("wrong"), py_str("regret"), py_str("regretting"), py_str("stuck with"), py_str("change my a level"), py_str("change a level"), py_str("change my a levels"), py_str("change a levels"), py_str("change my subject"), py_str("change subjects"), py_str("change my course"), py_str("change course"), py_str("change courses"), py_str("change my degree"), py_str("change degrees"), py_str("changing subjects"), py_str("changing course"), py_str("changing courses"), py_str("changing degree"), py_str("changing my degree"), py_str("changing my course"), py_str("changing my subjects"), py_str("switching subject"), py_str("switching subjects"), py_str("switching course"), py_str("switching courses"), py_str("switching degree"), py_str("switching degrees"), py_str("switching my course"), py_str("switch course"), py_str("switch courses"), py_str("switch degree"), py_str("switch degrees"), py_str("wrong a levels"), py_str("wrong a level"), py_str("hate my course"), py_str("hate my uni"), py_str("hate my degree"), py_str("regret my degree"), py_str("regret my course"), py_str("too late to change")]) }
    fn __data216() -> Py { py_tuple(vec![py_str("a level"), py_str("a levels"), py_str("a-level"), py_str("as level"), py_str("subjects"), py_str("subject"), py_str("course"), py_str("courses"), py_str("degree"), py_str("uni"), py_str("university"), py_str("college"), py_str("gcse"), py_str("gcses"), py_str("sixth form"), py_str("year 12"), py_str("year 13"), py_str("year 10"), py_str("first year"), py_str("second year"), py_str("apprenticeship")]) }
    fn __data217() -> Py { py_tuple(vec![py_str("drop a subject"), py_str("drop my subject"), py_str("drop an a level"), py_str("drop a level"), py_str("drop one of my a levels"), py_str("drop one of my"), py_str("dropping a level"), py_str("dropping an a level"), py_str("drop as level"), py_str("quit a subject"), py_str("give up a subject"), py_str("dropping a subject"), py_str("change the subject"), py_str("the subject to"), py_str("course offer"), py_str("university offer"), py_str("uni offer"), py_str("firm offer"), py_str("job"), py_str("jobs"), py_str("boss"), py_str("career"), py_str("careers"), py_str("my work"), py_str("at work"), py_str("address"), py_str("bank"), py_str("energy"), py_str("supplier"), py_str("broadband"), py_str("mobile"), py_str("insurance"), py_str("password"), py_str("email"), py_str("car"), py_str("boiler"), py_str("vet"), py_str("dog"), py_str("cat"), py_str("recipe"), py_str("diet"), py_str("gym"), py_str("running"), py_str("hair"), py_str("doctor"), py_str("medication"), py_str("dressing"), py_str("nappy"), py_str("light bulb"), py_str("bedding"), py_str("taps"), py_str("course of"), py_str("steroids"), py_str("antibiotics"), py_str("how i feel"), py_str("my mood")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("change-subject-or-course")); __args.push(py_tuple(vec![__data215(), __data216()])); __args.push(py_str("You can usually still change it, and the sooner you ask the easier it is: at school or college, go to the head of sixth form or the exams officer, and a change is normally possible in the first two or three weeks of Year 12 - after that it is their call, and how much you would catch up that decides it. At university, ask your personal tutor or the course leader about transferring to another course at the same place before you think about leaving, which is usually easier than a fresh application through UCAS or Clearing. Changing after a full year costs money as well as time, because student finance does not normally fund a second first year.")); __kws.extend(vec![("ask".to_string(), g__ASKED_TO_MOVE()), ("guard".to_string(), py_func("_asks_to_move", |args: &[Py]| _asks_to_move(args))), ("avoid".to_string(), __data217()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    s_CATCHING_UP(py_str("Find the first topic you stopped understanding rather than the newest one, because everything after it is built on it: go through the textbook contents or the topic list and mark the last thing that made sense. Then do three things - tell the teacher you have fallen behind and ask which two topics matter most for the next test, spend twenty minutes a day on those two only, and get one session with a classmate who is good at them. Catching up is a short list worked through patiently, not the whole course read again."));
    fn __data218() -> Py { py_tuple(vec![py_str("fallen behind"), py_str("fell behind"), py_str("behind at school"), py_str("behind in class"), py_str("catch up at school"), py_str("catch up in class"), py_str("catch up at college"), py_str("catch up on school"), py_str("catch up on schoolwork"), py_str("catch up on classwork"), py_str("catch up on lessons"), py_str("catch up on the work"), py_str("catch up on everything at school"), py_str("catch up on everything in class"), py_str("catch up after being off"), py_str("catching up at school"), py_str("catching up in class"), py_str("missed a lot"), py_str("missed lessons"), py_str("missed school"), py_str("missed classes"), py_str("a lot of school"), py_str("months off school"), py_str("off school for months"), py_str("off school for weeks"), py_str("gaps in my knowledge"), py_str("dont understand anything"), py_str("don't understand anything"), py_str("no idea what is going on"), py_str("lost at school"), py_str("struggling at school"), py_str("struggling in class"), py_str("school work"), py_str("schoolwork"), py_str("classwork"), py_str("coursework")]) }
    fn __data219() -> Py { py_tuple(vec![py_str("how do i"), py_str("how"), py_str("catch up"), py_str("catching up"), py_str("help"), py_str("behind"), py_str("school"), py_str("class"), py_str("college"), py_str("teacher"), py_str("subject"), py_str("start"), py_str("should i"), py_str("what do i do")]) }
    fn __data220() -> Py { py_tuple(vec![py_str("how"), py_str("catch up"), py_str("catching up"), py_str("help"), py_str("behind"), py_str("start"), py_str("what do i do"), py_str("should i"), py_str("missed a lot"), py_str("missed school"), py_str("missed lessons"), py_str("missed classes"), py_str("a lot of school"), py_str("months off school"), py_str("off school for months"), py_str("off school for weeks")]) }
    fn __data221() -> Py { py_tuple(vec![py_str("ill"), py_str("illness"), py_str("sick"), py_str("hospital"), py_str("diagnosis"), py_str("anxiety"), py_str("depressed"), py_str("depression"), py_str("adhd"), py_str("autism"), py_str("dyslexia"), py_str("statement"), py_str("ehcp"), py_str("sen"), py_str("expelled"), py_str("excluded"), py_str("bully"), py_str("bullied"), py_str("bullying"), py_str("rent"), py_str("mortgage"), py_str("bill"), py_str("bills"), py_str("credit"), py_str("loan"), py_str("sleep"), py_str("penalty"), py_str("fine"), py_str("council"), py_str("truancy"), py_str("attendance"), py_str("snow"), py_str("snowing"), py_str("snowed"), py_str("snow day"), py_str("closed"), py_str("shut"), py_str("strike"), py_str("inset day"), py_str("holidays"), py_str("half term"), py_str("summer holiday"), py_str("bored"), py_str("my children"), py_str("my kids"), py_str("the children"), py_str("the kids"), py_str("toddler"), py_str("nursery"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("child"), py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("stepson"), py_str("stepdaughter"), py_str("my boy"), py_str("my girl"), py_str("his school"), py_str("her school"), py_str("their school"), py_str("his teacher"), py_str("her teacher"), py_str("their teacher"), py_str("email"), py_str("emails"), py_str("inbox"), py_str("deadline"), py_str("deadlines"), py_str("boss"), py_str("job"), py_str("jobs"), py_str("project"), py_str("projects"), py_str("client"), py_str("clients"), py_str("paperwork"), py_str("timesheet"), py_str("overtime"), py_str("workload"), py_str("office"), py_str("meeting"), py_str("meetings"), py_str("shift"), py_str("my work"), py_str("at work"), py_str("on my work"), py_str("for work"), py_str("with work"), py_str("his work"), py_str("her work")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("catching-up-at-school")); __args.push(py_tuple(vec![__data218(), __data219()])); __args.push(g_CATCHING_UP()); __kws.extend(vec![("ask".to_string(), __data220()), ("avoid".to_string(), __data221()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _past_papers_reply(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The past-papers answer, opening with the timing when the line asks when to start.");
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PAST_PAPERS_ASK_WHEN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("when"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("should"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("start"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("begin"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&line.clone(), &py_str("how long"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&line.clone(), &py_str("how many"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("soon"))) } } } } } } } } } } } } } }) {
    return g__PAST_PAPERS_WHEN();
    }
    return g__PAST_PAPERS();
    Py::None
}
pub fn _fractions_reply(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut asking: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The fractions answer, opening with the operation the line asks about.");
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_FRACTIONS_FIRST());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    asking = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asking.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return said.clone();
    }
    }
    return g_FRACTIONS_MULTIPLY();
    Py::None
}
pub fn _the_ending_is_written(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about the end of something written, not the end of anything.");
    clean = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&op_add(&g__A_CONCLUSION_SAID(), &g__SOMETHING_WRITTEN()));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_translation_is_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut content: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks for the meaning of a foreign word, not when it uses an English one.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&g__A_MEANING_IS_ASKED(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    content = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__ENGLISH_FURNITURE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    named = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LANGUAGE_NAMES()) { let mut name: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &name.clone()), &py_str("(?![a-z])")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NATIVE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_MEANING_IS_ASKED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bsay\\b|\\bhow do you\\b|\\bhow do i\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } } } }) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[content.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NATIVE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _what_does_it_mean(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut concept: Py = Py::None;
    let mut english: Py = Py::None;
    let mut language: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Answer \"what does <foreign word> mean\" from the phrases table, backwards.");
    if py_truthy(&Py::Bool(!py_truthy(&_a_translation_is_asked(&[understood.clone()])))) {
    return Py::None;
    }
    clean = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NATIVE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __unpk2 = py_unpack(&geti(&g_NATIVE(), &word.clone()), 2, None);
    language = __unpk2[0].clone();
    concept = __unpk2[1].clone();
    english = if py_truthy(&Py::Bool(py_eq(&concept.clone(), &py_str("i don't understand")))) { py_str("I don't understand") } else { concept.clone() };
    return { let mut __s = String::new(); __s.push_str("That is "); __s.push_str(&fstr(&geti(&g_LANGUAGES(), &language.clone()), -1, "")); __s.push_str(" for '"); __s.push_str(&fstr(&english.clone(), -1, "")); __s.push_str("'. It is a phrase people say a lot, so learn it whole and out loud rather than word by word, and the reply you get back will tell you whether it landed."); py_string(__s) };
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_word_is_being_asked_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks what a word means, and the word in it is the thing asked about.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__MEANING_FRAME(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NATIVE()) { let mut word: Py = __item.clone(); if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g__ENGLISH_TOO(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[_find(&[said.clone(), g_LANGUAGES()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("\""))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("“"))) } } } };
    Py::None
}
pub fn _find(args: &[Py]) -> Py {
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
    let mut table: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "table") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: table"))) };
    let mut name: Py = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(table.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return name.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _concept__wrapped(args: &[Py]) -> Py {
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
    let mut alternative: Py = Py::None;
    let mut alternatives: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The PHRASES key the line is asking about, with word edges respected either side.");
    let __iter1 = py_iter_or_raise(&g_CONCEPTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    alternatives = __unpk2[0].clone();
    name = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&alternatives.clone());
    for __item in __iter3 {
    alternative = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[alternative.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return name.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_language_alone(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is a language and nothing else: `actually french`, `what about spanish`.\n\n    A correction that swaps one language for another is a language line, and the phrasebook is\n    the answer it wants. #11118's fence on \"what was the french revolution\" stood the phrasebook\n    down for every line that named a language without a say or translate in it, so `actually\n    french` after \"im learning spanish\" lost the French phrases and got \"I'm listening - say more\n    if it helps.\" The history question it was written for names a language among other words;\n    this one has nothing else in it.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__CORRECTION_FURNITURE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LANGUAGE_NAMES(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _a_phrase_is_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut text: Py = Py::None;
    let _ = py_str("True when the line asks for a phrase in another language rather than mentioning one.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&mcall(&g__PHRASE_ASK(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&py_call(&g__concept(), &[text.clone()])) {
    return Py::Bool(true);
    }
    return _a_language_alone(&[text.clone()]);
    Py::None
}
pub fn _say_in_language(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut concept: Py = Py::None;
    let mut language: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut question: Py = Py::None;
    let mut say: Py = Py::None;
    let _ = py_str("Answer \"how do you say <phrase> in <language>\" from the PHRASES table.");
    clean = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&{ let __b = mcall(&g__ABOUT_THE_PLATE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__SAY_OR_TRANSLATE(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    language = _find(&[clean.clone(), g_LANGUAGES()]);
    if py_truthy(&Py::Bool(!py_truthy(&language.clone()))) {
    return Py::None;
    }
    concept = py_call(&g__concept(), &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&concept.clone()))) {
    return { let mut __s = String::new(); __s.push_str("Learn "); __s.push_str(&fstr(&geti(&g_LANGUAGES(), &language.clone()), -1, "")); __s.push_str(" as a set of whole phrases rather than word by word, and learn them out loud: 'hello', 'please', 'thank you', 'sorry, I don't understand', 'do you speak English?' and 'how much is that?' cover most of a short trip. Ask me any of those in one language and I will give you the phrase and how to say it."); py_string(__s) };
    }
    pair = mcall(&mcall(&g_PHRASES(), "get", &[concept.clone(), py_dict(vec![])]), "get", &[language.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pair.clone()))) {
    return { let mut __s = String::new(); __s.push_str("Learn "); __s.push_str(&fstr(&geti(&g_LANGUAGES(), &language.clone()), -1, "")); __s.push_str(" as a set of whole phrases rather than word by word, and learn them out loud: 'hello', 'please', 'thank you', 'sorry, I don't understand', 'do you speak English?' and 'how much is that?' cover most of a short trip. Ask me any of those in one language and I will give you the phrase and how to say it."); py_string(__s) };
    }
    let __unpk1 = py_unpack(&pair.clone(), 2, None);
    phrase = __unpk1[0].clone();
    say = __unpk1[1].clone();
    question = if py_truthy(&Py::Bool(!py_eq(&concept.clone(), &py_str("i don't understand")))) { concept.clone() } else { py_str("I don't understand") };
    return { let mut __s = String::new(); __s.push_str("'"); __s.push_str(&fstr(&mcall(&question.clone(), "capitalize", &[]), -1, "")); __s.push_str("' in "); __s.push_str(&fstr(&geti(&g_LANGUAGES(), &language.clone()), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&phrase.clone(), -1, "")); __s.push_str(" — say it roughly \""); __s.push_str(&fstr(&say.clone(), -1, "")); __s.push_str("\". Learn it out loud a few times rather than reading it, and people will understand you even if the pronunciation is not perfect."); py_string(__s) };
    Py::None
}
pub fn _a_number_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[-\\s]"), word.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    part = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__NUMBER_WORDS(), &part.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_degrees_are_heat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("False when \"N degrees\" is a temperature rather than a qualification.\n\n    A number before \"degrees\" is a temperature unless the line says something else about\n    studying: \"35 degrees in my exam\" is a mark, \"wedding in italy and 35 degrees\" is the\n    weather. The word \"degree\" itself cannot count as the study word, because the phrase\n    carries it.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    words = call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&py_set(vec![py_str("degree"), py_str("degrees")]), &words.clone())))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__TEMPERATURE_DEGREES(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { _a_number_word(&[words.clone()]) } }))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&op_sub(&words.clone(), &py_set(vec![py_str("degree"), py_str("degrees")])), "intersection", &[g__EXAM_IN_THE_WORDS()])) {
    return Py::Bool(true);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:my|the|a|his|her|their|your)\\s+degree\\b|\\bdegree\\s+(?:in|course|exam)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_failure_of_something_else(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line fails at something that is not named as an exam.\n\n    \"i failed\" is a key of the exam-resit answer, and with the \"i\" and \"my\" of its second key\n    list it was enough on its own: \"im learning to drive at 40\" / \"i failed my test twice\" was\n    told to speak to the school or college on results day about a resit in maths and English,\n    and a swim badge went the same way. The exam words above are what makes a bare failure an\n    exam question; without one, the thing they failed at is the chat's own subject.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__FAILING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__EXAM_IN_THE_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_test_of_a_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when \"test\" in this line is the verb and the thing after it is the subject.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TEST_AS_A_VERB(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_TIME_AFTER_TEST(), "search", &[call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    return Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_LEARNING_OBJECTS())));
    Py::None
}
pub fn _unless_testing_a_thing(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let _ = py_str("The reply, unless the line is testing a thing rather than revising for a test.");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    if py_truthy(&_a_test_of_a_thing(&[understood.clone()])) {
    return py_str("");
    }
    return reply.clone();
    Py::None
}
    answer = {{ let answer = answer.clone(); let reply = reply.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), reply.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    return answer.clone();
    Py::None
}
pub fn _exam_nerves(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The exam nerves answer, with the body named first when the line is a symptom.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("knowledge.wording", "NERVES_BODY")) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&op_add(&pyrt::xrefs::xcall("knowledge.wording", "nerves_opening", &[said.clone(), py_str("before an exam"), py_str("adrenaline, not a bug")]), &py_str(": eat something plain, take water in with you, and breathe out slowly for a minute in the queue. ")), &g_EXAM_NERVES());
    }
    return g_EXAM_NERVES();
    Py::None
}
pub fn _somebody_else_forgets(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the forgetting in the line belongs to another person, not the speaker.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__FORGETTING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__OTHER_PEOPLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _grade_in(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut n: Py = Py::None;
    let mut next_two: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The grade number the line is about, or None when the number is a count.\n\n    A grade is a number on its own (\"a grade 4\", \"got a 5 in maths\"). A score is a number\n    with something either side of it (\"5 marks ... out of 10\", \"4 out of 9 in my mock\"),\n    and reading that as a GCSE grade tells a person their 4 out of 9 is a standard pass.\n    ");
    words = mcall(&mcall(&clean.clone(), "replace", &[py_str(","), py_str(" ")]), "split", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    n = __unpk2[0].clone();
    word = __unpk2[1].clone();
    match_ = stdlib_call("re", "fullmatch", &[py_str("([1-9])%?"), word.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    before = if py_truthy(&n.clone()) { geti(&words.clone(), &op_sub(&n.clone(), &Py::Int(1))) } else { py_str("") };
    after = if py_truthy(&op_lt(&op_add(&n.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &op_add(&n.clone(), &Py::Int(1))) } else { py_str("") };
    next_two = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &op_add(&n.clone(), &Py::Int(1)), &op_add(&n.clone(), &Py::Int(3)), &Py::None)]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_MARK_NOT_A_GRADE(), &before.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("%")]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_COUNTED_AFTER(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&next_two.clone(), "startswith", &[py_str("a level")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_GRADE_NAME(), "match", &[next_two.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_LETTER_GRADES(), "match", &[next_two.clone()]) } } } } } }) {
    continue;
    }
    return mcall(&match_.clone(), "group", &[Py::Int(1)]);
    }
    return Py::None;
    Py::None
}
pub fn _grade_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut grade: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "grade") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: grade"))) };
    let mut a: Py = Py::None;
    let _ = py_str("What one GCSE grade is worth, in the words a person wants to hear.");
    if py_truthy(&Py::Bool(py_eq(&grade.clone(), &py_str("9")))) {
    return py_str("A 9 is the top grade, above the old A*.");
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("7"), py_str("8")]), &grade.clone()))) {
    a = if py_truthy(&Py::Bool(py_eq(&grade.clone(), &py_str("8")))) { py_str("An") } else { py_str("A") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&a.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&grade.clone(), -1, "")); __s.push_str(" is a top grade, the old A and above."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&grade.clone(), &py_str("6")))) {
    return py_str("A 6 is a good pass, about the old B.");
    }
    if py_truthy(&Py::Bool(py_eq(&grade.clone(), &py_str("5")))) {
    return py_str("A 5 is a strong pass, the old high C, and over the line most colleges ask for.");
    }
    if py_truthy(&Py::Bool(py_eq(&grade.clone(), &py_str("4")))) {
    return py_str("A 4 is a standard pass, the old C: you are over the line most colleges ask for, and a 5 is the strong pass above it.");
    }
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&grade.clone(), -1, "")); __s.push_str(" is below a pass at GCSE. Most sixth forms and colleges take you on other subjects and have you retake English or maths alongside, and that retake is free until you are 18, so it does not close anything on its own."); py_string(__s) };
    Py::None
}
pub fn _score_out_of(args: &[Py]) -> Py {
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
    let mut got: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("A score and the total it is out of (\"4 out of 9\", \"5 marks out of 10\"), or None.\n\n    Half marks on a piece of work is a proportion, not a grade: the useful thing to say is how\n    much of the paper that is and which questions lost the rest, because one homework score\n    says nothing about where a person stands in the subject overall.\n    ");
    match_ = mcall(&g_SCORE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    got = __unpk1[0].clone();
    total = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __cmp21: Py = got.clone(); if py_truthy(&op_le(&Py::Int(1), &__cmp21)) { { let __cmp22: Py = total.clone(); if py_truthy(&op_le(&__cmp21, &__cmp22)) { op_le(&__cmp22, &Py::Int(200)) } else { Py::Bool(false) } } } else { Py::Bool(false) } })); if py_truthy(&__b) { __b } else { op_lt(&total.clone(), &Py::Int(5)) } }) {
    return Py::None;
    }
    return py_tuple(vec![got.clone(), total.clone(), call_builtin("round", &[op_truediv(&op_mul(&Py::Int(100), &got.clone()), &total.clone())]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _score_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut score: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "score") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: score"))) };
    let mut got: Py = Py::None;
    let mut part: Py = Py::None;
    let mut percent: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("How much of the paper that is, in the words a person uses about their own work.");
    let __unpk1 = py_unpack(&score.clone(), 3, None);
    got = __unpk1[0].clone();
    total = __unpk1[1].clone();
    percent = __unpk1[2].clone();
    part = if py_truthy(&{ let __cmp21: Py = percent.clone(); if py_truthy(&op_le(&Py::Int(40), &__cmp21)) { op_le(&__cmp21, &Py::Int(60)) } else { Py::Bool(false) } }) { py_str("about half") } else { if py_truthy(&op_gt(&percent.clone(), &Py::Int(60))) { py_str("most") } else { py_str("under half") } };
    return { let mut __s = String::new(); __s.push_str("That is "); __s.push_str(&fstr(&got.clone(), -1, "")); __s.push_str(" out of "); __s.push_str(&fstr(&total.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&part.clone(), -1, "")); __s.push_str(" the marks."); py_string(__s) };
    Py::None
}
pub fn _grade_answer(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut grade: Py = Py::None;
    let mut score: Py = Py::None;
    let _ = py_str("\"Is a grade 4 bad?\": what the number is worth, and what it opens up.");
    clean = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&mcall(&g_TOP_GRADES(), "search", &[clean.clone()])) {
    return Py::None;
    }
    score = _score_out_of(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_is(&score.clone(), &Py::None))) {
    return op_add(&_score_sentence(&[score.clone()]), &py_str(" Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time."));
    }
    grade = _grade_in(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&grade.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:gcses?|grades?|marks|score|scores|pass|passed|mocks?|exams?)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return py_str("GCSE grades run from 9 down to 1: 4 is a standard pass, 5 a strong one, and 7 to 9 are the top grades. Below 4 is not a pass yet, and English and maths can be retaken at college until you are 18. Tell me the grade and the subject and I will say what it opens up.");
    }
    if py_truthy(&op_ge(&grade.clone(), &py_str("4"))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_grade_sentence(&[grade.clone()]), -1, "")); __s.push_str(" English and maths are the two that colleges and employers look at first, whatever else you take."); py_string(__s) };
    }
    return _grade_sentence(&[grade.clone()]);
    Py::None
}
pub fn _mock_answer(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut answer: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut grade: Py = Py::None;
    let mut score: Py = Py::None;
    let _ = py_str("A mock grade is a map of what to fix, not a verdict on the real exam.");
    clean = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    grade = _grade_in(&[clean.clone()]);
    score = _score_out_of(&[clean.clone()]);
    answer = op_add(&if py_truthy(&Py::Bool(py_is(&score.clone(), &Py::None))) { py_str("") } else { op_add(&_score_sentence(&[score.clone()]), &py_str(" ")) }, &py_str("A mock or a predicted grade is a signal, not a verdict: find out which topics cost you the marks — the mark scheme for a mock, your teacher for a predicted grade — and give the worst two a 25-minute slot each week. Moving a grade or two in a term is normal."));
    if py_truthy(&Py::Bool(!py_is(&grade.clone(), &Py::None))) {
    answer = { let mut __s = String::new(); __s.push_str(&fstr(&_grade_sentence(&[grade.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&answer.clone(), -1, "")); py_string(__s) };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\buni\\b|\\buniversit(y|ies)\\b|\\bcollege\\b|sixth form"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_add(&answer.clone(), &py_str(" For the university question, what counts is what the course asks for: check two or three you like and their entry requirements, because a 6 at GCSE rarely closes a door on its own."));
    answer = __aug1;
    }
    return answer.clone();
    Py::None
}
pub fn _a_school_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("False when nothing in the line is about school, so the bare hate is somebody else's.");
    read = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[mcall(&g__SCHOOL_THINGS(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _strict_teacher(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut plan: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The strict-teacher advice, with a first sentence that takes the feeling when there is one.");
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    plan = py_str("Do the boring things first and most strict teachers stop being a problem: hand work in on time, sit where you are seen, and ask one question in the lesson so they know your name for the right reason. If it is personal - sarcasm, singled out, marks that do not match the work - ask for the marked work back in writing and take it to your form tutor or head of year with dates and one example, not a general complaint. Keep the tone flat and factual, because a complaint that is a feeling gets deflected and one that is a date, a thing said and a mark does not. Ask a parent to come to the meeting if you are at school.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__FEELS_WRONGED(), "search", &[mcall(&text.clone(), "lower", &[])])))) {
    return plan.clone();
    }
    if py_truthy(&mcall(&g__IS_CHILD(), "search", &[mcall(&text.clone(), "lower", &[])])) {
    return py_str("Take it seriously, then test it before you act: ask what happens in that lesson and whether other children say the same, because most of these turn out to be about one subject or one seating plan. Then ask the school for a short meeting with the class teacher, and go to the head of year if nothing changes in a term. Ask for the marked work so the conversation is about the marks and not about whether the teacher likes him.");
    }
    return op_add(&py_str("That is a miserable way to spend a lesson, and it is worth testing before you believe it: most teachers who seem to hate one person are strict with the whole class. "), &plan.clone());
    Py::None
}
pub fn _names_a_language(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut leaned: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names a language, or says what it is about without naming one.\n\n    The hours figure is only honest about a language it can name, and the row is reached by the\n    verb \"learn\" as well as by the language words: \"im learning to play the guitar\" then \"how\n    long will it take\" is read as an ask about learning a guitar, and the row answered it with\n    the one figure it has - \"Spanish, French or Italian\", because the line had given it no\n    language to say. A line about a guitar, a cook or a book says none of these words, so the\n    row stands down and the answer about the guitar gets the turn.\n\n    A line that says \"language\", \"fluent\" or \"hold a conversation\" asks about a language\n    without naming one, and the generic figure is its answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    clean = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    leaned = if py_truthy(&call_builtin("isinstance", &[mcall(&understood.clone(), "get", &[py_str("leans_on")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("leans_on")]) } else { py_dict(vec![]) };
    said = mcall(&mcall(&py_str(" "), "join", &[py_list(vec![clean.clone(), mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]), call_builtin("str", &[{ let __b = mcall(&leaned.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), mcall(&py_str(" "), "join", &[{ let __b = mcall(&leaned.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }])])]), "lower", &[]);
    if py_truthy(&_find(&[said.clone(), g_LANGUAGE_HOURS()])) {
    return Py::Bool(true);
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[op_bitand(&words.clone(), &py_set(vec![py_str("language"), py_str("languages"), py_str("fluent"), py_str("fluency"), py_str("fluently"), py_str("bilingual"), py_str("conversation"), py_str("conversations"), py_str("speak"), py_str("speaking"), py_str("spoken")]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_language_the_chat_named(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The language the person has already named in this chat, or \"\".\n\n    A follow-up says the language once and then leans on it: \"im learning spanish\", \"to speak to\n    my wifes family\", \"how long until i can hold a conversation\" is three lines about Spanish and\n    only the first one names it. The figure said back as \"Spanish, French or Italian\" tells\n    somebody who has already chosen a language nothing about their choice; the same figure for\n    Spanish answers what they asked.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    named = if py_truthy(&said.clone()) { _find(&[said.clone(), g_LANGUAGE_HOURS()]) } else { py_str("") };
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _language_hours(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut named: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut years: Py = Py::None;
    let _ = py_str("How many hours a language takes, said in the language the person named.");
    clean = mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    named = { let __b = _find(&[clean.clone(), g_LANGUAGE_HOURS()]); if py_truthy(&__b) { __b } else { _the_language_the_chat_named(&[memory.clone()]) } };
    hours = mcall(&g_LANGUAGE_HOURS(), "get", &[{ let __b = named.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, Py::Int(600)]);
    where_ = if py_truthy(&named.clone()) { mcall(&named.clone(), "capitalize", &[]) } else { py_str("Spanish, French or Italian") };
    years = call_builtin("max", &[Py::Int(2), call_builtin("round", &[op_truediv(&hours.clone(), &Py::Int(365))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&hours.clone(), -1, "")); __s.push_str(" hours of real study to hold a proper conversation in "); __s.push_str(&fstr(&where_.clone(), -1, "")); __s.push_str(" - around "); __s.push_str(&fstr(&years.clone(), -1, "")); __s.push_str(" years at an hour a day, or three times that at twenty minutes a day, which is why a little every day beats a long weekend. A holiday and small talk come far sooner: about 100 hours, which a twice-weekly evening class reaches in a year."); py_string(__s) };
    Py::None
}
pub fn _the_page_is_what_was_forgotten(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True only when the line is about a page: a thing to do, or a life, is not a page.\n\n    Every key in the row carries the bare \"cant remember\", so the row reached lines that name no\n    page at all. \"i cant remember the last time i went out\" was told to close the book and say its\n    three points out loud, and so were \"...i laughed\", \"...i slept well\", \"i cant remember what i\n    went upstairs for\" and \"i cant remember being this happy\" - each one a line about a person's\n    week, answered with revision technique. What is forgotten is named by the line, so the page\n    has to be in it: \"i cant remember what i read\" keeps this answer, and a question asking how to\n    remember keeps it too, because the key is the whole of that question.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__THE_PAGE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&text.clone(), "split", &[]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__HOW_TO_REMEMBER(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__LAST_TIME(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__FORGET_TO_DO(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _classes_named(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut patterns: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("The classes the line names, in the order they are written.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g_CLASS_NAMES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    patterns = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&patterns.clone());
    for __item in __iter3 {
    pattern = __item.clone();
    match_ = stdlib_call("re", "search", &[pattern.clone(), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&found.clone(), &name.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![mcall(&match_.clone(), "start", &[]), name.clone()])]);
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_THE_CLASS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    let __iter4 = py_iter_or_raise(&g_CLASS_ALONE());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    name = __unpk5[0].clone();
    pattern = __unpk5[1].clone();
    match_ = mcall(&pattern.clone(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&found.clone(), &name.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![mcall(&match_.clone(), "start", &[]), name.clone()])]);
    }
    }
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("sorted", &[found.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8719 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk8719[0].clone(); let mut name: Py = __unpk8719[1].clone(); __out.push(name.clone()); } py_list(__out) })() };
    Py::None
}
pub fn _half_of_class_said(args: &[Py]) -> Py {
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
    let mut high: Py = Py::None;
    let mut low: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut side: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The bottom or top half of a band, when the line asks for it by name.");
    let __iter1 = py_iter_or_raise(&g_HALF_OF_CLASS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 5, None);
    word = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    name = __unpk2[2].clone();
    low = __unpk2[3].clone();
    high = __unpk2[4].clone();
    if py_truthy(&stdlib_call("re", "search", &[pattern.clone(), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    side = if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("low")))) { py_str("bottom") } else { py_str("top") };
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is the "); __s.push_str(&fstr(&side.clone(), -1, "")); __s.push_str(" of that band: a weighted average of "); __s.push_str(&fstr(&low.clone(), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&high.clone(), -1, "")); __s.push_str("%, and it counts as a "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" on a certificate."); py_string(__s) };
    }
    }
    return py_str("");
    Py::None
}
pub fn _university_pass_mark(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut asks: Py = Py::None;
    let mut bands: Py = Py::None;
    let mut classes: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut defines: Py = Py::None;
    let mut first: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut half: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let mut understanding: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What the class the line names is, or what a degree passes at when it names none.\n\n    \"How do i get a 2:1\" is not this: it asks for a method, and the method is the study rows'\n    answer, so a how-question keeps the pass-mark paragraph it had rather than being told what\n    the class is worth.\n    ");
    understanding = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&understanding.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    asks = call_builtin("tuple", &[{ let __b = mcall(&understanding.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    classes = _classes_named(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&classes.clone()))) {
    return g_PASS_MARK();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&asks.clone(), &py_str("how"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&asks.clone(), &py_str("steps"))) } }) {
    return g_PASS_MARK();
    }
    defines = { let __b = Py::Bool(py_contains(&asks.clone(), &py_str("definition"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&asks.clone(), &py_str("compare"))); if py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("mean"), py_str("means"), py_str("stands for"), py_str("stand for"), py_str("difference between")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("percentage"), py_str("percent"), py_str("what grade")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&defines.clone()))) {
    return g_PASS_MARK();
    }
    half = _half_of_class_said(&[clean.clone()]);
    if py_truthy(&half.clone()) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&py_str("In UK universities, "), &mcall(&geti(&half.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&half.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str(" ")), &py_str("In full, ")), &geti(&g_CLASS_SAYS(), &geti(&classes.clone(), &Py::Int(0))));
    }
    said = op_add(&py_str("In UK universities, "), &geti(&g_CLASS_SAYS(), &geti(&classes.clone(), &Py::Int(0))));
    if py_truthy(&op_gt(&call_builtin("len", &[classes.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    let __aug1 = op_add(&said.clone(), &op_add(&op_add(&py_str(" "), &mcall(&geti(&geti(&g_CLASS_SAYS(), &geti(&classes.clone(), &Py::Int(1))), &Py::Int(0)), "upper", &[])), &slice_or_raise(&geti(&g_CLASS_SAYS(), &geti(&classes.clone(), &Py::Int(1))), &Py::Int(1), &Py::None, &Py::None)));
    said = __aug1;
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&classes.clone(), &Py::Int(0)), geti(&classes.clone(), &Py::Int(1))]), 2, None);
    first = __unpk2[0].clone();
    second = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CLASS_BAND(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CLASS_BAND(), &second.clone())) } }) {
    gap = call_builtin("abs", &[op_sub(&geti(&g_CLASS_BAND(), &first.clone()), &geti(&g_CLASS_BAND(), &second.clone()))]).unwrap_or_else(|e| exc::raise(e));
    bands = if py_truthy(&Py::Bool(py_eq(&gap.clone(), &Py::Int(10)))) { py_str("one band") } else { { let mut __s = String::new(); __s.push_str(&fstr(&op_floordiv(&gap.clone(), &Py::Int(10)), -1, "")); __s.push_str(" bands"); py_string(__s) } };
    let __aug3 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(" So the line between them is "); __s.push_str(&fstr(&gap.clone(), -1, "")); __s.push_str(" points of average, "); __s.push_str(&fstr(&bands.clone(), -1, "")); __s.push_str(", and the higher one is the class a course or a job asks for by name."); py_string(__s) });
    said = __aug3;
    }
    }
    return said.clone();
    Py::None
}
pub fn _memorise(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut line: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The recall method, for a line that is about remembering and not about another subject.");
    line = op_add(&op_add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { py_str("") } } } }) { let mut w: Py = __item.clone(); __out.push(call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__REMEMBERS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return g__MEMORISE();
    Py::None
}
pub fn _study_later(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut age: Py = Py::None;
    let mut career: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The age question about a course, which may name the course only in the turn before.");
    clean = stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__NOT_AGE_STUDY(), "search", &[clean.clone()])) {
    return Py::None;
    }
    career = call_builtin("bool", &[mcall(&g__CAREER_CHANGE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__AGE_ASK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { career.clone() } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__LATER_STUDY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { career.clone() } }))) {
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    seen = Py::Bool(false);
    if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g__LATER_STUDY(), "search", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]) } }) {
    seen = Py::Bool(true);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&seen.clone()))) {
    return Py::None;
    }
    }
    match_ = mcall(&g__AGE_NUMBER(), "search", &[clean.clone()]);
    age = if py_truthy(&match_.clone()) { call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    if py_truthy(&{ let __cmp21: Py = age.clone(); if py_truthy(&op_le(&Py::Int(18), &__cmp21)) { op_le(&__cmp21, &Py::Int(90)) } else { Py::Bool(false) } }) {
    if py_truthy(&op_gt(&age.clone(), &Py::Int(60))) {
    return { let mut __s = String::new(); __s.push_str("At "); __s.push_str(&fstr(&age.clone(), -1, "")); __s.push_str(" you would be older than most of the class, and that is normal on these courses: what decides it is not the age but the routine - one fixed evening in class and one more for reading, kept for the length of the course - and a subject you will still open on a bad week. Ask the college what share of last year's class finished, and whether any of it can be done in the daytime or online, because that is what makes it stick."); py_string(__s) };
    }
    opener = { let mut __s = String::new(); __s.push_str("At "); __s.push_str(&fstr(&age.clone(), -1, "")); __s.push_str(" you would not be late: "); py_string(__s) };
    } else {
    opener = py_str("You would not be late: ");
    }
    return op_add(&opener.clone(), &py_str("evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age."));
    Py::None
}
pub fn _course_hours(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut before: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&g__LATER_STUDY(), "search", &[clean.clone()])) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    before = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&g__LATER_STUDY(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__CAREER_CHANGE(), "search", &[before.clone()]) } }) {
    return g_COURSE_HOURS();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _job_grades(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut first: Py = Py::None;
    let mut length: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The job the line names sets the first sentence; the advice after it is the same for all.\n\n    A line that asks how long is answered with the years instead, from `JOB_LENGTHS`: \"how long\n    does it take to become a doctor\" was five years of medical school answered with grades.\n    ");
    length = _the_length_answer(&[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, memory.clone()]);
    if py_truthy(&length.clone()) {
    return length.clone();
    }
    text = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }])), &py_str(" "));
    first = py_str("");
    let __iter1 = py_iter_or_raise(&g__GRADES_ROUTE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    sentence = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    first = sentence.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&first.clone()))) {
    first = py_str("The route sets the grades, so there is no single number: the job names the course, and the course names the subjects.");
    }
    return op_add(&first.clone(), &py_str(" Check the entry requirements on two or three course pages before you apply, because the subjects named there are what decide it - a grade below them is the thing to fix first, and a course that asks for less is the fallback."));
    Py::None
}
pub fn _exam_when(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let _ = py_str("The date of the summer exams, and of the mocks, which the school sets itself.");
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("(?<![a-z])mocks?(?![a-z])"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Your school sets its own mock dates, and they are usually in November or December of the year before, so ask your head of year for the week rather than a national one. The summer exams themselves run from the middle of May to the middle of June, and the board's timetable for your subjects is on AQA, Edexcel, OCR or WJEC's site about a year ahead.");
    }
    return g_EXAM_WHEN();
    Py::None
}
pub fn _the_job_row(args: &[Py]) -> Py {
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
    let mut length: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut returning: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The JOB_LENGTHS row a text names, or None.");
    words = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g_JOB_LENGTHS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    names = __unpk2[0].clone();
    length = __unpk2[1].clone();
    returning = __unpk2[2].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), words.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![names.clone(), length.clone(), returning.clone()]);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_length_answer(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut length: Py = Py::None;
    let mut returning: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The years a job takes, when the line asks how long and names one, or None.\n\n    The job is remembered in `memory[\"career\"]` so a follow-up two turns later, which names no\n    job of its own, is still answered as the first line was: \"i was a nurse years ago\", \"do i\n    need to retrain\", \"how long would it take\" is one question about nursing.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    row = _the_job_row(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&row.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&memory.clone(), &py_str("career"), geti(&geti(&row.clone(), &Py::Int(0)), &Py::Int(0)));
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_is(&row.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("career")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    row = _the_job_row(&[geti(&memory.clone(), &py_str("career"))]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&row.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__ASKS_THE_LENGTH(), "search", &[said.clone()]))) } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&row.clone(), 3, None);
    _u = __unpk1[0].clone();
    length = __unpk1[1].clone();
    returning = __unpk1[2].clone();
    if py_truthy(&{ let __b = returning.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__DONE_IT_BEFORE(), "search", &[said.clone()]) } }) {
    return returning.clone();
    }
    return length.clone();
    Py::None
}
pub fn _career_subjects(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut length: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("The subjects a named job asks for, from the one table, or the general advice.\n\n    A line that asks how long gets the years first, because the length and the requirements are\n    two different questions and this row was answering both with the requirements.\n    ");
    clean = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    length = _the_length_answer(&[clean.clone(), memory.clone()]);
    if py_truthy(&length.clone()) {
    return length.clone();
    }
    row = _the_job_row(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&row.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&memory.clone(), &py_str("career"), geti(&geti(&row.clone(), &Py::Int(0)), &Py::Int(0)));
    }
    let __iter1 = py_iter_or_raise(&g_CAREER_SUBJECTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    }
    return g_THE_GENERAL_CAREER();
    Py::None
}
pub fn _the_age_of_a_child(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut age: Py = Py::None;
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The age the line gives, or None. Three is three in \"my son is 3\" and \"my 3 year old\".");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&g__AGE_SAID());
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "search", &[said.clone()]);
    if py_truthy(&found.clone()) {
    let __try2 = exc::catch(|| {
    age = call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&{ let __cmp71: Py = age.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp71)) { op_le(&__cmp71, &Py::Int(18)) } else { Py::Bool(false) } }) {
    return age.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn the_writing_card(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut age: Py = Py::None;
    let _ = py_str("The handwriting answer at the age the line names (see `_AGE_SAID`).");
    age = _the_age_of_a_child(&[reading.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_le(&age.clone(), &Py::Int(4)) } }) {
    return op_add(&op_add(&py_str("At three or four that is not late at all: the shoulder and hand are still building, and a child who cannot write their name yet is exactly on time. "), &g__WRITING_HAND()), &py_str(" Nobody should be sitting a four year old down to drill it, and if a five year old cannot hold a pencil or copy a circle, that is worth a word with the health visitor."));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_ge(&age.clone(), &Py::Int(7)) } }) {
    return py_str("At seven this is past the usual age, so get it looked at rather than practised harder: most children write a recognisable name by five or six. Ask the teacher what it looks like in class, have hearing and vision checked, and if it is just the same at school ask about handwriting practice with an occupational therapist or the SENCO. Keep the practice at home short and keep the worry off it.");
    }
    if py_truthy(&Py::Bool(!py_is(&age.clone(), &Py::None))) {
    return op_add(&op_add(&py_str("Late but still inside normal, and worth a light touch rather than a push: most children write a recognisable name between four and six. "), &g__WRITING_HAND()), &py_str(" Ask the teacher whether the grip looks right, and if it has not moved by the end of the year get hearing and vision checked."));
    }
    return op_add(&op_add(&py_str("Five is early for a name and nothing to worry about: the shoulder and hand are still building, so most children get a recognisable name somewhere between four and six. "), &mcall(&g__WRITING_HAND(), "replace", &[py_str("Give the hand"), py_str("Give the hand")])), &py_str(" Worth asking nursery or the reception teacher whether the grip is right, and worth a word with the health visitor if a child of six cannot hold a pencil or copy a circle at all."));
    Py::None
}
pub fn _asks_to_move(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut bare: Py = Py::None;
    let mut changed: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line itself asks something.\n\n    \"changing subject\" on its own is somebody moving the conversation on, and the answers module\n    reads that line with the turn before it put back in, which is how it reached this row: the\n    line has the two words and the earlier turn has the \"what\". The row keys \"changing subject\"\n    because that is how the question is typed (\"changing subject at-a-level, is it too late\"),\n    so the difference between the two has to be read from the shape of the line, not from its\n    neighbours - and not by fencing the phrase, because that fence also stood the row down for\n    the question that starts with it.\n    ");
    said = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    bare = mcall(&said.clone(), "strip", &[py_str(" .,!?;:-…")]);
    changed = Py::Bool(true);
    while py_truthy(&changed.clone()) {
    changed = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&g__LEAD_IN());
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&bare.clone(), &lead.clone())); if py_truthy(&__b) { __b } else { mcall(&bare.clone(), "startswith", &[op_add(&lead.clone(), &py_str(" "))]) } }) {
    bare = mcall(&slice_or_raise(&bare.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[py_str(" ,")]);
    changed = Py::Bool(true);
    }
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g__THE_ANNOUNCEMENT(), &bare.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ASKED_TO_MOVE()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "startswith", &[op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))]); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[op_add(&py_str(" "), &word.clone())]) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.study".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASLEEP" => Some(g_ASLEEP()),
            "A_MARK_NOT_A_GRADE" => Some(g_A_MARK_NOT_A_GRADE()),
            "A_SUBJECT" => Some(g_A_SUBJECT()),
            "A_WHAT_TO_WEAR" => Some(g_A_WHAT_TO_WEAR()),
            "CAREER_SUBJECTS" => Some(g_CAREER_SUBJECTS()),
            "CATCHING_UP" => Some(g_CATCHING_UP()),
            "CLASS_ALONE" => Some(g_CLASS_ALONE()),
            "CLASS_BAND" => Some(g_CLASS_BAND()),
            "CLASS_NAMES" => Some(g_CLASS_NAMES()),
            "CLASS_SAYS" => Some(g_CLASS_SAYS()),
            "CONCEPTS" => Some(g_CONCEPTS()),
            "CONCEPT_WORDS" => Some(g_CONCEPT_WORDS()),
            "COUNTED_AFTER" => Some(g_COUNTED_AFTER()),
            "COURSE_HOURS" => Some(g_COURSE_HOURS()),
            "ENGLISH_WORDS" => Some(g_ENGLISH_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "ESSAY_WORDS" => Some(g_ESSAY_WORDS()),
            "EXAM_NERVES" => Some(g_EXAM_NERVES()),
            "EXAM_WHEN" => Some(g_EXAM_WHEN()),
            "EXTENSION_ASK" => Some(g_EXTENSION_ASK()),
            "FRACTIONS_ADD" => Some(g_FRACTIONS_ADD()),
            "FRACTIONS_DIVIDE" => Some(g_FRACTIONS_DIVIDE()),
            "FRACTIONS_FIRST" => Some(g_FRACTIONS_FIRST()),
            "FRACTIONS_MULTIPLY" => Some(g_FRACTIONS_MULTIPLY()),
            "FRACTIONS_SIMPLIFY" => Some(g_FRACTIONS_SIMPLIFY()),
            "GRADE_NAME" => Some(g_GRADE_NAME()),
            "HALF_OF_CLASS" => Some(g_HALF_OF_CLASS()),
            "HOW_ASK" => Some(g_HOW_ASK()),
            "JOB_LENGTHS" => Some(g_JOB_LENGTHS()),
            "LANGUAGES" => Some(g_LANGUAGES()),
            "LANGUAGE_HOURS" => Some(g_LANGUAGE_HOURS()),
            "LANGUAGE_LEARNING" => Some(g_LANGUAGE_LEARNING()),
            "LANGUAGE_NAMES" => Some(g_LANGUAGE_NAMES()),
            "LEARNING_OBJECTS" => Some(g_LEARNING_OBJECTS()),
            "LETTER_GRADES" => Some(g_LETTER_GRADES()),
            "LOSING_TRACK_OF_MONEY" => Some(g_LOSING_TRACK_OF_MONEY()),
            "MOCK" => Some(g_MOCK()),
            "NATIVE" => Some(g_NATIVE()),
            "NERVES_BODY" => Some(g_NERVES_BODY()),
            "NOT_AN_EXAM" => Some(g_NOT_AN_EXAM()),
            "NOT_A_PERSONS_NOTE" => Some(g_NOT_A_PERSONS_NOTE()),
            "NOT_A_START_OF_EXAMS" => Some(g_NOT_A_START_OF_EXAMS()),
            "NOT_REVISION" => Some(g_NOT_REVISION()),
            "NOT_THE_CLASS" => Some(g_NOT_THE_CLASS()),
            "PASS_MARK" => Some(g_PASS_MARK()),
            "PHRASES" => Some(g_PHRASES()),
            "REST_ASK" => Some(g_REST_ASK()),
            "SCORE" => Some(g_SCORE()),
            "TEST_AS_A_VERB" => Some(g_TEST_AS_A_VERB()),
            "THE_GENERAL_CAREER" => Some(g_THE_GENERAL_CAREER()),
            "TIME_AFTER_TEST" => Some(g_TIME_AFTER_TEST()),
            "TOP_GRADES" => Some(g_TOP_GRADES()),
            "WELSH_AND_IRISH" => Some(g_WELSH_AND_IRISH()),
            "_ABOUT_THE_PLATE" => Some(g__ABOUT_THE_PLATE()),
            "_AGE_ASK" => Some(g__AGE_ASK()),
            "_AGE_NUMBER" => Some(g__AGE_NUMBER()),
            "_AGE_SAID" => Some(g__AGE_SAID()),
            "_ASKED_OUTRIGHT" => Some(g__ASKED_OUTRIGHT()),
            "_ASKED_TO_MOVE" => Some(g__ASKED_TO_MOVE()),
            "_ASKS_THE_LENGTH" => Some(g__ASKS_THE_LENGTH()),
            "_A_CONCLUSION_SAID" => Some(g__A_CONCLUSION_SAID()),
            "_A_DEGREE_OF_HEAT" => Some(g__A_DEGREE_OF_HEAT()),
            "_A_MEANING_IS_ASKED" => Some(g__A_MEANING_IS_ASKED()),
            "_CAREER_CHANGE" => Some(g__CAREER_CHANGE()),
            "_CHILD" => Some(g__CHILD()),
            "_CORRECTION_FURNITURE" => Some(g__CORRECTION_FURNITURE()),
            "_CORRECTION_WORDS" => Some(g__CORRECTION_WORDS()),
            "_COURSE_TIME" => Some(g__COURSE_TIME()),
            "_DONE_IT_BEFORE" => Some(g__DONE_IT_BEFORE()),
            "_ENGLISH_FURNITURE" => Some(g__ENGLISH_FURNITURE()),
            "_ENGLISH_TOO" => Some(g__ENGLISH_TOO()),
            "_EXAM_IN_THE_WORDS" => Some(g__EXAM_IN_THE_WORDS()),
            "_FAILING" => Some(g__FAILING()),
            "_FEELS_WRONGED" => Some(g__FEELS_WRONGED()),
            "_FORGETTING" => Some(g__FORGETTING()),
            "_FORGET_TO_DO" => Some(g__FORGET_TO_DO()),
            "_GRADES_ROUTE" => Some(g__GRADES_ROUTE()),
            "_HOW_TO_REMEMBER" => Some(g__HOW_TO_REMEMBER()),
            "_IS_CHILD" => Some(g__IS_CHILD()),
            "_LAST_TIME" => Some(g__LAST_TIME()),
            "_LATER_STUDY" => Some(g__LATER_STUDY()),
            "_LEAD_IN" => Some(g__LEAD_IN()),
            "_MEANING_FRAME" => Some(g__MEANING_FRAME()),
            "_MEMORISE" => Some(g__MEMORISE()),
            "_NOT_AGE_STUDY" => Some(g__NOT_AGE_STUDY()),
            "_NUMBER_WORDS" => Some(g__NUMBER_WORDS()),
            "_OTHER_PEOPLE" => Some(g__OTHER_PEOPLE()),
            "_PAST_PAPERS" => Some(g__PAST_PAPERS()),
            "_PAST_PAPERS_ASK_WHEN" => Some(g__PAST_PAPERS_ASK_WHEN()),
            "_PAST_PAPERS_WHEN" => Some(g__PAST_PAPERS_WHEN()),
            "_PHRASE_ASK" => Some(g__PHRASE_ASK()),
            "_REMEMBERS" => Some(g__REMEMBERS()),
            "_SAY_OR_TRANSLATE" => Some(g__SAY_OR_TRANSLATE()),
            "_SCHOOL_THINGS" => Some(g__SCHOOL_THINGS()),
            "_SOMETHING_WRITTEN" => Some(g__SOMETHING_WRITTEN()),
            "_TEMPERATURE_DEGREES" => Some(g__TEMPERATURE_DEGREES()),
            "_THE_ANNOUNCEMENT" => Some(g__THE_ANNOUNCEMENT()),
            "_THE_PAGE" => Some(g__THE_PAGE()),
            "_WRITING_HAND" => Some(g__WRITING_HAND()),
            "_a_failure_of_something_else" => Some(py_func("_a_failure_of_something_else", |args: &[Py]| _a_failure_of_something_else(args))),
            "_a_language_alone" => Some(py_func("_a_language_alone", |args: &[Py]| _a_language_alone(args))),
            "_a_number_word" => Some(py_func("_a_number_word", |args: &[Py]| _a_number_word(args))),
            "_a_phrase_is_asked" => Some(py_func("_a_phrase_is_asked", |args: &[Py]| _a_phrase_is_asked(args))),
            "_a_school_line" => Some(py_func("_a_school_line", |args: &[Py]| _a_school_line(args))),
            "_a_test_of_a_thing" => Some(py_func("_a_test_of_a_thing", |args: &[Py]| _a_test_of_a_thing(args))),
            "_a_translation_is_asked" => Some(py_func("_a_translation_is_asked", |args: &[Py]| _a_translation_is_asked(args))),
            "_a_word_is_being_asked_about" => Some(py_func("_a_word_is_being_asked_about", |args: &[Py]| _a_word_is_being_asked_about(args))),
            "_asks_to_move" => Some(py_func("_asks_to_move", |args: &[Py]| _asks_to_move(args))),
            "_by_language" => Some(g__by_language()),
            "_career_subjects" => Some(py_func("_career_subjects", |args: &[Py]| _career_subjects(args))),
            "_classes_named" => Some(py_func("_classes_named", |args: &[Py]| _classes_named(args))),
            "_concept" => Some(g__concept()),
            "_concept_name" => Some(g__concept_name()),
            "_course_hours" => Some(py_func("_course_hours", |args: &[Py]| _course_hours(args))),
            "_exam_nerves" => Some(py_func("_exam_nerves", |args: &[Py]| _exam_nerves(args))),
            "_exam_when" => Some(py_func("_exam_when", |args: &[Py]| _exam_when(args))),
            "_find" => Some(py_func("_find", |args: &[Py]| _find(args))),
            "_fractions_reply" => Some(py_func("_fractions_reply", |args: &[Py]| _fractions_reply(args))),
            "_grade_answer" => Some(py_func("_grade_answer", |args: &[Py]| _grade_answer(args))),
            "_grade_in" => Some(py_func("_grade_in", |args: &[Py]| _grade_in(args))),
            "_grade_sentence" => Some(py_func("_grade_sentence", |args: &[Py]| _grade_sentence(args))),
            "_half_of_class_said" => Some(py_func("_half_of_class_said", |args: &[Py]| _half_of_class_said(args))),
            "_job_grades" => Some(py_func("_job_grades", |args: &[Py]| _job_grades(args))),
            "_language" => Some(g__language()),
            "_language_hours" => Some(py_func("_language_hours", |args: &[Py]| _language_hours(args))),
            "_memorise" => Some(py_func("_memorise", |args: &[Py]| _memorise(args))),
            "_mock_answer" => Some(py_func("_mock_answer", |args: &[Py]| _mock_answer(args))),
            "_names_a_language" => Some(py_func("_names_a_language", |args: &[Py]| _names_a_language(args))),
            "_past_papers_reply" => Some(py_func("_past_papers_reply", |args: &[Py]| _past_papers_reply(args))),
            "_phrase" => Some(g__phrase()),
            "_plain" => Some(g__plain()),
            "_said_in" => Some(g__said_in()),
            "_say" => Some(g__say()),
            "_say_in_language" => Some(py_func("_say_in_language", |args: &[Py]| _say_in_language(args))),
            "_score_out_of" => Some(py_func("_score_out_of", |args: &[Py]| _score_out_of(args))),
            "_score_sentence" => Some(py_func("_score_sentence", |args: &[Py]| _score_sentence(args))),
            "_somebody_else_forgets" => Some(py_func("_somebody_else_forgets", |args: &[Py]| _somebody_else_forgets(args))),
            "_strict_teacher" => Some(py_func("_strict_teacher", |args: &[Py]| _strict_teacher(args))),
            "_study_later" => Some(py_func("_study_later", |args: &[Py]| _study_later(args))),
            "_the_age_of_a_child" => Some(py_func("_the_age_of_a_child", |args: &[Py]| _the_age_of_a_child(args))),
            "_the_degrees_are_heat" => Some(py_func("_the_degrees_are_heat", |args: &[Py]| _the_degrees_are_heat(args))),
            "_the_ending_is_written" => Some(py_func("_the_ending_is_written", |args: &[Py]| _the_ending_is_written(args))),
            "_the_job_row" => Some(py_func("_the_job_row", |args: &[Py]| _the_job_row(args))),
            "_the_language_the_chat_named" => Some(py_func("_the_language_the_chat_named", |args: &[Py]| _the_language_the_chat_named(args))),
            "_the_length_answer" => Some(py_func("_the_length_answer", |args: &[Py]| _the_length_answer(args))),
            "_the_page_is_what_was_forgotten" => Some(py_func("_the_page_is_what_was_forgotten", |args: &[Py]| _the_page_is_what_was_forgotten(args))),
            "_university_pass_mark" => Some(py_func("_university_pass_mark", |args: &[Py]| _university_pass_mark(args))),
            "_unless_testing_a_thing" => Some(py_func("_unless_testing_a_thing", |args: &[Py]| _unless_testing_a_thing(args))),
            "_what_does_it_mean" => Some(py_func("_what_does_it_mean", |args: &[Py]| _what_does_it_mean(args))),
            "_word" => Some(g__word()),
            "c" => Some(g_c()),
            "entry" => Some(g_entry()),
            "nerves_opening" => Some(g_nerves_opening()),
            "the_writing_card" => Some(py_func("the_writing_card", |args: &[Py]| the_writing_card(args))),
            "quantity" => Some(pyrt::xrefs::xmodule("knowledge.quantity")),
            "re" => Some(stdlib_module("re")),
            "unicodedata" => Some(stdlib_module("unicodedata")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASLEEP" => s_ASLEEP(v),
            "A_MARK_NOT_A_GRADE" => s_A_MARK_NOT_A_GRADE(v),
            "A_SUBJECT" => s_A_SUBJECT(v),
            "A_WHAT_TO_WEAR" => s_A_WHAT_TO_WEAR(v),
            "CAREER_SUBJECTS" => s_CAREER_SUBJECTS(v),
            "CATCHING_UP" => s_CATCHING_UP(v),
            "CLASS_ALONE" => s_CLASS_ALONE(v),
            "CLASS_BAND" => s_CLASS_BAND(v),
            "CLASS_NAMES" => s_CLASS_NAMES(v),
            "CLASS_SAYS" => s_CLASS_SAYS(v),
            "CONCEPTS" => s_CONCEPTS(v),
            "CONCEPT_WORDS" => s_CONCEPT_WORDS(v),
            "COUNTED_AFTER" => s_COUNTED_AFTER(v),
            "COURSE_HOURS" => s_COURSE_HOURS(v),
            "ENGLISH_WORDS" => s_ENGLISH_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            "ESSAY_WORDS" => s_ESSAY_WORDS(v),
            "EXAM_NERVES" => s_EXAM_NERVES(v),
            "EXAM_WHEN" => s_EXAM_WHEN(v),
            "EXTENSION_ASK" => s_EXTENSION_ASK(v),
            "FRACTIONS_ADD" => s_FRACTIONS_ADD(v),
            "FRACTIONS_DIVIDE" => s_FRACTIONS_DIVIDE(v),
            "FRACTIONS_FIRST" => s_FRACTIONS_FIRST(v),
            "FRACTIONS_MULTIPLY" => s_FRACTIONS_MULTIPLY(v),
            "FRACTIONS_SIMPLIFY" => s_FRACTIONS_SIMPLIFY(v),
            "GRADE_NAME" => s_GRADE_NAME(v),
            "HALF_OF_CLASS" => s_HALF_OF_CLASS(v),
            "HOW_ASK" => s_HOW_ASK(v),
            "JOB_LENGTHS" => s_JOB_LENGTHS(v),
            "LANGUAGES" => s_LANGUAGES(v),
            "LANGUAGE_HOURS" => s_LANGUAGE_HOURS(v),
            "LANGUAGE_LEARNING" => s_LANGUAGE_LEARNING(v),
            "LANGUAGE_NAMES" => s_LANGUAGE_NAMES(v),
            "LEARNING_OBJECTS" => s_LEARNING_OBJECTS(v),
            "LETTER_GRADES" => s_LETTER_GRADES(v),
            "LOSING_TRACK_OF_MONEY" => s_LOSING_TRACK_OF_MONEY(v),
            "MOCK" => s_MOCK(v),
            "NATIVE" => s_NATIVE(v),
            "NERVES_BODY" => s_NERVES_BODY(v),
            "NOT_AN_EXAM" => s_NOT_AN_EXAM(v),
            "NOT_A_PERSONS_NOTE" => s_NOT_A_PERSONS_NOTE(v),
            "NOT_A_START_OF_EXAMS" => s_NOT_A_START_OF_EXAMS(v),
            "NOT_REVISION" => s_NOT_REVISION(v),
            "NOT_THE_CLASS" => s_NOT_THE_CLASS(v),
            "PASS_MARK" => s_PASS_MARK(v),
            "PHRASES" => s_PHRASES(v),
            "REST_ASK" => s_REST_ASK(v),
            "SCORE" => s_SCORE(v),
            "TEST_AS_A_VERB" => s_TEST_AS_A_VERB(v),
            "THE_GENERAL_CAREER" => s_THE_GENERAL_CAREER(v),
            "TIME_AFTER_TEST" => s_TIME_AFTER_TEST(v),
            "TOP_GRADES" => s_TOP_GRADES(v),
            "WELSH_AND_IRISH" => s_WELSH_AND_IRISH(v),
            "_ABOUT_THE_PLATE" => s__ABOUT_THE_PLATE(v),
            "_AGE_ASK" => s__AGE_ASK(v),
            "_AGE_NUMBER" => s__AGE_NUMBER(v),
            "_AGE_SAID" => s__AGE_SAID(v),
            "_ASKED_OUTRIGHT" => s__ASKED_OUTRIGHT(v),
            "_ASKED_TO_MOVE" => s__ASKED_TO_MOVE(v),
            "_ASKS_THE_LENGTH" => s__ASKS_THE_LENGTH(v),
            "_A_CONCLUSION_SAID" => s__A_CONCLUSION_SAID(v),
            "_A_DEGREE_OF_HEAT" => s__A_DEGREE_OF_HEAT(v),
            "_A_MEANING_IS_ASKED" => s__A_MEANING_IS_ASKED(v),
            "_CAREER_CHANGE" => s__CAREER_CHANGE(v),
            "_CHILD" => s__CHILD(v),
            "_CORRECTION_FURNITURE" => s__CORRECTION_FURNITURE(v),
            "_CORRECTION_WORDS" => s__CORRECTION_WORDS(v),
            "_COURSE_TIME" => s__COURSE_TIME(v),
            "_DONE_IT_BEFORE" => s__DONE_IT_BEFORE(v),
            "_ENGLISH_FURNITURE" => s__ENGLISH_FURNITURE(v),
            "_ENGLISH_TOO" => s__ENGLISH_TOO(v),
            "_EXAM_IN_THE_WORDS" => s__EXAM_IN_THE_WORDS(v),
            "_FAILING" => s__FAILING(v),
            "_FEELS_WRONGED" => s__FEELS_WRONGED(v),
            "_FORGETTING" => s__FORGETTING(v),
            "_FORGET_TO_DO" => s__FORGET_TO_DO(v),
            "_GRADES_ROUTE" => s__GRADES_ROUTE(v),
            "_HOW_TO_REMEMBER" => s__HOW_TO_REMEMBER(v),
            "_IS_CHILD" => s__IS_CHILD(v),
            "_LAST_TIME" => s__LAST_TIME(v),
            "_LATER_STUDY" => s__LATER_STUDY(v),
            "_LEAD_IN" => s__LEAD_IN(v),
            "_MEANING_FRAME" => s__MEANING_FRAME(v),
            "_MEMORISE" => s__MEMORISE(v),
            "_NOT_AGE_STUDY" => s__NOT_AGE_STUDY(v),
            "_NUMBER_WORDS" => s__NUMBER_WORDS(v),
            "_OTHER_PEOPLE" => s__OTHER_PEOPLE(v),
            "_PAST_PAPERS" => s__PAST_PAPERS(v),
            "_PAST_PAPERS_ASK_WHEN" => s__PAST_PAPERS_ASK_WHEN(v),
            "_PAST_PAPERS_WHEN" => s__PAST_PAPERS_WHEN(v),
            "_PHRASE_ASK" => s__PHRASE_ASK(v),
            "_REMEMBERS" => s__REMEMBERS(v),
            "_SAY_OR_TRANSLATE" => s__SAY_OR_TRANSLATE(v),
            "_SCHOOL_THINGS" => s__SCHOOL_THINGS(v),
            "_SOMETHING_WRITTEN" => s__SOMETHING_WRITTEN(v),
            "_TEMPERATURE_DEGREES" => s__TEMPERATURE_DEGREES(v),
            "_THE_ANNOUNCEMENT" => s__THE_ANNOUNCEMENT(v),
            "_THE_PAGE" => s__THE_PAGE(v),
            "_WRITING_HAND" => s__WRITING_HAND(v),
            "_by_language" => s__by_language(v),
            "_concept" => s__concept(v),
            "_concept_name" => s__concept_name(v),
            "_language" => s__language(v),
            "_phrase" => s__phrase(v),
            "_plain" => s__plain(v),
            "_said_in" => s__said_in(v),
            "_say" => s__say(v),
            "_word" => s__word(v),
            "c" => s_c(v),
            "entry" => s_entry(v),
            "nerves_opening" => s_nerves_opening(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.study", module_obj);
    pyrt::xrefs::reg_fn("knowledge.study", "_past_papers_reply", py_func("_past_papers_reply", |args: &[Py]| _past_papers_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_fractions_reply", py_func("_fractions_reply", |args: &[Py]| _fractions_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_the_ending_is_written", py_func("_the_ending_is_written", |args: &[Py]| _the_ending_is_written(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_a_translation_is_asked", py_func("_a_translation_is_asked", |args: &[Py]| _a_translation_is_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_what_does_it_mean", py_func("_what_does_it_mean", |args: &[Py]| _what_does_it_mean(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_a_word_is_being_asked_about", py_func("_a_word_is_being_asked_about", |args: &[Py]| _a_word_is_being_asked_about(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_find", py_func("_find", |args: &[Py]| _find(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_concept", py_func("_concept", |args: &[Py]| py_call(&g__concept(), args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_a_language_alone", py_func("_a_language_alone", |args: &[Py]| _a_language_alone(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_a_phrase_is_asked", py_func("_a_phrase_is_asked", |args: &[Py]| _a_phrase_is_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_say_in_language", py_func("_say_in_language", |args: &[Py]| _say_in_language(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_a_number_word", py_func("_a_number_word", |args: &[Py]| _a_number_word(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_the_degrees_are_heat", py_func("_the_degrees_are_heat", |args: &[Py]| _the_degrees_are_heat(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_a_failure_of_something_else", py_func("_a_failure_of_something_else", |args: &[Py]| _a_failure_of_something_else(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_a_test_of_a_thing", py_func("_a_test_of_a_thing", |args: &[Py]| _a_test_of_a_thing(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_unless_testing_a_thing", py_func("_unless_testing_a_thing", |args: &[Py]| _unless_testing_a_thing(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_exam_nerves", py_func("_exam_nerves", |args: &[Py]| _exam_nerves(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_somebody_else_forgets", py_func("_somebody_else_forgets", |args: &[Py]| _somebody_else_forgets(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_grade_in", py_func("_grade_in", |args: &[Py]| _grade_in(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_grade_sentence", py_func("_grade_sentence", |args: &[Py]| _grade_sentence(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_score_out_of", py_func("_score_out_of", |args: &[Py]| _score_out_of(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_score_sentence", py_func("_score_sentence", |args: &[Py]| _score_sentence(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_grade_answer", py_func("_grade_answer", |args: &[Py]| _grade_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_mock_answer", py_func("_mock_answer", |args: &[Py]| _mock_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_a_school_line", py_func("_a_school_line", |args: &[Py]| _a_school_line(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_strict_teacher", py_func("_strict_teacher", |args: &[Py]| _strict_teacher(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_names_a_language", py_func("_names_a_language", |args: &[Py]| _names_a_language(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_the_language_the_chat_named", py_func("_the_language_the_chat_named", |args: &[Py]| _the_language_the_chat_named(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_language_hours", py_func("_language_hours", |args: &[Py]| _language_hours(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_the_page_is_what_was_forgotten", py_func("_the_page_is_what_was_forgotten", |args: &[Py]| _the_page_is_what_was_forgotten(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_classes_named", py_func("_classes_named", |args: &[Py]| _classes_named(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_half_of_class_said", py_func("_half_of_class_said", |args: &[Py]| _half_of_class_said(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_university_pass_mark", py_func("_university_pass_mark", |args: &[Py]| _university_pass_mark(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_memorise", py_func("_memorise", |args: &[Py]| _memorise(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_study_later", py_func("_study_later", |args: &[Py]| _study_later(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_course_hours", py_func("_course_hours", |args: &[Py]| _course_hours(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_job_grades", py_func("_job_grades", |args: &[Py]| _job_grades(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_exam_when", py_func("_exam_when", |args: &[Py]| _exam_when(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_the_job_row", py_func("_the_job_row", |args: &[Py]| _the_job_row(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_the_length_answer", py_func("_the_length_answer", |args: &[Py]| _the_length_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_career_subjects", py_func("_career_subjects", |args: &[Py]| _career_subjects(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_the_age_of_a_child", py_func("_the_age_of_a_child", |args: &[Py]| _the_age_of_a_child(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "the_writing_card", py_func("the_writing_card", |args: &[Py]| the_writing_card(args)));
    pyrt::xrefs::reg_fn("knowledge.study", "_asks_to_move", py_func("_asks_to_move", |args: &[Py]| _asks_to_move(args)));
    pyrt::xrefs::reg_access("knowledge.study", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.study", "ASLEEP", g_ASLEEP, s_ASLEEP);
    pyrt::xrefs::reg_access("knowledge.study", "A_MARK_NOT_A_GRADE", g_A_MARK_NOT_A_GRADE, s_A_MARK_NOT_A_GRADE);
    pyrt::xrefs::reg_access("knowledge.study", "A_SUBJECT", g_A_SUBJECT, s_A_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.study", "A_WHAT_TO_WEAR", g_A_WHAT_TO_WEAR, s_A_WHAT_TO_WEAR);
    pyrt::xrefs::reg_access("knowledge.study", "CAREER_SUBJECTS", g_CAREER_SUBJECTS, s_CAREER_SUBJECTS);
    pyrt::xrefs::reg_access("knowledge.study", "CATCHING_UP", g_CATCHING_UP, s_CATCHING_UP);
    pyrt::xrefs::reg_access("knowledge.study", "CLASS_ALONE", g_CLASS_ALONE, s_CLASS_ALONE);
    pyrt::xrefs::reg_access("knowledge.study", "CLASS_BAND", g_CLASS_BAND, s_CLASS_BAND);
    pyrt::xrefs::reg_access("knowledge.study", "CLASS_NAMES", g_CLASS_NAMES, s_CLASS_NAMES);
    pyrt::xrefs::reg_access("knowledge.study", "CLASS_SAYS", g_CLASS_SAYS, s_CLASS_SAYS);
    pyrt::xrefs::reg_access("knowledge.study", "CONCEPTS", g_CONCEPTS, s_CONCEPTS);
    pyrt::xrefs::reg_access("knowledge.study", "CONCEPT_WORDS", g_CONCEPT_WORDS, s_CONCEPT_WORDS);
    pyrt::xrefs::reg_access("knowledge.study", "COUNTED_AFTER", g_COUNTED_AFTER, s_COUNTED_AFTER);
    pyrt::xrefs::reg_access("knowledge.study", "COURSE_HOURS", g_COURSE_HOURS, s_COURSE_HOURS);
    pyrt::xrefs::reg_access("knowledge.study", "ENGLISH_WORDS", g_ENGLISH_WORDS, s_ENGLISH_WORDS);
    pyrt::xrefs::reg_access("knowledge.study", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.study", "ESSAY_WORDS", g_ESSAY_WORDS, s_ESSAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.study", "EXAM_NERVES", g_EXAM_NERVES, s_EXAM_NERVES);
    pyrt::xrefs::reg_access("knowledge.study", "EXAM_WHEN", g_EXAM_WHEN, s_EXAM_WHEN);
    pyrt::xrefs::reg_access("knowledge.study", "EXTENSION_ASK", g_EXTENSION_ASK, s_EXTENSION_ASK);
    pyrt::xrefs::reg_access("knowledge.study", "FRACTIONS_ADD", g_FRACTIONS_ADD, s_FRACTIONS_ADD);
    pyrt::xrefs::reg_access("knowledge.study", "FRACTIONS_DIVIDE", g_FRACTIONS_DIVIDE, s_FRACTIONS_DIVIDE);
    pyrt::xrefs::reg_access("knowledge.study", "FRACTIONS_FIRST", g_FRACTIONS_FIRST, s_FRACTIONS_FIRST);
    pyrt::xrefs::reg_access("knowledge.study", "FRACTIONS_MULTIPLY", g_FRACTIONS_MULTIPLY, s_FRACTIONS_MULTIPLY);
    pyrt::xrefs::reg_access("knowledge.study", "FRACTIONS_SIMPLIFY", g_FRACTIONS_SIMPLIFY, s_FRACTIONS_SIMPLIFY);
    pyrt::xrefs::reg_access("knowledge.study", "GRADE_NAME", g_GRADE_NAME, s_GRADE_NAME);
    pyrt::xrefs::reg_access("knowledge.study", "HALF_OF_CLASS", g_HALF_OF_CLASS, s_HALF_OF_CLASS);
    pyrt::xrefs::reg_access("knowledge.study", "HOW_ASK", g_HOW_ASK, s_HOW_ASK);
    pyrt::xrefs::reg_access("knowledge.study", "JOB_LENGTHS", g_JOB_LENGTHS, s_JOB_LENGTHS);
    pyrt::xrefs::reg_access("knowledge.study", "LANGUAGES", g_LANGUAGES, s_LANGUAGES);
    pyrt::xrefs::reg_access("knowledge.study", "LANGUAGE_HOURS", g_LANGUAGE_HOURS, s_LANGUAGE_HOURS);
    pyrt::xrefs::reg_access("knowledge.study", "LANGUAGE_LEARNING", g_LANGUAGE_LEARNING, s_LANGUAGE_LEARNING);
    pyrt::xrefs::reg_access("knowledge.study", "LANGUAGE_NAMES", g_LANGUAGE_NAMES, s_LANGUAGE_NAMES);
    pyrt::xrefs::reg_access("knowledge.study", "LEARNING_OBJECTS", g_LEARNING_OBJECTS, s_LEARNING_OBJECTS);
    pyrt::xrefs::reg_access("knowledge.study", "LETTER_GRADES", g_LETTER_GRADES, s_LETTER_GRADES);
    pyrt::xrefs::reg_access("knowledge.study", "LOSING_TRACK_OF_MONEY", g_LOSING_TRACK_OF_MONEY, s_LOSING_TRACK_OF_MONEY);
    pyrt::xrefs::reg_access("knowledge.study", "MOCK", g_MOCK, s_MOCK);
    pyrt::xrefs::reg_access("knowledge.study", "NATIVE", g_NATIVE, s_NATIVE);
    pyrt::xrefs::reg_access("knowledge.study", "NERVES_BODY", g_NERVES_BODY, s_NERVES_BODY);
    pyrt::xrefs::reg_access("knowledge.study", "NOT_AN_EXAM", g_NOT_AN_EXAM, s_NOT_AN_EXAM);
    pyrt::xrefs::reg_access("knowledge.study", "NOT_A_PERSONS_NOTE", g_NOT_A_PERSONS_NOTE, s_NOT_A_PERSONS_NOTE);
    pyrt::xrefs::reg_access("knowledge.study", "NOT_A_START_OF_EXAMS", g_NOT_A_START_OF_EXAMS, s_NOT_A_START_OF_EXAMS);
    pyrt::xrefs::reg_access("knowledge.study", "NOT_REVISION", g_NOT_REVISION, s_NOT_REVISION);
    pyrt::xrefs::reg_access("knowledge.study", "NOT_THE_CLASS", g_NOT_THE_CLASS, s_NOT_THE_CLASS);
    pyrt::xrefs::reg_access("knowledge.study", "PASS_MARK", g_PASS_MARK, s_PASS_MARK);
    pyrt::xrefs::reg_access("knowledge.study", "PHRASES", g_PHRASES, s_PHRASES);
    pyrt::xrefs::reg_access("knowledge.study", "REST_ASK", g_REST_ASK, s_REST_ASK);
    pyrt::xrefs::reg_access("knowledge.study", "SCORE", g_SCORE, s_SCORE);
    pyrt::xrefs::reg_access("knowledge.study", "TEST_AS_A_VERB", g_TEST_AS_A_VERB, s_TEST_AS_A_VERB);
    pyrt::xrefs::reg_access("knowledge.study", "THE_GENERAL_CAREER", g_THE_GENERAL_CAREER, s_THE_GENERAL_CAREER);
    pyrt::xrefs::reg_access("knowledge.study", "TIME_AFTER_TEST", g_TIME_AFTER_TEST, s_TIME_AFTER_TEST);
    pyrt::xrefs::reg_access("knowledge.study", "TOP_GRADES", g_TOP_GRADES, s_TOP_GRADES);
    pyrt::xrefs::reg_access("knowledge.study", "WELSH_AND_IRISH", g_WELSH_AND_IRISH, s_WELSH_AND_IRISH);
    pyrt::xrefs::reg_access("knowledge.study", "_ABOUT_THE_PLATE", g__ABOUT_THE_PLATE, s__ABOUT_THE_PLATE);
    pyrt::xrefs::reg_access("knowledge.study", "_AGE_ASK", g__AGE_ASK, s__AGE_ASK);
    pyrt::xrefs::reg_access("knowledge.study", "_AGE_NUMBER", g__AGE_NUMBER, s__AGE_NUMBER);
    pyrt::xrefs::reg_access("knowledge.study", "_AGE_SAID", g__AGE_SAID, s__AGE_SAID);
    pyrt::xrefs::reg_access("knowledge.study", "_ASKED_OUTRIGHT", g__ASKED_OUTRIGHT, s__ASKED_OUTRIGHT);
    pyrt::xrefs::reg_access("knowledge.study", "_ASKED_TO_MOVE", g__ASKED_TO_MOVE, s__ASKED_TO_MOVE);
    pyrt::xrefs::reg_access("knowledge.study", "_ASKS_THE_LENGTH", g__ASKS_THE_LENGTH, s__ASKS_THE_LENGTH);
    pyrt::xrefs::reg_access("knowledge.study", "_A_CONCLUSION_SAID", g__A_CONCLUSION_SAID, s__A_CONCLUSION_SAID);
    pyrt::xrefs::reg_access("knowledge.study", "_A_DEGREE_OF_HEAT", g__A_DEGREE_OF_HEAT, s__A_DEGREE_OF_HEAT);
    pyrt::xrefs::reg_access("knowledge.study", "_A_MEANING_IS_ASKED", g__A_MEANING_IS_ASKED, s__A_MEANING_IS_ASKED);
    pyrt::xrefs::reg_access("knowledge.study", "_CAREER_CHANGE", g__CAREER_CHANGE, s__CAREER_CHANGE);
    pyrt::xrefs::reg_access("knowledge.study", "_CHILD", g__CHILD, s__CHILD);
    pyrt::xrefs::reg_access("knowledge.study", "_CORRECTION_FURNITURE", g__CORRECTION_FURNITURE, s__CORRECTION_FURNITURE);
    pyrt::xrefs::reg_access("knowledge.study", "_CORRECTION_WORDS", g__CORRECTION_WORDS, s__CORRECTION_WORDS);
    pyrt::xrefs::reg_access("knowledge.study", "_COURSE_TIME", g__COURSE_TIME, s__COURSE_TIME);
    pyrt::xrefs::reg_access("knowledge.study", "_DONE_IT_BEFORE", g__DONE_IT_BEFORE, s__DONE_IT_BEFORE);
    pyrt::xrefs::reg_access("knowledge.study", "_ENGLISH_FURNITURE", g__ENGLISH_FURNITURE, s__ENGLISH_FURNITURE);
    pyrt::xrefs::reg_access("knowledge.study", "_ENGLISH_TOO", g__ENGLISH_TOO, s__ENGLISH_TOO);
    pyrt::xrefs::reg_access("knowledge.study", "_EXAM_IN_THE_WORDS", g__EXAM_IN_THE_WORDS, s__EXAM_IN_THE_WORDS);
    pyrt::xrefs::reg_access("knowledge.study", "_FAILING", g__FAILING, s__FAILING);
    pyrt::xrefs::reg_access("knowledge.study", "_FEELS_WRONGED", g__FEELS_WRONGED, s__FEELS_WRONGED);
    pyrt::xrefs::reg_access("knowledge.study", "_FORGETTING", g__FORGETTING, s__FORGETTING);
    pyrt::xrefs::reg_access("knowledge.study", "_FORGET_TO_DO", g__FORGET_TO_DO, s__FORGET_TO_DO);
    pyrt::xrefs::reg_access("knowledge.study", "_GRADES_ROUTE", g__GRADES_ROUTE, s__GRADES_ROUTE);
    pyrt::xrefs::reg_access("knowledge.study", "_HOW_TO_REMEMBER", g__HOW_TO_REMEMBER, s__HOW_TO_REMEMBER);
    pyrt::xrefs::reg_access("knowledge.study", "_IS_CHILD", g__IS_CHILD, s__IS_CHILD);
    pyrt::xrefs::reg_access("knowledge.study", "_LAST_TIME", g__LAST_TIME, s__LAST_TIME);
    pyrt::xrefs::reg_access("knowledge.study", "_LATER_STUDY", g__LATER_STUDY, s__LATER_STUDY);
    pyrt::xrefs::reg_access("knowledge.study", "_LEAD_IN", g__LEAD_IN, s__LEAD_IN);
    pyrt::xrefs::reg_access("knowledge.study", "_MEANING_FRAME", g__MEANING_FRAME, s__MEANING_FRAME);
    pyrt::xrefs::reg_access("knowledge.study", "_MEMORISE", g__MEMORISE, s__MEMORISE);
    pyrt::xrefs::reg_access("knowledge.study", "_NOT_AGE_STUDY", g__NOT_AGE_STUDY, s__NOT_AGE_STUDY);
    pyrt::xrefs::reg_access("knowledge.study", "_NUMBER_WORDS", g__NUMBER_WORDS, s__NUMBER_WORDS);
    pyrt::xrefs::reg_access("knowledge.study", "_OTHER_PEOPLE", g__OTHER_PEOPLE, s__OTHER_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.study", "_PAST_PAPERS", g__PAST_PAPERS, s__PAST_PAPERS);
    pyrt::xrefs::reg_access("knowledge.study", "_PAST_PAPERS_ASK_WHEN", g__PAST_PAPERS_ASK_WHEN, s__PAST_PAPERS_ASK_WHEN);
    pyrt::xrefs::reg_access("knowledge.study", "_PAST_PAPERS_WHEN", g__PAST_PAPERS_WHEN, s__PAST_PAPERS_WHEN);
    pyrt::xrefs::reg_access("knowledge.study", "_PHRASE_ASK", g__PHRASE_ASK, s__PHRASE_ASK);
    pyrt::xrefs::reg_access("knowledge.study", "_REMEMBERS", g__REMEMBERS, s__REMEMBERS);
    pyrt::xrefs::reg_access("knowledge.study", "_SAY_OR_TRANSLATE", g__SAY_OR_TRANSLATE, s__SAY_OR_TRANSLATE);
    pyrt::xrefs::reg_access("knowledge.study", "_SCHOOL_THINGS", g__SCHOOL_THINGS, s__SCHOOL_THINGS);
    pyrt::xrefs::reg_access("knowledge.study", "_SOMETHING_WRITTEN", g__SOMETHING_WRITTEN, s__SOMETHING_WRITTEN);
    pyrt::xrefs::reg_access("knowledge.study", "_TEMPERATURE_DEGREES", g__TEMPERATURE_DEGREES, s__TEMPERATURE_DEGREES);
    pyrt::xrefs::reg_access("knowledge.study", "_THE_ANNOUNCEMENT", g__THE_ANNOUNCEMENT, s__THE_ANNOUNCEMENT);
    pyrt::xrefs::reg_access("knowledge.study", "_THE_PAGE", g__THE_PAGE, s__THE_PAGE);
    pyrt::xrefs::reg_access("knowledge.study", "_WRITING_HAND", g__WRITING_HAND, s__WRITING_HAND);
    pyrt::xrefs::reg_access("knowledge.study", "_by_language", g__by_language, s__by_language);
    pyrt::xrefs::reg_access("knowledge.study", "_concept", g__concept, s__concept);
    pyrt::xrefs::reg_access("knowledge.study", "_concept_name", g__concept_name, s__concept_name);
    pyrt::xrefs::reg_access("knowledge.study", "_language", g__language, s__language);
    pyrt::xrefs::reg_access("knowledge.study", "_phrase", g__phrase, s__phrase);
    pyrt::xrefs::reg_access("knowledge.study", "_plain", g__plain, s__plain);
    pyrt::xrefs::reg_access("knowledge.study", "_said_in", g__said_in, s__said_in);
    pyrt::xrefs::reg_access("knowledge.study", "_say", g__say, s__say);
    pyrt::xrefs::reg_access("knowledge.study", "_word", g__word, s__word);
    pyrt::xrefs::reg_access("knowledge.study", "c", g_c, s_c);
    pyrt::xrefs::reg_access("knowledge.study", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.study", "nerves_opening", g_nerves_opening, s_nerves_opening);
}
