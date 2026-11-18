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


pub static AFTER_A_COLOUR_STANDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AN_ILLNESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static APPLIANCE_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKING_FIRST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKING_MODALS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKING_SUBJECTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_FOR_THE_HANDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_WHAT_IT_SAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_FRAME_IDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_FRAME_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_NAME_CAP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_PEOPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_PERSON_POSSESSIVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_PERSON_PRONOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_PHRASE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_PLAIN_CAP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AS_A_KIND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CHILD_GETTING_OLDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CHILD_WORD_BY_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COLOUR_ASKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COLOUR_USED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DEVICE_THAT_SLEEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DRESS_CODE_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_GOOD_SMELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HAND_OR_EYE_DOES_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HOURS_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HOUSE_WARMING_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LENGTH_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SAYS_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STAND_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STATE_AS_A_NAME_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STATE_IN_THE_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WEATHER_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WHAT_TO_WEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BACK_OF_THE_HOUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_WEIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BEFORE_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BODY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARER_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_AFTER_A_SIZE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_BY_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_PLACE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILD_SIZE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COLOUR_ASKED_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COLOUR_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMMON_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTAINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTEXT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COOKING_AS_A_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNTED_INTO_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DEAD_AS_VERY_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DENIERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DENY_BETWEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DENY_SAID_OF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DISH_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENDINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVERYDAY_CHILD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPLAIN_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPLAIN_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEARED_THINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_ABOUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_ABOUT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_DOMAINS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_ENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_FAULT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_NAMES_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_SPEAKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FEELING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOLLOW_ON_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOOD_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOOD_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_ENDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_PRONOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FRAME_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GENERAL_BODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GOES_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GONE_INTENSIFIER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GONE_OFF_SIGNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GONE_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GRAMMAR_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HANDLING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAPPENING_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAS_GONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOUSEHOLD_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MANY_DAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_OFTEN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IMPROVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IRREGULAR_PLURALS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static JOB_GUARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_OR_STORES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEPT_FOODS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS_ASKS_ME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS_ASKS_WITH_A_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS_FOLLOW_UP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS_NAMED_A_USE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS_SAYS_HOW_IT_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MACHINE_SOUNDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MIN_FORM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MIN_SCORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MIN_TYPO_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISSING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MODAL_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAME_ALONE_DOMAINS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_BASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_CHILD_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_NAME_ALONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_NOSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_WORD_WITH_AN_S: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_ADJECTIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_DOUBLED_STEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_STEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_STEM_WITH_E: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_TIRED_AFTER_ALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OCCASION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OFF_SAID_OF_FOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OFF_STATE_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ORIGIN_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_THING_ON_THEIR_SIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PACK_DATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PART_INSIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE_FEMALE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE_MALE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_PRONOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PHRASE_WEIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACES_AFTER_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAIN_MARGIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLURAL_ONLY_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRESENT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_COMPARED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPORT_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPORT_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPORT_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RETURNED_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RING_AS_A_CALL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SANDING_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT_OF_SLEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLIPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SMELL_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPEAKER_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPECIFIC: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPECIFIC_PARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATES_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOPPED_WORKING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBSTITUTE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN_HANDS_THINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_BACK_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_FAULT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_SAME_OCCASION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOOTH_OR_GUM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOOTH_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TWO_OF_US: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANT_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WANT_KNOWING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEARING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEIGHT_UNITS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_PART_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD_WEIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORN_AFTER_A_COLOUR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ABOUT_THE_CHAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AFFORD_THE_TRIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ARTICLE_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ARTICLE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_ABOUT_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_FRAME_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ASK_PHRASE_MASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BAKE_OR_A_PAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_BREATH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_KEEP_AND_A_LENGTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PERSON_WHO_SLEEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PLACE_ON_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_THING_JUST_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_THING_POINTED_AT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_THING_SAID_IS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_YEARS_OLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BAD_IS_A_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BARE_IN_A_KEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BEFORE_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BETWEEN_A_PLACE_AND_ITS_COLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BE_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BORROWING_IN_THE_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BOUNDED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BOUNDED_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BRIDGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BRIDGES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CACHE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CACHE_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CAME_OUT_FLAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CHAT_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CHAT_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CHILDS_AGE_ROWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CHILD_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CHILD_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COLD_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COMMON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CONTRACTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CURLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CURLY_ANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CUT_BESIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEFINING_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEFINITION_ARTICLES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEFINITION_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEFINITION_EMPTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEFINITION_POINTERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEFINITION_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DEFINITION_TROUBLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DREAM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FLAT_YOU_LIVE_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FORMS_CACHE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FORMS_CACHE_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FRAME_MASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FRAME_PRONOUN_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FRAME_STARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FUNCTION_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HOLDS_MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HOW_A_PERSON_IS_COLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _INDEX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ING_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEEPING_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEEPING_AN_EYE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEEPING_KEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEEPING_ROWS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEEPING_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEEPING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEPT_SOMEWHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _KEY_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LIFE_KEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LINE_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MARKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MIN_BARE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_THING_HAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_GONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OFF_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OF_THE_RULE_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ONE_SPELLING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ONE_SPELLING_EITHER_WAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ORDER_OF_STEPS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OTHER_SPELLINGS_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PART_FORMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PART_FORMS_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PEOPLE_AGED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PHRASE_OTHER_SPELLINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PHRASE_SPANS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PLACES_PEOPLE_LIVE_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POINTS_AT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POSSESSIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PUNCTUATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RECURRING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RECURRING_ANYWAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RECURRING_WITH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _RESULT_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ROWS_CACHE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ROWS_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAID_BETWEEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAVING_FOR_A_TRIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAVING_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAYS_OF_THEMSELVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SHAPE_CACHE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SLEEPY_MARKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SPOILING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STEMS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _STILL_GOOD_ASHERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SUBJECT_AT_MOST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SUBJECT_LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SUBJECT_POINTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TAIL_ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TAKE_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_DEGREE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_GROUND_UNDERFOOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_HEAT_OF_AN_ARGUMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_HEAT_OF_THE_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_PRONOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_THING_JUDGED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_WH_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THROAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WANTED_CACHE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WANTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WEAR_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WHERE_I_PUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WHERE_TO_KEEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WRAPPED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _YEAR_OLD_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _long: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _short: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AFTER_A_COLOUR_STANDS() -> Py { ensure_init(); AFTER_A_COLOUR_STANDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_A_COLOUR_STANDS(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_A_COLOUR_STANDS.write() { *w = v; } }
pub fn g_AN_ILLNESS() -> Py { ensure_init(); AN_ILLNESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_ILLNESS(v: Py) { ensure_init(); if let Ok(mut w) = AN_ILLNESS.write() { *w = v; } }
pub fn g_APPLIANCE_NAMES() -> Py { ensure_init(); APPLIANCE_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_APPLIANCE_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = APPLIANCE_NAMES.write() { *w = v; } }
pub fn g_ASKING_FIRST() -> Py { ensure_init(); ASKING_FIRST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING_FIRST(v: Py) { ensure_init(); if let Ok(mut w) = ASKING_FIRST.write() { *w = v; } }
pub fn g_ASKING_MODALS() -> Py { ensure_init(); ASKING_MODALS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING_MODALS(v: Py) { ensure_init(); if let Ok(mut w) = ASKING_MODALS.write() { *w = v; } }
pub fn g_ASKING_SUBJECTS() -> Py { ensure_init(); ASKING_SUBJECTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKING_SUBJECTS(v: Py) { ensure_init(); if let Ok(mut w) = ASKING_SUBJECTS.write() { *w = v; } }
pub fn g_ASKS_FOR_THE_HANDS() -> Py { ensure_init(); ASKS_FOR_THE_HANDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_THE_HANDS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_FOR_THE_HANDS.write() { *w = v; } }
pub fn g_ASKS_WHAT_IT_SAYS() -> Py { ensure_init(); ASKS_WHAT_IT_SAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHAT_IT_SAYS(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_WHAT_IT_SAYS.write() { *w = v; } }
pub fn g_ASK_ABOUT() -> Py { ensure_init(); ASK_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = ASK_ABOUT.write() { *w = v; } }
pub fn g_ASK_FRAME() -> Py { ensure_init(); ASK_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = ASK_FRAME.write() { *w = v; } }
pub fn g_ASK_FRAME_IDS() -> Py { ensure_init(); ASK_FRAME_IDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_FRAME_IDS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_FRAME_IDS.write() { *w = v; } }
pub fn g_ASK_FRAME_WORDS() -> Py { ensure_init(); ASK_FRAME_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_FRAME_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_FRAME_WORDS.write() { *w = v; } }
pub fn g_ASK_NAME_CAP() -> Py { ensure_init(); ASK_NAME_CAP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_NAME_CAP(v: Py) { ensure_init(); if let Ok(mut w) = ASK_NAME_CAP.write() { *w = v; } }
pub fn g_ASK_PEOPLE() -> Py { ensure_init(); ASK_PEOPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_PEOPLE(v: Py) { ensure_init(); if let Ok(mut w) = ASK_PEOPLE.write() { *w = v; } }
pub fn g_ASK_PERSON_POSSESSIVES() -> Py { ensure_init(); ASK_PERSON_POSSESSIVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_PERSON_POSSESSIVES(v: Py) { ensure_init(); if let Ok(mut w) = ASK_PERSON_POSSESSIVES.write() { *w = v; } }
pub fn g_ASK_PERSON_PRONOUNS() -> Py { ensure_init(); ASK_PERSON_PRONOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_PERSON_PRONOUNS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_PERSON_PRONOUNS.write() { *w = v; } }
pub fn g_ASK_PHRASE_WORDS() -> Py { ensure_init(); ASK_PHRASE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_PHRASE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_PHRASE_WORDS.write() { *w = v; } }
pub fn g_ASK_PLAIN_CAP() -> Py { ensure_init(); ASK_PLAIN_CAP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_PLAIN_CAP(v: Py) { ensure_init(); if let Ok(mut w) = ASK_PLAIN_CAP.write() { *w = v; } }
pub fn g_AS_A_KIND() -> Py { ensure_init(); AS_A_KIND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AS_A_KIND(v: Py) { ensure_init(); if let Ok(mut w) = AS_A_KIND.write() { *w = v; } }
pub fn g_A_CHILD_GETTING_OLDER() -> Py { ensure_init(); A_CHILD_GETTING_OLDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_GETTING_OLDER(v: Py) { ensure_init(); if let Ok(mut w) = A_CHILD_GETTING_OLDER.write() { *w = v; } }
pub fn g_A_CHILD_WORD_BY_ITSELF() -> Py { ensure_init(); A_CHILD_WORD_BY_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_WORD_BY_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = A_CHILD_WORD_BY_ITSELF.write() { *w = v; } }
pub fn g_A_COLOUR_ASKED() -> Py { ensure_init(); A_COLOUR_ASKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COLOUR_ASKED(v: Py) { ensure_init(); if let Ok(mut w) = A_COLOUR_ASKED.write() { *w = v; } }
pub fn g_A_COLOUR_USED() -> Py { ensure_init(); A_COLOUR_USED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COLOUR_USED(v: Py) { ensure_init(); if let Ok(mut w) = A_COLOUR_USED.write() { *w = v; } }
pub fn g_A_DEVICE_THAT_SLEEPS() -> Py { ensure_init(); A_DEVICE_THAT_SLEEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DEVICE_THAT_SLEEPS(v: Py) { ensure_init(); if let Ok(mut w) = A_DEVICE_THAT_SLEEPS.write() { *w = v; } }
pub fn g_A_DRESS_CODE_AFTER() -> Py { ensure_init(); A_DRESS_CODE_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DRESS_CODE_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = A_DRESS_CODE_AFTER.write() { *w = v; } }
pub fn g_A_GOOD_SMELL() -> Py { ensure_init(); A_GOOD_SMELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_GOOD_SMELL(v: Py) { ensure_init(); if let Ok(mut w) = A_GOOD_SMELL.write() { *w = v; } }
pub fn g_A_HAND_OR_EYE_DOES_IT() -> Py { ensure_init(); A_HAND_OR_EYE_DOES_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HAND_OR_EYE_DOES_IT(v: Py) { ensure_init(); if let Ok(mut w) = A_HAND_OR_EYE_DOES_IT.write() { *w = v; } }
pub fn g_A_HOURS_ASK() -> Py { ensure_init(); A_HOURS_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HOURS_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_HOURS_ASK.write() { *w = v; } }
pub fn g_A_HOUSE_WARMING_UP() -> Py { ensure_init(); A_HOUSE_WARMING_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HOUSE_WARMING_UP(v: Py) { ensure_init(); if let Ok(mut w) = A_HOUSE_WARMING_UP.write() { *w = v; } }
pub fn g_A_LENGTH_WORD() -> Py { ensure_init(); A_LENGTH_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_LENGTH_WORD.write() { *w = v; } }
pub fn g_A_SAYS_NOTHING() -> Py { ensure_init(); A_SAYS_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SAYS_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = A_SAYS_NOTHING.write() { *w = v; } }
pub fn g_A_STAND_IN() -> Py { ensure_init(); A_STAND_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STAND_IN(v: Py) { ensure_init(); if let Ok(mut w) = A_STAND_IN.write() { *w = v; } }
pub fn g_A_STATE_AS_A_NAME_END() -> Py { ensure_init(); A_STATE_AS_A_NAME_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STATE_AS_A_NAME_END(v: Py) { ensure_init(); if let Ok(mut w) = A_STATE_AS_A_NAME_END.write() { *w = v; } }
pub fn g_A_STATE_IN_THE_PHRASE() -> Py { ensure_init(); A_STATE_IN_THE_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STATE_IN_THE_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = A_STATE_IN_THE_PHRASE.write() { *w = v; } }
pub fn g_A_TEEN() -> Py { ensure_init(); A_TEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TEEN(v: Py) { ensure_init(); if let Ok(mut w) = A_TEEN.write() { *w = v; } }
pub fn g_A_WEATHER_ASK() -> Py { ensure_init(); A_WEATHER_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WEATHER_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_WEATHER_ASK.write() { *w = v; } }
pub fn g_A_WHAT_TO_WEAR() -> Py { ensure_init(); A_WHAT_TO_WEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WHAT_TO_WEAR(v: Py) { ensure_init(); if let Ok(mut w) = A_WHAT_TO_WEAR.write() { *w = v; } }
pub fn g_BACK_OF_THE_HOUSE() -> Py { ensure_init(); BACK_OF_THE_HOUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BACK_OF_THE_HOUSE(v: Py) { ensure_init(); if let Ok(mut w) = BACK_OF_THE_HOUSE.write() { *w = v; } }
pub fn g_BARE_WEIGHT() -> Py { ensure_init(); BARE_WEIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_WEIGHT(v: Py) { ensure_init(); if let Ok(mut w) = BARE_WEIGHT.write() { *w = v; } }
pub fn g_BEFORE_OFF() -> Py { ensure_init(); BEFORE_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEFORE_OFF(v: Py) { ensure_init(); if let Ok(mut w) = BEFORE_OFF.write() { *w = v; } }
pub fn g_BODY_WORDS() -> Py { ensure_init(); BODY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BODY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = BODY_WORDS.write() { *w = v; } }
pub fn g_CARER_FRAME() -> Py { ensure_init(); CARER_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARER_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = CARER_FRAME.write() { *w = v; } }
pub fn g_CHILD_AFTER_A_SIZE() -> Py { ensure_init(); CHILD_AFTER_A_SIZE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_AFTER_A_SIZE(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_AFTER_A_SIZE.write() { *w = v; } }
pub fn g_CHILD_BY_PLACE() -> Py { ensure_init(); CHILD_BY_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_BY_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_BY_PLACE.write() { *w = v; } }
pub fn g_CHILD_PLACE_WORDS() -> Py { ensure_init(); CHILD_PLACE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_PLACE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_PLACE_WORDS.write() { *w = v; } }
pub fn g_CHILD_SIZE_WORDS() -> Py { ensure_init(); CHILD_SIZE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILD_SIZE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = CHILD_SIZE_WORDS.write() { *w = v; } }
pub fn g_COLOUR_ASKED_WORDS() -> Py { ensure_init(); COLOUR_ASKED_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COLOUR_ASKED_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = COLOUR_ASKED_WORDS.write() { *w = v; } }
pub fn g_COLOUR_WORDS() -> Py { ensure_init(); COLOUR_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COLOUR_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = COLOUR_WORDS.write() { *w = v; } }
pub fn g_COMMON_WORD() -> Py { ensure_init(); COMMON_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMMON_WORD(v: Py) { ensure_init(); if let Ok(mut w) = COMMON_WORD.write() { *w = v; } }
pub fn g_CONTAINERS() -> Py { ensure_init(); CONTAINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTAINERS(v: Py) { ensure_init(); if let Ok(mut w) = CONTAINERS.write() { *w = v; } }
pub fn g_CONTEXT_WORDS() -> Py { ensure_init(); CONTEXT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTEXT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = CONTEXT_WORDS.write() { *w = v; } }
pub fn g_COOKING_AS_A_NAME() -> Py { ensure_init(); COOKING_AS_A_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COOKING_AS_A_NAME(v: Py) { ensure_init(); if let Ok(mut w) = COOKING_AS_A_NAME.write() { *w = v; } }
pub fn g_COUNTED_INTO_A_THING() -> Py { ensure_init(); COUNTED_INTO_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNTED_INTO_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = COUNTED_INTO_A_THING.write() { *w = v; } }
pub fn g_DAY_WORDS() -> Py { ensure_init(); DAY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DAY_WORDS.write() { *w = v; } }
pub fn g_DEAD_AS_VERY_AFTER() -> Py { ensure_init(); DEAD_AS_VERY_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DEAD_AS_VERY_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = DEAD_AS_VERY_AFTER.write() { *w = v; } }
pub fn g_DENIERS() -> Py { ensure_init(); DENIERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DENIERS(v: Py) { ensure_init(); if let Ok(mut w) = DENIERS.write() { *w = v; } }
pub fn g_DENY_BETWEEN() -> Py { ensure_init(); DENY_BETWEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DENY_BETWEEN(v: Py) { ensure_init(); if let Ok(mut w) = DENY_BETWEEN.write() { *w = v; } }
pub fn g_DENY_SAID_OF() -> Py { ensure_init(); DENY_SAID_OF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DENY_SAID_OF(v: Py) { ensure_init(); if let Ok(mut w) = DENY_SAID_OF.write() { *w = v; } }
pub fn g_DISH_ASK() -> Py { ensure_init(); DISH_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DISH_ASK(v: Py) { ensure_init(); if let Ok(mut w) = DISH_ASK.write() { *w = v; } }
pub fn g_ENDINGS() -> Py { ensure_init(); ENDINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENDINGS(v: Py) { ensure_init(); if let Ok(mut w) = ENDINGS.write() { *w = v; } }
pub fn g_EVERYDAY_CHILD() -> Py { ensure_init(); EVERYDAY_CHILD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVERYDAY_CHILD(v: Py) { ensure_init(); if let Ok(mut w) = EVERYDAY_CHILD.write() { *w = v; } }
pub fn g_EXPLAIN_FILLER() -> Py { ensure_init(); EXPLAIN_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPLAIN_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = EXPLAIN_FILLER.write() { *w = v; } }
pub fn g_EXPLAIN_LEADS() -> Py { ensure_init(); EXPLAIN_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPLAIN_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = EXPLAIN_LEADS.write() { *w = v; } }
pub fn g_FEARED_THINGS() -> Py { ensure_init(); FEARED_THINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEARED_THINGS(v: Py) { ensure_init(); if let Ok(mut w) = FEARED_THINGS.write() { *w = v; } }
pub fn g_FEELING_ABOUT() -> Py { ensure_init(); FEELING_ABOUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_ABOUT(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_ABOUT.write() { *w = v; } }
pub fn g_FEELING_ABOUT_A_THING() -> Py { ensure_init(); FEELING_ABOUT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_ABOUT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_ABOUT_A_THING.write() { *w = v; } }
pub fn g_FEELING_DOMAINS() -> Py { ensure_init(); FEELING_DOMAINS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_DOMAINS(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_DOMAINS.write() { *w = v; } }
pub fn g_FEELING_ENDS() -> Py { ensure_init(); FEELING_ENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_ENDS(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_ENDS.write() { *w = v; } }
pub fn g_FEELING_FAULT_WORDS() -> Py { ensure_init(); FEELING_FAULT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_FAULT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_FAULT_WORDS.write() { *w = v; } }
pub fn g_FEELING_NAMES_NOTHING() -> Py { ensure_init(); FEELING_NAMES_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_NAMES_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_NAMES_NOTHING.write() { *w = v; } }
pub fn g_FEELING_SPEAKER() -> Py { ensure_init(); FEELING_SPEAKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_SPEAKER(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_SPEAKER.write() { *w = v; } }
pub fn g_FEELING_WORDS() -> Py { ensure_init(); FEELING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FEELING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FEELING_WORDS.write() { *w = v; } }
pub fn g_FOLLOW_ON_WORDS() -> Py { ensure_init(); FOLLOW_ON_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_ON_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FOLLOW_ON_WORDS.write() { *w = v; } }
pub fn g_FOOD_NAMED() -> Py { ensure_init(); FOOD_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOOD_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = FOOD_NAMED.write() { *w = v; } }
pub fn g_FOOD_SUBJECT() -> Py { ensure_init(); FOOD_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOOD_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = FOOD_SUBJECT.write() { *w = v; } }
pub fn g_FRAME_ENDS() -> Py { ensure_init(); FRAME_ENDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_ENDS(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_ENDS.write() { *w = v; } }
pub fn g_FRAME_PRONOUNS() -> Py { ensure_init(); FRAME_PRONOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_PRONOUNS(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_PRONOUNS.write() { *w = v; } }
pub fn g_FRAME_WORDS() -> Py { ensure_init(); FRAME_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FRAME_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FRAME_WORDS.write() { *w = v; } }
pub fn g_GENERAL_BODY() -> Py { ensure_init(); GENERAL_BODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GENERAL_BODY(v: Py) { ensure_init(); if let Ok(mut w) = GENERAL_BODY.write() { *w = v; } }
pub fn g_GOES_IN() -> Py { ensure_init(); GOES_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GOES_IN(v: Py) { ensure_init(); if let Ok(mut w) = GOES_IN.write() { *w = v; } }
pub fn g_GONE_INTENSIFIER() -> Py { ensure_init(); GONE_INTENSIFIER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GONE_INTENSIFIER(v: Py) { ensure_init(); if let Ok(mut w) = GONE_INTENSIFIER.write() { *w = v; } }
pub fn g_GONE_OFF_SIGNS() -> Py { ensure_init(); GONE_OFF_SIGNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GONE_OFF_SIGNS(v: Py) { ensure_init(); if let Ok(mut w) = GONE_OFF_SIGNS.write() { *w = v; } }
pub fn g_GONE_STATE() -> Py { ensure_init(); GONE_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GONE_STATE(v: Py) { ensure_init(); if let Ok(mut w) = GONE_STATE.write() { *w = v; } }
pub fn g_GRAMMAR_ONLY() -> Py { ensure_init(); GRAMMAR_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GRAMMAR_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = GRAMMAR_ONLY.write() { *w = v; } }
pub fn g_HANDLING_WORDS() -> Py { ensure_init(); HANDLING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HANDLING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = HANDLING_WORDS.write() { *w = v; } }
pub fn g_HAPPENING_AGAIN() -> Py { ensure_init(); HAPPENING_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAPPENING_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = HAPPENING_AGAIN.write() { *w = v; } }
pub fn g_HAS_GONE() -> Py { ensure_init(); HAS_GONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAS_GONE(v: Py) { ensure_init(); if let Ok(mut w) = HAS_GONE.write() { *w = v; } }
pub fn g_HOUSEHOLD_WORDS() -> Py { ensure_init(); HOUSEHOLD_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOUSEHOLD_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = HOUSEHOLD_WORDS.write() { *w = v; } }
pub fn g_HOW_MANY_DAYS() -> Py { ensure_init(); HOW_MANY_DAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_DAYS(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MANY_DAYS.write() { *w = v; } }
pub fn g_HOW_OFTEN_WORDS() -> Py { ensure_init(); HOW_OFTEN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_OFTEN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = HOW_OFTEN_WORDS.write() { *w = v; } }
pub fn g_IMPROVE() -> Py { ensure_init(); IMPROVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IMPROVE(v: Py) { ensure_init(); if let Ok(mut w) = IMPROVE.write() { *w = v; } }
pub fn g_IRREGULAR_PLURALS() -> Py { ensure_init(); IRREGULAR_PLURALS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IRREGULAR_PLURALS(v: Py) { ensure_init(); if let Ok(mut w) = IRREGULAR_PLURALS.write() { *w = v; } }
pub fn g_JOB_GUARD() -> Py { ensure_init(); JOB_GUARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_JOB_GUARD(v: Py) { ensure_init(); if let Ok(mut w) = JOB_GUARD.write() { *w = v; } }
pub fn g_KEEPING_WORDS() -> Py { ensure_init(); KEEPING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = KEEPING_WORDS.write() { *w = v; } }
pub fn g_KEEPS_OR_STORES() -> Py { ensure_init(); KEEPS_OR_STORES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_OR_STORES(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_OR_STORES.write() { *w = v; } }
pub fn g_KEPT_FOODS() -> Py { ensure_init(); KEPT_FOODS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEPT_FOODS(v: Py) { ensure_init(); if let Ok(mut w) = KEPT_FOODS.write() { *w = v; } }
pub fn g_KINDS_ASKS_ME() -> Py { ensure_init(); KINDS_ASKS_ME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS_ASKS_ME(v: Py) { ensure_init(); if let Ok(mut w) = KINDS_ASKS_ME.write() { *w = v; } }
pub fn g_KINDS_ASKS_WITH_A_VERB() -> Py { ensure_init(); KINDS_ASKS_WITH_A_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS_ASKS_WITH_A_VERB(v: Py) { ensure_init(); if let Ok(mut w) = KINDS_ASKS_WITH_A_VERB.write() { *w = v; } }
pub fn g_KINDS_FOLLOW_UP_WORDS() -> Py { ensure_init(); KINDS_FOLLOW_UP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS_FOLLOW_UP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = KINDS_FOLLOW_UP_WORDS.write() { *w = v; } }
pub fn g_KINDS_NAMED_A_USE() -> Py { ensure_init(); KINDS_NAMED_A_USE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS_NAMED_A_USE(v: Py) { ensure_init(); if let Ok(mut w) = KINDS_NAMED_A_USE.write() { *w = v; } }
pub fn g_KINDS_SAYS_HOW_IT_IS() -> Py { ensure_init(); KINDS_SAYS_HOW_IT_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS_SAYS_HOW_IT_IS(v: Py) { ensure_init(); if let Ok(mut w) = KINDS_SAYS_HOW_IT_IS.write() { *w = v; } }
pub fn g_KINDS_SHAPE() -> Py { ensure_init(); KINDS_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = KINDS_SHAPE.write() { *w = v; } }
pub fn g_KINDS_WORDS() -> Py { ensure_init(); KINDS_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = KINDS_WORDS.write() { *w = v; } }
pub fn g_MACHINE_SOUNDS() -> Py { ensure_init(); MACHINE_SOUNDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MACHINE_SOUNDS(v: Py) { ensure_init(); if let Ok(mut w) = MACHINE_SOUNDS.write() { *w = v; } }
pub fn g_MIN_FORM() -> Py { ensure_init(); MIN_FORM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MIN_FORM(v: Py) { ensure_init(); if let Ok(mut w) = MIN_FORM.write() { *w = v; } }
pub fn g_MIN_SCORE() -> Py { ensure_init(); MIN_SCORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MIN_SCORE(v: Py) { ensure_init(); if let Ok(mut w) = MIN_SCORE.write() { *w = v; } }
pub fn g_MIN_TYPO_LENGTH() -> Py { ensure_init(); MIN_TYPO_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MIN_TYPO_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = MIN_TYPO_LENGTH.write() { *w = v; } }
pub fn g_MISSING_WORDS() -> Py { ensure_init(); MISSING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISSING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = MISSING_WORDS.write() { *w = v; } }
pub fn g_MODAL_PHRASES() -> Py { ensure_init(); MODAL_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MODAL_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = MODAL_PHRASES.write() { *w = v; } }
pub fn g_NAME_ALONE_DOMAINS() -> Py { ensure_init(); NAME_ALONE_DOMAINS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAME_ALONE_DOMAINS(v: Py) { ensure_init(); if let Ok(mut w) = NAME_ALONE_DOMAINS.write() { *w = v; } }
pub fn g_NOT_A_BASE() -> Py { ensure_init(); NOT_A_BASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_BASE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_BASE.write() { *w = v; } }
pub fn g_NOT_A_CHILD_SAID() -> Py { ensure_init(); NOT_A_CHILD_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_CHILD_SAID(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_CHILD_SAID.write() { *w = v; } }
pub fn g_NOT_A_NAME_ALONE() -> Py { ensure_init(); NOT_A_NAME_ALONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME_ALONE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_NAME_ALONE.write() { *w = v; } }
pub fn g_NOT_A_NOSE() -> Py { ensure_init(); NOT_A_NOSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NOSE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_NOSE.write() { *w = v; } }
pub fn g_NOT_A_WORD_WITH_AN_S() -> Py { ensure_init(); NOT_A_WORD_WITH_AN_S.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_WORD_WITH_AN_S(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_WORD_WITH_AN_S.write() { *w = v; } }
pub fn g_NOT_THE_ADJECTIVE() -> Py { ensure_init(); NOT_THE_ADJECTIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_ADJECTIVE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_ADJECTIVE.write() { *w = v; } }
pub fn g_NOT_THE_DOUBLED_STEM() -> Py { ensure_init(); NOT_THE_DOUBLED_STEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_DOUBLED_STEM(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_DOUBLED_STEM.write() { *w = v; } }
pub fn g_NOT_THE_STEM() -> Py { ensure_init(); NOT_THE_STEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_STEM(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_STEM.write() { *w = v; } }
pub fn g_NOT_THE_STEM_WITH_E() -> Py { ensure_init(); NOT_THE_STEM_WITH_E.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_STEM_WITH_E(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_STEM_WITH_E.write() { *w = v; } }
pub fn g_NOT_TIRED_AFTER_ALL() -> Py { ensure_init(); NOT_TIRED_AFTER_ALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_TIRED_AFTER_ALL(v: Py) { ensure_init(); if let Ok(mut w) = NOT_TIRED_AFTER_ALL.write() { *w = v; } }
pub fn g_OCCASION() -> Py { ensure_init(); OCCASION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OCCASION(v: Py) { ensure_init(); if let Ok(mut w) = OCCASION.write() { *w = v; } }
pub fn g_OFF_SAID_OF_FOOD() -> Py { ensure_init(); OFF_SAID_OF_FOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OFF_SAID_OF_FOOD(v: Py) { ensure_init(); if let Ok(mut w) = OFF_SAID_OF_FOOD.write() { *w = v; } }
pub fn g_OFF_STATE_ASKS() -> Py { ensure_init(); OFF_STATE_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OFF_STATE_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = OFF_STATE_ASKS.write() { *w = v; } }
pub fn g_ORIGIN_ASK() -> Py { ensure_init(); ORIGIN_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ORIGIN_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ORIGIN_ASK.write() { *w = v; } }
pub fn g_OWN_THING_ON_THEIR_SIDE() -> Py { ensure_init(); OWN_THING_ON_THEIR_SIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_THING_ON_THEIR_SIDE(v: Py) { ensure_init(); if let Ok(mut w) = OWN_THING_ON_THEIR_SIDE.write() { *w = v; } }
pub fn g_PACK_DATE() -> Py { ensure_init(); PACK_DATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PACK_DATE(v: Py) { ensure_init(); if let Ok(mut w) = PACK_DATE.write() { *w = v; } }
pub fn g_PART_INSIDE() -> Py { ensure_init(); PART_INSIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PART_INSIDE(v: Py) { ensure_init(); if let Ok(mut w) = PART_INSIDE.write() { *w = v; } }
pub fn g_PEOPLE_FEMALE() -> Py { ensure_init(); PEOPLE_FEMALE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_FEMALE(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE_FEMALE.write() { *w = v; } }
pub fn g_PEOPLE_MALE() -> Py { ensure_init(); PEOPLE_MALE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_MALE(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE_MALE.write() { *w = v; } }
pub fn g_PERSON_PRONOUNS() -> Py { ensure_init(); PERSON_PRONOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_PRONOUNS(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_PRONOUNS.write() { *w = v; } }
pub fn g_PHRASE_WEIGHT() -> Py { ensure_init(); PHRASE_WEIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PHRASE_WEIGHT(v: Py) { ensure_init(); if let Ok(mut w) = PHRASE_WEIGHT.write() { *w = v; } }
pub fn g_PLACES_AFTER_BACK() -> Py { ensure_init(); PLACES_AFTER_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACES_AFTER_BACK(v: Py) { ensure_init(); if let Ok(mut w) = PLACES_AFTER_BACK.write() { *w = v; } }
pub fn g_PLAIN_MARGIN() -> Py { ensure_init(); PLAIN_MARGIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAIN_MARGIN(v: Py) { ensure_init(); if let Ok(mut w) = PLAIN_MARGIN.write() { *w = v; } }
pub fn g_PLAN_WORDS() -> Py { ensure_init(); PLAN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_WORDS.write() { *w = v; } }
pub fn g_PLURAL_ONLY_PHRASES() -> Py { ensure_init(); PLURAL_ONLY_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLURAL_ONLY_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = PLURAL_ONLY_PHRASES.write() { *w = v; } }
pub fn g_POINTS_BACK() -> Py { ensure_init(); POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_BACK.write() { *w = v; } }
pub fn g_PRESENT_ASK() -> Py { ensure_init(); PRESENT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRESENT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PRESENT_ASK.write() { *w = v; } }
pub fn g_PRICE_COMPARED() -> Py { ensure_init(); PRICE_COMPARED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_COMPARED(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_COMPARED.write() { *w = v; } }
pub fn g_REPORT_FILLER() -> Py { ensure_init(); REPORT_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPORT_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = REPORT_FILLER.write() { *w = v; } }
pub fn g_REPORT_LEADS() -> Py { ensure_init(); REPORT_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPORT_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = REPORT_LEADS.write() { *w = v; } }
pub fn g_REPORT_PERSON() -> Py { ensure_init(); REPORT_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPORT_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = REPORT_PERSON.write() { *w = v; } }
pub fn g_RETURNED_AFTER() -> Py { ensure_init(); RETURNED_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RETURNED_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = RETURNED_AFTER.write() { *w = v; } }
pub fn g_RING_AS_A_CALL() -> Py { ensure_init(); RING_AS_A_CALL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RING_AS_A_CALL(v: Py) { ensure_init(); if let Ok(mut w) = RING_AS_A_CALL.write() { *w = v; } }
pub fn g_SAID_LEADS() -> Py { ensure_init(); SAID_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = SAID_LEADS.write() { *w = v; } }
pub fn g_SANDING_JOB() -> Py { ensure_init(); SANDING_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SANDING_JOB(v: Py) { ensure_init(); if let Ok(mut w) = SANDING_JOB.write() { *w = v; } }
pub fn g_SHORT_OF_SLEEP() -> Py { ensure_init(); SHORT_OF_SLEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT_OF_SLEEP(v: Py) { ensure_init(); if let Ok(mut w) = SHORT_OF_SLEEP.write() { *w = v; } }
pub fn g_SLIPS() -> Py { ensure_init(); SLIPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLIPS(v: Py) { ensure_init(); if let Ok(mut w) = SLIPS.write() { *w = v; } }
pub fn g_SMELL_VERBS() -> Py { ensure_init(); SMELL_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SMELL_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = SMELL_VERBS.write() { *w = v; } }
pub fn g_SPEAKER_ONLY() -> Py { ensure_init(); SPEAKER_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPEAKER_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = SPEAKER_ONLY.write() { *w = v; } }
pub fn g_SPECIFIC() -> Py { ensure_init(); SPECIFIC.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPECIFIC(v: Py) { ensure_init(); if let Ok(mut w) = SPECIFIC.write() { *w = v; } }
pub fn g_SPECIFIC_PARTS() -> Py { ensure_init(); SPECIFIC_PARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPECIFIC_PARTS(v: Py) { ensure_init(); if let Ok(mut w) = SPECIFIC_PARTS.write() { *w = v; } }
pub fn g_STATES_AFTER() -> Py { ensure_init(); STATES_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATES_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = STATES_AFTER.write() { *w = v; } }
pub fn g_STOPPED_WORKING() -> Py { ensure_init(); STOPPED_WORKING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOPPED_WORKING(v: Py) { ensure_init(); if let Ok(mut w) = STOPPED_WORKING.write() { *w = v; } }
pub fn g_SUBSTITUTE_WORDS() -> Py { ensure_init(); SUBSTITUTE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBSTITUTE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SUBSTITUTE_WORDS.write() { *w = v; } }
pub fn g_THEIR_OWN_HANDS_THINGS() -> Py { ensure_init(); THEIR_OWN_HANDS_THINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_HANDS_THINGS(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN_HANDS_THINGS.write() { *w = v; } }
pub fn g_THE_BACK_ITSELF() -> Py { ensure_init(); THE_BACK_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_BACK_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = THE_BACK_ITSELF.write() { *w = v; } }
pub fn g_THE_FAULT() -> Py { ensure_init(); THE_FAULT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_FAULT(v: Py) { ensure_init(); if let Ok(mut w) = THE_FAULT.write() { *w = v; } }
pub fn g_THE_SAME_OCCASION() -> Py { ensure_init(); THE_SAME_OCCASION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_SAME_OCCASION(v: Py) { ensure_init(); if let Ok(mut w) = THE_SAME_OCCASION.write() { *w = v; } }
pub fn g_TOOTH_OR_GUM() -> Py { ensure_init(); TOOTH_OR_GUM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOOTH_OR_GUM(v: Py) { ensure_init(); if let Ok(mut w) = TOOTH_OR_GUM.write() { *w = v; } }
pub fn g_TOOTH_OUT() -> Py { ensure_init(); TOOTH_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOOTH_OUT(v: Py) { ensure_init(); if let Ok(mut w) = TOOTH_OUT.write() { *w = v; } }
pub fn g_TWO_OF_US() -> Py { ensure_init(); TWO_OF_US.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TWO_OF_US(v: Py) { ensure_init(); if let Ok(mut w) = TWO_OF_US.write() { *w = v; } }
pub fn g_WANTING() -> Py { ensure_init(); WANTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANTING(v: Py) { ensure_init(); if let Ok(mut w) = WANTING.write() { *w = v; } }
pub fn g_WANT_FILLER() -> Py { ensure_init(); WANT_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANT_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = WANT_FILLER.write() { *w = v; } }
pub fn g_WANT_KNOWING() -> Py { ensure_init(); WANT_KNOWING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WANT_KNOWING(v: Py) { ensure_init(); if let Ok(mut w) = WANT_KNOWING.write() { *w = v; } }
pub fn g_WEARING() -> Py { ensure_init(); WEARING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEARING(v: Py) { ensure_init(); if let Ok(mut w) = WEARING.write() { *w = v; } }
pub fn g_WEIGHT_UNITS() -> Py { ensure_init(); WEIGHT_UNITS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEIGHT_UNITS(v: Py) { ensure_init(); if let Ok(mut w) = WEIGHT_UNITS.write() { *w = v; } }
pub fn g_WHEN_WORDS() -> Py { ensure_init(); WHEN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_WORDS.write() { *w = v; } }
pub fn g_WHERE_PART_WORDS() -> Py { ensure_init(); WHERE_PART_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_PART_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_PART_WORDS.write() { *w = v; } }
pub fn g_WHERE_PHRASES() -> Py { ensure_init(); WHERE_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_PHRASES.write() { *w = v; } }
pub fn g_WORD() -> Py { ensure_init(); WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); if let Ok(mut w) = WORD.write() { *w = v; } }
pub fn g_WORD_WEIGHT() -> Py { ensure_init(); WORD_WEIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD_WEIGHT(v: Py) { ensure_init(); if let Ok(mut w) = WORD_WEIGHT.write() { *w = v; } }
pub fn g_WORN_AFTER_A_COLOUR() -> Py { ensure_init(); WORN_AFTER_A_COLOUR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORN_AFTER_A_COLOUR(v: Py) { ensure_init(); if let Ok(mut w) = WORN_AFTER_A_COLOUR.write() { *w = v; } }
pub fn g__ABOUT_THE_CHAT() -> Py { ensure_init(); _ABOUT_THE_CHAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ABOUT_THE_CHAT(v: Py) { ensure_init(); if let Ok(mut w) = _ABOUT_THE_CHAT.write() { *w = v; } }
pub fn g__AFFORD_THE_TRIP() -> Py { ensure_init(); _AFFORD_THE_TRIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AFFORD_THE_TRIP(v: Py) { ensure_init(); if let Ok(mut w) = _AFFORD_THE_TRIP.write() { *w = v; } }
pub fn g__ARTICLE_FORMS() -> Py { ensure_init(); _ARTICLE_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ARTICLE_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = _ARTICLE_FORMS.write() { *w = v; } }
pub fn g__ARTICLE_WORDS() -> Py { ensure_init(); _ARTICLE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ARTICLE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _ARTICLE_WORDS.write() { *w = v; } }
pub fn g__ASK_ABOUT_SOMEONE() -> Py { ensure_init(); _ASK_ABOUT_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_ABOUT_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_ABOUT_SOMEONE.write() { *w = v; } }
pub fn g__ASK_FRAME() -> Py { ensure_init(); _ASK_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_FRAME.write() { *w = v; } }
pub fn g__ASK_FRAME_FORMS() -> Py { ensure_init(); _ASK_FRAME_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_FRAME_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_FRAME_FORMS.write() { *w = v; } }
pub fn g__ASK_PHRASE_MASK() -> Py { ensure_init(); _ASK_PHRASE_MASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ASK_PHRASE_MASK(v: Py) { ensure_init(); if let Ok(mut w) = _ASK_PHRASE_MASK.write() { *w = v; } }
pub fn g__A_BAKE_OR_A_PAN() -> Py { ensure_init(); _A_BAKE_OR_A_PAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BAKE_OR_A_PAN(v: Py) { ensure_init(); if let Ok(mut w) = _A_BAKE_OR_A_PAN.write() { *w = v; } }
pub fn g__A_BREATH() -> Py { ensure_init(); _A_BREATH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_BREATH(v: Py) { ensure_init(); if let Ok(mut w) = _A_BREATH.write() { *w = v; } }
pub fn g__A_KEEP_AND_A_LENGTH() -> Py { ensure_init(); _A_KEEP_AND_A_LENGTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_KEEP_AND_A_LENGTH(v: Py) { ensure_init(); if let Ok(mut w) = _A_KEEP_AND_A_LENGTH.write() { *w = v; } }
pub fn g__A_PERSON() -> Py { ensure_init(); _A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = _A_PERSON.write() { *w = v; } }
pub fn g__A_PERSON_WHO_SLEEPS() -> Py { ensure_init(); _A_PERSON_WHO_SLEEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PERSON_WHO_SLEEPS(v: Py) { ensure_init(); if let Ok(mut w) = _A_PERSON_WHO_SLEEPS.write() { *w = v; } }
pub fn g__A_PLACE_ON_THEM() -> Py { ensure_init(); _A_PLACE_ON_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PLACE_ON_THEM(v: Py) { ensure_init(); if let Ok(mut w) = _A_PLACE_ON_THEM.write() { *w = v; } }
pub fn g__A_THING_JUST_BEFORE() -> Py { ensure_init(); _A_THING_JUST_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_THING_JUST_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = _A_THING_JUST_BEFORE.write() { *w = v; } }
pub fn g__A_THING_POINTED_AT() -> Py { ensure_init(); _A_THING_POINTED_AT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_THING_POINTED_AT(v: Py) { ensure_init(); if let Ok(mut w) = _A_THING_POINTED_AT.write() { *w = v; } }
pub fn g__A_THING_SAID_IS() -> Py { ensure_init(); _A_THING_SAID_IS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_THING_SAID_IS(v: Py) { ensure_init(); if let Ok(mut w) = _A_THING_SAID_IS.write() { *w = v; } }
pub fn g__A_YEARS_OLD() -> Py { ensure_init(); _A_YEARS_OLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_YEARS_OLD(v: Py) { ensure_init(); if let Ok(mut w) = _A_YEARS_OLD.write() { *w = v; } }
pub fn g__BAD_IS_A_STATE() -> Py { ensure_init(); _BAD_IS_A_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BAD_IS_A_STATE(v: Py) { ensure_init(); if let Ok(mut w) = _BAD_IS_A_STATE.write() { *w = v; } }
pub fn g__BARE_IN_A_KEY() -> Py { ensure_init(); _BARE_IN_A_KEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BARE_IN_A_KEY(v: Py) { ensure_init(); if let Ok(mut w) = _BARE_IN_A_KEY.write() { *w = v; } }
pub fn g__BEFORE_THE_THING() -> Py { ensure_init(); _BEFORE_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BEFORE_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = _BEFORE_THE_THING.write() { *w = v; } }
pub fn g__BETWEEN_A_PLACE_AND_ITS_COLD() -> Py { ensure_init(); _BETWEEN_A_PLACE_AND_ITS_COLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BETWEEN_A_PLACE_AND_ITS_COLD(v: Py) { ensure_init(); if let Ok(mut w) = _BETWEEN_A_PLACE_AND_ITS_COLD.write() { *w = v; } }
pub fn g__BE_VERBS() -> Py { ensure_init(); _BE_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BE_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = _BE_VERBS.write() { *w = v; } }
pub fn g__BORROWING_IN_THE_LINE() -> Py { ensure_init(); _BORROWING_IN_THE_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BORROWING_IN_THE_LINE(v: Py) { ensure_init(); if let Ok(mut w) = _BORROWING_IN_THE_LINE.write() { *w = v; } }
pub fn g__BOUNDED() -> Py { ensure_init(); _BOUNDED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BOUNDED(v: Py) { ensure_init(); if let Ok(mut w) = _BOUNDED.write() { *w = v; } }
pub fn g__BOUNDED_KEEP() -> Py { ensure_init(); _BOUNDED_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BOUNDED_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = _BOUNDED_KEEP.write() { *w = v; } }
pub fn g__BRIDGE() -> Py { ensure_init(); _BRIDGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BRIDGE(v: Py) { ensure_init(); if let Ok(mut w) = _BRIDGE.write() { *w = v; } }
pub fn g__BRIDGES() -> Py { ensure_init(); _BRIDGES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BRIDGES(v: Py) { ensure_init(); if let Ok(mut w) = _BRIDGES.write() { *w = v; } }
pub fn g__CACHE() -> Py { ensure_init(); _CACHE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CACHE(v: Py) { ensure_init(); if let Ok(mut w) = _CACHE.write() { *w = v; } }
pub fn g__CACHE_KEEP() -> Py { ensure_init(); _CACHE_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CACHE_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = _CACHE_KEEP.write() { *w = v; } }
pub fn g__CAME_OUT_FLAT() -> Py { ensure_init(); _CAME_OUT_FLAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CAME_OUT_FLAT(v: Py) { ensure_init(); if let Ok(mut w) = _CAME_OUT_FLAT.write() { *w = v; } }
pub fn g__CHAT_PHRASES() -> Py { ensure_init(); _CHAT_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CHAT_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = _CHAT_PHRASES.write() { *w = v; } }
pub fn g__CHAT_TAIL() -> Py { ensure_init(); _CHAT_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CHAT_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = _CHAT_TAIL.write() { *w = v; } }
pub fn g__CHILDS_AGE_ROWS() -> Py { ensure_init(); _CHILDS_AGE_ROWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CHILDS_AGE_ROWS(v: Py) { ensure_init(); if let Ok(mut w) = _CHILDS_AGE_ROWS.write() { *w = v; } }
pub fn g__CHILD_PHRASES() -> Py { ensure_init(); _CHILD_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CHILD_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = _CHILD_PHRASES.write() { *w = v; } }
pub fn g__CHILD_WORDS() -> Py { ensure_init(); _CHILD_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CHILD_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _CHILD_WORDS.write() { *w = v; } }
pub fn g__COLD_WORDS() -> Py { ensure_init(); _COLD_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COLD_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _COLD_WORDS.write() { *w = v; } }
pub fn g__COMMON() -> Py { ensure_init(); _COMMON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COMMON(v: Py) { ensure_init(); if let Ok(mut w) = _COMMON.write() { *w = v; } }
pub fn g__CONTRACTION() -> Py { ensure_init(); _CONTRACTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CONTRACTION(v: Py) { ensure_init(); if let Ok(mut w) = _CONTRACTION.write() { *w = v; } }
pub fn g__CURLY() -> Py { ensure_init(); _CURLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CURLY(v: Py) { ensure_init(); if let Ok(mut w) = _CURLY.write() { *w = v; } }
pub fn g__CURLY_ANY() -> Py { ensure_init(); _CURLY_ANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CURLY_ANY(v: Py) { ensure_init(); if let Ok(mut w) = _CURLY_ANY.write() { *w = v; } }
pub fn g__CUT_BESIDE() -> Py { ensure_init(); _CUT_BESIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CUT_BESIDE(v: Py) { ensure_init(); if let Ok(mut w) = _CUT_BESIDE.write() { *w = v; } }
pub fn g__DEFINING_VERBS() -> Py { ensure_init(); _DEFINING_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEFINING_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = _DEFINING_VERBS.write() { *w = v; } }
pub fn g__DEFINITION_ARTICLES() -> Py { ensure_init(); _DEFINITION_ARTICLES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEFINITION_ARTICLES(v: Py) { ensure_init(); if let Ok(mut w) = _DEFINITION_ARTICLES.write() { *w = v; } }
pub fn g__DEFINITION_ASK() -> Py { ensure_init(); _DEFINITION_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEFINITION_ASK(v: Py) { ensure_init(); if let Ok(mut w) = _DEFINITION_ASK.write() { *w = v; } }
pub fn g__DEFINITION_EMPTY() -> Py { ensure_init(); _DEFINITION_EMPTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEFINITION_EMPTY(v: Py) { ensure_init(); if let Ok(mut w) = _DEFINITION_EMPTY.write() { *w = v; } }
pub fn g__DEFINITION_POINTERS() -> Py { ensure_init(); _DEFINITION_POINTERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEFINITION_POINTERS(v: Py) { ensure_init(); if let Ok(mut w) = _DEFINITION_POINTERS.write() { *w = v; } }
pub fn g__DEFINITION_TAIL() -> Py { ensure_init(); _DEFINITION_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEFINITION_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = _DEFINITION_TAIL.write() { *w = v; } }
pub fn g__DEFINITION_TROUBLE() -> Py { ensure_init(); _DEFINITION_TROUBLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DEFINITION_TROUBLE(v: Py) { ensure_init(); if let Ok(mut w) = _DEFINITION_TROUBLE.write() { *w = v; } }
pub fn g__DREAM() -> Py { ensure_init(); _DREAM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DREAM(v: Py) { ensure_init(); if let Ok(mut w) = _DREAM.write() { *w = v; } }
pub fn g__FILLER() -> Py { ensure_init(); _FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FILLER(v: Py) { ensure_init(); if let Ok(mut w) = _FILLER.write() { *w = v; } }
pub fn g__FLAT_YOU_LIVE_IN() -> Py { ensure_init(); _FLAT_YOU_LIVE_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FLAT_YOU_LIVE_IN(v: Py) { ensure_init(); if let Ok(mut w) = _FLAT_YOU_LIVE_IN.write() { *w = v; } }
pub fn g__FORMS_CACHE() -> Py { ensure_init(); _FORMS_CACHE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FORMS_CACHE(v: Py) { ensure_init(); if let Ok(mut w) = _FORMS_CACHE.write() { *w = v; } }
pub fn g__FORMS_CACHE_KEEP() -> Py { ensure_init(); _FORMS_CACHE_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FORMS_CACHE_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = _FORMS_CACHE_KEEP.write() { *w = v; } }
pub fn g__FRAME_MASK() -> Py { ensure_init(); _FRAME_MASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FRAME_MASK(v: Py) { ensure_init(); if let Ok(mut w) = _FRAME_MASK.write() { *w = v; } }
pub fn g__FRAME_PRONOUN_FORMS() -> Py { ensure_init(); _FRAME_PRONOUN_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FRAME_PRONOUN_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = _FRAME_PRONOUN_FORMS.write() { *w = v; } }
pub fn g__FRAME_STARTS() -> Py { ensure_init(); _FRAME_STARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FRAME_STARTS(v: Py) { ensure_init(); if let Ok(mut w) = _FRAME_STARTS.write() { *w = v; } }
pub fn g__FUNCTION_WORDS() -> Py { ensure_init(); _FUNCTION_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FUNCTION_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _FUNCTION_WORDS.write() { *w = v; } }
pub fn g__HOLDS_MONEY() -> Py { ensure_init(); _HOLDS_MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HOLDS_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = _HOLDS_MONEY.write() { *w = v; } }
pub fn g__HOW_A_PERSON_IS_COLD() -> Py { ensure_init(); _HOW_A_PERSON_IS_COLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HOW_A_PERSON_IS_COLD(v: Py) { ensure_init(); if let Ok(mut w) = _HOW_A_PERSON_IS_COLD.write() { *w = v; } }
pub fn g__INDEX() -> Py { ensure_init(); _INDEX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__INDEX(v: Py) { ensure_init(); if let Ok(mut w) = _INDEX.write() { *w = v; } }
pub fn g__ING_NOUNS() -> Py { ensure_init(); _ING_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ING_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = _ING_NOUNS.write() { *w = v; } }
pub fn g__KEEPING_ANSWER() -> Py { ensure_init(); _KEEPING_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEEPING_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = _KEEPING_ANSWER.write() { *w = v; } }
pub fn g__KEEPING_AN_EYE() -> Py { ensure_init(); _KEEPING_AN_EYE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEEPING_AN_EYE(v: Py) { ensure_init(); if let Ok(mut w) = _KEEPING_AN_EYE.write() { *w = v; } }
pub fn g__KEEPING_KEY() -> Py { ensure_init(); _KEEPING_KEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEEPING_KEY(v: Py) { ensure_init(); if let Ok(mut w) = _KEEPING_KEY.write() { *w = v; } }
pub fn g__KEEPING_ROWS() -> Py { ensure_init(); _KEEPING_ROWS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEEPING_ROWS(v: Py) { ensure_init(); if let Ok(mut w) = _KEEPING_ROWS.write() { *w = v; } }
pub fn g__KEEPING_VERB() -> Py { ensure_init(); _KEEPING_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEEPING_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _KEEPING_VERB.write() { *w = v; } }
pub fn g__KEEPING_WORDS() -> Py { ensure_init(); _KEEPING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEEPING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _KEEPING_WORDS.write() { *w = v; } }
pub fn g__KEPT_SOMEWHERE() -> Py { ensure_init(); _KEPT_SOMEWHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEPT_SOMEWHERE(v: Py) { ensure_init(); if let Ok(mut w) = _KEPT_SOMEWHERE.write() { *w = v; } }
pub fn g__KEY_FORMS() -> Py { ensure_init(); _KEY_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__KEY_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = _KEY_FORMS.write() { *w = v; } }
pub fn g__LIFE_KEY() -> Py { ensure_init(); _LIFE_KEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LIFE_KEY(v: Py) { ensure_init(); if let Ok(mut w) = _LIFE_KEY.write() { *w = v; } }
pub fn g__LINE_FORMS() -> Py { ensure_init(); _LINE_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LINE_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = _LINE_FORMS.write() { *w = v; } }
pub fn g__MARKS() -> Py { ensure_init(); _MARKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MARKS(v: Py) { ensure_init(); if let Ok(mut w) = _MARKS.write() { *w = v; } }
pub fn g__MIN_BARE_WORDS() -> Py { ensure_init(); _MIN_BARE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MIN_BARE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _MIN_BARE_WORDS.write() { *w = v; } }
pub fn g__NOT_A_SUBJECT() -> Py { ensure_init(); _NOT_A_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_SUBJECT.write() { *w = v; } }
pub fn g__NOT_A_THING() -> Py { ensure_init(); _NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_THING.write() { *w = v; } }
pub fn g__NOT_A_THING_HAD() -> Py { ensure_init(); _NOT_A_THING_HAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING_HAD(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_THING_HAD.write() { *w = v; } }
pub fn g__NOT_GONE() -> Py { ensure_init(); _NOT_GONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_GONE(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_GONE.write() { *w = v; } }
pub fn g__OFF_A_PERSON() -> Py { ensure_init(); _OFF_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OFF_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = _OFF_A_PERSON.write() { *w = v; } }
pub fn g__OF_THE_RULE_OUT() -> Py { ensure_init(); _OF_THE_RULE_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OF_THE_RULE_OUT(v: Py) { ensure_init(); if let Ok(mut w) = _OF_THE_RULE_OUT.write() { *w = v; } }
pub fn g__ONE_SPELLING() -> Py { ensure_init(); _ONE_SPELLING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ONE_SPELLING(v: Py) { ensure_init(); if let Ok(mut w) = _ONE_SPELLING.write() { *w = v; } }
pub fn g__ONE_SPELLING_EITHER_WAY() -> Py { ensure_init(); _ONE_SPELLING_EITHER_WAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ONE_SPELLING_EITHER_WAY(v: Py) { ensure_init(); if let Ok(mut w) = _ONE_SPELLING_EITHER_WAY.write() { *w = v; } }
pub fn g__ORDER_OF_STEPS() -> Py { ensure_init(); _ORDER_OF_STEPS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ORDER_OF_STEPS(v: Py) { ensure_init(); if let Ok(mut w) = _ORDER_OF_STEPS.write() { *w = v; } }
pub fn g__OTHER_SPELLINGS_KEEP() -> Py { ensure_init(); _OTHER_SPELLINGS_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OTHER_SPELLINGS_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = _OTHER_SPELLINGS_KEEP.write() { *w = v; } }
pub fn g__PART_FORMS() -> Py { ensure_init(); _PART_FORMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PART_FORMS(v: Py) { ensure_init(); if let Ok(mut w) = _PART_FORMS.write() { *w = v; } }
pub fn g__PART_FORMS_KEEP() -> Py { ensure_init(); _PART_FORMS_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PART_FORMS_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = _PART_FORMS_KEEP.write() { *w = v; } }
pub fn g__PEOPLE_AGED() -> Py { ensure_init(); _PEOPLE_AGED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PEOPLE_AGED(v: Py) { ensure_init(); if let Ok(mut w) = _PEOPLE_AGED.write() { *w = v; } }
pub fn g__PHRASE_OTHER_SPELLINGS() -> Py { ensure_init(); _PHRASE_OTHER_SPELLINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PHRASE_OTHER_SPELLINGS(v: Py) { ensure_init(); if let Ok(mut w) = _PHRASE_OTHER_SPELLINGS.write() { *w = v; } }
pub fn g__PHRASE_SPANS() -> Py { ensure_init(); _PHRASE_SPANS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PHRASE_SPANS(v: Py) { ensure_init(); if let Ok(mut w) = _PHRASE_SPANS.write() { *w = v; } }
pub fn g__PLACES_PEOPLE_LIVE_IN() -> Py { ensure_init(); _PLACES_PEOPLE_LIVE_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PLACES_PEOPLE_LIVE_IN(v: Py) { ensure_init(); if let Ok(mut w) = _PLACES_PEOPLE_LIVE_IN.write() { *w = v; } }
pub fn g__POINTS_AT_IT() -> Py { ensure_init(); _POINTS_AT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POINTS_AT_IT(v: Py) { ensure_init(); if let Ok(mut w) = _POINTS_AT_IT.write() { *w = v; } }
pub fn g__POSSESSIVE() -> Py { ensure_init(); _POSSESSIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POSSESSIVE(v: Py) { ensure_init(); if let Ok(mut w) = _POSSESSIVE.write() { *w = v; } }
pub fn g__PUNCTUATION() -> Py { ensure_init(); _PUNCTUATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PUNCTUATION(v: Py) { ensure_init(); if let Ok(mut w) = _PUNCTUATION.write() { *w = v; } }
pub fn g__RECURRING() -> Py { ensure_init(); _RECURRING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RECURRING(v: Py) { ensure_init(); if let Ok(mut w) = _RECURRING.write() { *w = v; } }
pub fn g__RECURRING_ANYWAY() -> Py { ensure_init(); _RECURRING_ANYWAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RECURRING_ANYWAY(v: Py) { ensure_init(); if let Ok(mut w) = _RECURRING_ANYWAY.write() { *w = v; } }
pub fn g__RECURRING_WITH() -> Py { ensure_init(); _RECURRING_WITH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RECURRING_WITH(v: Py) { ensure_init(); if let Ok(mut w) = _RECURRING_WITH.write() { *w = v; } }
pub fn g__RESULT_VERB() -> Py { ensure_init(); _RESULT_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__RESULT_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _RESULT_VERB.write() { *w = v; } }
pub fn g__ROWS_CACHE() -> Py { ensure_init(); _ROWS_CACHE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ROWS_CACHE(v: Py) { ensure_init(); if let Ok(mut w) = _ROWS_CACHE.write() { *w = v; } }
pub fn g__ROWS_KEEP() -> Py { ensure_init(); _ROWS_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ROWS_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = _ROWS_KEEP.write() { *w = v; } }
pub fn g__SAID_BETWEEN() -> Py { ensure_init(); _SAID_BETWEEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAID_BETWEEN(v: Py) { ensure_init(); if let Ok(mut w) = _SAID_BETWEEN.write() { *w = v; } }
pub fn g__SAVING_FOR_A_TRIP() -> Py { ensure_init(); _SAVING_FOR_A_TRIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAVING_FOR_A_TRIP(v: Py) { ensure_init(); if let Ok(mut w) = _SAVING_FOR_A_TRIP.write() { *w = v; } }
pub fn g__SAVING_VERB() -> Py { ensure_init(); _SAVING_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAVING_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _SAVING_VERB.write() { *w = v; } }
pub fn g__SAYS_OF_THEMSELVES() -> Py { ensure_init(); _SAYS_OF_THEMSELVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAYS_OF_THEMSELVES(v: Py) { ensure_init(); if let Ok(mut w) = _SAYS_OF_THEMSELVES.write() { *w = v; } }
pub fn g__SHAPE_CACHE() -> Py { ensure_init(); _SHAPE_CACHE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SHAPE_CACHE(v: Py) { ensure_init(); if let Ok(mut w) = _SHAPE_CACHE.write() { *w = v; } }
pub fn g__SLEEPY_MARKS() -> Py { ensure_init(); _SLEEPY_MARKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SLEEPY_MARKS(v: Py) { ensure_init(); if let Ok(mut w) = _SLEEPY_MARKS.write() { *w = v; } }
pub fn g__SPOILING() -> Py { ensure_init(); _SPOILING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SPOILING(v: Py) { ensure_init(); if let Ok(mut w) = _SPOILING.write() { *w = v; } }
pub fn g__STEMS() -> Py { ensure_init(); _STEMS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STEMS(v: Py) { ensure_init(); if let Ok(mut w) = _STEMS.write() { *w = v; } }
pub fn g__STILL_GOOD_ASHERS() -> Py { ensure_init(); _STILL_GOOD_ASHERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__STILL_GOOD_ASHERS(v: Py) { ensure_init(); if let Ok(mut w) = _STILL_GOOD_ASHERS.write() { *w = v; } }
pub fn g__SUBJECT_AT_MOST() -> Py { ensure_init(); _SUBJECT_AT_MOST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_AT_MOST(v: Py) { ensure_init(); if let Ok(mut w) = _SUBJECT_AT_MOST.write() { *w = v; } }
pub fn g__SUBJECT_LEAD() -> Py { ensure_init(); _SUBJECT_LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _SUBJECT_LEAD.write() { *w = v; } }
pub fn g__SUBJECT_POINTS() -> Py { ensure_init(); _SUBJECT_POINTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_POINTS(v: Py) { ensure_init(); if let Ok(mut w) = _SUBJECT_POINTS.write() { *w = v; } }
pub fn g__TAIL_ASK_WORDS() -> Py { ensure_init(); _TAIL_ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TAIL_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _TAIL_ASK_WORDS.write() { *w = v; } }
pub fn g__TAKE_OFF() -> Py { ensure_init(); _TAKE_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TAKE_OFF(v: Py) { ensure_init(); if let Ok(mut w) = _TAKE_OFF.write() { *w = v; } }
pub fn g__THE_DEGREE_WORDS() -> Py { ensure_init(); _THE_DEGREE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_DEGREE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _THE_DEGREE_WORDS.write() { *w = v; } }
pub fn g__THE_GROUND_UNDERFOOT() -> Py { ensure_init(); _THE_GROUND_UNDERFOOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_GROUND_UNDERFOOT(v: Py) { ensure_init(); if let Ok(mut w) = _THE_GROUND_UNDERFOOT.write() { *w = v; } }
pub fn g__THE_HEAT_OF_AN_ARGUMENT() -> Py { ensure_init(); _THE_HEAT_OF_AN_ARGUMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_HEAT_OF_AN_ARGUMENT(v: Py) { ensure_init(); if let Ok(mut w) = _THE_HEAT_OF_AN_ARGUMENT.write() { *w = v; } }
pub fn g__THE_HEAT_OF_THE_DAY() -> Py { ensure_init(); _THE_HEAT_OF_THE_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_HEAT_OF_THE_DAY(v: Py) { ensure_init(); if let Ok(mut w) = _THE_HEAT_OF_THE_DAY.write() { *w = v; } }
pub fn g__THE_PRONOUNS() -> Py { ensure_init(); _THE_PRONOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_PRONOUNS(v: Py) { ensure_init(); if let Ok(mut w) = _THE_PRONOUNS.write() { *w = v; } }
pub fn g__THE_THING_JUDGED() -> Py { ensure_init(); _THE_THING_JUDGED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_THING_JUDGED(v: Py) { ensure_init(); if let Ok(mut w) = _THE_THING_JUDGED.write() { *w = v; } }
pub fn g__THE_WH_WORDS() -> Py { ensure_init(); _THE_WH_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_WH_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _THE_WH_WORDS.write() { *w = v; } }
pub fn g__THROAT() -> Py { ensure_init(); _THROAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THROAT(v: Py) { ensure_init(); if let Ok(mut w) = _THROAT.write() { *w = v; } }
pub fn g__WANTED_CACHE() -> Py { ensure_init(); _WANTED_CACHE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WANTED_CACHE(v: Py) { ensure_init(); if let Ok(mut w) = _WANTED_CACHE.write() { *w = v; } }
pub fn g__WANTING() -> Py { ensure_init(); _WANTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WANTING(v: Py) { ensure_init(); if let Ok(mut w) = _WANTING.write() { *w = v; } }
pub fn g__WEAR_ASK() -> Py { ensure_init(); _WEAR_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WEAR_ASK(v: Py) { ensure_init(); if let Ok(mut w) = _WEAR_ASK.write() { *w = v; } }
pub fn g__WHERE() -> Py { ensure_init(); _WHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WHERE(v: Py) { ensure_init(); if let Ok(mut w) = _WHERE.write() { *w = v; } }
pub fn g__WHERE_I_PUT() -> Py { ensure_init(); _WHERE_I_PUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WHERE_I_PUT(v: Py) { ensure_init(); if let Ok(mut w) = _WHERE_I_PUT.write() { *w = v; } }
pub fn g__WHERE_TO_KEEP() -> Py { ensure_init(); _WHERE_TO_KEEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WHERE_TO_KEEP(v: Py) { ensure_init(); if let Ok(mut w) = _WHERE_TO_KEEP.write() { *w = v; } }
pub fn g__WRAPPED() -> Py { ensure_init(); _WRAPPED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WRAPPED(v: Py) { ensure_init(); if let Ok(mut w) = _WRAPPED.write() { *w = v; } }
pub fn g__YEAR_OLD_FIGURE() -> Py { ensure_init(); _YEAR_OLD_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__YEAR_OLD_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = _YEAR_OLD_FIGURE.write() { *w = v; } }
pub fn g__long() -> Py { ensure_init(); _long.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__long(v: Py) { ensure_init(); if let Ok(mut w) = _long.write() { *w = v; } }
pub fn g__short() -> Py { ensure_init(); _short.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__short(v: Py) { ensure_init(); if let Ok(mut w) = _short.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Find the knowledge entry a line is about, by meaning rather than by wording.\n\nEvery domain module in bot/knowledge/ exposes ENTRIES, a list of Entry built with\nentry(). An Entry says what it is about with `key` (single words or short phrases,\noptionally a tuple of alternatives) and how to answer with `reply`. The matcher finds\nthe entry whose key words the line actually contains, so one fact answers every way\nof asking for it: \"how long do I roast a chicken\", \"chicken cooking time\" and\n\"2kg chicken, how long?\" all reach the same entry.\n\n    entry(\"roast-chicken\", (\"chicken\", (\"roast\", \"oven\", \"cook\", \"time\")),\n          \"About 20 minutes per 500g at 190C (170C fan), plus 20 minutes: ...\")\n\nMatching is deliberately strict. Words match loosely (eggs/egg, cooking/cook), a\nphrase matches a short run of words, a phrase scores 3 and a single word 2, and a\nline needs MIN_SCORE (4) to be answered at all. So two signals are needed, and a line\nthat only mentions a topic in passing (\"my chicken pox\") falls through to a module\nthat can help, or to \"I don't know\", instead of getting the wrong canned answer.\n\nThe one exception is a misspelt key word. People type \"chiken\" and \"radiatior\", so a\nline word that is no key word at all and is a single slip (one letter wrong, missing,\nextra, or swapped with its neighbour) away from exactly one key word is read as that\nkey word. Three things keep that from turning a real word into a fact: the word must be\none the engine does not already read, in any of its forms (\"leaving\" is a form of\n\"leave\", so a charging phone does not become a changing leaf), the slip must lead to a\nsingle word family (\"radiaters\" is one answer, \"lips\" is not), and a slip never lifts a\nline past the entry's avoid list, not even an ambiguous one (\"brest\" is a whole-bird\nquestion neither way).\n");
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
    s__DREAM(Py::None);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    s_SLIPS(py_dict(vec![]));
    s_GRAMMAR_ONLY(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("and"), py_str("or"), py_str("but"), py_str("if"), py_str("so"), py_str("as"), py_str("at"), py_str("by"), py_str("for"), py_str("from"), py_str("in"), py_str("into"), py_str("of"), py_str("on"), py_str("onto"), py_str("over"), py_str("under"), py_str("to"), py_str("up"), py_str("with"), py_str("without"), py_str("about"), py_str("after"), py_str("before"), py_str("than"), py_str("then"), py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("it"), py_str("its"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been"), py_str("being"), py_str("am"), py_str("do"), py_str("does"), py_str("did"), py_str("doing"), py_str("done"), py_str("can"), py_str("could"), py_str("should"), py_str("would"), py_str("will"), py_str("shall"), py_str("may"), py_str("might"), py_str("must"), py_str("not"), py_str("no"), py_str("yes"), py_str("ok"), py_str("okay"), py_str("alright"), py_str("just"), py_str("also"), py_str("too"), py_str("very"), py_str("really"), py_str("quite"), py_str("more"), py_str("most"), py_str("much"), py_str("many"), py_str("any"), py_str("all"), py_str("some"), py_str("none"), py_str("every"), py_str("each"), py_str("i"), py_str("me"), py_str("my"), py_str("mine"), py_str("we"), py_str("us"), py_str("our"), py_str("you"), py_str("your"), py_str("he"), py_str("him"), py_str("his"), py_str("she"), py_str("her"), py_str("they"), py_str("them"), py_str("their"), py_str("there"), py_str("here"), py_str("when"), py_str("where"), py_str("what"), py_str("which"), py_str("who"), py_str("how"), py_str("why"), py_str("have"), py_str("has"), py_str("had"), py_str("having"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("go"), py_str("goes"), py_str("going"), py_str("went"), py_str("give"), py_str("gives"), py_str("gave"), py_str("giving"), py_str("take"), py_str("takes"), py_str("took"), py_str("taking"), py_str("make"), py_str("makes"), py_str("made"), py_str("making"), py_str("eat"), py_str("eats"), py_str("ate"), py_str("eating"), py_str("eaten"), py_str("drink"), py_str("drinks"), py_str("drank"), py_str("drinking"), py_str("drunk"), py_str("again"), py_str("still"), py_str("always"), py_str("never"), py_str("sometimes"), py_str("often"), py_str("now"), py_str("today"), py_str("tonight"), py_str("yesterday"), py_str("tomorrow"), py_str("lot"), py_str("lots"), py_str("loads"), py_str("bit"), py_str("kind"), py_str("sort"), py_str("one"), py_str("ones"), py_str("thing"), py_str("things"), py_str("time"), py_str("times"), py_str("day"), py_str("week"), py_str("month"), py_str("year")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9]+(?:'[a-z]+)?")]).unwrap_or_else(|e| exc::raise(e)));
    let __try6 = exc::catch(|| {
    let _ = py_setattr(&stdlib_module("re"), "_MAXCACHE", call_builtin("max", &[call_builtin("getattr", &[stdlib_module("re"), py_str("_MAXCACHE"), Py::Int(512)]).unwrap_or_else(|e| exc::raise(e)), Py::Int(16384)]).unwrap_or_else(|e| exc::raise(e)));
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
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    s_ASKING_MODALS(call_builtin("frozenset", &[mcall(&py_str("can could would should may might must shall will"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it u"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MIN_SCORE(Py::Int(4));
    s_MODAL_PHRASES(py_dict(vec![(py_str("will"), py_tuple(vec![py_str("a will"), py_str("my will"), py_str("the will"), py_str("in the will"), py_str("their will"), py_str("write a will"), py_str("will and testament")])), (py_str("can"), py_tuple(vec![py_str("can of"), py_str("cans of"), py_str("a can"), py_str("the can")])), (py_str("cans"), py_tuple(vec![py_str("cans of"), py_str("some cans"), py_str("two cans"), py_str("three cans")]))]));
    s_FEELING_WORDS(py_tuple(vec![py_str("panic"), py_str("panics"), py_str("panicking"), py_str("panicky"), py_str("panic attack"), py_str("panic attacks"), py_str("anxious"), py_str("anxiety"), py_str("worried"), py_str("worry"), py_str("worrying"), py_str("worries"), py_str("nervous"), py_str("on edge"), py_str("stressed"), py_str("stress"), py_str("stressing"), py_str("overwhelmed"), py_str("overwhelming"), py_str("freaking out"), py_str("depressed"), py_str("depression"), py_str("low mood"), py_str("feel low"), py_str("feeling low"), py_str("feels low"), py_str("feeling flat"), py_str("miserable"), py_str("crying"), py_str("cry"), py_str("tearful"), py_str("upset"), py_str("dreading"), py_str("dread"), py_str("hopeless"), py_str("exhausted"), py_str("drained"), py_str("burnt out"), py_str("burned out"), py_str("burnout"), py_str("fed up"), py_str("cant cope"), py_str("can't cope"), py_str("breaking down"), py_str("angry"), py_str("anger"), py_str("frustrated"), py_str("irritable"), py_str("snappy")]));
    s_ASK_ABOUT(py_tuple(vec![py_str("what"), py_str("how"), py_str("do"), py_str("can"), py_str("should"), py_str("help"), py_str("anything"), py_str("advice"), py_str("tips"), py_str("get rid"), py_str("rid"), py_str("stop"), py_str("treat"), py_str("treatment"), py_str("cure"), py_str("relief"), py_str("relieve"), py_str("ease"), py_str("prevent"), py_str("best"), py_str("works"), py_str("work"), py_str("deal with"), py_str("cope"), py_str("worse"), py_str("helps")]));
    s_ORIGIN_ASK(py_tuple(vec![py_str("come from"), py_str("comes from"), py_str("came from"), py_str("come out of"), py_str("comes out of"), py_str("made from"), py_str("made of"), py_str("made out of"), py_str("made with"), py_str("where from"), py_str("is grown"), py_str("grown in"), py_str("grows on"), py_str("is mined"), py_str("is produced")]));
    s_NOT_A_NOSE(py_tuple(vec![py_str("through my nose"), py_str("through your nose"), py_str("through his nose"), py_str("through her nose"), py_str("through their nose"), py_str("through the nose"), py_str("blocked nose"), py_str("blocked up nose"), py_str("stuffy nose"), py_str("bunged up nose"), py_str("nose is blocked"), py_str("nose is stuffy"), py_str("nose is bunged"), py_str("congested nose")]));
    s_TOOTH_OUT(py_tuple(vec![py_str("tooth out"), py_str("teeth out"), py_str("tooth removed"), py_str("teeth removed"), py_str("tooth pulled"), py_str("tooth extraction"), py_str("extractions"), py_str("tooth taken out"), py_str("teeth taken out"), py_str("had a tooth"), py_str("had my tooth"), py_str("had two teeth"), py_str("gap where"), py_str("where my tooth"), py_str("where the tooth"), py_str("the socket"), py_str("dry socket"), py_str("wisdom tooth out"), py_str("wisdom teeth out"), py_str("wisdom tooth removed"), py_str("after the extraction"), py_str("after my extraction"), py_str("after a tooth")]));
    s_TOOTH_OR_GUM(py_tuple(vec![py_str("tooth"), py_str("teeth"), py_str("toothache"), py_str("filling"), py_str("fillings"), py_str("gum"), py_str("gums"), py_str("molar"), py_str("molars"), py_str("dentist"), py_str("dentists"), py_str("dental"), py_str("denture"), py_str("dentures"), py_str("enamel"), py_str("wisdom tooth"), py_str("wisdom teeth"), py_str("braces"), py_str("mouth ulcer"), py_str("gumline")]));
    s_WORD_WEIGHT(Py::Int(2));
    s_PHRASE_WEIGHT(Py::Int(3));
    s__SAVING_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:save|saves|saving|saved|put\\s+away|putting\\s+away|put\\s+by|putting\\s+by|set\\s+aside|setting\\s+aside|afford|affords|affording)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SAVING_FOR_A_TRIP(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:a|an|the|my|our|their|your|his|her|this|next)?\\s*(?:holiday|holidays|trip|trips|break|weekend\\s+away|city\\s+break|getaway)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AFFORD_THE_TRIP(stdlib_call("re", "compile", &[py_str("\\bafford\\b[^.;!?]{0,25}?\\b(?:holiday|holidays|trip|breaks?|getaway)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRESENT_ASK(py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("gift idea"), py_str("gift ideas"), py_str("present idea"), py_str("present ideas"), py_str("christmas present"), py_str("christmas gift"), py_str("birthday present"), py_str("birthday gift"), py_str("what should i get"), py_str("what can i get"), py_str("what do i get"), py_str("what to get"), py_str("what should i buy"), py_str("what can i buy"), py_str("what do i buy"), py_str("what to buy"), py_str("what should i give"), py_str("what can i give"), py_str("what do i give"), py_str("what to give"), py_str("what would you get"), py_str("what would you buy"), py_str("what would you give"), py_str("get him"), py_str("get her"), py_str("get them"), py_str("buy him"), py_str("buy her"), py_str("buy them"), py_str("give him"), py_str("give her"), py_str("for his birthday"), py_str("for her birthday"), py_str("for their birthday"), py_str("for his christmas"), py_str("for her christmas"), py_str("for their christmas")]));
    s_IMPROVE(py_tuple(vec![py_str("get better at"), py_str("better at"), py_str("get better"), py_str("better"), py_str("improve"), py_str("improving"), py_str("improve my"), py_str("improvement"), py_str("sort out"), py_str("sort out my"), py_str("sort myself out"), py_str("work on"), py_str("work on my"), py_str("get good at"), py_str("good at"), py_str("get the hang of"), py_str("brush up on"), py_str("practise"), py_str("practice"), py_str("sharpen up")]));
    s_HOW_OFTEN_WORDS(py_tuple(vec![py_str("how often"), py_str("how frequently"), py_str("how regularly"), py_str("how many times"), py_str("often"), py_str("frequently"), py_str("regularly"), py_str("how long between"), py_str("every how long"), py_str("frequency")]));
    s_SANDING_JOB(stdlib_call("re", "compile", &[py_str("\\bsanding\\b|\\bsanded\\b|\\bsandpaper\\b|\\bsand paper\\b|\\bsander\\b|\\bsanders\\b|\\b(?:need|needs|needed|should|must|can|could|shall|will|would|got|better|have to|has to)\\s+(?:i\\s+)?(?:to\\s+)?sand\\b|\\bto sand\\b|\\ba sand\\b|\\bsand (?:it|them|this|that|down|off|back|first|before|after|with|the wall|the walls|the wood|the floor|the door|the doors|the ceiling|the trim|the edges?)\\b|\\bfiller\\b|\\bfilled and sanded\\b|\\bpolyfilla\\b|\\bwood filler\\b|\\bfilling knife\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_LEADS(py_tuple(vec![py_str("im"), py_str("i am"), py_str("im really"), py_str("im so"), py_str("im very"), py_str("im all"), py_str("ive been"), py_str("ive got"), py_str("i have"), py_str("i have been"), py_str("i feel"), py_str("feel"), py_str("i keep getting"), py_str("i keep being"), py_str("i get")]));
    s__WRAPPED(py_tuple(vec![stdlib_call("re", "compile", &[py_str("^(?:and |so |ok |okay |hi |hello )?(?:what|anything|something)\\s+(?:should|can|could|would|do|will|shall)\\s+i\\s+(?:do|use|try|take|put|buy)\\s+(?:about|for|on)\\s+(?P<subject>\\S.*?)\\s*\\??$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:and |so )?(?:what|anything)\\s+(?:helps|works|is good)\\s+(?:for\\s+|with\\s+)?(?P<subject>\\S.*?)\\s*\\??$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?:and |so )?(?:any|some|got any|have you got any|do you have any)\\s+(?:tips|advice|suggestions|ideas)\\s+(?:for|on|with|about)\\s+(?P<subject>\\S.*?)\\s*\\??$")]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^(?P<subject>\\S.*?)[,;:]?\\s+(?:what|anything)\\s+(?:helps|works)\\s*\\??$")]).unwrap_or_else(|e| exc::raise(e))]));
    s__NOT_A_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("a an the my your our their his her its it that this these those me us him them someone anyone everyone something anything nothing more less best good now then here there"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("a living"), py_str("a go"), py_str("a bit"), py_str("a while"), py_str("a change"), py_str("the best"), py_str("my best"), py_str("a start"), py_str("the same"), py_str("a look"), py_str("a hand"), py_str("a try"), py_str("a break"), py_str("a rest"), py_str("for good"), py_str("the day"), py_str("the weekend"), py_str("it"), py_str("that"), py_str("this"), py_str("them"), py_str("anything")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FRAME_IDS(call_builtin("frozenset", &[mcall(&py_str("\n    condensation damp limescale slug-control snoring snore-less hangover sore-throat\n    sleep-trouble\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AS_A_KIND(call_builtin("frozenset", &[mcall(&py_str("something anything nothing everything"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MIN_TYPO_LENGTH(Py::Int(4));
    s_RING_AS_A_CALL(py_tuple(vec![py_str("ring the"), py_str("ring my"), py_str("ring him"), py_str("ring her"), py_str("ring them"), py_str("ring you"), py_str("ring me"), py_str("ring someone"), py_str("ring somebody"), py_str("ring anyone"), py_str("ring anybody"), py_str("ring people"), py_str("ringing someone"), py_str("ringing somebody"), py_str("ringing people"), py_str("rang someone"), py_str("rang somebody"), py_str("ring a"), py_str("ring an"), py_str("ring about"), py_str("ring back"), py_str("ring up"), py_str("ring round"), py_str("ring around"), py_str("ring for"), py_str("ring to"), py_str("ring at"), py_str("ring and"), py_str("ring later"), py_str("ring tomorrow"), py_str("ring in"), py_str("ring sick"), py_str("ring ahead"), py_str("ring work"), py_str("ring first thing"), py_str("ring first"), py_str("ring instead"), py_str("ring again"), py_str("to ring"), py_str("should ring"), py_str("will ring"), py_str("can ring"), py_str("could ring"), py_str("would ring"), py_str("might ring"), py_str("must ring"), py_str("need to ring"), py_str("have to ring"), py_str("give a ring"), py_str("give her a ring"), py_str("give him a ring"), py_str("give them a ring"), py_str("give me a ring"), py_str("a ring round")]));
    s_ENDINGS(py_tuple(vec![py_str("iness"), py_str("iest"), py_str("ings"), py_str("ing"), py_str("ies"), py_str("ied"), py_str("ier"), py_str("ers"), py_str("est"), py_str("er"), py_str("es"), py_str("ed"), py_str("ly"), py_str("s"), py_str("y"), py_str("ness"), py_str("ful")]));
    s_MIN_FORM(Py::Int(3));
    register_class_method("Entry", "__init__", py_func("Entry.__init__", |args: &[Py]| Entry____init__(args)));
    register_class_method("Entry", "__repr__", py_func("Entry.__repr__", |args: &[Py]| Entry____repr__(args)));
    s_PLACES_AFTER_BACK(py_str("door|doors|fence|fences|gate|gates|gatepost|gateposts|garden|gardens|room|rooms|seat|seats|window|windows|wall|walls|yard|porch|lawn|path|pathway|step|steps|stairs|packet|packets|pocket|pockets|burner|burners|boiler|panel|panels|bed|beds|alley|bridge|road|lane|field|fields|paddock|plot|car|van|shed|garage|drive|driveway|garden"));
    s_BACK_OF_THE_HOUSE(stdlib_call("re", "compile", &[modulo(&py_str("\\bback\\s+(?:%s)\\b"), &g_PLACES_AFTER_BACK())]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_BACK_ITSELF(stdlib_call("re", "compile", &[modulo(&py_str("\\b(?:my|your|his|her|their|our|the|lower|upper|sore|stiff|aching|aching|bad|old)\\s+back\\b(?!\\s+(?:%s)\\b)"), &g_PLACES_AFTER_BACK())]).unwrap_or_else(|e| exc::raise(e)));
    s_APPLIANCE_NAMES(py_tuple(vec![py_str("tumble dryer"), py_str("washer dryer"), py_str("heat pump dryer"), py_str("condenser dryer"), py_str("vented dryer"), py_str("washing machine"), py_str("fridge freezer"), py_str("fish tank"), py_str("pool pump"), py_str("shower pump"), py_str("fan oven"), py_str("microwave"), py_str("dishwasher"), py_str("dryer"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("cooker"), py_str("hob"), py_str("kettle"), py_str("toaster"), py_str("vacuum"), py_str("hoover"), py_str("mower"), py_str("lawnmower"), py_str("strimmer"), py_str("radiator"), py_str("boiler"), py_str("car"), py_str("van"), py_str("motorbike"), py_str("bike"), py_str("bicycle"), py_str("aquarium"), py_str("hot tub")]));
    s_PEOPLE_MALE(py_tuple(vec![py_str("dad"), py_str("father"), py_str("grandad"), py_str("grandpa"), py_str("grandfather"), py_str("stepdad"), py_str("brother"), py_str("husband"), py_str("boyfriend"), py_str("son"), py_str("uncle"), py_str("nephew"), py_str("bloke"), py_str("fella"), py_str("guy"), py_str("he"), py_str("him"), py_str("his")]));
    s_PEOPLE_FEMALE(py_tuple(vec![py_str("mum"), py_str("mother"), py_str("mam"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandmother"), py_str("stepmum"), py_str("sister"), py_str("wife"), py_str("girlfriend"), py_str("daughter"), py_str("aunt"), py_str("niece"), py_str("she"), py_str("her"), py_str("hers")]));
    s_WHERE_PHRASES(py_tuple(vec![py_str("where to buy"), py_str("where can i buy"), py_str("where should i buy"), py_str("where do i buy"), py_str("where to get"), py_str("where can i get"), py_str("where should i get"), py_str("where do i get"), py_str("cheapest place"), py_str("cheapest shop"), py_str("best place to buy"), py_str("which shop"), py_str("what shop"), py_str("who sells"), py_str("where sells")]));
    s_PRICE_COMPARED(py_tuple(vec![py_str("fridge"), py_str("freezer"), py_str("fridge freezer"), py_str("washing machine"), py_str("washer dryer"), py_str("tumble dryer"), py_str("dishwasher"), py_str("oven"), py_str("hob"), py_str("microwave"), py_str("kettle"), py_str("toaster"), py_str("hoover"), py_str("vacuum"), py_str("tv"), py_str("television"), py_str("laptop"), py_str("phone"), py_str("tablet"), py_str("sofa"), py_str("bed"), py_str("mattress"), py_str("wardrobe"), py_str("pram"), py_str("pushchair"), py_str("car seat"), py_str("bike"), py_str("boiler"), py_str("mower"), py_str("lawnmower")]));
    s_WHERE_PART_WORDS(py_tuple(vec![py_str("spare"), py_str("spares"), py_str("part"), py_str("parts"), py_str("replacement"), py_str("door"), py_str("doors"), py_str("seal"), py_str("seals"), py_str("gasket"), py_str("filter"), py_str("filters"), py_str("basket"), py_str("baskets"), py_str("tray"), py_str("trays"), py_str("drawer"), py_str("drawers"), py_str("shelf"), py_str("shelves"), py_str("handle"), py_str("handles"), py_str("hinge"), py_str("hinges"), py_str("lid"), py_str("lids"), py_str("element"), py_str("elements"), py_str("drum"), py_str("drums"), py_str("belt"), py_str("belts"), py_str("pump"), py_str("pumps"), py_str("motor"), py_str("motors"), py_str("fan"), py_str("fans"), py_str("blade"), py_str("blades"), py_str("brush"), py_str("brushes"), py_str("hose"), py_str("hoses"), py_str("pipe"), py_str("pipes"), py_str("knob"), py_str("knobs"), py_str("bulb"), py_str("bulbs"), py_str("thermostat"), py_str("compressor"), py_str("descaler"), py_str("descale"), py_str("limescale")]));
    s_THE_FAULT(py_tuple(vec![py_tuple(vec![py_str("washing machine"), py_str("washing machine")]), py_tuple(vec![py_str("tumble dryer"), py_str("tumble dryer")]), py_tuple(vec![py_str("tumble drier"), py_str("tumble dryer")]), py_tuple(vec![py_str("spin dryer"), py_str("tumble dryer")]), py_tuple(vec![py_str("dishwasher"), py_str("dishwasher")]), py_tuple(vec![py_str("fridge freezer"), py_str("fridge freezer")]), py_tuple(vec![py_str("fridge"), py_str("fridge")]), py_tuple(vec![py_str("freezer"), py_str("freezer")]), py_tuple(vec![py_str("microwave"), py_str("microwave")]), py_tuple(vec![py_str("cooker"), py_str("cooker")]), py_tuple(vec![py_str("oven"), py_str("oven")]), py_tuple(vec![py_str("hob"), py_str("hob")]), py_tuple(vec![py_str("central heating"), py_str("central heating")]), py_tuple(vec![py_str("heating"), py_str("heating")]), py_tuple(vec![py_str("hot water"), py_str("hot water")]), py_tuple(vec![py_str("radiators"), py_str("radiators")]), py_tuple(vec![py_str("radiator"), py_str("radiator")]), py_tuple(vec![py_str("boiler"), py_str("boiler")]), py_tuple(vec![py_str("shower"), py_str("shower")]), py_tuple(vec![py_str("bath"), py_str("bath")]), py_tuple(vec![py_str("toilet"), py_str("toilet")]), py_tuple(vec![py_str("loo"), py_str("toilet")]), py_tuple(vec![py_str("extractor fan"), py_str("extractor fan")]), py_tuple(vec![py_str("smoke alarm"), py_str("smoke alarm")]), py_tuple(vec![py_str("front door"), py_str("front door")]), py_tuple(vec![py_str("letterbox"), py_str("letterbox")]), py_tuple(vec![py_str("intercom"), py_str("intercom")]), py_tuple(vec![py_str("window frame"), py_str("window")]), py_tuple(vec![py_str("window"), py_str("window")]), py_tuple(vec![py_str("roof"), py_str("roof")]), py_tuple(vec![py_str("guttering"), py_str("gutter")]), py_tuple(vec![py_str("gutter"), py_str("gutter")]), py_tuple(vec![py_str("chimney"), py_str("chimney")]), py_tuple(vec![py_str("damp patch"), py_str("damp")]), py_tuple(vec![py_str("damp"), py_str("damp")]), py_tuple(vec![py_str("mould"), py_str("mould")]), py_tuple(vec![py_str("mold"), py_str("mould")]), py_tuple(vec![py_str("condensation"), py_str("condensation")]), py_tuple(vec![py_str("leak"), py_str("leak")]), py_tuple(vec![py_str("electrics"), py_str("electrics")]), py_tuple(vec![py_str("electric"), py_str("electrics")]), py_tuple(vec![py_str("plug socket"), py_str("socket")]), py_tuple(vec![py_str("socket"), py_str("socket")]), py_tuple(vec![py_str("lights"), py_str("lights")]), py_tuple(vec![py_str("light"), py_str("light")]), py_tuple(vec![py_str("tap"), py_str("tap")]), py_tuple(vec![py_str("sink"), py_str("sink")]), py_tuple(vec![py_str("drains"), py_str("drain")]), py_tuple(vec![py_str("drain"), py_str("drain")]), py_tuple(vec![py_str("pipes"), py_str("pipes")]), py_tuple(vec![py_str("pipe"), py_str("pipe")]), py_tuple(vec![py_str("stairs"), py_str("stairs")]), py_tuple(vec![py_str("ceiling"), py_str("ceiling")]), py_tuple(vec![py_str("floor"), py_str("floor")]), py_tuple(vec![py_str("wall"), py_str("wall")]), py_tuple(vec![py_str("fence"), py_str("fence")]), py_tuple(vec![py_str("lock"), py_str("lock")]), py_tuple(vec![py_str("garden"), py_str("garden")]), py_tuple(vec![py_str("garage"), py_str("garage")])]));
    s__BOUNDED(py_dict(vec![]));
    s__BOUNDED_KEEP(Py::Int(40000));
    s_FOOD_SUBJECT(py_tuple(vec![py_str("sourdough"), py_str("starter"), py_str("kombucha"), py_str("kefir"), py_str("ferment"), py_str("fermented"), py_str("fermenting"), py_str("hooch"), py_str("dough"), py_str("bread"), py_str("beer"), py_str("wine"), py_str("vinegar"), py_str("pickle"), py_str("yoghurt"), py_str("yogurt"), py_str("milk"), py_str("cheese"), py_str("butter"), py_str("meat"), py_str("chicken"), py_str("fish"), py_str("rice"), py_str("pasta"), py_str("potato"), py_str("potatoes"), py_str("food"), py_str("foods"), py_str("drink"), py_str("drinks"), py_str("juice"), py_str("curry"), py_str("stew"), py_str("soup"), py_str("cake"), py_str("biscuit"), py_str("biscuits")]));
    s_HAS_GONE(stdlib_call("re", "compile", &[py_str("\\b(?:has|have|had|'s|s|is|was)\\s+(?:gone|went)\\b(.*)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_GONE_INTENSIFIER(call_builtin("frozenset", &[mcall(&py_str("all a an really very so quite proper totally completely kind sort\n    bit little lot bit too pretty well"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GONE_STATE(call_builtin("frozenset", &[mcall(&py_str("black white blue green grey gray red pink yellow orange purple brown\n    rusty fuzzy blurry funny fuzzy quiet noisy loud silent still cold hot warm cool dark dim\n    blank off bad worse flat dead solid hard soft sticky lumpy dry wet sour mouldy stale numb\n    bright pale gold silver cracked wet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STOPPED_WORKING(py_tuple(vec![py_str("stopped working"), py_str("stopped work"), py_str("stops working"), py_str("not working"), py_str("isnt working"), py_str("isn't working"), py_str("packed in"), py_str("wont work"), py_str("won't work"), py_str("wont start"), py_str("won't start"), py_str("wont come on"), py_str("won't come on"), py_str("dead"), py_str("died"), py_str("has gone")]));
    s_FOOD_NAMED(add(&g_FOOD_SUBJECT(), &py_tuple(vec![py_str("ham"), py_str("bacon"), py_str("sausage"), py_str("sausages"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("mince"), py_str("steak"), py_str("turkey"), py_str("salmon"), py_str("cod"), py_str("haddock"), py_str("tuna"), py_str("prawn"), py_str("prawns"), py_str("egg"), py_str("eggs"), py_str("tofu"), py_str("beans"), py_str("lentils"), py_str("leftover"), py_str("leftovers"), py_str("fruit"), py_str("berries"), py_str("veg"), py_str("vegetables"), py_str("salad"), py_str("gravy"), py_str("stew"), py_str("curry"), py_str("soup"), py_str("rice"), py_str("pasta"), py_str("noodles"), py_str("sandwich"), py_str("mayo"), py_str("mayonnaise"), py_str("cream"), py_str("custard"), py_str("pie"), py_str("quiche"), py_str("dinner"), py_str("lunch"), py_str("supper")])));
    s_SMELL_VERBS(call_builtin("frozenset", &[py_tuple(vec![py_str("smell"), py_str("smells"), py_str("smelled"), py_str("smelt"), py_str("smelling"), py_str("smelly"), py_str("stink"), py_str("stinks"), py_str("stank"), py_str("stinking")])]).unwrap_or_else(|e| exc::raise(e)));
    s__BAD_IS_A_STATE(call_builtin("frozenset", &[py_tuple(vec![py_str("has"), py_str("have"), py_str("had"), py_str("is"), py_str("was"), py_str("are"), py_str("were"), py_str("been"), py_str("be"), py_str("being"), py_str("got"), py_str("gets"), py_str("getting"), py_str("gone"), py_str("went"), py_str("go"), py_str("goes"), py_str("going"), py_str("looks"), py_str("looked"), py_str("looking"), py_str("tastes"), py_str("tasted"), py_str("tasting"), py_str("smells"), py_str("smelled"), py_str("smelt"), py_str("smelling"), py_str("smelling")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_SAID_OF_FOOD(call_builtin("frozenset", &[py_tuple(vec![py_str("sour"), py_str("soured"), py_str("souring"), py_str("slimy"), py_str("slime"), py_str("tacky"), py_str("sticky"), py_str("mouldy"), py_str("moldy"), py_str("rancid"), py_str("curdled"), py_str("curdling"), py_str("rotten"), py_str("putrid"), py_str("funny"), py_str("bad"), py_str("dodgy"), py_str("grey"), py_str("fizzy"), py_str("bubbling"), py_str("discoloured"), py_str("discolored"), py_str("off")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOOD_SMELL(call_builtin("frozenset", &[py_tuple(vec![py_str("great"), py_str("good"), py_str("amazing"), py_str("lovely"), py_str("nice"), py_str("delicious"), py_str("fine"), py_str("wonderful"), py_str("fantastic"), py_str("incredible"), py_str("tasty"), py_str("better"), py_str("perfect"), py_str("heavenly"), py_str("divine"), py_str("ok"), py_str("okay"), py_str("alright"), py_str("lush"), py_str("yummy"), py_str("appetising"), py_str("appetizing"), py_str("gorgeous"), py_str("brilliant"), py_str("of"), py_str("like")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BEFORE_OFF(call_builtin("frozenset", &[py_tuple(vec![py_str("has"), py_str("have"), py_str("had"), py_str("is"), py_str("was"), py_str("are"), py_str("were"), py_str("been"), py_str("be"), py_str("being"), py_str("got"), py_str("gets"), py_str("getting"), py_str("gone"), py_str("went"), py_str("go"), py_str("goes"), py_str("going"), py_str("looks"), py_str("looked"), py_str("looking"), py_str("tastes"), py_str("tasted"), py_str("the"), py_str("my"), py_str("a"), py_str("an"), py_str("bit"), py_str("little"), py_str("now"), py_str("and"), py_str("just"), py_str("really"), py_str("very"), py_str("so"), py_str("still"), py_str("quite"), py_str("definitely"), py_str("starting"), py_str("to"), py_str("on"), py_str("properly"), py_str("seriously")])]).unwrap_or_else(|e| exc::raise(e)));
    s_OFF_STATE_ASKS(py_tuple(vec![py_str("still ok"), py_str("still okay"), py_str("still good"), py_str("still fine"), py_str("still safe"), py_str("still alright"), py_str("still all right"), py_str("still edible"), py_str("gone off"), py_str("going off"), py_str("gone bad"), py_str("went off"), py_str("gone past"), py_str("gone sour"), py_str("gone mouldy"), py_str("gone slimy"), py_str("out of date"), py_str("past its date"), py_str("past the date"), py_str("past their date"), py_str("use by"), py_str("best before"), py_str("is off"), py_str("smells off"), py_str("are off"), py_str("smell off")]));
    s_JOB_GUARD(py_tuple(vec![py_str("repair"), py_str("repairs"), py_str("repaired"), py_str("repairing"), py_str("fix"), py_str("fixes"), py_str("fixed"), py_str("fixing"), py_str("install"), py_str("installs"), py_str("installed"), py_str("installing"), py_str("installation"), py_str("fit"), py_str("fits"), py_str("fitted"), py_str("fitting"), py_str("engineer"), py_str("engineers"), py_str("plumber"), py_str("plumbers"), py_str("callout"), py_str("call out"), py_str("call-out"), py_str("come out"), py_str("coming out"), py_str("swap"), py_str("swapping")]));
    s__BORROWING_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:borrow(?:s|ed|ing|er|ers)?|lend(?:s|ing)?|lent|loan|loans|owed?|owes|tide me over)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__OFF_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:money|quid|pounds?|cash|tenner|fiver|ton|grand|\\d+)\\s+off\\s+(?:of\\s+)?(?:a|an|my|our|your|his|her|their)\\s+(?:mum|mam|mammy|mother|mam|dad|father|parents?|brother|brothers|sister|sisters|gran|granny|nan|nana|grandma|grandad|grandpa|son|daughter|child|kid|uncle|aunt|auntie|cousin|husband|wife|partner|boyfriend|girlfriend|mate|mates|friend|friends|neighbour|neighbours|boss|colleague|colleagues|workmate|flatmate|housemate|lodger|landlord)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__FLAT_YOU_LIVE_IN(stdlib_call("re", "compile", &[py_str("\\b(?:my|the|our|this|that|their|his|her|a|one)\\s+(?:tiny\\s+|small\\s+|little\\s+|new\\s+|old\\s+|top\\s+floor\\s+|ground\\s+floor\\s+|second\\s+floor\\s+|studio\\s+)?flat\\b|\\b(?:move|moves|moved|moving|rent|rents|rented|renting|let|letting|buy|buys|bought|buying|sell|sells|selling|sold|view|viewing|sort|sorting|sorted|decorate|decorating|refurbish|refurbishing)\\s+(?:(?:a|an|the|my|our|this|that|his|her)\\s+)?(?:tiny\\s+|small\\s+|little\\s+|new\\s+|old\\s+)?flat\\b|\\b(?:heat|heating|clean|cleaning)\\s+(?:a|an|the|my|our|this|that)\\s+(?:tiny\\s+|small\\s+|little\\s+)?flat\\b|\\bflats\\b|\\bflat\\s?mates?\\b|\\bbedsit\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_HEAT_OF_AN_ARGUMENT(stdlib_call("re", "compile", &[py_str("\\b(?:got|get|gets|getting|bit|a bit|so|very|really|quite|all)\\s+heated\\b(?!\\s+up)|\\bheated\\s+(?:with|at|about|over|argument|arguments|discussion|discussions|row|rows|debate|debates|exchange|exchanges|words|moment|moments|conversation)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ORDER_OF_STEPS(stdlib_call("re", "compile", &[py_str("\n    \\b what \\s+ (?:should|shall|do|would|must|can) \\s+ (?:i|we) \\s+\n       (?: do|sort|pick|choose|get|buy|wash|clean|tackle|start|book|pack|fix|handle\n         | plan|change|replace|check|ask|say|look \\s+ at|deal \\s+ with|set \\s+ up\n         | make|go \\s+ for|put \\s+ up )\n    \\b .* \\b (?:first|next|before|afterwards|after \\s+ that) \\b\n"), bitor(&stdlib_attr("re", "X").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BAKE_OR_A_PAN(stdlib_call("re", "compile", &[py_str("\\b(?:bread|breads|loaf|loaves|cake|cakes|sponge|scones?|pastry|pastries|tarts?|buns?|rolls?|pizza|pizzas|pancakes?|omelettes?|omelets?|yorkshires?|puddings?|meringues?|souffles?|biscuits?|cookies?|brownies?|crumpets?|dough|naan|pittas?|pita|ciabatta|focaccia|baguette|batter|sourdough|flatbread|chapatis?|tortillas?|waffles?|loaf tin)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CAME_OUT_FLAT(stdlib_call("re", "compile", &[py_str("\\b(?:came|come|comes|coming|turned|turns|turning|went|gone|goes|going)\\s+(?:out\\s+)?(?:a\\s+bit\\s+|a\\s+little\\s+|really\\s+|so\\s+|rather\\s+|too\\s+)?flat\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_GROUND_UNDERFOOT(stdlib_call("re", "compile", &[py_str("\\b(?:walk|walks|walking|walked|stroll|strolls|hike|hikes|route|routes|path|paths|towpath|trail|trails|terrain|ground|fields?|park|parks|promenade|seafront|beach|miles?|kilometres?|outing|day out|somewhere to go|wheelchair|buggy|pram|pushchair)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__YEAR_OLD_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen)\\s*(?:years?|yrs?)\\s+old\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_THING_SAID_IS(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|your|his|her|their|the)\\s+(?P<noun>[a-z][a-z' -]{0,40}?)\\s*(?:is|was|are|were|'s|s)\\b[^.!?]{0,40}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_THING_JUST_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their)\\s+[a-z][a-z']{1,14}\\s*,?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_THING_POINTED_AT(stdlib_call("re", "compile", &[py_str("\\b(?:it|its|it's|that|this|they|these|those)\\s+(?:(?:is|was|are|were|'s|about|roughly|around|only|just|nearly|now|prob)\\b[\\s,]*){1,3}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PEOPLE_AGED(stdlib_call("re", "compile", &[py_str("\\b(?:i|me|my son|my daughter|my child|my children|my kid|my kids|my boy|my girl|son|daughter|child|children|kid|kids|boy|girl|baby|toddler|teen|teenager|he|she|him|her|his|hers|mum|dad|mother|father|wife|husband|partner|friend|nephew|niece|grandson|granddaughter|grandchild)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CHILD_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("child"), py_str("childs"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("boy"), py_str("boys"), py_str("girl"), py_str("girls"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("toddlers"), py_str("teen"), py_str("teens"), py_str("teenager"), py_str("teenagers"), py_str("newborn"), py_str("newborns"), py_str("stepson"), py_str("stepdaughter"), py_str("grandchild"), py_str("grandchildren"), py_str("grandson"), py_str("granddaughter"), py_str("nephew"), py_str("niece"), py_str("pupil"), py_str("pupils")])]).unwrap_or_else(|e| exc::raise(e)));
    s__CHILD_PHRASES(call_builtin("frozenset", &[py_tuple(vec![py_str("little one"), py_str("little boy"), py_str("little girl"), py_str("a child"), py_str("my child"), py_str("the child"), py_str("our child"), py_str("my kids"), py_str("school age child")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_YEARS_OLD(stdlib_call("re", "compile", &[py_str("\\b(?:years?|yrs?) old\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__CHILDS_AGE_ROWS(py_dict(vec![]));
    s__KEEPING_AN_EYE(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|kept|have|has|having|had|get|gets|getting|got)\\s+(?:a|an|one|my|your|his|her|their|our|both|the)?\\s*eyes?\\s*(?:out|on|peeled|open|skinned|glued|trained)\\b|\\beyes?\\s+(?:peeled|open\\s+for|on\\s+(?:the|him|her|them|it|that|this|what))\\b|\\bturn(?:s|ed|ing)?\\s+a\\s+blind\\s+eye\\b|\\beyes?\\s+in\\s+the\\s+back\\s+of\\b|\\beye\\s+contact\\b|\\beye\\s+witness\\b|\\bprivate\\s+eye\\b|\\beagle\\s+eye\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT_OF_SLEEP(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|kept)\\s+(?:(?:nodding|dozing)\\s*(?:off|asleep)?|(?:falling|dropping)\\s+(?:off|asleep))\\b|\\bnodd(?:ing|ed|s)?\\s+off\\b|\\bdoz(?:e|es|ed|ing)\\s+off\\b|\\beyes?\\s+(?:keep|keeps|are|were)?\\s*(?:closing|dropping|shutting)\\b|\\b(?:keep|keeps|keeping|kept)\\s+(?:my|your|his|her|their|our|both)?\\s*eyes?\\s+open\\b(?!\\s+for\\b)|\\bf(?:all|ell)(?:ing|s)?\\s+asleep\\s+(?:at|in|on|during|over)\\b|\\bcan\\s*(?:'?t|cant|not)?\\s*(?:barely|hardly)\\s+(?:keep|stay)\\s+(?:(?:my|your|his|her|their|our|both)\\s+)?(?:eyes?\\s+open|awake|alert)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_TIRED_AFTER_ALL(stdlib_call("re", "compile", &[py_str("\\b(?:hay ?fever|allerg(?:y|ies|ic)|pollen|conjunctivitis|stye|sty|glued|swollen|puffy|watering|streaming)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEVICE_THAT_SLEEPS(call_builtin("frozenset", &[py_tuple(vec![py_str("laptop"), py_str("laptops"), py_str("computer"), py_str("computers"), py_str("pc"), py_str("pcs"), py_str("mac"), py_str("macs"), py_str("macbook"), py_str("phone"), py_str("phones"), py_str("iphone"), py_str("android"), py_str("tablet"), py_str("tablets"), py_str("ipad"), py_str("monitor"), py_str("monitors"), py_str("screen"), py_str("screens"), py_str("tv"), py_str("television"), py_str("televisions"), py_str("console"), py_str("consoles"), py_str("xbox"), py_str("playstation"), py_str("kindle"), py_str("printer"), py_str("printers"), py_str("camera"), py_str("cameras"), py_str("watch"), py_str("fitbit")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PERSON_WHO_SLEEPS(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("i'm"), py_str("me"), py_str("myself"), py_str("we"), py_str("us"), py_str("ourselves"), py_str("he"), py_str("him"), py_str("himself"), py_str("she"), py_str("her"), py_str("herself"), py_str("they"), py_str("them"), py_str("themselves"), py_str("you"), py_str("eyes"), py_str("eye"), py_str("head"), py_str("body"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("baby"), py_str("son"), py_str("daughter"), py_str("boy"), py_str("girl"), py_str("boy"), py_str("boys"), py_str("girls"), py_str("toddler"), py_str("lad"), py_str("lass"), py_str("teenager"), py_str("nan"), py_str("dad"), py_str("mum")])]).unwrap_or_else(|e| exc::raise(e)));
    s__SLEEPY_MARKS(add(&stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e)), &py_str("’")));
    s_KINDS_WORDS(py_tuple(vec![py_str("kinds of"), py_str("kind of"), py_str("types of"), py_str("type of"), py_str("sorts of"), py_str("sort of"), py_str("varieties of"), py_str("variety of"), py_str("different kinds"), py_str("different types"), py_str("different sorts"), py_str("different varieties")]));
    s_KINDS_SHAPE(py_str("options"));
    s_KINDS_FOLLOW_UP_WORDS(py_tuple(vec![py_str("and"), py_str("what"), py_str("how"), py_str("about"), py_str("same"), py_str("also"), py_str("then"), py_str("so")]));
    s_KINDS_ASKS_ME(stdlib_call("re", "compile", &[py_str("\\b(?:do|does|would|will|did|can|could|are|were)\\s+(?:you|u)\\s+(?:like|love|prefer|drink|eat|use|having|have|think|recommend|choose|watch|read|make|order|rate|enjoy)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KINDS_ASKS_WITH_A_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|wanted|wanna|would like|like to|need|needs|needed|wonder|wondering|wondered|ask|asked|know|knowing|learn|find out|show|show me|tell|tell me|curious|after|looking for|interested|thinking of|choose|choosing|pick|picking|decide|deciding|recommend)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KINDS_SAYS_HOW_IT_IS(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|you|they|he|she|it|there|people|everyone|someone|nobody|this|that|these|those|the|my|our|his|her|their)\\b(?:\\s+\\w+){0,3}\\s+(?:is|are|was|were|am|has|have|had|do|does|did|can|could|will|would|should|tried|try|tries|bought|buy|buys|made|make|makes|ate|eat|eats|cooked|cooks|keep|keeps|stock|stocks|sell|sells|sold|use|uses|used|prefer|prefers|like|likes|liked|love| loves|love|loved|hate|hates|hated|grow|grows|grew|got|get|gets|need|needs|want|wants)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_KINDS_NAMED_A_USE(stdlib_call("re", "compile", &[py_str("\\b(?:for|best|better|good|right|make|makes|making|bake|baking|cook|cooking|roast|roasting|boil|boiling|mash|fry|frying|chips|salad|soup|sauce|crumble|pie|cakes?|bread|jam|chutney|keep|keeps|keeping|store|storing|freeze|frozen|buy|buying|cheap|cheapest|substitute|instead|replace|swap|goes|go|with|worth|healthy|bring|brings|take|takes|serve|serves|serving|dinner|lunch|supper|party|picnic|barbecue|bbq|wedding|christmas|birthday|gift|present|occasion|hosting)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__CONTRACTION(call_builtin("frozenset", &[py_tuple(vec![py_str("it's"), py_str("what's"), py_str("that's"), py_str("there's"), py_str("here's"), py_str("he's"), py_str("she's"), py_str("who's"), py_str("where's"), py_str("when's"), py_str("why's"), py_str("how's"), py_str("let's"), py_str("this's"), py_str("one's"), py_str("everyone's"), py_str("someone's"), py_str("nothing's"), py_str("everything's"), py_str("nobody's")])]).unwrap_or_else(|e| exc::raise(e)));
    s_IRREGULAR_PLURALS(py_dict(vec![(py_str("leaves"), py_str("leaf")), (py_str("loaves"), py_str("loaf")), (py_str("sheaves"), py_str("sheaf")), (py_str("thieves"), py_str("thief")), (py_str("wolves"), py_str("wolf")), (py_str("shelves"), py_str("shelf")), (py_str("selves"), py_str("self")), (py_str("elves"), py_str("elf")), (py_str("halves"), py_str("half")), (py_str("calves"), py_str("calf")), (py_str("scarves"), py_str("scarf")), (py_str("wharves"), py_str("wharf")), (py_str("hooves"), py_str("hoof")), (py_str("knives"), py_str("knife")), (py_str("wives"), py_str("wife")), (py_str("lives"), py_str("life"))]));
    s__CURLY(mcall(&py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e))), "maketrans", &[py_dict(vec![(py_str("’"), py_str("'")), (py_str("‘"), py_str("'")), (py_str("ʼ"), py_str("'"))])]));
    s__CURLY_ANY(py_tuple(vec![py_str("’"), py_str("‘"), py_str("ʼ")]));
    s__FORMS_CACHE(py_dict(vec![]));
    s__FORMS_CACHE_KEEP(Py::Int(50000));
    s_NOT_THE_STEM(call_builtin("frozenset", &[py_tuple(vec![py_str("teething"), py_str("sugaring"), py_str("evening"), py_str("stuffing"), py_str("skirting"), py_str("outing"), py_str("weathering"), py_str("lettering"), py_str("wording"), py_str("boarding"), py_str("footing"), py_str("decking"), py_str("clothing"), py_str("housing"), py_str("marketing"), py_str("banking"), py_str("accounting")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_WORD_WITH_AN_S(call_builtin("frozenset", &[py_tuple(vec![py_str("ages")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_STEM_WITH_E(call_builtin("frozenset", &[py_tuple(vec![py_str("housing")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_BASE(call_builtin("frozenset", &[mcall(&py_str("about above across after against along among around at before behind below beneath beside besides between beyond by down for from in into near of off on onto out over past since through till to toward towards under until up upon with within without"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_DOUBLED_STEM(call_builtin("frozenset", &[py_tuple(vec![py_str("bedding")])]).unwrap_or_else(|e| exc::raise(e)));
    s__CHAT_PHRASES(py_str("how did we (?:start|begin|get started|meet)|how did (?:this|our|the) (?:chat|conversation) (?:start|begin|go)|(?:start|beginning) of (?:this|our|the) (?:chat|conversation)|what did (?:i|we) (?:say|ask|tell you) (?:first|at the start|at the beginning)|what was (?:the first thing|my first (?:question|line|message))|first (?:thing|question|line|message) (?:i|we) (?:asked|said|sent)(?: you| him| her)?|do you (?:remember|recall) what (?:i|we) (?:said|asked|told)(?: you)?(?: first)?|what have we (?:talked|spoken) about|what did we talk about"));
    s__CHAT_TAIL(py_str("(?:\\s+(?:out|then|here|it|this|that|chat|conversation|talking|off|again|everything))*\\s*"));
    s__ABOUT_THE_CHAT(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("(?:^|\\s)(?:"), &g__CHAT_PHRASES()), &py_str(")")), &g__CHAT_TAIL()), &py_str("$"))]).unwrap_or_else(|e| exc::raise(e)));
    s__FUNCTION_WORDS(call_builtin("frozenset", &[mcall(&crate::gen::understanding::lexicon::g_CORE(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__THROAT(stdlib_call("re", "compile", &[py_str("^(?:(?:and|oh|ooh|ok|okay|so|but|right|ah|er|um|umm|hmm|sorry|also|just|plus|hey|ta|anyway|anyways|then|actually|well)\\b[\\s,:-]*)+")]).unwrap_or_else(|e| exc::raise(e)));
    s__BRIDGES(add(&call_builtin("tuple", &[crate::gen::understanding::lexicon::g_HANDOFF_BRIDGES()]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("is"), py_str("was")])));
    s__BRIDGE(stdlib_call("re", "compile", &[add(&add(&py_str("^\\s*(?:"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__BRIDGES()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut one: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str(")\\b[\\s,.:;\\-\\u2013\\u2014]*"))]).unwrap_or_else(|e| exc::raise(e)));
    s__PUNCTUATION(stdlib_call("re", "compile", &[py_str("^\\s*[,.:;\\-\\u2013\\u2014]+[\\s,.:;\\-\\u2013\\u2014]*")]).unwrap_or_else(|e| exc::raise(e)));
    s_CARER_FRAME(stdlib_call("re", "compile", &[py_str("^\\s*(?:my|our|the)\\s+(?:poor\\s+|little\\s+|eldest\\s+|youngest\\s+|old\\s+|new\\s+)?(?:son|sons|daughter|daughters|child|children|kid|kids|baby|boy|girl|toddler|teen|teenager|mum|mam|mom|mother|dad|father|wife|husband|partner|boyfriend|girlfriend|sister|brother|niece|nephew|grandson|granddaughter|grandchild|grandchildren|friend|mate)\\s*(?:'s|s')?\\s+(?:has|have|had|got|caught|come down with|came down with|is|are|was|were|keeps getting|came out in|is covered in|has come out in)\\s+(?:(?:an?|the|some)\\s+)*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_ILLNESS(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[crate::gen::understanding::lexicon::g_BODY_PARTS()]).unwrap_or_else(|e| exc::raise(e))])), &py_str("|")), &mcall(&py_str("|"), "join", &[mcall(&py_str("\nill unwell poorly sick sickness bug bugs virus infection infected cold flu fever temperature\ncough coughing sneeze sneezing wheeze wheezing breath breathing rash itchy itching itch spot\nspots lump lumps sore pain painful ache aching hurt hurts injury injured wound cut gash burn\nscald scab bruise bruised swelling swollen stiff sprain strain break broken fracture\ndislocated dizzy fainting faint vomit vomiting diarrhoea diarrhea constipation dehydrated\ndehydration allergy allergic reaction hives eczema psoriasis asthma croup colic teething\nchickenpox measles mumps rubella tonsillitis thrush nits lice worms threadworms ringworm\nverruca verrucas wart warts boil boils abscess stye conjunctivitis earache toothache headache\nmigraine migraines pneumonia bronchitis covid shingles gout sciatica arthritis cystitis uti\nscabies impetigo meningitis quinsy coldsore epilepsy seizure\nseizures dementia alzheimers diabetes anaemia thyroid tumour leukaemia cancer stroke\n"), "split", &[])])), &py_str(")(?:s|es)?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEAD_AS_VERY_AFTER(call_builtin("frozenset", &[py_tuple(vec![py_str("quiet"), py_str("tired"), py_str("easy"), py_str("good"), py_str("bad"), py_str("cheap"), py_str("slow"), py_str("boring"), py_str("cheerful"), py_str("glad"), py_str("lucky"), py_str("proud"), py_str("keen"), py_str("jealous"), py_str("happy"), py_str("sure"), py_str("tricky"), py_str("hard"), py_str("funny"), py_str("rude"), py_str("straight"), py_str("wrong"), py_str("serious"), py_str("certain"), py_str("ahead"), py_str("silent"), py_str("knackered"), py_str("shattered"), py_str("comfy"), py_str("comfortable"), py_str("soft"), py_str("cosy"), py_str("cozy"), py_str("warm"), py_str("cold"), py_str("nice"), py_str("sweet"), py_str("pretty"), py_str("smooth"), py_str("strong"), py_str("fussy"), py_str("broke"), py_str("skint"), py_str("fun"), py_str("obvious"), py_str("scary"), py_str("gentle"), py_str("relaxed"), py_str("bright"), py_str("dark"), py_str("loud"), py_str("chuffed"), py_str("mint"), py_str("annoying"), py_str("embarrassing"), py_str("quick"), py_str("fast"), py_str("clean"), py_str("simple"), py_str("smart"), py_str("friendly"), py_str("heavy")])]).unwrap_or_else(|e| exc::raise(e)));
    s__MARKS(add(&stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e)), &py_str("’")));
    s_COOKING_AS_A_NAME(call_builtin("frozenset", &[py_tuple(vec![py_str("oil"), py_str("oils"), py_str("apple"), py_str("apples"), py_str("chocolate"), py_str("bacon"), py_str("fat"), py_str("sauce"), py_str("wine"), py_str("salt"), py_str("cream"), py_str("onions"), py_str("onion"), py_str("ham")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_BY_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their)\\s+(?:youngest|eldest|oldest|little\\s+one|middle\\s+one|lad|lass|kiddo|laddie|un)\\b|\\b(?:my|our|the|his|her|their)\\s+(?:little|small|wee|big|older|elder|younger|poor)\\s+(?:youngest|eldest|oldest|one|lad|lass|kiddo|laddie|un|boy|girl|man)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_PLACE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("youngest"), py_str("eldest"), py_str("oldest"), py_str("little"), py_str("middle")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_SIZE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("little"), py_str("small"), py_str("wee"), py_str("big"), py_str("older"), py_str("elder"), py_str("younger"), py_str("poor")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHILD_AFTER_A_SIZE(call_builtin("frozenset", &[py_tuple(vec![py_str("boy"), py_str("girl"), py_str("un"), py_str("man")])]).unwrap_or_else(|e| exc::raise(e)));
    s_EVERYDAY_CHILD(call_builtin("frozenset", &[py_tuple(vec![py_str("lad"), py_str("lass"), py_str("kiddo"), py_str("laddie")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILD_WORD_BY_ITSELF(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|their)\\s+(?:youngest|eldest|oldest|little\\s+one|middle\\s+one|lad|lass|kiddo|laddie)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILD_GETTING_OLDER(stdlib_call("re", "compile", &[py_str("\\b(?:teenager|teenagers|teen|teens|teenage|teenaged|secondary\\s+school|high\\s?school|senior\\s+school|year\\s+(?:7|8|9|10|11))\\b|\\b(?:is|are|aged?|turned|turns|turning|now|nearly|just)\\s+(?:1[1-9]|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen)\\b(?!\\s*(?:months?|mths?|weeks?|mos?)\\b)|\\b(?:1[1-9]|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen)\\s*(?:year|yr)s?\\s*old\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TEEN(stdlib_call("re", "compile", &[py_str("\\b(?:teenager|teenagers|teen|teens|teenage|teenaged)\\b|\\b(?:is|are|aged?|turned|turns|turning|now)\\s+(?:1[3-9]|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen)\\b(?!\\s*(?:week|weeks|month|months|day|days|hour|hours|stone|stones|kilos?|kg|lbs?|pounds?))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_CHILD_SAID(call_builtin("frozenset", &[mcall(&py_str("dog dogs puppy puppies cat cats kitten kittens rabbit rabbits\n    hamster hamsters guinea bird birds parrot parrots fish horse horses pony ponies ferret\n    pet pets husband wife partner boyfriend girlfriend fiance fiancee friend friends mate mates\n    boss colleague colleagues flatmate flatmates housemate housemates neighbour neighbours\n    tenant tenants mum mother dad father nan nanna gran granny grandad grandpa sister brother\n    aunt uncle cousin nephew niece"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TWO_OF_US(stdlib_call("re", "compile", &[py_str("\\A\\s*(?:me|i|us)\\s*[,&]?\\s*(?:and|an|nd|n)\\s+(?:my|our|mine)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_PRONOUNS(call_builtin("frozenset", &[mcall(&py_str("it its itself he him his himself hes hed she her hers herself shes\n    shed they them their theirs theyre theyd this these those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__COMMON(bitor(&call_builtin("frozenset", &[mcall(&crate::gen::understanding::lexicon::g_COMMON(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[mcall(&crate::gen::understanding::lexicon::g_CORE(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    s__STEMS(bitor(&call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COMMON()) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g__COMMON()));
    s_PLURAL_ONLY_PHRASES(call_builtin("frozenset", &[py_tuple(vec![py_str("the runs"), py_str("the trots")])]).unwrap_or_else(|e| exc::raise(e)));
    s__WANTED_CACHE(py_dict(vec![]));
    s__ARTICLE_WORDS(call_builtin("frozenset", &[mcall(&py_str("the a an"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__MIN_BARE_WORDS(Py::Int(3));
    s_BARE_WEIGHT(sub(&g_PHRASE_WEIGHT(), &Py::Int(1)));
    s__ARTICLE_FORMS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ARTICLE_WORDS()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)), "union", args))(&__args) });
    s__SHAPE_CACHE(py_dict(vec![]));
    s__WHERE(call_builtin("object", &[]).unwrap_or_else(|e| exc::raise(e)));
    s__LINE_FORMS(call_builtin("object", &[]).unwrap_or_else(|e| exc::raise(e)));
    s__PHRASE_SPANS(call_builtin("object", &[]).unwrap_or_else(|e| exc::raise(e)));
    s__WANTING(call_builtin("object", &[]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_ABOUT_SOMEONE(call_builtin("object", &[]).unwrap_or_else(|e| exc::raise(e)));
    s__SAID_BETWEEN(call_builtin("frozenset", &[mcall(&py_str("\n    just only nearly almost practically finally already still again now then even really\n    actually suddenly completely totally literally basically quite very so too\n    has have had is was were been being keeps kept\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BREATH(Py::Int(2));
    s__ASK_FRAME_FORMS(Py::None);
    s__ONE_SPELLING(py_tuple(vec![py_tuple(vec![py_str("won't"), py_str("will not")]), py_tuple(vec![py_str("wont"), py_str("will not")]), py_tuple(vec![py_str("can't"), py_str("cannot")]), py_tuple(vec![py_str("cant"), py_str("cannot")]), py_tuple(vec![py_str("shan't"), py_str("shall not")]), py_tuple(vec![py_str("dont"), py_str("do not")]), py_tuple(vec![py_str("don't"), py_str("do not")]), py_tuple(vec![py_str("doesnt"), py_str("does not")]), py_tuple(vec![py_str("doesn't"), py_str("does not")]), py_tuple(vec![py_str("isnt"), py_str("is not")]), py_tuple(vec![py_str("isn't"), py_str("is not")]), py_tuple(vec![py_str("arent"), py_str("are not")]), py_tuple(vec![py_str("aren't"), py_str("are not")]), py_tuple(vec![py_str("wasnt"), py_str("was not")]), py_tuple(vec![py_str("wasn't"), py_str("was not")]), py_tuple(vec![py_str("werent"), py_str("were not")]), py_tuple(vec![py_str("weren't"), py_str("were not")]), py_tuple(vec![py_str("didnt"), py_str("did not")]), py_tuple(vec![py_str("didn't"), py_str("did not")]), py_tuple(vec![py_str("hasnt"), py_str("has not")]), py_tuple(vec![py_str("hasn't"), py_str("has not")]), py_tuple(vec![py_str("havent"), py_str("have not")]), py_tuple(vec![py_str("haven't"), py_str("have not")]), py_tuple(vec![py_str("couldnt"), py_str("could not")]), py_tuple(vec![py_str("couldn't"), py_str("could not")]), py_tuple(vec![py_str("wouldnt"), py_str("would not")]), py_tuple(vec![py_str("wouldn't"), py_str("would not")]), py_tuple(vec![py_str("shouldnt"), py_str("should not")]), py_tuple(vec![py_str("shouldn't"), py_str("should not")])]));
    s__ONE_SPELLING_EITHER_WAY(py_dict(vec![]));
    let __iter11 = py_iter_or_raise(&g__ONE_SPELLING());
    for __item in __iter11 {
    let __unpk12 = py_unpack(&__item.clone(), 2, None);
    s__short(__unpk12[0].clone());
    s__long(__unpk12[1].clone());
    let _ = mcall(&mcall(&g__ONE_SPELLING_EITHER_WAY(), "setdefault", &[g__short(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), "add", &[g__long()]);
    let _ = mcall(&mcall(&g__ONE_SPELLING_EITHER_WAY(), "setdefault", &[g__long(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), "add", &[g__short()]);
    }
    s__PHRASE_OTHER_SPELLINGS(py_dict(vec![]));
    s__OTHER_SPELLINGS_KEEP(Py::Int(4000));
    s__PART_FORMS(py_dict(vec![]));
    s__PART_FORMS_KEEP(Py::Int(20000));
    s_WANTING(py_tuple(vec![py_str("want to"), py_str("wants to"), py_str("wanted to"), py_str("wanting to"), py_str("would like to"), py_str("would love to"), py_str("feel like making"), py_str("feel like doing"), py_str("feel like baking"), py_str("feel like cooking"), py_str("fancy making"), py_str("fancy doing"), py_str("fancy baking"), py_str("fancy cooking"), py_str("fancied making"), py_str("fancied doing"), py_str("thinking of"), py_str("planning to"), py_str("keen to"), py_str("hoping to"), py_str("in the mood for"), py_str("need to"), py_str("needs to"), py_str("needing to"), py_str("have to"), py_str("has to"), py_str("got to"), py_str("gotta")]));
    s_WANT_FILLER(call_builtin("frozenset", &[mcall(&py_str("to like of in for would feel the mood keen hoping planning thinking love plan plans planned"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WANT_KNOWING(py_tuple(vec![py_str("want to know"), py_str("wants to know"), py_str("wanted to know"), py_str("wanting to know"), py_str("would like to know"), py_str("want to find out"), py_str("want to check"), py_str("want to ask"), py_str("want to learn"), py_str("like to find out"), py_str("need to know"), py_str("needs to know"), py_str("need to find out"), py_str("need to check"), py_str("need to ask"), py_str("need to learn")]));
    s_ASK_PEOPLE(call_builtin("frozenset", &[mcall(&py_str("\nhe him his she her hers they them their theirs we us our ours you your yours\nson daughter wife husband partner mum dad mother father child children kids baby boy girl\nbrother sister nan grandad grandma\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_PERSON_PRONOUNS(py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("we"), py_str("you")]));
    s_ASK_PERSON_POSSESSIVES(py_tuple(vec![py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("your")]));
    s_DENY_SAID_OF(call_builtin("frozenset", &[mcall(&py_str("i im ive we were he hes she shes they theyre it its there am is are\n    was were"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DENIERS(call_builtin("frozenset", &[mcall(&py_str("not no dont doesnt didnt isnt arent wasnt werent aint"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATES_AFTER(call_builtin("frozenset", &[mcall(&py_str("having getting being been in"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DENY_BETWEEN(call_builtin("frozenset", &[mcall(&py_str("really still even any more all at very too quite a an the"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__FRAME_MASK(call_builtin("object", &[]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_FRAME(call_builtin("frozenset", &[mcall(&py_str("\n    how hows what whats which who whom whose when where why\n    is are was were am be been can could should would will shall may might must\n    do does did doing i im get gets got getting\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_WORDS(call_builtin("frozenset", &[bitor(&bitor(&crate::gen::understanding::normalize::g_TIME_WORDS(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::understanding::normalize::g_TIME_WORDS()) { let mut word: Py = __item.clone(); __out.push(add(&word.clone(), &py_str("s"))); } py_set(__out) })() }), &py_set(vec![py_str("bedtime")]))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LENGTH_WORD(call_builtin("frozenset", &[mcall(&py_str("\n    long last lasts lasted lasting keep keeps kept take takes taking time times run runs\n    minute minutes hour hours day days week weeks month months year years\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STAND_IN(call_builtin("frozenset", &[mcall(&py_str("\n    it its itself they them their theirs this that these those you your we us our\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SAYS_NOTHING(bitor(&bitor(&bitor(&g_ASK_FRAME(), &g_WHEN_WORDS()), &g_A_LENGTH_WORD()), &g_A_STAND_IN()));
    s__TAKE_OFF(Py::None);
    s_FRAME_WORDS(py_tuple(vec![py_str("without")]));
    s_FRAME_ENDS(call_builtin("frozenset", &[py_tuple(vec![py_str("and"), py_str("but"), py_str("or"), py_str("so"), py_str("then"), py_str("when"), py_str("if"), py_str("because"), py_str("while"), py_str("which"), py_str("that"), py_str("to"), py_str("for"), py_str("with"), py_str("plus"), py_str("except"), py_str("besides")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FRAME_PRONOUNS(call_builtin("frozenset", &[mcall(&py_str("it its them they this that those these he she him his her"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__FRAME_PRONOUN_FORMS(Py::None);
    s__FRAME_STARTS(Py::None);
    s_ASK_PHRASE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("best"), py_str("way"), py_str("ways")])]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_PHRASE_MASK(call_builtin("object", &[]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_LEADS(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("ive"), py_str("am"), py_str("have"), py_str("has"), py_str("had"), py_str("got"), py_str("get"), py_str("gets"), py_str("getting"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("feel"), py_str("feels"), py_str("felt"), py_str("feeling"), py_str("been"), py_str("was"), py_str("suffer"), py_str("suffering"), py_str("think"), py_str("thought"), py_str("reckon"), py_str("suspect"), py_str("probably"), py_str("maybe"), py_str("definitely"), py_str("sure")])]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_PERSON(call_builtin("frozenset", &[py_tuple(vec![py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("boy"), py_str("girl"), py_str("baby"), py_str("toddler"), py_str("little"), py_str("youngest"), py_str("eldest"), py_str("oldest"), py_str("teen"), py_str("teenager"), py_str("lad"), py_str("lass"), py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("they"), py_str("them"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("friend"), py_str("friends"), py_str("mate"), py_str("mates"), py_str("mother"), py_str("mum"), py_str("mom"), py_str("father"), py_str("dad"), py_str("nan"), py_str("nana"), py_str("grandma"), py_str("grandad"), py_str("granny"), py_str("brother"), py_str("brothers"), py_str("sister"), py_str("sisters"), py_str("parent"), py_str("parents"), py_str("boss"), py_str("carer"), py_str("his"), py_str("her"), py_str("their"), py_str("our"), py_str("one")])]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_FILLER(bitor(&bitor(&g_REPORT_LEADS(), &g_REPORT_PERSON()), &call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("me"), py_str("really"), py_str("very"), py_str("so"), py_str("too"), py_str("just"), py_str("still"), py_str("also"), py_str("suddenly"), py_str("quite"), py_str("bit"), py_str("lot"), py_str("of"), py_str("with"), py_str("and"), py_str("at"), py_str("in"), py_str("on"), py_str("today"), py_str("now")])]).unwrap_or_else(|e| exc::raise(e))));
    s_HAPPENING_AGAIN(call_builtin("frozenset", &[py_tuple(vec![py_str("again"), py_str("back"), py_str("still"), py_str("already"), py_str("anyway"), py_str("now"), py_str("once"), py_str("twice"), py_str("more"), py_str("every"), py_str("year"), py_str("season"), py_str("spring"), py_str("summer"), py_str("autumn"), py_str("winter"), py_str("time"), py_str("times"), py_str("keeps"), py_str("keep"), py_str("kept"), py_str("coming"), py_str("come"), py_str("came"), py_str("going"), py_str("goes"), py_str("went"), py_str("on"), py_str("round"), py_str("around"), py_str("at"), py_str("as"), py_str("usual"), py_str("this"), py_str("same"), py_str("the"), py_str("it"), py_str("its"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("has"), py_str("have"), py_str("had"), py_str("here"), py_str("today"), py_str("tonight"), py_str("morning"), py_str("night"), py_str("week"), py_str("month"), py_str("of"), py_str("in"), py_str("and"), py_str("so"), py_str("but"), py_str("though"), py_str("now"), py_str("first"), py_str("before"), py_str("like"), py_str("last")])]).unwrap_or_else(|e| exc::raise(e)));
    s_EXPLAIN_LEADS(call_builtin("frozenset", &[py_tuple(vec![py_str("what"), py_str("whats"), py_str("explain"), py_str("explained"), py_str("definition"), py_str("define"), py_str("meaning"), py_str("describe"), py_str("tell"), py_str("info"), py_str("information"), py_str("basics"), py_str("overview")])]).unwrap_or_else(|e| exc::raise(e)));
    s_EXPLAIN_FILLER(bitor(&g_EXPLAIN_LEADS(), &call_builtin("frozenset", &[py_tuple(vec![py_str("s"), py_str("is"), py_str("are"), py_str("was"), py_str("does"), py_str("do"), py_str("did"), py_str("a"), py_str("an"), py_str("the"), py_str("of"), py_str("about"), py_str("on"), py_str("in"), py_str("for"), py_str("me"), py_str("us"), py_str("my"), py_str("mean"), py_str("means"), py_str("meant"), py_str("by"), py_str("symptom"), py_str("symptoms"), py_str("sign"), py_str("signs"), py_str("cause"), py_str("causes"), py_str("treatment"), py_str("exactly"), py_str("actually"), py_str("please"), py_str("and"), py_str("more")])]).unwrap_or_else(|e| exc::raise(e))));
    s_PACK_DATE(py_tuple(vec![py_str("past the date"), py_str("past its date"), py_str("past their date"), py_str("past the use by"), py_str("use by date"), py_str("use by"), py_str("sell by"), py_str("best before"), py_str("out of date"), py_str("after the date"), py_str("after its date"), py_str("after their date"), py_str("date on it"), py_str("date on the"), py_str("date says"), py_str("date said")]));
    s_DAY_WORDS(py_tuple(vec![py_str("next day"), py_str("the next day"), py_str("the day after"), py_str("day after"), py_str("yesterday"), py_str("tomorrow"), py_str("tomorow"), py_str("tmrw"), py_str("tomoz")]));
    s_ASKS_FOR_THE_HANDS(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|would|will|are|do)\\s+(?:you|u)\\b|\\b(?:for|help)\\s+(?:me|us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAND_OR_EYE_DOES_IT(stdlib_call("re", "compile", &[py_str("\\b(?:fill|fills|filling|sign|signs|signing|print|prints|printing|scan|scans|scanning|open|opens|opening|read|reads|reading|check|checks|checking|complete|completes|completing|translate|translates|translating|look\\s+(?:at|through|over))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_WHAT_IT_SAYS(stdlib_call("re", "compile", &[py_str("\\b(?:what|whether|if)\\b|\\b(?:means?|meaning|says?|explain|explains|explaining|understand|summar\\w+|translate\\w*|about\\s+it)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_THING_ON_THEIR_SIDE(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|this|that|these|those|the)\\s+(?:[a-z']+\\s+){0,2}(?:form|forms|pdf|pdfs|file|files|document|documents|paperwork|scan|scans|screenshot|screenshots|photo|photos|letter|letters|invoice|invoices|contract|application|applications|cv|questionnaire|questionnaires)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN_HANDS_THINGS(call_builtin("frozenset", &[mcall(&py_str("form forms pdf pdfs file files document documents paperwork\n    scan scans screenshot screenshots photo photos letter letters invoice invoices contract\n    application applications cv questionnaire questionnaires"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOUSE_WARMING_UP(py_tuple(vec![py_str("heating"), py_str("boiler"), py_str("boilers"), py_str("radiator"), py_str("radiators"), py_str("thermostat"), py_str("central heating"), py_str("hot water"), py_str("immersion"), py_str("combi"), py_str("flue"), py_str("trv"), py_str("house"), py_str("the flat"), py_str("my flat"), py_str("our flat"), py_str("room"), py_str("rooms"), py_str("bathroom"), py_str("kitchen"), py_str("bedroom"), py_str("living room"), py_str("overnight"), py_str("no heat")]));
    s_A_WEATHER_ASK(py_tuple(vec![py_str("weather like"), py_str("weather doing"), py_str("weather going to be"), py_str("weather forecast"), py_str("what is the weather"), py_str("whats the weather"), py_str("what's the weather"), py_str("wether like"), py_str("whats the forecast"), py_str("what's the forecast"), py_str("the forecast for"), py_str("is it cold outside"), py_str("is it cold today"), py_str("is it cold out"), py_str("is it cold enough"), py_str("is it warm outside"), py_str("is it warm today"), py_str("is it warm enough"), py_str("is it hot outside"), py_str("is it hot today"), py_str("is it hot enough"), py_str("is it freezing"), py_str("is it raining"), py_str("is it snowing"), py_str("warm enough to"), py_str("cold enough to"), py_str("how cold is it"), py_str("how hot is it"), py_str("how warm is it"), py_str("what is it like outside"), py_str("whats it like outside"), py_str("what's it like outside")]));
    s_A_WHAT_TO_WEAR(py_tuple(vec![py_str("what should i wear"), py_str("what do i wear"), py_str("what to wear"), py_str("what shall i wear"), py_str("what should i put on"), py_str("what do i put on"), py_str("what to put on"), py_str("what should he wear"), py_str("what should she wear"), py_str("what should they wear"), py_str("what should we wear"), py_str("what does he wear"), py_str("what does she wear"), py_str("what should i pack"), py_str("what do i pack"), py_str("what to pack"), py_str("what shall i pack"), py_str("what should i take"), py_str("what do i take"), py_str("what to take"), py_str("what should i bring"), py_str("what do i bring"), py_str("what to bring"), py_str("what do i need to take"), py_str("what do i need to bring"), py_str("what kit"), py_str("what clothes"), py_str("which clothes"), py_str("what should i put in my bag"), py_str("what do i need to wear")]));
    s_KEPT_FOODS(py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("loaves"), py_str("rolls"), py_str("bagels"), py_str("milk"), py_str("butter"), py_str("cheese"), py_str("yoghurt"), py_str("yogurt"), py_str("cream"), py_str("eggs"), py_str("egg"), py_str("meat"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("bacon"), py_str("ham"), py_str("sausages"), py_str("sausage"), py_str("fish"), py_str("salmon"), py_str("prawns"), py_str("rice"), py_str("pasta"), py_str("noodles"), py_str("potatoes"), py_str("potato"), py_str("flour"), py_str("oats"), py_str("cereal"), py_str("soup"), py_str("stew"), py_str("sauce"), py_str("gravy"), py_str("cake"), py_str("biscuits"), py_str("cookies"), py_str("chocolate"), py_str("fruit"), py_str("apples"), py_str("bananas"), py_str("berries"), py_str("strawberries"), py_str("salad"), py_str("lettuce"), py_str("spinach"), py_str("carrots"), py_str("mushrooms"), py_str("tomatoes"), py_str("onions"), py_str("peas"), py_str("beans"), py_str("leftovers"), py_str("leftover"), py_str("sandwich"), py_str("sandwiches"), py_str("hummus"), py_str("nuts")]));
    s_KEEPING_WORDS(py_tuple(vec![py_str("store"), py_str("stores"), py_str("stored"), py_str("storing"), py_str("storage"), py_str("last"), py_str("lasts"), py_str("lasted"), py_str("fresh"), py_str("fresher"), py_str("freshness"), py_str("stale"), py_str("staler"), py_str("stales"), py_str("staleness"), py_str("mould"), py_str("moulds"), py_str("mouldy"), py_str("mold"), py_str("molds"), py_str("moldy"), py_str("mildew"), py_str("mildewy"), py_str("goes off"), py_str("go off"), py_str("gone off"), py_str("going off"), py_str("goes bad"), py_str("gone bad"), py_str("goes hard"), py_str("gone hard"), py_str("goes soggy"), py_str("gone soggy"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("freezing"), py_str("bin"), py_str("shelf life"), py_str("cold"), py_str("warm"), py_str("damp"), py_str("how long"), py_str("longer"), py_str("longest"), py_str("keep fresh"), py_str("use by"), py_str("best before")]));
    s_GOES_IN(stdlib_call("re", "compile", &[py_str("\\b(?:go|goes|going|went)\\s+(?:in|into)\\b|\\bputs?\\s+in\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTED_INTO_A_THING(stdlib_call("re", "compile", &[py_str("\\bhow many\\b[^?]*\\b(?:in|into|for)\\s+(?:a|an|the|my|one|two|three|four|six|\\d+)\\s+(?!days?\\b|weeks?\\b|months?\\b|years?\\b|mornings?\\b|evenings?\\b|sittings?\\b|meals?\\b|servings?\\b|serving\\b|portion\\b|portions\\b|time\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY_DAYS(stdlib_call("re", "compile", &[py_str("\\bhow many (?:days|weeks|months)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_OR_STORES(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|kept|keeping|store|stores|stored|storing|last|lasts|lasting|fridge|freezer|freeze|go off|gone off)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTAINERS(py_tuple(vec![py_str("bag"), py_str("bags"), py_str("bin bag"), py_str("bin bags"), py_str("bin"), py_str("bins"), py_str("box"), py_str("boxes"), py_str("packet"), py_str("packets"), py_str("pack"), py_str("packs"), py_str("tin"), py_str("tins"), py_str("jar"), py_str("jars"), py_str("tub"), py_str("tubs"), py_str("container"), py_str("containers"), py_str("lunchbox"), py_str("lunch box"), py_str("sack"), py_str("sacks"), py_str("suitcase"), py_str("suitcases"), py_str("rucksack"), py_str("rucksacks"), py_str("backpack"), py_str("backpacks"), py_str("handbag"), py_str("handbags"), py_str("luggage"), py_str("basket"), py_str("baskets"), py_str("crate"), py_str("crates"), py_str("tray"), py_str("trays"), py_str("drawer"), py_str("drawers"), py_str("cupboard"), py_str("cupboards"), py_str("shelf"), py_str("shelves"), py_str("fridge"), py_str("freezer")]));
    s_A_HOURS_ASK(py_tuple(vec![py_str("what time does"), py_str("what time do"), py_str("what time is the"), py_str("what time are the"), py_str("opening hours"), py_str("opening times"), py_str("opening time"), py_str("closing time"), py_str("closes at"), py_str("closes today"), py_str("closes tonight"), py_str("shuts at"), py_str("shuts today"), py_str("close today"), py_str("close tonight"), py_str("close on"), py_str("open today"), py_str("open now"), py_str("open on"), py_str("is it open"), py_str("is it shut"), py_str("open until"), py_str("open till"), py_str("till what time"), py_str("until what time"), py_str("last entry"), py_str("last orders")]));
    s_DISH_ASK(py_tuple(vec![py_str("make with"), py_str("made with"), py_str("make using"), py_str("make from"), py_str("make out of"), py_str("what can i make"), py_str("what can i cook"), py_str("what should i make"), py_str("what can i do with"), py_str("what should i do with"), py_str("cook with"), py_str("cooking with"), py_str("do with"), py_str("using up"), py_str("for dinner"), py_str("for tea"), py_str("for lunch"), py_str("for supper"), py_str("for breakfast"), py_str("tonights tea"), py_str("something with"), py_str("dish with")]));
    s__KEEPING_VERB(call_builtin("frozenset", &[py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("kept"), py_str("keeping")])]).unwrap_or_else(|e| exc::raise(e)));
    s__RESULT_VERB(call_builtin("frozenset", &[py_tuple(vec![py_str("coming"), py_str("comes"), py_str("come"), py_str("turning"), py_str("turns"), py_str("turn")])]).unwrap_or_else(|e| exc::raise(e)));
    s__SPOILING(call_builtin("frozenset", &[py_tuple(vec![py_str("mouldy"), py_str("moldy"), py_str("mould"), py_str("mold"), py_str("moulding"), py_str("molding"), py_str("slimy"), py_str("slimy"), py_str("soggy"), py_str("mushy"), py_str("sour"), py_str("soured"), py_str("souring"), py_str("rancid"), py_str("stale"), py_str("wilted"), py_str("wilting"), py_str("wilt"), py_str("limp"), py_str("off"), py_str("bad"), py_str("rotten"), py_str("rotting"), py_str("rot"), py_str("brown"), py_str("browned"), py_str("browning"), py_str("discoloured"), py_str("discolored"), py_str("curdled"), py_str("curdling"), py_str("curdle"), py_str("smelly"), py_str("smelling"), py_str("stinking"), py_str("sprouting"), py_str("sprouted"), py_str("spotted"), py_str("furry"), py_str("fuzzy"), py_str("blue"), py_str("green"), py_str("shrivel"), py_str("shrivelled"), py_str("shriveling")])]).unwrap_or_else(|e| exc::raise(e)));
    s__ING_NOUNS(call_builtin("frozenset", &[py_tuple(vec![py_str("evening"), py_str("evenings"), py_str("morning"), py_str("mornings"), py_str("ceiling"), py_str("building"), py_str("feeling"), py_str("feelings"), py_str("meeting"), py_str("meetings"), py_str("wedding"), py_str("pudding"), py_str("stuffing"), py_str("dressing"), py_str("filling"), py_str("parking"), py_str("clothing"), py_str("bedding"), py_str("writing"), py_str("painting"), py_str("drawing"), py_str("reading"), py_str("heating"), py_str("lighting"), py_str("awning"), py_str("herring"), py_str("sterling"), py_str("seedling"), py_str("sibling"), py_str("dumpling"), py_str("spring"), py_str("springs"), py_str("string"), py_str("strings"), py_str("thing"), py_str("things"), py_str("ring"), py_str("rings"), py_str("king"), py_str("wing"), py_str("wings")])]).unwrap_or_else(|e| exc::raise(e)));
    s__KEEPING_KEY(call_builtin("frozenset", &[py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("kept"), py_str("keeping")])]).unwrap_or_else(|e| exc::raise(e)));
    s__LIFE_KEY(call_builtin("frozenset", &[py_tuple(vec![py_str("how long"), py_str("shelf life"), py_str("shelf-life"), py_str("store"), py_str("stores"), py_str("storing"), py_str("stored"), py_str("storage"), py_str("last"), py_str("lasts"), py_str("lasted"), py_str("lasting"), py_str("fresh"), py_str("fresh for"), py_str("still good"), py_str("gone off"), py_str("out of date"), py_str("use by")])]).unwrap_or_else(|e| exc::raise(e)));
    s__KEEPING_ROWS(py_dict(vec![]));
    s__RECURRING(call_builtin("frozenset", &[py_tuple(vec![py_str("getting"), py_str("happening"), py_str("happens"), py_str("recurring"), py_str("returning"), py_str("flaring"), py_str("cropping")])]).unwrap_or_else(|e| exc::raise(e)));
    s__RECURRING_ANYWAY(call_builtin("frozenset", &[py_tuple(vec![py_str("coming"), py_str("going"), py_str("giving")])]).unwrap_or_else(|e| exc::raise(e)));
    s__RECURRING_WITH(call_builtin("frozenset", &[py_tuple(vec![py_str("back"), py_str("again"), py_str("around"), py_str("round"), py_str("on")])]).unwrap_or_else(|e| exc::raise(e)));
    s__SUBJECT_POINTS(call_builtin("frozenset", &[mcall(&py_str("a an the my your our his her its it of in on at for to"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MACHINE_SOUNDS(py_tuple(vec![py_str("hum"), py_str("hums"), py_str("humming"), py_str("hummed"), py_str("buzz"), py_str("buzzes"), py_str("buzzing"), py_str("drone"), py_str("drones"), py_str("droning"), py_str("vibrate"), py_str("vibrates"), py_str("vibrating"), py_str("vibration"), py_str("whistle"), py_str("whistles"), py_str("whistling"), py_str("whine"), py_str("whines"), py_str("whining"), py_str("squeal"), py_str("squeals"), py_str("squealing"), py_str("squeak"), py_str("squeaks"), py_str("squeaking"), py_str("screech"), py_str("screeches"), py_str("screeching"), py_str("shriek"), py_str("shrieks"), py_str("shrieking"), py_str("hiss"), py_str("hisses"), py_str("hissing"), py_str("gurgle"), py_str("gurgles"), py_str("gurgling"), py_str("glug"), py_str("glugging"), py_str("bang"), py_str("bangs"), py_str("banging"), py_str("knock"), py_str("knocks"), py_str("knocking"), py_str("rumble"), py_str("rumbles"), py_str("rumbling"), py_str("clunk"), py_str("clunks"), py_str("clunking"), py_str("clank"), py_str("clanks"), py_str("clanking"), py_str("rattle"), py_str("rattles"), py_str("rattling"), py_str("tick"), py_str("ticks"), py_str("ticking"), py_str("tap"), py_str("taps"), py_str("tapping"), py_str("click"), py_str("clicks"), py_str("clicking"), py_str("thump"), py_str("thumps"), py_str("thumping"), py_str("roar"), py_str("roaring"), py_str("grind"), py_str("grinds"), py_str("grinding"), py_str("scrape"), py_str("scrapes"), py_str("scraping")]));
    s__TAIL_ASK_WORDS(Py::None);
    s__CACHE(py_dict(vec![]));
    s__CACHE_KEEP(Py::Int(6));
    s__KEY_FORMS(py_dict(vec![]));
    s__INDEX(py_dict(vec![]));
    s_NOT_THE_ADJECTIVE(call_builtin("frozenset", &[mcall(&py_str("\n    ready busy handy party army belly copy body study lady fully funny pity cosy policy every\n    bully tally holy early rally holly jelly apply battery cheeky dicky factory fairy gravy\n    irony kindly multiply pasty silly shiny story whisky clammy telly chilly watery shivery\n    sticky\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PERSON(bitor(&bitor(&call_builtin("frozenset", &[g_PEOPLE_MALE()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[g_PEOPLE_FEMALE()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("frozenset", &[py_tuple(vec![py_str("friend"), py_str("mate"), py_str("friends"), py_str("partner"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("baby"), py_str("neighbour"), py_str("neighbor"), py_str("colleague"), py_str("boss"), py_str("teacher"), py_str("someone"), py_str("somebody"), py_str("cousin"), py_str("person"), py_str("people"), py_str("they"), py_str("them"), py_str("we"), py_str("us"), py_str("you")])]).unwrap_or_else(|e| exc::raise(e))));
    s__NOT_A_THING_HAD(call_builtin("frozenset", &[py_tuple(vec![py_str("to"), py_str("had"), py_str("been"), py_str("being"), py_str("it"), py_str("them"), py_str("not"), py_str("no"), py_str("never"), py_str("nothing"), py_str("none"), py_str("enough"), py_str("already"), py_str("yet")])]).unwrap_or_else(|e| exc::raise(e)));
    s__FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("actually"), py_str("really"), py_str("honestly"), py_str("literally"), py_str("genuinely"), py_str("basically")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_ABOUT_A_THING(add(&g_FEELING_WORDS(), &py_tuple(vec![py_str("sad"), py_str("sadness"), py_str("unhappy"), py_str("scared"), py_str("frightened"), py_str("lonely"), py_str("gutted"), py_str("devastated"), py_str("heartbroken"), py_str("annoyed"), py_str("annoying"), py_str("furious"), py_str("cross"), py_str("upsetting"), py_str("fuming")])));
    s_FEELING_ABOUT(call_builtin("frozenset", &[mcall(&py_str("about of with over at by"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_ENDS(call_builtin("frozenset", &[mcall(&py_str("\n    and but because so then when if that which who whom whose or also plus after before since\n    till until while though although yet today tomorrow tonight now yesterday again too as\n    i im ive id we they he she them us\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_NAMES_NOTHING(call_builtin("frozenset", &[mcall(&py_str("\n    it them everything nothing anything something all\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_DOMAINS(call_builtin("frozenset", &[py_tuple(vec![py_str("family"), py_str("friends"), py_str("study"), py_str("health"), py_str("sleep"), py_str("pregnancy"), py_str("money"), py_str("work"), py_str("lifeadmin"), py_str("body"), py_str("wording"), py_str("complaints"), py_str("travel"), py_str("home")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_FAULT_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    leak leaks leaked leaking drip drips dripping burst bursts bursting broken broke breaks\n    blocked blocking stopped stops smoking sparking smells smelly smell damp mould mold\n    flooded flooding flood loose wobbles wobbling snapped cracked clicking noisy\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_SPEAKER(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("im"), py_str("ive"), py_str("i'm"), py_str("i've"), py_str("id"), py_str("me"), py_str("myself")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEARED_THINGS(call_builtin("frozenset", &[mcall(&py_str("\n    dentist dentists dentistry dental flying plane planes flight flights needles needle\n    injections injection jab jabs spiders spider snakes snake heights height lifts lift\n    escalators dark thunder lightning storms storm blood vomit sick wasps bees mice rats\n    clowns hospitals hospital doctors doctor dentists surgery operation operations\n    presentations public presentations interviews interview exams exam tests test motorways\n    motorway driving flying ferries tunnels enclosed spaces crowds dentists darkness\n    water swimming swim pool pools\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__WHERE_I_PUT(stdlib_call("re", "compile", &[py_str("\\bwhere\\s+(?:did|have|has|had)\\s+i\\s+(?:put|leave)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__WHERE_TO_KEEP(stdlib_call("re", "compile", &[py_str("\\bwhere\\s+(?:should|shall|do|can|could|would|will|might|am\\s+i\\s+to)?\\s*(?:to\\s+)?(?:i\\s+|we\\s+|you\\s+)?(?:keep|store|stow|put\\s+away|pack\\s+away)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__KEPT_SOMEWHERE(call_builtin("frozenset", &[mcall(&py_str("\nkeep keeps keeping kept store stores stored storing storage stow stows stowed stowing\naway loft wardrobe moth moths cedar\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__HOLDS_MONEY(stdlib_call("re", "compile", &[py_str("\\u00a3\\s?\\d|\\b\\d+(?:\\.\\d+)?\\s?(?:p|pence|pounds?|quid)\\b|\\b\\d+\\.\\d{2}\\b|\\bpounds?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_THING_JUDGED(stdlib_call("re", "compile", &[py_str("\\bfor\\s+((?:a|an|the|my|our|this|that|these|those)\\s+)?([a-z][a-z' ]{1,40})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_AT_IT(call_builtin("frozenset", &[mcall(&py_str("\n    one ones it that this these those them they him her he she the thing things that one this one\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PLACE_ON_THEM(stdlib_call("re", "compile", &[py_str("(?:,?\\s*(?:on|in|under|near|around|behind|above|below|beside|by|at|just (?:above|below|under|behind|over|beside|next to))\\s*(?:the\\s+)?(?:my|his|her|their|our|your|its|this|that)?\\s*(?:left|right|inner|outer|lower|upper|front|back of the|top of the|side of the)?\\s*([a-z]+))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_GONE(stdlib_call("re", "compile", &[py_str("\\b(?:has|have|had|is|are|was|were|does|did)?\\s*(?:not|n't|never)\\s+(?:gone|gone away|shifted|cleared|cleared up|eased|settled|improved|any better|got any better|got better|better)\\b|\\bno better\\b|\\bwont\\s+go\\b|\\bwon't\\s+go\\b|\\bwould\\s+not\\s+go\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SUBJECT_LEAD(stdlib_call("re", "compile", &[py_str("^(?:and|but|so|well|ok|okay|hi|hello|um|er)\\b[,\\s]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__POSSESSIVE(stdlib_call("re", "compile", &[py_str("^(?:the|my|his|her|their|our|your|a|an|this|that|these|those)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OF_THE_RULE_OUT(mcall(&py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), "fromkeys", &[mcall(&py_str("not a an the rather than instead of and but or please really\nactually just so though although is are was were be been do does did"), "split", &[]), Py::Bool(true)]));
    s__THE_DEGREE_WORDS(call_builtin("frozenset", &[mcall(&py_str("degree degrees"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_HEAT_OF_THE_DAY(call_builtin("frozenset", &[mcall(&py_str("\n    hot cold warm cool heat heating temperature weather forecast oven fan oven thermometer summer\n    winter sunny degrees\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_WH_WORDS(call_builtin("frozenset", &[mcall(&py_str("what whats why how when where which who whose whom"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_FRAME(call_builtin("frozenset", &[mcall(&py_str("\n    what whats why how when where which who whose whom do does did should shall can could will\n    would may might must i me my we our us you your the a an is are was were be been being to\n    for of in on at it that this these those there here and or but if so then also just\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__PLACES_PEOPLE_LIVE_IN(call_builtin("frozenset", &[mcall(&py_str("\n    flat house home room rooms kitchen bedroom bathroom lounge hall hallway office classroom\n    conservatory garage shed caravan cabin attic loft basement studio cottage\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__COLD_WORDS(call_builtin("frozenset", &[mcall(&py_str("cold freezing frozen chilly baltic perishing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__BETWEEN_A_PLACE_AND_ITS_COLD(call_builtin("frozenset", &[mcall(&py_str("\n    is was are were get gets got been be feels feel really very so too absolutely\n    bit little always still\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__HOW_A_PERSON_IS_COLD(call_builtin("frozenset", &[mcall(&py_str("\n    shiver shivers shivering shivery chilled freezing numb goosebumps\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__SAYS_OF_THEMSELVES(call_builtin("frozenset", &[mcall(&py_str("i im ive id iam me myself"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__DEFINITION_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:so\\s+|ok\\s+|okay\\s+|and\\s+|but\\s+)?(?:what\\s*(?:'s|\\u2019s|s)?\\s+(?:is|are|was|were)\\s+(?P<thing>[^?.!,;:]{1,60}?)|what\\s*(?:'s|\\u2019s|s)\\s+(?P<thing2>[^?.!,;:]{1,60}?)|what\\s+(?:does|do)\\s+(?P<thing3>[^?.!,;:]{1,60}?)\\s+mean|define\\s+(?P<thing4>[^?.!,;:]{1,60}?))\\s*(?:\\?)?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DEFINITION_TAIL(call_builtin("frozenset", &[mcall(&py_str("\n    for mean means meant exactly then again please called thats it its in simple terms\n    supposed to do stand stands actually really even anyway though\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__DEFINITION_POINTERS(call_builtin("frozenset", &[mcall(&py_str("\n    there it that this these those he she they we you i someone anyone somebody nobody\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__DEFINITION_TROUBLE(bitor(&call_builtin("frozenset", &[mcall(&py_str("\n    wrong matter happening pain painful hurts hurt hurting ache aches aching smell smells smelling\n    leak leaks leaking noisy noise broken stuck faulty fix\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("going on"), py_str("up with"), py_str("up to")])));
    s__DEFINITION_EMPTY(call_builtin("frozenset", &[mcall(&py_str("\n    a an the my your our their his her its some any this that these those of in on at to and or\n    for with from about it that this there here one ones thing things stuff kind sort type\n    what which who whom whose how why when where right best good\n    going happening wrong up on around next\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__DEFINING_VERBS(call_builtin("frozenset", &[mcall(&py_str("\n    is are was were means mean refers gives give costs cost covers cover keeps keep works work\n    helps help makes make takes take needs need holds hold comes come runs run lets let stops stop\n    protects protect lets replaces replace counts count serves serve\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__BE_VERBS(call_builtin("frozenset", &[mcall(&py_str("is are was were"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__SUBJECT_AT_MOST(Py::Int(6));
    s__DEFINITION_ARTICLES(call_builtin("frozenset", &[mcall(&py_str("a an the"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__BEFORE_THE_THING(call_builtin("frozenset", &[mcall(&py_str("\n    of in on with at from your my our their its his her\n    cup mug glass slice piece bit bowl plate packet tin can bottle spoon teaspoon tablespoon\n    handful pinch taste sip bite\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_ALONE_DOMAINS(call_builtin("frozenset", &[mcall(&py_str("family health body babyfood pregnancy"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NAME_ALONE(call_builtin("frozenset", &[mcall(&py_str("what why how when where which who whom whose is are was were\n    am do does did can could should would will shall may might i im ive id you your u my mine\n    me we our us it its that this these those there they them he she him her his hers the a an\n    and or but if so then than about for with on in at to of from not no yes ok okay thanks\n    please help huh"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STATE_AS_A_NAME_END(call_builtin("frozenset", &[mcall(&py_str("closed shut blocked swollen broken numb off blocked"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STATE_IN_THE_PHRASE(call_builtin("frozenset", &[mcall(&py_str("feel feels feeling felt gone went go goes going get gets\n    getting got gotten been being be keep keeps keeping kept start starts started starting\n    burn burns burning"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_WORDS(py_tuple(vec![py_str("plan"), py_str("plans"), py_str("planning"), py_str("organise"), py_str("organize"), py_str("prepare"), py_str("preparing")]));
    s_POINTS_BACK(call_builtin("frozenset", &[mcall(&py_str("it its it's that this these those there them they he she him her"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FOLLOW_ON_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nabout actually alright anyway bad better change good happen happens idea instead mean means meant\nnow okay otherwise question really safe suppose then thing think true wait worse wrong\ncommon dangerous fine fix fixed happening happened knackered normal repair repaired ruined ruin\nsalvage salvaged save saved serious survive survived survives worrying worried worth next\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__ROWS_CACHE(py_dict(vec![]));
    s__ROWS_KEEP(Py::Int(40));
    s_ASK_NAME_CAP(Py::Int(40));
    s_ASK_PLAIN_CAP(Py::Int(400));
    s_ASK_FRAME_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    what when where which who why how should could would can will shall may might must\n    is are was were be been being am do does did done doing have has had having\n    get gets got getting need needs want wants like likes make makes made take takes\n    go goes going put give gives tell say says know think use uses try tries\n    good best better bad worse right wrong sure possible worth\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN_MARGIN(Py::Int(3));
    s_SUBSTITUTE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("substitute"), py_str("substitutes"), py_str("substitution"), py_str("instead"), py_str("swap"), py_str("alternative"), py_str("alternatives"), py_str("replace"), py_str("replacement")])]).unwrap_or_else(|e| exc::raise(e)));
    s_RETURNED_AFTER(call_builtin("frozenset", &[mcall(&py_str("\n    is are was were be been am\n    im i'm youre you're hes he's shes she's theyre they're were we're\n    come comes came coming\n    go goes going went gone\n    get gets getting got gotten\n    walk walks walking walked\n    drive drives driving drove driven\n    fly flies flying flew flown\n    run runs running ran\n    head heads heading headed\n    move moves moving moved\n    travel travels travelling traveled\n    carry carries carrying carried\n    bring brings bringing brought\n    take takes taking took taken\n    send sends sending sent\n    call calls calling called\n    write writes writing wrote written\n    put puts putting\n    hand hands handing handed\n    pay pays paying paid\n    give gives giving gave given\n    kick kicks kicking kicked\n    sit sits sitting sat\n    lie lies lying lay lain\n    step steps stepping stepped\n    look looks looking looked\n    turn turns turning turned\n    think thinks thinking thought\n    bounce bounces bouncing bounced\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BODY_WORDS(crate::gen::understanding::lexicon::g_BODY_PARTS());
    s_GENERAL_BODY(call_builtin("frozenset", &[py_tuple(vec![py_str("skin"), py_str("muscle"), py_str("muscles")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPECIFIC_PARTS(Py::Int(2));
    s_PART_INSIDE(py_dict(vec![(py_str("heel"), py_str("foot")), (py_str("ankle"), py_str("leg")), (py_str("knee"), py_str("leg")), (py_str("hip"), py_str("leg")), (py_str("thumb"), py_str("hand")), (py_str("finger"), py_str("hand")), (py_str("knuckle"), py_str("hand")), (py_str("wrist"), py_str("hand")), (py_str("elbow"), py_str("arm")), (py_str("shoulder"), py_str("arm")), (py_str("jaw"), py_str("head")), (py_str("tooth"), py_str("head")), (py_str("ear"), py_str("head")), (py_str("eye"), py_str("head")), (py_str("rib"), py_str("chest")), (py_str("belly"), py_str("stomach"))]));
    s__CUT_BESIDE(call_builtin("frozenset", &[py_tuple(vec![py_str("lamb"), py_str("beef"), py_str("pork"), py_str("chicken"), py_str("turkey"), py_str("duck"), py_str("goose"), py_str("gammon"), py_str("veal"), py_str("venison"), py_str("meat"), py_str("steak"), py_str("joint"), py_str("roast"), py_str("roasting"), py_str("brisket"), py_str("mince"), py_str("sausage"), py_str("sausages"), py_str("ham"), py_str("bacon"), py_str("chops"), py_str("chop"), py_str("fillet"), py_str("breast"), py_str("thigh"), py_str("drumstick"), py_str("wing")])]).unwrap_or_else(|e| exc::raise(e)));
    s__STILL_GOOD_ASHERS(call_builtin("frozenset", &[py_tuple(vec![py_str("ok"), py_str("okay"), py_str("safe"), py_str("eat"), py_str("eats"), py_str("eating"), py_str("drink"), py_str("use"), py_str("still"), py_str("good"), py_str("gone"), py_str("off"), py_str("bin"), py_str("throw"), py_str("edible"), py_str("risky")])]).unwrap_or_else(|e| exc::raise(e)));
    s__KEEPING_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("kept"), py_str("keeping"), py_str("last"), py_str("lasts"), py_str("store"), py_str("stored"), py_str("storage"), py_str("freeze"), py_str("frozen"), py_str("reheat"), py_str("reheated"), py_str("mouldy"), py_str("sour"), py_str("slimy"), py_str("stale"), py_str("gone"), py_str("date"), py_str("bin"), py_str("throw"), py_str("overnight"), py_str("left")])]).unwrap_or_else(|e| exc::raise(e)));
    s__KEEPING_ANSWER(py_tuple(vec![py_str(" keeps "), py_str(" keep it "), py_str(" keep them "), py_str(" keeping it "), py_str(" in the fridge"), py_str(" in the freezer"), py_str(" in a sealed"), py_str(" in an airtight"), py_str("bin it"), py_str("bin them"), py_str("throw it"), py_str("throw them"), py_str("no more than"), py_str("shelf life"), py_str("hours"), py_str("days in the"), py_str("a day in the"), py_str("day in the fridge"), py_str("within a day"), py_str("within the hour"), py_str("goes off"), py_str("gone off"), py_str("until the date"), py_str("use-by"), py_str("use by"), py_str("best before"), py_str("cold box"), py_str("cool it"), py_str("cool it fast")]));
    s__A_KEEP_AND_A_LENGTH(stdlib_call("re", "compile", &[py_str("\\bkeep(?:s)?\\s+(?:about|for|up to|until|till|roughly|around|no more than|a day|a week|a fortnight|a month|a year|two|three|four|five|six|seven|eight|ten|twelve|\\d)")]).unwrap_or_else(|e| exc::raise(e)));
    s_COLOUR_WORDS(call_builtin("frozenset", &[mcall(&py_str("red orange yellow green blue indigo violet purple pink brown black white\n    grey gray beige cream tan navy gold golden silver amber turquoise lilac mauve crimson scarlet\n    burgundy maroon olive khaki teal cyan magenta peach coral rust blonde fair plum cherry lemon\n    lime salmon rose chestnut copper ochre ebony ivory pearl"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COLOUR_USED(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|am|be|been|being|looks?|looked|seems?|seemed|got|gets|getting|goes|going|went|gone|turn(?:s|ed|ing)?|becom(?:e|es|ing)|became|keep|keeps|kept|painted|paint|dyed|dye|stained?)\\s+(?:very\\s+|really\\s+|so\\s+|too\\s+|quite\\s+|a bit\\s+|a little\\s+|all\\s+|still\\s+|now\\s+)?(?P<colour>[a-z]+)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_FIRST(call_builtin("frozenset", &[mcall(&py_str("is are am was can could should would will shall may might do does did\n    what which any anything how"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COLOUR_ASKED(stdlib_call("re", "compile", &[modulo(&py_str("\\b(?:is|are|am|was|can|could|should|would|will|shall|may|might|do|does|did)\\b[^.?!]{0,30}?\\b(?:ok|okay|alright|all right|fine|suitable|appropriate|respectful|acceptable|allowed|rude|too much|not done|or|colour|color)\\b|\\b(?:what|which|any|anything|something)\\b[^.?!]{0,30}?\\b(?:wear|wearing|put on|outfit|clothes|colour|color|dress)\\b|\\b(?:%s)\\b[^.?!]{0,12}?\\b(?:or|vs|versus)\\b[^.?!]{0,12}?\\b(?:%s)\\b"), &py_tuple(vec![mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_COLOUR_WORDS()]).unwrap_or_else(|e| exc::raise(e))]), mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_COLOUR_WORDS()]).unwrap_or_else(|e| exc::raise(e))])])), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COLOUR_ASKED_WORDS(bitor(&g_COLOUR_WORDS(), &call_builtin("frozenset", &[py_tuple(vec![py_str("colour"), py_str("color"), py_str("colours"), py_str("colors")])]).unwrap_or_else(|e| exc::raise(e))));
    s_WEARING(call_builtin("frozenset", &[mcall(&py_str("wear wearing wore worn outfit outfits clothes clothing dressed dress dresses\n    suit suits tie frock gown hat hats shirt blouse trousers shoes heels"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OCCASION(stdlib_call("re", "compile", &[py_str("\\b(?:weddings?|funerals?|wakes?|memorial service|christenings?|baptisms?|naming ceremony|proms?|graduations?|black tie|cocktail|interviews?)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DRESS_CODE_AFTER(py_tuple(vec![py_str("tie")]));
    s_AFTER_A_COLOUR_STANDS(call_builtin("frozenset", &[mcall(&py_str("\n    ok okay alright fine acceptable allowed appropriate suitable good bad wrong safe rude risky\n    smart casual formal warm cool nice better best right darker lighter brighter\n    to for at in on with and or too not is was are were should would could can will shall must\n    the a an this that there it i we they he she my our their his her\n    be been being have has had do does did\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WORN_AFTER_A_COLOUR(call_builtin("frozenset", &[mcall(&py_str("\n    wear outfit dress dresses frock gown suit suits tie ties shirt shirts blouse top tops trousers\n    pants skirt skirts jeans shorts leggings socks tights boots shoes trainers sneakers sandals\n    heels flats hat hats cap jumper jumpers cardigan hoodie coat coats jacket blazer scarf gloves\n    cotton linen silk wool satin lace leather denim velvet chiffon knit cashmere\n    ring rings necklace bracelets earrings jewellery jewelry band watch\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__BARE_IN_A_KEY(call_builtin("frozenset", &[mcall(&py_str("a an the and or but if then than that this these those there here i\n    me my mine we us our you your yours he him his she her hers it its they them their is are was\n    were am be been being do does did done have has had got get gets getting go goes went gone\n    going make makes made just so not no yes very really quite too much many more most some any\n    all one two three last next night morning today tonight tomorrow now still also about of to in\n    on at for with from by as up down out off over again ever never always thing things bit lot\n    kind sort what when where which who how why"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_SAME_OCCASION(py_dict(vec![(py_str("funeral"), call_builtin("frozenset", &[mcall(&py_str("funeral funerals wake wakes memorial burial cremation"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("wedding"), call_builtin("frozenset", &[mcall(&py_str("wedding weddings married marriage"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("christening"), call_builtin("frozenset", &[mcall(&py_str("christening christenings baptism baptisms naming"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("prom"), call_builtin("frozenset", &[py_tuple(vec![py_str("prom"), py_str("proms")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("graduation"), call_builtin("frozenset", &[py_tuple(vec![py_str("graduation"), py_str("graduations")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("interview"), call_builtin("frozenset", &[py_tuple(vec![py_str("interview"), py_str("interviews")])]).unwrap_or_else(|e| exc::raise(e)))]));
    s_HOUSEHOLD_WORDS(py_dict(vec![(py_str("we"), py_str("i")), (py_str("us"), py_str("me")), (py_str("our"), py_str("my")), (py_str("ours"), py_str("mine")), (py_str("ourselves"), py_str("myself")), (py_str("weve"), py_str("ive")), (py_str("we'd"), py_str("i'd")), (py_str("we'll"), py_str("i'll")), (py_str("we're"), py_str("i'm")), (py_str("we've"), py_str("i've"))]));
    s_CONTEXT_WORDS(Py::Int(8));
    s_SPECIFIC(Py::Int(3));
    s_WEIGHT_UNITS(call_builtin("frozenset", &[py_tuple(vec![py_str("kg"), py_str("kgs"), py_str("g"), py_str("gram"), py_str("grams"), py_str("kilogram"), py_str("kilograms"), py_str("kilo"), py_str("kilos"), py_str("lb"), py_str("lbs"), py_str("pound"), py_str("pounds"), py_str("oz"), py_str("ounce"), py_str("ounces")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_PRONOUNS(call_builtin("frozenset", &[mcall(&py_str("he she they him them"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPEAKER_ONLY(py_tuple(vec![py_str("i"), py_str("im"), py_str("ive"), py_str("id"), py_str("ill")]));
    s_HANDLING_WORDS(py_tuple(vec![py_str("say"), py_str("saying"), py_str("said"), py_str("tell"), py_str("telling"), py_str("answer"), py_str("answering"), py_str("agree"), py_str("agreeing"), py_str("refuse"), py_str("refusing"), py_str("nod"), py_str("nodding"), py_str("smile"), py_str("smiling"), py_str("laugh"), py_str("laughing"), py_str("apologise"), py_str("apologize"), py_str("explain"), py_str("explaining"), py_str("ask"), py_str("asking"), py_str("keep"), py_str("keeps"), py_str("always"), py_str("never"), py_str("just"), py_str("usually")]));
    s__WEAR_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|how)\\b[^.?!]{0,40}?\\b(?:wear|wearing|put on|dress code)\\b|\\b(?:what|which)\\b[^.?!]{0,25}?\\boutfit\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MISSING_WORDS(stdlib_call("re", "compile", &[py_str("\\b(?:no|not|without|havent|hasnt|hadnt|dont|doesnt|didnt|cant|cannot|isnt|arent|wasnt|wont|lack|lacks|lacking|empty|none|nothing|ran out|run out|out of)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_COMMON_WORD(Py::Int(12));
    s_GONE_OFF_SIGNS(py_tuple(vec![py_str("smells"), py_str("smell"), py_str("smelly"), py_str("smelling"), py_str("stinks"), py_str("stinking"), py_str("slimy"), py_str("gone off"), py_str("went off"), py_str("going off"), py_str("has gone off"), py_str("have gone off"), py_str("gone bad"), py_str("has gone bad"), py_str("has gone slimy"), py_str("have gone slimy"), py_str("out of date"), py_str("past its date"), py_str("past the date"), py_str("past their date"), py_str("use by"), py_str("use-by"), py_str("best before"), py_str("still ok"), py_str("still good"), py_str("still fine"), py_str("ok to eat"), py_str("safe to eat"), py_str("is it off"), py_str("mouldy"), py_str("moldy"), py_str("funny smell"), py_str("off smell"), py_str("bin it"), py_str("bin them"), py_str("throw it away"), py_str("throw them away"), py_str("throw it out"), py_str("throw them out")]));
}
pub fn names_a_thing(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a phrase that fills a hole in a sentence names something.\n\n    Used where the phrase came out of the person's own line. A phrase of grammar alone (\"ok to\",\n    \"not\", \"is it\") fills no hole: the card says its plain answer instead of a broken sentence.\n    A number is not a thing either - \"Cut 3 out for a fortnight\" is the same salad.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(!py_contains(&g_GRAMMAR_ONLY(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asking_with(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let _ = py_str("True when the word at `at` is the line's own question word, not a thing in it.");
    if py_truthy(&Py::Bool(!py_contains(&g_ASKING_MODALS(), &geti(&typed.clone(), &at.clone())))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = lt(&add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASKING_SUBJECTS(), &geti(&typed.clone(), &add(&at.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    return { let __b = gt(&at.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASKING_SUBJECTS(), &geti(&typed.clone(), &sub(&at.clone(), &Py::Int(1))))) } };
    Py::None
}
pub fn saving_for_a_trip(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("False when the holiday in the line is one they are paying for, not one they are owed.");
    line = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&mcall(&g__AFFORD_THE_TRIP(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = mcall(&g__SAVING_VERB(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__SAVING_FOR_A_TRIP(), "search", &[line.clone()]) } }));
    Py::None
}
pub fn the_line(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let _ = py_str("The line as the readers read it, lowercased: what a reply callable checks its ask against.");
    understood = { let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    return mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("plain")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    Py::None
}
pub fn said_text(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line's own words as one string, from whichever reading has them.\n\n    A reading carries `words` and `norm` as *lists*. Joining one is right; taking a list as the\n    fallback for a string is not, and it failed quietly in a guard: `understood.get(\"words\") or\n    understood.get(\"norm\") or \"\"` handed a list to `finditer` on a line whose `words` were empty,\n    and the TypeError came out of the lookup, so the line lost its answer altogether - \"What's\n    15% of 80?\" got \"I don't know enough\" while \"what is 15% of 80\" was fine. This is the one\n    place that reads those two keys as text, so a row cannot make that mistake again.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("words"), py_str("norm")]));
    for __item in __iter1 {
    key = __item.clone();
    taken = mcall(&understood.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[taken.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&taken.clone(), "strip", &[]) } }) {
    return taken.clone();
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[taken.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { taken.clone() } }) {
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&taken.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    }
    return py_str("");
    Py::None
}
pub fn guard_passes(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut guard: Py = Py::None;
    let _ = py_str("The entry's own test of the reading, run so that a broken one stands only itself down.\n\n    A guard is called from inside the matcher, once per row the line reaches, so an exception in\n    it used to take the whole lookup down and the line lost every answer it had, not just this\n    row's. A row that cannot read the line is a row that does not answer it.\n    ");
    guard = call_builtin("getattr", &[item.clone(), py_str("guard"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&guard.clone(), &Py::None))) {
    return Py::Bool(true);
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&guard, &[reading.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn asks_how_often(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks the interval rather than the method.\n\n    Takes the reading (`memory[\"understood\"]`) or the line already flattened by `the_line`.\n    ");
    said = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { the_line(&[understood.clone()]) };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HOW_OFTEN_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_sanding_job(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is about sanding something down, not about buying a bag of sand.\n\n    Takes the reading (`memory[\"understood\"]`) or the line already flattened by `the_line`.\n    ");
    said = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { the_line(&[understood.clone()]) };
    return call_builtin("bool", &[mcall(&g_SANDING_JOB(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
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
    let mut words: Py = py_list(__pos.get(0..).unwrap_or(&[]).to_vec());
    let mut lead: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The phrases that make a one-word report of `words` reach an entry.\n\n        entry(\"blocked-nose\", ((\"nose\", \"sinuses\") + said(\"stuffy\", \"bunged up\"), (...)),\n              \"A blocked nose is swollen lining and mucus, so ...\")\n\n    so \"im stuffy\" and \"i am bunged up\" reach the answer that \"my nose is stuffy\" already had.\n    ");
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&leads.clone()) { let mut lead: Py = __item.clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&lead.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&word.clone(), -1, "")); py_string(__s) }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_appliance_swallows_the_name(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut appliance: Py = Py::None;
    let mut name: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when a named appliance contains one of `names` in its own name.\n\n    \"how much does a heat pump dryer cost\" names the appliance `heat pump dryer`, which holds the\n    row's name `heat pump`: the dryer's price is the question. \"how much does a plumber charge to\n    fit a dishwasher\" names an appliance the row's names are not inside, so the plumber's price\n    keeps its line.\n    ");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text"), py_str("norm")])) { let mut part: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[part.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    said = add(&add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[parts.clone()]), "lower", &[])), &py_str(" "));
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = names.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[{ let __b = name.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&names.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&g_APPLIANCE_NAMES());
    for __item in __iter1 {
    appliance = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[add(&add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[appliance.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])")), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&appliance.clone(), &name.clone())); if !py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[appliance.clone()]).unwrap_or_else(|e| exc::raise(e))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_back_is_a_body_part(args: &[Py]) -> Py {
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
    let mut above: Py = Py::None;
    let mut both: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("False when every \"back\" in the two lines is part of the house or the car.\n\n    \"my back fence is rotten\" then \"the dog keeps digging under it\" is a fence question, and the\n    manual-handling card is the wrong answer to it. A line that names the back itself - \"my back\n    hurts\", \"the lower back\", \"a bad back\" - keeps its answer whatever came before.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    above = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("leans_on")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    both = add(&add(&said.clone(), &py_str(" ")), &above.clone());
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_BACK_OF_THE_HOUSE(), "search", &[both.clone()])))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_THE_BACK_ITSELF(), "search", &[both.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _avoid_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut parts: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "parts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: parts"))) };
    let mut flat: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("The avoid list as a flat tuple of words.\n\n    A module often shares these as a constant (\"INFANT_RED_FLAGS\"), and dropping that tuple into\n    avoid= instead of adding it made an entry whose words were a tuple, which no word of a line\n    can ever equal; the matcher only broke when it reached for the text of one. Flatten at the\n    door, so a shared constant works whether it is added or nested, and so nothing that is not a\n    word can reach the matcher.\n    ");
    flat = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&parts.clone());
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&flat.clone(), "extend", &[mcall(&g_MODAL_PHRASES(), "get", &[part.clone(), py_tuple(vec![part.clone()])])]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&flat.clone(), "extend", &[_avoid_words(&[part.clone()])]);
    }
    }
    }
    return call_builtin("tuple", &[flat.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn about_them(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut template: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "template") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: template"))) };
    let mut male: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "male") { v.clone() }
    else { g_PEOPLE_MALE() };
    let mut female: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "female") { v.clone() }
    else { g_PEOPLE_FEMALE() };
    let mut reply: Py = Py::None;
    let _ = py_str("An Entry reply that says he, she or they according to the line.\n\n    template uses {they} {them} {their} for the words and {has} {is} {likes} {loves} {self} for\n    the verb and reflexive forms, so the same sentences work for one person, for a man, for a\n    woman, and for a line that names nobody (\"a present for someone into gardening\" is \"they\").\n    The first letter is capitalised so a template can be written the way it reads.\n    ");
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut female: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut male: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut template: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut reading: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "reading").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut fills: Py = Py::None;
    let mut out: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut they: Py = Py::None;
    let mut token: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    words = { let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    they = py_str("they");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&male.clone(), &word.clone()))) {
    they = py_str("he");
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&female.clone(), &word.clone()))) {
    they = py_str("she");
    break;
    }
    }
    plural = Py::Bool(py_eq(&they.clone(), &py_str("they")));
    fills = py_tuple(vec![py_tuple(vec![py_str("{they}"), they.clone()]), py_tuple(vec![py_str("{them}"), if py_truthy(&plural.clone()) { py_str("them") } else { if py_truthy(&Py::Bool(py_eq(&they.clone(), &py_str("he")))) { py_str("him") } else { py_str("her") } }]), py_tuple(vec![py_str("{their}"), if py_truthy(&plural.clone()) { py_str("their") } else { if py_truthy(&Py::Bool(py_eq(&they.clone(), &py_str("he")))) { py_str("his") } else { py_str("her") } }]), py_tuple(vec![py_str("{self}"), if py_truthy(&plural.clone()) { py_str("themselves") } else { if py_truthy(&Py::Bool(py_eq(&they.clone(), &py_str("he")))) { py_str("himself") } else { py_str("herself") } }]), py_tuple(vec![py_str("{has}"), if py_truthy(&plural.clone()) { py_str("have") } else { py_str("has") }]), py_tuple(vec![py_str("{is}"), if py_truthy(&plural.clone()) { py_str("are") } else { py_str("is") }]), py_tuple(vec![py_str("{likes}"), if py_truthy(&plural.clone()) { py_str("like") } else { py_str("likes") }]), py_tuple(vec![py_str("{loves}"), if py_truthy(&plural.clone()) { py_str("love") } else { py_str("loves") }])]);
    out = template.clone();
    let __iter2 = py_iter_or_raise(&fills.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    token = __unpk3[0].clone();
    word = __unpk3[1].clone();
    out = mcall(&out.clone(), "replace", &[token.clone(), word.clone()]);
    }
    return add(&mcall(&slice_or_raise(&out.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&out.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
    reply = py_func("reply", move |args: &[Py]| { let mut full = vec![female.clone(), male.clone(), reply.clone(), template.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) });
    return reply.clone();
    Py::None
}
pub fn asks_where(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks where to get something, which is a place and not a verdict.\n\n    \"Where can i buy a cheap freezer\" and \"cheapest place to buy a tumble dryer\" ask the same\n    question as \"where can i buy a kettle\", and the shopping domain's `where-to-buy-cheapest`\n    answers all three. The verdict rows never saw them: an appliance name and the bare word\n    \"buy\" are all those need, so a freezer was told it is worth it if you use it three times a\n    week, and a dryer the trade on dryers. A place is what the person asked for, so this is\n    what stands a verdict aside - the reading's own `where`, and the buying-place phrases for\n    the people who ask it without saying where.\n    ");
    if py_truthy(&Py::Bool(py_contains(&call_builtin("tuple", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("where")))) {
    return Py::Bool(true);
    }
    words = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    text = add(&add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[])), &py_str(" "));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WHERE_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_where_a_shop_answers(args: &[Py]) -> Py {
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
    let mut ref_: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks where to get one of the things the price answer is written for.\n\n    This is the guard the verdict rows use, and it is the same test the buying row applies to\n    itself, so a verdict only steps aside when a better answer really takes the line. Asking where\n    is not enough on its own: standing a verdict aside for a thing the buying row does not name\n    left the person with a shrug, or with whatever unrelated row scored next (\"where can i buy a\n    coffee machine\" was told to compare the price per kilo).\n\n    A part is not the thing: \"where can i buy a hoover filter\" and \"where do i get a new oven\n    element\" are pieces, and advice about comparing models, warranties and ex-display stock is\n    written for the machine. The same goes for a bare follow-up: \"where do i get one\" three turns\n    into a fridge door seal is the seal, so what the reading resolved it to has to end in one of\n    these things - \"tumble dryer\" does, \"fridge door\" does not.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&asks_where(&[understood.clone()])))) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_WHERE_PART_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    text = add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[words.clone()])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PRICE_COMPARED()) { let mut thing: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &thing.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    ref_ = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("ref")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("it")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&ref_.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PRICE_COMPARED()) { let mut thing: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&ref_.clone(), &thing.clone())); if py_truthy(&__b) { __b } else { mcall(&ref_.clone(), "endswith", &[add(&py_str(" "), &thing.clone())]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _bounded(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut got: Py = Py::None;
    let _ = py_str("The compiled pattern finding `name` as a whole word, worked out once for the phrase.");
    got = mcall(&g__BOUNDED(), "get", &[name.clone()]);
    if py_truthy(&Py::Bool(!py_is(&got.clone(), &Py::None))) {
    return got.clone();
    }
    got = stdlib_call("re", "compile", &[add(&add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])"))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&ge(&call_builtin("len", &[g__BOUNDED()]).unwrap_or_else(|e| exc::raise(e)), &g__BOUNDED_KEEP())) {
    let _ = mcall(&g__BOUNDED(), "clear", &[]);
    }
    let _ = py_setitem(&g__BOUNDED(), &name.clone(), got.clone());
    return got.clone();
    Py::None
}
pub fn the_fault(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut template: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "template") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: template"))) };
    let mut when_unnamed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when_unnamed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when_unnamed"))) };
    let mut table: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "table") { v.clone() }
    else { g_THE_FAULT() };
    let mut noun: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("An Entry reply that names the broken thing the person actually said.\n\n    template carries {fault}, which becomes the noun from `table` that the line names, so a\n    letter, a first check or a \"who to call\" answer can be about the person's own problem.\n    `when_unnamed` is used instead when the line names no thing at all (\"the landlord is ignoring\n    me about repairs\"). The generic words are deliberately not in the table: \"repair\", \"fault\" and\n    \"everything\" are longer than \"damp\" and \"oven\", so they would win on a line that names both,\n    and a letter about \"the repairs\" is the vagueness the person already had. Longest word first, and only the first fault named, because the\n    letter quotes one and the person adds the rest.\n    ");
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(table.clone()); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| {  let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); call_builtin("len", &[geti(&pair.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)) })), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk5797 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk5797[0].clone(); let mut noun: Py = __unpk5797[1].clone(); __out.push(py_tuple(vec![stdlib_call("re", "compile", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b"))]).unwrap_or_else(|e| exc::raise(e)), noun.clone()])); } py_list(__out) })() };
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut noun: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut table: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut template: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut wanted: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut when_unnamed: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(7).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut reading: Py = __pos.get(8).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "reading").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut line: Py = Py::None;
    let mut pattern: Py = Py::None;
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter1 = py_iter_or_raise(&wanted.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    noun = __unpk2[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[line.clone()])) {
    return mcall(&template.clone(), "replace", &[py_str("{fault}"), noun.clone()]);
    }
    }
    return when_unnamed.clone();
    Py::None
}
    reply = py_func("reply", move |args: &[Py]| { let mut full = vec![noun.clone(), reply.clone(), table.clone(), template.clone(), wanted.clone(), when_unnamed.clone(), word.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) });
    return reply.clone();
    Py::None
}
pub fn a_thing_has_stopped(args: &[Py]) -> Py {
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
    let mut entity: Py = Py::None;
    let mut line: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut things: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when what stopped working is a thing in the line, not the person saying it.\n\n    For \"my oven has stopped working, what do i do\" and \"the washing machine has packed in\" the\n    subject is the machine, and the answer is the plug, the fuse and the door switch. For \"i have\n    stopped working\", \"i gave up work\" and \"im retired and feel lost\" there is no thing in the\n    line, so nothing is taken away from the rows about a person's own working life. A machine the\n    person only mentions in passing (\"i lost my job and the oven has stopped working\") is not the\n    subject and does not count, because the subject has to be one of the things named.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    words = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } } } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    line = add(&add(&py_str(" "), &mcall(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)) } }, "lower", &[])), &py_str(" "));
    line = mcall(&mcall(&line.clone(), "replace", &[py_str(" | "), py_str(" ")]), "replace", &[py_str("|"), py_str(" ")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STOPPED_WORKING()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &modulo(&py_str(" %s "), &phrase.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    things = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut entity: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[entity.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&entity.clone(), "get", &[py_str("type")]), &py_str("thing"))) } }) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&entity.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&things.clone()))) {
    return Py::Bool(false);
    }
    subject = mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("subject")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&things.clone()) { let mut thing: Py = __item.clone(); __out.push({ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&subject.clone(), &thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&thing.clone(), &subject.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_thing_has_gone(args: &[Py]) -> Py {
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
    let mut after_gone: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether \"gone\"/\"went\" is what the thing did, rather than the state it went into.\n\n    True for \"my washing machine has gone and it is not a year old\", \"the seal on my fridge door\n    has gone\" and \"my kettle's gone\"; false for \"the telly has gone black\", \"the milk has gone\n    off\", \"the soup went cold\" and \"the picture has gone funny\", which name what came after, and\n    false for \"we went out and left the stew on the hob\", which is nobody's fault.\n    ");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    line = add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[said.clone()])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    line = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    }
    after_gone = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_HAS_GONE(), "finditer", &[line.clone()])) { let mut found: Py = __item.clone(); __out.push(mcall(&found.clone(), "group", &[Py::Int(1)])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&after_gone.clone());
    for __item in __iter1 {
    tail = __item.clone();
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&tail.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str("'.,!?")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GONE_INTENSIFIER(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_GONE_STATE(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn food_has_gone_off(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut between: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut pos: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line says the food it names has gone off.\n\n    \"my chicken smells a bit, can i cook it\", \"the milk has gone sour\", \"is my chicken still ok\n    to eat\". The rows that are about something else - a smell in the room, what is in a food -\n    stand down on it, and `bot/knowledge/still_ok.py` answers.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    words = tokens(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_FOOD_NAMED(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&g_OFF_STATE_ASKS());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &phrase.clone()))) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    pos = __unpk3[0].clone();
    word = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_FOOD_NAMED(), &word.clone()))) {
    continue;
    }
    at = add(&pos.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = lt(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BEFORE_OFF(), &geti(&words.clone(), &at.clone()))) } }) {
    let __aug4 = add(&at.clone(), &Py::Int(1));
    at = __aug4;
    }
    if py_truthy(&ge(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    continue;
    }
    said = geti(&words.clone(), &at.clone());
    if py_truthy(&Py::Bool(py_eq(&said.clone(), &py_str("bad")))) {
    between = slice_or_raise(&words.clone(), &add(&pos.clone(), &Py::Int(1)), &add(&at.clone(), &Py::Int(1)), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__BAD_IS_A_STATE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_OFF_SAID_OF_FOOD(), &said.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_SMELL_VERBS(), &said.clone()))) {
    after = slice_or_raise(&words.clone(), &add(&at.clone(), &Py::Int(1)), &add(&at.clone(), &Py::Int(2)), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_GOOD_SMELL(), &geti(&after.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn a_price_not_a_loan(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when a price row may answer: the line is a discount and not money off a person.\n\n    `guard=` on every entry that reads a price, so the whole lane agrees on the one rule instead\n    of each row fencing its own phrases. \"should i borrow money off my mum\", \"can i get twenty\n    quid off my dad\", \"is it ok to borrow money off my parents\" and the six other ways of saying\n    it are the borrowing rows' lines, and they fall through to those rows when this says no.\n    ");
    said = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = call_builtin("str", &[{ let __b = mcall(&said.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&said.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    text = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&said.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&{ let __b = mcall(&g__BORROWING_IN_THE_LINE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__OFF_A_PERSON(), "search", &[text.clone()]) } }));
    Py::None
}
pub fn not_the_heat_of_an_argument(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line's \"heated\" is people or a row rather than a room or a night.\n\n    A guard for the rows that read a temperature, so the words stay in the knowledge base: the\n    reading of the line comes from the understanding engine, and this only asks whether the\n    word in this line is the warmth the row is about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__THE_HEAT_OF_AN_ARGUMENT(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn the_ask_is_the_order_of_steps(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks what to do first rather than whether to do a thing.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__ORDER_OF_STEPS(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn not_the_ask_about_the_order(args: &[Py]) -> Py {
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
    let _ = py_str("For a row that answers a decision: an ask for the order is not a decision.");
    return Py::Bool(!py_truthy(&the_ask_is_the_order_of_steps(&[understood.clone()])));
    Py::None
}
pub fn not_the_flat_you_live_in(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line's \"flat\" is the home, for a row that keys flat as a bake or a hair.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__FLAT_YOU_LIVE_IN(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn not_the_flat_of_something_made(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line's \"flat\" is how a thing somebody made turned out.\n\n    A guard for the row that reads flat as the ground: \"the bread came out flat\" is the loaf, and\n    the bake rows have their own answers for it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    said = call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bflat\\b"), said.clone(), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_BAKE_OR_A_PAN(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__CAME_OUT_FLAT(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__THE_GROUND_UNDERFOOT(), "search", &[said.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn the_years_are_a_things(args: &[Py]) -> Py {
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
    let mut figures: Py = Py::None;
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let mut window: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when every figure in years on the line belongs to a thing and not to a person.");
    text = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    text = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    }
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    figures = call_builtin("list", &[mcall(&g__YEAR_OLD_FIGURE(), "finditer", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&figures.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&figures.clone());
    for __item in __iter1 {
    found = __item.clone();
    window = slice_or_raise(&text.clone(), &call_builtin("max", &[Py::Int(0), sub(&mcall(&found.clone(), "start", &[]), &Py::Int(60))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&found.clone(), "start", &[]), &Py::None);
    if py_truthy(&mcall(&g__PEOPLE_AGED(), "search", &[window.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__A_THING_SAID_IS(), "search", &[window.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__A_THING_JUST_BEFORE(), "search", &[window.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_THING_POINTED_AT(), "search", &[window.clone()]) } } } }))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_age_is_a_childs(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut a_person: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut known: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut stack: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a row's own key says the figure in years belongs to a child.\n\n    Rows that answer a parent's line carry the child in their key - `bike-child-seat` has \"child\",\n    \"kid\", \"son\", \"daughter\", \"toddler\", \"boy\", \"girl\" beside \"year old\" - and those rows are the\n    ones that read the age of the person's own bike or handset as a child's. Rows that answer a\n    thing's own line, a fridge that is 12 years old and wants replacing, name a thing and no\n    person, so the same test leaves them alone.\n    ");
    known = mcall(&g__CHILDS_AGE_ROWS(), "get", &[gattr(&item.clone(), "id")]);
    if py_truthy(&Py::Bool(!py_is(&known.clone(), &Py::None))) {
    return known.clone();
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), py_list(vec![])]), 2, None);
    words = __unpk1[0].clone();
    phrases = __unpk1[1].clone();
    stack = call_builtin("list", &[{ let __b = gattr(&item.clone(), "key"); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&stack.clone()) {
    part = mcall(&stack.clone(), "pop", &[]);
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    plain = mcall(&mcall(&part.clone(), "lower", &[]), "replace", &[py_str("'s"), py_str("s")]);
    let _ = mcall(&words.clone(), "update", &[mcall(&plain.clone(), "split", &[])]);
    let _ = mcall(&phrases.clone(), "append", &[plain.clone()]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&stack.clone(), "extend", &[part.clone()]);
    }
    }
    }
    a_person = { let __b = call_builtin("bool", &[mcall(&g__CHILD_WORDS(), "intersection", &[words.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__CHILD_PHRASES(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    answer = { let __b = a_person.clone(); if !py_truthy(&__b) { __b } else { _key_says_years_old(&[item.clone()]) } };
    let _ = py_setitem(&g__CHILDS_AGE_ROWS(), &gattr(&item.clone(), "id"), answer.clone());
    return answer.clone();
    Py::None
}
pub fn _key_says_years_old(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut part: Py = Py::None;
    let mut stack: Py = Py::None;
    let _ = py_str("True when any string in the row's key holds a figure in years.");
    stack = call_builtin("list", &[{ let __b = gattr(&item.clone(), "key"); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&stack.clone()) {
    part = mcall(&stack.clone(), "pop", &[]);
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&mcall(&g__A_YEARS_OLD(), "search", &[part.clone()])) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&stack.clone(), "extend", &[part.clone()]);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn not_keeping_an_eye_on_it(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line's \"eye\" is the watching rather than the thing that hurts.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__KEEPING_AN_EYE(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn _the_device_is_the_one_falling_asleep(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut bare: Py = Py::None;
    let mut device: Py = Py::None;
    let mut index: Py = Py::None;
    let mut person: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the words in front of a sleep shape name a device rather than the person.");
    device = neg(&Py::Int(1));
    person = neg(&Py::Int(1));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[mcall(&slice_or_raise(&said.clone(), &Py::None, &start.clone(), &Py::None), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    bare = mcall(&mcall(&word.clone(), "strip", &[g__SLEEPY_MARKS()]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_A_DEVICE_THAT_SLEEPS(), &bare.clone()))) {
    device = index.clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&g__A_PERSON_WHO_SLEEPS(), &bare.clone()))) {
    person = index.clone();
    }
    }
    }
    return { let __b = ge(&device.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { ge(&device.clone(), &person.clone()) } };
    Py::None
}
pub fn the_body_is_short_of_sleep(args: &[Py]) -> Py {
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
    let mut shape: Py = Py::None;
    let _ = py_str("Whether the line says the person's body is short of sleep, in the words that say it.");
    said = _the_whole_line(&[understood.clone()]);
    if py_truthy(&mcall(&g_NOT_TIRED_AFTER_ALL(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_SHORT_OF_SLEEP(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    shape = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&_the_device_is_the_one_falling_asleep(&[said.clone(), mcall(&shape.clone(), "start", &[])])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn not_a_sleepy_body(args: &[Py]) -> Py {
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
    let _ = py_str("A `guard=` test: True unless the line is a body that is short of sleep.\n\n    For the rows that answer where somebody is sitting or what they are looking at. \"i keep\n    nodding off at my desk\" is not a chair problem and \"my eyes keep closing at the screen\" is not\n    eye strain, so those rows stand aside and the tired answer takes the turn.\n    ");
    return Py::Bool(!py_truthy(&the_body_is_short_of_sleep(&[understood.clone()])));
    Py::None
}
pub fn all_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut guards: Py = py_list(__pos.get(0..).unwrap_or(&[]).to_vec());
    let mut guards: Py = Py::None;
    let mut test: Py = Py::None;
    let _ = py_str("One `guard=` test from several, true only when every one of them says the row may answer.\n\n    A row can have more than one thing to stand aside for - the chair row already keeps out of\n    place-shaped asks and now keeps out of a sleepy body too - and `entry()` takes one guard, so\n    they are combined here rather than by wrapping one in a lambda at the row.\n    ");
pub fn __nested_test_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut guards: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut test: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut guard: Py = Py::None;
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&guards.clone()) { let mut guard: Py = __item.clone(); if py_truthy(&guard.clone()) {  __out.push(call_builtin("bool", &[py_call(&guard, &[understood.clone()])]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
    test = py_func("test", move |args: &[Py]| { let mut full = vec![guards.clone(), test.clone()]; full.extend_from_slice(args); __nested_test_0(&full) });
    return test.clone();
    Py::None
}
pub fn _the_whole_line(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The line as the person typed it, in one string, for a guard on the whole reading.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return mcall(&call_builtin("str", &[{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("clean")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[])))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return mcall(&said.clone(), "lower", &[]);
    Py::None
}
pub fn entry(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut id: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "id") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: id"))) };
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut reply: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut avoid: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "avoid") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bias: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bias") { v.clone() }
    else { Py::Int(0) };
    let mut ask: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { py_tuple(vec![]) };
    let mut slips: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "slips") { v.clone() }
    else { py_tuple(vec![]) };
    let mut fault: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fault") { v.clone() }
    else { Py::Bool(false) };
    let mut report: Py = if __pos.len() > 8 { __pos[8].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "report") { v.clone() }
    else { Py::Bool(false) };
    let mut guard: Py = if __pos.len() > 9 { __pos[9].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "guard") { v.clone() }
    else { Py::None };
    let mut shape: Py = if __pos.len() > 10 { __pos[10].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shape") { v.clone() }
    else { py_tuple(vec![]) };
    let mut explain: Py = if __pos.len() > 11 { __pos[11].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "explain") { v.clone() }
    else { Py::Bool(false) };
    let mut lead: Py = if __pos.len() > 12 { __pos[12].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lead") { v.clone() }
    else { py_str("") };
    let mut outcomes: Py = if __pos.len() > 13 { __pos[13].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "outcomes") { v.clone() }
    else { py_tuple(vec![]) };
    let _ = py_str("Build an Entry. key: words, phrases, or tuples of alternatives for one key.\n\n    bias: a nudge when two entries fit a line equally well, for the more specific one\n    (a named dish or ingredient beats a general technique). Use 1, rarely 2.\n    ask: optional words or phrases that must also appear, for an entry that is only\n    an answer when the line is actually asking (a factual \"how many weeks in a year\"\n    should not fire on \"I'm away for three weeks in a year\", let alone on a month\n    named in passing). Empty means the key alone is enough.\n    slips: key words that are misspellings of another key word, not words of their own.\n    A table of hard spellings has to hold \"adress\" next to \"address\", and if both count as\n    words then the slip repair around them sees two near-miss neighbours in two families and\n    gives up: \"who do i tel about my new adress after moving\" stopped reaching the address\n    entry and got the shrug. A slip still matches its own entry; it is only kept out of the\n    vocabulary that decides what is a word and what is a misspelling.\n    report: the complaint this answer is about can be *stated* as well as asked about, so a\n    line that is nothing but \"i have <subject>\", \"ive got <subject>\" or \"i keep getting\n    <subject>\" is a request for it. Opt in only on rows where naming the complaint is the\n    question (a symptom in your own body), never on a row whose subject is a thing somebody\n    might own or want: \"i have a dog\" is not a question about looking after one.\n    guard: optional `fn(understood) -> bool`; the entry answers only when it returns true.\n    Use it where the reading, not a word in the line, says the key means something else.\n    shape: callables given the line's words as one string, each True when the line holds a\n    figure of the kind this row is about (\"4%\" for a phone's charge). A shape that matches is\n    one signal - the same weight as a key word - so the row still needs its subject in the\n    line, and a row with no shape is read exactly as before.\n    lead: the sentence the row owes a line that wants one shape of answer, in the row's own words\n    (\"Usually, yes\", \"About 20 minutes\"). `bot/compose/lead.py` puts it in front when the line's\n    `wants` is the shape the sentence gives and the card does not already open with it; a card\n    that opens with the answer, and every other shape of line, is left exactly as it is. Use it\n    on a card that is written for the harder question and has the plain one to answer as well.\n    explain: the row answers a named thing, so a plain request to explain it is its question:\n    \"what is sepsis\", \"tell me about gout\", \"explain shingles\". The ask has to sit in front of\n    the subject with nothing but the ask's own words between (\"what is the best way to clean a\n    sofa\" is a request for a method, not for a sofa), and the subject counts as one signal the\n    way a stated complaint does.\n    ");
    return Entry(&[id.clone(), key.clone(), reply.clone(), _avoid_words(&[avoid.clone()]), bias.clone(), ask.clone(), slips.clone(), fault.clone(), report.clone(), guard.clone(), shape.clone(), explain.clone(), lead.clone(), outcomes.clone()]);
    Py::None
}
pub fn asks_the_kinds(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says_it: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks which families a thing comes in.\n\n    Two ways a line asks it, and both are the same shape:\n\n    - in its own words, with a kinds word and no use named (\"what kinds of tea are there\",\n      \"the different types of potato\", \"what sort of apple is there\");\n    - leaning on the kinds question before it, which carries no kinds word of its own (\"and\n      rice\", \"what about apples\"). The shape is kept in `memory[\"understood\"][\"wants\"]` by\n      bot/modules/intent.py, so no phrase list has to guess at it here.\n\n    One reading, used two ways: a kinds row answers on it, and the table that answers the next step\n    of a meal stands down on it (the dish follow-up answered \"what types of potato are there\" with\n    a recipe, because the turn before it had named a food).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_KINDS_ASKS_ME(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_KINDS_NAMED_A_USE(), "search", &[said.clone()]))) } }) {
    words = modulo(&py_str(" %s "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&mcall(&mcall(&mcall(&said.clone(), "lower", &[]), "replace", &[py_str("["), py_str(" ")]), "replace", &[py_str("]"), py_str(" ")]), "replace", &[py_str(","), py_str(" ")]), "replace", &[py_str("'"), py_str(" ")]), "split", &[])]));
    lowered = mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]);
    says_it = { let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("intent")]), &py_str("news"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_KINDS_ASKS_WITH_A_VERB(), "search", &[lowered.clone()]))); if !py_truthy(&__b) { __b } else { mcall(&g_KINDS_SAYS_HOW_IT_IS(), "search", &[lowered.clone()]) } } } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&says_it.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KINDS_WORDS()) { let mut kind: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &kind.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    return { let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("wants")]), &g_KINDS_SHAPE())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_KINDS_NAMED_A_USE(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&sub(&call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("what"), py_str("yesno")])))) } } } };
    Py::None
}
pub fn kinds_entries(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rows: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rows") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rows"))) };
    let mut answer: Py = Py::None;
    let mut name: Py = Py::None;
    let mut out: Py = Py::None;
    let mut subjects: Py = Py::None;
    let _ = py_str("Build the `what kinds of X are there` answers for one domain.\n\n    `rows` is data: (the line's subject words, the subject for the row's id, the answer). The\n    answer opens on the families, because that is what was asked. The shape is the guard, not a\n    word in the key, so a follow-up that carries the shape (\"and rice\") reaches the same row, and a\n    line that names a use (\"what type of potato is best for mash\") reaches the row that answers the\n    use instead.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&rows.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    subjects = __unpk2[0].clone();
    name = __unpk2[1].clone();
    answer = __unpk2[2].clone();
    let _ = mcall(&out.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(modulo(&py_str("kinds-of-%s"), &name.clone())); __args.push(py_tuple(vec![g_KINDS_WORDS(), subjects.clone()])); __args.push(answer.clone()); __kws.extend(vec![("guard".to_string(), py_func("asks_the_kinds", |args: &[Py]| asks_the_kinds(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    let _ = mcall(&out.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(modulo(&py_str("kinds-of-%s-told"), &name.clone())); __args.push(py_tuple(vec![g_KINDS_FOLLOW_UP_WORDS(), subjects.clone()])); __args.push(answer.clone()); __kws.extend(vec![("guard".to_string(), py_func("asks_the_kinds", |args: &[Py]| asks_the_kinds(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| entry(args))(&__args) }]);
    }
    return out.clone();
    Py::None
}
pub fn _doubles_for_an_ending(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut stem: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stem") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stem"))) };
    let _ = py_str("True when the final doubled consonant of a stem comes from the ending, not from the word.\n\n    \"stopp\" in \"stopping\" and \"runn\" in \"running\" are one letter short of the word, so the\n    doubled \"p\" and \"n\" go and \"stop\"/\"run\" can still meet a key. \"hiss\", \"add\" and \"egg\"\n    already are words: dropping a letter invents a different one - \"hissing\" was handing \"his\"\n    to every key that said hissing, so \"my dad is tired of his job\" reached the puncture\n    instructions, and \"passed\" handed \"pas\". A doubled \"s\" is always the word's own, and the\n    doubling an ending makes always sits on a consonant after a vowel (\"stop\", \"run\", \"plan\").\n    ");
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&geti(&stem.clone(), &neg(&Py::Int(1))), &geti(&stem.clone(), &neg(&Py::Int(2))))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_str("aeiou"), &geti(&stem.clone(), &neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&stem.clone(), &neg(&Py::Int(1))), &py_str("s"))) } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&stem.clone(), &neg(&Py::Int(1))), &py_str("l")))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_contains(&py_str("aeiou"), &geti(&stem.clone(), &neg(&Py::Int(3)))))) {
    return Py::Bool(false);
    }
    return _is_consonant(&[stem.clone(), sub(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))]);
    Py::None
}
pub fn _is_consonant(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut letter: Py = Py::None;
    let _ = py_str("True for the letter at index, reading the \"u\" of \"qu\" as the consonant it sounds.");
    if py_truthy(&lt(&index.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    letter = geti(&word.clone(), &index.clone());
    if py_truthy(&Py::Bool(!py_contains(&py_str("aeiou"), &letter.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_eq(&letter.clone(), &py_str("u"))); if !py_truthy(&__b) { __b } else { { let __b = gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &sub(&index.clone(), &Py::Int(1))), &py_str("q"))) } } } };
    Py::None
}
pub fn _forms(args: &[Py]) -> Py {
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
    let mut forms: Py = Py::None;
    let mut frozen: Py = Py::None;
    let mut got: Py = Py::None;
    let _ = py_str("The loose forms of a word, so eggs/egg and cooking/cook match each other.\n\n    Two rules keep a loose form honest. A stem has to keep at least MIN_FORM letters to count\n    (short stems are not words of their own: \"weeing\" strips to \"we\", and that one form made\n    the cat litter entry answer \"we had to put our cat down\"). And a form is only handed out\n    when the strip is a real ending of the word and the leftover is not a different word of its\n    own: \"bites\" is not \"bit\", \"cares\" is not \"car\", \"biting\" is not \"bit\". Those reached a key\n    word that has nothing to do with the line (\"bit of a mad day\" was answered as the\n    sibling-hitting entry), and the person's real word still matches its own key, which lists\n    the spelling it answers.\n\n    The result comes from a cache and is a frozenset. The matcher asks for the same key word\n    thousands of times a turn (once per entry that says it), and the forms of a word depend on\n    nothing but the word, so working them out again was most of what a reply cost. A frozenset\n    is returned because `find` writes the typo forms onto a line word with `|=`; on a cached set\n    that would edit the copy for the rest of the conversation, on a frozenset the same line makes\n    a new set.\n    ");
    got = mcall(&g__FORMS_CACHE(), "get", &[word.clone()]);
    if py_truthy(&Py::Bool(!py_is(&got.clone(), &Py::None))) {
    return got.clone();
    }
    forms = _work_out_forms(&[word.clone()]);
    frozen = call_builtin("frozenset", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&ge(&call_builtin("len", &[g__FORMS_CACHE()]).unwrap_or_else(|e| exc::raise(e)), &g__FORMS_CACHE_KEEP())) {
    let _ = mcall(&g__FORMS_CACHE(), "clear", &[]);
    }
    let _ = py_setitem(&g__FORMS_CACHE(), &word.clone(), frozen.clone());
    return frozen.clone();
    Py::None
}
pub fn _work_out_forms(args: &[Py]) -> Py {
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
    let mut forms: Py = Py::None;
    let mut other: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The forms of one word, worked out from the endings and the tables (see _forms).");
    word = mcall(&word.clone(), "translate", &[g__CURLY()]);
    forms = py_set(vec![word.clone()]);
    other = mcall(&crate::gen::understanding::lexicon::g_US_SPELLINGS(), "get", &[word.clone()]);
    if py_truthy(&other.clone()) {
    let _ = mcall(&forms.clone(), "add", &[other.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ies")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    let _ = mcall(&forms.clone(), "add", &[add(&slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(3)), &Py::None), &py_str("y"))]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&word.clone(), &neg(&Py::Int(2))))) } } } }) {
    let _ = mcall(&forms.clone(), "add", &[add(&slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), &py_str("ies"))]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_IRREGULAR_PLURALS(), &word.clone()))) {
    let _ = mcall(&forms.clone(), "add", &[geti(&g_IRREGULAR_PLURALS(), &word.clone())]);
    return forms.clone();
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("'s")]); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__CONTRACTION(), &word.clone())) } } } }) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)]);
    } else {
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s'")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)]);
    let _ = mcall(&forms.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    }
    }
    if py_truthy(&gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ss")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_A_WORD_WITH_AN_S(), &word.clone())) } } } }) {
    let _ = _add(&[forms.clone(), slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("es")]); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_str("sxzi"), &geti(&word.clone(), &neg(&Py::Int(3))))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("ch"), py_str("sh")]), &slice_or_raise(&word.clone(), &neg(&Py::Int(4)), &neg(&Py::Int(2)), &Py::None))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &neg(&Py::Int(3))), &py_str("o"))) } } } } } } } }) {
    let _ = _add(&[forms.clone(), slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ed")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    if py_truthy(&{ let __b = _engine_reads(&[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)]); if !py_truthy(&__b) { __b } else { _engine_reads(&[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]) } }) {
    let _ = _add(&[forms.clone(), slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    } else {
    let _ = _add(&[forms.clone(), slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)]);
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { _doubles_for_an_ending(&[slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None)]) } }) {
    let _ = _add(&[forms.clone(), slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(3)), &Py::None)]);
    }
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    stem = slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(3)), &Py::None);
    if py_truthy(&Py::Bool(!py_contains(&g_NOT_THE_STEM_WITH_E(), &word.clone()))) {
    let _ = _add(&[forms.clone(), add(&stem.clone(), &py_str("e"))]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NOT_THE_STEM(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_BASE(), &stem.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_cvc(&[stem.clone()]))); if py_truthy(&__b) { __b } else { { let __b = { let __b = gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { mcall(&stem.clone(), "endswith", &[py_str("y")]) } }; if py_truthy(&__b) { __b } else { _bare_is_the_word(&[stem.clone()]) } } } } } } } }) {
    let _ = _add(&[forms.clone(), stem.clone()]);
    }
    if py_truthy(&{ let __b = _doubles_for_an_ending(&[stem.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_THE_DOUBLED_STEM(), &word.clone())) } }) {
    let _ = _add(&[forms.clone(), slice_or_raise(&stem.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]);
    }
    }
    }
    return forms.clone();
    Py::None
}
pub fn _add(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut form: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "form") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: form"))) };
    let _ = py_str("Keep a derived form only when it is long enough to be a word in its own right.");
    if py_truthy(&ge(&call_builtin("len", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MIN_FORM())) {
    let _ = mcall(&forms.clone(), "add", &[form.clone()]);
    }
    Py::None
}
pub fn about_the_chat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut own_line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own_line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: own_line"))) };
    let _ = py_str("True when the line asks about this conversation rather than about the world.\n\n    This runs on the words the person typed (\"text\"), not on the reading's words: a follow-up is\n    re-read with the memory's topic words added (\"how did this chat start save car\"), and those\n    are exactly what made a fact answer a question about the chat.\n    ");
    return call_builtin("bool", &[mcall(&g__ABOUT_THE_CHAT(), "search", &[mcall(&py_str(" "), "join", &[tokens(&[call_builtin("str", &[{ let __b = own_line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _bare_is_the_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut stem: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stem") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stem"))) };
    let _ = py_str("Whether a short consonant-vowel-consonant stem may stand for the word that was written.\n\n    \"opening\" -> \"open\" yes: the stem is a word the understanding engine reads and adding an\n    \"e\" to it makes a word nobody writes. \"biting\" -> \"bit\" no: both are words, and the person\n    meant \"bite\". \"hating\" -> \"hat\" no, for the same reason. A function word is never the base\n    of a longer word. When the engine cannot read the stem at all (\"stares\" -> \"star\") it is not\n    a base either, because nobody wrote it. The cost is that a word the engine does not know\n    loses that one reading; the entry that answers it lists the spelling, as a key always can.\n    ");
    return { let __b = _engine_reads(&[stem.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_engine_reads(&[add(&stem.clone(), &py_str("e"))]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__FUNCTION_WORDS(), &stem.clone())) } } } };
    Py::None
}
pub fn _cvc(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut stem: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stem") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stem"))) };
    let mut vowels: Py = Py::None;
    let _ = py_str("Whether a short stem is consonant-vowel-consonant: \"bit\", \"car\", \"hat\", \"hop\".\n\n    A short word of that shape after stripping \"-ing\" is nearly always a different word of its\n    own (\"bit\" for \"biting\", \"car\" for \"caring\") rather than the base of the word that was\n    written, so it needs a second look before it is handed out. Longer stems are left alone:\n    \"gardening\" is \"garden\", and its last three letters being consonant-vowel-consonant says\n    nothing.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(3), &{ let __cmp10 = call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(4)) } else { Py::Bool(false) }))) {
    return Py::Bool(false);
    }
    vowels = py_str("aeiou");
    return { let __b = Py::Bool(!py_contains(&vowels.clone(), &geti(&stem.clone(), &neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&vowels.clone(), &geti(&stem.clone(), &neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&vowels.clone(), &geti(&stem.clone(), &neg(&Py::Int(3))))) } } } };
    Py::None
}
pub fn tokens(args: &[Py]) -> Py {
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
    let _ = py_str("The loose words of a line, lower case, for matching against keys.");
    return mcall(&g_WORD(), "findall", &[mcall(&mcall(&text.clone(), "translate", &[g__CURLY()]), "lower", &[])]);
    Py::None
}
pub fn _past_a_handoff(args: &[Py]) -> Py {
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
    let mut bridge: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut punctuation: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The line after a hand-off at its front, or None when it does not open with one.\n\n    \"one more thing about the boiler\" and \"quick question, how do i descale a kettle\" announce a\n    question and then ask it. The announcement is not what the line is about: nothing in \"one\n    more thing\" should be matched against a key, or an entry keyed on \"one\" and \"thing\" answers a\n    line that was about the boiler. Only a hand-off that is followed by what it announces is\n    dropped, so a bare \"one more thing\" keeps every word it typed and is answered by whoever\n    reads the line itself (bot/compose/handoff.py).\n    ");
    text = mcall(&g__THROAT(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::understanding::lexicon::g_HANDOFF_LEADS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "startswith", &[lead.clone()])))) {
    continue;
    }
    rest = slice_or_raise(&text.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    after = Py::None;
    punctuation = mcall(&g__PUNCTUATION(), "match", &[rest.clone()]);
    if py_truthy(&punctuation.clone()) {
    after = slice_or_raise(&rest.clone(), &mcall(&punctuation.clone(), "end", &[]), &Py::None, &Py::None);
    } else {
    bridge = mcall(&g__BRIDGE(), "match", &[rest.clone()]);
    if py_truthy(&bridge.clone()) {
    after = slice_or_raise(&rest.clone(), &mcall(&bridge.clone(), "end", &[]), &Py::None, &Py::None);
    }
    }
    if py_truthy(&Py::Bool(py_is(&after.clone(), &Py::None))) {
    continue;
    }
    bridge = mcall(&g__BRIDGE(), "match", &[after.clone()]);
    if py_truthy(&bridge.clone()) {
    after = slice_or_raise(&after.clone(), &mcall(&bridge.clone(), "end", &[]), &Py::None, &Py::None);
    }
    return { let __b = mcall(&after.clone(), "strip", &[py_str(" ,.:;\t–—")]); if py_truthy(&__b) { __b } else { Py::None } };
    }
    return Py::None;
    Py::None
}
pub fn _past_the_carer(args: &[Py]) -> Py {
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
    let mut left: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing wrong with the person they look after, or None when the line is not that shape.\n\n    \"my daughter has head lice\" is a question about head lice; whoever types it wants the comb and\n    the conditioner, not a word about how hard caring is. Only the words that name the child and\n    the having are dropped, so a line that also says what they want - \"my son has chickenpox and\n    wont eat\" - is read on its own remaining words like any other line. A state of somebody's life\n    is not an illness and is left to the reader that knows it (`AN_ILLNESS`).\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    found = mcall(&g_CARER_FRAME(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    left = mcall(&slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" ,.:;\t")]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&left.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&mcall(&word.clone(), "strip", &[g__MARKS()])) {  __out.push(mcall(&word.clone(), "strip", &[g__MARKS()])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_AN_ILLNESS(), "fullmatch", &[geti(&words.clone(), &neg(&Py::Int(1)))]); if py_truthy(&__b) { __b } else { { let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&g_AN_ILLNESS(), "fullmatch", &[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return left.clone();
    }
    return Py::None;
    Py::None
}
pub fn _without_a_dead_that_only_means_very(args: &[Py]) -> Py {
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
    let mut following: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line's words with a \"dead\" that describes the next word taken out.");
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    following = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &add(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &py_str("dead"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DEAD_AS_VERY_AFTER(), &mcall(&mcall(&call_builtin("str", &[following.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[g__MARKS()]))) } }) {
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    return kept.clone();
    Py::None
}
pub fn _without_a_cooking_that_is_a_name(args: &[Py]) -> Py {
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
    let mut following: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line's words with a \"cooking\" that names a food taken out.\n\n    \"Cooking oil\" is the name of what is in the bottle, not a statement that something was\n    cooked. Read as the verb it made a shelf-life question into a cooked-food one: the\n    cooked-food table in bot/knowledge/cooking.py keys on \"cook\" and on a keeping word, so\n    \"how do i store cooking oil\" matched it twice and the oil's own keeping row lost the line,\n    while \"how do i store olive oil\" reached the right answer. The participle that says a food\n    was cooked is \"cooked\", which is not a word anyone wears in a name.\n    ");
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    following = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &add(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &py_str("cooking"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COOKING_AS_A_NAME(), &mcall(&mcall(&call_builtin("str", &[following.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[g__MARKS()]))) } }) {
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    return kept.clone();
    Py::None
}
pub fn _with_the_animal_in_hand(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut animal: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut low: Py = Py::None;
    let mut piece: Py = Py::None;
    let _ = py_str("The line's words with the animal it only points at, named where the pronoun stands.\n\n    A follow-up after a turn about a pet is about that pet: \"she's been drinking a lot more\n    water\", \"how much should he eat\", \"hes getting slow on walks\". Nothing in those lines says\n    which animal, so every entry keyed on a cat or a dog missed them and the answer for a person's\n    own body took the turn - the daily figure for drinking water went to a cat, and a dog with a\n    sore ear got the ear card for a person. The understanding engine has already worked out what\n    the pronoun points at (`animal_in_hand`), so the animal goes where the pronoun stands: an\n    entry that matches spans of positions still reads the line as it was typed.\n    ");
    animal = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("animal_in_hand")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_truthy(&animal.clone()))) {
    return words.clone();
    }
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    piece = __item.clone();
    low = geti(&mcall(&mcall(&mcall(&call_builtin("str", &[piece.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[g__MARKS()]), "split", &[py_str("'")]), &Py::Int(0));
    let _ = mcall(&kept.clone(), "append", &[if py_truthy(&Py::Bool(py_contains(&g__THE_PRONOUNS(), &low.clone()))) { animal.clone() } else { piece.clone() }]);
    }
    return if py_truthy(&Py::Bool(!py_eq(&kept.clone(), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))))) { kept.clone() } else { words.clone() };
    Py::None
}
pub fn _the_animal_rules_out_that_card(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut animal: Py = Py::None;
    let mut stood_down_for: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the animal the line only points at is a word the card that took the turn avoids.\n\n    A card carries in its avoid list the words that make it the wrong answer - the figure for a\n    person's water says \"cat\", \"dog\", \"puppy\", \"litter tray\" - so a card whose avoid list holds\n    the animal is a card written for somebody else and reached only because the line says \"she\"\n    rather than \"the cat\".\n    ");
    animal = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("animal_in_hand")]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_truthy(&animal.clone()))) {
    return Py::Bool(false);
    }
    stood_down_for = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = call_builtin("getattr", &[item.clone(), py_str("avoid"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    return { let __b = Py::Bool(py_contains(&stood_down_for.clone(), &animal.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&stood_down_for.clone(), &add(&animal.clone(), &py_str("s")))) } };
    Py::None
}
pub fn _with_the_child_by_place(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut child_word: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut skipping: Py = Py::None;
    let mut small: Py = Py::None;
    let mut word: Py = Py::None;
    let mut word2: Py = Py::None;
    let _ = py_str("The line's words with the child it names by place or by an everyday word, when it does.\n\n    The added word goes at the end: an entry has already matched spans of positions into this\n    list (see `_frame_masked`), so nothing may move. A line that names an animal or a grown-up\n    is left exactly as it came, which is also what keeps \"the little one\" about a dog the dog's\n    question.\n    ");
    line = call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_CHILD_BY_PLACE(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_TWO_OF_US(), "match", &[line.clone()]) } }) {
    return words.clone();
    }
    small = Py::Bool(!py_truthy(&mcall(&g_A_TEEN(), "search", &[line.clone()])));
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[g__MARKS()])); } py_set(__out) })() };
    if py_truthy(&bitand(&said.clone(), &g_NOT_A_CHILD_SAID())) {
    return words.clone();
    }
    kept = py_list(vec![]);
    skipping = Py::Int(0);
    if py_truthy(&{ let __b = mcall(&g_A_CHILD_GETTING_OLDER(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_CHILD_WORD_BY_ITSELF(), "search", &[line.clone()]))) } }) {
    return words.clone();
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    low = mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[g__MARKS()]);
    if py_truthy(&{ let __b = skipping.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&low.clone(), &py_str("one"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&low.clone())) } } } }) {
    skipping = Py::Int(0);
    continue;
    }
    skipping = Py::Int(0);
    child_word = { let __b = Py::Bool(py_contains(&g_CHILD_PLACE_WORDS(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = small.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_EVERYDAY_CHILD(), &low.clone())) } } } };
    if py_truthy(&Py::Bool(py_contains(&g_CHILD_SIZE_WORDS(), &low.clone()))) {
    after = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut word2: Py = __item.clone(); if py_truthy(&mcall(&mcall(&call_builtin("str", &[word2.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[g__MARKS()])) {  __out.push(mcall(&mcall(&call_builtin("str", &[word2.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[g__MARKS()])); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CHILD_PLACE_WORDS(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_CHILD_AFTER_A_SIZE(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = small.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_EVERYDAY_CHILD(), &after.clone())) } } } } } }) {
    continue;
    }
    }
    if py_truthy(&{ let __b = child_word.clone(); if py_truthy(&__b) { __b } else { { let __b = small.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_CHILD_AFTER_A_SIZE(), &low.clone())); if !py_truthy(&__b) { __b } else { { let __b = at.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CHILD_SIZE_WORDS(), &mcall(&mcall(&call_builtin("str", &[geti(&words.clone(), &sub(&at.clone(), &Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[g__MARKS()]))) } } } } } } } }) {
    let _ = mcall(&kept.clone(), "append", &[py_str("child")]);
    skipping = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("little"), py_str("middle")]), &low.clone()))) { Py::Int(1) } else { Py::Int(0) };
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    return kept.clone();
    Py::None
}
pub fn _one_edit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut shorter: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shorter") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: shorter"))) };
    let mut longer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "longer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: longer"))) };
    let mut i: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("True when the two words are one slip apart: a letter wrong, missing, extra, or two\n    neighbours swapped.");
    if py_truthy(&Py::Bool(py_eq(&shorter.clone(), &longer.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&gt(&call_builtin("abs", &[sub(&call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[longer.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    if py_truthy(&gt(&call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[longer.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    let __unpk1 = py_unpack(&py_tuple(vec![longer.clone(), shorter.clone()]), 2, None);
    shorter = __unpk1[0].clone();
    longer = __unpk1[1].clone();
    }
    i = Py::Int(0);
    while py_truthy(&{ let __b = lt(&i.clone(), &call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&shorter.clone(), &i.clone()), &geti(&longer.clone(), &i.clone()))) } }) {
    let __aug2 = add(&i.clone(), &Py::Int(1));
    i = __aug2;
    }
    if py_truthy(&lt(&call_builtin("len", &[shorter.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[longer.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(py_eq(&slice_or_raise(&shorter.clone(), &i.clone(), &Py::None, &Py::None), &slice_or_raise(&longer.clone(), &add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None)));
    }
    let __unpk3 = py_unpack(&py_tuple(vec![slice_or_raise(&shorter.clone(), &i.clone(), &Py::None, &Py::None), slice_or_raise(&longer.clone(), &i.clone(), &Py::None, &Py::None)]), 2, None);
    rest = __unpk3[0].clone();
    tail = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None), &slice_or_raise(&tail.clone(), &Py::Int(1), &Py::None, &Py::None)))) {
    return Py::Bool(true);
    }
    return { let __b = gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&rest.clone(), &Py::Int(0)), &geti(&tail.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&rest.clone(), &Py::Int(1)), &geti(&tail.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&rest.clone(), &Py::Int(2), &Py::None, &Py::None), &slice_or_raise(&tail.clone(), &Py::Int(2), &Py::None, &Py::None))) } } } } } };
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
    let mut ending: Py = Py::None;
    let _ = py_str("The word a form belongs to, so radiator/radiators and tomato/tomatoes are one.");
    let __iter1 = py_iter_or_raise(&g_ENDINGS());
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { ge(&sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(3)) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return word.clone();
    Py::None
}
pub fn _engine_reads(args: &[Py]) -> Py {
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
    let _ = py_str("True when the understanding engine already reads this word, as English or chat-speak.\n\n    Such a word is not a typo, however close a key word looks: \"park\" is one letter from\n    \"pack\", and it stays \"park\". The normaliser's own tables count too, so a chat form\n    (\"whats\") is left to it. The word's family counts as well: \"charging\" is a form of an\n    English word the engine knows a relative of (\"charger\"), so it is a word, not a slip\n    from \"changing\", and a phone charging overnight stays out of the leaves fact.\n\n    `lexicon.every_word()` is the rest of the reading's own vocabulary - the personal names, the\n    topic and kind words, the parts of the body - gathered from those tables instead of listed\n    again. The list this replaces was rebuilt one word at a time: \"kate\" was read as \"kale\",\n    \"preston\" as \"presto\", \"unfit\" as \"unit\", \"hiya\" as \"hoya\", each found on the deck after it\n    had already answered a line. A name bent into a key word is worse than a shrug, because the\n    answer is confident and about another question: \"enid\", \"elsie\" and \"esther\" were read as\n    \"end\", \"else\" and \"either\" (\"my friend enid\" got the after-a-breakup card), and \"fella\" and\n    \"bloke\" as \"fell\" and \"broke\", so a line about who takes the credit at work was answered by\n    the falls card for an elderly parent.\n\n    A colour is a word too, and the engine's own tables do not carry them: \"white\" is one letter\n    from \"write\", \"grey\" from \"grow\", \"navy\" from \"naval\", so wherever a row stays away from\n    the nearer word the colour stood the row down - \"is white ok at a christening\" scored nothing\n    at all because the christening card's own `avoid` holds \"write\", and a colour asked about a\n    service with a clothes answer got a shrug.\n    ");
    return { let __b = Py::Bool(py_contains(&g__COMMON(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::understanding::normalize::g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::understanding::lexicon::g_CHAT(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::understanding::lexicon::g_EXPAND(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__STEMS(), &_stem(&[word.clone()]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::understanding::lexicon::every_word(&[]), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COLOUR_WORDS(), &word.clone())) } } } } } } } } } } } };
    Py::None
}
pub fn _typo_forms(args: &[Py]) -> Py {
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
    let mut vocabulary: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "vocabulary") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: vocabulary"))) };
    let mut close: Py = Py::None;
    let mut families: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut other: Py = Py::None;
    let _ = py_str("The forms of the key word a misspelt line word is one slip away from, or nothing when\n    the slip is ambiguous (\"htis\" could be two words, \"lips\" could be \"lies\" or \"lids\"), the\n    word is too short to be sure, or the engine already reads the word as it is.\n\n    Two key words of one family are one candidate, so a misspelt word still reaches an\n    entry keyed on both \"radiator\" and \"radiators\"; two different families are none, so a\n    slip that could be either of two meanings (\"lips\" for \"lies\" or \"lids\") says nothing.\n    ");
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MIN_TYPO_LENGTH()); if py_truthy(&__b) { __b } else { _engine_reads(&[word.clone()]) } }) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    close = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&vocabulary.clone()) { let mut other: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&slice_or_raise(&other.clone(), &Py::None, &Py::Int(1), &Py::None), &slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None))); if !py_truthy(&__b) { __b } else { { let __b = le(&call_builtin("abs", &[sub(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { _one_edit(&[word.clone(), other.clone()]) } } } }) {  __out.push(other.clone()); } } py_list(__out) })() };
    families = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&close.clone()) { let mut other: Py = __item.clone(); __out.push(_stem(&[other.clone()])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[families.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    forms = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&close.clone());
    for __item in __iter1 {
    other = __item.clone();
    let __aug2 = bitor(&forms.clone(), &_forms(&[other.clone()]));
    forms = __aug2;
    }
    return forms.clone();
    Py::None
}
pub fn _wanted(args: &[Py]) -> Py {
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
    let mut got: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The forms of each word of a key phrase, worked out once for every entry that says it.");
    got = mcall(&g__WANTED_CACHE(), "get", &[phrase.clone()]);
    if py_truthy(&Py::Bool(py_is(&got.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(py_contains(&g_PLURAL_ONLY_PHRASES(), &mcall(&phrase.clone(), "lower", &[])))) {
    got = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&phrase.clone(), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(call_builtin("frozenset", &[py_tuple(vec![word.clone()])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    } else {
    got = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&phrase.clone(), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&ge(&call_builtin("len", &[g__WANTED_CACHE()]).unwrap_or_else(|e| exc::raise(e)), &g__FORMS_CACHE_KEEP())) {
    let _ = mcall(&g__WANTED_CACHE(), "clear", &[]);
    }
    let _ = py_setitem(&g__WANTED_CACHE(), &phrase.clone(), got.clone());
    }
    return got.clone();
    Py::None
}
pub fn _wanted_bare(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut got: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The forms of the phrase's words with its articles left out, or None when it has none.");
    got = mcall(&g__WANTED_CACHE(), "get", &[py_tuple(vec![py_str("bare"), phrase.clone()]), py_tuple(vec![])]);
    if py_truthy(&got.clone()) {
    return geti(&got.clone(), &Py::Int(0));
    }
    words = mcall(&mcall(&phrase.clone(), "lower", &[]), "split", &[]);
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__ARTICLE_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    bare = if py_truthy(&{ let __b = ge(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &g__MIN_BARE_WORDS()); if !py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) { call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None };
    if py_truthy(&ge(&call_builtin("len", &[g__WANTED_CACHE()]).unwrap_or_else(|e| exc::raise(e)), &g__FORMS_CACHE_KEEP())) {
    let _ = mcall(&g__WANTED_CACHE(), "clear", &[]);
    }
    let _ = py_setitem(&g__WANTED_CACHE(), &py_tuple(vec![py_str("bare"), phrase.clone()]), py_tuple(vec![bare.clone()]));
    return bare.clone();
    Py::None
}
pub fn _phrase_span_bare(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut here: Py = Py::None;
    let mut j: Py = Py::None;
    let mut last: Py = Py::None;
    let mut n: Py = Py::None;
    let mut span: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The span an article-free phrase covers on the line, where the line may keep its articles.\n\n    The words of the phrase have to sit in the line in order, as they do for the phrase itself,\n    and an article of the line may sit between any two of them: \"how do i look after battery on my\n    phone\" is the same question as \"how do i look after the battery on my phone\". Nothing else is\n    skipped, so the phrase still says what it said.\n    ");
    n = call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e));
    last = sub(&call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    start = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&geti(&forms.clone(), &start.clone()), &geti(&wanted.clone(), &Py::Int(0)))))) {
    continue;
    }
    span = py_list(vec![start.clone()]);
    here = add(&start.clone(), &Py::Int(1));
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), n.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let mut __else3 = false;
    for __item in __iter2 {
    j = __item.clone();
    while py_truthy(&{ let __b = le(&here.clone(), &last.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&geti(&forms.clone(), &here.clone()), &geti(&wanted.clone(), &j.clone())))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&geti(&forms.clone(), &here.clone()), &g__ARTICLE_FORMS())))) {
    break;
    }
    let __aug4 = add(&here.clone(), &Py::Int(1));
    here = __aug4;
    }
    if py_truthy(&{ let __b = gt(&here.clone(), &last.clone()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&geti(&forms.clone(), &here.clone()), &geti(&wanted.clone(), &j.clone())))) } }) {
    break;
    }
    let _ = mcall(&span.clone(), "append", &[here.clone()]);
    let __aug5 = add(&here.clone(), &Py::Int(1));
    here = __aug5;
    }
    return call_builtin("tuple", &[span.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _shape(args: &[Py]) -> Py {
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
    let mut got: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("(the words of a key phrase, its weight), worked out once for every entry that says it.\n\n    _part_matches reads the words and the weight of every alternative of every key part it is\n    asked about, and a line asks thousands of times: a phrase is a module-level constant, so\n    its shape is worked out once instead of being lower-cased and split each time.\n    ");
    got = mcall(&g__SHAPE_CACHE(), "get", &[phrase.clone()]);
    if py_truthy(&Py::Bool(py_is(&got.clone(), &Py::None))) {
    words = call_builtin("tuple", &[mcall(&mcall(&phrase.clone(), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    got = py_tuple(vec![words.clone(), if py_truthy(&gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { g_PHRASE_WEIGHT() } else { g_WORD_WEIGHT() }]);
    if py_truthy(&ge(&call_builtin("len", &[g__SHAPE_CACHE()]).unwrap_or_else(|e| exc::raise(e)), &g__FORMS_CACHE_KEEP())) {
    let _ = mcall(&g__SHAPE_CACHE(), "clear", &[]);
    }
    let _ = py_setitem(&g__SHAPE_CACHE(), &phrase.clone(), got.clone());
    }
    return got.clone();
    Py::None
}
pub fn _line_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut every: Py = Py::None;
    let mut got: Py = Py::None;
    let mut word_forms: Py = Py::None;
    let _ = py_str("Every form any word of the line has, worked out once for the line.\n\n    A key phrase whose words are not all here cannot be in the line, so the answer is no before\n    any of the positional work: the test is one set intersection per word, in C, and most of the\n    8,900 key parts of the knowledge base are not about the line in front of them.\n    ");
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[g__LINE_FORMS()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &forms.clone())) } }) {
    return geti(&got.clone(), &Py::Int(1));
    }
    }
    every = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&forms.clone());
    for __item in __iter1 {
    word_forms = __item.clone();
    let __aug2 = bitor(&every.clone(), &word_forms.clone());
    every = __aug2;
    }
    every = call_builtin("frozenset", &[every.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &g__LINE_FORMS(), py_tuple(vec![forms.clone(), every.clone()]));
    }
    return every.clone();
    Py::None
}
pub fn _where(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut at: Py = Py::None;
    let mut firsts: Py = Py::None;
    let mut form: Py = Py::None;
    let mut got: Py = Py::None;
    let mut index: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut word_forms: Py = Py::None;
    let _ = py_str("{form: the positions in the line that have it}, worked out once for the line.\n\n    Every entry the line reaches asks _phrase_span about its own key words, and each of those\n    answers is a question about the same line, so the line is read into this map once instead of\n    being walked again for every word of every key. `seen` is the line's memo (see _part_matches).\n    The second map is where each form first appears: a single key word only needs the earliest\n    position, and that is the question a line asks thousands of times.\n    ");
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[g__WHERE()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &forms.clone())) } }) {
    return py_tuple(vec![geti(&got.clone(), &Py::Int(1)), geti(&got.clone(), &Py::Int(2))]);
    }
    }
    where_ = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word_forms = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&word_forms.clone());
    for __item in __iter3 {
    form = __item.clone();
    at = mcall(&where_.clone(), "get", &[form.clone()]);
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    let _ = py_setitem(&where_.clone(), &form.clone(), py_list(vec![index.clone()]));
    } else {
    let _ = mcall(&at.clone(), "append", &[index.clone()]);
    }
    }
    }
    firsts = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&where_.clone(), "items", &[])) { let __unpk7702 = py_unpack(&__item.clone(), 2, None); let mut form: Py = __unpk7702[0].clone(); let mut at: Py = __unpk7702[1].clone(); __out.push((form.clone(), geti(&at.clone(), &Py::Int(0)))); } py_dict(__out) })() };
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &g__WHERE(), py_tuple(vec![forms.clone(), where_.clone(), firsts.clone()]));
    }
    return py_tuple(vec![where_.clone(), firsts.clone()]);
    Py::None
}
pub fn _phrase_span(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut end: Py = Py::None;
    let mut got: Py = Py::None;
    let mut spans: Py = Py::None;
    let _ = py_str("The positions in the line this phrase covers, or () when it is not there.\n\n    One position for a single word, several for a phrase, so score() can tell a line\n    that mentions two different key words from one that repeats the same word twice.\n\n    The answer is kept on the line's memo by the phrase itself. The same words are key phrases\n    of many entries - \"how long\", \"how do i\", \"without\" - and every entry the line reaches asks\n    about its own copy, so the answer was worked out thousands of times a turn.\n    ");
    spans = Py::None;
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[g__PHRASE_SPANS()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &forms.clone())) } }) {
    spans = geti(&got.clone(), &Py::Int(1));
    } else {
    spans = py_dict(vec![]);
    let _ = py_setitem(&seen.clone(), &g__PHRASE_SPANS(), py_tuple(vec![forms.clone(), spans.clone()]));
    }
    if py_truthy(&Py::Bool(py_contains(&spans.clone(), &phrase.clone()))) {
    return geti(&spans.clone(), &phrase.clone());
    }
    end = _phrase_span_work(&[forms.clone(), phrase.clone(), seen.clone()]);
    let _ = py_setitem(&spans.clone(), &phrase.clone(), end.clone());
    return end.clone();
    }
    return _phrase_span_work(&[forms.clone(), phrase.clone(), Py::None]);
    Py::None
}
pub fn _ask_frame_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = Py::None;
    let _ = py_str("The forms of ASK_FRAME, worked out once.");
    if py_truthy(&Py::Bool(py_is(&g__ASK_FRAME_FORMS(), &Py::None))) {
    s__ASK_FRAME_FORMS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASK_FRAME()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)), "union", args))(&__args) });
    }
    return g__ASK_FRAME_FORMS();
    Py::None
}
pub fn _phrase_span_between(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut answer: Py = Py::None;
    let mut got: Py = Py::None;
    let mut key: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut start: Py = Py::None;
    let mut walk: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The span of an avoid phrase where a breather may sit between its words, or ().\n\n    Two words at most may sit between any two words of the phrase, and each of them has to be a\n    breather: \"my phone has just died\" is the same line as \"my phone died\", and \"the phone rang\n    and the cat died\" is not.\n\n    A phrase that opens on an asking word keeps its words together. The phrase is a way of asking\n    there - \"how do i\", \"how to\", \"what is\", \"is it\" - and the asking words of two different\n    questions sit next to each other across a line that asks one of them for a price: \"how much\n    to hire a breaker\" has \"how\" and \"to\" with \"much\" between them, and every price row that\n    stands down on \"how to\" (the whole cost table does) then refused the line, so the commonest\n    way of asking what a hire costs had no answer at all. \"my phone has just died\" is what the\n    breathers are for, and no asking phrase needs one.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[phrase.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![]);
    }
    wanted = _wanted(&[phrase.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return py_tuple(vec![]);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return py_tuple(vec![]);
    }
    if py_truthy(&bitand(&geti(&wanted.clone(), &Py::Int(0)), &_ask_frame_forms(&[]))) {
    return py_tuple(vec![]);
    }
    key = py_tuple(vec![py_str("between"), phrase.clone()]);
    spans = Py::None;
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[g__PHRASE_SPANS()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &forms.clone())) } }) {
    spans = geti(&got.clone(), &Py::Int(1));
    } else {
    spans = py_dict(vec![]);
    let _ = py_setitem(&seen.clone(), &g__PHRASE_SPANS(), py_tuple(vec![forms.clone(), spans.clone()]));
    }
    if py_truthy(&Py::Bool(py_contains(&spans.clone(), &key.clone()))) {
    return geti(&spans.clone(), &key.clone());
    }
    }
pub fn __nested_walk_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut forms: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut got: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut key: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut phrase: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut seen: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut spans: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut start: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut walk: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut wanted: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut at: Py = __pos.get(10).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "at").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut next_word: Py = __pos.get(11).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "next_word").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut span: Py = __pos.get(12).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "span").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut gaps: Py = Py::None;
    if py_truthy(&Py::Bool(py_eq(&next_word.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("tuple", &[span.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    gaps = Py::Int(0);
    while py_truthy(&{ let __b = lt(&at.clone(), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { le(&gaps.clone(), &g__A_BREATH()) } }) {
    if py_truthy(&bitand(&geti(&forms.clone(), &at.clone()), &geti(&wanted.clone(), &next_word.clone()))) {
    got = py_call(&walk, &[add(&at.clone(), &Py::Int(1)), add(&next_word.clone(), &Py::Int(1)), add(&span.clone(), &py_list(vec![at.clone()]))]);
    if py_truthy(&got.clone()) {
    return got.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&geti(&forms.clone(), &at.clone()), &g__SAID_BETWEEN())))) {
    return py_tuple(vec![]);
    }
    let __aug1 = add(&gaps.clone(), &Py::Int(1));
    gaps = __aug1;
    let __aug2 = add(&at.clone(), &Py::Int(1));
    at = __aug2;
    }
    return py_tuple(vec![]);
    Py::None
}
    walk = py_func("walk", move |args: &[Py]| { let mut full = vec![answer.clone(), forms.clone(), got.clone(), key.clone(), phrase.clone(), seen.clone(), spans.clone(), start.clone(), walk.clone(), wanted.clone()]; full.extend_from_slice(args); __nested_walk_0(&full) });
    answer = py_tuple(vec![]);
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    if py_truthy(&bitand(&geti(&forms.clone(), &start.clone()), &geti(&wanted.clone(), &Py::Int(0)))) {
    answer = py_call(&walk, &[add(&start.clone(), &Py::Int(1)), Py::Int(1), py_list(vec![start.clone()])]);
    if py_truthy(&answer.clone()) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(!py_is(&spans.clone(), &Py::None))) {
    let _ = py_setitem(&spans.clone(), &key.clone(), answer.clone());
    }
    return answer.clone();
    Py::None
}
pub fn _the_same_words(args: &[Py]) -> Py {
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
    let mut got: Py = Py::None;
    let _ = py_str("The same phrase in the other spelling, or nothing when it has no contraction in it.");
    got = mcall(&g__PHRASE_OTHER_SPELLINGS(), "get", &[phrase.clone()]);
    if py_truthy(&Py::Bool(py_is(&got.clone(), &Py::None))) {
    got = _work_out_the_same_words(&[phrase.clone()]);
    if py_truthy(&ge(&call_builtin("len", &[g__PHRASE_OTHER_SPELLINGS()]).unwrap_or_else(|e| exc::raise(e)), &g__OTHER_SPELLINGS_KEEP())) {
    let _ = mcall(&g__PHRASE_OTHER_SPELLINGS(), "clear", &[]);
    }
    let _ = py_setitem(&g__PHRASE_OTHER_SPELLINGS(), &phrase.clone(), got.clone());
    }
    return got.clone();
    Py::None
}
pub fn _work_out_the_same_words(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut other: Py = Py::None;
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    words = mcall(&mcall(&phrase.clone(), "lower", &[]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_tuple(vec![]);
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&mcall(&g__ONE_SPELLING_EITHER_WAY(), "get", &[word.clone(), py_tuple(vec![])]));
    for __item in __iter3 {
    other = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&other.clone(), &py_str(" ")))) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(" "), "join", &[add(&add(&slice_or_raise(&words.clone(), &Py::None, &i.clone(), &Py::None), &mcall(&other.clone(), "split", &[])), &slice_or_raise(&words.clone(), &add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None))])]);
    } else {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(" "), "join", &[add(&add(&slice_or_raise(&words.clone(), &Py::None, &i.clone(), &Py::None), &py_list(vec![other.clone()])), &slice_or_raise(&words.clone(), &add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None))])]);
    }
    }
    }
    if py_truthy(&gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    i = __item.clone();
    joined = add(&add(&geti(&words.clone(), &i.clone()), &py_str(" ")), &geti(&words.clone(), &add(&i.clone(), &Py::Int(1))));
    let __iter5 = py_iter_or_raise(&mcall(&g__ONE_SPELLING_EITHER_WAY(), "get", &[joined.clone(), py_tuple(vec![])]));
    for __item in __iter5 {
    other = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&other.clone(), &py_str(" ")))) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(" "), "join", &[add(&add(&slice_or_raise(&words.clone(), &Py::None, &i.clone(), &Py::None), &py_list(vec![other.clone()])), &slice_or_raise(&words.clone(), &add(&i.clone(), &Py::Int(2)), &Py::None, &Py::None))])]);
    }
    }
    }
    }
    return call_builtin("tuple", &[out.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _phrase_span_work(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut end: Py = Py::None;
    let mut other: Py = Py::None;
    let _ = py_str("The phrase's span on this line, with no phrase memo in front of it.");
    end = _phrase_span_spelling(&[forms.clone(), phrase.clone(), seen.clone()]);
    if py_truthy(&end.clone()) {
    return end.clone();
    }
    let __iter1 = py_iter_or_raise(&_the_same_words(&[phrase.clone()]));
    for __item in __iter1 {
    other = __item.clone();
    end = _phrase_span_spelling(&[forms.clone(), other.clone(), seen.clone()]);
    if py_truthy(&end.clone()) {
    return end.clone();
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _phrase_span_spelling(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut firsts: Py = Py::None;
    let mut form: Py = Py::None;
    let mut found: Py = Py::None;
    let mut hits: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut line: Py = Py::None;
    let mut n: Py = Py::None;
    let mut start: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut word_forms: Py = Py::None;
    let _ = py_str("The span of this exact spelling of the phrase.");
    wanted = _wanted(&[phrase.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return py_tuple(vec![]);
    }
    n = call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    line = _line_forms(&[forms.clone(), seen.clone()]);
    let __iter1 = py_iter_or_raise(&wanted.clone());
    for __item in __iter1 {
    word_forms = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&word_forms.clone(), &line.clone())))) {
    return py_tuple(vec![]);
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&seen.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(py_eq(&n.clone(), &Py::Int(1)))) {
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    i = __unpk3[0].clone();
    word_forms = __unpk3[1].clone();
    if py_truthy(&bitand(&word_forms.clone(), &geti(&wanted.clone(), &Py::Int(0)))) {
    return py_tuple(vec![i.clone()]);
    }
    }
    return py_tuple(vec![]);
    }
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[add(&sub(&call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)), &n.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    i = __item.clone();
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[n.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut j: Py = __item.clone(); __out.push(bitand(&geti(&forms.clone(), &add(&i.clone(), &j.clone())), &geti(&wanted.clone(), &j.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return call_builtin("tuple", &[call_builtin("range", &[i.clone(), add(&i.clone(), &n.clone())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return py_tuple(vec![]);
    }
    let __unpk5 = py_unpack(&_where(&[forms.clone(), seen.clone()]), 2, None);
    where_ = __unpk5[0].clone();
    firsts = __unpk5[1].clone();
    if py_truthy(&Py::Bool(py_eq(&n.clone(), &Py::Int(1)))) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&geti(&wanted.clone(), &Py::Int(0)), "isdisjoint", &[firsts.clone()])))) {
    return py_tuple(vec![call_builtin("min", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&wanted.clone(), &Py::Int(0))) { let mut form: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&firsts.clone(), &form.clone()))) {  __out.push(geti(&firsts.clone(), &form.clone())); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return py_tuple(vec![]);
    }
    hits = py_list(vec![]);
    let __iter6 = py_iter_or_raise(&wanted.clone());
    for __item in __iter6 {
    word_forms = __item.clone();
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter7 = py_iter_or_raise(&word_forms.clone());
    for __item in __iter7 {
    form = __item.clone();
    let _ = mcall(&found.clone(), "update", &[mcall(&where_.clone(), "get", &[form.clone(), py_tuple(vec![])])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![]);
    }
    let _ = mcall(&hits.clone(), "append", &[found.clone()]);
    }
    let __iter8 = py_iter_or_raise(&call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&hits.clone(), &Py::Int(0))) { let mut i: Py = __item.clone(); if py_truthy(&le(&add(&i.clone(), &n.clone()), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(i.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter8 {
    start = __item.clone();
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(1), n.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut j: Py = __item.clone(); __out.push(Py::Bool(py_contains(&geti(&hits.clone(), &j.clone()), &add(&start.clone(), &j.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return call_builtin("tuple", &[call_builtin("range", &[start.clone(), add(&start.clone(), &n.clone())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _phrase_found(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    return call_builtin("bool", &[_phrase_span(&[forms.clone(), phrase.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _part_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut alternative: Py = Py::None;
    let mut alternatives: Py = Py::None;
    let mut every: Py = Py::None;
    let mut got: Py = Py::None;
    let mut word_forms: Py = Py::None;
    let _ = py_str("Every form of every word any alternative of this part says, worked out once per part.");
    got = mcall(&g__PART_FORMS(), "get", &[call_builtin("id", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &part.clone())) } }) {
    return geti(&got.clone(), &Py::Int(1));
    }
    alternatives = if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) };
    every = Py::None;
    let __iter1 = py_iter_or_raise(&alternatives.clone());
    for __item in __iter1 {
    alternative = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&alternative.clone(), "strip", &[]))) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&_wanted(&[alternative.clone()]));
    for __item in __iter2 {
    word_forms = __item.clone();
    if py_truthy(&Py::Bool(py_is(&every.clone(), &Py::None))) {
    every = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    let __aug3 = bitor(&every.clone(), &word_forms.clone());
    every = __aug3;
    }
    }
    if py_truthy(&Py::Bool(py_is(&every.clone(), &Py::None))) {
    return if py_truthy(&Py::Bool(!py_truthy(&alternatives.clone()))) { Py::None } else { call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)) };
    }
    every = call_builtin("frozenset", &[every.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&ge(&call_builtin("len", &[g__PART_FORMS()]).unwrap_or_else(|e| exc::raise(e)), &g__PART_FORMS_KEEP())) {
    let _ = mcall(&g__PART_FORMS(), "clear", &[]);
    }
    let _ = py_setitem(&g__PART_FORMS(), &call_builtin("id", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![part.clone(), every.clone()]));
    return every.clone();
    Py::None
}
pub fn _part_matches(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut alternative: Py = Py::None;
    let mut alternatives: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut bare_hit: Py = Py::None;
    let mut every: Py = Py::None;
    let mut found: Py = Py::None;
    let mut got: Py = Py::None;
    let mut span: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every matching alternative of one key part: [(weight, its words, its span), ...].\n\n    `seen` is this line's memo, a dict from the part's id to the answer. score() asks a part\n    about the line and _rank() asks the same part again, and the answer does not depend on\n    anything else, so the second question is not asked. The part belongs to a module-level\n    entry, so it outlives the call and its id cannot be reused inside it.\n    ");
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[call_builtin("id", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_is(&got.clone(), &Py::None))) {
    return got.clone();
    }
    }
    every = _part_forms(&[part.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&every.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&every.clone(), &_line_forms(&[forms.clone(), seen.clone()])))) } }) {
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &call_builtin("id", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), py_list(vec![]));
    }
    return py_list(vec![]);
    }
    alternatives = if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) };
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&alternatives.clone());
    for __item in __iter1 {
    alternative = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&alternative.clone(), "strip", &[]))) } }) {
    continue;
    }
    span = _phrase_span(&[forms.clone(), alternative.clone(), seen.clone()]);
    bare_hit = Py::Bool(false);
    if py_truthy(&Py::Bool(!py_truthy(&span.clone()))) {
    bare = _wanted_bare(&[alternative.clone()]);
    if py_truthy(&Py::Bool(!py_is(&bare.clone(), &Py::None))) {
    span = _phrase_span_bare(&[forms.clone(), bare.clone(), seen.clone()]);
    bare_hit = call_builtin("bool", &[span.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    if py_truthy(&span.clone()) {
    let __unpk2 = py_unpack(&_shape(&[alternative.clone()]), 2, None);
    words = __unpk2[0].clone();
    weight = __unpk2[1].clone();
    if py_truthy(&{ let __b = bare_hit.clone(); if !py_truthy(&__b) { __b } else { gt(&weight.clone(), &g_BARE_WEIGHT()) } }) {
    weight = g_BARE_WEIGHT();
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![weight.clone(), words.clone(), span.clone()])]);
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let alternative = alternative.clone(); let alternatives = alternatives.clone(); let bare = bare.clone(); let bare_hit = bare_hit.clone(); let every = every.clone(); let forms = forms.clone(); let found = found.clone(); let got = got.clone(); let part = part.clone(); let seen = seen.clone(); let span = span.clone(); let weight = weight.clone(); let words = words.clone(); let hit: Py = __kwp(args, 0, "hit").unwrap_or(Py::None); py_tuple(vec![geti(&hit.clone(), &Py::Int(0)), Py::Bool(!py_truthy(&_only_the_frame(&[geti(&hit.clone(), &Py::Int(1))])))]) })), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "sort", args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &call_builtin("id", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), found.clone());
    }
    return found.clone();
    Py::None
}
pub fn _part_match(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut covered: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "covered") { v.clone() }
    else { py_tuple(vec![]) };
    let mut seen: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut span: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The best matching alternative of one key part: (weight, its words, its span).\n\n    An alternative that says something new about the line beats one that reuses a word\n    already counted by another part, so (\"rain\", (\"raining\", \"why\")) scores on \"why does\n    it rain\" from the \"why\", not from a second reading of \"rain\".\n    ");
    found = _part_matches(&[part.clone(), forms.clone(), seen.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![Py::Int(0), py_tuple(vec![]), py_tuple(vec![])]);
    }
    let __iter1 = py_iter_or_raise(&found.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    weight = __unpk2[0].clone();
    words = __unpk2[1].clone();
    span = __unpk2[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&covered.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&le(&call_builtin("set", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[covered.clone()]).unwrap_or_else(|e| exc::raise(e))))) } }) {
    return py_tuple(vec![weight.clone(), words.clone(), span.clone()]);
    }
    }
    return geti(&found.clone(), &Py::Int(0));
    Py::None
}
pub fn _part_score(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    return geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(forms.clone()); __kws.extend(vec![("seen".to_string(), seen.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _part_match(args))(&__args) }, &Py::Int(0));
    Py::None
}
pub fn _own_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut key: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The single words this entry's own key already names, from its phrases as well.\n\n    \"self employed\" is one key phrase, and \"employed\" inside it is a word the entry has\n    already claimed: a row that keys it cannot then stay away from \"employee\" because the\n    line says \"self employed\". `_misspelt_avoid` uses this to tell a slip from a subject.\n    ");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = gattr(&item.clone(), "key"); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    part = py_tuple(vec![part.clone()]);
    }
    let __iter2 = py_iter_or_raise(&part.clone());
    for __item in __iter2 {
    key = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[key.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&mcall(&mcall(&mcall(&key.clone(), "strip", &[]), "lower", &[]), "split", &[]));
    for __item in __iter3 {
    word = __item.clone();
    if py_truthy(&ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MIN_TYPO_LENGTH())) {
    let _ = mcall(&words.clone(), "add", &[word.clone()]);
    }
    }
    }
    }
    return words.clone();
    Py::None
}
pub fn _misspelt_avoid(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut typed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut own: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own") { v.clone() }
    else { Py::None };
    let mut key: Py = Py::None;
    let mut typed_word: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a line word is one slip from a word this entry stays away from.\n\n    The forms of a repaired word already reach the avoid list, but a slip that two key\n    words could explain (\"brest\" is \"breast\" or \"best\") is never used to read a line, and\n    must still not be answered as the entry that disowns that word (\"how long do i cook a\n    chiken brest\" is not a whole-bird question).\n\n    A word the entry's own key names is not a slip, whatever the avoid list says: \"self\n    employed\" is in start-a-business's key, \"employee\" is in its avoid, and \"employed\" was\n    read as a slip of \"employee\", so \"im thinking of going self employed\" and \"should i go\n    self employed\" were both refused by the one row that answers them.\n    ");
    if py_truthy(&Py::Bool(py_is(&own.clone(), &Py::None))) {
    own = _own_words(&[item.clone()]);
    }
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "avoid"));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    key = mcall(&mcall(&word.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MIN_TYPO_LENGTH()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&key.clone(), &py_str(" "))) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&{ let __b = typed.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    typed_word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&own.clone(), &typed_word.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&own.clone(), &typed_word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[typed_word.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MIN_TYPO_LENGTH()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_engine_reads(&[typed_word.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&slice_or_raise(&typed_word.clone(), &Py::None, &Py::Int(1), &Py::None), &slice_or_raise(&key.clone(), &Py::None, &Py::Int(1), &Py::None))); if !py_truthy(&__b) { __b } else { _one_edit(&[typed_word.clone(), key.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _about_someone_else(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut person: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The ask phrase said about someone other than the speaker, or () when it cannot be.");
    words = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("me"), py_str("my")]), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![]);
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g_ASK_PERSON_PRONOUNS());
    for __item in __iter1 {
    person = __item.clone();
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("i"), py_str("me")]), &mcall(&word.clone(), "lower", &[])))) { person.clone() } else { word.clone() }); } py_list(__out) })() }])]);
    }
    let __iter2 = py_iter_or_raise(&g_ASK_PERSON_POSSESSIVES());
    for __item in __iter2 {
    person = __item.clone();
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&mcall(&word.clone(), "lower", &[]), &py_str("my")))) { person.clone() } else { word.clone() }); } py_list(__out) })() }])]);
    }
    return call_builtin("tuple", &[out.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_about_someone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut got: Py = Py::None;
    let mut line: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut variant: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when one of the entry's asks is said about a person other than the speaker.");
    line = _line_forms(&[forms.clone(), seen.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASK_PEOPLE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = ask.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[py_tuple(vec![g__ASK_ABOUT_SOMEONE(), call_builtin("id", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))])]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &forms.clone())) } }) {
    if py_truthy(&geti(&got.clone(), &Py::Int(1))) {
    return Py::Bool(true);
    }
    continue;
    }
    }
    said = Py::Bool(false);
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { py_tuple(vec![part.clone()]) } else { part.clone() });
    for __item in __iter2 {
    phrase = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[phrase.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&_about_someone_else(&[phrase.clone()]));
    for __item in __iter3 {
    variant = __item.clone();
    if py_truthy(&_phrase_span(&[forms.clone(), variant.clone(), seen.clone()])) {
    said = Py::Bool(true);
    break;
    }
    }
    if py_truthy(&said.clone()) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &py_tuple(vec![g__ASK_ABOUT_SOMEONE(), call_builtin("id", &[part.clone()]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![forms.clone(), said.clone()]));
    }
    if py_truthy(&said.clone()) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _ask_about_someone_only(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut part: Py = Py::None;
    let _ = py_str("True when the entry's ask is met only by its somebody-else reading.\n\n    The line asked the row's question, and it asked it about a person rather than about the\n    speaker: no phrase of the ask is said as written.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&ask.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask.clone()) { let mut part: Py = __item.clone(); __out.push(_part_score(&[part.clone(), forms.clone(), seen.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return _said_about_someone(&[ask.clone(), forms.clone(), seen.clone()]);
    Py::None
}
pub fn _the_subject_it_avoids(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut carried: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carried") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: carried"))) };
    let mut seen: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut memo: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the row asks about someone else on the very subject its own avoid list rules out.\n\n    \"my mums got a chest infection\" then \"should she see a doctor\": the ask is the see-somebody\n    row's, said about her instead of about the speaker, and that row's avoid list names a chest,\n    a cough, the flu and a mum - all words of the line this one leans on, so the row stands down\n    when the speaker asks it about themselves. Reached through the somebody-else reading the test\n    was never made, and two to six weeks was said of a chest infection.\n\n    The earlier line is read for the avoid list only when the ask is met through that reading and\n    not through a phrase of the entry's own, so a line the entry reaches as typed never turns on\n    this.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&carried.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_ask_about_someone_only(&[gattr(&item.clone(), "ask"), forms.clone(), seen.clone()])))) {
    return Py::Bool(false);
    }
    memo = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "avoid"));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { _part_score(&[word.clone(), carried.clone(), memo.clone()]) } } } }) {
    let mut os: Py = stdlib_module("os");
    let mut sys: Py = stdlib_module("sys");
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_subject_the_line_leans_on(args: &[Py]) -> Py {
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
    let mut leans: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The forms of the words the line leans on, or [] when it leans on nothing.\n\n    The person the earlier line is about is left out. \"husband\", \"wife\" and \"mum\" are in the\n    avoid list of nearly every card about a part of somebody else, because the speaker's own\n    part is what those cards hold, and asking about that somebody is the reading this test is\n    for - so a card would be ruled out for saying who the person was.\n    ");
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    words = if py_truthy(&call_builtin("isinstance", &[mcall(&leans.clone(), "get", &[py_str("words")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&leans.clone(), "get", &[py_str("words")]) } else { py_list(vec![]) };
    return slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ASK_PEOPLE(), &mcall(&mcall(&word.clone(), "strip", &[]), "lower", &[]))) } }) {  __out.push(_forms(&[word.clone()])); } } py_list(__out) })() }, &Py::None, &g_CONTEXT_WORDS(), &Py::None);
    Py::None
}
pub fn _the_ask_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut part: Py = Py::None;
    let _ = py_str("True when the line asks what the entry's `ask` says, about the speaker or someone else.");
    if py_truthy(&Py::Bool(!py_truthy(&ask.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask.clone()) { let mut part: Py = __item.clone(); __out.push(_part_score(&[part.clone(), forms.clone(), seen.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return _said_about_someone(&[ask.clone(), forms.clone(), seen.clone()]);
    Py::None
}
pub fn _wanting(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut answer: Py = Py::None;
    let mut got: Py = Py::None;
    let _ = py_str("True when the line asks for the thing as a want rather than as a question.\n\n    Read from the line, so the line's memo keeps it: this is asked once for every entry whose\n    ask the line did not say, and the answer does not depend on the entry.\n    ");
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[g__WANTING()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &forms.clone())) } }) {
    return geti(&got.clone(), &Py::Int(1));
    }
    }
    if py_truthy(&gt(&_part_score(&[g_WANT_KNOWING(), forms.clone(), seen.clone()]), &Py::Int(0))) {
    answer = Py::Bool(false);
    } else {
    answer = gt(&_part_score(&[g_WANTING(), forms.clone(), seen.clone()]), &Py::Int(0));
    }
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &g__WANTING(), py_tuple(vec![forms.clone(), answer.clone()]));
    }
    return answer.clone();
    Py::None
}
pub fn _only_the_want(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let _ = py_str("True when the want rule, and not the entry's own ask, is why this entry reached the line.\n\n    The want rule is a reading of an entry the line names but does not ask about: \"i want to make\n    a cake\" names the cake and the making, so the recipe's ask (\"how long\", \"how do i\") does not\n    have to be said. It is not a licence for an entry whose own ask is about something the line\n    never mentions: \"i want to lose weight\" reached the cycling entry this way, because its key\n    holds \"weight\" and \"lose\" and only its ask (\"bike\", \"cycle\", \"pedals\") says it is about riding\n    a bike, so the line got the calories-in-an-hour answer instead of the losing-weight one. An\n    entry whose ask the line does say is a better fit than one let in by the want alone, so the\n    two are ranked apart.\n    ");
    return if py_truthy(&gattr(&item.clone(), "ask")) { Py::Bool(!py_truthy(&_the_ask_said(&[gattr(&item.clone(), "ask"), forms.clone(), seen.clone()]))) } else { Py::Bool(false) };
    Py::None
}
pub fn _key_on_the_want(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut alternative: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("True when a key word this line matched is one of the want phrase's filler words.");
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = bitand(&call_builtin("set", &[mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_WANT_FILLER()); if !py_truthy(&__b) { __b } else { _part_score(&[alternative.clone(), forms.clone(), seen.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _denier_inside(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut covered: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "covered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: covered"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut index: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut at: Py = Py::None;
    let mut back: Py = Py::None;
    let _ = py_str("True when a denier between the subject and this word is inside the phrase that matched.\n\n    A key phrase can carry its own \"not\": \"not getting any interviews\" is what the entry asks\n    for, and \"im not getting any interviews\" is the same line with a subject in front of it.\n    Reading the key's own not as the speaker ruling the state out answered both of those with a\n    shrug, because the subject \"im\" made \"getting\" the state after it.\n    ");
    back = Py::Int(2);
    while py_truthy(&{ let __b = le(&back.clone(), &Py::Int(4)); if !py_truthy(&__b) { __b } else { ge(&sub(&index.clone(), &back.clone()), &Py::Int(0)) } }) {
    at = sub(&index.clone(), &back.clone());
    if py_truthy(&Py::Bool(!py_contains(&covered.clone(), &at.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&geti(&forms.clone(), &at.clone()), &g_DENIERS())) {
    return Py::Bool(true);
    }
    let __aug1 = add(&back.clone(), &Py::Int(1));
    back = __aug1;
    }
    return Py::Bool(false);
    Py::None
}
pub fn _denied(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut covered: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "covered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: covered"))) };
    let mut back: Py = Py::None;
    let mut before: Py = Py::None;
    let mut between: Py = Py::None;
    let mut index: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the speaker rules out the state a key word names.");
    let __iter1 = py_iter_or_raise(&covered.clone());
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&covered.clone(), &sub(&index.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { bitand(&geti(&forms.clone(), &sub(&index.clone(), &Py::Int(1))), &g_DENIERS()) } }) {
    continue;
    }
    if py_truthy(&_denier_inside(&[covered.clone(), forms.clone(), index.clone()])) {
    continue;
    }
    back = Py::Int(1);
    while py_truthy(&{ let __b = le(&back.clone(), &Py::Int(4)); if !py_truthy(&__b) { __b } else { ge(&sub(&index.clone(), &back.clone()), &Py::Int(0)) } }) {
    before = geti(&forms.clone(), &sub(&index.clone(), &back.clone()));
    if py_truthy(&bitand(&before.clone(), &g_DENIERS())) {
    subject = sub(&sub(&index.clone(), &back.clone()), &Py::Int(1));
    if py_truthy(&{ let __b = lt(&subject.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&geti(&forms.clone(), &subject.clone()), &g_DENY_SAID_OF()))) } }) {
    break;
    }
    between = slice_or_raise(&forms.clone(), &add(&sub(&index.clone(), &back.clone()), &Py::Int(1)), &index.clone(), &Py::None);
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(bitand(&word.clone(), &g_STATES_AFTER())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(bitand(&word.clone(), &bitor(&g_STATES_AFTER(), &g_DENY_BETWEEN()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&before.clone(), &g_DENY_BETWEEN()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&before.clone(), &g_STATES_AFTER()))) } }) {
    break;
    }
    let __aug2 = add(&back.clone(), &Py::Int(1));
    back = __aug2;
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _only_the_frame(args: &[Py]) -> Py {
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
    let _ = py_str("True when every word of a matched key part is one the line is wrapped in, not about.");
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ASK_FRAME(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _says_removal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut index: Py = Py::None;
    let mut of: Py = Py::None;
    let mut off: Py = Py::None;
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says something is to be taken off something else.");
    if py_truthy(&Py::Bool(py_is(&g__TAKE_OFF(), &Py::None))) {
    s__TAKE_OFF(py_tuple(vec![_forms(&[py_str("off")]), _forms(&[py_str("out")]), _forms(&[py_str("of")])]));
    }
    let __unpk1 = py_unpack(&g__TAKE_OFF(), 3, None);
    off = __unpk1[0].clone();
    out = __unpk1[1].clone();
    of = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    word = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    continue;
    }
    if py_truthy(&bitand(&word.clone(), &off.clone())) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { { let __b = of.clone(); if !py_truthy(&__b) { __b } else { { let __b = lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = bitand(&word.clone(), &out.clone()); if !py_truthy(&__b) { __b } else { bitand(&geti(&forms.clone(), &add(&index.clone(), &Py::Int(1))), &of.clone()) } } } } } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _frame_pronoun_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut got: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The forms of the pronouns above, worked out once.");
    if py_truthy(&Py::Bool(py_is(&g__FRAME_PRONOUN_FORMS(), &Py::None))) {
    got = call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_FRAME_PRONOUNS());
    for __item in __iter1 {
    word = __item.clone();
    let __aug2 = bitor(&got.clone(), &_forms(&[word.clone()]));
    got = __aug2;
    }
    s__FRAME_PRONOUN_FORMS(got.clone());
    }
    return g__FRAME_PRONOUN_FORMS();
    Py::None
}
pub fn _the_subjects_own_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut here: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "here") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: here"))) };
    let mut form: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the words at `here` say what the thing in hand does, not what is left out.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&here.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&here.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    mine = _frame_pronoun_forms(&[]);
    if py_truthy(&bitand(&geti(&here.clone(), &Py::Int(0)), &mine.clone())) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&here.clone(), &Py::Int(0))) { let mut form: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), "endswith", &[py_str("ing")])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&here.clone(), &Py::Int(1), &Py::Int(3), &Py::None)) { let mut word: Py = __item.clone(); __out.push({ let __b = word.clone(); if !py_truthy(&__b) { __b } else { bitand(&word.clone(), &mine.clone()) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_clause_about_the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut _end_forms: Py = Py::None;
    let mut after: Py = Py::None;
    let mut first_forms: Py = Py::None;
    let mut here: Py = Py::None;
    let mut index: Py = Py::None;
    let mut span: Py = Py::None;
    let mut starts: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when one of the line's frame clauses is about the thing in hand, not a thing left out.\n\n    \"how do i roll out pastry without it cracking\" has one; \"how do i sleep without pills\" and\n    \"can i make pesto without pine nuts\" have none, and nothing about them changes.\n    ");
    let __unpk1 = py_unpack(&_frame_starts(&[]), 2, None);
    starts = __unpk1[0].clone();
    _end_forms = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    here = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&here.clone()))) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&starts.clone());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    first_forms = __unpk5[0].clone();
    words = __unpk5[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&here.clone(), &first_forms.clone())))) {
    continue;
    }
    span = slice_or_raise(&forms.clone(), &index.clone(), &add(&index.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[span.clone(), words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk9829 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk9829[0].clone(); let mut wanted: Py = __unpk9829[1].clone(); __out.push({ let __b = word.clone(); if !py_truthy(&__b) { __b } else { bitand(&word.clone(), &_forms(&[wanted.clone()])) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    after = add(&index.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&_the_subjects_own_clause(&[slice_or_raise(&forms.clone(), &after.clone(), &add(&after.clone(), &Py::Int(3)), &Py::None)])) {
    return Py::Bool(true);
    }
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _frame_starts(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut end_forms: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut starts: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("[(the first word's forms, the phrase, how many words it is)], worked out once.");
    if py_truthy(&Py::Bool(py_is(&g__FRAME_STARTS(), &Py::None))) {
    starts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g_FRAME_WORDS());
    for __item in __iter1 {
    phrase = __item.clone();
    words = mcall(&phrase.clone(), "split", &[]);
    let _ = mcall(&starts.clone(), "append", &[py_tuple(vec![_forms(&[geti(&words.clone(), &Py::Int(0))]), call_builtin("tuple", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))])]);
    }
    end_forms = call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&g_FRAME_ENDS());
    for __item in __iter2 {
    word = __item.clone();
    let __aug3 = bitor(&end_forms.clone(), &_forms(&[word.clone()]));
    end_forms = __aug3;
    }
    s__FRAME_STARTS(py_tuple(vec![starts.clone(), end_forms.clone()]));
    }
    return g__FRAME_STARTS();
    Py::None
}
pub fn _frame_masked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut at: Py = Py::None;
    let mut dead: Py = Py::None;
    let mut end_forms: Py = Py::None;
    let mut first_forms: Py = Py::None;
    let mut found: Py = Py::None;
    let mut here: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut index: Py = Py::None;
    let mut span: Py = Py::None;
    let mut starts: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with its frame clauses emptied, and the frame words it uses.\n\n    Positions stay in place - a span is a tuple of positions into this list, so emptying a\n    position rather than removing it keeps every span an entry has already matched valid.\n    ");
    let __unpk1 = py_unpack(&_frame_starts(&[]), 2, None);
    starts = __unpk1[0].clone();
    end_forms = __unpk1[1].clone();
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    dead = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    index = Py::Int(0);
    while py_truthy(&lt(&index.clone(), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    here = geti(&forms.clone(), &index.clone());
    hit = Py::None;
    if py_truthy(&here.clone()) {
    let __iter2 = py_iter_or_raise(&starts.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    first_forms = __unpk3[0].clone();
    words = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&here.clone(), &first_forms.clone())))) {
    continue;
    }
    span = slice_or_raise(&forms.clone(), &index.clone(), &add(&index.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[span.clone(), words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk3139 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk3139[0].clone(); let mut wanted: Py = __unpk3139[1].clone(); __out.push({ let __b = word.clone(); if !py_truthy(&__b) { __b } else { bitand(&word.clone(), &_forms(&[wanted.clone()])) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    hit = geti(&words.clone(), &Py::Int(0));
    let __aug4 = add(&index.clone(), &sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    index = __aug4;
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&hit.clone(), &Py::None))) {
    let __aug5 = add(&index.clone(), &Py::Int(1));
    index = __aug5;
    continue;
    }
    let _ = mcall(&found.clone(), "add", &[hit.clone()]);
    let __aug6 = add(&index.clone(), &Py::Int(1));
    index = __aug6;
    while py_truthy(&lt(&index.clone(), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    if py_truthy(&{ let __b = geti(&forms.clone(), &index.clone()); if !py_truthy(&__b) { __b } else { bitand(&geti(&forms.clone(), &index.clone()), &end_forms.clone()) } }) {
    break;
    }
    let _ = mcall(&dead.clone(), "add", &[index.clone()]);
    let __aug7 = add(&index.clone(), &Py::Int(1));
    index = __aug7;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![forms.clone(), call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return py_tuple(vec![{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8962 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk8962[0].clone(); let mut word: Py = __unpk8962[1].clone(); __out.push(if py_truthy(&Py::Bool(py_contains(&dead.clone(), &at.clone()))) { call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)) } else { word.clone() }); } py_list(__out) })() }, call_builtin("frozenset", &[found.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _key_names(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frame_words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frame_words"))) };
    let mut item: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut alternative: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("True when this entry's key asks for the frame word itself.");
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &frame_words.clone()) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _unframed(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut frame_words: Py = Py::None;
    let mut got: Py = Py::None;
    let mut masked: Py = Py::None;
    let mut memo: Py = Py::None;
    let _ = py_str("(the forms to read, the memo to read them with, the frame words the line uses).\n\n    An entry that names the frame keeps the whole line. The others read the masked line with a\n    memo of its own: the per-part memo is keyed by the part alone, so the same part asked about\n    two different readings of one line would answer the first reading twice.\n    ");
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[g__FRAME_MASK()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &forms.clone())) } }) {
    return py_tuple(vec![geti(&got.clone(), &Py::Int(1)), geti(&got.clone(), &Py::Int(2)), geti(&got.clone(), &Py::Int(3))]);
    }
    }
    let __unpk1 = py_unpack(&_frame_masked(&[forms.clone()]), 2, None);
    masked = __unpk1[0].clone();
    frame_words = __unpk1[1].clone();
    memo = if py_truthy(&Py::Bool(py_is(&masked.clone(), &forms.clone()))) { seen.clone() } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &g__FRAME_MASK(), py_tuple(vec![forms.clone(), masked.clone(), memo.clone(), frame_words.clone()]));
    }
    return py_tuple(vec![masked.clone(), memo.clone(), frame_words.clone()]);
    Py::None
}
pub fn _without_ask_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut at: Py = Py::None;
    let mut got: Py = Py::None;
    let mut masked: Py = Py::None;
    let mut memo: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut span: Py = Py::None;
    let _ = py_str("(the forms to read an avoid word against, the memo to read them with).\n\n    The line with \"best way\", \"best ways\" or \"how best\" blanked, so a row that disowns \"best\"\n    or \"way\" is not stood down by the asking itself. A memo of its own, for the same reason\n    `_unframed` keeps one: the part memo is keyed by the part alone.\n    ");
    span = py_tuple(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("best way"), py_str("best ways"), py_str("how best")]));
    for __item in __iter1 {
    phrase = __item.clone();
    span = _phrase_span(&[forms.clone(), phrase.clone(), seen.clone()]);
    if py_truthy(&span.clone()) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&span.clone()))) {
    return py_tuple(vec![forms.clone(), seen.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[g__ASK_PHRASE_MASK()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &forms.clone())) } }) {
    return py_tuple(vec![geti(&got.clone(), &Py::Int(1)), geti(&got.clone(), &Py::Int(2))]);
    }
    }
    masked = call_builtin("list", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&span.clone());
    for __item in __iter2 {
    at = __item.clone();
    let _ = py_setitem(&masked.clone(), &at.clone(), call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)));
    }
    masked = call_builtin("tuple", &[masked.clone()]).unwrap_or_else(|e| exc::raise(e));
    memo = if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) { py_dict(vec![]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &g__ASK_PHRASE_MASK(), py_tuple(vec![forms.clone(), masked.clone(), memo.clone()]));
    }
    return py_tuple(vec![masked.clone(), memo.clone()]);
    Py::None
}
pub fn _said_again(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut covered: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "covered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: covered"))) };
    let mut typed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { Py::None };
    let mut after: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms_of_word: Py = Py::None;
    let mut group: Py = Py::None;
    let mut last: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words after the entry's subject are only the news that it has come back.\n\n    \"hay fever again\", \"eczema is back\" and \"my asthma keeps coming back\" state the complaint\n    and ask nothing, the same way \"i have hay fever\" does, and the order of the two is the only\n    difference: a report in front of the subject is `_bare_report`'s, and one behind it is this.\n    Only a row that has taken `report=True` asks this, so the list is the whole of the freedom\n    it gives: nothing in it can name a thing the line is asking about.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&covered.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[typed.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    } else {
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&forms.clone());
    for __item in __iter1 {
    group = __item.clone();
    forms_of_word = if py_truthy(&group.clone()) { call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&group.clone()) { let mut form: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } else { py_list(vec![]) };
    let _ = mcall(&words.clone(), "append", &[if py_truthy(&forms_of_word.clone()) { geti(&forms_of_word.clone(), &Py::Int(0)) } else { py_str("") }]);
    }
    }
    last = call_builtin("max", &[covered.clone()]).unwrap_or_else(|e| exc::raise(e));
    after = slice_or_raise(&words.clone(), &add(&last.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_HAPPENING_AGAIN(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_REPORT_FILLER(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _bare_report(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut covered: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "covered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: covered"))) };
    let mut typed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { Py::None };
    let mut first: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms_of_word: Py = Py::None;
    let mut group: Py = Py::None;
    let mut index: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut person: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line reports the entry's subject about themselves and asks nothing.\n\n    \"i have tinnitus\" and \"ive got hay fever that keeps coming back\" both start with the report\n    and then name the subject: what comes before the subject is only the report (\"i\", \"have\",\n    \"got\", \"think\"), so the person is telling you about themselves and the entry is the answer.\n    A line that puts anything else before the subject is asking or telling something else -\n    \"do i have hayfever\" carries its question word in front, \"my hayfever is bad\" names the\n    subject first - and keeps the two-position rule like every other line.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&covered.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[typed.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    } else {
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&forms.clone());
    for __item in __iter1 {
    group = __item.clone();
    forms_of_word = if py_truthy(&group.clone()) { call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&group.clone()) { let mut form: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } else { py_list(vec![]) };
    let _ = mcall(&words.clone(), "append", &[if py_truthy(&forms_of_word.clone()) { geti(&forms_of_word.clone(), &Py::Int(0)) } else { py_str("") }]);
    }
    }
    first = call_builtin("min", &[covered.clone()]).unwrap_or_else(|e| exc::raise(e));
    lead = Py::Bool(false);
    person = Py::Bool(false);
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    index = __item.clone();
    word = geti(&words.clone(), &index.clone());
    if py_truthy(&Py::Bool(py_contains(&g_REPORT_LEADS(), &word.clone()))) {
    lead = Py::Bool(true);
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_REPORT_PERSON(), &word.clone()))) {
    person = Py::Bool(true);
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_REPORT_FILLER(), &word.clone()))) {
    continue;
    }
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = person.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&lead.clone())) } }) {
    return Py::Bool(false);
    }
    return lead.clone();
    Py::None
}
pub fn _asks_to_explain(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut typed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { Py::None };
    let mut form: Py = Py::None;
    let mut forms_of_word: Py = Py::None;
    let mut group: Py = Py::None;
    let mut index: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words in front of the entry's subject are a request to explain it.\n\n    The subject is `start`, the first word of the part of the key that was matched; everything\n    before it has to be one of the ask's own words. \"what is sepsis\" and \"what are the symptoms\n    of shingles\" pass; \"what is the best way to clean a sofa\" does not, because \"best\", \"way\",\n    \"clean\" are not words of a request to explain the sofa, and the row that names a sofa is not\n    the row the line wants.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&start.clone())); if py_truthy(&__b) { __b } else { gt(&start.clone(), &Py::Int(12)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[typed.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    } else {
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&forms.clone());
    for __item in __iter1 {
    group = __item.clone();
    forms_of_word = if py_truthy(&group.clone()) { call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&group.clone()) { let mut form: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } else { py_list(vec![]) };
    let _ = mcall(&words.clone(), "append", &[if py_truthy(&forms_of_word.clone()) { geti(&forms_of_word.clone(), &Py::Int(0)) } else { py_str("") }]);
    }
    }
    lead = Py::Bool(false);
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[start.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    index = __item.clone();
    word = geti(&words.clone(), &index.clone());
    if py_truthy(&Py::Bool(py_contains(&g_EXPLAIN_LEADS(), &word.clone()))) {
    lead = Py::Bool(true);
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_EXPLAIN_FILLER(), &word.clone()))) {
    continue;
    }
    return Py::Bool(false);
    }
    return lead.clone();
    Py::None
}
pub fn _only_as_a_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut span: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "span") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: span"))) };
    let mut covered: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "covered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: covered"))) };
    let mut item: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut typed: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { Py::None };
    let mut alternative: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms_of_word: Py = Py::None;
    let mut group: Py = Py::None;
    let mut index: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the entry's subject is only ever said as a description of something else.\n\n    \"something light\", \"anything sweet\" - the word is an adjective where it stands, and a row\n    that names it as the subject of its own has nothing to answer. The row that explains light\n    was the answer to \"what should i watch tonight, something light\" and to \"whats on tonight,\n    something light\".\n\n    A single word is the whole of the shape. The description is one word and the row reads that\n    one word as its subject; a key phrase of several words is a clause the row reads for itself,\n    and the pronoun in front of it is the subject of that clause, which names the thing the row\n    is about (\"everything tastes of metal\", \"nothing wakes my baby\").\n\n    A row whose subject is a qualifier it was built for keeps the line, and so does one whose\n    other words are the ask itself: \"light\" is the subject group of the light-dinner row, so\n    \"something light for lunch\" is exactly its question, and the mood row asks \"something\" of\n    its own. What tells the rows apart is whether anything beside the subject is a word this row\n    answers with: the definition rows have nothing but their own ask words there (\"what\"), the\n    others have \"lunch\", or the \"something\" their key names.\n\n    Read on the words the person typed, and on the forms otherwise, exactly as\n    `_asks_to_explain` does, so a reading built without `typed` still matches.\n    ");
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::Bool(false);
    }
    start = call_builtin("min", &[span.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&lt(&start.clone(), &Py::Int(1))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[typed.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    } else {
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&forms.clone());
    for __item in __iter1 {
    group = __item.clone();
    forms_of_word = if py_truthy(&group.clone()) { call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&group.clone()) { let mut form: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } else { py_list(vec![]) };
    let _ = mcall(&words.clone(), "append", &[if py_truthy(&forms_of_word.clone()) { geti(&forms_of_word.clone(), &Py::Int(0)) } else { py_str("") }]);
    }
    }
    if py_truthy(&ge(&sub(&start.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_contains(&g_AS_A_KIND(), &geti(&words.clone(), &sub(&start.clone(), &Py::Int(1)))))) {
    return Py::Bool(false);
    }
    asked = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&{ let __b = gattr(&item.clone(), "ask"); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    part = __item.clone();
    let __iter3 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { py_tuple(vec![part.clone()]) } else { part.clone() });
    for __item in __iter3 {
    alternative = __item.clone();
    let __iter4 = py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[alternative.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]));
    for __item in __iter4 {
    word = __item.clone();
    let _ = mcall(&asked.clone(), "add", &[word.clone()]);
    let __aug5 = bitor(&asked.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_forms(&[word.clone()])) { let mut form: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() });
    asked = __aug5;
    }
    }
    }
    said = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter6 = py_iter_or_raise(&covered.clone());
    for __item in __iter6 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&span.clone(), &index.clone())); if py_truthy(&__b) { __b } else { ge(&index.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    continue;
    }
    let _ = mcall(&said.clone(), "add", &[geti(&words.clone(), &index.clone())]);
    }
    let _ = mcall(&said.clone(), "discard", &[py_str("")]);
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&asked.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_EXPLAIN_FILLER(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _says_a_shape(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut shapes: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shapes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: shapes"))) };
    let mut typed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut forms: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut form: Py = Py::None;
    let mut forms_of_word: Py = Py::None;
    let mut group: Py = Py::None;
    let mut line: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line, as the person typed it, holds a figure of the kind a row is about.\n\n    The shapes are given the line's own words joined by spaces (\"its on 4% and the charger is at\n    work\"), because a shape reads a figure rather than a word and the word forms of \"4%\" say\n    nothing: this is the one signal in the matcher that is not a key word, so it is handed the\n    line and answers for itself. Words are taken from `typed` when it lines up with the forms and\n    from the forms otherwise, exactly as `_bare_report` does, so a reading built without `typed`\n    still matches.\n    ");
    if py_truthy(&{ let __b = call_builtin("isinstance", &[typed.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    } else {
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&forms.clone());
    for __item in __iter1 {
    group = __item.clone();
    forms_of_word = if py_truthy(&group.clone()) { call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&group.clone()) { let mut form: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } else { py_list(vec![]) };
    let _ = mcall(&words.clone(), "append", &[if py_truthy(&forms_of_word.clone()) { geti(&forms_of_word.clone(), &Py::Int(0)) } else { py_str("") }]);
    }
    }
    line = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    let __iter2 = py_iter_or_raise(&shapes.clone());
    for __item in __iter2 {
    shape = __item.clone();
    let __try3 = exc::catch(|| {
    if py_truthy(&py_call(&shape, &[line.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret4: Py = Py::Bool(false);
    let mut __val5: Py = Py::None;
    let mut __unhandled6: Option<exc::PyExc> = None;
    match __try3 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret4 = __p[0].clone(); __val5 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled7 = false;
            if !__handled7 && exc::matches_any(&__e, &["Exception"]) {
                __handled7 = true;
    continue;
            }
            if !__handled7 { __unhandled6 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled6 { exc::raise(__e); }
    if py_truthy(&__ret4) { return __val5; }
    }
    return Py::Bool(false);
    Py::None
}
pub fn score(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut typed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { Py::None };
    let mut seen: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut frame: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { Py::Bool(true) };
    let mut framed: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "framed") { v.clone() }
    else { Py::Bool(false) };
    let mut _u: Py = Py::None;
    let mut ask_forms: Py = Py::None;
    let mut ask_seen: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut covered: Py = Py::None;
    let mut explained: Py = Py::None;
    let mut failing_at: Py = Py::None;
    let mut first: Py = Py::None;
    let mut frame_words: Py = Py::None;
    let mut framed_one: Py = Py::None;
    let mut grouped: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut index: Py = Py::None;
    let mut key: Py = Py::None;
    let mut masked: Py = Py::None;
    let mut memo: Py = Py::None;
    let mut one_word: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts_said: Py = Py::None;
    let mut removal: Py = Py::None;
    let mut shaped: Py = Py::None;
    let mut span: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut total: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The entry's score for a line, or 0 when it is not about the entry.\n\n    Two different words of the key have to be in the line. Without that, a part like\n    (\"rain\", (\"raining\", \"why\")) scores 4 from \"it rained\" alone, because raining and\n    rain are loose forms of the same word; one passing mention would then answer the\n    line. The same rule keeps \"freeze\" from a freezer fact out of a call that freezes.\n\n    The first part is the subject and it has to be said. An entry written as\n    (subject) x (what is said about it) x (how it is asked) can otherwise reach the\n    total from the last two parts alone: \"im thinking of getting a bike to cycle to\n    work\" brought \"work\" and \"get\" and was answered by the bank-holidays entry, and\n    \"how do i unblock a slow kitchen sink without calling a plumber\" reached the\n    cupboard clear-out on \"kitchen\" and \"how do i\". Both lines name a thing the entry\n    never mentions.\n\n    That only holds where the key is written that way. An entry whose key is a flat\n    list of phrasings - `water-in-ear` is \"water in my ear\", \"water is stuck in my ear\",\n    \"swimmers ear\", and twenty more, each a whole subject - has no first part to insist\n    on, and reading its first phrasing as the subject took \"i have swimmers ear\" off\n    its own answer: \"Noted: you have swimmers ear.\" So the rule is applied to every key\n    that has a group in it, which is what (subject) x (about) x (ask) looks like.\n\n    A key that is only partly grouped still has a first part: `email-to-teacher` is\n    ('teacher', 'childs', 'school', ('email', 'write', 'message')), and with the rule\n    read off the first part alone it answered \"how do i choose a school for my child\"\n    on the bare word \"school\" - the school was named, the email never was. So the test\n    is whether any part is a group, not whether the first one is.\n\n    framed: the line is the subject inside an advice ask (\"what should i do about the\n    condensation\"), which spends its other words on the asking. Only for the entries in\n    ASK_FRAME_IDS, whose subject is one everyday noun, the ask around it is the second signal\n    and one word of the subject part is enough.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&forms.clone()))) {
    return Py::Int(0);
    }
    if py_truthy(&{ let __b = typed.clone(); if !py_truthy(&__b) { __b } else { _under_their_own_hands(&[item.clone(), typed.clone()]) } }) {
    return Py::Int(0);
    }
    let __unpk1 = py_unpack(&if py_truthy(&frame.clone()) { _unframed(&[forms.clone(), seen.clone()]) } else { py_tuple(vec![forms.clone(), seen.clone(), call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e))]) }, 3, None);
    masked = __unpk1[0].clone();
    memo = __unpk1[1].clone();
    frame_words = __unpk1[2].clone();
    if py_truthy(&{ let __b = frame_words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_key_names(&[frame_words.clone(), item.clone()]))) } }) {
    let __unpk2 = py_unpack(&py_tuple(vec![masked.clone(), memo.clone()]), 2, None);
    forms = __unpk2[0].clone();
    seen = __unpk2[1].clone();
    }
    framed_one = call_builtin("bool", &[{ let __b = framed.clone(); if !py_truthy(&__b) { __b } else { { let __b = gattr(&item.clone(), "key"); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ASK_FRAME_IDS(), &call_builtin("getattr", &[item.clone(), py_str("id"), py_str("")]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { _the_whole_subject(&[geti(&gattr(&item.clone(), "key"), &Py::Int(0)), forms.clone(), seen.clone()]) } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&gattr(&item.clone(), "report"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&gattr(&item.clone(), "explain"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&framed_one.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&gattr(&item.clone(), "shape"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_two_positions(&[forms.clone(), item.clone()]))) } } } } } } } }) {
    return Py::Int(0);
    }
    if py_truthy(&{ let __b = gattr(&item.clone(), "ask"); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_ask_said(&[gattr(&item.clone(), "ask"), forms.clone(), seen.clone()]))) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_wanting(&[forms.clone(), seen.clone()]))); if py_truthy(&__b) { __b } else { _key_on_the_want(&[item.clone(), forms.clone(), seen.clone()]) } }) {
    return Py::Int(0);
    }
    }
    total = Py::Int(0);
    covered = py_list(vec![]);
    first = py_tuple(vec![]);
    removal = _says_removal(&[forms.clone()]);
    grouped = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = gattr(&item.clone(), "key"); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut part: Py = __item.clone(); __out.push(call_builtin("isinstance", &[part.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    let __iter3 = py_iter_or_raise(&call_builtin("enumerate", &[gattr(&item.clone(), "key")]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    index = __unpk4[0].clone();
    part = __unpk4[1].clone();
    let __unpk5 = py_unpack(&_part_match(&[part.clone(), forms.clone(), covered.clone(), seen.clone()]), 3, None);
    weight = __unpk5[0].clone();
    words = __unpk5[1].clone();
    span = __unpk5[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&index.clone(), &Py::Int(0))); if !py_truthy(&__b) { __b } else { { let __b = typed.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("len", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let __b = lt(&geti(&span.clone(), &Py::Int(0)), &call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _asking_with(&[typed.clone(), geti(&span.clone(), &Py::Int(0))]) } } } } } } } }) {
    bare = call_builtin("list", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&bare.clone(), &geti(&span.clone(), &Py::Int(0)), call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)));
    let __unpk6 = py_unpack(&_part_match(&[part.clone(), bare.clone(), covered.clone()]), 3, None);
    weight = __unpk6[0].clone();
    _u = __unpk6[1].clone();
    span = __unpk6[2].clone();
    } else {
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { { let __b = removal.clone(); if !py_truthy(&__b) { __b } else { { let __b = gt(&weight.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = _only_the_frame(&[words.clone()]); if !py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } } } } } }) {
    spare = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_part_matches(&[part.clone(), forms.clone()])) { let mut hit: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_only_the_frame(&[geti(&hit.clone(), &Py::Int(1))])))) {  __out.push(hit.clone()); } } py_list(__out) })() };
    if py_truthy(&spare.clone()) {
    let __unpk7 = py_unpack(&geti(&spare.clone(), &Py::Int(0)), 3, None);
    weight = __unpk7[0].clone();
    words = __unpk7[1].clone();
    span = __unpk7[2].clone();
    } else {
    weight = if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { Py::Int(1) } else { Py::Int(0) };
    }
    }
    }
    let __aug8 = add(&total.clone(), &weight.clone());
    total = __aug8;
    let _ = mcall(&covered.clone(), "extend", &[span.clone()]);
    if py_truthy(&Py::Bool(py_eq(&index.clone(), &Py::Int(0)))) {
    first = span.clone();
    }
    }
    if py_truthy(&{ let __b = grouped.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&first.clone())) } }) {
    return Py::Int(0);
    }
    bare = { let __b = call_builtin("bool", &[gattr(&item.clone(), "report")]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = _bare_report(&[forms.clone(), covered.clone(), typed.clone()]); if py_truthy(&__b) { __b } else { _said_again(&[forms.clone(), covered.clone(), typed.clone()]) } } } };
    shaped = { let __b = call_builtin("bool", &[gattr(&item.clone(), "shape")]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _says_a_shape(&[gattr(&item.clone(), "shape"), typed.clone(), forms.clone()]) } } } };
    explained = { let __b = call_builtin("bool", &[gattr(&item.clone(), "explain")]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _asks_to_explain(&[forms.clone(), call_builtin("min", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), typed.clone()]) } } } };
    if py_truthy(&{ let __b = first.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&explained.clone())); if !py_truthy(&__b) { __b } else { _only_as_a_kind(&[forms.clone(), first.clone(), covered.clone(), item.clone(), typed.clone()]) } } } }) {
    return Py::Int(0);
    }
    one_word = lt(&call_builtin("len", &[call_builtin("set", &[covered.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    if py_truthy(&{ let __b = one_word.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&bare.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&explained.clone())) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&framed_one.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&shaped.clone())) } }) {
    return Py::Int(0);
    }
    }
    if py_truthy(&{ let __b = bare.clone(); if py_truthy(&__b) { __b } else { { let __b = shaped.clone(); if py_truthy(&__b) { __b } else { explained.clone() } } } }) {
    let __aug9 = add(&total.clone(), &g_WORD_WEIGHT());
    total = __aug9;
    }
    if py_truthy(&_denied(&[forms.clone(), covered.clone()])) {
    return Py::Int(0);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&one_word.clone())); if !py_truthy(&__b) { __b } else { lt(&total.clone(), &g_MIN_SCORE()) } }) {
    return Py::Int(0);
    }
    parts_said = if py_truthy(&typed.clone()) { _body_parts(&[typed.clone()]) } else { Py::None };
    ask_forms = Py::None;
    ask_seen = Py::None;
    let __iter10 = py_iter_or_raise(&gattr(&item.clone(), "avoid"));
    for __item in __iter10 {
    word = __item.clone();
    key = if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&mcall(&word.clone(), "strip", &[]), "lower", &[]) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_ASK_PHRASE_WORDS(), &key.clone()))) {
    if py_truthy(&Py::Bool(py_is(&ask_forms.clone(), &Py::None))) {
    let __unpk11 = py_unpack(&_without_ask_phrase(&[forms.clone(), seen.clone()]), 2, None);
    ask_forms = __unpk11[0].clone();
    ask_seen = __unpk11[1].clone();
    }
    if py_truthy(&_part_score(&[word.clone(), ask_forms.clone(), ask_seen.clone()])) {
    return Py::Int(0);
    }
    } else {
    if py_truthy(&{ let __b = _part_score(&[word.clone(), forms.clone(), seen.clone()]); if py_truthy(&__b) { __b } else { _phrase_span_between(&[forms.clone(), word.clone(), seen.clone()]) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_a_part_the_line_does_not_have(&[word.clone(), parts_said.clone()])))) {
    return Py::Int(0);
    }
    }
    }
    }
    if py_truthy(&{ let __b = typed.clone(); if !py_truthy(&__b) { __b } else { _misspelt_avoid(&[item.clone(), typed.clone()]) } }) {
    return Py::Int(0);
    }
    failing_at = _keeps_failing_at(&[forms.clone(), seen.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&failing_at.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = _keeping_row(&[item.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("min", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &add(&failing_at.clone(), &Py::Int(1)))) } } } } } }) {
    return Py::Int(0);
    }
    return total.clone();
    Py::None
}
pub fn _under_their_own_hands(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut typed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut alternative: Py = Py::None;
    let mut line: Py = Py::None;
    let mut part: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks the bot to do something to a thing of the person's own.\n\n    Only the rows about that thing stand down, and only when the line is an ask to the bot with a\n    verb a hand or an eye would do: \"can you fill in this form\", \"could you print my cv\".\n    ");
    line = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ASKS_FOR_THE_HANDS(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_HAND_OR_EYE_DOES_IT(), "search", &[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_ASKS_WHAT_IT_SAYS(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_OWN_THING_ON_THEIR_SIDE(), "search", &[line.clone()])))) {
    return Py::Bool(false);
    }
    said = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = gattr(&item.clone(), "key"); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { py_tuple(vec![part.clone()]) } else { part.clone() });
    for __item in __iter2 {
    alternative = __item.clone();
    let __aug3 = bitor(&said.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[alternative.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])) { let mut piece: Py = __item.clone(); __out.push(piece.clone()); } py_set(__out) })() });
    said = __aug3;
    }
    }
    return call_builtin("bool", &[bitand(&said.clone(), &g_THEIR_OWN_HANDS_THINGS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn not_a_count_for_the_box_or_the_dish(args: &[Py]) -> Py {
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
    let _ = py_str("False when a \"how many\" line counts into a dish or asks how long the food keeps.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    text = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&{ let __b = mcall(&g_GOES_IN(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HOW_MANY_DAYS(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_KEEPS_OR_STORES(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_COUNTED_INTO_A_THING(), "search", &[text.clone()]) } } } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn a_food_is_being_kept(args: &[Py]) -> Py {
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
    let mut food: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks about keeping a food: it names a food and a keeping word.\n\n    A row about something else that shares a word with such a line answers the wrong half of it.\n    The container in \"why does bread go mouldy in a plastic bag\" is packaging, the bag in \"why\n    does bread go mouldy faster in a bag\" is where the loaf was kept, and the fridge in \"is it\n    better to keep bread in the fridge\" is where it is kept - so the garment rows, the carrier\n    bag charge and the which-carbohydrate-is-better row all stand down and the bread rows answer.\n    \"My coat smells of curry\" names no keeping word and no food, so nothing here touches it.\n    \"How do i keep my gym bag smelling fresh\" names a garment and a keeping word but no food.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    text = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KEEPING_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    words = { let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    let __iter1 = py_iter_or_raise(&g_KEPT_FOODS());
    for __item in __iter1 {
    food = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&food.clone(), &py_str(" ")))) {
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &food.clone()))) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &food.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &add(&add(&py_str(" "), &food.clone()), &py_str(" ")))) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn only_a_container(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut found: Py = Py::None;
    let mut one: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when every word of `names` the line says is a container a thing is kept in.\n\n    \"Why does bread go mouldy in a plastic bag\" says \"bag\" and nothing else of the garment table,\n    so the garment rows stand down for the loaf. \"My gym bag smells musty\" has no food in it, so\n    `a_food_is_being_kept` is already false and this is never asked. \"My leather jacket smells of\n    cheese\" says two words of that table and leather is not a container, so the jacket keeps its\n    answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    text = add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }])), &py_str(" "));
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&text.clone(), &add(&add(&py_str(" "), &one.clone()), &py_str(" "))))) {  __out.push(one.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CONTAINERS(), &one.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _keeping_row(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut alternative: Py = Py::None;
    let mut keeps: Py = Py::None;
    let mut known: Py = Py::None;
    let mut lives: Py = Py::None;
    let mut part: Py = Py::None;
    let mut shaped: Py = Py::None;
    let _ = py_str("True when this row answers a \"how long does it keep\" question.");
    known = mcall(&g__KEEPING_ROWS(), "get", &[call_builtin("id", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(py_is(&known.clone(), &Py::None))) {
    keeps = Py::Bool(false);
    lives = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    shaped = mcall(&mcall(&alternative.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g__KEEPING_KEY(), &shaped.clone()))) {
    keeps = Py::Bool(true);
    } else {
    if py_truthy(&Py::Bool(py_contains(&g__LIFE_KEY(), &shaped.clone()))) {
    lives = Py::Bool(true);
    }
    }
    }
    }
    known = { let __b = keeps.clone(); if !py_truthy(&__b) { __b } else { lives.clone() } };
    let _ = py_setitem(&g__KEEPING_ROWS(), &call_builtin("id", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)), known.clone());
    }
    return known.clone();
    Py::None
}
pub fn _gerund(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut form: Py = Py::None;
    let _ = py_str("The -ing verb at this position of the line, or \"\" when the word is a thing.");
    let __iter1 = py_iter_or_raise(&geti(&forms.clone(), &index.clone()));
    for __item in __iter1 {
    form = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__ING_NOUNS(), &form.clone())); if py_truthy(&__b) { __b } else { { let __b = lt(&call_builtin("len", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&form.clone(), "endswith", &[py_str("ing")]))) } } } }) {
    continue;
    }
    return form.clone();
    }
    return py_str("");
    Py::None
}
pub fn _later_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut reach: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reach") { v.clone() }
    else { Py::Int(3) };
    let mut seen: Py = Py::None;
    let mut step: Py = Py::None;
    let _ = py_str("The words after a position, for the state a \"keeps going ...\" line names.");
    seen = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[index.clone(), call_builtin("min", &[add(&index.clone(), &reach.clone()), call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    let __aug2 = bitor(&seen.clone(), &geti(&forms.clone(), &step.clone()));
    seen = __aug2;
    }
    return seen.clone();
    Py::None
}
pub fn _keeps_failing_at(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut failing: Py = Py::None;
    let mut gerund: Py = Py::None;
    let mut got: Py = Py::None;
    let mut index: Py = Py::None;
    let mut result: Py = Py::None;
    let _ = py_str("Where the \"keeps\" of a fault verb sits, or None when no \"keeps\" is one.\n\n    \"i keep running to the loo\" has the shape of \"my bread keeps coming out flat\" and is the\n    opposite case: the thing the row is about comes *after* the verb (what the person keeps\n    doing), so the verb is the person's own phrasing of a complaint and not the fault. The\n    caller compares this position with where the entry's subject matched.\n    ");
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    got = mcall(&seen.clone(), "get", &[py_str("keeps-failing-at")]);
    if py_truthy(&Py::Bool(!py_is(&got.clone(), &Py::None))) {
    return if py_truthy(&Py::Bool(py_is(&got.clone(), &Py::Bool(false)))) { Py::None } else { got.clone() };
    }
    }
    failing = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&geti(&forms.clone(), &index.clone()), &g__KEEPING_VERB())))) {
    continue;
    }
    gerund = _gerund(&[forms.clone(), add(&index.clone(), &Py::Int(1))]);
    if py_truthy(&Py::Bool(!py_truthy(&gerund.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g__RECURRING(), &gerund.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__RECURRING_ANYWAY(), &gerund.clone())); if !py_truthy(&__b) { __b } else { bitand(&_later_words(&[forms.clone(), add(&index.clone(), &Py::Int(1))]), &g__RECURRING_WITH()) } }) {
    continue;
    }
    result = { let __b = bitand(&_later_words(&[forms.clone(), add(&index.clone(), &Py::Int(1))]), &call_builtin("frozenset", &[py_tuple(vec![py_str("out")])]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = bitand(&_later_words(&[forms.clone(), add(&index.clone(), &Py::Int(1))]), &g__RESULT_VERB()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__RESULT_VERB(), &gerund.clone())) } } } };
    if py_truthy(&{ let __b = result.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&bitor(&_later_words(&[forms.clone(), add(&index.clone(), &Py::Int(1))]), &py_set(vec![gerund.clone()])), &g__SPOILING()))) } }) {
    failing = index.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_is(&seen.clone(), &Py::None))) {
    let _ = py_setitem(&seen.clone(), &py_str("keeps-failing-at"), if py_truthy(&Py::Bool(py_is(&failing.clone(), &Py::None))) { Py::Bool(false) } else { failing.clone() });
    }
    return failing.clone();
    Py::None
}
pub fn _keeps_failing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let _ = py_str("True when \"keeps\" is the fault verb, as in \"my bread keeps coming out flat\".");
    return Py::Bool(!py_is(&_keeps_failing_at(&[forms.clone(), seen.clone()]), &Py::None));
    Py::None
}
pub fn _the_whole_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut seen: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut _weight: Py = Py::None;
    let mut _words: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut index: Py = Py::None;
    let mut span: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word_forms: Py = Py::None;
    let _ = py_str("True when the entry's subject part matches every word the subject is made of.\n\n    \"what should i do about the damp patch\" is a patch, not condensation: the entry whose subject\n    part holds \"damp\" matches one word of it, and one word of a two-word subject is not what the\n    ask is about.\n    ");
    span = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&_part_matches(&[part.clone(), forms.clone(), seen.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _weight = __unpk2[0].clone();
    _words = __unpk2[1].clone();
    hit = __unpk2[2].clone();
    let _ = mcall(&span.clone(), "update", &[hit.clone()]);
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8550 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk8550[0].clone(); let mut word_forms: Py = __unpk8550[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&bitand(&word_forms.clone(), &g__SUBJECT_POINTS())))) {  __out.push(index.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut index: Py = __item.clone(); __out.push(Py::Bool(py_contains(&span.clone(), &index.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _rarity(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entries: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut alternative: Py = Py::None;
    let mut counts: Py = Py::None;
    let mut item: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("{key word: how many entries use it}, so rare words can count for more.");
    counts = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&entries.clone());
    for __item in __iter1 {
    item = __item.clone();
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter2 {
    part = __item.clone();
    let __iter3 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter3 {
    alternative = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&words.clone(), "update", &[mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[])]);
    }
    }
    }
    let __iter4 = py_iter_or_raise(&words.clone());
    for __item in __iter4 {
    word = __item.clone();
    let _ = py_setitem(&counts.clone(), &word.clone(), add(&mcall(&counts.clone(), "get", &[word.clone(), Py::Int(0)]), &Py::Int(1)));
    }
    }
    return counts.clone();
    Py::None
}
pub fn _the_tail_asks_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut ask_words: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the line's last question starts, or -1 when it asks in one breath.\n\n    People put the reason first and the question last: \"my nan is 80 next month and we want a\n    cake, what should i get her\", \"i have a christening sunday and it is meant to rain, what do i\n    wear\". The words after that question word are what was asked.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__TAIL_ASK_WORDS(), &Py::None))) {
    s__TAIL_ASK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("what"), py_str("whats"), py_str("which"), py_str("how"), py_str("where"), py_str("when"), py_str("who"), py_str("why"), py_str("should"), py_str("shall"), py_str("could"), py_str("would"), py_str("can")])) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)), "union", args))(&__args) });
    }
    ask_words = g__TAIL_ASK_WORDS();
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), Py::Int(3), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    word = geti(&forms.clone(), &index.clone());
    if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { bitand(&word.clone(), &ask_words.clone()) } }) {
    return index.clone();
    }
    }
    return neg(&Py::Int(1));
    Py::None
}
pub fn _rank(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut rarity: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rarity") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rarity"))) };
    let mut seen: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { Py::None };
    let mut matched: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut says_what: Py = Py::None;
    let mut span: Py = Py::None;
    let mut tail_from: Py = Py::None;
    let mut total: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Score plus specificity: a key word only a few entries use says more about the line.\n\n    A key part the line only reaches inside its last question, through the asking words alone\n    (\"what\",\"should\" / \"what\",\"do\",\"i\",\"do\"), is the asking of that question and not a thing the\n    row is about. The row that lists the asking was winning lines that put the reason first:\n    `babyfood-by-age` (key part \"what should\") took \"my nan is 80 next month and we want a cake,\n    what should i get her\" off `gift-mum`, and `telling-a-child-off` (key part \"what do i do\")\n    took \"the boiler broke and the landlord is ignoring me, what do i do\" off the landlord card.\n    The same part in the first clause is untouched, and so is any part that reaches a real word\n    of the question (\"what should i bake\", \"what do i wear\").\n    ");
    tail_from = _the_tail_asks_about(&[forms.clone()]);
    matched = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&item.clone(), "key")) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(forms.clone()); __kws.extend(vec![("seen".to_string(), seen.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _part_match(args))(&__args) }); } py_list(__out) })() };
    said = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&matched.clone()) { let __unpk7742 = py_unpack(&__item.clone(), 3, None); let mut weight: Py = __unpk7742[0].clone(); let mut words: Py = __unpk7742[1].clone(); let mut span: Py = __unpk7742[2].clone(); if py_truthy(&{ let __b = weight.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = ge(&tail_from.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = ge(&call_builtin("min", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &tail_from.clone()); if !py_truthy(&__b) { __b } else { _only_the_frame(&[words.clone()]) } } } })) } }) {  __out.push(weight.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    total = Py::Int(0);
    says_what = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __unpk2 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __args.push(forms.clone()); __kws.extend(vec![("seen".to_string(), seen.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _part_match(args))(&__args) }, 3, None);
    weight = __unpk2[0].clone();
    words = __unpk2[1].clone();
    span = __unpk2[2].clone();
    if py_truthy(&{ let __b = weight.clone(); if !py_truthy(&__b) { __b } else { { let __b = gt(&said.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = ge(&tail_from.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = ge(&call_builtin("min", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &tail_from.clone()); if !py_truthy(&__b) { __b } else { _only_the_frame(&[words.clone()]) } } } } } } } }) {
    continue;
    }
    let __aug3 = add(&total.clone(), &add(&weight.clone(), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&rarity.clone(), "get", &[word.clone(), Py::Int(0)]), &Py::Int(1)))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    total = __aug3;
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(!py_contains(&g_ASK_FRAME(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_WHEN_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_A_LENGTH_WORD(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_STAND_IN(), &word.clone())) } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    says_what = Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&says_what.clone()))) {
    return Py::Int(1);
    }
    return add(&total.clone(), &gattr(&item.clone(), "bias"));
    Py::None
}
pub fn _key_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut alternative: Py = Py::None;
    let mut part: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words the entry's key asks for, as written in it (\"roast chicken\" gives two).");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&words.clone(), "update", &[mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[])]);
    }
    }
    }
    return words.clone();
    Py::None
}
pub fn _key_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut alternative: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut got: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every form of every word the entry's key asks for, worked out once for the entry.\n\n    Read by the two-position test in score(), which is asked about the same few thousand\n    entries on every line. The entry is kept beside its forms because the card is keyed by id:\n    an Entry a test built and let go must not be read as the one that took its id.\n    ");
    got = mcall(&g__KEY_FORMS(), "get", &[call_builtin("id", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &item.clone())) } }) {
    return geti(&got.clone(), &Py::Int(1));
    }
    forms = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter3 = py_iter_or_raise(&mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[]));
    for __item in __iter3 {
    word = __item.clone();
    let __aug4 = bitor(&forms.clone(), &_forms(&[word.clone()]));
    forms = __aug4;
    }
    }
    }
    }
    forms = call_builtin("frozenset", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&ge(&call_builtin("len", &[g__KEY_FORMS()]).unwrap_or_else(|e| exc::raise(e)), &g__FORMS_CACHE_KEEP())) {
    let _ = mcall(&g__KEY_FORMS(), "clear", &[]);
    }
    let _ = py_setitem(&g__KEY_FORMS(), &call_builtin("id", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![item.clone(), forms.clone()]));
    return forms.clone();
    Py::None
}
pub fn _two_positions(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut item: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut word_forms: Py = Py::None;
    let _ = py_str("True when the line can have a form of the entry's key in two different places.\n\n    score() answers 0 to an entry that covers fewer than two positions of the line, and an\n    entry with one of its key's words in the line covers one position at most. Reading the\n    line for the entry's forms is a handful of set tests, while the parts of its key are a few\n    dozen phrase questions, so the entries that can only score 0 are dropped here: on a random\n    deck turn that is 63% of the entries the line reaches. A phrase of the key needs its words\n    side by side, which this does not check, so what it finds is an upper bound: no entry it\n    drops could have covered two positions.\n    ");
    key = _key_forms(&[item.clone()]);
    found = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&forms.clone());
    for __item in __iter1 {
    word_forms = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&word_forms.clone(), "isdisjoint", &[key.clone()])))) {
    if py_truthy(&found.clone()) {
    return Py::Bool(true);
    }
    found = Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _word_index(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entries: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut at: Py = Py::None;
    let mut cached: Py = Py::None;
    let mut form: Py = Py::None;
    let mut index: Py = Py::None;
    let mut item: Py = Py::None;
    let mut position: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("{form: [positions of the entries that ask for it]}, so a line starts from its own words.\n\n    score() gives an entry 0 unless two different words of its key are in the line, so an entry\n    that shares no word with the line can never win it. The knowledge base is 2,500 entries and\n    a line has six words, so walking every entry a turn cost a reply about a second of processor\n    time; looking up the words the line does have cuts the scan to the entries that can answer it\n    at all. Positions keep the registry's order, which the tie-break in `_best` needs.\n    ");
    cached = mcall(&g__INDEX(), "get", &[call_builtin("id", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&cached.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&cached.clone(), &Py::Int(0)), &entries.clone())) } }) {
    return geti(&cached.clone(), &Py::Int(1));
    }
    index = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    position = __unpk2[0].clone();
    item = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&_key_words(&[item.clone()]));
    for __item in __iter3 {
    word = __item.clone();
    let __iter4 = py_iter_or_raise(&_forms(&[word.clone()]));
    for __item in __iter4 {
    form = __item.clone();
    at = mcall(&index.clone(), "get", &[form.clone()]);
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    let _ = py_setitem(&index.clone(), &form.clone(), py_list(vec![position.clone()]));
    } else {
    if py_truthy(&Py::Bool(!py_eq(&geti(&at.clone(), &neg(&Py::Int(1))), &position.clone()))) {
    let _ = mcall(&at.clone(), "append", &[position.clone()]);
    }
    }
    }
    }
    }
    while py_truthy(&ge(&call_builtin("len", &[g__INDEX()]).unwrap_or_else(|e| exc::raise(e)), &g__CACHE_KEEP())) {
    let _ = mcall(&g__INDEX(), "pop", &[call_builtin("next", &[call_builtin("iter", &[g__INDEX()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    let _ = py_setitem(&g__INDEX(), &call_builtin("id", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![entries.clone(), index.clone()]));
    return index.clone();
    Py::None
}
pub fn _reaching(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entries: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut form: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut index: Py = Py::None;
    let mut position: Py = Py::None;
    let mut positions: Py = Py::None;
    let mut word_forms: Py = Py::None;
    let _ = py_str("The entries that share a word with the line, in the registry's order, or all of them.\n\n    A line of unknown words (a name, \"ok then\") reaches nothing, and the caller still has to see\n    every entry, so an empty answer is passed on as the whole list rather than as no candidates.\n    ");
    index = _word_index(&[entries.clone()]);
    hit = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&forms.clone());
    for __item in __iter1 {
    word_forms = __item.clone();
    let __iter2 = py_iter_or_raise(&word_forms.clone());
    for __item in __iter2 {
    form = __item.clone();
    positions = mcall(&index.clone(), "get", &[form.clone()]);
    if py_truthy(&positions.clone()) {
    let _ = mcall(&hit.clone(), "update", &[positions.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&hit.clone()))) {
    return entries.clone();
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("sorted", &[hit.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut position: Py = __item.clone(); __out.push(geti(&entries.clone(), &position.clone())); } py_list(__out) })() };
    Py::None
}
pub fn _domain_rarity(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entries: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut alternative: Py = Py::None;
    let mut by_domain: Py = Py::None;
    let mut counts: Py = Py::None;
    let mut domain: Py = Py::None;
    let mut every: Py = Py::None;
    let mut item: Py = Py::None;
    let mut owners: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("({domain: {key word: how many of its entries use it}}, every key word once).");
    let __unpk1 = py_unpack(&py_tuple(vec![py_dict(vec![]), py_dict(vec![])]), 2, None);
    by_domain = __unpk1[0].clone();
    every = __unpk1[1].clone();
    owners = py_dict(vec![]);
    let __iter2 = py_iter_or_raise(&entries.clone());
    for __item in __iter2 {
    item = __item.clone();
    domain = { let __b = call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } };
    counts = mcall(&by_domain.clone(), "setdefault", &[domain.clone(), py_dict(vec![])]);
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter3 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter3 {
    part = __item.clone();
    let __iter4 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter4 {
    alternative = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&words.clone(), "update", &[mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[])]);
    }
    }
    }
    let __iter5 = py_iter_or_raise(&words.clone());
    for __item in __iter5 {
    word = __item.clone();
    let _ = py_setitem(&counts.clone(), &word.clone(), add(&mcall(&counts.clone(), "get", &[word.clone(), Py::Int(0)]), &Py::Int(1)));
    let _ = mcall(&mcall(&owners.clone(), "setdefault", &[word.clone(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), "add", &[{ let __b = call_builtin("getattr", &[item.clone(), py_str("id"), Py::None]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("id", &[item.clone()]).unwrap_or_else(|e| exc::raise(e)) } }]);
    }
    }
    let __iter6 = py_iter_or_raise(&owners.clone());
    for __item in __iter6 {
    word = __item.clone();
    if py_truthy(&le(&geti(&owners.clone(), &word.clone()), &mcall(&g_SLIPS(), "get", &[word.clone(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]))) {
    continue;
    }
    let _ = py_setitem(&every.clone(), &word.clone(), call_builtin("len", &[geti(&owners.clone(), &word.clone())]).unwrap_or_else(|e| exc::raise(e)));
    }
    return py_tuple(vec![by_domain.clone(), every.clone()]);
    Py::None
}
pub fn _vocabulary(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entries: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut by_domain: Py = Py::None;
    let mut cached: Py = Py::None;
    let mut count: Py = Py::None;
    let mut every: Py = Py::None;
    let mut item: Py = Py::None;
    let mut known: Py = Py::None;
    let mut slip: Py = Py::None;
    let mut slips: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Per-domain rarity, every key word, and the forms the knowledge base knows.");
    cached = mcall(&g__CACHE(), "get", &[call_builtin("id", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&cached.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&cached.clone(), &Py::Int(0)), &entries.clone())) } }) {
    return py_tuple(vec![geti(&cached.clone(), &Py::Int(1)), geti(&cached.clone(), &Py::Int(2)), geti(&cached.clone(), &Py::Int(3))]);
    }
    let __unpk1 = py_unpack(&_domain_rarity(&[entries.clone()]), 2, None);
    by_domain = __unpk1[0].clone();
    every = __unpk1[1].clone();
    slips = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entries.clone()) { let mut item: Py = __item.clone(); for __item in py_iter_or_raise(&call_builtin("getattr", &[item.clone(), py_str("slips"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e))) { let mut slip: Py = __item.clone(); __out.push(mcall(&slip.clone(), "lower", &[])); } } py_set(__out) })() };
    every = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&every.clone(), "items", &[])) { let __unpk397 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk397[0].clone(); let mut count: Py = __unpk397[1].clone(); if py_truthy(&Py::Bool(!py_contains(&slips.clone(), &mcall(&word.clone(), "lower", &[])))) {  __out.push((word.clone(), count.clone())); } } py_dict(__out) })() };
    known = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&every.clone());
    for __item in __iter2 {
    word = __item.clone();
    let __aug3 = bitor(&known.clone(), &_forms(&[word.clone()]));
    known = __aug3;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entries.clone()) { let mut item: Py = __item.clone(); __out.push(call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    while py_truthy(&ge(&call_builtin("len", &[g__CACHE()]).unwrap_or_else(|e| exc::raise(e)), &g__CACHE_KEEP())) {
    let _ = mcall(&g__CACHE(), "pop", &[call_builtin("next", &[call_builtin("iter", &[g__CACHE()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    let _ = py_setitem(&g__CACHE(), &call_builtin("id", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![entries.clone(), by_domain.clone(), every.clone(), known.clone()]));
    }
    return py_tuple(vec![by_domain.clone(), every.clone(), known.clone()]);
    Py::None
}
pub fn _y_word_stems(args: &[Py]) -> Py {
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
    let mut form: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The thing a \"-y\" word describes, as the word under the adjective.\n\n    \"my tap is leaky\" is the leak entry's line, \"a creaky gate\" the creak one's, \"a scaly\n    kettle\" the scale one's, \"squeaky brakes\" the squeak one's. The matcher only ever handed\n    these the plural-looking \"leakies\", so an adjective the person typed never met a key that\n    lists the noun or the verb behind it: \"how do i fix a leaky tap\" was answered while \"the\n    garden gate is creaky\", \"my kettle is scaly\" and \"my car brakes are squeaky\" all got \"Got\n    it. Is there anything you'd like a hand with?\".\n\n    English builds the adjective two ways - on the word itself (leak, smell, creak, rust) and on\n    an \"e\" the adjective drops (noise, grease, scale, taste) - so both are offered. Only those\n    two shapes: the doubled consonant of \"muddy\" or \"runny\" is not chased, because the short\n    word under it is as often a different one (\"runny nose\" is not a run, \"sorry\" is not sore).\n    A stem shorter than MIN_FORM, or one from a word whose stem is an everyday word of its own\n    (NOT_THE_ADJECTIVE), is not offered either.\n    ");
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("y")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str("aeiouy"), &geti(&word.clone(), &neg(&Py::Int(2))))) } } } }) {
    return py_tuple(vec![]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_THE_ADJECTIVE(), &word.clone()))) {
    return py_tuple(vec![]);
    }
    stem = slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![stem.clone(), add(&stem.clone(), &py_str("e"))])) { let mut form: Py = __item.clone(); if py_truthy(&ge(&call_builtin("len", &[form.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MIN_FORM())) {  __out.push(form.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_word_itself(args: &[Py]) -> Py {
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
    let _ = py_str("True when the understanding engine holds this exact word, not only a relative of it.");
    return { let __b = Py::Bool(py_contains(&g__COMMON(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::understanding::normalize::g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::understanding::lexicon::g_CHAT(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::understanding::lexicon::g_EXPAND(), &word.clone())) } } } } } };
    Py::None
}
pub fn _read_the_adjective(args: &[Py]) -> Py {
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
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut vocabulary: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "vocabulary") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: vocabulary"))) };
    let mut gave: Py = Py::None;
    let mut index: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Add the thing each \"-y\" line word describes, where the knowledge base asks for it.\n\n    Only a stem some entry already keys as a word counts, so the rule cannot reach anything the\n    knowledge base does not ask about. It is a second read of a line that reached no entry as\n    typed: the words are the same, and the row that keys the adjective itself has already had\n    its turn, so \"my rice came out really sticky again\" stays with the sticky rice row rather\n    than being pulled to a pan-sticking one by the \"stick\" inside it.\n    ");
    gave = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&_a_word_itself(&[word.clone()])) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&_y_word_stems(&[word.clone()]));
    for __item in __iter3 {
    stem = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&vocabulary.clone(), &stem.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&geti(&forms.clone(), &index.clone()), &stem.clone())) } }) {
    let __aug4 = bitor(&geti(&forms.clone(), &index.clone()), &py_set(vec![stem.clone()]));
    let _ = py_setitem(&forms.clone(), &index.clone(), __aug4);
    gave = Py::Bool(true);
    }
    }
    }
    return gave.clone();
    Py::None
}
pub fn _read_a_thing_had(args: &[Py]) -> Py {
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
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Add \"have\" to a \"has\" that says somebody has a thing, in place.\n\n    Only the possession sense: \"my mum has a headache\" and \"my wife has got a bad back\" read as\n    \"have\", and \"he has to get two buses\", \"it has to be quick\" and \"my phone screen has a green\n    blotch\" do not. Four words back is enough to find the person, because a relation is said\n    before the verb (\"my mum\", \"my wife's brother\") and nothing else in the line can be the thing\n    the line is about.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &py_str("has")))) {
    continue;
    }
    after = if py_truthy(&lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { mcall(&call_builtin("str", &[geti(&words.clone(), &add(&index.clone(), &Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g__NOT_A_THING_HAD(), &after.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&after.clone(), &py_str("got"))); if !py_truthy(&__b) { __b } else { { let __b = lt(&add(&index.clone(), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[geti(&words.clone(), &add(&index.clone(), &Py::Int(2)))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &py_str("to"))) } } } }) {
    continue;
    }
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&index.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_PERSON(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __aug3 = bitor(&geti(&forms.clone(), &index.clone()), &_forms(&[py_str("have")]));
    let _ = py_setitem(&forms.clone(), &index.clone(), __aug3);
    }
    Py::None
}
pub fn _read_typos(args: &[Py]) -> Py {
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
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut vocabulary: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "vocabulary") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: vocabulary"))) };
    let mut known: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Add the key word each misspelt line word is a single slip away from, in place.\n\n    Only a word that is not a word of its own is read as a slip. A key word the knowledge base\n    knows is left alone, and so is any ordinary English word, in any of its forms: \"charging\"\n    is a form of one, so a slip of \"changing\" cannot be invented out of it, and \"write\" and\n    \"leave\" are words themselves.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = bitand(&geti(&forms.clone(), &index.clone()), &known.clone()); if py_truthy(&__b) { __b } else { _engine_reads(&[word.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str("'")))) {
    continue;
    }
    let __aug3 = bitor(&geti(&forms.clone(), &index.clone()), &_typo_forms(&[word.clone(), vocabulary.clone()]));
    let _ = py_setitem(&forms.clone(), &index.clone(), __aug3);
    }
    Py::None
}
pub fn _without_possessive(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let _ = py_str("The word without a possessive on the end: \"earth's\" and \"earths\" are \"earth\".");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("'s"), py_str("’s")]));
    for __item in __iter1 {
    tail = __item.clone();
    if py_truthy(&mcall(&word.clone(), "endswith", &[tail.clone()])) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None);
    }
    }
    return word.clone();
    Py::None
}
pub fn _names_a_fault(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut occasion: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "occasion") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: occasion"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether the thing a feeling is about is a thing that is broken.");
    return call_builtin("bool", &[mcall(&g_FEELING_FAULT_WORDS(), "intersection", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&occasion.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _speaker_is_feeling(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pos: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the speaker is the one with the feeling, not somebody the line is about.");
    low = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pos = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FEELING_ABOUT_A_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("fed"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&low.clone(), &add(&pos.clone(), &Py::Int(1)), &add(&pos.clone(), &Py::Int(2)), &Py::None), &py_list(vec![py_str("up")]))) } })) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&slice_or_raise(&low.clone(), &call_builtin("max", &[Py::Int(0), sub(&pos.clone(), &Py::Int(6))]).unwrap_or_else(|e| exc::raise(e)), &pos.clone(), &Py::None));
    for __item in __iter3 {
    back = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FEELING_SPEAKER(), &back.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_a_feared_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut occasion: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "occasion") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: occasion"))) };
    let mut word: Py = Py::None;
    let _ = py_str("Whether the thing a feeling is about is a thing people are afraid of as a class.");
    return call_builtin("bool", &[mcall(&g_FEARED_THINGS(), "intersection", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&occasion.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _feeling_occasion(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut got: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pos: Py = Py::None;
    let mut word: Py = Py::None;
    let mut word_after: Py = Py::None;
    let _ = py_str("The words naming the thing a feeling is about, or None.\n\n    \"how do i stop feeling anxious about my exams\" leaves the ask and the feeling itself, so the\n    anxiety card keeps the line.\n    ");
    low = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pos = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FEELING_ABOUT_A_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("fed"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&low.clone(), &add(&pos.clone(), &Py::Int(1)), &add(&pos.clone(), &Py::Int(2)), &Py::None), &py_list(vec![py_str("up")]))) } })) } }) {
    continue;
    }
    at = add(&pos.clone(), &Py::Int(1));
    if py_truthy(&{ let __b = lt(&at.clone(), &call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("up"), py_str("off")]), &geti(&low.clone(), &at.clone()))) } }) {
    let __aug3 = add(&at.clone(), &Py::Int(1));
    at = __aug3;
    }
    if py_truthy(&{ let __b = ge(&at.clone(), &call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FEELING_ABOUT(), &geti(&low.clone(), &at.clone()))) } }) {
    continue;
    }
    got = py_list(vec![]);
    let __iter4 = py_iter_or_raise(&slice_or_raise(&low.clone(), &add(&at.clone(), &Py::Int(1)), &add(&at.clone(), &Py::Int(6)), &Py::None));
    for __item in __iter4 {
    word_after = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FEELING_ENDS(), &word_after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word_after.clone(), "isalpha", &[]))) } }) {
    break;
    }
    let _ = mcall(&got.clone(), "append", &[word_after.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&got.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FEELING_NAMES_NOTHING(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }))) {
    continue;
    }
    return got.clone();
    }
    return Py::None;
    Py::None
}
pub fn _the_kept_rows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = Py::None;
    let _ = py_str("Every row whose key names keeping a thing somewhere.");
    let mut all_entries: Py = py_func("entries", |args: &[Py]| crate::gen::knowledge::entries(args));
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&all_entries, &[])) { let mut item: Py = __item.clone(); if py_truthy(&_key_names(&[g__KEPT_SOMEWHERE(), item.clone()])) {  __out.push(item.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _a_price_judged_is_not_answered_here(args: &[Py]) -> Py {
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
    let mut found: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut reply: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the line judges a price and this entry's answer holds no price.\n\n    \"is 4 pounds a lot for a chicken\" reached the card for the minutes a four-pound bird takes in\n    the oven, because four pounds is also a weight, and \"is £4 a lot for a pint of milk\" reached\n    the card about what milk does for you. Neither says anything about money, and the question is\n    entirely about money.\n    ");
    let mut a_price_is_judged: Py = py_func("a_price_is_judged", |args: &[Py]| crate::gen::knowledge::prices::a_price_is_judged(args));
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&py_call(&a_price_is_judged, &[text.clone()])))) {
    return Py::Bool(false);
    }
    reply = call_builtin("getattr", &[found.clone(), py_str("reply"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __try1 = exc::catch(|| {
    reply = py_call(&reply, &[py_dict(vec![]), understood.clone()]);
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
    }
    if py_truthy(&Py::Bool(py_is(&reply.clone(), &Py::None))) {
    reply = py_str("");
    }
    if py_truthy(&mcall(&g__HOLDS_MONEY(), "search", &[call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&_a_range_for_the_thing(&[text.clone(), understood.clone()]), &Py::None));
    Py::None
}
pub fn _the_thing_the_chat_is_on(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing the conversation is on, from the reading of the line (memory[\"understood\"]).");
    understood = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    ref_ = mcall(&understood.clone(), "get", &[py_str("ref")]);
    if py_truthy(&call_builtin("isinstance", &[ref_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("it"), py_str("that"), py_str("they"), py_str("them")]));
    for __item in __iter1 {
    key = __item.clone();
    thing = mcall(&ref_.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__POINTS_AT_IT(), &mcall(&call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return call_builtin("str", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    }
    subject = mcall(&understood.clone(), "get", &[py_str("subject")]);
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__POINTS_AT_IT(), &mcall(&call_builtin("str", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return call_builtin("str", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return py_str("");
    Py::None
}
pub fn _a_range_for_the_thing(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let mut question: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The answer the base gives to \"how much is <the thing>\", when it holds a price.");
    found = mcall(&g__THE_THING_JUDGED(), "search", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "rstrip", &[py_str("?.!")])]);
    said = if py_truthy(&found.clone()) { mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "strip", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__POINTS_AT_IT(), &mcall(&said.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    said = _the_thing_the_chat_is_on(&[understood.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    thing = if py_truthy(&found.clone()) { mcall(&py_str(" "), "join", &[add(&mcall(&{ let __b = mcall(&found.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { py_str("a ") } }, "split", &[]), &mcall(&said.clone(), "split", &[]))]) } else { add(&py_str("a "), &said.clone()) };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![modulo(&py_str("how much is %s"), &thing.clone()), modulo(&py_str("how much does %s cost"), &thing.clone())]));
    for __item in __iter1 {
    question = __item.clone();
    entry = find(&[py_dict(vec![(py_str("clean"), question.clone()), (py_str("words"), mcall(&question.clone(), "split", &[]))])]);
    reply = call_builtin("getattr", &[entry.clone(), py_str("reply"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __try2 = exc::catch(|| {
    reply = py_call(&reply, &[py_dict(vec![]), py_dict(vec![(py_str("clean"), question.clone()), (py_str("words"), mcall(&question.clone(), "split", &[]))])]);
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
    }
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__HOLDS_MONEY(), "search", &[call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return call_builtin("str", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return Py::None;
    Py::None
}
pub fn find(args: &[Py]) -> Py {
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
    let mut entries: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { Py::None };
    let mut reject: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { Py::None };
    let mut use_context: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "use_context") { v.clone() }
    else { Py::Bool(false) };
    let mut again: Py = Py::None;
    let mut found: Py = Py::None;
    let mut kept: Py = Py::None;
    let _ = py_str("The best Entry for this reading of a line, or None when nothing fits.\n\n    understood: memory[\"understood\"], with \"clean\" and/or \"words\".\n    entries: an explicit list (tests); by default every domain in bot/knowledge.\n    reject: a callable given an Entry; when it says True the entry is passed over and the next\n    best is returned. An entry whose reply turns a line down (\"how long do i cook it for\" after\n    mince) should not eat the turn that a lower entry can still answer.\n    use_context: also search the line this one leans on, and only when the line alone reached\n    nothing. Off by default, because a line that finds its own entry must be answered from its\n    own words; the module that has read the conversation turns it on for a follow-up, which\n    names no subject of its own (\"How long in the oven?\", \"And if I stuff it?\"), so the words\n    the understanding engine hands on in `leans_on` are searched as well.\n\n    The line is read as it was typed first, and every entry either line can reach is kept. Only\n    when the typed line reaches nothing, and it is a line about somebody the person looks after,\n    is it read again as the ask about that thing: \"my daughter has head lice\" is a question about\n    head lice, and the comb is the answer - while a line that already has its card (\"my baby is\n    teething\" is the teething card) is answered by it exactly as before.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__DREAM(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g__DREAM(), "told_as_a_dream", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&entries.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g__WHERE_I_PUT(), "search", &[mcall(&py_str(" "), "join", &[tokens(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }])])]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&entries.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g__WHERE_TO_KEEP(), "search", &[mcall(&py_str(" "), "join", &[tokens(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }])])]) } }) {
    kept = _the_best_entry(&[understood.clone(), _the_kept_rows(&[]), reject.clone(), use_context.clone()]);
    if py_truthy(&Py::Bool(!py_is(&kept.clone(), &Py::None))) {
    return kept.clone();
    }
    }
    found = _the_best_entry(&[understood.clone(), entries.clone(), reject.clone(), use_context.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&found.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _a_price_judged_is_not_answered_here(&[understood.clone(), found.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    return found.clone();
    }
    let __iter1 = py_iter_or_raise(&_what_is_wrong_with_them(&[understood.clone()]));
    for __item in __iter1 {
    again = __item.clone();
    found = _the_best_entry(&[again.clone(), entries.clone(), reject.clone(), use_context.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    return found.clone();
    }
    }
    let __iter2 = py_iter_or_raise(&_a_thing_that_has_not_gone(&[understood.clone()]));
    for __item in __iter2 {
    again = __item.clone();
    found = _the_best_entry(&[again.clone(), entries.clone(), reject.clone(), use_context.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    return found.clone();
    }
    }
    let __iter3 = py_iter_or_raise(&_without_the_place(&[understood.clone()]));
    for __item in __iter3 {
    again = __item.clone();
    found = _the_best_entry(&[again.clone(), entries.clone(), reject.clone(), use_context.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    return found.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _without_the_place(args: &[Py]) -> Py {
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
    let mut left: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shaped: Py = Py::None;
    let _ = py_str("The same reading with a part of the body the thing sits on taken out, or an empty list.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    said = mcall(&g__A_PLACE_ON_THEM(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&crate::gen::understanding::lexicon::g_BODY_PARTS(), &mcall(&mcall(&said.clone(), "group", &[Py::Int(1)]), "lower", &[]))) } }) {
    return py_list(vec![]);
    }
    left = mcall(&add(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&said.clone(), "start", &[]), &Py::None), &slice_or_raise(&clean.clone(), &mcall(&said.clone(), "end", &[]), &Py::None, &Py::None)), "strip", &[py_str(" ,.;:!?")]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&left.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_list(vec![]);
    }
    shaped = call_builtin("dict", &[understood.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&shaped.clone(), &py_str("clean"), left.clone());
    let _ = py_setitem(&shaped.clone(), &py_str("words"), tokens(&[left.clone()]));
    return py_list(vec![shaped.clone()]);
    Py::None
}
pub fn _a_thing_that_has_not_gone(args: &[Py]) -> Py {
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
    let mut again: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_so: Py = Py::None;
    let mut shaped: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The same line read as the ask about the thing said not to have gone.\n\n    Only the report is reshaped: the words before \"has not gone\" are the thing. Two openers are\n    tried for the same reason `_what_is_wrong_with_them` tries two - a card is keyed on one ask\n    and not the other (\"a rash that wont go away\" finds the rash card, \"how do i treat a rash\"\n    finds it too, while a bare \"the rash\" is not an ask).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    said = mcall(&g__NOT_GONE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_list(vec![]);
    }
    thing = mcall(&g__SUBJECT_LEAD(), "sub", &[py_str(""), mcall(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&said.clone(), "start", &[]), &Py::None), "strip", &[py_str(" ,.;:!?")])]);
    thing = mcall(&mcall(&g__POSSESSIVE(), "sub", &[py_str(""), thing.clone()]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&thing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return py_list(vec![]);
    }
    again = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![modulo(&py_str("a %s that wont go away"), &thing.clone()), modulo(&py_str("how do i treat %s"), &thing.clone()), modulo(&py_str("%s still there"), &thing.clone())]));
    for __item in __iter1 {
    said_so = __item.clone();
    shaped = call_builtin("dict", &[understood.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&shaped.clone(), &py_str("clean"), said_so.clone());
    let _ = py_setitem(&shaped.clone(), &py_str("words"), tokens(&[said_so.clone()]));
    let _ = mcall(&again.clone(), "append", &[shaped.clone()]);
    }
    return again.clone();
    Py::None
}
pub fn _what_is_wrong_with_them(args: &[Py]) -> Py {
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
    let mut again: Py = Py::None;
    let mut cared: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shaped: Py = Py::None;
    let _ = py_str("The same reading with a carer's line read as the ask about the thing named, or it back.\n\n    \"my daughter has head lice\", \"our baby's got conjunctivitis\", \"my son caught chickenpox\": the\n    child and the having are the shape of the sentence, not what it is about, and no card is keyed\n    on them. A bare \"head lice\" is not an ask either, so the rest of the line is read as the ask\n    it is - \"how do i deal with head lice\" - and the words of that ask are ones every card ignores.\n    Two openers are tried, because a card that answers a condition is keyed on the condition and the\n    opener it expects is not the same everywhere (\"how do i treat shingles\" finds it, \"how do i deal\n    with shingles\" does not). A blunt \"what do i do for <thing>\" is not among them: it answered a\n    water infection with the dry-eye card.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    cared = _past_the_carer(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]);
    if py_truthy(&Py::Bool(!py_truthy(&cared.clone()))) {
    return py_list(vec![]);
    }
    again = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("how do i deal with "), py_str("how do i treat ")]));
    for __item in __iter1 {
    opener = __item.clone();
    said = add(&opener.clone(), &cared.clone());
    shaped = call_builtin("dict", &[understood.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&shaped.clone(), &py_str("clean"), said.clone());
    let _ = py_setitem(&shaped.clone(), &py_str("words"), tokens(&[said.clone()]));
    let _ = mcall(&again.clone(), "append", &[shaped.clone()]);
    }
    return again.clone();
    Py::None
}
pub fn _without_the_ruled_out(args: &[Py]) -> Py {
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
    let mut gone: Py = Py::None;
    let mut key: Py = Py::None;
    let mut left: Py = Py::None;
    let mut read: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The same reading with the words of the thing the line rules out taken out.\n\n    \"for a holiday not a house\" is read as \"for a holiday\": the house-left-empty card opens on\n    \"house\" and closes on \"holiday\", and the person had just said it was not a house. Only the\n    reading a card is chosen from loses the words; the line itself is untouched, so every other\n    module is still told what they said.\n    ");
    gone = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("ruled_out")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&gone.clone()))) {
    return understood.clone();
    }
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    words = mcall(&understood.clone(), "get", &[py_str("words")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[words.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&words.clone())) } }) {
    words = tokens(&[clean.clone()]);
    }
    wanted = call_builtin("list", &[gone.clone()]).unwrap_or_else(|e| exc::raise(e));
    left = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    key = if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&word.clone(), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = key.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&wanted.clone(), &key.clone())) } }) {
    let _ = mcall(&wanted.clone(), "remove", &[key.clone()]);
    continue;
    }
    let _ = mcall(&left.clone(), "append", &[word.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_is(&mcall(&g__OF_THE_RULE_OUT(), "get", &[mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]), &Py::None))) {  __out.push(word.clone()); } } py_list(__out) })() }))) {
    return understood.clone();
    }
    read = call_builtin("dict", &[understood.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&read.clone(), &py_str("words"), left.clone());
    let _ = py_setitem(&read.clone(), &py_str("clean"), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&left.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }]));
    return read.clone();
    Py::None
}
pub fn _the_degrees_are_a_temperature(args: &[Py]) -> Py {
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
    let mut number: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line says how hot it is in degrees.\n\n    The reading keeps the numbers the line names with the words they were written with, so \"35\n    degrees\" is a measure of heat and \"a 2:1 degree\" is not. The same word, read from what sits in\n    front of it.\n    ");
    numbers = mcall(&understood.clone(), "get", &[py_str("numbers")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[numbers.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&numbers.clone())) } }) {
    return Py::Bool(false);
    }
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&numbers.clone());
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[number.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&number.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    }
    }
    words = mcall(&understood.clone(), "get", &[py_str("norm")]);
    if py_truthy(&call_builtin("isinstance", &[words.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]);
    }
    let __iter2 = py_iter_or_raise(&said.clone());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str("degree")))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("°c"), py_str("°f"), py_str("º"), py_str("°"), py_str("°c")])) { let mut sign: Py = __item.clone(); __out.push(Py::Bool(py_contains(&word.clone(), &sign.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _answers_a_qualification(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut key: Py = Py::None;
    let _ = py_str("Whether this card answers a degree you study for, which a 35-degree day is not.");
    key = _key_words(&[item.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&key.clone(), &g__THE_DEGREE_WORDS())))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("bool", &[bitand(&key.clone(), &sub(&g__THE_HEAT_OF_THE_DAY(), &g__THE_DEGREE_WORDS()))]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _the_person_says_they_are_cold(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line says the person themselves is cold or shivering.\n\n    \"the office is cold and i keep shivering\" is about the office, and the shivering is the person\n    in it: both cards answer something the line asked, so the place does not take it from them.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g__HOW_A_PERSON_IS_COLD(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__COLD_WORDS(), &word.clone())) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&slice_or_raise(&typed.clone(), &call_builtin("max", &[Py::Int(0), sub(&at.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None));
    for __item in __iter3 {
    before = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__SAYS_OF_THEMSELVES(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("feel"), py_str("feeling"), py_str("feels"), py_str("keep"), py_str("keeps"), py_str("kept"), py_str("being"), py_str("got"), py_str("get")]), &before.clone())) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_place_the_line_says_is_cold(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut at: Py = Py::None;
    let mut next_word: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place the line predicates the cold of, or None.\n\n    \"the flat is freezing and the heating is on, why is it so cold\" is about the flat: the cold is\n    said of the place, so a card about a person who feels the cold is not the answer to it. Only a\n    cold said of the place counts, because \"im always cold and my flat is freezing\" says both.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g__PLACES_PEOPLE_LIVE_IN(), &word.clone()))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&slice_or_raise(&typed.clone(), &add(&at.clone(), &Py::Int(1)), &add(&at.clone(), &Py::Int(4)), &Py::None));
    for __item in __iter3 {
    next_word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__COLD_WORDS(), &next_word.clone()))) {
    return word.clone();
    }
    if py_truthy(&Py::Bool(!py_contains(&g__BETWEEN_A_PLACE_AND_ITS_COLD(), &next_word.clone()))) {
    break;
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _answers_the_cold_of_a_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut place: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { Py::None };
    let mut key: Py = Py::None;
    let _ = py_str("Whether this card answers how cold a person feels rather than how cold a place is.");
    if py_truthy(&Py::Bool(!py_contains(&g_FEELING_DOMAINS(), &gattr(&item.clone(), "domain")))) {
    return Py::Bool(false);
    }
    key = _key_words(&[item.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&key.clone(), &g__COLD_WORDS())))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_is(&place.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&key.clone(), &place.clone())) } };
    Py::None
}
pub fn _answers_the_cold_of_a_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut place: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut key: Py = Py::None;
    let _ = py_str("Whether this card is about the place itself being cold, which is what the line asks.");
    if py_truthy(&_answers_the_cold_of_a_person(&[item.clone(), place.clone()])) {
    return Py::Bool(false);
    }
    key = _key_words(&[item.clone()]);
    return { let __b = Py::Bool(py_contains(&key.clone(), &place.clone())); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[bitand(&key.clone(), &g__COLD_WORDS())]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_card_for_the_place_first(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut place: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut reached: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reached") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reached"))) };
    let mut at: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("The place's own card before a card about a person, when the line says the place is cold.\n\n    \"the flat is freezing and the heating is on, why is it so cold\" reached the card about lifting a\n    low mood - \"flat\" and \"why\" are its words - and the flat's own answer, warm the room you are\n    sitting in, ranks under it. A card for the place is only put first when one reached the line;\n    nothing is dropped, so a line that says only how cold the person is keeps its card.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&reached.clone()))) {
    return reached.clone();
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[reached.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    row = __unpk2[1].clone();
    if py_truthy(&{ let __b = at.clone(); if !py_truthy(&__b) { __b } else { _answers_the_cold_of_a_place(&[geti(&row.clone(), &Py::Int(5)), place.clone()]) } }) {
    return add(&add(&py_list(vec![row.clone()]), &slice_or_raise(&reached.clone(), &Py::None, &at.clone(), &Py::None)), &slice_or_raise(&reached.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None));
    }
    }
    return reached.clone();
    Py::None
}
pub fn _the_ask_reading(args: &[Py]) -> Py {
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
    let mut ask: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut asked: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut number: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The reading cut down to the ask clause: what the setting said does not come with it.\n\n    The ask is read from a copy of the line's reading, and a copy carries what the clause before\n    it said. \"my mum is 80 next week and we want a cake, what should i bake\" kept the 80 in it,\n    and after the Italy wedding line a ferry question kept the 35 degrees: a money card, a date\n    card and a degree card can each win on a figure the ask never had. So the copy holds the\n    ask's own words and only the numbers, the things and the subject written inside them.\n    ");
    asked = call_builtin("dict", &[understood.clone()]).unwrap_or_else(|e| exc::raise(e));
    said = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&asked.clone(), &py_str("clean"), mcall(&py_str(" "), "join", &[ask.clone()]));
    let _ = py_setitem(&asked.clone(), &py_str("words"), call_builtin("list", &[ask.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setitem(&asked.clone(), &py_str("norm"), call_builtin("list", &[ask.clone()]).unwrap_or_else(|e| exc::raise(e)));
    numbers = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("numbers")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[number.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    written = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&number.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]);
    if py_truthy(&{ let __b = bitand(&call_builtin("set", &[written.clone()]).unwrap_or_else(|e| exc::raise(e)), &said.clone()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &call_builtin("str", &[mcall(&number.clone(), "get", &[py_str("value")])]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    let _ = mcall(&numbers.clone(), "append", &[number.clone()]);
    }
    }
    let _ = py_setitem(&asked.clone(), &py_str("numbers"), numbers.clone());
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("entities"), py_str("things")]));
    for __item in __iter2 {
    key = __item.clone();
    kept = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter3 {
    thing = __item.clone();
    text = if py_truthy(&call_builtin("isinstance", &[thing.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&thing.clone(), "get", &[py_str("text")]) } else { thing.clone() };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &said.clone()) } }) {
    let _ = mcall(&kept.clone(), "append", &[thing.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&understood.clone(), &key.clone()))) {
    let _ = py_setitem(&asked.clone(), &key.clone(), kept.clone());
    }
    }
    subject = mcall(&understood.clone(), "get", &[py_str("subject")]);
    let _ = py_setitem(&asked.clone(), &py_str("subject"), if py_truthy(&{ let __b = call_builtin("isinstance", &[subject.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &mcall(&subject.clone(), "lower", &[]))) } }) { subject.clone() } else { Py::None });
    return asked.clone();
    Py::None
}
pub fn _the_ask_on_its_own(args: &[Py]) -> Py {
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
    let mut ask: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut focus: Py = Py::None;
    let mut line: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The ask clause's words when the line says the setting first and asks second, or None.\n\n    \"wedding in italy and 35 degrees, what do i wear\", \"my mum is 80 next week and we want a cake,\n    what should i bake\", \"playing five a side sunday, what should i eat before\": the words before\n    the ask say where and when, and the ask is what the person wants answered. The understanding\n    engine has already cut the line at its clauses and hands the ask over in `focus`, so this is\n    the ask alone - and None when the line is one clause, or when its ask is the whole line.\n    ");
    focus = mcall(&understood.clone(), "get", &[py_str("focus")]);
    clauses = mcall(&understood.clone(), "get", &[py_str("clauses")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[focus.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clauses.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return Py::None;
    }
    ask = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&focus.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "strip", &[]) } }) {  __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&lt(&call_builtin("len", &[ask.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    whole = { let __b = mcall(&understood.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("words")]) } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[whole.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    line = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&whole.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() };
    tail = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[line.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&line.clone(), &neg(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None), &tail.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g__THE_WH_WORDS(), &geti(&tail.clone(), &Py::Int(0))))) {
    return Py::None;
    }
    return ask.clone();
    Py::None
}
pub fn _the_best_entry(args: &[Py]) -> Py {
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
    let mut entries: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { Py::None };
    let mut reject: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { Py::None };
    let mut use_context: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "use_context") { v.clone() }
    else { Py::Bool(false) };
    let mut _u: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut asked_for: Py = Py::None;
    let mut background: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut better: Py = Py::None;
    let mut by_domain: Py = Py::None;
    let mut ch: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut described: Py = Py::None;
    let mut drop: Py = Py::None;
    let mut every: Py = Py::None;
    let mut filler: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut found: Py = Py::None;
    let mut going: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut here: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut index: Py = Py::None;
    let mut intensifier: Py = Py::None;
    let mut item: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kept_words: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut known_words: Py = Py::None;
    let mut left: Py = Py::None;
    let mut occasion: Py = Py::None;
    let mut original: Py = Py::None;
    let mut past: Py = Py::None;
    let mut pointed: Py = Py::None;
    let mut seen_in: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut w: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let mut wrapped: Py = Py::None;
    let _ = py_str("The best Entry for this reading of a line, or None when nothing fits.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    understood = _without_the_ruled_out(&[understood.clone()]);
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    words = mcall(&understood.clone(), "get", &[py_str("words")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[words.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__CURLY_ANY()) { let mut ch: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &ch.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    words = tokens(&[clean.clone()]);
    }
    past = _past_a_handoff(&[clean.clone()]);
    if py_truthy(&past.clone()) {
    words = tokens(&[past.clone()]);
    }
    left = mcall(&understood.clone(), "get", &[py_str("intensifier_left")]);
    intensifier = if py_truthy(&Py::Bool(py_is(&left.clone(), &Py::None))) { mcall(&understood.clone(), "get", &[py_str("intensifier")]) } else { left.clone() };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[intensifier.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { intensifier.clone() } }) {
    going = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&intensifier.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&going.clone(), &mcall(&word.clone(), "lower", &[]), add(&mcall(&going.clone(), "get", &[mcall(&word.clone(), "lower", &[]), Py::Int(0)]), &Py::Int(1)));
    }
    }
    kept = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    key = if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&word.clone(), "lower", &[]) } else { py_str("") };
    stem = _without_possessive(&[key.clone()]);
    hit = if py_truthy(&mcall(&going.clone(), "get", &[key.clone()])) { key.clone() } else { if py_truthy(&mcall(&going.clone(), "get", &[stem.clone()])) { stem.clone() } else { py_str("") } };
    if py_truthy(&hit.clone()) {
    let __aug3 = sub(&geti(&going.clone(), &hit.clone()), &Py::Int(1));
    let _ = py_setitem(&going.clone(), &hit.clone(), __aug3);
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    words = slice_or_raise(&kept.clone(), &Py::None, &Py::None, &neg(&Py::Int(1)));
    }
    background = mcall(&understood.clone(), "get", &[py_str("background")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[background.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = background.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("topic_own")]))) } } } }) {
    drop = py_dict(vec![]);
    let __iter4 = py_iter_or_raise(&background.clone());
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&drop.clone(), &mcall(&word.clone(), "lower", &[]), add(&mcall(&drop.clone(), "get", &[mcall(&word.clone(), "lower", &[]), Py::Int(0)]), &Py::Int(1)));
    }
    }
    kept = py_list(vec![]);
    let __iter5 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    word = __item.clone();
    key = if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&word.clone(), "lower", &[]) } else { py_str("") };
    if py_truthy(&mcall(&drop.clone(), "get", &[key.clone()])) {
    let __aug6 = sub(&geti(&drop.clone(), &key.clone()), &Py::Int(1));
    let _ = py_setitem(&drop.clone(), &key.clone(), __aug6);
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[word.clone()]);
    }
    words = slice_or_raise(&kept.clone(), &Py::None, &Py::None, &neg(&Py::Int(1)));
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("empty")))) {
    return Py::None;
    }
    occasion = _feeling_occasion(&[words.clone()]);
    if py_truthy(&{ let __b = occasion.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("asks")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("topic")]), &py_str("pets"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_names_a_fault(&[occasion.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_speaker_is_feeling(&[words.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_feared_thing(&[occasion.clone()]))) } } } } } } } } } }) {
    if py_truthy(&Py::Bool(py_is(&entries.clone(), &Py::None))) {
    let mut all_entries: Py = py_func("entries", |args: &[Py]| crate::gen::knowledge::entries(args));
    entries = py_call(&all_entries, &[]);
    }
    let __unpk7 = py_unpack(&_vocabulary(&[entries.clone()]), 3, None);
    seen_in = __unpk7[0].clone();
    known_words = __unpk7[1].clone();
    _u = __unpk7[2].clone();
    here = _best(&[understood.clone(), words.clone(), clean.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }, { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() }, entries.clone(), seen_in.clone(), known_words.clone(), reject.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&here.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FEELING_DOMAINS(), &gattr(&here.clone(), "domain"))) } }) {
    gone = py_dict(vec![]);
    let __iter8 = py_iter_or_raise(&occasion.clone());
    for __item in __iter8 {
    word = __item.clone();
    let _ = py_setitem(&gone.clone(), &word.clone(), add(&mcall(&gone.clone(), "get", &[word.clone(), Py::Int(0)]), &Py::Int(1)));
    }
    kept_words = py_list(vec![]);
    let __iter9 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter9 {
    word = __item.clone();
    key = if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&word.clone(), "lower", &[]) } else { py_str("") };
    if py_truthy(&mcall(&gone.clone(), "get", &[key.clone()])) {
    let __aug10 = sub(&geti(&gone.clone(), &key.clone()), &Py::Int(1));
    let _ = py_setitem(&gone.clone(), &key.clone(), __aug10);
    continue;
    }
    let _ = mcall(&kept_words.clone(), "append", &[word.clone()]);
    }
    words = slice_or_raise(&kept_words.clone(), &Py::None, &Py::None, &neg(&Py::Int(1)));
    }
    }
    original = words.clone();
    stripped = Py::Bool(false);
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }, &py_str("dead")))) {
    words = _without_a_dead_that_only_means_very(&[words.clone()]);
    stripped = Py::Bool(!py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[original.clone()]).unwrap_or_else(|e| exc::raise(e))));
    }
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }, &py_str("cooking")))) {
    words = _without_a_cooking_that_is_a_name(&[words.clone()]);
    stripped = { let __b = stripped.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[original.clone()]).unwrap_or_else(|e| exc::raise(e)))) } };
    }
    words = _with_the_child_by_place(&[words.clone(), clean.clone()]);
    if py_truthy(&a_colour_is_worn(&[understood.clone()])) {
    words = add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk4891 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk4891[0].clone(); let mut word: Py = __unpk4891[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_COLOUR_ASKED_WORDS(), &mcall(&word.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_dress_code(&[words.clone(), index.clone()]))) } } } }))) {  __out.push(word.clone()); } } py_list(__out) })() }, &py_list(vec![py_str("wear")]));
    }
    forms = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(_forms(&[w.clone()])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_is(&entries.clone(), &Py::None))) {
    let mut all_entries: Py = py_func("entries", |args: &[Py]| crate::gen::knowledge::entries(args));
    entries = py_call(&all_entries, &[]);
    }
    let __unpk11 = py_unpack(&_vocabulary(&[entries.clone()]), 3, None);
    by_domain = __unpk11[0].clone();
    every = __unpk11[1].clone();
    known = __unpk11[2].clone();
    typed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() };
    if py_truthy(&about_the_chat(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { clean.clone() } }])) {
    return Py::None;
    }
    let _ = _read_typos(&[typed.clone(), forms.clone(), every.clone(), known.clone()]);
    let _ = _read_a_thing_had(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }, forms.clone()]);
    wrapped = _wrapped_subject(&[clean.clone(), mcall(&understood.clone(), "get", &[py_str("norm_text")])]);
    whole = _best(&[understood.clone(), words.clone(), clean.clone(), forms.clone(), typed.clone(), entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    whole = _the_answer_the_colour_dragged_in(&[whole.clone(), understood.clone(), words.clone(), clean.clone(), entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    if py_truthy(&a_colour_is_worn(&[understood.clone()])) {
    whole = _the_card_for_the_occasion_asked(&[whole.clone(), understood.clone(), words.clone(), clean.clone(), forms.clone(), typed.clone(), entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    }
    item = Py::None;
    if py_truthy(&wrapped.clone()) {
    item = _the_ask_it_wraps(&[understood.clone(), wrapped.clone(), whole.clone(), entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&item.clone(), &Py::None))) {
    item = whole.clone();
    }
    filler = mcall(&g__FILLER(), "intersection", &[typed.clone()]);
    kept = if py_truthy(&filler.clone()) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__FILLER(), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])))) {  __out.push(word.clone()); } } py_list(__out) })() } } else { py_list(vec![]) };
    bare = Py::None;
    if py_truthy(&filler.clone()) {
    bare = call_builtin("dict", &[understood.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&bare.clone(), &py_str("clean"), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]));
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&item.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&bare.clone(), &Py::None)) } }) {
    item = _best(&[bare.clone(), kept.clone(), geti(&bare.clone(), &py_str("clean")), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }, { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() }, entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    }
    ask = _the_ask_on_its_own(&[understood.clone()]);
    if py_truthy(&ask.clone()) {
    asked_for = sub(&call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g__ASK_FRAME());
    if py_truthy(&{ let __b = Py::Bool(py_is(&item.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&asked_for.clone(), &_key_words(&[item.clone()])))) } }) {
    asked = _the_ask_reading(&[understood.clone(), ask.clone()]);
    found = _best(&[asked.clone(), ask.clone(), geti(&asked.clone(), &py_str("clean")), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }, { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_list(__out) })() }, entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    item = found.clone();
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&item.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { stripped.clone() } }) {
    item = _best(&[understood.clone(), original.clone(), clean.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&original.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }, { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&original.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() }, entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&item.clone(), &Py::None))) {
    described = call_builtin("list", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&_read_the_adjective(&[typed.clone(), described.clone(), every.clone()])) {
    item = _best(&[understood.clone(), words.clone(), clean.clone(), described.clone(), typed.clone(), entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&item.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = use_context.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("leans_on")]); if !py_truthy(&__b) { __b } else { leans_on_a_subject(&[understood.clone()]) } } } } } }) {
    item = _following_on(&[understood.clone(), entries.clone(), forms.clone(), typed.clone(), by_domain.clone(), every.clone(), known.clone(), reject.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&item.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&bare.clone(), &Py::None)) } }) {
    item = _following_on(&[understood.clone(), entries.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }, { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() }, by_domain.clone(), every.clone(), known.clone(), reject.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&item.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = wrapped.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&about(&[item.clone(), wrapped.clone()]))) } } } }) {
    item = Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&item.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _the_animal_rules_out_that_card(&[item.clone(), understood.clone()]) } }) {
    pointed = _with_the_animal_in_hand(&[words.clone(), understood.clone()]);
    if py_truthy(&Py::Bool(!py_is(&pointed.clone(), &words.clone()))) {
    better = _best(&[understood.clone(), pointed.clone(), clean.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pointed.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }, { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pointed.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() }, entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    if py_truthy(&Py::Bool(!py_is(&better.clone(), &Py::None))) {
    item = better.clone();
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&item.clone(), &Py::None))) {
    item = _defines(&[understood.clone(), forms.clone(), typed.clone(), entries.clone(), reject.clone()]);
    }
    if py_truthy(&Py::Bool(py_is(&item.clone(), &Py::None))) {
    item = _the_name_alone(&[_the_name_card(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }, entries.clone(), reject.clone(), _topic_domains(&[understood.clone(), entries.clone()])]), entries.clone(), reject.clone(), _topic_domains(&[understood.clone(), entries.clone()]), understood.clone(), words.clone(), use_context.clone()]);
    }
    return item.clone();
    Py::None
}
pub fn _the_thing_asked_about(args: &[Py]) -> Py {
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
    let mut opening: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing a definition line asks about, as its words, or () when it is not one.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&clean.clone(), "strip", &[]))) } }) {
    return py_tuple(vec![]);
    }
    found = mcall(&g__DEFINITION_ASK(), "match", &[mcall(&mcall(&clean.clone(), "strip", &[]), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_tuple(vec![]);
    }
    thing = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&found.clone(), "groups", &[])) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    opening = mcall(&geti(&{ let __b = mcall(&thing.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0)), "strip", &[py_str("'\"-")]);
    if py_truthy(&Py::Bool(py_contains(&g__DEFINITION_POINTERS(), &opening.clone()))) {
    return py_tuple(vec![]);
    }
    raw = mcall(&mcall(&thing.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words_of(&[raw.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__DEFINITION_TROUBLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("going on"), py_str("up with"), py_str("up to")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&modulo(&py_str(" %s "), &raw.clone()), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_tuple(vec![]);
    }
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&mcall(&thing.clone(), "replace", &[py_str("’"), py_str("'")]), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str("'\"-")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__DEFINITION_EMPTY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__DEFINITION_TAIL(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))) } }) {
    continue;
    }
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__DEFINITION_TAIL(), &geti(&words.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&words.clone(), "pop", &[]);
    }
    return call_builtin("tuple", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(3), &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _words_of(args: &[Py]) -> Py {
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
    let _ = py_str("The words of a phrase, lowercased and stripped of punctuation.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&text.clone(), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&mcall(&word.clone(), "strip", &[py_str("'\"-?!.,;:")])) {  __out.push(mcall(&word.clone(), "strip", &[py_str("'\"-?!.,;:")])); } } py_list(__out) })() };
    Py::None
}
pub fn _asks_what_it_is_for(args: &[Py]) -> Py {
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
    let _ = py_str("True when the definition line asks what the thing is for, not what it is.\n\n    \"what is a semicolon for\" is answered by what the thing does (\"Use a semicolon to join two\n    full sentences\"), and \"what is a warranty\" by what the thing is (\"A warranty is a bonus on\n    top of your rights\"). The two shapes are not interchangeable: a use is not a definition, and\n    a definition is not an answer to what something is for.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bfor\\b\\s*\\??\\s*$|\\bused for\\b|\\bdo(?:es)? (?:it|that|this) do\\b"), mcall(&mcall(&clean.clone(), "strip", &[]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_number(args: &[Py]) -> Py {
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
    let mut char: Py = Py::None;
    let _ = py_str("True for a word with a digit in it (\"4pm\", \"2nd\", \"500ml\").");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&word.clone()) { let mut char: Py = __item.clone(); __out.push(mcall(&char.clone(), "isdigit", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thing_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut run: Py = Py::None;
    let mut start: Py = Py::None;
    let mut want: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Where the whole thing is said as one phrase in the sentence, or None.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tokens(&[sentence.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() };
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&thing.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[add(&sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    start = __item.clone();
    run = slice_or_raise(&words.clone(), &start.clone(), &add(&start.clone(), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[run.clone(), wanted.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk392 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk392[0].clone(); let mut want: Py = __unpk392[1].clone(); __out.push(bitand(&_forms(&[word.clone()]), &want.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![start.clone(), words.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_reply_defines(args: &[Py]) -> Py {
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
    let mut purpose: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "purpose") { v.clone() }
    else { Py::Bool(false) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the reply's opening sentence defines the thing, so a definition ask is answered.\n\n    Every word of the thing has to be said as one phrase: the row that answers an abnormal smear\n    result says \"An abnormal smear means some cells looked a little different\", which names half\n    of \"smear test\" and answers a different question, and the shower row says \"the wet air stays\n    in the room\" about a wet room.\n    Then the phrase has to be doing the defining - what it is (\"A warranty is a bonus on top of\n    your rights\") or, when the person asked what it is for, what it does (\"Use a semicolon to\n    join two full sentences\"). A row that merely mentions the thing while answering something\n    else stays out: \"Who owns a boundary fence is in the deeds\" is about a fence.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    opening = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s")); __args.push(mcall(&reply.clone(), "strip", &[])); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    found = _the_thing_in(&[opening.clone(), thing.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&gt(&geti(&found.clone(), &Py::Int(0)), &g__SUBJECT_AT_MOST())) {
    return Py::Bool(false);
    }
    if py_truthy(&purpose.clone()) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&found.clone(), 2, None);
    start = __unpk1[0].clone();
    words = __unpk1[1].clone();
    if py_truthy(&{ let __b = start.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__DEFINITION_ARTICLES(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    before = slice_or_raise(&words.clone(), &Py::None, &start.clone(), &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__BE_VERBS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__BEFORE_THE_THING(), &geti(&before.clone(), &neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { _a_number(&[geti(&before.clone(), &neg(&Py::Int(1)))]) } } } }) {
    return Py::Bool(false);
    }
    after = slice_or_raise(&words.clone(), &add(&start.clone(), &call_builtin("len", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e))), &add(&add(&start.clone(), &call_builtin("len", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1)), &Py::None);
    return { let __b = call_builtin("bool", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__DEFINING_VERBS(), &geti(&after.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _the_key_names_the_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut alternative: Py = Py::None;
    let mut part: Py = Py::None;
    let mut want: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when one phrase of the entry's key is the thing itself, word for word.\n\n    \"what is a food bank\" is the row keyed on \"food bank\", and the words are read the way every\n    other match is read, so \"will\" reaches \"wills\" and \"boundary\" reaches \"boundaries\". A key\n    that only holds the words inside a longer phrase is about that longer phrase - \"boundary\n    fence\" is not a definition of a boundary, \"sports wash\" is not a sports bra - so the phrase\n    has to be as long as the thing.\n    ");
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&thing.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    words = mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[]);
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[words.clone(), wanted.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk1337 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk1337[0].clone(); let mut want: Py = __unpk1337[1].clone(); __out.push(bitand(&_forms(&[word.clone()]), &want.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _said_as_a_report(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut entries: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut reject: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { Py::None };
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The entry the same name gets inside a sentence, when the line is only a name, or None.\n\n    A card is written for a thing being raised (\"i have a nappy rash\", \"ive got an ear\n    infection\"), so the bare words of the name reach several cards at once and the nearest one\n    is not the one the knowledge base would answer the same person with. Read the name as the\n    report it is and let the ordinary search pick: the bare \"head ache\" opened the migraine\n    card, while \"i have a head ache\" gets the ordinary-headache card, which every longer line\n    about a headache already gets.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = add(&py_str("i have a "), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]));
    second = call_builtin("dict", &[understood.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&second.clone(), &py_str("clean"), said.clone());
    let _ = py_setitem(&second.clone(), &py_str("words"), add(&py_list(vec![py_str("i"), py_str("have"), py_str("a")]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("background"), py_str("intensifier"), py_str("intensifier_left"), py_str("leans_on"), py_str("topic_own"), py_str("past"), py_str("handoff")]));
    for __item in __iter1 {
    key = __item.clone();
    let _ = mcall(&second.clone(), "pop", &[key.clone(), Py::None]);
    }
    return _the_best_entry(&[second.clone(), entries.clone(), reject.clone(), Py::Bool(false)]);
    Py::None
}
pub fn _the_name_card(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut entries: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut reject: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { Py::None };
    let mut allowed: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { Py::None };
    let mut _names_itself: Py = Py::None;
    let mut alternatives: Py = Py::None;
    let mut found: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The entry that is *named* the line, or None.\n\n    The entry's own name is the first alternative of the first part of its key, and only that\n    counts: a card lists many ways of saying its subject, and most of them are meant to be\n    reached with a second signal, not on their own. \"ear infection\" and \"head ache\" are\n    alternatives of the swimmer's-ear and migraine cards, and those cards are the right answer\n    to a sentence that says the rest; opened on the name alone they answered a question nobody\n    asked. The card called \"nappy rash\" is the card for nappy rash.\n\n    A one-word name is not enough either: \"money\", \"sofa\" and \"boiler\" are the objects of a\n    conversation as often as they are a subject to open, and a person answering the bot own\n    question meant the pizza, not the pizza card.\n    ");
    said = call_builtin("set", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e));
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&entries.clone());
    for __item in __iter1 {
    item = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_NAME_ALONE_DOMAINS(), &call_builtin("getattr", &[item.clone(), py_str("domain"), Py::None]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&reject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { py_call(&reject, &[item.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&allowed.clone(), &call_builtin("getattr", &[item.clone(), py_str("domain"), Py::None]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    key = { let __b = call_builtin("getattr", &[item.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    continue;
    }
    part = geti(&key.clone(), &Py::Int(0));
    alternatives = if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&alternatives.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&alternatives.clone(), &Py::Int(0)), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    phrase = call_builtin("set", &[mcall(&mcall(&geti(&alternatives.clone(), &Py::Int(0)), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&phrase.clone(), &said.clone())) } }) {
    let _ = mcall(&found.clone(), "append", &[item.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&found.clone(), &Py::Int(0));
    }
pub fn __nested__names_itself_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _names_itself: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut allowed: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut alternatives: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut entries: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut found: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut key: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut part: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut phrase: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut reject: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut typed: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(12).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "item").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut name: Py = Py::None;
    let mut reply: Py = Py::None;
    name = if py_truthy(&gattr(&item.clone(), "key")) { geti(&geti(&gattr(&item.clone(), "key"), &Py::Int(0)), &Py::Int(0)) } else { py_str("") };
    reply = slice_or_raise(&mcall(&{ let __b = call_builtin("getattr", &[item.clone(), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), &Py::None, &Py::Int(140), &Py::None);
    return if py_truthy(&{ let __b = call_builtin("isinstance", &[name.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&reply.clone(), &mcall(&name.clone(), "lower", &[]))) } }) { Py::Int(0) } else { Py::Int(1) };
    Py::None
}
    _names_itself = py_func("_names_itself", move |args: &[Py]| { let mut full = vec![_names_itself.clone(), allowed.clone(), alternatives.clone(), entries.clone(), found.clone(), item.clone(), key.clone(), part.clone(), phrase.clone(), reject.clone(), said.clone(), typed.clone()]; full.extend_from_slice(args); __nested__names_itself_1(&full) });
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); __kws.extend(vec![("key".to_string(), _names_itself.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_name_alone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut named: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut entries: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut reject: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reject"))) };
    let mut allowed: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { Py::None };
    let mut understood: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut words: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { Py::None };
    let mut use_context: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "use_context") { v.clone() }
    else { Py::Bool(false) };
    let mut said: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The card a line that is only the name of a thing gets, or None.\n\n    `named` is that name's own card - the one whose own name the line spells out. The checks\n    here are the ones a name has to pass before it is a name and not a sentence, and they are\n    the reading's own: a state in the phrase (\"gone numb\"), a clause ending (\"school closed\"),\n    a name that is one word, and a card from an area the line is not read as being about.\n    ");
    typed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(2), &{ let __cmp10 = call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(4)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_NAME_ALONE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_STATE_IN_THE_PHRASE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_A_STATE_AS_A_NAME_END(), &geti(&typed.clone(), &neg(&Py::Int(1)))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&named.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&allowed.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_is(&understood.clone(), &Py::None))) {
    allowed = _declared_for(&[allowed.clone(), understood.clone()]);
    }
    if py_truthy(&Py::Bool(!py_contains(&allowed.clone(), &call_builtin("getattr", &[named.clone(), py_str("domain"), Py::None]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = call_builtin("set", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return named.clone();
    Py::None
}
pub fn _declared_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut allowed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: allowed"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut found: Py = Py::None;
    let mut judged: Py = Py::None;
    let mut name: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut topics: Py = Py::None;
    let _ = py_str("The domains in `allowed` that declare one of the topics the line is about.\n\n    `_topic_domains` keeps a domain that declares nothing, because for a search a domain with no\n    opinion should not be ruled out. A card answering a bare name is a stronger claim than that,\n    so the domain has to say the topic is its own.\n    ");
    let mut by_topic: Py = py_func("by_topic", |args: &[Py]| crate::gen::knowledge::by_topic(args));
    judged = py_call(&by_topic, &[]);
    topics = py_list(vec![]);
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    found = mcall(&understood.clone(), "get", &[py_str("topics")]);
    topics = if py_truthy(&call_builtin("isinstance", &[found.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("list", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { py_list(vec![]) };
    let _ = mcall(&topics.clone(), "append", &[mcall(&understood.clone(), "get", &[py_str("topic")])]);
    }
    topics = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&topics.clone()) { let mut topic: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[topic.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { topic.clone() } }) {  __out.push(topic.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&topics.clone()))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&allowed.clone()) { let mut name: Py = __item.clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&topics.clone()) { let mut topic: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let __b = mcall(&judged.clone(), "get", &[name.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &topic.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(name.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _defines(args: &[Py]) -> Py {
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
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut typed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut entries: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut reject: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { Py::None };
    let mut best: Py = Py::None;
    let mut best_rank: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut item: Py = Py::None;
    let mut purpose: Py = Py::None;
    let mut rank: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The best entry that defines the thing a definition line asks about, or None.\n\n    Read only when every other reader gave up, so a line with an answer of its own is untouched.\n    The entry has to name the thing in its key and in the first sentence of its reply, and its\n    fences still hold: a \"what is a wet room\" line that also says the room smells is a damp\n    question and keeps the damp row.\n    ");
    clean = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("clean")]) } else { py_str("") };
    said = { let __b = clean.clone(); if py_truthy(&__b) { __b } else { { let __b = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("text")]) } else { py_str("") }; if py_truthy(&__b) { __b } else { py_str("") } } } };
    thing = _the_thing_asked_about(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    purpose = _asks_what_it_is_for(&[said.clone()]);
    best = Py::None;
    best_rank = Py::None;
    let __iter1 = py_iter_or_raise(&entries.clone());
    for __item in __iter1 {
    item = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&reject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { py_call(&reject, &[item.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&guard_passes(&[item.clone(), understood.clone()])))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_key_names_the_thing(&[item.clone(), thing.clone()])))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_reply_defines(&[call_builtin("getattr", &[item.clone(), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e)), thing.clone(), purpose.clone()])))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&item.clone(), "avoid")) { let mut word: Py = __item.clone(); __out.push(_part_score(&[word.clone(), forms.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    rank = neg(&call_builtin("len", &[_key_forms(&[item.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&{ let __b = Py::Bool(py_is(&best_rank.clone(), &Py::None)); if py_truthy(&__b) { __b } else { gt(&rank.clone(), &best_rank.clone()) } }) {
    let __unpk2 = py_unpack(&py_tuple(vec![item.clone(), rank.clone()]), 2, None);
    best = __unpk2[0].clone();
    best_rank = __unpk2[1].clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn _named_things(args: &[Py]) -> Py {
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
    let mut entities: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The names the line gives, which a question is about before it is about its ask words.");
    found = py_list(vec![]);
    entities = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("entities")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[entities.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { entities.clone() } else { py_tuple(vec![]) });
    for __item in __iter1 {
    entity = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entity.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("place"), py_str("name"), py_str("title")]), &mcall(&entity.clone(), "get", &[py_str("type")]))) } }) {
    continue;
    }
    name = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entity.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    let _ = mcall(&found.clone(), "append", &[name.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _plans_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks to plan something and the entry's key is the planning word.\n\n    \"help me plan a trip to Paris\" and \"i want to plan a weekend in Amsterdam\" name a place,\n    but the question is the plan, not the place: the city guide names Paris or Amsterdam and\n    would take the line on the named-thing tie-break. A key that itself says \"plan\" is the\n    answer to a line that says plan, so it comes before the name.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PLAN_WORDS()) { let mut word: Py = __item.clone(); __out.push(_part_score(&[word.clone(), forms.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PLAN_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_key_words(&[item.clone()]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_one(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut named: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut alternative: Py = Py::None;
    let mut name: Py = Py::None;
    let mut part: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the entry's own key names one of the things the line names.");
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[item.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[alternative.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter3 = py_iter_or_raise(&named.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&mcall(&_bounded(&[call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))]), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _only_asking_about_it(args: &[Py]) -> Py {
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
    let mut leans: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "leans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: leans"))) };
    let mut earlier: Py = Py::None;
    let mut now: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names nothing the line before it had not already named.\n\n    Only the content words count: what the line adds of its own is the subject, and a line that\n    adds none of its own is asking about the line before it. \"what about a large one\" adds\n    \"large\", so it is not this - the earlier card answers it only when the two lines complete a\n    key between them, as they do for a chicken.\n    ");
    earlier = call_builtin("set", &[tokens(&[call_builtin("str", &[{ let __b = mcall(&{ let __b = leans.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = leans.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    let _ = mcall(&earlier.clone(), "update", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = leans.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() }]);
    now = tokens(&[call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&now.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&now.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&earlier.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_GRAMMAR_ONLY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FOLLOW_ON_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "isdigit", &[]) } }) {
    continue;
    }
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn names_nothing_of_its_own(args: &[Py]) -> Py {
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
    let mut leans: Py = Py::None;
    let _ = py_str("True when the line asks about the subject of the line before it and adds no subject.\n\n    \"will it survive\", \"is that bad\" and \"what happens now\" carry an ask and nothing else, so\n    the only subject they can be about is the one the last answer was about. A line that adds a\n    content word of its own (\"what about a large one\") is not this: the word it adds is what it\n    is asking about.\n    ");
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return _only_asking_about_it(&[understood.clone(), leans.clone()]);
    Py::None
}
pub fn leans_on_a_subject(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line leans on the subject before it, rather than saying something itself.\n\n    A question left out its subject on purpose (\"how long in the oven?\"), and a line that\n    points back at the thing (\"is that bad\", \"its 3 years old\") is about it. A statement that\n    names its own subject is not: \"i have a cat\" then \"i work nights\" was answered with the\n    cats-at-night entry, because the carried subject was searched together with the line's own\n    words - \"cat\" plus \"nights\" reached an entry neither of them was about, and the person had\n    only said what their job was. Nothing else about the night shift was ever said, so the reply\n    answered a subject they had not raised.\n    ");
    if py_truthy(&Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("kind")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("question")))) {
    return Py::Bool(true);
    }
    if py_truthy(&_says_how_the_speaker_handles_it(&[understood.clone()])) {
    return Py::Bool(true);
    }
    line = tokens(&[call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e))]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&line.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_names_in_the_ask(args: &[Py]) -> Py {
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
    let mut every: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut focus: Py = Py::None;
    let mut low: Py = Py::None;
    let mut names: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut split: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words of the question half that the knowledge base treats as a name for something.\n\n    People put the situation first and the question last: \"i start a new job monday and the\n    commute is an hour, should i get a railcard\". The situation half is thick with subject words\n    and the question half is thin, so the card for the commute (a house move, a season ticket)\n    outscored the card for the railcard that was actually being decided. Nothing in the scan\n    knew which half a matched word came from.\n\n    `focus` is the question half, which the reading already hands over. Only the content words\n    of it count, and only the ones a card keys on as a name: the ask's own words (\"what\",\n    \"should\", \"get\") are the vocabulary of asking and every card in the base keys on some of\n    them, so counting those would put the whole base on the ask.\n\n    Only a line split into a situation and a question of its own is read this way (see\n    `_two_clauses`); a line that states one thing and asks nothing keeps the reading it had.\n    ");
    split = _two_clauses(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&split.clone()))) {
    return py_tuple(vec![]);
    }
    focus = split.clone();
    names = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&focus.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    low = mcall(&word.clone(), "lower", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__NOT_A_SUBJECT(), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_FRAME_WORDS(), &low.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = gt(&mcall(&every.clone(), "get", &[low.clone(), Py::Int(0)]), &g_ASK_NAME_CAP()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&names.clone(), &low.clone())) } }) {
    continue;
    }
    let _ = mcall(&names.clone(), "append", &[low.clone()]);
    }
    plain = Py::Bool(!py_contains(&{ let __b = mcall(&understood.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &py_str("yesno")));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names.clone())); if !py_truthy(&__b) { __b } else { plain.clone() } }) {
    let __iter2 = py_iter_or_raise(&focus.clone());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    low = mcall(&word.clone(), "lower", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__NOT_A_SUBJECT(), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_FRAME_WORDS(), &low.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = gt(&mcall(&every.clone(), "get", &[low.clone(), Py::Int(0)]), &g_ASK_PLAIN_CAP()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&names.clone(), &low.clone())) } }) {
    continue;
    }
    let _ = mcall(&names.clone(), "append", &[low.clone()]);
    }
    }
    return call_builtin("tuple", &[names.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_ask_in_order(args: &[Py]) -> Py {
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
    let _ = py_str("The words of the question half, in the order the person said them, or ().\n\n    The names `_the_names_in_the_ask` hands back are single words, and a card that keys a phrase\n    of two words is not reached by one of them alone: \"how do i get more vitamin c\" offered\n    \"vitamin\" and the expired-medicines row, which keys on \"vitamins\", took the line off the row\n    that keys on \"vitamin c\". Read as the words in order, the phrase a card keys on can be found\n    for what it is (see `_the_longest_run`).\n    ");
    said = _two_clauses(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_tuple(vec![]);
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _two_clauses(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut focus: Py = Py::None;
    let mut form: Py = Py::None;
    let mut last: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(the words of the question half, the loose forms said before it), or None.\n\n    Only a line that puts the situation first and asks after it is read this way: the ask has to\n    be there at all (`asks`), and it has to be a clause of its own, so that the words beside it\n    are the situation and not the same sentence. A line that states one thing and asks nothing\n    (\"my nose wont stop running and i keep sneezing\") has no question half to prefer, and reading\n    its whole line as one put the training-injury card - which keys on \"running\" - in front of\n    the runny nose.\n\n    Compared as loose forms, so \"stop\" and \"stopped\" are the same word here. A word of the ask\n    that the line says only earlier on means the reading is not split, and the rule stands down.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("asks")]))) } }) {
    return Py::None;
    }
    focus = mcall(&understood.clone(), "get", &[py_str("focus")]);
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("clauses")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut clause: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[clause.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&clause.clone(), &py_list(vec![py_str("text")]))) } }) {  __out.push(clause.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[focus.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&focus.clone())); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return Py::None;
    }
    last = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = geti(&clauses.clone(), &neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(_forms(&[call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))])); } } py_set(__out) })() };
    earlier = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&clauses.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut clause: Py = __item.clone(); for __item in py_iter_or_raise(&{ let __b = clause.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(_forms(&[call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))])); } } } py_set(__out) })() };
    longest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&clauses.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut clause: Py = __item.clone(); __out.push(call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = clause.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }); __kws.extend(vec![("default".to_string(), Py::Int(0))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&earlier.clone())); if py_truthy(&__b) { __b } else { lt(&longest.clone(), &Py::Int(4)) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&focus.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    form = _forms(&[word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&last.clone(), &form.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&earlier.clone(), &form.clone())) } }) {
    continue;
    }
    return Py::None;
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&focus.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _rows_key(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut typed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut named: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut frame: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frame"))) };
    let mut framed: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "framed") { v.clone() }
    else { Py::Bool(false) };
    let mut asked: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { py_tuple(vec![]) };
    let mut ask_said: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask_said") { v.clone() }
    else { py_tuple(vec![]) };
    let mut form: Py = Py::None;
    let mut group: Py = Py::None;
    let mut one: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reading of the line a scan answers, or None when it cannot be written down.");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), py_tuple(vec![call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut group: Py = __item.clone(); __out.push(call_builtin("tuple", &[call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&group.clone()) { let mut form: Py = __item.clone(); __out.push(call_builtin("str", &[form.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = typed.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = named.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("bool", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("bool", &[framed.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = asked.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = ask_said.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _line_rows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut typed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut entries: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut by_domain: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "by_domain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: by_domain"))) };
    let mut every: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut named: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { py_tuple(vec![]) };
    let mut frame: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { Py::Bool(true) };
    let mut stands_down: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stands_down") { v.clone() }
    else { Py::None };
    let mut framed: Py = if __pos.len() > 8 { __pos[8].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "framed") { v.clone() }
    else { Py::Bool(false) };
    let mut ask_names: Py = if __pos.len() > 9 { __pos[9].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask_names") { v.clone() }
    else { py_tuple(vec![]) };
    let mut ask_said: Py = if __pos.len() > 10 { __pos[10].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask_said") { v.clone() }
    else { py_tuple(vec![]) };
    let mut got: Py = Py::None;
    let mut key: Py = Py::None;
    let mut ranked_items: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("[(asked, rank, plans it, names the thing named, key parts, entry, on the ask)] reached.\n\n    `stands_down` is a test of the whole reading, not of the entry's own key words, so the entry\n    it passes over is left out of what this call returns and not out of what the line reaches:\n    the scan is kept as it is and the rows are filtered on the way out.\n\n    framed: the line is the subject inside an advice ask, which spends its other words on the\n    asking (see `score`). It changes what an entry scores, so it is part of the memo key.\n\n    ask_names: the names the question half holds (see `_the_names_in_the_ask`). They are a\n    column of the row, so they are part of the memo key too.\n\n    ask_said: the question half's words in the order they were said, which is how a card that\n    keys a phrase of two of them is reached (see `_the_longest_run`). Also part of the key.\n    ");
    key = _rows_key(&[forms.clone(), typed.clone(), named.clone(), frame.clone(), framed.clone(), ask_names.clone(), ask_said.clone()]);
    ranked_items = Py::None;
    if py_truthy(&Py::Bool(!py_is(&key.clone(), &Py::None))) {
    got = mcall(&g__ROWS_CACHE(), "get", &[py_tuple(vec![call_builtin("id", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)), key.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&got.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&geti(&got.clone(), &Py::Int(0)), &entries.clone())) } }) {
    ranked_items = geti(&got.clone(), &Py::Int(1));
    }
    }
    if py_truthy(&Py::Bool(py_is(&ranked_items.clone(), &Py::None))) {
    ranked_items = _scanned_rows(&[forms.clone(), typed.clone(), entries.clone(), by_domain.clone(), every.clone(), named.clone(), frame.clone(), framed.clone(), ask_names.clone(), ask_said.clone()]);
    if py_truthy(&Py::Bool(!py_is(&key.clone(), &Py::None))) {
    if py_truthy(&ge(&call_builtin("len", &[g__ROWS_CACHE()]).unwrap_or_else(|e| exc::raise(e)), &g__ROWS_KEEP())) {
    let _ = mcall(&g__ROWS_CACHE(), "pop", &[call_builtin("next", &[call_builtin("iter", &[g__ROWS_CACHE()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    let _ = py_setitem(&g__ROWS_CACHE(), &py_tuple(vec![call_builtin("id", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)), key.clone()]), py_tuple(vec![entries.clone(), ranked_items.clone()]));
    }
    }
    if py_truthy(&Py::Bool(py_is(&stands_down.clone(), &Py::None))) {
    return call_builtin("list", &[ranked_items.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked_items.clone()) { let mut row: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&py_call(&stands_down, &[geti(&row.clone(), &Py::Int(5))])))) {  __out.push(row.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _scanned_rows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut typed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut entries: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut by_domain: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "by_domain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: by_domain"))) };
    let mut every: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut named: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut frame: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frame"))) };
    let mut framed: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "framed") { v.clone() }
    else { Py::Bool(false) };
    let mut ask_names: Py = if __pos.len() > 8 { __pos[8].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask_names") { v.clone() }
    else { py_tuple(vec![]) };
    let mut ask_said: Py = if __pos.len() > 9 { __pos[9].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask_said") { v.clone() }
    else { py_tuple(vec![]) };
    let mut ask_forms: Py = Py::None;
    let mut ask_line: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut asked_seen: Py = Py::None;
    let mut domain: Py = Py::None;
    let mut item: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut ranked: Py = Py::None;
    let mut ranked_items: Py = Py::None;
    let mut row: Py = Py::None;
    let mut run: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every entry the line reaches, scored: the work one reading of one line costs.");
    ranked_items = py_list(vec![]);
    ask_forms = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask_names.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() };
    ask_line = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ask_said.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() };
    seen = py_dict(vec![]);
    asked_seen = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&_reaching(&[entries.clone(), forms.clone()]));
    for __item in __iter1 {
    item = __item.clone();
    value = score(&[item.clone(), forms.clone(), typed.clone(), seen.clone(), frame.clone(), framed.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    continue;
    }
    domain = { let __b = call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } };
    ranked = _rank(&[item.clone(), forms.clone(), { let __b = mcall(&by_domain.clone(), "get", &[domain.clone()]); if py_truthy(&__b) { __b } else { every.clone() } }, seen.clone()]);
    asked = if py_truthy(&_only_the_want(&[item.clone(), forms.clone(), seen.clone()])) { Py::Int(0) } else { Py::Int(1) };
    run = _the_longest_run(&[item.clone(), ask_line.clone(), ask_forms.clone(), asked_seen.clone()]);
    let _ = mcall(&ranked_items.clone(), "append", &[py_tuple(vec![asked.clone(), ranked.clone(), _plans_it(&[item.clone(), forms.clone()]), _names_one(&[item.clone(), named.clone()]), call_builtin("len", &[gattr(&item.clone(), "key")]).unwrap_or_else(|e| exc::raise(e)), item.clone(), run.clone()])]);
    }
    longest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked_items.clone()) { let mut row: Py = __item.clone(); __out.push(geti(&row.clone(), &Py::Int(6))); } py_list(__out) })() }); __kws.extend(vec![("default".to_string(), Py::Int(0))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&longest.clone()) {
    ranked_items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked_items.clone()) { let mut row: Py = __item.clone(); __out.push(add(&slice_or_raise(&row.clone(), &Py::None, &Py::Int(6), &Py::None), &py_tuple(vec![if py_truthy(&Py::Bool(py_eq(&geti(&row.clone(), &Py::Int(6)), &longest.clone()))) { Py::Int(1) } else { Py::Int(0) }]))); } py_list(__out) })() };
    }
    return ranked_items.clone();
    Py::None
}
pub fn _the_longest_run(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut ask_line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask_line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask_line"))) };
    let mut name_forms: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name_forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name_forms"))) };
    let mut asked_seen: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked_seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked_seen"))) };
    let mut best: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut part: Py = Py::None;
    let mut span: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How many words of the question half one key part of this entry accounts for, or 0.\n\n    A name of the question half has to be in what matched: the ask's own words (\"what\", \"get\",\n    \"should\") are the vocabulary of asking and a card that keys one of them is not on the ask.\n    Past that, the length of the run is the whole reading, because a card that keys a phrase of\n    two words of the ask is more about it than one that keys a single word of it.\n\n    The memo of matches is the caller's and holds no entry yet: `seen` in the scan is keyed by\n    id(part) alone, so reusing it would hand back the matches found in the whole line, which is\n    every part that reached the line at all.\n    ");
    wanted = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&name_forms.clone());
    for __item in __iter1 {
    forms = __item.clone();
    let __aug2 = bitor(&wanted.clone(), &forms.clone());
    wanted = __aug2;
    }
    best = Py::Int(0);
    let __iter3 = py_iter_or_raise(&{ let __b = gattr(&item.clone(), "key"); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter3 {
    part = __item.clone();
    let __iter4 = py_iter_or_raise(&_part_matches(&[part.clone(), ask_line.clone(), asked_seen.clone()]));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 3, None);
    weight = __unpk5[0].clone();
    words = __unpk5[1].clone();
    span = __unpk5[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&weight.clone())); if py_truthy(&__b) { __b } else { le(&call_builtin("len", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)), &best.clone()) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(bitand(&_forms(&[call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))]), &wanted.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    best = call_builtin("len", &[span.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return best.clone();
    Py::None
}
pub fn _row_order(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut row: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "row") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: row"))) };
    let _ = py_str("The order two entries compete in: asked, on the ask, score, plan, the named thing, parts.");
    return py_tuple(vec![geti(&row.clone(), &Py::Int(0)), geti(&row.clone(), &Py::Int(6)), geti(&row.clone(), &Py::Int(1)), geti(&row.clone(), &Py::Int(2)), geti(&row.clone(), &Py::Int(3)), geti(&row.clone(), &Py::Int(4))]);
    Py::None
}
pub fn _plainer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plain: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plain"))) };
    let mut framed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "framed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: framed"))) };
    let mut margin: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "margin") { v.clone() }
    else { g_PLAIN_MARGIN() };
    let mut better: Py = Py::None;
    let mut other: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("True when the whole line reaches a clearly better entry than the line without its clause.\n\n    A margin, not a straight comparison: reading the left-out thing as the subject beats the\n    rest of the line by one rank point in plenty of lines where it is wrong (\"how do i stop my\n    baby crying without picking him up\" reaches the skin-picking entry that way), and the entry\n    that wins here has to win by more than that to be believed.\n    ");
    better = call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut row: Py = __item.clone(); __out.push(slice_or_raise(&row.clone(), &Py::None, &Py::Int(2), &Py::None)); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    other = call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&framed.clone()) { let mut row: Py = __item.clone(); __out.push(slice_or_raise(&row.clone(), &Py::None, &Py::Int(2), &Py::None)); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = gt(&geti(&better.clone(), &Py::Int(0)), &geti(&other.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { ge(&geti(&better.clone(), &Py::Int(1)), &add(&geti(&other.clone(), &Py::Int(1)), &margin.clone())) } };
    Py::None
}
pub fn _the_subjects_own_row(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut forms: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut item: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut rows: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rows") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rows"))) };
    let mut domain: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("True when the whole line's fault row is the answer, not a card about a different question.\n\n    The whole line beats the line with its \"without\" clause emptied by one rank point on plenty\n    of lines where reading the clause as the subject is wrong, which is what the margin in\n    `_plainer` is for. A clause that names a fault of the thing in hand is the question, so the\n    whole line may win by that one point, but only where the rest of the line has no answer of\n    its own. Two things have to hold.\n\n    The row the whole line reaches is in the same domain as a row the rest of the line reaches:\n    a clause that buys a row from another domain has changed the subject, not answered the line\n    (\"how do i say sorry without it sounding fake\" is an apology, not a phishing warning).\n\n    And the row the rest of the line reaches is a stand-in list rather than an answer: an entry\n    whose key is what to use instead of something is not the how-to the person asked for, so the\n    fault card beside it is better. \"how do i fry eggs without them sticking\" reached the egg\n    swap (\"one egg becomes 60g of yoghurt\") and wanted the pan.\n\n        how do i fry eggs without them sticking   -> egg swap (a stand-in list) / fix-sticking\n        how do i cook salmon without it sticking  -> the salmon method: the answer, not a card\n        how do i paint a door without it dripping -> the door how-to: not a ceiling stain card\n        how do i iron a shirt without it shining  -> the shirt how-to, not the scorch card\n\n    Measured on the deck's \"without it\" lines: the salmon, door, shirt and puppy lines keep the\n    how-to they had, and the egg line gets the pan.\n    ");
    domain = call_builtin("getattr", &[item.clone(), py_str("domain"), Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&domain.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_clause_about_the_thing(&[forms.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rows.clone()) { let mut row: Py = __item.clone(); __out.push(Py::Bool(py_eq(&call_builtin("getattr", &[geti(&row.clone(), &Py::Int(5)), py_str("domain"), Py::None]).unwrap_or_else(|e| exc::raise(e)), &domain.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return _offers_a_substitute(&[geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rows.clone()); __kws.extend(vec![("key".to_string(), py_func("_row_order", |args: &[Py]| _row_order(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(5))]);
    Py::None
}
pub fn _offers_a_substitute(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut alternative: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("True when this entry exists to say what can stand in for something else.");
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SUBSTITUTE_WORDS()) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _inside(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut seen: Py = Py::None;
    let _ = py_str("True when this part is the other one, or a part of it.");
    seen = Py::Int(0);
    while py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { lt(&seen.clone(), &Py::Int(4)) } }) {
    if py_truthy(&bitand(&_forms(&[part.clone()]), &_forms(&[other.clone()]))) {
    return Py::Bool(true);
    }
    part = mcall(&g_PART_INSIDE(), "get", &[part.clone(), py_str("")]);
    let __aug1 = add(&seen.clone(), &Py::Int(1));
    seen = __aug1;
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_cut_of_meat(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut near: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the body word at `index` sits beside a cut of meat, so it names the cut.");
    near = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![sub(&index.clone(), &Py::Int(2)), sub(&index.clone(), &Py::Int(1)), add(&index.clone(), &Py::Int(1)), add(&index.clone(), &Py::Int(2))])) { let mut at: Py = __item.clone(); if py_truthy(&if py_truthy(&le(&Py::Int(0), &{ let __cmp10 = at.clone(); __cmp10.clone() })) { lt(&__cmp10.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Bool(false) }) {  __out.push(mcall(&call_builtin("str", &[geti(&words.clone(), &at.clone())]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&near.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__CUT_BESIDE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _body_parts(args: &[Py]) -> Py {
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
    let mut parts: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The body parts the line names, without a \"back\" that only says the thing returned.");
    parts = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[{ let __b = words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    word = mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_BODY_WORDS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("back"))); if !py_truthy(&__b) { __b } else { { let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_RETURNED_AFTER(), &mcall(&call_builtin("str", &[geti(&words.clone(), &sub(&index.clone(), &Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_GENERAL_BODY(), &word.clone()))) {
    continue;
    }
    if py_truthy(&_a_cut_of_meat(&[words.clone(), index.clone()])) {
    continue;
    }
    let _ = mcall(&parts.clone(), "add", &[word.clone()]);
    }
    return parts.clone();
    Py::None
}
pub fn _key_body_parts(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut alternative: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The body parts this entry's key is written on.\n\n    Read with the same eye as the line: a key that says \"keeps coming back\" names no part, and\n    the entry that answers a rash which returns is still the answer to ``the rash on my knee\n    keeps coming back``.\n    ");
    parts = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[item.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    alternative = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[alternative.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug3 = bitor(&parts.clone(), &_body_parts(&[mcall(&mcall(&alternative.clone(), "lower", &[]), "split", &[])]));
    parts = __aug3;
    }
    }
    }
    return parts.clone();
    Py::None
}
pub fn _a_part_the_line_does_not_have(args: &[Py]) -> Py {
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
    let mut parts_said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "parts_said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: parts_said"))) };
    let mut key: Py = Py::None;
    let _ = py_str("Whether this fence word names a body part the line says as something other than a part.\n\n    Only a single word that is a part can: a fence written on a phrase (\"a mole on my back\", \"bad\n    back\") says what it means, and is left alone. The word is only asked here after it was found\n    in the line, so it is in the line - the question is whether it is the part.\n    ");
    key = if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&key.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&key.clone(), &py_str(" "))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_BODY_WORDS(), &key.clone())) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&{ let __b = parts_said.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &key.clone()));
    Py::None
}
pub fn _another_part(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut line_parts: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line_parts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line_parts"))) };
    let mut other: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the entry is keyed on a body part the line does not have.\n\n    A line that names one part is about that part: an entry keyed on a different one answers a\n    question nobody asked, however well its other words fit (\"pain\" is in most of them). Only\n    entries whose key names a part are held to it, so a row about a return (\"keeps coming\n    back\") or a place is untouched, and a line that names no part at all is untouched too - the\n    filter narrows what a body word may reach, and never widens it.\n\n    A key that names many parts is not about one of them: `rash` lists the arm, the leg, the\n    chest and the back because a rash can be on any of them, and holding it to those words took\n    \"the rash on my knee keeps coming back\" off its own answer. Only a key written on one part,\n    or a part and its other name (`sore-knee` is \"knee\" and \"kneecap\"), says what the entry is\n    about. The words are compared by their loose forms, because the line says \"my knees ache\"\n    and the key says \"knee\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&line_parts.clone()))) {
    return Py::Bool(false);
    }
    parts = _key_body_parts(&[item.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&parts.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_SPECIFIC_PARTS()) } }) {
    return Py::Bool(false);
    }
    said = call_builtin("list", &[line_parts.clone()]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); for __item in py_iter_or_raise(&said.clone()) { let mut other: Py = __item.clone(); __out.push(_inside(&[other.clone(), part.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _asks_if_it_is_still_good(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks whether a thing is still fit to eat, drink or use.");
    if py_truthy(&Py::Bool(!py_truthy(&typed.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__STILL_GOOD_ASHERS(), "intersection", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_keeping_row(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("True when the entry answers how long a thing keeps, where to keep it, or if it is fit.");
    reply = { let __b = call_builtin("getattr", &[item.clone(), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return _a_keeping_key(&[item.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::Bool(false);
    }
    reply = add(&add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&reply.clone(), "lower", &[]), "split", &[])])), &py_str(" "));
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__KEEPING_ANSWER()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&reply.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__A_KEEP_AND_A_LENGTH(), "search", &[reply.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _a_keeping_key(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut collect: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The key-word test, used only for a row whose reply is built at the turn.");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
pub fn __nested_collect_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut collect: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut part: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "part").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut inner: Py = Py::None;
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&words.clone(), "update", &[mcall(&mcall(&part.clone(), "lower", &[]), "split", &[])]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&part.clone());
    for __item in __iter1 {
    inner = __item.clone();
    let _ = py_call(&collect, &[inner.clone()]);
    }
    }
    }
    Py::None
}
    collect = py_func("collect", move |args: &[Py]| { let mut full = vec![collect.clone(), item.clone(), words.clone()]; full.extend_from_slice(args); __nested_collect_0(&full) });
    let _ = py_call(&collect, &[{ let __b = call_builtin("getattr", &[item.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    return call_builtin("bool", &[mcall(&g__KEEPING_WORDS(), "intersection", &[words.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_dress_code(args: &[Py]) -> Py {
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
    let _ = py_str("True when the colour at *index* is the first word of a dress code (\"black tie\").");
    return { let __b = lt(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("black"), py_str("white")]), &geti(&words.clone(), &index.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_DRESS_CODE_AFTER(), &mcall(&call_builtin("str", &[geti(&words.clone(), &add(&index.clone(), &Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } } } };
    Py::None
}
pub fn _the_colour_is_the_whole_ask(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the colour at *index* is the subject, not the adjective of a thing.\n\n    Read from the word after it: a verdict carries the ask on the colour (\"is white ok\"), a\n    function word hands the ask on to what comes later (\"should i wear white to a wedding\"), and\n    something worn or another colour makes the colour an adjective of clothes (\"a white shirt\",\n    \"black and white\"). Any other noun takes the colour with it (\"a white cake\", \"a black dog\"),\n    and then the line is not a question about what to wear.\n    ");
    if py_truthy(&ge(&add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(true);
    }
    after = add(&index.clone(), &Py::Int(1));
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[call_builtin("str", &[geti(&words.clone(), &after.clone())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { lt(&add(&after.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    let __aug1 = add(&after.clone(), &Py::Int(1));
    after = __aug1;
    }
    word = mcall(&mcall(&call_builtin("str", &[geti(&words.clone(), &after.clone())]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&g_AFTER_A_COLOUR_STANDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WORN_AFTER_A_COLOUR(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COLOUR_WORDS(), &word.clone())) } } } };
    Py::None
}
pub fn a_colour_is_worn(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut index: Py = Py::None;
    let mut said: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut typing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks about a colour of something worn, so it asks what to put on.\n\n    Read from the line as typed, like `asks_what_to_wear`, and it takes either the line as a\n    string or the reading: the reading's own words carry the conversation's topic along with\n    them, and a carried topic is what made a colour question about the garden.\n    ");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    understood = py_dict(vec![(py_str("text"), understood.clone())]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    words = mcall(&understood.clone(), "get", &[py_str("words")]);
    said = if py_truthy(&call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]) } else { py_str("") };
    }
    typing = tokens(&[said.clone()]);
    typed = call_builtin("set", &[typing.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&typed.clone(), &g_COLOUR_ASKED_WORDS())))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[typing.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk4944 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk4944[0].clone(); let mut word: Py = __unpk4944[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_COLOUR_ASKED_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_dress_code(&[typing.clone(), index.clone()]))); if !py_truthy(&__b) { __b } else { _the_colour_is_the_whole_ask(&[typing.clone(), index.clone()]) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = bitand(&g_WEARING(), &typed.clone()); if py_truthy(&__b) { __b } else { mcall(&g_OCCASION(), "search", &[mcall(&py_str(" "), "join", &[typing.clone()])]) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::Bool(true);
    }
    first = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tokens(&[said.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("so")))) {  __out.push(word.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[Py::Bool(py_contains(&g_ASKING_FIRST(), &first.clone()))]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_COLOUR_ASKED(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_words_without_a_colour(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line's words with a colour it used as a colour taken out, or None when it used none.");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_COLOUR_USED(), "finditer", &[call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut match_: Py = __item.clone(); __out.push(mcall(&mcall(&match_.clone(), "group", &[py_str("colour")]), "lower", &[])); } py_set(__out) })() };
    let __aug1 = bitand(&said.clone(), &g_COLOUR_WORDS());
    said = __aug1;
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&said.clone(), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    return kept.clone();
    Py::None
}
pub fn _the_key_words(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Every word an entry is keyed on, flattened, so a line word can be looked for in it.");
    said = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[entry.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "update", &[mcall(&mcall(&part.clone(), "lower", &[]), "split", &[])]);
    } else {
    let __iter2 = py_iter_or_raise(&part.clone());
    for __item in __iter2 {
    one = __item.clone();
    let _ = mcall(&said.clone(), "update", &[mcall(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    }
    }
    }
    return said.clone();
    Py::None
}
pub fn _the_occasion_family(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut members: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The family a matched occasion belongs to, or \"\" when it has no clothes card of its own.");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[]);
    let __iter1 = py_iter_or_raise(&mcall(&g_THE_SAME_OCCASION(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    family = __unpk2[0].clone();
    members = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&members.clone(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[family.clone()]) } }) {
    return family.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_occasion_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words an entry may be keyed on, for the occasion this line names itself.\n\n    Read from the line as typed: an occasion carried over from an earlier turn is not what the\n    person named this time, and a card for it would answer a question they did not ask.\n    ");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]);
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_OCCASION(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    family = _the_occasion_family(&[mcall(&match_.clone(), "group", &[Py::Int(0)])]);
    if py_truthy(&family.clone()) {
    let __aug2 = bitor(&found.clone(), &geti(&g_THE_SAME_OCCASION(), &family.clone()));
    found = __aug2;
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_card_for_the_occasion_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut clean: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut forms: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut typed: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut entries: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut by_domain: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "by_domain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: by_domain"))) };
    let mut every: Py = if __pos.len() > 8 { __pos[8].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut reject: Py = if __pos.len() > 9 { __pos[9].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reject"))) };
    let mut again: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut for_this: Py = Py::None;
    let _ = py_str("The clothes card for the occasion the line names, when it is not the card reached.\n\n    Only a colour asked about what to wear gets here (see `a_colour_is_worn`), and only a clothes\n    entry that is keyed on the occasion can take the turn: the line still has to reach it, so a\n    row that does not fit is no answer and the line keeps the one it had.\n    ");
    asked = _the_occasion_words(&[typed.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return found.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&found.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { bitand(&_the_key_words(&[found.clone()]), &asked.clone()) } }) {
    return found.clone();
    }
    for_this = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entries.clone()) { let mut entry: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("str", &[call_builtin("getattr", &[entry.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &py_str("clothing"))); if !py_truthy(&__b) { __b } else { bitand(&_the_key_words(&[entry.clone()]), &asked.clone()) } }) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&for_this.clone()))) {
    return found.clone();
    }
    again = _best(&[understood.clone(), words.clone(), clean.clone(), forms.clone(), typed.clone(), for_this.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    return if py_truthy(&Py::Bool(!py_is(&again.clone(), &Py::None))) { again.clone() } else { found.clone() };
    Py::None
}
pub fn _the_answer_the_colour_dragged_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut clean: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut entries: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut by_domain: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "by_domain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: by_domain"))) };
    let mut every: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut reject: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reject"))) };
    let mut again: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut others: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The entry the line reaches once a colour it used as a colour is left out, else `found`.\n\n    A colour word is not one of the line's subject words, so the answer that needs it is an answer\n    about something else: the line is read once more without it, and that entry answers the line\n    when it reaches one. Only an entry that has nothing else of the line in it is given up - an\n    entry keyed on the colour and on another word the line says is about that word (\"the leaves\n    have gone brown\" is the leaves entry, keyed on both) - and when the second read reaches\n    nothing the line keeps the answer it had, so a colour the knowledge base does answer is not\n    taken away from a person who asked about it.\n    ");
    kept = _the_words_without_a_colour(&[clean.clone(), words.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&kept.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&found.clone(), &Py::None)) } }) {
    return found.clone();
    }
    others = sub(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() }, &g__BARE_IN_A_KEY());
    if py_truthy(&bitand(&others.clone(), &_the_key_words(&[found.clone()]))) {
    return found.clone();
    }
    again = _best(&[understood.clone(), kept.clone(), mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() }, { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() }, entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    return if py_truthy(&Py::Bool(!py_is(&again.clone(), &Py::None))) { again.clone() } else { found.clone() };
    Py::None
}
pub fn _best(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut forms: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut typed: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut entries: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut by_domain: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "by_domain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: by_domain"))) };
    let mut every: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut reject: Py = if __pos.len() > 8 { __pos[8].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { Py::None };
    let mut framed: Py = if __pos.len() > 9 { __pos[9].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "framed") { v.clone() }
    else { Py::Bool(false) };
    let mut _asked: Py = Py::None;
    let mut _keys: Py = Py::None;
    let mut _named: Py = Py::None;
    let mut _on_ask: Py = Py::None;
    let mut _plans: Py = Py::None;
    let mut _ranked: Py = Py::None;
    let mut aged_a_thing: Py = Py::None;
    let mut ask_names: Py = Py::None;
    let mut ask_said: Py = Py::None;
    let mut ask_thing: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut cold_place: Py = Py::None;
    let mut degrees_are_heat: Py = Py::None;
    let mut item: Py = Py::None;
    let mut k: Py = Py::None;
    let mut keeping_question: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut line_parts: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut rank: Py = Py::None;
    let mut ranked_items: Py = Py::None;
    let mut row: Py = Py::None;
    let mut says_ask: Py = Py::None;
    let mut says_scene: Py = Py::None;
    let mut scene: Py = Py::None;
    let mut scene_only: Py = Py::None;
    let mut turned_down: Py = Py::None;
    let mut v: Py = Py::None;
    let mut whole_line_first: Py = Py::None;
    let _ = py_str("The highest-ranked entry the line as typed reaches, or the one another reading reaches.\n\n    framed: the line is the subject inside an advice ask, which spends its other words on the\n    asking. Only the entries in ASK_FRAME_IDS read it that way (see `score`).\n    ");
    line_parts = _body_parts(&[words.clone()]);
    kept = mcall(&understood.clone(), "get", &[py_str("kept")]);
    keeping_question = { let __b = call_builtin("bool", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _asks_if_it_is_still_good(&[typed.clone()]) } };
    ask_names = _the_names_in_the_ask(&[understood.clone(), every.clone()]);
    ask_said = _the_ask_in_order(&[understood.clone()]);
    degrees_are_heat = _the_degrees_are_a_temperature(&[understood.clone()]);
    aged_a_thing = the_years_are_a_things(&[understood.clone()]);
    cold_place = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_speaker_is_feeling(&[typed.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_person_says_they_are_cold(&[typed.clone()]))) } }) {
    cold_place = _the_place_the_line_says_is_cold(&[typed.clone()]);
    }
pub fn __nested_rank_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _asked: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut _keys: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut _named: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut _on_ask: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut _plans: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut _ranked: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut aged_a_thing: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut ask_names: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut ask_said: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut ask_thing: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut by_domain: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut carried: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut clean: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut cold_place: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut degrees_are_heat: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut entries: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut every: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut forms: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut framed: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut k: Py = __pos.get(20).cloned().unwrap_or(Py::None);
    let mut keeping_question: Py = __pos.get(21).cloned().unwrap_or(Py::None);
    let mut kept: Py = __pos.get(22).cloned().unwrap_or(Py::None);
    let mut line_parts: Py = __pos.get(23).cloned().unwrap_or(Py::None);
    let mut plain: Py = __pos.get(24).cloned().unwrap_or(Py::None);
    let mut rank: Py = __pos.get(25).cloned().unwrap_or(Py::None);
    let mut ranked_items: Py = __pos.get(26).cloned().unwrap_or(Py::None);
    let mut reject: Py = __pos.get(27).cloned().unwrap_or(Py::None);
    let mut row: Py = __pos.get(28).cloned().unwrap_or(Py::None);
    let mut says_ask: Py = __pos.get(29).cloned().unwrap_or(Py::None);
    let mut says_scene: Py = __pos.get(30).cloned().unwrap_or(Py::None);
    let mut scene: Py = __pos.get(31).cloned().unwrap_or(Py::None);
    let mut scene_only: Py = __pos.get(32).cloned().unwrap_or(Py::None);
    let mut turned_down: Py = __pos.get(33).cloned().unwrap_or(Py::None);
    let mut typed: Py = __pos.get(34).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(35).cloned().unwrap_or(Py::None);
    let mut v: Py = __pos.get(36).cloned().unwrap_or(Py::None);
    let mut whole_line_first: Py = __pos.get(37).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(38).cloned().unwrap_or(Py::None);
    let mut reading: Py = __pos.get(39).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "reading").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut filter_keeping: Py = __pos.get(40).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "filter_keeping").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut named: Py = Py::None;
    let mut plain_rows: Py = Py::None;
    let mut reached: Py = Py::None;
    let mut stands_down: Py = Py::None;
    let _ = py_str("The entries the line reaches, with the keeping-only rule on or off.");
pub fn __nested_stands_down_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _asked: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut _keys: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut _named: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut _on_ask: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut _plans: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut _ranked: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut aged_a_thing: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut ask_names: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut ask_said: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut ask_thing: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut by_domain: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut carried: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut clean: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut cold_place: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut degrees_are_heat: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut entries: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut every: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut filter_keeping: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut forms: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut framed: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(20).cloned().unwrap_or(Py::None);
    let mut k: Py = __pos.get(21).cloned().unwrap_or(Py::None);
    let mut keeping_question: Py = __pos.get(22).cloned().unwrap_or(Py::None);
    let mut kept: Py = __pos.get(23).cloned().unwrap_or(Py::None);
    let mut line_parts: Py = __pos.get(24).cloned().unwrap_or(Py::None);
    let mut named: Py = __pos.get(25).cloned().unwrap_or(Py::None);
    let mut plain: Py = __pos.get(26).cloned().unwrap_or(Py::None);
    let mut plain_rows: Py = __pos.get(27).cloned().unwrap_or(Py::None);
    let mut rank: Py = __pos.get(28).cloned().unwrap_or(Py::None);
    let mut ranked_items: Py = __pos.get(29).cloned().unwrap_or(Py::None);
    let mut reached: Py = __pos.get(30).cloned().unwrap_or(Py::None);
    let mut reading: Py = __pos.get(31).cloned().unwrap_or(Py::None);
    let mut reject: Py = __pos.get(32).cloned().unwrap_or(Py::None);
    let mut row: Py = __pos.get(33).cloned().unwrap_or(Py::None);
    let mut says_ask: Py = __pos.get(34).cloned().unwrap_or(Py::None);
    let mut says_scene: Py = __pos.get(35).cloned().unwrap_or(Py::None);
    let mut scene: Py = __pos.get(36).cloned().unwrap_or(Py::None);
    let mut scene_only: Py = __pos.get(37).cloned().unwrap_or(Py::None);
    let mut stands_down: Py = __pos.get(38).cloned().unwrap_or(Py::None);
    let mut turned_down: Py = __pos.get(39).cloned().unwrap_or(Py::None);
    let mut typed: Py = __pos.get(40).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(41).cloned().unwrap_or(Py::None);
    let mut v: Py = __pos.get(42).cloned().unwrap_or(Py::None);
    let mut whole_line_first: Py = __pos.get(43).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(44).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(45).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "item").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&guard_passes(&[item.clone(), reading.clone()])))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = degrees_are_heat.clone(); if !py_truthy(&__b) { __b } else { _answers_a_qualification(&[item.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = aged_a_thing.clone(); if !py_truthy(&__b) { __b } else { _the_age_is_a_childs(&[item.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = filter_keeping.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_keeping_row(&[item.clone()]))) } }) {
    return Py::Bool(true);
    }
    return _another_part(&[item.clone(), line_parts.clone()]);
    Py::None
}
    stands_down = py_func("stands_down", move |args: &[Py]| { let mut full = vec![_asked.clone(), _keys.clone(), _named.clone(), _on_ask.clone(), _plans.clone(), _ranked.clone(), aged_a_thing.clone(), ask_names.clone(), ask_said.clone(), ask_thing.clone(), by_domain.clone(), carried.clone(), clean.clone(), cold_place.clone(), degrees_are_heat.clone(), entries.clone(), every.clone(), filter_keeping.clone(), forms.clone(), framed.clone(), item.clone(), k.clone(), keeping_question.clone(), kept.clone(), line_parts.clone(), named.clone(), plain.clone(), plain_rows.clone(), rank.clone(), ranked_items.clone(), reached.clone(), reading.clone(), reject.clone(), row.clone(), says_ask.clone(), says_scene.clone(), scene.clone(), scene_only.clone(), stands_down.clone(), turned_down.clone(), typed.clone(), understood.clone(), v.clone(), whole_line_first.clone(), words.clone()]; full.extend_from_slice(args); __nested_stands_down_0(&full) });
    named = _named_things(&[reading.clone()]);
    reached = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(forms.clone()); __args.push(typed.clone()); __args.push(entries.clone()); __args.push(by_domain.clone()); __args.push(every.clone()); __args.push(named.clone()); __kws.extend(vec![("stands_down".to_string(), stands_down.clone()), ("framed".to_string(), framed.clone()), ("ask_names".to_string(), ask_names.clone()), ("ask_said".to_string(), ask_said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _line_rows(args))(&__args) };
    plain_rows = Py::None;
    if py_truthy(&geti(&_frame_masked(&[forms.clone()]), &Py::Int(1))) {
    plain_rows = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(forms.clone()); __args.push(typed.clone()); __args.push(entries.clone()); __args.push(by_domain.clone()); __args.push(every.clone()); __args.push(named.clone()); __kws.extend(vec![("frame".to_string(), Py::Bool(false)), ("stands_down".to_string(), stands_down.clone()), ("framed".to_string(), framed.clone()), ("ask_names".to_string(), ask_names.clone()), ("ask_said".to_string(), ask_said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _line_rows(args))(&__args) };
    if py_truthy(&plain_rows.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), py_func("_row_order", |args: &[Py]| _row_order(args))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&plain_rows.clone(), "sort", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reached.clone())); if py_truthy(&__b) { __b } else { { let __b = _plainer(&[plain_rows.clone(), reached.clone()]); if py_truthy(&__b) { __b } else { { let __b = _offers_a_substitute(&[geti(&geti(&plain_rows.clone(), &Py::Int(0)), &Py::Int(5))]); if py_truthy(&__b) { __b } else { _the_subjects_own_row(&[forms.clone(), geti(&geti(&plain_rows.clone(), &Py::Int(0)), &Py::Int(5)), reached.clone()]) } } } } } }) {
    reached = plain_rows.clone();
    }
    }
    }
    return py_tuple(vec![reached.clone(), plain_rows.clone()]);
    Py::None
}
    rank = py_func("rank", move |args: &[Py]| { let mut full = vec![_asked.clone(), _keys.clone(), _named.clone(), _on_ask.clone(), _plans.clone(), _ranked.clone(), aged_a_thing.clone(), ask_names.clone(), ask_said.clone(), ask_thing.clone(), by_domain.clone(), carried.clone(), clean.clone(), cold_place.clone(), degrees_are_heat.clone(), entries.clone(), every.clone(), forms.clone(), framed.clone(), item.clone(), k.clone(), keeping_question.clone(), kept.clone(), line_parts.clone(), plain.clone(), rank.clone(), ranked_items.clone(), reject.clone(), row.clone(), says_ask.clone(), says_scene.clone(), scene.clone(), scene_only.clone(), turned_down.clone(), typed.clone(), understood.clone(), v.clone(), whole_line_first.clone(), words.clone()]; full.extend_from_slice(args); __nested_rank_0(&full) });
    let __unpk2 = py_unpack(&py_call(&rank, &[understood.clone(), keeping_question.clone()]), 2, None);
    ranked_items = __unpk2[0].clone();
    plain = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&ranked_items.clone())); if !py_truthy(&__b) { __b } else { keeping_question.clone() } }) {
    let __unpk3 = py_unpack(&py_call(&rank, &[{ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&understood.clone(), "items", &[])) { let __unpk821 = py_unpack(&__item.clone(), 2, None); let mut k: Py = __unpk821[0].clone(); let mut v: Py = __unpk821[1].clone(); if py_truthy(&Py::Bool(!py_eq(&k.clone(), &py_str("kept")))) {  __out.push((k.clone(), v.clone())); } } py_dict(__out) })() }, Py::Bool(false)]), 2, None);
    ranked_items = __unpk3[0].clone();
    plain = __unpk3[1].clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&ranked_items.clone()))) {
    return _other_reading(&[understood.clone(), words.clone(), clean.clone(), entries.clone(), reject.clone()]);
    }
    ranked_items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked_items.clone()) { let mut row: Py = __item.clone(); if py_truthy(&gt(&geti(&row.clone(), &Py::Int(1)), &Py::Int(1))) {  __out.push(row.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&ranked_items.clone()))) {
    return _other_reading(&[understood.clone(), words.clone(), clean.clone(), entries.clone(), reject.clone()]);
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), py_func("_row_order", |args: &[Py]| _row_order(args))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ranked_items.clone(), "sort", args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&cold_place.clone(), &Py::None))) {
    ranked_items = _the_card_for_the_place_first(&[cold_place.clone(), ranked_items.clone()]);
    }
    carried = _the_subject_the_line_leans_on(&[understood.clone()]);
    ask_thing = mcall(&understood.clone(), "get", &[py_str("ask_subject")]);
    scene = mcall(&understood.clone(), "get", &[py_str("setting")]);
    scene_only = { let __b = call_builtin("isinstance", &[ask_thing.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[scene.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&ask_thing.clone(), &scene.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_says_what_is_missing(&[understood.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_is_a_person(&[understood.clone(), ask_thing.clone()]), &_is_a_person(&[understood.clone(), scene.clone()]))) } } } } } } } };
    turned_down = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter4 = py_iter_or_raise(&py_tuple(vec![Py::Bool(true), Py::Bool(false)]));
    for __item in __iter4 {
    whole_line_first = __item.clone();
    let __iter5 = py_iter_or_raise(&ranked_items.clone());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 7, None);
    _asked = __unpk6[0].clone();
    _ranked = __unpk6[1].clone();
    _plans = __unpk6[2].clone();
    _named = __unpk6[3].clone();
    _keys = __unpk6[4].clone();
    item = __unpk6[5].clone();
    _on_ask = __unpk6[6].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&reject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { py_call(&reject, &[item.clone()]) } }) {
    let _ = mcall(&turned_down.clone(), "add", &[gattr(&item.clone(), "id")]);
    continue;
    }
    if py_truthy(&_the_subject_it_avoids(&[item.clone(), forms.clone(), carried.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&scene_only.clone()))) {
    return item.clone();
    }
    says_ask = about(&[item.clone(), ask_thing.clone()]);
    says_scene = about(&[item.clone(), scene.clone()]);
    if py_truthy(&{ let __b = says_scene.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&says_ask.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = says_scene.clone(); if !py_truthy(&__b) { __b } else { whole_line_first.clone() } }) {
    continue;
    }
    return item.clone();
    }
    let _ = mcall(&turned_down.clone(), "clear", &[]);
    }
    if py_truthy(&Py::Bool(!py_is(&plain.clone(), &Py::None))) {
    let __iter7 = py_iter_or_raise(&plain.clone());
    for __item in __iter7 {
    let __unpk8 = py_unpack(&__item.clone(), 7, None);
    _asked = __unpk8[0].clone();
    _ranked = __unpk8[1].clone();
    _plans = __unpk8[2].clone();
    _named = __unpk8[3].clone();
    _keys = __unpk8[4].clone();
    item = __unpk8[5].clone();
    _on_ask = __unpk8[6].clone();
    if py_truthy(&Py::Bool(py_contains(&turned_down.clone(), &gattr(&item.clone(), "id")))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&reject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { py_call(&reject, &[item.clone()]) } }) {
    continue;
    }
    if py_truthy(&_the_subject_it_avoids(&[item.clone(), forms.clone(), carried.clone()])) {
    continue;
    }
    return item.clone();
    }
    }
    return _other_reading(&[understood.clone(), words.clone(), clean.clone(), entries.clone(), reject.clone()]);
    Py::None
}
pub fn _household_reading(args: &[Py]) -> Py {
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
    let mut shifted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The same words said for one person, or None when the line says \"we\" nowhere.");
    shifted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&g_HOUSEHOLD_WORDS(), "get", &[mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), word.clone()])); } py_list(__out) })() };
    return if py_truthy(&Py::Bool(!py_eq(&shifted.clone(), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))))) { shifted.clone() } else { Py::None };
    Py::None
}
pub fn _other_reading(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut entries: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut reject: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reject"))) };
    let mut again: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut shifted: Py = Py::None;
    let mut tidied: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The same line read another way, when the line as typed reached nothing that answers.\n\n    Two readings sit beside the words a line was typed as. `plain` is the line with the words\n    that are near a key word repaired in place (\"wot shud i get me mum 4 xmas\", \"my scrambeld\n    eggs come out rubbrey\"); the understanding layer's `norm` is the line with its chat forms\n    and missing apostrophes put right, which reaches a key a repair cannot (\"what shood i cook\n    tonight\" carries no `what should i cook`). Both are tried only after the line as typed has\n    found nothing, or found only entries that turned it down, so an answer that stands today\n    cannot change.\n\n    A third reading is the line said for one person instead of a household, which is what the\n    keys are written in (`_household_reading`). It comes last, after the two repairs, because a\n    line that has a word to fix needs the fix more.\n    ");
    plain = mcall(&understood.clone(), "get", &[py_str("plain")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[plain.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = plain.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&plain.clone(), &clean.clone())) } } } }) {
    again = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&understood.clone(), "items", &[])) { let __unpk2866 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk2866[0].clone(); let mut value: Py = __unpk2866[1].clone(); if py_truthy(&Py::Bool(!py_eq(&key.clone(), &py_str("plain")))) {  __out.push((key.clone(), value.clone())); } } py_dict(__out) })() };
    let _ = py_setitem(&again.clone(), &py_str("clean"), plain.clone());
    let _ = py_setitem(&again.clone(), &py_str("words"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words_of(&[plain.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_list(__out) })() });
    found = find(&[again.clone(), entries.clone(), reject.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    return found.clone();
    }
    }
    tidied = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() };
    typed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() };
    if py_truthy(&{ let __b = tidied.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&tidied.clone(), &typed.clone())); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("norm")]) } } } }) {
    again = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&understood.clone(), "items", &[])) { let __unpk7234 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk7234[0].clone(); let mut value: Py = __unpk7234[1].clone(); if py_truthy(&Py::Bool(!py_eq(&key.clone(), &py_str("plain")))) {  __out.push((key.clone(), value.clone())); } } py_dict(__out) })() };
    let _ = py_setitem(&again.clone(), &py_str("clean"), mcall(&py_str(" "), "join", &[tidied.clone()]));
    let _ = py_setitem(&again.clone(), &py_str("words"), tidied.clone());
    let _ = py_setitem(&again.clone(), &py_str("norm"), py_list(vec![]));
    return find(&[again.clone(), entries.clone(), reject.clone()]);
    }
    shifted = _household_reading(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    if py_truthy(&shifted.clone()) {
    again = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&understood.clone(), "items", &[])) { let __unpk9970 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk9970[0].clone(); let mut value: Py = __unpk9970[1].clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("plain"), py_str("norm")]), &key.clone()))) {  __out.push((key.clone(), value.clone())); } } py_dict(__out) })() };
    let _ = py_setitem(&again.clone(), &py_str("clean"), mcall(&py_str(" "), "join", &[shifted.clone()]));
    let _ = py_setitem(&again.clone(), &py_str("words"), shifted.clone());
    return find(&[again.clone(), entries.clone(), reject.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _says_a_key_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut rarity: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rarity") { v.clone() }
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut ch: Py = Py::None;
    let mut part: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line itself names one specific key word of the entry.\n\n    This is narrower than `_touches()`: a phrase that only meets the entry as a phrase\n    (\"getting on\" in \"he is getting on a bit\") is not the line naming what the entry is\n    about, while \"bites\" in \"she says it hurts when she bites\" is. The distinction is what\n    keeps a comment about a dog's age from taking the dog-introduction answer, while a\n    follow-up about the biting itself still gets the biting answer.\n    ");
    if py_truthy(&Py::Bool(py_is(&rarity.clone(), &Py::None))) {
    rarity = py_dict(vec![]);
    }
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&_part_matches(&[part.clone(), forms.clone()]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    _u = __unpk3[0].clone();
    words = __unpk3[1].clone();
    _u = __unpk3[2].clone();
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&words.clone(), &Py::Int(0))) { let mut ch: Py = __item.clone(); __out.push(mcall(&ch.clone(), "isdigit", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { le(&mcall(&rarity.clone(), "get", &[geti(&words.clone(), &Py::Int(0)), Py::Int(0)]), &g_SPECIFIC()) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _touches(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut rarity: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rarity") { v.clone() }
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut ch: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the entry is reached from the line itself by a phrase or a rare word.\n\n    A carried word that the entry half-matches is not enough: \"do I need a cable for that\"\n    after moving phone photos to a laptop shares only \"need\" with the laptop-buying entry,\n    and \"need\" is in a dozen keys, so the entry is about something else.\n    ");
    if py_truthy(&Py::Bool(py_is(&rarity.clone(), &Py::None))) {
    rarity = py_dict(vec![]);
    }
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&_part_matches(&[part.clone(), forms.clone()]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 3, None);
    _u = __unpk3[0].clone();
    words = __unpk3[1].clone();
    _u = __unpk3[2].clone();
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_SAYS_NOTHING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&words.clone(), &Py::Int(0))) { let mut ch: Py = __item.clone(); __out.push(mcall(&ch.clone(), "isdigit", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { le(&mcall(&rarity.clone(), "get", &[geti(&words.clone(), &Py::Int(0)), Py::Int(0)]), &g_SPECIFIC()) } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _parts_across(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut carried_forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "carried_forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: carried_forms"))) };
    let mut forms: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut above: Py = Py::None;
    let mut index: Py = Py::None;
    let mut own: Py = Py::None;
    let mut part: Py = Py::None;
    let _ = py_str("True when the two lines between them complete the entry's key, one part from each.\n\n    A key written as (the thing) x (what is asked about it) is specific as a pair, and the two\n    halves can arrive a turn apart: \"its a garden party\" then \"what do i wear as a man in his\n    50s\" is the party-outfit card, the occasion in the line above and the asking in this one.\n    The own words alone touch that key only through \"wear\" and \"what\", which a hundred keys\n    share, so the entry looked like the earlier line's business rather than the person's. It is\n    the earlier line's business *and* this one's: different parts, one from each line.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    own = __unpk1[0].clone();
    above = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[{ let __b = call_builtin("getattr", &[item.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    part = __unpk3[1].clone();
    if py_truthy(&_part_matches(&[part.clone(), forms.clone()])) {
    let _ = mcall(&own.clone(), "add", &[index.clone()]);
    }
    if py_truthy(&_part_matches(&[part.clone(), carried_forms.clone()])) {
    let _ = mcall(&above.clone(), "add", &[index.clone()]);
    }
    }
    return { let __b = call_builtin("bool", &[sub(&own.clone(), &above.clone())]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[sub(&above.clone(), &own.clone())]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn key_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut found: Py = Py::None;
    let mut option: Py = Py::None;
    let mut options: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every word the entry says it is about, singular (\"cats\" and \"cat\" count as one).");
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[item.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    options = if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { py_tuple(vec![part.clone()]) } else { part.clone() };
    let __iter2 = py_iter_or_raise(&options.clone());
    for __item in __iter2 {
    option = __item.clone();
    let __iter3 = py_iter_or_raise(&mcall(&mcall(&call_builtin("str", &[option.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]));
    for __item in __iter3 {
    word = __item.clone();
    let _ = mcall(&found.clone(), "add", &[{ let __b = mcall(&word.clone(), "rstrip", &[py_str("s")]); if py_truthy(&__b) { __b } else { word.clone() } }]);
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut head: Py = Py::None;
    let _ = py_str("True when the entry is about the thing, not about a neighbour: a stew is not a steak.");
    head = geti(&{ let __b = mcall(&call_builtin("str", &[{ let __b = subject.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &neg(&Py::Int(1)));
    head = { let __b = mcall(&head.clone(), "rstrip", &[py_str("s")]); if py_truthy(&__b) { __b } else { head.clone() } };
    return { let __b = call_builtin("bool", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&key_words(&[item.clone()]), &head.clone())) } };
    Py::None
}
pub fn a_weight(args: &[Py]) -> Py {
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
    let mut number: Py = Py::None;
    let _ = py_str("True when the line gives a weight in kilos, grams or pounds.\n\n    A figure with a unit of mass is what makes a line a kitchen line rather than a question\n    about the thing the noun also names: \"how long does a 5kg turkey take\" is a bird for the\n    oven, and it was two full days in Istanbul; \"how long does a 5kg chicken take\" was the\n    lifespan of a pet. A card that answers a country, or how long an animal lives, is not\n    asked with a weight in it, so it stands down and the cooking row gets the line.\n    ");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("numbers")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[number.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WEIGHT_UNITS(), &mcall(&call_builtin("str", &[{ let __b = mcall(&number.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _refers_to_the_person(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line itself speaks of the person: \"he is getting on a bit\".");
    words = mcall(&understood.clone(), "get", &[py_str("norm")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[words.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    words = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PERSON_PRONOUNS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _says_how_the_speaker_handles_it(args: &[Py]) -> Py {
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
    let mut keys: Py = Py::None;
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is the speaker saying how they handle the thing before it.\n\n    \"my boss keeps dumping work on me last minute\" then \"i never say no\" is the saying-no\n    question, and the line on its own names the behaviour and not the situation: the words it\n    has are what the speaker does, and the situation was the line before. A statement that\n    names a subject of its own (\"i work nights\") is not this, so the cats-at-night trap does\n    not come back: nothing in it is about handling anything.\n    ");
    line = tokens(&[call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SPEAKER_ONLY(), &geti(&line.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    keys = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("keywords")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&keys.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_HANDLING_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _following_on(args: &[Py]) -> Py {
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
    let mut entries: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut forms: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut typed: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut by_domain: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "by_domain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: by_domain"))) };
    let mut every: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut known: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { py_tuple(vec![]) };
    let mut reject: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { Py::None };
    let mut allowed: Py = Py::None;
    let mut back: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_domain: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut carried_forms: Py = Py::None;
    let mut domain: Py = Py::None;
    let mut head: Py = Py::None;
    let mut item: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut leans_subject: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_subject: Py = Py::None;
    let mut other: Py = Py::None;
    let mut ranked: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut together: Py = Py::None;
    let mut together_typed: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The entry a follow-up reaches with the subject of the line it leans on, or None.\n\n    `reject` is honoured here too: the caller's veto is what stops an entry that declines the\n    line from being offered again, so a search that ignored it would loop for ever.\n    ");
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    words = if py_truthy(&call_builtin("isinstance", &[mcall(&leans.clone(), "get", &[py_str("words")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&leans.clone(), "get", &[py_str("words")]) } else { py_list(vec![]) };
    carried = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }, &Py::None, &g_CONTEXT_WORDS(), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&carried.clone()))) {
    return Py::None;
    }
    carried_forms = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&carried.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() };
    let _ = _read_typos(&[carried.clone(), carried_forms.clone(), every.clone(), known.clone()]);
    together = add(&carried_forms.clone(), &forms.clone());
    together_typed = add(&carried.clone(), &typed.clone());
    allowed = _topic_domains(&[understood.clone(), entries.clone()]);
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, Py::Int(0), py_str("")]), 3, None);
    best = __unpk1[0].clone();
    best_score = __unpk1[1].clone();
    best_domain = __unpk1[2].clone();
    seen = py_dict(vec![]);
    leans_subject = mcall(&leans.clone(), "get", &[py_str("subject")]);
    named_subject = if py_truthy(&call_builtin("isinstance", &[leans_subject.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { leans_subject.clone() } else { py_str("") };
    let __iter2 = py_iter_or_raise(&_reaching(&[entries.clone(), together.clone()]));
    for __item in __iter2 {
    item = __item.clone();
    domain = { let __b = call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&allowed.clone(), &domain.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&score(&[item.clone(), together.clone(), together_typed.clone(), seen.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&reject.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { py_call(&reject, &[item.clone()]) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&guard_passes(&[item.clone(), understood.clone()])))) {
    continue;
    }
    ranked = _rank(&[item.clone(), together.clone(), { let __b = mcall(&by_domain.clone(), "get", &[domain.clone()]); if py_truthy(&__b) { __b } else { every.clone() } }, seen.clone()]);
    if py_truthy(&{ let __b = ranked.clone(); if !py_truthy(&__b) { __b } else { { let __b = named_subject.clone(); if !py_truthy(&__b) { __b } else { about(&[item.clone(), named_subject.clone()]) } } } }) {
    let __aug3 = add(&ranked.clone(), &Py::Int(100));
    ranked = __aug3;
    }
    if py_truthy(&gt(&ranked.clone(), &best_score.clone())) {
    let __unpk4 = py_unpack(&py_tuple(vec![item.clone(), ranked.clone(), domain.clone()]), 3, None);
    best = __unpk4[0].clone();
    best_score = __unpk4[1].clone();
    best_domain = __unpk4[2].clone();
    }
    }
    if py_truthy(&_only_asking_about_it(&[understood.clone(), leans.clone()])) {
    back = _the_best_entry(&[leans.clone(), entries.clone(), reject.clone(), Py::Bool(false)]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&back.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&best.clone(), &back.clone())); if !py_truthy(&__b) { __b } else { _parts_across(&[best.clone(), carried_forms.clone(), forms.clone()]) } } } })) } }) {
    let __unpk5 = py_unpack(&py_tuple(vec![back.clone(), { let __b = call_builtin("getattr", &[back.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]), 2, None);
    best = __unpk5[0].clone();
    best_domain = __unpk5[1].clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_touches(&[best.clone(), forms.clone(), { let __b = mcall(&by_domain.clone(), "get", &[best_domain.clone()]); if py_truthy(&__b) { __b } else { every.clone() } }]))) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_parts_across(&[best.clone(), carried_forms.clone(), forms.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_only_asking_about_it(&[understood.clone(), leans.clone()]))) } }) {
    return Py::None;
    }
    }
    said = { let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _says_a_key_word(&[best.clone(), forms.clone(), { let __b = mcall(&by_domain.clone(), "get", &[best_domain.clone()]); if py_truthy(&__b) { __b } else { every.clone() } }]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_reaches(&[best.clone(), carried_forms.clone()]))) } }) {
    return Py::None;
    }
    subject = mcall(&leans.clone(), "get", &[py_str("subject")]);
    named = { let __b = call_builtin("isinstance", &[subject.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&about(&[best.clone(), subject.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&leans.clone(), "get", &[py_str("subject_type")]), &py_str("person"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("question"), py_str("request")]), &mcall(&understood.clone(), "get", &[py_str("kind")]))); if !py_truthy(&__b) { __b } else { { let __b = _refers_to_the_person(&[understood.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } } } } } } } } } }) {
    head = call_builtin("set", &[_forms(&[geti(&mcall(&call_builtin("str", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[]), &neg(&Py::Int(1)))])]).unwrap_or_else(|e| exc::raise(e));
    other = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&carried_forms.clone()) { let mut forms: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&bitand(&forms.clone(), &head.clone())))) {  __out.push(forms.clone()); } } py_list(__out) })() };
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter6 = py_iter_or_raise(&key_words(&[best.clone()]));
    for __item in __iter6 {
    word = __item.clone();
    let __aug7 = bitor(&words.clone(), &_forms(&[word.clone()]));
    words = __aug7;
    }
    if py_truthy(&{ let __b = other.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_reaches(&[best.clone(), other.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&other.clone()) { let mut forms: Py = __item.clone(); __out.push(bitand(&forms.clone(), &words.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&named.clone())) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_touches(&[best.clone(), carried_forms.clone(), { let __b = mcall(&by_domain.clone(), "get", &[best_domain.clone()]); if py_truthy(&__b) { __b } else { every.clone() } }])))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&best.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_in_topic(&[best.clone(), understood.clone()]))) } }) {
    return Py::None;
    }
    return best.clone();
    Py::None
}
pub fn asks_what_to_wear(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks what to put on, so the answer is a clothes one.\n\n    Read from the line as typed: the reading's own words carry the topic along with it, and the\n    carried words are what made the conversation about the garden rather than about the party.\n    ");
    said = py_str("");
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    said = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    words = mcall(&understood.clone(), "get", &[py_str("words")]);
    said = if py_truthy(&call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]) } else { py_str("") };
    }
    }
    return { let __b = call_builtin("bool", &[mcall(&g__WEAR_ASK(), "search", &[mcall(&py_str(" "), "join", &[tokens(&[said.clone()])])])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { a_colour_is_worn(&[understood.clone()]) } };
    Py::None
}
pub fn _topic_domains(args: &[Py]) -> Py {
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
    let mut entries: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { Py::None };
    let mut allowed: Py = Py::None;
    let mut answered: Py = Py::None;
    let mut found: Py = Py::None;
    let mut item: Py = Py::None;
    let mut judged: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut name: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut topics: Py = Py::None;
    let _ = py_str("The knowledge domains that answer a topic the line is about, or None for all of them.\n\n    A follow-up borrows the earlier line's words, so it can otherwise land in a domain the\n    conversation never touched: \"what can i take for it\" and \"how long before i should see a\n    doctor\" reach the entry for asking for time off, because that entry lists \"doctor\" too.\n    Understanding already knows what the conversation is about, so a fact from a domain that\n    answers none of those topics is not the same conversation's fact.\n\n    None means \"no opinion\": the line names no topic, or every domain declares nothing, and\n    then every entry is a candidate. Working it out before the search also keeps a follow-up\n    from scoring a thousand entries in domains the conversation never touched.\n    ");
    topics = py_list(vec![]);
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    found = mcall(&understood.clone(), "get", &[py_str("topics")]);
    topics = if py_truthy(&call_builtin("isinstance", &[found.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("list", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { py_list(vec![]) };
    let _ = mcall(&topics.clone(), "append", &[mcall(&understood.clone(), "get", &[py_str("topic")])]);
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&topics.clone(), "append", &[mcall(&leans.clone(), "get", &[py_str("topic")])]);
    }
    }
    topics = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&topics.clone()) { let mut topic: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[topic.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { topic.clone() } }) {  __out.push(topic.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&topics.clone()))) {
    return Py::None;
    }
    let mut by_topic: Py = py_func("by_topic", |args: &[Py]| crate::gen::knowledge::by_topic(args));
    judged = py_call(&by_topic, &[]);
    allowed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&judged.clone(), "items", &[])) { let __unpk7392 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk7392[0].clone(); let mut answered: Py = __unpk7392[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answered.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&topics.clone()) { let mut topic: Py = __item.clone(); __out.push(Py::Bool(py_contains(&answered.clone(), &topic.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(name.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_is(&entries.clone(), &Py::None))) {
    let __aug1 = bitand(&allowed.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entries.clone()) { let mut item: Py = __item.clone(); __out.push({ let __b = call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }); } py_set(__out) })() });
    allowed = __aug1;
    }
    if py_truthy(&{ let __b = allowed.clone(); if !py_truthy(&__b) { __b } else { asks_what_to_wear(&[understood.clone()]) } }) {
    let _ = mcall(&allowed.clone(), "add", &[py_str("clothing")]);
    }
    return if py_truthy(&allowed.clone()) { allowed.clone() } else { Py::None };
    Py::None
}
pub fn _in_topic(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut allowed: Py = Py::None;
    let _ = py_str("Whether `item`'s own domain is one that answers lines about a topic of the line.");
    allowed = _topic_domains(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&allowed.clone(), &Py::None))) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&allowed.clone(), &{ let __b = call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }));
    Py::None
}
pub fn about_the_topic(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("True when the entry's domain answers a topic this reading is about.\n\n    The same test a follow-up makes before it takes a fact: a bare ask (\"what can i do\") leans\n    on the words of the line before it, and one of those words sits in other domains too, so\n    \"my kids wont listen to me\" and then \"what can i do\" reached the music entry off the word\n    \"listen\". A fact from a part of the knowledge base the conversation never touched is not\n    the same conversation's fact.\n    ");
    return _in_topic(&[item.clone(), understood.clone()]);
    Py::None
}
pub fn _is_a_person(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut ent: Py = Py::None;
    let _ = py_str("True when the line named this word as a person rather than as a thing.");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    ent = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[ent.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[mcall(&ent.clone(), "get", &[py_str("text"), py_str("")])]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return Py::Bool(py_eq(&mcall(&ent.clone(), "get", &[py_str("type")]), &py_str("person")));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_what_is_missing(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut focus: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the half of the line before the ask says something is not there.\n\n    A clause in front of the question is usually a scene, and the ask is about the thing after\n    it. But a clause that says a thing is missing is a condition on the ask, not a scene: \"i\n    havent got a blender, can i still make soup\" asks how to make soup without one, so the card\n    keyed on the blender is the answer and the thing the ask names is not what to answer about.\n    ");
    focus = call_builtin("list", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("focus")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clauses")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&call_builtin("list", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &focus.clone()))) {
    continue;
    }
    text = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clause.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]);
    if py_truthy(&mcall(&g_MISSING_WORDS(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _reaches(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut forms: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "forms") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: forms"))) };
    let mut part: Py = Py::None;
    let _ = py_str("True when the entry matches at least one word of the line searched for it.\n\n    The earlier line has to be the reason the entry is on the table. When it is not, the words\n    carried over are a wish rather than a subject, and the entry answers something the person\n    did not say: \"it clicks once and then nothing\" after \"my van wont start\" reached the\n    brake-pedal entry, which shares no word with the van at all.\n    ");
    let __iter1 = py_iter_or_raise(&gattr(&item.clone(), "key"));
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&_part_matches(&[part.clone(), forms.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn names_its_own_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut clean: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut key: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the entry's own subject is one this line names for itself.\n\n    The first key part of an entry is its subject (the rest are the ask, the property or the\n    setting), so this asks the one question a topic test cannot: does the line say the thing the\n    entry is about? \"why do we get allergies\" says \"allergies\", which is the subject of the\n    allergy entry, whatever the rest of the conversation is on. A domain test on its own cannot\n    see that, because an entry lives in the domain that was written for it, and the domain that\n    explains allergies is not the one a chat about a pet is in.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    forms = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tokens(&[call_builtin("str", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e))])) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() };
    key = { let __b = call_builtin("getattr", &[item.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    return { let __b = call_builtin("bool", &[forms.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[_part_matches(&[geti(&key.clone(), &Py::Int(0)), forms.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _wrapped_subject(args: &[Py]) -> Py {
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
    let mut norm_text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm_text") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The subject an advice ask wraps, or None when the line is not one.\n\n    The wrapper is looked for in the line as typed and again in its ordinary words, because\n    \"any tips 4 a hangover\" only says \"for\" once the chat-speak is undone.\n    ");
    plain = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![clean.clone(), norm_text.clone(), plain.clone()]));
    for __item in __iter1 {
    text = __item.clone();
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "replace", &[py_str(" | "), py_str(" ")]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&g__WRAPPED());
    for __item in __iter2 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    subject = mcall(&mcall(&found.clone(), "group", &[py_str("subject")]), "strip", &[py_str(" .,!?;:")]);
    if py_truthy(&Py::Bool(py_contains(&g__NOT_A_THING(), &mcall(&subject.clone(), "lower", &[])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[subject.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g__NOT_A_SUBJECT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return subject.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_ask_it_wraps(args: &[Py]) -> Py {
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
    let mut subject: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut whole: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: whole"))) };
    let mut entries: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut by_domain: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "by_domain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: by_domain"))) };
    let mut every: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut reject: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reject"))) };
    let mut again: Py = Py::None;
    let mut alone: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The entry the subject inside an advice ask names, or None.\n\n    The subject read on its own is what the line is about, so it answers when it can: \"what\n    should i do about slugs\" was the tablespoons card, because \"slug\" is also how a cook says a\n    small splash. When the subject on its own reaches nothing, the reading adds nothing and the\n    line keeps whatever the whole of it found (\"my hay fever is awful this week, what actually\n    works\" is a line whose subject is not a thing to look up, and its answer stands).\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tokens(&[subject.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__SUBJECT_POINTS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    alone = _the_subject_alone(&[mcall(&py_str(" "), "join", &[words.clone()]), entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    if py_truthy(&Py::Bool(!py_is(&alone.clone(), &Py::None))) {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&whole.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_FRAME_IDS(), &call_builtin("getattr", &[whole.clone(), py_str("id"), py_str("")]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    return alone.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_is(&whole.clone(), &Py::None))) {
    return Py::None;
    }
    again = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&understood.clone(), "items", &[])) { let __unpk4086 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk4086[0].clone(); let mut value: Py = __unpk4086[1].clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("plain"), py_str("norm"), py_str("norm_text")]), &key.clone()))) {  __out.push((key.clone(), value.clone())); } } py_dict(__out) })() };
    let _ = py_setitem(&again.clone(), &py_str("clean"), subject.clone());
    let _ = py_setitem(&again.clone(), &py_str("words"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words_of(&[subject.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_list(__out) })() });
    found = find(&[again.clone(), entries.clone(), reject.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    return found.clone();
    }
    return _the_subject_alone(&[subject.clone(), entries.clone(), by_domain.clone(), every.clone(), reject.clone()]);
    Py::None
}
pub fn names_one_thing(args: &[Py]) -> Py {
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
    let mut every: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut counts: Py = Py::None;
    let mut form: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the words of a subject name one thing rather than a dozen.\n\n    A word a dozen entries use is a word about other things as well: \"kettle\" is a scaled one, a\n    broken one and the price of one, and \"what can i do about my kettle\" asks about whichever it\n    is, so the ask around it says nothing and the line is read by its words alone. A word one or\n    two entries use - slugs, limescale, snoring - names that thing, so being asked about it is\n    the second signal the one word cannot give.\n    ");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    counts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_forms(&[word.clone()])) { let mut form: Py = __item.clone(); __out.push({ let __b = mcall(&every.clone(), "get", &[form.clone()]); if py_truthy(&__b) { __b } else { Py::Int(0) } }); } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&counts.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("max", &[counts.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_COMMON_WORD()) } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_subject_alone(args: &[Py]) -> Py {
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
    let mut entries: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entries") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entries"))) };
    let mut by_domain: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "by_domain") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: by_domain"))) };
    let mut every: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "every") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: every"))) };
    let mut reject: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reject") { v.clone() }
    else { Py::None };
    let mut forms: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The entry a one-noun subject inside an advice ask reaches, or None.\n\n    The subject is read with the ask around it kept as a signal, which is what `framed` means:\n    \"what should i do about the condensation\" names one thing and asks about it, so the entry\n    that owns that thing answers, while every entry not listed in ASK_FRAME_IDS is still read by\n    the ordinary two-word rule. A subject with more words than that is read like any other line\n    and finds its answer without this.\n    ");
    words = tokens(&[subject.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    forms = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_forms(&[word.clone()])); } py_list(__out) })() };
    typed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("clean"), subject.clone()), (py_str("words"), Py::None), (py_str("kind"), py_str("question"))])); __args.push(words.clone()); __args.push(subject.clone()); __args.push(forms.clone()); __args.push(typed.clone()); __args.push(entries.clone()); __args.push(by_domain.clone()); __args.push(every.clone()); __args.push(reject.clone()); __kws.extend(vec![("framed".to_string(), names_one_thing(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__SUBJECT_POINTS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, every.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _best(args))(&__args) };
    Py::None
}
pub fn says_it_has_gone_off(args: &[Py]) -> Py {
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
    let mut sign: Py = Py::None;
    let _ = py_str("True when the line says the food has gone off, or asks whether it has.\n\n    `clean` is the line as the reading hands it over. The words are matched inside the line\n    because \"smells\" and \"gone off\" are said as part of a longer phrase (\"it smells a bit\",\n    \"the mince has gone off\"), and a word-by-word test would miss them.\n    ");
    low = modulo(&py_str(" %s "), &mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GONE_OFF_SIGNS()) { let mut sign: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&low.clone(), &modulo(&py_str(" %s "), &sign.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &modulo(&py_str(" %s"), &sign.clone()))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn said_in_this_line(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut carried: Py = Py::None;
    let mut focus: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut low: Py = Py::None;
    let mut text: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line's own words, with any earlier line put back out of it.\n\n    A follow-up is read with the line before it in front of it (\"can i freeze it\" after \"im\n    making a chilli tonight\" reads as \"im making a chilli tonight can i freeze it\"), which is what\n    lets \"how long does it need\" reach the dish at all. The cost is that a row keyed on a word\n    from the earlier line answers a line that never said it: the cooked pot's question was handed\n    to the row about freezing the *vegetable*. A row that has to be about what this line names\n    reads the line through here instead of `clean`.\n\n    Two shapes carry an earlier line: `leans_on` (the reading says which line it leans on) and\n    `said_now` (a module wrote the subject of the conversation into the line and kept the typed\n    line beside it).\n    ");
    if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    typed = mcall(&memory.clone(), "get", &[py_str("asked_now")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[typed.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&typed.clone(), "strip", &[]) } }) {
    return mcall(&typed.clone(), "strip", &[]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    typed = mcall(&understood.clone(), "get", &[py_str("said_now")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[typed.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&typed.clone(), "strip", &[]) } }) {
    return mcall(&typed.clone(), "strip", &[]);
    }
    focus = mcall(&understood.clone(), "get", &[py_str("focus")]);
    if py_truthy(&{ let __b = mcall(&understood.clone(), "get", &[py_str("setup")]); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[focus.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { focus.clone() } } } }) {
    return mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&focus.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "strip", &[]);
    }
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    carried = if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&leans.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&carried.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&text.clone(), "lower", &[]), mcall(&carried.clone(), "lower", &[])]), 2, None);
    low = __unpk1[0].clone();
    carried = __unpk1[1].clone();
    if py_truthy(&Py::Bool(py_contains(&low.clone(), &carried.clone()))) {
    text = mcall(&low.clone(), "replace", &[carried.clone(), py_str(" ")]);
    }
    }
    return mcall(&text.clone(), "strip", &[]);
    Py::None
}
pub fn Entry____init__(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut self_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "self") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: self"))) };
    let mut id: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "id") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: id"))) };
    let mut key: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut reply: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut avoid: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "avoid") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bias: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bias") { v.clone() }
    else { Py::Int(0) };
    let mut ask: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { py_tuple(vec![]) };
    let mut slips: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "slips") { v.clone() }
    else { py_tuple(vec![]) };
    let mut fault: Py = if __pos.len() > 8 { __pos[8].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fault") { v.clone() }
    else { Py::Bool(false) };
    let mut report: Py = if __pos.len() > 9 { __pos[9].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "report") { v.clone() }
    else { Py::Bool(false) };
    let mut guard: Py = if __pos.len() > 10 { __pos[10].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "guard") { v.clone() }
    else { Py::None };
    let mut shape: Py = if __pos.len() > 11 { __pos[11].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shape") { v.clone() }
    else { py_tuple(vec![]) };
    let mut explain: Py = if __pos.len() > 12 { __pos[12].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "explain") { v.clone() }
    else { Py::Bool(false) };
    let mut lead: Py = if __pos.len() > 13 { __pos[13].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lead") { v.clone() }
    else { py_str("") };
    let mut outcomes: Py = if __pos.len() > 14 { __pos[14].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "outcomes") { v.clone() }
    else { py_tuple(vec![]) };
    let _ = py_setattr(&self_.clone(), "id", id.clone());
    let _ = py_setattr(&self_.clone(), "key", call_builtin("tuple", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setattr(&self_.clone(), "reply", reply.clone());
    let _ = py_setattr(&self_.clone(), "avoid", call_builtin("tuple", &[avoid.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setattr(&self_.clone(), "bias", bias.clone());
    let _ = py_setattr(&self_.clone(), "ask", call_builtin("tuple", &[ask.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setattr(&self_.clone(), "slips", call_builtin("tuple", &[slips.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setattr(&self_.clone(), "domain", py_str(""));
    let _ = py_setattr(&self_.clone(), "fault", fault.clone());
    let _ = py_setattr(&self_.clone(), "report", report.clone());
    let _ = py_setattr(&self_.clone(), "guard", guard.clone());
    let _ = py_setattr(&self_.clone(), "shape", call_builtin("tuple", &[shape.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setattr(&self_.clone(), "explain", explain.clone());
    let _ = py_setattr(&self_.clone(), "lead", lead.clone());
    let _ = py_setattr(&self_.clone(), "outcomes", call_builtin("tuple", &[outcomes.clone()]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn Entry____repr__(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut self_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "self") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: self"))) };
    return { let mut __s = String::new(); __s.push_str("<Entry "); __s.push_str(&fstr(&gattr(&self_.clone(), "id"), -1, "")); __s.push_str(">"); py_string(__s) };
    Py::None
}
pub fn Entry(args: &[Py]) -> Py {
    ensure_init();
    let inst = Py::Instance(Rc::new(PyInstance { class_name: "Entry".to_string(), attrs: Rc::new(RefCell::new(IndexMap::new())) }));
    let mut full = vec![inst.clone()]; full.extend_from_slice(args);
    Entry____init__(&full);
    inst
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.lookup".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AFTER_A_COLOUR_STANDS" => Some(g_AFTER_A_COLOUR_STANDS()),
            "AN_ILLNESS" => Some(g_AN_ILLNESS()),
            "APPLIANCE_NAMES" => Some(g_APPLIANCE_NAMES()),
            "ASKING_FIRST" => Some(g_ASKING_FIRST()),
            "ASKING_MODALS" => Some(g_ASKING_MODALS()),
            "ASKING_SUBJECTS" => Some(g_ASKING_SUBJECTS()),
            "ASKS_FOR_THE_HANDS" => Some(g_ASKS_FOR_THE_HANDS()),
            "ASKS_WHAT_IT_SAYS" => Some(g_ASKS_WHAT_IT_SAYS()),
            "ASK_ABOUT" => Some(g_ASK_ABOUT()),
            "ASK_FRAME" => Some(g_ASK_FRAME()),
            "ASK_FRAME_IDS" => Some(g_ASK_FRAME_IDS()),
            "ASK_FRAME_WORDS" => Some(g_ASK_FRAME_WORDS()),
            "ASK_NAME_CAP" => Some(g_ASK_NAME_CAP()),
            "ASK_PEOPLE" => Some(g_ASK_PEOPLE()),
            "ASK_PERSON_POSSESSIVES" => Some(g_ASK_PERSON_POSSESSIVES()),
            "ASK_PERSON_PRONOUNS" => Some(g_ASK_PERSON_PRONOUNS()),
            "ASK_PHRASE_WORDS" => Some(g_ASK_PHRASE_WORDS()),
            "ASK_PLAIN_CAP" => Some(g_ASK_PLAIN_CAP()),
            "AS_A_KIND" => Some(g_AS_A_KIND()),
            "A_CHILD_GETTING_OLDER" => Some(g_A_CHILD_GETTING_OLDER()),
            "A_CHILD_WORD_BY_ITSELF" => Some(g_A_CHILD_WORD_BY_ITSELF()),
            "A_COLOUR_ASKED" => Some(g_A_COLOUR_ASKED()),
            "A_COLOUR_USED" => Some(g_A_COLOUR_USED()),
            "A_DEVICE_THAT_SLEEPS" => Some(g_A_DEVICE_THAT_SLEEPS()),
            "A_DRESS_CODE_AFTER" => Some(g_A_DRESS_CODE_AFTER()),
            "A_GOOD_SMELL" => Some(g_A_GOOD_SMELL()),
            "A_HAND_OR_EYE_DOES_IT" => Some(g_A_HAND_OR_EYE_DOES_IT()),
            "A_HOURS_ASK" => Some(g_A_HOURS_ASK()),
            "A_HOUSE_WARMING_UP" => Some(g_A_HOUSE_WARMING_UP()),
            "A_LENGTH_WORD" => Some(g_A_LENGTH_WORD()),
            "A_SAYS_NOTHING" => Some(g_A_SAYS_NOTHING()),
            "A_STAND_IN" => Some(g_A_STAND_IN()),
            "A_STATE_AS_A_NAME_END" => Some(g_A_STATE_AS_A_NAME_END()),
            "A_STATE_IN_THE_PHRASE" => Some(g_A_STATE_IN_THE_PHRASE()),
            "A_TEEN" => Some(g_A_TEEN()),
            "A_WEATHER_ASK" => Some(g_A_WEATHER_ASK()),
            "A_WHAT_TO_WEAR" => Some(g_A_WHAT_TO_WEAR()),
            "BACK_OF_THE_HOUSE" => Some(g_BACK_OF_THE_HOUSE()),
            "BARE_WEIGHT" => Some(g_BARE_WEIGHT()),
            "BEFORE_OFF" => Some(g_BEFORE_OFF()),
            "BODY_WORDS" => Some(g_BODY_WORDS()),
            "CARER_FRAME" => Some(g_CARER_FRAME()),
            "CHILD_AFTER_A_SIZE" => Some(g_CHILD_AFTER_A_SIZE()),
            "CHILD_BY_PLACE" => Some(g_CHILD_BY_PLACE()),
            "CHILD_PLACE_WORDS" => Some(g_CHILD_PLACE_WORDS()),
            "CHILD_SIZE_WORDS" => Some(g_CHILD_SIZE_WORDS()),
            "COLOUR_ASKED_WORDS" => Some(g_COLOUR_ASKED_WORDS()),
            "COLOUR_WORDS" => Some(g_COLOUR_WORDS()),
            "COMMON_WORD" => Some(g_COMMON_WORD()),
            "CONTAINERS" => Some(g_CONTAINERS()),
            "CONTEXT_WORDS" => Some(g_CONTEXT_WORDS()),
            "COOKING_AS_A_NAME" => Some(g_COOKING_AS_A_NAME()),
            "COUNTED_INTO_A_THING" => Some(g_COUNTED_INTO_A_THING()),
            "DAY_WORDS" => Some(g_DAY_WORDS()),
            "DEAD_AS_VERY_AFTER" => Some(g_DEAD_AS_VERY_AFTER()),
            "DENIERS" => Some(g_DENIERS()),
            "DENY_BETWEEN" => Some(g_DENY_BETWEEN()),
            "DENY_SAID_OF" => Some(g_DENY_SAID_OF()),
            "DISH_ASK" => Some(g_DISH_ASK()),
            "ENDINGS" => Some(g_ENDINGS()),
            "EVERYDAY_CHILD" => Some(g_EVERYDAY_CHILD()),
            "EXPLAIN_FILLER" => Some(g_EXPLAIN_FILLER()),
            "EXPLAIN_LEADS" => Some(g_EXPLAIN_LEADS()),
            "Entry" => Some(py_func("Entry", |args: &[Py]| Entry(args))),
            "FEARED_THINGS" => Some(g_FEARED_THINGS()),
            "FEELING_ABOUT" => Some(g_FEELING_ABOUT()),
            "FEELING_ABOUT_A_THING" => Some(g_FEELING_ABOUT_A_THING()),
            "FEELING_DOMAINS" => Some(g_FEELING_DOMAINS()),
            "FEELING_ENDS" => Some(g_FEELING_ENDS()),
            "FEELING_FAULT_WORDS" => Some(g_FEELING_FAULT_WORDS()),
            "FEELING_NAMES_NOTHING" => Some(g_FEELING_NAMES_NOTHING()),
            "FEELING_SPEAKER" => Some(g_FEELING_SPEAKER()),
            "FEELING_WORDS" => Some(g_FEELING_WORDS()),
            "FOLLOW_ON_WORDS" => Some(g_FOLLOW_ON_WORDS()),
            "FOOD_NAMED" => Some(g_FOOD_NAMED()),
            "FOOD_SUBJECT" => Some(g_FOOD_SUBJECT()),
            "FRAME_ENDS" => Some(g_FRAME_ENDS()),
            "FRAME_PRONOUNS" => Some(g_FRAME_PRONOUNS()),
            "FRAME_WORDS" => Some(g_FRAME_WORDS()),
            "GENERAL_BODY" => Some(g_GENERAL_BODY()),
            "GOES_IN" => Some(g_GOES_IN()),
            "GONE_INTENSIFIER" => Some(g_GONE_INTENSIFIER()),
            "GONE_OFF_SIGNS" => Some(g_GONE_OFF_SIGNS()),
            "GONE_STATE" => Some(g_GONE_STATE()),
            "GRAMMAR_ONLY" => Some(g_GRAMMAR_ONLY()),
            "HANDLING_WORDS" => Some(g_HANDLING_WORDS()),
            "HAPPENING_AGAIN" => Some(g_HAPPENING_AGAIN()),
            "HAS_GONE" => Some(g_HAS_GONE()),
            "HOUSEHOLD_WORDS" => Some(g_HOUSEHOLD_WORDS()),
            "HOW_MANY_DAYS" => Some(g_HOW_MANY_DAYS()),
            "HOW_OFTEN_WORDS" => Some(g_HOW_OFTEN_WORDS()),
            "IMPROVE" => Some(g_IMPROVE()),
            "IRREGULAR_PLURALS" => Some(g_IRREGULAR_PLURALS()),
            "JOB_GUARD" => Some(g_JOB_GUARD()),
            "KEEPING_WORDS" => Some(g_KEEPING_WORDS()),
            "KEEPS_OR_STORES" => Some(g_KEEPS_OR_STORES()),
            "KEPT_FOODS" => Some(g_KEPT_FOODS()),
            "KINDS_ASKS_ME" => Some(g_KINDS_ASKS_ME()),
            "KINDS_ASKS_WITH_A_VERB" => Some(g_KINDS_ASKS_WITH_A_VERB()),
            "KINDS_FOLLOW_UP_WORDS" => Some(g_KINDS_FOLLOW_UP_WORDS()),
            "KINDS_NAMED_A_USE" => Some(g_KINDS_NAMED_A_USE()),
            "KINDS_SAYS_HOW_IT_IS" => Some(g_KINDS_SAYS_HOW_IT_IS()),
            "KINDS_SHAPE" => Some(g_KINDS_SHAPE()),
            "KINDS_WORDS" => Some(g_KINDS_WORDS()),
            "MACHINE_SOUNDS" => Some(g_MACHINE_SOUNDS()),
            "MIN_FORM" => Some(g_MIN_FORM()),
            "MIN_SCORE" => Some(g_MIN_SCORE()),
            "MIN_TYPO_LENGTH" => Some(g_MIN_TYPO_LENGTH()),
            "MISSING_WORDS" => Some(g_MISSING_WORDS()),
            "MODAL_PHRASES" => Some(g_MODAL_PHRASES()),
            "NAME_ALONE_DOMAINS" => Some(g_NAME_ALONE_DOMAINS()),
            "NOT_A_BASE" => Some(g_NOT_A_BASE()),
            "NOT_A_CHILD_SAID" => Some(g_NOT_A_CHILD_SAID()),
            "NOT_A_NAME_ALONE" => Some(g_NOT_A_NAME_ALONE()),
            "NOT_A_NOSE" => Some(g_NOT_A_NOSE()),
            "NOT_A_WORD_WITH_AN_S" => Some(g_NOT_A_WORD_WITH_AN_S()),
            "NOT_THE_ADJECTIVE" => Some(g_NOT_THE_ADJECTIVE()),
            "NOT_THE_DOUBLED_STEM" => Some(g_NOT_THE_DOUBLED_STEM()),
            "NOT_THE_STEM" => Some(g_NOT_THE_STEM()),
            "NOT_THE_STEM_WITH_E" => Some(g_NOT_THE_STEM_WITH_E()),
            "NOT_TIRED_AFTER_ALL" => Some(g_NOT_TIRED_AFTER_ALL()),
            "OCCASION" => Some(g_OCCASION()),
            "OFF_SAID_OF_FOOD" => Some(g_OFF_SAID_OF_FOOD()),
            "OFF_STATE_ASKS" => Some(g_OFF_STATE_ASKS()),
            "ORIGIN_ASK" => Some(g_ORIGIN_ASK()),
            "OWN_THING_ON_THEIR_SIDE" => Some(g_OWN_THING_ON_THEIR_SIDE()),
            "PACK_DATE" => Some(g_PACK_DATE()),
            "PART_INSIDE" => Some(g_PART_INSIDE()),
            "PEOPLE_FEMALE" => Some(g_PEOPLE_FEMALE()),
            "PEOPLE_MALE" => Some(g_PEOPLE_MALE()),
            "PERSON_PRONOUNS" => Some(g_PERSON_PRONOUNS()),
            "PHRASE_WEIGHT" => Some(g_PHRASE_WEIGHT()),
            "PLACES_AFTER_BACK" => Some(g_PLACES_AFTER_BACK()),
            "PLAIN_MARGIN" => Some(g_PLAIN_MARGIN()),
            "PLAN_WORDS" => Some(g_PLAN_WORDS()),
            "PLURAL_ONLY_PHRASES" => Some(g_PLURAL_ONLY_PHRASES()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "PRESENT_ASK" => Some(g_PRESENT_ASK()),
            "PRICE_COMPARED" => Some(g_PRICE_COMPARED()),
            "REPORT_FILLER" => Some(g_REPORT_FILLER()),
            "REPORT_LEADS" => Some(g_REPORT_LEADS()),
            "REPORT_PERSON" => Some(g_REPORT_PERSON()),
            "RETURNED_AFTER" => Some(g_RETURNED_AFTER()),
            "RING_AS_A_CALL" => Some(g_RING_AS_A_CALL()),
            "SAID_LEADS" => Some(g_SAID_LEADS()),
            "SANDING_JOB" => Some(g_SANDING_JOB()),
            "SHORT_OF_SLEEP" => Some(g_SHORT_OF_SLEEP()),
            "SLIPS" => Some(g_SLIPS()),
            "SMELL_VERBS" => Some(g_SMELL_VERBS()),
            "SPEAKER_ONLY" => Some(g_SPEAKER_ONLY()),
            "SPECIFIC" => Some(g_SPECIFIC()),
            "SPECIFIC_PARTS" => Some(g_SPECIFIC_PARTS()),
            "STATES_AFTER" => Some(g_STATES_AFTER()),
            "STOPPED_WORKING" => Some(g_STOPPED_WORKING()),
            "SUBSTITUTE_WORDS" => Some(g_SUBSTITUTE_WORDS()),
            "THEIR_OWN_HANDS_THINGS" => Some(g_THEIR_OWN_HANDS_THINGS()),
            "THE_BACK_ITSELF" => Some(g_THE_BACK_ITSELF()),
            "THE_FAULT" => Some(g_THE_FAULT()),
            "THE_SAME_OCCASION" => Some(g_THE_SAME_OCCASION()),
            "TOOTH_OR_GUM" => Some(g_TOOTH_OR_GUM()),
            "TOOTH_OUT" => Some(g_TOOTH_OUT()),
            "TWO_OF_US" => Some(g_TWO_OF_US()),
            "WANTING" => Some(g_WANTING()),
            "WANT_FILLER" => Some(g_WANT_FILLER()),
            "WANT_KNOWING" => Some(g_WANT_KNOWING()),
            "WEARING" => Some(g_WEARING()),
            "WEIGHT_UNITS" => Some(g_WEIGHT_UNITS()),
            "WHEN_WORDS" => Some(g_WHEN_WORDS()),
            "WHERE_PART_WORDS" => Some(g_WHERE_PART_WORDS()),
            "WHERE_PHRASES" => Some(g_WHERE_PHRASES()),
            "WORD" => Some(g_WORD()),
            "WORD_WEIGHT" => Some(g_WORD_WEIGHT()),
            "WORN_AFTER_A_COLOUR" => Some(g_WORN_AFTER_A_COLOUR()),
            "_ABOUT_THE_CHAT" => Some(g__ABOUT_THE_CHAT()),
            "_AFFORD_THE_TRIP" => Some(g__AFFORD_THE_TRIP()),
            "_ARTICLE_FORMS" => Some(g__ARTICLE_FORMS()),
            "_ARTICLE_WORDS" => Some(g__ARTICLE_WORDS()),
            "_ASK_ABOUT_SOMEONE" => Some(g__ASK_ABOUT_SOMEONE()),
            "_ASK_FRAME" => Some(g__ASK_FRAME()),
            "_ASK_FRAME_FORMS" => Some(g__ASK_FRAME_FORMS()),
            "_ASK_PHRASE_MASK" => Some(g__ASK_PHRASE_MASK()),
            "_A_BAKE_OR_A_PAN" => Some(g__A_BAKE_OR_A_PAN()),
            "_A_BREATH" => Some(g__A_BREATH()),
            "_A_KEEP_AND_A_LENGTH" => Some(g__A_KEEP_AND_A_LENGTH()),
            "_A_PERSON" => Some(g__A_PERSON()),
            "_A_PERSON_WHO_SLEEPS" => Some(g__A_PERSON_WHO_SLEEPS()),
            "_A_PLACE_ON_THEM" => Some(g__A_PLACE_ON_THEM()),
            "_A_THING_JUST_BEFORE" => Some(g__A_THING_JUST_BEFORE()),
            "_A_THING_POINTED_AT" => Some(g__A_THING_POINTED_AT()),
            "_A_THING_SAID_IS" => Some(g__A_THING_SAID_IS()),
            "_A_YEARS_OLD" => Some(g__A_YEARS_OLD()),
            "_BAD_IS_A_STATE" => Some(g__BAD_IS_A_STATE()),
            "_BARE_IN_A_KEY" => Some(g__BARE_IN_A_KEY()),
            "_BEFORE_THE_THING" => Some(g__BEFORE_THE_THING()),
            "_BETWEEN_A_PLACE_AND_ITS_COLD" => Some(g__BETWEEN_A_PLACE_AND_ITS_COLD()),
            "_BE_VERBS" => Some(g__BE_VERBS()),
            "_BORROWING_IN_THE_LINE" => Some(g__BORROWING_IN_THE_LINE()),
            "_BOUNDED" => Some(g__BOUNDED()),
            "_BOUNDED_KEEP" => Some(g__BOUNDED_KEEP()),
            "_BRIDGE" => Some(g__BRIDGE()),
            "_BRIDGES" => Some(g__BRIDGES()),
            "_CACHE" => Some(g__CACHE()),
            "_CACHE_KEEP" => Some(g__CACHE_KEEP()),
            "_CAME_OUT_FLAT" => Some(g__CAME_OUT_FLAT()),
            "_CHAT_PHRASES" => Some(g__CHAT_PHRASES()),
            "_CHAT_TAIL" => Some(g__CHAT_TAIL()),
            "_CHILDS_AGE_ROWS" => Some(g__CHILDS_AGE_ROWS()),
            "_CHILD_PHRASES" => Some(g__CHILD_PHRASES()),
            "_CHILD_WORDS" => Some(g__CHILD_WORDS()),
            "_COLD_WORDS" => Some(g__COLD_WORDS()),
            "_COMMON" => Some(g__COMMON()),
            "_CONTRACTION" => Some(g__CONTRACTION()),
            "_CURLY" => Some(g__CURLY()),
            "_CURLY_ANY" => Some(g__CURLY_ANY()),
            "_CUT_BESIDE" => Some(g__CUT_BESIDE()),
            "_DEFINING_VERBS" => Some(g__DEFINING_VERBS()),
            "_DEFINITION_ARTICLES" => Some(g__DEFINITION_ARTICLES()),
            "_DEFINITION_ASK" => Some(g__DEFINITION_ASK()),
            "_DEFINITION_EMPTY" => Some(g__DEFINITION_EMPTY()),
            "_DEFINITION_POINTERS" => Some(g__DEFINITION_POINTERS()),
            "_DEFINITION_TAIL" => Some(g__DEFINITION_TAIL()),
            "_DEFINITION_TROUBLE" => Some(g__DEFINITION_TROUBLE()),
            "_DREAM" => Some(g__DREAM()),
            "_FILLER" => Some(g__FILLER()),
            "_FLAT_YOU_LIVE_IN" => Some(g__FLAT_YOU_LIVE_IN()),
            "_FORMS_CACHE" => Some(g__FORMS_CACHE()),
            "_FORMS_CACHE_KEEP" => Some(g__FORMS_CACHE_KEEP()),
            "_FRAME_MASK" => Some(g__FRAME_MASK()),
            "_FRAME_PRONOUN_FORMS" => Some(g__FRAME_PRONOUN_FORMS()),
            "_FRAME_STARTS" => Some(g__FRAME_STARTS()),
            "_FUNCTION_WORDS" => Some(g__FUNCTION_WORDS()),
            "_HOLDS_MONEY" => Some(g__HOLDS_MONEY()),
            "_HOW_A_PERSON_IS_COLD" => Some(g__HOW_A_PERSON_IS_COLD()),
            "_INDEX" => Some(g__INDEX()),
            "_ING_NOUNS" => Some(g__ING_NOUNS()),
            "_KEEPING_ANSWER" => Some(g__KEEPING_ANSWER()),
            "_KEEPING_AN_EYE" => Some(g__KEEPING_AN_EYE()),
            "_KEEPING_KEY" => Some(g__KEEPING_KEY()),
            "_KEEPING_ROWS" => Some(g__KEEPING_ROWS()),
            "_KEEPING_VERB" => Some(g__KEEPING_VERB()),
            "_KEEPING_WORDS" => Some(g__KEEPING_WORDS()),
            "_KEPT_SOMEWHERE" => Some(g__KEPT_SOMEWHERE()),
            "_KEY_FORMS" => Some(g__KEY_FORMS()),
            "_LIFE_KEY" => Some(g__LIFE_KEY()),
            "_LINE_FORMS" => Some(g__LINE_FORMS()),
            "_MARKS" => Some(g__MARKS()),
            "_MIN_BARE_WORDS" => Some(g__MIN_BARE_WORDS()),
            "_NOT_A_SUBJECT" => Some(g__NOT_A_SUBJECT()),
            "_NOT_A_THING" => Some(g__NOT_A_THING()),
            "_NOT_A_THING_HAD" => Some(g__NOT_A_THING_HAD()),
            "_NOT_GONE" => Some(g__NOT_GONE()),
            "_OFF_A_PERSON" => Some(g__OFF_A_PERSON()),
            "_OF_THE_RULE_OUT" => Some(g__OF_THE_RULE_OUT()),
            "_ONE_SPELLING" => Some(g__ONE_SPELLING()),
            "_ONE_SPELLING_EITHER_WAY" => Some(g__ONE_SPELLING_EITHER_WAY()),
            "_ORDER_OF_STEPS" => Some(g__ORDER_OF_STEPS()),
            "_OTHER_SPELLINGS_KEEP" => Some(g__OTHER_SPELLINGS_KEEP()),
            "_PART_FORMS" => Some(g__PART_FORMS()),
            "_PART_FORMS_KEEP" => Some(g__PART_FORMS_KEEP()),
            "_PEOPLE_AGED" => Some(g__PEOPLE_AGED()),
            "_PHRASE_OTHER_SPELLINGS" => Some(g__PHRASE_OTHER_SPELLINGS()),
            "_PHRASE_SPANS" => Some(g__PHRASE_SPANS()),
            "_PLACES_PEOPLE_LIVE_IN" => Some(g__PLACES_PEOPLE_LIVE_IN()),
            "_POINTS_AT_IT" => Some(g__POINTS_AT_IT()),
            "_POSSESSIVE" => Some(g__POSSESSIVE()),
            "_PUNCTUATION" => Some(g__PUNCTUATION()),
            "_RECURRING" => Some(g__RECURRING()),
            "_RECURRING_ANYWAY" => Some(g__RECURRING_ANYWAY()),
            "_RECURRING_WITH" => Some(g__RECURRING_WITH()),
            "_RESULT_VERB" => Some(g__RESULT_VERB()),
            "_ROWS_CACHE" => Some(g__ROWS_CACHE()),
            "_ROWS_KEEP" => Some(g__ROWS_KEEP()),
            "_SAID_BETWEEN" => Some(g__SAID_BETWEEN()),
            "_SAVING_FOR_A_TRIP" => Some(g__SAVING_FOR_A_TRIP()),
            "_SAVING_VERB" => Some(g__SAVING_VERB()),
            "_SAYS_OF_THEMSELVES" => Some(g__SAYS_OF_THEMSELVES()),
            "_SHAPE_CACHE" => Some(g__SHAPE_CACHE()),
            "_SLEEPY_MARKS" => Some(g__SLEEPY_MARKS()),
            "_SPOILING" => Some(g__SPOILING()),
            "_STEMS" => Some(g__STEMS()),
            "_STILL_GOOD_ASHERS" => Some(g__STILL_GOOD_ASHERS()),
            "_SUBJECT_AT_MOST" => Some(g__SUBJECT_AT_MOST()),
            "_SUBJECT_LEAD" => Some(g__SUBJECT_LEAD()),
            "_SUBJECT_POINTS" => Some(g__SUBJECT_POINTS()),
            "_TAIL_ASK_WORDS" => Some(g__TAIL_ASK_WORDS()),
            "_TAKE_OFF" => Some(g__TAKE_OFF()),
            "_THE_DEGREE_WORDS" => Some(g__THE_DEGREE_WORDS()),
            "_THE_GROUND_UNDERFOOT" => Some(g__THE_GROUND_UNDERFOOT()),
            "_THE_HEAT_OF_AN_ARGUMENT" => Some(g__THE_HEAT_OF_AN_ARGUMENT()),
            "_THE_HEAT_OF_THE_DAY" => Some(g__THE_HEAT_OF_THE_DAY()),
            "_THE_PRONOUNS" => Some(g__THE_PRONOUNS()),
            "_THE_THING_JUDGED" => Some(g__THE_THING_JUDGED()),
            "_THE_WH_WORDS" => Some(g__THE_WH_WORDS()),
            "_THROAT" => Some(g__THROAT()),
            "_WANTED_CACHE" => Some(g__WANTED_CACHE()),
            "_WANTING" => Some(g__WANTING()),
            "_WEAR_ASK" => Some(g__WEAR_ASK()),
            "_WHERE" => Some(g__WHERE()),
            "_WHERE_I_PUT" => Some(g__WHERE_I_PUT()),
            "_WHERE_TO_KEEP" => Some(g__WHERE_TO_KEEP()),
            "_WRAPPED" => Some(g__WRAPPED()),
            "_YEAR_OLD_FIGURE" => Some(g__YEAR_OLD_FIGURE()),
            "_a_clause_about_the_thing" => Some(py_func("_a_clause_about_the_thing", |args: &[Py]| _a_clause_about_the_thing(args))),
            "_a_cut_of_meat" => Some(py_func("_a_cut_of_meat", |args: &[Py]| _a_cut_of_meat(args))),
            "_a_dress_code" => Some(py_func("_a_dress_code", |args: &[Py]| _a_dress_code(args))),
            "_a_keeping_key" => Some(py_func("_a_keeping_key", |args: &[Py]| _a_keeping_key(args))),
            "_a_keeping_row" => Some(py_func("_a_keeping_row", |args: &[Py]| _a_keeping_row(args))),
            "_a_number" => Some(py_func("_a_number", |args: &[Py]| _a_number(args))),
            "_a_part_the_line_does_not_have" => Some(py_func("_a_part_the_line_does_not_have", |args: &[Py]| _a_part_the_line_does_not_have(args))),
            "_a_price_judged_is_not_answered_here" => Some(py_func("_a_price_judged_is_not_answered_here", |args: &[Py]| _a_price_judged_is_not_answered_here(args))),
            "_a_range_for_the_thing" => Some(py_func("_a_range_for_the_thing", |args: &[Py]| _a_range_for_the_thing(args))),
            "_a_thing_that_has_not_gone" => Some(py_func("_a_thing_that_has_not_gone", |args: &[Py]| _a_thing_that_has_not_gone(args))),
            "_a_word_itself" => Some(py_func("_a_word_itself", |args: &[Py]| _a_word_itself(args))),
            "_about_someone_else" => Some(py_func("_about_someone_else", |args: &[Py]| _about_someone_else(args))),
            "_add" => Some(py_func("_add", |args: &[Py]| _add(args))),
            "_another_part" => Some(py_func("_another_part", |args: &[Py]| _another_part(args))),
            "_answers_a_qualification" => Some(py_func("_answers_a_qualification", |args: &[Py]| _answers_a_qualification(args))),
            "_answers_the_cold_of_a_person" => Some(py_func("_answers_the_cold_of_a_person", |args: &[Py]| _answers_the_cold_of_a_person(args))),
            "_answers_the_cold_of_a_place" => Some(py_func("_answers_the_cold_of_a_place", |args: &[Py]| _answers_the_cold_of_a_place(args))),
            "_ask_about_someone_only" => Some(py_func("_ask_about_someone_only", |args: &[Py]| _ask_about_someone_only(args))),
            "_ask_frame_forms" => Some(py_func("_ask_frame_forms", |args: &[Py]| _ask_frame_forms(args))),
            "_asking_with" => Some(py_func("_asking_with", |args: &[Py]| _asking_with(args))),
            "_asks_if_it_is_still_good" => Some(py_func("_asks_if_it_is_still_good", |args: &[Py]| _asks_if_it_is_still_good(args))),
            "_asks_to_explain" => Some(py_func("_asks_to_explain", |args: &[Py]| _asks_to_explain(args))),
            "_asks_what_it_is_for" => Some(py_func("_asks_what_it_is_for", |args: &[Py]| _asks_what_it_is_for(args))),
            "_avoid_words" => Some(py_func("_avoid_words", |args: &[Py]| _avoid_words(args))),
            "_bare_is_the_word" => Some(py_func("_bare_is_the_word", |args: &[Py]| _bare_is_the_word(args))),
            "_bare_report" => Some(py_func("_bare_report", |args: &[Py]| _bare_report(args))),
            "_best" => Some(py_func("_best", |args: &[Py]| _best(args))),
            "_body_parts" => Some(py_func("_body_parts", |args: &[Py]| _body_parts(args))),
            "_bounded" => Some(py_func("_bounded", |args: &[Py]| _bounded(args))),
            "_cvc" => Some(py_func("_cvc", |args: &[Py]| _cvc(args))),
            "_declared_for" => Some(py_func("_declared_for", |args: &[Py]| _declared_for(args))),
            "_defines" => Some(py_func("_defines", |args: &[Py]| _defines(args))),
            "_denied" => Some(py_func("_denied", |args: &[Py]| _denied(args))),
            "_denier_inside" => Some(py_func("_denier_inside", |args: &[Py]| _denier_inside(args))),
            "_domain_rarity" => Some(py_func("_domain_rarity", |args: &[Py]| _domain_rarity(args))),
            "_doubles_for_an_ending" => Some(py_func("_doubles_for_an_ending", |args: &[Py]| _doubles_for_an_ending(args))),
            "_engine_reads" => Some(py_func("_engine_reads", |args: &[Py]| _engine_reads(args))),
            "_feeling_occasion" => Some(py_func("_feeling_occasion", |args: &[Py]| _feeling_occasion(args))),
            "_following_on" => Some(py_func("_following_on", |args: &[Py]| _following_on(args))),
            "_forms" => Some(py_func("_forms", |args: &[Py]| _forms(args))),
            "_frame_masked" => Some(py_func("_frame_masked", |args: &[Py]| _frame_masked(args))),
            "_frame_pronoun_forms" => Some(py_func("_frame_pronoun_forms", |args: &[Py]| _frame_pronoun_forms(args))),
            "_frame_starts" => Some(py_func("_frame_starts", |args: &[Py]| _frame_starts(args))),
            "_gerund" => Some(py_func("_gerund", |args: &[Py]| _gerund(args))),
            "_household_reading" => Some(py_func("_household_reading", |args: &[Py]| _household_reading(args))),
            "_in_topic" => Some(py_func("_in_topic", |args: &[Py]| _in_topic(args))),
            "_inside" => Some(py_func("_inside", |args: &[Py]| _inside(args))),
            "_is_a_person" => Some(py_func("_is_a_person", |args: &[Py]| _is_a_person(args))),
            "_is_consonant" => Some(py_func("_is_consonant", |args: &[Py]| _is_consonant(args))),
            "_keeping_row" => Some(py_func("_keeping_row", |args: &[Py]| _keeping_row(args))),
            "_keeps_failing" => Some(py_func("_keeps_failing", |args: &[Py]| _keeps_failing(args))),
            "_keeps_failing_at" => Some(py_func("_keeps_failing_at", |args: &[Py]| _keeps_failing_at(args))),
            "_key_body_parts" => Some(py_func("_key_body_parts", |args: &[Py]| _key_body_parts(args))),
            "_key_forms" => Some(py_func("_key_forms", |args: &[Py]| _key_forms(args))),
            "_key_names" => Some(py_func("_key_names", |args: &[Py]| _key_names(args))),
            "_key_on_the_want" => Some(py_func("_key_on_the_want", |args: &[Py]| _key_on_the_want(args))),
            "_key_says_years_old" => Some(py_func("_key_says_years_old", |args: &[Py]| _key_says_years_old(args))),
            "_key_words" => Some(py_func("_key_words", |args: &[Py]| _key_words(args))),
            "_later_words" => Some(py_func("_later_words", |args: &[Py]| _later_words(args))),
            "_line_forms" => Some(py_func("_line_forms", |args: &[Py]| _line_forms(args))),
            "_line_rows" => Some(py_func("_line_rows", |args: &[Py]| _line_rows(args))),
            "_long" => Some(g__long()),
            "_misspelt_avoid" => Some(py_func("_misspelt_avoid", |args: &[Py]| _misspelt_avoid(args))),
            "_named_things" => Some(py_func("_named_things", |args: &[Py]| _named_things(args))),
            "_names_a_fault" => Some(py_func("_names_a_fault", |args: &[Py]| _names_a_fault(args))),
            "_names_a_feared_thing" => Some(py_func("_names_a_feared_thing", |args: &[Py]| _names_a_feared_thing(args))),
            "_names_one" => Some(py_func("_names_one", |args: &[Py]| _names_one(args))),
            "_offers_a_substitute" => Some(py_func("_offers_a_substitute", |args: &[Py]| _offers_a_substitute(args))),
            "_one_edit" => Some(py_func("_one_edit", |args: &[Py]| _one_edit(args))),
            "_only_as_a_kind" => Some(py_func("_only_as_a_kind", |args: &[Py]| _only_as_a_kind(args))),
            "_only_asking_about_it" => Some(py_func("_only_asking_about_it", |args: &[Py]| _only_asking_about_it(args))),
            "_only_the_frame" => Some(py_func("_only_the_frame", |args: &[Py]| _only_the_frame(args))),
            "_only_the_want" => Some(py_func("_only_the_want", |args: &[Py]| _only_the_want(args))),
            "_other_reading" => Some(py_func("_other_reading", |args: &[Py]| _other_reading(args))),
            "_own_words" => Some(py_func("_own_words", |args: &[Py]| _own_words(args))),
            "_part_forms" => Some(py_func("_part_forms", |args: &[Py]| _part_forms(args))),
            "_part_match" => Some(py_func("_part_match", |args: &[Py]| _part_match(args))),
            "_part_matches" => Some(py_func("_part_matches", |args: &[Py]| _part_matches(args))),
            "_part_score" => Some(py_func("_part_score", |args: &[Py]| _part_score(args))),
            "_parts_across" => Some(py_func("_parts_across", |args: &[Py]| _parts_across(args))),
            "_past_a_handoff" => Some(py_func("_past_a_handoff", |args: &[Py]| _past_a_handoff(args))),
            "_past_the_carer" => Some(py_func("_past_the_carer", |args: &[Py]| _past_the_carer(args))),
            "_phrase_found" => Some(py_func("_phrase_found", |args: &[Py]| _phrase_found(args))),
            "_phrase_span" => Some(py_func("_phrase_span", |args: &[Py]| _phrase_span(args))),
            "_phrase_span_bare" => Some(py_func("_phrase_span_bare", |args: &[Py]| _phrase_span_bare(args))),
            "_phrase_span_between" => Some(py_func("_phrase_span_between", |args: &[Py]| _phrase_span_between(args))),
            "_phrase_span_spelling" => Some(py_func("_phrase_span_spelling", |args: &[Py]| _phrase_span_spelling(args))),
            "_phrase_span_work" => Some(py_func("_phrase_span_work", |args: &[Py]| _phrase_span_work(args))),
            "_plainer" => Some(py_func("_plainer", |args: &[Py]| _plainer(args))),
            "_plans_it" => Some(py_func("_plans_it", |args: &[Py]| _plans_it(args))),
            "_rank" => Some(py_func("_rank", |args: &[Py]| _rank(args))),
            "_rarity" => Some(py_func("_rarity", |args: &[Py]| _rarity(args))),
            "_reaches" => Some(py_func("_reaches", |args: &[Py]| _reaches(args))),
            "_reaching" => Some(py_func("_reaching", |args: &[Py]| _reaching(args))),
            "_read_a_thing_had" => Some(py_func("_read_a_thing_had", |args: &[Py]| _read_a_thing_had(args))),
            "_read_the_adjective" => Some(py_func("_read_the_adjective", |args: &[Py]| _read_the_adjective(args))),
            "_read_typos" => Some(py_func("_read_typos", |args: &[Py]| _read_typos(args))),
            "_refers_to_the_person" => Some(py_func("_refers_to_the_person", |args: &[Py]| _refers_to_the_person(args))),
            "_row_order" => Some(py_func("_row_order", |args: &[Py]| _row_order(args))),
            "_rows_key" => Some(py_func("_rows_key", |args: &[Py]| _rows_key(args))),
            "_said_about_someone" => Some(py_func("_said_about_someone", |args: &[Py]| _said_about_someone(args))),
            "_said_again" => Some(py_func("_said_again", |args: &[Py]| _said_again(args))),
            "_said_as_a_report" => Some(py_func("_said_as_a_report", |args: &[Py]| _said_as_a_report(args))),
            "_says_a_key_word" => Some(py_func("_says_a_key_word", |args: &[Py]| _says_a_key_word(args))),
            "_says_a_shape" => Some(py_func("_says_a_shape", |args: &[Py]| _says_a_shape(args))),
            "_says_how_the_speaker_handles_it" => Some(py_func("_says_how_the_speaker_handles_it", |args: &[Py]| _says_how_the_speaker_handles_it(args))),
            "_says_removal" => Some(py_func("_says_removal", |args: &[Py]| _says_removal(args))),
            "_says_what_is_missing" => Some(py_func("_says_what_is_missing", |args: &[Py]| _says_what_is_missing(args))),
            "_scanned_rows" => Some(py_func("_scanned_rows", |args: &[Py]| _scanned_rows(args))),
            "_shape" => Some(py_func("_shape", |args: &[Py]| _shape(args))),
            "_short" => Some(g__short()),
            "_speaker_is_feeling" => Some(py_func("_speaker_is_feeling", |args: &[Py]| _speaker_is_feeling(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_the_age_is_a_childs" => Some(py_func("_the_age_is_a_childs", |args: &[Py]| _the_age_is_a_childs(args))),
            "_the_animal_rules_out_that_card" => Some(py_func("_the_animal_rules_out_that_card", |args: &[Py]| _the_animal_rules_out_that_card(args))),
            "_the_answer_the_colour_dragged_in" => Some(py_func("_the_answer_the_colour_dragged_in", |args: &[Py]| _the_answer_the_colour_dragged_in(args))),
            "_the_ask_in_order" => Some(py_func("_the_ask_in_order", |args: &[Py]| _the_ask_in_order(args))),
            "_the_ask_it_wraps" => Some(py_func("_the_ask_it_wraps", |args: &[Py]| _the_ask_it_wraps(args))),
            "_the_ask_on_its_own" => Some(py_func("_the_ask_on_its_own", |args: &[Py]| _the_ask_on_its_own(args))),
            "_the_ask_reading" => Some(py_func("_the_ask_reading", |args: &[Py]| _the_ask_reading(args))),
            "_the_ask_said" => Some(py_func("_the_ask_said", |args: &[Py]| _the_ask_said(args))),
            "_the_best_entry" => Some(py_func("_the_best_entry", |args: &[Py]| _the_best_entry(args))),
            "_the_card_for_the_occasion_asked" => Some(py_func("_the_card_for_the_occasion_asked", |args: &[Py]| _the_card_for_the_occasion_asked(args))),
            "_the_card_for_the_place_first" => Some(py_func("_the_card_for_the_place_first", |args: &[Py]| _the_card_for_the_place_first(args))),
            "_the_colour_is_the_whole_ask" => Some(py_func("_the_colour_is_the_whole_ask", |args: &[Py]| _the_colour_is_the_whole_ask(args))),
            "_the_degrees_are_a_temperature" => Some(py_func("_the_degrees_are_a_temperature", |args: &[Py]| _the_degrees_are_a_temperature(args))),
            "_the_device_is_the_one_falling_asleep" => Some(py_func("_the_device_is_the_one_falling_asleep", |args: &[Py]| _the_device_is_the_one_falling_asleep(args))),
            "_the_kept_rows" => Some(py_func("_the_kept_rows", |args: &[Py]| _the_kept_rows(args))),
            "_the_key_names_the_thing" => Some(py_func("_the_key_names_the_thing", |args: &[Py]| _the_key_names_the_thing(args))),
            "_the_key_words" => Some(py_func("_the_key_words", |args: &[Py]| _the_key_words(args))),
            "_the_longest_run" => Some(py_func("_the_longest_run", |args: &[Py]| _the_longest_run(args))),
            "_the_name_alone" => Some(py_func("_the_name_alone", |args: &[Py]| _the_name_alone(args))),
            "_the_name_card" => Some(py_func("_the_name_card", |args: &[Py]| _the_name_card(args))),
            "_the_names_in_the_ask" => Some(py_func("_the_names_in_the_ask", |args: &[Py]| _the_names_in_the_ask(args))),
            "_the_occasion_family" => Some(py_func("_the_occasion_family", |args: &[Py]| _the_occasion_family(args))),
            "_the_occasion_words" => Some(py_func("_the_occasion_words", |args: &[Py]| _the_occasion_words(args))),
            "_the_person_says_they_are_cold" => Some(py_func("_the_person_says_they_are_cold", |args: &[Py]| _the_person_says_they_are_cold(args))),
            "_the_place_the_line_says_is_cold" => Some(py_func("_the_place_the_line_says_is_cold", |args: &[Py]| _the_place_the_line_says_is_cold(args))),
            "_the_reply_defines" => Some(py_func("_the_reply_defines", |args: &[Py]| _the_reply_defines(args))),
            "_the_same_words" => Some(py_func("_the_same_words", |args: &[Py]| _the_same_words(args))),
            "_the_subject_alone" => Some(py_func("_the_subject_alone", |args: &[Py]| _the_subject_alone(args))),
            "_the_subject_it_avoids" => Some(py_func("_the_subject_it_avoids", |args: &[Py]| _the_subject_it_avoids(args))),
            "_the_subject_the_line_leans_on" => Some(py_func("_the_subject_the_line_leans_on", |args: &[Py]| _the_subject_the_line_leans_on(args))),
            "_the_subjects_own_clause" => Some(py_func("_the_subjects_own_clause", |args: &[Py]| _the_subjects_own_clause(args))),
            "_the_subjects_own_row" => Some(py_func("_the_subjects_own_row", |args: &[Py]| _the_subjects_own_row(args))),
            "_the_tail_asks_about" => Some(py_func("_the_tail_asks_about", |args: &[Py]| _the_tail_asks_about(args))),
            "_the_thing_asked_about" => Some(py_func("_the_thing_asked_about", |args: &[Py]| _the_thing_asked_about(args))),
            "_the_thing_in" => Some(py_func("_the_thing_in", |args: &[Py]| _the_thing_in(args))),
            "_the_thing_the_chat_is_on" => Some(py_func("_the_thing_the_chat_is_on", |args: &[Py]| _the_thing_the_chat_is_on(args))),
            "_the_whole_line" => Some(py_func("_the_whole_line", |args: &[Py]| _the_whole_line(args))),
            "_the_whole_subject" => Some(py_func("_the_whole_subject", |args: &[Py]| _the_whole_subject(args))),
            "_the_words_without_a_colour" => Some(py_func("_the_words_without_a_colour", |args: &[Py]| _the_words_without_a_colour(args))),
            "_topic_domains" => Some(py_func("_topic_domains", |args: &[Py]| _topic_domains(args))),
            "_touches" => Some(py_func("_touches", |args: &[Py]| _touches(args))),
            "_two_clauses" => Some(py_func("_two_clauses", |args: &[Py]| _two_clauses(args))),
            "_two_positions" => Some(py_func("_two_positions", |args: &[Py]| _two_positions(args))),
            "_typo_forms" => Some(py_func("_typo_forms", |args: &[Py]| _typo_forms(args))),
            "_under_their_own_hands" => Some(py_func("_under_their_own_hands", |args: &[Py]| _under_their_own_hands(args))),
            "_unframed" => Some(py_func("_unframed", |args: &[Py]| _unframed(args))),
            "_vocabulary" => Some(py_func("_vocabulary", |args: &[Py]| _vocabulary(args))),
            "_wanted" => Some(py_func("_wanted", |args: &[Py]| _wanted(args))),
            "_wanted_bare" => Some(py_func("_wanted_bare", |args: &[Py]| _wanted_bare(args))),
            "_wanting" => Some(py_func("_wanting", |args: &[Py]| _wanting(args))),
            "_what_is_wrong_with_them" => Some(py_func("_what_is_wrong_with_them", |args: &[Py]| _what_is_wrong_with_them(args))),
            "_where" => Some(py_func("_where", |args: &[Py]| _where(args))),
            "_with_the_animal_in_hand" => Some(py_func("_with_the_animal_in_hand", |args: &[Py]| _with_the_animal_in_hand(args))),
            "_with_the_child_by_place" => Some(py_func("_with_the_child_by_place", |args: &[Py]| _with_the_child_by_place(args))),
            "_without_a_cooking_that_is_a_name" => Some(py_func("_without_a_cooking_that_is_a_name", |args: &[Py]| _without_a_cooking_that_is_a_name(args))),
            "_without_a_dead_that_only_means_very" => Some(py_func("_without_a_dead_that_only_means_very", |args: &[Py]| _without_a_dead_that_only_means_very(args))),
            "_without_ask_phrase" => Some(py_func("_without_ask_phrase", |args: &[Py]| _without_ask_phrase(args))),
            "_without_possessive" => Some(py_func("_without_possessive", |args: &[Py]| _without_possessive(args))),
            "_without_the_place" => Some(py_func("_without_the_place", |args: &[Py]| _without_the_place(args))),
            "_without_the_ruled_out" => Some(py_func("_without_the_ruled_out", |args: &[Py]| _without_the_ruled_out(args))),
            "_word_index" => Some(py_func("_word_index", |args: &[Py]| _word_index(args))),
            "_words_of" => Some(py_func("_words_of", |args: &[Py]| _words_of(args))),
            "_work_out_forms" => Some(py_func("_work_out_forms", |args: &[Py]| _work_out_forms(args))),
            "_work_out_the_same_words" => Some(py_func("_work_out_the_same_words", |args: &[Py]| _work_out_the_same_words(args))),
            "_wrapped_subject" => Some(py_func("_wrapped_subject", |args: &[Py]| _wrapped_subject(args))),
            "_y_word_stems" => Some(py_func("_y_word_stems", |args: &[Py]| _y_word_stems(args))),
            "a_colour_is_worn" => Some(py_func("a_colour_is_worn", |args: &[Py]| a_colour_is_worn(args))),
            "a_food_is_being_kept" => Some(py_func("a_food_is_being_kept", |args: &[Py]| a_food_is_being_kept(args))),
            "a_price_not_a_loan" => Some(py_func("a_price_not_a_loan", |args: &[Py]| a_price_not_a_loan(args))),
            "a_sanding_job" => Some(py_func("a_sanding_job", |args: &[Py]| a_sanding_job(args))),
            "a_thing_has_stopped" => Some(py_func("a_thing_has_stopped", |args: &[Py]| a_thing_has_stopped(args))),
            "a_weight" => Some(py_func("a_weight", |args: &[Py]| a_weight(args))),
            "about" => Some(py_func("about", |args: &[Py]| about(args))),
            "about_the_chat" => Some(py_func("about_the_chat", |args: &[Py]| about_the_chat(args))),
            "about_the_topic" => Some(py_func("about_the_topic", |args: &[Py]| about_the_topic(args))),
            "about_them" => Some(py_func("about_them", |args: &[Py]| about_them(args))),
            "all_of" => Some(py_func("all_of", |args: &[Py]| all_of(args))),
            "asks_how_often" => Some(py_func("asks_how_often", |args: &[Py]| asks_how_often(args))),
            "asks_the_kinds" => Some(py_func("asks_the_kinds", |args: &[Py]| asks_the_kinds(args))),
            "asks_what_to_wear" => Some(py_func("asks_what_to_wear", |args: &[Py]| asks_what_to_wear(args))),
            "asks_where" => Some(py_func("asks_where", |args: &[Py]| asks_where(args))),
            "asks_where_a_shop_answers" => Some(py_func("asks_where_a_shop_answers", |args: &[Py]| asks_where_a_shop_answers(args))),
            "entry" => Some(py_func("entry", |args: &[Py]| entry(args))),
            "find" => Some(py_func("find", |args: &[Py]| find(args))),
            "food_has_gone_off" => Some(py_func("food_has_gone_off", |args: &[Py]| food_has_gone_off(args))),
            "guard_passes" => Some(py_func("guard_passes", |args: &[Py]| guard_passes(args))),
            "key_words" => Some(py_func("key_words", |args: &[Py]| key_words(args))),
            "kinds_entries" => Some(py_func("kinds_entries", |args: &[Py]| kinds_entries(args))),
            "leans_on_a_subject" => Some(py_func("leans_on_a_subject", |args: &[Py]| leans_on_a_subject(args))),
            "names_a_thing" => Some(py_func("names_a_thing", |args: &[Py]| names_a_thing(args))),
            "names_its_own_subject" => Some(py_func("names_its_own_subject", |args: &[Py]| names_its_own_subject(args))),
            "names_nothing_of_its_own" => Some(py_func("names_nothing_of_its_own", |args: &[Py]| names_nothing_of_its_own(args))),
            "names_one_thing" => Some(py_func("names_one_thing", |args: &[Py]| names_one_thing(args))),
            "not_a_count_for_the_box_or_the_dish" => Some(py_func("not_a_count_for_the_box_or_the_dish", |args: &[Py]| not_a_count_for_the_box_or_the_dish(args))),
            "not_a_sleepy_body" => Some(py_func("not_a_sleepy_body", |args: &[Py]| not_a_sleepy_body(args))),
            "not_keeping_an_eye_on_it" => Some(py_func("not_keeping_an_eye_on_it", |args: &[Py]| not_keeping_an_eye_on_it(args))),
            "not_the_ask_about_the_order" => Some(py_func("not_the_ask_about_the_order", |args: &[Py]| not_the_ask_about_the_order(args))),
            "not_the_flat_of_something_made" => Some(py_func("not_the_flat_of_something_made", |args: &[Py]| not_the_flat_of_something_made(args))),
            "not_the_flat_you_live_in" => Some(py_func("not_the_flat_you_live_in", |args: &[Py]| not_the_flat_you_live_in(args))),
            "not_the_heat_of_an_argument" => Some(py_func("not_the_heat_of_an_argument", |args: &[Py]| not_the_heat_of_an_argument(args))),
            "only_a_container" => Some(py_func("only_a_container", |args: &[Py]| only_a_container(args))),
            "said" => Some(py_func("said", |args: &[Py]| said(args))),
            "said_in_this_line" => Some(py_func("said_in_this_line", |args: &[Py]| said_in_this_line(args))),
            "said_text" => Some(py_func("said_text", |args: &[Py]| said_text(args))),
            "saving_for_a_trip" => Some(py_func("saving_for_a_trip", |args: &[Py]| saving_for_a_trip(args))),
            "says_it_has_gone_off" => Some(py_func("says_it_has_gone_off", |args: &[Py]| says_it_has_gone_off(args))),
            "score" => Some(py_func("score", |args: &[Py]| score(args))),
            "the_appliance_swallows_the_name" => Some(py_func("the_appliance_swallows_the_name", |args: &[Py]| the_appliance_swallows_the_name(args))),
            "the_ask_is_the_order_of_steps" => Some(py_func("the_ask_is_the_order_of_steps", |args: &[Py]| the_ask_is_the_order_of_steps(args))),
            "the_back_is_a_body_part" => Some(py_func("the_back_is_a_body_part", |args: &[Py]| the_back_is_a_body_part(args))),
            "the_body_is_short_of_sleep" => Some(py_func("the_body_is_short_of_sleep", |args: &[Py]| the_body_is_short_of_sleep(args))),
            "the_fault" => Some(py_func("the_fault", |args: &[Py]| the_fault(args))),
            "the_line" => Some(py_func("the_line", |args: &[Py]| the_line(args))),
            "the_thing_has_gone" => Some(py_func("the_thing_has_gone", |args: &[Py]| the_thing_has_gone(args))),
            "the_years_are_a_things" => Some(py_func("the_years_are_a_things", |args: &[Py]| the_years_are_a_things(args))),
            "tokens" => Some(py_func("tokens", |args: &[Py]| tokens(args))),
            _ => None,
        }),
        set: None,
    }))
}
