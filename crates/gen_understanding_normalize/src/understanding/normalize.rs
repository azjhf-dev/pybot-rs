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


thread_local! { static G_ABOUT_THE_FORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AEIOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_AN_INVERTED_AUX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AINT_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AINT_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AMOUNT_BEFORE_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOSTROPHE_BASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOSTROPHE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APOSTROPHE_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_WHOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AUX_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AUX_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHOKE_SAID_THE_OTHER_WAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FEELING_CAN_BE_FOLLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GLASS_OR_A_PAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_KEEPING_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LABEL_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LETTER_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEGATION_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NUMBER_OF_SOMEONES_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PAIN_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PARTS_OF_A_PERIOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PART_A_PERSON_OWNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PART_THE_PAIN_IS_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERIOD_FOLLOWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRONOUN_GLUED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRONOUN_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE_THEN_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATE_YOU_CAN_FEEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_GOES_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_TYPED_AS_A_LITTLE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_WORN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_TO_LEAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERB_CAN_FOLLOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORD_CAN_FOLLOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEEN_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_A_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_A_PREPOSITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_THE_LOSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BE_BEFORE_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BE_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIN_CONTAINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BREAKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAN_FOLLOW_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAN_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSES_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSE_A_CLAUSE_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSE_A_DEMONSTRATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSE_A_SUBJECT_FOLLOWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSE_BEFORE_A_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAUSE_BEFORE_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_OF_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_AS_ONE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_KEPT_AS_TYPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLIPPED_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLIPPED_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLIPPED_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMMON_RANK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPOUND_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPOUND_TAILS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRACTION_NEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COPULAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRENCY_FACTOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRENCY_SIGN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CURRENCY_SIGNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAY_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEMONSTRATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINER_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIGIT_AS_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIGIT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOCS_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOCS_PAPER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOCS_SCREEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOCS_VISIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOING_NOT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DROPPED_SUBJECT_AUX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXCLAIMED_AFTER_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXCLAMATION_OPENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXCLAMATION_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_E_KEPT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_SHRT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEELING_BEFORE_SICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FINDING_A_TEAM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON_BE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_SLOT_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_A: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOR_OBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQUENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_F_TYPED_FOR_TH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLUED_AFTER_A_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLUED_COLLAPSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLUED_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLUED_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLUED_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAS_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAS_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAS_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAS_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEART_FEELINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEART_MEASURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEART_SAID_AS_A_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELLO_SAID_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HELLO_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_DEGREE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_FOLLOWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HR_IS_THE_DEPARTMENT_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HV_HELPERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HV_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ILL_GOES_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ING_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTRODUCING_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTRODUCING_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IRREGULAR_PARTICIPLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_I_GLUED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINED_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_FILLERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_HAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN_SINGULAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KNOW_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OFF_CONTRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LETTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LETTER_C_IS_SEE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEXICON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIKE_STARTS_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG_ENOUGH_TO_BE_A_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOSE_FOR_LOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ME_FOR_MY_CLOSES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSING_E_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MI_BACK_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MI_IS_A_NAME_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MI_IS_ME_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MI_IS_ME_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MODAL_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOTION_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MRS_IS_THE_WIFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_LETTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_CALLS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_ROLL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOMINAL_BEFORE_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_COMPOUND_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_COMPOUND_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_MEAL_TYPO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_ENDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_PAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_BEFORE_A_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_BEFORE_KNOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_COUNTED_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_IN_A_QUESTION_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MY_OWN_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_ASK_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_SWAP_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBERS_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBJECT_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_OF_A_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_PART_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_RELATION_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_WORD_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPERATORS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNER_WITHOUT_THE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNING_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWNING_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_REACH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_THAT_IS_BAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_AS_PARTICIPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERIOD_COUNTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_PREPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PHRASE_MEASURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_BASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PUNCT_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RAW_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REASON_STARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATIVE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_ONCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPEAT_ONCE_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_ON_LAST_PIECES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_ON_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_ON_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_ON_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_ON_SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_ON_SELF_LONGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_ON_SHORTEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RUN_ON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_APART_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_AS_A_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHRT_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SICK_AS_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOLFEGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SON_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORE_FILLERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORE_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SORE_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPELLING_ENDINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPLIT_CONTRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPLIT_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATES_OF_A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOPPED_TH_STEMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUGAR_AFTER_ROSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUGAR_FIRST_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUGAR_LEVEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUGAR_PRICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUGAR_PRICE_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUM_STAYS_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUM_THE_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_S_CLUSTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_GUARD_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_PLEASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_QUESTION_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_THANKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAG_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_OF_A_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HEART_SAID_THE_OTHER_WAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_LOSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ODD_PLURALS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_JOIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_PLACE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_PLACE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_POSSESSIVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_WORDS_BY_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD_PERSON_BE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THOUSANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THOUSANDS_SAID_WITH_A_SPACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIMES_A_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_AFTER_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_A: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_A_GRADED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TO_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_TOO_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_TO_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_TO_SPELLINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_TO_TAGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_URGENT_BONES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_URGENT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEGGIES_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VOWELS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VOWEL_SET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VOWEL_SET_OR_Y: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEARS_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_ARE_YOU_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_ARE_YOU_SPELLINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WH_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WH_INTENSIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WH_INTENSIFIER_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WH_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WH_QUANTIFIERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YOU_ARE_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_A_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_A_LEAD_IN_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_PARTICIPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANY_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLIPPED_HEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLOSING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HEAD_IN_THE_IDIOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_QUESTION_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SHORT_WORD_IS_NAMED_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SHORT_WORD_IS_NAMED_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STATE_OF_THE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WORD_THE_BOT_WRITES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EXCLAMATION_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEELING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HAVE_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HAVE_NEGATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HAVE_PARTICIPLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HAVE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INTENSIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEPING_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEPT_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEPT_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEADER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACES_SAID_WHOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACE_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACE_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREDICATE_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SMALL_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPACED_PIECE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPACED_RUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECTLESS_AUX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECTLESS_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TYPED_SURPRISE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WERE_ARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WERE_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WERE_IN_THE_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WERE_LEAD_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WERE_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WERE_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WH_QUESTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT understanding.normalize: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_THE_FORM() -> Py { ensure_init(); G_ABOUT_THE_FORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_FORM(v: Py) { ensure_init(); G_ABOUT_THE_FORM.with(|c| *c.borrow_mut() = v); }
pub fn g_AEIOU() -> Py { ensure_init(); G_AEIOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AEIOU(v: Py) { ensure_init(); G_AEIOU.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_AN_INVERTED_AUX() -> Py { ensure_init(); G_AFTER_AN_INVERTED_AUX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_AN_INVERTED_AUX(v: Py) { ensure_init(); G_AFTER_AN_INVERTED_AUX.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_A_STATE() -> Py { ensure_init(); G_AFTER_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_A_STATE(v: Py) { ensure_init(); G_AFTER_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_A_THING() -> Py { ensure_init(); G_AFTER_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_A_THING(v: Py) { ensure_init(); G_AFTER_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_A_TIME() -> Py { ensure_init(); G_AFTER_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_A_TIME(v: Py) { ensure_init(); G_AFTER_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_A_VERB() -> Py { ensure_init(); G_AFTER_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_A_VERB(v: Py) { ensure_init(); G_AFTER_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_AINT_FORMS() -> Py { ensure_init(); G_AINT_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AINT_FORMS(v: Py) { ensure_init(); G_AINT_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_AINT_PLURAL() -> Py { ensure_init(); G_AINT_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AINT_PLURAL(v: Py) { ensure_init(); G_AINT_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g_AMOUNT_BEFORE_A_STATE() -> Py { ensure_init(); G_AMOUNT_BEFORE_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_BEFORE_A_STATE(v: Py) { ensure_init(); G_AMOUNT_BEFORE_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_APOSTROPHE_BASES() -> Py { ensure_init(); G_APOSTROPHE_BASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOSTROPHE_BASES(v: Py) { ensure_init(); G_APOSTROPHE_BASES.with(|c| *c.borrow_mut() = v); }
pub fn g_APOSTROPHE_NOT() -> Py { ensure_init(); G_APOSTROPHE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOSTROPHE_NOT(v: Py) { ensure_init(); G_APOSTROPHE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_APOSTROPHE_TAILS() -> Py { ensure_init(); G_APOSTROPHE_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APOSTROPHE_TAILS(v: Py) { ensure_init(); G_APOSTROPHE_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_WHOSE() -> Py { ensure_init(); G_ASKS_WHOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_WHOSE(v: Py) { ensure_init(); G_ASKS_WHOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_NOUNS() -> Py { ensure_init(); G_ASK_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_NOUNS(v: Py) { ensure_init(); G_ASK_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WORDS() -> Py { ensure_init(); G_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); G_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AUX_OPENS() -> Py { ensure_init(); G_AUX_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AUX_OPENS(v: Py) { ensure_init(); G_AUX_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_AUX_WORDS() -> Py { ensure_init(); G_AUX_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AUX_WORDS(v: Py) { ensure_init(); G_AUX_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHOKE_SAID_THE_OTHER_WAY() -> Py { ensure_init(); G_A_CHOKE_SAID_THE_OTHER_WAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHOKE_SAID_THE_OTHER_WAY(v: Py) { ensure_init(); G_A_CHOKE_SAID_THE_OTHER_WAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COPULA() -> Py { ensure_init(); G_A_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COPULA(v: Py) { ensure_init(); G_A_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT() -> Py { ensure_init(); G_A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT(v: Py) { ensure_init(); G_A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FEELING_CAN_BE_FOLLOWED() -> Py { ensure_init(); G_A_FEELING_CAN_BE_FOLLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FEELING_CAN_BE_FOLLOWED(v: Py) { ensure_init(); G_A_FEELING_CAN_BE_FOLLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GLASS_OR_A_PAN() -> Py { ensure_init(); G_A_GLASS_OR_A_PAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GLASS_OR_A_PAN(v: Py) { ensure_init(); G_A_GLASS_OR_A_PAN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB() -> Py { ensure_init(); G_A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB(v: Py) { ensure_init(); G_A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_KEEPING_A_THING() -> Py { ensure_init(); G_A_KEEPING_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_KEEPING_A_THING(v: Py) { ensure_init(); G_A_KEEPING_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LABEL_BEFORE() -> Py { ensure_init(); G_A_LABEL_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LABEL_BEFORE(v: Py) { ensure_init(); G_A_LABEL_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LETTER_AFTER() -> Py { ensure_init(); G_A_LETTER_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LETTER_AFTER(v: Py) { ensure_init(); G_A_LETTER_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEGATION_BEFORE() -> Py { ensure_init(); G_A_NEGATION_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEGATION_BEFORE(v: Py) { ensure_init(); G_A_NEGATION_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NUMBER_OF_SOMEONES_DAY() -> Py { ensure_init(); G_A_NUMBER_OF_SOMEONES_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NUMBER_OF_SOMEONES_DAY(v: Py) { ensure_init(); G_A_NUMBER_OF_SOMEONES_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PAIN_WORD() -> Py { ensure_init(); G_A_PAIN_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PAIN_WORD(v: Py) { ensure_init(); G_A_PAIN_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PARTS_OF_A_PERIOD() -> Py { ensure_init(); G_A_PARTS_OF_A_PERIOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PARTS_OF_A_PERIOD(v: Py) { ensure_init(); G_A_PARTS_OF_A_PERIOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PART_A_PERSON_OWNS() -> Py { ensure_init(); G_A_PART_A_PERSON_OWNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PART_A_PERSON_OWNS(v: Py) { ensure_init(); G_A_PART_A_PERSON_OWNS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PART_THE_PAIN_IS_IN() -> Py { ensure_init(); G_A_PART_THE_PAIN_IS_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PART_THE_PAIN_IS_IN(v: Py) { ensure_init(); G_A_PART_THE_PAIN_IS_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERIOD_FOLLOWS() -> Py { ensure_init(); G_A_PERIOD_FOLLOWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERIOD_FOLLOWS(v: Py) { ensure_init(); G_A_PERIOD_FOLLOWS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRONOUN_GLUED() -> Py { ensure_init(); G_A_PRONOUN_GLUED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRONOUN_GLUED(v: Py) { ensure_init(); G_A_PRONOUN_GLUED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRONOUN_STATE() -> Py { ensure_init(); G_A_PRONOUN_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRONOUN_STATE(v: Py) { ensure_init(); G_A_PRONOUN_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SIDE() -> Py { ensure_init(); G_A_SIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SIDE(v: Py) { ensure_init(); G_A_SIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE_THEN_A_TIME() -> Py { ensure_init(); G_A_STATE_THEN_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE_THEN_A_TIME(v: Py) { ensure_init(); G_A_STATE_THEN_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATE_YOU_CAN_FEEL() -> Py { ensure_init(); G_A_STATE_YOU_CAN_FEEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATE_YOU_CAN_FEEL(v: Py) { ensure_init(); G_A_STATE_YOU_CAN_FEEL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_GOES_AFTER() -> Py { ensure_init(); G_A_THING_GOES_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_GOES_AFTER(v: Py) { ensure_init(); G_A_THING_GOES_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_TYPED_AS_A_LITTLE_WORD() -> Py { ensure_init(); G_A_THING_TYPED_AS_A_LITTLE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_TYPED_AS_A_LITTLE_WORD(v: Py) { ensure_init(); G_A_THING_TYPED_AS_A_LITTLE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_WORN() -> Py { ensure_init(); G_A_THING_WORN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_WORN(v: Py) { ensure_init(); G_A_THING_WORN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_TO_LEAVE() -> Py { ensure_init(); G_A_TIME_TO_LEAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_TO_LEAVE(v: Py) { ensure_init(); G_A_TIME_TO_LEAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERB_CAN_FOLLOW() -> Py { ensure_init(); G_A_VERB_CAN_FOLLOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERB_CAN_FOLLOW(v: Py) { ensure_init(); G_A_VERB_CAN_FOLLOW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORD_CAN_FOLLOW() -> Py { ensure_init(); G_A_WORD_CAN_FOLLOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORD_CAN_FOLLOW(v: Py) { ensure_init(); G_A_WORD_CAN_FOLLOW.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_SAYING() -> Py { ensure_init(); G_BAD_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_SAYING(v: Py) { ensure_init(); G_BAD_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_WORD() -> Py { ensure_init(); G_BAD_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_WORD(v: Py) { ensure_init(); G_BAD_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_TO() -> Py { ensure_init(); G_BARE_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_TO(v: Py) { ensure_init(); G_BARE_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_BASE() -> Py { ensure_init(); G_BASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BASE(v: Py) { ensure_init(); G_BASE.with(|c| *c.borrow_mut() = v); }
pub fn g_BEEN_SUBJECT() -> Py { ensure_init(); G_BEEN_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEEN_SUBJECT(v: Py) { ensure_init(); G_BEEN_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_BEEN_WORDS() -> Py { ensure_init(); G_BEEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEEN_WORDS(v: Py) { ensure_init(); G_BEEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_A_FEELING() -> Py { ensure_init(); G_BEFORE_A_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_A_FEELING(v: Py) { ensure_init(); G_BEFORE_A_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_A_JOB() -> Py { ensure_init(); G_BEFORE_A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_A_JOB(v: Py) { ensure_init(); G_BEFORE_A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_A_PREPOSITION() -> Py { ensure_init(); G_BEFORE_A_PREPOSITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_A_PREPOSITION(v: Py) { ensure_init(); G_BEFORE_A_PREPOSITION.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_A_THING() -> Py { ensure_init(); G_BEFORE_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_A_THING(v: Py) { ensure_init(); G_BEFORE_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_A_VERB() -> Py { ensure_init(); G_BEFORE_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_A_VERB(v: Py) { ensure_init(); G_BEFORE_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_THE_LOSING() -> Py { ensure_init(); G_BEFORE_THE_LOSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_THE_LOSING(v: Py) { ensure_init(); G_BEFORE_THE_LOSING.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_VERB() -> Py { ensure_init(); G_BEFORE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_VERB(v: Py) { ensure_init(); G_BEFORE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_WORD() -> Py { ensure_init(); G_BEFORE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_WORD(v: Py) { ensure_init(); G_BEFORE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_BE_BEFORE_A_STATE() -> Py { ensure_init(); G_BE_BEFORE_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BE_BEFORE_A_STATE(v: Py) { ensure_init(); G_BE_BEFORE_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_BE_FORMS() -> Py { ensure_init(); G_BE_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BE_FORMS(v: Py) { ensure_init(); G_BE_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_BE_VERBS() -> Py { ensure_init(); G_BE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BE_VERBS(v: Py) { ensure_init(); G_BE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_BE_WORDS() -> Py { ensure_init(); G_BE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BE_WORDS(v: Py) { ensure_init(); G_BE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BIN_CONTAINERS() -> Py { ensure_init(); G_BIN_CONTAINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIN_CONTAINERS(v: Py) { ensure_init(); G_BIN_CONTAINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_BREAKS() -> Py { ensure_init(); G_BREAKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BREAKS(v: Py) { ensure_init(); G_BREAKS.with(|c| *c.borrow_mut() = v); }
pub fn g_CAN_FOLLOW_A_VERB() -> Py { ensure_init(); G_CAN_FOLLOW_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAN_FOLLOW_A_VERB(v: Py) { ensure_init(); G_CAN_FOLLOW_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_CAN_NOT() -> Py { ensure_init(); G_CAN_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAN_NOT(v: Py) { ensure_init(); G_CAN_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSES_A_STATE() -> Py { ensure_init(); G_CAUSES_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSES_A_STATE(v: Py) { ensure_init(); G_CAUSES_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSE_A_CLAUSE_VERB() -> Py { ensure_init(); G_CAUSE_A_CLAUSE_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSE_A_CLAUSE_VERB(v: Py) { ensure_init(); G_CAUSE_A_CLAUSE_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSE_A_DEMONSTRATIVE() -> Py { ensure_init(); G_CAUSE_A_DEMONSTRATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSE_A_DEMONSTRATIVE(v: Py) { ensure_init(); G_CAUSE_A_DEMONSTRATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSE_A_SUBJECT_FOLLOWS() -> Py { ensure_init(); G_CAUSE_A_SUBJECT_FOLLOWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSE_A_SUBJECT_FOLLOWS(v: Py) { ensure_init(); G_CAUSE_A_SUBJECT_FOLLOWS.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSE_BEFORE_A_NOUN() -> Py { ensure_init(); G_CAUSE_BEFORE_A_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSE_BEFORE_A_NOUN(v: Py) { ensure_init(); G_CAUSE_BEFORE_A_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_CAUSE_BEFORE_A_VERB() -> Py { ensure_init(); G_CAUSE_BEFORE_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAUSE_BEFORE_A_VERB(v: Py) { ensure_init(); G_CAUSE_BEFORE_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_OF_SUBJECT() -> Py { ensure_init(); G_CHANGE_OF_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_OF_SUBJECT(v: Py) { ensure_init(); G_CHANGE_OF_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_AS_ONE_WORD() -> Py { ensure_init(); G_CHAT_AS_ONE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_AS_ONE_WORD(v: Py) { ensure_init(); G_CHAT_AS_ONE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_KEPT_AS_TYPED() -> Py { ensure_init(); G_CHAT_KEPT_AS_TYPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_KEPT_AS_TYPED(v: Py) { ensure_init(); G_CHAT_KEPT_AS_TYPED.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_NAMES() -> Py { ensure_init(); G_CHAT_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_NAMES(v: Py) { ensure_init(); G_CHAT_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_OPENERS() -> Py { ensure_init(); G_CLAUSE_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_OPENERS(v: Py) { ensure_init(); G_CLAUSE_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLIPPED_FAMILY() -> Py { ensure_init(); G_CLIPPED_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLIPPED_FAMILY(v: Py) { ensure_init(); G_CLIPPED_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_CLIPPED_THINGS() -> Py { ensure_init(); G_CLIPPED_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLIPPED_THINGS(v: Py) { ensure_init(); G_CLIPPED_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLIPPED_UNITS() -> Py { ensure_init(); G_CLIPPED_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLIPPED_UNITS(v: Py) { ensure_init(); G_CLIPPED_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMMON_RANK() -> Py { ensure_init(); G_COMMON_RANK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMMON_RANK(v: Py) { ensure_init(); G_COMMON_RANK.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPOUND_HEADS() -> Py { ensure_init(); G_COMPOUND_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPOUND_HEADS(v: Py) { ensure_init(); G_COMPOUND_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPOUND_TAILS() -> Py { ensure_init(); G_COMPOUND_TAILS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPOUND_TAILS(v: Py) { ensure_init(); G_COMPOUND_TAILS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRACTION_NEXT() -> Py { ensure_init(); G_CONTRACTION_NEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRACTION_NEXT(v: Py) { ensure_init(); G_CONTRACTION_NEXT.with(|c| *c.borrow_mut() = v); }
pub fn g_COPULAS() -> Py { ensure_init(); G_COPULAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COPULAS(v: Py) { ensure_init(); G_COPULAS.with(|c| *c.borrow_mut() = v); }
pub fn g_CORE_VERBS() -> Py { ensure_init(); G_CORE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORE_VERBS(v: Py) { ensure_init(); G_CORE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTS() -> Py { ensure_init(); G_COUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTS(v: Py) { ensure_init(); G_COUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_UNITS() -> Py { ensure_init(); G_COUNT_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_UNITS(v: Py) { ensure_init(); G_COUNT_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_CURRENCY_FACTOR() -> Py { ensure_init(); G_CURRENCY_FACTOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURRENCY_FACTOR(v: Py) { ensure_init(); G_CURRENCY_FACTOR.with(|c| *c.borrow_mut() = v); }
pub fn g_CURRENCY_SIGN() -> Py { ensure_init(); G_CURRENCY_SIGN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURRENCY_SIGN(v: Py) { ensure_init(); G_CURRENCY_SIGN.with(|c| *c.borrow_mut() = v); }
pub fn g_CURRENCY_SIGNS() -> Py { ensure_init(); G_CURRENCY_SIGNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CURRENCY_SIGNS(v: Py) { ensure_init(); G_CURRENCY_SIGNS.with(|c| *c.borrow_mut() = v); }
pub fn g_DAY_NAMED() -> Py { ensure_init(); G_DAY_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAY_NAMED(v: Py) { ensure_init(); G_DAY_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_DEMONSTRATIVE() -> Py { ensure_init(); G_DEMONSTRATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEMONSTRATIVE(v: Py) { ensure_init(); G_DEMONSTRATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINERS() -> Py { ensure_init(); G_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); G_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINER_IN_FRONT() -> Py { ensure_init(); G_DETERMINER_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINER_IN_FRONT(v: Py) { ensure_init(); G_DETERMINER_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_DIGIT_AS_WORD() -> Py { ensure_init(); G_DIGIT_AS_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIGIT_AS_WORD(v: Py) { ensure_init(); G_DIGIT_AS_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_DIGIT_WORDS() -> Py { ensure_init(); G_DIGIT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIGIT_WORDS(v: Py) { ensure_init(); G_DIGIT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DOCS_ON() -> Py { ensure_init(); G_DOCS_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOCS_ON(v: Py) { ensure_init(); G_DOCS_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_DOCS_PAPER() -> Py { ensure_init(); G_DOCS_PAPER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOCS_PAPER(v: Py) { ensure_init(); G_DOCS_PAPER.with(|c| *c.borrow_mut() = v); }
pub fn g_DOCS_SCREEN() -> Py { ensure_init(); G_DOCS_SCREEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOCS_SCREEN(v: Py) { ensure_init(); G_DOCS_SCREEN.with(|c| *c.borrow_mut() = v); }
pub fn g_DOCS_VISIT() -> Py { ensure_init(); G_DOCS_VISIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOCS_VISIT(v: Py) { ensure_init(); G_DOCS_VISIT.with(|c| *c.borrow_mut() = v); }
pub fn g_DOING_NOT_THING() -> Py { ensure_init(); G_DOING_NOT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOING_NOT_THING(v: Py) { ensure_init(); G_DOING_NOT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_DROPPED_SUBJECT_AUX() -> Py { ensure_init(); G_DROPPED_SUBJECT_AUX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DROPPED_SUBJECT_AUX(v: Py) { ensure_init(); G_DROPPED_SUBJECT_AUX.with(|c| *c.borrow_mut() = v); }
pub fn g_EXCLAIMED_AFTER_A_QUESTION() -> Py { ensure_init(); G_EXCLAIMED_AFTER_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXCLAIMED_AFTER_A_QUESTION(v: Py) { ensure_init(); G_EXCLAIMED_AFTER_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_EXCLAMATION_OPENS() -> Py { ensure_init(); G_EXCLAMATION_OPENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXCLAMATION_OPENS(v: Py) { ensure_init(); G_EXCLAMATION_OPENS.with(|c| *c.borrow_mut() = v); }
pub fn g_EXCLAMATION_PHRASE() -> Py { ensure_init(); G_EXCLAMATION_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXCLAMATION_PHRASE(v: Py) { ensure_init(); G_EXCLAMATION_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_E_KEPT_WORDS() -> Py { ensure_init(); G_E_KEPT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_E_KEPT_WORDS(v: Py) { ensure_init(); G_E_KEPT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_SHRT() -> Py { ensure_init(); G_FAMILY_SHRT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_SHRT(v: Py) { ensure_init(); G_FAMILY_SHRT.with(|c| *c.borrow_mut() = v); }
pub fn g_FEELING_BEFORE_SICK() -> Py { ensure_init(); G_FEELING_BEFORE_SICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEELING_BEFORE_SICK(v: Py) { ensure_init(); G_FEELING_BEFORE_SICK.with(|c| *c.borrow_mut() = v); }
pub fn g_FINDING_A_TEAM() -> Py { ensure_init(); G_FINDING_A_TEAM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FINDING_A_TEAM(v: Py) { ensure_init(); G_FINDING_A_TEAM.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON_BE() -> Py { ensure_init(); G_FIRST_PERSON_BE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON_BE(v: Py) { ensure_init(); G_FIRST_PERSON_BE.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_SLOT_LEAD() -> Py { ensure_init(); G_FOOD_SLOT_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_SLOT_LEAD(v: Py) { ensure_init(); G_FOOD_SLOT_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_A() -> Py { ensure_init(); G_FOR_A.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_A(v: Py) { ensure_init(); G_FOR_A.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_MEAL() -> Py { ensure_init(); G_FOR_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_MEAL(v: Py) { ensure_init(); G_FOR_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_FOR_OBJECT() -> Py { ensure_init(); G_FOR_OBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOR_OBJECT(v: Py) { ensure_init(); G_FOR_OBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQUENT() -> Py { ensure_init(); G_FREQUENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQUENT(v: Py) { ensure_init(); G_FREQUENT.with(|c| *c.borrow_mut() = v); }
pub fn g_F_TYPED_FOR_TH() -> Py { ensure_init(); G_F_TYPED_FOR_TH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_F_TYPED_FOR_TH(v: Py) { ensure_init(); G_F_TYPED_FOR_TH.with(|c| *c.borrow_mut() = v); }
pub fn g_GLUED_AFTER_A_DO() -> Py { ensure_init(); G_GLUED_AFTER_A_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLUED_AFTER_A_DO(v: Py) { ensure_init(); G_GLUED_AFTER_A_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_GLUED_COLLAPSE() -> Py { ensure_init(); G_GLUED_COLLAPSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLUED_COLLAPSE(v: Py) { ensure_init(); G_GLUED_COLLAPSE.with(|c| *c.borrow_mut() = v); }
pub fn g_GLUED_DO() -> Py { ensure_init(); G_GLUED_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLUED_DO(v: Py) { ensure_init(); G_GLUED_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_GLUED_LEFT() -> Py { ensure_init(); G_GLUED_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLUED_LEFT(v: Py) { ensure_init(); G_GLUED_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g_GLUED_RIGHT() -> Py { ensure_init(); G_GLUED_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLUED_RIGHT(v: Py) { ensure_init(); G_GLUED_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_HAS_AFTER() -> Py { ensure_init(); G_HAS_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAS_AFTER(v: Py) { ensure_init(); G_HAS_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_HAS_FORMS() -> Py { ensure_init(); G_HAS_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAS_FORMS(v: Py) { ensure_init(); G_HAS_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_HAS_SUBJECT() -> Py { ensure_init(); G_HAS_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAS_SUBJECT(v: Py) { ensure_init(); G_HAS_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_HAS_SUBJECTS() -> Py { ensure_init(); G_HAS_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAS_SUBJECTS(v: Py) { ensure_init(); G_HAS_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_FORMS() -> Py { ensure_init(); G_HAVE_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_FORMS(v: Py) { ensure_init(); G_HAVE_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEART_FEELINGS() -> Py { ensure_init(); G_HEART_FEELINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEART_FEELINGS(v: Py) { ensure_init(); G_HEART_FEELINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEART_MEASURES() -> Py { ensure_init(); G_HEART_MEASURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEART_MEASURES(v: Py) { ensure_init(); G_HEART_MEASURES.with(|c| *c.borrow_mut() = v); }
pub fn g_HEART_SAID_AS_A_FEELING() -> Py { ensure_init(); G_HEART_SAID_AS_A_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEART_SAID_AS_A_FEELING(v: Py) { ensure_init(); G_HEART_SAID_AS_A_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_HELLO_SAID_SHORT() -> Py { ensure_init(); G_HELLO_SAID_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELLO_SAID_SHORT(v: Py) { ensure_init(); G_HELLO_SAID_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g_HELLO_TAIL() -> Py { ensure_init(); G_HELLO_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HELLO_TAIL(v: Py) { ensure_init(); G_HELLO_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_DEGREE() -> Py { ensure_init(); G_HOW_DEGREE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_DEGREE(v: Py) { ensure_init(); G_HOW_DEGREE.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_FOLLOWS() -> Py { ensure_init(); G_HOW_FOLLOWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_FOLLOWS(v: Py) { ensure_init(); G_HOW_FOLLOWS.with(|c| *c.borrow_mut() = v); }
pub fn g_HR_IS_THE_DEPARTMENT_AFTER() -> Py { ensure_init(); G_HR_IS_THE_DEPARTMENT_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HR_IS_THE_DEPARTMENT_AFTER(v: Py) { ensure_init(); G_HR_IS_THE_DEPARTMENT_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_HV_HELPERS() -> Py { ensure_init(); G_HV_HELPERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HV_HELPERS(v: Py) { ensure_init(); G_HV_HELPERS.with(|c| *c.borrow_mut() = v); }
pub fn g_HV_SUBJECTS() -> Py { ensure_init(); G_HV_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HV_SUBJECTS(v: Py) { ensure_init(); G_HV_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_ILL_GOES_ON() -> Py { ensure_init(); G_ILL_GOES_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ILL_GOES_ON(v: Py) { ensure_init(); G_ILL_GOES_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_ING_THINGS() -> Py { ensure_init(); G_ING_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ING_THINGS(v: Py) { ensure_init(); G_ING_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_INTRODUCING_AFTER() -> Py { ensure_init(); G_INTRODUCING_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTRODUCING_AFTER(v: Py) { ensure_init(); G_INTRODUCING_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_INTRODUCING_A_PERSON() -> Py { ensure_init(); G_INTRODUCING_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTRODUCING_A_PERSON(v: Py) { ensure_init(); G_INTRODUCING_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_NOUNS() -> Py { ensure_init(); G_IN_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_NOUNS(v: Py) { ensure_init(); G_IN_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_IRREGULAR_PARTICIPLES() -> Py { ensure_init(); G_IRREGULAR_PARTICIPLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IRREGULAR_PARTICIPLES(v: Py) { ensure_init(); G_IRREGULAR_PARTICIPLES.with(|c| *c.borrow_mut() = v); }
pub fn g_I_GLUED() -> Py { ensure_init(); G_I_GLUED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_I_GLUED(v: Py) { ensure_init(); G_I_GLUED.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINED_WORDS() -> Py { ensure_init(); G_JOINED_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINED_WORDS(v: Py) { ensure_init(); G_JOINED_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_FILLERS() -> Py { ensure_init(); G_KIN_FILLERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_FILLERS(v: Py) { ensure_init(); G_KIN_FILLERS.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_HAS() -> Py { ensure_init(); G_KIN_HAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_HAS(v: Py) { ensure_init(); G_KIN_HAS.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_IS() -> Py { ensure_init(); G_KIN_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_IS(v: Py) { ensure_init(); G_KIN_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN_SINGULAR() -> Py { ensure_init(); G_KIN_SINGULAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN_SINGULAR(v: Py) { ensure_init(); G_KIN_SINGULAR.with(|c| *c.borrow_mut() = v); }
pub fn g_KNOW_SUBJECT() -> Py { ensure_init(); G_KNOW_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KNOW_SUBJECT(v: Py) { ensure_init(); G_KNOW_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADERS() -> Py { ensure_init(); G_LEADERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADERS(v: Py) { ensure_init(); G_LEADERS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OFF_CONTRACTION() -> Py { ensure_init(); G_LEFT_OFF_CONTRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OFF_CONTRACTION(v: Py) { ensure_init(); G_LEFT_OFF_CONTRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g_LETTERS() -> Py { ensure_init(); G_LETTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LETTERS(v: Py) { ensure_init(); G_LETTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_LETTER_C_IS_SEE() -> Py { ensure_init(); G_LETTER_C_IS_SEE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LETTER_C_IS_SEE(v: Py) { ensure_init(); G_LETTER_C_IS_SEE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEXICON() -> Py { ensure_init(); G_LEXICON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEXICON(v: Py) { ensure_init(); G_LEXICON.with(|c| *c.borrow_mut() = v); }
pub fn g_LIKE_STARTS_A_CLAUSE() -> Py { ensure_init(); G_LIKE_STARTS_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIKE_STARTS_A_CLAUSE(v: Py) { ensure_init(); G_LIKE_STARTS_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG_ENOUGH_TO_BE_A_VERB() -> Py { ensure_init(); G_LONG_ENOUGH_TO_BE_A_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG_ENOUGH_TO_BE_A_VERB(v: Py) { ensure_init(); G_LONG_ENOUGH_TO_BE_A_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOSE_FOR_LOSE() -> Py { ensure_init(); G_LOOSE_FOR_LOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOSE_FOR_LOSE(v: Py) { ensure_init(); G_LOOSE_FOR_LOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAL_WORDS() -> Py { ensure_init(); G_MEAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAL_WORDS(v: Py) { ensure_init(); G_MEAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ME_FOR_MY_CLOSES() -> Py { ensure_init(); G_ME_FOR_MY_CLOSES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ME_FOR_MY_CLOSES(v: Py) { ensure_init(); G_ME_FOR_MY_CLOSES.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSING_E_AFTER() -> Py { ensure_init(); G_MISSING_E_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSING_E_AFTER(v: Py) { ensure_init(); G_MISSING_E_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_MI_BACK_VERBS() -> Py { ensure_init(); G_MI_BACK_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MI_BACK_VERBS(v: Py) { ensure_init(); G_MI_BACK_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_MI_IS_A_NAME_AFTER() -> Py { ensure_init(); G_MI_IS_A_NAME_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MI_IS_A_NAME_AFTER(v: Py) { ensure_init(); G_MI_IS_A_NAME_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_MI_IS_ME_AFTER() -> Py { ensure_init(); G_MI_IS_ME_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MI_IS_ME_AFTER(v: Py) { ensure_init(); G_MI_IS_ME_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_MI_IS_ME_BEFORE() -> Py { ensure_init(); G_MI_IS_ME_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MI_IS_ME_BEFORE(v: Py) { ensure_init(); G_MI_IS_ME_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_MODAL_FORMS() -> Py { ensure_init(); G_MODAL_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MODAL_FORMS(v: Py) { ensure_init(); G_MODAL_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOTION_BEFORE() -> Py { ensure_init(); G_MOTION_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOTION_BEFORE(v: Py) { ensure_init(); G_MOTION_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_MRS_IS_THE_WIFE() -> Py { ensure_init(); G_MRS_IS_THE_WIFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MRS_IS_THE_WIFE(v: Py) { ensure_init(); G_MRS_IS_THE_WIFE.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_LETTER() -> Py { ensure_init(); G_NAMED_LETTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_LETTER(v: Py) { ensure_init(); G_NAMED_LETTER.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_CALLS() -> Py { ensure_init(); G_NAME_CALLS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_CALLS(v: Py) { ensure_init(); G_NAME_CALLS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_FRAMES() -> Py { ensure_init(); G_NAME_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_FRAMES(v: Py) { ensure_init(); G_NAME_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_ROLL() -> Py { ensure_init(); G_NAME_ROLL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_ROLL(v: Py) { ensure_init(); G_NAME_ROLL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOMINAL_BEFORE_COPULA() -> Py { ensure_init(); G_NOMINAL_BEFORE_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOMINAL_BEFORE_COPULA(v: Py) { ensure_init(); G_NOMINAL_BEFORE_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_COMPOUND_HEAD() -> Py { ensure_init(); G_NOT_A_COMPOUND_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_COMPOUND_HEAD(v: Py) { ensure_init(); G_NOT_A_COMPOUND_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_COMPOUND_TAIL() -> Py { ensure_init(); G_NOT_A_COMPOUND_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_COMPOUND_TAIL(v: Py) { ensure_init(); G_NOT_A_COMPOUND_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_MEAL_TYPO() -> Py { ensure_init(); G_NOT_A_MEAL_TYPO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MEAL_TYPO(v: Py) { ensure_init(); G_NOT_A_MEAL_TYPO.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_AFTER() -> Py { ensure_init(); G_NOT_A_THING_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_AFTER(v: Py) { ensure_init(); G_NOT_A_THING_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_ENDING() -> Py { ensure_init(); G_NOT_A_THING_ENDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_ENDING(v: Py) { ensure_init(); G_NOT_A_THING_ENDING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_PAIR() -> Py { ensure_init(); G_NOT_A_THING_PAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_PAIR(v: Py) { ensure_init(); G_NOT_A_THING_PAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_BEFORE_A_NOUN() -> Py { ensure_init(); G_NOT_BEFORE_A_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_BEFORE_A_NOUN(v: Py) { ensure_init(); G_NOT_BEFORE_A_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_BEFORE_KNOW() -> Py { ensure_init(); G_NOT_BEFORE_KNOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_BEFORE_KNOW(v: Py) { ensure_init(); G_NOT_BEFORE_KNOW.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_COUNTED_AFTER() -> Py { ensure_init(); G_NOT_COUNTED_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_COUNTED_AFTER(v: Py) { ensure_init(); G_NOT_COUNTED_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_IN_A_QUESTION_PHRASE() -> Py { ensure_init(); G_NOT_IN_A_QUESTION_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_IN_A_QUESTION_PHRASE(v: Py) { ensure_init(); G_NOT_IN_A_QUESTION_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MY_OWN_DAY() -> Py { ensure_init(); G_NOT_MY_OWN_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MY_OWN_DAY(v: Py) { ensure_init(); G_NOT_MY_OWN_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_WORDS() -> Py { ensure_init(); G_NOT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_WORDS(v: Py) { ensure_init(); G_NOT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_ASK_AFTER() -> Py { ensure_init(); G_NO_ASK_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_ASK_AFTER(v: Py) { ensure_init(); G_NO_ASK_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_SWAP_TO() -> Py { ensure_init(); G_NO_SWAP_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_SWAP_TO(v: Py) { ensure_init(); G_NO_SWAP_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBERS_BEFORE() -> Py { ensure_init(); G_NUMBERS_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBERS_BEFORE(v: Py) { ensure_init(); G_NUMBERS_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_OBJECT_PRONOUN() -> Py { ensure_init(); G_OBJECT_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBJECT_PRONOUN(v: Py) { ensure_init(); G_OBJECT_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_OF_A_KIND() -> Py { ensure_init(); G_ONE_OF_A_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_OF_A_KIND(v: Py) { ensure_init(); G_ONE_OF_A_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_PART_AFTER() -> Py { ensure_init(); G_ONE_PART_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_PART_AFTER(v: Py) { ensure_init(); G_ONE_PART_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_RELATION() -> Py { ensure_init(); G_ONE_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_RELATION(v: Py) { ensure_init(); G_ONE_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_RELATION_VERBS() -> Py { ensure_init(); G_ONE_RELATION_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_RELATION_VERBS(v: Py) { ensure_init(); G_ONE_RELATION_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_WORD_FOODS() -> Py { ensure_init(); G_ONE_WORD_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_WORD_FOODS(v: Py) { ensure_init(); G_ONE_WORD_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENERS() -> Py { ensure_init(); G_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENERS(v: Py) { ensure_init(); G_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPERATORS() -> Py { ensure_init(); G_OPERATORS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPERATORS(v: Py) { ensure_init(); G_OPERATORS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNER_WITHOUT_THE_DAY() -> Py { ensure_init(); G_OWNER_WITHOUT_THE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNER_WITHOUT_THE_DAY(v: Py) { ensure_init(); G_OWNER_WITHOUT_THE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNING_A_PERSON() -> Py { ensure_init(); G_OWNING_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNING_A_PERSON(v: Py) { ensure_init(); G_OWNING_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_OWNING_VERBS() -> Py { ensure_init(); G_OWNING_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWNING_VERBS(v: Py) { ensure_init(); G_OWNING_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_REACH() -> Py { ensure_init(); G_PART_REACH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_REACH(v: Py) { ensure_init(); G_PART_REACH.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_THAT_IS_BAD() -> Py { ensure_init(); G_PART_THAT_IS_BAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_THAT_IS_BAD(v: Py) { ensure_init(); G_PART_THAT_IS_BAD.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_AS_PARTICIPLE() -> Py { ensure_init(); G_PAST_AS_PARTICIPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_AS_PARTICIPLE(v: Py) { ensure_init(); G_PAST_AS_PARTICIPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PERIOD_COUNTS() -> Py { ensure_init(); G_PERIOD_COUNTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERIOD_COUNTS(v: Py) { ensure_init(); G_PERIOD_COUNTS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_PREPS() -> Py { ensure_init(); G_PERSON_PREPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_PREPS(v: Py) { ensure_init(); G_PERSON_PREPS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_WORDS() -> Py { ensure_init(); G_PERSON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_WORDS(v: Py) { ensure_init(); G_PERSON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PHRASES() -> Py { ensure_init(); G_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PHRASES(v: Py) { ensure_init(); G_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_PHRASE_MEASURES() -> Py { ensure_init(); G_PHRASE_MEASURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PHRASE_MEASURES(v: Py) { ensure_init(); G_PHRASE_MEASURES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_WORDS() -> Py { ensure_init(); G_PLACE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_WORDS(v: Py) { ensure_init(); G_PLACE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_SUBJECTS() -> Py { ensure_init(); G_PLURAL_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_SUBJECTS(v: Py) { ensure_init(); G_PLURAL_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVES() -> Py { ensure_init(); G_POSSESSIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVES(v: Py) { ensure_init(); G_POSSESSIVES.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE_IN_FRONT() -> Py { ensure_init(); G_POSSESSIVE_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE_IN_FRONT(v: Py) { ensure_init(); G_POSSESSIVE_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_BASES() -> Py { ensure_init(); G_PRONOUN_BASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_BASES(v: Py) { ensure_init(); G_PRONOUN_BASES.with(|c| *c.borrow_mut() = v); }
pub fn g_PUNCT_BREAK() -> Py { ensure_init(); G_PUNCT_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PUNCT_BREAK(v: Py) { ensure_init(); G_PUNCT_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g_RAW_WORD() -> Py { ensure_init(); G_RAW_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RAW_WORD(v: Py) { ensure_init(); G_RAW_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_REASON_STARTS() -> Py { ensure_init(); G_REASON_STARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REASON_STARTS(v: Py) { ensure_init(); G_REASON_STARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATIVE() -> Py { ensure_init(); G_RELATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATIVE(v: Py) { ensure_init(); G_RELATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATIVE_ASKS() -> Py { ensure_init(); G_RELATIVE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATIVE_ASKS(v: Py) { ensure_init(); G_RELATIVE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_ONCE() -> Py { ensure_init(); G_REPEAT_ONCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_ONCE(v: Py) { ensure_init(); G_REPEAT_ONCE.with(|c| *c.borrow_mut() = v); }
pub fn g_REPEAT_ONCE_SUBJECTS() -> Py { ensure_init(); G_REPEAT_ONCE_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPEAT_ONCE_SUBJECTS(v: Py) { ensure_init(); G_REPEAT_ONCE_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTING() -> Py { ensure_init(); G_REPORTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTING(v: Py) { ensure_init(); G_REPORTING.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_ON_LAST_PIECES() -> Py { ensure_init(); G_RUN_ON_LAST_PIECES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_ON_LAST_PIECES(v: Py) { ensure_init(); G_RUN_ON_LAST_PIECES.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_ON_MINE() -> Py { ensure_init(); G_RUN_ON_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_ON_MINE(v: Py) { ensure_init(); G_RUN_ON_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_ON_OPENERS() -> Py { ensure_init(); G_RUN_ON_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_ON_OPENERS(v: Py) { ensure_init(); G_RUN_ON_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_ON_PEOPLE() -> Py { ensure_init(); G_RUN_ON_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_ON_PEOPLE(v: Py) { ensure_init(); G_RUN_ON_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_ON_SELF() -> Py { ensure_init(); G_RUN_ON_SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_ON_SELF(v: Py) { ensure_init(); G_RUN_ON_SELF.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_ON_SELF_LONGEST() -> Py { ensure_init(); G_RUN_ON_SELF_LONGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_ON_SELF_LONGEST(v: Py) { ensure_init(); G_RUN_ON_SELF_LONGEST.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_ON_SHORTEST() -> Py { ensure_init(); G_RUN_ON_SHORTEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_ON_SHORTEST(v: Py) { ensure_init(); G_RUN_ON_SHORTEST.with(|c| *c.borrow_mut() = v); }
pub fn g_RUN_ON_WORDS() -> Py { ensure_init(); G_RUN_ON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RUN_ON_WORDS(v: Py) { ensure_init(); G_RUN_ON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_APART() -> Py { ensure_init(); G_SAID_APART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_APART(v: Py) { ensure_init(); G_SAID_APART.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_APART_FORMS() -> Py { ensure_init(); G_SAID_APART_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_APART_FORMS(v: Py) { ensure_init(); G_SAID_APART_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_AS_A_FEELING() -> Py { ensure_init(); G_SAID_AS_A_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_AS_A_FEELING(v: Py) { ensure_init(); G_SAID_AS_A_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_IT() -> Py { ensure_init(); G_SAID_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_IT(v: Py) { ensure_init(); G_SAID_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_OUT() -> Py { ensure_init(); G_SAID_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_OUT(v: Py) { ensure_init(); G_SAID_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE_END() -> Py { ensure_init(); G_SENTENCE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE_END(v: Py) { ensure_init(); G_SENTENCE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_SHION_WORDS() -> Py { ensure_init(); G_SHION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHION_WORDS(v: Py) { ensure_init(); G_SHION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SHRT_THINGS() -> Py { ensure_init(); G_SHRT_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHRT_THINGS(v: Py) { ensure_init(); G_SHRT_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_SICK_AS_FEELING() -> Py { ensure_init(); G_SICK_AS_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SICK_AS_FEELING(v: Py) { ensure_init(); G_SICK_AS_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_SOLFEGE() -> Py { ensure_init(); G_SOLFEGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOLFEGE(v: Py) { ensure_init(); G_SOLFEGE.with(|c| *c.borrow_mut() = v); }
pub fn g_SON_VERBS() -> Py { ensure_init(); G_SON_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SON_VERBS(v: Py) { ensure_init(); G_SON_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_SORE_FILLERS() -> Py { ensure_init(); G_SORE_FILLERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORE_FILLERS(v: Py) { ensure_init(); G_SORE_FILLERS.with(|c| *c.borrow_mut() = v); }
pub fn g_SORE_IN_FRONT() -> Py { ensure_init(); G_SORE_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORE_IN_FRONT(v: Py) { ensure_init(); G_SORE_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_SORE_PARTS() -> Py { ensure_init(); G_SORE_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SORE_PARTS(v: Py) { ensure_init(); G_SORE_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPELLING_ASK() -> Py { ensure_init(); G_SPELLING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPELLING_ASK(v: Py) { ensure_init(); G_SPELLING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_SPELLING_ENDINGS() -> Py { ensure_init(); G_SPELLING_ENDINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPELLING_ENDINGS(v: Py) { ensure_init(); G_SPELLING_ENDINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPLIT_CONTRACTION() -> Py { ensure_init(); G_SPLIT_CONTRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPLIT_CONTRACTION(v: Py) { ensure_init(); G_SPLIT_CONTRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g_SPLIT_NOT() -> Py { ensure_init(); G_SPLIT_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPLIT_NOT(v: Py) { ensure_init(); G_SPLIT_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_STATES_OF_A_PERSON() -> Py { ensure_init(); G_STATES_OF_A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATES_OF_A_PERSON(v: Py) { ensure_init(); G_STATES_OF_A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_STOPPED_TH_STEMS() -> Py { ensure_init(); G_STOPPED_TH_STEMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOPPED_TH_STEMS(v: Py) { ensure_init(); G_STOPPED_TH_STEMS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_WORDS() -> Py { ensure_init(); G_SUBJECT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_WORDS(v: Py) { ensure_init(); G_SUBJECT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUGAR_AFTER_ROSE() -> Py { ensure_init(); G_SUGAR_AFTER_ROSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUGAR_AFTER_ROSE(v: Py) { ensure_init(); G_SUGAR_AFTER_ROSE.with(|c| *c.borrow_mut() = v); }
pub fn g_SUGAR_FIRST_PERSON() -> Py { ensure_init(); G_SUGAR_FIRST_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUGAR_FIRST_PERSON(v: Py) { ensure_init(); G_SUGAR_FIRST_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SUGAR_LEVEL() -> Py { ensure_init(); G_SUGAR_LEVEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUGAR_LEVEL(v: Py) { ensure_init(); G_SUGAR_LEVEL.with(|c| *c.borrow_mut() = v); }
pub fn g_SUGAR_PRICE() -> Py { ensure_init(); G_SUGAR_PRICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUGAR_PRICE(v: Py) { ensure_init(); G_SUGAR_PRICE.with(|c| *c.borrow_mut() = v); }
pub fn g_SUGAR_PRICE_SHAPE() -> Py { ensure_init(); G_SUGAR_PRICE_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUGAR_PRICE_SHAPE(v: Py) { ensure_init(); G_SUGAR_PRICE_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_SUM_STAYS_AFTER() -> Py { ensure_init(); G_SUM_STAYS_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUM_STAYS_AFTER(v: Py) { ensure_init(); G_SUM_STAYS_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_SUM_THE_NOUN() -> Py { ensure_init(); G_SUM_THE_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUM_THE_NOUN(v: Py) { ensure_init(); G_SUM_THE_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_SUN_WORDS() -> Py { ensure_init(); G_SUN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUN_WORDS(v: Py) { ensure_init(); G_SUN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_S_CLUSTERS() -> Py { ensure_init(); G_S_CLUSTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_S_CLUSTERS(v: Py) { ensure_init(); G_S_CLUSTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_GUARD_BEFORE() -> Py { ensure_init(); G_TAG_GUARD_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_GUARD_BEFORE(v: Py) { ensure_init(); G_TAG_GUARD_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_PLEASE() -> Py { ensure_init(); G_TAG_PLEASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_PLEASE(v: Py) { ensure_init(); G_TAG_PLEASE.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_QUESTION_LEADS() -> Py { ensure_init(); G_TAG_QUESTION_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_QUESTION_LEADS(v: Py) { ensure_init(); G_TAG_QUESTION_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_THANKS() -> Py { ensure_init(); G_TAG_THANKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_THANKS(v: Py) { ensure_init(); G_TAG_THANKS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAG_WORDS() -> Py { ensure_init(); G_TAG_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAG_WORDS(v: Py) { ensure_init(); G_TAG_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_OF_A_DAY() -> Py { ensure_init(); G_TAIL_OF_A_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_OF_A_DAY(v: Py) { ensure_init(); G_TAIL_OF_A_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HEART_SAID_THE_OTHER_WAY() -> Py { ensure_init(); G_THE_HEART_SAID_THE_OTHER_WAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HEART_SAID_THE_OTHER_WAY(v: Py) { ensure_init(); G_THE_HEART_SAID_THE_OTHER_WAY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_LOSING() -> Py { ensure_init(); G_THE_LOSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_LOSING(v: Py) { ensure_init(); G_THE_LOSING.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ODD_PLURALS() -> Py { ensure_init(); G_THE_ODD_PLURALS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ODD_PLURALS(v: Py) { ensure_init(); G_THE_ODD_PLURALS.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_AFTER() -> Py { ensure_init(); G_THING_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_AFTER(v: Py) { ensure_init(); G_THING_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_DETERMINER() -> Py { ensure_init(); G_THING_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_DETERMINER(v: Py) { ensure_init(); G_THING_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_JOIN() -> Py { ensure_init(); G_THING_JOIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_JOIN(v: Py) { ensure_init(); G_THING_JOIN.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_PLACE_AFTER() -> Py { ensure_init(); G_THING_PLACE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_PLACE_AFTER(v: Py) { ensure_init(); G_THING_PLACE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_PLACE_WORDS() -> Py { ensure_init(); G_THING_PLACE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_PLACE_WORDS(v: Py) { ensure_init(); G_THING_PLACE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_POSSESSIVES() -> Py { ensure_init(); G_THING_POSSESSIVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_POSSESSIVES(v: Py) { ensure_init(); G_THING_POSSESSIVES.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_SUBJECT() -> Py { ensure_init(); G_THING_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_SUBJECT(v: Py) { ensure_init(); G_THING_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_WORDS() -> Py { ensure_init(); G_THING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_WORDS(v: Py) { ensure_init(); G_THING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_WORDS_BY_LENGTH() -> Py { ensure_init(); G_THING_WORDS_BY_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_WORDS_BY_LENGTH(v: Py) { ensure_init(); G_THING_WORDS_BY_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD_PERSON() -> Py { ensure_init(); G_THIRD_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD_PERSON(v: Py) { ensure_init(); G_THIRD_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD_PERSON_BE() -> Py { ensure_init(); G_THIRD_PERSON_BE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD_PERSON_BE(v: Py) { ensure_init(); G_THIRD_PERSON_BE.with(|c| *c.borrow_mut() = v); }
pub fn g_THOUSANDS() -> Py { ensure_init(); G_THOUSANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THOUSANDS(v: Py) { ensure_init(); G_THOUSANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THOUSANDS_SAID_WITH_A_SPACE() -> Py { ensure_init(); G_THOUSANDS_SAID_WITH_A_SPACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THOUSANDS_SAID_WITH_A_SPACE(v: Py) { ensure_init(); G_THOUSANDS_SAID_WITH_A_SPACE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIMES_A_STATE() -> Py { ensure_init(); G_TIMES_A_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIMES_A_STATE(v: Py) { ensure_init(); G_TIMES_A_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_AFTER_MEAL() -> Py { ensure_init(); G_TIME_AFTER_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_AFTER_MEAL(v: Py) { ensure_init(); G_TIME_AFTER_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WORDS() -> Py { ensure_init(); G_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WORDS(v: Py) { ensure_init(); G_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TOKEN() -> Py { ensure_init(); G_TOKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOKEN(v: Py) { ensure_init(); G_TOKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_A() -> Py { ensure_init(); G_TOO_A.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_A(v: Py) { ensure_init(); G_TOO_A.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_AFTER() -> Py { ensure_init(); G_TOO_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_AFTER(v: Py) { ensure_init(); G_TOO_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_A_GRADED() -> Py { ensure_init(); G_TOO_A_GRADED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_A_GRADED(v: Py) { ensure_init(); G_TOO_A_GRADED.with(|c| *c.borrow_mut() = v); }
pub fn g_TO_AFTER() -> Py { ensure_init(); G_TO_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TO_AFTER(v: Py) { ensure_init(); G_TO_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_TOO_ASKS() -> Py { ensure_init(); G_UP_TOO_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_TOO_ASKS(v: Py) { ensure_init(); G_UP_TOO_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_TO_LEADS() -> Py { ensure_init(); G_UP_TO_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_TO_LEADS(v: Py) { ensure_init(); G_UP_TO_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_TO_SPELLINGS() -> Py { ensure_init(); G_UP_TO_SPELLINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_TO_SPELLINGS(v: Py) { ensure_init(); G_UP_TO_SPELLINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_TO_TAGS() -> Py { ensure_init(); G_UP_TO_TAGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_TO_TAGS(v: Py) { ensure_init(); G_UP_TO_TAGS.with(|c| *c.borrow_mut() = v); }
pub fn g_URGENT_BONES() -> Py { ensure_init(); G_URGENT_BONES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_URGENT_BONES(v: Py) { ensure_init(); G_URGENT_BONES.with(|c| *c.borrow_mut() = v); }
pub fn g_URGENT_WORDS() -> Py { ensure_init(); G_URGENT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_URGENT_WORDS(v: Py) { ensure_init(); G_URGENT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VEGGIES_AFTER() -> Py { ensure_init(); G_VEGGIES_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEGGIES_AFTER(v: Py) { ensure_init(); G_VEGGIES_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_VOWELS() -> Py { ensure_init(); G_VOWELS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VOWELS(v: Py) { ensure_init(); G_VOWELS.with(|c| *c.borrow_mut() = v); }
pub fn g_VOWEL_SET() -> Py { ensure_init(); G_VOWEL_SET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VOWEL_SET(v: Py) { ensure_init(); G_VOWEL_SET.with(|c| *c.borrow_mut() = v); }
pub fn g_VOWEL_SET_OR_Y() -> Py { ensure_init(); G_VOWEL_SET_OR_Y.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VOWEL_SET_OR_Y(v: Py) { ensure_init(); G_VOWEL_SET_OR_Y.with(|c| *c.borrow_mut() = v); }
pub fn g_WANT_WORDS() -> Py { ensure_init(); G_WANT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANT_WORDS(v: Py) { ensure_init(); G_WANT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WEARS_AFTER() -> Py { ensure_init(); G_WEARS_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEARS_AFTER(v: Py) { ensure_init(); G_WEARS_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_ARE_YOU_AFTER() -> Py { ensure_init(); G_WHAT_ARE_YOU_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_ARE_YOU_AFTER(v: Py) { ensure_init(); G_WHAT_ARE_YOU_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_ARE_YOU_SPELLINGS() -> Py { ensure_init(); G_WHAT_ARE_YOU_SPELLINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_ARE_YOU_SPELLINGS(v: Py) { ensure_init(); G_WHAT_ARE_YOU_SPELLINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_WH_ANY() -> Py { ensure_init(); G_WH_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WH_ANY(v: Py) { ensure_init(); G_WH_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g_WH_INTENSIFIER() -> Py { ensure_init(); G_WH_INTENSIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WH_INTENSIFIER(v: Py) { ensure_init(); G_WH_INTENSIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_WH_INTENSIFIER_LINE() -> Py { ensure_init(); G_WH_INTENSIFIER_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WH_INTENSIFIER_LINE(v: Py) { ensure_init(); G_WH_INTENSIFIER_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_WH_OPENERS() -> Py { ensure_init(); G_WH_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WH_OPENERS(v: Py) { ensure_init(); G_WH_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_WH_QUANTIFIERS() -> Py { ensure_init(); G_WH_QUANTIFIERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WH_QUANTIFIERS(v: Py) { ensure_init(); G_WH_QUANTIFIERS.with(|c| *c.borrow_mut() = v); }
pub fn g_WH_WORDS() -> Py { ensure_init(); G_WH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WH_WORDS(v: Py) { ensure_init(); G_WH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_YOU_ARE_AFTER() -> Py { ensure_init(); G_YOU_ARE_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YOU_ARE_AFTER(v: Py) { ensure_init(); G_YOU_ARE_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_A_DOING() -> Py { ensure_init(); G__AFTER_A_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_A_DOING(v: Py) { ensure_init(); G__AFTER_A_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_A_LEAD_IN_SUBJECT() -> Py { ensure_init(); G__AFTER_A_LEAD_IN_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_A_LEAD_IN_SUBJECT(v: Py) { ensure_init(); G__AFTER_A_LEAD_IN_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_PARTICIPLE() -> Py { ensure_init(); G__AFTER_PARTICIPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_PARTICIPLE(v: Py) { ensure_init(); G__AFTER_PARTICIPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__ANY_WORD() -> Py { ensure_init(); G__ANY_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANY_WORD(v: Py) { ensure_init(); G__ANY_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLIP() -> Py { ensure_init(); G__A_CLIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLIP(v: Py) { ensure_init(); G__A_CLIP.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLIPPED_HEAD() -> Py { ensure_init(); G__A_CLIPPED_HEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLIPPED_HEAD(v: Py) { ensure_init(); G__A_CLIPPED_HEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLOSING_WORD() -> Py { ensure_init(); G__A_CLOSING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLOSING_WORD(v: Py) { ensure_init(); G__A_CLOSING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HEAD_IN_THE_IDIOM() -> Py { ensure_init(); G__A_HEAD_IN_THE_IDIOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HEAD_IN_THE_IDIOM(v: Py) { ensure_init(); G__A_HEAD_IN_THE_IDIOM.with(|c| *c.borrow_mut() = v); }
pub fn g__A_QUESTION_VERB() -> Py { ensure_init(); G__A_QUESTION_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_QUESTION_VERB(v: Py) { ensure_init(); G__A_QUESTION_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SHORT_WORD_IS_NAMED_AFTER() -> Py { ensure_init(); G__A_SHORT_WORD_IS_NAMED_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SHORT_WORD_IS_NAMED_AFTER(v: Py) { ensure_init(); G__A_SHORT_WORD_IS_NAMED_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SHORT_WORD_IS_NAMED_BEFORE() -> Py { ensure_init(); G__A_SHORT_WORD_IS_NAMED_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SHORT_WORD_IS_NAMED_BEFORE(v: Py) { ensure_init(); G__A_SHORT_WORD_IS_NAMED_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STATE_OF_THE_PERSON() -> Py { ensure_init(); G__A_STATE_OF_THE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STATE_OF_THE_PERSON(v: Py) { ensure_init(); G__A_STATE_OF_THE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WORDS() -> Py { ensure_init(); G__A_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WORDS(v: Py) { ensure_init(); G__A_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WORD_THE_BOT_WRITES() -> Py { ensure_init(); G__A_WORD_THE_BOT_WRITES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WORD_THE_BOT_WRITES(v: Py) { ensure_init(); G__A_WORD_THE_BOT_WRITES.with(|c| *c.borrow_mut() = v); }
pub fn g__BASE() -> Py { ensure_init(); G__BASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BASE(v: Py) { ensure_init(); G__BASE.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_WORDS() -> Py { ensure_init(); G__DAY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_WORDS(v: Py) { ensure_init(); G__DAY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__EXCLAMATION_SUBJECTS() -> Py { ensure_init(); G__EXCLAMATION_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EXCLAMATION_SUBJECTS(v: Py) { ensure_init(); G__EXCLAMATION_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g__FEELING_WORD() -> Py { ensure_init(); G__FEELING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEELING_WORD(v: Py) { ensure_init(); G__FEELING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__HAVE_ASKING() -> Py { ensure_init(); G__HAVE_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HAVE_ASKING(v: Py) { ensure_init(); G__HAVE_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g__HAVE_NEGATIONS() -> Py { ensure_init(); G__HAVE_NEGATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HAVE_NEGATIONS(v: Py) { ensure_init(); G__HAVE_NEGATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__HAVE_PARTICIPLES() -> Py { ensure_init(); G__HAVE_PARTICIPLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HAVE_PARTICIPLES(v: Py) { ensure_init(); G__HAVE_PARTICIPLES.with(|c| *c.borrow_mut() = v); }
pub fn g__HAVE_THING() -> Py { ensure_init(); G__HAVE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HAVE_THING(v: Py) { ensure_init(); G__HAVE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__INTENSIFIER() -> Py { ensure_init(); G__INTENSIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INTENSIFIER(v: Py) { ensure_init(); G__INTENSIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEPING_PLACES() -> Py { ensure_init(); G__KEEPING_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEPING_PLACES(v: Py) { ensure_init(); G__KEEPING_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g__KEPT_ALREADY() -> Py { ensure_init(); G__KEPT_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEPT_ALREADY(v: Py) { ensure_init(); G__KEPT_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g__KEPT_THINGS() -> Py { ensure_init(); G__KEPT_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEPT_THINGS(v: Py) { ensure_init(); G__KEPT_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__LEADER_WORD() -> Py { ensure_init(); G__LEADER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEADER_WORD(v: Py) { ensure_init(); G__LEADER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__MEASURE() -> Py { ensure_init(); G__MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEASURE(v: Py) { ensure_init(); G__MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_SHAPE() -> Py { ensure_init(); G__NUMBER_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_SHAPE(v: Py) { ensure_init(); G__NUMBER_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACES_SAID_WHOLE() -> Py { ensure_init(); G__PLACES_SAID_WHOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACES_SAID_WHOLE(v: Py) { ensure_init(); G__PLACES_SAID_WHOLE.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACE_LEADS() -> Py { ensure_init(); G__PLACE_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACE_LEADS(v: Py) { ensure_init(); G__PLACE_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACE_STOP() -> Py { ensure_init(); G__PLACE_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACE_STOP(v: Py) { ensure_init(); G__PLACE_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g__PREDICATE_ONLY() -> Py { ensure_init(); G__PREDICATE_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREDICATE_ONLY(v: Py) { ensure_init(); G__PREDICATE_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g__SMALL_NUMBERS() -> Py { ensure_init(); G__SMALL_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SMALL_NUMBERS(v: Py) { ensure_init(); G__SMALL_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g__SPACED_PIECE() -> Py { ensure_init(); G__SPACED_PIECE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPACED_PIECE(v: Py) { ensure_init(); G__SPACED_PIECE.with(|c| *c.borrow_mut() = v); }
pub fn g__SPACED_RUN() -> Py { ensure_init(); G__SPACED_RUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPACED_RUN(v: Py) { ensure_init(); G__SPACED_RUN.with(|c| *c.borrow_mut() = v); }
pub fn g__STATE_FILLER() -> Py { ensure_init(); G__STATE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATE_FILLER(v: Py) { ensure_init(); G__STATE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__STATE_TAIL() -> Py { ensure_init(); G__STATE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATE_TAIL(v: Py) { ensure_init(); G__STATE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECTLESS_AUX() -> Py { ensure_init(); G__SUBJECTLESS_AUX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECTLESS_AUX(v: Py) { ensure_init(); G__SUBJECTLESS_AUX.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECTLESS_ONLY() -> Py { ensure_init(); G__SUBJECTLESS_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECTLESS_ONLY(v: Py) { ensure_init(); G__SUBJECTLESS_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECTS() -> Py { ensure_init(); G__SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECTS(v: Py) { ensure_init(); G__SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_SAID() -> Py { ensure_init(); G__SUBJECT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_SAID(v: Py) { ensure_init(); G__SUBJECT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__TYPED_SURPRISE() -> Py { ensure_init(); G__TYPED_SURPRISE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TYPED_SURPRISE(v: Py) { ensure_init(); G__TYPED_SURPRISE.with(|c| *c.borrow_mut() = v); }
pub fn g__WERE_ARE() -> Py { ensure_init(); G__WERE_ARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WERE_ARE(v: Py) { ensure_init(); G__WERE_ARE.with(|c| *c.borrow_mut() = v); }
pub fn g__WERE_DOING() -> Py { ensure_init(); G__WERE_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WERE_DOING(v: Py) { ensure_init(); G__WERE_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g__WERE_IN_THE_PAST() -> Py { ensure_init(); G__WERE_IN_THE_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WERE_IN_THE_PAST(v: Py) { ensure_init(); G__WERE_IN_THE_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g__WERE_LEAD_IN() -> Py { ensure_init(); G__WERE_LEAD_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WERE_LEAD_IN(v: Py) { ensure_init(); G__WERE_LEAD_IN.with(|c| *c.borrow_mut() = v); }
pub fn g__WERE_PLACE() -> Py { ensure_init(); G__WERE_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WERE_PLACE(v: Py) { ensure_init(); G__WERE_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__WERE_STATE() -> Py { ensure_init(); G__WERE_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WERE_STATE(v: Py) { ensure_init(); G__WERE_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g__WH_QUESTIONS() -> Py { ensure_init(); G__WH_QUESTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WH_QUESTIONS(v: Py) { ensure_init(); G__WH_QUESTIONS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Normalisation: turn the way people type into ordinary words, once, for every module.\n\nThe understanding lane owns this file. `normalise(text)` cleans one line:\n\n- chat-speak and shorthand (\"u\", \"pls\", \"idk\", \"cn u\", \"wats\") become ordinary words;\n- apostrophes people leave out (\"cant\", \"ive\", \"whats\") are filled back in;\n- contractions are expanded (\"don't\" -> \"do not\"), so a module can match the verb;\n- obvious typos are corrected with one edit against the common-word lexicon, but only when\n  the correction is unambiguous: a wrong fix is worse than no fix;\n- letters people stretch (\"sooo\", \"pleeease\") are collapsed;\n- run-on lines are split into clauses at discourse markers (\"anyway\", \"so\", \"but\"), and the\n  clause that carries the ask is marked as the focus.\n\nIt returns a dict; nothing here reads `memory`, so it is safe to call on any line.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._MEASURE"); }
    s__MEASURE(py_str("[0-9]+(?:[.,][0-9]+)*[a-z%]*|[a-z]{1,3}[0-9]+(?:[a-z]{1,3}[0-9]*)*(?![a-z0-9])|[a-z]+(?:'[a-z]+)*"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WORD"); }
    s_WORD(stdlib_call("re", "compile", &[g__MEASURE()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SAID_OUT"); }
    s_SAID_OUT({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&op_bitor(&pyrt::xrefs::xget("understanding.lexicon", "PLAIN_EXPANDS"), &pyrt::xrefs::xget("understanding.lexicon", "PLAIN_ONE_WORD"))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), &word.clone())) } }) {  __out.push((word.clone(), geti(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &word.clone()))); } } py_dict(__out) })() });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CHAT_KEPT_AS_TYPED"); }
    s_CHAT_KEPT_AS_TYPED(call_builtin("frozenset", &[mcall(&py_str("\n    u ur urs yr yrs urself y r n k b c d v x ar av sed prob cause cos tho owt\n    wif wtf pressie pressies prezzie prezzies prezzy\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CHAT_AS_ONE_WORD"); }
    s_CHAT_AS_ONE_WORD({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "items", &[])) { let __unpk7224 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk7224[0].clone(); let mut meaning: Py = __unpk7224[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&meaning.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_CHAT_KEPT_AS_TYPED(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SAID_OUT(), &word.clone())) } } } }) {  __out.push((word.clone(), meaning.clone())); } } py_dict(__out) })() });
    let _ = mcall(&g_SAID_OUT(), "update", &[g_CHAT_AS_ONE_WORD()]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RAW_WORD"); }
    s_RAW_WORD(stdlib_call("re", "compile", &[g__MEASURE()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.URGENT_WORDS"); }
    s_URGENT_WORDS(op_bitor(&call_builtin("frozenset", &[mcall(&pyrt::xrefs::xget("understanding.lexicon", "URGENT_MISSPELLINGS"), "values", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[py_tuple(vec![py_str("having")])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._ANY_WORD"); }
    s__ANY_WORD(stdlib_call("re", "compile", &[py_str("[a-z]+(?:'[a-z]+)*"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CLIPPED_FAMILY"); }
    s_CLIPPED_FAMILY(stdlib_call("re", "compile", &[py_str("\\b(?P<owner>my|our|his|her|their)\\s+(?P<word>dd|ds|dh|mil|fil|sil|bil|oh|lo)\\b(?P<rest>(?:\\s+[a-z']+){0,2})"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.VOWEL_SET"); }
    s_VOWEL_SET(call_builtin("frozenset", &[py_str("aeiou")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.VOWEL_SET_OR_Y"); }
    s_VOWEL_SET_OR_Y(op_bitor(&g_VOWEL_SET(), &call_builtin("frozenset", &[py_str("y")]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._BASE"); }
    s__BASE(op_add(&op_add(&op_add(&op_add(&op_add(&mcall(&pyrt::xrefs::xget("understanding.lexicon", "COMMON"), "split", &[]), &mcall(&pyrt::xrefs::xget("understanding.lexicon", "CORE"), "split", &[])), &mcall(&mcall(&py_str(" "), "join", &[mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "values", &[])]), "split", &[])), &mcall(&mcall(&py_str(" "), "join", &[mcall(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), "values", &[])]), "split", &[])), &mcall(&mcall(&py_str(" "), "join", &[mcall(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), "values", &[])]), "split", &[])), &mcall(&mcall(&py_str(" "), "join", &[pyrt::xrefs::xget("understanding.lexicon", "REAL_WORDS")]), "split", &[])));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BASE"); }
    s_BASE(call_builtin("frozenset", &[g__BASE()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.LEXICON"); }
    s_LEXICON(op_bitor(&g_BASE(), &call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BASE()) { let mut word: Py = __item.clone(); for __item in py_iter_or_raise(&_inflections(&[word.clone()])) { let mut f: Py = __item.clone(); __out.push(f.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.COMMON_RANK"); }
    s_COMMON_RANK({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[op_add(&mcall(&pyrt::xrefs::xget("understanding.lexicon", "CORE"), "split", &[]), &mcall(&pyrt::xrefs::xget("understanding.lexicon", "COMMON"), "split", &[]))]).unwrap_or_else(|e| exc::raise(e))) { let __unpk6395 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk6395[0].clone(); let mut word: Py = __unpk6395[1].clone(); __out.push((word.clone(), i.clone())); } py_dict(__out) })() });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ME_FOR_MY_CLOSES"); }
    s_ME_FOR_MY_CLOSES(call_builtin("frozenset", &[mcall(&py_str("\n    is are was were am be been being isnt arent wasnt werent aint\n    has have had having got gets getting get\n    will wont would could should can cant cannot must might may shall\n    do does did doing done dont doesnt didnt\n    hurts hurt hurting aches ache aching bangs bang banging throb throbs throbbing kills kill\n    feels feel feeling looks look looking smells smell smelling sounds sound sounding seems seem\n    needs need wants want keeps keep kept goes go going went comes come coming came broke broken\n    breaks break stopped stops started starts works worked died dies dying\n    in on out off up down at to for with from by about over under around through into onto\n    and or but so because as than then\n    not no never still too also just again now today tomorrow yesterday tonight anymore\n    all both every any some\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_WORDS_BY_LENGTH"); }
    s_THING_WORDS_BY_LENGTH(py_dict(vec![]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_WORDS"); }
    s_THING_WORDS(call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PHRASES"); }
    s_PHRASES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitor(&mcall(&{ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "items", &[])) { let __unpk455 = py_unpack(&__item.clone(), 2, None); let mut k: Py = __unpk455[0].clone(); let mut v: Py = __unpk455[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&k.clone(), &py_str(" "))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&k.clone(), &py_str("/"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&k.clone(), &py_str("&"))) } } } }) {  __out.push((k.clone(), v.clone())); } } py_dict(__out) })() }, "items", &[]), &mcall(&{ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), "items", &[])) { let __unpk1688 = py_unpack(&__item.clone(), 2, None); let mut k: Py = __unpk1688[0].clone(); let mut v: Py = __unpk1688[1].clone(); if py_truthy(&Py::Bool(py_contains(&k.clone(), &py_str(" ")))) {  __out.push((k.clone(), v.clone())); } } py_dict(__out) })() }, "items", &[]))); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| {  let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); op_neg(&call_builtin("len", &[geti(&item.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BREAKS"); }
    s_BREAKS(call_builtin("frozenset", &[mcall(&py_str("anyway anyways btw actually instead otherwise also plus so but then oh ok okay right now still however meanwhile finally"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PRONOUN_BASES"); }
    s_PRONOUN_BASES(call_builtin("frozenset", &[mcall(&py_str("i it that this there here he she we they who what where how when why everyone somebody someone nobody everybody one"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ASK_WORDS"); }
    s_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("what why how when where who which whose can could should would is are do does did will have has shall may might am was were"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WANT_WORDS"); }
    s_WANT_WORDS(call_builtin("frozenset", &[mcall(&py_str("want need like looking help recommend suggest advice idea ideas tip tips know wonder thinking plan trying"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WH_WORDS"); }
    s_WH_WORDS(call_builtin("frozenset", &[mcall(&py_str("what why when where who which whose"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AUX_WORDS"); }
    s_AUX_WORDS(call_builtin("frozenset", &[mcall(&py_str("is are was were am do does did can could should would will shall have has may might"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BE_WORDS"); }
    s_BE_WORDS(call_builtin("frozenset", &[mcall(&py_str("is are was were am"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUBJECT_WORDS"); }
    s_SUBJECT_WORDS(call_builtin("frozenset", &[mcall(&py_str("i we you they he she"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HOW_DEGREE"); }
    s_HOW_DEGREE(call_builtin("frozenset", &[mcall(&py_str("long much many often old far big soon hot cold tall early late heavy deep"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NO_ASK_AFTER"); }
    s_NO_ASK_AFTER(call_builtin("frozenset", &[mcall(&py_str("that if whether what why when where who which whose how"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ASK_NOUNS"); }
    s_ASK_NOUNS(call_builtin("frozenset", &[mcall(&py_str("time day date week month year kind sort type colour color size\namount number temperature age price reason way one ones much many long often far"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.REPORTING"); }
    s_REPORTING(call_builtin("frozenset", &[mcall(&py_str("know knows knew known tell tells told say says said ask asks asked\nwonder wondered wondering remember remembered forget forgot decide decided sure idea clue\nmean means meant matter matters question doubt explain explains argue argued arguing\ndisagree disagreed discuss discussed debate debated talk talked mention mentioned\nconsider considered"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.COPULAS"); }
    s_COPULAS(call_builtin("frozenset", &[mcall(&py_str("is are was were am be been being"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOMINAL_BEFORE_COPULA"); }
    s_NOMINAL_BEFORE_COPULA(call_builtin("frozenset", &[mcall(&py_str("this that it there here all exactly precisely just\nquestion thing problem point issue doubt reason answer part bit main only also still really\nactually probably"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DETERMINERS"); }
    s_DETERMINERS(call_builtin("frozenset", &[mcall(&py_str("a an the my your his her our their its this that these those\nsome any no every each another anyone everyone someone nobody people ones"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RELATIVE"); }
    s_RELATIVE(call_builtin("frozenset", &[mcall(&py_str("who which whose"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RELATIVE_ASKS"); }
    s_RELATIVE_ASKS(call_builtin("frozenset", &[mcall(&py_str("could can should would will shall do does did"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FREQUENT"); }
    s_FREQUENT(call_builtin("frozenset", &[slice_or_raise(&mcall(&op_add(&op_add(&pyrt::xrefs::xget("understanding.lexicon", "CORE"), &py_str(" ")), &pyrt::xrefs::xget("understanding.lexicon", "COMMON")), "split", &[]), &Py::None, &Py::Int(420), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AEIOU"); }
    s_AEIOU(call_builtin("frozenset", &[py_str("aeiou")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.VOWELS"); }
    s_VOWELS(py_str("aeiou"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.S_CLUSTERS"); }
    s_S_CLUSTERS(py_str("tpskmlnwhqc"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_WORDS"); }
    s__A_WORDS(call_builtin("frozenset", &[mcall(&py_str("cola tuna mocha salsa karma pasta data area idea"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.E_KEPT_WORDS"); }
    s_E_KEPT_WORDS(call_builtin("frozenset", &[py_set(vec![py_str("dyeing"), py_str("singeing"), py_str("tingeing"), py_str("swingeing")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NO_SWAP_TO"); }
    s_NO_SWAP_TO(call_builtin("frozenset", &[py_tuple(vec![py_str("hanged")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_WORD_THE_BOT_WRITES"); }
    s__A_WORD_THE_BOT_WRITES(Py::None);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.URGENT_BONES"); }
    s_URGENT_BONES(py_dict(vec![]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NAME_FRAMES"); }
    s_NAME_FRAMES(call_builtin("frozenset", &[mcall(&py_str("called named"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CHAT_NAMES"); }
    s_CHAT_NAMES(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("understanding.lexicon", "CHAT")) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NAME_ROLL"); }
    s_NAME_ROLL(call_builtin("frozenset", &[mcall(&py_str("names name named go goes going by im am i'm"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NAME_CALLS"); }
    s_NAME_CALLS(call_builtin("frozenset", &[mcall(&py_str("call calls called calling callin name names named"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_dict(vec![(py_str("any"), py_str("anything")), (py_str("eny"), py_str("anything")), (py_str("ani"), py_str("anything")), (py_str("some"), py_str("something")), (py_str("sum"), py_str("something")), (py_str("som"), py_str("something")), (py_str("sumn"), py_str("something")), (py_str("every"), py_str("everything")), (py_str("evry"), py_str("everything")), (py_str("evrey"), py_str("everything")), (py_str("nuf"), py_str("nothing")), (py_str("nuff"), py_str("nothing")), (py_str("nof"), py_str("nothing")), (py_str("noff"), py_str("nothing")), (py_str("nut"), py_str("nothing")), (py_str("nuth"), py_str("nothing")), (py_str("not"), py_str("nothing")), (py_str("no"), py_str("nothing"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.STOPPED_TH_STEMS"); }
    s_STOPPED_TH_STEMS(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SHION_WORDS"); }
    s_SHION_WORDS(call_builtin("frozenset", &[mcall(&py_str("\ncushion fashion passion mission session version decision occasion discussion permission\ntelevision conclusion explosion confusion illusion vision revision division provision\nsuggestion tension expression compassion impression profession possession aggression\nadmission commission expansion extension pension mansion excursion invasion collision\nemission transmission procession succession confession depression digression obsession\nrecession concession progression exclusion inclusion seclusion precision derision\nmagician musician electrician politician technician physician statistician optician\nbeautician mathematician paediatrician dietician clinician"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_dict(vec![(py_str("fe"), py_str("the")), (py_str("fis"), py_str("this")), (py_str("fey"), py_str("they")), (py_str("fem"), py_str("them")), (py_str("fere"), py_str("there")), (py_str("fing"), py_str("thing")), (py_str("fings"), py_str("things")), (py_str("fird"), py_str("third")), (py_str("fousand"), py_str("thousand")), (py_str("fousands"), py_str("thousands")), (py_str("freat"), py_str("threat")), (py_str("freaten"), py_str("threaten")), (py_str("freatens"), py_str("threatens")), (py_str("freatened"), py_str("threatened")), (py_str("freatening"), py_str("threatening"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.F_TYPED_FOR_TH"); }
    s_F_TYPED_FOR_TH(__data2());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_THING_TYPED_AS_A_LITTLE_WORD"); }
    s_A_THING_TYPED_AS_A_LITTLE_WORD(py_dict(vec![(py_str("wif"), py_str("wife")), (py_str("wiff"), py_str("wife"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_THING_GOES_AFTER"); }
    s_A_THING_GOES_AFTER(call_builtin("frozenset", &[mcall(&py_str("the"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CORE_VERBS"); }
    s_CORE_VERBS(call_builtin("frozenset", &[mcall(&py_str("take get give make have go do say see know think want need feel look\n    come wake walk work eat drink buy pay keep leave sleep live start stop try help tell ask\n    call find bring wear watch read cook clean wash sit stand run drive ride"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AFTER_A_VERB"); }
    s_AFTER_A_VERB(call_builtin("frozenset", &[mcall(&py_str("up off out in on back over down away"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEFORE_A_VERB"); }
    s_BEFORE_A_VERB(call_builtin("frozenset", &[mcall(&py_str("i we they you he she"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PART_REACH"); }
    s_PART_REACH(Py::Int(3));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_SIDE"); }
    s_A_SIDE(call_builtin("frozenset", &[mcall(&py_str("five six seven eight nine"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ABOUT_THE_FORM"); }
    s_ABOUT_THE_FORM(stdlib_call("re", "compile", &[py_str("\n    \\bhow\\s+many\\s+(?:players?|people|a\\s+side)\\b\n  | \\bhow\\s+(?:long|many\\s+halves)\\b[^.]{0,40}\\b(?:match|game|half|halves)\\b\n  | \\b(?:the\\s+)?rules?\\b\n  | \\boffside\\b\n  | \\bhow\\s+big\\b\n  | \\bpitch\\s+(?:size|dimensions)\\b\n  | \\bfull\\s+(?:match|game)\\b\n"), op_bitor(&stdlib_attr("re", "VERBOSE").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FINDING_A_TEAM"); }
    s_FINDING_A_TEAM(stdlib_call("re", "compile", &[py_str("\n    \\b(?:join|joining|find|finding|looking\\s+for|sign\\s+up\\s+(?:for|to)|get\\s+into)\\b\n  | \\bteams?\\b[^.]{0,24}\\b(?:in|near|around|local)\\b\n  | \\b(?:clubs?|leagues?)\\b\n"), op_bitor(&stdlib_attr("re", "VERBOSE").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CAUSES_A_STATE"); }
    s_CAUSES_A_STATE(call_builtin("frozenset", &[mcall(&py_str("got get gets getting gotten make makes made making left leave leaves\n    leaving kept keep keeps keeping felt feels feel feeling found find finds find made became\n    become becomes turned turns turn drove driven drive drives"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.OBJECT_PRONOUN"); }
    s_OBJECT_PRONOUN(call_builtin("frozenset", &[mcall(&py_str("me us him her them you u u's myself yourself himself herself\n    ourselves themselves"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PHRASE_MEASURES"); }
    s_PHRASE_MEASURES(call_builtin("frozenset", &[mcall(&py_str("a an the bit little lot lots tad touch wee kind sort of out much\n    many far way right that this these those enough half twice few some more most less least\n    loads bunch ton tons hell heck mega extra damn damned fricking freaking"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TIMES_A_STATE"); }
    s_TIMES_A_STATE(call_builtin("frozenset", &[mcall(&py_str("yesterday last this week weekend morning mornings night nights\n    evening evenings afternoon afternoons ago monday tuesday wednesday thursday friday saturday\n    sunday january february march april june july august september october november december"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AFTER_A_TIME"); }
    s_AFTER_A_TIME(call_builtin("frozenset", &[mcall(&py_str("all after before until till during while once then later earlier\n    afterwards"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.INTRODUCING_A_PERSON"); }
    s_INTRODUCING_A_PERSON(call_builtin("frozenset", &[mcall(&py_str("\n    hi hello hey heya hiya hii hiii yo howdy greetings morning afternoon evening sup meet\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.INTRODUCING_AFTER"); }
    s_INTRODUCING_AFTER(call_builtin("frozenset", &[mcall(&py_str("here from"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.STATES_OF_A_PERSON"); }
    s_STATES_OF_A_PERSON(call_builtin("frozenset", &[mcall(&py_str("hungry thirsty starving famished peckish sleepy tired\n    exhausted shattered knackered drained dizzy queasy nauseous bunged freezing frozen cold\n    hot warm boiling sweaty bored lonely miserable"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.OPENERS"); }
    s_OPENERS(call_builtin("frozenset", &[mcall(&pyrt::xrefs::xget("understanding.lexicon", "OPENERS"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WH_OPENERS"); }
    s_WH_OPENERS(call_builtin("frozenset", &[mcall(&py_str("what why when where who which whose how"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DIGIT_WORDS"); }
    s_DIGIT_WORDS(call_builtin("dict", &[call_builtin("zip", &[py_str("0123456789"), mcall(&py_str("zero one two three four five six seven eight nine"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&g_DIGIT_WORDS(), "update", &[{ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[call_builtin("range", &[Py::Int(10), Py::Int(21)]).unwrap_or_else(|e| exc::raise(e)), mcall(&py_str("ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) { let __unpk3539 = py_unpack(&__item.clone(), 2, None); let mut n: Py = __unpk3539[0].clone(); let mut w: Py = __unpk3539[1].clone(); __out.push((call_builtin("str", &[n.clone()]).unwrap_or_else(|e| exc::raise(e)), w.clone())); } py_dict(__out) })() }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEFORE_WORD"); }
    s_BEFORE_WORD(call_builtin("frozenset", &[mcall(&py_str("the a an my your his her their our its this that these those\nme you him us them it"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEFORE_VERB"); }
    s_BEFORE_VERB(call_builtin("frozenset", &[mcall(&py_str("go get do make see eat cook buy book call ask tell say try keep\nput wear bring take watch read start stop pay save sleep work clean fix check learn study\nwrite send order pick choose plan join find leave stay drink wash boil roast give show help\nmove sort turn cut fill set talk chat walk run live meet visit play listen wait come sit\nstand hold pour chop stir fry bake serve mix add remove park drive catch post share print\nsign email text speak change cancel return swap replace freeze heat store wrap pack load\ngrill steam reheat defrost tidy paint build grow plant have know"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FOR_A"); }
    s_FOR_A(call_builtin("frozenset", &[mcall(&py_str("the a an my your his her their our its this that these those\nme you him us them one two three"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FOR_OBJECT"); }
    s_FOR_OBJECT(call_builtin("frozenset", &[mcall(&py_str("lunch dinner tea breakfast supper brunch now later tonight\ntomorrow today free once example sure good real ages fun sale keeps christmas weekend\neveryone everybody nothing starters afters dessert pudding coffee cake\nmilk bread butter cheese sugar flour rice pasta petrol diesel fuel wine beer juice"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FOR_MEAL"); }
    s_FOR_MEAL(call_builtin("frozenset", &[mcall(&py_str("lunch dinner tea breakfast supper brunch starters afters dessert\npudding cake"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BE_VERBS"); }
    s_BE_VERBS(call_builtin("frozenset", &[mcall(&py_str("am is are was were"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TOO_A"); }
    s_TOO_A(call_builtin("frozenset", &[mcall(&py_str("much many late early soon long often hard fast big small hot cold\ntired busy expensive cheap old young loud noisy sweet salty spicy easy quick heavy light\nslow warm dark bright strong weak soft loud"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TOO_AFTER"); }
    s_TOO_AFTER(call_builtin("frozenset", &[mcall(&py_str("me you us him her them i we they it"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TO_AFTER"); }
    s_TO_AFTER(call_builtin("frozenset", &[mcall(&py_str("up down over back out round through along"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_NEGATION_BEFORE"); }
    s_A_NEGATION_BEFORE(call_builtin("frozenset", &[mcall(&py_str("not"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TOO_A_GRADED"); }
    s_TOO_A_GRADED(op_bitor(&g_TOO_A(), &call_builtin("frozenset", &[mcall(&py_str("bad shabby keen happy far sure worried good pleased glad\n    proud cross upset annoyed safe well fun useful handy honest bright"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.OPERATORS"); }
    s_OPERATORS(call_builtin("frozenset", &[mcall(&py_str("x * / + - plus minus times divided squared"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.COUNTS"); }
    s_COUNTS(call_builtin("frozenset", &[mcall(&py_str("a an per each"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.COUNT_UNITS"); }
    s_COUNT_UNITS(call_builtin("frozenset", &[mcall(&py_str("year month week day hour minute second mile miles kilometre kilo\ngram litre pint cup quid pound euro dollar pence percent point star item person people child\nchildren man men woman women foot feet grand ton bob\nyr yrs mth wks wk hr hrs min mins sec secs km cm mm kg g mg ml cl lb lbs oz ft yds yd\ntsp tbsp\npack slice seat piece wheel toaster suite star speed door\njanuary february march april may june july august september october november december\njan feb mar apr jun jul aug sep sept oct nov dec"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.COMPOUND_HEADS"); }
    s_COMPOUND_HEADS(call_builtin("frozenset", &[mcall(&py_str("bed class bay birth seat"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.COMPOUND_TAILS"); }
    s_COMPOUND_TAILS(call_builtin("frozenset", &[mcall(&py_str("house flat home room rooms car cars van cottage bungalow\ndoor doors window windows drive suite garden"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_COUNTED_AFTER"); }
    s_NOT_COUNTED_AFTER(call_builtin("frozenset", &[mcall(&py_str("and or but so in on at of to by as is was were for with from\nabout than per each every a an the my your his her their our its this that these those\nmore fewer less most other others another few out tomorrow today tonight now later soon twice please\nam pm oclock o'clock o’clock"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_A_COMPOUND_HEAD"); }
    s_NOT_A_COMPOUND_HEAD(call_builtin("frozenset", &[mcall(&py_str("new old small big cheap little large lovely nice good bad\nfirst second third last next own same whole different other another\ni me you him her us them he she we they it"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_A_COMPOUND_TAIL"); }
    s_NOT_A_COMPOUND_TAIL(call_builtin("frozenset", &[mcall(&py_str("i we they he she it you me us them there thats that's\nand or but so if when because though anyway"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_LABEL_BEFORE"); }
    s_A_LABEL_BEFORE(call_builtin("frozenset", &[mcall(&py_str("year years class classes grade grades form forms level levels stage\nstages part parts section sections page pages question questions chapter chapters episode episodes\nlesson lessons unit units term terms step steps"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NUMBERS_BEFORE"); }
    s_NUMBERS_BEFORE(call_builtin("frozenset", &[mcall(&py_str("take took takes taking ate eaten eat drink drank drinks\nbuy bought buys get got gets use used uses add added adds gave give gives\na an the my your his her their our another first last only just"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.REPEAT_ONCE"); }
    s_REPEAT_ONCE(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it me my the a an to and of in is am are\nwas were do does not at on for"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.REPEAT_ONCE_SUBJECTS"); }
    s_REPEAT_ONCE_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it u"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AUX_OPENS"); }
    s_AUX_OPENS(call_builtin("frozenset", &[mcall(&py_str("am is are was were have has had been"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DROPPED_SUBJECT_AUX"); }
    s_DROPPED_SUBJECT_AUX(call_builtin("frozenset", &[mcall(&py_str("am was have"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._SUBJECTS"); }
    s__SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it there that this these those one anyone someone\n    everyone nobody nothing none who which what"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._PREDICATE_ONLY"); }
    s__PREDICATE_ONLY(call_builtin("frozenset", &[mcall(&py_str("going gonna off out away back home up down well knackered\n    shattered exhausted drained starving fine alright okay ok rubbish great late here"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_STATE_OF_THE_PERSON"); }
    s__A_STATE_OF_THE_PERSON(call_builtin("frozenset", &[mcall(&py_str("hungry thirsty starving famished peckish busy rushed\n    freezing frozen broke skint brassic sleepy\n    dizzy tired shattered knackered exhausted drained ill unwell poorly sick lousy rough\n    down low fed up miserable sad happy chuffed pleased livid fuming cross annoyed upset gutted\n    relieved worried anxious nervous scared frightened stressed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._SUBJECTLESS_AUX"); }
    s__SUBJECTLESS_AUX(call_builtin("frozenset", &[mcall(&py_str("am was were"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._STATE_FILLER"); }
    s__STATE_FILLER(call_builtin("frozenset", &[mcall(&py_str("both so really very totally absolutely properly too all quite still\n    always just completely dead kind of a bit"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._STATE_TAIL"); }
    s__STATE_TAIL(call_builtin("frozenset", &[mcall(&py_str("at in on for after before by since till until from about with around\n    over through during the a an this that these those my your his her their our\n    last next today tonight tomorrow yesterday week weekend morning afternoon evening night\n    month year day now then again still too also already anymore lately recently though but and\n    so because when while if all every up out off sick stiff rotten\n    to be been being had got go going"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._SUBJECTLESS_ONLY"); }
    s__SUBJECTLESS_ONLY(call_builtin("frozenset", &[mcall(&py_str("am was"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._HAVE_NEGATIONS"); }
    s__HAVE_NEGATIONS(call_builtin("frozenset", &[mcall(&py_str("no none nothing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._HAVE_PARTICIPLES"); }
    s__HAVE_PARTICIPLES(call_builtin("frozenset", &[mcall(&py_str("got had been seen made taken spent"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._HAVE_THING"); }
    s__HAVE_THING(call_builtin("frozenset", &[mcall(&py_str("a an the my our his her their your some this that"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._HAVE_ASKING"); }
    s__HAVE_ASKING(call_builtin("frozenset", &[mcall(&py_str("minute moment sec second mo chance word"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._AFTER_PARTICIPLE"); }
    s__AFTER_PARTICIPLE(call_builtin("frozenset", &[mcall(&py_str("of to for in on at with about from by up out off over through\n    into onto and but so or because if when while then that than as not never ever too also\n    just"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CAUSE_A_SUBJECT_FOLLOWS"); }
    s_CAUSE_A_SUBJECT_FOLLOWS(call_builtin("frozenset", &[mcall(&py_str("i im ive id ill you youre youve we were weve they theyre\n    theyve he hes she shes it its that thats this these those there theres my your our his her\n    their if when whenever while since as so and but or not never no nobody nothing everyone\n    everybody someone something anyone anything am is are was were do does did dont doesnt did\n    didnt cant cannot couldnt wont wouldnt shouldnt isnt wasnt arent hasnt havent hadnt u ur ya"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CAUSE_A_CLAUSE_VERB"); }
    s_CAUSE_A_CLAUSE_VERB(call_builtin("frozenset", &[mcall(&py_str("is was were are am has had have will would can could did does\n    do dont doesnt didnt wont cant cannot isnt wasnt arent hasnt hadnt"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CAUSE_BEFORE_A_NOUN"); }
    s_CAUSE_BEFORE_A_NOUN(call_builtin("frozenset", &[mcall(&py_str("the a an this that my your our his her their any no every some another one"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CAUSE_BEFORE_A_VERB"); }
    s_CAUSE_BEFORE_A_VERB(call_builtin("frozenset", &[mcall(&py_str("to could would can may might must should will shall does did do doesnt didnt dont wont cant cannot never not"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CAUSE_A_DEMONSTRATIVE"); }
    s_CAUSE_A_DEMONSTRATIVE(call_builtin("frozenset", &[mcall(&py_str("this that these those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ONE_WORD_FOODS"); }
    s_ONE_WORD_FOODS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z])chick pea(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)), py_str("chickpea")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z])chick peas(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)), py_str("chickpeas")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z])sweet corn(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)), py_str("sweetcorn")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.JOINED_WORDS"); }
    s_JOINED_WORDS(stdlib_call("re", "compile", &[op_modulo(&py_str("(?<![a-z])(?:%s)(?![a-z])"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("understanding.lexicon", "JOINED_WORDS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SPLIT_CONTRACTION"); }
    s_SPLIT_CONTRACTION(stdlib_call("re", "compile", &[py_str("\\b(i|u|you|he|she|it|we|they|that|there|what|who|where|when|how|this|these|those|let)\\s+(s|m|re|ve|ll|d)\\b(?=\\s|[.,!?;:)]|$)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SPLIT_NOT"); }
    s_SPLIT_NOT(stdlib_call("re", "compile", &[py_str("\\b([a-z]{2,}n)\\s+t\\b(?=\\s|[.,!?;:)]|$)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_WORDS"); }
    s_NOT_WORDS(call_builtin("frozenset", &[mcall(&py_str("dont doesnt didnt isnt wasnt arent werent havent hasnt hadnt cant couldnt\n    shouldnt wouldnt wont mustnt neednt aint shant mightnt"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.UP_TOO_ASKS"); }
    s_UP_TOO_ASKS(stdlib_call("re", "compile", &[py_str("(\\b(?:what|whats|watcha|whatcha|wotcha)\\b[^.!?]{0,24}?\\b)up too\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HEART_MEASURES"); }
    s_HEART_MEASURES(mcall(&py_str("so really very too pretty quite still just totally completely absolutely\n    utterly properly well a bit a little kind of sort of that this"), "split", &[]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HEART_FEELINGS"); }
    s_HEART_FEELINGS(py_tuple(vec![py_tuple(vec![py_str("heartbroken"), py_tuple(vec![py_str("broken"), py_str("breaking"), py_str("shattered"), py_str("crushed"), py_str("torn"), py_str("broke"), py_str("snapped"), py_str("in pieces"), py_str("in bits"), py_str("in two")])]), py_tuple(vec![py_str("disappointed"), py_tuple(vec![py_str("sank"), py_str("sinking"), py_str("dropped")])])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HEART_SAID_AS_A_FEELING"); }
    s_HEART_SAID_AS_A_FEELING(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HEART_FEELINGS()) { let __unpk7002 = py_unpack(&__item.clone(), 2, None); let mut feeling: Py = __unpk7002[0].clone(); let mut states: Py = __unpk7002[1].clone(); __out.push(py_tuple(vec![stdlib_call("re", "compile", &[op_modulo(&py_str("(?<![a-z0-9])(?:(?:my|me)\\s+heart(?:'s|s)?\\s+(?:(?:is|was|feels|felt|looks)\\s+)?(?:(?:%s)\\s+)*(?:%s)\\b(?!\\s+open)(?:\\s+(?:in\\s+(?:two|bits|pieces)|into\\s+(?:a\\s+million\\s+)?pieces|to\\s+pieces))?|(?:[a-z']+\\s+)?(?:broke|breaks|breaking|broken)\\s+(?:my|me)\\s+heart)"), &py_tuple(vec![mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HEART_MEASURES()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&states.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])]))]).unwrap_or_else(|e| exc::raise(e)), op_add(&op_add(&py_str(" i am "), &feeling.clone()), &py_str(" "))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SAID_AS_A_FEELING"); }
    s_SAID_AS_A_FEELING(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z0-9])broken[\\s-]?hearted(?![a-z0-9])")]).unwrap_or_else(|e| exc::raise(e)), py_str(" heartbroken ")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z0-9])broken[\\s-]?heart(?![a-z0-9])")]).unwrap_or_else(|e| exc::raise(e)), py_str(" heartbroken ")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z0-9])heart[\\s-]?broken(?![a-z0-9])")]).unwrap_or_else(|e| exc::raise(e)), py_str(" heartbroken ")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z0-9])(?:my|me)\\s+(?:whole\\s+)?(?:world|life)\\s+(?:fell|has\\s+fallen|had\\s+fallen|is\\s+falling|came|came\\s+crashing)\\s+(?:apart|down|crashing\\s+down)(?![a-z0-9])")]).unwrap_or_else(|e| exc::raise(e)), py_str(" i am sad ")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z0-9])(?:(?:my|me)\\s+)?heart(?:'s|s)?\\s+(?:(?:is|was|feels|felt)\\s+)?in\\s+(?:my|me)\\s+(?:mouth|boots|throat|stomach|shoes)(?![a-z0-9])")]).unwrap_or_else(|e| exc::raise(e)), py_str(" i am nervous ")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("(?<![a-z0-9])(?:im|i\\s+am|i\\s+feel|i\\s+felt|i\\s+was)\\s+(?:so\\s+|really\\s+|totally\\s+|completely\\s+|utterly\\s+|absolutely\\s+)?crushed(?![a-z0-9])(?!\\s+by\\b)")]).unwrap_or_else(|e| exc::raise(e)), py_str(" i am sad ")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SAID_IT"); }
    s_SAID_IT(stdlib_call("re", "compile", &[py_str("\\bwhat (?:was|is) it (?:that )?(?P<who>i|we) (?:said|told you|mentioned)\\b|\\bwhat (?:was|is) it (?:that )?(?P<saying>i|we) (?:was|were|am|are) (?:saying|telling you)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_CLOSING_WORD"); }
    s__A_CLOSING_WORD(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ME_FOR_MY_CLOSES()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_HEAD_IN_THE_IDIOM"); }
    s__A_HEAD_IN_THE_IDIOM(stdlib_call("re", "compile", &[py_str("(?<![a-z0-9])me head(?=\\s+in(?![a-z0-9]))")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_CLIPPED_HEAD"); }
    s__A_CLIPPED_HEAD(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![a-z0-9])(?:me|my) (?:ed|ead)(?!s?(?:[a-z0-9]))(?=\\s+(?:"), &g__A_CLOSING_WORD()), &py_str(")(?![a-z0-9])|\\s*$|\\s*[,.;:!?])"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PUNCT_BREAK"); }
    s_PUNCT_BREAK(stdlib_call("re", "compile", &[py_str("\\s*[,;:!?]\\s*")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SENTENCE_END"); }
    s_SENTENCE_END(stdlib_call("re", "compile", &[py_str("(?<=[a-z0-9]{2})\\.(?=\\s|$)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TOKEN"); }
    s_TOKEN(stdlib_call("re", "compile", &[op_add(&g__MEASURE(), &py_str("|\\|"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THOUSANDS"); }
    s_THOUSANDS(stdlib_call("re", "compile", &[py_str("(\\d),(?=\\d\\d\\d\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THOUSANDS_SAID_WITH_A_SPACE"); }
    s_THOUSANDS_SAID_WITH_A_SPACE(stdlib_call("re", "compile", &[py_str("(?<!\\d)(?<!\\d{4} )(\\d{1,3})((?:[ \\u00a0]\\d{3})+)(?!\\d)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CURRENCY_SIGNS"); }
    s_CURRENCY_SIGNS(py_dict(vec![(py_str("£"), py_str("quid")), (py_str("$"), py_str("dollars")), (py_str("€"), py_str("euros"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CURRENCY_SIGN"); }
    s_CURRENCY_SIGN(stdlib_call("re", "compile", &[py_str("([\\u00a3$\\u20ac])\\s*(\\d+(?:[.,]\\d+)*)\\s*([km])?(?![\\w])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CURRENCY_FACTOR"); }
    s_CURRENCY_FACTOR(py_dict(vec![(py_str("k"), Py::Int(1000)), (py_str("m"), Py::Int(1000000))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_A_THING"); }
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\na an the this that these those my your his her its our their some any no every each another\ni you he she it we they me him us them who whom whose which what when where why how\nis am are was were be been being do does did have has had can could will would shall should\nmay might must not so as than then too also either neither nor and or but if because\nwhile whilst until till unless whether whenever wherever though although since whereas\nin on at to for with from by about into over under after before between through during\nup down out off away here there now later soon again still just only even ever never always\nplease thanks sorry ok okay\nmyself yourself himself herself itself ourselves yourselves themselves\nthinks\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AFTER_A_THING"); }
    s_AFTER_A_THING(op_bitor(&op_bitor(&op_bitor(&g_BE_WORDS(), &g_AUX_WORDS()), &g_REPORTING()), &call_builtin("frozenset", &[g_DIGIT_WORDS()]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEFORE_A_THING"); }
    s_BEFORE_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nwith for about of on in at from to over under into by off near round around behind\nand but so then also plus because when if or though although since\nwiv wid wif abt frm\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEFORE_A_PREPOSITION"); }
    s_BEFORE_A_PREPOSITION(call_builtin("frozenset", &[mcall(&py_str("\nwith for about of on in at from to over under into by off near round around behind\nwiv wid wif abt frm\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_A_THING_ENDING"); }
    s_NOT_A_THING_ENDING(py_tuple(vec![py_str("ed"), py_str("ing")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.LONG_ENOUGH_TO_BE_A_VERB"); }
    s_LONG_ENOUGH_TO_BE_A_VERB(py_dict(vec![(py_str("ed"), Py::Int(5)), (py_str("ing"), Py::Int(6))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ING_THINGS"); }
    s_ING_THINGS(call_builtin("frozenset", &[mcall(&py_str("\nceiling ceilings landing landings building buildings bedding wedding weddings pudding\npuddings dressing dressings shopping evening evenings morning mornings washing parking\nmeeting meetings ceiling fencing railing railings\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DOING_NOT_THING"); }
    s_DOING_NOT_THING(call_builtin("frozenset", &[mcall(&py_str("\nshut went goes gone came come left leaves broke broken breaks keeps kept got get gives gave\nsaid says say told tells tell makes made takes took put puts ran runs run fell falls sees saw\nseen knew knows know thought thinks think found finds feel feels felt closes close opens open\nstops stop starts start works work sits sit"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.KIN"); }
    s_KIN(call_builtin("frozenset", &[mcall(&py_str("\nmother mum mom mam mammy mummy father dad daddy son daughter sister brother wife husband\npartner nan nana granny gran grandma grandad grandpa grandmother grandfather aunt auntie uncle\ncousin niece nephew kid kids child children baby toddler boyfriend girlfriend fiance fiancee\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_A_THING_AFTER"); }
    s_NOT_A_THING_AFTER(op_bitor(&g_NOT_A_THING(), &call_builtin("frozenset", &[mcall(&py_str("\nfirst last next once twice today tonight tomorrow yesterday afterwards afternoons personally\nanyway anyhow anymore anymore later sooner honestly really actually probably maybe usually\nnormally mostly nearly almost quite rather fairly simply clearly obviously certainly\ndefinitely absolutely totally completely entirely exactly precisely again yet ever still\nboth too either neither together alike forever\nalone sometimes earlier meanwhile eventually awhile anyway\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TIME_WORDS"); }
    s_TIME_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nmonday tuesday wednesday thursday friday saturday sunday\njanuary february march april may june july august september october november december\nmorning afternoon evening night tonight tomorrow today yesterday midday noon midnight\nweek weekend weekday month year hour minute\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PLACE_WORDS"); }
    s_PLACE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\noutside inside indoors outdoors upstairs downstairs home ahead nearby nowhere somewhere\nanywhere everywhere\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_A_THING_PAIR"); }
    s_NOT_A_THING_PAIR(call_builtin("frozenset", &[py_tuple(vec![py_str("part time"), py_str("full time"), py_str("half time"), py_str("day time"), py_str("night time"), py_str("real time"), py_str("spare time"), py_str("free time"), py_str("every time")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.OWNING_VERBS"); }
    s_OWNING_VERBS(call_builtin("frozenset", &[mcall(&py_str("\nlost lose losing dropped drop broke broken break cut burnt burned burn forgot forget forgotten\nleft spilled spilt ripped tore torn smashed chipped scratched hurt banged stubbed twisted\nstrained scraped find finding wash washing washed ironed ironing doing doin\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ASKS_WHOSE"); }
    s_ASKS_WHOSE(call_builtin("frozenset", &[mcall(&py_str("where wheres what whats wher whers wer wers"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUN_WORDS"); }
    s_SUN_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nbed beds cream creams lounger loungers tan tans burn burns burnt burned hat hats roof roofs\nlight lights shine shines shining shiny screen screens glasses lotion deck decks rise set\nheat trap traps flower flowers dial lamp star stroke dried dry baked bathing baked\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.OWNING_A_PERSON"); }
    s_OWNING_A_PERSON(call_builtin("frozenset", &[mcall(&py_str("my our their your his her me"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SON_VERBS"); }
    s_SON_VERBS(op_bitor(&g_AUX_WORDS(), &call_builtin("frozenset", &[mcall(&py_str("\ngot gotten gets get keeps keep kept went goes gone came comes coming said says saying told tells\ntelling feels feel felt seems seem seemed looks look looked needs need needed wants want wanted\nhates hate hated loves love loved likes like liked hurts hurt hurting cuts cut broke broken breaks\nstarted starts woke wakes woken sleeps slept eats ate eating drinks drank drinking walks walked\ntalks talked cries cried crying swims swam swim wont cant doesnt didnt isnt wasnt hasnt havent\ncouldnt wouldnt shouldnt\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.LOOSE_FOR_LOSE"); }
    s_LOOSE_FOR_LOSE(py_dict(vec![(py_str("loose"), py_str("lose")), (py_str("looses"), py_str("loses")), (py_str("loosing"), py_str("losing")), (py_str("loosed"), py_str("lost"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THE_LOSING"); }
    s_THE_LOSING(call_builtin("frozenset", &[mcall(&py_str("weight pounds lb lbs kilos kilo kg kgs stones stone inches inch\n    centimetres centimeters cm"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEFORE_THE_LOSING"); }
    s_BEFORE_THE_LOSING(call_builtin("frozenset", &[mcall(&py_str("some a an any that the this bit little more few of\n    couple coupleof stone\n    # \"sum\" is \"some\" with the o left out, and it stands where \"some\" stands: \"i gotta loose sum\n    # weight b4 the weddin\" was \"Got it. Is there anything you'd like a hand with?\" because the\n    # word between \"loose\" and \"weight\" was not one the reader knew, while \"i gotta lose some\n    # weight before the wedding\" gets the weight answer.\n    sum"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_PART_THE_PAIN_IS_IN"); }
    s_A_PART_THE_PAIN_IS_IN(call_builtin("frozenset", &[mcall(&py_str("chest stomach tummy throat ear ears tooth teeth jaw shoulder\n    shoulders hip hips neck wrist ankle elbow heel knee knees ribs rib kidney kidneys groin\n    armpit chests leg legs arm arms foot feet hand hands head"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_GLASS_OR_A_PAN"); }
    s_A_GLASS_OR_A_PAN(call_builtin("frozenset", &[mcall(&py_str("window windows windscreen windscreens glass door doors\n    frame frames greenhouse glazing mirror mirrors skylight skylights leaded stained shutter\n    shutters cupboard drawer drawers kitchen oven hob handle handles lid lids frying roasting\n    saucepan nonstick teflon wok tray freezer freezers"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_PART_A_PERSON_OWNS"); }
    s_A_PART_A_PERSON_OWNS(call_builtin("frozenset", &[mcall(&py_str("back side head belly gut upper lower middle shoulder\n    tummy chest arm arms leg legs foot feet hand hands"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_PAIN_WORD"); }
    s_A_PAIN_WORD(py_dict(vec![(py_str("pan"), py_str("pain")), (py_str("pane"), py_str("pain")), (py_str("panes"), py_str("pains"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_KEEPING_A_THING"); }
    s_A_KEEPING_A_THING(call_builtin("frozenset", &[mcall(&py_str("keep keeps kept store stored stores storing hang hangs hung\n    put puts putting stick stuck stash stashed"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THE_HEART_SAID_THE_OTHER_WAY"); }
    s_THE_HEART_SAID_THE_OTHER_WAY(call_builtin("frozenset", &[mcall(&py_str("attack attacks racing pounding murmur murmer murmurs\n    failure bypass flutter flutters palpitations transplant stent pacemaker cardiologist\n    valve valves arrest disease angina ecg"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_CHOKE_SAID_THE_OTHER_WAY"); }
    s_A_CHOKE_SAID_THE_OTHER_WAY(call_builtin("frozenset", &[mcall(&py_str("think he she hes shes im i we they baby son daughter child\n    kid kids boy girl man woman dad mum someone somebody anyone anyone food blue bit sweet grape\n    nearly started being"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_THING_WORN"); }
    s_A_THING_WORN(call_builtin("frozenset", &[mcall(&py_str("wedding funerals funeral party interview dress dressed suit suits\n    shoes shoe coat coats jeans jacket jackets gym tonight outside today work smart casual\n    layers boots hat"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_STATE_YOU_CAN_FEEL"); }
    s_A_STATE_YOU_CAN_FEEL(call_builtin("frozenset", &[mcall(&py_str("sick ill tired low anxious depressed dizzy faint better\n    worse fine ok okay good great awful terrible rough rubbish angry sad happy nervous scared\n    worried guilty lonely numb hot cold shaky weepy tearful emotional strange odd funny like\n    sickly unwell pleased proud relieved"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_COPULA"); }
    s_A_COPULA(call_builtin("frozenset", &[mcall(&py_str("am is are was were be been get got gets getting really very quite so\n    still bit too pretty rather always"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEFORE_A_JOB"); }
    s_BEFORE_A_JOB(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("the"), py_str("this"), py_str("a")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_JOB"); }
    s_A_JOB(call_builtin("frozenset", &[mcall(&py_str("job jobs work role position post company school course flat house city\n    country marriage"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_TIME_TO_LEAVE"); }
    s_A_TIME_TO_LEAVE(call_builtin("frozenset", &[mcall(&py_str("in next on at by from after before tomorrow tonight soon then end\n    month week year monday tuesday wednesday thursday friday saturday sunday january february\n    march april may june july august september october november december"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DIGIT_AS_WORD"); }
    s_DIGIT_AS_WORD(py_dict(vec![(py_str("2"), py_str("to")), (py_str("4"), py_str("for"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MOTION_BEFORE"); }
    s_MOTION_BEFORE(call_builtin("frozenset", &[mcall(&py_str("go goes going went gone come comes coming came back off\n    goin gona gonna"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DEMONSTRATIVE"); }
    s_DEMONSTRATIVE(call_builtin("frozenset", &[py_tuple(vec![py_str("this"), py_str("that"), py_str("these"), py_str("those")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_PERIOD_FOLLOWS"); }
    s_A_PERIOD_FOLLOWS(call_builtin("frozenset", &[mcall(&py_str("time times week weeks weekend month months year years\n    morning afternoon evening night term semester summer winter spring autumn day days\n    hour hours minute minutes second seconds"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("your"), py_str("his"), py_str("her"), py_str("our"), py_str("their"), py_str("you"), py_str("u"), py_str("ur"), py_str("it"), py_str("me"), py_str("us"), py_str("him"), py_str("them"), py_str("myself"), py_str("what"), py_str("how"), py_str("why"), py_str("where"), py_str("when"), py_str("who")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_WORD_CAN_FOLLOW"); }
    s_A_WORD_CAN_FOLLOW(call_builtin("frozenset", &[__data3()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("i"), py_str("u"), py_str("you"), py_str("we"), py_str("they"), py_str("he"), py_str("she"), py_str("anyone"), py_str("any1"), py_str("ne1"), py_str("someone"), py_str("some1"), py_str("everyone"), py_str("every1"), py_str("ppl"), py_str("people")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.KNOW_SUBJECT"); }
    s_KNOW_SUBJECT(call_builtin("frozenset", &[__data4()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("not"), py_str("dont"), py_str("don't"), py_str("didnt"), py_str("didn't"), py_str("doesnt"), py_str("doesn't"), py_str("dnt"), py_str("havent"), py_str("haven't"), py_str("havnt"), py_str("aint"), py_str("ain't"), py_str("never")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_BEFORE_KNOW"); }
    s_NOT_BEFORE_KNOW(call_builtin("frozenset", &[__data5()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data6() -> Py { py_tuple(vec![py_str("you"), py_str("u"), py_str("ur"), py_str("it"), py_str("me"), py_str("us"), py_str("him"), py_str("them"), py_str("i"), py_str("we"), py_str("he"), py_str("she"), py_str("they"), py_str("what"), py_str("how"), py_str("why"), py_str("were"), py_str("wer"), py_str("where"), py_str("when"), py_str("who"), py_str("that"), py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("your"), py_str("his"), py_str("her"), py_str("our"), py_str("their"), py_str("something"), py_str("anything"), py_str("nothing"), py_str("yet"), py_str("if"), py_str("whether"), py_str("about"), py_str("for"), py_str("much"), py_str("many"), py_str("anyone"), py_str("anybody"), py_str("anymore"), py_str("either"), py_str("myself")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_VERB_CAN_FOLLOW"); }
    s_A_VERB_CAN_FOLLOW(call_builtin("frozenset", &[__data6()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BE_FORMS"); }
    s_BE_FORMS(call_builtin("frozenset", &[mcall(&py_str("am is are was were"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HAVE_FORMS"); }
    s_HAVE_FORMS(call_builtin("frozenset", &[mcall(&py_str("have has had having"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MODAL_FORMS"); }
    s_MODAL_FORMS(call_builtin("frozenset", &[mcall(&py_str("could would should must might may ought shall will can"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.LEFT_OFF_CONTRACTION"); }
    s_LEFT_OFF_CONTRACTION(py_dict(vec![(py_str("were"), py_tuple(vec![py_str("we"), py_str("are")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CONTRACTION_NEXT"); }
    s_CONTRACTION_NEXT(op_bitor(&g_BEFORE_VERB(), &call_builtin("frozenset", &[mcall(&py_str("\n    be been being have had not never also just only still both already always definitely\n    probably going off back home away\n    having moving getting thinking trying staying doing seeing planning hoping\n    let ring phone pop drop grab manage chase quit resign hire rent lock measure weigh need\n    love like want miss mean mind aim assume bet guess hope wish promise remind stay sow weed\n    mow dig sand tile grout seal glue screw drill hang level varnish log batch bottle\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PLURAL_SUBJECTS"); }
    s_PLURAL_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i you we they"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THIRD_PERSON"); }
    s_THIRD_PERSON(call_builtin("frozenset", &[mcall(&py_str("he she it"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PERSON_WORDS"); }
    s_PERSON_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    i you he she we they it mum mums mom moms mother mother's fathers dad dads father sister\n    sisters brother brothers son sons daughter daughters wife husband partner mates mate\n    friend friends boss kid kids baby gran nans nan grandad grandma granny aunt uncle cousin\n    neighbour neighbor neighbour's colleague doctor nurse teacher cat cats dog dogs puppy\n    puppies kitten kittens rabbit rabbits hamster budgie bird birds horse horses fish\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PERSON_PREPS"); }
    s_PERSON_PREPS(call_builtin("frozenset", &[mcall(&py_str("in at on to out back away off home there round around up down"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HOW_FOLLOWS"); }
    s_HOW_FOLLOWS(call_builtin("frozenset", &[mcall(&py_str("much many long far old often big deep quick soon fast come\n    did do does can could would should is are was were about of to lot"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEEN_WORDS"); }
    s_BEEN_WORDS(call_builtin("frozenset", &[mcall(&py_str("sick ill unwell poorly bad awful great good alright quiet busy\n    late away round back out down off twice"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEEN_SUBJECT"); }
    s_BEEN_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it that thats its hes shes this these those\n    there which what who everything nothing something"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_POSSESSIVES"); }
    s_THING_POSSESSIVES(call_builtin("frozenset", &[mcall(&py_str("my our your his her their its"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.IN_NOUNS"); }
    s_IN_NOUNS(call_builtin("frozenset", &[mcall(&py_str("\ncabin robin basin margin satin ruin twin grin coin pumpkin napkin muffin raisin resin\nvitamin protein aspirin insulin cousin dolphin penguin violin mandolin origin virgin cabbage\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BIN_CONTAINERS"); }
    s_BIN_CONTAINERS(call_builtin("frozenset", &[mcall(&py_str("\nkitchen compost food wheelie recycling waste rubbish trash dust garden nappy nappies sharps\nbathroom pedal swing carrier paper\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.IRREGULAR_PARTICIPLES"); }
    s_IRREGULAR_PARTICIPLES(call_builtin("frozenset", &[mcall(&py_str("\n    been gone done had seen said made taken given told known written eaten drunk spoken broken\n    chosen driven fallen flown forgotten got gotten hidden held kept left lost met paid put read\n    run come become begun brought bought caught fought found heard meant sent shown slept spent\n    stood taught thought understood won worn\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_dict(vec![(py_str("mums"), py_str("mum")), (py_str("mams"), py_str("mam")), (py_str("moms"), py_str("mom")), (py_str("dads"), py_str("dad")), (py_str("husbands"), py_str("husband")), (py_str("wives"), py_str("wife")), (py_str("partners"), py_str("partner")), (py_str("nans"), py_str("nan")), (py_str("grans"), py_str("gran")), (py_str("grandmas"), py_str("grandma")), (py_str("grandads"), py_str("grandad")), (py_str("mothers"), py_str("mother")), (py_str("fathers"), py_str("father")), (py_str("boyfriends"), py_str("boyfriend")), (py_str("girlfriends"), py_str("girlfriend"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.KIN_SINGULAR"); }
    s_KIN_SINGULAR(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.KIN_HAS"); }
    s_KIN_HAS(call_builtin("frozenset", &[mcall(&py_str("got had been gone"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.KIN_IS"); }
    s_KIN_IS(call_builtin("frozenset", &[mcall(&py_str("\n    on in at off out away here back home due around\n    pregnant engaged married divorced retired widowed ill sick poorly unwell upset fine ok okay\n    worried tired late early better worse mad angry happy sad lonely dead alive well drunk hungover\n    coming going staying moving working getting having being doing feeling expecting visiting\n    arriving leaving starting recovering struggling coping waiting looking planning hoping trying\n    applying retraining babysitting sitting standing living travelling traveling\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.KIN_FILLERS"); }
    s_KIN_FILLERS(call_builtin("frozenset", &[mcall(&py_str("just still always now already recently never not really very so quite"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CAN_FOLLOW_A_VERB"); }
    s_CAN_FOLLOW_A_VERB(call_builtin("frozenset", &[mcall(&py_str("\n    a an the this that these those my his her their our your its\n    in on at to from with about for of into onto over under up down out off back home\n    away around round through across along behind past near\n    me him her them us it you\n    and but or so because when while if then than\n    today tonight tomorrow now again soon later already still just always never ever not\n    late early hard well badly properly slowly quickly together alone anymore\n    fine ok okay alright better worse\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FAMILY_SHRT"); }
    s_FAMILY_SHRT(py_dict(vec![(py_str("mil"), py_tuple(vec![py_str("mother"), py_str("in"), py_str("law")])), (py_str("fil"), py_tuple(vec![py_str("father"), py_str("in"), py_str("law")])), (py_str("sil"), py_tuple(vec![py_str("sister"), py_str("in"), py_str("law")])), (py_str("bil"), py_tuple(vec![py_str("brother"), py_str("in"), py_str("law")])), (py_str("oh"), py_tuple(vec![py_str("other"), py_str("half")])), (py_str("lo"), py_tuple(vec![py_str("little"), py_str("one")])), (py_str("dd"), py_tuple(vec![py_str("daughter")])), (py_str("ds"), py_tuple(vec![py_str("son")])), (py_str("dh"), py_tuple(vec![py_str("husband")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SHRT_THINGS"); }
    s_SHRT_THINGS(py_dict(vec![(py_str("ds"), call_builtin("frozenset", &[mcall(&py_str("broken broke charging charge charger battery screen console game games\n        lite dead working works"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("dd"), call_builtin("frozenset", &[mcall(&py_str("bra cup size in january february march april may june july august\n        september october november december"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.POSSESSIVES"); }
    s_POSSESSIVES(call_builtin("frozenset", &[mcall(&py_str("my your his her our their ur me mums dads sons daughters"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PERIOD_COUNTS"); }
    s_PERIOD_COUNTS(call_builtin("frozenset", &[mcall(&py_str("week weeks month months year years hour hours minute minutes"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CLIPPED_UNITS"); }
    s_CLIPPED_UNITS(py_dict(vec![(py_str("yr"), py_tuple(vec![py_str("year"), py_str("years")])), (py_str("hr"), py_tuple(vec![py_str("hour"), py_str("hours")])), (py_str("min"), py_tuple(vec![py_str("minute"), py_str("minutes")])), (py_str("sec"), py_tuple(vec![py_str("second"), py_str("seconds")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CLIPPED_THINGS"); }
    s_CLIPPED_THINGS(py_dict(vec![(py_str("rad"), py_tuple(vec![py_str("radiator"), py_str("radiators")])), (py_str("rads"), py_tuple(vec![py_str("radiator"), py_str("radiators")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_AFTER"); }
    s_THING_AFTER(call_builtin("frozenset", &[mcall(&py_str("is are was were wont wont keeps keep needs need broke broken leaking leaky\n    cold hot noisy warm blocked bleed bleeding clean cleaning fix fixed valve valves\n    died dies dead stopped stopping working works faulty glitchy tinny quiet sounds sound\n    cuts cutting muted mute drops dropped plug plugged cable battery"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_JOIN"); }
    s_THING_JOIN(call_builtin("frozenset", &[mcall(&py_str("and but so though although because when if then"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_PLACE_AFTER"); }
    s_THING_PLACE_AFTER(call_builtin("frozenset", &[mcall(&py_str("in on at"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_DETERMINER"); }
    s_THING_DETERMINER(call_builtin("frozenset", &[mcall(&py_str("a an the my our your his her their its both each every no any\n    this these those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_PLACE_WORDS"); }
    s_THING_PLACE_WORDS(call_builtin("frozenset", &[mcall(&py_str("a an the my our your his her their its this these those that"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THING_SUBJECT"); }
    s_THING_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("a an the my our your his her their its this these those both\n    each every no any leaky leaking cold hot noisy warm blocked old spare"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_COUNT"); }
    s_A_COUNT(call_builtin("frozenset", &[mcall(&py_str("one two three four five six seven eight nine ten eleven twelve\n    thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty thirty forty fifty\n    sixty ninety a an couple few several hundred thousand million"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_PARTS_OF_A_PERIOD"); }
    s_A_PARTS_OF_A_PERIOD(call_builtin("frozenset", &[mcall(&py_str("next last this every each per past coming previous\n    whole"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HR_IS_THE_DEPARTMENT_AFTER"); }
    s_HR_IS_THE_DEPARTMENT_AFTER(call_builtin("frozenset", &[mcall(&py_str("department departments team teams policy policies\n    advice manager managers meeting meetings issue issues people rep reps email emailed staff\n    director directors"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_BEFORE_A_NOUN"); }
    s_NOT_BEFORE_A_NOUN(call_builtin("frozenset", &[mcall(&py_str("a an the my your his her our their its this that these\n    of in at on to for from with per and or so but then than ago later old olds off out up\n    down back over about around by as if sleep nap rest wait left more less most least\n    now today tonight tomorrow yesterday"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_FEELING_CAN_BE_FOLLOWED"); }
    s_A_FEELING_CAN_BE_FOLLOWED(call_builtin("frozenset", &[mcall(&py_str("at in on with about of today tonight now again lately\n    atm tbh so as all time work home stiff shit"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BEFORE_A_FEELING"); }
    s_BEFORE_A_FEELING(call_builtin("frozenset", &[mcall(&py_str("so really very well still always constantly dead proper\n    pretty totally completely"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AINT_FORMS"); }
    s_AINT_FORMS(call_builtin("frozenset", &[py_tuple(vec![py_str("aint"), py_str("ain't"), py_str("aint'"), py_str("a'int"), py_str("ai'nt"), py_str("aint,")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HAS_SUBJECT"); }
    s_HAS_SUBJECT(op_bitor(&g_THIRD_PERSON(), &call_builtin("frozenset", &[mcall(&py_str("this that there everyone somebody someone nobody"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AINT_PLURAL"); }
    s_AINT_PLURAL(call_builtin("frozenset", &[mcall(&py_str("you we they ye these those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MISSING_E_AFTER"); }
    s_MISSING_E_AFTER(call_builtin("frozenset", &[py_str("ktvpsmfgxzcjq")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PAST_AS_PARTICIPLE"); }
    s_PAST_AS_PARTICIPLE(call_builtin("frozenset", &[mcall(&py_str("spoke broke woke drank wrote rang sang began swam took gave\n    ate fell forgot stole drove rode wore chose froze ran came grew knew threw flew drew"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HV_SUBJECTS"); }
    s_HV_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i we you they u ye"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HV_HELPERS"); }
    s_HV_HELPERS(op_bitor(&g_MODAL_FORMS(), &call_builtin("frozenset", &[mcall(&py_str("to do does did not"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FIRST_PERSON_BE"); }
    s_FIRST_PERSON_BE(call_builtin("frozenset", &[py_tuple(vec![py_str("im"), py_str("i'm"), py_str("i’m"), py_str("am"), py_str("m")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AMOUNT_BEFORE_A_STATE"); }
    s_AMOUNT_BEFORE_A_STATE(call_builtin("frozenset", &[mcall(&py_str("just only still so really very pretty quite too not never\n    always often all sometimes ever constantly honestly actually kinda rather super totally\n    completely proper propper dead well awful bloody terribly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BE_BEFORE_A_STATE"); }
    s_BE_BEFORE_A_STATE(call_builtin("frozenset", &[mcall(&py_str("im i'm i’m m am i been being feel feels feeling feelin felt"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THIRD_PERSON_BE"); }
    s_THIRD_PERSON_BE(call_builtin("frozenset", &[mcall(&py_str("is are was were r re hes he's shes she's its it's theyre they're\n    youre you're ure we're seems seem looks look"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BARE_TO"); }
    s_BARE_TO(call_builtin("frozenset", &[py_tuple(vec![py_str("to"), py_str("two"), py_str("2")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DETERMINER_IN_FRONT"); }
    s_DETERMINER_IN_FRONT(call_builtin("frozenset", &[mcall(&py_str("\na an the every each no another this these those some any\nmy your his her its our their\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.POSSESSIVE_IN_FRONT"); }
    s_POSSESSIVE_IN_FRONT(call_builtin("frozenset", &[mcall(&py_str("my your his her its our their"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AFTER_A_STATE"); }
    s_AFTER_A_STATE(call_builtin("frozenset", &[mcall(&py_str("about of at with by for from since because and but so that today\n    tonight now lately again too enough when as than in on to not all these days these days"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_STATE_THEN_A_TIME"); }
    s_A_STATE_THEN_A_TIME(call_builtin("frozenset", &[mcall(&py_str("after before yesterday tomorrow already last earlier later\n    soon still early late ever during while whenever yet until till whole every"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._WERE_STATE"); }
    s__WERE_STATE(call_builtin("frozenset", &[mcall(&py_str("gutted shattered exhausted knackered tired worried stressed anxious\n    frightened scared annoyed angry furious upset sad unhappy miserable depressed lonely bored\n    happy glad pleased proud relieved grateful shattered wiped drained bushed sleepy sick ill\n    unwell poorly broke skint done fine ok okay stuck sorted lost ready low"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._WERE_ARE"); }
    s__WERE_ARE(call_builtin("frozenset", &[mcall(&py_str("gonna gunna"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._WERE_DOING"); }
    s__WERE_DOING(call_builtin("frozenset", &[mcall(&py_str("thinking having going coming driving flying travelling traveling\n    staying doing getting making taking working playing visiting eating cooking watching\n    planning trying looking waiting hoping wondering moving camping hiking jogging running\n    walking cycling swimming skiing sailing fishing gardening studying revising rehearsing\n    baking painting decorating renovating unpacking babysitting feeling jogging"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._AFTER_A_DOING"); }
    s__AFTER_A_DOING(call_builtin("frozenset", &[mcall(&py_str("a an the my your our their his her its this that these those some\n    any no every each another all both just still quite really very too so not up out off away\n    back over down around together apart through in on at to with for from about into onto by of\n    and or but because while when if i you we they he she it me us them him there here"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._WERE_PLACE"); }
    s__WERE_PLACE(call_builtin("frozenset", &[mcall(&py_str("in at near up down out over round around on inside outside"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._WERE_IN_THE_PAST"); }
    s__WERE_IN_THE_PAST(call_builtin("frozenset", &[mcall(&py_str("last yesterday then earlier previously ago once before\n    afterwards that those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._AFTER_A_LEAD_IN_SUBJECT"); }
    s__AFTER_A_LEAD_IN_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it them us him her me of"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._WERE_LEAD_IN"); }
    s__WERE_LEAD_IN(call_builtin("frozenset", &[mcall(&py_str("so not too very really all both just still quite pretty always\n    never literally definitely probably already almost nearly gonna gunna about"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_QUESTION_VERB"); }
    s__A_QUESTION_VERB(call_builtin("frozenset", &[mcall(&py_str("is are was were am do does did can could would should will\n    shall may might must has have had"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.UP_TO_SPELLINGS"); }
    s_UP_TO_SPELLINGS(call_builtin("frozenset", &[mcall(&py_str("wuu2 wu2 wyu2 wuut"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WHAT_ARE_YOU_SPELLINGS"); }
    s_WHAT_ARE_YOU_SPELLINGS(call_builtin("frozenset", &[mcall(&py_str("watcha whatcha wotcha watchu whatchu watchya whatchya"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WHAT_ARE_YOU_AFTER"); }
    s_WHAT_ARE_YOU_AFTER(call_builtin("frozenset", &[mcall(&py_str("up doing doin do been"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.LETTER_C_IS_SEE"); }
    s_LETTER_C_IS_SEE(call_builtin("frozenset", &[mcall(&py_str("u ya you yeah yah"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.UP_TO_LEADS"); }
    s_UP_TO_LEADS(call_builtin("frozenset", &[mcall(&py_str("so ok okay oh ah and but well just hi hey hiya hello right then now\n    anyway"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.UP_TO_TAGS"); }
    s_UP_TO_TAGS(call_builtin("frozenset", &[mcall(&py_str("mate hun love pet duck sunshine hen flower chuck dear pal buddy friend\n    guys folks all both then though now lol haha ha x xo xx"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.GLUED_LEFT"); }
    s_GLUED_LEFT(call_builtin("frozenset", &[mcall(&py_str("is do does did can could would will should have are was were am what whats where wheres when why how hows who whos"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.GLUED_RIGHT"); }
    s_GLUED_RIGHT(call_builtin("frozenset", &[mcall(&py_str("it that this there they you i we he she your my much long many the can should do does is are will would could did"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.GLUED_COLLAPSE"); }
    s_GLUED_COLLAPSE(py_dict(vec![(py_str("itthere"), py_list(vec![py_str("there")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.GLUED_DO"); }
    s_GLUED_DO(call_builtin("frozenset", &[mcall(&py_str("do does did"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.GLUED_AFTER_A_DO"); }
    s_GLUED_AFTER_A_DO(call_builtin("frozenset", &[mcall(&py_str("is are was were am will would can could should shall may might must wont wouldnt cant couldnt shouldnt mustnt shant mightnt"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.I_GLUED"); }
    s_I_GLUED(call_builtin("frozenset", &[mcall(&py_str("dont cant wont didnt doesnt havent havnt couldnt shouldnt wouldnt isnt\n    wasnt want feel need know think hope wish hate love miss just also really always never got\n    went have had will would should could might must am was did saw live work been told said\n    tried lost quit moved found bought sold left met heard broke failed cried slept woke rang\n    asked felt started stopped changed forgot paid spent took gave kept began ran walked drove\n    ate drank won hired retired dumped applied booked cancelled emailed phoned texted replied\n    agreed signed decided hated loved missed needed wanted hoped knew thought"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_PRONOUN_GLUED"); }
    s_A_PRONOUN_GLUED(py_tuple(vec![py_str("u"), py_str("i"), py_str("we"), py_str("you"), py_str("they"), py_str("he"), py_str("she")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_PRONOUN_STATE"); }
    s_A_PRONOUN_STATE(py_dict(vec![(py_str("im"), call_builtin("frozenset", &[mcall(&py_str("not so sorry still done going glad happy sad fine ok okay afraid worried\n        tired starving hungry thirsty freezing boiling home back here sure just really trying\n        working off out up sick lost confused angry upset lonely scared nervous bored at on in\n        sitting lying waiting\n        struggling stressed fed exhausted overwhelmed anxious depressed panicking hopeless\n        worthless gutted furious annoyed irritated shattered unwell ill hurt aching shaky dizzy\n        sleepless restless moody numb broke skint stuck busy late"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))), (py_str("ive"), call_builtin("frozenset", &[mcall(&py_str("given got been had never just tried lost done seen told said finished\n        started stopped found made taken read heard watched eaten bought moved left paid spent\n        decided noticed asked called checked waited worked lived known thought wanted needed\n        felt learned kept written broken forgotten\n        only already finally recently lately quit joined applied booked sent posted texted\n        emailed ordered become grown gained"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._PLACES_SAID_WHOLE"); }
    s__PLACES_SAID_WHOLE(py_list(vec![Py::None]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MRS_IS_THE_WIFE"); }
    s_MRS_IS_THE_WIFE(stdlib_call("re", "compile", &[py_str("\\b(the|my|our|his|her)\\s+([Mm]rs)\\b\\.?(?!\\s+[A-Z])")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data8() -> Py { py_tuple(vec![py_str("i"), py_str("we"), py_str("you"), py_str("they"), py_str("he"), py_str("she"), py_str("it"), py_str("people"), py_str("folk"), py_str("u"), py_str("ya"), py_str("yous"), py_str("everyone"), py_str("everybody"), py_str("somebody"), py_str("nobody")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._SUBJECT_SAID"); }
    s__SUBJECT_SAID(call_builtin("frozenset", &[__data8()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HELLO_SAID_SHORT"); }
    s_HELLO_SAID_SHORT(py_tuple(vec![py_tuple(vec![py_str("top of the morning"), py_str("good morning")]), py_tuple(vec![py_str("g morning"), py_str("good morning")]), py_tuple(vec![py_str("gmorning"), py_str("good morning")]), py_tuple(vec![py_str("g'morning"), py_str("good morning")]), py_tuple(vec![py_str("gm"), py_str("good morning")]), py_tuple(vec![py_str("gday"), py_str("good day")]), py_tuple(vec![py_str("g'day"), py_str("good day")]), py_tuple(vec![py_str("mornin'"), py_str("morning")]), py_tuple(vec![py_str("mornin"), py_str("morning")]), py_tuple(vec![py_str("evenin'"), py_str("evening")]), py_tuple(vec![py_str("evenin"), py_str("evening")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HELLO_TAIL"); }
    s_HELLO_TAIL(call_builtin("frozenset", &[mcall(&py_str("all everyone everybody guys folks friends there you mate to"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RUN_ON_OPENERS"); }
    s_RUN_ON_OPENERS(call_builtin("frozenset", &[mcall(&py_str("what how why when where who which whats hows wheres whos nothing nobody everything everyone something someone can cant could couldnt should shouldnt would wouldnt will wont do dont does doesnt did didnt is are was were shall"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RUN_ON_SELF"); }
    s_RUN_ON_SELF(call_builtin("frozenset", &[mcall(&py_str("i im ive id ill idk idunno cant dont didnt doesnt wont isnt arent"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RUN_ON_MINE"); }
    s_RUN_ON_MINE(call_builtin("frozenset", &[mcall(&py_str("my your his her our their"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RUN_ON_WORDS"); }
    s_RUN_ON_WORDS(op_bitor(&g_LEXICON(), &call_builtin("frozenset", &[mcall(&py_str("a i im ive id ill idk idunno dunno cant dont didnt\n    doesnt wont isnt arent wasnt werent wouldnt couldnt shouldnt"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RUN_ON_PEOPLE"); }
    s_RUN_ON_PEOPLE(call_builtin("frozenset", &[mcall(&py_str("i im ive id ill me my we us our you your he him his she her it its\n    they them their there this that these those nothing nobody everything everyone something\n    someone anyone no one"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RUN_ON_SELF_LONGEST"); }
    s_RUN_ON_SELF_LONGEST(Py::Int(9));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RUN_ON_SHORTEST"); }
    s_RUN_ON_SHORTEST(Py::Int(8));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.RUN_ON_LAST_PIECES"); }
    s_RUN_ON_LAST_PIECES(call_builtin("frozenset", &[mcall(&py_str("ever ether soever"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.APOSTROPHE_TAILS"); }
    s_APOSTROPHE_TAILS(call_builtin("frozenset", &[mcall(&py_str("m re ve ll d s"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.APOSTROPHE_BASES"); }
    s_APOSTROPHE_BASES(call_builtin("frozenset", &[mcall(&py_str("i you we they he she it that there here what who where when\n    why how let everyone everybody someone somebody nobody"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.APOSTROPHE_NOT"); }
    s_APOSTROPHE_NOT(call_builtin("frozenset", &[py_list(vec![py_tuple(vec![py_str("i"), py_str("s")])])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SAID_APART_FORMS"); }
    s_SAID_APART_FORMS(_the_said_apart_forms(&[]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SAID_APART"); }
    s_SAID_APART(stdlib_call("re", "compile", &[op_modulo(&py_str("(?<![a-z])(?:%s)(?![a-z])"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SAID_APART_FORMS()); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| {  let pieces: Py = __kwp(args, 0, "pieces").unwrap_or(Py::None); op_neg(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut p: Py = __item.clone(); __out.push(call_builtin("len", &[p.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut pieces: Py = __item.clone(); __out.push(mcall(&py_str("\\s+"), "join", &[pieces.clone()])); } py_list(__out) })() }]))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_LETTER_AFTER"); }
    s_A_LETTER_AFTER(call_builtin("frozenset", &[mcall(&py_str("my your his her their our the a an this that these those no any\n    every each another some"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SOLFEGE"); }
    s_SOLFEGE(call_builtin("frozenset", &[mcall(&py_str("do re mi fa sol la ti so"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MI_IS_ME_AFTER"); }
    s_MI_IS_ME_AFTER(call_builtin("frozenset", &[mcall(&py_str("and n or nor & too either neither also"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MI_IS_ME_BEFORE"); }
    s_MI_IS_ME_BEFORE(call_builtin("frozenset", &[mcall(&py_str("the a an this that these those later soon now again what how why\n    when where if to in on at up out over please pls plz today tomorrow tonight then also\n    though anyway know alone"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MI_IS_A_NAME_AFTER"); }
    s_MI_IS_A_NAME_AFTER(call_builtin("frozenset", &[mcall(&py_str("the a an this that these those another"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MI_BACK_VERBS"); }
    s_MI_BACK_VERBS(call_builtin("frozenset", &[mcall(&py_str("text ring call phone send pass get give pay email message pm\n    whatsapp reply write leave take bring hand poke shout"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ILL_GOES_ON"); }
    s_ILL_GOES_ON(call_builtin("frozenset", &[mcall(&py_str("go take catch"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ONE_OF_A_KIND"); }
    s_ONE_OF_A_KIND(call_builtin("frozenset", &[mcall(&py_str("back head neck chest stomach tummy belly throat nose waist spine\n    forehead tongue heart pelvis gullet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ONE_PART_AFTER"); }
    s_ONE_PART_AFTER(call_builtin("frozenset", &[mcall(&py_str("is was has been s keeps kept does did will would going playing\n    killing hurting aching throbbing banging pounding killing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.VEGGIES_AFTER"); }
    s_VEGGIES_AFTER(call_builtin("frozenset", &[mcall(&py_str("roast roasted roasting grilled grilling steamed steaming fried frying\n    stir braised baked charred sauteed saute boiled raw leftover leftovers fresh frozen chopped\n    diced sliced seasonal mixed cook cooked cooking eat eating ate"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WEARS_AFTER"); }
    s_WEARS_AFTER(call_builtin("frozenset", &[mcall(&py_str("i you u we they he she it people everyone somebody anybody\nmum mums dad dads mam mother father sister brother son daughter wife husband nan gran\ngrandma grandad granparents mates kids children boy girl lad lass"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUM_THE_NOUN"); }
    s_SUM_THE_NOUN(call_builtin("frozenset", &[mcall(&py_str("a an the my your his her our their its this that these those another\n    each every no any do did does doing done work worked works add added adds total totals\n    lump round large small tidy princely big hefty cash whole same missing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUM_STAYS_AFTER"); }
    s_SUM_STAYS_AFTER(call_builtin("frozenset", &[mcall(&py_str("up it things that this them total"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.YOU_ARE_AFTER"); }
    s_YOU_ARE_AFTER(call_builtin("frozenset", &[mcall(&py_str("a an the such so too very really quite bloody proper\n    going gonna joking kidding being not"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._SPACED_RUN"); }
    s__SPACED_RUN(stdlib_call("re", "compile", &[py_str("(?<![\\w'])([a-z0-9](?:[ \\t]+[a-z0-9]){2,3})(?![\\w'])"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._SPACED_PIECE"); }
    s__SPACED_PIECE(stdlib_call("re", "compile", &[py_str("^[a-z0-9]$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DOCS_VISIT"); }
    s_DOCS_VISIT(call_builtin("frozenset", &[mcall(&py_str("go goes going gone went see sees seeing saw seen visit visits visiting\n    ring rings ringing rang call calls calling called phone phones phoned ask asks asking asked\n    book booked appointment appointments consult surgery gp"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DOCS_PAPER"); }
    s_DOCS_PAPER(call_builtin("frozenset", &[mcall(&py_str("send sends sent email emails emailed print printed download downloaded\n    upload uploaded scan scanned file files folder contract contracts paperwork invoice invoices\n    lease leases tax read reading sign signed attach attached share forward forwarded page pages\n    pdf copy copies draft document documents solicitor mortgage insurance check checked\n    through"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DOCS_SCREEN"); }
    s_DOCS_SCREEN(call_builtin("frozenset", &[mcall(&py_str("tablet laptop pc screen online app apps link links browser website web\n    google drive cloud portal intranet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DOCS_ON"); }
    s_DOCS_ON(call_builtin("frozenset", &[py_tuple(vec![py_str("on"), py_str("in"), py_str("via"), py_str("from"), py_str("as"), py_str("into")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.MEAL_WORDS"); }
    s_MEAL_WORDS(call_builtin("frozenset", &[mcall(&py_str("tea dinner supper lunch breakfast brunch snack food"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FOOD_SLOT_LEAD"); }
    s_FOOD_SLOT_LEAD(call_builtin("frozenset", &[mcall(&py_str("for at after before til till until round over about"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_A_MEAL_TYPO"); }
    s_NOT_A_MEAL_TYPO(call_builtin("frozenset", &[py_tuple(vec![py_str("tee")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TIME_AFTER_MEAL"); }
    s_TIME_AFTER_MEAL(call_builtin("frozenset", &[mcall(&py_str("time tonight tomorrow today now later"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ONE_RELATION"); }
    s_ONE_RELATION(call_builtin("frozenset", &[mcall(&py_str("mum mom mam mummy mommy dad daddy nan nana gran granny grandad\n    grandpa granddad grandma grandmother grandfather mother father husband wife"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HAS_FORMS"); }
    s_HAS_FORMS(call_builtin("frozenset", &[py_tuple(vec![py_str("got"), py_str("bin"), py_str("had")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.ONE_RELATION_VERBS"); }
    s_ONE_RELATION_VERBS(call_builtin("frozenset", &[mcall(&py_str("got bin wont cant said says told went came goes needs wants\n    keeps lives works gets thinks loves hates"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SORE_IN_FRONT"); }
    s_SORE_IN_FRONT(call_builtin("frozenset", &[mcall(&py_str("is are was were been feels feel felt gets get got still"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SORE_FILLERS"); }
    s_SORE_FILLERS(call_builtin("frozenset", &[mcall(&py_str("so very really bit little too pretty quiet that"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SORE_PARTS"); }
    s_SORE_PARTS(call_builtin("frozenset", &[mcall(&py_str("throat back head neck shoulder shoulders knee leg legs foot feet arm\n    arms toe toes hip ankle wrist elbow ear ears eye eyes tooth teeth gum mouth hand hands\n    finger fingers belly stomach muscle muscles chest side sides rib ribs heel heels jaw skin\n    tongue sunburn"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WH_QUANTIFIERS"); }
    s_WH_QUANTIFIERS(call_builtin("frozenset", &[mcall(&py_str("else other others kind sort type size colour color number\n    make model brand version flavour flavor"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_IN_A_QUESTION_PHRASE"); }
    s_NOT_IN_A_QUESTION_PHRASE(op_bitor(&call_builtin("frozenset", &[mcall(&py_str("\n    i me my mine we us our you your he him his she her it its they them their\n    the a an this that these those some each every which what who whom whose\n    to of for in on at by with from about as than if whether there here\n    do does did is are was were am be been being have has had will would shall should can\n    could may might must not no never"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_WH_QUANTIFIERS()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.AFTER_AN_INVERTED_AUX"); }
    s_AFTER_AN_INVERTED_AUX(op_bitor(&call_builtin("frozenset", &[mcall(&py_str("\n    i we you he she they there it this that these those\n    the a an my our your his her their some any"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SUBJECT_WORDS()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CLAUSE_OPENERS"); }
    s_CLAUSE_OPENERS(call_builtin("frozenset", &[mcall(&py_str("i we you they he she it this that there anyone anybody someone\n    everyone somebody nobody nothing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HAS_AFTER"); }
    s_HAS_AFTER(call_builtin("frozenset", &[mcall(&py_str("got been had"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.HAS_SUBJECTS"); }
    s_HAS_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("she he who"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.LEADERS"); }
    s_LEADERS(call_builtin("frozenset", &[mcall(&py_str("and so but also then ok okay oh well right now anyway anyways\nhi hiya hey hello morning evening thanks thank thx cheers ty please just actually still\nfinally instead otherwise plus however meanwhile btw honestly anyway"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.LIKE_STARTS_A_CLAUSE"); }
    s_LIKE_STARTS_A_CLAUSE(call_builtin("frozenset", &[mcall(&py_str("\ni im ive id ill me my mine we were weve us our ours you youre youve your he hes she shes it its\nthey theyre theyve their them\nwhat why how when where who which whose\ncan cant cannot could couldnt should shouldnt would wouldnt do dont doesnt did didnt\nis are was were will wont have has had havent hasnt dont didnt get got gets need needs want\nwants keep keeps just never not so too always still really gonna wanna dunno idk\nu ur ya yall\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.EXCLAIMED_AFTER_A_QUESTION"); }
    s_EXCLAIMED_AFTER_A_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(what|how|why|where|who|which|when|whatever|however|whoever|whenever)\\s+(?:on\\s+earth|in\\s+the\\s+world|the\\s+dickens|the\\s+blazes|on\\s+god'?s\\s+earth)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.EXCLAMATION_OPENS"); }
    s_EXCLAMATION_OPENS(call_builtin("frozenset", &[mcall(&py_str("god honestly ugh jesus christ blimey crikey cor gosh omg\n    jeez sheesh oh ah"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.EXCLAMATION_PHRASE"); }
    s_EXCLAMATION_PHRASE(stdlib_call("re", "compile", &[py_str("^(?:oh\\s+my\\s+god|my\\s+god|good\\s+grief|for\\s+goodness'?\\s*sake|for\\s+god'?s?\\s*sake)\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._EXCLAMATION_SUBJECTS"); }
    s__EXCLAMATION_SUBJECTS(call_builtin("frozenset", &[mcall(&py_str("i im ive id ill we weve youre you he hes she shes they\n    theyre it its my me your our"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._LEADER_WORD"); }
    s__LEADER_WORD(stdlib_call("re", "compile", &[py_str("^\\s*([A-Za-z']+)\\s*[,.;:!\\-\\u2014]*\\s*")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._PLACE_STOP"); }
    s__PLACE_STOP(call_builtin("frozenset", &[mcall(&py_str("last next this that yesterday today tomorrow tonight morning\n    afternoon evening night week weekend month year hour minute monday tuesday wednesday\n    thursday friday saturday sunday"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._PLACE_LEADS"); }
    s__PLACE_LEADS(call_builtin("frozenset", &[mcall(&py_str("was were is am are been being s started starts began begins\n    went go goes coming came got get stays stayed lived living working"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._WH_QUESTIONS"); }
    s__WH_QUESTIONS(call_builtin("frozenset", &[mcall(&py_str("what why how when where who whose which"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.REASON_STARTS"); }
    s_REASON_STARTS(call_builtin("frozenset", &[mcall(&py_str("because cause cos cus coz since"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data9() -> Py { py_dict(vec![(py_str("æ"), py_str("ae")), (py_str("Æ"), py_str("AE")), (py_str("œ"), py_str("oe")), (py_str("Œ"), py_str("OE")), (py_str("ø"), py_str("o")), (py_str("Ø"), py_str("O")), (py_str("ł"), py_str("l")), (py_str("Ł"), py_str("L")), (py_str("đ"), py_str("d")), (py_str("Đ"), py_str("D")), (py_str("ð"), py_str("d")), (py_str("Ð"), py_str("D")), (py_str("þ"), py_str("th")), (py_str("Þ"), py_str("TH")), (py_str("ß"), py_str("ss")), (py_str("ı"), py_str("i"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.LETTERS"); }
    s_LETTERS(mcall(&py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e))), "maketrans", &[__data9()]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._INTENSIFIER"); }
    s__INTENSIFIER(py_str("on\\s+earth|in\\s+the\\s+world|the\\s+(?:hell|heck|devil|blazes|dickens|fuck)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WH_INTENSIFIER"); }
    s_WH_INTENSIFIER(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\s*(?:"), &g__INTENSIFIER()), &py_str(")\\s*$"))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WH_INTENSIFIER_LINE"); }
    s_WH_INTENSIFIER_LINE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![a-z'])\\b(what|why|when|where|who|which|whose|how)\\s+(?:"), &g__INTENSIFIER()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.WH_ANY"); }
    s_WH_ANY(call_builtin("frozenset", &[mcall(&py_str("what why when where who which whose how"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._TYPED_SURPRISE"); }
    s__TYPED_SURPRISE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![a-z'])\\b(?:what|why|when|where|who|which|whose|how|whatever|however|whoever|whenever)\\s+(?:"), &g__INTENSIFIER()), &py_str(")")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_CLIP"); }
    s__A_CLIP(py_dict(vec![(py_str("mon"), py_tuple(vec![py_str("moon"), call_builtin("frozenset", &[mcall(&py_str("moon moons full half crescent phase waxing waning eclipse\n        lunar sky night tonight bright light lit shines shine rises rising stars"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TAG_WORDS"); }
    s_TAG_WORDS(call_builtin("frozenset", &[mcall(&py_str("x xx xxx xo xxo xoxo mwah kisses"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TAG_PLEASE"); }
    s_TAG_PLEASE(call_builtin("frozenset", &[mcall(&py_str("please pls plz pls."), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TAG_THANKS"); }
    s_TAG_THANKS(call_builtin("frozenset", &[mcall(&py_str("thanks thank cheers thx ty ta"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TAG_QUESTION_LEADS"); }
    s_TAG_QUESTION_LEADS(call_builtin("frozenset", &[mcall(&py_str("what why how when where who which whose can could should\n    would will is are do does did have has am was were may might shall"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TAG_GUARD_BEFORE"); }
    s_TAG_GUARD_BEFORE(call_builtin("frozenset", &[mcall(&py_str("a an the this that my your his her their our for of is are was\n    were be been being with than to from called named call name letter mark spot sign cross tick\n    size sized equals equal and or but not no"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CHANGE_OF_SUBJECT"); }
    s_CHANGE_OF_SUBJECT(stdlib_call("re", "compile", &[py_str("^(?:(?:and|so|but|oh|ok|okay|right|now|anyway|anyways|also|plus|well)\\s*,?\\s*){0,3}(?:(?:changing|change\\s+of|changing\\s+the)\\s+(?:the\\s+)?(?:subject|topic|tack)|(?:moving\\s+on\\s+to|on\\s+to|onto|on|to)\\s+(?:a\\s+)?(?:new|different|another)\\s+(?:subject|topic|tack)|(?:moving\\s+on\\s+to|on\\s+to|onto)\\s+something\\s+else|moving\\s+on|(?:a\\s+)?(?:new|different|another)\\s+(?:subject|topic|tack))\\s*[,;:.\\u2013\\u2014-]+\\s*(?=\\S)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SPELLING_ASK"); }
    s_SPELLING_ASK(call_builtin("frozenset", &[mcall(&py_str("spell spells spelling spelt spelled misspelt misspelled pronounce"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SPELLING_ENDINGS"); }
    s_SPELLING_ENDINGS(py_tuple(vec![py_str("ly"), py_str("ness"), py_str("ing"), py_str("ed"), py_str("es"), py_str("s"), py_str("d"), py_str("e")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NAMED_LETTER"); }
    s_NAMED_LETTER(stdlib_call("re", "compile", &[py_str("\\b(?:with\\s+(?:an?|one|two|three|double)\\s+[a-z](?![a-z])|(?:one|two|three|a\\s+double)\\s+[a-z](?![a-z])\\s+or\\s+(?:one|two|three|an?)\\b|double\\s+[a-z](?![a-z])\\b|[a-z](?<![a-z])\\s+or\\s+(?:two|three)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data10() -> Py { py_tuple(vec![py_str("letter"), py_str("letters"), py_str("word"), py_str("words"), py_str("vowel"), py_str("consonant"), py_str("initial"), py_str("alphabet"), py_str("grade"), py_str("grade"), py_str("type"), py_str("blood"), py_str("vitamin"), py_str("plan"), py_str("shape"), py_str("u"), py_str("the")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_SHORT_WORD_IS_NAMED_BEFORE"); }
    s__A_SHORT_WORD_IS_NAMED_BEFORE(call_builtin("frozenset", &[__data10()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._A_SHORT_WORD_IS_NAMED_AFTER"); }
    s__A_SHORT_WORD_IS_NAMED_AFTER(call_builtin("frozenset", &[py_tuple(vec![py_str("turn"), py_str("turns"), py_str("turn-"), py_str("boat"), py_str("boats"), py_str("bend"), py_str("shaped"), py_str("shape"), py_str("tube"), py_str("value"), py_str("values")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.DAY_NAMED"); }
    s_DAY_NAMED(stdlib_call("re", "compile", &[py_str("\\b(birthday|anniversary|wedding)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.OWNER_WITHOUT_THE_DAY"); }
    s_OWNER_WITHOUT_THE_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:it|its|it's|that|thats|that's)(?:\\s+is)?\\s+(?:not\\s+)?(?:my|our)\\s+([a-z]+'?s)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.TAIL_OF_A_DAY"); }
    s_TAIL_OF_A_DAY(call_builtin("frozenset", &[mcall(&py_str("\n    tomorrow today tonight soon next this on in at already too as well then\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.CAN_NOT"); }
    s_CAN_NOT(stdlib_call("re", "compile", &[py_str("\\bcan\\s+not\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.A_NUMBER_OF_SOMEONES_DAY"); }
    s_A_NUMBER_OF_SOMEONES_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:my|our)\\s+([a-z]+\\'?s?)\\s+(\\d+(?:st|nd|rd|th))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.NOT_MY_OWN_DAY"); }
    s_NOT_MY_OWN_DAY(stdlib_call("re", "compile", &[py_str("\\s*(?:,?\\s*\\b(?:and\\s+)?not\\s+(?:my|mine)\\s+(birthday|bday|anniversary|wedding)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data11() -> Py { py_tuple(vec![py_str("fridge"), py_str("fridgefreezer"), py_str("refrigerator"), py_str("freezer"), py_str("cupboard"), py_str("larder"), py_str("pantry"), py_str("worktop"), py_str("counter"), py_str("countertop"), py_str("out"), py_str("outside"), py_str("coolbox"), py_str("coolbag"), py_str("bag")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._KEEPING_PLACES"); }
    s__KEEPING_PLACES(call_builtin("frozenset", &[__data11()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._KEPT_THINGS"); }
    s__KEPT_THINGS(call_builtin("frozenset", &[py_tuple(vec![py_str("leftover"), py_str("leftovers"), py_str("cooked"), py_str("uneaten"), py_str("spare"), py_str("rest"), py_str("remains"), py_str("opened")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._DAY_WORDS"); }
    s__DAY_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("day"), py_str("days"), py_str("night"), py_str("nights"), py_str("week"), py_str("weeks"), py_str("month"), py_str("months")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._KEPT_ALREADY"); }
    s__KEPT_ALREADY(call_builtin("frozenset", &[py_tuple(vec![py_str("overnight"), py_str("yesterday"), py_str("since")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data12() -> Py { py_tuple(vec![py_str("a"), py_str("an"), py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("couple"), py_str("few"), py_str("several"), py_str("many"), py_str("half")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._SMALL_NUMBERS"); }
    s__SMALL_NUMBERS(call_builtin("frozenset", &[__data12()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._NUMBER_SHAPE"); }
    s__NUMBER_SHAPE(stdlib_call("re", "compile", &[py_str("^\\d{1,3}(?:st|nd|rd|th)?$")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SICK_AS_FEELING"); }
    s_SICK_AS_FEELING(stdlib_call("re", "compile", &[py_str("\\b(?:sick|ill|unwell|nauseous|nauseated|queasy|shaking|shaky|trembling|dizzy)\\s+(?:(?:with|from)\\s+)?(worry|worrying|worried|stress|stress(?:ed)?\\s+out|fear|fright|frightened|panic\\w*|nerves|nervous|anxious|anxiety|guilt\\w*|grief|shame|ashamed|dread\\w*|anger|rage|jealousy|it\\s+all)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data13() -> Py { py_dict(vec![(py_str("worry"), py_str("worried")), (py_str("worrying"), py_str("worried")), (py_str("worried"), py_str("worried")), (py_str("stress"), py_str("stressed")), (py_str("stressed out"), py_str("stressed")), (py_str("stressed"), py_str("stressed")), (py_str("fear"), py_str("frightened")), (py_str("fright"), py_str("frightened")), (py_str("frightened"), py_str("frightened")), (py_str("panic"), py_str("panicky")), (py_str("panicking"), py_str("panicky")), (py_str("panicky"), py_str("panicky")), (py_str("nerve"), py_str("nervous")), (py_str("nerves"), py_str("nervous")), (py_str("nervous"), py_str("nervous")), (py_str("anxious"), py_str("anxious")), (py_str("anxiety"), py_str("anxious")), (py_str("guilt"), py_str("guilty")), (py_str("guilty"), py_str("guilty")), (py_str("grief"), py_str("grieving")), (py_str("shame"), py_str("ashamed")), (py_str("ashamed"), py_str("ashamed")), (py_str("dread"), py_str("dreading")), (py_str("dreading"), py_str("dreading")), (py_str("anger"), py_str("angry")), (py_str("rage"), py_str("angry")), (py_str("jealousy"), py_str("jealous")), (py_str("it all"), py_str("overwhelmed"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize._FEELING_WORD"); }
    s__FEELING_WORD(__data13());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.FEELING_BEFORE_SICK"); }
    s_FEELING_BEFORE_SICK(stdlib_call("re", "compile", &[py_str("\\b(?:worried|scared|frightened|terrified|bored|stressed|anxious)\\s+sick\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUGAR_LEVEL"); }
    s_SUGAR_LEVEL(call_builtin("frozenset", &[mcall(&py_str("low high dropped drop drops dropping crashed crash crashes crashing\n    dipped dip dips dipped levels level reading readings rises rose rising spike spiked spiking\n    fell falls falling down up"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUGAR_AFTER_ROSE"); }
    s_SUGAR_AFTER_ROSE(call_builtin("frozenset", &[mcall(&py_str("to from after again overnight during before since sharply\n    steadily quickly suddenly a bit so very high low all above around past 6 7 8 9"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUGAR_PRICE"); }
    s_SUGAR_PRICE(call_builtin("frozenset", &[mcall(&py_str("price prices cost costs pound pounds lb lbs kilo kilos bag bags packet\n    packets tin tins gram grams spoon spoons teaspoon teaspoons"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUGAR_PRICE_SHAPE"); }
    s_SUGAR_PRICE_SHAPE(stdlib_call("re", "compile", &[py_str("^(?:[£$€]|\\d+(?:\\.\\d+)?p(?:ence)?$)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.SUGAR_FIRST_PERSON"); }
    s_SUGAR_FIRST_PERSON(stdlib_call("re", "compile", &[py_str("\\b(my|his|her|their|your)\\s+sugar\\b((?:\\s+[a-z0-9'£$€]+){0,5})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BAD_SAYING"); }
    s_BAD_SAYING(py_str("(?:an?\\s+|such\\s+an?\\s+|one\\s+big\\s+|an?\\s+(?:complete|right|total|proper|real|absolute|utter|bit\\s+of\\s+a|bloody)\\s+)?(?:joke|nightmare|mess|disaster|wreck|state|shambles|agony|torture|car\\s+crash|pain\\s+in\\s+the\\s+(?:arse|bum|butt|neck|back))"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.BAD_WORD"); }
    s_BAD_WORD(py_str("(?:terrible|awful|rubbish|crap|horrible|dreadful|brutal|shocking|knackered|shot|agony|unbearable|wrecked|ruined|buggered|done\\s+in)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.PART_THAT_IS_BAD"); }
    s_PART_THAT_IS_BAD(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(my|his|her|their|our|your)\\s+("), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("understanding.lexicon", "BODY_PARTS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\s+(?P<link>is|are|was|were|has\\s+been|have\\s+been|feels?|feels?\\s+like)\\s+(?:")), &g_BAD_SAYING()), &py_str("|")), &g_BAD_WORD()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET understanding.normalize.THE_ODD_PLURALS"); }
    s_THE_ODD_PLURALS(call_builtin("frozenset", &[mcall(&py_str("feet teeth calves"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn respell_urgent(args: &[Py]) -> Py {
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
    let mut fixes: Py = Py::None;
    let mut put: Py = Py::None;
    let _ = py_str("The line as typed, with only a misspelt emergency word put right and nothing else moved.\n\n    The reading has `norm_text`, but that splits clauses and drops the punctuation, and the\n    safety module matches apostrophes (\"can't breathe\") and one line, so it cannot read it.\n    This keeps the line as it was typed, word for word, and only puts back a word from the\n    curated emergency table: \"my son swalowed a coin\" -> \"my son swallowed a coin\". Nothing\n    else in the line is touched, so a module that reads it sees exactly what it read before\n    for every line that is not an emergency with a misspelt word in it.\n\n    The one other thing put back is a family word clipped to its initials, because that is a\n    word the reading knows and the module cannot: \"my dd fell off the slide\" was answered with\n    the falls answer for an older person, and \"my daughter fell off the slide\" gets the two\n    questions a child's fall needs. The clipped word is only read after a possessive, and only\n    when the word after it does not make it a thing of that name (\"my ds wont charge\" is still\n    the console), which is the same guard the reading puts on it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&text.clone())) } }) {
    return py_str("");
    }
    fixes = pyrt::xrefs::xget("understanding.lexicon", "URGENT_MISSPELLINGS");
pub fn __nested_put_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut fixes: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut put: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut fix: Py = Py::None;
    let mut guessed: Py = Py::None;
    let mut keep: Py = Py::None;
    let mut word: Py = Py::None;
    word = mcall(&match_.clone(), "group", &[Py::Int(0)]);
    fix = mcall(&fixes.clone(), "get", &[mcall(&word.clone(), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&fix.clone()))) {
    guessed = fix_typo(&[mcall(&word.clone(), "lower", &[])]);
    return if py_truthy(&Py::Bool(py_contains(&g_URGENT_WORDS(), &guessed.clone()))) { guessed.clone() } else { word.clone() };
    }
    keep = mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]);
    return if py_truthy(&keep.clone()) { mcall(&fix.clone(), "capitalize", &[]) } else { fix.clone() };
    Py::None
}
    put = {{ let fixes = fixes.clone(); let put = put.clone(); let text = text.clone(); py_func("put", move |args: &[Py]| { let mut full = vec![fixes.clone(), put.clone(), text.clone()]; full.extend_from_slice(args); __nested_put_0(&full) }) }};
    return mcall(&g_CLIPPED_FAMILY(), "sub", &[py_func("_family_put_back", |args: &[Py]| _family_put_back(args)), mcall(&g__ANY_WORD(), "sub", &[put.clone(), text.clone()])]);
    Py::None
}
pub fn _family_put_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut after: Py = Py::None;
    let mut after_after: Py = Py::None;
    let mut family: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The clipped family word read as the word it stands for, or the match left as it was.");
    rest = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("rest")]); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[]);
    after = if py_truthy(&rest.clone()) { mcall(&geti(&rest.clone(), &Py::Int(0)), "lower", &[]) } else { Py::None };
    after_after = if py_truthy(&op_gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { mcall(&geti(&rest.clone(), &Py::Int(1)), "lower", &[]) } else { Py::None };
    family = _family_short(&[mcall(&mcall(&match_.clone(), "group", &[py_str("word")]), "lower", &[]), mcall(&mcall(&match_.clone(), "group", &[py_str("owner")]), "lower", &[]), after.clone(), after_after.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&family.clone()))) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    return op_add(&op_add(&op_add(&mcall(&match_.clone(), "group", &[py_str("owner")]), &py_str(" ")), &mcall(&py_str(" "), "join", &[family.clone()])), &{ let __b = mcall(&match_.clone(), "group", &[py_str("rest")]); if py_truthy(&__b) { __b } else { py_str("") } });
    Py::None
}
pub fn _doubles_its_last_letter(args: &[Py]) -> Py {
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
    let _ = py_str("\"get\" gives \"getting\" and \"throb\" gives \"throbbing\": a one-beat word that ends in a\n    consonant after a single vowel writes that consonant twice before -ing and -ed.\n\n    Only the last syllable matters to English, and this has no way to hear the stress, so it\n    asks for a word of one beat - \"visit\" and \"open\" are left alone, and so are words that end\n    in a letter English never doubles (\"fix\", \"play\", \"snow\").\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("aeiou"), &geti(&word.clone(), &op_neg(&Py::Int(3))))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&py_str("aeiou"), &geti(&word.clone(), &op_neg(&Py::Int(2))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str("aeiouwxy"), &geti(&word.clone(), &op_neg(&Py::Int(1))))) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("[aeiou]+"), word.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)));
    Py::None
}
pub fn _drops_a_final_e(args: &[Py]) -> Py {
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
    let _ = py_str("True for a word that loses its last e before an ending: \"move\" is \"moving\", \"moved\".\n\n    Every word that ends in a silent e drops it before -ed and -er, so \"agree\" is \"agreed\",\n    \"free\" is \"freed\", \"eye\" is \"eyed\" and \"dye\" is \"dyed\"; a word ending in \"ee\" keeps both\n    letters only before -ing. That is the one ending the letter before the e is the whole\n    question for (\"seeing\", \"agreeing\", \"freeing\"), and it is named in _keeps_its_e_before_ing:\n    there is no rule in English that hears it.\n    ");
    return { let __b = mcall(&word.clone(), "endswith", &[py_str("e")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } };
    Py::None
}
pub fn _keeps_its_e_before_ing(args: &[Py]) -> Py {
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
    let _ = py_str("True for a word whose \"ee\", \"ye\" or \"oe\" keeps both letters before -ing.");
    return { let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("ee"), py_str("ye"), py_str("oe")])]) } };
    Py::None
}
pub fn _ing_form(args: &[Py]) -> Py {
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
    let _ = py_str("The -ing form of a word that ends in an e: \"moving\", \"agreeing\", \"eyeing\".");
    return if py_truthy(&_keeps_its_e_before_ing(&[word.clone()])) { op_add(&word.clone(), &py_str("ing")) } else { op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing")) };
    Py::None
}
pub fn _inflections(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut y_for_i: Py = Py::None;
    let _ = py_str("The everyday forms of a word, so 'likes' and 'jeans' are not 'typos'.");
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))) } }) {
    return out.clone();
    }
    y_for_i = { let __b = mcall(&word.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_VOWEL_SET(), &geti(&word.clone(), &op_neg(&Py::Int(2))))) } };
    let _ = mcall(&out.clone(), "add", &[op_add(&word.clone(), &py_str("ly"))]);
    if py_truthy(&Py::Bool(!py_truthy(&y_for_i.clone()))) {
    let _ = mcall(&out.clone(), "add", &[op_add(&word.clone(), &py_str("s"))]);
    }
    if py_truthy(&_doubles_its_last_letter(&[word.clone()])) {
    let _ = mcall(&out.clone(), "update", &[py_tuple(vec![op_add(&op_add(&word.clone(), &geti(&word.clone(), &op_neg(&Py::Int(1)))), &py_str("ing")), op_add(&op_add(&word.clone(), &geti(&word.clone(), &op_neg(&Py::Int(1)))), &py_str("ed")), op_add(&op_add(&word.clone(), &geti(&word.clone(), &op_neg(&Py::Int(1)))), &py_str("er"))])]);
    } else {
    if py_truthy(&_drops_a_final_e(&[word.clone()])) {
    let _ = mcall(&out.clone(), "update", &[py_tuple(vec![_ing_form(&[word.clone()]), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ed")), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("er"))])]);
    } else {
    let _ = mcall(&out.clone(), "add", &[op_add(&word.clone(), &py_str("ing"))]);
    if py_truthy(&Py::Bool(!py_truthy(&y_for_i.clone()))) {
    let _ = mcall(&out.clone(), "update", &[py_tuple(vec![op_add(&word.clone(), &py_str("ed")), op_add(&word.clone(), &py_str("er"))])]);
    }
    }
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("e")])) {
    let _ = mcall(&out.clone(), "update", &[py_tuple(vec![op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing")), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ed"))])]);
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("y")])) {
    let _ = mcall(&out.clone(), "update", &[py_tuple(vec![op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies")), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ied")), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ier"))])]);
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("s")])) {
    let _ = mcall(&out.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    return out.clone();
    Py::None
}
pub fn learn_thing_words(args: &[Py]) -> Py {
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
    let _ = py_str("Keep the vocabulary the topics answer for, so a thing typed a letter short is read.");
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    let _ = mcall(&g_THING_WORDS(), "add", &[word.clone()]);
    let _ = mcall(&mcall(&g_THING_WORDS_BY_LENGTH(), "setdefault", &[call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), "add", &[word.clone()]);
    }
    }
    Py::None
}
pub fn _double_variants(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("Words one doubled-letter change away (\"recomend\" -> \"recommend\", \"untill\" -> \"until\").");
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    char = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&op_add(&g_VOWELS(), &py_str("y")), &char.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&op_add(&i.clone(), &Py::Int(1)), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &op_add(&i.clone(), &Py::Int(1))), &char.clone())) } }) {
    let _ = mcall(&out.clone(), "add", &[op_add(&slice_or_raise(&word.clone(), &Py::None, &i.clone(), &Py::None), &slice_or_raise(&word.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None))]);
    } else {
    let _ = mcall(&out.clone(), "add", &[op_add(&op_add(&slice_or_raise(&word.clone(), &Py::None, &i.clone(), &Py::None), &char.clone()), &slice_or_raise(&word.clone(), &i.clone(), &Py::None, &Py::None))]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _a_doubled_letter_typed_once(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("Words made by writing one of this word's letters twice: \"wek\" -> \"week\".\n\n    The vowel is left out of `_double_variants` because most of the time it is the consonant\n    that doubles (\"recomend\"), and a doubled vowel is rarer still - but \"wek\", \"kep\" and \"cok\"\n    are the same slip, and a vowel is what the writer left one of.\n    ");
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    char = __unpk2[1].clone();
    if py_truthy(&mcall(&char.clone(), "isalpha", &[])) {
    let _ = mcall(&out.clone(), "add", &[op_add(&op_add(&slice_or_raise(&word.clone(), &Py::None, &i.clone(), &Py::None), &char.clone()), &slice_or_raise(&word.clone(), &i.clone(), &Py::None, &Py::None))]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _three_of_a_letter(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let _ = py_str("Whether one letter runs three times together, which no English word does.\n\n    The lexicon works a word's endings out, so a word that already ends in the letter an\n    ending opens with comes back with three of them: \"tell\" + \"ly\" is \"tellly\", \"full\" + \"ly\"\n    is \"fullly\". Nobody types those, so three together is the mark of a word the endings made\n    rather than a word, and a spelling is never bent towards one.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e))) { let mut at: Py = __item.clone(); __out.push({ let __cmp11: Py = geti(&word.clone(), &op_add(&at.clone(), &Py::Int(1))); if py_truthy(&Py::Bool(py_eq(&geti(&word.clone(), &at.clone()), &__cmp11))) { Py::Bool(py_eq(&__cmp11, &geti(&word.clone(), &op_add(&at.clone(), &Py::Int(2))))) } else { Py::Bool(false) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _doubled_letter_words(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let _ = py_str("The words a doubled letter would make of this one, and that the lexicon knows.\n\n    Only for a word the lexicon does not have: an ending of a word is a word of its own\n    (\"planed\" is not \"planned\"), and the lexicon's \"mattres\" is a plural rule run backwards\n    off \"mattress\", so a candidate that is a word only because a word ending in -ss lost an s\n    is not a word at all. That guard is what stops \"matres\" being read as \"mattres\".\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &word.clone()))) {
    return py_list(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_a_doubled_letter_typed_once(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&g_BASE(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_BASE(), &op_add(&w.clone(), &py_str("s")))) } } } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_three_of_a_letter(&[w.clone()]))) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _ie_variants(args: &[Py]) -> Py {
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
    let mut other: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("Words one ie/ei swap away (\"recieve\" -> \"receive\").");
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("ie"), py_str("ei")]), py_tuple(vec![py_str("ei"), py_str("ie")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pair = __unpk2[0].clone();
    other = __unpk2[1].clone();
    start = mcall(&word.clone(), "find", &[pair.clone()]);
    while py_truthy(&Py::Bool(!py_eq(&start.clone(), &op_neg(&Py::Int(1))))) {
    let _ = mcall(&out.clone(), "add", &[op_add(&op_add(&slice_or_raise(&word.clone(), &Py::None, &start.clone(), &Py::None), &other.clone()), &slice_or_raise(&word.clone(), &op_add(&start.clone(), &Py::Int(2)), &Py::None, &Py::None))]);
    start = mcall(&word.clone(), "find", &[pair.clone(), op_add(&start.clone(), &Py::Int(1))]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _missing_vowel(args: &[Py]) -> Py {
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
    let mut cluster: Py = Py::None;
    let mut end: Py = Py::None;
    let mut i: Py = Py::None;
    let mut out: Py = Py::None;
    let mut start: Py = Py::None;
    let mut vowel: Py = Py::None;
    let _ = py_str("The words a vowel left out of the middle of a long word would make: \"daughtr\" ->\n    \"daughter\", \"brthday\" -> \"birthday\", \"bicycl\" -> \"bicycle\".\n\n    A vowel is the letter people leave out of the middle of a word they are typing fast, and\n    none of the repairs the reader already has can put one back. The new vowel has to sit in a\n    cluster of three or more consonants (\"daughtr\" has \"ghtr\", \"brthday\" has \"brth\"), because\n    that is the shape English does not have and the vowel is the sound that is missing from it.\n    Two consonants are every sound a word needs (\"martin\", \"statin\", \"toast\") and a cluster\n    that opens with an s is English of its own (\"street\", \"sprint\"), so a vowel is never put\n    there; that is what keeps a name a name and a drug a drug.\n\n    Only a word of five letters or more is tried, and only one the lexicon does not know, so a\n    real word is never touched: below five there is no room left for a misspelt word to be told\n    from a real one (\"thnks\" is \"thanks\"; \"thnk\" is a keystroke and a word). The caller keeps\n    the repair only when exactly one word of the lexicon fits.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &word.clone())) } }) {
    return py_tuple(vec![]);
    }
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), op_add(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    i = __item.clone();
    start = i.clone();
    while py_truthy(&{ let __b = op_gt(&start.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_VOWELS(), &geti(&word.clone(), &op_sub(&start.clone(), &Py::Int(1))))) } }) {
    let __aug2 = op_sub(&start.clone(), &Py::Int(1));
    start = __aug2;
    }
    end = i.clone();
    while py_truthy(&{ let __b = op_lt(&end.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_VOWELS(), &geti(&word.clone(), &end.clone()))) } }) {
    let __aug3 = op_add(&end.clone(), &Py::Int(1));
    end = __aug3;
    }
    cluster = slice_or_raise(&word.clone(), &start.clone(), &end.clone(), &Py::None);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[cluster.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&cluster.clone(), &Py::Int(0)), &py_str("s"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_S_CLUSTERS(), &geti(&cluster.clone(), &Py::Int(1)))) } } } }) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&g_VOWELS());
    for __item in __iter4 {
    vowel = __item.clone();
    candidate = op_add(&op_add(&slice_or_raise(&word.clone(), &Py::None, &i.clone(), &Py::None), &vowel.clone()), &slice_or_raise(&word.clone(), &i.clone(), &Py::None, &Py::None));
    if py_truthy(&Py::Bool(py_contains(&g_BASE(), &candidate.clone()))) {
    let _ = mcall(&out.clone(), "add", &[candidate.clone()]);
    }
    }
    }
    return out.clone();
    Py::None
}
pub fn _extra_vowel(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut char: Py = Py::None;
    let mut i: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("The words a vowel typed next to another vowel would make: \"radiatior\" -> \"radiator\".\n\n    People adding a vowel where two already meet (\"ie\", \"ei\", \"io\", \"ue\") is the one extra\n    letter worth undoing: \"radiatior\", \"cookeing\". Only a long word is tried, because the\n    short ones this would bend are words in their own right (\"seing\" is one letter from \"sing\",\n    \"seizing\" from \"sizing\"), the last letter is never taken, because it is what makes\n    \"breathe\" a verb and \"breath\" a noun, and only an e or an i is removed, which is what\n    people insert. The caller keeps only the candidates the lexicon knows, and only when\n    exactly one fits.\n    ");
    if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8))) {
    return py_tuple(vec![]);
    }
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    char = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_str("ei"), &char.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&op_add(&i.clone(), &Py::Int(1)), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    before = if py_truthy(&i.clone()) { geti(&word.clone(), &op_sub(&i.clone(), &Py::Int(1))) } else { py_str("") };
    after = geti(&word.clone(), &op_add(&i.clone(), &Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VOWELS(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VOWELS(), &after.clone())) } }) {
    let _ = mcall(&out.clone(), "add", &[op_add(&slice_or_raise(&word.clone(), &Py::None, &i.clone(), &Py::None), &slice_or_raise(&word.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None))]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _a_said_for_an_ending(args: &[Py]) -> Py {
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
    let mut plural: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The words an -er, -or, -ar or -ow said as a would make: \"propa\" -> \"proper\", \"winda\" -> \"window\".\n\n    A last syllable nobody stresses comes out as an a when people write the way they talk\n    (\"propa\", \"showa\", \"brotha\", \"plumba\", \"docta\", \"simila\", \"winda\", \"narra\"), and none of\n    those is a word, so the lexicon can say what was meant: \"my winda wont shut\" was read as a\n    line about nothing while the word spelled right gets the window answer. Only a word of four\n    letters or more that ends in an a after a consonant is tried, so a real word ending in an a\n    (\"sofa\", \"pizza\", \"idea\") never reaches the guess - it is in the lexicon and fix_typo stops\n    there - and the few real words that are not are held below.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g__A_WORDS(), &word.clone()))) {
    return py_tuple(vec![]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("as")]); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), py_str("s")]), 2, None);
    stem = __unpk1[0].clone();
    plural = __unpk1[1].clone();
    } else {
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("a")]); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    let __unpk2 = py_unpack(&py_tuple(vec![slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), py_str("")]), 2, None);
    stem = __unpk2[0].clone();
    plural = __unpk2[1].clone();
    } else {
    return py_tuple(vec![]);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_VOWELS(), &slice_or_raise(&stem.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)))) {
    return py_tuple(vec![]);
    }
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("er"), py_str("or"), py_str("ar"), py_str("ow")])) { let mut ending: Py = __item.clone(); __out.push(op_add(&op_add(&stem.clone(), &ending.clone()), &plural.clone())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _dropped_letter(args: &[Py]) -> Py {
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
    let _ = py_str("The words a missing w would make: \"kno\" -> \"know\", \"thro\" -> \"throw\", \"shado\" -> \"shadow\".\n\n    Only a w is put back. It is the letter people leave out of a word and never hear missing\n    (\"know\", \"throw\", \"shadow\", \"answer\"), so the repair cannot turn a real word into another\n    one the way a general dropped letter can; the caller keeps only the candidates the lexicon\n    knows, which is what makes \"kno\" safe.\n\n    A word with a w already in it keeps one: \"the windo wont shut\" was read as a word the reader\n    did not know, so the window that will not shut was answered \"Right. What have you tried so\n    far?\" instead of the card. The w people leave off the end is the same w wherever the other\n    one sits, and the candidate still has to be a word of the lexicon, so the guard is on the\n    letter the repair would add rather than on the word (\"window\", \"willow\").\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("w")]) } }) {
    return py_tuple(vec![]);
    }
    return py_tuple(vec![op_add(&word.clone(), &py_str("w"))]);
    Py::None
}
pub fn _an_ing_said_as_n(args: &[Py]) -> Py {
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
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut known: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The -ing form typed with the i and the g both gone: \"gettn\" -> \"getting\".\n\n    A word ending in three letters that are all consonants and the last of them an n is an -ing\n    with nothing left of the ending but the n: \"forgettn\", \"gettn\", \"sittn\", \"chattn\". The shape\n    is what makes it safe - no English word ends that way, because a syllable needs a vowel - so\n    a word that has one in its last three letters (\"basin\", \"satin\", \"salmon\") is left alone\n    even when it is not in the lexicon, and one the lexicon holds is never reached at all.\n\n    The verb under it may double its last letter (\"get\" -> \"getting\", \"sit\" -> \"sitting\"), so\n    both spellings are tried and the caller keeps the one the lexicon knows.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("n")]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone())) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&word.clone(), &op_neg(&Py::Int(3)), &op_neg(&Py::Int(1)), &Py::None)) { let mut char: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_VOWELS(), &char.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    forms = py_list(vec![op_add(&stem.clone(), &py_str("ing"))]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&stem.clone(), &op_neg(&Py::Int(1))), &geti(&stem.clone(), &op_neg(&Py::Int(2))))) } }) {
    let _ = mcall(&forms.clone(), "append", &[op_add(&op_add(&slice_or_raise(&stem.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &geti(&stem.clone(), &op_neg(&Py::Int(1)))), &py_str("ing"))]);
    }
    known = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &form.clone()))) {  __out.push(form.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[known.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return mcall(&known.clone(), "pop", &[]);
    }
    return Py::None;
    Py::None
}
pub fn _a_leading_h(args: &[Py]) -> Py {
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
    let _ = py_str("The word an h put back on the front would make: \"eadache\" -> \"headache\", \"eart\" -> \"heart\".\n\n    An h at the front of a word is the letter people say and never hear, so it is the one that\n    goes missing when somebody types the way they talk: \"iv got a bad eadache\", \"my eart is\n    racing\", \"the ouse is freezing\", \"i need an ospital\". Only the front is tried, only a word\n    that opens on a vowel, and the caller keeps only candidates the lexicon knows - which is what\n    makes it safe: \"eadache\" is one letter from \"earache\" as well, and that repair reached the\n    earache card, so the h is the shorter path to the word the line is about. A word the lexicon\n    holds never arrives here (\"edge\", \"and\", \"am\" are words), and a word of three letters is\n    left alone because \"ed\" and \"ed\" are not a slip of anything the bot should answer with.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None))) } }) {
    return py_tuple(vec![]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BASE(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } }) {
    return py_tuple(vec![]);
    }
    return py_tuple(vec![op_add(&py_str("h"), &word.clone())]);
    Py::None
}
pub fn _e_kept_before_ing(args: &[Py]) -> Py {
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
    let mut shorter: Py = Py::None;
    let _ = py_str("The word an e kept before -ing would make: \"moveing\" -> \"moving\", \"takeing\" -> \"taking\".\n\n    A verb that ends in an e drops it before the ending, and a fast typist writes it anyway.\n    Only this ending is tried, because it is the one where the letter before the e decides\n    whether the e stays (\"seeing\", \"freeing\", \"canoeing\" all keep it) and that is a rule the\n    reader cannot hear - so the candidate has to be a word the lexicon knows and it has to be\n    the only one. The four real words that do keep the e are named above.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("eing")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_E_KEPT_WORDS(), &word.clone())) } } } }) {
    return py_tuple(vec![]);
    }
    shorter = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(4)), &Py::None), &py_str("ing"));
    return if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &shorter.clone()))) { py_tuple(vec![shorter.clone()]) } else { py_tuple(vec![]) };
    Py::None
}
pub fn _swapped_pair(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pair: Py = Py::None;
    let _ = py_str("The words swapping two letters typed the wrong way round would make, or ().\n\n    \"collasped\" is \"collapsed\" with the p and the s the wrong way round, and that is how a word\n    comes out when two keys are hit in the wrong order. Only a long word is tried, and the caller\n    keeps the candidates the vocabulary knows and only one of them: \"trail\"/\"trial\" and\n    \"form\"/\"from\" are both real words, so they never reach here, and a word with two fittings is\n    left alone. Five letters is the floor, the same as the vowel reader's: \"hpone\" is \"phone\",\n    and a word the bot's own answers write is not a slip at all.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { _a_word_the_bot_writes(&[word.clone()]) } }) {
    return py_tuple(vec![]);
    }
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    at = __item.clone();
    pair = slice_or_raise(&word.clone(), &at.clone(), &op_add(&at.clone(), &Py::Int(2)), &Py::None);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[pair.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&pair.clone(), &Py::Int(0)), &geti(&pair.clone(), &Py::Int(1)))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VOWELS(), &geti(&pair.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VOWELS(), &geti(&pair.clone(), &Py::Int(1)))) } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "add", &[op_add(&op_add(&op_add(&slice_or_raise(&word.clone(), &Py::None, &at.clone(), &Py::None), &geti(&word.clone(), &op_add(&at.clone(), &Py::Int(1)))), &geti(&word.clone(), &at.clone())), &slice_or_raise(&word.clone(), &op_add(&at.clone(), &Py::Int(2)), &Py::None, &Py::None))]);
    }
    return out.clone();
    Py::None
}
pub fn _two_letters_the_wrong_way_round(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut made: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pair: Py = Py::None;
    let _ = py_str("The words swapping two letters typed the wrong way round would make, vowels included:\n    \"brithday\" -> \"birthday\", \"cra\" -> \"car\", \"aeg\" -> \"age\", \"paekd\" is not one.\n\n    This is the same slip `_swapped_pair` reads, without its two limits: that one wants a long\n    word and two consonants, because in a real word a vowel pair is how one word becomes\n    another (\"form\" is not \"from\", \"trail\" is not \"trial\"). Neither limit is needed when the\n    typed word is not a word at all, which is what this is for: a person typing fast puts two\n    keys down in the wrong order, and half the time one of them is a vowel (\"brithday\",\n    \"taht\", \"hte\", \"aeg\"). Everything the lexicon holds is left alone, and the caller keeps the\n    repair only when exactly one word fits, so a slip with two readings (\"cra\" is one swap from\n    \"car\" and from \"arc\") is left as it was typed.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { _a_word_the_bot_writes(&[word.clone()]) } } } }) {
    return py_tuple(vec![]);
    }
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    at = __item.clone();
    pair = slice_or_raise(&word.clone(), &at.clone(), &op_add(&at.clone(), &Py::Int(2)), &Py::None);
    if py_truthy(&Py::Bool(py_eq(&geti(&pair.clone(), &Py::Int(0)), &geti(&pair.clone(), &Py::Int(1))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&Py::Bool(py_contains(&g_VOWELS(), &geti(&pair.clone(), &Py::Int(0)))), &Py::Bool(py_contains(&g_VOWELS(), &geti(&pair.clone(), &Py::Int(1))))))) {
    continue;
    }
    made = op_add(&op_add(&op_add(&slice_or_raise(&word.clone(), &Py::None, &at.clone(), &Py::None), &geti(&pair.clone(), &Py::Int(1))), &geti(&pair.clone(), &Py::Int(0))), &slice_or_raise(&word.clone(), &op_add(&at.clone(), &Py::Int(2)), &Py::None, &Py::None));
    if py_truthy(&Py::Bool(!py_contains(&g_NO_SWAP_TO(), &made.clone()))) {
    let _ = mcall(&out.clone(), "add", &[made.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _words_the_bot_writes(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut _u: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut field: Py = Py::None;
    let mut knowledge_entries: Py = Py::None;
    let mut knowledge_lookup: Py = Py::None;
    let mut known: Py = Py::None;
    let mut text: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every word the knowledge base's own keys and answers use, worked out once.\n\n    The guard on a swapped pair is \"only read a word the reader does not know as a word\", and\n    LEXICON cannot answer that on its own: it is the base words and the forms made from them, not\n    a dictionary, so \"stain\", \"order\", \"paper\", \"spilt\" and \"matters\" are not in it and each is\n    one swap from a word it does hold (\"satin\", \"roder\", \"apper\", \"split\", \"mattres\"). Read that\n    way, the reader bent 138 of the deck's own lines.\n\n    The knowledge base cannot make that mistake: its keys and answers are written by the bot, so\n    every word in them is well spelled by construction - the same reasoning `tools/typo_scan.py`\n    gives for scanning them. Asking it is a lazy import because the answer modules import it\n    anyway, and the result is held: one build a process, about a second, and nothing after that.\n    ");
    if py_truthy(&Py::Bool(py_is(&g__A_WORD_THE_BOT_WRITES(), &Py::None))) {
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    knowledge_entries = py_func("entries", |args: &[Py]| pyrt::xrefs::xcall("knowledge", "entries", args));
    knowledge_lookup = pyrt::xrefs::xmodule("knowledge.lookup");
    let __iter6 = py_iter_or_raise(&py_call(&knowledge_entries, &[]));
    for __item in __iter6 {
    entry = __item.clone();
    let __iter7 = py_iter_or_raise(&py_tuple(vec![py_str("reply"), py_str("ask"), py_str("key"), py_str("id"), py_str("domain")]));
    for __item in __iter7 {
    field = __item.clone();
    value = call_builtin("getattr", &[entry.clone(), field.clone(), py_str("")]).unwrap_or_else(|e| exc::raise(e));
    let __iter8 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { py_list(vec![value.clone()]) });
    for __item in __iter8 {
    text = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&words.clone(), "update", &[mcall(&knowledge_lookup.clone(), "tokens", &[text.clone()])]);
    }
    }
    }
    }
    let __unpk9 = py_unpack(&mcall(&knowledge_lookup.clone(), "_vocabulary", &[py_call(&knowledge_entries, &[])]), 3, None);
    _u = __unpk9[0].clone();
    _u = __unpk9[1].clone();
    known = __unpk9[2].clone();
    let _ = mcall(&words.clone(), "update", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&known.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    s__A_WORD_THE_BOT_WRITES(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&mcall(&word.clone(), "isalpha", &[])) {  __out.push(mcall(&word.clone(), "lower", &[])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    }
    return g__A_WORD_THE_BOT_WRITES();
    Py::None
}
pub fn _a_word_the_bot_writes(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the knowledge base writes this word as it stands, so it is not a slip.");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "TYPED_WORDS"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone())) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&_words_the_bot_writes(&[]), &word.clone()));
    Py::None
}
pub fn _bones(args: &[Py]) -> Py {
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
    let _ = py_str("The consonants of a word, in order: \"collapsed\" and \"collpsd\" are both \"cllpsd\".");
    return mcall(&py_str(""), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&word.clone()) { let mut char: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_VOWELS(), &char.clone()))) {  __out.push(char.clone()); } } py_list(__out) })() }]);
    Py::None
}
pub fn _longest_consonant_run(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut char: Py = Py::None;
    let mut run: Py = Py::None;
    let _ = py_str("The longest stretch of letters in a word with no vowel in it.\n\n    A vowel typed past closes a gap and leaves a run behind - \"collpsd\" has \"psd\", \"bleedng\"\n    has \"dng\" - and an ordinary word hardly ever has three consonants in a row. That is what\n    tells a typo from a word: \"balding\" and \"sizing\" are only two (\"ld\", \"ng\"), and they are\n    words, so they must not be read as \"bleeding\" and \"seizing\" (the same bones).\n    ");
    run = Py::Int(0);
    best = Py::Int(0);
    let __iter1 = py_iter_or_raise(&word.clone());
    for __item in __iter1 {
    char = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VOWELS(), &char.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&char.clone(), &py_str("y"))) } }) {
    run = Py::Int(0);
    } else {
    let __aug2 = op_add(&run.clone(), &Py::Int(1));
    run = __aug2;
    if py_truthy(&op_gt(&run.clone(), &best.clone())) {
    best = run.clone();
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _an_emergency_word_left_out(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut known: Py = Py::None;
    let _ = py_str("The emergency word this is with its vowels typed past, or None.\n\n    A frightened person types fast, and the vowels are what goes: \"my mum collpsd\". A letter\n    left out is the one typo the engine does not otherwise guess at, because a word one letter\n    short of another is usually the other word - but here the only words consulted are the\n    emergency vocabulary itself (lexicon.URGENT_MISSPELLINGS' values), and only when exactly\n    one of them has the same consonants in the same order. \"cllpsd\" is \"collapsed\" and nothing\n    else; a word whose bones fit two of them is left for the curated table.\n\n    The word also has to look like a vowel was left out: three consonants in a row. Without\n    that guard the same bones bend ordinary words into emergencies - \"balding\" is \"bleeding\"\n    and \"sizing\" is \"seizing\" - and a person talking about their hair got the bleeding card.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { op_lt(&_longest_consonant_run(&[word.clone()]), &Py::Int(3)) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&g_URGENT_BONES()))) {
    let __iter1 = py_iter_or_raise(&g_URGENT_WORDS());
    for __item in __iter1 {
    known = __item.clone();
    let _ = mcall(&mcall(&g_URGENT_BONES(), "setdefault", &[_bones(&[known.clone()]), py_list(vec![])]), "append", &[known.clone()]);
    }
    }
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_URGENT_BONES(), "get", &[_bones(&[word.clone()]), py_tuple(vec![])])) { let mut known: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[known.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(known.clone()); } } py_list(__out) })() };
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&found.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _a_name_slot(args: &[Py]) -> Py {
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
    let mut before_before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before_before"))) };
    let mut word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let _ = py_str("True when this word stands where a name goes, so a spelling repair may not touch it.\n\n    The lexicon makes an \"-ing\" out of every word it holds, so \"crispin\" is one letter from\n    \"crisping\" and \"justin\" from \"justing\": the left-off-g repair read both as verbs, and the\n    line came back as \"Noted: your brother is called crisping\" and \"Your cousin justing is\n    coming - noted\". The store had the name right both times (`others` holds \"crispin\" and\n    \"justin\"); it is the echo of the repaired line that was wrong.\n\n    Two frames are named, and nothing else: the word after \"called\"/\"named\"/\"name is\", where any\n    name may stand, and a known name right beside the person it belongs to (\"my cousin justin\").\n    Only a known name is held back in the second frame, because a verb sits there too: \"how do i\n    stop me baby wakin up at night\" and \"my sister sed she would come\" are both the repair\n    working, and the repair stands.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &word.clone())) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_NAME_FRAMES(), &before.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BE_WORDS(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&before_before.clone(), &py_str("name"))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CHAT_NAMES(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NAME_ROLL(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("me"), py_str("you"), py_str("u")]), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NAME_CALLS(), &before_before.clone())) } }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&before.clone(), &py_str("not"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_bitor(&g_BE_WORDS(), &py_set(vec![py_str("im"), py_str("i")])), &before_before.clone())) } } } } } } } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_KIN(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POSSESSIVES(), &before_before.clone())) } } } };
    Py::None
}
pub fn _dropped_g(args: &[Py]) -> Py {
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
    let mut doubled: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The -g people leave off a typed word: \"doin\" -> \"doing\", \"mornin\" -> \"morning\".\n\n    Only when the word is not one the lexicon knows and the form with the g is, so a real\n    word is never changed. The lexicon is missing ordinary -in words (\"satin\", \"raisin\"), so\n    those are named in lexicon.REAL_IN: \"a satin dress\" must not read as \"a sating dress\".\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("in")]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "REAL_IN"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone())) } } } } } } } }) {
    return Py::None;
    }
    longer = op_add(&word.clone(), &py_str("g"));
    if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &longer.clone()))) {
    return longer.clone();
    }
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    if py_truthy(&{ let __b = _drops_a_final_e(&[stem.clone()]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    dropped = _ing_form(&[stem.clone()]);
    if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &dropped.clone()))) {
    return dropped.clone();
    }
    }
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&stem.clone(), &op_neg(&Py::Int(1))), &geti(&stem.clone(), &op_neg(&Py::Int(2))))) } }) {
    stem = slice_or_raise(&stem.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&{ let __b = _doubles_its_last_letter(&[stem.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BASE(), &stem.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &stem.clone())) } } } }) {
    doubled = op_add(&op_add(&stem.clone(), &geti(&stem.clone(), &op_neg(&Py::Int(1)))), &py_str("ing"));
    if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &doubled.clone()))) {
    return doubled.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _stopped_th(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut suffix: Py = Py::None;
    let _ = py_str("The -thing word a stopped th is said as: \"anyfing\" -> \"anything\", \"nuffing\" -> \"nothing\".\n\n    Th-stopping (\"thing\" said as \"fing\") is regular in several of the accents people type in,\n    so it is a spelling a reader has to expect: \"anyfing i can do at lunch\" was read as nothing\n    in particular and answered with the stock line about a hard day. Only a word the lexicon\n    does not hold is touched, and only the four -thing words are reachable, so a line about\n    \"stuffing\" or \"waterproofing\" is left alone.\n\n    The same accents drop the g and the th: \"nuffin\", \"anyfink\" and \"sumfin\" are the same word\n    typed three ways, and people write all three, so the ending is not just \"fing\". The stem\n    still has to be one of the handful people write, which is what keeps \"muffin\", \"puffins\",\n    \"griffins\" and \"thinking\" out: no real word is built from one of these stems.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &word.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("fings"), py_str("fing"), py_str("finks"), py_str("fink"), py_str("fins"), py_str("fin")]));
    let mut __else2 = false;
    for __item in __iter1 {
    suffix = __item.clone();
    if py_truthy(&mcall(&word.clone(), "endswith", &[suffix.clone()])) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    break;
    }
    }
    return Py::None;
    plural = mcall(&suffix.clone(), "endswith", &[py_str("s")]);
    found = mcall(&g_STOPPED_TH_STEMS(), "get", &[stem.clone()]);
    return if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { plural.clone() } }) { op_add(&found.clone(), &py_str("s")) } else { found.clone() };
    Py::None
}
pub fn _shun_variant(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut singular: Py = Py::None;
    let mut spelled: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The -tion people spell the way it sounds: \"infecshun\" -> \"infection\".\n\n    English spells the /shun/ sound at the end of a word -tion, so a typed word ending in\n    \"shun\" (or \"kshun\", with the k of \"infection\" spelled out too) is that word, and no\n    ordinary word ends that way except \"shun\" itself. Nothing here needs the lexicon: the whole point is a word the\n    lexicon does not have (\"infection\" is not one of its words either).\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &word.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("shuns")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)) } }) {
    singular = _shun_variant(&[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    if py_truthy(&singular.clone()) {
    return op_add(&singular.clone(), &py_str("s"));
    }
    return Py::None;
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("kshun")])) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(5)), &Py::None);
    } else {
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("shun")])) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(4)), &Py::None);
    } else {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&stem.clone(), "isalpha", &[]))) } }) {
    return Py::None;
    }
    kept = op_add(&stem.clone(), &py_str("s"));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![op_add(&stem.clone(), &py_str("shion")), op_add(&stem.clone(), &py_str("ssion")), op_add(&stem.clone(), &py_str("sion")), op_add(&stem.clone(), &py_str("tion")), op_add(&kept.clone(), &py_str("tion")), op_add(&kept.clone(), &py_str("ssion")), op_add(&kept.clone(), &py_str("sion")), op_add(&stem.clone(), &py_str("cian"))]));
    for __item in __iter1 {
    spelled = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LEXICON(), &spelled.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SHION_WORDS(), &spelled.clone())) } }) {
    return spelled.clone();
    }
    }
    return op_add(&stem.clone(), &py_str("tion"));
    Py::None
}
pub fn _said_with_an_f(args: &[Py]) -> Py {
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
    let mut spelled: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The \"th\" sound typed as an \"f\": \"fanks\" -> \"thanks\", \"froat\" -> \"throat\".\n\n    A swapped sound is not a slip a one-letter repair can find - \"fanks\" is two letters from\n    \"thanks\" - and it is common enough that \"fanks\", \"fink\" and \"froat\" arrive as whole\n    lines. Only the front of the word is read this way, because that is where the sound is\n    swapped, and only for a word that is not a word itself whose \"th\" spelling the reading\n    already knows: \"fan\", \"fat\", \"fro\" and \"fort\" are real words and stay as they are, and\n    \"sumfink\" is left to the spelling tables.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_F_TYPED_FOR_TH(), &word.clone()))) {
    return geti(&g_F_TYPED_FOR_TH(), &word.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "startswith", &[py_str("f")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &word.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::None;
    }
    spelled = op_add(&py_str("th"), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LEXICON(), &spelled.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &spelled.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), &spelled.clone())) } } } }) {
    return spelled.clone();
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("s"), py_str("es"), py_str("d"), py_str("ed"), py_str("ing")]));
    for __item in __iter1 {
    ending = __item.clone();
    stem = slice_or_raise(&spelled.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    if py_truthy(&{ let __b = mcall(&spelled.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &stem.clone())) } } } }) {
    return spelled.clone();
    }
    }
    if py_truthy(&{ let __b = mcall(&spelled.clone(), "endswith", &[py_str("in")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &op_add(&spelled.clone(), &py_str("g")))) } }) {
    return op_add(&spelled.clone(), &py_str("g"));
    }
    return Py::None;
    Py::None
}
pub fn _said_with_an_f_anywhere(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut letter: Py = Py::None;
    let mut spelled: Py = Py::None;
    let _ = py_str("The same swapped sound away from the front: \"wif\" -> \"with\", \"teef\" -> \"teeth\".\n\n    The sound is swapped in the middle and at the end of a word too, and there it costs a line\n    its answer rather than only its spelling: \"my teef hurt\" was asked \"Does it come and go, or\n    is it there all the time?\" while \"my teeth hurt\" reaches the dentist, \"mouf ulcers\" reached\n    nothing at all, \"how do i clean a baff\" was a shrug, and a line said back to the person\n    repeated \"your birfday is the 3rd of may\". Only the f is touched, so the rest of the word\n    must already be the word it is meant to be: the \"th\" spelling has to be one the engine\n    knows, and the typed word must not be a word it knows (\"if\", \"off\", \"half\", \"loaf\" and\n    \"shelf\" stay as they are). Only an f nobody could mean as an f is read, which is why this\n    runs after `fix_typo` and the letters before it.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&word.clone(), &py_str("f"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &word.clone())) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ff")])) {
    if py_truthy(&mcall(&word.clone(), "startswith", &[py_str("w")])) {
    return Py::None;
    }
    spelled = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), &py_str("th"));
    return if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &spelled.clone()))) { spelled.clone() } else { Py::None };
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    letter = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&letter.clone(), &py_str("f")))) {
    continue;
    }
    spelled = op_add(&op_add(&slice_or_raise(&word.clone(), &Py::None, &at.clone(), &Py::None), &py_str("th")), &slice_or_raise(&word.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None));
    if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &spelled.clone()))) {
    return spelled.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_thing_where_a_thing_goes(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut thing: Py = Py::None;
    let _ = py_str("The person or thing a little word stands for in the place one goes, or None.\n\n    \"my wif\" and \"the wif\" are the wife typed without the last letter. This has to be asked\n    before the chat table, which answers `wif` with \"with\" wherever it appears.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_POSSESSIVES(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_THING_GOES_AFTER(), &before.clone())) } }) {
    return Py::None;
    }
    thing = mcall(&g_A_THING_TYPED_AS_A_LITTLE_WORD(), "get", &[word.clone()]);
    return if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &thing.clone()))) { thing.clone() } else { Py::None };
    Py::None
}
pub fn _missing_e(args: &[Py]) -> Py {
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
    let mut longer: Py = Py::None;
    let _ = py_str("The \"-ed\" people type as \"-d\": \"askd\" -> \"asked\", \"wantd\" -> \"wanted\", \"movd\" -> \"moved\".\n\n    A bare d is how no English word ends after a k, t, v, p, s, m, f or g, so the letter before\n    it is the sign the e was dropped; after a vowel, a l, an n or an r it is how plenty of words\n    end (\"road\", \"old\", \"mend\", \"bird\"), and there the repair could only bend a real word, so\n    those are left to the curated MISSPELLINGS map. The form with the e also has to be one the\n    lexicon knows, which is what keeps \"tird\" from becoming anything but \"tired\".\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("d")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &word.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("yd")])) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VOWELS(), &geti(&word.clone(), &op_neg(&Py::Int(3))))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &op_neg(&Py::Int(3))), &py_str("y"))) } }) {
    return Py::None;
    }
    longer = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), &py_str("ied"));
    return if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &longer.clone()))) { longer.clone() } else { Py::None };
    }
    if py_truthy(&Py::Bool(!py_contains(&g_MISSING_E_AFTER(), &geti(&word.clone(), &op_neg(&Py::Int(2)))))) {
    return Py::None;
    }
    longer = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ed"));
    return if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &longer.clone()))) { longer.clone() } else { Py::None };
    Py::None
}
pub fn _short_double(args: &[Py]) -> Py {
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
    let mut keep: Py = Py::None;
    let mut variant: Py = Py::None;
    let mut variants: Py = Py::None;
    let _ = py_str("A short word typed with a doubled letter: \"hii\" -> \"hi\", \"okk\" -> \"ok\", \"byee\" -> \"bye\".\n\n    Words under four letters used to be left alone, and they are where most of the one-word\n    replies live. Only an un-doubled form that is a frequent word is taken, and only for a\n    word the lexicon does not know, so \"all\" and \"off\" are never touched.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &word.clone())) } }) {
    return Py::None;
    }
    variants = _double_variants(&[word.clone()]);
    if py_truthy(&Py::Bool(py_eq(&geti(&word.clone(), &op_neg(&Py::Int(1))), &geti(&word.clone(), &op_neg(&Py::Int(2)))))) {
    let _ = mcall(&variants.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    keep = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&variants.clone()) { let mut variant: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LEXICON(), &variant.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FREQUENT(), &variant.clone())) } }) {  __out.push(variant.clone()); } } py_list(__out) })() };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let keep = keep.clone(); let variant = variant.clone(); let variants = variants.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let keep = keep.clone(); let variant = variant.clone(); let variants = variants.clone(); let word = word.clone(); let variant: Py = __kwp(args, 0, "variant").unwrap_or(Py::None); mcall(&g_COMMON_RANK(), "get", &[variant.clone(), op_pow(&Py::Int(10), &Py::Int(6))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&keep.clone(), "sort", args))(&__args) };
    return if py_truthy(&keep.clone()) { geti(&keep.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _one_letter_apart(args: &[Py]) -> Py {
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
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut differ: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut long: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("True when `word` is `other` with one letter added, left out, swapped round or replaced\n    by another vowel: the shapes people type at speed (\"knackerd\", \"exausted\", \"nerveus\").");
    if py_truthy(&op_gt(&call_builtin("abs", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    differ = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&geti(&word.clone(), &i.clone()), &geti(&other.clone(), &i.clone())))) {  __out.push(i.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[differ.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    i = geti(&differ.clone(), &Py::Int(0));
    return { let __b = Py::Bool(py_contains(&g_VOWEL_SET_OR_Y(), &geti(&word.clone(), &i.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VOWEL_SET_OR_Y(), &geti(&other.clone(), &i.clone()))) } };
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[differ.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    let __unpk1 = py_unpack(&differ.clone(), 2, None);
    i = __unpk1[0].clone();
    j = __unpk1[1].clone();
    return { let __b = Py::Bool(py_eq(&j.clone(), &op_add(&i.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&word.clone(), &i.clone()), &geti(&other.clone(), &j.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &j.clone()), &geti(&other.clone(), &i.clone()))) } } } };
    }
    return Py::Bool(false);
    }
    let __unpk2 = py_unpack(&if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![word.clone(), other.clone()]) } else { py_tuple(vec![other.clone(), word.clone()]) }, 2, None);
    short = __unpk2[0].clone();
    long = __unpk2[1].clone();
    i = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&i.clone(), &call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&short.clone(), &i.clone()), &geti(&long.clone(), &i.clone()))) } }) {
    let __aug3 = op_add(&i.clone(), &Py::Int(1));
    i = __aug3;
    }
    if py_truthy(&Py::Bool(py_eq(&i.clone(), &call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_eq(&slice_or_raise(&short.clone(), &i.clone(), &Py::None, &Py::None), &slice_or_raise(&long.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None)))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_is(&word.clone(), &long.clone()))) {
    return Py::Bool(py_contains(&g_VOWEL_SET(), &geti(&long.clone(), &i.clone())));
    }
    return Py::Bool(true);
    Py::None
}
pub fn _one_letter_wrong(args: &[Py]) -> Py {
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
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut differ: Py = Py::None;
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut long: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("True when one letter added, left out, swapped with its neighbour or typed for another\n    makes `other` out of `word`.\n\n    `_one_letter_apart` says the same thing for feelings and parts, with two limits that are\n    right there and wrong here: a letter typed *into* a word only counts when it is a vowel,\n    and a letter typed *for* another only when both are vowels. Those limits keep \"gland\" from\n    being \"glad\"; next to a closed list of the commonest verbs there is nothing to confuse it\n    with, so any letter counts.\n    ");
    if py_truthy(&op_gt(&call_builtin("abs", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    differ = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&geti(&word.clone(), &i.clone()), &geti(&other.clone(), &i.clone())))) {  __out.push(i.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[differ.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[differ.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    let __unpk1 = py_unpack(&differ.clone(), 2, None);
    i = __unpk1[0].clone();
    j = __unpk1[1].clone();
    return { let __b = Py::Bool(py_eq(&j.clone(), &op_add(&i.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&word.clone(), &i.clone()), &geti(&other.clone(), &j.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &j.clone()), &geti(&other.clone(), &i.clone()))) } } } };
    }
    return Py::Bool(false);
    }
    let __unpk2 = py_unpack(&if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![word.clone(), other.clone()]) } else { py_tuple(vec![other.clone(), word.clone()]) }, 2, None);
    short = __unpk2[0].clone();
    long = __unpk2[1].clone();
    i = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&i.clone(), &call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&short.clone(), &i.clone()), &geti(&long.clone(), &i.clone()))) } }) {
    let __aug3 = op_add(&i.clone(), &Py::Int(1));
    i = __aug3;
    }
    return Py::Bool(py_eq(&slice_or_raise(&short.clone(), &i.clone(), &Py::None, &Py::None), &slice_or_raise(&long.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None)));
    Py::None
}
pub fn _a_word_the_reading_knows(args: &[Py]) -> Py {
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
    let mut stem: Py = Py::None;
    let _ = py_str("Whether the topics' vocabulary or the lexicon already holds this word or a form of it.\n\n    \"seeding\" is not in the lexicon as it stands, and the bot does not write it, so a repair\n    read it as \"seedling\" - a word bent into a different word, in a line about a lawn. The\n    word, its stem and its everyday endings are asked, because the lexicon holds base words\n    and the topics hold the names of things (\"seed\" is a seed, \"seeds\" and \"seeded\" are forms\n    of it).\n    ");
    if py_truthy(&_known_word(&[word.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_stems_of(&[word.clone()])) { let mut stem: Py = __item.clone(); __out.push(_known_word(&[stem.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _known_word(args: &[Py]) -> Py {
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
    let _ = py_str("Whether one word is held by the lexicon or by the topics' vocabulary.");
    return { let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THING_WORDS(), &word.clone())) } };
    Py::None
}
pub fn _stems_of(args: &[Py]) -> Py {
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
    let mut stems: Py = Py::None;
    let _ = py_str("The words `word` could be an everyday ending of.");
    stems = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    let _ = mcall(&stems.clone(), "update", &[py_tuple(vec![slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("e"))])]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ed")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    let _ = mcall(&stems.clone(), "update", &[py_tuple(vec![slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None), slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)])]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("es")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    let _ = mcall(&stems.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None)]);
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }) {
    let _ = mcall(&stems.clone(), "add", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    return stems.clone();
    Py::None
}
pub fn _a_thing_a_letter_short(args: &[Py]) -> Py {
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
    let mut fits: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The thing `word` is one letter short of, or None when none or two fit.\n\n    The same reading as `_a_core_verb_typed_wrong`, with the topics' own vocabulary as the\n    closed list: a letter left out of a thing's name is the commonest slip in a line about a\n    thing (\"my fidge is broken\", \"my frige is making a noise\", \"how do i check my raiator\"),\n    and next to a list of things there is little to confuse it with. Eight household and garden\n    words typed a letter short reached no card on main: \"my fidge is broken\" got \"That would\n    bother me too. Tell me what it does\", \"how do i clean my frige\" and \"how do i clean my\n    caret\" got the search shrug, \"my biler is broken\" got \"Right. What have you tried so far?\",\n    \"how do i check my raiator\" and \"how often should i water my grss\" got \"I don't know enough\n    to answer that honestly\", and \"my feezer is frosting up\" got \"I'm listening — say more if it\n    helps.\"\n\n    The caller has already refused a real word and a name, so what is left is a word nothing\n    knows. Two things one letter away still say nothing (\"roter\" is a router and a roster),\n    which is the tie rule everywhere else.\n\n    The letter put back is one left out of the middle, never one on either end: a letter on the\n    end of a whole word is how a word grows, not how a typo looks. \"repeat that step by step\"\n    was read as \"steps\" and the follow-up that repeats the last answer fell to the search\n    shrug, and \"wols\" was read as \"wools\" - both a word of its own bent into a longer one.\n    \"step\" is not in the lexicon as it stands, so the guard against a real word did not catch\n    it; keeping the first and last letter does, and none of the repairs above loses one.\n\n    A word the reading knows somewhere else is not a word missing a letter. The lexicon is a\n    curated subset and not a dictionary, so the guard against a real word let 63 of the deck's\n    own 9,580 words through as \"slips\": \"on fire\" became \"on fibre\" and lost the 999 answer,\n    \"we have mice\" became a bolognese, \"my bother\", \"the plans\", \"it moves\" and \"the tables\"\n    were bent as well. Every one of them is a word the bot writes in its own answers, or a form\n    of one the topics name (\"seeding\" is a seed), so the reading asks both before it bends\n    anything - the same guard `_a_core_verb_typed_wrong` has always had.\n    ");
    if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _a_word_the_bot_writes(&[word.clone()]); if py_truthy(&__b) { __b } else { _a_word_the_reading_knows(&[word.clone()]) } }) {
    return Py::None;
    }
    fits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_THING_WORDS_BY_LENGTH(), "get", &[op_add(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), py_tuple(vec![])])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&w.clone(), &Py::Int(0)), &geti(&word.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&w.clone(), &op_neg(&Py::Int(1))), &geti(&word.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { _one_letter_wrong(&[word.clone(), w.clone()]) } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&fits.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _a_core_verb_typed_wrong(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("The everyday verb `word` is one letter away from, right after a subject, or None.\n\n    \"i tke the bus to work\" was answered \"I'm listening\", \"i gte home at 6\" and \"i hav a shower\n    then work\" the same, \"i mak dinner at 7\" nothing at all; \"i wek up at 6 every morning\" came\n    back as \"You week up at 6 every morning\", because the corrector had a spelling to guess at\n    and this word has a reading from where it sits. The word being read is not in the lexicon\n    and is not a name, and one letter out of place in \"take\", \"get\", \"have\" or \"make\" is that\n    word: nothing else in the language is one edit from exactly one of them.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_BEFORE_A_VERB(), &before.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_words_the_bot_writes(&[]), &word.clone())) } } } }) {
    return Py::None;
    }
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CORE_VERBS()) { let mut verb: Py = __item.clone(); if py_truthy(&_one_letter_wrong(&[word.clone(), verb.clone()])) {  __out.push(verb.clone()); } } py_set(__out) })() };
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("d")])) {
    let _ = mcall(&found.clone(), "discard", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AFTER_A_VERB(), &after.clone())) } }) {
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CORE_VERBS()) { let mut verb: Py = __item.clone(); if py_truthy(&_its_vowels_typed_wrong(&[word.clone(), verb.clone()])) {  __out.push(verb.clone()); } } py_set(__out) })() };
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return mcall(&found.clone(), "pop", &[]);
    }
    return Py::None;
    Py::None
}
pub fn _its_vowels_typed_wrong(args: &[Py]) -> Py {
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
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut consonants: Py = Py::None;
    let mut letter: Py = Py::None;
    let _ = py_str("True when `other` is `word` with a vowel (or a whole syllable) typed wrong.");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &other.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&geti(&word.clone(), &Py::Int(0)), &geti(&other.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&other.clone(), "startswith", &[geti(&word.clone(), &Py::Int(0))]))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&op_gt(&call_builtin("abs", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    consonants = mcall(&py_str(""), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&word.clone()) { let mut letter: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_str("aeiou"), &letter.clone()))) {  __out.push(letter.clone()); } } py_list(__out) })() }]);
    return { let __b = consonants.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&consonants.clone(), &mcall(&py_str(""), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&other.clone()) { let mut letter: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_str("aeiou"), &letter.clone()))) {  __out.push(letter.clone()); } } py_list(__out) })() }]))) } };
    Py::None
}
pub fn _near_a_part(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut i: Py = Py::None;
    let mut long: Py = Py::None;
    let mut part: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("The part of the body a misspelt part is, or None.\n\n    A part is a closed list, so a word with one letter typed into it or left out of it is still\n    that part: \"cheast\" is a chest, \"tomach\" a stomach, \"shouldr\" a shoulder, \"fingr\" a\n    finger, \"nee\" a knee, \"hed\" a head. Two parts fit the word and it is left alone, and a\n    word the lexicon knows is never bent: \"and\" is not a hand, \"year\" is not an ear. The three\n    letter parts are left out, because a letter in front of one of them makes an ordinary word\n    the bot's list does not hold: \"hear\", \"wear\" and \"tear\" are each \"ear\" with one letter on.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "BODY_PARTS"), &word.clone())) } } } } } }) {
    return Py::None;
    }
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xget("understanding.lexicon", "BODY_PARTS"));
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("abs", &[op_sub(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    continue;
    }
    let __unpk2 = py_unpack(&if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![word.clone(), part.clone()]) } else { py_tuple(vec![part.clone(), word.clone()]) }, 2, None);
    short = __unpk2[0].clone();
    long = __unpk2[1].clone();
    i = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&i.clone(), &call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&short.clone(), &i.clone()), &geti(&long.clone(), &i.clone()))) } }) {
    let __aug3 = op_add(&i.clone(), &Py::Int(1));
    i = __aug3;
    }
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&short.clone(), &i.clone(), &Py::None, &Py::None), &slice_or_raise(&long.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None)))) {
    let _ = mcall(&found.clone(), "add", &[part.clone()]);
    }
    }
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { mcall(&found.clone(), "pop", &[]) } else { Py::None };
    Py::None
}
pub fn _a_game_named_another_way(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut read: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(`norm`, `words`) with a game named by its shape read as the game's own name.\n\n    \"five a side\" is a small football match and nothing else is said that way, and one word stands\n    in its place so the game sits where the person put it - in their setting clause or in their ask\n    - instead of being bolted onto the end of the line. The knockabout is the same game named by the\n    thing you do with the ball. The game's name goes into `words` as well, because that is the list\n    the knowledge base is matched on; `clean` and `plain` keep the words the person typed, which is\n    what a reply quotes back.\n\n    A line about the make-up of the game, or about finding a team, names the game without asking\n    anything about playing it, so it keeps the words it typed (see `ABOUT_THE_FORM`). So does a\n    line whose subject is a part of the body that hurts: \"i play five a side twice a week and my\n    groin is sore\" has a groin answer of its own, and naming the game put a football-injury card's\n    clarifying question ahead of it - the person's part is the subject, and the game is only where\n    they use it.\n    ");
    flat = mcall(&py_str(" "), "join", &[norm.clone()]);
    if py_truthy(&{ let __b = mcall(&g_ABOUT_THE_FORM(), "search", &[flat.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FINDING_A_TEAM(), "search", &[flat.clone()]) } }) {
    return py_tuple(vec![norm.clone(), words.clone()]);
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "BODY_PARTS"), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "PAIN_WORDS"), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_tuple(vec![norm.clone(), words.clone()]);
    }
    out = py_list(vec![]);
    read = py_list(vec![]);
    index = Py::Int(0);
    while py_truthy(&op_lt(&index.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_SIDE(), &geti(&norm.clone(), &index.clone()))); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&index.clone(), &Py::Int(2)), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(1))), &py_str("a"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&norm.clone(), &op_add(&index.clone(), &Py::Int(2))), &py_str("side"))) } } } } } }) {
    let _ = mcall(&out.clone(), "append", &[py_str("football")]);
    let _ = mcall(&read.clone(), "append", &[py_str("football")]);
    let __aug1 = op_add(&index.clone(), &Py::Int(3));
    index = __aug1;
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&norm.clone(), &index.clone()), &py_str("kickabout"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&norm.clone(), &index.clone()), &py_str("kick"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&norm.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(2)), &Py::None), &py_list(vec![py_str("about")]))) } } } }) {
    let _ = mcall(&out.clone(), "append", &[py_str("football")]);
    let _ = mcall(&read.clone(), "append", &[py_str("football")]);
    let __aug2 = op_add(&index.clone(), &if py_truthy(&Py::Bool(py_eq(&geti(&norm.clone(), &index.clone()), &py_str("kickabout")))) { Py::Int(1) } else { Py::Int(2) });
    index = __aug2;
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[geti(&norm.clone(), &index.clone())]);
    let __aug3 = op_add(&index.clone(), &Py::Int(1));
    index = __aug3;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&read.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_str("football"))) } }) {
    return py_tuple(vec![out.clone(), words.clone()]);
    }
    return py_tuple(vec![out.clone(), op_add(&call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_list(vec![py_str("football")]))]);
    Py::None
}
pub fn _parts_that_hurt(args: &[Py]) -> Py {
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
    let mut fixes: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fixes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fixes"))) };
    let mut at: Py = Py::None;
    let mut beside: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut other: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pain: Py = Py::None;
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line with every misspelt part of the body read as that part, repairs kept in `fixes`.\n\n    Only where the line puts the word where a part goes: right beside a word of pain (\"my\n    cheast hurts\", \"a sore stomack\", \"my shouldr has been aching\"), or straight after \"my\" or\n    \"our\" with a word of pain close behind. The place is what tells a part of the body from an\n    ordinary word a letter away from one, and the bot's word list does not hold every ordinary\n    word (\"hear\", \"wear\", \"tear\" are each a letter from \"ear\"; \"gear\" and \"chip\" are a letter\n    from \"ear\" and \"hip\"), so the word is never repaired on the letters alone. The words looked\n    for are the two the topic rule needs (`topics.BODY` and `topics.HURTS`), so the repair and\n    the rule cannot disagree.\n    ");
    pain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk2008 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk2008[0].clone(); let mut word: Py = __unpk2008[1].clone(); if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "PAIN_WORDS"), &word.clone()))) {  __out.push(at.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&pain.clone()))) {
    return words.clone();
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    part = _near_a_part(&[word.clone()]);
    beside = { let __b = Py::Bool(py_contains(&pain.clone(), &op_sub(&at.clone(), &Py::Int(1)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pain.clone(), &op_add(&at.clone(), &Py::Int(1)))) } };
    mine = { let __b = Py::Bool(py_contains(&py_tuple(vec![py_list(vec![py_str("my")]), py_list(vec![py_str("our")])]), &slice_or_raise(&words.clone(), &op_sub(&at.clone(), &Py::Int(1)), &at.clone(), &Py::None))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pain.clone()) { let mut other: Py = __item.clone(); __out.push(op_le(&call_builtin("abs", &[op_sub(&other.clone(), &at.clone())]).unwrap_or_else(|e| exc::raise(e)), &g_PART_REACH())); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = part.clone(); if !py_truthy(&__b) { __b } else { { let __b = beside.clone(); if py_truthy(&__b) { __b } else { mine.clone() } } } }) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), part.clone()])]);
    let _ = mcall(&out.clone(), "append", &[part.clone()]);
    } else {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _where_a_state_stands(args: &[Py]) -> Py {
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
    let mut before_before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before_before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut before_words: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_words") { v.clone() }
    else { Py::None };
    let mut back: Py = Py::None;
    let mut done_to: Py = Py::None;
    let mut end: Py = Py::None;
    let mut stands: Py = Py::None;
    let mut wide: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words around this one are where a feeling can be said.\n\n    The first person is the whole of the evidence (\"im knackerd\", \"im so exausted\", \"i feel\n    awsome\"), and for somebody else it is the be-form and what follows it: \"he is worrid about\n    money\" is a state, \"what is nappy rash\" is a rash.\n\n    Every measure word between the two is stepped back over, not one of them: \"im absolutely\n    exausted\", \"im literally exausted\", \"im so so exausted\" and \"im really very exausted\" are\n    the same state as \"im exausted\" (lilac-cherry, #5358). Walking back is what keeps the frame\n    off the word list, so the next temper word nobody has thought of yet (\"im utterly flabbered\")\n    still reads as a state.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&if py_truthy(&Py::Bool(!py_is(&before_words.clone(), &Py::None))) { before_words.clone() } else { py_tuple(vec![before_before.clone(), before.clone()]) }) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_is(&word.clone(), &Py::None))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINER_IN_FRONT(), &geti(&words.clone(), &op_neg(&Py::Int(1))))) } }) {
    done_to = { let __b = Py::Bool(py_contains(&g_POSSESSIVE_IN_FRONT(), &geti(&words.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CAUSES_A_STATE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&done_to.clone()))) {
    return Py::Bool(false);
    }
    }
    end = op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    back = end.clone();
    while py_truthy(&{ let __b = op_ge(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { _measures_a_state(&[geti(&words.clone(), &back.clone())]) } }) {
    let __aug1 = op_sub(&back.clone(), &Py::Int(1));
    back = __aug1;
    }
    if py_truthy(&op_lt(&back.clone(), &Py::Int(0))) {
    return _opens_a_state(&[after.clone()]);
    }
    stands = geti(&words.clone(), &back.clone());
    if py_truthy(&Py::Bool(py_contains(&g_BE_BEFORE_A_STATE(), &stands.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_THIRD_PERSON_BE(), &stands.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); __kws.extend(vec![("opening".to_string(), Py::Bool(!py_truthy(&{ let __b = op_ge(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PERSON_WORDS(), &geti(&words.clone(), &op_sub(&back.clone(), &Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OBJECT_PRONOUN(), &geti(&words.clone(), &op_sub(&back.clone(), &Py::Int(1))))) } } } })))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ends_a_state(args))(&__args) };
    }
    if py_truthy(&Py::Bool(py_contains(&g_CAUSES_A_STATE(), &stands.clone()))) {
    return { let __b = op_lt(&back.clone(), &end.clone()); if py_truthy(&__b) { __b } else { _ends_a_state(&[after.clone()]) } };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OBJECT_PRONOUN(), &stands.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CAUSES_A_STATE(), &geti(&words.clone(), &op_sub(&back.clone(), &Py::Int(1))))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&stands.clone(), &py_str("my"))); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CAUSES_A_STATE(), &geti(&words.clone(), &op_sub(&back.clone(), &Py::Int(1))))) } } } }) {
    return _ends_a_state(&[after.clone()]);
    }
    wide = back.clone();
    while py_truthy(&{ let __b = op_ge(&wide.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&words.clone(), &wide.clone())); __kws.extend(vec![("wide".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _measures_a_state(args))(&__args) } } }) {
    let __aug2 = op_sub(&wide.clone(), &Py::Int(1));
    wide = __aug2;
    }
    if py_truthy(&{ let __b = op_lt(&wide.clone(), &back.clone()); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&wide.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BE_BEFORE_A_STATE(), &geti(&words.clone(), &wide.clone()))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = op_lt(&wide.clone(), &back.clone()); if !py_truthy(&__b) { __b } else { op_lt(&wide.clone(), &Py::Int(0)) } }) {
    return _opens_a_state(&[after.clone()]);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _ends_a_state(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut opening: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opening") { v.clone() }
    else { Py::Bool(false) };
    let _ = py_str("True when what follows a state is where a state can end: nothing, a function word, a time.\n\n    `opening` is for the word that opens a line: there is nothing in front of it to say a state is\n    what the line is about, so a word that a thing can be followed by (\"nappy after nappy\") is not\n    taken as the end of one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_is(&after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AFTER_A_STATE(), &after.clone())) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&opening.clone()) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_contains(&g_TIMES_A_STATE(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_AFTER_A_TIME(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_STATE_THEN_A_TIME(), &after.clone())) } } } };
    Py::None
}
pub fn _opens_a_state(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("True when what follows a word that opens a line is a state rather than a thing.\n\n    A state that opens a line is the whole of what they said (\"exausted\", \"exausted from work\",\n    \"worrid about money\", \"nackered all the time\"), so it is repaired; a word that opens a line\n    and is followed by a thing is part of that thing, so it is not (\"nappy cream\", \"nappy rash\",\n    \"nappy change\", \"nappy bag\", \"nappy bin\"). Only the word right after it decides, and every\n    function word - a preposition, a joiner, a time word, a determiner - says the line is about a\n    state, so the list of things does not have to be written down.\n\n    A word that says when is not one of those, because a thing is followed by one as easily as a\n    state is: \"nappy yesterday\" is a nappy, and the line would have to say a state some other way\n    (\"i was nackered yesterday\") before the word is read as one.\n    ");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); __kws.extend(vec![("opening".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ends_a_state(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &after.clone())) } };
    Py::None
}
pub fn _a_measure_spelt_wrong(args: &[Py]) -> Py {
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
    let mut measure: Py = Py::None;
    let _ = py_str("True when this is a measure word spelt wrong (\"completey exausted\", \"absolutley terified\").\n\n    The measure words are ordinary words, so a slip in one of them is repaired anywhere; this is\n    only about the frame, which cannot see that the word in front of a state is still a measure\n    word with a letter missing. Only words long enough to be no other word with one letter\n    different, so \"realy\" and \"completly\" count and \"to\" and \"so\" do not.\n\n    The closed list is not the whole story, because the \"-ly\" is what says a word measures a state\n    (`_measures_a_state`) and the words in the list above are the ones people say, not the ones\n    they type. A slip that breaks the ending is then in neither: `absolutley`, `extremley`,\n    `genuinly` and `realy` are none of them one letter from a word of the list, and none of them\n    ends in \"ly\" any more, so `im absolutley terified` lost the feeling the spelled line keeps.\n\n    The ending itself is what gets typed wrong, and it is typed wrong in one of two ways: the \"l\"\n    and the \"y\" of \"-ly\" in the wrong order (\"absolutley\", \"extremley\") or a doubled \"l\" typed\n    once (\"totaly\", \"realy\"). Both leave a word ending in \"y\" with a single \"l\" among its last\n    three letters, which no other long word has - \"safety\", \"memory\", \"quality\", \"mouldy\",\n    \"floppy\", \"turkey\" all end in a letter that is not the \"l\" of the ending, and the short words\n    that do (\"barley\", \"parsley\", \"trolley\") are shorter than an adverb of that shape. The adverb\n    itself is often not in the vocabulary at all (\"absolutely\", \"extremely\" are made by an ending\n    rule rather than written down, and their stem is not either), so it is the shape of the ending\n    that is read, and no list of adverbs is asked for and none is kept.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "isalpha", &[]) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AMOUNT_BEFORE_A_STATE()) { let mut measure: Py = __item.clone(); __out.push(_one_letter_apart(&[word.clone(), measure.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("aly"), py_str("lye"), py_str("yl")])])) {
    return Py::Bool(true);
    }
    return { let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ley")]) } };
    Py::None
}
pub fn _measures_a_state(args: &[Py]) -> Py {
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
    let mut wide: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wide") { v.clone() }
    else { Py::Bool(false) };
    let _ = py_str("True when this word only says how much or how, not what.\n\n    The measure words are a closed list of the ones people type (\"so\", \"very\", \"not\", \"dead\",\n    \"proper\"), and any word made into an adverb with \"-ly\": \"im absolutely exausted\", \"im\n    literally exausted\", \"im utterly exausted\" are the same state as \"im exausted\", and a list\n    of intensifiers is a list somebody has to keep adding to (lilac-cherry, #5358). The \"-ly\"\n    is what says the word is about the state rather than a thing, so \"im genuinely exausted\"\n    reads without anybody naming \"genuinely\" here.\n    ");
    if py_truthy(&{ let __b = wide.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PHRASE_MEASURES(), &word.clone())) } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&g_AMOUNT_BEFORE_A_STATE(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BARE_TO(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = { let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ly")]) } }; if py_truthy(&__b) { __b } else { _a_measure_spelt_wrong(&[word.clone()]) } } } } } };
    Py::None
}
pub fn _a_letter_typed_twice(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let mut shorter: Py = Py::None;
    let _ = py_str("The feeling a word is when one of its letters was typed twice, or None.\n\n    \"im saad\", \"im maad\", \"im loow\", \"im shyy\", \"im lonelyy\": the letter that came out twice is\n    one letter too many, and the word without it is a feeling. The one-edit reader in\n    `_one_letter_apart` refuses these on purpose - a letter typed into a word only counts when it\n    is a vowel, and a four letter word needs its feeling to be longer than it is, because a word\n    that short is as often a name - and the doubled letter is not always a vowel (\"lonelyy\").\n    Nothing is lost by reading the one slip on its own terms: the word being read is not in the\n    lexicon, and the word without the doubled letter is a feeling and not a name.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone())) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    place = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&geti(&word.clone(), &place.clone()), &geti(&word.clone(), &op_add(&place.clone(), &Py::Int(1)))))) {
    continue;
    }
    shorter = op_add(&slice_or_raise(&word.clone(), &Py::None, &place.clone(), &Py::None), &slice_or_raise(&word.clone(), &op_add(&place.clone(), &Py::Int(1)), &Py::None, &Py::None));
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS"), &shorter.clone()))) {
    return shorter.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _an_introduction(args: &[Py]) -> Py {
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
    let mut before_before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before_before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut before_words: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_words") { v.clone() }
    else { Py::None };
    let mut after_words: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_words") { v.clone() }
    else { Py::None };
    let mut word: Py = Py::None;
    let _ = py_str("True when this word is the name a line introduces rather than a state it states.\n\n    The five shapes are the ones a person gives their name in, and none of them is a shape a\n    feeling comes in: a greeting in front of it, \"this is X\", an \"X here\" or \"X from <place>\"\n    tail, an age after it, or a word at the front of the line that the verb after it has as its\n    subject (\"saad is my brother\"): a state is what a line says about somebody, not what it\n    says about a word.\n    ");
    let __iter1 = py_iter_or_raise(&{ let __b = before_words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_INTRODUCING_A_PERSON(), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_INTRODUCING_AFTER(), &after.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&before.clone(), &py_str("is"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("this"), py_str("that")]), &before_before.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&after.clone(), &py_str("is"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("|")]), &before.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("and"), py_str("|")]), &after.clone())); if py_truthy(&__b) { __b } else { mcall(&{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "isdigit", &[]) } }) {
    let __iter2 = py_iter_or_raise(&slice_or_raise(&call_builtin("list", &[{ let __b = after_words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(3), &Py::None));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&mcall(&word.clone(), "isdigit", &[])) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_letter_wrong_in_a_state(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut state: Py = Py::None;
    let _ = py_str("The one state word `word` is a letter away from, or None.\n\n    This is a whole-letter test, not the vowel-only one `_a_slip_of_a_state` makes, and it is\n    only asked where the line itself says the word is how the person is: a letter typed wrong\n    is the ordinary slip of a state word (\"hungy\", \"thisty\", \"colde\"), and the state list is\n    the closed one, so nothing else in the language is a letter from one of them. The word has\n    to be the last word of the clause as well, because \"been staring at a screen all day\" is a\n    letter from \"starving\" and is not about hunger at all.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone())) } } } }) {
    return Py::None;
    }
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STATES_OF_A_PERSON()) { let mut state: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&slice_or_raise(&state.clone(), &Py::None, &Py::Int(1), &Py::None), &slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None))); if !py_truthy(&__b) { __b } else { _a_letter_apart(&[word.clone(), state.clone()]) } }) {  __out.push(state.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return mcall(&found.clone(), "pop", &[]);
    }
    return Py::None;
    Py::None
}
pub fn _a_letter_apart(args: &[Py]) -> Py {
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
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let mut i: Py = Py::None;
    let mut long: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("True when one letter added, left out, doubled or swapped round makes the other word.\n\n    This is the feeling rule's own test (`_one_letter_apart`) with the consonant let in: people\n    do type \"hungy\" for hungry and \"thisty\" for thirsty, a consonant left out in the middle,\n    and \"dizzzy\" with one letter doubled. A letter on the front or the end of a whole word is\n    still not a slip, which is what keeps \"im busy\" out of it.\n    ");
    if py_truthy(&_one_letter_apart(&[word.clone(), other.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("abs", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![word.clone(), other.clone()]) } else { py_tuple(vec![other.clone(), word.clone()]) }, 2, None);
    short = __unpk1[0].clone();
    long = __unpk1[1].clone();
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(1), call_builtin("len", &[long.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut i: Py = __item.clone(); __out.push(Py::Bool(py_eq(&short.clone(), &op_add(&slice_or_raise(&long.clone(), &Py::None, &i.clone(), &Py::None), &slice_or_raise(&long.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _near_a_feeling(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { Py::None };
    let mut before_before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { Py::None };
    let mut after: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::None };
    let mut alone: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "alone") { v.clone() }
    else { Py::Bool(false) };
    let mut before_words: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_words") { v.clone() }
    else { Py::None };
    let mut after_words: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_words") { v.clone() }
    else { Py::None };
    let mut feeling: Py = Py::None;
    let mut found: Py = Py::None;
    let mut twice: Py = Py::None;
    let _ = py_str("The feeling a misspelt feeling word names, or None.\n\n    A feeling word is a closed list, so one letter out is still that feeling: \"knackerd\" is\n    \"knackered\", \"shatterd\" is \"shattered\", \"worrid\" is \"worried\", \"exausted\" is\n    \"exhausted\". This is the class a general spell-fixer has to refuse, because next to an\n    ordinary word the shortest edit is another ordinary word; next to a feeling word there is\n    nothing to confuse it with. A word that could be two feelings is left alone.\n\n    Only where a state can stand: after a be-form, with at most one measure word between (\"im so\n    exausted\", \"she was knackerd\", \"i feel awsome\"). Every other feeling repair in this file is\n    framed like that and this one was not, so a real word the lexicon does not hold became the\n    feeling anywhere in the line: \"how do i change a nappy\" was read as \"snappy\" (four deck\n    lines, and the question reached the pool for a short temper), \"my nappy bag is heavy\" as\n    \"snappy bag\". The frame is the second guard, after the lexicon.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&alone.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_where_a_state_stands(&[before.clone(), before_before.clone(), after.clone(), before_words.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "NOT_A_FEELING"), &word.clone())) } } } }) {
    return Py::None;
    }
    twice = _a_letter_typed_twice(&[word.clone()]);
    if py_truthy(&{ let __b = twice.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_an_introduction(&[before.clone(), before_before.clone(), after.clone(), before_words.clone(), after_words.clone()]))) } }) {
    return twice.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)))) {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&after.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&after.clone(), &py_str("|"))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone()))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("cious")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS"), &op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(5)), &Py::None), &py_str("xious")))) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(5)), &Py::None), &py_str("xious"));
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ctious")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS"), &op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(6)), &Py::None), &py_str("xious")))) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(6)), &Py::None), &py_str("xious"));
    }
    found = call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS")) { let mut feeling: Py = __item.clone(); if py_truthy(&{ let __b = _one_letter_apart(&[word.clone(), feeling.clone()]); if !py_truthy(&__b) { __b } else { { let __b = { let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[feeling.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_consonant_left_out_of_an_ing_word(&[word.clone(), feeling.clone()]))) } } } }) {  __out.push(feeling.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return mcall(&found.clone(), "pop", &[]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("|")]), &after.clone())); if !py_truthy(&__b) { __b } else { _the_state_is_the_whole_frame(&[before.clone(), before_before.clone(), before_words.clone()]) } }) {
    return the_feeling_two_letters_are_gone_from(&[word.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _a_consonant_left_out_of_an_ing_word(args: &[Py]) -> Py {
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
    let mut feeling: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "feeling") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: feeling"))) };
    let mut i: Py = Py::None;
    let _ = py_str("True when the only difference is a consonant missing from a word ending in \"-ing\".\n\n    A vowel is what people leave out of an \"-ing\" word when the keys are under the fingers\n    (\"starvng\", \"annoyng\", \"depresing\"), so a vowel is the whole of the slip the frame and the\n    closed feeling list were built to catch. A consonant left out of an \"-ing\" word is an\n    ordinary verb underneath instead, and the feeling on the list is another word: \"staring\" is\n    \"stare\" with \"-ing\" on it and only the \"v\" of \"starving\" makes it a state, so \"been staring\n    at a screen all day\" was read as \"been starving at a screen all day\". The frame cannot see\n    the difference - \"ive been exausted\" and \"ive been staring\" stand in the same place - so the\n    letter is what decides.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ing")]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("len", &[feeling.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) } }) {
    return Py::Bool(false);
    }
    i = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&i.clone(), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &i.clone()), &geti(&feeling.clone(), &i.clone()))) } }) {
    let __aug1 = op_add(&i.clone(), &Py::Int(1));
    i = __aug1;
    }
    return { let __b = Py::Bool(py_eq(&slice_or_raise(&word.clone(), &i.clone(), &Py::None, &Py::None), &slice_or_raise(&feeling.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_VOWEL_SET(), &geti(&feeling.clone(), &i.clone()))) } };
    Py::None
}
pub fn _the_state_is_the_whole_frame(args: &[Py]) -> Py {
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
    let mut before_before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before_before"))) };
    let mut before_words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_words") { v.clone() }
    else { Py::None };
    let mut back: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when nothing but the person or the be-form stands in front of the state.\n\n    Two letters out is a long enough slip for two different words to be that close, and a state\n    said of a thing is where the two meet: \"my phone screen is cracked\" is a screen, \"cracked\"\n    being \"crackered\" by two letters. So this repair only reaches a line whose subject is the\n    person - \"im stessd\", \"im so stessd\", \"she was stessd\" - and stands down the moment a thing is\n    named in front of the be-form (\"my phone screen is cracked\", \"the table leg is cracked\").\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&if py_truthy(&Py::Bool(!py_is(&before_words.clone(), &Py::None))) { before_words.clone() } else { py_tuple(vec![before_before.clone(), before.clone()]) }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    back = op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    while py_truthy(&{ let __b = op_ge(&back.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { _measures_a_state(&[geti(&words.clone(), &back.clone())]) } }) {
    let __aug1 = op_sub(&back.clone(), &Py::Int(1));
    back = __aug1;
    }
    return Py::Bool(py_eq(&back.clone(), &Py::Int(0)));
    Py::None
}
pub fn the_feeling_two_letters_are_gone_from(args: &[Py]) -> Py {
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
    let mut feeling: Py = Py::None;
    let mut found: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("The feeling a long word has lost two letters of, or None: \"stessd\" is \"stressed\".\n\n    Its own function because the name store has to ask the same question: \"im stessd\" was read\n    as a state and answered \"Hi Stessd\", and the name was kept for the rest of the chat\n    (\"whats my name\" then said Stessd). A word this long has room for two letters, and the slip\n    is always the same shape - letters left out, never swapped - so the test is that the typed\n    word is the feeling with one or two letters taken out, first letter kept. Two candidates and\n    a common name are refused: \"im sarah\" and \"im katie\" are people, not a state.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    low = mcall(&mcall(&word.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &low.clone())) } }) {
    return Py::None;
    }
    found = call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS")) { let mut feeling: Py = __item.clone(); if py_truthy(&_two_letters_out(&[low.clone(), feeling.clone()])) {  __out.push(feeling.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&found.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _two_letters_out(args: &[Py]) -> Py {
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
    let mut feeling: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "feeling") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: feeling"))) };
    let mut letter: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("`word` is `feeling` with one or two letters taken out of it, and nothing else changed.");
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![Py::Int(1), Py::Int(2)]), &op_sub(&call_builtin("len", &[feeling.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), &slice_or_raise(&feeling.clone(), &Py::None, &Py::Int(1), &Py::None))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_one_letter_apart(&[word.clone(), feeling.clone()])) {
    return Py::Bool(false);
    }
    rest = call_builtin("iter", &[feeling.clone()]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&word.clone()) { let mut letter: Py = __item.clone(); __out.push(Py::Bool(py_contains(&rest.clone(), &letter.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _feeling_without_its_ending(args: &[Py]) -> Py {
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
    let mut known: Py = Py::None;
    let _ = py_str("\"confuse\" is \"confused\": a feeling typed with its ending left off, or None.\n\n    \"im confuse\", \"im annoy\", \"im bore\" and \"im excit\" are how people type when the ending of\n    the word they mean does not come out, and they are words already (\"confuse\" is a verb), so\n    the misspelling fixer may not touch them - but next to \"im\" they are the feeling. The same\n    slip costs the last letter of \"nervous\", \"anxious\" and \"miserable\" (\"im nervou\", \"im anxio\",\n    \"im miserabl\"), and it can cost the whole of a longer ending - a whole \"-ing\", \"-l\" or\n    \"-ty\": \"im gratefu\", \"im guil\", \"im awfu\", \"im anxie\", \"im dread\", \"im mourn\" and\n    \"im emotiona\" were each answered \"Nice to meet you, Gratefu\". Only the closed feeling list\n    is consulted, and only the endings a feeling word takes, so a name is still a name\n    (\"im matt\", \"im justin\", \"im anna\").\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))) } }) {
    return Py::None;
    }
    known = op_bitor(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS"), &pyrt::xrefs::xget("understanding.lexicon", "A_STATE"));
    if py_truthy(&Py::Bool(py_contains(&known.clone(), &word.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ed"), py_str("d"), py_str("y"), py_str("s"), py_str("us"), py_str("e"), py_str("g"), py_str("ing"), py_str("l"), py_str("ty")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&known.clone(), &op_add(&word.clone(), &ending.clone())))) {
    return op_add(&word.clone(), &ending.clone());
    }
    }
    return Py::None;
    Py::None
}
pub fn reads_as_a_feeling(args: &[Py]) -> Py {
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
    let _ = py_str("True when this word is a feeling word, or a slip of one the engine repairs.\n\n    The reading already lowers a misspelt feeling to that feeling, so a module that takes a\n    bare word for a name has to ask the same question, or a person typing one word about\n    themselves is welcomed by name: \"im knackerd\" was answered \"Nice to meet you, Knackerd\".\n    A name that the one-edit fixer would bend is still a name (\"im matt\", \"im justin\"), and so\n    is a word that only looks like a state.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = mcall(&mcall(&mcall(&word.clone(), "strip", &[]), "strip", &[py_str(".,!?;:'\"()")]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&low.clone(), "isalpha", &[]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = _feeling_without_its_ending(&[low.clone()]); if py_truthy(&__b) { __b } else { _a_letter_wrong_in_a_state(&[low.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "SOUNDS_LIKE_A_FEELING"), &low.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); __args.push(py_list(vec![])); __kws.extend(vec![("alone".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _token(args))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS"), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "A_STATE"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STATES_OF_A_PERSON(), &word.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&op_bitor(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS"), &pyrt::xrefs::xget("understanding.lexicon", "A_STATE")), &op_add(&low.clone(), &py_str("d"))))) {
    return Py::Bool(true);
    }
    if py_truthy(&_a_vowel_left_out_of_a_state(&[low.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_vowel_left_out_of_a_state(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut state: Py = Py::None;
    let _ = py_str("The state a short word had a vowel left out of, or None: \"spnt\" is \"spent\", \"bet\" is \"beat\".\n\n    Only a vowel that was never typed, never a letter changed: a short word with one letter\n    changed is usually another word and often somebody's name (\"sid\" is not \"sad\"), and no name\n    people are given is a state with a vowel missing - the one that looked like it, \"don\" for\n    \"done\", is on the common names list and is left alone. A word the lexicon knows keeps its\n    own meaning (\"bat\" is not \"beat\").\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __cmp11: Py = call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(3), &__cmp11)) { op_lt(&__cmp11, &Py::Int(5)) } else { Py::Bool(false) } })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone())) } }) {
    return Py::None;
    }
    found = call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_bitor(&pyrt::xrefs::xget("understanding.lexicon", "FEELINGS"), &pyrt::xrefs::xget("understanding.lexicon", "A_STATE"))) { let mut state: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[state.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_one_vowel_less(&[state.clone()]), &word.clone())) } }) {  __out.push(state.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&found.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _one_vowel_less(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut letter: Py = Py::None;
    let _ = py_str("Every way of leaving one vowel out of the word, as a set.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk5026 = py_unpack(&__item.clone(), 2, None); let mut at: Py = __unpk5026[0].clone(); let mut letter: Py = __unpk5026[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VOWEL_SET(), &letter.clone())); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = at.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp11)) { op_lt(&__cmp11, &op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } else { Py::Bool(false) } } } }) {  __out.push(op_add(&slice_or_raise(&word.clone(), &Py::None, &at.clone(), &Py::None), &slice_or_raise(&word.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None))); } } py_set(__out) })() };
    Py::None
}
pub fn reads_as_a_sound(args: &[Py]) -> Py {
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
    let mut kinds: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when the word is the sound of a feeling (\"board\" is bored), not its spelling.\n\n    A module that files a bare word as a name asks this as well, because \"im board\" is how\n    somebody says they are bored, not what they are called. It is a separate question from\n    `reads_as_a_feeling`, which is about the word as it is spelt, and it is the caller with the\n    whole line in front of it that decides: \"my name is board\" names a person.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = mcall(&mcall(&mcall(&word.clone(), "strip", &[]), "strip", &[py_str(".,!?;:'\"()")]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&low.clone(), "isalpha", &[]))) } }) {
    return Py::Bool(false);
    }
    kinds = pyrt::xrefs::xmodule("understanding.kinds");
    return Py::Bool(!py_is(&mcall(&kinds.clone(), "feeling_by_sound", &[low.clone()]), &Py::None));
    Py::None
}
pub fn repaired_tokens(args: &[Py]) -> Py {
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
    let _ = py_str("What the engine reads this one token as, alone: \"gd\" is \"good\", \"y\" is \"why\".\n\n    A module holding its own list of words that cannot be a name asks this rather than the\n    private table, so \"im gd\" is caught by the same lists that hold \"im good\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    low = mcall(&mcall(&mcall(&word.clone(), "strip", &[]), "strip", &[py_str(".,!?;:'\"()")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return py_list(vec![]);
    }
    return call_builtin("list", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); __args.push(py_list(vec![])); __kws.extend(vec![("alone".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _token(args))(&__args) }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn spelled(args: &[Py]) -> Py {
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
    let mut table: Py = Py::None;
    let _ = py_str("What the engine's own tables read this one token as, or [] when only a guess would.\n\n    ``repaired_tokens`` also carries the fixer's guesses, and those need the \"says more than\n    the word did\" guard: a cut is not evidence (\"ann\" collapses to \"an\", and \"an\" is not a\n    reason to doubt somebody's name). A curated entry is evidence on its own - the table\n    says what the word stands for - so \"hungrey\" is \"hungry\" even though that is one letter\n    shorter, and a module that holds its own list of words asks this rather than re-reading\n    the tables.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    low = mcall(&mcall(&mcall(&word.clone(), "strip", &[]), "strip", &[py_str(".,!?;:'\"()")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return py_list(vec![]);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![pyrt::xrefs::xget("understanding.lexicon", "CHAT"), pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), pyrt::xrefs::xget("understanding.lexicon", "US_SPELLINGS"), pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), pyrt::xrefs::xget("understanding.lexicon", "URGENT_MISSPELLINGS")]));
    for __item in __iter1 {
    table = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&table.clone(), &low.clone()))) {
    return mcall(&geti(&table.clone(), &low.clone()), "split", &[]);
    }
    }
    return py_list(vec![]);
    Py::None
}
pub fn _unsays_the_word(args: &[Py]) -> Py {
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
    let mut variant: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "variant") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: variant"))) };
    let mut doubled: Py = Py::None;
    let mut end: Py = Py::None;
    let mut ending: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("True when the only change is an un-doubled consonant in front of -ing, -ed, -er, -est.\n\n    Both spellings are English words and they say different things: \"hopping\" is not \"hoping\",\n    \"taping\" is not \"tapping\", \"planed\" is not \"planned\". Which one the person meant is nowhere\n    in the letters, so the doubled letter in front of an ending is left as it was typed rather\n    than guessed at - \"island hopping in greece\" was read as \"island hoping\".\n    ");
    if py_truthy(&op_ge(&call_builtin("len", &[variant.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(false);
    }
    ending = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("est"), py_str("ed"), py_str("er")])) { let mut end: Py = __item.clone(); if py_truthy(&mcall(&variant.clone(), "endswith", &[end.clone()])) {  __out.push(end.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&ending.clone()))) {
    return Py::Bool(false);
    }
    stem = slice_or_raise(&variant.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    if py_truthy(&op_lt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    doubled = op_add(&stem.clone(), &geti(&stem.clone(), &op_neg(&Py::Int(1))));
    return Py::Bool(py_eq(&word.clone(), &op_add(&doubled.clone(), &ending.clone())));
    Py::None
}
pub fn _a_word_of_the_bots(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this is a word to repair *towards*: the lexicon, or a word the bot's own base writes.\n\n    The repair vocabulary used to be the lexicon alone, so a slip in a word only the bot's cards\n    use was left as typed. \"rouitne\" and \"chloesterol\" are one swapped pair from \"routine\" and\n    \"cholesterol\" - both of them things the bot has a card about, and neither of them an everyday\n    English word - so nothing was repaired, and the line was read as a word about nothing: the card\n    missed, or the slip was said back to the person (\"Same for the drippign\"). The words the bot\n    writes in its own keys and answers are spelled by construction (that is what\n    `_words_the_bot_writes` is for), so a slip one letter from one of them is a slip of that word.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_NO_SWAP_TO(), &word.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_contains(&g_BASE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_words_the_bot_writes(&[]), &word.clone())) } };
    Py::None
}
pub fn fix_typo(args: &[Py]) -> Py {
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
    let mut _a_silent_t_put_back: Py = Py::None;
    let mut a_thing: Py = Py::None;
    let mut best: Py = Py::None;
    let mut candidates: Py = Py::None;
    let mut doubled_once: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut flat_ed: Py = Py::None;
    let mut left_out: Py = Py::None;
    let mut said_as_n: Py = Py::None;
    let mut short: Py = Py::None;
    let mut shunned: Py = Py::None;
    let mut stopped: Py = Py::None;
    let mut typed_is_a_word: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A high-confidence correction for a word that is not in the lexicon, or None.\n\n    Only typo classes that do not collide with ordinary words are used: a doubled or\n    un-doubled letter (\"recomend\", \"untill\"), an ie/ei swap (\"recieve\"), a left-off final\n    g (\"mornin\") and a dropped w (\"kno\"). A confident wrong correction is worse than none, so one-letter\n    substitutions and transpositions are left to the curated MISSPELLINGS map: with a\n    lexicon this size, \"three\" would otherwise become \"there\". A test keeps that honest: a\n    real word is never bent into the word the bot answers with (\"stairs\" is not \"stars\",\n    \"swelling\" is not \"spelling\").\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_BASE(), &word.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "TYPED_WORDS"), &word.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&_words_the_bot_writes(&[]), &word.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "NOT_A_FEELING"), &word.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &word.clone()))) {
    return Py::None;
    }
    shunned = _shun_variant(&[word.clone()]);
    if py_truthy(&shunned.clone()) {
    return shunned.clone();
    }
    stopped = _stopped_th(&[word.clone()]);
    if py_truthy(&stopped.clone()) {
    return stopped.clone();
    }
    short = _short_double(&[word.clone()]);
    if py_truthy(&short.clone()) {
    return short.clone();
    }
    doubled_once = _doubled_letter_words(&[word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))); if !py_truthy(&__b) { __b } else { doubled_once.clone() } }) {
    best = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(doubled_once.clone()); __kws.extend(vec![("key".to_string(), { let _a_silent_t_put_back = _a_silent_t_put_back.clone(); let a_thing = a_thing.clone(); let best = best.clone(); let candidates = candidates.clone(); let doubled_once = doubled_once.clone(); let dropped = dropped.clone(); let flat_ed = flat_ed.clone(); let left_out = left_out.clone(); let said_as_n = said_as_n.clone(); let short = short.clone(); let shunned = shunned.clone(); let stopped = stopped.clone(); let typed_is_a_word = typed_is_a_word.clone(); let w = w.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let _a_silent_t_put_back = _a_silent_t_put_back.clone(); let a_thing = a_thing.clone(); let best = best.clone(); let candidates = candidates.clone(); let doubled_once = doubled_once.clone(); let dropped = dropped.clone(); let flat_ed = flat_ed.clone(); let left_out = left_out.clone(); let said_as_n = said_as_n.clone(); let short = short.clone(); let shunned = shunned.clone(); let stopped = stopped.clone(); let typed_is_a_word = typed_is_a_word.clone(); let w = w.clone(); let word = word.clone(); let words = words.clone(); let w: Py = __kwp(args, 0, "w").unwrap_or(Py::None); mcall(&g_COMMON_RANK(), "get", &[w.clone(), op_pow(&Py::Int(10), &Py::Int(6))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    if py_truthy(&op_lt(&mcall(&g_COMMON_RANK(), "get", &[best.clone(), op_pow(&Py::Int(10), &Py::Int(6))]), &Py::Int(2000))) {
    return best.clone();
    }
    }
    if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    return Py::None;
    }
    a_thing = _a_thing_a_letter_short(&[word.clone()]);
    if py_truthy(&a_thing.clone()) {
    return a_thing.clone();
    }
    candidates = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_double_variants(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = _a_word_of_the_bots(&[w.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_unsays_the_word(&[word.clone(), w.clone()]))) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    let __aug1 = op_add(&candidates.clone(), &doubled_once.clone());
    candidates = __aug1;
    let __aug2 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_ie_variants(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&_a_word_of_the_bots(&[w.clone()])) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug2;
    dropped = _dropped_g(&[word.clone()]);
    if py_truthy(&dropped.clone()) {
    let _ = mcall(&candidates.clone(), "append", &[dropped.clone()]);
    }
    said_as_n = _an_ing_said_as_n(&[word.clone()]);
    if py_truthy(&said_as_n.clone()) {
    let _ = mcall(&candidates.clone(), "append", &[said_as_n.clone()]);
    }
    let __aug3 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_dropped_letter(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&_a_word_of_the_bots(&[w.clone()])) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug3;
    let __aug4 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_a_leading_h(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&_a_word_of_the_bots(&[w.clone()])) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug4;
    let __aug5 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_extra_vowel(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&_a_word_of_the_bots(&[w.clone()])) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug5;
    let __aug6 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_missing_vowel(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&_a_word_of_the_bots(&[w.clone()])) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug6;
    let __aug7 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_a_said_for_an_ending(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BASE(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&w.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &w.clone())) } } } }) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug7;
    let __aug8 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_swapped_pair(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = _a_word_of_the_bots(&[w.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_URGENT_WORDS(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug8;
    let __aug9 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_two_letters_the_wrong_way_round(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&_a_word_of_the_bots(&[w.clone()])) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug9;
    flat_ed = _missing_e(&[word.clone()]);
    if py_truthy(&flat_ed.clone()) {
    let _ = mcall(&candidates.clone(), "append", &[flat_ed.clone()]);
    }
    let __aug10 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_e_kept_before_ing(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug10;
    left_out = _an_emergency_word_left_out(&[word.clone()]);
    if py_truthy(&left_out.clone()) {
    let _ = mcall(&candidates.clone(), "append", &[left_out.clone()]);
    }
pub fn __nested__a_silent_t_put_back_10(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut _a_silent_t_put_back: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut a_thing: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut best: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut candidates: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut doubled_once: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut dropped: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut flat_ed: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut left_out: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut said_as_n: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut short: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut shunned: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut stopped: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut typed_is_a_word: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut w: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut typed: Py = __pos.get(16).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "typed").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut ending: Py = Py::None;
    let mut __yielded: Vec<Py> = Vec::new();
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("en"), py_str("le")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = mcall(&typed.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    if py_truthy(&Py::Bool(py_contains(&g_VOWELS(), &geti(&typed.clone(), &op_sub(&op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(1)))))) {
    continue;
    }
    __yielded.push(op_add(&op_add(&slice_or_raise(&typed.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), &py_str("t")), &ending.clone()));
    }
    }
    py_list(__yielded)
}
    _a_silent_t_put_back = {{ let _a_silent_t_put_back = _a_silent_t_put_back.clone(); let a_thing = a_thing.clone(); let best = best.clone(); let candidates = candidates.clone(); let doubled_once = doubled_once.clone(); let dropped = dropped.clone(); let flat_ed = flat_ed.clone(); let left_out = left_out.clone(); let said_as_n = said_as_n.clone(); let short = short.clone(); let shunned = shunned.clone(); let stopped = stopped.clone(); let typed_is_a_word = typed_is_a_word.clone(); let w = w.clone(); let word = word.clone(); let words = words.clone(); py_func("_a_silent_t_put_back", move |args: &[Py]| { let mut full = vec![_a_silent_t_put_back.clone(), a_thing.clone(), best.clone(), candidates.clone(), doubled_once.clone(), dropped.clone(), flat_ed.clone(), left_out.clone(), said_as_n.clone(), short.clone(), shunned.clone(), stopped.clone(), typed_is_a_word.clone(), w.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested__a_silent_t_put_back_10(&full) }) }};
    let __aug12 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&_a_silent_t_put_back, &[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BASE(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug12;
    typed_is_a_word = Py::Bool(py_contains(&g_BASE(), &word.clone()));
    let __aug13 = op_add(&candidates.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_double_variants(&[word.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BASE(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = typed_is_a_word.clone(); if !py_truthy(&__b) { __b } else { _unsays_the_word(&[word.clone(), w.clone()]) } })) } }) {  __out.push(w.clone()); } } py_list(__out) })() });
    candidates = __aug13;
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(call_builtin("set", &[candidates.clone()]).unwrap_or_else(|e| exc::raise(e))); __kws.extend(vec![("key".to_string(), { let _a_silent_t_put_back = _a_silent_t_put_back.clone(); let a_thing = a_thing.clone(); let best = best.clone(); let candidates = candidates.clone(); let doubled_once = doubled_once.clone(); let dropped = dropped.clone(); let flat_ed = flat_ed.clone(); let left_out = left_out.clone(); let said_as_n = said_as_n.clone(); let short = short.clone(); let shunned = shunned.clone(); let stopped = stopped.clone(); let typed_is_a_word = typed_is_a_word.clone(); let w = w.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let _a_silent_t_put_back = _a_silent_t_put_back.clone(); let a_thing = a_thing.clone(); let best = best.clone(); let candidates = candidates.clone(); let doubled_once = doubled_once.clone(); let dropped = dropped.clone(); let flat_ed = flat_ed.clone(); let left_out = left_out.clone(); let said_as_n = said_as_n.clone(); let short = short.clone(); let shunned = shunned.clone(); let stopped = stopped.clone(); let typed_is_a_word = typed_is_a_word.clone(); let w = w.clone(); let word = word.clone(); let words = words.clone(); let w: Py = __kwp(args, 0, "w").unwrap_or(Py::None); mcall(&g_COMMON_RANK(), "get", &[w.clone(), op_pow(&Py::Int(10), &Py::Int(6))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&words.clone(), &Py::Int(0));
    }
    if py_truthy(&{ let __b = op_lt(&mcall(&g_COMMON_RANK(), "get", &[geti(&words.clone(), &Py::Int(0)), op_pow(&Py::Int(10), &Py::Int(6))]), &Py::Int(120)); if !py_truthy(&__b) { __b } else { op_gt(&mcall(&g_COMMON_RANK(), "get", &[geti(&words.clone(), &Py::Int(1)), op_pow(&Py::Int(10), &Py::Int(6))]), &Py::Int(900)) } }) {
    return geti(&words.clone(), &Py::Int(0));
    }
    return Py::None;
    Py::None
}
pub fn _slip(args: &[Py]) -> Py {
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
    let mut opener: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opener") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opener"))) };
    let mut i: Py = Py::None;
    let mut long: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("How `word` is one letter away from `opener`: \"missing\", \"extra\" or None.\n\n    A letter swapped for a different one is not counted: in a word this short a swap says\n    nothing (\"who\" and \"why\" are both one swap from \"whn\"), while a letter added or left out\n    is the shape people type at speed.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &opener.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), &slice_or_raise(&opener.clone(), &Py::None, &Py::Int(1), &Py::None))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&call_builtin("abs", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[opener.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&if py_truthy(&op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[opener.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![word.clone(), opener.clone()]) } else { py_tuple(vec![opener.clone(), word.clone()]) }, 2, None);
    short = __unpk1[0].clone();
    long = __unpk1[1].clone();
    i = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&i.clone(), &call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&short.clone(), &i.clone()), &geti(&long.clone(), &i.clone()))) } }) {
    let __aug2 = op_add(&i.clone(), &Py::Int(1));
    i = __aug2;
    }
    if py_truthy(&Py::Bool(!py_eq(&slice_or_raise(&short.clone(), &i.clone(), &Py::None, &Py::None), &slice_or_raise(&long.clone(), &op_add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None)))) {
    return Py::None;
    }
    return if py_truthy(&Py::Bool(py_is(&short.clone(), &word.clone()))) { py_str("missing") } else { py_str("extra") };
    Py::None
}
pub fn fix_opener(args: &[Py]) -> Py {
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
    let mut doubled: Py = Py::None;
    let mut f: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut short: Py = Py::None;
    let mut test: Py = Py::None;
    let _ = py_str("The opener word `word` is one letter away from, or None when none or two fit.\n\n    Openers of two letters are only reached by un-doubling a letter (\"hii\" -> \"hi\"), because\n    one letter means too little in a word that short. `normalise` only asks about a word\n    nothing else knows, and only where a clause begins.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    doubled = { let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &op_neg(&Py::Int(1))), &geti(&word.clone(), &op_neg(&Py::Int(2))))) } };
    short = Py::Bool(py_eq(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)));
    fits = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g_OPENERS());
    for __item in __iter1 {
    opener = __item.clone();
    shape = _slip(&[word.clone(), opener.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&shape.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = short.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_WH_OPENERS(), &opener.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&shape.clone(), &py_str("extra"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&doubled.clone())) } }) {
    continue;
    }
    let _ = mcall(&fits.clone(), "append", &[py_tuple(vec![opener.clone(), shape.clone()])]);
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![{ let doubled = doubled.clone(); let f = f.clone(); let fits = fits.clone(); let kept = kept.clone(); let opener = opener.clone(); let shape = shape.clone(); let short = short.clone(); let test = test.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let doubled = doubled.clone(); let f = f.clone(); let fits = fits.clone(); let kept = kept.clone(); let opener = opener.clone(); let shape = shape.clone(); let short = short.clone(); let test = test.clone(); let word = word.clone(); let opener: Py = __kwp(args, 0, "opener").unwrap_or(Py::None); Py::Bool(py_eq(&geti(&opener.clone(), &op_neg(&Py::Int(1))), &geti(&word.clone(), &op_neg(&Py::Int(1))))) }) }, { let doubled = doubled.clone(); let f = f.clone(); let fits = fits.clone(); let kept = kept.clone(); let opener = opener.clone(); let shape = shape.clone(); let short = short.clone(); let test = test.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let doubled = doubled.clone(); let f = f.clone(); let fits = fits.clone(); let kept = kept.clone(); let opener = opener.clone(); let shape = shape.clone(); let short = short.clone(); let test = test.clone(); let word = word.clone(); let opener: Py = __kwp(args, 0, "opener").unwrap_or(Py::None); { let __b = doubled.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&opener.clone(), &op_neg(&Py::Int(1))), &geti(&word.clone(), &op_neg(&Py::Int(2))))) } } }) }]));
    for __item in __iter2 {
    test = __item.clone();
    if py_truthy(&op_gt(&call_builtin("len", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fits.clone()) { let mut f: Py = __item.clone(); if py_truthy(&py_call(&test, &[geti(&f.clone(), &Py::Int(0))])) {  __out.push(f.clone()); } } py_list(__out) })() };
    fits = if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { kept.clone() } else { fits.clone() };
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    break;
    }
    }
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&geti(&fits.clone(), &Py::Int(0)), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _plain_word(args: &[Py]) -> Py {
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
    let _ = py_str("True when this is a real word the engine knows, not something it only guessed at.\n\n    The lexicon also holds the ordinary endings of real words (\"worries\", \"weres\"), which is\n    what a typo must still be allowed to reach: \"weres the best place to park\" is \"where is\",\n    and only a word that is real on its own is left alone.\n    ");
    return { let __b = Py::Bool(py_contains(&g_BASE(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), &word.clone())) } } } } } };
    Py::None
}
pub fn _repair_openers(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clauses: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clauses"))) };
    let mut fixes: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fixes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fixes"))) };
    let mut clause: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Fix a typo in the word a clause opens with, in place.\n\n    The opening word decides how the whole line is read: \"whts the weather like tmrw\" is a\n    question, \"thnks\" a thanks, \"hii\" a hello, and one misspelt letter otherwise leaves the\n    line looking like a statement. The repair stays very narrow: only a word no other rule\n    knows, only where a clause begins (after a discourse word too: \"so wy is my boiler...\"),\n    and only when exactly one opener is a single slip away.\n    ");
    let __iter1 = py_iter_or_raise(&clauses.clone());
    for __item in __iter1 {
    clause = __item.clone();
    let __iter2 = py_iter_or_raise(&call_builtin("sorted", &[py_set(vec![Py::Int(0), op_sub(&call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[strip_leaders(&[clause.clone()])]).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    if py_truthy(&op_ge(&start.clone(), &call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    continue;
    }
    word = geti(&clause.clone(), &start.clone());
    if py_truthy(&_plain_word(&[word.clone()])) {
    continue;
    }
    fixed = fix_opener(&[word.clone()]);
    if py_truthy(&fixed.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), fixed.clone()])]);
    let _ = py_setitem(&clause.clone(), &Py::Int(0), _token(&[fixed.clone(), fixes.clone()]));
    break;
    }
    }
    }
    return clauses.clone();
    Py::None
}
pub fn _four_spells_for(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let _ = py_str("A leading 4 inside a word is \"for\": \"4get\", \"4got\", \"4getting\", \"4give\", \"4ever\".\n\n    People write the first syllable of a word with the digit, and the word then means\n    nothing to any reader: \"i keep 4getting things\" came back as an echo of the line where\n    \"i keep forgetting things\" answers. Only a 4 in front of a word the engine already knows\n    is spelled out, so \"i have 4 cats\" and \"a table for 4\" are untouched, and \"4tunate\"\n    stays as typed because \"tunate\" is not a word. One 4 per word is all people write\n    (\"4give\", never \"4give4\"), so the rest must be letters. The words this happens to put\n    the \"for\" in front of a g or a vowel (\"forget\", \"forgot\", \"forgive\", \"forever\"), which is\n    what keeps \"4night\" and \"4ward\" from being spelled into a word that does not exist.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "startswith", &[py_str("4")]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None), "isalpha", &[]))) } } } }) {
    return Py::None;
    }
    rest = slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_contains(&py_str("aeioug"), &geti(&rest.clone(), &Py::Int(0))))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BASE(), &rest.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &rest.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), &rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &rest.clone())) } } } } } }) {
    return op_add(&py_str("for"), &rest.clone());
    }
    return Py::None;
    Py::None
}
pub fn _collapse(args: &[Py]) -> Py {
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
    let mut choices: Py = Py::None;
    let mut length: Py = Py::None;
    let mut lengths: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut runs: Py = Py::None;
    let mut variant: Py = Py::None;
    let _ = py_str("'sooo' -> 'so', 'pleeease' -> 'please'; a variant that is a real word, else None.\n\n    A word stretched in two places still lands: settling every run at once would take\n    \"heellooo\" to \"helo\" and throw it away, so the runs are tried in every combination.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("(.)\\1\\1"), word.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    runs = call_builtin("list", &[stdlib_call("re", "finditer", &[py_str("(.)\\1+"), word.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&call_builtin("len", &[runs.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::None;
    }
    choices = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&runs.clone()) { let mut match_: Py = __item.clone(); __out.push(call_builtin("range", &[Py::Int(1), op_add(&call_builtin("min", &[Py::Int(3), call_builtin("len", &[mcall(&match_.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&choices.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("itertools", "product", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    lengths = __item.clone();
    variant = word.clone();
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[call_builtin("zip", &[runs.clone(), lengths.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    match_ = __unpk3[0].clone();
    length = __unpk3[1].clone();
    variant = op_add(&op_add(&slice_or_raise(&variant.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), &op_mul(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &length.clone())), &slice_or_raise(&variant.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&variant.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &variant.clone())) } }) {
    return variant.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _apostrophe(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut base: Py = Py::None;
    let mut read: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("Expand or settle one token that has an apostrophe.");
    if py_truthy(&Py::Bool(!py_contains(&word.clone(), &py_str("'")))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&mcall(&word.clone(), "partition", &[py_str("'")]), 3, None);
    base = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    rest = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&rest.clone(), &py_str("t"))); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("n't")]) } }) {
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None);
    if py_truthy(&Py::Bool(py_eq(&stem.clone(), &py_str("ca")))) {
    return py_list(vec![py_str("can"), py_str("not")]);
    }
    if py_truthy(&Py::Bool(py_eq(&stem.clone(), &py_str("wo")))) {
    return py_list(vec![py_str("will"), py_str("not")]);
    }
    if py_truthy(&Py::Bool(py_eq(&stem.clone(), &py_str("sha")))) {
    return py_list(vec![py_str("shall"), py_str("not")]);
    }
    if py_truthy(&Py::Bool(py_eq(&stem.clone(), &py_str("ai")))) {
    return py_list(vec![py_str("is"), py_str("not")]);
    }
    return if py_truthy(&stem.clone()) { py_list(vec![stem.clone(), py_str("not")]) } else { Py::None };
    }
    if py_truthy(&Py::Bool(py_eq(&rest.clone(), &py_str("m")))) {
    return py_list(vec![base.clone(), py_str("am")]);
    }
    if py_truthy(&Py::Bool(py_eq(&rest.clone(), &py_str("re")))) {
    return py_list(vec![base.clone(), py_str("are")]);
    }
    if py_truthy(&Py::Bool(py_eq(&rest.clone(), &py_str("ve")))) {
    return py_list(vec![base.clone(), py_str("have")]);
    }
    if py_truthy(&Py::Bool(py_eq(&rest.clone(), &py_str("ll")))) {
    return py_list(vec![base.clone(), py_str("will")]);
    }
    if py_truthy(&Py::Bool(py_eq(&rest.clone(), &py_str("d")))) {
    return if py_truthy(&Py::Bool(py_contains(&g_PRONOUN_BASES(), &base.clone()))) { py_list(vec![base.clone(), py_str("would")]) } else { py_list(vec![base.clone()]) };
    }
    if py_truthy(&Py::Bool(py_eq(&rest.clone(), &py_str("s")))) {
    if py_truthy(&Py::Bool(py_contains(&g_PRONOUN_BASES(), &base.clone()))) {
    return py_list(vec![base.clone(), py_str("is")]);
    }
    read = { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[base.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), "get", &[base.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "URGENT_MISSPELLINGS"), "get", &[base.clone()]); if py_truthy(&__b) { __b } else { py_str("") } } } } } };
    if py_truthy(&{ let __b = read.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&read.clone(), &py_str(" "))) } }) {
    return py_list(vec![read.clone(), py_str("is")]);
    }
    return py_list(vec![base.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _counts_a_compound(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut head: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: head"))) };
    let mut tail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tail"))) };
    let _ = py_str("Whether a number `head tail` counts the head rather than meaning \"for\" it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&tail.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&head.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&tail.clone(), &py_str("|"))) } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&head.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { mcall(&tail.clone(), "isdigit", &[]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_COUNTED_AFTER(), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_COUNTED_AFTER(), &tail.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_COMPOUND_HEAD(), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_COMPOUND_TAIL(), &tail.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&head.clone(), "endswith", &[py_str("ing")]); if py_truthy(&__b) { __b } else { mcall(&tail.clone(), "endswith", &[py_str("ing")]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _counts_a_thing(args: &[Py]) -> Py {
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
    let _ = py_str("Whether a number in front of this word is counting something.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("|"))) } }) {
    return Py::Bool(true);
    }
    word = geti(&{ let __b = mcall(&word.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("|")]) } }, &Py::Int(0));
    if py_truthy(&{ let __b = mcall(&word.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_COUNT_UNITS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_COUNTED_AFTER(), &word.clone())) } } } }) {
    return Py::Bool(true);
    }
    return { let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_tuple(vec![py_str("ss"), py_str("us"), py_str("is")])]))) } } } };
    Py::None
}
pub fn _digit_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut after_after: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_after") { v.clone() }
    else { Py::None };
    let mut head: Py = Py::None;
    let mut number: Py = Py::None;
    let mut spelled: Py = Py::None;
    let mut spoken_before: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The word \"2\" or \"4\" stands for where it sits, or None to keep it a number.\n\n    The next word says which reading fits: \"2 the shop\" and \"2 cook\" are \"to\", \"2 eggs\" is\n    \"two\", \"me 2\" is \"too\", \"4 lunch\" is \"for\" and \"4 eggs\" is \"four\". The number is what\n    people mean most of the time, so it is the reading the digit keeps unless the next word\n    asks for another one: \"i have 4 cats\", \"meet me at 4\", \"party of 4\", \"4 pm\" and\n    \"we are 4 today\" are all counts.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("2"), py_str("4")]), &token.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("yr"), py_str("yrs")]), &mcall(&{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])))) {
    return if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("2")))) { py_str("two") } else { py_str("four") };
    }
    before = { let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("|") } };
    spelled = { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[before.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), "get", &[before.clone()]); if py_truthy(&__b) { __b } else { before.clone() } } } };
    spoken_before = geti(&{ let __b = mcall(&spelled.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("|")]) } }, &op_neg(&Py::Int(1)));
    after = if py_truthy(&after.clone()) { mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[after.clone(), after.clone()]) } else { py_str("|") };
    after = geti(&{ let __b = mcall(&after.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("|")]) } }, &Py::Int(0));
    number = if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("2")))) { py_str("two") } else { py_str("four") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_OPERATORS(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OPERATORS(), &before.clone())) } }) {
    return number.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_COUNTS(), &after.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BEFORE_VERB(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TOO_AFTER(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&before.clone(), &py_str("|"))) } } } } } }) {
    tail = if py_truthy(&after_after.clone()) { mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[after_after.clone(), after_after.clone()]) } else { py_str("|") };
    tail = geti(&{ let __b = mcall(&tail.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("|")]) } }, &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_PERIOD_FOLLOWS(), &tail.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&tail.clone(), &py_str("|"))) } }) {
    return number.clone();
    }
    return py_str("for");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_LABEL_BEFORE(), &spoken_before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&after.clone(), "isdigit", &[]))) } }) {
    return number.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("2"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MOTION_BEFORE(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_PERIOD_FOLLOWS(), &after.clone())) } } } }) {
    return py_str("to");
    }
    if py_truthy(&Py::Bool(py_contains(&g_DEMONSTRATIVE(), &after.clone()))) {
    after_after = if py_truthy(&after_after.clone()) { mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[after_after.clone(), after_after.clone()]) } else { py_str("|") };
    after_after = geti(&{ let __b = mcall(&after_after.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("|")]) } }, &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&g_A_PERIOD_FOLLOWS(), &after_after.clone()))) {
    return number.clone();
    }
    }
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("2")))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TO_AFTER(), &before.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_UP_TO_TAGS(), &after.clone())) } } } }) {
    return py_str("to");
    }
    if py_truthy(&Py::Bool(py_eq(&after.clone(), &py_str("|")))) {
    return if py_truthy(&Py::Bool(py_contains(&g_TOO_AFTER(), &before.clone()))) { py_str("too") } else { number.clone() };
    }
    if py_truthy(&Py::Bool(py_contains(&g_TOO_A(), &after.clone()))) {
    return py_str("too");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BEFORE_WORD(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BEFORE_VERB(), &after.clone())) } }) {
    return py_str("to");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TOO_A_GRADED(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_NEGATION_BEFORE(), &spoken_before.clone())) } }) {
    return py_str("too");
    }
    return number.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_BE_VERBS(), &before.clone()))) {
    return number.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_FOR_A(), &after.clone()))) {
    return py_str("for");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FOR_OBJECT(), &after.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&before.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FOR_MEAL(), &after.clone())) } } } }) {
    return py_str("for");
    }
    if py_truthy(&{ let __b = mcall(&after.clone(), "isdigit", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&before.clone(), "isdigit", &[]))) } }) {
    return py_str("for");
    }
    if py_truthy(&Py::Bool(!py_is(&after_after.clone(), &Py::None))) {
    head = after.clone();
    tail = if py_truthy(&after_after.clone()) { mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[after_after.clone(), after_after.clone()]) } else { py_str("|") };
    tail = geti(&{ let __b = mcall(&tail.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("|")]) } }, &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_COMPOUND_HEADS(), &head.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COMPOUND_TAILS(), &tail.clone())) } }) {
    return number.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NUMBERS_BEFORE(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_counts_a_thing(&[after.clone()]))) } }) {
    return number.clone();
    }
    if py_truthy(&Py::Bool(!py_is(&after_after.clone(), &Py::None))) {
    tail = if py_truthy(&after_after.clone()) { mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[after_after.clone(), after_after.clone()]) } else { py_str("|") };
    tail = geti(&{ let __b = mcall(&tail.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("|")]) } }, &Py::Int(0));
    if py_truthy(&_counts_a_compound(&[after.clone(), tail.clone()])) {
    return number.clone();
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&_counts_a_thing(&[after.clone()])))) {
    return py_str("for");
    }
    return number.clone();
    Py::None
}
pub fn _once_only(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("Drop a word typed twice in a row when it is one a line never says twice.\n\n    \"i dunno\" expands to \"i i do not know\" because the chat spelling carries its own \"i\", and\n    \"the the\" and \"to to\" are the same kind of slip. An adjective or a word like \"very\" is kept,\n    because people mean it twice.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&tokens.clone());
    for __item in __iter1 {
    token = __item.clone();
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&token.clone(), &geti(&out.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_REPEAT_ONCE(), &token.clone())) } } } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[token.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn opens_without_subject(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut second: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut word_after: Py = Py::None;
    let _ = py_str("True when `words[index]` is an auxiliary and nothing that can be its subject follows.\n\n    \"am off to bed\" and \"was thinking of quitting\" have no subject; \"am i too late\" and \"was the\n    film good\" do, so they are questions and are left alone.\n    ");
    if py_truthy(&{ let __b = op_ge(&index.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&words.clone(), &index.clone()))) } }) {
    return Py::Bool(false);
    }
    word = geti(&words.clone(), &index.clone());
    if py_truthy(&Py::Bool(!py_contains(&g_AUX_OPENS(), &word.clone()))) {
    return Py::Bool(false);
    }
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&w.clone(), &py_str("|")))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(false);
    }
    second = geti(&rest.clone(), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&g__SUBJECTS(), &second.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("am")))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g__PREDICATE_ONLY(), &second.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g__SUBJECTLESS_AUX(), &word.clone()))) {
    clause = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter1 {
    word_after = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&word_after.clone(), &py_str("|")))) {
    break;
    }
    let _ = mcall(&clause.clone(), "append", &[word_after.clone()]);
    }
    at = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&at.clone(), &call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__STATE_FILLER(), &geti(&clause.clone(), &at.clone()))) } }) {
    let __aug2 = op_add(&at.clone(), &Py::Int(1));
    at = __aug2;
    }
    if py_truthy(&{ let __b = op_lt(&at.clone(), &call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__A_STATE_OF_THE_PERSON(), &geti(&clause.clone(), &at.clone()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__STATE_TAIL(), &geti(&clause.clone(), &op_add(&at.clone(), &Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__STATE_FILLER(), &geti(&clause.clone(), &op_add(&at.clone(), &Py::Int(1))))) } } } } } } } }) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__HAVE_NEGATIONS(), &second.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("have"))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("have"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__HAVE_PARTICIPLES(), &second.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return { let __b = Py::Bool(py_contains(&g__HAVE_THING(), &geti(&rest.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__HAVE_ASKING(), &geti(&rest.clone(), &Py::Int(2)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&rest.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TIME_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } };
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[second.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&second.clone(), "endswith", &[py_str("ing")]); if py_truthy(&__b) { __b } else { mcall(&second.clone(), "endswith", &[py_str("ed")]) } } } }) {
    if py_truthy(&op_lt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g__SUBJECTLESS_ONLY(), &word.clone()))) {
    return Py::Bool(py_contains(&g__AFTER_PARTICIPLE(), &geti(&rest.clone(), &Py::Int(1))));
    }
    after = if py_truthy(&op_gt(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { geti(&rest.clone(), &Py::Int(2)) } else { py_str("") };
    return { let __b = Py::Bool(py_contains(&g__AFTER_PARTICIPLE(), &geti(&rest.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { mcall(&after.clone(), "endswith", &[py_tuple(vec![py_str("ing"), py_str("ed")])]) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _only_leaders_before(args: &[Py]) -> Py {
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
    let mut i: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this clause so far is nothing but filler (\"so am off to bed\").\n\n    \"he banged his head and was knocked out\" has a subject in front of the \"and\", so nothing was\n    dropped there and the line must be left alone.\n    ");
    start = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[index.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    i = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&words.clone(), &i.clone()), &py_str("|")))) {
    start = op_add(&i.clone(), &Py::Int(1));
    }
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &start.clone(), &index.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LEADERS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _lookahead(args: &[Py]) -> Py {
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
    let mut n: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { Py::Int(3) };
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The next few tokens as the reading will have them (\"thinkin\" -> \"thinking\").\n\n    A line typed in a hurry leaves the subject off and misspells the word after it: \"am thinkin\n    of jackin in me job\". Whether the subject is missing depends on the word the reader will see,\n    not the one that was typed.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&op_add(&index.clone(), &Py::Int(1)), &n.clone()), &Py::None));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("|")))) {
    break;
    }
    let _ = mcall(&out.clone(), "append", &[if py_truthy(&_token(&[word.clone(), py_list(vec![])])) { geti(&_token(&[word.clone(), py_list(vec![])]), &Py::Int(0)) } else { word.clone() }]);
    }
    return out.clone();
    Py::None
}
pub fn subject_missing(args: &[Py]) -> Py {
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
    let _ = py_str("True when the subject after this auxiliary was dropped and it was the person typing.");
    return { let __b = Py::Bool(py_contains(&g_DROPPED_SUBJECT_AUX(), &geti(&words.clone(), &index.clone()))); if !py_truthy(&__b) { __b } else { opens_without_subject(&[words.clone(), index.clone()]) } };
    Py::None
}
pub fn _opener_is(args: &[Py]) -> Py {
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
    let mut spelled: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("'wots'/'whatz'/'whyz' -> ['what', 'is'] / ['why', 'is'], or None.\n\n    People run the \"s\" of \"is\" onto the question word and often spell the question word as\n    they say it: \"wots the best way\", \"whyz my boiler losing pressure\". Neither half is a\n    typo the other readers can find - \"wots\" is not one letter from an opener, and it is\n    not in the spelling tables - so the line reads as news and gets a \"noted\" back. The\n    stem is read the way the chat table reads it (\"wot\" -> \"what\"), and only a question\n    word is ever completed this way.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("sz"), &geti(&word.clone(), &op_neg(&Py::Int(1))))) } }) {
    return Py::None;
    }
    stem = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    if py_truthy(&Py::Bool(py_contains(&g_WH_OPENERS(), &stem.clone()))) {
    return py_list(vec![stem.clone(), py_str("is")]);
    }
    spelled = mcall(&{ let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[stem.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), "get", &[stem.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), "get", &[stem.clone()]); if py_truthy(&__b) { __b } else { py_str("") } } } } } }, "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[spelled.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WH_OPENERS(), &geti(&spelled.clone(), &Py::Int(0)))) } }) {
    return py_list(vec![geti(&spelled.clone(), &Py::Int(0)), py_str("is")]);
    }
    return Py::None;
    Py::None
}
pub fn _cause_in_place_is_the_verb(args: &[Py]) -> Py {
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
    let mut after_words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_words") { v.clone() }
    else { Py::None };
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a typed `cause` is the verb or the noun rather than the chat spelling of because.\n\n    A subject after it is \"because\" (\"cause i was late\", \"cause my bus was late\"), and so is the\n    end of a line with no article in front (\"i did it just cause\"). Anything else keeps the word.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_CAUSE_BEFORE_A_VERB(), &before.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_CAUSE_BEFORE_A_NOUN(), &before.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_CAUSE_A_DEMONSTRATIVE(), &after.clone()))) {
    rest = slice_or_raise(&call_builtin("list", &[{ let __b = after_words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1), &Py::Int(2), &Py::None);
    return { let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_CAUSE_A_SUBJECT_FOLLOWS(), &geti(&rest.clone(), &Py::Int(0)))) } };
    }
    if py_truthy(&Py::Bool(py_contains(&g_CAUSE_A_SUBJECT_FOLLOWS(), &after.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_is(&after.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&after.clone(), &py_str("the"))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = after_words.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &Py::Int(1), &Py::Int(4), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CAUSE_A_CLAUSE_VERB(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _token(args: &[Py]) -> Py {
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
    let mut fixes: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fixes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fixes"))) };
    let mut before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { Py::None };
    let mut before_before: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { Py::None };
    let mut after: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::None };
    let mut alone: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "alone") { v.clone() }
    else { Py::Bool(false) };
    let mut before_words: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_words") { v.clone() }
    else { Py::None };
    let mut after_words: Py = if __pos.len() > 7 { __pos[7].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_words") { v.clone() }
    else { Py::None };
    let mut ached: Py = Py::None;
    let mut collapsed: Py = Py::None;
    let mut corrected: Py = Py::None;
    let mut expanded: Py = Py::None;
    let mut feeling: Py = Py::None;
    let mut for_spelled: Py = Py::None;
    let mut fronted: Py = Py::None;
    let mut opened: Py = Py::None;
    let mut pluraled: Py = Py::None;
    let mut swapped_inside: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("Normalise one lower-case token to zero or more ordinary words.\n\n    `before` is the word it follows, for the few repairs that depend on where a word sits.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("cause"))); if !py_truthy(&__b) { __b } else { _cause_in_place_is_the_verb(&[before.clone(), after.clone(), after_words.clone()]) } }) {
    return py_list(vec![word.clone()]);
    }
    if py_truthy(&Py::Bool(!py_is(&before.clone(), &Py::None))) {
    thing = _a_thing_where_a_thing_goes(&[word.clone(), before.clone()]);
    if py_truthy(&thing.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), thing.clone()])]);
    return py_list(vec![thing.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_DIGIT_WORDS(), &word.clone()))) {
    return py_list(vec![geti(&g_DIGIT_WORDS(), &word.clone())]);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &word.clone()))) {
    return mcall(&geti(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &word.clone()), "split", &[]);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), &word.clone()))) {
    return mcall(&geti(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), &word.clone()), "split", &[]);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "US_SPELLINGS"), &word.clone()))) {
    return py_list(vec![geti(&pyrt::xrefs::xget("understanding.lexicon", "US_SPELLINGS"), &word.clone())]);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), &word.clone()))) {
    return mcall(&geti(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), &word.clone()), "split", &[]);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "URGENT_MISSPELLINGS"), &word.clone()))) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), geti(&pyrt::xrefs::xget("understanding.lexicon", "URGENT_MISSPELLINGS"), &word.clone())])]);
    return py_list(vec![geti(&pyrt::xrefs::xget("understanding.lexicon", "URGENT_MISSPELLINGS"), &word.clone())]);
    }
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str("'")))) {
    expanded = _apostrophe(&[word.clone()]);
    if py_truthy(&expanded.clone()) {
    return expanded.clone();
    }
    }
    for_spelled = _four_spells_for(&[word.clone()]);
    if py_truthy(&for_spelled.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), for_spelled.clone()])]);
    return py_list(vec![for_spelled.clone()]);
    }
    collapsed = _collapse(&[word.clone()]);
    if py_truthy(&collapsed.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), collapsed.clone()])]);
    return py_list(vec![collapsed.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return py_list(vec![word.clone()]);
    }
    opened = _opener_is(&[word.clone()]);
    if py_truthy(&opened.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), geti(&opened.clone(), &Py::Int(0))])]);
    return opened.clone();
    }
    pluraled = _z_for_s(&[word.clone()]);
    if py_truthy(&pluraled.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), pluraled.clone()])]);
    return py_list(vec![pluraled.clone()]);
    }
    ached = _ache_for_aike(&[word.clone()]);
    if py_truthy(&ached.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), ached.clone()])]);
    return py_list(vec![ached.clone()]);
    }
    corrected = fix_typo(&[word.clone()]);
    if py_truthy(&corrected.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), corrected.clone()])]);
    return py_list(vec![corrected.clone()]);
    }
    fronted = _said_with_an_f(&[word.clone()]);
    if py_truthy(&fronted.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), fronted.clone()])]);
    return py_list(vec![fronted.clone()]);
    }
    swapped_inside = _said_with_an_f_anywhere(&[word.clone()]);
    if py_truthy(&swapped_inside.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), swapped_inside.clone()])]);
    return py_list(vec![swapped_inside.clone()]);
    }
    feeling = _near_a_feeling(&[word.clone(), before.clone(), before_before.clone(), after.clone(), alone.clone(), before_words.clone(), after_words.clone()]);
    if py_truthy(&feeling.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), feeling.clone()])]);
    return py_list(vec![feeling.clone()]);
    }
    return py_list(vec![word.clone()]);
    Py::None
}
pub fn _one_word_food(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The foods written as two words set down as the one word the pantry and answers use.");
    let __iter1 = py_iter_or_raise(&g_ONE_WORD_FOODS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    one = __unpk2[1].clone();
    clean = mcall(&pattern.clone(), "sub", &[one.clone(), clean.clone()]);
    }
    return clean.clone();
    Py::None
}
pub fn _joined_words(args: &[Py]) -> Py {
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
    let _ = py_str("The words people write as one and say as two, set down as the two in `clean`.\n\n    Nothing here is a word, so there is nothing to lose by repairing it where it is written: the\n    fed-up reader matches on `clean`, so \"im fedup with my job\" used to be a shrug while \"im fed up\n    with my job\" got the fed-up reply.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_JOINED_WORDS(), "search", &[clean.clone()])))) {
    return clean.clone();
    }
    return mcall(&g_JOINED_WORDS(), "sub", &[{ let clean = clean.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); geti(&pyrt::xrefs::xget("understanding.lexicon", "JOINED_WORDS"), &mcall(&m.clone(), "group", &[Py::Int(0)])) }) }, clean.clone()]);
    Py::None
}
pub fn _z_for_s(args: &[Py]) -> Py {
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
    let mut fixed: Py = Py::None;
    let _ = py_str("A plural written with a z, where the s is a word the engine knows: \"ideaz\", \"dogz\".\n\n    People type the plural with the letter that sounds like it - \"ideaz 4 tea\", \"my kidz\",\n    \"thingz\" - and the line reaches nothing, so a question about dinner comes back as a\n    search hedge. Only a z that stands where an s would is spelled, and only when the word\n    that makes is one the engine or the knowledge base already knows, so \"jazz\", \"quiz\",\n    \"buzz\" and every real word ending in z are left alone (a word the bot writes stops the\n    lookup, and a typed z-word that is a real word is not spelled).\n\n    The knowledge base is read as well as the lexicon because a plural is not always one of\n    the lexicon's own words: \"ants\" is the word the pest rows answer, while the lexicon only\n    holds \"ant\", so \"antz\" was left alone and \"how do i get rid of antz\" got the search hedge\n    that its correctly spelt twin does not.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("z")]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LEXICON(), &word.clone())); if py_truthy(&__b) { __b } else { _a_word_the_bot_writes(&[word.clone()]) } }) {
    return Py::None;
    }
    fixed = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("s"));
    return if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LEXICON(), &fixed.clone())); if py_truthy(&__b) { __b } else { _a_word_the_bot_writes(&[fixed.clone()]) } }) { fixed.clone() } else { Py::None };
    Py::None
}
pub fn _ache_for_aike(args: &[Py]) -> Py {
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
    let mut fixed: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("\"headaike\" is \"headache\": -ache written the way it is said.\n\n    Only the tail is read, and only where the word the tail makes is one the engine knows, so\n    a name or a place that happens to end in -aike is never bent into a body part.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("aike"), py_str("ayke"), py_str("eake")]));
    for __item in __iter1 {
    tail = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[tail.clone()]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    fixed = op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), &py_str("ache"));
    if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &fixed.clone()))) {
    return fixed.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_contraction_typed_with_a_space(args: &[Py]) -> Py {
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
    let mut join_not: Py = Py::None;
    let _ = py_str("The line with a contraction's two halves put back together, where the halves are its own.\n\n    \"she s 84\" is how \"she's 84\" is typed when the apostrophe is left out and the space is kept,\n    and the letters after the pronoun are never a word: no line says \"she s\" or \"i m\". The\n    apostrophe goes back with them, because the joined letters are a word of their own\n    elsewhere - \"we re based in leeds\" is not \"were based in leeds\". The\n    \"not\" half is joined only when the word it makes is one the engine knows, so \"an t\" and a\n    real word in front of a stray \"t\" keep their letters.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_SPLIT_CONTRACTION(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SPLIT_NOT(), "search", &[clean.clone()]))) } }) {
    return clean.clone();
    }
    clean = mcall(&g_SPLIT_CONTRACTION(), "sub", &[{ let clean = clean.clone(); let join_not = join_not.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let join_not = join_not.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); op_add(&op_add(&mcall(&m.clone(), "group", &[Py::Int(1)]), &py_str("'")), &mcall(&m.clone(), "group", &[Py::Int(2)])) }) }, clean.clone()]);
pub fn __nested_join_not_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut clean: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut join_not: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    if py_truthy(&Py::Bool(!py_contains(&g_NOT_WORDS(), &op_add(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &py_str("t"))))) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    return op_add(&mcall(&match_.clone(), "group", &[Py::Int(1)]), &py_str("'t"));
    Py::None
}
    join_not = {{ let clean = clean.clone(); let join_not = join_not.clone(); py_func("join_not", move |args: &[Py]| { let mut full = vec![clean.clone(), join_not.clone()]; full.extend_from_slice(args); __nested_join_not_0(&full) }) }};
    return mcall(&g_SPLIT_NOT(), "sub", &[join_not.clone(), clean.clone()]);
    Py::None
}
pub fn _the_ask_said_plainer(args: &[Py]) -> Py {
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
    let _ = py_str("`what was it i said about the oven` -> `what did i say about the oven`.\n\n    The `saying` half keeps its tense and its word order, because \"what was i saying about the\n    car\" is the ask the store already answers and \"what i was saying about the car\" is not.\n    ");
    return mcall(&g_SAID_IT(), "sub", &[{ let text = text.clone(); py_func("lambda", move |args: &[Py]| { let text = text.clone(); let one: Py = __kwp(args, 0, "one").unwrap_or(Py::None); if py_truthy(&mcall(&one.clone(), "group", &[py_str("who")])) { { let mut __s = String::new(); __s.push_str("what did "); __s.push_str(&fstr(&mcall(&one.clone(), "group", &[py_str("who")]), -1, "")); __s.push_str(" say"); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str("what "); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&mcall(&one.clone(), "group", &[py_str("saying")]), &py_str("i")))) { py_str("was") } else { py_str("were") }, -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&one.clone(), "group", &[py_str("saying")]), -1, "")); __s.push_str(" saying"); py_string(__s) } } }) }, text.clone()]);
    Py::None
}
pub fn _me_for_my_pass(args: &[Py]) -> Py {
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
    let _ = py_str("`me ed in` and `me head in` -> `my head in`, the two shapes `possessive_me` misses.\n\n    The rest of the possessive \"me\" is that pass's job, with its own fences (a kin word after a\n    verb, the owning verbs, the prepositions), so nothing here overrides it: \"she told me mum\n    died\" and \"give me car keys back\" keep their \"me\" because this pass never sees them.\n    ");
    text = mcall(&g__A_HEAD_IN_THE_IDIOM(), "sub", &[py_str(" my head"), text.clone()]);
    return mcall(&g__A_CLIPPED_HEAD(), "sub", &[py_str(" my head"), text.clone()]);
    Py::None
}
pub fn _phrase_pass(args: &[Py]) -> Py {
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
    let mut feeling: Py = Py::None;
    let mut heart: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Replace whole phrases (\"cn u\", \"b/c\", \"okey dokey\") before words are split.");
    text = _me_for_my_pass(&[text.clone()]);
    let __iter1 = py_iter_or_raise(&g_HEART_SAID_AS_A_FEELING());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    heart = __unpk2[0].clone();
    feeling = __unpk2[1].clone();
    if py_truthy(&mcall(&heart.clone(), "search", &[text.clone()])) {
    text = mcall(&heart.clone(), "sub", &[feeling.clone(), text.clone()]);
    }
    }
    let __iter3 = py_iter_or_raise(&g_SAID_AS_A_FEELING());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk4[0].clone();
    feeling = __unpk4[1].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[text.clone()])) {
    text = mcall(&pattern.clone(), "sub", &[feeling.clone(), text.clone()]);
    }
    }
    let __iter5 = py_iter_or_raise(&g_PHRASES());
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk6[0].clone();
    value = __unpk6[1].clone();
    pattern = op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])"));
    if py_truthy(&stdlib_call("re", "search", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    text = stdlib_call("re", "sub", &[pattern.clone(), op_add(&op_add(&py_str(" "), &value.clone()), &py_str(" ")), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    text = mcall(&g_UP_TOO_ASKS(), "sub", &[py_str("\\1up to"), text.clone()]);
    text = _the_ask_said_plainer(&[text.clone()]);
    return text.clone();
    Py::None
}
pub fn _the_figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut figure: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "figure") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: figure"))) };
    let mut decimal: Py = Py::None;
    let mut thousands: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The number a typed amount stands for: \"1,500\" is 1500 and \"1,50\" is 1.5.");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&figure.clone(), &py_str(","))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&figure.clone(), &py_str("."))) } }) {
    decimal = if py_truthy(&op_gt(&mcall(&figure.clone(), "rfind", &[py_str(",")]), &mcall(&figure.clone(), "rfind", &[py_str(".")]))) { py_str(",") } else { py_str(".") };
    thousands = if py_truthy(&Py::Bool(py_eq(&decimal.clone(), &py_str(",")))) { py_str(".") } else { py_str(",") };
    figure = mcall(&mcall(&figure.clone(), "replace", &[thousands.clone(), py_str("")]), "replace", &[decimal.clone(), py_str(".")]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&figure.clone(), &py_str(",")))) {
    figure = if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[geti(&mcall(&figure.clone(), "split", &[py_str(",")]), &op_neg(&Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)))) { mcall(&figure.clone(), "replace", &[py_str(","), py_str("")]) } else { mcall(&figure.clone(), "replace", &[py_str(","), py_str(".")]) };
    }
    }
    value = call_builtin("float", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&Py::Bool(py_eq(&value.clone(), &call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))))) { op_modulo(&py_str("%d"), &value.clone()) } else { mcall(&op_modulo(&py_str("%.2f"), &value.clone()), "rstrip", &[py_str("0")]) };
    Py::None
}
pub fn _a_sign_is_a_word(args: &[Py]) -> Py {
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
    let mut spelled: Py = Py::None;
    let _ = py_str("A currency sign in front of a figure becomes the word for it: \"£20\" is \"20 quid\".");
pub fn __nested_spelled_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut spelled: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut figure: Py = Py::None;
    let mut scale: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut value: Py = Py::None;
    let mut written: Py = Py::None;
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)]), mcall(&{ let __b = mcall(&match_.clone(), "group", &[Py::Int(3)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]), 3, None);
    sign = __unpk1[0].clone();
    figure = __unpk1[1].clone();
    scale = __unpk1[2].clone();
    value = call_builtin("float", &[mcall(&_the_figure(&[figure.clone()]), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&scale.clone()) {
    value = op_mul(&value.clone(), &geti(&g_CURRENCY_FACTOR(), &scale.clone()));
    }
    written = if py_truthy(&Py::Bool(py_eq(&value.clone(), &call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))))) { op_modulo(&py_str("%d"), &value.clone()) } else { op_modulo(&py_str("%.2f"), &value.clone()) };
    return op_add(&op_add(&written.clone(), &py_str(" ")), &geti(&g_CURRENCY_SIGNS(), &sign.clone()));
    Py::None
}
    spelled = {{ let spelled = spelled.clone(); let text = text.clone(); py_func("spelled", move |args: &[Py]| { let mut full = vec![spelled.clone(), text.clone()]; full.extend_from_slice(args); __nested_spelled_0(&full) }) }};
    return mcall(&g_CURRENCY_SIGN(), "sub", &[spelled.clone(), text.clone()]);
    Py::None
}
pub fn _is_doing_something(args: &[Py]) -> Py {
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
    let _ = py_str("True when a word after a preposition is a verb, and so nothing \"me\" could own.");
    if py_truthy(&Py::Bool(py_contains(&g_DOING_NOT_THING(), &word.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&g_NOT_A_THING_ENDING());
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &geti(&g_LONG_ENOUGH_TO_BE_A_VERB(), &ending.clone())) } }) {
    return Py::Bool(!py_contains(&g_ING_THINGS(), &word.clone()));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn possessive_me(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut asks_whose: Py = Py::None;
    let mut before: Py = Py::None;
    let mut before_before: Py = Py::None;
    let mut how_or_when: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut owns_a_person: Py = Py::None;
    let mut owns_the_thing: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("'me' used for 'my' in front of a thing: \"me back is killing me\", \"me mum 4 xmas\".\n\n    British informal writing uses \"me\" where \"my\" belongs, and it is the difference between a\n    line the bot can act on and one it cannot. Only taken when what follows the thing is a\n    be-word, an auxiliary, a number or a word of saying, when \"me\" opens the phrase it is in\n    (nothing in front of it, a joiner, or a preposition: \"with me sister\", \"in me head\"), or when\n    a question asks whose it is (\"wheres me keys\"), so an object \"me\" (\"tell me more\", \"help me\n    plan\", \"gave me keys\") is never touched. A kin word is the one exception to the verb: \"he\n    rang me mum\" and \"she told me dad\" own their person.\n\n    Public because the echo says a line back with the same repair (`bot.memory`'s you()), so\n    \"i dont get on with me sister\" is never said back as \"you ... with you sister\".\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    token = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("me"))); if !py_truthy(&__b) { __b } else { op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    thing = geti(&tokens.clone(), &op_add(&index.clone(), &Py::Int(1)));
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(2)), &call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&tokens.clone(), &op_add(&index.clone(), &Py::Int(2))) } else { Py::None };
    before = if py_truthy(&index.clone()) { geti(&tokens.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { Py::None };
    before_before = if py_truthy(&op_ge(&index.clone(), &Py::Int(2))) { geti(&tokens.clone(), &op_sub(&index.clone(), &Py::Int(2))) } else { Py::None };
    asks_whose = { let __b = Py::Bool(py_contains(&g_ASKS_WHOSE(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BE_WORDS(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASKS_WHOSE(), &before_before.clone())) } } } };
    owns_a_person = { let __b = Py::Bool(py_contains(&g_KIN(), &thing.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_AFTER_A_THING(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BEFORE_A_THING(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TIME_WORDS(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_WORDS(), &after.clone())) } } } } } } } } } };
    owns_the_thing = { let __b = Py::Bool(py_contains(&g_AFTER_A_THING(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_OWNING_VERBS(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = owns_a_person.clone(); if py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "PAIN_WORDS"), &after.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&before.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BEFORE_A_THING(), &before.clone())) } } } }; if py_truthy(&__b) { __b } else { { let __b = { let __b = asks_whose.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_THING_AFTER(), &thing.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&thing.clone(), "endswith", &[py_tuple(vec![py_str("ly"), py_str("ing"), py_str("ed")])]))) } } } }; if py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_is(&before.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BEFORE_A_THING(), &before.clone())) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_THING_AFTER(), &thing.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&thing.clone(), "endswith", &[py_str("ly")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&g_BEFORE_A_PREPOSITION(), &before.clone())); if !py_truthy(&__b) { __b } else { _is_doing_something(&[thing.clone()]) } })) } } } } } } } } } } } } } } } };
    how_or_when = { let __b = Py::Bool(py_contains(&g_TIME_WORDS(), &thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACE_WORDS(), &thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TIME_WORDS(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING_PAIR(), &{ let mut __s = String::new(); __s.push_str(&fstr(&thing.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&after.clone(), -1, "")); py_string(__s) })) } } } } } };
    if py_truthy(&{ let __b = owns_the_thing.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&thing.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[thing.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_THING(), &thing.clone())); if !py_truthy(&__b) { __b } else { { let __b = owns_a_person.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&how_or_when.clone())) } } } } } } } } } }) {
    let _ = mcall(&out.clone(), "append", &[py_str("my")]);
    continue;
    }
    }
    let _ = mcall(&out.clone(), "append", &[token.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn a_son_not_the_sun(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("'\"my sun has got a temperature\" is about a child, so after a possessive \"sun\" is \"son\".\n\n    \"sun\" and \"son\" are one sound, and a parent typing about a poorly child writes the star:\n    \"my sun has got a tempreture of 39\" found no person at all and came back as a shrug, where\n    \"my son has ...\" answers it. The star belongs to nobody - it is \"the sun\" - so a possessive\n    in front of it is the shape that says the person was meant.\n\n    The test is on the word after it, twice over. The star's own words fence it off (\"my sun\n    cream\", \"our sun lounger\", \"her sun hat\", \"my sun burn\"), and the person needs a verb or a\n    copula there (\"my sun allergy\" is the star on a person's skin, \"my sun has a temperature\" is\n    the person). The verb half is what keeps a noun nobody listed safe.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    token = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("sun"), py_str("suns")]), &token.clone())); if !py_truthy(&__b) { __b } else { { let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNING_A_PERSON(), &geti(&tokens.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } } } }) {
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&tokens.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_SUN_WORDS(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SON_VERBS(), &after.clone())) } }) {
    let _ = mcall(&out.clone(), "append", &[if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("sun")))) { py_str("son") } else { py_str("sons") }]);
    continue;
    }
    }
    let _ = mcall(&out.clone(), "append", &[token.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _a_slip_that_is_a_word(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut after_body: Py = Py::None;
    let mut after_that: Py = Py::None;
    let mut before: Py = Py::None;
    let mut before_body: Py = Py::None;
    let mut feels: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pain_frame: Py = Py::None;
    let mut part: Py = Py::None;
    let mut place: Py = Py::None;
    let mut right: Py = Py::None;
    let mut slips: Py = Py::None;
    let mut token: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(tokens with the word meant in them, {as typed: the word meant}) for a line that has one.\n\n    Only the pairs above, and only where the line says which word was meant: this is the curated\n    route `fix_typo` leaves open for exactly this class, not a guesser.\n    ");
    lowered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tokens.clone()) { let mut token: Py = __item.clone(); __out.push(mcall(&token.clone(), "lower", &[])); } py_list(__out) })() };
    out = call_builtin("list", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e));
    slips = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[lowered.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    token = __unpk2[1].clone();
    right = Py::None;
    before = if py_truthy(&index.clone()) { geti(&lowered.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { Py::None };
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[lowered.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&lowered.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { Py::None };
    after_that = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(2)), &call_builtin("len", &[lowered.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&lowered.clone(), &op_add(&index.clone(), &Py::Int(2))) } else { Py::None };
    if py_truthy(&Py::Bool(py_contains(&g_A_PAIN_WORD(), &token.clone()))) {
    kept = { let __b = Py::Bool(py_contains(&g_DETERMINERS(), &before.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&index.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_KEEPING_A_THING(), &geti(&lowered.clone(), &op_sub(&index.clone(), &Py::Int(2))))) } } } };
    after_body = Py::Bool(false);
    if py_truthy(&Py::Bool(py_eq(&after.clone(), &py_str("in")))) {
    let __iter3 = py_iter_or_raise(&call_builtin("range", &[op_add(&index.clone(), &Py::Int(2)), call_builtin("min", &[call_builtin("len", &[lowered.clone()]).unwrap_or_else(|e| exc::raise(e)), op_add(&index.clone(), &Py::Int(5))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    place = __item.clone();
    part = geti(&lowered.clone(), &place.clone());
    if py_truthy(&Py::Bool(py_contains(&g_A_PART_THE_PAIN_IS_IN(), &part.clone()))) {
    after_body = Py::Bool(true);
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_PART_A_PERSON_OWNS(), &part.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNING_A_PERSON(), &geti(&lowered.clone(), &op_sub(&place.clone(), &Py::Int(1))))) } }) {
    after_body = Py::Bool(true);
    break;
    }
    }
    }
    before_body = { let __b = Py::Bool(py_contains(&g_A_PART_THE_PAIN_IS_IN(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_A_PART_A_PERSON_OWNS(), &before.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&index.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWNING_A_PERSON(), &geti(&lowered.clone(), &op_sub(&index.clone(), &Py::Int(2))))) } } } } } };
    feels = { let __b = Py::Bool(py_contains(&g_A_STATE_YOU_CAN_FEEL(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_A_COPULA(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("hurt"), py_str("hurts"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("kill"), py_str("killing"), py_str("throb"), py_str("throbbing")]), &after.clone())) } } } };
    pain_frame = { let __b = { let __b = before_body.clone(); if !py_truthy(&__b) { __b } else { feels.clone() } }; if py_truthy(&__b) { __b } else { after_body.clone() } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lowered.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_GLASS_OR_A_PAN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { pain_frame.clone() } } } }) {
    right = geti(&g_A_PAIN_WORD(), &token.clone());
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("hart"))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lowered.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_THE_HEART_SAID_THE_OTHER_WAY(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    right = py_str("heart");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("chocked"), py_str("chockes")]), &token.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_DETERMINERS(), &after.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lowered.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_CHOKE_SAID_THE_OTHER_WAY(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    right = if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("chocked")))) { py_str("choked") } else { py_str("chokes") };
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("ware"), py_str("wared")]), &token.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lowered.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_THING_WORN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    right = if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("ware")))) { py_str("wear") } else { py_str("wore") };
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("desert"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&before.clone(), &py_str("for"))) } }) {
    right = py_str("dessert");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("live"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BEFORE_A_JOB(), &after.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_A_JOB(), &after_that.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_bitor(&g_A_TIME_TO_LEAVE(), &py_set(vec![Py::None, py_str("|")])), &geti(&{ let __b = slice_or_raise(&lowered.clone(), &op_add(&index.clone(), &Py::Int(3)), &op_add(&index.clone(), &Py::Int(4)), &Py::None); if py_truthy(&__b) { __b } else { py_list(vec![Py::None]) } }, &Py::Int(0)))) } } } } } }) {
    right = py_str("leave");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("saw"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_A_COPULA(), &before.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lowered.clone()) { let mut part: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_PART_THE_PAIN_IS_IN(), &part.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    right = py_str("sore");
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("felling"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_STATE_YOU_CAN_FEEL(), &after.clone())) } }) {
    right = py_str("feeling");
    }
    }
    }
    }
    }
    }
    }
    }
    if py_truthy(&{ let __b = right.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&right.clone(), &token.clone())) } }) {
    let _ = py_setitem(&out.clone(), &index.clone(), right.clone());
    let _ = py_setitem(&slips.clone(), &geti(&tokens.clone(), &index.clone()), right.clone());
    }
    }
    return py_tuple(vec![out.clone(), slips.clone()]);
    Py::None
}
pub fn loose_is_lose(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut step: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("'\"i need to loose weight\" is \"lose weight\": nobody is loose while putting on trousers.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    token = __unpk2[1].clone();
    fixed = mcall(&g_LOOSE_FOR_LOSE(), "get", &[token.clone()]);
    if py_truthy(&fixed.clone()) {
    let __iter3 = py_iter_or_raise(&py_tuple(vec![Py::Int(1), Py::Int(2), Py::Int(3), Py::Int(4)]));
    let mut __else4 = false;
    for __item in __iter3 {
    step = __item.clone();
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &step.clone()), &call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&tokens.clone(), &op_add(&index.clone(), &step.clone())) } else { Py::None };
    if py_truthy(&Py::Bool(py_is(&after.clone(), &Py::None))) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g_THE_LOSING(), &after.clone()))) {
    let _ = mcall(&out.clone(), "append", &[fixed.clone()]);
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_BEFORE_THE_LOSING(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&after.clone(), "isdigit", &[]))) } }) {
    break;
    }
    }
    let _ = mcall(&out.clone(), "append", &[token.clone()]);
    continue;
    if py_truthy(&{ let __b = out.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&out.clone(), &op_neg(&Py::Int(1))), &fixed.clone())) } }) {
    continue;
    }
    }
    let _ = mcall(&out.clone(), "append", &[token.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _the_thing_has_been(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let _ = py_str("Whether the word after a \"bin\" is what a thing that has been somewhere says next.");
    plain = if py_truthy(&word.clone()) { _plain_chat(&[word.clone()]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&plain.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BEEN_WORDS(), &plain.clone())); if py_truthy(&__b) { __b } else { mcall(&plain.clone(), "endswith", &[py_tuple(vec![py_str("ing"), py_str("ed")])]) } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(!py_eq(&plain.clone(), &py_str("in"))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&plain.clone(), "endswith", &[py_str("in")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_IN_NOUNS(), &plain.clone())) } } } };
    Py::None
}
pub fn _is_person(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut spelled: Py = Py::None;
    let _ = py_str("True when this token names the person a \"bin\" belongs to: \"mums\", \"hes\", \"my dad\".");
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    bare = mcall(&mcall(&word.clone(), "replace", &[py_str("'"), py_str("")]), "replace", &[py_str("’"), py_str("")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERSON_WORDS(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERSON_WORDS(), &mcall(&bare.clone(), "rstrip", &[py_str("s")]))) } }) {
    return Py::Bool(true);
    }
    plain = _plain_chat(&[word.clone()]);
    if py_truthy(&Py::Bool(py_contains(&g_PERSON_WORDS(), &plain.clone()))) {
    return Py::Bool(true);
    }
    spelled = mcall(&{ let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "split", &[]);
    return { let __b = call_builtin("bool", &[spelled.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERSON_WORDS(), &geti(&spelled.clone(), &Py::Int(0)))) } };
    Py::None
}
pub fn _plain_chat(args: &[Py]) -> Py {
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
    let mut spelled: Py = Py::None;
    let _ = py_str("The first word a chat spelling stands for: \"shud\" -> \"should\", \"ive\" -> \"i\".");
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("");
    }
    spelled = { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { word.clone() } } } };
    return geti(&mcall(&spelled.clone(), "split", &[]), &Py::Int(0));
    Py::None
}
pub fn _spells_a_helper(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut spelled: Py = Py::None;
    let _ = py_str("True when the token in front is a helper or a \"to\" once its chat spelling is undone.\n\n    \"b\" is the quick way of writing \"be\" (\"gona b late\", \"ill b there\", \"will b fine\"), but it\n    is a letter people also name (\"plan b\", \"vitamin b\", \"b side\"), so it only becomes a verb\n    where a verb has to be: after a helper, a form of have or be, \"to\", or \"not\".\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("|"))) } }) {
    return Py::Bool(false);
    }
    spelled = mcall(&{ let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { word.clone() } } } }, "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&spelled.clone()))) {
    return Py::Bool(false);
    }
    last = geti(&spelled.clone(), &op_neg(&Py::Int(1)));
    return { let __b = Py::Bool(py_contains(&g_MODAL_FORMS(), &last.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BE_FORMS(), &last.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_HAVE_FORMS(), &last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("to"), py_str("not")]), &last.clone())) } } } } } };
    Py::None
}
pub fn _is_be(args: &[Py]) -> Py {
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
    let mut spelled: Py = Py::None;
    let _ = py_str("True when the token in front is a form of \"be\", chat spelling or contraction included.");
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_BE_FORMS(), &word.clone()))) {
    return Py::Bool(true);
    }
    spelled = { let __b = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { mcall(&pyrt::xrefs::xget("understanding.lexicon", "EXPAND"), "get", &[word.clone()]) } };
    return { let __b = call_builtin("bool", &[spelled.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BE_FORMS(), &geti(&mcall(&spelled.clone(), "split", &[]), &op_neg(&Py::Int(1))))) } };
    Py::None
}
pub fn _is_participle(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let _ = py_str("Whether this word is the verb a modal helper's \"of\" is standing in for \"have\" before.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("|")]), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("or"))) } } } }) {
    return Py::Bool(false);
    }
    plain = _plain_chat(&[word.clone()]);
    if py_truthy(&{ let __b = mcall(&plain.clone(), "endswith", &[py_str("ed")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_IRREGULAR_PARTICIPLES(), &plain.clone())) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_gerund_reads_as_a_verb(args: &[Py]) -> Py {
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
    let mut after_after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after_after"))) };
    let _ = py_str("True when \"-ing\" is what the person is doing rather than the first half of a noun.\n\n    \"my dads working late\" and \"my mums coming to stay\" are verb phrases; \"my mums living room is\n    tiny\", \"my dads sitting room needs painting\" and \"my mums waiting list is long\" are things\n    that belong to them, and an \"is\" in front reads as nonsense.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ing")]))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str(""), py_str("|")]), &after_after.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&g_CAN_FOLLOW_A_VERB(), &after_after.clone()));
    Py::None
}
pub fn _kin_needs_a_helper(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut opens_the_clause: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "opens_the_clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: opens_the_clause"))) };
    let mut after_after: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_after") { v.clone() }
    else { Py::None };
    let _ = py_str("The family word a dropped-apostrophe possessive stands for, with its helper.\n\n    \"my mums got a chest infection\" is \"my mum has got a chest infection\", and \"my dads on his\n    own\" is \"my dad is on his own\". Only where the possessive opens the clause or follows a\n    leader (\"and my dads on his own\"), and only in front of a word that needs a helper, so a noun\n    coming next keeps the possessive as it was typed (\"my mums car\").\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&opens_the_clause.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&before.clone(), &py_str("my"))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_KIN_SINGULAR(), &token.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_KIN_HAS(), &after.clone()))) {
    return py_list(vec![geti(&g_KIN_SINGULAR(), &token.clone()), py_str("has")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_KIN_IS(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("not"))); if py_truthy(&__b) { __b } else { mcall(&{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "isdigit", &[]) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_KIN_IS(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_gerund_reads_as_a_verb(&[after.clone(), after_after.clone()]))) } }) {
    return Py::None;
    }
    return py_list(vec![geti(&g_KIN_SINGULAR(), &token.clone()), py_str("is")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_KIN_FILLERS(), &after.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_KIN_IS(), &after_after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_KIN_HAS(), &after_after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&after_after.clone(), &py_str("not"))) } } } } } }) {
    return py_list(vec![geti(&g_KIN_SINGULAR(), &token.clone()), if py_truthy(&Py::Bool(py_contains(&g_KIN_HAS(), &after_after.clone()))) { py_str("has") } else { py_str("is") }]);
    }
    return Py::None;
    Py::None
}
pub fn _family_short(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut after_after: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after_after"))) };
    let mut things: Py = Py::None;
    let _ = py_str("The family word a clipped initial stands for, or None to keep the letters.\n\n    \"my oh\", \"her mil\" and \"my dh\" are the partner and the in-laws, and the expansion is what\n    lets every entry that keys on \"husband\" or \"daughter\" answer a line typed in a hurry: \"my\n    dd wont sleep\" was read as the person not sleeping and answered \"Annoying. Is it every\n    time\". Only after a possessive, and only for an initial that cannot be the thing of that\n    name in the line (\"my ds wont charge\" is a console, \"my dd is in june\" a due date).\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FAMILY_SHRT(), &token.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_possessive(&[before.clone()]))) } }) {
    return Py::None;
    }
    things = mcall(&g_SHRT_THINGS(), "get", &[token.clone()]);
    if py_truthy(&{ let __b = things.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&things.clone(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&things.clone(), &after_after.clone())) } } } }) {
    return Py::None;
    }
    return call_builtin("list", &[geti(&g_FAMILY_SHRT(), &token.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _counts_a_period(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this word is a length of time, so \"thats bin 3 wks\" is \"that has been 3 weeks\".");
    return Py::Bool(py_contains(&g_PERIOD_COUNTS(), &_plain_chat(&[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])));
    Py::None
}
pub fn _is_a_count(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let _ = py_str("Whether this word is how many, so the clipped unit after it is the unit.");
    plain = _plain_chat(&[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return { let __b = mcall(&plain.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_COUNT(), &plain.clone())) } };
    Py::None
}
pub fn _hr_is_the_department(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("True when the word after \"hr\" says it is the department and not an hour.");
    return Py::Bool(py_contains(&g_HR_IS_THE_DEPARTMENT_AFTER(), &_plain_chat(&[{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])));
    Py::None
}
pub fn _says_one(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let _ = py_str("Whether this word in front of a length of time makes it one: \"next yr\", \"a 3 hr flight\".");
    plain = _plain_chat(&[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]);
    return { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("1"), py_str("a"), py_str("an"), py_str("one")]), &plain.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_PARTS_OF_A_PERIOD(), &plain.clone())) } };
    Py::None
}
pub fn _is_a_thing_here(args: &[Py]) -> Py {
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
    let mut after_after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_after") { v.clone() }
    else { Py::None };
    let mut said_after: Py = Py::None;
    let _ = py_str("Whether the clipped word beside this one is the thing it stands for.\n\n    \"my rads are noisy\" and \"the rad is cold\" say something about it; \"bleed a rad\" ends with it\n    after a determiner. \"thats rad mate\", \"pretty rad\" and \"a rad new bike\" are how somebody\n    rates something, so the word after them is not one of the things said about a thing.\n    ");
    said_after = mcall(&mcall(&{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "strip", &[py_str("|")]);
    if py_truthy(&Py::Bool(py_contains(&g_THING_AFTER(), &said_after.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g_THING_JOIN(), &said_after.clone()))) {
    return Py::Bool(py_contains(&g_THING_SUBJECT(), &_plain_chat(&[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])));
    }
    if py_truthy(&Py::Bool(py_contains(&g_THING_PLACE_AFTER(), &said_after.clone()))) {
    if py_truthy(&Py::Bool(!py_contains(&g_THING_DETERMINER(), &_plain_chat(&[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])))) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_THING_PLACE_WORDS(), &mcall(&mcall(&{ let __b = after_after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "strip", &[py_str("|")])));
    }
    return { let __b = Py::Bool(!py_truthy(&said_after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THING_SUBJECT(), &_plain_chat(&[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]))) } };
    Py::None
}
pub fn _clipped_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut many: Py = Py::None;
    let mut one: Py = Py::None;
    let _ = py_str("The full word for a clipped thing, plural when the line says so.");
    let __unpk1 = py_unpack(&geti(&g_CLIPPED_THINGS(), &token.clone()), 2, None);
    one = __unpk1[0].clone();
    many = __unpk1[1].clone();
    return if py_truthy(&mcall(&token.clone(), "endswith", &[py_str("s")])) { many.clone() } else { one.clone() };
    Py::None
}
pub fn _is_before_a_noun(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("Whether the clipped unit in front of this word is qualifying a noun: \"a 3 hr flight\".");
    if py_truthy(&{ let __b = mcall(&token.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&after.clone(), &py_str("|"))) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&g_NOT_BEFORE_A_NOUN(), &_plain_chat(&[after.clone()])));
    Py::None
}
pub fn _is_said_of_a_person(args: &[Py]) -> Py {
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
    let mut plain: Py = Py::None;
    let _ = py_str("Whether this word is what somebody has been: a verb with its ending on, or a state.\n\n    \"my mums bin in hospital\" is a \"bin\" that is the \"been\" it sounds like, and so is \"my cats\n    bin sick\" and \"bin thinkin bout it\". A bare noun is not: \"the bin day\", \"a bin lorry\".\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("|")]), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PERSON_PREPS(), &word.clone())) } } } }) {
    return Py::Bool(false);
    }
    plain = _plain_chat(&[word.clone()]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[plain.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&plain.clone(), "endswith", &[py_str("ing")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&plain.clone(), "endswith", &[py_str("ed")]); if py_truthy(&__b) { __b } else { mcall(&plain.clone(), "endswith", &[py_str("in")]) } } } } } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&g_IRREGULAR_PARTICIPLES(), &plain.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BEEN_WORDS(), &plain.clone())) } };
    Py::None
}
pub fn _is_possessive(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let _ = py_str("True when the token in front owns what comes next: \"my\", \"her\", \"your mums\".");
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    bare = mcall(&mcall(&word.clone(), "replace", &[py_str("'"), py_str("")]), "replace", &[py_str("’"), py_str("")]);
    return Py::Bool(py_contains(&g_POSSESSIVES(), &bare.clone()));
    Py::None
}
pub fn _sounds_like_a_feeling(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut before_before: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { Py::None };
    let _ = py_str("The feeling this word sounds like, in the place a feeling goes, or None.\n\n    \"im board\" is \"i'm bored\" and \"im tried\" is \"i'm tired\". Both words are ordinary ones, so\n    nothing repairs them anywhere else; the position is what makes the reading safe.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_BEFORE_A_FEELING(), &before.clone()))) {
    before = before_before.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FIRST_PERSON_BE(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.lexicon", "SOUNDS_LIKE_A_FEELING"), &token.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_FEELING_CAN_BE_FOLLOWED(), &after.clone())) } }) {
    return Py::None;
    }
    return geti(&pyrt::xrefs::xget("understanding.lexicon", "SOUNDS_LIKE_A_FEELING"), &token.clone());
    Py::None
}
pub fn _were_for_another_word(args: &[Py]) -> Py {
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
    let mut after_after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_after") { v.clone() }
    else { Py::None };
    let _ = py_str("What \"were\" at the front of a line stands for, or None to keep it as it was typed.\n\n    The front of a line is where a question puts \"were\" too, so the word after it decides which\n    word was meant. In front of a finite verb it can only be \"where\" (\"were is my wallet\", \"were\n    do i put it\", \"were have you been\"), because that word is the verb and not a subject. In\n    front of a state or a doing it is \"we are\" with the apostrophe left off (\"were shattered\",\n    \"were thinking of getting a cat\"), and a lead-in word is looked through for the same thing\n    (\"were so tired\", \"were all gutted\", \"were not happy\"). Every other word keeps the \"were\" it\n    was typed with, because a question about a thing is asked the same way: \"were dogs allowed\",\n    \"were parking available\", \"were the kids ok\", \"were you at the party\".\n\n    \"Were\" further into the line is the past tense of the plural (\"they were late\", \"so were\n    gutted\") and is never touched: only the front of a line, or the front of a clause.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![Py::None, py_str("|")]), &before.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("|")]), &after.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g__A_QUESTION_VERB(), &after.clone()))) {
    return py_list(vec![py_str("where")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__WERE_STATE(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__WERE_DOING(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__WERE_ARE(), &after.clone())) } } } }) {
    return py_list(vec![py_str("we"), py_str("are")]);
    }
    if py_truthy(&Py::Bool(py_contains(&g__WERE_LEAD_IN(), &after.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__WERE_STATE(), &after_after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__WERE_DOING(), &after_after.clone())) } }) {
    return py_list(vec![py_str("we"), py_str("are")]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&after_after.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _a_doing(&[after_after.clone()]) } }) {
    return py_list(vec![py_str("we"), py_str("are")]);
    }
    if py_truthy(&Py::Bool(py_contains(&g__AFTER_A_LEAD_IN_SUBJECT(), &after_after.clone()))) {
    return py_list(vec![py_str("were")]);
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = _a_doing(&[after.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&after_after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__AFTER_A_DOING(), &after_after.clone())) } } } }) {
    return py_list(vec![py_str("we"), py_str("are")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__WERE_PLACE(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__WERE_IN_THE_PAST(), &after_after.clone())) } }) {
    return py_list(vec![py_str("we"), py_str("are")]);
    }
    return Py::None;
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
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut bare: Py = Py::None;
    let _ = py_str("Whether a word ending in -ing is a doing rather than a thing: \"breaking\", \"selling\".\n\n    Only a word the engine knows is read this way, and the g may be left off the way people type\n    it (\"were goin out tonight\"). What follows decides which of the two it is; this says only\n    that the word itself could be the doing.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    bare = mcall(&word.clone(), "replace", &[py_str("'"), py_str("")]);
    if py_truthy(&mcall(&bare.clone(), "endswith", &[py_str("ing")])) {
    return { let __b = Py::Bool(py_contains(&g_LEXICON(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &slice_or_raise(&bare.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None))) } };
    }
    if py_truthy(&{ let __b = mcall(&bare.clone(), "endswith", &[py_str("in")]); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return { let __b = Py::Bool(py_contains(&g_LEXICON(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &slice_or_raise(&bare.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None))) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_doubtful_split(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pieces: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pieces") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pieces"))) };
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::None };
    let _ = py_str("True when the words a glued token would be cut into are a name rather than a question.\n\n    \"dodo wont eat\" is \"do do wont eat\" and \"isis is my cat\" is \"is is is my cat\": a question\n    never says its first two words the same, so a split that does is a name. \"didi is coming\n    round\" is \"did i is coming round\": no do-form is followed by a be-form or a modal, so the\n    opener there is a name too. A pet or a person typed lower case keeps its line as the news\n    it was.\n    ");
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&pieces.clone(), &Py::Int(0)), &geti(&pieces.clone(), &Py::Int(1)))) } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&g_GLUED_DO(), &geti(&pieces.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[after.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GLUED_AFTER_A_DO(), &after.clone())) } } } };
    Py::None
}
pub fn _a_place_said_whole(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut bare: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bare") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: bare"))) };
    let mut COUNTRIES: Py = Py::None;
    let mut places: Py = Py::None;
    let _ = py_str("True when the whole token is a place the bot knows, so it is not a pronoun and a verb.\n\n    \"iran is in the news\" is a country, and only by accident \"i\" + \"ran\"; the same is true of any\n    country whose name happens to start with a pronoun. The list is the bot's own, so a verb added\n    to `I_GLUED` later cannot take a country with it, and a name from `COMMON_NAMES` is refused the\n    same way.\n    ");
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "COMMON_NAMES"), &bare.clone()))) {
    return Py::Bool(true);
    }
    places = geti(&g__PLACES_SAID_WHOLE(), &Py::Int(0));
    if py_truthy(&Py::Bool(py_is(&places.clone(), &Py::None))) {
    let __try1 = exc::catch(|| {
    COUNTRIES = pyrt::xrefs::xget("knowledge.geography", "COUNTRIES");
    places = call_builtin("frozenset", &[COUNTRIES.clone()]).unwrap_or_else(|e| exc::raise(e));
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
    places = call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let _ = py_setitem(&g__PLACES_SAID_WHOLE(), &Py::Int(0), places.clone());
    }
    return Py::Bool(py_contains(&places.clone(), &bare.clone()));
    Py::None
}
pub fn _a_pronoun_said_together(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut after: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The pronoun and the word it is typed onto, or None.\n\n    The apostrophe is kept, because that is how the person typed it and the readers already know\n    \"can't\" and \"don't\": \"ican't go on\" is \"i can't go on\", which is the crisis line, while the\n    glued token was no line at all.\n    ");
    bare = mcall(&mcall(&token.clone(), "replace", &[py_str("'"), py_str("")]), "replace", &[py_str("’"), py_str("")]);
    if py_truthy(&_a_place_said_whole(&[bare.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_A_PRONOUN_STATE(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    who = __unpk2[0].clone();
    after = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&bare.clone(), "startswith", &[who.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&after.clone(), &slice_or_raise(&bare.clone(), &call_builtin("len", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))) } }) {
    return py_list(vec![who.clone(), slice_or_raise(&bare.clone(), &call_builtin("len", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)]);
    }
    }
    let __iter3 = py_iter_or_raise(&g_A_PRONOUN_GLUED());
    for __item in __iter3 {
    who = __item.clone();
    if py_truthy(&{ let __b = mcall(&bare.clone(), "startswith", &[who.clone()]); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_I_GLUED(), &slice_or_raise(&bare.clone(), &call_builtin("len", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))) } } } }) {
    return py_list(vec![who.clone(), if py_truthy(&mcall(&token.clone(), "startswith", &[who.clone()])) { slice_or_raise(&token.clone(), &call_builtin("len", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None) } else { slice_or_raise(&bare.clone(), &call_builtin("len", &[who.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None) }]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_state_typed_onto_im(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut rest: Py = Py::None;
    let _ = py_str("The state typed onto the \"im\", or None: \"imsad\" is \"im sad\", \"imexhausted\" is tired.\n\n    The token has to be one no reader knows (the guard in `_glued_words`), so \"image\", \"impact\",\n    \"import\", \"improve\", \"imagine\" and \"immune\" keep their meaning: each of them is a word. What\n    is left over after the \"im\" has to be a state the reading already knows, which is why\n    \"improv\" and \"impressed\" are left alone here and read elsewhere.\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&token.clone(), "startswith", &[py_str("im")]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&token.clone(), "islower", &[]))) } } } }) {
    return Py::None;
    }
    rest = slice_or_raise(&token.clone(), &Py::Int(2), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "STATES_SAID_ONTO_IM"), &rest.clone()))) {
    return py_list(vec![py_str("im"), rest.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _glued_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::None };
    let mut a_state: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut half: Py = Py::None;
    let mut odd: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said_together: Py = Py::None;
    let mut second: Py = Py::None;
    let _ = py_str("The two words a glued opener stands for, or None when the token is a word of its own.\n\n    Only a token that is not in the lexicon at all is a candidate, which is what keeps \"island\",\n    \"issue\", \"washer\", \"dinner\" and \"cannon\" whole: each of them is already a word, and a word\n    keeps its meaning. The split is tried left to right by the length of the opener, so the\n    longest opener that fits wins (\"doesit\" before \"do\" + \"seit\"). `after` is the next word of\n    the line, which is what tells a name's split from a question's (_a_doubtful_split).\n    ");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEXICON(), &token.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&token.clone(), "islower", &[]))) } } } }) {
    return Py::None;
    }
    said_together = _a_pronoun_said_together(&[token.clone()]);
    if py_truthy(&said_together.clone()) {
    return said_together.clone();
    }
    a_state = _a_state_typed_onto_im(&[token.clone()]);
    if py_truthy(&a_state.clone()) {
    return a_state.clone();
    }
    let __unpk1 = py_unpack(&call_builtin("divmod", &[call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(2)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    half = __unpk1[0].clone();
    odd = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&odd.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&token.clone(), &Py::None, &half.clone(), &Py::None), &slice_or_raise(&token.clone(), &half.clone(), &Py::None, &Py::None))) } }) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![Py::Int(6), Py::Int(5), Py::Int(4), Py::Int(3), Py::Int(2)]));
    for __item in __iter2 {
    cut = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_GLUED_LEFT(), &slice_or_raise(&token.clone(), &Py::None, &cut.clone(), &Py::None)))) {
    continue;
    }
    rest = slice_or_raise(&token.clone(), &cut.clone(), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_contains(&g_GLUED_RIGHT(), &rest.clone()))) {
    pieces = py_list(vec![slice_or_raise(&token.clone(), &Py::None, &cut.clone(), &Py::None), rest.clone()]);
    return if py_truthy(&_a_doubtful_split(&[pieces.clone(), after.clone()])) { Py::None } else { pieces.clone() };
    }
    if py_truthy(&Py::Bool(py_contains(&g_GLUED_COLLAPSE(), &rest.clone()))) {
    return op_add(&py_list(vec![slice_or_raise(&token.clone(), &Py::None, &cut.clone(), &Py::None)]), &geti(&g_GLUED_COLLAPSE(), &rest.clone()));
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![Py::Int(5), Py::Int(4), Py::Int(3), Py::Int(2)]));
    for __item in __iter3 {
    second = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_GLUED_RIGHT(), &slice_or_raise(&rest.clone(), &Py::None, &second.clone(), &Py::None))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_GLUED_RIGHT(), &slice_or_raise(&rest.clone(), &second.clone(), &Py::None, &Py::None))) } }) {
    pieces = py_list(vec![slice_or_raise(&token.clone(), &Py::None, &cut.clone(), &Py::None), slice_or_raise(&rest.clone(), &Py::None, &second.clone(), &Py::None), slice_or_raise(&rest.clone(), &second.clone(), &Py::None, &Py::None)]);
    return if py_truthy(&_a_doubtful_split(&[pieces.clone(), after.clone()])) { Py::None } else { pieces.clone() };
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_mrs_is_the_wife(args: &[Py]) -> Py {
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
    let _ = py_str("A wife said as \"the mrs\", left alone where \"mrs\" is a title in front of a name.");
    return mcall(&g_MRS_IS_THE_WIFE(), "sub", &[py_str("\\1 wife"), text.clone()]);
    Py::None
}
pub fn _chat_words_said_out(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut out: Py = Py::None;
    let mut spelled: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A chat word spelled out in the line the readers match on.\n\n    \"i dont know what i wanna do\" is the same statement as \"i dont know what i want to do\", and\n    the readers that answer it are written on the words the person meant. The reading already\n    spells these out (`norm`), but a reader matching on `clean` - the line as it was typed -\n    never met the phrase, so the same line typed two ways got two different replies. Only the\n    words in `lexicon.PLAIN_EXPANDS` are spelled out: a word that is not a word at all, whose\n    repair is the phrase a reader is looking for. A contraction stays as it was typed (\"dont\" is\n    how people write \"do not\", and the patterns are written on it), and so does a word whose\n    spelling is somebody's name.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return clean.clone();
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&clean.clone(), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    spelled = mcall(&g_SAID_OUT(), "get", &[word.clone()]);
    before = if py_truthy(&out.clone()) { mcall(&mcall(&g_SAID_OUT(), "get", &[mcall(&geti(&out.clone(), &op_neg(&Py::Int(1))), "lower", &[]), mcall(&geti(&out.clone(), &op_neg(&Py::Int(1))), "lower", &[])]), "split", &[]) } else { py_list(vec![]) };
    if py_truthy(&{ let __b = spelled.clone(); if !py_truthy(&__b) { __b } else { { let __b = before.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__SUBJECT_SAID(), &geti(&before.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { mcall(&spelled.clone(), "startswith", &[py_tuple(vec![py_str("i "), py_str("I ")])]) } } } } } }) {
    spelled = geti(&mcall(&spelled.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(1));
    }
    let _ = mcall(&out.clone(), "append", &[{ let __b = spelled.clone(); if py_truthy(&__b) { __b } else { word.clone() } }]);
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn _a_hello_said_short(args: &[Py]) -> Py {
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
    let mut meant: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with a hello typed as initials, or with its tail bitten off, read as a hello.");
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return clean.clone();
    }
    let __iter1 = py_iter_or_raise(&g_HELLO_SAID_SHORT());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    typed = __unpk2[0].clone();
    meant = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&clean.clone(), &typed.clone()))) {
    return meant.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&clean.clone(), "startswith", &[op_add(&typed.clone(), &py_str(" "))]); if py_truthy(&__b) { __b } else { { let __b = mcall(&clean.clone(), "startswith", &[op_add(&typed.clone(), &py_str(","))]); if py_truthy(&__b) { __b } else { mcall(&clean.clone(), "startswith", &[op_add(&typed.clone(), &py_str("!"))]) } } } }))) {
    continue;
    }
    words = mcall(&mcall(&slice_or_raise(&clean.clone(), &call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[py_str(" ,!.")]), "split", &[]);
    while py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HELLO_TAIL(), &mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(",.!")]))) } }) {
    let _ = mcall(&words.clone(), "pop", &[Py::Int(0)]);
    }
    rest = mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" ,!.")]);
    return { let __b = rest.clone(); if py_truthy(&__b) { __b } else { meant.clone() } };
    }
    return clean.clone();
    Py::None
}
pub fn _glued_said_apart(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut spread: Py = Py::None;
    let mut token: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with every glued question opener put back into the words it stands for.\n\n    A shortened opener is spelled out on the way: \"whatsthe best way\" is \"what is the best way\",\n    not \"whats the best way\", because the modules read the words of the line and \"whats\" means\n    nothing to them, while \"what is\" is the same two words the spaced line carries.\n    ");
    out = py_list(vec![]);
    words = mcall(&clean.clone(), "split", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    token = __unpk2[1].clone();
    pieces = _glued_words(&[token.clone(), if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { Py::None }]);
    if py_truthy(&pieces.clone()) {
    spread = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[geti(&pieces.clone(), &Py::Int(0))]);
    if py_truthy(&{ let __b = spread.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&spread.clone(), &py_str(" "))) } }) {
    pieces = op_add(&mcall(&spread.clone(), "split", &[]), &slice_or_raise(&pieces.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    let _ = mcall(&out.clone(), "extend", &[if py_truthy(&pieces.clone()) { pieces.clone() } else { py_list(vec![token.clone()]) }]);
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn _said_run_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut best: Py = Py::None;
    let mut end: Py = Py::None;
    let mut here: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut split: Py = Py::None;
    let mut start: Py = Py::None;
    let mut there: Py = Py::None;
    let _ = py_str("The words a line typed without spaces is made of, or None when the token is not one.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&token.clone(), "isalpha", &[])))) {
    return Py::None;
    }
    token = mcall(&token.clone(), "lower", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_RUN_ON_SHORTEST())) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_LEXICON(), &token.clone()))) {
    return Py::None;
    }
    best = py_dict(vec![(Py::Int(0), py_tuple(vec![]))]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    start = __item.clone();
    pieces = mcall(&best.clone(), "get", &[start.clone()]);
    if py_truthy(&Py::Bool(py_is(&pieces.clone(), &Py::None))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)), start.clone(), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    end = __item.clone();
    piece = slice_or_raise(&token.clone(), &start.clone(), &end.clone(), &Py::None);
    if py_truthy(&Py::Bool(!py_contains(&g_RUN_ON_WORDS(), &piece.clone()))) {
    continue;
    }
    here = op_add(&pieces.clone(), &py_tuple(vec![piece.clone()]));
    there = mcall(&best.clone(), "get", &[end.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&there.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[here.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[there.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    let _ = py_setitem(&best.clone(), &end.clone(), here.clone());
    }
    }
    }
    split = mcall(&best.clone(), "get", &[call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&split.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[split.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_RUN_ON_LAST_PIECES(), &geti(&split.clone(), &op_neg(&Py::Int(1)))))) {
    return Py::None;
    }
    if py_truthy(&op_gt(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&split.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[piece.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_RUN_ON_SELF(), &geti(&split.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_RUN_ON_MINE(), &geti(&split.clone(), &Py::Int(0)))) } }) {
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_RUN_ON_SELF_LONGEST()); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[split.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return Py::None;
    }
    return call_builtin("list", &[split.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(py_contains(&g_RUN_ON_OPENERS(), &geti(&split.clone(), &Py::Int(0))))) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&split.clone()) { let mut piece: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_RUN_ON_PEOPLE(), &piece.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return call_builtin("list", &[split.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::None;
    Py::None
}
pub fn _a_run_and_what_surrounds_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut end: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("A token as (what comes before the letters, the letters, what comes after them).\n\n    The letters of a run are the letters wherever a phone put a capital or a person put a\n    question mark: \"WhatShouldIDo\" and \"whatshouldido?\" are the same five words as\n    \"whatshouldido\", and all three were answered with a line that says nothing.\n    ");
    start = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&start.clone(), &call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&geti(&token.clone(), &start.clone()), "isalpha", &[]))) } }) {
    let __aug1 = op_add(&start.clone(), &Py::Int(1));
    start = __aug1;
    }
    end = call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&{ let __b = op_gt(&end.clone(), &start.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&geti(&token.clone(), &op_sub(&end.clone(), &Py::Int(1))), "isalpha", &[]))) } }) {
    let __aug2 = op_sub(&end.clone(), &Py::Int(1));
    end = __aug2;
    }
    return py_tuple(vec![slice_or_raise(&token.clone(), &Py::None, &start.clone(), &Py::None), slice_or_raise(&token.clone(), &start.clone(), &end.clone(), &Py::None), slice_or_raise(&token.clone(), &end.clone(), &Py::None, &Py::None)]);
    Py::None
}
pub fn _typed_without_spaces(args: &[Py]) -> Py {
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
    let mut letters: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut spread: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("The line with any run of words typed as one long word put back into its words.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&clean.clone(), "split", &[]));
    for __item in __iter1 {
    token = __item.clone();
    let __unpk2 = py_unpack(&_a_run_and_what_surrounds_it(&[token.clone()]), 3, None);
    before = __unpk2[0].clone();
    letters = __unpk2[1].clone();
    after = __unpk2[2].clone();
    pieces = _said_run_on(&[letters.clone()]);
    if py_truthy(&pieces.clone()) {
    spread = mcall(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), "get", &[geti(&pieces.clone(), &Py::Int(0))]);
    if py_truthy(&{ let __b = spread.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&spread.clone(), &py_str(" "))) } }) {
    pieces = op_add(&mcall(&spread.clone(), "split", &[]), &slice_or_raise(&pieces.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    let _ = mcall(&out.clone(), "extend", &[pieces.clone()]);
    if py_truthy(&after.clone()) {
    let _ = py_setitem(&out.clone(), &op_neg(&Py::Int(1)), op_add(&geti(&out.clone(), &op_neg(&Py::Int(1))), &after.clone()));
    }
    if py_truthy(&before.clone()) {
    let _ = py_setitem(&out.clone(), &Py::Int(0), op_add(&before.clone(), &geti(&out.clone(), &Py::Int(0))));
    }
    } else {
    let _ = mcall(&out.clone(), "append", &[token.clone()]);
    }
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn _the_said_apart_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut base: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("({pieces: the word}, one pattern) for the contractions that can be typed as two words.\n\n    Every way of splitting the known contraction spellings into two or three pieces, kept only\n    where one piece is a single letter, so the pattern can never claim two ordinary words.\n    ");
    forms = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&op_add(&call_builtin("tuple", &[pyrt::xrefs::xget("understanding.lexicon", "EXPAND")]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[pyrt::xrefs::xget("understanding.lexicon", "TYPED_CONTRACTIONS")]).unwrap_or_else(|e| exc::raise(e))));
    for __item in __iter1 {
    joined = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&joined.clone(), "isalpha", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __cmp21: Py = call_builtin("len", &[joined.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp21)) { op_le(&__cmp21, &Py::Int(10)) } else { Py::Bool(false) } })) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&_the_ways_a_word_is_typed_apart(&[joined.clone()]));
    for __item in __iter3 {
    pieces = __item.clone();
    let _ = mcall(&forms.clone(), "setdefault", &[pieces.clone(), joined.clone()]);
    }
    }
    let __iter4 = py_iter_or_raise(&g_APOSTROPHE_BASES());
    for __item in __iter4 {
    base = __item.clone();
    let __iter5 = py_iter_or_raise(&g_APOSTROPHE_TAILS());
    for __item in __iter5 {
    tail = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_APOSTROPHE_NOT(), &py_tuple(vec![base.clone(), tail.clone()])))) {
    continue;
    }
    one = op_add(&op_add(&base.clone(), &py_str("'")), &tail.clone());
    if py_truthy(&_apostrophe(&[one.clone()])) {
    let _ = mcall(&forms.clone(), "setdefault", &[py_tuple(vec![base.clone(), tail.clone()]), one.clone()]);
    }
    }
    }
    return forms.clone();
    Py::None
}
pub fn _the_ways_a_word_is_typed_apart(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut joined: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "joined") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: joined"))) };
    let mut cut: Py = Py::None;
    let mut first: Py = Py::None;
    let mut first_cut: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut second_cut: Py = Py::None;
    let mut ways: Py = Py::None;
    let _ = py_str("The ways this one word arrives as two or three tokens: \"dont\" is \"don t\" or \"d ont\".");
    ways = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), call_builtin("len", &[joined.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    cut = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![slice_or_raise(&joined.clone(), &Py::None, &cut.clone(), &Py::None), slice_or_raise(&joined.clone(), &cut.clone(), &Py::None, &Py::None)]), 2, None);
    first = __unpk2[0].clone();
    rest = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    let _ = mcall(&ways.clone(), "append", &[py_tuple(vec![first.clone(), rest.clone()])]);
    }
    }
    let __iter3 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), op_sub(&call_builtin("len", &[joined.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    first_cut = __item.clone();
    let __iter4 = py_iter_or_raise(&call_builtin("range", &[op_add(&first_cut.clone(), &Py::Int(1)), call_builtin("len", &[joined.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    second_cut = __item.clone();
    pieces = py_tuple(vec![slice_or_raise(&joined.clone(), &Py::None, &first_cut.clone(), &Py::None), slice_or_raise(&joined.clone(), &first_cut.clone(), &second_cut.clone(), &Py::None), slice_or_raise(&joined.clone(), &second_cut.clone(), &Py::None, &Py::None)]);
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); __out.push(call_builtin("len", &[piece.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }, &Py::Int(1)))) {
    let _ = mcall(&ways.clone(), "append", &[pieces.clone()]);
    }
    }
    }
    return ways.clone();
    Py::None
}
pub fn _said_apart(args: &[Py]) -> Py {
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
    let mut put_together: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The contractions typed with a space in place of the apostrophe, put back together.");
    if py_truthy(&op_lt(&call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return tokens.clone();
    }
    said = mcall(&py_str(" "), "join", &[tokens.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SAID_APART(), "search", &[said.clone()])))) {
    return tokens.clone();
    }
pub fn __nested_put_together_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut put_together: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut tokens: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut before: Py = Py::None;
    before = mcall(&slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[]);
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_LETTER_AFTER(), &geti(&before.clone(), &op_neg(&Py::Int(1))))) } }) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    return geti(&g_SAID_APART_FORMS(), &call_builtin("tuple", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
    put_together = {{ let put_together = put_together.clone(); let said = said.clone(); let tokens = tokens.clone(); py_func("put_together", move |args: &[Py]| { let mut full = vec![put_together.clone(), said.clone(), tokens.clone()]; full.extend_from_slice(args); __nested_put_together_0(&full) }) }};
    return mcall(&mcall(&g_SAID_APART(), "sub", &[put_together.clone(), said.clone()]), "split", &[]);
    Py::None
}
pub fn said_apart(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The line with a contraction typed as two words put back together: \"she s\" -> \"she's\".\n\n    A module that matches English words rather than the engine's reading calls this so that the\n    one word people typed as two arrives as one, the same shape the reading has.\n    ");
    line = call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    words = mcall(&line.clone(), "split", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return line.clone();
    }
    return mcall(&py_str(" "), "join", &[_said_apart(&[words.clone()])]);
    Py::None
}
pub fn _ill_is_i_will_going(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("[\"i\", \"will\"] for an \"ill\" with a verb after it that needs a subject, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&token.clone(), &py_str("ill"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ILL_GOES_ON(), &after.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&before.clone(), &py_str("|"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_LEADERS(), &before.clone())) } } } }) {
    return Py::None;
    }
    return py_list(vec![py_str("i"), py_str("will")]);
    Py::None
}
pub fn _an_ill_that_is_i_will(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut changed: Py = Py::None;
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`clean` with an \"ill\" that can only be \"i will\" set down as the two words.\n\n    The readers that answer match on `clean` and `words`, so the contraction has to be put right\n    where the line is written or the knowledge base still sees somebody who is ill. The same test\n    as `_ill_is_i_will_going`: the verb after it needs a subject, and nothing in front of the \"ill\"\n    is one.\n    ");
    words = mcall(&clean.clone(), "split", &[]);
    changed = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&word.clone(), &py_str("ill"))); if py_truthy(&__b) { __b } else { op_ge(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_ILL_GOES_ON(), &geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1)))))) {
    continue;
    }
    before = if py_truthy(&index.clone()) { geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&before.clone(), &py_str("|"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_LEADERS(), &before.clone())) } } } }) {
    continue;
    }
    let _ = py_setitem(&words.clone(), &index.clone(), py_str("i will"));
    changed = Py::Bool(true);
    }
    return if py_truthy(&changed.clone()) { mcall(&py_str(" "), "join", &[words.clone()]) } else { clean.clone() };
    Py::None
}
pub fn _left_off_contraction(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut before_before: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before_before"))) };
    let mut leaders_only: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The two words a contraction written without its apostrophe stands for, or None.\n\n    The next word has to be one only the contraction can be followed by, and the word in\n    front must leave the plain reading out: \"were going to spain\" reads as the contraction,\n    while \"we were late\", \"how were things\" and \"were you there\" keep their words.\n    ");
    words = mcall(&g_LEFT_OFF_CONTRACTION(), "get", &[token.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_CONTRACTION_NEXT(), &after.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&before.clone(), &py_str("|"))) } }) {
    leaders_only = { let __b = Py::Bool(py_contains(&g_LEADERS(), &before.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("|")]), &before_before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEADERS(), &before_before.clone())) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&leaders_only.clone()))) {
    return Py::None;
    }
    }
    return call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _one_of_the_parts(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("The part a typed s on one of a kind stands for: \"my backs\" -> \"my back\", or None.\n\n    Only after a possessive, and only when the word behind it says what the one part is doing,\n    so a real plural keeps its s: \"my knees have been playing up\" and \"our backs are aching\" are\n    untouched, and \"my backs playing up\" is read as the back it is.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&token.clone(), "endswith", &[py_str("s")]))); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[token.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ONE_OF_A_KIND(), &slice_or_raise(&token.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_is_possessive(&[before.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ONE_PART_AFTER(), &after.clone())); if py_truthy(&__b) { __b } else { mcall(&{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "endswith", &[py_tuple(vec![py_str("ing"), py_str("in")])]) } }) {
    return slice_or_raise(&token.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return Py::None;
    Py::None
}
pub fn _wers_word(args: &[Py]) -> Py {
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
    let _ = py_str("The two words a typed \"wers\" stands for, from the word in front of it.\n\n    \"wers\" is the e and the apostrophe gone from \"where's\" and the a gone from \"wears\", and no\n    word list can say which: \"wers my keys\" is the lost keys, \"she wers a red coat\" is what she\n    wears. The word in front decides, because the verb has to have a person doing the wearing\n    in front of it and the question has to be at the front of its clause: after a person it is\n    \"wears\", at the front of a line or a clause or after a lead-in word (\"so wers my keys\") it\n    is \"where is\".\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("|")]), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEADERS(), &before.clone())) } }) {
    return py_list(vec![py_str("where"), py_str("is")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WEARS_AFTER(), &_plain_chat(&[before.clone()]))); if py_truthy(&__b) { __b } else { _is_person(&[before.clone()]) } }) {
    return py_list(vec![py_str("wears")]);
    }
    return py_list(vec![py_str("where"), py_str("is")]);
    Py::None
}
pub fn _sum_word(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut behind: Py = Py::None;
    let mut spelled: Py = Py::None;
    let _ = py_str("The word a typed \"sum\" stands for, from the words around it.\n\n    \"sum\" is \"some\" with the o left out, and it is also the arithmetic noun and the verb of\n    adding up, and the maths reading was winning everywhere: \"i gotta loose sum weight b4 the\n    weddin\" was read as a line about numbers, so the topic was maths-and-numbers and the \"loose\"\n    in front of it was not repaired to \"lose\". A determiner or the name of a lump of money in\n    front is the noun; \"up\", \"it\", \"things\" or a demonstrative behind is the verb (\"to sum up\",\n    \"let me sum it up\", \"sum up what happened\"); anything else is \"some\" (\"i need sum help with\n    my cv\", \"can i get sum milk\", \"sum of us are going\").\n    ");
    spelled = if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![Py::None, py_str("|")]), &before.clone()))) { _plain_chat(&[before.clone()]) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_SUM_THE_NOUN(), &spelled.clone()))) {
    return py_list(vec![py_str("sum")]);
    }
    behind = if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![Py::None, py_str("|")]), &after.clone()))) { _plain_chat(&[after.clone()]) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_SUM_STAYS_AFTER(), &behind.clone()))) {
    return py_list(vec![py_str("sum")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("in"), py_str("to")]), &spelled.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_SUM_STAYS_AFTER(), &behind.clone())) } }) {
    return py_list(vec![py_str("sum")]);
    }
    return py_list(vec![py_str("some")]);
    Py::None
}
pub fn _positional_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut after_after: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_after") { v.clone() }
    else { Py::None };
    let mut before_before: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_before") { v.clone() }
    else { Py::None };
    let mut before_words: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before_words") { v.clone() }
    else { Py::None };
    let mut after_read: Py = Py::None;
    let mut before_read: Py = Py::None;
    let mut contraction: Py = Py::None;
    let mut family: Py = Py::None;
    let mut feeling: Py = Py::None;
    let mut going: Py = Py::None;
    let mut kin: Py = Py::None;
    let mut many: Py = Py::None;
    let mut one: Py = Py::None;
    let mut one_part: Py = Py::None;
    let mut read: Py = Py::None;
    let mut spelled: Py = Py::None;
    let mut state: Py = Py::None;
    let _ = py_str("The word a quickly typed token stands for, from where it sits, or the token itself.\n\n    \"w\" on its own is not a word and is \"with\"; \"2\" and \"4\" are \"to\" and \"for\" in front of a\n    phrase opener, and in front of a demonstrative unless a period follows it; \"no\" is \"know\"\n    between a subject and what a verb takes; \"were\" at the front of a line is \"where\" in front\n    of a verb and \"we are\" in front of anything else.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_UP_TO_SPELLINGS(), &token.clone())); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_is(&before.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&before.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_UP_TO_LEADS(), &before.clone())) } } } }; if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_is(&after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_UP_TO_TAGS(), &after.clone())) } } } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&after_after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&after_after.clone(), &py_str("|"))) } } } } } } } }) {
    return py_list(vec![py_str("what"), py_str("you"), py_str("up"), py_str("to")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WHAT_ARE_YOU_SPELLINGS(), &token.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WHAT_ARE_YOU_AFTER(), &after.clone())) } }) {
    return py_list(vec![py_str("what"), py_str("you")]);
    }
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("w")))) {
    return py_list(vec![py_str("with")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("c"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LETTER_C_IS_SEE(), &after.clone())) } }) {
    return py_list(vec![py_str("see")]);
    }
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("wers")))) {
    return _wers_word(&[before.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("sum")))) {
    return _sum_word(&[before.clone(), after.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("were")))) {
    spelled = _were_for_another_word(&[before.clone(), after.clone(), after_after.clone()]);
    if py_truthy(&spelled.clone()) {
    return spelled.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CLIPPED_THINGS(), &token.clone())); if !py_truthy(&__b) { __b } else { _is_a_thing_here(&[before.clone(), after.clone(), after_after.clone()]) } }) {
    return py_list(vec![_clipped_thing(&[token.clone()])]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_CLIPPED_UNITS(), &token.clone())); if !py_truthy(&__b) { __b } else { { let __b = _is_a_count(&[before.clone()]); if py_truthy(&__b) { __b } else { _says_one(&[before.clone()]) } } } }) {
    let __unpk1 = py_unpack(&geti(&g_CLIPPED_UNITS(), &token.clone()), 2, None);
    one = __unpk1[0].clone();
    many = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("hr"))); if !py_truthy(&__b) { __b } else { _hr_is_the_department(&[after.clone()]) } }) {
    return py_list(vec![token.clone()]);
    }
    if py_truthy(&{ let __b = _says_one(&[before.clone()]); if py_truthy(&__b) { __b } else { _is_before_a_noun(&[token.clone(), after.clone()]) } }) {
    return py_list(vec![one.clone()]);
    }
    return py_list(vec![many.clone()]);
    }
    contraction = _left_off_contraction(&[token.clone(), before.clone(), after.clone(), before_before.clone()]);
    if py_truthy(&contraction.clone()) {
    return contraction.clone();
    }
    going = _ill_is_i_will_going(&[token.clone(), before.clone(), after.clone()]);
    if py_truthy(&going.clone()) {
    return going.clone();
    }
    feeling = _sounds_like_a_feeling(&[token.clone(), before.clone(), after.clone(), before_before.clone()]);
    if py_truthy(&feeling.clone()) {
    return py_list(vec![feeling.clone()]);
    }
    read = _digit_word(&[token.clone(), before.clone(), after.clone(), after_after.clone()]);
    if py_truthy(&read.clone()) {
    return py_list(vec![read.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("wdyt"))); if !py_truthy(&__b) { __b } else { { let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&after.clone(), &py_str("|"))) } } } }) {
    return py_list(vec![py_str("what"), py_str("do"), py_str("you"), py_str("think")]);
    }
    one_part = _one_of_the_parts(&[token.clone(), before.clone(), after.clone()]);
    if py_truthy(&one_part.clone()) {
    return py_list(vec![one_part.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("ow"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HOW_FOLLOWS(), &after.clone())) } }) {
    return py_list(vec![py_str("how")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("bout"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DETERMINERS(), &before.clone())) } }) {
    return py_list(vec![py_str("about")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("no"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_KNOW_SUBJECT(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_BEFORE_KNOW(), &before.clone())) } } } }) {
    after_read = if py_truthy(&{ let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&after.clone(), &py_str("|"))) } }) { _plain_chat(&[after.clone()]) } else { after.clone() };
    if py_truthy(&{ let __b = Py::Bool(py_is(&after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_A_VERB_CAN_FOLLOW(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_VERB_CAN_FOLLOW(), &after_read.clone())) } } } } } }) {
    return py_list(vec![py_str("know")]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("hv")))) {
    before_read = if py_truthy(&before.clone()) { _plain_chat(&[before.clone()]) } else { py_str("") };
    after_read = if py_truthy(&after.clone()) { _plain_chat(&[after.clone()]) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_DETERMINERS(), &before_read.clone()))) {
    return py_list(vec![token.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_HV_SUBJECTS(), &before_read.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_HV_HELPERS(), &before_read.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&after_read.clone(), &py_str("to"))) } } } }) {
    return py_list(vec![py_str("have")]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_HV_SUBJECTS(), &after_read.clone()))) {
    return py_list(vec![py_str("have")]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("bin")))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_HAVE_FORMS(), &before.clone())); if py_truthy(&__b) { __b } else { mcall(&{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "endswith", &[py_str("ve")]) } }) {
    return py_list(vec![py_str("been")]);
    }
    if py_truthy(&{ let __b = _is_person(&[before.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PERSON_PREPS(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = _is_participle(&[after.clone()]); if py_truthy(&__b) { __b } else { _is_said_of_a_person(&[after.clone()]) } } } } } }) {
    return py_list(vec![py_str("been")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_THING_POSSESSIVES(), &before_before.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_BIN_CONTAINERS(), &if py_truthy(&before.clone()) { _plain_chat(&[before.clone()]) } else { py_str("") })); if !py_truthy(&__b) { __b } else { _the_thing_has_been(&[after.clone()]) } } } }) {
    return py_list(vec![py_str("been")]);
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&g_BEEN_SUBJECT(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BE_FORMS(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![Py::None, py_str("|")]), &before.clone())) } } } }; if !py_truthy(&__b) { __b } else { { let __b = _is_said_of_a_person(&[after.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "isdigit", &[]); if py_truthy(&__b) { __b } else { { let __b = _counts_a_period(&[after.clone()]); if py_truthy(&__b) { __b } else { { let __b = _counts_a_period(&[after_after.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BEEN_SUBJECT(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &after.clone())) } } } } } } } } } } } }) {
    return py_list(vec![py_str("been")]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("of"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MODAL_FORMS(), &_plain_chat(&[before.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = _is_participle(&[after.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DETERMINERS(), &before_before.clone())) } } } } } }) {
    return py_list(vec![py_str("have")]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_AINT_FORMS(), &token.clone()))) {
    if py_truthy(&{ let __b = _is_participle(&[after.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PAST_AS_PARTICIPLE(), &after.clone())) } }) {
    return if py_truthy(&Py::Bool(py_contains(&g_HAS_SUBJECT(), &before.clone()))) { py_list(vec![py_str("has"), py_str("not")]) } else { py_list(vec![py_str("have"), py_str("not")]) };
    }
    if py_truthy(&Py::Bool(py_eq(&_plain_chat(&[before.clone()]), &py_str("i")))) {
    return py_list(vec![py_str("am"), py_str("not")]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_AINT_PLURAL(), &_plain_chat(&[before.clone()])))) {
    return py_list(vec![py_str("are"), py_str("not")]);
    }
    return py_list(vec![py_str("is"), py_str("not")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("of"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("to"))); if !py_truthy(&__b) { __b } else { _is_be(&[before.clone()]) } } } }) {
    return py_list(vec![py_str("off")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("b"))); if !py_truthy(&__b) { __b } else { _spells_a_helper(&[before.clone()]) } }) {
    return py_list(vec![py_str("be")]);
    }
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_is(&after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&after.clone(), &py_str("|"))) } }; if !py_truthy(&__b) { __b } else { _where_a_state_stands(&[before.clone(), before_before.clone(), after.clone(), before_words.clone()]) } }) {
    state = _a_letter_wrong_in_a_state(&[token.clone()]);
    if py_truthy(&state.clone()) {
    return py_list(vec![state.clone()]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FIRST_PERSON_BE(), &before.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_FIRST_PERSON_BE(), &before_before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AMOUNT_BEFORE_A_STATE(), &before.clone())) } } } }) {
    feeling = _feeling_without_its_ending(&[token.clone()]);
    if py_truthy(&feeling.clone()) {
    return py_list(vec![feeling.clone()]);
    }
    }
    kin = _kin_needs_a_helper(&[token.clone(), before.clone(), after.clone(), { let __b = Py::Bool(py_is(&before_before.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&before_before.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEADERS(), &before_before.clone())) } } } }, after_after.clone()]);
    if py_truthy(&kin.clone()) {
    return kin.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("yr"), py_str("yrs")]), &token.clone())); if !py_truthy(&__b) { __b } else { mcall(&{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "isdigit", &[]) } }) {
    return py_list(vec![if py_truthy(&{ let __b = mcall(&token.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&after.clone(), &py_str("old"))) } }) { py_str("years") } else { py_str("year") }]);
    }
    family = _family_short(&[token.clone(), before.clone(), after.clone(), after_after.clone()]);
    if py_truthy(&family.clone()) {
    return family.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("veggies"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VEGGIES_AFTER(), &before.clone())) } }) {
    return py_list(vec![py_str("vegetables")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("ur"), py_str("your")]), &token.clone())); if !py_truthy(&__b) { __b } else { { let __b = after.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_YOU_ARE_AFTER(), &after.clone())) } } } }) {
    return py_list(vec![py_str("you're")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("mi"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MI_IS_ME_AFTER(), &after.clone())) } }) {
    return py_list(vec![py_str("me")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("mi"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MI_IS_A_NAME_AFTER(), &before.clone())) } }) {
    return py_list(vec![token.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("mi"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MI_IS_ME_BEFORE(), &after.clone())) } }) {
    return py_list(vec![py_str("me")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("mi"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("back"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MI_BACK_VERBS(), &before.clone())) } } } }) {
    return py_list(vec![py_str("me")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("mi"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&after.clone(), &py_str("|"))) } } } }) {
    return py_list(vec![py_str("me")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&token.clone(), &py_str("mi"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&g_SOLFEGE(), &before.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&after.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SOLFEGE(), &after.clone())) } } } } } })) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&before.clone(), &py_str("|"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_LEXICON(), &before.clone())) } } } }) {
    return py_list(vec![token.clone()]);
    }
    return py_list(vec![py_str("my")]);
    }
    return py_list(vec![token.clone()]);
    Py::None
}
pub fn _join_chat_digits(args: &[Py]) -> Py {
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
    let mut following: Py = Py::None;
    let mut index: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut spelt: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("A chat word typed in pieces stays one word: \"any1\", \"l8r\", \"2nite\", and \"i d k\".\n\n    The tokeniser keeps a measure together (\"5kg\", \"1.5kg\") but splits a letter run from a\n    digit run, so the digit spelling of a chat word arrives in two or three pieces (\"l\" + \"8r\",\n    \"2\" + \"nite\") and never reaches the chat table. Pieces are joined only into a word the\n    lexicon knows, so an ordinary digit pair (\"i have 4 cats\", \"a table for 2\") is left alone,\n    and a measure (\"1.5kg\", \"20mg\") is one token already and never a candidate.\n\n    Somebody typing with one hand writes the same chat word with a space between every letter:\n    \"i d k\", \"o m g\", \"n e 1\". Nothing read those runs, so `i d k` came out as \"i'd k\" and the\n    line reached no card at all. Three or four one-character pieces that spell a chat word are\n    joined the same way, and a run that spells nothing the lexicon knows (\"c a t\", \"i have 4\n    cats\") is left exactly as typed.\n    ");
    joined = py_list(vec![]);
    index = Py::Int(0);
    while py_truthy(&op_lt(&index.clone(), &call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    token = geti(&tokens.clone(), &index.clone());
    following = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&tokens.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&following.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&slice_or_raise(&following.clone(), &Py::None, &Py::Int(1), &Py::None), "isdigit", &[]); if py_truthy(&__b) { __b } else { mcall(&token.clone(), "isdigit", &[]) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &op_add(&token.clone(), &following.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), &op_add(&token.clone(), &following.clone()))) } } } } } }) {
    let _ = mcall(&joined.clone(), "append", &[op_add(&token.clone(), &following.clone())]);
    let __aug1 = op_add(&index.clone(), &Py::Int(2));
    index = __aug1;
    continue;
    }
    spelt = _said_apart_as_one(&[tokens.clone(), index.clone()]);
    if py_truthy(&Py::Bool(!py_is(&spelt.clone(), &Py::None))) {
    let _ = mcall(&joined.clone(), "append", &[geti(&spelt.clone(), &Py::Int(0))]);
    index = geti(&spelt.clone(), &Py::Int(1));
    continue;
    }
    let _ = mcall(&joined.clone(), "append", &[token.clone()]);
    let __aug2 = op_add(&index.clone(), &Py::Int(1));
    index = __aug2;
    }
    return joined.clone();
    Py::None
}
pub fn _said_apart_as_one_word(args: &[Py]) -> Py {
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
    let mut join: Py = Py::None;
    let _ = py_str("Join \"i d k\" into \"idk\", and only when the run spells a word the lexicon knows.\n\n    Longest first, so \"n e 1\" is read as \"ne1\" and not as a shorter piece of it. Two pieces are\n    deliberately left alone: \"i m\", \"didn t\" and \"c section\" are read by the tables that expect\n    them, and joining them here would take those lines away from those readers.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__SPACED_RUN(), "search", &[clean.clone()])))) {
    return clean.clone();
    }
pub fn __nested_join_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut clean: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut join: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut length: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut word: Py = Py::None;
    pieces = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(2), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    length = __item.clone();
    word = mcall(&mcall(&py_str(""), "join", &[slice_or_raise(&pieces.clone(), &Py::None, &length.clone(), &Py::None)]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), &word.clone())) } }) {
    return mcall(&py_str(" "), "join", &[op_add(&py_list(vec![word.clone()]), &slice_or_raise(&pieces.clone(), &length.clone(), &Py::None, &Py::None))]);
    }
    }
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    Py::None
}
    join = {{ let clean = clean.clone(); let join = join.clone(); py_func("join", move |args: &[Py]| { let mut full = vec![clean.clone(), join.clone()]; full.extend_from_slice(args); __nested_join_0(&full) }) }};
    return mcall(&g__SPACED_RUN(), "sub", &[join.clone(), clean.clone()]);
    Py::None
}
pub fn _said_apart_as_one(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut length: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The chat word at this position and the index after it, or None.\n\n    Longest first, so \"n e 1\" is \"ne1\" and never a shorter piece of it, and never more than four\n    pieces, because five one-character words in a row is a spelling and not a chat word.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__SPACED_PIECE(), "match", &[geti(&tokens.clone(), &start.clone())])))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![Py::Int(4), Py::Int(3)]));
    for __item in __iter1 {
    length = __item.clone();
    pieces = slice_or_raise(&tokens.clone(), &start.clone(), &op_add(&start.clone(), &length.clone()), &Py::None);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e)), &length.clone()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); __out.push(mcall(&g__SPACED_PIECE(), "match", &[piece.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    word = mcall(&py_str(""), "join", &[pieces.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CHAT"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), &word.clone())) } }) {
    return py_tuple(vec![word.clone(), op_add(&start.clone(), &length.clone())]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _docs_on_a_screen(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line is about the docs on a screen rather than a visit to the surgery.");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOCS_SCREEN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("phone"), py_str("tablet"), py_str("laptop"), py_str("pc"), py_str("screen"), py_str("file")]), &word.clone()))) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&index.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None)) { let mut back: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOCS_ON(), &back.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _near_a_meal(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::None };
    let mut found: Py = Py::None;
    let mut meal: Py = Py::None;
    let _ = py_str("The meal a misspelt word names, or None.\n\n    Only in the slot a meal goes in (behind \"for\", \"after\", \"before\", ...), and only one letter\n    out from one of the meals: \"tae\", \"taa\", \"tay\", \"dinnar\", \"sunner\". A word the lexicon knows\n    is left alone, so \"for two\", \"at tea\", \"for the launch\" and a golf tee are untouched.\n\n    The position is what keeps a real sentence out: the misspelt meal has to end the line or sit\n    in front of a time word, because that is where a meal stands (\"whats for tae\", \"whats for\n    brea tomorrow\"). Behind \"for\" alone it is far too easy - \"ask for branch details\", \"for\n    upper floor\", \"for inner peace\", \"for bunch of flowers\" are all one letter from a meal and\n    all words of their own, most of them outside the lexicon.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEXICON(), &word.clone())) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&g_FOOD_SLOT_LEAD(), &_plain_chat(&[before.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&after.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_TIME_AFTER_MEAL(), &_plain_chat(&[after.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_MEAL_TYPO(), &word.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("sorted", &[g_MEAL_WORDS()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    meal = __item.clone();
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&meal.clone(), "startswith", &[word.clone()]) } }) {
    return meal.clone();
    }
    }
    found = call_builtin("sorted", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MEAL_WORDS()) { let mut meal: Py = __item.clone(); if py_truthy(&_one_letter_apart(&[word.clone(), meal.clone()])) {  __out.push(meal.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[found.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&found.clone(), &Py::Int(0)) } else { Py::None };
    Py::None
}
pub fn _a_meal_word_misspelt(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut meal: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut places: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Put a meal word typed at speed back in the line, in `clean` as well as the reading.\n\n    It is repaired in `clean` for the reason the \"docs\" repair is: the kitchen's rows are keyed\n    on the meal, so a reading fixed on its own would leave the reply where it was. The slot is\n    what makes it safe - the word in front has to be the preposition a meal goes behind, and\n    what follows has to be the end of the line or the time it is for - and that is what keeps a\n    name out of it (\"tae is coming over\", \"me and tae went out\"). A name can sit behind \"for\"\n    (\"im waiting for tae\" reads as the meal), so it is the position, not the name, that decides.\n    ");
    places = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&index.clone()))) {
    continue;
    }
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { Py::None };
    meal = _near_a_meal(&[word.clone(), geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))), after.clone()]);
    if py_truthy(&meal.clone()) {
    let _ = py_setitem(&places.clone(), &index.clone(), meal.clone());
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&places.clone()))) {
    return py_tuple(vec![clean.clone(), words.clone()]);
    }
    let __unpk3 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    pieces = __unpk3[0].clone();
    last = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[mcall(&g_RAW_WORD(), "finditer", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    index = __unpk5[0].clone();
    match_ = __unpk5[1].clone();
    let _ = mcall(&pieces.clone(), "append", &[slice_or_raise(&clean.clone(), &last.clone(), &mcall(&match_.clone(), "start", &[]), &Py::None)]);
    let _ = mcall(&pieces.clone(), "append", &[mcall(&places.clone(), "get", &[index.clone(), mcall(&match_.clone(), "group", &[Py::Int(0)])])]);
    last = mcall(&match_.clone(), "end", &[]);
    }
    let _ = mcall(&pieces.clone(), "append", &[slice_or_raise(&clean.clone(), &last.clone(), &Py::None, &Py::None)]);
    let __iter6 = py_iter_or_raise(&mcall(&places.clone(), "items", &[]));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    index = __unpk7[0].clone();
    meal = __unpk7[1].clone();
    let _ = py_setitem(&words.clone(), &index.clone(), meal.clone());
    }
    return py_tuple(vec![mcall(&py_str(""), "join", &[pieces.clone()]), words.clone()]);
    Py::None
}
pub fn _docs_doctor(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut near: Py = Py::None;
    let mut other: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut places: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"Docs\" is read as the doctor where the line is about the surgery, and left as it is.\n\n    Only the two spellings people type for it, `doc` and `docs`, and only when a verb of going\n    or ringing is within a few words on either side and the line names no paperwork at all. A\n    line naming neither stays as typed: guessing one way would be worse than not guessing.\n\n    It is the one repair made in `clean` rather than only in `plain`, because no table anywhere\n    is keyed on \"docs\": the health answers hold \"doctor\" and the paperwork ones hold \"pdf\",\n    \"folder\", \"invoice\". Repairing `plain` alone moves the reading and leaves the reply alone,\n    which is where this line stood. The usual cost of repairing `clean` - a cue list that holds\n    the typed form loses its match - cannot be paid here, because no list holds this one.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("doc"), py_str("docs")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![clean.clone(), words.clone()]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOCS_PAPER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![clean.clone(), words.clone()]);
    }
    if py_truthy(&_docs_on_a_screen(&[words.clone()])) {
    return py_tuple(vec![clean.clone(), words.clone()]);
    }
    places = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("doc"), py_str("docs")]), &word.clone()))) {
    continue;
    }
    near = op_add(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&index.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None), &slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(3)), &Py::None));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&near.clone()) { let mut other: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DOCS_VISIT(), &other.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&places.clone(), "add", &[index.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&places.clone()))) {
    return py_tuple(vec![clean.clone(), words.clone()]);
    }
    let __unpk3 = py_unpack(&py_tuple(vec![py_list(vec![]), Py::Int(0)]), 2, None);
    pieces = __unpk3[0].clone();
    last = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[mcall(&g_RAW_WORD(), "finditer", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    index = __unpk5[0].clone();
    match_ = __unpk5[1].clone();
    let _ = mcall(&pieces.clone(), "append", &[slice_or_raise(&clean.clone(), &last.clone(), &mcall(&match_.clone(), "start", &[]), &Py::None)]);
    let _ = mcall(&pieces.clone(), "append", &[if py_truthy(&Py::Bool(py_contains(&places.clone(), &index.clone()))) { py_str("doctor") } else { mcall(&match_.clone(), "group", &[Py::Int(0)]) }]);
    last = mcall(&match_.clone(), "end", &[]);
    }
    let _ = mcall(&pieces.clone(), "append", &[slice_or_raise(&clean.clone(), &last.clone(), &Py::None, &Py::None)]);
    let __iter6 = py_iter_or_raise(&places.clone());
    for __item in __iter6 {
    index = __item.clone();
    let _ = py_setitem(&words.clone(), &index.clone(), py_str("doctor"));
    }
    return py_tuple(vec![mcall(&py_str(""), "join", &[pieces.clone()]), words.clone()]);
    Py::None
}
pub fn a_sor_bit_is_sore(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut fills: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("\"my throat is sor\" is \"my throat is sore\", and \"i am sor about that\" is left alone.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    token = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("sor")))) {
    before = if py_truthy(&index.clone()) { geti(&tokens.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { Py::None };
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&tokens.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { Py::None };
    fills = { let __b = op_gt(&index.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SORE_FILLERS(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SORE_IN_FRONT(), &geti(&tokens.clone(), &op_sub(&index.clone(), &Py::Int(2))))) } } } };
    if py_truthy(&{ let __b = { let __b = Py::Bool(!py_is(&before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SORE_IN_FRONT(), &before.clone())); if py_truthy(&__b) { __b } else { fills.clone() } } } }; if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&after.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SORE_PARTS(), &after.clone())) } } } }) {
    let _ = mcall(&out.clone(), "append", &[py_str("sore")]);
    continue;
    }
    }
    let _ = mcall(&out.clone(), "append", &[token.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _one_relation_says(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut read: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"my mums got a bad chest\" is \"my mum has got a bad chest\".\n\n    The possessive left out its \"'s\", so the relation is the subject on its own: the \"s\" on it\n    is the \"has\" or the \"is\" of the line and not a plural, and it goes when a verb that takes\n    one person follows it. A verb that takes a \"has\" in front of it gets the \"has\" back, which\n    is the way the line would be spelt out: \"my mums had a fall\" is \"my mum has had a fall\".\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { Py::None };
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { { let __b = after.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ONE_RELATION(), &slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))); if !py_truthy(&__b) { __b } else { _is_possessive(&[geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1)))]) } } } } } } } }) {
    read = _plain_chat(&[after.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_ONE_RELATION_VERBS(), &read.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_HAVE_FORMS(), &read.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BE_FORMS(), &read.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MODAL_FORMS(), &read.clone())) } } } } } }) {
    let _ = mcall(&out.clone(), "append", &[slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    if py_truthy(&Py::Bool(py_contains(&g_HAS_FORMS(), &read.clone()))) {
    let _ = mcall(&out.clone(), "append", &[py_str("has")]);
    }
    continue;
    }
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _ask_start(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut after: Py = Py::None;
    let mut following: Py = Py::None;
    let mut start: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("Where an ask begins inside a clause that did not open with one, or None.\n\n    Only shapes that read as a real question are taken: a question word with an auxiliary\n    or a degree word after it (\"what could be wrong\", \"how long until he sleeps\"), or an\n    auxiliary with a subject after it (\"... can i take ibuprofen\"). A question word that\n    introduces a reported clause (\"i do not know what to do\", \"what to do next\", \"the book\n    which i read\") is left alone, so a feeling is still a feeling.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clause.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WH_WORDS(), &geti(&clause.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AUX_WORDS(), &geti(&clause.clone(), &Py::Int(0)))) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    start = __item.clone();
    token = geti(&clause.clone(), &start.clone());
    following = if py_truthy(&op_lt(&op_add(&start.clone(), &Py::Int(1)), &call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&clause.clone(), &op_add(&start.clone(), &Py::Int(1))) } else { Py::None };
    after = if py_truthy(&op_lt(&op_add(&start.clone(), &Py::Int(2)), &call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&clause.clone(), &op_add(&start.clone(), &Py::Int(2))) } else { Py::None };
    if py_truthy(&Py::Bool(py_is(&following.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_WH_WORDS(), &token.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_AUX_WORDS(), &following.clone())); if py_truthy(&__b) { __b } else { _a_word_between_the_ask_and_its_aux(&[clause.clone(), start.clone()]) } }) {
    if py_truthy(&_reported(&[clause.clone(), start.clone()])) {
    continue;
    }
    return start.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_HOW_DEGREE(), &following.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ASK_NOUNS(), &following.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AUX_WORDS(), &after.clone())) } } } }) {
    if py_truthy(&_reported(&[clause.clone(), start.clone()])) {
    continue;
    }
    return start.clone();
    }
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_AUX_WORDS(), &token.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SUBJECT_WORDS(), &following.clone())) } }) {
    if py_truthy(&Py::Bool(py_contains(&g_NO_ASK_AFTER(), &geti(&clause.clone(), &op_sub(&start.clone(), &Py::Int(1)))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_SUBJECT_WORDS(), &geti(&clause.clone(), &op_sub(&start.clone(), &Py::Int(1)))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_BE_WORDS(), &token.clone()))) {
    continue;
    }
    return start.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_word_between_the_ask_and_its_aux(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut crossed: Py = Py::None;
    let mut offset: Py = Py::None;
    let mut span: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the question word at clause[start] reaches an auxiliary over a few words.\n\n    \"what present would she like\", \"what sort of shoes do i need\", \"what party games would he\n    like\": the words between belong to the question, and the auxiliary that follows asks it. A\n    pronoun, a determiner or a second verb between them means the question word is reported or\n    relative, so that is left alone.\n    ");
    span = slice_or_raise(&clause.clone(), &op_add(&start.clone(), &Py::Int(1)), &op_add(&start.clone(), &Py::Int(6)), &Py::None);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[span.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    offset = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g_AUX_WORDS(), &word.clone()))) {
    crossed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&span.clone(), &Py::None, &offset.clone(), &Py::None)) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_WH_QUANTIFIERS(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("of"), py_str("for")]), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    subject = if py_truthy(&op_lt(&op_add(&op_add(&start.clone(), &Py::Int(2)), &offset.clone()), &call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&clause.clone(), &op_add(&op_add(&start.clone(), &Py::Int(2)), &offset.clone())) } else { Py::None };
    return { let __b = op_le(&call_builtin("len", &[crossed.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[crossed.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_IN_A_QUESTION_PHRASE()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AFTER_AN_INVERTED_AUX(), &subject.clone())) } } } };
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _inverted_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut after: Py = Py::None;
    let _ = py_str("True when the relative word is really the question: \"who should i see\".");
    after = slice_or_raise(&clause.clone(), &op_add(&start.clone(), &Py::Int(1)), &op_add(&start.clone(), &Py::Int(3)), &Py::None);
    return { let __b = op_gt(&call_builtin("len", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_RELATIVE_ASKS(), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SUBJECT_WORDS(), &geti(&after.clone(), &Py::Int(1)))) } } } };
    Py::None
}
pub fn _reported(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut before: Py = Py::None;
    let _ = py_str("True when the question word at clause[start] reports or names rather than asks.");
    before = slice_or_raise(&clause.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&start.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &start.clone(), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_REPORTING())) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_RELATIVE(), &geti(&clause.clone(), &start.clone()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_inverted_ask(&[clause.clone(), start.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_AUX_WORDS(), &geti(&before.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_WH_WORDS(), &geti(&before.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_REPORTING(), &geti(&before.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DETERMINERS(), &geti(&before.clone(), &op_neg(&Py::Int(1))))); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DETERMINERS(), &geti(&before.clone(), &op_neg(&Py::Int(2))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOMINAL_BEFORE_COPULA(), &geti(&before.clone(), &op_neg(&Py::Int(2))))) } } } } } } } } } } } } } } } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(py_contains(&g_COPULAS(), &geti(&before.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOMINAL_BEFORE_COPULA(), &geti(&before.clone(), &op_neg(&Py::Int(2))))) } } } };
    Py::None
}
pub fn _split_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clauses: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clauses"))) };
    let mut clause: Py = Py::None;
    let mut out: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("Clauses with an ask that sits inside one moved out into a clause of its own.\n\n    \"im knackered all the time what could be wrong\" becomes two clauses, so the focus, the\n    kind and every module that reads the ask see the question rather than the news.\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&clauses.clone());
    for __item in __iter1 {
    clause = __item.clone();
    start = _ask_start(&[clause.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&start.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_ge(&start.clone(), &op_sub(&call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) } }) {
    let _ = mcall(&out.clone(), "append", &[clause.clone()]);
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[slice_or_raise(&clause.clone(), &Py::None, &start.clone(), &Py::None)]);
    let _ = mcall(&out.clone(), "append", &[slice_or_raise(&clause.clone(), &start.clone(), &Py::None, &Py::None)]);
    }
    return out.clone();
    Py::None
}
pub fn _opens_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let _ = py_str("Can a clause of its own begin with this word?");
    return { let __b = Py::Bool(py_contains(&g_CLAUSE_OPENERS(), &token.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SUBJECT_WORDS(), &token.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WH_WORDS(), &token.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_BREAKS(), &token.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEADERS(), &token.clone())) } } } } } } } };
    Py::None
}
pub fn _says_has(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("\"she is got a cold\" -> \"she has got a cold\": a pronoun, an \"is\" and a participle.\n\n    \"gone\" counts only with a word after it: \"she is gone to the doctors\" is \"she has gone to\n    the doctors\", while a bare \"she is gone\" is a person who is dead or away, which is a state\n    and stays as it was typed.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(1), op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&geti(&words.clone(), &index.clone()), &py_str("is"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_HAS_SUBJECTS(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } }) {
    continue;
    }
    after = geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_HAS_AFTER(), &after.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&after.clone(), &py_str("gone"))); if !py_truthy(&__b) { __b } else { op_lt(&op_add(&index.clone(), &Py::Int(2)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    let _ = py_setitem(&words.clone(), &index.clone(), py_str("has"));
    }
    }
    return words.clone();
    Py::None
}
pub fn _clauses(args: &[Py]) -> Py {
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
    let mut current: Py = Py::None;
    let mut following: Py = Py::None;
    let mut n: Py = Py::None;
    let mut out: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("Split a run-on line into clauses at punctuation and discourse markers.\n\n    A marker only opens a clause when a clause can begin after it. \"my landlord still has\n    not fixed the boiler\" and \"i wanted to go but could not afford it\" keep it: there the\n    marker is part of the clause, and cutting it off left \"has not fixed the boiler\" and\n    \"could not afford it\", which read as yes/no questions.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    out = __unpk1[0].clone();
    current = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    n = __unpk3[0].clone();
    token = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&token.clone(), &py_str("|")))) {
    if py_truthy(&current.clone()) {
    let _ = mcall(&out.clone(), "append", &[current.clone()]);
    current = py_list(vec![]);
    }
    continue;
    }
    following = if py_truthy(&op_lt(&op_add(&n.clone(), &Py::Int(1)), &call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&tokens.clone(), &op_add(&n.clone(), &Py::Int(1))) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_BREAKS(), &token.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[current.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = following.clone(); if !py_truthy(&__b) { __b } else { _opens_clause(&[following.clone()]) } } } } } }) {
    let _ = mcall(&out.clone(), "append", &[current.clone()]);
    current = py_list(vec![]);
    continue;
    }
    let _ = mcall(&current.clone(), "append", &[token.clone()]);
    }
    if py_truthy(&current.clone()) {
    let _ = mcall(&out.clone(), "append", &[current.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _a_leading_like_is_a_filler(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut head: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("\"like my cat wont eat\" is about the cat, so the \"like\" is not part of the line.\n\n    People open a line with \"like\" the way they open it with \"so\" or \"well\", and the word is a\n    real one, so it was read as a comparison and the line lost its answer: \"like my cat wont\n    eat\" was noted back and \"like my broadband keeps dropping out\" got the warm line where \"my\n    broadband keeps dropping out\" gets the router answer. Only a clause opener after it makes\n    it the filler, which is what keeps a comparison out of it.\n    ");
    let __unpk1 = py_unpack(&mcall(&clean.clone(), "partition", &[py_str(" ")]), 3, None);
    head = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    rest = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&head.clone(), "strip", &[py_str(",.!?")]), &py_str("like"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&rest.clone())) } }) {
    return clean.clone();
    }
    after = mcall(&geti(&mcall(&rest.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(0)), "strip", &[py_str(",.!?")]);
    if py_truthy(&Py::Bool(!py_contains(&g_LIKE_STARTS_A_CLAUSE(), &after.clone()))) {
    return clean.clone();
    }
    return rest.clone();
    Py::None
}
pub fn without_a_leading_exclamation(args: &[Py]) -> Py {
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
    let _ = py_str("The line without the exasperation it opens on: \"god i hate mondays\" is \"i hate mondays\".\n\n    Public because the composer says a line back in its own words and works from the line as it\n    was typed, not from `clean`: \"god i hate mondays\" came back as \"Noted: god you hate mondays.\"\n    ");
    return _an_exclamation_in_front(&[clean.clone()]);
    Py::None
}
pub fn _an_exclamation_in_front(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut head: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The line without the exasperation it opens on: \"god i hate mondays\" is \"i hate mondays\".");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    phrase = mcall(&g_EXCLAMATION_PHRASE(), "match", &[clean.clone()]);
    if py_truthy(&phrase.clone()) {
    rest = slice_or_raise(&clean.clone(), &mcall(&phrase.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__EXCLAMATION_SUBJECTS(), &mcall(&geti(&mcall(&rest.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(0)), "strip", &[py_str(",.!?;:-|")]))) } }) {
    clean = rest.clone();
    continue;
    }
    return clean.clone();
    }
    let __unpk2 = py_unpack(&mcall(&clean.clone(), "partition", &[py_str(" ")]), 3, None);
    head = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    rest = __unpk2[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_EXCLAMATION_OPENS(), &mcall(&head.clone(), "strip", &[py_str(",.!?;:-|")]))) } }) {
    return clean.clone();
    }
    after = mcall(&geti(&mcall(&rest.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(0)), "strip", &[py_str(",.!?;:-|")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__EXCLAMATION_SUBJECTS(), &after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_EXCLAMATION_OPENS(), &after.clone())) } }) {
    clean = rest.clone();
    continue;
    }
    return clean.clone();
    }
    return clean.clone();
    Py::None
}
pub fn _an_exclamation_is_not_a_thing(args: &[Py]) -> Py {
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
    let _ = py_str("The line without the surprise after a question word: \"what on earth\" is \"what\".\n\n    \"wtf\" is expanded to \"what\" (lexicon.CHAT_SPEAK), which is the same repair from the other\n    side: the question about the boiler, not a warning about the planet. This one holds for the\n    phrase typed out - \"what on earth do i do about my boiler\".\n    ");
    return mcall(&g_EXCLAIMED_AFTER_A_QUESTION(), "sub", &[py_str("\\1"), clean.clone()]);
    Py::None
}
pub fn strip_leaders(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut start: Py = Py::None;
    let _ = py_str("A clause without the filler it opens with.");
    start = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&start.clone(), &op_sub(&call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LEADERS(), &geti(&clause.clone(), &start.clone()))) } }) {
    let __aug1 = op_add(&start.clone(), &Py::Int(1));
    start = __aug1;
    }
    return slice_or_raise(&clause.clone(), &start.clone(), &Py::None, &Py::None);
    Py::None
}
pub fn without_leaders(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The line without the filler it opens with: \"oh and my son is 9\" -> \"my son is 9\".");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    while py_truthy(&Py::Bool(true)) {
    match_ = mcall(&g__LEADER_WORD(), "match", &[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&match_.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_LEADERS(), &mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]))) } }) {
    return said.clone();
    }
    rest = slice_or_raise(&said.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&rest.clone(), "strip", &[])))) {
    return said.clone();
    }
    said = rest.clone();
    }
    Py::None
}
pub fn _clause_stop(args: &[Py]) -> Py {
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
    let mut stop: Py = Py::None;
    let _ = py_str("The position just past the clause that opens at `index`.\n\n    A clause ends where the next one may open: at a conjunction, or at a question of its own.\n    \"it started last summer after a picnic, what should i do\" gives the picnic in the background\n    and leaves the ask alone.\n    ");
    stop = op_add(&index.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = op_lt(&stop.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CLAUSE_BREAKS"), &geti(&norm.clone(), &stop.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__WH_QUESTIONS(), &geti(&norm.clone(), &stop.clone()))) } } } }) {
    let __aug1 = op_add(&stop.clone(), &Py::Int(1));
    stop = __aug1;
    }
    return stop.clone();
    Py::None
}
pub fn background_clause(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The positions a clause saying WHEN or WHERE something happened takes up, or an empty set.\n\n    A line that says when or where it happened gives that time or place in a clause of its own:\n    \"it started this morning when i was in the garden\" is about whatever started. Only a clause\n    with the line's own words before it counts; a line that opens on the subordinator asks about\n    it (\"when should i plant tomatoes\"), a reporting verb in front of it makes it the thing being\n    asked about (\"no i mean where did i move from\"), and a line with a subject of its own keeps\n    its own clauses (\"the garden flooded when it rained\"). The clause stops where the next one\n    opens, so it never covers an ask of its own (\"my feet smell even after washing what helps\").\n    ");
    norm = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&index.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.lexicon", "BACKGROUND_STARTS"), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_REPORTING(), &geti(&norm.clone(), &op_sub(&index.clone(), &Py::Int(1)))))) {
    continue;
    }
    return call_builtin("set", &[call_builtin("range", &[index.clone(), _clause_stop(&[norm.clone(), index.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reason_positions(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The positions a clause giving the reason for the line takes up, or an empty set.\n\n    Only a clause with the line's own words before it counts, so a line that opens on the\n    subordinator (\"since when do i need a licence\") keeps its own reading.\n    ");
    norm = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&index.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_REASON_STARTS(), &word.clone())) } }) {
    continue;
    }
    return call_builtin("set", &[call_builtin("range", &[index.clone(), _clause_stop(&[norm.clone(), index.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn background_positions(args: &[Py]) -> Py {
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
    let mut anywhere: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut index: Py = Py::None;
    let mut skip: Py = Py::None;
    let mut spans: Py = Py::None;
    let mut start: Py = Py::None;
    let mut stop: Py = Py::None;
    let mut taken: Py = Py::None;
    let _ = py_str("The positions in this line taken by words that only say WHERE it happened.\n\n    Those words are not what the line is about: \"it started at work in the garden centre\" is\n    about whatever started, not about the garden, and \"it started this morning when i was in the\n    garden\" says the same with the place inside a when-clause. Only a place phrase counts - a\n    preposition and the thing it introduces - so the rest of the clause keeps its meaning:\n    \"i get a rash when i eat strawberries\" is a health line, and \"how do i stop shaking before a\n    talk\" is about the talk.\n    ");
    norm = call_builtin("list", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e));
    skip = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    spans = py_list(vec![]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&slice_or_raise(&norm.clone(), &Py::None, &Py::Int(1), &Py::None), &py_list(vec![py_str("it")]))); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "START_VERBS"), &geti(&norm.clone(), &Py::Int(1)))) } } } }) {
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![Py::Int(0), _clause_stop(&[norm.clone(), Py::Int(1)]), Py::Bool(true)])]);
    }
    clause = background_clause(&[norm.clone()]);
    if py_truthy(&clause.clone()) {
    let _ = mcall(&spans.clone(), "append", &[py_tuple(vec![call_builtin("min", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), op_add(&call_builtin("max", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), Py::Bool(false)])]);
    }
    let __iter1 = py_iter_or_raise(&spans.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    start = __unpk2[0].clone();
    stop = __unpk2[1].clone();
    anywhere = __unpk2[2].clone();
    index = start.clone();
    while py_truthy(&op_lt(&index.clone(), &stop.clone())) {
    if py_truthy(&Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.lexicon", "PLACE_PREPS"), &geti(&norm.clone(), &index.clone())))) {
    let __aug3 = op_add(&index.clone(), &Py::Int(1));
    index = __aug3;
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&anywhere.clone())); if !py_truthy(&__b) { __b } else { { let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&g__PLACE_LEADS(), &geti(&norm.clone(), &op_sub(&index.clone(), &Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&skip.clone(), &op_sub(&index.clone(), &Py::Int(1)))) } })) } } } }) {
    let __aug4 = op_add(&index.clone(), &Py::Int(1));
    index = __aug4;
    continue;
    }
    let __aug5 = op_add(&index.clone(), &Py::Int(1));
    index = __aug5;
    taken = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&index.clone(), &stop.clone()); if !py_truthy(&__b) { __b } else { { let __b = op_lt(&taken.clone(), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.lexicon", "CLAUSE_BREAKS"), &geti(&norm.clone(), &index.clone()))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__PLACE_STOP(), &geti(&norm.clone(), &index.clone()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.lexicon", "PLACE_PREPS"), &geti(&norm.clone(), &index.clone()))) } } } } } } } }) {
    let _ = mcall(&skip.clone(), "add", &[index.clone()]);
    let __aug6 = op_add(&index.clone(), &Py::Int(1));
    index = __aug6;
    let __aug7 = op_add(&taken.clone(), &Py::Int(1));
    taken = __aug7;
    }
    }
    }
    return skip.clone();
    Py::None
}
pub fn _the_measure_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut stripped: Py = Py::None;
    let _ = py_str("True for a clause that is only a measure word, with or without its noun.\n\n    \"how long\", \"how much water\", \"how many hours sleep\": the second word is the measure, and\n    whatever follows it inside the clause is the thing measured, so the whole clause belongs to\n    the ask that comes after it. A leading \"and\" or \"so\" is a leader, not part of the measure:\n    \"and how long do they stay on\" is the same ask as \"how long do they stay on\".\n    ");
    stripped = strip_leaders(&[clause.clone()]);
    return { let __b = op_gt(&call_builtin("len", &[stripped.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&stripped.clone(), &Py::Int(0)), &py_str("how"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HOW_DEGREE(), &geti(&stripped.clone(), &Py::Int(1)))) } } } };
    Py::None
}
pub fn _asking(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clauses: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clauses"))) };
    let mut focus: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "focus") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: focus"))) };
    let mut at: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut index: Py = Py::None;
    let _ = py_str("The clause that asks, with the measure word it was said with.\n\n    `focus` is the clause carrying the ask, and for \"how long do they stay on\" that is \"do they\n    stay on\": the measure word sits in the clause before it. A reader that reads the ask - the\n    shape of the answer, or the thing the question attaches to - needs the word that says which\n    measure is wanted, or a how-long reads as a yes/no. So the measure clause in front of the\n    focus is joined to it here, once, rather than by every reader of the ask.\n\n    Nothing else is added: a line with no measure word gets its focus back unchanged.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&focus.clone()))) {
    return py_list(vec![]);
    }
    at = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    clause = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_is(&clause.clone(), &focus.clone()))) {
    at = index.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&at.clone(), &Py::None))) {
    let __iter3 = py_iter_or_raise(&call_builtin("enumerate", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    index = __unpk4[0].clone();
    clause = __unpk4[1].clone();
    if py_truthy(&Py::Bool(py_eq(&clause.clone(), &focus.clone()))) {
    at = index.clone();
    break;
    }
    }
    }
    if py_truthy(&{ let __b = at.clone(); if !py_truthy(&__b) { __b } else { _the_measure_clause(&[geti(&clauses.clone(), &op_sub(&at.clone(), &Py::Int(1)))]) } }) {
    return op_add(&call_builtin("list", &[geti(&clauses.clone(), &op_sub(&at.clone(), &Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[focus.clone()]).unwrap_or_else(|e| exc::raise(e)));
    }
    return call_builtin("list", &[focus.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _focus(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clauses: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clauses"))) };
    let mut clause: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut token: Py = Py::None;
    let _ = py_str("The clause that carries the ask: the last one that opens like a question, else the\n    last one that asks for something, else the last one with something in it.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    clause = __item.clone();
    stripped = strip_leaders(&[clause.clone()]);
    if py_truthy(&{ let __b = stripped.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_WORDS(), &geti(&stripped.clone(), &Py::Int(0)))) } }) {
    return clause.clone();
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    clause = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clause.clone()) { let mut token: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_WANT_WORDS(), &token.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return clause.clone();
    }
    }
    let __iter3 = py_iter_or_raise(&call_builtin("reversed", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    clause = __item.clone();
    stripped = strip_leaders(&[clause.clone()]);
    if py_truthy(&stripped.clone()) {
    return clause.clone();
    }
    }
    return if py_truthy(&clauses.clone()) { geti(&clauses.clone(), &op_neg(&Py::Int(1))) } else { py_list(vec![]) };
    Py::None
}
pub fn _unaccent(args: &[Py]) -> Py {
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
    let mut char: Py = Py::None;
    let mut split: Py = Py::None;
    let _ = py_str("The same line with every accent mark taken off (\"crème\" -> \"creme\").\n\n    A keyboard often cannot type the accent, and one that arrives by copy or by a foreign\n    keyboard used to cut the word in two at the accent (\"creme fraiche\" typed with accents\n    read as \"cr me fra che\"), so the word matched nothing and the line read as a shrug.\n    NFKD splits a letter from its accent and the combining marks are dropped, for every\n    script, not just the words we happen to know.\n    ");
    split = stdlib_call("unicodedata", "normalize", &[py_str("NFKD"), mcall(&text.clone(), "translate", &[g_LETTERS()])]).unwrap_or_else(|e| exc::raise(e));
    return mcall(&py_str(""), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&split.clone()) { let mut char: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("unicodedata", "combining", &[char.clone()]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(char.clone()); } } py_list(__out) })() }]);
    Py::None
}
pub fn intensifier_positions(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut positions: Py = Py::None;
    let mut size: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the \"on earth\" / \"the hell\" in the line sits, as indices into `norm`.\n\n    The noun in the phrase is not a thing the line is about, so the reading gives its place to\n    the caller that keeps the words nothing answers on (`background`). Only the phrase right\n    after a question word counts, so \"life on earth\" and \"the biggest thing on earth\" keep\n    theirs.\n    ");
    positions = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_WH_ANY(), &word.clone()))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&py_tuple(vec![Py::Int(3), Py::Int(2)]));
    for __item in __iter3 {
    size = __item.clone();
    phrase = slice_or_raise(&norm.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&op_add(&index.clone(), &Py::Int(1)), &size.clone()), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &size.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_WH_INTENSIFIER(), "match", &[mcall(&py_str(" "), "join", &[phrase.clone()])]) } }) {
    let _ = mcall(&positions.clone(), "update", &[call_builtin("range", &[op_add(&index.clone(), &Py::Int(1)), op_add(&op_add(&index.clone(), &Py::Int(1)), &size.clone())]).unwrap_or_else(|e| exc::raise(e))]);
    break;
    }
    }
    }
    return positions.clone();
    Py::None
}
pub fn _the_surprise_typed(args: &[Py]) -> Py {
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
    let mut flat: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words of the surprise after a question word, as the line was typed.\n\n    \"what on earth is happening\" and \"what on earth's going on\" both hand over \"on\" and \"earth\",\n    with the clipped is taken off: the reading no longer holds the phrase, and a caller that\n    wants to know which surprise the line used has nothing else to ask.\n    ");
    flat = stdlib_call("re", "sub", &[py_str("[^a-z']+"), py_str(" "), mcall(&mcall(&text.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]).unwrap_or_else(|e| exc::raise(e));
    flat = stdlib_call("re", "sub", &[py_str("\\b(?:wtf|wth|wtaf)\\b"), py_str("what on earth"), flat.clone()]).unwrap_or_else(|e| exc::raise(e));
    words = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g__TYPED_SURPRISE(), "finditer", &[flat.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    let __iter2 = py_iter_or_raise(&slice_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "split", &[]), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter2 {
    word = __item.clone();
    word = if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("'s")])) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None) } else { word.clone() };
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &word.clone()))) {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    }
    }
    return words.clone();
    Py::None
}
pub fn _without_the_intensifier(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reading with \"on earth\" / \"the hell\" taken out from after a question word.\n\n    The words are not what the line is about, so every reader that matches English words gets\n    the line without them: \"what on earth is happening\" is a question about the person's day,\n    not about the planet.\n    ");
    out = py_list(vec![]);
    index = Py::Int(0);
    while py_truthy(&op_lt(&index.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    word = geti(&norm.clone(), &index.clone());
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_WH_ANY(), &word.clone())); if !py_truthy(&__b) { __b } else { op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    phrases = intensifier_positions(&[norm.clone()]);
    if py_truthy(&Py::Bool(py_contains(&phrases.clone(), &op_add(&index.clone(), &Py::Int(1))))) {
    end = op_add(&index.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = op_lt(&end.clone(), &call_builtin("len", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&phrases.clone(), &end.clone())) } }) {
    let __aug1 = op_add(&end.clone(), &Py::Int(1));
    end = __aug1;
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    index = end.clone();
    continue;
    }
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    let __aug2 = op_add(&index.clone(), &Py::Int(1));
    index = __aug2;
    }
    return out.clone();
    Py::None
}
pub fn _a_clip_kept_only_with_its_cue(args: &[Py]) -> Py {
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
    let mut norm: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "norm") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: norm"))) };
    let mut fixes: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fixes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fixes"))) };
    let mut line: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut bent: Py = Py::None;
    let mut cues: Py = Py::None;
    let mut longer: Py = Py::None;
    let mut pair: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reading with a clip's repair taken back when the line names nothing it is about.\n\n    The vowel class repairs \"mon\" to \"moon\" wherever it stands, and a line about a parent loses\n    its subject to it. The repair is kept when any word of the line is a cue for the long word,\n    and taken back - out of the plain line, the reading and the list of repairs - when none is.\n    ");
    bent = call_builtin("dict", &[fixes.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g__A_CLIP(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    typed = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    longer = __unpk3[0].clone();
    cues = __unpk3[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&bent.clone(), "get", &[typed.clone()]), &longer.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&line.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&cues.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    fixes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fixes.clone()) { let mut pair: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&geti(&pair.clone(), &Py::Int(0)), &typed.clone()))) {  __out.push(pair.clone()); } } py_list(__out) })() };
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&word.clone(), &longer.clone()))) { typed.clone() } else { word.clone() }); } py_list(__out) })() };
    norm = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_eq(&word.clone(), &longer.clone()))) { typed.clone() } else { word.clone() }); } py_list(__out) })() };
    }
    return py_tuple(vec![plain.clone(), norm.clone(), fixes.clone()]);
    Py::None
}
pub fn _fix_auxiliaries(args: &[Py]) -> Py {
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
    let mut before_it: Py = Py::None;
    let mut index: Py = Py::None;
    let mut last: Py = Py::None;
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A \"bin\" read as \"been\" gets the helper English needs: \"he is been\" is \"he has been\",\n    \"she been in hospital\" is \"she has been in hospital\". Nothing else is touched: a \"been\"\n    with a helper already (\"has been\", \"have been\") is left exactly as it is.");
    if py_truthy(&Py::Bool(!py_contains(&tokens.clone(), &py_str("been")))) {
    return tokens.clone();
    }
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("been"))); if !py_truthy(&__b) { __b } else { out.clone() } }) {
    last = geti(&out.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_contains(&g_BE_FORMS(), &last.clone()))) {
    let _ = py_setitem(&out.clone(), &op_neg(&Py::Int(1)), if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[out.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLURAL_SUBJECTS(), &geti(&out.clone(), &op_neg(&Py::Int(2))))) } }) { py_str("have") } else { py_str("has") });
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLURAL_SUBJECTS(), &last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THIRD_PERSON(), &last.clone())) } }) {
    before_it = if py_truthy(&op_gt(&call_builtin("len", &[out.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&out.clone(), &op_neg(&Py::Int(2))) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_HAVE_FORMS(), &before_it.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_BE_FORMS(), &before_it.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_MODAL_FORMS(), &before_it.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&before_it.clone(), &py_str("not"))) } } } } } }) {
    let _ = mcall(&out.clone(), "append", &[if py_truthy(&Py::Bool(py_contains(&g_PLURAL_SUBJECTS(), &last.clone()))) { py_str("have") } else { py_str("has") }]);
    }
    }
    }
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _change_of_subject_off(args: &[Py]) -> Py {
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
    let mut rest: Py = Py::None;
    let mut seen: Py = Py::None;
    let _ = py_str("The line without the clause that only announces a change of subject, and whether it went.");
    seen = mcall(&g_CHANGE_OF_SUBJECT(), "match", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&seen.clone()))) {
    return py_tuple(vec![clean.clone(), Py::Bool(false)]);
    }
    rest = slice_or_raise(&clean.clone(), &mcall(&seen.clone(), "end", &[]), &Py::None, &Py::None);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&g_RAW_WORD(), "findall", &[rest.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_tuple(vec![clean.clone(), Py::Bool(false)]);
    }
    return py_tuple(vec![rest.clone(), Py::Bool(true)]);
    Py::None
}
pub fn _without_a_tag_at_the_end(args: &[Py]) -> Py {
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
    let mut asks: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("`clean` without the tags at its end, and the tags that came off.");
    words = mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    asks = { let __b = call_builtin("bool", &[{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TAG_QUESTION_LEADS(), &mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(".,!?")]))) } }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } };
    dropped = py_list(vec![]);
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TAG_WORDS(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?")]))) } }) {
    if py_truthy(&Py::Bool(py_contains(&g_TAG_GUARD_BEFORE(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(2))), "strip", &[py_str(".,!?")])))) {
    break;
    }
    let _ = mcall(&dropped.clone(), "insert", &[Py::Int(0), mcall(&words.clone(), "pop", &[])]);
    }
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TAG_PLEASE(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?")]))); if !py_truthy(&__b) { __b } else { asks.clone() } } } }) {
    let _ = mcall(&dropped.clone(), "insert", &[Py::Int(0), mcall(&words.clone(), "pop", &[])]);
    }
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "LAUGHTER"), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?")]))); if !py_truthy(&__b) { __b } else { asks.clone() } } } }) {
    let _ = mcall(&dropped.clone(), "insert", &[Py::Int(0), mcall(&words.clone(), "pop", &[])]);
    }
    while py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TAG_THANKS(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?")]))); if !py_truthy(&__b) { __b } else { asks.clone() } } } }) {
    let _ = mcall(&dropped.clone(), "insert", &[Py::Int(0), mcall(&words.clone(), "pop", &[])]);
    }
    return py_tuple(vec![mcall(&py_str(" "), "join", &[words.clone()]), dropped.clone()]);
    Py::None
}
pub fn _spellings_of_one_word(args: &[Py]) -> Py {
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
    let mut longer: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("True when these two words are spellings or forms of the same word.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&other.clone())) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&one.clone(), &other.clone()))) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&if py_truthy(&op_le(&call_builtin("len", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![one.clone(), other.clone()]) } else { py_tuple(vec![other.clone(), one.clone()]) }, 2, None);
    short = __unpk1[0].clone();
    longer = __unpk1[1].clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&longer.clone(), "startswith", &[short.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_contains(&g_SPELLING_ENDINGS(), &slice_or_raise(&longer.clone(), &call_builtin("len", &[short.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)));
    Py::None
}
pub fn _names_a_letter_beside_a_slip(args: &[Py]) -> Py {
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
    let mut fixed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fixed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fixed"))) };
    let mut at: Py = Py::None;
    let mut near: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names a letter a few words from a slip it repaired.");
    if py_truthy(&Py::Bool(!py_truthy(&fixed.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&fixed.clone(), &word.clone()))) {
    continue;
    }
    near = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&at.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e)), &op_add(&at.clone(), &Py::Int(5)), &Py::None)]);
    if py_truthy(&mcall(&g_NAMED_LETTER(), "search", &[near.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _a_short_word_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut short: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "short") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: short"))) };
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("True when a short chat form is being named, not said: \"a u turn\", \"the letter u\".\n\n    \"u\" is both the chat form of \"you\" and a letter with its own words around it, and the two\n    are told apart by what stands beside it: \"do a u turn\", \"the letter u is silent\", \"a u-boat\",\n    \"what u's do i need for the exam\". Everything else is the word: \"how r u\", \"what are u\n    reading\", \"how do i write a thank u letter\".\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_SHORT_WORD_IS_NAMED_BEFORE(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_SHORT_WORD_IS_NAMED_AFTER(), &after.clone())) } }) {
    return Py::Bool(true);
    }
    return { let __b = Py::Bool(!py_is(&after.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&after.clone(), "endswith", &[py_str("'s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&short.clone(), &py_str("u"))) } } } };
    Py::None
}
pub fn _asks_the_spelling(args: &[Py]) -> Py {
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
    let mut fixed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fixed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fixed"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the typed spelling is what the line is asking about, so it is not put right.\n\n    Two shapes carry it, and both name the slip as the subject:\n\n    - the question is about spelling at all: \"how do you spell recieve\", \"how do i spell\n      embarased\". The spelling answers are reached from the word as it was typed, so a repaired\n      word is a word they can no longer see.\n    - the line offers two spellings of one word: \"is it definately or definitely\", \"which is\n      right, seperate or separate\". Only a repair with the other spelling beside it counts, so\n      \"im tiered or just bored\" is still read as tired.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&fixed.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SPELLING_ASK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&_names_a_letter_beside_a_slip(&[words.clone(), fixed.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("or")))) {
    continue;
    }
    before = if py_truthy(&at.clone()) { geti(&words.clone(), &op_sub(&at.clone(), &Py::Int(1))) } else { py_str("") };
    after = if py_truthy(&op_lt(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&words.clone(), &op_add(&at.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&fixed.clone(), &before.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![geti(&fixed.clone(), &before.clone()), before.clone()]), &after.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&fixed.clone(), &after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![geti(&fixed.clone(), &after.clone()), after.clone()]), &before.clone())) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&fixed.clone(), &before.clone())); if !py_truthy(&__b) { __b } else { _spellings_of_one_word(&[geti(&fixed.clone(), &before.clone()), after.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&fixed.clone(), &after.clone())); if !py_truthy(&__b) { __b } else { _spellings_of_one_word(&[geti(&fixed.clone(), &after.clone()), before.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _can_not_as_one_word(args: &[Py]) -> Py {
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
    let _ = py_str("The line with a \"can not\" typed as two words written as the one word \"cant\".");
    return mcall(&g_CAN_NOT(), "sub", &[py_str("cant"), clean.clone()]);
    Py::None
}
pub fn _the_day_left_out(args: &[Py]) -> Py {
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
    let mut day: Py = Py::None;
    let mut found: Py = Py::None;
    let mut last: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The same line with the day's name put back in the clause that only names its owner.");
    named = mcall(&g_DAY_NAMED(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return text.clone();
    }
    day = mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    pieces = py_list(vec![]);
    last = Py::Int(0);
    let __iter1 = py_iter_or_raise(&mcall(&g_OWNER_WITHOUT_THE_DAY(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&op_lt(&mcall(&found.clone(), "start", &[]), &mcall(&named.clone(), "end", &[]))) {
    continue;
    }
    tail = mcall(&geti(&stdlib_call("re", "split", &[py_str("[,.;:!?]"), slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "split", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&g_TAIL_OF_A_DAY(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    let _ = mcall(&pieces.clone(), "append", &[slice_or_raise(&text.clone(), &last.clone(), &mcall(&found.clone(), "end", &[]), &Py::None)]);
    let _ = mcall(&pieces.clone(), "append", &[op_add(&py_str(" "), &day.clone())]);
    last = mcall(&found.clone(), "end", &[]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&pieces.clone()))) {
    return text.clone();
    }
    let _ = mcall(&pieces.clone(), "append", &[slice_or_raise(&text.clone(), &last.clone(), &Py::None, &Py::None)]);
    return mcall(&py_str(""), "join", &[pieces.clone()]);
    Py::None
}
pub fn _a_day_named_as_not_mine(args: &[Py]) -> Py {
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
    let mut day: Py = Py::None;
    let mut found: Py = Py::None;
    let mut negated: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The same line with the day moved onto the person the clause before it named.");
    negated = mcall(&g_NOT_MY_OWN_DAY(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&negated.clone()))) {
    return text.clone();
    }
    day = mcall(&mcall(&negated.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    owner = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_A_NUMBER_OF_SOMEONES_DAY(), "finditer", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&negated.clone(), "start", &[]), &Py::None)]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_KIN(), &mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), "rstrip", &[py_str("s")])))) {
    owner = found.clone();
    }
    }
    if py_truthy(&Py::Bool(py_is(&owner.clone(), &Py::None))) {
    return text.clone();
    }
    whole = op_add(&op_add(&op_add(&op_add(&slice_or_raise(&text.clone(), &Py::None, &mcall(&owner.clone(), "end", &[]), &Py::None), &py_str(" ")), &day.clone()), &slice_or_raise(&text.clone(), &mcall(&owner.clone(), "end", &[]), &mcall(&negated.clone(), "start", &[]), &Py::None)), &slice_or_raise(&text.clone(), &mcall(&negated.clone(), "end", &[]), &Py::None, &Py::None));
    return mcall(&py_str(" "), "join", &[mcall(&whole.clone(), "split", &[])]);
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
    let _ = py_str("True for \"3\", \"2nd\", \"two\", \"a few\" - the number in front of the day words.");
    return { let __b = call_builtin("bool", &[mcall(&g__NUMBER_SHAPE(), "match", &[word.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__SMALL_NUMBERS(), &word.clone())) } };
    Py::None
}
pub fn _kept_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut flat: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "flat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: flat"))) };
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut index: Py = Py::None;
    let mut place: Py = Py::None;
    let mut time: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("{\"place\", \"time\"} when the line says a thing has already been kept, else None.\n\n    The place is the word the thing sat in (\"fridge\", \"cupboard\", \"out\") or None when the line\n    names only the thing that was kept (\"leftover rice after 3 days\"). The time is the phrase\n    that says how long (\"3 days\", \"overnight\", \"since monday\"). A line with a time and no place\n    and no kept thing is not this reading: \"i start my new job in 3 days\" is a date.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&flat.clone()))) {
    return Py::None;
    }
    time = Py::None;
    at = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[flat.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[flat.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { { let __b = _a_number(&[word.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__DAY_WORDS(), &geti(&flat.clone(), &op_add(&index.clone(), &Py::Int(1))))) } } } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![op_modulo(&py_str("%s %s"), &py_tuple(vec![word.clone(), geti(&flat.clone(), &op_add(&index.clone(), &Py::Int(1)))])), index.clone()]), 2, None);
    time = __unpk3[0].clone();
    at = __unpk3[1].clone();
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g__KEPT_ALREADY(), &word.clone()))) {
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[flat.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&flat.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { py_str("") };
    time = if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("since"))); if !py_truthy(&__b) { __b } else { after.clone() } }) { mcall(&op_add(&op_add(&word.clone(), &py_str(" ")), &after.clone()), "strip", &[]) } else { word.clone() };
    at = index.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&time.clone(), &Py::None))) {
    return Py::None;
    }
    place = Py::None;
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[flat.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    index = __unpk5[0].clone();
    word = __unpk5[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g__KEEPING_PLACES(), &word.clone()))) {
    place = word.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_is(&place.clone(), &Py::None))) {
    if py_truthy(&op_le(&call_builtin("abs", &[op_sub(&at.clone(), &index.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10))) {
    return py_dict(vec![(py_str("place"), place.clone()), (py_str("time"), time.clone())]);
    }
    return Py::None;
    }
    if py_truthy(&mcall(&g__KEPT_THINGS(), "intersection", &[flat.clone()])) {
    return py_dict(vec![(py_str("place"), Py::None), (py_str("time"), time.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _only_doubles_a_letter(args: &[Py]) -> Py {
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
    let mut mended: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mended") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mended"))) };
    let mut at: Py = Py::None;
    let _ = py_str("True when the repair puts back a copy of a letter the person typed once.\n\n    People do type one of a doubled letter (\"recomend\", \"hapened\") and the corrector puts it\n    back, but the same guess is how a word becomes a different word: \"mons\" was read as \"moons\",\n    \"thres\" as \"threes\" and \"moning\" as \"mooning\". A vowel put back where the line has three\n    consonants in a row (\"wndow\", \"kittn\", \"brdge\") is a shape no English word has, so that\n    repair is one the reader can be sure of; a doubled letter is a letter the person may never\n    have meant, so it is left to `plain` and the word the readers match on stays as it was typed\n    unless a misspelling list vouches for it.\n    ");
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[mended.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[typed.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[mended.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut at: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&geti(&mended.clone(), &at.clone()), &geti(&mended.clone(), &op_add(&at.clone(), &Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&op_add(&slice_or_raise(&mended.clone(), &Py::None, &at.clone(), &Py::None), &slice_or_raise(&mended.clone(), &op_add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None)), &typed.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn normalise(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut after_after: Py = Py::None;
    let mut after_words: Py = Py::None;
    let mut asking: Py = Py::None;
    let mut before: Py = Py::None;
    let mut before_before: Py = Py::None;
    let mut before_words: Py = Py::None;
    let mut change_of_subject: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut fixes: Py = Py::None;
    let mut flat: Py = Py::None;
    let mut focus: Py = Py::None;
    let mut index: Py = Py::None;
    let mut intensifier: Py = Py::None;
    let mut intensifier_left: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut keep: Py = Py::None;
    let mut meant: Py = Py::None;
    let mut mended: Py = Py::None;
    let mut mended_word: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence_ends: Py = Py::None;
    let mut settled: Py = Py::None;
    let mut signals: Py = Py::None;
    let mut slips: Py = Py::None;
    let mut spelled: Py = Py::None;
    let mut subject_put_back: Py = Py::None;
    let mut tags_at_the_end: Py = Py::None;
    let mut tokens: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("{\"text\", \"clean\", \"words\", \"plain\", \"norm\", \"norm_text\", \"fixes\", \"clauses\", \"focus\",\n    \"asking\", \"signals\"} for one line.\n\n    \"plain\" is the line as it was typed with each word repaired in place - the same order, the\n    same punctuation, no clause splitting - so a reader that matches on words gets the spelling\n    the writer meant without losing anything else. \"norm\" is the fuller reading: clauses split,\n    chat-speak and contractions expanded, digits read out.\n    ");
    text = if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { text.clone() } else { py_str("") };
    clean = _glued_said_apart(&[mcall(&py_str(" "), "join", &[mcall(&_unaccent(&[text.clone()]), "split", &[])])]);
    clean = _typed_without_spaces(&[clean.clone()]);
    clean = _said_apart_as_one_word(&[clean.clone()]);
    clean = _the_mrs_is_the_wife(&[clean.clone()]);
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&clean.clone(), "lower", &[]), "split", &[])]);
    clean = _can_not_as_one_word(&[clean.clone()]);
    clean = _the_day_left_out(&[clean.clone()]);
    clean = _a_day_named_as_not_mine(&[clean.clone()]);
    clean = mcall(&g_THOUSANDS(), "sub", &[py_str("\\1"), clean.clone()]);
    clean = mcall(&g_THOUSANDS_SAID_WITH_A_SPACE(), "sub", &[{ let after = after.clone(); let after_after = after_after.clone(); let after_words = after_words.clone(); let asking = asking.clone(); let before = before.clone(); let before_before = before_before.clone(); let before_words = before_words.clone(); let change_of_subject = change_of_subject.clone(); let clause = clause.clone(); let clauses = clauses.clone(); let clean = clean.clone(); let fixed = fixed.clone(); let fixes = fixes.clone(); let flat = flat.clone(); let focus = focus.clone(); let index = index.clone(); let intensifier = intensifier.clone(); let intensifier_left = intensifier_left.clone(); let joined = joined.clone(); let keep = keep.clone(); let meant = meant.clone(); let mended = mended.clone(); let mended_word = mended_word.clone(); let norm = norm.clone(); let parts = parts.clone(); let plain = plain.clone(); let said = said.clone(); let sentence_ends = sentence_ends.clone(); let settled = settled.clone(); let signals = signals.clone(); let slips = slips.clone(); let spelled = spelled.clone(); let subject_put_back = subject_put_back.clone(); let tags_at_the_end = tags_at_the_end.clone(); let text = text.clone(); let tokens = tokens.clone(); let typed = typed.clone(); let verb = verb.clone(); let w = w.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let after = after.clone(); let after_after = after_after.clone(); let after_words = after_words.clone(); let asking = asking.clone(); let before = before.clone(); let before_before = before_before.clone(); let before_words = before_words.clone(); let change_of_subject = change_of_subject.clone(); let clause = clause.clone(); let clauses = clauses.clone(); let clean = clean.clone(); let fixed = fixed.clone(); let fixes = fixes.clone(); let flat = flat.clone(); let focus = focus.clone(); let index = index.clone(); let intensifier = intensifier.clone(); let intensifier_left = intensifier_left.clone(); let joined = joined.clone(); let keep = keep.clone(); let meant = meant.clone(); let mended = mended.clone(); let mended_word = mended_word.clone(); let norm = norm.clone(); let parts = parts.clone(); let plain = plain.clone(); let said = said.clone(); let sentence_ends = sentence_ends.clone(); let settled = settled.clone(); let signals = signals.clone(); let slips = slips.clone(); let spelled = spelled.clone(); let subject_put_back = subject_put_back.clone(); let tags_at_the_end = tags_at_the_end.clone(); let text = text.clone(); let tokens = tokens.clone(); let typed = typed.clone(); let verb = verb.clone(); let w = w.clone(); let word = word.clone(); let words = words.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); op_add(&mcall(&m.clone(), "group", &[Py::Int(1)]), &mcall(&mcall(&mcall(&m.clone(), "group", &[Py::Int(2)]), "replace", &[py_str(" "), py_str("")]), "replace", &[py_str(" "), py_str("")])) }) }, clean.clone()]);
    clean = _one_word_food(&[clean.clone()]);
    clean = _a_leading_like_is_a_filler(&[clean.clone()]);
    clean = _joined_words(&[clean.clone()]);
    clean = _a_contraction_typed_with_a_space(&[clean.clone()]);
    clean = _an_exclamation_in_front(&[clean.clone()]);
    clean = _an_exclamation_is_not_a_thing(&[clean.clone()]);
    clean = _a_hello_said_short(&[clean.clone()]);
    clean = _chat_words_said_out(&[clean.clone()]);
    clean = _an_ill_that_is_i_will(&[clean.clone()]);
    let __unpk1 = py_unpack(&_without_a_tag_at_the_end(&[clean.clone()]), 2, None);
    clean = __unpk1[0].clone();
    tags_at_the_end = __unpk1[1].clone();
    let __unpk2 = py_unpack(&_change_of_subject_off(&[clean.clone()]), 2, None);
    clean = __unpk2[0].clone();
    change_of_subject = __unpk2[1].clone();
    let __unpk3 = py_unpack(&_a_meal_word_misspelt(&[clean.clone(), mcall(&g_RAW_WORD(), "findall", &[clean.clone()])]), 2, None);
    clean = __unpk3[0].clone();
    words = __unpk3[1].clone();
    let __unpk4 = py_unpack(&_docs_doctor(&[clean.clone(), words.clone()]), 2, None);
    clean = __unpk4[0].clone();
    words = __unpk4[1].clone();
    said = _a_sign_is_a_word(&[clean.clone()]);
    words = mcall(&g_RAW_WORD(), "findall", &[said.clone()]);
    fixes = py_list(vec![]);
    norm = py_list(vec![]);
    plain = py_list(vec![]);
    fixed = py_dict(vec![]);
    subject_put_back = Py::Bool(false);
    sentence_ends = mcall(&g_SENTENCE_END(), "sub", &[py_str(" | "), _phrase_pass(&[said.clone()])]);
    tokens = mcall(&g_TOKEN(), "findall", &[mcall(&g_PUNCT_BREAK(), "sub", &[py_str(" | "), sentence_ends.clone()])]);
    joined = _one_relation_says(&[a_son_not_the_sun(&[a_sor_bit_is_sore(&[loose_is_lose(&[possessive_me(&[_said_apart(&[_join_chat_digits(&[tokens.clone()])])])])])])]);
    let __unpk5 = py_unpack(&_a_slip_that_is_a_word(&[joined.clone()]), 2, None);
    joined = __unpk5[0].clone();
    slips = __unpk5[1].clone();
    let __iter6 = py_iter_or_raise(&mcall(&slips.clone(), "items", &[]));
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 2, None);
    typed = __unpk7[0].clone();
    meant = __unpk7[1].clone();
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![typed.clone(), meant.clone()])]);
    let _ = py_setitem(&fixed.clone(), &typed.clone(), meant.clone());
    }
    let __iter8 = py_iter_or_raise(&call_builtin("enumerate", &[joined.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter8 {
    let __unpk9 = py_unpack(&__item.clone(), 2, None);
    index = __unpk9[0].clone();
    word = __unpk9[1].clone();
    before = if py_truthy(&index.clone()) { geti(&joined.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { Py::None };
    after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[joined.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&joined.clone(), &op_add(&index.clone(), &Py::Int(1))) } else { Py::None };
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_is(&before.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&before.clone(), &py_str("|"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LEADERS(), &before.clone())); if !py_truthy(&__b) { __b } else { _only_leaders_before(&[joined.clone(), index.clone()]) } } } } } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DROPPED_SUBJECT_AUX(), &word.clone())); if !py_truthy(&__b) { __b } else { subject_missing(&[op_add(&py_list(vec![word.clone()]), &_lookahead(&[joined.clone(), index.clone()])), Py::Int(0)]) } } } }) {
    let _ = mcall(&norm.clone(), "extend", &[op_add(&py_list(vec![py_str("i")]), &_token(&[word.clone(), fixes.clone(), before.clone()]))]);
    let _ = mcall(&plain.clone(), "append", &[word.clone()]);
    subject_put_back = Py::Bool(true);
    continue;
    }
    after_after = if py_truthy(&op_lt(&op_add(&index.clone(), &Py::Int(2)), &call_builtin("len", &[joined.clone()]).unwrap_or_else(|e| exc::raise(e)))) { geti(&joined.clone(), &op_add(&index.clone(), &Py::Int(2))) } else { Py::None };
    before_before = if py_truthy(&op_ge(&index.clone(), &Py::Int(2))) { geti(&joined.clone(), &op_sub(&index.clone(), &Py::Int(2))) } else { Py::None };
    if py_truthy(&_a_name_slot(&[before.clone(), before_before.clone(), word.clone()])) {
    let _ = mcall(&norm.clone(), "append", &[word.clone()]);
    let _ = mcall(&plain.clone(), "append", &[word.clone()]);
    continue;
    }
    before_words = slice_or_raise(&joined.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&index.clone(), &Py::Int(5))]).unwrap_or_else(|e| exc::raise(e)), &index.clone(), &Py::None);
    after_words = slice_or_raise(&joined.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(4)), &Py::None);
    verb = _a_core_verb_typed_wrong(&[word.clone(), before.clone(), after.clone()]);
    if py_truthy(&verb.clone()) {
    let _ = mcall(&fixes.clone(), "append", &[py_tuple(vec![word.clone(), verb.clone()])]);
    let _ = py_setitem(&fixed.clone(), &word.clone(), verb.clone());
    }
    let __iter10 = py_iter_or_raise(&if py_truthy(&verb.clone()) { py_list(vec![verb.clone()]) } else { _positional_word(&[word.clone(), before.clone(), after.clone(), after_after.clone(), before_before.clone(), before_words.clone()]) });
    for __item in __iter10 {
    spelled = __item.clone();
    parts = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spelled.clone()); __args.push(fixes.clone()); __args.push(before.clone()); __args.push(before_before.clone()); __args.push(after.clone()); __kws.extend(vec![("before_words".to_string(), before_words.clone()), ("after_words".to_string(), after_words.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _token(args))(&__args) };
    if py_truthy(&{ let __b = parts.clone(); if !py_truthy(&__b) { __b } else { { let __b = norm.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&parts.clone(), &Py::Int(0)), &geti(&norm.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_REPEAT_ONCE(), &geti(&parts.clone(), &Py::Int(0)))) } } } } } }) {
    parts = slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None);
    } else {
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { { let __b = norm.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&parts.clone(), &Py::Int(0)), &py_str("i"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&parts.clone(), &Py::Int(1)), &py_str("do"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_REPEAT_ONCE_SUBJECTS(), &geti(&norm.clone(), &op_neg(&Py::Int(1))))) } } } } } } } }) {
    parts = slice_or_raise(&parts.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    continue;
    }
    let _ = mcall(&norm.clone(), "extend", &[parts.clone()]);
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "PLAIN_EXPANDS"), &spelled.clone())) } }) {
    let _ = mcall(&plain.clone(), "append", &[mcall(&py_str(" "), "join", &[parts.clone()])]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_DIGIT_WORDS(), &spelled.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&pyrt::xrefs::xget("understanding.lexicon", "US_SPELLINGS"), &spelled.clone())) } } } }) {
    let _ = mcall(&plain.clone(), "append", &[geti(&parts.clone(), &Py::Int(0))]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&geti(&parts.clone(), &Py::Int(0)), &spelled.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "MISSPELLINGS"), &spelled.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "URGENT_MISSPELLINGS"), &spelled.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "SAID_SHORT"), &spelled.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_short_word_named(&[spelled.clone(), before.clone(), after.clone()]))) } } } } } } } }) {
    let _ = py_setitem(&fixed.clone(), &spelled.clone(), geti(&parts.clone(), &Py::Int(0)));
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&spelled.clone(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "STATES_WITHOUT_G"), &word.clone())) } }) {
    let _ = py_setitem(&fixed.clone(), &word.clone(), spelled.clone());
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &py_str("bin"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&parts.clone(), &Py::Int(0)), &py_str("been"))) } }) {
    let _ = py_setitem(&fixed.clone(), &word.clone(), geti(&parts.clone(), &Py::Int(0)));
    }
    }
    }
    } else {
    let _ = mcall(&plain.clone(), "append", &[mcall(&pyrt::xrefs::xget("understanding.lexicon", "TYPED_CONTRACTIONS"), "get", &[spelled.clone(), spelled.clone()])]);
    }
    }
    }
    }
    norm = _fix_auxiliaries(&[_says_has(&[norm.clone()])]);
    intensifier = _the_surprise_typed(&[text.clone()]);
    intensifier_left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("sorted", &[intensifier_positions(&[norm.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut index: Py = __item.clone(); __out.push(geti(&norm.clone(), &index.clone())); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&intensifier.clone()))) {
    intensifier = call_builtin("list", &[intensifier_left.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&intensifier_left.clone()) {
    norm = _without_the_intensifier(&[norm.clone()]);
    }
    let __unpk11 = py_unpack(&_a_game_named_another_way(&[norm.clone(), words.clone()]), 2, None);
    norm = __unpk11[0].clone();
    words = __unpk11[1].clone();
    norm = _parts_that_hurt(&[norm.clone(), fixes.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&norm.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.lexicon", "PAIN_WORDS"), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    mended = call_builtin("dict", &[fixes.clone()]).unwrap_or_else(|e| exc::raise(e));
    plain = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plain.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mended.clone(), "get", &[word.clone(), word.clone()])); } py_list(__out) })() };
    }
    let __unpk12 = py_unpack(&_a_clip_kept_only_with_its_cue(&[plain.clone(), norm.clone(), fixes.clone(), joined.clone()]), 3, None);
    plain = __unpk12[0].clone();
    norm = __unpk12[1].clone();
    fixes = __unpk12[2].clone();
    clauses = _split_ask(&[_repair_openers(&[_clauses(&[norm.clone()]), fixes.clone()])]);
    flat = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut clause: Py = __item.clone(); for __item in py_iter_or_raise(&clause.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() };
    focus = _focus(&[clauses.clone()]);
    asking = _asking(&[clauses.clone(), focus.clone()]);
    signals = py_dict(vec![(py_str("question_mark"), Py::Bool(py_contains(&text.clone(), &py_str("?")))), (py_str("shout"), op_gt(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&text.clone(), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { mcall(&w.clone(), "isupper", &[]) } }) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0))), (py_str("laughter"), { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&norm.clone(), &tags_at_the_end.clone())) { let mut w: Py = __item.clone(); __out.push(pyrt::xrefs::xcall("understanding.lexicon", "is_a_laugh", &[w.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:ha\\s*){2,}|\\b(?:heh\\s*){2,}"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }), (py_str("ellipsis"), { let __b = Py::Bool(py_contains(&text.clone(), &py_str("..."))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str(".."))) } }), (py_str("exclamation"), Py::Bool(py_contains(&text.clone(), &py_str("!")))), (py_str("empty"), Py::Bool(!py_truthy(&words.clone())))]);
    keep = _asks_the_spelling(&[words.clone(), fixed.clone()]);
    settled = call_builtin("dict", &[fixed.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter13 = py_iter_or_raise(&fixes.clone());
    for __item in __iter13 {
    let __unpk14 = py_unpack(&__item.clone(), 2, None);
    typed = __unpk14[0].clone();
    mended_word = __unpk14[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&typed.clone(), &mended_word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&mended_word.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&fixed.clone(), &typed.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_only_doubles_a_letter(&[typed.clone(), mended_word.clone()]))) } } } } } }) {
    let _ = mcall(&settled.clone(), "setdefault", &[typed.clone(), mended_word.clone()]);
    }
    }
    return py_dict(vec![(py_str("text"), text.clone()), (py_str("clean"), clean.clone()), (py_str("words"), if py_truthy(&keep.clone()) { call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&settled.clone(), "get", &[word.clone(), word.clone()])); } py_list(__out) })() } }), (py_str("plain"), mcall(&stdlib_call("re", "sub", &[py_str("\\s*\\|"), py_str(","), mcall(&py_str(" "), "join", &[plain.clone()])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" ,")])), (py_str("norm"), flat.clone()), (py_str("norm_text"), if py_truthy(&clauses.clone()) { mcall(&py_str(" | "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut clause: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[clause.clone()])); } py_list(__out) })() }]) } else { py_str("") }), (py_str("fixes"), call_builtin("dict", &[fixes.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("intensifier"), intensifier.clone()), (py_str("intensifier_left"), intensifier_left.clone()), (py_str("clauses"), clauses.clone()), (py_str("focus"), focus.clone()), (py_str("asking"), asking.clone()), (py_str("signals"), signals.clone()), (py_str("dropped_subject"), subject_put_back.clone()), (py_str("change_of_subject"), change_of_subject.clone()), (py_str("kept"), _kept_for(&[flat.clone()]))]);
    Py::None
}
pub fn _the_feeling_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut said: Py = Py::None;
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]), "split", &[])]);
    return mcall(&g__FEELING_WORD(), "get", &[said.clone(), py_str("upset")]);
    Py::None
}
pub fn without_the_figure_of_speech(args: &[Py]) -> Py {
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
    let _ = py_str("`clean` with \"sick with worry\" read as the feeling it stands for, for the body readers.");
    clean = mcall(&g_FEELING_BEFORE_SICK(), "sub", &[{ let clean = clean.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); geti(&mcall(&mcall(&m.clone(), "group", &[Py::Int(0)]), "split", &[]), &Py::Int(0)) }) }, call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    return mcall(&g_SICK_AS_FEELING(), "sub", &[py_func("_the_feeling_word", |args: &[Py]| _the_feeling_word(args)), clean.clone()]);
    Py::None
}
pub fn _the_part_that_is_bad(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut link: Py = Py::None;
    let mut part: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut verb: Py = Py::None;
    part = mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "lower", &[]);
    plural = { let __b = mcall(&part.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THE_ODD_PLURALS(), &part.clone())) } };
    link = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("link")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("has been"), py_str("have been")]), &link.clone()))) {
    verb = if py_truthy(&plural.clone()) { py_str("have been hurting") } else { py_str("has been hurting") };
    return op_modulo(&py_str("%s %s %s"), &py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)]), verb.clone()]));
    }
    verb = if py_truthy(&plural.clone()) { py_str("hurt") } else { py_str("hurts") };
    return op_modulo(&py_str("%s %s %s"), &py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)]), verb.clone()]));
    Py::None
}
pub fn _the_part_that_hurts(args: &[Py]) -> Py {
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
    let _ = py_str("`clean` with \"my back is a joke\" said as \"my back hurts\", for the health readers.");
    return mcall(&g_PART_THAT_IS_BAD(), "sub", &[py_func("_the_part_that_is_bad", |args: &[Py]| _the_part_that_is_bad(args)), call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _the_sugar_that_is_blood(args: &[Py]) -> Py {
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
    let mut rewrite: Py = Py::None;
    let _ = py_str("`clean` with \"my sugar was low\" said as \"my blood sugar was low\", for the health readers.");
pub fn __nested_rewrite_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut clean: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut rewrite: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut at: Py = Py::None;
    let mut following: Py = Py::None;
    let mut level: Py = Py::None;
    let mut near: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    tail = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?")]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&g_SUGAR_PRICE_SHAPE(), "match", &[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "isdigit", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return op_modulo(&py_str("%s blood sugar%s"), &py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)])]));
    }
    near = slice_or_raise(&tail.clone(), &Py::None, &Py::Int(3), &Py::None);
    level = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&near.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_SUGAR_LEVEL(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&level.clone(), &py_list(vec![py_str("rose")])))) {
    following = slice_or_raise(&tail.clone(), &op_add(&mcall(&tail.clone(), "index", &[py_str("rose")]), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&{ let __b = following.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_SUGAR_AFTER_ROSE(), &geti(&following.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&geti(&following.clone(), &Py::Int(0)), &Py::None, &Py::Int(1), &Py::None), "isdigit", &[]))) } } } }) {
    level = py_list(vec![]);
    }
    }
    if py_truthy(&level.clone()) {
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    at = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SUGAR_LEVEL(), &word.clone())); if !py_truthy(&__b) { __b } else { op_bitand(&g_SUGAR_PRICE(), &call_builtin("set", &[slice_or_raise(&tail.clone(), &op_add(&at.clone(), &Py::Int(1)), &op_add(&at.clone(), &Py::Int(4)), &Py::None)]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    }
    return op_modulo(&py_str("%s blood sugar%s"), &py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)])]));
    }
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    Py::None
}
    rewrite = {{ let clean = clean.clone(); let rewrite = rewrite.clone(); py_func("rewrite", move |args: &[Py]| { let mut full = vec![clean.clone(), rewrite.clone()]; full.extend_from_slice(args); __nested_rewrite_0(&full) }) }};
    return mcall(&g_SUGAR_FIRST_PERSON(), "sub", &[rewrite.clone(), call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn glued_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { Py::None };
    let _ = py_str("The words a glued token stands for, or None - the name a module reads.\n\n    The reading splits these tokens for its own line, and a module that reads the line another\n    way (the safety module does, in `safety._norm`) reads the same table rather than keeping one\n    of its own: \"whatsthe point\" is the crisis line \"whats the point\" is, and \"iwant to die\" is\n    \"i want to die\".\n    ");
    return _glued_words(&[token.clone(), after.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.normalize".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_FORM" => Some(g_ABOUT_THE_FORM()),
            "AEIOU" => Some(g_AEIOU()),
            "AFTER_AN_INVERTED_AUX" => Some(g_AFTER_AN_INVERTED_AUX()),
            "AFTER_A_STATE" => Some(g_AFTER_A_STATE()),
            "AFTER_A_THING" => Some(g_AFTER_A_THING()),
            "AFTER_A_TIME" => Some(g_AFTER_A_TIME()),
            "AFTER_A_VERB" => Some(g_AFTER_A_VERB()),
            "AINT_FORMS" => Some(g_AINT_FORMS()),
            "AINT_PLURAL" => Some(g_AINT_PLURAL()),
            "AMOUNT_BEFORE_A_STATE" => Some(g_AMOUNT_BEFORE_A_STATE()),
            "APOSTROPHE_BASES" => Some(g_APOSTROPHE_BASES()),
            "APOSTROPHE_NOT" => Some(g_APOSTROPHE_NOT()),
            "APOSTROPHE_TAILS" => Some(g_APOSTROPHE_TAILS()),
            "ASKS_WHOSE" => Some(g_ASKS_WHOSE()),
            "ASK_NOUNS" => Some(g_ASK_NOUNS()),
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "AUX_OPENS" => Some(g_AUX_OPENS()),
            "AUX_WORDS" => Some(g_AUX_WORDS()),
            "A_CHOKE_SAID_THE_OTHER_WAY" => Some(g_A_CHOKE_SAID_THE_OTHER_WAY()),
            "A_COPULA" => Some(g_A_COPULA()),
            "A_COUNT" => Some(g_A_COUNT()),
            "A_FEELING_CAN_BE_FOLLOWED" => Some(g_A_FEELING_CAN_BE_FOLLOWED()),
            "A_GLASS_OR_A_PAN" => Some(g_A_GLASS_OR_A_PAN()),
            "A_JOB" => Some(g_A_JOB()),
            "A_KEEPING_A_THING" => Some(g_A_KEEPING_A_THING()),
            "A_LABEL_BEFORE" => Some(g_A_LABEL_BEFORE()),
            "A_LETTER_AFTER" => Some(g_A_LETTER_AFTER()),
            "A_NEGATION_BEFORE" => Some(g_A_NEGATION_BEFORE()),
            "A_NUMBER_OF_SOMEONES_DAY" => Some(g_A_NUMBER_OF_SOMEONES_DAY()),
            "A_PAIN_WORD" => Some(g_A_PAIN_WORD()),
            "A_PARTS_OF_A_PERIOD" => Some(g_A_PARTS_OF_A_PERIOD()),
            "A_PART_A_PERSON_OWNS" => Some(g_A_PART_A_PERSON_OWNS()),
            "A_PART_THE_PAIN_IS_IN" => Some(g_A_PART_THE_PAIN_IS_IN()),
            "A_PERIOD_FOLLOWS" => Some(g_A_PERIOD_FOLLOWS()),
            "A_PRONOUN_GLUED" => Some(g_A_PRONOUN_GLUED()),
            "A_PRONOUN_STATE" => Some(g_A_PRONOUN_STATE()),
            "A_SIDE" => Some(g_A_SIDE()),
            "A_STATE_THEN_A_TIME" => Some(g_A_STATE_THEN_A_TIME()),
            "A_STATE_YOU_CAN_FEEL" => Some(g_A_STATE_YOU_CAN_FEEL()),
            "A_THING_GOES_AFTER" => Some(g_A_THING_GOES_AFTER()),
            "A_THING_TYPED_AS_A_LITTLE_WORD" => Some(g_A_THING_TYPED_AS_A_LITTLE_WORD()),
            "A_THING_WORN" => Some(g_A_THING_WORN()),
            "A_TIME_TO_LEAVE" => Some(g_A_TIME_TO_LEAVE()),
            "A_VERB_CAN_FOLLOW" => Some(g_A_VERB_CAN_FOLLOW()),
            "A_WORD_CAN_FOLLOW" => Some(g_A_WORD_CAN_FOLLOW()),
            "BAD_SAYING" => Some(g_BAD_SAYING()),
            "BAD_WORD" => Some(g_BAD_WORD()),
            "BARE_TO" => Some(g_BARE_TO()),
            "BASE" => Some(g_BASE()),
            "BEEN_SUBJECT" => Some(g_BEEN_SUBJECT()),
            "BEEN_WORDS" => Some(g_BEEN_WORDS()),
            "BEFORE_A_FEELING" => Some(g_BEFORE_A_FEELING()),
            "BEFORE_A_JOB" => Some(g_BEFORE_A_JOB()),
            "BEFORE_A_PREPOSITION" => Some(g_BEFORE_A_PREPOSITION()),
            "BEFORE_A_THING" => Some(g_BEFORE_A_THING()),
            "BEFORE_A_VERB" => Some(g_BEFORE_A_VERB()),
            "BEFORE_THE_LOSING" => Some(g_BEFORE_THE_LOSING()),
            "BEFORE_VERB" => Some(g_BEFORE_VERB()),
            "BEFORE_WORD" => Some(g_BEFORE_WORD()),
            "BE_BEFORE_A_STATE" => Some(g_BE_BEFORE_A_STATE()),
            "BE_FORMS" => Some(g_BE_FORMS()),
            "BE_VERBS" => Some(g_BE_VERBS()),
            "BE_WORDS" => Some(g_BE_WORDS()),
            "BIN_CONTAINERS" => Some(g_BIN_CONTAINERS()),
            "BREAKS" => Some(g_BREAKS()),
            "CAN_FOLLOW_A_VERB" => Some(g_CAN_FOLLOW_A_VERB()),
            "CAN_NOT" => Some(g_CAN_NOT()),
            "CAUSES_A_STATE" => Some(g_CAUSES_A_STATE()),
            "CAUSE_A_CLAUSE_VERB" => Some(g_CAUSE_A_CLAUSE_VERB()),
            "CAUSE_A_DEMONSTRATIVE" => Some(g_CAUSE_A_DEMONSTRATIVE()),
            "CAUSE_A_SUBJECT_FOLLOWS" => Some(g_CAUSE_A_SUBJECT_FOLLOWS()),
            "CAUSE_BEFORE_A_NOUN" => Some(g_CAUSE_BEFORE_A_NOUN()),
            "CAUSE_BEFORE_A_VERB" => Some(g_CAUSE_BEFORE_A_VERB()),
            "CHANGE_OF_SUBJECT" => Some(g_CHANGE_OF_SUBJECT()),
            "CHAT_AS_ONE_WORD" => Some(g_CHAT_AS_ONE_WORD()),
            "CHAT_KEPT_AS_TYPED" => Some(g_CHAT_KEPT_AS_TYPED()),
            "CHAT_NAMES" => Some(g_CHAT_NAMES()),
            "CLAUSE_OPENERS" => Some(g_CLAUSE_OPENERS()),
            "CLIPPED_FAMILY" => Some(g_CLIPPED_FAMILY()),
            "CLIPPED_THINGS" => Some(g_CLIPPED_THINGS()),
            "CLIPPED_UNITS" => Some(g_CLIPPED_UNITS()),
            "COMMON_RANK" => Some(g_COMMON_RANK()),
            "COMPOUND_HEADS" => Some(g_COMPOUND_HEADS()),
            "COMPOUND_TAILS" => Some(g_COMPOUND_TAILS()),
            "CONTRACTION_NEXT" => Some(g_CONTRACTION_NEXT()),
            "COPULAS" => Some(g_COPULAS()),
            "CORE_VERBS" => Some(g_CORE_VERBS()),
            "COUNTS" => Some(g_COUNTS()),
            "COUNT_UNITS" => Some(g_COUNT_UNITS()),
            "CURRENCY_FACTOR" => Some(g_CURRENCY_FACTOR()),
            "CURRENCY_SIGN" => Some(g_CURRENCY_SIGN()),
            "CURRENCY_SIGNS" => Some(g_CURRENCY_SIGNS()),
            "DAY_NAMED" => Some(g_DAY_NAMED()),
            "DEMONSTRATIVE" => Some(g_DEMONSTRATIVE()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "DETERMINER_IN_FRONT" => Some(g_DETERMINER_IN_FRONT()),
            "DIGIT_AS_WORD" => Some(g_DIGIT_AS_WORD()),
            "DIGIT_WORDS" => Some(g_DIGIT_WORDS()),
            "DOCS_ON" => Some(g_DOCS_ON()),
            "DOCS_PAPER" => Some(g_DOCS_PAPER()),
            "DOCS_SCREEN" => Some(g_DOCS_SCREEN()),
            "DOCS_VISIT" => Some(g_DOCS_VISIT()),
            "DOING_NOT_THING" => Some(g_DOING_NOT_THING()),
            "DROPPED_SUBJECT_AUX" => Some(g_DROPPED_SUBJECT_AUX()),
            "EXCLAIMED_AFTER_A_QUESTION" => Some(g_EXCLAIMED_AFTER_A_QUESTION()),
            "EXCLAMATION_OPENS" => Some(g_EXCLAMATION_OPENS()),
            "EXCLAMATION_PHRASE" => Some(g_EXCLAMATION_PHRASE()),
            "E_KEPT_WORDS" => Some(g_E_KEPT_WORDS()),
            "FAMILY_SHRT" => Some(g_FAMILY_SHRT()),
            "FEELING_BEFORE_SICK" => Some(g_FEELING_BEFORE_SICK()),
            "FINDING_A_TEAM" => Some(g_FINDING_A_TEAM()),
            "FIRST_PERSON_BE" => Some(g_FIRST_PERSON_BE()),
            "FOOD_SLOT_LEAD" => Some(g_FOOD_SLOT_LEAD()),
            "FOR_A" => Some(g_FOR_A()),
            "FOR_MEAL" => Some(g_FOR_MEAL()),
            "FOR_OBJECT" => Some(g_FOR_OBJECT()),
            "FREQUENT" => Some(g_FREQUENT()),
            "F_TYPED_FOR_TH" => Some(g_F_TYPED_FOR_TH()),
            "GLUED_AFTER_A_DO" => Some(g_GLUED_AFTER_A_DO()),
            "GLUED_COLLAPSE" => Some(g_GLUED_COLLAPSE()),
            "GLUED_DO" => Some(g_GLUED_DO()),
            "GLUED_LEFT" => Some(g_GLUED_LEFT()),
            "GLUED_RIGHT" => Some(g_GLUED_RIGHT()),
            "HAS_AFTER" => Some(g_HAS_AFTER()),
            "HAS_FORMS" => Some(g_HAS_FORMS()),
            "HAS_SUBJECT" => Some(g_HAS_SUBJECT()),
            "HAS_SUBJECTS" => Some(g_HAS_SUBJECTS()),
            "HAVE_FORMS" => Some(g_HAVE_FORMS()),
            "HEART_FEELINGS" => Some(g_HEART_FEELINGS()),
            "HEART_MEASURES" => Some(g_HEART_MEASURES()),
            "HEART_SAID_AS_A_FEELING" => Some(g_HEART_SAID_AS_A_FEELING()),
            "HELLO_SAID_SHORT" => Some(g_HELLO_SAID_SHORT()),
            "HELLO_TAIL" => Some(g_HELLO_TAIL()),
            "HOW_DEGREE" => Some(g_HOW_DEGREE()),
            "HOW_FOLLOWS" => Some(g_HOW_FOLLOWS()),
            "HR_IS_THE_DEPARTMENT_AFTER" => Some(g_HR_IS_THE_DEPARTMENT_AFTER()),
            "HV_HELPERS" => Some(g_HV_HELPERS()),
            "HV_SUBJECTS" => Some(g_HV_SUBJECTS()),
            "ILL_GOES_ON" => Some(g_ILL_GOES_ON()),
            "ING_THINGS" => Some(g_ING_THINGS()),
            "INTRODUCING_AFTER" => Some(g_INTRODUCING_AFTER()),
            "INTRODUCING_A_PERSON" => Some(g_INTRODUCING_A_PERSON()),
            "IN_NOUNS" => Some(g_IN_NOUNS()),
            "IRREGULAR_PARTICIPLES" => Some(g_IRREGULAR_PARTICIPLES()),
            "I_GLUED" => Some(g_I_GLUED()),
            "JOINED_WORDS" => Some(g_JOINED_WORDS()),
            "KIN" => Some(g_KIN()),
            "KIN_FILLERS" => Some(g_KIN_FILLERS()),
            "KIN_HAS" => Some(g_KIN_HAS()),
            "KIN_IS" => Some(g_KIN_IS()),
            "KIN_SINGULAR" => Some(g_KIN_SINGULAR()),
            "KNOW_SUBJECT" => Some(g_KNOW_SUBJECT()),
            "LEADERS" => Some(g_LEADERS()),
            "LEFT_OFF_CONTRACTION" => Some(g_LEFT_OFF_CONTRACTION()),
            "LETTERS" => Some(g_LETTERS()),
            "LETTER_C_IS_SEE" => Some(g_LETTER_C_IS_SEE()),
            "LEXICON" => Some(g_LEXICON()),
            "LIKE_STARTS_A_CLAUSE" => Some(g_LIKE_STARTS_A_CLAUSE()),
            "LONG_ENOUGH_TO_BE_A_VERB" => Some(g_LONG_ENOUGH_TO_BE_A_VERB()),
            "LOOSE_FOR_LOSE" => Some(g_LOOSE_FOR_LOSE()),
            "MEAL_WORDS" => Some(g_MEAL_WORDS()),
            "ME_FOR_MY_CLOSES" => Some(g_ME_FOR_MY_CLOSES()),
            "MISSING_E_AFTER" => Some(g_MISSING_E_AFTER()),
            "MI_BACK_VERBS" => Some(g_MI_BACK_VERBS()),
            "MI_IS_A_NAME_AFTER" => Some(g_MI_IS_A_NAME_AFTER()),
            "MI_IS_ME_AFTER" => Some(g_MI_IS_ME_AFTER()),
            "MI_IS_ME_BEFORE" => Some(g_MI_IS_ME_BEFORE()),
            "MODAL_FORMS" => Some(g_MODAL_FORMS()),
            "MOTION_BEFORE" => Some(g_MOTION_BEFORE()),
            "MRS_IS_THE_WIFE" => Some(g_MRS_IS_THE_WIFE()),
            "NAMED_LETTER" => Some(g_NAMED_LETTER()),
            "NAME_CALLS" => Some(g_NAME_CALLS()),
            "NAME_FRAMES" => Some(g_NAME_FRAMES()),
            "NAME_ROLL" => Some(g_NAME_ROLL()),
            "NOMINAL_BEFORE_COPULA" => Some(g_NOMINAL_BEFORE_COPULA()),
            "NOT_A_COMPOUND_HEAD" => Some(g_NOT_A_COMPOUND_HEAD()),
            "NOT_A_COMPOUND_TAIL" => Some(g_NOT_A_COMPOUND_TAIL()),
            "NOT_A_MEAL_TYPO" => Some(g_NOT_A_MEAL_TYPO()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_A_THING_AFTER" => Some(g_NOT_A_THING_AFTER()),
            "NOT_A_THING_ENDING" => Some(g_NOT_A_THING_ENDING()),
            "NOT_A_THING_PAIR" => Some(g_NOT_A_THING_PAIR()),
            "NOT_BEFORE_A_NOUN" => Some(g_NOT_BEFORE_A_NOUN()),
            "NOT_BEFORE_KNOW" => Some(g_NOT_BEFORE_KNOW()),
            "NOT_COUNTED_AFTER" => Some(g_NOT_COUNTED_AFTER()),
            "NOT_IN_A_QUESTION_PHRASE" => Some(g_NOT_IN_A_QUESTION_PHRASE()),
            "NOT_MY_OWN_DAY" => Some(g_NOT_MY_OWN_DAY()),
            "NOT_WORDS" => Some(g_NOT_WORDS()),
            "NO_ASK_AFTER" => Some(g_NO_ASK_AFTER()),
            "NO_SWAP_TO" => Some(g_NO_SWAP_TO()),
            "NUMBERS_BEFORE" => Some(g_NUMBERS_BEFORE()),
            "OBJECT_PRONOUN" => Some(g_OBJECT_PRONOUN()),
            "ONE_OF_A_KIND" => Some(g_ONE_OF_A_KIND()),
            "ONE_PART_AFTER" => Some(g_ONE_PART_AFTER()),
            "ONE_RELATION" => Some(g_ONE_RELATION()),
            "ONE_RELATION_VERBS" => Some(g_ONE_RELATION_VERBS()),
            "ONE_WORD_FOODS" => Some(g_ONE_WORD_FOODS()),
            "OPENERS" => Some(g_OPENERS()),
            "OPERATORS" => Some(g_OPERATORS()),
            "OWNER_WITHOUT_THE_DAY" => Some(g_OWNER_WITHOUT_THE_DAY()),
            "OWNING_A_PERSON" => Some(g_OWNING_A_PERSON()),
            "OWNING_VERBS" => Some(g_OWNING_VERBS()),
            "PART_REACH" => Some(g_PART_REACH()),
            "PART_THAT_IS_BAD" => Some(g_PART_THAT_IS_BAD()),
            "PAST_AS_PARTICIPLE" => Some(g_PAST_AS_PARTICIPLE()),
            "PERIOD_COUNTS" => Some(g_PERIOD_COUNTS()),
            "PERSON_PREPS" => Some(g_PERSON_PREPS()),
            "PERSON_WORDS" => Some(g_PERSON_WORDS()),
            "PHRASES" => Some(g_PHRASES()),
            "PHRASE_MEASURES" => Some(g_PHRASE_MEASURES()),
            "PLACE_WORDS" => Some(g_PLACE_WORDS()),
            "PLURAL_SUBJECTS" => Some(g_PLURAL_SUBJECTS()),
            "POSSESSIVES" => Some(g_POSSESSIVES()),
            "POSSESSIVE_IN_FRONT" => Some(g_POSSESSIVE_IN_FRONT()),
            "PRONOUN_BASES" => Some(g_PRONOUN_BASES()),
            "PUNCT_BREAK" => Some(g_PUNCT_BREAK()),
            "RAW_WORD" => Some(g_RAW_WORD()),
            "REASON_STARTS" => Some(g_REASON_STARTS()),
            "RELATIVE" => Some(g_RELATIVE()),
            "RELATIVE_ASKS" => Some(g_RELATIVE_ASKS()),
            "REPEAT_ONCE" => Some(g_REPEAT_ONCE()),
            "REPEAT_ONCE_SUBJECTS" => Some(g_REPEAT_ONCE_SUBJECTS()),
            "REPORTING" => Some(g_REPORTING()),
            "RUN_ON_LAST_PIECES" => Some(g_RUN_ON_LAST_PIECES()),
            "RUN_ON_MINE" => Some(g_RUN_ON_MINE()),
            "RUN_ON_OPENERS" => Some(g_RUN_ON_OPENERS()),
            "RUN_ON_PEOPLE" => Some(g_RUN_ON_PEOPLE()),
            "RUN_ON_SELF" => Some(g_RUN_ON_SELF()),
            "RUN_ON_SELF_LONGEST" => Some(g_RUN_ON_SELF_LONGEST()),
            "RUN_ON_SHORTEST" => Some(g_RUN_ON_SHORTEST()),
            "RUN_ON_WORDS" => Some(g_RUN_ON_WORDS()),
            "SAID_APART" => Some(g_SAID_APART()),
            "SAID_APART_FORMS" => Some(g_SAID_APART_FORMS()),
            "SAID_AS_A_FEELING" => Some(g_SAID_AS_A_FEELING()),
            "SAID_IT" => Some(g_SAID_IT()),
            "SAID_OUT" => Some(g_SAID_OUT()),
            "SENTENCE_END" => Some(g_SENTENCE_END()),
            "SHION_WORDS" => Some(g_SHION_WORDS()),
            "SHRT_THINGS" => Some(g_SHRT_THINGS()),
            "SICK_AS_FEELING" => Some(g_SICK_AS_FEELING()),
            "SOLFEGE" => Some(g_SOLFEGE()),
            "SON_VERBS" => Some(g_SON_VERBS()),
            "SORE_FILLERS" => Some(g_SORE_FILLERS()),
            "SORE_IN_FRONT" => Some(g_SORE_IN_FRONT()),
            "SORE_PARTS" => Some(g_SORE_PARTS()),
            "SPELLING_ASK" => Some(g_SPELLING_ASK()),
            "SPELLING_ENDINGS" => Some(g_SPELLING_ENDINGS()),
            "SPLIT_CONTRACTION" => Some(g_SPLIT_CONTRACTION()),
            "SPLIT_NOT" => Some(g_SPLIT_NOT()),
            "STATES_OF_A_PERSON" => Some(g_STATES_OF_A_PERSON()),
            "STOPPED_TH_STEMS" => Some(g_STOPPED_TH_STEMS()),
            "SUBJECT_WORDS" => Some(g_SUBJECT_WORDS()),
            "SUGAR_AFTER_ROSE" => Some(g_SUGAR_AFTER_ROSE()),
            "SUGAR_FIRST_PERSON" => Some(g_SUGAR_FIRST_PERSON()),
            "SUGAR_LEVEL" => Some(g_SUGAR_LEVEL()),
            "SUGAR_PRICE" => Some(g_SUGAR_PRICE()),
            "SUGAR_PRICE_SHAPE" => Some(g_SUGAR_PRICE_SHAPE()),
            "SUM_STAYS_AFTER" => Some(g_SUM_STAYS_AFTER()),
            "SUM_THE_NOUN" => Some(g_SUM_THE_NOUN()),
            "SUN_WORDS" => Some(g_SUN_WORDS()),
            "S_CLUSTERS" => Some(g_S_CLUSTERS()),
            "TAG_GUARD_BEFORE" => Some(g_TAG_GUARD_BEFORE()),
            "TAG_PLEASE" => Some(g_TAG_PLEASE()),
            "TAG_QUESTION_LEADS" => Some(g_TAG_QUESTION_LEADS()),
            "TAG_THANKS" => Some(g_TAG_THANKS()),
            "TAG_WORDS" => Some(g_TAG_WORDS()),
            "TAIL_OF_A_DAY" => Some(g_TAIL_OF_A_DAY()),
            "THE_HEART_SAID_THE_OTHER_WAY" => Some(g_THE_HEART_SAID_THE_OTHER_WAY()),
            "THE_LOSING" => Some(g_THE_LOSING()),
            "THE_ODD_PLURALS" => Some(g_THE_ODD_PLURALS()),
            "THING_AFTER" => Some(g_THING_AFTER()),
            "THING_DETERMINER" => Some(g_THING_DETERMINER()),
            "THING_JOIN" => Some(g_THING_JOIN()),
            "THING_PLACE_AFTER" => Some(g_THING_PLACE_AFTER()),
            "THING_PLACE_WORDS" => Some(g_THING_PLACE_WORDS()),
            "THING_POSSESSIVES" => Some(g_THING_POSSESSIVES()),
            "THING_SUBJECT" => Some(g_THING_SUBJECT()),
            "THING_WORDS" => Some(g_THING_WORDS()),
            "THING_WORDS_BY_LENGTH" => Some(g_THING_WORDS_BY_LENGTH()),
            "THIRD_PERSON" => Some(g_THIRD_PERSON()),
            "THIRD_PERSON_BE" => Some(g_THIRD_PERSON_BE()),
            "THOUSANDS" => Some(g_THOUSANDS()),
            "THOUSANDS_SAID_WITH_A_SPACE" => Some(g_THOUSANDS_SAID_WITH_A_SPACE()),
            "TIMES_A_STATE" => Some(g_TIMES_A_STATE()),
            "TIME_AFTER_MEAL" => Some(g_TIME_AFTER_MEAL()),
            "TIME_WORDS" => Some(g_TIME_WORDS()),
            "TOKEN" => Some(g_TOKEN()),
            "TOO_A" => Some(g_TOO_A()),
            "TOO_AFTER" => Some(g_TOO_AFTER()),
            "TOO_A_GRADED" => Some(g_TOO_A_GRADED()),
            "TO_AFTER" => Some(g_TO_AFTER()),
            "UP_TOO_ASKS" => Some(g_UP_TOO_ASKS()),
            "UP_TO_LEADS" => Some(g_UP_TO_LEADS()),
            "UP_TO_SPELLINGS" => Some(g_UP_TO_SPELLINGS()),
            "UP_TO_TAGS" => Some(g_UP_TO_TAGS()),
            "URGENT_BONES" => Some(g_URGENT_BONES()),
            "URGENT_WORDS" => Some(g_URGENT_WORDS()),
            "VEGGIES_AFTER" => Some(g_VEGGIES_AFTER()),
            "VOWELS" => Some(g_VOWELS()),
            "VOWEL_SET" => Some(g_VOWEL_SET()),
            "VOWEL_SET_OR_Y" => Some(g_VOWEL_SET_OR_Y()),
            "WANT_WORDS" => Some(g_WANT_WORDS()),
            "WEARS_AFTER" => Some(g_WEARS_AFTER()),
            "WHAT_ARE_YOU_AFTER" => Some(g_WHAT_ARE_YOU_AFTER()),
            "WHAT_ARE_YOU_SPELLINGS" => Some(g_WHAT_ARE_YOU_SPELLINGS()),
            "WH_ANY" => Some(g_WH_ANY()),
            "WH_INTENSIFIER" => Some(g_WH_INTENSIFIER()),
            "WH_INTENSIFIER_LINE" => Some(g_WH_INTENSIFIER_LINE()),
            "WH_OPENERS" => Some(g_WH_OPENERS()),
            "WH_QUANTIFIERS" => Some(g_WH_QUANTIFIERS()),
            "WH_WORDS" => Some(g_WH_WORDS()),
            "WORD" => Some(g_WORD()),
            "YOU_ARE_AFTER" => Some(g_YOU_ARE_AFTER()),
            "_AFTER_A_DOING" => Some(g__AFTER_A_DOING()),
            "_AFTER_A_LEAD_IN_SUBJECT" => Some(g__AFTER_A_LEAD_IN_SUBJECT()),
            "_AFTER_PARTICIPLE" => Some(g__AFTER_PARTICIPLE()),
            "_ANY_WORD" => Some(g__ANY_WORD()),
            "_A_CLIP" => Some(g__A_CLIP()),
            "_A_CLIPPED_HEAD" => Some(g__A_CLIPPED_HEAD()),
            "_A_CLOSING_WORD" => Some(g__A_CLOSING_WORD()),
            "_A_HEAD_IN_THE_IDIOM" => Some(g__A_HEAD_IN_THE_IDIOM()),
            "_A_QUESTION_VERB" => Some(g__A_QUESTION_VERB()),
            "_A_SHORT_WORD_IS_NAMED_AFTER" => Some(g__A_SHORT_WORD_IS_NAMED_AFTER()),
            "_A_SHORT_WORD_IS_NAMED_BEFORE" => Some(g__A_SHORT_WORD_IS_NAMED_BEFORE()),
            "_A_STATE_OF_THE_PERSON" => Some(g__A_STATE_OF_THE_PERSON()),
            "_A_WORDS" => Some(g__A_WORDS()),
            "_A_WORD_THE_BOT_WRITES" => Some(g__A_WORD_THE_BOT_WRITES()),
            "_BASE" => Some(g__BASE()),
            "_DAY_WORDS" => Some(g__DAY_WORDS()),
            "_EXCLAMATION_SUBJECTS" => Some(g__EXCLAMATION_SUBJECTS()),
            "_FEELING_WORD" => Some(g__FEELING_WORD()),
            "_HAVE_ASKING" => Some(g__HAVE_ASKING()),
            "_HAVE_NEGATIONS" => Some(g__HAVE_NEGATIONS()),
            "_HAVE_PARTICIPLES" => Some(g__HAVE_PARTICIPLES()),
            "_HAVE_THING" => Some(g__HAVE_THING()),
            "_INTENSIFIER" => Some(g__INTENSIFIER()),
            "_KEEPING_PLACES" => Some(g__KEEPING_PLACES()),
            "_KEPT_ALREADY" => Some(g__KEPT_ALREADY()),
            "_KEPT_THINGS" => Some(g__KEPT_THINGS()),
            "_LEADER_WORD" => Some(g__LEADER_WORD()),
            "_MEASURE" => Some(g__MEASURE()),
            "_NUMBER_SHAPE" => Some(g__NUMBER_SHAPE()),
            "_PLACES_SAID_WHOLE" => Some(g__PLACES_SAID_WHOLE()),
            "_PLACE_LEADS" => Some(g__PLACE_LEADS()),
            "_PLACE_STOP" => Some(g__PLACE_STOP()),
            "_PREDICATE_ONLY" => Some(g__PREDICATE_ONLY()),
            "_SMALL_NUMBERS" => Some(g__SMALL_NUMBERS()),
            "_SPACED_PIECE" => Some(g__SPACED_PIECE()),
            "_SPACED_RUN" => Some(g__SPACED_RUN()),
            "_STATE_FILLER" => Some(g__STATE_FILLER()),
            "_STATE_TAIL" => Some(g__STATE_TAIL()),
            "_SUBJECTLESS_AUX" => Some(g__SUBJECTLESS_AUX()),
            "_SUBJECTLESS_ONLY" => Some(g__SUBJECTLESS_ONLY()),
            "_SUBJECTS" => Some(g__SUBJECTS()),
            "_SUBJECT_SAID" => Some(g__SUBJECT_SAID()),
            "_TYPED_SURPRISE" => Some(g__TYPED_SURPRISE()),
            "_WERE_ARE" => Some(g__WERE_ARE()),
            "_WERE_DOING" => Some(g__WERE_DOING()),
            "_WERE_IN_THE_PAST" => Some(g__WERE_IN_THE_PAST()),
            "_WERE_LEAD_IN" => Some(g__WERE_LEAD_IN()),
            "_WERE_PLACE" => Some(g__WERE_PLACE()),
            "_WERE_STATE" => Some(g__WERE_STATE()),
            "_WH_QUESTIONS" => Some(g__WH_QUESTIONS()),
            "_a_clip_kept_only_with_its_cue" => Some(py_func("_a_clip_kept_only_with_its_cue", |args: &[Py]| _a_clip_kept_only_with_its_cue(args))),
            "_a_consonant_left_out_of_an_ing_word" => Some(py_func("_a_consonant_left_out_of_an_ing_word", |args: &[Py]| _a_consonant_left_out_of_an_ing_word(args))),
            "_a_contraction_typed_with_a_space" => Some(py_func("_a_contraction_typed_with_a_space", |args: &[Py]| _a_contraction_typed_with_a_space(args))),
            "_a_core_verb_typed_wrong" => Some(py_func("_a_core_verb_typed_wrong", |args: &[Py]| _a_core_verb_typed_wrong(args))),
            "_a_day_named_as_not_mine" => Some(py_func("_a_day_named_as_not_mine", |args: &[Py]| _a_day_named_as_not_mine(args))),
            "_a_doing" => Some(py_func("_a_doing", |args: &[Py]| _a_doing(args))),
            "_a_doubled_letter_typed_once" => Some(py_func("_a_doubled_letter_typed_once", |args: &[Py]| _a_doubled_letter_typed_once(args))),
            "_a_doubtful_split" => Some(py_func("_a_doubtful_split", |args: &[Py]| _a_doubtful_split(args))),
            "_a_game_named_another_way" => Some(py_func("_a_game_named_another_way", |args: &[Py]| _a_game_named_another_way(args))),
            "_a_gerund_reads_as_a_verb" => Some(py_func("_a_gerund_reads_as_a_verb", |args: &[Py]| _a_gerund_reads_as_a_verb(args))),
            "_a_hello_said_short" => Some(py_func("_a_hello_said_short", |args: &[Py]| _a_hello_said_short(args))),
            "_a_leading_h" => Some(py_func("_a_leading_h", |args: &[Py]| _a_leading_h(args))),
            "_a_leading_like_is_a_filler" => Some(py_func("_a_leading_like_is_a_filler", |args: &[Py]| _a_leading_like_is_a_filler(args))),
            "_a_letter_apart" => Some(py_func("_a_letter_apart", |args: &[Py]| _a_letter_apart(args))),
            "_a_letter_typed_twice" => Some(py_func("_a_letter_typed_twice", |args: &[Py]| _a_letter_typed_twice(args))),
            "_a_letter_wrong_in_a_state" => Some(py_func("_a_letter_wrong_in_a_state", |args: &[Py]| _a_letter_wrong_in_a_state(args))),
            "_a_meal_word_misspelt" => Some(py_func("_a_meal_word_misspelt", |args: &[Py]| _a_meal_word_misspelt(args))),
            "_a_measure_spelt_wrong" => Some(py_func("_a_measure_spelt_wrong", |args: &[Py]| _a_measure_spelt_wrong(args))),
            "_a_name_slot" => Some(py_func("_a_name_slot", |args: &[Py]| _a_name_slot(args))),
            "_a_number" => Some(py_func("_a_number", |args: &[Py]| _a_number(args))),
            "_a_place_said_whole" => Some(py_func("_a_place_said_whole", |args: &[Py]| _a_place_said_whole(args))),
            "_a_pronoun_said_together" => Some(py_func("_a_pronoun_said_together", |args: &[Py]| _a_pronoun_said_together(args))),
            "_a_run_and_what_surrounds_it" => Some(py_func("_a_run_and_what_surrounds_it", |args: &[Py]| _a_run_and_what_surrounds_it(args))),
            "_a_said_for_an_ending" => Some(py_func("_a_said_for_an_ending", |args: &[Py]| _a_said_for_an_ending(args))),
            "_a_short_word_named" => Some(py_func("_a_short_word_named", |args: &[Py]| _a_short_word_named(args))),
            "_a_sign_is_a_word" => Some(py_func("_a_sign_is_a_word", |args: &[Py]| _a_sign_is_a_word(args))),
            "_a_slip_that_is_a_word" => Some(py_func("_a_slip_that_is_a_word", |args: &[Py]| _a_slip_that_is_a_word(args))),
            "_a_state_typed_onto_im" => Some(py_func("_a_state_typed_onto_im", |args: &[Py]| _a_state_typed_onto_im(args))),
            "_a_thing_a_letter_short" => Some(py_func("_a_thing_a_letter_short", |args: &[Py]| _a_thing_a_letter_short(args))),
            "_a_thing_where_a_thing_goes" => Some(py_func("_a_thing_where_a_thing_goes", |args: &[Py]| _a_thing_where_a_thing_goes(args))),
            "_a_vowel_left_out_of_a_state" => Some(py_func("_a_vowel_left_out_of_a_state", |args: &[Py]| _a_vowel_left_out_of_a_state(args))),
            "_a_word_between_the_ask_and_its_aux" => Some(py_func("_a_word_between_the_ask_and_its_aux", |args: &[Py]| _a_word_between_the_ask_and_its_aux(args))),
            "_a_word_of_the_bots" => Some(py_func("_a_word_of_the_bots", |args: &[Py]| _a_word_of_the_bots(args))),
            "_a_word_the_bot_writes" => Some(py_func("_a_word_the_bot_writes", |args: &[Py]| _a_word_the_bot_writes(args))),
            "_a_word_the_reading_knows" => Some(py_func("_a_word_the_reading_knows", |args: &[Py]| _a_word_the_reading_knows(args))),
            "_ache_for_aike" => Some(py_func("_ache_for_aike", |args: &[Py]| _ache_for_aike(args))),
            "_an_emergency_word_left_out" => Some(py_func("_an_emergency_word_left_out", |args: &[Py]| _an_emergency_word_left_out(args))),
            "_an_exclamation_in_front" => Some(py_func("_an_exclamation_in_front", |args: &[Py]| _an_exclamation_in_front(args))),
            "_an_exclamation_is_not_a_thing" => Some(py_func("_an_exclamation_is_not_a_thing", |args: &[Py]| _an_exclamation_is_not_a_thing(args))),
            "_an_ill_that_is_i_will" => Some(py_func("_an_ill_that_is_i_will", |args: &[Py]| _an_ill_that_is_i_will(args))),
            "_an_ing_said_as_n" => Some(py_func("_an_ing_said_as_n", |args: &[Py]| _an_ing_said_as_n(args))),
            "_an_introduction" => Some(py_func("_an_introduction", |args: &[Py]| _an_introduction(args))),
            "_apostrophe" => Some(py_func("_apostrophe", |args: &[Py]| _apostrophe(args))),
            "_ask_start" => Some(py_func("_ask_start", |args: &[Py]| _ask_start(args))),
            "_asking" => Some(py_func("_asking", |args: &[Py]| _asking(args))),
            "_asks_the_spelling" => Some(py_func("_asks_the_spelling", |args: &[Py]| _asks_the_spelling(args))),
            "_bones" => Some(py_func("_bones", |args: &[Py]| _bones(args))),
            "_can_not_as_one_word" => Some(py_func("_can_not_as_one_word", |args: &[Py]| _can_not_as_one_word(args))),
            "_cause_in_place_is_the_verb" => Some(py_func("_cause_in_place_is_the_verb", |args: &[Py]| _cause_in_place_is_the_verb(args))),
            "_change_of_subject_off" => Some(py_func("_change_of_subject_off", |args: &[Py]| _change_of_subject_off(args))),
            "_chat_words_said_out" => Some(py_func("_chat_words_said_out", |args: &[Py]| _chat_words_said_out(args))),
            "_clause_stop" => Some(py_func("_clause_stop", |args: &[Py]| _clause_stop(args))),
            "_clauses" => Some(py_func("_clauses", |args: &[Py]| _clauses(args))),
            "_clipped_thing" => Some(py_func("_clipped_thing", |args: &[Py]| _clipped_thing(args))),
            "_collapse" => Some(py_func("_collapse", |args: &[Py]| _collapse(args))),
            "_counts_a_compound" => Some(py_func("_counts_a_compound", |args: &[Py]| _counts_a_compound(args))),
            "_counts_a_period" => Some(py_func("_counts_a_period", |args: &[Py]| _counts_a_period(args))),
            "_counts_a_thing" => Some(py_func("_counts_a_thing", |args: &[Py]| _counts_a_thing(args))),
            "_digit_word" => Some(py_func("_digit_word", |args: &[Py]| _digit_word(args))),
            "_docs_doctor" => Some(py_func("_docs_doctor", |args: &[Py]| _docs_doctor(args))),
            "_docs_on_a_screen" => Some(py_func("_docs_on_a_screen", |args: &[Py]| _docs_on_a_screen(args))),
            "_double_variants" => Some(py_func("_double_variants", |args: &[Py]| _double_variants(args))),
            "_doubled_letter_words" => Some(py_func("_doubled_letter_words", |args: &[Py]| _doubled_letter_words(args))),
            "_doubles_its_last_letter" => Some(py_func("_doubles_its_last_letter", |args: &[Py]| _doubles_its_last_letter(args))),
            "_dropped_g" => Some(py_func("_dropped_g", |args: &[Py]| _dropped_g(args))),
            "_dropped_letter" => Some(py_func("_dropped_letter", |args: &[Py]| _dropped_letter(args))),
            "_drops_a_final_e" => Some(py_func("_drops_a_final_e", |args: &[Py]| _drops_a_final_e(args))),
            "_e_kept_before_ing" => Some(py_func("_e_kept_before_ing", |args: &[Py]| _e_kept_before_ing(args))),
            "_ends_a_state" => Some(py_func("_ends_a_state", |args: &[Py]| _ends_a_state(args))),
            "_extra_vowel" => Some(py_func("_extra_vowel", |args: &[Py]| _extra_vowel(args))),
            "_family_put_back" => Some(py_func("_family_put_back", |args: &[Py]| _family_put_back(args))),
            "_family_short" => Some(py_func("_family_short", |args: &[Py]| _family_short(args))),
            "_feeling_without_its_ending" => Some(py_func("_feeling_without_its_ending", |args: &[Py]| _feeling_without_its_ending(args))),
            "_fix_auxiliaries" => Some(py_func("_fix_auxiliaries", |args: &[Py]| _fix_auxiliaries(args))),
            "_focus" => Some(py_func("_focus", |args: &[Py]| _focus(args))),
            "_four_spells_for" => Some(py_func("_four_spells_for", |args: &[Py]| _four_spells_for(args))),
            "_glued_said_apart" => Some(py_func("_glued_said_apart", |args: &[Py]| _glued_said_apart(args))),
            "_glued_words" => Some(py_func("_glued_words", |args: &[Py]| _glued_words(args))),
            "_hr_is_the_department" => Some(py_func("_hr_is_the_department", |args: &[Py]| _hr_is_the_department(args))),
            "_ie_variants" => Some(py_func("_ie_variants", |args: &[Py]| _ie_variants(args))),
            "_ill_is_i_will_going" => Some(py_func("_ill_is_i_will_going", |args: &[Py]| _ill_is_i_will_going(args))),
            "_inflections" => Some(py_func("_inflections", |args: &[Py]| _inflections(args))),
            "_ing_form" => Some(py_func("_ing_form", |args: &[Py]| _ing_form(args))),
            "_inverted_ask" => Some(py_func("_inverted_ask", |args: &[Py]| _inverted_ask(args))),
            "_is_a_count" => Some(py_func("_is_a_count", |args: &[Py]| _is_a_count(args))),
            "_is_a_thing_here" => Some(py_func("_is_a_thing_here", |args: &[Py]| _is_a_thing_here(args))),
            "_is_be" => Some(py_func("_is_be", |args: &[Py]| _is_be(args))),
            "_is_before_a_noun" => Some(py_func("_is_before_a_noun", |args: &[Py]| _is_before_a_noun(args))),
            "_is_doing_something" => Some(py_func("_is_doing_something", |args: &[Py]| _is_doing_something(args))),
            "_is_participle" => Some(py_func("_is_participle", |args: &[Py]| _is_participle(args))),
            "_is_person" => Some(py_func("_is_person", |args: &[Py]| _is_person(args))),
            "_is_possessive" => Some(py_func("_is_possessive", |args: &[Py]| _is_possessive(args))),
            "_is_said_of_a_person" => Some(py_func("_is_said_of_a_person", |args: &[Py]| _is_said_of_a_person(args))),
            "_its_vowels_typed_wrong" => Some(py_func("_its_vowels_typed_wrong", |args: &[Py]| _its_vowels_typed_wrong(args))),
            "_join_chat_digits" => Some(py_func("_join_chat_digits", |args: &[Py]| _join_chat_digits(args))),
            "_joined_words" => Some(py_func("_joined_words", |args: &[Py]| _joined_words(args))),
            "_keeps_its_e_before_ing" => Some(py_func("_keeps_its_e_before_ing", |args: &[Py]| _keeps_its_e_before_ing(args))),
            "_kept_for" => Some(py_func("_kept_for", |args: &[Py]| _kept_for(args))),
            "_kin_needs_a_helper" => Some(py_func("_kin_needs_a_helper", |args: &[Py]| _kin_needs_a_helper(args))),
            "_known_word" => Some(py_func("_known_word", |args: &[Py]| _known_word(args))),
            "_left_off_contraction" => Some(py_func("_left_off_contraction", |args: &[Py]| _left_off_contraction(args))),
            "_longest_consonant_run" => Some(py_func("_longest_consonant_run", |args: &[Py]| _longest_consonant_run(args))),
            "_lookahead" => Some(py_func("_lookahead", |args: &[Py]| _lookahead(args))),
            "_me_for_my_pass" => Some(py_func("_me_for_my_pass", |args: &[Py]| _me_for_my_pass(args))),
            "_measures_a_state" => Some(py_func("_measures_a_state", |args: &[Py]| _measures_a_state(args))),
            "_missing_e" => Some(py_func("_missing_e", |args: &[Py]| _missing_e(args))),
            "_missing_vowel" => Some(py_func("_missing_vowel", |args: &[Py]| _missing_vowel(args))),
            "_names_a_letter_beside_a_slip" => Some(py_func("_names_a_letter_beside_a_slip", |args: &[Py]| _names_a_letter_beside_a_slip(args))),
            "_near_a_feeling" => Some(py_func("_near_a_feeling", |args: &[Py]| _near_a_feeling(args))),
            "_near_a_meal" => Some(py_func("_near_a_meal", |args: &[Py]| _near_a_meal(args))),
            "_near_a_part" => Some(py_func("_near_a_part", |args: &[Py]| _near_a_part(args))),
            "_once_only" => Some(py_func("_once_only", |args: &[Py]| _once_only(args))),
            "_one_letter_apart" => Some(py_func("_one_letter_apart", |args: &[Py]| _one_letter_apart(args))),
            "_one_letter_wrong" => Some(py_func("_one_letter_wrong", |args: &[Py]| _one_letter_wrong(args))),
            "_one_of_the_parts" => Some(py_func("_one_of_the_parts", |args: &[Py]| _one_of_the_parts(args))),
            "_one_relation_says" => Some(py_func("_one_relation_says", |args: &[Py]| _one_relation_says(args))),
            "_one_vowel_less" => Some(py_func("_one_vowel_less", |args: &[Py]| _one_vowel_less(args))),
            "_one_word_food" => Some(py_func("_one_word_food", |args: &[Py]| _one_word_food(args))),
            "_only_doubles_a_letter" => Some(py_func("_only_doubles_a_letter", |args: &[Py]| _only_doubles_a_letter(args))),
            "_only_leaders_before" => Some(py_func("_only_leaders_before", |args: &[Py]| _only_leaders_before(args))),
            "_opener_is" => Some(py_func("_opener_is", |args: &[Py]| _opener_is(args))),
            "_opens_a_state" => Some(py_func("_opens_a_state", |args: &[Py]| _opens_a_state(args))),
            "_opens_clause" => Some(py_func("_opens_clause", |args: &[Py]| _opens_clause(args))),
            "_parts_that_hurt" => Some(py_func("_parts_that_hurt", |args: &[Py]| _parts_that_hurt(args))),
            "_phrase_pass" => Some(py_func("_phrase_pass", |args: &[Py]| _phrase_pass(args))),
            "_plain_chat" => Some(py_func("_plain_chat", |args: &[Py]| _plain_chat(args))),
            "_plain_word" => Some(py_func("_plain_word", |args: &[Py]| _plain_word(args))),
            "_positional_word" => Some(py_func("_positional_word", |args: &[Py]| _positional_word(args))),
            "_repair_openers" => Some(py_func("_repair_openers", |args: &[Py]| _repair_openers(args))),
            "_reported" => Some(py_func("_reported", |args: &[Py]| _reported(args))),
            "_said_apart" => Some(py_func("_said_apart", |args: &[Py]| _said_apart(args))),
            "_said_apart_as_one" => Some(py_func("_said_apart_as_one", |args: &[Py]| _said_apart_as_one(args))),
            "_said_apart_as_one_word" => Some(py_func("_said_apart_as_one_word", |args: &[Py]| _said_apart_as_one_word(args))),
            "_said_run_on" => Some(py_func("_said_run_on", |args: &[Py]| _said_run_on(args))),
            "_said_with_an_f" => Some(py_func("_said_with_an_f", |args: &[Py]| _said_with_an_f(args))),
            "_said_with_an_f_anywhere" => Some(py_func("_said_with_an_f_anywhere", |args: &[Py]| _said_with_an_f_anywhere(args))),
            "_says_has" => Some(py_func("_says_has", |args: &[Py]| _says_has(args))),
            "_says_one" => Some(py_func("_says_one", |args: &[Py]| _says_one(args))),
            "_short_double" => Some(py_func("_short_double", |args: &[Py]| _short_double(args))),
            "_shun_variant" => Some(py_func("_shun_variant", |args: &[Py]| _shun_variant(args))),
            "_slip" => Some(py_func("_slip", |args: &[Py]| _slip(args))),
            "_sounds_like_a_feeling" => Some(py_func("_sounds_like_a_feeling", |args: &[Py]| _sounds_like_a_feeling(args))),
            "_spellings_of_one_word" => Some(py_func("_spellings_of_one_word", |args: &[Py]| _spellings_of_one_word(args))),
            "_spells_a_helper" => Some(py_func("_spells_a_helper", |args: &[Py]| _spells_a_helper(args))),
            "_split_ask" => Some(py_func("_split_ask", |args: &[Py]| _split_ask(args))),
            "_stems_of" => Some(py_func("_stems_of", |args: &[Py]| _stems_of(args))),
            "_stopped_th" => Some(py_func("_stopped_th", |args: &[Py]| _stopped_th(args))),
            "_sum_word" => Some(py_func("_sum_word", |args: &[Py]| _sum_word(args))),
            "_swapped_pair" => Some(py_func("_swapped_pair", |args: &[Py]| _swapped_pair(args))),
            "_the_ask_said_plainer" => Some(py_func("_the_ask_said_plainer", |args: &[Py]| _the_ask_said_plainer(args))),
            "_the_day_left_out" => Some(py_func("_the_day_left_out", |args: &[Py]| _the_day_left_out(args))),
            "_the_feeling_word" => Some(py_func("_the_feeling_word", |args: &[Py]| _the_feeling_word(args))),
            "_the_figure" => Some(py_func("_the_figure", |args: &[Py]| _the_figure(args))),
            "_the_measure_clause" => Some(py_func("_the_measure_clause", |args: &[Py]| _the_measure_clause(args))),
            "_the_mrs_is_the_wife" => Some(py_func("_the_mrs_is_the_wife", |args: &[Py]| _the_mrs_is_the_wife(args))),
            "_the_part_that_hurts" => Some(py_func("_the_part_that_hurts", |args: &[Py]| _the_part_that_hurts(args))),
            "_the_part_that_is_bad" => Some(py_func("_the_part_that_is_bad", |args: &[Py]| _the_part_that_is_bad(args))),
            "_the_said_apart_forms" => Some(py_func("_the_said_apart_forms", |args: &[Py]| _the_said_apart_forms(args))),
            "_the_state_is_the_whole_frame" => Some(py_func("_the_state_is_the_whole_frame", |args: &[Py]| _the_state_is_the_whole_frame(args))),
            "_the_sugar_that_is_blood" => Some(py_func("_the_sugar_that_is_blood", |args: &[Py]| _the_sugar_that_is_blood(args))),
            "_the_surprise_typed" => Some(py_func("_the_surprise_typed", |args: &[Py]| _the_surprise_typed(args))),
            "_the_thing_has_been" => Some(py_func("_the_thing_has_been", |args: &[Py]| _the_thing_has_been(args))),
            "_the_ways_a_word_is_typed_apart" => Some(py_func("_the_ways_a_word_is_typed_apart", |args: &[Py]| _the_ways_a_word_is_typed_apart(args))),
            "_three_of_a_letter" => Some(py_func("_three_of_a_letter", |args: &[Py]| _three_of_a_letter(args))),
            "_token" => Some(py_func("_token", |args: &[Py]| _token(args))),
            "_two_letters_out" => Some(py_func("_two_letters_out", |args: &[Py]| _two_letters_out(args))),
            "_two_letters_the_wrong_way_round" => Some(py_func("_two_letters_the_wrong_way_round", |args: &[Py]| _two_letters_the_wrong_way_round(args))),
            "_typed_without_spaces" => Some(py_func("_typed_without_spaces", |args: &[Py]| _typed_without_spaces(args))),
            "_unaccent" => Some(py_func("_unaccent", |args: &[Py]| _unaccent(args))),
            "_unsays_the_word" => Some(py_func("_unsays_the_word", |args: &[Py]| _unsays_the_word(args))),
            "_were_for_another_word" => Some(py_func("_were_for_another_word", |args: &[Py]| _were_for_another_word(args))),
            "_wers_word" => Some(py_func("_wers_word", |args: &[Py]| _wers_word(args))),
            "_where_a_state_stands" => Some(py_func("_where_a_state_stands", |args: &[Py]| _where_a_state_stands(args))),
            "_without_a_tag_at_the_end" => Some(py_func("_without_a_tag_at_the_end", |args: &[Py]| _without_a_tag_at_the_end(args))),
            "_without_the_intensifier" => Some(py_func("_without_the_intensifier", |args: &[Py]| _without_the_intensifier(args))),
            "_words_the_bot_writes" => Some(py_func("_words_the_bot_writes", |args: &[Py]| _words_the_bot_writes(args))),
            "_z_for_s" => Some(py_func("_z_for_s", |args: &[Py]| _z_for_s(args))),
            "a_son_not_the_sun" => Some(py_func("a_son_not_the_sun", |args: &[Py]| a_son_not_the_sun(args))),
            "a_sor_bit_is_sore" => Some(py_func("a_sor_bit_is_sore", |args: &[Py]| a_sor_bit_is_sore(args))),
            "background_clause" => Some(py_func("background_clause", |args: &[Py]| background_clause(args))),
            "background_positions" => Some(py_func("background_positions", |args: &[Py]| background_positions(args))),
            "fix_opener" => Some(py_func("fix_opener", |args: &[Py]| fix_opener(args))),
            "fix_typo" => Some(py_func("fix_typo", |args: &[Py]| fix_typo(args))),
            "glued_words" => Some(py_func("glued_words", |args: &[Py]| glued_words(args))),
            "intensifier_positions" => Some(py_func("intensifier_positions", |args: &[Py]| intensifier_positions(args))),
            "learn_thing_words" => Some(py_func("learn_thing_words", |args: &[Py]| learn_thing_words(args))),
            "loose_is_lose" => Some(py_func("loose_is_lose", |args: &[Py]| loose_is_lose(args))),
            "normalise" => Some(py_func("normalise", |args: &[Py]| normalise(args))),
            "opens_without_subject" => Some(py_func("opens_without_subject", |args: &[Py]| opens_without_subject(args))),
            "possessive_me" => Some(py_func("possessive_me", |args: &[Py]| possessive_me(args))),
            "reads_as_a_feeling" => Some(py_func("reads_as_a_feeling", |args: &[Py]| reads_as_a_feeling(args))),
            "reads_as_a_sound" => Some(py_func("reads_as_a_sound", |args: &[Py]| reads_as_a_sound(args))),
            "reason_positions" => Some(py_func("reason_positions", |args: &[Py]| reason_positions(args))),
            "repaired_tokens" => Some(py_func("repaired_tokens", |args: &[Py]| repaired_tokens(args))),
            "respell_urgent" => Some(py_func("respell_urgent", |args: &[Py]| respell_urgent(args))),
            "said_apart" => Some(py_func("said_apart", |args: &[Py]| said_apart(args))),
            "spelled" => Some(py_func("spelled", |args: &[Py]| spelled(args))),
            "strip_leaders" => Some(py_func("strip_leaders", |args: &[Py]| strip_leaders(args))),
            "subject_missing" => Some(py_func("subject_missing", |args: &[Py]| subject_missing(args))),
            "the_feeling_two_letters_are_gone_from" => Some(py_func("the_feeling_two_letters_are_gone_from", |args: &[Py]| the_feeling_two_letters_are_gone_from(args))),
            "without_a_leading_exclamation" => Some(py_func("without_a_leading_exclamation", |args: &[Py]| without_a_leading_exclamation(args))),
            "without_leaders" => Some(py_func("without_leaders", |args: &[Py]| without_leaders(args))),
            "without_the_figure_of_speech" => Some(py_func("without_the_figure_of_speech", |args: &[Py]| without_the_figure_of_speech(args))),
            "lexicon" => Some(pyrt::xrefs::xmodule("understanding.lexicon")),
            "itertools" => Some(stdlib_module("itertools")),
            "re" => Some(stdlib_module("re")),
            "unicodedata" => Some(stdlib_module("unicodedata")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THE_FORM" => s_ABOUT_THE_FORM(v),
            "AEIOU" => s_AEIOU(v),
            "AFTER_AN_INVERTED_AUX" => s_AFTER_AN_INVERTED_AUX(v),
            "AFTER_A_STATE" => s_AFTER_A_STATE(v),
            "AFTER_A_THING" => s_AFTER_A_THING(v),
            "AFTER_A_TIME" => s_AFTER_A_TIME(v),
            "AFTER_A_VERB" => s_AFTER_A_VERB(v),
            "AINT_FORMS" => s_AINT_FORMS(v),
            "AINT_PLURAL" => s_AINT_PLURAL(v),
            "AMOUNT_BEFORE_A_STATE" => s_AMOUNT_BEFORE_A_STATE(v),
            "APOSTROPHE_BASES" => s_APOSTROPHE_BASES(v),
            "APOSTROPHE_NOT" => s_APOSTROPHE_NOT(v),
            "APOSTROPHE_TAILS" => s_APOSTROPHE_TAILS(v),
            "ASKS_WHOSE" => s_ASKS_WHOSE(v),
            "ASK_NOUNS" => s_ASK_NOUNS(v),
            "ASK_WORDS" => s_ASK_WORDS(v),
            "AUX_OPENS" => s_AUX_OPENS(v),
            "AUX_WORDS" => s_AUX_WORDS(v),
            "A_CHOKE_SAID_THE_OTHER_WAY" => s_A_CHOKE_SAID_THE_OTHER_WAY(v),
            "A_COPULA" => s_A_COPULA(v),
            "A_COUNT" => s_A_COUNT(v),
            "A_FEELING_CAN_BE_FOLLOWED" => s_A_FEELING_CAN_BE_FOLLOWED(v),
            "A_GLASS_OR_A_PAN" => s_A_GLASS_OR_A_PAN(v),
            "A_JOB" => s_A_JOB(v),
            "A_KEEPING_A_THING" => s_A_KEEPING_A_THING(v),
            "A_LABEL_BEFORE" => s_A_LABEL_BEFORE(v),
            "A_LETTER_AFTER" => s_A_LETTER_AFTER(v),
            "A_NEGATION_BEFORE" => s_A_NEGATION_BEFORE(v),
            "A_NUMBER_OF_SOMEONES_DAY" => s_A_NUMBER_OF_SOMEONES_DAY(v),
            "A_PAIN_WORD" => s_A_PAIN_WORD(v),
            "A_PARTS_OF_A_PERIOD" => s_A_PARTS_OF_A_PERIOD(v),
            "A_PART_A_PERSON_OWNS" => s_A_PART_A_PERSON_OWNS(v),
            "A_PART_THE_PAIN_IS_IN" => s_A_PART_THE_PAIN_IS_IN(v),
            "A_PERIOD_FOLLOWS" => s_A_PERIOD_FOLLOWS(v),
            "A_PRONOUN_GLUED" => s_A_PRONOUN_GLUED(v),
            "A_PRONOUN_STATE" => s_A_PRONOUN_STATE(v),
            "A_SIDE" => s_A_SIDE(v),
            "A_STATE_THEN_A_TIME" => s_A_STATE_THEN_A_TIME(v),
            "A_STATE_YOU_CAN_FEEL" => s_A_STATE_YOU_CAN_FEEL(v),
            "A_THING_GOES_AFTER" => s_A_THING_GOES_AFTER(v),
            "A_THING_TYPED_AS_A_LITTLE_WORD" => s_A_THING_TYPED_AS_A_LITTLE_WORD(v),
            "A_THING_WORN" => s_A_THING_WORN(v),
            "A_TIME_TO_LEAVE" => s_A_TIME_TO_LEAVE(v),
            "A_VERB_CAN_FOLLOW" => s_A_VERB_CAN_FOLLOW(v),
            "A_WORD_CAN_FOLLOW" => s_A_WORD_CAN_FOLLOW(v),
            "BAD_SAYING" => s_BAD_SAYING(v),
            "BAD_WORD" => s_BAD_WORD(v),
            "BARE_TO" => s_BARE_TO(v),
            "BASE" => s_BASE(v),
            "BEEN_SUBJECT" => s_BEEN_SUBJECT(v),
            "BEEN_WORDS" => s_BEEN_WORDS(v),
            "BEFORE_A_FEELING" => s_BEFORE_A_FEELING(v),
            "BEFORE_A_JOB" => s_BEFORE_A_JOB(v),
            "BEFORE_A_PREPOSITION" => s_BEFORE_A_PREPOSITION(v),
            "BEFORE_A_THING" => s_BEFORE_A_THING(v),
            "BEFORE_A_VERB" => s_BEFORE_A_VERB(v),
            "BEFORE_THE_LOSING" => s_BEFORE_THE_LOSING(v),
            "BEFORE_VERB" => s_BEFORE_VERB(v),
            "BEFORE_WORD" => s_BEFORE_WORD(v),
            "BE_BEFORE_A_STATE" => s_BE_BEFORE_A_STATE(v),
            "BE_FORMS" => s_BE_FORMS(v),
            "BE_VERBS" => s_BE_VERBS(v),
            "BE_WORDS" => s_BE_WORDS(v),
            "BIN_CONTAINERS" => s_BIN_CONTAINERS(v),
            "BREAKS" => s_BREAKS(v),
            "CAN_FOLLOW_A_VERB" => s_CAN_FOLLOW_A_VERB(v),
            "CAN_NOT" => s_CAN_NOT(v),
            "CAUSES_A_STATE" => s_CAUSES_A_STATE(v),
            "CAUSE_A_CLAUSE_VERB" => s_CAUSE_A_CLAUSE_VERB(v),
            "CAUSE_A_DEMONSTRATIVE" => s_CAUSE_A_DEMONSTRATIVE(v),
            "CAUSE_A_SUBJECT_FOLLOWS" => s_CAUSE_A_SUBJECT_FOLLOWS(v),
            "CAUSE_BEFORE_A_NOUN" => s_CAUSE_BEFORE_A_NOUN(v),
            "CAUSE_BEFORE_A_VERB" => s_CAUSE_BEFORE_A_VERB(v),
            "CHANGE_OF_SUBJECT" => s_CHANGE_OF_SUBJECT(v),
            "CHAT_AS_ONE_WORD" => s_CHAT_AS_ONE_WORD(v),
            "CHAT_KEPT_AS_TYPED" => s_CHAT_KEPT_AS_TYPED(v),
            "CHAT_NAMES" => s_CHAT_NAMES(v),
            "CLAUSE_OPENERS" => s_CLAUSE_OPENERS(v),
            "CLIPPED_FAMILY" => s_CLIPPED_FAMILY(v),
            "CLIPPED_THINGS" => s_CLIPPED_THINGS(v),
            "CLIPPED_UNITS" => s_CLIPPED_UNITS(v),
            "COMMON_RANK" => s_COMMON_RANK(v),
            "COMPOUND_HEADS" => s_COMPOUND_HEADS(v),
            "COMPOUND_TAILS" => s_COMPOUND_TAILS(v),
            "CONTRACTION_NEXT" => s_CONTRACTION_NEXT(v),
            "COPULAS" => s_COPULAS(v),
            "CORE_VERBS" => s_CORE_VERBS(v),
            "COUNTS" => s_COUNTS(v),
            "COUNT_UNITS" => s_COUNT_UNITS(v),
            "CURRENCY_FACTOR" => s_CURRENCY_FACTOR(v),
            "CURRENCY_SIGN" => s_CURRENCY_SIGN(v),
            "CURRENCY_SIGNS" => s_CURRENCY_SIGNS(v),
            "DAY_NAMED" => s_DAY_NAMED(v),
            "DEMONSTRATIVE" => s_DEMONSTRATIVE(v),
            "DETERMINERS" => s_DETERMINERS(v),
            "DETERMINER_IN_FRONT" => s_DETERMINER_IN_FRONT(v),
            "DIGIT_AS_WORD" => s_DIGIT_AS_WORD(v),
            "DIGIT_WORDS" => s_DIGIT_WORDS(v),
            "DOCS_ON" => s_DOCS_ON(v),
            "DOCS_PAPER" => s_DOCS_PAPER(v),
            "DOCS_SCREEN" => s_DOCS_SCREEN(v),
            "DOCS_VISIT" => s_DOCS_VISIT(v),
            "DOING_NOT_THING" => s_DOING_NOT_THING(v),
            "DROPPED_SUBJECT_AUX" => s_DROPPED_SUBJECT_AUX(v),
            "EXCLAIMED_AFTER_A_QUESTION" => s_EXCLAIMED_AFTER_A_QUESTION(v),
            "EXCLAMATION_OPENS" => s_EXCLAMATION_OPENS(v),
            "EXCLAMATION_PHRASE" => s_EXCLAMATION_PHRASE(v),
            "E_KEPT_WORDS" => s_E_KEPT_WORDS(v),
            "FAMILY_SHRT" => s_FAMILY_SHRT(v),
            "FEELING_BEFORE_SICK" => s_FEELING_BEFORE_SICK(v),
            "FINDING_A_TEAM" => s_FINDING_A_TEAM(v),
            "FIRST_PERSON_BE" => s_FIRST_PERSON_BE(v),
            "FOOD_SLOT_LEAD" => s_FOOD_SLOT_LEAD(v),
            "FOR_A" => s_FOR_A(v),
            "FOR_MEAL" => s_FOR_MEAL(v),
            "FOR_OBJECT" => s_FOR_OBJECT(v),
            "FREQUENT" => s_FREQUENT(v),
            "F_TYPED_FOR_TH" => s_F_TYPED_FOR_TH(v),
            "GLUED_AFTER_A_DO" => s_GLUED_AFTER_A_DO(v),
            "GLUED_COLLAPSE" => s_GLUED_COLLAPSE(v),
            "GLUED_DO" => s_GLUED_DO(v),
            "GLUED_LEFT" => s_GLUED_LEFT(v),
            "GLUED_RIGHT" => s_GLUED_RIGHT(v),
            "HAS_AFTER" => s_HAS_AFTER(v),
            "HAS_FORMS" => s_HAS_FORMS(v),
            "HAS_SUBJECT" => s_HAS_SUBJECT(v),
            "HAS_SUBJECTS" => s_HAS_SUBJECTS(v),
            "HAVE_FORMS" => s_HAVE_FORMS(v),
            "HEART_FEELINGS" => s_HEART_FEELINGS(v),
            "HEART_MEASURES" => s_HEART_MEASURES(v),
            "HEART_SAID_AS_A_FEELING" => s_HEART_SAID_AS_A_FEELING(v),
            "HELLO_SAID_SHORT" => s_HELLO_SAID_SHORT(v),
            "HELLO_TAIL" => s_HELLO_TAIL(v),
            "HOW_DEGREE" => s_HOW_DEGREE(v),
            "HOW_FOLLOWS" => s_HOW_FOLLOWS(v),
            "HR_IS_THE_DEPARTMENT_AFTER" => s_HR_IS_THE_DEPARTMENT_AFTER(v),
            "HV_HELPERS" => s_HV_HELPERS(v),
            "HV_SUBJECTS" => s_HV_SUBJECTS(v),
            "ILL_GOES_ON" => s_ILL_GOES_ON(v),
            "ING_THINGS" => s_ING_THINGS(v),
            "INTRODUCING_AFTER" => s_INTRODUCING_AFTER(v),
            "INTRODUCING_A_PERSON" => s_INTRODUCING_A_PERSON(v),
            "IN_NOUNS" => s_IN_NOUNS(v),
            "IRREGULAR_PARTICIPLES" => s_IRREGULAR_PARTICIPLES(v),
            "I_GLUED" => s_I_GLUED(v),
            "JOINED_WORDS" => s_JOINED_WORDS(v),
            "KIN" => s_KIN(v),
            "KIN_FILLERS" => s_KIN_FILLERS(v),
            "KIN_HAS" => s_KIN_HAS(v),
            "KIN_IS" => s_KIN_IS(v),
            "KIN_SINGULAR" => s_KIN_SINGULAR(v),
            "KNOW_SUBJECT" => s_KNOW_SUBJECT(v),
            "LEADERS" => s_LEADERS(v),
            "LEFT_OFF_CONTRACTION" => s_LEFT_OFF_CONTRACTION(v),
            "LETTERS" => s_LETTERS(v),
            "LETTER_C_IS_SEE" => s_LETTER_C_IS_SEE(v),
            "LEXICON" => s_LEXICON(v),
            "LIKE_STARTS_A_CLAUSE" => s_LIKE_STARTS_A_CLAUSE(v),
            "LONG_ENOUGH_TO_BE_A_VERB" => s_LONG_ENOUGH_TO_BE_A_VERB(v),
            "LOOSE_FOR_LOSE" => s_LOOSE_FOR_LOSE(v),
            "MEAL_WORDS" => s_MEAL_WORDS(v),
            "ME_FOR_MY_CLOSES" => s_ME_FOR_MY_CLOSES(v),
            "MISSING_E_AFTER" => s_MISSING_E_AFTER(v),
            "MI_BACK_VERBS" => s_MI_BACK_VERBS(v),
            "MI_IS_A_NAME_AFTER" => s_MI_IS_A_NAME_AFTER(v),
            "MI_IS_ME_AFTER" => s_MI_IS_ME_AFTER(v),
            "MI_IS_ME_BEFORE" => s_MI_IS_ME_BEFORE(v),
            "MODAL_FORMS" => s_MODAL_FORMS(v),
            "MOTION_BEFORE" => s_MOTION_BEFORE(v),
            "MRS_IS_THE_WIFE" => s_MRS_IS_THE_WIFE(v),
            "NAMED_LETTER" => s_NAMED_LETTER(v),
            "NAME_CALLS" => s_NAME_CALLS(v),
            "NAME_FRAMES" => s_NAME_FRAMES(v),
            "NAME_ROLL" => s_NAME_ROLL(v),
            "NOMINAL_BEFORE_COPULA" => s_NOMINAL_BEFORE_COPULA(v),
            "NOT_A_COMPOUND_HEAD" => s_NOT_A_COMPOUND_HEAD(v),
            "NOT_A_COMPOUND_TAIL" => s_NOT_A_COMPOUND_TAIL(v),
            "NOT_A_MEAL_TYPO" => s_NOT_A_MEAL_TYPO(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_A_THING_AFTER" => s_NOT_A_THING_AFTER(v),
            "NOT_A_THING_ENDING" => s_NOT_A_THING_ENDING(v),
            "NOT_A_THING_PAIR" => s_NOT_A_THING_PAIR(v),
            "NOT_BEFORE_A_NOUN" => s_NOT_BEFORE_A_NOUN(v),
            "NOT_BEFORE_KNOW" => s_NOT_BEFORE_KNOW(v),
            "NOT_COUNTED_AFTER" => s_NOT_COUNTED_AFTER(v),
            "NOT_IN_A_QUESTION_PHRASE" => s_NOT_IN_A_QUESTION_PHRASE(v),
            "NOT_MY_OWN_DAY" => s_NOT_MY_OWN_DAY(v),
            "NOT_WORDS" => s_NOT_WORDS(v),
            "NO_ASK_AFTER" => s_NO_ASK_AFTER(v),
            "NO_SWAP_TO" => s_NO_SWAP_TO(v),
            "NUMBERS_BEFORE" => s_NUMBERS_BEFORE(v),
            "OBJECT_PRONOUN" => s_OBJECT_PRONOUN(v),
            "ONE_OF_A_KIND" => s_ONE_OF_A_KIND(v),
            "ONE_PART_AFTER" => s_ONE_PART_AFTER(v),
            "ONE_RELATION" => s_ONE_RELATION(v),
            "ONE_RELATION_VERBS" => s_ONE_RELATION_VERBS(v),
            "ONE_WORD_FOODS" => s_ONE_WORD_FOODS(v),
            "OPENERS" => s_OPENERS(v),
            "OPERATORS" => s_OPERATORS(v),
            "OWNER_WITHOUT_THE_DAY" => s_OWNER_WITHOUT_THE_DAY(v),
            "OWNING_A_PERSON" => s_OWNING_A_PERSON(v),
            "OWNING_VERBS" => s_OWNING_VERBS(v),
            "PART_REACH" => s_PART_REACH(v),
            "PART_THAT_IS_BAD" => s_PART_THAT_IS_BAD(v),
            "PAST_AS_PARTICIPLE" => s_PAST_AS_PARTICIPLE(v),
            "PERIOD_COUNTS" => s_PERIOD_COUNTS(v),
            "PERSON_PREPS" => s_PERSON_PREPS(v),
            "PERSON_WORDS" => s_PERSON_WORDS(v),
            "PHRASES" => s_PHRASES(v),
            "PHRASE_MEASURES" => s_PHRASE_MEASURES(v),
            "PLACE_WORDS" => s_PLACE_WORDS(v),
            "PLURAL_SUBJECTS" => s_PLURAL_SUBJECTS(v),
            "POSSESSIVES" => s_POSSESSIVES(v),
            "POSSESSIVE_IN_FRONT" => s_POSSESSIVE_IN_FRONT(v),
            "PRONOUN_BASES" => s_PRONOUN_BASES(v),
            "PUNCT_BREAK" => s_PUNCT_BREAK(v),
            "RAW_WORD" => s_RAW_WORD(v),
            "REASON_STARTS" => s_REASON_STARTS(v),
            "RELATIVE" => s_RELATIVE(v),
            "RELATIVE_ASKS" => s_RELATIVE_ASKS(v),
            "REPEAT_ONCE" => s_REPEAT_ONCE(v),
            "REPEAT_ONCE_SUBJECTS" => s_REPEAT_ONCE_SUBJECTS(v),
            "REPORTING" => s_REPORTING(v),
            "RUN_ON_LAST_PIECES" => s_RUN_ON_LAST_PIECES(v),
            "RUN_ON_MINE" => s_RUN_ON_MINE(v),
            "RUN_ON_OPENERS" => s_RUN_ON_OPENERS(v),
            "RUN_ON_PEOPLE" => s_RUN_ON_PEOPLE(v),
            "RUN_ON_SELF" => s_RUN_ON_SELF(v),
            "RUN_ON_SELF_LONGEST" => s_RUN_ON_SELF_LONGEST(v),
            "RUN_ON_SHORTEST" => s_RUN_ON_SHORTEST(v),
            "RUN_ON_WORDS" => s_RUN_ON_WORDS(v),
            "SAID_APART" => s_SAID_APART(v),
            "SAID_APART_FORMS" => s_SAID_APART_FORMS(v),
            "SAID_AS_A_FEELING" => s_SAID_AS_A_FEELING(v),
            "SAID_IT" => s_SAID_IT(v),
            "SAID_OUT" => s_SAID_OUT(v),
            "SENTENCE_END" => s_SENTENCE_END(v),
            "SHION_WORDS" => s_SHION_WORDS(v),
            "SHRT_THINGS" => s_SHRT_THINGS(v),
            "SICK_AS_FEELING" => s_SICK_AS_FEELING(v),
            "SOLFEGE" => s_SOLFEGE(v),
            "SON_VERBS" => s_SON_VERBS(v),
            "SORE_FILLERS" => s_SORE_FILLERS(v),
            "SORE_IN_FRONT" => s_SORE_IN_FRONT(v),
            "SORE_PARTS" => s_SORE_PARTS(v),
            "SPELLING_ASK" => s_SPELLING_ASK(v),
            "SPELLING_ENDINGS" => s_SPELLING_ENDINGS(v),
            "SPLIT_CONTRACTION" => s_SPLIT_CONTRACTION(v),
            "SPLIT_NOT" => s_SPLIT_NOT(v),
            "STATES_OF_A_PERSON" => s_STATES_OF_A_PERSON(v),
            "STOPPED_TH_STEMS" => s_STOPPED_TH_STEMS(v),
            "SUBJECT_WORDS" => s_SUBJECT_WORDS(v),
            "SUGAR_AFTER_ROSE" => s_SUGAR_AFTER_ROSE(v),
            "SUGAR_FIRST_PERSON" => s_SUGAR_FIRST_PERSON(v),
            "SUGAR_LEVEL" => s_SUGAR_LEVEL(v),
            "SUGAR_PRICE" => s_SUGAR_PRICE(v),
            "SUGAR_PRICE_SHAPE" => s_SUGAR_PRICE_SHAPE(v),
            "SUM_STAYS_AFTER" => s_SUM_STAYS_AFTER(v),
            "SUM_THE_NOUN" => s_SUM_THE_NOUN(v),
            "SUN_WORDS" => s_SUN_WORDS(v),
            "S_CLUSTERS" => s_S_CLUSTERS(v),
            "TAG_GUARD_BEFORE" => s_TAG_GUARD_BEFORE(v),
            "TAG_PLEASE" => s_TAG_PLEASE(v),
            "TAG_QUESTION_LEADS" => s_TAG_QUESTION_LEADS(v),
            "TAG_THANKS" => s_TAG_THANKS(v),
            "TAG_WORDS" => s_TAG_WORDS(v),
            "TAIL_OF_A_DAY" => s_TAIL_OF_A_DAY(v),
            "THE_HEART_SAID_THE_OTHER_WAY" => s_THE_HEART_SAID_THE_OTHER_WAY(v),
            "THE_LOSING" => s_THE_LOSING(v),
            "THE_ODD_PLURALS" => s_THE_ODD_PLURALS(v),
            "THING_AFTER" => s_THING_AFTER(v),
            "THING_DETERMINER" => s_THING_DETERMINER(v),
            "THING_JOIN" => s_THING_JOIN(v),
            "THING_PLACE_AFTER" => s_THING_PLACE_AFTER(v),
            "THING_PLACE_WORDS" => s_THING_PLACE_WORDS(v),
            "THING_POSSESSIVES" => s_THING_POSSESSIVES(v),
            "THING_SUBJECT" => s_THING_SUBJECT(v),
            "THING_WORDS" => s_THING_WORDS(v),
            "THING_WORDS_BY_LENGTH" => s_THING_WORDS_BY_LENGTH(v),
            "THIRD_PERSON" => s_THIRD_PERSON(v),
            "THIRD_PERSON_BE" => s_THIRD_PERSON_BE(v),
            "THOUSANDS" => s_THOUSANDS(v),
            "THOUSANDS_SAID_WITH_A_SPACE" => s_THOUSANDS_SAID_WITH_A_SPACE(v),
            "TIMES_A_STATE" => s_TIMES_A_STATE(v),
            "TIME_AFTER_MEAL" => s_TIME_AFTER_MEAL(v),
            "TIME_WORDS" => s_TIME_WORDS(v),
            "TOKEN" => s_TOKEN(v),
            "TOO_A" => s_TOO_A(v),
            "TOO_AFTER" => s_TOO_AFTER(v),
            "TOO_A_GRADED" => s_TOO_A_GRADED(v),
            "TO_AFTER" => s_TO_AFTER(v),
            "UP_TOO_ASKS" => s_UP_TOO_ASKS(v),
            "UP_TO_LEADS" => s_UP_TO_LEADS(v),
            "UP_TO_SPELLINGS" => s_UP_TO_SPELLINGS(v),
            "UP_TO_TAGS" => s_UP_TO_TAGS(v),
            "URGENT_BONES" => s_URGENT_BONES(v),
            "URGENT_WORDS" => s_URGENT_WORDS(v),
            "VEGGIES_AFTER" => s_VEGGIES_AFTER(v),
            "VOWELS" => s_VOWELS(v),
            "VOWEL_SET" => s_VOWEL_SET(v),
            "VOWEL_SET_OR_Y" => s_VOWEL_SET_OR_Y(v),
            "WANT_WORDS" => s_WANT_WORDS(v),
            "WEARS_AFTER" => s_WEARS_AFTER(v),
            "WHAT_ARE_YOU_AFTER" => s_WHAT_ARE_YOU_AFTER(v),
            "WHAT_ARE_YOU_SPELLINGS" => s_WHAT_ARE_YOU_SPELLINGS(v),
            "WH_ANY" => s_WH_ANY(v),
            "WH_INTENSIFIER" => s_WH_INTENSIFIER(v),
            "WH_INTENSIFIER_LINE" => s_WH_INTENSIFIER_LINE(v),
            "WH_OPENERS" => s_WH_OPENERS(v),
            "WH_QUANTIFIERS" => s_WH_QUANTIFIERS(v),
            "WH_WORDS" => s_WH_WORDS(v),
            "WORD" => s_WORD(v),
            "YOU_ARE_AFTER" => s_YOU_ARE_AFTER(v),
            "_AFTER_A_DOING" => s__AFTER_A_DOING(v),
            "_AFTER_A_LEAD_IN_SUBJECT" => s__AFTER_A_LEAD_IN_SUBJECT(v),
            "_AFTER_PARTICIPLE" => s__AFTER_PARTICIPLE(v),
            "_ANY_WORD" => s__ANY_WORD(v),
            "_A_CLIP" => s__A_CLIP(v),
            "_A_CLIPPED_HEAD" => s__A_CLIPPED_HEAD(v),
            "_A_CLOSING_WORD" => s__A_CLOSING_WORD(v),
            "_A_HEAD_IN_THE_IDIOM" => s__A_HEAD_IN_THE_IDIOM(v),
            "_A_QUESTION_VERB" => s__A_QUESTION_VERB(v),
            "_A_SHORT_WORD_IS_NAMED_AFTER" => s__A_SHORT_WORD_IS_NAMED_AFTER(v),
            "_A_SHORT_WORD_IS_NAMED_BEFORE" => s__A_SHORT_WORD_IS_NAMED_BEFORE(v),
            "_A_STATE_OF_THE_PERSON" => s__A_STATE_OF_THE_PERSON(v),
            "_A_WORDS" => s__A_WORDS(v),
            "_A_WORD_THE_BOT_WRITES" => s__A_WORD_THE_BOT_WRITES(v),
            "_BASE" => s__BASE(v),
            "_DAY_WORDS" => s__DAY_WORDS(v),
            "_EXCLAMATION_SUBJECTS" => s__EXCLAMATION_SUBJECTS(v),
            "_FEELING_WORD" => s__FEELING_WORD(v),
            "_HAVE_ASKING" => s__HAVE_ASKING(v),
            "_HAVE_NEGATIONS" => s__HAVE_NEGATIONS(v),
            "_HAVE_PARTICIPLES" => s__HAVE_PARTICIPLES(v),
            "_HAVE_THING" => s__HAVE_THING(v),
            "_INTENSIFIER" => s__INTENSIFIER(v),
            "_KEEPING_PLACES" => s__KEEPING_PLACES(v),
            "_KEPT_ALREADY" => s__KEPT_ALREADY(v),
            "_KEPT_THINGS" => s__KEPT_THINGS(v),
            "_LEADER_WORD" => s__LEADER_WORD(v),
            "_MEASURE" => s__MEASURE(v),
            "_NUMBER_SHAPE" => s__NUMBER_SHAPE(v),
            "_PLACES_SAID_WHOLE" => s__PLACES_SAID_WHOLE(v),
            "_PLACE_LEADS" => s__PLACE_LEADS(v),
            "_PLACE_STOP" => s__PLACE_STOP(v),
            "_PREDICATE_ONLY" => s__PREDICATE_ONLY(v),
            "_SMALL_NUMBERS" => s__SMALL_NUMBERS(v),
            "_SPACED_PIECE" => s__SPACED_PIECE(v),
            "_SPACED_RUN" => s__SPACED_RUN(v),
            "_STATE_FILLER" => s__STATE_FILLER(v),
            "_STATE_TAIL" => s__STATE_TAIL(v),
            "_SUBJECTLESS_AUX" => s__SUBJECTLESS_AUX(v),
            "_SUBJECTLESS_ONLY" => s__SUBJECTLESS_ONLY(v),
            "_SUBJECTS" => s__SUBJECTS(v),
            "_SUBJECT_SAID" => s__SUBJECT_SAID(v),
            "_TYPED_SURPRISE" => s__TYPED_SURPRISE(v),
            "_WERE_ARE" => s__WERE_ARE(v),
            "_WERE_DOING" => s__WERE_DOING(v),
            "_WERE_IN_THE_PAST" => s__WERE_IN_THE_PAST(v),
            "_WERE_LEAD_IN" => s__WERE_LEAD_IN(v),
            "_WERE_PLACE" => s__WERE_PLACE(v),
            "_WERE_STATE" => s__WERE_STATE(v),
            "_WH_QUESTIONS" => s__WH_QUESTIONS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.normalize", module_obj);
    pyrt::xrefs::reg_fn("understanding.normalize", "respell_urgent", py_func("respell_urgent", |args: &[Py]| respell_urgent(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_family_put_back", py_func("_family_put_back", |args: &[Py]| _family_put_back(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_doubles_its_last_letter", py_func("_doubles_its_last_letter", |args: &[Py]| _doubles_its_last_letter(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_drops_a_final_e", py_func("_drops_a_final_e", |args: &[Py]| _drops_a_final_e(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_keeps_its_e_before_ing", py_func("_keeps_its_e_before_ing", |args: &[Py]| _keeps_its_e_before_ing(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_ing_form", py_func("_ing_form", |args: &[Py]| _ing_form(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_inflections", py_func("_inflections", |args: &[Py]| _inflections(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "learn_thing_words", py_func("learn_thing_words", |args: &[Py]| learn_thing_words(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_double_variants", py_func("_double_variants", |args: &[Py]| _double_variants(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_doubled_letter_typed_once", py_func("_a_doubled_letter_typed_once", |args: &[Py]| _a_doubled_letter_typed_once(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_three_of_a_letter", py_func("_three_of_a_letter", |args: &[Py]| _three_of_a_letter(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_doubled_letter_words", py_func("_doubled_letter_words", |args: &[Py]| _doubled_letter_words(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_ie_variants", py_func("_ie_variants", |args: &[Py]| _ie_variants(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_missing_vowel", py_func("_missing_vowel", |args: &[Py]| _missing_vowel(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_extra_vowel", py_func("_extra_vowel", |args: &[Py]| _extra_vowel(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_said_for_an_ending", py_func("_a_said_for_an_ending", |args: &[Py]| _a_said_for_an_ending(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_dropped_letter", py_func("_dropped_letter", |args: &[Py]| _dropped_letter(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_an_ing_said_as_n", py_func("_an_ing_said_as_n", |args: &[Py]| _an_ing_said_as_n(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_leading_h", py_func("_a_leading_h", |args: &[Py]| _a_leading_h(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_e_kept_before_ing", py_func("_e_kept_before_ing", |args: &[Py]| _e_kept_before_ing(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_swapped_pair", py_func("_swapped_pair", |args: &[Py]| _swapped_pair(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_two_letters_the_wrong_way_round", py_func("_two_letters_the_wrong_way_round", |args: &[Py]| _two_letters_the_wrong_way_round(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_words_the_bot_writes", py_func("_words_the_bot_writes", |args: &[Py]| _words_the_bot_writes(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_word_the_bot_writes", py_func("_a_word_the_bot_writes", |args: &[Py]| _a_word_the_bot_writes(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_bones", py_func("_bones", |args: &[Py]| _bones(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_longest_consonant_run", py_func("_longest_consonant_run", |args: &[Py]| _longest_consonant_run(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_an_emergency_word_left_out", py_func("_an_emergency_word_left_out", |args: &[Py]| _an_emergency_word_left_out(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_name_slot", py_func("_a_name_slot", |args: &[Py]| _a_name_slot(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_dropped_g", py_func("_dropped_g", |args: &[Py]| _dropped_g(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_stopped_th", py_func("_stopped_th", |args: &[Py]| _stopped_th(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_shun_variant", py_func("_shun_variant", |args: &[Py]| _shun_variant(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_said_with_an_f", py_func("_said_with_an_f", |args: &[Py]| _said_with_an_f(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_said_with_an_f_anywhere", py_func("_said_with_an_f_anywhere", |args: &[Py]| _said_with_an_f_anywhere(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_thing_where_a_thing_goes", py_func("_a_thing_where_a_thing_goes", |args: &[Py]| _a_thing_where_a_thing_goes(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_missing_e", py_func("_missing_e", |args: &[Py]| _missing_e(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_short_double", py_func("_short_double", |args: &[Py]| _short_double(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_one_letter_apart", py_func("_one_letter_apart", |args: &[Py]| _one_letter_apart(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_one_letter_wrong", py_func("_one_letter_wrong", |args: &[Py]| _one_letter_wrong(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_word_the_reading_knows", py_func("_a_word_the_reading_knows", |args: &[Py]| _a_word_the_reading_knows(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_known_word", py_func("_known_word", |args: &[Py]| _known_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_stems_of", py_func("_stems_of", |args: &[Py]| _stems_of(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_thing_a_letter_short", py_func("_a_thing_a_letter_short", |args: &[Py]| _a_thing_a_letter_short(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_core_verb_typed_wrong", py_func("_a_core_verb_typed_wrong", |args: &[Py]| _a_core_verb_typed_wrong(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_its_vowels_typed_wrong", py_func("_its_vowels_typed_wrong", |args: &[Py]| _its_vowels_typed_wrong(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_near_a_part", py_func("_near_a_part", |args: &[Py]| _near_a_part(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_game_named_another_way", py_func("_a_game_named_another_way", |args: &[Py]| _a_game_named_another_way(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_parts_that_hurt", py_func("_parts_that_hurt", |args: &[Py]| _parts_that_hurt(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_where_a_state_stands", py_func("_where_a_state_stands", |args: &[Py]| _where_a_state_stands(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_ends_a_state", py_func("_ends_a_state", |args: &[Py]| _ends_a_state(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_opens_a_state", py_func("_opens_a_state", |args: &[Py]| _opens_a_state(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_measure_spelt_wrong", py_func("_a_measure_spelt_wrong", |args: &[Py]| _a_measure_spelt_wrong(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_measures_a_state", py_func("_measures_a_state", |args: &[Py]| _measures_a_state(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_letter_typed_twice", py_func("_a_letter_typed_twice", |args: &[Py]| _a_letter_typed_twice(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_an_introduction", py_func("_an_introduction", |args: &[Py]| _an_introduction(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_letter_wrong_in_a_state", py_func("_a_letter_wrong_in_a_state", |args: &[Py]| _a_letter_wrong_in_a_state(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_letter_apart", py_func("_a_letter_apart", |args: &[Py]| _a_letter_apart(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_near_a_feeling", py_func("_near_a_feeling", |args: &[Py]| _near_a_feeling(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_consonant_left_out_of_an_ing_word", py_func("_a_consonant_left_out_of_an_ing_word", |args: &[Py]| _a_consonant_left_out_of_an_ing_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_state_is_the_whole_frame", py_func("_the_state_is_the_whole_frame", |args: &[Py]| _the_state_is_the_whole_frame(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "the_feeling_two_letters_are_gone_from", py_func("the_feeling_two_letters_are_gone_from", |args: &[Py]| the_feeling_two_letters_are_gone_from(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_two_letters_out", py_func("_two_letters_out", |args: &[Py]| _two_letters_out(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_feeling_without_its_ending", py_func("_feeling_without_its_ending", |args: &[Py]| _feeling_without_its_ending(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "reads_as_a_feeling", py_func("reads_as_a_feeling", |args: &[Py]| reads_as_a_feeling(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_vowel_left_out_of_a_state", py_func("_a_vowel_left_out_of_a_state", |args: &[Py]| _a_vowel_left_out_of_a_state(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_one_vowel_less", py_func("_one_vowel_less", |args: &[Py]| _one_vowel_less(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "reads_as_a_sound", py_func("reads_as_a_sound", |args: &[Py]| reads_as_a_sound(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "repaired_tokens", py_func("repaired_tokens", |args: &[Py]| repaired_tokens(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "spelled", py_func("spelled", |args: &[Py]| spelled(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_unsays_the_word", py_func("_unsays_the_word", |args: &[Py]| _unsays_the_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_word_of_the_bots", py_func("_a_word_of_the_bots", |args: &[Py]| _a_word_of_the_bots(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "fix_typo", py_func("fix_typo", |args: &[Py]| fix_typo(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_slip", py_func("_slip", |args: &[Py]| _slip(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "fix_opener", py_func("fix_opener", |args: &[Py]| fix_opener(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_plain_word", py_func("_plain_word", |args: &[Py]| _plain_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_repair_openers", py_func("_repair_openers", |args: &[Py]| _repair_openers(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_four_spells_for", py_func("_four_spells_for", |args: &[Py]| _four_spells_for(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_collapse", py_func("_collapse", |args: &[Py]| _collapse(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_apostrophe", py_func("_apostrophe", |args: &[Py]| _apostrophe(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_counts_a_compound", py_func("_counts_a_compound", |args: &[Py]| _counts_a_compound(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_counts_a_thing", py_func("_counts_a_thing", |args: &[Py]| _counts_a_thing(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_digit_word", py_func("_digit_word", |args: &[Py]| _digit_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_once_only", py_func("_once_only", |args: &[Py]| _once_only(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "opens_without_subject", py_func("opens_without_subject", |args: &[Py]| opens_without_subject(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_only_leaders_before", py_func("_only_leaders_before", |args: &[Py]| _only_leaders_before(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_lookahead", py_func("_lookahead", |args: &[Py]| _lookahead(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "subject_missing", py_func("subject_missing", |args: &[Py]| subject_missing(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_opener_is", py_func("_opener_is", |args: &[Py]| _opener_is(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_cause_in_place_is_the_verb", py_func("_cause_in_place_is_the_verb", |args: &[Py]| _cause_in_place_is_the_verb(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_token", py_func("_token", |args: &[Py]| _token(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_one_word_food", py_func("_one_word_food", |args: &[Py]| _one_word_food(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_joined_words", py_func("_joined_words", |args: &[Py]| _joined_words(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_z_for_s", py_func("_z_for_s", |args: &[Py]| _z_for_s(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_ache_for_aike", py_func("_ache_for_aike", |args: &[Py]| _ache_for_aike(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_contraction_typed_with_a_space", py_func("_a_contraction_typed_with_a_space", |args: &[Py]| _a_contraction_typed_with_a_space(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_ask_said_plainer", py_func("_the_ask_said_plainer", |args: &[Py]| _the_ask_said_plainer(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_me_for_my_pass", py_func("_me_for_my_pass", |args: &[Py]| _me_for_my_pass(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_phrase_pass", py_func("_phrase_pass", |args: &[Py]| _phrase_pass(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_figure", py_func("_the_figure", |args: &[Py]| _the_figure(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_sign_is_a_word", py_func("_a_sign_is_a_word", |args: &[Py]| _a_sign_is_a_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_doing_something", py_func("_is_doing_something", |args: &[Py]| _is_doing_something(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "possessive_me", py_func("possessive_me", |args: &[Py]| possessive_me(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "a_son_not_the_sun", py_func("a_son_not_the_sun", |args: &[Py]| a_son_not_the_sun(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_slip_that_is_a_word", py_func("_a_slip_that_is_a_word", |args: &[Py]| _a_slip_that_is_a_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "loose_is_lose", py_func("loose_is_lose", |args: &[Py]| loose_is_lose(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_thing_has_been", py_func("_the_thing_has_been", |args: &[Py]| _the_thing_has_been(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_person", py_func("_is_person", |args: &[Py]| _is_person(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_plain_chat", py_func("_plain_chat", |args: &[Py]| _plain_chat(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_spells_a_helper", py_func("_spells_a_helper", |args: &[Py]| _spells_a_helper(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_be", py_func("_is_be", |args: &[Py]| _is_be(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_participle", py_func("_is_participle", |args: &[Py]| _is_participle(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_gerund_reads_as_a_verb", py_func("_a_gerund_reads_as_a_verb", |args: &[Py]| _a_gerund_reads_as_a_verb(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_kin_needs_a_helper", py_func("_kin_needs_a_helper", |args: &[Py]| _kin_needs_a_helper(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_family_short", py_func("_family_short", |args: &[Py]| _family_short(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_counts_a_period", py_func("_counts_a_period", |args: &[Py]| _counts_a_period(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_a_count", py_func("_is_a_count", |args: &[Py]| _is_a_count(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_hr_is_the_department", py_func("_hr_is_the_department", |args: &[Py]| _hr_is_the_department(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_says_one", py_func("_says_one", |args: &[Py]| _says_one(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_a_thing_here", py_func("_is_a_thing_here", |args: &[Py]| _is_a_thing_here(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_clipped_thing", py_func("_clipped_thing", |args: &[Py]| _clipped_thing(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_before_a_noun", py_func("_is_before_a_noun", |args: &[Py]| _is_before_a_noun(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_said_of_a_person", py_func("_is_said_of_a_person", |args: &[Py]| _is_said_of_a_person(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_is_possessive", py_func("_is_possessive", |args: &[Py]| _is_possessive(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_sounds_like_a_feeling", py_func("_sounds_like_a_feeling", |args: &[Py]| _sounds_like_a_feeling(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_were_for_another_word", py_func("_were_for_another_word", |args: &[Py]| _were_for_another_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_doing", py_func("_a_doing", |args: &[Py]| _a_doing(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_doubtful_split", py_func("_a_doubtful_split", |args: &[Py]| _a_doubtful_split(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_place_said_whole", py_func("_a_place_said_whole", |args: &[Py]| _a_place_said_whole(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_pronoun_said_together", py_func("_a_pronoun_said_together", |args: &[Py]| _a_pronoun_said_together(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_state_typed_onto_im", py_func("_a_state_typed_onto_im", |args: &[Py]| _a_state_typed_onto_im(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_glued_words", py_func("_glued_words", |args: &[Py]| _glued_words(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_mrs_is_the_wife", py_func("_the_mrs_is_the_wife", |args: &[Py]| _the_mrs_is_the_wife(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_chat_words_said_out", py_func("_chat_words_said_out", |args: &[Py]| _chat_words_said_out(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_hello_said_short", py_func("_a_hello_said_short", |args: &[Py]| _a_hello_said_short(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_glued_said_apart", py_func("_glued_said_apart", |args: &[Py]| _glued_said_apart(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_said_run_on", py_func("_said_run_on", |args: &[Py]| _said_run_on(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_run_and_what_surrounds_it", py_func("_a_run_and_what_surrounds_it", |args: &[Py]| _a_run_and_what_surrounds_it(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_typed_without_spaces", py_func("_typed_without_spaces", |args: &[Py]| _typed_without_spaces(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_said_apart_forms", py_func("_the_said_apart_forms", |args: &[Py]| _the_said_apart_forms(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_ways_a_word_is_typed_apart", py_func("_the_ways_a_word_is_typed_apart", |args: &[Py]| _the_ways_a_word_is_typed_apart(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_said_apart", py_func("_said_apart", |args: &[Py]| _said_apart(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "said_apart", py_func("said_apart", |args: &[Py]| said_apart(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_ill_is_i_will_going", py_func("_ill_is_i_will_going", |args: &[Py]| _ill_is_i_will_going(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_an_ill_that_is_i_will", py_func("_an_ill_that_is_i_will", |args: &[Py]| _an_ill_that_is_i_will(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_left_off_contraction", py_func("_left_off_contraction", |args: &[Py]| _left_off_contraction(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_one_of_the_parts", py_func("_one_of_the_parts", |args: &[Py]| _one_of_the_parts(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_wers_word", py_func("_wers_word", |args: &[Py]| _wers_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_sum_word", py_func("_sum_word", |args: &[Py]| _sum_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_positional_word", py_func("_positional_word", |args: &[Py]| _positional_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_join_chat_digits", py_func("_join_chat_digits", |args: &[Py]| _join_chat_digits(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_said_apart_as_one_word", py_func("_said_apart_as_one_word", |args: &[Py]| _said_apart_as_one_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_said_apart_as_one", py_func("_said_apart_as_one", |args: &[Py]| _said_apart_as_one(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_docs_on_a_screen", py_func("_docs_on_a_screen", |args: &[Py]| _docs_on_a_screen(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_near_a_meal", py_func("_near_a_meal", |args: &[Py]| _near_a_meal(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_meal_word_misspelt", py_func("_a_meal_word_misspelt", |args: &[Py]| _a_meal_word_misspelt(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_docs_doctor", py_func("_docs_doctor", |args: &[Py]| _docs_doctor(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "a_sor_bit_is_sore", py_func("a_sor_bit_is_sore", |args: &[Py]| a_sor_bit_is_sore(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_one_relation_says", py_func("_one_relation_says", |args: &[Py]| _one_relation_says(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_ask_start", py_func("_ask_start", |args: &[Py]| _ask_start(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_word_between_the_ask_and_its_aux", py_func("_a_word_between_the_ask_and_its_aux", |args: &[Py]| _a_word_between_the_ask_and_its_aux(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_inverted_ask", py_func("_inverted_ask", |args: &[Py]| _inverted_ask(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_reported", py_func("_reported", |args: &[Py]| _reported(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_split_ask", py_func("_split_ask", |args: &[Py]| _split_ask(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_opens_clause", py_func("_opens_clause", |args: &[Py]| _opens_clause(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_says_has", py_func("_says_has", |args: &[Py]| _says_has(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_clauses", py_func("_clauses", |args: &[Py]| _clauses(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_leading_like_is_a_filler", py_func("_a_leading_like_is_a_filler", |args: &[Py]| _a_leading_like_is_a_filler(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "without_a_leading_exclamation", py_func("without_a_leading_exclamation", |args: &[Py]| without_a_leading_exclamation(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_an_exclamation_in_front", py_func("_an_exclamation_in_front", |args: &[Py]| _an_exclamation_in_front(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_an_exclamation_is_not_a_thing", py_func("_an_exclamation_is_not_a_thing", |args: &[Py]| _an_exclamation_is_not_a_thing(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "strip_leaders", py_func("strip_leaders", |args: &[Py]| strip_leaders(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "without_leaders", py_func("without_leaders", |args: &[Py]| without_leaders(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_clause_stop", py_func("_clause_stop", |args: &[Py]| _clause_stop(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "background_clause", py_func("background_clause", |args: &[Py]| background_clause(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "reason_positions", py_func("reason_positions", |args: &[Py]| reason_positions(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "background_positions", py_func("background_positions", |args: &[Py]| background_positions(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_measure_clause", py_func("_the_measure_clause", |args: &[Py]| _the_measure_clause(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_asking", py_func("_asking", |args: &[Py]| _asking(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_focus", py_func("_focus", |args: &[Py]| _focus(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_unaccent", py_func("_unaccent", |args: &[Py]| _unaccent(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "intensifier_positions", py_func("intensifier_positions", |args: &[Py]| intensifier_positions(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_surprise_typed", py_func("_the_surprise_typed", |args: &[Py]| _the_surprise_typed(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_without_the_intensifier", py_func("_without_the_intensifier", |args: &[Py]| _without_the_intensifier(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_clip_kept_only_with_its_cue", py_func("_a_clip_kept_only_with_its_cue", |args: &[Py]| _a_clip_kept_only_with_its_cue(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_fix_auxiliaries", py_func("_fix_auxiliaries", |args: &[Py]| _fix_auxiliaries(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_change_of_subject_off", py_func("_change_of_subject_off", |args: &[Py]| _change_of_subject_off(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_without_a_tag_at_the_end", py_func("_without_a_tag_at_the_end", |args: &[Py]| _without_a_tag_at_the_end(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_spellings_of_one_word", py_func("_spellings_of_one_word", |args: &[Py]| _spellings_of_one_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_names_a_letter_beside_a_slip", py_func("_names_a_letter_beside_a_slip", |args: &[Py]| _names_a_letter_beside_a_slip(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_short_word_named", py_func("_a_short_word_named", |args: &[Py]| _a_short_word_named(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_asks_the_spelling", py_func("_asks_the_spelling", |args: &[Py]| _asks_the_spelling(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_can_not_as_one_word", py_func("_can_not_as_one_word", |args: &[Py]| _can_not_as_one_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_day_left_out", py_func("_the_day_left_out", |args: &[Py]| _the_day_left_out(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_day_named_as_not_mine", py_func("_a_day_named_as_not_mine", |args: &[Py]| _a_day_named_as_not_mine(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_a_number", py_func("_a_number", |args: &[Py]| _a_number(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_kept_for", py_func("_kept_for", |args: &[Py]| _kept_for(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_only_doubles_a_letter", py_func("_only_doubles_a_letter", |args: &[Py]| _only_doubles_a_letter(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "normalise", py_func("normalise", |args: &[Py]| normalise(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_feeling_word", py_func("_the_feeling_word", |args: &[Py]| _the_feeling_word(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "without_the_figure_of_speech", py_func("without_the_figure_of_speech", |args: &[Py]| without_the_figure_of_speech(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_part_that_is_bad", py_func("_the_part_that_is_bad", |args: &[Py]| _the_part_that_is_bad(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_part_that_hurts", py_func("_the_part_that_hurts", |args: &[Py]| _the_part_that_hurts(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "_the_sugar_that_is_blood", py_func("_the_sugar_that_is_blood", |args: &[Py]| _the_sugar_that_is_blood(args)));
    pyrt::xrefs::reg_fn("understanding.normalize", "glued_words", py_func("glued_words", |args: &[Py]| glued_words(args)));
    pyrt::xrefs::reg_access("understanding.normalize", "ABOUT_THE_FORM", g_ABOUT_THE_FORM, s_ABOUT_THE_FORM);
    pyrt::xrefs::reg_access("understanding.normalize", "AEIOU", g_AEIOU, s_AEIOU);
    pyrt::xrefs::reg_access("understanding.normalize", "AFTER_AN_INVERTED_AUX", g_AFTER_AN_INVERTED_AUX, s_AFTER_AN_INVERTED_AUX);
    pyrt::xrefs::reg_access("understanding.normalize", "AFTER_A_STATE", g_AFTER_A_STATE, s_AFTER_A_STATE);
    pyrt::xrefs::reg_access("understanding.normalize", "AFTER_A_THING", g_AFTER_A_THING, s_AFTER_A_THING);
    pyrt::xrefs::reg_access("understanding.normalize", "AFTER_A_TIME", g_AFTER_A_TIME, s_AFTER_A_TIME);
    pyrt::xrefs::reg_access("understanding.normalize", "AFTER_A_VERB", g_AFTER_A_VERB, s_AFTER_A_VERB);
    pyrt::xrefs::reg_access("understanding.normalize", "AINT_FORMS", g_AINT_FORMS, s_AINT_FORMS);
    pyrt::xrefs::reg_access("understanding.normalize", "AINT_PLURAL", g_AINT_PLURAL, s_AINT_PLURAL);
    pyrt::xrefs::reg_access("understanding.normalize", "AMOUNT_BEFORE_A_STATE", g_AMOUNT_BEFORE_A_STATE, s_AMOUNT_BEFORE_A_STATE);
    pyrt::xrefs::reg_access("understanding.normalize", "APOSTROPHE_BASES", g_APOSTROPHE_BASES, s_APOSTROPHE_BASES);
    pyrt::xrefs::reg_access("understanding.normalize", "APOSTROPHE_NOT", g_APOSTROPHE_NOT, s_APOSTROPHE_NOT);
    pyrt::xrefs::reg_access("understanding.normalize", "APOSTROPHE_TAILS", g_APOSTROPHE_TAILS, s_APOSTROPHE_TAILS);
    pyrt::xrefs::reg_access("understanding.normalize", "ASKS_WHOSE", g_ASKS_WHOSE, s_ASKS_WHOSE);
    pyrt::xrefs::reg_access("understanding.normalize", "ASK_NOUNS", g_ASK_NOUNS, s_ASK_NOUNS);
    pyrt::xrefs::reg_access("understanding.normalize", "ASK_WORDS", g_ASK_WORDS, s_ASK_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "AUX_OPENS", g_AUX_OPENS, s_AUX_OPENS);
    pyrt::xrefs::reg_access("understanding.normalize", "AUX_WORDS", g_AUX_WORDS, s_AUX_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "A_CHOKE_SAID_THE_OTHER_WAY", g_A_CHOKE_SAID_THE_OTHER_WAY, s_A_CHOKE_SAID_THE_OTHER_WAY);
    pyrt::xrefs::reg_access("understanding.normalize", "A_COPULA", g_A_COPULA, s_A_COPULA);
    pyrt::xrefs::reg_access("understanding.normalize", "A_COUNT", g_A_COUNT, s_A_COUNT);
    pyrt::xrefs::reg_access("understanding.normalize", "A_FEELING_CAN_BE_FOLLOWED", g_A_FEELING_CAN_BE_FOLLOWED, s_A_FEELING_CAN_BE_FOLLOWED);
    pyrt::xrefs::reg_access("understanding.normalize", "A_GLASS_OR_A_PAN", g_A_GLASS_OR_A_PAN, s_A_GLASS_OR_A_PAN);
    pyrt::xrefs::reg_access("understanding.normalize", "A_JOB", g_A_JOB, s_A_JOB);
    pyrt::xrefs::reg_access("understanding.normalize", "A_KEEPING_A_THING", g_A_KEEPING_A_THING, s_A_KEEPING_A_THING);
    pyrt::xrefs::reg_access("understanding.normalize", "A_LABEL_BEFORE", g_A_LABEL_BEFORE, s_A_LABEL_BEFORE);
    pyrt::xrefs::reg_access("understanding.normalize", "A_LETTER_AFTER", g_A_LETTER_AFTER, s_A_LETTER_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "A_NEGATION_BEFORE", g_A_NEGATION_BEFORE, s_A_NEGATION_BEFORE);
    pyrt::xrefs::reg_access("understanding.normalize", "A_NUMBER_OF_SOMEONES_DAY", g_A_NUMBER_OF_SOMEONES_DAY, s_A_NUMBER_OF_SOMEONES_DAY);
    pyrt::xrefs::reg_access("understanding.normalize", "A_PAIN_WORD", g_A_PAIN_WORD, s_A_PAIN_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "A_PARTS_OF_A_PERIOD", g_A_PARTS_OF_A_PERIOD, s_A_PARTS_OF_A_PERIOD);
    pyrt::xrefs::reg_access("understanding.normalize", "A_PART_A_PERSON_OWNS", g_A_PART_A_PERSON_OWNS, s_A_PART_A_PERSON_OWNS);
    pyrt::xrefs::reg_access("understanding.normalize", "A_PART_THE_PAIN_IS_IN", g_A_PART_THE_PAIN_IS_IN, s_A_PART_THE_PAIN_IS_IN);
    pyrt::xrefs::reg_access("understanding.normalize", "A_PERIOD_FOLLOWS", g_A_PERIOD_FOLLOWS, s_A_PERIOD_FOLLOWS);
    pyrt::xrefs::reg_access("understanding.normalize", "A_PRONOUN_GLUED", g_A_PRONOUN_GLUED, s_A_PRONOUN_GLUED);
    pyrt::xrefs::reg_access("understanding.normalize", "A_PRONOUN_STATE", g_A_PRONOUN_STATE, s_A_PRONOUN_STATE);
    pyrt::xrefs::reg_access("understanding.normalize", "A_SIDE", g_A_SIDE, s_A_SIDE);
    pyrt::xrefs::reg_access("understanding.normalize", "A_STATE_THEN_A_TIME", g_A_STATE_THEN_A_TIME, s_A_STATE_THEN_A_TIME);
    pyrt::xrefs::reg_access("understanding.normalize", "A_STATE_YOU_CAN_FEEL", g_A_STATE_YOU_CAN_FEEL, s_A_STATE_YOU_CAN_FEEL);
    pyrt::xrefs::reg_access("understanding.normalize", "A_THING_GOES_AFTER", g_A_THING_GOES_AFTER, s_A_THING_GOES_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "A_THING_TYPED_AS_A_LITTLE_WORD", g_A_THING_TYPED_AS_A_LITTLE_WORD, s_A_THING_TYPED_AS_A_LITTLE_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "A_THING_WORN", g_A_THING_WORN, s_A_THING_WORN);
    pyrt::xrefs::reg_access("understanding.normalize", "A_TIME_TO_LEAVE", g_A_TIME_TO_LEAVE, s_A_TIME_TO_LEAVE);
    pyrt::xrefs::reg_access("understanding.normalize", "A_VERB_CAN_FOLLOW", g_A_VERB_CAN_FOLLOW, s_A_VERB_CAN_FOLLOW);
    pyrt::xrefs::reg_access("understanding.normalize", "A_WORD_CAN_FOLLOW", g_A_WORD_CAN_FOLLOW, s_A_WORD_CAN_FOLLOW);
    pyrt::xrefs::reg_access("understanding.normalize", "BAD_SAYING", g_BAD_SAYING, s_BAD_SAYING);
    pyrt::xrefs::reg_access("understanding.normalize", "BAD_WORD", g_BAD_WORD, s_BAD_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "BARE_TO", g_BARE_TO, s_BARE_TO);
    pyrt::xrefs::reg_access("understanding.normalize", "BASE", g_BASE, s_BASE);
    pyrt::xrefs::reg_access("understanding.normalize", "BEEN_SUBJECT", g_BEEN_SUBJECT, s_BEEN_SUBJECT);
    pyrt::xrefs::reg_access("understanding.normalize", "BEEN_WORDS", g_BEEN_WORDS, s_BEEN_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "BEFORE_A_FEELING", g_BEFORE_A_FEELING, s_BEFORE_A_FEELING);
    pyrt::xrefs::reg_access("understanding.normalize", "BEFORE_A_JOB", g_BEFORE_A_JOB, s_BEFORE_A_JOB);
    pyrt::xrefs::reg_access("understanding.normalize", "BEFORE_A_PREPOSITION", g_BEFORE_A_PREPOSITION, s_BEFORE_A_PREPOSITION);
    pyrt::xrefs::reg_access("understanding.normalize", "BEFORE_A_THING", g_BEFORE_A_THING, s_BEFORE_A_THING);
    pyrt::xrefs::reg_access("understanding.normalize", "BEFORE_A_VERB", g_BEFORE_A_VERB, s_BEFORE_A_VERB);
    pyrt::xrefs::reg_access("understanding.normalize", "BEFORE_THE_LOSING", g_BEFORE_THE_LOSING, s_BEFORE_THE_LOSING);
    pyrt::xrefs::reg_access("understanding.normalize", "BEFORE_VERB", g_BEFORE_VERB, s_BEFORE_VERB);
    pyrt::xrefs::reg_access("understanding.normalize", "BEFORE_WORD", g_BEFORE_WORD, s_BEFORE_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "BE_BEFORE_A_STATE", g_BE_BEFORE_A_STATE, s_BE_BEFORE_A_STATE);
    pyrt::xrefs::reg_access("understanding.normalize", "BE_FORMS", g_BE_FORMS, s_BE_FORMS);
    pyrt::xrefs::reg_access("understanding.normalize", "BE_VERBS", g_BE_VERBS, s_BE_VERBS);
    pyrt::xrefs::reg_access("understanding.normalize", "BE_WORDS", g_BE_WORDS, s_BE_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "BIN_CONTAINERS", g_BIN_CONTAINERS, s_BIN_CONTAINERS);
    pyrt::xrefs::reg_access("understanding.normalize", "BREAKS", g_BREAKS, s_BREAKS);
    pyrt::xrefs::reg_access("understanding.normalize", "CAN_FOLLOW_A_VERB", g_CAN_FOLLOW_A_VERB, s_CAN_FOLLOW_A_VERB);
    pyrt::xrefs::reg_access("understanding.normalize", "CAN_NOT", g_CAN_NOT, s_CAN_NOT);
    pyrt::xrefs::reg_access("understanding.normalize", "CAUSES_A_STATE", g_CAUSES_A_STATE, s_CAUSES_A_STATE);
    pyrt::xrefs::reg_access("understanding.normalize", "CAUSE_A_CLAUSE_VERB", g_CAUSE_A_CLAUSE_VERB, s_CAUSE_A_CLAUSE_VERB);
    pyrt::xrefs::reg_access("understanding.normalize", "CAUSE_A_DEMONSTRATIVE", g_CAUSE_A_DEMONSTRATIVE, s_CAUSE_A_DEMONSTRATIVE);
    pyrt::xrefs::reg_access("understanding.normalize", "CAUSE_A_SUBJECT_FOLLOWS", g_CAUSE_A_SUBJECT_FOLLOWS, s_CAUSE_A_SUBJECT_FOLLOWS);
    pyrt::xrefs::reg_access("understanding.normalize", "CAUSE_BEFORE_A_NOUN", g_CAUSE_BEFORE_A_NOUN, s_CAUSE_BEFORE_A_NOUN);
    pyrt::xrefs::reg_access("understanding.normalize", "CAUSE_BEFORE_A_VERB", g_CAUSE_BEFORE_A_VERB, s_CAUSE_BEFORE_A_VERB);
    pyrt::xrefs::reg_access("understanding.normalize", "CHANGE_OF_SUBJECT", g_CHANGE_OF_SUBJECT, s_CHANGE_OF_SUBJECT);
    pyrt::xrefs::reg_access("understanding.normalize", "CHAT_AS_ONE_WORD", g_CHAT_AS_ONE_WORD, s_CHAT_AS_ONE_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "CHAT_KEPT_AS_TYPED", g_CHAT_KEPT_AS_TYPED, s_CHAT_KEPT_AS_TYPED);
    pyrt::xrefs::reg_access("understanding.normalize", "CHAT_NAMES", g_CHAT_NAMES, s_CHAT_NAMES);
    pyrt::xrefs::reg_access("understanding.normalize", "CLAUSE_OPENERS", g_CLAUSE_OPENERS, s_CLAUSE_OPENERS);
    pyrt::xrefs::reg_access("understanding.normalize", "CLIPPED_FAMILY", g_CLIPPED_FAMILY, s_CLIPPED_FAMILY);
    pyrt::xrefs::reg_access("understanding.normalize", "CLIPPED_THINGS", g_CLIPPED_THINGS, s_CLIPPED_THINGS);
    pyrt::xrefs::reg_access("understanding.normalize", "CLIPPED_UNITS", g_CLIPPED_UNITS, s_CLIPPED_UNITS);
    pyrt::xrefs::reg_access("understanding.normalize", "COMMON_RANK", g_COMMON_RANK, s_COMMON_RANK);
    pyrt::xrefs::reg_access("understanding.normalize", "COMPOUND_HEADS", g_COMPOUND_HEADS, s_COMPOUND_HEADS);
    pyrt::xrefs::reg_access("understanding.normalize", "COMPOUND_TAILS", g_COMPOUND_TAILS, s_COMPOUND_TAILS);
    pyrt::xrefs::reg_access("understanding.normalize", "CONTRACTION_NEXT", g_CONTRACTION_NEXT, s_CONTRACTION_NEXT);
    pyrt::xrefs::reg_access("understanding.normalize", "COPULAS", g_COPULAS, s_COPULAS);
    pyrt::xrefs::reg_access("understanding.normalize", "CORE_VERBS", g_CORE_VERBS, s_CORE_VERBS);
    pyrt::xrefs::reg_access("understanding.normalize", "COUNTS", g_COUNTS, s_COUNTS);
    pyrt::xrefs::reg_access("understanding.normalize", "COUNT_UNITS", g_COUNT_UNITS, s_COUNT_UNITS);
    pyrt::xrefs::reg_access("understanding.normalize", "CURRENCY_FACTOR", g_CURRENCY_FACTOR, s_CURRENCY_FACTOR);
    pyrt::xrefs::reg_access("understanding.normalize", "CURRENCY_SIGN", g_CURRENCY_SIGN, s_CURRENCY_SIGN);
    pyrt::xrefs::reg_access("understanding.normalize", "CURRENCY_SIGNS", g_CURRENCY_SIGNS, s_CURRENCY_SIGNS);
    pyrt::xrefs::reg_access("understanding.normalize", "DAY_NAMED", g_DAY_NAMED, s_DAY_NAMED);
    pyrt::xrefs::reg_access("understanding.normalize", "DEMONSTRATIVE", g_DEMONSTRATIVE, s_DEMONSTRATIVE);
    pyrt::xrefs::reg_access("understanding.normalize", "DETERMINERS", g_DETERMINERS, s_DETERMINERS);
    pyrt::xrefs::reg_access("understanding.normalize", "DETERMINER_IN_FRONT", g_DETERMINER_IN_FRONT, s_DETERMINER_IN_FRONT);
    pyrt::xrefs::reg_access("understanding.normalize", "DIGIT_AS_WORD", g_DIGIT_AS_WORD, s_DIGIT_AS_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "DIGIT_WORDS", g_DIGIT_WORDS, s_DIGIT_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "DOCS_ON", g_DOCS_ON, s_DOCS_ON);
    pyrt::xrefs::reg_access("understanding.normalize", "DOCS_PAPER", g_DOCS_PAPER, s_DOCS_PAPER);
    pyrt::xrefs::reg_access("understanding.normalize", "DOCS_SCREEN", g_DOCS_SCREEN, s_DOCS_SCREEN);
    pyrt::xrefs::reg_access("understanding.normalize", "DOCS_VISIT", g_DOCS_VISIT, s_DOCS_VISIT);
    pyrt::xrefs::reg_access("understanding.normalize", "DOING_NOT_THING", g_DOING_NOT_THING, s_DOING_NOT_THING);
    pyrt::xrefs::reg_access("understanding.normalize", "DROPPED_SUBJECT_AUX", g_DROPPED_SUBJECT_AUX, s_DROPPED_SUBJECT_AUX);
    pyrt::xrefs::reg_access("understanding.normalize", "EXCLAIMED_AFTER_A_QUESTION", g_EXCLAIMED_AFTER_A_QUESTION, s_EXCLAIMED_AFTER_A_QUESTION);
    pyrt::xrefs::reg_access("understanding.normalize", "EXCLAMATION_OPENS", g_EXCLAMATION_OPENS, s_EXCLAMATION_OPENS);
    pyrt::xrefs::reg_access("understanding.normalize", "EXCLAMATION_PHRASE", g_EXCLAMATION_PHRASE, s_EXCLAMATION_PHRASE);
    pyrt::xrefs::reg_access("understanding.normalize", "E_KEPT_WORDS", g_E_KEPT_WORDS, s_E_KEPT_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "FAMILY_SHRT", g_FAMILY_SHRT, s_FAMILY_SHRT);
    pyrt::xrefs::reg_access("understanding.normalize", "FEELING_BEFORE_SICK", g_FEELING_BEFORE_SICK, s_FEELING_BEFORE_SICK);
    pyrt::xrefs::reg_access("understanding.normalize", "FINDING_A_TEAM", g_FINDING_A_TEAM, s_FINDING_A_TEAM);
    pyrt::xrefs::reg_access("understanding.normalize", "FIRST_PERSON_BE", g_FIRST_PERSON_BE, s_FIRST_PERSON_BE);
    pyrt::xrefs::reg_access("understanding.normalize", "FOOD_SLOT_LEAD", g_FOOD_SLOT_LEAD, s_FOOD_SLOT_LEAD);
    pyrt::xrefs::reg_access("understanding.normalize", "FOR_A", g_FOR_A, s_FOR_A);
    pyrt::xrefs::reg_access("understanding.normalize", "FOR_MEAL", g_FOR_MEAL, s_FOR_MEAL);
    pyrt::xrefs::reg_access("understanding.normalize", "FOR_OBJECT", g_FOR_OBJECT, s_FOR_OBJECT);
    pyrt::xrefs::reg_access("understanding.normalize", "FREQUENT", g_FREQUENT, s_FREQUENT);
    pyrt::xrefs::reg_access("understanding.normalize", "F_TYPED_FOR_TH", g_F_TYPED_FOR_TH, s_F_TYPED_FOR_TH);
    pyrt::xrefs::reg_access("understanding.normalize", "GLUED_AFTER_A_DO", g_GLUED_AFTER_A_DO, s_GLUED_AFTER_A_DO);
    pyrt::xrefs::reg_access("understanding.normalize", "GLUED_COLLAPSE", g_GLUED_COLLAPSE, s_GLUED_COLLAPSE);
    pyrt::xrefs::reg_access("understanding.normalize", "GLUED_DO", g_GLUED_DO, s_GLUED_DO);
    pyrt::xrefs::reg_access("understanding.normalize", "GLUED_LEFT", g_GLUED_LEFT, s_GLUED_LEFT);
    pyrt::xrefs::reg_access("understanding.normalize", "GLUED_RIGHT", g_GLUED_RIGHT, s_GLUED_RIGHT);
    pyrt::xrefs::reg_access("understanding.normalize", "HAS_AFTER", g_HAS_AFTER, s_HAS_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "HAS_FORMS", g_HAS_FORMS, s_HAS_FORMS);
    pyrt::xrefs::reg_access("understanding.normalize", "HAS_SUBJECT", g_HAS_SUBJECT, s_HAS_SUBJECT);
    pyrt::xrefs::reg_access("understanding.normalize", "HAS_SUBJECTS", g_HAS_SUBJECTS, s_HAS_SUBJECTS);
    pyrt::xrefs::reg_access("understanding.normalize", "HAVE_FORMS", g_HAVE_FORMS, s_HAVE_FORMS);
    pyrt::xrefs::reg_access("understanding.normalize", "HEART_FEELINGS", g_HEART_FEELINGS, s_HEART_FEELINGS);
    pyrt::xrefs::reg_access("understanding.normalize", "HEART_MEASURES", g_HEART_MEASURES, s_HEART_MEASURES);
    pyrt::xrefs::reg_access("understanding.normalize", "HEART_SAID_AS_A_FEELING", g_HEART_SAID_AS_A_FEELING, s_HEART_SAID_AS_A_FEELING);
    pyrt::xrefs::reg_access("understanding.normalize", "HELLO_SAID_SHORT", g_HELLO_SAID_SHORT, s_HELLO_SAID_SHORT);
    pyrt::xrefs::reg_access("understanding.normalize", "HELLO_TAIL", g_HELLO_TAIL, s_HELLO_TAIL);
    pyrt::xrefs::reg_access("understanding.normalize", "HOW_DEGREE", g_HOW_DEGREE, s_HOW_DEGREE);
    pyrt::xrefs::reg_access("understanding.normalize", "HOW_FOLLOWS", g_HOW_FOLLOWS, s_HOW_FOLLOWS);
    pyrt::xrefs::reg_access("understanding.normalize", "HR_IS_THE_DEPARTMENT_AFTER", g_HR_IS_THE_DEPARTMENT_AFTER, s_HR_IS_THE_DEPARTMENT_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "HV_HELPERS", g_HV_HELPERS, s_HV_HELPERS);
    pyrt::xrefs::reg_access("understanding.normalize", "HV_SUBJECTS", g_HV_SUBJECTS, s_HV_SUBJECTS);
    pyrt::xrefs::reg_access("understanding.normalize", "ILL_GOES_ON", g_ILL_GOES_ON, s_ILL_GOES_ON);
    pyrt::xrefs::reg_access("understanding.normalize", "ING_THINGS", g_ING_THINGS, s_ING_THINGS);
    pyrt::xrefs::reg_access("understanding.normalize", "INTRODUCING_AFTER", g_INTRODUCING_AFTER, s_INTRODUCING_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "INTRODUCING_A_PERSON", g_INTRODUCING_A_PERSON, s_INTRODUCING_A_PERSON);
    pyrt::xrefs::reg_access("understanding.normalize", "IN_NOUNS", g_IN_NOUNS, s_IN_NOUNS);
    pyrt::xrefs::reg_access("understanding.normalize", "IRREGULAR_PARTICIPLES", g_IRREGULAR_PARTICIPLES, s_IRREGULAR_PARTICIPLES);
    pyrt::xrefs::reg_access("understanding.normalize", "I_GLUED", g_I_GLUED, s_I_GLUED);
    pyrt::xrefs::reg_access("understanding.normalize", "JOINED_WORDS", g_JOINED_WORDS, s_JOINED_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("understanding.normalize", "KIN_FILLERS", g_KIN_FILLERS, s_KIN_FILLERS);
    pyrt::xrefs::reg_access("understanding.normalize", "KIN_HAS", g_KIN_HAS, s_KIN_HAS);
    pyrt::xrefs::reg_access("understanding.normalize", "KIN_IS", g_KIN_IS, s_KIN_IS);
    pyrt::xrefs::reg_access("understanding.normalize", "KIN_SINGULAR", g_KIN_SINGULAR, s_KIN_SINGULAR);
    pyrt::xrefs::reg_access("understanding.normalize", "KNOW_SUBJECT", g_KNOW_SUBJECT, s_KNOW_SUBJECT);
    pyrt::xrefs::reg_access("understanding.normalize", "LEADERS", g_LEADERS, s_LEADERS);
    pyrt::xrefs::reg_access("understanding.normalize", "LEFT_OFF_CONTRACTION", g_LEFT_OFF_CONTRACTION, s_LEFT_OFF_CONTRACTION);
    pyrt::xrefs::reg_access("understanding.normalize", "LETTERS", g_LETTERS, s_LETTERS);
    pyrt::xrefs::reg_access("understanding.normalize", "LETTER_C_IS_SEE", g_LETTER_C_IS_SEE, s_LETTER_C_IS_SEE);
    pyrt::xrefs::reg_access("understanding.normalize", "LEXICON", g_LEXICON, s_LEXICON);
    pyrt::xrefs::reg_access("understanding.normalize", "LIKE_STARTS_A_CLAUSE", g_LIKE_STARTS_A_CLAUSE, s_LIKE_STARTS_A_CLAUSE);
    pyrt::xrefs::reg_access("understanding.normalize", "LONG_ENOUGH_TO_BE_A_VERB", g_LONG_ENOUGH_TO_BE_A_VERB, s_LONG_ENOUGH_TO_BE_A_VERB);
    pyrt::xrefs::reg_access("understanding.normalize", "LOOSE_FOR_LOSE", g_LOOSE_FOR_LOSE, s_LOOSE_FOR_LOSE);
    pyrt::xrefs::reg_access("understanding.normalize", "MEAL_WORDS", g_MEAL_WORDS, s_MEAL_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "ME_FOR_MY_CLOSES", g_ME_FOR_MY_CLOSES, s_ME_FOR_MY_CLOSES);
    pyrt::xrefs::reg_access("understanding.normalize", "MISSING_E_AFTER", g_MISSING_E_AFTER, s_MISSING_E_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "MI_BACK_VERBS", g_MI_BACK_VERBS, s_MI_BACK_VERBS);
    pyrt::xrefs::reg_access("understanding.normalize", "MI_IS_A_NAME_AFTER", g_MI_IS_A_NAME_AFTER, s_MI_IS_A_NAME_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "MI_IS_ME_AFTER", g_MI_IS_ME_AFTER, s_MI_IS_ME_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "MI_IS_ME_BEFORE", g_MI_IS_ME_BEFORE, s_MI_IS_ME_BEFORE);
    pyrt::xrefs::reg_access("understanding.normalize", "MODAL_FORMS", g_MODAL_FORMS, s_MODAL_FORMS);
    pyrt::xrefs::reg_access("understanding.normalize", "MOTION_BEFORE", g_MOTION_BEFORE, s_MOTION_BEFORE);
    pyrt::xrefs::reg_access("understanding.normalize", "MRS_IS_THE_WIFE", g_MRS_IS_THE_WIFE, s_MRS_IS_THE_WIFE);
    pyrt::xrefs::reg_access("understanding.normalize", "NAMED_LETTER", g_NAMED_LETTER, s_NAMED_LETTER);
    pyrt::xrefs::reg_access("understanding.normalize", "NAME_CALLS", g_NAME_CALLS, s_NAME_CALLS);
    pyrt::xrefs::reg_access("understanding.normalize", "NAME_FRAMES", g_NAME_FRAMES, s_NAME_FRAMES);
    pyrt::xrefs::reg_access("understanding.normalize", "NAME_ROLL", g_NAME_ROLL, s_NAME_ROLL);
    pyrt::xrefs::reg_access("understanding.normalize", "NOMINAL_BEFORE_COPULA", g_NOMINAL_BEFORE_COPULA, s_NOMINAL_BEFORE_COPULA);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_A_COMPOUND_HEAD", g_NOT_A_COMPOUND_HEAD, s_NOT_A_COMPOUND_HEAD);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_A_COMPOUND_TAIL", g_NOT_A_COMPOUND_TAIL, s_NOT_A_COMPOUND_TAIL);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_A_MEAL_TYPO", g_NOT_A_MEAL_TYPO, s_NOT_A_MEAL_TYPO);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_A_THING_AFTER", g_NOT_A_THING_AFTER, s_NOT_A_THING_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_A_THING_ENDING", g_NOT_A_THING_ENDING, s_NOT_A_THING_ENDING);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_A_THING_PAIR", g_NOT_A_THING_PAIR, s_NOT_A_THING_PAIR);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_BEFORE_A_NOUN", g_NOT_BEFORE_A_NOUN, s_NOT_BEFORE_A_NOUN);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_BEFORE_KNOW", g_NOT_BEFORE_KNOW, s_NOT_BEFORE_KNOW);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_COUNTED_AFTER", g_NOT_COUNTED_AFTER, s_NOT_COUNTED_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_IN_A_QUESTION_PHRASE", g_NOT_IN_A_QUESTION_PHRASE, s_NOT_IN_A_QUESTION_PHRASE);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_MY_OWN_DAY", g_NOT_MY_OWN_DAY, s_NOT_MY_OWN_DAY);
    pyrt::xrefs::reg_access("understanding.normalize", "NOT_WORDS", g_NOT_WORDS, s_NOT_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "NO_ASK_AFTER", g_NO_ASK_AFTER, s_NO_ASK_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "NO_SWAP_TO", g_NO_SWAP_TO, s_NO_SWAP_TO);
    pyrt::xrefs::reg_access("understanding.normalize", "NUMBERS_BEFORE", g_NUMBERS_BEFORE, s_NUMBERS_BEFORE);
    pyrt::xrefs::reg_access("understanding.normalize", "OBJECT_PRONOUN", g_OBJECT_PRONOUN, s_OBJECT_PRONOUN);
    pyrt::xrefs::reg_access("understanding.normalize", "ONE_OF_A_KIND", g_ONE_OF_A_KIND, s_ONE_OF_A_KIND);
    pyrt::xrefs::reg_access("understanding.normalize", "ONE_PART_AFTER", g_ONE_PART_AFTER, s_ONE_PART_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "ONE_RELATION", g_ONE_RELATION, s_ONE_RELATION);
    pyrt::xrefs::reg_access("understanding.normalize", "ONE_RELATION_VERBS", g_ONE_RELATION_VERBS, s_ONE_RELATION_VERBS);
    pyrt::xrefs::reg_access("understanding.normalize", "ONE_WORD_FOODS", g_ONE_WORD_FOODS, s_ONE_WORD_FOODS);
    pyrt::xrefs::reg_access("understanding.normalize", "OPENERS", g_OPENERS, s_OPENERS);
    pyrt::xrefs::reg_access("understanding.normalize", "OPERATORS", g_OPERATORS, s_OPERATORS);
    pyrt::xrefs::reg_access("understanding.normalize", "OWNER_WITHOUT_THE_DAY", g_OWNER_WITHOUT_THE_DAY, s_OWNER_WITHOUT_THE_DAY);
    pyrt::xrefs::reg_access("understanding.normalize", "OWNING_A_PERSON", g_OWNING_A_PERSON, s_OWNING_A_PERSON);
    pyrt::xrefs::reg_access("understanding.normalize", "OWNING_VERBS", g_OWNING_VERBS, s_OWNING_VERBS);
    pyrt::xrefs::reg_access("understanding.normalize", "PART_REACH", g_PART_REACH, s_PART_REACH);
    pyrt::xrefs::reg_access("understanding.normalize", "PART_THAT_IS_BAD", g_PART_THAT_IS_BAD, s_PART_THAT_IS_BAD);
    pyrt::xrefs::reg_access("understanding.normalize", "PAST_AS_PARTICIPLE", g_PAST_AS_PARTICIPLE, s_PAST_AS_PARTICIPLE);
    pyrt::xrefs::reg_access("understanding.normalize", "PERIOD_COUNTS", g_PERIOD_COUNTS, s_PERIOD_COUNTS);
    pyrt::xrefs::reg_access("understanding.normalize", "PERSON_PREPS", g_PERSON_PREPS, s_PERSON_PREPS);
    pyrt::xrefs::reg_access("understanding.normalize", "PERSON_WORDS", g_PERSON_WORDS, s_PERSON_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "PHRASES", g_PHRASES, s_PHRASES);
    pyrt::xrefs::reg_access("understanding.normalize", "PHRASE_MEASURES", g_PHRASE_MEASURES, s_PHRASE_MEASURES);
    pyrt::xrefs::reg_access("understanding.normalize", "PLACE_WORDS", g_PLACE_WORDS, s_PLACE_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "PLURAL_SUBJECTS", g_PLURAL_SUBJECTS, s_PLURAL_SUBJECTS);
    pyrt::xrefs::reg_access("understanding.normalize", "POSSESSIVES", g_POSSESSIVES, s_POSSESSIVES);
    pyrt::xrefs::reg_access("understanding.normalize", "POSSESSIVE_IN_FRONT", g_POSSESSIVE_IN_FRONT, s_POSSESSIVE_IN_FRONT);
    pyrt::xrefs::reg_access("understanding.normalize", "PRONOUN_BASES", g_PRONOUN_BASES, s_PRONOUN_BASES);
    pyrt::xrefs::reg_access("understanding.normalize", "PUNCT_BREAK", g_PUNCT_BREAK, s_PUNCT_BREAK);
    pyrt::xrefs::reg_access("understanding.normalize", "RAW_WORD", g_RAW_WORD, s_RAW_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "REASON_STARTS", g_REASON_STARTS, s_REASON_STARTS);
    pyrt::xrefs::reg_access("understanding.normalize", "RELATIVE", g_RELATIVE, s_RELATIVE);
    pyrt::xrefs::reg_access("understanding.normalize", "RELATIVE_ASKS", g_RELATIVE_ASKS, s_RELATIVE_ASKS);
    pyrt::xrefs::reg_access("understanding.normalize", "REPEAT_ONCE", g_REPEAT_ONCE, s_REPEAT_ONCE);
    pyrt::xrefs::reg_access("understanding.normalize", "REPEAT_ONCE_SUBJECTS", g_REPEAT_ONCE_SUBJECTS, s_REPEAT_ONCE_SUBJECTS);
    pyrt::xrefs::reg_access("understanding.normalize", "REPORTING", g_REPORTING, s_REPORTING);
    pyrt::xrefs::reg_access("understanding.normalize", "RUN_ON_LAST_PIECES", g_RUN_ON_LAST_PIECES, s_RUN_ON_LAST_PIECES);
    pyrt::xrefs::reg_access("understanding.normalize", "RUN_ON_MINE", g_RUN_ON_MINE, s_RUN_ON_MINE);
    pyrt::xrefs::reg_access("understanding.normalize", "RUN_ON_OPENERS", g_RUN_ON_OPENERS, s_RUN_ON_OPENERS);
    pyrt::xrefs::reg_access("understanding.normalize", "RUN_ON_PEOPLE", g_RUN_ON_PEOPLE, s_RUN_ON_PEOPLE);
    pyrt::xrefs::reg_access("understanding.normalize", "RUN_ON_SELF", g_RUN_ON_SELF, s_RUN_ON_SELF);
    pyrt::xrefs::reg_access("understanding.normalize", "RUN_ON_SELF_LONGEST", g_RUN_ON_SELF_LONGEST, s_RUN_ON_SELF_LONGEST);
    pyrt::xrefs::reg_access("understanding.normalize", "RUN_ON_SHORTEST", g_RUN_ON_SHORTEST, s_RUN_ON_SHORTEST);
    pyrt::xrefs::reg_access("understanding.normalize", "RUN_ON_WORDS", g_RUN_ON_WORDS, s_RUN_ON_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "SAID_APART", g_SAID_APART, s_SAID_APART);
    pyrt::xrefs::reg_access("understanding.normalize", "SAID_APART_FORMS", g_SAID_APART_FORMS, s_SAID_APART_FORMS);
    pyrt::xrefs::reg_access("understanding.normalize", "SAID_AS_A_FEELING", g_SAID_AS_A_FEELING, s_SAID_AS_A_FEELING);
    pyrt::xrefs::reg_access("understanding.normalize", "SAID_IT", g_SAID_IT, s_SAID_IT);
    pyrt::xrefs::reg_access("understanding.normalize", "SAID_OUT", g_SAID_OUT, s_SAID_OUT);
    pyrt::xrefs::reg_access("understanding.normalize", "SENTENCE_END", g_SENTENCE_END, s_SENTENCE_END);
    pyrt::xrefs::reg_access("understanding.normalize", "SHION_WORDS", g_SHION_WORDS, s_SHION_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "SHRT_THINGS", g_SHRT_THINGS, s_SHRT_THINGS);
    pyrt::xrefs::reg_access("understanding.normalize", "SICK_AS_FEELING", g_SICK_AS_FEELING, s_SICK_AS_FEELING);
    pyrt::xrefs::reg_access("understanding.normalize", "SOLFEGE", g_SOLFEGE, s_SOLFEGE);
    pyrt::xrefs::reg_access("understanding.normalize", "SON_VERBS", g_SON_VERBS, s_SON_VERBS);
    pyrt::xrefs::reg_access("understanding.normalize", "SORE_FILLERS", g_SORE_FILLERS, s_SORE_FILLERS);
    pyrt::xrefs::reg_access("understanding.normalize", "SORE_IN_FRONT", g_SORE_IN_FRONT, s_SORE_IN_FRONT);
    pyrt::xrefs::reg_access("understanding.normalize", "SORE_PARTS", g_SORE_PARTS, s_SORE_PARTS);
    pyrt::xrefs::reg_access("understanding.normalize", "SPELLING_ASK", g_SPELLING_ASK, s_SPELLING_ASK);
    pyrt::xrefs::reg_access("understanding.normalize", "SPELLING_ENDINGS", g_SPELLING_ENDINGS, s_SPELLING_ENDINGS);
    pyrt::xrefs::reg_access("understanding.normalize", "SPLIT_CONTRACTION", g_SPLIT_CONTRACTION, s_SPLIT_CONTRACTION);
    pyrt::xrefs::reg_access("understanding.normalize", "SPLIT_NOT", g_SPLIT_NOT, s_SPLIT_NOT);
    pyrt::xrefs::reg_access("understanding.normalize", "STATES_OF_A_PERSON", g_STATES_OF_A_PERSON, s_STATES_OF_A_PERSON);
    pyrt::xrefs::reg_access("understanding.normalize", "STOPPED_TH_STEMS", g_STOPPED_TH_STEMS, s_STOPPED_TH_STEMS);
    pyrt::xrefs::reg_access("understanding.normalize", "SUBJECT_WORDS", g_SUBJECT_WORDS, s_SUBJECT_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "SUGAR_AFTER_ROSE", g_SUGAR_AFTER_ROSE, s_SUGAR_AFTER_ROSE);
    pyrt::xrefs::reg_access("understanding.normalize", "SUGAR_FIRST_PERSON", g_SUGAR_FIRST_PERSON, s_SUGAR_FIRST_PERSON);
    pyrt::xrefs::reg_access("understanding.normalize", "SUGAR_LEVEL", g_SUGAR_LEVEL, s_SUGAR_LEVEL);
    pyrt::xrefs::reg_access("understanding.normalize", "SUGAR_PRICE", g_SUGAR_PRICE, s_SUGAR_PRICE);
    pyrt::xrefs::reg_access("understanding.normalize", "SUGAR_PRICE_SHAPE", g_SUGAR_PRICE_SHAPE, s_SUGAR_PRICE_SHAPE);
    pyrt::xrefs::reg_access("understanding.normalize", "SUM_STAYS_AFTER", g_SUM_STAYS_AFTER, s_SUM_STAYS_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "SUM_THE_NOUN", g_SUM_THE_NOUN, s_SUM_THE_NOUN);
    pyrt::xrefs::reg_access("understanding.normalize", "SUN_WORDS", g_SUN_WORDS, s_SUN_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "S_CLUSTERS", g_S_CLUSTERS, s_S_CLUSTERS);
    pyrt::xrefs::reg_access("understanding.normalize", "TAG_GUARD_BEFORE", g_TAG_GUARD_BEFORE, s_TAG_GUARD_BEFORE);
    pyrt::xrefs::reg_access("understanding.normalize", "TAG_PLEASE", g_TAG_PLEASE, s_TAG_PLEASE);
    pyrt::xrefs::reg_access("understanding.normalize", "TAG_QUESTION_LEADS", g_TAG_QUESTION_LEADS, s_TAG_QUESTION_LEADS);
    pyrt::xrefs::reg_access("understanding.normalize", "TAG_THANKS", g_TAG_THANKS, s_TAG_THANKS);
    pyrt::xrefs::reg_access("understanding.normalize", "TAG_WORDS", g_TAG_WORDS, s_TAG_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "TAIL_OF_A_DAY", g_TAIL_OF_A_DAY, s_TAIL_OF_A_DAY);
    pyrt::xrefs::reg_access("understanding.normalize", "THE_HEART_SAID_THE_OTHER_WAY", g_THE_HEART_SAID_THE_OTHER_WAY, s_THE_HEART_SAID_THE_OTHER_WAY);
    pyrt::xrefs::reg_access("understanding.normalize", "THE_LOSING", g_THE_LOSING, s_THE_LOSING);
    pyrt::xrefs::reg_access("understanding.normalize", "THE_ODD_PLURALS", g_THE_ODD_PLURALS, s_THE_ODD_PLURALS);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_AFTER", g_THING_AFTER, s_THING_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_DETERMINER", g_THING_DETERMINER, s_THING_DETERMINER);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_JOIN", g_THING_JOIN, s_THING_JOIN);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_PLACE_AFTER", g_THING_PLACE_AFTER, s_THING_PLACE_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_PLACE_WORDS", g_THING_PLACE_WORDS, s_THING_PLACE_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_POSSESSIVES", g_THING_POSSESSIVES, s_THING_POSSESSIVES);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_SUBJECT", g_THING_SUBJECT, s_THING_SUBJECT);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_WORDS", g_THING_WORDS, s_THING_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "THING_WORDS_BY_LENGTH", g_THING_WORDS_BY_LENGTH, s_THING_WORDS_BY_LENGTH);
    pyrt::xrefs::reg_access("understanding.normalize", "THIRD_PERSON", g_THIRD_PERSON, s_THIRD_PERSON);
    pyrt::xrefs::reg_access("understanding.normalize", "THIRD_PERSON_BE", g_THIRD_PERSON_BE, s_THIRD_PERSON_BE);
    pyrt::xrefs::reg_access("understanding.normalize", "THOUSANDS", g_THOUSANDS, s_THOUSANDS);
    pyrt::xrefs::reg_access("understanding.normalize", "THOUSANDS_SAID_WITH_A_SPACE", g_THOUSANDS_SAID_WITH_A_SPACE, s_THOUSANDS_SAID_WITH_A_SPACE);
    pyrt::xrefs::reg_access("understanding.normalize", "TIMES_A_STATE", g_TIMES_A_STATE, s_TIMES_A_STATE);
    pyrt::xrefs::reg_access("understanding.normalize", "TIME_AFTER_MEAL", g_TIME_AFTER_MEAL, s_TIME_AFTER_MEAL);
    pyrt::xrefs::reg_access("understanding.normalize", "TIME_WORDS", g_TIME_WORDS, s_TIME_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "TOKEN", g_TOKEN, s_TOKEN);
    pyrt::xrefs::reg_access("understanding.normalize", "TOO_A", g_TOO_A, s_TOO_A);
    pyrt::xrefs::reg_access("understanding.normalize", "TOO_AFTER", g_TOO_AFTER, s_TOO_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "TOO_A_GRADED", g_TOO_A_GRADED, s_TOO_A_GRADED);
    pyrt::xrefs::reg_access("understanding.normalize", "TO_AFTER", g_TO_AFTER, s_TO_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "UP_TOO_ASKS", g_UP_TOO_ASKS, s_UP_TOO_ASKS);
    pyrt::xrefs::reg_access("understanding.normalize", "UP_TO_LEADS", g_UP_TO_LEADS, s_UP_TO_LEADS);
    pyrt::xrefs::reg_access("understanding.normalize", "UP_TO_SPELLINGS", g_UP_TO_SPELLINGS, s_UP_TO_SPELLINGS);
    pyrt::xrefs::reg_access("understanding.normalize", "UP_TO_TAGS", g_UP_TO_TAGS, s_UP_TO_TAGS);
    pyrt::xrefs::reg_access("understanding.normalize", "URGENT_BONES", g_URGENT_BONES, s_URGENT_BONES);
    pyrt::xrefs::reg_access("understanding.normalize", "URGENT_WORDS", g_URGENT_WORDS, s_URGENT_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "VEGGIES_AFTER", g_VEGGIES_AFTER, s_VEGGIES_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "VOWELS", g_VOWELS, s_VOWELS);
    pyrt::xrefs::reg_access("understanding.normalize", "VOWEL_SET", g_VOWEL_SET, s_VOWEL_SET);
    pyrt::xrefs::reg_access("understanding.normalize", "VOWEL_SET_OR_Y", g_VOWEL_SET_OR_Y, s_VOWEL_SET_OR_Y);
    pyrt::xrefs::reg_access("understanding.normalize", "WANT_WORDS", g_WANT_WORDS, s_WANT_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "WEARS_AFTER", g_WEARS_AFTER, s_WEARS_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "WHAT_ARE_YOU_AFTER", g_WHAT_ARE_YOU_AFTER, s_WHAT_ARE_YOU_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "WHAT_ARE_YOU_SPELLINGS", g_WHAT_ARE_YOU_SPELLINGS, s_WHAT_ARE_YOU_SPELLINGS);
    pyrt::xrefs::reg_access("understanding.normalize", "WH_ANY", g_WH_ANY, s_WH_ANY);
    pyrt::xrefs::reg_access("understanding.normalize", "WH_INTENSIFIER", g_WH_INTENSIFIER, s_WH_INTENSIFIER);
    pyrt::xrefs::reg_access("understanding.normalize", "WH_INTENSIFIER_LINE", g_WH_INTENSIFIER_LINE, s_WH_INTENSIFIER_LINE);
    pyrt::xrefs::reg_access("understanding.normalize", "WH_OPENERS", g_WH_OPENERS, s_WH_OPENERS);
    pyrt::xrefs::reg_access("understanding.normalize", "WH_QUANTIFIERS", g_WH_QUANTIFIERS, s_WH_QUANTIFIERS);
    pyrt::xrefs::reg_access("understanding.normalize", "WH_WORDS", g_WH_WORDS, s_WH_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "YOU_ARE_AFTER", g_YOU_ARE_AFTER, s_YOU_ARE_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "_AFTER_A_DOING", g__AFTER_A_DOING, s__AFTER_A_DOING);
    pyrt::xrefs::reg_access("understanding.normalize", "_AFTER_A_LEAD_IN_SUBJECT", g__AFTER_A_LEAD_IN_SUBJECT, s__AFTER_A_LEAD_IN_SUBJECT);
    pyrt::xrefs::reg_access("understanding.normalize", "_AFTER_PARTICIPLE", g__AFTER_PARTICIPLE, s__AFTER_PARTICIPLE);
    pyrt::xrefs::reg_access("understanding.normalize", "_ANY_WORD", g__ANY_WORD, s__ANY_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_CLIP", g__A_CLIP, s__A_CLIP);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_CLIPPED_HEAD", g__A_CLIPPED_HEAD, s__A_CLIPPED_HEAD);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_CLOSING_WORD", g__A_CLOSING_WORD, s__A_CLOSING_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_HEAD_IN_THE_IDIOM", g__A_HEAD_IN_THE_IDIOM, s__A_HEAD_IN_THE_IDIOM);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_QUESTION_VERB", g__A_QUESTION_VERB, s__A_QUESTION_VERB);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_SHORT_WORD_IS_NAMED_AFTER", g__A_SHORT_WORD_IS_NAMED_AFTER, s__A_SHORT_WORD_IS_NAMED_AFTER);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_SHORT_WORD_IS_NAMED_BEFORE", g__A_SHORT_WORD_IS_NAMED_BEFORE, s__A_SHORT_WORD_IS_NAMED_BEFORE);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_STATE_OF_THE_PERSON", g__A_STATE_OF_THE_PERSON, s__A_STATE_OF_THE_PERSON);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_WORDS", g__A_WORDS, s__A_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "_A_WORD_THE_BOT_WRITES", g__A_WORD_THE_BOT_WRITES, s__A_WORD_THE_BOT_WRITES);
    pyrt::xrefs::reg_access("understanding.normalize", "_BASE", g__BASE, s__BASE);
    pyrt::xrefs::reg_access("understanding.normalize", "_DAY_WORDS", g__DAY_WORDS, s__DAY_WORDS);
    pyrt::xrefs::reg_access("understanding.normalize", "_EXCLAMATION_SUBJECTS", g__EXCLAMATION_SUBJECTS, s__EXCLAMATION_SUBJECTS);
    pyrt::xrefs::reg_access("understanding.normalize", "_FEELING_WORD", g__FEELING_WORD, s__FEELING_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "_HAVE_ASKING", g__HAVE_ASKING, s__HAVE_ASKING);
    pyrt::xrefs::reg_access("understanding.normalize", "_HAVE_NEGATIONS", g__HAVE_NEGATIONS, s__HAVE_NEGATIONS);
    pyrt::xrefs::reg_access("understanding.normalize", "_HAVE_PARTICIPLES", g__HAVE_PARTICIPLES, s__HAVE_PARTICIPLES);
    pyrt::xrefs::reg_access("understanding.normalize", "_HAVE_THING", g__HAVE_THING, s__HAVE_THING);
    pyrt::xrefs::reg_access("understanding.normalize", "_INTENSIFIER", g__INTENSIFIER, s__INTENSIFIER);
    pyrt::xrefs::reg_access("understanding.normalize", "_KEEPING_PLACES", g__KEEPING_PLACES, s__KEEPING_PLACES);
    pyrt::xrefs::reg_access("understanding.normalize", "_KEPT_ALREADY", g__KEPT_ALREADY, s__KEPT_ALREADY);
    pyrt::xrefs::reg_access("understanding.normalize", "_KEPT_THINGS", g__KEPT_THINGS, s__KEPT_THINGS);
    pyrt::xrefs::reg_access("understanding.normalize", "_LEADER_WORD", g__LEADER_WORD, s__LEADER_WORD);
    pyrt::xrefs::reg_access("understanding.normalize", "_MEASURE", g__MEASURE, s__MEASURE);
    pyrt::xrefs::reg_access("understanding.normalize", "_NUMBER_SHAPE", g__NUMBER_SHAPE, s__NUMBER_SHAPE);
    pyrt::xrefs::reg_access("understanding.normalize", "_PLACES_SAID_WHOLE", g__PLACES_SAID_WHOLE, s__PLACES_SAID_WHOLE);
    pyrt::xrefs::reg_access("understanding.normalize", "_PLACE_LEADS", g__PLACE_LEADS, s__PLACE_LEADS);
    pyrt::xrefs::reg_access("understanding.normalize", "_PLACE_STOP", g__PLACE_STOP, s__PLACE_STOP);
    pyrt::xrefs::reg_access("understanding.normalize", "_PREDICATE_ONLY", g__PREDICATE_ONLY, s__PREDICATE_ONLY);
    pyrt::xrefs::reg_access("understanding.normalize", "_SMALL_NUMBERS", g__SMALL_NUMBERS, s__SMALL_NUMBERS);
    pyrt::xrefs::reg_access("understanding.normalize", "_SPACED_PIECE", g__SPACED_PIECE, s__SPACED_PIECE);
    pyrt::xrefs::reg_access("understanding.normalize", "_SPACED_RUN", g__SPACED_RUN, s__SPACED_RUN);
    pyrt::xrefs::reg_access("understanding.normalize", "_STATE_FILLER", g__STATE_FILLER, s__STATE_FILLER);
    pyrt::xrefs::reg_access("understanding.normalize", "_STATE_TAIL", g__STATE_TAIL, s__STATE_TAIL);
    pyrt::xrefs::reg_access("understanding.normalize", "_SUBJECTLESS_AUX", g__SUBJECTLESS_AUX, s__SUBJECTLESS_AUX);
    pyrt::xrefs::reg_access("understanding.normalize", "_SUBJECTLESS_ONLY", g__SUBJECTLESS_ONLY, s__SUBJECTLESS_ONLY);
    pyrt::xrefs::reg_access("understanding.normalize", "_SUBJECTS", g__SUBJECTS, s__SUBJECTS);
    pyrt::xrefs::reg_access("understanding.normalize", "_SUBJECT_SAID", g__SUBJECT_SAID, s__SUBJECT_SAID);
    pyrt::xrefs::reg_access("understanding.normalize", "_TYPED_SURPRISE", g__TYPED_SURPRISE, s__TYPED_SURPRISE);
    pyrt::xrefs::reg_access("understanding.normalize", "_WERE_ARE", g__WERE_ARE, s__WERE_ARE);
    pyrt::xrefs::reg_access("understanding.normalize", "_WERE_DOING", g__WERE_DOING, s__WERE_DOING);
    pyrt::xrefs::reg_access("understanding.normalize", "_WERE_IN_THE_PAST", g__WERE_IN_THE_PAST, s__WERE_IN_THE_PAST);
    pyrt::xrefs::reg_access("understanding.normalize", "_WERE_LEAD_IN", g__WERE_LEAD_IN, s__WERE_LEAD_IN);
    pyrt::xrefs::reg_access("understanding.normalize", "_WERE_PLACE", g__WERE_PLACE, s__WERE_PLACE);
    pyrt::xrefs::reg_access("understanding.normalize", "_WERE_STATE", g__WERE_STATE, s__WERE_STATE);
    pyrt::xrefs::reg_access("understanding.normalize", "_WH_QUESTIONS", g__WH_QUESTIONS, s__WH_QUESTIONS);
}
