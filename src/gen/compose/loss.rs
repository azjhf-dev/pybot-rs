#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ABOUT_THEM_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ABSENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACKNOWLEDGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADMINISTRATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADMINISTRATION_KEYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AFTER_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANTICIPATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_ENDING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_ABOUT_THE_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_ABOUT_THE_FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_AT_START: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_BARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_FORM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_NORMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_WE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AT_THE_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DEATH_HAPPENED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FEELING_BETTER_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PART_OF_THE_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RELATION_IN_A_HARD_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SPAN_ELSEWHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SPAN_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SPAN_NO_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SPAN_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STRANGERS_ANIMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SWAP_INHERITS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEDTIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_FEELING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURIABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURIED_ADJECTIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BURIED_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CANT_FIND_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARD_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARD_TO_WRITE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARD_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_WHO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAR_LOCK_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILDS_OWN_LOSS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOSE_PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITIONAL_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CORRECTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COULD_BE_DAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAYS_LEFT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAYS_WAIT_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEAD_IDIOM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEAD_PREDICATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATHABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATH_AS_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATH_COMPOUND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATH_CUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATH_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATH_OF_OWNER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATH_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEATH_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEFINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING_ALREADY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING_GRIEF_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING_OTHER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING_SAY_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DYING_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENDING_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENDING_FEELINGS_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENDING_SINCE_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENDING_TURNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ESTIMATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EULOGY_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EULOGY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FALLBACK_MODULES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FAMILY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEARED_NOT_YET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_ONLY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_PERSON_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_STATES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FICTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FILLERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FINAL_DAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FINDABLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_DAYS_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_DAYS_PET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOUND_WHAT_WAS_LOST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERALS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL_COST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL_COST_COMPARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL_COST_COMPARE_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL_COST_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL_HELP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL_HELP_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FUNERAL_WHEN_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GAME_TITLES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GETTING_THROUGH_DYING_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GETTING_THROUGH_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_THROUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_THROUGH_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_THROUGH_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_THROUGH_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_THROUGH_THE_LOSS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_THROUGH_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIFT_THING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GIVEN_UP_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOING_TO_DIE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GONE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_ASK_FORM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_CUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_FEELINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_OBJECTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_REFERRING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRIEF_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HABIT_PATTERNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELLO_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELLO_ON_ITS_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HELP_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HIS_HERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOSPICE_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOSPICE_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HUMAN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HUNTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IDEA_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_IT_TIME_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IT_DIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPING_GOING_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPING_JUDGED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEP_LOSING_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEPT_GOING_MISSING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KILLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KILLED_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KNOCKED_BY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAST_DRUG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAST_STAGES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LAUGHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEFT_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOCKED_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSE_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSS_CONTEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSS_GRIEF_CUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSS_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSS_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOSS_STILL_THE_THREAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST_BARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST_IN_A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST_THE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST_THE_FOUND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST_THING_TO_FIND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MACHINE_ANIMALS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEASURE_TOPICS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEDIA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEDICATION_STOPPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISADVENTURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISLAID_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISSING_SAID_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_ANIMALS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_ANIMAL_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_NOT_BETWEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_PRONOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISS_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MONTHS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_LOSS_SAY_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEWS_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NORMAL_TO_FEEL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOTHING_MORE_TO_DO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_BEDSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_CARE_GIVER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DOOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING_NEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_GETTING_BETTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_MISLAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_WORKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_YET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OBJECTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OCCASION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OFF_LIFE_SUPPORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OFF_SUPPORT_WELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_LAST_DRUG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPEN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_WEEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWNED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_PET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_WORRY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PALLIATIVE_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAPER_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_ALONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_ALONE_HANDED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_ALONE_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_ALONE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_A_RESULT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_DATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_ON_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_ON_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSED_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PASSIVE_AUX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEACE_IS_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE_AFTER_A_GIVE_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_ARRANGEMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_AT_THE_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_DIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_GRIEF_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_HAS_HAD_ENOUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_HAS_HAD_ENOUGH_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLURAL_PAIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POOLS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRACTICAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRACTICAL_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREPARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREPARE_OTHERWISE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PROCEEDINGS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRONOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUTTING_REGRETTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUT_DOWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUT_DOWN_REGRET_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUANTITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUIETER_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REASSURING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFERS_TO_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RELATIVE_PET_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REMAINS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REMEMBER_NOT_WHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REMEMBER_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RITUAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RITUAL_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROLE_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAFETY_MODULES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAY_GOODBYE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAY_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAY_THE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAY_TO_WHO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_ON_THE_DRUG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SELF_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SERIOUS_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_DYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSES_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPEAKER_HAS_HAD_ENOUGH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPEAKER_TOO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPEAKER_TOO_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_ABOUT_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_AT_THE_DOOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_CRY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_CRYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_DOING_FOR_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_LAYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_SEEING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_WITHOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STORY_INCIDENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STORY_MEDIUM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STRANGER_ANIMAL_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SURVIVED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWAP_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWAP_MOST_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SYMPATHY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TELL_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_HARD_TIME_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_TEARS_IN_A_CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEMSELVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_CANNOT_STOP_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_DAYS_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_GIVE_UP_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_HABIT_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_HOUSE_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_JOBS_FOR_THEM_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_ROUTINE_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_DEATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_ROLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THIS_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THREAD_TOPICS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THREAD_TURNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_OF_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_SINCE_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOLD_US: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOOK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRADITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TREATMENT_STOPPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TWO_WAY_ROLES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TWO_WORD_PETS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UNNAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERB_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VISIT_TARGETS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_IS_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_IS_THING_BARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_I_PUT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WISH_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITHOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WOULD_HAVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WOULD_HAVE_BEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRITE_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YEAR_OLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DAY_PART: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BILL_MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BILL_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BURIAL_SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CAN_BE_DYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CONTRACTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COST_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CREMATION_SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEATH_IN_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FUNERAL_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HELPERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_BILL_THREAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_CONTRACTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PEOPLE_PRONOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PRONOUN_MONEY_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAID_DYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SPAN_COUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_ESTATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_ONE_WHO_LEFT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WORKING_WITH_THE_DYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _dream: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_THEM_OPENS() -> Py { ensure_init(); ABOUT_THEM_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THEM_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THEM_OPENS.write() { *w = v; } }
pub fn g_ABSENCE() -> Py { ensure_init(); ABSENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABSENCE(v: Py) { ensure_init(); if let Ok(mut w) = ABSENCE.write() { *w = v; } }
pub fn g_ACKNOWLEDGE() -> Py { ensure_init(); ACKNOWLEDGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACKNOWLEDGE(v: Py) { ensure_init(); if let Ok(mut w) = ACKNOWLEDGE.write() { *w = v; } }
pub fn g_ADMINISTRATION() -> Py { ensure_init(); ADMINISTRATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADMINISTRATION(v: Py) { ensure_init(); if let Ok(mut w) = ADMINISTRATION.write() { *w = v; } }
pub fn g_ADMINISTRATION_KEYS() -> Py { ensure_init(); ADMINISTRATION_KEYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADMINISTRATION_KEYS(v: Py) { ensure_init(); if let Ok(mut w) = ADMINISTRATION_KEYS.write() { *w = v; } }
pub fn g_AFTER_REPLIES() -> Py { ensure_init(); AFTER_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_REPLIES.write() { *w = v; } }
pub fn g_ANTICIPATED() -> Py { ensure_init(); ANTICIPATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANTICIPATED(v: Py) { ensure_init(); if let Ok(mut w) = ANTICIPATED.write() { *w = v; } }
pub fn g_AN_ENDING() -> Py { ensure_init(); AN_ENDING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ENDING(v: Py) { ensure_init(); if let Ok(mut w) = AN_ENDING.write() { *w = v; } }
pub fn g_ASKS() -> Py { ensure_init(); ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS.write() { *w = v; } }
pub fn g_ASKS_ABOUT_THE_DAY() -> Py { ensure_init(); ASKS_ABOUT_THE_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_ABOUT_THE_DAY(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_ABOUT_THE_DAY.write() { *w = v; } }
pub fn g_ASKS_ABOUT_THE_FEELING() -> Py { ensure_init(); ASKS_ABOUT_THE_FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_ABOUT_THE_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_ABOUT_THE_FEELING.write() { *w = v; } }
pub fn g_ASK_AT_START() -> Py { ensure_init(); ASK_AT_START.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_AT_START(v: Py) { ensure_init(); if let Ok(mut w) = ASK_AT_START.write() { *w = v; } }
pub fn g_ASK_BARE() -> Py { ensure_init(); ASK_BARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_BARE(v: Py) { ensure_init(); if let Ok(mut w) = ASK_BARE.write() { *w = v; } }
pub fn g_ASK_FORM() -> Py { ensure_init(); ASK_FORM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_FORM(v: Py) { ensure_init(); if let Ok(mut w) = ASK_FORM.write() { *w = v; } }
pub fn g_ASK_NORMAL() -> Py { ensure_init(); ASK_NORMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_NORMAL(v: Py) { ensure_init(); if let Ok(mut w) = ASK_NORMAL.write() { *w = v; } }
pub fn g_ASK_TAIL() -> Py { ensure_init(); ASK_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = ASK_TAIL.write() { *w = v; } }
pub fn g_ASK_WE() -> Py { ensure_init(); ASK_WE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_WE(v: Py) { ensure_init(); if let Ok(mut w) = ASK_WE.write() { *w = v; } }
pub fn g_ASK_WORDS() -> Py { ensure_init(); ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_WORDS.write() { *w = v; } }
pub fn g_AT_THE_END() -> Py { ensure_init(); AT_THE_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AT_THE_END(v: Py) { ensure_init(); if let Ok(mut w) = AT_THE_END.write() { *w = v; } }
pub fn g_A_DEATH_HAPPENED() -> Py { ensure_init(); A_DEATH_HAPPENED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DEATH_HAPPENED(v: Py) { ensure_init(); if let Ok(mut w) = A_DEATH_HAPPENED.write() { *w = v; } }
pub fn g_A_FEELING_BETTER_ASK() -> Py { ensure_init(); A_FEELING_BETTER_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FEELING_BETTER_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_FEELING_BETTER_ASK.write() { *w = v; } }
pub fn g_A_PART_OF_THE_DAY() -> Py { ensure_init(); A_PART_OF_THE_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PART_OF_THE_DAY(v: Py) { ensure_init(); if let Ok(mut w) = A_PART_OF_THE_DAY.write() { *w = v; } }
pub fn g_A_PRICE() -> Py { ensure_init(); A_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = A_PRICE.write() { *w = v; } }
pub fn g_A_RELATION_IN_A_HARD_TIME() -> Py { ensure_init(); A_RELATION_IN_A_HARD_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RELATION_IN_A_HARD_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_RELATION_IN_A_HARD_TIME.write() { *w = v; } }
pub fn g_A_SPAN() -> Py { ensure_init(); A_SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SPAN(v: Py) { ensure_init(); if let Ok(mut w) = A_SPAN.write() { *w = v; } }
pub fn g_A_SPAN_ELSEWHERE() -> Py { ensure_init(); A_SPAN_ELSEWHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SPAN_ELSEWHERE(v: Py) { ensure_init(); if let Ok(mut w) = A_SPAN_ELSEWHERE.write() { *w = v; } }
pub fn g_A_SPAN_LEAD() -> Py { ensure_init(); A_SPAN_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SPAN_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = A_SPAN_LEAD.write() { *w = v; } }
pub fn g_A_SPAN_NO_NUMBER() -> Py { ensure_init(); A_SPAN_NO_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SPAN_NO_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = A_SPAN_NO_NUMBER.write() { *w = v; } }
pub fn g_A_SPAN_TAIL() -> Py { ensure_init(); A_SPAN_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SPAN_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = A_SPAN_TAIL.write() { *w = v; } }
pub fn g_A_STRANGERS_ANIMAL() -> Py { ensure_init(); A_STRANGERS_ANIMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STRANGERS_ANIMAL(v: Py) { ensure_init(); if let Ok(mut w) = A_STRANGERS_ANIMAL.write() { *w = v; } }
pub fn g_A_SWAP_INHERITS() -> Py { ensure_init(); A_SWAP_INHERITS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SWAP_INHERITS(v: Py) { ensure_init(); if let Ok(mut w) = A_SWAP_INHERITS.write() { *w = v; } }
pub fn g_BEDTIME() -> Py { ensure_init(); BEDTIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEDTIME(v: Py) { ensure_init(); if let Ok(mut w) = BEDTIME.write() { *w = v; } }
pub fn g_BODY_ANSWER() -> Py { ensure_init(); BODY_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = BODY_ANSWER.write() { *w = v; } }
pub fn g_BODY_FEELING() -> Py { ensure_init(); BODY_FEELING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_FEELING(v: Py) { ensure_init(); if let Ok(mut w) = BODY_FEELING.write() { *w = v; } }
pub fn g_BODY_STATE() -> Py { ensure_init(); BODY_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_STATE(v: Py) { ensure_init(); if let Ok(mut w) = BODY_STATE.write() { *w = v; } }
pub fn g_BURIABLE() -> Py { ensure_init(); BURIABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURIABLE(v: Py) { ensure_init(); if let Ok(mut w) = BURIABLE.write() { *w = v; } }
pub fn g_BURIED() -> Py { ensure_init(); BURIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURIED(v: Py) { ensure_init(); if let Ok(mut w) = BURIED.write() { *w = v; } }
pub fn g_BURIED_ADJECTIVE() -> Py { ensure_init(); BURIED_ADJECTIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURIED_ADJECTIVE(v: Py) { ensure_init(); if let Ok(mut w) = BURIED_ADJECTIVE.write() { *w = v; } }
pub fn g_BURIED_OBJECT() -> Py { ensure_init(); BURIED_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BURIED_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = BURIED_OBJECT.write() { *w = v; } }
pub fn g_CANT_FIND_THING() -> Py { ensure_init(); CANT_FIND_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CANT_FIND_THING(v: Py) { ensure_init(); if let Ok(mut w) = CANT_FIND_THING.write() { *w = v; } }
pub fn g_CARD() -> Py { ensure_init(); CARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARD(v: Py) { ensure_init(); if let Ok(mut w) = CARD.write() { *w = v; } }
pub fn g_CARD_REPLIES() -> Py { ensure_init(); CARD_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARD_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = CARD_REPLIES.write() { *w = v; } }
pub fn g_CARD_TO_WRITE() -> Py { ensure_init(); CARD_TO_WRITE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARD_TO_WRITE(v: Py) { ensure_init(); if let Ok(mut w) = CARD_TO_WRITE.write() { *w = v; } }
pub fn g_CARD_WORD() -> Py { ensure_init(); CARD_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARD_WORD(v: Py) { ensure_init(); if let Ok(mut w) = CARD_WORD.write() { *w = v; } }
pub fn g_CARE_WHO() -> Py { ensure_init(); CARE_WHO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_WHO(v: Py) { ensure_init(); if let Ok(mut w) = CARE_WHO.write() { *w = v; } }
pub fn g_CAR_LOCK_OUT() -> Py { ensure_init(); CAR_LOCK_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAR_LOCK_OUT(v: Py) { ensure_init(); if let Ok(mut w) = CAR_LOCK_OUT.write() { *w = v; } }
pub fn g_CHILDS_OWN_LOSS() -> Py { ensure_init(); CHILDS_OWN_LOSS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILDS_OWN_LOSS(v: Py) { ensure_init(); if let Ok(mut w) = CHILDS_OWN_LOSS.write() { *w = v; } }
pub fn g_CLAUSE() -> Py { ensure_init(); CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = CLAUSE.write() { *w = v; } }
pub fn g_CLOSE() -> Py { ensure_init(); CLOSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOSE(v: Py) { ensure_init(); if let Ok(mut w) = CLOSE.write() { *w = v; } }
pub fn g_CLOSE_PEOPLE() -> Py { ensure_init(); CLOSE_PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOSE_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = CLOSE_PEOPLE.write() { *w = v; } }
pub fn g_CONDITIONAL_TAIL() -> Py { ensure_init(); CONDITIONAL_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = CONDITIONAL_TAIL.write() { *w = v; } }
pub fn g_CORRECTS() -> Py { ensure_init(); CORRECTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CORRECTS(v: Py) { ensure_init(); if let Ok(mut w) = CORRECTS.write() { *w = v; } }
pub fn g_COULD_BE_DAYS() -> Py { ensure_init(); COULD_BE_DAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COULD_BE_DAYS(v: Py) { ensure_init(); if let Ok(mut w) = COULD_BE_DAYS.write() { *w = v; } }
pub fn g_DAYS_LEFT() -> Py { ensure_init(); DAYS_LEFT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAYS_LEFT(v: Py) { ensure_init(); if let Ok(mut w) = DAYS_LEFT.write() { *w = v; } }
pub fn g_DAYS_WAIT_FOR() -> Py { ensure_init(); DAYS_WAIT_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAYS_WAIT_FOR(v: Py) { ensure_init(); if let Ok(mut w) = DAYS_WAIT_FOR.write() { *w = v; } }
pub fn g_DAY_QUESTION() -> Py { ensure_init(); DAY_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = DAY_QUESTION.write() { *w = v; } }
pub fn g_DAY_THING() -> Py { ensure_init(); DAY_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_THING(v: Py) { ensure_init(); if let Ok(mut w) = DAY_THING.write() { *w = v; } }
pub fn g_DEAD_IDIOM() -> Py { ensure_init(); DEAD_IDIOM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEAD_IDIOM(v: Py) { ensure_init(); if let Ok(mut w) = DEAD_IDIOM.write() { *w = v; } }
pub fn g_DEAD_PREDICATE() -> Py { ensure_init(); DEAD_PREDICATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEAD_PREDICATE(v: Py) { ensure_init(); if let Ok(mut w) = DEAD_PREDICATE.write() { *w = v; } }
pub fn g_DEATHABLE() -> Py { ensure_init(); DEATHABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATHABLE(v: Py) { ensure_init(); if let Ok(mut w) = DEATHABLE.write() { *w = v; } }
pub fn g_DEATH_AS_A_THING() -> Py { ensure_init(); DEATH_AS_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATH_AS_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = DEATH_AS_A_THING.write() { *w = v; } }
pub fn g_DEATH_COMPOUND() -> Py { ensure_init(); DEATH_COMPOUND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATH_COMPOUND(v: Py) { ensure_init(); if let Ok(mut w) = DEATH_COMPOUND.write() { *w = v; } }
pub fn g_DEATH_CUE() -> Py { ensure_init(); DEATH_CUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATH_CUE(v: Py) { ensure_init(); if let Ok(mut w) = DEATH_CUE.write() { *w = v; } }
pub fn g_DEATH_NOUN() -> Py { ensure_init(); DEATH_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATH_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = DEATH_NOUN.write() { *w = v; } }
pub fn g_DEATH_OF_OWNER() -> Py { ensure_init(); DEATH_OF_OWNER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATH_OF_OWNER(v: Py) { ensure_init(); if let Ok(mut w) = DEATH_OF_OWNER.write() { *w = v; } }
pub fn g_DEATH_SAID() -> Py { ensure_init(); DEATH_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATH_SAID(v: Py) { ensure_init(); if let Ok(mut w) = DEATH_SAID.write() { *w = v; } }
pub fn g_DEATH_WORD() -> Py { ensure_init(); DEATH_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEATH_WORD(v: Py) { ensure_init(); if let Ok(mut w) = DEATH_WORD.write() { *w = v; } }
pub fn g_DEFINES() -> Py { ensure_init(); DEFINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEFINES(v: Py) { ensure_init(); if let Ok(mut w) = DEFINES.write() { *w = v; } }
pub fn g_DIED() -> Py { ensure_init(); DIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DIED(v: Py) { ensure_init(); if let Ok(mut w) = DIED.write() { *w = v; } }
pub fn g_DYING() -> Py { ensure_init(); DYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING(v: Py) { ensure_init(); if let Ok(mut w) = DYING.write() { *w = v; } }
pub fn g_DYING_ALREADY() -> Py { ensure_init(); DYING_ALREADY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING_ALREADY(v: Py) { ensure_init(); if let Ok(mut w) = DYING_ALREADY.write() { *w = v; } }
pub fn g_DYING_GRIEF_REPLIES() -> Py { ensure_init(); DYING_GRIEF_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING_GRIEF_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = DYING_GRIEF_REPLIES.write() { *w = v; } }
pub fn g_DYING_NOW() -> Py { ensure_init(); DYING_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING_NOW(v: Py) { ensure_init(); if let Ok(mut w) = DYING_NOW.write() { *w = v; } }
pub fn g_DYING_OTHER() -> Py { ensure_init(); DYING_OTHER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING_OTHER(v: Py) { ensure_init(); if let Ok(mut w) = DYING_OTHER.write() { *w = v; } }
pub fn g_DYING_REPLIES() -> Py { ensure_init(); DYING_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = DYING_REPLIES.write() { *w = v; } }
pub fn g_DYING_SAY_REPLIES() -> Py { ensure_init(); DYING_SAY_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING_SAY_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = DYING_SAY_REPLIES.write() { *w = v; } }
pub fn g_DYING_TO() -> Py { ensure_init(); DYING_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DYING_TO(v: Py) { ensure_init(); if let Ok(mut w) = DYING_TO.write() { *w = v; } }
pub fn g_ENDING_ASKED() -> Py { ensure_init(); ENDING_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENDING_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = ENDING_ASKED.write() { *w = v; } }
pub fn g_ENDING_FEELINGS_REPLIES() -> Py { ensure_init(); ENDING_FEELINGS_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENDING_FEELINGS_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = ENDING_FEELINGS_REPLIES.write() { *w = v; } }
pub fn g_ENDING_SINCE_REPLIES() -> Py { ensure_init(); ENDING_SINCE_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENDING_SINCE_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = ENDING_SINCE_REPLIES.write() { *w = v; } }
pub fn g_ENDING_TURNS() -> Py { ensure_init(); ENDING_TURNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENDING_TURNS(v: Py) { ensure_init(); if let Ok(mut w) = ENDING_TURNS.write() { *w = v; } }
pub fn g_ESTIMATE() -> Py { ensure_init(); ESTIMATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ESTIMATE(v: Py) { ensure_init(); if let Ok(mut w) = ESTIMATE.write() { *w = v; } }
pub fn g_EULOGY_ASK() -> Py { ensure_init(); EULOGY_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EULOGY_ASK(v: Py) { ensure_init(); if let Ok(mut w) = EULOGY_ASK.write() { *w = v; } }
pub fn g_EULOGY_WORD() -> Py { ensure_init(); EULOGY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EULOGY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = EULOGY_WORD.write() { *w = v; } }
pub fn g_FALLBACK_MODULES() -> Py { ensure_init(); FALLBACK_MODULES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FALLBACK_MODULES(v: Py) { ensure_init(); if let Ok(mut w) = FALLBACK_MODULES.write() { *w = v; } }
pub fn g_FAMILY() -> Py { ensure_init(); FAMILY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FAMILY(v: Py) { ensure_init(); if let Ok(mut w) = FAMILY.write() { *w = v; } }
pub fn g_FEARED_NOT_YET() -> Py { ensure_init(); FEARED_NOT_YET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEARED_NOT_YET(v: Py) { ensure_init(); if let Ok(mut w) = FEARED_NOT_YET.write() { *w = v; } }
pub fn g_FEELING_ASK() -> Py { ensure_init(); FEELING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_ASK.write() { *w = v; } }
pub fn g_FEELING_ONLY_WORDS() -> Py { ensure_init(); FEELING_ONLY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_ONLY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_ONLY_WORDS.write() { *w = v; } }
pub fn g_FEELING_PERSON_WORDS() -> Py { ensure_init(); FEELING_PERSON_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_PERSON_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_PERSON_WORDS.write() { *w = v; } }
pub fn g_FEELING_REPLIES() -> Py { ensure_init(); FEELING_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_REPLIES.write() { *w = v; } }
pub fn g_FEELING_STATES() -> Py { ensure_init(); FEELING_STATES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_STATES(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_STATES.write() { *w = v; } }
pub fn g_FEELING_WORDS() -> Py { ensure_init(); FEELING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_WORDS.write() { *w = v; } }
pub fn g_FICTION() -> Py { ensure_init(); FICTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FICTION(v: Py) { ensure_init(); if let Ok(mut w) = FICTION.write() { *w = v; } }
pub fn g_FILLERS() -> Py { ensure_init(); FILLERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FILLERS(v: Py) { ensure_init(); if let Ok(mut w) = FILLERS.write() { *w = v; } }
pub fn g_FINAL_DAYS() -> Py { ensure_init(); FINAL_DAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FINAL_DAYS(v: Py) { ensure_init(); if let Ok(mut w) = FINAL_DAYS.write() { *w = v; } }
pub fn g_FINDABLE() -> Py { ensure_init(); FINDABLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FINDABLE(v: Py) { ensure_init(); if let Ok(mut w) = FINDABLE.write() { *w = v; } }
pub fn g_FIRST_DAYS_PERSON() -> Py { ensure_init(); FIRST_DAYS_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_DAYS_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_DAYS_PERSON.write() { *w = v; } }
pub fn g_FIRST_DAYS_PET() -> Py { ensure_init(); FIRST_DAYS_PET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_DAYS_PET(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_DAYS_PET.write() { *w = v; } }
pub fn g_FOUND_WHAT_WAS_LOST() -> Py { ensure_init(); FOUND_WHAT_WAS_LOST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOUND_WHAT_WAS_LOST(v: Py) { ensure_init(); if let Ok(mut w) = FOUND_WHAT_WAS_LOST.write() { *w = v; } }
pub fn g_FUNERAL() -> Py { ensure_init(); FUNERAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL.write() { *w = v; } }
pub fn g_FUNERALS() -> Py { ensure_init(); FUNERALS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERALS(v: Py) { ensure_init(); if let Ok(mut w) = FUNERALS.write() { *w = v; } }
pub fn g_FUNERAL_COST() -> Py { ensure_init(); FUNERAL_COST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL_COST(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL_COST.write() { *w = v; } }
pub fn g_FUNERAL_COST_COMPARE() -> Py { ensure_init(); FUNERAL_COST_COMPARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL_COST_COMPARE(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL_COST_COMPARE.write() { *w = v; } }
pub fn g_FUNERAL_COST_COMPARE_REPLIES() -> Py { ensure_init(); FUNERAL_COST_COMPARE_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL_COST_COMPARE_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL_COST_COMPARE_REPLIES.write() { *w = v; } }
pub fn g_FUNERAL_COST_ELSE() -> Py { ensure_init(); FUNERAL_COST_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL_COST_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL_COST_ELSE.write() { *w = v; } }
pub fn g_FUNERAL_HELP() -> Py { ensure_init(); FUNERAL_HELP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL_HELP(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL_HELP.write() { *w = v; } }
pub fn g_FUNERAL_HELP_REPLIES() -> Py { ensure_init(); FUNERAL_HELP_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL_HELP_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL_HELP_REPLIES.write() { *w = v; } }
pub fn g_FUNERAL_REPLIES() -> Py { ensure_init(); FUNERAL_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL_REPLIES.write() { *w = v; } }
pub fn g_FUNERAL_WHEN_REPLIES() -> Py { ensure_init(); FUNERAL_WHEN_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FUNERAL_WHEN_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = FUNERAL_WHEN_REPLIES.write() { *w = v; } }
pub fn g_GAME_TITLES() -> Py { ensure_init(); GAME_TITLES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GAME_TITLES(v: Py) { ensure_init(); if let Ok(mut w) = GAME_TITLES.write() { *w = v; } }
pub fn g_GETTING_THROUGH_DYING_REPLIES() -> Py { ensure_init(); GETTING_THROUGH_DYING_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GETTING_THROUGH_DYING_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = GETTING_THROUGH_DYING_REPLIES.write() { *w = v; } }
pub fn g_GETTING_THROUGH_REPLIES() -> Py { ensure_init(); GETTING_THROUGH_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GETTING_THROUGH_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = GETTING_THROUGH_REPLIES.write() { *w = v; } }
pub fn g_GET_THROUGH() -> Py { ensure_init(); GET_THROUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_THROUGH(v: Py) { ensure_init(); if let Ok(mut w) = GET_THROUGH.write() { *w = v; } }
pub fn g_GET_THROUGH_DAY() -> Py { ensure_init(); GET_THROUGH_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_THROUGH_DAY(v: Py) { ensure_init(); if let Ok(mut w) = GET_THROUGH_DAY.write() { *w = v; } }
pub fn g_GET_THROUGH_FILLER() -> Py { ensure_init(); GET_THROUGH_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_THROUGH_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = GET_THROUGH_FILLER.write() { *w = v; } }
pub fn g_GET_THROUGH_OBJECT() -> Py { ensure_init(); GET_THROUGH_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_THROUGH_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = GET_THROUGH_OBJECT.write() { *w = v; } }
pub fn g_GET_THROUGH_THE_LOSS() -> Py { ensure_init(); GET_THROUGH_THE_LOSS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_THROUGH_THE_LOSS(v: Py) { ensure_init(); if let Ok(mut w) = GET_THROUGH_THE_LOSS.write() { *w = v; } }
pub fn g_GET_THROUGH_TO() -> Py { ensure_init(); GET_THROUGH_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_THROUGH_TO(v: Py) { ensure_init(); if let Ok(mut w) = GET_THROUGH_TO.write() { *w = v; } }
pub fn g_GIFT_THING_ASK() -> Py { ensure_init(); GIFT_THING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIFT_THING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = GIFT_THING_ASK.write() { *w = v; } }
pub fn g_GIVEN_UP_ON() -> Py { ensure_init(); GIVEN_UP_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GIVEN_UP_ON(v: Py) { ensure_init(); if let Ok(mut w) = GIVEN_UP_ON.write() { *w = v; } }
pub fn g_GOING_TO_DIE() -> Py { ensure_init(); GOING_TO_DIE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOING_TO_DIE(v: Py) { ensure_init(); if let Ok(mut w) = GOING_TO_DIE.write() { *w = v; } }
pub fn g_GONE_THING() -> Py { ensure_init(); GONE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GONE_THING(v: Py) { ensure_init(); if let Ok(mut w) = GONE_THING.write() { *w = v; } }
pub fn g_GRIEF_ASK_FORM() -> Py { ensure_init(); GRIEF_ASK_FORM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_ASK_FORM(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_ASK_FORM.write() { *w = v; } }
pub fn g_GRIEF_CUE() -> Py { ensure_init(); GRIEF_CUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_CUE(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_CUE.write() { *w = v; } }
pub fn g_GRIEF_FEELINGS() -> Py { ensure_init(); GRIEF_FEELINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_FEELINGS(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_FEELINGS.write() { *w = v; } }
pub fn g_GRIEF_OBJECTS() -> Py { ensure_init(); GRIEF_OBJECTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_OBJECTS(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_OBJECTS.write() { *w = v; } }
pub fn g_GRIEF_POINTS_BACK() -> Py { ensure_init(); GRIEF_POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_POINTS_BACK.write() { *w = v; } }
pub fn g_GRIEF_REFERRING() -> Py { ensure_init(); GRIEF_REFERRING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_REFERRING(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_REFERRING.write() { *w = v; } }
pub fn g_GRIEF_REPLIES() -> Py { ensure_init(); GRIEF_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_REPLIES.write() { *w = v; } }
pub fn g_GRIEF_SELF() -> Py { ensure_init(); GRIEF_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRIEF_SELF(v: Py) { ensure_init(); if let Ok(mut w) = GRIEF_SELF.write() { *w = v; } }
pub fn g_HABIT_PATTERNS() -> Py { ensure_init(); HABIT_PATTERNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HABIT_PATTERNS(v: Py) { ensure_init(); if let Ok(mut w) = HABIT_PATTERNS.write() { *w = v; } }
pub fn g_HELLO_AFTER() -> Py { ensure_init(); HELLO_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELLO_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = HELLO_AFTER.write() { *w = v; } }
pub fn g_HELLO_ON_ITS_OWN() -> Py { ensure_init(); HELLO_ON_ITS_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELLO_ON_ITS_OWN(v: Py) { ensure_init(); if let Ok(mut w) = HELLO_ON_ITS_OWN.write() { *w = v; } }
pub fn g_HELP_ASK() -> Py { ensure_init(); HELP_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HELP_ASK(v: Py) { ensure_init(); if let Ok(mut w) = HELP_ASK.write() { *w = v; } }
pub fn g_HIS_HERS() -> Py { ensure_init(); HIS_HERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HIS_HERS(v: Py) { ensure_init(); if let Ok(mut w) = HIS_HERS.write() { *w = v; } }
pub fn g_HOSPICE_CARE() -> Py { ensure_init(); HOSPICE_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOSPICE_CARE(v: Py) { ensure_init(); if let Ok(mut w) = HOSPICE_CARE.write() { *w = v; } }
pub fn g_HOSPICE_JOB() -> Py { ensure_init(); HOSPICE_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOSPICE_JOB(v: Py) { ensure_init(); if let Ok(mut w) = HOSPICE_JOB.write() { *w = v; } }
pub fn g_HUMAN_WORDS() -> Py { ensure_init(); HUMAN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HUMAN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = HUMAN_WORDS.write() { *w = v; } }
pub fn g_HUNTING() -> Py { ensure_init(); HUNTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HUNTING(v: Py) { ensure_init(); if let Ok(mut w) = HUNTING.write() { *w = v; } }
pub fn g_IDEA_ASK() -> Py { ensure_init(); IDEA_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IDEA_ASK(v: Py) { ensure_init(); if let Ok(mut w) = IDEA_ASK.write() { *w = v; } }
pub fn g_IN_CARE() -> Py { ensure_init(); IN_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_CARE(v: Py) { ensure_init(); if let Ok(mut w) = IN_CARE.write() { *w = v; } }
pub fn g_IS_IT_TIME_REPLIES() -> Py { ensure_init(); IS_IT_TIME_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_IT_TIME_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = IS_IT_TIME_REPLIES.write() { *w = v; } }
pub fn g_IT_DIED() -> Py { ensure_init(); IT_DIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IT_DIED(v: Py) { ensure_init(); if let Ok(mut w) = IT_DIED.write() { *w = v; } }
pub fn g_KEEPING_GOING_REPLIES() -> Py { ensure_init(); KEEPING_GOING_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPING_GOING_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = KEEPING_GOING_REPLIES.write() { *w = v; } }
pub fn g_KEEPING_JUDGED() -> Py { ensure_init(); KEEPING_JUDGED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPING_JUDGED(v: Py) { ensure_init(); if let Ok(mut w) = KEEPING_JUDGED.write() { *w = v; } }
pub fn g_KEEPS() -> Py { ensure_init(); KEEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS.write() { *w = v; } }
pub fn g_KEEP_LOSING_THING() -> Py { ensure_init(); KEEP_LOSING_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEP_LOSING_THING(v: Py) { ensure_init(); if let Ok(mut w) = KEEP_LOSING_THING.write() { *w = v; } }
pub fn g_KEPT_GOING_MISSING() -> Py { ensure_init(); KEPT_GOING_MISSING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEPT_GOING_MISSING(v: Py) { ensure_init(); if let Ok(mut w) = KEPT_GOING_MISSING.write() { *w = v; } }
pub fn g_KILLED() -> Py { ensure_init(); KILLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KILLED(v: Py) { ensure_init(); if let Ok(mut w) = KILLED.write() { *w = v; } }
pub fn g_KILLED_IN() -> Py { ensure_init(); KILLED_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KILLED_IN(v: Py) { ensure_init(); if let Ok(mut w) = KILLED_IN.write() { *w = v; } }
pub fn g_KNOCKED_BY() -> Py { ensure_init(); KNOCKED_BY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KNOCKED_BY(v: Py) { ensure_init(); if let Ok(mut w) = KNOCKED_BY.write() { *w = v; } }
pub fn g_LAST_DRUG() -> Py { ensure_init(); LAST_DRUG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAST_DRUG(v: Py) { ensure_init(); if let Ok(mut w) = LAST_DRUG.write() { *w = v; } }
pub fn g_LAST_STAGES() -> Py { ensure_init(); LAST_STAGES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAST_STAGES(v: Py) { ensure_init(); if let Ok(mut w) = LAST_STAGES.write() { *w = v; } }
pub fn g_LAUGHING() -> Py { ensure_init(); LAUGHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LAUGHING(v: Py) { ensure_init(); if let Ok(mut w) = LAUGHING.write() { *w = v; } }
pub fn g_LEFT_THING() -> Py { ensure_init(); LEFT_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEFT_THING(v: Py) { ensure_init(); if let Ok(mut w) = LEFT_THING.write() { *w = v; } }
pub fn g_LOCKED_OUT() -> Py { ensure_init(); LOCKED_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOCKED_OUT(v: Py) { ensure_init(); if let Ok(mut w) = LOCKED_OUT.write() { *w = v; } }
pub fn g_LOSE_A_PERSON() -> Py { ensure_init(); LOSE_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSE_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = LOSE_A_PERSON.write() { *w = v; } }
pub fn g_LOSING() -> Py { ensure_init(); LOSING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSING(v: Py) { ensure_init(); if let Ok(mut w) = LOSING.write() { *w = v; } }
pub fn g_LOSS_CONTEXT() -> Py { ensure_init(); LOSS_CONTEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSS_CONTEXT(v: Py) { ensure_init(); if let Ok(mut w) = LOSS_CONTEXT.write() { *w = v; } }
pub fn g_LOSS_GRIEF_CUE() -> Py { ensure_init(); LOSS_GRIEF_CUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSS_GRIEF_CUE(v: Py) { ensure_init(); if let Ok(mut w) = LOSS_GRIEF_CUE.write() { *w = v; } }
pub fn g_LOSS_OF() -> Py { ensure_init(); LOSS_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSS_OF(v: Py) { ensure_init(); if let Ok(mut w) = LOSS_OF.write() { *w = v; } }
pub fn g_LOSS_REPLIES() -> Py { ensure_init(); LOSS_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSS_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = LOSS_REPLIES.write() { *w = v; } }
pub fn g_LOSS_STILL_THE_THREAD() -> Py { ensure_init(); LOSS_STILL_THE_THREAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOSS_STILL_THE_THREAD(v: Py) { ensure_init(); if let Ok(mut w) = LOSS_STILL_THE_THREAD.write() { *w = v; } }
pub fn g_LOST() -> Py { ensure_init(); LOST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST(v: Py) { ensure_init(); if let Ok(mut w) = LOST.write() { *w = v; } }
pub fn g_LOST_BARE() -> Py { ensure_init(); LOST_BARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST_BARE(v: Py) { ensure_init(); if let Ok(mut w) = LOST_BARE.write() { *w = v; } }
pub fn g_LOST_FORMS() -> Py { ensure_init(); LOST_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = LOST_FORMS.write() { *w = v; } }
pub fn g_LOST_IN_A_PLACE() -> Py { ensure_init(); LOST_IN_A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST_IN_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = LOST_IN_A_PLACE.write() { *w = v; } }
pub fn g_LOST_SELF() -> Py { ensure_init(); LOST_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST_SELF(v: Py) { ensure_init(); if let Ok(mut w) = LOST_SELF.write() { *w = v; } }
pub fn g_LOST_THE() -> Py { ensure_init(); LOST_THE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST_THE(v: Py) { ensure_init(); if let Ok(mut w) = LOST_THE.write() { *w = v; } }
pub fn g_LOST_THE_FOUND() -> Py { ensure_init(); LOST_THE_FOUND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST_THE_FOUND(v: Py) { ensure_init(); if let Ok(mut w) = LOST_THE_FOUND.write() { *w = v; } }
pub fn g_LOST_THING_TO_FIND() -> Py { ensure_init(); LOST_THING_TO_FIND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST_THING_TO_FIND(v: Py) { ensure_init(); if let Ok(mut w) = LOST_THING_TO_FIND.write() { *w = v; } }
pub fn g_MACHINE_ANIMALS() -> Py { ensure_init(); MACHINE_ANIMALS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MACHINE_ANIMALS(v: Py) { ensure_init(); if let Ok(mut w) = MACHINE_ANIMALS.write() { *w = v; } }
pub fn g_MEASURE_TOPICS() -> Py { ensure_init(); MEASURE_TOPICS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEASURE_TOPICS(v: Py) { ensure_init(); if let Ok(mut w) = MEASURE_TOPICS.write() { *w = v; } }
pub fn g_MEDIA() -> Py { ensure_init(); MEDIA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEDIA(v: Py) { ensure_init(); if let Ok(mut w) = MEDIA.write() { *w = v; } }
pub fn g_MEDICATION_STOPPED() -> Py { ensure_init(); MEDICATION_STOPPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEDICATION_STOPPED(v: Py) { ensure_init(); if let Ok(mut w) = MEDICATION_STOPPED.write() { *w = v; } }
pub fn g_MISADVENTURE() -> Py { ensure_init(); MISADVENTURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISADVENTURE(v: Py) { ensure_init(); if let Ok(mut w) = MISADVENTURE.write() { *w = v; } }
pub fn g_MISLAID_REPLIES() -> Py { ensure_init(); MISLAID_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISLAID_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = MISLAID_REPLIES.write() { *w = v; } }
pub fn g_MISSING_SAID_AGAIN() -> Py { ensure_init(); MISSING_SAID_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISSING_SAID_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = MISSING_SAID_AGAIN.write() { *w = v; } }
pub fn g_MISS_ANIMALS() -> Py { ensure_init(); MISS_ANIMALS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_ANIMALS(v: Py) { ensure_init(); if let Ok(mut w) = MISS_ANIMALS.write() { *w = v; } }
pub fn g_MISS_ANIMAL_NOUNS() -> Py { ensure_init(); MISS_ANIMAL_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_ANIMAL_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = MISS_ANIMAL_NOUNS.write() { *w = v; } }
pub fn g_MISS_NOT() -> Py { ensure_init(); MISS_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_NOT(v: Py) { ensure_init(); if let Ok(mut w) = MISS_NOT.write() { *w = v; } }
pub fn g_MISS_NOT_BETWEEN() -> Py { ensure_init(); MISS_NOT_BETWEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_NOT_BETWEEN(v: Py) { ensure_init(); if let Ok(mut w) = MISS_NOT_BETWEEN.write() { *w = v; } }
pub fn g_MISS_NOUN() -> Py { ensure_init(); MISS_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = MISS_NOUN.write() { *w = v; } }
pub fn g_MISS_PEOPLE() -> Py { ensure_init(); MISS_PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = MISS_PEOPLE.write() { *w = v; } }
pub fn g_MISS_PRONOUN() -> Py { ensure_init(); MISS_PRONOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_PRONOUN(v: Py) { ensure_init(); if let Ok(mut w) = MISS_PRONOUN.write() { *w = v; } }
pub fn g_MISS_REPLIES() -> Py { ensure_init(); MISS_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = MISS_REPLIES.write() { *w = v; } }
pub fn g_MISS_SUBJECT() -> Py { ensure_init(); MISS_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = MISS_SUBJECT.write() { *w = v; } }
pub fn g_MISS_TAIL() -> Py { ensure_init(); MISS_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = MISS_TAIL.write() { *w = v; } }
pub fn g_MISS_VERB() -> Py { ensure_init(); MISS_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISS_VERB(v: Py) { ensure_init(); if let Ok(mut w) = MISS_VERB.write() { *w = v; } }
pub fn g_MONTHS() -> Py { ensure_init(); MONTHS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MONTHS(v: Py) { ensure_init(); if let Ok(mut w) = MONTHS.write() { *w = v; } }
pub fn g_NAMED_LOSS_SAY_REPLIES() -> Py { ensure_init(); NAMED_LOSS_SAY_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_LOSS_SAY_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_LOSS_SAY_REPLIES.write() { *w = v; } }
pub fn g_NEWS_OF() -> Py { ensure_init(); NEWS_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEWS_OF(v: Py) { ensure_init(); if let Ok(mut w) = NEWS_OF.write() { *w = v; } }
pub fn g_NORMAL_TO_FEEL() -> Py { ensure_init(); NORMAL_TO_FEEL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NORMAL_TO_FEEL(v: Py) { ensure_init(); if let Ok(mut w) = NORMAL_TO_FEEL.write() { *w = v; } }
pub fn g_NOTHING_MORE_TO_DO() -> Py { ensure_init(); NOTHING_MORE_TO_DO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOTHING_MORE_TO_DO(v: Py) { ensure_init(); if let Ok(mut w) = NOTHING_MORE_TO_DO.write() { *w = v; } }
pub fn g_NOT_A_BEDSIDE() -> Py { ensure_init(); NOT_A_BEDSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_BEDSIDE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_BEDSIDE.write() { *w = v; } }
pub fn g_NOT_A_CARE_GIVER() -> Py { ensure_init(); NOT_A_CARE_GIVER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_CARE_GIVER(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_CARE_GIVER.write() { *w = v; } }
pub fn g_NOT_A_DEATH() -> Py { ensure_init(); NOT_A_DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DEATH(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DEATH.write() { *w = v; } }
pub fn g_NOT_A_DOOR() -> Py { ensure_init(); NOT_A_DOOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DOOR(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DOOR.write() { *w = v; } }
pub fn g_NOT_A_THING_NEXT() -> Py { ensure_init(); NOT_A_THING_NEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_NEXT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING_NEXT.write() { *w = v; } }
pub fn g_NOT_GETTING_BETTER() -> Py { ensure_init(); NOT_GETTING_BETTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_GETTING_BETTER(v: Py) { ensure_init(); if let Ok(mut w) = NOT_GETTING_BETTER.write() { *w = v; } }
pub fn g_NOT_MISLAID() -> Py { ensure_init(); NOT_MISLAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_MISLAID(v: Py) { ensure_init(); if let Ok(mut w) = NOT_MISLAID.write() { *w = v; } }
pub fn g_NOT_THE_END() -> Py { ensure_init(); NOT_THE_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_END(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_END.write() { *w = v; } }
pub fn g_NOT_WORKING() -> Py { ensure_init(); NOT_WORKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_WORKING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_WORKING.write() { *w = v; } }
pub fn g_NOT_YET() -> Py { ensure_init(); NOT_YET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_YET(v: Py) { ensure_init(); if let Ok(mut w) = NOT_YET.write() { *w = v; } }
pub fn g_OBJECTS() -> Py { ensure_init(); OBJECTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OBJECTS(v: Py) { ensure_init(); if let Ok(mut w) = OBJECTS.write() { *w = v; } }
pub fn g_OCCASION() -> Py { ensure_init(); OCCASION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OCCASION(v: Py) { ensure_init(); if let Ok(mut w) = OCCASION.write() { *w = v; } }
pub fn g_OFF_LIFE_SUPPORT() -> Py { ensure_init(); OFF_LIFE_SUPPORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OFF_LIFE_SUPPORT(v: Py) { ensure_init(); if let Ok(mut w) = OFF_LIFE_SUPPORT.write() { *w = v; } }
pub fn g_OFF_SUPPORT_WELL() -> Py { ensure_init(); OFF_SUPPORT_WELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OFF_SUPPORT_WELL(v: Py) { ensure_init(); if let Ok(mut w) = OFF_SUPPORT_WELL.write() { *w = v; } }
pub fn g_ON_THE_LAST_DRUG() -> Py { ensure_init(); ON_THE_LAST_DRUG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_LAST_DRUG(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_LAST_DRUG.write() { *w = v; } }
pub fn g_OPEN_ASK() -> Py { ensure_init(); OPEN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPEN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = OPEN_ASK.write() { *w = v; } }
pub fn g_OTHER_WEEPS() -> Py { ensure_init(); OTHER_WEEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_WEEPS(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_WEEPS.write() { *w = v; } }
pub fn g_OWNED() -> Py { ensure_init(); OWNED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWNED(v: Py) { ensure_init(); if let Ok(mut w) = OWNED.write() { *w = v; } }
pub fn g_OWN_DEATH() -> Py { ensure_init(); OWN_DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_DEATH(v: Py) { ensure_init(); if let Ok(mut w) = OWN_DEATH.write() { *w = v; } }
pub fn g_OWN_PET() -> Py { ensure_init(); OWN_PET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_PET(v: Py) { ensure_init(); if let Ok(mut w) = OWN_PET.write() { *w = v; } }
pub fn g_OWN_STATE() -> Py { ensure_init(); OWN_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_STATE(v: Py) { ensure_init(); if let Ok(mut w) = OWN_STATE.write() { *w = v; } }
pub fn g_OWN_WORRY() -> Py { ensure_init(); OWN_WORRY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_WORRY(v: Py) { ensure_init(); if let Ok(mut w) = OWN_WORRY.write() { *w = v; } }
pub fn g_PALLIATIVE_ITSELF() -> Py { ensure_init(); PALLIATIVE_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PALLIATIVE_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = PALLIATIVE_ITSELF.write() { *w = v; } }
pub fn g_PAPER_ASK() -> Py { ensure_init(); PAPER_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAPER_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PAPER_ASK.write() { *w = v; } }
pub fn g_PASSED_ALONE() -> Py { ensure_init(); PASSED_ALONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_ALONE(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_ALONE.write() { *w = v; } }
pub fn g_PASSED_ALONE_HANDED() -> Py { ensure_init(); PASSED_ALONE_HANDED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_ALONE_HANDED(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_ALONE_HANDED.write() { *w = v; } }
pub fn g_PASSED_ALONE_NOUN() -> Py { ensure_init(); PASSED_ALONE_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_ALONE_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_ALONE_NOUN.write() { *w = v; } }
pub fn g_PASSED_ALONE_TAIL() -> Py { ensure_init(); PASSED_ALONE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_ALONE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_ALONE_TAIL.write() { *w = v; } }
pub fn g_PASSED_A_RESULT() -> Py { ensure_init(); PASSED_A_RESULT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_A_RESULT(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_A_RESULT.write() { *w = v; } }
pub fn g_PASSED_DATE() -> Py { ensure_init(); PASSED_DATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_DATE(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_DATE.write() { *w = v; } }
pub fn g_PASSED_ON() -> Py { ensure_init(); PASSED_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_ON(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_ON.write() { *w = v; } }
pub fn g_PASSED_ON_OBJECT() -> Py { ensure_init(); PASSED_ON_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_ON_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_ON_OBJECT.write() { *w = v; } }
pub fn g_PASSED_ON_SUBJECT() -> Py { ensure_init(); PASSED_ON_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_ON_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_ON_SUBJECT.write() { *w = v; } }
pub fn g_PASSED_TIME() -> Py { ensure_init(); PASSED_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSED_TIME(v: Py) { ensure_init(); if let Ok(mut w) = PASSED_TIME.write() { *w = v; } }
pub fn g_PASSIVE_AUX() -> Py { ensure_init(); PASSIVE_AUX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PASSIVE_AUX(v: Py) { ensure_init(); if let Ok(mut w) = PASSIVE_AUX.write() { *w = v; } }
pub fn g_PEACE_IS_MINE() -> Py { ensure_init(); PEACE_IS_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEACE_IS_MINE(v: Py) { ensure_init(); if let Ok(mut w) = PEACE_IS_MINE.write() { *w = v; } }
pub fn g_PEOPLE() -> Py { ensure_init(); PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE.write() { *w = v; } }
pub fn g_PEOPLE_AFTER_A_GIVE_UP() -> Py { ensure_init(); PEOPLE_AFTER_A_GIVE_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_AFTER_A_GIVE_UP(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE_AFTER_A_GIVE_UP.write() { *w = v; } }
pub fn g_PEOPLE_PHRASES() -> Py { ensure_init(); PEOPLE_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE_PHRASES.write() { *w = v; } }
pub fn g_PET() -> Py { ensure_init(); PET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET(v: Py) { ensure_init(); if let Ok(mut w) = PET.write() { *w = v; } }
pub fn g_PET_ARRANGEMENT() -> Py { ensure_init(); PET_ARRANGEMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_ARRANGEMENT(v: Py) { ensure_init(); if let Ok(mut w) = PET_ARRANGEMENT.write() { *w = v; } }
pub fn g_PET_AT_THE_END() -> Py { ensure_init(); PET_AT_THE_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_AT_THE_END(v: Py) { ensure_init(); if let Ok(mut w) = PET_AT_THE_END.write() { *w = v; } }
pub fn g_PET_DEATH() -> Py { ensure_init(); PET_DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_DEATH(v: Py) { ensure_init(); if let Ok(mut w) = PET_DEATH.write() { *w = v; } }
pub fn g_PET_DIED() -> Py { ensure_init(); PET_DIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_DIED(v: Py) { ensure_init(); if let Ok(mut w) = PET_DIED.write() { *w = v; } }
pub fn g_PET_GRIEF_REPLIES() -> Py { ensure_init(); PET_GRIEF_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_GRIEF_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = PET_GRIEF_REPLIES.write() { *w = v; } }
pub fn g_PET_HAS_HAD_ENOUGH() -> Py { ensure_init(); PET_HAS_HAD_ENOUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_HAS_HAD_ENOUGH(v: Py) { ensure_init(); if let Ok(mut w) = PET_HAS_HAD_ENOUGH.write() { *w = v; } }
pub fn g_PET_HAS_HAD_ENOUGH_AFTER() -> Py { ensure_init(); PET_HAS_HAD_ENOUGH_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_HAS_HAD_ENOUGH_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = PET_HAS_HAD_ENOUGH_AFTER.write() { *w = v; } }
pub fn g_PET_WORDS() -> Py { ensure_init(); PET_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = PET_WORDS.write() { *w = v; } }
pub fn g_PLACES() -> Py { ensure_init(); PLACES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACES(v: Py) { ensure_init(); if let Ok(mut w) = PLACES.write() { *w = v; } }
pub fn g_PLAIN() -> Py { ensure_init(); PLAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAIN(v: Py) { ensure_init(); if let Ok(mut w) = PLAIN.write() { *w = v; } }
pub fn g_PLURAL_PAIRS() -> Py { ensure_init(); PLURAL_PAIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLURAL_PAIRS(v: Py) { ensure_init(); if let Ok(mut w) = PLURAL_PAIRS.write() { *w = v; } }
pub fn g_POOLS() -> Py { ensure_init(); POOLS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POOLS(v: Py) { ensure_init(); if let Ok(mut w) = POOLS.write() { *w = v; } }
pub fn g_PRACTICAL() -> Py { ensure_init(); PRACTICAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRACTICAL(v: Py) { ensure_init(); if let Ok(mut w) = PRACTICAL.write() { *w = v; } }
pub fn g_PRACTICAL_ASK() -> Py { ensure_init(); PRACTICAL_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRACTICAL_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PRACTICAL_ASK.write() { *w = v; } }
pub fn g_PREPARE() -> Py { ensure_init(); PREPARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREPARE(v: Py) { ensure_init(); if let Ok(mut w) = PREPARE.write() { *w = v; } }
pub fn g_PREPARE_OTHERWISE() -> Py { ensure_init(); PREPARE_OTHERWISE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREPARE_OTHERWISE(v: Py) { ensure_init(); if let Ok(mut w) = PREPARE_OTHERWISE.write() { *w = v; } }
pub fn g_PREY() -> Py { ensure_init(); PREY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREY(v: Py) { ensure_init(); if let Ok(mut w) = PREY.write() { *w = v; } }
pub fn g_PROCEEDINGS_ASK() -> Py { ensure_init(); PROCEEDINGS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PROCEEDINGS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PROCEEDINGS_ASK.write() { *w = v; } }
pub fn g_PRONOUN() -> Py { ensure_init(); PRONOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRONOUN(v: Py) { ensure_init(); if let Ok(mut w) = PRONOUN.write() { *w = v; } }
pub fn g_PUTTING_REGRETTED() -> Py { ensure_init(); PUTTING_REGRETTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUTTING_REGRETTED(v: Py) { ensure_init(); if let Ok(mut w) = PUTTING_REGRETTED.write() { *w = v; } }
pub fn g_PUT_DOWN() -> Py { ensure_init(); PUT_DOWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUT_DOWN(v: Py) { ensure_init(); if let Ok(mut w) = PUT_DOWN.write() { *w = v; } }
pub fn g_PUT_DOWN_REGRET_REPLIES() -> Py { ensure_init(); PUT_DOWN_REGRET_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUT_DOWN_REGRET_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = PUT_DOWN_REGRET_REPLIES.write() { *w = v; } }
pub fn g_QUANTITY() -> Py { ensure_init(); QUANTITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUANTITY(v: Py) { ensure_init(); if let Ok(mut w) = QUANTITY.write() { *w = v; } }
pub fn g_QUESTION() -> Py { ensure_init(); QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = QUESTION.write() { *w = v; } }
pub fn g_QUIETER_ASK() -> Py { ensure_init(); QUIETER_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUIETER_ASK(v: Py) { ensure_init(); if let Ok(mut w) = QUIETER_ASK.write() { *w = v; } }
pub fn g_REASSURING() -> Py { ensure_init(); REASSURING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REASSURING(v: Py) { ensure_init(); if let Ok(mut w) = REASSURING.write() { *w = v; } }
pub fn g_REFERS_TO_THEM() -> Py { ensure_init(); REFERS_TO_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFERS_TO_THEM(v: Py) { ensure_init(); if let Ok(mut w) = REFERS_TO_THEM.write() { *w = v; } }
pub fn g_RELATIVE_PET_REPLIES() -> Py { ensure_init(); RELATIVE_PET_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RELATIVE_PET_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = RELATIVE_PET_REPLIES.write() { *w = v; } }
pub fn g_REMAINS() -> Py { ensure_init(); REMAINS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REMAINS(v: Py) { ensure_init(); if let Ok(mut w) = REMAINS.write() { *w = v; } }
pub fn g_REMEMBER_NOT_WHERE() -> Py { ensure_init(); REMEMBER_NOT_WHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REMEMBER_NOT_WHERE(v: Py) { ensure_init(); if let Ok(mut w) = REMEMBER_NOT_WHERE.write() { *w = v; } }
pub fn g_REMEMBER_REPLIES() -> Py { ensure_init(); REMEMBER_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REMEMBER_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = REMEMBER_REPLIES.write() { *w = v; } }
pub fn g_RITUAL() -> Py { ensure_init(); RITUAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RITUAL(v: Py) { ensure_init(); if let Ok(mut w) = RITUAL.write() { *w = v; } }
pub fn g_RITUAL_PERSON() -> Py { ensure_init(); RITUAL_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RITUAL_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = RITUAL_PERSON.write() { *w = v; } }
pub fn g_ROLE_PHRASE() -> Py { ensure_init(); ROLE_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROLE_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = ROLE_PHRASE.write() { *w = v; } }
pub fn g_SAFETY_MODULES() -> Py { ensure_init(); SAFETY_MODULES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAFETY_MODULES(v: Py) { ensure_init(); if let Ok(mut w) = SAFETY_MODULES.write() { *w = v; } }
pub fn g_SAID_IT() -> Py { ensure_init(); SAID_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_IT(v: Py) { ensure_init(); if let Ok(mut w) = SAID_IT.write() { *w = v; } }
pub fn g_SAY_GOODBYE() -> Py { ensure_init(); SAY_GOODBYE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAY_GOODBYE(v: Py) { ensure_init(); if let Ok(mut w) = SAY_GOODBYE.write() { *w = v; } }
pub fn g_SAY_REPLIES() -> Py { ensure_init(); SAY_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAY_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = SAY_REPLIES.write() { *w = v; } }
pub fn g_SAY_THE_WORDS() -> Py { ensure_init(); SAY_THE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAY_THE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SAY_THE_WORDS.write() { *w = v; } }
pub fn g_SAY_TO_WHO() -> Py { ensure_init(); SAY_TO_WHO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAY_TO_WHO(v: Py) { ensure_init(); if let Ok(mut w) = SAY_TO_WHO.write() { *w = v; } }
pub fn g_SELF_NOUNS() -> Py { ensure_init(); SELF_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = SELF_NOUNS.write() { *w = v; } }
pub fn g_SELF_ON_THE_DRUG() -> Py { ensure_init(); SELF_ON_THE_DRUG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_ON_THE_DRUG(v: Py) { ensure_init(); if let Ok(mut w) = SELF_ON_THE_DRUG.write() { *w = v; } }
pub fn g_SELF_OPENERS() -> Py { ensure_init(); SELF_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SELF_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = SELF_OPENERS.write() { *w = v; } }
pub fn g_SERIOUS_CARE() -> Py { ensure_init(); SERIOUS_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SERIOUS_CARE(v: Py) { ensure_init(); if let Ok(mut w) = SERIOUS_CARE.write() { *w = v; } }
pub fn g_SHED() -> Py { ensure_init(); SHED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHED(v: Py) { ensure_init(); if let Ok(mut w) = SHED.write() { *w = v; } }
pub fn g_SOMEONE_DYING() -> Py { ensure_init(); SOMEONE_DYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_DYING(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_DYING.write() { *w = v; } }
pub fn g_SOMEONE_ELSES() -> Py { ensure_init(); SOMEONE_ELSES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSES.write() { *w = v; } }
pub fn g_SOMEONE_ELSES_OWN() -> Py { ensure_init(); SOMEONE_ELSES_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES_OWN(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSES_OWN.write() { *w = v; } }
pub fn g_SPEAKER_HAS_HAD_ENOUGH() -> Py { ensure_init(); SPEAKER_HAS_HAD_ENOUGH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPEAKER_HAS_HAD_ENOUGH(v: Py) { ensure_init(); if let Ok(mut w) = SPEAKER_HAS_HAD_ENOUGH.write() { *w = v; } }
pub fn g_SPEAKER_TOO() -> Py { ensure_init(); SPEAKER_TOO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPEAKER_TOO(v: Py) { ensure_init(); if let Ok(mut w) = SPEAKER_TOO.write() { *w = v; } }
pub fn g_SPEAKER_TOO_REPLIES() -> Py { ensure_init(); SPEAKER_TOO_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPEAKER_TOO_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = SPEAKER_TOO_REPLIES.write() { *w = v; } }
pub fn g_STATE_TAIL() -> Py { ensure_init(); STATE_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = STATE_TAIL.write() { *w = v; } }
pub fn g_STILL_ABOUT_THEM() -> Py { ensure_init(); STILL_ABOUT_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_ABOUT_THEM(v: Py) { ensure_init(); if let Ok(mut w) = STILL_ABOUT_THEM.write() { *w = v; } }
pub fn g_STILL_AT_THE_DOOR() -> Py { ensure_init(); STILL_AT_THE_DOOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_AT_THE_DOOR(v: Py) { ensure_init(); if let Ok(mut w) = STILL_AT_THE_DOOR.write() { *w = v; } }
pub fn g_STILL_CRY() -> Py { ensure_init(); STILL_CRY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_CRY(v: Py) { ensure_init(); if let Ok(mut w) = STILL_CRY.write() { *w = v; } }
pub fn g_STILL_CRYING() -> Py { ensure_init(); STILL_CRYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_CRYING(v: Py) { ensure_init(); if let Ok(mut w) = STILL_CRYING.write() { *w = v; } }
pub fn g_STILL_DOING_FOR_THEM() -> Py { ensure_init(); STILL_DOING_FOR_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_DOING_FOR_THEM(v: Py) { ensure_init(); if let Ok(mut w) = STILL_DOING_FOR_THEM.write() { *w = v; } }
pub fn g_STILL_KEEP() -> Py { ensure_init(); STILL_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = STILL_KEEP.write() { *w = v; } }
pub fn g_STILL_LAYING() -> Py { ensure_init(); STILL_LAYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_LAYING(v: Py) { ensure_init(); if let Ok(mut w) = STILL_LAYING.write() { *w = v; } }
pub fn g_STILL_SEEING() -> Py { ensure_init(); STILL_SEEING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_SEEING(v: Py) { ensure_init(); if let Ok(mut w) = STILL_SEEING.write() { *w = v; } }
pub fn g_STILL_WITHOUT() -> Py { ensure_init(); STILL_WITHOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_WITHOUT(v: Py) { ensure_init(); if let Ok(mut w) = STILL_WITHOUT.write() { *w = v; } }
pub fn g_STORY_INCIDENT() -> Py { ensure_init(); STORY_INCIDENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STORY_INCIDENT(v: Py) { ensure_init(); if let Ok(mut w) = STORY_INCIDENT.write() { *w = v; } }
pub fn g_STORY_MEDIUM() -> Py { ensure_init(); STORY_MEDIUM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STORY_MEDIUM(v: Py) { ensure_init(); if let Ok(mut w) = STORY_MEDIUM.write() { *w = v; } }
pub fn g_STRANGER_ANIMAL_REPLIES() -> Py { ensure_init(); STRANGER_ANIMAL_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STRANGER_ANIMAL_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = STRANGER_ANIMAL_REPLIES.write() { *w = v; } }
pub fn g_SURVIVED() -> Py { ensure_init(); SURVIVED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SURVIVED(v: Py) { ensure_init(); if let Ok(mut w) = SURVIVED.write() { *w = v; } }
pub fn g_SWAP_ASK() -> Py { ensure_init(); SWAP_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWAP_ASK(v: Py) { ensure_init(); if let Ok(mut w) = SWAP_ASK.write() { *w = v; } }
pub fn g_SWAP_MOST_WORDS() -> Py { ensure_init(); SWAP_MOST_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWAP_MOST_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SWAP_MOST_WORDS.write() { *w = v; } }
pub fn g_SYMPATHY() -> Py { ensure_init(); SYMPATHY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SYMPATHY(v: Py) { ensure_init(); if let Ok(mut w) = SYMPATHY.write() { *w = v; } }
pub fn g_TELL_CHILD() -> Py { ensure_init(); TELL_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TELL_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = TELL_CHILD.write() { *w = v; } }
pub fn g_THEIRS() -> Py { ensure_init(); THEIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIRS(v: Py) { ensure_init(); if let Ok(mut w) = THEIRS.write() { *w = v; } }
pub fn g_THEIR_HARD_TIME_REPLIES() -> Py { ensure_init(); THEIR_HARD_TIME_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_HARD_TIME_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_HARD_TIME_REPLIES.write() { *w = v; } }
pub fn g_THEIR_TEARS_IN_A_CLAUSE() -> Py { ensure_init(); THEIR_TEARS_IN_A_CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_TEARS_IN_A_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_TEARS_IN_A_CLAUSE.write() { *w = v; } }
pub fn g_THEM() -> Py { ensure_init(); THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEM(v: Py) { ensure_init(); if let Ok(mut w) = THEM.write() { *w = v; } }
pub fn g_THEMSELVES() -> Py { ensure_init(); THEMSELVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEMSELVES(v: Py) { ensure_init(); if let Ok(mut w) = THEMSELVES.write() { *w = v; } }
pub fn g_THE_CANNOT_STOP_REPLIES() -> Py { ensure_init(); THE_CANNOT_STOP_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_CANNOT_STOP_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = THE_CANNOT_STOP_REPLIES.write() { *w = v; } }
pub fn g_THE_DAYS_BEFORE() -> Py { ensure_init(); THE_DAYS_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_DAYS_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = THE_DAYS_BEFORE.write() { *w = v; } }
pub fn g_THE_DEATH() -> Py { ensure_init(); THE_DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_DEATH(v: Py) { ensure_init(); if let Ok(mut w) = THE_DEATH.write() { *w = v; } }
pub fn g_THE_GIVE_UP_NAMES() -> Py { ensure_init(); THE_GIVE_UP_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_GIVE_UP_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = THE_GIVE_UP_NAMES.write() { *w = v; } }
pub fn g_THE_HABIT_REPLIES() -> Py { ensure_init(); THE_HABIT_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_HABIT_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = THE_HABIT_REPLIES.write() { *w = v; } }
pub fn g_THE_HOUSE_REPLIES() -> Py { ensure_init(); THE_HOUSE_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_HOUSE_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = THE_HOUSE_REPLIES.write() { *w = v; } }
pub fn g_THE_JOBS_FOR_THEM_REPLIES() -> Py { ensure_init(); THE_JOBS_FOR_THEM_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_JOBS_FOR_THEM_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = THE_JOBS_FOR_THEM_REPLIES.write() { *w = v; } }
pub fn g_THE_ROUTINE_REPLIES() -> Py { ensure_init(); THE_ROUTINE_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_ROUTINE_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = THE_ROUTINE_REPLIES.write() { *w = v; } }
pub fn g_THINGS() -> Py { ensure_init(); THINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THINGS(v: Py) { ensure_init(); if let Ok(mut w) = THINGS.write() { *w = v; } }
pub fn g_THING_DEATH() -> Py { ensure_init(); THING_DEATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_DEATH(v: Py) { ensure_init(); if let Ok(mut w) = THING_DEATH.write() { *w = v; } }
pub fn g_THING_ROLE() -> Py { ensure_init(); THING_ROLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_ROLE(v: Py) { ensure_init(); if let Ok(mut w) = THING_ROLE.write() { *w = v; } }
pub fn g_THIS_ONE() -> Py { ensure_init(); THIS_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THIS_ONE(v: Py) { ensure_init(); if let Ok(mut w) = THIS_ONE.write() { *w = v; } }
pub fn g_THREAD_TOPICS() -> Py { ensure_init(); THREAD_TOPICS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THREAD_TOPICS(v: Py) { ensure_init(); if let Ok(mut w) = THREAD_TOPICS.write() { *w = v; } }
pub fn g_THREAD_TURNS() -> Py { ensure_init(); THREAD_TURNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THREAD_TURNS(v: Py) { ensure_init(); if let Ok(mut w) = THREAD_TURNS.write() { *w = v; } }
pub fn g_TIME_OF_DAY() -> Py { ensure_init(); TIME_OF_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_OF_DAY(v: Py) { ensure_init(); if let Ok(mut w) = TIME_OF_DAY.write() { *w = v; } }
pub fn g_TIME_SINCE_REPLIES() -> Py { ensure_init(); TIME_SINCE_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_SINCE_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = TIME_SINCE_REPLIES.write() { *w = v; } }
pub fn g_TOLD_US() -> Py { ensure_init(); TOLD_US.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOLD_US(v: Py) { ensure_init(); if let Ok(mut w) = TOLD_US.write() { *w = v; } }
pub fn g_TOOK() -> Py { ensure_init(); TOOK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOOK(v: Py) { ensure_init(); if let Ok(mut w) = TOOK.write() { *w = v; } }
pub fn g_TRADITION() -> Py { ensure_init(); TRADITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRADITION(v: Py) { ensure_init(); if let Ok(mut w) = TRADITION.write() { *w = v; } }
pub fn g_TREATMENT_STOPPED() -> Py { ensure_init(); TREATMENT_STOPPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TREATMENT_STOPPED(v: Py) { ensure_init(); if let Ok(mut w) = TREATMENT_STOPPED.write() { *w = v; } }
pub fn g_TWO_WAY_ROLES() -> Py { ensure_init(); TWO_WAY_ROLES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TWO_WAY_ROLES(v: Py) { ensure_init(); if let Ok(mut w) = TWO_WAY_ROLES.write() { *w = v; } }
pub fn g_TWO_WORD_PETS() -> Py { ensure_init(); TWO_WORD_PETS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TWO_WORD_PETS(v: Py) { ensure_init(); if let Ok(mut w) = TWO_WORD_PETS.write() { *w = v; } }
pub fn g_UNNAMED() -> Py { ensure_init(); UNNAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UNNAMED(v: Py) { ensure_init(); if let Ok(mut w) = UNNAMED.write() { *w = v; } }
pub fn g_VERB_OBJECT() -> Py { ensure_init(); VERB_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERB_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = VERB_OBJECT.write() { *w = v; } }
pub fn g_VISIT_TARGETS() -> Py { ensure_init(); VISIT_TARGETS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VISIT_TARGETS(v: Py) { ensure_init(); if let Ok(mut w) = VISIT_TARGETS.write() { *w = v; } }
pub fn g_WAKE() -> Py { ensure_init(); WAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAKE(v: Py) { ensure_init(); if let Ok(mut w) = WAKE.write() { *w = v; } }
pub fn g_WEAR() -> Py { ensure_init(); WEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEAR(v: Py) { ensure_init(); if let Ok(mut w) = WEAR.write() { *w = v; } }
pub fn g_WHEN_ASK() -> Py { ensure_init(); WHEN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_ASK.write() { *w = v; } }
pub fn g_WHEN_WORDS() -> Py { ensure_init(); WHEN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_WORDS.write() { *w = v; } }
pub fn g_WHERE_IS_THING() -> Py { ensure_init(); WHERE_IS_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_IS_THING(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_IS_THING.write() { *w = v; } }
pub fn g_WHERE_IS_THING_BARE() -> Py { ensure_init(); WHERE_IS_THING_BARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_IS_THING_BARE(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_IS_THING_BARE.write() { *w = v; } }
pub fn g_WHERE_I_PUT_IT() -> Py { ensure_init(); WHERE_I_PUT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_I_PUT_IT(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_I_PUT_IT.write() { *w = v; } }
pub fn g_WISH_FRAME() -> Py { ensure_init(); WISH_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WISH_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = WISH_FRAME.write() { *w = v; } }
pub fn g_WITHOUT() -> Py { ensure_init(); WITHOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITHOUT(v: Py) { ensure_init(); if let Ok(mut w) = WITHOUT.write() { *w = v; } }
pub fn g_WOULD_HAVE() -> Py { ensure_init(); WOULD_HAVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WOULD_HAVE(v: Py) { ensure_init(); if let Ok(mut w) = WOULD_HAVE.write() { *w = v; } }
pub fn g_WOULD_HAVE_BEEN() -> Py { ensure_init(); WOULD_HAVE_BEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WOULD_HAVE_BEEN(v: Py) { ensure_init(); if let Ok(mut w) = WOULD_HAVE_BEEN.write() { *w = v; } }
pub fn g_WRITE_WORD() -> Py { ensure_init(); WRITE_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRITE_WORD(v: Py) { ensure_init(); if let Ok(mut w) = WRITE_WORD.write() { *w = v; } }
pub fn g_YEAR_OLD() -> Py { ensure_init(); YEAR_OLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YEAR_OLD(v: Py) { ensure_init(); if let Ok(mut w) = YEAR_OLD.write() { *w = v; } }
pub fn g__A_DAY_PART() -> Py { ensure_init(); _A_DAY_PART.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DAY_PART(v: Py) { ensure_init(); if let Ok(mut w) = _A_DAY_PART.write() { *w = v; } }
pub fn g__BILL_MONEY() -> Py { ensure_init(); _BILL_MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BILL_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = _BILL_MONEY.write() { *w = v; } }
pub fn g__BILL_SUBJECT() -> Py { ensure_init(); _BILL_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BILL_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = _BILL_SUBJECT.write() { *w = v; } }
pub fn g__BURIAL_SIDE() -> Py { ensure_init(); _BURIAL_SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BURIAL_SIDE(v: Py) { ensure_init(); if let Ok(mut w) = _BURIAL_SIDE.write() { *w = v; } }
pub fn g__CAN_BE_DYING() -> Py { ensure_init(); _CAN_BE_DYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CAN_BE_DYING(v: Py) { ensure_init(); if let Ok(mut w) = _CAN_BE_DYING.write() { *w = v; } }
pub fn g__CONTRACTED() -> Py { ensure_init(); _CONTRACTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CONTRACTED(v: Py) { ensure_init(); if let Ok(mut w) = _CONTRACTED.write() { *w = v; } }
pub fn g__COST_NOUN() -> Py { ensure_init(); _COST_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COST_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = _COST_NOUN.write() { *w = v; } }
pub fn g__CREMATION_SIDE() -> Py { ensure_init(); _CREMATION_SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CREMATION_SIDE(v: Py) { ensure_init(); if let Ok(mut w) = _CREMATION_SIDE.write() { *w = v; } }
pub fn g__DEATH_IN_LINE() -> Py { ensure_init(); _DEATH_IN_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEATH_IN_LINE(v: Py) { ensure_init(); if let Ok(mut w) = _DEATH_IN_LINE.write() { *w = v; } }
pub fn g__FUNERAL_NOUN() -> Py { ensure_init(); _FUNERAL_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FUNERAL_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = _FUNERAL_NOUN.write() { *w = v; } }
pub fn g__HELPERS() -> Py { ensure_init(); _HELPERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HELPERS(v: Py) { ensure_init(); if let Ok(mut w) = _HELPERS.write() { *w = v; } }
pub fn g__NOT_A_BILL_THREAD() -> Py { ensure_init(); _NOT_A_BILL_THREAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_BILL_THREAD(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_BILL_THREAD.write() { *w = v; } }
pub fn g__NOT_A_CONTRACTION() -> Py { ensure_init(); _NOT_A_CONTRACTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_CONTRACTION(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_CONTRACTION.write() { *w = v; } }
pub fn g__PEOPLE_PRONOUNS() -> Py { ensure_init(); _PEOPLE_PRONOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PEOPLE_PRONOUNS(v: Py) { ensure_init(); if let Ok(mut w) = _PEOPLE_PRONOUNS.write() { *w = v; } }
pub fn g__PRONOUN_MONEY_ASK() -> Py { ensure_init(); _PRONOUN_MONEY_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PRONOUN_MONEY_ASK(v: Py) { ensure_init(); if let Ok(mut w) = _PRONOUN_MONEY_ASK.write() { *w = v; } }
pub fn g__SAID_DYING() -> Py { ensure_init(); _SAID_DYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAID_DYING(v: Py) { ensure_init(); if let Ok(mut w) = _SAID_DYING.write() { *w = v; } }
pub fn g__SPAN_COUNT() -> Py { ensure_init(); _SPAN_COUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SPAN_COUNT(v: Py) { ensure_init(); if let Ok(mut w) = _SPAN_COUNT.write() { *w = v; } }
pub fn g__THE_ESTATE() -> Py { ensure_init(); _THE_ESTATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_ESTATE(v: Py) { ensure_init(); if let Ok(mut w) = _THE_ESTATE.write() { *w = v; } }
pub fn g__THE_ONE_WHO_LEFT() -> Py { ensure_init(); _THE_ONE_WHO_LEFT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_ONE_WHO_LEFT(v: Py) { ensure_init(); if let Ok(mut w) = _THE_ONE_WHO_LEFT.write() { *w = v; } }
pub fn g__WORKING_WITH_THE_DYING() -> Py { ensure_init(); _WORKING_WITH_THE_DYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WORKING_WITH_THE_DYING(v: Py) { ensure_init(); if let Ok(mut w) = _WORKING_WITH_THE_DYING.write() { *w = v; } }
pub fn g__dream() -> Py { ensure_init(); _dream.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__dream(v: Py) { ensure_init(); if let Ok(mut w) = _dream.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A death or a loss is never answered as small talk.\n\n\"My dad died in march\" answered with \"Got it.\" is the kind of reply STYLE.md rules out: it is\ncareless, and it is also wrong — a death is not a thing to note and move on from. This reads a\ndeath in the line and picks the reply the situation asks for:\n\n* the user's own loss -> plain sympathy, and an offer of company or a practical hand;\n* someone else's loss (\"my friend's mum died\") -> what to say to that person, because that is\n  the question behind the line even when it is phrased as a statement;\n* a condolence card or message -> words they can actually use;\n* a question about grief itself (\"is it normal to still cry\", \"i can't sleep\") -> a short,\n  normalising answer and the one useful pointer (their GP if it stays heavy).\n\nOnly a real death fires this: \"my legs are dead\", \"dead against it\", \"my phone died\", \"nearly\ndied laughing\", \"a dead end\" and \"the deadline\" are ordinary lines and must pass, so the dead\nwords need a person or a pet behind them, and never an object right before them.\n\nThe pools vary and never repeat, like the fallback's (`_choose` in bot/compose/fallback.py).\n");
    let __try1 = exc::catch(|| {
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
    s__dream(Py::None);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    s_PEOPLE(call_builtin("frozenset", &[py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mam"), py_str("mother"), py_str("dad"), py_str("father"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandmother"), py_str("grandad"), py_str("granddad"), py_str("grandpa"), py_str("grandfather"), py_str("grandparents"), py_str("parents"), py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("boy"), py_str("girl"), py_str("brother"), py_str("sister"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("friend"), py_str("friends"), py_str("mate"), py_str("mates"), py_str("neighbour"), py_str("neighbor"), py_str("neighbours"), py_str("neighbors"), py_str("colleague"), py_str("colleagues"), py_str("aunt"), py_str("auntie"), py_str("uncle"), py_str("cousin"), py_str("niece"), py_str("nephew"), py_str("family"), py_str("relative"), py_str("relatives"), py_str("stepdad"), py_str("stepmum"), py_str("godmother"), py_str("godfather"), py_str("in-laws"), py_str("mil"), py_str("fil"), py_str("parent"), py_str("sibling"), py_str("siblings"), py_str("spouse"), py_str("spouses"), py_str("boyfriend"), py_str("girlfriend"), py_str("boyfriends"), py_str("girlfriends"), py_str("fiance"), py_str("fiancee"), py_str("fiancé"), py_str("fiancée"), py_str("fiances"), py_str("halfbrother"), py_str("halfsister"), py_str("ex"), py_str("stepson"), py_str("stepsons"), py_str("stepdaughter"), py_str("stepdaughters"), py_str("stepchild"), py_str("stepchildren"), py_str("stepbrother"), py_str("stepsister"), py_str("godson"), py_str("godsons"), py_str("goddaughter"), py_str("goddaughters"), py_str("godchild"), py_str("godchildren"), py_str("grandson"), py_str("grandsons"), py_str("granddaughter"), py_str("granddaughters"), py_str("grandchild"), py_str("grandchildren"), py_str("grandkids"), py_str("twin"), py_str("twins"), py_str("carer"), py_str("carers"), py_str("caregiver"), py_str("caregivers"), py_str("childminder"), py_str("guardian"), py_str("flatmate"), py_str("flatmates"), py_str("housemate"), py_str("housemates"), py_str("lodger"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("puppy"), py_str("kitten"), py_str("rabbit"), py_str("hamster"), py_str("budgie"), py_str("parrot"), py_str("horse"), py_str("pony"), py_str("pet"), py_str("pets"), py_str("tortoise"), py_str("ferret"), py_str("guinea"), py_str("fish"), py_str("fishes"), py_str("goldfish"), py_str("snake"), py_str("lizard"), py_str("gerbil")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSE_PEOPLE(call_builtin("frozenset", &[py_tuple(vec![py_str("carer"), py_str("carers"), py_str("caregiver"), py_str("caregivers"), py_str("nurse"), py_str("nurses"), py_str("doctor"), py_str("doctors"), py_str("gp"), py_str("gps"), py_str("surgeon"), py_str("surgeons"), py_str("midwife"), py_str("midwives"), py_str("therapist"), py_str("therapists"), py_str("counsellor"), py_str("counsellors"), py_str("counselor"), py_str("counselors"), py_str("physio"), py_str("physiotherapist"), py_str("psychiatrist"), py_str("childminder"), py_str("childminders"), py_str("babysitter"), py_str("babysitters"), py_str("nanny"), py_str("nannies"), py_str("cleaner"), py_str("cleaners"), py_str("lodger"), py_str("lodgers"), py_str("housemate"), py_str("housemates"), py_str("flatmate"), py_str("flatmates"), py_str("classmate"), py_str("classmates"), py_str("schoolfriend"), py_str("tutor"), py_str("tutors"), py_str("teacher"), py_str("teachers"), py_str("manager"), py_str("managers"), py_str("priest"), py_str("vicar"), py_str("pastor"), py_str("rabbi"), py_str("imam"), py_str("barber"), py_str("hairdresser"), py_str("warden"), py_str("mentor"), py_str("worker"), py_str("workers")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ROLE_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:home\\s+(?:help|carer)|care\\s+assistant|foster\\s+carer|support\\s+staff|(?:duty|night|live-in)\\s+(?:nurse|carer))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PEOPLE(bitor(&g_PEOPLE(), &g_CLOSE_PEOPLE()));
    s_THING_ROLE(stdlib_call("re", "compile", &[py_str("\\b(?:vacuum|oven|drain|steam|carpet|dishwasher|pipe|grill|barbecue|bbq|floor|gutter|jet|pressure|upholstery|chimney)\\s+cleaners?\\b|\\bnanny\\s+goats?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TWO_WAY_ROLES(call_builtin("frozenset", &[py_tuple(vec![py_str("cleaner"), py_str("cleaners"), py_str("nanny"), py_str("nannies")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIRS(call_builtin("frozenset", &[py_tuple(vec![py_str("badge"), py_str("badges"), py_str("card"), py_str("cards"), py_str("allowance"), py_str("funding"), py_str("letter"), py_str("letters"), py_str("note"), py_str("notes"), py_str("form"), py_str("forms"), py_str("paperwork"), py_str("payment"), py_str("payments"), py_str("appointment"), py_str("appointments"), py_str("referral"), py_str("referrals"), py_str("rota"), py_str("timesheet"), py_str("pass"), py_str("application"), py_str("number"), py_str("budget"), py_str("assessment"), py_str("plan"), py_str("diary"), py_str("file"), py_str("files"), py_str("folder"), py_str("prescription"), py_str("receipt"), py_str("invoice"), py_str("statement"), py_str("booking"), py_str("bookings"), py_str("certificate"), py_str("report"), py_str("results"), py_str("record"), py_str("records"), py_str("pin"), py_str("login"), py_str("password"), py_str("email"), py_str("address"), py_str("ticket"), py_str("tickets")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN(stdlib_call("re", "compile", &[py_str("\\b(passed away|funeral|funerals|burial|cremation|cremated|cremate|grief|grieving|grieve|bereaved|bereavement|widow|widowed|mourning|condolence|condolences|eulogy|memorial service|post mortem|in memoriam)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PRICE(stdlib_call("re", "compile", &[py_str("\\b(?:cost|costs|costing|price|prices|pricing|cheap|cheapest|expensive|afford|affordable|budget|pay|quote|quotes)\\b|\\bhow\\s+much\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PROCEEDINGS_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how|what|which|can|could|should|do|does|any|tips?|ideas?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THIS_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:the|this|that|our|their|his|her|its|today|tomorrow|yesterday)\\s+(?:\\w+\\s+){0,2}(?:funeral|burial|cremation|wake|service|viewing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEATH_HAPPENED(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|dies|dead|death|deaths|passed\\s+away|passed\\s+on|gone|killed|lost|losing|buried|bury|burying|ashes|widow|widowed|bereaved|bereavement|grief|grieving|mourning|condolence|condolences|eulogy|sympathy|miss|missing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTS(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:no|nope|sorry|apologies|actually|wait|hold on|oops)[\\s,.!-]+)*(?:i\\s+(?:mean|meant|said)(?:\\s+to\\s+say)?|(?:you\\s+)?mean|i\\s+did\\s+mean)[\\s,.!-]+(?P<rest>\\S.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WAKE(stdlib_call("re", "compile", &[py_str("\\b(?:his|her|their|the|a|mum's|dad's|nan's|grandad's|gran's)\\s+wake(?!\\s*(?:up|me|him|her|them|you|us|at|before|earlier|by|and|of)\\b)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BURIED(stdlib_call("re", "compile", &[py_str("\\bburied\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BURIED_OBJECT(py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("some"), py_str("old"), py_str("new"), py_str("these"), py_str("those"), py_str("his"), py_str("her"), py_str("my"), py_str("our"), py_str("their"), py_str("your")]));
    s_BURIED_ADJECTIVE(py_tuple(vec![py_str("old"), py_str("poor"), py_str("little"), py_str("beloved"), py_str("dear"), py_str("sweet"), py_str("big"), py_str("lovely"), py_str("late"), py_str("much"), py_str("loved"), py_str("own"), py_str("first"), py_str("precious"), py_str("wee"), py_str("new")]));
    s_DEATH_NOUN(stdlib_call("re", "compile", &[py_str("\\b(?:a|the|this|that|his|her|their|your|my|our)\\s+deaths?\\b|\\b[a-z]+(?:'s|s'|s)\\s+deaths?\\b|\\b(?:two|three|several)\\s+deaths\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|your|our)\\s+(?:[a-z]+(?:'s|s'|s)\\s+)?deaths?\\b|\\b([a-z]+)(?:'s|s'|s)\\s+deaths?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH_CUE(stdlib_call("re", "compile", &[py_str("\\b(?:guilty|cope|coping|cant stop|get over|getting over|anniversary|funeral|grief|grieving|sudden|suddenly|wasnt there|wasn't there|angry|sad|numb|miss|misses|missed|cry|crying|cried|wake|ashes|burial|memorial|condolence|mourning|hit me|hit him|hit her|in the family|on the news|normal to feel|keep thinking|cant sleep|to terms|how do i|what do i say|still)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH_COMPOUND(stdlib_call("re", "compile", &[py_str("\\bdeath\\s+(?!(?:of|was|is|in|at|on|has|had|came|come|hit|hits|left|changed|anniversary|day|days|month|months|year|years|certificate|notice|and|or|itself|row|toll|threat|bed|i|we|he|she|they|you|my|our|his|her|their|me|us|them|really|just|still|so|but|if|when|while|after|before|because|that|this|everything|nothing)\\b)[a-z]+")]).unwrap_or_else(|e| exc::raise(e)));
    s_FICTION(stdlib_call("re", "compile", &[py_str("\\b(?:film|films|movie|movies|book|books|novel|play|episode|series|documentary|characters?|scene|plot|chapter|show|netflix|tv|actor|actress|game|anime|comic|season|trailer|storyline)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_WORRY(stdlib_call("re", "compile", &[py_str("\\b(?:my|your|our)\\s+(?:own\\s+)?death\\b|\\bdeath anxiety\\b|\\bfear of death\\b|\\b(?:scared|afraid) of death\\b|\\bdeath itself\\b|\\bthink(?:ing)? about death\\b|\\b(?:what|when)\\s+(?:is|comes after|happens after)\\s+death\\b|\\bis death\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THINGS(call_builtin("frozenset", &[py_tuple(vec![py_str("laptop"), py_str("laptops"), py_str("phone"), py_str("phones"), py_str("car"), py_str("cars"), py_str("battery"), py_str("batteries"), py_str("boiler"), py_str("fridge"), py_str("freezer"), py_str("telly"), py_str("tv"), py_str("printer"), py_str("houseplant"), py_str("houseplants"), py_str("plant"), py_str("plants"), py_str("tree"), py_str("trees"), py_str("lawn"), py_str("garden"), py_str("business"), py_str("career"), py_str("job"), py_str("pub"), py_str("shop"), py_str("club"), py_str("brand"), py_str("game"), py_str("team"), py_str("band"), py_str("party"), py_str("project"), py_str("dream"), py_str("holiday"), py_str("marriage"), py_str("engine"), py_str("phone"), py_str("internet"), py_str("radio"), py_str("cinema"), py_str("print"), py_str("high")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:cat|cats|dog|dogs|puppy|puppies|kitten|kittens|rabbit|rabbits|hamster|hamsters|budgie|budgies|parrot|parrots|horse|horses|pony|ponies|guinea|goldfish|fish|pet|pets)(?:'s|s'|s)?\\s+deaths?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_DEATH(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_THINGS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")(?:'s|s'|s)?\\s+deaths?\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH_OF_OWNER(stdlib_call("re", "compile", &[py_str("\\bdeaths?\\s+of\\s+(?:the|my|our|a|an|his|her|their|your)\\s+((?:[a-z]+\\s+){0,2}[a-z]+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DIED(stdlib_call("re", "compile", &[py_str("\\b(died|dying|dead|dies)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DYING(stdlib_call("re", "compile", &[py_str("\\b(died|dies)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_DOWN(stdlib_call("re", "compile", &[py_str("\\bput\\s+(?:her|him|it|(?:my|our|the)\\s+\\w+)\\s+(?:down|to sleep)\\b|\\bputting\\s+(?:her|him|it|(?:my|our|the)\\s+\\w+)\\s+(?:down|to sleep)\\b|\\bput\\s+(?:down|to sleep)\\b|\\bhad\\s+(?:\\w+\\s+){0,3}?put\\s+(?:down|to sleep)\\b|\\b(?:was|were|been)\\s+put\\s+(?:down|to sleep)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("puppy"), py_str("puppies"), py_str("kitten"), py_str("kittens"), py_str("rabbit"), py_str("hamster"), py_str("budgie"), py_str("parrot"), py_str("horse"), py_str("pony"), py_str("pet"), py_str("pets"), py_str("tortoise"), py_str("ferret"), py_str("guinea"), py_str("hen"), py_str("chicken"), py_str("rat"), py_str("mouse"), py_str("gerbil"), py_str("lurcher"), py_str("greyhound"), py_str("collie"), py_str("labrador"), py_str("fish"), py_str("fishes"), py_str("goldfish"), py_str("snake"), py_str("lizard")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PEOPLE_PHRASES(py_tuple(vec![py_tuple(vec![py_str("other"), py_str("half")]), py_tuple(vec![py_str("other"), py_str("halves")]), py_tuple(vec![py_str("support"), py_str("worker")]), py_tuple(vec![py_str("support"), py_str("workers")]), py_tuple(vec![py_str("care"), py_str("worker")]), py_tuple(vec![py_str("care"), py_str("workers")]), py_tuple(vec![py_str("significant"), py_str("other")]), py_tuple(vec![py_str("home"), py_str("carer")]), py_tuple(vec![py_str("home"), py_str("carers")])]));
    s_HUMAN_WORDS(sub(&g_PEOPLE(), &g_PET_WORDS()));
    s_DEATHABLE(bitor(&g_PEOPLE(), &g_PET_WORDS()));
    s_MACHINE_ANIMALS(call_builtin("frozenset", &[py_tuple(vec![py_str("mouse"), py_str("mice")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MISADVENTURE(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|is|are|got|been|being|had been)\\s+(?:run\\s+over|knocked\\s+(?:down|over)|drowned)\\b|\\b(?:run|knocked)\\s+(?:over|down)\\s+by\\s+(?:a|the|an)?\\s*\\w+|\\bdrowned\\b|\\b(?:was|were|got|been)\\s+electrocuted\\b(?=[^.?!]{0,45}\\b(?:died|dead|killed|didn'?t\\s+make\\s+it|did\\s+not\\s+make\\s+it|passed\\s+(?:away|on)|funeral|buried|pronounced)\\b)|\\bdied\\s+(?:in|after)\\s+(?:a\\s+|the\\s+)?(?:accident|crash|fire|fit|seizure)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SURVIVED(stdlib_call("re", "compile", &[py_str("\\bsurvived\\b|\\bstill alive\\b|\\b(?:is|was|seems|doing)\\s+(?:ok|okay|alright|fine)\\b|\\bbut\\s+(?:he|she|they|it|we|i)(?:'s|s| is| was)?\\s+(?:ok|okay|alright|fine|alive)\\b|\\bin (?:the )?hospital\\b|\\brecovering\\b|\\bstable\\b|\\bjust bruises\\b|\\bbroke (?:his|her|their|a|my)\\b|\\bstitches\\b|\\bwalked\\s+(?:away|off)\\b|\\bno\\s+harm\\b|\\bunhurt\\b|\\bnot\\s+(?:a\\s+)?scratch\\b|\\bnot\\s+(?:hurt|injured|serious)\\b|\\bno\\s+(?:injuries|injury|damage)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSS_CONTEXT(py_tuple(vec![py_str("had to"), py_str("have to"), py_str("put to sleep"), py_str("vet"), py_str("euthanis"), py_str("last week"), py_str("last night"), py_str("yesterday"), py_str("we had"), py_str("i had"), py_str("it was time")]));
    s_OWNED(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their)\\s+((?:\\w+\\s+){0,2}\\w+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_VERB_OBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:drowned|run\\s+over|knocked\\s+(?:down|over))\\s+(?:her|his|my|our|their|its?|the|a|an)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KNOCKED_BY(stdlib_call("re", "compile", &[py_str("\\b(?:run\\s+over|knocked\\s+(?:down|over))\\s+by\\s+(?:a|the|an)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:died|dead|killed|didn'?t\\s+make\\s+it|did\\s+not\\s+make\\s+it|lost|passing|passed|gone|funeral|put\\s+to\\s+sleep|euthanis\\w*)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST(stdlib_call("re", "compile", &[py_str("\\b(lost|we lost|i lost|he lost|she lost|they lost)\\s+(?:my|our|his|her|their)\\s+([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST_BARE(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|he|she|they)\\s+lost\\s+(?!(?:my|our|his|her|their|the|it|a|an|that|this|him|them)\\b)([a-z]+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSE_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:lose|loses)\\s+(?:(?:my|our|his|her|their|the|a|an|this|that)\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST_FORMS(py_tuple(vec![py_tuple(vec![g_LOST(), Py::Int(2)]), py_tuple(vec![g_LOST_BARE(), Py::Int(1)]), py_tuple(vec![g_LOSE_A_PERSON(), Py::Int(1)])]));
    s_LOSS_OF(stdlib_call("re", "compile", &[py_str("\\b(?:the\\s+|a\\s+|any\\s+)?loss\\s+of\\s+(?:my|our|his|her|their)\\s+([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSING(stdlib_call("re", "compile", &[py_str("\\blosing\\s+(?:my|our|his|her|their|a|an|the)\\s+([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSE(call_builtin("frozenset", &[py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mam"), py_str("mother"), py_str("dad"), py_str("father"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandmother"), py_str("grandad"), py_str("granddad"), py_str("grandpa"), py_str("grandfather"), py_str("son"), py_str("daughter"), py_str("child"), py_str("baby"), py_str("brother"), py_str("sister"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("stepdad"), py_str("stepmum"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("niece"), py_str("nephew"), py_str("parents")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEWS_OF(stdlib_call("re", "compile", &[py_str("\\b(?:news|thought)\\s+of\\s+(?:my|our|his|her|their)\\s+([a-z]+)[^.]{0,24}?\\b(?:hit|hits|hurt|hurts|broke|breaks|is\\s+hard|was\\s+hard)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSS_GRIEF_CUE(stdlib_call("re", "compile", &[py_str("\\b(?:since|after|still|every|cant|couldnt|miss|missed|cry|crying|cried|grief|grieving|funeral|bereave\\w*|yesterday|anniversary|last\\s+(?:year|month|week|night|christmas|summer|winter)|(?:in|at|on)\\s+(?:january|february|march|april|may|june|july|august|september|october|november|december|christmas|christmas\\s+eve|boxing\\s+day|easter|new\\s+year))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST_IN_A_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:in|at|on|into|inside|outside|near|round|around)\\s+(?:the\\s+|a\\s+|an\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACES(call_builtin("frozenset", &[mcall(&py_str("\nsupermarket shop shops store town city park parks playground beach seafront street streets\nroad motorway station airport bus train tube metro queue market mall museum gallery zoo fair\nfestival concert woods forest hill hills garden gardens pool lido crowd crowds field fields\narcade campsite caravan centre center precinct pier promenade high street roundabout bus stop\ncoach tram ferry harbour harbor dock docks\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_UNNAMED(call_builtin("frozenset", &[py_tuple(vec![py_str("someone"), py_str("somebody"), py_str("anyone"), py_str("everyone"), py_str("people"), py_str("person")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEDIA(call_builtin("frozenset", &[py_tuple(vec![py_str("show"), py_str("shows"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("series"), py_str("episode"), py_str("episodes"), py_str("game"), py_str("games"), py_str("book"), py_str("books"), py_str("soap"), py_str("documentary"), py_str("telly"), py_str("tv")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST_SELF(call_builtin("frozenset", &[py_tuple(vec![py_str("appetite"), py_str("sleep"), py_str("confidence"), py_str("interest"), py_str("motivation"), py_str("energy"), py_str("joy"), py_str("focus"), py_str("weight"), py_str("voice"), py_str("self"), py_str("myself"), py_str("herself"), py_str("himself"), py_str("themselves")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PREY(stdlib_call("re", "compile", &[py_str("\\b(?:a\\s+|the\\s+|another\\s+)?(?:dead|dying|half\\s+dead)\\s+(?:bird|birds|mouse|mice|rat|rats|vole|voles|rabbit|rabbits|squirrel|squirrels|pigeon|pigeons|frog|frogs|hedgehog|hedgehogs|sparrow|sparrows|blackbird|blackbirds|chick|chicks|goldfish|fish)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HUNTING(stdlib_call("re", "compile", &[py_str("\\b(bring|brings|bringing|brought|drag|drags|dragging|dragged|catch|catches|catching|caught|kill|kills|killed|killing|left|leaves|leaving|found|finds|dropping|dropped|deposit|deposits|present|presents)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST_THE(stdlib_call("re", "compile", &[py_str("\\b(?:we|i|he|she|they)\\s+lost\\s+the\\s+([a-z]+)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST_THE_FOUND(stdlib_call("re", "compile", &[py_str("\\b(?:found|got)\\s+(?:him|her|them)\\b|\\b(?:in|at|near|outside|inside|by)\\s+the\\s+(?:shop|shops|park|supermarket|street|station|queue|mall|woods|beach|playground|car\\s*park|bus|train|airport|market)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_CRY(stdlib_call("re", "compile", &[py_str("\\b(?:still|always|every|keep|keeps)\\b[^.]{0,24}\\b(?:cry|crying|cries|tears|gutted)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OTHER_WEEPS(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|the)\\s+(?:[a-z']+\\s+){0,2}(?:keeps?|kept|is|are|was|were|has|have|had|still|always|constantly|every|starts?|started|won'?t|wont)\\b(?:(?!\\b(?:i|we)\\b)[^.]){0,40}\\b(?:cry|crying|cries|tears|sob\\w*)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_TEARS_IN_A_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:and|so|but|then)\\s+(?:he|she|they|it|the\\s+[a-z]+)\\b[^.]{0,24}\\b(?:cry|cries|cried|crying|sob\\w*|weeps?|weeping|tears)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH_AS_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:a|any|the)\\s+death\\b|\\b[a-z]+'?s\\s+death\\b|\\b(?:his|her|their|your|my|our)\\s+death\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DEATH(stdlib_call("re", "compile", &[py_str("\\bdeath\\s+(?:penalty|row|metal|stare|trap|wish|cab|valley|rate|rates|bed|note|star|toll|scene|threat|match|laptop|phone|duty|tax|benefit|certificate|notice|inquest|camp)\\b|\\bto\\s+death\\b|\\bdeath of (?:me|him|her|us|them|a\\s+salesman)\\b|\\bblack death\\b|\\bsudden death\\b|\\bdeath in (?:paradise|venice)\\b|\\b(?:death|deaths)\\s+(?:per|rate|rates|toll|count|figures)\\b|\\bdeath of (?:the|my|our|a)\\s+(?:[a-z]+\\s+){0,2}?(?:high street|cinema|print|telly|tv|radio|music|pub|shop|business|club|scene|brand|houseplant|plant|tree|lawn|car)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TOOK(stdlib_call("re", "compile", &[py_str("\\b(took|taken|take)\\s+(?:his|her|their)\\s+own life\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KILLED(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|got|been|being|is|are|had been)\\s+killed\\b|\\bkilled\\s+(?:in|by)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KILLED_IN(stdlib_call("re", "compile", &[py_str("\\bkilled\\s+(?:in|on|during|at)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STORY_MEDIUM(call_builtin("frozenset", &[mcall(&py_str("film films movie movies book books novel novels play episode\n    episodes series documentary scene chapter show netflix tv game games anime comic comics\n    season trailer storyline soap cartoon quiz"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STORY_INCIDENT(call_builtin("frozenset", &[mcall(&py_str("crash accident road motorway car bike motorbike lorry truck bus\n    train plane bomb blast explosion shooting shot stabbing stabbing attack fire house\n    hospital childbirth home work"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GAME_TITLES(stdlib_call("re", "compile", &[py_str("\\b(?:call\\s+of\\s+duty|fortnite|minecraft|roblox|fifa|halo|overwatch|zelda|mario|pokemon|among\\s+us|gta|grand\\s+theft\\s+auto|world\\s+of\\s+warcraft|league\\s+of\\s+legends|apex\\s+legends|counter\\s+strike|pubg|rocket\\s+league|skyrim|tetris|pac\\s*man|street\\s+fighter|mortal\\s+kombat)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WOULD_HAVE_BEEN(stdlib_call("re", "compile", &[py_str("\\bwould(?:'ve| have)\\s+been\\s+(?:\\d{1,3}|(?:her|his|their)\\s+\\d{1,3})\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_ON(py_str("passed on"));
    s_PASSED_ON_SUBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("him"), py_str("her"), py_str("them")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_ON_OBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("it"), py_str("them"), py_str("some"), py_str("any"), py_str("all"), py_str("another"), py_str("one"), py_str("to")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MONTHS(call_builtin("frozenset", &[py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_ALONE(py_str("passed"));
    s_PASSED_ALONE_TAIL(call_builtin("frozenset", &[py_tuple(vec![py_str("away"), py_str("on"), py_str("out"), py_str("by"), py_str("over"), py_str("up"), py_str("back")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_ALONE_HANDED(call_builtin("frozenset", &[py_tuple(vec![py_str("me"), py_str("him"), py_str("her"), py_str("us"), py_str("them"), py_str("it"), py_str("you")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_ALONE_NOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("gas"), py_str("wind"), py_str("water"), py_str("blood"), py_str("comment"), py_str("comments"), py_str("remark"), py_str("remarks"), py_str("judgement"), py_str("judgment"), py_str("sentence"), py_str("motion"), py_str("muster"), py_str("note"), py_str("notes"), py_str("word"), py_str("words"), py_str("message"), py_str("messages"), py_str("memo"), py_str("review"), py_str("stone"), py_str("stones"), py_str("ball"), py_str("balloon"), py_str("duty"), py_str("laws"), py_str("law"), py_str("bill"), py_str("act"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_A_RESULT(call_builtin("frozenset", &[py_tuple(vec![py_str("results"), py_str("result"), py_str("exam"), py_str("exams"), py_str("test"), py_str("tests"), py_str("interview"), py_str("course"), py_str("grade"), py_str("grades"), py_str("marks"), py_str("finals"), py_str("mocks"), py_str("driving"), py_str("theory"), py_str("gcse"), py_str("gcses"), py_str("alevels"), py_str("a-levels"), py_str("degree"), py_str("dissertation"), py_str("viva"), py_str("audition"), py_str("assessment"), py_str("inspection"), py_str("mot"), py_str("physical"), py_str("boards"), py_str("revision"), py_str("resit")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_DATE(call_builtin("frozenset", &[py_tuple(vec![py_str("jan"), py_str("january"), py_str("feb"), py_str("february"), py_str("mar"), py_str("march"), py_str("apr"), py_str("april"), py_str("may"), py_str("jun"), py_str("june"), py_str("jul"), py_str("july"), py_str("aug"), py_str("august"), py_str("sep"), py_str("sept"), py_str("september"), py_str("oct"), py_str("october"), py_str("nov"), py_str("november"), py_str("dec"), py_str("december"), py_str("christmas"), py_str("easter"), py_str("spring"), py_str("summer"), py_str("autumn"), py_str("winter"), py_str("the")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSED_TIME(call_builtin("frozenset", &[py_tuple(vec![py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("night"), py_str("week"), py_str("weekend"), py_str("month"), py_str("year"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OBJECTS(call_builtin("frozenset", &[py_tuple(vec![py_str("phone"), py_str("phones"), py_str("battery"), py_str("car"), py_str("engine"), py_str("laptop"), py_str("computer"), py_str("pc"), py_str("tv"), py_str("telly"), py_str("boiler"), py_str("wifi"), py_str("router"), py_str("tablet"), py_str("console"), py_str("watch"), py_str("camera"), py_str("mower"), py_str("lawnmower"), py_str("plant"), py_str("plants"), py_str("tree"), py_str("tree"), py_str("hedge"), py_str("weed"), py_str("weeds"), py_str("flower"), py_str("flowers"), py_str("fire"), py_str("signal"), py_str("connection"), py_str("internet"), py_str("broadband"), py_str("radio"), py_str("speaker"), py_str("speakers"), py_str("bulb"), py_str("fuse"), py_str("printer"), py_str("headphones"), py_str("earphones"), py_str("remote"), py_str("kettle"), py_str("toaster"), py_str("washer"), py_str("dishwasher"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("hoover"), py_str("vacuum"), py_str("scooter"), py_str("bike"), py_str("van")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BEDTIME(py_tuple(vec![py_str("at night"), py_str("through the night"), py_str("all night"), py_str("every night"), py_str("bedtime"), py_str("bed time"), py_str("sleep through"), py_str("sleeps through"), py_str("sleeping through"), py_str("nap")]));
    s_DEAD_IDIOM(call_builtin("frozenset", &[py_tuple(vec![py_str("against"), py_str("tired"), py_str("easy"), py_str("end"), py_str("ends"), py_str("battery"), py_str("silent"), py_str("pan"), py_str("weight"), py_str("set"), py_str("right"), py_str("on"), py_str("last"), py_str("by"), py_str("to"), py_str("heat"), py_str("centre"), py_str("center"), py_str("line"), py_str("bolt"), py_str("cert"), py_str("beat"), py_str("loss"), py_str("lift"), py_str("straight"), py_str("funny"), py_str("wrong"), py_str("certain"), py_str("serious"), py_str("stop"), py_str("cheap"), py_str("ahead"), py_str("slow"), py_str("cheerful"), py_str("glad"), py_str("lucky"), py_str("proud"), py_str("good"), py_str("bad"), py_str("keen"), py_str("jealous"), py_str("happy"), py_str("sure"), py_str("tricky"), py_str("hard"), py_str("quiet"), py_str("boring"), py_str("rude"), py_str("funny")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DEAD_PREDICATE(call_builtin("frozenset", &[mcall(&py_str("\na an the this that these those my your his her its our their\nand or but so if because when while after before since until till\nin on at by for to from with without about of into onto over under\nis was are were be been being am\nnow today already still yet again too also just only even ever never always here there\nevery any some no not nothing nobody none all both each other else\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GOING_TO_DIE(stdlib_call("re", "compile", &[py_str("\\b(?:going\\s+to|gonna|about\\s+to|soon\\s+to|will|won'?t|wont|isn'?t\\s+going\\s+to)\\s+(?:die\\b(?!\\s+(?:laughing|of\\s+(?:laughter|boredom|embarrassment|shame|excitement)))|pass\\s+away|make\\s+it\\b(?!\\s+(?:to\\s+the|in\\s+time|home))|be\\s+put\\s+(?:down|to\\s+sleep))")]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITIONAL_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:going\\s+to|gonna|about\\s+to|will|wont|won'?t|shall)\\b[^.?!]{0,30}?\\b(?:die|dies|be\\s+put\\s+(?:down|to\\s+sleep))\\b[^.?!]{0,40}?\\b(?:if|unless|when|before|otherwise|or\\s+else|without)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAUGHING(py_tuple(vec![py_str("died laughing"), py_str("die laughing"), py_str("dying to"), py_str("died of embarrassment"), py_str("die of embarrassment"), py_str("nearly died"), py_str("almost died"), py_str("i could have died"), py_str("would die"), py_str("died down"), py_str("die down"), py_str("legs are dead"), py_str("arm is dead"), py_str("legs were dead"), py_str("arms are dead"), py_str("feet are dead"), py_str("hands are dead"), py_str("is dead to me"), py_str("drop dead gorgeous"), py_str("dead on my feet"), py_str("dead on time")]));
    s_A_RELATION_IN_A_HARD_TIME(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?:mum|mom|mam|mother|dad|father|nan|nana|gran|granny|grandma|grandad|granddad|grandpa|sister|brother|son|daughter|wife|husband|partner|friend|mate|neighbour|neighbor|aunt|auntie|uncle|cousin|niece|nephew|parents|family)\\b[^.?!]{0,40}?\\b(?:is|are|seems?|has\\s+been|have\\s+been|keeps?\\s+(?:being|getting)|gets?|got)\\b[^.?!]{0,25}?\\b(?:lonely|alone|on\\s+(?:his|her|their)\\s+own|by\\s+(?:him|her|them)self|withdrawn|not\\s+(?:him|her|them)self|low|down|miserable|unhappy|quiet|stuck\\s+at\\s+home)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_HARD_TIME_REPLIES(py_tuple(vec![py_str("That's a heavy thing for them to carry, and it lands on you too. What helps is a standing date they can count on - the same walk or the same coffee each week - rather than \"let me know if you need anything\". What do they like doing most?"), py_str("The quiet after a death is what does the damage, so keep the invitation concrete and keep making it: \"Tuesday, I'll pick you up at ten\" lands where \"I'm here if you need me\" does not. Are they eating and sleeping all right?"), py_str("Ring rather than text, at the same time each week, and keep asking after a no - that is what stops somebody going days without hearing a voice. Is there anyone else close by, or are you it?")]));
    s_SOMEONE_ELSES(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|a|an|the|one of my)\\s+(?:friend|friends|mate|mates|colleague|colleagues|neighbour|neighbors|neighbours|neighbor|boss|landlord|girlfriend|boyfriend|ex|coworker|workmate)(?:'s|s)?\\s+(?:(?:of mine|of ours)\\s+)?(?:whose\\s+|with\\s+|who\\s+has\\s+)?(?:\\w+\\s+){0,2}?(?:mum|mom|mother|dad|father|son|daughter|wife|husband|partner|nan|gran|grandma|grandad|grandpa|brother|sister|aunt|uncle|cousin|child|baby|family)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HIS_HERS(stdlib_call("re", "compile", &[py_str("\\b(?:his|her|their)\\s+(?:mum|mom|mother|dad|father|son|daughter|wife|husband|partner|nan|gran|grandma|grandad|grandpa|brother|sister|aunt|uncle|cousin|child|baby|dog|cat|funeral|wake|memorial|service|viewing|burial)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSES_OWN(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:my|our)\\s+[a-z]+(?:'s|s'|s)\\s+(?:[a-z]+\\s+){0,2}?(?P<who>"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_PEOPLE()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STRANGERS_ANIMAL(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:the|a|an|this|that)\\s+(?:[a-z]+\\s+){0,2}?(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_PET_WORDS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")s?\\s+(?:[a-z]+\\s+){0,4}?\\b(?:next door|over (?:the )?road|upstairs|downstairs|in (?:our|the|my) (?:street|road|close|lane|block|crescent|estate))\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARD(stdlib_call("re", "compile", &[py_str("\\b(card|cards|condolence|sympathy|note|letter|message|text|msg)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EULOGY_WORD(stdlib_call("re", "compile", &[py_str("\\b(eulogy|eulogies)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EULOGY_ASK(stdlib_call("re", "compile", &[py_str("\\b(write|writing|wrote|help|helped|how|draft|start|starting|opening|open|begin|example|examples|template|structure|length|long|short|words|word|minutes|include|say|saying|nervous|stuck)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CARD_TO_WRITE(stdlib_call("re", "compile", &[py_str("\\b(card|cards|condolence|condolences|sympathy|note)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WRITE_WORD(stdlib_call("re", "compile", &[py_str("\\b(write|writing|wrote|put|say|saying|sign|signing|send|sending|word|words|something|anything|line|lines)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WEAR(stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+(?:do|should|can|could)\\s+i\\s+(?:wear|put on)\\b|\\bwhat\\s+to\\s+wear\\b|\\bdress\\s+code\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_WORDS(stdlib_call("re", "compile", &[py_str("\\b(what do i (?:say|write|put|tell)|what to (?:say|write|put)|what should i (?:say|write|put|tell)|what can i say|how do i (?:say|comfort|help|write|tell)|how to (?:comfort|write|word)|any ideas for what to say|words for|what do you say)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(is it normal|is that normal|why do i|why am i|how long (?:does|will|do)|does it (?:get|ever)|will it (?:get|ever)|cant sleep|can't sleep|keep (?:crying|waking)|still (?:cry|crying|hurts|gutted|struggling)|when does it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEF_ASK_FORM(stdlib_call("re", "compile", &[py_str("\\b(is it normal|is that normal|why do i|why am i|how long (?:does|will|do)|does it (?:get|ever)|will it (?:get|ever)|when does it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRACTICAL(stdlib_call("re", "compile", &[py_str("\\b(cost|costs|price|prices|pricing|cheap|cheapest|expensive|afford\\w*|pay|pays|paid|paying|payment|payments|cover|covers|covered|how much (?:is|are|was|were|does|do|will|did)|probate|paperwork|document\\w*|register|registered|registering|certificate|death certificate|wills|estate|insurance|solicitor|lawyer|undertaker|funeral director|coffin|crematorium|cook|cooking|cooks|food|sandwiches|buffet|catering|caterer|drinks|flowers|order|book|booking|berea\\w+ leave|time off|back to work|allowance|benefit|benefits|grant|quotes?|forms?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEF_CUE(stdlib_call("re", "compile", &[py_str("\\b(feel|feels|feeling|feelings|cope|coping|sleep|sleeping|crying|cry|normal|miss|missing|heartbroken|numb|empty|guilt|guilty|angry|lonely)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_DYING(py_tuple(vec![py_str("I'm sorry you are facing this. The waiting is the hardest part of it: ask the doctors what to expect and what will keep them comfortable, say what you would regret not saying, and let people help you - this is not a thing to carry on your own."), py_str("I'm sorry. Being there is most of it: sit with them, keep the room calm, and let them talk about it or not. Ask about the pain relief and who you ring at night, so you know what to do when the time comes."), py_str("I'm sorry. Spend the time on what they want rather than on what you think you should be doing. Tell the people around you what is happening - you will need them, and most of them will want to help."), py_str("I'm sorry. Ask the doctor to explain what the next few weeks look like, in plain words, and write it down - it is hard to hold in your head. Then let the rest go and be with them.")]));
    s_THE_DAYS_BEFORE(py_tuple(vec![py_str("I'm sorry the time is coming. Make the days they have left good ones: their favourite food and company, their usual routine as far as they can still enjoy it. Ask the vet what will keep them comfortable and what the last day will be like - knowing that is what makes it bearable."), py_str("I'm sorry. The waiting is the hard part now: ask the vet to walk you through the day itself before it comes, so nothing about it is a surprise, and spend the time you have with them."), py_str("I'm sorry you are facing this. Say the things you want to say while they can still hear them, spoil them a little, and be there at the end if you can bear it - most people are glad they were. Tell the vet now that you want to be there, so nobody has to ask you on the day."), py_str("I'm sorry. You are doing the last kind thing for them, and a day too early is kinder than a day too late. Keep them comfortable and close, and tell the vet you want each step explained before it happens.")]));
    s_FIRST_DAYS_PERSON(py_tuple(vec![py_str("Nothing has to be sorted today. When you are ready it is the register office first - within five days in England and Wales - and then the funeral director, who takes it from there and tells you what to bring. Ask someone to sit with you for the forms."), py_str("The practical list is shorter than it looks: the doctor or coroner gives you the medical certificate, you register the death at the local register office, and the funeral director handles what comes next. Tell Us Once covers the tax office, the council and benefits in one go. Everything else can wait."), py_str("Take it in the order it comes. Register the death - five days, in England and Wales - and the register office gives you the certificates for the bank and the insurers and a Tell Us Once code. The rest of it, the house and the belongings and the letters, none of that is this week's job.")]));
    s_FIRST_DAYS_PET(py_tuple(vec![py_str("Nothing has to be decided today. Ring the vet when you can face it and they will explain cremation or a home burial, and what you can keep - a paw print, a lock of fur. The rest of the day can just be quiet."), py_str("The vet does the practical part, so the one thing to think about is what you would like done with their body, and whether you want them back home afterwards. Everything else can wait, and asking for a paw print or a clipping of fur is worth doing if you might want it."), py_str("There is no rush to decide anything. The vet will talk you through the choices when you ring them, and they are used to being asked to wait while you think. Take the day slowly - people often find the first one is the hardest.")]));
    s_AT_THE_END(stdlib_call("re", "compile", &[py_str("\\bat\\s+the\\s+end\\b(?=\\s*(?:[,.!?;]|$)|\\s+of\\s+(?:his|her|its|their)\\s+(?:life|days|time))|\\bon\\s+(?:his|her|its|their)\\s+last\\s+legs\\b|\\bfading\\s+(?:away|fast|now)\\b|\\b(?:is|are|seems?|looks?)\\s+fading\\b(?=\\s*(?:[,.!?;]|$))|\\bnothing\\s+more\\s+(?:they|we|the\\s+vet|the\\s+doctors?|anyone)\\s+can\\s+do\\s+for\\s+(?:me|us|him|her|them|my|our|his|their)\\b|\\b(?:vets?|doctors?|consultants?|specialists?|oncologists?|surgeons?|hospital|hospice|nurses?)\\b[^.?!]{0,30}\\bnothing\\s+more\\b(?=\\s*(?:[,.!?;]|$))")]).unwrap_or_else(|e| exc::raise(e)));
    s_DYING_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|am|was|were|isn'?t|aren'?t|isnt|arent)\\s+(?:\\w+\\s+)?dying\\b(?!\\s+(?:to|for|my|her|his|their|your|our|the|a|an)\\b)|\\bdying\\s+of\\s+(?:cancer|the\\s+cancer|dementia|alzheimers|alzheimer'?s|motor\\s+neurone|mnd|huntingtons?|parkinsons?|copd|heart\\s+failure|kidney\\s+failure|liver\\s+failure|aids|leukaemia|leukemia|a\\s+brain\\s+tumour|a\\s+tumour|covid)\\b|\\b(?:(?:is|are|was|were)\\s+(?:now\\s+)?terminal|terminally\\s+ill|end\\s+of\\s+life|palliative|weeks\\s+to\\s+live|months\\s+to\\s+live|days\\s+to\\s+live|(?:is|are|was|were)\\s+(?:now\\s+)?(?:in|at)\\s+(?:a\\s+)?hospice|on\\s+(?:her|his|their)\\s+way\\s+out)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ANTICIPATED(stdlib_call("re", "compile", &[py_str("\\b(?:palliative|terminally\\s+ill|(?:is|are|was|were)\\s+terminal|end\\s+of\\s+life|weeks\\s+to\\s+live|months\\s+to\\s+live|days\\s+to\\s+live|(?:is|are|was|were)\\s+(?:now\\s+)?(?:in|at)\\s+(?:a\\s+)?hospice)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DYING_ALREADY(stdlib_call("re", "compile", &[py_str("\\b(?:died|has\\s+died|had\\s+died|passed\\s+away|was\\s+buried|the\\s+funeral|the\\s+death\\b|my\\s+late\\b)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__WORKING_WITH_THE_DYING(stdlib_call("re", "compile", &[py_str("\\b(?:works?|worked|working|volunteers?|volunteering)\\s+(?:in|at|for|with)\\b|\\b(?:is|are|was|were|am)\\s+(?:a\\s+|an\\s+|the\\s+|my\\s+|our\\s+)?(?:\\w+\\s+){0,2}?(?:nurse|nurses|doctor|doctors|gp|carer|carers|consultant|registrar|paramedic|paramedics)s?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SAID_DYING(stdlib_call("re", "compile", &[py_str("\\bdying\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_GIVE_UP_NAMES(stdlib_call("re", "compile", &[py_str("\\b(?:do|done|doing|try|trying|offer|suggest)\\s+(?:about|for|with|on|to)\\s+(.{0,40})")]).unwrap_or_else(|e| exc::raise(e)));
    s_PEOPLE_AFTER_A_GIVE_UP(call_builtin("frozenset", &[py_tuple(vec![py_str("she"), py_str("he"), py_str("her"), py_str("him"), py_str("his"), py_str("hers"), py_str("them"), py_str("their"), py_str("us"), py_str("me"), py_str("mum"), py_str("mum's"), py_str("mums"), py_str("mom"), py_str("mom's"), py_str("moms"), py_str("dad"), py_str("dad's"), py_str("dads"), py_str("nan"), py_str("nan's"), py_str("nans"), py_str("gran"), py_str("gran's"), py_str("grans"), py_str("mother"), py_str("father"), py_str("parents"), py_str("son"), py_str("daughter"), py_str("husband"), py_str("wife"), py_str("sister"), py_str("brother"), py_str("child"), py_str("baby"), py_str("friend"), py_str("partner")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_IT(stdlib_call("re", "compile", &[py_str("\\b(?:said|says|say|told|tells|telling)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_CARE_GIVER(stdlib_call("re", "compile", &[py_str("\\b(?:schools?|council|boss|manager|bank|landlord|letting\\s+agent|employer|work|hr|university|college|nursery|garage|mechanic|dealer|insurers?|insurance|dwp|benefits|union|lawyers?|solicitors?|social\\s+services|court|police|jobcentre|agent|unions?|customers?\\s+service|the\\s+office)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DYING_REPLIES(add(&g_SOMEONE_DYING(), &py_tuple(vec![py_str("I'm sorry. That's a hard thing to be living with - is there help coming in, or is most of it on you?"), py_str("I'm sorry. It is happening now rather than after, and that is the part nobody prepares you for. Who is looking after you while you look after them?")])));
    s_PUT_DOWN_REGRET_REPLIES(py_tuple(vec![py_str("I'm so sorry. Wanting it back is not the same as having done it wrong, and it is the last call anyone wants to make. When you feel up to it the vet can arrange the cremation or a home burial, and you can ask them for a paw print."), py_str("I'm sorry. That is the heaviest kind of decision to be carrying. You do not have to sort anything today; when you are ready the vet will handle the cremation, and they can take a paw print for you."), py_str("I'm really sorry. Regret is what love looks like afterwards, and it does not mean you got it wrong. The practical bit can wait a day or two - the vet arranges the cremation, and ask them about a paw print if you would want one.")]));
    s_LOSS_REPLIES(py_tuple(vec![py_str("I'm so sorry to hear that. It's a hard thing to carry — I'm here if you want to talk about it, or if anything practical would help."), py_str("I'm really sorry. Be gentle with yourself today, and tell me if there's anything you want to talk through."), py_str("That's such sad news, I'm sorry. Some days will be heavier than others — I'm here if you want to talk about it, or about anything else."), py_str("I'm sorry, that's a real loss. Take today slowly, and shout if there's anything I can help you sort out.")]));
    s_DYING_SAY_REPLIES(py_tuple(vec![py_str("Say the plain things while they can hear them: that you love them, one memory you both laugh at, and that you will be all right. Nothing clever is needed."), py_str("Skip the big speech and tell them one specific thing you are grateful for. It is easier to say than \"I love you\" out loud, and it is what people remember being told."), py_str("Ask them what they want to talk about, and follow that. If they want the ordinary stuff - the garden, the football - that is not avoiding it, that is the visit."), py_str("Bring them something to decide or to hear: how the family is, what the doctor said, the small gossip. Being useful is how most people say it.")]));
    s_DYING_GRIEF_REPLIES(py_tuple(vec![py_str("Grieving someone who is still here is normal, and it is not a rehearsal for the real thing: it is the real thing, starting early. It comes in waves in the same way."), py_str("That heaviness before the death is common - it is called anticipatory grief, and it does not make the later grief smaller or bigger, it is just its own thing. Talk to someone about it."), py_str("It is normal to be tired, snappy and forgetful while someone is dying, and it is not a failing. If it is stopping you sleeping or eating for more than a fortnight, tell your GP."), py_str("You can be grieving and relieved and guilty in the same hour, and none of that makes you bad. Most people who are where you are say the same.")]));
    s_GETTING_THROUGH_REPLIES(py_tuple(vec![py_str("One piece at a time is the only way anyone does it. Pick the next thing - a shower, a sandwich, one phone call - and let the rest wait until that is done."), py_str("Keep the shape of the day: up at the usual time, food at the usual times, bed at night. The routine carries you on the days you cannot carry yourself."), py_str("Tell one person how you are actually doing today and let them do one job for you. Most people want to help and do not know how to ask."), py_str("The first days are the hardest and they are not a forecast of the rest. If you cannot eat or sleep, or you cannot see a way forward, tell your GP - they can help with that.")]));
    s_GETTING_THROUGH_DYING_REPLIES(py_tuple(vec![py_str("An hour at a time, and the ordinary things - food, a shower, going to work - are not a break from it, they are how you keep going."), py_str("Take one day's jobs and no more, tell people what is happening so it is not all on you, and let the visits that do not matter drop."), py_str("You do not have to be all right while this is going on. If you cannot sleep or eat, or you are carrying it with nobody to say it to, tell your GP.")]));
    s_REMEMBER_REPLIES(py_tuple(vec![py_str("Do it smaller rather than skip it: the same lunch or the same cake, with one or two people, and say out loud why you are having it. A day you have planned for is easier than one that just arrives."), py_str("Keep the ritual and shrink it: the food they loved, their place at the table, and one story about them told to whoever is there. Doing something is what marks the day."), py_str("Pick one thing to do at the time you would normally have spent together - the lunch, a walk, a visit - and tell someone you are doing it, so you are not carrying it on your own."), py_str("Cook their dish or get their favourite in, and raise a glass to them: the point is to mark the day on purpose instead of letting it hit you sideways."), py_str("Make the day about them: the meal they would have chosen, the people they liked, and the one thing you would have told them. If it all feels too heavy, do a small version and stop there.")]));
    s_MISLAID_REPLIES(py_tuple(vec![py_str("Tell a member of staff straight away — shops can call it out over the tannoy and watch the doors, which beats searching on your own."), py_str("Go back to the last place you saw them and tell the nearest member of staff what they're wearing, so everyone is looking for the same person."), py_str("Ask staff to put a call out and cover the exits. If it's a child, or someone vulnerable, ring 999 once it's more than a few minutes.")]));
    s_SAY_REPLIES(py_tuple(vec![py_str("Keep it plain and warm: \"I'm so sorry\". Use the person's name, don't rush to fill the silence, and offer one concrete thing — a lift, a meal, covering a shift."), py_str("You don't need the perfect words. \"I'm so sorry\" and then listening is most of it; add one specific offer of help this week."), py_str("Say you're sorry, name them if you can, and offer something practical. \"I'll bring dinner on Thursday\" helps far more than \"let me know if you need anything\".")]));
    s_NAMED_LOSS_SAY_REPLIES(py_tuple(vec![geti(&g_SAY_REPLIES(), &Py::Int(0)), geti(&g_SAY_REPLIES(), &Py::Int(2))]));
    s_CARD_REPLIES(py_tuple(vec![py_str("Keep it short and warm: \"I'm so sorry for your loss. Thinking of you and your family.\" If you knew them, add one small memory — that is the part people keep."), py_str("Two or three lines is plenty: name the person, say you are sorry, and add a practical offer — a meal, a lift, a hand with the children.")]));
    s_GRIEF_REPLIES(py_tuple(vec![py_str("That is normal after a loss, and it eases slowly for most people. If it is still this heavy, your GP is the right person to talk to."), py_str("Most people find the same, and it does get easier. Be gentle with yourself — and see your GP if it has not lifted."), py_str("That is part of grief for most people. If it stays like this, tell your GP; there is real help for it.")]));
    s_PET_GRIEF_REPLIES(py_tuple(vec![py_str("It does get easier, and slowly. Losing a {animal} takes something out of every day, so the quiet is loudest in the first weeks; if it is still this heavy after a month, your GP is the right person to talk to."), py_str("It's real grief, and it eases the same way a person's does - slowly. Losing {article} {animal} leaves a gap in the day, so be gentle with yourself, and see your GP if it has not lifted in a few weeks."), py_str("Most people find it eases, and no timetable is right. If it stays this heavy, tell your GP: grief for {article} {animal} counts, and there is real help for it.")]));
    s_FUNERAL_REPLIES(py_tuple(vec![py_str("Go if you can, even just for the service: turning up matters far more than anything you say. Take a card, say the plain thing, and leave when you need to."), py_str("If you can make it, go — families remember who came. You don't have to speak; sitting with them is the thing.")]));
    s_FUNERAL_COST_COMPARE_REPLIES(py_tuple(vec![py_str("Cremation usually comes out cheaper than burial: the plot, the grave being dug and often a headstone are what the difference is made of. Ask two funeral directors for a written, itemised quote and compare the coffin, the cars and the cremation fee as separate lines."), py_str("Cremation, yes - a burial adds the plot, the digging and usually a headstone, and that is where the difference is. The rest of the bill is much the same either way, so ask for the itemised quote and look at the coffin first."), py_str("A cremation with no service is the cheapest of the lot, and a burial is the dearest: the plot and the headstone are the difference. Prices move a lot by area, so get two itemised quotes in writing before you decide.")]));
    s_FUNERAL_COST_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:flowers?|wreath|bouquet|music|songs?|hymns?|buffet|sandwiches|catering|drinks?|order of service|programme|cards|outfit|clothes|suit|dress|shoes|tie|travel|fare|fares|flights?|petrol|coach|taxi)\\b|\\b(?:go|get|going|getting) to (?:the|his|her|their) funeral\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DAY_PART(py_str("(?:music|songs?|hymns?|tunes?|readings?|reads?|recit(?:e|ed|ing)|poems?|psalms?|flowers?|wreaths?|bouquets?|order of service|kids?|children|child|baby|toddler|grandchildren|grandkids)"));
    s_A_PART_OF_THE_DAY(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\b(?:what|which|any)\\b(?:\\W+\\w+){0,5}\\W+"), &g__A_DAY_PART()), &py_str("\\b|\\b(?:should|shall|can|could|do)\\s+(?:i|we|you)\\b(?:\\W+\\w+){0,6}\\W+")), &g__A_DAY_PART()), &py_str("\\b|\\b(?:should|shall|can|could|do|does)\\s+(?:(?:i|we|you|a|the|any|my|our)\\s+)?(?:kids?|children|child|baby|toddler|grandchildren|grandkids)\\b|\\b(?:good|nice|right|best|suitable)\\s+(?:songs?|hymns?|tunes?|music|readings?|poems?|psalms?)\\b|\\b(?:should|shall|can|could)\\s+(?:my|our)\\b(?:\\W+\\w+){0,4}\\W+")), &g__A_DAY_PART()), &py_str("\\b|\\bhow\\s+long\\s+(?:is|does|do|are|will|would)\\b(?:\\W+\\w+){0,4}\\W+(?:funeral|service|cremation|burial|wake|memorial)\\b|\\bhow\\s+long\\s+(?!until|till|to|before)\\w*(?:\\W+\\w+){0,6}\\W+\\bfuneral\\b|\\bhow\\s+many\\s+(?:days|weeks)\\b(?:\\W+\\w+){0,6}\\W+\\bfuneral\\b|\\bhow\\s+soon\\b(?:\\W+\\w+){0,4}\\W+\\bfuneral\\b|\\bis\\s+it\\s+(?:ok|okay|fine|alright)\\b(?:\\W+\\w+){0,6}\\W+")), &g__A_DAY_PART()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAY_THE_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:what to say|what do i say|what can i say|what should i say|what shall i say|say at|say to|words for|eulogy|eulogies|speech)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DEATH_IN_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:died|dies|dying|dead|death|passed away|passed on|passed|killed|cremated|buried|widow|widower)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__FUNERAL_NOUN(py_str("(?:funerals?|cremations?|cremation|crematoriums?|cremate|burials?|bury|buried|"));
    let __aug6 = add(&g__FUNERAL_NOUN(), &py_str("undertakers?|funeral directors?)"));
    s__FUNERAL_NOUN(__aug6);
    s__COST_NOUN(py_str("(?:cost|costs|costing|price|prices|priced|expensive|cheaper|cheapest|cheap|"));
    let __aug7 = add(&g__COST_NOUN(), &py_str("afford|affordable|pay|paid|paying|fee|fees|money|budget|quote|quotes|"));
    s__COST_NOUN(__aug7);
    let __aug8 = add(&g__COST_NOUN(), &py_str("how much)"));
    s__COST_NOUN(__aug8);
    s_FUNERAL_COST(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&add(&add(&py_str("\\b"), &g__FUNERAL_NOUN()), &py_str("\\b[^.?!]{0,30}?\\b")), &g__COST_NOUN()), &py_str("\\b|\\b")), &g__COST_NOUN()), &py_str("\\b[^.?!]{0,30}?\\b")), &g__FUNERAL_NOUN()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_ESTATE(stdlib_call("re", "compile", &[py_str("\\b(?:estate|probate|inheritance|the will)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FUNERAL_HELP(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b"), &g__FUNERAL_NOUN()), &py_str("\\b[^.?!]{0,30}?\\b(?:help|afford|claim|payment|pay for|council|benefits?|dwp|social fund|charity|charities)\\b|\\b(?:help|afford|claim|payment|pay for|council|benefits?|dwp|social fund|charity|charities)\\b[^.?!]{0,30}?\\b")), &g__FUNERAL_NOUN()), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_HELP_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:help|claim|claiming)\\b|can'?t afford|cannot afford|cant pay|can'?t pay|no money|nothing to pay")]).unwrap_or_else(|e| exc::raise(e)));
    s_FUNERAL_HELP_REPLIES(py_tuple(vec![py_str("There is help with it: the DWP funeral expenses payment covers part of the bill if you are on certain benefits, and you claim it after the funeral, within six months. Keep every receipt, and ring the council as well - they have a fund of their own for people who cannot pay."), py_str("Claim the DWP funeral expenses payment within six months of the funeral: it covers the cremation or burial fees and some of the rest, and it goes to whoever arranged it. If you are on a low income, ask the council too - they pay for a funeral when there is no money at all."), py_str("You can apply for the funeral expenses payment from the DWP after the funeral - within six months, and only on certain benefits - so tell the funeral director to invoice you and keep the receipts. The council's bereavement service has a separate fund, and Citizens Advice will help you fill the forms in.")]));
    s_QUIETER_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:which|cheaper|cheapest|how much|prices?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FUNERAL_COST_COMPARE(stdlib_call("re", "compile", &[py_str("\\b(?:cheaper|cheapest|more expensive|less expensive|costs? more|costs? less|vs|versus|instead of|or a burial|or burial|or cremation|or a cremation|worth it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BURIAL_SIDE(stdlib_call("re", "compile", &[py_str("\\b(?:burials?|bury|buried|burying|graves?|plots?|headstones?|coffins?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__CREMATION_SIDE(stdlib_call("re", "compile", &[py_str("\\b(?:cremations?|cremate|cremated|crematoriums?|ashes|urns?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PRONOUN_MONEY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:help|pay|paying|paid|afford|claim|claiming|cost|costs|costing|money|cheaper|how much)\\b[^.?!]{0,24}?\\b(?:it|that|this|them)\\b|\\b(?:it|that|this|them)\\b[^.?!]{0,24}?\\b(?:cost|costs|pay|paid|afford|expenses?|expensive|cheap)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BILL_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:funerals?|cremations?|cremation|crematoriums?|burials?|bury|buried|coffins?|undertakers?|funeral directors?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BILL_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:cost|costs|price|prices|quote|quotes|pay|paid|paying|fee|fees|afford|bill|dwp|benefit|benefits|cheapest|cheaper)\\b|\\u00a3|\\bthousands?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_BILL_THREAD(call_builtin("frozenset", &[py_tuple(vec![py_str("safety"), py_str("urgent")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LOSS_STILL_THE_THREAD(Py::Int(3));
    s_ACKNOWLEDGE(stdlib_call("re", "compile", &[py_str("\\b(?:sorry (?:to hear|for your loss)|such sad news|condolences|my sympathy|sympathies|real loss|hard(?:est)? days?|nothing prepares you)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:i|i'm|im|i've|ive|me|my|we|we're|were|us)\\b[^.]*\\b(?:cry|crying|cried|tears|guilty|guilt|miss|missing|struggling|broken|heartbroken|devastated|numb|angry|lonely|sad|empty|numb|cant stop|can't stop|cannot stop|keep thinking|nightmares?|feel awful|feel terrible|feel lost|feel sick)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_PET(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:my|our)\\s+(?:(?:old|poor|little|beloved|lovely|sweet|baby|family|best)\\s+)?(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_PET_WORDS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE(stdlib_call("re", "compile", &[py_str("[;,]|\\b(?:and|but|then|so|because|although|though|while|after|before|which|who)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PUTTING_REGRETTED(stdlib_call("re", "compile", &[py_str("\\b(?:regrets?|regretted|regretting|wish)\\b[^.?!]{0,24}?\\bputting\\s+(?:\\w+\\s+){0,3}?(?:to sleep|down)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_YET(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|am)\\s+being\\s+(?:put|euthanis\\w*)|\\b(?:will|would|going to|about to|has to|have to|needs? to|got to|due to)\\s+(?:be\\s+)?(?:put|euthanis\\w*)|\\b(?:have|has|need|needs|got)\\s+to\\s+(?:put|be put)\\b|\\b(?:have|has|need|needs|got)\\s+to\\s+have\\s+(?:\\w+\\s+){0,3}?(?:put|euthanis\\w*)|\\bputting\\s+(?:\\w+\\s+){0,3}?(?:to sleep|down)\\b|\\b(?:is|are|am|was|were)\\s+dying\\b|\\b(?:mum|mom|mam|dad|father|mother|nan|nana|gran|granny|grandad|grandma|brother|sister|wife|husband|partner|friend|son|daughter)s\\s+dying\\b|\\b(?:isn'?t|isnt|not|never|won'?t|wont|is not)\\s+going\\s+to\\s+(?:make\\s+it|last)\\b|\\b(?:has|had|have|say|says|said|told)\\s+(?:\\w+\\s+){0,4}?(?:a\\s+few|a\\s+couple\\s+of|another|one|two|three|four|five|six|a\\s+matter\\s+of)\\s+(?:days?|weeks?|months?)\\s+(?:left|to\\s+go|or\\s+so)\\b|\\b(?:gave|given|gives)\\s+(?:\\w+\\s+){0,4}?(?:a\\s+few|a\\s+couple\\s+of|another|one|two|three|four|five|six|a\\s+matter\\s+of)\\s+(?:days?|weeks?|months?)\\b|\\b(?:days?|weeks?|months?)\\s+(?:left|to\\s+go)\\b(?!\\s+(?:of|in|on|until|till|before|for))")]).unwrap_or_else(|e| exc::raise(e)));
    s_ESTIMATE(stdlib_call("re", "compile", &[py_str("\\b(?:has|had|have|say|says|said|told)\\s+(?:\\w+\\s+){0,4}?(?:a\\s+few|a\\s+couple\\s+of|another|one|two|three|four|five|six|a\\s+matter\\s+of)\\s+(?:days?|weeks?|months?)\\s+(?:left|to\\s+go|or\\s+so)\\b|\\b(?:gave|given|gives)\\s+(?:\\w+\\s+){0,4}?(?:a\\s+few|a\\s+couple\\s+of|another|one|two|three|four|five|six|a\\s+matter\\s+of)\\s+(?:days?|weeks?|months?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_MORE_TO_DO(stdlib_call("re", "compile", &[py_str("\\b(?:nothing|not\\s+much|not\\s+anything)\\s+(?:more|else|further|left)\\b[^.!?]{0,40}?\\b(?:do|done|try|trying|offer|suggest)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TREATMENT_STOPPED(stdlib_call("re", "compile", &[py_str("\\b(?:stopped|stopping|stop|cease[ds]?|halted|withdrawn\\s+from|taken\\s+off|came\\s+off|no\\s+longer\\s+(?:having|on))\\s+(?:\\w+\\s+){0,3}?(?:treatment|chemo|chemotherapy|radiotherapy|dialysis)\\b|\\bno\\s+(?:more|further)\\s+(?:treatment|chemo|chemotherapy|radiotherapy|options)\\b|\\b(?:treatment|chemo|chemotherapy|radiotherapy|dialysis)\\s+(?:has|have|had|is|are|was|were)\\s+(?:been\\s+)?(?:stopped|halted|withdrawn)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MEDICATION_STOPPED(stdlib_call("re", "compile", &[py_str("\\b(?:stopped|stopping|stop|cease[ds]?|halted|no\\s+longer\\s+(?:having|on)|taken\\s+off)\\s+(?:\\w+\\s+){0,3}?(?:medication|meds|drugs|tablets|pills|painkillers)\\b|\\b(?:medication|meds|drugs|tablets|pills|painkillers)\\s+(?:has|have|had|is|are|was|were)\\s+(?:been\\s+)?(?:stopped|halted|withdrawn)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAST_DRUG(py_str("morphine|diamorphine|midazolam|fentanyl|oxycodone|alfentanil|levomepromazine|syringe\\s+driver|palliative\\s+sedation|end\\s+of\\s+life\\s+(?:meds|medication|drugs)"));
    s_ON_THE_LAST_DRUG(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:put|puts|putting|started|starting|starts?|switched|moved)\\s+(?:\\w+\\s+){0,3}?(?:on|onto)\\s+(?:a\\s+|the\\s+|her\\s+|his\\s+|their\\s+)?(?:"), &g_LAST_DRUG()), &py_str(")\\b|\\b(?:he|she|they|mum|dad|nan|gran|grandad|grandma|grandpa|husband|wife|son|daughter|mother|father|brother|sister|friend|mate)\\b[^.!?,]{0,12}?\\b(?:is|are|was|were)\\s+(?:now\\s+)?(?:on|getting)\\s+(?:a\\s+|the\\s+)?(?:")), &g_LAST_DRUG()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_ON_THE_DRUG(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:i|i'?m|im|i'?ve|i\\s+have|i\\s+was|i\\s+got|i'?d|me|my\\s+(?:back|knee|hip|leg|arm|shoulder|teeth|tooth|wrist|ankle))\\b[^.!?]{0,20}?\\b(?:"), &g_LAST_DRUG()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_WHO(stdlib_call("re", "compile", &[py_str("\\b(?:doctors?|doctor'?s|gps?|consultants?|surgeons?|specialists?|oncologists?|oncology|hospitals?|hospice|ward|icu|intensive\\s+care|nurses?|palliative|vets?|clinic|treatment|chemo|chemotherapy|radiotherapy|dialysis|cancer|tumours?|tumors?|leukaemia|leukemia|dementia|motor\\s+neurone|parkinsons?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAYS_LEFT(stdlib_call("re", "compile", &[py_str("\\b(?:days?|weeks?|months?)\\s+left\\b(?!\\s+(?:of|in|on|at|with|before|until|for)\\b)(?!\\s+to\\b(?!\\s+live\\b))|\\b(?:days?|weeks?|months?)\\s+to\\s+live\\b|\\b(?:not\\s+)?long\\s+(?:left|to\\s+go)\\b|\\b(?:a|only\\s+a)\\s+matter\\s+of\\s+(?:time|days|weeks|hours)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_END(stdlib_call("re", "compile", &[py_str("\\b(?:left|to\\s+go)\\s+(?:of|in|on|at|with|before|until|for)\\b|\\b(?:left|to\\s+go)\\s+to\\s+(?!live\\b|be\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_FINAL_DAYS(stdlib_call("re", "compile", &[py_str("\\b(?:her|his|their|my|our)\\s+final\\s+(?:days|weeks|months)\\b|\\bfinal\\s+(?:days|weeks|months)\\s+of\\s+(?:her|his|their|my|our)\\s+life\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COULD_BE_DAYS(stdlib_call("re", "compile", &[py_str("\\b(?:could|might|may|probably)\\s+be\\s+(?:a\\s+)?(?:few\\s+|couple\\s+of\\s+)?(?:days|weeks|hours)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAYS_WAIT_FOR(stdlib_call("re", "compile", &[py_str("\\b(?:days?|weeks?|hours?)\\s+(?:for|until|till|til|before|by|of)\\s+(?!the\\s+(?:end|worst)\\b|his\\s+end\\b|her\\s+end\\b|she\\s+goes\\b|he\\s+goes\\b|it\\s+happens\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAST_STAGES(stdlib_call("re", "compile", &[py_str("\\b(?:the\\s+)?(?:last|final|end)\\s+(?:days|weeks|stages?)\\b|\\bin\\s+(?:the\\s+)?(?:final|end)\\s+stages?\\b|\\b(?:final\\s+)?stages?\\s+of\\s+(?:her|his|their|my|our)\\s+(?:cancer|illness|disease|life)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_GETTING_BETTER(stdlib_call("re", "compile", &[py_str("\\b(?:isnt|isn'?t|wont|won'?t|never\\s+going\\s+to|not\\s+going\\s+to|will\\s+not)\\s+(?:will\\s+)?(?:going\\s+to\\s+)?get\\s+(?:any\\s+)?better\\b|\\bno\\s+longer\\s+going\\s+to\\s+get\\s+better\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GIVEN_UP_ON(stdlib_call("re", "compile", &[py_str("\\bgiven\\s+up\\s+on\\b|\\bhave\\s+given\\s+up\\b|\\bgive\\s+up\\s+on\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PREPARE(stdlib_call("re", "compile", &[py_str("\\bprepare\\s+(?:ourselves|yourself|yourselves|the\\s+family|them)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PREPARE_OTHERWISE(stdlib_call("re", "compile", &[py_str("\\b(?:results?|exams?|tests?|interviews?|inspections?|audit|inspection|wedding|move|visit|appointment|flight|holiday|winter|storm|review|scans?|x-?rays?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_LIFE_SUPPORT(stdlib_call("re", "compile", &[py_str("\\b(?:taken|taking|took|take|came|come|coming|coming\\s+off)\\s+(?:\\w+\\s+){0,2}?off\\s+(?:the\\s+)?(?:ventilator|ventilation|life\\s+support|breathing\\s+machine)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_SUPPORT_WELL(stdlib_call("re", "compile", &[py_str("\\b(?:on\\s+(?:his|her|their)\\s+own|breathing\\s+(?:on|by)|himself|herself|awake|talking)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TOLD_US(stdlib_call("re", "compile", &[py_str("\\b(?:we'?ve\\s+been\\s+told|we\\s+were\\s+told|they'?ve\\s+told\\s+us|they\\s+told\\s+us|they\\s+(?:have\\s+)?said|doctors?\\s+(?:have\\s+)?said|consultants?\\s+(?:have\\s+)?said)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_WORKING(stdlib_call("re", "compile", &[py_str("\\b(?:isnt|isn'?t|not|stopped|no\\s+longer)\\s+working\\b|\\b(?:has|have|had)\\s+stopped\\s+working\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SERIOUS_CARE(stdlib_call("re", "compile", &[py_str("\\b(?:oncologists?|oncology|consultants?|specialists?|palliative|hospices?|cancer|tumours?|tumors?|leukaemia|leukemia|dementia|motor\\s+neurone|parkinsons?|mnd|end\\s+of\\s+life|morphine|syringe\\s+driver|last\\s+stages)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOSPICE_CARE(stdlib_call("re", "compile", &[py_str("\\bhospice\\s+care\\b|\\b(?:in|at)\\s+(?:a\\s+|the\\s+|her\\s+|his\\s+|their\\s+)?hospice\\b|\\b(?:moved|moving|transferred|gone|went|going|admitted|taken|took)\\b(?:\\s+\\w+){0,3}?\\s+(?:in)?to\\s+(?:a\\s+|the\\s+|her\\s+|his\\s+|their\\s+)?hospice\\b|\\b(?:into|onto)\\s+(?:a\\s+|the\\s+|her\\s+|his\\s+|their\\s+)?hospice\\b|\\bhospice\\s+(?:is\\s+being\\s+arranged|has\\s+been\\s+arranged|at\\s+home)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOSPICE_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:jobs?|works?|worked|working|volunteers?|volunteering|nurse\\s+there|manager|staff|shifts?|sponsors?|sponsored|sponsoring|fundrais\\w*|raise\\s+(?:money|funds)|raising\\s+money|collecting|collection|donate|donating|donations?|cake\\s+sale|charity|justgiving|marathon|walk|run|cycle|ride|quiz|raffle|tombola|bring\\s+and\\s+buy|open\\s+day)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PALLIATIVE_ITSELF(stdlib_call("re", "compile", &[py_str("\\bpalliative\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DYING_OTHER(stdlib_call("re", "compile", &[py_str("\\bdying\\s+(?:for|to)\\b|\\bdying\\s+(?:my|her|his|their|our|your|the)\\s+(?:hair|beard|roots|eyebrows|eyebrow|nails|lashes)\\b|\\bdying\\s+it\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_BARE(stdlib_call("re", "compile", &[py_str("\\b(?:is|was|isn'?t|wasn'?t)\\s+(?:it|that|this)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRACTICAL_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|who|where|when|how)\\s+(?:do|does|should|shall|can|could|now|next)\\b|\\b(?:do|should|shall|can|could|need)\\s+(?:we|i)\\s+\\w+|\\b(?:is|was|isn'?t|wasn'?t)\\s+(?:\\w+\\s+){0,3}(?:expensive|costly|cheap)\\b|\\bhow\\s+(?:much|expensive)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ADMINISTRATION(stdlib_call("re", "compile", &[py_str("\\b(?:funerals?|cremation|crematorium|burial|coffin|headstone|probate|wills?|estate|solicitors?|coroner|register(?:ing)? the death|death certificate|medical certificate|bank accounts?|insurance|pension|bereavement|inheritance|funeral directors?|grave|chapel of rest|quotes?|cheapest)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ADMINISTRATION_KEYS(py_tuple(vec![py_tuple(vec![py_str("accounts"), py_str("\\b(?:bank\\s+)?accounts?\\b|\\bbereavement team\\b")]), py_tuple(vec![py_str("registration"), py_str("\\bregist(?:er|ering|ration)(?:\\s+(?:the|a|his|her|their))?\\s+death\\b|\\bregister office\\b")]), py_tuple(vec![py_str("death certificate"), py_str("\\bdeath certificate\\b|\\bmedical certificate\\b")]), py_tuple(vec![py_str("probate"), py_str("\\bprobate\\b|\\bletters of administration\\b")]), py_tuple(vec![py_str("inheritance"), py_str("\\binheritance(?: tax)?\\b|\\biht\\d*\\b")]), py_tuple(vec![py_str("estate"), py_str("\\bestate\\b|\\bexecutor\\b")]), py_tuple(vec![py_str("will"), py_str("\\bwills?\\b")]), py_tuple(vec![py_str("funeral"), py_str("\\bfunerals?\\b|\\bfuneral directors?\\b|\\bchapel of rest\\b")]), py_tuple(vec![py_str("cremation"), py_str("\\bcremat(?:ion|orium|e|ed)\\b")]), py_tuple(vec![py_str("burial"), py_str("\\bburials?\\b|\\bbury\\b|\\bburied\\b|\\bgrave(?:side)?\\b")]), py_tuple(vec![py_str("coffin"), py_str("\\bcoffins?\\b|\\bheadstones?\\b")]), py_tuple(vec![py_str("solicitor"), py_str("\\bsolicitors?\\b")]), py_tuple(vec![py_str("coroner"), py_str("\\bcoroners?\\b")]), py_tuple(vec![py_str("insurance"), py_str("\\binsurance\\b|\\blife cover\\b")]), py_tuple(vec![py_str("pension"), py_str("\\bpensions?\\b")])]));
    s_DEFINES(stdlib_call("re", "compile", &[py_str("^\\s*(?:a|an|the)\\s+[\\w'-]+\\s+(?:says|means|is)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_DEATH(stdlib_call("re", "compile", &[py_str("\\b(?:died|dies|dying|death|dead|deceased|passed away|bereave\\w*|widow\\w*|widower\\w*|estate|executor|next of kin|funeral|cremation|burial)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OPEN_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|so|ok|okay|but|well|right)\\s*,?\\s*)*(?:what\\s+(?:do|should|shall|can|could)\\s+(?:i|we)\\s+(?:do|now|next)(?:\\s+(?:now|next|today))?|what\\s+(?:now|next|happens\\s+now|do\\s+i\\s+do\\s+now)|now\\s+what|where\\s+do\\s+(?:i|we)\\s+(?:start|begin)(?:\\s+now)?|what\\s+am\\s+i\\s+supposed\\s+to\\s+do(?:\\s+now)?)(?:\\s+about\\s+(?:it|that|this|him|her|them|all this|all of it))?\\s*[.?!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_THROUGH(stdlib_call("re", "compile", &[py_str("\\bget(?:ting)?\\s+through\\s+(?:it|this|that|today|tonight|the\\s+(?:day|days|night|nights|week|weeks|weekend|first\\s+few\\s+days|first\\s+week))\\b|\\bhow\\s+(?:am|do|will|can)\\s+i\\s+(?:supposed\\s+to\\s+)?(?:get\\s+through|carry\\s+on)(?:\\s+(?:it|this|that|today|tonight|the\\s+(?:day|days|night|nights|week|weeks|weekend|first\\s+few\\s+days|first\\s+week)))?\\s*[.?!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_THROUGH_TO(stdlib_call("re", "compile", &[py_str("\\bget(?:s|ting)?\\s+through\\s+to\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_THROUGH_DAY(stdlib_call("re", "compile", &[py_str("\\bget(?:ting)?\\s+through\\s+(?:christmas|the\\s+holidays?|new\\s+year|easter|(?:his|her|their|my\\s+(?:mums?|dads?|nans?|grans?|sons?|daughters?))\\s+birthday|birthdays?|the\\s+anniversary|our\\s+anniversary|mothers?\\s+day|fathers?\\s+day)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_THROUGH_OBJECT(stdlib_call("re", "compile", &[py_str("\\bget(?:ting)?\\s+through\\s+([a-z' ]{1,60})")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_THROUGH_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("my"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("your"), py_str("of"), py_str("at"), py_str("in"), py_str("on"), py_str("to"), py_str("with"), py_str("without"), py_str("and"), py_str("for"), py_str("by"), py_str("from"), py_str("s"), py_str("too"), py_str("as"), py_str("well"), py_str("again"), py_str("now"), py_str("still"), py_str("just"), py_str("all"), py_str("every"), py_str("first"), py_str("next"), py_str("last"), py_str("few"), py_str("couple"), py_str("up"), py_str("through"), py_str("it"), py_str("him"), py_str("them"), py_str("us"), py_str("me"), py_str("i"), py_str("if"), py_str("when"), py_str("while"), py_str("til"), py_str("until"), py_str("alone"), py_str("so"), py_str("very"), py_str("much"), py_str("more"), py_str("each"), py_str("any"), py_str("some")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_THROUGH_THE_LOSS(call_builtin("frozenset", &[py_tuple(vec![py_str("day"), py_str("days"), py_str("night"), py_str("nights"), py_str("week"), py_str("weeks"), py_str("weekend"), py_str("weekends"), py_str("month"), py_str("months"), py_str("year"), py_str("years"), py_str("hour"), py_str("hours"), py_str("evening"), py_str("evenings"), py_str("morning"), py_str("mornings"), py_str("afternoon"), py_str("afternoons"), py_str("today"), py_str("tonight"), py_str("tomorrow"), py_str("time"), py_str("times"), py_str("christmas"), py_str("xmas"), py_str("easter"), py_str("new"), py_str("birthday"), py_str("birthdays"), py_str("anniversary"), py_str("wedding"), py_str("funeral"), py_str("wake"), py_str("holiday"), py_str("holidays"), py_str("mothers"), py_str("fathers"), py_str("mother"), py_str("father"), py_str("mum"), py_str("mums"), py_str("dad"), py_str("dads"), py_str("nan"), py_str("nans"), py_str("gran"), py_str("grans"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("valentines"), py_str("death"), py_str("dying"), py_str("loss"), py_str("grief"), py_str("sorrow"), py_str("mourning"), py_str("missing"), py_str("absence"), py_str("empty"), py_str("quiet"), py_str("silence"), py_str("sad"), py_str("hard"), py_str("hardest"), py_str("worst"), py_str("bad"), py_str("dark"), py_str("lonely"), py_str("pain"), py_str("ache"), py_str("rest"), py_str("everything"), py_str("bit"), py_str("part"), py_str("piece"), py_str("pieces"), py_str("one"), py_str("two"), py_str("three"), py_str("it"), py_str("them"), py_str("him"), py_str("her"), py_str("work"), py_str("office"), py_str("colleagues"), py_str("people")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_FEELING(stdlib_call("re", "compile", &[py_str("\\b(?:is|was|feels|seems|sounds)\\s+(?:\\w+\\s+){0,3}(?:cremat\\w*|burial|burying|ashes|coffin|casket|urn|grave|euthanasi\\w*)\\s+(?:\\w+\\s+){0,3}(?:sad|hard|difficult|awful|grim|brutal|expensive|cheap|costly|right|wrong|ok|okay|easy|quick|painful|kind|better)\\b|\\b(?:cremat\\w*|burial|burying|ashes|coffin|casket|urn|grave|euthanasi\\w*)\\s+(?:is|was|feels|seems|sounds)\\s+(?:\\w+\\s+){0,2}(?:sad|hard|difficult|awful|grim|brutal|expensive|cheap|costly|right|wrong|ok|okay|easy|quick|painful|kind|better)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_ASK(stdlib_call("re", "compile", &[py_str("\\b(what time|what day|what date|which day|when is|when's|whens|how long until|how long till|what time does)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FUNERAL_WHEN_REPLIES(py_tuple(vec![py_str("The time and place are on the family's notice or the funeral director's page — ask a family member or the funeral director if you can't find it, nobody minds being asked."), py_str("Check the notice the family put out, or ring the funeral director: they will tell you the time and where to meet. Ask whether to go straight to the crematorium or meet at the house."), py_str("Ask a relative or the funeral director for the time and the meeting place — most services are at the church or chapel first, then the crematorium — and get there a little early.")]));
    s_ASKS_ABOUT_THE_FEELING(stdlib_call("re", "compile", &[py_str("\\bis\\s+(?:it|that|this)\\s+(?:normal|weird|wrong|bad|ok|okay|common|selfish|strange|odd|awful)\\s+(?:to\\s+)?(?:feel|be)\\b|\\bis\\s+(?:it|that)\\s+(?:normal|weird|wrong|bad|ok|okay|common|selfish|strange|odd)\\s+(?:that|if)\\s+i\\s+(?:feel|felt|am)\\b|\\bis\\s+that\\s+a\\s+(?:normal|bad|selfish|strange)\\s+thing\\b|\\bwhy\\s+do\\s+i\\s+(?:still\\s+)?(?:feel|keep\\s+feeling)\\b|\\bdoes\\s+(?:it|that)\\s+(?:make\\s+me|mean\\s+i'?m)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SYMPATHY(stdlib_call("re", "compile", &[py_str("\\b(sympathy|condolence|condolences|bereavement)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CARD_WORD(stdlib_call("re", "compile", &[py_str("\\b(card|cards|message|note|letter|text)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FORM(stdlib_call("re", "compile", &[py_str("\\b(should i|do i|can i|how do i|shall i|would you)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_WE(stdlib_call("re", "compile", &[py_str("\\b(should we|can we|do we|how do we|shall we|where do we|what do we|when do we|who do we)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_TAIL(stdlib_call("re", "compile", &[py_str(",\\s*(?:is|are|was|were|do|does|did|should|can|how|what|where|when|who|will|would|shall)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_ARRANGEMENT(stdlib_call("re", "compile", &[py_str("\\b(bury|buried|burying|burial|body|bodies|dead|ashes|cremat\\w*|cremate|urn|plot|grave|graveside|headstone|memorial|paw\\s?print|lock\\s+of\\s+(?:fur|hair)|coffin|casket|collect\\w*|pick\\w*\\s+(?:\\w+\\s+){0,2}up|collect\\s+\\w+\\s+from\\s+the\\s+vet)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PET(stdlib_call("re", "compile", &[py_str("\\b(cat|dog|puppy|kitten|rabbit|hamster|guinea\\s?pig|horse|pony|budgie|parrot|pet|pets|ferret|gerbil|rat|mouse|tortoise|snake|lizard|fish|goldfish)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_HAS_HAD_ENOUGH_AFTER(py_str("has\\s+had\\s+enough(?=\\s*(?:[,.!?;]|$)|\\s*,?\\s*(?:of\\s+)?(?:it|this|life|everything|it\\s+all|all\\s+of\\s+it)\\b|\\s*,?\\s*(?:of\\s+)?(?:the|his|her|their)\\s+(?:pain|suffering|treatment|chemo|medication|operations|surgery|seizures|incontinence)\\b|\\s*,?\\s+(?:he|she|it|they)?\\s*(?:is|are|s)?\\s*(?:not\\s+eating|stopped\\s+eating|cant\\s+eat|cant\\s+get\\s+up|cant\\s+walk|cant\\s+stand|in\\s+pain|suffering|giving\\s+up)\\b|\\s+(?:now|today|already)\\b)"));
    s_PET_HAS_HAD_ENOUGH(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:wants?\\s+to\\s+die|wants?\\s+to\\s+be\\s+put\\s+to\\s+sleep|no\\s+will\\s+to\\s+live|doesn'?t\\s+want\\s+to\\s+live|does\\s+not\\s+want\\s+to\\s+live|no\\s+quality\\s+of\\s+life|"), &g_PET_HAS_HAD_ENOUGH_AFTER()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEAKER_HAS_HAD_ENOUGH(stdlib_call("re", "compile", &[py_str("\\b(?:i|me|myself)\\b[^.?!]{0,24}\\b(?:die|dying|kill\\s+myself|end\\s+it|go\\s+on)\\b|\\b(?:so\\s+(?:do|will|would|can|am|are|have|has)\\s+i|me\\s+too)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEAKER_TOO(stdlib_call("re", "compile", &[py_str("\\b(?:and\\s+)?so\\s+(?:am|do|have|has|will|would|can|are|is)\\s+i\\b|\\b(?:and\\s+)?i(?:'m|\\s+am|\\s+do|\\s+have|\\s+will|\\s+would|\\s+can)\\s+(?:too|as\\s+well)\\b|\\b(?:me\\s+too|same\\s+here|i\\s+feel\\s+the\\s+same|i\\s+feel\\s+it\\s+too|neither\\s+(?:do|am|have)\\s+i)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WISH_FRAME(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|wanted|wanna|wish|wishes|wished|plan|plans|planning|tempted|urge|thinking\\s+(?:of|about)|thought\\s+about)\\b|\\b(?:going|gonna)\\s+to\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEAKER_TOO_REPLIES(py_tuple(vec![py_str("You are carrying their ending and your own in the same breath, and that is more than one person should hold. Tell someone today - a GP, a friend, or a crisis line on 116 123 in the UK or 988 in the US - and let them take a share of it."), py_str("That is two of you at the end at once, and you should not be managing it on your own. Ring a crisis line today (116 123 in the UK, 988 in the US), or tell someone close so they can be with you while you say the rest."), py_str("I'm sorry. Both at once is more than one person can carry quietly: say it out loud to someone today - a friend, your GP, or a crisis line on 116 123 in the UK or 988 in the US - and let them help with the next bit."), py_str("You matter in this too, not only the one you are losing. Please tell someone today - a crisis line is 116 123 in the UK and 988 in the US - and let them sit with you while you say it.")]));
    s_PET_AT_THE_END(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|seems?|looks?)\\s+done\\b(?=\\s*(?:[,.!?;]|$))|\\bready\\s+to\\s+go\\b(?=\\s*(?:[,.!?;]|$))|\\btelling\\s+me\\s+(?:its|it'?s|his|her)\\s+time\\b|\\bi\\s+think\\s+(?:its|it'?s)\\s+time\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FILLERS(call_builtin("frozenset", &[py_tuple(vec![py_str("actually"), py_str("really"), py_str("even"), py_str("just"), py_str("exactly"), py_str("honestly"), py_str("literally"), py_str("basically"), py_str("anyway"), py_str("still"), py_str("perhaps"), py_str("maybe")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_WORDS(py_dict(vec![(py_str("guilty"), py_str("guilt")), (py_str("guilt"), py_str("guilt")), (py_str("ashamed"), py_str("guilt")), (py_str("shame"), py_str("guilt")), (py_str("angry"), py_str("anger")), (py_str("anger"), py_str("anger")), (py_str("furious"), py_str("anger")), (py_str("rage"), py_str("anger")), (py_str("numb"), py_str("numb")), (py_str("relieved"), py_str("relief")), (py_str("relief"), py_str("relief"))]));
    s_FEELING_REPLIES(py_dict(vec![(py_str("guilt"), py_tuple(vec![py_str("Guilt turns up with grief for most people, whether or not there is anything to be guilty about. Say the thing you keep going over out loud to someone who knew them; it weighs less that way."), py_str("That guilt is a common part of it, and it is rarely fair. Tell one person who knew them what you keep turning over — you do not have to carry it on your own.")])), (py_str("anger"), py_tuple(vec![py_str("Anger is part of grief for most people, and it does not have to make sense. Let it out somewhere safe, then tell someone who knew them what you are angry about."), py_str("Being angry at them, or at the world, or at whoever was there is common after a death. Say it out loud to one person you trust rather than keeping it in.")])), (py_str("numb"), py_tuple(vec![py_str("That numb feeling is common early on; it is not coldness. Be gentle with yourself today, and tell me or one person close to you if it stays like this."), py_str("Numb is one of the ways grief shows up first. Nothing has to be decided today — just keep eating and sleeping, and let someone know how you are.")])), (py_str("relief"), py_tuple(vec![py_str("Relief after a death is more common than people admit, and it does not cancel the grief. Say it to someone who will not judge it — that is usually enough to make it sit easier."), py_str("Feeling relief does not make you cold. If they were ill for a long time it is a normal thing to feel, and it can sit alongside missing them.")]))]));
    s_HELLO_ON_ITS_OWN(stdlib_call("re", "compile", &[py_str("^(?:hi+|hey+|hello+|hiya+|yo|howdy|morning|good morning|afternoon|good afternoon|evening|good evening|greetings)\\b[\\s,!.]*(?:(?:again|there|all|everyone|everybody|folks|you|mate|love|both)\\b[\\s,!.]*)*(?:(?:how are (?:you|things|you doing|you keeping)|how'?s it going|hows it going|how are we|you ok|you alright|hope you'?re well|its me)\\b[\\s,.!?]*)*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HELLO_AFTER(py_tuple(vec![py_str("Hello again. I'm still here - how are you doing today?"), py_str("Hi. No need to say anything in particular - how are you holding up?"), py_str("Hello. I'm here whenever you want to talk about it, or about anything else."), py_str("Hi again. How has the day been?")]));
    s_TIME_SINCE_REPLIES(py_tuple(vec![py_str("The calendar is a poor measure of it. However long the gap is, it takes the time it takes."), py_str("Dates like that come round whether you are ready or not. Whatever it is like on the day, it is allowed to be like that."), py_str("Most people around you will have stopped counting long before you did. That is not you doing it wrong."), py_str("However long it has been, this is not something you are meant to be over by now. Be as gentle with yourself about it as you would with anybody else."), py_str("It tends to come back in waves rather than fade in a straight line. What has it been like for you lately?")]));
    s_ENDING_SINCE_REPLIES(py_tuple(vec![py_str("The calendar is not the measure of this. However long it has been, it is not a deadline you are behind on."), py_str("Most people are still in pieces at this point and telling nobody. You are not late."), py_str("It does not fade in a straight line, so a bad morning now does not mean you went backwards."), py_str("Being able to say how long it has been is its own kind of progress, even on a day that hurts."), py_str("However long it has been, you are allowed to still mind. That is not you doing it wrong.")]));
    s__THE_ONE_WHO_LEFT(py_str("(?:he|she|they|my\\s+(?:wife|husband|partner|girlfriend|boyfriend|fiance|fiancee|ex|other\\s+half|missus|man|woman))"));
    s_AN_ENDING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:broke\\s+up\\s+with|(?:i|we|they|he|she)\\s+(?:just\\s+|finally\\s+|recently\\s+)?broke\\s+up|break\\s+up|breaking\\s+up|split\\s+up|splitting\\s+up|dumped\\s+me|got\\s+dumped|divorced\\s+(?:me|him|her)|got\\s+divorced|getting\\s+divorced|walked\\s+out\\s+on\\s+me|"), &g__THE_ONE_WHO_LEFT()), &py_str("\\s+left\\s+me|ended\\s+it\\s+with\\s+me|we\\s+are\\s+through|we\\s+are\\s+over)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FEELING_BETTER_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+long|when\\s+will|(?:will|does|is|can)\\s+it(?:\\s+ever)?)\\b.{0,45}?\\b(?:feel|get|be|stop)\\s+(?:better|over\\s+it|myself\\s+again|back\\s+to\\s+normal|hurting|easier|normal|right|ok|okay)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENDING_FEELINGS_REPLIES(py_tuple(vec![py_str("The worst of it usually eases over a few weeks and then comes back in shorter waves. A bad day a month from now is part of it, not a step back."), py_str("There is no set time, and anybody who gives you one is guessing. The mornings usually lift before the evenings do, and the odd day still catches you."), py_str("Most people find the sharp edge goes within a few weeks and the flat days last longer than the crying ones. If months go by with no good day in them, that is worth talking to someone about.")]));
    s__SPAN_COUNT(py_str("a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,2}|few|couple|several"));
    s_A_SPAN(stdlib_call("re", "compile", &[modulo(&py_str("\\b(?:(?:%s)(?:\\s+and\\s+a\\s+half)?\\s+(?:day|week|month|year)s?(?:\\s+and\\s+a\\s+half)?|half\\s+an?\\s+(?:day|week|month|year)s?)\\b"), &g__SPAN_COUNT()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SPAN_NO_NUMBER(stdlib_call("re", "compile", &[py_str("\\b(?:a (?:long|good|fair) (?:time|while)|(?:quite )?a while)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SPAN_LEAD(stdlib_call("re", "compile", &[py_str("\\b(?:been|nearly|almost|just over|over|about|only|coming up|nearly a|a year this week)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SPAN_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:now|this week|today|on|ago|already|next week|coming up|marks?\\b|marking)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SPAN_ELSEWHERE(stdlib_call("re", "compile", &[py_str("\\b(?:since|started|stopped|quit|gave up|off work|signed off|been doing|been on|sober|smoking|drinking|diet|gym|job|course|school|married|together|moved|live|living|rent|paying|treatment|therapy|sober)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MISS_REPLIES(py_tuple(vec![py_str("Of course you do. It goes in waves, and the waves get further apart."), py_str("The ones we miss are the ones who mattered. Say as much or as little as you like."), py_str("That ache is real, and it doesn't keep office hours. Say more whenever you want to."), py_str("Missing like that makes sense. Those days can be long ones — I'm around if talking would help."), py_str("Some days the missing is louder than others. No need to do anything with it here; I'm just listening.")]));
    s_MISS_SUBJECT(py_str("(?:i|we|i'm|im|i've|ive|we're|were|we've|weve)"));
    s_MISS_VERB(stdlib_call("re", "compile", &[add(&add(&py_str("\\b"), &g_MISS_SUBJECT()), &py_str("\\s+(?:really\\s+|so\\s+much\\s+|just\\s+|properly\\s+)?(?:have\\s+|been\\s+|am\\s+|are\\s+)?(?:miss|missing)\\b|\\bmissing\\s+(?:my|our)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MISS_PEOPLE(bitor(&g_PEOPLE(), &call_builtin("frozenset", &[py_tuple(vec![py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("twin"), py_str("twins"), py_str("stepdad"), py_str("stepmum"), py_str("ex"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("mates"), py_str("flatmate"), py_str("roommate"), py_str("bestie")])]).unwrap_or_else(|e| exc::raise(e))));
    s_MISS_NOUN(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:my|our|his|her|their)\\s+(?:[a-z]+\\s+){0,2}(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MISS_PEOPLE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MISS_NOT_BETWEEN(stdlib_call("re", "compile", &[py_str("\\b(?:before|after|without|since|from|during|when|until|like|around|away)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MISS_TAIL(stdlib_call("re", "compile", &[py_str("^\\s*(?:$|[.,;:!?]|\\b(?:today|tonight|so much|a lot|loads|terribly|badly|every day|still|again|now|already|lots|heaps|more than ever|like mad|to bits|awfully)\\b|\\band\\b|\\bbut\\b|\\bso\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MISS_ANIMALS(call_builtin("frozenset", &[mcall(&py_str("dog dogs cat cats puppy kitten rabbit hamster budgie parrot horse pony\npet pets tortoise ferret guinea"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MISS_ANIMAL_NOUNS(call_builtin("frozenset", &[mcall(&py_str("walker walkers sitter sitters food bed beds bowl bowls lead leads\ncollar collars toy toys treat treats vet vets breeder breeders groomer groomers blanket blankets\nbasket baskets door doors lead name"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MISS_PRONOUN(stdlib_call("re", "compile", &[add(&add(&py_str("\\b"), &g_MISS_SUBJECT()), &py_str("\\s+(?:really\\s+|so\\s+much\\s+|just\\s+)?(?:have\\s+|been\\s+|am\\s+|are\\s+)?(?:miss|missing)\\s+(?:him|her|them|mum|dad|nan|gran)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MISS_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:miss|missing)\\s+(?:having|being|doing|going|work|working|school|home|the|a|an|me|you)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_AT_START(stdlib_call("re", "compile", &[py_str("^\\s*(?:do|does|will|would|did|are|is|have|has|can|could|should)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_STATES(call_builtin("frozenset", &[py_tuple(vec![py_str("guilt"), py_str("guilty"), py_str("ashamed"), py_str("shame"), py_str("sad"), py_str("sadness"), py_str("angry"), py_str("anger"), py_str("furious"), py_str("rage"), py_str("numb"), py_str("relief"), py_str("relieved"), py_str("lonely"), py_str("empty"), py_str("hopeless"), py_str("anxious"), py_str("anxiety"), py_str("worried")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|could|should) i (?:stop|deal with|cope with|get rid of|handle|manage)\\b|\\bi (?:just |really |still |keep )?feel(?:ing)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_ONLY_WORDS(call_builtin("frozenset", &[mcall(&py_str("how do does can could should would i me my im am is are was\nwhat when it this that the a an so really still just very much anymore again now bad worse better\nfeel feels feeling stop deal with cope get rid of handle manage about over like why"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_PERSON_WORDS(bitor(&call_builtin("frozenset", &[py_tuple(vec![py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("hers"), py_str("they"), py_str("them"), py_str("their"), py_str("theirs"), py_str("his"), py_str("who")])]).unwrap_or_else(|e| exc::raise(e)), &g_MISS_PEOPLE()));
    s_GRIEF_SELF(call_builtin("frozenset", &[py_tuple(vec![py_str("hard"), py_str("hardest"), py_str("tough"), py_str("rough"), py_str("bad"), py_str("awful"), py_str("horrible"), py_str("heavy"), py_str("hurts"), py_str("hurt"), py_str("sleep"), py_str("sleeping"), py_str("sleepless"), py_str("slept"), py_str("bed"), py_str("nights"), py_str("night"), py_str("wake"), py_str("wakes"), py_str("woke"), py_str("tired"), py_str("exhausted"), py_str("weird"), py_str("strange"), py_str("stuck"), py_str("empty"), py_str("quiet"), py_str("silent"), py_str("silence"), py_str("coping"), py_str("cope"), py_str("eating"), py_str("appetite"), py_str("mornings"), py_str("morning"), py_str("struggling"), py_str("struggle"), py_str("breaking"), py_str("broken"), py_str("withdrawn"), py_str("shut"), py_str("drinking"), py_str("unbearable")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FAMILY(call_builtin("frozenset", &[py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mam"), py_str("dad"), py_str("mother"), py_str("father"), py_str("brother"), py_str("sister"), py_str("son"), py_str("daughter"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("nan"), py_str("nana"), py_str("grandad"), py_str("grandma"), py_str("grandpa"), py_str("granny"), py_str("gran"), py_str("uncle"), py_str("aunt"), py_str("auntie"), py_str("cousin"), py_str("family"), py_str("parents"), py_str("kids"), py_str("children"), py_str("baby")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_OPENERS(py_tuple(vec![py_str("i "), py_str("i'"), py_str("im "), py_str("ime "), py_str("ive "), py_str("id "), py_str("some days"), py_str("some nights"), py_str("most days"), py_str("most nights"), py_str("every night"), py_str("at night"), py_str("lately"), py_str("these days"), py_str("it "), py_str("it's "), py_str("its "), py_str("that "), py_str("that's "), py_str("this "), py_str("they "), py_str("we "), py_str("we're ")]));
    s_SELF_NOUNS(call_builtin("frozenset", &[py_tuple(vec![py_str("sleep"), py_str("night"), py_str("nights"), py_str("mornings"), py_str("mood"), py_str("head"), py_str("appetite"), py_str("grief"), py_str("sadness"), py_str("coping")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("this"), py_str("last"), py_str("next"), py_str("tomorrow"), py_str("yesterday"), py_str("every"), py_str("one"), py_str("that")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEF_REFERRING(py_tuple(vec![py_str("is that normal"), py_str("is it normal"), py_str("is this normal"), py_str("does it get better"), py_str("will it get better"), py_str("does it ever stop"), py_str("when does it stop"), py_str("how long does it last"), py_str("does it get easier"), py_str("will it always"), py_str("feel lost"), py_str("so lost"), py_str("feel empty"), py_str("feel numb")]));
    s_GRIEF_POINTS_BACK(stdlib_call("re", "compile", &[py_str("^(?:so\\s+|and\\s+|but\\s+)?(?:does|did|will|would|is|was|am|when\\s+(?:does|did|will|would|is|am)|how\\s+long\\s+(?:does|did|will|would|is|am))\\s+(?:it|that|this|i|things|the\\s+pain|the\\s+ache|this\\s+pain)\\b(?:\\s+(?:ever|always|still|really|actually|never|not))?\\s*(?:stop|end|ease|pass|fade|last|go\\s+away|let\\s+up|get\\s+easier|get\\s+better|feel\\s+better|feel\\s+normal|feel\\s+like\\s+myself|hurt|hurts|hurting|normal|better|easier|forever)(?:\\s+(?:hurting|aching))?\\b(?:\\s+(?:so|then|eventually|now|at\\s+some\\s+point|after\\s+a\\s+while|for\\s+ever|some\\s*day))?\\s*[.?!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEF_OBJECTS(py_tuple(vec![py_str("without her"), py_str("without him"), py_str("without them"), py_str("her things"), py_str("his things"), py_str("the house"), py_str("the flat"), py_str("her room"), py_str("his room"), py_str("empty house"), py_str("quiet house")]));
    s_REFERS_TO_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:him|her|them|his|hers|their)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_IDEA_ASK(stdlib_call("re", "compile", &[py_str("^(?:what|how|where|which)\\b.*\\b(?:do|does|could|should|can|would|shall)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THREAD_TOPICS(call_builtin("frozenset", &[py_tuple(vec![py_str("feelings"), py_str("grief"), py_str("loss"), py_str("small-talk"), py_str("sleep"), py_str("pets")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THEM(py_str("(?:him|her|them|he|she|they|hes|shes|theyre|his|hers|their|mum|mom|mam|dad|father|mother|nan|gran|granny|nana|grandad|granddad|grandma|grandpa|son|daughter|brother|sister|wife|husband|partner|baby|friend|uncle|aunt|cousin|dog|cat|puppy|kitten)"));
    s_ABSENCE(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:keep|keeps|kept|still)\\s+(?:expecting|expect|looking|look|waiting|wait|listening|listen|hearing|hear|seeing|see|reaching|reach|thinking|think|setting|set|saving|saved)\\b[^.]{0,30}\\b"), &g_THEM()), &py_str("\\b|\\b(?:habit|instinct|reflex|second nature)\\b|\\b(?:house|home|flat|kitchen|bed|place|room)\\s+(?:feels|felt|is|was)\\s+(?:so\\s+|too\\s+|very\\s+)?(?:empty|quiet|silent|big|strange|wrong)\\b|\\b(?:set|laid|lay|set out|lay out)\\s+(?:the\\s+|a\\s+)?table for (?:two|three|four)\\b|\\bfor(?:got|get|gets|getting)\\b[^.]{0,30}\\b(?:gone|dead|not here|isnt here|wouldnt be here)\\b|\\b(?:hear|hearing|smell|smelling)\\s+(?:his|her|their)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_LAYING(stdlib_call("re", "compile", &[py_str("\\b(?:set|setting|lay|laying|laid)\\s+(?:out\\s+)?(?:the\\s+|a\\s+)?table\\s+for\\s+(?:two|three|four)\\b|\\b(?:lay|laying|laid|set|setting|put|putting)\\s+(?:out\\s+)?(?:\\w+\\s+){0,3}(?:place|places|bowl|bowls|plate|plates|cup|cups)\\b(?:\\s+\\w+){0,4}\\s+for\\s+(?:two|three)\\b|\\b(?:keep|keeps|still)\\s+(?:\\w+\\s+){0,2}(?:set|setting|put|putting|lay|laying|buy|buying)\\b[^.]{0,24}\\b(?:bowls?|places?|plates?|cups?|mugs?|glasses|food|biscuits?|treats?)\\b|\\b(?:keep|keeps|kept|still)\\s+(?:\\w+\\s+){0,3}(?:make|making|made|pour|pouring|poured|buy|buying|bought|cook|cooking|cooked|dish|dishing|fill|filling|cut|cutting|butter|buttering|boil|boiling)\\b[^.]{0,24}\\b(?:two|three|both)\\b|\\b(?:two|three|both)\\s+(?:cups?|mugs?|teas?|coffees?|plates?|bowls?|portions?|helpings?|sandwiches?|glasses)\\b[^.]{0,24}\\b(?:for (?:us|two|both of us)|still)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_WITHOUT(stdlib_call("re", "compile", &[py_str("\\b(?:empty|quiet|silent)\\s+without\\s+(?:my|our|him|her|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_CRYING(stdlib_call("re", "compile", &[py_str("\\b(?:cry|cries|cried|crying)\\s+(?:over|about|for)\\s+(?:my|our|him|her|them)\\b|\\b(?:cant|cannot|can'?t)\\s+stop\\s+(?:cry\\w*|think\\w*)\\s+(?:about|over|for)\\s+(?:my|our|him|her|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_AT_THE_DOOR(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|kept|still)\\s+(?:\\w+\\s+){0,2}(?:looking|look|listening|listen|waiting|wait|reaching|reach|expecting|expect|setting|set)\\b[^.]{0,30}\\b(?:door|doorbell|phone|key|keys|ring(?:ing)?|places?|table|seat)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_SEEING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:keep|keeps|kept|still)\\s+(?:\\w+\\s+){0,3}(?:expect\\w*|looking|look|waiting|wait|listening|listen|hearing|hear|seeing|see|reaching|reach|saving|saved)\\b[^.]{0,30}\\b"), &g_THEM()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_ROUTINE_REPLIES(py_tuple(vec![py_str("The hands keep the routine for a while - the second bowl, the place at the table. That is muscle memory, not madness, and you can let it go when it stops being a comfort."), py_str("Laying it out for them is the routine still running. Put it away when you want to, not because anyone says it is time."), py_str("Half of it is the small jobs your hands still do. There is nothing wrong with any of them.")]));
    s_THE_JOBS_FOR_THEM_REPLIES(py_tuple(vec![py_str("The jobs you did for them do not stop on the day. The hands keep going for a while, and that is not a sign you are stuck."), py_str("Some part of the day is still theirs. Doing the job anyway is how a lot of people get through the first year, and there is nothing wrong with it."), py_str("A habit that was for them takes a while to notice as a habit. Do it, or stop, whichever comes first - neither is a betrayal of them.")]));
    s_THE_HOUSE_REPLIES(py_tuple(vec![py_str("An empty house is the hardest part of the day to fill. It does get less loud, and the quiet is not a measure of anything."), py_str("The quiet is the whole of it, and it is worst in the evenings. Nobody gets to tell you there is a timetable for that."), py_str("A house with one fewer person in it feels wrong before it feels like anything else. Be slow with it.")]));
    s_THE_CANNOT_STOP_REPLIES(py_tuple(vec![py_str("Crying over them, or thinking of nothing else, is the plainest way of saying they mattered. It comes in waves - let it, and be kind to yourself after."), py_str("If that is all you can do today, it is enough for today. It eases, and it keeps its own timetable."), py_str("That is grief doing its work, and it is not something to get on top of. I am here for as much of it as you want to say.")]));
    s_THE_HABIT_REPLIES(py_tuple(vec![py_str("Seeing them out of the corner of your eye is the habit outliving them. Most people who have lost someone do it for months, and it says nothing about how you are."), py_str("Seeing them, hearing them, waiting for them - it is a habit that had years to set. It fades slowly, and it is not a sign you are losing them."), py_str("That reflex is the shape of them still in your day. There is nothing to do about it, and it goes with how much of the day they were in.")]));
    s_STILL_DOING_FOR_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|kept|still)\\s+(?:\\w+\\s+){0,3}(?:make|making|pour|pouring|buy|buying|cook|cooking|dish|dishing|lay|laying|set|setting|put|putting|save|saving|leave|leaving|wash|washing|iron|ironing|fold|folding|call|calling|text|texting|ring|ringing|phone|phoning|greet|greeting|kiss|kissing|say|saying|check|checking|fill|filling|clean|cleaning|top|topping|pay|paying|paid|do|doing)\\b[^.]{0,30}\\b(?:his|her|their|him|them|hers|theirs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HABIT_PATTERNS(py_tuple(vec![py_tuple(vec![g_STILL_LAYING(), g_THE_ROUTINE_REPLIES()]), py_tuple(vec![g_STILL_DOING_FOR_THEM(), g_THE_JOBS_FOR_THEM_REPLIES()]), py_tuple(vec![g_STILL_WITHOUT(), g_THE_HOUSE_REPLIES()]), py_tuple(vec![g_STILL_CRYING(), g_THE_CANNOT_STOP_REPLIES()]), py_tuple(vec![g_STILL_SEEING(), g_THE_HABIT_REPLIES()]), py_tuple(vec![g_STILL_AT_THE_DOOR(), g_THE_HABIT_REPLIES()])]));
    s_AFTER_REPLIES(py_tuple(vec![py_str("I'm here. Take your time with it — tell me what it has been like, if you want to."), py_str("There's no rush to talk about it. I'm here when you want to say more."), py_str("That sounds heavy to be carrying. I'm listening, however much or little you want to say."), py_str("I'm glad you said that out loud. Say as much or as little as you like — I'm here.")]));
    s_GRIEF_FEELINGS(call_builtin("frozenset", &[py_tuple(vec![py_str("miss"), py_str("missing"), py_str("cry"), py_str("crying"), py_str("cried"), py_str("tears"), py_str("numb"), py_str("sad"), py_str("lonely"), py_str("angry"), py_str("guilty"), py_str("guilt"), py_str("grief"), py_str("grieving"), py_str("mourning"), py_str("funeral"), py_str("ashes"), py_str("grave"), py_str("anniversary"), py_str("reminded"), py_str("memories"), py_str("remember"), py_str("shock"), py_str("shocked"), py_str("unreal"), py_str("bereaved"), py_str("gutted"), py_str("waves"), py_str("dream"), py_str("dreams"), py_str("dreamt"), py_str("dreamed"), py_str("dreaming"), py_str("voice"), py_str("smell"), py_str("doorbell"), py_str("photo"), py_str("photos")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEF_SELF(call_builtin("frozenset", &[py_tuple(vec![py_str("hard"), py_str("hardest"), py_str("tough"), py_str("rough"), py_str("bad"), py_str("awful"), py_str("horrible"), py_str("heavy"), py_str("hurts"), py_str("hurt"), py_str("sleep"), py_str("sleeping"), py_str("sleepless"), py_str("slept"), py_str("bed"), py_str("nights"), py_str("night"), py_str("wake"), py_str("wakes"), py_str("woke"), py_str("tired"), py_str("exhausted"), py_str("weird"), py_str("strange"), py_str("stuck"), py_str("empty"), py_str("quiet"), py_str("silent"), py_str("silence"), py_str("coping"), py_str("cope"), py_str("eating"), py_str("appetite"), py_str("mornings"), py_str("morning"), py_str("struggling"), py_str("struggle"), py_str("breaking"), py_str("broken"), py_str("withdrawn"), py_str("shut"), py_str("drinking"), py_str("unbearable")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FAMILY(call_builtin("frozenset", &[py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mam"), py_str("dad"), py_str("mother"), py_str("father"), py_str("brother"), py_str("sister"), py_str("son"), py_str("daughter"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("nan"), py_str("nana"), py_str("grandad"), py_str("grandma"), py_str("grandpa"), py_str("granny"), py_str("gran"), py_str("uncle"), py_str("aunt"), py_str("auntie"), py_str("cousin"), py_str("family"), py_str("parents"), py_str("kids"), py_str("children"), py_str("baby")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SELF_OPENERS(py_tuple(vec![py_str("i "), py_str("i'"), py_str("im "), py_str("ime "), py_str("ive "), py_str("id "), py_str("some days"), py_str("some nights"), py_str("most days"), py_str("most nights"), py_str("every night"), py_str("at night"), py_str("lately"), py_str("these days"), py_str("it "), py_str("it's "), py_str("its "), py_str("that "), py_str("that's "), py_str("this "), py_str("they "), py_str("we "), py_str("we're ")]));
    s_SELF_NOUNS(call_builtin("frozenset", &[py_tuple(vec![py_str("sleep"), py_str("night"), py_str("nights"), py_str("mornings"), py_str("mood"), py_str("head"), py_str("appetite"), py_str("grief"), py_str("sadness"), py_str("coping")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("this"), py_str("last"), py_str("next"), py_str("tomorrow"), py_str("yesterday"), py_str("every"), py_str("one"), py_str("that")])]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEF_REFERRING(py_tuple(vec![py_str("is that normal"), py_str("is it normal"), py_str("is this normal"), py_str("does it get better"), py_str("will it get better"), py_str("does it ever stop"), py_str("when does it stop"), py_str("how long does it last"), py_str("does it get easier"), py_str("will it always"), py_str("feel lost"), py_str("so lost"), py_str("feel empty"), py_str("feel numb")]));
    s_GRIEF_POINTS_BACK(stdlib_call("re", "compile", &[py_str("^(?:so\\s+|and\\s+|but\\s+)?(?:does|did|will|would|is|was|am|when\\s+(?:does|did|will|would|is|am)|how\\s+long\\s+(?:does|did|will|would|is|am))\\s+(?:it|that|this|i|things|the\\s+pain|the\\s+ache|this\\s+pain)\\b(?:\\s+(?:ever|always|still|really|actually|never|not))?\\s*(?:stop|end|ease|pass|fade|last|go\\s+away|let\\s+up|get\\s+easier|get\\s+better|feel\\s+better|feel\\s+normal|feel\\s+like\\s+myself|hurt|hurts|hurting|normal|better|easier|forever)(?:\\s+(?:hurting|aching))?\\b(?:\\s+(?:so|then|eventually|now|at\\s+some\\s+point|after\\s+a\\s+while|for\\s+ever|some\\s*day))?\\s*[.?!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GRIEF_OBJECTS(py_tuple(vec![py_str("without her"), py_str("without him"), py_str("without them"), py_str("her things"), py_str("his things"), py_str("the house"), py_str("the flat"), py_str("her room"), py_str("his room"), py_str("empty house"), py_str("quiet house")]));
    s_REFERS_TO_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:him|her|them|his|hers|their)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_IDEA_ASK(stdlib_call("re", "compile", &[py_str("^(?:what|how|where|which)\\b.*\\b(?:do|does|could|should|can|would|shall)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THREAD_TOPICS(call_builtin("frozenset", &[py_tuple(vec![py_str("feelings"), py_str("grief"), py_str("loss"), py_str("small-talk"), py_str("sleep"), py_str("pets")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THEM(py_str("(?:him|her|them|he|she|they|hes|shes|theyre|his|hers|their|mum|mom|mam|dad|father|mother|nan|gran|granny|nana|grandad|granddad|grandma|grandpa|son|daughter|brother|sister|wife|husband|partner|baby|friend|uncle|aunt|cousin|dog|cat|puppy|kitten)"));
    s_ABSENCE(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:keep|keeps|kept|still)\\s+(?:expecting|expect|looking|look|waiting|wait|listening|listen|hearing|hear|seeing|see|reaching|reach|thinking|think|setting|set|saving|saved)\\b[^.]{0,30}\\b"), &g_THEM()), &py_str("\\b|\\b(?:habit|instinct|reflex|second nature)\\b|\\b(?:house|home|flat|kitchen|bed|place|room)\\s+(?:feels|felt|is|was)\\s+(?:so\\s+|too\\s+|very\\s+)?(?:empty|quiet|silent|big|strange|wrong)\\b|\\b(?:set|laid|lay|set out|lay out)\\s+(?:the\\s+|a\\s+)?table for (?:two|three|four)\\b|\\bfor(?:got|get|gets|getting)\\b[^.]{0,30}\\b(?:gone|dead|not here|isnt here|wouldnt be here)\\b|\\b(?:hear|hearing|smell|smelling)\\s+(?:his|her|their)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_REASSURING(stdlib_call("re", "compile", &[py_str("\\b(?:the right (?:thing|call|decision)|did the (?:right|kindest|best) thing|for the best|the kindest thing|no other choice|(?:he|she|they) had a good (?:life|innings)|would(?:'ve| have) wanted|at peace|no more pain|not in pain|out of (?:his|her|their) pain|no longer suffering)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PEACE_IS_MINE(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)(?:'?m|'?ve|\\s+am|\\s+are|\\s+have|\\s+feel|\\s+feeling|\\s+felt)?\\s+(?:now\\s+|so\\s+|really\\s+|quite\\s+|a bit\\s+|more\\s+)*at (?:peace|rest)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WOULD_HAVE(stdlib_call("re", "compile", &[py_str("\\bwould\\s+(?:have|ve)\\s+(?:been|turned)\\s+(?:\\d{1,3}\\b|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)")]).unwrap_or_else(|e| exc::raise(e)));
    s_OCCASION(stdlib_call("re", "compile", &[py_str("\\b(birthday|birthdays|xmas|christmas|anniversary|anniversaries|mothers day|mothering sunday|fathers day|new year|new years|easter|the day itself|her day|his day|their day)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WITHOUT(stdlib_call("re", "compile", &[py_str("\\bwithout\\s+(?:my|our|his|her|them|him|mum|dad)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TRADITION(stdlib_call("re", "compile", &[py_str("\\b(we always|always did|always had|every year|used to|tradition|same as always|instead|what do i do|dont know what to do|don t know what to do|nothing feels right)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_KEEP(stdlib_call("re", "compile", &[py_str("\\b(?:should i|shall i|do you think i should|am i supposed to|is it (?:ok|okay|alright|wrong) (?:to|if i))\\s+still\\s+(?:do|have|bake|cook|make|put|keep|send|host|hang|light|set out|visit)\\b|\\bis it (?:ok|okay|alright|wrong) to (?:put up|put|do|keep|hang) (?:the|a|our) (?:tree|decorations|lights|wreath|cake|cards?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_THING(stdlib_call("re", "compile", &[py_str("\\b(cakes?|tree|dinner|lunch|meal|roast|turkey|cards?|presents?|gifts?|decorations|wreath|flowers|party|tradition|toast|grave|christmas|xmas|birthday|anniversary)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:what|how)\\s+(?:should|do|can|could)\\s+(?:i|we)\\s+(?:do|mark|handle|spend|keep)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS(py_tuple(vec![g_TRADITION(), g_STILL_KEEP()]));
    s_FUNERAL(stdlib_call("re", "compile", &[py_str("\\b(funeral|memorial|service|viewing|burial|cremation)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FUNERALS(stdlib_call("re", "compile", &[py_str("\\b(?:funerals|memorials|burials|cremations|crematoria|crematorium|graveside|gravesides)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TELL_CHILD(stdlib_call("re", "compile", &[py_str("\\b(?:tell|telling|explain|explaining|talk to|talking to)\\b([\\w\\s]{0,24}?)\\b(?:child|children|kid|kids|son|daughter|toddler|boy|girl|baby|teenager)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_YEAR_OLD(stdlib_call("re", "compile", &[py_str("\\d+\\s*(?:year|yr)s?[\\s-]*old")]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("him"), py_str("her"), py_str("them")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_ABOUT_THE_DAY(stdlib_call("re", "compile", &[py_str("\\b(what (?:do|should|can|could) i|what (?:do|would) we|should i|shall i|how do i|how can i|should we|shall we|do we|would we|can we|could we|how do we|any ideas?|what to do|instead|nothing feels right|dont know|don t know|do you think|is it (?:ok|okay|alright|wrong)|am i supposed to)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_OF_DAY(stdlib_call("re", "compile", &[py_str("^\\s*the\\s+(?:mornings?|nights?|evenings?|weekends?|days?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_TAIL(call_builtin("frozenset", &[py_tuple(vec![py_str("and"), py_str("but"), py_str("or"), py_str("so"), py_str("too"), py_str("all"), py_str("at"), py_str("in"), py_str("on"), py_str("to"), py_str("with"), py_str("when"), py_str("since"), py_str("because"), py_str("today"), py_str("tonight"), py_str("now"), py_str("still"), py_str("lately"), py_str("already"), py_str("again"), py_str("yet"), py_str("just"), py_str("quite"), py_str("very"), py_str("really"), py_str("bit"), py_str("as"), py_str("like"), py_str("about"), py_str("for"), py_str("of"), py_str("that"), py_str("it"), py_str("i"), py_str("im"), py_str("i'm"), py_str("this"), py_str("these"), py_str("some"), py_str("most"), py_str("every"), py_str("days"), py_str("day"), py_str("weeks"), py_str("week"), py_str("months"), py_str("time"), py_str("times"), py_str("though"), py_str("honestly"), py_str("actually"), py_str("sometimes"), py_str("if"), py_str("than"), py_str("without"), py_str("after"), py_str("before"), py_str("anymore"), py_str("any"), py_str("more"), py_str("less"), py_str("enough")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_STATE(call_builtin("frozenset", &[py_tuple(vec![py_str("eat"), py_str("eats"), py_str("ate"), py_str("eating"), py_str("appetite"), py_str("hungry"), py_str("food"), py_str("meal"), py_str("meals"), py_str("drink"), py_str("drinks"), py_str("drank"), py_str("drinking"), py_str("alcohol"), py_str("wine"), py_str("sleep"), py_str("sleeps"), py_str("sleeping"), py_str("slept"), py_str("weight"), py_str("cry"), py_str("crying")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THEMSELVES(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|me|my|myself)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_NORMAL(stdlib_call("re", "compile", &[py_str("^is (?:it|that) normal\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NORMAL_TO_FEEL(stdlib_call("re", "compile", &[py_str("^is (?:it|that|this) (?:normal|ok|okay|common|usual) to feel\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUANTITY(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURE_TOPICS(call_builtin("frozenset", &[py_tuple(vec![py_str("maths-and-numbers")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_ANSWER(call_builtin("frozenset", &[py_tuple(vec![py_str("eat"), py_str("eats"), py_str("ate"), py_str("eating"), py_str("appetite"), py_str("hungry"), py_str("food"), py_str("meal"), py_str("meals"), py_str("drink"), py_str("drinks"), py_str("drank"), py_str("drinking"), py_str("thirsty"), py_str("sleep"), py_str("sleeps"), py_str("sleeping"), py_str("slept"), py_str("wake"), py_str("wakes"), py_str("waking"), py_str("woke"), py_str("awake"), py_str("insomnia"), py_str("tired"), py_str("weight")])]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_ABOUT_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|him|her|they|them|his|hers|their|theirs)\\b|\\b(?:keep|keeps|kept|still|always|expect|expecting|used to|every (?:day|night|year)|last (?:week|month|year)|anniversary|birthday|that (?:day|night|song|photo|place))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_THEM_OPENS(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|so|yeah|yes|oh|well|then|also|plus|too|god|honestly)\\b[\\s,]*)*(?:he|she|they|him|her|his|their|them|theyre|he's|she's|he'd|she'd|they'd|hes|shes)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MISSING_SAID_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:miss|missed|misses|missing|cry|cried|crying|tears|sad|sadness|ache|aches|aching|keep|keeps|kept|still|always|expect|expecting|hard|hardest|heavy|hurts?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CONTRACTED(stdlib_call("re", "compile", &[py_str("\\b(\\w+)['\\u2019]?s\\s+(?=(?:dying|terminal|terminally|palliative|in\\s+(?:a\\s+)?hospice)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__PEOPLE_PRONOUNS(call_builtin("frozenset", &[py_tuple(vec![py_str("she"), py_str("he"), py_str("they"), py_str("someone"), py_str("anyone"), py_str("everyone"), py_str("nobody"), py_str("who")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DYING_TO(stdlib_call("re", "compile", &[py_str("\\bdying\\s+to\\b|\\bdying\\s+for\\b|\\bdying\\s+of\\s+(?:boredom|embarrassment|laughter|curiosity|shame|excitement|a\\s+cold)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_CONTRACTION(call_builtin("frozenset", &[py_tuple(vec![py_str("is"), py_str("was"), py_str("has"), py_str("does"), py_str("this"), py_str("its"), py_str("his"), py_str("hers"), py_str("ours"), py_str("theirs")])]).unwrap_or_else(|e| exc::raise(e)));
    s__CAN_BE_DYING(bitor(&call_builtin("frozenset", &[g_PEOPLE()]).unwrap_or_else(|e| exc::raise(e)), &g__PEOPLE_PRONOUNS()));
    s_FINDABLE(py_str("keys?|key ?ring|phone|mobile|wallet|purse|handbag|backpack|rucksack|bag|passport|tickets?|(?:(?:bank|debit|credit|travel|id|library) )?cards?|glasses|specs|headphones|earphones|earbuds|airpods|charger|laptop|tablet|mouse|umbrella|coat|jacket|scarf|glove|hat|shoes|trainers|book|file|documents?|parcel|package|remote|toy|teddy"));
    s_LOST_THING_TO_FIND(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:i|we|ive|i've|he|she|they|you|my [a-z]+|our [a-z]+)\\s+(?:have\\s+|had\\s+)?(?:lost|losing)\\s+(?:my|our|his|her|their|your|the|a|an)\\s+(?:"), &g_FINDABLE()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_CANT_FIND_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:(?:i|we|he|she|they)\\s+)?(?:can ?not|can't|cant|couldn't|could not)\\s+(?:find|see)\\s+(?:my|our|his|her|their|the|a|an)\\s+(?:"), &g_FINDABLE()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEFT_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:i|we|he|she|they)(?:\\s+think i)?\\s+(?:left|forgot|lost track of)\\s+(?:my|our|his|her|their|the|a|an)\\s+(?:"), &g_FINDABLE()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMEMBER_NOT_WHERE(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:forgot|forget|can ?not remember|can't remember|cant remember|do not remember|don't remember|dont remember|no idea|not sure|no clue)\\s+where\\s+(?:i|we|he|she|they)\\s+(?:have\\s+|had\\s+)?(?:put|left|laid|placed)\\s+(?:my|our|his|her|their|the|a|an)\\s+(?:"), &g_FINDABLE()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEP_LOSING_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:i|we)\\s+keep\\s+(?:losing|misplacing)\\s+(?:my|our|the)\\s+(?:"), &g_FINDABLE()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEPT_GOING_MISSING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:my|our|his|her|their|the)\\s+(?:"), &g_FINDABLE()), &py_str(")\\s+(?:keeps?|kept)\\s+(?:going\\s+missing|disappearing|vanishing|getting\\s+lost|walking)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_GONE_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:my|our|his|her|their)\\s+(?:"), &g_FINDABLE()), &py_str(")\\s+(?:is|are|has gone|have gone)\\s+(?:gone|missing|nowhere)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_IS_THING(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:where(?:'s| is| are)|where has|where have|where did i (?:put|leave)|where have i put|have you seen|has anyone seen|did you see|any sign of)\\s+(?:my|our|his|her|their|the|a|an)\\s+(?:"), &g_FINDABLE()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_I_PUT_IT(stdlib_call("re", "compile", &[py_str("\\bwhere\\s+(?:did|have|has|had)\\s+i\\s+(?:put|leave)\\b(?:\\s+(?:my|our|his|her|their|the|a|an))?")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_MISLAID(bitor(&g_PEOPLE(), &call_builtin("frozenset", &[py_tuple(vec![py_str("job"), py_str("jobs"), py_str("shift"), py_str("shifts"), py_str("foot"), py_str("feet"), py_str("finger"), py_str("fingers"), py_str("hand"), py_str("hands"), py_str("mind"), py_str("way"), py_str("place"), py_str("bins"), py_str("bin"), py_str("rubbish"), py_str("washing"), py_str("recycling"), py_str("ironing"), py_str("football"), py_str("weight"), py_str("effort"), py_str("record"), py_str("case")])]).unwrap_or_else(|e| exc::raise(e))));
    s_NOT_A_THING_NEXT(call_builtin("frozenset", &[py_tuple(vec![py_str("out"), py_str("away"), py_str("back"), py_str("down"), py_str("off"), py_str("and"), py_str("but"), py_str("or"), py_str("so"), py_str("then"), py_str("i"), py_str("it"), py_str("was"), py_str("were"), py_str("is"), py_str("are"), py_str("that"), py_str("this"), py_str("for"), py_str("to"), py_str("in"), py_str("on"), py_str("at"), py_str("with"), py_str("from"), py_str("again"), py_str("somewhere"), py_str("before"), py_str("after"), py_str("today"), py_str("yesterday"), py_str("last"), py_str("now"), py_str("though"), py_str("honestly"), py_str("probably"), py_str("right"), py_str("anyway")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_IS_THING_BARE(stdlib_call("re", "compile", &[add(&add(&py_str("^\\s*(?:so |ok |okay |and |but )?where\\s+(?:my|our|his|her|their|the|a|an)\\s+(?:"), &g_FINDABLE()), &py_str(")\\s*$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_LOCKED_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:locked out|lock(?:ed)? (?:me|myself|us) out|shut out of (?:the|my|our) (?:flat|house|home|car|van)|keys? (?:are |is )?(?:still )?(?:in|inside) (?:the|my) (?:door|lock|ignition)|lock(?:ed)? (?:my|the) keys? (?:in|inside) (?:the|my) (?:car|van|house|flat|boot|door)|no spare key)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DOOR(stdlib_call("re", "compile", &[py_str("\\b(?:account|email|e-mail|bank|banking|online|app|website|site|password|login|log in|profile|facebook|instagram|twitter|whatsapp|icloud|google|netflix|laptop|computer|phone|tablet|xbox|playstation|steam|psn|system|server|admin|router|wifi|cat|dog|puppy|kitten|pet|bird|hen|chicken|rabbit|hamster)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CAR_LOCK_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:car|van|boot|ignition|keys? (?:are |is )?in the (?:car|van))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOUND_WHAT_WAS_LOST(stdlib_call("re", "compile", &[py_str("\\b(?:found (?:them|it|ours|his|hers|mine|my [a-z]+)|(?:it|they|he|she|my [a-z]+) (?:has |have )?(?:turned up|was in|were in|was under|were under|was behind|were behind|was on|were on|was at|were at|showed up))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SHED(stdlib_call("re", "compile", &[py_str("\\s+(?:weight|fat|pounds|stone|belly|blubber|tummy|tummies|inches|waist|middle|kilos?|kilograms?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FEARED_NOT_YET(stdlib_call("re", "compile", &[py_str("\\b(?:dont|don't|do not|doesnt|doesn't|never|hope|hoping|hoped|scared|afraid|worried|worry|worrying|dread|dreading|if|unless|before|hate|bear)\\b[^.!?]{0,24}?\\b(?:lose|loses|losing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_DIED(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|dead|dies|passed away|passed on|put down|put to sleep|euthanised|euthanized|pts)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_IT_DIED(stdlib_call("re", "compile", &[py_str("\\bit\\s+(?:just\\s+|has\\s+|had\\s+|finally\\s+)?(?:died|dies)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PASSIVE_AUX(py_tuple(vec![py_str("be"), py_str("being"), py_str("been"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("is"), py_str("was"), py_str("are"), py_str("were"), py_str("will"), py_str("to")]));
    s_REMAINS(py_tuple(vec![py_str("ashes"), py_str("body"), py_str("bodies"), py_str("coffin"), py_str("coffins"), py_str("casket"), py_str("caskets"), py_str("urn"), py_str("urns"), py_str("remains"), py_str("grave"), py_str("graves"), py_str("plot"), py_str("plots")]));
    s_BURIABLE(py_tuple(vec![py_str("post"), py_str("posts"), py_str("fence"), py_str("fences"), py_str("fencing"), py_str("panel"), py_str("panels"), py_str("gate"), py_str("gates"), py_str("cable"), py_str("cables"), py_str("wire"), py_str("wires"), py_str("pipe"), py_str("pipes"), py_str("hose"), py_str("hosepipe"), py_str("drain"), py_str("drains"), py_str("treasure"), py_str("bulb"), py_str("bulbs"), py_str("seed"), py_str("seeds"), py_str("tank"), py_str("tub"), py_str("rubbish"), py_str("waste"), py_str("compost"), py_str("manure"), py_str("sand"), py_str("gravel"), py_str("rubble"), py_str("shed"), py_str("trampoline")]));
    s_CHILDS_OWN_LOSS(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+(?:son|sons|daughter|daughters|child|children|kid|kids|boy|girl|lad|lass|teenager|baby|toddler|stepsons?|stepdaughters?|stepchildren)\\b[^.]{0,40}\\b(?:his|her|their)\\s+(?:dad|daddy|mum|mummy|father|mother|parent|parents|brother|sister|grandad|grandads|granddad|grandma|grandmas|gran|grans|nan|nans|grandpa|grandparents)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RELATIVE_PET_REPLIES(py_tuple(vec![py_str("Oh, that's hard on them. A {one} is family to the person who had it — say straight out that you're sorry, and ask how they're doing."), py_str("That's a real loss for them, not a small one. Say it plainly (\"I'm so sorry about the {animal}\"), then check in again in a week or so."), py_str("Pets are family, so treat it the way you would a family death for them: say you're sorry about the {animal}, and offer one concrete thing if you're close enough.")]));
    s_STRANGER_ANIMAL_REPLIES(py_tuple(vec![py_str("That's sad. A {one} you only see over the fence is still part of the day, and the street is quieter without it."), py_str("Sorry to hear that. You don't have to own a {one} to miss one — it's a real gap when one that was always about goes."), py_str("Oh no. Even a neighbour's {animal} leaves a quiet patch behind it, and noticing that isn't silly.")]));
    s_TWO_WORD_PETS(py_tuple(vec![py_tuple(vec![py_str("guinea"), py_str("pig")]), py_tuple(vec![py_str("guinea"), py_str("pigs")])]));
    s_PLURAL_PAIRS(py_dict(vec![(py_str("cats"), py_str("cat")), (py_str("dogs"), py_str("dog")), (py_str("rabbits"), py_str("rabbit")), (py_str("bunnies"), py_str("bunny")), (py_str("hamsters"), py_str("hamster")), (py_str("budgies"), py_str("budgie")), (py_str("parrots"), py_str("parrot")), (py_str("horses"), py_str("horse")), (py_str("ponies"), py_str("pony")), (py_str("ferrets"), py_str("ferret")), (py_str("rats"), py_str("rat")), (py_str("tortoises"), py_str("tortoise")), (py_str("snakes"), py_str("snake")), (py_str("lizards"), py_str("lizard")), (py_str("gerbils"), py_str("gerbil")), (py_str("chinchillas"), py_str("chinchilla")), (py_str("cockatiels"), py_str("cockatiel")), (py_str("guinea pigs"), py_str("guinea pig")), (py_str("puppies"), py_str("puppy")), (py_str("kittens"), py_str("kitten")), (py_str("fish"), py_str("fish")), (py_str("mice"), py_str("mouse"))]));
    s_DEATH_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:died|dies|dying|dead|death|passed|put\\s+down|put\\s+to\\s+sleep|euthanised|euthanized|killed|pts|gone|loss|lost)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SWAP_ASK(stdlib_call("re", "compile", &[py_str("^(?:and|but|also|what\\s+about|how\\s+about|what\\s+(?:of|say)|and\\s+what\\s+about)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SWAP_MOST_WORDS(Py::Int(8));
    s_ASKS(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:what|which|how|when|where|who|why|should|shall|can|could|do|does|did|is|are|was|were|will|would|must|have|has)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SWAP_INHERITS(stdlib_call("re", "compile", &[py_str("\\b(?:wear|wearing|put\\s+on|bring|brings|take|give|send|order|buy|cook|make|pack|choose|pick|pay|cost|costs)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__HELPERS(stdlib_call("re", "compile", &[py_str("\\b(?:doctor|doctors|gp|gps|consultant|nurse|nurses|vet|vets|hospice|hospital|carers?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPING_JUDGED(stdlib_call("re", "compile", &[py_str("\\b(?:cruel|cruelty|unkind|wrong|selfish|unfair|mean|nasty)\\b[^.?!]{0,60}?\\b(?:keep|keeping|kept)\\b(?=[^.?!]{0,50}\\b(?:going|alive)\\b)|\\b(?:keep|keeping|kept)\\b(?:\\s+\\w+){0,4}\\s+(?:going|alive)\\b[^.?!]{0,50}?\\b(?:cruel|cruelty|unkind|wrong|selfish|unfair)\\b|\\b(?:cruel|cruelty|unkind|wrong|selfish|unfair)\\b[^.?!]{0,60}?\\b(?:put|putting|having\\s+(?:\\w+\\s+){0,3}put)\\b(?:\\s+\\w+){0,4}\\s+(?:down|to\\s+sleep)\\b|\\b(?:cruel|cruelty|unkind|wrong|selfish|unfair)\\b[^.?!]{0,60}?\\blet\\s+(?:\\w+\\s+)?go\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ENDING_ASKED(stdlib_call("re", "compile", &[py_str("^(?:(?:and|so|but|oh|god|honestly|now|then|ok|okay)\\s+)*(?:should|shall|ought|when|how|do|does|is|will)\\b(?![^.?!]{0,40}\\b(?:bed|bedtime|crate|kennel|lead|garden|room|downstairs|upstairs|outside|night|nights|nap|naps)\\b)[^.?!]{0,40}?\\b(?:put|putting|have\\s+(?:\\w+\\s+){0,3}put|let\\s+(?:\\w+\\s+)?go)\\b(?:\\s+\\w+){0,4}\\s+(?:down|to\\s+sleep)\\b|^(?:(?:and|so|but|oh|honestly|now|then)\\s+)*(?:when|how)\\b[^.?!]{0,30}?\\b(?:know|tell|decide|sure)\\b[^.?!]{0,30}?\\b(?:its|it's|the)\\s+(?:time|right\\s+time|end)\\b|^(?:(?:and|so|but|oh|honestly|now|then)\\s+)*is\\s+it\\s+(?:the\\s+|the\\s+right\\s+)?time\\b[^.?!]{0,20}\\b(?:to\\s+)?(?:let|put|say\\s+goodbye|stop|end)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPING_GOING_REPLIES(py_tuple(vec![py_str("Not cruel, no. Wanting to keep a pet with you is the ordinary way love behaves, and asking the question is what someone who is still looking after them does. What helps is a written week - did they eat, settle, sleep, enjoy one thing - because a change is easier to see over a fortnight than in one bad night."), py_str("That is not selfish, and it is not a vote. Ask the vet what they would do with their own animal and ask them to be blunt about pain and appetite, then keep a note of the good days against the bad ones - most people find the answer is in that note rather than in the guilt."), py_str("Asking it says most of what needs saying: you have not stopped looking after them. Whether keeping on is fair depends on the animal, not on you - eating, drinking, pain, one thing still enjoyed. Take that list to the vet and let them tell you what the next weeks look like, rather than working it out alone."), py_str("No, not cruel. There is no scoreboard for this, and the person who is doing it for the animal's sake is usually the one who feels worst about it. The useful thing is the vet's honest view plus a normal week written down: food, drink, sleep, pain, one good thing a day.")]));
    s_IS_IT_TIME_REPLIES(py_tuple(vec![py_str("There is no single day, and waiting for a sign that everyone agrees on is what makes this the hardest part. What people use: more bad days than good, no interest in food or in the things they loved, pain the vet cannot get on top of. Ask the vet to go through a quality-of-life list with you - it turns a guess into something you can look at."), py_str("Only you and the vet can say, and the vet has done this far more often than you have. Write down a normal day for the next week - eating, drinking, moving, sleeping, one thing they still enjoy - and take that to the appointment; a fortnight of it usually makes the call clearer than any single bad night."), py_str("It is a judgement rather than a date, and the ones that go wrong are the ones made in a panic at 2am. Book a talk with the vet about quality of life and ask them plainly: would they wait, and what would make them stop waiting?")]));
    s_SAY_GOODBYE(stdlib_call("re", "compile", &[py_str("\\b(?:say\\s+good\\s?bye|good\\s?bye\\s+to\\s+(?:him|her|them)|last\\s+words|what\\s+(?:do|should|can)\\s+i\\s+say\\b|what\\s+to\\s+say\\b|how\\s+(?:do|can)\\s+i\\s+say\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAY_TO_WHO(stdlib_call("re", "compile", &[py_str("\\b(?:say|talk)\\s+to\\s+(?:(?:the|my|our|his|her|their|your)\\s+)?(\\w+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_VISIT_TARGETS(call_builtin("frozenset", &[py_tuple(vec![py_str("him"), py_str("her"), py_str("them"), py_str("himself"), py_str("herself"), py_str("themselves")])]).unwrap_or_else(|e| exc::raise(e)));
    s_RITUAL(stdlib_call("re", "compile", &[py_str("\\b(?:we (?:always|used to)|every year|her birthday|his birthday|their birthday|anniversary|(?:first )?(?:christmas|birthday|mothers day|fathers day) without)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_RITUAL_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:her|his|their|mum|mom|mam|dad|mother|father|nan|nana|gran|granny|grandad|grandma|grandpa|husband|wife|partner|son|daughter|brother|sister|birthday|anniversary|christmas)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THREAD_TURNS(Py::Int(6));
    s_PAPER_ASK(stdlib_call("re", "compile", &[py_str("\\b(probate|grant of probate|letters of administration|estate|paperwork|solicitor|wills|(?:the|his|her|their|my|our|a) will|inheritance tax|death certificate|register (?:the |his |her |their )?death)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GIFT_THING_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:to|should i|shall i|can i|could i|do i|would you|should we|can we|do we)\\s+(?:buy|bring|get|take|give|make)\\b|\\bwhat (?:do|would|could|should) you\\s+(?:buy|bring|get|take|give)\\b|\\b(?:dont|don't|do not|not)\\s+know what to\\s+(?:buy|bring|get|take|give)\\b|\\b(?:gift|present)\\s+(?:idea|ideas)\\b|\\b(?:a|any) gift\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_CARE(stdlib_call("re", "compile", &[py_str("\\b(?:hospice|hospital|ward|icu|hdu|hospice bed)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_BEDSIDE(stdlib_call("re", "compile", &[py_str("\\b(funeral|funerals|wake|burial|cremation|memorial|service|church|grave|graveside|scattering)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_POOLS(add(&py_tuple(vec![g_LOSS_REPLIES(), g_SAY_REPLIES(), g_CARD_REPLIES(), g_GRIEF_REPLIES(), g_FUNERAL_REPLIES(), g_FUNERAL_COST_COMPARE_REPLIES(), g_FUNERAL_HELP_REPLIES(), g_REMEMBER_REPLIES(), g_FUNERAL_WHEN_REPLIES(), g_AFTER_REPLIES(), g_HELLO_AFTER(), g_MISS_REPLIES(), g_THE_ROUTINE_REPLIES(), g_THE_HOUSE_REPLIES(), g_THE_CANNOT_STOP_REPLIES(), g_THE_HABIT_REPLIES(), g_THE_JOBS_FOR_THEM_REPLIES(), g_GETTING_THROUGH_REPLIES()]), &call_builtin("tuple", &[mcall(&g_FEELING_REPLIES(), "values", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s_SAFETY_MODULES(py_tuple(vec![py_str("safety")]));
    s_FALLBACK_MODULES(py_tuple(vec![py_str("greeting")]));
    s_ENDING_TURNS(Py::Int(3));
}
pub fn _only_a_new_subject(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let _ = py_str("True when the line only puts a different thing in place of the earlier one.");
    found = mcall(&g_CORRECTS(), "match", &[mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "strip", &[py_str(" .!?,")]);
    return { let __b = call_builtin("bool", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = le(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DIED(), "search", &[rest.clone()]))) } } } };
    Py::None
}
pub fn _only_the_proceedings(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let _ = py_str("True when the line only asks about funerals as a thing, so the knowledge base answers it.");
    if py_truthy(&{ let __b = mcall(&g_A_DEATH_HAPPENED(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { _has_person(&[words.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _unnamed_person(&[words.clone(), clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THIS_ONE(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_ASK_WORDS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CARD_WORD(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_A_PRICE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PROCEEDINGS_ASK(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _killed_in_a_story(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True when the thing \"killed in/on\" points at is a story, not an incident.");
    if py_truthy(&mcall(&g_GAME_TITLES(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    found = mcall(&g_KILLED_IN(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&mcall(&slice_or_raise(&clean.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "split", &[]), &Py::None, &Py::Int(4), &Py::None));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_STORY_INCIDENT(), &word.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_STORY_MEDIUM(), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _dead_is_not_a_death(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut following: Py = Py::None;
    let _ = py_str("True when the \"dead\" at `index` is not a death.\n\n    Either it describes the word after it (\"dead birds\", \"dead leaves on the lawn\") or the two\n    words are an idiom (\"dead easy\", \"dead against it\", \"the battery is dead\").\n    ");
    if py_truthy(&ge(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(false);
    }
    following = geti(&words.clone(), &add(&index.clone(), &Py::Int(1)));
    return { let __b = Py::Bool(py_contains(&g_DEAD_IDIOM(), &following.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DEAD_PREDICATE(), &following.clone())) } };
    Py::None
}
pub fn is_dying_now(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when someone of ours is dying but not dead: \"my dad is dying\", \"she's terminal\".\n\n    Kept apart from `reads_as_loss` on purpose - the reading is the reader's, and this only\n    decides which wording fits. A pet dying counts: the dog in front of them has not gone yet.\n    ");
    said = if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { text.clone() } else { mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]) };
    said = _spoken(&[said.clone()]);
    clean = _tight(&[said.clone()]);
    words = words_of(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_DYING_NOW(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = _the_end_is_near(&[clean.clone(), words.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_AT_THE_END(), "search", &[clean.clone()]) } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_DYING_ALREADY(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__WORKING_WITH_THE_DYING(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__SAID_DYING(), "search", &[clean.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _the_end_is_near(&[clean.clone(), words.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DYING_NOW(), "search", &[clean.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_family_is_told(&[clean.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &bitor(&bitor(&g_HUMAN_WORDS(), &g_PET_WORDS()), &g__PEOPLE_PRONOUNS()))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { _a_role_phrase(&[clean.clone()]) } };
    Py::None
}
pub fn dying_in_the_chat(args: &[Py]) -> Py {
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
    let mut turns: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { Py::Int(6) };
    let mut history: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when a line earlier in this chat said that someone of ours is dying.\n\n    The question after it (\"is it normal to cry all the time\", \"how do i cope\") is about the\n    person asking, and it says nothing about a death, so on its own it reads as a bereavement\n    and gets the answer for after the funeral. The chat already knows which week this is.\n    ");
    history = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[history.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&history.clone(), &neg(&turns.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    said = __item.clone();
    text = if py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&said.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { is_dying_now(&[text.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_pet_of_ours(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut said: Py = Py::None;
    let _ = py_str("True when the one dying is an animal, so the vet has the timeline and not a hospice.");
    if py_truthy(&Py::Bool(py_is(&words.clone(), &Py::None))) {
    words = words_of(&[line.clone()]);
    }
    said = call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[bitand(&said.clone(), &g_PET_WORDS())]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("bool", &[bitand(&said.clone(), &g_HUMAN_WORDS())]).unwrap_or_else(|e| exc::raise(e)))) } };
    Py::None
}
pub fn _the_end_is_near(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut somebody: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("True when the line says the treatment has run out or the days are counted.\n\n    These are the sentences somebody's dying arrives in, and none of them has a death word in it:\n    \"the doctor said theres nothing more they can do\", \"the vet says theres not much more they can\n    do\", \"theyve stopped mums chemo\", \"the hospital said theres nothing else they can try\", \"the\n    consultant said its a matter of time\", \"my dad has weeks left\". The reader above knew \"dying\",\n    \"terminal\" and \"hospice\" and answered all of them as small talk.\n\n    The who is what keeps the reading honest: a give-up is about somebody dying when a doctor, a\n    vet or a hospital says it, while the council, the mechanic and the bank say the same sentence\n    about a noise, a car and a loan and each has an entry that answers it.\n    ");
    if py_truthy(&Py::Bool(py_is(&words.clone(), &Py::None))) {
    words = words_of(&[tight.clone()]);
    }
    who = call_builtin("bool", &[mcall(&g_CARE_WHO(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOTHING_MORE_TO_DO(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TREATMENT_STOPPED(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_WORKING(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_SERIOUS_CARE(), "search", &[tight.clone()]) } } } } } } } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_NOTHING_MORE_TO_DO(), "search", &[tight.clone()]))); if py_truthy(&__b) { __b } else { _the_give_up_is_about_somebody(&[tight.clone(), words.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&_the_give_up_without_a_care_word(&[tight.clone(), who.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_MEDICATION_STOPPED(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_SERIOUS_CARE(), "search", &[tight.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&says_the_last_drug(&[tight.clone()])) {
    return Py::Bool(true);
    }
    somebody = { let __b = _has_person(&[words.clone()]); if py_truthy(&__b) { __b } else { { let __b = _has_pet(&[words.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("she"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("he"))); if py_truthy(&__b) { __b } else { call_builtin("bool", &[bitand(&g_HUMAN_WORDS(), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } } } } } };
    if py_truthy(&mcall(&g_FINAL_DAYS(), "search", &[tight.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&somebody.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&who.clone())) } }) {
    return _the_family_is_told(&[tight.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_DAYS_LEFT(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_THE_END(), "search", &[tight.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_COULD_BE_DAYS(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_THE_END(), "search", &[tight.clone()]))) } }) {
    return { let __b = Py::Bool(!py_truthy(&mcall(&g_DAYS_WAIT_FOR(), "search", &[tight.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = who.clone(); if py_truthy(&__b) { __b } else { { let __b = _has_person(&[words.clone()]); if py_truthy(&__b) { __b } else { _has_pet(&[words.clone()]) } } } } } };
    }
    if py_truthy(&mcall(&g_LAST_STAGES(), "search", &[tight.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_NOT_GETTING_BETTER(), "search", &[tight.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_GIVEN_UP_ON(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { who.clone() } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_PREPARE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = who.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PREPARE_OTHERWISE(), "search", &[tight.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_OFF_LIFE_SUPPORT(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_OFF_SUPPORT_WELL(), "search", &[tight.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_HOSPICE_CARE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_HOSPICE_JOB(), "search", &[tight.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_PALLIATIVE_ITSELF(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asks_anything(&[tight.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__WORKING_WITH_THE_DYING(), "search", &[tight.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_family_is_told(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut serious: Py = Py::None;
    let _ = py_str("True for the give-up that names nobody but carries the specialist's word.\n\n    \"theyve said its palliative now\", \"the consultant said the treatment isnt working anymore\",\n    \"the doctor said to prepare ourselves\": the person is in the chat, not in the line, and the\n    other readings answered these as small talk. Only the words that are a death in every context\n    are here - a specialist or an oncologist, palliative care, hospice - so \"my acne treatment isnt\n    working anymore\" and \"are you a consultant\" stay where they were.\n    ");
    serious = call_builtin("bool", &[mcall(&g_SERIOUS_CARE(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = serious.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_NOT_WORKING(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PREPARE(), "search", &[tight.clone()]) } } } }) {
    return Py::Bool(!py_truthy(&mcall(&g_PREPARE_OTHERWISE(), "search", &[tight.clone()])));
    }
    if py_truthy(&{ let __b = mcall(&g_PALLIATIVE_ITSELF(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asks_anything(&[tight.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__WORKING_WITH_THE_DYING(), "search", &[tight.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_COULD_BE_DAYS(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_TOLD_US(), "search", &[tight.clone()]) } }) {
    return Py::Bool(!py_truthy(&mcall(&g_DAYS_WAIT_FOR(), "search", &[tight.clone()])));
    }
    if py_truthy(&{ let __b = mcall(&g_OFF_LIFE_SUPPORT(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_OFF_SUPPORT_WELL(), "search", &[tight.clone()]))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_HOSPICE_CARE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_HOSPICE_JOB(), "search", &[tight.clone()]))) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_vet_said_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let _ = py_str("True when a vet is the one who said it, so the animal's own pool answers.");
    return { let __b = { let __b = Py::Bool(py_contains(&mcall(&tight.clone(), "split", &[]), &py_str("vet"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&tight.clone(), "split", &[]), &py_str("vets"))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_HUMAN_WORDS()))) } };
    Py::None
}
pub fn _the_give_up_is_about_somebody(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut third_person_only: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "third_person_only") { v.clone() }
    else { Py::Bool(false) };
    let mut last: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_words: Py = Py::None;
    let mut subject: Py = Py::None;
    let _ = py_str("True when the doctor's giving up is about somebody, else False.\n\n    \"The doctor said theres nothing more they can do\" is the sentence this whole cluster is about.\n    \"The doctor said theres nothing more they can do about my knee\" is an orthopaedic\n    appointment, and the same words sit in front of a back, a hip, a shoulder and a pain. Nothing\n    before the verb separates them, so what the giving up names decides it:\n\n    - no object at all (\"nothing more they can do\") is the whole of somebody's care;\n    - an object that is a person (\"for my mum\", \"about him\") or their illness (\"about the\n      cancer\", \"for the tumour\") is the same news;\n    - an object that is a body part or a thing (\"about my knee\", \"about the pain\") is not, and the\n      line goes back to the ordinary answers, each of which has one.\n\n    Only the text after the verb is read, so the specialist in front of it (\"the doctor said\",\n    \"the consultant said\") cannot stand in for the person the giving up is about.\n\n    `third_person_only` is for the reading that has no care word at all to carry it (see\n    `_the_give_up_without_a_care_word`): there, \"nothing more they can do for me\" is the speaker\n    talking about themselves, which the care words already cover, and only somebody else counts.\n    ");
    named = mcall(&g_THE_GIVE_UP_NAMES(), "search", &[tight.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::Bool(!py_truthy(&third_person_only.clone()));
    }
    subject = mcall(&named.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&mcall(&g_SERIOUS_CARE(), "search", &[subject.clone()])) {
    return Py::Bool(true);
    }
    named_words = words_of(&[subject.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named_words.clone()))) {
    return Py::Bool(!py_truthy(&third_person_only.clone()));
    }
    last = geti(&named_words.clone(), &neg(&Py::Int(1)));
    if py_truthy(&{ let __b = third_person_only.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("me"), py_str("us"), py_str("i"), py_str("mine"), py_str("ours")]), &last.clone())) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_contains(&g_PEOPLE_AFTER_A_GIVE_UP(), &last.clone())); if py_truthy(&__b) { __b } else { _has_person(&[py_list(vec![last.clone()])]) } };
    Py::None
}
pub fn _the_give_up_without_a_care_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let _ = py_str("True for the give-up that names a person but no doctor, else False.\n\n    \"they said there is nothing more they can do for her\" has the shape and the person but none of\n    the words `CARE_WHO` wants, so it stayed small talk while the same sentence with \"the doctor\"\n    in front of it was read. Somebody said it, and it is about a third person: that is the news.\n    \"My boss said theres nothing more he can do for me\" is the speaker's own job, which the care\n    words cover when it is care at all, and an institution saying it about a pupil or a tenant is\n    an institution's answer, so neither is here.\n    ");
    if py_truthy(&{ let __b = who.clone(); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOTHING_MORE_TO_DO(), "search", &[tight.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SAID_IT(), "search", &[tight.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NOT_A_CARE_GIVER(), "search", &[tight.clone()])) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tight.clone()); __args.push(Py::None); __kws.extend(vec![("third_person_only".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_give_up_is_about_somebody(args))(&__args) };
    Py::None
}
pub fn _the_bill_was_the_last_turn(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last answer was about the arrangements and what they cost.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&g__NOT_A_BILL_THREAD(), &mcall(&last.clone(), "get", &[py_str("module")])))) {
    return Py::Bool(false);
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("bool", &[{ let __b = mcall(&g__BILL_SUBJECT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__BILL_MONEY(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _loss_is_the_thread(args: &[Py]) -> Py {
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
    let mut record: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when a death has been talked about in the last few turns of this conversation.");
    record = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("loss")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    return le(&sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[{ let __b = mcall(&record.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e))), &g_LOSS_STILL_THE_THREAD());
    Py::None
}
pub fn arrangement_cost_pool(args: &[Py]) -> Py {
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
    let mut tight: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { Py::None };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("The cost of the arrangements, asked on its own: the pool to answer with, or None.\n\n    Read before `is_practical_ask`, which stands the composer aside for every other practical\n    question inside a bereavement: a price question with no death in it is not one of those, and\n    the modules it was standing aside for had nothing about a funeral to say. A comparison of the\n    two (\"is cremation cheaper than burial\") has its own pool, because the answer is which of them\n    costs less rather than where the money goes.\n    ");
    tight = if py_truthy(&Py::Bool(py_is(&tight.clone(), &Py::None))) { _tight(&[line.clone()]) } else { tight.clone() };
    if py_truthy(&{ let __b = mcall(&g_FUNERAL_COST_ELSE(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__DEATH_IN_LINE(), "search", &[tight.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__PRONOUN_MONEY_ASK(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { _the_bill_was_the_last_turn(&[memory.clone()]) } }) {
    if py_truthy(&{ let __b = mcall(&g_FUNERAL_COST_COMPARE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_HELP_ASK(), "search", &[tight.clone()]))) } }) {
    return g_FUNERAL_COST_COMPARE_REPLIES();
    }
    return g_FUNERAL_HELP_REPLIES();
    }
    if py_truthy(&_loss_is_the_thread(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_FUNERAL_COST(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FUNERAL_HELP(), "search", &[tight.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = asks_anything(&[line.clone(), tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUIETER_ASK(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_FUNERAL_COST_COMPARE(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HELP_ASK(), "search", &[tight.clone()]) } } } } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_FUNERAL_HELP(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__THE_ESTATE(), "search", &[tight.clone()]))) } }) {
    return g_FUNERAL_HELP_REPLIES();
    }
    if py_truthy(&{ let __b = mcall(&g_FUNERAL_COST_COMPARE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__BURIAL_SIDE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__CREMATION_SIDE(), "search", &[tight.clone()]) } } } }) {
    return g_FUNERAL_COST_COMPARE_REPLIES();
    }
    return Py::None;
    Py::None
}
pub fn says_the_last_drug(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let _ = py_str("True when the line says somebody has been started on the drug that ends the treatment.\n\n    \"they put my dad on morphine\", \"they started my mum on morphine\", \"she is on a syringe\n    driver now\": the cure has stopped, the comfort has started, and it is the sentence a family\n    hears in the room. The person's own painkiller is not that news (\"im on morphine for my\n    back\", \"i was put on morphine after my op\"), so the first person switches it off.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ON_THE_LAST_DRUG(), "search", &[tight.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_SELF_ON_THE_DRUG(), "search", &[tight.clone()])));
    Py::None
}
pub fn asks_about_the_remains(args: &[Py]) -> Py {
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
    let mut tight: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { Py::None };
    let _ = py_str("True when the pets entry is the answer to what the line asks.\n\n    The entry knows one thing: what happens to the body, and that the vet can arrange it, so it\n    keeps the turn only for a line that asks about the body - \"can we bury him at home\", \"what do\n    i do with his bed\". A line that asks something else is a person the card has no answer for:\n    \"my dog died, when should i get another one\" got the cremation paragraph, and \"our hamster\n    died, is it my fault\" got a paw print. The condolence answers those.\n\n    A line that asks nothing at all is not this reading's business either: `bot/modules/loss.py`\n    asks `asks_for_a_decision` first, so a bare \"my dog died\" is met rather than carded, and this\n    only decides which asks the card answers. It reads the ask itself (`ASK_BARE` is the shape\n    with no wh-word and no modal, \"is it my fault\"), so it is not standing on that call.\n    ");
    line = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    tight = if py_truthy(&Py::Bool(py_is(&tight.clone(), &Py::None))) { _tight(&[line.clone()]) } else { tight.clone() };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PET_ARRANGEMENT(), "search", &[tight.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = asks_anything(&[line.clone(), tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASK_WE(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASK_TAIL(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASK_BARE(), "search", &[tight.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_pet_died(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the death in the line is a pet's, so the pets entry is the answer to it.\n\n    \"we had to put our cat down yesterday\" and \"my dog died last night\" are that entry's lines.\n    \"my aunt died and her dog needs a home\" is a person's death with a live dog in the next\n    clause, and the cremation card is not the answer to that. Read clause by clause, and reuse\n    this file's reading of a death so \"dead birds\" and \"put my cat to sleep at night\" stay out.\n\n    A death still to come is not a death (`NOT_YET`): the dog is alive on the day before.\n    ");
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    if py_truthy(&{ let __b = mcall(&g_NOT_YET(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PUTTING_REGRETTED(), "search", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_CLAUSE(), "split", &[text.clone()]));
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_OWN_PET(), "search", &[clause.clone()])))) {
    continue;
    }
    words = _plain(&[clause.clone()]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { reads_as_loss(&[clause.clone(), words.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn asks_the_administration(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line asks about the paperwork or the money a death leaves behind.\n\n    Read on the words of the administration themselves, not on the shape of the ask: \"is the\n    cremation expensive\" and \"my dad died last week how long does probate take\" are the funeral\n    and the probate entries' lines, while \"is it normal to still cry\" is about the person and\n    belongs to the condolence.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    return call_builtin("bool", &[mcall(&g_ADMINISTRATION(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn administration_keys(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The pieces of paperwork a line or a reply names, as plain keys.");
    lowered = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    return call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ADMINISTRATION_KEYS()) { let __unpk3842 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk3842[0].clone(); let mut pattern: Py = __unpk3842[1].clone(); if py_truthy(&stdlib_call("re", "search", &[pattern.clone(), lowered.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(key.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn is_about_the_paperwork(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut text: Py = Py::None;
    let _ = py_str("Whether the reply answers the paperwork the line named, not merely the right shape.\n\n    Three things have to hold, and each of them was a real turn: the reply has to name the same\n    piece of paperwork the line named (\"what do i bring to the funeral\" was answered with the\n    wake food list, which names none), it has to say whose paperwork it is (\"i have to sort out\n    her pension\" was answered with a card about paying into your own), and it must not be a\n    definition of the word (\"what do i do about her will\" got what a will is).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&administration_keys(&[line.clone()]), &administration_keys(&[reply.clone()]))))) {
    return Py::Bool(false);
    }
    text = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_DEFINES(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THE_DEATH(), "search", &[text.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_thing_of_its_own_to_get_through(args: &[Py]) -> Py {
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
    let _ = py_str("True when the \"get through\" ask is about something that has an answer of its own.\n\n    The pool belongs to the loss: a pronoun, a bare day or week, a date, a ceremony. Anything\n    else in the object names a thing with its own subject - night shifts, the rota, a driving\n    test, the exam period, the paperwork - and the entry that knows that thing is the answer.\n    ");
    found = mcall(&g_GET_THROUGH_OBJECT(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&words_of(&[mcall(&found.clone(), "group", &[Py::Int(1)])]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GET_THROUGH_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GET_THROUGH_THE_LOSS(), &word.clone())) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn asks_for_a_decision(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line asks what to do now, not how the person feels.\n\n    \"my dog died\" and \"we had to put our cat down yesterday\" say the thing and ask for nothing:\n    the card's \"nothing has to be decided today, the vet can arrange cremation\" is colder than\n    a sympathy on that day. \"my rabbit died, do we need to bury it deep\" and \"what do i do with\n    her bed\" do ask, and the pets entry is the only thing that knows.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    return call_builtin("bool", &[mcall(&g_PRACTICAL_ASK(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn feeling_about_the_body(args: &[Py]) -> Py {
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
    let _ = py_str("True when a feeling word in the line is about the body, not about the person.");
    return call_builtin("bool", &[mcall(&g_BODY_FEELING(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn names_a_feeling(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line talks about the person's own feelings, so sympathy beats the card.\n\n    \"my cat died and i keep crying\" is not a question about cremation; \"we had to put our cat\n    down yesterday\" is, and the pets entry earns that turn. The loss module reads this before it\n    lets another candidate keep the turn (`DEFERS_TO`).\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[{ let __b = mcall(&g_OWN_STATE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ABSENCE(), "search", &[text.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn acknowledges(args: &[Py]) -> Py {
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
    let _ = py_str("True when a reply says the sad thing in its own words, so an entry that does can speak.\n\n    Public because the loss module hands it to the arbiter as `DEFERS_TO`: a topic entry that\n    already acknowledges the death (the pets entry, with cremation and a paw print) is a better\n    answer than a generic sympathy, so the arbiter lets it keep the turn.\n    ");
    return call_builtin("bool", &[mcall(&g_ACKNOWLEDGE(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _when_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let _ = py_str("True for a line asking when the funeral is, rather than whether to go.");
    return call_builtin("bool", &[mcall(&g_WHEN_ASK(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_part_of_the_day_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let _ = py_str("True when the line asks about a part of the service, not whether to be at it.\n\n    \"what song should i play at my dads funeral\", \"what reading should i do at mums funeral\",\n    \"should we have flowers at the funeral\" and \"should i take my kids to my mums funeral\" all\n    reached the funeral card (\"If you can make it, go\"), which is about turning up. The music,\n    the readings, the flowers, the length and whether the children come are each a knowledge\n    row, and the person asking is often the one arranging the day. A line asking what to *say*\n    stays here: the words for a funeral are this file's.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_PART_OF_THE_DAY(), "search", &[tight.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_SAY_THE_WORDS(), "search", &[tight.clone()])));
    Py::None
}
pub fn _asks_about_the_feeling(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let _ = py_str("Whether the question is about the feeling rather than about the death or the day.");
    return call_builtin("bool", &[mcall(&g_ASKS_ABOUT_THE_FEELING(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _feeling(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The feeling the line names, if it names one of ours, else \"\".");
    let __iter1 = py_iter_or_raise(&mcall(&tight.clone(), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FEELING_WORDS(), &word.clone()))) {
    return geti(&g_FEELING_WORDS(), &word.clone());
    }
    }
    return py_str("");
    Py::None
}
pub fn _how_long_since(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut rest: Py = Py::None;
    let mut span: Py = Py::None;
    let _ = py_str("True when the line says only how long it has been, inside a bereavement chat.\n\n    \"its been a year\", \"six months now\", \"nearly two years\". The span has to be there, the line\n    has to be short, and it must not name anything else with a length in it - a job, a course, a\n    habit - because those are their own subjects even three lines after a death.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&tight.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&tight.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_ASK_FORM(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&tight.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_SPAN_ELSEWHERE(), "search", &[tight.clone()])) {
    return Py::Bool(false);
    }
    span = mcall(&g_A_SPAN(), "search", &[tight.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&span.clone()))) {
    span = mcall(&g_A_SPAN_NO_NUMBER(), "search", &[tight.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&span.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&add(&add(&slice_or_raise(&tight.clone(), &Py::None, &mcall(&span.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&tight.clone(), &mcall(&span.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[]);
    if py_truthy(&gt(&add(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &Py::Int(6))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_SPAN_LEAD(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_SPAN_TAIL(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { le(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _missing_someone(args: &[Py]) -> Py {
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
    let mut animal: Py = Py::None;
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the line says they miss a person or a pet, without saying that anyone died.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_MISS_VERB(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_ASK_AT_START(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    noun = mcall(&g_MISS_NOUN(), "search", &[text.clone()]);
    if py_truthy(&{ let __b = noun.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_MISS_NOT_BETWEEN(), "search", &[mcall(&noun.clone(), "group", &[Py::Int(0)])]) } }) {
    noun = Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = noun.clone(); if py_truthy(&__b) { __b } else { mcall(&g_MISS_PRONOUN(), "search", &[text.clone()]) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_MISS_NOT(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    found = { let __b = noun.clone(); if py_truthy(&__b) { __b } else { mcall(&g_MISS_PRONOUN(), "search", &[text.clone()]) } };
    if py_truthy(&{ let __b = noun.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MISS_TAIL(), "match", &[slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]))) } }) {
    rest = mcall(&slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    animal = Py::Bool(py_contains(&g_MISS_ANIMALS(), &mcall(&geti(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "split", &[]), &neg(&Py::Int(1))), "lower", &[])));
    named = stdlib_call("re", "match", &[py_str("([A-Za-z]+)([.!?]?)(.*)$"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = animal.clone(); if !py_truthy(&__b) { __b } else { { let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_MISS_ANIMAL_NOUNS(), &mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "lower", &[]))); if !py_truthy(&__b) { __b } else { mcall(&g_MISS_TAIL(), "match", &[mcall(&named.clone(), "group", &[Py::Int(3)])]) } } } } } }))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_question_about_the_feeling(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let _ = py_str("True when the line asks about a feeling itself and names nothing else to be about.\n\n    Main has a reader of its own called `_asks_about_the_feeling(tight)` a few hundred lines up,\n    and its call site passes one argument: a second function of the same name here does not\n    replace that call, it breaks it - the later definition wins and the call raises a TypeError\n    that the module's own `except` swallows, so the loss stands down for every line that reaches\n    it. Hence the different name.\n\n    The person can be named and it is still the feeling: \"i feel guilty about her\" asks about the\n    guilt, and the answer to it is the same one the line gets standing alone. What ends it is a\n    deed or a thing in the line - \"i feel guilty about shouting at her\" is about the shouting.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_FEELING_ASK(), "search", &[line.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &g_FEELING_STATES())))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&sub(&sub(&sub(&words.clone(), &g_FEELING_STATES()), &g_FEELING_ONLY_WORDS()), &g_FEELING_PERSON_WORDS())));
    Py::None
}
pub fn _an_ask_in_the_line(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line puts a question, whatever its punctuation.\n\n    `QUESTION` is the grief shapes (\"is it normal\", \"why do i\"); this is the ordinary ask that no\n    pool here answers: \"should i still set a place for him\", \"do i keep the bowls out\".\n    ");
    if py_truthy(&mcall(&g_ASK_AT_START(), "match", &[line.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:should|shall|do|does|did|can|could|would|will|is|are|was|were|why|how|what|when|where|who)\\s+(?:i|we|you|he|she|they|it)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _still_here(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut pattern: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("The pool for the habit a line reports, or None.\n\n    It is asked only once a death is read in the line, because the habit alone is not a death:\n    \"i still see my dad every sunday\" and \"the office feels so empty without my team\" are\n    somebody alive and away, and the loss pool says the person is gone. `ABSENCE` stays where it\n    is for the same reason - inside a thread that already has the death. What this adds is the\n    line that carries both: \"my dad died in march and i keep reaching for the phone to ring\n    him\" is a death said again with the part they actually want to talk about, and the condolence\n    is the stock answer STYLE.md rules out.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g_QUESTION(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&_an_ask_in_the_line(&[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_MISS_NOT(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { _a_thing_of_theirs(&[words.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&_only_the_proceedings(&[mcall(&py_str(" "), "join", &[words.clone()]), words.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_HABIT_PATTERNS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    pool = __unpk2[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[line.clone()])) {
    return pool.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_habit_in(args: &[Py]) -> Py {
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
    let mut _pool: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("True when the line reports one of the habits the pools above answer, whatever the pool.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HABIT_PATTERNS()) { let __unpk9301 = py_unpack(&__item.clone(), 2, None); let mut pattern: Py = __unpk9301[0].clone(); let mut _pool: Py = __unpk9301[1].clone(); __out.push(mcall(&pattern.clone(), "search", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _telling_a_child(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut between: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("True when the line asks how to explain a death to a child of the user's.\n\n    \"i have to tell my boss my dad died\" is not this: the person being told is not a child.\n    Neither is \"how do i tell people about my son\", where the child is who died rather than who\n    has to be told.\n    ");
    found = mcall(&g_TELL_CHILD(), "search", &[tight.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return call_builtin("bool", &[{ let __b = mcall(&g_YEAR_OLD(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:tell|explain)\\w*\\b"), tight.clone()]).unwrap_or_else(|e| exc::raise(e)) } }]).unwrap_or_else(|e| exc::raise(e));
    }
    between = { let __b = mcall(&found.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:about|of|people|everyone|office|work|boss)\\b"), between.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn marks_a_day(args: &[Py]) -> Py {
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
    let mut known: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { Py::Bool(false) };
    let mut day_named: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut tight: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is about a day that matters and someone who is not there for it.\n\n    Strong evidence, enough on its own: the age someone would have been (\"my mum would have been\n    70 this week\", \"he would have been 60 next month\"), and an occasion with the missing person\n    in it (\"the first christmas without my dad\"). An age in the conditional is only a person if\n    the line names one, so \"the cake would have been 20 minutes\" is left alone.\n    Weak evidence, only enough once the chat is already about a loss (`known`): the tradition\n    (\"we always did a lunch for her birthday, what do i do instead\") and the question about\n    keeping it going (\"should i still do a cake\"). A line on its own that says only \"my sisters\n    birthday is coming up and i dont know what to do\" is a question about a present, and must not\n    be read as a bereavement.\n    ");
    if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    raw = mcall(&py_str(" "), "join", &[_plain(&[text.clone()])]);
    words = mcall(&raw.clone(), "split", &[]);
    } else {
    words = call_builtin("list", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    raw = mcall(&py_str(" "), "join", &[words.clone()]);
    }
    tight = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FILLERS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    subject = { let __b = _has_person(&[words.clone()]); if py_truthy(&__b) { __b } else { call_builtin("bool", &[bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PRONOUN())]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&mcall(&g_WOULD_HAVE(), "search", &[tight.clone()])) {
    return subject.clone();
    }
    day_named = call_builtin("bool", &[{ let __b = mcall(&g_DAY_THING(), "search", &[raw.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_OCCASION(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { _has_person(&[words.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = known.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_TRADITION(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { day_named.clone() } }; if py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_STILL_KEEP(), "search", &[raw.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_DAY_THING(), "search", &[raw.clone()]) } }; if py_truthy(&__b) { __b } else { { let __b = mcall(&g_DAY_QUESTION(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { day_named.clone() } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_OCCASION(), "search", &[tight.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_WITHOUT(), "search", &[tight.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn asks_what_to_do(args: &[Py]) -> Py {
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
    let mut raw: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line about the day also asks what to do about it.\n\n    \"we always did a lunch for her birthday, i don't know what to do instead\" asks; \"my mum would\n    have been 70 this week\" does not, however much the day is on their mind.\n    ");
    if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    raw = mcall(&py_str(" "), "join", &[_plain(&[text.clone()])]);
    words = mcall(&raw.clone(), "split", &[]);
    } else {
    words = call_builtin("list", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    raw = mcall(&py_str(" "), "join", &[words.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("?")))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_ASKS_ABOUT_THE_DAY(), "search", &[raw.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _plain(args: &[Py]) -> Py {
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
    let _ = py_str("The line as words: lower case, apostrophes and punctuation flattened.");
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&text.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?;:\"'()")])); } py_list(__out) })() };
    Py::None
}
pub fn _about_them(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is about the person talking, not about the pub or the boss.");
    text = mcall(&{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&mcall(&text.clone(), "startswith", &[g_SELF_OPENERS()])) {
    return Py::Bool(true);
    }
    words = mcall(&text.clone(), "split", &[]);
    return { let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("my"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SELF_NOUNS(), &mcall(&geti(&words.clone(), &Py::Int(1)), "strip", &[py_str(",.!?")]))) } } } };
    Py::None
}
pub fn _about_family(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names a family member as its subject: \"my mum isnt coping\".");
    return { let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("his"), py_str("her"), py_str("their"), py_str("our")]), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &Py::Int(3), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_FAMILY(), &mcall(&word.clone(), "strip", &[py_str(",.!?")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _own_state(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person's own state: \"can't sleep\", \"so tired\", \"the mornings are hard\" - but not\n    \"i lost my keys this morning\".");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GRIEF_SELF(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WHEN_WORDS(), &geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))))) } })) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _state_describes_them(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a state word in the line describes the person, not a thing they named.\n\n    \"i feel so bad\", \"it was really hard\", \"the mornings are hard\" are the person's own state.\n    \"i have a bad cough\", \"my boss is hard work\" are a subject with an answer of its own: the\n    state word there only describes that thing, so the line is not the bereavement.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GRIEF_SELF(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = ge(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STATE_TAIL(), &geti(&words.clone(), &add(&index.clone(), &Py::Int(1))))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_body_is_theirs(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks what the loss is doing to the person's own body or appetite.");
    if py_truthy(&mcall(&g_QUANTITY(), "search", &[tight.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NORMAL_TO_FEEL(), "match", &[tight.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_THEMSELVES(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASK_NORMAL(), "match", &[tight.clone()]) } }))) {
    return Py::Bool(false);
    }
    words = call_builtin("tuple", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BODY_STATE(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = ge(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STATE_TAIL(), &geti(&words.clone(), &add(&index.clone(), &Py::Int(1))))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_body_with_its_own_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the line reports the person's own body in a way that has a practical answer.\n\n    \"i cant sleep\", \"i keep waking at 3\", \"i have no appetite\", \"i cant face food\": sleep and\n    food have entries, and a turn into a bereavement they are the better answer. \"how much water\n    should i drink\" is a quantity asked about water, so it does not count.\n    ");
    if py_truthy(&mcall(&g_QUANTITY(), "search", &[tight.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THEMSELVES(), "search", &[tight.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_BODY_ANSWER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_a_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut topics: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topics") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topics"))) };
    let _ = py_str("True when the reader gave the line a subject of its own to be answered.\n\n    `_own_subject` with the measure topics taken out: \"how long does this go on for\" is read as\n    maths and numbers, and a length asked about the loss is still the loss.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[topics.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&topics.clone())) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[sub(&sub(&call_builtin("set", &[topics.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_THREAD_TOPICS()), &g_MEASURE_TOPICS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _own_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut topics: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topics") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topics"))) };
    let _ = py_str("True when the reader gave the line a subject that has an answer of its own.\n\n    A cough, a bad back, a broken boiler, a lost job: the line names something the bot knows\n    about, and a bereavement chat does not stop it being that. Only the topics that are the\n    bereavement itself, or fillers, leave the thread where it is.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[topics.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&topics.clone())) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[sub(&call_builtin("set", &[topics.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_THREAD_TOPICS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _hands_over_a_detail(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let _ = py_str("Whether the line adds something new about the person rather than the missing said again.\n\n    \"i miss my nan\" then \"she taught me to bake\" is a friend's cue: the detail is the thing to\n    answer, and the store's own line - \"She taught you to bake. That is a lot to carry on top of\n    how you feel\" - beats the company pool, which would say the same warm thing to any line at\n    all. \"i miss my mum\" then \"i keep expecting her to call\" is the missing again with nothing\n    new in it, and that one is the pool's. \"she died two years ago\" opens on her as well, and the\n    death modules have their own answer for it, so the thread stands aside for that too.\n    ");
    return call_builtin("bool", &[{ let __b = mcall(&g_ABOUT_THEM_OPENS(), "match", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_MISSING_SAID_AGAIN(), "search", &[tight.clone()]))) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _thread_pool(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The pool behind the bot's last line, when that line was the what-to-say advice.\n\n    \"what do i say to my friend\" then \"its a workmate, i barely know him\" is the same question\n    about what to say, not the person's own bereavement, so the advice continues.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    last = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_SAY_REPLIES(), &last.clone()))) {
    return g_SAY_REPLIES();
    }
    if py_truthy(&Py::Bool(py_contains(&g_MISS_REPLIES(), &last.clone()))) {
    return g_MISS_REPLIES();
    }
    return Py::None;
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut spell: Py = Py::None;
    let _ = py_str("The line with the contractions spelled out, so \"my mum's dying\" matches \"my mum is dying\".\n\n    People write the contraction and often drop the apostrophe - \"my mum's dying\", \"my dads\n    dying\", \"my nan's in a hospice\" - and both readers wanted the copula spelled out, so those\n    lines were answered as if nobody were dying.\n    ");
pub fn __nested_spell_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut spell: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut base: Py = Py::None;
    base = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__CAN_BE_DYING(), &base.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NOT_A_CONTRACTION(), &add(&base.clone(), &py_str("s")))) } }) {
    return add(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &py_str(" is "));
    }
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    Py::None
}
    spell = py_func("spell", move |args: &[Py]| { let mut full = vec![spell.clone(), text.clone()]; full.extend_from_slice(args); __nested_spell_0(&full) });
    return mcall(&g__CONTRACTED(), "sub", &[spell.clone(), text.clone()]);
    Py::None
}
pub fn _tight(args: &[Py]) -> Py {
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
    let _ = py_str("The line without filler words, for phrase matching: \"what do I even say\" is \"what do\n    I say\".");
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_plain(&[text.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FILLERS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    Py::None
}
pub fn _their_thing_after(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut rest: Py = Py::None;
    let _ = py_str("True when the word after the person is one of their things rather than a place or a time.\n\n    \"i lost my carer badge\", \"i lost my nurse notes\", \"i lost my mums keys\": the person is the\n    qualifier and the thing after them is what went missing.\n    ");
    rest = mcall(&slice_or_raise(&clean.clone(), &start.clone(), &Py::None, &Py::None), "split", &[]);
    return { let __b = call_builtin("bool", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&bitor(&g_OBJECTS(), &g_THEIRS()), &geti(&rest.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _another_person(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let _ = py_str("True when the line names a person who is not the role word a thing has borrowed.\n\n    \"the drain cleaner died\" is a blocked pipe; \"my mum thinks the drain cleaner died\" is still\n    about the pipe, but \"my mum died and the drain cleaner is in the cupboard\" has a person in it\n    and keeps the death reading.\n    ");
    if py_truthy(&bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &sub(&g_HUMAN_WORDS(), &g_TWO_WAY_ROLES()))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_ROLE_PHRASE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_thing_of_theirs(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut other: Py = Py::None;
    let mut others: Py = Py::None;
    let mut place: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a person word is only the qualifier of one of their things.\n\n    \"my carer badge\", \"my nurse notes\", \"my carer allowance letter\": the thing after the person\n    is what the line is about, and the person is whose it is. Read as a person, a badge that has\n    stopped got a condolence.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = ge(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&bitor(&g_OBJECTS(), &g_THEIRS()), &geti(&words.clone(), &add(&index.clone(), &Py::Int(1))))) } }) {
    continue;
    }
    stem = if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("'s")])) { slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None) } else { if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("s")])) { slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None) } else { word.clone() } };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_PEOPLE(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PEOPLE(), &stem.clone())) } }) {
    continue;
    }
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk7169 = py_unpack(&__item.clone(), 2, None); let mut place: Py = __unpk7169[0].clone(); let mut other: Py = __unpk7169[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_eq(&place.clone(), &index.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PEOPLE(), &other.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&other.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PEOPLE(), &slice_or_raise(&other.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } } } } } } } }) {  __out.push(other.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&others.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_role_phrase(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line names a person a death can be said about with two words.\n\n    \"my home help died\" and \"my night carer died\" name nobody word by word: \"help\" is the commonest\n    verb in the language and \"night\" is a time of day. The whole phrase is the person, so it is read\n    as one.\n    ");
    return call_builtin("bool", &[mcall(&g_ROLE_PHRASE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _has_person(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut index: Py = Py::None;
    let mut second: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a person or a pet is named, in any form people type it.\n\n    \"my mum's birthday\", \"my mums anniversary\" and \"my sisters wedding\" are the same person as\n    \"my mum\", \"my sister\": a possessive and a plural are how the line says whose day it is, and\n    without them the day went unread.\n    ");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_PEOPLE(), &word.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("'s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PEOPLE(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PEOPLE(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } }) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&g_PEOPLE_PHRASES());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    first = __unpk3[0].clone();
    second = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    index = __unpk5[0].clone();
    word = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&words.clone(), &add(&index.clone(), &Py::Int(1))), &second.clone())) } }) {
    return Py::Bool(true);
    }
    }
    }
    return _a_role_phrase(&[mcall(&py_str(" "), "join", &[words.clone()])]);
    Py::None
}
pub fn _object_died(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the thing before \"died\"/\"dead\" is an object, not a person.");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("died"), py_str("dies"), py_str("dead"), py_str("dying")]), &word.clone())); if !py_truthy(&__b) { __b } else { index.clone() } }) {
    if py_truthy(&Py::Bool(py_contains(&g_OBJECTS(), &geti(&words.clone(), &sub(&index.clone(), &Py::Int(1)))))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = gt(&index.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OBJECTS(), &geti(&words.clone(), &sub(&index.clone(), &Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("is"), py_str("was"), py_str("has"), py_str("had")]), &geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))))) } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _where_i_put_it(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the line asks where a thing was put, people and errands aside.");
    let __iter1 = py_iter_or_raise(&mcall(&g_WHERE_I_PUT_IT(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    rest = mcall(&slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "split", &[]);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_MISLAID(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING_NEXT(), &geti(&rest.clone(), &Py::Int(1)))) } }) {
    continue;
    }
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn locked_out(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when they are shut out of their home or car: \"im locked out, i lost my keys\".\n\n    Kept apart from `lost_thing_to_find` because the answer is different in kind: a search is\n    no use to someone standing in the street, and the way back in is the urgent half. A digital\n    account (\"locked out of my email\") and the cat being shut out are not this.\n    ");
    said = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_LOCKED_OUT(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_NOT_A_DOOR(), "search", &[said.clone()])));
    Py::None
}
pub fn locked_out_of_the_car(args: &[Py]) -> Py {
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
    let _ = py_str("True when the thing they are shut out of is the car, which has its own way in.");
    return call_builtin("bool", &[mcall(&g_CAR_LOCK_OUT(), "search", &[mcall(&py_str(" "), "join", &[words_of(&[text.clone()])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _readable(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let _ = py_str("The line with the engine's repairs in it: \"wheres my keys\" reads as \"where is my keys\".\n\n    The patterns below match a line as it was typed, and people type \"wheres\", \"weres\",\n    \"whers\" and \"where r\" far more often than \"where's\". The understanding engine already knows\n    every one of those spellings, so the repairs are taken from it rather than listed again here:\n    a table of misspellings in a composer is a second copy of the engine's, and it lags it.\n    ");
    let __try1 = exc::catch(|| {
    read = py_call(&crate::gen::understanding::g_normalise(), &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
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
    return call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return { let __b = mcall(&py_str(" "), "join", &[{ let __b = mcall(&read.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]); if py_truthy(&__b) { __b } else { call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn lost_thing_to_find(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line is something mislaid and findable: \"i lost my keys\", \"ive lost my bag\".\n\n    Kept separate from `lost_a_thing`, which also reads a job, a match and a train of thought:\n    those are disappointments and idioms, and the answer to them is not \"retrace your steps\".\n    ");
    said = mcall(&py_str(" "), "join", &[words_of(&[_readable(&[text.clone()])])]);
    return call_builtin("bool", &[{ let __b = mcall(&g_LOST_THING_TO_FIND(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CANT_FIND_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHERE_IS_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = _where_i_put_it(&[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_WHERE_IS_THING_BARE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_LEFT_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_GONE_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_KEEP_LOSING_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REMEMBER_NOT_WHERE(), "search", &[said.clone()]) } } } } } } } } } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn keeps_losing_thing(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the same thing goes missing again and again: \"i keep losing my keys\".\n\n    The answer to that is a habit rather than a search, so it gets its own pool.\n    ");
    said = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    return call_builtin("bool", &[{ let __b = mcall(&g_KEEP_LOSING_THING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_KEPT_GOING_MISSING(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn found_what_was_lost(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says a thing that was being looked for has turned up.\n\n    Only worth a reply of its own when the last line was that search, so the caller checks\n    `lost_thing_to_find(`the previous line`)` as well.\n    ");
    return call_builtin("bool", &[mcall(&g_FOUND_WHAT_WAS_LOST(), "search", &[mcall(&py_str(" "), "join", &[words_of(&[text.clone()])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn lost_in_a_place(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut found: Py = Py::None;
    let mut group: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut place: Py = Py::None;
    let _ = py_str("True when a person was lost in an everyday place: \"i lost my son at the park\".\n\n    It reads the person the line lost, then looks for a place after it. Kept public because the\n    modules that answer a lost child or a lost parent need the same reading, and because it is\n    the guard that keeps the sympathy away from a panic.\n    ");
    clean = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    let __iter1 = py_iter_or_raise(&g_LOST_FORMS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    group = __unpk2[1].clone();
    found = mcall(&pattern.clone(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DEATHABLE(), &mcall(&found.clone(), "group", &[group.clone()]))) } }) {
    continue;
    }
    place = mcall(&g_LOST_IN_A_PLACE(), "search", &[clean.clone(), mcall(&found.clone(), "end", &[])]);
    return { let __b = call_builtin("bool", &[place.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACES(), &mcall(&place.clone(), "group", &[Py::Int(1)]))) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _shed_after(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let _ = py_str("True when what follows the relation is the weight rather than the person.");
    return call_builtin("bool", &[mcall(&g_SHED(), "match", &[clean.clone(), start.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _feared_not_done(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut frame: Py = Py::None;
    let _ = py_str("True when the verb a match begins at sits behind a frame that keeps it in the future.\n\n    The frame ends on the verb itself, so a match that starts before the frame's end is the verb\n    the frame is about: \"i dont want to lose my mum\" matches at \"lose\", four letters before the\n    frame ends.\n    ");
    let __iter1 = py_iter_or_raise(&mcall(&g_FEARED_NOT_YET(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    frame = __item.clone();
    if py_truthy(&{ let __b = le(&mcall(&frame.clone(), "start", &[]), &start.clone()); if !py_truthy(&__b) { __b } else { if py_truthy(&le(&Py::Int(4), &{ let __cmp20 = sub(&mcall(&frame.clone(), "end", &[]), &start.clone()); __cmp20.clone() })) { le(&__cmp20.clone(), &Py::Int(6)) } else { Py::Bool(false) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _lost_in_a_place_after(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut place: Py = Py::None;
    let _ = py_str("True when an everyday place follows the person at *start*.\n\n    \"losing my son at the park\" is a mislaid child, not a bereavement, and the -ing form and the\n    noun form have to use the same list of places as \"i lost my son at the park\".\n    ");
    place = mcall(&g_LOST_IN_A_PLACE(), "search", &[clean.clone(), start.clone()]);
    return { let __b = call_builtin("bool", &[place.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACES(), &mcall(&place.clone(), "group", &[Py::Int(1)]))) } };
    Py::None
}
pub fn _can_die(args: &[Py]) -> Py {
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
    let _ = py_str("What can die in this line. A mouse in a line about a computer cannot: \"i lost my mouse\"\n    is a bad afternoon with a mouse mat, not a bereavement, and it wants the search for a lost\n    thing rather than sympathy. \"my pet mouse died\" keeps the animal reading.\n    ");
    if py_truthy(&crate::gen::understanding::topics::is_a_machine_word(&[clean.clone()])) {
    return sub(&g_DEATHABLE(), &g_MACHINE_ANIMALS());
    }
    return g_DEATHABLE();
    Py::None
}
pub fn lost_a_thing(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut deathable: Py = Py::None;
    let mut found: Py = Py::None;
    let mut group: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line says something was lost and it is not a person or a pet.\n\n    A follow-up to sympathy is read as still part of the loss, and a negative word is often all\n    that is in the line (\"it was really hard\"). \"i lost my keys\", \"i lost my phone\" and \"i lost\n    the plot a bit today\" are also negative and are not the loss: they belong to the modules\n    that know about keys and phones, or to no module at all.\n    ");
    words = words_of(&[text.clone()]);
    clean = mcall(&py_str(" "), "join", &[words.clone()]);
    deathable = _can_die(&[clean.clone()]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\blost\\s+it\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&g_LOST_FORMS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    group = __unpk2[1].clone();
    found = mcall(&pattern.clone(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    thing = mcall(&found.clone(), "group", &[group.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&deathable.clone(), &thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LOST_SELF(), &thing.clone())) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:lost|losing|lose)\\s+the\\s+([a-z]+)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
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
    let _ = py_str("The line's words, lower case and without punctuation: the reading both functions use.");
    return _plain(&[text.clone()]);
    Py::None
}
pub fn _brought_in_prey(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let _ = py_str("True when a living pet has brought something dead in, which is not a loss.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _has_pet(&[words.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PREY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_HUNTING(), "search", &[clean.clone()]) } } } }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&mcall(&g_PET_DIED(), "search", &[mcall(&g_PREY(), "sub", &[py_str(" "), clean.clone()])])));
    Py::None
}
pub fn _a_pronoun_is_what_died(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when \"it\" is what died and the line names a thing that could be the \"it\".\n\n    \"i bought a phone for my daughter and it died\" is a fault; \"my dad died last week\" is not,\n    and neither is a family noun with a pronoun that names no thing (\"my nan ... then it died\"\n    about a car still names the car, so the thing decides, not the pronoun).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_IT_DIED(), "search", &[clean.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_THINGS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_thing_stopped(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut place: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the death word in the line is about a thing, not a person or a pet.\n\n    A phone, a laptop, a boiler and a car all \"die\", and \"dead\" is also an intensifier: \"my\n    trainers are dead comfy\", \"the pub was dead quiet\", \"im dead tired\". Read on its own the\n    word says a bereavement, which is why the composer's doubt floor (\"is that normal\" after \"my\n    laptop battery dies fast\") has to be able to ask this before it reaches for the condolence.\n    Only the death words are read here, so a line that says a person died stays a death however\n    much of the rest of it is about a thing: \"my nan died and the boiler went the same week\" is\n    still their nan.\n    ");
    if py_truthy(&Py::Bool(py_is(&words.clone(), &Py::None))) {
    words = words_of(&[text.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_object_died(&[words.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk1435 = py_unpack(&__item.clone(), 2, None); let mut place: Py = __unpk1435[0].clone(); let mut word: Py = __unpk1435[1].clone(); __out.push({ let __b = Py::Bool(py_eq(&word.clone(), &py_str("dead"))); if !py_truthy(&__b) { __b } else { _dead_is_not_a_death(&[words.clone(), place.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_loss(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut a_pet: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a_pet") { v.clone() }
    else { Py::Bool(false) };
    let mut a_pronoun: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut deathable: Py = Py::None;
    let mut found: Py = Py::None;
    let mut group: Py = Py::None;
    let mut index: Py = Py::None;
    let mut losing: Py = Py::None;
    let mut lost_the: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut person: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about a death or a loss of a person or a pet.\n\n    `a_pet` says the line names an animal the store already holds (\"rex died\"): a name is not in\n    any of the word lists, so without it a death said by name alone is read as small talk.\n    ");
    if py_truthy(&Py::Bool(py_is(&words.clone(), &Py::None))) {
    words = words_of(&[text.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    deathable = _can_die(&[mcall(&py_str(" "), "join", &[words.clone()])]);
    clean = mcall(&py_str(" "), "join", &[words.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LAUGHING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&lost_in_a_place(&[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_WOULD_HAVE_BEEN(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _has_person(&[words.clone()]); if py_truthy(&__b) { __b } else { _unnamed_person(&[words.clone(), clean.clone()]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_brought_in_prey(&[clean.clone(), words.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_thing_of_theirs(&[words.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_pronoun_is_what_died(&[words.clone(), clean.clone()])) {
    return Py::Bool(false);
    }
    person = { let __b = _has_person(&[words.clone()]); if py_truthy(&__b) { __b } else { _unnamed_person(&[words.clone(), clean.clone()]) } };
    if py_truthy(&_the_end_is_near(&[clean.clone(), words.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_only_the_proceedings(&[clean.clone(), words.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_THING_ROLE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_another_person(&[words.clone(), clean.clone()]))) } } } }) {
    person = Py::Bool(false);
    }
    if py_truthy(&mcall(&g_PLAIN(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = _passed_alone(&[clean.clone(), person.clone()]); if py_truthy(&__b) { __b } else { { let __b = _passed_on(&[clean.clone(), person.clone()]); if py_truthy(&__b) { __b } else { _death_noun(&[clean.clone(), words.clone(), person.clone()]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_WAKE(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_buried_person(&[clean.clone(), words.clone(), person.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_SYMPATHY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_CARD_WORD(), "search", &[clean.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_TOOK(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_KILLED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_killed_in_a_story(&[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = person.clone(); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g_PASSED_ON_SUBJECT()]).unwrap_or_else(|e| exc::raise(e))) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_put_down(&[clean.clone(), words.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_STILL_CRY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_OTHER_WEEPS(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THEIR_TEARS_IN_A_CLAUSE(), "search", &[clean.clone()]))) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_DEATH_AS_A_THING(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_DEATH(), "search", &[clean.clone()]))) } }) {
    if py_truthy(&{ let __b = mcall(&g_OWN_WORRY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_THING_DEATH(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PET_DEATH(), "search", &[clean.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_FICTION(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&person.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_DEATH_COMPOUND(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    owner = mcall(&g_DEATH_OF_OWNER(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = owner.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_THINGS(), &geti(&mcall(&mcall(&owner.clone(), "group", &[Py::Int(1)]), "split", &[]), &neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&person.clone())) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    lost_the = mcall(&g_LOST_THE(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = lost_the.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&deathable.clone(), &mcall(&lost_the.clone(), "group", &[Py::Int(1)]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_PET_WORDS(), &mcall(&lost_the.clone(), "group", &[Py::Int(1)]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_LOST_THE_FOUND(), "search", &[clean.clone()]))) } } } } } }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&g_LOST_FORMS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    group = __unpk2[1].clone();
    found = mcall(&pattern.clone(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&deathable.clone(), &mcall(&found.clone(), "group", &[group.clone()]))) } }) {
    if py_truthy(&_their_thing_after(&[clean.clone(), mcall(&found.clone(), "end", &[])])) {
    continue;
    }
    if py_truthy(&{ let __b = _lost_in_a_place_after(&[clean.clone(), mcall(&found.clone(), "end", &[])]); if py_truthy(&__b) { __b } else { { let __b = _shed_after(&[clean.clone(), mcall(&found.clone(), "end", &[])]); if py_truthy(&__b) { __b } else { _feared_not_done(&[clean.clone(), mcall(&found.clone(), "start", &[])]) } } } }) {
    continue;
    }
    return Py::Bool(true);
    }
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![g_LOSS_OF(), g_NEWS_OF()]));
    for __item in __iter3 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&deathable.clone(), &mcall(&found.clone(), "group", &[Py::Int(1)]))) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&pattern.clone(), &g_LOSS_OF())); if !py_truthy(&__b) { __b } else { _lost_in_a_place_after(&[clean.clone(), mcall(&found.clone(), "end", &[])]) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    }
    losing = mcall(&g_LOSING(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = losing.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&deathable.clone(), &mcall(&losing.clone(), "group", &[Py::Int(1)]))); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_contains(&g_CLOSE(), &mcall(&losing.clone(), "group", &[Py::Int(1)]))); if py_truthy(&__b) { __b } else { mcall(&g_LOSS_GRIEF_CUE(), "search", &[clean.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_shed_after(&[clean.clone(), mcall(&losing.clone(), "end", &[])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_feared_not_done(&[clean.clone(), mcall(&losing.clone(), "start", &[])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_lost_in_a_place_after(&[clean.clone(), mcall(&losing.clone(), "end", &[])]))) } } } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&marks_a_day(&[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_CONDITIONAL_TAIL(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_HUMAN_WORDS()))) } }) {
    return Py::Bool(false);
    }
    a_pronoun = call_builtin("bool", &[bitand(&call_builtin("set", &[words_of(&[_spoken(&[clean.clone()])])]).unwrap_or_else(|e| exc::raise(e)), &g__PEOPLE_PRONOUNS())]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_ANTICIPATED(), "search", &[_spoken(&[clean.clone()])]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = person.clone(); if py_truthy(&__b) { __b } else { a_pronoun.clone() } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_object_died(&[words.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__WORKING_WITH_THE_DYING(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__SAID_DYING(), "search", &[clean.clone()]))) } })) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&person.clone())); if !py_truthy(&__b) { __b } else { { let __b = a_pronoun.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_DYING_NOW(), "search", &[_spoken(&[clean.clone()])]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_object_died(&[words.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DYING_TO(), "search", &[clean.clone()]))) } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_GOING_TO_DIE(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = person.clone(); if py_truthy(&__b) { __b } else { a_pronoun.clone() } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_object_died(&[words.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = a_pet.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_DYING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DIED(), "search", &[clean.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_object_died(&[words.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_DEATH(), "search", &[clean.clone()]))) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_accident_death(&[clean.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_DYING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DIED(), "search", &[clean.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = person.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_object_died(&[words.clone()]))) } } } }) {
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    index = __unpk5[0].clone();
    word = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("dead"))); if !py_truthy(&__b) { __b } else { _dead_is_not_a_death(&[words.clone(), index.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("died"), py_str("dies"), py_str("dead"), py_str("dying")]), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _accident_death(args: &[Py]) -> Py {
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
    let mut victim: Py = Py::None;
    let _ = py_str("True when the line says someone of ours died in an accident, without the word \"died\".\n\n    The passive is what makes it a death rather than a mishap of the speaker's doing: \"my cat was\n    run over last night\" is a loss, \"i ran over a cat\" is not the same line from the wheel. The\n    loss also has to be of ours - an owner and a person or a pet - so \"the shed was knocked down\"\n    and \"we were run over by the rush\" stay ordinary lines, and a line that says the person is\n    fine (\"was run over but hes ok\", \"she's in hospital\") is a fright, not a bereavement.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_MISADVENTURE(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_SURVIVED(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NOT_A_DEATH(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_VERB_OBJECT(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    victim = _owned_deathable(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&victim.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_PET_WORDS(), &victim.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_KNOCKED_BY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DEATH_SAID(), "search", &[clean.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _owned_deathable(args: &[Py]) -> Py {
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
    let mut owner: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The person or pet an accident happened to, or None.\n\n    Every word after the owner is tried, so the adjective in \"our old dog\" and \"my little\n    brother\" does not stand in for the one that can die.\n    ");
    let __iter1 = py_iter_or_raise(&mcall(&g_OWNED(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    owner = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&mcall(&owner.clone(), "group", &[Py::Int(1)]), "split", &[]));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_DEATHABLE(), &word.clone()))) {
    return word.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _passed_on(args: &[Py]) -> Py {
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
    let mut person: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut _u: Py = Py::None;
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when \"passed on\" is a death, not a declined offer.\n\n    \"my nan passed on in march\" and \"she passed on last year\" are bereavements; \"i passed on the\n    message to my boss\", \"we passed on the house\" and \"i passed on dessert\" are not. A person or\n    pet has to be the subject, and the word after \"on\" must not be an object.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&clean.clone(), &g_PASSED_ON()))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&mcall(&clean.clone(), "partition", &[g_PASSED_ON()]), 3, None);
    before = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    after = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&person.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[slice_or_raise(&mcall(&before.clone(), "split", &[]), &neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_PASSED_ON_SUBJECT()))) } }) {
    return Py::Bool(false);
    }
    rest = mcall(&after.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_PASSED_ON_OBJECT(), &geti(&rest.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&rest.clone(), &Py::Int(0)), &py_str("the"))); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MONTHS(), &geti(&rest.clone(), &Py::Int(1)))); if py_truthy(&__b) { __b } else { { let __b = mcall(&slice_or_raise(&geti(&rest.clone(), &Py::Int(1)), &Py::None, &Py::Int(1), &Py::None), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("1st"), py_str("2nd"), py_str("3rd")]), &geti(&rest.clone(), &Py::Int(1)))) } } } } } } } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _passed_alone(args: &[Py]) -> Py {
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
    let mut person: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut head: Py = Py::None;
    let mut i: Py = Py::None;
    let mut nxt: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when \"passed\" on its own is a death, not a test, a deadline or a plate.\n\n    \"my grandad passed in jan\", \"my dad passed last year\" and \"she passed suddenly\" are\n    bereavements; \"i passed my test\", \"the deadline passed\" and \"he passed me the salt\" are not.\n    A person or pet has to be the subject, and what follows has to be a date, a time or nothing.\n    ");
    words = mcall(&clean.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &g_PASSED_ALONE()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &g_PASSED_ALONE()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&person.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&i.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &i.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_PASSED_ON_SUBJECT()))) } }) {
    continue;
    }
    if py_truthy(&bitand(&call_builtin("set", &[slice_or_raise(&words.clone(), &Py::None, &i.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &g_PASSED_A_RESULT())) {
    continue;
    }
    rest = slice_or_raise(&words.clone(), &add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(true);
    }
    head = geti(&rest.clone(), &Py::Int(0));
    nxt = if py_truthy(&gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&rest.clone(), &Py::Int(1)) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_PASSED_ALONE_TAIL(), &head.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("in"), py_str("on"), py_str("at"), py_str("last"), py_str("this"), py_str("that"), py_str("back")]), &head.clone()))) {
    if py_truthy(&Py::Bool(py_eq(&head.clone(), &py_str("at")))) {
    if py_truthy(&Py::Bool(py_contains(&g_PASSED_TIME(), &nxt.clone()))) {
    return Py::Bool(true);
    }
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PASSED_DATE(), &nxt.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PASSED_TIME(), &nxt.clone())); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&nxt.clone(), &Py::None, &Py::Int(1), &Py::None), "isdigit", &[]) } } } }) {
    return Py::Bool(true);
    }
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&head.clone(), &py_str("it"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("was"), py_str("is"), py_str("has"), py_str("had")]), &nxt.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PASSED_ON_OBJECT(), &head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PASSED_ALONE_HANDED(), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PASSED_ALONE_NOUN(), &head.clone())) } } } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _death_noun(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut person: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: person"))) };
    let mut owned: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("True when the word \"death\" in this line is someone's death, not a word doing another job.\n\n    An owner makes it a particular death (\"his death\", \"my mums death\"), so it counts on its own.\n    A bare article needs a bereavement cue as well, which is the general rule that also protects\n    the proper nouns nobody listed (\"the death star lego set\", \"death valley\", \"the death\n    toll\"). Fiction, the speaker's own death and object deaths are all read out.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_DEATH_NOUN(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_DEATH(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_THING_DEATH(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PET_DEATH(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_DEATH_COMPOUND(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_FICTION(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&person.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_OWN_WORRY(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&person.clone()) {
    return Py::Bool(true);
    }
    owned = mcall(&g_OWN_DEATH(), "search", &[clean.clone()]);
    if py_truthy(&owned.clone()) {
    owner = geti(&mcall(&slice_or_raise(&clean.clone(), &mcall(&owned.clone(), "start", &[]), &mcall(&owned.clone(), "end", &[]), &Py::None), "split", &[]), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("his"), py_str("her"), py_str("their"), py_str("your"), py_str("our")]), &owner.clone()))) {
    return Py::Bool(true);
    }
    stem = mcall(&mcall(&owner.clone(), "rstrip", &[py_str("s")]), "rstrip", &[py_str("'")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PEOPLE(), &stem.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PEOPLE(), &owner.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return call_builtin("bool", &[mcall(&g_DEATH_CUE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _unnamed_person(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True only when \"someone\"/\"people\" is a real death: the line asks what to say about it.\n\n    \"what do i say when someone dies\" is the same question as \"what do i say to my friend\", so\n    the ask has to be there. A show or a film dying is never anyone's loss.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_UNNAMED(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_MEDIA(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_ASK_WORDS(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _buried_person(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut person: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "person") { v.clone() }
    else { Py::Bool(false) };
    let mut index: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True for \"we buried my dad\", false for \"i buried the fence posts\" and \"the dog buried a\n    bone\": after the verb, the thing being buried has to be a person or a pet. A passive\n    \"be buried\" is read from the subject in front of it, which the line names as a person or\n    not at all.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_BURIED(), "search", &[clean.clone()])))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("buried"), py_str("bury")]), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PASSIVE_AUX(), &geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))))) } }) {
    if py_truthy(&{ let __b = person.clone(); if py_truthy(&__b) { __b } else { { let __b = _has_pet(&[words.clone()]); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g_REMAINS()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g_BURIABLE()]).unwrap_or_else(|e| exc::raise(e)))));
    }
    rest = slice_or_raise(&words.clone(), &add(&index.clone(), &Py::Int(1)), &add(&index.clone(), &Py::Int(5)), &Py::None);
    while py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BURIED_ADJECTIVE(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("him"), py_str("her"), py_str("them")]), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BURIED_OBJECT(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None);
    while py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BURIED_ADJECTIVE(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    rest = slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_PEOPLE(), &geti(&rest.clone(), &Py::Int(0))));
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _put_down(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut phrase: Py = Py::None;
    let _ = py_str("True when \"put down\" / \"put to sleep\" is about a pet that died, not a nap or a phone.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PUT_DOWN(), "search", &[clean.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BEDTIME()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LOSS_CONTEXT()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_has_pet(&[words.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LOSS_CONTEXT()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_a_stored_pet(args: &[Py]) -> Py {
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
    let mut known: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line names a pet the store holds: \"rex died\" is about a pet the bot knows.");
    known = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    pets = add(&call_builtin("list", &[{ let __b = mcall(&known.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[{ let __b = mcall(&known.clone(), "get", &[py_str("pets_gone")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    said = call_builtin("set", &[words_of(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&pets.clone());
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    name = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = name.clone(); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &name.clone())) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _has_pet(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PET_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_person_dying(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a death that is still ahead is somebody's: \"my dad is dying\", \"shes dying\".\n\n    A phone dies, a battery dies, the house plants die. The person is read from the same words\n    the rest of the loss code uses (`_has_person`), so a relation that is not the closest family\n    counts too: \"my carer is dying\" and \"my other half is dying\" were both the person in front\n    of them, and both fell through to an echo or to the money answer that reads \"half\" as a\n    split.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&tight.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(",.?!")])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("him"), py_str("her"), py_str("them")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return _has_person(&[words.clone()]);
    Py::None
}
pub fn _a_hard_time_that_is_theirs(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line says a person close to the user is having the hard time.\n\n    \"my mum is lonely since dad died\" is about the mum, and \"i'm lonely since dad died\" is about\n    the person typing - so the relation has to be the subject, the state has to be theirs, and a\n    death has to be in the line. A pet in the line is somebody else's answer (`a_relative_pet`),\n    and so is a line about what to say to them.\n    ");
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_RELATION_IN_A_HARD_TIME(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_DEATH_WORD(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&the_animal(&[text.clone()])));
    Py::None
}
pub fn whose(args: &[Py]) -> Py {
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
    let _ = py_str("'someone' when the loss belongs to someone else, else 'own'.\n\n    Three shapes: a friend named with their relation (\"my friend's mum\"), a relation or an\n    animal named as his or hers (\"her dad\", \"their dog\"), and a household named with a\n    possessive or a neighbour's animal said as the street's (\"my mum's cat\", \"the cat next\n    door\"). All of them mean the same thing: the person in front of us is not the bereaved one,\n    so the reply is what to say rather than sympathy for them.\n    ");
    if py_truthy(&{ let __b = mcall(&g_CHILDS_OWN_LOSS(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_STRANGERS_ANIMAL(), "search", &[clean.clone()]))) } }) {
    return py_str("own");
    }
    if py_truthy(&{ let __b = mcall(&g_SOMEONE_ELSES(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HIS_HERS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_SOMEONE_ELSES_OWN(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_STRANGERS_ANIMAL(), "search", &[clean.clone()]) } } } } } }) {
    return py_str("someone");
    }
    return py_str("own");
    Py::None
}
pub fn one_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut animal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let _ = py_str("The animal in the singular, because \"A cats is family\" is not a sentence.");
    return mcall(&g_PLURAL_PAIRS(), "get", &[animal.clone(), animal.clone()]);
    Py::None
}
pub fn the_animal(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut death: Py = Py::None;
    let mut first: Py = Py::None;
    let mut named: Py = Py::None;
    let mut second: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The animal the line names (\"dog\", \"guinea pig\"), or \"\" when it names none.");
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    words = words_of(&[text.clone()]);
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_PET_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return py_str("");
    }
    death = mcall(&g_DEATH_WORD(), "search", &[text.clone()]);
    if py_truthy(&death.clone()) {
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[slice_or_raise(&text.clone(), &Py::None, &mcall(&death.clone(), "start", &[]), &Py::None)])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_PET_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    named = { let __b = before.clone(); if py_truthy(&__b) { __b } else { named.clone() } };
    }
    let __iter1 = py_iter_or_raise(&g_TWO_WORD_PETS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    first = __unpk2[0].clone();
    second = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &second.clone())) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&first.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&second.clone(), -1, "")); py_string(__s) };
    }
    }
    return geti(&named.clone(), &neg(&Py::Int(1)));
    Py::None
}
pub fn named_animal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut replies: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "replies") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: replies"))) };
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut animal: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The pool with the animal the line named in it: \"my mum's dog died\" hears about the dog.\n\n    \"pet\" is the word when the line names no animal at all, which reads in every one of these\n    sentences and cannot be wrong the way a guessed animal can.\n    ");
    animal = { let __b = the_animal(&[line.clone()]); if py_truthy(&__b) { __b } else { py_str("pet") } };
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&replies.clone()) { let mut reply: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("animal".to_string(), animal.clone()), ("one".to_string(), one_of(&[animal.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_relative_pet(args: &[Py]) -> Py {
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
    let _ = py_str("True when the animal that died was someone else's in the family: \"my mum's cat died\".");
    match_ = mcall(&g_SOMEONE_ELSES_OWN(), "search", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]);
    return call_builtin("bool", &[{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PET_WORDS(), &mcall(&match_.clone(), "group", &[py_str("who")]))) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_strangers_animal(args: &[Py]) -> Py {
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
    let _ = py_str("True when the animal that died belongs to nobody in the house: \"the cat next door\".");
    return call_builtin("bool", &[mcall(&g_A_STRANGERS_ANIMAL(), "search", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_swap_of_the_question_before(args: &[Py]) -> Py {
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
    let mut previous: Py = Py::None;
    let mut t: Py = Py::None;
    let mut tight_line: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the line only swaps the thing in the question before it, and that question asked\n    something this file's funeral card does not answer.\n\n    \"what about the wake\" a turn after \"what should i wear to a funeral\" is the wear question\n    with the wake in it, so the clothes answer fits and \"Go if you can, even just for the\n    service\" is advice for a different question. The card keeps the line when the question\n    before was about going, about what to say, or when nothing was asked before at all.\n    ");
    tight_line = _tight(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SWAP_ASK(), "match", &[tight_line.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&gt(&call_builtin("len", &[_plain(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_SWAP_MOST_WORDS())) {
    return Py::Bool(false);
    }
    previous = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }, &Py::None, &Py::Int(8), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&line.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_plain(&[line.clone()]), &_plain(&[text.clone()]))) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_ASKS(), "search", &[mcall(&py_str(" "), "join", &[_plain(&[line.clone()])])])) {
    previous = line.clone();
    break;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&previous.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_plain(&[previous.clone()]))) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_SWAP_INHERITS(), "search", &[_tight(&[previous.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _still_ahead(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut record: Py = Py::None;
    let mut t: Py = Py::None;
    let mut tight_line: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The pool for a death that has not come yet, or None when it has.\n\n    A chat about a pet the vet has given weeks to, or a parent in the last stages, is at the\n    days before - and the answer to \"what do i do now\" there is about those days, never the card\n    for what happens to a body.\n    ");
    record = mcall(&memory.clone(), "get", &[py_str("loss")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&record.clone(), "get", &[py_str("ahead")]) } }) {
    pool = if py_truthy(&mcall(&record.clone(), "get", &[py_str("pet")])) { g_THE_DAYS_BEFORE() } else { g_SOMEONE_DYING() };
    return _less_what_was_said(&[pool.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }]);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    tight_line = _tight(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUESTION(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_YET(), "search", &[tight_line.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&_has_pet(&[_plain(&[line.clone()])])) {
    return _less_what_was_said(&[g_THE_DAYS_BEFORE(), turns.clone()]);
    }
    if py_truthy(&_a_person_dying(&[tight_line.clone()])) {
    return _less_what_was_said(&[g_SOMEONE_DYING(), turns.clone()]);
    }
    }
    last = if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THE_DAYS_BEFORE(), &last.clone())) } }) {
    return _less_what_was_said(&[g_THE_DAYS_BEFORE(), turns.clone()]);
    }
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SOMEONE_DYING(), &last.clone())) } }) {
    return _less_what_was_said(&[g_SOMEONE_DYING(), turns.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _less_what_was_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut turns: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut kept: Py = Py::None;
    let mut line: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let _ = py_str("The pool with the advice this chat has already been given taken out.\n\n    \"my dad is dying\" then \"what do i do now\": the answer to the second is about the days before\n    it, and the bot had already said to ask the doctor what to expect. Handing that back with the\n    words rearranged reads as a bot that is not listening, so a line the chat has already had -\n    and a line that sends them to the same people - is not offered again (fast-shelf, #3379).\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return pool.clone();
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &mcall(&line.clone(), "strip", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__HELPERS(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&g__HELPERS(), "search", &[one.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } })) } }) {  __out.push(line.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("tuple", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { pool.clone() } };
    Py::None
}
pub fn _an_animal_in_the_chat(args: &[Py]) -> Py {
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
    let mut turns: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { Py::Int(6) };
    let mut history: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when a line earlier in this chat named an animal of ours.\n\n    \"is it cruel to keep him going\" carries no animal in it, and the only way to know it is a dog\n    \"or a cat is the line before it: \"my dog is 12 and going blind\" then \"he keeps bumping into\n    \"things\". Read off the turns, never assumed.\n    ");
    history = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[history.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&slice_or_raise(&history.clone(), &neg(&turns.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_PET(), "search", &[_tight(&[said.clone()])]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _keeping_going(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut judged: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("The pool for whether to keep an animal going, or None when the line is not that question.\n\n    Kept to animals: the end of a person's life is a different question with a different answer,\n    and the lines that ask it are handled by the dying pools above. When the line names no animal,\n    the chat has to have one, so \"is it cruel to keep him going\" after \"my dog is 12\" reads and\n    \"is it cruel to keep him going\" on its own does not.\n    ");
    judged = call_builtin("bool", &[mcall(&g_KEEPING_JUDGED(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&judged.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ENDING_ASKED(), "search", &[tight.clone()]))) } }) {
    return Py::None;
    }
    where_ = if py_truthy(&judged.clone()) { g_KEEPING_GOING_REPLIES() } else { g_IS_IT_TIME_REPLIES() };
    if py_truthy(&_has_pet(&[words.clone()])) {
    return where_.clone();
    }
    if py_truthy(&_has_person(&[words.clone()])) {
    return Py::None;
    }
    if py_truthy(&_an_animal_in_the_chat(&[memory.clone()])) {
    return where_.clone();
    }
    return Py::None;
    Py::None
}
pub fn _first_days(args: &[Py]) -> Py {
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
    let mut pet: Py = Py::None;
    let mut record: Py = Py::None;
    let _ = py_str("The pool for the days just after a death: the animal's, or a person's.");
    record = mcall(&memory.clone(), "get", &[py_str("loss")]);
    pet = { let __b = call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&record.clone(), "get", &[py_str("pet")]) } };
    return if py_truthy(&pet.clone()) { g_FIRST_DAYS_PET() } else { g_FIRST_DAYS_PERSON() };
    Py::None
}
pub fn _the_visit_ask(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks what to say to the person who is dying, not what to tell others.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SAY_GOODBYE(), "search", &[clean.clone()])))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_SAY_TO_WHO(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_VISIT_TARGETS(), &mcall(&found.clone(), "group", &[Py::Int(1)])))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_animal_of(args: &[Py]) -> Py {
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
    let mut animal: Py = Py::None;
    let mut record: Py = Py::None;
    let _ = py_str("The animal this turn is about: the one the line names, else the one the loss was about.");
    animal = the_animal(&[text.clone()]);
    if py_truthy(&animal.clone()) {
    return animal.clone();
    }
    record = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("loss")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    return the_animal(&[call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return py_str("");
    Py::None
}
pub fn _grief_replies(args: &[Py]) -> Py {
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
    let mut animal: Py = Py::None;
    let mut article: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut singular: Py = Py::None;
    let _ = py_str("The pool for \"does it get easier\": a pet's death is answered with the animal in it.\n\n    Only the person's own animal counts: \"my friend's dog died\" is a line about what to say to\n    your friend, and a line about the gap in *your* day would be somebody else's grief.\n    ");
    if py_truthy(&Py::Bool(py_eq(&whose(&[line.clone()]), &py_str("someone")))) {
    return g_GRIEF_REPLIES();
    }
    animal = _the_animal_of(&[line.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&animal.clone()))) {
    return g_GRIEF_REPLIES();
    }
    if py_truthy(&mcall(&animal.clone(), "endswith", &[py_str("s")])) {
    singular = slice_or_raise(&animal.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PET_WORDS(), &singular.clone())); if py_truthy(&__b) { __b } else { mcall(&singular.clone(), "endswith", &[py_str(" pig")]) } }) {
    animal = singular.clone();
    }
    }
    article = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &slice_or_raise(&animal.clone(), &Py::None, &Py::Int(1), &Py::None)))) { py_str("an") } else { py_str("a") };
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PET_GRIEF_REPLIES()) { let mut reply: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("animal".to_string(), animal.clone()), ("article".to_string(), article.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_colour_is_worn(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks about a colour of clothes (the knowledge base's own reading).");
    let __try1 = exc::catch(|| {
    let mut lookup: Py = crate::gen::knowledge::lookup::module_obj();
    return py_tuple(vec![Py::Bool(true), mcall(&lookup.clone(), "a_colour_is_worn", &[line.clone()])]);
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
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { Py::None };
    let mut topics: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topics") { v.clone() }
    else { Py::None };
    let mut mood: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mood") { v.clone() }
    else { Py::None };
    let mut ahead: Py = Py::None;
    let mut ahead_now: Py = Py::None;
    let mut animal: Py = Py::None;
    let mut asking: Py = Py::None;
    let mut cost_pool: Py = Py::None;
    let mut feeling: Py = Py::None;
    let mut habit: Py = Py::None;
    let mut keeping: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tight: Py = Py::None;
    let mut words: Py = Py::None;
    let mut words_now: Py = Py::None;
    let _ = py_str("The pool the line asks for, or None when it is not about a death.\n\n    The caller picks from the pool, so the wording still varies and never repeats. A follow-up\n    to the bot's own sympathy (\"what do I even say to him\", \"i still can't sleep\") is read the\n    same way, because the family of a death keeps asking the same few things.\n    ");
    words = words_of(&[text.clone()]);
    line = if py_truthy(&{ let __b = call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { clean.clone() } }) { clean.clone() } else { mcall(&py_str(" "), "join", &[words.clone()]) };
    tight = _tight(&[line.clone()]);
    if py_truthy(&{ let __b = _a_colour_is_worn(&[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_DEATH_HAPPENED(), "search", &[tight.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__dream(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&gattr(&g__dream(), "SETTLED_IT"), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__dream(), "told_in_the_turn_before", &[memory.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__dream(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__dream(), "a_dream_is_not_happening", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__dream(), "a_death_came_first", &[line.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&_only_a_new_subject(&[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_DYING_OTHER(), "search", &[tight.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_SPEAKER_TOO(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WISH_FRAME(), "search", &[tight.clone()]))) } }) {
    rest = mcall(&mcall(&g_SPEAKER_TOO(), "sub", &[py_str(" "), tight.clone()]), "strip", &[]);
    animal = { let __b = call_builtin("bool", &[mcall(&g_PET(), "search", &[rest.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _a_pet_of_ours(&[rest.clone(), words.clone()]) } };
    if py_truthy(&{ let __b = is_dying_now(&[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = _the_end_is_near(&[rest.clone(), words.clone()]); if py_truthy(&__b) { __b } else { { let __b = animal.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_AT_THE_END(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PET_AT_THE_END(), "search", &[rest.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_PET_HAS_HAD_ENOUGH(), "search", &[rest.clone()]) } } } } } } } } } }) {
    return g_SPEAKER_TOO_REPLIES();
    }
    }
    if py_truthy(&{ let __b = mcall(&g_PET_HAS_HAD_ENOUGH(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PET(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _a_pet_of_ours(&[line.clone(), words.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SPEAKER_HAS_HAD_ENOUGH(), "search", &[tight.clone()]))) } } } } } }) {
    return g_THE_DAYS_BEFORE();
    }
    if py_truthy(&{ let __b = mcall(&g_PET_AT_THE_END(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PET(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _a_pet_of_ours(&[line.clone(), words.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SPEAKER_HAS_HAD_ENOUGH(), "search", &[tight.clone()]))) } } } } } }) {
    return g_THE_DAYS_BEFORE();
    }
    if py_truthy(&{ let __b = mcall(&g_AT_THE_END(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SPEAKER_HAS_HAD_ENOUGH(), "search", &[tight.clone()]))) } }) {
    if py_truthy(&is_dying_now(&[tight.clone()])) {
    if py_truthy(&{ let __b = mcall(&g_PET(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { _a_pet_of_ours(&[line.clone(), words.clone()]) } }) {
    return g_THE_DAYS_BEFORE();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASK_WORDS(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&line.clone(), "startswith", &[py_tuple(vec![py_str("what"), py_str("how"), py_str("should"), py_str("can"), py_str("do"), py_str("does"), py_str("is"), py_str("why"), py_str("when")])]) } } } }) {
    return g_DYING_SAY_REPLIES();
    }
    return g_DYING_REPLIES();
    }
    }
    if py_truthy(&{ let __b = _the_visit_ask(&[tight.clone()]); if !py_truthy(&__b) { __b } else { dying_in_the_chat(&[memory.clone()]) } }) {
    return g_DYING_SAY_REPLIES();
    }
    asking = { let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_ASK_FORM(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_ASK_WORDS(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&line.clone(), "startswith", &[py_tuple(vec![py_str("what"), py_str("how"), py_str("should"), py_str("can"), py_str("do"), py_str("does"), py_str("is"), py_str("why"), py_str("when")])]) } } } } } };
    if py_truthy(&{ let __b = mcall(&g_EULOGY_WORD(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_EULOGY_ASK(), "search", &[tight.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_WEAR(), "search", &[tight.clone()])) {
    return Py::None;
    }
    if py_truthy(&_a_swap_of_the_question_before(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&_a_gift_for_someone_in_care(&[tight.clone()])) {
    return Py::None;
    }
    if py_truthy(&_telling_a_child(&[tight.clone()])) {
    return Py::None;
    }
    cost_pool = { let __b = arrangement_cost_pool(&[line.clone(), tight.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { arrangement_cost_pool(&[text.clone(), tight.clone(), memory.clone()]) } };
    if py_truthy(&cost_pool.clone()) {
    return cost_pool.clone();
    }
    if py_truthy(&{ let __b = is_practical_ask(&[line.clone(), tight.clone()]); if py_truthy(&__b) { __b } else { is_practical_ask(&[text.clone(), tight.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(!py_truthy(&mcall(&g_QUESTION(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_ASK_WORDS(), "search", &[tight.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_NOT_THE_END(), "search", &[tight.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_ESTIMATE(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { _the_end_is_near(&[tight.clone(), words.clone()]) } } } } } } } }; if !py_truthy(&__b) { __b } else { { let __b = _has_pet(&[words.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_person_dying(&[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CARE_WHO(), "search", &[tight.clone()]) } } } } } }) {
    pet = { let __b = _has_pet(&[words.clone()]); if py_truthy(&__b) { __b } else { _the_vet_said_it(&[tight.clone(), words.clone()]) } };
    return if py_truthy(&pet.clone()) { g_THE_DAYS_BEFORE() } else { g_SOMEONE_DYING() };
    }
    if py_truthy(&mcall(&g_OPEN_ASK(), "match", &[tight.clone()])) {
    ahead = _still_ahead(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_is(&ahead.clone(), &Py::None))) {
    return ahead.clone();
    }
    }
    if py_truthy(&lost_in_a_place(&[text.clone()])) {
    return g_MISLAID_REPLIES();
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_GET_THROUGH(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_GET_THROUGH_DAY(), "search", &[tight.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GET_THROUGH_TO(), "search", &[tight.clone()]))) } }) {
    ahead = call_builtin("bool", &[{ let __b = is_dying_now(&[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = dying_in_the_chat(&[memory.clone()]); if py_truthy(&__b) { __b } else { _still_ahead(&[memory.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&_a_thing_of_its_own_to_get_through(&[tight.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = ahead.clone(); if py_truthy(&__b) { __b } else { { let __b = continues(&[memory.clone()]); if py_truthy(&__b) { __b } else { reads_as_loss(&[text.clone(), words.clone(), _names_a_stored_pet(&[text.clone(), memory.clone()])]) } } } }) {
    if py_truthy(&mcall(&g_GET_THROUGH_DAY(), "search", &[tight.clone()])) {
    return g_REMEMBER_REPLIES();
    }
    if py_truthy(&ahead.clone()) {
    return g_GETTING_THROUGH_DYING_REPLIES();
    }
    return g_GETTING_THROUGH_REPLIES();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&continues(&[memory.clone()]))); if !py_truthy(&__b) { __b } else { _an_ending_in_the_chat(&[memory.clone()]) } }) {
    if py_truthy(&{ let __b = _how_long_since(&[tight.clone(), mcall(&tight.clone(), "split", &[])]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_subject(&[topics.clone()]))) } }) {
    return g_ENDING_SINCE_REPLIES();
    }
    if py_truthy(&{ let __b = mcall(&g_A_FEELING_BETTER_ASK(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&sub(&sub(&sub(&call_builtin("set", &[{ let __b = topics.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &g_THREAD_TOPICS()), &g_MEASURE_TOPICS()), &py_set(vec![py_str("health")])))) } }) {
    return g_ENDING_FEELINGS_REPLIES();
    }
    }
    keeping = _keeping_going(&[tight.clone(), words.clone(), memory.clone()]);
    if py_truthy(&keeping.clone()) {
    return keeping.clone();
    }
    if py_truthy(&reads_as_loss(&[text.clone(), words.clone(), _names_a_stored_pet(&[text.clone(), memory.clone()])])) {
    if py_truthy(&{ let __b = mcall(&g_CARD(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASK_WORDS(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = asking.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CARD_TO_WRITE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_WRITE_WORD(), "search", &[tight.clone()]) } } } } } } } }) {
    return g_CARD_REPLIES();
    }
    ahead_now = { let __b = is_dying_now(&[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = _the_end_is_near(&[tight.clone(), words.clone()]); if py_truthy(&__b) { __b } else { { let __b = dying_in_the_chat(&[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_TREATMENT_STOPPED(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOTHING_MORE_TO_DO(), "search", &[tight.clone()]) } } } } } } } };
    if py_truthy(&{ let __b = ahead_now.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_FUNERAL(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WAKE(), "search", &[tight.clone()]) } })) } }) {
    if py_truthy(&{ let __b = _a_pet_of_ours(&[line.clone(), words.clone()]); if py_truthy(&__b) { __b } else { _the_vet_said_it(&[tight.clone(), words.clone()]) } }) {
    return g_THE_DAYS_BEFORE();
    }
    if py_truthy(&mcall(&g_ASK_WORDS(), "search", &[tight.clone()])) {
    return g_DYING_SAY_REPLIES();
    }
    if py_truthy(&mcall(&g_QUESTION(), "search", &[line.clone()])) {
    return g_DYING_GRIEF_REPLIES();
    }
    return g_DYING_REPLIES();
    }
    if py_truthy(&mcall(&g_ASK_WORDS(), "search", &[tight.clone()])) {
    return g_SAY_REPLIES();
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_YET(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_PUTTING_REGRETTED(), "search", &[tight.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_QUESTION(), "search", &[line.clone()]))) } } } }) {
    if py_truthy(&mcall(&g_PET(), "search", &[tight.clone()])) {
    return g_THE_DAYS_BEFORE();
    }
    if py_truthy(&_a_person_dying(&[tight.clone()])) {
    return g_SOMEONE_DYING();
    }
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_FUNERAL(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WAKE(), "search", &[tight.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = { let __b = asking.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&whose(&[line.clone()]), &py_str("someone"))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_feeling(&[tight.clone()]))) } } } }) {
    if py_truthy(&_when_asked(&[tight.clone()])) {
    return g_FUNERAL_WHEN_REPLIES();
    }
    return g_FUNERAL_REPLIES();
    }
    if py_truthy(&{ let __b = marks_a_day(&[tight.clone()]); if !py_truthy(&__b) { __b } else { asks_what_to_do(&[tight.clone()]) } }) {
    return g_REMEMBER_REPLIES();
    }
    feeling = _feeling(&[tight.clone()]);
    if py_truthy(&{ let __b = feeling.clone(); if !py_truthy(&__b) { __b } else { _asks_about_the_feeling(&[tight.clone()]) } }) {
    return geti(&g_FEELING_REPLIES(), &feeling.clone());
    }
    if py_truthy(&mcall(&g_QUESTION(), "search", &[line.clone()])) {
    return _grief_replies(&[line.clone(), memory.clone()]);
    }
    if py_truthy(&feeling.clone()) {
    return geti(&g_FEELING_REPLIES(), &feeling.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&whose(&[line.clone()]), &py_str("someone")))) {
    if py_truthy(&a_relative_pet(&[line.clone()])) {
    return named_animal(&[g_RELATIVE_PET_REPLIES(), line.clone()]);
    }
    if py_truthy(&a_strangers_animal(&[line.clone()])) {
    return named_animal(&[g_STRANGER_ANIMAL_REPLIES(), line.clone()]);
    }
    if py_truthy(&bitand(&call_builtin("set", &[words_of(&[line.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_CLOSE())) {
    return g_NAMED_LOSS_SAY_REPLIES();
    }
    return g_SAY_REPLIES();
    }
    if py_truthy(&mcall(&g_PUTTING_REGRETTED(), "search", &[tight.clone()])) {
    return g_PUT_DOWN_REGRET_REPLIES();
    }
    habit = _still_here(&[line.clone(), { let __b = words.clone(); if py_truthy(&__b) { __b } else { mcall(&tight.clone(), "split", &[]) } }]);
    if py_truthy(&habit.clone()) {
    return habit.clone();
    }
    if py_truthy(&{ let __b = _a_hard_time_that_is_theirs(&[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&whose(&[line.clone()]), &py_str("own"))) } }) {
    return g_THEIR_HARD_TIME_REPLIES();
    }
    return g_LOSS_REPLIES();
    }
    if py_truthy(&Py::Bool(!py_truthy(&continues(&[memory.clone()])))) {
    if py_truthy(&_missing_someone(&[tight.clone()])) {
    return g_MISS_REPLIES();
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = dying_in_the_chat(&[memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUESTION(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { _feeling(&[tight.clone()]) } } } }) {
    return g_DYING_GRIEF_REPLIES();
    }
    if py_truthy(&{ let __b = dying_in_the_chat(&[memory.clone()]); if !py_truthy(&__b) { __b } else { _the_visit_ask(&[tight.clone()]) } }) {
    return g_DYING_SAY_REPLIES();
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&_thread_pool(&[memory.clone()]), &g_MISS_REPLIES())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_GRIEF_ASK_FORM(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_REFERS_TO_THEM(), "search", &[tight.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__DEATH_IN_LINE(), "search", &[tight.clone()]))); if !py_truthy(&__b) { __b } else { _a_body_with_its_own_answer(&[tight.clone(), mcall(&tight.clone(), "split", &[])]) } } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_HELLO_ON_ITS_OWN(), "match", &[tight.clone()])) {
    return g_HELLO_AFTER();
    }
    if py_truthy(&mcall(&g_OPEN_ASK(), "match", &[tight.clone()])) {
    return { let __b = _still_ahead(&[memory.clone()]); if py_truthy(&__b) { __b } else { _first_days(&[memory.clone()]) } };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __kws.extend(vec![("known".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| marks_a_day(args))(&__args) }; if !py_truthy(&__b) { __b } else { asks_what_to_do(&[line.clone()]) } }) {
    return g_REMEMBER_REPLIES();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asking.clone())); if !py_truthy(&__b) { __b } else { { let __b = _a_habit_in(&[add(&add(&tight.clone(), &py_str(" ")), &line.clone())]); if !py_truthy(&__b) { __b } else { { let __b = continues(&[memory.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![g_MISS_REPLIES(), g_AFTER_REPLIES(), g_THE_HABIT_REPLIES()]), &_thread_pool(&[memory.clone()]))) } } } } } }) {
    habit = _still_here(&[line.clone(), mcall(&tight.clone(), "split", &[])]);
    if py_truthy(&habit.clone()) {
    return habit.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&_thread_pool(&[memory.clone()]), &g_MISS_REPLIES())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asking.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_STILL_ABOUT_THEM(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_question_about_the_feeling(&[line.clone(), call_builtin("set", &[mcall(&tight.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_hands_over_a_detail(&[tight.clone()]))) } } } } } } } }) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&{ let __b = mcall(&g_RITUAL(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_RITUAL_PERSON(), "search", &[tight.clone()]) } }) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&lost_a_thing(&[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_CARD(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASK_WORDS(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CARD_TO_WRITE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_WRITE_WORD(), "search", &[tight.clone()]) } } } } } }) {
    return g_CARD_REPLIES();
    }
    if py_truthy(&mcall(&g_ASK_WORDS(), "search", &[tight.clone()])) {
    return g_SAY_REPLIES();
    }
    if py_truthy(&{ let __b = { let __b = mcall(&g_FUNERAL(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WAKE(), "search", &[tight.clone()]) } }; if !py_truthy(&__b) { __b } else { { let __b = asking.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_feeling(&[tight.clone()]))) } } } }) {
    if py_truthy(&_when_asked(&[tight.clone()])) {
    return g_FUNERAL_WHEN_REPLIES();
    }
    return g_FUNERAL_REPLIES();
    }
    if py_truthy(&mcall(&g_GRIEF_POINTS_BACK(), "match", &[tight.clone()])) {
    return _grief_replies(&[line.clone(), memory.clone()]);
    }
    words_now = mcall(&tight.clone(), "split", &[]);
    if py_truthy(&{ let __b = mcall(&g_QUESTION(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = _names_a_subject(&[topics.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_state_describes_them(&[words_now.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_body_is_theirs(&[tight.clone(), words_now.clone()]))) } } } })) } }) {
    return _grief_replies(&[line.clone(), memory.clone()]);
    }
    said = call_builtin("set", &[words_now.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&bitand(&said.clone(), &g_GRIEF_FEELINGS())) {
    if py_truthy(&_a_question_about_the_feeling(&[line.clone(), said.clone()])) {
    return Py::None;
    }
    return g_AFTER_REPLIES();
    }
    if py_truthy(&{ let __b = _about_family(&[words_now.clone()]); if !py_truthy(&__b) { __b } else { bitand(&said.clone(), &g_GRIEF_SELF()) } }) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&{ let __b = mcall(&g_REFERS_TO_THEM(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _about_them(&[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mood.clone(), &py_str("positive"))) } } } }) {
    if py_truthy(&{ let __b = mcall(&g_IDEA_ASK(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = asking.clone(); if !py_truthy(&__b) { __b } else { _own_subject(&[topics.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&_thread_pool(&[memory.clone()]), &g_SAY_REPLIES()))) {
    return g_SAY_REPLIES();
    }
    return g_AFTER_REPLIES();
    }
    if py_truthy(&{ let __b = mcall(&g_ABSENCE(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asking.clone())) } }) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&{ let __b = mcall(&g_REASSURING(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asking.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_own_subject(&[topics.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PEACE_IS_MINE(), "search", &[tight.clone()]))) } } } } } }) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&{ let __b = _how_long_since(&[tight.clone(), words_now.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_subject(&[topics.clone()]))) } }) {
    return g_TIME_SINCE_REPLIES();
    }
    if py_truthy(&{ let __b = _own_subject(&[topics.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_state_describes_them(&[words_now.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let __b = _about_them(&[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_TIME_OF_DAY(), "search", &[tight.clone()]) } }; if !py_truthy(&__b) { __b } else { _own_state(&[words_now.clone()]) } }) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&add(&g_GRIEF_REFERRING(), &g_GRIEF_OBJECTS())) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&tight.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&Py::Bool(py_eq(&mood.clone(), &py_str("negative")))) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&mcall(&g_ABSENCE(), "search", &[line.clone()])) {
    return g_AFTER_REPLIES();
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[topics.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { topics.clone() } }) {
    if py_truthy(&Py::Bool(!py_truthy(&sub(&call_builtin("set", &[topics.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("feelings"), py_str("health"), py_str("sleep")]))))) {
    return _grief_replies(&[line.clone(), memory.clone()]);
    }
    }
    if py_truthy(&_missing_someone(&[tight.clone()])) {
    return g_MISS_REPLIES();
    }
    return Py::None;
    Py::None
}
pub fn asks_anything(args: &[Py]) -> Py {
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
    let mut tight: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { Py::None };
    let _ = py_str("True when the line puts a question, however it is written.\n\n    One reading for the whole composer: \"can we bury him at home\" asks, \"my dog died yesterday\"\n    does not, and \"my cat died, should we bury her\" asks in its second half. A bare \"?\" counts,\n    and so does a wh-word or a modal at the front without one.\n    ");
    line = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    tight = if py_truthy(&Py::Bool(py_is(&tight.clone(), &Py::None))) { _tight(&[line.clone()]) } else { tight.clone() };
    return { let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_ASK_FORM(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&line.clone(), "startswith", &[py_tuple(vec![py_str("what"), py_str("how"), py_str("should"), py_str("can"), py_str("do"), py_str("does"), py_str("is"), py_str("why"), py_str("when"), py_str("where")])]) } } } };
    Py::None
}
pub fn _a_gift_for_someone_in_care(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tight: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tight"))) };
    let _ = py_str("True for a gift asked for a person in care: the present entry answers it, not sympathy.");
    return call_builtin("bool", &[{ let __b = mcall(&g_GIFT_THING_ASK(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_IN_CARE(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_A_BEDSIDE(), "search", &[tight.clone()]))) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn is_practical_ask(args: &[Py]) -> Py {
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
    let mut tight: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tight") { v.clone() }
    else { Py::None };
    let mut asking: Py = Py::None;
    let mut opened_late: Py = Py::None;
    let _ = py_str("True for a practical question inside a bereavement: what a funeral costs, probate.\n\n    The composer does not answer these, and the person is still bereaved: \"how long does probate\n    take\" followed by \"i cant sleep\" is one conversation, so this is also what `continues` reads\n    to keep the thread open past the answer modules' reply.\n    ");
    line = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    tight = if py_truthy(&Py::Bool(py_is(&tight.clone(), &Py::None))) { _tight(&[line.clone()]) } else { tight.clone() };
    asking = { let __b = asks_anything(&[line.clone(), tight.clone()]); if py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "match", &[py_str("\\s*(?:who|who's|whos|whose|where|which)\\b"), tight.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    opened_late = { let __b = call_builtin("bool", &[mcall(&g_ASK_WE(), "search", &[tight.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_ASK_TAIL(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = { let __b = mcall(&g_FUNERAL(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_FUNERALS(), "search", &[tight.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WAKE(), "search", &[tight.clone()]) } } } }; if !py_truthy(&__b) { __b } else { a_part_of_the_day_asked(&[tight.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = { let __b = asking.clone(); if py_truthy(&__b) { __b } else { opened_late.clone() } }; if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PET_ARRANGEMENT(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_PET(), "search", &[tight.clone()]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = opened_late.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PAPER_ASK(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_GRIEF_CUE(), "search", &[tight.clone()]))) } } } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_PRACTICAL(), "search", &[tight.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_GRIEF_CUE(), "search", &[tight.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASK_WORDS(), "search", &[tight.clone()]))) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_ending_in_the_chat(args: &[Py]) -> Py {
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
    let mut stepped: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the chat holds a break-up, so the gap and the how-long ask are about that.\n\n    An ending is nobody's death, so `reads_as_loss` leaves those lines alone and the pools above\n    never see them; the chat still holds an ending, and \"its been 3 weeks\" said after it is about\n    that. It reads back past this file's own answers - the second and third follow-up in the same\n    chat are still the same chat - and stops at the first turn that is something else, so an\n    ending with a sleepless night or a job after it is history and those answers stand.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    stepped = Py::Int(0);
    while py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { lt(&stepped.clone(), &g_ENDING_TURNS()) } }) {
    said = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_AN_ENDING(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("loss"), py_str("grief")]), &call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![slice_or_raise(&turns.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), add(&stepped.clone(), &Py::Int(1))]), 2, None);
    turns = __unpk1[0].clone();
    stepped = __unpk1[1].clone();
    }
    return Py::Bool(false);
    Py::None
}
pub fn continues(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut looked_past: Py = Py::None;
    let mut module: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut stepped: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last line was sympathy or what to say, so this is still that chat.\n\n    A safety answer does not end the thread: whatever was last said about the death still stands.\n    When the death itself arrived inside the emergency (\"my mum died and i cant breathe\"), the\n    sympathy was never said, so the line that was answered as an emergency is read instead.\n\n    A practical question inside a bereavement is the same shape for a different reason: \"how long\n    does probate take\" is answered by the answer modules, not by a pool, and the answer must not\n    close the thread - the next line (\"i cant sleep\") is still about the person who died. So the\n    thread reads past up to THREAD_TURNS of those turns, and it closes on any other kind of line.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    looked_past = Py::Bool(false);
    stepped = Py::Int(0);
    while py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { lt(&stepped.clone(), &g_THREAD_TURNS()) } }) {
    if py_truthy(&Py::Bool(py_contains(&g_SAFETY_MODULES(), &call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {
    let __unpk1 = py_unpack(&py_tuple(vec![slice_or_raise(&turns.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), Py::Bool(true)]), 2, None);
    turns = __unpk1[0].clone();
    looked_past = __unpk1[1].clone();
    continue;
    }
    if py_truthy(&is_practical_ask(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")])])) {
    let __unpk2 = py_unpack(&py_tuple(vec![slice_or_raise(&turns.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), Py::Bool(true), add(&stepped.clone(), &Py::Int(1))]), 3, None);
    turns = __unpk2[0].clone();
    looked_past = __unpk2[1].clone();
    stepped = __unpk2[2].clone();
    continue;
    }
    break;
    }
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&looked_past.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(reads_as_loss(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    last = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = last.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_POOLS()) { let mut pool: Py = __item.clone(); __out.push(Py::Bool(py_contains(&pool.clone(), &last.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    module = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[module.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&add(&g_FALLBACK_MODULES(), &g_SAFETY_MODULES()), &module.clone())); if !py_truthy(&__b) { __b } else { reads_as_loss(&[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.loss".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THEM_OPENS" => Some(g_ABOUT_THEM_OPENS()),
            "ABSENCE" => Some(g_ABSENCE()),
            "ACKNOWLEDGE" => Some(g_ACKNOWLEDGE()),
            "ADMINISTRATION" => Some(g_ADMINISTRATION()),
            "ADMINISTRATION_KEYS" => Some(g_ADMINISTRATION_KEYS()),
            "AFTER_REPLIES" => Some(g_AFTER_REPLIES()),
            "ANTICIPATED" => Some(g_ANTICIPATED()),
            "AN_ENDING" => Some(g_AN_ENDING()),
            "ASKS" => Some(g_ASKS()),
            "ASKS_ABOUT_THE_DAY" => Some(g_ASKS_ABOUT_THE_DAY()),
            "ASKS_ABOUT_THE_FEELING" => Some(g_ASKS_ABOUT_THE_FEELING()),
            "ASK_AT_START" => Some(g_ASK_AT_START()),
            "ASK_BARE" => Some(g_ASK_BARE()),
            "ASK_FORM" => Some(g_ASK_FORM()),
            "ASK_NORMAL" => Some(g_ASK_NORMAL()),
            "ASK_TAIL" => Some(g_ASK_TAIL()),
            "ASK_WE" => Some(g_ASK_WE()),
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "AT_THE_END" => Some(g_AT_THE_END()),
            "A_DEATH_HAPPENED" => Some(g_A_DEATH_HAPPENED()),
            "A_FEELING_BETTER_ASK" => Some(g_A_FEELING_BETTER_ASK()),
            "A_PART_OF_THE_DAY" => Some(g_A_PART_OF_THE_DAY()),
            "A_PRICE" => Some(g_A_PRICE()),
            "A_RELATION_IN_A_HARD_TIME" => Some(g_A_RELATION_IN_A_HARD_TIME()),
            "A_SPAN" => Some(g_A_SPAN()),
            "A_SPAN_ELSEWHERE" => Some(g_A_SPAN_ELSEWHERE()),
            "A_SPAN_LEAD" => Some(g_A_SPAN_LEAD()),
            "A_SPAN_NO_NUMBER" => Some(g_A_SPAN_NO_NUMBER()),
            "A_SPAN_TAIL" => Some(g_A_SPAN_TAIL()),
            "A_STRANGERS_ANIMAL" => Some(g_A_STRANGERS_ANIMAL()),
            "A_SWAP_INHERITS" => Some(g_A_SWAP_INHERITS()),
            "BEDTIME" => Some(g_BEDTIME()),
            "BODY_ANSWER" => Some(g_BODY_ANSWER()),
            "BODY_FEELING" => Some(g_BODY_FEELING()),
            "BODY_STATE" => Some(g_BODY_STATE()),
            "BURIABLE" => Some(g_BURIABLE()),
            "BURIED" => Some(g_BURIED()),
            "BURIED_ADJECTIVE" => Some(g_BURIED_ADJECTIVE()),
            "BURIED_OBJECT" => Some(g_BURIED_OBJECT()),
            "CANT_FIND_THING" => Some(g_CANT_FIND_THING()),
            "CARD" => Some(g_CARD()),
            "CARD_REPLIES" => Some(g_CARD_REPLIES()),
            "CARD_TO_WRITE" => Some(g_CARD_TO_WRITE()),
            "CARD_WORD" => Some(g_CARD_WORD()),
            "CARE_WHO" => Some(g_CARE_WHO()),
            "CAR_LOCK_OUT" => Some(g_CAR_LOCK_OUT()),
            "CHILDS_OWN_LOSS" => Some(g_CHILDS_OWN_LOSS()),
            "CLAUSE" => Some(g_CLAUSE()),
            "CLOSE" => Some(g_CLOSE()),
            "CLOSE_PEOPLE" => Some(g_CLOSE_PEOPLE()),
            "CONDITIONAL_TAIL" => Some(g_CONDITIONAL_TAIL()),
            "CORRECTS" => Some(g_CORRECTS()),
            "COULD_BE_DAYS" => Some(g_COULD_BE_DAYS()),
            "DAYS_LEFT" => Some(g_DAYS_LEFT()),
            "DAYS_WAIT_FOR" => Some(g_DAYS_WAIT_FOR()),
            "DAY_QUESTION" => Some(g_DAY_QUESTION()),
            "DAY_THING" => Some(g_DAY_THING()),
            "DEAD_IDIOM" => Some(g_DEAD_IDIOM()),
            "DEAD_PREDICATE" => Some(g_DEAD_PREDICATE()),
            "DEATHABLE" => Some(g_DEATHABLE()),
            "DEATH_AS_A_THING" => Some(g_DEATH_AS_A_THING()),
            "DEATH_COMPOUND" => Some(g_DEATH_COMPOUND()),
            "DEATH_CUE" => Some(g_DEATH_CUE()),
            "DEATH_NOUN" => Some(g_DEATH_NOUN()),
            "DEATH_OF_OWNER" => Some(g_DEATH_OF_OWNER()),
            "DEATH_SAID" => Some(g_DEATH_SAID()),
            "DEATH_WORD" => Some(g_DEATH_WORD()),
            "DEFINES" => Some(g_DEFINES()),
            "DIED" => Some(g_DIED()),
            "DYING" => Some(g_DYING()),
            "DYING_ALREADY" => Some(g_DYING_ALREADY()),
            "DYING_GRIEF_REPLIES" => Some(g_DYING_GRIEF_REPLIES()),
            "DYING_NOW" => Some(g_DYING_NOW()),
            "DYING_OTHER" => Some(g_DYING_OTHER()),
            "DYING_REPLIES" => Some(g_DYING_REPLIES()),
            "DYING_SAY_REPLIES" => Some(g_DYING_SAY_REPLIES()),
            "DYING_TO" => Some(g_DYING_TO()),
            "ENDING_ASKED" => Some(g_ENDING_ASKED()),
            "ENDING_FEELINGS_REPLIES" => Some(g_ENDING_FEELINGS_REPLIES()),
            "ENDING_SINCE_REPLIES" => Some(g_ENDING_SINCE_REPLIES()),
            "ENDING_TURNS" => Some(g_ENDING_TURNS()),
            "ESTIMATE" => Some(g_ESTIMATE()),
            "EULOGY_ASK" => Some(g_EULOGY_ASK()),
            "EULOGY_WORD" => Some(g_EULOGY_WORD()),
            "FALLBACK_MODULES" => Some(g_FALLBACK_MODULES()),
            "FAMILY" => Some(g_FAMILY()),
            "FEARED_NOT_YET" => Some(g_FEARED_NOT_YET()),
            "FEELING_ASK" => Some(g_FEELING_ASK()),
            "FEELING_ONLY_WORDS" => Some(g_FEELING_ONLY_WORDS()),
            "FEELING_PERSON_WORDS" => Some(g_FEELING_PERSON_WORDS()),
            "FEELING_REPLIES" => Some(g_FEELING_REPLIES()),
            "FEELING_STATES" => Some(g_FEELING_STATES()),
            "FEELING_WORDS" => Some(g_FEELING_WORDS()),
            "FICTION" => Some(g_FICTION()),
            "FILLERS" => Some(g_FILLERS()),
            "FINAL_DAYS" => Some(g_FINAL_DAYS()),
            "FINDABLE" => Some(g_FINDABLE()),
            "FIRST_DAYS_PERSON" => Some(g_FIRST_DAYS_PERSON()),
            "FIRST_DAYS_PET" => Some(g_FIRST_DAYS_PET()),
            "FOUND_WHAT_WAS_LOST" => Some(g_FOUND_WHAT_WAS_LOST()),
            "FUNERAL" => Some(g_FUNERAL()),
            "FUNERALS" => Some(g_FUNERALS()),
            "FUNERAL_COST" => Some(g_FUNERAL_COST()),
            "FUNERAL_COST_COMPARE" => Some(g_FUNERAL_COST_COMPARE()),
            "FUNERAL_COST_COMPARE_REPLIES" => Some(g_FUNERAL_COST_COMPARE_REPLIES()),
            "FUNERAL_COST_ELSE" => Some(g_FUNERAL_COST_ELSE()),
            "FUNERAL_HELP" => Some(g_FUNERAL_HELP()),
            "FUNERAL_HELP_REPLIES" => Some(g_FUNERAL_HELP_REPLIES()),
            "FUNERAL_REPLIES" => Some(g_FUNERAL_REPLIES()),
            "FUNERAL_WHEN_REPLIES" => Some(g_FUNERAL_WHEN_REPLIES()),
            "GAME_TITLES" => Some(g_GAME_TITLES()),
            "GETTING_THROUGH_DYING_REPLIES" => Some(g_GETTING_THROUGH_DYING_REPLIES()),
            "GETTING_THROUGH_REPLIES" => Some(g_GETTING_THROUGH_REPLIES()),
            "GET_THROUGH" => Some(g_GET_THROUGH()),
            "GET_THROUGH_DAY" => Some(g_GET_THROUGH_DAY()),
            "GET_THROUGH_FILLER" => Some(g_GET_THROUGH_FILLER()),
            "GET_THROUGH_OBJECT" => Some(g_GET_THROUGH_OBJECT()),
            "GET_THROUGH_THE_LOSS" => Some(g_GET_THROUGH_THE_LOSS()),
            "GET_THROUGH_TO" => Some(g_GET_THROUGH_TO()),
            "GIFT_THING_ASK" => Some(g_GIFT_THING_ASK()),
            "GIVEN_UP_ON" => Some(g_GIVEN_UP_ON()),
            "GOING_TO_DIE" => Some(g_GOING_TO_DIE()),
            "GONE_THING" => Some(g_GONE_THING()),
            "GRIEF_ASK_FORM" => Some(g_GRIEF_ASK_FORM()),
            "GRIEF_CUE" => Some(g_GRIEF_CUE()),
            "GRIEF_FEELINGS" => Some(g_GRIEF_FEELINGS()),
            "GRIEF_OBJECTS" => Some(g_GRIEF_OBJECTS()),
            "GRIEF_POINTS_BACK" => Some(g_GRIEF_POINTS_BACK()),
            "GRIEF_REFERRING" => Some(g_GRIEF_REFERRING()),
            "GRIEF_REPLIES" => Some(g_GRIEF_REPLIES()),
            "GRIEF_SELF" => Some(g_GRIEF_SELF()),
            "HABIT_PATTERNS" => Some(g_HABIT_PATTERNS()),
            "HELLO_AFTER" => Some(g_HELLO_AFTER()),
            "HELLO_ON_ITS_OWN" => Some(g_HELLO_ON_ITS_OWN()),
            "HELP_ASK" => Some(g_HELP_ASK()),
            "HIS_HERS" => Some(g_HIS_HERS()),
            "HOSPICE_CARE" => Some(g_HOSPICE_CARE()),
            "HOSPICE_JOB" => Some(g_HOSPICE_JOB()),
            "HUMAN_WORDS" => Some(g_HUMAN_WORDS()),
            "HUNTING" => Some(g_HUNTING()),
            "IDEA_ASK" => Some(g_IDEA_ASK()),
            "IN_CARE" => Some(g_IN_CARE()),
            "IS_IT_TIME_REPLIES" => Some(g_IS_IT_TIME_REPLIES()),
            "IT_DIED" => Some(g_IT_DIED()),
            "KEEPING_GOING_REPLIES" => Some(g_KEEPING_GOING_REPLIES()),
            "KEEPING_JUDGED" => Some(g_KEEPING_JUDGED()),
            "KEEPS" => Some(g_KEEPS()),
            "KEEP_LOSING_THING" => Some(g_KEEP_LOSING_THING()),
            "KEPT_GOING_MISSING" => Some(g_KEPT_GOING_MISSING()),
            "KILLED" => Some(g_KILLED()),
            "KILLED_IN" => Some(g_KILLED_IN()),
            "KNOCKED_BY" => Some(g_KNOCKED_BY()),
            "LAST_DRUG" => Some(g_LAST_DRUG()),
            "LAST_STAGES" => Some(g_LAST_STAGES()),
            "LAUGHING" => Some(g_LAUGHING()),
            "LEFT_THING" => Some(g_LEFT_THING()),
            "LOCKED_OUT" => Some(g_LOCKED_OUT()),
            "LOSE_A_PERSON" => Some(g_LOSE_A_PERSON()),
            "LOSING" => Some(g_LOSING()),
            "LOSS_CONTEXT" => Some(g_LOSS_CONTEXT()),
            "LOSS_GRIEF_CUE" => Some(g_LOSS_GRIEF_CUE()),
            "LOSS_OF" => Some(g_LOSS_OF()),
            "LOSS_REPLIES" => Some(g_LOSS_REPLIES()),
            "LOSS_STILL_THE_THREAD" => Some(g_LOSS_STILL_THE_THREAD()),
            "LOST" => Some(g_LOST()),
            "LOST_BARE" => Some(g_LOST_BARE()),
            "LOST_FORMS" => Some(g_LOST_FORMS()),
            "LOST_IN_A_PLACE" => Some(g_LOST_IN_A_PLACE()),
            "LOST_SELF" => Some(g_LOST_SELF()),
            "LOST_THE" => Some(g_LOST_THE()),
            "LOST_THE_FOUND" => Some(g_LOST_THE_FOUND()),
            "LOST_THING_TO_FIND" => Some(g_LOST_THING_TO_FIND()),
            "MACHINE_ANIMALS" => Some(g_MACHINE_ANIMALS()),
            "MEASURE_TOPICS" => Some(g_MEASURE_TOPICS()),
            "MEDIA" => Some(g_MEDIA()),
            "MEDICATION_STOPPED" => Some(g_MEDICATION_STOPPED()),
            "MISADVENTURE" => Some(g_MISADVENTURE()),
            "MISLAID_REPLIES" => Some(g_MISLAID_REPLIES()),
            "MISSING_SAID_AGAIN" => Some(g_MISSING_SAID_AGAIN()),
            "MISS_ANIMALS" => Some(g_MISS_ANIMALS()),
            "MISS_ANIMAL_NOUNS" => Some(g_MISS_ANIMAL_NOUNS()),
            "MISS_NOT" => Some(g_MISS_NOT()),
            "MISS_NOT_BETWEEN" => Some(g_MISS_NOT_BETWEEN()),
            "MISS_NOUN" => Some(g_MISS_NOUN()),
            "MISS_PEOPLE" => Some(g_MISS_PEOPLE()),
            "MISS_PRONOUN" => Some(g_MISS_PRONOUN()),
            "MISS_REPLIES" => Some(g_MISS_REPLIES()),
            "MISS_SUBJECT" => Some(g_MISS_SUBJECT()),
            "MISS_TAIL" => Some(g_MISS_TAIL()),
            "MISS_VERB" => Some(g_MISS_VERB()),
            "MONTHS" => Some(g_MONTHS()),
            "NAMED_LOSS_SAY_REPLIES" => Some(g_NAMED_LOSS_SAY_REPLIES()),
            "NEWS_OF" => Some(g_NEWS_OF()),
            "NORMAL_TO_FEEL" => Some(g_NORMAL_TO_FEEL()),
            "NOTHING_MORE_TO_DO" => Some(g_NOTHING_MORE_TO_DO()),
            "NOT_A_BEDSIDE" => Some(g_NOT_A_BEDSIDE()),
            "NOT_A_CARE_GIVER" => Some(g_NOT_A_CARE_GIVER()),
            "NOT_A_DEATH" => Some(g_NOT_A_DEATH()),
            "NOT_A_DOOR" => Some(g_NOT_A_DOOR()),
            "NOT_A_THING_NEXT" => Some(g_NOT_A_THING_NEXT()),
            "NOT_GETTING_BETTER" => Some(g_NOT_GETTING_BETTER()),
            "NOT_MISLAID" => Some(g_NOT_MISLAID()),
            "NOT_THE_END" => Some(g_NOT_THE_END()),
            "NOT_WORKING" => Some(g_NOT_WORKING()),
            "NOT_YET" => Some(g_NOT_YET()),
            "OBJECTS" => Some(g_OBJECTS()),
            "OCCASION" => Some(g_OCCASION()),
            "OFF_LIFE_SUPPORT" => Some(g_OFF_LIFE_SUPPORT()),
            "OFF_SUPPORT_WELL" => Some(g_OFF_SUPPORT_WELL()),
            "ON_THE_LAST_DRUG" => Some(g_ON_THE_LAST_DRUG()),
            "OPEN_ASK" => Some(g_OPEN_ASK()),
            "OTHER_WEEPS" => Some(g_OTHER_WEEPS()),
            "OWNED" => Some(g_OWNED()),
            "OWN_DEATH" => Some(g_OWN_DEATH()),
            "OWN_PET" => Some(g_OWN_PET()),
            "OWN_STATE" => Some(g_OWN_STATE()),
            "OWN_WORRY" => Some(g_OWN_WORRY()),
            "PALLIATIVE_ITSELF" => Some(g_PALLIATIVE_ITSELF()),
            "PAPER_ASK" => Some(g_PAPER_ASK()),
            "PASSED_ALONE" => Some(g_PASSED_ALONE()),
            "PASSED_ALONE_HANDED" => Some(g_PASSED_ALONE_HANDED()),
            "PASSED_ALONE_NOUN" => Some(g_PASSED_ALONE_NOUN()),
            "PASSED_ALONE_TAIL" => Some(g_PASSED_ALONE_TAIL()),
            "PASSED_A_RESULT" => Some(g_PASSED_A_RESULT()),
            "PASSED_DATE" => Some(g_PASSED_DATE()),
            "PASSED_ON" => Some(g_PASSED_ON()),
            "PASSED_ON_OBJECT" => Some(g_PASSED_ON_OBJECT()),
            "PASSED_ON_SUBJECT" => Some(g_PASSED_ON_SUBJECT()),
            "PASSED_TIME" => Some(g_PASSED_TIME()),
            "PASSIVE_AUX" => Some(g_PASSIVE_AUX()),
            "PEACE_IS_MINE" => Some(g_PEACE_IS_MINE()),
            "PEOPLE" => Some(g_PEOPLE()),
            "PEOPLE_AFTER_A_GIVE_UP" => Some(g_PEOPLE_AFTER_A_GIVE_UP()),
            "PEOPLE_PHRASES" => Some(g_PEOPLE_PHRASES()),
            "PET" => Some(g_PET()),
            "PET_ARRANGEMENT" => Some(g_PET_ARRANGEMENT()),
            "PET_AT_THE_END" => Some(g_PET_AT_THE_END()),
            "PET_DEATH" => Some(g_PET_DEATH()),
            "PET_DIED" => Some(g_PET_DIED()),
            "PET_GRIEF_REPLIES" => Some(g_PET_GRIEF_REPLIES()),
            "PET_HAS_HAD_ENOUGH" => Some(g_PET_HAS_HAD_ENOUGH()),
            "PET_HAS_HAD_ENOUGH_AFTER" => Some(g_PET_HAS_HAD_ENOUGH_AFTER()),
            "PET_WORDS" => Some(g_PET_WORDS()),
            "PLACES" => Some(g_PLACES()),
            "PLAIN" => Some(g_PLAIN()),
            "PLURAL_PAIRS" => Some(g_PLURAL_PAIRS()),
            "POOLS" => Some(g_POOLS()),
            "PRACTICAL" => Some(g_PRACTICAL()),
            "PRACTICAL_ASK" => Some(g_PRACTICAL_ASK()),
            "PREPARE" => Some(g_PREPARE()),
            "PREPARE_OTHERWISE" => Some(g_PREPARE_OTHERWISE()),
            "PREY" => Some(g_PREY()),
            "PROCEEDINGS_ASK" => Some(g_PROCEEDINGS_ASK()),
            "PRONOUN" => Some(g_PRONOUN()),
            "PUTTING_REGRETTED" => Some(g_PUTTING_REGRETTED()),
            "PUT_DOWN" => Some(g_PUT_DOWN()),
            "PUT_DOWN_REGRET_REPLIES" => Some(g_PUT_DOWN_REGRET_REPLIES()),
            "QUANTITY" => Some(g_QUANTITY()),
            "QUESTION" => Some(g_QUESTION()),
            "QUIETER_ASK" => Some(g_QUIETER_ASK()),
            "REASSURING" => Some(g_REASSURING()),
            "REFERS_TO_THEM" => Some(g_REFERS_TO_THEM()),
            "RELATIVE_PET_REPLIES" => Some(g_RELATIVE_PET_REPLIES()),
            "REMAINS" => Some(g_REMAINS()),
            "REMEMBER_NOT_WHERE" => Some(g_REMEMBER_NOT_WHERE()),
            "REMEMBER_REPLIES" => Some(g_REMEMBER_REPLIES()),
            "RITUAL" => Some(g_RITUAL()),
            "RITUAL_PERSON" => Some(g_RITUAL_PERSON()),
            "ROLE_PHRASE" => Some(g_ROLE_PHRASE()),
            "SAFETY_MODULES" => Some(g_SAFETY_MODULES()),
            "SAID_IT" => Some(g_SAID_IT()),
            "SAY_GOODBYE" => Some(g_SAY_GOODBYE()),
            "SAY_REPLIES" => Some(g_SAY_REPLIES()),
            "SAY_THE_WORDS" => Some(g_SAY_THE_WORDS()),
            "SAY_TO_WHO" => Some(g_SAY_TO_WHO()),
            "SELF_NOUNS" => Some(g_SELF_NOUNS()),
            "SELF_ON_THE_DRUG" => Some(g_SELF_ON_THE_DRUG()),
            "SELF_OPENERS" => Some(g_SELF_OPENERS()),
            "SERIOUS_CARE" => Some(g_SERIOUS_CARE()),
            "SHED" => Some(g_SHED()),
            "SOMEONE_DYING" => Some(g_SOMEONE_DYING()),
            "SOMEONE_ELSES" => Some(g_SOMEONE_ELSES()),
            "SOMEONE_ELSES_OWN" => Some(g_SOMEONE_ELSES_OWN()),
            "SPEAKER_HAS_HAD_ENOUGH" => Some(g_SPEAKER_HAS_HAD_ENOUGH()),
            "SPEAKER_TOO" => Some(g_SPEAKER_TOO()),
            "SPEAKER_TOO_REPLIES" => Some(g_SPEAKER_TOO_REPLIES()),
            "STATE_TAIL" => Some(g_STATE_TAIL()),
            "STILL_ABOUT_THEM" => Some(g_STILL_ABOUT_THEM()),
            "STILL_AT_THE_DOOR" => Some(g_STILL_AT_THE_DOOR()),
            "STILL_CRY" => Some(g_STILL_CRY()),
            "STILL_CRYING" => Some(g_STILL_CRYING()),
            "STILL_DOING_FOR_THEM" => Some(g_STILL_DOING_FOR_THEM()),
            "STILL_KEEP" => Some(g_STILL_KEEP()),
            "STILL_LAYING" => Some(g_STILL_LAYING()),
            "STILL_SEEING" => Some(g_STILL_SEEING()),
            "STILL_WITHOUT" => Some(g_STILL_WITHOUT()),
            "STORY_INCIDENT" => Some(g_STORY_INCIDENT()),
            "STORY_MEDIUM" => Some(g_STORY_MEDIUM()),
            "STRANGER_ANIMAL_REPLIES" => Some(g_STRANGER_ANIMAL_REPLIES()),
            "SURVIVED" => Some(g_SURVIVED()),
            "SWAP_ASK" => Some(g_SWAP_ASK()),
            "SWAP_MOST_WORDS" => Some(g_SWAP_MOST_WORDS()),
            "SYMPATHY" => Some(g_SYMPATHY()),
            "TELL_CHILD" => Some(g_TELL_CHILD()),
            "THEIRS" => Some(g_THEIRS()),
            "THEIR_HARD_TIME_REPLIES" => Some(g_THEIR_HARD_TIME_REPLIES()),
            "THEIR_TEARS_IN_A_CLAUSE" => Some(g_THEIR_TEARS_IN_A_CLAUSE()),
            "THEM" => Some(g_THEM()),
            "THEMSELVES" => Some(g_THEMSELVES()),
            "THE_CANNOT_STOP_REPLIES" => Some(g_THE_CANNOT_STOP_REPLIES()),
            "THE_DAYS_BEFORE" => Some(g_THE_DAYS_BEFORE()),
            "THE_DEATH" => Some(g_THE_DEATH()),
            "THE_GIVE_UP_NAMES" => Some(g_THE_GIVE_UP_NAMES()),
            "THE_HABIT_REPLIES" => Some(g_THE_HABIT_REPLIES()),
            "THE_HOUSE_REPLIES" => Some(g_THE_HOUSE_REPLIES()),
            "THE_JOBS_FOR_THEM_REPLIES" => Some(g_THE_JOBS_FOR_THEM_REPLIES()),
            "THE_ROUTINE_REPLIES" => Some(g_THE_ROUTINE_REPLIES()),
            "THINGS" => Some(g_THINGS()),
            "THING_DEATH" => Some(g_THING_DEATH()),
            "THING_ROLE" => Some(g_THING_ROLE()),
            "THIS_ONE" => Some(g_THIS_ONE()),
            "THREAD_TOPICS" => Some(g_THREAD_TOPICS()),
            "THREAD_TURNS" => Some(g_THREAD_TURNS()),
            "TIME_OF_DAY" => Some(g_TIME_OF_DAY()),
            "TIME_SINCE_REPLIES" => Some(g_TIME_SINCE_REPLIES()),
            "TOLD_US" => Some(g_TOLD_US()),
            "TOOK" => Some(g_TOOK()),
            "TRADITION" => Some(g_TRADITION()),
            "TREATMENT_STOPPED" => Some(g_TREATMENT_STOPPED()),
            "TWO_WAY_ROLES" => Some(g_TWO_WAY_ROLES()),
            "TWO_WORD_PETS" => Some(g_TWO_WORD_PETS()),
            "UNNAMED" => Some(g_UNNAMED()),
            "VERB_OBJECT" => Some(g_VERB_OBJECT()),
            "VISIT_TARGETS" => Some(g_VISIT_TARGETS()),
            "WAKE" => Some(g_WAKE()),
            "WEAR" => Some(g_WEAR()),
            "WHEN_ASK" => Some(g_WHEN_ASK()),
            "WHEN_WORDS" => Some(g_WHEN_WORDS()),
            "WHERE_IS_THING" => Some(g_WHERE_IS_THING()),
            "WHERE_IS_THING_BARE" => Some(g_WHERE_IS_THING_BARE()),
            "WHERE_I_PUT_IT" => Some(g_WHERE_I_PUT_IT()),
            "WISH_FRAME" => Some(g_WISH_FRAME()),
            "WITHOUT" => Some(g_WITHOUT()),
            "WOULD_HAVE" => Some(g_WOULD_HAVE()),
            "WOULD_HAVE_BEEN" => Some(g_WOULD_HAVE_BEEN()),
            "WRITE_WORD" => Some(g_WRITE_WORD()),
            "YEAR_OLD" => Some(g_YEAR_OLD()),
            "_A_DAY_PART" => Some(g__A_DAY_PART()),
            "_BILL_MONEY" => Some(g__BILL_MONEY()),
            "_BILL_SUBJECT" => Some(g__BILL_SUBJECT()),
            "_BURIAL_SIDE" => Some(g__BURIAL_SIDE()),
            "_CAN_BE_DYING" => Some(g__CAN_BE_DYING()),
            "_CONTRACTED" => Some(g__CONTRACTED()),
            "_COST_NOUN" => Some(g__COST_NOUN()),
            "_CREMATION_SIDE" => Some(g__CREMATION_SIDE()),
            "_DEATH_IN_LINE" => Some(g__DEATH_IN_LINE()),
            "_FUNERAL_NOUN" => Some(g__FUNERAL_NOUN()),
            "_HELPERS" => Some(g__HELPERS()),
            "_NOT_A_BILL_THREAD" => Some(g__NOT_A_BILL_THREAD()),
            "_NOT_A_CONTRACTION" => Some(g__NOT_A_CONTRACTION()),
            "_PEOPLE_PRONOUNS" => Some(g__PEOPLE_PRONOUNS()),
            "_PRONOUN_MONEY_ASK" => Some(g__PRONOUN_MONEY_ASK()),
            "_SAID_DYING" => Some(g__SAID_DYING()),
            "_SPAN_COUNT" => Some(g__SPAN_COUNT()),
            "_THE_ESTATE" => Some(g__THE_ESTATE()),
            "_THE_ONE_WHO_LEFT" => Some(g__THE_ONE_WHO_LEFT()),
            "_WORKING_WITH_THE_DYING" => Some(g__WORKING_WITH_THE_DYING()),
            "_a_body_with_its_own_answer" => Some(py_func("_a_body_with_its_own_answer", |args: &[Py]| _a_body_with_its_own_answer(args))),
            "_a_colour_is_worn" => Some(py_func("_a_colour_is_worn", |args: &[Py]| _a_colour_is_worn(args))),
            "_a_gift_for_someone_in_care" => Some(py_func("_a_gift_for_someone_in_care", |args: &[Py]| _a_gift_for_someone_in_care(args))),
            "_a_habit_in" => Some(py_func("_a_habit_in", |args: &[Py]| _a_habit_in(args))),
            "_a_hard_time_that_is_theirs" => Some(py_func("_a_hard_time_that_is_theirs", |args: &[Py]| _a_hard_time_that_is_theirs(args))),
            "_a_person_dying" => Some(py_func("_a_person_dying", |args: &[Py]| _a_person_dying(args))),
            "_a_pet_of_ours" => Some(py_func("_a_pet_of_ours", |args: &[Py]| _a_pet_of_ours(args))),
            "_a_pronoun_is_what_died" => Some(py_func("_a_pronoun_is_what_died", |args: &[Py]| _a_pronoun_is_what_died(args))),
            "_a_question_about_the_feeling" => Some(py_func("_a_question_about_the_feeling", |args: &[Py]| _a_question_about_the_feeling(args))),
            "_a_role_phrase" => Some(py_func("_a_role_phrase", |args: &[Py]| _a_role_phrase(args))),
            "_a_swap_of_the_question_before" => Some(py_func("_a_swap_of_the_question_before", |args: &[Py]| _a_swap_of_the_question_before(args))),
            "_a_thing_of_its_own_to_get_through" => Some(py_func("_a_thing_of_its_own_to_get_through", |args: &[Py]| _a_thing_of_its_own_to_get_through(args))),
            "_a_thing_of_theirs" => Some(py_func("_a_thing_of_theirs", |args: &[Py]| _a_thing_of_theirs(args))),
            "_about_family" => Some(py_func("_about_family", |args: &[Py]| _about_family(args))),
            "_about_them" => Some(py_func("_about_them", |args: &[Py]| _about_them(args))),
            "_accident_death" => Some(py_func("_accident_death", |args: &[Py]| _accident_death(args))),
            "_an_animal_in_the_chat" => Some(py_func("_an_animal_in_the_chat", |args: &[Py]| _an_animal_in_the_chat(args))),
            "_an_ask_in_the_line" => Some(py_func("_an_ask_in_the_line", |args: &[Py]| _an_ask_in_the_line(args))),
            "_an_ending_in_the_chat" => Some(py_func("_an_ending_in_the_chat", |args: &[Py]| _an_ending_in_the_chat(args))),
            "_another_person" => Some(py_func("_another_person", |args: &[Py]| _another_person(args))),
            "_asks_about_the_feeling" => Some(py_func("_asks_about_the_feeling", |args: &[Py]| _asks_about_the_feeling(args))),
            "_brought_in_prey" => Some(py_func("_brought_in_prey", |args: &[Py]| _brought_in_prey(args))),
            "_buried_person" => Some(py_func("_buried_person", |args: &[Py]| _buried_person(args))),
            "_can_die" => Some(py_func("_can_die", |args: &[Py]| _can_die(args))),
            "_dead_is_not_a_death" => Some(py_func("_dead_is_not_a_death", |args: &[Py]| _dead_is_not_a_death(args))),
            "_death_noun" => Some(py_func("_death_noun", |args: &[Py]| _death_noun(args))),
            "_dream" => Some(g__dream()),
            "_feared_not_done" => Some(py_func("_feared_not_done", |args: &[Py]| _feared_not_done(args))),
            "_feeling" => Some(py_func("_feeling", |args: &[Py]| _feeling(args))),
            "_first_days" => Some(py_func("_first_days", |args: &[Py]| _first_days(args))),
            "_grief_replies" => Some(py_func("_grief_replies", |args: &[Py]| _grief_replies(args))),
            "_hands_over_a_detail" => Some(py_func("_hands_over_a_detail", |args: &[Py]| _hands_over_a_detail(args))),
            "_has_person" => Some(py_func("_has_person", |args: &[Py]| _has_person(args))),
            "_has_pet" => Some(py_func("_has_pet", |args: &[Py]| _has_pet(args))),
            "_how_long_since" => Some(py_func("_how_long_since", |args: &[Py]| _how_long_since(args))),
            "_keeping_going" => Some(py_func("_keeping_going", |args: &[Py]| _keeping_going(args))),
            "_killed_in_a_story" => Some(py_func("_killed_in_a_story", |args: &[Py]| _killed_in_a_story(args))),
            "_less_what_was_said" => Some(py_func("_less_what_was_said", |args: &[Py]| _less_what_was_said(args))),
            "_loss_is_the_thread" => Some(py_func("_loss_is_the_thread", |args: &[Py]| _loss_is_the_thread(args))),
            "_lost_in_a_place_after" => Some(py_func("_lost_in_a_place_after", |args: &[Py]| _lost_in_a_place_after(args))),
            "_missing_someone" => Some(py_func("_missing_someone", |args: &[Py]| _missing_someone(args))),
            "_names_a_stored_pet" => Some(py_func("_names_a_stored_pet", |args: &[Py]| _names_a_stored_pet(args))),
            "_names_a_subject" => Some(py_func("_names_a_subject", |args: &[Py]| _names_a_subject(args))),
            "_object_died" => Some(py_func("_object_died", |args: &[Py]| _object_died(args))),
            "_only_a_new_subject" => Some(py_func("_only_a_new_subject", |args: &[Py]| _only_a_new_subject(args))),
            "_only_the_proceedings" => Some(py_func("_only_the_proceedings", |args: &[Py]| _only_the_proceedings(args))),
            "_own_state" => Some(py_func("_own_state", |args: &[Py]| _own_state(args))),
            "_own_subject" => Some(py_func("_own_subject", |args: &[Py]| _own_subject(args))),
            "_owned_deathable" => Some(py_func("_owned_deathable", |args: &[Py]| _owned_deathable(args))),
            "_passed_alone" => Some(py_func("_passed_alone", |args: &[Py]| _passed_alone(args))),
            "_passed_on" => Some(py_func("_passed_on", |args: &[Py]| _passed_on(args))),
            "_plain" => Some(py_func("_plain", |args: &[Py]| _plain(args))),
            "_put_down" => Some(py_func("_put_down", |args: &[Py]| _put_down(args))),
            "_readable" => Some(py_func("_readable", |args: &[Py]| _readable(args))),
            "_shed_after" => Some(py_func("_shed_after", |args: &[Py]| _shed_after(args))),
            "_spoken" => Some(py_func("_spoken", |args: &[Py]| _spoken(args))),
            "_state_describes_them" => Some(py_func("_state_describes_them", |args: &[Py]| _state_describes_them(args))),
            "_still_ahead" => Some(py_func("_still_ahead", |args: &[Py]| _still_ahead(args))),
            "_still_here" => Some(py_func("_still_here", |args: &[Py]| _still_here(args))),
            "_telling_a_child" => Some(py_func("_telling_a_child", |args: &[Py]| _telling_a_child(args))),
            "_the_animal_of" => Some(py_func("_the_animal_of", |args: &[Py]| _the_animal_of(args))),
            "_the_bill_was_the_last_turn" => Some(py_func("_the_bill_was_the_last_turn", |args: &[Py]| _the_bill_was_the_last_turn(args))),
            "_the_body_is_theirs" => Some(py_func("_the_body_is_theirs", |args: &[Py]| _the_body_is_theirs(args))),
            "_the_end_is_near" => Some(py_func("_the_end_is_near", |args: &[Py]| _the_end_is_near(args))),
            "_the_family_is_told" => Some(py_func("_the_family_is_told", |args: &[Py]| _the_family_is_told(args))),
            "_the_give_up_is_about_somebody" => Some(py_func("_the_give_up_is_about_somebody", |args: &[Py]| _the_give_up_is_about_somebody(args))),
            "_the_give_up_without_a_care_word" => Some(py_func("_the_give_up_without_a_care_word", |args: &[Py]| _the_give_up_without_a_care_word(args))),
            "_the_vet_said_it" => Some(py_func("_the_vet_said_it", |args: &[Py]| _the_vet_said_it(args))),
            "_the_visit_ask" => Some(py_func("_the_visit_ask", |args: &[Py]| _the_visit_ask(args))),
            "_their_thing_after" => Some(py_func("_their_thing_after", |args: &[Py]| _their_thing_after(args))),
            "_thread_pool" => Some(py_func("_thread_pool", |args: &[Py]| _thread_pool(args))),
            "_tight" => Some(py_func("_tight", |args: &[Py]| _tight(args))),
            "_unnamed_person" => Some(py_func("_unnamed_person", |args: &[Py]| _unnamed_person(args))),
            "_when_asked" => Some(py_func("_when_asked", |args: &[Py]| _when_asked(args))),
            "_where_i_put_it" => Some(py_func("_where_i_put_it", |args: &[Py]| _where_i_put_it(args))),
            "a_part_of_the_day_asked" => Some(py_func("a_part_of_the_day_asked", |args: &[Py]| a_part_of_the_day_asked(args))),
            "a_pet_died" => Some(py_func("a_pet_died", |args: &[Py]| a_pet_died(args))),
            "a_relative_pet" => Some(py_func("a_relative_pet", |args: &[Py]| a_relative_pet(args))),
            "a_strangers_animal" => Some(py_func("a_strangers_animal", |args: &[Py]| a_strangers_animal(args))),
            "a_thing_stopped" => Some(py_func("a_thing_stopped", |args: &[Py]| a_thing_stopped(args))),
            "acknowledges" => Some(py_func("acknowledges", |args: &[Py]| acknowledges(args))),
            "administration_keys" => Some(py_func("administration_keys", |args: &[Py]| administration_keys(args))),
            "arrangement_cost_pool" => Some(py_func("arrangement_cost_pool", |args: &[Py]| arrangement_cost_pool(args))),
            "asks_about_the_remains" => Some(py_func("asks_about_the_remains", |args: &[Py]| asks_about_the_remains(args))),
            "asks_anything" => Some(py_func("asks_anything", |args: &[Py]| asks_anything(args))),
            "asks_for_a_decision" => Some(py_func("asks_for_a_decision", |args: &[Py]| asks_for_a_decision(args))),
            "asks_the_administration" => Some(py_func("asks_the_administration", |args: &[Py]| asks_the_administration(args))),
            "asks_what_to_do" => Some(py_func("asks_what_to_do", |args: &[Py]| asks_what_to_do(args))),
            "continues" => Some(py_func("continues", |args: &[Py]| continues(args))),
            "dying_in_the_chat" => Some(py_func("dying_in_the_chat", |args: &[Py]| dying_in_the_chat(args))),
            "feeling_about_the_body" => Some(py_func("feeling_about_the_body", |args: &[Py]| feeling_about_the_body(args))),
            "found_what_was_lost" => Some(py_func("found_what_was_lost", |args: &[Py]| found_what_was_lost(args))),
            "is_about_the_paperwork" => Some(py_func("is_about_the_paperwork", |args: &[Py]| is_about_the_paperwork(args))),
            "is_dying_now" => Some(py_func("is_dying_now", |args: &[Py]| is_dying_now(args))),
            "is_practical_ask" => Some(py_func("is_practical_ask", |args: &[Py]| is_practical_ask(args))),
            "keeps_losing_thing" => Some(py_func("keeps_losing_thing", |args: &[Py]| keeps_losing_thing(args))),
            "locked_out" => Some(py_func("locked_out", |args: &[Py]| locked_out(args))),
            "locked_out_of_the_car" => Some(py_func("locked_out_of_the_car", |args: &[Py]| locked_out_of_the_car(args))),
            "lost_a_thing" => Some(py_func("lost_a_thing", |args: &[Py]| lost_a_thing(args))),
            "lost_in_a_place" => Some(py_func("lost_in_a_place", |args: &[Py]| lost_in_a_place(args))),
            "lost_thing_to_find" => Some(py_func("lost_thing_to_find", |args: &[Py]| lost_thing_to_find(args))),
            "marks_a_day" => Some(py_func("marks_a_day", |args: &[Py]| marks_a_day(args))),
            "named_animal" => Some(py_func("named_animal", |args: &[Py]| named_animal(args))),
            "names_a_feeling" => Some(py_func("names_a_feeling", |args: &[Py]| names_a_feeling(args))),
            "one_of" => Some(py_func("one_of", |args: &[Py]| one_of(args))),
            "reads_as_loss" => Some(py_func("reads_as_loss", |args: &[Py]| reads_as_loss(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "says_the_last_drug" => Some(py_func("says_the_last_drug", |args: &[Py]| says_the_last_drug(args))),
            "the_animal" => Some(py_func("the_animal", |args: &[Py]| the_animal(args))),
            "whose" => Some(py_func("whose", |args: &[Py]| whose(args))),
            "words_of" => Some(py_func("words_of", |args: &[Py]| words_of(args))),
            _ => None,
        }),
        set: None,
    }))
}
