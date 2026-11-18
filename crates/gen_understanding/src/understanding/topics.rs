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


thread_local! { static G_ADDRESS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_IN_A_YEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_OF_A_CHILD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARTICLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_AMOUNT_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_AMOUNT_SYMBOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHILD_TILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHILD_TO_READ_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GUT_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MONEY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_WHO_WANTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PORTION_NEEDS_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PORTION_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_WANTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WAIT_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEHAVIOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIG_PURSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODILY_OUTPUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODILY_OUTPUT_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_OR_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_TWO_LIFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BROKE_AS_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BROKE_AS_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANCELS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANCEL_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAR_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLIPPED_AFTER_A_SLASH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLIPPED_UNIT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLD_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMMON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPARE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPLAINT_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPLAINT_OF_NEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_NEEDS_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_NEEDS_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_NOT_NEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_OTHER_LIFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTRY_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTRY_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTRY_HOMOGRAPHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROP_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROP_KITCHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROP_STATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUE_CARRIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEGREE_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DERIVED_ENDINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRIVEN_JOINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRIVEN_MOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRIVEN_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRIVEN_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXERCISE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXERCISE_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXTRA_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_PAIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_STEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_SIZE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRIEND_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRIEND_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GENERIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFTS_OR_OCCASIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_ALWAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_WANTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVEN_UP_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVE_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVE_RELATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVE_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVE_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVING_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIVING_UP_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROWING_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAD_FOG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAD_FOG_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAD_FOG_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEARERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURT_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDIOM_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGEMENT_OF_RECORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MACHINE_BIN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MACHINE_WORD_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MACHINE_WORD_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MECHANISM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MECHANISM_HOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MIXED_FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MIXED_JOINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_TIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_TIGHT_MAX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_BREAKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NATURE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOSE_BLOCKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOSE_BLOCKED_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_CUT_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOUNY_SIGNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_IS_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLY_A_STOOL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDINARY_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNED_BY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNS_THE_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_UP_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_TOPICS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_HOMOGRAPHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_WORDS_WITH_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAYS_UP_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAY_UP_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POLYSEMOUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POLY_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PORTION_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PORTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_OUT_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAMPLE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SCIENCE_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SEASONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENSE_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENSE_ELSEWHERE_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENSE_NEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENSE_PAIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENSE_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENSE_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGN_FOLLOWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGN_NOT_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SKIN_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SKIN_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SKIN_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SKIN_STATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STONE_COMPOUNDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STONE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOOL_AS_FURNITURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STUDY_OBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUMS_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUM_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEETHING_AND_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEETHING_IS_A_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PERSONS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAVEL_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAVEL_STRONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_HOMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_LIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_LIVES_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_LIVES_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNWELL_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNWELL_IDIOMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNWELL_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNWELL_SIGNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTED_BEFORE_A_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANT_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WARMTH_ARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WARMTH_DEGREE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WARMTH_HABITUAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WARMTH_THEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WARMTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATCHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATCH_OUT_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEATHER_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEATHER_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_DIGITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORLD_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORN_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORN_OUT_BOOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORN_OUT_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORN_OUT_LOOKBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORN_OUT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STATE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BREAKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FUNCTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GRAND_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KIN_BUILT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SENSE_TIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEPPED_RELATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_OWNER_SETTLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__form: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__forms: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__sense_topic: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__tiers: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ADDRESS_WORDS() -> Py { ensure_init(); G_ADDRESS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADDRESS_WORDS(v: Py) { ensure_init(); G_ADDRESS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_AFTER() -> Py { ensure_init(); G_AGE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_AFTER(v: Py) { ensure_init(); G_AGE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_IN_A_YEAR() -> Py { ensure_init(); G_AGE_IN_A_YEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_IN_A_YEAR(v: Py) { ensure_init(); G_AGE_IN_A_YEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_NOUNS() -> Py { ensure_init(); G_AGE_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_NOUNS(v: Py) { ensure_init(); G_AGE_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_OF_A_CHILD_WORDS() -> Py { ensure_init(); G_AGE_OF_A_CHILD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_OF_A_CHILD_WORDS(v: Py) { ensure_init(); G_AGE_OF_A_CHILD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMALS() -> Py { ensure_init(); G_ANIMALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMALS(v: Py) { ensure_init(); G_ANIMALS.with(|c| *c.borrow_mut() = v); }
pub fn g_ARTICLES() -> Py { ensure_init(); G_ARTICLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARTICLES(v: Py) { ensure_init(); G_ARTICLES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_PHRASES() -> Py { ensure_init(); G_ASK_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_PHRASES(v: Py) { ensure_init(); G_ASK_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WORDS() -> Py { ensure_init(); G_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); G_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_AMOUNT_NOUN() -> Py { ensure_init(); G_A_AMOUNT_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_AMOUNT_NOUN(v: Py) { ensure_init(); G_A_AMOUNT_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_AMOUNT_SYMBOL() -> Py { ensure_init(); G_A_AMOUNT_SYMBOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_AMOUNT_SYMBOL(v: Py) { ensure_init(); G_A_AMOUNT_SYMBOL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHILD_TILL() -> Py { ensure_init(); G_A_CHILD_TILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_TILL(v: Py) { ensure_init(); G_A_CHILD_TILL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHILD_TO_READ_FOR() -> Py { ensure_init(); G_A_CHILD_TO_READ_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_TO_READ_FOR(v: Py) { ensure_init(); G_A_CHILD_TO_READ_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GUT_ONLY() -> Py { ensure_init(); G_A_GUT_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GUT_ONLY(v: Py) { ensure_init(); G_A_GUT_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MONEY_WORD() -> Py { ensure_init(); G_A_MONEY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_WORD(v: Py) { ensure_init(); G_A_MONEY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_WHO_WANTS() -> Py { ensure_init(); G_A_PERSON_WHO_WANTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_WHO_WANTS(v: Py) { ensure_init(); G_A_PERSON_WHO_WANTS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PORTION_NEEDS_OF() -> Py { ensure_init(); G_A_PORTION_NEEDS_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PORTION_NEEDS_OF(v: Py) { ensure_init(); G_A_PORTION_NEEDS_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PORTION_UNITS() -> Py { ensure_init(); G_A_PORTION_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PORTION_UNITS(v: Py) { ensure_init(); G_A_PORTION_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_WANTED() -> Py { ensure_init(); G_A_THING_WANTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_WANTED(v: Py) { ensure_init(); G_A_THING_WANTED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAIT() -> Py { ensure_init(); G_A_WAIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAIT(v: Py) { ensure_init(); G_A_WAIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WAIT_TO() -> Py { ensure_init(); G_A_WAIT_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WAIT_TO(v: Py) { ensure_init(); G_A_WAIT_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_BEHAVIOUR() -> Py { ensure_init(); G_BEHAVIOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEHAVIOUR(v: Py) { ensure_init(); G_BEHAVIOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_BIG_PURSE() -> Py { ensure_init(); G_BIG_PURSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIG_PURSE(v: Py) { ensure_init(); G_BIG_PURSE.with(|c| *c.borrow_mut() = v); }
pub fn g_BODILY_OUTPUT() -> Py { ensure_init(); G_BODILY_OUTPUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODILY_OUTPUT(v: Py) { ensure_init(); G_BODILY_OUTPUT.with(|c| *c.borrow_mut() = v); }
pub fn g_BODILY_OUTPUT_NOUN() -> Py { ensure_init(); G_BODILY_OUTPUT_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODILY_OUTPUT_NOUN(v: Py) { ensure_init(); G_BODILY_OUTPUT_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY() -> Py { ensure_init(); G_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY(v: Py) { ensure_init(); G_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_CUES() -> Py { ensure_init(); G_BODY_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_CUES(v: Py) { ensure_init(); G_BODY_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_OR_SELF() -> Py { ensure_init(); G_BODY_OR_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_OR_SELF(v: Py) { ensure_init(); G_BODY_OR_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_TWO_LIFE() -> Py { ensure_init(); G_BODY_TWO_LIFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_TWO_LIFE(v: Py) { ensure_init(); G_BODY_TWO_LIFE.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOK_WORDS() -> Py { ensure_init(); G_BOOK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOK_WORDS(v: Py) { ensure_init(); G_BOOK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BROKE_AS_SUBJECT() -> Py { ensure_init(); G_BROKE_AS_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BROKE_AS_SUBJECT(v: Py) { ensure_init(); G_BROKE_AS_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_BROKE_AS_TIME() -> Py { ensure_init(); G_BROKE_AS_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BROKE_AS_TIME(v: Py) { ensure_init(); G_BROKE_AS_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_CANCELS() -> Py { ensure_init(); G_CANCELS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANCELS(v: Py) { ensure_init(); G_CANCELS.with(|c| *c.borrow_mut() = v); }
pub fn g_CANCEL_AFTER() -> Py { ensure_init(); G_CANCEL_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANCEL_AFTER(v: Py) { ensure_init(); G_CANCEL_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRY() -> Py { ensure_init(); G_CARRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRY(v: Py) { ensure_init(); G_CARRY.with(|c| *c.borrow_mut() = v); }
pub fn g_CAR_NAMES() -> Py { ensure_init(); G_CAR_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAR_NAMES(v: Py) { ensure_init(); G_CAR_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_CLIPPED_AFTER_A_SLASH() -> Py { ensure_init(); G_CLIPPED_AFTER_A_SLASH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLIPPED_AFTER_A_SLASH(v: Py) { ensure_init(); G_CLIPPED_AFTER_A_SLASH.with(|c| *c.borrow_mut() = v); }
pub fn g_CLIPPED_UNIT_WORDS() -> Py { ensure_init(); G_CLIPPED_UNIT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLIPPED_UNIT_WORDS(v: Py) { ensure_init(); G_CLIPPED_UNIT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COLD_PEOPLE() -> Py { ensure_init(); G_COLD_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLD_PEOPLE(v: Py) { ensure_init(); G_COLD_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_COLD_WORDS() -> Py { ensure_init(); G_COLD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLD_WORDS(v: Py) { ensure_init(); G_COLD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMMON_WORDS() -> Py { ensure_init(); G_COMMON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMMON_WORDS(v: Py) { ensure_init(); G_COMMON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMMS() -> Py { ensure_init(); G_COMMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMMS(v: Py) { ensure_init(); G_COMMS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPARE_WORDS() -> Py { ensure_init(); G_COMPARE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPARE_WORDS(v: Py) { ensure_init(); G_COMPARE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPLAINT_OF() -> Py { ensure_init(); G_COMPLAINT_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPLAINT_OF(v: Py) { ensure_init(); G_COMPLAINT_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPLAINT_OF_NEXT() -> Py { ensure_init(); G_COMPLAINT_OF_NEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPLAINT_OF_NEXT(v: Py) { ensure_init(); G_COMPLAINT_OF_NEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITIONS() -> Py { ensure_init(); G_CONDITIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITIONS(v: Py) { ensure_init(); G_CONDITIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_BOOST() -> Py { ensure_init(); G_CONDITION_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_BOOST(v: Py) { ensure_init(); G_CONDITION_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_NEEDS_PART() -> Py { ensure_init(); G_CONDITION_NEEDS_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_NEEDS_PART(v: Py) { ensure_init(); G_CONDITION_NEEDS_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_NEEDS_WORD() -> Py { ensure_init(); G_CONDITION_NEEDS_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_NEEDS_WORD(v: Py) { ensure_init(); G_CONDITION_NEEDS_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_NOT_NEXT() -> Py { ensure_init(); G_CONDITION_NOT_NEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_NOT_NEXT(v: Py) { ensure_init(); G_CONDITION_NOT_NEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_OTHER_LIFE() -> Py { ensure_init(); G_CONDITION_OTHER_LIFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_OTHER_LIFE(v: Py) { ensure_init(); G_CONDITION_OTHER_LIFE.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_REACH() -> Py { ensure_init(); G_CONDITION_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_REACH(v: Py) { ensure_init(); G_CONDITION_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTRY_BOOST() -> Py { ensure_init(); G_COUNTRY_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTRY_BOOST(v: Py) { ensure_init(); G_COUNTRY_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTRY_CUES() -> Py { ensure_init(); G_COUNTRY_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTRY_CUES(v: Py) { ensure_init(); G_COUNTRY_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTRY_HOMOGRAPHS() -> Py { ensure_init(); G_COUNTRY_HOMOGRAPHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTRY_HOMOGRAPHS(v: Py) { ensure_init(); G_COUNTRY_HOMOGRAPHS.with(|c| *c.borrow_mut() = v); }
pub fn g_CROPS() -> Py { ensure_init(); G_CROPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROPS(v: Py) { ensure_init(); G_CROPS.with(|c| *c.borrow_mut() = v); }
pub fn g_CROP_BOOST() -> Py { ensure_init(); G_CROP_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROP_BOOST(v: Py) { ensure_init(); G_CROP_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_CROP_KITCHEN() -> Py { ensure_init(); G_CROP_KITCHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROP_KITCHEN(v: Py) { ensure_init(); G_CROP_KITCHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_CROP_STATES() -> Py { ensure_init(); G_CROP_STATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROP_STATES(v: Py) { ensure_init(); G_CROP_STATES.with(|c| *c.borrow_mut() = v); }
pub fn g_CUE_CARRIERS() -> Py { ensure_init(); G_CUE_CARRIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUE_CARRIERS(v: Py) { ensure_init(); G_CUE_CARRIERS.with(|c| *c.borrow_mut() = v); }
pub fn g_DEGREE_NOUNS() -> Py { ensure_init(); G_DEGREE_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEGREE_NOUNS(v: Py) { ensure_init(); G_DEGREE_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_DERIVED_ENDINGS() -> Py { ensure_init(); G_DERIVED_ENDINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DERIVED_ENDINGS(v: Py) { ensure_init(); G_DERIVED_ENDINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINERS() -> Py { ensure_init(); G_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); G_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_DRIVEN_JOINERS() -> Py { ensure_init(); G_DRIVEN_JOINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRIVEN_JOINERS(v: Py) { ensure_init(); G_DRIVEN_JOINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_DRIVEN_MOOD() -> Py { ensure_init(); G_DRIVEN_MOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRIVEN_MOOD(v: Py) { ensure_init(); G_DRIVEN_MOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_DRIVEN_PERSON() -> Py { ensure_init(); G_DRIVEN_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRIVEN_PERSON(v: Py) { ensure_init(); G_DRIVEN_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_DRIVEN_TAIL() -> Py { ensure_init(); G_DRIVEN_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRIVEN_TAIL(v: Py) { ensure_init(); G_DRIVEN_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_DRIVES() -> Py { ensure_init(); G_DRIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRIVES(v: Py) { ensure_init(); G_DRIVES.with(|c| *c.borrow_mut() = v); }
pub fn g_EXERCISE_AFTER() -> Py { ensure_init(); G_EXERCISE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXERCISE_AFTER(v: Py) { ensure_init(); G_EXERCISE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_EXERCISE_NOUNS() -> Py { ensure_init(); G_EXERCISE_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXERCISE_NOUNS(v: Py) { ensure_init(); G_EXERCISE_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_EXTRA_PEOPLE() -> Py { ensure_init(); G_EXTRA_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXTRA_PEOPLE(v: Py) { ensure_init(); G_EXTRA_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_PAIRS() -> Py { ensure_init(); G_FAMILY_PAIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_PAIRS(v: Py) { ensure_init(); G_FAMILY_PAIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_STEM() -> Py { ensure_init(); G_FAMILY_STEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_STEM(v: Py) { ensure_init(); G_FAMILY_STEM.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_WORD() -> Py { ensure_init(); G_FAMILY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_WORD(v: Py) { ensure_init(); G_FAMILY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_SIZE_WORDS() -> Py { ensure_init(); G_FIGURE_SIZE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_SIZE_WORDS(v: Py) { ensure_init(); G_FIGURE_SIZE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FRIEND_BOOST() -> Py { ensure_init(); G_FRIEND_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRIEND_BOOST(v: Py) { ensure_init(); G_FRIEND_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_FRIEND_WORDS() -> Py { ensure_init(); G_FRIEND_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRIEND_WORDS(v: Py) { ensure_init(); G_FRIEND_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GENERIC() -> Py { ensure_init(); G_GENERIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GENERIC(v: Py) { ensure_init(); G_GENERIC.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFTS_OR_OCCASIONS() -> Py { ensure_init(); G_GIFTS_OR_OCCASIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFTS_OR_OCCASIONS(v: Py) { ensure_init(); G_GIFTS_OR_OCCASIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_ALWAYS() -> Py { ensure_init(); G_GIFT_ALWAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_ALWAYS(v: Py) { ensure_init(); G_GIFT_ALWAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_ASKING() -> Py { ensure_init(); G_GIFT_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_ASKING(v: Py) { ensure_init(); G_GIFT_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_BOOST() -> Py { ensure_init(); G_GIFT_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_BOOST(v: Py) { ensure_init(); G_GIFT_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_OCCASION() -> Py { ensure_init(); G_GIFT_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_OCCASION(v: Py) { ensure_init(); G_GIFT_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_WANTING() -> Py { ensure_init(); G_GIFT_WANTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_WANTING(v: Py) { ensure_init(); G_GIFT_WANTING.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_WORDS() -> Py { ensure_init(); G_GIFT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_WORDS(v: Py) { ensure_init(); G_GIFT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVEN_UP_FOOD() -> Py { ensure_init(); G_GIVEN_UP_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVEN_UP_FOOD(v: Py) { ensure_init(); G_GIVEN_UP_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVE_DETERMINERS() -> Py { ensure_init(); G_GIVE_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVE_DETERMINERS(v: Py) { ensure_init(); G_GIVE_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVE_RELATIVE() -> Py { ensure_init(); G_GIVE_RELATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVE_RELATIVE(v: Py) { ensure_init(); G_GIVE_RELATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVE_THING() -> Py { ensure_init(); G_GIVE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVE_THING(v: Py) { ensure_init(); G_GIVE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVE_THINGS() -> Py { ensure_init(); G_GIVE_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVE_THINGS(v: Py) { ensure_init(); G_GIVE_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVE_TO() -> Py { ensure_init(); G_GIVE_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVE_TO(v: Py) { ensure_init(); G_GIVE_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVE_VERB() -> Py { ensure_init(); G_GIVE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVE_VERB(v: Py) { ensure_init(); G_GIVE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVING_UP() -> Py { ensure_init(); G_GIVING_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVING_UP(v: Py) { ensure_init(); G_GIVING_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_GIVING_UP_BOOST() -> Py { ensure_init(); G_GIVING_UP_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIVING_UP_BOOST(v: Py) { ensure_init(); G_GIVING_UP_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_GROWING_PLACES() -> Py { ensure_init(); G_GROWING_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROWING_PLACES(v: Py) { ensure_init(); G_GROWING_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAD_FOG() -> Py { ensure_init(); G_HEAD_FOG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAD_FOG(v: Py) { ensure_init(); G_HEAD_FOG.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAD_FOG_BOOST() -> Py { ensure_init(); G_HEAD_FOG_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAD_FOG_BOOST(v: Py) { ensure_init(); G_HEAD_FOG_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAD_FOG_REACH() -> Py { ensure_init(); G_HEAD_FOG_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAD_FOG_REACH(v: Py) { ensure_init(); G_HEAD_FOG_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAD_WORDS() -> Py { ensure_init(); G_HEAD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAD_WORDS(v: Py) { ensure_init(); G_HEAD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEARERS() -> Py { ensure_init(); G_HEARERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEARERS(v: Py) { ensure_init(); G_HEARERS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOT_WORDS() -> Py { ensure_init(); G_HOT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOT_WORDS(v: Py) { ensure_init(); G_HOT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_HURTS() -> Py { ensure_init(); G_HURTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURTS(v: Py) { ensure_init(); G_HURTS.with(|c| *c.borrow_mut() = v); }
pub fn g_HURT_BOOST() -> Py { ensure_init(); G_HURT_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURT_BOOST(v: Py) { ensure_init(); G_HURT_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_IDIOM_PHRASES() -> Py { ensure_init(); G_IDIOM_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDIOM_PHRASES(v: Py) { ensure_init(); G_IDIOM_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGEMENT_OF_RECORD() -> Py { ensure_init(); G_JUDGEMENT_OF_RECORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGEMENT_OF_RECORD(v: Py) { ensure_init(); G_JUDGEMENT_OF_RECORD.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADING() -> Py { ensure_init(); G_LEADING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADING(v: Py) { ensure_init(); G_LEADING.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVES() -> Py { ensure_init(); G_LIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVES(v: Py) { ensure_init(); G_LIVES.with(|c| *c.borrow_mut() = v); }
pub fn g_MACHINE_BIN_WORDS() -> Py { ensure_init(); G_MACHINE_BIN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MACHINE_BIN_WORDS(v: Py) { ensure_init(); G_MACHINE_BIN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MACHINE_WORD_CUES() -> Py { ensure_init(); G_MACHINE_WORD_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MACHINE_WORD_CUES(v: Py) { ensure_init(); G_MACHINE_WORD_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_MACHINE_WORD_PHRASES() -> Py { ensure_init(); G_MACHINE_WORD_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MACHINE_WORD_PHRASES(v: Py) { ensure_init(); G_MACHINE_WORD_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_MECHANISM() -> Py { ensure_init(); G_MECHANISM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MECHANISM(v: Py) { ensure_init(); G_MECHANISM.with(|c| *c.borrow_mut() = v); }
pub fn g_MECHANISM_HOW() -> Py { ensure_init(); G_MECHANISM_HOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MECHANISM_HOW(v: Py) { ensure_init(); G_MECHANISM_HOW.with(|c| *c.borrow_mut() = v); }
pub fn g_MIXED_FRACTION() -> Py { ensure_init(); G_MIXED_FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MIXED_FRACTION(v: Py) { ensure_init(); G_MIXED_FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g_MIXED_JOINERS() -> Py { ensure_init(); G_MIXED_JOINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MIXED_JOINERS(v: Py) { ensure_init(); G_MIXED_JOINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_TIGHT() -> Py { ensure_init(); G_MONEY_TIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_TIGHT(v: Py) { ensure_init(); G_MONEY_TIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_TIGHT_MAX() -> Py { ensure_init(); G_MONEY_TIGHT_MAX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_TIGHT_MAX(v: Py) { ensure_init(); G_MONEY_TIGHT_MAX.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_BREAKS() -> Py { ensure_init(); G_NAME_BREAKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_BREAKS(v: Py) { ensure_init(); G_NAME_BREAKS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_FRAMES() -> Py { ensure_init(); G_NAME_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_FRAMES(v: Py) { ensure_init(); G_NAME_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_NATURE_WORDS() -> Py { ensure_init(); G_NATURE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NATURE_WORDS(v: Py) { ensure_init(); G_NATURE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOSE_BLOCKED() -> Py { ensure_init(); G_NOSE_BLOCKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOSE_BLOCKED(v: Py) { ensure_init(); G_NOSE_BLOCKED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOSE_BLOCKED_BOOST() -> Py { ensure_init(); G_NOSE_BLOCKED_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOSE_BLOCKED_BOOST(v: Py) { ensure_init(); G_NOSE_BLOCKED_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_CUT_OUT() -> Py { ensure_init(); G_NOT_CUT_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_CUT_OUT(v: Py) { ensure_init(); G_NOT_CUT_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_PLACES() -> Py { ensure_init(); G_NOT_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_PLACES(v: Py) { ensure_init(); G_NOT_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THINGS() -> Py { ensure_init(); G_NOT_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THINGS(v: Py) { ensure_init(); G_NOT_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_WORDS() -> Py { ensure_init(); G_NOT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_WORDS(v: Py) { ensure_init(); G_NOT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOUNY_SIGNS() -> Py { ensure_init(); G_NOUNY_SIGNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOUNY_SIGNS(v: Py) { ensure_init(); G_NOUNY_SIGNS.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_FILLER() -> Py { ensure_init(); G_NUMBER_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_FILLER(v: Py) { ensure_init(); G_NUMBER_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_RE() -> Py { ensure_init(); G_NUMBER_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_RE(v: Py) { ensure_init(); G_NUMBER_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); G_NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); G_NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_IS_A_THING() -> Py { ensure_init(); G_ONE_IS_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_IS_A_THING(v: Py) { ensure_init(); G_ONE_IS_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLY_A_STOOL() -> Py { ensure_init(); G_ONLY_A_STOOL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLY_A_STOOL(v: Py) { ensure_init(); G_ONLY_A_STOOL.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDINARY_VERBS() -> Py { ensure_init(); G_ORDINARY_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDINARY_VERBS(v: Py) { ensure_init(); G_ORDINARY_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PERSON() -> Py { ensure_init(); G_OTHER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PERSON(v: Py) { ensure_init(); G_OTHER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNED_BY() -> Py { ensure_init(); G_OWNED_BY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNED_BY(v: Py) { ensure_init(); G_OWNED_BY.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNERS() -> Py { ensure_init(); G_OWNERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNERS(v: Py) { ensure_init(); G_OWNERS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNING_WORDS() -> Py { ensure_init(); G_OWNING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNING_WORDS(v: Py) { ensure_init(); G_OWNING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNS_THE_UNIT() -> Py { ensure_init(); G_OWNS_THE_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNS_THE_UNIT(v: Py) { ensure_init(); G_OWNS_THE_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_SELF() -> Py { ensure_init(); G_OWN_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_SELF(v: Py) { ensure_init(); G_OWN_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTS() -> Py { ensure_init(); G_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTS(v: Py) { ensure_init(); G_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_UP_BOOST() -> Py { ensure_init(); G_PART_UP_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_UP_BOOST(v: Py) { ensure_init(); G_PART_UP_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_TOPICS() -> Py { ensure_init(); G_PEOPLE_TOPICS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_TOPICS(v: Py) { ensure_init(); G_PEOPLE_TOPICS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_WORDS() -> Py { ensure_init(); G_PERSON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_WORDS(v: Py) { ensure_init(); G_PERSON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PETS() -> Py { ensure_init(); G_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PETS(v: Py) { ensure_init(); G_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_BOOST() -> Py { ensure_init(); G_PET_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_BOOST(v: Py) { ensure_init(); G_PET_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_PHRASES() -> Py { ensure_init(); G_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PHRASES(v: Py) { ensure_init(); G_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACES() -> Py { ensure_init(); G_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACES(v: Py) { ensure_init(); G_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_HOMOGRAPHS() -> Py { ensure_init(); G_PLACE_HOMOGRAPHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_HOMOGRAPHS(v: Py) { ensure_init(); G_PLACE_HOMOGRAPHS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_OVER() -> Py { ensure_init(); G_PLACE_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_OVER(v: Py) { ensure_init(); G_PLACE_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_THING() -> Py { ensure_init(); G_PLACE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_THING(v: Py) { ensure_init(); G_PLACE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_WORDS() -> Py { ensure_init(); G_PLACE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_WORDS(v: Py) { ensure_init(); G_PLACE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_WORDS_WITH_CUE() -> Py { ensure_init(); G_PLACE_WORDS_WITH_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_WORDS_WITH_CUE(v: Py) { ensure_init(); G_PLACE_WORDS_WITH_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAYS_UP_BEFORE() -> Py { ensure_init(); G_PLAYS_UP_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAYS_UP_BEFORE(v: Py) { ensure_init(); G_PLAYS_UP_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAY_UP_REACH() -> Py { ensure_init(); G_PLAY_UP_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAY_UP_REACH(v: Py) { ensure_init(); G_PLAY_UP_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_POLYSEMOUS() -> Py { ensure_init(); G_POLYSEMOUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POLYSEMOUS(v: Py) { ensure_init(); G_POLYSEMOUS.with(|c| *c.borrow_mut() = v); }
pub fn g_POLY_FORMS() -> Py { ensure_init(); G_POLY_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POLY_FORMS(v: Py) { ensure_init(); G_POLY_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_PORTION_UNITS() -> Py { ensure_init(); G_PORTION_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PORTION_UNITS(v: Py) { ensure_init(); G_PORTION_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_PORTION_WORDS() -> Py { ensure_init(); G_PORTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PORTION_WORDS(v: Py) { ensure_init(); G_PORTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATIONS() -> Py { ensure_init(); G_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATIONS(v: Py) { ensure_init(); G_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_OUT_BEFORE() -> Py { ensure_init(); G_RUN_OUT_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_OUT_BEFORE(v: Py) { ensure_init(); G_RUN_OUT_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAMPLE_WORDS() -> Py { ensure_init(); G_SAMPLE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAMPLE_WORDS(v: Py) { ensure_init(); G_SAMPLE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SCIENCE_BOOST() -> Py { ensure_init(); G_SCIENCE_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SCIENCE_BOOST(v: Py) { ensure_init(); G_SCIENCE_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_SEASONS() -> Py { ensure_init(); G_SEASONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SEASONS(v: Py) { ensure_init(); G_SEASONS.with(|c| *c.borrow_mut() = v); }
pub fn g_SENSE_ELSEWHERE() -> Py { ensure_init(); G_SENSE_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENSE_ELSEWHERE(v: Py) { ensure_init(); G_SENSE_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_SENSE_ELSEWHERE_FORMS() -> Py { ensure_init(); G_SENSE_ELSEWHERE_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENSE_ELSEWHERE_FORMS(v: Py) { ensure_init(); G_SENSE_ELSEWHERE_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_SENSE_NEAR() -> Py { ensure_init(); G_SENSE_NEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENSE_NEAR(v: Py) { ensure_init(); G_SENSE_NEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_SENSE_PAIRS() -> Py { ensure_init(); G_SENSE_PAIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENSE_PAIRS(v: Py) { ensure_init(); G_SENSE_PAIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_SENSE_REACH() -> Py { ensure_init(); G_SENSE_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENSE_REACH(v: Py) { ensure_init(); G_SENSE_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_SENSE_WEIGHT() -> Py { ensure_init(); G_SENSE_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENSE_WEIGHT(v: Py) { ensure_init(); G_SENSE_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGN_FOLLOWS() -> Py { ensure_init(); G_SIGN_FOLLOWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGN_FOLLOWS(v: Py) { ensure_init(); G_SIGN_FOLLOWS.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGN_NOT_AFTER() -> Py { ensure_init(); G_SIGN_NOT_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGN_NOT_AFTER(v: Py) { ensure_init(); G_SIGN_NOT_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_SKIN_BOOST() -> Py { ensure_init(); G_SKIN_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SKIN_BOOST(v: Py) { ensure_init(); G_SKIN_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_SKIN_PARTS() -> Py { ensure_init(); G_SKIN_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SKIN_PARTS(v: Py) { ensure_init(); G_SKIN_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SKIN_REACH() -> Py { ensure_init(); G_SKIN_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SKIN_REACH(v: Py) { ensure_init(); G_SKIN_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_SKIN_STATES() -> Py { ensure_init(); G_SKIN_STATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SKIN_STATES(v: Py) { ensure_init(); G_SKIN_STATES.with(|c| *c.borrow_mut() = v); }
pub fn g_STONE_COMPOUNDS() -> Py { ensure_init(); G_STONE_COMPOUNDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STONE_COMPOUNDS(v: Py) { ensure_init(); G_STONE_COMPOUNDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STONE_VERBS() -> Py { ensure_init(); G_STONE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STONE_VERBS(v: Py) { ensure_init(); G_STONE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_STOOL_AS_FURNITURE() -> Py { ensure_init(); G_STOOL_AS_FURNITURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOOL_AS_FURNITURE(v: Py) { ensure_init(); G_STOOL_AS_FURNITURE.with(|c| *c.borrow_mut() = v); }
pub fn g_STRONG() -> Py { ensure_init(); G_STRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STRONG(v: Py) { ensure_init(); G_STRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_STUDY_OBJECTS() -> Py { ensure_init(); G_STUDY_OBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STUDY_OBJECTS(v: Py) { ensure_init(); G_STUDY_OBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUMS_AFTER() -> Py { ensure_init(); G_SUMS_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUMS_AFTER(v: Py) { ensure_init(); G_SUMS_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_SUM_MONEY() -> Py { ensure_init(); G_SUM_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUM_MONEY(v: Py) { ensure_init(); G_SUM_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_TABLE() -> Py { ensure_init(); G_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TABLE(v: Py) { ensure_init(); G_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_TEETHING_AND_A_CHILD() -> Py { ensure_init(); G_TEETHING_AND_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEETHING_AND_A_CHILD(v: Py) { ensure_init(); G_TEETHING_AND_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_TEETHING_IS_A_FAULT() -> Py { ensure_init(); G_TEETHING_IS_A_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEETHING_IS_A_FAULT(v: Py) { ensure_init(); G_TEETHING_IS_A_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PEOPLE() -> Py { ensure_init(); G_THE_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PEOPLE(v: Py) { ensure_init(); G_THE_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PERSONS_OWN() -> Py { ensure_init(); G_THE_PERSONS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PERSONS_OWN(v: Py) { ensure_init(); G_THE_PERSONS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_THINGS() -> Py { ensure_init(); G_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THINGS(v: Py) { ensure_init(); G_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_WORDS() -> Py { ensure_init(); G_THING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_WORDS(v: Py) { ensure_init(); G_THING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAVEL_CUES() -> Py { ensure_init(); G_TRAVEL_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAVEL_CUES(v: Py) { ensure_init(); G_TRAVEL_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAVEL_STRONG() -> Py { ensure_init(); G_TRAVEL_STRONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAVEL_STRONG(v: Py) { ensure_init(); G_TRAVEL_STRONG.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_HOMES() -> Py { ensure_init(); G_TWO_HOMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_HOMES(v: Py) { ensure_init(); G_TWO_HOMES.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_LIVES() -> Py { ensure_init(); G_TWO_LIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_LIVES(v: Py) { ensure_init(); G_TWO_LIVES.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_LIVES_FORMS() -> Py { ensure_init(); G_TWO_LIVES_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_LIVES_FORMS(v: Py) { ensure_init(); G_TWO_LIVES_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_LIVES_REACH() -> Py { ensure_init(); G_TWO_LIVES_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_LIVES_REACH(v: Py) { ensure_init(); G_TWO_LIVES_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_WORDS() -> Py { ensure_init(); G_UNIT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_WORDS(v: Py) { ensure_init(); G_UNIT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNWELL_BOOST() -> Py { ensure_init(); G_UNWELL_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNWELL_BOOST(v: Py) { ensure_init(); G_UNWELL_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_UNWELL_IDIOMS() -> Py { ensure_init(); G_UNWELL_IDIOMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNWELL_IDIOMS(v: Py) { ensure_init(); G_UNWELL_IDIOMS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNWELL_NOT() -> Py { ensure_init(); G_UNWELL_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNWELL_NOT(v: Py) { ensure_init(); G_UNWELL_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_UNWELL_SIGNS() -> Py { ensure_init(); G_UNWELL_SIGNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNWELL_SIGNS(v: Py) { ensure_init(); G_UNWELL_SIGNS.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB_CUES() -> Py { ensure_init(); G_VERB_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB_CUES(v: Py) { ensure_init(); G_VERB_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_NOUNS() -> Py { ensure_init(); G_VERDICT_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_NOUNS(v: Py) { ensure_init(); G_VERDICT_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTED_BEFORE_A_STOP() -> Py { ensure_init(); G_WANTED_BEFORE_A_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTED_BEFORE_A_STOP(v: Py) { ensure_init(); G_WANTED_BEFORE_A_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_WANT_VERBS() -> Py { ensure_init(); G_WANT_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANT_VERBS(v: Py) { ensure_init(); G_WANT_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_WARMTH_ARE() -> Py { ensure_init(); G_WARMTH_ARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WARMTH_ARE(v: Py) { ensure_init(); G_WARMTH_ARE.with(|c| *c.borrow_mut() = v); }
pub fn g_WARMTH_DEGREE() -> Py { ensure_init(); G_WARMTH_DEGREE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WARMTH_DEGREE(v: Py) { ensure_init(); G_WARMTH_DEGREE.with(|c| *c.borrow_mut() = v); }
pub fn g_WARMTH_HABITUAL() -> Py { ensure_init(); G_WARMTH_HABITUAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WARMTH_HABITUAL(v: Py) { ensure_init(); G_WARMTH_HABITUAL.with(|c| *c.borrow_mut() = v); }
pub fn g_WARMTH_THEN() -> Py { ensure_init(); G_WARMTH_THEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WARMTH_THEN(v: Py) { ensure_init(); G_WARMTH_THEN.with(|c| *c.borrow_mut() = v); }
pub fn g_WARMTH_WORDS() -> Py { ensure_init(); G_WARMTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WARMTH_WORDS(v: Py) { ensure_init(); G_WARMTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WATCHING() -> Py { ensure_init(); G_WATCHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATCHING(v: Py) { ensure_init(); G_WATCHING.with(|c| *c.borrow_mut() = v); }
pub fn g_WATCH_OUT_AFTER() -> Py { ensure_init(); G_WATCH_OUT_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATCH_OUT_AFTER(v: Py) { ensure_init(); G_WATCH_OUT_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_WEAK() -> Py { ensure_init(); G_WEAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEAK(v: Py) { ensure_init(); G_WEAK.with(|c| *c.borrow_mut() = v); }
pub fn g_WEATHER_BEFORE() -> Py { ensure_init(); G_WEATHER_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEATHER_BEFORE(v: Py) { ensure_init(); G_WEATHER_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_WEATHER_ITSELF() -> Py { ensure_init(); G_WEATHER_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEATHER_ITSELF(v: Py) { ensure_init(); G_WEATHER_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_DIGITS() -> Py { ensure_init(); G_WORD_DIGITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_DIGITS(v: Py) { ensure_init(); G_WORD_DIGITS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD_NUMBERS() -> Py { ensure_init(); G_WORD_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD_NUMBERS(v: Py) { ensure_init(); G_WORD_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK_WORDS() -> Py { ensure_init(); G_WORK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK_WORDS(v: Py) { ensure_init(); G_WORK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORLD() -> Py { ensure_init(); G_WORLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORLD(v: Py) { ensure_init(); G_WORLD.with(|c| *c.borrow_mut() = v); }
pub fn g_WORLD_NOT() -> Py { ensure_init(); G_WORLD_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORLD_NOT(v: Py) { ensure_init(); G_WORLD_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_WORN_OUT() -> Py { ensure_init(); G_WORN_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORN_OUT(v: Py) { ensure_init(); G_WORN_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_WORN_OUT_BOOST() -> Py { ensure_init(); G_WORN_OUT_BOOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORN_OUT_BOOST(v: Py) { ensure_init(); G_WORN_OUT_BOOST.with(|c| *c.borrow_mut() = v); }
pub fn g_WORN_OUT_CUE() -> Py { ensure_init(); G_WORN_OUT_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORN_OUT_CUE(v: Py) { ensure_init(); G_WORN_OUT_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORN_OUT_LOOKBACK() -> Py { ensure_init(); G_WORN_OUT_LOOKBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORN_OUT_LOOKBACK(v: Py) { ensure_init(); G_WORN_OUT_LOOKBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_WORN_OUT_SAID() -> Py { ensure_init(); G_WORN_OUT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORN_OUT_SAID(v: Py) { ensure_init(); G_WORN_OUT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STATE_VERB() -> Py { ensure_init(); G__A_STATE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STATE_VERB(v: Py) { ensure_init(); G__A_STATE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__BREAKS() -> Py { ensure_init(); G__BREAKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREAKS(v: Py) { ensure_init(); G__BREAKS.with(|c| *c.borrow_mut() = v); }
pub fn g__FUNCTION_WORDS() -> Py { ensure_init(); G__FUNCTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FUNCTION_WORDS(v: Py) { ensure_init(); G__FUNCTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__GRAND_RELATIONS() -> Py { ensure_init(); G__GRAND_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GRAND_RELATIONS(v: Py) { ensure_init(); G__GRAND_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__KIN_BUILT() -> Py { ensure_init(); G__KIN_BUILT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KIN_BUILT(v: Py) { ensure_init(); G__KIN_BUILT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_SUBJECT() -> Py { ensure_init(); G__NOT_THE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_SUBJECT(v: Py) { ensure_init(); G__NOT_THE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__POSSESSIVE() -> Py { ensure_init(); G__POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POSSESSIVE(v: Py) { ensure_init(); G__POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__SENSE_TIERS() -> Py { ensure_init(); G__SENSE_TIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SENSE_TIERS(v: Py) { ensure_init(); G__SENSE_TIERS.with(|c| *c.borrow_mut() = v); }
pub fn g__STEPPED_RELATIONS() -> Py { ensure_init(); G__STEPPED_RELATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEPPED_RELATIONS(v: Py) { ensure_init(); G__STEPPED_RELATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_OWNER_SETTLES() -> Py { ensure_init(); G__THE_OWNER_SETTLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_OWNER_SETTLES(v: Py) { ensure_init(); G__THE_OWNER_SETTLES.with(|c| *c.borrow_mut() = v); }
pub fn g__form() -> Py { ensure_init(); G__form.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__form(v: Py) { ensure_init(); G__form.with(|c| *c.borrow_mut() = v); }
pub fn g__forms() -> Py { ensure_init(); G__forms.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__forms(v: Py) { ensure_init(); G__forms.with(|c| *c.borrow_mut() = v); }
pub fn g__sense_topic() -> Py { ensure_init(); G__sense_topic.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__sense_topic(v: Py) { ensure_init(); G__sense_topic.with(|c| *c.borrow_mut() = v); }
pub fn g__tiers() -> Py { ensure_init(); G__tiers.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__tiers(v: Py) { ensure_init(); G__tiers.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What the line is about: the topic, the things it names, and the numbers in it.\n\nData first: every topic is a small vocabulary of distinctive words (\"strong\"), ordinary words\n(\"weak\") and short phrases. A match scores 2 for a strong word, 3 for a phrase and 1 for a\nweak word, so a distinctive word beats a generic one, and a line only gets a topic when\nsomething in it really points that way. Ties go to the topic mentioned last, because people\nend a line with what they actually want.\n\nEntities come from what the line names: capitalised names (\"Lisbon\", \"Gone Girl\"), the people\nit mentions (\"my daughter\"), quoted titles, and the everyday things in the vocabulary above.\nNumbers come with their unit when the line gives one (\"2kg\", \"20 minutes\", \"15%\").\n\nThe understanding lane owns this file; extend a vocabulary rather than special-casing a line.\n");
    s_A_GUT_ONLY(crate::understanding::body::g_A_GUT_ONLY());
    s_ADDRESS_WORDS(crate::understanding::kinds::g_ADDRESS_WORDS());
    s_VERDICT_NOUNS(crate::understanding::kinds::g_VERDICT_NOUNS());
    s_COMMON_WORDS(crate::understanding::normalize::g_LEXICON());
    s_STRONG(py_dict(vec![(py_str("cooking"), py_str("cook cooking recipe recipes bake baking boiled boil fry frying roast roasted\n        pudding puddings bbq\n        grill simmer oven hob pan pot saucepan whisk knead dough batter marinade seasoning\n        chop chopped chopped stir fry leftover leftovers sourdough pastry cake cupcakes\n        biscuits bread pizza pasta curry soup stew rice risotto noodles sauce gravy stuffing\n        roast scrambled poached omelette pancake pancakes lasagne lasagna meatballs\n        undercooked overcooked sticky mushy crispy seasoning vegan vegetarian cheese\n        dairy gluten lactose nut soya soy wheat\n        cheeses chicken beef pork lamb mince bacon sausages ham fish salmon tuna prawns\n        potato potatoes tomato tomatoes onion onions garlic ginger carrot carrots\n        cabbage leeks cauliflowers swede peas celery cucumber lettuce beetroot sprouts pepper\n        peppers mushroom mushrooms spinach courgette courgettes broccoli beans lentils\n        chickpeas pasta spaghetti noodles rice flour sugar butter egg eggs milk cream\n        lemon lemons lime limes orange herbs basil thyme rosemary parsley coriander cumin\n        paprika cinnamon vanilla honey mustard mayonnaise yoghurt oil vinegar stock\n        casserole tray tin whisk blender frying baking grill saucepan saucepan dish\n        defrost thaw preheat simmering frying boiling recipes portions portion serving\n        brine brining deseed deseeded meringue meringues pavlova guacamole milkshake\n        milkshakes melon melons burrito burritos\n        refreeze cabbage cauliflower kale lettuce sprouts peas veggie veggies pie quiche\n        shortbread sponge scones icing frosting buttercream jam pesto hummus couscous halloumi\n        buttermilk pudding puddings bolognese bol spag chilli chips mash loaf yeast\n        parboil thicken reheat bland soggy rubbery watery steak chops breast thighs burgers meat\n tablespoon dessertspoon rinse tinned muffins muffin custard flapjacks flapjack popcorn cheesecake brownies brownie\n        caramel tzatziki wok jelly aioli clams crumble meringue sorbet coleslaw\n        burgers pineapple oats\n turkey knives cheddar quinoa scales sweetcorn mussels burger chinese indian grainy")), (py_str("food-and-drink"), py_str("eat eating food meal breakfast lunch dinner snack snacks drink drinking\n        supper brunch dessert pud afters nibbles elevenses brekkie brekky sarnie\n        butty cuppa takeout nosh chow kebab chippy munchies bap sanger\n        coffee tea water juice wine beer sugar salt pepper cheese butter milk yogurt cream\n        vegan vegetarian gluten allergy allergic caffeine decaf fizzy chocolate crisps\n        takeaway restaurant menu hungry thirsty tasty delicious bread toast sandwiches\n        sandwich cereal porridge yoghurt smoothie chocolate crisps nuts fruit apple apples\n        banana bananas orange strawberries berries salad salad dressing mouldy moldy\n leftovers reheating leftovers portions\n        ripe granola")), (py_str("health"), py_str("doctor gp hospital nurse pharmacy medicine tablet tablets pill pills symptom\n        symptoms pain ache ill illness sick unwell fever temperature cold flu cough sneeze\n        headache headaches migraine throat sore throat bleeding bleed cut burn burnt bruise\n        infection antibiotics vaccine vaccination allergy allergic dizziness dizzy nausea\n        blood pressure appointment clinic injury prescription eyes eye vision headache\n        migraines dentist dental toothache teeth stroke numb numbness slurred\n        seizure fainted fainting sprain sprained swelling swollen blister blisters\n        sliced rash itchy itch vomiting vomit puking puke puked barf barfing diarrhoea constipation\n        heartburn indigestion reflux\n        cramps cramp bloated congestion earache ankle wrist elbow shoulder knee hip\n        thumb finger fingers toe toes spine joint tendon ligament stomach calpol nurofen\n        ibuprofen\n        paracetamol painkiller painkillers cyst infected hangover hangovers\n        nostrils nostril sinus sinuses sniffles sniffly stinging stings stung hoarse\n        shivery chills queasy laryngitis tonsils wheezy croaky phlegmy smoking braces\n        cholesterol heal healing snacking aching aches hurting nails toothbrush fibre\n        fasting rundown mole moles acne hayfever inhaler inhalers colic palpitations antidepressants\n        antidepressant dandruff floss plaque smear nits verruca verrucas wart warts\n cystitis menopause measles mumps chickenpox tonsillitis earache toenail\n        toenails adhd autism dyslexia ocd ptsd sunscreen aftersun blistering\n        pregnancy vitamin limping")), (py_str("fitness"), py_str("run running jog jogging gym workout exercise exercises stretch stretching\n        training train fitness muscle muscles cardio weights lifting squat squats push-ups\n plank cycling swim swimming yoga pilates marathon 10k 5k steps calories toned\n        insoles reps pace sets")), (py_str("sleep"), py_str("sleep sleeping slept insomnia bedtime nap naps wake waking woke tired exhausted\n mattress pillow duvet toddler wakes snore snoring dreams nightmare melatonin")), (py_str("money"), py_str("money bank savings save saving budget budgeting salary wage wages pay paid\n        loan loans mortgage rent debt debts credit debit card overdraft pension\n        invest investing insurance tax taxes bills bill invoice receipt refund expensive\n        cheap afford spending spent pound pounds euro euros dollar dollars cash atm council\n        owes owed supplier payday payment payments doubled income lease vat switching\n deposit skint overcharging overcharged isa retire retired retirement subscriptions allowance overdrawn bailiff bailiffs\n        dividend dividends apr payslip payslips annuity\nbenefit declare hmrc")), (py_str("work"), py_str("job jobs work working boss manager colleague colleagues office interview cv\n        resume salary promotion review meeting meetings deadline project team hire hiring\n        fired quit resign resignation contract shift shifts overtime coworker workplace\n        conference presentation badge rota roster timesheet timesheets appraisal\n payslip warehouse supervisor defensive\n        employer reference probation earlies sacked staff published")), (py_str("school-and-study"), py_str("school exam exams revise revising revision study studying homework\n        teacher class lesson lessons term semester university college student grades grade\n        test tests essays essay coursework dissertation notebook learn learning\n        spelling vocabulary subject tutor a-level a-levels alevel alevels gcse\n        gcses apprenticeship apprenticeships lecture lectures seminar\n seminars memorise memorising memorised cram cramming timetable\n        uni squared levels phd ucas factorise accountant thesis courses maintenance equations lab")), (py_str("science"), py_str("science scientific physics chemistry biology atoms molecules gravity energy\n        evolution photosynthesis oxygen carbon climate vaccines virus bacteria cells species\n        experiment theory hypothesis dna planets jupiter saturn mars venus mercury\n orbit orbits seasons solar astronomy telescope moon moons\n        tan planet thunder lava reflect blink shiver freckles digestion conductor insulator galaxy skyscrapers berry")), (py_str("history"), py_str("history historical century ancient empire revolution war wars king queen\n        pharaoh pyramids roman romans greeks medieval victorian industrial\n archaeology civilisation civilization darwin einstein freud waterloo tsar lenin bismarck attila napoleon\n        boleyn tudor tudors victorians aztec aztecs inca vikings\n        invented britain blitz hitler marx antoinette curie crusades")), (py_str("geography-and-travel"), py_str("travel travelling traveling trip holiday holidays flight fly\n        flying airport boarding plane train station passport luggage packing pack suitcase hotel\n        booking itinerary visa abroad city countries country capital jet lag jetlag map\n tour tourist sightseeing beach mountains island abroad drive driving\n        spain rome cornwall lisbon ferry edinburgh paris portugal london croatia coast naples amalfi airline cabin metro mexico heathrow prague pompeii benidorm")), (py_str("technology"), py_str("phone laptop computer tablet screen keyboard battery charger wifi wi-fi\n        internet email app apps website software password passwords download upload update\n        install printer router bluetooth notification notifications file files folder backup\n        virus hacking privacy screenshot log login account data iphone android mac macbook\n        photos delete uninstall restart spam scam texting pointer icloud zoom\n        broadband headphones earphones robocall robocalls landline\n        \n        ipad signal earbuds pdf blurry monitor sim print facebook ssd speaker usb spreadsheet vlookup teams google cloud youtube photograph")), (py_str("home-and-diy"), py_str("house flat apartment kitchen bathroom bedroom ceiling wall walls floor\n        carpet paint painting diy drill hammer screw screws shelf cupboard radiator boiler\n        sink tap drain drains mould mold damp leak leaking plumber electrician fuse\n        furniture sofa bed washing machine dishwasher hoover dyson vacuum suction filter\n        ironing ironed curtain curtains\n        moving move removal landlord toilet flush flushing grout tile tiles shower smoke alarm\n        carbon monoxide draught draughts door doors window windows lock locks thermostat\n        heating freezer fridge kettle draft loft gutter guttering locksmith doorbell\n        burner burners stove stoves fireplace chimney flue hearth worktop worktops dryer tumble\n        jeans shirt shirts jumper towels stain stains condensation limescale kettling extractor\n radiators gutters fence plasterboard drum loader wooden spilt pipes\n wallpaper wallpapering decking wardrobe wardrobes patio toaster cistern ladder ladders tenancy chipped varnish\n        blinds cutlery crockery doormat skirting tiling umbrellas knit knitted scarf\n        recycling rug bins bedding softener stool")), (py_str("garden-and-plants"), py_str("garden gardening plant plants planting grow growing seed seeds\n        seedling seedlings soil compost weed weeds weeding watering pruning prune lawn grass\n        flower flowers bulbs tulip tulips basil tomato tomatoes balcony allotment greenhouse\n        pots shrubs shrub hedge hedges turf mower rake hoe mulch repot repotting\n        deadhead deadheading cuttings propagate houseplant houseplants monstera orchid orchids\n        cactus cacti succulent succulents fern ferns lily lilies aloe lavender hydrangea\n        daffodil daffodils sunflower sunflowers composted harvest harvesting harvested\n        sow sows sowing sowed slug slugs snail snails\n        roses moss lawnmower vase bonsai bamboo baskets basket chit")), (py_str("pets"), py_str("pet pets dog dogs puppy cat cats kitten kittens vet vet rabbit rabbits hamster\n        goldfish fish tank lead leash walkies dog lead kitten litter kennel rescue adopted\n adopt purring barking bark treats barks fleas collar paw poisonous\n        labrador greyhound guinea whippet spaniel pug pig neutering")), (py_str("family-and-parenting"), py_str("child children kid kids baby toddler toddler son daughter\n        parent parents mum mom mother dad father husband wife partner family babies nappy nappies\n        pram nursery babysit twins birthday party school run parenting newborn\n        toddler toddlers potty weaning teething playgroup playschool childminder\n        gran grans sister sisters brother brothers sibling siblings grandma grandmas granny grandad\n        grandpa grandparents grandmother grandfather grandson granddaughter grandchild\n        grandchildren nan nana nanna nephew nephews niece nieces cousin cousins aunt aunty\n        auntie uncle uncles stepmother stepfather stepmum stepdad stepson stepdaughter\n        stepchild stepchildren stepsister stepbrother half-brother half-sister in-laws\n relatives ex exes\n        teen invitations godparent babysitting hymns boys")), (py_str("friends-and-relationships"), py_str("friend friends friendship partner boyfriend girlfriend\n        wife husband marry married wedding dating date anniversary argument argued row\n        apology apologise apologize forgive trust relationship relationships breakup\n        neighbour neighbours colleague flatmate housemate ex exes\n divorce divorced separation separated cheat cheated cheating affair affairs")), (py_str("shopping"), py_str("buy buying bought shop shopping store price prices cheap expensive\n        sale discount refund warranty returns second-hand reviews review order\n delivery parcel stock size sizes brand\n moisturiser housewarming bleaching")), (py_str("cars"), py_str("car cars tyre tyres tire tires mot brakes brake braking engine engines clutch\n        gearbox windscreen windshield wipers wiper headlights headlight bonnet coolant\n        dipstick tread puncture petrol diesel mpg speedometer dashboard mechanic garage\n servicing motoring motorway alloy alloys demist\n        ebike pcn bumper pedal penalty motor")), (py_str("entertainment"), py_str("film films movie movies watch watching series tv television show\n        shows netflix book books novel novels read reading podcast podcasts music song songs\n        album band concert theatre theatre game games play console chess draughts\n        backgammon scrabble monopoly jigsaw jigsaws\n        documentary comedy comedy episode episodes season author writer actor actress\n streaming\n chords amp iplayer succession western jazz")), (py_str("sport"), py_str("sport sports football footy footie soccer rugby cricket tennis golf match\n        matches team\n        league cup goal goals offside wicket wickets innings coach coaching training\n        bowling yorker googly bouncer crease stumps batsman batsmen wicketkeeper keeper\n        olympics tennis badminton swimming race racing formula tennis wimbledon\n player players referee darts snooker hockey netball volleyball athletics\n        basketball baseball softball handball squash boxing marathon marathons triathlon\n        skiing surfing rowing bowls\n        hamstrings")), (py_str("language-and-writing"), py_str("grammar spelling word words writing wrote write sentence\n        sentences essay paragraph punctuation apostrophe comma vocabulary phrase\n        wording translate translation english language meaning affect effect fewer less\n email draft letter poem speech bio caption congratulations plural plurals semicolon semicolons colon voicemail thesaurus dictionary mumbling\n        monotone\n        spell fluent exclamation bullet rewrite")), (py_str("feelings"), py_str("feel feeling felt emotional lonely alone sad unhappy upset anxious worried\n        nervous stressed stress overwhelmed depressed crying tears heartbroken gutted angry\n        annoyed frustrated guilty ashamed jealous embarrassed happy excited relieved proud\n grateful mood feelings grief grieving funeral devastated\n        coping knots churning")), (py_str("small-talk"), py_str("jigsaw chatty catch chat catch-up catchup rain raining sunny freezing\n chilly warm cold weather bye lovely\n easter")), (py_str("opinions"), py_str("opinion favourite favorite prefer rather\n worth overrated underrated agree disagree")), (py_str("creative"), py_str("poem poetry story stories joke jokes nickname creative\n write writing speech rhyme character plot lyrics slogan tagline design\n        limerick haiku rap")), (py_str("maths-and-numbers"), py_str("percent percentage maths math numbers calculate sum total average\n        divide multiply minus plus equation fraction fractions decimals half double triple\n algebra geometry numerator denominator perimeter theorem pythagoras protractor diameter radius\n        isosceles quadrilateral quadrilaterals parallelogram rhombus trapezium cuboid\n        triangles triangle probability median")), (py_str("nature-and-weather"), py_str("weather rain raining snow snowing storm thunderstorm lightning\n        wind windy sunny sunshine clouds cloudy fog foggy freezing frost heatwave forecast\n        autumn spring winter summer birds heron robin sparrow wildlife trees nature\n river sea ocean beach stars moon\n        animal whale cheetah penguins penguin")), (py_str("about-the-bot"), py_str("chatbot bot robot artificial intelligence human person alive feelings\n remember yourself yourself\n        version chatgpt soul"))]));
    let __aug1 = op_add(&geti(&g_STRONG(), &py_str("home-and-diy")), &py_str("\n        attic cellar mirror mirrors picture pictures frame frames lamp lamps\n        socket sockets plug plugs wiring hook hooks armchair armchairs"));
    let _ = py_setitem(&g_STRONG(), &py_str("home-and-diy"), __aug1);
    s_WEAK(py_dict(vec![(py_str("cooking"), py_str("make made time minutes hours temperature heat hot plate bowl spoon knife cookies dinner tonight lunch breakfast tastes frozen handful freeze")), (py_str("food-and-drink"), py_str("like dislike sweet sour spicy fresh cold hot bottle cup glass")), (py_str("health"), py_str("feel better worse days week rest sleep tired checked")), (py_str("fitness"), py_str("minute minutes week day morning evening body")), (py_str("sleep"), py_str("night morning bed room")), (py_str("money"), py_str("much cost worth pay month year percent save owe minimum")), (py_str("work"), py_str("day week hours people ask")), (py_str("school-and-study"), py_str("help practice remember book week day")), (py_str("science"), py_str("why works explain")), (py_str("history"), py_str("time people built")), (py_str("geography-and-travel"), py_str("go going place days week month")), (py_str("technology"), py_str("work working broken fix new old connect storage charging broadband")), (py_str("home-and-diy"), py_str("room house fix clean door doors window windows repairing replace serviced bleach smells smelling dripping banging rattling humming stinks stinking squeaking leather patch")), (py_str("garden-and-plants"), py_str("water green yellow leaves grow")), (py_str("pets"), py_str("feed cat dog walk scratching")), (py_str("family-and-parenting"), py_str("home help age")), (py_str("friends-and-relationships"), py_str("talk said told mate mates pal pals buddy bloke fella")), (py_str("shopping"), py_str("need new look")), (py_str("cars"), py_str("start starting noise noisy sound light warning fix cost worth check insurance")), (py_str("entertainment"), py_str("good funny watch read listen")), (py_str("sport"), py_str("play watch season")), (py_str("language-and-writing"), py_str("say write help mean")), (py_str("feelings"), py_str("really very today week")), (py_str("small-talk"), py_str("today hello hi tomorrow day week morning evening busy tired")), (py_str("opinions"), py_str("think really")), (py_str("creative"), py_str("help idea")), (py_str("maths-and-numbers"), py_str("how many long")), (py_str("nature-and-weather"), py_str("today season year")), (py_str("about-the-bot"), py_str("you your help"))]));
    s_THING_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STRONG(), "values", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WEAK(), "values", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { let mut words: Py = __item.clone(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THING_WORDS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::learn_thing_words(args))(&__args) };
    s_SUMS_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("how what which if whether when where why much many long cost costs\n    budget rate rates salary pay bill bills price prices percentage percent total average\n    amount area size volume length distance mpg miles tax taxes loan loans owe owes\n    spend spending split share calories tiles paint dose timer timers worth number numbers\n    cheaper cheapest\n    the my our your his her their its this that these those"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NUMBER_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("one two three four five six seven eight nine ten eleven twelve\n    thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty thirty forty fifty\n    sixty seventy eighty ninety hundred thousand million billion half quarter"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SUM_MONEY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("pay paid salary wage wages tax taxes bill bills budget price prices\n    cost costs rate hourly income rent loan loans mortgage overtime notice pension fee fees\n    profit loss interest entitlement entitlements cheaper cheapest owe owes\n    debt debts refund refunds save savings total"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EXERCISE_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("gym home more often regularly times sets reps cardio weights\n    running fit fitness training body muscles arms arm legs leg abs back chest shoulders\n    glutes core biceps triceps thighs butt hard lightly daily weekly workout session\n    day days week weeks once twice\n    morning mornings evening evenings night afternoon today tomorrow"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EXERCISE_NOUNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("heart pulse zone calorie calories macro macros protein\n    rep reps weight weights pace treadmill lifting lift sets bodyfat bmi 1rm\n    km kilometre kilometres mile miles"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CANCELS(py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:do|does|did|would|will|should) (?:[a-z]+ ){0,3}?work\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("work")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow (?:it|they|this|that|these|those) works?\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("work")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:does|do|will|would|can|could) (?:it|that|this|they) work\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("work")]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat makes (?:[a-z]+ ){0,3}?work\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_str("work")])]));
    s_CANCEL_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    for with in on at to out up like if when as that this it than from about\n    and or but so then though because\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PHRASES(py_dict(vec![(py_str("cooking"), py_tuple(vec![py_str("packed lunch"), py_str("packed lunches"), py_str("party food"), py_str("lunch box"), py_str("lunchbox"), py_str("how long should i cook"), py_str("how do i cook"), py_str("what can i make"), py_str("what can i cook"), py_str("main course"), py_str("cooking for"), py_str("i am cooking"), py_str("cook for"), py_str("easy dinner"), py_str("quick dinner"), py_str("quick meal"), py_str("use up"), py_str("birthday cake"), py_str("something to cook"), py_str("cook tonight"), py_str("meal for"), py_str("sunday roast"), py_str("roast dinner for"), py_str("christmas dinner"), py_str("yorkshire pudding"), py_str("yorkshire puddings"), py_str("pressure cooker"), py_str("slow cooker"), py_str("air fryer"), py_str("leftover"), py_str("meal idea"), py_str("meal ideas"), py_str("dinner idea"), py_str("dinner ideas"), py_str("something quick"), py_str("under cooked"), py_str("ice cream"), py_str("chopping board"), py_str("chopping boards"), py_str("cutting board"), py_str("cutting boards")])), (py_str("food-and-drink"), py_tuple(vec![py_str("book a table"), py_str("a table for"), py_str("table for two"), py_str("table for four"), py_str("high in iron"), py_str("iron rich"), py_str("rich in iron"), py_str("low in iron"), py_str("low iron"), py_str("iron levels"), py_str("iron level"), py_str("iron deficiency"), py_str("deficient in iron"), py_str("more iron"), py_str("iron in"), py_str("sources of iron"), py_str("getting enough iron"), py_str("enough iron"), py_str("need iron"), py_str("anaemic"), py_str("anaemia"), py_str("anemia"), py_str("anemic"), py_str("eat out"), py_str("go out for dinner"), py_str("too much coffee"), py_str("less coffee"), py_str("ice cream"), py_str("is it safe to eat"), py_str("out of date"), py_str("goes well with"), py_str("cheese board"), py_str("graze board"), py_str("graze boards"), py_str("picky bits"), py_str("picky tea"), py_str("main course"), py_str("use by date"), py_str("sell by date"), py_str("best before"), py_str("past its date"), py_str("in season")])), (py_str("health"), py_tuple(vec![py_str("see a doctor"), py_str("go to the doctor"), py_str("should i see a doctor"), py_str("sore throat"), py_str("root canal"), py_str("mole checked"), py_str("throwing up"), py_str("throws up"), py_str("threw up"), py_str("thrown up"), py_str("want to throw up"), py_str("going to throw up"), py_str("gonna throw up"), py_str("high temperature"), py_str("blood pressure"), py_str("cut my finger"), py_str("feeling ill"), py_str("feel unwell"), py_str("get headaches"), py_str("have a headache"), py_str("my head hurts"), py_str("making me cry"), py_str("stopped bleeding"), py_str("wont stop bleeding"), py_str("signs of a stroke"), py_str("burnt my hand"), py_str("burnt myself"), py_str("burnt my arm"), py_str("burnt my finger"), py_str("coming down with"), py_str("going down with"), py_str("runny nose"), py_str("stuffy nose"), py_str("blocked nose"), py_str("bunged up"), py_str("bunged nose"), py_str("blocked ear"), py_str("blocked ears"), py_str("ear is blocked"), py_str("ears are blocked"), py_str("ear infection"), py_str("sore ears"), py_str("ringing in my ears"), py_str("ears are ringing"), py_str("stuffy head"), py_str("scratchy throat"), py_str("sinus infection"), py_str("my nose is blocked"), py_str("my nose is running"), py_str("my nose is bunged"), py_str("my nose is stuffy"), py_str("nose bleed"), py_str("nose bleeds"), py_str("nosebleed"), py_str("chest congestion"), py_str("my chest is congested"), py_str("congested chest"), py_str("nasal congestion"), py_str("sinus congestion"), py_str("congested nose"), py_str("im congested"), py_str("i am congested"), py_str("feel congested"), py_str("feeling congested"), py_str("really congested"), py_str("mouth ulcer"), py_str("mouth ulcers"), py_str("ulcer in my mouth"), py_str("ulcers in my mouth"), py_str("ulcer on my tongue"), py_str("ulcer on my gum"), py_str("canker sore"), py_str("canker sores"), py_str("water in my ear"), py_str("water in my ears"), py_str("water trapped in my ear"), py_str("water trapped in my ears"), py_str("water stuck in my ear"), py_str("water stuck in my ears"), py_str("water in my ear after swimming"), py_str("swimmers ear"), py_str("swimmer's ear"), py_str("upset stomach"), py_str("upset tummy"), py_str("upset belly"), py_str("upset guts"), py_str("bad guts"), py_str("bad belly"), py_str("dicky tummy"), py_str("dicky stomach"), py_str("dodgy tummy"), py_str("dodgy stomach"), py_str("tummy bug"), py_str("stomach bug"), py_str("sick bug"), py_str("the runs"), py_str("the trots"), py_str("the squits"), py_str("got the runs"), py_str("have the runs"), py_str("loose stools"), py_str("runny tummy"), py_str("runny bottom"), py_str("running to the loo"), py_str("running to the toilet"), py_str("runs to the loo"), py_str("runs to the toilet"), py_str("back and forth to the loo"), py_str("stomach is off"), py_str("tummy is off"), py_str("belly is off"), py_str("stomach is queasy"), py_str("tummy is queasy"), py_str("diarrhoea"), py_str("diarrhea"), py_str("sick with diarrhoea"), py_str("sick with diarrhea"), py_str("wisdom tooth"), py_str("wisdom teeth"), py_str("ingrown toenail"), py_str("ingrown nail"), py_str("ingrowing toenail"), py_str("ingrowing nail"), py_str("nail is ingrown"), py_str("nettle sting"), py_str("nettle stings"), py_str("stung by nettles"), py_str("insect bite"), py_str("insect bites"), py_str("insect sting"), py_str("bug bite"), py_str("bug bites"), py_str("hair loss"), py_str("losing my hair"), py_str("going bald"), py_str("panic attack"), py_str("panic attacks"), py_str("acid reflux"), py_str("acid indigestion"), py_str("athletes foot"), py_str("athlete's foot"), py_str("sweating too much"), py_str("excessive sweating"), py_str("night sweats"), py_str("heavy period"), py_str("period pain"), py_str("period pains"), py_str("period cramps"), py_str("on my period"), py_str("missed period"), py_str("late period"), py_str("irregular periods"), py_str("stop sweating"), py_str("sweat so much"), py_str("sweating a lot"), py_str("sweat a lot"), py_str("lump in my breast"), py_str("lump in my armpit"), py_str("lump in my neck"), py_str("lump under my skin"), py_str("swollen glands"), py_str("glands are up")])), (py_str("fitness"), py_tuple(vec![py_str("out of breath"), py_str("get fit"), py_str("lose weight"), py_str("work out"), py_str("start running"), py_str("training for"), py_str("train for"), py_str("train hard"), py_str("lower back")])), (py_str("sleep"), py_tuple(vec![py_str("get to sleep"), py_str("back to sleep"), py_str("wake up"), py_str("wakes up"), py_str("can not sleep"), py_str("cant sleep"), py_str("sleep through")])), (py_str("money"), py_tuple(vec![py_str("save for"), py_str("how much should i save"), py_str("pay off"), py_str("stamp duty"), py_str("worth paying"), py_str("interest rate"), py_str("interest rates"), py_str("savings account"), py_str("the interest"), py_str("compound interest"), py_str("cut my bills"), py_str("energy bills"), py_str("energy bill"), py_str("electricity bill"), py_str("gas bill"), py_str("water bill"), py_str("small salary"), py_str("how should i budget")])), (py_str("work"), py_tuple(vec![py_str("my boss"), py_str("my manager"), py_str("job interview"), py_str("my colleague"), py_str("teaching assistant"), py_str("performance review"), py_str("my team"), py_str("too much on my plate"), py_str("public speaking"), py_str("speaking in front of people"), py_str("speaking in front of an audience"), py_str("in front of an audience"), py_str("speaking to a room"), py_str("before a talk"), py_str("at a talk"), py_str("giving a talk"), py_str("give a talk"), py_str("nervous about my presentation")])), (py_str("school-and-study"), py_tuple(vec![py_str("revise for"), py_str("exam in"), py_str("how do i revise"), py_str("learn vocabulary"), py_str("struggling with"), py_str("get started on"), py_str("science project"), py_str("school project"), py_str("for an exam"), py_str("a degree worth"), py_str("degree still worth"), py_str("do a degree"), py_str("a-level results"), py_str("do my a-levels"), py_str("my mocks"), py_str("mock exam"), py_str("mock exams"), py_str("mocks in"), py_str("mocks next")])), (py_str("science"), py_tuple(vec![py_str("how does it work"), py_str("why does"), py_str("why is the sky"), py_str("what is the difference between"), py_str("how do plants"), py_str("in a nutshell"), py_str("speed of light"), py_str("how many moons"), py_str("why is it hotter"), py_str("human body"), py_str("bones in the"), py_str("bones are in"), py_str("how many bones"), py_str("what causes an earthquake"), py_str("what causes earthquakes"), py_str("how do earthquakes"), py_str("why do earthquakes"), py_str("earthquake happen"), py_str("earthquakes happen"), py_str("how do tsunamis"), py_str("what causes a tsunami"), py_str("tsunami warning"), py_str("what causes a volcano"), py_str("how do volcanoes"), py_str("volcano erupt"), py_str("volcano erupts"), py_str("volcano erupted"), py_str("volcanoes erupt"), py_str("volcano erupting"), py_str("volcanoes work"), py_str("volcano work"), py_str("volcano project"), py_str("volcanoes project"), py_str("project on volcanoes"), py_str("project on volcano"), py_str("volcanic eruption")])), (py_str("history"), py_tuple(vec![py_str("who built"), py_str("when did"), py_str("in a nutshell"), py_str("roman empire"), py_str("industrial revolution"), py_str("slave trade"), py_str("genghis khan"), py_str("great wall of china"), py_str("ancient china"), py_str("berlin wall"), py_str("the berlin wall"), py_str("cold war"), py_str("moon landing"), py_str("titanic sank"), py_str("titanic sink"), py_str("titanic sinking"), py_str("titanic wreck"), py_str("titanic iceberg"), py_str("the titanic hit"), py_str("when did the titanic"), py_str("how did the titanic"), py_str("why did the titanic"), py_str("the tudors"), py_str("tudor period"), py_str("tudor times"), py_str("tudor england")])), (py_str("geography-and-travel"), py_tuple(vec![py_str("day trip"), py_str("jet lag"), py_str("shouldn miss"), py_str("don t miss"), py_str("what should i see"), py_str("long flight"), py_str("car journey"), py_str("capital of"), py_str("book a flight"), py_str("book a hotel"), py_str("book a train"), py_str("book a room"), py_str("by coach"), py_str("a coach to"), py_str("coach from"), py_str("coach station"), py_str("coach trip"), py_str("coach leaves"), py_str("by train"), py_str("train ticket"), py_str("train tickets"), py_str("train station"), py_str("train times"), py_str("train fare"), py_str("train fares"), py_str("catch the train"), py_str("the next train"), py_str("the last train"), py_str("missed the train"), py_str("missed my train"), py_str("the train to"), py_str("train to london"), py_str("train to leeds")])), (py_str("technology"), py_tuple(vec![py_str("battery drains"), py_str("wont turn on"), py_str("wont start"), py_str("keeps dropping"), py_str("improve the signal"), py_str("take a screenshot"), py_str("password manager"), py_str("group chat")])), (py_str("home-and-diy"), py_tuple(vec![py_str("get rid of"), py_str("unblock"), py_str("bleed the radiator"), py_str("front door"), py_str("back door"), py_str("clothes iron"), py_str("steam iron"), py_str("iron a shirt"), py_str("iron the shirt"), py_str("iron my shirt"), py_str("iron a few shirts"), py_str("ironing board"), py_str("iron board"), py_str("my iron"), py_str("the iron"), py_str("our iron"), py_str("iron is broken"), py_str("iron broke"), py_str("iron has broken"), py_str("iron stopped"), py_str("iron wont heat"), py_str("iron is not heating"), py_str("iron is leaking"), py_str("iron spits"), py_str("iron spitting"), py_str("never iron"), py_str("never ironing"), py_str("hate ironing"), py_str("creases out of"), py_str("crease out of"), py_str("get creases"), py_str("stop creases"), py_str("trousers creased"), py_str("iron the creases"), py_str("heat pump"), py_str("heat pumps"), py_str("cold at the top"), py_str("desk fan"), py_str("desk fans"), py_str("ceiling fan"), py_str("ceiling fans"), py_str("hot at the bottom"), py_str("get red wine out"), py_str("paint my"), py_str("second hand"), py_str("do the dishes"), py_str("wash the dishes"), py_str("the dishes"), py_str("dishes are"), py_str("the spring on"), py_str("spring broke"), py_str("spring has gone"), py_str("a new spring"), py_str("the spring needs"), py_str("smoke alarm"), py_str("wont flush"), py_str("won t flush"), py_str("cold air coming"), py_str("water pressure"), py_str("low pressure"), py_str("pressure gauge"), py_str("wood burner"), py_str("woodburner"), py_str("log burner"), py_str("log fire"), py_str("wood fire"), py_str("burning stove"), py_str("wood stove"), py_str("fire place")])), (py_str("garden-and-plants"), py_tuple(vec![py_str("turning yellow"), py_str("easy to grow"), py_str("when should i plant"), py_str("plant bulbs"), py_str("on the balcony"), py_str("when do i harvest"), py_str("when should i harvest"), py_str("when can i harvest"), py_str("when do i sow"), py_str("when should i sow"), py_str("when can i sow"), py_str("when do i lift"), py_str("when should i lift"), py_str("ready to harvest"), py_str("ready to lift")])), (py_str("pets"), py_tuple(vec![py_str("pulls on the lead"), py_str("dangerous for dogs"), py_str("clean the tank"), py_str("getting a puppy"), py_str("getting a kitten"), py_str("rescue dog"), py_str("stopped eating"), py_str("has not eaten")])), (py_str("family-and-parenting"), py_tuple(vec![py_str("starting school"), py_str("with kids"), py_str("for my son"), py_str("for my daughter"), py_str("potty training"), py_str("health visitor"), py_str("health visitors"), py_str("my health visitor"), py_str("my parents are visiting"), py_str("birthday party"), py_str("watch the kids"), py_str("watching the kids"), py_str("watch my kids"), py_str("to watch my kids"), py_str("to watch the kids"), py_str("to watch the children"), py_str("someone to watch")])), (py_str("friends-and-relationships"), py_tuple(vec![py_str("forgot my birthday"), py_str("make friends"), py_str("anniversary gift"), py_str("had an argument"), py_str("bring it up"), py_str("best friend"), py_str("on a date"), py_str("date night"), py_str("blind date"), py_str("first date"), py_str("going on a date"), py_str("a date with"), py_str("fell out with"), py_str("avoiding me"), py_str("stopped asking me out"), py_str("stopped speaking to me"), py_str("not speaking to me")])), (py_str("shopping"), py_tuple(vec![py_str("worth it"), py_str("worth the money"), py_str("should i look for"), py_str("before buying"), py_str("second-hand"), py_str("which one should i buy"), py_str("buying a car"), py_str("used car"), py_str("worth paying"), py_str("good deal"), py_str("bad deal"), py_str("better deal"), py_str("a deal"), py_str("the store"), py_str("at the store"), py_str("to the store"), py_str("in the store"), py_str("store was"), py_str("store is"), py_str("which store"), py_str("store sells"), py_str("store open"), py_str("store closed"), py_str("any store"), py_str("the shops"), py_str("what specs"), py_str("how much should a decent")])), (py_str("cars"), py_tuple(vec![py_str("jump start"), py_str("warning light"), py_str("engine light"), py_str("tyre pressure"), py_str("oil change"), py_str("break down"), py_str("broken down"), py_str("hard shoulder"), py_str("won t start"), py_str("wont start"), py_str("won t turn over"), py_str("new tyres"), py_str("car insurance"), py_str("road tax"), py_str("mot due"), py_str("mot fail"), py_str("failed its mot"), py_str("car tax")])), (py_str("entertainment"), py_tuple(vec![py_str("harry potter"), py_str("what should i read"), py_str("what should i watch"), py_str("anything good"), py_str("watch tv"), py_str("watch a film"), py_str("watch a movie"), py_str("watch a series"), py_str("watching tv"), py_str("watch the match"), py_str("watching the match"), py_str("something to watch"), py_str("anything to watch"), py_str("what can i watch"), py_str("can i watch"), py_str("this book"), py_str("a book about"), py_str("book club"), py_str("which book"), py_str("a book for"), py_str("what book"), py_str("good book"), py_str("need a book"), py_str("book to read"), py_str("book recommendations"), py_str("recommend a book"), py_str("books for"), py_str("favourite book"), py_str("favorite book"), py_str("to watch"), py_str("worth watching"), py_str("what to read"), py_str("family night"), py_str("board game"), py_str("play station"), py_str("playstation"), py_str("for my commute"), py_str("season ticket"), py_str("season finale"), py_str("new season of"), py_str("next season of"), py_str("series finale")])), (py_str("sport"), py_tuple(vec![py_str("offside rule"), py_str("world cup"), py_str("how does the game work"), py_str("my coach"), py_str("coached"), py_str("coaching"), py_str("the match"), py_str("match is on"), py_str("match tonight"), py_str("match day"), py_str("cricket match"), py_str("football match"), py_str("rugby match"), py_str("tennis match"), py_str("darts match"), py_str("boxing match"), py_str("this season"), py_str("next season"), py_str("pre-season")])), (py_str("language-and-writing"), py_tuple(vec![py_str("difference between"), py_str("is it bad to say"), py_str("how do i word"), py_str("without sounding rude"), py_str("can you help me word")])), (py_str("feelings"), py_tuple(vec![py_str("i feel"), py_str("i am feeling"), py_str("been feeling"), py_str("feel like"), py_str("so stressed"), py_str("so done"), py_str("done with today"), py_str("done with this week"), py_str("done with today"), py_str("fed up"), py_str("had enough"), py_str("long day"), py_str("what a day"), py_str("tough day"), py_str("rough day"), py_str("really lonely"), py_str("i am nervous"), py_str("nervous about"), py_str("i am worried"), py_str("i am gutted"), py_str("i am upset"), py_str("let off steam"), py_str("miss my old"), py_str("get everything wrong"), py_str("gets everything wrong"), py_str("do everything wrong"), py_str("did everything wrong"), py_str("doing everything wrong"), py_str("doing it all wrong"), py_str("did it all wrong"), py_str("everything i do goes wrong"), py_str("everything i do is wrong"), py_str("mess everything up"), py_str("messed everything up"), py_str("messing everything up"), py_str("mess it all up"), py_str("messed it all up"), py_str("cant do anything right"), py_str("can not do anything right"), py_str("never get anything right"), py_str("never do anything right")])), (py_str("small-talk"), py_tuple(vec![py_str("how s your day"), py_str("how is your day"), py_str("how are you"), py_str("how is it going"), py_str("nice to meet you"), py_str("what s up"), py_str("any plans"), py_str("plans for the weekend"), py_str("up to this weekend"), py_str("this weekend"), py_str("weekend plans")])), (py_str("opinions"), py_tuple(vec![py_str("what do you think"), py_str("do you think"), py_str("your favourite"), py_str("which do you prefer")])), (py_str("creative"), py_tuple(vec![py_str("write a poem"), py_str("tell me a joke"), py_str("name for"), py_str("opening line")])), (py_str("maths-and-numbers"), py_tuple(vec![py_str("how many weeks"), py_str("how much is"), py_str("percent of"), py_str("what is the average"), py_str("miles an hour"), py_str("miles per hour"), py_str("km an hour"), py_str("per hour"), py_str("per day"), py_str("per week"), py_str("how many miles")])), (py_str("nature-and-weather"), py_tuple(vec![py_str("change colour"), py_str("change color"), py_str("leaves change"), py_str("leaves fall"), py_str("leaves turn"), py_str("in a thunderstorm"), py_str("in the spring"), py_str("this spring"), py_str("next spring"), py_str("spring is"), py_str("spring flowers"), py_str("early spring"), py_str("going to rain"), py_str("will it rain"), py_str("is it going to be sunny"), py_str("birds might i see"), py_str("leaves are")])), (py_str("about-the-bot"), py_tuple(vec![py_str("are you a bot"), py_str("are you real"), py_str("are you human"), py_str("what are you"), py_str("who are you"), py_str("do you remember"), py_str("what can you do"), py_str("what kind of things can you help"), py_str("how can you help"), py_str("how was your day"), py_str("how has your day been"), py_str("hows your day been"), py_str("how s your day been"), py_str("hows your day going"), py_str("how was your week"), py_str("how has your week been"), py_str("hows your week been"), py_str("how s your week been"), py_str("how have you been"), py_str("how you been")]))]));
    s_NOT_THINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\ncook cooking bake baking boil boiled frying fry roast roasted grill simmer chop chopped stir\neat eating drink drinking feed fed save saving pay paid spend spent buy buying bought sell\nsold work working revise revising study studying learn learning teach taught travel travelling\ntraveling fly flying drive driving pack packing book booking plan planning move moving rent\nwatch watching read reading write writing wrote felt feel feeling sleep sleeping slept wake\nwaking run running jog jogging train training exercise stretching stretch grow growing plant\nplanting water watering clean cleaning fix fixing paint painting repair repairing adopt adopted\nrescue buy fix bleed bleeding cut chopped knocked dropped broke broken spill spilled leak\nleaking drain drains turned turning turned stuck stop stopped start started finish finished\nscrambled poached undercooked overcooked sticky mushy crispy hungry thirsty tasty delicious\nexpensive cheap worth tired exhausted lonely sad unhappy upset anxious worried nervous stressed\noverwhelmed depressed crying angry annoyed frustrated guilty ashamed jealous happy excited\nrelieved proud grateful dizzy sore ill sick unwell allergic vegan vegetarian second-hand\nbest better worst good bad great nice lovely favourite favorite perfect brilliant amazing\nfine okay first second third last next more less most least many few several some\npercent percentage maths math number numbers average total half double\nlog lag row sow hoe\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STRONG(), "values", args))(&__args) }) { let mut words: Py = __item.clone(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "isalpha", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_THINGS(), &word.clone())) } } } }) {  __out.push(word.clone()); } } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLACES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nlondon paris rome florence lisbon madrid barcelona berlin amsterdam brussels dublin edinburgh\nglasgow manchester birmingham leeds liverpool bristol cardiff belfast newcastle oxford\ncambridge brighton york bath bristol glasgow tokyo kyoto osaka beijing shanghai hong kong\nsingapore bangkok sydney melbourne auckland delhi mumbai goa istanbul athens cairo dubai\nnew york chicago boston seattle toronto vancouver mexico brazil argentina spain france\nitaly germany portugal greece turkey egypt india china japan korea thailand vietnam canada\naustralia america england scotland wales ireland europe africa asia\neiffel\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_UNIT_WORDS(py_dict(vec![(py_str("g"), py_str("g")), (py_str("gram"), py_str("g")), (py_str("grams"), py_str("g")), (py_str("gramme"), py_str("g")), (py_str("grammes"), py_str("g")), (py_str("kg"), py_str("kg")), (py_str("kilo"), py_str("kg")), (py_str("kilos"), py_str("kg")), (py_str("kilogram"), py_str("kg")), (py_str("kilograms"), py_str("kg")), (py_str("lb"), py_str("lb")), (py_str("lbs"), py_str("lb")), (py_str("pound"), py_str("lb")), (py_str("pounds"), py_str("lb")), (py_str("oz"), py_str("oz")), (py_str("ounce"), py_str("oz")), (py_str("ounces"), py_str("oz")), (py_str("ml"), py_str("ml")), (py_str("millilitre"), py_str("ml")), (py_str("millilitres"), py_str("ml")), (py_str("milliliter"), py_str("ml")), (py_str("l"), py_str("litre")), (py_str("litre"), py_str("litre")), (py_str("litres"), py_str("litre")), (py_str("liter"), py_str("litre")), (py_str("liters"), py_str("litre")), (py_str("tsp"), py_str("tsp")), (py_str("teaspoon"), py_str("tsp")), (py_str("teaspoons"), py_str("tsp")), (py_str("tbsp"), py_str("tbsp")), (py_str("tablespoon"), py_str("tbsp")), (py_str("tablespoons"), py_str("tbsp")), (py_str("cup"), py_str("cup")), (py_str("cups"), py_str("cup")), (py_str("c"), py_str("celsius")), (py_str("°c"), py_str("celsius")), (py_str("celsius"), py_str("celsius")), (py_str("centigrade"), py_str("celsius")), (py_str("f"), py_str("fahrenheit")), (py_str("°f"), py_str("fahrenheit")), (py_str("fahrenheit"), py_str("fahrenheit")), (py_str("sec"), py_str("seconds")), (py_str("secs"), py_str("seconds")), (py_str("second"), py_str("seconds")), (py_str("seconds"), py_str("seconds")), (py_str("min"), py_str("minutes")), (py_str("mins"), py_str("minutes")), (py_str("minute"), py_str("minutes")), (py_str("minutes"), py_str("minutes")), (py_str("hr"), py_str("hours")), (py_str("hrs"), py_str("hours")), (py_str("hour"), py_str("hours")), (py_str("hours"), py_str("hours")), (py_str("day"), py_str("days")), (py_str("days"), py_str("days")), (py_str("week"), py_str("weeks")), (py_str("weeks"), py_str("weeks")), (py_str("month"), py_str("months")), (py_str("months"), py_str("months")), (py_str("year"), py_str("years")), (py_str("years"), py_str("years")), (py_str("mile"), py_str("miles")), (py_str("miles"), py_str("miles")), (py_str("km"), py_str("km")), (py_str("kilometre"), py_str("km")), (py_str("kilometres"), py_str("km")), (py_str("kilometer"), py_str("km")), (py_str("kilometers"), py_str("km")), (py_str("metre"), py_str("metres")), (py_str("metres"), py_str("metres")), (py_str("meter"), py_str("metres")), (py_str("meters"), py_str("metres")), (py_str("foot"), py_str("feet")), (py_str("feet"), py_str("feet")), (py_str("ft"), py_str("feet")), (py_str("inch"), py_str("inches")), (py_str("inches"), py_str("inches")), (py_str("mph"), py_str("mph")), (py_str("percent"), py_str("percent")), (py_str("%"), py_str("percent")), (py_str("quid"), py_str("gbp")), (py_str("£"), py_str("gbp")), (py_str("euro"), py_str("eur")), (py_str("euros"), py_str("eur")), (py_str("€"), py_str("eur")), (py_str("dollar"), py_str("usd")), (py_str("dollars"), py_str("usd")), (py_str("$"), py_str("usd")), (py_str("people"), py_str("people")), (py_str("person"), py_str("people")), (py_str("kid"), py_str("people")), (py_str("kids"), py_str("people")), (py_str("adult"), py_str("people")), (py_str("adults"), py_str("people")), (py_str("guest"), py_str("people")), (py_str("guests"), py_str("people"))]));
    s_WORD_NUMBERS(py_dict(vec![(py_str("half"), Py::Float(0.5)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90)), (py_str("hundred"), Py::Int(100)), (py_str("thousand"), Py::Int(1000)), (py_str("couple"), Py::Int(2)), (py_str("dozen"), Py::Int(12))]));
    s_FIGURE_SIZE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("k ks grand thousand million millionaire billion bn mil mths"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NUMBER_RE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(\\d+(?:[.,]\\d+)?)\\s*(%|[a-z]+)?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLIPPED_UNIT_WORDS(py_dict(vec![(py_str("yr"), py_str("years")), (py_str("yrs"), py_str("years")), (py_str("y"), py_str("years")), (py_str("yo"), py_str("years")), (py_str("yos"), py_str("years")), (py_str("mth"), py_str("months")), (py_str("mths"), py_str("months")), (py_str("mnt"), py_str("months")), (py_str("mnts"), py_str("months")), (py_str("wk"), py_str("weeks")), (py_str("wks"), py_str("weeks"))]));
    s_CLIPPED_AFTER_A_SLASH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:y/o|y\\.o\\.?|yo)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MIXED_FRACTION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(?:(?:and|n)\\s+)?(?:(?:a|an)\\s+)?(?:half|quarter)s?\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MIXED_JOINERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str(""), py_str("n"), py_str("and"), py_str("a"), py_str("an")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BREAKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9' ]+")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CARRY(Py::Int(2));
    s_BODY(crate::understanding::lexicon::g_BODY_PARTS());
    s_HURTS(crate::understanding::lexicon::g_PAIN_WORDS());
    s_SKIN_STATES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("dry drier dryness flaky flakes flaking cracked cracking chapped\n    itchy itching peeling"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SKIN_REACH(Py::Int(3));
    s_SKIN_BOOST(Py::Int(4));
    s_SKIN_PARTS(op_bitor(&g_BODY(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("face cheeks cheek knuckles knuckle lips lip mouth heels"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_HEAD_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("head brain mind thoughts thinking concentration memory"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HEAD_FOG({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("fog foggy cloudy misty muzzy woolly wooly fuzzy thick"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HEAD_FOG_REACH(Py::Int(4));
    s_HEAD_FOG_BOOST(Py::Int(6));
    s_PLAYS_UP_BEFORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("playing playin plays play played acting acts acted\n    "), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLAY_UP_REACH(Py::Int(3));
    s_PART_UP_BOOST(Py::Int(4));
    s_HURT_BOOST(Py::Int(5));
    s_CONDITION_BOOST(Py::Int(4));
    s_CONDITIONS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cystitis uti thrush dandruff eczema asthma verruca verrucas piles\n    haemorrhoids sciatica gout stye styes shingles tonsillitis arthritis coeliac anaemic anaemia\n    psoriasis rosacea conjunctivitis bronchitis pneumonia appendicitis vertigo tinnitus\n    endometriosis ibs crohns colitis prolapse bunion bunions whitlow ringworm scabies nits\n    headlice hypo hypos coldsore coldsores abscess quinsy phlebitis dvt cataract cataracts\n    glaucoma gingivitis hiccup hiccups thyroid constipated\n    boil boils lump lumps spot spots cyst cysts filling fillings\n    pcos copd lupus fibromyalgia hashimotos hypothyroid hyperthyroid epilepsy diabetes\n    anorexia bulimia cfs ibd"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDITION_NEEDS_PART({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("boil boils lump lumps spot spots cyst cysts"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDITION_NEEDS_WORD(py_dict(vec![(py_str("filling"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("tooth teeth molar molars cavity cavities dentist dental gum gums\n        filled"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("fillings"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("tooth teeth molar molars cavity cavities dentist dental gum gums\n        filled"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s_CONDITION_OTHER_LIFE(py_dict(vec![(py_str("thrush"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("garden lawn bird birds nest hedge tree trees feeder"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), (py_str("shingles"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("roof roofs tile tiles tiled shed sheds wall walls house chimney slate"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s_CONDITION_NOT_NEXT(py_dict(vec![(py_str("piles"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("of"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s_CONDITION_REACH(Py::Int(3));
    s_UNWELL_BOOST(Py::Int(3));
    s_PEOPLE_TOPICS(py_tuple(vec![py_str("family-and-parenting"), py_str("friends-and-relationships"), py_str("pets")]));
    s_UNWELL_SIGNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("unresponsive unconscious drowsy confused floppy limp listless\n    dehydrated collapsed fitting seizure seizures temperature fever sepsis bleeding puking\n    puke puked barf\n    breathless rash cold cough flu dizzy vomiting vomit diarrhoea diarrhea constipated\n    wheeze wheezing swollen infection itchy ill unwell allergy allergic diabetic diabetes\n    overdose pills operation surgery chemo chemotherapy fell fall fallen broken broke\n    fracture fractured stroke injured injury"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_UNWELL_NOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("feeding drinking eating responding waking weeing"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOUNY_SIGNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("temperature cold cough flu pills pill operation surgery infection\n    rash seizure seizures allergy overdose chemo chemotherapy diabetes dose medicine"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_EXTRA_PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("newborn newborns toddler toddlers baby babies person people someone\n    somebody anyone anybody child children kid kids"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SIGN_FOLLOWS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the his her my your our their its is was are were am has have\n    had got get gets of and or but so to for in on at with from since last this that these those\n    today tonight yesterday morning night now still again too very really all just should shall\n    will would can could do does did not no time times day days week weeks month months about\n    up down out over"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HEARERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("me us name names pain voice voices anything anyone nothing stimulus\n    sounds noise light lights it them him her breakfast medicine"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COMMS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("text texts message messages phone phones mobile mobiles email emails\n    call calls whatsapp dm dms letter letters post emailing texting"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BEHAVIOUR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("when whenever if unless because after before till until once while"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SIGN_NOT_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("asleep out over apart behind short through pregnant ill about\n    for"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_UNWELL_IDIOMS(py_tuple(vec![py_str("bleeding me dry"), py_str("bleed me dry"), py_str("bleeding us dry"), py_str("bleed us dry"), py_str("drove me mad"), py_str("driving me mad"), py_str("worried sick"), py_str("bored stiff")]));
    s_PETS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("dog dogs puppy puppies cat cats kitten kittens rabbit hamster parrot goldfish\npet pets"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWNERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my our his her their"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANIMALS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("dog dogs puppy puppies cat cats kitten kittens rabbit rabbits hamster\nhamsters guinea parrot parrots budgie budgies goldfish fish tortoise tortoises turtle turtles\nsnake snakes lizard lizards gecko geckos horse horses pony"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PET_BOOST(Py::Int(3));
    s_COUNTRY_BOOST(Py::Int(6));
    s_GIFT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("gift gifts present presents"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIFT_ALWAYS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("gifts presents"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIVE_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("get gets got gotten getting buy buys buying bought pick wrap"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIFT_WANTING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("need needs needed needing still yet havent havnt haventgot\n    must should ought havent got gotta later tomorrow soon"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIVE_TO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("her him them mum mom mother dad father sister brother nan gran\n    granny grandma grandad grandpa friend friends teacher husband wife partner son daughter\n    nephew niece colleague boss kids child children inlaws flatmate flatmates housemate\n    roommate mate mates cousin cousins aunt aunty uncle neighbour neighbor girlfriend\n    boyfriend"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIFT_OCCASION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("christmas xmas birthday anniversary wedding graduation\n    leaving retiring retirement"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIFT_ASKING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("should could would shall what which worth best any idea ideas"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIVE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("something anything ideas idea nothing"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIFT_BOOST(Py::Int(5));
    s_GIFTS_OR_OCCASIONS(op_bitor(&op_bitor(&g_GIVE_THING(), &g_GIFT_WORDS()), &g_GIFT_OCCASION()));
    s_GIVE_THINGS(op_bitor(&g_GIVE_THING(), &g_GIFT_WORDS()));
    s_GIVE_DETERMINERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my your his their our the a an some"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIVE_RELATIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("who whos that"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CROP_STATES(py_tuple(vec![py_str("not ripening"), py_str("going blue"), py_str("gone blue"), py_str("going yellow"), py_str("turning yellow"), py_str("gone to seed"), py_str("blossom end rot"), py_str("not flowering"), py_str("no flowers"), py_str("not fruiting"), py_str("no fruit"), py_str("ripening"), py_str("bolting"), py_str("leggy"), py_str("wilting"), py_str("drooping"), py_str("rotting"), py_str("rotten"), py_str("stunted"), py_str("flowering"), py_str("splitting"), py_str("dying")]));
    s_CROP_KITCHEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("fridge freezer cupboard pantry shelf shop shops supermarket market\nbag box tub jar tin pan pans oven hob pot pots bowl plate dinner lunch tea recipe cook cooked\ncooking eat eating mash roast boil boiled fry fried bake baked salad sandwich soup sauce"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CROPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("tomato tomatoes tom toms potato potatoes courgette courgettes zucchini\ncucumber cucumbers pepper peppers chilli chillies chili chillis carrot carrots onion onions\nleek leeks garlic lettuce spinach kale cabbage cabbages bean beans pea peas strawberry\nstrawberries raspberry raspberries blackberry blackberries blueberry blueberries apple apples\npear pears plum plums cherry cherries grape grapes rhubarb beetroot beetroot parsnip parsnips\nsquash pumpkin marrows marrow broccoli cauliflower sweetcorn corn aubergine aubergines\nbasil mint parsley rosemary thyme coriander chives sage oregano dill herbs herb"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GROWING_PLACES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("plant plants vine vines tree trees greenhouse greenhouse allotment\nallotments garden plot pots bed beds border seedling seedlings bush bushes"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CROP_BOOST(Py::Int(3));
    s_TRAVEL_CUES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("trip holiday flight fly flying visit visiting travel travelling\ntraveling abroad hotel booking itinerary sightseeing tourist stay staying going go get\nplan plans planned planning break weekend week days"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORLD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("sun sky moon stars earth planets planet ocean sea river mountain\nmountains snow ice rain wind thunder lightning clouds tides gravity rainbow seasons\nwinter summer spring autumn frost temperature fish fishes bird\nbirds animal animals insects bees brain bones lungs cells atoms veins"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let __aug2 = op_bitor(&g_WORLD(), &op_sub(&g_BODY(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("back hand hands arm arms leg legs foot feet chest"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_WORLD(__aug2);
    s_WORLD_NOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("maker machine tray cube bin bucket box cream lolly lollies"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NATURE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_STRONG(), &py_str("nature-and-weather")), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MECHANISM(py_tuple(vec![py_str("why"), py_str("how")]));
    s_MECHANISM_HOW({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("old many long far hot cold big deep fast wide"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SCIENCE_BOOST(Py::Int(2));
    s_VERB_CUES(py_dict(vec![(py_str("school-and-study"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("revise revising revision"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s_STUDY_OBJECTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("exam exams test tests mock mocks gcse a-level alevel biology\nchemistry physics maths math history geography french spanish german notes revision\ncoursework subject term paper spelling vocabulary homework school college university\nclass lesson a2 as-level sat sats"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DETERMINERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my your his her their our the a an this that these those some any"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BODY_CUES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("burn burnt burned burning scald scalded scalds cut cuts cutting\n    bleed bleeding"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PARTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("me myself yourself himself herself themselves ourselves itself tongue lip\nlips gum gums cheek cheeks chin forehead scalp brow eyebrow eyelash eyelid palm palms nail\nnails knuckle knuckles thigh thighs calf calves shin shins shing toe toes sole soles mouth\nmouths face faces nose noses bottom bottoms bum waist sides upper lower"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BODY_OR_SELF(op_bitor(&g_BODY(), &g_PARTS()));
    s_CUE_CARRIERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("and or but so now again today yesterday tomorrow still then than\n    it into in on at when if because very really badly slightly a bit\n    down back up out off away through over around first next"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_JUDGEMENT_OF_RECORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|i'm)\\s+(?:am |was |have |had |been |being |always |still |just |really |keep |keeps |kept |constantly |probably )*(?:get|gets|getting|got|do|does|did|doing|mess|messes|messed|messing)\\s+(?:everything|it all)\\s+(?:wrong|up)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_CUT_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cut out|made) (?:for|to be)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("not never dont don't didnt didn't isnt isn't wasnt wasn't aint ain't"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWN_SELF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i im i'm me myself"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OTHER_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("he she they hes shes theyre him her them his her their you your\n    we we're us our my brother sister son daughter mum mom dad father mother husband wife\n    partner friend boss colleague"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIVING_UP(py_tuple(vec![py_str("gave up"), py_str("give up"), py_str("given up"), py_str("gives up"), py_str("giving up"), py_str("packed in"), py_str("packing in"), py_str("cut out"), py_str("cutting out"), py_str("cut down on"), py_str("cutting down on"), py_str("stopped eating"), py_str("stopped drinking"), py_str("gone off")]));
    s_GIVEN_UP_FOOD(op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_STRONG(), &py_str("food-and-drink")), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_STRONG(), &py_str("cooking")), "split", args))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("meat"), py_str("dairy"), py_str("gluten"), py_str("lactose"), py_str("nut"), py_str("soya"), py_str("soy"), py_str("wheat")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_GIVING_UP_BOOST(Py::Int(3));
    s_GENERIC({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the to of in on at for with my your i you it is and or something\nsomeone thing things any some this that these those be have do does"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POLYSEMOUS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("date season store book coach dish match spring watch cream\n    pressure"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PORTION_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("half double triple"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PORTION_UNITS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cup cups stone stones pint pints litre litres ml gram grams kilo\nkilos kg ounce ounces oz tablespoon tablespoons tbsp teaspoon teaspoons inch inches foot feet\nmile miles metre metres metre hour hours minute minutes"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POLY_FORMS(op_bitor(&g_POLYSEMOUS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_POLYSEMOUS()) { let mut word: Py = __item.clone(); __out.push(op_add(&word.clone(), &py_str("s"))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_SEASONS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("spring summer autumn winter"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WEATHER_BEFORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("dry wet cold warm mild harsh hard bad good long short late early\n    frosty freezing sunny rainy"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WEATHER_ITSELF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("warm cold hot sunny cloudy raining snowing windy icy freezing\n    icy humid muggy showers drizzle"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SENSE_WEIGHT(Py::Int(6));
    s_SENSE_REACH(Py::Int(3));
    s_SENSE_NEAR(Py::Int(2));
    s_MACHINE_BIN_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("recycle trash windows mac macbook laptop desktop disk disks\n    file files folder folders downloads download storage usb restore permanently"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SENSE_PAIRS(py_dict(vec![(py_str("train"), py_tuple(vec![py_tuple(vec![py_str("train"), py_str("trains")]), py_str("geography-and-travel"), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("ticket tickets station platform timetable fare fares rail\n            railcard carriage intercity"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cancelled cancel delayed delay"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("missed miss catch catching missed leaves leaving departs\n            departing arrives arrival"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("run"), py_tuple(vec![py_tuple(vec![py_str("run"), py_str("runs"), py_str("running"), py_str("ran")]), py_str("geography-and-travel"), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("bus buses train trains tram trams coach coaches tube\n            ferry ferries flight flights timetable"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("chippy"), py_tuple(vec![py_tuple(vec![py_str("chippy"), py_str("chippies")]), py_str("home-and-diy"), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("door doors hinge hinges hang hanging shelf shelves skirting\n            timber joiner joiners fitter fitted"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("mole"), py_tuple(vec![py_tuple(vec![py_str("mole"), py_str("moles")]), py_str("garden-and-plants"), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("garden gardens lawn lawns grass hedge hedges border borders\n            soil flowerbed flowerbed lawnmower molehill molehills tunnel tunnels"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("root"), py_tuple(vec![py_tuple(vec![py_str("root"), py_str("roots")]), py_str("garden-and-plants"), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("plant plants tree trees soil pot pots potted repot repotting\n            cutting cuttings seedling seedlings"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("root-canal"), py_tuple(vec![py_tuple(vec![py_str("root"), py_str("roots")]), py_str("health"), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("canal tooth teeth dentist dental filling abscess gum\n            gums"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("root-maths"), py_tuple(vec![py_tuple(vec![py_str("root"), py_str("roots")]), py_str("maths-and-numbers"), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("square squares squared cube cubed equation equations\n            number numbers solve solved divided prime"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("root-access"), py_tuple(vec![py_tuple(vec![py_str("root"), py_str("roots")]), py_str("technology"), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("phone android device access enable enabled boot bootloader\n            samsung pixel tablet"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("stock"), py_tuple(vec![py_tuple(vec![py_str("stock"), py_str("stocks")]), py_str("money"), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("market markets share shares trading trader exchange portfolio\n            dividend dividends isa"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("stock-pot"), py_tuple(vec![py_tuple(vec![py_str("stock"), py_str("stocks")]), py_str("cooking"), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cube cubes pot pan soup gravy vegetable chicken beef ham\n            simmer simmering broth"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("match"), py_tuple(vec![py_tuple(vec![py_str("match"), py_str("matches")]), py_str("home-and-diy"), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("box damp flame flames firelighter firelighters kindling bbq\n            barbecue burner smoulder smolder"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("light lights lit strike struck"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("bin"), py_tuple(vec![py_tuple(vec![py_str("bin"), py_str("bins")]), py_str("technology"), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), g_MACHINE_BIN_WORDS()])])]))]));
    s_TWO_LIVES(py_dict(vec![(py_str("bank"), py_tuple(vec![py_tuple(vec![py_str("bank"), py_str("banks")]), py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("account accounts money cash card cards overdraft loan loans mortgage\n            mortgages payment payments paid pay savings branch branches switch switches\n            statement statements balance balances deposit deposits withdraw withdrawal\n            transfer transfers debit credit cheque cheques interest debt debts atm cashpoint\n            salary salaries wages building society current holiday holidays food blood\n            england sperm said says told rang ring called phoned robbed robbery robber cashier teller\n            vault charge charges charged charging fee fees refund refunds claim claims useless\n            overdrawn locked blocked phone app online free pending shut shuts empty sunday\n            sundays saturday saturdays weekday weekdays hours opening opens"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("money"), py_str("shopping"), py_str("life-admin"), py_str("work-and-money")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("river rivers water fish swim swimming path paths walk walked walking boat\n            boats flood flooded muddy grass trees reeds ducks swans fishing current currents\n            flow steep bridge bridges canal canals lake lakes stream streams shore bankside"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("nature-and-weather"), py_str("geography-and-travel"), py_str("fitness"), py_str("home-and-diy")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("seal"), py_tuple(vec![py_tuple(vec![py_str("seal"), py_str("seals")]), py_tuple(vec![py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("zoo sea beach ocean shore rocks ice swimming fish pup pups rescue wildlife\n            aquarium flipper flippers colony warden vet injured entangled plastic cute cuddly\n            adorable"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("pets"), py_str("nature-and-weather"), py_str("geography-and-travel")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("jar jars lid lids door doors window windows freezer fridge container\n            containers bottle bottles cap caps rubber gasket airtight vacuum preserve jam\n            pickle filter filters oven washing machine drum black mould mold leak leaks\n            leaking leaked dripping split torn worn perished hardened stiff sticky crumpled\n            shut shuts"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("home-and-diy"), py_str("cooking"), py_str("cleaning"), py_str("food-and-drink")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])]))]));
    s_TWO_LIVES_FORMS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TWO_LIVES(), "values", args))(&__args) }) { let __unpk6353 = py_unpack(&__item.clone(), 2, None); let mut _forms: Py = __unpk6353[0].clone(); let mut tiers: Py = __unpk6353[1].clone(); for __item in py_iter_or_raise(&_forms.clone()) { let mut form: Py = __item.clone(); __out.push((form.clone(), tiers.clone())); } } py_dict(__out) })() });
    s_TWO_LIVES_REACH(Py::Int(99));
    s__A_STATE_VERB({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("is are was were has have had looks seems seemed becomes became\n    sounds feels closed shut broke broken breaks split splits tore torn worn wears"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THE_OWNER_SETTLES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("bank")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__POSSESSIVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my our your his her their"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_THE_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the my our your his her their this that these those i we\n    you he she they it there old new local main high street current former usual regular\n    well so but and oh hmm actually honestly anyway right now then also just only even\n    hi hello hey good morning afternoon evening please thanks thank perhaps maybe"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SENSE_ELSEWHERE(py_dict(vec![(py_str("pie"), py_tuple(vec![py_tuple(vec![py_str("pie"), py_str("pies")]), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("chart charts graph graphs diagram diagrams"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("jam"), py_tuple(vec![py_tuple(vec![py_str("jam"), py_str("jams")]), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("traffic queued stuck pickle spot trouble bind work"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("chips"), py_tuple(vec![py_tuple(vec![py_str("chip"), py_str("chips")]), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("down poker casino betting stack stacks"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("yeast"), py_tuple(vec![py_tuple(vec![py_str("yeast")]), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("infection infections thrush"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("nails"), py_tuple(vec![py_tuple(vec![py_str("nail"), py_str("nails")]), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("shelf hammer wooden wood wall walls box boxes diy rawlplug\n            plasterboard studwork picture pictures hang hanging screws screw"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("breast"), py_tuple(vec![py_tuple(vec![py_str("breast"), py_str("breasts")]), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("lump lumps cancer nipple nipples areola sore hurting hurt\n            hurts painful ache aches aching swelling discharge mastitis checked checking scan\n            mammogram"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("milk feeding breastfeeding fed"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("spam"), py_tuple(vec![py_tuple(vec![py_str("spam")]), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("post letter letters envelope envelopes doormat junk pile"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("thighs"), py_tuple(vec![py_tuple(vec![py_str("thigh"), py_str("thighs")]), py_tuple(vec![py_tuple(vec![g_SENSE_NEAR(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("tight sore ache aches aching hurt hurts hurting cramp cramps\n            chafing"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("icing"), py_tuple(vec![py_tuple(vec![py_str("icing")]), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("knee knees ankle ankles shoulder elbow muscle muscles\n            injury injured swelling sprain bruise bruising hurt hurts"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])])), (py_str("fibre"), py_tuple(vec![py_tuple(vec![py_str("fibre"), py_str("fiber")]), py_tuple(vec![py_tuple(vec![g_SENSE_REACH(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("broadband internet router connection cable provider\n            install installed"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])])]))]));
    s_TEETHING_IS_A_FAULT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("problem problems trouble troubles fault faults\n    niggles issues"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TEETHING_AND_A_CHILD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("baby babies child children kid kids toddler toddlers\n    son daughter newborn girl boy he she his her their ours"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SENSE_ELSEWHERE_FORMS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENSE_ELSEWHERE(), "values", args))(&__args) }) { let __unpk3079 = py_unpack(&__item.clone(), 2, None); let mut _forms: Py = __unpk3079[0].clone(); let mut tiers: Py = __unpk3079[1].clone(); for __item in py_iter_or_raise(&_forms.clone()) { let mut form: Py = __item.clone(); __out.push((form.clone(), tiers.clone())); } } py_dict(__out) })() });
    s_BODY_TWO_LIFE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("breast breasts thigh thighs"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWNING_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my her his their your our"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COLD_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cold freezing frozen chilled"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOT_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("hot boiling roasting sweltering sweaty sweating shivery shivering\n    feverish"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WARMTH_WORDS(op_bitor(&g_COLD_WORDS(), &g_HOT_WORDS()));
    s_WARMTH_ARE(op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("is are was were am feeling getting being"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("feel feels felt"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_WARMTH_DEGREE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a bit lot little tad wee touch really so very too proper bloody\n    freezing stone quite that dead still always absolutely proper"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WARMTH_HABITUAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("always still"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WARMTH_THEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("now today tonight again too still here outside lately"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COLD_PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i im i'm iam me she he they we you u everyone everybody someone\n    somebody people"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MACHINE_WORD_CUES(py_dict(vec![(py_str("mouse"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("mice kitchen loft attic shed garage garden cupboard bedroom house home\n        flat wall walls nest trap traps mousetrap droppings infestation pest pests rodent\n        rodents chewing chewed scurrying poison wild tame pet pets caught catch cat cats dog\n        dogs cage hutch tank vet alive ran running escape escaped squeak squeaking bait cheese\n        bins bin cavity floorboards floorboard living saw seen"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })]));
    s_MACHINE_WORD_PHRASES(py_tuple(vec![py_str("mouse problem"), py_str("mouse problems"), py_str("mice problem"), py_str("mice problems"), py_str("mouse infestation"), py_str("mouse in the"), py_str("mouse in my"), py_str("mice in the"), py_str("mice in my"), py_str("rid of mice"), py_str("rid of the mouse"), py_str("mouse trap")]));
    s_ASK_PHRASES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_list(vec![py_str("any ideas"), py_str("do you think")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ORDINARY_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("keep keeps kept drop drops dropped dropping fall falls falling\n    cut cuts cutting stop stops stopped start starts started go goes going get gets getting\n    turn turns turned work works worked show shows showed"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SENSE_TIERS(py_dict(vec![]));
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENSE_PAIRS(), "values", args))(&__args) });
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 3, None);
    s__forms(__unpk4[0].clone());
    s__sense_topic(__unpk4[1].clone());
    s__tiers(__unpk4[2].clone());
    let __iter5 = py_iter_or_raise(&g__forms());
    for __item in __iter5 {
    s__form(__item.clone());
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![g__sense_topic(), g__tiers()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__form()); __args.push(py_list(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SENSE_TIERS(), "setdefault", args))(&__args) }, "append", args))(&__args) };
    }
    }
    s_CAR_NAMES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("audi fords ford vauxhall toyota honda nissan renault peugeot citroen\n    skoda hyundai kia mazda volvo suzuki bmw mercedes volkswagen vw fiat tesla dacia chevrolet\n    chrysler dodge subaru mitsubishi lexus saab alfa jeep bentley porsche ferrari lamborghini\n    aston lada daewoo infiniti acura punto corsa clio astra mondeo passat micra yaris fabia\n    octavia ibiza qashqai juke captur kadjar sandero duster sprinter picanto zafira insignia\n    vectra laguna megane berlingo picasso cupra leon toledo kamiq karoq kodiaq tucson sportage\n    camaro impreza forester hilux auris avensis prius celica supra aygo navara"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TABLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _index(args))(&__args) });
    s_DEGREE_NOUNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("lot lots bit little much few couple plenty"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RUN_OUT_BEFORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("run runs running ran"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WATCH_OUT_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("for out"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WATCHING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("watch watches watching watched"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DRIVEN_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("me us him her them you u everyone everybody"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DRIVEN_MOOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("mad insane crazy nuts bonkers barmy potty daft distracted wild\n    spare"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DRIVEN_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("wall walls bend"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DRIVEN_JOINERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("up round around over"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DRIVES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("drive drives driving drove driven"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AGE_NOUNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("year years yr yrs month months mth mths week weeks wk wks"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AGE_IN_A_YEAR(py_dict(vec![(py_str("year"), Py::Int(1)), (py_str("years"), Py::Int(1)), (py_str("yr"), Py::Int(1)), (py_str("yrs"), Py::Int(1)), (py_str("month"), Py::Int(12)), (py_str("months"), Py::Int(12)), (py_str("mth"), Py::Int(12)), (py_str("mths"), Py::Int(12)), (py_str("week"), Py::Int(52)), (py_str("weeks"), Py::Int(52)), (py_str("wk"), Py::Int(52)), (py_str("wks"), Py::Int(52))]));
    s_A_CHILD_TILL(Py::Int(19));
    s_AGE_OF_A_CHILD_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("baby babies child children kid kids toddler toddlers\n    infant infants newborn boy girl son daughter grandchild lad lass"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_PERSONS_OWN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my our his her their"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_AGE_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("is are was were be been about turned turns nearly almost just only"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BOOK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("book books novel novels"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_CHILD_TO_READ_FOR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("child children kid kids baby toddler boy girl son\n    daughter niece nephew grandson granddaughter grandchild teenager teen"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IDIOM_PHRASES(py_tuple(vec![py_tuple(vec![py_str("run"), py_str("off"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("run"), py_str("off"), py_str("our"), py_str("feet")]), py_tuple(vec![py_str("run"), py_str("off"), py_str("your"), py_str("feet")]), py_tuple(vec![py_str("running"), py_str("off"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("rushed"), py_str("off"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("rushed"), py_str("off"), py_str("our"), py_str("feet")]), py_tuple(vec![py_str("pushed"), py_str("off"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("ran"), py_str("off"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("on"), py_str("my"), py_str("feet"), py_str("all"), py_str("day")]), py_tuple(vec![py_str("on"), py_str("our"), py_str("feet"), py_str("all"), py_str("day")]), py_tuple(vec![py_str("on"), py_str("my"), py_str("feet"), py_str("all"), py_str("week")]), py_tuple(vec![py_str("up"), py_str("on"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("run"), py_str("ragged")]), py_tuple(vec![py_str("running"), py_str("ragged")]), py_tuple(vec![py_str("ran"), py_str("ragged")]), py_tuple(vec![py_str("dead"), py_str("on"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("dead"), py_str("on"), py_str("her"), py_str("feet")]), py_tuple(vec![py_str("dead"), py_str("on"), py_str("his"), py_str("feet")]), py_tuple(vec![py_str("dead"), py_str("on"), py_str("their"), py_str("feet")]), py_tuple(vec![py_str("back"), py_str("on"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("back"), py_str("on"), py_str("his"), py_str("feet")]), py_tuple(vec![py_str("back"), py_str("on"), py_str("her"), py_str("feet")]), py_tuple(vec![py_str("back"), py_str("on"), py_str("its"), py_str("feet")]), py_tuple(vec![py_str("back"), py_str("on"), py_str("their"), py_str("feet")]), py_tuple(vec![py_str("back"), py_str("on"), py_str("our"), py_str("feet")]), py_tuple(vec![py_str("find"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("finding"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("found"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("two"), py_str("left"), py_str("feet")]), py_tuple(vec![py_str("swept"), py_str("off"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("swept"), py_str("me"), py_str("off"), py_str("my"), py_str("feet")]), py_tuple(vec![py_str("cold"), py_str("feet")]), py_tuple(vec![py_str("cold"), py_str("turkey")]), py_tuple(vec![py_str("snowed"), py_str("under")]), py_tuple(vec![py_str("snowed"), py_str("under"), py_str("with")]), py_tuple(vec![py_str("under"), py_str("the"), py_str("weather")]), py_tuple(vec![py_str("under"), py_str("my"), py_str("belt")]), py_tuple(vec![py_str("out"), py_str("of"), py_str("my"), py_str("depth")]), py_tuple(vec![py_str("out"), py_str("of"), py_str("our"), py_str("depth")]), py_tuple(vec![py_str("at"), py_str("my"), py_str("wits"), py_str("end")]), py_tuple(vec![py_str("at"), py_str("our"), py_str("wits"), py_str("end")]), py_tuple(vec![py_str("at"), py_str("wits"), py_str("end")]), py_tuple(vec![py_str("on"), py_str("the"), py_str("fence")]), py_tuple(vec![py_str("in"), py_str("two"), py_str("minds")]), py_tuple(vec![py_str("on"), py_str("the"), py_str("same"), py_str("page")]), py_tuple(vec![py_str("over"), py_str("the"), py_str("moon")]), py_tuple(vec![py_str("in"), py_str("the"), py_str("same"), py_str("boat")]), py_tuple(vec![py_str("doing"), py_str("my"), py_str("head"), py_str("in")]), py_tuple(vec![py_str("doing"), py_str("my"), py_str("nut"), py_str("in")]), py_tuple(vec![py_str("does"), py_str("my"), py_str("head"), py_str("in")]), py_tuple(vec![py_str("doing"), py_str("his"), py_str("head"), py_str("in")]), py_tuple(vec![py_str("doing"), py_str("her"), py_str("head"), py_str("in")]), py_tuple(vec![py_str("doing"), py_str("our"), py_str("head"), py_str("in")]), py_tuple(vec![py_str("a"), py_str("piece"), py_str("of"), py_str("cake")]), py_tuple(vec![py_str("piece"), py_str("of"), py_str("cake")]), py_tuple(vec![py_str("easy"), py_str("as"), py_str("pie")]), py_tuple(vec![py_str("spill"), py_str("the"), py_str("beans")]), py_tuple(vec![py_str("spilling"), py_str("the"), py_str("beans")]), py_tuple(vec![py_str("spilled"), py_str("the"), py_str("beans")]), py_tuple(vec![py_str("not"), py_str("my"), py_str("cup"), py_str("of"), py_str("tea")]), py_tuple(vec![py_str("my"), py_str("cup"), py_str("of"), py_str("tea")]), py_tuple(vec![py_str("the"), py_str("icing"), py_str("on"), py_str("the"), py_str("cake")]), py_tuple(vec![py_str("icing"), py_str("on"), py_str("the"), py_str("cake")]), py_tuple(vec![py_str("cherry"), py_str("on"), py_str("the"), py_str("cake")]), py_tuple(vec![py_str("half"), py_str("baked"), py_str("idea")]), py_tuple(vec![py_str("the"), py_str("last"), py_str("straw")]), py_tuple(vec![py_str("a"), py_str("storm"), py_str("in"), py_str("a"), py_str("teacup")]), py_tuple(vec![py_str("a"), py_str("pain"), py_str("in"), py_str("the"), py_str("neck")]), py_tuple(vec![py_str("a"), py_str("pain"), py_str("in"), py_str("the"), py_str("backside")]), py_tuple(vec![py_str("music"), py_str("to"), py_str("my"), py_str("ears")]), py_tuple(vec![py_str("a"), py_str("shot"), py_str("in"), py_str("the"), py_str("arm")]), py_tuple(vec![py_str("the"), py_str("tip"), py_str("of"), py_str("the"), py_str("iceberg")]), py_tuple(vec![py_str("a"), py_str("drop"), py_str("in"), py_str("the"), py_str("ocean")]), py_tuple(vec![py_str("a"), py_str("grey"), py_str("area")]), py_tuple(vec![py_str("off"), py_str("the"), py_str("rails")]), py_tuple(vec![py_str("off"), py_str("the"), py_str("boil")]), py_tuple(vec![py_str("on"), py_str("its"), py_str("last"), py_str("legs")]), py_tuple(vec![py_str("on"), py_str("their"), py_str("last"), py_str("legs")])]));
    s_COMPLAINT_OF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("sick tired bored weary"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COMPLAINT_OF_NEXT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("of with about"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BODILY_OUTPUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("poo poos pooed pooing poop poops pooped pooping stool stools\n    faeces feces turd turds bowels bowel"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BODILY_OUTPUT_NOUN(op_sub(&g_BODILY_OUTPUT(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("pooed pooped pooing pooping"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_SAMPLE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("sample samples test tests tested testing kit pot container jar\n    form forms"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STOOL_AS_FURNITURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("wobbly wobble wobbles wobbliest rickety leg legs joint joints\n    seat seats wood wooden screw screws screwdriver tighten tight creak creaks squeak squeaks\n    bracket glue chair chairs table tables bench furniture dresser sideboard carpenter flatpack\n    kitchen"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ONLY_A_STOOL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("stool stools"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DERIVED_ENDINGS(py_tuple(vec![py_str("ness"), py_str("ment"), py_str("tion"), py_str("sion"), py_str("ity"), py_str("ance"), py_str("ence"), py_str("ancy"), py_str("ency"), py_str("ism"), py_str("ist"), py_str("ful"), py_str("less"), py_str("ing"), py_str("ers"), py_str("ant"), py_str("ent"), py_str("er"), py_str("ed"), py_str("cy"), py_str("ic"), py_str("al"), py_str("ly"), py_str("th"), py_str("y"), py_str("s")]));
    s_FAMILY_PAIRS(py_tuple(vec![py_tuple(vec![py_str("ant"), py_str("ancy")]), py_tuple(vec![py_str("ent"), py_str("ency")]), py_tuple(vec![py_str("ate"), py_str("ation")]), py_tuple(vec![py_str("ate"), py_str("acy")]), py_tuple(vec![py_str("ive"), py_str("ion")]), py_tuple(vec![py_str("ible"), py_str("ibility")]), py_tuple(vec![py_str("able"), py_str("ability")])]));
    s_FAMILY_STEM(Py::Int(5));
    s_FAMILY_WORD(Py::Int(5));
    s__FUNCTION_WORDS(Py::None);
    s_TWO_HOMES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    tablet stock fish draughts tomato tomatoes basil virus team\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LIVES(py_dict(vec![(py_str("tablet"), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("pill pills dose dosage swallow swallowed blister pack pharmacy\n            prescription headache headachey painkiller painkillers ibuprofen paracetamol\n            antibiotic antibiotics chemist gp food meal meals stomach empty water milk\n            coffee breakfast with"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("screen screens battery charge charging charger charged app apps keypad\n            keyboard wifi update downloading cracked ipad android"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), (py_str("stock"), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cube cubes gravy soup broth pot saucepan freezer shelf cupboard kitchen\n            chicken vegetable vegetables veg boil simmer bone bones"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("shares share market markets invest investing investment portfolio dividend\n            dividends trading trader isa pension ftse broker buy sell buying selling worth\n            price prices"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), (py_str("fish"), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("dinner tea supper lunch cook cooked frying fried grill grilled baked\n            fillet fillets bones chips salmon tuna cod haddock mackerel monger fingers rice\n            lemon batter"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("tank tanks pond pond bowl aquarium goldfish swim swimming died dying filter\n            feeder feeding feed water"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), (py_str("draughts"), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("board pieces checkers play playing game games opponent win won lose"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("door doors window windows draught gap gaps under cold blow blowing seal"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), (py_str("tomato"), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("sauce sauces salad ketchup slice sliced soup tinned canned ripe puree\n            passata sandwich make making cook cooking dinner meal"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("plant plants grow growing grew seed seeds seedling seedlings garden\n            greenhouse blight water watering stake"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), (py_str("basil"), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("pesto pasta sauce leaves leaf fresh cook cooked make making recipe salad\n            sprinkle dish dinner meal pizza"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("plant plants grow growing grew died dying wilted wilt pot soil water\n            watering repot seed seeds"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), (py_str("virus"), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("laptop computer pc file files email software antivirus malware download\n            downloaded popup popups password browser update"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cold flu round caught fever cough symptoms sick ill bug bugs tummy\n            throwing headache"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), (py_str("team"), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("rota shift shifts manager boss job work project colleagues colleague\n            meeting employer office"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("match game win won lost lose score scored goal goals player players coach\n            league season fans football rugby cricket kit"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]))]));
    let _ = py_setitem(&g_LIVES(), &py_str("tomatoes"), geti(&g_LIVES(), &py_str("tomato")));
    s_PLACE_OVER(Py::Int(2));
    s_TRAVEL_STRONG({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("trip trips holiday holidays flight flights fly flying flew\n    visit visiting visited travel travelling traveling travelled traveled abroad hotel hotels\n    booking itinerary sightseeing tourist tourists weekend weekends break honeymoon\n    going drove driving ride staying"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("what how where when why which who whom whose can could should would\n    will shall do does did is are was were am any anyone something tell give suggest\n    recommend show me"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WANT_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("want wants wanted wanting need needs needed fancies fancy\n    fancied fancies"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_THING_WANTED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the some another new more"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WANTED_BEFORE_A_STOP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("for to of at with and but so because when if\n    about by from in on"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON_WHO_WANTS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("wife husband mrs missus partner son daughter kid kids\n    child children baby teenager mum mom mother dad father brother sister mate mates friend\n    friends boyfriend girlfriend fella bloke lad lass nan nana granny grandad"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FRIEND_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("mate mates pal pals buddy buddies bloke fella\n    flatmate flatmates housemate housemates"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWNED_BY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("my your his her our their me"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FRIEND_BOOST(Py::Int(1));
    s_NOSE_BLOCKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:im|i am|i feel|im feeling|i am feeling|i feel like i am)\\b(?:\\s+(?:all|really|so|very|a\\s+bit|a\\s+little|totally|properly|quite|that|bit|still|now|again|bit)){0,3}\\s+(?:stuffy|stuffed\\s+up|bunged\\s+up|blocked\\s+up|congested|congestion|clogged\\s+up)\\b|\\b(?:im|ive|i've|i\\s+have|i\\s+keep)\\b(?:(?!\\b(?:dog|cat|puppy|kitten|baby|kid|boy|girl)\\b)[^.!?]){0,18}?\\b(?:sniffly|sniffles|sniffling|sniffing|snotty)\\b|\\bmy\\s+(?:head|nose|sinuses?|nose\\s+passages?|face)\\b(?:\\s+(?:is|are|feels?|feeling|gets?|keeps?|has\\s+been)){0,2}(?:\\s+(?:all|really|so|very|a\\s+bit|a\\s+little|totally|quite|still|now|again|bit)){0,3}\\s+(?:stuffy|stuffed\\s+up|bunged\\s+up|blocked\\s+up|blocked|congested|congestion)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOSE_BLOCKED_BOOST(Py::Int(6));
    s_MONEY_TIGHT(py_tuple(vec![py_str("skint"), py_str("brassic"), py_str("till payday"), py_str("until payday"), py_str("money is tight"), py_str("tight month"), py_str("on the breadline"), py_str("no money till"), py_str("no money until"), py_str("no money left"), py_str("no money"), py_str("running out of money"), py_str("a bit short this month"), py_str("short this month"), py_str("short till payday"), py_str("short till"), py_str("short this week"), py_str("nothing till"), py_str("nothing until"), py_str("nowt till"), py_str("nowt until"), py_str("cleaned out"), py_str("not got a penny"), py_str("penny to my name")]));
    s_MONEY_TIGHT_MAX(Py::Int(10));
    s_BROKE_AS_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i im am we are he she hes shes they theyre you youre u is\n    was were been be being totally completely so officially proper flat dead stone absolutely\n    very pretty quite really just still honestly yeah ok okay and but now"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BROKE_AS_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("till until by for this week month payday weekend end today\n    tomorrow again now already friday monday tuesday wednesday thursday saturday sunday and but\n    so or time yet though honestly lol ha thanks anyway right now"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_MONEY_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("quid pound pounds tenner tenners fiver fivers bucks"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_AMOUNT_NOUN(py_dict(vec![(py_str("tenner"), Py::Int(10)), (py_str("tenners"), Py::Int(10)), (py_str("fiver"), Py::Int(5)), (py_str("fivers"), Py::Int(5))]));
    s_A_AMOUNT_SYMBOL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("£\\s*(\\d+(?:[.,]\\d+)?)|(?<![\\w.])(\\d{1,4})\\s*p\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WAIT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("till until til"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_WAIT_TO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("payday friday saturday sunday monday tuesday wednesday thursday week\n    weekend month end tomorrow today next"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BIG_PURSE(Py::Int(500));
    s_STONE_VERBS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("lose loses losing lost loss shed sheds shedding gained gain\n    gains gaining weighed weigh weighs weigh-in diet dieting slimming slimmer"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_STONE_COMPOUNDS(py_tuple(vec![py_str("put on"), py_str("piling on"), py_str("piled on")]));
    s_WORN_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("shattered whacked zonked pooped bushed fagged drained wrecked frazzled"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORN_OUT_SAID(py_tuple(vec![py_str("wiped out"), py_str("done in"), py_str("worn out"), py_str("cream crackered"), py_str("wiped")]));
    s_WORN_OUT_CUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("im am feel feeling"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORN_OUT_LOOKBACK(Py::Int(3));
    s_WORN_OUT_BOOST(Py::Int(4));
    s_NAME_BREAKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an the and or but if then than that this these those there here\n    i me my we our us you your he him his she her they them their it its am is are was were be\n    been being do does did have has had will would shall should can could may might must not no\n    so too very just really quite only also as at by for from in into of off on onto out over\n    under up with within without about after again against all any both each few more most other\n    some such own same what which who whom whose when where why how"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEADING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i i'm im i've ive i'll ill i'd id we we're we've you you're\n    he he's she she's it it's they they're"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLACE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("in at from near around across through visit visiting visited\n    outside inside via"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLACE_WORDS_WITH_CUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("to towards toward"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_PLACES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("excel word powerpoint outlook onenote slack zoom teams skype discord\n    facebook instagram twitter whatsapp tiktok youtube netflix spotify amazon ebay etsy google\n    gmail github gitlab python javascript chrome safari windows android iphone ipad mac\n    photoshop canva notion trello jira dropbox onedrive icloud uber airbnb booking expedia\n    linkedin paypal ebay tinder\ndrive docs sheets slides mail calendar\ntesco sainsbury asda aldi lidl waitrose morrisons coop boots argos ikea currys"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORK_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("work works working worked workplace office job jobs shift shifts\n    employer company meeting meetings boss manager team interview roster rota colleagues"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLACE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("the a an my your his her our their this that some any another"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLACE_HOMOGRAPHS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("turkey china brazil bath reading new"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COUNTRY_HOMOGRAPHS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("turkey china brazil"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COUNTRY_CUES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("hot warm cold sunny rainy weather temperature safe dangerous\n    expensive cheap currency capital visa language speak driving flights flight hotel hotels\n    resort beaches beach coast airport north south east west trip holiday visit visiting\n    travel travelling traveling season best part parts city cities region regions\n    january february march april may june july august september october november\n    december"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COMPARE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("or and vs versus compared than"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ARTICLES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("the a an my our your his her their this that these those some any"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NAME_FRAMES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("called named call calls"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORD_DIGITS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[0-9]+(?:[.,][0-9]+)?")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__STEPPED_RELATIONS(py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("son"), py_str("daughter"), py_str("brother"), py_str("sister")]));
    s__GRAND_RELATIONS(py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("son"), py_str("daughter"), py_str("child")]));
    s__KIN_BUILT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STEPPED_RELATIONS()) { let mut word: Py = __item.clone(); __out.push(op_add(&py_str("step"), &word.clone())); } py_list(__out) })() }, &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GRAND_RELATIONS()) { let mut word: Py = __item.clone(); __out.push(op_add(&py_str("grand"), &word.clone())); } py_list(__out) })() })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RELATIONS(op_bitor(&op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nson daughter kid kids child children baby toddler mum mom mother dad father husband wife\npartner boyfriend girlfriend friend friends boss manager colleague brother sister aunt\nuncle cousin gran grandma grandad grandpa grandmother grandfather neighbour neighbour\nlandlord flatmate housemate boss team pet dog cat puppy kitten\nex exes\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__KIN_BUILT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("gran"), py_str("granddad"), py_str("granny")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_PERSON_WORDS(op_bitor(&g_RELATIONS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("mate mates nan nana granny auntie pal lad lass fella\n    bloke guy missus fiance fiancee stepson stepdaughter"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_THE_PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _person_words(args))(&__args) });
    s_NUMBER_FILLER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("a an of"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PORTION_UNITS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("g gram grams gramme grammes kg kilo kilos kilogram kilograms\nlb lbs pound pounds oz ounce ounces ml millilitre millilitres milliliter milliliters\nl litre litres liter liters tsp teaspoon teaspoons tbsp tablespoon tablespoons cup cups\nmile miles km kilometre kilometres kilometer kilometers metre metres meter meters\nfoot feet ft inch inches"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OWNS_THE_UNIT(op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WORD_NUMBERS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_set(vec![py_str("quarter"), py_str("quarters"), py_str("third"), py_str("thirds")])));
    s_A_PORTION_NEEDS_OF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_set(vec![py_str("pound"), py_str("pounds"), py_str("lb"), py_str("lbs")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ONE_IS_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("the this that these those which no any each every another\n    nice good great big small little wrong right other same next last first"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
}
pub fn _work_out_sums(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut after: Py = Py::None;
    let mut hits: Py = Py::None;
    let mut index: Py = Py::None;
    let mut training: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where \"work out\" is arithmetic or deciding, not the job and not the exercise.\n\n    \"work out a percentage\", \"work out my hourly rate\", \"work out how much i owe\" and\n    \"work out if the loan is worth it\" are sums, and there the word \"work\" is no evidence for\n    the work topic either. \"work out at home\", \"work out three times a week\" and \"work out\n    more\" are the exercise, and those keep both the word and the fitness phrase.\n    ");
    hits = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    training = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_EXERCISE_NOUNS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str("work"), py_str("works"), py_str("working"), py_str("worked")]), &word.clone())); if py_truthy(&__b) { __b } else { op_ge(&op_add(&index.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(1))), &py_str("out")))) {
    continue;
    }
    after = slice_or_raise(&norm.clone(), &op_add(&index.clone(), &Py::Int(2)), &op_add(&index.clone(), &Py::Int(8)), &Py::None);
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NUMBER_WORDS(), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&after.clone(), &Py::Int(1)), &py_str("times"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NUMBER_WORDS(), &geti(&after.clone(), &Py::Int(2)))) } } } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&hits.clone(), "add", args))(&__args) };
    continue;
    }
    if py_truthy(&{ let __b = training.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(3), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_EXERCISE_AFTER(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_SUMS_AFTER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NUMBER_WORDS(), &word.clone())) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "isdigit", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&hits.clone(), "add", args))(&__args) };
    }
    }
    return hits.clone();
    Py::None
}
pub fn _head_fog(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut foggy: Py = Py::None;
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says the speaker's own head, brain or mind is foggy or thick.");
    foggy = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk5816 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk5816[0].clone(); let mut word: Py = __unpk5816[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_HEAD_FOG(), &word.clone()))) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&foggy.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_HEAD_FOG(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_HEAD_WORDS(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&foggy.clone()) { let mut other: Py = __item.clone(); __out.push({ let __cmp11: Py = op_sub(&other.clone(), &at.clone()); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &g_HEAD_FOG_REACH()) } else { Py::Bool(false) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _skin_state(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: skip"))) };
    let mut at: Py = Py::None;
    let mut state: Py = Py::None;
    let mut states: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The position of a body part within `SKIN_REACH` of the state of its skin, or None.\n\n    The reach keeps a state said of something else off the body: \"the cracked tile under my\n    feet\" is the tile, and only a part next to the state is the skin (\"my heels are cracked\").\n    ");
    states = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk6344 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk6344[0].clone(); let mut word: Py = __unpk6344[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SKIN_STATES(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&skip.clone(), &at.clone())) } }) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&states.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_SKIN_PARTS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&skip.clone(), &at.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&states.clone()) { let mut state: Py = __item.clone(); __out.push(op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&state.clone(), &at.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_SKIN_REACH())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return at.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_part_plays_up(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: skip"))) };
    let mut at: Py = Py::None;
    let mut up: Py = Py::None;
    let mut ups: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The position of a body part that is playing up, or None.\n\n    The part has to be next to the words, so \"the kids are playing up\" (no part) and \"the\n    boiler is playing up\" (a part of the house, not of a person) are not health lines.\n    ");
    ups = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk506 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk506[0].clone(); let mut word: Py = __unpk506[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("up"))); if !py_truthy(&__b) { __b } else { { let __b = at.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&skip.clone(), &at.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLAYS_UP_BEFORE(), &geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(1))))); if py_truthy(&__b) { __b } else { { let __b = op_ge(&at.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("me"), py_str("my")]), &geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLAYS_UP_BEFORE(), &geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(2))))) } } } } } } } } } } } }) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&ups.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_BODY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&skip.clone(), &at.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ups.clone()) { let mut up: Py = __item.clone(); __out.push({ let __cmp11: Py = op_sub(&up.clone(), &at.clone()); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &g_PLAY_UP_REACH()) } else { Py::Bool(false) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return at.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _named_condition(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut skip: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: skip"))) };
    let mut at: Py = Py::None;
    let mut nexts: Py = Py::None;
    let mut other: Py = Py::None;
    let mut other_at: Py = Py::None;
    let mut other_word: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names a condition that is not, here, its other life.\n\n    The part the word is on has to be within `CONDITION_REACH` for a word in\n    `CONDITION_NEEDS_PART` (\"i have a boil on my back\" is health, \"boil the kettle\" is not),\n    and one of the other-life words has to be absent for a word in `CONDITION_OTHER_LIFE`. An\n    animal that has the condition is the animal's: \"the dog is constipated\" is a pet question\n    with an answer about warm beds and short walks, so the condition counts only where no\n    animal is named, unless a person is named as well (\"my son has nits and so does the cat\").\n    ");
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_PETS()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _person_words(args))(&__args) }, &g_PETS())))) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_CONDITIONS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&skip.clone(), &at.clone())) } }) {
    continue;
    }
    nexts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITION_NOT_NEXT(), "get", args))(&__args) };
    if py_truthy(&{ let __b = nexts.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&at.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&nexts.clone(), &geti(&norm.clone(), &op_add(&at.clone(), &Py::Int(1))))) } } } }) {
    continue;
    }
    other = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITION_OTHER_LIFE(), "get", args))(&__args) };
    if py_truthy(&{ let __b = other.clone(); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &other.clone()) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CONDITION_NEEDS_PART(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk6100 = py_unpack(&__item.clone(), 2, None); let mut other_at: Py = __unpk6100[0].clone(); let mut other_word: Py = __unpk6100[1].clone(); if py_truthy(&Py::Bool(!py_contains(&skip.clone(), &other_at.clone()))) {  __out.push({ let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&other_at.clone(), &at.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_CONDITION_REACH()); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BODY(), &other_word.clone())) } }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    continue;
    }
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITION_NEEDS_WORD(), "get", args))(&__args) };
    if py_truthy(&{ let __b = wanted.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk6901 = py_unpack(&__item.clone(), 2, None); let mut other_at: Py = __unpk6901[0].clone(); let mut other_word: Py = __unpk6901[1].clone(); if py_truthy(&Py::Bool(!py_contains(&skip.clone(), &other_at.clone()))) {  __out.push({ let __b = op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&other_at.clone(), &at.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_CONDITION_REACH()); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&wanted.clone(), &other_word.clone())) } }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _person_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut topic: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every word that can name a person a symptom could be said of.");
    words = op_bitor(&op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_RELATIONS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_EXTRA_PEOPLE()), &g_PETS());
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("family-and-parenting"), py_str("friends-and-relationships")]));
    for __item in __iter1 {
    topic = __item.clone();
    let __aug2 = op_bitor(&words.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&g_STRONG(), &topic.clone()), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    words = __aug2;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_unfit(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut person: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the words say they are not cut out for something, or None.\n\n    Shared with the composer (`bot/modules/chat.py`), which answers it with the pool for a\n    judgement about themselves: the engine decides the line is about how they see themselves, and\n    the composer must not then treat it as a fact about a job.\n\n    The negative and the person are what make it a judgement: \"im cut out for this\" is a boast,\n    \"i never cut out sugar\" is a diet, and \"my brother is not cut out for the army\" is about him.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_CUT_OUT(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    before = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "split", args))(&__args) }, &op_neg(&Py::Int(6)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    person = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&before.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OWN_SELF(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OTHER_PERSON(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&person.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_OWN_SELF(), &geti(&person.clone(), &op_neg(&Py::Int(1))))) } }) {
    continue;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn _a_habit_given_up(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut phrase: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line gives up something they eat or drink, read off the whole line.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_GIVEN_UP_FOOD(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GIVING_UP()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_bin_is_on_a_machine(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the sentence's bin is the one on a computer (\"how do i empty the recycle bin on\n    my laptop\"), False when it says nothing about a machine (\"the bin needs emptying\").\n\n    The machine words above are what settles it, and the technology topic is the same reading\n    from the other side: a line that names a laptop or a disk is about a machine whether or not\n    it uses one of these words. A line that names neither keeps the everyday bin, which is what\n    \"my bin is full\" and \"how do i empty my bin\" are.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_bitand(&words.clone(), &g_MACHINE_BIN_WORDS())) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("topics")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &py_str("technology")));
    Py::None
}
pub fn unclear_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut inherited: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "inherited") { v.clone() }
    else { Py::None };
    let mut previous: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "previous") { v.clone() }
    else { Py::None };
    let mut _cues: Py = Py::None;
    let mut _sense_topics: Py = Py::None;
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut content: Py = Py::None;
    let mut cues: Py = Py::None;
    let mut first: Py = Py::None;
    let mut other: Py = Py::None;
    let mut sense_topics: Py = Py::None;
    let mut tiers: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The two-life word the line leaves unsettled, or None.\n\n    Only the first thing the line names, only when it is the subject of a state (\"the bank was\n    closed\", \"the seal is broken\"), and only when no word in the line fits one of its lives and\n    the conversation has not already settled on a subject. A line with a cue (\"my bank account\",\n    \"the seal on the fridge door\") is not unclear, and neither is a line that only passes the\n    word by (\"we walked past the bank\").\n    ");
    content = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk4658 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk4658[0].clone(); let mut word: Py = __unpk4658[1].clone(); if py_truthy(&Py::Bool(!py_contains(&g__NOT_THE_SUBJECT(), &word.clone()))) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&content.clone()))) {
    return Py::None;
    }
    first = geti(&content.clone(), &Py::Int(0));
    word = geti(&norm.clone(), &first.clone());
    tiers = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TWO_LIVES_FORMS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&tiers.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__THE_OWNER_SETTLES(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = first.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__POSSESSIVE(), &geti(&norm.clone(), &op_sub(&first.clone(), &Py::Int(1))))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = inherited.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tiers.clone()) { let __unpk1822 = py_unpack(&__item.clone(), 2, None); let mut _cues: Py = __unpk1822[0].clone(); let mut sense_topics: Py = __unpk1822[1].clone(); __out.push(Py::Bool(py_contains(&sense_topics.clone(), &inherited.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::None;
    }
    if py_truthy(&previous.clone()) {
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("norm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&previous.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tiers.clone()) { let __unpk3469 = py_unpack(&__item.clone(), 2, None); let mut cues: Py = __unpk3469[0].clone(); let mut _sense_topics: Py = __unpk3469[1].clone(); __out.push(op_bitand(&before.clone(), &cues.clone())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    }
    after = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&content.clone(), &Py::Int(1), &Py::Int(9), &Py::None)) { let mut at: Py = __item.clone(); __out.push(geti(&norm.clone(), &at.clone())); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut other: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_STATE_VERB(), &other.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tiers.clone()) { let __unpk5710 = py_unpack(&__item.clone(), 2, None); let mut cues: Py = __unpk5710[0].clone(); let mut _sense_topics: Py = __unpk5710[1].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(py_list(vec![first.clone()])); __args.push(py_tuple(vec![py_tuple(vec![g_TWO_LIVES_REACH(), cues.clone()])])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sense_cue(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    return word.clone();
    Py::None
}
pub fn _teething_is_a_fault(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let _ = py_str("True when \"teething\" is a new thing finding its feet, not a child cutting teeth.");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_TEETHING_IS_A_FAULT())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_TEETHING_AND_A_CHILD()))) } };
    Py::None
}
pub fn the_warmth_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut back: Py = Py::None;
    let mut habit: Py = Py::None;
    let mut person: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The word for how warm somebody is, when that is what the line says: \"cold\", \"hot\", else None.\n\n    Read over the whole sentence rather than the word: a person, then is/are/feels, then the\n    warmth with no more than a degree word or two in between, then nothing but when it was. So\n    \"my mum is cold\" and \"the kids are cold\" are the warmth, and \"my mum has a cold\", \"my mum\n    is full of cold\", \"my mum is cold all the time\" and \"my hands are cold\" are all something\n    else and keep the reading they had. A habit is not this turn's question either: \"my mum is\n    always cold\" runs like \"cold all the time\" and \"my dad is always hot\" runs like it too. The\n    hot side answers the same question from the other end: \"my nan is boiling\" and \"my son is\n    shivery\" are a temperature to take, not a shrug.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_WARMTH_WORDS(), &word.clone()))) {
    continue;
    }
    back = op_sub(&at.clone(), &Py::Int(1));
    habit = Py::Bool(false);
    while py_truthy(&{ let __b = op_ge(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WARMTH_DEGREE(), &geti(&norm.clone(), &back.clone()))) } }) {
    if py_truthy(&Py::Bool(py_contains(&g_WARMTH_HABITUAL(), &geti(&norm.clone(), &back.clone())))) {
    habit = Py::Bool(true);
    }
    let __aug3 = op_sub(&back.clone(), &Py::Int(1));
    back = __aug3;
    }
    if py_truthy(&{ let __b = op_lt(&back.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_WARMTH_ARE(), &geti(&norm.clone(), &back.clone()))) } }) {
    continue;
    }
    if py_truthy(&habit.clone()) {
    continue;
    }
    who = geti(&norm.clone(), &op_sub(&back.clone(), &Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&g_COLD_PEOPLE(), &who.clone()))) {
    person = Py::Bool(true);
    } else {
    if py_truthy(&Py::Bool(py_contains(&g_PERSON_WORDS(), &who.clone()))) {
    person = Py::Bool(true);
    } else {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&who.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERSON_WORDS(), &slice_or_raise(&who.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    person = Py::Bool(true);
    } else {
    person = Py::Bool(false);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&person.clone()))) {
    continue;
    }
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_WARMTH_THEN(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("."), py_str("!"), py_str("?"), py_str(",")]), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    continue;
    }
    return word.clone();
    }
    return Py::None;
    Py::None
}
pub fn _means_something_else(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut tiers: Py = Py::None;
    let _ = py_str("True when the word at `position` names another thing in this line (SENSE_ELSEWHERE).");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&norm.clone(), &position.clone()), &py_str("teething"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _teething_is_a_fault(args))(&__args) } } }) {
    return Py::Bool(true);
    }
    tiers = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&norm.clone(), &position.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENSE_ELSEWHERE_FORMS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&tiers.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BODY_TWO_LIFE(), &geti(&norm.clone(), &position.clone()))); if !py_truthy(&__b) { __b } else { { let __b = position.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNING_WORDS(), &geti(&norm.clone(), &op_sub(&position.clone(), &Py::Int(1))))) } } } }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(py_list(vec![position.clone()])); __args.push(tiers.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sense_cue(args))(&__args) };
    Py::None
}
pub fn says_another_thing(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The two-life words in a line that a cue has taken to their other life, as a set.\n\n    `text` is a normalised line, so the words are split on anything that is not one.\n    ");
    norm = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z0-9]+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| names_another_thing(args))(&__args) }) { let mut at: Py = __item.clone(); __out.push(geti(&norm.clone(), &at.clone())); } py_set(__out) })() };
    Py::None
}
pub fn names_another_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The positions in `norm` of two-life words a cue has taken to their other life.\n\n    The topics read drops these words; a table that looks for a dish by name has its own match\n    and needs the same answer, or \"how do i check my breasts for lumps\" keeps the chicken and\n    \"a lump in my breast\" does not.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk1314 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk1314[0].clone(); let mut word: Py = __unpk1314[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SENSE_ELSEWHERE_FORMS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _means_something_else(args))(&__args) } } }) {  __out.push(at.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn is_a_machine_word(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line's \"mouse\" is the computer part, not the animal.\n\n    \"my mouse isnt working\", \"i lost my mouse\" and \"my mouse battery is dead\" are the device;\n    \"we have mice in the kitchen\", \"my cat caught a mouse\" and \"my pet mouse died\" are not.\n    A line with no cue at all (\"my mouse\") is read as the device, which is the everyday meaning.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[^a-z]+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&words.clone(), &py_str("mouse"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&words.clone(), &py_str("mice"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&words.clone(), &py_str("mousepad"))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("mice")))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MACHINE_WORD_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&op_bitand(&words.clone(), &geti(&g_MACHINE_WORD_CUES(), &py_str("mouse")))));
    Py::None
}
pub fn _the_sense_the_cues_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut _topic: Py = Py::None;
    let mut tiers: Py = Py::None;
    let _ = py_str("True when a cue word beside this one names the other sense of a two-life word.\n\n    \"a mole in the garden\" is the lawn's, though \"mole\" on its own is the doctor's, and \"the\n    stock market\" is money though \"stock\" alone is the shop's. The word keeps its own weight\n    only where nothing beside it says which thing is meant.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SENSE_TIERS(), "get", args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    _topic = __unpk2[0].clone();
    tiers = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(py_list(vec![position.clone()])); __args.push(tiers.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sense_cue(args))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _sense_cue(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut positions: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "positions") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: positions"))) };
    let mut tiers: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tiers") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tiers"))) };
    let mut cue_at: Py = Py::None;
    let mut cues: Py = Py::None;
    let mut reach: Py = Py::None;
    let mut spot: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a cue word sits near one of the two-life words (see SENSE_PAIRS).");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    cue_at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&tiers.clone());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    reach = __unpk4[0].clone();
    cues = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&cues.clone(), &word.clone()))) {
    continue;
    }
    let __iter5 = py_iter_or_raise(&positions.clone());
    for __item in __iter5 {
    spot = __item.clone();
    if py_truthy(&{ let __cmp61: Py = cue_at.clone(); if py_truthy(&op_le(&op_sub(&spot.clone(), &reach.clone()), &__cmp61)) { op_le(&__cmp61, &op_add(&spot.clone(), &reach.clone())) } else { Py::Bool(false) } }) {
    return Py::Bool(true);
    }
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _index(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut add: Py = Py::None;
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    let mut ordinary: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut table: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("word/phrase -> [(topic, weight)], built once from the data above.");
    table = py_dict(vec![]);
pub fn __nested_add_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut add: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut key: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut ordinary: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut phrase: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut phrases: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut table: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut topic: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut key: Py = __pos.get(10).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "key").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut topic: Py = __pos.get(11).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "topic").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut weight: Py = __pos.get(12).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "weight").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut entries: Py = Py::None;
    let mut index: Py = Py::None;
    let mut points: Py = Py::None;
    let mut seen: Py = Py::None;
    entries = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); __args.push(py_list(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&table.clone(), "setdefault", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entries.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    seen = __unpk3[0].clone();
    points = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&seen.clone(), &topic.clone()))) {
    if py_truthy(&op_gt(&weight.clone(), &points.clone())) {
    let _ = py_setitem(&entries.clone(), &index.clone(), py_tuple(vec![topic.clone(), weight.clone()]));
    }
    return Py::None;
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![topic.clone(), weight.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entries.clone(), "append", args))(&__args) };
    Py::None
}
    add = {{ let add = add.clone(); let key = key.clone(); let name = name.clone(); let ordinary = ordinary.clone(); let phrase = phrase.clone(); let phrases = phrases.clone(); let table = table.clone(); let topic = topic.clone(); let word = word.clone(); let words = words.clone(); py_func("add", move |args: &[Py]| { let mut full = vec![add.clone(), key.clone(), name.clone(), ordinary.clone(), phrase.clone(), phrases.clone(), table.clone(), topic.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_add_0(&full) }) }};
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_STRONG(), "items", args))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    topic = __unpk3[0].clone();
    words = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) });
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_GENERIC(), &word.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(topic.clone()); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    }
    }
    }
    let __iter5 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WEAK(), "items", args))(&__args) });
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    topic = __unpk6[0].clone();
    words = __unpk6[1].clone();
    let __iter7 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "split", args))(&__args) });
    for __item in __iter7 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_GENERIC(), &word.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(topic.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    }
    }
    }
    let __iter8 = py_iter_or_raise(&g_CAR_NAMES());
    for __item in __iter8 {
    name = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); __args.push(py_str("cars")); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    }
    let __iter9 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PHRASES(), "items", args))(&__args) });
    for __item in __iter9 {
    let __unpk10 = py_unpack(&__item.clone(), 2, None);
    topic = __unpk10[0].clone();
    phrases = __unpk10[1].clone();
    let __iter11 = py_iter_or_raise(&phrases.clone());
    for __item in __iter11 {
    phrase = __item.clone();
    key = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BREAKS(), "sub", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = key.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_GENERIC(), &key.clone())) } }) {
    ordinary = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&key.clone(), "split", args))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ORDINARY_VERBS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); __args.push(topic.clone()); __args.push(if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ASK_PHRASES(), &key.clone())); if py_truthy(&__b) { __b } else { ordinary.clone() } }) { Py::Int(1) } else { Py::Int(3) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    }
    }
    }
    return table.clone();
    Py::None
}
pub fn _takes_an_object(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let _ = py_str("True when a word at `position` is followed by a determiner and the thing it names.");
    return { let __b = op_lt(&op_add(&position.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1))))) } };
    Py::None
}
pub fn _acts_on_a_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut topic: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topic"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut norm: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut start: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut offset: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a verb cue in these words is followed by the thing it acts on: \"revise my cv\".\n    The test runs after the cue itself, not after the whole phrase.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    offset = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_VERB_CUES(), "get", args))(&__args) }, &word.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(op_add(&start.clone(), &offset.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _takes_an_object(args))(&__args) };
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _thing_a_cue_acts_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut index: Py = Py::None;
    let mut noun: Py = Py::None;
    let _ = py_str("The head noun of the thing after a cue, or \"\" when it names nothing: \"the pan\",\n    \"the bottom of the pan\" -> pan.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _takes_an_object(args))(&__args) }))) {
    return py_str("");
    }
    index = op_add(&position.clone(), &Py::Int(2));
    noun = if py_truthy(&op_lt(&index.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&norm.clone(), &index.clone()) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DEGREE_NOUNS(), &noun.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(1))), &py_str("of"))) } })) } }) {
    return py_str("");
    }
    while py_truthy(&{ let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(1))), &py_str("of"))) } }) {
    let __aug1 = op_add(&index.clone(), &Py::Int(2));
    index = __aug1;
    if py_truthy(&{ let __b = op_lt(&index.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &geti(&norm.clone(), &index.clone()))) } }) {
    let __aug2 = op_add(&index.clone(), &Py::Int(1));
    index = __aug2;
    }
    noun = if py_truthy(&op_lt(&index.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&norm.clone(), &index.clone()) } else { noun.clone() };
    }
    return noun.clone();
    Py::None
}
pub fn _acts_on_the_body(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut after: Py = Py::None;
    let mut index: Py = Py::None;
    let mut others: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a burn or a cut at `position` is on the body, or names nothing at all.");
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing_a_cue_acts_on(args))(&__args) };
    if py_truthy(&thing.clone()) {
    return Py::Bool(py_contains(&g_BODY_OR_SELF(), &thing.clone()));
    }
    after = slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &op_add(&position.clone(), &Py::Int(2)), &Py::None);
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_BODY_OR_SELF(), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_CUE_CARRIERS(), &geti(&after.clone(), &Py::Int(0)))) } } } }) {
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk1188 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk1188[0].clone(); let mut word: Py = __unpk1188[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_eq(&index.clone(), &position.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_POLY_FORMS(), &word.clone())) } }) {  for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _lookup(args))(&__args) }) { let __unpk1476 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk1476[0].clone(); let mut weight: Py = __unpk1476[1].clone(); if py_truthy(&{ let __b = op_ge(&weight.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("health"), py_str("small-talk")]), &topic.clone())) } }) {  __out.push(topic.clone()); } } } } py_set(__out) })() };
    return Py::Bool(!py_truthy(&others.clone()));
    }
    return Py::Bool(true);
    Py::None
}
pub fn _is_run_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let _ = py_str("True when the word at `position` is the `run` of \"run out of\" rather than a run.");
    return { let __b = Py::Bool(py_contains(&g_RUN_OUT_BEFORE(), &geti(&norm.clone(), &position.clone()))); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&position.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1))), &py_str("out"))) } } } };
    Py::None
}
pub fn _watches_out_for(args: &[Py]) -> Py {
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
    let mut norm: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let _ = py_str("True when the watching in this phrase is followed by the for or out of \"watch for\".");
    return { let __b = slice_or_raise(&words.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WATCHING(), &geti(&words.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&end.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WATCH_OUT_AFTER(), &geti(&norm.clone(), &end.clone()))) } } } } } };
    Py::None
}
pub fn _says_nothing_here(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut topic: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topic"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut norm: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut start: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut study: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "study") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: study"))) };
    let mut rest: Py = Py::None;
    let mut topic_: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this word or phrase cannot name `topic` in this line: the same words are an\n    ordinary verb here, and the line does not say what they act on.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(norm.clone()); __args.push(op_add(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _watches_out_for(args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&topic.clone(), &py_str("family-and-parenting"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &py_str("health visitor"))) } }) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health visitors")); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health visitor")); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "replace", args))(&__args) }, "replace", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _lookup(args))(&__args) }) { let __unpk6466 = py_unpack(&__item.clone(), 2, None); let mut topic_: Py = __unpk6466[0].clone(); let mut weight: Py = __unpk6466[1].clone(); __out.push({ let __b = Py::Bool(py_eq(&topic_.clone(), &py_str("health"))); if !py_truthy(&__b) { __b } else { op_ge(&weight.clone(), &Py::Int(2)) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BODY_CUES(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(start.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _acts_on_the_body(args))(&__args) }));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_VERB_CUES(), "get", args))(&__args) }, &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(words.clone()); __args.push(norm.clone()); __args.push(start.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _acts_on_a_thing(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&study.clone())) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _age_value(args: &[Py]) -> Py {
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
    let mut digits: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The number a child's age is said with, or a large value when it is not a small count.");
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD_NUMBERS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    digits = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD_DIGITS(), "match", args))(&__args) };
    value = if py_truthy(&digits.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&digits.clone(), "group", args))(&__args) }, "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { Py::Int(99) };
    }
    return value.clone();
    Py::None
}
pub fn _ages_of_a_child(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut ages: Py = Py::None;
    let mut number_at: Py = Py::None;
    let mut position: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The positions of a \"a 7 year old\" in this line: a child, not a machine's age.");
    ages = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    position = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_AGE_NOUNS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&position.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { op_ge(&op_add(&position.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1))), &py_str("old")))) {
    continue;
    }
    number_at = op_sub(&position.clone(), &Py::Int(1));
    if py_truthy(&Py::Bool(py_contains(&g_AGE_AFTER(), &geti(&norm.clone(), &number_at.clone())))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&number_at.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DETERMINERS(), &geti(&norm.clone(), &op_sub(&number_at.clone(), &Py::Int(1))))) } }) {
    continue;
    }
    if py_truthy(&op_gt(&op_truediv(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&norm.clone(), &number_at.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _age_value(args))(&__args) }, &geti(&g_AGE_IN_A_YEAR(), &word.clone())), &g_A_CHILD_TILL())) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number_at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ages.clone(), "append", args))(&__args) };
    }
    return ages.clone();
    Py::None
}
pub fn _a_book_for_a_child(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut child_ages: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "child_ages") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: child_ages"))) };
    let mut after: Py = Py::None;
    let mut age: Py = Py::None;
    let mut position: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The position of \"book for a 10 year old\" in this line, or None.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    position = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_BOOK_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &op_add(&position.clone(), &Py::Int(2)), &Py::None), &py_list(vec![py_str("for")]))) } }) {
    continue;
    }
    after = op_add(&position.clone(), &Py::Int(2));
    tail = slice_or_raise(&norm.clone(), &after.clone(), &op_add(&after.clone(), &Py::Int(5)), &Py::None);
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_A_CHILD_TO_READ_FOR())) {
    return position.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&child_ages.clone()) { let mut age: Py = __item.clone(); __out.push({ let __cmp11: Py = age.clone(); if py_truthy(&op_le(&after.clone(), &__cmp11)) { op_le(&__cmp11, &op_add(&after.clone(), &Py::Int(3))) } else { Py::Bool(false) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return position.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&tail.clone(), &py_str("year"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&tail.clone(), &py_str("old"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&tail.clone(), &py_str("olds"))) } } } }) {
    return position.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _drives_a_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut tail: Py = Py::None;
    let _ = py_str("True for \"driving me mad\": a person object and then a mood, not a destination.");
    if py_truthy(&Py::Bool(!py_contains(&g_DRIVES(), &geti(&norm.clone(), &position.clone())))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &op_add(&position.clone(), &Py::Int(2)), &Py::None), &py_list(vec![]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DRIVEN_PERSON(), &geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &op_add(&position.clone(), &Py::Int(2)), &Py::None)))) {
    return Py::Bool(false);
    }
    tail = slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(2)), &op_add(&position.clone(), &Py::Int(7)), &Py::None);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_DRIVEN_MOOD()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_DRIVEN_TAIL()); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_DRIVEN_JOINERS()) } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _idiom_positions(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut needle: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut spots: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The positions taken by a phrase in `IDIOM_PHRASES`, or an empty set.\n\n    Only a whole phrase counts, so `moon` on its own is still the sky, `cake` on its own is\n    still to eat, and `my feet are killing me` is still about feet.\n    ");
    spots = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    joined = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g_IDIOM_PHRASES());
    for __item in __iter1 {
    phrase = __item.clone();
    needle = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
    start = Py::Int(0);
    while py_truthy(&Py::Bool(true)) {
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(needle.clone()); __args.push(start.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&joined.clone(), "find", args))(&__args) };
    if py_truthy(&op_lt(&at.clone(), &Py::Int(0))) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&joined.clone(), &Py::None, &at.clone(), &Py::None), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&joined.clone(), &Py::None, &at.clone(), &Py::None), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&spots.clone(), "update", args))(&__args) };
    start = op_add(&at.clone(), &Py::Int(1));
    }
    }
    return spots.clone();
    Py::None
}
pub fn _bad_verdict_positions(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = Py::None;
    let _ = py_str("The positions of a verdict on a thing: \"my landlord is a joke\", \"the nhs is a shambles\".\n\n    The noun is what the person thinks of the thing they named, and it is not the subject of the\n    line: \"the nhs is a joke\" is about the nhs, and read as a line about a joke it went to the\n    punchline pool. Nothing inside the verdict scores a topic or names an entity, which is what\n    makes \"is a joke\" a comment on the thing in front of it.\n    ");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut position: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::a_bad_verdict(args))(&__args) }) {  __out.push(position.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _drive_idiom(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut idiom: Py = Py::None;
    let mut offset: Py = Py::None;
    let mut position: Py = Py::None;
    let _ = py_str("The positions of a \"driving me up the wall\" idiom: its verb, its person and its mood.\n\n    Inside the idiom no word names a thing: `wall` is not the wall of a house and `bend` is not\n    a road, so a line made of the idiom alone (`driving me up the wall`, `shes driving me round\n    the bend`) must not be read as home-and-diy or a journey. The idiom says how the speaker\n    feels, and the topic comes from whatever the line names outside it (`the kids are driving me\n    up the wall` is still family-and-parenting).\n    ");
    idiom = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    position = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _drives_a_person(args))(&__args) }))) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![position.clone(), op_add(&position.clone(), &Py::Int(1))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&idiom.clone(), "update", args))(&__args) };
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&position.clone(), &Py::Int(2))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&position.clone(), &Py::Int(7))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    offset = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DRIVEN_MOOD(), &geti(&norm.clone(), &offset.clone()))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DRIVEN_TAIL(), &geti(&norm.clone(), &offset.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DRIVEN_JOINERS(), &geti(&norm.clone(), &offset.clone()))) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(offset.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&idiom.clone(), "add", args))(&__args) };
    }
    }
    }
    return idiom.clone();
    Py::None
}
pub fn _is_a_complaint(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let _ = py_str("True when the word at `position` is \"sick of\"/\"tired of\": a complaint, not a symptom.");
    if py_truthy(&Py::Bool(!py_contains(&g_COMPLAINT_OF(), &geti(&norm.clone(), &position.clone())))) {
    return Py::Bool(false);
    }
    return { let __b = op_lt(&op_add(&position.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COMPLAINT_OF_NEXT(), &geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1))))) } };
    Py::None
}
pub fn _function_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let _ = py_str("The words that are only grammar: they are never another word with an ending on.");
    if py_truthy(&Py::Bool(py_is(&g__FUNCTION_WORDS(), &Py::None))) {
    let __try1 = exc::catch(|| {
    let mut STOPWORDS: Py = crate::understanding::g_STOPWORDS();
    s__FUNCTION_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(STOPWORDS.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    s__FUNCTION_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    return g__FUNCTION_WORDS();
    Py::None
}
pub fn _family(args: &[Py]) -> Py {
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
    let mut here: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut there: Py = Py::None;
    let _ = py_str("The table entries for another form of this word, if the list holds that form.");
    let __iter1 = py_iter_or_raise(&g_DERIVED_ENDINGS());
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ending.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None);
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stem.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FAMILY_STEM())) {
    hit = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stem.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&stem.clone(), &py_str("e"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "get", args))(&__args) } } };
    if py_truthy(&hit.clone()) {
    return hit.clone();
    }
    }
    }
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_FAMILY_WORD()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _function_words(args))(&__args) }, &word.clone())) } }) {
    hit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&word.clone(), &ending.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "get", args))(&__args) };
    if py_truthy(&hit.clone()) {
    return hit.clone();
    }
    }
    }
    let __iter2 = py_iter_or_raise(&g_FAMILY_PAIRS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    here = __unpk3[0].clone();
    there = __unpk3[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    hit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None), &there.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "get", args))(&__args) };
    if py_truthy(&hit.clone()) {
    return hit.clone();
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(there.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    hit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(there.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None), &here.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "get", args))(&__args) };
    if py_truthy(&hit.clone()) {
    return hit.clone();
    }
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _lookup(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut replacement: Py = Py::None;
    let mut suffix: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut weight: Py = Py::None;
    let _ = py_str("The table entries for a word, also trying its singular and verb stem.");
    hit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "get", args))(&__args) };
    if py_truthy(&hit.clone()) {
    return hit.clone();
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("ies"), py_str("y")]), py_tuple(vec![py_str("es"), py_str("")]), py_tuple(vec![py_str("s"), py_str("")]), py_tuple(vec![py_str("ing"), py_str("")]), py_tuple(vec![py_str("ing"), py_str("e")]), py_tuple(vec![py_str("ed"), py_str("")]), py_tuple(vec![py_str("ed"), py_str("e")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    suffix = __unpk2[0].clone();
    replacement = __unpk2[1].clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(suffix.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(suffix.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) } }) {
    hit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(suffix.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None), &replacement.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "get", args))(&__args) };
    if py_truthy(&hit.clone()) {
    return hit.clone();
    }
    }
    }
    family = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _family(args))(&__args) };
    if py_truthy(&family.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&family.clone()) { let __unpk6593 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk6593[0].clone(); let mut weight: Py = __unpk6593[1].clone(); if py_truthy(&Py::Bool(!py_eq(&topic.clone(), &py_str("about-the-bot")))) {  __out.push(py_tuple(vec![topic.clone(), weight.clone()])); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _is_a_portion(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut after: Py = Py::None;
    let _ = py_str("True for \"half a cabbage\", \"half the batter\", \"double the recipe\".\n\n    A quantity word in front of an article is a piece of a thing the line already names, which\n    is not a sum: \"ive got half a cabbage left, what can i do with it\" is a cooking line. A\n    figure after it (\"half of 80\") is arithmetic and keeps its full weight.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&g_PORTION_WORDS(), &geti(&norm.clone(), &position.clone())))) {
    return Py::Bool(false);
    }
    after = slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &op_add(&position.clone(), &Py::Int(3)), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &geti(&after.clone(), &Py::Int(0))))) {
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PORTION_UNITS(), &geti(&after.clone(), &Py::Int(1)))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_eq(&geti(&after.clone(), &Py::Int(0)), &py_str("of"))); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the")]), &geti(&after.clone(), &Py::Int(1)))) } } } };
    Py::None
}
pub fn _the_weather_is_just_a_season(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut _weight: Py = Py::None;
    let mut at: Py = Py::None;
    let mut entries: Py = Py::None;
    let mut key: Py = Py::None;
    let mut spots: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the only weather in the line is a season word.\n\n    A season phrase (\"in the spring\", \"next winter\") counts as the season, not as weather of its\n    own. A temperature or a sky word does not: \"is it warm in the summer in spain\" is still the\n    weather.\n    ");
    spots = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk1505 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk1505[0].clone(); let mut word: Py = __unpk1505[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_SEASONS(), &word.clone()))) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&spots.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spots.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spots.clone()) { let mut at: Py = __item.clone(); if py_truthy(&at.clone()) {  __out.push(Py::Bool(py_contains(&g_WEATHER_BEFORE(), &geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(1)))))); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_WEATHER_ITSELF(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _lookup(args))(&__args) }) { let __unpk7808 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk7808[0].clone(); let mut weight: Py = __unpk7808[1].clone(); __out.push({ let __b = Py::Bool(!py_contains(&g_SEASONS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&topic.clone(), &py_str("nature-and-weather"))); if !py_truthy(&__b) { __b } else { op_ge(&weight.clone(), &Py::Int(2)) } } } }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "items", args))(&__args) }) { let __unpk6464 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk6464[0].clone(); let mut entries: Py = __unpk6464[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&key.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entries.clone()) { let __unpk9331 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk9331[0].clone(); let mut _weight: Py = __unpk9331[1].clone(); __out.push(Py::Bool(py_eq(&topic.clone(), &py_str("nature-and-weather")))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {  __out.push({ let __b = Py::Bool(py_contains(&text.clone(), &key.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&key.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_SEASONS()))) } }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn score_topics(args: &[Py]) -> Py {
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
    let mut inherited: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "inherited") { v.clone() }
    else { Py::None };
    let mut _first: Py = Py::None;
    let mut _tiers: Py = Py::None;
    let mut _top: Py = Py::None;
    let mut _topic: Py = Py::None;
    let mut _weight: Py = Py::None;
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut background: Py = Py::None;
    let mut book_at: Py = Py::None;
    let mut child_ages: Py = Py::None;
    let mut elsewhere: Py = Py::None;
    let mut entries: Py = Py::None;
    let mut family_score: Py = Py::None;
    let mut first: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut forms_of: Py = Py::None;
    let mut found: Py = Py::None;
    let mut idiom: Py = Py::None;
    let mut key: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut note: Py = Py::None;
    let mut other: Py = Py::None;
    let mut output_at: Py = Py::None;
    let mut own: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut position: Py = Py::None;
    let mut ranked: Py = Py::None;
    let mut score: Py = Py::None;
    let mut sense_topic: Py = Py::None;
    let mut spots: Py = Py::None;
    let mut start: Py = Py::None;
    let mut stone_at: Py = Py::None;
    let mut study: Py = Py::None;
    let mut sums: Py = Py::None;
    let mut text: Py = Py::None;
    let mut text_norm: Py = Py::None;
    let mut tiers: Py = Py::None;
    let mut tight_at: Py = Py::None;
    let mut top: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut unfit: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("[(topic, score, best weight)] for this line, best first.\n\n    The best weight is the strongest single thing that pointed at the topic: 2 or more means a\n    distinctive word or a phrase, 1 means only ordinary words (see `subject_topic`).\n    ");
    norm = geti(&line.clone(), &py_str("norm"));
    text = op_add(&op_add(&py_str(" "), &geti(&line.clone(), &py_str("norm_text"))), &py_str(" "));
    sums = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _work_out_sums(args))(&__args) };
    study = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_STUDY_OBJECTS())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    background = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::background_positions(args))(&__args) };
    let __unpk1 = py_unpack(&py_tuple(vec![py_dict(vec![]), py_dict(vec![])]), 2, None);
    found = __unpk1[0].clone();
    own = __unpk1[1].clone();
pub fn __nested_note_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _first: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut _tiers: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut _top: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut _topic: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut _weight: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut after: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut at: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut background: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut book_at: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut child_ages: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut elsewhere: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut entries: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut family_score: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut first: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut form: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut forms: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut forms_of: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut found: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut idiom: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut inherited: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut key: Py = __pos.get(20).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(21).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(22).cloned().unwrap_or(Py::None);
    let mut named: Py = __pos.get(23).cloned().unwrap_or(Py::None);
    let mut norm: Py = __pos.get(24).cloned().unwrap_or(Py::None);
    let mut note: Py = __pos.get(25).cloned().unwrap_or(Py::None);
    let mut other: Py = __pos.get(26).cloned().unwrap_or(Py::None);
    let mut output_at: Py = __pos.get(27).cloned().unwrap_or(Py::None);
    let mut own: Py = __pos.get(28).cloned().unwrap_or(Py::None);
    let mut pattern: Py = __pos.get(29).cloned().unwrap_or(Py::None);
    let mut position: Py = __pos.get(30).cloned().unwrap_or(Py::None);
    let mut ranked: Py = __pos.get(31).cloned().unwrap_or(Py::None);
    let mut score: Py = __pos.get(32).cloned().unwrap_or(Py::None);
    let mut sense_topic: Py = __pos.get(33).cloned().unwrap_or(Py::None);
    let mut spots: Py = __pos.get(34).cloned().unwrap_or(Py::None);
    let mut start: Py = __pos.get(35).cloned().unwrap_or(Py::None);
    let mut stone_at: Py = __pos.get(36).cloned().unwrap_or(Py::None);
    let mut study: Py = __pos.get(37).cloned().unwrap_or(Py::None);
    let mut sums: Py = __pos.get(38).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(39).cloned().unwrap_or(Py::None);
    let mut text_norm: Py = __pos.get(40).cloned().unwrap_or(Py::None);
    let mut tiers: Py = __pos.get(41).cloned().unwrap_or(Py::None);
    let mut tight_at: Py = __pos.get(42).cloned().unwrap_or(Py::None);
    let mut top: Py = __pos.get(43).cloned().unwrap_or(Py::None);
    let mut topic: Py = __pos.get(44).cloned().unwrap_or(Py::None);
    let mut unfit: Py = __pos.get(45).cloned().unwrap_or(Py::None);
    let mut weight: Py = __pos.get(46).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(47).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(48).cloned().unwrap_or(Py::None);
    let mut topic: Py = __pos.get(49).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "topic").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut weight: Py = __pos.get(50).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "weight").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut position: Py = __pos.get(51).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "position").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut polysemous: Py = __pos.get(52).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "polysemous").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_tuple(vec![Py::Int(0), Py::Int(0), position.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) }, 3, None);
    score = __unpk1[0].clone();
    top = __unpk1[1].clone();
    first = __unpk1[2].clone();
    let _ = py_setitem(&found.clone(), &topic.clone(), py_tuple(vec![op_add(&score.clone(), &weight.clone()), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(top.clone()); __args.push(weight.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, first.clone()]));
    if py_truthy(&Py::Bool(!py_truthy(&polysemous.clone()))) {
    let _ = py_setitem(&own.clone(), &topic.clone(), Py::Bool(true));
    }
    Py::None
}
    note = {{ let _first = _first.clone(); let _tiers = _tiers.clone(); let _top = _top.clone(); let _topic = _topic.clone(); let _weight = _weight.clone(); let after = after.clone(); let at = at.clone(); let background = background.clone(); let book_at = book_at.clone(); let child_ages = child_ages.clone(); let elsewhere = elsewhere.clone(); let entries = entries.clone(); let family_score = family_score.clone(); let first = first.clone(); let form = form.clone(); let forms = forms.clone(); let forms_of = forms_of.clone(); let found = found.clone(); let idiom = idiom.clone(); let inherited = inherited.clone(); let key = key.clone(); let line = line.clone(); let match_ = match_.clone(); let named = named.clone(); let norm = norm.clone(); let note = note.clone(); let other = other.clone(); let output_at = output_at.clone(); let own = own.clone(); let pattern = pattern.clone(); let position = position.clone(); let ranked = ranked.clone(); let score = score.clone(); let sense_topic = sense_topic.clone(); let spots = spots.clone(); let start = start.clone(); let stone_at = stone_at.clone(); let study = study.clone(); let sums = sums.clone(); let text = text.clone(); let text_norm = text_norm.clone(); let tiers = tiers.clone(); let tight_at = tight_at.clone(); let top = top.clone(); let topic = topic.clone(); let unfit = unfit.clone(); let weight = weight.clone(); let word = word.clone(); let words = words.clone(); py_func("note", move |args: &[Py]| { let mut full = vec![_first.clone(), _tiers.clone(), _top.clone(), _topic.clone(), _weight.clone(), after.clone(), at.clone(), background.clone(), book_at.clone(), child_ages.clone(), elsewhere.clone(), entries.clone(), family_score.clone(), first.clone(), form.clone(), forms.clone(), forms_of.clone(), found.clone(), idiom.clone(), inherited.clone(), key.clone(), line.clone(), match_.clone(), named.clone(), norm.clone(), note.clone(), other.clone(), output_at.clone(), own.clone(), pattern.clone(), position.clone(), ranked.clone(), score.clone(), sense_topic.clone(), spots.clone(), start.clone(), stone_at.clone(), study.clone(), sums.clone(), text.clone(), text_norm.clone(), tiers.clone(), tight_at.clone(), top.clone(), topic.clone(), unfit.clone(), weight.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_note_1(&full) }) }};
    forms_of = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENSE_PAIRS(), "values", args))(&__args) }) { let __unpk4420 = py_unpack(&__item.clone(), 3, None); let mut forms: Py = __unpk4420[0].clone(); let mut _topic: Py = __unpk4420[1].clone(); let mut _tiers: Py = __unpk4420[2].clone(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_set(__out) })() };
    idiom = op_bitor(&op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _drive_idiom(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _idiom_positions(args))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _bad_verdict_positions(args))(&__args) });
    child_ages = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ages_of_a_child(args))(&__args) };
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    position = __unpk4[0].clone();
    word = __unpk4[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&sums.clone(), &position.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&background.clone(), &position.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&idiom.clone(), &position.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_run_out(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_a_complaint(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACES(), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_person(args))(&__args) } } }) {
    continue;
    }
    elsewhere = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _means_something_else(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WARMTH_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_warmth_said(args))(&__args) } } }) {
    elsewhere = Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&forms_of.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_sense_the_cues_name(args))(&__args) } } }) {
    continue;
    }
    let __iter5 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _lookup(args))(&__args) });
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    topic = __unpk6[0].clone();
    weight = __unpk6[1].clone();
    if py_truthy(&elsewhere.clone()) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_list(vec![word.clone()])); __args.push(norm.clone()); __args.push(position.clone()); __args.push(study.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says_nothing_here(args))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(weight.clone()); __args.push(position.clone()); __args.push({ let __b = Py::Bool(py_contains(&g_POLY_FORMS(), &word.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_a_portion(args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    }
    let __iter7 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SENSE_PAIRS(), "values", args))(&__args) });
    for __item in __iter7 {
    let __unpk8 = py_unpack(&__item.clone(), 3, None);
    forms = __unpk8[0].clone();
    sense_topic = __unpk8[1].clone();
    tiers = __unpk8[2].clone();
    spots = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk3877 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk3877[0].clone(); let mut word: Py = __unpk3877[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&forms_of.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&forms.clone(), &word.clone())) } }) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = spots.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(spots.clone()); __args.push(tiers.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sense_cue(args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sense_topic.clone()); __args.push(g_SENSE_WEIGHT()); __args.push(geti(&spots.clone(), &Py::Int(0))); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    }
    family_score = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("family-and-parenting")); __args.push(py_tuple(vec![Py::Int(0), Py::Int(0), Py::Int(0)])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) }, &Py::Int(0));
    if py_truthy(&{ let __b = child_ages.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&family_score.clone(), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) }) { let __unpk5125 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk5125[0].clone(); let __unpk4981 = py_unpack(&__unpk5125[1].clone(), 3, None); let mut score: Py = __unpk4981[0].clone(); let mut _top: Py = __unpk4981[1].clone(); let mut _first: Py = __unpk4981[2].clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("family-and-parenting"), py_str("feelings")]), &topic.clone()))) {  __out.push(op_ge(&score.clone(), &Py::Int(2))); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("family-and-parenting")); __args.push(Py::Int(2)); __args.push(geti(&child_ages.clone(), &Py::Int(0))); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    book_at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(child_ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_book_for_a_child(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&book_at.clone(), &Py::None))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("entertainment")); __args.push(Py::Int(3)); __args.push(book_at.clone()); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    if py_truthy(&{ let __b = sums.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "values", args))(&__args) }) { let __unpk868 = py_unpack(&__item.clone(), 3, None); let mut score: Py = __unpk868[0].clone(); let mut _top: Py = __unpk868[1].clone(); let mut _first: Py = __unpk868[2].clone(); __out.push(op_ge(&score.clone(), &Py::Int(2))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("maths-and-numbers")); __args.push(Py::Int(2)); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sums.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    output_at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_persons_own_output(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&output_at.clone(), &Py::None))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(Py::Int(2)); __args.push(output_at.clone()); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    stone_at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stone_weight_at(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&stone_at.clone(), &Py::None))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fitness")); __args.push(Py::Int(3)); __args.push(stone_at.clone()); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    tight_at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tight_money_at(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&tight_at.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) }) { let __unpk7219 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk7219[0].clone(); let __unpk7123 = py_unpack(&__unpk7219[1].clone(), 3, None); let mut score: Py = __unpk7123[0].clone(); let mut _top: Py = __unpk7123[1].clone(); let mut _first: Py = __unpk7123[2].clone(); if py_truthy(&Py::Bool(!py_eq(&topic.clone(), &py_str("money")))) {  __out.push(op_ge(&score.clone(), &Py::Int(2))); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("money")); __args.push(Py::Int(3)); __args.push(tight_at.clone()); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    if py_truthy(&{ let __b = sums.clone(); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_SUM_MONEY()) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("money")); __args.push(Py::Int(3)); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sums.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&crate::understanding::kinds::g_ADDRESS_WORDS(), &word.clone()))) {  for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _lookup(args))(&__args) }) { let __unpk4771 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk4771[0].clone(); let mut weight: Py = __unpk4771[1].clone(); if py_truthy(&{ let __b = op_ge(&weight.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&topic.clone(), &py_str("small-talk"))) } }) {  __out.push(topic.clone()); } } } } py_set(__out) })() };
    let __iter9 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "items", args))(&__args) });
    for __item in __iter9 {
    let __unpk10 = py_unpack(&__item.clone(), 2, None);
    key = __unpk10[0].clone();
    entries = __unpk10[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&key.clone(), &py_str(" "))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&text.clone(), &key.clone())) } }) {
    continue;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&key.clone(), "split", args))(&__args) };
    let __iter11 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter11 {
    start = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&slice_or_raise(&norm.clone(), &start.clone(), &op_add(&start.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None), &words.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&background.clone(), &start.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = sums.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&key.clone(), &py_str("work out"))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entries.clone()) { let __unpk547 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk547[0].clone(); let mut _weight: Py = __unpk547[1].clone(); __out.push(Py::Bool(py_eq(&topic.clone(), &py_str("small-talk")))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    continue;
    }
    let __iter12 = py_iter_or_raise(&entries.clone());
    for __item in __iter12 {
    let __unpk13 = py_unpack(&__item.clone(), 2, None);
    topic = __unpk13[0].clone();
    weight = __unpk13[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(words.clone()); __args.push(norm.clone()); __args.push(start.clone()); __args.push(study.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _says_nothing_here(args))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(weight.clone()); __args.push(start.clone()); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    break;
    }
    }
    text_norm = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text_norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_JUDGEMENT_OF_RECORD(), "search", args))(&__args) };
    if py_truthy(&{ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "values", args))(&__args) }) { let __unpk7666 = py_unpack(&__item.clone(), 3, None); let mut score: Py = __unpk7666[0].clone(); let mut _top: Py = __unpk7666[1].clone(); let mut _first: Py = __unpk7666[2].clone(); __out.push(op_ge(&score.clone(), &Py::Int(3))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("feelings")); __args.push(Py::Int(3)); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text_norm.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "count", args))(&__args) }); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    unfit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text_norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_unfit(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&unfit.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "values", args))(&__args) }) { let __unpk3783 = py_unpack(&__item.clone(), 3, None); let mut score: Py = __unpk3783[0].clone(); let mut _top: Py = __unpk3783[1].clone(); let mut _first: Py = __unpk3783[2].clone(); __out.push(op_ge(&score.clone(), &Py::Int(3))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("feelings")); __args.push(Py::Int(3)); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text_norm.clone(), &Py::None, &unfit.clone(), &Py::None), "count", args))(&__args) }); __args.push(Py::Bool(false)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&note, args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_weather_is_just_a_season(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) }) { let __unpk1911 = py_unpack(&__item.clone(), 2, None); let mut other: Py = __unpk1911[0].clone(); let __unpk1815 = py_unpack(&__unpk1911[1].clone(), 3, None); let mut score: Py = __unpk1815[0].clone(); let mut _top: Py = __unpk1815[1].clone(); let mut _first: Py = __unpk1815[2].clone(); __out.push({ let __b = Py::Bool(!py_eq(&other.clone(), &py_str("nature-and-weather"))); if !py_truthy(&__b) { __b } else { op_ge(&score.clone(), &Py::Int(2)) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nature-and-weather")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&own.clone(), "pop", args))(&__args) };
    }
    let __iter14 = py_iter_or_raise(&g_CANCELS());
    for __item in __iter14 {
    let __unpk15 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk15[0].clone();
    topic = __unpk15[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &topic.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) }) { let __unpk9942 = py_unpack(&__item.clone(), 2, None); let mut other: Py = __unpk9942[0].clone(); let __unpk9846 = py_unpack(&__unpk9942[1].clone(), 3, None); let mut score: Py = __unpk9846[0].clone(); let mut _top: Py = __unpk9846[1].clone(); let mut _first: Py = __unpk9846[2].clone(); __out.push({ let __b = Py::Bool(!py_eq(&other.clone(), &topic.clone())); if !py_truthy(&__b) { __b } else { op_ge(&score.clone(), &Py::Int(2)) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    continue;
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text_norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text_norm.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_CANCEL_AFTER(), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("."), py_str(","), py_str(";"), py_str(":")]), &geti(&after.clone(), &Py::Int(0)))) } } } }) {
    continue;
    }
    let __unpk16 = py_unpack(&geti(&found.clone(), &topic.clone()), 3, None);
    score = __unpk16[0].clone();
    top = __unpk16[1].clone();
    first = __unpk16[2].clone();
    let _ = py_setitem(&found.clone(), &topic.clone(), py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&score.clone(), &Py::Int(2))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, top.clone(), first.clone()]));
    }
    ranked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) }); __kws.extend(vec![("key".to_string(), { let _first = _first.clone(); let _tiers = _tiers.clone(); let _top = _top.clone(); let _topic = _topic.clone(); let _weight = _weight.clone(); let after = after.clone(); let at = at.clone(); let background = background.clone(); let book_at = book_at.clone(); let child_ages = child_ages.clone(); let elsewhere = elsewhere.clone(); let entries = entries.clone(); let family_score = family_score.clone(); let first = first.clone(); let form = form.clone(); let forms = forms.clone(); let forms_of = forms_of.clone(); let found = found.clone(); let idiom = idiom.clone(); let inherited = inherited.clone(); let key = key.clone(); let line = line.clone(); let match_ = match_.clone(); let named = named.clone(); let norm = norm.clone(); let note = note.clone(); let other = other.clone(); let output_at = output_at.clone(); let own = own.clone(); let pattern = pattern.clone(); let position = position.clone(); let ranked = ranked.clone(); let score = score.clone(); let sense_topic = sense_topic.clone(); let spots = spots.clone(); let start = start.clone(); let stone_at = stone_at.clone(); let study = study.clone(); let sums = sums.clone(); let text = text.clone(); let text_norm = text_norm.clone(); let tiers = tiers.clone(); let tight_at = tight_at.clone(); let top = top.clone(); let topic = topic.clone(); let unfit = unfit.clone(); let weight = weight.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let _first = _first.clone(); let _tiers = _tiers.clone(); let _top = _top.clone(); let _topic = _topic.clone(); let _weight = _weight.clone(); let after = after.clone(); let at = at.clone(); let background = background.clone(); let book_at = book_at.clone(); let child_ages = child_ages.clone(); let elsewhere = elsewhere.clone(); let entries = entries.clone(); let family_score = family_score.clone(); let first = first.clone(); let form = form.clone(); let forms = forms.clone(); let forms_of = forms_of.clone(); let found = found.clone(); let idiom = idiom.clone(); let inherited = inherited.clone(); let key = key.clone(); let line = line.clone(); let match_ = match_.clone(); let named = named.clone(); let norm = norm.clone(); let note = note.clone(); let other = other.clone(); let output_at = output_at.clone(); let own = own.clone(); let pattern = pattern.clone(); let position = position.clone(); let ranked = ranked.clone(); let score = score.clone(); let sense_topic = sense_topic.clone(); let spots = spots.clone(); let start = start.clone(); let stone_at = stone_at.clone(); let study = study.clone(); let sums = sums.clone(); let text = text.clone(); let text_norm = text_norm.clone(); let tiers = tiers.clone(); let tight_at = tight_at.clone(); let top = top.clone(); let topic = topic.clone(); let unfit = unfit.clone(); let weight = weight.clone(); let word = word.clone(); let words = words.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); py_tuple(vec![geti(&geti(&item.clone(), &Py::Int(1)), &Py::Int(0)), geti(&geti(&item.clone(), &Py::Int(1)), &Py::Int(1)), op_neg(&geti(&geti(&item.clone(), &Py::Int(1)), &Py::Int(2)))]) }) }), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked.clone()) { let __unpk3702 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk3702[0].clone(); let __unpk3606 = py_unpack(&__unpk3702[1].clone(), 3, None); let mut score: Py = __unpk3606[0].clone(); let mut top: Py = __unpk3606[1].clone(); let mut _first: Py = __unpk3606[2].clone(); __out.push(py_tuple(vec![topic.clone(), if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&own.clone(), "get", args))(&__args) }) { score.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(score.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } }, if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&own.clone(), "get", args))(&__args) }) { top.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(top.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } }])); } py_list(__out) })() };
    Py::None
}
pub fn unsure_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut topic: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topic"))) };
    let mut home: Py = Py::None;
    let mut in_doubt: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words whose second life is still in play, when the line said nothing to settle it.\n\n    \"my tablet has gone missing\" names a thing that is a pill and a screen, and the rest of the\n    line does not say which, so the area the reader picked from it is a guess. \"should i take a\n    tablet with food\" says which in another word (`food` is the pill's life), and \"the freezer is\n    full, how do i stock it\" (`freezer` is the kitchen's). Those lines have said what they are\n    about, and nothing is in doubt.\n\n    The line is asked which of the two lives it named (LIVES), and only that. A word of neither\n    life, however well it scores for the area the line was read as, leaves the doubt standing:\n    the area came from the doubtful word in the first place.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&topic.clone()))) {
    return py_list(vec![]);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    in_doubt = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_TWO_HOMES(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LIVES(), "get", args))(&__args) }) { let mut home: Py = __item.clone(); __out.push(op_bitand(&words.clone(), &home.clone())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&in_doubt.clone(), "append", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(in_doubt.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _place_points(args: &[Py]) -> Py {
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
    let mut entities: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entities"))) };
    let mut scores: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "scores") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: scores"))) };
    let mut asks: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asks") { v.clone() }
    else { Py::None };
    let mut asked: Py = Py::None;
    let mut item: Py = Py::None;
    let mut others: Py = Py::None;
    let mut score: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How much a place the line names says about what the line is about.\n\n    A place is what a line is about when the person is going there or planning something\n    there (\"a weekend in Lisbon\", \"we are going to Rome next month\"), and when they ask what\n    to do there (\"museums in Rome\", \"restaurants in Edinburgh\"). It is only the background of\n    a line about something else: \"how do i get a mortgage in Birmingham\" is a money question,\n    \"i am a bus driver in Leeds\" is a self-introduction, \"my sister lives in Paris\" is news\n    about a sister. So the place scores 4 with a travel word, 3 when the line asks something\n    and nothing else in it scores, and nothing otherwise.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_bitand(&words.clone(), &g_TRAVEL_STRONG())) {
    return Py::Int(4);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entities.clone()) { let mut item: Py = __item.clone(); __out.push(Py::Bool(py_eq(&geti(&item.clone(), &py_str("type")), &py_str("person")))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Int(0);
    }
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scores.clone(), "items", args))(&__args) }) { let __unpk5215 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk5215[0].clone(); let mut score: Py = __unpk5215[1].clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("geography-and-travel"), py_str("small-talk")]), &topic.clone()))) {  __out.push(score.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = others.clone(); if !py_truthy(&__b) { __b } else { op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(others.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_PLACE_OVER()) } }) {
    return Py::Int(0);
    }
    asked = if py_truthy(&Py::Bool(py_is(&asks.clone(), &Py::None))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _looks_like_an_ask(args))(&__args) } } else { asks.clone() };
    return if py_truthy(&asked.clone()) { Py::Int(3) } else { Py::Int(0) };
    Py::None
}
pub fn _what_someone_else_wants(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut spot: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut topics: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(topic, position) for the thing a person in the line wants, or None.\n\n    \"my wife wants a takeaway\" -> (\"cooking-and-food\", 4), \"my son wants a dog\" -> pets. The\n    person has to be the one wanting - the person word sits just before the verb - so \"i need\n    some breathing room from my husband\" is not read as something the husband wants, and the\n    thing wanted has to be written as a thing (\"a takeaway\", \"some new bedding\"), not abstractly\n    or after a preposition: \"school wants 20 pounds for a trip\" is a money line about a trip.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_WANT_VERBS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&g_A_PERSON_WHO_WANTS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&at.clone(), &Py::Int(3))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &at.clone(), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&at.clone(), &Py::Int(1))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(op_add(&at.clone(), &Py::Int(5))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    spot = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_WANTED_BEFORE_A_STOP(), &geti(&norm.clone(), &spot.clone())))) {
    break;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_A_THING_WANTED(), &geti(&norm.clone(), &spot.clone())))) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&slice_or_raise(&norm.clone(), &op_add(&spot.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(op_add(&spot.clone(), &Py::Int(5))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None));
    for __item in __iter4 {
    candidate = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_A_THING_WANTED(), &candidate.clone()))) {
    continue;
    }
    topics = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(candidate.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _lookup(args))(&__args) }) { let __unpk5152 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk5152[0].clone(); let mut weight: Py = __unpk5152[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_PEOPLE_TOPICS(), &topic.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&topic.clone(), &py_str("small-talk"))); if !py_truthy(&__b) { __b } else { op_ge(&weight.clone(), &Py::Int(2)) } } } }) {  __out.push(topic.clone()); } } py_list(__out) })() };
    if py_truthy(&topics.clone()) {
    return py_tuple(vec![geti(&topics.clone(), &Py::Int(0)), op_add(&spot.clone(), &Py::Int(1))]);
    }
    }
    break;
    }
    }
    return Py::None;
    Py::None
}
pub fn _looks_like_an_ask(args: &[Py]) -> Py {
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
    let mut norm: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Does the line ask for something? A question mark, a question word, or an imperative.");
    text = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    norm = geti(&line.clone(), &py_str("norm"));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&norm.clone())) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&g_ASK_WORDS(), &geti(&norm.clone(), &Py::Int(0))));
    Py::None
}
pub fn _names_a_friend_of_their_own(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut before: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says \"my mate\", \"our mates\", \"your bloke\": a friend they claim.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(slice_or_raise(&norm.clone(), &Py::Int(1), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("zip", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk1181 = py_unpack(&__item.clone(), 2, None); let mut before: Py = __unpk1181[0].clone(); let mut word: Py = __unpk1181[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_FRIEND_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNED_BY(), &before.clone())) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_persons_own_output(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut person: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Where the line talks about what comes out of the person, or None.\n\n    \"my poo has been green for a week\" named no part of the body, so the only words left were\n    \"green\" and \"a week\" and it was answered with the salad-drawer advice. The words for it are\n    also the words for what an animal does and for a job to be done - \"how do i get bird poo off\n    my car\" is the car - so the line has to make it the person's own: said of them (\"i have not\n    pooed in 3 days\", \"my son hasnt pooed for two days\") or owned by them (\"my poo\", \"her\n    stools\"). A sample or a test is a thing being handed in, not a symptom.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_BODILY_OUTPUT()))); if py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_SAMPLE_WORDS()) } }) {
    return Py::None;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_ONLY_A_STOOL()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &op_sub(&g_BODILY_OUTPUT(), &g_ONLY_A_STOOL())))) } }) {
    if py_truthy(&op_bitand(&words.clone(), &g_STOOL_AS_FURNITURE())) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::understanding::body::g_A_GUT_ONLY(), "search", args))(&__args) }))) {
    return Py::None;
    }
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_ANIMALS())) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_PETS())) {
    return Py::None;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    person = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&words.clone(), &op_sub(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _person_words(args))(&__args) }, &g_PETS()), &g_ANIMALS()))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_BODILY_OUTPUT(), &word.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_BODILY_OUTPUT_NOUN(), &word.clone()))) {
    if py_truthy(&{ let __b = at.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNERS(), &geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(1))))) } }) {
    return at.clone();
    }
    continue;
    }
    if py_truthy(&{ let __b = person.clone(); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("i im ive we weve"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    return at.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn own_output_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let _ = py_str("The word for what comes out of the person, when the line makes it theirs, or None.\n\n    The reading carries it as `own_output` so a knowledge row that shares the word with a thing\n    - a stool is a chair, green leaves keep a week in the drawer - can stand aside for the body.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_persons_own_output(args))(&__args) };
    Py::None
}
pub fn _owns_an_animal(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names an animal somebody owns (\"my fish\", \"our rabbit\").");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk14 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk14[0].clone(); let mut word: Py = __unpk14[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_OWNERS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ANIMALS(), &geti(&norm.clone(), &op_add(&at.clone(), &Py::Int(1))))) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _a_crop_is_failing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut state: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line says a crop of the person's own is not doing well where it grows.\n\n    The state has to be one a plant is in rather than a dish: \"my tomatoes are not ripening\" is\n    the plant, \"my tomatoes are in the salad\" is dinner. A crop kept in a kitchen, or one the\n    line cooks, is dinner whatever else it says, so the kitchen words hold the fence off.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_bitand(&words.clone(), &g_CROP_KITCHEN())) {
    return Py::Bool(false);
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CROP_STATES()) { let mut state: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &state.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&words.clone(), &g_GROWING_PLACES())) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OWNERS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CROPS(), &geti(&norm.clone(), &op_add(&at.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asks_mechanism(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let _ = py_str("True when the line asks how the world works (\"why do...\", \"how old is...\").");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_MECHANISM(), &geti(&norm.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&norm.clone(), &Py::Int(0)), &py_str("how")))) {
    return { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MECHANISM_HOW(), &geti(&norm.clone(), &Py::Int(1)))) } };
    }
    return Py::Bool(true);
    Py::None
}
pub fn _sign_said_of_a_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut after: Py = Py::None;
    let mut ahead: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a sign word in the line is the state of the person it is said of.\n\n    A sign only counts with a person in the line to have it, and then only in the shapes that\n    make it the person's state rather than a modifier, a behaviour or an idiom.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_UNWELL_IDIOMS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    after = if py_truthy(&op_lt(&op_add(&at.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&norm.clone(), &op_add(&at.clone(), &Py::Int(1))) } else { py_str("") };
    before = if py_truthy(&at.clone()) { geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_UNWELL_SIGNS(), &word.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BEHAVIOUR(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SIGN_NOT_AFTER(), &after.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOUNY_SIGNS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SIGN_FOLLOWS(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&after.clone(), "isdigit", args))(&__args) })) } } } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("broke"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_BODY()))) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&before.clone(), &py_str("not"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_UNWELL_NOT(), &word.clone())) } }) {
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("responding")))) {
    return Py::Bool(true);
    }
    let __iter3 = py_iter_or_raise(&slice_or_raise(&norm.clone(), &op_add(&at.clone(), &Py::Int(1)), &op_add(&at.clone(), &Py::Int(4)), &Py::None));
    for __item in __iter3 {
    ahead = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_COMMS(), &ahead.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_HEARERS(), &ahead.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_person_is_unwell(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    let _ = py_str("A line that names a person and says something is wrong with them (\"my baby has a fever\").");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_UNWELL_SIGNS()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &py_str("not"))); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_UNWELL_NOT()) } })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_THE_PEOPLE(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sign_said_of_a_person(args))(&__args) };
    Py::None
}
pub fn broke_at(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the line says the speaker has no money with `broke`, or None.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("broke")))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &Py::None, &at.clone(), &Py::None)) { let mut before: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_BROKE_AS_SUBJECT(), &before.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut after: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_BROKE_AS_TIME(), &after.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    return at.clone();
    }
    return Py::None;
    Py::None
}
pub fn _purse_amount(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let mut amount: Py = Py::None;
    let mut before: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("How much the line says is left, in pounds, or None when it does not say.");
    amount = geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(1)));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&amount.clone(), "replace", args))(&__args) }, "isdigit", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD_NUMBERS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("hundred"), py_str("thousand")]), &amount.clone())); if !py_truthy(&__b) { __b } else { op_ge(&at.clone(), &Py::Int(2)) } }) {
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(2)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD_NUMBERS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&before.clone(), &Py::None))) {
    return op_mul(&before.clone(), &value.clone());
    }
    }
    return value.clone();
    Py::None
}
pub fn _a_wait_follows(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut spot: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the words after an amount say when the money comes.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&words.clone(), &Py::None, &Py::Int(3), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    spot = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_A_WAIT(), &word.clone()))) {
    continue;
    }
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&spot.clone(), &Py::Int(1)), &op_add(&spot.clone(), &Py::Int(3)), &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("i")]), &one.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_WAIT_TO(), &geti(&rest.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn purse_till_at(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut after: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut at: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pence: Py = Py::None;
    let mut pounds: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the line says what is left and when the money comes, or None.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&at.clone(), &Py::Int(0)))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_A_AMOUNT_NOUN(), &word.clone()))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_wait_follows(args))(&__args) }) {
    return op_sub(&at.clone(), &Py::Int(1));
    }
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_A_MONEY_WORD(), &word.clone()))) {
    continue;
    }
    amount = geti(&norm.clone(), &op_sub(&at.clone(), &Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NUMBER_WORDS(), &amount.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&amount.clone(), "replace", args))(&__args) }, "isdigit", args))(&__args) })) } }) {
    continue;
    }
    if py_truthy(&op_ge(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(at.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _purse_amount(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Int(0) } }, &g_BIG_PURSE())) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_wait_follows(args))(&__args) }) {
    return op_sub(&at.clone(), &Py::Int(1));
    }
    }
    if py_truthy(&text.clone()) {
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_AMOUNT_SYMBOL(), "finditer", args))(&__args) });
    for __item in __iter3 {
    match_ = __item.clone();
    let __unpk4 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }]), 2, None);
    pounds = __unpk4[0].clone();
    pence = __unpk4[1].clone();
    value = if py_truthy(&pounds.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pounds.clone(), "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { op_truediv(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Float(100.0)) };
    if py_truthy(&op_ge(&value.clone(), &g_BIG_PURSE())) {
    continue;
    }
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_wait_follows(args))(&__args) }) {
    return Py::Int(0);
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn tight_money_at(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut at: Py = Py::None;
    let mut index: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut purse: Py = Py::None;
    let mut spot: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the line says the money has run out, or None: \"im skint till friday\", \"a bit short\".\n\n    The phrase has to be the whole of what the line names. \"im vegetarian and im skint, cheap\n    dinners\" asks for dinners and is answered as the cooking ask it is, so naming another\n    subject at all leaves the tight week unsaid.\n    ");
    joined = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
    at = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| broke_at(args))(&__args) };
    purse = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| purse_till_at(args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&purse.clone(), &Py::None))) {
    at = if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) { purse.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(at.clone()); __args.push(purse.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    }
    let __iter1 = py_iter_or_raise(&g_MONEY_TIGHT());
    for __item in __iter1 {
    phrase = __item.clone();
    spot = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&joined.clone(), "find", args))(&__args) };
    if py_truthy(&op_lt(&spot.clone(), &Py::Int(0))) {
    continue;
    }
    index = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&joined.clone(), &Py::None, &spot.clone(), &Py::None), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    at = if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) { index.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(at.clone()); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    }
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&norm.clone());
    for __item in __iter2 {
    word = __item.clone();
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _lookup(args))(&__args) });
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    topic = __unpk4[0].clone();
    weight = __unpk4[1].clone();
    if py_truthy(&{ let __b = op_ge(&weight.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("money"), py_str("small-talk")]), &topic.clone())) } }) {
    return Py::None;
    }
    }
    }
    return at.clone();
    Py::None
}
pub fn stone_weight_at(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut cues: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut spot: Py = Py::None;
    let mut spots: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the line weighs something in stones, or None.\n\n    Only the verb makes it a weight, and it has to be near the stone (\"i want to lose a stone\",\n    \"half a stone to shift before the wedding\"), so a line about a wall or a fruit stone keeps\n    its own area.\n    ");
    spots = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk8500 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk8500[0].clone(); let mut word: Py = __unpk8500[1].clone(); if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("stone"), py_str("stones")]), &word.clone()))) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&spots.clone()))) {
    return Py::None;
    }
    joined = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    cues = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk9988 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk9988[0].clone(); let mut word: Py = __unpk9988[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_STONE_VERBS(), &word.clone()))) {  __out.push(at.clone()); } } py_set(__out) })() };
    let __iter1 = py_iter_or_raise(&g_STONE_COMPOUNDS());
    for __item in __iter1 {
    phrase = __item.clone();
    start = Py::Int(0);
    while py_truthy(&Py::Bool(true)) {
    spot = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); __args.push(start.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&joined.clone(), "find", args))(&__args) };
    if py_truthy(&op_lt(&spot.clone(), &Py::Int(0))) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&joined.clone(), &Py::None, &spot.clone(), &Py::None), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&cues.clone(), "add", args))(&__args) };
    start = op_add(&spot.clone(), &Py::Int(1));
    }
    }
    let __iter2 = py_iter_or_raise(&spots.clone());
    for __item in __iter2 {
    at = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&cues.clone()) { let mut cue: Py = __item.clone(); __out.push(op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&at.clone(), &cue.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return at.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn tight_on_money(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { Py::None };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let _ = py_str("True when the line says the money has run out and is about nothing else.\n\n    A question is left alone - \"what can i cook till payday\" is a cooking ask that happens to\n    mention payday - and so is a line long enough to be about something else.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("question"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&norm.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_MONEY_TIGHT_MAX()) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| tight_money_at(args))(&__args) }, &Py::None));
    Py::None
}
pub fn _worn_out(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut at: Py = Py::None;
    let mut back: Py = Py::None;
    let mut item: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut spot: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says the person is worn out, in the words people use for it.\n\n    The cue is the person: \"im shattered\" is the tiredness and \"the shed window is shattered\"\n    is not, so the word has to come within three words of \"im\", \"am\" or \"feel\". \"Wiped\", \"done\"\n    and \"worn\" only count inside their phrase, because \"im done with that\" is not tiredness.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&norm.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WORN_OUT(), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&norm.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&at.clone(), &g_WORN_OUT_LOOKBACK())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &at.clone(), &Py::None)) { let mut item: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_WORN_OUT_CUE(), &item.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    joined = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "));
    let __iter3 = py_iter_or_raise(&g_WORN_OUT_SAID());
    for __item in __iter3 {
    phrase = __item.clone();
    spot = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &phrase.clone()), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&joined.clone(), "find", args))(&__args) };
    if py_truthy(&op_lt(&spot.clone(), &Py::Int(0))) {
    continue;
    }
    back = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&joined.clone(), &Py::None, &spot.clone(), &Py::None), "split", args))(&__args) }, &op_neg(&g_WORN_OUT_LOOKBACK()), &Py::None, &Py::None);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&back.clone()) { let mut item: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_WORN_OUT_CUE(), &item.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn rank_topics(args: &[Py]) -> Py {
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
    let mut inherited: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "inherited") { v.clone() }
    else { Py::None };
    let mut entities: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { Py::None };
    let mut asks: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asks") { v.clone() }
    else { Py::None };
    let mut _top: Py = Py::None;
    let mut animals: Py = Py::None;
    let mut at: Py = Py::None;
    let mut boost: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut found: Py = Py::None;
    let mut item: Py = Py::None;
    let mut number: Py = Py::None;
    let mut other: Py = Py::None;
    let mut points: Py = Py::None;
    let mut position: Py = Py::None;
    let mut ranked: Py = Py::None;
    let mut reason: Py = Py::None;
    let mut score: Py = Py::None;
    let mut scores: Py = Py::None;
    let mut sign_words: Py = Py::None;
    let mut top: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut warmth: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("[(topic, score)] for this line, best first, with what the line names taken into\n    account: a place plus a travel cue, a pet that is the subject, a percentage.");
    found = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| score_topics(args))(&__args) }) { let __unpk5511 = py_unpack(&__item.clone(), 3, None); let mut topic: Py = __unpk5511[0].clone(); let mut score: Py = __unpk5511[1].clone(); let mut top: Py = __unpk5511[2].clone(); __out.push((topic.clone(), py_list(vec![score.clone(), top.clone()]))); } py_dict(__out) })() };
pub fn __nested_boost_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _top: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut animals: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut asks: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut at: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut boost: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut entities: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut entry: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut found: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut inherited: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut number: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut other: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut points: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut position: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut ranked: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut reason: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut score: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut scores: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut sign_words: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut top: Py = __pos.get(20).cloned().unwrap_or(Py::None);
    let mut topic: Py = __pos.get(21).cloned().unwrap_or(Py::None);
    let mut wanted: Py = __pos.get(22).cloned().unwrap_or(Py::None);
    let mut warmth: Py = __pos.get(23).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(24).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(25).cloned().unwrap_or(Py::None);
    let mut topic: Py = __pos.get(26).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "topic").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut points: Py = __pos.get(27).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "points").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    entry = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_list(vec![Py::Int(0), Py::Int(0)])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "setdefault", args))(&__args) };
    let __aug1 = op_add(&geti(&entry.clone(), &Py::Int(0)), &points.clone());
    let _ = py_setitem(&entry.clone(), &Py::Int(0), __aug1);
    let _ = py_setitem(&entry.clone(), &Py::Int(1), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&entry.clone(), &Py::Int(1))); __args.push(points.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    Py::None
}
    boost = {{ let _top = _top.clone(); let animals = animals.clone(); let asks = asks.clone(); let at = at.clone(); let boost = boost.clone(); let entities = entities.clone(); let entry = entry.clone(); let found = found.clone(); let inherited = inherited.clone(); let item = item.clone(); let line = line.clone(); let number = number.clone(); let other = other.clone(); let points = points.clone(); let position = position.clone(); let ranked = ranked.clone(); let reason = reason.clone(); let score = score.clone(); let scores = scores.clone(); let sign_words = sign_words.clone(); let top = top.clone(); let topic = topic.clone(); let wanted = wanted.clone(); let warmth = warmth.clone(); let word = word.clone(); let words = words.clone(); py_func("boost", move |args: &[Py]| { let mut full = vec![_top.clone(), animals.clone(), asks.clone(), at.clone(), boost.clone(), entities.clone(), entry.clone(), found.clone(), inherited.clone(), item.clone(), line.clone(), number.clone(), other.clone(), points.clone(), position.clone(), ranked.clone(), reason.clone(), score.clone(), scores.clone(), sign_words.clone(), top.clone(), topic.clone(), wanted.clone(), warmth.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_boost_0(&full) }) }};
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = entities.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entities.clone()) { let mut item: Py = __item.clone(); __out.push(Py::Bool(py_eq(&geti(&item.clone(), &py_str("type")), &py_str("place")))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    points = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(entities.clone()); __args.push({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) }) { let __unpk2247 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk2247[0].clone(); let __unpk2151 = py_unpack(&__unpk2247[1].clone(), 2, None); let mut score: Py = __unpk2151[0].clone(); let mut _top: Py = __unpk2151[1].clone(); __out.push((topic.clone(), score.clone())); } py_dict(__out) })() }); __args.push(asks.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _place_points(args))(&__args) };
    if py_truthy(&points.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("geography-and-travel")); __args.push(points.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = entities.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entities.clone()) { let mut item: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(item.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_country_named(args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("geography-and-travel")); __args.push(g_COUNTRY_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let __b = entities.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entities.clone()) { let mut item: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_PETS(), &geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&item.clone(), &py_str("text")), "lower", args))(&__args) }, "split", args))(&__args) }, &op_neg(&Py::Int(1)))))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pets")); __args.push(g_PET_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    reason = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::normalize::reason_positions(args))(&__args) };
    if py_truthy(&{ let __b = op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk8070 = py_unpack(&__item.clone(), 2, None); let mut position: Py = __unpk8070[0].clone(); let mut word: Py = __unpk8070[1].clone(); if py_truthy(&Py::Bool(!py_contains(&reason.clone(), &position.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_BODY()); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk7158 = py_unpack(&__item.clone(), 2, None); let mut position: Py = __unpk7158[0].clone(); let mut word: Py = __unpk7158[1].clone(); if py_truthy(&Py::Bool(!py_contains(&reason.clone(), &position.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_HURTS()) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(g_HURT_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _head_fog(args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(g_HEAD_FOG_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(reason.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _skin_state(args))(&__args) }, &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PEOPLE_TOPICS()) { let mut topic: Py = __item.clone(); __out.push(op_ge(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_list(vec![Py::Int(0)])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) }, &Py::Int(0)), &Py::Int(2))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(g_SKIN_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(words.clone()); __args.push(reason.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named_condition(args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(g_CONDITION_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(reason.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_part_plays_up(args))(&__args) }, &Py::None))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(g_PART_UP_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    warmth = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_warmth_said(args))(&__args) };
    sign_words = if py_truthy(&warmth.clone()) { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&line.clone(), &py_str("norm"))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&word.clone(), &warmth.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() } } else { geti(&line.clone(), &py_str("norm")) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sign_words.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sign_words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_person_is_unwell(args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(g_UNWELL_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOSE_BLOCKED(), "search", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(g_NOSE_BLOCKED_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    animals = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _owns_an_animal(args))(&__args) };
    if py_truthy(&animals.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pets")); __args.push(g_PET_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_friend_of_their_own(args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("friends-and-relationships")); __args.push(g_FRIEND_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _what_someone_else_wants(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&wanted.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PEOPLE_TOPICS()) { let mut topic: Py = __item.clone(); __out.push(op_ge(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_list(vec![Py::Int(0)])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) }, &Py::Int(0)), &Py::Int(2))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let __unpk2 = py_unpack(&wanted.clone(), 2, None);
    topic = __unpk2[0].clone();
    at = __unpk2[1].clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_list(vec![Py::Int(0)])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) }, &Py::Int(0))); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_crop_is_failing(args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("garden-and-plants")); __args.push(g_CROP_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_habit_given_up(args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("food-and-drink")); __args.push(g_GIVING_UP_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _worn_out(args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::_asks_back(args))(&__args) })) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sleep")); __args.push(g_WORN_OUT_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_WORLD()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_WORLD_NOT()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_NATURE_WORDS()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g_HURTS()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&animals.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_mechanism(args))(&__args) } } } } } } } } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("science")); __args.push(g_SCIENCE_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| detect_numbers(args))(&__args) });
    for __item in __iter3 {
    number = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&number.clone(), &py_str("unit")), &py_str("percent")))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("maths-and-numbers")); __args.push(Py::Int(3)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_ask(args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("shopping")); __args.push(g_GIFT_BOOST()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    if py_truthy(&{ let __b = op_ge(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("health")); __args.push(py_list(vec![Py::Int(0)])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "get", args))(&__args) }, &Py::Int(0)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { op_le(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) }) { let __unpk2861 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk2861[0].clone(); let mut entry: Py = __unpk2861[1].clone(); if py_truthy(&op_ge(&geti(&entry.clone(), &Py::Int(0)), &Py::Int(2))) {  __out.push(topic.clone()); } } py_set(__out) })() }, &py_set(vec![py_str("health"), py_str("small-talk")])) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("small-talk")); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "pop", args))(&__args) };
    }
    if py_truthy(&inherited.clone()) {
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) });
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    topic = __unpk5[0].clone();
    entry = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&topic.clone(), &inherited.clone())); if !py_truthy(&__b) { __b } else { op_lt(&geti(&entry.clone(), &Py::Int(1)), &Py::Int(2)) } }) {
    let __aug6 = op_sub(&geti(&entry.clone(), &Py::Int(0)), &Py::Int(1));
    let _ = py_setitem(&entry.clone(), &Py::Int(0), __aug6);
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(inherited.clone()); __args.push(g_CARRY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&boost, args))(&__args) };
    }
    scores = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "items", args))(&__args) }) { let __unpk5693 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk5693[0].clone(); let __unpk5789 = py_unpack(&__unpk5693[1].clone(), 2, None); let mut score: Py = __unpk5789[0].clone(); let mut _top: Py = __unpk5789[1].clone(); __out.push((topic.clone(), score.clone())); } py_dict(__out) })() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(op_add(&op_add(&py_str(" "), &geti(&line.clone(), &py_str("norm_text"))), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_weather_is_just_a_season(args))(&__args) }) {
    entry = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nature-and-weather")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scores.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&entry.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scores.clone(), "items", args))(&__args) }) { let __unpk1242 = py_unpack(&__item.clone(), 2, None); let mut other: Py = __unpk1242[0].clone(); let mut score: Py = __unpk1242[1].clone(); __out.push({ let __b = Py::Bool(!py_eq(&other.clone(), &py_str("nature-and-weather"))); if !py_truthy(&__b) { __b } else { op_ge(&score.clone(), &Py::Int(2)) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let _ = py_setitem(&scores.clone(), &py_str("nature-and-weather"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entry.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    }
    ranked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scores.clone(), "items", args))(&__args) }) { let __unpk2298 = py_unpack(&__item.clone(), 2, None); let mut topic: Py = __unpk2298[0].clone(); let mut score: Py = __unpk2298[1].clone(); if py_truthy(&op_ge(&score.clone(), &Py::Int(2))) {  __out.push(py_tuple(vec![topic.clone(), score.clone()])); } } py_list(__out) })() };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let _top = _top.clone(); let animals = animals.clone(); let asks = asks.clone(); let at = at.clone(); let boost = boost.clone(); let entities = entities.clone(); let entry = entry.clone(); let found = found.clone(); let inherited = inherited.clone(); let item = item.clone(); let line = line.clone(); let number = number.clone(); let other = other.clone(); let points = points.clone(); let position = position.clone(); let ranked = ranked.clone(); let reason = reason.clone(); let score = score.clone(); let scores = scores.clone(); let sign_words = sign_words.clone(); let top = top.clone(); let topic = topic.clone(); let wanted = wanted.clone(); let warmth = warmth.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let _top = _top.clone(); let animals = animals.clone(); let asks = asks.clone(); let at = at.clone(); let boost = boost.clone(); let entities = entities.clone(); let entry = entry.clone(); let found = found.clone(); let inherited = inherited.clone(); let item = item.clone(); let line = line.clone(); let number = number.clone(); let other = other.clone(); let points = points.clone(); let position = position.clone(); let ranked = ranked.clone(); let reason = reason.clone(); let score = score.clone(); let scores = scores.clone(); let sign_words = sign_words.clone(); let top = top.clone(); let topic = topic.clone(); let wanted = wanted.clone(); let warmth = warmth.clone(); let word = word.clone(); let words = words.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); py_tuple(vec![geti(&item.clone(), &Py::Int(1)), Py::Bool(py_eq(&geti(&item.clone(), &Py::Int(0)), &inherited.clone()))]) }) }), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ranked.clone(), "sort", args))(&__args) };
    return ranked.clone();
    Py::None
}
pub fn gift_ask(args: &[Py]) -> Py {
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
    let mut norm: Py = Py::None;
    let _ = py_str("True when this reading asks what to give someone. Public: the answer entry asks too.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    norm = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("norm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&norm.clone())) } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_ask(args))(&__args) };
    Py::None
}
pub fn _gift_ask(args: &[Py]) -> Py {
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
    let mut norm: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut ch: Py = Py::None;
    let mut head: Py = Py::None;
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut position: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks what to give someone.\n\n    A gift word needs one other sign of a gift (\"a present for\", \"an anniversary gift\"), and\n    a plain \"get\" needs a person to give to and then nothing that turns it into another\n    errand: \"get her\" is a present, \"get her to eat\", \"get her a taxi\", \"get him tested\" and\n    \"get him for his cough\" are not. What follows the person decides - the end of the line, or\n    \"something\", a present word, or an occasion (\"for christmas\", \"for her birthday\").\n    ");
    if py_truthy(&op_bitand(&words.clone(), &g_GIFT_ALWAYS())) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_GIFT_WORDS()); if !py_truthy(&__b) { __b } else { { let __b = op_bitand(&words.clone(), &g_GIFT_ASKING()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&words.clone(), &g_GIVE_VERB()); if py_truthy(&__b) { __b } else { { let __b = op_bitand(&words.clone(), &g_GIFT_OCCASION()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("for"))) } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_GIFT_WANTING()); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_still_to_get(args))(&__args) } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_bitand(&words.clone(), &g_GIFT_ASKING()); if py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_GIFT_OCCASION()) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&words.clone(), &g_GIFT_OCCASION())) {
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    position = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GIVE_TO(), &word.clone())); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &op_add(&position.clone(), &Py::Int(3)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_GIVE_THINGS()) } }) {
    return Py::Bool(true);
    }
    }
    }
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    position = __unpk4[0].clone();
    word = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_GIVE_VERB(), &word.clone()))) {
    continue;
    }
    rest = slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    continue;
    }
    head = geti(&rest.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GIVE_THING(), &head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_GIFT_WORDS(), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GIFT_OCCASION(), &head.clone())) } } } }) {
    return Py::Bool(true);
    }
    tail = rest.clone();
    let __iter5 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&rest.clone(), &Py::None, &Py::Int(2), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let mut __else6 = false;
    for __item in __iter5 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    index = __unpk7[0].clone();
    one = __unpk7[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GIVE_TO(), &one.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_GIVE_DETERMINERS(), &one.clone())); if !py_truthy(&__b) { __b } else { { let __b = slice_or_raise(&rest.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(2)), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GIVE_TO(), &geti(&rest.clone(), &op_add(&index.clone(), &Py::Int(1))))) } } } } } }) {
    tail = if py_truthy(&Py::Bool(py_contains(&g_GIVE_TO(), &one.clone()))) { slice_or_raise(&rest.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None) } else { slice_or_raise(&rest.clone(), &op_add(&index.clone(), &Py::Int(2)), &Py::None, &Py::None) };
    break;
    }
    }
    continue;
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&tail.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GIVE_RELATIVE(), &geti(&tail.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&tail.clone(), &Py::None, &Py::Int(3), &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_GIFTS_OR_OCCASIONS(), &one.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&tail.clone(), &Py::Int(0)), &py_str("for"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&tail.clone(), &Py::Int(1), &Py::Int(4), &Py::None)) { let mut one: Py = __item.clone(); for __item in py_iter_or_raise(&one.clone()) { let mut ch: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ch.clone(), "isdigit", args))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _gift_still_to_get(args: &[Py]) -> Py {
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
    let mut norm: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut position: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line says a present is still not got: the person and the thing, no question.\n\n    \"i still havent got her anything\", \"still need to get my mum something\", \"i havent bought\n    my sister her present yet\", \"nothing for him yet\". The giving verb is the same one a gift\n    question uses, so the shape is the same - a person to give to and then the thing given -\n    and what makes it a gift line rather than an errand is that the thing is still owed: a\n    want word somewhere in the line, and \"anything\" or \"nothing\" as the thing.\n    \"get her a taxi\", \"get him tested\" and \"get the kids to bed\" name no thing given.\n    ");
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_GIVE_TO()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&words.clone(), &py_str("for"))); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &py_set(vec![py_str("nothing"), py_str("anything")])) } } } }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    position = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_GIVE_VERB(), &word.clone()))) {
    continue;
    }
    rest = slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &Py::None, &Py::None);
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&rest.clone(), &Py::None, &Py::Int(2), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let mut __else4 = false;
    for __item in __iter3 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    index = __unpk5[0].clone();
    one = __unpk5[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_GIVE_TO(), &one.clone()))) {
    tail = slice_or_raise(&rest.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None);
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GIVE_DETERMINERS(), &one.clone())); if !py_truthy(&__b) { __b } else { { let __b = slice_or_raise(&rest.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(2)), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GIVE_TO(), &geti(&rest.clone(), &op_add(&index.clone(), &Py::Int(1))))) } } } }) {
    tail = slice_or_raise(&rest.clone(), &op_add(&index.clone(), &Py::Int(2)), &Py::None, &Py::None);
    break;
    }
    }
    continue;
    if py_truthy(&Py::Bool(!py_truthy(&tail.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GIVE_THING(), &geti(&tail.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GIFT_WORDS(), &geti(&tail.clone(), &Py::Int(0)))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&tail.clone(), &py_str("for"))); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &py_set(vec![py_str("nothing"), py_str("anything")])) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&tail.clone(), &Py::None, &Py::Int(3), &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_GIFT_OCCASION(), &one.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn subject_topic(args: &[Py]) -> Py {
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
    let mut entities: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { Py::None };
    let mut _score: Py = Py::None;
    let mut top: Py = Py::None;
    let mut topic: Py = Py::None;
    let _ = py_str("The topic this line names with a distinctive word or a phrase, or None.\n\n    Ordinary words alone (\"year\", \"old\", \"think\") do not make a subject: a line built from\n    them is a follow-up on whatever the conversation is already about, so this is what a\n    caller asks before treating the line as a new subject.\n    ");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| score_topics(args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    topic = __unpk2[0].clone();
    _score = __unpk2[1].clone();
    top = __unpk2[2].clone();
    if py_truthy(&op_ge(&top.clone(), &Py::Int(2))) {
    return topic.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn the_words_that_said(args: &[Py]) -> Py {
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
    let mut topic: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "topic") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: topic"))) };
    let mut entities: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { Py::None };
    let mut covered: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut name: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut position: Py = Py::None;
    let mut size: Py = Py::None;
    let mut start: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("[(word, weight)] for what in the line points at `topic`, strongest first.\n\n    A phrase counts as the phrase (`hot dog`, `work out`) and not as its words, because the\n    phrase is what says which sense a word is being used in: \"hot dog\" is a food, while `dog`\n    on its own is the animal. Understanding publishes this so a reader can see how much of the\n    sentence backs a topic before it answers from it (`understood[\"topic_from\"]`).\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(topic.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&topic.clone())) } }) {
    return py_list(vec![]);
    }
    norm = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("norm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 2, None);
    found = __unpk1[0].clone();
    covered = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); __args.push(Py::Int(5)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    size = __item.clone();
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &size.clone()), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    start = __item.clone();
    key = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &start.clone(), &op_add(&start.clone(), &size.clone()), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TABLE(), "get", args))(&__args) });
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    name = __unpk5[0].clone();
    weight = __unpk5[1].clone();
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &topic.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![key.clone(), weight.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &start.clone(), &op_add(&start.clone(), &size.clone()), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&covered.clone(), "update", args))(&__args) };
    }
    }
    }
    }
    let __iter6 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    position = __unpk7[0].clone();
    word = __unpk7[1].clone();
    if py_truthy(&Py::Bool(py_contains(&covered.clone(), &word.clone()))) {
    continue;
    }
    let __iter8 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _lookup(args))(&__args) });
    for __item in __iter8 {
    let __unpk9 = py_unpack(&__item.clone(), 2, None);
    name = __unpk9[0].clone();
    weight = __unpk9[1].clone();
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &topic.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![word.clone(), weight.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "append", args))(&__args) };
    }
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let covered = covered.clone(); let entities = entities.clone(); let found = found.clone(); let key = key.clone(); let line = line.clone(); let name = name.clone(); let norm = norm.clone(); let position = position.clone(); let size = size.clone(); let start = start.clone(); let topic = topic.clone(); let weight = weight.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let covered = covered.clone(); let entities = entities.clone(); let found = found.clone(); let key = key.clone(); let line = line.clone(); let name = name.clone(); let norm = norm.clone(); let position = position.clone(); let size = size.clone(); let start = start.clone(); let topic = topic.clone(); let weight = weight.clone(); let word = word.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); py_tuple(vec![op_neg(&geti(&item.clone(), &Py::Int(1))), geti(&item.clone(), &Py::Int(0))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "sort", args))(&__args) };
    return found.clone();
    Py::None
}
pub fn detect_topic(args: &[Py]) -> Py {
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
    let mut inherited: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "inherited") { v.clone() }
    else { Py::None };
    let mut entities: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { Py::None };
    let mut asks: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asks") { v.clone() }
    else { Py::None };
    let mut ranked: Py = Py::None;
    let _ = py_str("The topic the line is about (or inherited when it adds nothing), plus its score.");
    ranked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(inherited.clone()); __args.push(entities.clone()); __args.push(asks.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| rank_topics(args))(&__args) };
    if py_truthy(&ranked.clone()) {
    return geti(&ranked.clone(), &Py::Int(0));
    }
    return if py_truthy(&inherited.clone()) { py_tuple(vec![inherited.clone(), Py::Int(0)]) } else { py_tuple(vec![Py::None, Py::Int(0)]) };
    Py::None
}
pub fn _is_name(args: &[Py]) -> Py {
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
    let mut lower: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("A capitalised word is a name when it is not just an ordinary word in capitals.\n\n    \"She's 12.\" and \"It's about the booking system\" only start with a capital because they\n    start the sentence: an ordinary word that happens to be written short (\"she's\", \"don't\",\n    \"it's\") is no more a name than \"the\" is.\n    ");
    lower = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::understanding::normalize::g_LEXICON(), &lower.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&lower.clone(), &py_str("i"))) } }) {
    return Py::Bool(false);
    }
    stem = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lower.clone(), "split", args))(&__args) }, &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&stem.clone(), &lower.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::understanding::lexicon::g_CORE_WORDS(), &stem.clone())); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&stem.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::understanding::lexicon::g_CORE_WORDS(), &slice_or_raise(&stem.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } } } } } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(!py_contains(&crate::understanding::lexicon::g_CHAT(), &lower.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&lower.clone(), &py_str("i"))) } };
    Py::None
}
pub fn _capitalised(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut current: Py = Py::None;
    let mut names: Py = Py::None;
    let mut part: Py = Py::None;
    let mut run: Py = Py::None;
    let mut runs: Py = Py::None;
    let mut start: Py = Py::None;
    let mut token: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Runs of capitalised words that read as names: the people and things a line mentions.\n\n    A name can carry an ordinary word written with a capital: \"Eiffel Tower\" and \"Niagara\n    Falls\" are one name, and the knowledge base knows the whole phrase. Only the run that\n    starts with the name is kept, so an article in front of it (\"the Eiffel Tower\") and a\n    sentence after it (\"the Eiffel Tower is tall\") do not become part of the name.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    runs = __unpk1[0].clone();
    current = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) });
    for __item in __iter2 {
    token = __item.clone();
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'()[]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&token.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&bare.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("-")); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&bare.clone(), "replace", args))(&__args) }, "replace", args))(&__args) }, "isalpha", args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&current.clone(), "append", args))(&__args) };
    } else {
    if py_truthy(&current.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(current.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&runs.clone(), "append", args))(&__args) };
    current = py_list(vec![]);
    }
    }
    }
    if py_truthy(&current.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(current.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&runs.clone(), "append", args))(&__args) };
    }
    names = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&runs.clone());
    for __item in __iter3 {
    run = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(run.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&run.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_name(args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&run.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&names.clone(), "append", args))(&__args) };
    }
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ARTICLES(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&run.clone(), &Py::Int(0)), "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEADING(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&run.clone(), &Py::Int(0)), "lower", args))(&__args) })) } }) {
    run = slice_or_raise(&run.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    if py_truthy(&Py::Bool(!py_truthy(&run.clone()))) {
    continue;
    }
    start = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(run.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let __unpk5146 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk5146[0].clone(); let mut word: Py = __unpk5146[1].clone(); if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _is_name(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACES(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) })) } }) {  __out.push(at.clone()); } } py_list(__out) })() }); __args.push(Py::None); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("next", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_is(&start.clone(), &Py::None))) {
    continue;
    }
    part = py_list(vec![]);
    let __iter4 = py_iter_or_raise(&slice_or_raise(&run.clone(), &start.clone(), &Py::None, &Py::None));
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_NAME_BREAKS(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }))) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "append", args))(&__args) };
    }
    if py_truthy(&part.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&names.clone(), "append", args))(&__args) };
    }
    }
    return names.clone();
    Py::None
}
pub fn detect_entities(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { Py::None };
    let mut add: Py = Py::None;
    let mut after: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut at: Py = Py::None;
    let mut entities: Py = Py::None;
    let mut first_word: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut person: Py = Py::None;
    let mut run: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut verdicts: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The things the line names: people, places, titles and everyday objects.");
    text = if py_truthy(&Py::Bool(py_is(&text.clone(), &Py::None))) { geti(&line.clone(), &py_str("text")) } else { text.clone() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::understanding::kinds::small_talk_only(args))(&__args) }) {
    return py_list(vec![]);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 2, None);
    entities = __unpk1[0].clone();
    seen = __unpk1[1].clone();
pub fn __nested_add_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut add: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut after: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut asked: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut at: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut entities: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut first_word: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut index: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut kind: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut person: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut run: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut seen: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut thing: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut verdicts: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(17).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "name").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut kind: Py = __pos.get(18).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "kind").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut at: Py = __pos.get(19).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "at").map(|(_, v)| v.clone())).unwrap_or_else(|| Py::None);
    let mut key: Py = Py::None;
    key = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "lower", args))(&__args) };
    if py_truthy(&{ let __b = key.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&seen.clone(), &key.clone())); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("text"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "strip", args))(&__args) }), (py_str("type"), kind.clone()), (py_str("at"), if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _position(args))(&__args) } } else { at.clone() })])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entities.clone(), "append", args))(&__args) };
    }
    Py::None
}
    add = {{ let add = add.clone(); let after = after.clone(); let asked = asked.clone(); let at = at.clone(); let entities = entities.clone(); let first_word = first_word.clone(); let index = index.clone(); let kind = kind.clone(); let line = line.clone(); let match_ = match_.clone(); let person = person.clone(); let run = run.clone(); let seen = seen.clone(); let text = text.clone(); let thing = thing.clone(); let verdicts = verdicts.clone(); let word = word.clone(); py_func("add", move |args: &[Py]| { let mut full = vec![add.clone(), after.clone(), asked.clone(), at.clone(), entities.clone(), first_word.clone(), index.clone(), kind.clone(), line.clone(), match_.clone(), person.clone(), run.clone(), seen.clone(), text.clone(), thing.clone(), verdicts.clone(), word.clone()]; full.extend_from_slice(args); __nested_add_1(&full) }) }};
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\*([^*]{2,60})\\*|\\\"([^\\\"]{2,60})\\\"")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter3 {
    match_ = __item.clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } } }); __args.push(py_str("title")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    }
    let __iter4 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bmy ([a-z]+(?:\\s+[a-z]+)?)")); __args.push(geti(&line.clone(), &py_str("clean"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "finditer", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter4 {
    person = __item.clone();
    word = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&person.clone(), "group", args))(&__args) }, "split", args))(&__args) }, &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&g_RELATIONS(), &word.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("my "), &word.clone())); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    }
    }
    let __iter5 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ages_of_a_child(args))(&__args) });
    for __item in __iter5 {
    at = __item.clone();
    after = if py_truthy(&op_lt(&op_add(&at.clone(), &Py::Int(3)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&geti(&line.clone(), &py_str("norm")), &op_add(&at.clone(), &Py::Int(3))) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_contains(&g_THE_PERSONS_OWN(), &geti(&geti(&line.clone(), &py_str("norm")), &op_sub(&at.clone(), &Py::Int(1)))))) {
    continue;
    }
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_AGE_OF_A_CHILD_WORDS(), &after.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PETS(), &after.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing(args))(&__args) } } } } } } }) {
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&geti(&line.clone(), &py_str("norm")), &op_sub(&at.clone(), &Py::Int(1)), &op_add(&at.clone(), &Py::Int(3)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); __args.push(py_str("person")); __kws.extend(vec![("at".to_string(), op_sub(&at.clone(), &Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    }
    let __iter6 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _capitalised(args))(&__args) });
    for __item in __iter6 {
    run = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&py_set(vec![py_str("i"), py_str("i'm"), py_str("im")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&run.clone(), "lower", args))(&__args) }))) {
    continue;
    }
    first_word = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&run.clone(), "lower", args))(&__args) }, "split", args))(&__args) }, &Py::Int(0));
    kind = if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACES(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&run.clone(), "lower", args))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACES(), &first_word.clone())) } }) { py_str("place") } else { py_str("name") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("name"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(run.clone()); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_as_a_place(args))(&__args) } } }) {
    kind = py_str("place");
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(run.clone()); __args.push(kind.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    }
    verdicts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _bad_verdict_positions(args))(&__args) };
    let __iter7 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter7 {
    let __unpk8 = py_unpack(&__item.clone(), 2, None);
    index = __unpk8[0].clone();
    word = __unpk8[1].clone();
    if py_truthy(&Py::Bool(py_contains(&verdicts.clone(), &index.clone()))) {
    continue;
    }
    asked = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asked_as_a_country(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACES(), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_person(args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACES(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(text.clone()); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_place(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _placed_by_a_word(args))(&__args) } } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_str("place")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_RELATIONS(), &word.clone()))) {
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&line.clone(), &py_str("clean")), "split", args))(&__args) }, &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNERS(), &geti(&geti(&line.clone(), &py_str("norm")), &op_sub(&index.clone(), &Py::Int(1))))) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&geti(&geti(&line.clone(), &py_str("norm")), &op_sub(&index.clone(), &Py::Int(1))), &py_str(" ")), &word.clone())); __args.push(py_str("person")); __kws.extend(vec![("at".to_string(), op_sub(&index.clone(), &Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    continue;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_str("person")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing(args))(&__args) };
    if py_truthy(&thing.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&add, args))(&__args) };
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let add = add.clone(); let after = after.clone(); let asked = asked.clone(); let at = at.clone(); let entities = entities.clone(); let first_word = first_word.clone(); let index = index.clone(); let kind = kind.clone(); let line = line.clone(); let match_ = match_.clone(); let person = person.clone(); let run = run.clone(); let seen = seen.clone(); let text = text.clone(); let thing = thing.clone(); let verdicts = verdicts.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let add = add.clone(); let after = after.clone(); let asked = asked.clone(); let at = at.clone(); let entities = entities.clone(); let first_word = first_word.clone(); let index = index.clone(); let kind = kind.clone(); let line = line.clone(); let match_ = match_.clone(); let person = person.clone(); let run = run.clone(); let seen = seen.clone(); let text = text.clone(); let thing = thing.clone(); let verdicts = verdicts.clone(); let word = word.clone(); let entity: Py = __kwp(args, 0, "entity").unwrap_or(Py::None); geti(&entity.clone(), &py_str("at")) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entities.clone(), "sort", args))(&__args) };
    return entities.clone();
    Py::None
}
pub fn _said_as_a_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut run: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "run") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: run"))) };
    let mut norm: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut first: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line puts the name where a place goes (\"in Ljubljana\", \"to Osaka\").");
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'()[]")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&run.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g_NOT_PLACES(), &first.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &first.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _places_a_name(args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_ge(&index.clone(), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACE_THING(), &geti(&norm.clone(), &op_sub(&index.clone(), &Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_WORDS(), &geti(&norm.clone(), &op_sub(&index.clone(), &Py::Int(2))))) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&norm.clone(), &op_sub(&index.clone(), &Py::Int(2))), &py_str("at"))); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_WORK_WORDS()) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _placed_by_a_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let _ = py_str("True when the word sits right after a place word (\"in rome\"), not after an article.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _places_a_name(args))(&__args) };
    Py::None
}
pub fn _has_travel_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Does the line carry a word that is about going somewhere (\"flights\", \"visit\", \"trip\")?");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&op_bitand(&words.clone(), &g_TRAVEL_CUES())) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {  __out.push(Py::Bool(py_contains(&g_TRAVEL_CUES(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)))); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _places_a_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut before: Py = Py::None;
    let _ = py_str("Does the word at index sit where a place goes, so the word before it places the line?\n\n    \"in rome\", \"at the airport in Lisbon\" place a line on their own. \"to\" does not: it points\n    at a person just as often (\"i said to Sarah\"), so it needs a travel word in the line.\n    \"at\" with a work word around it names a workplace (\"she works at Google\"), not a place.\n    ");
    if py_truthy(&{ let __b = op_lt(&index.clone(), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_ge(&index.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) } }) {
    return Py::Bool(false);
    }
    before = geti(&norm.clone(), &op_sub(&index.clone(), &Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&g_PLACE_WORDS_WITH_CUE(), &before.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _has_travel_word(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_contains(&g_PLACE_WORDS(), &before.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&before.clone(), &py_str("at"))); if !py_truthy(&__b) { __b } else { op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_WORK_WORDS()) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_country_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entity: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entity") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entity"))) };
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Is this a place the sentence has said is the country, though the word is also a thing?\n\n    The entity is typed by the place rules - a capital letter, a place word in front, a word only\n    a country takes, \"in europe\" after it - so a place whose last word is also an everyday thing\n    is the country here, and the everyday thing is not what the line is about.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entity.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("type")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }, &py_str("place"))) } }) {
    return Py::Bool(false);
    }
    text = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&entity.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "lower", args))(&__args) }, "split", args))(&__args) };
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_HOMOGRAPHS(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } };
    Py::None
}
pub fn _set_in_a_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut between: Py = Py::None;
    let mut near: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Is the word at index said to be somewhere, the way a country is? \"is turkey in europe\",\n    \"is china in asia\", \"china is bigger than india\". A place named after it with \"in\" between\n    (\"turkey in europe\") or a comparison with another place says the word is the country, not\n    the bird or the cup: nothing is said to be \"in europe\" except a place.\n    ");
    if py_truthy(&{ let __b = op_lt(&op_add(&index.clone(), &Py::Int(2)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(1))), &py_str("in"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACES(), &geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PLACE_HOMOGRAPHS(), &geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(2))))) } } } } } }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&index.clone(), &Py::Int(3))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(op_add(&index.clone(), &Py::Int(4))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    near = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&near.clone(), &index.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACES(), &geti(&norm.clone(), &near.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PLACE_HOMOGRAPHS(), &geti(&norm.clone(), &near.clone()))) } } } }) {
    between = slice_or_raise(&norm.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(index.clone()); __args.push(near.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(index.clone()); __args.push(near.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_COMPARE_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _asked_as_a_country(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut between: Py = Py::None;
    let mut near: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Is the line about the country, not about the thing the country is also called?\n\n    \"is turkey hot in june\" and \"how safe is turkey\" ask about the country; \"the turkey is\n    still frozen\" does not, and the words only a country takes (\"weather\", \"visa\", \"currency\")\n    are how it is asked without a capital letter.\n    ");
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ARTICLES(), &geti(&norm.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_COUNTRY_CUES())) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _set_in_a_place(args))(&__args) }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&index.clone(), &Py::Int(2))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(op_add(&index.clone(), &Py::Int(3))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    near = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&near.clone(), &index.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACES(), &geti(&norm.clone(), &near.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_PLACE_HOMOGRAPHS(), &geti(&norm.clone(), &near.clone()))) } } } }) {
    between = slice_or_raise(&norm.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(index.clone()); __args.push(near.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(index.clone()); __args.push(near.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&between.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_COMPARE_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _names_a_person(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut before: Py = Py::None;
    let _ = py_str("Is the word at index somebody's name in this line, though the vocabulary knows it as a\n    place?");
    before = if py_truthy(&index.clone()) { geti(&norm.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERSON_WORDS(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NAME_FRAMES(), &before.clone())) } }) {
    return Py::Bool(true);
    }
    return { let __b = op_lt(&op_add(&index.clone(), &Py::Int(3)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(1))), &py_str("is"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(2))), &py_str("my"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERSON_WORDS(), &geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(3))))) } } } } } };
    Py::None
}
pub fn person_names(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut names: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place words this line uses as somebody's name, in the order said.\n\n    \"my mate paris is visiting\" is a friend, not the city: the knowledge base must not read the\n    word as the place the line is about. Kept as its own list so the memory lane can still see\n    the name, and every domain gets the same answer to \"is this a place here\".\n    ");
    norm = geti(&line.clone(), &py_str("norm"));
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_list(vec![])]), 2, None);
    seen = __unpk1[0].clone();
    names = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    word = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACES(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&seen.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(index.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_a_person(args))(&__args) } } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&names.clone(), "append", args))(&__args) };
    }
    }
    return names.clone();
    Py::None
}
pub fn _names_a_place(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut asked: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { Py::Bool(false) };
    let _ = py_str("Some place names are also ordinary words (\"bath\", \"reading\", \"turkey\"): those only name\n    a place when the line writes them as one, or asks about the country.");
    if py_truthy(&Py::Bool(!py_contains(&g_PLACE_HOMOGRAPHS(), &word.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COUNTRY_HOMOGRAPHS(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("\\b%s\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "capitalize", args))(&__args) })); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None));
    Py::None
}
pub fn _position(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut first: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where in the line the first mention appears, so entities read in the order said.");
    first = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "split", args))(&__args) }, &Py::Int(0));
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &first.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(first.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "startswith", args))(&__args) } } }) {
    return index.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("norm"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _thing(args: &[Py]) -> Py {
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
    let mut candidate: Py = Py::None;
    let _ = py_str("The everyday thing a word names, singular, or None.\n\n    A short word is looked up before it is turned away. The length check below is there for the\n    plural endings - \"as\" is not \"a\" and \"is\" is not \"i\" - but it was also throwing away every\n    three-letter thing `THINGS` holds, so \"egg\", \"pot\", \"pan\", \"tap\", \"cup\", \"car\", \"job\" and\n    fifty-odd more named nothing at all. A line with no thing of its own (\"how long do i boil an\n    egg\") is then read as a follow-up on whatever came before it, and the mince from the turn\n    ago answered the question about eggs.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_THINGS(), &word.clone()))) {
    return word.clone();
    }
    if py_truthy(&op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "rstrip", args))(&__args) }, if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("es")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None) } else { Py::None }, if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ies")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None) } else { Py::None }]));
    for __item in __iter1 {
    candidate = __item.clone();
    if py_truthy(&{ let __b = candidate.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THINGS(), &candidate.clone())) } }) {
    return word.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _spoken_numbers(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut digits: Py = Py::None;
    let mut value: Py = Py::None;
    let mut values: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The values the normalised line still says, so a digit read as a word can be dropped.\n\n    \"4 the kids\" reads as \"for the kids\": the line has no four in it any more, so listing one\n    would have a module do arithmetic on a number the person never said.\n    ");
    values = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&norm.clone());
    for __item in __iter1 {
    word = __item.clone();
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD_NUMBERS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_is(&value.clone(), &Py::None))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&values.clone(), "add", args))(&__args) };
    continue;
    }
    digits = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD_DIGITS(), "match", args))(&__args) };
    if py_truthy(&digits.clone()) {
    let __try2 = exc::catch(|| {
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&digits.clone(), "group", args))(&__args) }, "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&value.clone(), "is_integer", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { value.clone() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&values.clone(), "add", args))(&__args) };
    }
    }
    return values.clone();
    Py::None
}
pub fn _mixed_number(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut value: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut fraction: Py = Py::None;
    let mut step: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A count with a half after it: \"one and a half kilos\" is 1.5, not a half.\n\n    Read as two numbers it is worse than as none at all - a recipe told \"half a kilo\" of\n    potatoes it was told one and a half kilos of. So the \"and a half\" is folded into the count in\n    front of it, and the words it used are returned so the half is not counted twice.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &op_add(&position.clone(), &Py::Int(4)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("and"))) } }) {
    return py_tuple(vec![value.clone(), py_tuple(vec![])]);
    }
    step = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_tuple(vec![py_str("a")]), py_tuple(vec![py_str("an")])]), &slice_or_raise(&words.clone(), &Py::Int(1), &Py::Int(2), &Py::None)))) { Py::Int(2) } else { Py::Int(1) };
    fraction = if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &step.clone())) { geti(&words.clone(), &step.clone()) } else { Py::None };
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("half"), py_str("quarter")]), &fraction.clone()))) {
    return py_tuple(vec![value.clone(), py_tuple(vec![])]);
    }
    return py_tuple(vec![op_add(&value.clone(), &if py_truthy(&Py::Bool(py_eq(&fraction.clone(), &py_str("half")))) { Py::Float(0.5) } else { Py::Float(0.25) }), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&position.clone(), &Py::Int(1))); __args.push(op_add(&op_add(&op_add(&position.clone(), &Py::Int(1)), &step.clone()), &Py::Int(1))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("range", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    Py::None
}
pub fn _the_figure_and_its_unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut digits: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "digits") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: digits"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut raw: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "raw") { v.clone() }
    else { py_str("") };
    let mut gap: Py = Py::None;
    let _ = py_str("The figure as it is said back, with the unit only when the word is a unit.\n\n    The unit group of NUMBER_RE takes any word at all, which the unit table then reads: most\n    words are not units and are dropped there, but the text kept them, so a figure said with a\n    reason or a preposition was stored as \"75 because\", \"34 and\", \"2000 on\", \"5 for\". The text is\n    what a reply says back (\"You said 75 because.\"), and it is also what a later value is read\n    out of, so the word that follows belongs in it only when it is a measure of the figure.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&unit.clone()))) {
    return digits.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("%")))) {
    return op_add(&digits.clone(), &py_str("%"));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_UNIT_WORDS(), &unit.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_CLIPPED_UNIT_WORDS(), &unit.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FIGURE_SIZE_WORDS(), &unit.clone())) } } } }) {
    gap = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(raw.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(digits.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(digits.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)), &Py::None), "isspace", args))(&__args) }) { py_str(" ") } else { py_str("") };
    return op_add(&op_add(&digits.clone(), &gap.clone()), &unit.clone());
    }
    return digits.clone();
    Py::None
}
pub fn detect_numbers(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut after_word: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut canonical: Py = Py::None;
    let mut clipped: Py = Py::None;
    let mut digits: Py = Py::None;
    let mut folded: Py = Py::None;
    let mut got: Py = Py::None;
    let mut key: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut merged: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut out: Py = Py::None;
    let mut position: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut spoken: Py = Py::None;
    let mut step: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Numbers in the line with their unit, so a module can use \"2kg\" or \"20 minutes\".");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, py_list(vec![]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 4, None);
    out = __unpk1[0].clone();
    seen = __unpk1[1].clone();
    typed = __unpk1[2].clone();
    merged = __unpk1[3].clone();
    norm = geti(&line.clone(), &py_str("norm"));
    spoken = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _spoken_numbers(args))(&__args) };
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clean"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NUMBER_RE(), "finditer", args))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    raw = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) };
    let __unpk3 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }]), 2, None);
    digits = __unpk3[0].clone();
    unit = __unpk3[1].clone();
    let __try4 = exc::catch(|| {
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(py_str(".")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&digits.clone(), "replace", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret5: Py = Py::Bool(false);
    let mut __val6: Py = Py::None;
    let mut __unhandled7: Option<exc::PyExc> = None;
    match __try4 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret5 = __p[0].clone(); __val6 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled8 = false;
            if !__handled8 && exc::matches_any(&__e, &["ValueError"]) {
                __handled8 = true;
    continue;
            }
            if !__handled8 { __unhandled7 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled7 { exc::raise(__e); }
    if py_truthy(&__ret5) { return __val6; }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&value.clone(), "is_integer", args))(&__args) }) {
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_contains(&spoken.clone(), &value.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MIXED_JOINERS(), &unit.clone())); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clean"))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MIXED_FRACTION(), "match", args))(&__args) } } }) {
    continue;
    }
    canonical = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(unit.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_UNIT_WORDS(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&unit.clone(), "rstrip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_UNIT_WORDS(), "get", args))(&__args) } } };
    if py_truthy(&Py::Bool(py_is(&canonical.clone(), &Py::None))) {
    canonical = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(unit.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLIPPED_UNIT_WORDS(), "get", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&canonical.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&unit.clone())) } }) {
    clipped = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&line.clone(), &py_str("clean"))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLIPPED_AFTER_A_SLASH(), "match", args))(&__args) };
    if py_truthy(&clipped.clone()) {
    canonical = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clipped.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) }); __args.push(py_str("years")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLIPPED_UNIT_WORDS(), "get", args))(&__args) };
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(" ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&geti(&line.clone(), &py_str("clean")), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "count", args))(&__args) }, value.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&typed.clone(), "append", args))(&__args) };
    key = py_tuple(vec![value.clone(), canonical.clone()]);
    if py_truthy(&Py::Bool(!py_contains(&seen.clone(), &key.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("value"), value.clone()), (py_str("unit"), canonical.clone()), (py_str("text"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(digits.clone()); __args.push(unit.clone()); __args.push(raw.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_figure_and_its_unit(args))(&__args) })])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    let __iter9 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter9 {
    let __unpk10 = py_unpack(&__item.clone(), 2, None);
    position = __unpk10[0].clone();
    word = __unpk10[1].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &word.clone()))) {
    if py_truthy(&{ let __b = op_lt(&op_add(&position.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_PORTION_UNITS(), &geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1))))) } }) {
    before = if py_truthy(&position.clone()) { geti(&norm.clone(), &op_sub(&position.clone(), &Py::Int(1))) } else { py_str("") };
    after = if py_truthy(&op_lt(&op_add(&position.clone(), &Py::Int(2)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(2))) } else { py_str("") };
    unit = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_UNIT_WORDS(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_PORTION_NEEDS_OF(), &geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&after.clone(), &py_str("of"))) } }) {
    continue;
    }
    key = py_tuple(vec![Py::Int(1), unit.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_OWNS_THE_UNIT(), &before.clone())); if !py_truthy(&__b) { __b } else { { let __b = unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&seen.clone(), &key.clone())) } } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("value"), Py::Int(1)), (py_str("unit"), unit.clone()), (py_str("text"), op_modulo(&py_str("%s %s"), &py_tuple(vec![word.clone(), geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)))])))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    continue;
    }
    value = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORD_NUMBERS(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&value.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _counts_as_number(args))(&__args) } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&merged.clone(), &position.clone()))) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let __unpk891 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk891[0].clone(); let mut got: Py = __unpk891[1].clone(); __out.push({ let __b = Py::Bool(py_eq(&got.clone(), &value.clone())); if !py_truthy(&__b) { __b } else { op_le(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_sub(&at.clone(), &position.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("abs", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    let __unpk11 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); __args.push(position.clone()); __args.push(value.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _mixed_number(args))(&__args) }, 2, None);
    value = __unpk11[0].clone();
    folded = __unpk11[1].clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(folded.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&merged.clone(), "update", args))(&__args) };
    unit = Py::None;
    step = op_add(&Py::Int(1), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(folded.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    while py_truthy(&{ let __b = op_lt(&op_add(&position.clone(), &step.clone()), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !py_truthy(&__b) { __b } else { op_le(&step.clone(), &Py::Int(5)) } }) {
    after_word = geti(&norm.clone(), &op_add(&position.clone(), &step.clone()));
    if py_truthy(&Py::Bool(py_contains(&g_UNIT_WORDS(), &after_word.clone()))) {
    unit = geti(&g_UNIT_WORDS(), &after_word.clone());
    break;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_NUMBER_FILLER(), &after_word.clone()))) {
    break;
    }
    let __aug12 = op_add(&step.clone(), &Py::Int(1));
    step = __aug12;
    }
    key = py_tuple(vec![value.clone(), unit.clone(), word.clone()]);
    if py_truthy(&Py::Bool(!py_contains(&seen.clone(), &key.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&seen.clone(), "add", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_dict(vec![(py_str("value"), value.clone()), (py_str("unit"), unit.clone()), (py_str("text"), word.clone())])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    return out.clone();
    Py::None
}
pub fn _counts_as_number(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut norm: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut position: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "position") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: position"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut following: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the word there is not being used as a count (\"one\" mostly is not).");
    word = geti(&norm.clone(), &position.clone());
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("one")))) {
    return Py::Bool(false);
    }
    following = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&norm.clone(), &op_add(&position.clone(), &Py::Int(1)), &op_add(&position.clone(), &Py::Int(4)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_tuple(vec![py_str("and"), py_str("a"), py_str("half")]), py_tuple(vec![py_str("and"), py_str("an"), py_str("half")])]), &slice_or_raise(&following.clone(), &Py::None, &Py::Int(3), &Py::None))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_tuple(vec![py_str("and"), py_str("half")]), py_tuple(vec![py_str("and"), py_str("quarter")])]), &slice_or_raise(&following.clone(), &Py::None, &Py::Int(2), &Py::None))) } }) {
    return Py::Bool(false);
    }
    before = if py_truthy(&position.clone()) { geti(&norm.clone(), &op_sub(&position.clone(), &Py::Int(1))) } else { Py::None };
    after = if py_truthy(&op_lt(&op_add(&position.clone(), &Py::Int(1)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(norm.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) { geti(&norm.clone(), &op_add(&position.clone(), &Py::Int(1))) } else { Py::None };
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_UNIT_WORDS(), &after.clone())) } }) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_is(&before.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ONE_IS_A_THING(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&after.clone(), &Py::None)) } } } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.topics".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADDRESS_WORDS" => Some(g_ADDRESS_WORDS()),
            "AGE_AFTER" => Some(g_AGE_AFTER()),
            "AGE_IN_A_YEAR" => Some(g_AGE_IN_A_YEAR()),
            "AGE_NOUNS" => Some(g_AGE_NOUNS()),
            "AGE_OF_A_CHILD_WORDS" => Some(g_AGE_OF_A_CHILD_WORDS()),
            "ANIMALS" => Some(g_ANIMALS()),
            "ARTICLES" => Some(g_ARTICLES()),
            "ASK_PHRASES" => Some(g_ASK_PHRASES()),
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "A_AMOUNT_NOUN" => Some(g_A_AMOUNT_NOUN()),
            "A_AMOUNT_SYMBOL" => Some(g_A_AMOUNT_SYMBOL()),
            "A_CHILD_TILL" => Some(g_A_CHILD_TILL()),
            "A_CHILD_TO_READ_FOR" => Some(g_A_CHILD_TO_READ_FOR()),
            "A_GUT_ONLY" => Some(g_A_GUT_ONLY()),
            "A_MONEY_WORD" => Some(g_A_MONEY_WORD()),
            "A_PERSON_WHO_WANTS" => Some(g_A_PERSON_WHO_WANTS()),
            "A_PORTION_NEEDS_OF" => Some(g_A_PORTION_NEEDS_OF()),
            "A_PORTION_UNITS" => Some(g_A_PORTION_UNITS()),
            "A_THING_WANTED" => Some(g_A_THING_WANTED()),
            "A_WAIT" => Some(g_A_WAIT()),
            "A_WAIT_TO" => Some(g_A_WAIT_TO()),
            "BEHAVIOUR" => Some(g_BEHAVIOUR()),
            "BIG_PURSE" => Some(g_BIG_PURSE()),
            "BODILY_OUTPUT" => Some(g_BODILY_OUTPUT()),
            "BODILY_OUTPUT_NOUN" => Some(g_BODILY_OUTPUT_NOUN()),
            "BODY" => Some(g_BODY()),
            "BODY_CUES" => Some(g_BODY_CUES()),
            "BODY_OR_SELF" => Some(g_BODY_OR_SELF()),
            "BODY_TWO_LIFE" => Some(g_BODY_TWO_LIFE()),
            "BOOK_WORDS" => Some(g_BOOK_WORDS()),
            "BROKE_AS_SUBJECT" => Some(g_BROKE_AS_SUBJECT()),
            "BROKE_AS_TIME" => Some(g_BROKE_AS_TIME()),
            "CANCELS" => Some(g_CANCELS()),
            "CANCEL_AFTER" => Some(g_CANCEL_AFTER()),
            "CARRY" => Some(g_CARRY()),
            "CAR_NAMES" => Some(g_CAR_NAMES()),
            "CLIPPED_AFTER_A_SLASH" => Some(g_CLIPPED_AFTER_A_SLASH()),
            "CLIPPED_UNIT_WORDS" => Some(g_CLIPPED_UNIT_WORDS()),
            "COLD_PEOPLE" => Some(g_COLD_PEOPLE()),
            "COLD_WORDS" => Some(g_COLD_WORDS()),
            "COMMON_WORDS" => Some(g_COMMON_WORDS()),
            "COMMS" => Some(g_COMMS()),
            "COMPARE_WORDS" => Some(g_COMPARE_WORDS()),
            "COMPLAINT_OF" => Some(g_COMPLAINT_OF()),
            "COMPLAINT_OF_NEXT" => Some(g_COMPLAINT_OF_NEXT()),
            "CONDITIONS" => Some(g_CONDITIONS()),
            "CONDITION_BOOST" => Some(g_CONDITION_BOOST()),
            "CONDITION_NEEDS_PART" => Some(g_CONDITION_NEEDS_PART()),
            "CONDITION_NEEDS_WORD" => Some(g_CONDITION_NEEDS_WORD()),
            "CONDITION_NOT_NEXT" => Some(g_CONDITION_NOT_NEXT()),
            "CONDITION_OTHER_LIFE" => Some(g_CONDITION_OTHER_LIFE()),
            "CONDITION_REACH" => Some(g_CONDITION_REACH()),
            "COUNTRY_BOOST" => Some(g_COUNTRY_BOOST()),
            "COUNTRY_CUES" => Some(g_COUNTRY_CUES()),
            "COUNTRY_HOMOGRAPHS" => Some(g_COUNTRY_HOMOGRAPHS()),
            "CROPS" => Some(g_CROPS()),
            "CROP_BOOST" => Some(g_CROP_BOOST()),
            "CROP_KITCHEN" => Some(g_CROP_KITCHEN()),
            "CROP_STATES" => Some(g_CROP_STATES()),
            "CUE_CARRIERS" => Some(g_CUE_CARRIERS()),
            "DEGREE_NOUNS" => Some(g_DEGREE_NOUNS()),
            "DERIVED_ENDINGS" => Some(g_DERIVED_ENDINGS()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "DRIVEN_JOINERS" => Some(g_DRIVEN_JOINERS()),
            "DRIVEN_MOOD" => Some(g_DRIVEN_MOOD()),
            "DRIVEN_PERSON" => Some(g_DRIVEN_PERSON()),
            "DRIVEN_TAIL" => Some(g_DRIVEN_TAIL()),
            "DRIVES" => Some(g_DRIVES()),
            "EXERCISE_AFTER" => Some(g_EXERCISE_AFTER()),
            "EXERCISE_NOUNS" => Some(g_EXERCISE_NOUNS()),
            "EXTRA_PEOPLE" => Some(g_EXTRA_PEOPLE()),
            "FAMILY_PAIRS" => Some(g_FAMILY_PAIRS()),
            "FAMILY_STEM" => Some(g_FAMILY_STEM()),
            "FAMILY_WORD" => Some(g_FAMILY_WORD()),
            "FIGURE_SIZE_WORDS" => Some(g_FIGURE_SIZE_WORDS()),
            "FRIEND_BOOST" => Some(g_FRIEND_BOOST()),
            "FRIEND_WORDS" => Some(g_FRIEND_WORDS()),
            "GENERIC" => Some(g_GENERIC()),
            "GIFTS_OR_OCCASIONS" => Some(g_GIFTS_OR_OCCASIONS()),
            "GIFT_ALWAYS" => Some(g_GIFT_ALWAYS()),
            "GIFT_ASKING" => Some(g_GIFT_ASKING()),
            "GIFT_BOOST" => Some(g_GIFT_BOOST()),
            "GIFT_OCCASION" => Some(g_GIFT_OCCASION()),
            "GIFT_WANTING" => Some(g_GIFT_WANTING()),
            "GIFT_WORDS" => Some(g_GIFT_WORDS()),
            "GIVEN_UP_FOOD" => Some(g_GIVEN_UP_FOOD()),
            "GIVE_DETERMINERS" => Some(g_GIVE_DETERMINERS()),
            "GIVE_RELATIVE" => Some(g_GIVE_RELATIVE()),
            "GIVE_THING" => Some(g_GIVE_THING()),
            "GIVE_THINGS" => Some(g_GIVE_THINGS()),
            "GIVE_TO" => Some(g_GIVE_TO()),
            "GIVE_VERB" => Some(g_GIVE_VERB()),
            "GIVING_UP" => Some(g_GIVING_UP()),
            "GIVING_UP_BOOST" => Some(g_GIVING_UP_BOOST()),
            "GROWING_PLACES" => Some(g_GROWING_PLACES()),
            "HEAD_FOG" => Some(g_HEAD_FOG()),
            "HEAD_FOG_BOOST" => Some(g_HEAD_FOG_BOOST()),
            "HEAD_FOG_REACH" => Some(g_HEAD_FOG_REACH()),
            "HEAD_WORDS" => Some(g_HEAD_WORDS()),
            "HEARERS" => Some(g_HEARERS()),
            "HOT_WORDS" => Some(g_HOT_WORDS()),
            "HURTS" => Some(g_HURTS()),
            "HURT_BOOST" => Some(g_HURT_BOOST()),
            "IDIOM_PHRASES" => Some(g_IDIOM_PHRASES()),
            "JUDGEMENT_OF_RECORD" => Some(g_JUDGEMENT_OF_RECORD()),
            "LEADING" => Some(g_LEADING()),
            "LIVES" => Some(g_LIVES()),
            "MACHINE_BIN_WORDS" => Some(g_MACHINE_BIN_WORDS()),
            "MACHINE_WORD_CUES" => Some(g_MACHINE_WORD_CUES()),
            "MACHINE_WORD_PHRASES" => Some(g_MACHINE_WORD_PHRASES()),
            "MECHANISM" => Some(g_MECHANISM()),
            "MECHANISM_HOW" => Some(g_MECHANISM_HOW()),
            "MIXED_FRACTION" => Some(g_MIXED_FRACTION()),
            "MIXED_JOINERS" => Some(g_MIXED_JOINERS()),
            "MONEY_TIGHT" => Some(g_MONEY_TIGHT()),
            "MONEY_TIGHT_MAX" => Some(g_MONEY_TIGHT_MAX()),
            "NAME_BREAKS" => Some(g_NAME_BREAKS()),
            "NAME_FRAMES" => Some(g_NAME_FRAMES()),
            "NATURE_WORDS" => Some(g_NATURE_WORDS()),
            "NOSE_BLOCKED" => Some(g_NOSE_BLOCKED()),
            "NOSE_BLOCKED_BOOST" => Some(g_NOSE_BLOCKED_BOOST()),
            "NOT_CUT_OUT" => Some(g_NOT_CUT_OUT()),
            "NOT_PLACES" => Some(g_NOT_PLACES()),
            "NOT_THINGS" => Some(g_NOT_THINGS()),
            "NOT_WORDS" => Some(g_NOT_WORDS()),
            "NOUNY_SIGNS" => Some(g_NOUNY_SIGNS()),
            "NUMBER_FILLER" => Some(g_NUMBER_FILLER()),
            "NUMBER_RE" => Some(g_NUMBER_RE()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "ONE_IS_A_THING" => Some(g_ONE_IS_A_THING()),
            "ONLY_A_STOOL" => Some(g_ONLY_A_STOOL()),
            "ORDINARY_VERBS" => Some(g_ORDINARY_VERBS()),
            "OTHER_PERSON" => Some(g_OTHER_PERSON()),
            "OWNED_BY" => Some(g_OWNED_BY()),
            "OWNERS" => Some(g_OWNERS()),
            "OWNING_WORDS" => Some(g_OWNING_WORDS()),
            "OWNS_THE_UNIT" => Some(g_OWNS_THE_UNIT()),
            "OWN_SELF" => Some(g_OWN_SELF()),
            "PARTS" => Some(g_PARTS()),
            "PART_UP_BOOST" => Some(g_PART_UP_BOOST()),
            "PEOPLE_TOPICS" => Some(g_PEOPLE_TOPICS()),
            "PERSON_WORDS" => Some(g_PERSON_WORDS()),
            "PETS" => Some(g_PETS()),
            "PET_BOOST" => Some(g_PET_BOOST()),
            "PHRASES" => Some(g_PHRASES()),
            "PLACES" => Some(g_PLACES()),
            "PLACE_HOMOGRAPHS" => Some(g_PLACE_HOMOGRAPHS()),
            "PLACE_OVER" => Some(g_PLACE_OVER()),
            "PLACE_THING" => Some(g_PLACE_THING()),
            "PLACE_WORDS" => Some(g_PLACE_WORDS()),
            "PLACE_WORDS_WITH_CUE" => Some(g_PLACE_WORDS_WITH_CUE()),
            "PLAYS_UP_BEFORE" => Some(g_PLAYS_UP_BEFORE()),
            "PLAY_UP_REACH" => Some(g_PLAY_UP_REACH()),
            "POLYSEMOUS" => Some(g_POLYSEMOUS()),
            "POLY_FORMS" => Some(g_POLY_FORMS()),
            "PORTION_UNITS" => Some(g_PORTION_UNITS()),
            "PORTION_WORDS" => Some(g_PORTION_WORDS()),
            "RELATIONS" => Some(g_RELATIONS()),
            "RUN_OUT_BEFORE" => Some(g_RUN_OUT_BEFORE()),
            "SAMPLE_WORDS" => Some(g_SAMPLE_WORDS()),
            "SCIENCE_BOOST" => Some(g_SCIENCE_BOOST()),
            "SEASONS" => Some(g_SEASONS()),
            "SENSE_ELSEWHERE" => Some(g_SENSE_ELSEWHERE()),
            "SENSE_ELSEWHERE_FORMS" => Some(g_SENSE_ELSEWHERE_FORMS()),
            "SENSE_NEAR" => Some(g_SENSE_NEAR()),
            "SENSE_PAIRS" => Some(g_SENSE_PAIRS()),
            "SENSE_REACH" => Some(g_SENSE_REACH()),
            "SENSE_WEIGHT" => Some(g_SENSE_WEIGHT()),
            "SIGN_FOLLOWS" => Some(g_SIGN_FOLLOWS()),
            "SIGN_NOT_AFTER" => Some(g_SIGN_NOT_AFTER()),
            "SKIN_BOOST" => Some(g_SKIN_BOOST()),
            "SKIN_PARTS" => Some(g_SKIN_PARTS()),
            "SKIN_REACH" => Some(g_SKIN_REACH()),
            "SKIN_STATES" => Some(g_SKIN_STATES()),
            "STONE_COMPOUNDS" => Some(g_STONE_COMPOUNDS()),
            "STONE_VERBS" => Some(g_STONE_VERBS()),
            "STOOL_AS_FURNITURE" => Some(g_STOOL_AS_FURNITURE()),
            "STRONG" => Some(g_STRONG()),
            "STUDY_OBJECTS" => Some(g_STUDY_OBJECTS()),
            "SUMS_AFTER" => Some(g_SUMS_AFTER()),
            "SUM_MONEY" => Some(g_SUM_MONEY()),
            "TABLE" => Some(g_TABLE()),
            "TEETHING_AND_A_CHILD" => Some(g_TEETHING_AND_A_CHILD()),
            "TEETHING_IS_A_FAULT" => Some(g_TEETHING_IS_A_FAULT()),
            "THE_PEOPLE" => Some(g_THE_PEOPLE()),
            "THE_PERSONS_OWN" => Some(g_THE_PERSONS_OWN()),
            "THINGS" => Some(g_THINGS()),
            "THING_WORDS" => Some(g_THING_WORDS()),
            "TRAVEL_CUES" => Some(g_TRAVEL_CUES()),
            "TRAVEL_STRONG" => Some(g_TRAVEL_STRONG()),
            "TWO_HOMES" => Some(g_TWO_HOMES()),
            "TWO_LIVES" => Some(g_TWO_LIVES()),
            "TWO_LIVES_FORMS" => Some(g_TWO_LIVES_FORMS()),
            "TWO_LIVES_REACH" => Some(g_TWO_LIVES_REACH()),
            "UNIT_WORDS" => Some(g_UNIT_WORDS()),
            "UNWELL_BOOST" => Some(g_UNWELL_BOOST()),
            "UNWELL_IDIOMS" => Some(g_UNWELL_IDIOMS()),
            "UNWELL_NOT" => Some(g_UNWELL_NOT()),
            "UNWELL_SIGNS" => Some(g_UNWELL_SIGNS()),
            "VERB_CUES" => Some(g_VERB_CUES()),
            "VERDICT_NOUNS" => Some(g_VERDICT_NOUNS()),
            "WANTED_BEFORE_A_STOP" => Some(g_WANTED_BEFORE_A_STOP()),
            "WANT_VERBS" => Some(g_WANT_VERBS()),
            "WARMTH_ARE" => Some(g_WARMTH_ARE()),
            "WARMTH_DEGREE" => Some(g_WARMTH_DEGREE()),
            "WARMTH_HABITUAL" => Some(g_WARMTH_HABITUAL()),
            "WARMTH_THEN" => Some(g_WARMTH_THEN()),
            "WARMTH_WORDS" => Some(g_WARMTH_WORDS()),
            "WATCHING" => Some(g_WATCHING()),
            "WATCH_OUT_AFTER" => Some(g_WATCH_OUT_AFTER()),
            "WEAK" => Some(g_WEAK()),
            "WEATHER_BEFORE" => Some(g_WEATHER_BEFORE()),
            "WEATHER_ITSELF" => Some(g_WEATHER_ITSELF()),
            "WORD_DIGITS" => Some(g_WORD_DIGITS()),
            "WORD_NUMBERS" => Some(g_WORD_NUMBERS()),
            "WORK_WORDS" => Some(g_WORK_WORDS()),
            "WORLD" => Some(g_WORLD()),
            "WORLD_NOT" => Some(g_WORLD_NOT()),
            "WORN_OUT" => Some(g_WORN_OUT()),
            "WORN_OUT_BOOST" => Some(g_WORN_OUT_BOOST()),
            "WORN_OUT_CUE" => Some(g_WORN_OUT_CUE()),
            "WORN_OUT_LOOKBACK" => Some(g_WORN_OUT_LOOKBACK()),
            "WORN_OUT_SAID" => Some(g_WORN_OUT_SAID()),
            "_A_STATE_VERB" => Some(g__A_STATE_VERB()),
            "_BREAKS" => Some(g__BREAKS()),
            "_FUNCTION_WORDS" => Some(g__FUNCTION_WORDS()),
            "_GRAND_RELATIONS" => Some(g__GRAND_RELATIONS()),
            "_KIN_BUILT" => Some(g__KIN_BUILT()),
            "_NOT_THE_SUBJECT" => Some(g__NOT_THE_SUBJECT()),
            "_POSSESSIVE" => Some(g__POSSESSIVE()),
            "_SENSE_TIERS" => Some(g__SENSE_TIERS()),
            "_STEPPED_RELATIONS" => Some(g__STEPPED_RELATIONS()),
            "_THE_OWNER_SETTLES" => Some(g__THE_OWNER_SETTLES()),
            "_a_book_for_a_child" => Some(py_func("_a_book_for_a_child", |args: &[Py]| _a_book_for_a_child(args))),
            "_a_country_named" => Some(py_func("_a_country_named", |args: &[Py]| _a_country_named(args))),
            "_a_crop_is_failing" => Some(py_func("_a_crop_is_failing", |args: &[Py]| _a_crop_is_failing(args))),
            "_a_habit_given_up" => Some(py_func("_a_habit_given_up", |args: &[Py]| _a_habit_given_up(args))),
            "_a_part_plays_up" => Some(py_func("_a_part_plays_up", |args: &[Py]| _a_part_plays_up(args))),
            "_a_person_is_unwell" => Some(py_func("_a_person_is_unwell", |args: &[Py]| _a_person_is_unwell(args))),
            "_a_wait_follows" => Some(py_func("_a_wait_follows", |args: &[Py]| _a_wait_follows(args))),
            "_acts_on_a_thing" => Some(py_func("_acts_on_a_thing", |args: &[Py]| _acts_on_a_thing(args))),
            "_acts_on_the_body" => Some(py_func("_acts_on_the_body", |args: &[Py]| _acts_on_the_body(args))),
            "_age_value" => Some(py_func("_age_value", |args: &[Py]| _age_value(args))),
            "_ages_of_a_child" => Some(py_func("_ages_of_a_child", |args: &[Py]| _ages_of_a_child(args))),
            "_asked_as_a_country" => Some(py_func("_asked_as_a_country", |args: &[Py]| _asked_as_a_country(args))),
            "_asks_mechanism" => Some(py_func("_asks_mechanism", |args: &[Py]| _asks_mechanism(args))),
            "_bad_verdict_positions" => Some(py_func("_bad_verdict_positions", |args: &[Py]| _bad_verdict_positions(args))),
            "_capitalised" => Some(py_func("_capitalised", |args: &[Py]| _capitalised(args))),
            "_counts_as_number" => Some(py_func("_counts_as_number", |args: &[Py]| _counts_as_number(args))),
            "_drive_idiom" => Some(py_func("_drive_idiom", |args: &[Py]| _drive_idiom(args))),
            "_drives_a_person" => Some(py_func("_drives_a_person", |args: &[Py]| _drives_a_person(args))),
            "_family" => Some(py_func("_family", |args: &[Py]| _family(args))),
            "_form" => Some(g__form()),
            "_forms" => Some(g__forms()),
            "_function_words" => Some(py_func("_function_words", |args: &[Py]| _function_words(args))),
            "_gift_ask" => Some(py_func("_gift_ask", |args: &[Py]| _gift_ask(args))),
            "_gift_still_to_get" => Some(py_func("_gift_still_to_get", |args: &[Py]| _gift_still_to_get(args))),
            "_has_travel_word" => Some(py_func("_has_travel_word", |args: &[Py]| _has_travel_word(args))),
            "_head_fog" => Some(py_func("_head_fog", |args: &[Py]| _head_fog(args))),
            "_idiom_positions" => Some(py_func("_idiom_positions", |args: &[Py]| _idiom_positions(args))),
            "_index" => Some(py_func("_index", |args: &[Py]| _index(args))),
            "_is_a_complaint" => Some(py_func("_is_a_complaint", |args: &[Py]| _is_a_complaint(args))),
            "_is_a_portion" => Some(py_func("_is_a_portion", |args: &[Py]| _is_a_portion(args))),
            "_is_name" => Some(py_func("_is_name", |args: &[Py]| _is_name(args))),
            "_is_run_out" => Some(py_func("_is_run_out", |args: &[Py]| _is_run_out(args))),
            "_looks_like_an_ask" => Some(py_func("_looks_like_an_ask", |args: &[Py]| _looks_like_an_ask(args))),
            "_lookup" => Some(py_func("_lookup", |args: &[Py]| _lookup(args))),
            "_means_something_else" => Some(py_func("_means_something_else", |args: &[Py]| _means_something_else(args))),
            "_mixed_number" => Some(py_func("_mixed_number", |args: &[Py]| _mixed_number(args))),
            "_named_condition" => Some(py_func("_named_condition", |args: &[Py]| _named_condition(args))),
            "_names_a_friend_of_their_own" => Some(py_func("_names_a_friend_of_their_own", |args: &[Py]| _names_a_friend_of_their_own(args))),
            "_names_a_person" => Some(py_func("_names_a_person", |args: &[Py]| _names_a_person(args))),
            "_names_a_place" => Some(py_func("_names_a_place", |args: &[Py]| _names_a_place(args))),
            "_owns_an_animal" => Some(py_func("_owns_an_animal", |args: &[Py]| _owns_an_animal(args))),
            "_person_words" => Some(py_func("_person_words", |args: &[Py]| _person_words(args))),
            "_place_points" => Some(py_func("_place_points", |args: &[Py]| _place_points(args))),
            "_placed_by_a_word" => Some(py_func("_placed_by_a_word", |args: &[Py]| _placed_by_a_word(args))),
            "_places_a_name" => Some(py_func("_places_a_name", |args: &[Py]| _places_a_name(args))),
            "_position" => Some(py_func("_position", |args: &[Py]| _position(args))),
            "_purse_amount" => Some(py_func("_purse_amount", |args: &[Py]| _purse_amount(args))),
            "_said_as_a_place" => Some(py_func("_said_as_a_place", |args: &[Py]| _said_as_a_place(args))),
            "_says_nothing_here" => Some(py_func("_says_nothing_here", |args: &[Py]| _says_nothing_here(args))),
            "_sense_cue" => Some(py_func("_sense_cue", |args: &[Py]| _sense_cue(args))),
            "_sense_topic" => Some(g__sense_topic()),
            "_set_in_a_place" => Some(py_func("_set_in_a_place", |args: &[Py]| _set_in_a_place(args))),
            "_sign_said_of_a_person" => Some(py_func("_sign_said_of_a_person", |args: &[Py]| _sign_said_of_a_person(args))),
            "_skin_state" => Some(py_func("_skin_state", |args: &[Py]| _skin_state(args))),
            "_spoken_numbers" => Some(py_func("_spoken_numbers", |args: &[Py]| _spoken_numbers(args))),
            "_takes_an_object" => Some(py_func("_takes_an_object", |args: &[Py]| _takes_an_object(args))),
            "_teething_is_a_fault" => Some(py_func("_teething_is_a_fault", |args: &[Py]| _teething_is_a_fault(args))),
            "_the_figure_and_its_unit" => Some(py_func("_the_figure_and_its_unit", |args: &[Py]| _the_figure_and_its_unit(args))),
            "_the_persons_own_output" => Some(py_func("_the_persons_own_output", |args: &[Py]| _the_persons_own_output(args))),
            "_the_sense_the_cues_name" => Some(py_func("_the_sense_the_cues_name", |args: &[Py]| _the_sense_the_cues_name(args))),
            "_the_weather_is_just_a_season" => Some(py_func("_the_weather_is_just_a_season", |args: &[Py]| _the_weather_is_just_a_season(args))),
            "_thing" => Some(py_func("_thing", |args: &[Py]| _thing(args))),
            "_thing_a_cue_acts_on" => Some(py_func("_thing_a_cue_acts_on", |args: &[Py]| _thing_a_cue_acts_on(args))),
            "_tiers" => Some(g__tiers()),
            "_watches_out_for" => Some(py_func("_watches_out_for", |args: &[Py]| _watches_out_for(args))),
            "_what_someone_else_wants" => Some(py_func("_what_someone_else_wants", |args: &[Py]| _what_someone_else_wants(args))),
            "_work_out_sums" => Some(py_func("_work_out_sums", |args: &[Py]| _work_out_sums(args))),
            "_worn_out" => Some(py_func("_worn_out", |args: &[Py]| _worn_out(args))),
            "broke_at" => Some(py_func("broke_at", |args: &[Py]| broke_at(args))),
            "detect_entities" => Some(py_func("detect_entities", |args: &[Py]| detect_entities(args))),
            "detect_numbers" => Some(py_func("detect_numbers", |args: &[Py]| detect_numbers(args))),
            "detect_topic" => Some(py_func("detect_topic", |args: &[Py]| detect_topic(args))),
            "gift_ask" => Some(py_func("gift_ask", |args: &[Py]| gift_ask(args))),
            "is_a_machine_word" => Some(py_func("is_a_machine_word", |args: &[Py]| is_a_machine_word(args))),
            "names_another_thing" => Some(py_func("names_another_thing", |args: &[Py]| names_another_thing(args))),
            "own_output_word" => Some(py_func("own_output_word", |args: &[Py]| own_output_word(args))),
            "person_names" => Some(py_func("person_names", |args: &[Py]| person_names(args))),
            "purse_till_at" => Some(py_func("purse_till_at", |args: &[Py]| purse_till_at(args))),
            "rank_topics" => Some(py_func("rank_topics", |args: &[Py]| rank_topics(args))),
            "reads_as_unfit" => Some(py_func("reads_as_unfit", |args: &[Py]| reads_as_unfit(args))),
            "says_another_thing" => Some(py_func("says_another_thing", |args: &[Py]| says_another_thing(args))),
            "score_topics" => Some(py_func("score_topics", |args: &[Py]| score_topics(args))),
            "stone_weight_at" => Some(py_func("stone_weight_at", |args: &[Py]| stone_weight_at(args))),
            "subject_topic" => Some(py_func("subject_topic", |args: &[Py]| subject_topic(args))),
            "the_bin_is_on_a_machine" => Some(py_func("the_bin_is_on_a_machine", |args: &[Py]| the_bin_is_on_a_machine(args))),
            "the_warmth_said" => Some(py_func("the_warmth_said", |args: &[Py]| the_warmth_said(args))),
            "the_words_that_said" => Some(py_func("the_words_that_said", |args: &[Py]| the_words_that_said(args))),
            "tight_money_at" => Some(py_func("tight_money_at", |args: &[Py]| tight_money_at(args))),
            "tight_on_money" => Some(py_func("tight_on_money", |args: &[Py]| tight_on_money(args))),
            "unclear_subject" => Some(py_func("unclear_subject", |args: &[Py]| unclear_subject(args))),
            "unsure_words" => Some(py_func("unsure_words", |args: &[Py]| unsure_words(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADDRESS_WORDS" => s_ADDRESS_WORDS(v),
            "AGE_AFTER" => s_AGE_AFTER(v),
            "AGE_IN_A_YEAR" => s_AGE_IN_A_YEAR(v),
            "AGE_NOUNS" => s_AGE_NOUNS(v),
            "AGE_OF_A_CHILD_WORDS" => s_AGE_OF_A_CHILD_WORDS(v),
            "ANIMALS" => s_ANIMALS(v),
            "ARTICLES" => s_ARTICLES(v),
            "ASK_PHRASES" => s_ASK_PHRASES(v),
            "ASK_WORDS" => s_ASK_WORDS(v),
            "A_AMOUNT_NOUN" => s_A_AMOUNT_NOUN(v),
            "A_AMOUNT_SYMBOL" => s_A_AMOUNT_SYMBOL(v),
            "A_CHILD_TILL" => s_A_CHILD_TILL(v),
            "A_CHILD_TO_READ_FOR" => s_A_CHILD_TO_READ_FOR(v),
            "A_GUT_ONLY" => s_A_GUT_ONLY(v),
            "A_MONEY_WORD" => s_A_MONEY_WORD(v),
            "A_PERSON_WHO_WANTS" => s_A_PERSON_WHO_WANTS(v),
            "A_PORTION_NEEDS_OF" => s_A_PORTION_NEEDS_OF(v),
            "A_PORTION_UNITS" => s_A_PORTION_UNITS(v),
            "A_THING_WANTED" => s_A_THING_WANTED(v),
            "A_WAIT" => s_A_WAIT(v),
            "A_WAIT_TO" => s_A_WAIT_TO(v),
            "BEHAVIOUR" => s_BEHAVIOUR(v),
            "BIG_PURSE" => s_BIG_PURSE(v),
            "BODILY_OUTPUT" => s_BODILY_OUTPUT(v),
            "BODILY_OUTPUT_NOUN" => s_BODILY_OUTPUT_NOUN(v),
            "BODY" => s_BODY(v),
            "BODY_CUES" => s_BODY_CUES(v),
            "BODY_OR_SELF" => s_BODY_OR_SELF(v),
            "BODY_TWO_LIFE" => s_BODY_TWO_LIFE(v),
            "BOOK_WORDS" => s_BOOK_WORDS(v),
            "BROKE_AS_SUBJECT" => s_BROKE_AS_SUBJECT(v),
            "BROKE_AS_TIME" => s_BROKE_AS_TIME(v),
            "CANCELS" => s_CANCELS(v),
            "CANCEL_AFTER" => s_CANCEL_AFTER(v),
            "CARRY" => s_CARRY(v),
            "CAR_NAMES" => s_CAR_NAMES(v),
            "CLIPPED_AFTER_A_SLASH" => s_CLIPPED_AFTER_A_SLASH(v),
            "CLIPPED_UNIT_WORDS" => s_CLIPPED_UNIT_WORDS(v),
            "COLD_PEOPLE" => s_COLD_PEOPLE(v),
            "COLD_WORDS" => s_COLD_WORDS(v),
            "COMMON_WORDS" => s_COMMON_WORDS(v),
            "COMMS" => s_COMMS(v),
            "COMPARE_WORDS" => s_COMPARE_WORDS(v),
            "COMPLAINT_OF" => s_COMPLAINT_OF(v),
            "COMPLAINT_OF_NEXT" => s_COMPLAINT_OF_NEXT(v),
            "CONDITIONS" => s_CONDITIONS(v),
            "CONDITION_BOOST" => s_CONDITION_BOOST(v),
            "CONDITION_NEEDS_PART" => s_CONDITION_NEEDS_PART(v),
            "CONDITION_NEEDS_WORD" => s_CONDITION_NEEDS_WORD(v),
            "CONDITION_NOT_NEXT" => s_CONDITION_NOT_NEXT(v),
            "CONDITION_OTHER_LIFE" => s_CONDITION_OTHER_LIFE(v),
            "CONDITION_REACH" => s_CONDITION_REACH(v),
            "COUNTRY_BOOST" => s_COUNTRY_BOOST(v),
            "COUNTRY_CUES" => s_COUNTRY_CUES(v),
            "COUNTRY_HOMOGRAPHS" => s_COUNTRY_HOMOGRAPHS(v),
            "CROPS" => s_CROPS(v),
            "CROP_BOOST" => s_CROP_BOOST(v),
            "CROP_KITCHEN" => s_CROP_KITCHEN(v),
            "CROP_STATES" => s_CROP_STATES(v),
            "CUE_CARRIERS" => s_CUE_CARRIERS(v),
            "DEGREE_NOUNS" => s_DEGREE_NOUNS(v),
            "DERIVED_ENDINGS" => s_DERIVED_ENDINGS(v),
            "DETERMINERS" => s_DETERMINERS(v),
            "DRIVEN_JOINERS" => s_DRIVEN_JOINERS(v),
            "DRIVEN_MOOD" => s_DRIVEN_MOOD(v),
            "DRIVEN_PERSON" => s_DRIVEN_PERSON(v),
            "DRIVEN_TAIL" => s_DRIVEN_TAIL(v),
            "DRIVES" => s_DRIVES(v),
            "EXERCISE_AFTER" => s_EXERCISE_AFTER(v),
            "EXERCISE_NOUNS" => s_EXERCISE_NOUNS(v),
            "EXTRA_PEOPLE" => s_EXTRA_PEOPLE(v),
            "FAMILY_PAIRS" => s_FAMILY_PAIRS(v),
            "FAMILY_STEM" => s_FAMILY_STEM(v),
            "FAMILY_WORD" => s_FAMILY_WORD(v),
            "FIGURE_SIZE_WORDS" => s_FIGURE_SIZE_WORDS(v),
            "FRIEND_BOOST" => s_FRIEND_BOOST(v),
            "FRIEND_WORDS" => s_FRIEND_WORDS(v),
            "GENERIC" => s_GENERIC(v),
            "GIFTS_OR_OCCASIONS" => s_GIFTS_OR_OCCASIONS(v),
            "GIFT_ALWAYS" => s_GIFT_ALWAYS(v),
            "GIFT_ASKING" => s_GIFT_ASKING(v),
            "GIFT_BOOST" => s_GIFT_BOOST(v),
            "GIFT_OCCASION" => s_GIFT_OCCASION(v),
            "GIFT_WANTING" => s_GIFT_WANTING(v),
            "GIFT_WORDS" => s_GIFT_WORDS(v),
            "GIVEN_UP_FOOD" => s_GIVEN_UP_FOOD(v),
            "GIVE_DETERMINERS" => s_GIVE_DETERMINERS(v),
            "GIVE_RELATIVE" => s_GIVE_RELATIVE(v),
            "GIVE_THING" => s_GIVE_THING(v),
            "GIVE_THINGS" => s_GIVE_THINGS(v),
            "GIVE_TO" => s_GIVE_TO(v),
            "GIVE_VERB" => s_GIVE_VERB(v),
            "GIVING_UP" => s_GIVING_UP(v),
            "GIVING_UP_BOOST" => s_GIVING_UP_BOOST(v),
            "GROWING_PLACES" => s_GROWING_PLACES(v),
            "HEAD_FOG" => s_HEAD_FOG(v),
            "HEAD_FOG_BOOST" => s_HEAD_FOG_BOOST(v),
            "HEAD_FOG_REACH" => s_HEAD_FOG_REACH(v),
            "HEAD_WORDS" => s_HEAD_WORDS(v),
            "HEARERS" => s_HEARERS(v),
            "HOT_WORDS" => s_HOT_WORDS(v),
            "HURTS" => s_HURTS(v),
            "HURT_BOOST" => s_HURT_BOOST(v),
            "IDIOM_PHRASES" => s_IDIOM_PHRASES(v),
            "JUDGEMENT_OF_RECORD" => s_JUDGEMENT_OF_RECORD(v),
            "LEADING" => s_LEADING(v),
            "LIVES" => s_LIVES(v),
            "MACHINE_BIN_WORDS" => s_MACHINE_BIN_WORDS(v),
            "MACHINE_WORD_CUES" => s_MACHINE_WORD_CUES(v),
            "MACHINE_WORD_PHRASES" => s_MACHINE_WORD_PHRASES(v),
            "MECHANISM" => s_MECHANISM(v),
            "MECHANISM_HOW" => s_MECHANISM_HOW(v),
            "MIXED_FRACTION" => s_MIXED_FRACTION(v),
            "MIXED_JOINERS" => s_MIXED_JOINERS(v),
            "MONEY_TIGHT" => s_MONEY_TIGHT(v),
            "MONEY_TIGHT_MAX" => s_MONEY_TIGHT_MAX(v),
            "NAME_BREAKS" => s_NAME_BREAKS(v),
            "NAME_FRAMES" => s_NAME_FRAMES(v),
            "NATURE_WORDS" => s_NATURE_WORDS(v),
            "NOSE_BLOCKED" => s_NOSE_BLOCKED(v),
            "NOSE_BLOCKED_BOOST" => s_NOSE_BLOCKED_BOOST(v),
            "NOT_CUT_OUT" => s_NOT_CUT_OUT(v),
            "NOT_PLACES" => s_NOT_PLACES(v),
            "NOT_THINGS" => s_NOT_THINGS(v),
            "NOT_WORDS" => s_NOT_WORDS(v),
            "NOUNY_SIGNS" => s_NOUNY_SIGNS(v),
            "NUMBER_FILLER" => s_NUMBER_FILLER(v),
            "NUMBER_RE" => s_NUMBER_RE(v),
            "NUMBER_WORDS" => s_NUMBER_WORDS(v),
            "ONE_IS_A_THING" => s_ONE_IS_A_THING(v),
            "ONLY_A_STOOL" => s_ONLY_A_STOOL(v),
            "ORDINARY_VERBS" => s_ORDINARY_VERBS(v),
            "OTHER_PERSON" => s_OTHER_PERSON(v),
            "OWNED_BY" => s_OWNED_BY(v),
            "OWNERS" => s_OWNERS(v),
            "OWNING_WORDS" => s_OWNING_WORDS(v),
            "OWNS_THE_UNIT" => s_OWNS_THE_UNIT(v),
            "OWN_SELF" => s_OWN_SELF(v),
            "PARTS" => s_PARTS(v),
            "PART_UP_BOOST" => s_PART_UP_BOOST(v),
            "PEOPLE_TOPICS" => s_PEOPLE_TOPICS(v),
            "PERSON_WORDS" => s_PERSON_WORDS(v),
            "PETS" => s_PETS(v),
            "PET_BOOST" => s_PET_BOOST(v),
            "PHRASES" => s_PHRASES(v),
            "PLACES" => s_PLACES(v),
            "PLACE_HOMOGRAPHS" => s_PLACE_HOMOGRAPHS(v),
            "PLACE_OVER" => s_PLACE_OVER(v),
            "PLACE_THING" => s_PLACE_THING(v),
            "PLACE_WORDS" => s_PLACE_WORDS(v),
            "PLACE_WORDS_WITH_CUE" => s_PLACE_WORDS_WITH_CUE(v),
            "PLAYS_UP_BEFORE" => s_PLAYS_UP_BEFORE(v),
            "PLAY_UP_REACH" => s_PLAY_UP_REACH(v),
            "POLYSEMOUS" => s_POLYSEMOUS(v),
            "POLY_FORMS" => s_POLY_FORMS(v),
            "PORTION_UNITS" => s_PORTION_UNITS(v),
            "PORTION_WORDS" => s_PORTION_WORDS(v),
            "RELATIONS" => s_RELATIONS(v),
            "RUN_OUT_BEFORE" => s_RUN_OUT_BEFORE(v),
            "SAMPLE_WORDS" => s_SAMPLE_WORDS(v),
            "SCIENCE_BOOST" => s_SCIENCE_BOOST(v),
            "SEASONS" => s_SEASONS(v),
            "SENSE_ELSEWHERE" => s_SENSE_ELSEWHERE(v),
            "SENSE_ELSEWHERE_FORMS" => s_SENSE_ELSEWHERE_FORMS(v),
            "SENSE_NEAR" => s_SENSE_NEAR(v),
            "SENSE_PAIRS" => s_SENSE_PAIRS(v),
            "SENSE_REACH" => s_SENSE_REACH(v),
            "SENSE_WEIGHT" => s_SENSE_WEIGHT(v),
            "SIGN_FOLLOWS" => s_SIGN_FOLLOWS(v),
            "SIGN_NOT_AFTER" => s_SIGN_NOT_AFTER(v),
            "SKIN_BOOST" => s_SKIN_BOOST(v),
            "SKIN_PARTS" => s_SKIN_PARTS(v),
            "SKIN_REACH" => s_SKIN_REACH(v),
            "SKIN_STATES" => s_SKIN_STATES(v),
            "STONE_COMPOUNDS" => s_STONE_COMPOUNDS(v),
            "STONE_VERBS" => s_STONE_VERBS(v),
            "STOOL_AS_FURNITURE" => s_STOOL_AS_FURNITURE(v),
            "STRONG" => s_STRONG(v),
            "STUDY_OBJECTS" => s_STUDY_OBJECTS(v),
            "SUMS_AFTER" => s_SUMS_AFTER(v),
            "SUM_MONEY" => s_SUM_MONEY(v),
            "TABLE" => s_TABLE(v),
            "TEETHING_AND_A_CHILD" => s_TEETHING_AND_A_CHILD(v),
            "TEETHING_IS_A_FAULT" => s_TEETHING_IS_A_FAULT(v),
            "THE_PEOPLE" => s_THE_PEOPLE(v),
            "THE_PERSONS_OWN" => s_THE_PERSONS_OWN(v),
            "THINGS" => s_THINGS(v),
            "THING_WORDS" => s_THING_WORDS(v),
            "TRAVEL_CUES" => s_TRAVEL_CUES(v),
            "TRAVEL_STRONG" => s_TRAVEL_STRONG(v),
            "TWO_HOMES" => s_TWO_HOMES(v),
            "TWO_LIVES" => s_TWO_LIVES(v),
            "TWO_LIVES_FORMS" => s_TWO_LIVES_FORMS(v),
            "TWO_LIVES_REACH" => s_TWO_LIVES_REACH(v),
            "UNIT_WORDS" => s_UNIT_WORDS(v),
            "UNWELL_BOOST" => s_UNWELL_BOOST(v),
            "UNWELL_IDIOMS" => s_UNWELL_IDIOMS(v),
            "UNWELL_NOT" => s_UNWELL_NOT(v),
            "UNWELL_SIGNS" => s_UNWELL_SIGNS(v),
            "VERB_CUES" => s_VERB_CUES(v),
            "VERDICT_NOUNS" => s_VERDICT_NOUNS(v),
            "WANTED_BEFORE_A_STOP" => s_WANTED_BEFORE_A_STOP(v),
            "WANT_VERBS" => s_WANT_VERBS(v),
            "WARMTH_ARE" => s_WARMTH_ARE(v),
            "WARMTH_DEGREE" => s_WARMTH_DEGREE(v),
            "WARMTH_HABITUAL" => s_WARMTH_HABITUAL(v),
            "WARMTH_THEN" => s_WARMTH_THEN(v),
            "WARMTH_WORDS" => s_WARMTH_WORDS(v),
            "WATCHING" => s_WATCHING(v),
            "WATCH_OUT_AFTER" => s_WATCH_OUT_AFTER(v),
            "WEAK" => s_WEAK(v),
            "WEATHER_BEFORE" => s_WEATHER_BEFORE(v),
            "WEATHER_ITSELF" => s_WEATHER_ITSELF(v),
            "WORD_DIGITS" => s_WORD_DIGITS(v),
            "WORD_NUMBERS" => s_WORD_NUMBERS(v),
            "WORK_WORDS" => s_WORK_WORDS(v),
            "WORLD" => s_WORLD(v),
            "WORLD_NOT" => s_WORLD_NOT(v),
            "WORN_OUT" => s_WORN_OUT(v),
            "WORN_OUT_BOOST" => s_WORN_OUT_BOOST(v),
            "WORN_OUT_CUE" => s_WORN_OUT_CUE(v),
            "WORN_OUT_LOOKBACK" => s_WORN_OUT_LOOKBACK(v),
            "WORN_OUT_SAID" => s_WORN_OUT_SAID(v),
            "_A_STATE_VERB" => s__A_STATE_VERB(v),
            "_BREAKS" => s__BREAKS(v),
            "_FUNCTION_WORDS" => s__FUNCTION_WORDS(v),
            "_GRAND_RELATIONS" => s__GRAND_RELATIONS(v),
            "_KIN_BUILT" => s__KIN_BUILT(v),
            "_NOT_THE_SUBJECT" => s__NOT_THE_SUBJECT(v),
            "_POSSESSIVE" => s__POSSESSIVE(v),
            "_SENSE_TIERS" => s__SENSE_TIERS(v),
            "_STEPPED_RELATIONS" => s__STEPPED_RELATIONS(v),
            "_THE_OWNER_SETTLES" => s__THE_OWNER_SETTLES(v),
            "_form" => s__form(v),
            "_forms" => s__forms(v),
            "_sense_topic" => s__sense_topic(v),
            "_tiers" => s__tiers(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.topics", module_obj);
    pyrt::xrefs::reg_fn("understanding.topics", "_work_out_sums", py_func("_work_out_sums", |args: &[Py]| _work_out_sums(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_head_fog", py_func("_head_fog", |args: &[Py]| _head_fog(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_skin_state", py_func("_skin_state", |args: &[Py]| _skin_state(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_a_part_plays_up", py_func("_a_part_plays_up", |args: &[Py]| _a_part_plays_up(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_named_condition", py_func("_named_condition", |args: &[Py]| _named_condition(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_person_words", py_func("_person_words", |args: &[Py]| _person_words(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "reads_as_unfit", py_func("reads_as_unfit", |args: &[Py]| reads_as_unfit(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_a_habit_given_up", py_func("_a_habit_given_up", |args: &[Py]| _a_habit_given_up(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "the_bin_is_on_a_machine", py_func("the_bin_is_on_a_machine", |args: &[Py]| the_bin_is_on_a_machine(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "unclear_subject", py_func("unclear_subject", |args: &[Py]| unclear_subject(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_teething_is_a_fault", py_func("_teething_is_a_fault", |args: &[Py]| _teething_is_a_fault(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "the_warmth_said", py_func("the_warmth_said", |args: &[Py]| the_warmth_said(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_means_something_else", py_func("_means_something_else", |args: &[Py]| _means_something_else(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "says_another_thing", py_func("says_another_thing", |args: &[Py]| says_another_thing(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "names_another_thing", py_func("names_another_thing", |args: &[Py]| names_another_thing(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "is_a_machine_word", py_func("is_a_machine_word", |args: &[Py]| is_a_machine_word(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_the_sense_the_cues_name", py_func("_the_sense_the_cues_name", |args: &[Py]| _the_sense_the_cues_name(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_sense_cue", py_func("_sense_cue", |args: &[Py]| _sense_cue(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_index", py_func("_index", |args: &[Py]| _index(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_takes_an_object", py_func("_takes_an_object", |args: &[Py]| _takes_an_object(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_acts_on_a_thing", py_func("_acts_on_a_thing", |args: &[Py]| _acts_on_a_thing(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_thing_a_cue_acts_on", py_func("_thing_a_cue_acts_on", |args: &[Py]| _thing_a_cue_acts_on(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_acts_on_the_body", py_func("_acts_on_the_body", |args: &[Py]| _acts_on_the_body(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_is_run_out", py_func("_is_run_out", |args: &[Py]| _is_run_out(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_watches_out_for", py_func("_watches_out_for", |args: &[Py]| _watches_out_for(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_says_nothing_here", py_func("_says_nothing_here", |args: &[Py]| _says_nothing_here(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_age_value", py_func("_age_value", |args: &[Py]| _age_value(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_ages_of_a_child", py_func("_ages_of_a_child", |args: &[Py]| _ages_of_a_child(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_a_book_for_a_child", py_func("_a_book_for_a_child", |args: &[Py]| _a_book_for_a_child(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_drives_a_person", py_func("_drives_a_person", |args: &[Py]| _drives_a_person(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_idiom_positions", py_func("_idiom_positions", |args: &[Py]| _idiom_positions(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_bad_verdict_positions", py_func("_bad_verdict_positions", |args: &[Py]| _bad_verdict_positions(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_drive_idiom", py_func("_drive_idiom", |args: &[Py]| _drive_idiom(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_is_a_complaint", py_func("_is_a_complaint", |args: &[Py]| _is_a_complaint(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_function_words", py_func("_function_words", |args: &[Py]| _function_words(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_family", py_func("_family", |args: &[Py]| _family(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_lookup", py_func("_lookup", |args: &[Py]| _lookup(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_is_a_portion", py_func("_is_a_portion", |args: &[Py]| _is_a_portion(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_the_weather_is_just_a_season", py_func("_the_weather_is_just_a_season", |args: &[Py]| _the_weather_is_just_a_season(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "score_topics", py_func("score_topics", |args: &[Py]| score_topics(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "unsure_words", py_func("unsure_words", |args: &[Py]| unsure_words(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_place_points", py_func("_place_points", |args: &[Py]| _place_points(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_what_someone_else_wants", py_func("_what_someone_else_wants", |args: &[Py]| _what_someone_else_wants(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_looks_like_an_ask", py_func("_looks_like_an_ask", |args: &[Py]| _looks_like_an_ask(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_names_a_friend_of_their_own", py_func("_names_a_friend_of_their_own", |args: &[Py]| _names_a_friend_of_their_own(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_the_persons_own_output", py_func("_the_persons_own_output", |args: &[Py]| _the_persons_own_output(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "own_output_word", py_func("own_output_word", |args: &[Py]| own_output_word(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_owns_an_animal", py_func("_owns_an_animal", |args: &[Py]| _owns_an_animal(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_a_crop_is_failing", py_func("_a_crop_is_failing", |args: &[Py]| _a_crop_is_failing(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_asks_mechanism", py_func("_asks_mechanism", |args: &[Py]| _asks_mechanism(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_sign_said_of_a_person", py_func("_sign_said_of_a_person", |args: &[Py]| _sign_said_of_a_person(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_a_person_is_unwell", py_func("_a_person_is_unwell", |args: &[Py]| _a_person_is_unwell(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "broke_at", py_func("broke_at", |args: &[Py]| broke_at(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_purse_amount", py_func("_purse_amount", |args: &[Py]| _purse_amount(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_a_wait_follows", py_func("_a_wait_follows", |args: &[Py]| _a_wait_follows(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "purse_till_at", py_func("purse_till_at", |args: &[Py]| purse_till_at(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "tight_money_at", py_func("tight_money_at", |args: &[Py]| tight_money_at(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "stone_weight_at", py_func("stone_weight_at", |args: &[Py]| stone_weight_at(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "tight_on_money", py_func("tight_on_money", |args: &[Py]| tight_on_money(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_worn_out", py_func("_worn_out", |args: &[Py]| _worn_out(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "rank_topics", py_func("rank_topics", |args: &[Py]| rank_topics(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "gift_ask", py_func("gift_ask", |args: &[Py]| gift_ask(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_gift_ask", py_func("_gift_ask", |args: &[Py]| _gift_ask(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_gift_still_to_get", py_func("_gift_still_to_get", |args: &[Py]| _gift_still_to_get(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "subject_topic", py_func("subject_topic", |args: &[Py]| subject_topic(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "the_words_that_said", py_func("the_words_that_said", |args: &[Py]| the_words_that_said(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "detect_topic", py_func("detect_topic", |args: &[Py]| detect_topic(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_is_name", py_func("_is_name", |args: &[Py]| _is_name(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_capitalised", py_func("_capitalised", |args: &[Py]| _capitalised(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "detect_entities", py_func("detect_entities", |args: &[Py]| detect_entities(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_said_as_a_place", py_func("_said_as_a_place", |args: &[Py]| _said_as_a_place(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_placed_by_a_word", py_func("_placed_by_a_word", |args: &[Py]| _placed_by_a_word(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_has_travel_word", py_func("_has_travel_word", |args: &[Py]| _has_travel_word(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_places_a_name", py_func("_places_a_name", |args: &[Py]| _places_a_name(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_a_country_named", py_func("_a_country_named", |args: &[Py]| _a_country_named(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_set_in_a_place", py_func("_set_in_a_place", |args: &[Py]| _set_in_a_place(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_asked_as_a_country", py_func("_asked_as_a_country", |args: &[Py]| _asked_as_a_country(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_names_a_person", py_func("_names_a_person", |args: &[Py]| _names_a_person(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "person_names", py_func("person_names", |args: &[Py]| person_names(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_names_a_place", py_func("_names_a_place", |args: &[Py]| _names_a_place(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_position", py_func("_position", |args: &[Py]| _position(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_thing", py_func("_thing", |args: &[Py]| _thing(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_spoken_numbers", py_func("_spoken_numbers", |args: &[Py]| _spoken_numbers(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_mixed_number", py_func("_mixed_number", |args: &[Py]| _mixed_number(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_the_figure_and_its_unit", py_func("_the_figure_and_its_unit", |args: &[Py]| _the_figure_and_its_unit(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "detect_numbers", py_func("detect_numbers", |args: &[Py]| detect_numbers(args)));
    pyrt::xrefs::reg_fn("understanding.topics", "_counts_as_number", py_func("_counts_as_number", |args: &[Py]| _counts_as_number(args)));
    pyrt::xrefs::reg_access("understanding.topics", "ADDRESS_WORDS", g_ADDRESS_WORDS, s_ADDRESS_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "AGE_AFTER", g_AGE_AFTER, s_AGE_AFTER);
    pyrt::xrefs::reg_access("understanding.topics", "AGE_IN_A_YEAR", g_AGE_IN_A_YEAR, s_AGE_IN_A_YEAR);
    pyrt::xrefs::reg_access("understanding.topics", "AGE_NOUNS", g_AGE_NOUNS, s_AGE_NOUNS);
    pyrt::xrefs::reg_access("understanding.topics", "AGE_OF_A_CHILD_WORDS", g_AGE_OF_A_CHILD_WORDS, s_AGE_OF_A_CHILD_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "ANIMALS", g_ANIMALS, s_ANIMALS);
    pyrt::xrefs::reg_access("understanding.topics", "ARTICLES", g_ARTICLES, s_ARTICLES);
    pyrt::xrefs::reg_access("understanding.topics", "ASK_PHRASES", g_ASK_PHRASES, s_ASK_PHRASES);
    pyrt::xrefs::reg_access("understanding.topics", "ASK_WORDS", g_ASK_WORDS, s_ASK_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "A_AMOUNT_NOUN", g_A_AMOUNT_NOUN, s_A_AMOUNT_NOUN);
    pyrt::xrefs::reg_access("understanding.topics", "A_AMOUNT_SYMBOL", g_A_AMOUNT_SYMBOL, s_A_AMOUNT_SYMBOL);
    pyrt::xrefs::reg_access("understanding.topics", "A_CHILD_TILL", g_A_CHILD_TILL, s_A_CHILD_TILL);
    pyrt::xrefs::reg_access("understanding.topics", "A_CHILD_TO_READ_FOR", g_A_CHILD_TO_READ_FOR, s_A_CHILD_TO_READ_FOR);
    pyrt::xrefs::reg_access("understanding.topics", "A_GUT_ONLY", g_A_GUT_ONLY, s_A_GUT_ONLY);
    pyrt::xrefs::reg_access("understanding.topics", "A_MONEY_WORD", g_A_MONEY_WORD, s_A_MONEY_WORD);
    pyrt::xrefs::reg_access("understanding.topics", "A_PERSON_WHO_WANTS", g_A_PERSON_WHO_WANTS, s_A_PERSON_WHO_WANTS);
    pyrt::xrefs::reg_access("understanding.topics", "A_PORTION_NEEDS_OF", g_A_PORTION_NEEDS_OF, s_A_PORTION_NEEDS_OF);
    pyrt::xrefs::reg_access("understanding.topics", "A_PORTION_UNITS", g_A_PORTION_UNITS, s_A_PORTION_UNITS);
    pyrt::xrefs::reg_access("understanding.topics", "A_THING_WANTED", g_A_THING_WANTED, s_A_THING_WANTED);
    pyrt::xrefs::reg_access("understanding.topics", "A_WAIT", g_A_WAIT, s_A_WAIT);
    pyrt::xrefs::reg_access("understanding.topics", "A_WAIT_TO", g_A_WAIT_TO, s_A_WAIT_TO);
    pyrt::xrefs::reg_access("understanding.topics", "BEHAVIOUR", g_BEHAVIOUR, s_BEHAVIOUR);
    pyrt::xrefs::reg_access("understanding.topics", "BIG_PURSE", g_BIG_PURSE, s_BIG_PURSE);
    pyrt::xrefs::reg_access("understanding.topics", "BODILY_OUTPUT", g_BODILY_OUTPUT, s_BODILY_OUTPUT);
    pyrt::xrefs::reg_access("understanding.topics", "BODILY_OUTPUT_NOUN", g_BODILY_OUTPUT_NOUN, s_BODILY_OUTPUT_NOUN);
    pyrt::xrefs::reg_access("understanding.topics", "BODY", g_BODY, s_BODY);
    pyrt::xrefs::reg_access("understanding.topics", "BODY_CUES", g_BODY_CUES, s_BODY_CUES);
    pyrt::xrefs::reg_access("understanding.topics", "BODY_OR_SELF", g_BODY_OR_SELF, s_BODY_OR_SELF);
    pyrt::xrefs::reg_access("understanding.topics", "BODY_TWO_LIFE", g_BODY_TWO_LIFE, s_BODY_TWO_LIFE);
    pyrt::xrefs::reg_access("understanding.topics", "BOOK_WORDS", g_BOOK_WORDS, s_BOOK_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "BROKE_AS_SUBJECT", g_BROKE_AS_SUBJECT, s_BROKE_AS_SUBJECT);
    pyrt::xrefs::reg_access("understanding.topics", "BROKE_AS_TIME", g_BROKE_AS_TIME, s_BROKE_AS_TIME);
    pyrt::xrefs::reg_access("understanding.topics", "CANCELS", g_CANCELS, s_CANCELS);
    pyrt::xrefs::reg_access("understanding.topics", "CANCEL_AFTER", g_CANCEL_AFTER, s_CANCEL_AFTER);
    pyrt::xrefs::reg_access("understanding.topics", "CARRY", g_CARRY, s_CARRY);
    pyrt::xrefs::reg_access("understanding.topics", "CAR_NAMES", g_CAR_NAMES, s_CAR_NAMES);
    pyrt::xrefs::reg_access("understanding.topics", "CLIPPED_AFTER_A_SLASH", g_CLIPPED_AFTER_A_SLASH, s_CLIPPED_AFTER_A_SLASH);
    pyrt::xrefs::reg_access("understanding.topics", "CLIPPED_UNIT_WORDS", g_CLIPPED_UNIT_WORDS, s_CLIPPED_UNIT_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "COLD_PEOPLE", g_COLD_PEOPLE, s_COLD_PEOPLE);
    pyrt::xrefs::reg_access("understanding.topics", "COLD_WORDS", g_COLD_WORDS, s_COLD_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "COMMON_WORDS", g_COMMON_WORDS, s_COMMON_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "COMMS", g_COMMS, s_COMMS);
    pyrt::xrefs::reg_access("understanding.topics", "COMPARE_WORDS", g_COMPARE_WORDS, s_COMPARE_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "COMPLAINT_OF", g_COMPLAINT_OF, s_COMPLAINT_OF);
    pyrt::xrefs::reg_access("understanding.topics", "COMPLAINT_OF_NEXT", g_COMPLAINT_OF_NEXT, s_COMPLAINT_OF_NEXT);
    pyrt::xrefs::reg_access("understanding.topics", "CONDITIONS", g_CONDITIONS, s_CONDITIONS);
    pyrt::xrefs::reg_access("understanding.topics", "CONDITION_BOOST", g_CONDITION_BOOST, s_CONDITION_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "CONDITION_NEEDS_PART", g_CONDITION_NEEDS_PART, s_CONDITION_NEEDS_PART);
    pyrt::xrefs::reg_access("understanding.topics", "CONDITION_NEEDS_WORD", g_CONDITION_NEEDS_WORD, s_CONDITION_NEEDS_WORD);
    pyrt::xrefs::reg_access("understanding.topics", "CONDITION_NOT_NEXT", g_CONDITION_NOT_NEXT, s_CONDITION_NOT_NEXT);
    pyrt::xrefs::reg_access("understanding.topics", "CONDITION_OTHER_LIFE", g_CONDITION_OTHER_LIFE, s_CONDITION_OTHER_LIFE);
    pyrt::xrefs::reg_access("understanding.topics", "CONDITION_REACH", g_CONDITION_REACH, s_CONDITION_REACH);
    pyrt::xrefs::reg_access("understanding.topics", "COUNTRY_BOOST", g_COUNTRY_BOOST, s_COUNTRY_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "COUNTRY_CUES", g_COUNTRY_CUES, s_COUNTRY_CUES);
    pyrt::xrefs::reg_access("understanding.topics", "COUNTRY_HOMOGRAPHS", g_COUNTRY_HOMOGRAPHS, s_COUNTRY_HOMOGRAPHS);
    pyrt::xrefs::reg_access("understanding.topics", "CROPS", g_CROPS, s_CROPS);
    pyrt::xrefs::reg_access("understanding.topics", "CROP_BOOST", g_CROP_BOOST, s_CROP_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "CROP_KITCHEN", g_CROP_KITCHEN, s_CROP_KITCHEN);
    pyrt::xrefs::reg_access("understanding.topics", "CROP_STATES", g_CROP_STATES, s_CROP_STATES);
    pyrt::xrefs::reg_access("understanding.topics", "CUE_CARRIERS", g_CUE_CARRIERS, s_CUE_CARRIERS);
    pyrt::xrefs::reg_access("understanding.topics", "DEGREE_NOUNS", g_DEGREE_NOUNS, s_DEGREE_NOUNS);
    pyrt::xrefs::reg_access("understanding.topics", "DERIVED_ENDINGS", g_DERIVED_ENDINGS, s_DERIVED_ENDINGS);
    pyrt::xrefs::reg_access("understanding.topics", "DETERMINERS", g_DETERMINERS, s_DETERMINERS);
    pyrt::xrefs::reg_access("understanding.topics", "DRIVEN_JOINERS", g_DRIVEN_JOINERS, s_DRIVEN_JOINERS);
    pyrt::xrefs::reg_access("understanding.topics", "DRIVEN_MOOD", g_DRIVEN_MOOD, s_DRIVEN_MOOD);
    pyrt::xrefs::reg_access("understanding.topics", "DRIVEN_PERSON", g_DRIVEN_PERSON, s_DRIVEN_PERSON);
    pyrt::xrefs::reg_access("understanding.topics", "DRIVEN_TAIL", g_DRIVEN_TAIL, s_DRIVEN_TAIL);
    pyrt::xrefs::reg_access("understanding.topics", "DRIVES", g_DRIVES, s_DRIVES);
    pyrt::xrefs::reg_access("understanding.topics", "EXERCISE_AFTER", g_EXERCISE_AFTER, s_EXERCISE_AFTER);
    pyrt::xrefs::reg_access("understanding.topics", "EXERCISE_NOUNS", g_EXERCISE_NOUNS, s_EXERCISE_NOUNS);
    pyrt::xrefs::reg_access("understanding.topics", "EXTRA_PEOPLE", g_EXTRA_PEOPLE, s_EXTRA_PEOPLE);
    pyrt::xrefs::reg_access("understanding.topics", "FAMILY_PAIRS", g_FAMILY_PAIRS, s_FAMILY_PAIRS);
    pyrt::xrefs::reg_access("understanding.topics", "FAMILY_STEM", g_FAMILY_STEM, s_FAMILY_STEM);
    pyrt::xrefs::reg_access("understanding.topics", "FAMILY_WORD", g_FAMILY_WORD, s_FAMILY_WORD);
    pyrt::xrefs::reg_access("understanding.topics", "FIGURE_SIZE_WORDS", g_FIGURE_SIZE_WORDS, s_FIGURE_SIZE_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "FRIEND_BOOST", g_FRIEND_BOOST, s_FRIEND_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "FRIEND_WORDS", g_FRIEND_WORDS, s_FRIEND_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "GENERIC", g_GENERIC, s_GENERIC);
    pyrt::xrefs::reg_access("understanding.topics", "GIFTS_OR_OCCASIONS", g_GIFTS_OR_OCCASIONS, s_GIFTS_OR_OCCASIONS);
    pyrt::xrefs::reg_access("understanding.topics", "GIFT_ALWAYS", g_GIFT_ALWAYS, s_GIFT_ALWAYS);
    pyrt::xrefs::reg_access("understanding.topics", "GIFT_ASKING", g_GIFT_ASKING, s_GIFT_ASKING);
    pyrt::xrefs::reg_access("understanding.topics", "GIFT_BOOST", g_GIFT_BOOST, s_GIFT_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "GIFT_OCCASION", g_GIFT_OCCASION, s_GIFT_OCCASION);
    pyrt::xrefs::reg_access("understanding.topics", "GIFT_WANTING", g_GIFT_WANTING, s_GIFT_WANTING);
    pyrt::xrefs::reg_access("understanding.topics", "GIFT_WORDS", g_GIFT_WORDS, s_GIFT_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "GIVEN_UP_FOOD", g_GIVEN_UP_FOOD, s_GIVEN_UP_FOOD);
    pyrt::xrefs::reg_access("understanding.topics", "GIVE_DETERMINERS", g_GIVE_DETERMINERS, s_GIVE_DETERMINERS);
    pyrt::xrefs::reg_access("understanding.topics", "GIVE_RELATIVE", g_GIVE_RELATIVE, s_GIVE_RELATIVE);
    pyrt::xrefs::reg_access("understanding.topics", "GIVE_THING", g_GIVE_THING, s_GIVE_THING);
    pyrt::xrefs::reg_access("understanding.topics", "GIVE_THINGS", g_GIVE_THINGS, s_GIVE_THINGS);
    pyrt::xrefs::reg_access("understanding.topics", "GIVE_TO", g_GIVE_TO, s_GIVE_TO);
    pyrt::xrefs::reg_access("understanding.topics", "GIVE_VERB", g_GIVE_VERB, s_GIVE_VERB);
    pyrt::xrefs::reg_access("understanding.topics", "GIVING_UP", g_GIVING_UP, s_GIVING_UP);
    pyrt::xrefs::reg_access("understanding.topics", "GIVING_UP_BOOST", g_GIVING_UP_BOOST, s_GIVING_UP_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "GROWING_PLACES", g_GROWING_PLACES, s_GROWING_PLACES);
    pyrt::xrefs::reg_access("understanding.topics", "HEAD_FOG", g_HEAD_FOG, s_HEAD_FOG);
    pyrt::xrefs::reg_access("understanding.topics", "HEAD_FOG_BOOST", g_HEAD_FOG_BOOST, s_HEAD_FOG_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "HEAD_FOG_REACH", g_HEAD_FOG_REACH, s_HEAD_FOG_REACH);
    pyrt::xrefs::reg_access("understanding.topics", "HEAD_WORDS", g_HEAD_WORDS, s_HEAD_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "HEARERS", g_HEARERS, s_HEARERS);
    pyrt::xrefs::reg_access("understanding.topics", "HOT_WORDS", g_HOT_WORDS, s_HOT_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "HURTS", g_HURTS, s_HURTS);
    pyrt::xrefs::reg_access("understanding.topics", "HURT_BOOST", g_HURT_BOOST, s_HURT_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "IDIOM_PHRASES", g_IDIOM_PHRASES, s_IDIOM_PHRASES);
    pyrt::xrefs::reg_access("understanding.topics", "JUDGEMENT_OF_RECORD", g_JUDGEMENT_OF_RECORD, s_JUDGEMENT_OF_RECORD);
    pyrt::xrefs::reg_access("understanding.topics", "LEADING", g_LEADING, s_LEADING);
    pyrt::xrefs::reg_access("understanding.topics", "LIVES", g_LIVES, s_LIVES);
    pyrt::xrefs::reg_access("understanding.topics", "MACHINE_BIN_WORDS", g_MACHINE_BIN_WORDS, s_MACHINE_BIN_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "MACHINE_WORD_CUES", g_MACHINE_WORD_CUES, s_MACHINE_WORD_CUES);
    pyrt::xrefs::reg_access("understanding.topics", "MACHINE_WORD_PHRASES", g_MACHINE_WORD_PHRASES, s_MACHINE_WORD_PHRASES);
    pyrt::xrefs::reg_access("understanding.topics", "MECHANISM", g_MECHANISM, s_MECHANISM);
    pyrt::xrefs::reg_access("understanding.topics", "MECHANISM_HOW", g_MECHANISM_HOW, s_MECHANISM_HOW);
    pyrt::xrefs::reg_access("understanding.topics", "MIXED_FRACTION", g_MIXED_FRACTION, s_MIXED_FRACTION);
    pyrt::xrefs::reg_access("understanding.topics", "MIXED_JOINERS", g_MIXED_JOINERS, s_MIXED_JOINERS);
    pyrt::xrefs::reg_access("understanding.topics", "MONEY_TIGHT", g_MONEY_TIGHT, s_MONEY_TIGHT);
    pyrt::xrefs::reg_access("understanding.topics", "MONEY_TIGHT_MAX", g_MONEY_TIGHT_MAX, s_MONEY_TIGHT_MAX);
    pyrt::xrefs::reg_access("understanding.topics", "NAME_BREAKS", g_NAME_BREAKS, s_NAME_BREAKS);
    pyrt::xrefs::reg_access("understanding.topics", "NAME_FRAMES", g_NAME_FRAMES, s_NAME_FRAMES);
    pyrt::xrefs::reg_access("understanding.topics", "NATURE_WORDS", g_NATURE_WORDS, s_NATURE_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "NOSE_BLOCKED", g_NOSE_BLOCKED, s_NOSE_BLOCKED);
    pyrt::xrefs::reg_access("understanding.topics", "NOSE_BLOCKED_BOOST", g_NOSE_BLOCKED_BOOST, s_NOSE_BLOCKED_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "NOT_CUT_OUT", g_NOT_CUT_OUT, s_NOT_CUT_OUT);
    pyrt::xrefs::reg_access("understanding.topics", "NOT_PLACES", g_NOT_PLACES, s_NOT_PLACES);
    pyrt::xrefs::reg_access("understanding.topics", "NOT_THINGS", g_NOT_THINGS, s_NOT_THINGS);
    pyrt::xrefs::reg_access("understanding.topics", "NOT_WORDS", g_NOT_WORDS, s_NOT_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "NOUNY_SIGNS", g_NOUNY_SIGNS, s_NOUNY_SIGNS);
    pyrt::xrefs::reg_access("understanding.topics", "NUMBER_FILLER", g_NUMBER_FILLER, s_NUMBER_FILLER);
    pyrt::xrefs::reg_access("understanding.topics", "NUMBER_RE", g_NUMBER_RE, s_NUMBER_RE);
    pyrt::xrefs::reg_access("understanding.topics", "NUMBER_WORDS", g_NUMBER_WORDS, s_NUMBER_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "ONE_IS_A_THING", g_ONE_IS_A_THING, s_ONE_IS_A_THING);
    pyrt::xrefs::reg_access("understanding.topics", "ONLY_A_STOOL", g_ONLY_A_STOOL, s_ONLY_A_STOOL);
    pyrt::xrefs::reg_access("understanding.topics", "ORDINARY_VERBS", g_ORDINARY_VERBS, s_ORDINARY_VERBS);
    pyrt::xrefs::reg_access("understanding.topics", "OTHER_PERSON", g_OTHER_PERSON, s_OTHER_PERSON);
    pyrt::xrefs::reg_access("understanding.topics", "OWNED_BY", g_OWNED_BY, s_OWNED_BY);
    pyrt::xrefs::reg_access("understanding.topics", "OWNERS", g_OWNERS, s_OWNERS);
    pyrt::xrefs::reg_access("understanding.topics", "OWNING_WORDS", g_OWNING_WORDS, s_OWNING_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "OWNS_THE_UNIT", g_OWNS_THE_UNIT, s_OWNS_THE_UNIT);
    pyrt::xrefs::reg_access("understanding.topics", "OWN_SELF", g_OWN_SELF, s_OWN_SELF);
    pyrt::xrefs::reg_access("understanding.topics", "PARTS", g_PARTS, s_PARTS);
    pyrt::xrefs::reg_access("understanding.topics", "PART_UP_BOOST", g_PART_UP_BOOST, s_PART_UP_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "PEOPLE_TOPICS", g_PEOPLE_TOPICS, s_PEOPLE_TOPICS);
    pyrt::xrefs::reg_access("understanding.topics", "PERSON_WORDS", g_PERSON_WORDS, s_PERSON_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "PETS", g_PETS, s_PETS);
    pyrt::xrefs::reg_access("understanding.topics", "PET_BOOST", g_PET_BOOST, s_PET_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "PHRASES", g_PHRASES, s_PHRASES);
    pyrt::xrefs::reg_access("understanding.topics", "PLACES", g_PLACES, s_PLACES);
    pyrt::xrefs::reg_access("understanding.topics", "PLACE_HOMOGRAPHS", g_PLACE_HOMOGRAPHS, s_PLACE_HOMOGRAPHS);
    pyrt::xrefs::reg_access("understanding.topics", "PLACE_OVER", g_PLACE_OVER, s_PLACE_OVER);
    pyrt::xrefs::reg_access("understanding.topics", "PLACE_THING", g_PLACE_THING, s_PLACE_THING);
    pyrt::xrefs::reg_access("understanding.topics", "PLACE_WORDS", g_PLACE_WORDS, s_PLACE_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "PLACE_WORDS_WITH_CUE", g_PLACE_WORDS_WITH_CUE, s_PLACE_WORDS_WITH_CUE);
    pyrt::xrefs::reg_access("understanding.topics", "PLAYS_UP_BEFORE", g_PLAYS_UP_BEFORE, s_PLAYS_UP_BEFORE);
    pyrt::xrefs::reg_access("understanding.topics", "PLAY_UP_REACH", g_PLAY_UP_REACH, s_PLAY_UP_REACH);
    pyrt::xrefs::reg_access("understanding.topics", "POLYSEMOUS", g_POLYSEMOUS, s_POLYSEMOUS);
    pyrt::xrefs::reg_access("understanding.topics", "POLY_FORMS", g_POLY_FORMS, s_POLY_FORMS);
    pyrt::xrefs::reg_access("understanding.topics", "PORTION_UNITS", g_PORTION_UNITS, s_PORTION_UNITS);
    pyrt::xrefs::reg_access("understanding.topics", "PORTION_WORDS", g_PORTION_WORDS, s_PORTION_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "RELATIONS", g_RELATIONS, s_RELATIONS);
    pyrt::xrefs::reg_access("understanding.topics", "RUN_OUT_BEFORE", g_RUN_OUT_BEFORE, s_RUN_OUT_BEFORE);
    pyrt::xrefs::reg_access("understanding.topics", "SAMPLE_WORDS", g_SAMPLE_WORDS, s_SAMPLE_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "SCIENCE_BOOST", g_SCIENCE_BOOST, s_SCIENCE_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "SEASONS", g_SEASONS, s_SEASONS);
    pyrt::xrefs::reg_access("understanding.topics", "SENSE_ELSEWHERE", g_SENSE_ELSEWHERE, s_SENSE_ELSEWHERE);
    pyrt::xrefs::reg_access("understanding.topics", "SENSE_ELSEWHERE_FORMS", g_SENSE_ELSEWHERE_FORMS, s_SENSE_ELSEWHERE_FORMS);
    pyrt::xrefs::reg_access("understanding.topics", "SENSE_NEAR", g_SENSE_NEAR, s_SENSE_NEAR);
    pyrt::xrefs::reg_access("understanding.topics", "SENSE_PAIRS", g_SENSE_PAIRS, s_SENSE_PAIRS);
    pyrt::xrefs::reg_access("understanding.topics", "SENSE_REACH", g_SENSE_REACH, s_SENSE_REACH);
    pyrt::xrefs::reg_access("understanding.topics", "SENSE_WEIGHT", g_SENSE_WEIGHT, s_SENSE_WEIGHT);
    pyrt::xrefs::reg_access("understanding.topics", "SIGN_FOLLOWS", g_SIGN_FOLLOWS, s_SIGN_FOLLOWS);
    pyrt::xrefs::reg_access("understanding.topics", "SIGN_NOT_AFTER", g_SIGN_NOT_AFTER, s_SIGN_NOT_AFTER);
    pyrt::xrefs::reg_access("understanding.topics", "SKIN_BOOST", g_SKIN_BOOST, s_SKIN_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "SKIN_PARTS", g_SKIN_PARTS, s_SKIN_PARTS);
    pyrt::xrefs::reg_access("understanding.topics", "SKIN_REACH", g_SKIN_REACH, s_SKIN_REACH);
    pyrt::xrefs::reg_access("understanding.topics", "SKIN_STATES", g_SKIN_STATES, s_SKIN_STATES);
    pyrt::xrefs::reg_access("understanding.topics", "STONE_COMPOUNDS", g_STONE_COMPOUNDS, s_STONE_COMPOUNDS);
    pyrt::xrefs::reg_access("understanding.topics", "STONE_VERBS", g_STONE_VERBS, s_STONE_VERBS);
    pyrt::xrefs::reg_access("understanding.topics", "STOOL_AS_FURNITURE", g_STOOL_AS_FURNITURE, s_STOOL_AS_FURNITURE);
    pyrt::xrefs::reg_access("understanding.topics", "STRONG", g_STRONG, s_STRONG);
    pyrt::xrefs::reg_access("understanding.topics", "STUDY_OBJECTS", g_STUDY_OBJECTS, s_STUDY_OBJECTS);
    pyrt::xrefs::reg_access("understanding.topics", "SUMS_AFTER", g_SUMS_AFTER, s_SUMS_AFTER);
    pyrt::xrefs::reg_access("understanding.topics", "SUM_MONEY", g_SUM_MONEY, s_SUM_MONEY);
    pyrt::xrefs::reg_access("understanding.topics", "TABLE", g_TABLE, s_TABLE);
    pyrt::xrefs::reg_access("understanding.topics", "TEETHING_AND_A_CHILD", g_TEETHING_AND_A_CHILD, s_TEETHING_AND_A_CHILD);
    pyrt::xrefs::reg_access("understanding.topics", "TEETHING_IS_A_FAULT", g_TEETHING_IS_A_FAULT, s_TEETHING_IS_A_FAULT);
    pyrt::xrefs::reg_access("understanding.topics", "THE_PEOPLE", g_THE_PEOPLE, s_THE_PEOPLE);
    pyrt::xrefs::reg_access("understanding.topics", "THE_PERSONS_OWN", g_THE_PERSONS_OWN, s_THE_PERSONS_OWN);
    pyrt::xrefs::reg_access("understanding.topics", "THINGS", g_THINGS, s_THINGS);
    pyrt::xrefs::reg_access("understanding.topics", "THING_WORDS", g_THING_WORDS, s_THING_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "TRAVEL_CUES", g_TRAVEL_CUES, s_TRAVEL_CUES);
    pyrt::xrefs::reg_access("understanding.topics", "TRAVEL_STRONG", g_TRAVEL_STRONG, s_TRAVEL_STRONG);
    pyrt::xrefs::reg_access("understanding.topics", "TWO_HOMES", g_TWO_HOMES, s_TWO_HOMES);
    pyrt::xrefs::reg_access("understanding.topics", "TWO_LIVES", g_TWO_LIVES, s_TWO_LIVES);
    pyrt::xrefs::reg_access("understanding.topics", "TWO_LIVES_FORMS", g_TWO_LIVES_FORMS, s_TWO_LIVES_FORMS);
    pyrt::xrefs::reg_access("understanding.topics", "TWO_LIVES_REACH", g_TWO_LIVES_REACH, s_TWO_LIVES_REACH);
    pyrt::xrefs::reg_access("understanding.topics", "UNIT_WORDS", g_UNIT_WORDS, s_UNIT_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "UNWELL_BOOST", g_UNWELL_BOOST, s_UNWELL_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "UNWELL_IDIOMS", g_UNWELL_IDIOMS, s_UNWELL_IDIOMS);
    pyrt::xrefs::reg_access("understanding.topics", "UNWELL_NOT", g_UNWELL_NOT, s_UNWELL_NOT);
    pyrt::xrefs::reg_access("understanding.topics", "UNWELL_SIGNS", g_UNWELL_SIGNS, s_UNWELL_SIGNS);
    pyrt::xrefs::reg_access("understanding.topics", "VERB_CUES", g_VERB_CUES, s_VERB_CUES);
    pyrt::xrefs::reg_access("understanding.topics", "VERDICT_NOUNS", g_VERDICT_NOUNS, s_VERDICT_NOUNS);
    pyrt::xrefs::reg_access("understanding.topics", "WANTED_BEFORE_A_STOP", g_WANTED_BEFORE_A_STOP, s_WANTED_BEFORE_A_STOP);
    pyrt::xrefs::reg_access("understanding.topics", "WANT_VERBS", g_WANT_VERBS, s_WANT_VERBS);
    pyrt::xrefs::reg_access("understanding.topics", "WARMTH_ARE", g_WARMTH_ARE, s_WARMTH_ARE);
    pyrt::xrefs::reg_access("understanding.topics", "WARMTH_DEGREE", g_WARMTH_DEGREE, s_WARMTH_DEGREE);
    pyrt::xrefs::reg_access("understanding.topics", "WARMTH_HABITUAL", g_WARMTH_HABITUAL, s_WARMTH_HABITUAL);
    pyrt::xrefs::reg_access("understanding.topics", "WARMTH_THEN", g_WARMTH_THEN, s_WARMTH_THEN);
    pyrt::xrefs::reg_access("understanding.topics", "WARMTH_WORDS", g_WARMTH_WORDS, s_WARMTH_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "WATCHING", g_WATCHING, s_WATCHING);
    pyrt::xrefs::reg_access("understanding.topics", "WATCH_OUT_AFTER", g_WATCH_OUT_AFTER, s_WATCH_OUT_AFTER);
    pyrt::xrefs::reg_access("understanding.topics", "WEAK", g_WEAK, s_WEAK);
    pyrt::xrefs::reg_access("understanding.topics", "WEATHER_BEFORE", g_WEATHER_BEFORE, s_WEATHER_BEFORE);
    pyrt::xrefs::reg_access("understanding.topics", "WEATHER_ITSELF", g_WEATHER_ITSELF, s_WEATHER_ITSELF);
    pyrt::xrefs::reg_access("understanding.topics", "WORD_DIGITS", g_WORD_DIGITS, s_WORD_DIGITS);
    pyrt::xrefs::reg_access("understanding.topics", "WORD_NUMBERS", g_WORD_NUMBERS, s_WORD_NUMBERS);
    pyrt::xrefs::reg_access("understanding.topics", "WORK_WORDS", g_WORK_WORDS, s_WORK_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "WORLD", g_WORLD, s_WORLD);
    pyrt::xrefs::reg_access("understanding.topics", "WORLD_NOT", g_WORLD_NOT, s_WORLD_NOT);
    pyrt::xrefs::reg_access("understanding.topics", "WORN_OUT", g_WORN_OUT, s_WORN_OUT);
    pyrt::xrefs::reg_access("understanding.topics", "WORN_OUT_BOOST", g_WORN_OUT_BOOST, s_WORN_OUT_BOOST);
    pyrt::xrefs::reg_access("understanding.topics", "WORN_OUT_CUE", g_WORN_OUT_CUE, s_WORN_OUT_CUE);
    pyrt::xrefs::reg_access("understanding.topics", "WORN_OUT_LOOKBACK", g_WORN_OUT_LOOKBACK, s_WORN_OUT_LOOKBACK);
    pyrt::xrefs::reg_access("understanding.topics", "WORN_OUT_SAID", g_WORN_OUT_SAID, s_WORN_OUT_SAID);
    pyrt::xrefs::reg_access("understanding.topics", "_A_STATE_VERB", g__A_STATE_VERB, s__A_STATE_VERB);
    pyrt::xrefs::reg_access("understanding.topics", "_BREAKS", g__BREAKS, s__BREAKS);
    pyrt::xrefs::reg_access("understanding.topics", "_FUNCTION_WORDS", g__FUNCTION_WORDS, s__FUNCTION_WORDS);
    pyrt::xrefs::reg_access("understanding.topics", "_GRAND_RELATIONS", g__GRAND_RELATIONS, s__GRAND_RELATIONS);
    pyrt::xrefs::reg_access("understanding.topics", "_KIN_BUILT", g__KIN_BUILT, s__KIN_BUILT);
    pyrt::xrefs::reg_access("understanding.topics", "_NOT_THE_SUBJECT", g__NOT_THE_SUBJECT, s__NOT_THE_SUBJECT);
    pyrt::xrefs::reg_access("understanding.topics", "_POSSESSIVE", g__POSSESSIVE, s__POSSESSIVE);
    pyrt::xrefs::reg_access("understanding.topics", "_SENSE_TIERS", g__SENSE_TIERS, s__SENSE_TIERS);
    pyrt::xrefs::reg_access("understanding.topics", "_STEPPED_RELATIONS", g__STEPPED_RELATIONS, s__STEPPED_RELATIONS);
    pyrt::xrefs::reg_access("understanding.topics", "_THE_OWNER_SETTLES", g__THE_OWNER_SETTLES, s__THE_OWNER_SETTLES);
    pyrt::xrefs::reg_access("understanding.topics", "_form", g__form, s__form);
    pyrt::xrefs::reg_access("understanding.topics", "_forms", g__forms, s__forms);
    pyrt::xrefs::reg_access("understanding.topics", "_sense_topic", g__sense_topic, s__sense_topic);
    pyrt::xrefs::reg_access("understanding.topics", "_tiers", g__tiers, s__tiers);
}
