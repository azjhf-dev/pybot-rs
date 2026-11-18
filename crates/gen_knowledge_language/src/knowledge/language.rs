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


thread_local! { static G_ANIMAL_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_FOR_THE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNTED_MINUTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DASH_OF_SOMETHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DASH_LOOKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVERYDAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVERY_MINUTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRAMMAR_TERMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IRREGULAR_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSONAL_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_SINGULARS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_GLOSSES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_SETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANIMAL_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_OPENING_IS_WRITTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_OPENING_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_SPANS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_KIND_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MATTER_TO_RAISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEANING_IS_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAKE_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BUYING_BRACKETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONNECTORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DOUBLED_L: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXT_ILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXT_LATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXT_LATE_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXT_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FIGURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOOD_PHOTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRAMMAR_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRAMMAR_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIND_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIND_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LOOKS_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MARK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MARK_BY_LOOK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MARK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PUNCTUATION_BRACKET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_MATHS_BRACKETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PADDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAST_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAST_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE_PHOTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PET_PHOTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PHOTO_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PHOTO_DETAIL_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PHOTO_DETAIL_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PHRASE_KEYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACE_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACE_PHOTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLAIN_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_ASK_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_GENERAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_GROUPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_LATIN_TWIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_NONE_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_NOT_A_NOUN_AT_ALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_NOT_THE_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_NO_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_OES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_RULE_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_SAME_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_US_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLUSES_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLUS_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POEM_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POEM_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RING_OR_WRITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOMEBODY_FORGETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STRANGER_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SYNONYM_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_TWO_FIGURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNCOUNTABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERSE_INSTANCE_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERSE_METHOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VOWELS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHAT_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WRITING_BRACKETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_thanks_note: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.language: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANIMAL_NOUNS() -> Py { ensure_init(); G_ANIMAL_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_NOUNS(v: Py) { ensure_init(); G_ANIMAL_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_FOR_THE_WORD() -> Py { ensure_init(); G_ASK_FOR_THE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_FOR_THE_WORD(v: Py) { ensure_init(); G_ASK_FOR_THE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNTED_MINUTE() -> Py { ensure_init(); G_A_COUNTED_MINUTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNTED_MINUTE(v: Py) { ensure_init(); G_A_COUNTED_MINUTE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DASH_OF_SOMETHING() -> Py { ensure_init(); G_A_DASH_OF_SOMETHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DASH_OF_SOMETHING(v: Py) { ensure_init(); G_A_DASH_OF_SOMETHING.with(|c| *c.borrow_mut() = v); }
pub fn g_DASH_LOOKS() -> Py { ensure_init(); G_DASH_LOOKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DASH_LOOKS(v: Py) { ensure_init(); G_DASH_LOOKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EVERYDAY_WORDS() -> Py { ensure_init(); G_EVERYDAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVERYDAY_WORDS(v: Py) { ensure_init(); G_EVERYDAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_EVERY_MINUTE() -> Py { ensure_init(); G_EVERY_MINUTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVERY_MINUTE(v: Py) { ensure_init(); G_EVERY_MINUTE.with(|c| *c.borrow_mut() = v); }
pub fn g_GRAMMAR_TERMS() -> Py { ensure_init(); G_GRAMMAR_TERMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRAMMAR_TERMS(v: Py) { ensure_init(); G_GRAMMAR_TERMS.with(|c| *c.borrow_mut() = v); }
pub fn g_IRREGULAR_VERBS() -> Py { ensure_init(); G_IRREGULAR_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IRREGULAR_VERBS(v: Py) { ensure_init(); G_IRREGULAR_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSONAL_NEWS() -> Py { ensure_init(); G_PERSONAL_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSONAL_NEWS(v: Py) { ensure_init(); G_PERSONAL_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURALS() -> Py { ensure_init(); G_PLURALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURALS(v: Py) { ensure_init(); G_PLURALS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_SINGULARS() -> Py { ensure_init(); G_PLURAL_SINGULARS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_SINGULARS(v: Py) { ensure_init(); G_PLURAL_SINGULARS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_GLOSSES() -> Py { ensure_init(); G_WORD_GLOSSES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_GLOSSES(v: Py) { ensure_init(); G_WORD_GLOSSES.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_SETS() -> Py { ensure_init(); G_WORD_SETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_SETS(v: Py) { ensure_init(); G_WORD_SETS.with(|c| *c.borrow_mut() = v); }
pub fn g__ANIMAL_KIND() -> Py { ensure_init(); G__ANIMAL_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANIMAL_KIND(v: Py) { ensure_init(); G__ANIMAL_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_OPENING_IS_WRITTEN() -> Py { ensure_init(); G__AN_OPENING_IS_WRITTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_OPENING_IS_WRITTEN(v: Py) { ensure_init(); G__AN_OPENING_IS_WRITTEN.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_OPENING_QUESTION() -> Py { ensure_init(); G__AN_OPENING_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_OPENING_QUESTION(v: Py) { ensure_init(); G__AN_OPENING_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_SPANS() -> Py { ensure_init(); G__ASK_SPANS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_SPANS(v: Py) { ensure_init(); G__ASK_SPANS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_KIND_WORD() -> Py { ensure_init(); G__A_KIND_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_KIND_WORD(v: Py) { ensure_init(); G__A_KIND_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MATTER_TO_RAISE() -> Py { ensure_init(); G__A_MATTER_TO_RAISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MATTER_TO_RAISE(v: Py) { ensure_init(); G__A_MATTER_TO_RAISE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEANING_IS_ASKED() -> Py { ensure_init(); G__A_MEANING_IS_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEANING_IS_ASKED(v: Py) { ensure_init(); G__A_MEANING_IS_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__BAKE_KIND() -> Py { ensure_init(); G__BAKE_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAKE_KIND(v: Py) { ensure_init(); G__BAKE_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__BUYING_BRACKETS() -> Py { ensure_init(); G__BUYING_BRACKETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BUYING_BRACKETS(v: Py) { ensure_init(); G__BUYING_BRACKETS.with(|c| *c.borrow_mut() = v); }
pub fn g__CONNECTORS() -> Py { ensure_init(); G__CONNECTORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONNECTORS(v: Py) { ensure_init(); G__CONNECTORS.with(|c| *c.borrow_mut() = v); }
pub fn g__DOUBLED_L() -> Py { ensure_init(); G__DOUBLED_L.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DOUBLED_L(v: Py) { ensure_init(); G__DOUBLED_L.with(|c| *c.borrow_mut() = v); }
pub fn g__EXT_ILL() -> Py { ensure_init(); G__EXT_ILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXT_ILL(v: Py) { ensure_init(); G__EXT_ILL.with(|c| *c.borrow_mut() = v); }
pub fn g__EXT_LATE() -> Py { ensure_init(); G__EXT_LATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXT_LATE(v: Py) { ensure_init(); G__EXT_LATE.with(|c| *c.borrow_mut() = v); }
pub fn g__EXT_LATE_WORK() -> Py { ensure_init(); G__EXT_LATE_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXT_LATE_WORK(v: Py) { ensure_init(); G__EXT_LATE_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g__EXT_SUBJECTS() -> Py { ensure_init(); G__EXT_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXT_SUBJECTS(v: Py) { ensure_init(); G__EXT_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g__EXT_WORDS() -> Py { ensure_init(); G__EXT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXT_WORDS(v: Py) { ensure_init(); G__EXT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__FIGURES() -> Py { ensure_init(); G__FIGURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FIGURES(v: Py) { ensure_init(); G__FIGURES.with(|c| *c.borrow_mut() = v); }
pub fn g__FILLER() -> Py { ensure_init(); G__FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FILLER(v: Py) { ensure_init(); G__FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__FOOD_PHOTO() -> Py { ensure_init(); G__FOOD_PHOTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOOD_PHOTO(v: Py) { ensure_init(); G__FOOD_PHOTO.with(|c| *c.borrow_mut() = v); }
pub fn g__GRAMMAR_ASK() -> Py { ensure_init(); G__GRAMMAR_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRAMMAR_ASK(v: Py) { ensure_init(); G__GRAMMAR_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__GRAMMAR_WORDS() -> Py { ensure_init(); G__GRAMMAR_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRAMMAR_WORDS(v: Py) { ensure_init(); G__GRAMMAR_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__KIND_ASKS() -> Py { ensure_init(); G__KIND_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIND_ASKS(v: Py) { ensure_init(); G__KIND_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__KIND_SUBJECTS() -> Py { ensure_init(); G__KIND_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIND_SUBJECTS(v: Py) { ensure_init(); G__KIND_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g__LOOKS_KIND() -> Py { ensure_init(); G__LOOKS_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LOOKS_KIND(v: Py) { ensure_init(); G__LOOKS_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__MARK_ASK() -> Py { ensure_init(); G__MARK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MARK_ASK(v: Py) { ensure_init(); G__MARK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__MARK_BY_LOOK() -> Py { ensure_init(); G__MARK_BY_LOOK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MARK_BY_LOOK(v: Py) { ensure_init(); G__MARK_BY_LOOK.with(|c| *c.borrow_mut() = v); }
pub fn g__MARK_WORDS() -> Py { ensure_init(); G__MARK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MARK_WORDS(v: Py) { ensure_init(); G__MARK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PUNCTUATION_BRACKET() -> Py { ensure_init(); G__NOT_A_PUNCTUATION_BRACKET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PUNCTUATION_BRACKET(v: Py) { ensure_init(); G__NOT_A_PUNCTUATION_BRACKET.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_MATHS_BRACKETS() -> Py { ensure_init(); G__NOT_MATHS_BRACKETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_MATHS_BRACKETS(v: Py) { ensure_init(); G__NOT_MATHS_BRACKETS.with(|c| *c.borrow_mut() = v); }
pub fn g__PADDING() -> Py { ensure_init(); G__PADDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PADDING(v: Py) { ensure_init(); G__PADDING.with(|c| *c.borrow_mut() = v); }
pub fn g__PAST_ASK() -> Py { ensure_init(); G__PAST_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAST_ASK(v: Py) { ensure_init(); G__PAST_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PAST_OF() -> Py { ensure_init(); G__PAST_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAST_OF(v: Py) { ensure_init(); G__PAST_OF.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE_KIND() -> Py { ensure_init(); G__PEOPLE_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE_KIND(v: Py) { ensure_init(); G__PEOPLE_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE_PHOTO() -> Py { ensure_init(); G__PEOPLE_PHOTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE_PHOTO(v: Py) { ensure_init(); G__PEOPLE_PHOTO.with(|c| *c.borrow_mut() = v); }
pub fn g__PET_PHOTO() -> Py { ensure_init(); G__PET_PHOTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PET_PHOTO(v: Py) { ensure_init(); G__PET_PHOTO.with(|c| *c.borrow_mut() = v); }
pub fn g__PHOTO_DETAIL() -> Py { ensure_init(); G__PHOTO_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PHOTO_DETAIL(v: Py) { ensure_init(); G__PHOTO_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__PHOTO_DETAIL_ANY() -> Py { ensure_init(); G__PHOTO_DETAIL_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PHOTO_DETAIL_ANY(v: Py) { ensure_init(); G__PHOTO_DETAIL_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g__PHOTO_DETAIL_PEOPLE() -> Py { ensure_init(); G__PHOTO_DETAIL_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PHOTO_DETAIL_PEOPLE(v: Py) { ensure_init(); G__PHOTO_DETAIL_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__PHRASE_KEYS() -> Py { ensure_init(); G__PHRASE_KEYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PHRASE_KEYS(v: Py) { ensure_init(); G__PHRASE_KEYS.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACE_KIND() -> Py { ensure_init(); G__PLACE_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACE_KIND(v: Py) { ensure_init(); G__PLACE_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACE_PHOTO() -> Py { ensure_init(); G__PLACE_PHOTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACE_PHOTO(v: Py) { ensure_init(); G__PLACE_PHOTO.with(|c| *c.borrow_mut() = v); }
pub fn g__PLAIN_NOUNS() -> Py { ensure_init(); G__PLAIN_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLAIN_NOUNS(v: Py) { ensure_init(); G__PLAIN_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_ASK() -> Py { ensure_init(); G__PLURAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_ASK(v: Py) { ensure_init(); G__PLURAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_ASK_FRAME() -> Py { ensure_init(); G__PLURAL_ASK_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_ASK_FRAME(v: Py) { ensure_init(); G__PLURAL_ASK_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_GENERAL() -> Py { ensure_init(); G__PLURAL_GENERAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_GENERAL(v: Py) { ensure_init(); G__PLURAL_GENERAL.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_GROUPS() -> Py { ensure_init(); G__PLURAL_GROUPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_GROUPS(v: Py) { ensure_init(); G__PLURAL_GROUPS.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_LATIN_TWIN() -> Py { ensure_init(); G__PLURAL_LATIN_TWIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_LATIN_TWIN(v: Py) { ensure_init(); G__PLURAL_LATIN_TWIN.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_NONE_SENTENCE() -> Py { ensure_init(); G__PLURAL_NONE_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_NONE_SENTENCE(v: Py) { ensure_init(); G__PLURAL_NONE_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_NOT_A_NOUN_AT_ALL() -> Py { ensure_init(); G__PLURAL_NOT_A_NOUN_AT_ALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_NOT_A_NOUN_AT_ALL(v: Py) { ensure_init(); G__PLURAL_NOT_A_NOUN_AT_ALL.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_NOT_THE_NOUN() -> Py { ensure_init(); G__PLURAL_NOT_THE_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_NOT_THE_NOUN(v: Py) { ensure_init(); G__PLURAL_NOT_THE_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_NO_COUNT() -> Py { ensure_init(); G__PLURAL_NO_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_NO_COUNT(v: Py) { ensure_init(); G__PLURAL_NO_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_OES() -> Py { ensure_init(); G__PLURAL_OES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_OES(v: Py) { ensure_init(); G__PLURAL_OES.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_RULE() -> Py { ensure_init(); G__PLURAL_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_RULE(v: Py) { ensure_init(); G__PLURAL_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_RULE_SENTENCE() -> Py { ensure_init(); G__PLURAL_RULE_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_RULE_SENTENCE(v: Py) { ensure_init(); G__PLURAL_RULE_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_SAME_WORD() -> Py { ensure_init(); G__PLURAL_SAME_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_SAME_WORD(v: Py) { ensure_init(); G__PLURAL_SAME_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_US_SENTENCE() -> Py { ensure_init(); G__PLURAL_US_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_US_SENTENCE(v: Py) { ensure_init(); G__PLURAL_US_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLUSES_REPLY() -> Py { ensure_init(); G__PLUSES_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLUSES_REPLY(v: Py) { ensure_init(); G__PLUSES_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__PLUS_FORMS() -> Py { ensure_init(); G__PLUS_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLUS_FORMS(v: Py) { ensure_init(); G__PLUS_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g__POEM_ASK() -> Py { ensure_init(); G__POEM_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POEM_ASK(v: Py) { ensure_init(); G__POEM_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__POEM_FORMS() -> Py { ensure_init(); G__POEM_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POEM_FORMS(v: Py) { ensure_init(); G__POEM_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g__RING_OR_WRITE() -> Py { ensure_init(); G__RING_OR_WRITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RING_OR_WRITE(v: Py) { ensure_init(); G__RING_OR_WRITE.with(|c| *c.borrow_mut() = v); }
pub fn g__SOMEBODY_FORGETTING() -> Py { ensure_init(); G__SOMEBODY_FORGETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOMEBODY_FORGETTING(v: Py) { ensure_init(); G__SOMEBODY_FORGETTING.with(|c| *c.borrow_mut() = v); }
pub fn g__STRANGER_KIND() -> Py { ensure_init(); G__STRANGER_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STRANGER_KIND(v: Py) { ensure_init(); G__STRANGER_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g__SYNONYM_ASK() -> Py { ensure_init(); G__SYNONYM_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SYNONYM_ASK(v: Py) { ensure_init(); G__SYNONYM_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_TWO_FIGURES() -> Py { ensure_init(); G__THE_TWO_FIGURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_TWO_FIGURES(v: Py) { ensure_init(); G__THE_TWO_FIGURES.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_VERBS() -> Py { ensure_init(); G__TWO_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_VERBS(v: Py) { ensure_init(); G__TWO_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__UNCOUNTABLE() -> Py { ensure_init(); G__UNCOUNTABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNCOUNTABLE(v: Py) { ensure_init(); G__UNCOUNTABLE.with(|c| *c.borrow_mut() = v); }
pub fn g__VERSE_INSTANCE_LEAD() -> Py { ensure_init(); G__VERSE_INSTANCE_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERSE_INSTANCE_LEAD(v: Py) { ensure_init(); G__VERSE_INSTANCE_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__VERSE_METHOD_WORDS() -> Py { ensure_init(); G__VERSE_METHOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERSE_METHOD_WORDS(v: Py) { ensure_init(); G__VERSE_METHOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__VOWELS() -> Py { ensure_init(); G__VOWELS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VOWELS(v: Py) { ensure_init(); G__VOWELS.with(|c| *c.borrow_mut() = v); }
pub fn g__WHAT_IS() -> Py { ensure_init(); G__WHAT_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHAT_IS(v: Py) { ensure_init(); G__WHAT_IS.with(|c| *c.borrow_mut() = v); }
pub fn g__WRITING_BRACKETS() -> Py { ensure_init(); G__WRITING_BRACKETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WRITING_BRACKETS(v: Py) { ensure_init(); G__WRITING_BRACKETS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_thanks_note() -> Py { ensure_init(); G_thanks_note.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_thanks_note(v: Py) { ensure_init(); G_thanks_note.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Language and writing: spelling, the words people mix up, punctuation, and how to word a\nmessage that is hard to send.\n\nThe answers lane owns this domain. English has a small number of traps that ordinary people\nhit over and over — its and it's, affect and effect, fewer and less, the doubles in\n\"necessary\" — and a rule answers all of them without any guessing, which is exactly the kind\nof knowledge a knowledge base should hold. The second half is the writing people ask for\nhelp with: saying no to an invitation, asking for more time, a condolence card, a complaint\nletter, an apology. Each entry gives the words to use, not just advice about them.\n\nKeys are the words being compared (both of them, so a passing mention is not enough), or the\nthing being written plus what the line wants to do with it (\"condolence\" and \"card\").\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANIMAL_NOUNS(pyrt::xrefs::xget("knowledge.pets", "ANIMAL_NOUNS"));
    s_thanks_note(py_func("thanks_note", |args: &[Py]| pyrt::xrefs::xcall("knowledge.wording", "thanks_note", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("language-and-writing"), py_str("school-and-study")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._FIGURES"); }
    s__FIGURES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("irony"), py_str("ironic"), py_str("ironically")]), py_str("Irony says the opposite of what is meant and lets the situation do the work: a fire station burning down, or a no-smoking sign behind the smokers. Sarcasm is the spoken, pointed version of it.")]), py_tuple(vec![py_tuple(vec![py_str("onomatopoeia"), py_str("onomatopoeic")]), py_str("Onomatopoeia is a word that sounds like the thing it names: splash, buzz, hiss, cuckoo, sizzle. Comics lean on it, and so does most children's poetry.")]), py_tuple(vec![py_tuple(vec![py_str("alliteration")]), py_str("Alliteration repeats the opening sound of nearby words: slippery slope, sweet sixteen, the wild wind. It is about the sound, not the letter - \"know nothing\" is alliteration and \"cute cat\" is not.")]), py_tuple(vec![py_tuple(vec![py_str("personification")]), py_str("Personification gives a thing or an idea human qualities: the wind whispered, the deadline stalked me, the kettle sulked. It turns an abstract thing into something you can picture.")]), py_tuple(vec![py_tuple(vec![py_str("hyperbole")]), py_str("Hyperbole is exaggeration on purpose, not meant literally: I've told you a million times, this bag weighs a ton. It works because both people know it is not true.")]), py_tuple(vec![py_tuple(vec![py_str("euphemism")]), py_str("A euphemism is a soft word for a hard one: passed away for died, let go for sacked, between jobs for unemployed. It spares feelings, and it can hide what happened.")])]));
    fn __data1() -> Py { py_tuple(vec![py_str("ring or write"), py_str("call or write"), py_str("phone or write"), py_str("ring or email"), py_str("call or email"), py_str("phone or email"), py_str("ring or send"), py_str("write or ring"), py_str("write or call"), py_str("write or phone"), py_str("write or email"), py_str("ring or post"), py_str("call or post"), py_str("text or ring"), py_str("ring or text"), py_str("ring or letter"), py_str("letter or a call"), py_str("ring them or write"), py_str("call them or write"), py_str("email them or ring"), py_str("or should i ring"), py_str("or should i write"), py_str("or should i call"), py_str("or should i email"), py_str("ring or call"), py_str("call or ring"), py_str("phone or ring"), py_str("write or send"), py_str("send or ring"), py_str("email or ring")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._RING_OR_WRITE"); }
    s__RING_OR_WRITE(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.DASH_LOOKS"); }
    s_DASH_LOOKS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("em dash"), py_str("em-dash"), py_str("long dash"), py_str("longest")]), py_str("An em dash is the longest dash — like this — and it marks a break or an aside where a comma would be too quiet. On Windows hold Alt and type 0151; on a Mac it is Shift and Option and the hyphen key.")]), py_tuple(vec![py_tuple(vec![py_str("en dash"), py_str("en-dash"), py_str("minus"), py_str("range")]), py_str("An en dash is the middle-length dash, and its job is a range or a pair: 10–20 people, the London–Paris train. On a Mac it is Option and the hyphen key; in Word two hyphens turn into one as you type.")]), py_tuple(vec![py_tuple(vec![py_str("type"), py_str("keyboard"), py_str("how do i get"), py_str("how do i make"), py_str("shortcut")]), py_str("For a dash you can just write two hyphens and let the word processor turn them into one; in Word and Google Docs that happens as you type. On a Mac use Option and the hyphen key, and on Windows Alt and 0151.")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.A_DASH_OF_SOMETHING"); }
    s_A_DASH_OF_SOMETHING(stdlib_call("re", "compile", &[py_str("\\bdash(?:es)? of\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_dict(vec![(py_str("ambiguous"), py_str("Ambiguous means it can be read two ways and you cannot tell which was meant: 'the meeting is at some point tomorrow' is ambiguous about the time.")), (py_str("bittersweet"), py_str("Bittersweet is happy and sad in the same moment: the last day of a job you loved, or a child's first day at school.")), (py_str("cynical"), py_str("Cynical means expecting the worst of people's reasons - assuming a charity drive is really about publicity. It is different from sceptical, which just wants proof.")), (py_str("mundane"), py_str("Mundane means ordinary and a bit dull: the mundane jobs like the washing up and the school run. It is not an insult to a person.")), (py_str("serendipity"), py_str("Serendipity is a good thing found while looking for something else: finding a twenty in an old coat, or meeting your best friend by taking the wrong bus.")), (py_str("procrastinate"), py_str("To procrastinate is to put off what you meant to do today, usually by doing something easier instead. It is about the delay, not about being lazy.")), (py_str("gaslighting"), py_str("Gaslighting is making someone doubt their own memory of events: 'that never happened, you're imagining it'. The point is to leave them unsure of themselves.")), (py_str("condescending"), py_str("Condescending means talking down to someone, as if they were slower than you: explaining the job to the person who has done it for ten years.")), (py_str("patronising"), py_str("Patronising is the same talking-down, often with false kindness: 'well done you!' to a competent adult. Condescending is the colder word for it.")), (py_str("pretentious"), py_str("Pretentious means claiming more importance or taste than the thing deserves: a menu that calls chips 'triple-cooked potato batons'.")), (py_str("no worries"), py_str("'No worries' means it is fine and nothing is owed - an answer to an apology or a thank you, not a warning. In Australia and New Zealand it is also the everyday reply to 'thank you'.")), (py_str("no hard feelings"), py_str("'No hard feelings' means you are not holding a grudge: 'you got the job and I didn't, but no hard feelings'. It closes a disappointment without pretending it did not sting.")), (py_str("no offence"), py_str("'No offence' is a warning that the next thing is blunt, as in 'no offence, but the kitchen is a mess'. It does not make an insult kind, so only say it if you would say the same thing to their face.")), (py_str("all good"), py_str("'All good' means nothing is wrong and there is nothing to apologise for - the relaxed answer to 'sorry I'm late' or 'thanks'. It is casual, so keep it out of a formal email.")), (py_str("fair enough"), py_str("'Fair enough' means you accept the answer even if you would rather not - it closes a small disagreement rather than starting one.")), (py_str("ballpark figure"), py_str("A ballpark figure is a rough number, close enough to plan with: 'about two thousand, give or take'. Ballpark on its own means roughly right.")), (py_str("touch base"), py_str("To touch base is to check in briefly with someone: 'I'll touch base on Friday about the timetable'. It is a short catch-up, not a meeting.")), (py_str("in two minds"), py_str("To be in two minds is to be unable to choose: 'I'm in two minds about the job'. The American version is 'on the fence'.")), (py_str("on the fence"), py_str("To be on the fence is to have not decided yet: 'the council is still on the fence about the bins'. Sitting on the fence means avoiding the choice.")), (py_str("out of the loop"), py_str("To be out of the loop is to stop being told what is happening: 'I was on leave, so I'm out of the loop on the merger'.")), (py_str("red flag"), py_str("A red flag is a warning sign worth taking seriously: a deposit asked for before a viewing, or a new job with no contract.")), (py_str("grey area"), py_str("A grey area is a case the rules do not clearly cover either way: taking a sick day for a sick child. Grey in Britain, gray in America.")), (py_str("catch 22"), py_str("A catch-22 is a rule you cannot win either way: you need experience to get the job, and you need the job to get the experience.")), (py_str("silver lining"), py_str("A silver lining is the small good thing inside a bad one: the flight was cancelled, but you got an extra day with your family.")), (py_str("cold feet"), py_str("To get cold feet is to lose your nerve just before a big step: the wedding is booked and now you are panicking. It means doubt, not that you are ill.")), (py_str("under the bus"), py_str("To throw someone under the bus is to blame them to save yourself: the manager took the credit and then blamed the team when it failed.")), (py_str("hindsight"), py_str("Hindsight is seeing what you should have done once it is too late: 'in hindsight we should have booked earlier'. Hindsight is always 20/20 means it looks obvious afterwards.")), (py_str("cliche"), py_str("A cliche is a phrase so overused it has stopped meaning anything: 'thinking outside the box', 'at the end of the day'. Clichéd describes the phrase, not you.")), (py_str("food for thought"), py_str("Food for thought is something worth thinking about: 'her talk on sleep gave me food for thought'. Nothing to do with eating.")), (py_str("the big picture"), py_str("The big picture is the whole situation rather than the detail: stepping back to see whether the plan still makes sense.")), (py_str("bonjour"), py_str("Bonjour is French for hello - literally 'good day'. You say it from morning until the evening, and bonsoir (good evening) after that.")), (py_str("bonsoir"), py_str("Bonsoir is French for good evening. It takes over from bonjour once it gets dark, and it works both arriving and leaving.")), (py_str("au revoir"), py_str("Au revoir is French for goodbye - literally 'until we see each other again'. Bonjour when you arrive, au revoir when you go.")), (py_str("merci"), py_str("Merci is French for thank you. Merci beaucoup is thank you very much, and the reply is je vous en prie or, informally, pas de problème.")), (py_str("merci beaucoup"), py_str("Merci beaucoup is French for thank you very much - merci is thanks, beaucoup is a lot.")), (py_str("sil vous plait"), py_str("S'il vous plaît is French for please - literally 'if it pleases you'. Say it when you ask for something, and merci when you get it.")), (py_str("bon appetit"), py_str("Bon appétit is French for enjoy your meal - literally 'good appetite'. Said before eating, the way people say 'enjoy' in English.")), (py_str("ciao"), py_str("Ciao is Italian for hello and goodbye, both - you say it arriving and leaving. Arrivederci is the more formal goodbye.")), (py_str("arrivederci"), py_str("Arrivederci is Italian for goodbye - literally 'until we see each other again'. Ciao does the same job between friends.")), (py_str("grazie"), py_str("Grazie is Italian for thank you. Grazie mille (a thousand thanks) is the warmer version, and prego is the reply - you're welcome.")), (py_str("prego"), py_str("Prego is Italian for you're welcome - the answer to grazie. It also means 'please, go ahead' when someone waits for you to pass.")), (py_str("hola"), py_str("Hola is Spanish for hello. It works all day, and buenos días (good morning), buenas tardes (afternoon) and buenas noches (evening) are the politer versions.")), (py_str("adios"), py_str("Adiós is Spanish for goodbye. Hasta luego (see you later) is the softer way out of a conversation.")), (py_str("gracias"), py_str("Gracias is Spanish for thank you. Muchas gracias is thanks a lot, and de nada is the reply - you're welcome.")), (py_str("por favor"), py_str("Por favor is Spanish for please - literally 'by favour'. Gracias is the thank you that goes with it.")), (py_str("danke"), py_str("Danke is German for thank you. Danke schön is thanks a lot, and bitte is the reply - you're welcome, or please, depending on how it is used.")), (py_str("bitte"), py_str("Bitte is German for please and for you're welcome, depending on the moment: bitte when you ask, bitte when you hand something over. Danke is thanks.")), (py_str("auf wiedersehen"), py_str("Auf Wiedersehen is German for goodbye - literally 'until we see each other again'. Tschüss is the casual version between friends.")), (py_str("guten tag"), py_str("Guten Tag is German for good day - hello, in the middle of the day. Guten Morgen is good morning and Guten Abend good evening.")), (py_str("salud"), py_str("Salud is Spanish for health. It is the toast - the same job as 'cheers' - and also what you say after a sneeze.")), (py_str("namaste"), py_str("Namaste is Hindi for hello and goodbye, said with the palms together. It means roughly 'I bow to you'.")), (py_str("sayonara"), py_str("Sayonara is Japanese for goodbye. Konnichiwa is hello, and arigato is thank you.")), (py_str("konnichiwa"), py_str("Konnichiwa is Japanese for hello - strictly good afternoon, though it works as a general hello. Arigato is thank you.")), (py_str("arigato"), py_str("Arigato is Japanese for thank you. Arigato gozaimasu is the politer, fuller form.")), (py_str("ni hao"), py_str("Nǐ hǎo is Mandarin for hello - literally 'you good'. Xièxie (thank you) is the pair you will use most.")), (py_str("buenos dias"), py_str("Buenos días is Spanish for good morning. Hola does for hello all day, and buenas noches is good evening or good night.")), (py_str("buenas noches"), py_str("Buenas noches is Spanish for good evening, and also good night when you go to bed. Hasta mañana is see you tomorrow.")), (py_str("hasta luego"), py_str("Hasta luego is Spanish for see you later - the everyday goodbye. Adiós is the fuller goodbye, and hasta mañana means see you tomorrow.")), (py_str("mucho gusto"), py_str("Mucho gusto is Spanish for pleased to meet you - literally 'much pleasure'. The reply is igualmente, likewise.")), (py_str("de nada"), py_str("De nada is Spanish for you're welcome - literally 'of nothing'. It answers gracias, the way 'no problem' does here.")), (py_str("como estas"), py_str("Cómo estás is Spanish for how are you. The answer people expect is bien, gracias (well, thanks).")), (py_str("que pasa"), py_str("Qué pasa is Spanish for what's happening - a casual 'what's up'. Qué tal is the softer version.")), (py_str("xiexie"), py_str("Xièxie is Mandarin for thank you. Bù kèqì (you're welcome) is the reply, and nǐ hǎo is hello.")), (py_str("oui"), py_str("Oui is French for yes, and non is no. Oui, merci is yes please, which is the pair you will use most.")), (py_str("d accord"), py_str("D'accord is French for okay or agreed - the sound of 'that's fine'. Say it to agree to a plan.")), (py_str("daccord"), py_str("D'accord is French for okay or agreed - the sound of 'that's fine'. Say it to agree to a plan.")), (py_str("ca va"), py_str("Ça va is French for how are you, and also 'it's fine' as an answer. Ça va bien is 'going well'.")), (py_str("pas de probleme"), py_str("Pas de problème is French for no problem - both 'you're welcome' and 'that's fine'.")), (py_str("je vous en prie"), py_str("Je vous en prie is French for you're welcome, and also 'please do' when you hold a door. It answers merci.")), (py_str("tschuss"), py_str("Tschüss is German for bye - the casual form between friends. Auf Wiedersehen is the formal goodbye.")), (py_str("servus"), py_str("Servus is a greeting in southern Germany and Austria: hello, and also bye, depending on the moment.")), (py_str("scheisse"), py_str("Scheiße is German for shit - the everyday swear word, used far more casually than in English, so hearing it is not as strong as it sounds.")), (py_str("molto bene"), py_str("Molto bene is Italian for very good or very well - molto is very, bene is good."))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.WORD_GLOSSES"); }
    s_WORD_GLOSSES(__data2());
    fn __data3() -> Py { py_dict(vec![(py_str("gutted"), py_str("Gutted means deeply disappointed: 'gutted I missed it'. It is the everyday British word for it - stronger than annoyed, milder than devastated.")), (py_str("chuffed"), py_str("Chuffed means pleased: 'chuffed with the result'. Chuffed to bits is very pleased indeed, and not chuffed is put out.")), (py_str("knackered"), py_str("Knackered means exhausted: 'knackered after the move'. It also means broken, as in 'the washing machine is knackered' - the context tells you which.")), (py_str("peckish"), py_str("Peckish means a bit hungry rather than starving: 'I'm peckish' is an invitation to a snack, not to a meal.")), (py_str("dodgy"), py_str("Dodgy means unreliable or not to be trusted: a dodgy knee, dodgy wifi, a dodgy deal. It is a mild word, and it hedges - dodgy suggests you are not certain.")), (py_str("cheeky"), py_str("Cheeky means pleasantly rude or a bit forward: a cheeky joke, a cheeky pint. Brits use it warmly as often as critically, so 'cheeky' is not an insult on its own.")), (py_str("gobsmacked"), py_str("Gobsmacked means so surprised you cannot speak: 'gobsmacked when he turned up'. Gob is slang for mouth, and you are smacked about it.")), (py_str("faff"), py_str("A faff is a job with more fiddly steps than it is worth: 'the paperwork is such a faff'. To faff about is to spend time on small things instead of the real one.")), (py_str("kerfuffle"), py_str("A kerfuffle is a fuss about something small: 'a right kerfuffle over the seating'. It sounds more serious than it is.")), (py_str("naff"), py_str("Naff means unfashionable or a bit cheap: naff souvenirs, a naff joke. It is a gentle dig rather than a real insult.")), (py_str("nicked"), py_str("Nicked means stolen: 'someone nicked my bike'. It is also a narrow win - 'they nicked it 1-0' - and, in police talk, arrested.")), (py_str("buzzing"), py_str("Buzzing means excited and delighted: 'buzzing for the trip'. It is the happy twin of dreading.")), (py_str("niggling"), py_str("Niggling means small but persistent and hard to ignore: a niggling worry, a niggling pain. The size is not the point - the persistence is.")), (py_str("miffed"), py_str("Miffed means mildly annoyed or put out: 'a bit miffed she wasn't asked'. Milder than angry, and usually about something small.")), (py_str("cheesed off"), py_str("Cheesed off means fed up: 'cheesed off with the weather'. Brassed off and browned off mean the same thing.")), (py_str("cringe"), py_str("Cringe is the wince you feel at someone else's embarrassment: 'watching him sing was pure cringe'. As a verb, the memory makes you cringe.")), (py_str("salty"), py_str("Salty means bitter and sulky about losing or being left out: 'still salty about the game'. The food sense is the plain one - too much salt - so the joke is that the person is tasting tears.")), (py_str("sus"), py_str("Sus means suspicious or not quite right, of a thing or a person: 'that email looks sus', 'he was acting sus all night'. It is short for suspicious.")), (py_str("no cap"), py_str("'No cap' means no lie: 'it's the best one, no cap'. Cap is the lie and capping is exaggerating, so no cap is the promise that you mean it.")), (py_str("vibes"), py_str("Vibes are the feeling a place or person gives off: 'good vibes', 'a bad vibe'. Not something you can measure, which is the point of the word.")), (py_str("ghosting"), py_str("Ghosting is disappearing on someone without a word: no reply, no explanation, just silence. It is common enough in dating to have its own word.")), (py_str("catfish"), py_str("To catfish is to invent an online identity to draw someone into a relationship: the person you have been talking to is not who the photos showed.")), (py_str("doomscrolling"), py_str("Doomscrolling is scrolling bad news long after you have stopped taking it in. The feed carries on; the mood does not recover.")), (py_str("fomo"), py_str("FOMO is fear of missing out - the tug you feel seeing everyone else's weekend. It is said as a word, 'fomo', not spelled out.")), (py_str("clickbait"), py_str("Clickbait is a headline written to get the click rather than to tell you anything: 'you won't believe what happened next'.")), (py_str("troll"), py_str("A troll is someone who posts to wind people up rather than to argue in good faith. Answering them is the contact they wanted.")), (py_str("meme"), py_str("A meme is a joke or an idea copied and varied across the internet: one picture with a caption that people keep reworking.")), (py_str("empathy"), py_str("Empathy is feeling with someone - you can imagine how it is from inside their side. Sympathy is feeling sorry for them, which is kind but stands further away.")), (py_str("sympathy"), py_str("Sympathy is feeling sorry for someone's trouble and saying so: 'you have my sympathy'. Empathy is the one where you feel it with them.")), (py_str("apathy"), py_str("Apathy is not caring when caring would be reasonable - the flat 'what's the point'. It is not the same as being calm or easy-going.")), (py_str("ambivalent"), py_str("Ambivalent means holding both feelings at once: wanting the job and dreading it. It does not mean you do not care.")), (py_str("indifferent"), py_str("Indifferent means it genuinely makes no difference to you either way. Being indifferent is flatter than disliking something.")), (py_str("boundaries"), py_str("A boundary is a limit you set on what you will do or accept: 'I don't answer work messages after eight'. It is a rule for yourself, not a demand on someone else - which is why saying it out loud is the hard part.")), (py_str("closure"), py_str("Closure is the sense of an ending that lets you stop going over something: an explanation, an apology, a last conversation. Plenty of people never get it, and you can still move on without it.")), (py_str("narcissist"), py_str("A narcissist is someone with an inflated sense of their own importance and little room for other people's feelings. The word is used loosely in ordinary talk; as a diagnosis it is a specific personality disorder, and only a professional can make it.")), (py_str("validation"), py_str("Validation is being told that what you feel or did makes sense: 'that's reasonable'. People want it because being disbelieved is a sting of its own.")), (py_str("redundancy"), py_str("Redundancy is losing your job because the role is going, not because of anything you did: the work is cut, moved or merged. It usually means notice plus a statutory payment based on your age and length of service.")), (py_str("furlough"), py_str("Furlough is a temporary lay-off where your employer keeps your job but stops paying you - or the state pays part of your wage through the employer.")), (py_str("appraisal"), py_str("An appraisal is the regular sit-down where your manager goes over your work: what went well, what to change, and sometimes pay. Bring your own list of what you did.")), (py_str("paradox"), py_str("A paradox is a statement that seems to contradict itself but may still be true: 'the more you learn, the less you know'.")), (py_str("hypothetical"), py_str("Hypothetical means imagined rather than real: 'hypothetically, if I left'. People use it to ask a question without owning it.")), (py_str("arbitrary"), py_str("Arbitrary means decided without a rule or reason: an arbitrary deadline, an arbitrary number. It is a criticism of how the choice was made.")), (py_str("mitigate"), py_str("To mitigate is to make something less bad: extra drainage mitigates flooding. Do not confuse it with 'militate', which means to weigh against.")), (py_str("inertia"), py_str("Inertia is the tendency to carry on as you were: in physics, an object's resistance to a change of movement; at home, the reason a routine outlasts its point.")), (py_str("reiterate"), py_str("To reiterate is to say something again, usually to be sure it was heard: 'let me reiterate the deadline'.")), (py_str("satire"), py_str("Satire uses humour to mock something in order to change it: a sketch that makes a habit ridiculous. It is aimed at the powerful, which is what separates it from a joke at someone's expense.")), (py_str("sarcasm"), py_str("Sarcasm is saying the opposite of what you mean with a bite to it: 'great, another meeting'. Irony is the wider pattern; sarcasm is aimed at a person.")), (py_str("stereotype"), py_str("A stereotype is a fixed, over-simple picture of a group, applied to individuals who may not fit it at all.")), (py_str("prejudice"), py_str("Prejudice is judging before you know: a set view about a group of people, held regardless of what they are actually like.")), (py_str("discrimination"), py_str("Discrimination is treating someone worse because of who they are - their sex, race, age, disability, religion and so on - rather than what they do.")), (py_str("consent"), py_str("Consent is a clear yes, given freely, and it can be taken back at any point. Someone asleep, too drunk to follow what is happening, or being pressured cannot give it.")), (py_str("placebo"), py_str("A placebo is a treatment with no active ingredient in it. People often improve anyway, which is the placebo effect, and it is why trials compare against one.")), (py_str("kudos"), py_str("Kudos is praise and credit for something done: 'kudos for getting through that'. It is uncountable - you get kudos, not a kudo.")), (py_str("schadenfreude"), py_str("Schadenfreude is pleasure at someone else's bad luck - German for 'harm-joy', and borrowed because it names the feeling exactly.")), (py_str("zeitgeist"), py_str("Zeitgeist is the mood of a period: the ideas and tastes that feel typical of the time. German for 'time-spirit'."))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.EVERYDAY_WORDS"); }
    s_EVERYDAY_WORDS(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._A_MEANING_IS_ASKED"); }
    s__A_MEANING_IS_ASKED(stdlib_call("re", "compile", &[py_str("\\bwhat\\s+(?:does|do|did)\\b[^?.!]{0,40}?\\bmean(?:s|ing)?\\b|\\bmean(?:s|ing|ings)?\\s+of\\b|\\bdefin(?:e|ed|ing|ition|itions)\\b|\\bstands?\\s+for\\b|\\brefers?\\s+to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._WHAT_IS"); }
    s__WHAT_IS(py_tuple(vec![py_str("what"), py_str("whats"), py_str("what's"), py_str("who"), py_str("whos"), py_str("who's"), py_str("explain"), py_str("define"), py_str("meaning")]));
    fn __data4() -> Py { py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("s"), py_str("a"), py_str("an"), py_str("the"), py_str("of"), py_str("mean"), py_str("means"), py_str("meant"), py_str("exactly"), py_str("actually"), py_str("please"), py_str("word"), py_str("term")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._FILLER"); }
    s__FILLER(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.ASK_FOR_THE_WORD"); }
    s_ASK_FOR_THE_WORD(py_str("I can take a single word or saying apart if you give me the one you are stuck on - the meaning, then an example of it in a sentence."));
    fn __data5() -> Py { py_tuple(vec![py_str("deadline"), py_str("deadlines"), py_str("extension"), py_str("extensions"), py_str("extend"), py_str("extended"), py_str("more time"), py_str("extra time"), py_str("late submission"), py_str("late work"), py_str("late"), py_str("extenuating circumstances"), py_str("mitigating circumstances"), py_str("mitigation"), py_str("exemptions"), py_str("exemption")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._EXT_WORDS"); }
    s__EXT_WORDS(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("essay"), py_str("assignment"), py_str("coursework"), py_str("dissertation"), py_str("thesis"), py_str("project"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("tutor"), py_str("lecturer"), py_str("professor"), py_str("teacher"), py_str("uni"), py_str("university"), py_str("college"), py_str("school"), py_str("course"), py_str("module"), py_str("marks"), py_str("mark"), py_str("grade"), py_str("grades"), py_str("submit"), py_str("submission"), py_str("hand in"), py_str("hand it in"), py_str("handing")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._EXT_SUBJECTS"); }
    s__EXT_SUBJECTS(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("ill"), py_str("illness"), py_str("sick"), py_str("hospital"), py_str("bereavement"), py_str("died"), py_str("death"), py_str("funeral"), py_str("depressed"), py_str("anxiety"), py_str("mental health"), py_str("family emergency"), py_str("unwell"), py_str("flu"), py_str("covid"), py_str("migraine")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._EXT_ILL"); }
    s__EXT_ILL(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._EXT_LATE"); }
    s__EXT_LATE(stdlib_call("re", "compile", &[py_str("\\blate\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data8() -> Py { py_tuple(vec![py_str("essay"), py_str("assignment"), py_str("coursework"), py_str("dissertation"), py_str("thesis"), py_str("project"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("course"), py_str("module"), py_str("submission"), py_str("submitted"), py_str("submit"), py_str("hand in"), py_str("handed in"), py_str("handing"), py_str("marks"), py_str("mark"), py_str("grade"), py_str("grades"), py_str("deadline"), py_str("deadlines"), py_str("extension"), py_str("extensions"), py_str("due in"), py_str("was due"), py_str("overdue"), py_str("late work"), py_str("late submission")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._EXT_LATE_WORK"); }
    s__EXT_LATE_WORK(__data8());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._NOT_MATHS_BRACKETS"); }
    s__NOT_MATHS_BRACKETS(stdlib_call("re", "compile", &[py_str("\\b(expand|expanding|simplify|simplifying|multiply|multiplying|factorise|factorize|factorising|solve|solving|remove|removing)\\b|\\d|\\bx\\b|\\bxy\\b|\\bequals\\b|\\btax\\b|\\bincome\\b|\\bsalary\\b|\\bband\\b|\\bprice\\b|\\bpay\\b|\\bage\\b|\\bearn\\b|\\bcup\\b|\\bround\\b|\\btournament\\b|\\bleague\\b|\\bcompetition\\b|\\balgebra\\b|\\balgebraic\\b|\\bmaths\\b|\\bmath\\b|\\bequations?\\b|\\bsimplif\\w*\\b|\\bpowers?\\b|\\bterms?\\b|\\bcoefficients?\\b|\\bnegatives?\\b|\\bdouble\\b|\\btriple\\b|\\bunderstand\\b|\\bmultiply out\\b|\\bsums?\\b|\\bcalculate\\b|\\bexpressions?\\b|\\bworking out\\b|\\bwork out\\b|\\bteam\\b|\\bteams\\b|\\bplayer\\b|\\bplayers\\b|\\bknockout\\b|\\bqualify\\b|\\bfixture\\b|\\bmatch\\b|\\bmatches\\b|\\bdraw\\b|\\bgroup\\b|\\bengland\\b|\\bscotland\\b|\\bwales\\b|\\bireland\\b|\\bfinal\\b|\\bshel(?:f|ves)\\b|\\bwall\\b|\\btv\\b|\\btelevision\\b|\\bscrews?\\b|\\bdrill\\b|\\bfixings?\\b|\\bcurtains?\\b|\\bblinds?\\b|\\bmonitor\\b|\\bspeakers?\\b|\\baerial\\b|\\bcupboard\\b|\\bwardrobe\\b|\\bmount\\b|\\bmounted\\b|\\bhinge\\b|\\bgate\\b|\\bradiators?\\b|\\bboiler\\b|\\btoilet\\b|\\bsink\\b|\\bcarrier\\b|\\btowel rail\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._NOT_A_PUNCTUATION_BRACKET"); }
    s__NOT_A_PUNCTUATION_BRACKET(stdlib_call("re", "compile", &[py_str("\\b(wall|walls|shelf|shelves|shelving|tv|telly|television|tvs|monitor|monitors|speaker|speakers|soundbar|radiator|radiators|curtain|curtains|rail|rails|blind|blinds|pipe|pipes|plasterboard|stud|studs|drill|screws?|screwdriver|fixings?|rawl|mount|mounts|mounted|mounting|cistern|sink|boiler|cabinet|cabinets|wardrobe|mirror|aerial|satellite|dish|hinges?|fence|post|posts|beam|joists?|trellis|gutter|guttering|bathroom|kitchen)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._BUYING_BRACKETS"); }
    s__BUYING_BRACKETS(stdlib_call("re", "compile", &[py_str("\\bneed(s|ed)?\\b|\\bshould i (?:get|buy|use)\\b|\\bbuy\\b|\\bwhat size\\b|\\bwhich one\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._WRITING_BRACKETS"); }
    s__WRITING_BRACKETS(stdlib_call("re", "compile", &[py_str("\\bsentences?\\b|\\bwriting\\b|\\bwrote\\b|\\bgrammar\\b|\\bpunctuat\\w*\\b|\\bessays?\\b|\\bparagraphs?\\b|\\bcomma\\b|\\bsemicolon\\b|\\bquote\\b|\\bquotes\\b|\\bspell\\w*\\b|\\btext\\b|\\bword\\b|\\bwords\\b|\\bmarks?\\b|\\bmean\\b|\\bmeans\\b|\\benglish\\b|\\bwriter\\b|\\bdraft\\b|\\bcopy\\b|\\bedit\\w*\\b|\\bproofread\\w*\\b|\\bstyle\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLUSES_REPLY"); }
    s__PLUSES_REPLY(py_str("Octopuses is right in English, and octopi is the older Latin-style plural plenty of people still use. The same -uses ending covers viruses, cactuses and walruses; crises and analyses keep the Latin -es."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLUS_FORMS"); }
    s__PLUS_FORMS(py_tuple(vec![py_str("octopus"), py_str("octopuses"), py_str("octopi"), py_str("cactus"), py_str("cactuses"), py_str("cacti"), py_str("walrus"), py_str("walruses"), py_str("virus"), py_str("viruses")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._THE_TWO_FIGURES"); }
    s__THE_TWO_FIGURES(py_dict(vec![(py_str("metaphor"), py_str("A metaphor says a thing is another thing (her laugh was a kettle, the office is a zoo), so the picture does the work without a 'like' in front of it. A simile does put one in: her laugh was like a kettle. Both paint a picture; the metaphor just claims it outright.")), (py_str("simile"), py_str("A simile compares one thing to another with like or as (her laugh was like a kettle, as busy as a bee). A metaphor skips those two words and says the thing is the other one (her laugh was a kettle). Both paint a picture; the simile keeps the comparison out in the open."))]));
    fn __data10() -> Py { py_tuple(vec![py_str("tired"), py_str("late"), py_str("much"), py_str("many"), py_str("young"), py_str("old"), py_str("big"), py_str("small"), py_str("hot"), py_str("cold"), py_str("fast"), py_str("slow"), py_str("early"), py_str("expensive"), py_str("busy"), py_str("hard"), py_str("long"), py_str("far"), py_str("loud"), py_str("noisy"), py_str("heavy"), py_str("sweet"), py_str("salty"), py_str("sharp"), py_str("tough"), py_str("soon"), py_str("often"), py_str("close"), py_str("windy"), py_str("wind"), py_str("rainy"), py_str("rain"), py_str("raining"), py_str("sunny"), py_str("snowy"), py_str("snow"), py_str("wet"), py_str("dry"), py_str("cloudy"), py_str("foggy"), py_str("misty"), py_str("stormy"), py_str("chilly"), py_str("warm"), py_str("cool"), py_str("freezing"), py_str("boiling"), py_str("dark"), py_str("bright"), py_str("mild"), py_str("humid"), py_str("muggy"), py_str("frosty"), py_str("parasol"), py_str("umbrella"), py_str("washing"), py_str("laundry"), py_str("washing line"), py_str("barbecue"), py_str("bbq"), py_str("picnic"), py_str("bike"), py_str("bicycle"), py_str("cycle"), py_str("ride"), py_str("walk"), py_str("hike"), py_str("garden"), py_str("outside"), py_str("outdoors"), py_str("dog"), py_str("run"), py_str("running"), py_str("plant"), py_str("plants"), py_str("paint")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("difference"), py_str("correct"), py_str("spell"), py_str("spelling"), py_str("grammar"), py_str("should it be"), py_str("do i use"), py_str("to or"), py_str("or too"), py_str("too or"), py_str("when do i use"), py_str("which is right"), py_str("is it spelled"), py_str("is it spelt"), py_str("spelt"), py_str("spelled")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("between you and i"), py_str("between you and me"), py_str("me or i"), py_str("i or me"), py_str("than i"), py_str("than me"), py_str("him and me"), py_str("he and i"), py_str("him or me"), py_str("he or i"), py_str("my husband and me"), py_str("my friend and me"), py_str("my friend and i"), py_str("me and my")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("apostrophe after s"), py_str("apostrophe after a name"), py_str("apostrophe after names"), py_str("apostrophe with names ending in s"), py_str("apostrophe for names ending in s"), py_str("names ending in s"), py_str("name ending in s"), py_str("names that end in s"), py_str("name ends in s"), py_str("possessive apostrophe"), py_str("apostrophe for possession"), py_str("james's"), py_str("james")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("apostrophe"), py_str("apostrophes"), py_str("name"), py_str("names"), py_str("surname"), py_str("possessive"), py_str("belonging"), py_str("after"), py_str("ends"), py_str("ending"), py_str("s"), py_str("james"), py_str("or")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("how much"), py_str("price"), py_str("quote me"), py_str("insurance"), py_str("estimate"), py_str("trade"), py_str("garage"), py_str("in an essay"), py_str("in my essay"), py_str("in a essay"), py_str("quote someone"), py_str("quote a source"), py_str("quotation in an essay"), py_str("essay"), py_str("essays"), py_str("coursework"), py_str("dissertation"), py_str("citation"), py_str("citations"), py_str("cite"), py_str("harvard"), py_str("referencing"), py_str("job"), py_str("jobs"), py_str("for a job"), py_str("quote for"), py_str("quotes for"), py_str("customer"), py_str("customers"), py_str("client"), py_str("clients"), py_str("pricing"), py_str("supplier"), py_str("suppliers"), py_str("tender"), py_str("invoice")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("shopping"), py_str("to do"), py_str("todo"), py_str("checklist"), py_str("packing"), py_str("steps"), py_str("chores"), py_str("cv"), py_str("resume"), py_str("cover letter"), py_str("skills"), py_str("references"), py_str("interview")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("difference"), py_str("use"), py_str("when"), py_str("for"), py_str("do"), py_str("does"), py_str("mean"), py_str("used"), py_str("explain"), py_str("what"), py_str("type"), py_str("long"), py_str("longer"), py_str("how")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("keeps cancelling"), py_str("keeps canceling"), py_str("keeps bailing"), py_str("always cancelling"), py_str("cancels on me"), py_str("cancelled on me"), py_str("canceled on me"), py_str("cancelled again"), py_str("bailed on me"), py_str("let me down"), py_str("keeps flaking"), py_str("no show"), py_str("no-show"), py_str("my friend keeps"), py_str("my mate keeps"), py_str("he keeps"), py_str("she keeps"), py_str("they keep"), py_str("someone keeps"), py_str("someone cancelled"), py_str("someone cancelled on")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("rsvp"), py_str("reply to the invitation"), py_str("reply to the invite"), py_str("reply to a wedding invitation"), py_str("reply to a wedding invite"), py_str("reply to an invitation"), py_str("respond to the invitation"), py_str("respond to an invite"), py_str("respond to an invitation"), py_str("answer the invitation"), py_str("answer an invitation"), py_str("reply to a wedding card"), py_str("rsvp to a wedding"), py_str("rsvp to the wedding"), py_str("accept the invitation"), py_str("accept an invite"), py_str("how do i reply to an invite"), py_str("wedding invite"), py_str("wedding invitation")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("how"), py_str("what"), py_str("do i"), py_str("should"), py_str("write"), py_str("say"), py_str("send"), py_str("back"), py_str("card"), py_str("rsvp"), py_str("reply"), py_str("respond"), py_str("answer"), py_str("accept"), py_str("wedding"), py_str("invitation"), py_str("invite"), py_str("date"), py_str("deadline")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("cancel"), py_str("cancelling"), py_str("canceling"), py_str("changed my mind"), py_str("change my mind"), py_str("decline"), py_str("declining"), py_str("turn down"), py_str("say no"), py_str("said yes"), py_str("already said"), py_str("cant go"), py_str("can't go"), py_str("not going"), py_str("kids"), py_str("children"), py_str("child"), py_str("word"), py_str("wording"), py_str("reword"), py_str("write my"), py_str("write our"), py_str("write the"), py_str("writing my"), py_str("writing the"), py_str("send out"), py_str("send them"), py_str("send it out"), py_str("sending out"), py_str("print"), py_str("printed"), py_str("printing"), py_str("printer"), py_str("design"), py_str("designer"), py_str("draft"), py_str("template"), py_str("stationery"), py_str("invitation wording"), py_str("invites wording"), py_str("guest list"), py_str("how many guests"), py_str("how many people"), py_str("save the date"), py_str("the invitation says"), py_str("send wedding"), py_str("send the invitation"), py_str("send the invitations"), py_str("send invitations"), py_str("sending the invitation"), py_str("sending the invitations"), py_str("mail them"), py_str("mail out"), py_str("ask for rsvps"), py_str("ask for rsvp"), py_str("asking for rsvps"), py_str("collect rsvps"), py_str("chase rsvps"), py_str("chasing rsvps"), py_str("yes or no"), py_str("whether"), py_str("not sure if i should"), py_str("should i go"), py_str("what is rsvp"), py_str("whats rsvp"), py_str("what's rsvp"), py_str("what does rsvp"), py_str("rsvp mean"), py_str("rsvp means"), py_str("rsvp stand for"), py_str("rsvp stands for")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("reply"), py_str("rsvp"), py_str("respond"), py_str("answer"), py_str("say"), py_str("write back"), py_str("email back"), py_str("text back"), py_str("confirm"), py_str("accept"), py_str("decline"), py_str("let them know"), py_str("get back")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("word"), py_str("wording"), py_str("reword"), py_str("write"), py_str("writing"), py_str("draft"), py_str("drafting"), py_str("design"), py_str("what to put"), py_str("what should i put"), py_str("how do i word"), py_str("how do i write"), py_str("how to word"), py_str("how to write")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("rsvp"), py_str("reply"), py_str("replying"), py_str("respond"), py_str("responding"), py_str("answer the"), py_str("accept"), py_str("decline"), py_str("declining"), py_str("turn down"), py_str("say no"), py_str("speech"), py_str("toast"), py_str("eulogy"), py_str("vows"), py_str("vow"), py_str("reading"), py_str("readings"), py_str("thank you"), py_str("thankyou"), py_str("card message"), py_str("christening card"), py_str("birthday card"), py_str("place card"), py_str("place cards"), py_str("menu card"), py_str("order of service"), py_str("how many guests"), py_str("guest list"), py_str("seating"), py_str("table plan"), py_str("when should i send"), py_str("when to send"), py_str("print"), py_str("printing"), py_str("printer"), py_str("birthday"), py_str("birthdays"), py_str("christening"), py_str("baptism"), py_str("baby shower"), py_str("baby's"), py_str("anniversary"), py_str("hen"), py_str("hen do"), py_str("stag"), py_str("engagement"), py_str("retirement"), py_str("housewarming"), py_str("house warming"), py_str("graduation"), py_str("party"), py_str("parties")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("plus one"), py_str("plus-one"), py_str("+1"), py_str("what is a plus one"), py_str("what does plus one mean"), py_str("bring a partner"), py_str("bring my partner"), py_str("bring my boyfriend"), py_str("bring my girlfriend"), py_str("bring my husband"), py_str("bring my wife"), py_str("bring someone"), py_str("bring a friend"), py_str("bring my friend"), py_str("a guest with me"), py_str("one seat")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("how many guests"), py_str("guest list"), py_str("seating"), py_str("table plan"), py_str("word"), py_str("wording"), py_str("write"), py_str("writing"), py_str("print"), py_str("print the"), py_str("decline"), py_str("turn down"), py_str("say no")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("send"), py_str("sends"), py_str("sending"), py_str("sent"), py_str("post"), py_str("posting"), py_str("out"), py_str("when"), py_str("how early"), py_str("how far in advance"), py_str("in advance"), py_str("ahead"), py_str("months before"), py_str("weeks before"), py_str("deadline"), py_str("how long before")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("word"), py_str("wording"), py_str("reword"), py_str("write"), py_str("writing"), py_str("design"), py_str("print"), py_str("printing"), py_str("printer"), py_str("template"), py_str("stationery"), py_str("rsvp"), py_str("reply"), py_str("replying"), py_str("respond"), py_str("accept"), py_str("decline"), py_str("plus one"), py_str("how many guests"), py_str("guest list"), py_str("seating"), py_str("table plan"), py_str("speech"), py_str("thank you"), py_str("place card"), py_str("order of service"), py_str("birthday"), py_str("christening"), py_str("baby shower"), py_str("funeral"), py_str("party"), py_str("parties"), py_str("hen do"), py_str("stag")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("tax"), py_str("hmrc"), py_str("self assessment"), py_str("vat"), py_str("census"), py_str("passport"), py_str("visa"), py_str("insurance"), py_str("subscription"), py_str("renew"), py_str("renewal"), py_str("plan"), py_str("gantt"), py_str("sprint"), py_str("client"), py_str("stakeholder"), py_str("roadmap"), py_str("milestone"), py_str("probation"), py_str("companies house"), py_str("confirmation statement"), py_str("annual return"), py_str("accounts"), py_str("statutory accounts"), py_str("court"), py_str("court date"), py_str("hearing"), py_str("tribunal"), py_str("mortgage"), py_str("planning permission"), py_str("landlord"), py_str("tenancy"), py_str("deposit"), py_str("council tax"), py_str("insurance claim"), py_str("claim form"), py_str("warranty"), py_str("application form")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("refund"), py_str("money back"), py_str("overcharged"), py_str("bill"), py_str("bills"), py_str("charge"), py_str("charges"), py_str("arrears"), py_str("to the"), py_str("to my"), py_str("to them"), py_str("with the"), py_str("for the")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("my mum"), py_str("my dad"), py_str("my ex"), py_str("my friend"), py_str("girlfriend"), py_str("boyfriend"), py_str("wife"), py_str("husband"), py_str("son"), py_str("daughter"), py_str("birthday"), py_str("wedding"), py_str("invite"), py_str("invitation"), py_str("cover letter"), py_str("resignation"), py_str("thank you"), py_str("condolence"), py_str("eulogy"), py_str("speech"), py_str("a girl"), py_str("a boy"), py_str("a date"), py_str("first date"), py_str("dating"), py_str("crush"), py_str("tinder"), py_str("after a date"), py_str("asking her out"), py_str("asking him out")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("landlord"), py_str("letting agent"), py_str("council"), py_str("mp"), py_str("msp"), py_str("councillor"), py_str("hospital"), py_str("clinic"), py_str("doctor"), py_str("gp"), py_str("school"), py_str("headteacher"), py_str("appeal"), py_str("insurance"), py_str("insurer"), py_str("neighbour"), py_str("employer"), py_str("company"), py_str("bank"), py_str("official"), py_str("housing"), py_str("council tax"), py_str("benefits"), py_str("dwp"), py_str("ombudsman"), py_str("solicitor")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("cover letter"), py_str("resignation"), py_str("thank you"), py_str("thanks"), py_str("reference"), py_str("cv"), py_str("will"), py_str("speech"), py_str("eulogy"), py_str("sick note"), py_str("condolence"), py_str("refund"), py_str("birthday"), py_str("christmas"), py_str("cake"), py_str("icing"), py_str("pipe"), py_str("love"), py_str("bin"), py_str("bins"), py_str("wheelie"), py_str("bin day"), py_str("collection"), py_str("collected"), py_str("waste"), py_str("recycling"), py_str("rubbish"), py_str("fly tipping"), py_str("or write"), py_str("or email"), py_str("or a letter"), py_str("or letter"), py_str("ring or"), py_str("call or"), py_str("phone or"), py_str("or should i ring"), py_str("or should i write"), py_str("understand"), py_str("understand it"), py_str("make sense"), py_str("makes sense"), py_str("confusing"), py_str("confused"), py_str("jargon"), py_str("cant follow"), py_str("dont follow"), py_str("explain it"), py_str("card"), py_str("to my mum"), py_str("to my dad"), py_str("to my friend")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("write"), py_str("writing"), py_str("draft"), py_str("drafting"), py_str("start"), py_str("starting"), py_str("begin"), py_str("beginning"), py_str("format"), py_str("layout"), py_str("lay out"), py_str("set out"), py_str("structure"), py_str("open"), py_str("opening"), py_str("end"), py_str("ending"), py_str("finish"), py_str("word"), py_str("words"), py_str("example"), py_str("template"), py_str("how do i"), py_str("how to")]) }
    fn __data35() -> Py { py_tuple(vec![py_str("complaint"), py_str("complain"), py_str("refund"), py_str("resignation"), py_str("resign"), py_str("leaving my job"), py_str("cv"), py_str("curriculum vitae"), py_str("cover letter"), py_str("covering letter"), py_str("reference"), py_str("thank you"), py_str("thanks"), py_str("eulogy"), py_str("condolence"), py_str("sick note"), py_str("love"), py_str("landlord"), py_str("letting agent"), py_str("council"), py_str("council tax"), py_str("housing"), py_str("mp"), py_str("msp"), py_str("dwp"), py_str("benefits"), py_str("ombudsman"), py_str("insurance"), py_str("insurer"), py_str("school"), py_str("appeal"), py_str("will"), py_str("speech"), py_str("cake"), py_str("icing"), py_str("pipe"), py_str("birthday"), py_str("christmas"), py_str("to my mum"), py_str("to my dad"), py_str("to my mother"), py_str("to my father"), py_str("to my friend"), py_str("yellow"), py_str("yellowing"), py_str("yellowed"), py_str("going yellow"), py_str("gone yellow"), py_str("musty"), py_str("fusty"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("fading"), py_str("faded"), py_str("foxing"), py_str("store"), py_str("storing"), py_str("preserve"), py_str("keep my"), py_str("keep old"), py_str("old letters"), py_str("old letter"), py_str("photos"), py_str("photographs"), py_str("postcards"), py_str("keepsakes"), py_str("the date"), py_str("date on a letter"), py_str("date in a letter"), py_str("what date"), py_str("date to put"), py_str("write the date"), py_str("address a letter"), py_str("addressing"), py_str("envelope"), py_str("envelopes"), py_str("stamp"), py_str("stamps"), py_str("scan"), py_str("scans"), py_str("scanned"), py_str("scanning"), py_str("photograph a letter"), py_str("post a letter"), py_str("posting"), py_str("postage"), py_str("salutation"), py_str("sign off"), py_str("signing off"), py_str("dear"), py_str("to a stranger"), py_str("stranger"), py_str("start a letter"), py_str("start my letter"), py_str("start the letter"), py_str("starting a letter"), py_str("begin a letter"), py_str("beginning a letter"), py_str("open a letter"), py_str("opening a letter"), py_str("end a letter"), py_str("ending a letter"), py_str("finish a letter"), py_str("finishing a letter"), py_str("close a letter"), py_str("closing a letter"), py_str("sign a letter"), py_str("signing a letter"), py_str("close"), py_str("closing"), py_str("closed"), py_str("sign"), py_str("signed"), py_str("signing"), py_str("signature"), py_str("last line"), py_str("end of a letter"), py_str("end of the letter"), py_str("first line"), py_str("opening line"), py_str("greeting"), py_str("salutation"), py_str("start"), py_str("starting"), py_str("start a"), py_str("begin"), py_str("beginning"), py_str("open"), py_str("opening"), py_str("finish"), py_str("finishing"), py_str("send a letter"), py_str("sending a letter"), py_str("letter to a company"), py_str("to a company"), py_str("someone i dont know"), py_str("somebody i dont know"), py_str("someone i don't know"), py_str("somebody i don't know"), py_str("understand"), py_str("understand it"), py_str("make sense"), py_str("makes sense"), py_str("make of it"), py_str("confusing"), py_str("confused"), py_str("jargon"), py_str("cant follow"), py_str("dont follow"), py_str("explain it"), py_str("fold"), py_str("folds"), py_str("folding"), py_str("seal"), py_str("seals"), py_str("sealing"), py_str("sealed"), py_str("santa"), py_str("father christmas"), py_str("to santa"), py_str("daughter"), py_str("son"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("future self"), py_str("wedding"), py_str("anniversary"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("grandson"), py_str("granddaughter"), py_str("bank"), py_str("banks"), py_str("building society"), py_str("hospital"), py_str("doctor"), py_str("gp"), py_str("solicitor"), py_str("neighbour"), py_str("neighbor"), py_str("pay rise"), py_str("payrise"), py_str("pay raise"), py_str("more money"), py_str("salary increase"), py_str("apology"), py_str("apologies")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("letting agent"), py_str("council"), py_str("housing"), py_str("bank"), py_str("insurance"), py_str("insurer"), py_str("mp"), py_str("msp"), py_str("councillor"), py_str("school"), py_str("employer"), py_str("ombudsman"), py_str("dwp"), py_str("benefits"), py_str("council tax")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("cake"), py_str("icing"), py_str("pipe"), py_str("broken up"), py_str("ex"), py_str("cheating"), py_str("not sure i love"), py_str("falling out of love"), py_str("does he love me"), py_str("does she love me"), py_str("valentine card"), py_str("valentines card"), py_str("valentine's card")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("email"), py_str("message"), py_str("text"), py_str("letter"), py_str("write"), py_str("properly"), py_str("friend"), py_str("boss"), py_str("customer"), py_str("customers"), py_str("client"), py_str("clients"), py_str("company"), py_str("order"), py_str("refund"), py_str("grovelling"), py_str("professional"), py_str("colleague"), py_str("colleagues"), py_str("staff"), py_str("work"), py_str("at work"), py_str("office"), py_str("meeting"), py_str("team")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("my card"), py_str("my credit card"), py_str("my debit card"), py_str("my loan"), py_str("my payment"), py_str("my benefit"), py_str("my benefits"), py_str("my uc"), py_str("my universal credit"), py_str("my pip"), py_str("my esa"), py_str("my child benefit"), py_str("my tax credits"), py_str("benefit payment"), py_str("benefits payment"), py_str("dwp"), py_str("jobcentre"), py_str("job centre"), py_str("hardship payment"), py_str("my direct debit"), py_str("my mortgage"), py_str("my rent"), py_str("my bill"), py_str("my insurance"), py_str("i missed"), py_str("i forgot"), py_str("i didnt pay"), py_str("i didn't pay"), py_str("i cant pay"), py_str("i can't pay"), py_str("cant pay"), py_str("can't pay"), py_str("they charged me"), py_str("charged me"), py_str("behind on"), py_str("be paid for"), py_str("paid for it"), py_str("get paid"), py_str("am i paid"), py_str("being paid"), py_str("paid overtime"), py_str("overtime"), py_str("work late"), py_str("working late"), py_str("work extra"), py_str("hours i work"), py_str("for the hours")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("handwriting"), py_str("signature"), py_str("sign"), py_str("spell"), py_str("spelling"), py_str("letter to"), py_str("speaking"), py_str("speak"), py_str("spoken"), py_str("pronunciation"), py_str("accent"), py_str("listening"), py_str("cover letter"), py_str("cv"), py_str("resume"), py_str("card"), py_str("cards"), py_str("email to"), py_str("text to")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("for feedback"), py_str("ask for feedback"), py_str("asking for feedback"), py_str("my teacher"), py_str("my lecturer"), py_str("my tutor"), py_str("my professor"), py_str("after an interview"), py_str("after the interview"), py_str("after my interview"), py_str("interview"), py_str("interviews"), py_str("interviewer"), py_str("interviewers"), py_str("a shop"), py_str("for a shop"), py_str("a restaurant"), py_str("for a restaurant"), py_str("a hotel"), py_str("for a hotel"), py_str("a product"), py_str("my order"), py_str("the seller"), py_str("the app"), py_str("review"), py_str("reviews")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("tells me"), py_str("told me"), py_str("tells us"), py_str("told us"), py_str("someone tells"), py_str("someone told"), py_str("somebody tells"), py_str("somebody told"), py_str("my friend"), py_str("my mate"), py_str("my mum"), py_str("my dad"), py_str("my sister"), py_str("my brother"), py_str("my partner"), py_str("my husband"), py_str("my wife"), py_str("in person"), py_str("face to face"), py_str("phone call"), py_str("on the phone"), py_str("a friend")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("misspell"), py_str("grammar"), py_str("noun"), py_str("verb"), py_str("adjective"), py_str("adverb"), py_str("pronoun"), py_str("preposition"), py_str("conjunction"), py_str("semicolon"), py_str("apostrophe"), py_str("comma"), py_str("synonym"), py_str("metaphor"), py_str("simile"), py_str("irony"), py_str("ironic"), py_str("hyphen"), py_str("bisect"), py_str("appointment"), py_str("medical"), py_str("mean for"), py_str("means for"), py_str("mean for my"), py_str("means for my"), py_str("formula 1"), py_str("formula one"), py_str("f1"), py_str("grand prix"), py_str("race"), py_str("racing"), py_str("football"), py_str("rugby"), py_str("cricket"), py_str("match"), py_str("x ray"), py_str("x-ray"), py_str("scan"), py_str("mri"), py_str("ultrasound"), py_str("chest"), py_str("lung"), py_str("lungs"), py_str("lump"), py_str("mole"), py_str("tooth"), py_str("teeth")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("mean"), py_str("means"), py_str("meaning"), py_str("meanings"), py_str("define"), py_str("definition"), py_str("explain"), py_str("what does"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("called"), py_str("stand for"), py_str("refers")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("recipe"), py_str("recipes"), py_str("cook"), py_str("cooking"), py_str("bake"), py_str("how long"), py_str("how many"), py_str("how do i"), py_str("how can i"), py_str("what should i"), py_str("help me"), py_str("what do i do"), py_str("hurts"), py_str("hurting"), py_str("pain"), py_str("painful"), py_str("sore"), py_str("ache"), py_str("aches"), py_str("aching")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("use"), py_str("when"), py_str("instead"), py_str("list"), py_str("semicolon"), py_str("difference"), py_str("do"), py_str("does"), py_str("mean"), py_str("meaning"), py_str("for"), py_str("used"), py_str("capital"), py_str("capitalise"), py_str("capitalize"), py_str("explain"), py_str("what")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("apostrophe"), py_str("apostrophes"), py_str("correct"), py_str("correctly"), py_str("right"), py_str("wrong"), py_str("need"), py_str("needs"), py_str("needed"), py_str("should"), py_str("where"), py_str("why"), py_str("which"), py_str("when"), py_str("how"), py_str("use"), py_str("used"), py_str("spell"), py_str("spelling"), py_str("spelled"), py_str("spelt"), py_str("grammar"), py_str("punctuate"), py_str("punctuation"), py_str("matter"), py_str("difference"), py_str("write"), py_str("writing"), py_str("written"), py_str("is"), py_str("are"), py_str("or")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("when"), py_str("use"), py_str("where"), py_str("put"), py_str("need"), py_str("go"), py_str("rule"), py_str("rules"), py_str("correctly"), py_str("properly"), py_str("sentence"), py_str("pause"), py_str("what"), py_str("whats"), py_str("what is"), py_str("mean"), py_str("meaning"), py_str("called"), py_str("explain"), py_str("definition"), py_str("why")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("use"), py_str("when"), py_str("need"), py_str("sentence"), py_str("end"), py_str("instead"), py_str("how many"), py_str("where"), py_str("what"), py_str("whats"), py_str("what is"), py_str("whats a"), py_str("mean"), py_str("meaning"), py_str("called"), py_str("explain"), py_str("definition"), py_str("for"), py_str("do"), py_str("does"), py_str("why")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("filler"), py_str("fillers"), py_str("filler words"), py_str("um"), py_str("ums"), py_str("erm"), py_str("er"), py_str("mumble"), py_str("mumbling"), py_str("mumbles"), py_str("speak"), py_str("speaking"), py_str("talk"), py_str("talking"), py_str("voice"), py_str("monotone")]) }
    fn __data51() -> Py { py_tuple(vec![py_str("clearly"), py_str("clearer"), py_str("filler"), py_str("fillers"), py_str("um"), py_str("ums"), py_str("erm"), py_str("confident"), py_str("confidence"), py_str("confidently"), py_str("nervous"), py_str("nerves"), py_str("stop saying"), py_str("when i talk"), py_str("when i speak"), py_str("stop using"), py_str("mumbling"), py_str("monotone"), py_str("too fast"), py_str("too quiet"), py_str("shaking"), py_str("shaky"), py_str("wobble"), py_str("wobbly"), py_str("wavers"), py_str("slowly"), py_str("slower"), py_str("too quietly"), py_str("so quietly"), py_str("more quietly"), py_str("quietly"), py_str("quieter"), py_str("louder"), py_str("too loud"), py_str("too loudly"), py_str("speak louder"), py_str("talk louder"), py_str("speak up"), py_str("project my voice"), py_str("project your voice"), py_str("stop mumbling"), py_str("stop being"), py_str("say um"), py_str("say erm"), py_str("too much"), py_str("less monotone"), py_str("sound monotone"), py_str("sound"), py_str("sounds"), py_str("talk fast"), py_str("speak fast"), py_str("fast when")]) }
    fn __data52() -> Py { py_tuple(vec![py_str("my boss"), py_str("my manager"), py_str("my landlord"), py_str("my partner"), py_str("my husband"), py_str("my wife"), py_str("denture"), py_str("dentures"), py_str("false teeth"), py_str("dental plate"), py_str("my plate"), py_str("my tooth"), py_str("my teeth"), py_str("invisalign"), py_str("retainer"), py_str("my mum"), py_str("my dad"), py_str("my son"), py_str("my daughter"), py_str("my friend"), py_str("the doctor"), py_str("gp"), py_str("talk to"), py_str("talking to"), py_str("talk about"), py_str("talking about"), py_str("speak to"), py_str("speaking to"), py_str("speak about"), py_str("talk too much"), py_str("talks too much"), py_str("talking too much"), py_str("talk over"), py_str("talks over"), py_str("talking over"), py_str("interrupt"), py_str("interrupts"), py_str("interrupting"), py_str("someone they"), py_str("somebody they"), py_str("someone who"), py_str("somebody who"), py_str("they talk"), py_str("they speak"), py_str("keep talking"), py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("teens"), py_str("teenage"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("my mother"), py_str("my father"), py_str("my parents"), py_str("my sister"), py_str("my brother"), py_str("my family"), py_str("teacher"), py_str("colleague"), py_str("coworker"), py_str("neighbour"), py_str("the school"), py_str("the council"), py_str("about pay"), py_str("about a rise"), py_str("about money"), py_str("ask for"), py_str("complaint"), py_str("complain"), py_str("in meetings"), py_str("in a meeting"), py_str("at the meeting"), py_str("speak up in"), py_str("speak up at"), py_str("speak in meetings"), py_str("talk in meetings"), py_str("standup"), py_str("stand-up meeting"), py_str("stand up meeting"), py_str("team meeting"), py_str("contribute in"), py_str("in public"), py_str("public speaking"), py_str("speech"), py_str("presentation"), py_str("audience"), py_str("wedding"), py_str("best man"), py_str("maid of honour"), py_str("toast"), py_str("eulogy"), py_str("interview")]) }
    fn __data53() -> Py { py_tuple(vec![py_str("rude"), py_str("politely"), py_str("polite"), py_str("awkward"), py_str("drama"), py_str("offend"), py_str("offending"), py_str("silent"), py_str("quietly"), py_str("straight out"), py_str("remove myself"), py_str("should i"), py_str("being a thing"), py_str("without it being"), py_str("without anyone"), py_str("notice"), py_str("notices"), py_str("noticed")]) }
    fn __data54() -> Py { py_tuple(vec![py_str("job"), py_str("work"), py_str("boss"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("marriage"), py_str("school"), py_str("college"), py_str("course"), py_str("husband"), py_str("band"), py_str("choir"), py_str("running club"), py_str("cult"), py_str("facebook"), py_str("admin"), py_str("invite"), py_str("add me"), py_str("create"), py_str("set up"), py_str("make a group")]) }
    fn __data55() -> Py { py_tuple(vec![py_str("start"), py_str("starting"), py_str("begin"), py_str("greeting"), py_str("opening"), py_str("first"), py_str("never met"), py_str("stranger"), py_str("dear"), py_str("hi"), py_str("hello"), py_str("dear or hi"), py_str("hi or dear"), py_str("dear sir")]) }
    fn __data56() -> Py { py_tuple(vec![py_str("dear sir"), py_str("dear madam"), py_str("dear sir or madam"), py_str("sir or madam"), py_str("dear madam or sir"), py_str("salutation"), py_str("to whom it may concern"), py_str("address a letter"), py_str("address a formal letter"), py_str("dear"), py_str("start of a letter"), py_str("the start of a letter"), py_str("start a letter"), py_str("starting a letter"), py_str("begin a letter"), py_str("beginning a letter"), py_str("beginning of a letter"), py_str("the beginning of a letter"), py_str("open a letter"), py_str("opening a letter"), py_str("opening line"), py_str("first line of a letter")]) }
    fn __data57() -> Py { py_tuple(vec![py_str("use"), py_str("letter"), py_str("letters"), py_str("email"), py_str("formal"), py_str("start"), py_str("addressing"), py_str("address"), py_str("write"), py_str("madam"), py_str("sir"), py_str("dear"), py_str("name"), py_str("right"), py_str("rude"), py_str("ok"), py_str("okay"), py_str("polite")]) }
    fn __data58() -> Py { py_tuple(vec![py_str("card"), py_str("cards"), py_str("note"), py_str("txt"), py_str("text"), py_str("birthday"), py_str("christmas"), py_str("wedding"), py_str("condolence"), py_str("thank you"), py_str("complaint"), py_str("refund"), py_str("cover letter"), py_str("cv"), py_str("resume"), py_str("applying for"), py_str("resignation"), py_str("resigning"), py_str("my boss"), py_str("my landlord"), py_str("my neighbour"), py_str("my teacher"), py_str("my ex"), py_str("my mum"), py_str("my dad"), py_str("my friend")]) }
    fn __data59() -> Py { py_tuple(vec![py_str("long"), py_str("length"), py_str("short"), py_str("how long"), py_str("should"), py_str("split"), py_str("writing"), py_str("sentences"), py_str("what is"), py_str("whats"), py_str("definition"), py_str("mean"), py_str("means"), py_str("how many"), py_str("words"), py_str("much")]) }
    fn __data60() -> Py { py_tuple(vec![py_str("repeat that"), py_str("say that again"), py_str("say it again"), py_str("repeat after me"), py_str("same as before"), py_str("again please"), py_str("what did you say"), py_str("toddler"), py_str("child"), py_str("children"), py_str("son"), py_str("daughter"), py_str("baby"), py_str("kid"), py_str("kids"), py_str("my mum"), py_str("my mom"), py_str("my dad"), py_str("nan"), py_str("grandad"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("speech"), py_str("hearing"), py_str("autistic"), py_str("autism"), py_str("dementia"), py_str("alzheimers"), py_str("alzheimer's"), py_str("keeps saying"), py_str("same word"), py_str("sentence he"), py_str("sentence she"), py_str("stutter"), py_str("stammer"), py_str("tourettes")]) }
    fn __data61() -> Py { py_tuple(vec![py_str("letter"), py_str("letters"), py_str("email"), py_str("emails"), py_str("note"), py_str("notes"), py_str("memo"), py_str("message"), py_str("formal"), py_str("best"), py_str("kind"), py_str("regards"), py_str("signature"), py_str("sincerely"), py_str("faithfully")]) }
    fn __data62() -> Py { py_tuple(vec![py_str("sick note"), py_str("fit note"), py_str("thank you"), py_str("cover letter"), py_str("covering letter"), py_str("cv"), py_str("resume"), py_str("account"), py_str("accounts"), py_str("log in"), py_str("login"), py_str("sign in"), py_str("sign up"), py_str("unsubscribe"), py_str("subscribe"), py_str("subscription"), py_str("subscriptions"), py_str("lease"), py_str("tenancy"), py_str("contract"), py_str("deed"), py_str("passport"), py_str("cheque"), py_str("mortgage"), py_str("agreement"), py_str("licence"), py_str("license"), py_str("bill"), py_str("bills"), py_str("a text"), py_str("text message"), py_str("texting"), py_str("txt"), py_str("note taking"), py_str("note-taking"), py_str("taking notes"), py_str("study notes")]) }
    fn __data63() -> Py { py_tuple(vec![py_str("thank you"), py_str("cover letter"), py_str("covering letter"), py_str("cv"), py_str("resume"), py_str("account"), py_str("accounts"), py_str("log in"), py_str("login"), py_str("sign in"), py_str("sign up"), py_str("unsubscribe"), py_str("subscribe"), py_str("subscription"), py_str("subscriptions"), py_str("lease"), py_str("tenancy"), py_str("contract"), py_str("deed"), py_str("passport"), py_str("cheque"), py_str("mortgage"), py_str("agreement"), py_str("licence"), py_str("license"), py_str("bill"), py_str("bills"), py_str("a text"), py_str("text message"), py_str("texting"), py_str("txt"), py_str("note taking"), py_str("note-taking"), py_str("taking notes"), py_str("study notes"), py_str("essay"), py_str("essays"), py_str("complaint"), py_str("complaints"), py_str("complaint letter"), py_str("letter of complaint"), py_str("landlord"), py_str("landlords"), py_str("letting agent"), py_str("mp"), py_str("mps"), py_str("mp's"), py_str("member of parliament"), py_str("resignation"), py_str("resigning"), py_str("reference"), py_str("references"), py_str("referee"), py_str("appeal"), py_str("appeals"), py_str("sick note"), py_str("fit note")]) }
    fn __data9() -> Py { py_list(vec![pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("affect-effect"), py_tuple(vec![py_tuple(vec![py_str("affect"), py_str("affects"), py_str("affected")]), py_tuple(vec![py_str("effect"), py_str("effects")])]), py_str("Affect is the verb and effect is the noun: the weather affects my mood, and the weather has an effect on my mood. A useful memory: 'cause and effect' is always the noun.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("its-its"), py_tuple(vec![py_tuple(vec![py_str("its")]), py_tuple(vec![py_str("apostrophe"), py_str("difference"), py_str("correct"), py_str("grammar"), py_str("punctuation")]), py_tuple(vec![py_str("its or it's"), py_str("it's or its")])]), py_str("It's with an apostrophe always means 'it is' or 'it has': it's raining, it's been ages. Everything else is its, no apostrophe: the cat licked its paw. If you can swap in 'it is' and the sentence still works, use it's.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("your-youre"), py_tuple(vec![py_tuple(vec![py_str("your")]), py_tuple(vec![py_str("youre"), py_str("apostrophe"), py_str("difference"), py_str("grammar")]), py_tuple(vec![py_str("your or you're"), py_str("you're or your")])]), py_str("You're means 'you are', so 'you're welcome' and 'you're late' take it. Your shows ownership: your coat, your turn. Read the sentence with 'you are' in it; if that sounds wrong, use your.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("their-there"), py_tuple(vec![py_tuple(vec![py_str("their"), py_str("theirs")]), py_tuple(vec![py_str("there"), py_str("theyre"), py_str("apostrophe"), py_str("difference")]), py_tuple(vec![py_str("their or there"), py_str("there or their"), py_str("their there theyre")])]), py_str("They're means 'they are', their means it belongs to them, and there is a place or 'there is'. So: they're late, their car, over there. In a hurry, ask what the word is doing: person, owner or place.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("fewer-less"), py_tuple(vec![py_tuple(vec![py_str("fewer")]), py_tuple(vec![py_str("less")])]), py_func("_fewer", |args: &[Py]| _fewer(args))]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("lose-loose"), py_tuple(vec![py_tuple(vec![py_str("lose"), py_str("losing"), py_str("lost")]), py_tuple(vec![py_str("loose"), py_str("looser")])]), py_str("Lose has one o and means not winning or not being able to find something; loose has two and means not tight. Your trousers are loose, and you might lose them.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("then-than"), py_tuple(vec![py_tuple(vec![py_str("then")]), py_tuple(vec![py_str("than")])]), py_str("Then is about time, than is about comparing: we ate, then we walked; she is taller than me. If you can put 'at that time' in place of it, it's then.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("could-of"), py_tuple(vec![py_tuple(vec![py_str("could've"), py_str("could of"), py_str("should of"), py_str("would of"), py_str("couldve"), py_str("shouldve")]), py_tuple(vec![py_str("could have"), py_str("should have"), py_str("would have"), py_str("correct"), py_str("grammar"), py_str("write"), py_str("spell"), py_str("wrong")])]), py_str("It's 'could have', usually shortened to could've, never 'could of'. The same goes for should've and would've: the sound is 'of' but the word is 'have'.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("practice-practise"), py_tuple(vec![py_tuple(vec![py_str("practice")]), py_tuple(vec![py_str("practise"), py_str("advice"), py_str("advise")])]), py_str("In British English the noun is practice and the verb is practise: piano practice, but I practise every day. Advice and advise work the same way: advice is the thing, advise is what you do. In American English practice does both jobs.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("who-whom"), py_tuple(vec![py_tuple(vec![py_str("whom")]), py_tuple(vec![py_str("who")])]), py_str("Whom is only for the object of a sentence, and in everyday writing who is almost always fine. Use whom after a preposition, as in 'to whom it may concern' or 'for whom'. If it sounds stiff, rewrite the sentence instead.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("complement-compliment"), py_tuple(vec![py_tuple(vec![py_str("complement")]), py_tuple(vec![py_str("compliment")])]), py_str("A compliment is praise; a complement completes something, like a wine that complements the meal. 'Complimentary' means free, which is why hotel soap is complimentary.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("amount-number"), py_tuple(vec![py_tuple(vec![py_str("amount")]), py_tuple(vec![py_str("number"), py_str("much"), py_str("many")])]), py_str("Use number for things you can count and amount for things you can't: a number of people, a large amount of work. 'Amount of people' is the usual slip.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("ie-eg"), py_tuple(vec![py_tuple(vec![py_str("ie"), py_str("eg")]), py_tuple(vec![py_str("difference"), py_str("mean"), py_str("use")])]), py_str("e.g. means 'for example' (think 'example given') and i.e. means 'that is' (a restatement). Both take full stops in British style, and both want a comma after them.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("alot-a-lot"), py_tuple(vec![py_tuple(vec![py_str("alot")]), py_tuple(vec![py_str("a lot"), py_str("one word"), py_str("two words"), py_str("spell"), py_str("spelling"), py_str("correct"), py_str("write"), py_str("writing"), py_str("wrong"), py_str("is it"), py_str("should it be")])]), py_str("It is always two words: a lot. 'Alot' is not a word, so if you would write 'a lot of 'people' or 'a lot better', put the space in.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("to-too")); __args.push(py_tuple(vec![py_tuple(vec![py_str("too")]), py_tuple(vec![py_str("to")])])); __args.push(py_str("To is the direction or the infinitive - to the shop, to run. Too means also or more than you want - I want to come too, it's too late. If you could swap in 'also' or 'very', it's too.")); __kws.extend(vec![("avoid".to_string(), __data10()), ("ask".to_string(), __data11())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("quiet-quite")); __args.push(py_tuple(vec![py_tuple(vec![py_str("quiet")]), py_tuple(vec![py_str("quite")])])); __args.push(py_str("Quiet means little noise - a quiet street. Quite means fairly or completely - quite good, quite sure. The e and the t swap places, so check which word you meant, not which one looks right.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("difference"), py_str("correct"), py_str("spell"), py_str("spelling"), py_str("grammar"), py_str("should it be"), py_str("do i use"), py_str("quiet or"), py_str("or quite")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("apart-a-part")); __args.push(py_tuple(vec![py_tuple(vec![py_str("apart")]), py_tuple(vec![py_str("a part")])])); __args.push(py_str("Apart means separated - we live apart, take it apart. A part means one piece of something - a part of the team, a part of the problem. If you can drop the 'a', it's apart.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("difference"), py_str("correct"), py_str("spell"), py_str("spelling"), py_str("grammar"), py_str("should it be"), py_str("do i use"), py_str("apart or"), py_str("or a part")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("everyday-every-day")); __args.push(py_tuple(vec![py_tuple(vec![py_str("everyday")]), py_tuple(vec![py_str("every day")])])); __args.push(py_str("Every day, two words, means each day - I walk every day. Everyday, one word, is the adjective for ordinary - everyday shoes. If you can say 'each day', keep the space.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("difference"), py_str("correct"), py_str("spell"), py_str("spelling"), py_str("grammar"), py_str("should it be"), py_str("do i use"), py_str("everyday or"), py_str("or every day")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("anyway-any-way")); __args.push(py_tuple(vec![py_tuple(vec![py_str("anyway")]), py_tuple(vec![py_str("any way")])])); __args.push(py_str("Anyway means in any case - anyway, I went home. Any way means any method - is there any way to fix it. If you can put 'in any case' there, it's one word.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("difference"), py_str("correct"), py_str("spell"), py_str("spelling"), py_str("grammar"), py_str("should it be"), py_str("do i use"), py_str("anyway or"), py_str("or any way")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("onto-on-to")); __args.push(py_tuple(vec![py_tuple(vec![py_str("onto")]), py_tuple(vec![py_str("on to")])])); __args.push(py_str("Onto is one word when something moves onto a surface - he climbed onto the roof. Keep them apart when 'on' belongs to the verb - we carried on to the next town, hold on to it. If you can't move the 'on' away, it's one word.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("difference"), py_str("correct"), py_str("spell"), py_str("spelling"), py_str("grammar"), py_str("should it be"), py_str("do i use"), py_str("onto or"), py_str("or on to")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("altogether-all-together")); __args.push(py_tuple(vec![py_tuple(vec![py_str("altogether")]), py_tuple(vec![py_str("all together")])])); __args.push(py_str("All together means everyone in one place - we sang all together. Altogether means completely or in total - that's altogether different, ten pounds altogether.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("difference"), py_str("correct"), py_str("spell"), py_str("spelling"), py_str("grammar"), py_str("should it be"), py_str("do i use"), py_str("altogether or"), py_str("or all together")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("desert-dessert"), py_tuple(vec![py_tuple(vec![py_str("desert")]), py_tuple(vec![py_str("dessert")])]), py_str("Dessert is the sweet course, and it has two s's because you want more of it. Desert, one s, is sand or the verb to abandon; the stress moves too - DESSert, deSERT.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("stationery-stationary"), py_tuple(vec![py_tuple(vec![py_str("stationery"), py_str("stationary")]), py_tuple(vec![py_str("stationary"), py_str("stationery")])]), py_str("Stationery with an e is paper and pens - remember e for envelope. Stationary with an a is not moving - a stationary car.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("principal-principle"), py_tuple(vec![py_tuple(vec![py_str("principal")]), py_tuple(vec![py_str("principle")])]), py_str("Principal means main or the head of a school; principle is a rule or belief - it's the principle of the thing. The school one is your pal: principAL, the princiPAL.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("passed-past"), py_tuple(vec![py_tuple(vec![py_str("passed")]), py_tuple(vec![py_str("past")])]), py_str("Passed is the verb - I passed the shop, she passed the exam. Past is time before now, or beyond something - in the past, walk past the bank.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("whose-whos"), py_tuple(vec![py_tuple(vec![py_str("whose")]), py_tuple(vec![py_str("who's"), py_str("whos")])]), py_str("Whose asks who owns it - whose coat is this. Who's is short for who is or who has - who's coming? If you can say 'who is', put the apostrophe in.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("me-or-i"), py_tuple(vec![__data12(), py_tuple(vec![py_str("or"), py_str("correct"), py_str("grammar"), py_str("right"), py_str("wrong")])]), py_str("Use 'me' after a preposition or when you're the one being acted on: between you and me, she phoned me. Use 'I' only as the subject of the verb: my sister and I went. 'Between you and I' is the mistake people over-correct into.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("advice-advise"), py_tuple(vec![py_tuple(vec![py_str("advice")]), py_tuple(vec![py_str("advise"), py_str("difference"), py_str("spell"), py_str("correct"), py_str("use")])]), py_str("Advice is the noun, advise is the verb: give advice, and I'd advise you to wait. Both are said with a soft 's' in advice and a 'z' sound in advise, which is the memory hook: advi-s-e, it doe-s.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("could-care-less"), py_tuple(vec![py_tuple(vec![py_str("could care less"), py_str("i could care less")]), py_tuple(vec![py_str("couldn't care less"), py_str("couldnt care less"), py_str("could not care less"), py_str("mean"), py_str("correct"), py_str("right"), py_str("wrong"), py_str("rude"), py_str("say")])]), py_str("'I couldn't care less' is the one that means what it says - I care so little that I have no less to give. 'I could care less' says the opposite, though people use it that way anyway; in writing, use couldn't.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("capital-letters"), py_tuple(vec![py_tuple(vec![py_str("capitals"), py_str("capital letters"), py_str("capitalisation"), py_str("capitalization"), py_str("capitalise"), py_str("capitalize"), py_str("uppercase"), py_str("upper case")]), py_tuple(vec![py_str("when"), py_str("use"), py_str("rules"), py_str("which")])]), py_str("Capitalise the first word of a sentence, names of people, places, days and months, and titles before a name: Nadia, Cardiff, Tuesday, Doctor Reid. Don't capitalise seasons, job titles on their own, or words in the middle of a sentence just because they feel important.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-definitely"), py_tuple(vec![py_tuple(vec![py_str("definitely"), py_str("definately"), py_str("definatly")]), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("correct")])]), py_str("It's definitely: de-fin-ite-ly, with 'finite' hidden in the middle. The misspelling 'definately' is the one people write most, so check for that a in the middle.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-necessary"), py_tuple(vec![py_tuple(vec![py_str("necessary"), py_str("neccessary"), py_str("necesary")]), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("correct")])]), py_str("It's necessary: one c, two s's. A one-collar shirt, two sleeves — one c, two s.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-accommodation"), py_tuple(vec![py_tuple(vec![py_str("accommodation"), py_str("accomodation")]), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("correct")])]), py_str("It's accommodation: two c's and two m's, because it has room for everyone. Same for accommodate and accommodating.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-separate"), py_tuple(vec![py_tuple(vec![py_str("separate"), py_str("seperate")]), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("correct")])]), py_str("It's separate: sep-a-rate, with 'a rat' in the middle. There is no e after the p.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-receive"), py_tuple(vec![py_tuple(vec![py_str("receive"), py_str("recieve")]), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("correct")])]), py_str("It's receive: i before e, except after c. The same rule gives believe, achieve and piece, and it is only a rule of thumb — weird, seize and their all break it.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-embarrass"), py_tuple(vec![py_tuple(vec![py_str("embarrass"), py_str("embarass"), py_str("embarrassed")]), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("correct")])]), py_str("It's embarrass: two r's and two s's. Think of it as being doubly awkward.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-rhythm"), py_tuple(vec![py_tuple(vec![py_str("rhythm"), py_str("rythm")]), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("correct")])]), py_str("It's rhythm: r-h-y-t-h-m, with no vowel between the y and the t. It is one of the few English words where y does the vowel's job.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-restaurant"), py_tuple(vec![py_tuple(vec![py_str("restaurant"), py_str("restaraunt"), py_str("restraunt"), py_str("restarant")]), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("correct")])]), py_str("It's restaurant: r-e-s-t-a-u-r-a-n-t, with 'aunt' in the middle after the u. The usual slips are restaraunt and restraunt - remember the au, as in the French it comes from.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-uk-us"), py_tuple(vec![py_tuple(vec![py_str("english"), py_str("spelling"), py_str("spell")]), py_tuple(vec![py_str("british"), py_str("american"), py_str("uk"), py_str("difference")])]), py_str("Both are correct; just be consistent. British English uses colour, organise, centre and travelled, and American English uses color, organize, center and traveled. Most word processors can set the dictionary to the one you want.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spell-check"), py_tuple(vec![py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("misspelt"), py_str("misspelled")]), py_tuple(vec![py_str("check"), py_str("correctly"), py_str("autocorrect"), py_str("tool"), py_str("mistakes")])]), py_str("Turn on the spell checker in whatever you're writing in and read it once more yourself: a checker catches typos but not the wrong word, so its, it's, your and you're still need your eyes.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("apostrophes-plural"), py_tuple(vec![py_tuple(vec![py_str("apostrophe"), py_str("apostrophes")]), py_tuple(vec![py_str("plural"), py_str("plurals"), py_str("cds"), py_str("apple's"), py_str("plural or")])]), py_str("Plurals never take an apostrophe: two apples, the 1990s, CDs. Keep the apostrophe for missing letters (it's, don't) and for belonging (the dog's bowl). If it's plural, leave it out.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("apostrophe-after-s"), py_tuple(vec![__data13(), __data14()]), py_str("Names ending in s take an apostrophe and another s: James's coat, Chris's car, the class's timetable. You will also see James' and Chris' in older British style and some publishers still print it, so either is defensible - just keep to one in the same document. It is only plurals that take no apostrophe at all: the Joneses are coming.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("semicolon")); __args.push(py_tuple(vec![py_tuple(vec![py_str("semicolon"), py_str("semi-colon")]), py_tuple(vec![py_str("use"), py_str("when"), py_str("instead"), py_str("comma"), py_str("do"), py_str("does"), py_str("mean"), py_str("meaning"), py_str("for"), py_str("explain"), py_str("what")])])); __args.push(py_str("Use a semicolon to join two full sentences that are closely related: I missed the bus; the next one was an hour away. If a full stop would work, either is fine — a semicolon just says the two halves belong together.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("python"), py_str("javascript"), py_str("java"), py_str(" sql"), py_str("coding"), py_str("programming"), py_str("php"), py_str("css")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("i-before-e"), py_tuple(vec![py_tuple(vec![py_str("i before e")]), py_tuple(vec![py_str("rule"), py_str("spell"), py_str("spelling"), py_str("except"), py_str("believe"), py_str("receive")])]), py_str("It's i before e except after c: believe, piece, but receive, ceiling. It's only a rule of thumb, though — weird, seize, their and either all break it, so use it as a nudge and learn the exceptions you write often.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("comma-splice"), py_tuple(vec![py_tuple(vec![py_str("comma"), py_str("commas")]), py_tuple(vec![py_str("splice"), py_str("joining"), py_str("two sentences"), py_str("run on"), py_str("run-on")])]), py_str("Two complete sentences can't be held together by a comma alone. Use a full stop, a semicolon, or a comma plus and/but/because: I was late, so I skipped breakfast.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("ellipsis"), py_tuple(vec![py_tuple(vec![py_str("ellipsis"), py_str("ellipses"), py_str("three dots"), py_str("dot dot dot")]), py_tuple(vec![py_str("use"), py_str("used"), py_str("when"), py_str("mean"), py_str("meaning"), py_str("do"), py_str("does"), py_str("for"), py_str("writing"), py_str("what")])]), py_func("_ellipsis_reply", |args: &[Py]| _ellipsis_reply(args))]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("brackets-punctuation")); __args.push(py_tuple(vec![py_tuple(vec![py_str("brackets"), py_str("bracket"), py_str("parentheses"), py_str("parenthesis")]), py_tuple(vec![py_str("use"), py_str("used"), py_str("when"), py_str("mean"), py_str("meaning"), py_str("do"), py_str("does"), py_str("for"), py_str("writing"), py_str("punctuation"), py_str("what")])])); __args.push(py_str("Brackets hold an aside the sentence would survive without: the date of a law, a translation, a word the reader may not know. Read the sentence without it - if the grammar still works, the brackets are doing their job. In British style the full stop goes outside them unless the bracket is a whole sentence of its own.")); __kws.extend(vec![("guard".to_string(), py_func("_brackets_are_punctuation", |args: &[Py]| _brackets_are_punctuation(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("speech-marks")); __args.push(py_tuple(vec![py_tuple(vec![py_str("speech marks"), py_str("quotation marks"), py_str("quote marks"), py_str("inverted commas"), py_str("double quotes"), py_str("single quotes"), py_str("quotes")]), py_tuple(vec![py_str("use"), py_str("used"), py_str("when"), py_str("mean"), py_str("do"), py_str("does"), py_str("for"), py_str("writing"), py_str("what"), py_str("how")])])); __args.push(py_func("_speech_marks_reply", |args: &[Py]| _speech_marks_reply(args))); __kws.extend(vec![("avoid".to_string(), __data15())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("list-punctuation")); __args.push(py_tuple(vec![py_tuple(vec![py_str("list"), py_str("lists"), py_str("bullet points"), py_str("bullets"), py_str("bulleted list")]), py_tuple(vec![py_str("punctuation"), py_str("punctuate"), py_str("punctuating"), py_str("write"), py_str("writing"), py_str("sentence"), py_str("sentences"), py_str("grammar"), py_str("paragraph"), py_str("paragraphs")])])); __args.push(py_func("_list_reply", |args: &[Py]| _list_reply(args))); __kws.extend(vec![("avoid".to_string(), __data16())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dash-hyphen")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dash"), py_str("hyphen"), py_str("dashes"), py_str("hyphens"), py_str("em dash"), py_str("en dash"), py_str("em-dash"), py_str("en-dash"), py_str("minus sign")]), __data17()])); __args.push(py_func("_dash_or_hyphen", |args: &[Py]| _dash_or_hyphen(args))); __kws.extend(vec![("guard".to_string(), py_func("_dash_is_the_mark", |args: &[Py]| _dash_is_the_mark(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cancel-plans-politely")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cancel"), py_str("cancelling"), py_str("canceling"), py_str("pull out")]), py_tuple(vec![py_str("dinner"), py_str("plans"), py_str("invite"), py_str("invitation"), py_str("polite"), py_str("rude"), py_str("text"), py_str("message")])])); __args.push(py_str("Say it early, give the reason briefly, and offer the next time: \"So sorry, I need to cancel Thursday — work has landed on me. Can we do the week after? I'll book it.\" One short apology and a new date beats a long excuse.")); __kws.extend(vec![("avoid".to_string(), __data18())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("decline-invitation"), py_tuple(vec![py_tuple(vec![py_str("decline"), py_str("declining"), py_str("turn down"), py_str("say no")]), py_tuple(vec![py_str("invitation"), py_str("invite"), py_str("party"), py_str("wedding"), py_str("offer")])]), py_str("Thank them, say no plainly, and don't over-explain: \"Thanks so much for thinking of me, but I can't make it. I hope it goes brilliantly.\" If you'd like to keep the door open, add one line about catching up another time.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rsvp-invitation")); __args.push(py_tuple(vec![__data19(), __data20()])); __args.push(py_str("Reply by the date on the card, and answer both things they asked in one line: yes or no, and the meal or dietary bit. \"Thank you so much - we'd love to come. Two of us, one vegetarian, no allergies.\" If the invitation names you alone, go alone: asking to bring someone is the one thing that puts a host in an awkward spot. A no is fine and better early: \"Thank you for thinking of us, but we can't make it - I hope it's a wonderful day.\"")); __kws.extend(vec![("avoid".to_string(), __data21()), ("ask".to_string(), __data22())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word-an-invitation")); __args.push(py_tuple(vec![py_tuple(vec![py_str("invitation"), py_str("invitations"), py_str("invite"), py_str("invites"), py_str("wedding invitation"), py_str("wedding invite"), py_str("save the date")]), __data23()])); __args.push(py_str("Both names in full, the date and time written out, and the place with a postcode: then the one thing the guest has to act on - whether there is a meal to choose, whether children are invited - and an RSVP date with a name and a number to reply to. One side of a card is plenty. \"Alex and Sam would love you to join them\" does more work than three lines of poetry, and \"no gifts please\" only goes on if you mean it.")); __kws.extend(vec![("avoid".to_string(), __data24()), ("ask".to_string(), py_tuple(vec![py_str("word"), py_str("wording"), py_str("reword"), py_str("write"), py_str("writing"), py_str("draft"), py_str("drafting"), py_str("design"), py_str("what to put"), py_str("what should i put")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plus-one")); __args.push(py_tuple(vec![__data25(), py_tuple(vec![py_str("wedding"), py_str("invitation"), py_str("invite"), py_str("invitations"), py_str("rsvp"), py_str("should i"), py_str("can i"), py_str("allowed"), py_str("ask"), py_str("asks"), py_str("said")])])); __args.push(py_str("Read the envelope: if it names only you, or says one seat, it is just you - a plus one is only a plus one when the invitation says so or the host names your partner. Asking straight is fine and better early: \"Is there room for a plus one?\" If the answer is yes, remember the meal and the seat were counted for two from then on.")); __kws.extend(vec![("avoid".to_string(), __data26()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("when-to-send-invitations")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wedding invitation"), py_str("wedding invitations"), py_str("wedding invite"), py_str("wedding invites"), py_str("invitation"), py_str("invitations"), py_str("invites"), py_str("save the date"), py_str("save the dates")]), __data27()])); __args.push(py_str("Six to eight weeks before the day is the usual rule here, and three months if people have to travel or it is a bank holiday weekend, because the invitation is asking for a yes, a meal choice and sometimes a bed. Post save-the-dates six to twelve months out, put the RSVP date three to four weeks before the wedding, and give the venue the final numbers about two weeks ahead. Chasing the people who have not answered is easier if the card names one date and one name to reply to.")); __kws.extend(vec![("avoid".to_string(), __data28()), ("ask".to_string(), py_tuple(vec![py_str("when"), py_str("how early"), py_str("how far"), py_str("how long before"), py_str("how many weeks"), py_str("how many months"), py_str("weeks before"), py_str("months before"), py_str("in advance"), py_str("ahead"), py_str("too early"), py_str("too late")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("deadline-extension")); __args.push(py_tuple(vec![g__EXT_WORDS(), op_add(&op_add(&py_tuple(vec![py_str("email"), py_str("ask"), py_str("asking"), py_str("lazy"), py_str("boss"), py_str("professor")]), &g__EXT_SUBJECTS()), &py_tuple(vec![py_str("get"), py_str("need"), py_str("needs"), py_str("how"), py_str("can i"), py_str("could i"), py_str("what do i"), py_str("should i"), py_str("said no"), py_str("refused"), py_str("missed"), py_str("help")]))])); __args.push(py_func("_a_deadline_extension", |args: &[Py]| _a_deadline_extension(args))); __kws.extend(vec![("avoid".to_string(), __data29()), ("guard".to_string(), py_func("_late_names_the_work", |args: &[Py]| _late_names_the_work(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("condolence-message"), py_tuple(vec![py_tuple(vec![py_str("condolence"), py_str("condolences"), py_str("sympathy"), py_str("sorry for your loss"), py_str("bereaved"), py_str("what do i say to someone who lost")]), py_tuple(vec![py_str("card"), py_str("write"), py_str("say"), py_str("message"), py_str("note")])]), py_str("Name the person, say you're sorry, and mention one thing you remember or liked about them: \"I'm so sorry about your mum. She made the whole street laugh and I'll miss her. Thinking of you all.\" Keep it short, avoid 'at least' and any advice, and write it by hand if you can.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ring-or-write")); __args.push(py_tuple(vec![g__RING_OR_WRITE(), py_tuple(vec![py_str("should i"), py_str("shall i"), py_str("do i"), py_str("is it better"), py_str("better to"), py_str("would it be better"), py_str("worth"), py_str("rather"), py_str("which is better"), py_str("whats better"), py_str("what is better")]), __data30()])); __args.push(py_str("Ring if you want it sorted this week, and write if you need a record. A call gets an answer the same day and you can ask a second question; a letter or an email is what you point to later if nothing happens. The easiest is both: ring, then send two lines saying what was agreed and by when, and keep a copy.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![g__RING_OR_WRITE()])), ("avoid".to_string(), __data31())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-letter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("letter"), py_str("letters"), py_str("in writing"), py_str("put it in writing"), py_str("write to"), py_str("writing to"), py_str("draft a letter"), py_str("compose a letter"), py_str("send a letter")]), __data32()])); __args.push(py_str("Put what you want in the first line, then the facts in date order, then a deadline. \"I'm writing about the damp in the back bedroom at 14 Elm Road, which I reported on 3 March. Could you send someone to look at it by 20 March?\" Name the address or the reference, keep it to one page, and say what you'll do next if nothing happens. Keep a copy, and send it by email or recorded post so you can show when it went. Ringing as well often gets an answer sooner than the letter does.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what should i say"), py_str("write"), py_str("draft"), py_str("help"), py_str("can i"), py_str("should i")])), ("avoid".to_string(), __data33())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("formal-letter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("formal letter"), py_str("business letter"), py_str("official letter"), py_str("letter"), py_str("letters"), py_str("letter to my boss"), py_str("letter to the boss"), py_str("letter to my manager"), py_str("letter to my employer"), py_str("letter to my company"), py_str("letter to the head")]), __data34()])); __args.push(py_str("Four short paragraphs: what you want and by when, the facts in date order, what you have already tried, and what happens next if nothing changes. Put your address, their name and address and the date at the top with a subject line saying what it is about, open with 'Dear Mr Smith' or 'Dear Sir or Madam' and close with 'Yours sincerely' or 'Yours faithfully', and keep the whole thing to one page.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("write"), py_str("draft"), py_str("help"), py_str("need"), py_str("want"), py_str("should"), py_str("can"), py_str("start")])), ("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data35())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("complaint-letter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("complaint"), py_str("complain")]), py_tuple(vec![py_str("letter"), py_str("email"), py_str("write"), py_str("company"), py_str("refund")])])); __args.push(py_str("Keep it to facts and what you want: what you bought, when, what went wrong, what you've already tried, and the outcome you're after, with dates and order numbers. Say what you want plainly (\"I'd like a refund of £40\") and give a deadline for a reply. Stay polite — the person reading it isn't the one who caused it.")); __kws.extend(vec![("avoid".to_string(), __data36())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("love-letter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("love letter"), py_str("love note"), py_str("valentine"), py_str("valentine's"), py_str("valentines"), py_str("secret admirer")]), py_tuple(vec![py_str("write"), py_str("writing"), py_str("written"), py_str("say"), py_str("saying"), py_str("message"), py_str("note"), py_str("card"), py_str("send"), py_str("draft"), py_str("put in"), py_str("words")])])); __args.push(py_str("Say the one thing you would say out loud if they were in front of you, then the specific thing you noticed: \"I still think about the way you laughed at the wrong bit of that film.\" Three or four sentences is enough, no jokes at your own expense and nothing borrowed from a card. Write it by hand if you can, read it once and take out anything you would not say to their face.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what should i"), py_str("write"), py_str("draft"), py_str("help"), py_str("can i"), py_str("should i")])), ("avoid".to_string(), __data37())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("apology-fits-the-person"), py_tuple(vec![py_tuple(vec![py_str("apologise"), py_str("apologize"), py_str("apology"), py_str("apologising"), py_str("apologizing"), py_str("say sorry"), py_str("said sorry"), py_str("apologise properly")]), __data38()]), py_func("_apology", |args: &[Py]| _apology(args))]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chase-payment")); __args.push(py_tuple(vec![py_tuple(vec![py_str("invoice"), py_str("payment"), py_str("paid"), py_str("owed")]), py_tuple(vec![py_str("chase"), py_str("chasing"), py_str("overdue"), py_str("unpaid"), py_str("reminder"), py_str("late")])])); __args.push(py_str("Send one friendly reminder with the details: invoice number, amount, due date, and a link or the bank details again. \"Just a nudge on invoice 214 (£340), due on the 3rd — could you confirm when it's scheduled?\" If it's already overdue, add a date you'll escalate on, and keep it in writing.")); __kws.extend(vec![("avoid".to_string(), __data39())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("emojis-at-work"), py_tuple(vec![py_tuple(vec![py_str("emoji"), py_str("emojis"), py_str("emoticon"), py_str("emoticons")]), py_tuple(vec![py_str("work"), py_str("email"), py_str("professional"), py_str("boss"), py_str("client"), py_str("colleague"), py_str("rude"), py_str("slack"), py_str("teams"), py_str("office"), py_str("meeting")])]), py_str("One emoji, only where the tone is already warm: a thumbs-up or a smile in a work chat is fine, a string of them rarely is. Skip faces in anything to a client, in bad news, and in a first email to someone senior; a plain \"Thanks!\" or a short \"Sounds good\" carries the warmth without the risk. Never let an emoji replace the words that say what you mean, and never use a winking one to soften a real complaint.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clear-writing")); __args.push(py_tuple(vec![py_tuple(vec![py_str("writing"), py_str("write")]), py_tuple(vec![py_str("clear"), py_str("clearer"), py_str("clearly"), py_str("concise"), py_str("concisely"), py_str("shorter"), py_str("tighter"), py_str("plain"), py_str("better"), py_str("professional"), py_str("formal"), py_str("polished")])])); __args.push(py_str("Shorten sentences, cut the warm-up, and put the ask first: \"Could you send the file by Friday?\" beats three lines of context before the point. Use the same word for the same thing throughout, and read it aloud — anything you stumble on, rewrite.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("cv"), py_str("resume"), py_str("résumé")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("writing-improve")); __args.push(py_tuple(vec![py_tuple(vec![py_str("writing"), py_str("write"), py_str("writer"), py_str("essays"), py_str("essay"), py_str("english")]), py_tuple(vec![py_str("improve"), py_str("improving"), py_str("improvement"), py_str("better at"), py_str("get better"), py_str("become better"), py_str("become a better"), py_str("practice"), py_str("practise"), py_str("practising"), py_str("practicing")])])); __args.push(py_str("Write 200 words a day and edit them the next morning, when you can see what is on the page rather than what you meant: cut one sentence and rewrite one. Copy a paragraph you admire by hand once a week to feel how the sentences are built, and read your own work aloud - the place you stumble is the place to fix.")); __kws.extend(vec![("avoid".to_string(), __data40())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("email-subject"), py_tuple(vec![py_tuple(vec![py_str("subject line"), py_str("subject lines"), py_str("email subject"), py_str("subject for an email")]), py_tuple(vec![py_str("write"), py_str("good"), py_str("better"), py_str("opening"), py_str("start"), py_str("email")])]), py_str("Say what it's about and what you need in six words or so: 'Invoice 214 — payment date?' or 'Flat 3 viewing Saturday 10am'. Put the action at the front, and don't write 'Quick question' — it tells the reader nothing.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("proofread"), py_tuple(vec![py_tuple(vec![py_str("proofread"), py_str("proofreading"), py_str("proof read"), py_str("check my writing"), py_str("check my email"), py_str("check an email")]), py_tuple(vec![py_str("how"), py_str("tips"), py_str("better"), py_str("faster"), py_str("mistakes")])]), py_str("Read it once for what it says and once only for mistakes, and change the order - bottom to top, or aloud - so your eye stops filling in what you meant. Check names, dates and numbers against the source, then the four usual slips: its/it's, your/you're, then/than, and missing words.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thank-you-note")); __args.push(py_tuple(vec![py_tuple(vec![py_str("thank you note"), py_str("thank you letter"), py_str("thank you card"), py_str("thankyou note"), py_str("thank-you note")]), py_tuple(vec![py_str("write"), py_str("after"), py_str("say"), py_str("gift"), py_str("help")])])); __args.push(g_thanks_note()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("interview"), py_str("interviews"), py_str("interviewer"), py_str("interviewing"), py_str("job interview"), py_str("panel"), py_str("recruiter"), py_str("job")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("give-feedback")); __args.push(py_tuple(vec![py_tuple(vec![py_str("feedback")]), py_tuple(vec![py_str("how"), py_str("give"), py_str("giving"), py_str("criticism"), py_str("criticise"), py_str("difficult"), py_str("badly")])])); __args.push(py_str("Name the thing, say the effect, then ask: 'Your reports land late on Fridays, so I'm guessing the plan slips - what's getting in the way?' Keep it about the work and a particular time, not their character, and give the good news just as specifically: 'the client email was exactly right.'")); __kws.extend(vec![("avoid".to_string(), __data41())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("disagree-kindly"), py_tuple(vec![py_tuple(vec![py_str("disagree"), py_str("disagreement"), py_str("disagreeing")]), py_tuple(vec![py_str("how"), py_str("without"), py_str("politely"), py_str("argument"), py_str("rude"), py_str("tactfully")])]), py_str("Say what you agree with first, then the one thing you'd change: 'I'm with you on the timeline - what I'd worry about is the testing.' Use 'I think' and 'from what I've seen', keep it to one point at a time, and if it heats up, offer to take it offline.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bad-news-email")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bad news"), py_str("turn down a candidate"), py_str("reject an applicant"), py_str("reject a candidate"), py_str("rejection"), py_str("rejected"), py_str("unsuccessful"), py_str("decline a request")]), py_tuple(vec![py_str("email"), py_str("emails"), py_str("write"), py_str("writing"), py_str("word"), py_str("wording"), py_str("letter"), py_str("letters")])])); __args.push(py_str("Give it in the first line, without a warm-up: 'I'm sorry to say we won't be going ahead with your application.' Then one plain sentence on why, if it's fair to say, and what happens next. No 'however' or 'unfortunately' to soften it - the person reads them as stalling.")); __kws.extend(vec![("avoid".to_string(), __data42())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("congratulations-message"), py_tuple(vec![py_tuple(vec![py_str("congratulations"), py_str("congrats"), py_str("well done card"), py_str("new baby message"), py_str("graduation message")]), py_tuple(vec![py_str("write"), py_str("message"), py_str("say"), py_str("card"), py_str("text")])]), py_str("Name what they did and what you noticed: 'The grand opening looked brilliant - I've watched you plan it for a year. Congratulations.' Three lines is plenty, and add something only you could say about them.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("metaphor-simile"), py_tuple(vec![py_tuple(vec![py_str("metaphor"), py_str("simile"), py_str("metaphors"), py_str("similes")]), py_tuple(vec![py_str("difference"), py_str("mean"), py_str("means"), py_str("example"), py_str("examples"), py_str("what"), py_str("whats"), py_str("what is"), py_str("definition"), py_str("define"), py_str("explain"), py_str("describe")])]), py_func("_the_figure_that_was_named", |args: &[Py]| _the_figure_that_was_named(args))]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("figure-of-speech"), py_tuple(vec![py_tuple(vec![py_str("irony"), py_str("ironic"), py_str("onomatopoeia"), py_str("alliteration"), py_str("personification"), py_str("hyperbole"), py_str("euphemism"), py_str("figure of speech"), py_str("figures of speech")]), py_tuple(vec![py_str("mean"), py_str("means"), py_str("what"), py_str("define"), py_str("definition"), py_str("example"), py_str("examples"), py_str("difference"), py_str("explain")])]), py_func("_figure_of_speech", |args: &[Py]| _figure_of_speech(args))]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word-meaning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mean"), py_str("means"), py_str("meaning"), py_str("meanings"), py_str("define"), py_str("definition"), py_str("explain"), py_str("what does"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("called")]), call_builtin("tuple", &[g_WORD_GLOSSES()]).unwrap_or_else(|e| exc::raise(e))])); __args.push(py_func("_word_meaning", |args: &[Py]| _word_meaning(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("mean"), py_str("means"), py_str("meaning"), py_str("define"), py_str("definition"), py_str("explain"), py_str("what does"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("called")])), ("avoid".to_string(), __data43())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("everyday-word-meaning")); __args.push(py_tuple(vec![call_builtin("tuple", &[g_EVERYDAY_WORDS()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("tuple", &[g_EVERYDAY_WORDS()]).unwrap_or_else(|e| exc::raise(e)), __data44()])); __args.push(py_func("_an_everyday_word", |args: &[Py]| _an_everyday_word(args))); __kws.extend(vec![("guard".to_string(), py_func("_asks_what_a_word_means", |args: &[Py]| _asks_what_a_word_means(args))), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data45())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("active-passive"), py_tuple(vec![py_tuple(vec![py_str("active"), py_str("passive")]), py_tuple(vec![py_str("voice"), py_str("difference"), py_str("writing"), py_str("use")])]), py_str("In the active voice the doer comes first: the dog ate my homework. In the passive the thing done to comes first: my homework was eaten. Active is usually shorter and clearer; passive earns its place when the doer doesn't matter or you'd rather not name them.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("colon-use"), py_tuple(vec![py_tuple(vec![py_str("colon"), py_str("colons")]), __data46()]), py_str("A colon introduces what explains or follows: a list, an example, a quote. \"Take three things: milk, eggs and bread.\" No capital after it unless a full sentence follows, and no colon straight after a verb - \"the things I need are milk, eggs and bread\", not \"are: milk\".")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("comma-before-but"), py_tuple(vec![py_tuple(vec![py_str("comma"), py_str("commas")]), py_tuple(vec![py_str("but"), py_str("however"), py_str("before but")])]), py_str("Put a comma before but when it joins two full sentences: \"I wanted to come, but my train was cancelled.\" When it only pairs two words or short phrases - small but strong - leave the comma out.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("start-with-and")); __args.push(py_tuple(vec![py_tuple(vec![py_str("start"), py_str("starting"), py_str("begin"), py_str("beginning")]), py_tuple(vec![py_str("sentence"), py_str("sentences")])])); __args.push(py_str("Yes, a sentence can start with and or but - writers have done it for centuries and it reads naturally in ordinary writing. What gets noticed is doing it over and over (\"And then... And then...\"), so use it for effect rather than as a habit.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("ok"), py_str("okay"), py_str("fine"), py_str("alright"), py_str("allowed"), py_str("correct"), py_str("wrong"), py_str("can i"), py_str("can you"), py_str("should i"), py_str("may i"), py_str("is it")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("apostrophe-contractions")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dont"), py_str("dont"), py_str("cant"), py_str("wont"), py_str("doesnt"), py_str("isnt"), py_str("couldnt"), py_str("contraction"), py_str("contractions")]), py_tuple(vec![py_str("apostrophe"), py_str("apostrophes"), py_str("missing"), py_str("letters")])])); __args.push(py_str("The apostrophe goes where the letters were left out: don't for do not, can't for cannot, I'm for I am, won't for will not. That is why won't and can't don't look like the words they shorten - the old forms were woll not and can not.")); __kws.extend(vec![("ask".to_string(), __data47())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("am-pm"), py_tuple(vec![py_tuple(vec![py_str("12am"), py_str("12pm"), py_str("12 am"), py_str("12 pm"), py_str("noon"), py_str("midnight"), py_str("am or pm"), py_str("am and pm"), py_str("am pm")]), py_tuple(vec![py_str("mean"), py_str("correct"), py_str("is it"), py_str("which"), py_str("difference"), py_str("technically"), py_str("write"), py_str("written"), py_str("how")])]), py_str("12 am is midnight and 12 pm is noon: am runs from midnight up to just before noon, so the first minute of the day is 12:00 am. Most people write 12 noon and 12 midnight instead, which nobody argues with.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("alright-all-right")); __args.push(py_tuple(vec![py_tuple(vec![py_str("alright")]), py_tuple(vec![py_str("all right"), py_str("correct"), py_str("spell"), py_str("ok"), py_str("okay"), py_str("either"), py_str("wrong")])])); __args.push(py_str("All right is the form to write: two words, as in \"are you all right?\" Alright is fine in a text or a line of dialogue and most people read it without noticing, so keep it for those.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("which"), py_str("correct"), py_str("spell"), py_str("either"), py_str("wrong"), py_str("or")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("titles-italics")); __args.push(py_tuple(vec![py_tuple(vec![py_str("italics"), py_str("italic"), py_str("underline"), py_str("quotation marks"), py_str("quote marks"), py_str("book title"), py_str("film title"), py_str("song title"), py_str("titles")]), py_tuple(vec![py_str("title"), py_str("titles"), py_str("book"), py_str("film"), py_str("song"), py_str("poem"), py_str("album"), py_str("chapter"), py_str("article"), py_str("essay")])])); __args.push(py_str("Italicise whole works - book titles, films, albums, newspapers - and put quotation marks around the parts of a whole: a chapter, an article, a song, a poem. Underline instead of italicising if you are writing by hand.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("should i"), py_str("do i"), py_str("how do i"), py_str("which"), py_str("italicise"), py_str("italicize"), py_str("use")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("numbers-in-writing")); __args.push(py_tuple(vec![py_tuple(vec![py_str("numbers"), py_str("number"), py_str("five"), py_str("ten"), py_str("figures")]), py_tuple(vec![py_str("essay"), py_str("writing"), py_str("write"), py_str("style"), py_str("spell"), py_str("words")])])); __args.push(py_str("Write one to nine as words and 10 and up as figures, except when a number starts a sentence - then spell it out: \"Twenty people came.\" Keep it consistent within a sentence, and use figures for money, times and measurements.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("should i"), py_str("do i"), py_str("when"), py_str("is it"), py_str("write")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("seasons-capitals")); __args.push(py_tuple(vec![py_tuple(vec![py_str("seasons"), py_str("season"), py_str("winter"), py_str("summer"), py_str("autumn"), py_str("spring")]), py_tuple(vec![py_str("capital"), py_str("capitalise"), py_str("capitalize"), py_str("capitals"), py_str("capitalised")])])); __args.push(py_str("Seasons stay lower case: spring, summer, autumn, winter. Days and months take a capital - Monday, January - and so do holidays, as in Christmas.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("do i"), py_str("should i"), py_str("is it"), py_str("does"), py_str("are"), py_str("capitalise"), py_str("capitalize"), py_str("capitalised"), py_str("capitalized")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("contractions-essay")); __args.push(py_tuple(vec![py_tuple(vec![py_str("contractions"), py_str("contraction")]), py_tuple(vec![py_str("essay"), py_str("writing"), py_str("formal"), py_str("academic"), py_str("ok"), py_str("allowed"), py_str("school")])])); __args.push(py_str("Contractions are fine in ordinary writing, emails and anything with a human voice, and they read as friendlier. In formal academic writing most style guides still prefer the full form - write \"do not\" rather than \"don't\" and you cannot be marked down for it.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("are they"), py_str("can i"), py_str("should i"), py_str("ok"), py_str("allowed")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("rhetorical-question"), py_tuple(vec![py_tuple(vec![py_str("rhetorical")]), py_tuple(vec![py_str("question"), py_str("mean"), py_str("what"), py_str("example"), py_str("definition")])]), py_str("A rhetorical question is asked for effect, not for an answer: \"Who could blame her?\" It nudges the reader towards the answer you already have, which is why it works well as an opener and badly as a real enquiry.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("oxymoron"), py_tuple(vec![py_tuple(vec![py_str("oxymoron"), py_str("oxymorons")]), py_tuple(vec![py_str("mean"), py_str("what"), py_str("example"), py_str("examples"), py_str("definition")])]), py_str("An oxymoron puts two words together that fight: deafening silence, living dead, bittersweet, a small crowd. It is a deliberate contradiction, and it is different from a plain contradiction because the clash is the point.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("synonym"), py_tuple(vec![py_tuple(vec![py_str("synonym"), py_str("synonyms")]), py_tuple(vec![py_str("mean"), py_str("what"), py_str("word"), py_str("for"), py_str("another word")])]), py_str("A synonym is a word that means roughly the same as another: happy and glad, begin and start. Roughly is the important bit - glad fits \"I'm glad you came\" where happy would be too strong, so pick by the shade you want.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("spaces-after-full-stop"), py_tuple(vec![py_tuple(vec![py_str("space"), py_str("spaces"), py_str("spacing")]), py_tuple(vec![py_str("full stop"), py_str("period"), py_str("sentence"), py_str("sentences"), py_str("after")])]), py_str("One space after a full stop, not two - the double space was a typewriter habit and word processors already add the right gap. The same goes after a comma and a question mark.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("formal-date")); __args.push(py_tuple(vec![py_tuple(vec![py_str("date"), py_str("dates"), py_str("the date"), py_str("date in a letter"), py_str("date on a letter"), py_str("what date")]), py_tuple(vec![py_str("write"), py_str("writing"), py_str("written"), py_str("formally"), py_str("letter"), py_str("letter to"), py_str("form"), py_str("forms"), py_str("correct"), py_str("diary"), py_str("put")])])); __args.push(py_str("Write the date in full and in one order: 10 March 2025 (British) or March 10, 2025 (American), with no th and no comma in the British form. In a letter, put it above the greeting; the ordinal - the tenth of March - is for speech.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how do i"), py_str("how do you"), py_str("how to"), py_str("should i"), py_str("which"), py_str("what is"), py_str("correct")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("dating app"), py_str("dating apps"), py_str("dating profile"), py_str("dating site"), py_str("online dating"), py_str("first date"), py_str("on a date"), py_str("ask her out"), py_str("ask him out"), py_str("tinder"), py_str("hinge"), py_str("bumble")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("homophone"), py_tuple(vec![py_tuple(vec![py_str("homophone"), py_str("homophones")]), py_tuple(vec![py_str("mean"), py_str("what"), py_str("definition"), py_str("example"), py_str("examples")])]), py_str("Homophones sound the same but are spelt differently and mean different things: their/there/they're, to/too/two, right/write, sea/see. If swapping a word changes the sentence's meaning, you have a homophone pair - that is what the other half of these questions is about.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("grey-gray")); __args.push(py_tuple(vec![py_tuple(vec![py_str("grey"), py_str("gray")]), py_tuple(vec![py_str("correct"), py_str("spell"), py_str("spelling"), py_str("wrong"), py_str("either"), py_str("difference"), py_str("or")])])); __args.push(py_str("Both are right: grey is the British spelling and gray the American one, and neither is a mistake. Pick the one your reader expects and keep it the same through a piece of writing.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is it"), py_str("which"), py_str("correct"), py_str("spell"), py_str("either"), py_str("wrong")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("comma-before-and"), py_tuple(vec![py_tuple(vec![py_str("comma before and"), py_str("oxford comma"), py_str("serial comma")]), py_tuple(vec![py_str("need"), py_str("should"), py_str("use"), py_str("when"), py_str("yes")])]), py_str("Not needed for a simple list: bread, milk and eggs. Put one in when 'and' joins two complete sentences (I booked the table, and she drove), or when the list items are long enough that the reader needs the pause: we need the report signed, the photos checked, and the invoice sent.")]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("oxford-comma"), py_tuple(vec![py_tuple(vec![py_str("oxford comma"), py_str("serial comma")]), py_tuple(vec![py_str("what"), py_str("whats"), py_str("what is"), py_str("mean"), py_str("definition"), py_str("explain"), py_str("use"), py_str("why"), py_str("rule"), py_str("difference"), py_str("called"), py_str("comma")])]), py_str("It is the comma before the last item in a list: milk, eggs, and bread. British writing usually leaves it out and American keeps it, and it earns its place when it stops a misreading - I'd like to thank my parents, Ayn Rand, and God. Keep to one habit through a document either way.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("comma-rules")); __args.push(py_tuple(vec![py_tuple(vec![py_str("comma"), py_str("commas")]), __data48(), py_tuple(vec![py_str("however")])])); __args.push(py_func("_comma_rules", |args: &[Py]| _comma_rules(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("splice"), py_str("splices"), py_str("run on"), py_str("run-on"), py_str("two sentences")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("sentence-end"), py_tuple(vec![py_tuple(vec![py_str("full stop"), py_str("full-stop"), py_str("question mark"), py_str("question marks"), py_str("exclamation mark"), py_str("exclamation marks")]), __data49()]), py_func("_end_mark", |args: &[Py]| _end_mark(args))]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("speak-clearly")); __args.push(py_tuple(vec![__data50(), __data51()])); __args.push(py_func("_speak_clearly", |args: &[Py]| _speak_clearly(args))); __kws.extend(vec![("avoid".to_string(), __data52()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("plural-octopus"), py_tuple(vec![py_tuple(vec![py_str("plural"), py_str("plurals")]), py_tuple(vec![py_str("octopus"), py_str("octopi"), py_str("virus"), py_str("cactus"), py_str("walrus")])]), g__PLUSES_REPLY()]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("which-plural-of-a-us-word")); __args.push(py_tuple(vec![g__PLUS_FORMS()])); __args.push(g__PLUSES_REPLY()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("shape".to_string(), py_tuple(vec![py_func("_offers_two_of_the_family", |args: &[Py]| _offers_two_of_the_family(args))])), ("guard".to_string(), py_func("_asks_which_of_the_two", |args: &[Py]| _asks_which_of_the_two(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leaving-group-chat")); __args.push(py_tuple(vec![py_tuple(vec![py_str("group chat"), py_str("group chats"), py_str("group message"), py_str("whatsapp group"), py_str("family group"), py_str("group text"), py_str("facebook group")]), __data53()])); __args.push(py_str("Mute it first - leaving is for the chat that is genuinely noise, not for one busy week. If you do go, the quiet exit is the polite one: turn off notifications, leave, and tell anyone who matters in a private message. Only a family or work group needs a goodbye line, and one sentence does it: 'I'm going to hop out of this one, message me directly.'")); __kws.extend(vec![("avoid".to_string(), __data54())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word-start-email")); __args.push(py_tuple(vec![py_tuple(vec![py_str("email"), py_str("message"), py_str("letter"), py_str("dear or hi"), py_str("hi or dear"), py_str("dear or hello")]), __data55()])); __args.push(py_str("Use the name: 'Dear Ms Okafor' if you know it and it is formal, 'Hello Sam' for anything else. Then say why you are writing in the first sentence, before any introductions. 'I hope this email finds you well' is filler that most readers skip.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("cover letter"), py_str("cv"), py_str("resume"), py_str("applying for"), py_str("application"), py_str("resignation"), py_str("resigning"), py_str("leaving my job"), py_str("job offer"), py_str("interview")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("letter-salutation")); __args.push(py_tuple(vec![__data56(), __data57()])); __args.push(py_str("If you know the name, use it: 'Dear Ms Okafor' or 'Dear Dr Patel' beats any general form. 'Dear Sir or Madam' is fine when you genuinely have no name, and it pairs with 'Yours faithfully'; with a name it is 'Yours sincerely'. Never 'To whom it may concern' - it reads like a circular - and check the spelling of the name, which is the thing people notice most.")); __kws.extend(vec![("avoid".to_string(), __data58())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("envelope-addressing"), py_tuple(vec![py_tuple(vec![py_str("address an envelope"), py_str("addressing an envelope"), py_str("address the envelope"), py_str("what goes on an envelope"), py_str("write an address on an envelope"), py_str("envelope")]), py_tuple(vec![py_str("address"), py_str("addressing"), py_str("write"), py_str("writing"), py_str("post"), py_str("postcode"), py_str("name"), py_str("names"), py_str("how"), py_str("where"), py_str("correctly")])]), py_str("Name on the top line, house number and street below it, then the town, then the postcode in capitals on its own line: Ms J Patel, 14 Elm Road, Leeds, LS6 2AB. Put your own name and address on the back, and use the full name and title - Mr, Mrs, Ms or Dr - of whoever will open it.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("paragraph-length")); __args.push(py_tuple(vec![py_tuple(vec![py_str("paragraph"), py_str("paragraphs")]), __data59()])); __args.push(py_func("_paragraph", |args: &[Py]| _paragraph(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("essay"), py_str("word count"), py_str("words should"), py_str("introduction"), py_str("conclusion")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-poem")); __args.push(py_tuple(vec![py_tuple(vec![py_str("poem"), py_str("poems"), py_str("poetry"), py_str("verse")]), py_tuple(vec![py_str("write"), py_str("writing"), py_str("start"), py_str("begin"), py_str("good"), py_str("better"), py_str("how"), py_str("short")])])); __args.push(py_str("Start with one true picture rather than a feeling - the chipped mug, the bus you missed - and write it in short plain lines you would actually say out loud. Keep the best line, cut the explaining around it, and stop one line before you want to. Ten minutes with no rhyming is a good first poem.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("love poem"), py_str("valentine"), py_str("wedding"), py_str("funeral"), py_str("eulogy"), py_str("read"), py_str("reading"), py_str("recommend"), py_str("book"), py_str("books"), py_str("poet"), py_str("poets")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-dialogue")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dialogue"), py_str("dialog")]), py_tuple(vec![py_str("write"), py_str("writing"), py_str("good"), py_str("better"), py_str("how"), py_str("natural"), py_str("realistic"), py_str("story"), py_str("conversation"), py_str("start")])])); __args.push(py_str("Keep each speech short and let people interrupt, dodge and answer a different question than the one asked. Use 'said' and nothing fancier, cut the adverbs, and put a small action between speeches so the reader can see who is talking. Read it aloud in two voices: anything that sounds written, cut.")); __kws.extend(vec![("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("use-a-thesaurus"), py_tuple(vec![py_tuple(vec![py_str("thesaurus")]), py_tuple(vec![py_str("use"), py_str("how"), py_str("best"), py_str("good"), py_str("find"), py_str("word"), py_str("words"), py_str("another")])]), py_str("Look up the plainest word in the sentence, not the cleverest: that is the one with a useful choice. Take a word you would say out loud, check it in the dictionary for the sense you mean, and leave the rest alone - a thesaurus makes writing worse when it is used to sound clever.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("use-a-dictionary")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dictionary"), py_str("dictionaries")]), py_tuple(vec![py_str("use"), py_str("how"), py_str("best"), py_str("good"), py_str("properly"), py_str("word"), py_str("words"), py_str("look up"), py_str("looking up")])])); __args.push(py_str("Look up the base word rather than the ending - 'running' is under 'run' - and read the example sentence before the definition, because the example shows the word at work. Then write one sentence of your own with it the same day: a word you only read is one you lose by Friday.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("thesaurus"), py_str("synonym"), py_str("synonyms"), py_str("different word")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("repeating-yourself")); __args.push(py_tuple(vec![py_tuple(vec![py_str("repeat"), py_str("repeating"), py_str("repetitive"), py_str("repetition"), py_str("repeats")]), py_tuple(vec![py_str("writing"), py_str("write"), py_str("essay"), py_str("myself"), py_str("sentences"), py_str("words"), py_str("paragraph"), py_str("paragraphs"), py_str("story")])])); __args.push(py_str("Most repetition is the same point made twice in different words: keep the shorter one and delete the other. Vary how sentences start - if three in a row begin with 'the' or 'I', rewrite two of them - and search the document for your favourite filler ('really', 'just', 'important') at the end, because you will not hear it yourself.")); __kws.extend(vec![("avoid".to_string(), __data60())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word-sign-off")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sign off"), py_str("signing off"), py_str("signoff"), py_str("sign-off"), py_str("sign"), py_str("signed"), py_str("signing"), py_str("signature")]), __data61()])); __args.push(py_str("'Kind regards' or 'Best wishes' are safe for almost anything work-related, and 'Thanks' works when you have asked for something. 'Yours sincerely' only when you started with Dear Mr or Ms, and 'Yours faithfully' when you started Dear Sir or Madam.")); __kws.extend(vec![("avoid".to_string(), __data62()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word-close-a-letter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("close"), py_str("closing"), py_str("closed"), py_str("end"), py_str("ending"), py_str("finish"), py_str("finishing"), py_str("at the end"), py_str("the end of"), py_str("last line"), py_str("final line"), py_str("wrapping up")]), py_tuple(vec![py_str("letter"), py_str("letters"), py_str("email"), py_str("emails"), py_str("memo"), py_str("memos"), py_str("message"), py_str("messages"), py_str("formal")])])); __args.push(py_str("'Kind regards' or 'Best wishes' are safe for almost anything work-related, and 'Thanks' works when you have asked for something. 'Yours sincerely' only when you started with Dear Mr or Ms, and 'Yours faithfully' when you started Dear Sir or Madam.")); __kws.extend(vec![("avoid".to_string(), __data63()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.ENTRIES"); }
    s_ENTRIES(__data9());
    fn __data64() -> Py { py_dict(vec![(py_str("happy"), py_str("Glad, cheerful, pleased, content, delighted. Glad is the everyday one, content means quietly satisfied, and delighted is a big thing.")), (py_str("sad"), py_str("Unhappy, down, miserable, low, gutted. Down and low are for a flat week, miserable and gutted for something that has actually happened.")), (py_str("big"), py_str("Large, huge, enormous, vast, sizeable. Large is neutral and fits writing; huge and enormous are the spoken ones, and vast is for space rather than a thing.")), (py_str("small"), py_str("Little, tiny, slight, compact, minor. Tiny is smaller than little, and minor is for importance rather than size.")), (py_str("good"), py_str("Fine, solid, strong, decent, excellent. Fine is faint praise, decent is a real compliment in speech, and excellent is for something you would single out.")), (py_str("bad"), py_str("Poor, weak, awful, dreadful, rubbish. Poor and weak are the calm written ones; awful and rubbish are how people say it out loud.")), (py_str("said"), py_str("Said, asked, replied, muttered, shouted, whispered. Said is invisible and usually the right word - save the others for the two or three places where how it was said matters.")), (py_str("nice"), py_str("Kind, friendly, thoughtful, pleasant, lovely. Kind and thoughtful are about what someone did; pleasant and lovely are about how something felt.")), (py_str("tired"), py_str("Weary, worn out, exhausted, drained, sleepy. Sleepy means ready for bed, exhausted means past it, and drained is the one after a hard week.")), (py_str("angry"), py_str("Cross, annoyed, furious, irritated, livid. Cross is the mild everyday word, annoyed and irritated are about a thing, and furious and livid are the top end.")), (py_str("scared"), py_str("Frightened, afraid, nervous, anxious, terrified. Nervous is before, frightened is now, and terrified is the extreme of either.")), (py_str("beautiful"), py_str("Lovely, stunning, gorgeous, striking, handsome. Beautiful is the safe one, stunning is a reaction, and handsome works for a building as well as a face.")), (py_str("interesting"), py_str("Intriguing, absorbing, fascinating, notable, worth a look. Absorbing means it held your attention, fascinating is stronger, and notable is the neutral written one.")), (py_str("important"), py_str("Key, vital, crucial, significant, essential. Significant is the calm report word; crucial and essential mean it cannot be left out.")), (py_str("difficult"), py_str("Hard, tricky, demanding, awkward, challenging. Tricky is a puzzle, demanding is effort, and awkward is a person or a corner rather than a task.")), (py_str("easy"), py_str("Simple, straightforward, effortless, quick, painless. Simple is about the steps, straightforward is about the instructions, and effortless is how it felt.")), (py_str("funny"), py_str("Amusing, comical, hilarious, witty, odd. Witty is clever, hilarious is loud, and odd is the other meaning of funny.")), (py_str("hungry"), py_str("Starving, peckish, famished, ravenous. Peckish is a bit, starving is an exaggeration everyone uses, and ravenous is the real thing.")), (py_str("thank you"), py_str("Thanks, many thanks, thank you so much, I really appreciate it, much appreciated. Thanks is fine in a message and the longer ones belong in a card or after a real favour.")), (py_str("sorry"), py_str("I apologise, my apologies, I'm sorry about that, excuse me. I apologise is the written one for a mistake, and sorry is warmer when you mean it.")), (py_str("no"), py_str("Not this time, I can't, I'd rather not, that doesn't work for me. I'd rather not is the kindest firm one, and adding a reason of one clause is plenty.")), (py_str("good luck"), py_str("Best of luck, all the best, fingers crossed, break a leg. Fingers crossed is the friendly one, and break a leg is only for performers.")), (py_str("congratulations"), py_str("Well done, nice one, many congratulations, thrilled for you. Well done is for effort, congratulations for the news itself.")), (py_str("delicious"), py_str("Tasty, mouthwatering, lovely, moreish, scrumptious. Tasty is the everyday one, mouthwatering is for how it looks, and moreish means you want more of it.")), (py_str("clever"), py_str("Bright, sharp, smart, inventive, quick. Bright and smart are the everyday ones, sharp is fast on the uptake, and inventive is about ideas.")), (py_str("brave"), py_str("Courageous, bold, gutsy, plucky, fearless. Courageous is the formal one, bold is about a decision, and gutsy is doing it while scared.")), (py_str("busy"), py_str("Flat out, rushed, swamped, hectic, tied up. Flat out and swamped are the spoken ones, hectic is about the day, and tied up is the polite one.")), (py_str("great"), py_str("Excellent, brilliant, superb, terrific, first-rate. Brilliant is the British spoken one, superb is the calm written one, and terrific is louder.")), (py_str("quick"), py_str("Fast, swift, speedy, brisk, prompt. Fast is about speed, brisk is a pace you set, and prompt means on time.")), (py_str("i dont know"), py_str("I'm not sure, I'm not certain, I couldn't say, I'd have to check. I'd have to check is the one that keeps you honest without sounding helpless."))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.WORD_SETS"); }
    s_WORD_SETS(__data64());
    fn __data65() -> Py { py_tuple(vec![py_str("another word"), py_str("a better word"), py_str("better word"), py_str("different word"), py_str("other word"), py_str("word for"), py_str("another way to say"), py_str("another way of saying"), py_str("nicer way to say"), py_str("politer way"), py_str("more formal way"), py_str("less formal"), py_str("instead of saying"), py_str("synonym for"), py_str("synonyms for"), py_str("word that means"), py_str("good word for")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._SYNONYM_ASK"); }
    s__SYNONYM_ASK(__data65());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._ASK_SPANS"); }
    s__ASK_SPANS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SYNONYM_ASK()) { let mut one: Py = __item.clone(); __out.push(call_builtin("tuple", &[mcall(&one.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._CONNECTORS"); }
    s__CONNECTORS(py_tuple(vec![py_str("for"), py_str("say"), py_str("saying"), py_str("means")]));
    fn __data66() -> Py { py_tuple(vec![py_str("thanks"), py_str("please"), py_str("then"), py_str("too"), py_str("as"), py_str("well"), py_str("again"), py_str("instead"), py_str("there"), py_str("here"), py_str("somebody"), py_str("someone"), py_str("people")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PADDING"); }
    s__PADDING(call_builtin("frozenset", &[__data66()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PHRASE_KEYS"); }
    s__PHRASE_KEYS(call_builtin("tuple", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WORD_SETS()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&one.clone(), &py_str(" ")))) {  __out.push(one.clone()); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| {  let one: Py = __kwp(args, 0, "one").unwrap_or(Py::None); op_neg(&call_builtin("len", &[mcall(&one.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word-synonyms")); __args.push(py_tuple(vec![op_add(&call_builtin("tuple", &[g_WORD_SETS()]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("synonym"), py_str("synonyms"), py_str("word")])), g__SYNONYM_ASK()])); __args.push(py_func("_synonyms", |args: &[Py]| _synonyms(args))); __kws.extend(vec![("ask".to_string(), g__SYNONYM_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("mean"), py_str("meaning"), py_str("definition"), py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("pronounce"), py_str("in french"), py_str("in spanish"), py_str("in german"), py_str("translation"), py_str("translate")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data67() -> Py { py_dict(vec![(py_str("crisis"), py_str("crises")), (py_str("analysis"), py_str("analyses")), (py_str("thesis"), py_str("theses")), (py_str("basis"), py_str("bases")), (py_str("axis"), py_str("axes")), (py_str("oasis"), py_str("oases")), (py_str("diagnosis"), py_str("diagnoses")), (py_str("prognosis"), py_str("prognoses")), (py_str("hypothesis"), py_str("hypotheses")), (py_str("synopsis"), py_str("synopses")), (py_str("emphasis"), py_str("emphases")), (py_str("parenthesis"), py_str("parentheses")), (py_str("paralysis"), py_str("paralyses")), (py_str("ellipsis"), py_str("ellipses")), (py_str("neurosis"), py_str("neuroses")), (py_str("metamorphosis"), py_str("metamorphoses"))]) }
    fn __data68() -> Py { py_dict(vec![(py_str("child"), py_str("children")), (py_str("person"), py_str("people")), (py_str("man"), py_str("men")), (py_str("woman"), py_str("women")), (py_str("tooth"), py_str("teeth")), (py_str("foot"), py_str("feet")), (py_str("mouse"), py_str("mice")), (py_str("goose"), py_str("geese")), (py_str("ox"), py_str("oxen")), (py_str("louse"), py_str("lice")), (py_str("policeman"), py_str("policemen")), (py_str("fireman"), py_str("firemen")), (py_str("spokesman"), py_str("spokesmen")), (py_str("chairman"), py_str("chairmen"))]) }
    fn __data69() -> Py { py_dict(vec![(py_str("knife"), py_str("knives")), (py_str("wife"), py_str("wives")), (py_str("life"), py_str("lives")), (py_str("leaf"), py_str("leaves")), (py_str("half"), py_str("halves")), (py_str("loaf"), py_str("loaves")), (py_str("shelf"), py_str("shelves")), (py_str("thief"), py_str("thieves")), (py_str("wolf"), py_str("wolves")), (py_str("calf"), py_str("calves")), (py_str("scarf"), py_str("scarves")), (py_str("hoof"), py_str("hooves")), (py_str("self"), py_str("selves")), (py_str("elf"), py_str("elves")), (py_str("wharf"), py_str("wharves")), (py_str("sheaf"), py_str("sheaves"))]) }
    fn __data70() -> Py { py_dict(vec![(py_str("bus"), py_str("buses")), (py_str("box"), py_str("boxes")), (py_str("church"), py_str("churches")), (py_str("dish"), py_str("dishes")), (py_str("glass"), py_str("glasses")), (py_str("class"), py_str("classes")), (py_str("watch"), py_str("watches")), (py_str("brush"), py_str("brushes")), (py_str("bench"), py_str("benches")), (py_str("branch"), py_str("branches")), (py_str("match"), py_str("matches")), (py_str("quiz"), py_str("quizzes")), (py_str("fox"), py_str("foxes")), (py_str("dress"), py_str("dresses")), (py_str("wish"), py_str("wishes")), (py_str("kiss"), py_str("kisses")), (py_str("bunch"), py_str("bunches")), (py_str("boss"), py_str("bosses"))]) }
    fn __data71() -> Py { py_dict(vec![(py_str("city"), py_str("cities")), (py_str("baby"), py_str("babies")), (py_str("story"), py_str("stories")), (py_str("country"), py_str("countries")), (py_str("party"), py_str("parties")), (py_str("family"), py_str("families")), (py_str("company"), py_str("companies")), (py_str("lady"), py_str("ladies")), (py_str("puppy"), py_str("puppies")), (py_str("enemy"), py_str("enemies")), (py_str("hobby"), py_str("hobbies")), (py_str("diary"), py_str("diaries")), (py_str("memory"), py_str("memories")), (py_str("library"), py_str("libraries")), (py_str("army"), py_str("armies")), (py_str("factory"), py_str("factories")), (py_str("century"), py_str("centuries")), (py_str("salary"), py_str("salaries")), (py_str("berry"), py_str("berries")), (py_str("supply"), py_str("supplies")), (py_str("sky"), py_str("skies")), (py_str("fly"), py_str("flies")), (py_str("penny"), py_str("pennies")), (py_str("trolley"), py_str("trolleys"))]) }
    fn __data72() -> Py { py_dict(vec![(py_str("datum"), py_str("data")), (py_str("medium"), py_str("media")), (py_str("bacterium"), py_str("bacteria")), (py_str("criterion"), py_str("criteria")), (py_str("phenomenon"), py_str("phenomena")), (py_str("curriculum"), py_str("curricula")), (py_str("memorandum"), py_str("memoranda")), (py_str("index"), py_str("indices")), (py_str("appendix"), py_str("appendices")), (py_str("matrix"), py_str("matrices")), (py_str("vertex"), py_str("vertices")), (py_str("formula"), py_str("formulae")), (py_str("larva"), py_str("larvae"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_GROUPS"); }
    s__PLURAL_GROUPS(py_tuple(vec![py_tuple(vec![py_str("The -is words change to -es, as in analysis and analyses, basis and bases and thesis and theses."), __data67()]), py_tuple(vec![py_str("These are the old ones with no pattern to learn: person and people, man and men, tooth and teeth."), __data68()]), py_tuple(vec![py_str("Deer, fish, trout, aircraft and series work the same way: the word does not change at all."), py_dict(vec![(py_str("sheep"), py_str("sheep")), (py_str("deer"), py_str("deer")), (py_str("fish"), py_str("fish")), (py_str("trout"), py_str("trout")), (py_str("salmon"), py_str("salmon")), (py_str("aircraft"), py_str("aircraft")), (py_str("series"), py_str("series")), (py_str("species"), py_str("species")), (py_str("offspring"), py_str("offspring")), (py_str("headquarters"), py_str("headquarters"))])]), py_tuple(vec![py_str("The -f and -fe words turn into -ves: knife and knives, wife and wives, half and halves."), __data69()]), py_tuple(vec![py_str("A short list of -o words takes -es: tomato and tomatoes, hero and heroes, echo and echoes."), py_dict(vec![(py_str("potato"), py_str("potatoes")), (py_str("tomato"), py_str("tomatoes")), (py_str("hero"), py_str("heroes")), (py_str("echo"), py_str("echoes")), (py_str("veto"), py_str("vetoes")), (py_str("torpedo"), py_str("torpedoes")), (py_str("volcano"), py_str("volcanoes")), (py_str("mosquito"), py_str("mosquitoes")), (py_str("buffalo"), py_str("buffaloes")), (py_str("domino"), py_str("dominoes")), (py_str("cargo"), py_str("cargoes")), (py_str("mango"), py_str("mangoes"))])]), py_tuple(vec![py_str("Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes, church and churches."), __data70()]), py_tuple(vec![py_str("A consonant then y becomes -ies: baby and babies, story and stories. A vowel then y just takes s, as in day and days."), __data71()]), py_tuple(vec![py_str("Not every -f word changes: roof and roofs, chief and chiefs, safe and safes."), py_dict(vec![(py_str("roof"), py_str("roofs")), (py_str("chief"), py_str("chiefs")), (py_str("chef"), py_str("chefs")), (py_str("belief"), py_str("beliefs")), (py_str("cliff"), py_str("cliffs")), (py_str("safe"), py_str("safes")), (py_str("gulf"), py_str("gulfs")), (py_str("handkerchief"), py_str("handkerchiefs")), (py_str("reef"), py_str("reefs")), (py_str("proof"), py_str("proofs"))])]), py_tuple(vec![py_str("These keep the Latin or Greek ending: medium and media, criterion and criteria, phenomenon and phenomena."), __data72()])]));
    fn __data73() -> Py { py_set(vec![py_str("information"), py_str("advice"), py_str("furniture"), py_str("luggage"), py_str("homework"), py_str("equipment"), py_str("knowledge"), py_str("money"), py_str("water"), py_str("milk"), py_str("rice"), py_str("bread"), py_str("news"), py_str("research"), py_str("traffic"), py_str("weather"), py_str("hair"), py_str("salt")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._UNCOUNTABLE"); }
    s__UNCOUNTABLE(__data73());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLAIN_NOUNS"); }
    s__PLAIN_NOUNS(mcall(&py_str("answer animal apple bag ball banana basket bath bean bed bee bike bird birthday\nbiscuit boat body bone bottle bowl brother building burger cake camera candle card carpet carrot\ncat chair chicken chip clock coat coin computer cook corner cow cup curtain customer daughter day\ndesk dinner doctor dog door driver ear egg engineer evening event exam eye face fact farm farmer\nfather festival film finger floor flower fork friend game garden gate girl goal group guest hand\nhat head hill holiday home horse hospital hotel hour house idea island jacket job journey kettle\nkey kid kitchen lake lamp laptop leg lesson letter light line list lunch machine map market meal\nmeeting message minute mirror mistake month morning mother mountain mouth movie museum name\nneighbour night noise nose note nurse office onion orange page paper parent park passenger pasta\npath pen pencil phone photo piano picture piece plane plate pocket point question radio receipt\nrestaurant ring river road room salad sandwich school scientist sea season seat sentence shop\nshoulder sister site slice sofa son song spoon sport spring stair star station street student\nsummer sun supermarket surprise sweet table taxi teacher team teenager telephone television thing\nticket town train tree trip truck uncle vegetable village visit voice wall week weekend window\nwinter word worker world year zoo ant bear camel dolphin duck elephant frog giraffe insect kangaroo\nlion monkey parrot penguin rabbit shark snake spider tiger whale zebra"), "split", &[]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_GROUPS"); }
    s__PLURAL_GROUPS(op_add(&g__PLURAL_GROUPS(), &py_tuple(vec![py_tuple(vec![py_str("Most words just add s: cat and cats, plan and plans, house and houses."), _plain_plurals(&[])])])));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.PLURALS"); }
    s_PLURALS(_plurals_by_word(&[]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.PLURAL_SINGULARS"); }
    s_PLURAL_SINGULARS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_PLURALS(), "items", &[])) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut one: Py = __unpk1[0].clone(); let __unpk2 = py_unpack(&__unpk1[1].clone(), 2, None); let mut more: Py = __unpk2[0].clone(); let mut _pattern: Py = __unpk2[1].clone(); if py_truthy(&Py::Bool(!py_eq(&more.clone(), &one.clone()))) {  __out.push((more.clone(), one.clone())); } } py_dict(__out) })() });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_ASK"); }
    s__PLURAL_ASK(py_tuple(vec![py_str("plural"), py_str("plurals"), py_str("pluralise"), py_str("pluralize"), py_str("pluralised"), py_str("pluralized")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_ASK_FRAME"); }
    s__PLURAL_ASK_FRAME(py_tuple(vec![py_str("of"), py_str("for"), py_str("make"), py_str("say"), py_str("in"), py_str("to"), py_str("work"), py_str("works"), py_str("made"), py_str("spelt"), py_str("spelled"), py_str("formed")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_GENERAL"); }
    s__PLURAL_GENERAL(py_tuple(vec![py_str("word"), py_str("words"), py_str("plurals"), py_str("nouns"), py_str("names")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_RULE"); }
    s__PLURAL_RULE(py_str("Most plurals just add s (cat, cats). Words ending in s, x, z, ch or sh take -es (bus, buses), a consonant then y becomes -ies (baby, babies), and -f or -fe turns into -ves (knife, knives)."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_RULE_SENTENCE"); }
    s__PLURAL_RULE_SENTENCE(py_dict(vec![(py_str("es"), py_str("Words ending in s, x, z, ch or sh take -es: bus and buses, box and boxes.")), (py_str("ies"), py_str("A consonant then y becomes -ies: baby and babies, city and cities.")), (py_str("oes"), py_str("A short list of -o words takes -es: potato and potatoes, hero and heroes.")), (py_str("plain"), py_str("Most words just add s: cat and cats, house and houses.")), (py_str("os"), py_str("A short -o word takes -es (potato and potatoes, echo and echoes); a -o word that takes a plain s does too, so it is burritos, avocados and pianos."))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_LATIN_TWIN"); }
    s__PLURAL_LATIN_TWIN(py_dict(vec![(py_str("cactus"), py_str("cacti")), (py_str("octopus"), py_str("octopi")), (py_str("fungus"), py_str("fungi")), (py_str("nucleus"), py_str("nuclei")), (py_str("syllabus"), py_str("syllabi"))]));
    fn __data74() -> Py { py_set(vec![py_str("potato"), py_str("tomato"), py_str("hero"), py_str("echo"), py_str("veto"), py_str("torpedo"), py_str("volcano"), py_str("mosquito"), py_str("buffalo"), py_str("domino"), py_str("cargo"), py_str("embargo"), py_str("mango"), py_str("tornado"), py_str("motto"), py_str("grotto"), py_str("dingo")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_OES"); }
    s__PLURAL_OES(call_builtin("frozenset", &[__data74()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data75() -> Py { py_set(vec![py_str("moose"), py_str("bison"), py_str("swine"), py_str("spacecraft"), py_str("hovercraft"), py_str("salmon"), py_str("trout"), py_str("cod"), py_str("grouse"), py_str("offspring"), py_str("corps"), py_str("means"), py_str("series"), py_str("species"), py_str("aircraft"), py_str("deer"), py_str("sheep"), py_str("fish")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_SAME_WORD"); }
    s__PLURAL_SAME_WORD(__data75());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_NO_COUNT"); }
    s__PLURAL_NO_COUNT(py_tuple(vec![py_str("ness")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_NONE_SENTENCE"); }
    s__PLURAL_NONE_SENTENCE(py_str("{Word} has no plural: it is an amount, not a count, so you would say some {word}, or a bit of {word}."));
    fn __data76() -> Py { py_set(vec![py_str("has"), py_str("have"), py_str("had"), py_str("is"), py_str("was"), py_str("were"), py_str("be"), py_str("been"), py_str("am"), py_str("are"), py_str("do"), py_str("does"), py_str("did"), py_str("will"), py_str("would"), py_str("can"), py_str("could"), py_str("should"), py_str("may"), py_str("might"), py_str("must"), py_str("shall"), py_str("happy"), py_str("sad"), py_str("angry"), py_str("good"), py_str("bad"), py_str("nice"), py_str("big"), py_str("small"), py_str("little"), py_str("great"), py_str("lovely"), py_str("tired"), py_str("hungry"), py_str("thirsty"), py_str("cold"), py_str("hot"), py_str("warm"), py_str("wet"), py_str("dry"), py_str("fast"), py_str("slow"), py_str("easy"), py_str("hard"), py_str("difficult"), py_str("beautiful"), py_str("important"), py_str("different"), py_str("better"), py_str("best"), py_str("worse")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_NOT_A_NOUN_AT_ALL"); }
    s__PLURAL_NOT_A_NOUN_AT_ALL(__data76());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_US_SENTENCE"); }
    s__PLURAL_US_SENTENCE(py_str("A -us word takes the English -es: virus and viruses, campus and campuses."));
    fn __data77() -> Py { py_set(vec![py_str("the"), py_str("a"), py_str("an"), py_str("it"), py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("there"), py_str("word"), py_str("words"), py_str("noun"), py_str("nouns"), py_str("name"), py_str("names"), py_str("thing"), py_str("things"), py_str("one"), py_str("ones"), py_str("and"), py_str("or"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been"), py_str("please"), py_str("thanks"), py_str("for"), py_str("of"), py_str("in"), py_str("to"), py_str("make"), py_str("say"), py_str("spell"), py_str("spelling"), py_str("rule"), py_str("rules"), py_str("any"), py_str("some"), py_str("more"), py_str("up"), py_str("what"), py_str("whats"), py_str("what's"), py_str("how"), py_str("do"), py_str("does"), py_str("did"), py_str("i"), py_str("you"), py_str("we"), py_str("they"), py_str("me"), py_str("my"), py_str("tell"), py_str("give"), py_str("sentence"), py_str("sentences"), py_str("about"), py_str("wrong"), py_str("right"), py_str("correct"), py_str("english"), py_str("wifi"), py_str("wi-fi"), py_str("internet"), py_str("broadband")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_NOT_THE_NOUN"); }
    s__PLURAL_NOT_THE_NOUN(__data77());
    let __aug78 = op_bitor(&g__PLURAL_NOT_THE_NOUN(), &op_bitor(&call_builtin("set", &[g__PLURAL_ASK()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g__PLURAL_ASK_FRAME()]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLURAL_NOT_THE_NOUN"); }
    s__PLURAL_NOT_THE_NOUN(__aug78);
    fn __data79() -> Py { py_dict(vec![(py_str("be"), py_tuple(vec![py_str("was"), py_str("been")])), (py_str("become"), py_tuple(vec![py_str("became"), py_str("become")])), (py_str("begin"), py_tuple(vec![py_str("began"), py_str("begun")])), (py_str("break"), py_tuple(vec![py_str("broke"), py_str("broken")])), (py_str("bring"), py_tuple(vec![py_str("brought"), py_str("brought")])), (py_str("build"), py_tuple(vec![py_str("built"), py_str("built")])), (py_str("buy"), py_tuple(vec![py_str("bought"), py_str("bought")])), (py_str("catch"), py_tuple(vec![py_str("caught"), py_str("caught")])), (py_str("choose"), py_tuple(vec![py_str("chose"), py_str("chosen")])), (py_str("come"), py_tuple(vec![py_str("came"), py_str("come")])), (py_str("do"), py_tuple(vec![py_str("did"), py_str("done")])), (py_str("drink"), py_tuple(vec![py_str("drank"), py_str("drunk")])), (py_str("drive"), py_tuple(vec![py_str("drove"), py_str("driven")])), (py_str("eat"), py_tuple(vec![py_str("ate"), py_str("eaten")])), (py_str("fall"), py_tuple(vec![py_str("fell"), py_str("fallen")])), (py_str("feel"), py_tuple(vec![py_str("felt"), py_str("felt")])), (py_str("fight"), py_tuple(vec![py_str("fought"), py_str("fought")])), (py_str("find"), py_tuple(vec![py_str("found"), py_str("found")])), (py_str("flee"), py_tuple(vec![py_str("fled"), py_str("fled")])), (py_str("fly"), py_tuple(vec![py_str("flew"), py_str("flown")])), (py_str("forget"), py_tuple(vec![py_str("forgot"), py_str("forgotten")])), (py_str("get"), py_tuple(vec![py_str("got"), py_str("got")])), (py_str("give"), py_tuple(vec![py_str("gave"), py_str("given")])), (py_str("go"), py_tuple(vec![py_str("went"), py_str("gone")])), (py_str("grow"), py_tuple(vec![py_str("grew"), py_str("grown")])), (py_str("have"), py_tuple(vec![py_str("had"), py_str("had")])), (py_str("hear"), py_tuple(vec![py_str("heard"), py_str("heard")])), (py_str("hide"), py_tuple(vec![py_str("hid"), py_str("hidden")])), (py_str("hold"), py_tuple(vec![py_str("held"), py_str("held")])), (py_str("keep"), py_tuple(vec![py_str("kept"), py_str("kept")])), (py_str("know"), py_tuple(vec![py_str("knew"), py_str("known")])), (py_str("leave"), py_tuple(vec![py_str("left"), py_str("left")])), (py_str("lose"), py_tuple(vec![py_str("lost"), py_str("lost")])), (py_str("make"), py_tuple(vec![py_str("made"), py_str("made")])), (py_str("meet"), py_tuple(vec![py_str("met"), py_str("met")])), (py_str("pay"), py_tuple(vec![py_str("paid"), py_str("paid")])), (py_str("put"), py_tuple(vec![py_str("put"), py_str("put")])), (py_str("read"), py_tuple(vec![py_str("read"), py_str("read")])), (py_str("ride"), py_tuple(vec![py_str("rode"), py_str("ridden")])), (py_str("ring"), py_tuple(vec![py_str("rang"), py_str("rung")])), (py_str("rise"), py_tuple(vec![py_str("rose"), py_str("risen")])), (py_str("run"), py_tuple(vec![py_str("ran"), py_str("run")])), (py_str("say"), py_tuple(vec![py_str("said"), py_str("said")])), (py_str("see"), py_tuple(vec![py_str("saw"), py_str("seen")])), (py_str("sell"), py_tuple(vec![py_str("sold"), py_str("sold")])), (py_str("send"), py_tuple(vec![py_str("sent"), py_str("sent")])), (py_str("sing"), py_tuple(vec![py_str("sang"), py_str("sung")])), (py_str("sit"), py_tuple(vec![py_str("sat"), py_str("sat")])), (py_str("sleep"), py_tuple(vec![py_str("slept"), py_str("slept")])), (py_str("speak"), py_tuple(vec![py_str("spoke"), py_str("spoken")])), (py_str("spend"), py_tuple(vec![py_str("spent"), py_str("spent")])), (py_str("stand"), py_tuple(vec![py_str("stood"), py_str("stood")])), (py_str("steal"), py_tuple(vec![py_str("stole"), py_str("stolen")])), (py_str("swim"), py_tuple(vec![py_str("swam"), py_str("swum")])), (py_str("take"), py_tuple(vec![py_str("took"), py_str("taken")])), (py_str("teach"), py_tuple(vec![py_str("taught"), py_str("taught")])), (py_str("tell"), py_tuple(vec![py_str("told"), py_str("told")])), (py_str("think"), py_tuple(vec![py_str("thought"), py_str("thought")])), (py_str("throw"), py_tuple(vec![py_str("threw"), py_str("thrown")])), (py_str("understand"), py_tuple(vec![py_str("understood"), py_str("understood")])), (py_str("wake"), py_tuple(vec![py_str("woke"), py_str("woken")])), (py_str("wear"), py_tuple(vec![py_str("wore"), py_str("worn")])), (py_str("win"), py_tuple(vec![py_str("won"), py_str("won")])), (py_str("write"), py_tuple(vec![py_str("wrote"), py_str("written")])), (py_str("bet"), py_tuple(vec![py_str("bet"), py_str("bet")])), (py_str("burst"), py_tuple(vec![py_str("burst"), py_str("burst")])), (py_str("cast"), py_tuple(vec![py_str("cast"), py_str("cast")])), (py_str("cost"), py_tuple(vec![py_str("cost"), py_str("cost")])), (py_str("cut"), py_tuple(vec![py_str("cut"), py_str("cut")])), (py_str("hit"), py_tuple(vec![py_str("hit"), py_str("hit")])), (py_str("hurt"), py_tuple(vec![py_str("hurt"), py_str("hurt")])), (py_str("let"), py_tuple(vec![py_str("let"), py_str("let")])), (py_str("quit"), py_tuple(vec![py_str("quit"), py_str("quit")])), (py_str("set"), py_tuple(vec![py_str("set"), py_str("set")])), (py_str("shut"), py_tuple(vec![py_str("shut"), py_str("shut")])), (py_str("split"), py_tuple(vec![py_str("split"), py_str("split")])), (py_str("spread"), py_tuple(vec![py_str("spread"), py_str("spread")]))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.IRREGULAR_VERBS"); }
    s_IRREGULAR_VERBS(__data79());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PAST_ASK"); }
    s__PAST_ASK(py_tuple(vec![py_str("past tense"), py_str("past simple"), py_str("past participle"), py_str("past form"), py_str("participle"), py_str("past of"), py_str("what is the past"), py_str("what's the past"), py_str("whats the past"), py_str("irregular verb"), py_str("irregular verbs")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._VOWELS"); }
    s__VOWELS(py_str("aeiou"));
    fn __data80() -> Py { py_set(vec![py_str("travel"), py_str("cancel"), py_str("label"), py_str("signal"), py_str("model"), py_str("level"), py_str("dial"), py_str("patrol"), py_str("total"), py_str("tunnel"), py_str("fuel"), py_str("marshal"), py_str("worship"), py_str("kidnap"), py_str("handicap")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._DOUBLED_L"); }
    s__DOUBLED_L(__data80());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PAST_OF"); }
    s__PAST_OF(stdlib_call("re", "compile", &[py_str("past (?:tense |form |participle |simple )?of (?:the verb )?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._TWO_VERBS"); }
    s__TWO_VERBS(py_dict(vec![(py_str("lie"), py_str("Lie is two verbs. Lying down is lie, lay, lain: 'I lay on the bed', 'I have lain there'. Telling a lie is lie, lied, lied: 'I lied to him'. The one people get wrong is the reclining one - 'I laid on the bed' should be 'I lay on the bed', because laid is the past of lay, meaning to put something down.")), (py_str("lay"), py_str("Lay is laid, laid: 'I laid the table', 'I have laid it out'. The trap is that lay is also the past of lie, the lying-down verb - so 'I lay on the bed' is correct for reclining, and 'I laid the baby on the bed' is correct for putting her down. If you can replace it with put, use laid."))]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("regular-past")); __args.push(py_tuple(vec![g__PAST_ASK(), g__PAST_ASK()])); __args.push(py_func("_regular_past", |args: &[Py]| _regular_past(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("guard".to_string(), py_func("_names_a_past_verb", |args: &[Py]| _names_a_past_verb(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("irregular-past")); __args.push(py_tuple(vec![call_builtin("tuple", &[g_IRREGULAR_VERBS()]).unwrap_or_else(|e| exc::raise(e)), g__PAST_ASK()])); __args.push(py_func("_irregular_past", |args: &[Py]| _irregular_past(args))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("guard".to_string(), py_func("_names_a_past_table_verb", |args: &[Py]| _names_a_past_table_verb(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plural-word")); __args.push(py_tuple(vec![g__PLURAL_ASK(), g__PLURAL_ASK_FRAME()])); __args.push(py_func("_plural_of", |args: &[Py]| _plural_of(args))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("guard".to_string(), py_func("_names_a_word_to_change", |args: &[Py]| _names_a_word_to_change(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data81() -> Py { py_dict(vec![(py_str("noun"), py_str("A noun names a person, place or thing - Jess, Leeds, table, kindness. If you can put 'the' in front of it, it is a noun.")), (py_str("verb"), py_str("A verb is the doing or being word: run, eat, seems, is. Every sentence needs one, and the verb changes with the time - I walk today, I walked yesterday.")), (py_str("adjective"), py_str("An adjective describes a noun: a red bus, a tired dog, an honest answer. It sits before the noun or after 'is' - the bus is red.")), (py_str("adverb"), py_str("An adverb says how, when or where something happened: she sang loudly, we left early, he waited outside. Many end in -ly, but not all do (soon, well, fast).")), (py_str("pronoun"), py_str("A pronoun stands in for a noun so you are not repeating it: I, you, he, she, it, we, they, me, him, her, them. 'Sam lost Sam's keys' is why we have them.")), (py_str("preposition"), py_str("A preposition says where or when in relation to something: in, on, under, between, before, after, with. The cat sat on the mat - 'on' is the preposition.")), (py_str("conjunction"), py_str("A conjunction joins words or clauses: and, but, because, although, so. 'I was late because the bus broke down' joins two ideas with 'because'.")), (py_str("tense"), py_str("Tense is when the verb happens: past (I cooked), present (I cook, I am cooking), future (I will cook). Pick one and keep to it within a sentence.")), (py_str("clause"), py_str("A clause is a group of words with its own verb and subject: 'when the bus came' and 'we got on'. A sentence is one or more clauses; a phrase has no verb at all.")), (py_str("sentence"), py_str("A sentence needs a subject and a verb and makes one complete point, starting with a capital and ending with a full stop: 'The bus was late.'")), (py_str("fronted adverbial"), py_str("A fronted adverbial is the phrase that opens a sentence and says when, where or how: 'After lunch, we walked.' 'Slowly, the door opened.' Put a comma after it, and make sure the thing it describes is the subject of the sentence - 'Walking home, the rain started' says the rain was walking.")), (py_str("split infinitive"), py_str("A split infinitive puts a word between to and the verb: 'to boldly go'. It is not an error, whatever you were taught - split it when it reads better and move the word when it gets clumsy: 'to go boldly'.")), (py_str("dangling modifier"), py_str("A dangling modifier is an opening phrase that describes the wrong thing: 'Walking to work, the rain soaked me' says the rain was walking. Name the person straight after the comma - 'Walking to work, I was soaked by the rain' - or turn the phrase into a clause.")), (py_str("whom"), py_str("Use whom after a preposition and where it is the one being acted on: 'to whom it may concern', 'the person whom I met'. If you can say he or she, write who; if you would say him or her, write whom. In speech who nearly always sounds right.")), (py_str("adverbial"), py_str("An adverbial is any phrase that says when, where, how or why: 'at six', 'in the kitchen', 'very slowly', 'because it rained'. It is not always an adverb - it can be a whole phrase or clause.")), (py_str("apostrophe"), py_str("An apostrophe does two jobs: it marks something missing (do not becomes don't) or shows belonging (Jess's coat). It never makes a plain plural - two cats, not two cat's.")), (py_str("hashtag"), py_str("A hashtag is the # sign used as a label: #slowcooking groups posts about one thing. On a UK keyboard it is Shift and 3, and on a phone it is on the symbols page.")), (py_str("number sign"), py_str("The number sign is # - Shift and 3 on a UK keyboard. In the US it is how you write a number (No. 5) and online it starts a hashtag.")), (py_str("ampersand"), py_str("An ampersand is the & sign and it means 'and'. Write and out in a sentence; the sign is for names and headings, like Marks & Spencer or R&D.")), (py_str("at sign"), py_str("The at sign is @ - Shift and 2 on a UK keyboard - and it is in every email address: name@example.com.")), (py_str("asterisk"), py_str("An asterisk is the * sign, a small star. It marks a footnote or shows anything goes (*), and on a phone keyboard it is usually behind the ?123 key.")), (py_str("slash"), py_str("A slash is the / sign, and it mostly means 'or' (and/or) or separates the parts of a date (12/05/2024). A backslash \\ is the other way over and turns up in computer paths rather than in writing.")), (py_str("backslash"), py_str("A backslash is the \\ sign, the mirror of a slash. It almost never belongs in writing - it separates folders in a Windows file path, like C:\\Users\\me.")), (py_str("underscore"), py_str("An underscore is the _ sign, the low line under a letter. It joins words in file names and usernames (my_notes.txt); a hyphen is the shorter mark at half height.")), (py_str("tilde"), py_str("A tilde is the ~ sign, the small wavy line. It means 'about' (~20 people came), and on most keyboards it is Shift and the hash key.")), (py_str("pipe"), py_str("A pipe is the | sign, the upright bar. Most people call it the pipe or the vertical bar, and it separates options in a computer command; in writing a slash does the job better.")), (py_str("caret"), py_str("A caret is the ^ sign, a small wedge above the line. In proofreading it marks where a word is to be slipped in, and in maths it means to the power of - 2^3 is eight.")), (py_str("bullet point"), py_str("A bullet point is the dot that starts each item in a list. Keep every item to a line, start each one the same way, and leave the full stops off short items.")), (py_str("curly bracket"), py_str("Curly brackets { } group things in maths and code, and square brackets [ ] hold your own words inside someone else's quotation. Round brackets ( ) take an aside the sentence could do without.")), (py_str("square bracket"), py_str("Square brackets [ ] hold your own words inside a quotation, and they mark a missing word in a transcript: 'she said it was [unclear] on the tape'.")), (py_str("angle bracket"), py_str("Angle brackets < > mark a tag in code and stand in for a word to fill in later, as in <your name>. The plain brackets ( ) are the ones for writing."))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.GRAMMAR_TERMS"); }
    s_GRAMMAR_TERMS(__data81());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._GRAMMAR_WORDS"); }
    s__GRAMMAR_WORDS(op_add(&call_builtin("tuple", &[g_GRAMMAR_TERMS()]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("grammar"), py_str("parts of speech"), py_str("word class")])));
    fn __data82() -> Py { py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("what are"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("define"), py_str("explain"), py_str("difference"), py_str("example"), py_str("how do i know"), py_str("simple terms"), py_str("use"), py_str("uses"), py_str("when do i"), py_str("where do i"), py_str("for kids"), py_str("for a child"), py_str("called"), py_str("what do you call"), py_str("what is that"), py_str("what is this"), py_str("name of"), py_str("which key"), py_str("what key")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._GRAMMAR_ASK"); }
    s__GRAMMAR_ASK(__data82());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._SOMEBODY_FORGETTING"); }
    s__SOMEBODY_FORGETTING(stdlib_call("re", "compile", &[py_str("\\b(?:keeps?\\s+forgetting|forgets|forgetting|forgot|repeats?\\s+(?:himself|herself|themselves|itself)|cant\\s+remember|can't\\s+remember|mid\\s+sentence|cant\\s+find\\s+(?:his|her|their)\\s+words|can't\\s+find\\s+(?:his|her|their)\\s+words)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("grammar-term")); __args.push(py_tuple(vec![g__GRAMMAR_WORDS(), g__GRAMMAR_ASK()])); __args.push(py_func("_grammar_term", |args: &[Py]| _grammar_term(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("difference between affect"), py_str("affect"), py_str("effect"), py_str("apostrophe its"), py_str("its or it's")])), ("ask".to_string(), g__GRAMMAR_ASK()), ("guard".to_string(), py_func("_nobody_in_the_line_is_forgetting", |args: &[Py]| _nobody_in_the_line_is_forgetting(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._POEM_FORMS"); }
    s__POEM_FORMS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("sonnet"), py_str("sonnets")]), py_str("A sonnet is fourteen lines in iambic pentameter - ten syllables a line, stress on every second one. Shakespeare's kind rhymes in three quatrains and a closing couplet (abab cdcd efef gg) and turns at line nine; the Italian kind splits six and eight.")]), py_tuple(vec![py_tuple(vec![py_str("stanza"), py_str("stanzas"), py_str("verse paragraph")]), py_str("A stanza is a group of lines set apart in a poem, the way a paragraph is in prose. Two lines are a couplet, four a quatrain, and the pattern of rhymes across them is the rhyme scheme.")]), py_tuple(vec![py_tuple(vec![py_str("free verse"), py_str("free-verse")]), py_str("Free verse is poetry with no fixed metre and no rhyme: the line breaks and the rhythm of the sentences do the work instead. Most modern poetry is written this way.")]), py_tuple(vec![py_tuple(vec![py_str("iambic pentameter"), py_str("iambic"), py_str("pentameter"), py_str("blank verse")]), py_str("Iambic pentameter is five beats of da-DUM to the line - ten syllables with the stress on every second one: 'Shall I compare thee to a summer's day'. Blank verse is that without rhyme, which is what most of Shakespeare's plays are in.")]), py_tuple(vec![py_tuple(vec![py_str("couplet"), py_str("couplets")]), py_str("A couplet is two lines that rhyme and usually stand together: 'Good night, good night! Parting is such sweet sorrow / That I shall say good night till it be morrow.'")]), py_tuple(vec![py_tuple(vec![py_str("ballad"), py_str("ballads")]), py_str("A ballad is a story told in short verses, meant to be sung or recited: four-line stanzas that rhyme abab or abcb, with a refrain that comes back. The old folk ballads are the kind that run to a dozen verses, and pop still uses the name for a slow song about one feeling.")])]));
    fn __data83() -> Py { py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("what are"), py_str("mean"), py_str("meaning"), py_str("definition"), py_str("define"), py_str("explain"), py_str("example"), py_str("how many lines"), py_str("structure"), py_str("form"), py_str("difference"), py_str("written"), py_str("write a"), py_str("poem"), py_str("poetry")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._POEM_ASK"); }
    s__POEM_ASK(__data83());
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("poem-form")); __args.push(py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__POEM_FORMS()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk1[0].clone(); let mut _u: Py = __unpk1[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), g__POEM_ASK()])); __args.push(py_func("_poem_form", |args: &[Py]| _poem_form(args))); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("write me"), py_str("write a poem"), py_str("write one"), py_str("about my"), py_str("for my"), py_str("birthday"), py_str("wedding"), py_str("card"), py_str("funny"), py_str("sad poem"), py_str("short poem")])), ("ask".to_string(), g__POEM_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data84() -> Py { py_tuple(vec![py_str("how do i"), py_str("how"), py_str("write"), py_str("writing"), py_str("start"), py_str("structure"), py_str("what should"), py_str("should i"), py_str("begin"), py_str("open"), py_str("good"), py_str("help"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("define"), py_str("definition"), py_str("what makes")]) }
    fn __data85() -> Py { py_tuple(vec![py_str("film"), py_str("movie"), py_str("tv"), py_str("netflix"), py_str("restaurant"), py_str("hotel"), py_str("album"), py_str("gig"), py_str("concert"), py_str("video game"), py_str("app"), py_str("product"), py_str("amazon"), py_str("perfume"), py_str("refund"), py_str("faulty"), py_str("broken"), py_str("return it"), py_str("one star"), py_str("bad review"), py_str("left a review")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("book-review")); __args.push(py_tuple(vec![py_tuple(vec![py_str("book review"), py_str("book reviews"), py_str("review a book"), py_str("reviewing a book"), py_str("review of a book"), py_str("review the book"), py_str("book report")]), __data84()])); __args.push(py_str("Open with your verdict in one sentence, then say what the book tries to do and whether it works: a paragraph on the whole thing, a paragraph for each point you want to make with a quote or a page number, and no more than a few lines of plot. Leave the ending out, and finish with who the book suits, so the reader can decide.")); __kws.extend(vec![("avoid".to_string(), __data85())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data86() -> Py { py_tuple(vec![py_str("start an email"), py_str("start the email"), py_str("start my email"), py_str("how do i start"), py_str("ignores my email"), py_str("ignores my emails"), py_str("ignoring my email"), py_str("ignoring my emails"), py_str("wont reply"), py_str("won't reply"), py_str("wont answer"), py_str("won't answer"), py_str("doesnt reply"), py_str("doesn't reply"), py_str("not replying"), py_str("hasnt replied"), py_str("hasn't replied"), py_str("never replies"), py_str("no reply"), py_str("keeps sending"), py_str("keeps emailing"), py_str("sent me an email"), py_str("sent me a email"), py_str("sending me"), py_str("sends me"), py_str("emailed me"), py_str("email from"), py_str("emails from"), py_str("message from"), py_str("formal"), py_str("polite"), py_str("professional"), py_str("rude"), py_str("angry"), py_str("tone"), py_str("friendly"), py_str("blunt"), py_str("firm"), py_str("sounds"), py_str("leaving"), py_str("leaving do"), py_str("last day"), py_str("goodbye"), py_str("farewell"), py_str("gushy"), py_str("my team"), py_str("deadline"), py_str("extension"), py_str("extensions"), py_str("resign"), py_str("resigning"), py_str("quit"), py_str("notice"), py_str("applying"), py_str("application"), py_str("applications"), py_str("job"), py_str("jobs"), py_str("interview"), py_str("interviewer"), py_str("reference"), py_str("references"), py_str("cover letter"), py_str("cv"), py_str("complain"), py_str("complaining"), py_str("complaint"), py_str("complaints"), py_str("sick"), py_str("ill"), py_str("unwell"), py_str("poorly"), py_str("doctor"), py_str("not working"), py_str("won't send"), py_str("wont send"), py_str("bounce"), py_str("bounced"), py_str("spam"), py_str("junk"), py_str("inbox"), py_str("gmail"), py_str("outlook"), py_str("yahoo"), py_str("password"), py_str("hacked"), py_str("phishing"), py_str("scam"), py_str("set up"), py_str("setup"), py_str("sign in"), py_str("log in"), py_str("login"), py_str("account"), py_str("address"), py_str("attachment"), py_str("unsubscribe"), py_str("mailing list"), py_str("newsletter"), py_str("marketing"), py_str("so many emails"), py_str("so many email"), py_str("too many emails"), py_str("fewer emails"), py_str("less email"), py_str("loads of emails"), py_str("hundreds of emails"), py_str("flood of emails"), py_str("emails every day"), py_str("emails all day"), py_str("stop getting"), py_str("stop the emails"), py_str("drowning in email"), py_str("swamped with email"), py_str("filter emails"), py_str("filter the emails"), py_str("sort out my emails"), py_str("reduce emails"), py_str("reduce the emails"), py_str("scan"), py_str("scanner"), py_str("scanning"), py_str("scanned")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-an-email")); __args.push(py_tuple(vec![py_tuple(vec![py_str("email"), py_str("emails"), py_str("e-mail"), py_str("e mail"), py_str("an email to my boss"), py_str("email to a company"), py_str("email a company"), py_str("chase by email")]), py_tuple(vec![py_str("how do i"), py_str("how"), py_str("write"), py_str("writing"), py_str("subject"), py_str("structure"), py_str("what should"), py_str("should i"), py_str("phrase")])])); __args.push(py_func("_write_an_email", |args: &[Py]| _write_an_email(args))); __kws.extend(vec![("avoid".to_string(), __data86())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.PERSONAL_NEWS"); }
    s_PERSONAL_NEWS(py_tuple(vec![py_str("i quit"), py_str("i want to quit"), py_str("i am quitting"), py_str("im quitting"), py_str("i resigned"), py_str("i got fired"), py_str("i was sacked"), py_str("i lost my job"), py_str("i got the job"), py_str("i handed in my notice"), py_str("i am pregnant"), py_str("im pregnant")]));
    fn __data87() -> Py { py_tuple(vec![py_str("text"), py_str("texts"), py_str("txt"), py_str("message"), py_str("messages"), py_str("whatsapp"), py_str("sms"), py_str("dm"), py_str("reply"), py_str("replies"), py_str("reply to"), py_str("text back"), py_str("respond"), py_str("response")]) }
    fn __data88() -> Py { py_tuple(vec![py_str("help me write"), py_str("help me word"), py_str("write"), py_str("writing"), py_str("draft"), py_str("word it"), py_str("wording"), py_str("how do i"), py_str("how"), py_str("what should"), py_str("how do i say"), py_str("how should i say"), py_str("what should i say"), py_str("how do i ask"), py_str("how to ask"), py_str("text to"), py_str("message to"), py_str("whatsapp to"), py_str("open a text"), py_str("start a text"), py_str("start the text")]) }
    fn __data89() -> Py { py_tuple(vec![py_str("email"), py_str("emails"), py_str("e-mail"), py_str("e mail"), py_str("letter"), py_str("letters"), py_str("card"), py_str("cards"), py_str("cover letter"), py_str("cv"), py_str("resume"), py_str("essay"), py_str("assignment"), py_str("homework"), py_str("review"), py_str("review"), py_str("feedback"), py_str("complaint"), py_str("complaining"), py_str("resign"), py_str("resigning"), py_str("loud music"), py_str("music at night"), py_str("noise at night"), py_str("landlord"), py_str("landlady"), py_str("boiler"), py_str("leaking"), py_str("repair"), py_str("leaving"), py_str("last day"), py_str("farewell"), py_str("goodbye"), py_str("my team"), py_str("notice"), py_str("noticed"), py_str("got a text"), py_str("got a message"), py_str("received"), py_str("from my bank"), py_str("screenshot"), py_str("what does it mean"), py_str("mean"), py_str("means"), py_str("spam"), py_str("scam"), py_str("phishing"), py_str("wrong number"), py_str("not delivered"), py_str("wont send"), py_str("won't send"), py_str("blocked"), py_str("delete"), py_str("deleted"), py_str("reply to an apology"), py_str("respond to an apology"), py_str("answer an apology"), py_str("reply to a sorry"), py_str("accept an apology"), py_str("accepting an apology"), py_str("someone said sorry"), py_str("someone says sorry"), py_str("someone apologises"), py_str("someone apologized"), py_str("someone apologised"), py_str("said sorry to me"), py_str("says sorry to me"), py_str("apologised to me"), py_str("apologized to me"), py_str("apologises to me"), py_str("apologizes to me"), py_str("when someone apologises"), py_str("when someone apologized"), py_str("when someone says sorry"), py_str("my ex"), py_str("an ex"), py_str("to my ex"), py_str("should i send"), py_str("seller"), py_str("company"), py_str("refund"), py_str("money back"), py_str("chargeback"), py_str("charged"), py_str("paid by card"), py_str("ignoring"), py_str("ignored"), py_str("unresponsive"), py_str("not sending"), py_str("wont send"), py_str("won't send"), py_str("invitation"), py_str("invitations"), py_str("rsvp"), py_str("wedding"), py_str("job offer"), py_str("job offers"), py_str("offer"), py_str("interview"), py_str("interviewer"), py_str("recruiter"), py_str("application"), py_str("accept the"), py_str("professional"), py_str("formal"), py_str("polite"), py_str("register"), py_str("how long"), py_str("how soon"), py_str("how often"), py_str("how many days"), py_str("first date"), py_str("second date"), py_str("before texting"), py_str("when to text"), py_str("should i wait"), py_str("how many texts")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-text")); __args.push(py_tuple(vec![__data87(), __data88()])); __args.push(py_str("Three short lines: what it is about, what you want, and a thanks. \"Hi Sam, could we talk about the rota this week? Thursday suits me best. Thanks.\" No apology paragraph at the front, and say who you are if they may not have your number - tell me who it is for and what you need and I will write yours.")); __kws.extend(vec![("avoid".to_string(), op_add(&__data89(), &g_PERSONAL_NEWS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data90() -> Py { py_tuple(vec![py_str("write"), py_str("writing"), py_str("written"), py_str("how do i"), py_str("how to"), py_str("book"), py_str("novel"), py_str("help"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("define"), py_str("definition")]) }
    fn __data91() -> Py { py_tuple(vec![py_str("film"), py_str("movie"), py_str("netflix"), py_str("series"), py_str("episode"), py_str("school"), py_str("book report"), py_str("summary"), py_str("summarise"), py_str("summarize"), py_str("essay"), py_str("assignment"), py_str("class"), py_str("teacher"), py_str("dissertation"), py_str("thesis"), py_str("paper"), py_str("research"), py_str("abstract")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("book-blurb")); __args.push(py_tuple(vec![py_tuple(vec![py_str("blurb"), py_str("blurbs"), py_str("back cover"), py_str("back of the book"), py_str("jacket copy"), py_str("cover copy"), py_str("book description"), py_str("synopsis")]), __data90()])); __args.push(py_str("Write it in three moves: who the person is and what they want, the thing that gets in the way, and the question the reader will buy the book to answer - all in the present tense and under 200 words. Keep the climax out of it, end on the question rather than the answer, and read it aloud once: a blurb with the names of five characters in it reads like a synopsis.")); __kws.extend(vec![("avoid".to_string(), __data91()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data92() -> Py { py_tuple(vec![py_str("eulogy"), py_str("speech"), py_str("read out"), py_str("what do i say"), py_str("what to say"), py_str("condolence"), py_str("sympathy"), py_str("cost"), py_str("costs"), py_str("price"), py_str("how much"), py_str("pay for"), py_str("funeral plan"), py_str("will"), py_str("probate"), py_str("grief"), py_str("grieving"), py_str("miss her"), py_str("miss him")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-an-obituary")); __args.push(py_tuple(vec![py_tuple(vec![py_str("obituary"), py_str("obituaries"), py_str("death notice"), py_str("death notices"), py_str("announce a death"), py_str("announcement of a death"), py_str("notice of death"), py_str("in the paper"), py_str("in the newspaper")]), py_tuple(vec![py_str("write"), py_str("writing"), py_str("how do i"), py_str("how to"), py_str("word"), py_str("wording"), py_str("draft"), py_str("what do i put"), py_str("what to include"), py_str("help"), py_str("structure"), py_str("should i")])])); __args.push(py_str("Five things in the first two sentences: full name, age, where they lived, the date they died, and who they leave behind. Then one or two sentences of character - what they did for a living, what they loved, a phrase people will recognise - and the funeral or donation details at the end. Ring the funeral director first: they will word it with you and most put it on their own website free, and a local paper charges by the line.")); __kws.extend(vec![("avoid".to_string(), __data92()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._VERSE_INSTANCE_LEAD"); }
    s__VERSE_INSTANCE_LEAD(stdlib_call("re", "compile", &[py_str("^(?:please |can you |could you |will you |would you |ok |okay |so |and )*(?:give|write|make|compose|pen|say|tell|do|recite|sing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data93() -> Py { py_tuple(vec![py_str("how"), py_str("howto"), py_str("tips"), py_str("tip"), py_str("advice"), py_str("structure"), py_str("form"), py_str("forms"), py_str("rules"), py_str("rule"), py_str("explain"), py_str("explained"), py_str("guide"), py_str("better"), py_str("improve"), py_str("learn"), py_str("teach"), py_str("what"), py_str("whats"), py_str("why"), py_str("difference"), py_str("example"), py_str("examples"), py_str("start"), py_str("begin"), py_str("beginning"), py_str("method"), py_str("rhyme"), py_str("rhymes"), py_str("rhythm"), py_str("beats"), py_str("syllables"), py_str("lines"), py_str("write"), py_str("writing"), py_str("written")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._VERSE_METHOD_WORDS"); }
    s__VERSE_METHOD_WORDS(call_builtin("frozenset", &[__data93()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data94() -> Py { py_tuple(vec![py_str("ireland"), py_str("county"), py_str("city"), py_str("accent"), py_str("joke"), py_str("song"), py_str("school"), py_str("homework"), py_str("essay"), py_str("assignment"), py_str("class"), py_str("about"), py_str("for my"), py_str("for a"), py_str("for the"), py_str("for her"), py_str("for him"), py_str("for their")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-limerick")); __args.push(py_tuple(vec![py_tuple(vec![py_str("limerick"), py_str("limericks"), py_str("funny poem"), py_str("silly poem"), py_str("rhyming joke")]), py_tuple(vec![py_str("write"), py_str("writing"), py_str("how do i"), py_str("how to"), py_str("make up"), py_str("compose"), py_str("help"), py_str("give me"), py_str("can you")])])); __args.push(py_str("Five lines that rhyme AABBA, and the rhythm is the whole thing: three beats then three then three on the first two lines, two then two on the short ones, then back to three. Put a person and a place in line one (\"A fellow who lived in Peru\"), keep line five for the punchline, and read it aloud - if a beat is missing, the joke dies.")); __kws.extend(vec![("avoid".to_string(), __data94()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&_asks_for_the_verse(&[understood.clone()]))) })), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data95() -> Py { py_tuple(vec![py_str("essay"), py_str("analysis"), py_str("analyse"), py_str("analyze"), py_str("school"), py_str("homework"), py_str("exam"), py_str("revision"), py_str("class"), py_str("teacher"), py_str("shakespeare wrote"), py_str("18"), py_str("about"), py_str("for my"), py_str("for a"), py_str("for the"), py_str("for her"), py_str("for him"), py_str("for their")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-sonnet")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sonnet"), py_str("sonnets"), py_str("shakespearean sonnet"), py_str("petrarchan sonnet")]), py_tuple(vec![py_str("write"), py_str("writing"), py_str("how do i"), py_str("how to"), py_str("form"), py_str("rhyme"), py_str("structure"), py_str("help"), py_str("give me"), py_str("can you")])])); __args.push(py_str("Fourteen lines of iambic pentameter - ten beats a line, da-DUM five times - with either ABAB CDCD EFEF GG (Shakespeare's) or ABBAABBA then a sestet (the older Italian shape). The easy way in is Shakespeare's: three quatrains that each push the idea one step further, then a couplet that turns it, so write the last two lines first.")); __kws.extend(vec![("avoid".to_string(), __data95()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("word-parts")); __args.push(py_tuple(vec![py_tuple(vec![py_str("prefix"), py_str("prefixes"), py_str("suffix"), py_str("suffixes"), py_str("antonym"), py_str("antonyms")]), py_tuple(vec![py_str("mean"), py_str("means"), py_str("what"), py_str("whats"), py_str("definition"), py_str("example"), py_str("examples"), py_str("explain"), py_str("difference")])])); __args.push(py_str("A prefix goes on the front of a word and a suffix on the end: un- in unhappy, -ness in happiness, and the base word (happy) is the root. An antonym is the opposite - hot and cold, early and late - which is the other half of what a synonym is, so if the word you want means the same, look for a synonym, and if it means the reverse, an antonym.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("hyphen"), py_str("dashes"), py_str("prefix a call"), py_str("phone code"), py_str("dialling code"), py_str("numbers")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data96() -> Py { py_tuple(vec![py_str("app"), py_str("apps"), py_str("planner"), py_str("calendar"), py_str("notebook"), py_str("buy"), py_str("gift"), py_str("present"), py_str("school"), py_str("assignment"), py_str("class"), py_str("teacher"), py_str("essay")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-diary")); __args.push(py_tuple(vec![py_tuple(vec![py_str("diary"), py_str("diaries"), py_str("diary entry"), py_str("dear diary")]), py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("write"), py_str("writing"), py_str("start"), py_str("keep"), py_str("begin"), py_str("help")])])); __args.push(py_str("Write the date, then the one thing you would tell a friend about the day - what happened, and what you thought of it - in three or four lines and without tidying it up. Keep it where you will actually open it and write at a fixed time, because the habit is the hard part; a page of plain sentences beats a blank page waiting for a good day.")); __kws.extend(vec![("avoid".to_string(), __data96()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-toast-few-words")); __args.push(py_tuple(vec![py_tuple(vec![py_str("toast"), py_str("toasts"), py_str("raise a glass"), py_str("say a few words"), py_str("short speech")]), py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("write"), py_str("writing"), py_str("give"), py_str("make"), py_str("propose"), py_str("help")])])); __args.push(py_str("Two minutes is plenty: say who you are and why you are standing up, tell one story with a detail only you would know, then say the thing you actually want to say and sit down. Write the first and last line out word for word and let the middle be notes, because the room forgives a stumble and does not forgive a ramble.")); __kws.extend(vec![("avoid".to_string(), op_add(&py_tuple(vec![py_str("wedding"), py_str("bride"), py_str("groom"), py_str("best man"), py_str("eulogy"), py_str("funeral"), py_str("birthday"), py_str("retirement"), py_str("leaving"), py_str("how long is a speech"), py_str("450 words")]), &g_ANIMAL_NOUNS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("book-summary")); __args.push(py_tuple(vec![py_tuple(vec![py_str("book summary"), py_str("summarise a book"), py_str("summarize a book"), py_str("summary of a book"), py_str("plot summary"), py_str("chapter summary"), py_str("summarise the book"), py_str("summarize the book")]), py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("write"), py_str("writing"), py_str("make"), py_str("help"), py_str("notes")])])); __args.push(py_str("Go chapter by chapter before you write a line: one sentence per chapter on what changed, then join them into a paragraph that follows the argument or the plot rather than the order of the pages. Keep the ending in - a summary that hides it is a blurb - and put your own one-line judgement at the end, clearly marked as yours.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("review"), py_str("blurb"), py_str("back cover"), py_str("film"), py_str("movie"), py_str("episode"), py_str("abstract"), py_str("dissertation"), py_str("thesis"), py_str("paper"), py_str("essay"), py_str("school")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._AN_OPENING_QUESTION"); }
    s__AN_OPENING_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:opening|opener|starter|first|lead) questions?\\b|\\bquestion to open\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._AN_OPENING_IS_WRITTEN"); }
    s__AN_OPENING_IS_WRITTEN(stdlib_call("re", "compile", &[py_str("\\b(?:book|books|story|stories|novel|chapter|essay|blog|post|article|draft|manuscript|scripts?|write|writing|written|author|reader|readers)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data97() -> Py { py_tuple(vec![py_str("poem"), py_str("poems"), py_str("song"), py_str("lyrics"), py_str("rap"), py_str("speech"), py_str("essay"), py_str("email"), py_str("cover letter"), py_str("cv"), py_str("letter"), py_str("journal"), py_str("diary")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("story-first-line")); __args.push(py_tuple(vec![py_tuple(vec![py_str("opening line"), py_str("opening lines"), py_str("first line"), py_str("first lines"), py_str("first sentence"), py_str("opening sentence"), py_str("hook"), py_str("hooks"), py_str("opening")]), py_tuple(vec![py_str("book"), py_str("story"), py_str("novel"), py_str("chapter"), py_str("reader"), py_str("readers"), py_str("good"), py_str("best"), py_str("great")])])); __args.push(py_str("Start with a voice or a fact, not the weather: \"My mother stole a car on her way to my christening\" and \"The rent was due on Friday and I had 40p\" both make a reader ask a question. One sentence, something happening or someone talking, and no throat-clearing before it. Write ten, throw eight away - the tenth is usually the one you keep.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("whats"), py_str("what's"), py_str("how"), py_str("good"), py_str("best"), py_str("any"), py_str("suggest"), py_str("give"), py_str("need")])), ("avoid".to_string(), op_add(&__data97(), &g_PERSONAL_NEWS())), ("guard".to_string(), py_func("_an_opening_is_written_not_asked", |args: &[Py]| _an_opening_is_written_not_asked(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("describe-a-character")); __args.push(py_tuple(vec![py_tuple(vec![py_str("describe"), py_str("describing"), py_str("description"), py_str("describe a character"), py_str("character description"), py_str("character"), py_str("characters")]), py_tuple(vec![py_str("character"), py_str("characters"), py_str("person"), py_str("people"), py_str("book"), py_str("story"), py_str("novel"), py_str("dialogue")])])); __args.push(py_str("Give one thing they do and one thing they own before you describe their face: a woman who counts the stairs as she climbs them and keeps her bus tickets in a tin is a person, and \"tall, with brown hair\" is a form. Two details, one of them slightly odd, and let the reader do the rest; people remember what a character does far longer than how they look.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("any"), py_str("should"), py_str("tips"), py_str("good")])), ("avoid".to_string(), op_add(&py_tuple(vec![py_str("photo"), py_str("picture"), py_str("appearance"), py_str("haircut"), py_str("outfit"), py_str("dating profile"), py_str("cv"), py_str("personal statement"), py_str("job interview")]), &g_PERSONAL_NEWS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data98() -> Py { py_tuple(vec![py_str("my partner"), py_str("my husband"), py_str("my wife"), py_str("my boyfriend"), py_str("my girlfriend"), py_str("argument"), py_str("argue"), py_str("arguing"), py_str("row with"), py_str("he hit"), py_str("she hit"), py_str("hit me"), py_str("police"), py_str("abuse")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-fight-scene")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fight scene"), py_str("fight scenes"), py_str("action scene"), py_str("battle scene"), py_str("chase scene"), py_str("write a fight"), py_str("fighting scene")]), py_tuple(vec![py_str("scene"), py_str("scenes"), py_str("better"), py_str("describe"), py_str("good"), py_str("punchy"), py_str("fast")])])); __args.push(py_str("Keep the sentences short and the geography simple: who is where, what is in reach, and what each of them wants in the next ten seconds. Two or three blows that land badly, one thing that goes wrong - a foot in a puddle, a hand that will not close - and a cost at the end, because a fight nobody loses anything in reads like a dance.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("any"), py_str("should"), py_str("help"), py_str("tips")])), ("avoid".to_string(), op_add(&__data98(), &g_PERSONAL_NEWS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.A_COUNTED_MINUTE"); }
    s_A_COUNTED_MINUTE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|couple|few|half)(?:\\s+(?:of|more|whole|short|quick|extra|good))?\\s+(?:minutes|minute|mins|min)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language.EVERY_MINUTE"); }
    s_EVERY_MINUTE(stdlib_call("re", "compile", &[py_str("\\b(?:minutes|minute|mins|min)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meeting-minutes")); __args.push(py_tuple(vec![py_tuple(vec![py_str("minutes"), py_str("minute taking"), py_str("take minutes"), py_str("taking minutes"), py_str("meeting notes"), py_str("note taking")]), py_tuple(vec![py_str("meeting"), py_str("meetings"), py_str("notes"), py_str("work"), py_str("actions"), py_str("decisions")])])); __args.push(py_str("Three parts and nothing else: what was decided, who is doing what by when, and what was left open. Write the decisions as they happen rather than from memory afterwards, put a name and a date against every action, and send them out the same day - minutes nobody reads by Friday are a diary.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("should"), py_str("write"), py_str("take"), py_str("any")])), ("avoid".to_string(), op_add(&py_tuple(vec![py_str("minutes walk"), py_str("minutes drive"), py_str("in minutes"), py_str("for 10"), py_str("for 20")]), &g_PERSONAL_NEWS())), ("guard".to_string(), py_func("_minutes_are_a_length_of_time", |args: &[Py]| _minutes_are_a_length_of_time(args))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-work-report")); __args.push(py_tuple(vec![py_tuple(vec![py_str("report"), py_str("reports"), py_str("write-up"), py_str("write up"), py_str("writeup")]), py_tuple(vec![py_str("work"), py_str("office"), py_str("weekly"), py_str("monthly"), py_str("quarterly"), py_str("progress"), py_str("numbers"), py_str("structure")])])); __args.push(py_str("Answer the question the report exists for in the first two lines, then the evidence: what you did, what it showed, what you want to happen next. Short sections with headings, one number or example per claim, and a line at the end saying what you need from the reader. Anybody who reads only the first paragraph should still know the answer.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("should"), py_str("write"), py_str("any"), py_str("help")])), ("avoid".to_string(), op_add(&py_tuple(vec![py_str("school report"), py_str("medical report"), py_str("police report"), py_str("accident report"), py_str("annual report"), py_str("tax return"), py_str("self assessment"), py_str("sick note"), py_str("safeguarding"), py_str("raise it"), py_str("speak to my boss"), py_str("talk to my boss")]), &g_PERSONAL_NEWS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wedding-reading")); __args.push(py_tuple(vec![py_tuple(vec![py_str("reading"), py_str("readings"), py_str("passage"), py_str("passages"), py_str("what to read"), py_str("poem to read"), py_str("something to read")]), py_tuple(vec![py_str("wedding"), py_str("ceremony"), py_str("church"), py_str("registry office"), py_str("read out"), py_str("choose")])])); __args.push(py_str("Pick by length and tone rather than by fame: 60 to 90 seconds, one idea, and nothing that makes the couple laugh at each other. A poem works if it is plain and short (Auden's \"Funeral Blues\" is a funeral poem - be careful), and a passage from a book the two of them love beats a sonnet nobody understands. Read it aloud twice before the day, and tell the registrar what you are reading.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("which"), py_str("how"), py_str("any"), py_str("best"), py_str("good"), py_str("should"), py_str("suggest")])), ("avoid".to_string(), op_add(&py_tuple(vec![py_str("card"), py_str("cards"), py_str("message"), py_str("note"), py_str("wishes"), py_str("vows"), py_str("write my own"), py_str("tarot"), py_str("reading a book"), py_str("reading list"), py_str("reading to my")]), &g_PERSONAL_NEWS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data99() -> Py { py_tuple(vec![py_str("wedding"), py_str("bride"), py_str("groom"), py_str("best man"), py_str("toast"), py_str("eulogy"), py_str("funeral"), py_str("birthday"), py_str("retirement"), py_str("leaving do"), py_str("speech therapy"), py_str("speech delay"), py_str("speech and language")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("school-speech")); __args.push(py_tuple(vec![py_tuple(vec![py_str("speech"), py_str("presentation"), py_str("assembly"), py_str("speak in assembly")]), py_tuple(vec![py_str("school"), py_str("class"), py_str("gcse"), py_str("students"), py_str("year 11"), py_str("leavers"), py_str("leaving assembly"), py_str("head boy"), py_str("head girl"), py_str("teacher"), py_str("teachers"), py_str("assembly")])])); __args.push(py_str("Open with a story about one day at that school, not with a thank-you list, and keep the whole thing to three minutes: the story, what it taught you, then the one sentence you want people to remember. Say it out loud standing up four times before the day, and put the last line on a card in your hand - a rehearsed ending is what makes a speech land.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("should"), py_str("write"), py_str("any"), py_str("help")])), ("avoid".to_string(), op_add(&__data99(), &g_PERSONAL_NEWS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data100() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pup"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("rabbit"), py_str("bunny"), py_str("horse"), py_str("pony"), py_str("parrot"), py_str("hamster"), py_str("guinea pig"), py_str("tortoise"), py_str("pets"), py_str("pet")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PET_PHOTO"); }
    s__PET_PHOTO(__data100());
    fn __data101() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("bread"), py_str("loaf"), py_str("dinner"), py_str("lunch"), py_str("breakfast"), py_str("soup"), py_str("pie"), py_str("biscuits"), py_str("cookies"), py_str("curry"), py_str("roast"), py_str("pancakes"), py_str("sourdough"), py_str("pizza"), py_str("pasta"), py_str("barbecue"), py_str("bbq"), py_str("coffee"), py_str("chocolate")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._FOOD_PHOTO"); }
    s__FOOD_PHOTO(__data101());
    fn __data102() -> Py { py_tuple(vec![py_str("sea"), py_str("beach"), py_str("mountain"), py_str("mountains"), py_str("hill"), py_str("hills"), py_str("sunset"), py_str("sunrise"), py_str("sky"), py_str("lake"), py_str("river"), py_str("city"), py_str("street"), py_str("forest"), py_str("snow"), py_str("holiday"), py_str("view"), py_str("garden")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLACE_PHOTO"); }
    s__PLACE_PHOTO(__data102());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PHOTO_DETAIL"); }
    s__PHOTO_DETAIL(py_tuple(vec![py_tuple(vec![g__PET_PHOTO(), py_str("and that look it gives you just before it decides not to listen")]), py_tuple(vec![g__FOOD_PHOTO(), py_str("and it did not last the afternoon")]), py_tuple(vec![g__PLACE_PHOTO(), py_str("and nobody else was awake yet")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PHOTO_DETAIL_PEOPLE"); }
    s__PHOTO_DETAIL_PEOPLE(py_str("and the hour before anybody started posing"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PHOTO_DETAIL_ANY"); }
    s__PHOTO_DETAIL_ANY(py_str("and the part of the day nobody photographs"));
    fn __data103() -> Py { py_tuple(vec![py_str("me"), py_str("us"), py_str("my mum"), py_str("my mom"), py_str("my dad"), py_str("my sister"), py_str("my brother"), py_str("my friend"), py_str("my friends"), py_str("family"), py_str("the kids"), py_str("my son"), py_str("my daughter"), py_str("my wife"), py_str("my husband"), py_str("my partner"), py_str("my nan"), py_str("my grandma"), py_str("my grandad")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PEOPLE_PHOTO"); }
    s__PEOPLE_PHOTO(__data103());
    fn __data104() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("write"), py_str("writing"), py_str("what"), py_str("whats"), py_str("should i"), py_str("help"), py_str("good"), py_str("photo"), py_str("picture"), py_str("post"), py_str("instagram"), py_str("insta"), py_str("for")]) }
    fn __data105() -> Py { py_tuple(vec![py_str("subtitle"), py_str("subtitles"), py_str("closed captions"), py_str("tv"), py_str("television"), py_str("film"), py_str("movie"), py_str("youtube"), py_str("netflix"), py_str("cancel"), py_str("subscription"), py_str("delete my account"), py_str("edit a photo"), py_str("photo editor"), py_str("how do i post"), py_str("instagram account"), py_str("hacked")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-caption")); __args.push(py_tuple(vec![py_tuple(vec![py_str("caption"), py_str("captions"), py_str("caption my"), py_str("write a caption"), py_str("insta caption"), py_str("instagram caption"), py_str("photo caption"), py_str("picture caption")]), __data104()])); __args.push(py_func("_a_caption", |args: &[Py]| _a_caption(args))); __kws.extend(vec![("avoid".to_string(), __data105()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data106() -> Py { py_tuple(vec![py_str("say something nice"), py_str("say something nice about"), py_str("nice about my"), py_str("something nice about"), py_str("compliment about my"), py_str("compliment for my"), py_str("something nice to say"), py_str("a nice thing to say"), py_str("nice thing to say"), py_str("kind thing to say"), py_str("something kind to say"), py_str("a compliment"), py_str("some compliments"), py_str("compliments for"), py_str("compliment for"), py_str("compliment about"), py_str("compliment to"), py_str("pay a compliment"), py_str("give a compliment"), py_str("nice things to say"), py_str("what should i say to"), py_str("something good to say")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._KIND_ASKS"); }
    s__KIND_ASKS(__data106());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._KIND_SUBJECTS"); }
    s__KIND_SUBJECTS(py_tuple(vec![py_str("about"), py_str("for"), py_str("to"), py_str("on")]));
    fn __data107() -> Py { py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pup"), py_str("rabbit"), py_str("bunny"), py_str("hamster"), py_str("horse"), py_str("pony"), py_str("parrot"), py_str("bird"), py_str("tortoise"), py_str("pet"), py_str("pets"), py_str("guinea pig"), py_str("ferret"), py_str("donkey"), py_str("goat"), py_str("snake"), py_str("lizard"), py_str("fish")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._ANIMAL_KIND"); }
    s__ANIMAL_KIND(__data107());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._STRANGER_KIND"); }
    s__STRANGER_KIND(py_tuple(vec![py_str("stranger"), py_str("strangers"), py_str("someone i don't know"), py_str("someone i dont know"), py_str("somebody i don't know"), py_str("a stranger"), py_str("people i don't know"), py_str("new person"), py_str("someone new"), py_str("somebody new"), py_str("a customer"), py_str("a colleague i just met")]));
    fn __data108() -> Py { py_tuple(vec![py_str("cake"), py_str("bake"), py_str("baking"), py_str("bread"), py_str("sourdough"), py_str("loaf"), py_str("pie"), py_str("biscuits"), py_str("cookies"), py_str("dinner"), py_str("cooking"), py_str("meal"), py_str("roast"), py_str("jam"), py_str("curry"), py_str("casserole"), py_str("barbecue"), py_str("bbq"), py_str("pizza"), py_str("brownies")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._BAKE_KIND"); }
    s__BAKE_KIND(__data108());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._LOOKS_KIND"); }
    s__LOOKS_KIND(py_tuple(vec![py_str("haircut"), py_str("hair"), py_str("beard"), py_str("outfit"), py_str("dress"), py_str("coat"), py_str("glasses"), py_str("quilt"), py_str("jumper"), py_str("nails")]));
    fn __data109() -> Py { py_tuple(vec![py_str("garden"), py_str("kitchen"), py_str("room"), py_str("shed"), py_str("allotment"), py_str("flowers"), py_str("plant"), py_str("plants"), py_str("house"), py_str("car"), py_str("flat"), py_str("bathroom"), py_str("painting"), py_str("drawing"), py_str("kitchen floor")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PLACE_KIND"); }
    s__PLACE_KIND(__data109());
    fn __data110() -> Py { py_tuple(vec![py_str("wife"), py_str("husband"), py_str("partner"), py_str("girlfriend"), py_str("boyfriend"), py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("son"), py_str("daughter"), py_str("child"), py_str("kid"), py_str("sister"), py_str("brother"), py_str("friend"), py_str("mate"), py_str("colleague"), py_str("coworker"), py_str("boss"), py_str("neighbour"), py_str("neighbor"), py_str("teacher"), py_str("grandma"), py_str("grandmother"), py_str("grandad"), py_str("grandfather"), py_str("nan"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("person"), py_str("someone"), py_str("somebody"), py_str("man"), py_str("woman"), py_str("carer"), py_str("nurse"), py_str("gp"), py_str("doctor")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._PEOPLE_KIND"); }
    s__PEOPLE_KIND(__data110());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._A_KIND_WORD"); }
    s__A_KIND_WORD(py_dict(vec![(py_str("animal"), py_str("Go on the animal and not the owner: \"He doesn't just ignore you, he audibly ignores you.\" One true thing you have watched it do beats the word cute every time, because it proves you were paying attention.")), (py_str("person"), py_str("Name the thing they do that nobody remarks on and say it straight: \"You're the one who keeps this group actually going.\" The specific thing is the compliment; \"you're great\" is a placeholder and they can tell.")), (py_str("stranger"), py_str("Pick something they chose rather than something they were born with - the coat, the dog, the way they handled the queue - and say it in five words as you go: \"That coat is a very good coat.\" No follow-up needed, so it does not turn into a conversation they did not ask for.")), (py_str("bake"), py_str("Praise the thing they made rather than the person making it: \"That crust has a proper crackle to it.\" They know how much work went into it, so the detail is what lands; \"it's lovely\" is what people say when they did not look.")), (py_str("looks"), py_str("Praise the detail you can see, not the thing itself: \"That cut makes your jaw look sharp,\" not \"nice haircut.\" Say it once, plainly, and let it stand - the detail is the proof you looked.")), (py_str("place"), py_str("Praise the work you can see rather than the whole thing: \"That border is doing a lot of quiet work.\" The one detail you noticed is the compliment; \"it looks lovely\" is a placeholder and they can tell.")), (py_str("someone"), py_str("Say the one thing you actually noticed and stop: \"You always remember how everyone takes their tea.\" Praise that names a thing they did is a compliment; praise with no subject in it is noise."))]));
    fn __data111() -> Py { py_tuple(vec![py_str("rent"), py_str("rents"), py_str("rent rise"), py_str("rent increase"), py_str("money"), py_str("pay"), py_str("wages"), py_str("overtime"), py_str("hours"), py_str("shifts"), py_str("rota"), py_str("workload"), py_str("noise"), py_str("noisy"), py_str("mess"), py_str("messy"), py_str("rubbish"), py_str("bins"), py_str("bin"), py_str("damp"), py_str("mould"), py_str("mold"), py_str("repairs"), py_str("repair"), py_str("deposit"), py_str("bill"), py_str("bills"), py_str("arrears"), py_str("debt"), py_str("debts"), py_str("behaviour"), py_str("behavior"), py_str("attitude"), py_str("manners"), py_str("deadline"), py_str("deadlines"), py_str("lateness"), py_str("chores"), py_str("boundaries"), py_str("smoking"), py_str("drinking")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._A_MATTER_TO_RAISE"); }
    s__A_MATTER_TO_RAISE(__data111());
    fn __data112() -> Py { py_tuple(vec![py_str("say"), py_str("something"), py_str("nice"), py_str("kind"), py_str("compliment"), py_str("compliments"), py_str("what"), py_str("should"), py_str("give"), py_str("me"), py_str("can"), py_str("i"), py_str("you")]) }
    fn __data113() -> Py { py_tuple(vec![py_str("complement"), py_str("complements"), py_str("complementary"), py_str("complimentary"), py_str("paint"), py_str("colour"), py_str("color"), py_str("scheme"), py_str("spell"), py_str("spelling"), py_str("spelt"), py_str("mean"), py_str("means"), py_str("meaning"), py_str("difference between"), py_str("well done"), py_str("appraisal"), py_str("performance review"), py_str("feedback"), py_str("notice period"), py_str("resign"), py_str("vaccine"), py_str("insurance"), py_str("complimentary breakfast")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-kind-word")); __args.push(py_tuple(vec![g__KIND_ASKS(), op_add(&g__KIND_SUBJECTS(), &__data112())])); __args.push(py_func("_a_kind_word", |args: &[Py]| _a_kind_word(args))); __kws.extend(vec![("avoid".to_string(), __data113()), ("guard".to_string(), py_func("_the_matter_is_worth_praise", |args: &[Py]| _the_matter_is_worth_praise(args))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-blog-post")); __args.push(py_tuple(vec![py_tuple(vec![py_str("blog post"), py_str("blog"), py_str("blogging"), py_str("article"), py_str("post about"), py_str("write a post"), py_str("blog page")]), py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("write"), py_str("writing"), py_str("start"), py_str("starting"), py_str("good"), py_str("first"), py_str("help"), py_str("what should"), py_str("structure"), py_str("long")])])); __args.push(py_str("Open with the thing you found out, not with the background: \"Cheap ovens lie about their temperature\" gets read, \"Many people own ovens\" does not. One idea per post, 600 to 900 words, subheadings every few paragraphs, and end by telling the reader what to do next.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("blogger"), py_str("blog template"), py_str("wordpress"), py_str("squarespace"), py_str("make money"), py_str("monetise"), py_str("get traffic"), py_str("seo"), py_str("domain name"), py_str("hosting")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data114() -> Py { py_tuple(vec![py_str("about me page"), py_str("about me section"), py_str("about me blurb"), py_str("write an about me"), py_str("writing an about me"), py_str("my about me"), py_str("about page"), py_str("about us page"), py_str("about us section"), py_str("about us blurb"), py_str("write an about us"), py_str("our about us"), py_str("bio"), py_str("bios"), py_str("my bio"), py_str("author bio"), py_str("profile bio"), py_str("biography for"), py_str("short bio"), py_str("one line bio")]) }
    fn __data115() -> Py { py_tuple(vec![py_str("tinder"), py_str("dating"), py_str("hinge"), py_str("bumble"), py_str("cv"), py_str("resume"), py_str("cover letter"), py_str("instagram"), py_str("tiktok"), py_str("twitter"), py_str("linkedin headline"), py_str("job application"), py_str("remember"), py_str("remembering"), py_str("memorise"), py_str("memorize")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("about-me-page")); __args.push(py_tuple(vec![__data114(), py_tuple(vec![py_str("write"), py_str("writing"), py_str("how do i"), py_str("what should"), py_str("what do i put"), py_str("put in"), py_str("include"), py_str("start"), py_str("say"), py_str("good"), py_str("better"), py_str("help")])])); __args.push(py_str("Five lines in the first person: what you do, who you do it for, one proof - a project, a number, a place you worked - and how to reach you. Cut the adjectives at the top, because \"passionate about stories\" tells a reader nothing, and write the sentence you would say if someone asked at a party what you do.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how"), py_str("what"), py_str("should"), py_str("which"), py_str("any"), py_str("can")])), ("avoid".to_string(), __data115()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-a-product-review")); __args.push(py_tuple(vec![py_tuple(vec![py_str("product review"), py_str("amazon review"), py_str("review a product"), py_str("review on amazon"), py_str("leave a review"), py_str("write a review of"), py_str("one star review")]), py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("write"), py_str("writing"), py_str("good"), py_str("help"), py_str("what should"), py_str("bad"), py_str("negative"), py_str("complaint")])])); __args.push(py_str("Say what you bought, how long you have used it and what it did well, then the one thing that went wrong, with a fact: \"the handle bent on the fourth use\". Two lines of that beats a star rating, and a fair review is far harder for a shop to dismiss than an angry one. Put it where people look and the business will see it - the order page for an online shop, Google Maps for a cafe or restaurant, TripAdvisor for a hotel - and keep the order number and any photos.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("book review"), py_str("book"), py_str("novel"), py_str("film review"), py_str("movie"), py_str("restaurant review"), py_str("how do i get"), py_str("followers"), py_str("review my cv")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data116() -> Py { py_tuple(vec![py_str("better"), py_str("improve"), py_str("nervous"), py_str("nerves"), py_str("confident"), py_str("confidence"), py_str("afraid"), py_str("scared"), py_str("how do i"), py_str("how to"), py_str("practice"), py_str("practise"), py_str("help"), py_str("calm")]) }
    fn __data117() -> Py { py_tuple(vec![py_str("wedding"), py_str("best man"), py_str("bridesmaid"), py_str("eulogy"), py_str("funeral"), py_str("toast"), py_str("write"), py_str("draft"), py_str("opening line"), py_str("joke"), py_str("a present"), py_str("the present"), py_str("presents for"), py_str("present for"), py_str("a gift"), py_str("the gift"), py_str("gifts"), py_str("cost of a present"), py_str("cost of the present"), py_str("split the cost of"), py_str("what to give"), py_str("what to buy"), py_str("what to get"), py_str("wrap"), py_str("wrapping")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("public-speaking-nerves")); __args.push(py_tuple(vec![py_tuple(vec![py_str("public speaking"), py_str("presentation"), py_str("presenting"), py_str("speak in front of"), py_str("speaking in public"), py_str("stage fright"), py_str("talking in front of people"), py_str("speak to a room"), py_str("webinar"), py_str("viva")]), __data116()])); __args.push(py_str("Rehearse the first sixty seconds out loud until you could say it half asleep, because that is the only part your nerves will attack. Then slow down and pause at the ends of sentences rather than filling the silence - a room reads a pause as confidence, and it gives you time to breathe.")); __kws.extend(vec![("avoid".to_string(), __data117()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data118() -> Py { py_tuple(vec![py_str("mark"), py_str("marks"), py_str("symbol"), py_str("symbols"), py_str("sign"), py_str("signs"), py_str("character"), py_str("characters"), py_str("line"), py_str("lines"), py_str("squiggle"), py_str("squiggles"), py_str("punctuation"), py_str("key"), py_str("keys"), py_str("keyboard")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._MARK_WORDS"); }
    s__MARK_WORDS(__data118());
    fn __data119() -> Py { py_tuple(vec![py_str("called"), py_str("name"), py_str("do you call"), py_str("which key"), py_str("what key"), py_str("what is this"), py_str("what is that"), py_str("whats this"), py_str("whats that"), py_str("what does this"), py_str("what does that"), py_str("what is the #"), py_str("what is the *"), py_str("what is the @"), py_str("what is the ~"), py_str("what is the _"), py_str("what is the |")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._MARK_ASK"); }
    s__MARK_ASK(__data119());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.language._MARK_BY_LOOK"); }
    s__MARK_BY_LOOK(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("wavy"), py_str("wave"), py_str("squiggle"), py_str("squiggly"), py_str("tilde"), py_str("squiggles")]), py_str("That is a tilde, ~. It means 'about' (~20 people came), and on most keyboards it is Shift and the hash key.")]), py_tuple(vec![py_tuple(vec![py_str("low line"), py_str("low dash"), py_str("underline"), py_str("under the letter"), py_str("under a letter"), py_str("between words")]), py_str("That is an underscore, _. It joins words in file names and usernames (my_notes.txt); a hyphen is the shorter mark at half height.")]), py_tuple(vec![py_tuple(vec![py_str("upright"), py_str("vertical"), py_str("tall line"), py_str("straight line")]), py_str("That is a pipe, |, also called the vertical bar. It separates options in a computer command; in writing a slash does the job better.")]), py_tuple(vec![py_tuple(vec![py_str("star"), py_str("little star"), py_str("star shaped")]), py_str("That is an asterisk, *. It marks a footnote, and on a phone keyboard it hides behind the ?123 key.")]), py_tuple(vec![py_tuple(vec![py_str("wedge"), py_str("little hat"), py_str("roof"), py_str("pointing up"), py_str("arrow head")]), py_str("That is a caret, ^. It marks where a word is to be slipped in, and in maths it means to the power of - 2^3 is eight.")]), py_tuple(vec![py_tuple(vec![py_str("backwards"), py_str("other way"), py_str("wrong way")]), py_str("The one leaning the other way is a backslash, \\. It separates folders in a Windows file path like C:\\Users\\me, and it almost never belongs in writing.")]), py_tuple(vec![py_tuple(vec![py_str("over the 2"), py_str("above the 2"), py_str("at sign"), py_str("a with a circle")]), py_str("That is the at sign, @ - Shift and 2 on a UK keyboard, and it is in every email address.")]), py_tuple(vec![py_tuple(vec![py_str("over the 3"), py_str("above the 3"), py_str("hash"), py_str("pound"), py_str("number sign")]), py_str("That is the hash, # - Shift and 3 on a UK keyboard. On a phone it starts a hashtag, and in the US it labels a number.")]), py_tuple(vec![py_tuple(vec![py_str("dot"), py_str("dots"), py_str("three dots"), py_str("full stop"), py_str("period")]), py_str("Three dots in a row are an ellipsis (...), and it means something has been left out or the thought trails off. A single dot is a full stop.")]), py_tuple(vec![py_tuple(vec![py_str("squarish"), py_str("square"), py_str("box")]), py_str("Square brackets are [ ], and they hold your own words inside a quotation or a gap in a transcript.")]), py_tuple(vec![py_tuple(vec![py_str("curly"), py_str("squiggly brackets"), py_str("brace"), py_str("braces")]), py_str("Curly brackets are { }, and they group things in maths and in code. Round brackets ( ) take an aside the sentence could do without.")]), py_tuple(vec![py_tuple(vec![py_str("sideways v"), py_str("pointy"), py_str("arrow brackets")]), py_str("Angle brackets are < >. They mark a tag in code and stand in for a word to fill in later, as in <your name>.")])]));
    fn __data120() -> Py { py_tuple(vec![py_str("cancer"), py_str("mole"), py_str("moles"), py_str("skin"), py_str("symptom"), py_str("symptoms"), py_str("stroke"), py_str("infection"), py_str("rash"), py_str("label"), py_str("labels"), py_str("care label"), py_str("washing"), py_str("laundry"), py_str("tub"), py_str("instruction"), py_str("instructions"), py_str("clothes"), py_str("tag"), py_str("tags"), py_str("road"), py_str("zodiac"), py_str("star sign"), py_str("birth"), py_str("period pain"), py_str("periods"), py_str("arm"), py_str("arms"), py_str("leg"), py_str("legs"), py_str("back"), py_str("face"), py_str("bruise"), py_str("bruises"), py_str("spot"), py_str("spots"), py_str("lump"), py_str("lumps"), py_str("mark on my"), py_str("marks on my"), py_str("birthmark"), py_str("freckle"), py_str("freckles"), py_str("scar"), py_str("scars"), py_str("dementia"), py_str("alzheimers"), py_str("alzheimer's"), py_str("memory"), py_str("memories"), py_str("forget"), py_str("forgets"), py_str("forgetting"), py_str("forgetful"), py_str("forgot"), py_str("remember"), py_str("remembering")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mark-unnamed")); __args.push(py_tuple(vec![g__MARK_WORDS(), g__MARK_ASK()])); __args.push(py_func("_mark_unnamed", |args: &[Py]| _mark_unnamed(args))); __kws.extend(vec![("avoid".to_string(), __data120()), ("ask".to_string(), g__MARK_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _apology(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    words = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("customer"), py_str("client"), py_str("company"), py_str("order"), py_str("refund"), py_str("grovelling"), py_str("professional"), py_str("boss"), py_str("manager"), py_str("colleague"), py_str("work"), py_str("team"), py_str("staff"), py_str("late")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Say what you did and what you'll fix, in that order, with no 'if': \"I'm sorry the order arrived late — I posted it on Tuesday and it should have been there on Thursday. I've refunded the delivery charge.\" Then one line on what changes, and leave out the 'but'.");
    }
    return py_str("Say what you did, not 'if you were upset': \"I'm sorry I forgot your birthday — I had it in my head as next week and that's no excuse.\" Then say what you'll do about it, and leave out the 'but'.");
    Py::None
}
pub fn _fewer(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    words = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("money"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("time"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("water"))) } } } }) {
    return py_str("Use 'less' for things you can't count: less time, less money, less water. Use 'fewer' for things you can: fewer people, fewer cars, fewer mistakes. 'Less people' is the one people notice.");
    }
    return py_str("Use 'fewer' for things you can count and 'less' for things you can't: fewer people, fewer mistakes, but less time and less money. 'Fewer people' is right; 'less people' is the mistake everyone notices.");
    Py::None
}
pub fn _figure_of_speech(args: &[Py]) -> Py {
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
    let mut k: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text"), py_str("line")])) { let mut k: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[k.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g__FIGURES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    return py_str("Irony says the opposite of what is meant (a fire station burning down); onomatopoeia is a word that sounds like the thing (splash, buzz); alliteration repeats the opening sound (slippery slope); hyperbole exaggerates on purpose (a million times). Ask me about one and I'll give you its example.");
    Py::None
}
pub fn _dash_is_the_mark(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line means the punctuation mark, not a dash of a liquid.");
    said = py_str("");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("plain"), py_str("clean"), py_str("text")]));
    for __item in __iter1 {
    name = __item.clone();
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[name.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&said.clone()) {
    break;
    }
    }
    return Py::Bool(!py_truthy(&mcall(&g_A_DASH_OF_SOMETHING(), "search", &[mcall(&said.clone(), "lower", &[])])));
    Py::None
}
pub fn _dash_or_hyphen(args: &[Py]) -> Py {
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
    let mut look: Py = Py::None;
    let mut looks: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The dash the line names, or the rule that tells the two apart.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g_DASH_LOOKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    looks = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&{ let __b = _mentions(&[text.clone(), looks.clone()]); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&looks.clone()) { let mut look: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&look.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &look.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return reply.clone();
    }
    }
    return py_str("A hyphen joins words into one (well-known, two-year-old) and has no spaces. A dash separates a phrase for emphasis — like this — and British style usually spaces an en dash. Word processors can turn two hyphens into a dash for you.");
    Py::None
}
pub fn _comma_rules(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("The general rule, or the however one when that is the word the line names.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("however")))) {
    return py_str("Put the comma after however when it opens a sentence or a clause: 'However, the train was late.' It goes before it in the middle of one: 'the plan was fine, however late.' With two full sentences, a semicolon before and a comma after is tidier: 'I wanted to go; however, I was ill.'");
    }
    return py_str("A comma marks a pause the sentence needs: between the items in a list (bread, milk, eggs), before and, but or which when a whole new clause follows, and around a name or an aside (my brother, who lives in Leeds, is coming). Read it aloud - if you would take a breath there, a comma usually belongs; if both halves could stand alone as sentences, use a full stop or a semicolon instead.");
    Py::None
}
pub fn _end_mark(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("Which mark ends the sentence the line asks about.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("question mark"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("questions mark"))) } }) {
    return py_str("A question mark closes a direct question: 'where are you going?' Not an indirect one - 'she asked where I was going' keeps a full stop. One mark is enough, so never '?!' more than once in a piece of writing.");
    }
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("exclamation")))) {
    return py_str("An exclamation mark is for a real exclamation - surprise, shouting, a warning - and one per page is plenty. Use it instead of a full stop at the end of the sentence, never in the middle, and never more than one in a row.");
    }
    return py_str("A full stop ends a statement, and every sentence needs one even when the next word carries on the thought. Use one rather than a comma where both halves could stand alone, and never put a space before it. A question takes a question mark and a real exclamation an exclamation mark.");
    Py::None
}
pub fn _speak_clearly(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("The filler, the mumble or the nerves: the one that is in the line.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&_mentions(&[text.clone(), py_tuple(vec![py_str("um"), py_str("ums"), py_str("erm"), py_str("er"), py_str("filler"), py_str("fillers"), py_str("like"), py_str("y'know")])])) {
    return py_str("At the moment you feel an 'um' coming, close your mouth and breathe instead: a half-second of silence sounds calm and no one notices it, where a filler sounds like doubt. Record thirty seconds of yourself on your phone and count the fillers; most people halve them in a week just by hearing them once.");
    }
    if py_truthy(&_mentions(&[text.clone(), py_tuple(vec![py_str("confident"), py_str("confidence"), py_str("confidently"), py_str("nervous"), py_str("nerves"), py_str("shaking"), py_str("shaky"), py_str("wobble"), py_str("wobbly"), py_str("wavers")])])) {
    return py_str("Slow down and pause at the full stops rather than at the commas, and look at one person for a whole sentence instead of sweeping the room - both read as confidence even when you are nervous. Write the first thirty seconds word for word and say it aloud five times, because getting started is the part nerves spoil.");
    }
    return py_str("Slow to about two-thirds of your usual speed and finish the ends of words, which is most of what 'clear' means. Pause at the full stops instead of running the sentences together, drop your voice a little at the end of a sentence rather than letting it rise, and if you are on a call, stand up - it makes the voice carry.");
    Py::None
}
pub fn _mentions(args: &[Py]) -> Py {
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
    let mut form: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A word, with its plural and its -ing allowed: the line says the thing, not the spelling.");
    said = stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![word.clone(), op_add(&word.clone(), &py_str("s")), op_add(&word.clone(), &py_str("es")), op_add(&word.clone(), &py_str("ing")), op_add(&word.clone(), &py_str("ed"))]));
    for __item in __iter2 {
    form = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &form.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_what_a_word_means(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is asking what a word or saying means, not using it.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    clean = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = clean.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__A_MEANING_IS_ASKED(), "search", &[clean.clone()]) } }) {
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return Py::Bool(false);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__WHAT_IS(), &geti(&words.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__FILLER(), &geti(&words.clone(), &Py::Int(0)))) } } } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    return { let __b = Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_EVERYDAY_WORDS(), &mcall(&py_str(" "), "join", &[words.clone()]))) } };
    Py::None
}
pub fn _same_word(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut ending: Py = Py::None;
    let _ = py_str("The table's word and the word the line said, allowing a regular ending.\n\n    The key is written once and people say it in any form: the row is keyed on \"boundaries\" and\n    the line says \"boundary\", which the scoring already reads as the same word, so the reply has\n    to as well - otherwise the row is chosen and then has nothing to say, and the person got the\n    fallback (\"I can take a single word apart...\") about a word the table holds. Only a regular\n    ending counts, and only where no key is the word itself: an exact key always wins, so the\n    table's own \"gaslighting\" is not read as \"gaslight\".\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("s"), py_str("d"), py_str("ed"), py_str("es"), py_str("ing"), py_str("ings")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&said.clone(), &op_add(&phrase.clone(), &ending.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&phrase.clone(), &op_add(&said.clone(), &ending.clone()))) } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = mcall(&phrase.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &op_add(&slice_or_raise(&phrase.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies")))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&said.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&phrase.clone(), &op_add(&slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies")))) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _word_meaning(args: &[Py]) -> Py {
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
    let mut tables: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tables") { v.clone() }
    else { py_tuple(vec![g_WORD_GLOSSES(), g_EVERYDAY_WORDS()]) };
    let mut flat: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut table: Py = Py::None;
    let mut word: Py = Py::None;
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    flat = mcall(&py_str(" "), "join", &[said.clone()]);
    let __iter1 = py_iter_or_raise(&tables.clone());
    for __item in __iter1 {
    table = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&table.clone(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk3[0].clone();
    reply = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&phrase.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat.clone(), &phrase.clone())) } } } }) {
    return reply.clone();
    }
    }
    }
    let __iter4 = py_iter_or_raise(&tables.clone());
    for __item in __iter4 {
    table = __item.clone();
    let __iter5 = py_iter_or_raise(&mcall(&table.clone(), "items", &[]));
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk6[0].clone();
    reply = __unpk6[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&phrase.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(_same_word(&[phrase.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return reply.clone();
    }
    }
    }
    return g_ASK_FOR_THE_WORD();
    Py::None
}
pub fn _an_everyday_word(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let _ = py_str("The everyday table's definition, or None when the table has not got the word.\n\n    The row's fence reads the shape of the line (\"what does X mean\", \"define X\"), and that shape\n    is the same for every word there is: \"what does gut mean\" and \"what does mean time mean\" are\n    that asks with words the table does not hold. Answering them with the sentence that asks for\n    a word hands back an ask for the very word the person just gave - and on main the first was\n    the bloating card and the second the honest hedge, so the row has to stand down and let the\n    other rows read the line as they did before.\n    ");
    reply = _word_meaning(&[memory.clone(), understood.clone(), py_tuple(vec![g_EVERYDAY_WORDS()])]);
    return if py_truthy(&Py::Bool(py_eq(&reply.clone(), &g_ASK_FOR_THE_WORD()))) { Py::None } else { reply.clone() };
    Py::None
}
pub fn _paragraph(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("What a paragraph is, or how long one is: the length ask wants the number first.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("how long"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("how many words"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("how much"))) } } } }) {
    return py_str("Three or four sentences, about 60 to 100 words, is a good paragraph: long enough to make one point and short enough to take in at a glance. Start a new one when you move to a new idea, and if one runs past a screenful, split it where the subject changes.");
    }
    return py_str("A paragraph is one idea in three or four sentences, about 60 to 100 words: long enough to make the point and short enough to take in at a glance. Start a new one when you move to a new idea, and if one runs past a screenful, split it where the subject changes.");
    Py::None
}
pub fn _late_names_the_work(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line's only sign of a deadline is the bare word \"late\".");
    text = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__EXT_LATE(), "search", &[text.clone()])))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__EXT_LATE_WORK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_deadline_extension(args: &[Py]) -> Py {
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
    line = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:too late|too old) to (?:get|be|become) (?:any )?(?:good|better|decent|anywhere)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__EXT_SUBJECTS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("said no"), py_str("says no"), py_str("refused"), py_str("refuses"), py_str("turned me down"), py_str("said i cant"), py_str("said i can't"), py_str("wouldnt give"), py_str("wouldn't give")])) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Ask again in writing, once, with the date and the evidence attached - a refusal at the desk is often the person, not the rule. Then go up one step: the module leader, then the extenuating-circumstances route, which is a form the university has to consider separately from the tutor's answer. Hand in the best version you have by the original date whatever happens, because a late mark beats a zero.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("missed"), py_str("miss"), py_str("was due"), py_str("is late"), py_str("in late"), py_str("already late"), py_str("past the deadline"), py_str("overdue"), py_str("didnt hand in"), py_str("didn't hand in"), py_str("nowhere near done"), py_str("not done"), py_str("ran out of time"), py_str("havent done"), py_str("haven't done")])) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Email the tutor today, before they mark it: the date you can deliver, what you have done so far, and the reason in one sentence. Universities cap late work rather than refusing it, so a complete essay handed in now usually beats a perfect one next month - and if the reason is illness or a bereavement, ask for the extenuating-circumstances form as well, because that can lift the cap.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("extra time"), py_str("access arrangements"), py_str("special arrangements"), py_str("dyslexia"), py_str("adhd"), py_str("extra 25"), py_str("rest breaks")])) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Extra time in an exam is arranged months in advance, not on the day: the exams office needs evidence - a diagnosis, or a history of extra time at school - and the deadline is usually early in the year. Go to the disability or study-support service with any report you have and ask what you qualify for; 25 per cent extra time and rest breaks are the common ones, and the application is theirs to make.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("extenuating"), py_str("mitigating"), py_str("ec form"), py_str("special circumstances")])) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Ask the school or module office for the extenuating-circumstances form the same week, and fill it in yourself: what happened, the dates it covered, and the evidence - a doctor's note, a hospital letter, a death notice. It goes to a panel rather than to your tutor, and it is the route that protects the mark or gives you another attempt; an extension only moves the date. Keep a copy, and note the deadline for the form, which is usually days after the assessment.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__EXT_ILL()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Ask for the extension in writing now, name what happened in one line, and attach the evidence you have - a doctor's note, a hospital letter. Then ask the office for the extenuating-circumstances form: that is the route that protects the mark itself, while an extension only moves the date. Do it the same week, because most deadlines for that form are days, not weeks, after the assessment.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("keep track"), py_str("track of"), py_str("track my"), py_str("organise"), py_str("organize"), py_str("planner"), py_str("diary"), py_str("reminder")])) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Put every deadline in one place the day you get it - a paper week-to-view diary or the calendar on your phone - and write each one a week early as well as on the day. Then work backwards: the day you must start, not the day it is due. Five minutes every Monday moving the next fortnight into view is what stops the surprise, and check the same page for the exam dates.");
    }
    return py_str("Ask before the deadline, say how much more time you need and what you'll deliver: \"I'm not going to do this justice by Friday. Could I have until Tuesday? I'd then send the finished draft rather than a thin one.\" A specific new date does the work — no long explanation of why you're behind.");
    Py::None
}
pub fn _brackets_are_punctuation(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line means the mark, not a sum, a band of money or a piece of metal.");
    said = py_str("");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("plain"), py_str("clean"), py_str("text")]));
    for __item in __iter1 {
    name = __item.clone();
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[name.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&said.clone()) {
    break;
    }
    }
    low = mcall(&said.clone(), "lower", &[]);
    if py_truthy(&mcall(&g__WRITING_BRACKETS(), "search", &[low.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g__NOT_A_PUNCTUATION_BRACKET(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__BUYING_BRACKETS(), "search", &[low.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g__NOT_MATHS_BRACKETS(), "search", &[low.clone()])));
    Py::None
}
pub fn _ellipsis_reply(args: &[Py]) -> Py {
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
    return py_str("An ellipsis is three dots (...) and it means something has been left out or left hanging: a sentence that trails off, a pause, or words cut from a quote. Use it once in a while, not as a habit, and in British style it closes up to the word before it: 'I was going to...'");
    Py::None
}
pub fn _speech_marks_reply(args: &[Py]) -> Py {
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
    return py_str("Speech goes in double quotation marks, and a quote inside a quote takes single ones: 'She said \"I am coming\" and left.' The full stop or comma goes inside the closing mark when it belongs to the speech, and every new speaker starts a new line. Do not close the marks and carry the same speech on after a comma.");
    Py::None
}
pub fn _list_reply(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str("bullet")))) {
    return py_str("A bulleted list needs the same lead-in for every line and no full stops if the lines are fragments: 'Bring:' then 'milk', 'eggs', 'bread'. Capitalise each line or none, and keep the grammar parallel - all nouns or all verbs, not a mix.");
    }
    return py_str("A list inside a sentence takes commas, and semicolons when the items have commas of their own: 'we need flour, which is in the cupboard; eggs, which are in the fridge; and milk.' A colon introduces the list, and never after a verb: 'the things I need are milk, eggs and bread', not 'are: milk'.");
    Py::None
}
pub fn _offers_two_of_the_family(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line holds two of those forms, as \"is it octopuses or octopi\" does.\n\n    A shape reads the line as it was typed, so this counts spellings rather than words of the\n    key: the question is which of the two to use, and one spelling on its own is not that\n    question - it is the plural row's, or this row's without a plural to choose.\n    ");
    return op_gt(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&line.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g__PLUS_FORMS(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn _asks_which_of_the_two(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line offers two forms and asks which: \"octopuses or octopi\".");
    return Py::Bool(py_contains(&_the_words_of(&[understood.clone()]), &py_str("or")));
    Py::None
}
pub fn _the_figure_that_was_named(args: &[Py]) -> Py {
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
    let mut k: Py = Py::None;
    let mut named_metaphor: Py = Py::None;
    let mut named_simile: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    reading = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text"), py_str("line"), py_str("norm_text")])) { let mut k: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[k.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    named_metaphor = Py::Bool(py_contains(&said.clone(), &py_str("metaphor")));
    named_simile = Py::Bool(py_contains(&said.clone(), &py_str("simile")));
    if py_truthy(&{ let __b = named_metaphor.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&named_simile.clone())) } }) {
    return geti(&g__THE_TWO_FIGURES(), &py_str("metaphor"));
    }
    if py_truthy(&{ let __b = named_simile.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&named_metaphor.clone())) } }) {
    return geti(&g__THE_TWO_FIGURES(), &py_str("simile"));
    }
    return py_str("A simile compares with like or as (her laugh was like a kettle), and a metaphor says it is the thing (her laugh was a kettle). Both make a picture; the metaphor just does it without the 'like'.");
    Py::None
}
pub fn _span_after_ask(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut index: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut size: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words the line is asking about: everything after \"another word for\" and its kin.");
    end = Py::Int(0);
    let __iter1 = py_iter_or_raise(&g__ASK_SPANS());
    for __item in __iter1 {
    parts = __item.clone();
    size = call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &size.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&slice_or_raise(&said.clone(), &start.clone(), &op_add(&start.clone(), &size.clone()), &Py::None), &parts.clone())); if !py_truthy(&__b) { __b } else { op_gt(&op_add(&start.clone(), &size.clone()), &end.clone()) } }) {
    end = op_add(&start.clone(), &size.clone());
    }
    }
    }
    if py_truthy(&Py::Bool(py_eq(&end.clone(), &Py::Int(0)))) {
    let __iter3 = py_iter_or_raise(&call_builtin("enumerate", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    index = __unpk4[0].clone();
    word = __unpk4[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g__CONNECTORS(), &word.clone()))) {
    end = op_add(&index.clone(), &Py::Int(1));
    }
    }
    }
    return slice_or_raise(&said.clone(), &end.clone(), &Py::None, &Py::None);
    Py::None
}
pub fn _synonyms(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut one: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The set for the word the line asked about, or None so another module can answer.");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    span = _span_after_ask(&[said.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[span.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    start = __item.clone();
    let __iter2 = py_iter_or_raise(&g__PHRASE_KEYS());
    for __item in __iter2 {
    key = __item.clone();
    parts = mcall(&key.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&span.clone(), &start.clone(), &op_add(&start.clone(), &call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), &parts.clone()))) {
    rest = slice_or_raise(&span.clone(), &op_add(&start.clone(), &call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None);
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__PADDING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g_WORD_SETS(), &key.clone());
    }
    }
    }
    word = geti(&span.clone(), &start.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_WORD_SETS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WORD_SETS(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } } } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WORD_SETS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&word.clone(), &py_str(" "))) } }) {
    rest = slice_or_raise(&span.clone(), &op_add(&start.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__PADDING(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g_WORD_SETS(), &word.clone());
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _plain_plurals(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut _pattern: Py = Py::None;
    let mut one: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("{noun: noun + \"s\"} for the everyday nouns no group above already answers.");
    taken = call_builtin("set", &[g__UNCOUNTABLE()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g__PLURAL_GROUPS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    _pattern = __unpk2[0].clone();
    words = __unpk2[1].clone();
    let __aug3 = op_bitor(&taken.clone(), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    taken = __aug3;
    }
    return { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g__PLAIN_NOUNS()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&taken.clone(), &one.clone()))) {  __out.push((one.clone(), op_add(&one.clone(), &py_str("s")))); } } py_dict(__out) })() };
    Py::None
}
pub fn _plurals_by_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut more: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut table: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every word the table holds, with its plural and the pattern of its group.");
    table = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&g__PLURAL_GROUPS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    words = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&mcall(&words.clone(), "items", &[]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    one = __unpk4[0].clone();
    more = __unpk4[1].clone();
    let _ = py_setitem(&table.clone(), &one.clone(), py_tuple(vec![more.clone(), pattern.clone()]));
    }
    }
    return table.clone();
    Py::None
}
pub fn _the_words_of(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let _ = py_str("The line's own words, lowercased and with the punctuation taken off the ends.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() };
    Py::None
}
pub fn _names_a_word_to_change(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names a word this table has, or asks about plurals in general.\n\n    The row is reached by the ask rather than by the word, so the guard is what keeps it off a\n    plural question about a word another row answers better: \"what is the plural of octopus\" and\n    \"whats the plural of virus\" belong to the row that says octopuses or octopi, and a line with\n    no word of this table in it (\"whats the plural of that\", \"how do plurals work\" is the\n    general case below) is no answer of this row's to give.\n    ");
    said = _the_words_of(&[understood.clone()]);
    let __iter1 = py_iter_or_raise(&said.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLURALS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLURAL_SINGULARS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__UNCOUNTABLE(), &word.clone())) } } } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__PLURAL_GENERAL(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[_the_words_the_ask_names(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_words_the_ask_names(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words the line asks the plural of, read after the ask; empty when it names none.");
    said = _the_words_of(&[understood.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let mut __else2 = false;
    for __item in __iter1 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    word = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__PLURAL_ASK(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__PLURAL_ASK_FRAME(), &word.clone())) } }) {
    break;
    }
    }
    return py_list(vec![]);
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&said.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__PLURAL_NOT_THE_NOUN(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__PLURAL_ASK(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_PLURALS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_PLURAL_SINGULARS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__UNCOUNTABLE(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__PLUS_FORMS(), &word.clone())) } } } } } } } } } } } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return named.clone();
    Py::None
}
pub fn _the_plural_by_its_ending(args: &[Py]) -> Py {
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
    let mut more: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut twin: Py = Py::None;
    let _ = py_str("The plural of a noun no group holds, and the sentence for its ending.\n\n    Only the endings that are certain are read: -s, -x, -z, -ch, -sh take -es, a consonant then y\n    becomes -ies, the -o words in the list take -es, and everything else adds s. A -f or -fe word\n    is left to the group above, because half of them keep the -f (roof, chief, giraffe) and\n    \"giraves\" is a worse answer than a punt.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g__PLURAL_SAME_WORD(), &word.clone()))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&word.clone(), "capitalize", &[]), -1, "")); __s.push_str(" is its own plural - the same word for one and for many: one "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(", two "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[g__PLURAL_NO_COUNT()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Word".to_string(), mcall(&word.clone(), "capitalize", &[])), ("word".to_string(), word.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__PLURAL_NONE_SENTENCE(), "format", args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_contains(&g__PLURAL_NOT_A_NOUN_AT_ALL(), &word.clone()))) {
    return g__PLURAL_RULE();
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("us")])) {
    more = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), &py_str("uses"));
    twin = mcall(&g__PLURAL_LATIN_TWIN(), "get", &[word.clone()]);
    if py_truthy(&twin.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&more.clone(), "capitalize", &[]), -1, "")); __s.push_str(" is the plural of "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(", and "); __s.push_str(&fstr(&twin.clone(), -1, "")); __s.push_str(" is the Latin form plenty of people still use. "); __s.push_str(&fstr(&g__PLURAL_US_SENTENCE(), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&more.clone(), "capitalize", &[]), -1, "")); __s.push_str(" is the plural of "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&g__PLURAL_US_SENTENCE(), -1, "")); py_string(__s) };
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("s"), py_str("x"), py_str("z"), py_str("ch"), py_str("sh")])])) {
    more = op_add(&word.clone(), &if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("z")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("zz")]))) } }) { py_str("zes") } else { py_str("es") });
    rule = py_str("es");
    } else {
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&word.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies")), py_str("ies")]), 2, None);
    more = __unpk1[0].clone();
    rule = __unpk1[1].clone();
    } else {
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("o")])) {
    let __unpk2 = py_unpack(&if py_truthy(&Py::Bool(py_contains(&g__PLURAL_OES(), &word.clone()))) { py_tuple(vec![op_add(&word.clone(), &py_str("es")), py_str("oes")]) } else { py_tuple(vec![op_add(&word.clone(), &py_str("s")), py_str("os")]) }, 2, None);
    more = __unpk2[0].clone();
    rule = __unpk2[1].clone();
    } else {
    let __unpk3 = py_unpack(&py_tuple(vec![op_add(&word.clone(), &py_str("s")), py_str("plain")]), 2, None);
    more = __unpk3[0].clone();
    rule = __unpk3[1].clone();
    }
    }
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&more.clone(), "capitalize", &[]), -1, "")); __s.push_str(" is the plural of "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&geti(&g__PLURAL_RULE_SENTENCE(), &rule.clone()), -1, "")); py_string(__s) };
    Py::None
}
pub fn _plural_of(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut found: Py = Py::None;
    let mut more: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The plural of the word the line asked about, with its pattern, or None to pass.");
    said = _the_words_of(&[understood.clone()]);
    let __iter1 = py_iter_or_raise(&said.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__PLURAL_GENERAL(), &word.clone()))) {
    continue;
    }
    found = mcall(&g_PLURALS(), "get", &[word.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    let __unpk2 = py_unpack(&found.clone(), 2, None);
    more = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&more.clone(), &word.clone()))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&word.clone(), "capitalize", &[]), -1, "")); __s.push_str(" is its own plural - the same word for one and for many: one "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(", two "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&pattern.clone(), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&more.clone(), "capitalize", &[]), -1, "")); __s.push_str(" is the plural of "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&pattern.clone(), -1, "")); py_string(__s) };
    }
    one = mcall(&g_PLURAL_SINGULARS(), "get", &[word.clone()]);
    if py_truthy(&Py::Bool(!py_is(&one.clone(), &Py::None))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&word.clone(), "capitalize", &[]), -1, "")); __s.push_str(" is already plural - one "); __s.push_str(&fstr(&one.clone(), -1, "")); __s.push_str(", two "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&geti(&geti(&g_PLURALS(), &one.clone()), &Py::Int(1)), -1, "")); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_contains(&g__UNCOUNTABLE(), &word.clone()))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&word.clone(), "capitalize", &[]), -1, "")); __s.push_str(" has no plural in the everyday sense: you would say some "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(", or a bit of "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(", not "); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str("s."); py_string(__s) };
    }
    }
    let __iter3 = py_iter_or_raise(&said.clone());
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__PLURAL_GENERAL(), &word.clone()))) {
    return g__PLURAL_RULE();
    }
    }
    asked = _the_words_the_ask_names(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[call_builtin("set", &[asked.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return _the_plural_by_its_ending(&[geti(&asked.clone(), &Py::Int(0))]);
    }
    if py_truthy(&asked.clone()) {
    return g__PLURAL_RULE();
    }
    return Py::None;
    Py::None
}
pub fn _line_text(args: &[Py]) -> Py {
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
    return mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    Py::None
}
pub fn _regular_form(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut verb: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verb") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verb"))) };
    let mut groups: Py = Py::None;
    let mut letter: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut vowel: Py = Py::None;
    let _ = py_str("The past tense a regular verb takes, by the spelling rules people are taught.\n\n    An e takes a d (hope, hoped); a consonant and a y becomes -ied (carry, carried); a one\n    syllable verb ending in one vowel and one consonant doubles that consonant (stop, stopped).\n    The vowel-group count is what keeps \"visit\" and \"open\" from becoming \"visitted\" and\n    \"openned\".");
    if py_truthy(&Py::Bool(!py_truthy(&verb.clone()))) {
    return op_add(&verb.clone(), &py_str("ed"));
    }
    if py_truthy(&mcall(&verb.clone(), "endswith", &[py_str("e")])) {
    return op_add(&verb.clone(), &py_str("d"));
    }
    if py_truthy(&{ let __b = mcall(&verb.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[verb.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__VOWELS(), &geti(&verb.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    return op_add(&slice_or_raise(&verb.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ied"));
    }
    if py_truthy(&Py::Bool(py_contains(&g__DOUBLED_L(), &verb.clone()))) {
    return op_add(&op_add(&verb.clone(), &geti(&verb.clone(), &op_neg(&Py::Int(1)))), &py_str("ed"));
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[verb.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__VOWELS(), &geti(&verb.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("wxy"), &geti(&verb.clone(), &op_neg(&Py::Int(1))))) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__VOWELS(), &geti(&verb.clone(), &op_neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__VOWELS(), &geti(&verb.clone(), &op_neg(&Py::Int(3))))) } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![Py::Int(0), Py::Bool(false)]), 2, None);
    groups = __unpk1[0].clone();
    previous = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&verb.clone());
    for __item in __iter2 {
    letter = __item.clone();
    vowel = Py::Bool(py_contains(&g__VOWELS(), &letter.clone()));
    if py_truthy(&{ let __b = vowel.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&previous.clone())) } }) {
    let __aug3 = op_add(&groups.clone(), &Py::Int(1));
    groups = __aug3;
    }
    previous = vowel.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&groups.clone(), &Py::Int(1)))) {
    return op_add(&op_add(&verb.clone(), &geti(&verb.clone(), &op_neg(&Py::Int(1)))), &py_str("ed"));
    }
    }
    }
    return op_add(&verb.clone(), &py_str("ed"));
    Py::None
}
pub fn _irregular_past(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut participle: Py = Py::None;
    let mut past: Py = Py::None;
    let mut text: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("The two forms of the verb the line names, in the words of a person asking.\n\n    The verb is the one after \"past ... of\", the same way the regular row reads it. Scanning the\n    line for the first table verb instead was a fault of its own: \"can you tell me the past tense\n    of swim\" is a question about swim, and tell and do are in the table and come first, so the\n    person got a confident answer about a verb they did not ask about.");
    text = _line_text(&[understood.clone()]);
    match_ = mcall(&g__PAST_OF(), "search", &[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&match_.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_IRREGULAR_VERBS(), &mcall(&match_.clone(), "group", &[Py::Int(1)]))) } }) {
    return Py::None;
    }
    verb = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    let __unpk1 = py_unpack(&geti(&g_IRREGULAR_VERBS(), &verb.clone()), 2, None);
    past = __unpk1[0].clone();
    participle = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_eq(&participle.clone(), &past.clone()))) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("The past tense of "), &verb.clone()), &py_str(" is ")), &past.clone()), &py_str(", and the past participle is the same: 'I ")), &past.clone()), &py_str(" it yesterday, I have ")), &past.clone()), &py_str(" it before.' Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning."));
    }
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("The past tense of "), &verb.clone()), &py_str(" is ")), &past.clone()), &py_str(" - 'I ")), &past.clone()), &py_str(" yesterday' - and the past participle is ")), &participle.clone()), &py_str(", the one that goes with have: 'I have ")), &participle.clone()), &py_str("'. Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning."));
    Py::None
}
pub fn _names_a_past_table_verb(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("The row's own question: a frame saying \"past ... of\", and a verb of the table after it.\n\n    Without it the table in the key means any mention of any table verb: \"when do i use past\n    tense\" is not the past tense of do, and \"i have a question about past tense\" is not the past\n    tense of have.");
    match_ = mcall(&g__PAST_OF(), "search", &[_line_text(&[understood.clone()])]);
    return { let __b = call_builtin("bool", &[match_.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_IRREGULAR_VERBS(), &mcall(&match_.clone(), "group", &[Py::Int(1)]))) } };
    Py::None
}
pub fn _names_a_past_verb(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    match_ = mcall(&g__PAST_OF(), "search", &[_line_text(&[understood.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&match_.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_IRREGULAR_VERBS(), &mcall(&match_.clone(), "group", &[Py::Int(1)]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _regular_past(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("The -ed rule, with the verb the line names.");
    match_ = mcall(&g__PAST_OF(), "search", &[_line_text(&[understood.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    verb = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&Py::Bool(py_contains(&g_IRREGULAR_VERBS(), &verb.clone()))) {
    return _irregular_past(&[memory.clone(), understood.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&g__TWO_VERBS(), &verb.clone()))) {
    return geti(&g__TWO_VERBS(), &verb.clone());
    }
    if py_truthy(&op_gt(&call_builtin("len", &[verb.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12))) {
    return op_add(&op_add(&op_add(&op_add(&py_str("I don't know a verb '"), &verb.clone()), &py_str("'. If it is a name or a made-up word the -ed rule still gives ")), &verb.clone()), &py_str("ed, and if it is a typo the nearest real verb is the one to take."));
    }
    return op_add(&op_add(&op_add(&mcall(&verb.clone(), "capitalize", &[]), &py_str(" -> ")), &_regular_form(&[verb.clone()])), &py_str(". Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought."));
    Py::None
}
pub fn _grammar_term(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    flat = mcall(&py_str(" "), "join", &[said.clone()]);
    let __iter1 = py_iter_or_raise(&mcall(&g_GRAMMAR_TERMS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &phrase.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &op_add(&mcall(&phrase.clone(), "rstrip", &[py_str("s")]), &py_str("s")))) } }) {
    return reply.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&phrase.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat.clone(), &phrase.clone())) } }) {
    return reply.clone();
    }
    }
    return py_str("The parts of a sentence, in the order they turn up: a noun names something, a verb is what it does or is, an adjective describes it, an adverb says how the doing happened, and a pronoun stands in for the noun. 'The tired dog slept badly' - dog is the noun, tired the adjective, slept the verb, badly the adverb.");
    Py::None
}
pub fn _nobody_in_the_line_is_forgetting(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True unless the line is about a person losing their words, not about the words.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    return Py::Bool(!py_truthy(&mcall(&g__SOMEBODY_FORGETTING(), "search", &[said.clone()])));
    Py::None
}
pub fn _poem_form(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut one: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    flat = mcall(&py_str(" "), "join", &[said.clone()]);
    let __iter1 = py_iter_or_raise(&g__POEM_FORMS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&word.clone());
    for __item in __iter3 {
    one = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &one.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&one.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat.clone(), &one.clone())) } } } }) {
    return reply.clone();
    }
    }
    }
    return py_str("A haiku is three lines of five, seven and five sounds; a limerick is five lines rhyming aabba with a joke in it; a sonnet is fourteen lines in iambic pentameter. Ask about one and I'll give you its shape.");
    Py::None
}
pub fn _write_an_email(args: &[Py]) -> Py {
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
    let mut example_subject: Py = Py::None;
    let _ = py_str("The shape of an email, with the sample subject line in the world the line names.\n\n    The card's own example was \"Invoice 2144 - payment due\", so \"how do i email my teacher\" and\n    \"how do i email my plumber\" were answered about an invoice: right about the shape of a\n    message and wrong about the person's life, which reads as a different answer altogether.\n    Only the example moves; every sentence after it is the same advice.\n    ");
    example_subject = py_func("example_subject", |args: &[Py]| pyrt::xrefs::xcall("compose.draft", "example_subject", args));
    return op_modulo(&py_str("Put the ask in the subject line and in the first sentence: \"%s\" tells them what it is before they open it. Then three or four sentences - what you want, the one or two facts they need, and by when - and a plain sign-off with your name. If you want an answer, say what happens next: \"If I do not hear back by Friday I will call.\""), &py_call(&example_subject, &[understood.clone()]));
    Py::None
}
pub fn _asks_for_the_verse(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks for a poem of this form rather than for its rules.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__VERSE_METHOD_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("write"), py_str("writing")]), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__VERSE_INSTANCE_LEAD(), "match", &[mcall(&py_str(" "), "join", &[words.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_opening_is_written_not_asked(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line asks for a question to ask: an interview question is not a first line.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__AN_OPENING_QUESTION(), "search", &[text.clone()])))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g__AN_OPENING_IS_WRITTEN(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _minutes_are_a_length_of_time(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line has a \"minutes\" that is not a count of them, so the row keeps it.\n\n    A count is not enough to stand the row down on its own: \"the meeting ran 90 minutes, how do\n    i write up the minutes\" counts minutes and then asks for the minutes of the meeting, and\n    standing down there lost the card to a count that is not the subject. The question the\n    guard asks is whether every minute word in the line is a count - only then is the line\n    about a length of time and not about the record of a meeting.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return op_gt(&call_builtin("len", &[mcall(&g_EVERY_MINUTE(), "findall", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&g_A_COUNTED_MINUTE(), "findall", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _the_photo_of(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What is in the picture, when the line says: \"a photo of my dog in the snow\".");
    found = stdlib_call("re", "search", &[py_str("(?:photo|picture|pic|snap|shot|image)\\s+(?:of|showing)\\s+(.+)$"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("and"), py_str("but"), py_str("that"), py_str("which"), py_str("please"), py_str("for"), py_str("from"), py_str("on"), py_str("at")]), &word.clone()))) {
    break;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    while py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("me"), py_str("my"), py_str("the"), py_str("a"), py_str("an"), py_str("just"), py_str("that")]), &geti(&kept.clone(), &op_neg(&Py::Int(1))))) } }) {
    let _ = mcall(&kept.clone(), "pop", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return py_str("");
    }
    subject = mcall(&py_str(" "), "join", &[kept.clone()]);
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("the"), py_str("a"), py_str("an"), py_str("our"), py_str("some")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    return subject.clone();
    Py::None
}
pub fn _the_caption_detail(args: &[Py]) -> Py {
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
    let mut detail: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The half of the caption the picture cannot show, chosen for the kind of picture.");
    said = op_add(&op_add(&py_str(" "), &subject.clone()), &py_str(" "));
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PEOPLE_PHOTO()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:me|us|we)\\b"), subject.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return g__PHOTO_DETAIL_PEOPLE();
    }
    let __iter1 = py_iter_or_raise(&g__PHOTO_DETAIL());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    detail = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return detail.clone();
    }
    }
    return g__PHOTO_DETAIL_ANY();
    Py::None
}
pub fn _a_caption(args: &[Py]) -> Py {
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
    let mut example: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut typed: Py = Py::None;
    let _ = py_str("The one-line caption rule, with an example built from the picture the person named.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    subject = _the_photo_of(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return py_str("Say what is in the picture and one thing the picture cannot show, then stop: \"Third attempt at the sourdough - this one actually rose.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own.");
    }
    said = op_add(&mcall(&geti(&subject.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&subject.clone(), &Py::Int(1), &Py::None, &Py::None));
    typed = stdlib_call("re", "sub", &[py_str("\\bi\\b"), py_str("I"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    example = op_modulo(&py_str("%s, %s"), &py_tuple(vec![typed.clone(), _the_caption_detail(&[subject.clone()])]));
    return op_modulo(&py_str("Say what is in the picture and one thing the picture cannot show, then stop: \"%s.\" One line beats three, a joke only works if it is about the picture, and leave the hashtags to a line of their own."), &example.clone());
    Py::None
}
pub fn _who_the_kind_word_is_for(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Who the line is about, from the words after about/for/to, or the words anywhere in it.");
    after = stdlib_call("re", "search", &[py_str("\\b(?:about|for|to)\\s+(.+)$"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    said = op_add(&op_add(&py_str(" "), &if py_truthy(&after.clone()) { mcall(&after.clone(), "group", &[Py::Int(1)]) } else { clean.clone() }), &py_str(" "));
    whole = op_add(&op_add(&py_str(" "), &clean.clone()), &py_str(" "));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("stranger"), g__STRANGER_KIND()]), py_tuple(vec![py_str("animal"), g__ANIMAL_KIND()]), py_tuple(vec![py_str("person"), g__PEOPLE_KIND()]), py_tuple(vec![py_str("bake"), g__BAKE_KIND()]), py_tuple(vec![py_str("looks"), g__LOOKS_KIND()]), py_tuple(vec![py_str("place"), g__PLACE_KIND()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    place = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return place.clone();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&whole.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return place.clone();
    }
    }
    return py_str("someone");
    Py::None
}
pub fn _a_kind_word(args: &[Py]) -> Py {
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
    let _ = py_str("A compliment the person can copy, chosen for what the line says it is about.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return geti(&g__A_KIND_WORD(), &_who_the_kind_word_is_for(&[clean.clone()]));
    Py::None
}
pub fn _the_matter_is_worth_praise(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut matter: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True unless the line names a thing to raise with somebody, rather than a thing to praise.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    after = stdlib_call("re", "search", &[py_str("\\babout\\s+(.+)$"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return Py::Bool(true);
    }
    matter = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&after.clone(), "group", &[Py::Int(1)]), "split", &[])])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g__A_MATTER_TO_RAISE());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&matter.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _mark_unnamed(args: &[Py]) -> Py {
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
    let mut look: Py = Py::None;
    let mut looks: Py = Py::None;
    let mut name: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The mark from the way it looks or the key it sits on, else how to describe it.");
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g__MARK_BY_LOOK());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    looks = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&{ let __b = _mentions(&[text.clone(), looks.clone()]); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&looks.clone()) { let mut look: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&look.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &look.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return reply.clone();
    }
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("#"), py_str("hashtag")]), py_tuple(vec![py_str("@"), py_str("at sign")]), py_tuple(vec![py_str("|"), py_str("pipe")]), py_tuple(vec![py_str("~"), py_str("tilde")]), py_tuple(vec![py_str("*"), py_str("asterisk")]), py_tuple(vec![py_str("^"), py_str("caret")]), py_tuple(vec![py_str("_"), py_str("underscore")])]));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    sign = __unpk4[0].clone();
    name = __unpk4[1].clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &sign.clone()))) {
    return geti(&g_GRAMMAR_TERMS(), &name.clone());
    }
    }
    let __iter5 = py_iter_or_raise(&mcall(&g_GRAMMAR_TERMS(), "items", &[]));
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    name = __unpk6[0].clone();
    reply = __unpk6[1].clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &name.clone()))) {
    return reply.clone();
    }
    }
    return py_str("Tell me which key it is on and I will name it: the tall line on the left of the keyboard is the pipe (|), the mark over 2 is the at sign (@), over 3 the hash (#), and the wavy one on the left is a tilde (~). If you can paste it into a line, I can read it too.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.language".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANIMAL_NOUNS" => Some(g_ANIMAL_NOUNS()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ASK_FOR_THE_WORD" => Some(g_ASK_FOR_THE_WORD()),
            "A_COUNTED_MINUTE" => Some(g_A_COUNTED_MINUTE()),
            "A_DASH_OF_SOMETHING" => Some(g_A_DASH_OF_SOMETHING()),
            "DASH_LOOKS" => Some(g_DASH_LOOKS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EVERYDAY_WORDS" => Some(g_EVERYDAY_WORDS()),
            "EVERY_MINUTE" => Some(g_EVERY_MINUTE()),
            "GRAMMAR_TERMS" => Some(g_GRAMMAR_TERMS()),
            "IRREGULAR_VERBS" => Some(g_IRREGULAR_VERBS()),
            "PERSONAL_NEWS" => Some(g_PERSONAL_NEWS()),
            "PLURALS" => Some(g_PLURALS()),
            "PLURAL_SINGULARS" => Some(g_PLURAL_SINGULARS()),
            "WORD_GLOSSES" => Some(g_WORD_GLOSSES()),
            "WORD_SETS" => Some(g_WORD_SETS()),
            "_ANIMAL_KIND" => Some(g__ANIMAL_KIND()),
            "_AN_OPENING_IS_WRITTEN" => Some(g__AN_OPENING_IS_WRITTEN()),
            "_AN_OPENING_QUESTION" => Some(g__AN_OPENING_QUESTION()),
            "_ASK_SPANS" => Some(g__ASK_SPANS()),
            "_A_KIND_WORD" => Some(g__A_KIND_WORD()),
            "_A_MATTER_TO_RAISE" => Some(g__A_MATTER_TO_RAISE()),
            "_A_MEANING_IS_ASKED" => Some(g__A_MEANING_IS_ASKED()),
            "_BAKE_KIND" => Some(g__BAKE_KIND()),
            "_BUYING_BRACKETS" => Some(g__BUYING_BRACKETS()),
            "_CONNECTORS" => Some(g__CONNECTORS()),
            "_DOUBLED_L" => Some(g__DOUBLED_L()),
            "_EXT_ILL" => Some(g__EXT_ILL()),
            "_EXT_LATE" => Some(g__EXT_LATE()),
            "_EXT_LATE_WORK" => Some(g__EXT_LATE_WORK()),
            "_EXT_SUBJECTS" => Some(g__EXT_SUBJECTS()),
            "_EXT_WORDS" => Some(g__EXT_WORDS()),
            "_FIGURES" => Some(g__FIGURES()),
            "_FILLER" => Some(g__FILLER()),
            "_FOOD_PHOTO" => Some(g__FOOD_PHOTO()),
            "_GRAMMAR_ASK" => Some(g__GRAMMAR_ASK()),
            "_GRAMMAR_WORDS" => Some(g__GRAMMAR_WORDS()),
            "_KIND_ASKS" => Some(g__KIND_ASKS()),
            "_KIND_SUBJECTS" => Some(g__KIND_SUBJECTS()),
            "_LOOKS_KIND" => Some(g__LOOKS_KIND()),
            "_MARK_ASK" => Some(g__MARK_ASK()),
            "_MARK_BY_LOOK" => Some(g__MARK_BY_LOOK()),
            "_MARK_WORDS" => Some(g__MARK_WORDS()),
            "_NOT_A_PUNCTUATION_BRACKET" => Some(g__NOT_A_PUNCTUATION_BRACKET()),
            "_NOT_MATHS_BRACKETS" => Some(g__NOT_MATHS_BRACKETS()),
            "_PADDING" => Some(g__PADDING()),
            "_PAST_ASK" => Some(g__PAST_ASK()),
            "_PAST_OF" => Some(g__PAST_OF()),
            "_PEOPLE_KIND" => Some(g__PEOPLE_KIND()),
            "_PEOPLE_PHOTO" => Some(g__PEOPLE_PHOTO()),
            "_PET_PHOTO" => Some(g__PET_PHOTO()),
            "_PHOTO_DETAIL" => Some(g__PHOTO_DETAIL()),
            "_PHOTO_DETAIL_ANY" => Some(g__PHOTO_DETAIL_ANY()),
            "_PHOTO_DETAIL_PEOPLE" => Some(g__PHOTO_DETAIL_PEOPLE()),
            "_PHRASE_KEYS" => Some(g__PHRASE_KEYS()),
            "_PLACE_KIND" => Some(g__PLACE_KIND()),
            "_PLACE_PHOTO" => Some(g__PLACE_PHOTO()),
            "_PLAIN_NOUNS" => Some(g__PLAIN_NOUNS()),
            "_PLURAL_ASK" => Some(g__PLURAL_ASK()),
            "_PLURAL_ASK_FRAME" => Some(g__PLURAL_ASK_FRAME()),
            "_PLURAL_GENERAL" => Some(g__PLURAL_GENERAL()),
            "_PLURAL_GROUPS" => Some(g__PLURAL_GROUPS()),
            "_PLURAL_LATIN_TWIN" => Some(g__PLURAL_LATIN_TWIN()),
            "_PLURAL_NONE_SENTENCE" => Some(g__PLURAL_NONE_SENTENCE()),
            "_PLURAL_NOT_A_NOUN_AT_ALL" => Some(g__PLURAL_NOT_A_NOUN_AT_ALL()),
            "_PLURAL_NOT_THE_NOUN" => Some(g__PLURAL_NOT_THE_NOUN()),
            "_PLURAL_NO_COUNT" => Some(g__PLURAL_NO_COUNT()),
            "_PLURAL_OES" => Some(g__PLURAL_OES()),
            "_PLURAL_RULE" => Some(g__PLURAL_RULE()),
            "_PLURAL_RULE_SENTENCE" => Some(g__PLURAL_RULE_SENTENCE()),
            "_PLURAL_SAME_WORD" => Some(g__PLURAL_SAME_WORD()),
            "_PLURAL_US_SENTENCE" => Some(g__PLURAL_US_SENTENCE()),
            "_PLUSES_REPLY" => Some(g__PLUSES_REPLY()),
            "_PLUS_FORMS" => Some(g__PLUS_FORMS()),
            "_POEM_ASK" => Some(g__POEM_ASK()),
            "_POEM_FORMS" => Some(g__POEM_FORMS()),
            "_RING_OR_WRITE" => Some(g__RING_OR_WRITE()),
            "_SOMEBODY_FORGETTING" => Some(g__SOMEBODY_FORGETTING()),
            "_STRANGER_KIND" => Some(g__STRANGER_KIND()),
            "_SYNONYM_ASK" => Some(g__SYNONYM_ASK()),
            "_THE_TWO_FIGURES" => Some(g__THE_TWO_FIGURES()),
            "_TWO_VERBS" => Some(g__TWO_VERBS()),
            "_UNCOUNTABLE" => Some(g__UNCOUNTABLE()),
            "_VERSE_INSTANCE_LEAD" => Some(g__VERSE_INSTANCE_LEAD()),
            "_VERSE_METHOD_WORDS" => Some(g__VERSE_METHOD_WORDS()),
            "_VOWELS" => Some(g__VOWELS()),
            "_WHAT_IS" => Some(g__WHAT_IS()),
            "_WRITING_BRACKETS" => Some(g__WRITING_BRACKETS()),
            "_a_caption" => Some(py_func("_a_caption", |args: &[Py]| _a_caption(args))),
            "_a_deadline_extension" => Some(py_func("_a_deadline_extension", |args: &[Py]| _a_deadline_extension(args))),
            "_a_kind_word" => Some(py_func("_a_kind_word", |args: &[Py]| _a_kind_word(args))),
            "_an_everyday_word" => Some(py_func("_an_everyday_word", |args: &[Py]| _an_everyday_word(args))),
            "_an_opening_is_written_not_asked" => Some(py_func("_an_opening_is_written_not_asked", |args: &[Py]| _an_opening_is_written_not_asked(args))),
            "_apology" => Some(py_func("_apology", |args: &[Py]| _apology(args))),
            "_asks_for_the_verse" => Some(py_func("_asks_for_the_verse", |args: &[Py]| _asks_for_the_verse(args))),
            "_asks_what_a_word_means" => Some(py_func("_asks_what_a_word_means", |args: &[Py]| _asks_what_a_word_means(args))),
            "_asks_which_of_the_two" => Some(py_func("_asks_which_of_the_two", |args: &[Py]| _asks_which_of_the_two(args))),
            "_brackets_are_punctuation" => Some(py_func("_brackets_are_punctuation", |args: &[Py]| _brackets_are_punctuation(args))),
            "_comma_rules" => Some(py_func("_comma_rules", |args: &[Py]| _comma_rules(args))),
            "_dash_is_the_mark" => Some(py_func("_dash_is_the_mark", |args: &[Py]| _dash_is_the_mark(args))),
            "_dash_or_hyphen" => Some(py_func("_dash_or_hyphen", |args: &[Py]| _dash_or_hyphen(args))),
            "_ellipsis_reply" => Some(py_func("_ellipsis_reply", |args: &[Py]| _ellipsis_reply(args))),
            "_end_mark" => Some(py_func("_end_mark", |args: &[Py]| _end_mark(args))),
            "_fewer" => Some(py_func("_fewer", |args: &[Py]| _fewer(args))),
            "_figure_of_speech" => Some(py_func("_figure_of_speech", |args: &[Py]| _figure_of_speech(args))),
            "_grammar_term" => Some(py_func("_grammar_term", |args: &[Py]| _grammar_term(args))),
            "_irregular_past" => Some(py_func("_irregular_past", |args: &[Py]| _irregular_past(args))),
            "_late_names_the_work" => Some(py_func("_late_names_the_work", |args: &[Py]| _late_names_the_work(args))),
            "_line_text" => Some(py_func("_line_text", |args: &[Py]| _line_text(args))),
            "_list_reply" => Some(py_func("_list_reply", |args: &[Py]| _list_reply(args))),
            "_mark_unnamed" => Some(py_func("_mark_unnamed", |args: &[Py]| _mark_unnamed(args))),
            "_mentions" => Some(py_func("_mentions", |args: &[Py]| _mentions(args))),
            "_minutes_are_a_length_of_time" => Some(py_func("_minutes_are_a_length_of_time", |args: &[Py]| _minutes_are_a_length_of_time(args))),
            "_names_a_past_table_verb" => Some(py_func("_names_a_past_table_verb", |args: &[Py]| _names_a_past_table_verb(args))),
            "_names_a_past_verb" => Some(py_func("_names_a_past_verb", |args: &[Py]| _names_a_past_verb(args))),
            "_names_a_word_to_change" => Some(py_func("_names_a_word_to_change", |args: &[Py]| _names_a_word_to_change(args))),
            "_nobody_in_the_line_is_forgetting" => Some(py_func("_nobody_in_the_line_is_forgetting", |args: &[Py]| _nobody_in_the_line_is_forgetting(args))),
            "_offers_two_of_the_family" => Some(py_func("_offers_two_of_the_family", |args: &[Py]| _offers_two_of_the_family(args))),
            "_paragraph" => Some(py_func("_paragraph", |args: &[Py]| _paragraph(args))),
            "_plain_plurals" => Some(py_func("_plain_plurals", |args: &[Py]| _plain_plurals(args))),
            "_plural_of" => Some(py_func("_plural_of", |args: &[Py]| _plural_of(args))),
            "_plurals_by_word" => Some(py_func("_plurals_by_word", |args: &[Py]| _plurals_by_word(args))),
            "_poem_form" => Some(py_func("_poem_form", |args: &[Py]| _poem_form(args))),
            "_regular_form" => Some(py_func("_regular_form", |args: &[Py]| _regular_form(args))),
            "_regular_past" => Some(py_func("_regular_past", |args: &[Py]| _regular_past(args))),
            "_same_word" => Some(py_func("_same_word", |args: &[Py]| _same_word(args))),
            "_span_after_ask" => Some(py_func("_span_after_ask", |args: &[Py]| _span_after_ask(args))),
            "_speak_clearly" => Some(py_func("_speak_clearly", |args: &[Py]| _speak_clearly(args))),
            "_speech_marks_reply" => Some(py_func("_speech_marks_reply", |args: &[Py]| _speech_marks_reply(args))),
            "_synonyms" => Some(py_func("_synonyms", |args: &[Py]| _synonyms(args))),
            "_the_caption_detail" => Some(py_func("_the_caption_detail", |args: &[Py]| _the_caption_detail(args))),
            "_the_figure_that_was_named" => Some(py_func("_the_figure_that_was_named", |args: &[Py]| _the_figure_that_was_named(args))),
            "_the_matter_is_worth_praise" => Some(py_func("_the_matter_is_worth_praise", |args: &[Py]| _the_matter_is_worth_praise(args))),
            "_the_photo_of" => Some(py_func("_the_photo_of", |args: &[Py]| _the_photo_of(args))),
            "_the_plural_by_its_ending" => Some(py_func("_the_plural_by_its_ending", |args: &[Py]| _the_plural_by_its_ending(args))),
            "_the_words_of" => Some(py_func("_the_words_of", |args: &[Py]| _the_words_of(args))),
            "_the_words_the_ask_names" => Some(py_func("_the_words_the_ask_names", |args: &[Py]| _the_words_the_ask_names(args))),
            "_who_the_kind_word_is_for" => Some(py_func("_who_the_kind_word_is_for", |args: &[Py]| _who_the_kind_word_is_for(args))),
            "_word_meaning" => Some(py_func("_word_meaning", |args: &[Py]| _word_meaning(args))),
            "_write_an_email" => Some(py_func("_write_an_email", |args: &[Py]| _write_an_email(args))),
            "entry" => Some(g_entry()),
            "thanks_note" => Some(g_thanks_note()),
            "re" => Some(stdlib_module("re")),
            "string" => Some(stdlib_module("string")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANIMAL_NOUNS" => s_ANIMAL_NOUNS(v),
            "ANSWERS" => s_ANSWERS(v),
            "ASK_FOR_THE_WORD" => s_ASK_FOR_THE_WORD(v),
            "A_COUNTED_MINUTE" => s_A_COUNTED_MINUTE(v),
            "A_DASH_OF_SOMETHING" => s_A_DASH_OF_SOMETHING(v),
            "DASH_LOOKS" => s_DASH_LOOKS(v),
            "ENTRIES" => s_ENTRIES(v),
            "EVERYDAY_WORDS" => s_EVERYDAY_WORDS(v),
            "EVERY_MINUTE" => s_EVERY_MINUTE(v),
            "GRAMMAR_TERMS" => s_GRAMMAR_TERMS(v),
            "IRREGULAR_VERBS" => s_IRREGULAR_VERBS(v),
            "PERSONAL_NEWS" => s_PERSONAL_NEWS(v),
            "PLURALS" => s_PLURALS(v),
            "PLURAL_SINGULARS" => s_PLURAL_SINGULARS(v),
            "WORD_GLOSSES" => s_WORD_GLOSSES(v),
            "WORD_SETS" => s_WORD_SETS(v),
            "_ANIMAL_KIND" => s__ANIMAL_KIND(v),
            "_AN_OPENING_IS_WRITTEN" => s__AN_OPENING_IS_WRITTEN(v),
            "_AN_OPENING_QUESTION" => s__AN_OPENING_QUESTION(v),
            "_ASK_SPANS" => s__ASK_SPANS(v),
            "_A_KIND_WORD" => s__A_KIND_WORD(v),
            "_A_MATTER_TO_RAISE" => s__A_MATTER_TO_RAISE(v),
            "_A_MEANING_IS_ASKED" => s__A_MEANING_IS_ASKED(v),
            "_BAKE_KIND" => s__BAKE_KIND(v),
            "_BUYING_BRACKETS" => s__BUYING_BRACKETS(v),
            "_CONNECTORS" => s__CONNECTORS(v),
            "_DOUBLED_L" => s__DOUBLED_L(v),
            "_EXT_ILL" => s__EXT_ILL(v),
            "_EXT_LATE" => s__EXT_LATE(v),
            "_EXT_LATE_WORK" => s__EXT_LATE_WORK(v),
            "_EXT_SUBJECTS" => s__EXT_SUBJECTS(v),
            "_EXT_WORDS" => s__EXT_WORDS(v),
            "_FIGURES" => s__FIGURES(v),
            "_FILLER" => s__FILLER(v),
            "_FOOD_PHOTO" => s__FOOD_PHOTO(v),
            "_GRAMMAR_ASK" => s__GRAMMAR_ASK(v),
            "_GRAMMAR_WORDS" => s__GRAMMAR_WORDS(v),
            "_KIND_ASKS" => s__KIND_ASKS(v),
            "_KIND_SUBJECTS" => s__KIND_SUBJECTS(v),
            "_LOOKS_KIND" => s__LOOKS_KIND(v),
            "_MARK_ASK" => s__MARK_ASK(v),
            "_MARK_BY_LOOK" => s__MARK_BY_LOOK(v),
            "_MARK_WORDS" => s__MARK_WORDS(v),
            "_NOT_A_PUNCTUATION_BRACKET" => s__NOT_A_PUNCTUATION_BRACKET(v),
            "_NOT_MATHS_BRACKETS" => s__NOT_MATHS_BRACKETS(v),
            "_PADDING" => s__PADDING(v),
            "_PAST_ASK" => s__PAST_ASK(v),
            "_PAST_OF" => s__PAST_OF(v),
            "_PEOPLE_KIND" => s__PEOPLE_KIND(v),
            "_PEOPLE_PHOTO" => s__PEOPLE_PHOTO(v),
            "_PET_PHOTO" => s__PET_PHOTO(v),
            "_PHOTO_DETAIL" => s__PHOTO_DETAIL(v),
            "_PHOTO_DETAIL_ANY" => s__PHOTO_DETAIL_ANY(v),
            "_PHOTO_DETAIL_PEOPLE" => s__PHOTO_DETAIL_PEOPLE(v),
            "_PHRASE_KEYS" => s__PHRASE_KEYS(v),
            "_PLACE_KIND" => s__PLACE_KIND(v),
            "_PLACE_PHOTO" => s__PLACE_PHOTO(v),
            "_PLAIN_NOUNS" => s__PLAIN_NOUNS(v),
            "_PLURAL_ASK" => s__PLURAL_ASK(v),
            "_PLURAL_ASK_FRAME" => s__PLURAL_ASK_FRAME(v),
            "_PLURAL_GENERAL" => s__PLURAL_GENERAL(v),
            "_PLURAL_GROUPS" => s__PLURAL_GROUPS(v),
            "_PLURAL_LATIN_TWIN" => s__PLURAL_LATIN_TWIN(v),
            "_PLURAL_NONE_SENTENCE" => s__PLURAL_NONE_SENTENCE(v),
            "_PLURAL_NOT_A_NOUN_AT_ALL" => s__PLURAL_NOT_A_NOUN_AT_ALL(v),
            "_PLURAL_NOT_THE_NOUN" => s__PLURAL_NOT_THE_NOUN(v),
            "_PLURAL_NO_COUNT" => s__PLURAL_NO_COUNT(v),
            "_PLURAL_OES" => s__PLURAL_OES(v),
            "_PLURAL_RULE" => s__PLURAL_RULE(v),
            "_PLURAL_RULE_SENTENCE" => s__PLURAL_RULE_SENTENCE(v),
            "_PLURAL_SAME_WORD" => s__PLURAL_SAME_WORD(v),
            "_PLURAL_US_SENTENCE" => s__PLURAL_US_SENTENCE(v),
            "_PLUSES_REPLY" => s__PLUSES_REPLY(v),
            "_PLUS_FORMS" => s__PLUS_FORMS(v),
            "_POEM_ASK" => s__POEM_ASK(v),
            "_POEM_FORMS" => s__POEM_FORMS(v),
            "_RING_OR_WRITE" => s__RING_OR_WRITE(v),
            "_SOMEBODY_FORGETTING" => s__SOMEBODY_FORGETTING(v),
            "_STRANGER_KIND" => s__STRANGER_KIND(v),
            "_SYNONYM_ASK" => s__SYNONYM_ASK(v),
            "_THE_TWO_FIGURES" => s__THE_TWO_FIGURES(v),
            "_TWO_VERBS" => s__TWO_VERBS(v),
            "_UNCOUNTABLE" => s__UNCOUNTABLE(v),
            "_VERSE_INSTANCE_LEAD" => s__VERSE_INSTANCE_LEAD(v),
            "_VERSE_METHOD_WORDS" => s__VERSE_METHOD_WORDS(v),
            "_VOWELS" => s__VOWELS(v),
            "_WHAT_IS" => s__WHAT_IS(v),
            "_WRITING_BRACKETS" => s__WRITING_BRACKETS(v),
            "entry" => s_entry(v),
            "thanks_note" => s_thanks_note(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.language", module_obj);
    pyrt::xrefs::reg_fn("knowledge.language", "_apology", py_func("_apology", |args: &[Py]| _apology(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_fewer", py_func("_fewer", |args: &[Py]| _fewer(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_figure_of_speech", py_func("_figure_of_speech", |args: &[Py]| _figure_of_speech(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_dash_is_the_mark", py_func("_dash_is_the_mark", |args: &[Py]| _dash_is_the_mark(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_dash_or_hyphen", py_func("_dash_or_hyphen", |args: &[Py]| _dash_or_hyphen(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_comma_rules", py_func("_comma_rules", |args: &[Py]| _comma_rules(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_end_mark", py_func("_end_mark", |args: &[Py]| _end_mark(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_speak_clearly", py_func("_speak_clearly", |args: &[Py]| _speak_clearly(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_mentions", py_func("_mentions", |args: &[Py]| _mentions(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_asks_what_a_word_means", py_func("_asks_what_a_word_means", |args: &[Py]| _asks_what_a_word_means(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_same_word", py_func("_same_word", |args: &[Py]| _same_word(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_word_meaning", py_func("_word_meaning", |args: &[Py]| _word_meaning(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_an_everyday_word", py_func("_an_everyday_word", |args: &[Py]| _an_everyday_word(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_paragraph", py_func("_paragraph", |args: &[Py]| _paragraph(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_late_names_the_work", py_func("_late_names_the_work", |args: &[Py]| _late_names_the_work(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_a_deadline_extension", py_func("_a_deadline_extension", |args: &[Py]| _a_deadline_extension(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_brackets_are_punctuation", py_func("_brackets_are_punctuation", |args: &[Py]| _brackets_are_punctuation(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_ellipsis_reply", py_func("_ellipsis_reply", |args: &[Py]| _ellipsis_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_speech_marks_reply", py_func("_speech_marks_reply", |args: &[Py]| _speech_marks_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_list_reply", py_func("_list_reply", |args: &[Py]| _list_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_offers_two_of_the_family", py_func("_offers_two_of_the_family", |args: &[Py]| _offers_two_of_the_family(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_asks_which_of_the_two", py_func("_asks_which_of_the_two", |args: &[Py]| _asks_which_of_the_two(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_the_figure_that_was_named", py_func("_the_figure_that_was_named", |args: &[Py]| _the_figure_that_was_named(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_span_after_ask", py_func("_span_after_ask", |args: &[Py]| _span_after_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_synonyms", py_func("_synonyms", |args: &[Py]| _synonyms(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_plain_plurals", py_func("_plain_plurals", |args: &[Py]| _plain_plurals(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_plurals_by_word", py_func("_plurals_by_word", |args: &[Py]| _plurals_by_word(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_the_words_of", py_func("_the_words_of", |args: &[Py]| _the_words_of(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_names_a_word_to_change", py_func("_names_a_word_to_change", |args: &[Py]| _names_a_word_to_change(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_the_words_the_ask_names", py_func("_the_words_the_ask_names", |args: &[Py]| _the_words_the_ask_names(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_the_plural_by_its_ending", py_func("_the_plural_by_its_ending", |args: &[Py]| _the_plural_by_its_ending(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_plural_of", py_func("_plural_of", |args: &[Py]| _plural_of(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_line_text", py_func("_line_text", |args: &[Py]| _line_text(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_regular_form", py_func("_regular_form", |args: &[Py]| _regular_form(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_irregular_past", py_func("_irregular_past", |args: &[Py]| _irregular_past(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_names_a_past_table_verb", py_func("_names_a_past_table_verb", |args: &[Py]| _names_a_past_table_verb(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_names_a_past_verb", py_func("_names_a_past_verb", |args: &[Py]| _names_a_past_verb(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_regular_past", py_func("_regular_past", |args: &[Py]| _regular_past(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_grammar_term", py_func("_grammar_term", |args: &[Py]| _grammar_term(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_nobody_in_the_line_is_forgetting", py_func("_nobody_in_the_line_is_forgetting", |args: &[Py]| _nobody_in_the_line_is_forgetting(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_poem_form", py_func("_poem_form", |args: &[Py]| _poem_form(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_write_an_email", py_func("_write_an_email", |args: &[Py]| _write_an_email(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_asks_for_the_verse", py_func("_asks_for_the_verse", |args: &[Py]| _asks_for_the_verse(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_an_opening_is_written_not_asked", py_func("_an_opening_is_written_not_asked", |args: &[Py]| _an_opening_is_written_not_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_minutes_are_a_length_of_time", py_func("_minutes_are_a_length_of_time", |args: &[Py]| _minutes_are_a_length_of_time(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_the_photo_of", py_func("_the_photo_of", |args: &[Py]| _the_photo_of(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_the_caption_detail", py_func("_the_caption_detail", |args: &[Py]| _the_caption_detail(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_a_caption", py_func("_a_caption", |args: &[Py]| _a_caption(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_who_the_kind_word_is_for", py_func("_who_the_kind_word_is_for", |args: &[Py]| _who_the_kind_word_is_for(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_a_kind_word", py_func("_a_kind_word", |args: &[Py]| _a_kind_word(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_the_matter_is_worth_praise", py_func("_the_matter_is_worth_praise", |args: &[Py]| _the_matter_is_worth_praise(args)));
    pyrt::xrefs::reg_fn("knowledge.language", "_mark_unnamed", py_func("_mark_unnamed", |args: &[Py]| _mark_unnamed(args)));
    pyrt::xrefs::reg_access("knowledge.language", "ANIMAL_NOUNS", g_ANIMAL_NOUNS, s_ANIMAL_NOUNS);
    pyrt::xrefs::reg_access("knowledge.language", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.language", "ASK_FOR_THE_WORD", g_ASK_FOR_THE_WORD, s_ASK_FOR_THE_WORD);
    pyrt::xrefs::reg_access("knowledge.language", "A_COUNTED_MINUTE", g_A_COUNTED_MINUTE, s_A_COUNTED_MINUTE);
    pyrt::xrefs::reg_access("knowledge.language", "A_DASH_OF_SOMETHING", g_A_DASH_OF_SOMETHING, s_A_DASH_OF_SOMETHING);
    pyrt::xrefs::reg_access("knowledge.language", "DASH_LOOKS", g_DASH_LOOKS, s_DASH_LOOKS);
    pyrt::xrefs::reg_access("knowledge.language", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.language", "EVERYDAY_WORDS", g_EVERYDAY_WORDS, s_EVERYDAY_WORDS);
    pyrt::xrefs::reg_access("knowledge.language", "EVERY_MINUTE", g_EVERY_MINUTE, s_EVERY_MINUTE);
    pyrt::xrefs::reg_access("knowledge.language", "GRAMMAR_TERMS", g_GRAMMAR_TERMS, s_GRAMMAR_TERMS);
    pyrt::xrefs::reg_access("knowledge.language", "IRREGULAR_VERBS", g_IRREGULAR_VERBS, s_IRREGULAR_VERBS);
    pyrt::xrefs::reg_access("knowledge.language", "PERSONAL_NEWS", g_PERSONAL_NEWS, s_PERSONAL_NEWS);
    pyrt::xrefs::reg_access("knowledge.language", "PLURALS", g_PLURALS, s_PLURALS);
    pyrt::xrefs::reg_access("knowledge.language", "PLURAL_SINGULARS", g_PLURAL_SINGULARS, s_PLURAL_SINGULARS);
    pyrt::xrefs::reg_access("knowledge.language", "WORD_GLOSSES", g_WORD_GLOSSES, s_WORD_GLOSSES);
    pyrt::xrefs::reg_access("knowledge.language", "WORD_SETS", g_WORD_SETS, s_WORD_SETS);
    pyrt::xrefs::reg_access("knowledge.language", "_ANIMAL_KIND", g__ANIMAL_KIND, s__ANIMAL_KIND);
    pyrt::xrefs::reg_access("knowledge.language", "_AN_OPENING_IS_WRITTEN", g__AN_OPENING_IS_WRITTEN, s__AN_OPENING_IS_WRITTEN);
    pyrt::xrefs::reg_access("knowledge.language", "_AN_OPENING_QUESTION", g__AN_OPENING_QUESTION, s__AN_OPENING_QUESTION);
    pyrt::xrefs::reg_access("knowledge.language", "_ASK_SPANS", g__ASK_SPANS, s__ASK_SPANS);
    pyrt::xrefs::reg_access("knowledge.language", "_A_KIND_WORD", g__A_KIND_WORD, s__A_KIND_WORD);
    pyrt::xrefs::reg_access("knowledge.language", "_A_MATTER_TO_RAISE", g__A_MATTER_TO_RAISE, s__A_MATTER_TO_RAISE);
    pyrt::xrefs::reg_access("knowledge.language", "_A_MEANING_IS_ASKED", g__A_MEANING_IS_ASKED, s__A_MEANING_IS_ASKED);
    pyrt::xrefs::reg_access("knowledge.language", "_BAKE_KIND", g__BAKE_KIND, s__BAKE_KIND);
    pyrt::xrefs::reg_access("knowledge.language", "_BUYING_BRACKETS", g__BUYING_BRACKETS, s__BUYING_BRACKETS);
    pyrt::xrefs::reg_access("knowledge.language", "_CONNECTORS", g__CONNECTORS, s__CONNECTORS);
    pyrt::xrefs::reg_access("knowledge.language", "_DOUBLED_L", g__DOUBLED_L, s__DOUBLED_L);
    pyrt::xrefs::reg_access("knowledge.language", "_EXT_ILL", g__EXT_ILL, s__EXT_ILL);
    pyrt::xrefs::reg_access("knowledge.language", "_EXT_LATE", g__EXT_LATE, s__EXT_LATE);
    pyrt::xrefs::reg_access("knowledge.language", "_EXT_LATE_WORK", g__EXT_LATE_WORK, s__EXT_LATE_WORK);
    pyrt::xrefs::reg_access("knowledge.language", "_EXT_SUBJECTS", g__EXT_SUBJECTS, s__EXT_SUBJECTS);
    pyrt::xrefs::reg_access("knowledge.language", "_EXT_WORDS", g__EXT_WORDS, s__EXT_WORDS);
    pyrt::xrefs::reg_access("knowledge.language", "_FIGURES", g__FIGURES, s__FIGURES);
    pyrt::xrefs::reg_access("knowledge.language", "_FILLER", g__FILLER, s__FILLER);
    pyrt::xrefs::reg_access("knowledge.language", "_FOOD_PHOTO", g__FOOD_PHOTO, s__FOOD_PHOTO);
    pyrt::xrefs::reg_access("knowledge.language", "_GRAMMAR_ASK", g__GRAMMAR_ASK, s__GRAMMAR_ASK);
    pyrt::xrefs::reg_access("knowledge.language", "_GRAMMAR_WORDS", g__GRAMMAR_WORDS, s__GRAMMAR_WORDS);
    pyrt::xrefs::reg_access("knowledge.language", "_KIND_ASKS", g__KIND_ASKS, s__KIND_ASKS);
    pyrt::xrefs::reg_access("knowledge.language", "_KIND_SUBJECTS", g__KIND_SUBJECTS, s__KIND_SUBJECTS);
    pyrt::xrefs::reg_access("knowledge.language", "_LOOKS_KIND", g__LOOKS_KIND, s__LOOKS_KIND);
    pyrt::xrefs::reg_access("knowledge.language", "_MARK_ASK", g__MARK_ASK, s__MARK_ASK);
    pyrt::xrefs::reg_access("knowledge.language", "_MARK_BY_LOOK", g__MARK_BY_LOOK, s__MARK_BY_LOOK);
    pyrt::xrefs::reg_access("knowledge.language", "_MARK_WORDS", g__MARK_WORDS, s__MARK_WORDS);
    pyrt::xrefs::reg_access("knowledge.language", "_NOT_A_PUNCTUATION_BRACKET", g__NOT_A_PUNCTUATION_BRACKET, s__NOT_A_PUNCTUATION_BRACKET);
    pyrt::xrefs::reg_access("knowledge.language", "_NOT_MATHS_BRACKETS", g__NOT_MATHS_BRACKETS, s__NOT_MATHS_BRACKETS);
    pyrt::xrefs::reg_access("knowledge.language", "_PADDING", g__PADDING, s__PADDING);
    pyrt::xrefs::reg_access("knowledge.language", "_PAST_ASK", g__PAST_ASK, s__PAST_ASK);
    pyrt::xrefs::reg_access("knowledge.language", "_PAST_OF", g__PAST_OF, s__PAST_OF);
    pyrt::xrefs::reg_access("knowledge.language", "_PEOPLE_KIND", g__PEOPLE_KIND, s__PEOPLE_KIND);
    pyrt::xrefs::reg_access("knowledge.language", "_PEOPLE_PHOTO", g__PEOPLE_PHOTO, s__PEOPLE_PHOTO);
    pyrt::xrefs::reg_access("knowledge.language", "_PET_PHOTO", g__PET_PHOTO, s__PET_PHOTO);
    pyrt::xrefs::reg_access("knowledge.language", "_PHOTO_DETAIL", g__PHOTO_DETAIL, s__PHOTO_DETAIL);
    pyrt::xrefs::reg_access("knowledge.language", "_PHOTO_DETAIL_ANY", g__PHOTO_DETAIL_ANY, s__PHOTO_DETAIL_ANY);
    pyrt::xrefs::reg_access("knowledge.language", "_PHOTO_DETAIL_PEOPLE", g__PHOTO_DETAIL_PEOPLE, s__PHOTO_DETAIL_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.language", "_PHRASE_KEYS", g__PHRASE_KEYS, s__PHRASE_KEYS);
    pyrt::xrefs::reg_access("knowledge.language", "_PLACE_KIND", g__PLACE_KIND, s__PLACE_KIND);
    pyrt::xrefs::reg_access("knowledge.language", "_PLACE_PHOTO", g__PLACE_PHOTO, s__PLACE_PHOTO);
    pyrt::xrefs::reg_access("knowledge.language", "_PLAIN_NOUNS", g__PLAIN_NOUNS, s__PLAIN_NOUNS);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_ASK", g__PLURAL_ASK, s__PLURAL_ASK);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_ASK_FRAME", g__PLURAL_ASK_FRAME, s__PLURAL_ASK_FRAME);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_GENERAL", g__PLURAL_GENERAL, s__PLURAL_GENERAL);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_GROUPS", g__PLURAL_GROUPS, s__PLURAL_GROUPS);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_LATIN_TWIN", g__PLURAL_LATIN_TWIN, s__PLURAL_LATIN_TWIN);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_NONE_SENTENCE", g__PLURAL_NONE_SENTENCE, s__PLURAL_NONE_SENTENCE);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_NOT_A_NOUN_AT_ALL", g__PLURAL_NOT_A_NOUN_AT_ALL, s__PLURAL_NOT_A_NOUN_AT_ALL);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_NOT_THE_NOUN", g__PLURAL_NOT_THE_NOUN, s__PLURAL_NOT_THE_NOUN);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_NO_COUNT", g__PLURAL_NO_COUNT, s__PLURAL_NO_COUNT);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_OES", g__PLURAL_OES, s__PLURAL_OES);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_RULE", g__PLURAL_RULE, s__PLURAL_RULE);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_RULE_SENTENCE", g__PLURAL_RULE_SENTENCE, s__PLURAL_RULE_SENTENCE);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_SAME_WORD", g__PLURAL_SAME_WORD, s__PLURAL_SAME_WORD);
    pyrt::xrefs::reg_access("knowledge.language", "_PLURAL_US_SENTENCE", g__PLURAL_US_SENTENCE, s__PLURAL_US_SENTENCE);
    pyrt::xrefs::reg_access("knowledge.language", "_PLUSES_REPLY", g__PLUSES_REPLY, s__PLUSES_REPLY);
    pyrt::xrefs::reg_access("knowledge.language", "_PLUS_FORMS", g__PLUS_FORMS, s__PLUS_FORMS);
    pyrt::xrefs::reg_access("knowledge.language", "_POEM_ASK", g__POEM_ASK, s__POEM_ASK);
    pyrt::xrefs::reg_access("knowledge.language", "_POEM_FORMS", g__POEM_FORMS, s__POEM_FORMS);
    pyrt::xrefs::reg_access("knowledge.language", "_RING_OR_WRITE", g__RING_OR_WRITE, s__RING_OR_WRITE);
    pyrt::xrefs::reg_access("knowledge.language", "_SOMEBODY_FORGETTING", g__SOMEBODY_FORGETTING, s__SOMEBODY_FORGETTING);
    pyrt::xrefs::reg_access("knowledge.language", "_STRANGER_KIND", g__STRANGER_KIND, s__STRANGER_KIND);
    pyrt::xrefs::reg_access("knowledge.language", "_SYNONYM_ASK", g__SYNONYM_ASK, s__SYNONYM_ASK);
    pyrt::xrefs::reg_access("knowledge.language", "_THE_TWO_FIGURES", g__THE_TWO_FIGURES, s__THE_TWO_FIGURES);
    pyrt::xrefs::reg_access("knowledge.language", "_TWO_VERBS", g__TWO_VERBS, s__TWO_VERBS);
    pyrt::xrefs::reg_access("knowledge.language", "_UNCOUNTABLE", g__UNCOUNTABLE, s__UNCOUNTABLE);
    pyrt::xrefs::reg_access("knowledge.language", "_VERSE_INSTANCE_LEAD", g__VERSE_INSTANCE_LEAD, s__VERSE_INSTANCE_LEAD);
    pyrt::xrefs::reg_access("knowledge.language", "_VERSE_METHOD_WORDS", g__VERSE_METHOD_WORDS, s__VERSE_METHOD_WORDS);
    pyrt::xrefs::reg_access("knowledge.language", "_VOWELS", g__VOWELS, s__VOWELS);
    pyrt::xrefs::reg_access("knowledge.language", "_WHAT_IS", g__WHAT_IS, s__WHAT_IS);
    pyrt::xrefs::reg_access("knowledge.language", "_WRITING_BRACKETS", g__WRITING_BRACKETS, s__WRITING_BRACKETS);
    pyrt::xrefs::reg_access("knowledge.language", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.language", "thanks_note", g_thanks_note, s_thanks_note);
}
